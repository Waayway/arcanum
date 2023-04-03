use std::{
    collections::HashMap,
    fs,
    process::exit,
    thread, vec,
};

use crate::{
    request::Request,
    response::{Response, ResponseConstruction, ReturnData},
    router::Router,
};
use tiny_http::{Server, Request as Req};

#[derive(Clone)]
pub enum RouteHandler {
    WithRouteParams(fn(Request, &mut Response, HashMap<String, String>) -> ReturnData),
    Simple(fn(Request, &mut Response) -> ReturnData),
    WithRouteAndOptionalParams(
        fn(Request, &mut Response, HashMap<String, String>) -> ReturnData,
        HashMap<String, String>,
    ),
}

pub struct WebServer {
    server: Server,
    internal_router: Router,
    routers: Vec<Router>,
}

impl WebServer {
    pub fn new(ip: &str, port: i32) -> Self {
        let server = tiny_http::Server::http(format!("{ip}:{port}")).unwrap_or_else(|err| {
            eprintln!("ERROR: Cannot setup the server port or ip {err}");
            exit(1);
        });

        println!("INFO: server started at http://{ip}:{port}/");
        let internal_router = Router::new("/");
        Self {
            server: server,
            internal_router: internal_router,
            routers: vec![],
        }
    }

    pub fn add_route_with_params(
        &mut self,
        route: &str,
        function: fn(Request, &mut Response, HashMap<String, String>) -> ReturnData,
    ) {
        self.internal_router.add_route_with_params(route, function);
    }

    pub fn add_simple_route(
        &mut self,
        route: &str,
        function: fn(Request, &mut Response) -> ReturnData,
    ) {
        self.internal_router.add_simple_route(route, function);
    }

    pub fn add_static_file_route(&mut self, route: &str, path: &str) {
        self.internal_router.add_static_file_route(route, path);
    }

    pub fn add_router(&mut self, router: Router) {
        self.routers.push(router);
    }

    pub fn run(&self) {
        while let Ok(req) = self.server.recv() {
            // println!("INFO: Connection Established!");
            let internal_router = self.internal_router.clone();
            let other_routers = self.routers.clone();
            thread::spawn(move || {
                Self::handle_connection(internal_router, other_routers, req);
            });
        }
    }

    fn handle_connection(
        internal_router: Router,
        other_routers: Vec<Router>,
        mut req: Req,
    ) {
        let mut request = Request::new(&req);
        let mut response = Response::new();

        let content = Self::handle_routes(
            internal_router,
            other_routers,
            req.url(),
            request,
            &mut response,
        );
        // println!("INFO: Request {:#?}", http_request);

        let mut response = ResponseConstruction::generate_response(response);

        match content {
            ReturnData::RawData(data) => response.set_raw_data(data),
            ReturnData::Text(text) => response.add_content(&text),
            ReturnData::Json(json) => {
                response.add_header("Content-Type", "application/json");
                response.add_content(&json);
            }
        }

        let mut raw_response = response.render_response();
        req.respond(raw_response).unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't write content to stream: {err}");
        });
    }

    // # Parse path params, so :id for example
    // @param route, ROUTE so the configured route
    // @param path, PATH so the path to the current page
    fn parse_path_params(route: &str, path: &str) -> HashMap<String, String> {
        let route_parts: Vec<&str> = route.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();
        let mut map: HashMap<String, String> = HashMap::new();
        for (route_part, path_part) in route_parts.iter().zip(path_parts.iter()) {
            // DEBUG: println!("route_part: {route_part}\n path_part: {path_part}\n");
            if route_part.starts_with(":") {
                map.insert(
                    route_part[1..route_part.len()].to_string(),
                    path_part.to_string(),
                );
            } else if route_part.starts_with("*") {
                let rel_path = path.replace(&route.replace("*", ""), "");
                map.insert("path".to_string(), rel_path);
            }
        }
        map
    }

    fn handle_routes(
        internal_router: Router,
        other_routers: Vec<Router>,
        route: &str,
        req: Request,
        res: &mut Response,
    ) -> ReturnData {
        let mut route_path = internal_router.does_path_exists(&req.route);
        // println!("{:?}",route_path);
        let mut route_handler: Option<RouteHandler> = None;
        if route_path.is_none() {
            for i in other_routers {
                if let Some(temp) = i.does_path_exists(&req.route) {
                    route_path = Some(temp.clone());
                    route_handler = i.route_handler_from_path(temp.clone())
                }
            }
        } else {
            route_handler = internal_router.route_handler_from_path(route_path.as_deref().unwrap().to_string());
        }

        if route_path.clone().is_none() {
            return ReturnData::Text(
                fs::read_to_string("views/404.html")
                    .unwrap_or_else(|err| format!("404.html couldnt be found {err}")),
            );
        }
        
        let route = route_path.unwrap();
        let path_params = Self::parse_path_params(&route, &req.route);
        match route_handler.unwrap() {
            RouteHandler::Simple(handler) => handler(req, res),
            RouteHandler::WithRouteParams(handler) => handler(req, res, path_params),
            RouteHandler::WithRouteAndOptionalParams(handler, ref opt_params) => handler(
                req,
                res,
                path_params.into_iter().chain(opt_params.clone()).collect(),
            ),
        }
    }
}

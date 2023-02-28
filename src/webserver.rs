use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    process::exit,
    thread,
};

use crate::{request::Request, response::{Response, ReturnData, ResponseConstruction}, serve_files::serve_static_file};

#[derive(Clone)]
enum RouteHandler {
    WithRouteParams(fn(Request, &mut Response, HashMap<String, String>) -> ReturnData),
    Simple(fn(Request, &mut Response) -> ReturnData),
    WithRouteAndOptionalParams(
        fn(Request, &mut Response, HashMap<String, String>) -> ReturnData,
        HashMap<String, String>,
    ),
}

pub struct WebServer {
    listener: TcpListener,
    routes: HashMap<String, RouteHandler>,
}

impl WebServer {
    pub fn new(ip: &str, port: i32) -> Self {
        let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap_or_else(|err| {
            eprintln!("ERROR: Cannot setup the server port or ip {err}");
            exit(1);
        });

        println!("INFO: server started at http://{ip}:{port}/");

        Self {
            listener: listener,
            routes: HashMap::new(),
        }
    }

    pub fn add_route_with_params(
        &mut self,
        route: &str,
        function: fn(Request, &mut Response, HashMap<String, String>) -> ReturnData,
    ) {
        self.routes
            .insert(route.to_owned(), RouteHandler::WithRouteParams(function));
    }

    pub fn add_simple_route(
        &mut self,
        route: &str,
        function: fn(Request, &mut Response) -> ReturnData,
    ) {
        self.routes
            .insert(route.to_owned(), RouteHandler::Simple(function));
    }

    pub fn add_static_file_route(&mut self, route: &str, path: &str) {
        let mut params: HashMap<String, String> = HashMap::new();
        params.insert("basepath".to_string(), path.to_string());
        self.routes.insert(
            route.to_owned(),
            RouteHandler::WithRouteAndOptionalParams(
                |_req: Request, res: &mut Response, params: HashMap<String, String>| {
                    if !params.contains_key("basepath") {
                        res.set_status_code(500);
                        return ReturnData::Text("Something went wrong!".to_string());
                    }
                    if !params.contains_key("path") {
                        res.set_status_code(403);
                        return ReturnData::Text("Cannot index folders".to_string());
                    } else if params["path"].ends_with("/") {
                        res.set_status_code(403);
                        return ReturnData::Text("Cannot index folders".to_string());
                    }
                    let path = Path::new(&params["basepath"]).join(&params["path"]);
                    serve_static_file(path.to_str().unwrap(), res)
                },
                params,
            ),
        );
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            
            // println!("INFO: Connection Established!");
            let all_routes = self.routes.clone();
            thread::spawn(move || {
                Self::handle_connection(all_routes, stream);
            });
        }
    }

    fn handle_connection(all_routes: HashMap<String, RouteHandler>, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut http_iter = http_request.iter();

        let route = http_iter.next().unwrap();

        let mut request = Request::new(&Self::compute_route_request(route));
        let mut response = Response::new();

        for i in http_iter {
            request.add_header(i);
        }

        let content = Self::handle_routes(all_routes, route, request, &mut response);
        // println!("INFO: Request {:#?}", http_request);

        let mut response = ResponseConstruction::generate_response(response);

        match content {
            ReturnData::RawData(data) => response.set_raw_data(data),
            ReturnData::Text(text) => response.add_content(&text),
        }

        let mut raw_response = response.render_response().as_bytes().to_vec();
        raw_response.append(&mut response.get_raw_data_if_any().clone());
        stream.write_all(&raw_response).unwrap_or_else(|err| {
            eprintln!("ERROR: Couldn't write content to stream: {err}");
        });
    }

    fn compute_route_request(route: &str) -> [String; 3] {
        let splitted_route: Vec<&str> = route.split(" ").collect();
        [
            splitted_route[0].to_owned(),
            splitted_route[1].to_owned(),
            splitted_route[2].to_owned(),
        ]
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

    fn does_path_exists_in_routes(all_routes: &HashMap<String, RouteHandler>, path: &str) -> Option<String> {
        let mut current_path = None;
        let path_parts: Vec<&str> = path.split("/").filter(|i| !i.is_empty()).collect();
        for i in all_routes {
            let route_parts: Vec<&str> = i.0.split("/").filter(|i| !i.is_empty()).collect();
            // DEBUG: println!("route_parts: {route_parts:?}\n path_parts: {path_parts:?}\n");
            // if route_parts.len() != path_parts.len() {
            //     continue;
            // }
            if route_parts == path_parts {
                current_path = Some(i.0.to_owned());
                continue;
            }
            for (route_part, path_part) in route_parts.iter().zip(path_parts.iter()) {
                // DEBUG: println!("route_part: {route_part}\n path_part: {path_part}\n");
                if route_part.starts_with(":") || route_part == path_part || route_part == &"*" {
                    current_path = Some(i.0.to_owned());
                } else if route_part == &"**" {
                    current_path = Some(i.0.to_owned());
                    break;
                } else if route_part != path_part {
                    current_path = None;
                    break;
                }
            }
            if current_path.is_some() {
                break;
            }
        }
        // DEBUG: println!("{:?}", current_path);
        current_path
    }

    fn handle_routes(all_routes: HashMap<String, RouteHandler>, route: &str, req: Request, res: &mut Response) -> ReturnData {
        let route_data = Self::compute_route_request(route);
        let route_path = Self::does_path_exists_in_routes(&all_routes, &route_data[1]);

        if route_path.is_none() {
            return ReturnData::Text(
                fs::read_to_string("views/404.html")
                    .unwrap_or_else(|err| format!("404.html couldnt be found {err}")),
            );
        }
        let route = route_path.unwrap();
        let path_params = Self::parse_path_params(&route, &route_data[1]);
        match all_routes[&route] {
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

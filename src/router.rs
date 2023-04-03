use std::{collections::HashMap, path::Path};

use crate::{serve_static_file, webserver::RouteHandler, Request, Response, ReturnData};

#[derive(Clone)]
pub struct Router {
    basepath: String,
    routes: HashMap<String, RouteHandler>,
}

impl Router {
    pub fn new(base_path: &str) -> Self {
        Self {
            basepath: base_path.to_string(),
            routes: HashMap::new(),
        }
    }

    pub fn add_simple_route(
        &mut self,
        route: &str,
        function: fn(_: Request, _: &mut Response) -> ReturnData,
    ) {
        self.routes
            .insert(route.to_owned(), RouteHandler::Simple(function));
    }

    pub fn add_route_with_params(
        &mut self,
        route: &str,
        function: fn(Request, &mut Response, HashMap<String, String>) -> ReturnData,
    ) {
        self.routes
            .insert(route.to_owned(), RouteHandler::WithRouteParams(function));
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

    

    pub fn does_path_exists(
        &self,
        path: &str,
    ) -> Option<String> {
        let mut current_path = None;
        let path_parts: Vec<&str> = path.split("/").filter(|i| !i.is_empty()).collect();
        for i in self.routes.clone() {
            let mut route_parts: Vec<&str> = i.0.split("/").filter(|i| !i.is_empty()).collect();
            // println!("route_parts: {route_parts:?}\n path_parts: {path_parts:?}\n")
            let basepath_parts: Vec<&str> = self.basepath.split("/").filter(|i| !i.is_empty()).collect();
            route_parts.extend(basepath_parts);
            // println!("route_parts: {route_parts:?}\n path_parts: {path_parts:?}\n");
            if route_parts.len() != path_parts.len() {
                let diff = path_parts.len() as i32 - route_parts.len() as i32;
                if diff > 0 {
                    for _ in 0..diff {
                        route_parts.push("");
                    }
                }
            }
            if route_parts == path_parts {
                current_path = Some(i.0.to_owned());
                continue;
            }

            for (route_part, path_part) in route_parts.iter().zip(path_parts.iter()) {
                // println!("route_part: {route_part}\n path_part: {path_part}\n");
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
    pub fn route_handler_from_path(&self, path: String) -> Option<RouteHandler> {
        if self.routes.keys().any(|i| i == &path) {
            return Some(self.routes[&path].clone());
        }
        None
    }
}

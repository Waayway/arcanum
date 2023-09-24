use std::{ collections::HashMap, vec, future::Future };

pub mod enums;
pub use enums::*;

pub mod routes;
pub use routes::*;


pub struct Router {
    routes: HashMap<(Method, String), (Box<dyn RouteHandler + Sync + Send>, Option<Vec<String>>)>,
}

impl Router {
    pub fn new(routes: Vec<(Method, &str, Box<dyn RouteHandler + Sync + Send>)>) -> Router {
        let mut router = Router {
            routes: HashMap::new(),
        };

        for route in routes {
            let r = route.1.split("/").collect::<Vec<&str>>();
            let mut params = vec![];
            let mut s = String::new();
            for i in r {
                if i.starts_with("{") && i.ends_with("}") {
                    let temp = i.replace("{", "").replace("}", "");
                    let split: Vec<&str> = temp.split(":").collect();
                    params.push(split[0].to_string());
                    if split[1] == "number" {
                        s.push_str("/[0-9]+");
                    } else if split[1] == "string" {
                        s.push_str("/[a-zA-Z0-9]+");
                    } else {
                        s.push_str("/[a-zA-Z0-9]+");
                    }
                } else {
                    s.push_str(i);
                    s.push_str("/");
                }
            }
            // println!("Final Route: {}, Original Route: {}", s, route.1);
            let final_param = if params.len() >= 1 { Some(params) } else { None };

            router.routes.insert((route.0, s), (route.2, final_param));
        }

        router
    }

    pub fn route(&self, method: Method, path: &str) -> Option<(String, Type)> {
        for i in self.routes.keys() {
            if i.0 != method {
                continue;
            }

            let split_path: Vec<String> = path
                .split("/")
                .map(|x| { x.to_string() })
                .filter(|x| !x.is_empty())
                .collect();
            let split_route: Vec<String> = i.1
                .split("/")
                .map(|x| { x.to_string() })
                .filter(|x| !x.is_empty())
                .collect();

            if split_path.len() != split_route.len() {
                continue;
            }
            let mut params: Vec<Param> = Vec::new();
            let mut correct = true;
            for (route, path) in split_route.iter().zip(split_path.iter()) {
                // println!("Route: {}, Path: {}", route, path);
                if !regex::Regex::new(route).unwrap().is_match(path) {
                    correct = false;
                    break;
                } else {
                    if let Some(cap) = regex::Regex::new(&route).unwrap().captures(path) {
                        let param = if let Some(raw_param) = cap.get(0) {
                            let param = raw_param.as_str().to_string();
                            match route.as_str() {
                                "[a-zA-Z0-9]+" => { Some(Param::string(param)) }
                                "[0-9]+" => Some(Param::number(param.parse::<isize>().unwrap())),
                                _ => { None }
                            }
                        } else {
                            None
                        };
                        if let Some(param) = param {
                            params.push(param);
                        }
                    }
                }
            }
            if correct {
                let params = if let Some(route_params) = self.routes.get(i).unwrap().1.clone() {
                    let mut map = HashMap::new();
                    for (param, value) in route_params.iter().zip(params.iter()) {
                        map.insert(param.to_string(), value.clone());
                    }
                    map
                } else {
                    HashMap::new()
                };
                return Some(self.routes.get(i).unwrap().0.handle(params));
            }
        }
        None
    }
}

// #[test]
// fn basic_test_router() {
//     let router = Router::new(
//         vec![
//             (Method::GET, "/", Route::raw(|_| { "Hello /".to_string() })),
//             (Method::GET, "/test", Route::raw(|_| { "Hello test".to_string() }))
//         ]
//     );

//     assert_eq!(router.route(Method::GET, "/"), Some("Hello /".to_string()));
//     assert_eq!(router.route(Method::GET, "/test"), Some("Hello test".to_string()));
//     assert_eq!(router.route(Method::GET, "/notfound"), None);
// }

// #[test]
// fn test_advanced_router() {
//     let router = Router::new(
//         vec![
//             (Method::GET, "/", Route::raw(|_| { "Hello /".to_string() })),
//             (
//                 Method::GET,
//                 "/{id:number}",
//                 Route::raw(|x| {
//                     let x = match x.get("id") {
//                         Some(Param::number(x)) => x.clone(),
//                         _ => { 0 }
//                     };
//                     format!("Hello from Number {}", x).to_string()
//                 }),
//             ),
//             (
//                 Method::GET,
//                 "/{id:string}",
//                 Route::raw(|x| {
//                     let x = match x.get("id") {
//                         Some(Param::string(x)) => x.clone(),
//                         _ => { "".to_string() }
//                     };
//                     format!("Hello from String {}", x).to_string()
//                 }),
//             )
//         ]
//     );

//     assert_eq!(router.route(Method::GET, "/"), Some(("Hello /".to_string(), Type::Raw)));
//     assert_eq!(router.route(Method::GET, "/123"), Some(("Hello from Number 123".to_string(), Type::Raw)));
//     assert_eq!(router.route(Method::GET, "/100"), Some(("Hello from Number 100".to_string(), Type::Raw)));
//     assert_eq!(router.route(Method::GET, "/abc"), Some(("Hello from String abc".to_string(), Type::Raw)));
//     assert_eq!(router.route(Method::GET, "/test/eng"), None);
// }

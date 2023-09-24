use std::collections::HashMap;

use arcanum::{controller::{Routing, BaseController, controller, route}, router::{Method, Route, Param, CRoute}, ApplicationController};


fn main() {
    let bs = BaseController::new("examples/templates");
    let mut ac = ApplicationController::controller("127.0.0.1", 8080, vec![
        controller::<ExampleController>(bs)
    ]);
    ac.start();
}

#[derive(Clone)]
struct ExampleController {
    bs: BaseController,
}

impl Routing for ExampleController {
    fn new(bs: BaseController) -> Self where Self: Sized {
        Self {
            bs,
        }
    }
    fn routes(&self) -> Vec<(arcanum::router::Method, &str, Box<dyn arcanum::router::RouteHandler + Sync + Send>)> {
        vec![
            (Method::GET, route(self, "/"), CRoute::html(Self::example_func, self.clone())),
        ]
    }
    fn base_route(&self) -> String {
        "/example".to_string()
    }
}

impl ExampleController {
    fn example_func(s: Self, _: HashMap<String, Param>) -> String {
        "Hello from ExampleController".to_string()
    }
}
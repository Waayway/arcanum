use std::collections::HashMap;


pub struct Route {
    pub raw_path: String,
    pub path_variables: HashMap<String, String>,
}

impl Route {
    pub fn construct_route(route: [String;3]) -> Self {
        Self {
            raw_path: route[1].clone(),
            path_variables: HashMap::new(),
        }
    }
}
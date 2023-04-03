use std::collections::HashMap;

use tiny_http::Request as Req;

pub struct Request {
    pub headers: HashMap<String, String>,
    pub connection_type: String,
    pub route: String
}

impl Request {
    pub fn new(req: &Req) -> Self {
        Self {
            headers: HashMap::new(),
            connection_type: req.http_version().to_string(),
            route: req.url().to_string()
        }
    }
    pub fn add_header(&mut self, header: &str) {
        let header_data: Vec<&str> = header.split(":").collect();
        self.headers.insert(header_data[0].to_owned(), header_data[1].to_owned());
    }
    pub fn set_route(&mut self, route: String) {
        self.route = route;
    }
}

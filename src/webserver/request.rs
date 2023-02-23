use std::collections::HashMap;

pub struct Request {
    pub headers: HashMap<String, String>,
    pub connection_type: String
}

impl Request {
    pub fn new(route: &[String; 3]) -> Self {
        Self {
            headers: HashMap::new(),
            connection_type: route[0].clone()
        }
    }
    pub fn add_header(&mut self, header: &str) {
        let header_data: Vec<&str> = header.split(":").collect();
        self.headers.insert(header_data[0].to_owned(), header_data[1].to_owned());
    }
}

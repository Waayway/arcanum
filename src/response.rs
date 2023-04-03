use std::{collections::HashMap, io::Cursor, str::FromStr};

use tiny_http::Response as Res;


pub struct ResponseConstruction {
    headers: HashMap<String, String>,
    content: String,
    data: Vec<u8>,
}

impl ResponseConstruction {
    pub fn generate_response(res: Response) -> Self {
        let response = format!("HTTP/1.1 {status_code}", status_code = res.status_code);
        Self {
            headers: res.headers,
            content: "".to_owned(),
            data: vec![]
        }
    }
    
    pub fn add_header(&mut self, header: &str, value: &str) {
        self.headers.insert(header.to_owned(), value.to_owned());
    } 
    pub fn add_content(&mut self, content: &str) {
        self.content += content;
    }
    pub fn set_raw_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
    pub fn render_response(&self) -> Res<Cursor<Vec<u8>>> { 
        let mut newData: Vec<u8>;
        if self.data.len() < 1 {
            let content_length = self.content.len();
            newData = format!("{}", self.content).as_bytes().to_vec()
        } else {
            let content_length = self.data.len();
            newData = self.data.clone();
        }
        let mut response = Res::from_data(newData);
        for i in self.headers.iter() {
            response.add_header(tiny_http::Header::from_str(&format!("{}: {}", i.0,i.1)).unwrap());
        }
        response
    }
}


pub struct Response {
    headers: HashMap<String, String>,
    status_code: i32
}
impl Response {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
            status_code: 200
        }
    }
    pub fn add_header(&mut self, header: &str, value: &str) {
        self.headers.insert(header.to_owned(), value.to_owned());
    } 
    pub fn set_status_code(&mut self, status_code: i32) {
        self.status_code = status_code;
    }
}


pub enum ReturnData {
    RawData(Vec<u8>),
    Text(String),
    Json(String)
}
use std::collections::HashMap;


pub struct ResponseConstruction {
    base_response: String,
    headers: HashMap<String, String>,
    content: String,
    data: Vec<u8>,
}

impl ResponseConstruction {
    pub fn generate_response(res: Response) -> Self {
        let response = format!("HTTP/1.1 {status_code}", status_code = res.status_code);
        Self {
            base_response: response.to_owned(),
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
    pub fn render_response(&self) -> String { 
        let headers: String = self.headers.iter().map(|(header, value)| format!("{header}: {value}\r\n")).collect();
        if self.data.len() < 1 {
            let content_length = self.content.len();
            format!("{}\r\nContent-Length: {content_length}\r\n{headers}\r\n{}", self.base_response, self.content)
        } else {
            let content_length = self.data.len();
            format!("{}\r\nContent-Length: {content_length}\r\n{headers}\r\n", self.base_response)
        }
    }
    pub fn get_raw_data_if_any(&self) -> Vec<u8> {
        return self.data.clone();
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
    Text(String)
}
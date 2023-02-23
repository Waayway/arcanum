// # Imports
use std::{
    collections::HashMap,
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process::exit,
};
pub mod request;
pub mod response;
pub mod serve_files;
use request::Request;
use response::{Response, ResponseConstruction};

use self::response::ReturnData;

pub struct WebServer {
    listener: TcpListener,
    routes: HashMap<String, fn(Request, &mut Response) -> ReturnData>,
}

impl WebServer {
    pub fn new(ip: &str, port: i32) -> Self {
        let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap_or_else(|err| {
            eprintln!("ERROR: Cannot setup the server port or ip {err}");
            exit(1);
        });

        println!("INFO: server started at {ip}:{port}");

        Self {
            listener: listener,
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, route: &str, function: fn(Request, &mut Response) -> ReturnData) {
        self.routes.insert(route.to_owned(), function);
    }

    pub fn add_static_file_route(&mut self, route: &str) {}
    
    pub fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            println!("INFO: Connection Established!");
            self.handle_connection(stream);
        }
    }
    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut http_iter = http_request.iter();

        let route = http_iter.next().unwrap();

        let mut request = Request::new(&self.compute_route_request(route));
        let mut response = Response::new();

        for i in http_iter {
            request.add_header(i);
        }

        let content = self.handle_routes(route, request, &mut response);
        // println!("INFO: Request {:#?}", http_request);

        let mut response = ResponseConstruction::generate_response(200, response);
        match content {
            ReturnData::RawData(data) => response.set_raw_data(data),
            ReturnData::Text(text) => response.add_content(&text),
        }
        let mut raw_response = response.render_response().as_bytes().to_vec();
                raw_response.append(&mut response.get_raw_data_if_any().clone());
        stream
            .write_all(&raw_response)
            .unwrap_or_else(|err| {
                eprintln!("ERROR: Couldn't write content to stream: {err}");
            });
    }

    fn compute_route_request(&self, route: &str) -> [String; 3] {
        let splitted_route: Vec<&str> = route.split(" ").collect();
        println!("{:?}", splitted_route);

        [
            splitted_route[0].to_owned(),
            splitted_route[1].to_owned(),
            splitted_route[2].to_owned(),
        ]
    }

    fn handle_routes(&self, route: &str, req: Request, res: &mut Response) -> ReturnData {
        let route_data = self.compute_route_request(route);
        if !self.routes.contains_key(&route_data[1]) {
            return ReturnData::Text(
                fs::read_to_string("views/404.html")
                    .unwrap_or_else(|err| format!("404.html couldnt be found {err}")),
            );
        }
        return self.routes[&route_data[1]](req, res);
    }
}

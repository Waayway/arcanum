use std::{ sync::Arc, thread::{ self, JoinHandle } };

use tiny_http::{ Server as TinyServer, Response};

use crate::router::{Router, Method, Type};

pub struct Server {
    addr: String,
    port: u16,
    pub guard_amount: u8,
    guards: Vec<JoinHandle<()>>,
    router: Arc<Router>,
}

impl Server {
    pub fn new(addr: &str, port: u16, router: Arc<Router>) -> Self {
        Self {
            addr: addr.to_string(),
            port,
            guard_amount: 4,
            guards: vec![],
            router: router,
        }
    }

    pub fn start(&mut self) {
        let addr = format!("{}:{}", self.addr, self.port);
        let server = Arc::new(match TinyServer::http(addr.clone()) {
            Ok(s) => s,
            Err(e) => {
                panic!("Error opening on {}, with error: {}", addr, e);
            }
        });

        println!("Server started on {}", addr);

        let mut working = true;
        for _ in 0..self.guard_amount {
            let server = server.clone();
            let router = self.router.clone();
            let guard = thread::spawn(move || {
                while working {
                    let rq = match server.recv() {
                        Ok(rq) => rq,
                        Err(e) => {
                            println!("Error receiving request: {}", e);
                            working = false;
                            break;
                        }
                    };
                    println!("Received request: {} {}", rq.method(), rq.url());

                    let route = match router.route(Method::from_tiny_http(rq.method()), rq.url()) {
                        Some(r) => r,
                        None => {
                            println!("No route found for {} {}", rq.method(), rq.url());
                            ("404 Not Found".to_string(), Type::Raw)
                        }
                    };
                    let mut response = Response::from_string(route.0);
                    response.add_header(
                        match route.1 {
                            Type::Html => tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=UTF-8"[..]).unwrap(),
                            Type::Json => tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/json; charset=UTF-8"[..]).unwrap(),
                            Type::Raw => tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/plain; charset=UTF-8"[..]).unwrap(),
                        }
                    );
                    
                    let req = rq.respond(response);

                    if req.is_err() {
                        println!("Error responding to request: {}", req.err().unwrap());
                        break;
                    }
                }
            });
            self.guards.push(guard);
        }
        while working {
            thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

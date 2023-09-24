use std::sync::Arc;

use arcanum::{serve::{Server},router::{Router, Method, Route, Param}};

fn main() {
    let router = Router::new(vec![
        (Method::GET, "/", Route::html(|_| {"Hello /".to_string()})),
        (Method::GET, "/number/{id:number}", Route::html(|x| {
            let x = match x.get("id") {
                Some(Param::number(x)) => x.clone(),
                _ => {0}
            };
            format!("Hello from Number {}", x).to_string()
        })),
        (Method::GET, "/string/{id:string}", Route::html(|x| {
            let x = match x.get("id") {
                Some(Param::string(x)) => x.clone(),
                _ => {"".to_string()}
            };
            format!("Hello from String {}", x).to_string()
        })),
    ]);
    let mut server = Server::new("127.0.0.1", 8080, Arc::new(router));
    server.start();
}
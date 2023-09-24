use std::sync::Arc;

use arcanum::{ApplicationController, router::{Method, Route}};

fn main() {
    let mut ac = ApplicationController::basic("127.0.0.1", 8080 ,vec![
        (Method::GET, "/", Route::html(|_| {"<h1>Hello /</h1>".to_string()})),
    ]);
    ac.start();
}
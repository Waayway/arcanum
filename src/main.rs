
mod webserver;
use std::fs;

use crate::webserver::WebServer;
use webserver::{request::Request, response::{Response, ReturnData}, serve_files::serve_image};
fn main() {
    let mut server = WebServer::new("127.0.0.1", 7878);
    server.add_route("/", handle_main_route);
    server.add_route("/img", |req: Request, res: &mut Response| serve_image("public/user.png", res));
    server.run();
}

fn handle_main_route(req: Request, res: &mut Response) -> ReturnData {
    println!("{:?}", req.headers);
    return ReturnData::Text(fs::read_to_string("views/index.html").unwrap());
}
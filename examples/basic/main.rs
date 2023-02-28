use std::collections::HashMap;

use arcanum::{serve_static_file, Request, Response, ReturnData, Template, WebServer};
use serde::Serialize;

fn main() {
    let mut server = WebServer::new("127.0.0.1", 7878);
    server.add_simple_route("/", handle_main_route);
    server.add_simple_route("/img/**", |_req: Request, res: &mut Response| {
        serve_static_file("public/user.png", res)
    });
    server.add_route_with_params("/id/:id", handle_id_route);
    server.add_static_file_route("/public/**", "public/");
    server.run();
}

#[derive(Serialize)]
struct HomepageContext {
    title: String,
}

fn handle_main_route(_req: Request, _res: &mut Response) -> ReturnData {
    let context = HomepageContext {
        title: "Hello, world!".to_string(),
    };
    let template = Template::render_template("views/index.html", context);
    return ReturnData::Text(template);
}

#[derive(Serialize)]
struct IdPageContext {
    id: String,
}

fn handle_id_route(
    _req: Request,
    _res: &mut Response,
    params: HashMap<String, String>,
) -> ReturnData {
    let context = IdPageContext {
        id: params["id"].clone(),
    };
    let template = Template::render_template("views/id.html", context);
    return ReturnData::Text(template);
}

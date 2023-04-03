use arcanum::{WebServer, Router, Response, ReturnData, Request};

fn main() {
    let mut server = WebServer::new("127.0.0.1", 7878);
    
    let mut router = Router::new("/test");
    router.add_simple_route("/", test);
    router.add_simple_route("/test", test);

    server.add_router(router);
    server.run();
}

fn test(_req: Request, _res: &mut Response) -> ReturnData {
    ReturnData::Text("Hello, World from /test".to_string())
}
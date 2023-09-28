use std::sync::Arc;

use controller::{ Routing };
use router::{ Method, RouteHandler };

pub mod router;
pub mod serve;
pub mod controller;
pub mod templates;
pub mod db;

pub struct ApplicationController {
    pub router: Arc<router::Router>,
    pub server: serve::Server,
}

impl ApplicationController {
    pub fn basic(
        addr: &str,
        port: u16,
        routes: Vec<(Method, &str, Box<dyn RouteHandler + Sync + Send>)>
    ) -> Self {
        let mut router = Arc::new(router::Router::new(routes));
        let server = serve::Server::new(addr, port, router.clone());
        Self {
            router,
            server,
        }
    }

    pub fn controller<C>(addr: &str, port: u16, controllers: Vec<Box<C>>) -> Self
        where C: Routing
    {
        let mut routes = vec![];

        controllers.iter().for_each(|c| {
            let mut controller_routes = c.routes();
            routes.append(&mut controller_routes);
        });

        Self::basic(addr, port, routes)
    }

    pub fn start(&mut self) {
        self.server.start();
    }
}
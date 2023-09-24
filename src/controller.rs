use crate::{router::{Method, RouteHandler}, templates::Templates};


pub trait Routing {
    fn new(bs: BaseController) -> Self where Self: Sized;
    fn routes(&self) -> Vec<(Method, &str, Box<dyn RouteHandler + Sync + Send>)>;
    fn base_route(&self) -> String {
        "".to_string()
    }
}

pub fn controller<C>(bs: BaseController) -> Box<C> where C: Routing {
    Box::new(C::new(bs))
}

pub fn route<'a, C>(controller: &C, route: &'a str) -> &'a str where C: Routing {
    let concat = format!("{}{}", controller.base_route(), route);

    Box::leak(concat.into_boxed_str())
}

#[derive(Clone)]
pub struct BaseController {
    templates: Templates,    
}

impl BaseController {
    pub fn new(template_dir: &str) -> Self {
        Self {
            templates: Templates::new(template_dir),
        }
    }
    pub fn template(&self, template_name: &str, context: &tera::Context) -> String {
        match self.templates.render(template_name, context) {
            Ok(s) => s,
            Err(e) => format!("Error: {}", e),
        }
    }
}
use tera::Tera;

pub use tera::Context;

#[derive(Clone)]
pub struct Templates {
    pub tera: Tera,
}

impl Templates {
    pub fn new(template_dir: &str) -> Self {
        let tera = match Tera::new(format!("{}/**/*", template_dir).as_str()) {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        Self { tera }
    }

    pub fn render(&self, template_name: &str, context: &Context) -> Result<String, tera::Error> {
        self.tera.render(template_name, context)
    }
}
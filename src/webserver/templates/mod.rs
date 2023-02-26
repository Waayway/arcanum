use std::fs;

use serde::Serialize;
use tinytemplate::TinyTemplate;

pub struct Template {}


impl Template {
    pub fn render_template<T>(file_path: &str, context: T) -> String where T: Serialize  {
        let mut tt = TinyTemplate::new();
        let file_content = fs::read_to_string(file_path).unwrap_or_else(|err| {
            eprintln!("ERROR: cannot load file {file_path}, {err}");
            "Couldn't find file {file_path}".to_string()
        });
        tt.add_template("temp-template", &file_content).unwrap_or_else(|err| {
            eprintln!("ERROR: Something went wrong with adding the template, {err}");
        });
        tt.render("temp-template", &context).unwrap_or_else(|err| {
            eprintln!("ERROR: Something went wrong with rendering the file {file_path} because of {err}");
            "Something went wrong".to_string()
        })
    }
}
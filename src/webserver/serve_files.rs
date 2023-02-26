use std::fs;

use super::response::{Response, ReturnData};

pub fn serve_static_file(path: &str, res: &mut Response) -> ReturnData {
    let file = fs::read(path).unwrap_or_else(|err| {
        eprintln!("ERROR: File doesn't exist: {path}, with error: {err}");
        res.set_status_code(404);
        vec![]
    });
    let mime_type = new_mime_guess::from_path(path).first_raw().unwrap_or_else(|| "text/plain");
    res.add_header("Content-type", mime_type);
    return ReturnData::RawData(file);
}
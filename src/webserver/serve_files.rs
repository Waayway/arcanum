use std::fs;

use super::response::{Response, ReturnData};

pub fn serve_image(image_path: &str, res: &mut Response) -> ReturnData {
    let img = fs::read(image_path).unwrap();
    res.add_header("Content-type", "image/png");    
    return ReturnData::RawData(img);
}
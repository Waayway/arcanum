use std::{collections::HashMap, future::Future};

use tiny_http::Method as TinyMethod;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    CONNECT,
    PATCH,
    TRACE,
}
impl Method {
    pub fn from_tiny_http(method: &TinyMethod) -> Self {
        match method.clone() {
            TinyMethod::Connect => Self::CONNECT,
            TinyMethod::Delete => Self::DELETE,
            TinyMethod::Get => Self::GET,
            TinyMethod::Head => Self::HEAD,
            TinyMethod::Options => Self::OPTIONS,
            TinyMethod::Patch => Self::PATCH,
            TinyMethod::Post => Self::POST,
            TinyMethod::Put => Self::PUT,
            TinyMethod::Trace => Self::TRACE,
            TinyMethod::NonStandard(_) => Self::GET,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Param {
    number(isize),
    string(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Json,
    Raw,
    Html,
}
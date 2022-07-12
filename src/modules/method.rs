pub enum Method {
    POST,
    GET,
    NA,
}
impl Method {
    pub fn from_str(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::NA,
        }
    }
}

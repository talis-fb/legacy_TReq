#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum METHODS {
    DELETE,
    GET,
    HEAD,
    PATCH,
    POST,
    PUT,
}
impl ToString for METHODS {
    fn to_string(&self) -> String {
        match self {
            Self::GET => "GET".to_string(),
            Self::POST => "POST".to_string(),
            Self::HEAD => "HEAD".to_string(),
            Self::PATCH => "PATCH".to_string(),
            Self::PUT => "PUT".to_string(),
            Self::DELETE => "DELETE".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Request {
    pub name: String,
    pub url: String,
    pub method: METHODS,
    pub headers: String,
    pub body: String,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: METHODS::GET,
            name: String::from("New Request"),
            url: String::new(),
            headers: String::new(),
            body: String::new(),
        }
    }
}

impl Request {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_method(&mut self, method: METHODS) {
        self.method = method;
    }

    pub fn set_headers(&mut self, headers: String) {
        self.headers = headers;
    }

    pub fn set_body(&mut self, body: String) {
        self.body = body;
    }
}

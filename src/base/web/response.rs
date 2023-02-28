use std::collections::HashMap;

#[derive(Default, Copy, Clone, Debug)]
pub enum ResponseStage {
    #[default]
    Empty,

    Waiting,
    Finished,

    InternalError,
}

#[derive(Default, Clone, Debug)]
pub struct Response {
    pub status: i32,
    pub response_time: f64,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub stage: ResponseStage,
}

impl Response {
    fn replace(&mut self, new_response: Response) {
        *self = new_response;
    }

    pub fn default_internal_error(err: String) -> Self {
        Self {
            status: 77, // A STATUS CODE INTERNAL TO INTERNAL ERROR
            response_time: 0.0,
            headers: HashMap::new(),
            body: err,
            stage: ResponseStage::InternalError,
        }
    }
}

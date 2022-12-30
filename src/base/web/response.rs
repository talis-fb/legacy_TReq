#[derive(Default, Clone, Debug)]
pub struct Response {
    pub status: i32,
    pub response_time: i32,
    pub headers: String,
    pub body: String,
}

impl Response {
    fn replace(&mut self, new_response: Response) {
        *self = new_response;
    }
}

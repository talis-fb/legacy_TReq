use reqwest::Client;

use super::repository::HttpClientRepository;
use super::request::METHODS;
use super::{request::Request, response::Response};

use std::sync::mpsc::{self, Sender, Receiver};

// #[derive(Default, Clone)]
pub struct WebClient<T: HttpClientRepository> {
    http_client: T,

    // sender: Sender<Request>,
    // receiver: Receiver<Request>,

    draft: Option<Request>,
    // response: Option<Response>,
}

impl<T> WebClient<T>
where
    T: HttpClientRepository,
{

    pub fn init(repository: T) -> Self {
        // let (sender, receiver): (Sender<Request>, Receiver<Request>) = mpsc::channel();

        Self { http_client: repository, draft: None }
        // Self { http_client: repository, sender, receiver, draft: None }
        // draft: None,
        // response: None
    }

    pub fn set_request(&mut self) -> () {

    }

    pub async fn submit(&self, request_to_do:Request) -> Result<Response, String> {
        let Request {
            url, headers, body, ..
        } = request_to_do;

        let response = match request_to_do.method {
            METHODS::GET => self.http_client.call_get(url, headers).await,
            METHODS::POST => self.http_client.call_post(url, headers, body).await,
            METHODS::PUT => self.http_client.call_put(url, headers, body).await,
            METHODS::PATCH => self.http_client.call_patch(url, headers, body).await,
            METHODS::HEAD => self.http_client.call_patch(url, headers, body).await,
            METHODS::DELETE => self.http_client.call_patch(url, headers, body).await,
            METHODS::OPTIONS => self.http_client.call_options(url, headers, body).await,
        };

        Ok(response)
    }
}

use reqwest::Client;

use super::repository::HttpClientRepository;
use super::request::METHODS;
use super::{request::Request, response::Response};

use std::sync::mpsc::{self, Receiver, Sender};

pub struct WebClient<T: HttpClientRepository> {
    http_client: T,
    response: Option<Response>,
}

impl<T> WebClient<T>
where
    T: HttpClientRepository,
{
    pub fn init(repository: T) -> Self {
        Self {
            http_client: repository,
            response: None,
        }
    }

    pub async fn submit(&self, request_to_do: Request) -> Result<Response, String> {
        let Request {
            url, headers, body, ..
        } = request_to_do;

        let response = self.http_client.call_get(url, headers).await;
        // let response = match request_to_do.method {
        //     METHODS::GET => self.http_client.call_get(url, headers).await,
        //     METHODS::POST => self.http_client.call_post(url, headers, body).await,
        //     METHODS::PUT => self.http_client.call_put(url, headers, body).await,
        //     METHODS::PATCH => self.http_client.call_patch(url, headers, body).await,
        //     METHODS::HEAD => self.http_client.call_patch(url, headers, body).await,
        //     METHODS::DELETE => self.http_client.call_patch(url, headers, body).await,
        //     METHODS::OPTIONS => self.http_client.call_options(url, headers, body).await,
        // };

        if let Ok(a) = &response {
            Ok(response.unwrap())
        } else {
            Err(String::from(""))
        }
    }
}

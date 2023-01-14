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

        let has_the_protocol_in_begin =
            regex::Regex::new("^(?!https?://.)*$").map_err(|e| e.to_string())?;

        if !has_the_protocol_in_begin.is_match(&url) {
            println!("FOII CARAI");
        }



        let response = match request_to_do.method {
            METHODS::GET => self.http_client.call_get(url, headers).await,
            METHODS::POST => self.http_client.call_post(url, headers, body).await,
            METHODS::PUT => self.http_client.call_put(url, headers, body).await,
            METHODS::PATCH => self.http_client.call_patch(url, headers, body).await,
            METHODS::HEAD => self.http_client.call_head(url, headers, body).await,
            METHODS::DELETE => self.http_client.call_delete(url, headers, body).await,
        };

        response
    }
}

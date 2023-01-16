use crate::base::validators::{Validators, ValidatorsHandler};

use super::repository::HttpClientRepository;
use super::request::METHODS;
use super::{request::Request, response::Response};

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

    pub async fn submit(&self, request: Request) -> Result<Response, String> {
        let request_to_do = ValidatorsHandler::from(request.clone())
            .execute([Validators::url_protocol_request()])?;

        let Request {
            url, headers, body, ..
        } = request_to_do;

        let response = match request_to_do.method {
            METHODS::GET => self.http_client.call_get(url, headers).await,
            METHODS::POST => self.http_client.call_post(url, headers, body).await,
            METHODS::PUT => self.http_client.call_put(url, headers, body).await,
            METHODS::PATCH => self.http_client.call_patch(url, headers, body).await,
            METHODS::HEAD => self.http_client.call_head(url, headers, body).await,
            METHODS::DELETE => self.http_client.call_delete(url, headers, body).await,
        };

        let response = ValidatorsHandler::from(response?.clone())
            .execute_ignoring_errors([Validators::set_pretty_json_response()])?;

        Ok(response)
    }
}

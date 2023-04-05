use std::collections::HashMap;

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

    pub async fn submit(&self, request: Request, variables: &HashMap<String, String>) -> Result<Response, String> {
        let request_to_do = ValidatorsHandler::from(&request).execute([
            Validators::url_protocol_request(),
            Validators::url_and_body_template_engine(variables),
        ])?;

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
        }?;

        let response = ValidatorsHandler::from(&response)
            .execute_ignoring_errors([Validators::set_pretty_json_response()]);

        Ok(response)
    }
}

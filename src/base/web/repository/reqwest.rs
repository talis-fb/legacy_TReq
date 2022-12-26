use crate::base::web::response::Response;

use super::HttpClientRepository;
use async_trait::async_trait;

#[derive(Default)]
pub struct ReqwestClientRepository;

#[async_trait::async_trait]
impl HttpClientRepository for ReqwestClientRepository {
    async fn call_get(&self, url: String, headers: String) -> Response {
        Response::default()
    }

    async fn call_post(&self, url: String, headers: String, body: String) -> Response {
        Response::default()
    }

    async fn call_delete(&self, url: String, headers: String, body: String) -> Response {
        Response::default()
    }

    async fn call_patch(&self, url: String, headers: String, body: String) -> Response {
        Response::default()
    }

    async fn call_put(&self, url: String, headers: String, body: String) -> Response {
        Response::default()
    }

    async fn call_options(&self, url: String, headers: String, body: String) -> Response {
        Response::default()
    }
}

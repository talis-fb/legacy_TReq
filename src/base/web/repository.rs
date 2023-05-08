pub mod reqwest;
use super::response::Response;
use async_trait::async_trait;
use std::collections::HashMap;

// #[cfg_attr(test, mockall::automock)]
#[mockall::automock]
#[async_trait]
pub trait HttpClientRepository: Send + Sync {
    async fn call_get(
        &self,
        url: String,
        headers: HashMap<String, String>,
    ) -> Result<Response, String>;
    async fn call_post(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Result<Response, String>;
    async fn call_delete(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Result<Response, String>;
    async fn call_patch(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Result<Response, String>;
    async fn call_put(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Result<Response, String>;
    async fn call_head(
        &self,
        url: String,
        headers: HashMap<String, String>,
        body: String,
    ) -> Result<Response, String>;
}

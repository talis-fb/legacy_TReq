use async_trait::async_trait;
pub mod reqwest;
use std::error::Error;

use super::{request::Request, response::Response};

#[async_trait]
pub trait HttpClientRepository {
    async fn call_get(&self, url: String, headers: String) -> Result<Response, String>;
    async fn call_post(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String>;
    async fn call_delete(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String>;
    async fn call_patch(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String>;
    async fn call_put(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String>;
    async fn call_head(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String>;
}

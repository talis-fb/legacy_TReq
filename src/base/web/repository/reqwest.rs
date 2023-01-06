use async_trait::async_trait;
use std::error::Error;

use crate::{app::states, base::web::response::Response};

use super::HttpClientRepository;
use reqwest::Client;

#[derive(Default)]
pub struct ReqwestClientRepository;
impl ReqwestClientRepository {
    async fn convert_to_app_response(response: reqwest::Response) -> Result<Response, String> {
        let status: i32 = response.status().as_u16().into();
        let body = response.text().await.map_err(|e| e.to_string())?;

        Ok(Response {
            status,
            body,
            response_time: 1,
            headers: String::from(""),
        })
    }
}

#[async_trait]
impl HttpClientRepository for ReqwestClientRepository {
    async fn call_get(&self, url: String, headers: String) -> Result<Response, String> {
        let client = Client::new();
        let response = client.get(url).send().await.map_err(|e| e.to_string())?;
        ReqwestClientRepository::convert_to_app_response(response).await
    }

    async fn call_post(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String> {
        let client = Client::new();
        let response = client
            .post(url)
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        ReqwestClientRepository::convert_to_app_response(response).await
    }

    async fn call_delete(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String> {
        let client = Client::new();
        let response = client
            .delete(url)
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        ReqwestClientRepository::convert_to_app_response(response).await
    }

    async fn call_patch(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String> {
        let client = Client::new();
        let response = client
            .patch(url)
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        ReqwestClientRepository::convert_to_app_response(response).await
    }

    async fn call_put(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String> {
        let client = Client::new();
        let response = client
            .put(url)
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        ReqwestClientRepository::convert_to_app_response(response).await
    }

    async fn call_head(
        &self,
        url: String,
        headers: String,
        body: String,
    ) -> Result<Response, String> {
        let client = Client::new();
        let response = client
            .head(url)
            .body(body)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        ReqwestClientRepository::convert_to_app_response(response).await
    }
}

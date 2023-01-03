use std::error::Error;
use async_trait::async_trait;

use crate::{base::web::response::Response, app::states};

use super::HttpClientRepository;
use reqwest::{Client, Result};

#[derive(Default)]
pub struct ReqwestClientRepository;

#[async_trait]
impl HttpClientRepository for ReqwestClientRepository {
    async fn call_get(&self, url: String, headers: String) -> Result<Response> {
        let client = Client::new();
        let response = client.get(url).send().await;

        let response = match response {
            Ok(response) => response,
            Err(e) => return Err(e.to_string()),
        };

        let body = match response.text().await {
            Ok(body) => body,
            Err(e) => return Err(e.to_string()),
        };
        let status: i32 = response.status().as_u16().into();
        Ok(Response { status, body, response_time: 1, headers: String::from(""), })

        // let client = Client::new();
        // let response = client.get(url)
        //     .send()
        //     .await
        //     .map_err(|e| e.to_string());
        //
        //
        // let body = response.unwrap().text().await.map_err(|e| e.to_string()).unwrap();
        // let status: i32 = response.unwrap().status().as_u16().into();
        // return Ok(Response { status, body, response_time: 1, headers: String::from(""), });

        
        // if let Ok(res) = response {
        //     if let Ok(body) = res.text().await {
        //         let status: i32 = res.status().as_u16().into();
        //         return Ok(Response { status, body, response_time: 1, headers: String::from(""), });
        //     } else {
        //         return Err(String::from(""));
        //     }
        // } else {
        //     return Err(String::from(""));
        // }

    }

    // async fn call_post(&self, url: String, headers: String, body: String) -> Result<Response> {
    //     Ok(Response::default())
    // }
    //
    // async fn call_delete(&self, url: String, headers: String, body: String) -> Result<Response> {
    //     Ok(Response::default())
    // }
    //
    // async fn call_patch(&self, url: String, headers: String, body: String) -> Result<Response> {
    //     Ok(Response::default())
    // }
    //
    // async fn call_put(&self, url: String, headers: String, body: String) -> Result<Response> {
    //     Ok(Response::default())
    // }
    //
    // async fn call_options(&self, url: String, headers: String, body: String) -> Result<Response> {
    //     Ok(Response::default())
    // }
}

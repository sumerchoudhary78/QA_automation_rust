use anyhow::{Context, Result};
use once_cell::sync::OnceCell;
use reqwest::{Client, ClientBuilder, Response};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

static HTTP_CLIENT: OnceCell<ApiClient> = OnceCell::new();

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    base_url: String,
    auth_token: Arc<RwLock<Option<String>>>, // we will use the parking_lot::rwlock for less overhead fewerr system calls googd for performance
}

impl ApiClient {
    pub fn new(base_url: String) -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(60))
            .cookie_store(true)
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            base_url,
            auth_token: Arc::new(RwLock::new(None)),
        })
    }

    pub fn global() -> &'static ApiClient {
        HTTP_CLIENT.get_or_init(|| {
            let config = lib_test_helpers::config::get_config();
            let base_url = config.api_base_url.trim_end_matches('/').to_string();

            ApiClient::new(base_url).expect("Failed to initialize global API client")
        })
    }

    pub async fn set_auth_token(&self, token: String) {
        let mut auth = self.auth_token.write().await;
        *auth = Some(token);
    }

    pub async fn get_auth_token(&self) -> Option<String> {
        let auth = self.auth_token.read().await;
        auth.clone()
    }

    pub async fn clear_auth_token(&self) {
        let mut auth = self.auth_token.write().await;
        *auth = None;
    }

    pub fn get(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        RequestBuilder::new(self.clone(), self.client.get(&url))
    }

    pub fn post(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        RequestBuilder::new(self.clone(), self.client.post(&url))
    }

    pub fn put(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        RequestBuilder::new(self.clone(), self.client.put(&url))
    }

    pub fn patch(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        RequestBuilder::new(self.clone(), self.client.patch(&url))
    }

    pub fn delete(&self, path: &str) -> RequestBuilder {
        let url = self.build_url(path);
        RequestBuilder::new(self.clone(), self.client.delete(&url))
    }

    fn build_url(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');
        format!("{}/{}", self.base_url, path)
    }
}

pub struct RequestBuilder {
    client: ApiClient,
    request: reqwest::RequestBuilder,
}

impl RequestBuilder {
    fn new(client: ApiClient, request: reqwest::RequestBuilder) -> Self {
        Self { client, request }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.request = self.request.header(key, value);
        self
    }

    pub fn json<T: serde::Serialize>(mut self, body: &T) -> Self {
        self.request = self.request.json(body);
        self
    }

    pub fn query<T: serde::Serialize>(mut self, params: &T) -> Self {
        self.request = self.request.query(params);
        self
    }

    pub async fn send(mut self) -> Result<Response> {
        if let Some(token) = self.client.get_auth_token().await {
            self.request = self.request.bearer_auth(token);
        }

        let response = self
            .request
            .send()
            .await
            .context("Failed to send HTTP request")?;

        eprintln!("[DEBUG] Request URL: {}", response.url());
        eprintln!("[DEBUG] Response Status: {}", response.status());

        Ok(response)
    }

    pub async fn send_json<T: DeserializeOwned>(self) -> Result<T> {
        let response = self.send().await?;
        let status = response.status();
        let url = response.url().clone();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            eprintln!("[ERROR] Request to {} failed", url);
            eprintln!("[ERROR] Status: {}", status);
            eprintln!("[ERROR] Response body: {}", error_text);
            anyhow::bail!("Request failed with status {}: {}", status, error_text);
        }

        let body_text = response
            .text()
            .await
            .context("Failed to get response body")?;
        eprintln!(
            "[DEBUG] Response body: {}",
            &body_text[..200.min(body_text.len())]
        );

        let data =
            serde_json::from_str::<T>(&body_text).context("Failed to deserialize JSON response")?;

        Ok(data)
    }

    pub async fn send_text(self) -> Result<String> {
        let response = self.send().await?;
        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("Request failed with status {}: {}", status, error_text);
        }

        let text = response
            .text()
            .await
            .context("Failed to get response text")?;

        Ok(text)
    }
}

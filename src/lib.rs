use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct InstagresClient {
    client: Client,
}

impl InstagresClient {
    fn get_base_url(referrer: &str) -> Result<String> {
        if referrer.trim().is_empty() {
            return Err(anyhow::anyhow!("referrer is required and cannot be empty"));
        }

        let db_id = uuid::Uuid::now_v7().to_string();
        Ok(format!("https://neon.new/api/v1/database/{db_id}?referrer={referrer}"))
    }

    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Create a new SDK client with custom HTTP client
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
        }
    }

    /// Make a POST request and return the JSON response
    pub async fn post<T>(&self, referrer: impl AsRef<str>, body: impl Serialize) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/create", Self::get_base_url(referrer.as_ref())?);

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let json: T = response.json().await?;
        Ok(json)
    }

    /// Make a POST request with custom headers
    pub async fn post_with_headers<T>(
        &self,
        referrer: impl AsRef<str>,
        body: impl Serialize,
        headers: HashMap<String, String>,
    ) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = format!("{}/create", Self::get_base_url(referrer.as_ref())?);

        let mut request = self.client.post(&url).json(&body);

        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        let response = request.send().await?;
        let json: T = response.json().await?;
        Ok(json)
    }

    /// Make a POST request and return raw JSON string
    pub async fn post_raw(&self, referrer: impl AsRef<str>, body: impl Serialize) -> Result<String> {
        let url = Self::get_base_url(referrer.as_ref())?;

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let text = response.text().await?;
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_base_url() {
        let result = InstagresClient::get_base_url("tester123");
        assert!(result.is_ok());
        assert!(result.unwrap().contains("referrer=tester123"));
    }

    #[tokio::test]
    async fn test_empty_referrer() {
        let result = InstagresClient::get_base_url("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "referrer is required and cannot be empty");
    }
}


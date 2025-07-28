use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT_ENCODING, CONTENT_ENCODING},
    Client,
};
use std::{sync::Arc, time::Duration};

use crate::elong::error::ElongError;

#[derive(Clone)]
pub(crate) struct HttpClient {
    client: Arc<Client>,
}

impl HttpClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip"));

        let client = Client::builder()
            .pool_max_idle_per_host(100)
            .timeout(Duration::from_secs(30))
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        HttpClient {
            client: Arc::new(client),
        }
    }

    pub async fn get(&self, url: &str) -> Result<String, ElongError> {
        let response = self.client.get(url).send().await?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await?;
        if !status.is_success() {
            return Err(ElongError::HttpError(format!(
                "HTTP request failed with status: {status}"
            )));
        }
        let body = if headers.get(CONTENT_ENCODING).is_some() {
            log::debug!("Response is gzipped");
            let mut decoder = flate2::read::GzDecoder::new(body.as_ref());
            let mut decompressed = Vec::new();
            std::io::copy(&mut decoder, &mut decompressed)?;
            String::from_utf8(decompressed)?
        } else {
            log::debug!("Response is not gzipped");
            String::from_utf8(body.to_vec())?
        };
        Ok(body)
    }
}

use std::env;

use async_trait::async_trait;

use crate::{
    request::static_city::*,
    response::{api_response::ElongResponse, static_city::*},
    Elong,
};

use super::{
    client::ElongClient,
    endpoints::{ApiEndpoint, ApiMethod},
    error::ElongError,
};

#[derive(Clone)]
pub struct ElongService {
    pub client: ElongClient,
    pub url: String,
}

impl Default for ElongService {
    fn default() -> Self {
        Self::new()
    }
}

impl ElongService {
    pub fn new() -> Self {
        let username =
            env::var("ELONG_USERNAME").expect("Environment variable ELONG_USERNAME is not set");
        let app_key =
            env::var("ELONG_APP_KEY").expect("Environment variable ELONG_APP_KEY is not set");
        let app_secret =
            env::var("ELONG_APP_SECRET").expect("Environment variable ELONG_APP_SECRET is not set");
        let client = ElongClient::new(username, app_key, app_secret);
        let url = ApiEndpoint::Prod.url();
        ElongService { client, url }
    }

    pub fn new_with_endpoint(
        username: String,
        app_key: String,
        app_secret: String,
        api_endpoints: ApiEndpoint,
    ) -> Self {
        let client = ElongClient::new(username, app_key, app_secret);
        let url = api_endpoints.url();
        ElongService { client, url }
    }
}

#[async_trait]
impl Elong for ElongService {
    async fn get_static_city(
        &self,
        request: StaticCityReq,
    ) -> Result<ElongResponse<StaticCityRes>, ElongError> {
        let res: ElongResponse<StaticCityRes> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticCity, request)
            .await?;
        Ok(res)
    }
}

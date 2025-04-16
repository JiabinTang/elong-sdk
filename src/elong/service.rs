use std::env;

use async_trait::async_trait;

use crate::{
    request::{
        data_inventory::InventoryRequest, data_rate::DataRateRequest, incr_inv::IncrInvRequest,
        static_city::*, static_info::StaticInfoRequest, static_list::StaticListRequest,
    },
    response::{
        api_response::ElongResponse, data_inventory::InventoryResponse,
        data_rate::DataRateResponse, incr_inv::IncrInvResponse, static_city::*,
        static_info::StaticInfoResponse, static_list::StaticListResponse,
    },
    Elong, ElongResult,
};

use super::{
    client::ElongClient,
    endpoints::{ApiEndpoint, ApiMethod},
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
    /// 静态数据 - 城市列表
    async fn get_static_city(&self, request: StaticCityRequest) -> ElongResult<StaticCityResponse> {
        let res: ElongResponse<StaticCityResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticCity, request)
            .await?;
        Ok(res)
    }

    /// 静态数据 - 酒店列表
    async fn get_static_list(&self, request: StaticListRequest) -> ElongResult<StaticListResponse> {
        let res: ElongResponse<StaticListResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticList, request)
            .await?;
        Ok(res)
    }

    /// 静态数据 - 酒店详情
    async fn get_static_info(&self, request: StaticInfoRequest) -> ElongResult<StaticInfoResponse> {
        let res: ElongResponse<StaticInfoResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticInfo, request)
            .await?;
        Ok(res)
    }

    /// 静态数据 - 库存全量
    async fn get_inventory(&self, request: InventoryRequest) -> ElongResult<InventoryResponse> {
        let res: ElongResponse<InventoryResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::DataInventory, request)
            .await?;
        Ok(res)
    }

    /// 静态数据 - 库存增量
    async fn get_incr_inv(&self, request: IncrInvRequest) -> ElongResult<IncrInvResponse> {
        let res: ElongResponse<IncrInvResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrInv, request)
            .await?;
        Ok(res)
    }

    /// 静态数据 - 库存增量切片
    async fn get_incr_sharding_inv(&self, request: IncrInvRequest) -> ElongResult<IncrInvResponse> {
        let res: ElongResponse<IncrInvResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrShardingInv, request)
            .await?;
        Ok(res)
    }

    /// 静态数据 - 价格全量
    async fn get_data_rate(&self, request: DataRateRequest) -> ElongResult<DataRateResponse> {
        let res: ElongResponse<DataRateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::DataRate, request)
            .await?;
        Ok(res)
    }
}

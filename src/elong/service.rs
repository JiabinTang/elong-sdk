use std::env;

use async_trait::async_trait;

use crate::{
    request::{
        data_booking::DataBookingRequest, data_inventory::InventoryRequest,
        data_rate::DataRateRequest, data_rp::DataRpRequest, data_validate::DataValidateRequest,
        incr_id::IncrIdRequest, incr_inv::IncrInvRequest, incr_rate::IncrRateRequest,
        incr_state::IncrStateRequest, static_city::*, static_info::StaticInfoRequest,
        static_list::StaticListRequest,
    },
    response::{
        api_response::ElongResponse, data_booking::DataBookingResponse,
        data_inventory::InventoryResponse, data_rate::DataRateResponse, data_rp::DataRpResponse,
        data_validate::DataValidateResponse, incr_id::IncrIdResponse, incr_inv::IncrInvResponse,
        incr_rate::IncrRateResponse, incr_state::IncrStateResponse, static_city::*,
        static_info::StaticInfoResponse, static_list::StaticListResponse,
    },
    types::*,
    Elong,
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
    /// 城市列表
    async fn get_static_city(&self, request: StaticCityRequest) -> RECityResp {
        let res: ElongResponse<StaticCityResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticCity, request)
            .await?;
        Ok(res)
    }

    /// 酒店列表
    async fn get_static_list(&self, request: StaticListRequest) -> REListResp {
        let res: ElongResponse<StaticListResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticList, request)
            .await?;
        Ok(res)
    }

    /// 酒店详情
    async fn get_static_info(&self, request: StaticInfoRequest) -> REInfoResp {
        let res: ElongResponse<StaticInfoResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticInfo, request)
            .await?;
        Ok(res)
    }

    /// 产品详情
    async fn get_data_rp(&self, request: DataRpRequest) -> REDataRpResp {
        let res: ElongResponse<DataRpResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::DataRp, request)
            .await?;
        Ok(res)
    }

    /// 增量编号
    async fn get_incr_id(&self, request: IncrIdRequest) -> REIncrIdResp {
        let res: ElongResponse<IncrIdResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrId, request)
            .await?;
        Ok(res)
    }

    /// 增量编号分片
    async fn get_incr_sharding_id(&self, request: IncrIdRequest) -> REIncrIdResp {
        let res: ElongResponse<IncrIdResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrShardingId, request)
            .await?;
        Ok(res)
    }

    /// 状态增量
    async fn get_incr_state(&self, request: IncrStateRequest) -> REIncrStateResp {
        let res: ElongResponse<IncrStateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrState, request)
            .await?;
        Ok(res)
    }

    /// 状态增量分片
    async fn get_incr_sharding_state(&self, request: IncrStateRequest) -> REIncrStateResp {
        let res: ElongResponse<IncrStateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrShardingState, request)
            .await?;
        Ok(res)
    }

    /// 库存全量
    async fn get_inventory(&self, request: InventoryRequest) -> REInvResp {
        let res: ElongResponse<InventoryResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::DataInventory, request)
            .await?;
        Ok(res)
    }

    /// 库存增量
    async fn get_incr_inv(&self, request: IncrInvRequest) -> REIncrInvResp {
        let res: ElongResponse<IncrInvResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrInv, request)
            .await?;
        Ok(res)
    }

    /// 库存增量切片
    async fn get_incr_sharding_inv(&self, request: IncrInvRequest) -> REIncrInvResp {
        let res: ElongResponse<IncrInvResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrShardingInv, request)
            .await?;
        Ok(res)
    }

    /// 价格全量
    async fn get_data_rate(&self, request: DataRateRequest) -> RERateResp {
        let res: ElongResponse<DataRateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::DataRate, request)
            .await?;
        Ok(res)
    }

    /// 价格增量
    async fn get_incr_rate(&self, request: IncrRateRequest) -> REIncrRateResp {
        let res: ElongResponse<IncrRateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrRate, request)
            .await?;
        Ok(res)
    }

    /// 价格增量分片
    async fn get_incr_sharding_rate(&self, request: IncrRateRequest) -> REIncrRateResp {
        let res: ElongResponse<IncrRateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::ShardingRate, request)
            .await?;
        Ok(res)
    }

    /// 数据校验
    async fn data_validate(&self, request: DataValidateRequest) -> REDataValidateResp {
        let res: ElongResponse<DataValidateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::DataValidate, request)
            .await?;
        Ok(res)
    }

    /// 预订数据
    async fn data_booking(&self, request: DataBookingRequest) -> REDataBookingResp {
        let res: ElongResponse<DataBookingResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::DataBooking, request)
            .await?;
        Ok(res)
    }
}

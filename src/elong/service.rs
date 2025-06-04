use std::env;

use async_trait::async_trait;

use crate::{
    request::{
        data_booking::DataBookingRequest, data_inventory::InventoryRequest,
        data_rate::DataRateRequest, data_rp::DataRpRequest, data_validate::DataValidateRequest,
        dictionary::DictionaryRequest, incr_id::IncrIdRequest, incr_inv::IncrInvRequest,
        incr_order::IncrOrderRequest, incr_rate::IncrRateRequest, incr_state::IncrStateRequest,
        order_addinvoice::OrderAddinvoiceRequest, order_cancel::OrderCancelRequest,
        order_create::OrderCreateRequest, order_detail::OrderDetailRequest,
        order_feedback::OrderFeedbackRequest, order_list::OrderListRequest,
        order_pay::OrderPayRequest, order_pay_confirm::OrderPayConfirmRequest,
        order_promote::OrderPromoteRequest, order_related::OrderRelatedRequest,
        static_brand::StaticBrandRequest, static_city::*, static_grade::StaticGradeRequest,
        static_group::StaticGroupRequest, static_info::StaticInfoRequest,
        static_list::StaticListRequest,
    },
    response::{
        api_response::ElongResponse, data_booking::DataBookingResponse,
        data_inventory::InventoryResponse, data_rate::DataRateResponse, data_rp::DataRpResponse,
        data_validate::DataValidateResponse, dictionary::DictionaryResponse,
        incr_id::IncrIdResponse, incr_inv::IncrInvResponse, incr_order::IncrOrderResponse,
        incr_rate::IncrRateResponse, incr_state::IncrStateResponse,
        order_addinvoice::OrderAddinvoiceResponse, order_cancel::OrderCancelResponse,
        order_create::OrderCreateResponse, order_detail::OrderDetailReponse,
        order_feedback::OrderFeedbackResponse, order_list::OrderListResponse,
        order_pay::OrderPayResponse, order_pay_confirm::OrderPayConfirmResponse,
        order_promote::OrderPromoteResponse, order_related::OrderRelatedResponse,
        static_brand::StaticBrandResponse, static_city::*, static_grade::StaticGradeResponse,
        static_group::StaticGroupResponse, static_info::StaticInfoResponse,
        static_list::StaticListResponse,
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

    /// 点评评分
    async fn get_static_grade(&self, request: StaticGradeRequest) -> REGradeResp {
        let res: ElongResponse<StaticGradeResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticGrade, request)
            .await?;
        Ok(res)
    }

    /// 酒店品牌
    async fn get_static_brand(&self, request: StaticBrandRequest) -> REBrandResp {
        let res: ElongResponse<StaticBrandResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticBrand, request)
            .await?;
        Ok(res)
    }

    /// 酒店集团
    async fn get_static_group(&self, request: StaticGroupRequest) -> REGroupResp {
        let res: ElongResponse<StaticGroupResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::StaticGroup, request)
            .await?;
        Ok(res)
    }

    /// 酒店字典
    async fn get_hotel_dictionary(&self, request: DictionaryRequest) -> REDictionaryResp {
        let res: ElongResponse<DictionaryResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::Dictionary, request)
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

    /// 创建订单
    async fn order_create(&self, request: OrderCreateRequest) -> REOrderCreateResp {
        let res: ElongResponse<OrderCreateResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderCreate, request)
            .await?;
        Ok(res)
    }

    /// 订单支付
    async fn order_pay(&self, request: OrderPayRequest) -> REOrderPayResp {
        let res: ElongResponse<OrderPayResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderPay, request)
            .await?;
        Ok(res)
    }

    /// 订单支付确认
    async fn order_pay_confirm(&self, request: OrderPayConfirmRequest) -> REOrderPayConfirmResp {
        let res: ElongResponse<OrderPayConfirmResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderPayConfirm, request)
            .await?;
        Ok(res)
    }

    /// 订单增量
    async fn order_incr(&self, request: IncrOrderRequest) -> REIncrOrderResp {
        let res: ElongResponse<IncrOrderResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::IncrOrder, request)
            .await?;
        Ok(res)
    }

    /// 订单详情
    async fn order_detail(&self, request: OrderDetailRequest) -> REOrderDetailResp {
        let res: ElongResponse<OrderDetailReponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderDetail, request)
            .await?;
        Ok(res)
    }

    /// 取消订单
    async fn order_cancel(&self, request: OrderCancelRequest) -> REOrderCancelResp {
        let res: ElongResponse<OrderCancelResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderCancel, request)
            .await?;
        Ok(res)
    }

    /// 订单崔确认
    async fn order_promote(&self, request: OrderPromoteRequest) -> REOrderPromoteResp {
        let res: ElongResponse<OrderPromoteResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderPromote, request)
            .await?;
        Ok(res)
    }

    /// 关联订单
    async fn order_related(&self, request: OrderRelatedRequest) -> REOrderRelatedResp {
        let res: ElongResponse<OrderRelatedResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderRelated, request)
            .await?;
        Ok(res)
    }

    /// 入住反馈
    async fn order_feedback(&self, request: OrderFeedbackRequest) -> REOrderFeedbackResp {
        let res: ElongResponse<OrderFeedbackResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderFeedback, request)
            .await?;
        Ok(res)
    }

    /// 补开发票
    async fn order_addinvoice(&self, request: OrderAddinvoiceRequest) -> REOrderAddinvoiceResp {
        let res: ElongResponse<OrderAddinvoiceResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderAddinvoice, request)
            .await?;
        Ok(res)
    }

    /// 订单列表
    async fn order_list(&self, request: OrderListRequest) -> REOrderListResp {
        let res: ElongResponse<OrderListResponse> = self
            .client
            .fetch_data(&self.url, ApiMethod::OrderList, request)
            .await?;
        Ok(res)
    }
}

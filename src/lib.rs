//! # 艺龙 SDK
//!
//! 这是一个用于与艺龙交互的 SDK，提供了多种异步接口以获取静态和动态数据。
//!
//! ## 模块概览
//!
//! - `elong`: 提供与 Elong 平台交互的具体实现。
//! - `network`: 处理网络请求的底层模块。
//! - `request`: 定义了所有请求类型。
//! - `response`: 定义了所有响应类型。
//! - `types`: 定义了通用的数据类型和结构。
//!
//! ## Trait: Elong
//!
//! `Elong` trait 定义了与 Elong 平台交互的核心接口，所有方法均为异步方法。
//!
//!

use async_trait::async_trait;
use request::{
    data_booking::DataBookingRequest, data_inventory::InventoryRequest, data_rate::DataRateRequest,
    data_rp::DataRpRequest, data_validate::DataValidateRequest, dictionary::DictionaryRequest,
    incr_id::IncrIdRequest, incr_inv::IncrInvRequest, incr_rate::IncrRateRequest,
    incr_state::IncrStateRequest, order_create::OrderCreateRequest,
    static_brand::StaticBrandRequest, static_city::StaticCityRequest,
    static_grade::StaticGradeRequest, static_group::StaticGroupRequest,
    static_info::StaticInfoRequest, static_list::StaticListRequest,
};

use types::*;

use crate::request::{
    exchangerate::ExchangerateRequest, hotel_detail_request::HotelDetailRequest,
    incr_order::IncrOrderRequest, order_addinvoice::OrderAddinvoiceRequest,
    order_cancel::OrderCancelRequest, order_detail::OrderDetailRequest,
    order_feedback::OrderFeedbackRequest, order_list::OrderListRequest, order_pay::OrderPayRequest,
    order_pay_confirm::OrderPayConfirmRequest, order_promote::OrderPromoteRequest,
    order_related::OrderRelatedRequest,
};

pub mod elong;
mod network;
pub mod request;
pub mod response;
mod types;

#[async_trait]
pub trait Elong {
    /// 城市列表
    async fn get_static_city(&self, req: StaticCityRequest) -> RECityResp;

    /// 酒店列表
    async fn get_static_list(&self, req: StaticListRequest) -> REListResp;

    /// 酒店详情
    async fn get_static_info(&self, req: StaticInfoRequest) -> REInfoResp;

    /// 点评评分
    async fn get_static_grade(&self, req: StaticGradeRequest) -> REGradeResp;

    /// 酒店品牌
    async fn get_static_brand(&self, req: StaticBrandRequest) -> REBrandResp;

    /// 酒店集团
    async fn get_static_group(&self, req: StaticGroupRequest) -> REGroupResp;

    /// 酒店字典
    async fn get_hotel_dictionary(&self, req: DictionaryRequest) -> REDictionaryResp;

    /// 产品详情
    async fn get_data_rp(&self, req: DataRpRequest) -> REDataRpResp;

    /// 状态增量
    async fn get_incr_state(&self, req: IncrStateRequest) -> REIncrStateResp;

    /// 库存全量
    async fn get_inventory(&self, req: InventoryRequest) -> REInvResp;

    /// 库存增量
    async fn get_incr_inv(&self, req: IncrInvRequest) -> REIncrInvResp;

    /// 库存增量分片
    async fn get_incr_sharding_inv(&self, req: IncrInvRequest) -> REIncrInvResp;

    /// 价格全量
    async fn get_data_rate(&self, req: DataRateRequest) -> RERateResp;

    /// 价格增量
    async fn get_incr_rate(&self, req: IncrRateRequest) -> REIncrRateResp;

    /// 价格增量分片
    async fn get_incr_sharding_rate(&self, req: IncrRateRequest) -> REIncrRateResp;

    /// 增量编号
    async fn get_incr_id(&self, req: IncrIdRequest) -> REIncrIdResp;

    /// 增量编号分片
    async fn get_incr_sharding_id(&self, req: IncrIdRequest) -> REIncrIdResp;

    /// 状态增量分片
    async fn get_incr_sharding_state(&self, req: IncrStateRequest) -> REIncrStateResp;

    /// 数据验证
    async fn data_validate(&self, req: DataValidateRequest) -> REDataValidateResp;

    /// 预订数据
    async fn data_booking(&self, req: DataBookingRequest) -> REDataBookingResp;

    ///  创建订单
    async fn order_create(&self, req: OrderCreateRequest) -> REOrderCreateResp;

    /// 订单支付
    async fn order_pay(&self, req: OrderPayRequest) -> REOrderPayResp;

    /// 订单支付确认
    async fn order_pay_confirm(&self, req: OrderPayConfirmRequest) -> REOrderPayConfirmResp;

    /// 订单增量
    async fn order_incr(&self, req: IncrOrderRequest) -> REIncrOrderResp;

    /// 订单详情
    async fn order_detail(&self, req: OrderDetailRequest) -> REOrderDetailResp;

    /// 取消订单
    async fn order_cancel(&self, req: OrderCancelRequest) -> REOrderCancelResp;

    /// 订单崔确认
    async fn order_promote(&self, req: OrderPromoteRequest) -> REOrderPromoteResp;

    /// 关联订单
    async fn order_related(&self, req: OrderRelatedRequest) -> REOrderRelatedResp;

    /// 入住反馈
    async fn order_feedback(&self, req: OrderFeedbackRequest) -> REOrderFeedbackResp;

    /// 补开发票
    async fn order_addinvoice(&self, req: OrderAddinvoiceRequest) -> REOrderAddinvoiceResp;

    /// 订单列表
    async fn order_list(&self, req: OrderListRequest) -> REOrderListResp;

    /// 汇率
    async fn exchangerate(&self, req: ExchangerateRequest) -> REExchangeRateResp;

    /// 酒店详情搜索
    async fn hotel_detail(&self, req: HotelDetailRequest) -> REHotelDetailResp;
}

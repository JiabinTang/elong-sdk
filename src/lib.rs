//! # 艺龙离线数据 SDK
//!
//! 这是一个用于与艺龙离线数据交互的 SDK，提供了多种异步接口以获取静态和动态数据。
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
    data_inventory::InventoryRequest, data_rate::DataRateRequest, data_rp::DataRpRequest, data_validate::DataValidateRequest, incr_id::IncrIdRequest, incr_inv::IncrInvRequest, incr_rate::IncrRateRequest, incr_state::IncrStateRequest, static_city::StaticCityRequest, static_info::StaticInfoRequest, static_list::StaticListRequest
};

use types::*;

mod network;
mod types;
pub mod elong;
pub mod request;
pub mod response;

#[async_trait]
pub trait Elong {
    /// 城市列表
    async fn get_static_city(&self, req: StaticCityRequest) -> RECityResp;

    /// 酒店列表
    async fn get_static_list(&self, req: StaticListRequest) -> REListResp;

    /// 酒店详情
    async fn get_static_info(&self, req: StaticInfoRequest) -> REInfoResp;

    /// 产品详情
    async fn get_data_rp(&self, req: DataRpRequest) -> REDataRpResp;

    /// 增量编号
    async fn get_incr_id(&self, req: IncrIdRequest) -> REIncrIdResp;

    /// 增量编号分片
    async fn get_incr_sharding_id(&self, req: IncrIdRequest) -> REIncrIdResp;

    /// 状态增量
    async fn get_incr_state(&self, req: IncrStateRequest) -> REIncrStateResp;

    /// 状态增量分片
    async fn get_incr_sharding_state(&self, req: IncrStateRequest) -> REIncrStateResp;

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

    /// 数据验证
    async fn data_validate(&self, req: DataValidateRequest) -> REDataValidateResp;
}

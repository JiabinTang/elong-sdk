use async_trait::async_trait;
use request::{
    data_inventory::InventoryRequest, data_rate::DataRateRequest, data_rp::DataRpRequest, incr_id::IncrIdRequest, incr_inv::IncrInvRequest, incr_rate::IncrRateRequest, static_city::StaticCityRequest, static_info::StaticInfoRequest, static_list::StaticListRequest
};

use types::*;

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

    /// 产品详情
    async fn get_data_rp(&self, req: DataRpRequest) -> REDataRpResp;

    /// 增量编号
    async fn get_incr_id(&self, req: IncrIdRequest) -> REIncrIdResp;

    /// 增量编号分片
    async fn get_incr_sharding_id(&self, req: IncrIdRequest) -> REIncrIdResp;

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
}

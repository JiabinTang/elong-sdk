use async_trait::async_trait;
use elong::error::ElongError;
use request::{
    data_inventory::InventoryRequest, data_rate::DataRateRequest, incr_inv::IncrInvRequest, incr_rate::IncrRateRequest, static_city::StaticCityRequest, static_info::StaticInfoRequest, static_list::StaticListRequest
};
use response::{
    api_response::ElongResponse, data_inventory::InventoryResponse, data_rate::DataRateResponse,
    incr_inv::IncrInvResponse, incr_rate::IncrRateResponse, static_city::*,
    static_info::StaticInfoResponse, static_list::StaticListResponse,
};

pub mod elong;
mod network;
pub mod request;
pub mod response;

type ElongResult<T> = Result<ElongResponse<T>, ElongError>;

#[async_trait]
pub trait Elong {
    /// 静态数据 - 城市列表
    async fn get_static_city(&self, req: StaticCityRequest) -> ElongResult<StaticCityResponse>;

    /// 静态数据 - 酒店列表
    async fn get_static_list(&self, req: StaticListRequest) -> ElongResult<StaticListResponse>;

    /// 静态数据 - 酒店详情
    async fn get_static_info(&self, req: StaticInfoRequest) -> ElongResult<StaticInfoResponse>;

    /// 静态数据 - 库存全量
    async fn get_inventory(&self, req: InventoryRequest) -> ElongResult<InventoryResponse>;

    /// 静态数据 - 库存增量
    async fn get_incr_inv(&self, req: IncrInvRequest) -> ElongResult<IncrInvResponse>;

    /// 静态数据 - 库存增量切片
    async fn get_incr_sharding_inv(&self, req: IncrInvRequest) -> ElongResult<IncrInvResponse>;

    /// 静态数据 - 价格全量
    async fn get_data_rate(&self, req: DataRateRequest) -> ElongResult<DataRateResponse>;

    /// 静态数据 - 价格增量
    async fn get_incr_rate(&self, req: IncrRateRequest) -> ElongResult<IncrRateResponse>;
}

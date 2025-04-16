use async_trait::async_trait;
use elong::error::ElongError;
use request::{data_inventory::DataInventoryRequest, static_city::StaticCityRequest, static_info::StaticInfoRequest, static_list::StaticListRequest};
use response::{api_response::ElongResponse, data_inventory::DataInventoryResponse, static_city::*, static_info::StaticInfoResponse, static_list::StaticListResponse};

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
    async fn get_data_inventory(&self, req: DataInventoryRequest) -> ElongResult<DataInventoryResponse>;
}

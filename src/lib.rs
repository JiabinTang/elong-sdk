use async_trait::async_trait;
use elong::error::ElongError;
use request::{static_city::StaticCityRequest, static_info::StaticInfoRequest, static_list::StaticListRequest};
use response::{api_response::ElongResponse, static_city::*, static_info::StaticInfoResponse, static_list::StaticListResponse};

pub mod elong;
mod network;
pub mod request;
pub mod response;

type ElongResult<T> = Result<ElongResponse<T>, ElongError>;

#[async_trait]
pub trait Elong {
    async fn get_static_city(&self, req: StaticCityRequest) -> ElongResult<StaticCityResponse>;

    async fn get_static_list(&self, req: StaticListRequest) -> ElongResult<StaticListResponse>;

    async fn get_static_info(&self, req: StaticInfoRequest) -> ElongResult<StaticInfoResponse>;
}

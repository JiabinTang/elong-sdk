use crate::elong::error::ElongError;
use crate::response::data_booking::DataBookingResponse;
use crate::response::data_rp::DataRpResponse;
use crate::response::data_validate::DataValidateResponse;
use crate::response::incr_id::IncrIdResponse;
use crate::response::incr_state::IncrStateResponse;
use crate::response::static_grade::StaticGradeResponse;
use crate::response::{
    api_response::ElongResponse, data_inventory::InventoryResponse, data_rate::DataRateResponse,
    incr_inv::IncrInvResponse, incr_rate::IncrRateResponse, static_city::StaticCityResponse,
    static_info::StaticInfoResponse, static_list::StaticListResponse,
};

// 响应体别名
pub type RECityResp = Result<ElongResponse<StaticCityResponse>, ElongError>;
pub type REListResp = Result<ElongResponse<StaticListResponse>, ElongError>;
pub type REInfoResp = Result<ElongResponse<StaticInfoResponse>, ElongError>;
pub type REGradeResp = Result<ElongResponse<StaticGradeResponse>, ElongError>;
pub type REDataRpResp = Result<ElongResponse<DataRpResponse>, ElongError>;
pub type REIncrIdResp = Result<ElongResponse<IncrIdResponse>, ElongError>;
pub type REIncrStateResp = Result<ElongResponse<IncrStateResponse>, ElongError>;
pub type REInvResp = Result<ElongResponse<InventoryResponse>, ElongError>;
pub type REIncrInvResp = Result<ElongResponse<IncrInvResponse>, ElongError>;
pub type RERateResp = Result<ElongResponse<DataRateResponse>, ElongError>;
pub type REIncrRateResp = Result<ElongResponse<IncrRateResponse>, ElongError>;
pub type REDataValidateResp = Result<ElongResponse<DataValidateResponse>, ElongError>;
pub type REDataBookingResp = Result<ElongResponse<DataBookingResponse>, ElongError>;

use crate::elong::error::ElongError;
use crate::response::order_create::OrderCreateResponse;
use crate::response::{
    api_response::ElongResponse, data_booking::DataBookingResponse,
    data_inventory::InventoryResponse, data_rate::DataRateResponse, data_rp::DataRpResponse,
    data_validate::DataValidateResponse, dictionary::DictionaryResponse, incr_id::IncrIdResponse,
    incr_inv::IncrInvResponse, incr_rate::IncrRateResponse, incr_state::IncrStateResponse,
    static_brand::StaticBrandResponse, static_city::StaticCityResponse,
    static_grade::StaticGradeResponse, static_group::StaticGroupResponse,
    static_info::StaticInfoResponse, static_list::StaticListResponse,
};

pub type RECityResp = Result<ElongResponse<StaticCityResponse>, ElongError>;
pub type REListResp = Result<ElongResponse<StaticListResponse>, ElongError>;
pub type REInfoResp = Result<ElongResponse<StaticInfoResponse>, ElongError>;
pub type REGradeResp = Result<ElongResponse<StaticGradeResponse>, ElongError>;
pub type REBrandResp = Result<ElongResponse<StaticBrandResponse>, ElongError>;
pub type REGroupResp = Result<ElongResponse<StaticGroupResponse>, ElongError>;
pub type REDataRpResp = Result<ElongResponse<DataRpResponse>, ElongError>;
pub type REIncrIdResp = Result<ElongResponse<IncrIdResponse>, ElongError>;
pub type REIncrStateResp = Result<ElongResponse<IncrStateResponse>, ElongError>;
pub type REInvResp = Result<ElongResponse<InventoryResponse>, ElongError>;
pub type REIncrInvResp = Result<ElongResponse<IncrInvResponse>, ElongError>;
pub type RERateResp = Result<ElongResponse<DataRateResponse>, ElongError>;
pub type REIncrRateResp = Result<ElongResponse<IncrRateResponse>, ElongError>;
pub type REDataValidateResp = Result<ElongResponse<DataValidateResponse>, ElongError>;
pub type REDataBookingResp = Result<ElongResponse<DataBookingResponse>, ElongError>;
pub type REDictionaryResp = Result<ElongResponse<DictionaryResponse>, ElongError>;
pub type REOrderCreateResp = Result<ElongResponse<OrderCreateResponse>, ElongError>;

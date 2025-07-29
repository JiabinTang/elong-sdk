use crate::elong::error::ElongError;

use crate::response::{
    api_response::ElongResponse, data_booking::DataBookingResponse,
    data_inventory::InventoryResponse, data_rate::DataRateResponse, data_rp::DataRpResponse,
    data_validate::DataValidateResponse, dictionary::DictionaryResponse,
    exchangerate::ExchangerateResponse, hotel_detail_response::HotelDetailResponse,
    hotel_rate_min_response::HotelRateMinResponse, incr_id::IncrIdResponse,
    incr_inv::IncrInvResponse, incr_order::IncrOrderResponse, incr_rate::IncrRateResponse,
    incr_state::IncrStateResponse, order_addinvoice::OrderAddinvoiceResponse,
    order_cancel::OrderCancelResponse, order_create::OrderCreateResponse,
    order_detail::OrderDetailReponse, order_feedback::OrderFeedbackResponse,
    order_list::OrderListResponse, order_pay::OrderPayResponse,
    order_pay_confirm::OrderPayConfirmResponse, order_promote::OrderPromoteResponse,
    order_related::OrderRelatedResponse, static_brand::StaticBrandResponse,
    static_city::StaticCityResponse, static_grade::StaticGradeResponse,
    static_group::StaticGroupResponse, static_info::StaticInfoResponse,
    static_list::StaticListResponse,
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
pub type REOrderPayResp = Result<ElongResponse<OrderPayResponse>, ElongError>;
pub type REOrderPayConfirmResp = Result<ElongResponse<OrderPayConfirmResponse>, ElongError>;
pub type REIncrOrderResp = Result<ElongResponse<IncrOrderResponse>, ElongError>;
pub type REOrderDetailResp = Result<ElongResponse<OrderDetailReponse>, ElongError>;
pub type REOrderCancelResp = Result<ElongResponse<OrderCancelResponse>, ElongError>;
pub type REOrderPromoteResp = Result<ElongResponse<OrderPromoteResponse>, ElongError>;
pub type REOrderRelatedResp = Result<ElongResponse<OrderRelatedResponse>, ElongError>;
pub type REOrderFeedbackResp = Result<ElongResponse<OrderFeedbackResponse>, ElongError>;
pub type REOrderAddinvoiceResp = Result<ElongResponse<OrderAddinvoiceResponse>, ElongError>;
pub type REOrderListResp = Result<ElongResponse<OrderListResponse>, ElongError>;
pub type REExchangeRateResp = Result<ElongResponse<ExchangerateResponse>, ElongError>;
pub type REHotelDetailResp = Result<ElongResponse<HotelDetailResponse>, ElongError>;
pub type REHotelRateMinResp = Result<ElongResponse<HotelRateMinResponse>, ElongError>;

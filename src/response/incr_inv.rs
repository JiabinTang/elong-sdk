use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncrInvResponse {
    ///库存集合
    pub inventories: Vec<Inventory>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Inventory {
    /// LastId 增量ID Long N 
    pub last_id: i64,
    /// Time 变化时间 Datetime N 
    pub time: String,
    /// HotelID 酒店ID String(8) N 这几个属性是业务主键。 HotelCode关联搜索接口的RatePlan.HotelCode Date表示的是某天的库存。
    #[serde(rename = "HotelID")]
    pub hotel_id: String,
    /// RoomTypeId 房型ID String(10) N
    pub room_type_id: String,
    /// HotelCode 酒店编码 String(8) N
    pub hotel_code: String,
    /// Date 库存时间 Date N
    pub date: String,
    /// Status 库存状态 Boolean N False-不可用 True-可用
    pub status: bool,
    /// Amount 库存数量 Int N 剩余的可知库存数量
    pub amount: i32,
    /// OverBooking 超售状态 Int N 0---可超售，1—不可超售。可超售的时候即使Amount等于0也是可以继续销售的。
    pub over_booking: i32,
    /// StartDate 可用开始日期 Date N 库存可用开始日期
    pub start_date: String,
    /// EndDate 可用结束日期 Date N 库存可用结束日期
    pub end_date: String,
    /// StartTime 可用开始时间 Time N 预订当天库存，须校验库存可用开始时间(错误,见新校验逻辑)
    pub start_time: String,
    /// EndTime 可用结束时间 Time N 预订当天库存，须校验库存可用结束时间; 若为23:59:59则为无限制;(错误,见新校验逻辑)
    pub end_time: String,
    /// IsInstantConfirm 库存是否支持即时确认 Boolean Y V1.22新增,具体使用请见接口使用说明
    pub is_instant_confirm: Option<bool>,
    /// IC_BeginTime 即时确认可用开始时间 Time Y
    #[serde(rename = "IC_BeginTime")]
    pub ic_begin_time: Option<String>,
    /// IC_EndTime 即时确认可用结束时间 Time Y
    #[serde(rename = "IC_EndTime")]
    pub ic_end_time: Option<String>,
}

impl BaseResponse for ElongResponse<IncrInvResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<IncrInvResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

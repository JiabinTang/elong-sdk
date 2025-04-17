use crate::elong::error::ElongError;
use serde::{Deserialize, Serialize};

use super::api_request::BaseRequest;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct InventoryRequest {
    /// 酒店编号，最多10个，逗号分隔
    pub hotel_ids: String,
    /// 酒店编码，最多10个，逗号分隔。如果输入这个参数，请确保这些HotelCodes都是HotelIds(只能输入一个)所属的
    pub hotel_codes: Option<String>,
    /// 供应商房型编号，V1.10后对应搜索接口中的RatePlan.RoomTypeId
    pub room_type_id: Option<String>,
    /// 开始日期，使用yyyy-MM-dd格式，例如:2022-12-09
    pub start_date: String,
    /// 结束日期，使用yyyy-MM-dd格式，例如:2022-12-09
    pub end_date: String,
    /// 是否返回即时确认数据，建议不使用，返回速度会变慢
    pub is_need_instant_confirm: Option<bool>,
}

impl BaseRequest for InventoryRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

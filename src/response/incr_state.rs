use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncrStateResponse {
    /// States 变化集合 State[] Y
    /// 包含多个State节点
    pub states: Vec<State>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    /// LastId 增量Id Long N
    pub last_id: i64,
    /// Time 变化时间 DateTime N
    pub time: String,
    /// HotelId 酒店编号 String(8) Y
    pub hotel_id: Option<String>,
    /// HotelCode 酒店供应商编码 String(8) Y
    pub hotel_code: Option<String>,
    /// RoomId 展示房型ID String(8) Y
    pub room_id: Option<String>,
    /// RoomTypeId 销售房型编号 String(8) Y
    pub room_type_id: Option<String>,
    /// RatePlanId 产品编号 Int Y
    pub rate_plan_id: Option<String>,
    /// Name 对象的名称 String(100) Y
    pub name: Option<String>,
    /// Status 有效状态 Boolean Y
    /// 当StateType为RatePlanPolicy的时候Status无意义
    pub status: Option<bool>,
    /// StateType 变化类型 Enum N
    /// HotelId：酒店 HotelCode：酒店供应商 RoomId：展示房型
    /// RoomTypeId：销售房型 RatePlanId：产品 RatePlanPolicy：担保或预付规则
    pub state_type: String,
}

impl BaseResponse for ElongResponse<IncrStateResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<IncrIdResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

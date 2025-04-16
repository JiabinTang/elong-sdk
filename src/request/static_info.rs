use crate::elong::error::ElongError;
use serde::{Deserialize, Serialize};

use super::api_request::BaseRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct StaticInfoRequest {
    /// 酒店ID，注意：hotel.static.list返回酒店状态HotelStatus非有效时，无需再请求该接口
    #[serde(rename = "HotelId")]
    pub hotel_id: String,
    /// 其他条件，英文逗号分割，当Version大于等于1.6时必填
    /// 例如 1,5 ：返回Detail节点和TelList节点
    #[serde(rename = "Options", skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
}

impl BaseRequest for StaticInfoRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

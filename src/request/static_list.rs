use crate::elong::error::ElongError;

use super::api_request::BaseRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StaticListRequest {
    /// 开始时间，格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式：yyyy-MM-dd HH:mm:ss
    #[serde(rename = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 城市ID，只能传入1个
    #[serde(rename = "CityId")]
    pub city_id: String,
    /// 每页数据量，默认值：2000
    #[serde(rename = "PageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页码，从1开始
    #[serde(rename = "PageIndex", skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i32>,
}

impl BaseRequest for StaticListRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

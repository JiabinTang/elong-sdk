use super::api_request::BaseRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|err| {
            log::error!("Failed to serialize StaticListRequest: {}", err);
            format!(
                r#"{{"StartTime":"{}","EndTime":"{}","CityId":"{}","PageSize":{},"PageIndex":{}}}"#,
                self.start_time.as_deref().unwrap_or(""),
                self.end_time.as_deref().unwrap_or(""),
                self.city_id,
                self.page_size.unwrap_or(2000),
                self.page_index.unwrap_or(1)
            )
        })
    }
}

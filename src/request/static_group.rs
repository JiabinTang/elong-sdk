use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StaticGroupRequest {
    /// 状态：不传时返回所有；0有效，1无效
    pub status: Option<i32>, // 可选字段
    /// 每页数据量，默认值为 200
    pub page_size: i32, // 可选字段
    /// 页码，从 1 开始
    pub page_index: i32, // 可选字段
}

impl BaseRequest for StaticGroupRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

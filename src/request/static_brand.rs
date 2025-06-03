use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StaticBrandRequest {
    /// Status 状态 Int Y 不传时返回所有；0有效 1无效
    pub status: Option<i32>,
    /// PageSize 每页数据量 Int N 默认：200
    pub page_size: i32,
    /// PageIndex 页码 Int N 从1开始
    pub page_index: i32,
}

impl BaseRequest for StaticBrandRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

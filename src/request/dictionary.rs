use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DictionaryRequest {
    /// 字典数据类型。1：酒店标签；2：酒店设施；3：酒店主题
    pub r#type: i32,
    /// 分页页数，大于 0
    pub page: i32,
    /// 分页大小，最小 10，最大 1000
    pub limit: i32,
}

impl BaseRequest for DictionaryRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct IncrOrderRequest {
    /// 最后的更新ID
    pub last_id: i64,

    /// 抓取的数量
    /// 不传时默认值为 1000；最大不能超过 5000
    pub count: Option<i32>,
}

impl BaseRequest for IncrOrderRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

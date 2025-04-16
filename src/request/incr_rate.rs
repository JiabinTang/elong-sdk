use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncrRateRequest {
    /// LastId 最后的更新ID Long N
    pub last_id: i64,
    /// ShardingKey 分片Key String(8) N 传入的分片Key必须是当前分片的Key，不能是其他分片的Key
    pub sharding_key: i32,
    /// Count 抓取的数量 Integer Y 不传，默认：1000；最大不能超过5000
    pub count: Option<i32>,
}

impl BaseRequest for IncrRateRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

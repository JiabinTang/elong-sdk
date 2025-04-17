use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct IncrIdRequest {
    /// LastTime 最后的更新时间 DateTime N
    /// 第一次同步参考全量开始同步的时间。要求格式为yyyy-MM-dd HH:mm:ss
    pub last_time: String,
    /// ShardingKey 分片键 Integer N
    /// 1-16，用于查询指定分片数据
    pub sharding_key: i32,
    /// IncrType 增量类型 Enum N
    /// State - 状态变化，对应 hotel.incr.sharding.state
    /// Inventory - 状态变化，对应 hotel.incr.sharding.inv
    /// Rate - 状态变化，对应 hotel.incr.sharding.rate
    pub incr_type: String,
}

impl BaseRequest for IncrIdRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

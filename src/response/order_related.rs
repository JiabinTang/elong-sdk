use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderRelatedResponse {
    /// 订单关系
    /// 包含多个 Relation 节点，不存在指定关系的不返回到结果中
    pub relations: Option<Vec<Relation>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Relation {
    /// 父订单
    pub parent_id: i64,

    /// 子订单
    pub child_id: i64,
}

impl BaseResponse for ElongResponse<OrderRelatedResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderRelatedResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

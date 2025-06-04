use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderRelatedRequest {
    /// 订单编号
    /// 支持多个订单号查询，最多10个，以逗号分隔
    pub order_ids: String,

    /// 关联类型
    /// Child: 根据原来订单号查询新生成的订单
    /// Parent: 根据新生成的订单号查询原来订单
    pub relation_type: String,
}

impl BaseRequest for OrderRelatedRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

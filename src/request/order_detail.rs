use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderDetailRequest {
    /// 订单编号
    /// 如果 OrderId 不为 0，以 OrderId 为主
    pub order_id: i64,

    /// 联盟的订单编号
    /// 当 OrderId = 0 的时候，则按 AffiliateConfirmationId 查询
    pub affiliate_confirmation_id: Option<String>,

    /// 其他条件
    /// 多个英文逗号分隔，例如返回 SpecialCancelApply
    pub options: Option<String>,
}

impl BaseRequest for OrderDetailRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

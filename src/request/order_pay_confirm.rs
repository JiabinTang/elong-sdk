use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderPayConfirmRequest {
    /// 订单号
    pub order_id: i64,

    /// 短信验证码
    /// 信用卡支付使用此属性
    pub sms_code: String,

    /// 支付金额（人民币价格）
    pub amount: f64,
}

impl BaseRequest for OrderPayConfirmRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, response::api_response::{BaseResponse, ElongResponse}};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderPayConfirmResponse {
    /// 支付请求是否接收成功
    /// 该字段表示艺龙分销系统是否成功接收到了支付请求，不代表支付成功。
    pub is_success: bool,

    /// 备注（失败原因）
    pub notes: Option<String>,
}

impl BaseResponse for ElongResponse<OrderPayConfirmResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderPayConfirmResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}
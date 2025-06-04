use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangerateRequest {
    /// 货币代码
    /// 可选值：RMB, HKD, MOP, TWD
    pub currency_id: String,
}

impl BaseRequest for ExchangerateRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

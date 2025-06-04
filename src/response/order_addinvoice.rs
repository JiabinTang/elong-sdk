use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderAddinvoiceResponse {
    /// 补开发票是否提交成功
    pub success: bool,
}

impl BaseResponse for ElongResponse<OrderAddinvoiceResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderAddinvoiceResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

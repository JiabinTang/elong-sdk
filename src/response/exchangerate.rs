use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangerateResponse {
    /// 外币对应人民币的汇率
    /// 如港币的是 0.79
    pub exchange_rate: f64,
}

impl BaseResponse for ElongResponse<ExchangerateResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<ExchangerateResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

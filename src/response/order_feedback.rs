use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderFeedbackResponse {
    /// 错误信息
    /// 具体的错误信息；成功的时候为空
    pub error_message: Option<String>,
}

impl BaseResponse for ElongResponse<OrderFeedbackResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderFeedbackResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

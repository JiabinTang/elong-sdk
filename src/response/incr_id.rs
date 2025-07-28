use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IncrIdResponse {
    /// LastId 最后的更新ID Long N
    pub last_id: i64,
}

impl BaseResponse for ElongResponse<IncrIdResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<IncrIdResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

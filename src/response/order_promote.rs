use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderPromoteResponse {
    /// 调整后的反馈时间点
    /// 格式：hh:mm (如果早于当前时间则表示第二天的时间点)
    pub adjust_time: Option<String>,
}

impl BaseResponse for ElongResponse<OrderPromoteResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderPromoteResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderCancelResponse {
    /// 取消请求处理结果
    /// 注意，是三个 s。
    /// 返回 true 表示艺龙已经收到了请求，是否能够退款需要查看订单详情中的 refundDetail 节点。
    pub successs: bool,

    /// 取消罚金
    /// 取消罚金值
    pub penalty_amount: Option<f64>,
}

impl BaseResponse for ElongResponse<OrderCancelResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderCancelResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

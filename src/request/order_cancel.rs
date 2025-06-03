use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderCancelRequest {
    /// 订单编号
    pub order_id: i64,

    /// 取消类型
    /// 示例: 对酒店相关条件不满意、航班推迟、价格过高，客人不接受等
    pub cancel_code: String,

    /// 具体原因
    pub reason: Option<String>,

    /// 取消罚金
    /// 0：默认值，不做取消罚金校验，能取消就取消，有罚金也取消，以艺龙接口计算的罚金为准
    /// -1：若取消需要收取罚金就不取消，无罚金则取消
    /// 大于0：校验取消罚金，需要与艺龙的罚金相等才取消，否则拒绝取消
    pub penalty_amount: Option<f64>,
}

impl BaseRequest for OrderCancelRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

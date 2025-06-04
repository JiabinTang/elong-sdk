use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct IncrOrderResponse {
    /// 订单增量
    /// 包含多个 Order 节点
    pub orders: Option<Vec<Order>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    /// 增长ID
    pub last_id: i64,

    /// 变化时间
    pub time: String,

    /// 订单ID
    pub order_id: i64,

    /// 订单状态
    pub status: String,

    /// 入住日期
    pub arrival_date: String,

    /// 离店日期
    pub departure_date: String,

    /// 总价
    pub total_price: f64,

    /// 房间数量
    pub number_of_rooms: i32,

    /// 合作伙伴从成单接口传入的订单号
    pub affiliate_confirmation_id: Option<String>,

    /// 订单退款
    pub all_refund_amount: Option<f64>,

    /// 支付状态
    /// -1 -- 无支付信息
    /// 1 -- 等待担保或支付
    /// 2 -- 担保或支付中
    /// 3 -- 担保或支付（或退款）成功
    /// 4 -- 担保或支付（或退款）失败
    /// 5 -- 暂缓
    pub pay_status: Option<i32>,

    /// 是否即时确认
    /// 此字段为 true 时，只要订单状态变成 V，就代表着订单被确认了。
    pub is_instant_confirm: Option<bool>,
}

impl BaseResponse for ElongResponse<IncrOrderResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<IncrOrderResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

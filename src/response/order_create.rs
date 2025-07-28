use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderCreateResponse {
    /// 订单编号
    pub order_id: i64,
    /// 最晚取消时间。如果日期为 9999-12-30 23:00:00 等条件代表不限制取消时间
    pub cancel_time: String,
    /// 担保金额。如果此订单是担保订单，则在此列出担保金额，币种是人民币
    pub guarantee_amount: Option<f64>,
    /// 货币类型
    pub currency_code: Option<String>,
    /// 是否是即时确认
    pub is_instant_confirm: Option<bool>,
    /// 支付最后期限。如果担保预付订单支付失败，系统保留一段时间继续支付
    pub payment_deadline_time: Option<String>,
    /// 支付错误信息
    pub payment_message: Option<String>,
}

impl BaseResponse for ElongResponse<OrderCreateResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderCreateResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    request::{
        api_request::BaseRequest,
        order_create::{CreditCard, DoveCorpCard},
    },
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderPayRequest {
    /// 订单号
    pub order_id: i64,

    /// 是否已担保或已付款
    /// 开通了公司担保业务的合作伙伴才能使用该属性
    pub is_guarantee_or_charged: bool,

    /// 信用卡信息
    /// 担保订单和预付订单才须传信用卡
    pub credit_card: Option<CreditCard>,

    /// 第三方支付信息
    /// 担保订单和预付订单才须传第三方支付信息。
    /// 如果 is_guarantee_or_charged 为 false，则不能传第三方支付信息。
    pub dove_corp_card: Option<DoveCorpCard>,

    /// 支付金额（人民币价格）
    pub amount: f64,
}

impl BaseRequest for OrderPayRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

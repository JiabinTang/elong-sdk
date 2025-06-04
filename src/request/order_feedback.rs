use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderFeedbackRequest {
    /// 订单编号
    pub order_id: i64,

    /// 入住日期
    pub arrival_date: Option<String>,

    /// 离店日期
    pub departure_date: Option<String>,

    /// 入住客人姓名
    /// 三者（OrderId、ArrivalDate、CustomerName）至少提供一个，多个请用逗号分割
    pub customer_name: Option<String>,

    /// 酒店的房间号
    pub room_number: Option<String>,

    /// 其他说明
    pub notes: Option<String>,
}

impl BaseRequest for OrderFeedbackRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

/// 酒店最小价请求结构体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct HotelRateMinRequest {
    /// 入住日期，格式: yyyy-MM-dd
    pub arrival_date: String,
    /// 离店日期，格式: yyyy-MM-dd
    pub departure_date: String,
    /// 酒店ID，只能输入一个酒店ID
    pub hotel_ids: String,
    /// 房型编号，可为空
    pub room_id: Option<String>,
    /// 支付方式，可为空，All-不限、SelfPay-现付、Prepay-预付
    pub payment_type: Option<String>,
    /// 成人数，国际特有字段
    pub number_of_adults: i32,
    /// 房间数量，国际特有字段,暂未使用
    pub number_of_rooms: Option<i32>,
    /// 儿童年龄，国际特有字段
    pub child_ages: Option<Vec<i32>>,
    /// 预付发票模式，可为空
    pub invoice_mode: Option<String>,
    /// 最晚到店时间，可为空，格式: yyyy-MM-dd HH:mm:ss
    pub latest_arrival_time: Option<String>,
    /// 其他条件，建议仅传入0获取酒店最小价信息
    pub options: Option<String>,
}

impl BaseRequest for HotelRateMinRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

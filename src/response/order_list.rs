use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderListResponse {
    /// 总订单数
    pub count: i32,

    /// 订单数组
    /// 包含多个 Order 节点
    pub orders: Option<Vec<Order>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    /// 订单编号
    pub order_id: i64,

    /// 状态
    pub status: String,

    /// 总价
    pub total_price: f64,

    /// 货币类型
    pub currency_code: String,

    /// 酒店编号
    pub hotel_id: String,

    /// 酒店名称（V1.03新增）
    pub hotel_name: Option<String>,

    /// 房型编号
    pub room_type_id: String,

    /// 房型名称（V1.03新增）
    pub room_type_name: Option<String>,

    /// 产品编号
    pub rate_plan_id: i32,

    /// 产品名称（V1.05新增）
    pub rate_plan_name: Option<String>,

    /// 入住日期
    pub arrival_date: String,

    /// 离店日期
    pub departure_date: String,

    /// 客人类型
    /// All=统一价, Chinese=内宾价, OtherForeign=外宾价, HongKong=港澳台客人价, ChinaGuest=中宾价
    pub customer_type: String,

    /// 房间数量
    pub number_of_rooms: i32,

    /// 客人数量
    pub number_of_customers: i32,

    /// 付款类型
    /// SelfPay=前台现付, Prepay=预付
    pub payment_type: String,

    /// 最早到店时间
    pub earliest_arrival_time: String,

    /// 最晚到店时间
    pub latest_arrival_time: String,

    /// 确认类型
    /// NotAllowedConfirm=不允许确认, SMS_cn=艺龙发短信给客人, NoNeed=艺龙发短信给客人但不主动联系
    pub confirmation_type: String,

    /// 给酒店备注
    pub note_to_hotel: Option<String>,

    /// 给艺龙备注
    pub note_to_elong: Option<String>,
}

impl BaseResponse for ElongResponse<OrderListResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderListResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

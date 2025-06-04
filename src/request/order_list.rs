use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderListRequest {
    /// 预定时间开始点，要求格式为 yyyy-MM-dd HH:mm:ss
    pub creation_time_from: Option<String>,

    /// 预定时间结束点，要求格式为 yyyy-MM-dd HH:mm:ss
    /// 预定时间范围尽量控制在一个月范围内，否则会超时
    pub creation_time_to: Option<String>,

    /// 酒店编号
    pub hotel_id: Option<String>,

    /// 房型编号
    pub room_type_id: Option<String>,

    /// 产品编号
    pub rate_plan_id: Option<i32>,

    /// 入住日期开始点，要求格式为 yyyy-MM-dd
    pub arrival_date_from: Option<String>,

    /// 入住日期结束点，要求格式为 yyyy-MM-dd
    pub arrival_date_to: Option<String>,

    /// 离店日期开始点，要求格式为 yyyy-MM-dd
    pub departure_date_from: Option<String>,

    /// 离店日期结束点，要求格式为 yyyy-MM-dd
    pub departure_date_to: Option<String>,

    /// 最后更新时间开始点，要求格式为 yyyy-MM-dd HH:mm:ss
    /// 2014-06-20新增
    pub min_update_time: Option<String>,

    /// 最后更新时间结束点，要求格式为 yyyy-MM-dd HH:mm:ss
    /// 2014-06-20新增
    pub max_update_time: Option<String>,

    /// 联系人手机
    pub mobile: Option<String>,

    /// 入住人姓名
    pub customer_name: Option<String>,

    /// 订单状态
    pub status: Option<String>,

    /// 分页页码，分页大小为10
    pub page_index: i32,
}

impl BaseRequest for OrderListRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

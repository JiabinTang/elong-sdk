use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct HotelDetailRequest {
    /// 入住日期，格式：yyyy-MM-dd，酒店当地时间
    pub arrival_date: String,
    /// 离店日期，格式：yyyy-MM-dd，酒店当地时间
    pub departure_date: String,
    /// 最晚到店时间，格式：yyyy-MM-dd HH:mm:ss，酒店当地时间
    pub latest_arrival_time: Option<String>,
    /// 支付方式，默认为 "All"，可选："All"（不限）、"SelfPay"（现付）、"Prepay"（预付），注意字母大小写
    pub payment_type: Option<String>,
    /// 预付发票模式，可选："Elong"（艺龙开发票）、"Hotel"（酒店开发票）、"NoSense"（全部），仅针对预付产品
    pub invoice_mode: Option<String>,
    /// 酒店ID列表，英文逗号分隔，中国大陆最多10个，非中国大陆支持1个
    pub hotel_ids: String,
    /// 房型编号，当 RatePlanId 传值时不能为空
    pub room_type_id: Option<String>,
    /// 产品编码
    pub rate_plan_id: Option<i64>,
    /// 成人数，仅用于国际及港澳台酒店
    pub number_of_adults: u32,
    /// 儿童年龄，仅用于国际及港澳台酒店
    pub child_ages: Option<Vec<u32>>,
    /// 房间数量，仅用于国际及港澳台酒店
    pub number_of_rooms: Option<u32>,
    /// 是否保存 Littlemajiaid，仅用于国际及港澳台酒店
    pub save_majia_id: Option<bool>,
    /// 其他条件，仅单酒店有效，可逗号分割
    pub options: Option<String>,
    /// 国籍，详见 Nat
    pub nat: Option<Vec<String>>,
}

impl BaseRequest for HotelDetailRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

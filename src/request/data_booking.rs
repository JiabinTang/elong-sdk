use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DataBookingRequest {
    /// ArrivalDate 入住日期 Date N 要求使用yyyy-MM-dd格式，例如:2022-12-09，其他格式不保证持续支持
    pub arrival_date: String,
    /// DepartureDate 离店日期 Date N 要求使用yyyy-MM-dd格式，例如:2022-12-09，其他格式不保证持续支持
    pub departure_date: String,
    /// HotelId 酒店编号 String(8) N
    pub hotel_id: String,
    /// HotelCode 酒店编码 String(8) N
    pub hotel_code: String,
    /// RoomId 展示房型编号 String(10) Y 允许为空，当传入时会校验房型编号绑定关系
    pub room_id: Option<String>,
    /// RoomTypeID 销售房型编码 String(10) N
    pub room_type_id: String,
    /// RatePlanId 产品编号 Int N
    pub rate_plan_id: i32,
    /// PaymentType 支付类型 Enum N SelfPay--前台自付   Prepay--预付
    pub payment_type: String,
    /// IsRatesWithDRR 返回的价格是否通过DRR计算 Boolean Y 默认为false；如果为true则返回的Rates节点里面的价格为DRR计算后的，false则为原始价格。促销产品调用时，需要透传该字段。
    pub is_rates_with_drr: Option<bool>,
    /// MethodType 调用监控 Int Y 正常用户调用不用传，批量调用传入32，方便艺龙方对接口进行监控。
    pub method_type: Option<i32>,
    /// LittleMajiaId 马甲Id String Y 从hotel.detail接口获取，用于促销，促销产品调用时，需要透传该字段；新接入搜索模式必传；建议历史已接入搜索模式分销商也将此字段全部传回；国内酒店马甲Id不再有30分钟限制 搜索模式所有产品必传
    pub little_majia_id: Option<String>,
    /// GoodsUniqId 商品唯一标示 String Y 从hotel.detail接口获取，用于促销，促销产品调用时，需要透传该字段；新接入搜索模式必传；建议历史已接入搜索模式分销商也将此字段全部传回 搜索模式所有产品必传
    pub goods_uniq_id: Option<String>,
    /// NumberOfRooms 房间数量 Int Y 当促销产品调用时，需要传入具体房间数量
    pub number_of_rooms: Option<i32>,
    /// EarliestArrivalTime 最早到店时间 DateTime Y 请传入最早到店时间和最晚到店时间（不传时默认逻辑是：当天预订时，当前时间>=23点，最早最晚到店时间赋值为当天的23点59分，当前时间<23点时，最早到店时间为当前时间+30分钟，最晚到店时间为当前时间+60分钟；非当天预订时，最早到店时间为入住日的14点，最晚到店时间为入住日的15点）。要求使用yyyy-MM-dd HH:mm:ss格式，注意最早到店时间需要在hotel.static.info接口的ArrivalTime和LatestArrivalTime之间
    pub earliest_arrival_time: Option<String>,
    /// LatestArrivalTime 最晚到店时间 DateTime Y 请传入最早到店时间和最晚到店时间，请注意保证搜索时传入此参数与试单和成单时一致，否则对应担保规则结果会不尽相同，到店时间更改，担保类型为到店时间担保的订单取消规则和担保规则均可能发生变化。要求使用yyyy-MM-dd HH:mm:ss格式，注意最早到店时间需要在hotel.static.info接口的ArrivalTime和LatestArrivalTime之间且小于最晚到店时间
    pub latest_arrival_time: Option<String>,
}

impl BaseRequest for DataBookingRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

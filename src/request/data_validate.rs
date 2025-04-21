use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DataValidateRequest {
    /// ArrivalDate 入住日期 Date N 使用yyyy-MM-dd格式，例如:2022-12-09
    pub arrival_date: String,
    /// DepartureDate 离店日期 Date N 使用yyyy-MM-dd格式，例如:2022-12-09
    pub departure_date: String,
    /// EarliestArrivalTime 最早到店时间 Date Time N 请参考hotel.order.create中的说明
    pub earliest_arrival_time: String,
    /// LatestArrivalTime 最晚到店时间 DateTime N
    pub latest_arrival_time: String,
    /// HourRoomStartTime 钟点房入住开始时间 DateTime Y 当试单的产品为钟点房时，需要入参该字段，格式为yyyy-MM-dd HH:mm:ss，且应符合钟点房的开始结束时间； v1.55添加，可参照：https://open.elong.com/faq/detail?id=318&plt=2传入则进行验证，不传不进行验证
    pub hour_room_start_time: Option<String>,
    /// HourRoomEndTime 钟点房入住结束时间 DateTime Y
    pub hour_room_end_time: Option<String>,
    /// HotelId 酒店编号 String(8) N
    pub hotel_id: String,
    /// RoomId 展示房型编号 String(10) Y 允许为空，当传入时会校验房型编号绑定关系
    pub room_id: Option<String>,
    /// RoomTypeID 销售房型编号 String(10) N
    pub room_type_id: String,
    /// RatePlanId 产品编号 Int N
    pub rate_plan_id: i32,
    /// TotalPrice 总价 Decimal N 货币类型为原币种
    pub total_price: f64,
    /// NumberOfRooms 房间数量 Int N 客人想要预订的房间数量
    pub number_of_rooms: i32,
    /// LittleMajiaId 马甲Id String Y 从hotel.detail接口获取，用于促销，促销产品调用时，需要透传该字段；新接入搜索模式或者国际酒店请求时，必传；建议历史已接入搜索模式分销商也将此字段全部传回；国内酒店马甲Id不再有30分钟限制搜索模式所有产品必传
    pub little_majia_id: Option<String>,
    /// GoodsUniqId 商品唯一标示 String Y 从hotel.detail接口获取，用于促销，促销产品调用时，需要透传该字段；新接入搜索模式或者国际酒店请求时，必传；建议历史已接入搜索模式分销商也将此字段全部传回搜索模式所有产品必传
    pub goods_uniq_id: Option<String>,
    /// ChildAges 儿童年龄 Int[] Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub child_ages: Option<Vec<i32>>,
    /// NumberOfAdults 成人数 Int N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub number_of_adults: Option<i32>,
    /// HotelCode 酒店code String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub hotel_code: Option<String>,
    /// SupplierId 供应商id String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub supplier_id: Option<String>,
    /// SubSupplierId 二级供应商id String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub sub_supplier_id: Option<String>,
    /// ShopperProductId 商品库shopperid String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub shopper_product_id: Option<String>,
    /// CurrencyCode 币种 Enum N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub currency_code: Option<String>,
    /// Nat 国籍 String[] Y 非中国大陆特有字段，如果调用该接口试单时可获取到客人国籍，可传入，我们将验证国籍是否符合当前产品；如果调用该接口时获取不到客人国籍，请按照出参中的Nat字段自行验证当前产品的国籍限制与入住人是否符合
    pub nat: Option<Vec<String>>,
    /// DayPriceList 每日价 DayPrice[] Y 每日价透传：用于每日金额校验，避免出现订单部分退艺龙与合作方退款金额不一致现象发生 。DayPriceList节点里每个DayPrice里的Price之和 * NumberOfRooms = TotalPrice
    pub day_price_list: Option<Vec<DayPrice>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DayPrice {
    /// Price  每日价 BigDecimal N 每日价格
    pub price: f64,
    /// Date 日期 Date N 价格对应的日期
    pub date: String,
    /// MinRate 税后价  BigDecimal Y 国际必传、国内不允许传。对应于NightRate里MinRate，同时Price为NightRate里Rate
    pub min_rate: Option<f64>,
}

impl BaseRequest for DataValidateRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

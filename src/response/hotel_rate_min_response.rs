use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
/// 酒店最低价响应
pub struct HotelRateMinResponse {
    pub hotels: Vec<Hotel>,
}

impl BaseResponse for ElongResponse<HotelRateMinResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<HotelRateMinResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

/// 酒店信息
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Hotel {
    /// 酒店编号
    pub hotel_id: String,
    /// 最小价格信息
    pub min_rates: Option<Vec<MinRate>>,
    /// 房型列表
    pub rooms: Option<Vec<Room>>,
    /// 增值服务
    pub value_adds: Option<Vec<ValueAdd>>,
    /// 预付规则
    pub prepay_rules: Option<Vec<PrepayRule>>,
    /// 担保规则
    pub guarantee_rules: Option<Vec<GuaranteeRule>>,
    /// 预订规则
    pub booking_rules: Option<Vec<BookingRule>>,
    /// 促销规则
    pub drr_rules: Option<Vec<DrrRule>>,
    /// 送礼活动
    pub gifts: Option<Vec<Gift>>,
    /// 礼包套餐
    pub gift_packages: Option<Vec<GiftPackage>>,
}

/// 房型信息
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Room {
    /// 房型编号
    pub room_id: String,
    /// 房型名称
    pub name: String,
    /// 最小价格，包含多个 MinRate 节点，房型的最小价格信息
    pub min_rates: Option<Vec<MinRate>>,
}

/// 最小价格信息
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MinRate {
    /// 最小价类型  
    /// 1. 全日房最小卖价  
    /// 2. 钟点房最小卖价  
    /// 3. 限价的全日房最小卖价（结算价模式时给出）  
    /// 4. 限价的钟点房最小卖价（结算价模式时给出）  
    /// 5. 非限价的全日房最小结算价（结算价模式时给出）  
    /// 6. 非限价的钟点房最小结算价（结算价模式时给出）  
    pub r#type: i32,
    /// 原始价，未经过DRR计算过的原始价格
    pub basis: Option<f64>,
    /// 会员价
    pub member: f64,
    /// 结算价
    pub cost: f64,
    /// 币种，参考 Currency 文档
    pub currency_code: String,
    /// 关联房型编号集合，MinRate节点为酒店下最小价时返回，关联最小价对应房型商品信息，不同房型多个商品可能对应同一个最小价
    pub room_ids: Option<Vec<RoomId>>,
}

/// 房型编号
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct RoomId {
    /// 房型编号
    pub room_id: String,
    /// 房型名称
    pub goods_uniq_ids: Vec<String>,
}

/// 增值服务
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValueAdd {
    /// 增值服务编号
    pub value_add_id: String,
    /// 业务代码
    pub type_code: String,
    /// 描述，附加服务描述，代理不想解析的话，可以直接显示该描述
    pub description: String,
    /// 是否包含在房费中，false-不包含 true-包含
    pub is_include: bool,
    /// 包含的份数
    pub amount: Option<i32>,
    /// 货币代码
    pub currency_code: String,
    /// 单价默认选项（Money-金额，Percent-比例，None-无效）
    pub price_option: Option<String>,
    /// 单价，视 price_option 表示金额或比例
    pub price: Option<rust_decimal::Decimal>,
    /// 是否单加，目前只有早餐服务该字段有意义
    pub is_ext_add: bool,
    /// 单加单价默认选项（Money-金额，Percent-比例）
    pub ext_option: Option<String>,
    /// 单加单价，视 ext_option 不同表示金额或比例值
    pub ext_price: Option<rust_decimal::Decimal>,
    /// 开始日期，特殊早餐有效日期
    pub start_date: Option<String>,
    /// 结束日期，特殊早餐有效日期
    pub end_date: Option<String>,
    /// 周有效设置
    pub week_set: Option<String>,
}

/// 预付规则 (不建议使用)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrepayRule {
    /// 规则编号
    pub prepay_rule_id: i64,
    /// 描述
    pub description: String,
    /// 日期类型（如 CheckInDay）
    pub date_type: String,
    /// 开始日期
    pub start_date: Option<String>,
    /// 结束日期
    pub end_date: Option<String>,
    /// 周有效设置
    pub week_set: Option<String>,
    /// 变更规则（如 PrepayNoChange、PrepayNeedSomeDay、PrepayNeedOneTime）
    pub change_rule: String,
    /// 第一阶段提前的几小时（用于 PrepayNeedSomeDay）
    pub hour: Option<i32>,
    /// 第二阶段提前的几小时（用于 PrepayNeedSomeDay）
    pub hour2: Option<i32>,
    /// 具体取消时间日期部分（用于 PrepayNeedOneTime）
    pub date_num: Option<String>,
    /// 具体取消时间小时部分（用于 PrepayNeedOneTime）
    pub time: Option<String>,
    /// 在变更时间点前是否扣费（1表示扣费，0表示不扣费）
    pub deduct_fees_before: Option<i32>,
    /// 时间点前扣费的金额或比例
    pub deduct_num_before: Option<rust_decimal::Decimal>,
    /// 时间点前扣款类型（如 Money、Percent、FristNight）
    pub cash_scale_first_before: Option<String>,
    /// 在变更时间点后是否扣费（1表示扣费，0表示不扣费）
    pub deduct_fees_after: Option<i32>,
    /// 时间点后扣费的金额或比例
    pub deduct_num_after: Option<rust_decimal::Decimal>,
    /// 时间点后扣款类型（如 Money、Percent、FristNight）
    pub cash_scale_first_after: Option<String>,
}

/// 担保规则 (不建议使用)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GuaranteeRule {
    /// 担保规则编号
    pub gurantee_rule_id: i64,
    /// 描述，如果担保规则存在，将此字段展示给用户
    pub description: String,
    /// 日期类型（如 CheckInDay、StayDay）
    pub date_type: String,
    /// 开始日期
    pub start_date: String,
    /// 结束日期
    pub end_date: String,
    /// 周有效天数
    pub week_set: Option<String>,
    /// 是否到店时间担保，False:为不校验到店时间，True:为需要校验到店时间
    pub is_time_guarantee: bool,
    /// 到店担保开始时间，用于 is_time_guarantee==true 时进行检查
    pub start_time: Option<String>,
    /// 到店担保结束时间
    pub end_time: Option<String>,
    /// 到店担保的结束时间是否为第二天，false为当天，true为次日
    pub is_tomorrow: Option<bool>,
    /// 是否房量担保，False:为不校验房量条件，True:为校验房量条件
    pub is_amount_guarantee: bool,
    /// 担保的房间数，预定几间房及以上要担保
    pub amount: Option<i32>,
    /// 担保类型（如 FirstNightCost、FullNightCost）
    pub guarantee_type: Option<String>,
    /// 变更规则（如 NoChange、NeedSomeDay、NeedCheckin24hour）
    pub change_rule: Option<String>,
    /// 日期参数，change_rule=NeedSomeDay时使用
    pub day: Option<String>,
    /// 时间参数，change_rule=NeedSomeDay时使用
    pub time: Option<String>,
    /// 小时参数，change_rule=NeedCheckinTime/NeedCheckin24hour时使用
    pub hour: Option<i32>,
}

/// 预订规则
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BookingRule {
    /// 规则类型
    pub type_code: String,
    /// 预订规则编号，RatePlan.BookingRuleIds将与此关联
    pub booking_rule_id: i64,
    /// 描述
    pub description: String,
    /// 日期类型
    pub date_type: Option<String>,
    /// 开始日期
    pub start_date: Option<String>,
    /// 结束日期
    pub end_date: Option<String>,
    /// 每天开始时间
    pub start_hour: Option<String>,
    /// 每天结束时间
    pub end_hour: Option<String>,
}

/// 促销规则 （已废弃）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DrrRule {
    /// 促销规则编号
    pub drr_rule_id: i32,
    /// 产品促销规则类型代码
    pub type_code: String,
    /// 描述，该字段可以展示给用户
    pub description: Option<String>,
    /// 日期类型（如 CheckInDay、StayDay、BookDay）
    pub date_type: String,
    /// 促销生效开始日期
    pub start_date: Option<String>,
    /// 促销生效结束日期
    pub end_date: Option<String>,
    /// 提前几天
    pub day_num: Option<i32>,
    /// 连住几天
    pub check_in_num: Option<i32>,
    /// 每连住几晚
    pub every_check_in_num: Option<i32>,
    /// 最后几天
    pub last_day_num: Option<i32>,
    /// 第几晚及以后优惠
    pub which_day_num: Option<i32>,
    /// 按金额或按比例来优惠（Cash-金额、Money-金额、Percent-比例、Scale-比例）
    pub cash_scale: Option<String>,
    /// 按金额或比例优惠的数值
    pub deduct_num: Option<rust_decimal::Decimal>,
    /// 星期有效设置
    pub week_set: Option<String>,
    /// 价格类型（WeekendFee-周末价格、WeekdayFee-平日价格）
    pub fee_type: Option<String>,
}

/// 送礼信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gift {
    /// 送礼编号
    pub gift_id: i64,
    /// 礼包副标题
    pub gift_description: Option<String>,
    /// 描述
    pub description: String,
    /// 礼品有效开始时间
    pub start_date: String,
    /// 礼品有效结束时间
    pub end_date: String,
    /// 日期类型（如 CheckinDate、BookingDate、StayDate）
    pub date_type: String,
    /// 星期设置
    pub week_set: Option<String>,
    /// 活动内容
    pub gift_content: Option<String>,
    /// 送礼类型（已废弃，建议使用 gift_infos 字段）
    pub gift_types: Option<String>,
    /// 新的送礼类型
    pub gift_infos: Option<Vec<GiftInfo>>,
    /// 小时数
    pub hour_number: Option<i32>,
    /// 小时数的类型（如 Hours24、XhourBefore、XHourAfter）
    pub hour_type: Option<String>,
    /// 送礼方式（如 EveryRoom、EveryRoomPerDay、Other）
    pub way_of_giving: Option<String>,
    /// 其他的送礼具体方式
    pub way_of_giving_other: Option<String>,
    /// 礼包价值
    pub gift_value: Option<f64>,
}

/// 礼包一级信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftInfo {
    /// 礼包一级编号
    pub gift_info: i64,
    /// 二级礼包内容
    pub gift_sub_infos: Option<Vec<GiftSubInfo>>,
}

/// 礼包二级信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftSubInfo {
    /// 礼包二级编号
    pub sub_info: i32,
}

/// 礼包套餐信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftPackage {
    /// 礼包套餐ID
    pub pkg_product_id: i64,
    /// 礼包套餐类型（0：礼包，1：套餐）
    pub pkg_type: i32,
    /// 礼包套餐名字
    pub pkg_product_name: Option<String>,
    /// 礼包套餐特别说明
    pub rule_description_additional: Option<String>,
    /// 礼包套餐图片
    pub pictures: Option<Vec<Picture>>,
    /// X产品列表
    pub x_products: Option<Vec<XProduct>>,
}

/// X产品信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XProduct {
    /// X产品ID
    pub x_product_id: i64,
    /// X产品名字
    pub x_product_name: Option<String>,
    /// X产品类型，有"食"，"享" 两种类型
    pub type_name: Option<String>,
    /// X产品数量
    pub quantity: Option<String>,
    /// X产品接待时间
    pub reception_times: Option<String>,
    /// X产品适用人数
    pub capacity: Option<String>,
    /// X产品预订电话
    pub booking_phone: Option<String>,
    /// X产品预订规则
    pub appoint_policy: Option<String>,
    /// X使用地点描述
    pub ticket_use_site_desc: Option<String>,
    /// X特别说明
    pub freestyle_desc: Option<String>,
}

/// 礼包套餐图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Picture {
    /// 礼包套餐图片顺序
    pub img_index: Option<i32>,
    /// 礼包套餐图片链接
    pub img_url: Option<String>,
}

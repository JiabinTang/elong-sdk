use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
/// 酒店详情响应
pub struct HotelDetailResponse {
    /// 查询到的酒店总数
    pub count: i32,
    /// 酒店结果集
    pub hotels: Option<Vec<Hotel>>,
    /// 汇率信息
    pub exchange_rate_list: Option<Vec<ExchangeRate>>,
}

impl BaseResponse for ElongResponse<HotelDetailResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<HotelDetailResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

/// 酒店信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Hotel {
    /// 酒店编号
    pub hotel_id: String,
    /// 最低价格，返回的是该酒店中所有符合查询条件的产品的最低价
    pub low_rate: rust_decimal::Decimal,
    /// 最低价格的货币，参考 Currency 枚举
    pub currency_code: Option<String>,
    /// 预订规则列表
    pub booking_rules: Option<Vec<BookingRule>>,
    /// 担保规则列表（不建议使用）
    pub guarantee_rules: Option<Vec<GuaranteeRule>>,
    /// 预付规则列表（不建议使用）
    pub prepay_rules: Option<Vec<PrepayRule>>,
    /// 新担保规则列表（不建议使用）
    pub guarantee_rule_extends: Option<Vec<GuaranteeRuleExtend>>,
    /// 新预付规则列表（不建议使用）
    pub prepay_rule_extends: Option<Vec<PrepayRuleExtend>>,
    /// 增值服务列表
    pub value_adds: Option<Vec<ValueAdd>>,
    /// 促销规则列表（已废弃）
    pub drr_rules: Option<Vec<DrrRule>>,
    /// 酒店设施信息
    pub facilities: Option<String>,
    /// 房型列表
    pub rooms: Option<Vec<Room>>,
    /// 酒店详细信息
    pub detail: Option<Detail>,
    /// 酒店图片列表（已废弃）
    pub images: Option<Vec<Image>>,
    /// 送礼活动列表
    pub gifts: Option<Vec<Gift>>,
    /// 礼包套餐列表
    pub gift_packages: Option<Vec<GiftPackage>>,
    /// 酒店特殊信息提示列表
    pub h_avail_policys: Option<Vec<HAvailPolicy>>,
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

/// 新担保规则 (不建议使用)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GuaranteeRuleExtend {
    /// 规则ID
    pub guarantee_rule_extend_id: i32,
    /// 开始时间
    pub start_date: String,
    /// 结束时间
    pub end_date: String,
    /// 周有效设置
    pub week_set: String,
    /// 担保类型（0:需担保 1:无需担保 2:超时担保）
    pub guarantee_type: i32,
    /// 取消费用类型（0:跟随取消费用 1:订单全额）
    pub noshow_penalty: Option<i32>,
    /// 超时担保时间，单位分钟，相对入住日24点的小时偏移量, 范围[0,840]
    pub grt_latest_check_time: Option<i32>,
    /// 取消规则列表
    pub penalty_rule_list: Option<Vec<PenaltyWindowType>>,
}

/// 新预付规则 (不建议使用)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrepayRuleExtend {
    /// 规则ID
    pub prepay_rule_extend_id: i32,
    /// 开始时间
    pub start_date: String,
    /// 结束时间
    pub end_date: String,
    /// 周有效设置
    pub week_set: String,
    /// 取消费用类型（0:跟随取消费用 1:订单全额，目前只有0）
    pub noshow_penalty: Option<i32>,
    /// 取消规则列表
    pub penalty_rule_list: Option<Vec<PenaltyWindowType>>,
}

/// 取消规则窗口类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PenaltyWindowType {
    /// 扣款类型（0:百分比 1:晚数 2:首晚百分比）
    pub penalty_type: i32,
    /// 罚金
    pub penalty_value: f64,
    /// 规则时间分割起始点，单位分钟
    pub deadline: String,
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

/// 房型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Room {
    /// 展示房型编号
    pub room_id: String,
    /// 房间英文名
    pub name_en: Option<String>,
    /// 房型名称
    pub name: String,
    /// 产品信息
    pub rate_plans: Option<Vec<RatePlan>>,
    /// 图片地址（已废弃）
    pub image_url: Option<String>,
    /// 楼层
    pub floor: Option<String>,
    /// 上网情况
    pub broadnet: Option<String>,
    /// 床型
    pub bed_type: Option<String>,
    /// 床型描述
    pub bed_desc: Option<String>,
    /// 房间描述
    pub description: Option<String>,
    /// 房间备注
    pub comments: Option<String>,
    /// 面积
    pub area: Option<String>,
    /// 可容纳人数
    pub capcity: Option<String>,
    /// 窗户类型ID
    pub window_type_id: Option<i32>,
    /// 窗户类型描述
    pub windos_type: Option<String>,
}

/// 产品信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RatePlan {
    /// 产品编号
    pub rate_plan_id: i64,
    /// 销售房型编号
    pub room_type_id: String,
    /// 产品名称
    pub rate_plan_name: String,
    /// 供应商酒店编码
    pub hotel_code: Option<String>,
    /// 商品唯一标识
    pub goods_uniq_id: Option<String>,
    /// 产品唯一标识（API成单使用）
    pub littlemajiaid: Option<String>,
    /// 产品唯一标识（H5或小程序跳转模式）
    pub majia_id: Option<String>,
    /// 销售状态，true可销售，false不可销售
    pub status: bool,
    /// 供应商房型附加名称
    pub suffix_name: Option<String>,
    /// 是否支持专票
    pub support_special_invoice: Option<bool>,
    /// 客人类型（已废弃）
    pub customer_type: Option<String>,
    /// 适用人群
    pub guest_type: Option<String>,
    /// 适用人群中其他内容（已废弃）
    pub guest_type_extend_ch: Option<String>,
    /// 房量限额
    pub current_alloment: Option<i32>,
    /// 预计确认时长（单位秒）
    pub confirm_duration_time: Option<i32>,
    /// 是否支持即时确认
    pub instant_confirmation: Option<bool>,
    /// 付款类型（SelfPay-前台现付、Prepay-预付）
    pub payment_type: Option<String>,
    /// 对应的预订规则编号，英文逗号分隔
    pub booking_rule_ids: Option<String>,
    /// 是否为限价产品
    #[serde(rename = "isPriceLimittedProduct")]
    pub is_price_limitted_product: bool,
    /// 限价类型
    pub price_limited_type: Option<i32>,
    /// 对应的担保规则编号，英文逗号分隔
    pub guarantee_rule_ids: Option<String>,
    /// 对应的预付规则编号，英文逗号分隔
    pub prepay_rule_ids: Option<String>,
    /// 对应的新担保规则ID，英文逗号分隔
    pub guarantee_rule_extend_ids: Option<String>,
    /// 对应的新预付规则ID，英文逗号分隔
    pub prepay_rule_extend_ids: Option<String>,
    /// 对应的促销规则编号，英文逗号分隔
    pub drr_rule_ids: Option<String>,
    /// 对应的增值服务编号，英文逗号分隔
    pub value_add_ids: Option<String>,
    /// 礼品ID，英文逗号分隔
    pub gift_ids: Option<String>,
    /// 礼包套餐ID，英文逗号分隔
    pub pkg_product_ids: Option<String>,
    /// 新餐食节点
    pub meals: Option<Meals>,
    /// 酒店特殊信息提示的编号
    pub h_avail_policy_ids: Option<String>,
    /// 产品特性类型，英文逗号分隔
    pub product_types: Option<String>,
    /// 分销渠道，英文逗号分隔
    pub sell_channels: Option<String>,
    /// 是否今日特价（即尾房）
    pub is_last_minute_sale: Option<bool>,
    /// 是否需要提供身份证号（已废弃）
    pub need_id_no: Option<bool>,
    /// 身份信息验证类型
    pub identification: Option<i32>,
    /// 尾房开始时间
    pub start_time: Option<String>,
    /// 尾房结束时间
    pub end_time: Option<String>,
    /// 预定最少数量
    pub min_amount: Option<i32>,
    /// 最少入住天数
    pub min_days: Option<i32>,
    /// 最多入住天数
    pub max_days: Option<i32>,
    /// 最多预订间数
    pub max_checkin_rooms: Option<i32>,
    /// 最少提前预订小时数
    pub min_adv_hours: Option<i32>,
    /// 最多提前预订小时数
    pub max_adv_hours: Option<i32>,
    /// 总价
    pub total_rate: Option<rust_decimal::Decimal>,
    /// 日均价
    pub average_rate: Option<rust_decimal::Decimal>,
    /// 促销前的日均价（废弃）
    pub average_base_rate: Option<rust_decimal::Decimal>,
    /// 货币
    pub currency_code: Option<String>,
    /// 同程促销金额
    pub coupon: Option<rust_decimal::Decimal>,
    /// 每天价格数组
    pub nightly_rates: Option<Vec<NightlyRate>>,
    /// 预付产品发票模式
    pub invoice_mode: Option<String>,
    /// 酒店签约类型
    pub cooperation_type: Option<i32>,
    /// 产品可以展示销售的渠道（已废弃）
    pub booking_channels: Option<String>,
    /// 可住开始时间（钟点房）
    pub earliest_tolive_time: Option<String>,
    /// 可住结束时间（钟点房）
    pub latest_tolive_time: Option<String>,
    /// 可住时长（钟点房）
    pub stay_time: Option<String>,
    /// 可入住人数（已废弃）
    pub x_stay_people_num: Option<String>,
    /// 可入住性别（已废弃）
    pub x_stay_sex: Option<String>,
    /// 床型（已废弃）
    pub x_bed_type: Option<String>,
    /// 楼层（已废弃）
    pub x_floor: Option<String>,
    /// 朝向（已废弃）
    pub x_orientation: Option<String>,
    /// 自定义说明（已废弃）
    pub x_user_defined: Option<String>,
    /// 酒店等级
    pub hotel_level: Option<String>,
    /// 国籍限制
    pub nat: Option<Nat>,
    /// 每日促销明细集合
    pub day_promotions: Option<Vec<DayPromotion>>,
    /// 促销活动集合
    pub promotion_flags: Option<Vec<PromotionFlag>>,
    /// 是否促销
    pub is_promotion: Option<bool>,
    /// 是否需要回传马甲
    pub need_majia_id: Option<bool>,
    /// 使用的同程促销类型
    pub used_promotion_values: Option<Vec<UsedPromotionValue>>,
    /// 网络标准化
    pub internet_filter: Option<i32>,
    /// 早餐标准化
    pub board_filter: Option<i32>,
    /// 预付结果节点
    pub prepay_result: Option<PrepayResult>,
    /// 现付结果节点
    pub guarantee_result: Option<GuaranteeResult>,
    /// 房间面积
    pub room_square_metres: Option<String>,
    /// 税和服务费原币种（废弃）
    pub tax_and_service_fee: Option<rust_decimal::Decimal>,
    /// 税和服务费人民币币种（废弃）
    pub tax_and_service_fee_rmb: Option<rust_decimal::Decimal>,
    /// 床型描述
    pub bed_description: Option<String>,
    /// 床型标准化描述
    pub bed_type_associational_filter: Option<String>,
    /// 是否有窗
    pub has_window: Option<bool>,
    /// 吸烟偏好描述
    pub smoking_desc: Option<String>,
    /// 膳食
    pub board: Option<Board>,
    /// 房间可住儿童年龄
    pub room_child_age: Option<i32>,
    /// 房间最大入住人数
    pub room_max_pax: Option<i32>,
    /// 房间最大可住成人数
    pub adult_occupancy_per_room: Option<i32>,
    /// 房间最大可住儿童数
    pub children_occupancy_per_room: Option<i32>,
    /// 入住需知
    pub check_in_instructions: Option<String>,
    /// 额外人员费用(附加费)
    pub extra_person_fee: Option<rust_decimal::Decimal>,
    /// 额外人员费用(附加费人民币)
    pub extra_person_fee_rmb: Option<rust_decimal::Decimal>,
    /// 床型信息
    pub bed_groups: Option<Vec<InterBedGroup>>,
    /// 网络描述
    pub internet_desc: Option<String>,
    /// 供应商id
    pub supplier_id: Option<String>,
    /// 二级供应商id
    pub sub_supplier_id: Option<String>,
    /// 商品库shopperid
    pub shopper_product_id: Option<String>,
    /// 另付税和服务费
    pub additional_tax: Option<AdditionalTax>,
}

/// 每日价格信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NightlyRate {
    /// 当天日期
    pub date: String,
    /// 会员价，已经通过DRR的计算可以直接显示给客人
    pub member: rust_decimal::Decimal,
    /// 结算价，仅用于结算价模式下的预付产品
    pub cost: rust_decimal::Decimal,
    /// 库存状态，表示当天库存是否可用
    pub status: bool,
    /// 加床价，-1表示不能加床
    pub add_bed: Option<rust_decimal::Decimal>,
    /// 原始价格，未经过DRR计算过的原始价格
    pub basis: Option<rust_decimal::Decimal>,
    /// 早餐数量
    pub breakfast_count: Option<i32>,
    /// 同程促销金额
    pub coupon: Option<rust_decimal::Decimal>,
    /// 每晚每间房价（含税费，仅用于国际及港澳台酒店）
    pub rate: Option<rust_decimal::Decimal>,
    /// 最小价（不含税费，仅用于国际及港澳台酒店）
    pub min_rate: Option<rust_decimal::Decimal>,
}

/// 使用的同程促销类型及金额
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UsedPromotionValue {
    /// 促销类型id（1:返现; 9:立减; 10:普通红包返现; 11:普通红包立减; 62:满返红包; 63:满减红包）
    pub promotion_type_id: i32,
    /// 优惠金额
    pub promotion_value: rust_decimal::Decimal,
}

/// 国籍限制信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Nat {
    /// 国籍类型（1:不适用; 2:适用）
    pub r#type: i32,
    /// 国籍Code列表，例如 ["CN","GB"]
    pub list: Option<Vec<String>>,
}

/// 酒店详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Detail {
    /// 酒店名称
    pub hotel_name: String,
    /// 挂牌星级
    pub star_rate: i32,
    /// 艺龙推荐级别
    pub category: i32,
    /// 纬度
    pub latitude: Option<String>,
    /// 经度
    pub longitude: Option<String>,
    /// 地址
    pub address: Option<String>,
    /// 前台电话
    pub phone: Option<String>,
    /// 封面图片
    pub thumb_nail_url: Option<String>,
    /// 城市ID
    pub city: Option<String>,
    /// 城市名称
    pub city_name: Option<String>,
    /// 行政区ID
    pub district: Option<String>,
    /// 行政区名称
    pub district_name: Option<String>,
    /// 商业区ID
    pub business_zone: Option<String>,
    /// 商业区名称
    pub business_zone_name: Option<String>,
    /// 评价
    pub review: Option<Review>,
    /// 特色介绍
    pub features: Option<String>,
    /// 设施服务
    pub general_amenities: Option<String>,
    /// 交通状况
    pub traffic: Option<String>,
    /// 酒店描述
    pub description: Option<String>,
    /// 酒店英文名
    pub hotel_name_en: Option<String>,
    /// 英文地址
    pub address_en: Option<String>,
    /// 到店时间
    pub check_in_time: Option<String>,
    /// 离店时间
    pub check_out_time: Option<String>,
    /// 国家名称
    pub hotel_country_name: Option<String>,
    /// 国家编码
    pub hotel_country_code: Option<String>,
    /// 国家id
    pub hotel_country_id: Option<i32>,
    /// 酒店政策
    pub hotel_policy: Option<String>,
}

/// 酒店评论信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Review {
    /// 好评数
    pub good: String,
    /// 差评数
    pub poor: String,
    /// 评论总数
    pub count: String,
    /// 好评率，可能带百分号
    pub score: Option<String>,
}

/// 图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    /// 图片类型
    pub r#type: i32,
    /// 是否是酒店封面，"true" 表示封面图片
    pub is_cover_image: Option<String>,
    /// 是否为房型封面，"true" 表示房型封面图片
    pub is_room_cover_image: Option<String>,
    /// 房型编号
    pub room_id: Option<String>,
    /// 图片地址列表
    pub locations: Option<Vec<Location>>,
}

/// 图片地址信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    /// 图片规格
    pub size_type: i32,
    /// 是否有水印
    pub water_mark: String,
    /// 图片地址
    pub url: String,
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

/// 酒店特殊信息提示
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HAvailPolicy {
    /// 提示编号，RatePlan.HAvailPolicyIds 与此关联
    pub id: String,
    /// 提示内容
    pub avail_policy_text: String,
    /// 有效开始时间
    pub avail_policy_start: String,
    /// 有效结束时间
    pub avail_policy_end: String,
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

/// 汇率信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExchangeRate {
    /// 货币编码
    pub currency_code: String,
    /// 汇率值，对应货币转换成人民币的汇率
    pub rate: rust_decimal::Decimal,
}

/// 每日促销明细
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DayPromotion {
    /// 日期
    pub date: Option<String>,
    /// 每日促销明细，可以包含多个促销
    pub promotions: Option<Vec<Promotion>>,
}

/// 促销信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Promotion {
    /// 促销类型（0:未定义、1:天天特价、2:门店新客、3:优享会、4:其他促销、5:权益云）
    pub promotion_type: i32,
    /// 卖价优惠金额
    pub price_discount_value: rust_decimal::Decimal,
    /// 促销活动Id
    pub promotion_id: i64,
}

/// 促销活动标签
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionFlag {
    /// 促销活动Id
    pub promotion_id: i64,
    /// 促销活动标签
    pub promotion_tag: Option<String>,
}

/// 膳食信息（仅用于国际及港澳台酒店）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Board {
    /// 是否含早
    pub breakfast_included: Option<bool>,
    /// 是否半膳
    pub halfboard_included: Option<bool>,
    /// 是否全膳
    pub fullboard_included: Option<bool>,
    /// 膳食明细
    pub boards: Option<BoardDetail>,
    /// 膳食描述
    pub board_desc: Option<String>,
}

/// 膳食明细（仅用于国际及港澳台酒店）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoardDetail {
    /// 描述
    pub description: Option<String>,
    /// 膳食数量
    pub count: Option<i32>,
    /// 膳食类型（已废弃）
    pub type_: Option<i32>,
}

/// 床型信息（仅用于国际及港澳台酒店）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InterBedGroup {
    /// 床型信息id
    pub bed_group_id: Option<String>,
    /// 床型信息描述
    pub bed_group_desc: Option<String>,
    /// 床类型列表
    pub bed_types: Option<Vec<BedType>>,
}

/// 床类型信息（仅用于国际及港澳台酒店）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BedType {
    /// 床类型id
    pub bed_type_id: Option<String>,
    /// 床类型名称
    pub bed_type_name: Option<String>,
    /// 床类型
    pub bed_type: Option<String>,
    /// 床数
    pub count: Option<i32>,
    /// 床大小
    pub size: Option<String>,
}

/// 预付结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrepayResult {
    /// 取消规则
    pub cancel_description: String,
    /// 取消类型（1：免费取消；2：收费取消，3：限时取消，4：不可取消）
    pub cancel_type: i32,
    /// 是否使用阶梯担保规则（废弃）
    pub ladder_vouch: Option<bool>,
    /// 取消规则明细
    pub ladder_parse_list: Option<Vec<LadderParse>>,
    /// 取消规则标签
    pub cancel_tag: Option<String>,
}

/// 阶梯取消规则明细
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LadderParse {
    /// 开始时间，北京时间时间戳，秒
    pub begin_time: i64,
    /// 结束时间，北京时间时间戳，秒
    pub end_time: i64,
    /// 扣费类型（0:不扣费；1:金额；2:比例；3:首晚房费；4:全额房费）
    pub cut_type: i32,
    /// 扣费值，原始币种
    pub cut_value: rust_decimal::Decimal,
    /// 扣费值，国际现付的是原币，预付对客的是人民币，预付对酒店的是原币
    pub amount: rust_decimal::Decimal,
    /// 短文案
    pub short_desc: Option<String>,
    /// 扣费值，人民币
    pub amount_rmb: rust_decimal::Decimal,
    /// 汇率
    pub exchange_rate: Option<rust_decimal::Decimal>,
}

/// 现付结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GuaranteeResult {
    /// 到店时间触发时的担保金额（废弃，货币类型为原币种）
    pub money_arrival_time: f64,
    /// 需要担保的到店时间(格式：hh:mm)
    pub arrival_time: String,
    /// 房量担保分割点
    pub room_count: i32,
    /// 担保类型
    pub guarantee_type: i32,
    /// 当前条件下需要担保的金额
    pub guarantee_money: f64,
    /// 当前条件下是否需要担保
    pub need_guarantee: bool,
    /// 可以取消的时间点，单位秒（废弃）
    pub cancel_time: i64,
    /// 取消规则详细描述
    pub cancel_description: Option<String>,
    /// 取消规则标签
    pub cancel_tag: Option<String>,
    /// 取消类型（1. 免费取消、2.付费取消、3.可取消、4.不可取消）
    pub cancel_type: i32,
    /// 取消规则明细
    pub ladder_parse_list: Option<Vec<LadderParse>>,
}

/// 餐食信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meals {
    /// 是否存在餐食表格，为 true 代表取 day_meal_table 字段
    pub has_meal_table: bool,
    /// 餐食文案描述，总餐食描述
    pub meal_copy_writing: String,
    /// 每日餐食表格，包含多个 DayMeal
    pub day_meal_table: Vec<DayMeal>,
}

/// 每日餐食信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DayMeal {
    /// 餐食的日期，yyyy-MM-dd格式
    pub date: String,
    /// 是否使用动态餐食，为 true 取 dynamic_meal_desc
    pub use_dynamic_meal: bool,
    /// 动态餐食描述
    pub dynamic_meal_desc: Option<String>,
    /// 早餐数量
    pub breakfast_share: Option<i32>,
    /// 早餐描述
    pub breakfast_desc: Option<String>,
    /// 午餐数量
    pub lunch_share: Option<i32>,
    /// 午餐描述
    pub lunch_desc: Option<String>,
    /// 晚餐数量
    pub dinner_share: Option<i32>,
    /// 晚餐描述
    pub dinner_desc: Option<String>,
    /// 到天餐食描述
    pub day_meal_copy_writing: Option<String>,
}

/// 另付税和服务费信息
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalTax {
    /// 另付税和服务费总额(人民币)
    pub total_amount_rmb: Option<rust_decimal::Decimal>,
    /// 另付税和服务费总额(支付币)
    pub total_amount_paid: Option<rust_decimal::Decimal>,
    /// 另付税和服务费明细
    pub additional_tax_items: Option<Vec<AdditionalTaxItem>>,
}

/// 另付税和服务费明细
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalTaxItem {
    /// 另付税和服务费明细描述
    pub description: Option<String>,
    /// 另付税和服务费明细金额
    pub amount: Option<rust_decimal::Decimal>,
}

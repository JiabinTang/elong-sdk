use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRpResponse {
    /// 酒店列表，包含多个Hotel节点
    pub hotels: Option<Vec<Hotel>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Hotel {
    /// 酒店编号   HotelID 酒店编号 String(8) N
    #[serde(rename = "HotelID")]
    pub hotel_id: String,
    /// 供应商列表，包含多个Supplier节点 Suppliers 供应商列表 Supplier[] Y 包含多个Supplier节点，一个Supplier表示一个供应商，一家酒店可能代理给多个供应商，艺龙也会从其他供应商分销部分产品，所以一个HotelId会对应多个HotelCode来表示不同的供应商酒店
    pub suppliers: Option<Vec<Supplier>>,
    /// 产品列表，包含多个RatePlan节点 RatePlans 产品列表 RatePlan[] Y 包含多个 RatePlan节点
    pub rate_plans: Option<Vec<RatePlan>>,
    /// 礼包信息，包含多个Gift节点 Gifts 礼包 Gift[] Y酒店送礼信息, 礼品信息和预订或入住日期相关。包含多个Gift节点（建议使用GiftPackages节点）GiftPackages礼包套餐GiftPackage[]Y参考多个GiftPackage节点，参考下方文末礼包套餐（GiftPackage）使用说明
    pub gifts: Option<Vec<Gift>>,
    /// GiftPackages 礼包套餐 GiftPackage[] Y 参考多个GiftPackage节点，参考下方文末礼包套餐（GiftPackage）使用说明
    pub gift_packages: Option<Vec<GiftPackage>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Supplier {
    /// HotelCode 酒店编码 String(8) N
    pub hotel_code: String,
    /// WeekendStart 周末开始 Int N 为0表示周末设置从周一开始
    pub weekend_start: i32,
    /// WeekendEnd 周末结束 Int N 为0表示到周日结束，但是两个都为0表示无周末设置；
    /// 如果开始为3，结束为1，表示从周三到下周1都是周末设置 1代表周一，7代表周日
    pub weekend_end: i32,
    /// BookingRules 预订规则 BookingRule[] Y 参考BookingRule节点
    pub booking_rules: Option<Vec<BookingRule>>,
    /// Rooms 房间对应关系 Room[] Y 用于产品获取对应房型静态信息包含多个Room节点
    pub rooms: Option<Vec<Room>>,
    /// InvoiceMode 预付产品发票模式 Enum N 建议使用RatePlan节点上的InvoiceMode，
    /// 该字段之后将不再使用。仅用于预付产品的发票开具模式。
    /// Elong-艺龙开发票、Hotel-酒店开发票前台自付产品都是酒店开发票，
    /// 这里的过滤是针对预付产品。需要注意Elong艺龙开发票其实是艺龙可以提供代开发票服务，
    /// 如果需要开通，请联系商务，否则可能需要自行开具发票
    pub invoice_mode: Option<String>,
    /// hotellevel 酒店等级 String Y V1.34新增 1:特牌 2:金牌 3:银牌 4:蓝牌 0:非挂牌
    pub hotel_level: Option<String>,
    /// DCSupplierId 直连供应商ID String Y 仅表示直连供应商Id
    pub dc_supplier_id: Option<String>,
    /// SupplierType 供应商类型 String Y 供应商类型
    pub supplier_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BookingRule {
    /// TypeCode 规则类型 Enum N
    /// NeedNationality：务必提供客人国籍
    /// PerRoomPerName：您预订了N间房，请您提供不少于N的入住客人姓名
    /// ForeignerNeedEnName：此酒店要求外宾务必留英文拼写 ，港澳台酒店出现这个字段的时候，所有人都需要填写英文名或姓名拼音
    /// RejectCheckinTime：几点到几点酒店不接受预订 , 此处校验的是下单时的当前时间
    /// NeedPhoneNo：务必提供联系人手机号(请加在联系人结点Contact上)
    pub type_code: String,
    /// RoomTypeIds 关联的销售房型Id string(500) Y all 表示所有房型
    pub room_type_ids: Option<String>,
    /// Description 描述 String(255) N
    pub description: String,
    /// DateType 日期类型 Enum Y BookDay –预订日期（订单的创建日期）
    pub date_type: Option<String>,
    /// StartDate 开始日期 Date Y
    pub start_date: Option<String>,
    /// EndDate 结束日期 Date Y
    pub end_date: Option<String>,
    /// StartHour 每天开始时间 Time Y
    /// 针对日期段内每天生效, 当TypeCode 为RejectCheckinTime时表示当前预订时间在StartHour到EndHour区间内酒店不接受预订。
    /// 当EndHour大于24点的时候是表示第二天的几点加上了24小时，如26:00表示第二天的2点。
    pub start_hour: Option<String>,
    /// EndHour 每天结束时间 Time Y
    pub end_hour: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Room {
    /// RoomTypeId 销售房型编号 String(8) N 关联RatePlan.RoomTypeIds
    pub room_type_id: String,
    /// RoomId 展示房型编号 String(8) N 参考静态xml中的Room@Id
    pub room_id: String,
    /// Status 销售房型可用状态 Boolean Y 版本V1.11增加；true---该销售房型可销售，false-该销售房型不能销售
    pub status: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RatePlan {
    /// RatePlanId 产品编号 Int N
    pub rate_plan_id: i32,
    /// RatePlanName 产品名称 String(50) N
    pub rate_plan_name: String,
    /// guestType 适用人群 String Y
    /// 宾客类型的适用人群：
    /// 0:不限 1:持中国身份证的居民 2:持回乡证的港澳人士
    /// 3:持台胞证的台湾人士 4:持中国护照的侨胞
    /// 5:持大陆工作证/居留许可的外籍人士 6:持非中国护照的外籍人士 7:其他
    pub guest_type: Option<String>,
    /// guestTypeExtendCh 其他 String Y
    /// 适用人群中其他的内容
    pub guest_type_extend_ch: Option<String>,
    /// HotelCode 对应的酒店编码 String(8) N
    pub hotel_code: String,
    /// PaymentType 付款类型 Enum N
    /// SelfPay-前台现付、Prepay-预付，具体信息参看使用说明
    pub payment_type: String,
    /// RoomTypeIds 关联的房型编码 String(255) N
    /// 多个房型编码时以英文逗号分隔
    pub room_type_ids: String,
    /// ProductTypes 产品特性类型 String(20) Y
    /// 可逗号分隔，目前取值：
    /// 3-限时抢购 4-钟点房 5-手机专享 6-铂涛产品
    pub product_types: Option<String>,
    /// NeedIdNo 是否需要提供身份证号 Boolean Y
    /// 如果是铂涛产品，不能通过此字段判断，不管true还是false，都要传入身份证。
    /// 如果不是铂涛产品，指该RatePlan在下单的时候，是否需要传入入住人的身份证号信息。
    pub need_id_no: Option<bool>,
    /// Identification 身份信息验证类型 Int N
    /// 0-无特殊验证要求（默认值）
    /// 1-整个订单至少传一个身份证
    /// 2-订单中每个房间至少传一个证件
    /// 3-订单中每个房间至少传一个身份证
    /// 4-每个客人传一个身份证
    /// 5-整个订单至少传一个身份证且需预订本人入住
    pub identification: Option<i32>,
    /// IsLimitTimeSale 是否是今日特价(尾房) Boolean Y
    /// 如果为true，则要校验当前时间是否在StartTime和EndTime的范围内，从而决定这个RP是否显示在可销售产品中。
    pub is_limit_time_sale: Option<bool>,
    /// StartTime 尾房每天预订开始时间 Time Y
    /// 默认值：00:00。仅在尾房情况下有效。
    pub start_time: Option<String>,
    /// EndTime 尾房每天预订结束时间 Time Y
    /// 默认值:23:59。仅在尾房情况下有效。如果结束时间是00:00至6:00，则表示是次日。
    pub end_time: Option<String>,
    /// MinAmount 预订最少数量 Int Y
    /// 默认值：1
    pub min_amount: Option<i32>,
    /// MinDays 最少入住天数 Int Y
    /// 默认值：1
    pub min_days: Option<i32>,
    /// MaxDays 最多入住天数 Int Y
    /// 默认值：365
    pub max_days: Option<i32>,
    /// MaxCheckinRooms 最多预订间数 Int Y
    /// 版本v1.35新增，规定了每个订单最多预订多少间。
    /// 为0时默认按照10间处理，默认值：9999
    pub max_checkin_rooms: Option<i32>,
    /// MinAdvHours 最少提前预订小时数 Int N
    /// 按checkInDate的23:59:59(一般认为24点)来计算
    pub min_adv_hours: Option<i32>,
    /// MaxAdvHours 最多提前预订小时数 Int N
    pub max_adv_hours: Option<i32>,
    /// GuaranteeRules 担保规则 GuaranteeRule[] Y
    /// 参考 GuaranteeRule节点
    pub guarantee_rules: Option<Vec<GuaranteeRule>>,
    /// PrepayRules 预付规则 PrepayRule[] Y
    /// 参考PrepayRule节点
    pub prepay_rules: Option<Vec<PrepayRule>>,
    /// ValueAdds 增值服务 ValueAdd[] Y
    /// 参考ValueAdd节点
    pub value_adds: Option<Vec<ValueAdd>>,
    /// Meals 新餐食节点 Meal[] Y
    /// 参考Meals节点，相较增值服务中的早餐，拓展了午餐、晚餐和动态餐食。
    pub meals: Option<Vec<Meal>>,
    /// BookingChannels 产品可以展示销售的渠道 String Y
    /// 逗号分隔的数字列表：1---线上(普通的PC访问的Web) 2---线下(呼叫中心、门店) 3---手机(Mobile App、H5)
    pub booking_channels: Option<String>,
    /// IsPriceLimitProduct 是否为限价产品 Boolean N
    /// 表示本RatePlan是否为限价产品，限价产品必须按照艺龙给出的售价进行售卖。
    pub is_price_limit_product: Option<bool>,
    /// PriceLimitedType 限价类型 Int N
    /// 二进制bit位分别表示各个限价条件，0为非限价。
    pub price_limited_type: Option<i32>,
    /// CustomerLevel 可售会员等级 Int[] Y
    /// 选值：0-非会员 1-普通会员 2-贵宾会员 3-龙萃会员 4-钻石龙萃
    pub customer_level: Option<Vec<i32>>,
    /// InvoiceMode 预付产品发票模式 String N
    /// Elong-艺龙开发票、Hotel-酒店开发票。
    pub invoice_mode: Option<String>,
    /// CooperationType 酒店签约类型 Int Y
    /// 1为直签，2为非直签，0为未知
    pub cooperation_type: Option<i32>,
    /// earliestToliveTime 可住开始时间 String(6) Y
    /// 钟点房产品特有字段。
    pub earliest_tolive_time: Option<String>,
    /// latestToliveTime 可住结束时间 String(6) Y
    pub latest_tolive_time: Option<String>,
    /// stayTime 可住时长 String(2) Y
    pub stay_time: Option<String>,
    /// SupportSpecialInvoice 是否支持专票 Boolean Y
    /// true 支持专票，false 不支持
    pub support_special_invoice: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GuaranteeRule {
    /// Description 描述 String(255) N
    pub description: String,
    /// DateType 日期类型 Enum N
    /// CheckInDay-入住日期 StayDay-在店日期
    pub date_type: String,
    /// StartDate 开始日期 date N
    /// 举例：DateType为CheckInDay：表示当前订单的入住日期落在StartDate和EndDate之间，
    /// 并且入住日期符合周设置时才需要判断其它条件是否担保，否则不需要担保
    pub start_date: String,
    /// EndDate 结束日期 date N
    pub end_date: String,
    /// WeekSet 周有效天数 String(20) Y
    /// 一般为周一到周日都有效，判断日期符合日期段同时也要满足周设置的有效。
    /// 周一对应为1，周二对应为2，依次类推；逗号分隔，为空时表示无周末设置。
    /// DateType为StayDay：表示当前订单的客人只要有住在店里面的日期（ArrivalDate, DepartureDate）
    /// 落在StartDate和EndDate之间，并且入住日期符合周设置时才需要判断其它条件是否担保，否则不需要担保。
    pub week_set: Option<String>,
    /// IsTimeGuarantee 是否到店时间担保 Boolean N
    /// False:为不校验到店时间 True:为需要校验到店时间。
    /// 此字段与之后的IsAmountGuarantee字段用法比较特殊，请仔细阅读注意事项中关于这两个字段的说明。
    pub is_time_guarantee: bool,
    /// StartTime 到店担保开始时间 Time Y
    /// 用于IsTimeGuarantee==true进行检查。
    pub start_time: Option<String>,
    /// EndTime 到店担保结束时间 Time Y
    /// 当EndTime小于StartTime的时候，默认从StartTime到次日6点都需要担保。
    pub end_time: Option<String>,
    /// IsTomorrow 到店担保的结束时间是否为第二天 Boolean Y
    /// 0为当天，1为次日
    pub is_tomorrow: Option<bool>,
    /// IsAmountGuarantee 是否房量担保 Boolean N
    /// False:为不校验房量条件 True:为校验房量条件
    pub is_amount_guarantee: bool,
    /// Amount 担保的房间数 Int Y
    /// 预定几间房以上要担保，用于IsAmountGuarantee==true进行检查。
    pub amount: Option<i32>,
    /// GuaranteeType 担保类型 String Y
    /// FirstNightCost为首晚房费担保 FullNightCost为全额房费担保
    pub guarantee_type: Option<String>,
    /// ChangeRule 变更规则 Enum Y
    /// 担保规则取消变更规则：
    /// NoChange、不允许变更取消
    /// NeedSomeDay、允许变更/取消,需在XX日YY时之前通知
    /// NeedCheckinTime、允许变更/取消,需在最早到店时间之前几小时通知
    /// NeedCheckin24hour、允许变更/取消,需在到店日期的24点之前几小时通知
    pub change_rule: Option<String>,
    /// Day 日期参数 Date Y
    /// ChangeRule=NeedSomeDay时，对应规则2描述中
    /// “允许变更/取消,需在XX日YY时之前通知”中的XX日
    pub day: Option<String>,
    /// Time 时间参数 Time Y
    pub time: Option<String>,
    /// Hour 小时参数 Int Y
    /// ChangeRule=NeedCheckinTime时，对应规则3描述中
    /// “允许变更/取消,需在最早到店时间之前几小时通知”中的几小时。
    /// ChangeRule=NeedCheckin24hour时，对应规则4描述中
    /// “允许变更/取消,需在到店日期的24点之前几小时通知”中的几小时。
    pub hour: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PrepayRule {
    /// Description 描述 String(255) N
    pub description: String,
    /// DateType 日期类型 Enum N
    /// CheckInDay：入住日期（该字段后期下线，可以不用判断）
    pub date_type: String,
    /// StartDate 开始日期 Date Y
    /// 使用离线数据模式需要判断
    pub start_date: Option<String>,
    /// EndDate 结束日期 Date Y
    pub end_date: Option<String>,
    /// WeekSet 周有效设置 String(20) Y
    pub week_set: Option<String>,
    /// ChangeRule 变更规则 Enum N
    /// PrepayNoChange：不允许变更取消；
    /// PrepayNeedSomeDay：在到店当日24点前Hour小时前按规则看是否可以免费变更取消（一般是不收罚金），
    /// 在Hour和Hour2之间按规则存在罚金，Hour2之后不能变更取消；
    /// PrepayNeedOneTime：在约定日期时间点(DateNum + Time)前可以免费变更取消
    pub change_rule: String,
    /// Hour 第一阶段提前的几小时 Int Y
    /// 用于PrepayNeedSomeDay
    pub hour: Option<i32>,
    /// Hour2 第二阶段提前的几小时 Int Y
    pub hour2: Option<i32>,
    /// DateNum 具体取消时间日期部分 Date Y
    /// 用于PrepayNeedOneTime
    pub date_num: Option<String>,
    /// Time 具体取消时间小时部分 Time Y
    pub time: Option<String>,
    /// DeductFeesBefore 在变更时间点前是否扣费 Int Y
    /// 用于 PrepayNeedSomeDay的Hour前扣款类型（一般不收罚金）。
    /// DeductFeesBefore为1表示扣款，0表示不扣款。
    pub deduct_fees_before: Option<i32>,
    /// DeductNumBefore 时间点前扣费的金额或比例 Decimal Y
    pub deduct_num_before: Option<f64>,
    /// CashScaleFirstAfter 时间点后扣款类型 Enum Y
    /// Money：金额 Percent：比例 FristNight：首晚
    pub cash_scale_first_after: Option<String>,
    /// DeductFeesAfter 在变更时间点后是否扣费 Int Y
    /// 用于 PrepayNeedSomeDay的Hour到Hour2之间的扣款类型。
    /// DeductFeesAfter为1表示扣款，0表示不扣款。
    /// 如果CashScaleFirstAfter为FristNight，则返回-1，没有意义
    pub deduct_fees_after: Option<i32>,
    /// DeductNumAfter 时间点后扣费的金额或比例 Decimal Y
    pub deduct_num_after: Option<f64>,
    /// CashScaleFirstBefore 时间点前扣款类型 Enum Y
    /// Money：金额 Percent：比例 FristNight：首晚
    pub cash_scale_first_before: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ValueAdd {
    /// TypeCode 业务代码 String(2) N
    /// 01-早餐 02-午餐 03-晚餐 04-宽带上网 05-服务费 06-政府税
    /// 99-特殊早餐，有效日期内生效，优先级高于01早餐。
    /// 当99特殊早餐和01早餐同时存在时，需要根据特殊早餐的有效日期判断哪种早餐生效，
    /// 即在特殊早餐有效日期内99特殊早餐生效，有效日期外01早餐生效。
    pub type_code: String,
    /// Description 描述 String(255) N
    /// 附加服务描述，代理不想解析的话，可以直接显示该描述。
    pub description: String,
    /// IsInclude 是否包含在房费中 Boolean N
    /// false-不包含 true-包含，例如业务代码为早餐时，false即为不含早，true为含早。
    pub is_include: bool,
    /// Amount 包含的份数 Int Y
    pub amount: Option<i32>,
    /// CurrencyCode 货币代码 Enum N
    /// 参考Currency。
    pub currency_code: Option<String>,
    /// PriceOption 单价默认选项 Enum Y
    /// Money-金额，Percent-比例，None-无效。
    pub price_option: Option<String>,
    /// Price 单价 Decimal Y
    /// 视PriceOption表示金额或比例，比例值保存的百分数，不是最终的小数，
    /// 例如 20%， 则该字段保存为20。
    pub price: Option<f64>,
    /// IsExtAdd 是否单加 Boolean N
    /// 目前只有早餐服务该字段有意义。
    pub is_ext_add: bool,
    /// ExtOption 单加单价默认选项 Enum Y
    /// Money-金额，Percent-比例。
    pub ext_option: Option<String>,
    /// ExtPrice 单加单价 Decimal Y
    /// 视 extOption 不同表示金额或比例值, 比例值保存的百分数，不是最终的小数，
    /// 例如 20%， 则该字段保存为20。
    pub ext_price: Option<f64>,
    /// StartDate 开始日期 Date Y
    /// 特殊早餐有效日期。
    pub start_date: Option<String>,
    /// EndDate 结束日期 Date Y
    pub end_date: Option<String>,
    /// WeekSet 周有效设置 String(20) Y
    pub week_set: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Meal {
    /// Type 餐食类型 String(2) N
    /// 01-默认餐食 02-带有效期范围的餐食
    /// 当02和01同时存在时，入住日在02餐食有效日期内则02餐食生效，
    /// 入住日在02范围内解析是无餐食也不看01默认餐食了；否则有效日期外01默认餐食生效。
    /// 02餐食可能有多条，01默认餐食最多1条。
    pub r#type: String,
    /// IsInclude 是否包含在房费中 Boolean N
    /// false-不包含 true-包含，false即为不含餐食，true为含餐食
    pub is_include: bool,
    /// NumberOfBreakfast 早餐份数 Int N
    pub number_of_breakfast: i32,
    /// NumberOfLunch 午餐份数 Int N
    pub number_of_lunch: i32,
    /// NumberOfDinner 晚餐份数 Int N
    pub number_of_dinner: i32,
    /// NumberOfTypeMeal 餐食种类数量 Int N
    /// 表示早餐份数、午餐份数、晚餐份数这三个字段大于0的字段数量之和；
    /// 最小为0，最大为3
    pub number_of_type_meal: i32,
    /// NumberOfOptionalMeal 可选餐食种类数量 Int N
    /// 当NumberOfTypeMeal > NumberOfOptionalMeal时，表示动态餐食，有以下几种情况：
    /// 1、早午晚餐，三选一、三选二
    /// 2、早午餐，二选一
    /// 3、午晚餐，二选一
    /// 4、早晚餐，二选一
    /// 当NumberOfTypeMeal = NumberOfOptionalMeal时，表示固定餐食，有以下几种情况：
    /// 1、早+午+晚
    /// 2、早+午
    /// 3、早+晚
    /// 4、午+晚
    /// 5、只有早/午/晚
    /// NumberOfTypeMeal为0或NumberOfOptionalMeal为0，均为无餐食
    pub number_of_optional_meal: i32,
    /// OptionalMeals 可选餐食类型 String Y
    /// 表示可选餐食的类型，多个类型以“,”分割，只有当餐食为固定+动态餐食时才会有值。
    /// 比如餐食为早餐两份+午餐2份或晚餐2份(到店2选1)，该字段有值，为"Lunch,Dinner"
    pub optional_meals: Option<String>,
    /// Description 描述 String(500) N
    /// 餐食描述
    pub description: String,
    /// DescribeOfBreakfast 早餐描述 String Y
    pub describe_of_breakfast: Option<String>,
    /// DescribeOfLunch 午餐描述 String Y
    pub describe_of_lunch: Option<String>,
    /// DescribeOfDinner 晚餐描述 String Y
    pub describe_of_dinner: Option<String>,
    /// StartDate 开始日期 Date Y
    /// 02餐食的有效日期；01默认餐食时此字段均为空
    pub start_date: Option<String>,
    /// EndDate 结束日期 Date Y
    pub end_date: Option<String>,
    /// WeekSet 周有效设置 String(20) Y
    pub week_set: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Gift {
    /// GiftId 送礼编号 Int N
    /// 关联RatePlan.GiftId
    pub gift_id: i32,
    /// GiftDescription 礼包副标题 String Y
    /// 关于礼品副标题的描述
    pub gift_description: Option<String>,
    /// Description 描述 String N
    /// 关于礼品的描述
    pub description: String,
    /// EffectiveDates 礼包有效日期 HotelGiftDate[] N
    /// 参考HotelGiftDate节点
    pub effective_dates: Vec<HotelGiftDate>,
    /// RelatedProducts 礼包关联产品 HotelGiftProductRelation[] N
    /// 参考HotelGiftProductRelation节点
    pub related_products: Vec<HotelGiftProductRelation>,
    /// DateType 日期类型 Enum N
    /// CheckinDate:入住日 BookingDate:预订日 StayDate:在店日
    pub date_type: String,
    /// WeekSet 星期设置 String(20) N
    pub week_set: String,
    /// GiftContent 活动内容 String(2000) N
    pub gift_content: String,
    /// GiftTypes 送礼类型 String(20) N
    /// 1.礼品-含果盘 3.礼品-含水果 4.礼品-含饮品 5.礼品-含精美艺品
    /// 6.礼品-其他 7.延迟退房-延迟至13点退房 8.延迟退房-延迟至14点退房
    /// 9.延迟退房-延迟至15点退房 10.延迟退房-其他 11.餐饮-含午餐
    /// 12.餐饮-含晚餐 13.餐饮-含下午茶 14.餐饮-含餐券 15.餐饮-其他
    /// 16.旅游门票-含景点门票 17.旅游门票-含演出门票 18.旅游门票-其他
    /// 19.折扣/折扣券-含店内折扣/折扣券 20.折扣/折扣券-含外部折扣/折扣券
    /// 21.折扣/折扣券-其他 22.交通-含接站 23.交通-含接机 24.交通-含送站
    /// 25.交通-含送机 26.交通-含景区直通车 27.交通-其他 28.其他-其他
    /// 注意：1.52版本及以后该字段废弃，请使用GiftInfos字段，1.52之前的版本仍使用该字段。
    pub gift_types: String,
    /// GiftInfos 新的送礼类型 GiftInfo[] Y
    /// 参考GiftInfo字段
    pub gift_infos: Option<Vec<GiftInfo>>,
    /// HourNumber 小时数 Int Y
    pub hour_number: Option<i32>,
    /// HourType 小时数的类型 Enum Y
    /// Hours24: 全天24小时都可以送礼品
    /// XhourBefore: 几点之前送礼品
    /// XHourAfter: 几点之后送礼品
    pub hour_type: Option<String>,
    /// WayOfGiving 送礼方式 Enum N
    /// EveryRoom: 每间房送一回礼品
    /// EveryRoomPerDay: 每间房每个晚上送一回礼品
    /// Other: 其他，参考下面的描述
    pub way_of_giving: String,
    /// WayOfGivingOther 其他的送礼具体方式 String(20) Y
    /// 送礼方式为其他的时候，送礼活动的名称
    pub way_of_giving_other: Option<String>,
    /// GiftValue 礼包价值 Double Y
    /// 礼包、套餐的预估总价值
    pub gift_value: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HotelGiftDate {
    /// StartDate 礼包开始日期 DateTime N
    pub start_date: String,
    /// EndDate 礼包结束日期 DateTime N
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HotelGiftProductRelation {
    /// RoomTypeIds 关联的房型 String(500) Y
    /// All –表示所有房型（仅限同一个HotelCode）
    pub room_type_ids: Option<String>,
    /// RatePlanId 关联的RatePlan Int N
    /// 0表示所有
    pub rate_plan_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftInfo {
    /// GiftInfo 礼包一级编号 Integer N
    /// 1.含礼品 2.延迟退房 3.含餐饮 4.含旅游门票 5.含折扣/抵扣券 6.含交通 7.其他
    pub gift_info: i32,
    /// GiftSubInfos 二级礼包内容 GiftSubInfo[] N
    /// 参考GiftSubInfo节点
    pub gift_sub_infos: Vec<GiftSubInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftSubInfo {
    /// SubInfo 礼包二级编号 Integer N
    /// 1.含果盘 3.含水果 4.含饮品 5.含精美艺品 6.其他
    /// 7.延迟至13点退房 8.延迟至14点退房 9.延迟至15点退房 10.其他
    /// 11.含午餐 12.含晚餐 13.含下午茶 14.含餐券 15.其他
    /// 16.含景点门票 17.含演出门票 18.其他
    /// 19.含店内折扣/抵扣券 20.含外部折扣/抵扣券 21.其他
    /// 22.含接站 23.含接机 24.含送站 25.含送机
    /// 26.含景区直通车 27.其他 28.其他
    /// 注意：1-6对应一级编号的1，7-10对应一级编号的2，
    /// 11-15对应一级编号的3，16-18对应一级编号的4，
    /// 19-21对应一级编号的5，22-27对应一级编号的6，
    /// 28对应一级编号的7
    pub sub_info: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GiftPackage {
    /// PkgProductId 礼包套餐ID Long N
    /// 关联RatePlan.PkgProductids
    pub pkg_product_id: i64,
    /// PkgType 礼包套餐类型 Int N
    /// 0：礼包，1：套餐
    pub pkg_type: i32,
    /// Status 礼包套餐状态 Int N
    /// 0：无效，1：有效
    pub status: i32,
    /// PkgProductName 礼包套餐名字 String N
    pub pkg_product_name: String,
    /// RuleDescriptionAdditional 礼包套餐特别说明 String Y
    pub rule_description_additional: Option<String>,
    /// Pictures 礼包套餐图片 Picture[] Y
    /// 参考Picture节点
    pub pictures: Option<Vec<Picture>>,
    /// XProducts X产品列表 XProduct[] Y
    /// 参考XProduct节点
    pub x_products: Option<Vec<XProduct>>,
    /// RelatedProduct 礼包套餐关联的产品 RelatedProduct N
    pub related_product: Option<RelatedProduct>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Picture {
    /// ImgIndex 礼包图片顺序 Int N
    pub img_index: i32,
    /// ImgUrl 礼包图片链接 String N
    pub img_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct XProduct {
    /// XProductId X产品ID Long N
    /// X产品即一个具体的礼包
    pub x_product_id: i64,
    /// XProductName X产品名字 String N
    pub x_product_name: String,
    /// Status X产品状态 Int N
    /// 0：无效，1：有效
    pub status: i32,
    /// TypeName X产品类型 String Y
    pub type_name: Option<String>,
    /// Quantity X产品数量 String Y
    pub quantity: Option<String>,
    /// ReceptionTimes X产品接待时间 String Y
    pub reception_times: Option<String>,
    /// Capacity X产品适用人数 String Y
    pub capacity: Option<String>,
    /// BookingPhone X产品预订电话 String Y
    pub booking_phone: Option<String>,
    /// AppointPolicy X产品预订规则 String Y
    pub appoint_policy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RelatedProduct {
    /// RoomTypeId 供应商房型id String N
    pub room_type_id: String,
    /// RatePlanId 价格计划ID Int N
    pub rate_plan_id: i32,
}

impl BaseResponse for ElongResponse<DataRpResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<DataRpResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

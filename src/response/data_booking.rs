use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::{
    api_response::{BaseResponse, ElongResponse},
    data_rp::RatePlan,
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DataBookingResponse {
    /// RatePlan 产品信息 RatePlan Y 参考hotel.data.rp#RatePlan
    pub rate_plan: Option<RatePlan>,
    /// WeekendStart 周末价格起始日 Int Y 为0表示周末设置从周一开始为0表示到周日结束，但是两个都为0表示无周末设置；如果开始为3，结束为1，表示从周三到下周1都是周末设置1代表周一，7代表周日
    pub weekend_start: Option<i32>,
    /// WeekendEnd 周末价格结束日 Int Y
    pub weekend_end: Option<i32>,
    /// BookingRules 预订规则 BookingRule[] Y 参考BookingRule
    pub booking_rules: Option<Vec<BookingRule>>,
    /// Inventories 库存 Inventory[] Y 参考Inventory
    pub inventories: Option<Vec<Inventory>>,
    /// Rates 价格 Rate[] Y 参考Rate
    pub rates: Option<Vec<Rate>>,
    /// ObjectEffectiveStatus 对象状态 ObjectEffectiveStatus Y v1.26新增。参考ObjectEffectiveStatus节点
    pub object_effective_status: Option<ObjectEffectiveStatus>,
    /// Meals 到天餐食结果表格 Meals Y 参考hotel.detail#Meals，注意：此为移位后餐食结果表格。根据餐食原始规则，当入住日期内全部为固定餐食或入住日期内全部为半固定餐食且固定餐食类型一样时，固定餐食中的早餐、午餐会向后移一天展示；动态餐食分别与入住日期对应，不会后移一天。hasMealTable为true，表示存在餐食表格。
    pub meals: Option<Meals>,
    /// PrepayResult 预付规则结果 PrepayResult Y 参考PrepayResult
    pub prepay_result: Option<PrepayResult>,
    /// GuaranteeResult 现付规则结果 GuaranteeResult Y 参考GuaranteeResult
    pub guarantee_result: Option<GuaranteeResult>,
    /// DataSource 数据来源 Enum Y v1.32新增，特定情况使用，一般可忽略不管 DB-数据库 DC-直连
    pub data_source: Option<String>,
    /// TotalRate 总卖价 Decimal Y
    pub total_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BookingRule {
    /// TypeCode 规则类型 Enum N NeedNationality：务必提供客人国籍
    /// PerRoomPerName：您预订了N间房，请您提供不少于N的入住客人姓名
    /// ForeignerNeedEnName：此酒店要求外宾务必留英文拼写 ，港澳台酒店出现这个字段的时候，所有人都需要填写英文名或姓名拼音
    /// RejectCheckinTime：几点到几点酒店不接受预订 , 此处校验的是下单时的当前时间
    /// NeedPhoneNo：务必提供联系人手机号(请加在联系人结点Contact上)
    pub type_code: String,
    /// RoomTypeIds 关联的销售房型Id String(500) Y all 表示所有房型
    pub room_type_ids: Option<String>,
    /// Description 描述 String(255) N
    pub description: String,
    /// DateType 日期类型 Enum Y BookDay –预订日期（订单的创建日期）
    pub date_type: Option<String>,
    /// StartDate 开始日期 Date Y
    pub start_date: Option<String>,
    /// EndDate 结束日期 Date Y
    pub end_date: Option<String>,
    /// StartHour 每天开始时间 Time Y 针对日期段内每天生效, 当TypeCode 为RejectCheckinTime时表示当前预订时间在StartHour到EndHour区间内酒店不接受预订。当EndHour小于StartHour的时候是表示第二天的几点，如2:00表示第二天的2点。
    pub start_hour: Option<String>,
    /// EndHour 每天结束时间 Time Y
    pub end_hour: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Inventory {
    /// HotelID 酒店ID String(8) N 这几个属性是业务主键。Date表示的是某天的库存。HotelCode关联搜索接口的RatePlan.HotelCode
    #[serde(rename = "HotelID")]
    pub hotel_id: String,
    /// RoomTypeId 房型ID String(10) N
    pub room_type_id: String,
    /// HotelCode 酒店编码 String(8) N
    pub hotel_code: String,
    /// Date 库存时间 Date N
    pub date: String,
    /// Status 库存状态 Boolean N False-不可用 True-可用
    pub status: bool,
    /// Amount 库存数量 Int N 剩余的可知库存数量
    pub amount: i32,
    /// OverBooking 超售状态 Int N 0---可超售，1—不可超售。可超售的时候即使Amount等于0也是可以继续销售的。
    pub over_booking: i32,
    /// StartDate 可用开始日期 Date N 库存可用开始日期
    pub start_date: String,
    /// EndDate 可用结束日期 Date N 库存可用结束日期
    pub end_date: String,
    /// StartTime 可用开始时间 Time N 预订当天库存，须校验库存可用开始时间
    pub start_time: String,
    /// EndTime 可用结束时间 Time N 预订当天库存，须校验库存可用结束时间; 若为23:59:59则为无限制；
    pub end_time: String,
    /// IsInstantConfirm 当天库存是否支持即时确认 Boolean Y V1.22新增,具体使用请见接口使用说明。注意：此三个字段已无效，是否即时确认请以创建订单接口的返回值或者订单详情中的即时确认相关字段为准。
    pub is_instant_confirm: Option<bool>,
    /// IC_BeginTime 预订当天即时确认可用开始时间 Time Y
    pub ic_begin_time: Option<String>,
    /// IC_EndTime 预订当天即时确认可用结束时间 Time Y
    pub ic_end_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Rate {
    /// HotelID 酒店编号 String(8) N 这几个属性是业务主键
    #[serde(rename = "HotelID")]
    pub hotel_id: String,
    /// RoomTypeId 房型编号 String(10) N
    pub room_type_id: String,
    /// RateplanId 产品编号 Int N
    pub rateplan_id: i32,
    /// StartDate 开始时间 Date N
    pub start_date: String,
    /// EndDate 结束时间 Date N
    pub end_date: String,
    /// HotelCode 酒店编码 String(8) Y v1.13新增
    pub hotel_code: Option<String>,
    /// Status 状态 Boolean N false--无效、true--有效
    pub status: bool,
    /// Member 平日卖价 Decimal N -1代表此房无价，无价和满房都不能进行预订
    pub member: Decimal,
    /// Weekend 周末卖价 Decimal N 同上，在周末时使用此价格，周末设置参考hotel.data.rp接口
    pub weekend: Decimal,
    /// MemberCost 平日结算价 Decimal N 同上，开通了结算价模式的接入方才可以使用
    pub member_cost: Decimal,
    /// WeekendCost 周末结算价 Decimal N 同上，开通了结算价模式的接入方才可以使用
    pub weekend_cost: Decimal,
    /// AddBed 加床价 Decimal Y V1.01新增 -1代表不能加床，0-免费加床，大于0表示加床的费用
    pub add_bed: Option<Decimal>,
    /// PriceID 价格ID Long Y V1.08新增
    pub price_id: Option<i64>,
    /// CurrencyCode 货币类型 String Y V1.08新增参考Currency
    pub currency_code: Option<String>,
    /// UsedPromotionDayRoomValues 每间的同程促销明细 UsedPromotionDayRoomValue[] Y 每间的同程促销明细,参考 UsedPromotionDayRoomValue节点
    pub used_promotion_day_room_values: Option<Vec<UsedPromotionDayRoomValue>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct UsedPromotionDayRoomValue {
    /// RoomNumber 房间号 String Y
    pub room_number: Option<String>,
    /// PromotionDetailList 促销明细 PromotionDetail[] Y 此间促销明细
    pub promotion_detail_list: Option<Vec<PromotionDetail>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionDetail {
    /// Amount 促销金额 Decimal Y
    pub amount: Option<Decimal>,
    /// PromotionType 促销类型 Int Y 促销类型 9:立减
    pub promotion_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectEffectiveStatus {
    /// HotelIdStatus HotelId 对应对象的状态 Boolean N true --有效，false--无效
    pub hotel_id_status: bool,
    /// RoomStatus RoomId对应对象的状态 Boolean N
    pub room_status: bool,
    /// RatePlanStatus RatePlanId对应对象的状态 Boolean N
    pub rate_plan_status: bool,
    /// HotelCodeStatus HotelCode对应对象的状态 Boolean N
    pub hotel_code_status: bool,
    /// RoomTypeStatus RoomTypeId对应对象的状态 Boolean N
    pub room_type_status: bool,
    /// ProductRelation RoomTypeId和RatePlanId是否存在绑定关系 Boolean N
    pub product_relation: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Meals {
    /// hasMealTable 是否存在餐食表格 Boolean N 为true代表取“dayMealTable”餐食表格字段，查看每天的餐食情况
    pub has_meal_table: bool,
    /// mealCopyWriting 餐食文案描述 String N 总餐食描述
    pub meal_copy_writing: String,
    /// dayMealTable 每日餐食表格 List<DayMeal> N 包含多个DayMeal，见DayMeal节点；具体场景demo可见：https://open.elong.com/faq/detail?id=312&plt=2
    pub day_meal_table: Vec<DayMeal>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DayMeal {
    /// date 餐食的日期 String N yyyy-MM-dd格式，例如2021-08-12
    pub date: String,
    /// useDynamicMeal 是否使用动态餐食 Boolean N 为true取dynamicMealDesc；为false取breakfastDesc、lunchDesc、dinnerDesc;
    pub use_dynamic_meal: bool,
    /// dynamicMealDesc 动态餐食描述 String Y 例如：3种餐食(3选2)
    pub dynamic_meal_desc: Option<String>,
    /// breakfastShare 早餐数量 Int N 例如：0
    pub breakfast_share: i32,
    /// breakfastDesc 早餐描述 String Y
    pub breakfast_desc: Option<String>,
    /// lunchShare 午餐数量 Int N 例如：1
    pub lunch_share: i32,
    /// lunchDesc 午餐描述 String Y 例如：1份午餐
    pub lunch_desc: Option<String>,
    /// dinnerShare 晚餐数量 Int N 例如：2
    pub dinner_share: i32,
    /// dinnerDesc 晚餐描述 String Y 例如：2份晚餐
    pub dinner_desc: Option<String>,
    /// dayMealCopyWriting 到天餐食描述 String Y 例如：2份早餐/间
    pub day_meal_copy_writing: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PrepayResult {
    /// CancelDescription 取消规则 String N
    pub cancel_description: String,
    /// CancelType 取消类型 int N 1：免费取消；2：收费取消；3：限时取消；4：不可取消
    pub cancel_type: i32,
    /// LadderVouch 是否使用阶梯担保规则 boolean Y 废弃，此字段无意义，与LadderParseList节点无关，可以忽略
    pub ladder_vouch: Option<bool>,
    /// LadderParseList 取消规则明细 LadderParse[] N 参考LadderParse节点
    pub ladder_parse_list: Vec<LadderParse>,
    /// CancelTag 取消规则标签 String Y 如果规则是任意取消和不可取消的没有这个字段和对应值，限时取消和付费取消则会返回该字段
    pub cancel_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LadderParse {
    /// BeginTime 开始时间 long N
    pub begin_time: i64,
    /// EndTime 结束时间 long N
    pub end_time: i64,
    /// CutType 扣费类型 int N 0:不扣费；1:金额；2：比例；3：首晚房费；4：全额房费；
    /// 请注意：hotel.detail、hotel.data.booking、hotel.order.detail接口中CutType、CutValue存在不一致的情况，
    /// 对客展示建议直接使用具体扣费金额：AmountRmb（兜底当扣费金额AmountRmb大于订单金额时，展示不可取消）
    pub cut_type: i32,
    /// CutValue 扣费值 Decimal N 原始币种
    pub cut_value: Decimal,
    /// Amount 扣费值 Decimal N 国际现付的是原币，预付对客的是人民币，预付对酒店的是原币。
    pub amount: Decimal,
    /// ShortDesc 短文案 String Y
    pub short_desc: Option<String>,
    /// AmountRmb 扣费值 Decimal N 人民币
    pub amount_rmb: Decimal,
    /// ExchangeRate 汇率 Decimal Y
    pub exchange_rate: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GuaranteeResult {
    /// MoneyArrivalTime 到店时间触发时的担保金额 Double N （废弃）货币类型为原币种，即所在RatePlan节点下的CurrencyCode
    pub money_arrival_time: f64,
    /// ArrivalTime 需要担保的到店时间(格式：hh:mm) String N 用户填写的最晚到店早于此时间，不需要关注MoneyArrivalTime，否则需要担保MoneyArrivalTime
    pub arrival_time: String,
    /// RoomCount 房量担保分割点 Integer N 用户预订间数小于此值不需要担保，否则需要担保
    pub room_count: i32,
    /// GuaranteeType 担保类型 Integer N 二进制位表示:1:到店时间担保,2:房量担保,3:预订即需担保,4:免担保
    /// 0：免担保；2：到店时间担保；4：房量担保；6：到店时间或房量担保；8：预定即需担保
    pub guarantee_type: i32,
    /// GuaranteeMoney 当前条件下需要担保的金额 Double N 具体见取消规则明细
    pub guarantee_money: f64,
    /// NeedGuarantee 当前条件下是否需要担保 Boolean N
    pub need_guarantee: bool,
    /// CancelTime 可以取消的时间点，单位秒 Long N （废弃）之前可以取消，之后不可取消，不可取消时:为-28800，免费取消时:Long.MAX_VALUE，其他情况下为北京时间的时间戳
    pub cancel_time: i64,
    /// CancelDescription 取消规则详细描述 String Y
    pub cancel_description: Option<String>,
    /// CancelTag 取消规则标签 String Y 如果规则是任意取消和不可取消的没有这个字段和对应值,限时取消和付费取消则会返回该字段
    pub cancel_tag: Option<String>,
    /// CancelType 取消类型 Integer N 1. 免费取消、2.付费取消、3.可取消、4.不可取消
    pub cancel_type: i32,
    /// LadderParseList 取消规则明细 LadderParse[] N 参考LadderParse节点
    pub ladder_parse_list: Vec<LadderParse>,
}

impl BaseResponse for ElongResponse<DataBookingResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<DataBookingResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

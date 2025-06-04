use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderDetailReponse {
    /// 订单编号
    pub order_id: i64,

    /// 酒店编号
    pub hotel_id: String,

    /// 酒店名称（V1.03新增）
    pub hotel_name: Option<String>,

    /// 房型编号（V1.36新增）
    pub room_id: Option<String>,

    /// 房型名称
    pub room_name: String,

    /// 销售房型编号
    pub room_type_id: String,

    /// 销售房型名称（V1.03新增，仅作为参考，不应展示给客人）
    pub room_type_name: Option<String>,

    /// 产品编号
    pub rate_plan_id: i32,

    /// 产品名称（V1.05新增）
    pub rate_plan_name: Option<String>,

    /// 入住日期
    pub arrival_date: String,

    /// 离店日期
    pub departure_date: String,

    /// 订单状态
    /// A-已确认, B-NO SHOW, C-已结帐, D-删除, E-取消, F-已入住, G-变价, H-变更,
    /// N-新单, O-满房, S-特殊, U-特殊满房, V-已审, Z-删除,另换酒店
    pub status: String,

    /// 对用户展示的订单状态（V1.12新增）
    pub show_status: Option<i64>,

    /// 下一次确认反馈时间点（V1.12新增）
    pub confirm_point: Option<String>,

    /// 客人类型
    /// All=统一价, Chinese=内宾价, OtherForeign=外宾价, HongKong=港澳台客人价, ChinaGuest=中宾价
    pub customer_type: String,

    /// 付款类型
    /// SelfPay-前台现付, Prepay-预付
    pub payment_type: String,

    /// 房间数量
    pub number_of_rooms: i32,

    /// 客人数量
    pub number_of_customers: i32,

    /// 最早到店时间
    pub earliest_arrival_time: String,

    /// 最晚到店时间
    pub latest_arrival_time: String,

    /// 货币类型
    /// RMB, HKD, MOP, TWD, USD
    pub currency_code: String,

    /// 总价
    pub total_price: f64,

    /// 艺龙会员卡号（V1.01新增）
    pub elong_card_no: Option<String>,

    /// 确认类型
    /// NotAllowedConfirm, SMS_cn, NoNeed
    pub confirmation_type: String,

    /// 给酒店备注
    pub note_to_hotel: Option<String>,

    /// 给艺龙备注
    pub note_to_elong: Option<String>,

    /// 给客人的备注（V1.06新增）
    pub note_to_guest: Option<String>,

    /// 订单产生的罚金
    pub penalty_to_customer: Option<f64>,

    /// 罚金货币类型
    pub penalty_currency_code: Option<String>,

    /// 是否可退款（V1.35新增）
    pub is_refund: Option<bool>,

    /// 预订时间（V1.08新增）
    pub creation_date: Option<String>,

    /// 当前是否可以取消（V1.07新增）
    pub is_cancelable: Option<bool>,

    /// 最晚取消时间
    pub cancel_time: String,

    /// 是否有发票信息
    pub has_invoice: Option<bool>,

    /// 产品是否支持开专票
    /// 0：不支持, 1：支持
    pub support_anticipation: Option<String>,

    /// 发票信息
    /// 参考 Invoice 节点
    pub invoice: Option<Invoice>,

    /// 臻选特惠促销信息
    /// 参考 DayPromotion 节点
    pub day_promotions: Option<Vec<DayPromotion>>,

    /// 同程促销促销信息
    /// 参考 OrderDetailDayRoomPromotion 节点
    pub used_promotion_day_room_values: Option<Vec<OrderDetailDayRoomPromotion>>,

    /// 联系人信息
    /// 参考 Contact 节点
    pub contact: Contact,

    /// 信用卡信息
    /// 参考 CreditCard 节点
    pub credit_card: Option<CreditCard>,

    /// 每夜价格
    /// 参考 NightlyRate 节点
    pub nightly_rates: Option<Vec<NightlyRate>>,

    /// 扩展信息
    /// 参考 ExtendInfo 节点
    pub extend_info: Option<ExtendInfo>,

    /// 房间信息
    /// 参考 OrderRoom 节点
    pub order_rooms: Vec<OrderRoom>,

    /// 担保规则
    /// 参考 GuaranteeRule 节点
    pub guarantee_rule: Option<GuaranteeRule>,

    /// 预付规则
    /// 参考 PrepayRule 节点
    pub prepay_rule: Option<PrepayRule>,

    /// 预付结果
    /// 参考 PrepayResult 节点
    pub prepay_result: Option<PrepayResult>,

    /// 担保结果
    /// 参考 GuaranteeResult 节点
    pub guarantee_result: Option<GuaranteeResult>,

    /// 多次退款详情
    /// 参考 RefundDetail 节点
    pub refund_detail: Option<RefundDetail>,

    /// 增值服务
    pub value_adds: Option<Vec<String>>,

    /// 礼包套餐
    /// 参考 GiftPackage 节点
    pub gift_packages: Option<Vec<GiftPackage>>,

    /// 预付订单的发票开具模式（V1.11新增）
    /// Elong-艺龙开发票, Hotel-酒店开发票
    pub invoice_mode: Option<String>,

    /// 换算为人民币的订单总卖价（V1.28新增）
    pub total_price_exchanged: Option<f64>,

    /// 换算为人民币的订单总底价（V1.28新增）
    pub total_cost_price_exchanged: Option<f64>,

    /// 是否及时确认（V1.28新增）
    pub is_instant_confirm: Option<bool>,

    /// 代理自己的订单号（V1.28新增）
    pub affiliate_confirmation_id: Option<String>,

    /// 订单关联的酒店信息（V1.28新增）
    /// 参考 OrderHotel 节点
    pub order_hotel: Option<OrderHotel>,

    /// 预付订单线下退款金额（V1.29新增）
    pub refund_amount: Option<f64>,

    /// 销售给客人的最终价格
    pub customer_price: Option<f64>,

    /// 实际支付金额
    pub pay_amount: Option<f64>,

    /// 是否为钟点房（V1.55新增）
    pub hour_room: Option<bool>,

    /// 钟点房入住开始时间
    pub hour_room_start_time: Option<String>,

    /// 钟点房入住结束时间
    pub hour_room_end_time: Option<String>,

    /// 餐食信息
    /// 参考 DayMeal 节点
    pub meals: Option<Vec<DayMeal>>,

    /// 特殊取消申请
    pub special_cancel_apply: bool,

    /// 早餐（国际特有字段）
    pub breakfast: Option<String>,

    /// 基础预付规则（国际特有字段）
    /// 参考 BasePrepayRule 节点
    pub base_prepay_rule: Option<BasePrepayRule>,

    /// 供应商ID（国际特有字段）
    pub supplier_id: Option<String>,

    /// 二级供应商ID（国际特有字段）
    pub sub_supplier_id: Option<String>,

    /// 特殊要求（国际特有字段）
    pub specific_remark: Option<String>,

    /// 税和服务费原币种（国际特有字段）
    pub tax_and_service: Option<f64>,

    /// 税和服务费人民币币种（国际特有字段）
    pub tax_and_service_rmb: Option<f64>,

    /// 额外人员费用（附加费人民币）（国际特有字段）
    pub extra_person_fee_rmb: Option<f64>,

    /// 入住需知（国际特有字段）
    pub check_in_instructions: Option<String>,

    /// 床型描述（国际特有字段）
    pub bed_description: Option<String>,

    /// 膳食信息（国际特有字段）
    /// 参考 BoardInfo 节点
    pub board_info: Option<BoardInfo>,

    /// 房间最大可住成人数（国际特有字段）
    pub occupancy_per_room: Option<i32>,

    /// 房间最大可住儿童数（国际特有字段）
    pub children_occupancy_per_room: Option<i32>,

    /// 成人数（国际特有字段）
    pub number_of_adults: Option<i32>,

    /// 儿童年龄（国际特有字段）
    pub child_ages: Option<Vec<i32>>,

    /// 是否经历过确认前取消（国际特有字段）
    pub is_cancel_before_confirmation: Option<bool>,

    /// 网络类型（国际特有字段）
    pub internet_type: Option<String>,

    /// 上网描述（国际特有字段）
    pub internet_desc: Option<String>,

    /// 床型ID（国际特有字段）
    pub bed_type_id: Option<String>,

    /// 网络（国际特有字段）
    pub internet: Option<String>,

    /// 另付税和服务费（国际特有字段）
    /// 参考 AdditionalTax 节点
    pub additional_tax: Option<AdditionalTax>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalTax {
    /// 另付税和服务费总额（人民币）
    pub total_amount_rmb: Option<f64>,

    /// 另付税和服务费总额（支付币）
    pub total_amount_paid: Option<f64>,

    /// 另付税和服务费明细
    /// 参考 AdditionalTaxItem 节点
    pub additional_tax_items: Option<Vec<AdditionalTaxItem>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalTaxItem {
    /// 另付税和服务费明细描述
    pub description: Option<String>,

    /// 另付税和服务费明细金额
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GuaranteeRule {
    /// 担保规则编号
    pub gurantee_rule_id: i32,

    /// 描述
    /// 如果担保规则存在，将此字段展示给用户
    pub description: String,

    /// 日期类型
    /// BookDay-预定日期, CheckInDay-入住日期, StayDay-在店日期
    pub date_type: String,

    /// 开始日期
    pub start_date: String,

    /// 结束日期
    pub end_date: String,

    /// 周有效天数
    pub week_set: Option<String>,

    /// 是否到店时间担保
    /// False: 不校验到店时间, True: 需要校验到店时间
    pub is_time_guarantee: bool,

    /// 到店担保开始时间
    /// 用于 is_time_guarantee == true 进行检查
    pub start_time: Option<String>,

    /// 到店担保结束时间
    /// 当 end_time 小于 start_time 时，默认从 start_time 到次日6点都需要担保
    pub end_time: Option<String>,

    /// 到店担保的结束时间是否为第二天
    /// false 为当天，true 为次日
    pub is_tomorrow: Option<bool>,

    /// 是否房量担保
    /// False: 不校验房量条件, True: 校验房量条件
    /// 如果 is_time_guarantee 和 is_amount_guarantee 都为 false 则为强制担保
    pub is_amount_guarantee: bool,

    /// 担保的房间数
    /// 预定几间房及以上要担保，用于 is_amount_guarantee == true 进行检查
    pub amount: Option<i32>,

    /// 担保类型
    /// FirstNightCost 为首晚房费担保, FullNightCost 为全额房费担保
    pub guarantee_type: Option<String>,

    /// 变更规则
    /// NoChange: 不允许变更取消
    /// NeedSomeDay: 允许变更/取消, 需在 XX 日 YY 时之前通知
    /// NeedCheckinTime: 允许变更/取消, 需在最早到店时间之前几小时通知
    /// NeedCheckin24hour: 允许变更/取消, 需在到店日期的24点之前几小时通知
    pub change_rule: Option<String>,

    /// 日期参数
    /// ChangeRule = NeedSomeDay 时，对应规则中 "允许变更/取消, 需在 XX 日 YY 时之前通知" 中的 XX 日
    pub day: Option<String>,

    /// 时间参数
    /// ChangeRule = NeedSomeDay 时，对应规则中 "允许变更/取消, 需在 XX 日 YY 时之前通知" 中的 YY 时
    pub time: Option<String>,

    /// 小时参数
    /// ChangeRule = NeedCheckinTime 时，对应规则中 "允许变更/取消, 需在最早到店时间之前几小时通知" 中的几小时
    /// ChangeRule = NeedCheckin24hour 时，对应规则中 "允许变更/取消, 需在到店日期的24点之前几小时通知" 中的几小时
    pub hour: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PrepayRule {
    /// 规则编号
    pub prepay_rule_id: i32,

    /// 描述
    pub description: String,

    /// 日期类型
    /// BookDay: 预定日期, CheckInDay: 入住日期, StayDay: 在店日期
    pub date_type: String,

    /// 开始日期
    pub start_date: Option<String>,

    /// 结束日期
    pub end_date: Option<String>,

    /// 周有效设置
    pub week_set: Option<String>,

    /// 变更规则
    /// PrepayNoChange: 不允许变更取消
    /// PrepayNeedSomeDay: 按规则看是否可以免费变更取消
    /// PrepayNeedOneTime: 在约定时间点前可以免费变更取消
    pub change_rule: String,

    /// 第一阶段提前的几小时（用于 PrepayNeedSomeDay）
    pub hour: Option<i32>,

    /// 第二阶段提前的几小时（用于 PrepayNeedSomeDay）
    pub hour2: Option<i32>,

    /// 具体取消时间日期部分（用于 PrepayNeedOneTime）
    pub date_num: Option<String>,

    /// 具体取消时间小时部分（用于 PrepayNeedOneTime）
    pub time: Option<String>,

    /// 在变更时间点前是否扣费（用于 PrepayNeedSomeDay 的 Hour 前扣款类型）
    /// DeductFeesBefore 为 1 表示扣费，0 表示不扣费
    pub deduct_fees_before: Option<i32>,

    /// 时间点前扣费的金额或比例
    pub deduct_num_before: Option<f64>,

    /// 时间点后扣款类型（用于 PrepayNeedOneTime）
    /// Money: 金额, Percent: 比例, FristNight: 首晚
    pub cash_scale_first_after: Option<String>,

    /// 在变更时间点后是否扣费（用于 PrepayNeedSomeDay 的 Hour 到 Hour2 之间的扣款类型）
    /// DeductFeesAfter 为 1 表示扣费，0 表示不扣费
    pub deduct_fees_after: Option<i32>,

    /// 时间点后扣费的金额或比例
    pub deduct_num_after: Option<f64>,

    /// 时间点前扣款类型（用于 PrepayNeedOneTime）
    /// Money: 金额, Percent: 比例, FristNight: 首晚
    pub cash_scale_first_before: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PrepayResult {
    /// 取消规则
    pub cancel_description: String,

    /// 取消规则明细
    /// 参考 LadderParse 节点
    pub ladder_parse_list: Vec<LadderParse>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GuaranteeResult {
    /// 取消规则
    pub cancel_description: String,

    /// 是否需要担保
    pub need_guarantee: bool,

    /// 担保的金额
    pub guarantee_money: Option<f64>,

    /// 取消规则明细
    /// 参考 LadderParse 节点
    pub ladder_parse_list: Option<Vec<LadderParse>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LadderParse {
    /// 开始时间
    pub begin_time: i64,

    /// 结束时间
    pub end_time: i64,

    /// 扣费类型
    /// 0: 不扣费；1: 金额；2: 比例；3: 首晚房费；4: 全额房费
    pub cut_type: i32,

    /// 扣费值（原始币种）
    pub cut_value: f64,

    /// 扣费值
    /// 国际现付的是原币，预付对客的是人民币，预付对酒店的是原币
    pub amount: f64,

    /// 短文案
    pub short_desc: Option<String>,

    /// 扣费值（人民币）
    pub amount_rmb: f64,

    /// 汇率
    pub exchange_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Invoice {
    /// 发票类型
    /// v1.31新增。Paper-纸质发票，Electronic-电子发票，SpecialPaper-专用发票。
    pub invoice_type: String,

    /// 抬头类型
    /// Personally-个人，Enterprise-企业，Government-政府机关行政单位，默认为企业。
    pub title_type: String,

    /// 抬头
    pub title: String,

    /// 纳税人识别号/统一社会信用代码
    /// 字符长度是15、18或20位，是数字和字母的组合或纯数字。
    pub itin: String,

    /// 发票内容
    /// 代订房费或代订住宿费。
    pub item_name: String,

    /// 金额
    pub amount: f64,

    /// 收件人
    /// 参考 Recipient 节点。
    pub recipient: Recipient,

    /// 是否添加发票备注
    /// true-在发票备注栏中添加酒店预订信息（酒店名称、入住日期、离店日期、房间数）。
    /// false-不添加，默认值。
    pub is_need_relation_order: Option<bool>,

    /// 发票备注内容
    pub memo_info: Option<String>,

    /// 纸质发票状态
    /// false--未处理、true--已开票。
    pub status: bool,

    /// 纸质发票邮寄状态
    /// false--未邮寄、true--已邮寄。
    pub delivery_status: bool,

    /// 电子发票处理类型
    /// v1.31新增。0 开票，1 红冲，2 修改。
    pub process_type: Option<i32>,

    /// 电子发票处理状态
    /// v1.31新增。0 未处理，1 处理中，2 成功，3 失败。
    pub process_status: Option<i32>,

    /// 电子发票PDF下载URL
    /// v1.31新增。
    pub url_for_pdf: Option<String>,

    /// 电子发票加入微信卡券的链接URL
    /// v1.31新增。可将该链接转为二维码供用户扫码，扫描后自动将该电子发票添加至微信卡包。
    pub url_for_weixin_card: Option<String>,

    /// 电子发票发票代码
    /// v1.31新增。
    pub inv_code: Option<String>,

    /// 电子发票发票号
    /// v1.31新增。
    pub inv_number: Option<String>,

    /// 电子发票单据号
    /// v1.31新增。
    pub bill_number: Option<String>,

    /// 纳税人识别号（专票必传）
    pub tax_payer_num: Option<String>,

    /// 开户银行（专票必传）
    pub tax_register_bank: Option<String>,

    /// 银行账号（专票必传）
    pub register_bank_num: Option<String>,

    /// 注册地址（专票必传）
    pub register_address: Option<String>,

    /// 电话（专票必传）
    pub register_phone_num: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Recipient {
    /// 省份
    pub province: String,

    /// 城市
    pub city: String,

    /// 行政区
    pub district: String,

    /// 街道
    pub street: String,

    /// 邮编
    pub postal_code: Option<String>,

    /// 收件人姓名
    pub name: String,

    /// 电话
    pub phone: String,

    /// Email
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DayPromotion {
    /// 日期，格式为 yyyy-MM-dd
    pub date: String,

    /// 促销信息
    /// 参考 Promotion 节点
    pub promotions: Vec<Promotion>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Promotion {
    /// 卖价优惠的金额
    pub price_discount_value: Option<f64>,

    /// 促销名称
    pub promotion_tag: Option<String>,

    /// 促销ID
    pub promotion_id: Option<i64>,

    /// 促销类型
    /// 0: 未定义, 1: 天天特价, 2: 门店新客, 3: 优享会, 4: 其他促销, 5: 权益云
    pub promotion_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderDetailDayRoomPromotion {
    /// 促销日期，格式为 yyyy-MM-dd
    pub stay_date: String,

    /// 促销日明细
    /// 参考 PromotionDayRoom 节点
    pub promotion_day_room: Vec<PromotionDayRoom>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionDayRoom {
    /// 房间编号
    pub room_number: String,

    /// 促销明细
    /// 参考 PromotionDayRoomItem 节点
    pub promotion_detail_list: Vec<PromotionDayRoomItem>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionDayRoomItem {
    /// 促销金额
    pub amount: f64,

    /// 促销类型
    /// 9 - 立减, 11 - 红包
    pub promotion_type: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Contact {
    /// 姓名
    pub name: String,

    /// Email
    pub email: Option<String>,

    /// 手机号区号
    pub mobile_area_code: Option<String>,

    /// 手机号
    pub mobile: Option<String>,

    /// 电话
    pub phone: Option<String>,

    /// 传真
    pub fax: Option<String>,

    /// 性别
    /// Female 女, Male 男, Unknown 保密
    pub gender: Option<String>,

    /// 证件类型
    /// 身份证 IdentityCard, 护照 Passport, 其他 Other
    pub id_type: Option<String>,

    /// 证件号码
    pub id_no: Option<String>,

    /// 名（国际特有字段，已废弃）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// 姓（国际特有字段，已废弃）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreditCard {
    /// 交易类型
    /// Auth—授权, CancelAuth-取消授权, Charge-授权后扣款, Refund-退款, DirectCharge-直接扣款
    pub process_type: Option<String>,

    /// 交易状态
    /// UnProcess-未处理, Succeed-成功, Processing-处理中, Fail-失败
    pub status: Option<String>,

    /// 交易金额
    /// 注意：预付订单在 ProcessType 为 Refund 时，此处金额不一定是实际退款金额，还需要结合 RefundAmount 字段查看
    pub amount: Option<f64>,

    /// 备注或失败原因
    /// v1.19新增，仅订单创建时使用新支付流程
    pub notes: Option<String>,

    /// 是否可以继续支付
    /// v1.19新增，仅订单创建时使用新支付流程
    pub is_payable: Option<bool>,

    /// 最晚支付时间
    /// v1.19新增，仅订单创建时使用新支付流程
    /// 格式为 yyyy-MM-dd HH:mm:ss
    pub latest_pay_time: Option<String>,

    /// 有效年份（已弃用，仅有默认值0）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_year: Option<i32>,

    /// 有效月份（已弃用，仅有默认值0）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_month: Option<i32>,

    /// 证件类型（已弃用，仅有默认值 IdentifyCard）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct NightlyRate {
    /// 日期，格式为 yyyy-MM-dd
    pub date: String,

    /// 会员卖价
    pub member: f64,

    /// 结算价（仅结算价模式下有值）
    pub cost: Option<f64>,

    /// 早餐数量（V1.24版本新增）
    pub breakfast_count: Option<i32>,

    /// 加床价（已弃用，仅有默认值0）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_bed: Option<f64>,

    /// 状态（已弃用，仅有默认值false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,

    /// 税和服务费（原币种，国际特有字段）
    pub tax_and_service_fee: Option<f64>,

    /// 税和服务费（人民币币种，国际特有字段）
    pub tax_and_service_fee_rmb: Option<f64>,

    /// 房价（国际特有字段）
    pub room_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ExtendInfo {
    /// 可以存放合作伙伴自己的一些信息
    /// (Api不会改变这里的信息)，有利于获取订单后进行一些渠道分析等
    pub string1: Option<String>,

    /// 扩展字段
    pub string2: Option<String>,

    /// 扩展字段
    pub string3: Option<String>,

    /// 扩展字段
    pub int1: Option<i32>,

    /// 扩展字段
    pub int2: Option<i32>,

    /// 扩展字段
    pub int3: Option<i32>,

    /// Web传入的参数（V1.13新增）
    pub partner_parameter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderRoom {
    /// 客人信息
    /// 参考 Customer 节点
    pub customers: Vec<Customer>,

    /// 入住房间号（v1.20新增）
    pub room_no: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Customer {
    /// 姓名
    pub name: String,

    /// Email（已不再使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// 手机（已不再使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,

    /// 电话（已不再使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// 传真（已不再使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,

    /// 性别
    pub gender: Option<String>,

    /// 证件类型（已不再使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_type: Option<i32>,

    /// 证件号码（已不再使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_no: Option<String>,

    /// 国籍
    pub nationality: Option<String>,

    /// 酒店确认号
    /// 酒店自己的订单号
    pub confirmation_number: Option<String>,

    /// 名（国际特有字段）
    pub first_name: Option<String>,

    /// 姓（国际特有字段）
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderHotel {
    /// 酒店ID
    pub hotel_id: String,

    /// 酒店名称
    pub name: String,

    /// 酒店地址
    pub address: String,

    /// 酒店电话
    pub phone: Option<String>,

    /// 城市名称
    pub city_name: String,

    /// 酒店英文名（国际特有字段）
    pub hotel_english_name: String,

    /// 国家名称（国际特有字段）
    pub hotel_country_name: Option<String>,

    /// 维度（国际特有字段）
    pub hotel_latitude: Option<f64>,

    /// 经度（国际特有字段）
    pub hotel_longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BoardInfo {
    /// 是否包含早餐（国际特有字段）
    pub is_breakfast_included: Option<bool>,

    /// 是否半膳（国际特有字段）
    pub half_board_included: Option<bool>,

    /// 是否全膳（国际特有字段）
    pub full_board_included: Option<bool>,

    /// 膳食描述（国际特有字段）
    pub board_desc: Option<String>,

    /// 膳食明细（国际特有字段）
    /// 参考 BoardDetail 节点
    pub board_details: Option<Vec<BoardDetail>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BasePrepayRule {
    /// 描述（国际特有字段）
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct BoardDetail {
    /// 描述（国际特有字段）
    pub description: Option<String>,

    /// 膳食数量（国际特有字段）
    pub count: Option<i32>,

    /// 膳食类型（国际特有字段）
    /// 1：早餐；2：午餐；3：晚餐；0：未知餐型
    pub r#type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct GiftPackage {
    /// 礼包套餐ID
    /// 关联 RatePlan.PkgProductIds
    pub pkg_product_id: i64,

    /// 礼包套餐类型
    /// 0：礼包，1：套餐
    pub pkg_type: i32,

    /// 礼包套餐名字
    pub pkg_product_name: Option<String>,

    /// 礼包套餐特别说明
    pub rule_description_additional: Option<String>,

    /// 礼包套餐图片
    /// 参考 Picture 节点
    pub pictures: Option<Vec<Picture>>,

    /// X产品列表
    /// 参考 XProduct 节点
    pub x_products: Option<Vec<XProduct>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct XProduct {
    /// X产品ID
    /// X产品即一个具体的礼包
    pub x_product_id: i64,

    /// X产品名字
    pub x_product_name: String,

    /// X产品类型
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
    /// 例：“使用地点：酒店内”
    pub ticket_use_site_desc: Option<String>,

    /// X特别说明
    pub freestyle_desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Picture {
    /// 礼包图片顺序
    pub img_index: i32,

    /// 礼包图片链接
    pub img_url: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DayMeal {
    /// 餐食的日期，格式为 yyyy-MM-dd，例如 2021-08-12
    pub date: String,

    /// 是否使用动态餐食
    /// 为 true 取 dynamicMealDesc；为 false 取 breakfastDesc、lunchDesc、dinnerDesc
    pub use_dynamic_meal: bool,

    /// 动态餐食描述，例如：3种餐食(3选2)
    pub dynamic_meal_desc: Option<String>,

    /// 早餐数量，例如：0
    pub breakfast_share: i32,

    /// 早餐描述
    pub breakfast_desc: Option<String>,

    /// 午餐数量，例如：1
    pub lunch_share: i32,

    /// 午餐描述，例如：1份午餐
    pub lunch_desc: Option<String>,

    /// 晚餐数量，例如：2
    pub dinner_share: i32,

    /// 晚餐描述，例如：2份晚餐
    pub dinner_desc: Option<String>,

    /// 到天餐食描述，例如：2份早餐/间
    pub day_meal_desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct RefundDetail {
    /// 总的退款金额
    pub refund_amount: f64,

    /// 每笔退款明细
    /// 参考 Refund 节点
    pub refund_details: Vec<Refund>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Refund {
    /// 每笔退款金额
    pub refund_amount: f64,

    /// 每笔退款的时间
    /// 格式为 yyyy-MM-dd HH:mm:ss
    pub refund_time: String,
}

impl BaseResponse for ElongResponse<OrderDetailReponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<OrderDetailReponse> json: {}", json);
        print!("ElongResponse<OrderDetailReponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

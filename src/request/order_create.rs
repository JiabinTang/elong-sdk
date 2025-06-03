use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderCreateRequest {
    /// 合作伙伴订单确认号
    pub affiliate_confirmation_id: String,

    /// 酒店编号
    pub hotel_id: String,

    /// 展示房型编号
    pub room_id: Option<String>,

    /// 销售房型编号
    pub room_type_id: String,

    /// 产品编号
    pub rate_plan_id: i32,

    /// 入住日期，格式为 yyyy-MM-dd
    pub arrival_date: String,

    /// 离店日期，格式为 yyyy-MM-dd
    pub departure_date: String,

    /// 钟点房入住开始时间，格式为 yyyy-MM-dd HH:mm:ss
    pub hour_room_start_time: Option<String>,

    /// 钟点房入住结束时间，格式为 yyyy-MM-dd HH:mm:ss
    pub hour_room_end_time: Option<String>,

    /// 客人类型（已废弃）
    pub customer_type: Option<String>,

    /// 付款类型
    /// SelfPay-前台现付、Prepay-预付
    pub payment_type: String,

    /// 房间数量
    pub number_of_rooms: i32,

    /// 客人数量
    pub number_of_customers: i32,

    /// 最早到店时间，格式为 yyyy-MM-dd HH:mm:ss
    pub earliest_arrival_time: String,

    /// 最晚到店时间，格式为 yyyy-MM-dd HH:mm:ss
    pub latest_arrival_time: String,

    /// 货币类型
    pub currency_code: String,

    /// 总价
    pub total_price: f64,

    /// 客人访问IP
    pub customer_ip_address: String,

    /// 是否已担保或已付款
    pub is_guarantee_or_charged: Option<bool>,

    /// 确认类型
    pub confirmation_type: String,

    /// 给酒店备注
    pub note_to_hotel: Option<String>,

    /// 给艺龙备注
    pub note_to_elong: Option<String>,

    /// 是否需要发票
    pub is_need_invoice: Option<bool>,

    /// 客人信息
    pub order_rooms: Vec<OrderRoom>,

    /// 发票信息
    pub invoice: Option<Invoice>,

    /// 联系人信息
    pub contact: Contact,

    /// 信用卡信息
    pub credit_card: Option<CreditCard>,

    /// 第三方支付信息
    pub dove_corp_card: Option<DoveCorpCard>,

    /// 扩展字段
    pub extend_info: Option<ExtendInfo>,

    /// 是否仅创建订单
    pub is_create_order_only: Option<bool>,

    /// 销售给客人的最终价格
    pub customer_price: Option<f64>,

    /// 订单数据校验
    pub order_validation: Option<OrderValidation>,

    /// 促销信息（已废弃）
    pub coupon: Option<CouponInfo>,

    /// 马甲Id
    pub little_majia_id: Option<String>,

    /// 商品唯一标识
    pub goods_uniq_id: Option<String>,

    /// 特殊要求
    pub specific_remark: Option<String>,

    /// 儿童年龄
    pub child_ages: Option<Vec<i32>>,

    /// 成人数
    pub number_of_adults: Option<i32>,

    /// 酒店code
    pub hotel_code: Option<String>,

    /// 供应商id
    pub supplier_id: Option<String>,

    /// 二级供应商id
    pub sub_supplier_id: Option<String>,

    /// 商品库shopperid
    pub shopper_product_id: Option<String>,

    /// 用户信息加密选项
    pub encrypt_option: Option<i32>,

    /// 每日价
    pub day_price_list: Option<Vec<DayPrice>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Invoice {
    /// 发票类型
    /// v1.31新增。Paper-纸质发票，Electronic-电子发票，SpecialPaper-专用发票。
    pub invoice_type: String,

    /// 抬头类型
    /// Personally-个人，Enterprise-企业，Government-政府机关行政单位，默认为企业；
    /// Personally时，不需填写抬头Title和纳税人识别号/统一社会信用代码ITIN；
    /// Enterprise时，必须填写抬头Title和纳税人识别号/统一社会信用代码ITIN；
    /// Government时，必须填写抬头Title。
    pub title_type: String,

    /// 抬头
    pub title: String,

    /// 纳税人识别号/统一社会信用代码
    /// 字符长度是15、18或20位，是数字和字母的组合或纯数字。
    pub itin: String,

    /// 发票内容
    /// 代订房费或代订住宿费。
    pub item_name: String,

    /// 金额（人民币价格）
    pub amount: f64,

    /// 收件人
    /// 参考Recipient节点。
    pub recipient: Recipient,

    /// 是否添加发票备注
    /// true-在发票备注栏中添加酒店预订信息（酒店名称、入住日期、离店日期、房间数）；
    /// false-不添加，默认值。
    pub is_need_relation_order: Option<bool>,

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
    /// 接口中无规范定义，按照国家邮政规范填写即可。电子发票无需填写。
    pub province: String,

    /// 城市
    pub city: String,

    /// 行政区
    pub district: String,

    /// 街道
    pub street: String,

    /// 邮编
    pub postal_code: Option<String>,

    /// 收件人
    pub name: String,

    /// 电话
    pub phone: String,

    /// 邮箱
    /// 电子发票时该字段必须填写
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Contact {
    /// 姓名
    /// 在正式环境中姓名请不要填写“张三、李四、王五、test、测试”等，艺龙系统有对应姓名黑名单；
    /// 也不能填写“小姐”、“先生”或“女士”。
    pub name: String,

    /// 邮箱
    /// 下单时如果输入了邮箱，将会向对应邮箱发送预定成功等通知邮件。
    /// 国际请求时必传。
    pub email: Option<String>,

    /// 手机号区号
    pub mobile_area_code: String,

    /// 手机号
    /// 仅支持数字。如果区号为 86 时还会有中国手机号的校验。
    pub mobile: String,

    /// 电话
    /// 格式：国家代码-地区代码-电话号码-分机号，如 +86-010-58602288-6660
    pub phone: Option<String>,

    /// 传真
    pub fax: Option<String>,

    /// 性别
    /// Female 女，Male 男，Unknown 保密
    pub gender: String,

    /// 名（仅用于国际及港澳台酒店，大陆酒店无需关注，已废弃）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    /// 姓（仅用于国际及港澳台酒店，大陆酒店无需关注，已废弃）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreditCard {
    /// 卡号（加密后的值）
    /// 注意：字段对大小写敏感
    /// 加密方法：使用DES对称加密算法（CBC模式，key值和iv值一致）
    /// 加密内容 = 当前时间戳 + "#" + 卡号
    /// 加密密钥 = 账号appkey的后8位
    pub number: String,

    /// CVV（加密后的值）
    /// 加密方式同卡号
    pub cvv: String,

    /// 有效期-年
    pub expiration_year: i32,

    /// 有效期-月
    pub expiration_month: i32,

    /// 持卡人姓名（加密后的值）
    /// 中文姓名，不含空格
    pub holder_name: String,

    /// 证件类型
    /// 身份证 IdentityCard，护照 Passport，其他 Other
    pub id_type: String,

    /// 证件号码（加密后的值）
    /// 需加密，加密方式同卡号
    pub id_no: String,

    /// 手机号（加密后的值）
    /// 需加密，加密方式同卡号
    pub mobile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DoveCorpCard {
    /// 卡号（加密后的值）
    /// 注意：字段对大小写敏感
    /// 加密方法：使用DES对称加密算法（CBC模式，key值和iv值一致）
    /// 加密内容 = 当前时间戳 + "#" + 卡号
    /// 加密密钥 = 账号appkey的后8位
    pub number: String,

    /// 有效期，格式为 MMYY
    pub expiration_date: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ExtendInfo {
    /// 扩展字段，可以存放合作伙伴自己的一些信息
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
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderRoom {
    /// 入住人信息
    /// 每个房间支持传入多个入住人信息，该字段为数组。
    /// 转 XML 时需要自定义一个节点。
    pub customers: Vec<Customer>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Customer {
    /// 姓名
    /// 禁止使用“小姐”、“先生”、“女士”、名人姓名、污秽词语。
    /// 中文姓名不得出现任何汉字外的字符。
    /// 当客人输入拼音或英文姓名时，应给与明确的提示。
    pub name: String,

    /// 性别
    /// Female 女，Male 男，Unknown 保密
    pub gender: String,

    /// 国籍
    /// 填写具体的国籍，如中国、日本、美国、USA等
    pub nationality: String,

    /// 国籍代码
    /// 非中国大陆特有字段，建议传入以避免入住问题。
    pub nat: Option<String>,

    /// 证件号
    /// 按校验要求传对应身份信息，可加密。
    pub id_card_no: Option<String>,

    /// 证件类型
    /// IdentityCard 身份证，Passport 护照，HomeVisitingCertificate 回乡证，
    /// TaiwanCompatriotCertificate 台胞证，HongkongMacaoTaiwanResidencePermit 港澳台居民居住证
    pub id_card_type: Option<String>,

    /// 名（仅用于国际及港澳台酒店）
    pub first_name: Option<String>,

    /// 姓（仅用于国际及港澳台酒店）
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderValidation {
    /// 校验类型，逗号分隔的数字：
    /// 0: 不进行校验
    /// 1: 早餐数量校验
    /// 2: 取消时间校验
    /// 3: 担保金额校验
    /// 如果校验所有：Type=1,2,3
    pub r#type: String,
    /// 担保金额，预付金额或担保金额
    pub guarantee_amount: Option<f64>,
    /// 最晚取消时间，担保预付最晚取消时间
    pub cancel_time: Option<String>,
    /// 每日早餐数量列表，参考 DateBreakFast 节点
    pub date_break_fast_list: Option<Vec<DateBreakFast>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DateBreakFast {
    /// 日期，格式为 yyyy-MM-dd，例如 2024-09-14
    pub date: String,
    /// 包含的早餐份数
    pub break_fast_count: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CouponInfo {
    /// 优惠总金额
    pub amount: f64,
    /// 每日促销详情
    pub nightly_coupons: Option<Vec<NightlyCoupon>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct NightlyCoupon {
    /// 在店日期，格式为 yyyy-MM-dd，例如 2024-09-14
    pub date: String,
    /// 每间夜优惠详情
    pub promotion_room_nights: Vec<PromotionRoomNight>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PromotionRoomNight {
    /// 优惠金额
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DayPrice {
    /// 每日价格
    pub price: f64,
    /// 价格对应的日期，格式为 yyyy-MM-dd，例如 2024-09-14
    pub date: String,
    /// 税后价。国际必传，国内不允许传
    pub min_rate: Option<f64>,
}

impl BaseRequest for OrderCreateRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

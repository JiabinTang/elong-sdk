use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRateResponse {
    /// Rates 价格集合 Rate[] Y 包含多个Rate节点
    pub rates: Option<Vec<Rate>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Rate {
    /// HotelID 酒店ID String(8) N 这几个属性是业务主键
    #[serde(rename = "HotelID")]
    pub hotel_id: String,
    /// RoomTypeId 房型ID String(10) N
    pub room_type_id: String,
    /// RateplanId 产品 ID Int N
    #[serde(rename = "RateplanId")]
    pub rate_plan_id: i64,
    /// StartDate 开始时间 Date N
    pub start_date: String,
    /// EndDate 结束时间 Date N
    pub end_date: String,
    /// HotelCode 供应商ID String(8) Y v1.13新增
    pub hotel_code: Option<String>,
    /// Status 状态 Boolean N 已废弃，总是返回true，返回的都是有效价格，请注意清理本地无效价格
    pub status: bool,
    /// Member 平日卖价 Decimal N -1代表此房无价，无价和满房都不能进行预订
    pub member: Decimal,
    /// Weekend 周末卖价 Decimal N 同上
    pub weekend: Decimal,
    /// MemberCost 平日结算价 Decimal N 同上，开通了结算价模式的接入方才可以使用
    pub member_cost: Decimal,
    /// WeekendCost 周末结算价 Decimal N 同上，开通了结算价模式的接入方才可以使用
    pub weekend_cost: Decimal,
    /// AddBed 加床价 Decimal Y V1.01新增 -1代表不能加床，0-免费加床，大于0表示加床的费用
    pub add_bed: Option<Decimal>,
    /// PriceID 价格ID Long Y V1.08新增
    #[serde(rename = "PriceID")]
    pub price_id: Option<i64>,
    /// CurrencyCode 货币类型 String Y V1.08新增 参考Currency
    pub currency_code: Option<String>,
    /// InvoiceMode 发票模式 Enum Y V1.36新增 NoSense --全部 Elong --艺龙开票 Hotel --酒店开票
    pub invoice_mode: Option<String>,
    /// IsPriceLimit 是否限价 Boolean N 表示当前价格是否限价 ，限价时须按照艺龙给出的售价进行售卖。不同限价类型约束规则不同，详见下面PriceLimitedType字段，接入完成后通知商务变更，新校验规则生效。多天连住时，有一天为true，则所有天限价。 判断限价时与hotel.data.rp接口中IsPriceLimitProduct字段为或关系，两者有其一为true，均为限价。 false：非限价 true：限价
    pub is_price_limit: bool,
    /// PriceLimitedType 限价类型 Int N 二进制bit位分别表示各个限价条件，0为非限价，详见
    pub price_limited_type: i32,
}

impl BaseResponse for ElongResponse<DataRateResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<DataRateResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataRateRequest {
    /// HotelIds 酒店编号 String(100) N 最多10个,逗号分隔
    pub hotel_ids: String,
    /// HotelCodes 酒店编码 String Y 最多10个,逗号分隔。如果输入这个参数，请确保这些HotelCodes都是HotelIds(只能输入一个)所属的
    pub hotel_codes: Option<String>,
    /// PaymentType 付款类型 Enum N All - 全部 SelfPay - 前台现付 Prepay - 预付
    pub payment_type: String,
    /// StartDate 开始时间 DateTime N 大于等于昨天, 使用yyyy-MM-dd格式，例如:2022-12-09
    pub start_date: String,
    /// EndDate 结束时间 DateTime N 和开始时间不超过90天, 使用yyyy-MM-dd格式，例如:2022-12-09
    pub end_date: String,
    /// InvoiceMode 发票模式 Enum Y  NoSense --全部 Elong --艺龙开票 Hotel --酒店开票 注：该字段只过滤预付产品。
    pub invoice_mode: Option<String>,
}

impl BaseRequest for DataRateRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

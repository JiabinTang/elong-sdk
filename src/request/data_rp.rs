use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DataRpRequest {
    /// 酒店编号列表，最多10个，逗号分隔
    pub hotel_ids: String,
    /// 付款类型，枚举值：All（全部）、SelfPay（前台现付）、Prepay（预付）
    pub payment_type: Option<String>,
    /// 扩展参数，逗号分隔的数字列表：
    /// 1 - 返回RatePlan对应的可售卖渠道
    /// 2 - 返回结果中包含钟点房产品（1.32新增）
    pub options: Option<String>,
    /// 发票模式，枚举值：
    /// NoSense（全部）、Elong（艺龙开票）、Hotel（酒店开票）
    /// 注：该字段只过滤预付产品
    pub invoice_mode: Option<String>,
}

impl BaseRequest for DataRpRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

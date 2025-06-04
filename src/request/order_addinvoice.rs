use serde::{Deserialize, Serialize};

use crate::{elong::error::ElongError, request::api_request::BaseRequest};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OrderAddinvoiceRequest {
    /// 配送信息
    /// 参考 DeliveryAddress 节点
    pub delivery_info: DeliveryAddress,

    /// 订单号
    pub order_id: i64,

    /// 发票抬头
    /// 用户类型为企业、政府必填
    pub title: Option<String>,

    /// 用户类型
    /// 1-个人, 2-企业, 3-政府机关行政单位
    pub user_type: i32,

    /// 发票内容
    /// 必填，填写代订房费或代订住宿费
    pub item_name: String,

    /// 开票金额
    pub amount: f64,

    /// 发票类型
    /// 用来区分电子发票还是纸质发票，0：纸质，1：电子
    pub invoice_type: i32,

    /// 发票级别
    /// 用来区分专用发票还是普通发票，0：普通发票，1：专用发票
    pub invoice_level: i32,

    /// 税号
    /// 用户类型为企业必填
    pub itin: Option<String>,

    /// 是否关联订单
    /// 即在发票备注栏中添加酒店预订信息（酒店名称、入住日期、离店日期、房间数）
    /// 0-不关联, 1-关联
    pub need_relation_order: i32,

    /// 专用发票信息
    /// 专票必填，参考 DedicatedInvoice 节点
    pub dedicated_invoice: Option<DedicatedInvoice>,

    /// 用户信息加密选项
    /// 0：无需加密, 1：对称加密
    /// 传 1 时，需要对入参中的以下字段进行加密:
    /// - DeliveryAddress 节点
    /// - DedicatedInvoice 节点
    pub encrypt_option: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DedicatedInvoice {
    /// 纳税人识别号
    pub tax_payer_num: String,

    /// 开户银行
    pub tax_register_bank: String,

    /// 行政区
    pub register_bank_num: String,

    /// 注册地址
    pub shotel_address: String,

    /// 电话
    pub register_phone_num: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DeliveryAddress {
    /// 省，纸质发票必填
    pub province: Option<String>,

    /// 市，纸质发票必填
    pub city: Option<String>,

    /// 区，纸质发票必填
    pub district: Option<String>,

    /// 街道，纸质发票必填
    pub street: Option<String>,

    /// 收件人姓名，纸质发票必填
    pub recipient_name: Option<String>,

    /// 邮寄电子邮箱地址，电子发票必填
    pub post_email: Option<String>,

    /// 电话号码
    pub phone: String,

    /// 邮箱地址（已废弃）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl BaseRequest for OrderAddinvoiceRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

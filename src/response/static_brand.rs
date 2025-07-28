use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticBrandResponse {
    /// count 品牌总数 Int N 品牌总数
    pub count: i32,
    /// brands 品牌结果集 Brand[] Y 包含多个Brand节点
    pub brands: Option<Vec<Brand>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// 首字母小写的驼峰命名
#[serde(rename_all = "camelCase")]
pub struct Brand {
    /// brandId 品牌id Long N 品牌id
    pub brand_id: i64,
    /// groupId 集团id Long N 集团id
    pub group_id: i64,
    /// brandName 品牌名称 String N 品牌名称
    pub brand_name: String,
    /// brandNameEn 品牌英文名称 String Y
    pub brand_name_en: Option<String>,
    /// brandNameShou 品牌首字母 String Y
    pub brand_name_shou: Option<String>,
    /// brandNameDetail 品牌全称 String Y
    pub brand_name_detail: Option<String>,
    /// brandNameDetailEn 品牌英文全称 String Y
    pub brand_name_detail_en: Option<String>,
    /// pinYin 品牌拼音 String Y
    pub pin_yin: Option<String>,
    /// shouPinyin 品牌拼音首字母 String Y
    pub shou_pinyin: Option<String>,
    /// status 品牌状态 int N 0有效 1无效
    pub status: i32,
    /// optTime 操作时间 DateTime N
    pub opt_time: String,
}

impl BaseResponse for ElongResponse<StaticBrandResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<StaticBrandResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

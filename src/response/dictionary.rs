use serde::{Deserialize, Serialize};

use crate::{
    elong::error::ElongError,
    response::api_response::{BaseResponse, ElongResponse},
};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DictionaryResponse {
    /// 总共数量
    pub total: i32,
    /// 标签字典明细。当字典类型为酒店标签时返回
    pub tags: Option<Vec<Tag>>,
    /// 设施字典明细。当字典类型为酒店设施时返回
    pub facilities: Option<Vec<Facility>>,
    /// 主题字典明细。当字典类型为酒店主题时返回
    pub themes: Option<Vec<Theme>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    /// 标签ID
    pub tag_id: i32,
    /// 标签名称
    pub tag_name: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Facility {
    /// 设施ID
    pub facility_id: i32,
    /// 设施名称
    pub facility_name: String,
    /// 设施英文名称
    pub facility_name_en: Option<String>,
    /// 设施分类ID
    pub facility_type_id: i32,
    /// 设施分类名称
    pub facility_type_name: String,
    /// 设施分类英文名称
    pub facility_type_name_en: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Theme {
    /// 主题ID
    #[serde(rename = "PropertyID")]
    pub property_id: i32,
    /// 主题中文名称
    pub name_cn: String,
    /// 主题英文名称
    pub name_en: String,
}

impl BaseResponse for ElongResponse<DictionaryResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<DictionaryResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

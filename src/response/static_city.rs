use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticCityResponse {
    pub count: u32,
    pub citys: Vec<City>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct City {
    pub city_id: String,
    pub city_name: String,
    pub city_name_en: String,
    pub city_longitude: String,
    pub city_latitude: String,
    #[serde(rename = "CityParentID")]
    pub city_parent_id: String,
    pub province_id: String,
    pub province_name: String,
    pub province_name_en: String,
    pub country_id: String,
    pub country_name: String,
    pub country_name_en: String,
    pub country_code: String,
    ///Locations Location数据    Location[] Y   Location数据，包含行政区、商圈、标示物
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    /// LocationID LocationID String N
    #[serde(rename = "LocationID")]
    pub location_id: String,
    /// LocationName Location中文名称 String N
    pub location_name: String,
    /// LocationNameEn Location英文名称 String Y
    pub location_name_en: Option<String>,
    /// LocationType Location类型 Int N 1:行政区2:商圈3:标示物
    pub location_type: i32,
}

impl BaseResponse for ElongResponse<StaticCityResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<StaticCityRes> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticCityResponse {
    pub count: u32,
    pub citys: Vec<City>,
}

#[derive(Debug, Serialize, Deserialize)]
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
}

impl BaseResponse for ElongResponse<StaticCityResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<StaticCityRes> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

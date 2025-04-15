use serde::{Deserialize, Serialize};

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticListResponse {
    pub count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotels: Option<Vec<Hotel>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Hotel {
    pub hotel_id: String,
    pub hotel_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotel_name_en: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotel_status: Option<i32>,
    pub modification: String,
    pub update_time: String,
}

impl BaseResponse for ElongResponse<StaticListResponse> {
    fn from_json(json: String) -> Self {
        log::debug!("ElongResponse<StaticListRes> json: {}", json);
        serde_json::from_str(&json).unwrap_or_else(|err| {
            log::error!("Failed to parse ElongResponse<StaticListRes>: {}", err);
            ElongResponse {
                code: String::new(),
                result: StaticListResponse {
                    count: 0,
                    hotels: None,
                },
                guid: String::new(),
            }
        })
    }
}
use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StaticGradeRequest {
    ///  HotelId 酒店id String N 酒店id
    #[serde(rename = "HotelId")]
    pub hotel_id: String,
}

impl BaseRequest for StaticGradeRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

use serde::{Deserialize, Serialize};

pub trait BaseResponse {
    fn from_json(json: String) -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ElongResponse<T> {
    pub code: String,
    pub result: T,
    pub guid: String,
}

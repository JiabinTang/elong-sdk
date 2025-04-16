use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

pub trait BaseResponse: Sized {
    fn from_json(json: String) -> Result<Self, ElongError>;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ElongResponse<T> {
    pub code: String,
    pub result: T,
    pub guid: String,
}

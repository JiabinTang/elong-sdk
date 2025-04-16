use std::{fmt, io, string::FromUtf8Error};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ElongError {
    /// 网络请求错误
    HttpError(String),
    /// 序列化或反序列化错误
    SerdeError(String),
    /// 艺龙接口错误
    ApiError(String),
    /// IO 错误
    IoError(String),
    /// UTF-8 解码错误
    Utf8Error(String),
    /// 其他错误
    Other(String),
}

impl fmt::Display for ElongError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElongError::HttpError(msg) => write!(f, "HTTP Error: {}", msg),
            ElongError::SerdeError(msg) => write!(f, "Serialization Error: {}", msg),
            ElongError::ApiError(msg) => write!(f, "API Error: {}", msg),
            ElongError::IoError(msg) => write!(f, "IO Error: {}", msg),
            ElongError::Utf8Error(msg) => write!(f, "UTF-8 Error: {}", msg),
            ElongError::Other(msg) => write!(f, "Other Error: {}", msg),
        }
    }
}

impl std::error::Error for ElongError {}

impl From<reqwest::Error> for ElongError {
    fn from(err: reqwest::Error) -> Self {
        ElongError::HttpError(err.to_string())
    }
}

impl From<serde_json::Error> for ElongError {
    fn from(err: serde_json::Error) -> Self {
        ElongError::SerdeError(err.to_string())
    }
}

impl From<io::Error> for ElongError {
    fn from(err: io::Error) -> Self {
        ElongError::IoError(err.to_string())
    }
}

impl From<FromUtf8Error> for ElongError {
    fn from(err: FromUtf8Error) -> Self {
        ElongError::Utf8Error(err.to_string())
    }
}

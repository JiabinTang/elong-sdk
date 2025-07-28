use urlencoding::encode;

use crate::elong::error::ElongError;

pub struct ApiSignedRequest {
    /// 账户名
    pub user: String,
    /// 方法名
    pub method: String,
    /// 时间戳
    pub timestamp: String,
    /// 格式
    pub format: String,
    /// 数据
    pub data: String,
    /// 签名
    pub signature: String,
}

pub struct ApiRequestPayload {
    /// 版本号
    pub version: f64,
    /// 语言
    pub local: String,
    /// 请求参数
    pub request: String,
}

impl ApiSignedRequest {
    /// 获取签名
    pub fn new(
        user: String,
        app_key: String,
        app_secret: String,
        method: String,
        data: ApiRequestPayload,
    ) -> Self {
        let data = data.to_json();
        let format = "json".to_string();

        let timestamp = chrono::Local::now().timestamp().to_string();
        log::debug!("timestamp: {timestamp}");
        let data_app_key = format!("{data}{app_key}");
        log::debug!("data_app_key: {data_app_key}");
        let data_app_key_sig = format!("{:x}", md5::compute(data_app_key));
        log::debug!("data_app_key_signature: {data_app_key_sig}");
        let sig_str = format!("{timestamp}{data_app_key_sig}{app_secret}");
        log::debug!("sig_str: {sig_str}");
        let signature = format!("{:x}", md5::compute(sig_str));
        log::debug!("signature: {signature}");

        ApiSignedRequest {
            user,
            method,
            timestamp,
            format,
            data,
            signature,
        }
    }

    pub fn to_params(&self) -> String {
        format!(
            "timestamp={}&format={}&method={}&signature={}&user={}&data={}",
            self.timestamp,
            self.format,
            self.method,
            self.signature,
            self.user,
            encode(&self.data)
        )
    }
}

impl ApiRequestPayload {
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"Version":"{}","Local":"{}","Request":{}}}"#,
            self.version, self.local, self.request
        )
    }
}

pub trait BaseRequest {
    fn to_json(&self) -> Result<String, ElongError>;
}

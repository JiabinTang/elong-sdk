use crate::{
    elong::error::ElongError,
    network::http_client::HttpClient,
    request::api_request::{ApiRequestPayload, ApiSignedRequest, BaseRequest},
    response::api_response::BaseResponse,
};

use super::endpoints::ApiMethod;

#[derive(Clone)]
pub struct ElongClient {
    /// HTTP 请求客户端
    client: HttpClient,
    /// 账户名
    username: String,
    /// appKey
    app_key: String,
    /// appSecret
    app_secret: String,
}

impl ElongClient {
    pub fn new(username: String, app_key: String, app_secret: String) -> Self {
        let client = HttpClient::new();
        ElongClient {
            client,
            username,
            app_key,
            app_secret,
        }
    }

    /// 获取数据
    pub async fn fetch_data<T, U>(
        &self,
        url: &str,
        method: ApiMethod,
        request: T,
    ) -> Result<U, ElongError>
    where
        T: BaseRequest,
        U: BaseResponse,
    {
        let data = ApiRequestPayload {
            version: 1.62,
            local: "zh-CN".to_string(),
            request: request.to_json(),
        };

        let params = ApiSignedRequest::new(
            self.username.clone(),
            self.app_key.clone(),
            self.app_secret.clone(),
            method.name().to_owned(),
            data,
        )
        .to_params();

        let url = format!("{}?{}", url, params);
        log::debug!("url: {}", url);

        let response = self.client.get(&url).await?;

        let result = U::from_json(response);

        Ok(result)
    }
}

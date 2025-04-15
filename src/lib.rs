use async_trait::async_trait;
use elong::error::ElongError;
use request::static_city::*;
use response::{api_response::ElongResponse, static_city::*};

pub mod elong;
mod network;
pub mod request;
pub mod response;

#[async_trait]
pub trait Elong {
    async fn get_static_city(
        &self,
        req: StaticCityReq,
    ) -> Result<ElongResponse<StaticCityRes>, ElongError>;
}

// impl Elong {
//     pub fn new(username: String, app_key: String, app_secret: String) -> Self {
//         let elong_client = ElongClient::new(username.clone(), app_key.clone(), app_secret.clone());
//         Elong { elong_client }
//     }

//     pub async fn get_static_city(&self) -> Result<StaticCityRes, ElongError> {
//         let url = "https://api.elong.com/rest";
//         let method = "hotel.static.city";
//         let request = StaticCityReq {
//             country_type: 0,
//             city_id_type: 0,
//             is_need_location: false,
//             page_size: 10,
//             page_index: 1,
//         };
//         let res: StaticCityRes = self.elong_client.fetch_data(url, method, request).await?;
//         Ok(res)
//     }

//     /// 获取静态城市测试
//     pub async fn get_static_city_test(&self) -> Result<StaticCityRes, ElongError> {
//         let url = "https://api.test.lohoo.com/rest";
//         let method = "hotel.static.city";
//         let request = StaticCityReq {
//             country_type: 0,
//             city_id_type: 0,
//             is_need_location: false,
//             page_size: 10,
//             page_index: 1,
//         };
//         let res: StaticCityRes = self.elong_client.fetch_data(url, method, request).await?;
//         Ok(res)
//     }
// }

use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_request::BaseRequest;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StaticCityRequest {
    /// CountryType     国家类型        Int Y   默认0：所有城市、 1：国内 、2：国际
    pub country_type: Option<i32>,
    /// CityIdType      城市Id类型      Int Y   默认0：所有Id，即搜索id+内容id;1:只返回内容Id 注：搜索id、内容Id均支持hotel.list这样的动态接口搜索某个城市下的酒店；搜索Id不支持hotel.static.list静态接口搜索酒店，只有内容id支持hotel.static.list搜索酒店。以上只针对国内城市有效，国际可不传。
    pub city_id_type: Option<i32>,
    /// IsNeedLocation  是否Location    Boolean 默认：false，Location数据包括，行政区、商圈、标示物
    pub is_need_location: Option<bool>,
    /// PageSize        每页数据量      Int Y   默认：200，范围1～200，每页抓去的数据量
    pub page_size: Option<i32>,
    /// PageIndex       页码            Int Y   从1开始
    pub page_index: Option<i32>,
}

impl BaseRequest for StaticCityRequest {
    fn to_json(&self) -> Result<String, ElongError> {
        Ok(serde_json::to_string(self)?)
    }
}

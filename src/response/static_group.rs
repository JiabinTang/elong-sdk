use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticGroupResponse {
    /// count 品牌总数 Int N 品牌总数
    pub count: i32,
    /// brands 品牌结果集 Brand[] Y 包含多个Brand节点
    pub groups: Option<Vec<Group>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// 首字母小写的驼峰命名
#[serde(rename_all = "camelCase")]
pub struct Group {
    /// groupId 集团id Long N 集团id
    pub group_id: i64,
    /// groupName   集团中文简称 String N 集团中文简称
    pub group_name: String,
    /// groupNameDetail 集团中文全称 String N 集团中文全称
    pub group_name_detail: String,
    /// groupNameEn 集团英文简称    String Y
    pub group_name_en: Option<String>,
    /// groupNameDetailEn   集团英文全称    String Y
    pub group_name_detail_en: Option<String>,
    /// groupStatus 集团状态 int N 0有效 1无效
    pub group_status: i32,
    /// optTime 操作时间 DateTime N
    pub opt_time: String,
}

impl BaseResponse for ElongResponse<StaticGroupResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<StaticGroupResponse> json: {json}");
        Ok(serde_json::from_str(&json)?)
    }
}

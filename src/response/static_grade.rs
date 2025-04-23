use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticGradeResponse {
    /// Review 点评 Review Y 酒店点评数据，参考Review节点
    pub review: Option<Review>,
    /// Grade 评分 Grade Y 酒店评分数据，参考Grade节点 已经废弃不准确
    pub grade: Option<Grade>,
    /// CommentTags 点评标签 CommentTag[] Y 酒店点评标签，参考CommentTag节点
    pub comment_tags: Option<Vec<CommentTag>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Review {
     /// ReviewCount 自有点评总数 Int N
     pub review_count: i32,
     /// ReviewGoodsCount 自有点评好评数 Int N
     pub review_goods_count: i32,
     /// ReviewPoorCount 自有差评数 Int N
     pub review_poor_count: i32,
     /// ReviewScore 自有评分 String N 已经计算为百分比。周边酒店的评价评分可以自行通过酒店经纬度计算周边酒店，再计算平均评分
     pub review_score: String,
     /// TripartiteReviewCount 第三方点评总数 Int N
     pub tripartite_review_count: i32,
     /// ReviewCommonScore 总评分 Double N
     pub review_common_score: f64,
     /// ReviewPositionScore 位置分 Double N
     pub review_position_score: f64,
     /// ReviewFacilityScore 设施分 Double N
     pub review_facility_score: f64,
     /// ReviewServiceScore 服务分 Double N
     pub review_service_score: f64,
     /// ReviewSanitationScore 卫生分 Double N
     pub review_sanitation_score: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Grade {
    /// ConfirmRateOf30Minute 订单30分钟内确认率 Int Y
    pub confirm_rate_of_30_minute: Option<i32>,
    /// GradeOfOrderCountB90 分销过去90天产量得分 Int Y
    pub grade_of_order_count_b90: Option<i32>,
    /// GradeOfOrderCountC90 C端过去90天产量得分 Int Y
    pub grade_of_order_count_c90: Option<i32>,
    /// GradeOfProd30 产品过去30天可卖得分 Int Y
    pub grade_of_prod_30: Option<i32>,
    /// ValRateOf90 分销过去90天可定成功率 Int Y
    pub val_rate_of_90: Option<i32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommentTag {
    /// TagName 标签名称 String N 
    pub tag_name:String,
    /// HeatValue 标签热度 Int N 
    pub heat_value: i32,
}
impl BaseResponse for ElongResponse<StaticGradeResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<StaticGradeResponse> json: {}", json);
        print!("ElongResponse<StaticGradeResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

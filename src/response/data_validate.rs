use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DataValidateResponse {
    /// ResultCode 验证结果 Enum N OK: 正常可预订 Product: 产品无效或关房 Inventory: 房量不够 Rate: 价格不符
    pub result_code: String,
    /// ErrorMessage 具体结果信息 String(255) Y V1.02新增
    pub error_message: Option<String>,
    /// GuaranteeRate 担保金额 Decimal Y 如果是担保订单才有这个值
    pub guarantee_rate: Option<f64>,
    /// CurrencyCode 担保金额的货币类型 Enum Y 参考Currency
    pub currency_code: Option<String>,
    /// CancelTime 最晚取消时间 DateTime Y 担保订单可取消的时间，如果返回的时间小于当前时间，则代表此订单不可变更取消
    pub cancel_time: Option<String>,
    /// FreeCancelTime 免费取消时间 DateTime Y V1.33新增
    pub free_cancel_time: Option<String>,
    /// PenaltyAmount 罚金金额 Decimal Y V1.33新增货币类型为人民币，只代表取消时间处于免费取消时间跟最晚取消时间之间产生的罚金金额，早于免费取消时间不收罚金，晚于最晚取消100%罚金
    pub penalty_amount: Option<f64>,
    /// interValidateInfo 国际验证详情 Element N 参考interValidateInfo节点，仅用于国际及港澳台酒店，大陆酒店无需关注
    pub inter_validate_info: Option<Vec<InterValidateInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InterValidateInfo {
    /// ratePlanInfo 政策列表信息 Element Y 参考RatePlanInfo节点，仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub rate_plan_info: Option<RatePlanInfo>,
    /// orderHotel 酒店信息 Element Y 参考OrderHotel节点，仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub order_hotel: Option<OrderHotel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RatePlanInfo {
    /// ConfirmType 确认类型 Int N 确认类型，0延迟确认，1立即确认；仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub confirm_type: i32,
    /// CancelName 取消名称 String Y 取消名称：限时取消、不可取消、随时取消；仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub cancel_name: Option<String>,
    /// CancelDescription 取消政策说明文案 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub cancel_description: Option<String>,
    /// RateNightlyRateList 每晚房价 RoomRateNightlyRate[] Y 参考RoomRateNightlyRate节点，仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub rate_nightly_rate_list: Option<Vec<RoomRateNightlyRate>>,
    /// CancelPolicyList 取消政策集合 CancelPolicy[] Y 参考CancelPolicy节点，仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub cancel_policy_list: Option<Vec<CancelPolicy>>,
    /// OccupancyPerRoom 房间最大入住人数 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub occupancy_per_room: Option<i32>,
    /// AdultPerRoom 成人数 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub adult_per_room: Option<i32>,
    /// ChildPerRoom 儿童数 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub child_per_room: Option<i32>,
    /// RoomChildAge 儿童年龄 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub room_child_age: Option<i32>,
    /// RoomDescription 房间描述 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub room_description: Option<String>,
    /// CheckInInstructions 入住需知 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub check_in_instructions: Option<String>,
    /// SmokingPreferences 客房的可用吸烟偏好 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub smoking_preferences: Option<String>,
    /// RestInventoryCount 剩余房间数 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub rest_inventory_count: Option<String>,
    /// ExtraPersonFee 额外人员费用 Decimal Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub extra_person_fee: Option<f64>,
    /// ExtraPersonFeeRMB 额外人员费用(人民币) Decimal Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub extra_person_fee_rmb: Option<f64>,
    /// BedGroups 床型信息 BedGroup[] Y 参考BedGroup节点，仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub bed_groups: Option<Vec<BedGroup>>,
    /// AdditionalTax 另付税和服务费 AdditionalTax Y 参考AdditionalTax节点，仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub additional_tax: Option<AdditionalTax>,
    /// boardInfo 餐 Board Y 仅用于国际及港澳台酒店，大陆酒店无需关注。注意兼容首字母的大小写。
    pub board_info: Option<Board>,
    /// Nat 国籍限制 Nat Y 为空时，不一定代表一定不限制国籍，也可能是供应商无能力提供标准限制。如果当前字段不为空，代表产品一定有限制，见Nat节点。
    pub nat: Option<Nat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OrderHotel {
    /// HotelId 物理酒店ID String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub hotel_id: String,
    /// Name 酒店名称 String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub name: String,
    /// HotelEnglishName 酒店英文名称 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub hotel_english_name: Option<String>,
    /// Address 地址 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub address: Option<String>,
    /// AddressEn 英文地址 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub address_en: Option<String>,
    /// Phone 手机号 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub phone: Option<String>,
    /// CityName 城市名称 String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub city_name: String,
    /// HotelCountryName 国家名称 String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub hotel_country_name: String,
    /// HotelCountryId 国家ID String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub hotel_country_id: String,
    /// HotelLatitude 纬度 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub hotel_latitude: Option<String>,
    /// HotelLongitude 经度 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub hotel_longitude: Option<String>,
    /// CityId 城市ID String N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub city_id: String,
    /// Star 星级 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub star: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoomRateNightlyRate {
    /// Rate 每晚每间房价(含税费) Decimal N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub rate: f64,
    /// MinRate 最小价(不含税费) Decimal N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub min_rate: f64,
    /// Date 日期 Date N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CancelPolicy {
    /// Penalty 罚金 Decimal N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub penalty: f64,
    /// PenaltyRMB 罚金(人民币) Decimal N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub penalty_rmb: f64,
    /// DateFrom 取消开始时间 Date N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub date_from: String,
    /// DateTo 取消结束时间 Date N 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub date_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BedGroup {
    /// BedGroupId 床型信息id String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub bed_group_id: Option<String>,
    /// BedGroupDesc 床型信息描述 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub bed_group_desc: Option<String>,
    /// BedTypes 床类型集合 BedType[] Y 参考BedType节点，仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub bed_types: Option<Vec<BedType>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BedType {
    /// BedTypeId 床类型id String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub bed_type_id: Option<String>,
    /// BedTypeName 床类型名称 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub bed_type_name: Option<String>,
    /// BedType 床类型 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub bed_type: Option<String>,
    /// Count 床数 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub count: Option<i32>,
    /// Size 床大小 String Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub size: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalTax {
    /// TotalAmountRmb 另付税和服务费总额(人民币) Decimal Y
    pub total_amount_rmb: Option<f64>,
    /// TotalAmountPaid 另付税和服务费总额(支付币) Decimal Y
    pub total_amount_paid: Option<f64>,
    /// AdditionalTaxItems 另付税和服务费明细 AdditionalTaxItem[] Y 参考AdditionalTaxItem节点
    pub additional_tax_items: Option<Vec<AdditionalTaxItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AdditionalTaxItem {
       /// Description 另付税和服务费明细描述 String Y
       pub description: Option<String>,
       /// Amount 另付税和服务费明细金额 Decimal Y
       pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Board {
    /// IsBreakfastIncluded 是否含早 Boolean Y 仅用于国际及港澳台酒店，大陆酒店无需关注。注意兼容首字母的大小写
    pub is_breakfast_included: Option<bool>,
    /// HalfBoardIncluded 是否半膳 Boolean Y 仅用于国际及港澳台酒店，大陆酒店无需关注。注意兼容首字母的大小写
    pub half_board_included: Option<bool>,
    /// FullBoardIncluded 是否全膳 Boolean Y 仅用于国际及港澳台酒店，大陆酒店无需关注。注意兼容首字母的大小写
    pub full_board_included: Option<bool>,
    /// BoardDetails 膳食明细 Element Y 仅用于国际及港澳台酒店，大陆酒店无需关注。注意兼容首字母的大小写
    pub board_details: Option<BoardDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BoardDetail {
    /// Count 膳食数量 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。
    pub count: Option<i32>,
    /// Type 膳食类型 Int Y 仅用于国际及港澳台酒店，大陆酒店无需关注。注意兼容首字母的大小写
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Nat {
    /// Type 类型 Int N 1:不适用；2:适用
    pub r#type: i32,
    /// List 国籍Code列表 String[] N 例如：["CN", "GB"]
    pub list: Vec<String>,
}

impl BaseResponse for ElongResponse<DataValidateResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<DataValidateResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

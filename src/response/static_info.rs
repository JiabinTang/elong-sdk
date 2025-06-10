use serde::{Deserialize, Serialize};

use crate::elong::error::ElongError;

use super::api_response::{BaseResponse, ElongResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StaticInfoResponse {
    /// Detail 详情 Detail N 酒店详情数据，参考Detail节点
    pub detail: Option<Hotel>,
    /// Suppliers 供应商列表 Supplier[] Y 酒店供应商列表，参考Supplier节点
    pub suppliers: Option<Vec<Supplier>>,
    /// Rooms 房间列表 Room[] Y 酒店房间列表，参考Room节点
    pub rooms: Option<Vec<Room>>,
    /// Images 图片列表 Image[] Y 酒店图片列表，参考Image节点
    pub images: Option<Vec<Image>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Hotel {
    /// 酒店ID
    pub hotel_id: String,
    /// 酒店名称
    pub hotel_name: String,
    /// 酒店名称英文
    pub hotel_name_en: Option<String>,
    /// 酒店状态 0：正常 1:无效 2：删除
    pub hotel_status: Option<i32>,
    /// 酒店中文曾用名
    pub hotel_used_name: Option<String>,
    /// 酒店英文曾用名
    pub hotel_used_name_en: Option<String>,
    /// 酒店当地中文名
    pub hotel_name_local: Option<String>,
    /// 酒店当地英文名
    pub hotel_name_local_en: Option<String>,
    /// 酒店中文简称
    pub short_name: Option<String>,
    /// 酒店英文简称
    pub short_name_en: Option<String>,
    /// 酒店中文地址
    pub address: Option<String>,
    /// 酒店英文地址
    pub address_en: Option<String>,
    /// 邮编
    pub postal_code: Option<String>,
    /// 挂牌星级
    pub star_rate: Option<i32>,
    /// 艺龙推荐星级
    pub category: Option<i32>,
    /// 电话
    pub phone: Option<String>,
    /// 传真
    pub fax: Option<String>,
    /// 酒店邮箱地址
    pub email: Option<String>,
    /// 酒店时区
    pub timezone: Option<String>,
    /// 营业执照
    pub licenses: Option<Vec<String>>,
    /// 开业时间
    pub establishment_date: Option<String>,
    /// 装修时间
    pub renovation_date: Option<String>,
    /// 集团编号
    pub group_id: Option<String>,
    /// 集团中文名称
    pub group_name: Option<String>,
    /// 集团英文名称
    pub group_name_en: Option<String>,
    /// 品牌编号
    pub brand_id: Option<String>,
    /// 品牌中文名称
    pub brand_name: Option<String>,
    /// 品牌英文名称
    pub brand_name_en: Option<String>,
    /// 是否经济型 默认值为0，1代表是经济型酒店
    pub is_economic: Option<i32>,
    /// 是否是公寓 默认值为0，1代表是酒店式公寓
    pub is_apartment: Option<i32>,
    /// 最早入住时间    酒店入住的最早最晚时间，例如ArrivalTime为14:00表示酒店在入住日当天14:00之后允许客人入住，LatestArrivalTime为03:00，LatestArrivalTimeIsNextDay为true，则表示最晚的入住时间应该在入住日次日凌晨3:00之前，LatestArrivalTimeIsNextDay为false则最晚入住时间的日期入住日当天。
    /// 注意：hotel.data.validate，hotel.data.validate以及hotel.order.create接口入参EarliestArrivalTime以及LatestArrivalTime需要在ArrivalTime和LatestArrivalTime之间
    pub arrival_time: Option<String>,
    /// 最晚入住时间
    pub latest_arrival_time: Option<String>,
    /// 最晚入住时间是否次日
    pub latest_arrival_time_is_next_day: Option<bool>,
    /// 离店时间 酒店离店的最晚时间，例如12:00表示客人必须在离店日当天12:00之前离店
    pub departure_time: Option<String>,
    /// Google纬度
    pub google_lat: Option<f64>,
    /// Google经度
    pub google_lon: Option<f64>,
    /// Baidu纬度
    pub baidu_lat: Option<f64>,
    /// Baidu经度
    pub baidu_lon: Option<f64>,
    /// 国家Id
    pub country_id: Option<String>,
    /// 国家中文名
    pub country_name: Option<String>,
    /// 国家英文名
    pub country_name_en: Option<String>,
    /// 主城市Id
    pub city_id: Option<String>,
    /// 城市中文名
    pub city_name: Option<String>,
    /// 城市英文名
    pub city_name_en: Option<String>,
    /// 关联城市
    pub city_id2: Option<String>,
    /// 行政区Id
    pub district: Option<String>,
    /// 行政区中文名称
    pub district_name: Option<String>,
    /// 行政区英文名称
    pub district_name_en: Option<String>,
    /// 主商圈Id
    pub business_zone: Option<String>,
    /// 主商圈中文名称
    pub business_zone_name: Option<String>,
    /// 主商圈英文名称
    pub business_zone_name_en: Option<String>,
    /// 附属商圈Id
    pub business_zone2: Option<String>,
    /// 附属商圈中文名
    pub business_zone2_name: Option<String>,
    /// 附属商圈英文名
    pub business_zone2_name_en: Option<String>,
    /// 酒店支持的信用卡
    pub credit_cards: Option<String>,
    /// 酒店支持的信用卡英文
    pub credit_cards_en: Option<String>,
    /// 酒店中文简介
    pub intro_editor: Option<String>,
    /// 英文简介
    pub intro_editor_en: Option<String>,
    /// 中文描述
    pub description: Option<String>,
    /// 英文描述
    pub description_en: Option<String>,
    /// 中文接机服务
    pub airport_pick_up_service: Option<String>,
    /// 英文接机服务
    pub airport_pick_up_service_en: Option<String>,
    ///GeneralFacilities酒店基础施列表 Facility[] Y
    pub general_facilities: Option<Vec<Facility>>,
    ///RecreationFacilities酒店休闲设施列表 Facility[] Y
    pub recreation_facilities: Option<Vec<Facility>>,
    ///ServiceFacilities酒店服务设施列表 Facility[] Y
    pub service_facilities: Option<Vec<Facility>>,
    ///BookingNoticeFacilities预订须知列表 Facility[] Y 新增
    pub booking_notice_facilities: Option<Vec<Facility>>,
    ///HasCoupon 是否允许返现 Boolean Y 不允许返现的酒店请在各个渠道上面都不要提供返现或类似返现的活动，否则酒店会对此做出处罚。
    pub has_coupon: Option<bool>,
    ///Themes 酒店主题列表 Theme[]  参考Theme节点（已废弃；替代节点：HotelTags）
    pub themes: Option<Vec<Theme>>,
    ///RoomTotalAmount 客房总数量 Int Y
    pub room_total_amount: Option<i32>,
    ///HotelTypes 酒店类型 HotelType[] Y 酒店类型Ids：679(酒店),680(公寓),681(客栈\民宿),682(农家乐),1303(特色住宿),1304(青年旅舍),1305(别墅),1306(客栈),1403(连锁酒店),1404(休闲度假),1405(住宿加早餐旅馆),1406(旅舍),1407(汽车旅馆),1408(特色小屋),1409(排屋),1410(日式旅店),1411(摩洛哥特色旅馆),1412(Safari/帐篷屋),1413(船屋),1414(游轮),1415(城堡),1416(宫殿),1417(树屋)等，参考HotelType节点
    pub hotel_types: Option<Vec<HotelType>>,
    ///ServiceRank 酒店服务指数 ServiceRank Y 参考ServiceRank节点
    pub service_rank: Option<ServiceRank>,
    ///ParkInfos 停车信息 ParkInfo[] Y 新增,参考ParkInfo节点
    pub park_infos: Option<Vec<ParkInfo>>,
    ///TelList 电话信息 Tel[] Y 新增,参考Tel节点
    pub tel_list: Option<Vec<Tel>>,
    ///PetPolicy 宠物政策 String Y V1.62新增
    pub pet_policy: Option<String>,
    ///Notices 新预定政策 Notice[] Y V1.62新增,参考Notice节点
    pub notices: Option<Vec<Notice>>,
    ///DepositPolicy 押金政策 DepositPolicy Y V1.62新增,参考DepositPolicy节点
    pub deposit_policy: Option<DepositPolicy>,
    ///CheckinPolicies 入住方式 CheckinPolicy[] Y V1.62新增,参考CheckinPolicy节点
    pub checkin_policies: Option<Vec<CheckinPolicy>>,
    ///StayPolicy 住宿规定 StayPolicy Y V1.62新增,参考StayPolicy节点
    pub stay_policy: Option<StayPolicy>,
    ///FacilityV2 新V2设施 FacilityType[] Y V1.62新增,参考FacilityType节点
    pub facility_v2: Option<Vec<FacilityType>>,
    ///HotelCloseTimes 临时歇业 HotelCloseTime[] Y V1.62新增,参考HotelCloseTime节点
    pub hotel_close_times: Option<Vec<HotelCloseTime>>,
    ///HotelTags 酒店标签 Integer[] Y 酒店标签Id
    pub hotel_tags: Option<Vec<i32>>,
    ///ChildPolicy 儿童政策 ChildPolicy Y V1.62新增,参考ChildPolicy节点儿童及加床政策请参考您所选择的客房政策，若超过房型限定人数，可能需要收取额外费用。提出的任何请求均需要获得酒店的确认，所有服务详情以酒店告知为准。
    pub child_policy: Option<ChildPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Facility {
    ///FacilityId  设施ID String N
    pub facility_id: i32,
    ///FacilityName    设施中文名称 String Y
    pub facility_name: Option<String>,
    ///FacilityNameEn  设施英文名称 String Y
    pub facility_name_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Theme {
    ///ThemeId  主题ID String Y
    pub theme_id: i32,
    ///ThemeName  主题名称 String Y
    pub theme_name: Option<String>,
    ///ThemeNameEn  主题英文名称 String Y
    pub theme_name_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HotelType {
    ///HotelTypeId  酒店类型ID String Y
    pub hotel_type_id: i32,
    ///HotelTypeName  酒店类型名称 String Y
    pub hotel_type_name: Option<String>,
    ///HotelTypeNameEn  酒店类型英文名称 String Y
    pub hotel_type_name_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceRank {
    /// SummaryScore 酒店服务总评分
    pub summary_score: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParkInfo {
    ///title 名称 String(50) Y
    pub title: Option<String>,
    ///desc 描述 String[] Y
    pub desc: Option<Vec<String>>,
    ///type 类型 String(50) Y
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tel {
    /// nationCode 国家码 String(30) Y
    pub nation_code: Option<String>,
    /// areaCode 区域码 String(50) Y
    pub area_code: Option<String>,
    /// mainCode 主机号 String(100) N
    pub main_code: Option<String>,
    /// extCode 分机号 String(50) Y
    pub ext_code: Option<String>,
    /// type 类型 String(10) N 1:座机 2:手机号
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Notice {
    ///Category 分类 String(30) N Hotel:酒店维度 City:城市维度
    pub category: String,
    ///Start 开始日期 Date Y
    pub start: Option<String>,
    ///End 结束日期 Date Y
    pub end: Option<String>,
    ///Text 中文文本 String N
    pub text: String,
    ///TextEn 英文文本 String Y
    pub text_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DepositPolicy {
    ///DepositSwitch 是否收取押金 Integer N 1:是, 0:否, null:未知
    pub deposit_switch: Option<i32>,
    ///Frequency 收取频次 Integer Y 1:固定金额，2:每间，3:每晚
    pub frequency: Option<i32>,
    ///Amount 收取金额 f64 Y
    pub amount: Option<f64>,
    ///PayType 押金支付方式 Integer[] Y 1:现金，2:信用卡，3:借记卡，4:第三方平台
    pub pay_type: Option<Vec<i32>>,
    ///RefundType 押金退还方式 Integer[] Y 0:不原路退还，1:原路退还
    pub refund_type: Option<Vec<i32>>,
    ///RefundTime 押金退还时间 Integer Y 0:当日退还，1:一周内退还，2:两周内退还
    pub refund_time: Option<i32>,
    ///Currency 押金币种 String N 默认RMB
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheckinPolicy {
    /// CheckInWay 入住方式 String Y frontdesk:请到前台领取钥匙/门卡 reception:住宿方会有专人等侯迎接    password:住宿方会提供住宿的进门密码 keybox:住宿方会将钥匙存放于隐蔽处，并会在你入住前提供详细说明   keyhide:住宿方会将钥匙存在保管箱内，并会在你入住前提供详细说明  instruction:住宿方会在你入住前提供详细说明  contactus:【注】本酒店/民宿务必提前联系，确认入住事宜，否则可能影响入住     other:其他
    pub check_in_way: Option<String>,
    /// CheckInAddress 入住地址 String Y
    pub check_in_address: Option<String>,
    /// CheckInAddressEn 入住地址英文 String Y
    pub check_in_address_en: Option<String>,
    /// CheckInNote 入住方式描述 String Y
    pub check_in_note: Option<String>,
    /// CheckInNoteEn 入住方式描述英文 String Y
    pub check_in_note_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StayPolicy {
    ///QuietTime 安静时间 String[] Y
    pub quiet_time: Option<Vec<String>>,
    ///PartyAllowed 是否允许派对 String Y
    pub party_allowed: Option<String>,
    ///PhotoAllowed 是否允许拍照 String Y
    pub photo_allowed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoomBed {
    /// RoomBedInfo 床型信息 RoomBedInfo[] Y 参见RoomBedInfo节点
    pub room_bed_info: Option<Vec<RoomBedInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoomBedInfo {
    /// RoomDetailBeds 卧室床信息 RoomBedDetail[] Y 参见RoomBedDetail节点
    pub room_detail_beds: Option<Vec<RoomBedDetail>>,
    /// LivingRoomBeds 客厅床信息 RoomBedDetail[] Y 参见RoomBedDetail节点
    pub living_room_beds: Option<Vec<RoomBedDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RoomBedDetail {
    /// BedGroups 或分组 BedGroup[] Y BedGroups链表的每个元素之间为“或”的关系 参见BedGroup节点
    pub bed_groups: Option<Vec<BedGroup>>,
    /// RoomIndex 房间号 Integer N
    pub room_index: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BedGroup {
    /// BedInfos 和分组 BedInfo[] Y BedInfos链表的每个元素之间为“和”的关系 参见BedInfo节点
    pub bed_infos: Option<Vec<BedInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BedInfo {
    /// BedTypeId 床型id int N
    pub bed_type_id: i32,
    /// BedWidth 床宽，单位：m double N
    pub bed_width: Option<f64>,
    /// BedCount 床的数量 int N
    pub bed_count: i32,
    /// BedName 床的名称 String Y
    pub bed_name: Option<String>,
    /// Remark 备注 String Y
    pub remark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FacilityType {
    /// FacilityTypeId 设施分类Id Long N
    pub facility_type_id: i32,
    /// FacilityTypeName 设施分类名称 String N
    pub facility_type_name: String,
    /// FacilityInfoList 设施信息       FacilityInfo[] Y
    pub facility_info_list: Option<Vec<FacilityInfo>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FacilityInfo {
    ///FacilityId 设施Id Long N
    pub facility_id: i32,
    ///FacilityName 设施名称 String N
    pub facility_name: String,
    ///FeeInfo 设施收费信息     FeeInfo Y
    pub fee_info: Option<FeeInfo>,
    ///BusinessHourInfos 设施营业时间 BusinessHourInfo[] Y
    pub business_hour_infos: Option<Vec<BusinessHourInfo>>,
    ///AgeLimitInfo 设施年龄限制信息    AgeLimitInfo Y
    pub age_limit_info: Option<AgeLimitInfo>,
    ///ReservationInfo 设施预约信息 ReservationInfo Y
    pub reservation_info: Option<ReservationInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FeeInfo {
    ///FeeChargeType 设施费用类型 Enum Y Paid：收费；Free：免费；None：未知
    pub fee_charge_type: Option<String>,
    ///FeeDetail 设施费用明细 FeeDetail[] Y
    pub fee_detail: Option<Vec<FeeDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FeeDetail {
    ///Amount 设施费用金额 f64 Y
    pub amount: Option<f64>,
    ///Currency 设施费用币种 String Y
    pub currency: Option<String>,
    ///FeeTimeType 设施费用类型    Enum Y Time：次数；Minute：分钟；Quarter：一刻钟；HalfHour：半小时；Hour：小时；Day：天；Week：周；Person：人；Bed：床；Car：车；Bottle：瓶
    pub fee_time_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessHourInfo {
    ///OpenDayType 设施营业时间类型 Enum    Y OpenDay：开放时间；CloseDay:关闭时间
    pub open_day_type: Option<String>,
    ///StartTime 设施营业时间开始时间 String Y HH:MM
    pub start_time: Option<String>,
    ///EndTime 设施营业时间结束时间 String Y HH:MM
    pub end_time: Option<String>,
    ///WeeklyIndex 设施营业时间周有效  String Y 适用星期几，从周一到周日 （如1110110表示周四、周日无效）
    pub weekly_index: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AgeLimitInfo {
    ///MinAge 设施使用最小年龄 String  Y
    pub min_age: Option<String>,
    ///MaxAge 设施使用最大年龄 String Y
    pub max_age: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ReservationInfo {
    ///Reserve 设施预约限制 Enum    Y T：需要预约；F：无需预约
    pub reserve: Option<String>,
    ///Time 提前预约时间 String Y
    pub time: Option<String>,
    ///TimeUnit 提前预约时间单位 Enum Y Dia：天；Hora：小时；Minuto：分钟
    pub time_unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HotelCloseTime {
    ///startDay 开始时间 String Y
    pub statrt_day: Option<String>,
    ///endDay 结束时间 String Y
    pub end_day: Option<String>,
    ///reason 原因 String Y 1=政府要求关停；2=短期装修/停业；3=季节性关停；4=酒店停水停电； 5=酒店设施设备维修；6=不可抗力
    pub reason: Option<String>,
    ///reasonName 原因描述 String Y
    pub reason_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChildPolicy {
    ///AllowChildrenToStayV2 是否允许携带儿童入住 String Y 可能的值：true, false, unknown；true时解析ChildPolicy节点的其他信息
    pub allow_children_to_stay_v2: Option<String>,
    ///AllowChildrenToStaLimitOfChildAge 允许携带儿童的年龄限制 String  Y > 0 酒店允许携带X岁以上儿童入住，<= 0 不限制年龄
    pub allow_children_to_sta_limit_of_child_age: Option<String>,
    ///ChildBreakfast 儿童早餐政策 ChildBreakfast Y
    pub child_breakfast: Option<ChildBreakfast>,
    ///AllowUseExistingBedV2 儿童是否可使用现有床位 String Y 可能的值：true, false, unknown；true时解析ExistingBed查看具体政策
    pub allow_use_existing_bed_v2: Option<String>,
    ///ExistingBed 使用现有床铺政策 ExistingBed Y
    pub existing_bed: Option<ExistingBed>,
    ///ExtraBedLimit 是否能加床 String Y -1 未知，0 不能，1 能；1时解析ExtraBedPolicy查看具体政策
    pub extra_bed_limit: Option<String>,
    ///ExtraCribLimit 是否能加婴儿床 String Y -1 未知，0 不能，1 能；1时解析ExtraBedPolicy查看具体政策
    pub extra_crib_limit: Option<String>,
    ///ExtraBedPolicy 加床政策列表 ExtraBedPolicy[] Y
    pub extra_bed_policy: Option<Vec<ExtraBedPolicy>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChildBreakfast {
    /// SetSeparately 是否有单独儿童早餐 String Y T有；F无；T时解析ChildBreakfast节点的其他信息
    pub set_separately: Option<String>,
    /// RangeType 范围类型 String Y Age：年龄；Height：身高
    pub range_type: Option<String>,
    /// Currenry 货币类型 String Y
    pub currency: Option<String>,
    /// ChildBreakfastDetails 儿童早餐详细信息 ChildBreakfastDetail[] Y
    pub child_breakfast_details: Option<Vec<ChildBreakfastDetail>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ChildBreakfastDetail {
    /// RangeFrom 范围起始 String Y
    pub range_from: Option<String>,
    /// RangeTo 范围截止 String Y
    pub range_to: Option<String>,
    /// Amount 金额 f64 Y 0代表免费；-1代表未知，详询酒店
    pub amount: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExistingBed {
    /// Fees 收费标准列表 Fee[] Y
    pub fees: Option<Vec<Fee>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Fee {
    ///Amount 收费金额 Amount[] Y 该节点为空时表示价格未知
    pub amount: Option<Vec<Amount>>,
    ///ChargeFrequency 收费频率 String Y Daily每晚；PerStay每次入住
    pub charge_frequency: Option<String>,
    ///ChargeUnit 收费单位 String Y PerPerson每人
    pub charge_unit: Option<String>,
    ///RangeLimit 使用儿童范围 RangeLimit[] Y
    pub range_limit: Option<Vec<RangeLimit>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Amount {
    /// Amount 金额 f64 Y 0为免费
    pub amount: Option<f64>,
    /// Currency 币种 String Y
    pub currency: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MealInfo {
    ///BreakfastType 早餐类型 Integer Y 0.未知，1.不包含儿童早餐，2.包含儿童早餐
    pub breakfast_type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RangeLimit {
    ///Start 起始值 String Y
    pub start: Option<String>,
    ///End   截止值String Y
    pub end: Option<String>,
    ///Type 类型 String Y Age：年龄；Height：身高
    pub type_: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExtraBedPolicy {
    ///BedType 加床类型 Integer Y 1： 加床； 2：加婴儿床
    pub bed_type: Option<i32>,
    ///ChargeFrequency 收费频率 Integer Y 1：每晚 2：每次入住
    pub charge_frequency: Option<i32>,
    ///ChargeType 收费类型 String Y 1：明确价格；2：占房费比
    pub charge_type: Option<i32>,
    ///Currency 加床费用币种 String Y
    pub currency: Option<String>,
    ///Fee 收费金额或比例 f64 Y
    pub fee: Option<f64>,
    ///RangeFrom 年龄范围左 String Y
    pub range_from: Option<String>,
    ///RangeTo 年龄范围右 String Y
    pub range_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Supplier {
    ///SupplierID  供应商IDString N
    pub supplier_id: Option<String>,
    ///HotelCode   对应供应商编码String N 对应供应商编码  
    pub hotel_code: Option<String>,
    ///WeekendStart    Int N 用于房价的周末价计算。为0表示周末设置从周一开始
    pub weekend_start: Option<i32>,
    ///WeekendEnd  星期结束设置Int Y 为0表示到周日结束，但是两个都为0表示无周末设置； 如果开始为3，结束为1，表示从周三到下周1都是周末设置1代表周一，7代表周日
    pub weekend_end: Option<i32>,
    ///InstantRoomTypes    即时确认的销售房型String Y 多个房型以逗号分隔订单是否即时订单还受订单使用库存的影响，最终下单后通过即时接口查询
    pub instant_room_types: Option<String>,
    ///Status  供应商有效状态Boolean N 是否有效；无效的供应商关联的产品和库存不能销售
    pub status: Option<bool>,
    ///InvokeType  酒店使用库存和价格的方式 String Y DATA： 使用离线数据接口和搜索接口均可获取到该酒店；SEARCH：只有搜索接口能获取到该酒店。为空时默认为DATA（该字段已废弃）
    pub invoke_type: Option<String>,
    ///AvailPolicy 特殊政策 AvailPolicy Y 请把此信息展示给用户，以便用户预订。参考AvailPolicy节点
    pub avail_policy: Option<AvailPolicy>,
    ///HelpfulTip 温馨提示 HelpFulTipY 参考HelpFulTip节点
    pub helpful_tip: Option<HelpfulTip>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AvailPolicy {
    ///StartDate   特殊政策开始日期    Date Y 格式：yyyy-MM-dd'T'HH:mm:ss'+08:00'；例如：2021-09-15T00:00:00+08:00
    pub start_date: Option<String>,
    ///EndDate 特殊政策结束日期    Date Y
    pub end_date: Option<String>,
    ///Description 特殊政策中文描述    String Y 例如：此酒店不能接待外宾；  例如：1、客人延住请发延住单，不要发新订单;2、此酒店无停车场。
    pub description: Option<String>,
    ///DescriptionEn   特殊政策英文描述    String    Y
    pub description_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HelpfulTip {
    ///StartDate 温馨提示开始日期 Date Y
    pub start_date: Option<String>,
    ///EndDate 温馨提示结束日期  Date Y
    pub end_date: Option<String>,
    ///Description 温馨提示中文描述 String Y
    pub description: Option<String>,
    ///DescriptionEn 温馨提示英文描述 String Y
    pub description_en: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Room {
    /// RoomID  房型ID String N 对应动态接口中的RoomId
    #[serde(rename = "RoomID")]
    pub room_id: String,
    /// RoomName 房型中文名称 String N
    pub room_name: String,
    /// RoomNameEn 房型英文名称 String Y
    pub room_name_en: Option<String>,
    /// Area 房型面积 String Y
    pub area: Option<String>,
    /// Floor 楼层 String Y
    pub floor: Option<String>,
    /// Comments 中文备注 String Y
    pub comments: Option<String>,
    /// CommentsEn 英文备注 String Y
    pub comments_en: Option<String>,
    /// Capacity    房间最大入住人数    Int Y 如没有提供请根据房间名称判断：单人间或有单字的为1人，三人间的为3人，其他的默认2人；7表示6人以上。床位房购买的为床位，与此处的入住人数无关
    pub capacity: Option<i32>,
    /// Amount 房型数量    Int Y
    pub amount: Option<i32>,
    /// Facilities 房间设施列表 Facility[] Y 参见Facility节点，建议切换为：FacilityV2进行展示
    pub facilities: Option<Vec<Facility>>,
    /// RoomBed 新床型信息 RoomBed Y V1.62新增，参见RoomBed节点
    pub room_bed: Option<RoomBed>,
    /// FacilityV2 新V2设施节点 FacilityType[] Y V1.62新增，参见FacilityType节点 V2设施节点相较Facilities节点拓展了设施的分类、是否收费、使用限制等其他信息。
    pub facility_v2: Option<Vec<FacilityType>>,
    /// Sharing 户型类型 String Y V1.62新增，民宿特有 bed:床位 true:独立单间 false:整套
    pub sharing: Option<String>,
    /// RoomTypeSummary 户型 String Y V1.62新增，几室几厅几卫
    pub room_type_summary: Option<String>,
    /// WindowTypeId 窗户类型ID String Y 84：无窗 85：部分窗 677：有窗 897：內窗 898：天窗 899：封闭窗 900：飘窗 2268：落地窗 2269：装饰性假窗 2270：窗户较小 2271：窗外有墙体或遮挡 2272：部分有窗且位于走廊或过道 2273：部分有窗且为天窗 2274：部分有窗且为封闭窗 2275：部分有窗且窗户较小 2276：部分有窗且窗外有墙体或遮挡 2277：部分有窗且为装饰性假窗 2278：部分有窗且为飘窗 2279：部分有窗且为落地窗
    pub window_type_id: Option<String>,
    /// WindosType 窗户类型描述 String Y 窗户类型的名称
    pub window_type: Option<String>,
    /// RoomTags 房型标签 Integer[] Y 房型标签ID
    pub room_tags: Option<Vec<i32>>,
    /// ChildPolicy 儿童政策 ChildPolicy Y V1.62新增，参见ChildPolicy节点 儿童及加床政策，若超过房型限定人数，可能需要收取额外费用。提出的任何请求均需要获得酒店的确认，所有服务详情以酒店告知为准。
    pub child_policy: Option<ChildPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Image {
    ///RoomID 关联的房型ID String Y
    #[serde(rename = "RoomID")]
    pub room_id: Option<String>,
    ///Type 图片类型 Int N 1 - 餐厅 (Restaurant) 2 - 休闲 (Recreation Facilities) 3 - 会议室 (Meeting/Conference) 5 - 外观 (Exterior) 6 - 大堂/接待台   (Lobby/ Reception) 8 - 客房 (Guest Room) 10 - 其他 (Other Facilities) 11 - 公共区域 (Public Area) 12 - 周边景点 (Nearby Attractions)
    pub r#type: Option<i32>,
    ///TypeName 图片类型中文名 String Y
    pub type_name: Option<String>,
    ///TypeNameEn 图片类型英文名 String Y
    pub type_name_en: Option<String>,
    ///AuthorType 图片来源 String Y 由用户上传，或者酒店上传，Hotel - 酒店；User - 用户
    pub author_type: Option<String>,
    ///IsCoverImage 是否是主图 Boolean Y 当值为true的时候，表示这个图片是主图(封面图)，可用于显示在列表中
    pub is_cover_image: Option<bool>,
    ///IsRoomCoverImage 是否为房间主图 Boolean Y 当值为true的时候，表示这个图片是房间主图，可用于显示在列表中
    pub is_room_cover_image: Option<bool>,
    ///Locations 图片地址列表 Location[] Y 参考Location节点
    pub locations: Option<Vec<Location>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    ///WaterMark 是否有水印 Int N 0-无,1-有。默认为有水印图片
    pub watermark: Option<i32>,
    ///Size 尺寸类型 Int N  包含以下尺寸类型 375x200、350x350、640x960、800x600、1080x800、1140x640
    pub size: Option<i32>,
    ///Url 图片地址 String Y
    pub url: Option<String>,
}

impl BaseResponse for ElongResponse<StaticInfoResponse> {
    fn from_json(json: String) -> Result<Self, ElongError> {
        log::debug!("ElongResponse<StaticHotelResponse> json: {}", json);
        Ok(serde_json::from_str(&json)?)
    }
}

use std::env;

pub enum ApiEndpoint {
    /// 生产环境
    Prod,
    /// 测试环境
    Test,
}

impl ApiEndpoint {
    pub fn url(&self) -> String {
        match self {
            ApiEndpoint::Prod => env::var("ELONG_API_URL")
                .unwrap_or_else(|_| "https://api.elong.com/rest".to_string()),
            ApiEndpoint::Test => env::var("ELONG_TEST_API_URL")
                .unwrap_or_else(|_| "https://api-test.elong.com/rest".to_string()),
        }
    }
}

pub enum ApiMethod {
    /// hotel.static.city           城市列表-静态信息(json)
    StaticCity,
    /// hotel.static.list           酒店列表-静态信息(json)
    StaticList,
    /// hotel.static.info           酒店详情-静态信息(json)
    StaticInfo,
    /// hotel.static.grade          酒店点评评分、点评标签
    StaticGrade,
    /// hotel.static.brand          品牌列表-静态信息(json)
    StaticBrand,
    /// hotel.static.group          酒店集团信息-静态信息
    StaticGroup,
    /// hotel.dictionary            酒店字典数据-静态信息
    Dictionary,
    /// hotel.data.rp               国内酒店-产品详情 (不建议使用)
    DataRp,
    /// hotel.incr.state            国内酒店-产品状态增量 (不建议使用)
    IncrState,
    /// hotel.data.inventory        国内酒店-房态库存 (不建议使用)
    DataInventory,
    /// hotel.incr.inv              国内酒店-库存增量
    IncrInv,
    /// hotel.incr.sharding.inv     国内酒店-分片库存增量 (不建议使用)
    IncrShardingInv,
    /// hotel.data.rate             国内酒店-产品价格 (不建议使用)
    DataRate,
    /// hotel.incr.rate             国内酒店-价格增量 (不建议使用)
    IncrRate,
    /// hotel.incr.sharding.rate    国内酒店-分片价格增量 (不建议使用)
    ShardingRate,
    /// hotel.incr.id               国内酒店-增量编号
    IncrId,
    /// hotel.incr.sharding.id      国内酒店-分片增量编号
    IncrShardingId,
    /// hotel.incr.sharding.state   国内酒店-分片状态增量 (不建议使用)
    IncrShardingState,
    /// hotel.data.validate         国内酒店-数据验证
    DataValidate,
    /// hotel.data.booking          国内酒店-预定数据
    DataBooking,
    /// common.creditcard.validate  国内酒店-信用卡验证
    CreditCardValidate,
    /// hotel.order.create          国内酒店-创建订单
    OrderCreate,
    /// hotel.order.pay             国内酒店-支付订单
    OrderPay,
    /// hotel.order.pay.confirm     国内酒店-支付订单确认
    OrderPayConfirm,
    /// hotel.incr.order            国内酒店-订单增量
    IncrOrder,
    /// hotel.order.detail          国内酒店-订单详情
    OrderDetail,
    /// hotel.order.update          国内酒店-修改订单
    OrderUpdate,
    /// hotel.order.cancel          国内酒店-取消订单
    OrderCancel,
    /// hotel.order.promote         国内酒店-催确认
    OrderPromote,
    /// hotel.order.related         国内酒店-关联订单
    OrderRelated,
    /// hotel.order.feedback        国内酒店-入住反馈
    OrderFeedback,
    /// hotel.order.addinvoice      国内酒店-补开发票
    OrderAddinvoice,
    /// hotel.order.list            国内酒店-订单列表
    OrderList,
    /// common.exchangerate         国内酒店-汇率
    ExchangeRate,
    /// hotel.detail                国内酒店-酒店详情搜索
    HotelDetail,
    /// hotel.rate.min         国内酒店-酒店最低价搜索
    HotelRateMin,
}

impl ApiMethod {
    pub fn name(&self) -> &'static str {
        match self {
            ApiMethod::StaticCity => "hotel.static.city",
            ApiMethod::StaticList => "hotel.static.list",
            ApiMethod::StaticInfo => "hotel.static.info",
            ApiMethod::StaticGrade => "hotel.static.grade",
            ApiMethod::StaticBrand => "hotel.static.brand",
            ApiMethod::StaticGroup => "hotel.static.group",
            ApiMethod::Dictionary => "hotel.dictionary",
            ApiMethod::DataRp => "hotel.data.rp",
            ApiMethod::IncrState => "hotel.incr.state",
            ApiMethod::DataInventory => "hotel.data.inventory",
            ApiMethod::IncrInv => "hotel.incr.inv",
            ApiMethod::IncrShardingInv => "hotel.incr.sharding.inv",
            ApiMethod::DataRate => "hotel.data.rate",
            ApiMethod::IncrRate => "hotel.incr.rate",
            ApiMethod::ShardingRate => "hotel.incr.sharding.rate",
            ApiMethod::IncrId => "hotel.incr.id",
            ApiMethod::IncrShardingId => "hotel.incr.sharding.id",
            ApiMethod::IncrShardingState => "hotel.incr.sharding.state",
            ApiMethod::DataValidate => "hotel.data.validate",
            ApiMethod::DataBooking => "hotel.data.booking",
            ApiMethod::CreditCardValidate => "common.creditcard.validate",
            ApiMethod::OrderCreate => "hotel.order.create",
            ApiMethod::OrderPay => "hotel.order.pay",
            ApiMethod::OrderPayConfirm => "hotel.order.pay.confirm",
            ApiMethod::IncrOrder => "hotel.incr.order",
            ApiMethod::OrderDetail => "hotel.order.detail",
            ApiMethod::OrderUpdate => "hotel.order.update",
            ApiMethod::OrderCancel => "hotel.order.cancel",
            ApiMethod::OrderPromote => "hotel.order.promote",
            ApiMethod::OrderRelated => "hotel.order.related",
            ApiMethod::OrderFeedback => "hotel.order.feedback",
            ApiMethod::OrderAddinvoice => "hotel.order.addinvoice",
            ApiMethod::OrderList => "hotel.order.list",
            ApiMethod::ExchangeRate => "common.exchangerate",
            ApiMethod::HotelDetail => "hotel.detail",
            ApiMethod::HotelRateMin => "hotel.rate.min",
        }
    }
}

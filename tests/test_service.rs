use elong_sdk::elong::endpoints::ApiEndpoint;
use elong_sdk::elong::service::ElongService;
use elong_sdk::request::data_booking::DataBookingRequest;
use elong_sdk::request::data_inventory::InventoryRequest;
use elong_sdk::request::data_rate::DataRateRequest;
use elong_sdk::request::data_rp::DataRpRequest;
use elong_sdk::request::data_validate::DataValidateRequest;
use elong_sdk::request::dictionary::DictionaryRequest;
use elong_sdk::request::exchangerate::ExchangerateRequest;
use elong_sdk::request::incr_id::IncrIdRequest;
use elong_sdk::request::incr_inv::IncrInvRequest;
use elong_sdk::request::incr_order::IncrOrderRequest;
use elong_sdk::request::incr_rate::IncrRateRequest;
use elong_sdk::request::incr_state::IncrStateRequest;
use elong_sdk::request::order_addinvoice::{
    DedicatedInvoice, DeliveryAddress, OrderAddinvoiceRequest,
};
use elong_sdk::request::order_cancel::OrderCancelRequest;
use elong_sdk::request::order_create::{Contact, Customer, OrderCreateRequest, OrderRoom};
use elong_sdk::request::order_detail::OrderDetailRequest;
use elong_sdk::request::order_feedback::OrderFeedbackRequest;
use elong_sdk::request::order_list::OrderListRequest;
use elong_sdk::request::order_pay::OrderPayRequest;
use elong_sdk::request::order_pay_confirm::OrderPayConfirmRequest;
use elong_sdk::request::order_related::OrderRelatedRequest;
use elong_sdk::request::static_brand::StaticBrandRequest;
use elong_sdk::request::static_city::StaticCityRequest;
use elong_sdk::request::static_grade::StaticGradeRequest;
use elong_sdk::request::static_group::StaticGroupRequest;
use elong_sdk::request::static_info::StaticInfoRequest;
use elong_sdk::request::static_list::StaticListRequest;
use elong_sdk::Elong;

fn create_test_service() -> ElongService {
    let mut service = ElongService::new();
    service.url = ApiEndpoint::Prod.url();
    service
}

/// 城市列表
#[tokio::test]
async fn test_get_static_city() {
    let service = create_test_service();

    let request = StaticCityRequest {
        country_type: Some(1),
        is_need_location: Some(true),
        ..Default::default()
    };

    let result = service.get_static_city(request).await;
    print!("result: {:?}", result);
    assert!(result.is_ok());
}

/// 酒店列表
#[tokio::test]
async fn test_get_static_list() {
    let service = create_test_service();

    let request = StaticListRequest {
        start_time: Some("2025-04-016 00:00:00".to_string()),
        end_time: Some("2025-07-016 00:00:00".to_string()),
        city_id: "0101".to_string(),
        page_size: Some(10),
        page_index: Some(1),
    };

    let result = service.get_static_list(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 酒店信息
#[tokio::test]
async fn test_get_static_info() {
    let service = create_test_service();

    let request = StaticInfoRequest {
        hotel_id: "91427260".to_string(),
        options: Some("1,2,3,4,5,6".to_string()),
    };

    let result = service.get_static_info(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 点评评分
#[tokio::test]
async fn test_get_static_grade() {
    let service = create_test_service();

    let request = StaticGradeRequest {
        hotel_id: "93993637".to_string(),
    };

    let result = service.get_static_grade(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 酒店品牌
#[tokio::test]
async fn test_get_static_brand() {
    let service = create_test_service();

    let request = StaticBrandRequest {
        status: Some(0),
        page_size: 200,
        page_index: 1,
    };

    let result = service.get_static_brand(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 酒店集团
#[tokio::test]
async fn test_get_static_group() {
    let service = create_test_service();

    let request = StaticGroupRequest {
        status: Some(0),
        page_size: 200,
        page_index: 1,
    };

    let result = service.get_static_group(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 酒店字典
#[tokio::test]
async fn test_get_hotel_dictionary() {
    let service = create_test_service();

    let request = DictionaryRequest {
        r#type: 3,
        page: 1,
        limit: 100,
    };

    let result = service.get_hotel_dictionary(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 产品信息
#[tokio::test]
async fn test_get_data_rp() {
    let service = create_test_service();
    let request = DataRpRequest {
        hotel_ids: "03001008".to_string(),
        ..Default::default()
    };
    let result = service.get_data_rp(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量ID
#[tokio::test]
async fn test_get_incr_id() {
    let service = create_test_service();

    let request = IncrIdRequest {
        last_time: "2025-04-16 00:00:00".to_string(),
        sharding_key: 16,
        incr_type: "State".to_string(),
    };

    let result = service.get_incr_id(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量ID分片
#[tokio::test]
async fn test_get_incr_sharding_id() {
    let service = create_test_service();

    let request = IncrIdRequest {
        last_time: "2025-04-16 00:00:00".to_string(),
        sharding_key: 16,
        incr_type: "Rate".to_string(),
    };

    let result = service.get_incr_sharding_id(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量状态
#[tokio::test]
async fn test_get_incr_state() {
    let service = create_test_service();

    let request = IncrStateRequest {
        last_id: 9699962496,
        sharding_key: 16,
        count: Some(10),
    };

    let result = service.get_incr_state(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量状态分片
#[tokio::test]
async fn test_get_incr_sharding_state() {
    let service = create_test_service();

    let request = IncrStateRequest {
        last_id: 9699962496,
        sharding_key: 16,
        count: Some(10),
    };

    let result = service.get_incr_sharding_state(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 房态库存
#[tokio::test]
async fn test_get_data_inventory() {
    let service = create_test_service();

    let request = InventoryRequest {
        hotel_ids: "93993637".to_string(),
        hotel_codes: None,
        room_type_id: None,
        start_date: "2025-04-16".to_string(),
        end_date: "2025-04-17".to_string(),
        is_need_instant_confirm: None,
    };

    let result = service.get_inventory(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 产品价格
#[tokio::test]
async fn test_get_data_rate() {
    let service = create_test_service();

    let request = DataRateRequest {
        hotel_ids: "28005348".to_string(),
        hotel_codes: None,
        payment_type: "All".to_string(),
        start_date: "2025-06-03".to_string(),
        end_date: "2025-06-04".to_string(),
        invoice_mode: None,
    };

    let result = service.get_data_rate(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量库存
#[tokio::test]
async fn test_get_incr_inv() {
    let service = create_test_service();

    let request = IncrInvRequest {
        last_id: 0,
        sharding_key: 16,
        count: None,
    };

    let result = service.get_incr_inv(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量库存分片
#[tokio::test]
async fn test_get_incr_sharding_inv() {
    let service = create_test_service();

    let request = IncrInvRequest {
        last_id: 440360000001,
        sharding_key: 16,
        count: None,
    };

    let result = service.get_incr_sharding_inv(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量价格
#[tokio::test]
async fn test_get_incr_rate() {
    let service = create_test_service();

    let request = IncrRateRequest {
        last_id: 0,
        sharding_key: 16,
        count: Some(10),
    };

    let result = service.get_incr_rate(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 增量价格分片
#[tokio::test]
async fn test_get_incr_sharding_rate() {
    let service = create_test_service();

    let request = IncrRateRequest {
        last_id: 318860000001,
        sharding_key: 16,
        count: Some(10),
    };

    let result = service.get_incr_sharding_rate(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 数据验证
#[tokio::test]
async fn test_data_validate() {
    let service = create_test_service();

    let request = DataValidateRequest {
        arrival_date: "2025-04-25".to_string(),
        departure_date: "2025-04-26".to_string(),
        earliest_arrival_time: "2025-04-25 14:00:00".to_string(),
        latest_arrival_time: "2025-04-26 18:00:00".to_string(),
        hotel_id: "24600325".to_string(),
        room_type_id: "0001".to_string(),
        rate_plan_id: 415276135,
        total_price: 244.44,
        number_of_rooms: 1,
        ..Default::default()
    };

    let result = service.data_validate(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 预定数据
#[tokio::test]
async fn test_data_booking() {
    let service = create_test_service();

    let request = DataBookingRequest {
        arrival_date: "2025-04-23".to_string(),
        departure_date: "2025-04-24".to_string(),
        hotel_id: "26333143".to_string(),
        hotel_code: "26548651".to_string(),
        room_type_id: "0003".to_string(),
        rate_plan_id: 390911172,
        payment_type: "Prepay".to_string(),
        ..Default::default()
    };

    let result = service.data_booking(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 订单创建
#[tokio::test]
async fn test_order_create() {
    let service = create_test_service();

    // 自定义订单号
    let custom_order_id = "1234567890".to_string();

    let order_rooms = vec![OrderRoom {
        customers: vec![Customer {
            name: "林立体".to_string(),
            gender: "Unknown".to_string(),
            nationality: "中国".to_string(),
            nat: None,
            id_card_no: None,
            id_card_type: None,
            first_name: None,
            last_name: None,
        }],
    }];
    let contact = Contact {
        name: "林立体".to_string(),
        email: None,
        mobile_area_code: "86".to_string(),
        mobile: "18301221126".to_string(),
        phone: None,
        fax: None,
        gender: "Unknown".to_string(),
        first_name: None,
        last_name: None,
    };

    let request = OrderCreateRequest {
        affiliate_confirmation_id: custom_order_id,
        hotel_id: "28005348".to_string(),
        room_id: None,
        room_type_id: "0003".to_string(),
        rate_plan_id: 416853117,
        arrival_date: "2025-06-03".to_string(),
        departure_date: "2025-06-04".to_string(),
        hour_room_start_time: None,
        hour_room_end_time: None,
        customer_type: None,
        payment_type: "Prepay".to_string(),
        number_of_rooms: 1,
        number_of_customers: 1,
        earliest_arrival_time: "2025-06-03 14:00:00".to_string(),
        latest_arrival_time: "2025-06-04 18:00:00".to_string(),
        currency_code: "RMB".to_string(),
        total_price: 284.59,
        customer_ip_address: "127.0.0.1".to_string(),
        is_guarantee_or_charged: Some(true),
        confirmation_type: "SMS_cn".to_string(),
        note_to_hotel: None,
        note_to_elong: None,
        is_need_invoice: None,
        order_rooms,
        invoice: None,
        contact,
        credit_card: None,
        dove_corp_card: None,
        extend_info: None,
        is_create_order_only: None,
        customer_price: Some(308.00),
        order_validation: None,
        coupon: None,
        little_majia_id: None,
        goods_uniq_id: None,
        specific_remark: None,
        child_ages: None,
        number_of_adults: None,
        hotel_code: None,
        supplier_id: None,
        sub_supplier_id: None,
        shopper_product_id: None,
        encrypt_option: None,
        day_price_list: None,
    };

    let result = service.order_create(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 订单支付
#[tokio::test]
async fn test_order_pay() {
    let service = create_test_service();

    let request = OrderPayRequest {
        order_id: 1234567890,
        is_guarantee_or_charged: true,
        credit_card: None,
        dove_corp_card: None,
        amount: 284.59,
    };

    let result = service.order_pay(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 订单支付确认
#[tokio::test]
async fn test_order_pay_confirm() {
    let service = create_test_service();

    let request = OrderPayConfirmRequest {
        order_id: 1234567890,
        sms_code: "123456".to_string(),
        amount: 284.59,
    };

    let result = service.order_pay_confirm(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 订单增量
#[tokio::test]
async fn test_order_incr() {
    let service = create_test_service();

    let request = IncrOrderRequest {
        last_id: 0,
        count: Some(1000),
    };

    let result = service.order_incr(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 订单详情
#[tokio::test]
async fn test_order_detail() {
    let service = create_test_service();

    let request = OrderDetailRequest {
        order_id: 123456,
        affiliate_confirmation_id: None,
        options: None,
    };

    let result = service.order_detail(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 取消订单
#[tokio::test]
async fn test_order_cancel() {
    let service = create_test_service();

    let request = OrderCancelRequest {
        order_id: 1234567890,
        cancel_code: "行程变更".to_string(),
        reason: Some("对酒店相关条件不满意".to_string()),
        penalty_amount: Some(0.0),
    };

    let result = service.order_cancel(request).await;

    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 订单崔确认
#[tokio::test]
async fn test_order_promote() {
    let service = create_test_service();

    let request = elong_sdk::request::order_promote::OrderPromoteRequest {
        order_id: 1234567890,
    };

    let result = service.order_promote(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 关联订单
#[tokio::test]
async fn test_order_related() {
    let service = create_test_service();
    let request = OrderRelatedRequest {
        order_ids: "1234567890".to_string(),
        relation_type: "0987654321".to_string(),
    };

    let result = service.order_related(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 入住反馈
#[tokio::test]
async fn test_order_feedback() {
    let service = create_test_service();

    let request = OrderFeedbackRequest {
        order_id: 1234567890,
        arrival_date: None,
        departure_date: None,
        customer_name: Some("林立体".to_string()),
        room_number: None,
        notes: None,
    };

    let result = service.order_feedback(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 补开发票
#[tokio::test]
async fn test_order_addinvoice() {
    let service = create_test_service();

    let delivery_address = DeliveryAddress {
        province: Some("北京市".to_string()),
        city: Some("北京市".to_string()),
        district: Some("海淀区".to_string()),
        street: Some("某街道".to_string()),
        recipient_name: Some("林立体".to_string()),
        post_email: Some("123@123.COM".to_string()),
        phone: "12345678901".to_string(),
        email: None,
    };

    let dedicated_invoice = DedicatedInvoice {
        tax_payer_num: "123456789012345678".to_string(),
        tax_register_bank: "某银行".to_string(),
        register_bank_num: "123456789012345678".to_string(),
        shotel_address: "某地址".to_string(),
        register_phone_num: "12345678901".to_string(),
    };

    let request = OrderAddinvoiceRequest {
        delivery_info: delivery_address,
        order_id: 1234567890,
        title: Some("林立体".to_string()),
        user_type: 1,
        item_name: "代订住宿费".to_string(),
        amount: 284.59,
        invoice_type: 1,
        invoice_level: 1,
        itin: Some("123456789012345678".to_string()),
        need_relation_order: 1,
        dedicated_invoice: Some(dedicated_invoice),
        encrypt_option: Some(0),
    };

    let result = service.order_addinvoice(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 订单列表
#[tokio::test]
async fn test_order_list() {
    let service = create_test_service();

    let request = OrderListRequest {
        creation_time_from: None,
        creation_time_to: None,
        hotel_id: None,
        room_type_id: None,
        rate_plan_id: None,
        arrival_date_from: None,
        arrival_date_to: None,
        departure_date_from: None,
        departure_date_to: None,
        min_update_time: None,
        max_update_time: None,
        mobile: None,
        customer_name: None,
        status: None,
        page_index: 1,
    };

    let result = service.order_list(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 汇率
#[tokio::test]
async fn test_get_exchange_rate() {
    let service = create_test_service();

    let request = ExchangerateRequest {
        currency_id: "USD".to_string(),
    };
    let result = service.exchangerate(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

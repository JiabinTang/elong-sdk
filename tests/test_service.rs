use elong_offline_sdk::elong::service::ElongService;
use elong_offline_sdk::request::data_booking::DataBookingRequest;
use elong_offline_sdk::request::data_inventory::InventoryRequest;
use elong_offline_sdk::request::data_rate::DataRateRequest;
use elong_offline_sdk::request::data_rp::DataRpRequest;
use elong_offline_sdk::request::data_validate::DataValidateRequest;
use elong_offline_sdk::request::incr_id::IncrIdRequest;
use elong_offline_sdk::request::incr_inv::IncrInvRequest;
use elong_offline_sdk::request::incr_rate::IncrRateRequest;
use elong_offline_sdk::request::incr_state::IncrStateRequest;
use elong_offline_sdk::request::static_brand::StaticBrandRequest;
use elong_offline_sdk::request::static_city::StaticCityRequest;
use elong_offline_sdk::request::static_grade::StaticGradeRequest;
use elong_offline_sdk::request::static_info::StaticInfoRequest;
use elong_offline_sdk::request::static_list::StaticListRequest;
use elong_offline_sdk::Elong;

fn create_test_service() -> ElongService {
    ElongService::new()
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
        hotel_id: "93993637".to_string(),
        options: Some("1,2,3,4,5,6".to_string()),
    };

    let result = service.get_static_info(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

/// 点评评分
#[tokio::test]
async fn test_get_static_grade(){
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

/// 产品信息
#[tokio::test]
async fn test_get_data_rp() {
    let service = create_test_service();
    let request = DataRpRequest {
        hotel_ids: "93993637".to_string(),
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
        hotel_ids: "26315956".to_string(),
        hotel_codes: None,
        payment_type: "All".to_string(),
        start_date: "2025-04-23".to_string(),
        end_date: "2025-04-23".to_string(),
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
        departure_date:"2025-04-24".to_string(),
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

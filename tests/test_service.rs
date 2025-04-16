use elong_offline_sdk::elong::service::ElongService;
use elong_offline_sdk::request::data_inventory::InventoryRequest;
use elong_offline_sdk::request::data_rate::DataRateRequest;
use elong_offline_sdk::request::incr_inv::IncrInvRequest;
use elong_offline_sdk::request::incr_rate::IncrRateRequest;
use elong_offline_sdk::request::static_city::StaticCityRequest;
use elong_offline_sdk::request::static_info::StaticInfoRequest;
use elong_offline_sdk::request::static_list::StaticListRequest;
use elong_offline_sdk::Elong;

fn create_test_service() -> ElongService {
    ElongService::new()
}

#[tokio::test]
async fn test_get_static_city() {
    let service = create_test_service();

    let request = StaticCityRequest {
        country_type: 0,
        city_id_type: 0,
        is_need_location: false,
        page_size: 1,
        page_index: 1,
    };

    let result = service.get_static_city(request).await;
    print!("result: {:?}", result);
    assert!(result.is_ok());
}

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

#[tokio::test]
async fn test_get_data_rate() {
    let service = create_test_service();

    let request = DataRateRequest {
        hotel_ids: "93993637".to_string(),
        hotel_codes: None,
        payment_type: "All".to_string(),
        start_date: "2025-04-16".to_string(),
        end_date: "2025-04-16".to_string(),
        invoice_mode: None,
    };

    let result = service.get_data_rate(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}

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

#[tokio::test]
async fn test_get_incr_rate() {
    let service = create_test_service();

    let request = IncrRateRequest{
        last_id: 0,
        count: Some(10),
    };

    let result = service.get_incr_rate(request).await;
    print!("result: {:?}", result);

    assert!(result.is_ok());
    assert!(result.unwrap().is_success());
}
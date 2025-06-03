use elong_sdk::elong::{endpoints::ApiEndpoint, service::ElongService};

#[test]
fn test_new() {
    let service = ElongService::new();
    assert_eq!(service.url, ApiEndpoint::Prod.url());
}

#[test]
fn test_new_with_endpoint() {
    let service = ElongService::new_with_endpoint(
        "test_user".to_string(),
        "test_key".to_string(),
        "test_secret".to_string(),
        ApiEndpoint::Test,
    );
    assert_eq!(service.url, ApiEndpoint::Test.url());
}

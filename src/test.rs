use insta::assert_debug_snapshot;
use serde::Serialize;
use std::collections::HashMap;
use super::*;

fn get_api() -> SarufiAPI {
    let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    super::SarufiAPI::new(api_key).unwrap()

}

#[tokio::test]
async fn test_get_bot() {
    let api = get_api();
    let bot = api.get_bot(1045).await.unwrap();
    println!("Result: {:?}", bot);
    assert_debug_snapshot!(bot);
    
}
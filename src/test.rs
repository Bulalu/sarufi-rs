use std::time::Duration;

use dotenv::dotenv;


use super::*;
#[tokio::test]
async fn test_get_bot() {
    dotenv().ok();
    let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    let api = Sarufi::new(api_key).unwrap();
    let bot = api.get_bot(1205).await.unwrap();
    
    
    println!("Name: {:?}", bot.name);
    println!("ID: {:?}", bot.id);
    println!("Description: {:?}", bot.description);
    println!("Industry: {:?}", bot.industry);
    

}

#[tokio::test]
async fn test_get_all_bot() {
    dotenv().ok();
    let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    let api = Sarufi::new(api_key).unwrap();
    let bots = api.get_all_bots().await.unwrap();

    // assert_eq!(bots[0].id, "My Rust Chatbot");

    println!("Result: {:?}", bots.len());
    
}

// #[tokio::test]
// async fn test_delete_all_bots() {
//     dotenv().ok();
//     let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
//     let api = Sarufi::new(api_key).unwrap();
//     let bots = api.get_all_bots().await.unwrap();
    
//     for bot in bots {
//         api.delete_bot(bot.id).await.unwrap();
//         println!("Deleted bot {}", bot.id);
//         tokio::time::sleep(Duration::from_secs(1)).await; // Delay for one second
//     }
// }

#[tokio::test]
async fn test_create_bot() -> Result<(), ApiError> {
    dotenv().ok();
    let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    // println!("API_KEY: {:?}", api_key);
    let api = Sarufi::new(api_key).unwrap();

    let name = "My Rusty Chatbot";
    let description = Some("A rusty chatbot created using Sarufi API");
    let industry = Some("Technology");
    let flow: Option<HashMap<String, Value>> = None;
    let intents: Option<HashMap<String, Vec<String>>> = None;
    let webhook_url = Some("https://example.com/webhook");
    let webhook_trigger_intents: Option<Vec<String>> = None;
    let visible_on_community = Some(true);  

    let bot = api.create_bot(
            name,
            description,
            industry,
            flow,
            intents,
            webhook_url,
            webhook_trigger_intents,
            visible_on_community,
    ).await?;  

    println!("Result: {:?}", bot);
    println!("ID: {:?}", bot.id);


    assert_eq!(bot.name, name);
    assert_eq!(bot.description, description.unwrap());
    assert_eq!(bot.industry, industry.unwrap());

    Ok(())

    
}

#[tokio::test]
async fn test_update_bot() {
    dotenv().ok();
    let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    // println!("API_KEY: {:?}", api_key);
    let api = Sarufi::new(api_key).unwrap();

    let id = 1112; // change this to your bot id

    let prev_bot = api.get_bot(id).await.unwrap();
    
        
    let name = "My Other Rusty Chatbot";
    let description = Some("A rusty chatbot created using Sarufi API");
    let industry = Some("Technology");
    let flow: Option<HashMap<String, Value>> = None;
    let intents: Option<HashMap<String, Vec<String>>> = None;
    let webhook_url = Some("https://example.com/webhook");
    let webhook_trigger_intents: Option<Vec<String>> = None;
    let visible_on_community = Some(true);  

    let bot = api.update_bot(
            id,
            name,
            description,
            industry,
            flow,
            intents,
            webhook_url,
            webhook_trigger_intents,
            visible_on_community,
    ).await.unwrap();

    println!("Result: {:?}", bot.id);

    assert_eq!(bot.name, name);
    assert_eq!(bot.description, description.unwrap());
    assert_eq!(bot.industry, industry.unwrap());

}

#[tokio::test]
async fn test_fetch () {
    dotenv().ok();
    let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    let api = Sarufi::new(api_key).unwrap();


    let bot_id = 1145; // change this to your bot id
    let chat_id = "123456789";
    let message = "Hello";
    let message_type = "text";
    let channel = "other";

    let response = api._fetch_response(bot_id, chat_id, message, message_type, channel).await.unwrap();
    println!("Result: {:?}", response);

}

#[tokio::test]
async fn test_chat () {
    // Import the required types and traits
    dotenv().ok();
    let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    let api = Sarufi::new(api_key).unwrap();

    let bot_id = 1145; // change this to your bot id
    let response = api.chat(bot_id).await.unwrap();
    println!("Result: {:?}", response.as_str());


}








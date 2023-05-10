#[cfg_attr(test, macro_use)]
extern crate log;

use reqwest::{Client, ClientBuilder, header::HeaderMap, Response};
use serde::de::DeserializeOwned;

pub use errors::ApiError;
pub use bot::{Bot};
use serde_json::{json, Map, Value};
use std::{collections::HashMap, result};
use dotenv::dotenv;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

mod errors;
mod utils;
mod api;
mod bot;

use crate::api::SarufiApiError;



/// API struct. Exposes function to interact with the Sarufi API 🥷
/// #[derive(Debug, Serialize, Deserialize)]
pub struct SarufiAPI {
    client: Client,
}


impl SarufiAPI {
    /// Creates a new instance of SarufiAPI using the provided api key
    /// this function panics if the api_key is empty 🤒
    pub fn new<S: Into<String>>(api_key: S) -> Result<SarufiAPI, ApiError> {
        let owned_key = api_key.into();

        utils::validate_keys(&owned_key)?;

        let mut default_headers = HeaderMap::new();
        default_headers.insert("Authorization", format!("Bearer {}", owned_key).parse().unwrap());
        default_headers.insert("Content-Type", "application/json".parse().unwrap());


        let client = ClientBuilder::new().default_headers(default_headers).build()?;

        Ok(SarufiAPI { client })
    }


     
        pub async fn get_bot(&self, id: usize) -> Result<(Bot), ApiError> {
            let url = utils::api_url(&format!("/chatbot/{}", id));
            let response = self.client.get(&url).send().await?;

            if response.status().is_success() {
                let mut result = response.json::<Bot>().await?;
                Ok(result)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
  
        }


        pub async fn get_all_bots(&self) -> Result<Vec<Bot>, ApiError> {
            let url = utils::api_url("/chatbots");
            let response = self.client.get(&url).send().await?;

            if response.status().is_success() {
                // let json_string = response.text().await?;
                // let json_value: Value = serde_json::from_str(&json_string).unwrap();
                // let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
                // println!("{}", pretty_json);
                let result = response.json::<Vec<Bot>>().await?;
                Ok(result)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
  
        }

        pub async fn _fetch_response(&self, bot_id: usize, chat_id: &str, message: &str, message_type: &str, channel: &str) -> Result<(), ApiError> {
            let url = utils::api_url("/conversation");

            if (channel == "whatsapp") {
                println!("Whent to whatsapp");
                let url = utils::api_url("/conversation/whatsapp");
                println!("URL: {:?}", url)
            }

            let mut data = HashMap::new();
            data.insert("bot_id".to_owned(), Value::Number(serde_json::Number::from(bot_id)));
            data.insert("chat_id".to_owned(), Value::String(chat_id.to_owned()));
            data.insert("message".to_owned(), Value::String(message.to_owned()));
            data.insert("message_type".to_owned(), Value::String(message_type.to_owned()));
            data.insert("channel".to_owned(), Value::String(channel.to_owned()));

            let response = self.client.post(&url).json(&Value::Object(data.into_iter().collect())).send().await?;
            // println!("Data: {:?}", data);
            if response.status().is_success() {
                let json_string = response.text().await?;
                let json_value: Value = serde_json::from_str(&json_string).unwrap();
                let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
                println!("{}", pretty_json);
                Ok(())
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
                // println!("Error")
            }

        }


        pub async fn delete_bot(&self, id: usize) -> Result<(), ApiError> {
            let url = utils::api_url(&format!("/chatbot/{}", id));
            let response = self.client.delete(&url).send().await?;

            if response.status().is_success() {
     
                Ok(())
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
  
        }

        /// Creates a new bot
        pub async fn create_bot(&self, 
            name: &str,
            description: Option<&str>,
            industry: Option<&str>,
            flow: Option<HashMap<String, Value>>,
            intents: Option<HashMap<String, Vec<String>>>,
            webhook_url: Option<&str>,
            webhook_trigger_intents: Option<Vec<String>>,
            visible_on_community: Option<bool>) -> Result<Bot, ApiError> {

            let url = utils::api_url("/chatbot");
            let mut data = HashMap::new();

            data.insert("name".to_owned(), Value::String(name.to_owned()));
    
            if let Some(description) = description {
                data.insert("description".to_owned(), Value::String(description.to_owned()));
            }
        
            if let Some(industry) = industry {
                data.insert("industry".to_owned(), Value::String(industry.to_owned()));
            }
        
            if let Some(flow) = flow {
                data.insert("flow".to_owned(), Value::Object(flow.into_iter().collect()));
            }
        
            if let Some(intents) = intents {
                data.insert(
                    "intents".to_owned(),
                    Value::Object(
                        intents
                            .into_iter()
                            .map(|(k, v)| (k, Value::Array(v.into_iter().map(Value::String).collect())))
                            .collect(),
                    ),
                );
            }
        
            if let Some(webhook_url) = webhook_url {
                data.insert("webhook_url".to_owned(), Value::String(webhook_url.to_owned()));
            }
        
            if let Some(webhook_trigger_intents) = webhook_trigger_intents {
                data.insert(
                    "webhook_trigger_intents".to_owned(),
                    Value::Array(webhook_trigger_intents.into_iter().map(Value::String).collect()),
                );
            }

            if let Some(visible_on_community) = visible_on_community {
                data.insert("visible_on_community".to_owned(), Value::Bool(visible_on_community));
            }
        
            let response = self.client.post(&url).json(&Value::Object(data.into_iter().collect())).send().await?;
            
         
            if response.status().is_success() {
                
                let mut result = response.json::<Bot>().await?;

                // do you really need to check this?, seems to be working fine without it
                if let Some(e_metrics) = result.evaluation_metrics {
                    
                    result.evaluation_metrics = Some(e_metrics);
                }

                if let Some(confidence_threshold) = result.confidence_threshold {
                    
                    result.confidence_threshold = Some(confidence_threshold);
                }
                // let json_string = response.text().await?;
                // let json_value: Value = serde_json::from_str(&json_string).unwrap();
                // let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
                // println!("{}", pretty_json);
               
                Ok(result)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
            

        }


        pub async fn create_bot_from_file(
            &self,
            file_path: &str,
        ) -> Result<Bot, ApiError> {
            let file = File::open(file_path)?;
            let reader = BufReader::new(file);
            let data: Value = serde_json::from_reader(reader)?;
            let data = data.as_object().ok_or_else(|| ApiError::GenericError("Invalid JSON".to_owned()))?;
            print!("{:?}", data);
        
            let url = utils::api_url("/chatbot");
            let response = self.client.post(&url).json(&data).send().await?;
        
            if response.status().is_success() {
                let mut result = response.json::<Bot>().await?;
        
                if let Some(e_metrics) = result.evaluation_metrics {
                    result.evaluation_metrics = Some(e_metrics);
                }
        
                if let Some(confidence_threshold) = result.confidence_threshold {
                    result.confidence_threshold = Some(confidence_threshold);
                }
        
                Ok(result)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
        }

          pub async fn update_bot(&self, 
            id: usize,
            name: &str,
            description: Option<&str>,
            industry: Option<&str>,
            flow: Option<HashMap<String, Value>>,
            intents: Option<HashMap<String, Vec<String>>>,
            webhook_url: Option<&str>,
            webhook_trigger_intents: Option<Vec<String>>,
            visible_on_community: Option<bool>) -> Result<Bot, ApiError> {

            let url = utils::api_url(&format!("/chatbot/{}", id));
            let mut data = HashMap::new();

            data.insert("name".to_owned(), Value::String(name.to_owned()));
    
            if let Some(description) = description {
                data.insert("description".to_owned(), Value::String(description.to_owned()));
            }
        
            if let Some(industry) = industry {
                data.insert("industry".to_owned(), Value::String(industry.to_owned()));
            }
        
            if let Some(flow) = flow {
                data.insert("flow".to_owned(), Value::Object(flow.into_iter().collect()));
            }
        
            if let Some(intents) = intents {
                data.insert(
                    "intents".to_owned(),
                    Value::Object(
                        intents
                            .into_iter()
                            .map(|(k, v)| (k, Value::Array(v.into_iter().map(Value::String).collect())))
                            .collect(),
                    ),
                );
            }
        
            if let Some(webhook_url) = webhook_url {
                data.insert("webhook_url".to_owned(), Value::String(webhook_url.to_owned()));
            }
        
            if let Some(webhook_trigger_intents) = webhook_trigger_intents {
                data.insert(
                    "webhook_trigger_intents".to_owned(),
                    Value::Array(webhook_trigger_intents.into_iter().map(Value::String).collect()),
                );
            }

            if let Some(visible_on_community) = visible_on_community {
                data.insert("visible_on_community".to_owned(), Value::Bool(visible_on_community));
            }
        
            let response = self.client.put(&url).json(&Value::Object(data.into_iter().collect())).send().await?;
         
            if response.status().is_success() {
                let result = response.json::<Bot>().await?;
                Ok(result)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
          
        }

   
}

    
#[cfg(test)]
mod tests {
    use std::{env, time::Duration};

    use super::*;

    #[tokio::test]
    async fn test_get_bot() {
        dotenv().ok();
        let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
        let api = SarufiAPI::new(api_key).unwrap();
        let bot = api.get_bot(1046).await.unwrap();
        println!("Result: {:?}", bot);
    }

    #[tokio::test]
    async fn test_get_all_bot() {
        dotenv().ok();
        let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
        let api = SarufiAPI::new(api_key).unwrap();
        let bots = api.get_all_bots().await.unwrap();

        // assert_eq!(bots[0].id, "My Rust Chatbot");

        println!("Result: {:?}", bots.len());
        
    }

    // #[tokio::test]
    // async fn test_delete_all_bots() {
    //     dotenv().ok();
    //     let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
    //     let api = SarufiAPI::new(api_key).unwrap();
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
        let api = SarufiAPI::new(api_key).unwrap();
  
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
        // println!("Name: {:?}", bot.name);
        // println!("Description: {:?}", bot.description);
        // println!("Industry: {:?}", bot.industry);
        // println!("Webhook URL: {:?}", bot.webhook_url);
        // println!("Webhook Trigger Intents: {:?}", bot.webhook_trigger_intents);
        // println!("Visible on Community: {:?}", bot.visible_on_community);
        // println!("Intents: {:?}", bot.intents);
        // println!("Flows: {:?}", bot.flows);

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
        let api = SarufiAPI::new(api_key).unwrap();

        let id = 1112;

        let prev_bot = api.get_bot(id).await.unwrap();
        // println!("Prev Bot: {:?}", prev_bot);
         
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
        // println!("API_KEY: {:?}", api_key);
        let api = SarufiAPI::new(api_key).unwrap();

        let bot = api._fetch_response(1046, "123456789", "Hello", "text", "whatsapp").await;
    }
  

}



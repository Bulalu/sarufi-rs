#[cfg_attr(test, macro_use)]
extern crate log;

use reqwest::{Client, ClientBuilder, header::HeaderMap, Response};
use serde::de::DeserializeOwned;

pub use errors::ApiError;
pub use bot::{Bot, BotResponse};
use serde_json::{json, Map, Value};
use std::collections::HashMap;
use dotenv::dotenv;


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

        /// Returns a bot object for the given id
        // pub async fn get_bot(&self, id: usize) -> Result<Bot, ApiError> {
        //     let url = utils::api_url(&format!("/chatbot/{}", id));
        //     let response = self.client.get(&url).send().await?;
            
            
             
        //     if response.status().is_success() {
        //         let result = response.json::<Bot>().await?;
        //         // println!("BOT Response: {:?}", response.text().await?);

        //         Ok(result)
        //     } else {
        //         let error = response.json::<SarufiApiError>().await?;
        //         Err(ApiError::GenericError(error.message()))
        //     }
            
            
        // }
        pub async fn get_bot(&self, id: usize) -> Result<Bot, ApiError> {
            let url = utils::api_url(&format!("/chatbot/{}", id));
            let response = self.client.get(&url).send().await?;
            
            
             
            if response.status().is_success() {
                let mut result = response.json::<Bot>().await?;
                if let Some(metrics) = result.evaluation_metrics {
                    
                    result.evaluation_metrics = Some(metrics);
                }
                // let json_string = response.text().await?;
                // let json_value: Value = serde_json::from_str(&json_string).unwrap();
                // let pretty_json = serde_json::to_string_pretty(&json_value).unwrap();
                // println!("{}", pretty_json);

                // println!("BOT Response: {:?}", response.text().await?);

                Ok(result)
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
                if let Some(metrics) = result.evaluation_metrics {
                    
                    result.evaluation_metrics = Some(metrics);
                }
               
                Ok(result)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
            


           
        }

        async fn parse_result<R>(&self, response: Response) -> Result<R, ApiError> 
            where R: DeserializeOwned
          {
            if response.status().is_success() {
                
              let result = response.json::<R>().await?;
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
    use super::*;

    #[tokio::test]
    async fn test_api_url() {
        dotenv().ok();
        let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
        let api = SarufiAPI::new(api_key).unwrap();
        let bot = api.get_bot(1045).await.unwrap();
        println!("Result: {:?}", bot);
    }

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

        println!("Result: {:?}", bot.id);
        println!("Name: {:?}", bot.name);
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

        // let prev_bot = api.get_bot(id).await.unwrap();
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


    }
}



use reqwest::{Client, ClientBuilder, header::HeaderMap};

pub use errors::ApiError;
pub use bot::{Bot};
use serde_json::{ Value};
use std::{collections::HashMap};
use std::fs::File;
use std::io::BufReader;

mod errors;
mod utils;
mod api;
mod bot;
mod test;

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

    
    pub async fn get_bot(&self, id: usize) -> Result<Bot, ApiError> {
            let url = utils::api_url(&format!("/chatbot/{}", id));
            
            let response = self.client.get(&url).send().await?;

            if response.status().is_success() {
                let  result = response.json::<Bot>().await?;
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
                let result = response.json::<Vec<Bot>>().await?;
                Ok(result)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
  
        }
     
        pub async fn _fetch_response(&self, bot_id: usize, chat_id: &str, message: &str, message_type: &str, channel: &str) -> Result<String, ApiError> {
            let _url = utils::api_url("/conversation");
        
            if  channel == "whatsapp" {
                let _url = utils::api_url("/conversation/whatsapp");  
            }

            let mut data = HashMap::new();
            data.insert("bot_id".to_owned(), Value::Number(serde_json::Number::from(bot_id)));
            data.insert("chat_id".to_owned(), Value::String(chat_id.to_owned()));
            data.insert("message".to_owned(), Value::String(message.to_owned()));
            data.insert("message_type".to_owned(), Value::String(message_type.to_owned()));
            data.insert("channel".to_owned(), Value::String(channel.to_owned()));
        
            let response = self.client.post(&_url).json(&Value::Object(data.into_iter().collect())).send().await?;
        
            if response.status().is_success() {
               
                let json_string = response.text().await.unwrap();
                let json_value: Value = serde_json::from_str(&json_string).unwrap();
                let result = & json_value["message"][0];
               
                Ok(result.to_string())
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
        }
        
        pub async fn chat(&self, bot_id: usize) -> Result<String, ApiError> {
            let chat_id = utils::generate_uuid().to_string();
            println!("Chat ID: {:?}", chat_id);
            let message = "Hello";
            let message_type = "text";
            let channel = "general";

            let response = self._fetch_response(bot_id, &chat_id, message, message_type, channel).await.unwrap();

            Ok(response)
        }

        pub async fn chat_status(&self, bot_id: usize, chat_id: &str) -> Result<String, ApiError> {
            let url = utils::api_url("/allchannels/status");
        
            let mut data = HashMap::new();
            data.insert("bot_id".to_owned(), Value::Number(serde_json::Number::from(bot_id)));
            data.insert("chat_id".to_owned(), Value::String(chat_id.to_owned()));
        
            let response = self.client.post(&url).json(&Value::Object(data.into_iter().collect())).send().await?;
        
            if response.status().is_success() {
                let json_string = response.text().await.unwrap();
                Ok(json_string)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
            }
        }

        pub async fn update_conversation_state(&self, bot_id: usize, chat_id: &str, next_state: &str) -> Result<String, ApiError> {

            let url = utils::api_url("/conversation-state");
        
            let mut data = HashMap::new();
            data.insert("bot_id".to_owned(), Value::Number(serde_json::Number::from(bot_id)));
            data.insert("chat_id".to_owned(), Value::String(chat_id.to_owned()));
            data.insert("next_state".to_owned(), Value::String(next_state.to_owned()));

            let response = self.client.post(&url).json(&Value::Object(data.into_iter().collect())).send().await?;

            if response.status().is_success() {
                let json_string = response.text().await.unwrap();
                // let json_value: Value = serde_json::from_str(&json_string).unwrap();
                // let result = & json_value["message"][0];
               
                Ok(json_string)
            } else {
                let error = response.json::<SarufiApiError>().await?;
                Err(ApiError::GenericError(error.message()))
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

    
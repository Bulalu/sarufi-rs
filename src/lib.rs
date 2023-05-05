#[cfg_attr(test, macro_use)]
extern crate log;

use reqwest::{Client, ClientBuilder, header::HeaderMap, Response};
use serde::de::DeserializeOwned;

pub use errors::ApiError;
pub use bot::Bot;
use serde_json::{json, Map, Value};
use std::collections::HashMap;

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
        pub async fn get_bot(&self, id: usize) -> Result<Bot, ApiError> {
            let url = utils::api_url(&format!("/chatbot/{}", id));
            let response = self.client.get(&url).send().await?;
            println!("Response: {:?}", response);
            
            self.parse_result(response).await?
            
            
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
            // println!("URL: {:?}", url);
            // let mut payload = Map::new();
            // payload.insert("name".to_string(), json!(name));
            // payload.insert("description".to_string(), json!(description));
            // payload.insert("industry".to_string(), json!(industry));
            // payload.insert("flow".to_string(), json!(flow));
            // payload.insert("intents".to_string(), json!(intents));
            // payload.insert("webhook_url".to_string(), json!(webhook_url));
            // payload.insert("webhook_trigger_intents".to_string(), json!(webhook_trigger_intents));
            // payload.insert("visible_on_community".to_string(), json!(visible_on_community));

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
        
            // println!("Payload: {:?}", payload);

            let response = self.client.post(&url).json(&Value::Object(data.into_iter().collect())).send().await?;
            // println!("Response: {:?}", response);
            self.parse_result(response).await?
            


           
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

}

    
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_url() {
        let api = SarufiAPI::new("").unwrap();
        let bot = api.get_bot(1045).await.unwrap();
        println!("Result: {:?}", bot);
    }

    #[tokio::test]
    async fn test_create_bot() -> Result<(), ApiError> {
        // let api_key = std::env::var("SARUFI_API_KEY").expect("API_KEY env required to run test");
        let api = SarufiAPI::new("af88e925a9c16f42e4da4d2d6b7b13ac619aaa5477066d6ae933dd057c0e08ea").unwrap();
  
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

        Ok(())

       
    }
}


      // let mut flow = HashMap::new();
        // flow.insert("start".to_owned(), Value::String("Hello".to_owned()));
        // flow.insert("end".to_owned(), Value::String("Bye".to_owned()));

        // let mut intents = HashMap::new();
        // intents.insert("greeting".to_owned(), vec!["hi".to_owned(), "hello".to_owned()]);
        // intents.insert("goodbye".to_owned(), vec!["bye".to_owned(), "goodbye".to_owned()]);

        // let webhook_trigger_intents = vec!["greeting".to_owned(), "goodbye".to_owned()];

        // let bot = api.create_bot(
        //     "Test Bot",
        //     Some("This is a test bot"),
        //     Some("Test Industry"),
        //     Some(flow),
        //     Some(intents),
        //     Some("http://example.com/webhook"),
        //     Some(webhook_trigger_intents),
        //     Some(true),
        // ).await?;
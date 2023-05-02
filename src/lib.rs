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
            let mut payload = Map::new();
            payload.insert("name".to_string(), json!(name));
            payload.insert("description".to_string(), json!(description));
            payload.insert("industry".to_string(), json!(industry));
            payload.insert("flow".to_string(), json!(flow));
            payload.insert("intents".to_string(), json!(intents));
            payload.insert("webhook_url".to_string(), json!(webhook_url));
            payload.insert("webhook_trigger_intents".to_string(), json!(webhook_trigger_intents));
            payload.insert("visible_on_community".to_string(), json!(visible_on_community));

            let response = self.client.post(&url).json(&payload).send().await?;
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
}
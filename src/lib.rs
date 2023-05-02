#[cfg_attr(test, macro_use)]
extern crate log;

use reqwest::{Client, ClientBuilder, header::HeaderMap, Response};
use serde::de::DeserializeOwned;

pub use errors::ApiError;
pub use bot::Bot;
use dotenv::dotenv;
use std::env;

mod errors;
mod utils;
mod api;
mod bot;

use crate::api::SarufiApiError;



/// API struct. Exposes function to interact with the Sarufi API 🥷
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

    #[test]
    fn test_api_url() {
        assert_eq!(utils::api_url("/chatbot"), "https://developers.sarufi.io/chatbot");
    }
}

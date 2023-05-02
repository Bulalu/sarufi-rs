use reqwest::{Client, ClientBuilder, header::HeaderMap, Response};

pub use errors::ApiError;
use dotenv::dotenv;
use std::env;

mod errors;
mod utils;


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

    /// Returns a list of all bots created by the user
    pub fn get_bots(&self) -> Result<Response, ApiError> {
        let url = utils::api_url("/bots");
        let resp = self.client.get(&url).send()?;
        Ok(resp)
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

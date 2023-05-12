use crate::errors::{ApiError};
use uuid::Uuid;

static BASE_URL: &'static str = "https://developers.sarufi.io";

/// Checks to ensure keys are not empty
pub(crate) fn validate_keys(api_key: &str) -> Result<(), ApiError> {
  if api_key.is_empty() {
    Err(ApiError::InvalidApiKey())?
  }

  Ok(())
}

pub(crate) fn api_url(path: &str) -> String {
  format!("{}{}", BASE_URL, path)
}



pub(crate) fn generate_uuid() -> String {
  let uuid = Uuid::new_v4();
  uuid.to_string()
}


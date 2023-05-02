use serde::Deserialize;

#[derive(Deserialize)]
/// Error response structure from sarufi api
pub(crate) struct SarufiApiError {
  error: String
}

impl SarufiApiError {
    pub fn message(&self) -> String {
      self.error.clone()
    }
}

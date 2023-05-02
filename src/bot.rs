use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::{json, Map, Value};




#[derive(Debug, Serialize, Deserialize)]
pub struct Bot {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: String,
    pub intents: HashMap<String, Vec<String>>,
    pub flows: HashMap<String, serde_json::Value>,
    pub model_name: String,
    pub confidence_threshold: f64,
    pub evaluation_metrics: HashMap<String, f64>,
    pub industry: String,
    pub language: String,
    pub visible_on_community: bool,
    pub webhook_url: String,
    pub webhook_trigger_intents: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}


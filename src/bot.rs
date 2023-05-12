use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Debug, Serialize, Deserialize)]
pub struct Bot{
    pub id: usize,
    pub confidence_threshold: Option<f64>,
    pub user_id: u64,
    pub name: String,
    pub description: String,
    pub intents: HashMap<String, Vec<String>>,
    pub flows: HashMap<String, serde_json::Value>,
    pub model_name: String,
    pub evaluation_metrics: Option<EvaluationMetrics>,
    pub industry: String,
    pub language: String,
    pub visible_on_community: bool,
    pub webhook_url: String,
    pub webhook_trigger_intents: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EvaluationMetrics {
    metrics: Option<Metrics>,
    model_type: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Metrics {
    classification_report: ClassificationReport,
    model_metrics: ModelMetrics,
}

#[derive(Debug, Deserialize, Serialize)]
struct ClassificationMetrics {
  
    precision: f64,
    recall: f64,
    support: usize,
    #[serde(rename = "f1-score")]
    f1_score: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClassificationReport {
    accuracy: f64,
    #[serde(flatten)]
    categories: HashMap<String, ClassificationMetrics>,
}


#[derive(Debug, Deserialize, Serialize)]
struct ModelMetrics {
    accuracy: f64,
    error_rate: f64,
    recall: f64,
}
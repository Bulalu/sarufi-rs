use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::{json, Map, Value};




#[derive(Deserialize, Debug)]
pub struct Bot {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub description: String,
    pub intents: HashMap<String, Vec<String>>,
    pub flows: HashMap<String, serde_json::Value>,
    pub model_name: String,
    pub confidence_threshold: f64,
    pub evaluation_metrics: EvaluationMetrics,
    pub industry: String,
    pub language: String,
    pub visible_on_community: bool,
    pub webhook_url: String,
    pub webhook_trigger_intents: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug)]
pub struct EvaluationMetrics {
    pub metrics: Metrics,
    pub model_type: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct Metrics {
    pub model_metrics: ModelMetrics,
    pub classification_report: ClassificationReport,
    pub accuracy: f64,
    pub macro_avg: MacroAvg,
    pub weighted_avg: WeightedAvg,
}

#[derive(Deserialize, Debug)]
pub struct ModelMetrics {
    pub accuracy: f64,
    pub recall: f64,
    pub error_rate: f64,
}

#[derive(Deserialize, Debug)]
pub struct ClassificationReport {
    pub bye: ReportItem,
    pub greetings: ReportItem,
    pub thanks: ReportItem,
}

#[derive(Deserialize, Debug)]
pub struct ReportItem {
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub support: u64,
}

#[derive(Deserialize, Debug)]
pub struct MacroAvg {
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub support: u64,
}

#[derive(Deserialize, Debug)]
pub struct WeightedAvg {
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub support: u64,
}


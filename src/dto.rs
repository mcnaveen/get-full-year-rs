use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct YearResponseDTO {
    pub year: u32,
    pub sponsored_by: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

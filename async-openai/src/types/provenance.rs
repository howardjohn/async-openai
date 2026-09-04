use crate::types::InputSource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateContentProvenanceRequest {
    /// The image or audio file to check for supported OpenAI provenance signals.
    pub file: InputSource,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceDetectionResult {
    Detected,
    NotDetected,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum C2paValidationState {
    Trusted,
    Valid,
    Invalid,
    NotPresent,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct C2paProvenanceResult {
    pub outcome: ProvenanceDetectionResult,
    pub validation_state: C2paValidationState,
    pub issuer: Option<String>,
    pub model: Option<String>,
    pub generated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SynthIdProvenanceResult {
    pub outcome: ProvenanceDetectionResult,
    pub model: Option<String>,
    pub generated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ProvenanceResult {
    C2pa(C2paProvenanceResult),
    Synthid(SynthIdProvenanceResult),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ProvenanceResource {
    pub object: String,
    pub created_at: u64,
    pub results: Vec<ProvenanceResult>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SafetyAlertErrorType {
    PotentiallyUnintendedDataTransfer,
    PotentiallyUnintendedDataAccess,
    PotentiallyUnintendedDestructiveActivity,
    Other,
}

/// An approved safety alert for an API project or enterprise workspace.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SafetyAlertResource {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub request_id: String,
    pub response_id: String,
    pub model: String,
    pub request_paused: bool,
    pub error_type: SafetyAlertErrorType,
    pub reason: Option<String>,
}

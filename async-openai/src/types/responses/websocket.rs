use serde::{Deserialize, Serialize};

use crate::types::responses::{CreateResponse, InputItem, ResponseStreamEvent};

/// Input queued to steer an active response over a Responses WebSocket connection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResponseSteerInput {
    Text(String),
    Items(Vec<InputItem>),
}

/// Starts a response on a Responses WebSocket connection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponsesClientEventResponseCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    #[serde(flatten)]
    pub response: CreateResponse,
}

/// Queues input that changes the direction of an active response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseSteerEvent {
    pub previous_response_id: String,
    pub input: ResponseSteerInput,
}

/// Events sent by a client over a Responses WebSocket connection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ResponsesClientEvent {
    #[serde(rename = "response.create")]
    ResponseCreate(Box<ResponsesClientEventResponseCreate>),
    #[serde(rename = "response.steer")]
    ResponseSteer(ResponseSteerEvent),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ResponseSteerReference {
    pub id: String,
    pub previous_response_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ResponseSteerAcceptedEvent {
    pub sequence_number: u64,
    pub steer: ResponseSteerReference,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseSteerPendingReason {
    WaitingForRequiredInput,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseSteerPendingEvent {
    pub sequence_number: u64,
    pub steer: ResponseSteerReference,
    pub reason: ResponseSteerPendingReason,
    pub required_input: Vec<ResponseSteerRequiredInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseSteerErrorCode {
    ResponseNotFound,
    InvalidInput,
    SteeringNotSupported,
    TooManyPendingSteers,
    ResponseAlreadyCompleted,
    ResponseNotActive,
    SuccessorCreationFailed,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ResponseSteerError {
    pub r#type: ResponseSteerErrorType,
    pub code: ResponseSteerErrorCode,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseSteerErrorType {
    InvalidRequestError,
}

/// A client-owned result or approval that must be supplied before queued steering input can run.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseSteerRequiredInput {
    FunctionCallOutput {
        call_id: String,
        name: String,
    },
    CustomToolCallOutput {
        call_id: String,
    },
    ComputerCallOutput {
        call_id: String,
    },
    ShellCallOutput {
        call_id: String,
    },
    ApplyPatchCallOutput {
        call_id: String,
    },
    ToolSearchOutput {
        call_id: String,
        execution: ResponseSteerToolSearchExecution,
    },
    McpApprovalResponse {
        approval_request_id: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ResponseSteerToolSearchExecution {
    Client,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseSteerFailedSubmission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub previous_response_id: String,
    pub input: ResponseSteerInput,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseSteerFailedEvent {
    pub sequence_number: u64,
    pub steer: ResponseSteerFailedSubmission,
    pub error: ResponseSteerError,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponseWebSocketErrorEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<u64>,
    pub error: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

/// A regular Responses streaming event annotated with its WebSocket lane.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ResponsesWebSocketStreamEvent {
    #[serde(flatten)]
    pub event: ResponseStreamEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ResponsesWebSocketControlEvent {
    #[serde(rename = "response.steer.accepted")]
    ResponseSteerAccepted(ResponseSteerAcceptedEvent),
    #[serde(rename = "response.steer.pending")]
    ResponseSteerPending(ResponseSteerPendingEvent),
    #[serde(rename = "response.steer.failed")]
    ResponseSteerFailed(ResponseSteerFailedEvent),
    #[serde(rename = "error")]
    Error(ResponseWebSocketErrorEvent),
}

/// Events received over a Responses WebSocket connection.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum ResponsesWebSocketServerEvent {
    Control(ResponsesWebSocketControlEvent),
    Stream(Box<ResponsesWebSocketStreamEvent>),
}

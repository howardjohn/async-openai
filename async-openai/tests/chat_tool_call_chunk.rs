#![cfg(feature = "chat-completion-types")]

use async_openai::types::chat::ChatCompletionMessageToolCallChunk;
use serde_json::json;

#[test]
fn streamed_tool_call_without_index() {
    let chunk: ChatCompletionMessageToolCallChunk = serde_json::from_value(json!({
        "id": "call_1",
        "type": "function",
        "function": {"name": "get_weather", "arguments": "{}"}
    }))
    .unwrap();

    assert_eq!(chunk.index, None);
    let value = serde_json::to_value(chunk).unwrap();
    assert!(value.get("index").is_none());
}

#[test]
fn streamed_tool_call_keeps_index() {
    let chunk: ChatCompletionMessageToolCallChunk =
        serde_json::from_value(json!({"index": 1})).unwrap();

    assert_eq!(chunk.index, Some(1));
    assert_eq!(serde_json::to_value(chunk).unwrap()["index"], 1);
}

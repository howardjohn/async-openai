use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct InputTokenDetails {
    /// The number of tokens that were retrieved from the cache.
    /// [More on prompt caching](https://platform.openai.com/docs/guides/prompt-caching).
    #[serde(default)]
    pub cached_tokens: u32,
    /// The number of input tokens that were written to the cache.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens: Option<i64>,
    /// A breakdown of `cache_write_tokens` by cache lifetime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_write_tokens_details: Option<CacheWriteTokenDetails>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CacheWriteTokenDetails {
    /// The number of input tokens written to the cache with a 30-minute lifetime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_30m_tokens: Option<i64>,
    /// The number of input tokens written to the cache with a 12-hour lifetime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_12h_tokens: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct OutputTokenDetails {
    /// The number of reasoning tokens.
    #[serde(default)]
    pub reasoning_tokens: u32,
}

/// Usage statistics for a response.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ResponseUsage {
    /// The number of input tokens.
    pub input_tokens: u32,
    /// A detailed breakdown of the input tokens.
    #[serde(default)]
    pub input_tokens_details: InputTokenDetails,
    /// The number of output tokens.
    pub output_tokens: u32,
    /// A detailed breakdown of the output tokens.
    #[serde(default)]
    pub output_tokens_details: OutputTokenDetails,
    /// The total number of tokens used.
    pub total_tokens: u32,
}

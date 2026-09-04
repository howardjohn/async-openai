use crate::{
    config::Config, error::OpenAIError, types::admin::safety_alerts::SafetyAlertResource, Client,
    RequestOptions,
};

/// Retrieve approved safety alerts.
pub struct SafetyAlerts<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> SafetyAlerts<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, alert_id: &str) -> Result<SafetyAlertResource, OpenAIError> {
        self.client
            .get(&format!("/safety/alerts/{alert_id}"), &self.request_options)
            .await
    }
}

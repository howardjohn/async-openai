use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::spend_limits::{
        OrganizationSpendLimitDeletedResource, OrganizationSpendLimitResource,
        UpdateOrganizationSpendLimitBody,
    },
    Client, RequestOptions,
};

/// Manage the organization's hard spend limit.
pub struct SpendLimits<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> SpendLimits<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<OrganizationSpendLimitResource, OpenAIError> {
        self.client
            .get("/organization/spend_limit", &self.request_options)
            .await
    }

    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: UpdateOrganizationSpendLimitBody,
    ) -> Result<OrganizationSpendLimitResource, OpenAIError> {
        self.client
            .post("/organization/spend_limit", request, &self.request_options)
            .await
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn delete(&self) -> Result<OrganizationSpendLimitDeletedResource, OpenAIError> {
        self.client
            .delete("/organization/spend_limit", &self.request_options)
            .await
    }
}

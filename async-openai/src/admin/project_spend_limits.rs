use crate::{
    config::Config,
    error::OpenAIError,
    types::admin::spend_limits::{
        ProjectSpendLimitDeletedResource, ProjectSpendLimitResource, UpdateProjectSpendLimitBody,
    },
    Client, RequestOptions,
};

/// Manage a project's hard spend limit.
pub struct ProjectSpendLimits<'c, C: Config> {
    client: &'c Client<C>,
    project_id: String,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ProjectSpendLimits<'c, C> {
    pub fn new(client: &'c Client<C>, project_id: &str) -> Self {
        Self {
            client,
            project_id: project_id.into(),
            request_options: RequestOptions::new(),
        }
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self) -> Result<ProjectSpendLimitResource, OpenAIError> {
        self.client
            .get(
                &format!("/organization/projects/{}/spend_limit", self.project_id),
                &self.request_options,
            )
            .await
    }

    #[crate::byot(T0 = serde::Serialize, R = serde::de::DeserializeOwned)]
    pub async fn update(
        &self,
        request: UpdateProjectSpendLimitBody,
    ) -> Result<ProjectSpendLimitResource, OpenAIError> {
        self.client
            .post(
                &format!("/organization/projects/{}/spend_limit", self.project_id),
                request,
                &self.request_options,
            )
            .await
    }

    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn delete(&self) -> Result<ProjectSpendLimitDeletedResource, OpenAIError> {
        self.client
            .delete(
                &format!("/organization/projects/{}/spend_limit", self.project_id),
                &self.request_options,
            )
            .await
    }
}

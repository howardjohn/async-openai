use crate::{
    config::Config,
    error::OpenAIError,
    traits::AsyncTryFrom,
    types::provenance::{CreateContentProvenanceRequest, ProvenanceResource},
    util::create_file_part,
    Client, RequestOptions,
};

pub struct ContentProvenance<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> ContentProvenance<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Checks an image or audio file for supported OpenAI provenance signals.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause = "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>, T0: crate::traits::MaybeSend + 'static"
    )]
    pub async fn check(
        &self,
        request: CreateContentProvenanceRequest,
    ) -> Result<ProvenanceResource, OpenAIError> {
        self.client
            .post_form("/content_provenance_checks", request, &self.request_options)
            .await
    }
}

impl AsyncTryFrom<CreateContentProvenanceRequest> for reqwest::multipart::Form {
    type Error = OpenAIError;

    async fn try_from(request: CreateContentProvenanceRequest) -> Result<Self, Self::Error> {
        Ok(reqwest::multipart::Form::new().part("file", create_file_part(request.file).await?))
    }
}

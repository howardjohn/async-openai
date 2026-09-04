use crate::error::OpenAIError;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum SpendLimitCurrency {
    USD,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitInterval {
    Month,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SpendLimitEnforcementStatus {
    Inactive,
    Enforcing,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SpendLimitEnforcement {
    pub status: SpendLimitEnforcementStatus,
}

/// Parameters for creating or replacing an organization or project hard spend limit.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Builder)]
#[builder(
    name = "UpdateSpendLimitArgs",
    pattern = "mutable",
    setter(into),
    build_fn(error = "OpenAIError")
)]
pub struct UpdateSpendLimitBody {
    /// The hard spend limit amount, in cents.
    pub threshold_amount: u64,
    pub currency: SpendLimitCurrency,
    pub interval: SpendLimitInterval,
}

macro_rules! spend_limit_types {
    ($resource:ident, $deleted:ident) => {
        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
        pub struct $resource {
            pub object: String,
            pub threshold_amount: u64,
            pub currency: SpendLimitCurrency,
            pub interval: SpendLimitInterval,
            pub enforcement: SpendLimitEnforcement,
        }

        #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
        pub struct $deleted {
            pub object: String,
            pub deleted: bool,
        }
    };
}

spend_limit_types!(
    OrganizationSpendLimitResource,
    OrganizationSpendLimitDeletedResource
);
spend_limit_types!(ProjectSpendLimitResource, ProjectSpendLimitDeletedResource);

pub type UpdateOrganizationSpendLimitBody = UpdateSpendLimitBody;
pub type UpdateProjectSpendLimitBody = UpdateSpendLimitBody;

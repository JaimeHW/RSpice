//! The administrator identity recorded on technology registry changes.

use super::{PdkTechnologyError, validate_identifier};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PdkAdministrativeAuthority {
    pub actor_id: String,
    pub authority_id: String,
}

impl PdkAdministrativeAuthority {
    pub fn validate(&self) -> Result<(), PdkTechnologyError> {
        validate_identifier("authority.actor_id", &self.actor_id)?;
        validate_identifier("authority.authority_id", &self.authority_id)
    }
}

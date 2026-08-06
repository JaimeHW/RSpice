use std::fmt;

use rspice_cloud_contract::WorkspaceInvitationRole;
use serde::{Serialize, Serializer};

use crate::InvitationToken;

/// Borrowed, client-committed workspace-invitation creation command.
///
/// Construct this only after atomically persisting the raw invitation token,
/// idempotency key, email, role, optional expiry, and delivery state. The wire
/// representation contains only the token's SHA-256 commitment; the raw bearer
/// credential is never serialized into the Cloud API request.
#[derive(Clone, Copy)]
pub struct CreateWorkspaceInvitation<'a> {
    email: &'a str,
    role: WorkspaceInvitationRole,
    expires_at: Option<&'a str>,
    token: InvitationToken<'a>,
}

impl<'a> CreateWorkspaceInvitation<'a> {
    /// Builds a command using the server's bounded default expiration window.
    #[must_use]
    pub const fn new(
        email: &'a str,
        role: WorkspaceInvitationRole,
        token: InvitationToken<'a>,
    ) -> Self {
        Self {
            email,
            role,
            expires_at: None,
            token,
        }
    }

    /// Adds an RFC 3339 expiry retained as part of the exact retry command.
    #[must_use]
    pub const fn with_expires_at(mut self, expires_at: &'a str) -> Self {
        self.expires_at = Some(expires_at);
        self
    }

    pub(crate) const fn email(self) -> &'a str {
        self.email
    }

    pub(crate) const fn role(self) -> WorkspaceInvitationRole {
        self.role
    }

    pub(crate) const fn expires_at(self) -> Option<&'a str> {
        self.expires_at
    }
}

impl Serialize for CreateWorkspaceInvitation<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct WireRequest<'a> {
            email: &'a str,
            role: WorkspaceInvitationRole,
            #[serde(skip_serializing_if = "Option::is_none")]
            expires_at: Option<&'a str>,
            token_sha256: String,
        }

        WireRequest {
            email: self.email,
            role: self.role,
            expires_at: self.expires_at,
            token_sha256: self.token.commitment_sha256(),
        }
        .serialize(serializer)
    }
}

impl fmt::Debug for CreateWorkspaceInvitation<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CreateWorkspaceInvitation")
            .field("email", &"[REDACTED]")
            .field("role", &self.role)
            .field("has_explicit_expiry", &self.expires_at.is_some())
            .field("token", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invitation_commands_serialize_only_the_token_commitment() {
        let raw = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let token = InvitationToken::new(raw).expect("canonical invitation token");
        let command = CreateWorkspaceInvitation::new(
            "Engineer@RSpice.App",
            WorkspaceInvitationRole::Viewer,
            token,
        )
        .with_expires_at("2026-08-01T00:00:00Z");
        let json = serde_json::to_value(command).expect("serialize invitation command");
        assert_eq!(json["email"], "Engineer@RSpice.App");
        assert_eq!(json["role"], "viewer");
        assert_eq!(json["expires_at"], "2026-08-01T00:00:00Z");
        assert_eq!(
            json["token_sha256"],
            "0f007385b6f9d4b7eeb2748605afe1a984a0a3bfa3f014d09e2a784ce9e5cd1a"
        );
        assert!(!json.to_string().contains(raw));

        let debug = format!("{command:?}");
        assert!(!debug.contains(raw));
        assert!(!debug.contains("Engineer@RSpice.App"));
        assert!(debug.matches("[REDACTED]").count() >= 2);

        let default_expiry = CreateWorkspaceInvitation::new(
            "engineer@rspice.test",
            WorkspaceInvitationRole::Viewer,
            token,
        );
        let json = serde_json::to_value(default_expiry).expect("serialize default expiry");
        assert!(json.get("expires_at").is_none());
    }
}

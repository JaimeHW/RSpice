use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceRole {
    Owner,
    Administrator,
    Editor,
    Viewer,
}

impl WorkspaceRole {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Owner => "owner",
            Self::Administrator => "administrator",
            Self::Editor => "editor",
            Self::Viewer => "viewer",
        }
    }

    #[must_use]
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "owner" => Some(Self::Owner),
            "administrator" => Some(Self::Administrator),
            "editor" => Some(Self::Editor),
            "viewer" => Some(Self::Viewer),
            _ => None,
        }
    }

    #[must_use]
    pub const fn can_view(self) -> bool {
        true
    }

    #[must_use]
    pub const fn can_edit_circuits(self) -> bool {
        matches!(self, Self::Owner | Self::Administrator | Self::Editor)
    }

    #[must_use]
    pub const fn can_manage_workspace(self) -> bool {
        matches!(self, Self::Owner | Self::Administrator)
    }

    #[must_use]
    pub const fn can_manage_owners(self) -> bool {
        matches!(self, Self::Owner)
    }

    /// Whether this actor can assign `desired` to a member who does not yet
    /// belong to the workspace.
    #[must_use]
    pub const fn can_assign_member(self, desired: Self) -> bool {
        match self {
            Self::Owner => true,
            Self::Administrator => matches!(desired, Self::Editor | Self::Viewer),
            Self::Editor | Self::Viewer => false,
        }
    }

    /// Whether this actor can change an existing member from `current` to
    /// `desired`. The last-owner invariant is enforced by persistence code.
    #[must_use]
    pub const fn can_change_member(self, current: Self, desired: Self) -> bool {
        match self {
            Self::Owner => true,
            Self::Administrator => {
                matches!(current, Self::Editor | Self::Viewer)
                    && matches!(desired, Self::Editor | Self::Viewer)
            }
            Self::Editor | Self::Viewer => false,
        }
    }

    /// Whether this actor can remove an existing member. The last-owner
    /// invariant is enforced by persistence code.
    #[must_use]
    pub const fn can_remove_member(self, current: Self) -> bool {
        match self {
            Self::Owner => true,
            Self::Administrator => matches!(current, Self::Editor | Self::Viewer),
            Self::Editor | Self::Viewer => false,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CircuitVisibility {
    Private,
    Unlisted,
    Public,
}

impl CircuitVisibility {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::Unlisted => "unlisted",
            Self::Public => "public",
        }
    }

    #[must_use]
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "private" => Some(Self::Private),
            "unlisted" => Some(Self::Unlisted),
            "public" => Some(Self::Public),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SharePermission {
    View,
}

impl SharePermission {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::View => "view",
        }
    }

    #[must_use]
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "view" => Some(Self::View),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementStatus {
    Pending,
    Active,
    GracePeriod,
    Suspended,
    Expired,
    Revoked,
}

impl EntitlementStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Active => "active",
            Self::GracePeriod => "grace_period",
            Self::Suspended => "suspended",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
        }
    }

    #[must_use]
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "pending" => Some(Self::Pending),
            "active" => Some(Self::Active),
            "grace_period" => Some(Self::GracePeriod),
            "suspended" => Some(Self::Suspended),
            "expired" => Some(Self::Expired),
            "revoked" => Some(Self::Revoked),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SimulationRunStatus {
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}

impl SimulationRunStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }

    #[must_use]
    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "queued" => Some(Self::Queued),
            "running" => Some(Self::Running),
            "succeeded" => Some(Self::Succeeded),
            "failed" => Some(Self::Failed),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed | Self::Cancelled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wire_names_are_stable_snake_case_values() {
        assert_eq!(
            serde_json::to_string(&CircuitVisibility::Unlisted).expect("serialize"),
            "\"unlisted\""
        );
        assert_eq!(
            serde_json::to_string(&EntitlementStatus::GracePeriod).expect("serialize"),
            "\"grace_period\""
        );
        assert_eq!(
            serde_json::to_string(&SharePermission::View).expect("serialize"),
            "\"view\""
        );
        assert!(SharePermission::from_db("fork").is_none());
        assert!(SharePermission::from_db("edit").is_none());
    }

    #[test]
    fn workspace_role_management_preserves_administrative_boundaries() {
        assert!(
            WorkspaceRole::Owner
                .can_change_member(WorkspaceRole::Administrator, WorkspaceRole::Owner)
        );
        assert!(
            WorkspaceRole::Administrator
                .can_change_member(WorkspaceRole::Editor, WorkspaceRole::Viewer)
        );
        assert!(
            !WorkspaceRole::Administrator
                .can_change_member(WorkspaceRole::Administrator, WorkspaceRole::Viewer)
        );
        assert!(!WorkspaceRole::Editor.can_assign_member(WorkspaceRole::Viewer));
    }
}

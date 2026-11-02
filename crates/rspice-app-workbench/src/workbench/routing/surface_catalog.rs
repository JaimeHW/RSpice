//! Canonical GUI surface identities and immutable design metadata.
//!
//! Identity, labels, archetypes, tiers, and deep links begin as a Rust
//! projection of `mockups/rspice-workbench-host/implementation/surface-registry.json`,
//! less the specialist designs no route in this build opens.
//! Release status is deliberately fail-closed against the executable Rust
//! route registry: a design-specification release target remains Preview here
//! until a complete route owner is registered. Catalog presence alone is not
//! an implementation, engine, platform, entitlement, or readiness claim.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use rspice_app_state::workbench::workspace::Workspace;

/// Canonical composition family for a GUI surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SurfaceArchetype {
    PrimaryWorkspace,
    SpecialistWorkspace,
    Manager,
    Modal,
    Overlay,
    Internal,
}

impl SurfaceArchetype {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryWorkspace => "primary-workspace",
            Self::SpecialistWorkspace => "specialist-workspace",
            Self::Manager => "manager",
            Self::Modal => "modal",
            Self::Overlay => "overlay",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for SurfaceArchetype {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Canonical design tier from the governed surface registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CanonicalTier {
    ReleaseTarget,
    Preview,
    Internal,
}

impl CanonicalTier {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseTarget => "release-target",
            Self::Preview => "preview",
            Self::Internal => "internal",
        }
    }
}

impl fmt::Display for CanonicalTier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Commercial release classification from the governed surface registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReleaseStatus {
    ReleaseScope,
    Preview,
    InternalOnly,
}

impl ReleaseStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseScope => "release-scope",
            Self::Preview => "preview",
            Self::InternalOnly => "internal-only",
        }
    }
}

impl fmt::Display for ReleaseStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Immutable metadata for one exact canonical surface identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceMetadata {
    pub id: SurfaceId,
    pub stable_id: &'static str,
    pub label: &'static str,
    pub archetype: SurfaceArchetype,
    pub canonical_tier: CanonicalTier,
    pub release_status: ReleaseStatus,
    pub deep_link: &'static str,
}

macro_rules! define_surface_catalog {
    (
        $(
            $variant:ident => {
                id: $id:literal,
                label: $label:literal,
                archetype: $archetype:ident,
                tier: $tier:ident,
                status: $status:ident,
                deep_link: $deep_link:literal
            }
        ),+ $(,)?
    ) => {
        /// Stable identity for every canonical GUI surface in registry order.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum SurfaceId {
            $($variant),+
        }

        impl SurfaceId {
            /// Every canonical surface in exact registry order.
            #[cfg(any(test, feature = "test-support"))]
            pub const ALL: [Self; 27] = [$(Self::$variant),+];

            #[must_use]
            pub const fn metadata(self) -> SurfaceMetadata {
                match self {
                    $(
                        Self::$variant => SurfaceMetadata {
                            id: Self::$variant,
                            stable_id: $id,
                            label: $label,
                            archetype: SurfaceArchetype::$archetype,
                            canonical_tier: CanonicalTier::$tier,
                            release_status: ReleaseStatus::$status,
                            deep_link: $deep_link,
                        }
                    ),+
                }
            }

            #[must_use]
            pub const fn as_str(self) -> &'static str {
                self.metadata().stable_id
            }

            #[must_use]
            pub const fn label(self) -> &'static str {
                self.metadata().label
            }

            #[must_use]
            pub const fn archetype(self) -> SurfaceArchetype {
                self.metadata().archetype
            }

            #[must_use]
            #[cfg(test)]
            pub const fn canonical_tier(self) -> CanonicalTier {
                self.metadata().canonical_tier
            }

            #[must_use]
            #[cfg(test)]
            pub const fn release_status(self) -> ReleaseStatus {
                self.metadata().release_status
            }

            /// Exact base deep link from the registry. Primary workspaces use
            /// `?view=...`; every other canonical surface uses `?surface=...`.
            #[must_use]
            pub const fn deep_link(self) -> &'static str {
                self.metadata().deep_link
            }

            /// Map a canonical primary surface to its workbench selector.
            /// Specialist and supporting surfaces return `None`; use
            /// [`Self::owner_workspace`] for the render-owner projection of a
            /// persistent non-primary document.
            #[must_use]
            pub const fn workspace(self) -> Option<Workspace> {
                match self {
                    Self::Project => Some(Workspace::Project),
                    Self::Design => Some(Workspace::Design),
                    Self::Simulate => Some(Workspace::Simulate),
                    Self::Stimulus => Some(Workspace::Stimulus),
                    Self::Results => Some(Workspace::Results),
                    Self::Verify => Some(Workspace::Verify),
                    Self::Models => Some(Workspace::Models),
                    Self::Netlist => Some(Workspace::Netlist),
                    _ => None,
                }
            }

            /// Workspace that owns the persistent document rendered for this
            /// route. This is deliberately separate from [`Self::workspace`]:
            /// a specialist route keeps its canonical `?surface=` identity
            /// while borrowing the owning workspace's chrome, dock snapshot,
            /// shortcuts, and document context.
            #[must_use]
            pub const fn owner_workspace(self) -> Option<Workspace> {
                match self {
                    Self::DesignManagement => Some(Workspace::Design),
                    Self::ModelEditor
                    | Self::ModelCorrelation
                    | Self::LibraryCellviewManager => Some(Workspace::Models),
                    _ => self.workspace(),
                }
            }

            #[must_use]
            pub const fn from_workspace(workspace: Workspace) -> Self {
                match workspace {
                    Workspace::Project => Self::Project,
                    Workspace::Design => Self::Design,
                    Workspace::Simulate => Self::Simulate,
                    Workspace::Stimulus => Self::Stimulus,
                    Workspace::Results => Self::Results,
                    Workspace::Verify => Self::Verify,
                    Workspace::Models => Self::Models,
                    Workspace::Netlist => Self::Netlist,
                }
            }
        }

        impl FromStr for SurfaceId {
            type Err = SurfaceIdParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($id => Ok(Self::$variant),)+
                    _ => Err(SurfaceIdParseError::Unknown(value.to_owned())),
                }
            }
        }
    };
}

define_surface_catalog! {
    Project => { id: "project", label: "Project overview", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=project" },
    Design => { id: "design", label: "Design entry", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=design" },
    Simulate => { id: "simulate", label: "Simulation setup", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=simulate" },
    Stimulus => { id: "stimulus", label: "Stimulus Library", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=stimulus" },
    Results => { id: "results", label: "Results", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=results" },
    Verify => { id: "verify", label: "Verification", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=verify" },
    Models => { id: "models", label: "Models and libraries", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=models" },
    Netlist => { id: "netlist", label: "Code and automation", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=netlist" },
    ReleaseCockpit => { id: "release-cockpit", label: "Release closure", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=release-cockpit" },
    ReportAuthoring => { id: "report-authoring", label: "Verification report", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=report-authoring" },
    ModelEditor => { id: "model-editor", label: "Device model editor", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=model-editor" },
    ModelCorrelation => { id: "model-correlation", label: "Measurement correlation", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=model-correlation" },
    LibraryCellviewManager => { id: "library-cellview-manager", label: "Library, cellview, symbol and form authoring", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=library-cellview-manager" },
    ProjectLauncher => { id: "project-launcher", label: "Project launcher", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=project-launcher" },
    Preferences => { id: "preferences", label: "Preferences", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=preferences" },
    DesignManagement => { id: "design-management", label: "Sheets, variants and annotation", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=design-management" },
    AccountOrganization => { id: "account-organization", label: "Account and administration", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=account-organization" },
    CommandPalette => { id: "command-palette", label: "Command palette", archetype: Overlay, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=command-palette" },
    JobsManager => { id: "jobs-manager", label: "Run history", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=jobs-manager" },
    WorkflowDialog => { id: "workflow-dialog", label: "Transactional workflow dialog", archetype: Modal, tier: ReleaseTarget, status: Preview, deep_link: "?surface=workflow-dialog" },
    NotificationCenter => { id: "notification-center", label: "Notifications", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=notification-center" },
    HelpCenter => { id: "help-center", label: "Legal & privacy", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=help-center" },
    ProductReadinessCenter => { id: "product-readiness-center", label: "Product readiness contracts", archetype: Internal, tier: Internal, status: InternalOnly, deep_link: "?surface=product-readiness-center" },
    MobileNavigation => { id: "mobile-navigation", label: "Compact task navigation", archetype: Overlay, tier: ReleaseTarget, status: Preview, deep_link: "?surface=mobile-navigation" },
    MobileTaskHandoff => { id: "mobile-task-handoff", label: "Qualified-target handoff", archetype: Modal, tier: ReleaseTarget, status: Preview, deep_link: "?surface=mobile-task-handoff" },
    SurfaceContextMenu => { id: "surface-context-menu", label: "Selection context menu", archetype: Overlay, tier: ReleaseTarget, status: Preview, deep_link: "?surface=surface-context-menu" },
    DockedTool => { id: "docked-tool", label: "Docked subordinate tool", archetype: Manager, tier: ReleaseTarget, status: Preview, deep_link: "?surface=docked-tool" },
}

impl fmt::Display for SurfaceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl Serialize for SurfaceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for SurfaceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

impl From<Workspace> for SurfaceId {
    fn from(workspace: Workspace) -> Self {
        Self::from_workspace(workspace)
    }
}

impl TryFrom<SurfaceId> for Workspace {
    type Error = NonPrimarySurface;

    fn try_from(surface_id: SurfaceId) -> Result<Self, Self::Error> {
        surface_id
            .workspace()
            .ok_or(NonPrimarySurface { surface_id })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SurfaceIdParseError {
    #[error("unknown surface ID `{0}`")]
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("surface `{surface_id}` is not one of the eight primary workspaces")]
pub struct NonPrimarySurface {
    pub surface_id: SurfaceId,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn catalog_has_exact_count_and_unique_contract_values() {
        assert_eq!(SurfaceId::ALL.len(), 27);

        let ids = SurfaceId::ALL
            .iter()
            .map(|surface| surface.as_str())
            .collect::<HashSet<_>>();
        let labels = SurfaceId::ALL
            .iter()
            .map(|surface| surface.label())
            .collect::<HashSet<_>>();
        let deep_links = SurfaceId::ALL
            .iter()
            .map(|surface| surface.deep_link())
            .collect::<HashSet<_>>();

        assert_eq!(ids.len(), 27);
        assert_eq!(labels.len(), 27);
        assert_eq!(deep_links.len(), 27);
    }

    #[test]
    fn catalog_classification_counts_match_the_registry() {
        let count_status = |status| {
            SurfaceId::ALL
                .iter()
                .filter(|surface| surface.release_status() == status)
                .count()
        };
        assert_eq!(count_status(ReleaseStatus::ReleaseScope), 19);
        assert_eq!(count_status(ReleaseStatus::Preview), 7);
        assert_eq!(count_status(ReleaseStatus::InternalOnly), 1);

        let count_archetype = |archetype| {
            SurfaceId::ALL
                .iter()
                .filter(|surface| surface.archetype() == archetype)
                .count()
        };
        assert_eq!(count_archetype(SurfaceArchetype::PrimaryWorkspace), 8);
        assert_eq!(count_archetype(SurfaceArchetype::SpecialistWorkspace), 5);
        assert_eq!(count_archetype(SurfaceArchetype::Manager), 8);
        assert_eq!(count_archetype(SurfaceArchetype::Modal), 2);
        assert_eq!(count_archetype(SurfaceArchetype::Overlay), 3);
        assert_eq!(count_archetype(SurfaceArchetype::Internal), 1);
    }

    #[test]
    fn models_and_pdk_commercial_tiers_match_the_governed_mockup_contract() {
        assert_eq!(
            SurfaceId::ModelCorrelation.canonical_tier(),
            CanonicalTier::Preview
        );
        assert_eq!(
            SurfaceId::ModelCorrelation.release_status(),
            ReleaseStatus::Preview
        );
        for surface in [SurfaceId::ModelEditor, SurfaceId::LibraryCellviewManager] {
            assert_eq!(surface.canonical_tier(), CanonicalTier::ReleaseTarget);
            assert_eq!(surface.release_status(), ReleaseStatus::ReleaseScope);
        }
    }

    #[test]
    fn every_surface_has_strict_text_and_serde_roundtrips() {
        for surface in SurfaceId::ALL {
            let stable_id = surface.as_str();
            assert_eq!(surface.to_string(), stable_id);
            assert_eq!(stable_id.parse::<SurfaceId>(), Ok(surface));

            let encoded = serde_json::to_string(&surface).expect("surface ID serializes");
            assert_eq!(encoded, format!("\"{stable_id}\""));
            assert_eq!(
                serde_json::from_str::<SurfaceId>(&encoded).expect("surface ID deserializes"),
                surface
            );
        }

        for invalid in [
            "",
            "Project",
            "project ",
            " project",
            "release_cockpit",
            "unknown-surface",
        ] {
            assert!(
                invalid.parse::<SurfaceId>().is_err(),
                "accepted `{invalid}`"
            );
            assert!(
                serde_json::from_str::<SurfaceId>(&format!("\"{invalid}\"")).is_err(),
                "deserialized `{invalid}`"
            );
        }
    }

    #[test]
    fn base_deep_links_preserve_the_exact_view_surface_split() {
        for (index, surface) in SurfaceId::ALL.into_iter().enumerate() {
            let expected = if index < Workspace::ALL.len() {
                format!("?view={surface}")
            } else {
                format!("?surface={surface}")
            };
            assert_eq!(surface.deep_link(), expected);
        }
    }

    #[test]
    fn all_primary_surface_workspace_mappings_roundtrip() {
        let expected = [
            (Workspace::Project, SurfaceId::Project),
            (Workspace::Design, SurfaceId::Design),
            (Workspace::Simulate, SurfaceId::Simulate),
            (Workspace::Stimulus, SurfaceId::Stimulus),
            (Workspace::Results, SurfaceId::Results),
            (Workspace::Verify, SurfaceId::Verify),
            (Workspace::Models, SurfaceId::Models),
            (Workspace::Netlist, SurfaceId::Netlist),
        ];

        assert_eq!(Workspace::ALL.len(), expected.len());
        for (workspace, surface) in expected {
            assert_eq!(SurfaceId::from(workspace), surface);
            assert_eq!(surface.workspace(), Some(workspace));
            assert_eq!(Workspace::try_from(surface), Ok(workspace));
        }

        assert_eq!(
            Workspace::try_from(SurfaceId::ReleaseCockpit),
            Err(NonPrimarySurface {
                surface_id: SurfaceId::ReleaseCockpit,
            })
        );
    }

    #[test]
    fn library_cellview_manager_keeps_specialist_identity_with_models_ownership() {
        assert_eq!(SurfaceId::LibraryCellviewManager.workspace(), None);
        assert_eq!(
            SurfaceId::LibraryCellviewManager.owner_workspace(),
            Some(Workspace::Models)
        );
        assert_eq!(
            Workspace::try_from(SurfaceId::LibraryCellviewManager),
            Err(NonPrimarySurface {
                surface_id: SurfaceId::LibraryCellviewManager,
            })
        );
        assert_eq!(
            SurfaceId::LibraryCellviewManager.deep_link(),
            "?surface=library-cellview-manager"
        );
    }

    #[test]
    fn design_management_keeps_manager_identity_with_design_ownership() {
        assert_eq!(SurfaceId::DesignManagement.workspace(), None);
        assert_eq!(
            SurfaceId::DesignManagement.owner_workspace(),
            Some(Workspace::Design)
        );
        assert_eq!(
            SurfaceId::DesignManagement.label(),
            "Sheets, variants and annotation"
        );
        assert_eq!(
            SurfaceId::DesignManagement.archetype(),
            SurfaceArchetype::Manager
        );
        assert_eq!(
            SurfaceId::DesignManagement.canonical_tier(),
            CanonicalTier::ReleaseTarget
        );
        assert_eq!(
            SurfaceId::DesignManagement.release_status(),
            ReleaseStatus::ReleaseScope
        );
        assert_eq!(
            SurfaceId::DesignManagement.deep_link(),
            "?surface=design-management"
        );
    }
}

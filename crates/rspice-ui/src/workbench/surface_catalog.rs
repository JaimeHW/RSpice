//! Canonical GUI surface identities and immutable design metadata.
//!
//! Values in this module are a direct Rust projection of
//! `mockups/rspice-workbench-host/implementation/surface-registry.json`.
//! They describe product/design identity only; catalog presence is not an
//! implementation, engine, platform, entitlement, or release-readiness claim.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::state::Workspace;

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
    QualifiedExternalFirst,
    Internal,
}

impl CanonicalTier {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseTarget => "release-target",
            Self::Preview => "preview",
            Self::QualifiedExternalFirst => "qualified-external-first",
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
    ExternalFirst,
    InternalOnly,
}

impl ReleaseStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseScope => "release-scope",
            Self::Preview => "preview",
            Self::ExternalFirst => "external-first",
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
            pub const ALL: [Self; 63] = [$(Self::$variant),+];

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
            pub const fn canonical_tier(self) -> CanonicalTier {
                self.metadata().canonical_tier
            }

            #[must_use]
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
                    Self::VisualizationStudio => Some(Workspace::Results),
                    _ => self.workspace(),
                }
            }

            #[must_use]
            pub const fn from_workspace(workspace: Workspace) -> Self {
                match workspace {
                    Workspace::Project => Self::Project,
                    Workspace::Design => Self::Design,
                    Workspace::Simulate => Self::Simulate,
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
    Results => { id: "results", label: "Results", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=results" },
    Verify => { id: "verify", label: "Verification", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=verify" },
    Models => { id: "models", label: "Models and libraries", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=models" },
    Netlist => { id: "netlist", label: "Code and automation", archetype: PrimaryWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?view=netlist" },
    ReleaseCockpit => { id: "release-cockpit", label: "Release closure · RC-19", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=release-cockpit" },
    MultiTestStudio => { id: "multi-test-studio", label: "AFE release verification", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=multi-test-studio" },
    AmsWorkbench => { id: "ams-workbench", label: "AMS · mixed-signal verification", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=ams-workbench" },
    RfWorkbench => { id: "rf-workbench", label: "RF · receiver and PA verification", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=rf-workbench" },
    HighSigmaWorkbench => { id: "high-sigma-workbench", label: "High-sigma variation", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=high-sigma-workbench" },
    LayoutWorkbench => { id: "layout-workbench", label: "top · physical layout", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=layout-workbench" },
    LvsPexWorkbench => { id: "lvs-pex-workbench", label: "LVS and parasitic extraction", archetype: SpecialistWorkspace, tier: QualifiedExternalFirst, status: ExternalFirst, deep_link: "?surface=lvs-pex-workbench" },
    EmWorkbench => { id: "em-workbench", label: "EM, IR and electrothermal", archetype: SpecialistWorkspace, tier: QualifiedExternalFirst, status: ExternalFirst, deep_link: "?surface=em-workbench" },
    SiPiWorkbench => { id: "si-pi-workbench", label: "SI, PI and SerDes", archetype: SpecialistWorkspace, tier: QualifiedExternalFirst, status: ExternalFirst, deep_link: "?surface=si-pi-workbench" },
    ReliabilityWorkbench => { id: "reliability-workbench", label: "Reliability, fault and SOA", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=reliability-workbench" },
    VisualizationStudio => { id: "visualization-studio", label: "Visualization Studio", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=visualization-studio" },
    DigitalEventWorkbench => { id: "digital-event-workbench", label: "Digital & AMS event viewer", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=digital-event-workbench" },
    RfDataDisplayWorkbench => { id: "rf-data-display-workbench", label: "RF data display worksheet", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=rf-data-display-workbench" },
    MeasurementInterchangeWorkbench => { id: "measurement-interchange-workbench", label: "Measurement data interchange", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=measurement-interchange-workbench" },
    StatisticalVisualizationWorkbench => { id: "statistical-visualization-workbench", label: "Statistical visualization laboratory", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=statistical-visualization-workbench" },
    ReportAuthoring => { id: "report-authoring", label: "Verification report · RC-19", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=report-authoring" },
    ModelEditor => { id: "model-editor", label: "Device model editor", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=model-editor" },
    PcellDesigner => { id: "pcell-designer", label: "Parameterized-cell designer", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=pcell-designer" },
    ModelExtraction => { id: "model-extraction", label: "Model extraction · BSIM4 candidate", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=model-extraction" },
    ModelCorrelation => { id: "model-correlation", label: "Measurement correlation · OPA189_A", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=model-correlation" },
    JobDiagnostics => { id: "job-diagnostics", label: "Failure diagnostics · Run 40", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=job-diagnostics" },
    RegressionOrchestration => { id: "regression-orchestration", label: "Regression orchestration · main", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=regression-orchestration" },
    ApiBrowser => { id: "api-browser", label: "Automation API browser", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=api-browser" },
    FastspiceWorkbench => { id: "fastspice-workbench", label: "FastSPICE · full-chip and memory", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=fastspice-workbench" },
    PhotonicsWorkbench => { id: "photonics-workbench", label: "Electronic-photonic co-design", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=photonics-workbench" },
    HeterogeneousWorkbench => { id: "heterogeneous-workbench", label: "Heterogeneous assembly · AFE module", archetype: SpecialistWorkspace, tier: QualifiedExternalFirst, status: ExternalFirst, deep_link: "?surface=heterogeneous-workbench" },
    LibraryCharacterization => { id: "library-characterization", label: "Cell-library characterization", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=library-characterization" },
    PdkTechnologyAdmin => { id: "pdk-technology-admin", label: "PDK technology administration", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=pdk-technology-admin" },
    PowerElectronicsWorkbench => { id: "power-electronics-workbench", label: "Power electronics design center", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=power-electronics-workbench" },
    InstrumentWorkbench => { id: "instrument-workbench", label: "Measurement and calibration hub", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=instrument-workbench" },
    PcbWorkbench => { id: "pcb-workbench", label: "PCB and laminate authoring", archetype: SpecialistWorkspace, tier: QualifiedExternalFirst, status: ExternalFirst, deep_link: "?surface=pcb-workbench" },
    CliBatchWorkbench => { id: "cli-batch-workbench", label: "CLI, batch and reproducible execution", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=cli-batch-workbench" },
    DesignMigrationWorkbench => { id: "design-migration-workbench", label: "Process migration and layout reuse", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=design-migration-workbench" },
    RfApplicationWorkbench => { id: "rf-application-workbench", label: "RF application synthesis and characterization", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=rf-application-workbench" },
    CosimulationWorkbench => { id: "cosimulation-workbench", label: "System co-simulation and external runtimes", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=cosimulation-workbench" },
    ComplianceLabWorkbench => { id: "compliance-lab-workbench", label: "High-speed compliance laboratory", archetype: SpecialistWorkspace, tier: QualifiedExternalFirst, status: ExternalFirst, deep_link: "?surface=compliance-lab-workbench" },
    QuantumWorkbench => { id: "quantum-workbench", label: "Quantum electronics design center", archetype: SpecialistWorkspace, tier: Preview, status: Preview, deep_link: "?surface=quantum-workbench" },
    SolverQualificationCenter => { id: "solver-qualification-center", label: "Simulator qualification center", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=solver-qualification-center" },
    DeckCompatibilityCenter => { id: "deck-compatibility-center", label: "Netlist compatibility and migration", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=deck-compatibility-center" },
    ProtectedIpCenter => { id: "protected-ip-center", label: "Protected model and IP execution", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=protected-ip-center" },
    LibraryCellviewManager => { id: "library-cellview-manager", label: "Library, cellview, symbol and form authoring", archetype: SpecialistWorkspace, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=library-cellview-manager" },
    ProjectLauncher => { id: "project-launcher", label: "Project launcher", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=project-launcher" },
    Preferences => { id: "preferences", label: "Preferences", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=preferences" },
    AccountOrganization => { id: "account-organization", label: "Account and administration", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=account-organization" },
    CommandPalette => { id: "command-palette", label: "Command palette", archetype: Overlay, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=command-palette" },
    JobsManager => { id: "jobs-manager", label: "Jobs, targets and run history", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=jobs-manager" },
    WorkflowDialog => { id: "workflow-dialog", label: "Transactional workflow dialog", archetype: Modal, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=workflow-dialog" },
    NotificationCenter => { id: "notification-center", label: "Notifications and activity", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=notification-center" },
    HelpCenter => { id: "help-center", label: "Help center", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=help-center" },
    FeatureAvailability => { id: "feature-availability", label: "Capability and platform matrix", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=feature-availability" },
    ProductReadinessCenter => { id: "product-readiness-center", label: "Product readiness contracts", archetype: Internal, tier: Internal, status: InternalOnly, deep_link: "?surface=product-readiness-center" },
    SpecialistToolBrowser => { id: "specialist-tool-browser", label: "Specialist tool browser", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=specialist-tool-browser" },
    MobileNavigation => { id: "mobile-navigation", label: "Compact task navigation", archetype: Overlay, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=mobile-navigation" },
    MobileTaskHandoff => { id: "mobile-task-handoff", label: "Qualified-target handoff", archetype: Modal, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=mobile-task-handoff" },
    SurfaceContextMenu => { id: "surface-context-menu", label: "Selection context menu", archetype: Overlay, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=surface-context-menu" },
    DockedTool => { id: "docked-tool", label: "Docked subordinate tool", archetype: Manager, tier: ReleaseTarget, status: ReleaseScope, deep_link: "?surface=docked-tool" },
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
    #[error("unknown canonical surface ID `{0}`")]
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("surface `{surface_id}` is not one of the seven primary workspaces")]
pub struct NonPrimarySurface {
    pub surface_id: SurfaceId,
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn catalog_has_exact_count_and_unique_contract_values() {
        assert_eq!(SurfaceId::ALL.len(), 63);

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

        assert_eq!(ids.len(), 63);
        assert_eq!(labels.len(), 63);
        assert_eq!(deep_links.len(), 63);
    }

    #[test]
    fn catalog_classification_counts_match_the_registry() {
        let count_status = |status| {
            SurfaceId::ALL
                .iter()
                .filter(|surface| surface.release_status() == status)
                .count()
        };
        assert_eq!(count_status(ReleaseStatus::ReleaseScope), 46);
        assert_eq!(count_status(ReleaseStatus::Preview), 10);
        assert_eq!(count_status(ReleaseStatus::ExternalFirst), 6);
        assert_eq!(count_status(ReleaseStatus::InternalOnly), 1);

        let count_archetype = |archetype| {
            SurfaceId::ALL
                .iter()
                .filter(|surface| surface.archetype() == archetype)
                .count()
        };
        assert_eq!(count_archetype(SurfaceArchetype::PrimaryWorkspace), 7);
        assert_eq!(count_archetype(SurfaceArchetype::SpecialistWorkspace), 41);
        assert_eq!(count_archetype(SurfaceArchetype::Manager), 9);
        assert_eq!(count_archetype(SurfaceArchetype::Modal), 2);
        assert_eq!(count_archetype(SurfaceArchetype::Overlay), 3);
        assert_eq!(count_archetype(SurfaceArchetype::Internal), 1);
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
    fn visualization_studio_keeps_specialist_identity_with_results_ownership() {
        assert_eq!(SurfaceId::VisualizationStudio.workspace(), None);
        assert_eq!(
            SurfaceId::VisualizationStudio.owner_workspace(),
            Some(Workspace::Results)
        );
        assert_eq!(
            Workspace::try_from(SurfaceId::VisualizationStudio),
            Err(NonPrimarySurface {
                surface_id: SurfaceId::VisualizationStudio,
            })
        );
        assert_eq!(
            SurfaceId::VisualizationStudio.deep_link(),
            "?surface=visualization-studio"
        );
    }
}

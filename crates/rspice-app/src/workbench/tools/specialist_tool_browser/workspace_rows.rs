//! The specialist-workspace catalog the tool browser searches.
//!
//! One row per specialist engineering workspace: its owner, purpose, tier and
//! commercial module. Whether a row can open is never stored here; it is asked
//! of the route registry at the point of use.

use crate::workbench::routing::availability::{SurfaceExecutionAvailability, surface_availability};
use crate::workbench::routing::surface_catalog::{CanonicalTier, SurfaceId};
use crate::workbench::state::{EngineeringProfile, Workspace};

/// Commercial module disposition from the governed surface registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommercialModuleAvailability {
    Included,
    OptionalModule,
    ExternalIntegration,
    EnterpriseControlled,
    ResearchPreview,
}

impl CommercialModuleAvailability {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Included => "included",
            Self::OptionalModule => "optional-module",
            Self::ExternalIntegration => "external-integration",
            Self::EnterpriseControlled => "enterprise-controlled",
            Self::ResearchPreview => "research-preview",
        }
    }
}

/// One owned specialist engineering workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpecialistWorkspaceRow {
    pub surface_id: SurfaceId,
    pub owner: Workspace,
    pub purpose: &'static str,
    pub tier: CanonicalTier,
    pub primary_module_id: &'static str,
    pub module_availability: CommercialModuleAvailability,
}

impl SpecialistWorkspaceRow {
    #[must_use]
    pub const fn label(self) -> &'static str {
        self.surface_id.label()
    }

    /// Whether the selected engineering profile includes this workspace
    /// in everyday navigation. Profile visibility is discoverability only and
    /// never changes route execution availability.
    #[must_use]
    pub const fn shown_in_profile(self, profile: EngineeringProfile) -> bool {
        match profile {
            EngineeringProfile::AnalogIc => matches!(
                self.surface_id,
                SurfaceId::ReleaseCockpit
                    | SurfaceId::MultiTestStudio
                    | SurfaceId::AmsWorkbench
                    | SurfaceId::RfWorkbench
                    | SurfaceId::HighSigmaWorkbench
                    | SurfaceId::DeviceSafetyWorkbench
                    | SurfaceId::DigitalEventWorkbench
                    | SurfaceId::StatisticalVisualizationWorkbench
                    | SurfaceId::ReportAuthoring
                    | SurfaceId::ModelEditor
                    | SurfaceId::ModelExtraction
                    | SurfaceId::ModelCorrelation
                    | SurfaceId::JobDiagnostics
                    | SurfaceId::RegressionOrchestration
                    | SurfaceId::ApiBrowser
                    | SurfaceId::LibraryCharacterization
                    | SurfaceId::PdkTechnologyAdmin
                    | SurfaceId::CliBatchWorkbench
                    | SurfaceId::SolverQualificationCenter
                    | SurfaceId::DeckCompatibilityCenter
                    | SurfaceId::ProtectedIpCenter
                    | SurfaceId::LibraryCellviewManager
            ),
            EngineeringProfile::RfMicrowave => matches!(
                self.surface_id,
                SurfaceId::ReleaseCockpit
                    | SurfaceId::MultiTestStudio
                    | SurfaceId::RfWorkbench
                    | SurfaceId::RfApplicationWorkbench
                    | SurfaceId::HighSigmaWorkbench
                    | SurfaceId::DeviceSafetyWorkbench
                    | SurfaceId::RfDataDisplayWorkbench
                    | SurfaceId::StatisticalVisualizationWorkbench
                    | SurfaceId::ReportAuthoring
                    | SurfaceId::ModelEditor
                    | SurfaceId::ModelExtraction
                    | SurfaceId::ModelCorrelation
                    | SurfaceId::JobDiagnostics
                    | SurfaceId::RegressionOrchestration
                    | SurfaceId::ApiBrowser
                    | SurfaceId::PdkTechnologyAdmin
                    | SurfaceId::CliBatchWorkbench
                    | SurfaceId::SolverQualificationCenter
                    | SurfaceId::DeckCompatibilityCenter
                    | SurfaceId::ProtectedIpCenter
                    | SurfaceId::LibraryCellviewManager
            ),
            EngineeringProfile::SiPi => matches!(
                self.surface_id,
                SurfaceId::ReleaseCockpit
                    | SurfaceId::MultiTestStudio
                    | SurfaceId::DeviceSafetyWorkbench
                    | SurfaceId::ReportAuthoring
                    | SurfaceId::JobDiagnostics
                    | SurfaceId::RegressionOrchestration
                    | SurfaceId::ApiBrowser
                    | SurfaceId::PdkTechnologyAdmin
                    | SurfaceId::CliBatchWorkbench
                    | SurfaceId::SolverQualificationCenter
                    | SurfaceId::DeckCompatibilityCenter
                    | SurfaceId::ProtectedIpCenter
                    | SurfaceId::LibraryCellviewManager
            ),
            EngineeringProfile::Power => matches!(
                self.surface_id,
                SurfaceId::ReleaseCockpit
                    | SurfaceId::MultiTestStudio
                    | SurfaceId::DeviceSafetyWorkbench
                    | SurfaceId::StatisticalVisualizationWorkbench
                    | SurfaceId::ReportAuthoring
                    | SurfaceId::ModelEditor
                    | SurfaceId::ModelExtraction
                    | SurfaceId::ModelCorrelation
                    | SurfaceId::JobDiagnostics
                    | SurfaceId::RegressionOrchestration
                    | SurfaceId::ApiBrowser
                    | SurfaceId::CliBatchWorkbench
                    | SurfaceId::SolverQualificationCenter
                    | SurfaceId::DeckCompatibilityCenter
                    | SurfaceId::ProtectedIpCenter
                    | SurfaceId::LibraryCellviewManager
            ),
            EngineeringProfile::Emerging => matches!(
                self.surface_id,
                SurfaceId::ReleaseCockpit
                    | SurfaceId::MultiTestStudio
                    | SurfaceId::DeviceSafetyWorkbench
                    | SurfaceId::ReportAuthoring
                    | SurfaceId::ModelEditor
                    | SurfaceId::ModelExtraction
                    | SurfaceId::JobDiagnostics
                    | SurfaceId::RegressionOrchestration
                    | SurfaceId::ApiBrowser
                    | SurfaceId::PdkTechnologyAdmin
                    | SurfaceId::CliBatchWorkbench
                    | SurfaceId::SolverQualificationCenter
                    | SurfaceId::DeckCompatibilityCenter
                    | SurfaceId::ProtectedIpCenter
                    | SurfaceId::LibraryCellviewManager
            ),
            EngineeringProfile::All => true,
        }
    }

    /// Engine/service boundary for the workspace's tier. It is a qualification
    /// requirement, not a statement that the capability exists.
    #[must_use]
    pub const fn engine_service_boundary(self) -> &'static str {
        match self.tier {
            CanonicalTier::QualifiedExternalFirst => {
                "qualified external first \u{00b7} native roadmap"
            }
            CanonicalTier::Preview => "preview / qualification required",
            CanonicalTier::ReleaseTarget => "release target \u{00b7} qualification required",
            CanonicalTier::Internal => "internal-only design contract",
        }
    }

    /// Runtime execution is delegated to the sole fail-closed route registry;
    /// no catalog or module metadata can make a route executable by itself.
    #[must_use]
    pub const fn runtime_availability(self) -> SurfaceExecutionAvailability {
        surface_availability(self.surface_id)
    }
}

pub const SPECIALIST_WORKSPACE_ROWS: [SpecialistWorkspaceRow; 40] = [
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ReleaseCockpit,
        owner: Workspace::Verify,
        purpose: "Freeze, package, approve, promote, supersede, revoke, and roll back release candidates without overriding source-owned gates.",
        tier: CanonicalTier::Preview,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::MultiTestStudio,
        owner: Workspace::Verify,
        purpose: "Compose versioned multi-test verification plans and coverage stages without duplicating simulation-plan options.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::AmsWorkbench,
        owner: Workspace::Simulate,
        purpose: "Define mixed-language elaboration, connect rules, power intent, UVM context, coverage, and mixed-domain result review.",
        tier: CanonicalTier::Preview,
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RfWorkbench,
        owner: Workspace::Simulate,
        purpose: "Configure periodic, nonlinear, network, modulated-signal, and RF verification tasks with explicit engine availability.",
        tier: CanonicalTier::Preview,
        primary_module_id: "rf-periodic",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::HighSigmaWorkbench,
        owner: Workspace::Verify,
        purpose: "Specify rare-event estimators, confidence, convergence, diagnostics, and the tail samples behind each estimate.",
        tier: CanonicalTier::Preview,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LayoutWorkbench,
        owner: Workspace::Design,
        purpose: "Author hierarchical custom layout with geometry, connectivity, constraints, and in-design checks.",
        tier: CanonicalTier::Preview,
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LvsPexWorkbench,
        owner: Workspace::Verify,
        purpose: "Configure, launch, reconcile, and review LVS and extraction results, keeping track of which tool produced each one.",
        tier: CanonicalTier::QualifiedExternalFirst,
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::EmWorkbench,
        owner: Workspace::Verify,
        purpose: "Prepare field-solver structures, ports, meshing, calibration, coupled analyses, and results tied to the solver that made them.",
        tier: CanonicalTier::QualifiedExternalFirst,
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::SiPiWorkbench,
        owner: Workspace::Verify,
        purpose: "Define channel, PDN, SerDes, compliance, margin, and cross-domain verification workflows.",
        tier: CanonicalTier::QualifiedExternalFirst,
        primary_module_id: "pcb-si-pi",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::DeviceSafetyWorkbench,
        owner: Workspace::Verify,
        purpose: "Configure SOA checks, inspect device stress and rule violations, and cross-probe their circuit locations.",
        tier: CanonicalTier::Preview,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::DigitalEventWorkbench,
        owner: Workspace::Results,
        purpose: "Inspect digital, analog, protocol, assertion, and mixed-domain events with linked navigation to each event's time.",
        tier: CanonicalTier::Preview,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RfDataDisplayWorkbench,
        owner: Workspace::Results,
        purpose: "Author equation-driven RF worksheets as result documents with network-aware viewers and units.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "rf-periodic",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::MeasurementInterchangeWorkbench,
        owner: Workspace::Results,
        purpose: "Import, map, calibrate, de-embed, compare, and publish reviewed measurement datasets.",
        tier: CanonicalTier::Preview,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::StatisticalVisualizationWorkbench,
        owner: Workspace::Results,
        purpose: "Explore saved statistical samples, filters, correlations, outliers, and result views without changing the yield results.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ReportAuthoring,
        owner: Workspace::Results,
        purpose: "Compose traceable review documents from saved results and hand finished documents to Release Closure.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ModelEditor,
        owner: Workspace::Models,
        purpose: "Author typed model parameters, provenance, limits, variants, tests, and qualified revisions.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PcellDesigner,
        owner: Workspace::Models,
        purpose: "Design typed parameterized geometry, terminals, interactions, tests, signing, and migrations.",
        tier: CanonicalTier::Preview,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ModelExtraction,
        owner: Workspace::Models,
        purpose: "Fit versioned model candidates to measured data with bounds, objectives, diagnostics, and retained histories.",
        tier: CanonicalTier::Preview,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ModelCorrelation,
        owner: Workspace::Models,
        purpose: "Correlate model predictions and qualified measurements with explicit metrics, uncertainty, and source bindings.",
        tier: CanonicalTier::Preview,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::JobDiagnostics,
        owner: Workspace::Simulate,
        purpose: "Inspect the saved residual, timestep, matrix, device, log, checkpoint, and target data of a failed run.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RegressionOrchestration,
        owner: Workspace::Netlist,
        purpose: "Orchestrate stable plan and command references, dependencies, retries, and artifact routing without copying setup.",
        tier: CanonicalTier::Preview,
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ApiBrowser,
        owner: Workspace::Netlist,
        purpose: "Discover typed automation objects, members, examples, permissions, compatibility, and sandbox behavior.",
        tier: CanonicalTier::Preview,
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::FastspiceWorkbench,
        owner: Workspace::Simulate,
        purpose: "Specify partitioning, capacity, accuracy, equivalence, failure, and full-chip execution contracts.",
        tier: CanonicalTier::Preview,
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PhotonicsWorkbench,
        owner: Workspace::Design,
        purpose: "Co-design electrical and optical sources, models, geometry, ports, co-simulation, and optical result documents.",
        tier: CanonicalTier::Preview,
        primary_module_id: "emerging-domains",
        module_availability: CommercialModuleAvailability::ResearchPreview,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::HeterogeneousWorkbench,
        owner: Workspace::Design,
        purpose: "Author cross-domain assembly hierarchy, stacks, connectivity, partitions, extraction, and sign-off handoffs.",
        tier: CanonicalTier::QualifiedExternalFirst,
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LibraryCharacterization,
        owner: Workspace::Models,
        purpose: "Define cell sets, arcs, templates, conditions, execution, failures, formats, and publication candidates.",
        tier: CanonicalTier::Preview,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PdkTechnologyAdmin,
        owner: Workspace::Models,
        purpose: "Administer signed technology packages, layer resources, stream maps, recognition, extraction, trust, and audit.",
        tier: CanonicalTier::Preview,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PowerElectronicsWorkbench,
        owner: Workspace::Simulate,
        purpose: "Design converter topologies, switching plans, device and magnetic models, control, thermal, and EMI studies.",
        tier: CanonicalTier::Preview,
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::InstrumentWorkbench,
        owner: Workspace::Results,
        purpose: "Configure safe instrument sessions, calibration, acquisition, uncertainty, de-embedding, and reviewed publication.",
        tier: CanonicalTier::Preview,
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PcbWorkbench,
        owner: Workspace::Design,
        purpose: "Author stackups, constraints, routing, extraction, verification, and manufacturing handoff with explicit units.",
        tier: CanonicalTier::QualifiedExternalFirst,
        primary_module_id: "pcb-si-pi",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::CliBatchWorkbench,
        owner: Workspace::Netlist,
        purpose: "Build, validate, schedule, observe, and replay batch commands and frozen manifests just as they first ran.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::DesignMigrationWorkbench,
        owner: Workspace::Design,
        purpose: "Dry-run and review technology, device, geometry, electrical, verification, and rollback migrations.",
        tier: CanonicalTier::Preview,
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RfApplicationWorkbench,
        owner: Workspace::Simulate,
        purpose: "Guide RF topology synthesis, matching, compression, stability, modulated-signal, and release characterization.",
        tier: CanonicalTier::Preview,
        primary_module_id: "rf-periodic",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::CosimulationWorkbench,
        owner: Workspace::Simulate,
        purpose: "Define participants, interfaces, synchronization, transport, failure isolation, and parity checks.",
        tier: CanonicalTier::Preview,
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ComplianceLabWorkbench,
        owner: Workspace::Verify,
        purpose: "Bind standards, fixtures, execution matrices, measurements, margins, and signed compliance reports.",
        tier: CanonicalTier::QualifiedExternalFirst,
        primary_module_id: "pcb-si-pi",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::QuantumWorkbench,
        owner: Workspace::Simulate,
        purpose: "Specify quantum devices, eigenmodes, participation, Hamiltonians, controls, uncertainty, and calibration data.",
        tier: CanonicalTier::Preview,
        primary_module_id: "emerging-domains",
        module_availability: CommercialModuleAvailability::ResearchPreview,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::SolverQualificationCenter,
        owner: Workspace::Simulate,
        purpose: "Qualify numerical engines, analyses, models, targets, platforms, tolerances, and reproducibility independently.",
        tier: CanonicalTier::Internal,
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::DeckCompatibilityCenter,
        owner: Workspace::Netlist,
        purpose: "Inspect what each deck statement means, mappings, differences, equivalence checks, and reversible migration without rewriting input decks.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ProtectedIpCenter,
        owner: Workspace::Models,
        purpose: "Review package trust, entitlement, target eligibility, redaction, diagnostics, retention, and protected execution policy.",
        tier: CanonicalTier::Preview,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LibraryCellviewManager,
        owner: Workspace::Models,
        purpose: "Author libraries, cells, views, symbols, pins, forms, inheritance, locks, revisions, and publication handoffs.",
        tier: CanonicalTier::ReleaseTarget,
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::workbench::routing::surface_catalog::SurfaceArchetype;

    #[test]
    fn one_row_per_specialist_workspace_and_runtime_asks_the_registry() {
        let surfaces = SPECIALIST_WORKSPACE_ROWS
            .iter()
            .map(|row| row.surface_id)
            .collect::<HashSet<_>>();
        assert_eq!(surfaces.len(), SPECIALIST_WORKSPACE_ROWS.len());
        let specialist = SurfaceId::ALL
            .into_iter()
            .filter(|surface| surface.archetype() == SurfaceArchetype::SpecialistWorkspace)
            .collect::<HashSet<_>>();
        assert_eq!(surfaces, specialist);
        for row in SPECIALIST_WORKSPACE_ROWS {
            assert_eq!(
                row.runtime_availability(),
                surface_availability(row.surface_id),
                "workspace data may not infer a route executor"
            );
            assert!(row.shown_in_profile(EngineeringProfile::All));
        }
    }
}

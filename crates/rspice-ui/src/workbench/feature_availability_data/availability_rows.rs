//! The analysis and specialist-workspace availability tables.
//!
//! These rows are the authored record of what the product currently supports,
//! and each states its evidence tier rather than a bare yes/no — "available"
//! here always means available on a named platform, at a named tier, with the
//! evidence that tier requires. A row is added only when that evidence exists,
//! which is why this is data and not a computed projection.

use super::*;

pub const ANALYSIS_AVAILABILITY_ROWS: [AnalysisAvailabilityRow; 34] = [
    AnalysisAvailabilityRow {
        id: "op",
        code: "OP",
        title: "Operating point",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::OperatingPoint),
    },
    AnalysisAvailabilityRow {
        id: "tran",
        code: "TRAN",
        title: "Transient",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Transient),
    },
    AnalysisAvailabilityRow {
        id: "ac",
        code: "AC",
        title: "AC response",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Ac),
    },
    AnalysisAvailabilityRow {
        id: "dc",
        code: "DC",
        title: "DC sweep",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::DcSweep),
    },
    AnalysisAvailabilityRow {
        id: "noise",
        code: "NOISE",
        title: "Noise",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Noise),
    },
    AnalysisAvailabilityRow {
        id: "pz",
        code: "PZ",
        title: "Pole-zero",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::PoleZero),
    },
    AnalysisAvailabilityRow {
        id: "sens",
        code: "SENS",
        title: "Sensitivity",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Sensitivity),
    },
    AnalysisAvailabilityRow {
        id: "stb",
        code: "STB",
        title: "Loop stability",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Stb),
    },
    AnalysisAvailabilityRow {
        id: "xf",
        code: "XF",
        title: "Transfer function",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::TransferFunction),
    },
    AnalysisAvailabilityRow {
        id: "pss",
        code: "PSS",
        title: "Periodic steady state",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Pss),
    },
    AnalysisAvailabilityRow {
        id: "qpss",
        code: "QPSS",
        title: "Quasi-periodic steady state",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Qpss),
    },
    AnalysisAvailabilityRow {
        id: "hb",
        code: "HB",
        title: "Harmonic balance",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::HarmonicBalance),
    },
    AnalysisAvailabilityRow {
        id: "sp",
        code: "SP",
        title: "S-parameters",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::SParameter),
    },
    AnalysisAvailabilityRow {
        id: "hbsp",
        code: "HBSP",
        title: "Large-signal S-parameters",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Hbsp),
    },
    AnalysisAvailabilityRow {
        id: "hbnoise",
        code: "HBNOISE",
        title: "Harmonic-balance noise",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Hbnoise),
    },
    AnalysisAvailabilityRow {
        id: "envelope",
        code: "ENV",
        title: "Envelope",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Envelope),
    },
    AnalysisAvailabilityRow {
        id: "pac",
        code: "PAC",
        title: "Periodic AC",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Pac),
    },
    AnalysisAvailabilityRow {
        id: "pnoise",
        code: "PNOISE",
        title: "Periodic noise",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Pnoise),
    },
    AnalysisAvailabilityRow {
        id: "pxf",
        code: "PXF",
        title: "Periodic transfer",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Pxf),
    },
    AnalysisAvailabilityRow {
        id: "pstb",
        code: "PSTB",
        title: "Periodic stability",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Pstb),
    },
    AnalysisAvailabilityRow {
        id: "psp",
        code: "PSP",
        title: "Periodic S-parameters",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Psp),
    },
    AnalysisAvailabilityRow {
        id: "qpac",
        code: "QPAC",
        title: "Quasi-periodic AC",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Qpac),
    },
    AnalysisAvailabilityRow {
        id: "qpnoise",
        code: "QPNOISE",
        title: "Quasi-periodic noise",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Qpnoise),
    },
    AnalysisAvailabilityRow {
        id: "qpxf",
        code: "QPXF",
        title: "Quasi-periodic transfer",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Qpxf),
    },
    AnalysisAvailabilityRow {
        id: "tnoise",
        code: "TNOISE",
        title: "Transient noise",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::TransientNoise),
    },
    AnalysisAvailabilityRow {
        id: "mc",
        code: "MC",
        title: "Monte Carlo",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::MonteCarlo),
    },
    AnalysisAvailabilityRow {
        id: "temp",
        code: "TEMP",
        title: "Temperature sweep",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Temperature),
    },
    AnalysisAvailabilityRow {
        id: "corner",
        code: "CORNER",
        title: "Process corners",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Corner),
    },
    AnalysisAvailabilityRow {
        id: "dcmatch",
        code: "DCMATCH",
        title: "DC mismatch contribution",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::DcMismatch),
    },
    AnalysisAvailabilityRow {
        id: "fourier",
        code: "FOUR",
        title: "Fourier measurements",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Fourier),
    },
    AnalysisAvailabilityRow {
        id: "disto",
        code: "DISTO",
        title: "Distortion compatibility",
        intended_tier: AnalysisIntendedTier::CompatibilityPath,
        analysis_kind: Some(AnalysisKind::Disto),
    },
    AnalysisAvailabilityRow {
        id: "reliability",
        code: "REL",
        title: "Reliability & aging",
        intended_tier: AnalysisIntendedTier::PreviewNonSignOff,
        analysis_kind: Some(AnalysisKind::Reliability),
    },
    AnalysisAvailabilityRow {
        id: "soa",
        code: "SOA",
        title: "Safe operating area",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Soa),
    },
    AnalysisAvailabilityRow {
        id: "opt",
        code: "OPT",
        title: "Optimization",
        intended_tier: AnalysisIntendedTier::ReleaseTarget,
        analysis_kind: Some(AnalysisKind::Optimization),
    },
];

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
    pub evidence_role: &'static str,
    pub primary_module_id: &'static str,
    pub module_availability: CommercialModuleAvailability,
}

impl SpecialistWorkspaceRow {
    #[must_use]
    pub const fn label(self) -> &'static str {
        self.surface_id.label()
    }

    /// Whether the selected mockup engineering profile includes this workspace
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
                    | SurfaceId::ReliabilityWorkbench
                    | SurfaceId::VisualizationStudio
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
                    | SurfaceId::ReliabilityWorkbench
                    | SurfaceId::VisualizationStudio
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
                    | SurfaceId::ReliabilityWorkbench
                    | SurfaceId::VisualizationStudio
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
                    | SurfaceId::ReliabilityWorkbench
                    | SurfaceId::VisualizationStudio
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
                    | SurfaceId::ReliabilityWorkbench
                    | SurfaceId::VisualizationStudio
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

    /// Exact engine/service boundary copy used by the mockup matrix. It is a
    /// qualification requirement, not a statement that the capability exists.
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

pub const SPECIALIST_WORKSPACE_ROWS: [SpecialistWorkspaceRow; 41] = [
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ReleaseCockpit,
        owner: Workspace::Verify,
        purpose: "Freeze, package, approve, promote, supersede, revoke, and roll back release candidates without overriding source-owned gates.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::MultiTestStudio,
        owner: Workspace::Verify,
        purpose: "Compose versioned multi-test verification plans and coverage stages without duplicating simulation-plan options.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::AmsWorkbench,
        owner: Workspace::Simulate,
        purpose: "Define mixed-language elaboration, connect rules, power intent, UVM context, coverage, and mixed-domain result review.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RfWorkbench,
        owner: Workspace::Simulate,
        purpose: "Configure periodic, nonlinear, network, modulated-signal, and RF verification tasks with explicit engine availability.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "rf-periodic",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::HighSigmaWorkbench,
        owner: Workspace::Verify,
        purpose: "Specify rare-event estimators, confidence, convergence, diagnostics, and source-owned tail evidence.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LayoutWorkbench,
        owner: Workspace::Design,
        purpose: "Author hierarchical custom layout with exact geometry, connectivity, constraints, and in-design checks.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LvsPexWorkbench,
        owner: Workspace::Verify,
        purpose: "Configure, launch, reconcile, and review LVS and extraction evidence while preserving producer identity.",
        tier: CanonicalTier::QualifiedExternalFirst,
        evidence_role: "producer-bound external evidence",
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::EmWorkbench,
        owner: Workspace::Verify,
        purpose: "Prepare field-solver structures, ports, meshing, calibration, coupled analyses, and producer-bound evidence.",
        tier: CanonicalTier::QualifiedExternalFirst,
        evidence_role: "producer-bound external evidence",
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::SiPiWorkbench,
        owner: Workspace::Verify,
        purpose: "Define channel, PDN, SerDes, compliance, margin, and cross-domain verification workflows.",
        tier: CanonicalTier::QualifiedExternalFirst,
        evidence_role: "producer-bound external evidence",
        primary_module_id: "pcb-si-pi",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ReliabilityWorkbench,
        owner: Workspace::Verify,
        purpose: "Own mission profiles, fault campaigns, aging, SOA, safety coverage, and append-only dispositions.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::VisualizationStudio,
        owner: Workspace::Results,
        purpose: "Create persistent dataset-bound result documents and structured engineering viewers without mutating samples.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::DigitalEventWorkbench,
        owner: Workspace::Results,
        purpose: "Inspect digital, analog, protocol, assertion, and mixed-domain events with linked exact-time navigation.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RfDataDisplayWorkbench,
        owner: Workspace::Results,
        purpose: "Author equation-driven RF worksheets as result documents with network-aware viewers and units.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "rf-periodic",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::MeasurementInterchangeWorkbench,
        owner: Workspace::Results,
        purpose: "Import, map, calibrate, de-embed, compare, and publish reviewed measurement datasets.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::StatisticalVisualizationWorkbench,
        owner: Workspace::Results,
        purpose: "Explore immutable statistical samples, filters, correlations, outliers, and evidence views without changing yield truth.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ReportAuthoring,
        owner: Workspace::Results,
        purpose: "Compose traceable review documents from immutable evidence and hand finished artifacts to Release Closure.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ModelEditor,
        owner: Workspace::Models,
        purpose: "Author typed model parameters, provenance, limits, variants, tests, and qualified revisions.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PcellDesigner,
        owner: Workspace::Models,
        purpose: "Design typed parameterized geometry, terminals, interactions, tests, signing, and migrations.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ModelExtraction,
        owner: Workspace::Models,
        purpose: "Fit versioned model candidates to measured data with bounds, objectives, diagnostics, and retained histories.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ModelCorrelation,
        owner: Workspace::Models,
        purpose: "Correlate model predictions and qualified measurements with explicit metrics, uncertainty, and source bindings.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::JobDiagnostics,
        owner: Workspace::Simulate,
        purpose: "Inspect immutable residual, timestep, matrix, device, log, checkpoint, and target evidence for failed execution.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RegressionOrchestration,
        owner: Workspace::Netlist,
        purpose: "Orchestrate stable plan and command references, dependencies, retries, and artifact routing without copying setup.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ApiBrowser,
        owner: Workspace::Netlist,
        purpose: "Discover typed automation objects, members, examples, permissions, compatibility, and sandbox behavior.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::FastspiceWorkbench,
        owner: Workspace::Simulate,
        purpose: "Specify partitioning, capacity, accuracy, equivalence, failure, and full-chip execution contracts.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PhotonicsWorkbench,
        owner: Workspace::Design,
        purpose: "Co-design electrical and optical sources, models, geometry, ports, co-simulation, and optical result documents.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "emerging-domains",
        module_availability: CommercialModuleAvailability::ResearchPreview,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::HeterogeneousWorkbench,
        owner: Workspace::Design,
        purpose: "Author cross-domain assembly hierarchy, stacks, connectivity, partitions, extraction, and sign-off handoffs.",
        tier: CanonicalTier::QualifiedExternalFirst,
        evidence_role: "producer-bound external evidence",
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LibraryCharacterization,
        owner: Workspace::Models,
        purpose: "Define cell sets, arcs, templates, conditions, execution, failures, formats, and publication candidates.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PdkTechnologyAdmin,
        owner: Workspace::Models,
        purpose: "Administer signed technology packages, layer resources, stream maps, recognition, extraction, trust, and audit.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PowerElectronicsWorkbench,
        owner: Workspace::Simulate,
        purpose: "Design converter topologies, switching plans, device and magnetic models, control, thermal, and EMI studies.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "core-analog-ams",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::InstrumentWorkbench,
        owner: Workspace::Results,
        purpose: "Configure safe instrument sessions, calibration, acquisition, uncertainty, de-embedding, and reviewed publication.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "results-verification",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::PcbWorkbench,
        owner: Workspace::Design,
        purpose: "Author stackups, constraints, routing, extraction, verification, and manufacturing handoff with exact units.",
        tier: CanonicalTier::QualifiedExternalFirst,
        evidence_role: "producer-bound external evidence",
        primary_module_id: "pcb-si-pi",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::CliBatchWorkbench,
        owner: Workspace::Netlist,
        purpose: "Build, validate, schedule, observe, and exactly replay typed batch commands and frozen manifests.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::DesignMigrationWorkbench,
        owner: Workspace::Design,
        purpose: "Dry-run and review technology, device, geometry, electrical, verification, and rollback migrations.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "physical-integration",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::RfApplicationWorkbench,
        owner: Workspace::Simulate,
        purpose: "Guide RF topology synthesis, matching, compression, stability, modulated-signal, and release characterization.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "rf-periodic",
        module_availability: CommercialModuleAvailability::OptionalModule,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::CosimulationWorkbench,
        owner: Workspace::Simulate,
        purpose: "Define participants, typed interfaces, synchronization, transport, failure isolation, and parity evidence.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ComplianceLabWorkbench,
        owner: Workspace::Verify,
        purpose: "Bind standards, fixtures, execution matrices, measurements, margins, and signed compliance reports.",
        tier: CanonicalTier::QualifiedExternalFirst,
        evidence_role: "producer-bound external evidence",
        primary_module_id: "pcb-si-pi",
        module_availability: CommercialModuleAvailability::ExternalIntegration,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::QuantumWorkbench,
        owner: Workspace::Simulate,
        purpose: "Specify quantum devices, eigenmodes, participation, Hamiltonians, controls, uncertainty, and calibration evidence.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "emerging-domains",
        module_availability: CommercialModuleAvailability::ResearchPreview,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::SolverQualificationCenter,
        owner: Workspace::Simulate,
        purpose: "Qualify numerical engines, analyses, models, targets, platforms, tolerances, and reproducibility independently.",
        tier: CanonicalTier::Internal,
        evidence_role: "internal qualification authority only",
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::DeckCompatibilityCenter,
        owner: Workspace::Netlist,
        purpose: "Inspect source semantics, mapping, deltas, equivalence evidence, and reversible migration without rewriting input decks.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "enterprise-automation",
        module_availability: CommercialModuleAvailability::EnterpriseControlled,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::ProtectedIpCenter,
        owner: Workspace::Models,
        purpose: "Review package trust, entitlement, target eligibility, redaction, diagnostics, retention, and protected execution policy.",
        tier: CanonicalTier::Preview,
        evidence_role: "non-sign-off until separately qualified",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
    SpecialistWorkspaceRow {
        surface_id: SurfaceId::LibraryCellviewManager,
        owner: Workspace::Models,
        purpose: "Author libraries, cells, views, symbols, pins, forms, inheritance, locks, revisions, and publication handoffs.",
        tier: CanonicalTier::ReleaseTarget,
        evidence_role: "eligible only when all capability gates pass",
        primary_module_id: "models-pdk",
        module_availability: CommercialModuleAvailability::Included,
    },
];

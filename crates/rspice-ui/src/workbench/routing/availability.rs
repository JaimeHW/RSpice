//! Fail-closed route-executor availability for canonical GUI surfaces.
//!
//! Registry presence describes the governed design contract only. It never
//! proves that a Rust executor, engine, entitlement, platform qualification,
//! or sign-off path exists. This module is the single runtime gate between a
//! canonical [`SurfaceId`] and application navigation.

use crate::workbench::{CapabilityWorkflowId, SurfaceId, SurfaceRoute};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceExecutionAvailability {
    /// A complete Rust route owner is registered for this surface. This says
    /// nothing about numerical, platform, release, or sign-off qualification.
    Available {
        executor: &'static str,
        evidence_boundary: &'static str,
    },
    /// The design identity is known but no complete route owner is available.
    Unavailable { reason: &'static str },
}

impl SurfaceExecutionAvailability {
    #[must_use]
    pub const fn can_open(self) -> bool {
        matches!(self, Self::Available { .. })
    }

    #[must_use]
    #[cfg(any(test, target_arch = "wasm32"))]
    pub const fn reason(self) -> Option<&'static str> {
        match self {
            Self::Available { .. } => None,
            Self::Unavailable { reason } => Some(reason),
        }
    }
}

/// Runtime route availability. Keep this match explicit: adding a catalog
/// variant can never accidentally make it executable.
#[must_use]
pub const fn route_availability(route: SurfaceRoute) -> SurfaceExecutionAvailability {
    if route.object_ref().is_some() {
        return SurfaceExecutionAvailability::Unavailable {
            reason: "Object-aware routes remain unavailable until a runtime object resolver and consuming surface executor are registered.",
        };
    }
    if let Some(workflow) = route.capability_workflow_id() {
        return capability_workflow_availability(workflow);
    }
    surface_availability(route.surface_id())
}

/// Nested workflow availability is intentionally exhaustive. A newly added
/// workflow identity must choose an executor boundary here before navigation
/// can expose it; ownership by an available parent surface is never enough.
const fn capability_workflow_availability(
    workflow: CapabilityWorkflowId,
) -> SurfaceExecutionAvailability {
    match workflow {
        CapabilityWorkflowId::GenericMultidimensionalRunsetController
        | CapabilityWorkflowId::SourceLoadPullAnalysis
        | CapabilityWorkflowId::XParameterGeneration
        | CapabilityWorkflowId::ModulatedSignalCharacterization
        | CapabilityWorkflowId::CalibrationDeembeddingPlan
        | CapabilityWorkflowId::StatisticalDistributionAuthoring
        | CapabilityWorkflowId::ErcPerc
        | CapabilityWorkflowId::AntennaChecks
        | CapabilityWorkflowId::DensityFillDfm
        | CapabilityWorkflowId::GdsOasisStreamOut
        | CapabilityWorkflowId::TapeoutAssemblyHandoff => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui governed planned-workflow specification viewer",
            evidence_boundary: "Read-only design inspection is executable; the described engineering capability and operational menu entry remain unavailable.",
        },
        CapabilityWorkflowId::InteroperabilityMatrix | CapabilityWorkflowId::PlatformLifecycle => {
            SurfaceExecutionAvailability::Available {
                executor: "rspice-ui governed capability-contract inspection document",
                evidence_boundary: "Read-only contract inspection and local filtering are executable; displayed format, connector, browser, operating-system, and engine qualification records remain governed reference data rather than runtime availability claims.",
            }
        }
        CapabilityWorkflowId::TouchEditGuide => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui governed touch-editing guidance document",
            evidence_boundary: "Read-only gesture and precision guidance is executable; the document does not mutate a design or assert device-specific platform qualification.",
        },
    }
}

#[must_use]
pub const fn surface_availability(surface: SurfaceId) -> SurfaceExecutionAvailability {
    match surface {
        SurfaceId::Project
        | SurfaceId::Design
        | SurfaceId::Simulate
        | SurfaceId::Results
        | SurfaceId::Verify
        | SurfaceId::Models
        | SurfaceId::Netlist => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui canonical primary workspace router",
            evidence_boundary: "Rust route and interaction executor present; no platform, numerical, release, or sign-off qualification is implied.",
        },
        SurfaceId::ProjectLauncher => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui canonical local project launcher",
            evidence_boundary: "Canonical route, browser history, local create/open/recent-project selection, pinning, recovery comparison and discard, and local safe-mode controls are executable. Remote collaboration, repository cloning, sharing, and archival services are not exposed as available actions.",
        },
        SurfaceId::Preferences => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui canonical Preferences manager",
            evidence_boundary: "Only the currently visible runtime-backed appearance, workspace, autosave, accessibility, capability, and local account/licensing controls are executable; incomplete mockup categories and actions remain omitted rather than inert.",
        },
        SurfaceId::DesignManagement => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui canonical sheet, variant, annotation, and hierarchy manager",
            evidence_boundary: "Versioned project-owned sheet ordering and assignment, immutable variant lineage and comparison, annotation allocation, and hierarchy audits are executable for an editable active schematic; the manager does not claim layout or sign-off authority.",
        },
        SurfaceId::AccountOrganization => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui account, licensing, and cloud session console",
            evidence_boundary: "The verified on-device license, the cloud account session (server-verified principal, entitlements, and license leases when this build carries release-pinned endpoints), and the current build/process are the executable data owners; builds without cloud endpoints and browser sessions report those boundaries explicitly and expose no speculative actions.",
        },
        SurfaceId::JobsManager => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui jobs, targets, and retained run history manager",
            evidence_boundary: "Stable local job/run selection, current controller progress and cancellation, retained target/provenance inspection, and versioned manifest export are executable. Unconfigured remote schedulers, quotas, and checkpoints are not represented as live capabilities.",
        },
        SurfaceId::SpecialistToolBrowser => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui canonical specialist workspace discovery manager",
            evidence_boundary: "Search, profile filtering, personal favorites and pins, device-local recents, canonical metadata inspection, and route availability disclosure are executable. Discoverability never implies that the selected engineering surface, engine, connector, entitlement, platform, or sign-off path is available.",
        },
        SurfaceId::VisualizationStudio => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui persistent Visualization Studio result-document executor",
            evidence_boundary: "Versioned pane composition, native waveform/Bode/spectrum/Smith/table/histogram/eye/pole-zero viewers, exact source-row inspection, cursor/marker/annotation entities, retained-run overlays, display LOD policy, and CSV/Touchstone/PNG export are executable. Catalog viewers without a compatible native renderer or qualified external producer remain explicitly unavailable and never receive fabricated fallback data.",
        },
        SurfaceId::ReportAuthoring => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui project-owned report and datasheet document composer",
            evidence_boundary: "Versioned project-owned report documents, template-seeded outlines, page creation/reordering/paper settings, typed prose and result blocks, live result references that re-resolve against current project evidence, frozen references that capture deterministic authenticated raster artifacts, publication settings, and hardcopy export are executable. Unresolved, stale, or unbound evidence references are disclosed and fail closed rather than rendering fabricated content. The composer discloses release-handoff readiness and routes to Release Closure; it never freezes, approves, promotes, or signs off a release candidate, and it exposes no external publication or distribution service.",
        },
        SurfaceId::ModelEditor => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui governed project-owned device-model definition editor",
            evidence_boundary: "A selected coherent project-owned single-card model can be opened, edited through typed parameter, section, statistical, temperature, and executable qualification contracts, parser-validated, compared with retained releases, and committed as guarded source or evidence-only revisions. Qualification runs execute on the current real Desktop or WebAssembly runtime and assemble parity evidence only from an exact retained pair. Promotion remains fail-closed behind complete source-bound evidence, declarations, compatibility, and independent approvals. Built-in and external sources remain read-only.",
        },
        SurfaceId::ModelCorrelation => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui project-owned measurement-correlation workspace",
            evidence_boundary: "Immutable CSV datasets, exact source and simulation provenance, unit-aware alignment and metrics, retained residuals, append-only outlier dispositions, independent review evidence, and qualification handoff are executable against project history. Correlation evidence never promotes a model or converts a failed qualification vector into a pass.",
        },
        SurfaceId::PdkTechnologyAdmin => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui signed PDK technology-package administrator",
            evidence_boundary: "Exact Ed25519 manifest signatures, artifact digests and sizes, layer-purpose and stream-map completeness, connectivity, callback capabilities, platform contracts, typed recognition and extraction contract completeness, deterministic comparison of currently trusted signed revisions, immutable package-bound personal-device display profiles, immutable installed revisions, current publisher trust, exact active bindings, durable configuration, local administrator-recorded trust-key provisioning and irreversible revocation, and hash-chained trust/install/activation/rollback receipts are executable. Typed SPICE model artifacts are decoded, dependency-closed, process-section-selected, and merged into the simulator's content-addressed sealed model resolver only when an exact project pin resolves to that currently trusted archive; administrative activation alone cannot change a project's executable model authority. Schema-3 signed callback modules are ABI-validated at installation and execute for an exact project pin through a deterministic, fuel-metered, memory-bounded interpreter with no WASI or network surface; canonical active-plan inputs and exact package, artifact, output, target, fuel, operator, plan, project-revision, and receipt-chain identity are retained as verifiable project evidence. No qualified product engine consumes callback-derived metadata, and rule decks are not executed. Display profiles are governed administration data and are not consumed by a production layout renderer; project and organization display-profile repositories are unavailable. The local trust file is not an operating-system or organization trust service, packages cannot mutate pinned projects implicitly, and package validation does not execute or qualify layout recognition, parasitic extraction, PCells, migration, or sign-off.",
        },
        SurfaceId::LibraryCellviewManager => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui project-owned library, cellview, symbol, and component-form authoring workspace",
            evidence_boundary: "The canonical three-column project library browser, guarded cell/view mutations, exact shared selection, authored symbol preview, typed terminal and parameter-form inspection, symbol editor handoff, model-bound symbol creation/import, form editing, edit-lock inspection, audit history, and versioned project-library publication evidence are executable. Legacy or invalid symbol metadata is disclosed and fails closed; the workspace does not claim PCell, layout, extraction, characterization, protected-IP, remote collaboration, or sign-off authority.",
        },
        SurfaceId::NotificationCenter => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui retained notification and activity center",
            evidence_boundary: "Filtering, read state, retention disclosure, notification settings routing, and clearing retained read activity operate on the real device-local activity stream; no external approval or remote background service is inferred.",
        },
        SurfaceId::FeatureAvailability => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui capability and platform matrix manager",
            evidence_boundary: "Read-only disclosure executor present; bundled design fixtures are not product readiness evidence.",
        },
        _ => SurfaceExecutionAvailability::Unavailable {
            reason: "A canonical GUI design exists, but no complete Rust route executor is registered for this surface.",
        },
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("surface `{surface}` is unavailable: {reason}")]
pub struct SurfaceRouteUnavailable {
    pub surface: SurfaceId,
    pub reason: &'static str,
}

pub fn require_available(route: SurfaceRoute) -> Result<(), SurfaceRouteUnavailable> {
    match route_availability(route) {
        SurfaceExecutionAvailability::Available { .. } => Ok(()),
        SurfaceExecutionAvailability::Unavailable { reason } => Err(SurfaceRouteUnavailable {
            surface: route.surface_id(),
            reason,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ObjectRef, ProductObjectKind};
    use uuid::Uuid;

    #[test]
    fn catalog_presence_never_defaults_to_runtime_availability() {
        let available = SurfaceId::ALL
            .into_iter()
            .filter(|surface| surface_availability(*surface).can_open())
            .collect::<Vec<_>>();
        assert_eq!(
            available,
            [
                SurfaceId::Project,
                SurfaceId::Design,
                SurfaceId::Simulate,
                SurfaceId::Results,
                SurfaceId::Verify,
                SurfaceId::Models,
                SurfaceId::Netlist,
                SurfaceId::VisualizationStudio,
                SurfaceId::ReportAuthoring,
                SurfaceId::ModelEditor,
                SurfaceId::ModelCorrelation,
                SurfaceId::PdkTechnologyAdmin,
                SurfaceId::LibraryCellviewManager,
                SurfaceId::ProjectLauncher,
                SurfaceId::Preferences,
                SurfaceId::DesignManagement,
                SurfaceId::AccountOrganization,
                SurfaceId::JobsManager,
                SurfaceId::NotificationCenter,
                SurfaceId::FeatureAvailability,
                SurfaceId::SpecialistToolBrowser,
            ]
        );
    }

    #[test]
    fn specialist_designs_fail_closed_until_an_executor_is_registered() {
        let route = SurfaceRoute::surface(SurfaceId::RfWorkbench);
        let error = require_available(route).expect_err("RF route is not implemented");
        assert_eq!(error.surface, SurfaceId::RfWorkbench);
        assert!(error.reason.contains("no complete Rust route executor"));
    }

    #[test]
    fn model_editor_has_an_explicit_project_owned_executor_boundary() {
        let availability = surface_availability(SurfaceId::ModelEditor);
        assert_eq!(
            availability,
            SurfaceExecutionAvailability::Available {
                executor: "rspice-ui governed project-owned device-model definition editor",
                evidence_boundary: "A selected coherent project-owned single-card model can be opened, edited through typed parameter, section, statistical, temperature, and executable qualification contracts, parser-validated, compared with retained releases, and committed as guarded source or evidence-only revisions. Qualification runs execute on the current real Desktop or WebAssembly runtime and assemble parity evidence only from an exact retained pair. Promotion remains fail-closed behind complete source-bound evidence, declarations, compatibility, and independent approvals. Built-in and external sources remain read-only.",
            }
        );
        assert!(route_availability(SurfaceRoute::surface(SurfaceId::ModelEditor)).can_open());
        require_available(SurfaceRoute::surface(SurfaceId::ModelEditor))
            .expect("model editor route executor is registered");
    }

    #[test]
    fn pdk_administration_has_an_explicit_signed_package_executor_boundary() {
        let availability = surface_availability(SurfaceId::PdkTechnologyAdmin);
        assert!(availability.can_open());
        assert!(matches!(
            availability,
            SurfaceExecutionAvailability::Available {
                executor: "rspice-ui signed PDK technology-package administrator",
                ..
            }
        ));
        require_available(SurfaceRoute::surface(SurfaceId::PdkTechnologyAdmin))
            .expect("PDK technology administrator route executor is registered");
    }

    #[test]
    fn library_cellview_manager_has_an_explicit_shared_project_domain_boundary() {
        let availability = surface_availability(SurfaceId::LibraryCellviewManager);
        assert!(matches!(
            availability,
            SurfaceExecutionAvailability::Available {
                executor: "rspice-ui project-owned library, cellview, symbol, and component-form authoring workspace",
                ..
            }
        ));
        require_available(SurfaceRoute::surface(SurfaceId::LibraryCellviewManager))
            .expect("Library Cellview Manager route executor is registered");
    }

    #[test]
    fn models_and_pdk_surface_execution_matrix_is_explicit_and_fail_closed() {
        for surface in [
            SurfaceId::ModelEditor,
            SurfaceId::ModelCorrelation,
            SurfaceId::PdkTechnologyAdmin,
            SurfaceId::LibraryCellviewManager,
        ] {
            assert!(
                surface_availability(surface).can_open(),
                "{surface} lost its registered Rust executor"
            );
        }
        for surface in [
            SurfaceId::PcellDesigner,
            SurfaceId::ModelExtraction,
            SurfaceId::LibraryCharacterization,
            SurfaceId::ProtectedIpCenter,
        ] {
            let availability = surface_availability(surface);
            assert!(
                !availability.can_open(),
                "{surface} must not become available merely because its mockup exists"
            );
            assert!(
                availability
                    .reason()
                    .is_some_and(|reason| reason.contains("no complete Rust route executor")),
                "{surface} does not expose the expected implementation boundary"
            );
        }
    }

    #[test]
    fn every_object_aware_route_fails_closed_without_a_runtime_resolver() {
        let object_ref = ObjectRef::new(
            ProductObjectKind::Project,
            Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").expect("fixture UUID"),
        )
        .expect("non-nil object reference");

        for surface in SurfaceId::ALL {
            let availability = route_availability(SurfaceRoute::for_object(surface, object_ref));
            assert!(!availability.can_open(), "{surface} object route opened");
            assert!(
                availability
                    .reason()
                    .is_some_and(|reason| reason.contains("runtime object resolver")),
                "{surface} did not report the resolver boundary"
            );
        }
    }

    #[test]
    fn every_capability_inspection_route_has_an_explicit_read_only_executor() {
        for workflow in CapabilityWorkflowId::ALL {
            let availability = route_availability(SurfaceRoute::capability_workflow(workflow));
            assert!(availability.can_open(), "{workflow} should be inspectable");
        }
    }
}

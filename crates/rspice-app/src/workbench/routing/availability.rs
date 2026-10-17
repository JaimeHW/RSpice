//! Fail-closed route-executor availability for canonical GUI surfaces.
//!
//! Registry presence describes the governed design contract only. It never
//! proves that a Rust executor, engine, entitlement, platform qualification,
//! or sign-off path exists. This module is the single runtime gate between a
//! canonical [`SurfaceId`] and application navigation.

use crate::workbench::{SurfaceId, SurfaceRoute};

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
    surface_availability(route.surface_id())
}

#[must_use]
pub const fn surface_availability(surface: SurfaceId) -> SurfaceExecutionAvailability {
    match surface {
        SurfaceId::Project
        | SurfaceId::Design
        | SurfaceId::Simulate
        | SurfaceId::Stimulus
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
        SurfaceId::ReportAuthoring => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui project-owned report and datasheet document composer",
            evidence_boundary: "Versioned report documents in the project, outlines from templates, adding and reordering pages, paper settings, text and result blocks, live result references that follow the project's latest results, frozen references that keep a fixed image, publication settings, and hardcopy export all work. A reference to missing, stale, or unlinked results is shown as such and is never drawn with made-up content. The composer shows whether a release can be handed off and links to Release Closure; it never freezes, approves, promotes, or signs off a release candidate, and offers no external publishing or distribution service.",
        },
        SurfaceId::ModelEditor => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui governed project-owned device-model definition editor",
            evidence_boundary: "A selected single-card model owned by the project can be opened, edited through its parameter, section, statistical, temperature, and qualification forms, checked by the parser, compared with saved releases, and saved as a source revision or a results-only revision. Qualification runs on this desktop or browser build and compares only a matching saved pair of results. Promotion stays blocked until the source's results, declarations, compatibility checks, and independent approvals are complete. Built-in and external sources stay read-only.",
        },
        SurfaceId::ModelCorrelation => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui project-owned measurement-correlation workspace",
            evidence_boundary: "Read-only CSV measurement sets, the source and simulation each comparison came from, unit-aware alignment and metrics, saved residuals, outlier decisions that can only be added to, independent review records, and handoff to qualification all work against the project history. Correlation results never promote a model or turn a failed qualification vector into a pass.",
        },
        SurfaceId::PdkTechnologyAdmin => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui signed PDK technology-package administrator",
            evidence_boundary: "Checking Ed25519 manifest signatures, artifact digests and sizes, layer-purpose and stream-map completeness, connectivity, callback capabilities, platform requirements, and recognition and extraction definitions; comparing trusted signed revisions; the read-only display profiles a package ships; installed revisions; publisher trust; active bindings; saved configuration; trust keys an administrator adds or revokes on this computer (a revocation cannot be undone); and a tamper-evident log of trust, install, activation, and rollback steps all work. A package's SPICE model files are decoded, resolved with their includes, narrowed to the selected process section, and given to the simulator only when a project is pinned to that trusted archive; activating a package does not change which models a project uses. Signed callback modules (schema 3) are checked when installed and run for a pinned project in a sandboxed interpreter with fixed fuel and memory limits and no file or network access; each run's inputs, outputs, and identities are recorded with the project. No product engine uses callback output yet, and rule decks are not run. Display profiles are stored but not used by a layout renderer; project and organization display-profile repositories are not available. The local trust file is not an operating-system or organization trust service, packages never change pinned projects on their own, and package checks do not run or qualify layout recognition, parasitic extraction, PCells, migration, or sign-off.",
        },
        SurfaceId::LibraryCellviewManager => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui project-owned library, cellview, symbol, and component-form authoring workspace",
            evidence_boundary: "The three-column project library browser, checked cell and view edits, a selection shared with the rest of RSpice, symbol preview, terminal and parameter-form inspection, opening the symbol editor, creating or importing model-bound symbols, form editing, edit-lock inspection, audit history, and versioned project-library publication records all work. Old or invalid symbol metadata is reported and blocks use; the workspace does not provide PCells, layout, extraction, characterization, protected IP, remote collaboration, or sign-off.",
        },
        SurfaceId::NotificationCenter => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui session notification panel",
            evidence_boundary: "Filtering by domain and read state, marking read, dismissing one notice, clearing read notices, retention disclosure, and each notice's one offer (its run in Results, or the Console) operate on the real in-memory activity stream of this application session; nothing is persisted, and no external approval or remote background service is inferred.",
        },
        SurfaceId::CommandPalette => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui route-owned command palette overlay",
            evidence_boundary: "Canonical deep links, browser history, keyboard focus restoration, scoped command search, availability disclosure, and execution through the typed command registry are executable.",
        },
        SurfaceId::HelpCenter => SurfaceExecutionAvailability::Available {
            executor: "rspice-ui route-owned help and diagnostics manager",
            evidence_boundary: "Canonical deep links, browser history, versioned guidance, release and migration notes, diagnostics, support-bundle disclosure, and legal/privacy notices are executable. External support delivery is not implied.",
        },
        SurfaceId::MultiTestStudio
        | SurfaceId::RfDataDisplayWorkbench
        | SurfaceId::StatisticalVisualizationWorkbench
        | SurfaceId::JobDiagnostics
        | SurfaceId::CliBatchWorkbench
        | SurfaceId::DeckCompatibilityCenter => SurfaceExecutionAvailability::Unavailable {
            reason: "This specialist workspace is a governed design specification, but this build does not ship its complete Rust document and interaction executor. Use its owning primary workspace for currently implemented tasks.",
        },
        SurfaceId::WorkflowDialog => SurfaceExecutionAvailability::Unavailable {
            reason: "Transactional workflow dialogs are available only from the command or object that supplies their required draft context; the generic catalog identity is not a standalone destination.",
        },
        SurfaceId::MobileNavigation => SurfaceExecutionAvailability::Unavailable {
            reason: "Compact task navigation is exposed automatically by the responsive workbench at supported viewport and input sizes; the contextual drawer is not a standalone destination.",
        },
        SurfaceId::MobileTaskHandoff => SurfaceExecutionAvailability::Unavailable {
            reason: "Qualified-target handoff is not implemented in this build, so no navigation action or deep link is offered.",
        },
        SurfaceId::SurfaceContextMenu => SurfaceExecutionAvailability::Unavailable {
            reason: "Selection context menus open from a current selection in a document; they are not a page you can go to on their own.",
        },
        SurfaceId::DockedTool => SurfaceExecutionAvailability::Unavailable {
            reason: "Docked tools are opened by their owning document with a concrete tool identity; the generic catalog identity is not a standalone destination.",
        },
        _ => SurfaceExecutionAvailability::Unavailable {
            reason: "A design for this workspace exists, but this build cannot open it yet.",
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
    use crate::workbench::routing::surface_catalog::ReleaseStatus;
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
                SurfaceId::Stimulus,
                SurfaceId::Results,
                SurfaceId::Verify,
                SurfaceId::Models,
                SurfaceId::Netlist,
                SurfaceId::ReportAuthoring,
                SurfaceId::ModelEditor,
                SurfaceId::ModelCorrelation,
                SurfaceId::PdkTechnologyAdmin,
                SurfaceId::LibraryCellviewManager,
                SurfaceId::ProjectLauncher,
                SurfaceId::Preferences,
                SurfaceId::DesignManagement,
                SurfaceId::AccountOrganization,
                SurfaceId::CommandPalette,
                SurfaceId::JobsManager,
                SurfaceId::NotificationCenter,
                SurfaceId::HelpCenter,
                SurfaceId::SpecialistToolBrowser,
            ]
        );
    }

    #[test]
    fn every_release_scope_surface_has_a_registered_route_executor() {
        let missing = SurfaceId::ALL
            .into_iter()
            .filter(|surface| surface.release_status() == ReleaseStatus::ReleaseScope)
            .filter(|surface| !surface_availability(*surface).can_open())
            .collect::<Vec<_>>();
        assert!(
            missing.is_empty(),
            "release-scope surfaces without route executors: {missing:?}"
        );
    }

    #[test]
    fn contextual_and_unimplemented_catalog_routes_fail_with_specific_guidance() {
        let cases = [
            (SurfaceId::MultiTestStudio, "owning primary workspace"),
            (SurfaceId::WorkflowDialog, "required draft context"),
            (SurfaceId::MobileNavigation, "responsive workbench"),
            (SurfaceId::MobileTaskHandoff, "not implemented"),
            (SurfaceId::SurfaceContextMenu, "current selection"),
            (SurfaceId::DockedTool, "concrete tool identity"),
        ];
        for (surface, expected) in cases {
            let error = require_available(SurfaceRoute::surface(surface))
                .expect_err("catalog-only route must fail closed");
            assert!(
                error.reason.contains(expected),
                "{surface} did not explain its unavailable route: {}",
                error.reason
            );
        }
    }

    #[test]
    fn specialist_designs_fail_closed_until_an_executor_is_registered() {
        let route = SurfaceRoute::surface(SurfaceId::RfWorkbench);
        let error = require_available(route).expect_err("RF route is not implemented");
        assert_eq!(error.surface, SurfaceId::RfWorkbench);
        assert!(error.reason.contains("this build cannot open it yet"));
    }

    #[test]
    fn model_editor_has_an_explicit_project_owned_executor_boundary() {
        let availability = surface_availability(SurfaceId::ModelEditor);
        assert_eq!(
            availability,
            SurfaceExecutionAvailability::Available {
                executor: "rspice-ui governed project-owned device-model definition editor",
                evidence_boundary: "A selected single-card model owned by the project can be opened, edited through its parameter, section, statistical, temperature, and qualification forms, checked by the parser, compared with saved releases, and saved as a source revision or a results-only revision. Qualification runs on this desktop or browser build and compares only a matching saved pair of results. Promotion stays blocked until the source's results, declarations, compatibility checks, and independent approvals are complete. Built-in and external sources stay read-only.",
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
                    .is_some_and(|reason| reason.contains("this build cannot open it yet")),
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
}

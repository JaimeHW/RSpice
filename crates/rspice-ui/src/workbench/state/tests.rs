//! Tests for workspace layout persistence.
//!
//! Each workspace keeps an independent dock composition, presets apply to the
//! live layout owner rather than a copy, and every layout must round-trip
//! through the serialized session unchanged.

use super::*;
use crate::workbench::CapabilityWorkflowId;

#[test]
fn canonical_workspace_order_is_stable() {
    assert_eq!(Workspace::ALL.len(), 7);
}

#[test]
fn project_page_catalog_is_the_complete_five_tab_contract() {
    assert_eq!(
        ProjectPage::ALL,
        [
            ProjectPage::Overview,
            ProjectPage::Library,
            ProjectPage::Configuration,
            ProjectPage::Dependencies,
            ProjectPage::Recovery,
        ]
    );
    assert_eq!(ProjectPage::default(), ProjectPage::Overview);
    assert_eq!(
        ProjectPage::ALL.map(ProjectPage::label),
        [
            "Overview",
            "Library",
            "Configuration",
            "Dependencies",
            "Recovery",
        ]
    );
    assert_eq!(
        ProjectPage::ALL.into_iter().collect::<HashSet<_>>().len(),
        ProjectPage::ALL.len(),
        "the project tab catalog must not contain duplicate routes"
    );
}

#[test]
fn legacy_project_pages_restore_to_their_supported_successors() {
    for (serialized, expected) in [
        (r#""Dashboard""#, ProjectPage::Overview),
        (r#""Activity""#, ProjectPage::Overview),
        (r#""Technology""#, ProjectPage::Dependencies),
    ] {
        assert_eq!(
            serde_json::from_str::<ProjectPage>(serialized)
                .expect("legacy project page remains decodable"),
            expected
        );
    }

    for page in ProjectPage::ALL {
        let encoded = serde_json::to_string(&page).expect("project page serializes");
        let restored: ProjectPage =
            serde_json::from_str(&encoded).expect("canonical project page restores");
        assert_eq!(restored, page);
    }
}

#[test]
fn workbench_without_a_project_page_defaults_to_overview() {
    let mut encoded = serde_json::to_value(WorkbenchState::default()).unwrap();
    encoded
        .as_object_mut()
        .expect("workbench is an object")
        .remove("project_page");

    let restored: WorkbenchState = serde_json::from_value(encoded).unwrap();

    assert_eq!(restored.project_page, ProjectPage::Overview);
}

#[test]
fn visualization_studio_presentation_round_trips_with_the_workbench() {
    let mut state = WorkbenchState::default();
    state.visualization_studio.section =
        crate::workbench::documents::visualization_studio::VisualizationSection::Axes;
    state.visualization_studio.tool =
        crate::workbench::documents::visualization_studio::ViewerTool::Pan;
    state.visualization_studio.zoom = 2.5;
    state.visualization_studio.selected_viewer_document = "viewer-bode".to_owned();

    let encoded = serde_json::to_string(&state).expect("workbench serializes");
    let restored: WorkbenchState =
        serde_json::from_str(&encoded).expect("visualization document restores");

    assert_eq!(
        restored.visualization_studio.section,
        crate::workbench::documents::visualization_studio::VisualizationSection::Axes
    );
    assert_eq!(
        restored.visualization_studio.tool,
        crate::workbench::documents::visualization_studio::ViewerTool::Pan
    );
    assert_eq!(restored.visualization_studio.zoom, 2.5);
    assert_eq!(
        restored.visualization_studio.selected_viewer_document,
        "viewer-bode"
    );
}

#[test]
fn legacy_workbench_defaults_the_visualization_document() {
    let mut encoded = serde_json::to_value(WorkbenchState::default()).unwrap();
    encoded
        .as_object_mut()
        .expect("workbench is an object")
        .remove("visualization_studio");

    let restored: WorkbenchState = serde_json::from_value(encoded).unwrap();
    assert_eq!(restored.visualization_studio.zoom, 1.0);
    assert_eq!(
        restored.visualization_studio.selected_viewer_document,
        "viewer-waveform"
    );
    assert!(restored.visualization_studio.panes.is_empty());
}

#[test]
fn ordinary_noise_uses_the_frequency_document_identity() {
    use super::super::ResultViewer;
    assert_eq!(
        ResultViewer::NoiseContrib.viewer_document_id(),
        Some("viewer-bode")
    );
    assert_eq!(
        ResultViewer::Fft.viewer_document_id(),
        Some("viewer-spectrum")
    );
}

/// Every document a sheet claims must exist in the catalog and must accept
/// the sheet's analysis. Three copies of this map used to exist; the Studio's
/// said the noise sheet renders `viewer-spectrum`, whose catalog entry rejects
/// a `noise` analysis outright.
#[test]
fn every_claimed_viewer_document_is_in_the_catalog() {
    use super::super::ResultViewer;
    for viewer in ResultViewer::every() {
        let Some(id) = viewer.viewer_document_id() else {
            continue;
        };
        assert!(
            crate::results::viewer_catalog::viewer_document(id).is_some(),
            "{viewer:?} claims the unregistered document {id}"
        );
    }
    let noise = crate::results::viewer_catalog::viewer_document("viewer-spectrum")
        .expect("the spectrum document is registered");
    assert!(
        !noise.analysis_ids.contains(&"noise"),
        "the spectrum document would accept noise, so this test proves nothing"
    );
}

#[test]
fn restored_visualization_document_repairs_bounds_and_identities() {
    use crate::product::DatasetId;
    use crate::workbench::documents::visualization_studio::{
        VisualizationAnnotation, VisualizationPane,
    };

    let mut state = WorkbenchState::default();
    let dataset_id = DatasetId::new();
    state.visualization_studio.zoom = 99.0;
    state.visualization_studio.selected_viewer_document = "removed-viewer".to_owned();
    state.visualization_studio.next_identity = 0;
    state.visualization_studio.revision = 0;
    state.visualization_studio.active_pane = Some(42);
    state.visualization_studio.panes = vec![
        VisualizationPane {
            id: 42,
            viewer: super::super::ResultViewer::Waves,
            viewer_document_id: "removed-viewer".to_owned(),
            dataset_id,
            analysis_sequence: 1,
            x_link: None,
            cursor_group: None,
            page: "Page 1".to_owned(),
            placement: Default::default(),
        },
        VisualizationPane {
            id: 42,
            viewer: super::super::ResultViewer::Bode,
            viewer_document_id: "viewer-bode".to_owned(),
            dataset_id,
            analysis_sequence: 1,
            x_link: None,
            cursor_group: None,
            page: "Page 1".to_owned(),
            placement: Default::default(),
        },
    ];
    state.visualization_studio.annotations = vec![VisualizationAnnotation {
        id: 42,
        dataset_id,
        analysis_sequence: 1,
        x: 0.0,
        text: "threshold".to_owned(),
    }];

    state.normalize_visualization_studio();

    assert_eq!(state.visualization_studio.zoom, 8.0);
    assert_eq!(state.visualization_studio.revision, 1);
    assert_eq!(
        state.visualization_studio.selected_viewer_document,
        "viewer-waveform"
    );
    assert_eq!(state.visualization_studio.panes.len(), 2);
    assert_eq!(state.visualization_studio.panes[0].id, 42);
    assert_eq!(
        state.visualization_studio.panes[0].viewer_document_id,
        "viewer-waveform"
    );
    let identities = state
        .visualization_studio
        .panes
        .iter()
        .map(|pane| pane.id)
        .chain(
            state
                .visualization_studio
                .annotations
                .iter()
                .map(|annotation| annotation.id),
        )
        .collect::<HashSet<_>>();
    assert_eq!(identities.len(), 3);
    assert_eq!(state.visualization_studio.active_pane, Some(42));
    assert!(state.visualization_studio.next_identity > identities.iter().copied().max().unwrap());
}

#[test]
fn visualization_studio_is_a_persistent_surface_not_an_application_modal() {
    let mut state = WorkbenchState::default();
    state.navigation.replace(
        SurfaceRoute::surface(SurfaceId::VisualizationStudio),
        RouteTransitionSource::Restore,
    );
    state.workspace = Workspace::Results;

    assert_eq!(
        state.current_route().surface_id(),
        SurfaceId::VisualizationStudio
    );
    assert_eq!(state.workspace, Workspace::Results);
    assert!(!state.application_modal_open());
}

#[test]
fn library_cellview_route_and_page_restore_with_models_workspace_ownership() {
    let mut state = WorkbenchState::default();
    state.library_cellview_page = LibraryCellviewPage::SymbolForm;
    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::LibraryCellviewManager),
            RouteTransitionSource::User,
        )
        .expect("Library Cellview Manager has a registered executor");

    assert_eq!(state.workspace, Workspace::Models);
    assert_eq!(
        state.current_route().surface_id(),
        SurfaceId::LibraryCellviewManager
    );
    assert!(!state.application_modal_open());

    let encoded = serde_json::to_string(&state).expect("workbench session serializes");
    let mut restored: WorkbenchState =
        serde_json::from_str(&encoded).expect("workbench session restores");
    restored.workspace = Workspace::Project;
    restored.reconcile_restored_navigation();

    assert_eq!(restored.workspace, Workspace::Models);
    assert_eq!(
        restored.current_route().surface_id(),
        SurfaceId::LibraryCellviewManager
    );
    assert_eq!(
        restored.library_cellview_page,
        LibraryCellviewPage::SymbolForm
    );
}

#[test]
fn legacy_session_without_library_cellview_page_defaults_to_libraries() {
    let state: WorkbenchState =
        serde_json::from_str(r#"{"workspace":"Models"}"#).expect("legacy workbench restores");
    assert_eq!(state.library_cellview_page, LibraryCellviewPage::Libraries);
}

#[test]
fn specialist_preferences_restore_known_ids_and_ignore_corrupt_entries() {
    let restored: SpecialistToolBrowserState = serde_json::from_value(serde_json::json!({
        "favorites": ["rf-workbench", "removed-workspace", 42, null],
        "pinned": "not-an-array",
        "recents": ["photonics-workbench", {}, "model-editor"]
    }))
    .expect("future and malformed preference entries are isolated");

    assert_eq!(restored.favorites, [SurfaceId::RfWorkbench]);
    assert!(restored.pinned.is_empty());
    assert_eq!(
        restored.recents,
        [SurfaceId::PhotonicsWorkbench, SurfaceId::ModelEditor]
    );
}

#[test]
fn specialist_preferences_normalize_identity_and_bound_recent_history() {
    let specialist_ids = SurfaceId::ALL
        .into_iter()
        .filter(|surface| surface.archetype() == SurfaceArchetype::SpecialistWorkspace)
        .take(SpecialistToolBrowserState::RECENT_LIMIT + 3)
        .collect::<Vec<_>>();
    let mut browser = SpecialistToolBrowserState {
        favorites: vec![
            SurfaceId::RfWorkbench,
            SurfaceId::Project,
            SurfaceId::RfWorkbench,
        ],
        pinned: vec![SurfaceId::ModelEditor, SurfaceId::ModelEditor],
        recents: specialist_ids.clone(),
        ..SpecialistToolBrowserState::default()
    };

    browser.normalize();
    assert_eq!(browser.favorites, [SurfaceId::RfWorkbench]);
    assert_eq!(browser.pinned, [SurfaceId::ModelEditor]);
    assert_eq!(
        browser.recents,
        specialist_ids[..SpecialistToolBrowserState::RECENT_LIMIT]
    );

    browser.record_recent(SurfaceId::Project);
    assert_eq!(
        browser.recents,
        specialist_ids[..SpecialistToolBrowserState::RECENT_LIMIT]
    );
    browser.record_recent(SurfaceId::RfWorkbench);
    assert_eq!(browser.recents.first(), Some(&SurfaceId::RfWorkbench));
    assert_eq!(
        browser.recents.len(),
        SpecialistToolBrowserState::RECENT_LIMIT
    );
}

#[test]
fn specialist_session_round_trip_retains_preferences_not_transient_review_state() {
    let browser = SpecialistToolBrowserState {
        query: "rf gain".to_owned(),
        filter: SpecialistToolFilter::Favorites,
        focus_search: true,
        favorites: vec![SurfaceId::RfWorkbench],
        pinned: vec![SurfaceId::ModelEditor],
        recents: vec![SurfaceId::PhotonicsWorkbench],
    };

    let encoded = serde_json::to_string(&browser).expect("browser preferences serialize");
    let restored: SpecialistToolBrowserState =
        serde_json::from_str(&encoded).expect("browser preferences restore");
    assert_eq!(restored.query, "");
    assert_eq!(restored.filter, SpecialistToolFilter::All);
    assert!(!restored.focus_search);
    assert_eq!(restored.favorites, browser.favorites);
    assert_eq!(restored.pinned, browser.pinned);
    assert_eq!(restored.recents, browser.recents);
}

#[test]
fn responsive_width_classes_match_every_mockup_boundary() {
    assert_eq!(WidthClass::for_width(560.0), WidthClass::Phone);
    assert_eq!(WidthClass::for_width(561.0), WidthClass::Tablet);
    assert_eq!(WidthClass::for_width(820.0), WidthClass::Tablet);
    assert_eq!(WidthClass::for_width(821.0), WidthClass::Desktop);
    assert_eq!(WidthClass::for_width(1260.0), WidthClass::Desktop);
    assert_eq!(WidthClass::for_width(1261.0), WidthClass::Wide);
}

#[test]
fn workspace_activation_updates_canonical_route_and_recent_history() {
    let mut state = WorkbenchState::default();
    state.activate(Workspace::Results);
    state.activate(Workspace::Design);

    assert_eq!(state.workspace, Workspace::Design);
    assert_eq!(
        state.navigation.recent_entries(),
        &[
            SurfaceRoute::surface(SurfaceId::Results),
            SurfaceRoute::surface(SurfaceId::Design),
        ]
    );
}

#[test]
fn results_split_is_limited_to_the_three_mockup_primary_stages() {
    let mut state = WorkbenchState {
        split_with_results: true,
        ..WorkbenchState::default()
    };

    for workspace in [Workspace::Design, Workspace::Netlist, Workspace::Simulate] {
        state.activate(workspace);
        assert!(state.supports_results_split(), "{workspace:?}");
        assert!(state.results_split_visible(true, true), "{workspace:?}");
    }

    for workspace in [
        Workspace::Project,
        Workspace::Results,
        Workspace::Verify,
        Workspace::Models,
    ] {
        state.activate(workspace);
        assert!(!state.supports_results_split(), "{workspace:?}");
        assert!(!state.results_split_visible(true, true), "{workspace:?}");
    }
}

#[test]
fn remembered_split_never_projects_without_project_and_retained_evidence() {
    let state = WorkbenchState {
        split_with_results: true,
        ..WorkbenchState::default()
    };

    assert!(!state.results_split_visible(false, true));
    assert!(!state.results_split_visible(true, false));
    assert!(state.results_split_visible(true, true));
}

#[test]
fn multi_step_return_reconciles_manager_and_workspace_once_at_final_route() {
    let mut state = WorkbenchState::default();
    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::Results),
            RouteTransitionSource::User,
        )
        .expect("Results is available");
    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::FeatureAvailability),
            RouteTransitionSource::User,
        )
        .expect("capability matrix is available");
    state
        .navigate(
            SurfaceRoute::capability_workflow(CapabilityWorkflowId::SourceLoadPullAnalysis),
            RouteTransitionSource::User,
        )
        .expect("planned workflow inspection is available");
    state.capability_matrix.section = CapabilityMatrixSection::PlannedDesigns;
    state.capability_matrix.scroll_offset = 240.0;
    state.clear_browser_history_effects();

    let transition = state
        .navigate_back_steps(2, RouteTransitionSource::User)
        .expect("matrix and source routes exist");

    assert_eq!(
        transition.previous,
        SurfaceRoute::capability_workflow(CapabilityWorkflowId::SourceLoadPullAnalysis)
    );
    assert_eq!(
        transition.current,
        SurfaceRoute::surface(SurfaceId::Results)
    );
    assert_eq!(state.workspace, Workspace::Results);
    assert_eq!(
        state.capability_matrix.section,
        CapabilityMatrixSection::Platforms
    );
    assert_eq!(state.capability_matrix.scroll_offset, 0.0);
    assert_eq!(
        state.take_browser_history_effect(),
        Some(BrowserHistoryEffect::Traverse {
            delta: -2,
            destination: SurfaceRoute::surface(SurfaceId::Results),
        })
    );
}

#[test]
fn touch_guide_history_roundtrips_through_its_platform_lifecycle_source() {
    let mut state = WorkbenchState::default();
    let matrix = SurfaceRoute::surface(SurfaceId::FeatureAvailability);
    let lifecycle = SurfaceRoute::capability_workflow(CapabilityWorkflowId::PlatformLifecycle);
    let touch_guide = SurfaceRoute::capability_workflow(CapabilityWorkflowId::TouchEditGuide);

    for route in [matrix, lifecycle, touch_guide] {
        state
            .navigate(route, RouteTransitionSource::User)
            .expect("the informational route has an explicit executor");
    }

    assert_eq!(state.current_route(), touch_guide);
    assert_eq!(state.previous_route(), Some(lifecycle));
    assert_eq!(
        state
            .navigate_back(RouteTransitionSource::User)
            .expect("touch guide retains its lifecycle source")
            .current,
        lifecycle
    );
    assert_eq!(
        state
            .navigate_back(RouteTransitionSource::User)
            .expect("lifecycle retains the capability matrix source")
            .current,
        matrix
    );
    assert_eq!(
        state
            .navigate_forward(RouteTransitionSource::User)
            .expect("forward restores lifecycle")
            .current,
        lifecycle
    );
    assert_eq!(
        state
            .navigate_forward(RouteTransitionSource::User)
            .expect("forward restores touch guidance")
            .current,
        touch_guide
    );
}

#[test]
fn legacy_workspace_session_migrates_to_the_matching_primary_route() {
    let mut state: WorkbenchState =
        serde_json::from_str(r#"{"workspace":"Results"}"#).expect("legacy workbench restores");
    assert_eq!(state.current_route().surface_id(), SurfaceId::Design);

    state.reconcile_restored_navigation();
    assert_eq!(state.workspace, Workspace::Results);
    assert_eq!(state.current_route().surface_id(), SurfaceId::Results);
    assert_eq!(
        state.take_browser_history_effect(),
        Some(BrowserHistoryEffect::Replace(SurfaceRoute::surface(
            SurfaceId::Results
        )))
    );
}

#[test]
fn legacy_shell_geometry_migrates_to_redesigned_responsive_docks() {
    let mut state = WorkbenchState::default();
    state.layout_schema_version = 0;
    state.navigator_width = 232.0;
    state.navigator_width_custom = true;
    state.inspector_width = 440.0;
    state.inspector_width_custom = true;
    state.workspace_layouts.insert(
        Workspace::Design,
        WorkspaceLayoutState {
            navigator_width: 232.0,
            navigator_width_custom: true,
            inspector_width: 440.0,
            inspector_width_custom: true,
            ..WorkspaceLayoutState::default()
        },
    );

    state.reconcile_restored_navigation();

    assert_eq!(state.layout_schema_version, LAYOUT_SCHEMA_VERSION);
    assert_eq!(state.navigator_width, default_navigator_width());
    assert!(!state.navigator_width_custom);
    assert_eq!(state.inspector_width, default_inspector_width());
    assert!(!state.inspector_width_custom);
    let design = state
        .workspace_layouts
        .get(&Workspace::Design)
        .expect("design workspace layout remains present");
    assert_eq!(design.navigator_width, default_navigator_width());
    assert!(!design.navigator_width_custom);
    assert_eq!(design.inspector_width, default_inspector_width());
    assert!(!design.inspector_width_custom);
}

#[test]
fn current_canonical_route_is_the_primary_workspace_source_of_truth() {
    let mut state = WorkbenchState::default();
    state.workspace = Workspace::Project;
    state.navigation.replace(
        SurfaceRoute::surface(SurfaceId::Results),
        RouteTransitionSource::BrowserPop,
    );

    state.reconcile_restored_navigation();
    assert_eq!(state.workspace, Workspace::Results);
    assert_eq!(state.current_route().surface_id(), SurfaceId::Results);
}

#[test]
fn restored_unavailable_verification_page_fails_closed_to_cockpit() {
    let mut state = WorkbenchState::default();
    state.verification_page = VerificationPage::Drc;

    state.reconcile_restored_navigation();

    assert_eq!(state.verification_page, VerificationPage::Yield);
    assert!(
        state
            .take_route_diagnostic()
            .is_some_and(|message| message.contains("executable evidence pipeline"))
    );
}

#[test]
fn toggling_the_active_drawer_closes_it_deterministically() {
    let mut state = WorkbenchState::default();

    state.toggle_drawer(Drawer::Navigator);
    assert_eq!(state.drawer, Some(Drawer::Navigator));
    state.toggle_drawer(Drawer::Navigator);
    assert_eq!(state.drawer, None);

    state.toggle_drawer(Drawer::Navigator);
    state.dismiss_navigator();
    assert!(state.navigator_visible);
    assert_eq!(state.drawer, None);
    state.dismiss_navigator();
    assert!(!state.navigator_visible);

    state.navigator_visible = true;
    state.toggle_drawer(Drawer::Navigator);
    state.toggle_drawer(Drawer::Inspector);
    assert_eq!(state.drawer, Some(Drawer::Inspector));
    state.dismiss_inspector();
    assert!(state.inspector_visible);
    assert_eq!(state.drawer, None);

    state.dismiss_inspector();
    assert!(!state.inspector_visible);
}

#[test]
fn invalidating_preflight_drops_every_revision_bound_presentation_artifact() {
    let mut preflight = PreflightDialogState {
        open: true,
        report: Some(PreflightReport {
            project_revision: 4,
            topology_root: "user/top/schematic".to_owned(),
            topology_revision: 9,
            topology_closure: vec![("user/top/schematic".to_owned(), 9)],
            simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
            simulation_plan_revision: Some(crate::product::ObjectRevision::INITIAL),
            blockers: Vec::new(),
            advisories: Vec::new(),
            prepared: None,
        }),
        pending_toast: Some(PreflightToast {
            message: "stale success".to_owned(),
            warning: false,
        }),
        ..Default::default()
    };

    preflight.invalidate();

    assert!(!preflight.open);
    assert!(preflight.report.is_none());
    assert!(preflight.pending_toast.is_none());
}

#[test]
fn mixed_dock_composition_keeps_only_the_supported_drawer() {
    let mut state = WorkbenchState::default();
    state.toggle_drawer(Drawer::Navigator);
    state.reconcile_drawer_mode(false, true, false);
    assert_eq!(state.drawer, None);

    state.toggle_drawer(Drawer::Inspector);
    state.reconcile_drawer_mode(false, true, false);
    assert_eq!(state.drawer, Some(Drawer::Inspector));

    state.toggle_drawer(Drawer::Workspaces);
    state.reconcile_drawer_mode(false, true, false);
    assert_eq!(state.drawer, None);
}

#[test]
fn application_modals_own_global_input_exclusively() {
    let mut state = WorkbenchState::default();
    assert!(!state.application_modal_open());

    state.project_launcher_open = true;
    assert!(state.application_modal_open());
    state.project_launcher_open = false;
    state.preflight.open = true;
    assert!(state.application_modal_open());
    state.preflight.open = false;
    state.verification.regression_baseline_picker_open = true;
    assert!(state.application_modal_open());
    state.verification.regression_baseline_picker_open = false;
    state.verification.tuning_review_open = true;
    assert!(state.application_modal_open());
    state.verification.tuning_review_open = false;
    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::Preferences),
            RouteTransitionSource::User,
        )
        .expect("Preferences has a registered executor");
    assert!(state.application_modal_open());
    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::DesignManagement),
            RouteTransitionSource::User,
        )
        .expect("Design Management has a registered executor");
    assert_eq!(state.workspace, Workspace::Design);
    assert!(state.application_modal_open());
    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::FeatureAvailability),
            RouteTransitionSource::User,
        )
        .expect("capability manager has a registered executor");
    assert!(state.application_modal_open());
}

#[test]
fn capability_matrix_persists_profile_but_not_local_presentation() {
    let mut state = WorkbenchState::default();
    state.engineering_profile = EngineeringProfile::RfMicrowave;
    state.capability_matrix = CapabilityMatrixState {
        section: CapabilityMatrixSection::Workspaces,
        scroll_offset: 318.5,
        last_document_compact: Some(false),
        drilldown: Some(CapabilityMatrixDrilldown::PlannedWorkflow(
            "transient-noise".to_owned(),
        )),
        drilldown_scroll_offset: 91.0,
        interoperability_section: InteroperabilitySection::Qualification,
        interoperability_domain: InteroperabilityDomain::MechanicalExchange,
        interoperability_support_level: InteroperabilitySupportLevel::Planned,
    };

    let json = serde_json::to_string(&state).expect("workbench serializes");
    let restored: WorkbenchState =
        serde_json::from_str(&json).expect("workbench review context restores");
    assert_eq!(
        restored.engineering_profile,
        EngineeringProfile::RfMicrowave
    );
    assert_eq!(
        restored.capability_matrix.section,
        CapabilityMatrixSection::Platforms
    );
    assert_eq!(restored.capability_matrix.scroll_offset, 0.0);
    assert_eq!(restored.capability_matrix.last_document_compact, None);
    assert_eq!(restored.capability_matrix.drilldown, None);
    assert_eq!(restored.capability_matrix.drilldown_scroll_offset, 0.0);
    assert_eq!(
        restored.capability_matrix.interoperability_section,
        InteroperabilitySection::FormatMatrix
    );
    assert_eq!(
        restored.capability_matrix.interoperability_domain,
        InteroperabilityDomain::All
    );
    assert_eq!(
        restored.capability_matrix.interoperability_support_level,
        InteroperabilitySupportLevel::All
    );
}

#[test]
fn capability_matrix_reopens_at_platforms_and_dismisses_drilldowns() {
    let mut state = WorkbenchState::default();
    state.capability_matrix.section = CapabilityMatrixSection::Analyses;
    state.capability_matrix.scroll_offset = 200.0;
    state.capability_matrix.last_document_compact = Some(false);
    state.capability_matrix.drilldown = Some(CapabilityMatrixDrilldown::PlannedWorkflow(
        "transient-noise".to_owned(),
    ));
    state.capability_matrix.interoperability_section = InteroperabilitySection::Qualification;
    state.capability_matrix.interoperability_domain = InteroperabilityDomain::MechanicalExchange;
    state.capability_matrix.interoperability_support_level =
        InteroperabilitySupportLevel::ConnectorDependent;

    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::FeatureAvailability),
            RouteTransitionSource::User,
        )
        .expect("capability manager has an executor");
    assert_eq!(
        state.capability_matrix.section,
        CapabilityMatrixSection::Platforms
    );
    assert_eq!(state.capability_matrix.scroll_offset, 0.0);
    assert_eq!(state.capability_matrix.last_document_compact, None);
    assert_eq!(state.capability_matrix.drilldown, None);
    assert_eq!(
        state.capability_matrix.interoperability_section,
        InteroperabilitySection::FormatMatrix
    );
    assert_eq!(
        state.capability_matrix.interoperability_domain,
        InteroperabilityDomain::All
    );
    assert_eq!(
        state.capability_matrix.interoperability_support_level,
        InteroperabilitySupportLevel::All
    );

    state.capability_matrix.section = CapabilityMatrixSection::Workspaces;
    state.capability_matrix.scroll_offset = 42.0;
    state.capability_matrix.drilldown = Some(CapabilityMatrixDrilldown::PlannedWorkflow(
        "transient-noise".to_owned(),
    ));
    state.capability_matrix.interoperability_section = InteroperabilitySection::RoundTripContract;
    state.capability_matrix.interoperability_domain = InteroperabilityDomain::IcDesignAndLayout;
    state.capability_matrix.interoperability_support_level =
        InteroperabilitySupportLevel::Qualified;
    state.activate(Workspace::Design);
    assert_eq!(
        state.capability_matrix.section,
        CapabilityMatrixSection::Platforms
    );
    assert_eq!(state.capability_matrix.scroll_offset, 0.0);
    assert_eq!(state.capability_matrix.drilldown, None);
    assert_eq!(
        state.capability_matrix.interoperability_section,
        InteroperabilitySection::FormatMatrix
    );
    assert_eq!(
        state.capability_matrix.interoperability_domain,
        InteroperabilityDomain::All
    );
    assert_eq!(
        state.capability_matrix.interoperability_support_level,
        InteroperabilitySupportLevel::All
    );
}

#[test]
fn fresh_browser_session_keeps_route_but_drops_restored_traversal() {
    let mut state = WorkbenchState::default();
    state.activate(Workspace::Results);
    assert!(state.previous_route().is_some());
    assert!(state.has_pending_browser_history_effects());

    state.reset_navigation_history_for_fresh_browser_session();

    assert_eq!(
        state.current_route(),
        SurfaceRoute::surface(SurfaceId::Results)
    );
    assert_eq!(state.previous_route(), None);
    assert_eq!(state.take_browser_history_effect(), None);
    assert!(state.navigate_back(RouteTransitionSource::User).is_none());
}

#[test]
fn restored_route_reconciliation_is_idempotent_across_two_passes() {
    let mut state = WorkbenchState::default();
    state.navigation = serde_json::from_str(
        r#"{
                "current":"?view=design",
                "back":["?surface=rf-workbench","?surface=not-a-surface"],
                "forward":[],
                "recent":[]
            }"#,
    )
    .expect("navigation wire recovers malformed entries");

    state.reconcile_restored_navigation();
    let first = state
        .take_route_diagnostic()
        .expect("first pass reports malformed recovery");
    assert!(first.contains("Malformed routes"));
    assert!(state.navigation.back_entries().is_empty());
    assert!(!state.navigation.recovered_invalid_routes());

    state.reconcile_restored_navigation();
    assert_eq!(state.take_route_diagnostic(), None);
    assert!(!state.navigation.recovered_invalid_routes());
}

#[test]
fn unavailable_history_removal_is_reported_once_without_malformed_flag() {
    let mut state = WorkbenchState::default();
    state.navigation = serde_json::from_str(
        r#"{
                "current":"?view=design",
                "back":["?surface=rf-workbench"],
                "forward":[],
                "recent":[]
            }"#,
    )
    .expect("canonical unavailable history restores");

    state.reconcile_restored_navigation();
    let first = state
        .take_route_diagnostic()
        .expect("first pass reports unavailable removal");
    assert!(first.contains("Unavailable routes"));
    assert!(!state.navigation.recovered_invalid_routes());

    state.reconcile_restored_navigation();
    assert_eq!(state.take_route_diagnostic(), None);
}

#[test]
fn browser_effect_overflow_reaches_the_workbench_recovery_gate() {
    let mut state = WorkbenchState::default();
    for index in 0..65 {
        state.activate(if index % 2 == 0 {
            Workspace::Results
        } else {
            Workspace::Models
        });
    }

    assert!(state.has_pending_browser_history_effects());
    assert_eq!(state.take_browser_history_effect(), None);
    assert!(state.take_browser_history_effect_queue_overflowed());
    assert!(!state.take_browser_history_effect_queue_overflowed());
    assert!(!state.has_pending_browser_history_effects());
    assert_eq!(state.current_route().surface_id(), SurfaceId::Results);
}

#[test]
fn verification_routes_are_the_exact_canonical_seven() {
    assert_eq!(
        VerificationPage::ALL,
        [
            VerificationPage::Yield,
            VerificationPage::Corners,
            VerificationPage::Tuning,
            VerificationPage::Optimization,
            VerificationPage::Reliability,
            VerificationPage::Regression,
            VerificationPage::Drc,
        ]
    );
    assert!(
        VerificationPage::ALL
            .into_iter()
            .filter(|page| page.is_operational())
            .all(|page| page != VerificationPage::Drc)
    );
    assert!(VerificationPage::Tuning.is_operational());
    assert!(!VerificationPage::Drc.is_operational());
}

#[test]
fn removed_tuning_sandbox_fields_migrate_without_restoring_fake_state() {
    let restored: VerificationSessionState = serde_json::from_value(serde_json::json!({
        "tuning_baseline": {"rgain_ohm": 499.0, "cfilt_nf": 22.0, "vref_v": 2.5},
        "tuning_values": {"rgain_ohm": 620.0, "cfilt_nf": 31.0, "vref_v": 2.8},
        "regression_baseline_run": null
    }))
    .expect("legacy tuning fields are ignored during migration");

    assert_eq!(restored, VerificationSessionState::default());
}

#[test]
fn workspace_presets_apply_to_the_live_layout_owner() {
    let mut state = WorkbenchState::default();

    state.apply_workspace_preset(WorkspacePreset::Canvas);
    assert!(state.focus_mode);
    assert!(!state.console_visible);

    state.apply_workspace_preset(WorkspacePreset::Diagnostics);
    assert!(!state.focus_mode);
    assert!(state.console_visible);
    assert_eq!(state.console_height, 260.0);
}

#[test]
fn workspace_switches_restore_independent_dock_compositions() {
    let mut state = WorkbenchState::default();
    state.navigator_width = 340.0;
    state.navigator_width_custom = true;
    state.console_visible = false;

    state.activate(Workspace::Results);
    state.navigator_width = 224.0;
    state.navigator_width_custom = true;
    state.console_visible = true;
    state.console_height = 275.0;

    state.activate(Workspace::Design);
    assert_eq!(state.navigator_width, 340.0);
    assert!(!state.console_visible);

    state.activate(Workspace::Results);
    assert_eq!(state.navigator_width, 224.0);
    assert!(state.console_visible);
    assert_eq!(state.console_height, 275.0);
}

#[test]
fn independent_workspace_layouts_round_trip_with_the_session() {
    let mut state = WorkbenchState::default();
    state.apply_workspace_preset(WorkspacePreset::Diagnostics);
    state.activate(Workspace::Results);
    state.apply_results_review_layout();

    let encoded = serde_json::to_string(&state).unwrap();
    let restored: WorkbenchState = serde_json::from_str(&encoded).unwrap();
    assert!(restored.workspace_layout(Workspace::Design).console_visible);
    assert!(
        !restored
            .workspace_layout(Workspace::Results)
            .navigator_visible
    );
    assert_eq!(
        restored
            .workspace_layout(Workspace::Results)
            .inspector_width,
        332.0
    );
}

#[test]
fn console_pages_include_the_mockup_interactive_surface() {
    assert_eq!(
        ConsolePage::ALL,
        [
            ConsolePage::Console,
            ConsolePage::Problems,
            ConsolePage::Measurements,
            ConsolePage::TaskLog,
            ConsolePage::Interactive,
        ]
    );
    assert_eq!(ConsolePage::Interactive.label(), "Interactive");
}

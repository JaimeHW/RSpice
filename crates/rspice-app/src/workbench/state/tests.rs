//! Tests for workspace layout persistence.
//!
//! Each workspace keeps an independent dock composition, presets apply to the
//! live layout owner rather than a copy, and every layout must round-trip
//! through the serialized session unchanged.

use super::*;

#[test]
fn canonical_workspace_order_is_stable() {
    assert_eq!(Workspace::ALL.len(), 8);
    assert_eq!(
        Workspace::ALL.map(Workspace::label),
        [
            "Project",
            "Design",
            "Simulate",
            "Stimulus Library",
            "Results",
            "Verify",
            "Models",
            "Netlist & Script Editor",
        ]
    );
}

/// One workspace, one name. Each of the three names has a job — the rail and
/// menus read `label`, a specialist row names its `owner_label`, and the
/// context dock is headed by `inspector_title` — so they may differ, but a
/// workspace whose product name already answers two of them must not coin a
/// second spelling for the same thing.
#[test]
fn the_stimulus_library_is_named_once_across_its_three_registers() {
    assert_eq!(Workspace::Stimulus.label(), "Stimulus Library");
    assert_eq!(Workspace::Stimulus.owner_label(), "Stimulus Library");
    assert_eq!(Workspace::Stimulus.inspector_title(), "Definition details");
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
/// the sheet's analysis. Three copies of this map used to exist; one said
/// the noise sheet renders `viewer-spectrum`, whose catalog entry rejects a
/// `noise` analysis outright.
#[test]
fn every_claimed_viewer_document_is_in_the_catalog() {
    use super::super::ResultViewer;
    for viewer in ResultViewer::all() {
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

/// A session saved while the Visualization Studio existed carries its
/// `visualization_studio` block and may have quit on the Studio. The block
/// below is copied from a real session file, with a pane, a selection and a
/// cache budget added. The session must still load: the block is ignored,
/// the Studio route is dropped with one diagnostic, and the rest of the
/// session survives.
#[test]
fn session_saved_with_the_deleted_visualization_studio_still_restores() {
    const STUDIO: &str = concat!(
        "visualization_studio:(section:viewers,tool:select,zoom:2.5,",
        "selected_viewer_document:\"viewer-bode\",panes:[(id:42,viewer:Bode,",
        "viewer_document_id:\"viewer-bode\",",
        "dataset_id:\"6f1c2c1e-5b7a-4c1d-9a54-0d3c2f6b8e01\",analysis_sequence:1,",
        "x_link:Some(1),cursor_group:None,page:\"Page 1\",placement:r#below-selected)],",
        "active_pane:Some(42),next_identity:43,revision:7,annotations:[],markers:[],",
        "measurements:[],linked_x_ranges:{1:(0.0,1e-3)},linked_cursor_positions:{},",
        "pane_x_ranges:{},pane_cursor_positions:{},family_policies:{},",
        "report_page_policies:{},comparison_receipts:[],difference_trace_sets:[],",
        "autoscale:r#robust-visible,complex_projection:r#magnitude-phase,",
        "display_lod:r#envelope-preserving,tile_memory_mib:2048,significant_digits:9,",
        "touch_pane:stage)"
    );
    let mut state = WorkbenchState::default();
    state.engineering_profile = EngineeringProfile::RfMicrowave;
    state.activate(Workspace::Results);
    let saved = ron::to_string(&state).expect("workbench serializes");
    let saved = saved.replacen(
        "current:\"?view=results\"",
        "current:\"?surface=visualization-studio\"",
        1,
    );
    assert!(
        saved.contains("?surface=visualization-studio"),
        "the session must have quit on the Studio for this to prove anything"
    );
    let saved = format!(
        "{},{STUDIO})",
        saved
            .strip_suffix(')')
            .expect("a workbench session is a RON struct")
    );

    let mut restored: WorkbenchState =
        ron::from_str(&saved).expect("a session that kept the Studio's state restores");
    restored.reconcile_restored_navigation();

    assert_eq!(
        restored.engineering_profile,
        EngineeringProfile::RfMicrowave
    );
    assert_eq!(
        restored.current_route(),
        SurfaceRoute::surface(SurfaceId::Design)
    );
    assert!(
        restored
            .take_route_diagnostic()
            .is_some_and(|message| message.contains("Malformed routes"))
    );
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
            SurfaceRoute::surface(SurfaceId::JobsManager),
            RouteTransitionSource::User,
        )
        .expect("jobs manager is available");
    state
        .navigate(
            SurfaceRoute::surface(SurfaceId::SpecialistToolBrowser),
            RouteTransitionSource::User,
        )
        .expect("specialist tool browser is available");
    state.clear_browser_history_effects();

    let transition = state
        .navigate_back_steps(2, RouteTransitionSource::User)
        .expect("manager and source routes exist");

    assert_eq!(
        transition.previous,
        SurfaceRoute::surface(SurfaceId::SpecialistToolBrowser)
    );
    assert_eq!(
        transition.current,
        SurfaceRoute::surface(SurfaceId::Results)
    );
    assert_eq!(state.workspace, Workspace::Results);
    assert_eq!(
        state.take_browser_history_effect(),
        Some(BrowserHistoryEffect::Traverse {
            delta: -2,
            destination: SurfaceRoute::surface(SurfaceId::Results),
        })
    );
}

#[test]
fn manager_history_roundtrips_back_and_forward_to_its_source() {
    let mut state = WorkbenchState::default();
    let jobs = SurfaceRoute::surface(SurfaceId::JobsManager);
    let tools = SurfaceRoute::surface(SurfaceId::SpecialistToolBrowser);
    for route in [jobs, tools] {
        state
            .navigate(route, RouteTransitionSource::User)
            .expect("the manager route has an executor");
    }

    assert_eq!(state.previous_route(), Some(jobs));
    assert_eq!(
        state
            .navigate_back(RouteTransitionSource::User)
            .expect("the tool browser retains its jobs source")
            .current,
        jobs
    );
    assert_eq!(
        state
            .navigate_forward_steps(1, RouteTransitionSource::User)
            .expect("forward restores the tool browser")
            .current,
        tools
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
fn a_session_saved_on_the_removed_drc_page_restores_to_the_first_page() {
    let restored: VerificationPage =
        serde_json::from_value(serde_json::json!("Drc")).expect("the old page name decodes");

    assert_eq!(restored, VerificationPage::Yield);
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
            SurfaceRoute::surface(SurfaceId::JobsManager),
            RouteTransitionSource::User,
        )
        .expect("jobs manager has a registered executor");
    assert!(state.application_modal_open());
}

/// A session saved while the capability matrix existed carries its
/// `capability_matrix` block and may have the matrix open or in its history.
/// It must still load: the stale block is ignored, the dead routes are
/// dropped with one diagnostic, and the rest of the session survives.
#[test]
fn session_saved_with_the_deleted_capability_matrix_still_restores() {
    let mut state = WorkbenchState::default();
    state.engineering_profile = EngineeringProfile::RfMicrowave;
    state.activate(Workspace::Results);
    let mut saved = serde_json::to_value(&state).expect("workbench serializes");
    saved["capability_matrix"] = serde_json::json!({ "section": "workspaces" });
    saved["navigation"] = serde_json::json!({
        "current": "?surface=feature-availability",
        "back": ["?view=results", "?surface=touch-edit-guide"],
        "forward": [],
        "recent": ["?surface=interoperability-matrix"]
    });

    let mut restored: WorkbenchState =
        serde_json::from_value(saved).expect("an old session restores");
    restored.reconcile_restored_navigation();

    assert_eq!(
        restored.engineering_profile,
        EngineeringProfile::RfMicrowave
    );
    assert_eq!(
        restored.current_route(),
        SurfaceRoute::surface(SurfaceId::Design)
    );
    assert_eq!(
        restored.previous_route(),
        Some(SurfaceRoute::surface(SurfaceId::Results))
    );
    assert!(
        restored
            .take_route_diagnostic()
            .is_some_and(|message| message.contains("Malformed routes"))
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
fn verification_routes_are_the_exact_canonical_six() {
    assert_eq!(
        VerificationPage::NAVIGATION,
        [
            VerificationPage::Yield,
            VerificationPage::Corners,
            VerificationPage::Tuning,
            VerificationPage::Optimization,
            VerificationPage::DeviceSafety,
            VerificationPage::Regression,
        ]
    );
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
fn an_unvisited_workspace_opens_from_the_preset_not_the_shipped_default() {
    let mut state = WorkbenchState::default();
    state.set_workspace_preset(WorkspacePreset::Diagnostics);

    // Nothing has arranged Results yet, so it has no snapshot of its own and
    // the preset is the only thing that can say what it opens with. Before
    // the preset was mirrored here this resolved to the shipped default,
    // whose console is hidden.
    assert!(state.workspace_layout(Workspace::Results).console_visible);
    state.activate(Workspace::Results);
    assert!(state.console_visible);
    assert_eq!(state.console_height, 260.0);
}

#[test]
fn choosing_a_preset_leaves_an_arranged_workspace_alone() {
    let mut state = WorkbenchState::default();
    state.activate(Workspace::Results);
    state.console_visible = false;
    state.console_height = 180.0;
    state.activate(Workspace::Design);

    state.set_workspace_preset(WorkspacePreset::Diagnostics);

    // Results was arranged by hand, so it keeps what it was given while the
    // workspaces that were never opened take the preset.
    state.activate(Workspace::Results);
    assert!(!state.console_visible);
    assert_eq!(state.console_height, 180.0);
    assert!(state.workspace_layout(Workspace::Models).console_visible);
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
fn console_pages_are_the_four_evidence_surfaces() {
    assert_eq!(
        ConsolePage::ALL,
        [
            ConsolePage::Console,
            ConsolePage::Problems,
            ConsolePage::Measurements,
            ConsolePage::TaskLog,
        ]
    );
}

/// A session saved while the retired Interactive page was open must still
/// restore. Its page name resolves to the engine log rather than failing the
/// whole workbench snapshot on a variant this build no longer carries.
#[test]
fn a_session_left_on_the_retired_interactive_page_restores_onto_the_console() {
    let page: ConsolePage =
        serde_json::from_str("\"Interactive\"").expect("a retired page name still deserializes");
    assert_eq!(page, ConsolePage::Console);
}

//! Tests for workspace layout persistence.
//!
//! Each workspace keeps an independent dock composition, and presets apply to
//! the live layout owner rather than a copy. That every layout round-trips
//! through the saved session unchanged is tested with the session wire.

use rspice_app_state::workbench::preferences::WorkspacePreset;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::requests::FullScreen;
use rspice_app_workbench::workbench::routing::{
    navigation::{BrowserHistoryEffect, RouteTransitionSource},
    surface_catalog::SurfaceId,
    surface_route::SurfaceRoute,
};
use rspice_app_workbench::workbench::state::workbench_session::{
    ConsolePage, Drawer, WorkbenchSession,
};

/// Every document a sheet claims must exist in the catalog and must accept
/// the sheet's analysis. Three copies of this map used to exist; one said
/// the noise sheet renders `viewer-spectrum`, whose catalog entry rejects a
/// `noise` analysis outright.
#[test]
fn every_claimed_viewer_document_is_in_the_catalog() {
    use rspice_app_results::workbench::documents::result_document::ResultViewer;
    for viewer in ResultViewer::all() {
        let id = viewer.viewer_document_id();
        assert!(
            rspice_app_state::results::viewer_catalog::viewer_document(id).is_some(),
            "{viewer:?} claims the unregistered document {id}"
        );
    }
    let noise = rspice_app_state::results::viewer_catalog::viewer_document("viewer-spectrum")
        .expect("the spectrum document is registered");
    assert!(
        !noise.analysis_ids.contains(&"noise"),
        "the spectrum document would accept noise, so this test proves nothing"
    );
}

#[test]
fn workspace_activation_updates_canonical_route_and_recent_history() {
    let mut state = WorkbenchSession::default();
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
fn the_dock_plots_only_on_the_three_authoring_stages() {
    let mut state = WorkbenchSession::default();

    for workspace in [Workspace::Design, Workspace::Netlist, Workspace::Simulate] {
        state.activate(workspace);
        assert!(state.plots_in_dock(), "{workspace:?}");
        assert!(state.offers_plot_page(true, true), "{workspace:?}");
    }

    for workspace in [
        Workspace::Project,
        Workspace::Results,
        Workspace::Verify,
        Workspace::Models,
    ] {
        state.activate(workspace);
        assert!(!state.plots_in_dock(), "{workspace:?}");
        assert!(!state.offers_plot_page(true, true), "{workspace:?}");
    }
}

#[test]
fn the_plot_page_is_never_offered_without_a_project_and_a_result() {
    let mut state = WorkbenchSession::default();
    state.activate(Workspace::Design);

    assert!(!state.offers_plot_page(false, true));
    assert!(!state.offers_plot_page(true, false));
    assert!(state.offers_plot_page(true, true));
}

/// A run opens the dock on its plot until the reader puts the plot away, by
/// collapsing the dock or by choosing another page; opening the plot again,
/// as a probe, Show plot or the Plot tab does, brings runs back to it.
#[test]
fn a_plot_the_reader_put_away_stays_away_until_they_open_it() {
    let mut state = WorkbenchSession::default();
    state.activate(Workspace::Design);
    assert!(state.run_opens_plot_page(), "the first run opens the plot");

    state.show_plot_page();
    assert!(state.dock_shows(ConsolePage::Plot));
    state.collapse_dock();
    assert!(!state.dock_shows(ConsolePage::Plot));
    assert!(
        !state.run_opens_plot_page(),
        "a collapsed dock stays collapsed"
    );

    state.show_plot_page();
    assert!(
        state.run_opens_plot_page(),
        "opening the plot brings runs back"
    );
    state.choose_dock_page(ConsolePage::Problems, true);
    assert!(
        !state.run_opens_plot_page(),
        "another page the reader chose stays chosen"
    );
    state.choose_dock_page(ConsolePage::Plot, true);
    assert!(state.run_opens_plot_page());

    // Choosing a page before there is a plot to offer refuses nothing.
    state.choose_dock_page(ConsolePage::Console, false);
    assert!(state.run_opens_plot_page());
}

#[test]
fn multi_step_return_reconciles_manager_and_workspace_once_at_final_route() {
    let mut state = WorkbenchSession::default();
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
            SurfaceRoute::surface(SurfaceId::HelpCenter),
            RouteTransitionSource::User,
        )
        .expect("the help center is available");
    state.clear_browser_history_effects();

    let transition = state
        .navigate_back_steps(2, RouteTransitionSource::User)
        .expect("manager and source routes exist");

    assert_eq!(
        transition.previous,
        SurfaceRoute::surface(SurfaceId::HelpCenter)
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
    let mut state = WorkbenchSession::default();
    let jobs = SurfaceRoute::surface(SurfaceId::JobsManager);
    let help = SurfaceRoute::surface(SurfaceId::HelpCenter);
    for route in [jobs, help] {
        state
            .navigate(route, RouteTransitionSource::User)
            .expect("the manager route has an executor");
    }

    assert_eq!(state.previous_route(), Some(jobs));
    assert_eq!(
        state
            .navigate_back(RouteTransitionSource::User)
            .expect("the help center retains its jobs source")
            .current,
        jobs
    );
    assert_eq!(
        state
            .navigate_forward_steps(1, RouteTransitionSource::User)
            .expect("forward restores the help center")
            .current,
        help
    );
}

#[test]
fn current_canonical_route_is_the_primary_workspace_source_of_truth() {
    let mut state = WorkbenchSession::default();
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
fn toggling_the_active_drawer_closes_it_deterministically() {
    let mut state = WorkbenchSession::default();

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
fn mixed_dock_composition_keeps_only_the_supported_drawer() {
    let mut state = WorkbenchSession::default();
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
fn fresh_browser_session_keeps_route_but_drops_restored_traversal() {
    let mut state = WorkbenchSession::default();
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
    let mut state = WorkbenchSession::default();
    state.navigation = serde_json::from_str(
        r#"{
                "current":"?view=design",
                "back":["?surface=release-cockpit","?surface=not-a-surface"],
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
    let mut state = WorkbenchSession::default();
    state.navigation = serde_json::from_str(
        r#"{
                "current":"?view=design",
                "back":["?surface=release-cockpit"],
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
    let mut state = WorkbenchSession::default();
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
fn workspace_presets_apply_to_the_live_layout_owner() {
    let mut state = WorkbenchSession::default();

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
    let mut state = WorkbenchSession::default();
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
    let mut state = WorkbenchSession::default();
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
    let mut state = WorkbenchSession::default();
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
fn full_screen_request_is_consumed_exactly_once() {
    let mut session = WorkbenchSession::default();
    session.outbox.push(FullScreen(true));

    assert_eq!(take_full_screen_request(&mut session), Some(true));
    assert_eq!(take_full_screen_request(&mut session), None);
}

#[test]
fn newest_full_screen_request_supersedes_an_unapplied_request() {
    let mut session = WorkbenchSession::default();
    session.outbox.push(FullScreen(true));
    session.outbox.push(FullScreen(false));

    assert_eq!(take_full_screen_request(&mut session), Some(false));
}

fn take_full_screen_request(session: &mut WorkbenchSession) -> Option<bool> {
    session
        .outbox
        .take::<FullScreen>()
        .map(|FullScreen(enabled)| enabled)
}

//! Where following a finding lands.
//!
//! The destinations a dispatch finding names are covered beside the report
//! that produces them, in `preflight::tests`. What is here is the hop itself:
//! a request recorded by a page is served once, by the frame boundary, and
//! the two routes whose destination has to be resolved rather than named — a
//! netlist record, and a source document in whichever editor owns its bundle.

use super::*;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;

fn context() -> Context {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

/// A request is served once. Serving it twice would re-open a destination the
/// reader has since navigated away from, on every frame after the click.
#[test]
fn a_requested_route_is_applied_once() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    app.state.diagnostics.request_route(DiagnosticRoute::Repair(
        PreflightRemediation::SimulationPlan,
    ));

    apply_pending(&context(), &mut app);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Simulate
    );

    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Results);
    apply_pending(&context(), &mut app);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Results,
        "a served request must not be served again",
    );
}

/// A source route resolves its own destination from the bundle's owner.
///
/// A bundle the project no longer holds says so rather than selecting a
/// document in whatever editor happened to be open — which is what carrying
/// only the language would have done, since the Verilog-A bundle in front of
/// the reader is whichever cell view is active.
#[test]
fn a_source_route_to_a_departed_bundle_states_its_refusal() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let before = app.state.workbench.workspace;
    app.state
        .diagnostics
        .request_route(DiagnosticRoute::CodeDocument {
            bundle: crate::state::ProjectSourceId::new(),
            logical_path: std::sync::Arc::from("amp.va"),
            line: Some(3),
        });

    apply_pending(&context(), &mut app);

    assert_eq!(
        app.state.workbench.workspace, before,
        "a route that cannot resolve must not move the reader"
    );
    let refusal = app
        .state
        .log_buffer
        .entries()
        .last()
        .expect("the refusal is stated")
        .message
        .clone();
    assert!(
        refusal.contains("amp.va") && refusal.contains("no longer part of this project"),
        "{refusal}"
    );
}

/// A netlist record is opened by its canonical identity, and an identity the
/// collection no longer holds is refused in the editor's own words.
#[test]
fn a_netlist_route_to_a_retired_record_states_its_refusal() {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state
        .diagnostics
        .request_route(DiagnosticRoute::NetlistDiagnostic(uuid::Uuid::nil()));

    apply_pending(&context(), &mut app);

    assert!(
        app.state
            .log_buffer
            .entries()
            .any(|entry| entry.severity == crate::diagnostics::LogSeverity::Warning),
        "an unresolvable record must say so rather than opening nothing silently"
    );
}

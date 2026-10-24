//! `workbench/app_state/diagnostic_store/current` observed through a whole session.

use crate::state::netlist_diagnostics::Diagnostic;
use crate::state::netlist_diagnostics::NetlistDiagnosticCollection;
use crate::workbench::app_state::AppState;
use std::sync::Arc;

fn collection(diagnostics: Vec<Diagnostic>) -> Arc<NetlistDiagnosticCollection> {
    Arc::new(
        NetlistDiagnosticCollection::try_new(diagnostics, "nchh\nr1 a b 1k\n")
            .expect("the fixture collection is well formed"),
    )
}

/// A netlist finding's identity names the buffer revision and its byte
/// range, so an edit above it re-mints it. The current finding follows
/// it to its new identity rather than being lost.
#[test]
fn the_current_netlist_finding_survives_a_revalidation_that_re_mints_it() {
    let mut state = AppState::default();
    let first = Diagnostic::error("unknown model nchh").with_line(Some(0));
    let other = Diagnostic::error("missing node").with_line(Some(1));
    let selected = first.canonical.diagnostic_id;
    state.code.netlist.diagnostics = collection(vec![first, other]);
    state.model.diagnostics.set_current_finding(Some(selected));
    state.reconcile_current_finding();
    assert_eq!(state.model.diagnostics.current_finding(), Some(selected));

    // An edit above it moves it down a line: same finding, new identity.
    let moved = Diagnostic::error("unknown model nchh").with_line(Some(1));
    let renamed = moved.canonical.diagnostic_id;
    assert_ne!(renamed, selected, "the fixture re-mints the identity");
    state.code.netlist.diagnostics = collection(vec![
        Diagnostic::error("missing node").with_line(Some(2)),
        moved,
    ]);
    state.reconcile_current_finding();

    assert_eq!(state.model.diagnostics.current_finding(), Some(renamed));
}

/// A finding that was fixed has nothing to follow, and the current
/// selection is not moved onto another finding.
#[test]
fn a_fixed_netlist_finding_hands_the_selection_to_nothing() {
    let mut state = AppState::default();
    let fixed = Diagnostic::error("unknown model nchh").with_line(Some(0));
    let selected = fixed.canonical.diagnostic_id;
    state.code.netlist.diagnostics = collection(vec![
        fixed,
        Diagnostic::error("missing node").with_line(Some(1)),
    ]);
    state.model.diagnostics.set_current_finding(Some(selected));
    state.reconcile_current_finding();

    state.code.netlist.diagnostics =
        collection(vec![Diagnostic::error("missing node").with_line(Some(0))]);
    state.reconcile_current_finding();

    assert_eq!(state.model.diagnostics.current_finding(), Some(selected));
}

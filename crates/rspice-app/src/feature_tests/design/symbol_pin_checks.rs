//! The symbol pin checks run through the whole application: their findings are published
//! on the Problems page, each routed to the pin that fails.

use super::app_state_of;
use crate::workbench::app_state::AppState;
use rspice_app_state::diagnostics::LogAnchor;
use rspice_app_state::state::{CellViewRef, Point};

/// [`state_with_amp_symbol_pin`](rspice_app_design::workbench::app::actions::workspace::tests::state_with_amp_symbol_pin) made a whole application.
fn state_with_amp_symbol_pin(pin_name: &str, position: Option<Point>) -> AppState {
    app_state_of(
        rspice_app_design::workbench::app::actions::workspace::tests::state_with_amp_symbol_pin(
            pin_name, position,
        ),
    )
}

/// A pin finding is a canonical record routed to the pin it is about.
///
/// The check used to stop at `dialogs.drc_results`, so its findings reached
/// no count, no listing and no selection: the Problems page claimed to be the
/// one list of every standing condition and had nothing of theirs on it.
#[test]
fn symbol_pin_checks_publish_records_routed_to_the_failing_pin() {
    use crate::workbench::app_state::diagnostic_store::{DiagnosticInputs, DiagnosticRoute};

    let mut state = state_with_amp_symbol_pin("IN", None);
    state.open_project_session();

    let facts = state.slot_facts();
    rspice_app_design::workbench::app::actions::workspace::symbol_document::run_active_symbol_pin_checks(
        &mut state.model,
        facts,
    );

    let subject = CellViewRef::new("work", "amp", "symbol");
    let batch = state
        .model
        .diagnostics
        .batches()
        .find(|batch| {
            batch.inputs
                == DiagnosticInputs::SymbolPins {
                    subject: subject.clone(),
                }
        })
        .expect("the pin checks publish their own batch");
    let unplaced = batch
        .records
        .iter()
        .find(|record| record.canonical.code.as_ref() == "SYMBOL-UNPLACED-PIN")
        .expect("the unplaced pin is one of the findings");
    assert_eq!(unplaced.subject.cell_view.as_ref(), Some(&subject));
    // Routed through the one resolver every "go to this finding" surface
    // shares, so following the row lands on the pin rather than on the
    // document that holds it.
    assert!(matches!(
        &unplaced.route,
        Some(DiagnosticRoute::Anchor(LogAnchor::Symbol { pin_name, .. })) if pin_name == "IN"
    ));

    // A pin the symbol has not placed is a warning: it never stops a save.
    let listed = crate::workbench::app_state::diagnostic_store::listing::listing(&state);
    assert_eq!(listed.counts().warnings, batch.records.len());
    let row = (0..listed.len())
        .filter_map(|index| listed.row(index))
        .find(|row| row.id == unplaced.canonical.diagnostic_id)
        .expect("the finding is on the page");
    assert_eq!(row.subject, "pin IN");
    assert_eq!(row.location, subject.display_path());
}

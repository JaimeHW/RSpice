//! A failed generation drawn through the application's context.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::{ComponentType, LibraryCellInstance, Point};
use crate::workbench::RSpiceApp;
use crate::workbench::surfaces::netlist::generation::refresh_generated_artifact;

/// A deck that cannot be written is built once per set of inputs, its
/// failure is recorded once, and the page states the generator's own
/// reason even with the Console's ring full.
///
/// The page used to recover that reason by reading the log entries a
/// build had added. Once the ring held its capacity, the count it skipped
/// past never grew, so it found nothing and the page said only that
/// generation had failed — while the build ran, and wrote its failure
/// again, every frame.
#[test]
fn a_failed_generation_states_its_reason_with_a_full_ring_and_is_not_rebuilt() {
    let mut app = RSpiceApp::test_instance();
    let mut absent = LibraryCellInstance::new("work", "absent", "schematic");
    absent.terminal_order = vec!["a".to_owned(), "b".to_owned()];
    app.state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), absent);
    app.state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));
    for index in 0..10_000 {
        app.state.model.log_buffer.log(
            LogSeverity::Info,
            LogSource::Studio,
            format!("line {index}"),
        );
    }

    refresh_generated_artifact(&mut crate::workbench::features::cx(&mut app));
    let error = app
        .state
        .model
        .netlist_artifacts
        .generation_error
        .clone()
        .expect("the failure is stated on the page");
    let newest = app
        .state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the failure is recorded")
        .clone();
    assert_eq!(newest.severity, LogSeverity::Error);
    assert_eq!(
        newest.message,
        format!("Netlist generation failed: {error}"),
        "the page states the generator's own reason, which the Console records"
    );
    let recorded = app.state.model.log_buffer.revision();

    for _ in 0..3 {
        refresh_generated_artifact(&mut crate::workbench::features::cx(&mut app));
    }
    assert_eq!(
        app.state.model.log_buffer.revision(),
        recorded,
        "the same inputs are neither rebuilt nor reported again"
    );
    assert_eq!(
        app.state
            .model
            .netlist_artifacts
            .generation_error
            .as_deref(),
        Some(error.as_str())
    );
}

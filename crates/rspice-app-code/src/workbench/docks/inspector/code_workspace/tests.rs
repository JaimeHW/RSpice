//! A diagnostic's place is stated one-based, and the generated deck's
//! provenance speaks only when it needs attention.

use super::*;
use crate::workbench::code::tests::harness;

#[test]
fn netlist_diagnostic_locations_are_one_based_and_exact() {
    let mut diagnostic = crate::state::netlist_diagnostics::Diagnostic::current(
        "rspice.test",
        "TEST-ADVISORY",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Warning,
        "Maximum transient step is implicit",
    );
    diagnostic.source_line = Some(127);
    diagnostic.line = Some(127);
    diagnostic.column = Some(8);

    assert_eq!(diagnostic_location(&diagnostic), "line 128 · column 9");
}

/// A deck that matches the schematic says nothing at all: the three states
/// left are the ones a designer has to act on.
#[test]
fn generated_provenance_speaks_only_when_the_deck_is_not_the_schematic() {
    let mut harness = harness();
    harness.model.netlist_artifacts.generated_source = "generated\n.end\n".to_owned();
    let input = crate::product::ContentDigest::from_bytes([0x31; 32]);
    harness.model.netlist_artifacts.generated_input_digest = Some(input);
    harness
        .model
        .netlist_artifacts
        .current_generation_input_digest = Some(input);
    assert_eq!(generated_state(&harness.model), None);

    harness
        .model
        .netlist_artifacts
        .current_generation_input_digest =
        Some(crate::product::ContentDigest::from_bytes([0x32; 32]));
    assert_eq!(
        generated_state(&harness.model),
        Some(("Older than the schematic", Problem::Warning))
    );

    harness.model.netlist_artifacts.generated_source.clear();
    assert_eq!(
        generated_state(&harness.model),
        Some(("Not generated yet", Problem::Warning))
    );
}

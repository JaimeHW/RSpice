//! Decks written for other simulators, opened as decks: each reaches the
//! engine, which reads it in the dialect its text names, and what the check
//! says is the engine's word on it. No deck is held back for a dialect
//! review first.

use crate::workbench::RSpiceApp;
use crate::workbench::features::cx;
use rspice_app_code::workbench::code::netlist_validation::validate_visible_netlist_source;

/// Open `source` as the deck file `name` in an application with no
/// schematic, check it as Run does, and give back what the check said: the
/// engine's refusal, or nothing.
fn checked(source: &str, name: &str) -> Option<String> {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert!(app.state.open_test_deck(source.to_owned(), None, name));
    if validate_visible_netlist_source(&mut cx(&mut app)) {
        assert_eq!(app.manual_deck_run_block_reason(), None, "{name} runs");
        return None;
    }
    Some(
        app.state
            .model
            .netlist_artifacts
            .validation_error
            .clone()
            .unwrap_or_default(),
    )
}

#[test]
fn an_ngspice_deck_with_a_control_block_reaches_the_engine() {
    let said = checked(
        "ngspice control deck\nV1 in 0 1\nR1 in 0 1k\n.control\nrun\nplot v(in)\n.endc\n.op\n.end\n",
        "control.cir",
    );
    assert_eq!(said, None);
}

#[test]
fn an_hspice_deck_with_post_and_alter_reaches_the_engine() {
    let said = checked(
        "hspice deck\n.option post=2\n.param r=1k\nV1 in 0 1\nR1 in 0 r\n.op\n.alter\n.param r=2k\n.end\n",
        "alter.sp",
    );
    assert_eq!(said, None);
}

/// A Spectre deck is read by the engine's Spectre reader. Whatever it says
/// of the deck is its own finding on a line of the deck, never a hold for a
/// dialect review.
#[test]
fn a_spectre_deck_reaches_the_engine_and_is_never_held_for_review() {
    let said = checked(
        "// spectre deck\nsimulator lang=spectre\nV1 (in 0) vsource dc=1\nR1 (in 0) resistor r=1k\nop1 dc\n",
        "amp.scs",
    );
    if let Some(said) = said {
        assert!(!said.to_ascii_lowercase().contains("profile"), "{said}");
        assert!(said.contains("line"), "{said}");
    }
}

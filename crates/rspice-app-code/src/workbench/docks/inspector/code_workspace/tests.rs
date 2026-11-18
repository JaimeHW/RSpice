//! A diagnostic's place is stated one-based, a finding's fix stands under
//! it, and the generated deck's provenance speaks only when it needs
//! attention.

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

    let messages = MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates);
    assert_eq!(
        diagnostic_location(&diagnostic, messages),
        "line 128 · column 9"
    );
}

/// A finding that offers a fix shows it under its row, labelled as its
/// Problems row labels it; pressing it asks for that finding's fix, and
/// pressing the row still goes to its line.
#[test]
fn a_finding_offers_its_fix_under_its_row() {
    let mut diagnostic = crate::state::netlist_diagnostics::Diagnostic::current(
        "rspice.test",
        "TEST-MISSING-FILE",
        crate::workbench::documents::netlist_document::DiagnosticSeverity::Error,
        "models/opamp.inc is not in this deck. Add the file to the deck to use it.",
    );
    diagnostic.fix = Some(crate::state::netlist_diagnostics::DiagnosticFix {
        label: "Add file\u{2026}".to_owned(),
        span: 0..0,
        replacement: String::new(),
        add_file: None,
    });
    diagnostic.refresh_canonical_location();
    let finding = diagnostic.canonical.diagnostic_id;
    let messages = MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let frame = |events: Vec<egui::Event>| {
        let mut pressed = None;
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(320.0, 400.0))),
                events,
                ..egui::RawInput::default()
            },
            |ui| pressed = diagnostic_row(ui, &diagnostic, messages),
        );
        (pressed, output)
    };
    let (_, output) = frame(Vec::new());
    let bounds = output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Add file\u{2026}")
        })
        .and_then(|(_, node)| node.bounds())
        .expect("the fix stands under the finding");
    // Over it a frame, down a frame, and up: what the press asked for.
    let press = |at: Pos2| {
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let _ = frame(vec![egui::Event::PointerMoved(at)]);
        let _ = frame(vec![button(true)]);
        frame(vec![button(false)]).0
    };

    let fix = Pos2::new(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    assert_eq!(press(fix), Some(FindingPress::Fix(finding)));
    assert_eq!(
        press(Pos2::new(fix.x, fix.y - 20.0)),
        Some(FindingPress::Row(finding))
    );
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
        Some((MessageId::NetlistOlderThanSchematic, Problem::Warning))
    );

    harness.model.netlist_artifacts.generated_source.clear();
    assert_eq!(
        generated_state(&harness.model),
        Some((MessageId::NetlistNotGeneratedYet, Problem::Warning))
    );
}

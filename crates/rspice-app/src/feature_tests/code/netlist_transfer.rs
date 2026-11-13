//! The netlist transfer forms drawn through the application's context.

use crate::workbench::surfaces::netlist::transfer::export_generated_dialog_window;

/// Export is a command form: the dialect, what goes with the deck, and
/// Export…; a refusal is its status. It used to open on a boxed paragraph
/// about snapshots, close on a box saying "0 external dependencies ·
/// sealed" and "accompanied by an integrity manifest", and repeat the
/// count in the footer.
#[test]
fn export_is_a_command_form_without_the_sealing_boxes() {
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.code.netlist.export_dialog.open = true;
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ui| {
                export_generated_dialog_window(
                    ui.ctx(),
                    &mut crate::workbench::features::cx(&mut app),
                )
            },
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let names = nodes
        .values()
        .flat_map(|node| [node.label(), node.value()])
        .flatten()
        .collect::<Vec<_>>();
    let says = |text: &str| names.iter().any(|name| name.contains(text));
    assert!(says("Dialect"), "{names:?}");
    assert!(says("Export\u{2026}"), "the primary is not Export…");
    assert!(says("Cancel"));
    for jargon in ["sealed", "integrity", "snapshot", "dependency members"] {
        assert!(!says(jargon), "the dialog says {jargon:?}");
    }
}

/// A refusal is one line in the footer beside Cancel and Export…: the
/// netlist that could not be generated used to say so in two.
#[test]
fn every_refusal_is_one_footer_line() {
    let refusals = [
        (
            "Fix the netlist's problems, then export.",
            Some("unresolved model"),
            None,
        ),
        ("No netlist has been generated yet.", None, None),
        (
            "Export failed: see the application log.",
            None,
            Some("Export failed: see the application log."),
        ),
    ];
    for (refusal, generation_error, export_error) in refusals {
        let mut app = crate::workbench::RSpiceApp::test_instance();
        app.state.code.netlist.export_dialog.open = true;
        app.state.code.netlist.export_dialog.error = export_error.map(str::to_owned);
        app.state.model.netlist_artifacts.generation_error = generation_error.map(str::to_owned);
        let painted = crate::ui::widgets::painted_runs::painted_runs(
            egui::vec2(1440.0, 900.0),
            crate::ui::tokens::Mode::Light,
            3,
            |ctx| {
                export_generated_dialog_window(ctx, &mut crate::workbench::features::cx(&mut app))
            },
        );
        let line = painted
            .rect_of("Dialect")
            .expect("the Dialect row")
            .height();
        let rect = painted
            .rect_of(refusal)
            .unwrap_or_else(|| panic!("{refusal} is painted"));
        assert!(
            rect.height() < 1.5 * line,
            "{refusal:?} is {} tall, a line is {line}",
            rect.height()
        );
    }
}

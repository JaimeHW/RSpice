//! The netlist transfer forms drawn through the application's context.

use rspice_app_code::workbench::surfaces::netlist::transfer::export_generated_dialog_window;

/// A refusal is one line in the footer beside Cancel and Export…: the
/// netlist that could not be generated used to say so in two.
#[test]
fn every_refusal_is_one_footer_line() {
    let refusals = [
        (
            "Fix the netlist's errors, then export.",
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
        let painted = rspice_app_workbench::ui::widgets::painted_runs::painted_runs(
            egui::vec2(1440.0, 900.0),
            rspice_app_workbench::ui::tokens::Mode::Light,
            3,
            |ctx| {
                export_generated_dialog_window(ctx, &mut crate::workbench::features::cx(&mut app))
            },
        );
        let line = painted
            .rect_of("Source map")
            .expect("the Source map option")
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

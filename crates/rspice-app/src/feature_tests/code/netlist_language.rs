//! The deck's language tools, observed through a whole application: where
//! a finding of the deck is listed and where following it lands.

use crate::workbench::RSpiceApp;
use rspice_app_code::workbench::documents::netlist_document;
use rspice_app_state::workbench::app_state::diagnostic_store::listing::listing;
use rspice_app_state::workbench::workspace::Workspace;

/// Draw the Netlist page's editor over `app` for `frames` frames.
fn draw_editor(ctx: &egui::Context, app: &mut RSpiceApp, frames: usize) {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1024.0, 768.0),
        )),
        ..Default::default()
    };
    for _ in 0..frames {
        let _ = ctx.clone().run_ui(input.clone(), |ui| {
            let state = &mut app.state;
            netlist_document::show_editor(
                ui,
                &mut state.model,
                &mut state.code.netlist,
                &state.code.code_workspace.include_resolutions,
                &state.runs,
                &mut state.workbench,
            );
        });
    }
}

/// A finding of the deck is listed under the deck's own file name, never
/// the name the editor's check reads it under, and following its row puts
/// the caret on its line.
#[test]
fn a_deck_finding_names_the_decks_file_and_its_row_goes_to_its_line() {
    let deck = "amp\nM1 d g s b nchh W=1u L=1u\n.model nch nmos\n.end\n";
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(deck.to_owned(), None, "amp_tb.cir"));
    app.state.workbench.workspace = Workspace::Netlist;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    draw_editor(&ctx, &mut app, 1);

    let listed = listing(&app.state);
    let row = (0..listed.len())
        .filter_map(|index| listed.row(index))
        .find(|row| row.message.contains("nchh"))
        .expect("the unknown model is listed");
    assert_eq!(row.subject, "amp_tb.cir");
    let route = row.route.clone().expect("a current finding goes to its line");

    app.state.code.netlist.cursor_line = 3;
    app.state.model.diagnostics.request_route(route);
    crate::workbench::diagnostic_routes::apply_pending(&ctx, &mut app);
    assert_eq!(app.state.prompt_text(), None);
    assert_eq!(app.state.code.netlist.cursor_line, 1);
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
}

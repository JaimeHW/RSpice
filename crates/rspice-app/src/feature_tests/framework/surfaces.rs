//! The workspace surfaces: a source write the bundle refuses leaves
//! the editor as the bundle holds the source.

use crate::workbench::RSpiceApp;
use egui::Context;
use rspice_app_code::workbench::documents::source_editing;

/// An application on the page that edits a Verilog-A source, and the owner
/// of the bundle it edits.
fn source_editor_page() -> (
    Context,
    RSpiceApp,
    rspice_app_state::state::ProjectSourceOwner,
) {
    let ctx = Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut app = RSpiceApp::test_instance();
    // A Verilog-A module is a cell view, edited in Design.
    let owner = {
        let reference = rspice_app_state::state::CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = rspice_app_state::state::View::new(
            "veriloga",
            rspice_app_state::state::ViewType::VerilogA,
        );
        view.metadata
            .insert("veriloga.module".to_owned(), "gain".to_owned());
        let mut cell = rspice_app_state::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = rspice_app_state::state::Library::new("behavioral");
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        let owner = rspice_app_state::state::ProjectSourceOwner::cell_view(reference.clone());
        app.state
            .model.workspace
            .content
            .project_sources
            .insert_bundle(
                rspice_app_state::state::ProjectSourceBundle::try_new(
                    owner.clone(),
                    "gain.va",
                    "module gain(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n);\nendmodule\n",
                    [],
                    [],
                )
                .unwrap(),
            )
            .unwrap();
        app.state.open_workspace_view(reference);
        app.state
            .workbench
            .activate(rspice_app_state::workbench::workspace::Workspace::Design);
        owner
    };
    (ctx, app, owner)
}

/// One frame of the page, with `events`.
fn render_page(ctx: &Context, app: &mut RSpiceApp, events: Vec<egui::Event>) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ui| {
            app.handle_shortcuts(ui);
            rspice_app_workbench::workbench::surfaces::show(ui, app);
            app.render_frame_dialogs(ui);
        },
    );
}

/// A write the source bundle refuses leaves the editor as the bundle holds
/// the source: the next frame draws the stored text with the caret where it
/// was, and the next edit is stored without what was refused.
#[test]
fn a_refused_source_write_returns_the_editor_to_the_stored_source() {
    refused_source_write();
}

fn refused_source_write() {
    let (ctx, mut app, owner) = source_editor_page();
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .unwrap();
    let key = (bundle.id(), bundle.root().logical_path().to_owned());
    let editor_id = source_editing::source_editor_id(key.0, &key.1);
    let stored = |app: &RSpiceApp| {
        app.state
            .model
            .workspace
            .content
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap()
            .root()
            .content()
            .to_owned()
    };
    let caret = |app: &RSpiceApp| {
        app.state
            .code
            .source_editing
            .source_carets
            .get(&key)
            .copied()
    };
    let insert = |text: &str, at: usize, inserted: &str| {
        let mut text = text.to_owned();
        let byte = text
            .char_indices()
            .nth(at)
            .map_or(text.len(), |(byte, _)| byte);
        text.insert_str(byte, inserted);
        text
    };
    let original = stored(&app);
    render_page(&ctx, &mut app, Vec::new());
    render_page(&ctx, &mut app, Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(editor_id));
    let at = caret(&app).expect("the page reports its caret");
    render_page(&ctx, &mut app, vec![egui::Event::Text("x".to_owned())]);
    let typed = insert(&original, at, "x");
    assert_eq!(stored(&app), typed, "Verilog-A: the typing is stored");
    assert_eq!(caret(&app), Some(at + 1));

    // The bundle refuses a pasted NUL.
    render_page(&ctx, &mut app, vec![egui::Event::Paste("\u{0}".to_owned())]);
    assert_eq!(stored(&app), typed, "Verilog-A: the paste is refused");
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|prompt| prompt.starts_with("Could not update")),
        "Verilog-A: the frame of the refusal says so"
    );

    render_page(&ctx, &mut app, Vec::new());
    assert_eq!(
        rspice_app_code::workbench::documents::virtual_text_editor::shown_source(&ctx, editor_id),
        Some(typed.clone()),
        "Verilog-A: the next frame draws the stored source"
    );
    assert_eq!(
        caret(&app),
        Some(at + 1),
        "Verilog-A: the caret is where it was before the paste"
    );

    render_page(&ctx, &mut app, vec![egui::Event::Text("y".to_owned())]);
    assert_eq!(
        stored(&app),
        insert(&typed, at + 1, "y"),
        "Verilog-A: the next edit is stored without the refused paste"
    );
}

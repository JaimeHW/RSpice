//! The workspace surfaces: the search shortcut's hand-off keeps the source
//! editor and the edits before it, a source write the bundle refuses leaves
//! the editor as the bundle holds the source, and the Results split stage
//! stays equal, bounded at narrow widths and named for assistive technology.

use crate::state::ProjectSourceLanguage;
use crate::workbench::RSpiceApp;
use crate::workbench::documents::code_workspace::{self, CodeWorkspacePage};
use crate::workbench::surfaces::{register_results_split_region, split_stage_rects};
use egui::Context;

#[test]
fn search_shortcut_handoff_preserves_the_source_editor() {
    for language in [
        ProjectSourceLanguage::RSpiceAutomation,
        ProjectSourceLanguage::VerilogA,
    ] {
        source_editor_handoff(language, "");
    }
}

#[test]
fn search_shortcut_handoff_binds_preceding_source_edits() {
    for language in [
        ProjectSourceLanguage::RSpiceAutomation,
        ProjectSourceLanguage::VerilogA,
    ] {
        source_editor_handoff(language, "preceding_source_edit");
    }
}

fn source_editor_handoff(language: ProjectSourceLanguage, preceding: &str) {
    let (ctx, mut app, owner) = source_editor_page(language);
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .unwrap();
    let editor_id =
        code_workspace::source_editor_id(language, bundle.id(), bundle.root().logical_path());
    let original = bundle.root().content().to_owned();
    let sources = app.state.model.workspace.content.project_sources.clone();
    let mut render = |events| render_page(&ctx, &mut app, events);
    render(Vec::new());
    render(Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(editor_id));
    let key = |key, modifiers| egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers,
    };
    render(vec![
        egui::Event::Text(preceding.to_owned()),
        key(
            egui::Key::H,
            egui::Modifiers::CTRL | egui::Modifiers::COMMAND | egui::Modifiers::SHIFT,
        ),
        egui::Event::Text("wrong".to_owned()),
        key(
            egui::Key::A,
            egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
        ),
        egui::Event::Paste("import".to_owned()),
    ]);
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .unwrap();
    if preceding.is_empty() {
        assert!(
            app.state.model.workspace.content.project_sources == sources,
            "search text must not edit {language:?} source code"
        );
    } else {
        assert!(
            bundle.root().content().contains(preceding),
            "preceding {language:?} source edit was lost"
        );
        assert_eq!(bundle.root().content().replacen(preceding, "", 1), original);
    }
    let search = app
        .state
        .code
        .code_workspace
        .source_search
        .as_ref()
        .expect("bundle search opened from the editor shortcut");
    assert_eq!(search.query, "import", "{language:?} opening query");
    assert_eq!(
        search.bundle_id,
        bundle.id(),
        "{language:?} search binds the bundle the editor is showing"
    );
    assert_eq!(search.bundle_revision, bundle.revision().get());
    assert!(code_workspace::source_search_results(&app, search).is_ok());
}

/// An application on the page that edits a `language` source, and the owner
/// of the bundle it edits.
fn source_editor_page(
    language: ProjectSourceLanguage,
) -> (Context, RSpiceApp, crate::state::ProjectSourceOwner) {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let mut app = RSpiceApp::test_instance();
    // Automation is edited on its Netlist-workspace page; a Verilog-A
    // module is a cell view, edited in Design.
    let owner = match language {
        ProjectSourceLanguage::RSpiceAutomation => {
            app.state
                .workbench
                .activate(crate::workbench::state::Workspace::Netlist);
            app.state.code.code_workspace.page = CodeWorkspacePage::Automation;
            crate::state::ProjectSourceOwner::code_workspace(language)
        }
        ProjectSourceLanguage::VerilogA => {
            let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
            let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
            view.metadata
                .insert("veriloga.module".to_owned(), "gain".to_owned());
            let mut cell = crate::state::Cell::new("gain");
            cell.add_view(view);
            let mut library = crate::state::Library::new("behavioral");
            library.add_cell(cell);
            app.state.model.library_manager.add_library(library);
            let owner = crate::state::ProjectSourceOwner::cell_view(reference.clone());
            app.state
                .model.workspace
                .content
                .project_sources
                .insert_bundle(
                    crate::state::ProjectSourceBundle::try_new(
                        owner.clone(),
                        language,
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
                .activate(crate::workbench::state::Workspace::Design);
            owner
        }
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
            crate::workbench::surfaces::show(ui, app);
            app.render_frame_dialogs(ui);
        },
    );
}

/// A write the source bundle refuses leaves the editor as the bundle holds
/// the source: the next frame draws the stored text with the caret where it
/// was, and the next edit is stored without what was refused.
#[test]
fn a_refused_source_write_returns_the_editor_to_the_stored_source() {
    for language in [
        ProjectSourceLanguage::RSpiceAutomation,
        ProjectSourceLanguage::VerilogA,
    ] {
        refused_source_write(language);
    }
}

fn refused_source_write(language: ProjectSourceLanguage) {
    let (ctx, mut app, owner) = source_editor_page(language);
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&owner)
        .unwrap();
    let key = (bundle.id(), bundle.root().logical_path().to_owned());
    let editor_id = code_workspace::source_editor_id(language, key.0, &key.1);
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
            .code_workspace
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
    assert_eq!(stored(&app), typed, "{language:?}: the typing is stored");
    assert_eq!(caret(&app), Some(at + 1));

    // The bundle refuses a pasted NUL.
    render_page(&ctx, &mut app, vec![egui::Event::Paste("\u{0}".to_owned())]);
    assert_eq!(stored(&app), typed, "{language:?}: the paste is refused");
    assert!(
        app.state
            .prompt_text()
            .is_some_and(|prompt| prompt.starts_with("Could not update")),
        "{language:?}: the frame of the refusal says so"
    );

    render_page(&ctx, &mut app, Vec::new());
    assert_eq!(
        crate::workbench::documents::virtual_text_editor::shown_source(&ctx, editor_id),
        Some(typed.clone()),
        "{language:?}: the next frame draws the stored source"
    );
    assert_eq!(
        caret(&app),
        Some(at + 1),
        "{language:?}: the caret is where it was before the paste"
    );

    render_page(&ctx, &mut app, vec![egui::Event::Text("y".to_owned())]);
    assert_eq!(
        stored(&app),
        insert(&typed, at + 1, "y"),
        "{language:?}: the next edit is stored without the refused paste"
    );
}

#[test]
fn split_stage_is_equal_zero_gap_with_one_pixel_divider() {
    let stage = egui::Rect::from_min_size(egui::pos2(7.0, 11.0), egui::vec2(1001.0, 620.0));
    let (primary, divider, secondary) = split_stage_rects(stage);

    assert_eq!(primary.left(), stage.left());
    assert_eq!(primary.right(), divider.left());
    assert_eq!(divider.width(), 1.0);
    assert_eq!(divider.right(), secondary.left());
    assert_eq!(secondary.right(), stage.right());
    assert!((primary.width() - secondary.width()).abs() <= f32::EPSILON);
    assert_eq!(primary.height(), stage.height());
    assert_eq!(secondary.height(), stage.height());
}

#[test]
fn split_stage_remains_bounded_at_narrow_widths() {
    for width in [0.0, 0.5, 1.0, 2.0, 44.0] {
        let stage = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, 80.0));
        let (primary, divider, secondary) = split_stage_rects(stage);
        for rect in [primary, divider, secondary] {
            assert!(rect.left() >= stage.left());
            assert!(rect.right() <= stage.right());
            assert!(rect.width() >= 0.0);
        }
    }
}

#[test]
fn results_secondary_pane_is_an_accessible_named_region() {
    let ctx = egui::Context::default();
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(800.0, 600.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                register_results_split_region(ui, rect);
            });
        },
    );

    let nodes = output
        .platform_output
        .accesskit_update
        .expect("split accessibility tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Region && node.label() == Some("Results split pane")
    }));
}

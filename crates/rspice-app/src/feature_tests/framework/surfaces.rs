//! The workspace surfaces: the search shortcut's hand-off keeps the source
//! editor and the edits before it, and the Results split stage stays equal,
//! bounded at narrow widths and named for assistive technology.

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
    let mut render = |events| {
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
                crate::workbench::surfaces::show(ui, &mut app);
                app.render_source_find_dialog(ui);
            },
        );
    };
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

//! The source language tools through the whole application: a compile
//! refused while one is pending, and a confirmed rename's outcome drawn on
//! the next frame.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::source_language_tools::RENAME_TITLE;
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::documents::code_workspace;
use egui::Context;

/// `resolve_veriloga_compile` was written to gate the compile command and
/// never attached to it, so the button stayed enabled while a compile was
/// already running and the surface then refused the request.
#[test]
fn compiling_verilog_a_is_refused_while_a_compile_is_already_pending() {
    let mut app = RSpiceApp::test_instance();
    // A Verilog-A cell view open in Design: the place Compile is offered.
    let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
    let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    let mut cell = crate::state::Cell::new("gain");
    cell.add_view(view);
    let mut library = crate::state::Library::new("behavioral");
    library.add_cell(cell);
    app.state.model.library_manager.add_library(library);
    app.state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(
            crate::state::ProjectSourceBundle::try_new(
                crate::state::ProjectSourceOwner::cell_view(reference.clone()),
                crate::state::ProjectSourceLanguage::VerilogA,
                "gain.va",
                "module gain(p, n); inout p, n; electrical p, n; endmodule\n",
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
    let enabled_before = app.command_key_enabled(CodeCommand::CompileVerilogA.key());
    assert!(
        enabled_before,
        "a writable project compiles the Verilog-A cell view in front"
    );

    app.state.model.safe_mode.active = true;
    app.state.model.safe_mode.applied.open_project_read_only = true;
    assert!(
        !app.command_key_enabled(CodeCommand::CompileVerilogA.key()),
        "a read-only project cannot compile; it was enabled before as {enabled_before}"
    );
}

/// An Automation project whose entry defines `helper` and whose second
/// file, `shared.py`, uses it; `shared.py` is the environment lock when
/// `locked`, so a rename may change the entry but not it. The Rename
/// form is open on `helper` with `assist` typed.
fn rename_form_over_two_files(locked: bool) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.code.code_workspace.page =
        crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
    let language = crate::state::ProjectSourceLanguage::RSpiceAutomation;
    let owner = crate::state::ProjectSourceOwner::code_workspace(language);
    let content = &mut app.state.model.workspace.content;
    let bundle = content
        .project_sources
        .bundle_for_owner(&owner)
        .expect("the fixture project owns an Automation bundle");
    let (bundle_id, root) = (bundle.id(), bundle.root().logical_path().to_owned());
    let locks = bundle
        .paths_for_role(crate::state::ProjectSourceRole::AutomationEnvironmentLock)
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let entry = "def helper():
return 1

helper()
";
    content
        .replace_project_source_bundle_files_transactionally(
            bundle_id,
            [(root.clone(), entry.to_owned())],
        )
        .expect("the entry is rewritten");
    content
        .add_project_source_bundle_file(
            bundle_id,
            &root,
            crate::state::ProjectSourceFile::try_new(
                "shared.py",
                "value = helper()
",
            )
            .expect("a valid file"),
        )
        .expect("the second file joins the bundle");
    if locked {
        for lock in &locks {
            content
                .set_project_source_bundle_non_entry_role(bundle_id, lock, None)
                .expect("the old lock is released");
        }
        content
            .set_project_source_bundle_non_entry_role(
                bundle_id,
                "shared.py",
                Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock),
            )
            .expect("the second file is the lock");
    }
    code_workspace::open_language_tools_in_bundle(
        &mut app.code_workspace_parts(),
        language,
        bundle_id,
        &root,
        "def h".chars().count(),
    )
    .expect("the language tools open on helper");
    let tools = app
        .state
        .code
        .code_workspace
        .language_tools
        .as_mut()
        .expect("the form is open");
    assert_eq!(tools.query, "helper");
    tools.replacement = "assist".to_owned();
    app
}

/// One frame of the frame's dialogs: whether it asked for the next one
/// straight away, and every label and value it put on the screen.
fn dialog_frame(
    ctx: &Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> (bool, Vec<String>) {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ui| app.render_frame_dialogs(ui.ctx()),
    );
    let immediate = output
        .viewport_output
        .get(&egui::ViewportId::ROOT)
        .is_some_and(|viewport| viewport.repaint_delay.is_zero());
    let text = output
        .platform_output
        .accesskit_update
        .into_iter()
        .flat_map(|update| update.nodes)
        .flat_map(|(_, node)| {
            [node.label(), node.value()]
                .into_iter()
                .flatten()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect();
    (immediate, text)
}

/// The rename is carried out after the form has drawn, as the frame's
/// dialogs answer what they raised, so its outcome is drawn on the frame
/// after the one that confirmed it. That frame must come with no input
/// in between: a refused rename states why on the form, and one that
/// applies has closed it.
#[test]
fn a_confirmed_rename_shows_its_outcome_on_the_next_frame_without_input() {
    let enter = egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: Some(egui::Key::Enter),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    };
    for locked in [true, false] {
        let mut app = rename_form_over_two_files(locked);
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        for _ in 0..2 {
            let (_, text) = dialog_frame(&ctx, &mut app, Vec::new());
            assert!(
                text.iter().any(|text| text == RENAME_TITLE),
                "the form is open"
            );
        }
        let (immediate, _) = dialog_frame(&ctx, &mut app, vec![enter.clone()]);
        assert!(
            immediate,
            "the frame that confirmed the rename asks for the next one"
        );
        let (_, text) = dialog_frame(&ctx, &mut app, Vec::new());
        let open = text.iter().any(|text| text == RENAME_TITLE);
        if locked {
            assert!(open, "a refused rename leaves the form open");
            assert!(
                text.iter()
                    .any(|text| text.contains("governed read-only source 'shared.py'")),
                "the refusal is on the form: {text:?}"
            );
        } else {
            assert!(!open, "a rename that applied closed the form");
            assert!(
                app.state.code.code_workspace.language_tools.is_none(),
                "the transaction is done"
            );
        }
    }
}

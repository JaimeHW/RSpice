//! A Verilog-A cell view is a Design-workspace document.
//!
//! These pins drive the application the way a reader does: open the view,
//! render frames, run commands. They live with the application root because
//! that is the one layer allowed to reach the frame loop, the commands and the
//! document engines together.

use std::sync::{Arc, Mutex, mpsc};

use super::*;
use crate::state::{
    Cell, CellViewRef, Library, ProjectSourceBundle, ProjectSourceId, ProjectSourceLanguage,
    ProjectSourceOwner, View, ViewType,
};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::documents::code_workspace::{
    self, LanguageToolView, PendingVerilogACompile, VerilogACompileOutcome,
};
use crate::workbench::state::Workspace;

const LIBRARY: &str = "behavioral";

/// A library cell whose `veriloga` view owns a one-file bundle at `root`.
fn add_veriloga_cell(
    state: &mut AppState,
    cell: &str,
    root: &str,
    module: &str,
) -> (CellViewRef, ProjectSourceId) {
    if state.library_manager.get_library(LIBRARY).is_none() {
        state.library_manager.add_library(Library::new(LIBRARY));
    }
    let reference = CellViewRef::new(LIBRARY, cell, "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), module.to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut library_cell = Cell::new(cell);
    library_cell.add_view(view);
    state
        .library_manager
        .get_library_mut(LIBRARY)
        .expect("the behavioral library")
        .add_cell(library_cell);
    let bundle = ProjectSourceBundle::try_new(
        ProjectSourceOwner::cell_view(reference.clone()),
        ProjectSourceLanguage::VerilogA,
        root,
        module_source(module, ""),
        [],
        [],
    )
    .expect("a valid cell-view bundle");
    let id = bundle.id();
    state
        .workspace
        .content
        .project_sources
        .insert_bundle(bundle)
        .expect("one bundle per cell view");
    (reference, id)
}

fn module_source(module: &str, extra: &str) -> String {
    format!(
        "// Behavioral conductance.\nmodule {module}(p, n);\n  inout p, n;\n  electrical p, n;\n  parameter real g = 1m;\n{extra}  analog I(p, n) <+ g * V(p, n);\nendmodule\n"
    )
}

fn open_project_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app
}

/// Open `reference` the way the library and the schematic do, then show it.
fn open_in_design(state: &mut AppState, reference: &CellViewRef) {
    state.open_workspace_view(reference.clone());
    state.workbench.activate(Workspace::Design);
}

fn singleton_bundle(app: &RSpiceApp) -> Option<ProjectSourceId> {
    app.state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::VerilogA,
        ))
        .map(ProjectSourceBundle::id)
}

fn editor_id(bundle_id: ProjectSourceId, logical_path: &str) -> egui::Id {
    code_workspace::source_editor_id(ProjectSourceLanguage::VerilogA, bundle_id, logical_path)
}

/// One whole application frame: the frame loop's preparation, the chrome,
/// the docks and the surface, and the dialogs.
fn frame(ctx: &egui::Context, app: &mut RSpiceApp, size: egui::Vec2) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ui| {
            app.prepare_frame(ui.ctx());
            app.render_frame_chrome(ui);
            app.render_frame_dialogs(ui.ctx());
        },
    )
}

fn themed_context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

/// Whether the frame put an editable Verilog-A source editor in front of the
/// reader — what a screen reader announces, so what a reader is offered.
fn shows_veriloga_editor(output: &egui::FullOutput) -> bool {
    let label =
        crate::workbench::MessageCatalog::new(crate::workbench::UiTextLocale::EnglishUnitedStates)
            .text(crate::workbench::MessageId::EditorVerilogASourceEditor);
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .is_some_and(|update| {
            update.nodes.iter().any(|(_, node)| {
                node.label() == Some(label.as_str()) || node.value() == Some(label.as_str())
            })
        })
}

const DESKTOP: egui::Vec2 = egui::vec2(1440.0, 900.0);

fn tab_is_dirty(app: &RSpiceApp, reference: &CellViewRef) -> bool {
    app.state
        .workspace
        .content
        .open_views
        .iter()
        .find(|open| open.reference == *reference)
        .is_some_and(|open| open.dirty)
}

/// The Design workspace used to draw a stub reading a metadata key nothing
/// writes, so every Verilog-A tab said "No source text stored". It draws the
/// editor over the view's own bundle.
#[test]
fn a_veriloga_cell_view_opens_in_design_with_its_editor() {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();

    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert!(
        shows_veriloga_editor(&output),
        "the Design surface shows the source editor"
    );
    assert!(
        app.state
            .ui
            .code_workspace
            .source_carets
            .contains_key(&(bundle_id, "model.va".to_owned())),
        "the editor is the one over the view's own bundle"
    );
}

/// Find used to resolve the project's code-workspace bundle whatever was on
/// screen, so Ctrl+F in a cell view searched a different module.
#[test]
fn find_in_a_veriloga_cell_view_searches_that_views_bundle() {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    assert!(singleton_bundle(&app).is_some_and(|singleton| singleton != bundle_id));

    assert!(Command::FindCodeDocument.is_enabled(&app));
    Command::FindCodeDocument.execute(&mut app);

    let search = app
        .state
        .ui
        .code_workspace
        .source_search
        .as_ref()
        .expect("the bundle search opened");
    assert_eq!(search.bundle_id, bundle_id);
    assert_eq!(search.language, ProjectSourceLanguage::VerilogA);
    assert_eq!(search.active_path, "model.va");
    assert!(
        !app.state.ui.netlist.find.open,
        "the deck's find window is not the cell view's"
    );
}

/// The language tools index the bundle in front of the reader.
#[test]
fn language_tools_in_a_veriloga_cell_view_index_that_views_bundle() {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);

    assert!(Command::SourceLanguageTools.is_enabled(&app));
    Command::SourceLanguageTools.execute(&mut app);

    let tools = app
        .state
        .ui
        .code_workspace
        .language_tools
        .as_ref()
        .expect("the tools opened");
    assert_eq!(tools.index.bundle_id, bundle_id);
    assert_eq!(tools.active_path, "model.va");
}

/// Format is the editor's request, and it reaches the tools over the bundle
/// the editor is showing.
#[test]
fn format_in_a_veriloga_cell_view_acts_on_that_views_bundle() {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);

    crate::workbench::documents::text_editor_commands::queue_format_document(
        &ctx,
        editor_id(bundle_id, "model.va"),
    );
    frame(&ctx, &mut app, DESKTOP);

    let tools = app
        .state
        .ui
        .code_workspace
        .language_tools
        .as_ref()
        .expect("Format opened the document actions");
    assert_eq!(tools.index.bundle_id, bundle_id);
    assert_eq!(tools.view, LanguageToolView::CodeActions);
}

/// Carets used to be keyed by language and path, so two cell views that each
/// own a `model.va` overwrote one another's place.
#[test]
fn two_cell_views_sharing_a_logical_path_keep_separate_carets() {
    let mut app = open_project_app();
    let (first, first_bundle) = add_veriloga_cell(&mut app.state, "first", "model.va", "first");
    let (second, second_bundle) = add_veriloga_cell(&mut app.state, "second", "model.va", "second");
    let ctx = themed_context();
    // The editor's own caret request, applied by the editor on its next frame
    // and reported back the way a click would be.
    let place_caret = |ctx: &egui::Context, bundle_id: ProjectSourceId, index: usize| {
        crate::workbench::documents::text_editor_commands::queue_caret_char_index(
            ctx,
            editor_id(bundle_id, "model.va"),
            index,
        );
    };

    open_in_design(&mut app.state, &first);
    frame(&ctx, &mut app, DESKTOP);
    place_caret(&ctx, first_bundle, 5);
    frame(&ctx, &mut app, DESKTOP);
    open_in_design(&mut app.state, &second);
    frame(&ctx, &mut app, DESKTOP);
    place_caret(&ctx, second_bundle, 9);
    frame(&ctx, &mut app, DESKTOP);

    // Back to the first view without a frame of its editor in between: the
    // caret the tools read is the one that view's editor last reported.
    open_in_design(&mut app.state, &first);
    super::open_active_language_tools(&mut app).expect("the first view is indexable");
    let tools = app
        .state
        .ui
        .code_workspace
        .language_tools
        .as_ref()
        .expect("the tools opened");
    assert_eq!(tools.index.bundle_id, first_bundle);
    assert_eq!(tools.caret_char_index, 5);
}

/// Compile is a Design command for a Verilog-A view, and its answer is
/// published by the frame loop. It used to be consumed only while the
/// Netlist page was drawn, so a result landing in Design was never read.
#[test]
fn a_compile_from_design_is_reviewed_and_published_while_design_is_showing() {
    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();

    assert!(Command::CompileVerilogA.is_enabled(&app));
    Command::CompileVerilogA.execute(&mut app);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    app.prepare_frame(&ctx);
    let review = app
        .state
        .ui
        .code_workspace
        .veriloga
        .compile_dialog
        .as_ref()
        .expect("the request became the review transaction");
    assert_eq!(review.bundle_id, bundle_id);
    assert!(!app.state.ui.code_workspace.veriloga.compile_requested);

    // The worker answers while Design is still the workspace on screen.
    app.state.ui.code_workspace.veriloga.compile_dialog = None;
    let selected = code_workspace::selected_veriloga_source(&app).unwrap();
    let receipt = code_workspace::compile_project_bundle_receipt(
        app.state.workspace.content.project.id(),
        selected.bundle(),
        selected.selected_module(),
    )
    .expect("the module compiles");
    let (sender, receiver) = mpsc::channel();
    sender
        .send(VerilogACompileOutcome::Success(Box::new(
            (**receipt.compilation.report()).clone(),
        )))
        .unwrap();
    app.state.ui.code_workspace.veriloga.pending = Some(PendingVerilogACompile {
        token: receipt.compilation.token(),
        receiver: Arc::new(Mutex::new(receiver)),
    });
    frame(&ctx, &mut app, DESKTOP);

    assert!(app.state.ui.code_workspace.veriloga.pending.is_none());
    let published = app
        .state
        .ui
        .code_workspace
        .veriloga
        .receipt
        .as_ref()
        .expect("the result was published");
    assert_eq!(published.compilation.token().bundle_id, bundle_id);
}

/// A cell view's bundle is part of that view's document: an edit marks the
/// tab, and the save that publishes it clears the mark.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_source_edit_marks_its_tab_dirty_and_saving_clears_it() {
    use crate::workbench::lifecycle::project_lifecycle::{
        DestinationAuthority, SaveScope, save_native,
    };

    let path = std::env::temp_dir().join(format!(
        "rspice-va-tab-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4().simple()
    ));
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    save_native(
        &mut app.state,
        SaveScope::AllDocuments,
        &path,
        DestinationAuthority::UserSelected,
    )
    .expect("establish the saved baseline");
    assert!(!tab_is_dirty(&app, &reference));

    let selected = code_workspace::selected_veriloga_source(&app).unwrap();
    assert!(
        code_workspace::replace_selected_veriloga_file(
            &mut app,
            &selected,
            "model.va",
            module_source("gain", "  real unused;\n"),
        )
        .unwrap()
    );
    assert!(tab_is_dirty(&app, &reference), "the edit is the view's");

    save_native(
        &mut app.state,
        SaveScope::ActiveDocument,
        &path,
        DestinationAuthority::Canonical,
    )
    .expect("save the Verilog-A view");
    assert!(!tab_is_dirty(&app, &reference), "the save published it");

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
}

/// Reopening a session whose Design document is a Verilog-A view brings the
/// view back as that document, editor and all.
#[test]
fn session_restore_reopens_the_veriloga_tab_in_design() {
    #[derive(Default)]
    struct MemoryStorage(std::collections::HashMap<String, String>);

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.to_owned(), value);
        }

        fn remove_string(&mut self, key: &str) {
            self.0.remove(key);
        }

        fn flush(&mut self) {}
    }

    let mut app = open_project_app();
    let (reference, bundle_id) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let mut storage = MemoryStorage::default();
    eframe::set_value(&mut storage, eframe::APP_KEY, &app.state);

    let mut restored = AppState::restore_eframe_session(&storage).expect("the session restores");
    restored.restore_active_schematic_from_workspace();
    crate::workbench::lifecycle::project_lifecycle::initialize_from_session(&mut restored);
    let mut reopened = RSpiceApp::test_instance();
    reopened.state = restored;

    assert_eq!(reopened.state.workbench.workspace, Workspace::Design);
    assert_eq!(reopened.state.workspace.content.active_view, reference);
    assert_eq!(
        crate::workbench::lifecycle::project_lifecycle::active_document(&reopened.state),
        crate::workbench::lifecycle::project_lifecycle::ProjectDocumentId::CellView(
            reference.clone()
        )
    );
    let ctx = themed_context();
    frame(&ctx, &mut reopened, DESKTOP);
    let output = frame(&ctx, &mut reopened, DESKTOP);
    assert!(
        shows_veriloga_editor(&output),
        "the restored Design document is the editor, not a stub"
    );
    assert!(
        reopened
            .state
            .ui
            .code_workspace
            .source_carets
            .contains_key(&(bundle_id, "model.va".to_owned())),
        "over the restored view's own bundle"
    );
}

/// Renders for review: Design with a Verilog-A tab beside the schematic it
/// came from, at desktop and phone widths, and the document bar alone.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_veriloga_design_document_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    let schematic = RSpiceApp::test_instance()
        .state
        .workspace
        .content
        .active_view
        .clone();

    let reviewed = |coarse: bool, compiled: bool| {
        let mut app = open_project_app();
        let (reference, _) = add_veriloga_cell(
            &mut app.state,
            "gain_cell",
            "models/gain_cell.va",
            "gain_cell",
        );
        open_in_design(&mut app.state, &reference);
        app.state.workbench.coarse_pointer = coarse;
        if compiled {
            let selected = code_workspace::selected_veriloga_source(&app).unwrap();
            app.state.ui.code_workspace.veriloga.receipt = Some(
                code_workspace::compile_project_bundle_receipt(
                    app.state.workspace.content.project.id(),
                    selected.bundle(),
                    selected.selected_module(),
                )
                .expect("the module compiles"),
            );
        }
        app
    };
    let mut write = |name: &str, canvas: &crate::ui::raster::Canvas, height: usize| {
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(height)).expect("write the render");
        writeln!(report, "{}", path.display()).ok();
    };

    for (name, size, coarse) in [
        ("veriloga-design-1440x900", DESKTOP, false),
        ("veriloga-design-390x844", egui::vec2(390.0, 844.0), true),
    ] {
        let mut app = reviewed(coarse, true);
        let canvas = crate::ui::raster::render(size, |ui, _| app.render_frame_chrome(ui));
        write(name, &canvas, size.y as usize);
    }

    let mut uncompiled = reviewed(false, false);
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| uncompiled.render_frame_chrome(ui));
    write(
        "veriloga-design-uncompiled-1440x900",
        &canvas,
        DESKTOP.y as usize,
    );

    // The strip with the module active, then with the schematic beside it
    // active, so the mark is reviewed in both of its tab states.
    let mut strip = reviewed(false, true);
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| strip.render_frame_chrome(ui));
    write("veriloga-document-bar-active", &canvas, 140);
    strip.state.open_workspace_view(schematic.clone());
    let canvas = crate::ui::raster::render(DESKTOP, |ui, _| strip.render_frame_chrome(ui));
    write("veriloga-document-bar-inactive", &canvas, 140);

    // The two new marks at tab, toolbar and inspection sizes: one row each.
    let icons = crate::ui::raster::render(egui::vec2(200.0, 128.0), |ui, background| {
        let t = crate::ui::tokens::Tokens::get(ui.ctx());
        ui.painter().rect_filled(ui.max_rect(), 0.0, background);
        for (row, icon) in [
            crate::workbench::design_system::WorkbenchIcon::VerilogA,
            crate::workbench::design_system::WorkbenchIcon::Format,
        ]
        .into_iter()
        .enumerate()
        {
            let mut left = 8.0;
            for side in [14.0_f32, 16.0, 24.0, 48.0] {
                let top = 8.0 + row as f32 * 60.0;
                icon.paint(
                    ui.painter(),
                    egui::Rect::from_min_size(egui::pos2(left, top), egui::vec2(side, side)),
                    t.color.text,
                );
                left += side + 12.0;
            }
        }
    });
    write("veriloga-icons", &icons, 128);
}

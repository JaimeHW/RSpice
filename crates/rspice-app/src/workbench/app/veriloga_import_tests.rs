//! File > Import Verilog-A/AMS reads a root file with every file its
//! includes reach: from the files picked or dropped with it, then from the
//! folder it was read from. A file still missing is a row the reader locates,
//! and the import waits for it rather than refusing. Add files on the Files
//! rail adds files from disk to a source that already exists.

use super::veriloga_design_tests::{
    DESKTOP, add_veriloga_cell, frame, frame_with, open_in_design, open_project_app, themed_context,
};
use super::*;
use crate::state::workspace::ImportedInclude;
use crate::state::{CellViewRef, ProjectSourceOwner};
use crate::workbench::commands::vocabulary::Command;
use crate::workbench::documents::code_workspace;

/// A root whose parameter and body live in files of their own, as the model
/// authors' sources keep them.
const ROOT: &str = "`include \"disciplines.vams\"\nmodule gainer(a, b);\n  inout a, b;\n  electrical a, b;\n`include \"params.inc\"\n  analog begin\n`include \"body.inc\"\n  end\nendmodule\n";
const PARAMS: &str = "  parameter real g = 2;\n";
const BODY: &str = "    I(a, b) <+ g * V(a, b);\n";

/// A folder of its own under the system's temporary directory, holding
/// `files`.
fn folder_with(name: &str, files: &[(&str, &str)]) -> std::path::PathBuf {
    let folder = std::env::temp_dir().join(format!(
        "rspice-veriloga-import-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&folder);
    std::fs::create_dir_all(&folder).expect("the folder is created");
    for (file, contents) in files {
        std::fs::write(folder.join(file), contents).expect("the file is written");
    }
    folder
}

/// Where the frame's accessibility tree places `name`.
fn named(output: &egui::FullOutput, name: &str) -> Option<egui::Rect> {
    output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .find(|(_, node)| node.label() == Some(name) || node.value() == Some(name))
        .and_then(|(_, node)| node.bounds())
        .map(|bounds| {
            egui::Rect::from_min_max(
                egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
            )
        })
}

/// Settle the frame, then press and release the pointer on `name`.
fn click(ctx: &egui::Context, app: &mut RSpiceApp, name: &str) {
    frame(ctx, app, DESKTOP);
    let output = frame(ctx, app, DESKTOP);
    let at = named(&output, name)
        .unwrap_or_else(|| panic!("nothing on screen is named {name:?}"))
        .center();
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    frame_with(
        ctx,
        app,
        DESKTOP,
        vec![egui::Event::PointerMoved(at), button(true)],
    );
    frame_with(ctx, app, DESKTOP, vec![button(false)]);
}

fn form(app: &RSpiceApp) -> crate::workbench::documents::code_workspace::VerilogAImportFormState {
    app.state
        .code
        .code_workspace
        .veriloga
        .import_form
        .clone()
        .expect("the import form is open")
}

fn bundle_files(app: &RSpiceApp, reference: &CellViewRef) -> Vec<(String, String)> {
    let bundle = app
        .state
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
        .expect("the view has a source");
    bundle
        .files()
        .iter()
        .map(|file| (file.logical_path().to_owned(), file.content().to_owned()))
        .collect()
}

/// A root picked from its folder brings the files it includes from beside
/// it. One its folder lacks is listed as missing, the import waits for it,
/// and Locate supplies it under the name the include asks for, whatever the
/// picked file is called.
#[test]
fn a_root_import_reads_its_includes_from_its_folder_and_a_missing_one_is_located() {
    let folder = folder_with("locate", &[("gainer.va", ROOT), ("params.inc", PARAMS)]);
    let mut app = open_project_app();
    let ctx = themed_context();
    let sources = app.state.model.workspace.content.project_sources.clone();

    Command::ImportVerilogA.execute(&mut app);
    code_workspace::complete_veriloga_files_pick(&mut app, &[("gainer.va", ROOT)], Some(&folder));

    let listed = form(&app)
        .closure
        .files
        .into_iter()
        .map(|file| (file.path, file.include))
        .collect::<Vec<_>>();
    assert_eq!(
        listed,
        [
            ("disciplines.vams".to_owned(), ImportedInclude::BuiltIn),
            ("params.inc".to_owned(), ImportedInclude::Found),
            (
                "body.inc".to_owned(),
                ImportedInclude::Missing {
                    included_by: "gainer.va".to_owned()
                }
            ),
        ]
    );
    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);
    assert!(
        named(
            &output,
            "Locate body.inc, which gainer.va includes, before importing."
        )
        .is_some(),
        "the status says which file the import waits for"
    );
    assert!(app.commit_veriloga_import().is_err());
    assert_eq!(app.state.model.workspace.content.project_sources, sources);

    click(&ctx, &mut app, "Locate…");
    assert!(app.state.code.code_workspace.veriloga.import_requested);
    assert_eq!(form(&app).locating.as_deref(), Some("body.inc"));
    code_workspace::complete_veriloga_file_pick(&mut app, "body_from_elsewhere.inc", BODY);

    let located = form(&app);
    assert_eq!(located.closure.missing().count(), 0);
    assert_eq!(
        located.files.get("body.inc").map(String::as_str),
        Some(BODY)
    );
    let first = app.commit_veriloga_import().expect("the cell is created");
    assert_eq!(first.cell, "gainer");
    let mut files = bundle_files(&app, &first);
    files.sort();
    assert_eq!(
        files,
        [
            ("body.inc".to_owned(), BODY.to_owned()),
            ("params.inc".to_owned(), PARAMS.to_owned()),
        ]
    );
    let _ = std::fs::remove_dir_all(folder);
}

/// A drop of several files is one import per root: a file a dropped root
/// includes is that root's, not a root of its own, and each root's form
/// opens once the one before it is done.
#[test]
fn a_multi_file_drop_imports_each_root_with_the_files_it_includes() {
    let mut app = open_project_app();
    let other = "`include \"disciplines.vams\"\nmodule other(p, n);\n  inout p, n;\n  electrical p, n;\n  analog I(p, n) <+ V(p, n);\nendmodule\n";
    let dropped = [
        ("params.inc", PARAMS),
        ("gainer.va", ROOT),
        ("body.inc", BODY),
        ("other.vams", other),
    ]
    .into_iter()
    .map(|(name, contents)| code_workspace::PickedSource {
        name: name.to_owned(),
        contents: contents.to_owned(),
        folder: None,
    })
    .collect();

    code_workspace::import_dropped_veriloga_sources(
        &app.state.model,
        &mut app.state.code.code_workspace.veriloga,
        dropped,
    )
    .expect("the drop is read");

    let first = form(&app);
    assert_eq!(first.file_name, "gainer.va");
    assert_eq!(first.closure.missing().count(), 0);
    assert_eq!(
        app.state.code.code_workspace.veriloga.import_queue.len(),
        1,
        "the other root waits its turn"
    );
    app.commit_veriloga_import().expect("gainer is created");
    assert_eq!(form(&app).file_name, "other.vams");
    app.commit_veriloga_import().expect("other is created");
    assert!(app.state.code.code_workspace.veriloga.import_form.is_none());
}

/// Add files on the Files rail adds the picked files to the source in
/// front, beside the file being edited.
#[test]
fn add_files_on_the_files_rail_adds_the_picked_files_beside_the_open_file() {
    let mut app = open_project_app();
    let (reference, _) = add_veriloga_cell(&mut app.state, "gain", "model.va", "gain");
    open_in_design(&mut app.state, &reference);
    let ctx = themed_context();

    click(&ctx, &mut app, "Add files…");
    assert!(app.state.code.code_workspace.veriloga.import_requested);
    code_workspace::complete_veriloga_files_pick(
        &mut app,
        &[("a.inc", "// a\n"), ("b.vh", "// b\n")],
        None,
    );

    let mut files = bundle_files(&app, &reference);
    files.sort();
    assert_eq!(
        files,
        [
            ("a.inc".to_owned(), "// a\n".to_owned()),
            ("b.vh".to_owned(), "// b\n".to_owned()),
        ]
    );
    assert!(
        app.state
            .code
            .code_workspace
            .veriloga
            .import_target
            .is_none()
    );
}

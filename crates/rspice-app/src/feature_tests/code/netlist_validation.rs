//! Validating the visible deck, in whole applications: what opening and
//! editing a deck leaves validated, where the check found the deck's
//! includes, and which deck the Validate command checks.

use crate::workbench::RSpiceApp;
use crate::workbench::features::cx;
use rspice_app_code::workbench::code::netlist_validation::validate_visible_netlist_source;
use rspice_app_state::workbench::preferences::FolderListPreference;

#[test]
fn an_opened_deck_validates_and_runs() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let source = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";

    assert!(
        app.state
            .open_test_deck(source.to_owned(), None, "standalone.cir")
    );
    assert!(validate_visible_netlist_source(&mut cx(&mut app)));
    assert!(app.state.model.netlist_artifacts.validation.is_some());
    assert_eq!(app.manual_deck_run_block_reason(), None);
}

/// An edit is validated and run as the session holds it, before it is
/// saved to the deck's file.
#[test]
fn an_edited_deck_validates_and_runs_before_it_is_saved() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let opened = "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n";
    let edited = "standalone edited\nV1 out 0 2\nR1 out 0 2k\n.op\n.end\n";

    assert!(
        app.state
            .open_test_deck(opened.to_owned(), None, "standalone.cir")
    );
    assert!(
        rspice_app_code::workbench::documents::netlist_document::edit_deck_file(
            &mut app.state.model,
            &mut app.state.code.netlist,
            edited.to_owned()
        )
    );
    let deck = app.state.code.netlist.open_deck.as_ref().unwrap();
    assert!(deck.root.dirty, "the edit is not saved yet");
    assert_eq!(
        std::fs::read_to_string(&deck.root.path).unwrap(),
        opened,
        "the file still holds what was opened"
    );

    assert!(validate_visible_netlist_source(&mut cx(&mut app)));
    assert_eq!(
        app.state
            .model
            .netlist_artifacts
            .validation
            .as_ref()
            .map(|receipt| receipt.visible_content_digest),
        Some(rspice_app_state::state::content_digest(edited))
    );
    assert_eq!(app.manual_deck_run_block_reason(), None);
}

#[test]
fn validation_finds_direct_and_transitive_includes_where_they_are() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-netlist-dependency-closure-{}",
        uuid::Uuid::new_v4()
    ));
    let models = dir.join("models");
    std::fs::create_dir_all(&models).expect("create dependency fixture");
    let root = dir.join("root.cir");
    let first_source = ".include \"nested.inc\"\nRmodel out 0 2k\n";
    let nested_source = "Cmodel out 0 1p\n";
    std::fs::write(models.join("first.inc"), first_source).expect("write direct include");
    std::fs::write(models.join("nested.inc"), nested_source).expect("write transitive include");
    let source = "standalone\n.include \"models/first.inc\"\nV1 out 0 1\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck(source.to_owned(), Some(root.clone()), "root.cir")
    );
    assert!(validate_visible_netlist_source(&mut cx(&mut app)));

    let resolutions = &app.state.code.code_workspace.include_resolutions;
    let canonical = |path: &std::path::Path| std::fs::canonicalize(path).expect("fixture path");
    assert_eq!(
        resolutions.keys().map(String::as_str).collect::<Vec<_>>(),
        ["models/first.inc", "nested.inc"],
        "the direct include and the include of an include, as their lines name them"
    );
    assert_eq!(
        canonical(resolutions["models/first.inc"].resolved()),
        canonical(&models.join("first.inc"))
    );
    assert_eq!(
        canonical(resolutions["nested.inc"].resolved()),
        canonical(&models.join("nested.inc")),
        "an include's own include is found beside it"
    );

    std::fs::remove_dir_all(dir).expect("remove dependency fixture");
}

#[test]
fn validation_distinguishes_inline_lib_sections_from_external_lib_paths() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-netlist-library-closure-{}",
        uuid::Uuid::new_v4()
    ));
    std::fs::create_dir_all(&dir).expect("create library fixture");
    let root = dir.join("root.cir");
    let library_source =
        ".lib TT\n.model DMOD D IS=1e-14\n.endl TT\n.lib SS\n.model DMOD D IS=2e-14\n.endl SS\n";
    std::fs::write(dir.join("corners.lib"), library_source).expect("write library");
    let source = "standalone\n.lib LOCAL\n.model LOCALMOD D IS=3e-14\n.endl LOCAL\n.lib \"corners.lib\" TT\nD1 out 0 DMOD\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck(source.to_owned(), Some(root), "root.cir")
    );
    assert!(validate_visible_netlist_source(&mut cx(&mut app)));

    assert_eq!(
        app.state
            .code
            .code_workspace
            .include_resolutions
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["corners.lib"],
        "an inline section is part of the deck; only the library file is read"
    );

    std::fs::remove_dir_all(dir).expect("remove library fixture");
}

#[test]
fn validation_resolves_an_include_only_a_library_folder_holds() {
    let directory = std::env::temp_dir().join(format!(
        "rspice-netlist-library-folder-{}",
        uuid::Uuid::new_v4()
    ));
    let deck = directory.join("deck");
    let shared = directory.join("shared");
    std::fs::create_dir_all(&deck).expect("create deck folder");
    std::fs::create_dir_all(&shared).expect("create library folder");
    std::fs::write(shared.join("device.inc"), "Rmodel out 0 2k\n").expect("write library include");
    let root = deck.join("root.cir");
    let source = "standalone\n.include \"device.inc\"\nV1 out 0 1\n.op\n.end\n";
    std::fs::write(&root, source).expect("write root deck");

    let mut app = crate::workbench::RSpiceApp::test_instance();
    assert!(
        app.state
            .open_test_deck(source.to_owned(), Some(root), "root.cir")
    );
    assert!(
        !validate_visible_netlist_source(&mut cx(&mut app)),
        "nothing the deck is searched from holds device.inc"
    );

    app.state
        .model
        .preferences
        .set_folders(FolderListPreference::LibraryFolders, vec![shared.clone()])
        .expect("a full path");
    assert!(validate_visible_netlist_source(&mut cx(&mut app)));
    let resolution = app
        .state
        .code
        .code_workspace
        .include_resolutions
        .get("device.inc")
        .expect("validation retains where the include resolved");
    assert_eq!(
        resolution.stage(),
        rspice_core::netlist::IncludeSearchStage::LibraryPath(0)
    );
    let canonical = |path: &std::path::Path| std::fs::canonicalize(path).expect("fixture path");
    assert_eq!(
        resolution.resolved().parent().map(canonical),
        Some(canonical(&shared))
    );

    std::fs::remove_dir_all(directory).expect("remove library fixture");
}

/// Validate on the netlist page, as the command carries it out: the deck the
/// page shows is validated and its run prepared, and a deck whose include is
/// nowhere is refused with why, leaving nothing validated.
#[test]
fn the_validate_command_validates_the_deck_the_netlist_page_shows() {
    use rspice_app_code::workbench::code::commands::CodeCommand;
    use rspice_app_state::workbench::workspace::Workspace;

    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert!(app.state.open_test_deck(
        "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
        None,
        "standalone.cir"
    ));
    app.state.workbench.workspace = Workspace::Netlist;

    app.execute_command_key(CodeCommand::ValidateCodeDocument.key());

    let receipt = app
        .state
        .model
        .netlist_artifacts
        .validation
        .as_ref()
        .expect("the deck validates");
    assert_eq!(receipt.task_count, 1);
    assert_eq!(app.state.model.netlist_artifacts.validation_error, None);
    assert_eq!(app.manual_deck_run_block_reason(), None);

    assert!(app.state.open_test_deck(
        "standalone\n.include \"missing.inc\"\nV1 out 0 1\n.op\n.end\n".to_owned(),
        None,
        "missing.cir"
    ));
    app.state.workbench.workspace = Workspace::Netlist;

    app.execute_command_key(CodeCommand::ValidateCodeDocument.key());

    assert!(app.state.model.netlist_artifacts.validation.is_none());
    let error = app
        .state
        .model
        .netlist_artifacts
        .validation_error
        .as_deref()
        .unwrap_or_default();
    assert!(error.contains("missing.inc"), "{error}");
}

/// A validation pressed while a Verilog-A source its preparation reads is
/// compiling on another thread holds no frame: it waits, the status bar says
/// so, and frames keep coming while the compile runs. The first frame after
/// the compile ends validates the deck. Stop drops a validation that waits.
#[test]
fn a_validation_pressed_mid_compile_waits_while_frames_keep_coming() {
    use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, themed_context};
    use rspice_app_code::workbench::code::commands::CodeCommand;
    use rspice_app_runs::workbench::waiting::{WaitingValidation, hold_compiling};
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::command_registry::ids;

    const WAITING: &str = "Validating the netlist \u{b7} waiting for the Verilog-A compile";
    fn says(output: &egui::FullOutput, text: &str) -> bool {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .is_some_and(|update| {
                update
                    .nodes
                    .iter()
                    .any(|(_, node)| node.label() == Some(text) || node.value() == Some(text))
            })
    }
    fn validations(app: &RSpiceApp) -> usize {
        app.state
            .model
            .log_buffer
            .entries()
            .filter(|entry| entry.message.starts_with("Validated the netlist"))
            .count()
    }

    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    assert!(app.state.open_test_deck(
        "standalone\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n".to_owned(),
        None,
        "standalone.cir"
    ));
    app.state.workbench.workspace = Workspace::Netlist;
    let ctx = themed_context();

    let compile = hold_compiling();
    app.execute_command_key(CodeCommand::ValidateCodeDocument.key());
    assert_eq!(
        app.state.runs.waiting.validation,
        Some(WaitingValidation { then_format: false })
    );
    for _ in 0..3 {
        let output = frame(&ctx, &mut app, DESKTOP);
        assert!(says(&output, WAITING), "the status bar says what waits");
        assert!(app.state.model.netlist_artifacts.validation.is_none());
    }
    drop(compile);
    let output = frame(&ctx, &mut app, DESKTOP);
    assert!(app.state.runs.waiting.is_empty());
    assert!(app.state.model.netlist_artifacts.validation.is_some());
    assert_eq!(validations(&app), 1);
    assert!(!says(&output, WAITING));

    let compile = hold_compiling();
    app.execute_command_key(CodeCommand::ValidateCodeDocument.key());
    assert!(app.state.runs.waiting.validation.is_some());
    assert!(app.command_key_enabled(ids::STOP_SIMULATION));
    app.execute_command_key(ids::STOP_SIMULATION);
    assert!(app.state.runs.waiting.is_empty());
    drop(compile);
    frame(&ctx, &mut app, DESKTOP);
    assert_eq!(validations(&app), 1, "a stopped validation never runs");
}

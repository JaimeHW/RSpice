use super::shared::{validate_lcv_name, DialogActionOutcome};
use super::*;
use crate::state::{Cell, Library, View, ViewType};

fn make_test_app() -> RSpiceApp {
    RSpiceApp::new_for_tests(super::super::AppState::default())
}

#[test]
fn test_validate_lcv_name_rejects_empty_and_invalid_chars() {
    assert_eq!(
        validate_lcv_name("", "Cell name").as_deref(),
        Some("Cell name cannot be empty")
    );
    assert_eq!(
        validate_lcv_name("bad-name", "View name").as_deref(),
        Some("View name must contain only letters, numbers, and underscores")
    );
    assert!(validate_lcv_name("valid_name_1", "Cell name").is_none());
}

#[test]
fn test_handle_new_view_create_action_reports_missing_library() {
    let mut app = make_test_app();
    app.state.dialogs.new_view_name = "schematic".to_string();
    app.state.dialogs.new_view_library = "missing_lib".to_string();
    app.state.dialogs.new_view_cell = "my_cell".to_string();

    let outcome = app.handle_new_view_create_action();

    assert_eq!(outcome, DialogActionOutcome::default());
    assert_eq!(
        app.state.dialogs.new_view_error.as_deref(),
        Some("Library 'missing_lib' not found")
    );
}

#[test]
fn test_handle_new_view_create_action_reports_missing_cell() {
    let mut app = make_test_app();
    app.state.library_manager.add_library(Library::new("work"));
    app.state.dialogs.new_view_name = "schematic".to_string();
    app.state.dialogs.new_view_library = "work".to_string();
    app.state.dialogs.new_view_cell = "missing_cell".to_string();

    let outcome = app.handle_new_view_create_action();

    assert_eq!(outcome, DialogActionOutcome::default());
    assert_eq!(
        app.state.dialogs.new_view_error.as_deref(),
        Some("Cell 'missing_cell' not found in library 'work'")
    );
}

#[test]
fn test_handle_new_view_create_action_reports_duplicate_view() {
    let mut app = make_test_app();
    let mut lib = Library::new("work");
    let mut cell = Cell::new("my_cell");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    lib.add_cell(cell);
    app.state.library_manager.add_library(lib);

    app.state.dialogs.new_view_name = "schematic".to_string();
    app.state.dialogs.new_view_library = "work".to_string();
    app.state.dialogs.new_view_cell = "my_cell".to_string();

    let outcome = app.handle_new_view_create_action();

    assert_eq!(outcome, DialogActionOutcome::default());
    assert_eq!(
        app.state.dialogs.new_view_error.as_deref(),
        Some("View 'schematic' already exists in cell 'my_cell'")
    );
}

#[test]
fn test_handle_new_view_create_action_adds_view_and_requests_close() {
    let mut app = make_test_app();
    let mut lib = Library::new("work");
    lib.add_cell(Cell::new("my_cell"));
    app.state.library_manager.add_library(lib);

    app.state.dialogs.new_view_name = "symbol".to_string();
    app.state.dialogs.new_view_library = "work".to_string();
    app.state.dialogs.new_view_cell = "my_cell".to_string();
    app.state.dialogs.new_view_type = ViewType::Symbol;

    let outcome = app.handle_new_view_create_action();

    assert!(outcome.close);
    assert!(!outcome.persist_global_veriloga);
    assert!(app.state.dialogs.new_view_error.is_none());
    let created = app
        .state
        .library_manager
        .get_library("work")
        .and_then(|lib| lib.get_cell("my_cell"))
        .and_then(|cell| cell.get_view("symbol"))
        .is_some();
    assert!(created);
}

#[test]
fn test_handle_new_cell_create_action_reports_missing_library() {
    let mut app = make_test_app();
    app.state.dialogs.new_cell_name = "my_cell".to_string();
    app.state.dialogs.new_cell_library = "missing".to_string();

    let outcome = app.handle_new_cell_create_action();

    assert_eq!(outcome, DialogActionOutcome::default());
    assert_eq!(
        app.state.dialogs.new_cell_error.as_deref(),
        Some("Library 'missing' not found")
    );
}

#[test]
fn test_handle_new_cell_create_action_reports_duplicate_cell() {
    let mut app = make_test_app();
    let mut lib = Library::new("work");
    lib.add_cell(Cell::new("my_cell"));
    app.state.library_manager.add_library(lib);
    app.state.dialogs.new_cell_name = "my_cell".to_string();
    app.state.dialogs.new_cell_library = "work".to_string();

    let outcome = app.handle_new_cell_create_action();

    assert_eq!(outcome, DialogActionOutcome::default());
    assert_eq!(
        app.state.dialogs.new_cell_error.as_deref(),
        Some("Cell 'my_cell' already exists in library 'work'")
    );
}

#[test]
fn test_handle_new_cell_create_action_adds_cell_and_views() {
    let mut app = make_test_app();
    app.state.library_manager.add_library(Library::new("work"));
    app.state.dialogs.new_cell_name = "my_cell".to_string();
    app.state.dialogs.new_cell_library = "work".to_string();
    app.state.dialogs.new_cell_create_schematic = true;
    app.state.dialogs.new_cell_create_symbol = true;
    app.state.dialogs.new_cell_create_testbench = false;

    let outcome = app.handle_new_cell_create_action();

    assert!(outcome.close);
    assert!(!outcome.persist_global_veriloga);
    assert!(app.state.dialogs.new_cell_error.is_none());
    let created = app
        .state
        .library_manager
        .get_library("work")
        .and_then(|lib| lib.get_cell("my_cell"));
    assert!(created.is_some());
    let cell = created.unwrap();
    assert!(cell.get_view("schematic").is_some());
    assert!(cell.get_view("symbol").is_some());
    assert!(cell.get_view("testbench").is_none());
}

#[test]
fn test_process_pending_library_deletions_removes_cell_and_view() {
    let mut app = make_test_app();
    let mut lib = Library::new("work");
    let mut cell = Cell::new("my_cell");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    lib.add_cell(cell);
    app.state.library_manager.add_library(lib);

    app.state.pending_delete_view = Some((
        "work".to_string(),
        "my_cell".to_string(),
        "schematic".to_string(),
    ));
    app.process_pending_library_deletions();
    let view_exists = app
        .state
        .library_manager
        .get_library("work")
        .and_then(|lib| lib.get_cell("my_cell"))
        .and_then(|cell| cell.get_view("schematic"))
        .is_some();
    assert!(!view_exists);

    app.state.pending_delete_cell = Some(("work".to_string(), "my_cell".to_string()));
    app.process_pending_library_deletions();
    let cell_exists = app
        .state
        .library_manager
        .get_library("work")
        .and_then(|lib| lib.get_cell("my_cell"))
        .is_some();
    assert!(!cell_exists);
}

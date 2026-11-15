//! Source guards for the design feature.
//!
//! `rspice-app` re-exports this crate's modules at the paths they had inside
//! it, so a module the application reaches by path has to be `pub mod` here.
//! That is the only reason a module is public: the list below is exact, and a
//! module that turns public for any other reason fails it, as does one that
//! stops being reached and is left public.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `pub mod` declaration, as (file relative to `src/`, module name). A
/// name `lib.rs` declares twice, under two `cfg`s or two parents, is one entry.
const PUBLIC_MODULES: &[(&str, &str)] = &[
    ("lib.rs", "actions"),
    ("lib.rs", "app"),
    ("lib.rs", "app_state"),
    ("lib.rs", "chrome"),
    ("lib.rs", "connectivity_manager"),
    ("lib.rs", "create_model_bound_symbol"),
    ("lib.rs", "cross_probe"),
    ("lib.rs", "design"),
    ("lib.rs", "design_request"),
    ("lib.rs", "design_review_comments"),
    ("lib.rs", "dialogs"),
    ("lib.rs", "docks"),
    ("lib.rs", "document_bar"),
    ("lib.rs", "engineering_table"),
    ("lib.rs", "find_replace"),
    ("lib.rs", "grid_snap_routing"),
    ("lib.rs", "hierarchy"),
    ("lib.rs", "hierarchy_net_highlight"),
    ("lib.rs", "interaction_state"),
    ("lib.rs", "navigator"),
    ("lib.rs", "object_properties"),
    ("lib.rs", "placement"),
    ("lib.rs", "properties"),
    ("lib.rs", "property_dialog"),
    ("lib.rs", "property_edit"),
    ("lib.rs", "replace_instance"),
    ("lib.rs", "rule_setup"),
    ("lib.rs", "schematic"),
    ("lib.rs", "selection"),
    ("lib.rs", "slots"),
    ("lib.rs", "state"),
    ("lib.rs", "status_bar"),
    ("lib.rs", "stimulus"),
    ("lib.rs", "stimulus_browser_state"),
    ("lib.rs", "stimulus_editor"),
    ("lib.rs", "stimulus_link"),
    ("lib.rs", "stimulus_placement"),
    ("lib.rs", "surfaces"),
    ("lib.rs", "symbol"),
    ("lib.rs", "symbol_definition"),
    ("lib.rs", "title_bar"),
    ("lib.rs", "visibility_options"),
    ("lib.rs", "workbench"),
    ("lib.rs", "workspace"),
    ("properties.rs", "property_bridge"),
    ("properties.rs", "source_preview"),
    ("schematic.rs", "symbol_editor"),
    ("schematic.rs", "view"),
    ("schematic/symbol_editor.rs", "interaction"),
    ("schematic/symbol_editor/interaction.rs", "tests"),
    ("schematic/view.rs", "array_interaction"),
    ("schematic/view.rs", "context_menu"),
    ("schematic/view.rs", "frame_cost"),
    ("schematic/view.rs", "gesture_commits"),
    ("schematic/view.rs", "interaction"),
    ("schematic/view.rs", "keyboard_navigation"),
    ("schematic/view.rs", "mobile_controls"),
    ("schematic/view.rs", "move_interaction"),
    ("schematic/view.rs", "requests"),
    ("schematic/view.rs", "scene"),
    ("schematic/view.rs", "selection_drag"),
    ("schematic/view.rs", "selection_transform"),
    ("schematic/view.rs", "sheet_visibility"),
    ("schematic/view.rs", "stretch_interaction"),
    ("schematic/view.rs", "violations"),
    ("schematic/view/keyboard_navigation.rs", "tests"),
    ("schematic/view/scene.rs", "tests"),
    ("schematic/view/stretch_interaction.rs", "tests"),
    ("workbench/app/actions/workspace.rs", "design_checks"),
    ("workbench/app/actions/workspace.rs", "history"),
    ("workbench/app/actions/workspace.rs", "symbol_document"),
    ("workbench/app/actions/workspace.rs", "symbol_in_place"),
    ("workbench/app/actions/workspace.rs", "tests"),
    ("workbench/app/actions/workspace.rs", "waivers"),
    (
        "workbench/app/dialogs/create_model_bound_symbol.rs",
        "controller",
    ),
    (
        "workbench/app/dialogs/create_model_bound_symbol.rs",
        "state",
    ),
    (
        "workbench/app/dialogs/create_model_bound_symbol.rs",
        "tests",
    ),
    ("workbench/app/dialogs/design_review_comments.rs", "tests"),
    ("workbench/app/dialogs/engineering_table.rs", "tests"),
    ("workbench/app/dialogs/find_replace.rs", "tests"),
    ("workbench/app/dialogs/hierarchy.rs", "create"),
    ("workbench/app/dialogs/hierarchy.rs", "descend"),
    ("workbench/app/dialogs/object_properties.rs", "tests"),
    ("workbench/app/dialogs/placement.rs", "bus_tap"),
    ("workbench/app/dialogs/placement.rs", "design_note"),
    ("workbench/app/dialogs/placement.rs", "net_label"),
    ("workbench/app/dialogs/placement.rs", "pin_port"),
    ("workbench/app/dialogs/placement/bus_tap.rs", "tests"),
    ("workbench/app/dialogs/placement/design_note.rs", "tests"),
    ("workbench/app/dialogs/placement/net_label.rs", "tests"),
    ("workbench/app/dialogs/placement/pin_port.rs", "tests"),
    ("workbench/app/dialogs/selection.rs", "array"),
    ("workbench/app/dialogs/selection.rs", "bulk_edit"),
    ("workbench/app/dialogs/selection.rs", "bulk_filter"),
    ("workbench/app/dialogs/selection.rs", "commands"),
    ("workbench/app/dialogs/selection.rs", "move_selection"),
    ("workbench/app/dialogs/selection.rs", "rename"),
    ("workbench/app/dialogs/selection.rs", "shared_properties"),
    ("workbench/app/dialogs/selection.rs", "stretch"),
    ("workbench/app/dialogs/selection/bulk_edit.rs", "tests"),
    ("workbench/app/dialogs/selection/commands.rs", "floating"),
    ("workbench/app/dialogs/selection/rename.rs", "tests"),
    (
        "workbench/app/dialogs/selection/shared_properties.rs",
        "tests",
    ),
    ("workbench/app/dialogs/stimulus_link.rs", "adopt"),
    (
        "workbench/app/dialogs/symbol_definition.rs",
        "import_workflow",
    ),
    ("workbench/app/dialogs/symbol_definition.rs", "state"),
    ("workbench/app/schematic.rs", "conductor_routes"),
    ("workbench/app/schematic.rs", "named_net"),
    ("workbench/app/schematic.rs", "placement"),
    ("workbench/design.rs", "commands"),
    ("workbench/design.rs", "inspector"),
    ("workbench/design.rs", "navigator"),
    ("workbench/design.rs", "surface"),
    ("workbench/design.rs", "test_host"),
    ("workbench/design.rs", "toolbar"),
    ("workbench/design/inspector.rs", "component"),
    ("workbench/design/inspector.rs", "inline_instance"),
    ("workbench/design/inspector.rs", "model_evidence"),
    ("workbench/design/inspector.rs", "stimulus"),
    ("workbench/design/inspector.rs", "symbol"),
    ("workbench/design/inspector.rs", "tests"),
    ("workbench/design/navigator.rs", "hierarchy_tree"),
    ("workbench/design/navigator.rs", "shelf"),
    ("workbench/design/navigator.rs", "stimulus"),
    ("workbench/design/navigator.rs", "symbol"),
    ("workbench/design/navigator.rs", "tests"),
    ("workbench/design/navigator/shelf.rs", "search"),
    ("workbench/design/navigator/stimulus.rs", "tests"),
    ("workbench/design/navigator/tests.rs", "shelf"),
    ("workbench/design/surface.rs", "layout_editor"),
    ("workbench/design/surface.rs", "tests"),
    ("workbench/surfaces/stimulus.rs", "program"),
    ("workbench/surfaces/stimulus.rs", "tests"),
];

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        for entry in
            fs::read_dir(&dir).unwrap_or_else(|error| panic!("read {}: {error}", dir.display()))
        {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}

fn relative(root: &Path, file: &Path) -> String {
    file.strip_prefix(root)
        .unwrap_or(file)
        .to_string_lossy()
        .replace('\\', "/")
}

fn without_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split_once("//").map_or(line, |(code, _)| code))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn only_the_modules_the_application_re_exports_are_public() {
    let root = src_dir();
    let mut measured: Vec<(String, String)> = Vec::new();
    for file in rust_sources(&root) {
        let source = fs::read_to_string(&file).unwrap_or_default();
        for line in without_line_comments(&source).lines() {
            let Some(rest) = line.trim_start().strip_prefix("pub mod ") else {
                continue;
            };
            let name: String = rest
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            measured.push((relative(&root, &file), name));
        }
    }
    assert!(
        !measured.is_empty(),
        "{} holds no public module",
        root.display()
    );
    measured.sort();
    let mut expected: Vec<(String, String)> = PUBLIC_MODULES
        .iter()
        .map(|(file, name)| ((*file).to_owned(), (*name).to_owned()))
        .collect();
    expected.sort();
    let added: Vec<_> = measured
        .iter()
        .filter(|entry| !expected.contains(entry))
        .collect();
    let dropped: Vec<_> = expected
        .iter()
        .filter(|entry| !measured.contains(entry))
        .collect();
    assert!(
        added.is_empty() && dropped.is_empty(),
        "the public modules changed.\n  newly public: {added:?}\n  no longer public: {dropped:?}\n\
         A module is public here only because rspice-app re-exports it at its old path; \
         update PUBLIC_MODULES in tests/module_guards.rs to match exactly."
    );
}

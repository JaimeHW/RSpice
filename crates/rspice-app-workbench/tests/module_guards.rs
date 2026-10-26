//! Source guards for the workbench framework.
//!
//! `rspice-app` re-exports this crate's modules at the paths they had inside
//! it, so a module the application reaches by path has to be `pub mod` here.
//! That is the only reason a module is public: the list below is exact, and a
//! module that turns public for any other reason fails it, as does one that
//! stops being reached and is left public. Lint suppressions are counted the
//! way `rspice-app` counts its own.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `pub mod` declaration, as (file relative to `src/`, module name). A
/// name `lib.rs` declares twice, under two `cfg`s or two parents, is one entry.
const PUBLIC_MODULES: &[(&str, &str)] = &[
    ("lib.rs", "app"),
    ("lib.rs", "app_state"),
    ("lib.rs", "browser"),
    ("lib.rs", "chrome"),
    ("lib.rs", "command_palette"),
    ("lib.rs", "confirmation"),
    ("lib.rs", "confirmation_state"),
    ("lib.rs", "console_views"),
    ("lib.rs", "dialogs"),
    ("lib.rs", "docks"),
    ("lib.rs", "documents"),
    ("lib.rs", "fact_grid"),
    ("lib.rs", "framework"),
    ("lib.rs", "layout"),
    ("lib.rs", "lifecycle"),
    ("lib.rs", "localization"),
    ("lib.rs", "model_browser"),
    ("lib.rs", "navigation"),
    ("lib.rs", "navigator_tree"),
    ("lib.rs", "notification_center"),
    ("lib.rs", "operation_primitives"),
    ("lib.rs", "problems_view"),
    ("lib.rs", "retired_documents"),
    ("lib.rs", "review_primitives"),
    ("lib.rs", "routing"),
    ("lib.rs", "session"),
    ("lib.rs", "shortcuts"),
    ("lib.rs", "specialist_tool_browser"),
    ("lib.rs", "state"),
    ("lib.rs", "surfaces"),
    ("lib.rs", "tools"),
    ("lib.rs", "ui"),
    ("lib.rs", "window_session"),
    ("lib.rs", "workbench"),
    ("lib.rs", "workbench_session"),
    ("workbench/app_state/session/shortcuts.rs", "category"),
    ("workbench/app_state/session/shortcuts.rs", "resolver"),
    ("workbench/chrome.rs", "activity_rail"),
    ("workbench/chrome.rs", "document_bar"),
    ("workbench/chrome.rs", "phone_navigation"),
    ("workbench/chrome.rs", "status_bar"),
    ("workbench/chrome.rs", "title_bar"),
    ("workbench/chrome.rs", "toolbar"),
    ("workbench/chrome/title_bar.rs", "application_menus"),
    ("workbench/chrome/title_bar.rs", "menu_entries"),
    ("workbench/chrome/toolbar.rs", "overflow"),
    ("workbench/docks.rs", "console"),
    ("workbench/docks.rs", "drawers"),
    ("workbench/docks.rs", "inspector"),
    ("workbench/docks.rs", "navigator"),
    ("workbench/docks/console.rs", "log_anchor"),
    ("workbench/docks/console.rs", "problems"),
    ("workbench/docks/console.rs", "transcript"),
    ("workbench/docks/console/problems.rs", "arrange"),
    ("workbench/docks/console/problems.rs", "detail"),
    ("workbench/docks/console/problems.rs", "grid"),
    ("workbench/docks/console/transcript.rs", "lines"),
    ("workbench/docks/inspector.rs", "shared"),
    ("workbench/docks/navigator.rs", "rail"),
    ("workbench/framework.rs", "command_registry"),
    ("workbench/framework.rs", "dialogs"),
    ("workbench/framework.rs", "feature"),
    ("workbench/framework.rs", "host"),
    ("workbench/framework.rs", "ports"),
    ("workbench/framework.rs", "requests"),
    ("workbench/framework.rs", "workbench_host"),
    ("workbench/framework/command_registry.rs", "ids"),
    ("workbench/framework/feature.rs", "harness"),
    ("workbench/routing.rs", "availability"),
    ("workbench/routing.rs", "navigation"),
    ("workbench/routing.rs", "surface_catalog"),
    ("workbench/routing.rs", "surface_route"),
];

/// `lib.rs` allows `clippy::too_many_arguments`, as `rspice-app` does, and the
/// shortcut platform allows the host variants only one target constructs.
const MAX_LINT_SUPPRESSIONS: usize = 2;

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

#[test]
fn lint_suppressions_do_not_grow() {
    let mut suppressions = 0usize;
    for file in rust_sources(&src_dir()) {
        let source = without_line_comments(&fs::read_to_string(&file).unwrap_or_default());
        suppressions += source.matches("[allow(").count();
        suppressions += source.matches("[expect(").count();
    }
    assert_eq!(
        suppressions, MAX_LINT_SUPPRESSIONS,
        "{suppressions} lint suppressions, ceiling is {MAX_LINT_SUPPRESSIONS}. Fix what the lint \
         is pointing at, or lower MAX_LINT_SUPPRESSIONS when one goes."
    );
}

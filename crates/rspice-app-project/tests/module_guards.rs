//! Source guards for the project feature.
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
    ("lib.rs", "app"),
    ("lib.rs", "check_and_save"),
    ("lib.rs", "check_and_save_validation"),
    ("lib.rs", "chrome"),
    ("lib.rs", "design_management"),
    ("lib.rs", "dialogs"),
    ("lib.rs", "docks"),
    ("lib.rs", "document_bar"),
    ("lib.rs", "examples_browser"),
    ("lib.rs", "file_workflow"),
    ("lib.rs", "inspector"),
    ("lib.rs", "library"),
    ("lib.rs", "navigator"),
    ("lib.rs", "project"),
    ("lib.rs", "project_launcher"),
    ("lib.rs", "project_revision_history"),
    ("lib.rs", "project_workflow"),
    ("lib.rs", "slots"),
    ("lib.rs", "surfaces"),
    ("lib.rs", "toolbar"),
    ("lib.rs", "tools"),
    ("lib.rs", "workbench"),
    ("lib.rs", "workflows"),
    ("workbench/app/dialogs/design_management.rs", "operations"),
    ("workbench/app/dialogs/design_management.rs", "widgets"),
    ("workbench/app/dialogs/project.rs", "configuration"),
    ("workbench/app/dialogs/project.rs", "configuration_dialogs"),
    ("workbench/app/dialogs/project.rs", "new_project"),
    ("workbench/docks/inspector/project.rs", "configuration"),
    ("workbench/project.rs", "commands"),
    ("workbench/project.rs", "configuration_state"),
    ("workbench/project.rs", "library_dialogs"),
    ("workbench/project.rs", "workspace_state"),
    ("workbench/project/library_dialogs.rs", "cell_ops"),
    ("workbench/project/library_dialogs.rs", "deletion"),
    ("workbench/project/library_dialogs.rs", "library_ops"),
    ("workbench/project/library_dialogs.rs", "new_cell"),
    ("workbench/project/library_dialogs.rs", "new_view"),
    ("workbench/project/library_dialogs.rs", "rename_view"),
    ("workbench/surfaces/project.rs", "configuration"),
    ("workbench/surfaces/project.rs", "dependencies"),
    ("workbench/surfaces/project.rs", "landing"),
    ("workbench/surfaces/project.rs", "library"),
    ("workbench/surfaces/project.rs", "library_publication"),
    ("workbench/surfaces/project.rs", "overview"),
    ("workbench/surfaces/project.rs", "recovery"),
    ("workbench/surfaces/project/library.rs", "builtin_models"),
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

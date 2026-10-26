//! Source guards for the sheets feature.
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
    ("lib.rs", "actions"),
    ("lib.rs", "app"),
    ("lib.rs", "chrome"),
    ("lib.rs", "delete_sheet"),
    ("lib.rs", "design"),
    ("lib.rs", "dialogs"),
    ("lib.rs", "docks"),
    ("lib.rs", "drawing_sheet_defaults"),
    ("lib.rs", "drawing_sheet_edit"),
    ("lib.rs", "drawing_sheet_overflow"),
    ("lib.rs", "drawing_sheet_presets"),
    ("lib.rs", "drawing_sheet_preview"),
    ("lib.rs", "drawing_sheet_support"),
    ("lib.rs", "inspector"),
    ("lib.rs", "sheet"),
    ("lib.rs", "sheet_strip"),
    ("lib.rs", "sheets"),
    ("lib.rs", "slots"),
    ("lib.rs", "status_bar"),
    ("lib.rs", "workbench"),
    ("workbench/app/dialogs/drawing_sheet_presets.rs", "model"),
    ("workbench/app/dialogs/drawing_sheet_presets.rs", "render"),
    ("workbench/app/drawing_sheet_edit.rs", "transaction"),
    ("workbench/sheets.rs", "commands"),
];

/// `lib.rs` allows `clippy::too_many_arguments`, as `rspice-app` does.
const MAX_LINT_SUPPRESSIONS: usize = 1;

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

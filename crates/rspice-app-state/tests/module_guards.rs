//! Source guards for the persisted model.
//!
//! `rspice-app` re-exports this crate's modules at the paths they had inside
//! it, so a module the application reaches by path has to be `pub mod` here,
//! as does one a higher application crate calls, such as the browser store.
//! Those are the only reasons a module is public: the list below is exact, and a
//! module that turns public for any other reason fails it, as does one that
//! stops being reached and is left public. Lint suppressions are counted the
//! way `rspice-app` counts its own.

use std::fs;
use std::path::{Path, PathBuf};

/// Every `pub mod` declaration, as (file relative to `src/`, module name).
const PUBLIC_MODULES: &[(&str, &str)] = &[
    ("diagnostics.rs", "engine_log"),
    ("lib.rs", "app"),
    ("lib.rs", "app_state"),
    ("lib.rs", "browser_store"),
    ("lib.rs", "cell_usage"),
    ("lib.rs", "design_checks"),
    ("lib.rs", "design_history"),
    ("lib.rs", "diagnostic_store"),
    ("lib.rs", "diagnostics"),
    ("lib.rs", "edit_authority"),
    ("lib.rs", "examples"),
    ("lib.rs", "fixture_root"),
    ("lib.rs", "model"),
    ("lib.rs", "placed_sources"),
    ("lib.rs", "plan"),
    ("lib.rs", "preferences"),
    ("lib.rs", "product"),
    ("lib.rs", "quantity"),
    ("lib.rs", "results"),
    ("lib.rs", "rule_deck"),
    ("lib.rs", "run_identity"),
    ("lib.rs", "schematic"),
    ("lib.rs", "schematic_preferences"),
    ("lib.rs", "sim_setup"),
    ("lib.rs", "simulation"),
    ("lib.rs", "source_revision"),
    ("lib.rs", "state"),
    ("lib.rs", "stimulus_realize"),
    ("lib.rs", "symbol_contract"),
    ("lib.rs", "table_route"),
    ("lib.rs", "time_compat"),
    ("lib.rs", "waivers"),
    ("lib.rs", "workbench"),
    ("lib.rs", "workspace"),
    ("results.rs", "safety"),
    ("state.rs", "code_diagnostics"),
    ("state.rs", "command_catalog"),
    ("state.rs", "engineering_table"),
    ("state.rs", "model_hub"),
    ("state.rs", "model_library"),
    ("state.rs", "netlist_diagnostics"),
    ("state.rs", "notices"),
    ("state.rs", "pdk_config"),
    ("state.rs", "project_snapshot"),
    ("state.rs", "shortcuts"),
    ("state.rs", "stimulus_library"),
    ("state.rs", "workspace"),
    ("state.rs", "write_policy"),
    ("state/model_hub.rs", "durable"),
    ("state/model_hub.rs", "provider"),
    ("state/model_hub.rs", "tests"),
    ("state/model_hub.rs", "transport"),
    ("state/pdk_config/technology_diff.rs", "tests"),
    ("state/schematic.rs", "bulk_edit"),
    ("state/schematic.rs", "named_net"),
    ("state/schematic.rs", "overflow_memo"),
    ("state/schematic.rs", "policy_labels"),
    ("state/schematic.rs", "symbol_context_cache"),
    ("state/schematic.rs", "transform_preview_memo"),
    ("state/stimulus_library.rs", "fixtures"),
    ("time_compat.rs", "tests"),
    ("workbench/app_state/diagnostic_store.rs", "digest_memo"),
    ("workbench/app_state/diagnostic_store.rs", "listing"),
    ("workbench/app_state/diagnostic_store.rs", "producers"),
    ("workbench/app_state/diagnostic_store.rs", "run_notices"),
    ("workbench/app_state/sim_setup.rs", "plan_catalog"),
];

/// `lib.rs` allows `clippy::too_many_arguments`, as `rspice-app` does, and
/// `clippy::len_without_is_empty` for the diagnostic listing's row count.
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

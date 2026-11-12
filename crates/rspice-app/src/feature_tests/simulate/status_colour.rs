//! No surface states a status in a colour of its own, read from the shipped
//! source of the shell and of each feature crate. It strips the test-only items
//! with the shell's source guard, so it runs here, in the shell.

use std::path::{Path, PathBuf};

/// Every `.rs` file the Simulation Studio ships, read at test time.
///
/// Walked rather than listed with `include_str!`, so a page added to the
/// studio is scanned the day it lands rather than the day someone
/// remembers to add it here.
fn studio_sources() -> Vec<(PathBuf, String)> {
    fn walk(directory: &Path, out: &mut Vec<PathBuf>) {
        let entries = std::fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/workbench/surfaces");
    let mut paths = vec![root.join("simulate.rs")];
    walk(&root.join("simulate"), &mut paths);
    assert!(
        paths.len() > 1,
        "the walk of {} found no source files; a scan that reaches nothing passes forever",
        root.join("simulate").display()
    );
    paths.sort();
    let sources: Vec<(PathBuf, String)> = paths
        .into_iter()
        .map(|path| {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            (path, source)
        })
        .collect();
    // A test-only file is free to author whatever it needs to author. Which
    // ones those are is read from the module that declared them, not from a
    // list here that would go stale.
    let roots: Vec<PathBuf> = sources
        .iter()
        .flat_map(|(path, source)| crate::source_guard::test_only_roots(path, source))
        .collect();
    sources
        .iter()
        .filter(|(path, _)| crate::source_guard::ships(path, &roots))
        .map(|(path, source)| {
            (
                path.clone(),
                without_line_comments(&crate::source_guard::without_test_items(source)),
            )
        })
        .collect()
}

/// `source` with every `//` tail removed, line for line.
///
/// A comment that quotes a banned spelling is prose about it, not a use of
/// it.
fn without_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| line.split("//").next().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n")
}

/// What paints a status in colour, banned on every slice below.
///
/// Success is the absence of a problem, so it has no styling at all: no
/// success or information tone, no status dot, and neither of the two
/// `StatusMark` variants that are not problems. A warning or a failure is
/// a `Problem` mark in front of neutral text, which owns its own colour.
const STATUS_IN_COLOUR: [&str; 6] = [
    "colored_label",
    "status_dot(",
    "StatusMark::Success",
    "StatusMark::Neutral",
    "color.ok",
    "color.info",
];

/// The surfaces held to [`STATUS_IN_COLOUR`]: each one's shipped files
/// under `src/workbench`, and the spellings it bans on top, each the trace
/// of a habit that surface was written with. The studio is the walked
/// [`studio_sources`] instead of a list.
const STATUS_IN_COLOUR_SLICES: [(&str, &[&str], &[&str]); 8] = [
    (
        "dialog content",
        &[
            "app/dialogs/check_and_save.rs",
            "app/dialogs/check_and_save_validation.rs",
            "app/dialogs/review_primitives.rs",
            "app/dialogs/live_session.rs",
            "app/dialogs/publish_web.rs",
            "app/dialogs/unpublish_web.rs",
            "app/dialogs/source_document.rs",
            "app/dialogs/source_find.rs",
            "app/dialogs/source_language_tools.rs",
            "app/dialogs/design_management.rs",
            "app/dialogs/design_management/manager.rs",
            "app/dialogs/design_management/subflows.rs",
            "app/dialogs/design_management/widgets.rs",
            "app/dialogs/design_management/operations.rs",
            "app/dialogs/design_management/controller.rs",
            "app/dialogs/preferences/shortcut_preferences.rs",
            "app/dialogs/preferences/shortcut_portability_dialogs.rs",
            "app/dialogs/preferences/preference_pages.rs",
            "app/dialogs/preferences/preferences_shell.rs",
        ],
        &[],
    ),
    (
        "selection commands",
        &[
            "app/dialogs/selection/array.rs",
            "app/dialogs/selection/bulk_edit.rs",
            "app/dialogs/selection/commands.rs",
            "app/dialogs/selection/move_selection.rs",
            "app/dialogs/selection/rename.rs",
            "app/dialogs/selection/stretch.rs",
            "app/dialogs/object_properties.rs",
            "surfaces/design/tool_options.rs",
        ],
        &[".color(t.color.warn"],
    ),
    (
        "Design and Verilog-A inspectors",
        &[
            "docks/inspector.rs",
            "docks/inspector/code_workspace.rs",
            "docks/inspector/design.rs",
            "docks/inspector/design/component.rs",
            "docks/inspector/design/inline_instance.rs",
            "docks/inspector/design/model_evidence.rs",
            "docks/inspector/project.rs",
            "docks/inspector/results.rs",
            "docks/inspector/verify.rs",
            "documents/code_workspace/veriloga_inspector.rs",
            "surfaces/veriloga.rs",
        ],
        &[],
    ),
    (
        // Colour here belongs to the comparison plot's two series; the dot
        // that led a state line is banned as the literal it was painted
        // from, so comments recording why it went still read.
        "model correlation",
        &[
            "surfaces/model_correlation.rs",
            "surfaces/model_correlation/sections.rs",
            "surfaces/model_correlation/dialogs.rs",
        ],
        &["Tone::Good", "\"●\""],
    ),
    (
        "model editor",
        &[
            "surfaces/model_editor.rs",
            "surfaces/model_editor/promotion.rs",
            "surfaces/model_editor/qualification.rs",
            "surfaces/model_editor/schema_editors.rs",
            "surfaces/model_editor/workflows.rs",
        ],
        &["MetricTone::Good"],
    ),
    (
        "Models and PDK",
        &[
            "surfaces/models/manager.rs",
            "surfaces/models/manager/adoption.rs",
            "surfaces/models/manager/bindings.rs",
            "surfaces/models/manager/bins.rs",
            "surfaces/models/manager/corners.rs",
            "surfaces/models/manager/corpus.rs",
            "surfaces/models/manager/detail.rs",
            "surfaces/models/manager/dialogs.rs",
            "surfaces/models/manager/drift.rs",
            "surfaces/models/manager/hub.rs",
            "surfaces/models/manager/held_catalog.rs",
            "surfaces/models/manager/include.rs",
            "surfaces/models/manager/paint.rs",
            "surfaces/models/manager/qualification_page.rs",
            "surfaces/models/manager/shelf.rs",
            "surfaces/models/manager/symbols.rs",
        ],
        &[],
    ),
    (
        // No data identity and no verdict table: no severity tone at all,
        // and no shouted label.
        "Project and navigator",
        &[
            "surfaces/project.rs",
            "surfaces/project/configuration.rs",
            "surfaces/project/dependencies.rs",
            "surfaces/project/landing.rs",
            "surfaces/project/library.rs",
            "surfaces/project/library_publication.rs",
            "surfaces/project/overview.rs",
            "surfaces/project/page.rs",
            "surfaces/project/recovery.rs",
            "docks/navigator.rs",
            "docks/navigator/project.rs",
            "docks/navigator/results.rs",
            "docks/navigator/results_actions.rs",
            "docks/navigator/results_rows.rs",
            "docks/navigator/simulate.rs",
            "docks/navigator/verify.rs",
        ],
        &["color.warn", "color.err", "to_ascii_uppercase"],
    ),
    (
        "Verify",
        &[
            "surfaces/verify.rs",
            "surfaces/verify/corners.rs",
            "surfaces/verify/histogram.rs",
            "surfaces/verify/optimization.rs",
            "surfaces/verify/regression_contract.rs",
            "surfaces/verify/regression_panel.rs",
            "surfaces/verify/safety.rs",
            "surfaces/verify/table.rs",
            "surfaces/verify/toolbar.rs",
            "surfaces/verify/tuning.rs",
            "surfaces/verify/yield_page.rs",
        ],
        &["status_dot"],
    ),
];

/// No surface states a status in a colour of its own.
///
/// In the studio `Tone::Fail` is banned too, except on the Specifications
/// page: its Result column paints a failing measurement in the error
/// colour with no mark, a verdict rather than a status, because a column
/// of marks beside a column of numbers reads as two verdicts.
#[test]
fn no_surface_paints_a_status_in_colour() {
    /// Whether `line` names `token` rather than sitting inside a longer
    /// identifier: `PlanDetailTone::Failed` is not a `Tone::Fail`.
    fn names(line: &str, token: &str) -> bool {
        let ident = |byte: Option<&u8>| {
            byte.is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_')
        };
        let bytes = line.as_bytes();
        line.match_indices(token).any(|(index, _)| {
            let before = index.checked_sub(1).and_then(|before| bytes.get(before));
            !ident(before) || !ident(bytes.get(index + token.len()))
        })
    }
    fn scan(name: &str, source: &str, also: &[&str], sites: &mut Vec<String>) {
        for (index, line) in source.lines().enumerate() {
            for token in STATUS_IN_COLOUR.iter().chain(also) {
                if names(line, token) {
                    sites.push(format!("{name}:{}: {token}", index + 1));
                }
            }
        }
    }

    let workbench = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/workbench");
    // A framework file compiles in rspice-app-workbench, and a feature's file in
    // that feature's crate, each at the same path under its crate's `src`.
    let framework =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-workbench/src/workbench");
    let settings =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-settings/src/workbench");
    let results = Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-results/src/workbench");
    let cloud = Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-cloud/src/workbench");
    let models = Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-models/src/workbench");
    let mut sites = Vec::new();
    for (surface, files, also) in STATUS_IN_COLOUR_SLICES {
        for file in files {
            let path = [
                workbench.join(file),
                framework.join(file),
                settings.join(file),
                results.join(file),
                cloud.join(file),
                models.join(file),
            ]
            .into_iter()
            .find(|path| path.is_file())
            .unwrap_or_else(|| workbench.join(file));
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{surface}: read {}: {error}", path.display()));
            let shipped = crate::source_guard::without_test_items(&source);
            scan(file, &shipped, also, &mut sites);
        }
    }
    let studio = studio_sources();
    assert!(
        studio.len() >= 20,
        "the studio scan found only {} files; the walk has stopped reaching them",
        studio.len()
    );
    for (path, source) in &studio {
        let also: &[&str] = if path.ends_with("page_specs.rs") {
            &[]
        } else {
            &["Tone::Fail"]
        };
        let name = path.strip_prefix(&workbench).unwrap_or(path).display();
        scan(&name.to_string(), source, also, &mut sites);
    }
    assert!(
        sites.is_empty(),
        "these paint a status in colour:\n  {}",
        sites.join("\n  ")
    );
}

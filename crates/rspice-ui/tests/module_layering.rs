//! Architectural layering ratchet for the `rspice-ui` application crate.
//!
//! `rspice-ui` is deliberately a single application crate: the simulation
//! engine lives in `rspice-core` and `rspice-veriloga`, and everything the
//! GUI application itself owns — state, project I/O, orchestration, viewer
//! mathematics, chrome — stays here. That is a considered decision, but it
//! costs the one thing a crate split would have given for free: the compiler
//! no longer refuses an import that inverts the architecture.
//!
//! This test is the substitute. It declares the intended layer order and
//! fails when a module reaches sideways or upward through `crate::`.
//!
//! # How it ratchets
//!
//! Every edge that violates the layer order today is recorded in
//! [`ALLOWED_VIOLATIONS`] with its exact reference count. The count is a
//! ceiling, never a target:
//!
//! - A **new** violating edge fails the build.
//! - An **existing** violating edge that grows fails the build.
//! - An edge that shrinks is fine; lower the ceiling when convenient.
//! - An edge that reaches zero fails the build until its entry is deleted,
//!   so the allowlist cannot rot into a list of problems that no longer
//!   exist.
//!
//! Each entry carries the remediation phase that retires it. When a phase
//! lands, its entries disappear from this list rather than being renumbered.
//!
//! # What counts as a reference
//!
//! An occurrence of `crate::<module>` in a `.rs` file under `src/`, after
//! line comments are stripped. Doc comments describing a relationship are
//! not a dependency, so they are excluded. `use`, type positions, and paths
//! all count equally: the metric tracks coupling, not import style.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Intended layer order, lowest first.
///
/// A module may reference any module in a strictly lower layer. Referencing
/// the same layer or a higher one is a violation: same-layer edges are how
/// cycles reappear, so the ordering is total by construction.
///
/// The ordering encodes the target architecture, not today's reality —
/// [`ALLOWED_VIOLATIONS`] carries the gap between the two.
const LAYERS: &[(&str, u32)] = &[
    // Framework-independent contracts. These depend on nothing.
    ("product", 0),
    ("quantity", 0),
    ("time_compat", 0),
    // Presentation-independent result contracts, and the design system.
    ("results", 1),
    ("ui", 1),
    // What a project persists about printing: page setup, print mappings, and
    // the digest-authenticated source-set records. Hardcopy *rendering* needs
    // the schematic symbol library and the analysis viewers, so it stays up in
    // `workbench`; only the persisted contract lives down here, where `state`
    // can reach it. It sits above `results` because a source-set error can
    // name an unauthenticated report block.
    ("hardcopy", 2),
    // The persisted design and project model.
    ("state", 3),
    // Domain services over the persisted model.
    ("analysis", 4),
    ("automation_workflow", 4),
    ("io", 4),
    ("services", 5),
    ("simulation", 6),
    // Editors and inspectors.
    ("properties", 7),
    ("schematic", 9),
    // Application chrome and top-level navigation.
    ("workbench", 10),
    // The application root: `RSpiceApp`, dialogs, and workflows.
];

/// Layer-order violations present in the tree today, with exact counts.
///
/// Sorted by remediation phase. Do not add entries to unblock new code —
/// a new violation means the code is in the wrong module.
const ALLOWED_VIOLATIONS: &[(&str, &str, usize)] = &[
    // The `common` <-> `workbench` cycle is retired.
    //
    // It could not be broken by moving a module. `workbench` reached for
    // `RSpiceApp` 861 times and `AppState` 356 times, and the workflows in
    // `common` reached back for `AppState` just as hard (project_lifecycle
    // 109, menu_bar 90, project_workflow 62). Hoisting the application root
    // would only have renamed the edge; pushing state down was blocked because
    // `UiSessionState` aggregates the document engines' own session state.
    //
    // The boundary was not describing a real seam, so the two modules were
    // merged: `workbench` is now the whole application shell. 294 upward
    // references became internal. Do not recreate the split.
    //
    // `state -> workbench` is also retired. It started at 39 and was cleared
    // by moving what the project actually persists out of the shell: the
    // engineering-table views (39 -> 37), the page-setup contract and print
    // mappings (-> 16), the schematic visibility policy (-> 9), the hardcopy
    // source-set records (-> 2), and the netlist document (-> 0). The merge
    // briefly reintroduced three via the wasm clock shim, which now sits at
    // layer 0 as `crate::time_compat`.

    // Reaching up into the application shell for `RSpiceApp` rather than
    // taking the state actually used. Retired by narrowing those signatures.
    ("schematic", "workbench", 124),
    ("simulation", "workbench", 87),
    ("io", "workbench", 12),
    // The persisted model reaching up into orchestration and editors.
    ("state", "simulation", 26),
    ("state", "services", 7),
    ("state", "io", 5),
    ("state", "schematic", 2),
    ("state", "analysis", 1),
    // `analysis` is viewer mathematics and must not reach into orchestration.
    ("analysis", "simulation", 1),
    // Editors and orchestration referencing each other sideways; retired by
    // the granularity folds and the `properties`/`panels` merge.
    ("io", "simulation", 22),
    ("services", "simulation", 11),
    ("simulation", "schematic", 3),
    ("services", "properties", 2),
    ("services", "schematic", 2),
];

/// Whole-application mutable access, frozen at its current level.
///
/// A function taking `&mut RSpiceApp` may touch every field of every
/// subsystem. Narrowing all of them at once is not worth the regression
/// risk, so the count is frozen instead: it may fall, never rise. Prefer a
/// parameter naming exactly the state a handler needs.
const MAX_WHOLE_APP_MUTABLE_PARAMS: usize = 541;

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let entries = fs::read_dir(&dir)
            .unwrap_or_else(|error| panic!("read {}: {error}", dir.display()));
        for entry in entries {
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

/// The top-level module a source file belongs to.
///
/// Files sitting directly in `src/` (`lib.rs`, `main.rs`, and crate-wide
/// helpers) belong to no module and are skipped: the crate root is allowed
/// to see everything.
fn owning_module(root: &Path, file: &Path) -> Option<String> {
    let relative = file.strip_prefix(root).ok()?;
    let mut components = relative.components();
    let first = components.next()?.as_os_str().to_str()?.to_owned();
    components.next()?;
    Some(first)
}

/// Strip line comments so prose describing a relationship is not counted as
/// one. A `//` inside a string literal costs at most an undercount, which
/// can only make this test more permissive, never falsely red.
fn strip_line_comments(source: &str) -> String {
    source
        .lines()
        .map(|line| match line.find("//") {
            Some(index) => &line[..index],
            None => line,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn count_occurrences(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut rest = haystack;
    while let Some(index) = rest.find(needle) {
        let after = &rest[index + needle.len()..];
        // Reject a longer identifier: `crate::state` must not match
        // `crate::statement`.
        let boundary = after
            .chars()
            .next()
            .is_none_or(|next| !next.is_alphanumeric() && next != '_');
        if boundary {
            count += 1;
        }
        rest = &rest[index + needle.len()..];
    }
    count
}

/// Reference counts between top-level modules, keyed `(from, to)`.
fn edge_counts() -> BTreeMap<(String, String), usize> {
    let root = src_dir();
    let modules: Vec<&str> = LAYERS.iter().map(|(name, _)| *name).collect();
    let mut edges: BTreeMap<(String, String), usize> = BTreeMap::new();

    for file in rust_sources(&root) {
        let Some(from) = owning_module(&root, &file) else {
            continue;
        };
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
        let code = strip_line_comments(&source);
        for to in &modules {
            if *to == from {
                continue;
            }
            let count = count_occurrences(&code, &format!("crate::{to}"));
            if count > 0 {
                *edges.entry((from.clone(), (*to).to_owned())).or_default() += count;
            }
        }
    }
    edges
}

fn layer_of(module: &str) -> u32 {
    LAYERS
        .iter()
        .find(|(name, _)| *name == module)
        .map(|(_, layer)| *layer)
        .unwrap_or_else(|| {
            panic!(
                "module `{module}` has no declared layer.\n\
                 Add it to LAYERS in tests/module_layering.rs, placing it \
                 strictly above everything it may reference."
            )
        })
}

#[test]
fn every_top_level_module_declares_a_layer() {
    let root = src_dir();
    let entries = fs::read_dir(&root).expect("read src/");
    let mut undeclared = Vec::new();
    for entry in entries {
        let path = entry.expect("directory entry").path();
        if !path.is_dir() {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("module directory name")
            .to_owned();
        if !LAYERS.iter().any(|(declared, _)| *declared == name) {
            undeclared.push(name);
        }
    }
    assert!(
        undeclared.is_empty(),
        "new top-level modules without a declared layer: {undeclared:?}\n\
         Add them to LAYERS in tests/module_layering.rs. A module with no \
         declared position in the architecture is a module nobody decided \
         the position of."
    );
}

#[test]
fn module_references_respect_the_layer_order() {
    let edges = edge_counts();
    let mut allowed: BTreeMap<(&str, &str), usize> = ALLOWED_VIOLATIONS
        .iter()
        .map(|(from, to, count)| ((*from, *to), *count))
        .collect();
    assert_eq!(
        allowed.len(),
        ALLOWED_VIOLATIONS.len(),
        "ALLOWED_VIOLATIONS contains a duplicate (from, to) pair"
    );

    let mut new_violations = Vec::new();
    let mut grown = Vec::new();

    for ((from, to), count) in &edges {
        if layer_of(from) > layer_of(to) {
            continue;
        }
        match allowed.remove(&(from.as_str(), to.as_str())) {
            None => new_violations.push(format!(
                "  {from} -> {to} ({count} references, layer {} -> layer {})",
                layer_of(from),
                layer_of(to)
            )),
            Some(ceiling) if *count > ceiling => grown.push(format!(
                "  {from} -> {to}: {count} references, ceiling is {ceiling}"
            )),
            Some(_) => {}
        }
    }

    // Entries left over describe edges that no longer exist. Deleting them
    // is the point of the exercise, so the allowlist must be kept honest.
    let retired: Vec<String> = allowed
        .keys()
        .map(|(from, to)| format!("  {from} -> {to}"))
        .collect();

    let mut failures = String::new();
    if !new_violations.is_empty() {
        failures.push_str(&format!(
            "\nNew layer-order violations:\n{}\n\
             This code reaches sideways or upward through the architecture. \
             Move it to a module that sits above what it needs, or take the \
             state it uses as a parameter instead of reaching for it.\n",
            new_violations.join("\n")
        ));
    }
    if !grown.is_empty() {
        failures.push_str(&format!(
            "\nExisting violations that grew:\n{}\n\
             These edges are being retired, not extended.\n",
            grown.join("\n")
        ));
    }
    if !retired.is_empty() {
        failures.push_str(&format!(
            "\nAllowlist entries for edges that no longer exist:\n{}\n\
             Delete them from ALLOWED_VIOLATIONS in \
             tests/module_layering.rs.\n",
            retired.join("\n")
        ));
    }
    assert!(failures.is_empty(), "{failures}");
}

/// The crate uses one module-file convention: `foo.rs` beside `foo/`.
///
/// It previously mixed both forms, sometimes inside the same directory —
/// `dialogs/` held `hardcopy.rs` + `hardcopy/` next to `library/mod.rs`.
/// Picking one makes a module's own file findable by name instead of by
/// remembering which style its author used.
#[test]
fn modules_use_the_sibling_file_convention_not_mod_rs() {
    let root = src_dir();
    let offenders: Vec<String> = rust_sources(&root)
        .into_iter()
        .filter(|path| path.file_name().is_some_and(|name| name == "mod.rs"))
        .map(|path| {
            path.strip_prefix(&root)
                .unwrap_or(&path)
                .display()
                .to_string()
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "these modules use `mod.rs` instead of a sibling `foo.rs`:\n  {}\n\
         Rename `foo/mod.rs` to `foo.rs`, leaving `foo/` for its submodules.\n\
         Note that `include_str!` inside a moved file resolves against the new \
         parent directory, so self-referencing test fixtures need updating too.",
        offenders.join("\n  ")
    );
}

#[test]
fn whole_application_mutable_access_does_not_grow() {
    let root = src_dir();
    let mut count = 0;
    for file in rust_sources(&root) {
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
        count += strip_line_comments(&source).matches("&mut RSpiceApp").count();
    }
    assert!(
        count <= MAX_WHOLE_APP_MUTABLE_PARAMS,
        "`&mut RSpiceApp` parameters rose to {count}, ceiling is \
         {MAX_WHOLE_APP_MUTABLE_PARAMS}.\n\
         A function taking the whole application can mutate every subsystem. \
         Take the state the handler actually needs instead.\n\
         If a reduction elsewhere justifies the change, lower \
         MAX_WHOLE_APP_MUTABLE_PARAMS in tests/module_layering.rs."
    );
}

/// Files are budgeted at 2500 lines.
///
/// The budget is not about style. A 8000-line module is where a state machine
/// and its renderer get to share private fields without anyone deciding they
/// should, which is how `hardcopy_sources.rs` came to hold both the persisted
/// source-set records and the adapters that resolve live documents.
///
/// Everything currently over budget is listed with its exact length. As with
/// the layering table, the number is a ceiling: a file may shrink, never grow,
/// and an entry that drops under budget must be deleted.
const LINE_BUDGET: usize = 2500;

const OVERSIZED_FILES: &[(&str, usize)] = &[
    ("workbench/hardcopy_sources.rs", 8328),
    ("simulation/runner/worker_contract.rs", 8226),
    ("io/project_io.rs", 8214),
    ("workbench/hardcopy_render.rs", 8095),
    ("state/workspace.rs", 7524),
    ("workbench/visualization_studio.rs", 7600),
    ("state/model_library/qualification.rs", 6924),
    ("workbench/surfaces/model_editor.rs", 6892),
    ("workbench/surfaces/verify.rs", 6794),
    ("workbench/surfaces/simulate.rs", 6406),
    ("results/visualization_document.rs", 6394),
    ("workbench/model_editor.rs", 5107),
    ("results/report_document.rs", 4877),
    ("workbench/commands.rs", 4716),
    ("io/durable_file.rs", 4689),
    ("product/capability_readiness.rs", 4593),
    ("simulation/plan/model.rs", 4241),
    ("workbench/surfaces/models.rs", 4224),
    ("workbench/result_document/waves.rs", 4199),
    ("state/model_library/manager.rs", 4171),
    ("state/model_library/correlation.rs", 4049),
    ("hardcopy/contract.rs", 4031),
    ("workbench/docks/inspector/design.rs", 3994),
    ("workbench/surfaces/project.rs", 3992),
    ("workbench/feature_availability_data.rs", 3872),
    ("workbench/project_lifecycle.rs", 3741),
    ("workbench/surfaces/model_correlation.rs", 3631),
    ("workbench/state.rs", 3507),
    ("schematic/view/interaction.rs", 3490),
    ("workbench/app/actions/workspace.rs", 3400),
    ("workbench/hardcopy_print.rs", 3355),
    ("workbench/app/dialogs/hardcopy/render.rs", 3348),
    ("state/schematic/state/editor_ops/array_ops.rs", 3320),
    ("simulation/controller.rs", 3213),
    ("simulation/execution/snapshot.rs", 3068),
    ("workbench/project_lifecycle/persistence.rs", 3004),
    ("workbench/feature_availability.rs", 2987),
    ("workbench/project_launcher.rs", 2931),
    ("workbench/docks/navigator.rs", 2877),
    ("workbench/chrome/title_bar.rs", 2775),
    ("state/project_sources.rs", 2709),
    ("io/project_execution.rs", 2709),
    ("simulation/controller/prepared_run.rs", 2676),
    ("state/schematic/state/editor_ops/movement_ops.rs", 2629),
    ("workbench/shortcut_artifacts/merge.rs", 2597),
    ("state/netlist_document/document.rs", 2579),
    ("workbench/app/dialogs/hardcopy/publish.rs", 2564),
    ("workbench/app/dialogs/preferences/shortcut_preferences.rs", 2529),
];

#[test]
fn source_files_stay_within_the_line_budget() {
    let root = src_dir();
    let allowed: BTreeMap<&str, usize> = OVERSIZED_FILES.iter().copied().collect();
    assert_eq!(
        allowed.len(),
        OVERSIZED_FILES.len(),
        "OVERSIZED_FILES contains a duplicate path"
    );

    let mut new_offenders = Vec::new();
    let mut grown = Vec::new();
    let mut seen = Vec::new();

    for file in rust_sources(&root) {
        let relative = file
            .strip_prefix(&root)
            .unwrap_or(&file)
            .display()
            .to_string()
            .replace('\\', "/");
        let lines = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("read {}: {error}", file.display()))
            .lines()
            .count();
        match allowed.get(relative.as_str()) {
            Some(ceiling) => {
                seen.push(relative.clone());
                if lines > *ceiling {
                    grown.push(format!("  {relative}: {lines} lines, ceiling is {ceiling}"));
                } else if lines <= LINE_BUDGET {
                    grown.push(format!(
                        "  {relative}: now {lines} lines, under the {LINE_BUDGET} budget \
                         — delete its OVERSIZED_FILES entry"
                    ));
                }
            }
            None if lines > LINE_BUDGET => {
                new_offenders.push(format!("  {relative}: {lines} lines"));
            }
            None => {}
        }
    }

    let mut failures = String::new();
    if !new_offenders.is_empty() {
        failures.push_str(&format!(
            "\nFiles newly over the {LINE_BUDGET}-line budget:\n{}\n\
             Split along a seam the file already has — state from rendering, \
             contract from adapter — rather than by line count.\n",
            new_offenders.join("\n")
        ));
    }
    if !grown.is_empty() {
        failures.push_str(&format!(
            "\nOversized files that grew, or that are now under budget:\n{}\n",
            grown.join("\n")
        ));
    }
    let missing: Vec<String> = allowed
        .keys()
        .filter(|path| !seen.iter().any(|s| s == *path))
        .map(|path| format!("  {path}"))
        .collect();
    if !missing.is_empty() {
        failures.push_str(&format!(
            "\nOVERSIZED_FILES entries for files that no longer exist:\n{}\n\
             Delete them.\n",
            missing.join("\n")
        ));
    }
    assert!(failures.is_empty(), "{failures}");
}

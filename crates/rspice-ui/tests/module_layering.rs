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
    ("utils", 0),
    // Presentation-independent result contracts, the design system, and the
    // persisted page-setup contract. Hardcopy *rendering* needs the schematic
    // symbol library and the analysis viewers, so it stays up in `workbench`;
    // only what a project persists about printing lives down here, where
    // `state` can reach it.
    ("hardcopy", 1),
    ("results", 1),
    ("ui", 1),
    // The persisted design and project model.
    ("state", 2),
    // Domain services over the persisted model.
    ("analysis", 3),
    ("automation_workflow", 3),
    ("io", 3),
    ("waveform", 3),
    ("services", 4),
    ("simulation", 5),
    // Editors and inspectors.
    ("properties", 6),
    ("panels", 7),
    ("schematic", 8),
    // Application chrome and top-level navigation.
    ("workbench", 9),
    // The application root: `RSpiceApp`, dialogs, and workflows.
    ("common", 10),
];

/// Layer-order violations present in the tree today, with exact counts.
///
/// Sorted by remediation phase. Do not add entries to unblock new code —
/// a new violation means the code is in the wrong module.
const ALLOWED_VIOLATIONS: &[(&str, &str, usize)] = &[
    // Phase 1 — break the `common` <-> `workbench` cycle.
    //
    // The cycle is not fixed by moving the application root up: `workbench`
    // reaches for `RSpiceApp` 861 times and `AppState` 356 times, so hoisting
    // `common/app` to `crate::app` would only rename this edge. It is fixed by
    // pushing state *down*. Inside `workbench`, `app.state.` appears 2679
    // times while `RSpiceApp`'s non-state fields appear about 30 times, so
    // nearly all of the coupling is really to `AppState`.
    //
    // The plan: move the egui-free state modules (`state.rs`, `session.rs`,
    // `preferences.rs`, `engineering_table.rs`, `hardcopy.rs`, `shortcuts/`,
    // and the `Command` enum) out of `workbench` into `state`, then convert
    // `workbench` signatures from `&mut RSpiceApp` to `&mut AppState`.
    ("workbench", "common", 294),
    // The persisted project model still reaches up for hardcopy source sets
    // and the netlist document. Each module that moves down lowers this
    // count: it started at 39, `engineering_table` took it to 37, the
    // page-setup contract to 16, and the schematic visibility policy to 9.
    ("state", "workbench", 9),
    ("state", "simulation", 26),
    ("state", "properties", 10),
    ("state", "services", 7),
    ("state", "io", 5),
    ("state", "common", 3),
    ("state", "schematic", 2),
    ("state", "analysis", 1),
    // Phase 3 — core boundary.
    // `analysis` is viewer mathematics and must not reach into orchestration.
    ("analysis", "simulation", 1),
    // Phase 4 — narrow the application god object.
    // These modules reach up for `RSpiceApp` rather than taking the state
    // they actually use.
    ("schematic", "common", 71),
    ("simulation", "common", 48),
    ("io", "common", 16),
    ("services", "common", 5),
    ("panels", "common", 4),
    ("schematic", "workbench", 61),
    ("simulation", "workbench", 42),
    ("panels", "workbench", 1),
    // Phase 5 — module placement.
    // Editors and orchestration referencing each other sideways; retired by
    // the granularity folds and the `properties`/`panels` merge.
    ("simulation", "properties", 31),
    ("io", "simulation", 22),
    ("services", "simulation", 11),
    ("simulation", "schematic", 3),
    ("services", "properties", 2),
    ("services", "schematic", 2),
    ("ui", "state", 1),
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

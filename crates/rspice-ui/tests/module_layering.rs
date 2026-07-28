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
//! # Two levels
//!
//! [`LAYERS`] orders the crate's top-level modules. That alone is not enough:
//! `workbench` is 283k lines across 254 files — about half the crate — so
//! treating it as one unit left its entire interior unmeasured, and a cycle
//! that an earlier refactor believed it had retired was in fact still there.
//! [`WORKBENCH_LAYERS`] therefore orders `workbench`'s own submodules, and
//! both orders ratchet the same way.
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
    // What the application reports about itself. It is a value model and
    // would sit lower, but a log entry may anchor to a schematic object, so
    // it cannot sit below `state`.
    ("diagnostics", 4),
    ("io", 4),
    ("services", 5),
    ("simulation", 6),
    // Editors and inspectors.
    ("properties", 7),
    ("schematic", 8),
    // The application shell: `RSpiceApp`, chrome, dialogs, surfaces, and the
    // workflows that mutate them. Its interior is ordered separately by
    // [`WORKBENCH_LAYERS`] — see `workbench_references_respect_the_layer_order`.
    ("workbench", 9),
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

    // These were recorded as signature-narrowing work — modules reaching up
    // for `RSpiceApp` instead of the state they use. That was wrong about
    // what the edges contain. Only 11 of `schematic`'s references were
    // `AppState`; the bulk were types and functions parked in the shell that
    // belong further down, and no amount of narrowing would have moved them.
    // They are retired by relocating what is misplaced, not by rewriting
    // signatures: the armed-tool lifecycle moved to `state::schematic`
    // (124 -> 102), since it only ever read and wrote `SchematicState`, and
    // the console/log models moved to `crate::diagnostics` (schematic
    // 102 -> 98, simulation 87 -> 60).
    //
    // What is left in `simulation -> workbench` is one entangled type, not a
    // misfiled one. `PreparedVerilogARuntime` is engine state, but its only
    // constructors validate against `VerilogACompileReceipt`, which also
    // carries `CodeEditorDiagnostic` — an egui type. Moving the runtime down
    // means first narrowing those constructors to the three fields they
    // actually read (`token`, `module_name`, `report`) so the receipt can
    // stay in the editor where it belongs. That is a design change, not a
    // relocation, so it is not folded in here.
    ("schematic", "workbench", 98),
    ("simulation", "workbench", 28),
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
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|error| panic!("read {}: {error}", dir.display()));
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

// =============================================================================
// The interior of `workbench`
// =============================================================================

/// Intended order inside `workbench`, lowest first. Position is the rank.
///
/// [`LAYERS`] treats `workbench` as one unit, which left 283k lines — roughly
/// half the crate — with no layering enforcement at all. The `common` merge in
/// `7401cdcf0` did not retire the cycle it describes; it moved the boundary so
/// the top-level test could no longer see it. Sixteen module pairs still
/// reference each other in both directions, and `app` is in twelve of them.
///
/// Unlike [`LAYERS`], this order is total: a module may reference anything
/// earlier in this list and nothing at or after it. Ties are what let a cycle
/// hide inside a layer, and the whole point of this table is that one already
/// did.
const WORKBENCH_LAYERS: &[&str] = &[
    // Contracts, routes, and policy. These describe the shell without
    // operating it.
    "platform",
    "state",
    "routing/surface_route",
    "routing/surface_catalog",
    "routing/availability",
    "routing/navigation",
    "workflows/capability_workflow",
    "logging",
    "layout",
    // The command vocabulary: what commands exist, their portable identity,
    // and the label a menu shows. Shortcut profiles, preferences, and the
    // artifact writers name a command without reaching the dispatcher that
    // runs it, which is up at `commands`.
    "commands/vocabulary",
    "shortcuts",
    "preferences",
    "feature_availability",
    "feature_availability_data",
    "design_system",
    "documents/visualization_family",
    // Document engines and the interaction session they hang off.
    "lifecycle/session",
    "documents/netlist_document",
    "documents/result_document",
    "documents/code_workspace",
    "documents/model_editor",
    "documents/model_correlation",
    "lifecycle/window_session",
    // Import, persistence, checkpointing, and recovery.
    "browser/file_import",
    "lifecycle/project_lifecycle",
    "lifecycle/project_checkpoint",
    "lifecycle/recovery",
    "lifecycle/recovery_checkpoint",
    "shortcuts/library_persistence",
    "shortcuts/artifacts",
    // Hardcopy adapters: resolve sources, then render, then print.
    "hardcopy_adapters/sources",
    "hardcopy_adapters/render",
    "hardcopy_adapters/print",
    // The session aggregate: the design, the simulation, the document engines'
    // session state, and the dialogs' visibility. It sits directly below the
    // frame loop because it aggregates the whole session — a module that only
    // needs one part of it should take that part as a parameter, which is what
    // retires the `-> app_state` entries below.
    "app_state",
    // The application root. Everything above operates on it; everything below
    // is operated on by it.
    "app",
    // A static catalog of demonstration circuits. It sits here rather than up
    // in the presentation group because nothing presents it except the menu
    // that lists it, and it loads a circuit into `AppState`.
    "examples",
    // Workflows that mutate application state.
    "workflows/file_workflow",
    "workflows/project_workflow",
    "workflows/export_workflow",
    "workflows/netlist_workflow",
    "shortcuts/profile_workflow",
    "workflows/file_actions",
    // Command dispatch.
    "commands",
    // Chrome, docks, surfaces, and the rest of the presentation layer.
    "panels",
    "chrome",
    "docks",
    "menu_bar",
    "surfaces",
    "documents/visualization_studio",
    "tools/notification_center",
    "tools/project_launcher",
    "tools/specialist_tool_browser",
    "tools/jobs_manager",
    "tools/calculator_tool",
    "preflight",
    "cross_probe",
    "account_organization",
    "simulation_analysis_tabs",
    "browser/navigation",
    "browser/download",
    "browser/accessibility",
    // The frame is painted last and may reach anything below it.
    "frame",
];

/// Order violations inside `workbench` today, with exact counts.
///
/// Same rules as [`ALLOWED_VIOLATIONS`]: a ceiling, never a target. Every
/// count here is the measured value, not a rounded-up allowance — a ceiling
/// with slack in it is an edge nobody is watching.
///
/// The concentration is the finding: `app` and `app_state` carry a third of
/// the list between them. That is what splitting them apart exposed. Before
/// the split, `AppState` and `RSpiceApp` shared a module, so "this module
/// needs the session data" and "this module calls the application root" were
/// the same measurement. They are now separate problems with separate fixes —
/// the first retires by passing a slice of the session instead of the whole
/// aggregate, the second by the `&mut RSpiceApp` burndown.
const ALLOWED_WORKBENCH_VIOLATIONS: &[(&str, &str, usize)] = &[
    // `app` is the frame loop, and it drives everything. These are the calls
    // out of `RSpiceApp` into the presentation and workflow layers it renders.
    // They retire as the frame loop stops calling and starts setting state
    // that the called module reads — the pattern `frame` already follows.
    ("app", "workflows/project_workflow", 45),
    ("app", "commands", 16),
    ("app", "workflows/export_workflow", 14),
    ("app", "chrome", 12),
    ("app", "menu_bar", 12),
    ("app", "workflows/file_workflow", 8),
    ("app", "browser/navigation", 7),
    ("app", "panels", 7),
    ("app", "workflows/file_actions", 6),
    ("app", "frame", 5),
    ("app", "browser/accessibility", 2),
    ("app", "workflows/netlist_workflow", 2),
    ("app", "browser/download", 1),
    ("app", "cross_probe", 1),
    ("app", "tools/calculator_tool", 1),
    // `AppState` is the session aggregate, so it inherits two problems from
    // the root it was split out of.
    //
    // Downward: a module that needs one slice of the session takes the whole
    // aggregate. Each of these retires by passing the slice — the schematic,
    // the workspace, the netlist session — instead of `&AppState`.
    ("documents/result_document", "app_state", 19),
    ("lifecycle/recovery", "app_state", 6),
    ("documents/netlist_document", "app_state", 4),
    ("lifecycle/project_lifecycle", "app_state", 3),
    ("hardcopy_adapters/sources", "app_state", 1),
    ("lifecycle/project_checkpoint", "app_state", 1),
    // Upward: fields whose types live in the feature that renders them, plus
    // gating methods that query a module above. `-> app` is `DialogState`,
    // which still lives in the dialogs tree.
    ("app_state", "panels", 6),
    ("app_state", "app", 5),
    ("app_state", "commands", 5),
    ("app_state", "simulation_analysis_tabs", 3),
    ("app_state", "workflows/file_workflow", 1),
    ("app_state", "workflows/project_workflow", 1),
    // What still reaches the application root rather than the session data.
    // These name `RSpiceApp` itself, so they retire with the `&mut RSpiceApp`
    // burndown, not with a module move.
    ("documents/code_workspace", "app", 2),
    ("documents/model_editor", "app", 1),
    ("documents/result_document", "app", 1),
    ("feature_availability", "app", 1),
    ("browser/file_import", "app", 1),
    ("lifecycle/recovery", "app", 1),
    // What still names the dispatcher rather than the vocabulary it routes.
    // These are shortcut artifacts asking whether a command is *available*,
    // which needs the running application, so they retire with the
    // `&mut RSpiceApp` burndown rather than with another module move.
    ("shortcuts/artifacts", "commands", 3),
    ("shortcuts", "commands", 1),
    // Dispatch reaching up into what it renders. A command should name a
    // route or a state transition, not the widget that draws the result.
    ("commands", "chrome", 10),
    ("commands", "documents/visualization_studio", 6),
    ("commands", "menu_bar", 5),
    ("commands", "surfaces", 4),
    ("commands", "account_organization", 1),
    ("commands", "frame", 1),
    ("commands", "preflight", 1),
    ("commands", "tools/jobs_manager", 1),
    ("commands", "tools/specialist_tool_browser", 1),
    ("chrome", "preflight", 1),
    ("surfaces", "preflight", 1),
    // The shell's own persisted state reaching up into routing, presentation,
    // and the document engines. This is the same inversion the top-level
    // `state -> *` entries describe, one layer down: `WorkbenchState` stores
    // what a surface is showing instead of the surface deriving it.
    ("state", "documents/visualization_studio", 7),
    ("state", "routing/availability", 4),
    ("state", "documents/model_correlation", 2),
    ("state", "documents/model_editor", 2),
    ("state", "lifecycle/recovery", 2),
    ("state", "lifecycle/window_session", 2),
    ("state", "preferences", 1),
    ("state", "simulation_analysis_tabs", 1),
    // The interaction session reaching into the engines that hang off it.
    ("lifecycle/session", "documents/result_document", 2),
    ("lifecycle/session", "cross_probe", 1),
    ("lifecycle/session", "documents/code_workspace", 1),
    ("lifecycle/session", "documents/netlist_document", 1),
    // Browser import/download sitting above what needs it.
    ("documents/code_workspace", "browser/file_import", 7),
    ("workflows/export_workflow", "browser/download", 3),
    ("shortcuts/artifacts", "browser/download", 1),
    ("surfaces", "browser/download", 1),
    ("workflows/project_workflow", "browser/download", 1),
    // Recovery and checkpointing reaching sideways into the workflows.
    ("lifecycle/recovery", "workflows/file_workflow", 6),
    ("lifecycle/recovery_checkpoint", "workflows/file_workflow", 6),
    ("shortcuts/artifacts", "shortcuts/profile_workflow", 4),
    ("lifecycle/recovery", "lifecycle/recovery_checkpoint", 2),
    ("lifecycle/recovery", "workflows/project_workflow", 2),
    ("browser/file_import", "shortcuts/profile_workflow", 1),
    ("shortcuts/library_persistence", "shortcuts/artifacts", 1),
    ("documents/code_workspace", "workflows/export_workflow", 1),
    ("documents/netlist_document", "workflows/netlist_workflow", 1),
    // Presentation reaching sideways into a peer surface.
    ("docks", "documents/visualization_studio", 1),
    ("hardcopy_adapters/sources", "documents/visualization_studio", 1),
    ("surfaces", "documents/visualization_studio", 1),
    ("feature_availability", "design_system", 1),
    // A route describing its own catalog and availability.
    ("routing/surface_route", "routing/surface_catalog", 1),
    ("routing/surface_route", "workflows/capability_workflow", 1),
];

fn workbench_dir() -> PathBuf {
    src_dir().join("workbench")
}

fn workbench_rank(module: &str) -> usize {
    WORKBENCH_LAYERS
        .iter()
        .position(|declared| *declared == module)
        .unwrap_or_else(|| {
            panic!(
                "workbench module `{module}` has no declared rank.\n\
                 Add it to WORKBENCH_LAYERS in tests/module_layering.rs, \
                 placing it strictly after everything it may reference."
            )
        })
}

/// The `workbench` submodule that owns a file under `src/workbench/`.
///
/// `src/workbench/app.rs` and `src/workbench/app/dialogs/state.rs` both belong
/// to `app`. `src/workbench.rs` is the module root and is not scanned: like
/// the crate root, it is allowed to see everything it declares.
///
/// Declared names may be nested — `hardcopy/sources` — so that grouping files
/// into a directory does not collapse their ranks into one. The longest
/// declared prefix wins, which keeps `hardcopy/render` distinct from
/// `hardcopy/print` while a bare `app` still owns everything beneath it.
fn workbench_owning_module(file: &Path) -> Option<String> {
    let relative = file.strip_prefix(workbench_dir()).ok()?;
    let path = relative
        .to_str()?
        .replace('\\', "/")
        .trim_end_matches(".rs")
        .to_owned();
    longest_declared_prefix(&path)
}

/// The longest entry of [`WORKBENCH_LAYERS`] that is a component-wise prefix
/// of `path`, which is itself `/`-separated.
fn longest_declared_prefix(path: &str) -> Option<String> {
    WORKBENCH_LAYERS
        .iter()
        .filter(|declared| {
            path == **declared
                || path
                    .strip_prefix(*declared)
                    .is_some_and(|rest| rest.starts_with('/'))
        })
        .max_by_key(|declared| declared.len())
        .map(|declared| (*declared).to_owned())
}

/// Map each name re-exported by `workbench.rs` back to the module that owns it.
///
/// Two thirds of the shell's public names are re-exported from its root, so
/// `crate::workbench::ResultViewer` and `crate::workbench::result_document::
/// ResultViewer` are the same edge written two ways — 239 references, 17% of
/// the interior total, took the short form. Resolving them here keeps this
/// test measuring coupling rather than import style, and means the re-export
/// cleanup can happen without the numbers moving underneath it.
fn workbench_reexports() -> BTreeMap<String, String> {
    let source = fs::read_to_string(src_dir().join("workbench.rs")).expect("read workbench.rs");
    let declared: Vec<&str> = WORKBENCH_LAYERS.to_vec();
    let mut map = BTreeMap::new();

    for statement in strip_line_comments(&source).split(';') {
        let Some(rest) = statement
            .trim_start()
            .strip_prefix("pub")
            .map(|rest| rest.trim_start().trim_start_matches("(crate)"))
        else {
            continue;
        };
        let Some(path) = rest.trim_start().strip_prefix("use ") else {
            continue;
        };
        let path = path.trim_start();
        let Some((module, names)) = path.split_once("::") else {
            continue;
        };
        let module = module.trim();
        if !declared.contains(&module) {
            continue;
        }
        for name in names.replace(['{', '}'], " ").split(',') {
            let name = name.trim();
            // `pub use foo::bar::Baz;` re-exports through a nested path; the
            // owning top-level module is still `foo`.
            let leaf = name.rsplit("::").next().unwrap_or(name).trim();
            if !leaf.is_empty() && leaf != "self" {
                map.insert(leaf.to_owned(), module.to_owned());
            }
        }
    }
    map
}

/// Normalize a Rust path tail to the `/`-separated form the layer table uses,
/// keeping only the leading identifier run: `hardcopy::sources::Foo` becomes
/// `hardcopy/sources/Foo`, which [`longest_declared_prefix`] then resolves.
fn path_segments(item: &str) -> String {
    item.split("::")
        .map(|segment| {
            segment
                .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_')
                .trim()
        })
        .take_while(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("/")
}

/// Reference counts between `workbench` submodules, keyed `(from, to)`.
fn workbench_edge_counts() -> BTreeMap<(String, String), usize> {
    const PREFIX: &str = "crate::workbench::";
    let reexports = workbench_reexports();
    let mut edges: BTreeMap<(String, String), usize> = BTreeMap::new();

    for file in rust_sources(&workbench_dir()) {
        let Some(from) = workbench_owning_module(&file) else {
            continue;
        };
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
        let mut code = strip_line_comments(&source);

        // `super::` is the same edge written the short way, and 310
        // references took that form — about 15% of the shell's interior —
        // while this test read only `crate::workbench::`. Resolve it against
        // the file's own parent module so grouping files into a directory
        // cannot hide the edges between them: in `workbench/lifecycle/x.rs`,
        // `super::y` is `crate::workbench::lifecycle::y`.
        if let Ok(relative) = file.strip_prefix(workbench_dir()) {
            let mut parent: Vec<&str> = relative
                .to_str()
                .unwrap_or_default()
                .split(['\\', '/'])
                .collect();
            parent.pop(); // the file itself
            let mut resolved = PREFIX.to_owned();
            for segment in parent {
                resolved.push_str(segment);
                resolved.push_str("::");
            }
            code = code.replace("super::", &resolved);
        }

        let mut rest = code.as_str();
        while let Some(index) = rest.find(PREFIX) {
            rest = &rest[index + PREFIX.len()..];

            // `use crate::workbench::{A, b::C};` names two modules in one
            // occurrence. Expanding the group keeps the metric on coupling
            // rather than on whether the author happened to merge imports.
            let names: Vec<String> = if rest.starts_with('{') {
                let Some(close) = rest.find('}') else {
                    continue;
                };
                let group = &rest[1..close];
                rest = &rest[close + 1..];
                group
                    .split(',')
                    .map(|item| path_segments(item.trim()))
                    .filter(|name| !name.is_empty())
                    .collect()
            } else {
                let end = rest
                    .find(|c: char| !c.is_alphanumeric() && c != '_' && c != ':')
                    .unwrap_or(rest.len());
                let path = &rest[..end];
                rest = &rest[end..];
                vec![path_segments(path)]
            };

            for name in names {
                // A path names either a submodule — possibly nested, as
                // `hardcopy::sources` — or a type the root re-exports from
                // one. Anything else is a function defined in `workbench.rs`
                // itself, which belongs to the root, not a module.
                let to = if let Some(module) = longest_declared_prefix(&name) {
                    module
                } else if let Some(owner) = reexports.get(name.as_str()) {
                    owner.clone()
                } else {
                    continue;
                };
                if to != from {
                    *edges.entry((from.clone(), to)).or_default() += 1;
                }
            }
        }
    }
    edges
}

#[test]
fn every_workbench_module_declares_a_layer() {
    let entries = fs::read_dir(workbench_dir()).expect("read src/workbench/");
    let mut undeclared = Vec::new();
    for entry in entries {
        let path = entry.expect("directory entry").path();
        let Some(name) = workbench_owning_module(&path) else {
            continue;
        };
        if !WORKBENCH_LAYERS.contains(&name.as_str()) {
            undeclared.push(name);
        }
    }
    undeclared.sort();
    undeclared.dedup();
    assert!(
        undeclared.is_empty(),
        "new workbench modules without a declared rank: {undeclared:?}\n\
         Add them to WORKBENCH_LAYERS in tests/module_layering.rs. The shell \
         is half this crate; a module with no position in it is a module \
         nobody decided the position of."
    );
}

#[test]
fn workbench_references_respect_the_layer_order() {
    let edges = workbench_edge_counts();
    let mut allowed: BTreeMap<(&str, &str), usize> = ALLOWED_WORKBENCH_VIOLATIONS
        .iter()
        .map(|(from, to, count)| ((*from, *to), *count))
        .collect();
    assert_eq!(
        allowed.len(),
        ALLOWED_WORKBENCH_VIOLATIONS.len(),
        "ALLOWED_WORKBENCH_VIOLATIONS contains a duplicate (from, to) pair"
    );

    let mut new_violations = Vec::new();
    let mut grown = Vec::new();

    for ((from, to), count) in &edges {
        if workbench_rank(from) > workbench_rank(to) {
            continue;
        }
        match allowed.remove(&(from.as_str(), to.as_str())) {
            None => new_violations.push(format!(
                "  {from} -> {to} ({count} references, rank {} -> rank {})",
                workbench_rank(from),
                workbench_rank(to)
            )),
            Some(ceiling) if *count > ceiling => grown.push(format!(
                "  {from} -> {to}: {count} references, ceiling is {ceiling}"
            )),
            Some(_) => {}
        }
    }

    let retired: Vec<String> = allowed
        .keys()
        .map(|(from, to)| format!("  {from} -> {to}"))
        .collect();

    let mut failures = String::new();
    if !new_violations.is_empty() {
        failures.push_str(&format!(
            "\nNew layer-order violations inside `workbench`:\n{}\n\
             This code reaches sideways or upward through the shell. Move it \
             below what needs it, or take the state it uses as a parameter \
             instead of reaching for the module that holds it.\n",
            new_violations.join("\n")
        ));
    }
    if !grown.is_empty() {
        failures.push_str(&format!(
            "\nExisting workbench violations that grew:\n{}\n\
             These edges are being retired, not extended.\n",
            grown.join("\n")
        ));
    }
    if !retired.is_empty() {
        failures.push_str(&format!(
            "\nAllowlist entries for workbench edges that no longer exist:\n{}\n\
             Delete them from ALLOWED_WORKBENCH_VIOLATIONS in \
             tests/module_layering.rs.\n",
            retired.join("\n")
        ));
    }
    assert!(failures.is_empty(), "{failures}");
}

/// The crate's public surface, frozen at its current size.
///
/// `rspice-ui` is an application, not a library: its only consumers are its
/// own `main.rs`, the three wasm entry points in `lib.rs`, the integration
/// tests, and the `license_tool` example. Everything else being `pub` meant
/// no item was ever safely removable, and the layering tables above were
/// standing in for a visibility boundary the compiler could have enforced.
///
/// The count is a ceiling: it may fall, never rise. Prefer `pub(crate)`, and
/// prefer a re-export at the crate root over widening a module.
const MAX_PUBLIC_MODULES: usize = 132;

#[test]
fn the_public_module_surface_does_not_grow() {
    let root = src_dir();
    let mut count = 0;
    for file in rust_sources(&root) {
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
        count += strip_line_comments(&source)
            .lines()
            .filter(|line| line.trim_start().starts_with("pub mod "))
            .count();
    }
    assert!(
        count <= MAX_PUBLIC_MODULES,
        "`pub mod` declarations rose to {count}, ceiling is {MAX_PUBLIC_MODULES}.\n\
         A public module cannot be refactored without checking the whole \
         world. Use `pub(crate) mod`, and re-export from the crate root if an \
         integration test or example genuinely needs the item.\n\
         If a reduction elsewhere justifies the change, lower \
         MAX_PUBLIC_MODULES in tests/module_layering.rs."
    );
}

/// No source file starts with a UTF-8 byte-order mark.
///
/// Three did. A BOM is invisible in every editor that matters, survives
/// copy-paste, and turns the first `//!` of a module into something a doc
/// tool can misread — so it is worth a test rather than a periodic sweep.
#[test]
fn source_files_have_no_byte_order_mark() {
    const BOM: &[u8] = &[0xEF, 0xBB, 0xBF];
    let root = src_dir();
    let offenders: Vec<String> = rust_sources(&root)
        .into_iter()
        .filter(|path| {
            fs::read(path)
                .map(|bytes| bytes.starts_with(BOM))
                .unwrap_or(false)
        })
        .map(|path| {
            path.strip_prefix(&root)
                .unwrap_or(&path)
                .display()
                .to_string()
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "these files begin with a UTF-8 BOM:\n  {}\nStrip the leading EF BB BF bytes.",
        offenders.join("\n  ")
    );
}

/// Lint suppressions in the crate, frozen at the current count.
///
/// There were 110. Removing all 32 `#[allow(dead_code)]` attributes produced
/// zero new warnings — every one of them suppressed nothing, which is worse
/// than noise, because a stale suppression silently absorbs the *next* real
/// warning at that site. `#[allow(unused_mut)]` was stale the same way.
///
/// The 77 that remain are load-bearing: removing one produces the warning it
/// claims to be silencing.
const MAX_LINT_SUPPRESSIONS: usize = 77;

/// The crate does not accumulate lint suppressions.
#[test]
fn lint_suppressions_do_not_grow() {
    let root = src_dir();
    let mut suppressions = 0usize;
    for path in rust_sources(&root) {
        let source = fs::read_to_string(&path).unwrap_or_default();
        suppressions += strip_line_comments(&source).matches("[allow(").count();
    }
    assert!(
        suppressions <= MAX_LINT_SUPPRESSIONS,
        "{suppressions} lint suppressions, ceiling is {MAX_LINT_SUPPRESSIONS}. \
         Fix what the lint is pointing at, or say in a comment why the lint is \
         wrong here."
    );
    assert_eq!(
        suppressions, MAX_LINT_SUPPRESSIONS,
        "{suppressions} lint suppressions, down from {MAX_LINT_SUPPRESSIONS}. \
         Lower MAX_LINT_SUPPRESSIONS to hold the ground."
    );
}

/// Every module says what it is.
///
/// This began as a ceiling over 331 undocumented files, because a burndown
/// that large is not one change. It reached zero, so it is a gate now: a
/// module with no `//!` header fails the build.
///
/// The bar is not a restatement of the module path. Where a module encodes a
/// rule its code does not make obvious — why an order matters, what a receipt
/// is pinned to, what a silent fallback would hide — the header says so.
#[test]
fn modules_explain_themselves() {
    let root = src_dir();
    let undocumented: Vec<String> = rust_sources(&root)
        .into_iter()
        .filter(|path| {
            !fs::read_to_string(path)
                .unwrap_or_default()
                .lines()
                .next()
                .is_some_and(|line| line.trim_start().starts_with("//!"))
        })
        .map(|path| {
            path.strip_prefix(&root)
                .unwrap_or(&path)
                .display()
                .to_string()
        })
        .collect();

    assert!(
        undocumented.is_empty(),
        "these modules do not say what they are:\n  {}\n\
         Open each with a `//!` header. Name what the module holds, and what \
         a reader could not infer from the code.",
        undocumented.join("\n  ")
    );
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
        count += strip_line_comments(&source)
            .matches("&mut RSpiceApp")
            .count();
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
/// The leading `//!` header does not count. The budget exists so the *code*
/// in a file stays reviewable, and a module header is what makes it easier to
/// review — a budget that turns writing one into a build failure is measuring
/// the wrong thing. Everything below the header counts, including ordinary
/// comments.
///
/// Everything currently over budget is listed with its exact length. As with
/// the layering table, the number is a ceiling: a file may shrink, never grow,
/// and an entry that drops under budget must be deleted.
const LINE_BUDGET: usize = 2500;

/// Lines in a file, not counting its leading `//!` module header.
fn budgeted_lines(source: &str) -> usize {
    source
        .lines()
        .skip_while(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with("//!") || trimmed.is_empty()
        })
        .count()
}

const OVERSIZED_FILES: &[(&str, usize)] = &[
    ("workbench/hardcopy_adapters/sources.rs", 4400),
    ("simulation/runner/worker_contract.rs", 4068),
    ("io/project_io/tests.rs", 4156),
    ("workbench/hardcopy_adapters/render.rs", 3956),
    ("workbench/hardcopy_adapters/render/publication.rs", 4123),
    ("state/workspace.rs", 4412),
    ("workbench/documents/visualization_studio.rs", 3501),
    ("state/model_library/qualification.rs", 4479),
    ("workbench/surfaces/model_editor.rs", 4031),
    ("workbench/surfaces/verify.rs", 4202),
    ("workbench/surfaces/simulate.rs", 3483),
    ("results/visualization_document.rs", 4657),
    ("workbench/documents/model_editor.rs", 3915),
    ("results/report_document.rs", 4367),
    ("io/durable_file.rs", 4682),
    ("simulation/plan/model.rs", 4241),
    ("workbench/surfaces/models.rs", 4226),
    ("workbench/documents/result_document/waves.rs", 4191),
    ("state/model_library/manager.rs", 4171),
    ("state/model_library/correlation.rs", 4042),
    ("hardcopy/contract.rs", 4022),
    ("workbench/docks/inspector/design.rs", 3985),
    ("workbench/surfaces/project.rs", 3988),
    ("workbench/feature_availability_data.rs", 3864),
    ("workbench/surfaces/model_correlation.rs", 3624),
    ("workbench/state.rs", 3501),
    ("workbench/hardcopy_adapters/print.rs", 3348),
    ("workbench/app/dialogs/hardcopy/render.rs", 3338),
    ("workbench/lifecycle/project_lifecycle/persistence.rs", 3002),
    ("workbench/feature_availability.rs", 2981),
    ("workbench/tools/project_launcher.rs", 2925),
    ("workbench/docks/navigator.rs", 2875),
    (
        "workbench/app/dialogs/preferences/shortcut_preferences.rs",
        2530,
    ),
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
        let lines = budgeted_lines(
            &fs::read_to_string(&file)
                .unwrap_or_else(|error| panic!("read {}: {error}", file.display())),
        );
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

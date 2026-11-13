//! Architectural layering rule for the `rspice-app` application crate.
//!
//! The application still contains modules awaiting extraction: state, project
//! I/O, orchestration, viewer mathematics, and chrome currently share this
//! crate. The engine already lives in `rspice-core` and `rspice-veriloga`.
//! Until the remaining owners have crate boundaries, the compiler cannot
//! reject every import that inverts this internal architecture.
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
//! both orders are checked the same way.
//!
//! # Allowed edges
//!
//! Every edge that violates the layer order today is listed in
//! [`ALLOWED_VIOLATIONS`]:
//!
//! - A **new** violating edge fails the build.
//! - A listed edge that no longer exists fails the build until its entry is
//!   deleted, so the allowlist cannot rot into a list of problems that no
//!   longer exist.
//!
//! Each entry carries the remediation that retires it.
//!
//! # What counts as a reference
//!
//! An occurrence of `crate::<module>` in a `.rs` file under `src/`, after
//! line comments are stripped. Doc comments describing a relationship are
//! not a dependency, so they are excluded. `use`, type positions, and paths
//! all count equally: the metric tracks coupling, not import style.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Intended layer order, lowest first.
///
/// A module may reference any module in a strictly lower layer. Referencing
/// the same layer or a higher one is a violation: same-layer edges are how
/// cycles reappear, so the ordering is total by construction.
///
/// The ordering encodes the target architecture, not today's reality —
/// [`ALLOWED_VIOLATIONS`] carries the gap between the two.
const LAYERS: &[(&str, u32)] = &[
    // The persisted model, its contracts and its diagnostics live in
    // `rspice-app-state`, and the platform services (project I/O, the
    // lifecycle, recovery, the managed runtime and the build identity) in
    // `rspice-app-services`, and the run tier (the controller, its prepared
    // runs, the run preflight and the runs' slot) in `rspice-app-runs`, below
    // every module here; each may reference them. The workbench framework
    // compiles as `rspice-app-workbench`, and the sheets, settings, results,
    // hardcopy, cloud, models and simulate features as `rspice-app-sheets`,
    // `rspice-app-settings`, `rspice-app-results`, `rspice-app-hardcopy`,
    // `rspice-app-cloud`, `rspice-app-models` and `rspice-app-simulate`, inside
    // `workbench`, where the interior order below ranks them.
    //
    // Locating the production half of a source file that inspects itself.
    // It references nothing, and `automation_workflow`, `simulation` and
    // `workbench` all reference it, so the bottom layer is the only position
    // that clears every inbound edge at once.
    ("source_guard", 0),
    // The design system.
    ("ui", 1),
    // Domain services over the persisted model.
    ("analysis", 4),
    ("automation_workflow", 4),
    ("services", 5),
    ("simulation", 6),
    // Editors and inspectors.
    ("properties", 7),
    ("schematic", 8),
    // The application shell: `RSpiceApp`, chrome, dialogs, surfaces, and the
    // workflows that mutate them. Its interior is ordered separately by
    // [`WORKBENCH_LAYERS`] — see `workbench_references_respect_the_layer_order`.
    ("workbench", 9),
    // Test-only: whole-application tests of one slice's behaviour, which
    // construct `AppState` or `RSpiceApp` and so sit above the shell.
    ("feature_tests", 10),
];

/// Layer-order violations present in the tree today.
///
/// Do not add entries to unblock new code — a new violation means the code is
/// in the wrong module.
const ALLOWED_VIOLATIONS: &[(&str, &str)] = &[
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
    // `schematic -> workbench` is the same shape as the retired
    // `state -> workbench` edge: types the project *persists* still live in
    // the shell, so a lower layer must name the shell to read them. It retires
    // the way `state -> workbench` did — by moving the persisted contract down,
    // not by narrowing a signature.
    ("schematic", "workbench"),
    // Editors and orchestration referencing each other sideways; retired by
    // the granularity folds and the `properties`/`panels` merge.
    // Result aggregation, optimization execution, netlisting and materialized
    // OP dispatch remain app-owned until R04/R11 finish their extraction.
    ("services", "simulation"),
];

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// File stems of the crate's entry points, read from `Cargo.toml`.
///
/// A `[lib]` or `[[bin]]` root is not a module: it links against the whole
/// crate, declares no position of its own, and nothing can reference it.
/// This used to be the hardcoded pair `lib`/`main`, which silently excluded
/// `src/worker_main.rs` — the `rspice-ui-worker` bin root — and demanded a
/// layer for a file that has no place in the order. Resolving the roots from
/// the manifest that defines them keeps the exemption honest: adding a bin
/// exempts exactly that root, and renaming one cannot leave a stale name
/// behind.
///
/// Only roots sitting directly in `src/` are collected, because those are the
/// only ones the two callers can mistake for a top-level module. Cargo's own
/// defaults — `src/lib.rs` and `src/main.rs`, which need no explicit `path` —
/// are seeded for the same reason.
fn crate_root_stems() -> &'static [String] {
    static ROOTS: OnceLock<Vec<String>> = OnceLock::new();
    ROOTS.get_or_init(|| {
        let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let manifest = fs::read_to_string(&manifest_path)
            .unwrap_or_else(|error| panic!("read {}: {error}", manifest_path.display()));
        let mut stems = vec!["lib".to_owned(), "main".to_owned()];
        let mut section = String::new();
        for line in manifest.lines() {
            let line = line.trim();
            if line.starts_with('[') {
                section = line.to_owned();
                continue;
            }
            if section != "[lib]" && section != "[[bin]]" {
                continue;
            }
            let Some(value) = line
                .strip_prefix("path")
                .map(str::trim_start)
                .and_then(|rest| rest.strip_prefix('='))
            else {
                continue;
            };
            let value = value.trim().trim_matches('"');
            let path = Path::new(value);
            if path.parent() != Some(Path::new("src")) {
                continue;
            }
            let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            if !stems.iter().any(|known| known == stem) {
                stems.push(stem.to_owned());
            }
        }
        stems
    })
}

fn is_crate_root_stem(stem: &str) -> bool {
    crate_root_stems().iter().any(|root| root == stem)
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
    // A root that moved leaves an empty directory or none; either way every
    // check over it would pass on nothing.
    assert!(
        !found.is_empty(),
        "{} holds no Rust source; a scan that reaches nothing passes forever",
        root.display()
    );
    found.sort();
    found
}

/// The application crates every whole-file scan must find: this crate and the
/// eleven split out of it so far. Discovery may find more, never fewer.
const KNOWN_APP_CRATES: [&str; 12] = [
    "rspice-app",
    "rspice-app-cloud",
    "rspice-app-hardcopy",
    "rspice-app-models",
    "rspice-app-results",
    "rspice-app-runs",
    "rspice-app-services",
    "rspice-app-settings",
    "rspice-app-sheets",
    "rspice-app-simulate",
    "rspice-app-state",
    "rspice-app-workbench",
];

/// Every application crate's `src`: this crate's and each `rspice-app-*`
/// crate's, the model, the services, the run tier, the workbench framework and
/// the features that compile as crates of their own.
///
/// The whole-file conventions below hold in all of them. A module that moves
/// to another application crate keeps its path under that crate's `src` and is
/// read there, so a move neither drops it from a scan nor renames its entries
/// here, and a new application crate is read without an edit. A crate the
/// discovery misses fails here rather than leaving every scan to pass without
/// it.
fn app_src_dirs() -> Vec<PathBuf> {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crates directory");
    let mut dirs: Vec<PathBuf> = fs::read_dir(crates)
        .unwrap_or_else(|error| panic!("read {}: {error}", crates.display()))
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == "rspice-app" || name.starts_with("rspice-app-"))
                && path.join("Cargo.toml").is_file()
        })
        .map(|path| path.join("src"))
        .collect();
    dirs.sort();
    let found: BTreeSet<String> = dirs.iter().map(|dir| crate_of(dir)).collect();
    for known in KNOWN_APP_CRATES {
        assert!(
            found.contains(known),
            "application crate {known} was not found under {}; every whole-file \
             scan would pass without reading it",
            crates.display()
        );
    }
    dirs
}

/// The crate a `src` directory belongs to.
fn crate_of(src: &Path) -> String {
    src.parent()
        .and_then(Path::file_name)
        .expect("a crate directory")
        .to_string_lossy()
        .into_owned()
}

/// One application source: the file, its crate and the name a report gives it.
struct AppSource {
    file: PathBuf,
    krate: String,
    shown: String,
}

/// Every Rust source of every application crate.
fn app_sources() -> Vec<AppSource> {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the crates directory");
    app_src_dirs()
        .iter()
        .flat_map(|root| {
            let krate = crate_of(root);
            rust_sources(root).into_iter().map(move |file| {
                let shown = file
                    .strip_prefix(crates)
                    .unwrap_or(&file)
                    .display()
                    .to_string()
                    .replace('\\', "/");
                AppSource {
                    file,
                    krate: krate.clone(),
                    shown,
                }
            })
        })
        .collect()
}

/// Fails unless `scan` read a source of every discovered application crate,
/// so a crate a scan stops reaching fails instead of passing on nothing.
fn assert_reads_every_app_crate<'a>(scan: &str, read: impl IntoIterator<Item = &'a str>) {
    let read: BTreeSet<&str> = read.into_iter().collect();
    let missed: Vec<String> = app_src_dirs()
        .iter()
        .map(|dir| crate_of(dir))
        .filter(|krate| !read.contains(krate.as_str()))
        .collect();
    assert!(
        missed.is_empty(),
        "{scan} read no source of {}; a scan that reaches nothing there passes forever",
        missed.join(", ")
    );
}

/// The top-level module a source file belongs to.
///
/// The crate roots — `lib.rs` and every `[[bin]]` entry point, resolved by
/// [`crate_root_stems`] — are allowed to see everything, so they belong to no
/// module. Every *other* file sitting directly in `src/` is a module in its
/// own right and is layered like a directory: both the `foo.rs` half of the
/// `foo.rs`-beside-`foo/` convention, and standalone modules such as
/// `output_spec` that have no directory at all.
///
/// This used to return `None` for every root-level file, which meant the
/// `mod.rs`-replacement files — `state.rs`, `simulation.rs`, `workbench.rs`
/// and the rest — had their outbound edges counted nowhere, and a module
/// with no directory escaped the order entirely in both directions.
fn owning_module(root: &Path, file: &Path) -> Option<String> {
    let relative = file.strip_prefix(root).ok()?;
    let mut components = relative.components();
    let first = components.next()?.as_os_str().to_str()?.to_owned();
    if components.next().is_none() {
        let stem = Path::new(&first).file_stem()?.to_str()?;
        if is_crate_root_stem(stem) {
            return None;
        }
        return Some(stem.to_owned());
    }
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
    let mut checked = 0_usize;
    for entry in entries {
        let path = entry.expect("directory entry").path();
        // A top-level module is either a directory or a `.rs` file sitting
        // directly in `src/`. Checking only directories let `output_spec`
        // sit in the tree with no declared position at all.
        let name = if path.is_dir() {
            path.file_name()
                .and_then(|name| name.to_str())
                .expect("module directory name")
                .to_owned()
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            let stem = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .expect("module file name");
            // Crate roots are entry points, not modules — see
            // `crate_root_stems`.
            if is_crate_root_stem(stem) {
                continue;
            }
            stem.to_owned()
        } else {
            continue;
        };
        checked += 1;
        if !LAYERS.iter().any(|(declared, _)| *declared == name) && !undeclared.contains(&name) {
            undeclared.push(name);
        }
    }
    assert!(
        checked > 0,
        "{} holds no module; a scan that reaches nothing passes forever",
        root.display()
    );
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
    let mut allowed: BTreeSet<(&str, &str)> = ALLOWED_VIOLATIONS.iter().copied().collect();
    assert_eq!(
        allowed.len(),
        ALLOWED_VIOLATIONS.len(),
        "ALLOWED_VIOLATIONS contains a duplicate (from, to) pair"
    );

    let mut new_violations = Vec::new();

    for ((from, to), count) in &edges {
        if layer_of(from) > layer_of(to) {
            continue;
        }
        if !allowed.remove(&(from.as_str(), to.as_str())) {
            new_violations.push(format!(
                "  {from} -> {to} ({count} references, layer {} -> layer {})",
                layer_of(from),
                layer_of(to)
            ));
        }
    }

    // Entries left over describe edges that no longer exist. Deleting them
    // is the point of the exercise, so the allowlist must be kept honest.
    let retired: Vec<String> = allowed
        .iter()
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
    // The requests the parts of the shell raise for one another and the
    // frame points that take them. It names nothing of the shell's, so every
    // part can raise or take one.
    "framework",
    // What the sheets feature's commands declare: keys, labels and bindings,
    // values the command vocabulary lists in the shell's order.
    "sheets/commands",
    // What the settings feature's commands declare, the same way.
    "settings/commands",
    // What the hardcopy feature's commands declare, the same way.
    "hardcopy/commands",
    // What the cloud feature's commands declare, the same way.
    "cloud/commands",
    // What the models feature's commands declare, the same way.
    "models/commands",
    // What the simulate feature's commands declare, the same way.
    "simulate/commands",
    // What the code feature's commands declare, the same way.
    "code/commands",
    // What the design feature's commands declare, the same way.
    "design/commands",
    // The documents module's own vocabulary: which Code page is showing and
    // what a source-file action is. Each document engine has its own entry;
    // a document file without one is owned here, where its first reach into
    // the shell fails the order.
    "documents",
    "system_contrast",
    "routing/surface_route",
    "routing/surface_catalog",
    "routing/availability",
    "routing/navigation",
    // The workbench's session and the pages' persisted views. The session
    // holds the route and its history, so it sits above routing; the routes
    // name a workspace, which is the model's.
    "state",
    // What one feature shows that another reads, where every answer is a
    // value of a lower crate or names the session, so the code that answers
    // it sits anywhere above the session.
    "framework/ports",
    // What the Models & PDKs workspace keeps between frames. The models slot
    // holds it and the session wire persists it, so it ranks below both; it
    // names only the session's vocabulary and lower crates.
    "models/workspace_state",
    // What the Project workspace and the project launcher keep between
    // frames. The project slot holds it and the session wire persists it, so
    // it ranks below both; it names nothing of the workbench.
    "project/workspace_state",
    // What the Project > Configuration page keeps between frames. The project
    // slot holds it, so it ranks below it; it names only lower crates.
    "project/configuration_state",
    // What the project feature's commands declare, the same way as the
    // other features' declarations. They name the Project workspace's
    // pages, so they rank above them.
    "project/commands",
    "logging",
    "layout",
    // The interface text catalog and its locales. It names nothing of the
    // workbench; the command vocabulary reads the labels it owns, and the
    // editors, docks and surfaces read it.
    "localization",
    // Reads the accessibility tree egui builds and writes to the page: it needs
    // nothing of the shell but the words.
    "browser/accessible_mirror",
    // What the results feature's commands declare, the same way. Its labels
    // read the interface text catalog, so it ranks above it.
    "results/commands",
    // The command vocabulary: what commands exist, their portable identity,
    // and the label a menu shows. Shortcut profiles, preferences, and the
    // artifact writers name a command without reaching the dispatcher that
    // runs it, which is up at `commands`.
    "commands/vocabulary",
    "shortcuts",
    "design_system",
    // The text model and editing commands the source editors share, and the
    // virtualized editor drawn over them.
    "documents/text_document_model",
    "documents/text_editor_commands",
    "documents/virtual_text_editor",
    // Document engines and the interaction session they hang off.
    "lifecycle/session",
    "documents/netlist_document",
    "documents/result_document",
    "documents/code_workspace",
    "documents/model_editor",
    "documents/model_correlation",
    // What the release gate concludes for each model, from the model library
    // and the open candidate. The models slot answers another feature's
    // question with it, so it ranks below the slot.
    "models/qualification",
    "lifecycle/window_session",
    // The shortcut libraries' persistence and the artifacts they write.
    "shortcuts/library_persistence",
    "shortcuts/artifacts",
    // Application hardcopy source resolution and host printing adapters.
    "hardcopy_adapters/sources",
    // Test fixtures that render resolved sources; the tests of the session
    // and the dialogs above read them.
    "hardcopy_adapters/tests",
    "hardcopy_adapters/print",
    // The session aggregate: the design, the simulation, the document engines'
    // session state, and the dialogs' visibility. It sits directly below the
    // frame loop because it aggregates the whole session — a module that only
    // needs one part of it should take that part as a parameter, which is what
    // retires the `-> app_state` entries below.
    "app_state",
    // The host the workbench's containers are drawn for, keyed by the
    // workbench's own types. It hands them the session, so it sits above the
    // session aggregate; every container that takes it sits above it.
    "framework/workbench_host",
    // A feature, the context its commands run in and the registry they are
    // listed in. A context holds the model, so it sits above the session
    // aggregate; every feature sits above it.
    "framework/feature",
    // The run tier's host over a feature command's context: it takes the
    // context's parts, the peers and the registry, so it sits above the
    // feature and below every feature that starts a run.
    "framework/run_view",
    // The live-session engine the application pumps each frame, and result
    // import. Both take the session aggregate, not the application root.
    "live_session",
    "workflows/result_import_workflow",
    "workflows/result_import_adapters",
    // The publication snapshot builder: reads the session aggregate and the
    // hardcopy adapters' resolved scenes to produce the sealed interchange
    // snapshot. It mutates nothing, but it reads the whole session.
    "hardcopy_adapters/publication_snapshot",
    // Demonstration circuits and the shared hierarchy reference populate
    // AppState through its APIs. They do not depend on the application root;
    // application validation tests can use the same authored projects.
    "examples",
    // The one name the user is known by. It reads the cloud session and the
    // license, never the application.
    "user_identity",
    // The models feature's browser PDK store: the application starts its
    // first read at launch, and the feature takes what it settled each
    // frame. It names the session's parts and lower crates, never the
    // application, so the root starts it from above.
    "models/pdk_persistence",
    // The models feature's browser pack restore: the application starts it
    // at launch, and the feature takes the packs it kept on the frame they
    // land. It names the hub's handle, the model and the feature's view
    // state, never the application, so the root starts it from above.
    "models/pack_persistence",
    // The application root. Everything above operates on it; everything below
    // is operated on by it.
    "app",
    // Workflows that mutate application state.
    "workflows/file_workflow",
    "workflows/project_workflow",
    "workflows/netlist_workflow",
    "workflows/foreign_schematic_workflow",
    "shortcuts/profile_workflow",
    "workflows/file_actions",
    // The shell's transactions over the whole project, run where a feature's
    // work asks for one: they take the project workflows and the feature that
    // asked, so they sit above both.
    "session/project_transactions",
    // The netlist import's project replacements: they take the netlist
    // workflow's parts, so they sit above it.
    "session/netlist_import_transactions",
    // Command dispatch.
    "commands",
    // Following a finding to the object it is about. It reads a route a page
    // recorded and opens its destination, which takes the command vocabulary
    // and the document engines — so it sits above the commands and below
    // every surface that offers one, none of which it references.
    "diagnostic_routes",
    // Chrome, docks, surfaces, and the rest of the presentation layer.
    "chrome",
    "docks",
    "menu_bar",
    // The state each feature owns, one slot per feature. A slot names the
    // dialog and draft types its feature renders, so it ranks above them;
    // the session aggregate that holds the slots reaches up to it.
    "slots",
    // The sheets feature: its commands' answers over its own slot, the model
    // and the session.
    "sheets",
    // The settings feature: its commands' answers over its own slot, the
    // model and the session, and the keys its shortcut tables name.
    "settings",
    // The hardcopy feature: its commands' answers over its own slot, the
    // model, the session and what the results workspace shows.
    "hardcopy",
    // The results feature: its commands' answers over its own slot, the
    // model, the session and the platform state.
    "results",
    // The account console, which the cloud feature registers and draws on
    // the shell's route.
    "account_organization",
    // The cloud feature: its commands' answers over its own slot, the model,
    // the session, and the account service and live-session engine it runs.
    "cloud",
    // The models feature's home.
    "models",
    // The project feature's home.
    "project",
    // The simulate feature's home.
    "simulate",
    // The design feature's home.
    "design",
    // The code feature's home.
    "code",
    // The shell's list of registered features, which builds each one's
    // context from the application state.
    "features",
    // The one dispatcher every command runs through: its prologue, then the
    // feature that declares the command or the legacy table the commands
    // module keeps.
    "dispatch",
    "surfaces",
    "tools/notification_center",
    "tools/project_launcher",
    "tools/jobs_manager",
    "preflight",
    "cross_probe",
    "browser/navigation",
    // Opening an example replaces the session with a new project and saves
    // it, through the project workflow; its test runs the plan through the
    // preflight the frame runs.
    "tools/examples_browser",
    // The shell's host: which owner draws at each contribution point the
    // docks and surfaces leave to one. It names every owner it resolves to,
    // so it ranks above all of them; the containers reach it only through
    // the framework's trait.
    "shell_host",
    // The frame is painted last and may reach anything below it.
    "frame",
    // A run driven through whole frames: it opens an example project and
    // reads what the frame drew, so it ranks above both.
    "app/run_flow_tests",
];

/// Order violations inside `workbench` today.
///
/// Same rules as [`ALLOWED_VIOLATIONS`]: a new edge fails, and an entry whose
/// edge is gone fails until it is deleted. A new edge means the code is in the
/// wrong module rather than that the table needs another row.
///
/// The concentration is the finding: `app` and `app_state` carry a third of
/// the list between them. That is what splitting them apart exposed. Before
/// the split, `AppState` and `RSpiceApp` shared a module, so "this module
/// needs the session data" and "this module calls the application root" were
/// the same measurement. They are now separate problems with separate fixes —
/// the first retires by passing a slice of the session instead of the whole
/// aggregate, the second by narrowing signatures that take `&mut RSpiceApp`.
const ALLOWED_WORKBENCH_VIOLATIONS: &[(&str, &str)] = &[
    // `app` is the frame loop, and it drives everything. These are the calls
    // out of `RSpiceApp` into the presentation and workflow layers it renders.
    // They retire as the frame loop stops calling and starts setting state
    // that the called module reads — the pattern `frame` already follows.
    ("app", "workflows/project_workflow"),
    ("app", "commands"),
    ("app", "chrome"),
    ("app", "workflows/file_workflow"),
    ("app", "browser/navigation"),
    ("app", "workflows/file_actions"),
    ("app", "frame"),
    ("app", "cross_probe"),
    // The source find dialog's test, which only seeds and reads code's state,
    // runs over the code feature's harness, as code's own seeders do.
    ("app", "code"), // retires at the code flip
    // `AppState` is the session aggregate, so it inherits two problems from
    // the root it was split out of.
    //
    // Downward: a module that needs one slice of the session takes the whole
    // aggregate. Each of these retires by passing the slice — the schematic,
    // the workspace, the netlist session — instead of `&AppState`.
    ("documents/result_document", "app_state"),
    // The results feature's test host holds the results slot whose parts its
    // tests run against, as the session does.
    ("documents/result_document", "slots"),
    // Upward: fields whose types live in the feature that renders them, plus
    // gating methods that query a module above; `-> app` also counts the
    // session wire naming the bulk-edit filters it keeps, which live with the
    // dialog that edits them. `-> slots` is the feature slots the state
    // holds, the platform state it names for the services below it, and the
    // session wire reading what it keeps of them; slots rank with the dialog
    // types they name.
    ("app_state", "app"),
    ("app_state", "slots"),
    ("app_state", "workflows/file_workflow"),
    // The code workspace's and the netlist document's tests that only seed
    // and read state run over the code feature on its own, and the feature
    // builds that harness, as the models feature builds the model editor's
    // below.
    ("documents/code_workspace", "code"), // retires at the code flip
    ("documents/netlist_document", "code"), // retires at the code flip
    // The model editor document's tests run over the models feature on its
    // own, and the feature builds that harness, as the results feature's test
    // host holds the slot its parts run against. The row retires when the
    // document moves into the models crate with the feature, where the edge
    // is internal to it.
    ("documents/model_editor", "models"),
    // Dispatch reaching up into what it renders. A command should name a
    // route or a state transition, not the widget that draws the result.
    ("commands", "chrome"),
    ("commands", "menu_bar"),
    ("surfaces", "preflight"),
    // The shell's own persisted state reaching up into the window session.
    // This is the same inversion the top-level `state -> *` entries describe,
    // one layer down: `WorkbenchSession` stores what a surface is showing
    // instead of the surface deriving it.
    ("state", "lifecycle/window_session"),
    // The framework dialogs the workbench holds: the window-session manager,
    // the command palette, and the confirmation and project-review dialogs.
    // Their types still live with the application root.
    ("state", "app"),
    // The text locale the workbench holds: its type lives with the message
    // catalog.
    ("state", "localization"),
    // The design slot holds the schematic cross-probe runtime, whose type
    // lives with the frame pass that synchronizes it.
    ("slots", "cross_probe"),
    // Artifacts reaching sideways into the workflows.
    ("shortcuts/artifacts", "shortcuts/profile_workflow"),
    // Result import opens the imported result's tab through the document
    // bar, and the dispatcher that starts an import ranks below the bar.
    ("workflows/result_import_workflow", "chrome"),
    ("shortcuts/library_persistence", "shortcuts/artifacts"),
    // Presentation reaching sideways into a peer surface.
    ("documents/result_document", "chrome"),
    // A route describing its own catalog and availability.
    ("routing/surface_route", "routing/surface_catalog"),
];

fn workbench_dir() -> PathBuf {
    src_dir().join("workbench")
}

/// The workbench's framework modules compile as `rspice-app-workbench`, and the
/// sheets, settings, results, hardcopy, cloud, models and simulate features as
/// `rspice-app-sheets`, `rspice-app-settings`, `rspice-app-results`,
/// `rspice-app-hardcopy`, `rspice-app-cloud`, `rspice-app-models` and
/// `rspice-app-simulate`, at the paths they had here, and keep their rank in
/// the same order: each crate's `src/workbench` is scanned beside this crate's.
fn workbench_roots() -> [PathBuf; 9] {
    [
        workbench_dir(),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-workbench/src/workbench"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-sheets/src/workbench"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-settings/src/workbench"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-results/src/workbench"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-hardcopy/src/workbench"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-cloud/src/workbench"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-models/src/workbench"),
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../rspice-app-simulate/src/workbench"),
    ]
}

/// A source's path under whichever workbench root holds it.
fn workbench_relative(file: &Path) -> Option<PathBuf> {
    workbench_roots()
        .iter()
        .find_map(|root| file.strip_prefix(root).ok().map(Path::to_path_buf))
}

/// Every source under either workbench root.
fn workbench_sources() -> Vec<PathBuf> {
    workbench_roots()
        .iter()
        .flat_map(|root| rust_sources(root))
        .collect()
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
    let relative = workbench_relative(file)?;
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

    for file in workbench_sources() {
        let Some(from) = workbench_owning_module(&file) else {
            continue;
        };
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
        let mut code = strip_line_comments(&source);

        // `super::` is the same edge written the short way, and 310
        // references took that form — about 15% of the shell's interior —
        // while this test read only `crate::workbench::`. Resolve it against
        // the file's own module so grouping files into a directory cannot
        // hide the edges between them: in `workbench/lifecycle/x.rs`,
        // `super::y` is `crate::workbench::lifecycle::y`.
        if let Some(relative) = workbench_relative(&file) {
            let module: Vec<&str> = relative
                .to_str()
                .unwrap_or_default()
                .trim_end_matches(".rs")
                .split(['\\', '/'])
                .collect();
            code = resolve_super_chains(&code, &module);
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

/// Rewrite every `super::` chain in `code` as the `crate::workbench::` path it
/// names, in a file whose module under `workbench` is `module`.
///
/// A chain climbs one module per `super`: in `workbench/a/b/c.rs`,
/// `super::super::d` is `crate::workbench::a::d`. Rewriting each `super::` as
/// the file's parent, as this once did, glued the two parents of such a chain
/// into one path that named the file's own module, so about a hundred
/// references between workbench modules went unseen. Inside an inline
/// `mod tests { .. }` the chain starts from that block, one module deeper than
/// the file. A chain that climbs out of `workbench` names no workbench module,
/// and is written from `crate::` so the scan passes over it.
fn resolve_super_chains(code: &str, module: &[&str]) -> String {
    const SUPER: &str = "super::";
    let mut resolved = String::with_capacity(code.len());
    let mut inline: Vec<(&str, i64)> = Vec::new();
    let mut depth: i64 = 0;
    for line in code.lines() {
        if let Some(name) = inline_module_name(line) {
            inline.push((name, depth));
        }
        let mut rest = line;
        while let Some(index) = rest.find(SUPER) {
            resolved.push_str(&rest[..index]);
            let starts_a_path = rest[..index]
                .chars()
                .next_back()
                .is_none_or(|previous| !previous.is_alphanumeric() && previous != '_');
            let mut tail = &rest[index..];
            if !starts_a_path {
                resolved.push_str(SUPER);
                rest = &tail[SUPER.len()..];
                continue;
            }
            let mut climb = 0;
            while let Some(after) = tail.strip_prefix(SUPER) {
                climb += 1;
                tail = after;
            }
            let mut here: Vec<&str> = module.to_vec();
            here.extend(inline.iter().map(|(name, _)| *name));
            if climb <= here.len() {
                here.truncate(here.len() - climb);
                resolved.push_str("crate::workbench::");
                for segment in here {
                    resolved.push_str(segment);
                    resolved.push_str("::");
                }
            } else {
                resolved.push_str("crate::");
            }
            rest = tail;
        }
        resolved.push_str(rest);
        resolved.push('\n');
        depth += line.matches('{').count() as i64 - line.matches('}').count() as i64;
        while inline
            .last()
            .is_some_and(|(_, opened_at)| depth <= *opened_at)
        {
            inline.pop();
        }
    }
    resolved
}

/// The name of the inline module `line` opens, as in `mod tests {` or
/// `#[cfg(test)] pub(crate) mod helpers {`, and `None` for any other line,
/// including a `mod name;` that declares a file.
fn inline_module_name(line: &str) -> Option<&str> {
    let mut rest = line.trim_start();
    while rest.starts_with("#[") {
        rest = rest[rest.find(']')? + 1..].trim_start();
    }
    if let Some(after) = rest.strip_prefix("pub") {
        rest = after.trim_start();
        if rest.starts_with('(') {
            rest = rest[rest.find(')')? + 1..].trim_start();
        }
    }
    let rest = rest.strip_prefix("mod ")?.trim_start();
    let end = rest.find(|c: char| !c.is_alphanumeric() && c != '_')?;
    let (name, after) = rest.split_at(end);
    (!name.is_empty() && after.trim_start().starts_with('{')).then_some(name)
}

/// A `super::` chain names the module it climbs to: one module per `super`,
/// counted from inside any inline module, and from `crate::` once it leaves
/// `workbench`. A word that merely ends in `super` is not a chain.
#[test]
fn super_chains_resolve_to_the_module_they_name() {
    let file = ["a", "b", "c"];
    assert_eq!(
        resolve_super_chains("use super::d::X;", &file),
        "use crate::workbench::a::b::d::X;\n"
    );
    assert_eq!(
        resolve_super_chains("use super::super::d::X;", &file),
        "use crate::workbench::a::d::X;\n"
    );
    assert_eq!(
        resolve_super_chains(
            "#[cfg(test)]\nmod tests {\n    use super::super::d;\n}\nuse super::e;",
            &file
        ),
        "#[cfg(test)]\nmod tests {\n    use crate::workbench::a::b::d;\n}\nuse crate::workbench::a::b::e;\n"
    );
    assert_eq!(
        resolve_super_chains("use super::super::super::super::f;", &file),
        "use crate::f;\n"
    );
    assert_eq!(resolve_super_chains("my_super::g", &file), "my_super::g\n");
}

/// Whether `file` is the module file of a directory beside it that does
/// nothing but declare that directory's modules. Such a file holds no code to
/// rank; each module it declares is ranked on its own.
fn declares_only_submodules(file: &Path) -> bool {
    if !file.with_extension("").is_dir() {
        return false;
    }
    let source =
        fs::read_to_string(file).unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
    strip_line_comments(&source)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .all(|line| line.starts_with("#[") || is_module_declaration(line))
}

/// `mod x;` behind any visibility: `pub`, `pub(crate)`, `pub(in path)`. A
/// module a lower application crate compiles, re-exported here at the path it
/// had (`pub(crate) use rspice_app_services::workbench::workflows::x;`), is
/// the same declaration.
fn is_module_declaration(line: &str) -> bool {
    let Some(declaration) = line.strip_suffix(';') else {
        return false;
    };
    let words = declaration.split_whitespace().collect::<Vec<_>>();
    let visibility = match words.as_slice() {
        [visibility @ .., "mod", _] => visibility,
        [visibility @ .., "use", path]
            if path.starts_with("rspice_app_") && !path.contains(['{', '*']) =>
        {
            visibility
        }
        _ => return false,
    };
    visibility
        .iter()
        .all(|word| word.starts_with("pub") || word.ends_with(')'))
}

/// A file the order cannot place is a file whose references nothing counts:
/// [`workbench_edge_counts`] skips every file with no owning module. So every
/// source file under `src/workbench/` must have one, at any depth, other than
/// a directory's module file that only declares the directory's modules.
#[test]
fn every_workbench_module_declares_a_layer() {
    let mut undeclared = Vec::new();
    for file in workbench_sources() {
        if workbench_owning_module(&file).is_none() && !declares_only_submodules(&file) {
            let relative = workbench_relative(&file)
                .expect("a source under src/workbench/")
                .with_extension("");
            undeclared.push(relative.to_string_lossy().replace('\\', "/"));
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
    let mut allowed: BTreeSet<(&str, &str)> =
        ALLOWED_WORKBENCH_VIOLATIONS.iter().copied().collect();
    assert_eq!(
        allowed.len(),
        ALLOWED_WORKBENCH_VIOLATIONS.len(),
        "ALLOWED_WORKBENCH_VIOLATIONS contains a duplicate (from, to) pair"
    );

    let mut new_violations = Vec::new();

    for ((from, to), count) in &edges {
        if workbench_rank(from) > workbench_rank(to) {
            continue;
        }
        if !allowed.remove(&(from.as_str(), to.as_str())) {
            new_violations.push(format!(
                "  {from} -> {to} ({count} references, rank {} -> rank {})",
                workbench_rank(from),
                workbench_rank(to)
            ));
        }
    }

    let retired: Vec<String> = allowed
        .iter()
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

/// The crate has no `pub mod` declarations, and must not gain one.
///
/// `rspice-app` is an application, not a library: its only consumers are its
/// own `main.rs`, the three wasm entry points in `lib.rs`, and the integration
/// tests. Everything they need is re-exported from the crate root, so no
/// module needs to be public at all.
///
/// This is not a style preference. A `pub` module is one the compiler must
/// assume has callers it cannot see, so it will not report an unreachable
/// item inside it. While the eight domain roots were `pub`, dead-code
/// analysis was disabled across 262k lines -- 45% of the crate -- and the
/// layering tables above were standing in for a visibility boundary the
/// compiler could have enforced. Closing them took reported dead code from
/// 259 to 1531.
///
/// To reach something from a test, add a re-export in `lib.rs`. Adding a
/// `pub mod` re-opens the hole for that whole subtree.
#[test]
fn no_module_is_public() {
    let root = src_dir();
    let mut public = Vec::new();
    for file in rust_sources(&root) {
        let source = fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
        if strip_line_comments(&source)
            .lines()
            .any(|line| line.trim_start().starts_with("pub mod "))
        {
            public.push(file.display().to_string());
        }
    }
    assert!(
        public.is_empty(),
        "these files declare a `pub mod`:\n  {}\n\
         A public module cannot be refactored without checking the whole \
         world. Use `pub(crate) mod`, and re-export from the crate root if an \
         integration test or example genuinely needs the item.",
        public.join("\n  ")
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
    let sources = app_sources();
    assert_reads_every_app_crate(
        "the byte-order-mark scan",
        sources.iter().map(|source| source.krate.as_str()),
    );
    let offenders: Vec<String> = sources
        .into_iter()
        .filter(|source| {
            fs::read(&source.file)
                .map(|bytes| bytes.starts_with(BOM))
                .unwrap_or(false)
        })
        .map(|source| source.shown)
        .collect();
    assert!(
        offenders.is_empty(),
        "these files begin with a UTF-8 BOM:\n  {}\nStrip the leading EF BB BF bytes.",
        offenders.join("\n  ")
    );
}

/// Every module says what it is.
///
/// A module with no `//!` header fails the build.
///
/// The bar is not a restatement of the module path. Where a module encodes a
/// rule its code does not make obvious — why an order matters, what a receipt
/// is pinned to, what a silent fallback would hide — the header says so.
#[test]
fn modules_explain_themselves() {
    let sources = app_sources();
    assert_reads_every_app_crate(
        "the module-doc scan",
        sources.iter().map(|source| source.krate.as_str()),
    );
    let undocumented: Vec<String> = sources
        .into_iter()
        .filter(|source| {
            !fs::read_to_string(&source.file)
                .unwrap_or_default()
                .lines()
                .next()
                .is_some_and(|line| line.trim_start().starts_with("//!"))
        })
        .map(|source| source.shown)
        .collect();

    assert!(
        undocumented.is_empty(),
        "these modules do not say what they are:\n  {}\n\
         Open each with a `//!` header. Name what the module holds, and what \
         a reader could not infer from the code.",
        undocumented.join("\n  ")
    );
}

/// The application's crates use one module-file convention: `foo.rs` beside `foo/`.
///
/// It previously mixed both forms, sometimes inside the same directory —
/// `dialogs/` held `hardcopy.rs` + `hardcopy/` next to `library/mod.rs`.
/// Picking one makes a module's own file findable by name instead of by
/// remembering which style its author used.
#[test]
fn modules_use_the_sibling_file_convention_not_mod_rs() {
    let sources = app_sources();
    assert_reads_every_app_crate(
        "the sibling-file scan",
        sources.iter().map(|source| source.krate.as_str()),
    );
    let offenders: Vec<String> = sources
        .into_iter()
        .filter(|source| source.file.file_name().is_some_and(|name| name == "mod.rs"))
        .map(|source| source.shown)
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
fn every_production_dialog_callsite_supplies_a_description() {
    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut sources: Vec<PathBuf> = app_sources()
        .into_iter()
        .map(|source| source.file)
        .collect();
    sources.extend(rust_sources(&source_root.join("rspice-ui-kit/src")));
    sources.extend(rust_sources(&source_root.join("rspice-results-ui/src")));

    let mut audited = 0;
    let mut read = BTreeSet::new();
    let mut missing = Vec::new();
    for path in sources {
        let relative = path
            .strip_prefix(source_root)
            .expect("source beneath crate root")
            .to_string_lossy()
            .replace('\\', "/");
        // The primitive contains deliberately partial construction tests.
        // The command form forwards its own caller's purpose text, so the
        // audit belongs at those callers. Extracted test modules are also
        // non-production sources and may intentionally exercise incomplete
        // builder chains.
        if relative.ends_with("/tests.rs")
            || relative.contains("/tests/")
            || matches!(
                relative.as_str(),
                "rspice-ui-kit/src/widgets/dialog.rs" | "rspice-ui-kit/src/widgets/command_form.rs"
            )
        {
            continue;
        }

        read.insert(relative.split('/').next().unwrap_or_default().to_owned());
        let source = std::fs::read_to_string(&path).expect("read Rust source");
        let production_source = source.split("\n#[cfg(test)]").next().unwrap_or(&source);
        for (offset, _) in production_source.match_indices("Dialog::new(") {
            if offset > 0
                && (production_source.as_bytes()[offset - 1].is_ascii_alphanumeric()
                    || production_source.as_bytes()[offset - 1] == b'_')
            {
                // Exclude other types whose names end in `Dialog`, such
                // as native `FileDialog` and persisted dialog-state data.
                continue;
            }
            audited += 1;
            let tail = &production_source[offset..];
            let chain_end = [
                tail.find(".show("),
                tail.find(".show_with_initial_body_focus("),
                tail.find(';'),
            ]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(tail.len());
            if !tail[..chain_end].contains(".description(") {
                let line = production_source[..offset]
                    .bytes()
                    .filter(|byte| *byte == b'\n')
                    .count()
                    + 1;
                missing.push(format!("{relative}:{line}"));
            }
        }
    }

    assert!(audited > 0, "source audit did not find a production dialog");
    assert_reads_every_app_crate(
        "the dialog-description audit",
        read.iter().map(String::as_str),
    );
    assert!(
        missing.is_empty(),
        "production dialogs must publish explicit purpose text:\n{}",
        missing.join("\n")
    );
}

/// The UI crates this file's accessibility scan covers, relative to the
/// workspace's `crates/` directory.
const PAINTED_UI_CRATES: &[&str] = &[
    "rspice-app/src",
    "rspice-app-state/src",
    "rspice-app-services/src",
    "rspice-app-runs/src",
    "rspice-app-workbench/src",
    "rspice-app-sheets/src",
    "rspice-app-settings/src",
    "rspice-app-results/src",
    "rspice-app-hardcopy/src",
    "rspice-app-cloud/src",
    "rspice-app-models/src",
    "rspice-app-simulate/src",
    "rspice-ui-kit/src",
    "rspice-results-ui/src",
    "rspice-schematic-editor/src",
];

/// Row helpers that paint words and publish nothing a screen reader can
/// reach, recorded by file and by the function they sit in.
///
/// This is a debt list, not a dispensation. Every entry is a row, a cell, a
/// header or a chip whose text renders correctly and is invisible to
/// assistive technology, and each one is fixed the same way: give the row a
/// response, call `widget_info` on it for the role and the bounds, then state
/// the pair through the AccessKit builder. Delete the entry in the same
/// commit as the fix — an entry whose site is clean fails this test, so the
/// list cannot rot into a record of problems that no longer exist.
///
/// It began at 62 entries because the class was 62 sites wide and fixing
/// them is a program rather than a change. It was measured, not estimated: the two
/// known-worst sites were repaired first — Check and save's rows, and
/// `models/manager/paint.rs::property`, which painted all three columns of
/// every property of every card in that workspace and published nothing at
/// all — and this is what was left.
const UNPUBLISHED_PAINTED_ROWS: &[&str] = &[
    "rspice-app-models/src/workbench/surfaces/model_editor.rs::dialog_subheading",
    "rspice-app-models/src/workbench/surfaces/model_editor.rs::release_no_candidate_row",
    "rspice-app-models/src/workbench/surfaces/model_editor.rs::section_header",
    "rspice-app-models/src/workbench/surfaces/model_editor.rs::specialist_header",
    "rspice-app-models/src/workbench/surfaces/model_editor.rs::table_empty",
    "rspice-app-models/src/workbench/surfaces/model_editor/schema_editors.rs::temperature_plot",
    "rspice-app-models/src/workbench/surfaces/model_editor/workflows.rs::workflow_purpose_note",
    "rspice-app-models/src/workbench/surfaces/models/manager.rs::catalog_footer_capped",
    "rspice-app-models/src/workbench/surfaces/models/manager/bins.rs::map_legend",
    "rspice-app-models/src/workbench/surfaces/models/manager/detail.rs::origin_property",
    "rspice-app-models/src/workbench/surfaces/models/manager/include.rs::include_closure_graph",
    "rspice-app-models/src/workbench/surfaces/models/manager/qualification_page.rs::closure_ledger",
    "rspice-app-models/src/workbench/surfaces/models/manager/symbols.rs::detail_table_header",
    "rspice-app-results/src/workbench/docks/inspector/results.rs::trace_colour_row",
    "rspice-app-simulate/src/workbench/surfaces/simulate.rs::analysis_group_header",
    "rspice-app-simulate/src/workbench/surfaces/simulate.rs::paint_control_row_label",
    "rspice-app-simulate/src/workbench/surfaces/simulate/analysis_form.rs::named_periodic_source_row",
    "rspice-app-simulate/src/workbench/surfaces/simulate/analysis_form.rs::property_row",
    "rspice-app-simulate/src/workbench/surfaces/simulate/analysis_form/envelope.rs::envelope_choice_row",
    "rspice-app-simulate/src/workbench/surfaces/simulate/catalog.rs::analysis_catalog_window",
    "rspice-app-simulate/src/workbench/surfaces/simulate/page_kit.rs::card_head_row",
    "rspice-app-simulate/src/workbench/surfaces/simulate/page_runset.rs::value_chip",
    "rspice-app-simulate/src/workbench/surfaces/simulate/page_solver.rs::ladder_stage",
    "rspice-app-simulate/src/workbench/surfaces/simulate/plan_manager/kit.rs::detail_row",
    "rspice-app-simulate/src/workbench/surfaces/simulate/plan_manager/kit.rs::section_head",
    "rspice-app-simulate/src/workbench/surfaces/simulate/plan_manager/kit.rs::table",
    "rspice-app-simulate/src/workbench/surfaces/simulate/workflows.rs::workflow_section_heading",
    "rspice-app-simulate/src/workbench/surfaces/verify/tuning.rs::tuning_waveform_preview",
    "rspice-app-workbench/src/workbench/docks/drawers.rs::mobile_navigation_dialog",
    "rspice-app-workbench/src/workbench/docks/drawers.rs::mobile_navigation_group",
    "rspice-app-workbench/src/workbench/tools/notification_center.rs::foot",
    "rspice-app/src/workbench/app/dialogs/design_management/widgets.rs::schematic_preview",
    "rspice-app/src/workbench/app/dialogs/design_review_comments.rs::review_message",
    "rspice-app/src/workbench/app/dialogs/help.rs::render_shortcuts_help_dialog",
    "rspice-app/src/workbench/design/navigator.rs::tabs",
    "rspice-app/src/workbench/design/surface/layout_editor.rs::show_canvas",
    "rspice-app/src/workbench/project/library_dialogs/cell_ops.rs::process_copy_cell_dialog",
    "rspice-app/src/workbench/surfaces/project/landing.rs::compact_recent_header",
    "rspice-app/src/workbench/surfaces/project/landing.rs::compact_section_header",
    "rspice-app/src/workbench/surfaces/project/overview.rs::status_rows",
    "rspice-app/src/workbench/surfaces/stimulus/pwl.rs::table_header",
    "rspice-results-ui/src/noise_contrib.rs::contributor_header",
    "rspice-results-ui/src/waves/expression_editor.rs::show",
    "rspice-schematic-editor/src/property_values.rs::render_value_editor",
    "rspice-ui-kit/src/panels.rs::card_with_body_margin",
    "rspice-ui-kit/src/widgets/canvas_entry.rs::derived",
    "rspice-ui-kit/src/widgets/canvas_entry.rs::paint_status",
    "rspice-ui-kit/src/widgets/command_form.rs::list_entry",
    "rspice-ui-kit/src/widgets/dialog.rs::mark",
    "rspice-ui-kit/src/widgets/form.rs::kv_row",
];

/// `source` with every `#[cfg(test)]` item blanked, line for line.
///
/// The crate's own `source_guard::without_test_items` does this for scans that
/// live inside the library. An integration test cannot reach it, so the rule
/// is restated here: cutting the file at the first `#[cfg(test)]` — which the
/// dialog-purpose audit above does — leaves everything after an early test
/// module unscanned, and that is not a small gap. The models workspace's
/// painter module declares a test module a third of the way down; cutting
/// there hid `property`, the single worst accessibility defect in the
/// product, from the very scan written to find it.
///
/// Removed lines become empty ones so the line a match sits on is still the
/// line in the file.
fn without_test_items(source: &str) -> String {
    const ATTRIBUTE: &str = "#[cfg(test)]";
    let lines = source.lines().collect::<Vec<_>>();
    let mut kept: Vec<&str> = Vec::with_capacity(lines.len());
    let mut index = 0;
    while index < lines.len() {
        let line = lines[index];
        let Some(rest) = line.trim_start().strip_prefix(ATTRIBUTE) else {
            kept.push(line);
            index += 1;
            continue;
        };
        let indent = line.len() - line.trim_start().len();
        kept.push("");
        index += 1;
        let mut item = rest.trim();
        let mut opened = false;
        loop {
            let trimmed = item.trim_end();
            if !opened && trimmed.matches('{').count() > trimmed.matches('}').count() {
                opened = true;
            }
            let closed = if opened {
                // The closer of a braced item is the only `}` at the
                // attribute's own indentation; its contents are indented
                // further. Reading the end that way rather than by counting
                // braces keeps a brace inside a string from moving it.
                trimmed.trim_start().starts_with('}')
                    && item.len() - item.trim_start().len() == indent
            } else {
                // Balanced braces on the first line finish the item here — a
                // one-line `fn`, or a `use` naming a brace list. So does a
                // statement, an out-of-line `mod`, or an attributed field.
                trimmed.contains('{') || trimmed.ends_with(';') || trimmed.ends_with(',')
            };
            if closed || index >= lines.len() {
                break;
            }
            item = lines[index];
            kept.push("");
            index += 1;
        }
    }
    kept.join("\n")
}

/// The function a line sits in, by the nearest `fn` declared above it.
///
/// A line number would rot on the next edit anywhere above it; a function
/// name survives one, which is what makes [`UNPUBLISHED_PAINTED_ROWS`] a list
/// a reader can check against the source.
fn enclosing_fn(lines: &[&str], index: usize) -> String {
    for line in lines[..=index].iter().rev() {
        let mut rest = line.trim_start();
        if let Some(after) = rest.strip_prefix("pub") {
            let after = after.strip_prefix('(').map_or(after, |inner| {
                inner.split_once(')').map_or(inner, |(_, tail)| tail)
            });
            if after.starts_with(' ') {
                rest = after.trim_start();
            }
        }
        for keyword in ["const ", "async ", "unsafe "] {
            rest = rest.strip_prefix(keyword).unwrap_or(rest).trim_start();
        }
        if let Some(name) = rest.strip_prefix("fn ") {
            let name: String = name
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() {
                return name;
            }
        }
    }
    "<file>".to_owned()
}

/// The response identifier an `accesskit_node_builder` call is keyed on.
fn builder_response(line: &str) -> Option<String> {
    let tail = line.split_once("accesskit_node_builder(")?.1.trim_start();
    let tail = tail.strip_prefix('&').unwrap_or(tail);
    let name: String = tail
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if name.is_empty() {
        return None;
    }
    // `let response_id = response.id` is the same response under another name.
    Some(name.strip_suffix("_id").unwrap_or(&name).to_owned())
}

/// Does `line` bind `name`, either alone or as the second half of an
/// `allocate_exact_size` tuple?
fn binds(line: &str, name: &str) -> bool {
    let squeezed = line.split_whitespace().collect::<Vec<_>>().join(" ");
    squeezed.contains(&format!(", {name}) ="))
        || squeezed.contains(&format!("let {name} ="))
        || squeezed.contains(&format!("let mut {name} ="))
}

fn allocates(window: &str) -> bool {
    [
        "allocate_exact_size",
        "allocate_response",
        "allocate_at_least",
    ]
    .iter()
    .any(|call| window.contains(call))
}

/// Does `window` paint text through the raw painter?
fn paints_text(window: &str) -> bool {
    window.match_indices("painter()").any(|(at, _)| {
        let reach = window.len().min(at + 200);
        window[at..reach].contains(".text(")
    })
}

/// Everything a surface paints must also reach assistive technology.
///
/// Two shapes of the same bug, and the second is the first with its visible
/// half missing:
///
/// 1. **A node created but never given a role or bounds.**
///    `Context::accesskit_node_builder` *creates* the node, so a helper that
///    calls only the builder looks like it published something. Only
///    `Response::widget_info` fills in the role and the geometry, and a node
///    with neither is `Role::Unknown` at no position: unreachable by any
///    assistive technology, and dropped outright by
///    `tools/ci/browser_workbench.py::controls()`, which appends a node only
///    `if bounds:`. This is how the whole body of the Check and save dialog —
///    every fact row and the entire change preview — rendered correctly and
///    was read as empty.
///
/// 2. **No node at all.** A helper that allocates its own row, throws the
///    response away into `_`, and paints the row's words through
///    `ui.painter()`. There is no builder call to find here, because nothing
///    was ever created; the only handle on the row is gone by the time the
///    text is drawn.
///
/// Both are recorded by function in [`UNPUBLISHED_PAINTED_ROWS`], and the list
/// is checked for equality: a new one fails, and a fixed one that is still
/// listed fails too.
///
/// What this test cannot see: whether a published node says anything useful.
/// Assert *bounds* when pinning a specific surface, never only the text — the
/// text was always painted, which is exactly why this class went unnoticed.
#[test]
fn a_painted_row_reaches_the_accessibility_tree() {
    let crates_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut sources = Vec::new();
    for crate_src in PAINTED_UI_CRATES {
        sources.extend(rust_sources(&crates_root.join(crate_src)));
    }

    let mut scanned = 0_usize;
    let mut found: BTreeMap<String, String> = BTreeMap::new();
    for path in sources {
        let relative = path
            .strip_prefix(crates_root)
            .expect("source beneath the crates directory")
            .to_string_lossy()
            .replace('\\', "/");
        if relative.ends_with("/tests.rs") || relative.contains("/tests/") {
            continue;
        }
        scanned += 1;
        let source = fs::read_to_string(&path).expect("read Rust source");
        let shipped = without_test_items(&source);
        let lines = shipped.lines().collect::<Vec<_>>();

        for (index, line) in lines.iter().enumerate() {
            // --- a node created with no role and no bounds ---
            if let Some(name) = builder_response(line) {
                let mut published = false;
                let mut raw = false;
                for back in (0..=index).rev() {
                    if index - back > 120 {
                        break;
                    }
                    if lines[back].contains(".widget_info(") {
                        published = true;
                    }
                    if binds(lines[back], &name) {
                        let reach = lines.len().min(back + 6);
                        raw = allocates(&lines[back..reach].join("\n"));
                        break;
                    }
                }
                // Setting the bounds by hand inside the builder is the one
                // other way a node gets geometry, so it counts as published.
                let reach = lines.len().min(index + 30);
                if lines[index..reach].join("\n").contains("set_bounds") {
                    published = true;
                }
                if raw && !published {
                    found
                        .entry(format!("{relative}::{}", enclosing_fn(&lines, index)))
                        .or_insert_with(|| {
                            format!("{relative}:{}: builder with no widget_info", index + 1)
                        });
                }
            }

            // --- a row allocated, discarded, and painted anyway ---
            let squeezed = line.split_whitespace().collect::<Vec<_>>().join(" ");
            if squeezed.contains(", _) = ui.allocate_exact_size") {
                let reach = lines.len().min(index + 60);
                let mut ahead = lines[index + 1..reach].join("\n");
                // Stay inside this row's own statements.
                if let Some(cut) = ahead.find("allocate_") {
                    ahead.truncate(cut);
                }
                if paints_text(&ahead)
                    && !ahead.contains("widget_info")
                    && !ahead.contains("accesskit_node_builder")
                {
                    found
                        .entry(format!("{relative}::{}", enclosing_fn(&lines, index)))
                        .or_insert_with(|| {
                            format!("{relative}:{}: painted row, no node", index + 1)
                        });
                }
            }
        }
    }

    assert!(scanned > 100, "the accessibility scan read {scanned} files");

    let listed: std::collections::BTreeSet<&str> =
        UNPUBLISHED_PAINTED_ROWS.iter().copied().collect();
    assert_eq!(
        listed.len(),
        UNPUBLISHED_PAINTED_ROWS.len(),
        "UNPUBLISHED_PAINTED_ROWS contains a duplicate entry"
    );

    let mut failures = String::new();
    let fresh = found
        .iter()
        .filter(|(key, _)| !listed.contains(key.as_str()))
        .map(|(key, site)| format!("  {key}\n    {site}"))
        .collect::<Vec<_>>();
    if !fresh.is_empty() {
        failures.push_str(&format!(
            "These helpers paint words no screen reader can reach:\n{}\n\
             Give the row a response, call `widget_info` on it for the role and the\n\
             bounds, then state the pair through the AccessKit builder — in that\n\
             order, because `widget_info` would otherwise overwrite the value with\n\
             the label.\n",
            fresh.join("\n")
        ));
    }
    let repaired = UNPUBLISHED_PAINTED_ROWS
        .iter()
        .filter(|key| !found.contains_key(**key))
        .copied()
        .collect::<Vec<_>>();
    if !repaired.is_empty() {
        failures.push_str(&format!(
            "\nUNPUBLISHED_PAINTED_ROWS entries that are clean, renamed or gone:\n  {}\n\
             Delete them, so the list stays a record of what is still broken.\n",
            repaired.join("\n  ")
        ));
    }
    assert!(failures.is_empty(), "{failures}");
}

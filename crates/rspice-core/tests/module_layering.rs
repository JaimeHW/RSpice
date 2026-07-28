//! Architectural layering ratchet for the `rspice-core` engine crate.
//!
//! `rspice-core` is one crate by design: the analyses, the device models, and
//! the sparse assembly they share are a single numerical program, and cutting
//! them into published crates would buy compiler-enforced layering at the cost
//! of a versioning surface nobody wants across an engine boundary. That is a
//! considered decision, but it costs the one thing a crate split gives for
//! free: the compiler no longer refuses an import that inverts the
//! architecture.
//!
//! This test is the substitute. It declares the intended layer order and fails
//! when a module reaches sideways or upward through `crate::`.
//!
//! # Why the order is total
//!
//! [`LAYERS`] assigns a distinct rank to every module that participates in a
//! real edge. Ties are how a cycle hides inside a layer: when `solver` and
//! `simd` shared a rank, `solver -> simd` was invisible, and so was
//! `library -> netlist`. Modules that genuinely never reference each other
//! (the layer-0 leaves) may share rank 0, because there is no edge for a tie
//! to conceal.
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
//! - An edge that reaches zero fails the build until its entry is deleted, so
//!   the allowlist cannot rot into a list of problems that no longer exist.
//!
//! Each entry carries the remediation phase that retires it. When a phase
//! lands, its entries disappear from this list rather than being renumbered.
//!
//! # What counts as a reference
//!
//! An occurrence of `crate::<module>` in a `.rs` file under `src/`, after line
//! comments are stripped. Doc comments describing a relationship are not a
//! dependency, so they are excluded. `use`, type positions, and paths all
//! count equally: the metric tracks coupling, not import style.
//!
//! `device/veriloga_generated/` is excluded. It is machine output — 172 files
//! and 49 MB — and generated code is not subject to architectural review; the
//! generator is. Scanning it would also cost every run of this test.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Intended layer order, lowest first.
///
/// A module may reference any module of strictly lower rank. Referencing the
/// same rank or a higher one is a violation.
///
/// The ordering encodes the target architecture, not today's reality —
/// [`ALLOWED_VIOLATIONS`] carries the gap between the two. Several ranks name
/// a module that does not exist yet; the comments say what will occupy them.
const LAYERS: &[(&str, u32)] = &[
    // Framework-independent leaves. These reference nothing above them, so
    // they may share a rank without concealing an edge.
    //
    // `constants` is also the intended home of the unit conversions currently
    // parked in `analysis::temperature` (Phase 2), and `resource` the home of
    // the include-depth limit currently owned by `netlist`.
    ("constants", 0),
    ("time_compat", 0),
    ("resource", 0),
    ("abort_signal", 0),
    ("builtin_lib", 0),
    // SPICE naming rules. Everything above depends on this, so it depends on
    // nothing.
    ("naming", 0),
    // Structured convergence-quality reporting. Wired into the drivers in
    // Phase 5b; it depends only on `Value` and stays a leaf.
    ("diagnostics", 0),
    // Numerics. `simd` is a kernel library beneath the solvers; `solver`
    // becomes `numerics::la` in Phase 5, absorbing `engine::convergence`, and
    // gains a `numerics::integration` sibling holding the companion
    // coefficients and LTE machinery that `analysis::core::transient` owns
    // today.
    ("simd", 1),
    ("solver", 2),
    // Expression evaluation. Two deliberate subsystems: the bytecode VM here
    // and the complex-valued `.PARAM` evaluator inside `netlist`, mirroring
    // ngspice's inpptree/numparam split.
    ("expr", 3),
    // Deck text to AST. Phase 7 moves its circuit transforms (flattener,
    // add_resistors, remove_unused, topology) up into `elab`, leaving parsing.
    ("netlist", 4),
    // `.lib` model-library and Verilog-A pack discovery. Above `netlist`
    // because resolving a library produces deck content.
    ("library", 5),
    // Device model evaluation, plus the Verilog-A and FFI extension points.
    ("device", 6),
    // The XSPICE code-model subsystem: a device extension with its own event
    // queue, so it sits just above `device`.
    ("xspice", 7),
    // Struct-of-arrays circuit storage and stamping. Phase 8 consolidates the
    // four stamping surfaces into `circuit::assembly`.
    ("circuit", 8),
    // Analysis algorithms and result types. Phase 6 merges the `engine::X` /
    // `analysis::X` twins into one module per analysis.
    ("analysis", 9),
    // Foreign-format IO. `compat` is a bag today: a 924-line LTspice RAW
    // reader plus a 10-line ground-name predicate that eight modules call.
    // Phase 2 splits the predicate down to a layer-0 `naming` leaf and leaves
    // this rank to the reader, joined by `analysis::output`.
    ("compat", 10),
    // The facade: configuration resolution, dispatch, health, abort plumbing.
    // Phase 3 moves `SimulationConfig` and friends out to their own low rank;
    // Phase 7 moves `engine::builder` out to `elab`.
    ("engine", 11),
];

/// Layer-order violations present in the tree today, with exact counts.
///
/// Sorted by the remediation phase that retires them. Do not add entries to
/// unblock new code — a new violation means the code is in the wrong module.
const ALLOWED_VIOLATIONS: &[(&str, &str, usize)] = &[
    // ---------------------------------------------------------------------
    // Phase 2 — foundation leaves.
    //
    // The ground-name predicate moved to the layer-0 `naming` leaf, which
    // retired `device`, `netlist`, `circuit` and `solver` reaching into
    // `compat`.
    //
    // `analysis -> compat` is what remains, and it is a different edge:
    // `waveform_stream` reading an LTspice RAW file. It is retired by moving
    // `analysis::output` up beside the reader, not by moving anything down.
    ("analysis", "compat", 2),
    // `expr -> analysis` and `resource -> netlist` are single references each:
    // a temperature conversion and the default include depth, both of which
    // belong in layer-0 leaves. The include-depth constant is defined in
    // `netlist/include.rs`.
    ("expr", "analysis", 3),
    ("resource", "netlist", 1),
    // ---------------------------------------------------------------------
    // Phase 3 — extract `config`.
    //
    // All three edges are `SimulationConfig` and its companions, which live in
    // `engine` only because that is where the orchestrator is. Nothing about a
    // configuration struct requires the module that consumes it; moving them
    // below everything that reads them makes all three downward.
    ("circuit", "engine", 21),
    ("solver", "engine", 5),
    ("device", "engine", 2),
    // `expr::parser` naming `netlist::ExpressionDialect` to build a test
    // context. The dialect enums are configuration, and travel to `config`
    // with `SpiceDialect`.
    ("expr", "netlist", 1),
    // ---------------------------------------------------------------------
    // Phase 4 — `.MEAS` statement types are AST, not results.
    //
    // `MeasureStatement`, `MeasureType`, `EdgeType`, `WhenCondition`,
    // `TrigSpec` and the rest are parsed deck syntax. The parser reaches nine
    // layers up for them. Moving the types into `netlist::ast` — leaving the
    // evaluator in `analysis` — is the whole fix.
    ("netlist", "analysis", 80),
    // ---------------------------------------------------------------------
    // Phase 5 — numerics.
    //
    // `CompanionCoefficients` is an integration-method primitive that lives in
    // `analysis::core::transient`, so every module that stamps a companion
    // model reaches up for it.
    ("circuit", "analysis", 11),
    ("device", "analysis", 11),
    ("xspice", "analysis", 2),
    // ---------------------------------------------------------------------
    // Phase 6 — one module per analysis.
    //
    // `analysis` reaching for `TransientResult` and `TransientStoreTrace`,
    // which are driver-side result types today. The merge puts each analysis's
    // result type in the same module as its driver.
    ("analysis", "engine", 2),
    // ---------------------------------------------------------------------
    // Phase 7 — elaboration.
    //
    // The parser asking the device layer which built-in Verilog-A models exist
    // (`flattener`), and reaching into the XSPICE subsystem while parsing `A`
    // cards. Both are elaboration concerns that will sit above both modules.
    ("netlist", "xspice", 5),
    ("netlist", "device", 1),
    // ---------------------------------------------------------------------
    // Phase 8 — device layer.
    //
    // Devices reaching into circuit storage. Retired by the `circuit::assembly`
    // consolidation, which gives devices a stamp target instead of the store.
    ("device", "circuit", 4),
];

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every `.rs` file under `src/`, excluding generated Verilog-A.
fn rust_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(dir) = pending.pop() {
        let entries =
            fs::read_dir(&dir).unwrap_or_else(|error| panic!("read {}: {error}", dir.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "veriloga_generated") {
                    continue;
                }
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
/// `lib.rs` is the crate root and is allowed to see everything, so it belongs
/// to no module. Every *other* file sitting directly in `src/` is a module in
/// its own right and is layered like a directory: both the `foo.rs` half of
/// the `foo.rs`-beside-`foo/` convention and standalone modules such as
/// `resource` that have no directory at all.
fn owning_module(root: &Path, file: &Path) -> Option<String> {
    let relative = file.strip_prefix(root).ok()?;
    let mut components = relative.components();
    let first = components.next()?.as_os_str().to_str()?.to_owned();
    if components.next().is_none() {
        let stem = Path::new(&first).file_stem()?.to_str()?;
        if stem == "lib" {
            return None;
        }
        return Some(stem.to_owned());
    }
    Some(first)
}

/// Strip line comments so prose describing a relationship is not counted as
/// one. A `//` inside a string literal costs at most an undercount, which can
/// only make this test more permissive, never falsely red.
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
        // Reject a longer identifier: `crate::device` must not match
        // `crate::device_foo`.
        let boundary = after
            .chars()
            .next()
            .is_none_or(|next| !next.is_alphanumeric() && next != '_');
        if boundary {
            count += 1;
        }
        rest = after;
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
            if stem == "lib" {
                continue;
            }
            stem.to_owned()
        } else {
            continue;
        };
        if !LAYERS.iter().any(|(declared, _)| *declared == name) && !undeclared.contains(&name) {
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

    // Entries left over describe edges that no longer exist. Deleting them is
    // the point of the exercise, so the allowlist must be kept honest.
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
             value it uses as a parameter instead of reaching for it.\n",
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

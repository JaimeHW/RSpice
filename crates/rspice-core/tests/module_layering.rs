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
//! Nothing under `src/` is excluded. `device/veriloga_builtins/` used to be,
//! first because it held the generated corpus — 172 files, 49 MB, and generated
//! code is the generator's business rather than this ratchet's — and then,
//! after the shard moved those files into `rspice-veriloga-models`, because the
//! one hand-written file left behind named `crate::engine::SimulationError`
//! from rank 8. That was a real upward edge wearing an exemption's clothes.
//! `instantiate_builtin` returns a device-level error now and `engine` converts,
//! so the directory is measured like every other.

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
    // `constants` holds the physical constants and the Celsius/Kelvin
    // conversions that are arithmetic on them. `resource` holds the limits
    // that bound what a deck may ask for, including the include depth.
    ("constants", 0),
    ("time_compat", 0),
    ("resource", 0),
    ("abort_signal", 0),
    ("builtin_lib", 0),
    // Names of the XSPICE code models compiled in. A catalogue of names is
    // data: the parser classifies against it while reading an `A` card and
    // `xspice` registers against it, so both read down instead of the parser
    // reaching eight ranks up.
    ("codemodels", 0),
    // Physical-unit vocabulary is data carried by both engine results and
    // execution schemas, so neither producer owns it.
    ("signal_unit", 0),
    // SPICE naming rules. Everything above depends on this, so it depends on
    // nothing.
    ("naming", 0),
    // Structured convergence-quality reporting. Wired into the drivers in
    // Phase 5b; it depends only on `Value` and stays a leaf.
    ("diagnostics", 0),
    // The labels an operating-point report may name a family, a region or a
    // quantity with. A vocabulary of names is data, on the same argument as
    // `codemodels`: the device families that emit a label and the circuit
    // store that assembles the report both read down into it instead of the
    // families reaching ten ranks up. The reader's half, which also answers
    // for the compiled Verilog-A catalog, stays in `circuit` for that reason.
    ("op_label", 0),
    // The names an event-only net is refused and published under: which
    // event domain owns it, which result class is being addressed, and the
    // one sentence every surface says. Data, like `op_label`: a solved
    // point in `solver` and the analog-touch classification in `circuit`
    // both carry the kind, and neither should reach up into `analysis` for
    // two field-less enums. `analysis::transient` re-exports all three.
    ("event_net", 0),
    // Sparse charge observations are data shared by results and live hooks.
    ("transient_observation", 0),
    // SPICE engineering-suffix semantics. Both expression parsers and the
    // deck lexer consume this leaf so numeric text has one semantic owner.
    ("spice_number", 0),
    // Analysis-instance and run-coordinate identity. A vocabulary of names,
    // like `codemodels` and `op_label`: the planner in `execution` mints them
    // and the engine's typed errors carry them, so a leaf is what keeps the
    // error taxonomy from reaching up into the orchestration layer.
    ("identity", 0),
    // Which run a log record belongs to. It names nothing in the crate: the
    // analyses that hand work to another thread read down into it, and so
    // does `notice`, whose collector rides in the scope as an opaque value
    // that `notice` alone sets and downcasts.
    ("log_scope", 0),
    // SPICE RAW waveform files, read and written. A leaf by construction:
    // `raw_export` names nothing in the crate at all and the reader needs only
    // `resource` for its read limits, so a format module never has to move
    // when a result type changes. It shares rank 1 with `numerics` because
    // neither references the other.
    ("io", 1),
    // Numerics shared by every analysis: how a derivative is discretized, how
    // large a step may be, where a step may not land, and whether the step
    // taken was accurate enough to keep. Below the circuit store, the device
    // models and XSPICE, all three of which stamp with the coefficients.
    //
    // `solver` stays its own rank rather than becoming `numerics::la`: it
    // solves against an assembled matrix, so it sits above the layer that
    // defines one, and the rename would have cost five crates a path for no
    // gain.
    ("numerics", 1),
    // Typed per-analysis scalars. Data, like the `signal_unit` vocabulary
    // whose units and wire spelling they carry: the analysis results that
    // determine one, the control host that prints one and the result document
    // that publishes one all read down into it instead of the control host
    // reaching up into `execution`.
    ("result_scalar", 1),
    // SIMD acceleration uses the scalar numerical definitions for exceptional
    // values, unaligned tails and convergence policy. Numerics stays independent
    // of the accelerator so native and browser fallbacks have the same meaning.
    ("simd", 2),
    // What a run raised and went on to complete with: the parameters the
    // device models clamp, the convergence aids that solved, the timestep
    // floor the transient stepper was held at. The device models record into
    // it, so it sits below them, and it owns the `ContinuationAid` vocabulary
    // its convergence notice carries rather than reaching up into `analysis`
    // for it; `analysis::error` re-exports the aid for the failure that names
    // it. It reads `log_scope` and `numerics`, and shares rank 2 with `simd`
    // because neither references the other.
    ("notice", 2),
    // Simulation configuration. Below the solver, the device models and the
    // circuit store, all of which read a tolerance or a dialect flag and used
    // to reach up into `engine` to do it.
    ("config", 3),
    ("solver", 4),
    // Expression evaluation. Two deliberate subsystems: the bytecode VM here
    // and the complex-valued `.PARAM` evaluator inside `netlist`, mirroring
    // ngspice's inpptree/numparam split.
    ("expr", 5),
    // Deck text to AST. Phase 7 moves its circuit transforms (flattener,
    // add_resistors, remove_unused, topology) up into `elab`, leaving parsing.
    ("netlist", 6),
    // Shared interface between the control-session state machine and its
    // electrical host. It names ParamContext, so it sits immediately above
    // netlist and below both participants.
    ("control_protocol", 7),
    // `.lib` model-library and Verilog-A pack discovery. Above `netlist`
    // because resolving a library produces deck content.
    ("library", 7),
    // Device model evaluation, including the Verilog-A device integration.
    ("device", 8),
    // The XSPICE code-model subsystem: a device extension with its own event
    // queue, so it sits just above `device`.
    ("xspice", 9),
    // Struct-of-arrays circuit storage and stamping. Phase 8 consolidates the
    // four stamping surfaces into `circuit::assembly`.
    ("circuit", 10),
    // Analysis algorithms and result types. Phase 6 merges the `engine::X` /
    // `analysis::X` twins into one module per analysis.
    ("analysis", 11),
    // The facade: configuration resolution, dispatch, health, abort plumbing.
    // Phase 3 moves `SimulationConfig` and friends out to their own low rank;
    // Phase 7 moves `engine::builder` out to `elab`.
    ("engine", 12),
    // Target-neutral deck planning and execution orchestration. It consumes
    // parsed netlists and, as the shared executor is completed, the engine
    // facade; neither lower layer may depend on frontend run coordinates.
    ("execution", 13),
];

/// Upward dependencies, recorded with the exact reference count each stands at.
///
/// This list was empty until 2026-09-11 and is meant to be empty again. Three
/// edges landed in the September parser and sensitivity work without it, and
/// each entry below says what retires it.
///
/// - `netlist -> engine`, 5. Unit tests in `netlist.rs`, `netlist/flattener.rs`
///   and `netlist/parser/values.rs` that check a parse by running
///   `Engine::default().run_dc_op` on the netlist they just built. The physical
///   check is the point of them — a positional `AREA`, a scoped subcircuit
///   binding and an instance-argument expression are all things a parser can
///   get wrong without saying so — but reaching six ranks up to make it is
///   still the inversion this test measures. They retire by moving the
///   physical half into an integration test under `crates/rspice-core/tests/`,
///   which is an external crate and therefore not an edge at all; the
///   parser-internal half has to stay, because it drives
///   `super::parse_numeric_field_value` and its siblings directly.
/// - `netlist -> circuit`, 1. One unit test in `netlist/flattener.rs` that
///   reads a materialized source waveform back through
///   `circuit::VoltageSources::evaluate_source_spec_at_time_with_dialect`.
///   Retires the same way.
/// - `netlist -> device`, 1. Production, and the only one of the three that is:
///   `netlist/expr/scalar_direction.rs` takes `BehavioralEnvironment`,
///   `compiled_expression_node_direction` and `eval_binary_with_derivative`
///   from `device::behavioral`, which is the one implementation of the
///   derivative arithmetic it needs. Restating that arithmetic inside `netlist`
///   is what the adapter was written to avoid, so the edge retires by moving
///   the shared kernel below both callers — into `expr`, whose types its
///   signatures already name — rather than by duplicating it.
const ALLOWED_VIOLATIONS: &[(&str, &str, usize)] = &[
    ("netlist", "circuit", 1),
    ("netlist", "device", 1),
    ("netlist", "engine", 5),
];

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Every `.rs` file under `src/`.
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

/// Rewrite `crate::{a, b::c}` into `crate::a crate::b::c`.
///
/// The counter below matches the literal prefix `crate::<module>`, so a
/// grouped import hides every member but the first behind a brace. That is
/// not a rare spelling: `use crate::{Value, circuit::NodeId};` was how
/// thirty-four of the thirty-eight device references to `NodeId` stayed
/// invisible while the edge showed a count of four.
fn expand_grouped_paths(source: &str) -> String {
    const OPEN: &str = "crate::{";
    let mut out = String::with_capacity(source.len());
    let mut rest = source;
    while let Some(index) = rest.find(OPEN) {
        out.push_str(&rest[..index]);
        let after = &rest[index + OPEN.len()..];
        let mut depth = 1usize;
        let mut end = None;
        for (offset, ch) in after.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(offset);
                        break;
                    }
                }
                _ => {}
            }
        }
        // An unbalanced brace means this is not an import we can expand;
        // emit the opener verbatim so the literal match still applies.
        let Some(end) = end else {
            out.push_str(OPEN);
            rest = after;
            continue;
        };
        let (group, remainder) = after.split_at(end);
        let mut depth = 0usize;
        let mut member = String::new();
        let flush = |member: &mut String, out: &mut String| {
            let trimmed = member.trim();
            if !trimmed.is_empty() {
                out.push_str("crate::");
                out.push_str(trimmed);
                out.push(' ');
            }
            member.clear();
        };
        for ch in group.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    member.push(ch);
                }
                '}' => {
                    depth -= 1;
                    member.push(ch);
                }
                ',' if depth == 0 => flush(&mut member, &mut out),
                _ => member.push(ch),
            }
        }
        flush(&mut member, &mut out);
        rest = &remainder[1..];
    }
    out.push_str(rest);
    out
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

/// A name a `use` declaration brings into scope, and the path it stands for,
/// segment by segment as written.
type UseBinding = (String, Vec<String>);

/// The names a file's own `use` declarations bind, comments already stripped.
///
/// Only a declaration that opens a line at column zero is read. Rustfmt puts
/// every item of a file's own namespace there, so this takes the module's
/// imports and re-exports and leaves out those scoped to a function body or
/// an inline `mod`: `lib.rs`'s `pub mod error { pub use … }` binds
/// `crate::error::CircuitError`, not `crate::CircuitError`. A glob binds no
/// name this test can follow, and `as _` binds none at all.
fn use_bindings(code: &str) -> Vec<UseBinding> {
    let mut bindings = Vec::new();
    let mut lines = code.lines();
    while let Some(line) = lines.next() {
        let Some(start) = use_tree_start(line) else {
            continue;
        };
        let mut tree = start.to_owned();
        while !tree.contains(';') {
            let Some(next) = lines.next() else { break };
            tree.push(' ');
            tree.push_str(next);
        }
        let tree = tree.split(';').next().unwrap_or_default();
        let tree = tree.split_whitespace().collect::<Vec<_>>().join(" ");
        push_use_tree(&tree, &[], &mut bindings);
    }
    bindings
}

/// What follows `use` on a line that opens a `use` declaration at column
/// zero, whatever its visibility.
fn use_tree_start(line: &str) -> Option<&str> {
    let declaration = match line.strip_prefix("pub") {
        Some(visibility) => {
            let visibility = match visibility.strip_prefix('(') {
                Some(scope) => &scope[scope.find(')')? + 1..],
                None => visibility,
            };
            visibility.strip_prefix(' ')?
        }
        None => line,
    };
    declaration.strip_prefix("use ")
}

/// Flatten one use tree, written under `prefix`, into the names it binds.
fn push_use_tree(tree: &str, prefix: &[String], bindings: &mut Vec<UseBinding>) {
    let tree = tree.trim();
    // What follows a group's trailing comma.
    if tree.is_empty() {
        return;
    }
    if let Some(open) = tree.find('{') {
        let path = [prefix, &path_segments(&tree[..open])].concat();
        let group = &tree[open + 1..];
        let group = group.strip_suffix('}').unwrap_or(group);
        let mut depth = 0usize;
        let mut start = 0;
        for (index, ch) in group.char_indices() {
            match ch {
                '{' => depth += 1,
                '}' => depth = depth.saturating_sub(1),
                ',' if depth == 0 => {
                    push_use_tree(&group[start..index], &path, bindings);
                    start = index + 1;
                }
                _ => {}
            }
        }
        push_use_tree(&group[start..], &path, bindings);
        return;
    }
    let (written, alias) = match tree.split_once(" as ") {
        Some((written, alias)) => (written, Some(alias.trim())),
        None => (tree, None),
    };
    let mut path = [prefix, &path_segments(written)].concat();
    if path.last().is_some_and(|last| last == "self") {
        path.pop();
    }
    let Some(last) = path.last() else { return };
    let name = alias.unwrap_or(last);
    if last == "*" || name == "_" {
        return;
    }
    bindings.push((name.to_owned(), path));
}

fn path_segments(path: &str) -> Vec<String> {
    path.split("::")
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(str::to_owned)
        .collect()
}

/// The source of the module at `path` from the crate root, if it has a file
/// of its own: `lib.rs` for the root itself, then `a/b.rs` or `a/b/mod.rs`.
fn module_source(root: &Path, path: &[String]) -> Option<String> {
    if path.is_empty() {
        return fs::read_to_string(root.join("lib.rs")).ok();
    }
    let file: PathBuf = path.iter().collect();
    fs::read_to_string(root.join(&file).with_extension("rs"))
        .or_else(|_| fs::read_to_string(root.join(file).join("mod.rs")))
        .ok()
}

/// Reads a module's source by its path from the crate root, so the resolver
/// below can be pointed at a tree written inside a test.
type ModuleSource<'a> = &'a dyn Fn(&[String]) -> Option<String>;

/// `path`, as a `use` declaration in the module at `module` writes it, from
/// the crate root; `None` when it leaves the crate.
///
/// A leading segment that is neither `crate`, `self` nor `super` is a
/// submodule when the module has a file by that name, a name another of the
/// file's declarations binds (`engine` binds `error` to
/// `crate::analysis::error` and re-exports through it), or else another crate.
fn absolute_path(
    source: ModuleSource<'_>,
    module: &[String],
    bindings: &[UseBinding],
    path: &[String],
) -> Option<Vec<String>> {
    let (first, rest) = path.split_first()?;
    match first.as_str() {
        "crate" => Some(rest.to_vec()),
        "self" => Some([module, rest].concat()),
        "super" => {
            let parent = &module[..module.len().checked_sub(1)?];
            if rest.first().is_some_and(|next| next == "super") {
                absolute_path(source, parent, &[], rest)
            } else {
                Some([parent, rest].concat())
            }
        }
        _ => {
            let child = [module, std::slice::from_ref(first)].concat();
            if source(&child).is_some() {
                return Some([child.as_slice(), rest].concat());
            }
            let (_, aliased) = bindings
                .iter()
                .find(|(bound, aliased)| bound == first && aliased.as_slice() != path)?;
            let aliased = absolute_path(source, module, bindings, aliased)?;
            Some([aliased.as_slice(), rest].concat())
        }
    }
}

/// The top-level module that defines what `name` stands for in the module at
/// `module`, following each `use` that passes it on, or `None` when that is
/// nothing of this crate's: another crate's item, or one the root defines.
fn defining_module(source: ModuleSource<'_>, module: &[String], name: &str) -> Option<String> {
    let mut module = module.to_vec();
    let mut name = name.to_owned();
    // A re-export chain this long would be a cycle, which rustc refuses.
    for _ in 0..16 {
        let Some(code) = source(&module) else {
            return module.first().cloned();
        };
        let bindings = use_bindings(&strip_line_comments(&code));
        let Some((_, path)) = bindings.iter().find(|(bound, _)| *bound == name) else {
            return module.first().cloned();
        };
        let mut target = absolute_path(source, &module, &bindings, path)?;
        name = target.pop()?;
        module = target;
    }
    panic!(
        "`{name}` is re-exported in a cycle ending at `{}`",
        module.join("::")
    );
}

/// Every name the crate root's `use` declarations bring into scope, with the
/// top-level module that defines it. Names the root defines or takes from
/// another crate are left out: a path through them is no edge.
fn crate_root_names(source: ModuleSource<'_>) -> BTreeMap<String, String> {
    let lib = strip_line_comments(&source(&[]).expect("the crate root has a source"));
    use_bindings(&lib)
        .into_iter()
        .filter_map(|(name, _)| {
            let module = defining_module(source, &[], &name)?;
            Some((name, module))
        })
        .collect()
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
        let code = expand_grouped_paths(&strip_line_comments(&source));
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

fn binding(name: &str, path: &str) -> UseBinding {
    (name.to_owned(), path_segments(path))
}

#[test]
fn a_use_declaration_binds_each_name_it_brings_into_the_file() {
    let code = strip_line_comments(
        "use std::sync::Arc;\n\
         pub use engine::{\n    \
             Engine as Facade, SimulationError,\n    \
             sub::{self, Inner},\n\
         };\n\
         pub(crate) use log_scope::*;\n\
         #[cfg(feature = \"simd\")]\n\
         pub use simd::Lanes;\n\
         use std::fmt::Write as _;\n\
         // pub use commented::Out;\n\
         pub mod error {\n    \
             pub use crate::circuit::CircuitError;\n\
         }\n\
         fn body() {\n    \
             use crate::device::Device;\n\
         }\n",
    );
    assert_eq!(
        use_bindings(&code),
        [
            binding("Arc", "std::sync::Arc"),
            binding("Facade", "engine::Engine"),
            binding("SimulationError", "engine::SimulationError"),
            binding("sub", "engine::sub"),
            binding("Inner", "engine::sub::Inner"),
            binding("Lanes", "simd::Lanes"),
        ]
    );
}

/// A root that re-exports through a submodule, through a name another
/// declaration binds, and through a second module's own re-export, beside a
/// name it defines and one it takes from another crate.
#[test]
fn a_crate_root_name_counts_toward_the_module_that_defines_it() {
    let tree = |path: &[String]| -> Option<String> {
        let source = match path.join("/").as_str() {
            "" => {
                "pub mod analysis;\npub mod engine;\npub mod notice;\n\
                 pub use engine::{ContinuationAid, Engine, SimulationError};\n\
                 pub use notice::Scalar;\n\
                 pub use num_complex::Complex64;\n\
                 pub type Value = f64;\n"
            }
            "engine" => {
                "mod core;\nuse crate::analysis::error;\n\
                 pub use core::Engine;\n\
                 pub use error::{ContinuationAid, SimulationError};\n"
            }
            "engine/core" => "pub struct Engine;\n",
            "analysis" => "pub mod error;\n",
            "analysis/error" => {
                "pub use crate::notice::ContinuationAid;\npub enum SimulationError {}\n"
            }
            "notice" => "pub use super::Value as Scalar;\npub enum ContinuationAid {}\n",
            _ => return None,
        };
        Some(source.to_owned())
    };
    let names = crate_root_names(&tree);
    let names: Vec<(&str, &str)> = names
        .iter()
        .map(|(name, module)| (name.as_str(), module.as_str()))
        .collect();
    assert_eq!(
        names,
        [
            ("ContinuationAid", "notice"),
            ("Engine", "engine"),
            ("SimulationError", "analysis"),
        ]
    );
}

/// The resolver reads the real root too: a reformatted `lib.rs` it can no
/// longer parse would leave the ratchet blind again without failing it.
#[test]
fn the_crate_roots_own_names_are_read() {
    let root = src_dir();
    let names = crate_root_names(&|path| module_source(&root, path));
    assert_eq!(names.get("Engine").map(String::as_str), Some("engine"));
    assert_eq!(
        names.get("CircuitData").map(String::as_str),
        Some("circuit")
    );
    assert!(!names.contains_key("Value"), "the root defines `Value`");
    assert!(
        !names.contains_key("Complex64"),
        "`Complex64` is another crate's"
    );
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

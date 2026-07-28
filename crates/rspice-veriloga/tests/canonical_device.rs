//! The canonical backend produces a device, and the device compiles.
//!
//! A generator test that only inspects the emitted text proves the text looks
//! right, which is not the property anyone needs. What matters is that the two
//! files fit together and fit the runtime: that `stamp.rs` names fields
//! `state.rs` actually declares, that the borrows inside a stamp are disjoint,
//! and that every stamper call matches a real signature with the right arity.
//! Only `rustc` can answer those, so this hands them to `rustc`.
//!
//! The runtime is stubbed rather than linked, because linking `rspice-core`
//! would mean writing the device into its source tree. The stub carries the
//! exact signatures the emitted code calls and nothing else — if a call shape
//! drifts, this fails at the call site with the same message the real build
//! would give.

use rspice_veriloga::rust_backend::{RustTranspileOptions, canonical};
use rspice_veriloga::VerilogACompiler;
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn a_generated_device_compiles_against_the_runtime_contract() {
    for (name, source) in fixtures() {
        let artifact = VerilogACompiler::default()
            .compile_canonical_ir(source)
            .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
        let device = canonical::generate_device(&artifact, &options())
            .unwrap_or_else(|error| panic!("{name}: generation: {error}"));

        let files: Vec<(&str, &str)> = device
            .files
            .iter()
            .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
            .collect();
        let state = find(&files, "state.rs", name);
        let stamp = find(&files, "stamp.rs", name);
        let noise = find(&files, "noise.rs", name);

        if let Err(report) = compile(name, state, stamp, noise) {
            panic!("{name}: the generated device does not compile:\n{report}");
        }
    }
}

/// The zeros are the point, so they are checked separately from compiling.
///
/// A two-terminal resistor reaches two unknowns and no more. The tier this
/// replaces writes `multiplicity * 0.0` for the rest of the row; here the
/// entries do not exist, so the literal never appears.
#[test]
fn a_stamp_writes_no_literal_zero_entries() {
    let source = r#"
module divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r1 = 100.0;
    parameter real r2 = 220.0;
    analog begin
        I(p, mid) <+ V(p, mid) / r1;
        I(mid, n) <+ V(mid, n) / r2;
    end
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files: Vec<(&str, &str)> = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect();
    let stamp = find(&files, "stamp.rs", "divider");

    for (index, line) in stamp.lines().enumerate() {
        assert!(
            !line.contains("multiplicity * 0.0"),
            "divider: stamp.rs line {} writes a literal zero: {line}",
            index + 1
        );
    }
    assert!(
        stamp.contains("stamp_current_sparse_local::<2, 0>"),
        "each branch of the divider reaches exactly two nodes; stamp.rs was:\n{stamp}"
    );
}

/// A model whose residual is a `ddt` gets a reactive stamp, and one without
/// gets an empty one rather than the conduction Jacobian by mistake.
#[test]
fn charge_storage_reaches_the_reactive_matrix_and_conduction_does_not() {
    let capacitor = r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#;
    let resistor = r#"
module res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

    let stamp = stamp_of(capacitor, "cap");
    assert!(
        stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a capacitor stores charge and must write the reactive matrix:\n{stamp}"
    );

    let stamp = stamp_of(resistor, "res");
    assert!(
        !stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a resistor stores no charge, so its reactive stamp writes nothing:\n{stamp}"
    );
    assert!(
        stamp.contains("pub fn stamp_reactive"),
        "the reactive entry point exists whether or not it has work:\n{stamp}"
    );
}

/// How far the canonical backend gets across the shipped models, and why it
/// stops where it does.
///
/// Numbers and reasons, not assertions. What it answers is the only question
/// that decides when this backend takes over from the tiers: which models it
/// carries end to end, and what each of the rest is waiting on.
#[test]
#[ignore = "generates every shipped model through the canonical backend; run with --ignored"]
fn the_whole_corpus_reports_what_the_canonical_backend_carries() {
    let root = model_root();
    let candidates =
        rspice_veriloga::rust_backend::discover_veriloga_sources(&root).expect("model tree");
    let mut carried = 0usize;
    let mut refused = 0usize;
    let mut bytes = 0usize;
    let mut stamp_bytes = 0usize;
    let mut noise_bytes = 0usize;
    let mut noise_fallbacks = 0usize;

    for candidate in &candidates {
        for module in &candidate.modules {
            let mut options = rspice_veriloga::CompilerOptions::default();
            options.include_paths.push(root.clone());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiled = match VerilogACompiler::new(options)
                .compile_file_canonical_ir_with_metadata(&candidate.path, Some(module))
            {
                Ok(compiled) => compiled,
                Err(error) => {
                    refused += 1;
                    eprintln!("{module:>24}  front end: {error}");
                    continue;
                }
            };
            match std::panic::catch_unwind(|| {
                canonical::generate_device(&compiled.artifact, &RustTranspileOptions::default())
            }) {
                Ok(Ok(device)) => {
                    carried += 1;
                    let total: usize = device
                        .files
                        .iter()
                        .map(|file| file.contents.len())
                        .sum::<usize>();
                    let sized = |name: &str| {
                        device
                            .files
                            .iter()
                            .find(|file| file.relative_path == name)
                            .map_or(0, |file| file.contents.len())
                    };
                    let (stamp, noise) = (sized("stamp.rs"), sized("noise.rs"));
                    // The replaced generator replays statements through a
                    // workspace array; the canonical one emits a body. Which
                    // wrote this file is the difference between a model whose
                    // noise the CFG carries and one that fell back to keep its
                    // device, and a byte count alone does not say which.
                    let fell_back = device
                        .files
                        .iter()
                        .find(|file| file.relative_path == "noise.rs")
                        .is_some_and(|file| file.contents.contains("let mut w = [0.0;"));
                    if fell_back {
                        noise_fallbacks += 1;
                    }
                    bytes += total;
                    stamp_bytes += stamp;
                    noise_bytes += noise;
                    eprintln!(
                        "{module:>24}  {total:>10} bytes  ({stamp} stamp, {noise} noise{})",
                        if fell_back { ", fell back" } else { "" }
                    );
                }
                Ok(Err(error)) => {
                    refused += 1;
                    eprintln!("{module:>24}  refused: {error}");
                }
                Err(payload) => {
                    refused += 1;
                    eprintln!("{module:>24}  panicked: {}", panic_reason(&payload));
                }
            }
        }
    }
    eprintln!(
        "\n{carried} carried in {bytes} bytes, {refused} not \
         ({stamp_bytes} stamp, {noise_bytes} noise, \
         {noise_fallbacks} of them from the replaced generator)"
    );
}

fn panic_reason(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    payload
        .downcast_ref::<&str>()
        .map_or_else(|| "no known payload".to_string(), |message| (*message).to_string())
}

fn model_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    assert!(root.exists(), "model tree missing: {}", root.display());
    root
}

fn stamp_of(source: &str, name: &str) -> String {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let device = canonical::generate_device(&artifact, &options())
        .unwrap_or_else(|error| panic!("{name}: generation: {error}"));
    device
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .map(|file| file.contents.clone())
        .unwrap_or_else(|| panic!("{name}: no stamp.rs"))
}

fn options() -> RustTranspileOptions {
    RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
    }
}

fn find<'a>(files: &[(&'a str, &'a str)], name: &str, model: &str) -> &'a str {
    files
        .iter()
        .find(|(path, _)| *path == name)
        .map(|(_, contents)| *contents)
        .unwrap_or_else(|| panic!("{model}: no {name} was generated"))
}

fn compile(name: &str, state: &str, stamp: &str, noise: &str) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let lib = root.join("lib.rs");
    std::fs::write(
        &lib,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("--crate-type=lib")
        .arg("-A")
        .arg("warnings")
        .arg("--out-dir")
        .arg(&root)
        .arg(&lib)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(String::from_utf8_lossy(&output.stderr).into_owned())
}

/// One generated file, as a module beside its siblings.
///
/// The three become sibling modules of one crate rather than one flat module,
/// which is the shape the real tree has: `stamp.rs` and `noise.rs` both reach
/// `Instance` through `super::state`, and both import from the runtime under
/// their own names. Flattening them makes those imports collide over nothing.
///
/// Only the inner attributes come out, because a `#![..]` is legal at the top of
/// a module but not after the module's first item, and the generated file writes
/// one that the surrounding stub already covers.
fn indent(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.starts_with("#!["))
        .collect::<Vec<_>>()
        .join("\n")
}

fn scratch() -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("canonical-device");
    std::fs::create_dir_all(&root).expect("scratch directory");
    root
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "resistor",
            r#"
module resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        ),
        (
            "capacitor",
            r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
        ),
        (
            "diode",
            r#"
module diode(a, c);
    inout a, c;
    electrical a, c;
    parameter real is = 1.0e-14;
    parameter real n = 1.0;
    analog begin
        I(a, c) <+ is * (exp(V(a, c) / (n * $vt)) - 1.0);
    end
endmodule
"#,
        ),
        // A guard, a temperature fold and a parameter prologue: the shape that
        // makes the invalidation split worth taking, so this exercises the
        // staged slots as well as the body.
        (
            "staged transistor",
            r#"
module staged(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    real geometry, vth, vov, ids;
    analog begin
        geometry = width * width * 1.0e12;
        if (geometry > 1.0e-3) begin
            geometry = geometry * 2.0;
        end
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            ids = geometry * vov * vov;
        end else begin
            ids = 0.0;
        end
        I(d, s) <+ 1.0e-6 * ids;
    end
endmodule
"#,
        ),
        // The same prologue, plus a contribution that reads no unknown at all.
        // Its residual is instance-class, so the stamp reads it from a slot —
        // and the Newton body has no staged operand of its own, which is what
        // makes the slot array's binding independent of what the body reads.
        (
            "staged transistor with a leakage floor",
            r#"
module floored(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    parameter real ileak = 1.0e-12;
    real geometry, vth, vov, ids;
    analog begin
        geometry = width * width * 1.0e12;
        if (geometry > 1.0e-3) begin
            geometry = geometry * 2.0;
        end
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            ids = geometry * vov * vov;
        end else begin
            ids = 0.0;
        end
        I(d, s) <+ 1.0e-6 * ids;
        I(d, s) <+ ileak * ileak;
    end
endmodule
"#,
        ),
        // A potential contribution, which stamps through a branch unknown
        // rather than a node pair.
        (
            "voltage source",
            r#"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real dc = 1.0;
    parameter real rs = 1.0e-3;
    analog V(p, n) <+ dc + rs * I(p, n);
endmodule
"#,
        ),
        // Noise, in the three shapes the descriptors distinguish. The table one
        // is here because its operands are the only magnitudes that reach the
        // visitor as a slice, and the guarded flicker because an inactive source
        // still has to be visited with the index its descriptor sits at.
        (
            "noisy resistor",
            r#"
module noisy_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0;
    analog begin
        I(p, n) <+ V(p, n) / r;
        I(p, n) <+ white_noise(4.0 * 1.380649e-23 * $temperature / r, "thermal");
    end
endmodule
"#,
        ),
        (
            "guarded flicker and table noise",
            r#"
module noisy_transistor(d, g, s);
    inout d, g, s;
    electrical d, g, s;
    parameter real kf = 1.0e-25;
    parameter real af = 1.2;
    parameter real beta = 1.0e-3;
    parameter real vth = 0.4;
    real ids;
    analog begin
        ids = 0.0;
        if (V(g, s) > vth) begin
            ids = beta * (V(g, s) - vth) * (V(g, s) - vth);
            I(d, s) <+ flicker_noise(kf * ids, af, "flicker");
        end
        I(d, s) <+ ids;
        I(d, s) <+ white_noise(2.0 * 1.602176634e-19 * ids, "shot");
        I(g, s) <+ noise_table({1.0, 1.0e-20, 1.0e6, 1.0e-22}, "gate");
    end
endmodule
"#,
        ),
    ]
}

/// Only what the emitted code calls, with the signatures it calls them by.
const RUNTIME_STUB: &str = r#"
#![allow(dead_code, non_snake_case, unused_parens, unused_variables, unused_mut, unused_imports)]

pub mod runtime {
    pub type Value = f64;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedDdtCoefficients {
        pub active: bool,
        pub derivative_scale: Value,
        pub previous_value_scale: Value,
        pub older_value_scale: Value,
        pub previous_derivative_scale: Value,
    }

    impl GeneratedDdtCoefficients {
        pub const fn inactive() -> Self {
            Self {
                active: false,
                derivative_scale: 0.0,
                previous_value_scale: 0.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            }
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct GeneratedVerilogAPersistentState {
        pub ddt_previous: Vec<Value>,
        pub ddt_older: Vec<Value>,
        pub ddt_derivative_previous: Vec<Value>,
        pub ddt_initialized: Vec<bool>,
        pub idt_previous: Vec<Value>,
        pub idt_initialized: Vec<bool>,
        pub limiter_anchor: Vec<Value>,
        pub limiter_initialized: Vec<bool>,
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct GeneratedVerilogARollbackState {
        pub values: Vec<Value>,
        pub flags: Vec<bool>,
    }

    pub struct GeneratedEvalContext<'a> {
        pub voltages: &'a [Value],
        pub temperature: Value,
    }

    impl GeneratedEvalContext<'_> {
        pub fn node_voltage(&self, node: usize) -> Value {
            self.voltages.get(node).copied().unwrap_or(0.0)
        }
        pub fn branch_current(&self, branch: usize) -> Value {
            self.voltages.get(branch).copied().unwrap_or(0.0)
        }
        pub fn temperature(&self) -> Value {
            self.temperature
        }
        pub fn thermal_voltage(&self) -> Value {
            self.temperature * 8.617_333_262e-5
        }
        pub fn analysis(&self, _query: &str) -> bool {
            false
        }
        pub fn simparam_or(&self, _name: &str, fallback: Value) -> Value {
            fallback
        }
    }

    #[derive(Default)]
    pub struct GeneratedStamper<'a> {
        pub sink: Option<&'a mut [Value]>,
    }

    impl GeneratedStamper<'_> {
        pub fn stamp_current_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _value: Value,
            _node_indices: [usize; NODE_COUNT],
            _node_derivatives: [Value; NODE_COUNT],
            _branch_indices: [usize; BRANCH_COUNT],
            _branch_derivatives: [Value; BRANCH_COUNT],
            _scale: Value,
        ) {
        }

        pub fn stamp_potential_branch_local(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _branch: usize,
            _multiplicity: Value,
        ) {
        }

        pub fn stamp_potential_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
            &mut self,
            _branch: usize,
            _value: Value,
            _node_indices: [usize; NODE_COUNT],
            _node_derivatives: [Value; NODE_COUNT],
            _branch_indices: [usize; BRANCH_COUNT],
            _branch_derivatives: [Value; BRANCH_COUNT],
        ) {
        }
    }

    #[derive(Default)]
    pub struct GeneratedReactiveStamper<'a> {
        pub sink: Option<&'a mut [Value]>,
    }

    impl GeneratedReactiveStamper<'_> {
        pub fn stamp_current_reactive_indexed_dense_local(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _nodes: &[usize],
            _node_derivatives: &[Value],
            _branches: &[usize],
            _branch_derivatives: &[Value],
            _scale: Value,
        ) {
        }

        pub fn stamp_potential_reactive_indexed_dense_local(
            &mut self,
            _branch: usize,
            _nodes: &[usize],
            _node_derivatives: &[Value],
            _branches: &[usize],
            _branch_derivatives: &[Value],
        ) {
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedNoiseKind {
        White,
        Flicker,
        Table,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseEndpoint {
        pub local_node: Option<usize>,
        pub name: &'static str,
        pub is_internal: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseDescriptor {
        pub mechanism: &'static str,
        pub label: Option<&'static str>,
        pub kind: GeneratedNoiseKind,
        pub equation: usize,
        pub is_current: bool,
        pub branch_ordinal: Option<usize>,
        pub pos: GeneratedNoiseEndpoint,
        pub neg: GeneratedNoiseEndpoint,
        pub table_len: usize,
        pub table_log_interp: bool,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GeneratedNoiseEvaluation {
        pub active: bool,
        pub psd: Value,
        pub exponent: Option<Value>,
        pub table_operands: Vec<Value>,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedNoiseEvaluationRef<'a> {
        pub active: bool,
        pub psd: Value,
        pub exponent: Option<Value>,
        pub table_operands: &'a [Value],
    }

    pub trait GeneratedNoiseVisitor {
        fn visit(&mut self, index: usize, evaluation: GeneratedNoiseEvaluationRef<'_>) -> bool;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum GeneratedNoiseEvaluationError {
        SourceIndexOutOfRange { index: usize, count: usize },
        NonFinite { index: usize, quantity: &'static str, value: Value },
        NegativePower { index: usize, value: Value },
        InvalidMultiplicity { value: Value },
    }
}
"#;

//! What the emitted code *costs*, measured by running it.
//!
//! Every other measurement of this pipeline counts bytes. Bytes are a proxy,
//! and the objective is not a proxy: generated Rust that matches or beats
//! hand-written C. So this compiles the emitted body with `rustc -O` and times
//! it, which is the only way that claim can be checked.
//!
//! Three things make a microbenchmark of generated code lie, and all three are
//! defended against here:
//!
//! 1. **Constant folding through the inputs.** A model's parameters are literals
//!    in the generated program, so LLVM would fold the entire instance and
//!    temperature prologue away and report a Newton-only cost for the whole
//!    body — destroying the exact comparison this exists to make. Every input
//!    goes through `black_box` first.
//! 2. **Hoisting out of the timing loop.** The evaluated function is
//!    `#[inline(never)]`, and the bias is perturbed per iteration, so nothing
//!    the model computes is loop-invariant.
//! 3. **Dead-code elimination of the result.** The outputs are accumulated and
//!    the accumulator is `black_box`ed.
//!
//! The reported figure is the minimum over trials, not the mean: the true cost
//! is a floor that scheduler noise can only add to.

use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::schedule::{Stage, split as split_cfg};
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgEvalInputs, ValueId, differentiate, evaluate_cfg, optimize_cfg,
    schedule_cfg,
};
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::rust_backend::emit::{EmitBindings, RUNTIME_PRELUDE, emit_body};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// `(module, directory under models/veriloga, file)`.
type Model = (&'static str, &'static str, &'static str);

/// Models that compile fast enough to time on an ordinary run.
///
/// `vbic_4T_et_cf` is here for one reason: it is the only model in the corpus
/// with a checked-in ngspice C anchor (740.6 ns per device per Newton
/// iteration), so it is the one place "versus hand-written C" is a subtraction
/// rather than an argument.
const QUICK_MODELS: &[Model] = &[
    ("vbic_4T_et_cf", "vbic_1.3/vacode", "vbic_4T_et_cf.va"),
    ("r3_cmc", "cmc/r3_cmc_release1.1.2_2023Jun16", "r3_cmc.va"),
    ("EPFL_HEMT_10a", "epfl_hemt_3.0.0/vacode", "epfl_hemt.va"),
    (
        "DIODE_CMC",
        "cmc/diode_cmc_3.0_20250714/vacode",
        "diode_cmc.va",
    ),
];

/// The wide MOSFETs. Minutes each, mostly in their own front end and then in
/// `rustc` on a megabyte of emitted Rust — which is itself one of the numbers.
const HEAVY_MODELS: &[Model] = &[
    ("bsimbulk", "cmc/BSIM-BULK107.2.1_02112025/code", "bsimbulk.va"),
    ("bsimcmg_va", "cmc/BSIM-CMG_112.1.0_04282026/code", "bsimcmg.va"),
    (
        "hisimhv_va",
        "cmc/HiSIM_HV_2.5.1_Release_20230209/HiSIM_HV_2.5.1_VA-Code/hisimhv_va",
        "hisimhv.va",
    ),
];

#[test]
#[ignore = "compiles the emitted source with rustc and times it; run with --ignored"]
fn the_emitted_code_reports_what_it_costs() {
    report(QUICK_MODELS);
}

#[test]
#[ignore = "the wide MOSFETs: minutes each, most of it rustc; run with --ignored"]
fn the_wide_models_report_what_they_cost() {
    report(HEAVY_MODELS);
}

fn report(models: &[Model]) {
    let root = model_root();
    eprintln!(
        "{:>16}  {:>8}  {:>8}  {:>10}  {:>10}  {:>7}   {:>10}  {:>10}  {:>7}   {:>6}  {:>10}",
        "model",
        "newton%",
        "val cut",
        "whole ns",
        "newton ns",
        "speedup",
        "whole B",
        "newton B",
        "ratio",
        "rustc s",
        "notes"
    );
    for (module, directory, file) in models {
        let path = directory
            .split('/')
            .fold(root.clone(), |path, part| path.join(part))
            .join(file);
        assert!(path.exists(), "model fixture missing: {}", path.display());

        let measurement = measure(&root, &path, module);
        measurement.print();
    }
}

struct Measurement {
    module: String,
    whole_ns: f64,
    newton_ns: f64,
    whole_bytes: usize,
    newton_bytes: usize,
    rustc_seconds: f64,
    /// Cached values that came out non-finite at this synthetic bias, and how
    /// many there were. A Newton figure taken against poisoned inputs is not
    /// wrong so much as not about the model, and that has to be visible rather
    /// than folded into a number that looks like every other number.
    unusable_staged: (usize, usize),
    /// Variables the model read before assigning.
    warnings: usize,
    /// Share of values in the Newton class. What the split has to work against:
    /// everything else is work a Newton iteration should never repeat, and a
    /// model with nothing else has nothing to gain.
    newton_share: f64,
    /// Values in the whole body against values in the Newton stage. The
    /// candidate for deciding whether to split at all, and the reason to
    /// measure it here is that it is free — no second emission — where the byte
    /// ratio is not.
    value_ratio: f64,
}

impl Measurement {
    fn print(&self) {
        let mut notes = Vec::new();
        if self.unusable_staged.0 > 0 {
            notes.push(format!(
                "{}/{} cached values non-finite",
                self.unusable_staged.0, self.unusable_staged.1
            ));
        }
        if self.warnings > 0 {
            notes.push(format!("{} read before assignment", self.warnings));
        }
        eprintln!(
            "{:>16}  {:>7.1}%  {:>7.2}x  {:>10.1}  {:>10.1}  {:>6.2}x   {:>10}  {:>10}  {:>6.2}x   {:>6.1}  {}",
            self.module,
            100.0 * self.newton_share,
            self.value_ratio,
            self.whole_ns,
            self.newton_ns,
            self.whole_ns / self.newton_ns.max(f64::MIN_POSITIVE),
            self.whole_bytes,
            self.newton_bytes,
            self.whole_bytes as f64 / self.newton_bytes.max(1) as f64,
            self.rustc_seconds,
            notes.join("; "),
        );
    }
}

fn measure(root: &Path, path: &Path, module: &str) -> Measurement {
    let artifact = compile(root, path, module);
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
        .unwrap_or_else(|diagnostics| panic!("{module}: lowering: {diagnostics:?}"));

    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len())
                .map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .collect();
    let mut differentiated = differentiate(&cfg.function, &lanes)
        .unwrap_or_else(|error| panic!("{module}: differentiation: {error}"));

    // Every read-out has to be taken before anything evaluates or emits: taking
    // a lane appends an instruction to the function.
    let mut wanted = cfg.residuals.clone();
    for residual in &cfg.residuals.clone() {
        wanted.extend(differentiated.derivative_row(*residual).into_iter().flatten());
    }
    let (optimized, wanted) = optimize_cfg(&differentiated.function, &wanted);

    let bias = Bias::of(&artifact);
    let directory = scratch(module);

    // The whole body, as one function.
    let (whole_body, whole_names) = emit_body(&optimized, &wanted, &EmitBindings::default())
        .unwrap_or_else(|error| panic!("{module}: emission: {error}"));
    let whole_program = program(&whole_body, &whole_names, &bias, &[]);
    let (whole_ns, whole_rustc, whole_outputs) =
        compile_and_time(&directory, "whole", &whole_program, module);

    let reference = evaluate_cfg(&optimized, &bias.inputs(&[]))
        .unwrap_or_else(|error| panic!("{module}: interpreter: {error}"));
    let expected: Vec<f64> = wanted
        .iter()
        .map(|value| {
            reference.value(*value).unwrap_or_else(|| {
                panic!(
                    "{module}: {value} ({:?}) has no value at this bias, though the \
                     emitted code produced one",
                    optimized.values[usize::from(*value)].kind
                )
            })
        })
        .collect();
    agree(module, "whole", &whole_outputs, &expected);

    // The Newton stage alone, reading what the coarser stages cached.
    let schedule = schedule_cfg(&optimized);
    let census = schedule.census();
    let newton_share = census[3] as f64 / census.iter().sum::<usize>().max(1) as f64;
    let stages = split_cfg(&optimized, &schedule, &wanted)
        .unwrap_or_else(|error| panic!("{module}: split: {error}"));
    let staged = staged_values(&stages, &bias, module);
    let newton = stages
        .last()
        .expect("the split always produces at least one stage");
    let produced: Vec<ValueId> = newton.outputs.iter().flatten().copied().collect();
    let (newton_body, newton_names) =
        emit_body(&newton.function, &produced, &EmitBindings::default())
            .unwrap_or_else(|error| panic!("{module}: newton stage: {error}"));
    let newton_program = program(&newton_body, &newton_names, &bias, &staged);
    let (newton_ns, newton_rustc, newton_outputs) =
        compile_and_time(&directory, "newton", &newton_program, module);

    // Against the interpreter on the *stage*, not the whole body: the split's
    // own equivalence is `cfg_schedule`'s job, and conflating the two would
    // leave a failure here pointing at both.
    let reference = evaluate_cfg(&newton.function, &bias.inputs(&staged))
        .unwrap_or_else(|error| panic!("{module}: newton interpreter: {error}"));
    let expected: Vec<f64> = produced
        .iter()
        .map(|value| reference.value(*value).expect("defined on every path"))
        .collect();
    agree(module, "newton", &newton_outputs, &expected);

    Measurement {
        module: module.to_string(),
        whole_ns,
        newton_ns,
        whole_bytes: whole_body.len(),
        newton_bytes: newton_body.len(),
        rustc_seconds: whole_rustc + newton_rustc,
        unusable_staged: (
            staged.iter().filter(|value| !value.is_finite()).count(),
            staged.len(),
        ),
        warnings: cfg.warnings.len(),
        newton_share,
        value_ratio: optimized.values.len().max(1) as f64
            / newton.function.values.len().max(1) as f64,
    }
}

/// Run the stages in order through the interpreter to fill the slot array the
/// Newton stage reads.
///
/// These have to be real values rather than zeros: a stage fed zeros can divide
/// by one, take a logarithm of one, or branch the other way, and would not be
/// the same computation.
fn staged_values(stages: &[Stage], bias: &Bias, module: &str) -> Vec<f64> {
    let slots = stages
        .iter()
        .flat_map(|stage| stage.exports.iter().map(|(slot, _)| *slot as usize + 1))
        .max()
        .unwrap_or(0);
    let mut staged = vec![0.0f64; slots];
    for stage in stages {
        let snapshot = evaluate_cfg(&stage.function, &bias.inputs(&staged))
            .unwrap_or_else(|error| panic!("{module}: {} stage: {error}", stage.class.name()));
        for (slot, value) in &stage.exports {
            // A value defined only inside a conditional this bias did not take
            // has nothing to cache, and nothing reads it for the same reason.
            if let Some(held) = snapshot.value(*value) {
                staged[*slot as usize] = held;
            }
        }
    }
    staged
}

/// Wrap an emitted body in a self-timing program.
fn program(body: &str, names: &[String], bias: &Bias, staged: &[f64]) -> String {
    let assignments: String = names
        .iter()
        .enumerate()
        .map(|(index, name)| format!("    out[{index}] = {name};\n"))
        .collect();

    // With no nodes there is nothing to perturb, and nothing bias-dependent to
    // measure either — but the program still has to compile.
    let perturb = if bias.node_potentials.is_empty() {
        String::new()
    } else {
        "            nodes[0] = base_nodes[0] + (step & 31) as f64 * 1.0e-9;".to_string()
    };

    format!(
        r#"#![allow(unused_variables, unused_parens, unused_mut, dead_code, clippy::all)]
{RUNTIME_PRELUDE}

fn analysis(_name: &str) -> f64 {{ 0.0 }}
fn simparam(_name: &str, fallback: f64) -> f64 {{ fallback }}
fn ddt(_operator: usize, _input: f64) -> f64 {{ 0.0 }}
fn ddt_scale() -> f64 {{ 0.0 }}
fn limit(_operator: usize, _proposed: f64, candidate: f64) -> f64 {{ candidate }}
fn limit_previous(_operator: usize, proposed: f64) -> f64 {{ proposed }}

/// Never inlined, so the timing loop cannot hoist any part of the model out of
/// itself — the same boundary a generated device method actually sits behind.
#[inline(never)]
fn evaluate(
    parameters: &[f64; {parameter_count}],
    parameter_given: &[bool; {parameter_count}],
    node_potentials: &[f64; {node_count}],
    branch_flows: &[f64; {branch_count}],
    branch_unknown_flows: &[f64; {unknown_count}],
    temperature: f64,
    thermal_voltage: f64,
    multiplicity: f64,
    time: f64,
    staged: &[f64; {staged_count}],
    out: &mut [f64; {output_count}],
) {{
{body}
{assignments}}}

fn main() {{
    // Opaque to the optimiser, all of it. Left as literals, the parameters
    // would fold the whole instance and temperature prologue to constants and
    // the whole-body figure would silently become a Newton-only one.
    let parameters = std::hint::black_box([{parameters}]);
    let parameter_given = std::hint::black_box([{given}]);
    let base_nodes = std::hint::black_box([{nodes}]);
    let branch_flows = std::hint::black_box([{branches}]);
    let branch_unknown_flows = std::hint::black_box([{unknowns}]);
    let staged = std::hint::black_box([{staged}]);
    let temperature = std::hint::black_box({temperature:e}f64);
    let thermal_voltage = std::hint::black_box({thermal_voltage:e}f64);
    let multiplicity = std::hint::black_box(1.0f64);
    let time = std::hint::black_box(0.0f64);

    let mut nodes = base_nodes;
    let mut out = [0.0f64; {output_count}];
    let mut checksum = 0.0f64;

    let mut run = |iterations: u64, checksum: &mut f64, nodes: &mut [f64; {node_count}]| {{
        let start = std::time::Instant::now();
        for step in 0..iterations {{
{perturb}
            evaluate(
                &parameters, &parameter_given, nodes, &branch_flows,
                &branch_unknown_flows, temperature, thermal_voltage,
                multiplicity, time, &staged, &mut out,
            );
            *checksum += out[0];
        }}
        start.elapsed().as_nanos() as f64 / iterations as f64
    }};

    // Pilot, then size the trials to about 50 ms each so a fast model is not
    // measured against timer granularity and a slow one does not take a minute.
    let pilot = run(64, &mut checksum, &mut nodes);
    let iterations = (50_000_000.0 / pilot.max(1.0)) as u64;
    let iterations = iterations.clamp(64, 400_000);

    let mut best = f64::INFINITY;
    for _ in 0..5 {{
        let each = run(iterations, &mut checksum, &mut nodes);
        if each < best {{
            best = each;
        }}
    }}

    std::hint::black_box(checksum);
    println!("{{best}}");

    // Then the same evaluation at the unperturbed bias, so the caller can hold
    // this against the reference interpreter. A timing harness that does not
    // also check the answer measures how fast the wrong number arrives.
    nodes = base_nodes;
    evaluate(
        &parameters, &parameter_given, &nodes, &branch_flows,
        &branch_unknown_flows, temperature, thermal_voltage,
        multiplicity, time, &staged, &mut out,
    );
    for value in out.iter() {{
        println!("{{:x}}", f64::to_bits(*value));
    }}
}}
"#,
        parameter_count = bias.parameters.len().max(1),
        parameters = literals(&bias.parameters),
        given = if bias.parameters.is_empty() {
            "false".to_string()
        } else {
            vec!["false"; bias.parameters.len()].join(", ")
        },
        node_count = bias.node_potentials.len().max(1),
        nodes = literals(&bias.node_potentials),
        branch_count = bias.branch_flows.len().max(1),
        branches = literals(&bias.branch_flows),
        unknown_count = bias.branch_unknown_flows.len().max(1),
        unknowns = literals(&bias.branch_unknown_flows),
        staged_count = staged.len().max(1),
        staged = literals(staged),
        temperature = 300.15,
        thermal_voltage = 300.15 * 8.617_333_262e-5,
        output_count = names.len().max(1),
    )
}

/// A Rust array body, never empty — a zero-length array would need a type
/// annotation everywhere it is passed.
fn literals(values: &[f64]) -> String {
    if values.is_empty() {
        return "0.0f64".to_string();
    }
    values
        .iter()
        .map(|value| literal(*value))
        .collect::<Vec<_>>()
        .join(", ")
}

/// One `f64` as Rust source.
///
/// `{:e}` prints a non-finite as `NaN` or `inf`, neither of which is a Rust
/// literal — and a cached value being non-finite is a thing that happens, since
/// these biases are synthetic rather than a model's own operating point.
fn literal(value: f64) -> String {
    if value.is_nan() {
        return "f64::NAN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 {
            "f64::INFINITY".to_string()
        } else {
            "f64::NEG_INFINITY".to_string()
        };
    }
    format!("{value:e}f64")
}

/// Compile, run, and return `(nanoseconds per evaluation, rustc seconds, outputs)`.
fn compile_and_time(
    directory: &Path,
    tag: &str,
    program: &str,
    module: &str,
) -> (f64, f64, Vec<f64>) {
    let source = directory.join(format!("{tag}.rs"));
    let binary = directory.join(format!("{tag}{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&source, program).expect("scratch directory is writable");

    // `codegen-units=1` because the shipped build is LTO'd: measuring the
    // emitted code under weaker optimisation than it will actually get would
    // understate it.
    let started = Instant::now();
    let compile = Command::new("rustc")
        .arg("--edition=2021")
        .arg("-C")
        .arg("opt-level=3")
        .arg("-C")
        .arg("codegen-units=1")
        .arg("-o")
        .arg(&binary)
        .arg(&source)
        .output()
        .expect("rustc must be on PATH");
    let rustc_seconds = started.elapsed().as_secs_f64();
    assert!(
        compile.status.success(),
        "{module}/{tag}: the emitted source did not compile\n{}",
        String::from_utf8_lossy(&compile.stderr)
    );

    let run = Command::new(&binary).output().expect("the program runs");
    assert!(
        run.status.success(),
        "{module}/{tag}: the emitted program failed\n{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    let mut lines = stdout.lines();
    let nanoseconds = lines
        .next()
        .expect("the program prints its timing first")
        .trim()
        .parse::<f64>()
        .expect("the timing is a number");
    let outputs = lines
        .map(|line| {
            f64::from_bits(u64::from_str_radix(line.trim(), 16).expect("a hexadecimal bit pattern"))
        })
        .collect();
    (nanoseconds, rustc_seconds, outputs)
}

/// The emitter and the interpreter evaluate the same operations in the same
/// order, so anything less than bit equality is a difference in meaning rather
/// than a rounding artefact.
fn agree(module: &str, tag: &str, emitted: &[f64], interpreted: &[f64]) {
    assert_eq!(
        emitted.len(),
        interpreted.len(),
        "{module}/{tag}: emitted {} values against {} interpreted",
        emitted.len(),
        interpreted.len()
    );
    for (index, (emitted, interpreted)) in emitted.iter().zip(interpreted).enumerate() {
        assert!(
            emitted.to_bits() == interpreted.to_bits()
                || (emitted.is_nan() && interpreted.is_nan()),
            "{module}/{tag}: output {index} is {emitted} from the emitted code \
             and {interpreted} from the interpreter"
        );
    }
}

struct Bias {
    parameters: Vec<f64>,
    node_potentials: Vec<f64>,
    branch_flows: Vec<f64>,
    branch_unknown_flows: Vec<f64>,
}

impl Bias {
    fn of(artifact: &CanonicalIrArtifact) -> Self {
        Self {
            parameters: artifact
                .mir
                .parameters
                .iter()
                .map(|parameter| parameter.default.unwrap_or(0.0))
                .collect(),
            node_potentials: (0..artifact.mir.nodes.len())
                .map(|index| 0.41 - 0.13 * index as f64)
                .collect(),
            branch_flows: (0..artifact.mir.branches.len())
                .map(|index| 1.0e-4 * (index as f64 + 1.0))
                .collect(),
            branch_unknown_flows: (0..artifact.mir.branch_unknowns.len())
                .map(|index| 1.0e-4 * (index as f64 + 1.0))
                .collect(),
        }
    }

    fn inputs(&self, staged: &[f64]) -> CfgEvalInputs<f64> {
        CfgEvalInputs {
            parameters: self.parameters.clone(),
            parameter_given: vec![false; self.parameters.len()],
            node_potentials: self.node_potentials.clone(),
            branch_flows: self.branch_flows.clone(),
            branch_unknown_flows: self.branch_unknown_flows.clone(),
            temperature: 300.15,
            thermal_voltage: 300.15 * 8.617_333_262e-5,
            multiplicity: 1.0,
            time: 0.0,
            analyses: HashSet::new(),
            simparams: HashMap::new(),
            ddt: 0.0,
            ddt_scale: 0.0,
            idt: 0.0,
            idt_scale: 0.0,
            staged: staged.to_vec(),
        }
    }
}

fn compile(root: &Path, path: &Path, module: &str) -> CanonicalIrArtifact {
    let mut options = CompilerOptions::default();
    options.include_paths.push(root.to_path_buf());
    if let Some(profile) = profile_for(path) {
        options.defines = profile.0;
        options.undefines = profile.1;
    }
    VerilogACompiler::new(options)
        .compile_file_canonical_ir_with_metadata(path, Some(module))
        .unwrap_or_else(|error| panic!("{module}: front end: {error}"))
        .artifact
}

fn profile_for(path: &Path) -> Option<(Vec<(String, Option<String>)>, Vec<String>)> {
    let directory = path.parent()?;
    discover_veriloga_sources(directory)
        .ok()?
        .into_iter()
        .find(|candidate| candidate.path == path)
        .map(|candidate| {
            (
                candidate.compile_profile.defines,
                candidate.compile_profile.undefines,
            )
        })
}

fn scratch(name: &str) -> PathBuf {
    let slug: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let directory = std::env::temp_dir().join("rspice-cfg-runtime").join(slug);
    std::fs::create_dir_all(&directory).expect("scratch directory is creatable");
    directory
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

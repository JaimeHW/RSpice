//! Agreement census between the canonical CFG's reference interpreter and the
//! block program this backend lowers from the same CFG.
//!
//! [`cfg_program`](crate::jit::cfg_program) is a second route to machine code:
//! where the shipped route lowers MIR's flat postfix stream, this one carries
//! the CFG's blocks, terminators and typed block parameters onto the native
//! block model. The two routes cannot be compared bit for bit — a CFG-sourced
//! program associates its arithmetic differently, so rounding differs, and
//! W-F is where that bar is retired — so the oracle here is
//! [`evaluate_cfg`](crate::canonical_ir::evaluate_cfg), the reference
//! interpreter the module documentation names as the thing every backend is
//! checked against.
//!
//! What the census answers, per shipped module:
//!
//! * does the lowering cover the module at all, and if not, which canonical
//!   construct stopped it — every refusal names one;
//! * does the compiled block program agree with the interpreter at the
//!   operating points, and by how much.
//!
//! `#[ignore]`d: this is release-qualification work. Run it with
//! `--release --features native -- --ignored --nocapture`.

use std::path::{Path, PathBuf};

use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::{
    CanonicalStateFamily, CfgEvalInputs, CfgStateAllocation, evaluate_cfg, prune_cfg_to_outputs,
};
use crate::jit::cfg_program::{CfgRuntimeBindings, lower_cfg_function};
use crate::jit::plan_builder::canonical_branch_unknown_runtime_map;
use crate::native::abi::EvalContext;
use crate::native::runtime::ExecutableMemory;
use crate::native::x64::codegen::compile_value_function_artifact_from_ssa;
use crate::rust_backend::discover_veriloga_sources;
use crate::{CompilerOptions, VerilogACompiler};

/// Outputs lowered and compared per model.
///
/// Every residual is *lowered*, because totality of the lowering over the
/// corpus is half of what the census answers. Only a bounded sample is
/// compiled and executed: a compact model's residual depends on nearly its
/// whole body, so machine code for each of eighty of them is eighty copies of
/// the same twenty thousand instructions and measures nothing the first four
/// do not.
const EXECUTED_OUTPUTS_PER_MODEL: usize = 4;

/// A deterministic spread of operating points.
///
/// Zeros would make every comparison vacuous and one repeated value would hide
/// an index mistake, so each input gets its own value from a fixed sequence.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Self(seed | 1)
    }

    fn next(&mut self) -> f64 {
        // SplitMix64, for a reproducible spread without a dependency.
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^= z >> 31;
        // A decade around unity, signed: small enough that an exponential does
        // not overflow every model, large enough that a comparison is not
        // deciding on noise.
        let unit = (z >> 11) as f64 / (1_u64 << 53) as f64;
        (unit - 0.5) * 2.0
    }
}

/// Everything both routes read, filled once per operating point.
struct OperatingPoint {
    parameters: Vec<f64>,
    parameter_given: Vec<u8>,
    port_connected: Vec<u8>,
    terminal_voltages: Vec<f64>,
    internal_voltages: Vec<f64>,
    branch_unknowns: Vec<f64>,
    temperature: f64,
    multiplicity: f64,
    time: f64,
    analysis: u8,
    /// Zeroed analog-operator storage, sized by the module's canonical state
    /// layout. With `integration_active` clear, `ddt` and `idt` answer zero
    /// through it, which is exactly the static evaluation the interpreter
    /// performs when its `ddt` and `idt` inputs are zero.
    state: Vec<f64>,
    state_flags: Vec<u8>,
    /// Accepted event-controlled procedural state, in dense slot order. The
    /// interpreter refuses a slot it was not given, and every value is zero
    /// because a static evaluation has no accepted history.
    event_state_slots: usize,
}

const BOLTZMANN_OVER_ELECTRON: f64 = 1.380_649e-23 / 1.602_176_634e-19;

impl OperatingPoint {
    fn new(
        seed: u64,
        analysis: u8,
        parameter_defaults: &[Option<f64>],
        terminal_count: usize,
        internal_count: usize,
        branch_unknown_count: usize,
        state_len: usize,
        event_state_slots: usize,
    ) -> Self {
        let parameter_count = parameter_defaults.len();
        let mut rng = Rng::new(seed);
        let mut fill = |count: usize| (0..count).map(|_| rng.next()).collect::<Vec<f64>>();
        // A compact model's parameters are not interchangeable numbers: a
        // random oxide thickness makes half the body compute a logarithm of
        // something negative, and a comparison that both routes refuse proves
        // nothing. Declared defaults are the operating point the model was
        // written for; only a parameter with no default gets a number from the
        // sequence.
        let parameters = parameter_defaults
            .iter()
            .zip(fill(parameter_count))
            .map(|(default, sampled)| default.unwrap_or(sampled))
            .collect();
        Self {
            parameters,
            parameter_given: vec![1; parameter_count],
            port_connected: vec![1; terminal_count],
            terminal_voltages: fill(terminal_count),
            internal_voltages: fill(internal_count),
            branch_unknowns: fill(branch_unknown_count),
            temperature: 300.15,
            multiplicity: 1.0,
            time: 1.0e-9,
            analysis,
            state: vec![0.0; state_len],
            state_flags: vec![0; state_len],
            event_state_slots,
        }
    }

    fn interpreter_inputs(&self, node_count: usize, branch_count: usize) -> CfgEvalInputs<f64> {
        let mut node_potentials = self.terminal_voltages.clone();
        node_potentials.extend(self.internal_voltages.iter().copied());
        node_potentials.resize(node_count, 0.0);
        CfgEvalInputs {
            parameters: self.parameters.clone(),
            parameter_given: self.parameter_given.iter().map(|byte| *byte != 0).collect(),
            port_connected: self.port_connected.iter().map(|byte| *byte != 0).collect(),
            event_state: vec![0.0; self.event_state_slots],
            node_potentials,
            branch_flows: vec![0.0; branch_count],
            branch_unknown_flows: self.branch_unknowns.clone(),
            temperature: self.temperature,
            thermal_voltage: BOLTZMANN_OVER_ELECTRON * self.temperature,
            multiplicity: self.multiplicity,
            time: self.time,
            analyses: analysis_names(self.analysis),
            simparams: std::collections::HashMap::new(),
            ddt: 0.0,
            ddt_scale: 0.0,
            idt: 0.0,
            idt_scale: 0.0,
            event_controls: std::collections::HashMap::new(),
            staged: Vec::new(),
        }
    }

    fn context(&mut self) -> EvalContext {
        let mut context = EvalContext::empty_for_test();
        context.params = self.parameters.as_ptr();
        context.param_given = self.parameter_given.as_ptr();
        context.param_given_len = self.parameter_given.len();
        context.port_connected = self.port_connected.as_ptr();
        context.port_connected_len = self.port_connected.len();
        context.voltages = self.terminal_voltages.as_ptr();
        context.internal_voltages = self.internal_voltages.as_ptr();
        context.branch_unknowns = self.branch_unknowns.as_ptr();
        context.num_terminals = self.terminal_voltages.len();
        context.temperature = self.temperature;
        context.multiplicity = self.multiplicity;
        context.time = self.time;
        context.analysis_type = self.analysis;
        context.integration_active = 0;
        let values = self.state.as_mut_ptr();
        let flags = self.state_flags.as_mut_ptr();
        let len = self.state.len();
        context.state_prev = values;
        context.state_prev_len = len;
        context.state_older = values;
        context.state_older_len = len;
        context.state_derivatives_prev = values;
        context.state_derivatives_prev_len = len;
        context.state_values = values;
        context.state_values_len = len;
        context.state_derivatives = values;
        context.state_derivatives_len = len;
        context.state_older_candidate = values;
        context.state_older_candidate_len = len;
        context.state_initialized = flags;
        context.state_initialized_len = len;
        context.state_candidate_valid = flags;
        context.state_candidate_valid_len = len;
        context
    }
}

fn analysis_names(analysis: u8) -> std::collections::HashSet<smol_str::SmolStr> {
    let names: &[&str] = match analysis {
        0 => &["dc", "op", "static"],
        1 => &["ac"],
        2 => &["tran", "transient"],
        3 => &["noise"],
        _ => &[],
    };
    names.iter().map(|name| (*name).into()).collect()
}

fn shipped_model_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
}

#[derive(Default)]
struct Tally {
    models: usize,
    lowered_outputs: usize,
    refused_outputs: usize,
    executed_outputs: usize,
    comparisons: usize,
    /// Evaluations the block program refused at run time — a bounds-checked
    /// read or a domain error at a randomly chosen operating point. Counted
    /// rather than compared: the interpreter reports such a failure as a
    /// different kind of answer, so there is no number to subtract.
    runtime_errors: usize,
    /// Models whose primal CFG the reference interpreter itself cannot
    /// evaluate, which is every model whose residual reads an undifferentiated
    /// `ddx` probe.
    oracle_refusals: usize,
}

/// Relative deviation, treating two NaNs and two identical infinities as
/// agreement: the two routes must agree about a model leaving the reals, not
/// about which NaN payload it produced.
fn deviation(expected: f64, actual: f64) -> Option<f64> {
    if expected.is_nan() && actual.is_nan() {
        return None;
    }
    if expected == actual {
        return None;
    }
    let scale = expected.abs().max(actual.abs()).max(f64::MIN_POSITIVE);
    Some((expected - actual).abs() / scale)
}

#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_block_lowering_agrees_with_the_reference_interpreter() {
    let root = shipped_model_root();
    let candidates = discover_veriloga_sources(&root).expect("discover shipped Verilog-A sources");
    let mut tally = Tally::default();
    let mut worst_overall = 0.0_f64;
    // The same affordance the emitted-code benchmark carries: a substring that
    // narrows repeated runs to one model while it is being investigated. The
    // corpus assertion below is skipped whenever it is set, so a filtered run
    // cannot be mistaken for a full one.
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    for candidate in candidates {
        for module in &candidate.modules {
            if filter
                .as_deref()
                .is_some_and(|filter| !module.contains(filter))
            {
                continue;
            }
            let started = std::time::Instant::now();
            let mut options = CompilerOptions::default();
            options.include_paths.push(root.clone());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiler = VerilogACompiler::new(options);
            let runtime = compiler
                .compile_file_runtime_with_metadata(&candidate.path, Some(module))
                .unwrap_or_else(|error| {
                    panic!("compile {} :: {module}: {error}", candidate.path.display())
                });
            let artifact = &runtime.canonical_ir;
            tally.models += 1;
            let cfg = match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
                Ok(cfg) => cfg,
                Err(diagnostics) => {
                    println!(
                        "cfg-census model={module} refused=cfg-lowering detail={}",
                        diagnostics.first().map_or_else(
                            || "unknown".to_string(),
                            |first| first.message.to_string()
                        )
                    );
                    continue;
                }
            };
            let state = match CfgStateAllocation::build(&artifact.hir, &cfg.function) {
                Ok(state) => state,
                Err(errors) => {
                    println!(
                        "cfg-census model={module} refused=state-allocation detail={}",
                        errors
                            .first()
                            .map_or_else(|| "unknown".to_string(), ToString::to_string)
                    );
                    continue;
                }
            };
            let branch_unknowns =
                canonical_branch_unknown_runtime_map(&runtime.model, &artifact.mir)
                    .unwrap_or_else(|error| panic!("{module}: branch unknown map: {error}"));
            // The CFG numbers event-controlled state in HIR declaration order;
            // the runtime numbers every variable the model has. Names are what
            // the two spaces share.
            let event_state_variables: Vec<Option<usize>> = artifact
                .hir
                .variables
                .iter()
                .filter(|variable| variable.is_state)
                .map(|variable| {
                    runtime
                        .model
                        .variable_names
                        .iter()
                        .position(|name| *name == variable.name)
                })
                .collect();
            let bindings = CfgRuntimeBindings::from_mir(
                module.as_str(),
                &artifact.mir,
                branch_unknowns,
                event_state_variables,
            );

            // Only the integration family is addressable from a program this
            // lowering emits — `ddt` and `idt` are the two state-bearing kinds
            // it covers — and its records are the parallel scalar lanes the
            // context exposes. The slack keeps a stray slot inside the
            // allocation rather than deciding whether this process survives.
            let state_len = state.family_len(CanonicalStateFamily::Integration) + 8;
            let parameter_defaults: Vec<Option<f64>> = artifact
                .mir
                .parameters
                .iter()
                .map(|parameter| parameter.default)
                .collect();
            let mut points: Vec<OperatingPoint> =
                [(0x0005_EED1_u64, 0_u8), (0x00C0_FFEE, 2), (0x0000_BEEF, 0)]
                    .into_iter()
                    .map(|(seed, analysis)| {
                        OperatingPoint::new(
                            seed,
                            analysis,
                            &parameter_defaults,
                            bindings.terminal_count,
                            bindings.internal_node_count,
                            artifact.mir.branch_unknowns.len(),
                            state_len,
                            cfg.event_state_candidates.len(),
                        )
                    })
                    .collect();

            // The interpreter runs once per operating point on the whole
            // function; pruning to one output does not change any value it
            // keeps, so its answer serves every output.
            let expected: Vec<_> = points
                .iter()
                .map(|point| {
                    evaluate_cfg(
                        &cfg.function,
                        &point.interpreter_inputs(
                            artifact.mir.nodes.len(),
                            artifact.mir.branches.len(),
                        ),
                    )
                })
                .collect();
            if let Some(Err(error)) = expected.iter().find(|result| result.is_err()) {
                tally.oracle_refusals += 1;
                println!("cfg-census model={module} oracle-refused detail={error:?}");
            }

            // Zeroed, and the interpreter's accepted event state is zero too:
            // the only variable slot a CFG-lowered residual reads is the
            // accepted value of an event-controlled state variable, and a
            // static evaluation has no accepted history.
            let variables = vec![0.0_f64; runtime.model.num_variables + 8];

            let mut lowered = 0_usize;
            let mut first_refusal: Option<String> = None;
            let mut executed = 0_usize;
            let mut worst = 0.0_f64;
            for (ordinal, residual) in cfg.residuals.iter().copied().enumerate() {
                let (pruned, outputs) = prune_cfg_to_outputs(&cfg.function, &[residual]);
                let output = outputs[0];
                let program = match lower_cfg_function(&pruned, output, &state, &bindings) {
                    Ok(program) => {
                        lowered += 1;
                        program
                    }
                    Err(error) => {
                        tally.refused_outputs += 1;
                        first_refusal.get_or_insert_with(|| error.to_string());
                        continue;
                    }
                };
                if executed >= EXECUTED_OUTPUTS_PER_MODEL {
                    continue;
                }
                let artifact_image = compile_value_function_artifact_from_ssa(&program)
                    .unwrap_or_else(|error| {
                        panic!("{module} residual {ordinal}: x64 codegen: {error}")
                    });
                let memory =
                    ExecutableMemory::allocate(artifact_image.bytes()).unwrap_or_else(|error| {
                        panic!("{module} residual {ordinal}: publish: {error}")
                    });
                let entry = memory.ptr_at(0).expect("entry inside published image");
                let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                    unsafe { std::mem::transmute(entry) };
                executed += 1;
                for (point, snapshot) in points.iter_mut().zip(&expected) {
                    let Ok(snapshot) = snapshot else { continue };
                    let Some(reference) = snapshot.value(residual) else {
                        continue;
                    };
                    let context = point.context();
                    context.clear_runtime_error();
                    let actual = function(&context, variables.as_ptr());
                    if context.take_runtime_error().is_some() {
                        tally.runtime_errors += 1;
                        continue;
                    }
                    tally.comparisons += 1;
                    if let Some(delta) = deviation(reference, actual) {
                        worst = worst.max(delta);
                    }
                }
            }
            tally.lowered_outputs += lowered;
            tally.executed_outputs += executed;
            worst_overall = worst_overall.max(worst);
            println!(
                "cfg-census model={module} outputs={} lowered={lowered} executed={executed} max_relative_deviation={worst:.3e} seconds={:.1}{}",
                cfg.residuals.len(),
                started.elapsed().as_secs_f64(),
                first_refusal
                    .map(|refusal| format!(" first_refusal={refusal}"))
                    .unwrap_or_default(),
            );
        }
    }
    println!(
        "cfg-census models={} lowered_outputs={} refused_outputs={} executed_outputs={} comparisons={} runtime_errors={} oracle_refusals={} max_relative_deviation={worst_overall:.3e}",
        tally.models,
        tally.lowered_outputs,
        tally.refused_outputs,
        tally.executed_outputs,
        tally.comparisons,
        tally.runtime_errors,
        tally.oracle_refusals,
    );
    if filter.is_none() {
        assert_eq!(tally.models, 43, "the shipped census is 43 modules");
    }
    assert!(
        tally.comparisons > 0,
        "the census must actually execute the lowered programs"
    );
}

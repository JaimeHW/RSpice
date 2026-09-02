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

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::{
    AdSeed, CanonicalStateFamily, CfgEvalInputs, CfgEvalSnapshot, CfgScalar, CfgStateAllocation,
    CfgValueKind, ComplexStep, MirModel, ValueId, differentiate, evaluate_cfg,
    prune_cfg_to_outputs,
};
use crate::codegen::ColumnAxis;
use crate::jit::cfg_lanes::scalarize_lanes;
use crate::jit::cfg_program::{CfgRuntimeBindings, lower_cfg_function};
use crate::jit::expr::BranchUnknownRuntimeMapping;
use crate::jit::plan_builder::canonical_branch_unknown_runtime_map;
use crate::native::abi::EvalContext;
use crate::native::model::NativeRequiredStorage;
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
    /// Branch-unknown flows in *canonical* order, which is what the CFG names
    /// and the interpreter indexes.
    branch_unknowns: Vec<f64>,
    /// The same flows in *runtime* order, with the sign the runtime branch
    /// source carries.
    ///
    /// Two arrays rather than one: a canonical branch unknown and the runtime
    /// branch source it maps to are two numbering spaces, and one of them
    /// reverses the branch's ends. Handing a single array to both routes makes
    /// them disagree about a value neither got wrong.
    runtime_branch_unknowns: Vec<f64>,
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
        branch_unknowns: &[BranchUnknownRuntimeMapping],
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
        let canonical_flows = fill(branch_unknowns.len());
        let mut runtime_flows = vec![0.0; branch_unknowns.len()];
        for (mapping, flow) in branch_unknowns.iter().zip(&canonical_flows) {
            if let Some(slot) = runtime_flows.get_mut(mapping.runtime_index) {
                *slot = if mapping.inverted { -*flow } else { *flow };
            }
        }
        Self {
            parameters,
            parameter_given: vec![1; parameter_count],
            port_connected: vec![1; terminal_count],
            terminal_voltages: fill(terminal_count),
            internal_voltages: fill(internal_count),
            branch_unknowns: canonical_flows,
            runtime_branch_unknowns: runtime_flows,
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

    /// The same operating point in complex-step arithmetic, with `seed`
    /// perturbed along the imaginary axis.
    ///
    /// The oracle for a Jacobian entry, and it is a good one precisely because
    /// it contains no chain rule: the derivative falls out of evaluating the
    /// *primal* function in complex arithmetic, so it cannot repeat a mistake
    /// the derivative pass made. See
    /// [`cfg_complex`](crate::canonical_ir::cfg_complex).
    fn complex_inputs(
        &self,
        node_count: usize,
        branch_count: usize,
        seed: AdSeed,
    ) -> CfgEvalInputs<ComplexStep> {
        let real = self.interpreter_inputs(node_count, branch_count);
        let lift = |values: &[f64]| -> Vec<ComplexStep> {
            values.iter().copied().map(ComplexStep::from_f64).collect()
        };
        let mut node_potentials = lift(&real.node_potentials);
        let mut branch_unknown_flows = lift(&real.branch_unknown_flows);
        match seed {
            AdSeed::NodePotential(node) => {
                if let Some(slot) = node_potentials.get_mut(usize::from(node)) {
                    *slot = ComplexStep::seed(slot.re);
                }
            }
            AdSeed::BranchUnknownFlow(unknown) => {
                if let Some(slot) = branch_unknown_flows.get_mut(usize::from(unknown)) {
                    *slot = ComplexStep::seed(slot.re);
                }
            }
            // Neither is an unknown a difference oracle can displace: a
            // noise process has no bias to perturb, and the limiter
            // correction is a displacement rather than a coordinate. The
            // caller never asks for them.
            AdSeed::NoiseProcess(_) | AdSeed::LimiterCorrection => {}
        }
        CfgEvalInputs {
            parameters: lift(&real.parameters),
            parameter_given: real.parameter_given,
            port_connected: real.port_connected,
            event_state: lift(&real.event_state),
            node_potentials,
            branch_flows: lift(&real.branch_flows),
            branch_unknown_flows,
            temperature: ComplexStep::from_f64(real.temperature),
            thermal_voltage: ComplexStep::from_f64(real.thermal_voltage),
            multiplicity: ComplexStep::from_f64(real.multiplicity),
            time: ComplexStep::from_f64(real.time),
            analyses: real.analyses,
            simparams: std::collections::HashMap::new(),
            ddt: ComplexStep::from_f64(0.0),
            ddt_scale: ComplexStep::from_f64(0.0),
            idt: ComplexStep::from_f64(0.0),
            idt_scale: ComplexStep::from_f64(0.0),
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
        context.branch_unknowns = self.runtime_branch_unknowns.as_ptr();
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

/// The `$analysis` names active at one runtime analysis code.
///
/// The two routes read the same question from different sides: the interpreter
/// is handed the set of active names, while the compiled program compares the
/// context's analysis code. `static` is true for a DC or an initial-condition
/// analysis and `smallsignal` for an AC or a noise one, so the set has to say
/// the same or the two disagree for a reason that is not the lowering.
fn analysis_names(analysis: u8) -> std::collections::HashSet<smol_str::SmolStr> {
    let names: &[&str] = match analysis {
        0 => &["dc", "op", "static"],
        1 => &["ac", "smallsig", "smallsignal", "small_signal"],
        2 => &["tran", "transient"],
        3 => &["noise", "smallsig", "smallsignal", "small_signal"],
        4 => &["ic", "static"],
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

/// The seed list, in the order every consumer of the derivative pass already
/// uses: node potentials, then branch unknowns, then the limiter correction if
/// the model limits at all.
///
/// Read off the generated-Rust backend rather than invented here, because it is
/// the one shipped consumer of CFG-level AD and the lane index means "unknown
/// number n" only if both agree. Its own words for putting the correction last:
/// "a model without `$limit` carries no lane for it and every other lane index
/// still means 'unknown number n'".
fn derivative_seeds(cfg: &CfgModel, mir: &MirModel) -> (Vec<AdSeed>, Option<usize>) {
    let limits = cfg
        .function
        .values
        .iter()
        .any(|value| matches!(value.kind, CfgValueKind::Limit { .. }));
    let seeds: Vec<AdSeed> = (0..mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain((0..mir.branch_unknowns.len()).map(|index| AdSeed::BranchUnknownFlow(index.into())))
        .chain(limits.then_some(AdSeed::LimiterCorrection))
        .collect();
    let correction = limits.then(|| seeds.len() - 1);
    (seeds, correction)
}

/// Which lane each shipped Jacobian entry stands for.
///
/// The inverse of the seed-list construction, and the same arithmetic the
/// generated backend's `emit_row` performs when it splits a lane index back
/// into a node column and a branch column: below the node count it is a node,
/// above it a branch unknown rebased by that count.
fn shipped_entry_lane(axis: &ColumnAxis, node_count: usize) -> usize {
    match axis {
        ColumnAxis::Node(node) => *node,
        ColumnAxis::Branch(branch) => node_count + *branch,
    }
}

/// What the two routes each say exists.
#[derive(Default)]
struct SparsityTally {
    /// Pairs both routes carry.
    shared: usize,
    /// Pairs the CFG route carries and the shipped planner does not.
    ///
    /// Expected, and not a defect: the shipped planner drops an entry whose
    /// differentiated expression *simplifies* to a constant zero, while a lane
    /// that liveness proved reachable stays reachable. The generated backend
    /// keeps the two apart under the same names — `structurally_absent` versus
    /// `folded_to_zero`.
    cfg_only: usize,
    /// Pairs the shipped planner carries and the CFG route does not.
    ///
    /// This is the direction that could be a dropped conductance, so it is not
    /// reported as a count alone: [`SparsityTally::shipped_only_nonzero`] is
    /// what decides whether it is one.
    shipped_only: usize,
    /// Shipped-only pairs whose complex-step derivative is *not* zero at a
    /// drawn bias — the ones that would be a dropped conductance.
    ///
    /// A shipped-only pair by itself is expected and benign. The shipped
    /// planner decides sparsity with a 128-bit structural reachability mask and
    /// then drops whatever its simplifier folds to a literal zero, so an entry
    /// that is identically zero but does not *simplify* to zero survives as a
    /// program that always returns zero. The CFG route's liveness is exact and
    /// drops it. That direction costs the shipped route work and costs the
    /// matrix nothing.
    ///
    /// A shipped-only pair with a nonzero derivative is the other thing
    /// entirely, and this is the count that would say so.
    shipped_only_nonzero: usize,
    /// Shipped-only pairs whose lane index is outside the seed list, so no
    /// oracle can be asked. A nonzero count means the two routes disagree about
    /// the *numbering* of the unknowns, which is worse than disagreeing about
    /// which ones exist.
    shipped_only_unmapped: usize,
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
                            &bindings.branch_unknowns,
                            state_len,
                            cfg.event_state_candidates.len(),
                        )
                    })
                    .collect();

            // Zeroed, and the interpreter's accepted event state is zero too:
            // the only variable slot a CFG-lowered residual reads is the
            // accepted value of an event-controlled state variable, and a
            // static evaluation has no accepted history.
            let variables = vec![0.0_f64; runtime.model.num_variables + 8];

            let mut lowered = 0_usize;
            let mut first_refusal: Option<String> = None;
            let mut oracle_refusal: Option<String> = None;
            let mut executed = 0_usize;
            let mut worst = 0.0_f64;
            let mut worst_case: Option<String> = None;
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
                // The oracle runs on the *pruned* function, one output at a
                // time. Interpreting the whole body instead would report a
                // refusal for the whole model whenever any residual reads an
                // undifferentiated `ddx`, including for the residuals that do
                // not, which is most of them.
                for (index, point) in points.iter_mut().enumerate() {
                    let snapshot = match evaluate_cfg(
                        &pruned,
                        &point.interpreter_inputs(
                            artifact.mir.nodes.len(),
                            artifact.mir.branches.len(),
                        ),
                    ) {
                        Ok(snapshot) => snapshot,
                        Err(error) => {
                            tally.oracle_refusals += 1;
                            oracle_refusal.get_or_insert_with(|| format!("{error:?}"));
                            continue;
                        }
                    };
                    let Some(reference) = snapshot.value(output) else {
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
                    if let Some(delta) = deviation(reference, actual)
                        && delta > worst
                    {
                        worst = delta;
                        worst_case = Some(format!(
                            "residual={ordinal} point={index} interpreter={reference:.17e} block_program={actual:.17e}"
                        ));
                    }
                }
            }
            tally.lowered_outputs += lowered;
            tally.executed_outputs += executed;
            worst_overall = worst_overall.max(worst);
            println!(
                "cfg-census model={module} outputs={} lowered={lowered} executed={executed} max_relative_deviation={worst:.3e} seconds={:.1}{}{}{}",
                cfg.residuals.len(),
                started.elapsed().as_secs_f64(),
                worst_case
                    .map(|case| format!(" worst_case[{case}]"))
                    .unwrap_or_default(),
                oracle_refusal
                    .map(|refusal| format!(" oracle_refused={refusal}"))
                    .unwrap_or_default(),
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

/// How many Jacobian entries per model are compiled and executed.
///
/// The same argument the residual census makes, one level up: a compact model's
/// Jacobian row shares nearly all of its cone with the residual, so machine
/// code for hundreds of entries measures nothing the first few do not.
const EXECUTED_JACOBIAN_ENTRIES_PER_MODEL: usize = 8;

/// How many entries per model are also checked against the complex-step oracle.
///
/// Deliberately small. The oracle re-interprets the whole function in complex
/// arithmetic once per seed, which is the most expensive thing in this census,
/// and it is the *independent* check rather than the exhaustive one: agreement
/// between the block program and the interpreter covers every executed entry,
/// while this covers a sample with something that shares no chain rule with
/// either.
const ORACLE_ENTRIES_PER_MODEL: usize = 2;

/// Below this share of the residual an oracle reading is a structural zero.
///
/// The figure `tests/cfg_derivatives.rs` uses, for its reason: a difference of
/// two numbers that nearly cancel carries no significant figures, and demanding
/// agreement there manufactures failures rather than finding them.
const ORACLE_SIGNIFICANCE: f64 = 1.0e-9;

/// Above this multiple of the residual an oracle reading is a diverged
/// measurement rather than a derivative.
///
/// Also `tests/cfg_derivatives.rs`'s figure and its argument: twelve orders is
/// deliberately generous, because what it excludes is the *perturbed*
/// evaluation overflowing — which reports as 1e197 and is not a statement about
/// the chain rule at all — while a real dropped term shows up as an
/// ordinary-magnitude reading and still fails.
const ORACLE_DIVERGENCE_FACTOR: f64 = 1.0e12;

#[derive(Default)]
struct JacobianTally {
    models: usize,
    differentiation_refusals: usize,
    residual_programs: usize,
    jacobian_programs: usize,
    refused_programs: usize,
    lowered_instructions: usize,
    executed: usize,
    comparisons: usize,
    runtime_errors: usize,
    oracle_comparisons: usize,
    sparsity: SparsityTally,
    /// Models whose shipped equation count and canonical residual count differ,
    /// so no sparsity comparison is meaningful for them.
    sparsity_unpaired: usize,
}

/// Jacobians from CFG-level AD, lowered on the block route and checked against
/// everything that can answer.
///
/// Three questions, and they are different:
///
/// 1. **Totality.** Does every Jacobian entry the derivative pass produces
///    lower onto the block model, and if not, which construct stopped it?
/// 2. **Value.** Does the compiled entry agree with the reference interpreter
///    evaluating the same scalarized function, and with a complex-step
///    derivative of the primal — an oracle containing no chain rule at all?
/// 3. **Sparsity.** Does the set of `(residual, unknown)` pairs match the one
///    the shipped planner enumerates? A pair on one route and not the other is
///    a finding rather than noise, and the two directions mean different
///    things: see [`SparsityTally`].
///
/// Running the derivative pass first is also what makes the `ddx` models
/// evaluable at all. A primal CFG containing an undifferentiated `ddx` is
/// refused by the reference interpreter and by every backend, because the
/// readback has no value until the pass that resolves it has run; ten shipped
/// modules are in that position and none of them appears here as a refusal.
#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_jacobian_route_agrees_with_the_interpreter_and_the_oracles() {
    let root = shipped_model_root();
    let candidates = discover_veriloga_sources(&root).expect("discover shipped Verilog-A sources");
    let mut tally = JacobianTally::default();
    let mut worst_overall = 0.0_f64;
    let mut worst_oracle_overall = 0.0_f64;
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
            let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
                println!("cfg-jacobian model={module} refused=cfg-lowering");
                continue;
            };
            let Ok(state) = CfgStateAllocation::build(&artifact.hir, &cfg.function) else {
                println!("cfg-jacobian model={module} refused=state-allocation");
                continue;
            };

            let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
            let mut differentiated = match differentiate(&cfg.function, &seeds) {
                Ok(differentiated) => differentiated,
                Err(error) => {
                    tally.differentiation_refusals += 1;
                    println!("cfg-jacobian model={module} refused=differentiate detail={error:?}");
                    continue;
                }
            };
            // Every read-out before anything evaluates or lowers: taking one
            // appends an instruction, so a function captured earlier would not
            // contain the later ones.
            let rows: Vec<Vec<Option<ValueId>>> = cfg
                .residuals
                .iter()
                .map(|residual| differentiated.derivative_row(*residual))
                .collect();
            let scalarized = match scalarize_lanes(&differentiated.function) {
                Ok(scalarized) => scalarized,
                Err(error) => {
                    println!("cfg-jacobian model={module} refused=scalarize detail={error}");
                    continue;
                }
            };

            let branch_unknowns =
                canonical_branch_unknown_runtime_map(&runtime.model, &artifact.mir)
                    .unwrap_or_else(|error| panic!("{module}: branch unknown map: {error}"));
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
                            &bindings.branch_unknowns,
                            state_len,
                            cfg.event_state_candidates.len(),
                        )
                    })
                    .collect();
            let variables = vec![0.0_f64; runtime.model.num_variables + 8];

            // ---- sparsity ---------------------------------------------------
            let node_count = artifact.mir.nodes.len();
            let mut cfg_pairs: HashSet<(usize, usize)> = HashSet::new();
            for (equation, row) in rows.iter().enumerate() {
                for (lane, entry) in row.iter().enumerate() {
                    if entry.is_some() && Some(lane) != correction_lane {
                        cfg_pairs.insert((equation, lane));
                    }
                }
            }
            let mut shipped_pairs: HashSet<(usize, usize)> = HashSet::new();
            for (equation, program) in runtime.model.stamp_programs.iter().enumerate() {
                for entry in &program.jacobian_programs {
                    shipped_pairs
                        .insert((equation, shipped_entry_lane(&entry.col_axis, node_count)));
                }
            }
            let paired = runtime.model.stamp_programs.len() == cfg.residuals.len();
            let (shared, cfg_only, shipped_only) = if paired {
                (
                    cfg_pairs.intersection(&shipped_pairs).count(),
                    cfg_pairs.difference(&shipped_pairs).count(),
                    shipped_pairs.difference(&cfg_pairs).count(),
                )
            } else {
                tally.sparsity_unpaired += 1;
                (0, 0, 0)
            };
            tally.sparsity.shared += shared;
            tally.sparsity.cfg_only += cfg_only;
            tally.sparsity.shipped_only += shipped_only;

            // Every shipped-only pair gets asked the one question that decides
            // whether it is a finding: is the derivative actually zero there?
            // The oracle is complex step on the primal, which shares no chain
            // rule with either route's sparsity analysis.
            let mut shipped_only_worst = 0.0_f64;
            let mut shipped_only_case: Option<String> = None;
            if paired && shipped_only > 0 {
                let mut missing: Vec<(usize, usize)> =
                    shipped_pairs.difference(&cfg_pairs).copied().collect();
                // By lane, not by pair: a complex-step evaluation perturbs one
                // unknown and answers for *every* residual at once, so grouping
                // turns one interpretation per missing entry into one per
                // distinct unknown. On a compact model that is the difference
                // between a few dozen evaluations and a few.
                missing.sort_unstable_by_key(|(equation, lane)| (*lane, *equation));
                let mut current: Option<(usize, CfgEvalSnapshot<ComplexStep>)> = None;
                for (equation, lane) in missing {
                    let Some(seed) = seeds.get(lane).copied() else {
                        tally.sparsity.shipped_only_unmapped += 1;
                        continue;
                    };
                    if current.as_ref().is_none_or(|(held, _)| *held != lane) {
                        let complex = points[0].complex_inputs(
                            artifact.mir.nodes.len(),
                            artifact.mir.branches.len(),
                            seed,
                        );
                        current = evaluate_cfg(&differentiated.function, &complex)
                            .ok()
                            .map(|snapshot| (lane, snapshot));
                    }
                    let Some((_, snapshot)) = current.as_ref() else {
                        continue;
                    };
                    let Some(value) = snapshot.value(cfg.residuals[equation]) else {
                        continue;
                    };
                    let derivative = value.derivative();
                    // Scaled against the residual it belongs to: an absolute
                    // threshold would call a picoamp model's whole row zero and
                    // an ampere model's rounding nonzero.
                    let scale = value.real().abs().max(f64::MIN_POSITIVE);
                    // Bounded on both sides, and the upper bound is not
                    // slack. `tests/cfg_derivatives.rs` carries the same
                    // ceiling under the same name for the same reason: a
                    // reading twelve orders above the residual it belongs to is
                    // the *perturbed evaluation* overflowing, not a statement
                    // about a derivative. A genuinely dropped conductance shows
                    // up as an ordinary-magnitude reading against a structural
                    // zero, and that still counts.
                    if derivative.abs() > scale * ORACLE_SIGNIFICANCE
                        && derivative.abs() < scale * ORACLE_DIVERGENCE_FACTOR
                    {
                        tally.sparsity.shipped_only_nonzero += 1;
                        if derivative.abs() > shipped_only_worst {
                            shipped_only_worst = derivative.abs();
                            shipped_only_case = Some(format!(
                                "d(equation {equation})/d(lane {lane})={derivative:.6e} \
                                 residual={:.6e}",
                                value.real()
                            ));
                        }
                    }
                }
            }

            // ---- lowering, execution and comparison -------------------------
            let mut residual_programs = 0_usize;
            let mut jacobian_programs = 0_usize;
            let mut refused = 0_usize;
            let mut instructions = 0_usize;
            let mut executed = 0_usize;
            let mut oracle_checks = 0_usize;
            let mut worst = 0.0_f64;
            let mut worst_case: Option<String> = None;
            let mut worst_oracle = 0.0_f64;
            let mut oracle_case: Option<String> = None;
            let mut first_refusal: Option<String> = None;

            for (equation, residual) in cfg.residuals.iter().copied().enumerate() {
                // The primal first, then its row: an entry shares nearly all of
                // the residual's cone, which is what makes the per-entry program
                // count the cost this census exists to measure.
                let mut outputs: Vec<(Option<usize>, ValueId)> = Vec::new();
                if let Some(primal) = scalarized.scalar(residual) {
                    outputs.push((None, primal));
                }
                for (lane, entry) in rows[equation].iter().enumerate() {
                    if Some(lane) == correction_lane {
                        continue;
                    }
                    if let Some(entry) = entry
                        && let Some(scalar) = scalarized.scalar(*entry)
                    {
                        outputs.push((Some(lane), scalar));
                    }
                }

                // Sliced to the whole row once, then to each entry within it.
                // Pruning is linear in the function, so slicing the *model* per
                // entry would be the equation count times the lane count passes
                // over a compact model's whole body; slicing the row first
                // makes every inner pass linear in the row instead. The result
                // is the same function: pruning is idempotent, and composing
                // two slices keeps exactly what the inner one asked for.
                let row_outputs: Vec<ValueId> = outputs.iter().map(|(_, output)| *output).collect();
                let (row_function, row_mapped) =
                    prune_cfg_to_outputs(&scalarized.function, &row_outputs);
                let outputs: Vec<(Option<usize>, ValueId)> = outputs
                    .into_iter()
                    .zip(row_mapped)
                    .map(|((lane, _), mapped)| (lane, mapped))
                    .collect();

                for (lane, output) in outputs {
                    let (pruned, pruned_outputs) = prune_cfg_to_outputs(&row_function, &[output]);
                    let program =
                        match lower_cfg_function(&pruned, pruned_outputs[0], &state, &bindings) {
                            Ok(program) => program,
                            Err(error) => {
                                refused += 1;
                                first_refusal.get_or_insert_with(|| error.to_string());
                                continue;
                            }
                        };
                    if lane.is_some() {
                        jacobian_programs += 1;
                    } else {
                        residual_programs += 1;
                    }
                    instructions += program.instructions().len();

                    let Some(lane) = lane else { continue };
                    if executed >= EXECUTED_JACOBIAN_ENTRIES_PER_MODEL {
                        continue;
                    }
                    let image = compile_value_function_artifact_from_ssa(&program)
                        .unwrap_or_else(|error| panic!("{module} d{equation}/d{lane}: {error}"));
                    let memory = ExecutableMemory::allocate(image.bytes())
                        .unwrap_or_else(|error| panic!("{module} d{equation}/d{lane}: {error}"));
                    let entry_point = memory.ptr_at(0).expect("entry inside published image");
                    let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                        unsafe { std::mem::transmute(entry_point) };
                    executed += 1;

                    for (index, point) in points.iter_mut().enumerate() {
                        let interpreter_inputs = point.interpreter_inputs(
                            artifact.mir.nodes.len(),
                            artifact.mir.branches.len(),
                        );
                        let Ok(snapshot) = evaluate_cfg(&pruned, &interpreter_inputs) else {
                            continue;
                        };
                        let Some(reference) = snapshot.value(pruned_outputs[0]) else {
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
                        if let Some(delta) = deviation(reference, actual)
                            && delta > worst
                        {
                            worst = delta;
                            worst_case = Some(format!(
                                "d(equation {equation})/d(lane {lane}) point={index} \
                                 interpreter={reference:.17e} block_program={actual:.17e}"
                            ));
                        }

                        // The independent oracle, on the first operating point
                        // only and for a bounded number of entries.
                        if index == 0 && oracle_checks < ORACLE_ENTRIES_PER_MODEL {
                            oracle_checks += 1;
                            let complex = point.complex_inputs(
                                artifact.mir.nodes.len(),
                                artifact.mir.branches.len(),
                                seeds[lane],
                            );
                            if let Ok(snapshot) = evaluate_cfg(&differentiated.function, &complex)
                                && let Some(value) = snapshot.value(residual)
                            {
                                let oracle = value.derivative();
                                tally.oracle_comparisons += 1;
                                // A near-cancelling entry carries no significant
                                // figures, and demanding agreement there
                                // manufactures failures rather than finding them.
                                let scale = oracle.abs().max(actual.abs());
                                if scale > 1.0e-30
                                    && let Some(delta) = deviation(oracle, actual)
                                    && delta > worst_oracle
                                {
                                    worst_oracle = delta;
                                    oracle_case = Some(format!(
                                        "d(equation {equation})/d(lane {lane}) \
                                         complex_step={oracle:.17e} block_program={actual:.17e}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }

            tally.residual_programs += residual_programs;
            tally.jacobian_programs += jacobian_programs;
            tally.refused_programs += refused;
            tally.lowered_instructions += instructions;
            tally.executed += executed;
            worst_overall = worst_overall.max(worst);
            worst_oracle_overall = worst_oracle_overall.max(worst_oracle);
            println!(
                "cfg-jacobian model={module} equations={} lanes={} residual_programs={residual_programs} \
                 jacobian_programs={jacobian_programs} refused={refused} instructions={instructions} \
                 executed={executed} max_relative_deviation={worst:.3e} oracle_deviation={worst_oracle:.3e} \
                 sparsity[shared={shared} cfg_only={cfg_only} shipped_only={shipped_only}{}] seconds={:.1}{}{}{}{}",
                cfg.residuals.len(),
                seeds.len(),
                if paired { "" } else { " UNPAIRED" },
                started.elapsed().as_secs_f64(),
                shipped_only_case
                    .map(|case| format!(" SHIPPED_ONLY_NONZERO[{case}]"))
                    .unwrap_or_default(),
                worst_case
                    .map(|case| format!(" worst_case[{case}]"))
                    .unwrap_or_default(),
                oracle_case
                    .map(|case| format!(" oracle_case[{case}]"))
                    .unwrap_or_default(),
                first_refusal
                    .map(|refusal| format!(" first_refusal={refusal}"))
                    .unwrap_or_default(),
            );
        }
    }
    println!(
        "cfg-jacobian models={} differentiation_refusals={} residual_programs={} jacobian_programs={} \
         refused_programs={} lowered_instructions={} executed={} comparisons={} runtime_errors={} \
         oracle_comparisons={} sparsity[shared={} cfg_only={} shipped_only={} \
         shipped_only_nonzero={} shipped_only_unmapped={} unpaired_models={}] \
         max_relative_deviation={worst_overall:.3e} max_oracle_deviation={worst_oracle_overall:.3e}",
        tally.models,
        tally.differentiation_refusals,
        tally.residual_programs,
        tally.jacobian_programs,
        tally.refused_programs,
        tally.lowered_instructions,
        tally.executed,
        tally.comparisons,
        tally.runtime_errors,
        tally.oracle_comparisons,
        tally.sparsity.shared,
        tally.sparsity.cfg_only,
        tally.sparsity.shipped_only,
        tally.sparsity.shipped_only_nonzero,
        tally.sparsity.shipped_only_unmapped,
        tally.sparsity_unpaired,
    );
    assert_eq!(
        tally.sparsity.shipped_only_nonzero, 0,
        "the CFG route drops a Jacobian entry the shipped route stamps and the primal's own \
         complex-step derivative says is nonzero"
    );
    if filter.is_none() {
        assert_eq!(tally.models, 43, "the shipped census is 43 modules");
    }
    assert!(
        tally.comparisons > 0,
        "the census must actually execute the lowered Jacobian programs"
    );
}

/// Which shipped modules the two state-slot numberings disagree about.
///
/// [`CfgStateAllocation`] numbers analog-operator records per *site*; the
/// bytecode generator numbers them per *emission*, and a module with noise in
/// an assignment is emitted twice, so one canonical `ddt` can own two slots.
/// `CfgStateAllocation`'s own documentation names the two spaces and says the
/// second is not proven to agree; this measures how many modules actually
/// differ and by how many slots, which is what decides whether moving the JIT
/// runtime to per-site numbering is a shipped-behaviour change at all.
///
/// The measurement is direct rather than inferred:
/// [`NativeRequiredStorage::for_model`] reports how many integration slots the
/// *compiled* model addresses — the per-emission count, read off the emitted
/// instruction stream — against
/// [`CfgStateAllocation::family_len`]'s per-site count for the same family.
/// `agrees_with_emission_allocation` is reported alongside, because it is the
/// sufficient condition callers actually consult and a divergence between the
/// two would mean that predicate is answering the wrong question.
#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_two_state_slot_numberings_are_censused_over_the_shipped_corpus() {
    let root = shipped_model_root();
    let candidates = discover_veriloga_sources(&root).expect("discover shipped Verilog-A sources");
    let mut models = 0_usize;
    let mut with_state = 0_usize;
    let mut with_noise = 0_usize;
    let mut disagreeing_predicate = 0_usize;
    let mut differing_counts = 0_usize;
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    for candidate in candidates {
        for module in &candidate.modules {
            if filter
                .as_deref()
                .is_some_and(|filter| !module.contains(filter))
            {
                continue;
            }
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
            models += 1;
            let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
                continue;
            };
            let Ok(state) = CfgStateAllocation::build(&artifact.hir, &cfg.function) else {
                continue;
            };
            let per_site = state.family_len(CanonicalStateFamily::Integration);
            let per_emission = NativeRequiredStorage::for_model(&runtime.model).state_values;
            let agrees = state.agrees_with_emission_allocation(&artifact.hir);
            let noise = !runtime.model.noise_sources.is_empty();
            if per_site > 0 {
                with_state += 1;
            }
            if noise {
                with_noise += 1;
            }
            if per_site != per_emission {
                differing_counts += 1;
            }
            // `agrees_with_emission_allocation` is documented as a sufficient
            // condition for the two spaces coinciding, so this combination —
            // the predicate saying they agree while the counts differ — is the
            // one that says the condition is not sufficient. It is not zero:
            // see the census's own documentation.
            if agrees && per_site != per_emission {
                disagreeing_predicate += 1;
            }
            if per_site != per_emission || (!agrees && noise) {
                println!(
                    "state-slots model={module} per_site={per_site} per_emission={per_emission} \
                     agrees_with_emission_allocation={agrees} noise_sources={}",
                    runtime.model.noise_sources.len()
                );
            }
        }
    }
    println!(
        "state-slots models={models} with_integration_state={with_state} with_noise={with_noise} \
         differing_counts={differing_counts} predicate_wrong={disagreeing_predicate}"
    );
    // Deliberately not an equality assertion on the counts. This is a census:
    // what it establishes is *that* the two spaces differ and on which modules,
    // and pinning a number here would freeze a measurement of the bytecode
    // generator that this lane does not own. What it does assert is the shape
    // of the finding, so that a change making the two numberings agree — or
    // making the predicate detect the difference — fails here and is read
    // rather than absorbed.
    assert!(
        differing_counts > 0 && disagreeing_predicate > 0,
        "the two state-slot numberings now agree, or the predicate now detects where they do \
         not; either is a change to the ruling recorded on CfgStateAllocation and wants reading"
    );
}

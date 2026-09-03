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

use super::census_models::shipped_census_models_matching;
use crate::canonical_ir::cfg_lower::{CfgModel, CfgNoiseProcess};
use crate::canonical_ir::{
    AdFunction, AdSeed, CanonicalNoiseSourceKind, CanonicalStateFamily, CanonicalStateLayout,
    CfgEvalInputs, CfgEvalSnapshot, CfgFunction, CfgScalar, CfgStateAllocation, ComplexStep,
    EmissionCensus, ValueId, differentiate, evaluate_cfg, prune_cfg_to_outputs,
};
use crate::codegen::{AssignmentStep, BytecodeProgram, CompiledModel, Instruction};
use crate::jit::cfg_lanes::{ScalarLanes, scalarize_lanes};
use crate::jit::cfg_plan_builder::{ShippedColumnLanes, derivative_seeds};
use crate::jit::cfg_program::{CfgRuntimeBindings, lower_cfg_function};
use crate::jit::expr::BranchUnknownRuntimeMapping;
use crate::jit::plan_builder::canonical_branch_unknown_runtime_map;
use crate::native::abi::EvalContext;
use crate::native::model::NativeRequiredStorage;
use crate::native::runtime::ExecutableMemory;
use crate::native::x64::codegen::compile_value_function_artifact_from_ssa;
use crate::rust_backend::canonical::{noise_plan_decline, stored_charges};

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
pub(super) struct OperatingPoint {
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
    /// Whether `@(initial_step)` is active here. See
    /// [`OperatingPoint::with_initial_step`].
    initial_step: bool,
}

const BOLTZMANN_OVER_ELECTRON: f64 = 1.380_649e-23 / 1.602_176_634e-19;

impl OperatingPoint {
    pub(super) fn new(
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
        // Slack past the canonical count, because the *runtime* branch-source
        // array is numbered by the compiled model rather than by the canonical
        // map, and this array has no length field in the context for a
        // compiled program to be bounds-checked against. Trailing zeros change
        // no indexed read; running off the end would decide whether the census
        // process survives.
        let mut runtime_flows = vec![0.0; branch_unknowns.len() + 64];
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
            initial_step: false,
        }
    }

    /// Evaluate here with `@(initial_step)` active.
    ///
    /// # Why a point without it is not a point at all for some models
    ///
    /// A compact model that maps its parameters to temperature does that work
    /// once, in an initial block, and the body then divides by what the block
    /// wrote. VBIC writes `tiniK = TABS + tnom` under `@(initial_step)` and the
    /// body computes `rT = tdevK / tiniK`; with the block skipped `tiniK` is the
    /// zero its slot was allocated with, `rT` is infinite, and
    /// `exp(-dear * (1 - rT) / …)` with VBIC's default `dear = 0` is `exp(0 *
    /// -inf)` — a NaN that reaches 347 of the module's 1052 variables.
    ///
    /// Nothing about that measures a route. Both plans read the one variable
    /// array the shipped assignment pass fills, so a poisoned array is poisoned
    /// for both, and every entry that loads one of those variables compares two
    /// readings of the same NaN. It also hides the entries that *would* differ:
    /// `deviation` cannot separate two NaNs, so they are counted as agreeing.
    ///
    /// Production never evaluates in this state — the first load of a device
    /// runs its initial step — so this is not a wider operating envelope than
    /// the model was written for. It is the one the model was written for.
    pub(super) fn with_initial_step(mut self) -> Self {
        self.initial_step = true;
        self
    }

    /// How many event-controlled state slots the interpreter will be asked for.
    ///
    /// Set after construction because the count is a property of the *CFG*, and
    /// a census that only drives compiled plans never builds one. The
    /// interpreter refuses a slot it was not given, so a route that does build
    /// one has to say how many there are.
    pub(super) fn set_event_state_slots(&mut self, slots: usize) {
        self.event_state_slots = slots;
    }

    pub(super) fn interpreter_inputs(
        &self,
        node_count: usize,
        branch_count: usize,
    ) -> CfgEvalInputs<f64> {
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
            analyses: analysis_names(self.analysis, self.initial_step),
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

    pub(super) fn context(&mut self) -> EvalContext {
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
        context.analysis_initial_step = u8::from(self.initial_step);
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
///
/// # `@(initial_step)` is one of those names
///
/// The front end lowers `@(initial_step)` to `$analysis("__rspice_initial_step")`
/// — see [`crate::canonical_ir::cfg_lower`] and
/// [`crate::device`], which inserts the same name — and the compiled program
/// reads it off `EvalContext::analysis_initial_step`. Building the set from the
/// analysis *code* alone therefore leaves an interpreted CFG evaluating it false
/// while a compiled plan at the same operating point runs with the flag set,
/// which is a disagreement about the point rather than about the route.
///
/// It did not show while the only interpreted census (`cfg_census`) never asked
/// for an initial step. Any mixed comparison does, so the flag belongs here
/// rather than in each caller.
fn analysis_names(
    analysis: u8,
    initial_step: bool,
) -> std::collections::HashSet<smol_str::SmolStr> {
    let names: &[&str] = match analysis {
        0 => &["dc", "op", "static"],
        1 => &["ac", "smallsig", "smallsignal", "small_signal"],
        2 => &["tran", "transient"],
        3 => &["noise", "smallsig", "smallsignal", "small_signal"],
        4 => &["ic", "static"],
        _ => &[],
    };
    let mut active: std::collections::HashSet<smol_str::SmolStr> =
        names.iter().map(|name| (*name).into()).collect();
    if initial_step {
        active.insert(smol_str::SmolStr::new("__rspice_initial_step"));
    }
    active
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
pub(super) fn deviation(expected: f64, actual: f64) -> Option<f64> {
    if expected.is_nan() && actual.is_nan() {
        return None;
    }
    if expected == actual {
        return None;
    }
    let scale = expected.abs().max(actual.abs()).max(f64::MIN_POSITIVE);
    Some((expected - actual).abs() / scale)
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

/// Modules whose residuals are allowed to deviate from the reference
/// interpreter, and by how much.
///
/// Empty, and that is the measurement rather than an aspiration: the block
/// lowering reproduces the interpreter *exactly* — bit for bit, at every
/// operating point, on all forty-three shipped modules. The one module that
/// ever deviated was `vbic_4T_et_cf`, and the cause was not the lowering: the
/// x64 backend split `limexp` at 40 while the interpreter split it at 80. W-F3a
/// (`84ba2c2bb`) ruled that threshold once for the whole estate and the
/// deviation went to zero.
///
/// # Checked in both directions
///
/// A module listed here that comes back at zero fails the gate until it is
/// removed. An allowlist that only ever grows is a record of what someone once
/// tolerated; one that has to shrink when the defect is fixed is a statement
/// about the tree as it is now.
const RESIDUAL_DEVIATION_ALLOWLIST: &[(&str, f64)] = &[];

#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_block_lowering_agrees_with_the_reference_interpreter() {
    let mut tally = Tally::default();
    let mut worst_overall = 0.0_f64;
    let mut per_module: Vec<(String, f64)> = Vec::new();
    // The same affordance the emitted-code benchmark carries: a substring that
    // narrows repeated runs to one model while it is being investigated. The
    // corpus assertion below is skipped whenever it is set, so a filtered run
    // cannot be mistaken for a full one. It narrows the shared provider rather
    // than its output, so a filtered run does not compile the models it skips.
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    let mut total_compile_seconds = 0.0_f64;
    let mut total_census_seconds = 0.0_f64;
    // Declared once so every exit from a model's body reports the same split,
    // including the two that refuse the model before it is compared.
    let mut note_split = |module: &str, compile_seconds: f64, census_seconds: f64, cached: bool| {
        println!(
            "cfg-census model={module} compile_seconds={compile_seconds:.1} census_seconds={census_seconds:.1} cached={cached}"
        );
        total_compile_seconds += compile_seconds;
        total_census_seconds += census_seconds;
    };
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = &shipped.name;
        let started = std::time::Instant::now();
        let runtime = &shipped;
        let artifact = &runtime.canonical_ir;
        tally.models += 1;
        let cfg = match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
            Ok(cfg) => cfg,
            Err(diagnostics) => {
                println!(
                    "cfg-census model={module} refused=cfg-lowering detail={}",
                    diagnostics
                        .first()
                        .map_or_else(|| "unknown".to_string(), |first| first.message.to_string())
                );
                note_split(
                    module,
                    shipped.compile_seconds,
                    started.elapsed().as_secs_f64(),
                    shipped.from_cache,
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
                note_split(
                    module,
                    shipped.compile_seconds,
                    started.elapsed().as_secs_f64(),
                    shipped.from_cache,
                );
                continue;
            }
        };
        let branch_unknowns = canonical_branch_unknown_runtime_map(&runtime.model, &artifact.mir)
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
            let artifact_image =
                compile_value_function_artifact_from_ssa(&program).unwrap_or_else(|error| {
                    panic!("{module} residual {ordinal}: x64 codegen: {error}")
                });
            let memory = ExecutableMemory::allocate(artifact_image.bytes())
                .unwrap_or_else(|error| panic!("{module} residual {ordinal}: publish: {error}"));
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
                    &point
                        .interpreter_inputs(artifact.mir.nodes.len(), artifact.mir.branches.len()),
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
        per_module.push((module.clone(), worst));
        println!(
            "cfg-census model={module} outputs={} lowered={lowered} executed={executed} max_relative_deviation={worst:.3e} seconds={:.1}{}{}{}",
            cfg.residuals.len(),
            shipped.compile_seconds + started.elapsed().as_secs_f64(),
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
        note_split(
            module,
            shipped.compile_seconds,
            started.elapsed().as_secs_f64(),
            shipped.from_cache,
        );
    }
    println!(
        "cfg-census total_compile_seconds={total_compile_seconds:.1} total_census_seconds={total_census_seconds:.1}"
    );
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

    let mut findings = Vec::new();
    for (module, worst) in &per_module {
        let allowed = RESIDUAL_DEVIATION_ALLOWLIST
            .iter()
            .find(|(name, _)| name == module)
            .map(|(_, allowed)| *allowed);
        match allowed {
            None if *worst != 0.0 => findings.push(format!(
                "{module} deviates from the reference interpreter by {worst:.3e}; the module's \
                 own line above names the residual, the point and both readings"
            )),
            Some(allowed) if *worst == 0.0 => findings.push(format!(
                "{module} is on RESIDUAL_DEVIATION_ALLOWLIST at {allowed:.3e} and now deviates by \
                 nothing; remove it from the list"
            )),
            Some(allowed) if *worst > allowed => findings.push(format!(
                "{module} deviates by {worst:.3e}, past the {allowed:.3e} it is allowed"
            )),
            _ => {}
        }
    }
    // A listed module the run never reached is the third direction: it would
    // leave an entry in the list that nothing can ever clear. Only a full run
    // can say so — a filtered one skips modules on purpose.
    if filter.is_none() {
        for (module, _) in RESIDUAL_DEVIATION_ALLOWLIST {
            assert!(
                per_module.iter().any(|(name, _)| name == module),
                "{module} is on RESIDUAL_DEVIATION_ALLOWLIST but the census never reached it"
            );
        }
    }
    assert!(
        findings.is_empty(),
        "the block lowering no longer reproduces the reference interpreter exactly:\n{}",
        findings.join("\n")
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
    let mut tally = JacobianTally::default();
    let mut worst_overall = 0.0_f64;
    let mut worst_oracle_overall = 0.0_f64;
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = &shipped.name;
        let started = std::time::Instant::now();
        let runtime = &shipped;
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

        let branch_unknowns = canonical_branch_unknown_runtime_map(&runtime.model, &artifact.mir)
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
        let column_lanes = ShippedColumnLanes::build(&runtime.model, &artifact.mir)
            .unwrap_or_else(|error| panic!("{module}: shipped column lanes: {error}"));
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
                if let Some(lane) = column_lanes.lane(&entry.col_axis) {
                    shipped_pairs.insert((equation, lane));
                }
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
                    let interpreter_inputs = point
                        .interpreter_inputs(artifact.mir.nodes.len(), artifact.mir.branches.len());
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

/// Which context of the bytecode generator emitted each integration state
/// slot, in the order `generate_from_ir` compiles them.
///
/// The generator's slot allocator is one monotonic counter, so the emission
/// numbering *is* this sequence. Recovering it context by context — rather than
/// reading the largest slot the model addresses — is what turns "the two
/// numberings differ by ten slots" into "the extra ten are the noise-assignment
/// replay and they sit in front of every contribution's slot", which is the
/// difference between W-F's move being a shrink and being a renumbering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmissionContext {
    /// A parameter default, bound, or exclude program. Compiled first, and
    /// therefore able to displace everything after it.
    Parameter,
    /// `ir.assignments`: the module's statements plus the derivative shadow
    /// assignments the front end appends to them.
    Assignment,
    /// `ir.noise_assignments`, the clone carrying noise shadows.
    NoiseAssignment,
    /// An equation's own value program or peeled static condition — the
    /// contribution-borne sites, at the position the per-site layout gives
    /// them.
    EquationPrimal,
    /// An equation's resistive or reactive derivative program. A re-emission:
    /// `d(l*r) = dl*r + l*dr` keeps the primal factors.
    EquationDerivative,
    /// A noise source's PSD, exponent, or injection-gain program.
    NoiseSource,
}

/// Which operator took an integration slot.
///
/// The five spellings that share `limit_state_count`. Reported per context
/// because it is what names the mechanism behind a re-emission: a `ddt`
/// reappearing in a derivative program is the product rule keeping a primal
/// factor, while a `$limit` reappearing there would be something else.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmittedOperator {
    Ddt,
    Idt,
    IdtMod,
    Limit,
    CanonicalLimit,
}

/// One slot the generator's integration counter handed out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Emission {
    context: EmissionContext,
    operator: EmittedOperator,
}

/// Which integration-family slots one bytecode program allocates, in order.
fn integration_emissions(program: &BytecodeProgram) -> impl Iterator<Item = EmittedOperator> + '_ {
    program
        .instructions
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::DdtState(_) => Some(EmittedOperator::Ddt),
            Instruction::IdtState(_) => Some(EmittedOperator::Idt),
            Instruction::IdtModState(_) => Some(EmittedOperator::IdtMod),
            Instruction::LimitState(_) => Some(EmittedOperator::Limit),
            Instruction::CanonicalLimitState(_) => Some(EmittedOperator::CanonicalLimit),
            _ => None,
        })
}

fn tag_program(tags: &mut Vec<Emission>, program: &BytecodeProgram, context: EmissionContext) {
    tags.extend(integration_emissions(program).map(|operator| Emission { context, operator }));
}

fn tag_assignment_steps(
    tags: &mut Vec<Emission>,
    steps: &[AssignmentStep],
    context: EmissionContext,
) {
    for step in steps {
        match step {
            AssignmentStep::Assign(assignment) => tag_program(tags, &assignment.program, context),
            // The *value* first: `compile_assignment_items` binds the value
            // program before it compiles the index expression.
            AssignmentStep::AssignIndexed { index, value, .. } => {
                tag_program(tags, value, context);
                tag_program(tags, index, context);
            }
            AssignmentStep::Loop { condition, body } => {
                tag_program(tags, condition, context);
                tag_assignment_steps(tags, body, context);
            }
        }
    }
}

/// Every integration-family slot the compiled model allocates, tagged with the
/// context that allocated it, in generator order.
///
/// Mirrors `CodeGenerator::generate_from_ir` exactly, including the ordering
/// detail that decides the answer: a current contribution's
/// `jacobian_programs` holds **two entries per compiled derivative** — the
/// positive and negative KCL rows share one `compile_expr` result by `clone()`
/// — so scanning both would count every re-emission twice.
pub(super) fn integration_emission_contexts(model: &CompiledModel) -> Vec<Emission> {
    let mut tags = Vec::new();

    for parameter in &model.parameters {
        for program in parameter
            .default_program
            .iter()
            .chain(parameter.min_program.iter())
            .chain(parameter.max_program.iter())
            .chain(parameter.exclude_programs.iter())
        {
            tag_program(&mut tags, program, EmissionContext::Parameter);
        }
    }

    tag_assignment_steps(
        &mut tags,
        &model.assignment_steps,
        EmissionContext::Assignment,
    );
    tag_assignment_steps(
        &mut tags,
        &model.noise_assignment_steps,
        EmissionContext::NoiseAssignment,
    );

    for stamp in &model.stamp_programs {
        tag_program(
            &mut tags,
            &stamp.value_program,
            EmissionContext::EquationPrimal,
        );
        if let Some(condition) = &stamp.static_condition {
            tag_program(&mut tags, condition, EmissionContext::EquationPrimal);
        }
        // One `compile_expr` per *pair* of entries on a current contribution,
        // one per entry on a branch row.
        let stride = if stamp.branch_ordinal.is_none() { 2 } else { 1 };
        for entry in stamp.jacobian_programs.iter().step_by(stride) {
            tag_program(
                &mut tags,
                &entry.program,
                EmissionContext::EquationDerivative,
            );
        }
        for entry in stamp.reactive_jacobians.iter().step_by(stride) {
            tag_program(
                &mut tags,
                &entry.program,
                EmissionContext::EquationDerivative,
            );
        }
    }

    for source in &model.noise_sources {
        tag_program(&mut tags, &source.psd_program, EmissionContext::NoiseSource);
        if let Some(program) = &source.exponent_program {
            tag_program(&mut tags, program, EmissionContext::NoiseSource);
        }
        for injection in &source.injections {
            tag_program(
                &mut tags,
                &injection.gain_program,
                EmissionContext::NoiseSource,
            );
        }
    }

    tags
}

/// What the emission sequence does to the per-site numbering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PrefixShape {
    /// The two numberings are the same length and the same order.
    Identical,
    /// Slots `0..per_site` mean the same thing under both numberings and the
    /// extra emissions sit after them. Adopting the per-site numbering shrinks
    /// the state vector and moves nothing.
    Append,
    /// A re-emission sits in front of a site's own slot, so adopting the
    /// per-site numbering permutes the array. This is the case that moves
    /// `RUNTIME_CHECKPOINT_STATE_VERSION`.
    Interleave,
}

/// Whether the first `statements + contributions` emissions are exactly the
/// per-site sequence.
///
/// The per-site numbering walks statements then contributions; the generator
/// walks parameters, assignments, the noise-assignment clone, then each
/// equation's value program followed immediately by that equation's own
/// derivatives. The prefix therefore survives exactly when the first
/// `statements` emissions all come from the assignment pass and the next
/// `contributions` all come from an equation's own value program — which is
/// what this checks, tag by tag, rather than inferring it from the counts.
pub(super) fn prefix_shape(
    tags: &[Emission],
    statements: usize,
    contributions: usize,
) -> PrefixShape {
    let sites = statements + contributions;
    let prefix_holds = tags.len() >= sites
        && tags[..statements]
            .iter()
            .all(|emission| emission.context == EmissionContext::Assignment)
        && tags[statements..sites]
            .iter()
            .all(|emission| emission.context == EmissionContext::EquationPrimal);
    match (prefix_holds, tags.len() == sites) {
        (true, true) => PrefixShape::Identical,
        (true, false) => PrefixShape::Append,
        (false, _) => PrefixShape::Interleave,
    }
}

/// Which shipped modules the two state-slot numberings disagree about, why, and
/// whether the difference is a shrink or a renumbering.
///
/// [`CfgStateAllocation`] numbers analog-operator records per *site*; the
/// bytecode generator numbers them per *emission*, with one monotonic counter
/// running across every context `generate_from_ir` compiles. This census reads
/// that counter's sequence back off the compiled model
/// ([`integration_emission_contexts`]) and answers three questions per module:
///
/// 1. **How many.** The per-site count from [`CfgStateAllocation::family_len`]
///    against the emission count, and against
///    [`NativeRequiredStorage::for_model`]'s figure — which scans a *subset* of
///    the contexts, deliberately, because it sizes storage for the programs the
///    native plan lowers and the plan lowers no noise-assignment step and no
///    injection gain. The two figures differing is expected; the census reports
///    both so that a change to either is read rather than absorbed.
/// 2. **Where from.** The per-context breakdown, which is what says whether the
///    extra slots are the noise-assignment replay, the derivative programs, or
///    the PSD programs.
/// 3. **Append or interleave.** [`prefix_shape`], which is what decides whether
///    W-F's move is a shrink or a renumbering that moves
///    `RUNTIME_CHECKPOINT_STATE_VERSION`.
///
/// [`CfgStateAllocation::agrees_with_emission_allocation`] is checked against
/// the measurement on every module: a `true` that does not coincide is an
/// unsound licence and fails the test.
#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_two_state_slot_numberings_are_censused_over_the_shipped_corpus() {
    let mut models = 0_usize;
    let mut with_state = 0_usize;
    let mut with_noise = 0_usize;
    let mut unsound_predicate = 0_usize;
    let mut imprecise_predicate = 0_usize;
    let mut differing_counts = 0_usize;
    let mut appending = 0_usize;
    let mut interleaving = 0_usize;
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = &shipped.name;
        let runtime = &shipped;
        let artifact = &runtime.canonical_ir;
        models += 1;
        let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
            println!("state-slots model={module} refused=cfg-lowering");
            continue;
        };
        let Ok(state) = CfgStateAllocation::build(&artifact.hir, &cfg.function) else {
            println!("state-slots model={module} refused=state-allocation");
            continue;
        };
        let per_site = state.family_len(CanonicalStateFamily::Integration);
        let native_storage = NativeRequiredStorage::for_model(&runtime.model).state_values;
        let agrees = state.agrees_with_emission_allocation(&artifact.hir);
        let contexts = EmissionCensus::of(&artifact.hir);
        let noise = !runtime.model.noise_sources.is_empty();

        // Where the module's sites fall across the generator's
        // assignments/equations boundary.
        let statement_sites = CanonicalStateLayout::statement_prefix(&artifact.hir)
            .family_len(CanonicalStateFamily::Integration);
        let contribution_sites = per_site.saturating_sub(statement_sites);

        let tags = integration_emission_contexts(&runtime.model);
        let per_emission = tags.len();
        let breakdown = |wanted: EmissionContext| {
            tags.iter()
                .filter(|emission| emission.context == wanted)
                .count()
        };
        // Which operator each re-emission is, which is what names the
        // mechanism rather than just its size.
        let reemitted = |wanted: EmittedOperator| {
            tags.iter()
                .filter(|emission| {
                    emission.operator == wanted
                        && !matches!(
                            emission.context,
                            EmissionContext::Assignment | EmissionContext::EquationPrimal
                        )
                })
                .count()
        };
        let shape = prefix_shape(&tags, statement_sites, contribution_sites);

        if per_site > 0 {
            with_state += 1;
        }
        if noise {
            with_noise += 1;
        }
        if per_site != per_emission {
            differing_counts += 1;
            match shape {
                PrefixShape::Append | PrefixShape::Identical => appending += 1,
                PrefixShape::Interleave => interleaving += 1,
            }
        }
        if agrees && per_site != per_emission {
            unsound_predicate += 1;
        }
        if !agrees && per_site == per_emission {
            imprecise_predicate += 1;
        }
        println!(
            "state-slots model={module} per_site={per_site} per_emission={per_emission} \
                 native_required={native_storage} predicate={agrees} shape={shape:?} \
                 sites[statements={statement_sites} contributions={contribution_sites}] \
                 contexts[parameters={} assignments={} noise_assignments={} equation_primal={} \
                 equation_derivative={} noise_sources={}] \
                 reemitted[ddt={} idt={} idtmod={} limit={} canonical_limit={}] \
                 hir[parameters={} statements={} contributions={} noise={}] noise_sources={}",
            breakdown(EmissionContext::Parameter),
            breakdown(EmissionContext::Assignment),
            breakdown(EmissionContext::NoiseAssignment),
            breakdown(EmissionContext::EquationPrimal),
            breakdown(EmissionContext::EquationDerivative),
            breakdown(EmissionContext::NoiseSource),
            reemitted(EmittedOperator::Ddt),
            reemitted(EmittedOperator::Idt),
            reemitted(EmittedOperator::IdtMod),
            reemitted(EmittedOperator::Limit),
            reemitted(EmittedOperator::CanonicalLimit),
            contexts.parameters,
            contexts.statements,
            contexts.contributions,
            contexts.has_noise,
            runtime.model.noise_sources.len(),
        );
    }
    println!(
        "state-slots models={models} with_integration_state={with_state} with_noise={with_noise} \
         differing_counts={differing_counts} append={appending} interleave={interleaving} \
         unsound_predicate={unsound_predicate} imprecise_predicate={imprecise_predicate}"
    );
    // The soundness direction is an assertion, because a caller resuming a
    // foreign checkpoint acts on it: the predicate saying the two spaces
    // coincide while the measurement says they do not is an unsound licence.
    assert_eq!(
        unsound_predicate, 0,
        "agrees_with_emission_allocation licensed a module whose two state-slot numberings \
         measurably differ"
    );
    // Deliberately not an equality assertion on the counts. This is a census:
    // what it establishes is *that* the two spaces differ, on which modules,
    // and whether the extra slots append or interleave. Pinning a number here
    // would freeze a measurement of the bytecode generator that this lane does
    // not own. What it does assert is the shape of the finding, so that a
    // change making the two numberings agree fails here and is read rather than
    // absorbed.
    //
    // Corpus-wide, and therefore only over the corpus. "No shipped module has
    // two differing numberings" is a claim about all forty-three; a run
    // narrowed by `RSPICE_CFG_CENSUS_FILTER` cannot make it or refute it, and
    // asserting it there fails on any slice whose modules happen to agree —
    // `l_utsoi` is two such modules. A filtered run measures and reports.
    if filter.is_none() {
        assert!(
            differing_counts > 0,
            "the two state-slot numberings now agree on every shipped module; that is a change to \
             the ruling recorded on CfgStateAllocation and wants reading"
        );
        assert_eq!(models, 43, "the shipped census is 43 modules");
    } else {
        println!(
            "state-slots filtered={models} differing_counts={differing_counts}; the corpus-wide \
             ruling is asserted only on an unfiltered run"
        );
    }
}

/// What a complex-step probe of one set of `(equation, unknown)` pairs found.
#[derive(Default)]
struct ChargeProbe {
    /// Pairs whose `d(charge)/d(unknown)` is measurably nonzero at the drawn
    /// bias.
    nonzero: usize,
    /// Pairs no oracle can be asked about: a lane outside the seed list, or an
    /// equation this route found no charge for at all.
    unmapped: usize,
    worst: f64,
    case: Option<String>,
}

/// Ask the complex-step oracle whether each pair's charge derivative is zero.
///
/// Grouped by lane, because one complex-step evaluation perturbs one unknown
/// and answers for *every* charge at once: on a compact model that is the
/// difference between one interpretation per pair and one per distinct unknown.
#[allow(clippy::too_many_arguments)]
fn probe_charge_pairs(
    mut pairs: Vec<(usize, usize)>,
    charges: &[Option<ValueId>],
    seeds: &[AdSeed],
    differentiated: &CfgFunction,
    point: &OperatingPoint,
    node_count: usize,
    branch_count: usize,
    found: &mut ChargeProbe,
) {
    pairs.sort_unstable_by_key(|(equation, lane)| (*lane, *equation));
    let mut current: Option<(usize, CfgEvalSnapshot<ComplexStep>)> = None;
    for (equation, lane) in pairs {
        let Some(seed) = seeds.get(lane).copied() else {
            found.unmapped += 1;
            continue;
        };
        let Some(charge) = charges.get(equation).copied().flatten() else {
            found.unmapped += 1;
            continue;
        };
        if current.as_ref().is_none_or(|(held, _)| *held != lane) {
            let complex = point.complex_inputs(node_count, branch_count, seed);
            current = evaluate_cfg(differentiated, &complex)
                .ok()
                .map(|snapshot| (lane, snapshot));
        }
        let Some((_, snapshot)) = current.as_ref() else {
            continue;
        };
        let Some(value) = snapshot.value(charge) else {
            continue;
        };
        let derivative = value.derivative();
        // Scaled against the charge it belongs to, and bounded above for the
        // reason `tests/cfg_derivatives.rs` gives: a reading twelve orders
        // above its own primal is the perturbed evaluation overflowing rather
        // than a statement about a derivative.
        let scale = value.real().abs().max(f64::MIN_POSITIVE);
        if derivative.abs() > scale * ORACLE_SIGNIFICANCE
            && derivative.abs() < scale * ORACLE_DIVERGENCE_FACTOR
        {
            found.nonzero += 1;
            if derivative.abs() > found.worst {
                found.worst = derivative.abs();
                found.case = Some(format!(
                    "d(charge {equation})/d(lane {lane})={derivative:.6e} charge={:.6e}",
                    value.real()
                ));
            }
        }
    }
}

#[derive(Default)]
struct ReactiveTally {
    models: usize,
    /// Pairs the CFG route stamps, the shipped route does not, and whose
    /// complex-step derivative is nonzero — a capacitance missing from the
    /// shipped AC matrix. Reported, not asserted: it is a defect in the route
    /// this lane is replacing, not in the one it is building.
    cfg_only_nonzero: usize,
    /// Models whose CFG carries no stored charge at all, so there is no
    /// reactive matrix to lower.
    resistive_models: usize,
    charged_equations: usize,
    reactive_programs: usize,
    refused_programs: usize,
    lowered_instructions: usize,
    executed: usize,
    comparisons: usize,
    runtime_errors: usize,
    oracle_comparisons: usize,
    sparsity: SparsityTally,
    sparsity_unpaired: usize,
}

/// Reactive Jacobians from CFG-level AD, lowered on the block route and checked
/// against everything that can answer.
///
/// # What a reactive Jacobian is, and why it is a second program set
///
/// The shipped route keeps `dQ/dx` apart from `dI/dx` because the two are
/// stamped by different callers at different times: the conduction matrix goes
/// into every Newton iteration, while `StampProgram::reactive_jacobians` is
/// read only by `VerilogADevice::stamp_reactive`, whose caller multiplies each
/// entry by `jω`. Transient does not use it at all — there the `ddt` operator
/// carries its own companion coefficient inside the residual, and its
/// derivative is `ddt_companion`, which is already in the conduction matrix. So
/// this is the small-signal capacitance matrix and nothing else.
///
/// # Where the charge comes from on this route
///
/// [`stored_charges`] — the CFG-level extraction the generated backend already
/// ships — rather than a fourth copy of the peel. It resolves the charge
/// through block parameters, which is what a guarded contribution reaches its
/// equation as; the `IrExpr` and MIR peels the other two routes use see a
/// `Conditional` node instead and cannot follow one. The sparsity table below
/// is where that difference shows up, in the `cfg_only` column.
///
/// Three questions, and they are different:
///
/// 1. **Totality.** Does every entry of every charge row lower onto the block
///    model, and if not, which construct stopped it?
/// 2. **Value.** Does the compiled entry agree with the reference interpreter
///    evaluating the same scalarized function, and with a complex-step
///    derivative of the *primal charge* — an oracle containing no chain rule?
/// 3. **Sparsity.** Does the set of `(equation, unknown)` pairs match the one
///    the shipped planner enumerates in `reactive_jacobians`? A pair the
///    shipped route stamps and this one does not is a dropped capacitance
///    unless its complex-step derivative is zero, and
///    [`SparsityTally::shipped_only_nonzero`] is what decides which.
#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_reactive_jacobian_route_agrees_with_the_interpreter_and_the_oracles() {
    let mut tally = ReactiveTally::default();
    let mut worst_overall = 0.0_f64;
    let mut worst_oracle_overall = 0.0_f64;
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = &shipped.name;
        let started = std::time::Instant::now();
        let runtime = &shipped;
        let artifact = &runtime.canonical_ir;
        tally.models += 1;
        let Ok(mut cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
            println!("cfg-reactive model={module} refused=cfg-lowering");
            continue;
        };

        // Charges first: the extraction *builds* the values a scaled or
        // summed charge needs (`k * ddt(q)` stores `k * q`, which exists
        // nowhere until it is spliced in), so it has to run before the
        // state allocation is read off the function and before anything is
        // differentiated.
        let charges = stored_charges(&mut cfg.function, &cfg.residuals);
        let Ok(state) = CfgStateAllocation::build(&artifact.hir, &cfg.function) else {
            println!("cfg-reactive model={module} refused=state-allocation");
            continue;
        };

        let (seeds, correction_lane) = derivative_seeds(&cfg, &artifact.mir);
        let mut differentiated = match differentiate(&cfg.function, &seeds) {
            Ok(differentiated) => differentiated,
            Err(error) => {
                println!("cfg-reactive model={module} refused=differentiate detail={error:?}");
                continue;
            }
        };
        // Every read-out before anything evaluates or lowers: taking one
        // appends an instruction, so a function captured earlier would not
        // contain the later ones.
        let rows: Vec<Vec<Option<ValueId>>> = charges
            .iter()
            .map(|charge| match charge {
                Some(charge) => differentiated.derivative_row(*charge),
                None => Vec::new(),
            })
            .collect();
        let scalarized = match scalarize_lanes(&differentiated.function) {
            Ok(scalarized) => scalarized,
            Err(error) => {
                println!("cfg-reactive model={module} refused=scalarize detail={error}");
                continue;
            }
        };

        let branch_unknowns = canonical_branch_unknown_runtime_map(&runtime.model, &artifact.mir)
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

        let charged_equations = charges.iter().filter(|charge| charge.is_some()).count();
        tally.charged_equations += charged_equations;
        if charged_equations == 0 {
            tally.resistive_models += 1;
        }

        // ---- sparsity ---------------------------------------------------
        let column_lanes = ShippedColumnLanes::build(&runtime.model, &artifact.mir)
            .unwrap_or_else(|error| panic!("{module}: shipped column lanes: {error}"));
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
            for entry in &program.reactive_jacobians {
                if let Some(lane) = column_lanes.lane(&entry.col_axis) {
                    shipped_pairs.insert((equation, lane));
                }
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

        // Both directions get asked the same question, because for the
        // reactive matrix both are findings and they are different ones.
        //
        // * A **shipped-only** pair with a nonzero `d(charge)/d(unknown)`
        //   is a capacitance *this* route drops.
        // * A **cfg-only** pair with a nonzero one is a capacitance the
        //   *shipped* route drops — `DeviceIR::extract_charge` refuses a
        //   `ddt` in a shape it does not know and logs a warning rather
        //   than failing, so a whole reactive row can be missing from the
        //   AC matrix with nothing but a log line to say so.
        let probe = |pairs: Vec<(usize, usize)>, sink: &mut ChargeProbe| {
            probe_charge_pairs(
                pairs,
                &charges,
                &seeds,
                &differentiated.function,
                &points[0],
                artifact.mir.nodes.len(),
                artifact.mir.branches.len(),
                sink,
            );
        };
        let mut shipped_only_probe = ChargeProbe::default();
        let mut cfg_only_probe = ChargeProbe::default();
        if paired {
            if shipped_only > 0 {
                probe(
                    shipped_pairs.difference(&cfg_pairs).copied().collect(),
                    &mut shipped_only_probe,
                );
            }
            if cfg_only > 0 {
                probe(
                    cfg_pairs.difference(&shipped_pairs).copied().collect(),
                    &mut cfg_only_probe,
                );
            }
        }
        tally.sparsity.shipped_only_nonzero += shipped_only_probe.nonzero;
        tally.sparsity.shipped_only_unmapped += shipped_only_probe.unmapped;
        tally.cfg_only_nonzero += cfg_only_probe.nonzero;

        // ---- lowering, execution and comparison -------------------------
        let mut reactive_programs = 0_usize;
        let mut refused = 0_usize;
        let mut instructions = 0_usize;
        let mut executed = 0_usize;
        let mut oracle_checks = 0_usize;
        let mut worst = 0.0_f64;
        let mut worst_case: Option<String> = None;
        let mut worst_oracle = 0.0_f64;
        let mut oracle_case: Option<String> = None;
        let mut first_refusal: Option<String> = None;

        for (equation, charge) in charges.iter().enumerate() {
            let Some(charge) = *charge else { continue };
            let mut outputs: Vec<(usize, ValueId)> = Vec::new();
            for (lane, entry) in rows[equation].iter().enumerate() {
                if Some(lane) == correction_lane {
                    continue;
                }
                if let Some(entry) = entry
                    && let Some(scalar) = scalarized.scalar(*entry)
                {
                    outputs.push((lane, scalar));
                }
            }

            // Sliced to the whole row once, then to each entry within it,
            // for the reason the conduction census gives: pruning is linear
            // in the function, so slicing the model per entry would be the
            // equation count times the lane count passes over the whole
            // body.
            let row_outputs: Vec<ValueId> = outputs.iter().map(|(_, output)| *output).collect();
            let (row_function, row_mapped) =
                prune_cfg_to_outputs(&scalarized.function, &row_outputs);
            let outputs: Vec<(usize, ValueId)> = outputs
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
                reactive_programs += 1;
                instructions += program.instructions().len();

                if executed >= EXECUTED_JACOBIAN_ENTRIES_PER_MODEL {
                    continue;
                }
                let image = compile_value_function_artifact_from_ssa(&program)
                    .unwrap_or_else(|error| panic!("{module} dQ{equation}/d{lane}: {error}"));
                let memory = ExecutableMemory::allocate(image.bytes())
                    .unwrap_or_else(|error| panic!("{module} dQ{equation}/d{lane}: {error}"));
                let entry_point = memory.ptr_at(0).expect("entry inside published image");
                let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                    unsafe { std::mem::transmute(entry_point) };
                executed += 1;

                for (index, point) in points.iter_mut().enumerate() {
                    let interpreter_inputs = point
                        .interpreter_inputs(artifact.mir.nodes.len(), artifact.mir.branches.len());
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
                            "d(charge {equation})/d(lane {lane}) point={index} \
                                 interpreter={reference:.17e} block_program={actual:.17e}"
                        ));
                    }

                    if index == 0 && oracle_checks < ORACLE_ENTRIES_PER_MODEL {
                        oracle_checks += 1;
                        let complex = point.complex_inputs(
                            artifact.mir.nodes.len(),
                            artifact.mir.branches.len(),
                            seeds[lane],
                        );
                        if let Ok(snapshot) = evaluate_cfg(&differentiated.function, &complex)
                            && let Some(value) = snapshot.value(charge)
                        {
                            let oracle = value.derivative();
                            tally.oracle_comparisons += 1;
                            let scale = oracle.abs().max(actual.abs());
                            if scale > 1.0e-30
                                && let Some(delta) = deviation(oracle, actual)
                                && delta > worst_oracle
                            {
                                worst_oracle = delta;
                                oracle_case = Some(format!(
                                    "d(charge {equation})/d(lane {lane}) \
                                         complex_step={oracle:.17e} block_program={actual:.17e}"
                                ));
                            }
                        }
                    }
                }
            }
        }

        tally.reactive_programs += reactive_programs;
        tally.refused_programs += refused;
        tally.lowered_instructions += instructions;
        tally.executed += executed;
        worst_overall = worst_overall.max(worst);
        worst_oracle_overall = worst_oracle_overall.max(worst_oracle);
        println!(
            "cfg-reactive model={module} equations={} charged={charged_equations} \
                 shipped_reactive_rows={} reactive_programs={reactive_programs} refused={refused} \
                 instructions={instructions} executed={executed} \
                 max_relative_deviation={worst:.3e} oracle_deviation={worst_oracle:.3e} \
                 sparsity[shared={shared} cfg_only={cfg_only} shipped_only={shipped_only} \
                 cfg_only_nonzero={} shipped_only_nonzero={}{}] \
                 seconds={:.1}{}{}{}{}{}",
            cfg.residuals.len(),
            runtime
                .model
                .stamp_programs
                .iter()
                .filter(|stamp| !stamp.reactive_jacobians.is_empty())
                .count(),
            cfg_only_probe.nonzero,
            shipped_only_probe.nonzero,
            if paired { "" } else { " UNPAIRED" },
            started.elapsed().as_secs_f64(),
            shipped_only_probe
                .case
                .map(|case| format!(" SHIPPED_ONLY_NONZERO[{case}]"))
                .unwrap_or_default(),
            cfg_only_probe
                .case
                .map(|case| format!(" SHIPPED_DROPS_CAPACITANCE[{case}]"))
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
    println!(
        "cfg-reactive models={} resistive_models={} charged_equations={} reactive_programs={} \
         refused_programs={} lowered_instructions={} executed={} comparisons={} runtime_errors={} \
         oracle_comparisons={} sparsity[shared={} cfg_only={} shipped_only={} \
         cfg_only_nonzero={} shipped_only_nonzero={} shipped_only_unmapped={} \
         unpaired_models={}] \
         max_relative_deviation={worst_overall:.3e} max_oracle_deviation={worst_oracle_overall:.3e}",
        tally.models,
        tally.resistive_models,
        tally.charged_equations,
        tally.reactive_programs,
        tally.refused_programs,
        tally.lowered_instructions,
        tally.executed,
        tally.comparisons,
        tally.runtime_errors,
        tally.oracle_comparisons,
        tally.sparsity.shared,
        tally.sparsity.cfg_only,
        tally.sparsity.shipped_only,
        tally.cfg_only_nonzero,
        tally.sparsity.shipped_only_nonzero,
        tally.sparsity.shipped_only_unmapped,
        tally.sparsity_unpaired,
    );
    assert_eq!(
        tally.sparsity.shipped_only_nonzero, 0,
        "the CFG route drops a reactive Jacobian entry the shipped route stamps and the primal \
         charge's own complex-step derivative says is nonzero"
    );
    if filter.is_none() {
        assert_eq!(tally.models, 43, "the shipped census is 43 modules");
    }
    assert!(
        tally.comparisons > 0,
        "the census must actually execute the lowered reactive Jacobian programs"
    );
}

/// How many noise values per model are compiled and executed.
///
/// The same argument the residual census makes: a noise power shares nearly all
/// of its cone with the operating point the same body computes, so machine code
/// for every magnitude in a compact model measures nothing the first few do
/// not.
const EXECUTED_NOISE_VALUES_PER_MODEL: usize = 6;

#[derive(Default)]
struct NoiseTally {
    models: usize,
    /// Models the body lowers no noise process for. A silent model is not a
    /// refusal: most compact models declare no noise at all.
    silent_models: usize,
    processes: usize,
    shipped_sources: usize,
    /// Processes the two routes agree exist, by process id and by shape.
    paired: usize,
    /// Processes one route carries and the other does not, or that disagree
    /// about kind, exponent, or table width.
    unpaired: usize,
    values: usize,
    lowered_primal: usize,
    lowered_differentiated: usize,
    refused: usize,
    lowered_instructions: usize,
    executed: usize,
    comparisons: usize,
    runtime_errors: usize,
    /// Models whose noise slice the generated backend declines, so the flat
    /// HIR-driven emitter carries them.
    flat_fallback_models: usize,
}

/// What one noise process asks the body for, named.
fn noise_process_values(process: &CfgNoiseProcess) -> Vec<(String, ValueId)> {
    let mut values = vec![
        ("active".to_string(), process.active),
        ("psd".to_string(), process.psd),
    ];
    if let Some(exponent) = process.exponent {
        values.push(("exponent".to_string(), exponent));
    }
    for (index, entry) in process.table.iter().enumerate() {
        values.push((format!("table[{index}]"), *entry));
    }
    values
}

/// Noise magnitudes lowered on the block route and checked against the
/// interpreter, with the correspondence to the shipped noise programs censused
/// alongside.
///
/// # What a noise program is on each route, and why they are compared this way
///
/// The bytecode route compiles one `psd_program` and one optional
/// `exponent_program` per `CompiledNoiseSource`, out of the flat `IrExpr` the
/// front end extracted, and the native plan lowers exactly those two. The CFG
/// route has the same magnitudes as ordinary values of the body — a
/// `CfgNoiseProcess` names them — and `NoiseProcess` itself lowers to the
/// constant zero, because a noise source contributes nothing large-signal.
/// Everything a magnitude needs is therefore already covered by the block
/// lowering, and what this census establishes is that it *is* covered: that
/// every magnitude the shipped route compiles has a block program, that the two
/// routes name the same processes, and that the block program agrees with the
/// reference interpreter.
///
/// The oracle is the interpreter rather than the shipped bytecode, deliberately
/// and for a reason that is not convenience: a shipped `psd_program` reads
/// variable slots that only the shipped assignment pass fills, so executing one
/// at an operating point means replaying `assignment_steps` and
/// `noise_assignment_steps` with the runtime's own event-state and activation
/// rules. A census that reimplemented that replay would be measuring its own
/// copy of the runtime. `evaluate_cfg` is the semantic authority every other
/// backend is checked against, and it needs no replay.
///
/// # The two-pass rule, kept
///
/// `plan_noise` cuts its slice from the primal body first and from the
/// differentiated one only if that fails, because six shipped models read a
/// `ddx` inside a noise power and only the AD pass resolves one. This census
/// keeps the same rule per value and reports which pass carried it, so a model
/// moving between the two is visible.
#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn the_cfg_noise_route_agrees_with_the_interpreter_and_the_shipped_plan() {
    let mut tally = NoiseTally::default();
    let mut worst_overall = 0.0_f64;
    let filter = std::env::var("RSPICE_CFG_CENSUS_FILTER").ok();
    for shipped in shipped_census_models_matching(filter.as_deref()) {
        let module = &shipped.name;
        let started = std::time::Instant::now();
        let runtime = &shipped;
        let artifact = &runtime.canonical_ir;
        tally.models += 1;
        let Ok(cfg) = CfgModel::from_hir(&artifact.hir, &artifact.mir) else {
            println!("cfg-noise model={module} refused=cfg-lowering");
            continue;
        };
        let Ok(state) = CfgStateAllocation::build(&artifact.hir, &cfg.function) else {
            println!("cfg-noise model={module} refused=state-allocation");
            continue;
        };

        tally.processes += cfg.noise_processes.len();
        tally.shipped_sources += runtime.model.noise_sources.len();
        if cfg.noise_processes.is_empty() && runtime.model.noise_sources.is_empty() {
            tally.silent_models += 1;
        }

        // ---- correspondence with the shipped noise programs -------------
        //
        // Keyed by `process_id`, which is the one name the two routes
        // share: `DeviceIR` takes it from the front end's noise-site
        // ordinal and the CFG lowering carries the same number on every
        // `CfgNoiseProcess`.
        let mut paired = 0_usize;
        let mut unpaired = 0_usize;
        let mut mismatch: Option<String> = None;
        for source in &runtime.model.noise_sources {
            let Some(process) = cfg
                .noise_processes
                .iter()
                .find(|process| usize::try_from(process.process_id) == Ok(source.process_id))
            else {
                unpaired += 1;
                mismatch.get_or_insert_with(|| {
                    format!("shipped process {} has no CFG process", source.process_id)
                });
                continue;
            };
            let shipped_table = source.table.as_ref().map_or(0, |(points, _)| points.len());
            let kind_agrees = match process.kind {
                CanonicalNoiseSourceKind::White => {
                    source.exponent_program.is_none() && source.table.is_none()
                }
                CanonicalNoiseSourceKind::Flicker => {
                    source.exponent_program.is_some() && source.table.is_none()
                }
                CanonicalNoiseSourceKind::Table => source.table.is_some(),
            };
            if !kind_agrees || process.exponent.is_some() != source.exponent_program.is_some() {
                unpaired += 1;
                mismatch.get_or_insert_with(|| {
                    format!(
                        "process {} kind={:?} exponent={} against shipped exponent={} table={}",
                        source.process_id,
                        process.kind,
                        process.exponent.is_some(),
                        source.exponent_program.is_some(),
                        shipped_table,
                    )
                });
                continue;
            }
            paired += 1;
        }
        // A CFG process the shipped model has no source for is the other
        // direction of the same finding.
        for process in &cfg.noise_processes {
            if !runtime
                .model
                .noise_sources
                .iter()
                .any(|source| usize::try_from(process.process_id) == Ok(source.process_id))
            {
                unpaired += 1;
                mismatch.get_or_insert_with(|| {
                    format!("CFG process {} has no shipped source", process.process_id)
                });
            }
        }
        tally.paired += paired;
        tally.unpaired += unpaired;

        // ---- which emitter the generated backend uses -------------------
        //
        // The emitter's own rule, kept: the slice is cut from the primal
        // body first and from the differentiated one only if that fails, so
        // a model that declines on the primal has *not* fallen back until
        // the second attempt declines too.
        let seeds = derivative_seeds(&cfg, &artifact.mir).0;
        let primal_decline = noise_plan_decline(artifact, &cfg, &cfg.function);
        let mut differentiated: Option<AdFunction> = None;
        let decline = match primal_decline {
            None => None,
            Some(primal) => {
                differentiated = differentiate(&cfg.function, &seeds).ok();
                match &differentiated {
                    Some(resolved) => noise_plan_decline(artifact, &cfg, &resolved.function),
                    None => Some(primal),
                }
            }
        };
        if decline.is_some() {
            tally.flat_fallback_models += 1;
        }

        if cfg.noise_processes.is_empty() {
            println!(
                "cfg-noise model={module} processes=0 shipped_sources={} \
                     plan_decline={decline:?} primal_decline={primal_decline:?} seconds={:.1}",
                runtime.model.noise_sources.len(),
                started.elapsed().as_secs_f64(),
            );
            continue;
        }

        // ---- lowering, execution and comparison -------------------------
        let branch_unknowns = canonical_branch_unknown_runtime_map(&runtime.model, &artifact.mir)
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
        // Noise first, because that is the analysis these magnitudes are
        // evaluated in; a DC point alongside it so a `$analysis` guard
        // inside a power is exercised both ways.
        let mut points: Vec<OperatingPoint> = [(0x0000_5E15_u64, 3_u8), (0x0005_EED1, 0)]
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

        // The scalarized differentiated body, cut once and only if a
        // magnitude needs it: a `ddx` inside a noise power has no value
        // until the AD pass resolves it, and running that pass on a model
        // whose magnitudes lower from the primal would cost only time.
        let mut resolved: Option<Option<ScalarLanes>> = None;

        let mut lowered_primal = 0_usize;
        let mut lowered_differentiated = 0_usize;
        let mut refused = 0_usize;
        let mut instructions = 0_usize;
        let mut executed = 0_usize;
        let mut worst = 0.0_f64;
        let mut worst_case: Option<String> = None;
        let mut first_refusal: Option<String> = None;

        for process in &cfg.noise_processes {
            for (name, value) in noise_process_values(process) {
                tally.values += 1;
                let (pruned, pruned_outputs) = prune_cfg_to_outputs(&cfg.function, &[value]);
                let mut program = lower_cfg_function(&pruned, pruned_outputs[0], &state, &bindings)
                    .map(|program| (program, pruned, pruned_outputs[0], false));
                if program.is_err() {
                    if resolved.is_none() {
                        if differentiated.is_none() {
                            differentiated = differentiate(&cfg.function, &seeds).ok();
                        }
                        resolved = Some(
                            differentiated
                                .as_ref()
                                .and_then(|body| scalarize_lanes(&body.function).ok()),
                        );
                    }
                    let scalarized = resolved.as_ref().expect("just filled");
                    if let Some(scalarized) = scalarized.as_ref()
                        && let Some(scalar) = scalarized.scalar(value)
                    {
                        let (pruned, outputs) =
                            prune_cfg_to_outputs(&scalarized.function, &[scalar]);
                        program = lower_cfg_function(&pruned, outputs[0], &state, &bindings)
                            .map(|program| (program, pruned, outputs[0], true));
                    }
                }
                let (program, pruned, output, differentiated) = match program {
                    Ok(lowered) => lowered,
                    Err(error) => {
                        refused += 1;
                        tally.refused += 1;
                        first_refusal.get_or_insert_with(|| format!("{name}: {error}"));
                        continue;
                    }
                };
                if differentiated {
                    lowered_differentiated += 1;
                    tally.lowered_differentiated += 1;
                } else {
                    lowered_primal += 1;
                    tally.lowered_primal += 1;
                }
                instructions += program.instructions().len();

                if executed >= EXECUTED_NOISE_VALUES_PER_MODEL {
                    continue;
                }
                let image =
                    compile_value_function_artifact_from_ssa(&program).unwrap_or_else(|error| {
                        panic!("{module} process {} {name}: {error}", process.process_id)
                    });
                let memory = ExecutableMemory::allocate(image.bytes()).unwrap_or_else(|error| {
                    panic!("{module} process {} {name}: {error}", process.process_id)
                });
                let entry_point = memory.ptr_at(0).expect("entry inside published image");
                let function: extern "C" fn(*const EvalContext, *const f64) -> f64 =
                    unsafe { std::mem::transmute(entry_point) };
                executed += 1;
                tally.executed += 1;

                for (index, point) in points.iter_mut().enumerate() {
                    let interpreter_inputs = point
                        .interpreter_inputs(artifact.mir.nodes.len(), artifact.mir.branches.len());
                    let Ok(snapshot) = evaluate_cfg(&pruned, &interpreter_inputs) else {
                        continue;
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
                            "process={} {name} point={index} interpreter={reference:.17e} \
                                 block_program={actual:.17e}",
                            process.process_id
                        ));
                    }
                }
            }
        }

        tally.lowered_instructions += instructions;
        worst_overall = worst_overall.max(worst);
        println!(
            "cfg-noise model={module} processes={} shipped_sources={} paired={paired} \
                 unpaired={unpaired} primal={lowered_primal} differentiated={lowered_differentiated} \
                 refused={refused} instructions={instructions} executed={executed} \
                 max_relative_deviation={worst:.3e} plan_decline={decline:?} \
                 primal_decline={primal_decline:?} seconds={:.1}{}{}{}",
            cfg.noise_processes.len(),
            runtime.model.noise_sources.len(),
            started.elapsed().as_secs_f64(),
            mismatch
                .map(|case| format!(" MISMATCH[{case}]"))
                .unwrap_or_default(),
            worst_case
                .map(|case| format!(" worst_case[{case}]"))
                .unwrap_or_default(),
            first_refusal
                .map(|refusal| format!(" first_refusal={refusal}"))
                .unwrap_or_default(),
        );
    }
    println!(
        "cfg-noise models={} silent_models={} processes={} shipped_sources={} paired={} \
         unpaired={} values={} lowered_primal={} lowered_differentiated={} refused={} \
         lowered_instructions={} executed={} comparisons={} runtime_errors={} \
         flat_fallback_models={} max_relative_deviation={worst_overall:.3e}",
        tally.models,
        tally.silent_models,
        tally.processes,
        tally.shipped_sources,
        tally.paired,
        tally.unpaired,
        tally.values,
        tally.lowered_primal,
        tally.lowered_differentiated,
        tally.refused,
        tally.lowered_instructions,
        tally.executed,
        tally.comparisons,
        tally.runtime_errors,
        tally.flat_fallback_models,
    );
    assert_eq!(
        tally.unpaired, 0,
        "the CFG body and the shipped noise programs disagree about which noise processes a \
         module has, or about one's kind"
    );
    assert_eq!(
        tally.refused, 0,
        "the block route refuses a noise magnitude the shipped route compiles"
    );
    if filter.is_none() {
        assert_eq!(tally.models, 43, "the shipped census is 43 modules");
        // The flat fallback's whole remaining reach, pinned. Both models are
        // silent — `EPFL_HEMT_10a` and `vbic_4T_et_cf` declare no noise source
        // at all — so `plan_noise` declines on `NoPlannedSources` and there is
        // nothing to plan. Every other module is carried by the CFG slice, on
        // the primal body or on the differentiated one.
        //
        // Deliberately not *closed*. Making `plan_noise` accept an empty plan
        // would route those two through the canonical emitter, and the two
        // emitters do not write the same file for a silent model: the flat one
        // opens with `#![allow(dead_code, non_snake_case, unused_assignments,
        // unused_parens, unused_variables)]` and defines `LIMEXP_MAX` and
        // `THERMAL_VOLTAGE_PER_K`, the canonical one omits `unused_assignments`
        // and both constants. That is a change to the shipped bundle, which
        // this lane does not make.
        assert_eq!(
            tally.silent_models, 2,
            "the shipped corpus has two noise-free modules"
        );
        assert_eq!(
            tally.flat_fallback_models, tally.silent_models,
            "a module with noise sources now takes the flat noise emitter, or a silent one no \
             longer does; either is a change to which emitter the generated bundle uses"
        );
    }
    assert!(
        tally.comparisons > 0,
        "the census must actually execute the lowered noise programs"
    );
}

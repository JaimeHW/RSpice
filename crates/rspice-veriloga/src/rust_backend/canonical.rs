//! Generating a device from the canonical CFG, rather than from a tier.
//!
//! This is the emitter the rebuild exists to produce, wired to the device
//! contract the tiers already satisfy: `state.rs` holds the parameters and the
//! per-instance state, `stamp.rs` evaluates the body and writes the matrix, and
//! `noise.rs` evaluates the small-signal powers. The last two are both the
//! output of [`super::emit`] over a simplified CFG — `stamp.rs` over a
//! differentiated and scheduled one, `noise.rs` over the primal body with no
//! derivative lanes at all, because a noise power is a magnitude and nothing
//! asks for its slope.
//!
//! ## What the tiers did that this does not
//!
//! *Scalarised derivatives.* A tier gives every lane its own value, so a wide
//! MOSFET carries a hundred thousand of them. Here a derivative is one packed
//! value over its own live lane set.
//!
//! *Flattened guards.* A tier turns `if` into arithmetic over both arms. Here
//! the control flow survives into the generated Rust, so the code skips the work
//! the model said to skip.
//!
//! *Zeros.* 202 of 931 stamp arguments in the tier output are literal
//! `multiplicity * 0.0`. [`super::stamp_plan`] decides which entries exist at
//! all, and the ones that do not are simply absent.
//!
//! ## Stages are functions, and that is why outputs need slots
//!
//! [`crate::canonical_ir::schedule::split`] cuts the body by how often each
//! value goes stale. Every class coarser than Newton becomes its own `fn` on
//! `Instance` that writes what later readers need into a slot array; the Newton
//! body runs in `stamp` and reads those slots. The instance and temperature
//! stages are guarded by validity flags, which is where the caching is. The
//! timestep stage runs on every call — nothing in the device contract tells
//! `stamp` that a new timestep began, and recomputing is correct, merely
//! uncached.
//!
//! Splitting at all is decided per model by
//! [`crate::canonical_ir::schedule::worth_splitting`], because a body that is
//! 97% Newton pays for the staged loads and gets nothing back. Most compact
//! models decline it.
//!
//! ## Charge storage
//!
//! A reactive stamp writes `d(charge)/d(unknown)`, not the residual's Jacobian,
//! so it needs the `ddt` operand rather than the `ddt` result. A contribution is
//! reactive when its residual *is* a `ddt` — which is how MIR presents it, one
//! equation per `<+` statement — and the charge is that operator's input,
//! differentiated by the same pass in the same body.
//!
//! ## What it refuses
//!
//! Indirect contributions, an unresolved flow probe, and the simulator-control
//! tasks `$bound_step` and `$discontinuity`. Each is a piece the canonical
//! level has not finished, and a device that quietly computed something else
//! would be worse than one that is not generated: the caller falls back to a
//! tier, which is what the tiers are still there for.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

use crate::canonical_ir::ad::{DifferentiationError, differentiate_with_control};
use crate::canonical_ir::cfg::{
    CfgBinaryOp, CfgFunction, CfgInstruction, CfgTerminator, CfgUnaryOp, CfgValue, CfgValueKind,
    CfgValueType,
};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::cfg_opt::{
    optimize_with_control, optimize_with_control_and_tracking, optimize_with_tracking,
};
use crate::canonical_ir::schedule::{
    InvalidationClass, Stage, schedule_with_parameter_scopes, split, structural_guards,
    worth_splitting,
};
use crate::canonical_ir::{
    AdSeed, BlockId, CanonicalIrArtifact, ExprId, MirEquationKind, NodeId, ValueId, optimize_cfg,
};
use crate::metrics::{
    CfgStructureMetrics, KernelRegionMetric, MetricsRecorder, PipelineCancelled, PipelineControl,
    PipelinePhase,
};

use super::emit::{EmitBindings, emit_body, lane_runtime_types};
use super::expr::parameter_field_names;
use super::stamp_plan::{StampPlan, StampRow, split_row};
use super::{GeneratedRustDevice, GeneratedRustFile, RustBackendError, RustDeviceNames};
use super::{RustTranspileOptions, state_file};

pub fn generate_device(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
) -> Result<GeneratedRustDevice, RustBackendError> {
    let mut measurements = MetricsRecorder::new(0, options.performance_budget.clone());
    generate_device_measured(artifact, options, &mut measurements)
}

pub(crate) fn generate_device_measured(
    artifact: &CanonicalIrArtifact,
    options: &RustTranspileOptions,
    measurements: &mut MetricsRecorder,
) -> Result<GeneratedRustDevice, RustBackendError> {
    artifact.validate().map_err(|diagnostics| {
        RustBackendError::internal(
            artifact.metadata.source_package.as_str(),
            artifact.hir.module_name.as_str(),
            diagnostics
                .first()
                .map(|diagnostic| diagnostic.message.clone())
                .unwrap_or_else(|| "canonical artifact validation failed".to_string()),
        )
    })?;
    if let Some(parameter) = artifact
        .hir
        .parameters
        .iter()
        .find(|parameter| !parameter.dimensions.is_empty())
    {
        return Err(RustBackendError::unsupported(
            artifact.metadata.source_package.as_str(),
            artifact.hir.module_name.as_str(),
            format!(
                "parameter array '{}' requires the array-valued generated-runtime ABI",
                parameter.name
            ),
        ));
    }
    let plan = ModelPlan::build(artifact, measurements)?;

    let names = RustDeviceNames::new(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        artifact.metadata.source_digest.as_str(),
    );
    let parameter_fields = parameter_field_names(artifact);

    checkpoint_phase(artifact, measurements, PipelinePhase::StampEmission)?;
    let phase_started = web_time::Instant::now();
    let stamp = plan.stamp_file(artifact, options, measurements.control())?;
    record_phase(
        artifact,
        measurements,
        PipelinePhase::StampEmission,
        phase_started.elapsed(),
    )?;
    checkpoint_phase(artifact, measurements, PipelinePhase::StateEmission)?;
    let phase_started = web_time::Instant::now();
    let state = state_file::generate_state_file_with_extensions(
        artifact,
        options,
        &parameter_fields,
        plan.ddt_slots.len(),
        plan.idt_slots.len(),
        plan.one_step_dae_split_safe,
        artifact.mir.branch_unknowns.len(),
        &plan.state_extensions(artifact),
    )?;
    record_phase(
        artifact,
        measurements,
        PipelinePhase::StateEmission,
        phase_started.elapsed(),
    )?;
    checkpoint_phase(artifact, measurements, PipelinePhase::NoiseEmission)?;
    let phase_started = web_time::Instant::now();
    let noise = plan.noise_file(artifact, options)?;
    record_phase(
        artifact,
        measurements,
        PipelinePhase::NoiseEmission,
        phase_started.elapsed(),
    )?;

    let files = vec![
        GeneratedRustFile {
            relative_path: "mod.rs".to_string(),
            contents: state_file::generate_mod_file(),
        },
        GeneratedRustFile {
            relative_path: "state.rs".to_string(),
            contents: state,
        },
        GeneratedRustFile {
            relative_path: "stamp.rs".to_string(),
            contents: stamp,
        },
        noise,
    ];

    Ok(GeneratedRustDevice {
        module_name: artifact.mir.module_name.to_string(),
        public_model_name: names.public_model_name,
        folder_name: names.folder,
        source_digest: artifact.metadata.source_digest.to_string(),
        files,
    })
}

fn record_phase(
    artifact: &CanonicalIrArtifact,
    measurements: &mut MetricsRecorder,
    phase: PipelinePhase,
    elapsed: std::time::Duration,
) -> Result<(), RustBackendError> {
    measurements.record(phase, elapsed).map_err(|error| {
        RustBackendError::performance_budget(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            error,
        )
    })
}

fn checkpoint_phase(
    artifact: &CanonicalIrArtifact,
    measurements: &MetricsRecorder<'_>,
    phase: PipelinePhase,
) -> Result<(), RustBackendError> {
    measurements.checkpoint(phase).map_err(|error| {
        RustBackendError::cancelled(
            artifact.metadata.source_package.as_str(),
            artifact.mir.module_name.as_str(),
            error,
        )
    })
}

fn cfg_structure_metrics(function: &CfgFunction) -> CfgStructureMetrics {
    CfgStructureMetrics {
        value_count: crate::metrics::usize_to_u64(function.values.len()),
        instruction_count: crate::metrics::usize_to_u64(
            function
                .blocks
                .iter()
                .map(|block| block.instructions.len())
                .sum(),
        ),
        block_count: crate::metrics::usize_to_u64(function.blocks.len()),
        block_parameter_count: crate::metrics::usize_to_u64(
            function.blocks.iter().map(|block| block.params.len()).sum(),
        ),
        branch_count: crate::metrics::usize_to_u64(
            function
                .blocks
                .iter()
                .filter(|block| matches!(block.terminator, CfgTerminator::Branch { .. }))
                .count(),
        ),
        lane_widen_count: crate::metrics::usize_to_u64(
            function
                .values
                .iter()
                .filter(|value| matches!(value.kind, CfgValueKind::LaneWiden { .. }))
                .count(),
        ),
    }
}

fn kernel_region_metrics(
    artifact: &CanonicalIrArtifact,
    function: &CfgFunction,
    schedule: &crate::canonical_ir::schedule::Schedule,
) -> Vec<KernelRegionMetric> {
    fn visit(
        function: &CfgFunction,
        block: BlockId,
        visited: &mut HashSet<BlockId>,
        postorder: &mut Vec<BlockId>,
    ) {
        if !visited.insert(block) {
            return;
        }
        for successor in function.block(block).successors() {
            visit(function, successor, visited, postorder);
        }
        postorder.push(block);
    }

    let mut postorder = Vec::new();
    visit(
        function,
        function.entry,
        &mut HashSet::new(),
        &mut postorder,
    );
    postorder.reverse();
    let mut definitions = vec![None; function.values.len()];
    for (block_index, block_id) in postorder.iter().enumerate() {
        let block = function.block(*block_id);
        for (local_index, value) in block
            .params
            .iter()
            .copied()
            .chain(
                block
                    .instructions
                    .iter()
                    .map(|instruction| instruction.result),
            )
            .enumerate()
        {
            definitions[usize::from(value)] = Some((block_index, local_index));
        }
    }

    let mut node_indices = HashMap::new();
    let mut branch_indices = HashMap::new();
    let mut unknown_indices = HashMap::new();
    let mut operator_indices = HashMap::new();
    let mut staged_indices = HashMap::new();
    for block_id in &postorder {
        let block = function.block(*block_id);
        for value_id in block.params.iter().copied().chain(
            block
                .instructions
                .iter()
                .map(|instruction| instruction.result),
        ) {
            match &function.value(value_id).kind {
                CfgValueKind::NodePotential(node) => {
                    let next = node_indices.len();
                    node_indices.entry(*node).or_insert(next);
                }
                CfgValueKind::BranchFlow(branch) => {
                    let next = branch_indices.len();
                    branch_indices.entry(*branch).or_insert(next);
                }
                CfgValueKind::BranchUnknownFlow(branch) => {
                    let next = unknown_indices.len();
                    unknown_indices.entry(*branch).or_insert(next);
                }
                CfgValueKind::Ddt { operator, .. }
                | CfgValueKind::Idt { operator, .. }
                | CfgValueKind::Cross { operator, .. }
                | CfgValueKind::Above { operator, .. }
                | CfgValueKind::Timer { operator, .. }
                | CfgValueKind::Limit { operator, .. }
                | CfgValueKind::LimitPrevious { operator, .. } => {
                    let next = operator_indices.len();
                    operator_indices.entry(*operator).or_insert(next);
                }
                CfgValueKind::Staged { slot } => {
                    let next = staged_indices.len();
                    staged_indices.entry(*slot).or_insert(next);
                }
                _ => {}
            }
        }
    }
    for index in 0..artifact.mir.nodes.len() {
        let node = index.into();
        let next = node_indices.len();
        node_indices.entry(node).or_insert(next);
    }
    for index in 0..artifact.mir.branch_unknowns.len() {
        let branch = index.into();
        let next = unknown_indices.len();
        unknown_indices.entry(branch).or_insert(next);
    }

    let lane_signature = |lane: u32| {
        let lane = lane as usize;
        if lane < artifact.mir.nodes.len() {
            format!("n{}", node_indices[&lane.into()])
        } else if lane < artifact.mir.nodes.len() + artifact.mir.branch_unknowns.len() {
            let raw = lane - artifact.mir.nodes.len();
            format!("b{}", unknown_indices[&raw.into()])
        } else {
            format!(
                "c{}",
                lane - artifact.mir.nodes.len() - artifact.mir.branch_unknowns.len()
            )
        }
    };
    let shape_signature = |value: ValueId| {
        let Some(lanes) = function.lanes_of(value) else {
            return String::new();
        };
        let mut out = String::from("[");
        for (index, lane) in lanes.iter().copied().enumerate() {
            if index != 0 {
                out.push(',');
            }
            out.push_str(&lane_signature(lane));
        }
        out.push(']');
        out
    };

    let base_signature = |value_id: ValueId| {
        let value = function.value(value_id);
        let mut out = format!(
            "{:?}{}:",
            schedule.class(value_id),
            shape_signature(value_id)
        );
        match &value.kind {
            CfgValueKind::RealConstant(value) => write!(out, "real:{:016x}", value.to_bits()),
            CfgValueKind::BooleanConstant(value) => write!(out, "bool:{value}"),
            CfgValueKind::BlockParameter => write!(out, "block-param"),
            CfgValueKind::Parameter(parameter) => write!(
                out,
                "parameter:{}",
                artifact.mir.parameters[usize::from(*parameter)].name
            ),
            CfgValueKind::ParameterGiven(parameter) => write!(
                out,
                "parameter-given:{}",
                artifact.mir.parameters[usize::from(*parameter)].name
            ),
            CfgValueKind::EventState(slot) => write!(out, "event-state:{slot}"),
            CfgValueKind::Temperature => write!(out, "temperature"),
            CfgValueKind::ThermalVoltage => write!(out, "thermal-voltage"),
            CfgValueKind::Multiplicity => write!(out, "multiplicity"),
            CfgValueKind::Time => write!(out, "time"),
            CfgValueKind::Analysis(name) => write!(out, "analysis:{name}"),
            CfgValueKind::SimParam { name, .. } => write!(out, "simparam:{name}"),
            CfgValueKind::NodePotential(node) => write!(out, "node:{}", node_indices[node]),
            CfgValueKind::BranchFlow(branch) => {
                write!(out, "branch:{}", branch_indices[branch])
            }
            CfgValueKind::BranchUnknownFlow(branch) => {
                write!(out, "unknown:{}", unknown_indices[branch])
            }
            CfgValueKind::Ddt { operator, .. } => {
                write!(out, "ddt:{}", operator_indices[operator])
            }
            CfgValueKind::DdtScale => write!(out, "ddt-scale"),
            CfgValueKind::Idt { operator, .. } => {
                write!(out, "idt:{}", operator_indices[operator])
            }
            CfgValueKind::IdtScale => write!(out, "idt-scale"),
            CfgValueKind::Cross { operator, .. } => {
                write!(out, "cross:{}", operator_indices[operator])
            }
            CfgValueKind::Above { operator, .. } => {
                write!(out, "above:{}", operator_indices[operator])
            }
            CfgValueKind::Timer { operator, .. } => {
                write!(out, "timer:{}", operator_indices[operator])
            }
            CfgValueKind::Limit {
                operator, selector, ..
            } => write!(out, "limit:{}:{selector}", operator_indices[operator]),
            CfgValueKind::Ddx {
                pos_node, neg_node, ..
            } => write!(
                out,
                "ddx:{:?}:{:?}",
                pos_node.map(|node| node_indices[&node]),
                neg_node.map(|node| node_indices[&node])
            ),
            CfgValueKind::LimitPrevious { operator, .. } => {
                write!(out, "limit-previous:{}", operator_indices[operator])
            }
            CfgValueKind::Unary { op, .. } => write!(out, "unary:{op:?}"),
            CfgValueKind::Binary { op, .. } => write!(out, "binary:{op:?}"),
            CfgValueKind::LaneSplat(value) => {
                write!(out, "lane-splat:{:016x}", value.to_bits())
            }
            CfgValueKind::LaneWiden { .. } => write!(out, "lane-widen"),
            CfgValueKind::LaneBinary { op, .. } => write!(out, "lane-binary:{op:?}"),
            CfgValueKind::LaneScalar { op, .. } => write!(out, "lane-scalar:{op:?}"),
            CfgValueKind::LaneExtract { lane, .. } => {
                write!(out, "lane-extract:{}", lane_signature(*lane))
            }
            CfgValueKind::Staged { slot } => write!(out, "staged:{}", staged_indices[slot]),
        }
        .expect("write value signature");
        out
    };

    let operand_signature =
        |block_index: usize, value: ValueId| match definitions[usize::from(value)] {
            Some((owner, local)) if owner == block_index => format!("local:{local}"),
            _ => format!("external:{}", base_signature(value)),
        };

    let mut regions = Vec::new();
    for (block_index, block_id) in postorder.iter().copied().enumerate() {
        let block = function.block(block_id);
        let mut signature = format!("block:{:?}|", schedule.blocks[usize::from(block_id)]);
        for parameter in &block.params {
            write!(signature, "param:{};", base_signature(*parameter))
                .expect("write block parameter signature");
        }
        let mut newton_instructions = 0usize;
        for instruction in &block.instructions {
            let value = function.value(instruction.result);
            if schedule.class(instruction.result) == InvalidationClass::Newton {
                newton_instructions += 1;
            }
            write!(signature, "value:{}(", base_signature(instruction.result))
                .expect("write block value signature");
            for operand in value.kind.operands() {
                write!(signature, "{},", operand_signature(block_index, operand))
                    .expect("write block operand signature");
            }
            signature.push_str(");");
        }
        let target_signature = |target: BlockId| {
            let target = function.block(target);
            let terminator = match target.terminator {
                CfgTerminator::Jump { .. } => "jump",
                CfgTerminator::Branch { .. } => "branch",
                CfgTerminator::Return => "return",
                CfgTerminator::Unset => "unset",
            };
            format!(
                "{:?}:{}:{}:{terminator}",
                schedule.blocks[usize::from(target.id)],
                target.params.len(),
                target.instructions.len()
            )
        };
        match &block.terminator {
            CfgTerminator::Jump { target, args } => {
                write!(signature, "jump:{}(", target_signature(*target))
                    .expect("write jump signature");
                for argument in args {
                    write!(signature, "{},", operand_signature(block_index, *argument))
                        .expect("write jump argument signature");
                }
                signature.push(')');
            }
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => {
                write!(
                    signature,
                    "branch:{}:{}:{}(",
                    operand_signature(block_index, *condition),
                    target_signature(*then_target),
                    target_signature(*else_target)
                )
                .expect("write branch signature");
                for argument in then_args.iter().chain(else_args) {
                    write!(signature, "{},", operand_signature(block_index, *argument))
                        .expect("write branch argument signature");
                }
                signature.push(')');
            }
            CfgTerminator::Return => signature.push_str("return"),
            CfgTerminator::Unset => signature.push_str("unset"),
        }
        regions.push(KernelRegionMetric {
            fingerprint: blake3::hash(signature.as_bytes()).to_hex().to_string(),
            instruction_count: crate::metrics::usize_to_u64(block.instructions.len()),
            newton_instruction_count: crate::metrics::usize_to_u64(newton_instructions),
        });
    }
    regions
}

/// Every noise magnitude the model declares, as one body.
///
/// Emitted from the *primal* CFG rather than the differentiated one: a noise
/// power is a magnitude, and nothing asks for its derivative. That is the whole
/// saving. The generator this replaces re-derived each magnitude from HIR
/// through a hand-written liveness index and emitted a schedule per source,
/// which is why `r3_cmc` — a resistor — carries 3,722 lines of it for six
/// magnitudes, and why `noise.rs` was 50.8% of the whole generated tree.
///
/// Not folded into the stamp's body and cached, the way the reactive matrix is.
/// `evaluate_noise_sources` is handed the DC solution as an argument and builds
/// its own context from it, so it does not run after a `stamp` at that solution
/// and has no cache to read. Reading one anyway would make the answer depend on
/// a call order the device contract does not promise.
struct NoisePlan {
    function: CfgFunction,
    outputs: Vec<ValueId>,
    sources: Vec<NoiseSourceValues>,
    /// An original pre-optimization value represented by each compacted value.
    /// Used only to prove that a noise preprocessing result is already live in
    /// the stamp slice; absence means sharing is not legal, never that a value
    /// should be kept alive for it.
    origins: Vec<Option<ValueId>>,
    /// Stamp-stage slot buffer prepared locally by the noise evaluator.
    prepared_slots: usize,
    /// Deepest shared stamp preprocessing helper the independent noise path
    /// must run. `None` keeps the original self-contained body.
    shared_through: Option<InvalidationClass>,
}

/// Where one source's magnitudes sit in [`NoisePlan::outputs`].
struct NoiseSourceValues {
    active: usize,
    psd: usize,
    exponent: Option<usize>,
    table: Vec<usize>,
    /// Multiplicity multiplies a current source's power and divides a
    /// potential source's, matching the contribution it was lifted from.
    is_current: bool,
}

/// One matrix's worth of stamps: the rows to write, and where each row's values
/// sit in the shared output list.
struct Stamps {
    rows: Vec<StampRow>,
    /// Parallel to `rows`: `(residual, one per surviving derivative)`, as
    /// indices into [`ModelPlan::outputs`].
    positions: Vec<(usize, Vec<usize>)>,
    /// Parallel to `rows`: where each row's limiter correction landed, for the
    /// rows that have one.
    corrections: Vec<Option<usize>>,
}

/// How one potential contribution maps onto the single solver branch used for
/// its physical node pair.
#[derive(Debug, Clone, Copy)]
struct PotentialEquationPlan {
    branch: usize,
    /// `1` when the contribution uses the group's orientation, `-1` when its
    /// source branch is reversed and its residual/Jacobian must be negated.
    sign: i8,
}

/// A physical potential branch. Canonical MIR intentionally owns one branch
/// unknown per contribution statement; generated devices instead assemble the
/// Verilog-A branch semantics: one physical flow unknown, one structural KCL
/// coupling, and the sum of every active potential contribution on that branch.
struct PotentialBranchGroup {
    pos: Option<NodeId>,
    neg: Option<NodeId>,
    branch: usize,
    equations: Vec<usize>,
    duplicate_branches: Vec<usize>,
}

struct ModelPlan {
    /// One body computing both matrices' worth of values.
    ///
    /// Not two, and the corpus is what settled it: separate simplifications and
    /// separate emissions gave `hisimhv_va` 5.2 MB of stamp against a 2.2 MB
    /// whole body, because the charge in a wide MOSFET depends on nearly
    /// everything the conduction path computes and pruning has nothing to take
    /// away. Sharing is close to free in the other direction — the conduction
    /// Jacobian is `ddt_scale * d(q)/du`, so `d(q)/du` is already an operand it
    /// holds, and asking for it costs one lane read.
    function: CfgFunction,
    outputs: Vec<ValueId>,
    conduction: Stamps,
    /// Empty when no contribution stores charge.
    reactive: Stamps,
    /// The conduction body cut by invalidation class, or empty when the split
    /// was measured not to be worth taking for this model.
    stages: Vec<Stage>,
    slots: usize,
    node_count: usize,
    /// Physical-branch target and orientation per potential equation.
    potential_equations: Vec<Option<PotentialEquationPlan>>,
    /// Stable source-order groups of potential equations sharing a node pair.
    potential_groups: Vec<PotentialBranchGroup>,
    /// Output position of each equation's control-flow activation value.
    activation_positions: Vec<Option<usize>>,
    /// Output position of each event-controlled procedural variable candidate,
    /// in dense accepted-state slot order.
    event_state_candidate_positions: Vec<usize>,
    /// The noise magnitudes, as their own body.
    ///
    /// `None` where the canonical level cannot express them and the generator
    /// being replaced can — which today is exactly a magnitude that reads `ddx`.
    /// Refusing the whole device over it would trade a working noise file for no
    /// device at all, so this one file falls back and the rest does not.
    noise: Option<NoisePlan>,
    /// One history slot per `ddt` in the body, allocated from the CFG.
    ///
    /// Not from `state_file::collect_ddt_slots`, and the reason is worth stating.
    /// That walks `mir.equations` and `hir.statements`; the CFG is lowered from
    /// `hir.body`, the structured region tree. The front end builds those from
    /// *separate copies* of the same expression tree — a two-terminal capacitor
    /// arena holds `ddt` twice, at ids 4 and 8 — so an operator id the CFG
    /// carries is not one that walk ever saw, and every lookup missed. The CFG
    /// is what this backend emits, so it is also what decides how many slots
    /// there are and which is which.
    ddt_slots: HashMap<ExprId, usize>,
    /// One history slot per `idt`, allocated from the CFG for the same reason.
    idt_slots: HashMap<ExprId, usize>,
    /// One accepted/candidate detector slot shared by `cross` and `above`.
    cross_slots: HashMap<ExprId, usize>,
    /// Dense source-order timer ids (timers share one per-instance next-event
    /// bound, but ids preserve diagnostics and emitted-code stability).
    timer_slots: HashMap<ExprId, usize>,
    /// One anchor slot per `$limit`, holding what it returned on the previous
    /// Newton iteration. `Limit` and its `LimitPrevious` readers carry the same
    /// operator id, so they resolve to the same slot by construction.
    limit_slots: HashMap<ExprId, usize>,
    /// Whether Xyce OneStep may split this generated model into a full-weight
    /// dynamic residual and a half-weight static history contribution.
    /// Models with `idt`, nonlinear use of `ddt`, or `ddt`-dependent control
    /// flow remain on OneStep order one rather than changing their equations.
    one_step_dae_split_safe: bool,
}

impl ModelPlan {
    fn build(
        artifact: &CanonicalIrArtifact,
        measurements: &mut MetricsRecorder,
    ) -> Result<Self, RustBackendError> {
        checkpoint_phase(artifact, measurements, PipelinePhase::CfgLowering)?;
        let phase_started = web_time::Instant::now();
        let mut cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).map_err(|diagnostics| {
            let mut reasons: Vec<String> = diagnostics
                .iter()
                .map(|diagnostic| diagnostic.message.to_string())
                .collect();
            reasons.sort();
            reasons.dedup();
            reasons.truncate(4);
            unsupported(
                artifact,
                format!("the body does not lower to a CFG: {}", reasons.join("; ")),
            )
        })?;
        reject_unsupported_kinds(artifact, &cfg.function)?;
        record_phase(
            artifact,
            measurements,
            PipelinePhase::CfgLowering,
            phase_started.elapsed(),
        )?;

        checkpoint_phase(artifact, measurements, PipelinePhase::DerivativePreparation)?;
        let phase_started = web_time::Instant::now();
        // In value order, which is the lowering's order, so the numbering is a
        // property of the model rather than of a hash map's iteration.
        let mut ddt_slots: HashMap<ExprId, usize> = HashMap::new();
        let mut idt_slots: HashMap<ExprId, usize> = HashMap::new();
        let mut limit_slots: HashMap<ExprId, usize> = HashMap::new();
        let mut cross_slots: HashMap<ExprId, usize> = HashMap::new();
        let mut timer_slots: HashMap<ExprId, usize> = HashMap::new();
        for value in &cfg.function.values {
            match &value.kind {
                CfgValueKind::Ddt { operator, .. } => {
                    let next = ddt_slots.len();
                    ddt_slots.entry(*operator).or_insert(next);
                }
                CfgValueKind::Idt { operator, .. } => {
                    let next = idt_slots.len();
                    idt_slots.entry(*operator).or_insert(next);
                }
                CfgValueKind::Cross { operator, .. } | CfgValueKind::Above { operator, .. } => {
                    let next = cross_slots.len();
                    cross_slots.entry(*operator).or_insert(next);
                }
                CfgValueKind::Timer { operator, .. } => {
                    let next = timer_slots.len();
                    timer_slots.entry(*operator).or_insert(next);
                }
                // `LimitPrevious` is included so a `$limit` whose body reads the
                // previous iterate before the `Limit` value is built still finds
                // a slot; both carry the id of the same `$limit` call.
                CfgValueKind::Limit { operator, .. }
                | CfgValueKind::LimitPrevious { operator, .. } => {
                    let next = limit_slots.len();
                    limit_slots.entry(*operator).or_insert(next);
                }
                _ => {}
            }
        }

        // The correction lane goes last, and only where the model limits, so a
        // model without `$limit` carries no lane for it and every other lane
        // index still means "unknown number n".
        let limits = cfg
            .function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::Limit { .. }));
        let seeds: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .chain(limits.then_some(AdSeed::LimiterCorrection))
            .collect();
        let correction_lane = limits.then(|| seeds.len() - 1);

        let residuals: Vec<ValueId> = artifact
            .mir
            .equations
            .iter()
            .map(|equation| cfg.residuals[usize::from(equation.contribution)])
            .collect();
        let activations: Vec<Option<ValueId>> = artifact
            .mir
            .equations
            .iter()
            .map(|equation| {
                (equation.kind == MirEquationKind::Potential)
                    .then(|| cfg.activations[usize::from(equation.contribution)])
                    .flatten()
            })
            .collect();
        // Before differentiation, because a guarded charge is recovered by
        // *adding* a merge to the graph and a value added afterwards would have
        // no derivative. Differentiating a block parameter is the ordinary case,
        // so nothing else has to know this happened.
        let values_reaching_ddt = values_reaching_a_ddt(&cfg.function);
        let ddt_controls_flow = cfg.function.blocks.iter().any(|block| {
            matches!(
                &block.terminator,
                CfgTerminator::Branch { condition, .. }
                    if values_reaching_ddt[usize::from(*condition)]
            )
        });
        let charges = stored_charges(&mut cfg.function, &residuals);
        let one_step_dae_split_safe = idt_slots.is_empty()
            && !ddt_controls_flow
            && residuals.iter().zip(&charges).all(|(residual, charge)| {
                !values_reaching_ddt[usize::from(*residual)] || charge.is_some()
            });
        cfg.function
            .validate()
            .map_err(|error| unsupported(artifact, format!("charge recovery: {error}")))?;
        measurements.metrics_mut().primal_cfg = cfg_structure_metrics(&cfg.function);
        record_phase(
            artifact,
            measurements,
            PipelinePhase::DerivativePreparation,
            phase_started.elapsed(),
        )?;

        checkpoint_phase(artifact, measurements, PipelinePhase::Differentiation)?;
        let phase_started = web_time::Instant::now();
        let mut differentiated =
            match differentiate_with_control(&cfg.function, &seeds, measurements.control()) {
                Ok(function) => function,
                Err(DifferentiationError::Validation(error)) => {
                    return Err(unsupported(artifact, format!("differentiation: {error}")));
                }
                Err(DifferentiationError::Cancelled(error)) => {
                    return Err(RustBackendError::cancelled(
                        artifact.metadata.source_package.as_str(),
                        artifact.mir.module_name.as_str(),
                        error,
                    ));
                }
            };
        record_phase(
            artifact,
            measurements,
            PipelinePhase::Differentiation,
            phase_started.elapsed(),
        )?;

        checkpoint_phase(artifact, measurements, PipelinePhase::DerivativeExtraction)?;
        let phase_started = web_time::Instant::now();
        // Every read-out first, and both bodies' worth of them: taking a lane
        // appends an instruction, so a row taken after a simplification would
        // name values the simplified function does not have.
        let conduction_rows: Vec<Vec<Option<ValueId>>> = residuals
            .iter()
            .map(|residual| differentiated.derivative_row(*residual))
            .collect();
        let reactive_rows: Vec<Vec<Option<ValueId>>> = charges
            .iter()
            .map(|charge| match charge {
                Some(charge) => differentiated.derivative_row(*charge),
                None => Vec::new(),
            })
            .collect();
        measurements.metrics_mut().differentiated_cfg =
            cfg_structure_metrics(&differentiated.function);
        record_phase(
            artifact,
            measurements,
            PipelinePhase::DerivativeExtraction,
            phase_started.elapsed(),
        )?;

        checkpoint_phase(artifact, measurements, PipelinePhase::NoisePlanning)?;
        let phase_started = web_time::Instant::now();
        // Every read-out is taken, so the function has stopped growing and the
        // noise slice can be cut from it.
        //
        // The primal body first, and the differentiated one only if that fails.
        // A magnitude wants no derivative, so cutting from the primal is what
        // keeps the slice to the arithmetic the powers actually name — but
        // `ddx` is a symbolic derivative only the AD pass resolves, and six
        // shipped models read one inside a noise power. Measured both ways over
        // the corpus: always-primal costs those six 5.4 MB of fallback, and
        // always-differentiated costs the other 35 more than it saves the six,
        // because the AD pass leaves bookkeeping the magnitudes can reach.
        let mut noise = plan_noise(artifact, &cfg, &cfg.function)
            .or_else(|| plan_noise(artifact, &cfg, &differentiated.function));
        if let Some(noise) = &noise {
            let parameter_scopes = artifact
                .mir
                .parameters
                .iter()
                .map(|parameter| parameter.scope)
                .collect::<Vec<_>>();
            let noise_schedule = schedule_with_parameter_scopes(&noise.function, &parameter_scopes);
            measurements.metrics_mut().noise_cfg = cfg_structure_metrics(&noise.function);
            measurements.metrics_mut().noise_invalidation_value_count =
                noise_schedule.census().map(crate::metrics::usize_to_u64);
        }
        record_phase(
            artifact,
            measurements,
            PipelinePhase::NoisePlanning,
            phase_started.elapsed(),
        )?;

        // A contribution with no `ddt` stores no charge. Its row is kept and
        // emptied rather than dropped, so both row lists stay parallel to
        // `mir.equations` and an equation's index means the same thing in each.
        let charged = charges.iter().any(Option::is_some);
        let charge_values: Vec<ValueId> = charges
            .iter()
            .zip(&residuals)
            .map(|(charge, residual)| charge.unwrap_or(*residual))
            .collect();

        checkpoint_phase(artifact, measurements, PipelinePhase::StampPlanning)?;
        let phase_started = web_time::Instant::now();
        let mut conduction = plan_stamps(artifact, &residuals, &conduction_rows, correction_lane);
        // The reactive matrix stamps no residual, so a charge's correction lane
        // has nothing to correct: it is split out and dropped rather than
        // written.
        let mut reactive = plan_stamps(artifact, &charge_values, &reactive_rows, correction_lane);
        for row in &mut reactive.rows {
            row.correction = None;
        }
        for (index, charge) in charges.iter().enumerate() {
            if charge.is_none() {
                reactive.rows[index].derivatives.clear();
            }
        }
        if !charged {
            reactive.rows.clear();
        }
        record_phase(
            artifact,
            measurements,
            PipelinePhase::StampPlanning,
            phase_started.elapsed(),
        )?;

        checkpoint_phase(artifact, measurements, PipelinePhase::CfgOptimization)?;
        let phase_started = web_time::Instant::now();
        // One simplification over both, so what the two matrices share is
        // computed once.
        let mut wanted = conduction.wanted();
        let reactive_wanted = reactive.wanted();
        wanted.extend_from_slice(&reactive_wanted);
        let stamp_wanted = wanted.len();
        wanted.extend(activations.iter().flatten().copied());
        let activation_wanted = activations.iter().flatten().count();
        wanted.extend(cfg.event_state_candidates.iter().copied());
        let tracked_primal = (0..cfg.function.values.len())
            .map(ValueId::from)
            .collect::<Vec<_>>();
        let (function, mapped, stamp_primal_values) = optimize_with_control_and_tracking(
            &differentiated.function,
            &wanted,
            &tracked_primal,
            measurements.control(),
        )
        .map_err(|error| {
            RustBackendError::cancelled(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                error,
            )
        })?;
        let conduction_wanted = stamp_wanted - reactive_wanted.len();
        conduction.remap(&mapped[..conduction_wanted]);
        reactive.remap(&mapped[conduction_wanted..stamp_wanted]);
        let activation_end = stamp_wanted + activation_wanted;
        let mut mapped_activations = mapped[stamp_wanted..activation_end].iter().copied();
        let activations = activations
            .iter()
            .map(|activation| {
                activation.map(|_| mapped_activations.next().expect("mapped activation"))
            })
            .collect::<Vec<_>>();
        debug_assert!(mapped_activations.next().is_none());
        let event_state_candidates = mapped[activation_end..].to_vec();
        conduction.drop_zeros(&function);
        reactive.drop_zeros(&function);
        let mut scalar_derivatives = 0usize;
        let mut packed_derivatives = 0usize;
        let mut lane_entries = 0usize;
        let mut max_width = 0usize;
        for value in &function.values {
            let Some(lanes) = function.lanes_of(value.id) else {
                continue;
            };
            if lanes.len() == 1 {
                scalar_derivatives += 1;
            } else {
                packed_derivatives += 1;
            }
            lane_entries = lane_entries.saturating_add(lanes.len());
            max_width = max_width.max(lanes.len());
        }
        let metrics = measurements.metrics_mut();
        metrics.derivative_seed_count = crate::metrics::usize_to_u64(seeds.len());
        metrics.scalar_derivative_value_count = crate::metrics::usize_to_u64(scalar_derivatives);
        metrics.packed_derivative_value_count = crate::metrics::usize_to_u64(packed_derivatives);
        metrics.derivative_lane_entry_count = crate::metrics::usize_to_u64(lane_entries);
        metrics.max_derivative_width = crate::metrics::usize_to_u64(max_width);
        metrics.optimized_cfg = cfg_structure_metrics(&function);
        record_phase(
            artifact,
            measurements,
            PipelinePhase::CfgOptimization,
            phase_started.elapsed(),
        )?;

        checkpoint_phase(artifact, measurements, PipelinePhase::Scheduling)?;
        let phase_started = web_time::Instant::now();
        // The output list both stamps read from, conduction first.
        let mut outputs = Vec::new();
        let conduction = Stamps::place(conduction, &mut outputs);
        let reactive = Stamps::place(reactive, &mut outputs);
        let activation_positions = activations
            .iter()
            .map(|activation| {
                activation.map(|activation| {
                    outputs.push(activation);
                    outputs.len() - 1
                })
            })
            .collect();
        let event_state_candidate_positions = event_state_candidates
            .iter()
            .map(|candidate| {
                outputs.push(*candidate);
                outputs.len() - 1
            })
            .collect();

        let parameter_scopes: Vec<_> = artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.scope)
            .collect();
        let schedule = schedule_with_parameter_scopes(&function, &parameter_scopes);
        measurements.metrics_mut().kernel_regions =
            kernel_region_metrics(artifact, &function, &schedule);
        let structural_guards = structural_guards(&function, &schedule, &parameter_scopes);
        measurements.metrics_mut().model_structural_guard_count = crate::metrics::usize_to_u64(
            structural_guards
                .iter()
                .filter(|guard| guard.class == InvalidationClass::Model)
                .count(),
        );
        measurements.metrics_mut().instance_structural_guard_count = crate::metrics::usize_to_u64(
            structural_guards
                .iter()
                .filter(|guard| guard.class == InvalidationClass::Instance)
                .count(),
        );
        measurements.metrics_mut().structural_guard_newton_values =
            structural_guards.iter().fold(0_u64, |total, guard| {
                total.saturating_add(crate::metrics::usize_to_u64(guard.newton_values))
            });
        // Stage splitting is an optimization only. Some irreducible-looking
        // projections cannot discard a volatile branch test while preserving
        // a unique route through coarser blocks. The original direct CFG is
        // still fully valid and semantically authoritative, so declining that
        // split is the safe result rather than rejecting the model.
        let stages = match split(&function, &schedule, &outputs) {
            Ok(stages) => stages,
            Err(_error) => {
                measurements.metrics_mut().invalidation_split_fallback_count += 1;
                Vec::new()
            }
        };
        let (stages, slots) = if worth_splitting(&function, &stages) {
            let slots = stages
                .iter()
                .flat_map(|stage| stage.exports.iter().map(|(slot, _)| *slot as usize + 1))
                .max()
                .unwrap_or(0);
            (stages, slots)
        } else {
            (Vec::new(), 0)
        };
        if let Some(noise) = &mut noise {
            measurements
                .metrics_mut()
                .noise_shared_preprocess_value_count = share_noise_preprocessing(
                noise,
                &parameter_scopes,
                &schedule,
                &stages,
                &stamp_primal_values,
                slots,
            );
        }

        let branch_of_equation: Vec<Option<usize>> = artifact
            .mir
            .equations
            .iter()
            .map(|equation| {
                artifact
                    .mir
                    .branch_unknowns
                    .iter()
                    .find(|unknown| unknown.equation == equation.id)
                    .map(|unknown| usize::from(unknown.id))
            })
            .collect();
        let (potential_equations, potential_groups) =
            plan_potential_branches(artifact, &branch_of_equation)?;
        record_phase(
            artifact,
            measurements,
            PipelinePhase::Scheduling,
            phase_started.elapsed(),
        )?;

        Ok(Self {
            function,
            outputs,
            conduction,
            reactive,
            stages,
            slots,
            node_count: artifact.mir.nodes.len(),
            potential_equations,
            potential_groups,
            activation_positions,
            event_state_candidate_positions,
            noise,
            ddt_slots,
            idt_slots,
            cross_slots,
            timer_slots,
            limit_slots,
            one_step_dae_split_safe,
        })
    }
}

fn share_noise_preprocessing(
    noise: &mut NoisePlan,
    parameter_scopes: &[crate::semantic::ParameterScope],
    stamp_schedule: &crate::canonical_ir::schedule::Schedule,
    stamp_stages: &[Stage],
    stamp_primal_values: &[Option<ValueId>],
    stamp_slots: usize,
) -> u64 {
    if stamp_stages.is_empty() {
        return 0;
    }
    let noise_schedule = schedule_with_parameter_scopes(&noise.function, parameter_scopes);
    let Ok(noise_stages) = split(&noise.function, &noise_schedule, &noise.outputs) else {
        return 0;
    };
    let before = noise
        .function
        .blocks
        .iter()
        .map(|block| block.instructions.len())
        .sum::<usize>();
    let mut replacements = HashMap::<ValueId, (u32, InvalidationClass)>::new();
    for stage in noise_stages {
        if stage.class > InvalidationClass::Temperature
            || !stamp_stages
                .iter()
                .any(|candidate| candidate.class == stage.class)
        {
            continue;
        }
        for ((_, _), noise_origin) in stage.exports.iter().zip(&stage.export_origins) {
            let Some(original) = noise
                .origins
                .get(usize::from(*noise_origin))
                .copied()
                .flatten()
            else {
                continue;
            };
            let Some(stamp_value) = stamp_primal_values
                .get(usize::from(original))
                .copied()
                .flatten()
            else {
                continue;
            };
            if stamp_schedule.class(stamp_value) != stage.class {
                continue;
            }
            let Some(stamp_stage) = stamp_stages
                .iter()
                .find(|candidate| candidate.class == stage.class)
            else {
                continue;
            };
            let Some((slot, _)) = stamp_stage
                .exports
                .iter()
                .zip(&stamp_stage.export_origins)
                .find_map(|(export, origin)| (*origin == stamp_value).then_some(export))
            else {
                continue;
            };
            replacements.insert(*noise_origin, (*slot, stage.class));
        }
    }
    if replacements.is_empty() {
        return 0;
    }
    let mut function = noise.function.clone();
    let removed_parameters = function
        .blocks
        .iter()
        .map(|block| {
            block
                .params
                .iter()
                .map(|parameter| replacements.contains_key(parameter))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    for block in &mut function.blocks {
        block
            .params
            .retain(|parameter| !replacements.contains_key(parameter));
        let retain_arguments = |target: crate::canonical_ir::BlockId, args: &mut Vec<ValueId>| {
            let removed = &removed_parameters[usize::from(target)];
            let mut position = 0usize;
            args.retain(|_| {
                let keep = !removed.get(position).copied().unwrap_or(false);
                position += 1;
                keep
            });
        };
        match &mut block.terminator {
            CfgTerminator::Jump { target, args } => retain_arguments(*target, args),
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                retain_arguments(*then_target, then_args);
                retain_arguments(*else_target, else_args);
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    for value in &mut function.values {
        let Some((slot, _)) = replacements.get(&value.id) else {
            continue;
        };
        value.kind = CfgValueKind::Staged { slot: *slot };
    }
    for block in &mut function.blocks {
        block
            .instructions
            .retain(|instruction| !replacements.contains_key(&instruction.result));
    }
    if function.validate().is_err() {
        return 0;
    }
    let (function, outputs) = optimize_cfg(&function, &noise.outputs);
    noise.function = function;
    noise.outputs = outputs;
    noise.origins = vec![None; noise.function.values.len()];
    noise.prepared_slots = stamp_slots;
    noise.shared_through = replacements.values().map(|(_, class)| *class).max();
    let after = noise
        .function
        .blocks
        .iter()
        .map(|block| block.instructions.len())
        .sum::<usize>();
    crate::metrics::usize_to_u64(before.saturating_sub(after))
}

/// Match the lowered noise sources to the plan the descriptors come from, and
/// reduce the body to just the magnitudes.
///
/// The correspondence is checked rather than assumed. A source is named by the
/// contribution it was written in and its position among that contribution's
/// sources, because the plan is extracted from a second lowering of the same
/// expressions and shares no expression ids with the body. Where the two
/// disagree this returns `None` rather than guessing: a noise source silently
/// given another source's power would be reported under the wrong mechanism at
/// the wrong branch, which reads as a physics result rather than as a compiler
/// fault. `None` costs the model nothing but the smaller file.
fn plan_noise(
    artifact: &CanonicalIrArtifact,
    cfg: &CfgModel,
    function: &CfgFunction,
) -> Option<NoisePlan> {
    let planned = &artifact.noise_sources.sources;
    // A silent model falls back too: the generator being replaced already emits
    // the empty evaluator for one, and this emitter has no reason to.
    if planned.is_empty() || cfg.noise.len() != planned.len() {
        return None;
    }

    let mut wanted = Vec::new();
    let mut sources = Vec::with_capacity(planned.len());
    for (index, source) in planned.iter().enumerate() {
        let contribution = artifact
            .mir
            .equations
            .get(usize::from(source.equation))
            .map(|equation| equation.contribution)?;
        let ordinal = planned[..index]
            .iter()
            .filter(|earlier| earlier.equation == source.equation)
            .count();
        let lowered = cfg
            .noise
            .iter()
            .find(|lowered| lowered.contribution == contribution && lowered.ordinal == ordinal)?;

        let table_width = source
            .table
            .as_ref()
            .map_or(0, |table| table.operands.len());
        if lowered.kind != source.kind
            || lowered.exponent.is_some() != source.exponent.is_some()
            || lowered.table.len() != table_width
        {
            return None;
        }

        let mut place = |value: ValueId| {
            wanted.push(value);
            wanted.len() - 1
        };
        sources.push(NoiseSourceValues {
            active: place(lowered.active),
            psd: place(lowered.psd),
            exponent: lowered.exponent.map(&mut place),
            table: lowered.table.iter().copied().map(place).collect(),
            is_current: source.is_current,
        });
    }

    let tracked = (0..function.values.len())
        .map(ValueId::from)
        .collect::<Vec<_>>();
    let (mut function, outputs, mapped) = optimize_with_tracking(function, &wanted, &tracked);
    let mut origins = vec![None; function.values.len()];
    for (original, mapped) in tracked.into_iter().zip(mapped) {
        if let Some(mapped) = mapped {
            origins[usize::from(mapped)].get_or_insert(original);
        }
    }

    // One kind cannot appear in this body. `ddt` reads and writes per-instance
    // history and `evaluate_noise_sources` takes `&self`, so a magnitude that
    // depended on one would advance the transient state while the noise
    // analysis merely read it.
    //
    // Whether any `ddt` is *left* is not the question — a charge storing one is
    // the residual of its own contribution, and a residual can survive into a
    // slice on an edge nothing here reads. The question is whether a magnitude
    // reads one, and only reachability answers it.
    let live = reachable(&function, &outputs);
    for (index, value) in function.values.iter_mut().enumerate() {
        // `ddt` reads and writes per-instance history, and a magnitude that
        // reached one would advance the transient state while the noise
        // analysis merely read it. A `ddx` is unresolved, which only happens in
        // the primal body — returning `None` on one is what sends the model
        // round again against the differentiated function.
        if !matches!(
            value.kind,
            CfgValueKind::Ddt { .. } | CfgValueKind::Ddx { .. }
        ) {
            continue;
        }
        if live[index] {
            return None;
        }
        // Nothing the magnitudes read can observe this, by the walk just done,
        // so it becomes a constant rather than a call: left as it was, the body
        // would still advance the history of a charge no noise source mentions.
        value.kind = CfgValueKind::RealConstant(0.0);
    }
    Some(NoisePlan {
        function,
        outputs,
        sources,
        origins,
        prepared_slots: 0,
        shared_through: None,
    })
}

/// Every value `roots` can read, following a block parameter back through the
/// terminators that supply it.
fn reachable(function: &CfgFunction, roots: &[ValueId]) -> Vec<bool> {
    let mut declared: HashMap<ValueId, (usize, usize)> = HashMap::new();
    for (block, data) in function.blocks.iter().enumerate() {
        for (index, parameter) in data.params.iter().enumerate() {
            declared.insert(*parameter, (block, index));
        }
    }

    let mut live = vec![false; function.values.len()];
    let mut work: Vec<ValueId> = roots.to_vec();
    while let Some(value) = work.pop() {
        if std::mem::replace(&mut live[usize::from(value)], true) {
            continue;
        }
        work.extend(function.value(value).kind.operands());
        let Some((block, index)) = declared.get(&value).copied() else {
            continue;
        };
        // A merge reads whatever each edge into it carries, and — because which
        // edge was taken decides the answer — the condition that chose.
        for source in &function.blocks {
            match &source.terminator {
                CfgTerminator::Jump { target, args } if usize::from(*target) == block => {
                    work.extend(args.get(index).copied());
                }
                CfgTerminator::Branch {
                    condition,
                    then_target,
                    then_args,
                    else_target,
                    else_args,
                } => {
                    let mut taken = false;
                    if usize::from(*then_target) == block {
                        work.extend(then_args.get(index).copied());
                        taken = true;
                    }
                    if usize::from(*else_target) == block {
                        work.extend(else_args.get(index).copied());
                        taken = true;
                    }
                    if taken {
                        work.push(*condition);
                    }
                }
                _ => {}
            }
        }
    }
    live
}

/// The rows one matrix writes, before simplification has run.
///
/// `values` is parallel to `mir.equations`, holding whichever quantity this
/// matrix stamps — the residual for conduction, the stored charge for the
/// reactive one.
fn plan_stamps(
    artifact: &CanonicalIrArtifact,
    values: &[ValueId],
    rows: &[Vec<Option<ValueId>>],
    correction_lane: Option<usize>,
) -> StampPlan {
    let mut plan = StampPlan {
        rows: Vec::with_capacity(artifact.mir.equations.len()),
        structurally_absent: 0,
        folded_to_zero: 0,
    };
    for (index, equation) in artifact.mir.equations.iter().enumerate() {
        let row = rows.get(index).cloned().unwrap_or_default();
        plan.rows.push(split_row(
            equation.branch.pos_node,
            equation.branch.neg_node,
            equation.kind,
            values[index],
            row,
            correction_lane,
            &mut plan.structurally_absent,
        ));
    }
    plan
}

fn plan_potential_branches(
    artifact: &CanonicalIrArtifact,
    branch_of_equation: &[Option<usize>],
) -> Result<
    (
        Vec<Option<PotentialEquationPlan>>,
        Vec<PotentialBranchGroup>,
    ),
    RustBackendError,
> {
    let mut equations = vec![None; artifact.mir.equations.len()];
    let mut groups: Vec<PotentialBranchGroup> = Vec::new();

    for (equation_index, equation) in artifact.mir.equations.iter().enumerate() {
        if equation.kind != MirEquationKind::Potential {
            continue;
        }
        let branch = branch_of_equation
            .get(equation_index)
            .copied()
            .flatten()
            .ok_or_else(|| {
                RustBackendError::internal(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    format!("potential equation {equation_index} has no branch unknown"),
                )
            })?;
        let pos = equation.branch.pos_node;
        let neg = equation.branch.neg_node;

        let existing = groups.iter().position(|group| {
            (group.pos == pos && group.neg == neg) || (group.pos == neg && group.neg == pos)
        });
        let (group_index, sign) = match existing {
            Some(group_index) => {
                let group = &mut groups[group_index];
                let sign = if group.pos == pos && group.neg == neg {
                    1
                } else {
                    -1
                };
                group.equations.push(equation_index);
                group.duplicate_branches.push(branch);
                (group_index, sign)
            }
            None => {
                let group_index = groups.len();
                groups.push(PotentialBranchGroup {
                    pos,
                    neg,
                    branch,
                    equations: vec![equation_index],
                    duplicate_branches: Vec::new(),
                });
                (group_index, 1)
            }
        };
        equations[equation_index] = Some(PotentialEquationPlan {
            branch: groups[group_index].branch,
            sign,
        });
    }

    Ok((equations, groups))
}

impl Stamps {
    /// Append this matrix's values to the shared output list, recording where
    /// each row's landed.
    fn place(plan: StampPlan, outputs: &mut Vec<ValueId>) -> Self {
        let mut positions = Vec::with_capacity(plan.rows.len());
        let mut corrections = Vec::with_capacity(plan.rows.len());
        for row in &plan.rows {
            let residual = outputs.len();
            outputs.push(row.residual);
            let derivatives = row
                .derivatives
                .iter()
                .map(|(_, value)| {
                    outputs.push(*value);
                    outputs.len() - 1
                })
                .collect();
            corrections.push(row.correction.map(|value| {
                outputs.push(value);
                outputs.len() - 1
            }));
            positions.push((residual, derivatives));
        }
        Self {
            rows: plan.rows,
            positions,
            corrections,
        }
    }

    /// How many cached values the reactive stamp reads.
    fn width(&self) -> usize {
        self.rows.iter().map(|row| 1 + row.derivatives.len()).sum()
    }
}

impl ModelPlan {
    fn emit_bindings(&self) -> EmitBindings {
        EmitBindings {
            ddt_slots: self.ddt_slots.clone(),
            idt_slots: self.idt_slots.clone(),
            cross_slots: self.cross_slots.clone(),
            timer_slots: self.timer_slots.clone(),
            limit_slots: self.limit_slots.clone(),
            ..bindings()
        }
    }

    fn stamp_file(
        &self,
        artifact: &CanonicalIrArtifact,
        options: &RustTranspileOptions,
        control: &dyn PipelineControl,
    ) -> Result<String, RustBackendError> {
        let mut out = String::new();
        out.push_str(
            "// @generated by rspice-veriloga; do not edit.\n#![allow(dead_code, non_snake_case, unused_imports, unused_mut, unused_parens, unused_variables)]\n\n",
        );
        if self.model_stage().is_some() {
            out.push_str(
                "use super::state::{CanonicalModelValues, Instance, PARAMETER_MODEL_FLAGS};\n",
            );
        } else {
            out.push_str("use super::state::Instance;\n");
        }
        let mut runtime_support = vec![
            "GeneratedEvalContext".to_string(),
            "GeneratedReactiveStamper".to_string(),
            "GeneratedStamper".to_string(),
        ];
        if self
            .stages
            .iter()
            .any(|stage| stage.class != InvalidationClass::Newton && !stage.exports.is_empty())
        {
            runtime_support.push("install_generated_stage_values".to_string());
        }
        let mut lane_types = lane_runtime_types(&self.function);
        lane_types.extend(
            self.stages
                .iter()
                .flat_map(|stage| lane_runtime_types(&stage.function)),
        );
        runtime_support.extend(lane_types);
        runtime_support.extend([
            "evaluate_generated_above".to_string(),
            "evaluate_generated_cross".to_string(),
            "evaluate_generated_timer".to_string(),
            "rspice_eval_ddt".to_string(),
            "rspice_eval_idt".to_string(),
            "rspice_limexp".to_string(),
            "rspice_limited_exp".to_string(),
            "rspice_limited_exp_derivative".to_string(),
        ]);
        let _ = writeln!(
            out,
            "use {}::{{{}}};",
            options.runtime_path,
            runtime_support.join(", ")
        );
        if self.model_stage().is_some() {
            out.push_str(
                "use std::collections::HashMap;\n\
                 use std::sync::{Arc, Mutex, OnceLock, Weak};\n",
            );
        }
        let mut emitted_slot_table = false;
        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton || stage.exports.is_empty() {
                continue;
            }
            self.emit_stage_slot_table(artifact, stage, &mut out)?;
            emitted_slot_table = true;
        }
        if emitted_slot_table {
            out.push('\n');
        }
        for stage in &self.stages {
            if stage.class <= InvalidationClass::Temperature && !stage.exports.is_empty() {
                self.emit_preprocess_helper(artifact, stage, &mut out)?;
            }
        }
        if self.model_stage().is_some() {
            self.emit_model_cache_support(&mut out);
        }
        out.push_str("impl Instance {\n");

        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton
                || (stage.class == InvalidationClass::Model && stage.exports.is_empty())
            {
                continue;
            }
            self.emit_cached_stage(artifact, stage, &mut out)?;
        }
        self.emit_stamp(artifact, control, &mut out)?;
        self.emit_stamp_reactive(&mut out)?;

        out.push_str("}\n");
        Ok(out)
    }

    fn emit_stage_slot_table(
        &self,
        _artifact: &CanonicalIrArtifact,
        stage: &Stage,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let slots = stage
            .exports
            .iter()
            .map(|(slot, _)| *slot)
            .collect::<Vec<_>>();
        let visibility = if stage.class <= InvalidationClass::Temperature {
            "pub(super) "
        } else {
            ""
        };
        let _ = writeln!(
            out,
            "{visibility}const {}: [u32; {}] = [{}];",
            stage_slot_table_name(stage.class),
            slots.len(),
            slots
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        );
        Ok(())
    }

    /// Pure, explicitly-bound preprocessing shared by cached stamping and an
    /// independent fresh noise evaluation.
    fn emit_preprocess_helper(
        &self,
        artifact: &CanonicalIrArtifact,
        stage: &Stage,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let name = preprocess_fn_name(stage.class);
        let produced = stage
            .exports
            .iter()
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        let (body, names) = emit_body(&stage.function, &produced, &self.emit_bindings())
            .map_err(|error| unsupported(artifact, format!("{name}: {error}")))?;
        let _ = writeln!(
            out,
            "pub(super) fn {name}(\n\
             \x20   parameters: &[f64],\n\
             \x20   parameter_given: &[bool],\n\
             \x20   multiplicity: f64,\n\
             \x20   staged: &[f64],\n\
             \x20   temperature: f64,\n\
             \x20   thermal_voltage: f64,\n\
             ) -> [f64; {}] {{",
            produced.len()
        );
        out.push_str(&indent(&body, 1));
        let names = numeric_output_names(&stage.function, &produced, &names);
        let _ = writeln!(out, "    [{}]\n}}\n", names.join(", "));
        Ok(())
    }

    /// A stage coarser than Newton: run it once and cache what later readers
    /// take from it.
    fn emit_cached_stage(
        &self,
        artifact: &CanonicalIrArtifact,
        stage: &Stage,

        out: &mut String,
    ) -> Result<(), RustBackendError> {
        if stage.class == InvalidationClass::Model {
            return self.emit_model_stage(artifact, stage, out);
        }
        let name = stage_fn_name(stage.class);
        let produced: Vec<ValueId> = stage.exports.iter().map(|(_, value)| *value).collect();
        let direct = stage.class > InvalidationClass::Temperature || produced.is_empty();
        let emitted = direct
            .then(|| emit_body(&stage.function, &produced, &self.emit_bindings()))
            .transpose()
            .map_err(|error| unsupported(artifact, format!("{name}: {error}")))?;

        let _ = writeln!(
            out,
            "    fn {name}(&mut self, ctx: &GeneratedEvalContext<'_>) {{"
        );
        match stage.class {
            InvalidationClass::Temperature => out.push_str(
                "        let temperature = ctx.temperature();\n\
                 \x20       let thermal_voltage = ctx.thermal_voltage();\n\
                 \x20       if self.canonical_temperature_valid\n\
                 \x20           && self.canonical_temperature == temperature\n\
                 \x20           && self.canonical_thermal_voltage == thermal_voltage\n\
                 \x20       {\n            return;\n        }\n",
            ),
            InvalidationClass::Instance => out.push_str(
                "        if self.canonical_instance_valid {\n            return;\n        }\n",
            ),
            // Nothing tells `stamp` that a new timestep began, so this one is
            // recomputed rather than cached.
            _ => {}
        }

        if let Some((body, names)) = emitted {
            // Captured through a block so the immutable borrow of the slot
            // array ends before the writes into it begin.
            let _ = writeln!(
                out,
                "        let produced: [f64; {}] = {{",
                produced.len().max(1)
            );
            self.emit_prologue(artifact, &stage.function, 3, out)?;
            out.push_str(&indent(&body, 3));
            if produced.is_empty() {
                out.push_str("            [0.0]\n");
            } else {
                let names = numeric_output_names(&stage.function, &produced, &names);
                let _ = writeln!(out, "            [{}]", names.join(", "));
            }
            out.push_str("        };\n");
        } else {
            let _ = writeln!(
                out,
                "        let produced = {}(\n\
                 \x20           &self.params.values,\n\
                 \x20           &self.param_given[..],\n\
                 \x20           self.multiplicity,\n\
                 \x20           &self.canonical_staged[..],\n\
                 \x20           ctx.temperature(),\n\
                 \x20           ctx.thermal_voltage(),\n\
                 \x20       );",
                preprocess_fn_name(stage.class)
            );
        }
        if !produced.is_empty() {
            let _ = writeln!(
                out,
                "        install_generated_stage_values(&mut self.canonical_staged[..], &produced, &{});",
                stage_slot_table_name(stage.class)
            );
        }
        match stage.class {
            InvalidationClass::Temperature => out.push_str(
                "        self.canonical_temperature = temperature;\n\
                 \x20       self.canonical_thermal_voltage = thermal_voltage;\n\
                 \x20       self.canonical_temperature_valid = true;\n",
            ),
            InvalidationClass::Instance => {
                out.push_str("        self.canonical_instance_valid = true;\n")
            }
            _ => {}
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    fn model_stage(&self) -> Option<&Stage> {
        self.stages
            .iter()
            .find(|stage| stage.class == InvalidationClass::Model && !stage.exports.is_empty())
    }

    fn emit_model_cache_support(&self, out: &mut String) {
        out.push_str(
            "\nstatic CANONICAL_MODEL_CACHE: OnceLock<Mutex<HashMap<Box<[u64]>, \
             Weak<CanonicalModelValues>>>> = OnceLock::new();\n\n\
             fn canonical_model_cache() -> &'static Mutex<HashMap<Box<[u64]>, \
             Weak<CanonicalModelValues>>> {\n\
             \x20   CANONICAL_MODEL_CACHE.get_or_init(|| Mutex::new(HashMap::new()))\n\
             }\n\n\
             fn canonical_model_cache_lookup(key: &[u64]) -> Option<Arc<CanonicalModelValues>> {\n\
             \x20   let mut cache = canonical_model_cache()\n\
             \x20       .lock()\n\
             \x20       .unwrap_or_else(|poisoned| poisoned.into_inner());\n\
             \x20   let found = cache.get(key).and_then(Weak::upgrade);\n\
             \x20   if found.is_none() {\n\
             \x20       cache.remove(key);\n\
             \x20   }\n\
             \x20   found\n\
             }\n\n\
             fn canonical_model_cache_intern(\n\
             \x20   key: Box<[u64]>,\n\
             \x20   candidate: Arc<CanonicalModelValues>,\n\
             ) -> Arc<CanonicalModelValues> {\n\
             \x20   let mut cache = canonical_model_cache()\n\
             \x20       .lock()\n\
             \x20       .unwrap_or_else(|poisoned| poisoned.into_inner());\n\
             \x20   if let Some(existing) = cache.get(key.as_ref()).and_then(Weak::upgrade) {\n\
             \x20       return existing;\n\
             \x20   }\n\
             \x20   cache.retain(|_, values| values.strong_count() > 0);\n\
             \x20   cache.insert(key, Arc::downgrade(&candidate));\n\
             \x20   candidate\n\
             }\n\n",
        );
    }

    fn emit_model_stage(
        &self,
        artifact: &CanonicalIrArtifact,
        stage: &Stage,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let model_key_words = artifact
            .mir
            .parameters
            .iter()
            .filter(|parameter| parameter.scope == crate::semantic::ParameterScope::Model)
            .count()
            .saturating_mul(2);
        let _ = writeln!(
            out,
            "    fn canonical_model_key(&self) -> Box<[u64]> {{\n\
             \x20       let mut key = Vec::with_capacity({model_key_words});"
        );
        out.push_str(
            "        for index in 0..Self::PARAMETER_COUNT {\n\
             \x20           if PARAMETER_MODEL_FLAGS[index] {\n\
             \x20               key.push(self.params.values[index].to_bits());\n\
             \x20               key.push(u64::from(self.param_given[index]));\n\
             \x20           }\n\
             \x20       }\n\
             \x20       key.into_boxed_slice()\n\
             \x20   }\n\n\
             \x20   fn canonical_install_model_values(&mut self, values: \
             Arc<CanonicalModelValues>) {\n",
        );
        let _ = writeln!(
            out,
            "        install_generated_stage_values(&mut self.canonical_staged[..], values.as_ref(), &{});",
            stage_slot_table_name(stage.class)
        );
        out.push_str(
            "        self.canonical_model_values = Some(values);\n\
             \x20   }\n\n\
             \x20   fn canonical_model_stage(&mut self, ctx: &GeneratedEvalContext<'_>) {\n\
             \x20       if self.canonical_model_values.is_some() {\n\
             \x20           return;\n\
             \x20       }\n\
             \x20       let key = self.canonical_model_key();\n\
             \x20       if let Some(values) = canonical_model_cache_lookup(key.as_ref()) {\n\
             \x20           self.canonical_install_model_values(values);\n\
             \x20           return;\n\
             \x20       }\n\
             \x20       let produced: CanonicalModelValues = canonical_model_preprocess(\n\
             \x20           &self.params.values,\n\
             \x20           &self.param_given[..],\n\
             \x20           self.multiplicity,\n\
             \x20           &self.canonical_staged[..],\n\
             \x20           ctx.temperature(),\n\
             \x20           ctx.thermal_voltage(),\n\
             \x20       );\n",
        );
        out.push_str(
            "        let values = canonical_model_cache_intern(key, Arc::new(produced));\n\
             \x20       self.canonical_install_model_values(values);\n\
             \x20   }\n\n",
        );
        Ok(())
    }

    fn emit_stamp(
        &self,
        artifact: &CanonicalIrArtifact,
        control: &dyn PipelineControl,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        out.push_str(
            "    pub fn stamp(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedStamper<'_>) {\n",
        );
        // Cleared per evaluation, so "was this device limited?" is a question
        // about *this* iteration. Only when limiting is on: with it off the flag
        // is never set, and clearing it would erase a damped step recorded by
        // the iteration that is about to be re-judged.
        if !self.limit_slots.is_empty() {
            out.push_str(
                "        if ctx.limiting_enabled() {\n            self.canonical_limit.active = false;\n        }\n",
            );
        }
        for stage in &self.stages {
            if stage.class == InvalidationClass::Newton
                || (stage.class == InvalidationClass::Model && stage.exports.is_empty())
            {
                continue;
            }
            let _ = writeln!(out, "        self.{}(ctx);", stage_fn_name(stage.class));
        }

        let newton = self
            .stages
            .iter()
            .find(|stage| stage.class == InvalidationClass::Newton);
        let function = newton.map_or(&self.function, |stage| &stage.function);
        let (body, values) = self.newton_outputs(artifact, newton, control)?;
        self.emit_prologue(artifact, function, 2, out)?;
        out.push_str(&indent(&body, 2));

        for (slot, position) in self
            .event_state_candidate_positions
            .iter()
            .copied()
            .enumerate()
        {
            let _ = writeln!(
                out,
                "        self.event_state_candidate[{slot}] = {};",
                values[position]
            );
        }

        self.emit_potential_structure(&values, out);

        for (index, row) in self.conduction.rows.iter().enumerate() {
            let (residual, derivatives) = &self.conduction.positions[index];
            let residual = self.corrected_residual(index, &values, *residual);
            let activation = (row.kind == MirEquationKind::Potential)
                .then(|| self.activation_expression(index, &values));
            self.emit_row(
                row,
                &residual,
                &derivatives
                    .iter()
                    .map(|at| values[*at].clone())
                    .collect::<Vec<_>>(),
                index,
                Reactive::No,
                activation.as_deref(),
                out,
            )?;
        }

        // The charge and its derivatives were computed here whether or not
        // anything asked for them, so this is where they are kept.
        for (index, (residual, derivatives)) in self.reactive.positions.iter().enumerate() {
            let mut at = self.reactive_base(index);
            let _ = writeln!(
                out,
                "        self.canonical_reactive[{at}] = {};",
                values[*residual]
            );
            for position in derivatives {
                at += 1;
                let _ = writeln!(
                    out,
                    "        self.canonical_reactive[{at}] = {};",
                    values[*position]
                );
            }
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    /// The reactive matrix, written from what `stamp` already worked out.
    ///
    /// No body at all, and that is the design rather than a shortcut. Two
    /// reasons it has to be this way:
    ///
    /// *Correctness.* An `eval_ddt` call reads and writes per-instance history,
    /// so a second evaluation from here would advance the transient state a
    /// second time for one solve. The tier being replaced avoids it by keeping
    /// the `ddt` calls out of what it shares; keeping no body avoids it
    /// outright.
    ///
    /// *Physics, and it agrees.* The reactive matrix is `d(charge)/d(unknown)`
    /// at the operating point, and an AC sweep holds that point fixed across
    /// every frequency. Recomputing it per point would produce the same numbers
    /// more slowly.
    fn emit_stamp_reactive(&self, out: &mut String) -> Result<(), RustBackendError> {
        out.push_str(
            "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        if self.reactive.rows.is_empty() {
            out.push_str("    }\n\n");
            return Ok(());
        }
        out.push_str("        let multiplicity = self.multiplicity;\n");
        out.push_str("        let cached = &*self.canonical_reactive;\n");
        for (index, row) in self.reactive.rows.iter().enumerate() {
            if row.derivatives.is_empty() {
                continue;
            }
            let base = self.reactive_base(index);
            let derivatives: Vec<String> = (1..=row.derivatives.len())
                .map(|offset| format!("cached[{}]", base + offset))
                .collect();
            self.emit_row(
                row,
                &format!("cached[{base}]"),
                &derivatives,
                index,
                Reactive::Yes,
                None,
                out,
            )?;
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    /// `noise.rs`: the descriptor table, then one body for every magnitude.
    fn noise_file(
        &self,
        artifact: &CanonicalIrArtifact,
        options: &RustTranspileOptions,
    ) -> Result<GeneratedRustFile, RustBackendError> {
        let Some(noise) = &self.noise else {
            return super::noise::generate_noise_file(artifact, options);
        };
        let function = &noise.function;
        let mut out = String::new();
        out.push_str(
            "// @generated by rspice-veriloga; do not edit.\n#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]\n\n\
             use super::state::Instance;\n",
        );
        let _ = writeln!(
            out,
            "use {}::GeneratedEvalContext;\npub use {}::{{GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseKind, GeneratedNoiseVisitor}};\n",
            options.runtime_path, options.runtime_path
        );
        let shared_stages = noise
            .shared_through
            .into_iter()
            .flat_map(|through| {
                self.stages.iter().filter(move |stage| {
                    stage.class <= through
                        && stage.class <= InvalidationClass::Temperature
                        && !stage.exports.is_empty()
                })
            })
            .collect::<Vec<_>>();
        if !shared_stages.is_empty() {
            let mut imports = Vec::with_capacity(shared_stages.len() * 2);
            for stage in &shared_stages {
                imports.push(preprocess_fn_name(stage.class));
                imports.push(stage_slot_table_name(stage.class));
            }
            let _ = writeln!(out, "use super::stamp::{{{}}};", imports.join(", "));
        }
        // Emitted only where the body reaches something in it: on a model whose
        // magnitudes are plain arithmetic it would be an unused import. Two
        // things reach it, and the second is what the corpus
        // found — a resolved `ddx` is a lane read, so a magnitude using one
        // needs the packed-value support `stamp.rs` imports, *and* the clamped
        // exponentials are functions in here rather than method calls, so a
        // magnitude with a diode in it needs the helpers with no packed value
        // anywhere. Twenty of the corpus models are the second case.
        let uses_unary = |target| {
            function
                .values
                .iter()
                .any(|value| matches!(value.kind, CfgValueKind::Unary { op, .. } if op == target))
        };
        let mut math_support = Vec::new();
        if !shared_stages.is_empty() {
            math_support.push("install_generated_stage_values".to_string());
        }
        math_support.extend(lane_runtime_types(function));
        if uses_unary(CfgUnaryOp::LimExp) {
            math_support.push("rspice_limexp".to_string());
        }
        if uses_unary(CfgUnaryOp::LimitedExp) {
            math_support.push("rspice_limited_exp".to_string());
        }
        if uses_unary(CfgUnaryOp::LimitedExpDerivative) {
            math_support.push("rspice_limited_exp_derivative".to_string());
        }
        if !math_support.is_empty() {
            if math_support.len() == 1 {
                let _ = writeln!(out, "use {}::{};", options.runtime_path, math_support[0]);
            } else {
                let _ = writeln!(
                    out,
                    "use {}::{{{}}};",
                    options.runtime_path,
                    math_support.join(", ")
                );
            }
        }
        out.push_str(&super::noise::descriptor_table(artifact));
        out.push_str("\nimpl Instance {\n");
        out.push_str(
            "    pub fn evaluate_noise_sources(&self, ctx: &GeneratedEvalContext<'_>, visitor: &mut dyn GeneratedNoiseVisitor) -> Result<(), GeneratedNoiseEvaluationError> {\n\
             \x20       if !self.multiplicity.is_finite() || self.multiplicity <= 0.0 {\n\
             \x20           return Err(GeneratedNoiseEvaluationError::InvalidMultiplicity { value: self.multiplicity });\n\
             \x20       }\n",
        );
        if !shared_stages.is_empty() {
            let _ = writeln!(
                out,
                "        let mut prepared = [0.0; {}];",
                noise.prepared_slots
            );
            for stage in &shared_stages {
                let _ = writeln!(
                    out,
                    "        let produced = {}(\n\
                     \x20           &self.params.values,\n\
                     \x20           &self.param_given[..],\n\
                     \x20           self.multiplicity,\n\
                     \x20           &prepared[..],\n\
                     \x20           ctx.temperature(),\n\
                     \x20           ctx.thermal_voltage(),\n\
                     \x20       );\n\
                     \x20       install_generated_stage_values(&mut prepared[..], &produced, &{});",
                    preprocess_fn_name(stage.class),
                    stage_slot_table_name(stage.class),
                );
            }
        }
        let (body, values) = emit_body(function, &noise.outputs, &self.emit_bindings())
            .map_err(|error| unsupported(artifact, format!("noise body: {error}")))?;
        self.emit_noise_prologue(artifact, function, &mut out);
        out.push_str(&indent(&body, 2));

        for (index, source) in noise.sources.iter().enumerate() {
            // The guard is the control flow the source was written in, already
            // merged into one value by the lowering. An inactive source still
            // has to be visited: the analysis pairs visits with descriptors by
            // index, so skipping one would shift every source after it.
            //
            // A source that no control flow guards merges to a constant, and
            // most do. Emitting the branch anyway would put an arm that cannot
            // run into every device that declares noise at all.
            let always = match function.value(noise.outputs[source.active]).kind {
                CfgValueKind::RealConstant(active) => Some(active != 0.0),
                CfgValueKind::BooleanConstant(active) => Some(active),
                _ => None,
            };
            if always == Some(false) {
                let _ = writeln!(
                    out,
                    "        if !visitor.visit({index}, GeneratedNoiseEvaluationRef {{ active: false, psd: 0.0, exponent: None, table_operands: &[] }}) {{ return Ok(()); }}"
                );
                continue;
            }
            if always.is_none() {
                let active = truth_output(
                    function,
                    noise.outputs[source.active],
                    &values[source.active],
                );
                let _ = writeln!(
                    out,
                    "        if !({active}) {{\n\
                     \x20           if !visitor.visit({index}, GeneratedNoiseEvaluationRef {{ active: false, psd: 0.0, exponent: None, table_operands: &[] }}) {{ return Ok(()); }}\n\
                     \x20       }} else {{"
                );
            } else {
                out.push_str("        {\n");
            }
            // The order of the checks, and the scaling written as one operation,
            // are the generator this replaces: same rejections, and a power that
            // is bit-identical rather than merely close.
            let _ = writeln!(out, "            let psd = {};", values[source.psd]);
            emit_noise_check(&mut out, index, "psd", "psd");
            // A noise power reaches us signed, and the magnitude is the spectral
            // density. PSP104 is why: it clips its flicker density non-negative
            // (`S_fl = CLIP_LOW(S_fl, 0.0)`) and then contributes
            // `flicker_noise(sigVds * MULT_inst * MULT_FN * S_fl, ...)`, where
            // `sigVds` is literally +-1.0 and goes negative whenever the device
            // is operating in reverse. The sign is there to orient the branch
            // against the model's own internal source/drain swap; it cannot mean
            // anything about the density, because an independent noise source is
            // zero-mean and its orientation is unobservable.
            //
            // Rejecting the negative -- which is what this did -- failed the
            // whole evaluation for four models at ordinary reverse bias.
            // Clamping to zero would be worse than the error: it would silently
            // delete the flicker noise of every reversed PSP device, which is
            // wrong physics rather than a loud stop. The magnitude is the one
            // reading that is right in both directions.
            let _ = writeln!(out, "            let psd = psd.abs();");
            match source.exponent {
                Some(at) => {
                    let _ = writeln!(
                        out,
                        "            let exponent: Option<f64> = Some({});",
                        values[at]
                    );
                    let _ = writeln!(
                        out,
                        "            if let Some(value) = exponent {{ if !value.is_finite() {{ return Err(GeneratedNoiseEvaluationError::NonFinite {{ index: {index}, quantity: \"exponent\", value }}); }} }}"
                    );
                }
                None => out.push_str("            let exponent: Option<f64> = None;\n"),
            }
            for (operand, at) in source.table.iter().enumerate() {
                let _ = writeln!(
                    out,
                    "            let noise_table_operand_{operand} = {};",
                    values[*at]
                );
                emit_noise_check(
                    &mut out,
                    index,
                    &format!("table operand {operand}"),
                    &format!("noise_table_operand_{operand}"),
                );
            }
            let _ = writeln!(
                out,
                "            let table_operands = [{}];",
                (0..source.table.len())
                    .map(|operand| format!("noise_table_operand_{operand}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
            let scaled = if source.is_current {
                "psd * self.multiplicity"
            } else {
                "psd / self.multiplicity"
            };
            let _ = writeln!(out, "            let psd = {scaled};");
            emit_noise_check(&mut out, index, "scaled psd", "psd");
            let _ = writeln!(
                out,
                "            if !visitor.visit({index}, GeneratedNoiseEvaluationRef {{ active: true, psd, exponent, table_operands: &table_operands }}) {{ return Ok(()); }}\n\
                 \x20       }}"
            );
        }
        out.push_str("        Ok(())\n    }\n}\n");

        Ok(GeneratedRustFile {
            relative_path: "noise.rs".to_string(),
            contents: out,
        })
    }

    /// The noise body's bindings.
    ///
    /// Not [`Self::emit_prologue`]: that one forces `multiplicity` in because
    /// every stamper call scales by it, and binds the slot array because a
    /// stamp can read a slot the body never touches. Neither is true here —
    /// this body stamps nothing and runs unstaged — and an unused binding of
    /// `self.canonical_staged` would not even compile on a model that has none.
    fn emit_noise_prologue(
        &self,
        artifact: &CanonicalIrArtifact,
        function: &CfgFunction,
        out: &mut String,
    ) {
        let mut wants = Wants::default();
        for value in &function.values {
            wants.observe(&value.kind);
        }
        if wants.parameters {
            out.push_str("        let parameters = &self.params.values;\n");
        }
        if wants.parameter_given {
            out.push_str("        let parameter_given = &*self.param_given;\n");
        }
        if wants.event_state {
            out.push_str("        let event_state = &*self.event_state_accepted;\n");
        }
        if wants.multiplicity {
            out.push_str("        let multiplicity = self.multiplicity;\n");
        }
        if wants.time {
            out.push_str("        let time = self.time;\n");
        }
        if wants.temperature {
            out.push_str("        let temperature = ctx.temperature();\n");
        }
        if wants.thermal_voltage {
            out.push_str("        let thermal_voltage = ctx.thermal_voltage();\n");
        }
        if wants.staged {
            out.push_str("        let staged = &prepared[..];\n");
        }
        if wants.node_potentials {
            let _ = writeln!(
                out,
                "        let node_potentials = [{}];",
                (0..self.node_count)
                    .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.branch_unknown_flows {
            let _ = writeln!(
                out,
                "        let branch_unknown_flows = [{}];",
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| format!("ctx.branch_current(self.branches[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.ddt_scale {
            out.push_str(
                "        let ddt_scale_value = self.ddt_coefficients.derivative_scale;\n\
                 \x20       let ddt_scale = move || ddt_scale_value;\n",
            );
        }
        // `evaluate_noise_sources` takes `&self`, runs at a fixed operating
        // point, and is not a Newton step — there is nothing to damp and no
        // history it may write. Both bindings are therefore the
        // limiting-disabled ones, which is what the device itself does when
        // `ctx.limiting_enabled()` is false, so a magnitude that reads a limited
        // voltage reads the proposed one.
        if wants.limit {
            out.push_str(
                "        let limit = |_operator: usize, proposed: f64, _candidate: f64| proposed;\n",
            );
        }
        if wants.limit_previous {
            out.push_str(
                "        let limit_previous = |_operator: usize, proposed: f64| proposed;\n",
            );
        }
        if wants.cross {
            out.push_str("        macro_rules! rspice_cross { ($slot:expr, $value:expr, $direction:expr, $time_tol:expr, $expr_tol:expr, $enable:expr) => {{ let _ = ($slot, $value, $direction, $time_tol, $expr_tol, $enable); 0.0 }}; }\n");
        }
        if wants.above {
            out.push_str("        macro_rules! rspice_above { ($slot:expr, $value:expr, $time_tol:expr, $expr_tol:expr, $enable:expr) => {{ let _ = ($slot, $value, $time_tol, $expr_tol, $enable); 0.0 }}; }\n");
        }
        if wants.timer {
            out.push_str("        macro_rules! rspice_timer { ($slot:expr, $start:expr, $period:expr, $time_tol:expr, $enable:expr) => {{ let _ = ($slot, $start, $period, $time_tol, $enable); 0.0 }}; }\n");
        }
    }

    /// Where equation `index`'s reactive values start in the cache.
    fn reactive_base(&self, index: usize) -> usize {
        self.reactive.rows[..index]
            .iter()
            .map(|row| 1 + row.derivatives.len())
            .sum()
    }

    /// The Newton body, and an expression per conduction output.
    ///
    /// When the body is split, an output a coarser stage owns is read from its
    /// slot rather than recomputed, which is the whole point of the split.
    fn newton_outputs(
        &self,
        artifact: &CanonicalIrArtifact,
        newton: Option<&Stage>,
        control: &dyn PipelineControl,
    ) -> Result<(String, Vec<String>), RustBackendError> {
        let Some(newton) = newton else {
            let (body, names) = emit_body(&self.function, &self.outputs, &self.emit_bindings())
                .map_err(|error| unsupported(artifact, format!("body: {error}")))?;
            return Ok((body, names));
        };

        let owned: Vec<(usize, ValueId)> = newton
            .outputs
            .iter()
            .enumerate()
            .filter_map(|(index, value)| value.map(|value| (index, value)))
            .collect();
        let owned_values: Vec<ValueId> = owned.iter().map(|(_, value)| *value).collect();
        let emit_bindings = self.emit_bindings();
        let (body, names) = emit_body(&newton.function, &owned_values, &emit_bindings)
            .map_err(|error| unsupported(artifact, format!("newton stage: {error}")))?;
        let (body, names) = specialize_repeated_static_guards(
            &newton.function,
            &owned_values,
            body,
            names,
            &emit_bindings,
            control,
        )
        .map_err(|error| match error {
            StructuralSpecializationError::Emit(error) => {
                unsupported(artifact, format!("specialized newton stage: {error}"))
            }
            StructuralSpecializationError::Cancelled(error) => RustBackendError::cancelled(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                error,
            ),
        })?;

        let mut values = vec![String::new(); self.outputs.len()];
        for ((index, _), name) in owned.iter().zip(names) {
            values[*index] = name;
        }
        for (index, value) in values.iter_mut().enumerate() {
            if !value.is_empty() {
                continue;
            }
            let slot = self
                .stages
                .iter()
                .find_map(|stage| stage.outputs[index].and_then(|held| stage.slot_of(held)))
                .ok_or_else(|| {
                    // `split` is supposed to make this unreachable by demanding
                    // every output at the deepest class; if it ever is reached,
                    // the alternative is a silent zero in the matrix.
                    unsupported(
                        artifact,
                        format!("stamp output {index} is computed by no stage and cached by none"),
                    )
                })?;
            *value = format!("staged[{slot}]");
        }
        Ok((body, values))
    }

    fn activation_expression(&self, equation: usize, values: &[String]) -> String {
        let position = self.activation_positions[equation]
            .expect("only potential equations request topology activation");
        truth_output(&self.function, self.outputs[position], &values[position])
    }

    fn emit_potential_structure(&self, values: &[String], out: &mut String) {
        for group in &self.potential_groups {
            let active = group
                .equations
                .iter()
                .map(|equation| format!("({})", self.activation_expression(*equation, values)))
                .collect::<Vec<_>>()
                .join(" || ");
            let pos = optional_node(group.pos);
            let neg = optional_node(group.neg);
            let _ = writeln!(
                out,
                "        if {active} {{\n\
                 \x20           stamper.stamp_potential_branch_local({pos}, {neg}, {}, multiplicity);\n\
                 \x20       }} else {{\n\
                 \x20           stamper.stamp_inactive_potential_branch_local({});\n\
                 \x20       }}",
                group.branch, group.branch,
            );
            // MIR keeps one branch-current unknown per contribution statement.
            // Only the source-first unknown represents the physical branch;
            // every duplicate remains in the generated ABI and is pinned to
            // zero current so it cannot make the solver matrix singular.
            for duplicate in &group.duplicate_branches {
                let _ = writeln!(
                    out,
                    "        stamper.stamp_inactive_potential_branch_local({duplicate});"
                );
            }
        }
    }

    /// One equation's stamper calls.
    /// The residual as the matrix wants it, with the limiter's displacement
    /// taken back out.
    ///
    /// The model evaluated its currents at the limited operating point `L`
    /// while the solver is asking about `v`, and the Jacobian reported is
    /// `dI/dv` at `L` by the `dL/dv := 1` convention. Linearising there gives
    /// `I(L) + G*(v - L)`, so what has to be subtracted is `G*(L - v)` — the
    /// directional derivative along the displacement, which is exactly what the
    /// correction lane accumulated. Any `ddt` scaling is already inside it,
    /// because this pass differentiates the residual rather than the charge.
    ///
    /// Guarded on `limiting_enabled` because a probe with limiting off must see
    /// the undamped equations: that is the mode the derivative oracles measure,
    /// and correcting there would make the stamp disagree with its own
    /// currents.
    fn corrected_residual(&self, equation: usize, values: &[String], residual: usize) -> String {
        match self.conduction.corrections.get(equation).copied().flatten() {
            Some(at) => format!(
                "(({}) - (if ctx.limiting_enabled() {{ {} }} else {{ 0.0 }}))",
                values[residual], values[at]
            ),
            None => values[residual].clone(),
        }
    }

    fn emit_row(
        &self,
        row: &StampRow,
        residual: &str,
        derivatives: &[String],
        equation: usize,
        reactive: Reactive,
        activation: Option<&str>,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let mut nodes = Vec::new();
        let mut node_values = Vec::new();
        let mut branches = Vec::new();
        let mut branch_values = Vec::new();
        for ((unknown, _), value) in row.derivatives.iter().zip(derivatives) {
            if *unknown < self.node_count {
                nodes.push(unknown.to_string());
                node_values.push(value.clone());
            } else {
                branches.push((unknown - self.node_count).to_string());
                branch_values.push(value.clone());
            }
        }
        let pos = optional_node(row.pos);
        let neg = optional_node(row.neg);

        match (row.kind, reactive) {
            (MirEquationKind::Current, Reactive::No) => {
                let _ = writeln!(
                    out,
                    "        stamper.stamp_current_sparse_local::<{}, {}>(\n\
                     \x20           {pos},\n            {neg},\n            multiplicity * ({residual}),\n\
                     \x20           [{}],\n            [{}],\n            [{}],\n            [{}],\n\
                     \x20           multiplicity,\n        );",
                    nodes.len(),
                    branches.len(),
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Current, Reactive::Yes) => {
                let _ = writeln!(
                    out,
                    "        stamper.stamp_current_reactive_indexed_dense_local(\n\
                     \x20           {pos},\n            {neg},\n            &[{}],\n            &[{}],\n\
                     \x20           &[{}],\n            &[{}],\n            multiplicity,\n        );",
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Potential, Reactive::No) => {
                let plan = self.potential_equations[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("potential equation {equation} has no physical branch plan"),
                    )
                })?;
                let signed = |value: &str| {
                    if plan.sign < 0 {
                        format!("-({value})")
                    } else {
                        value.to_string()
                    }
                };
                let residual = signed(residual);
                let node_values = node_values
                    .iter()
                    .map(|value| signed(value))
                    .collect::<Vec<_>>();
                let branch_values = branch_values
                    .iter()
                    .map(|value| signed(value))
                    .collect::<Vec<_>>();
                let active = activation.ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("potential equation {equation} has no activation expression"),
                    )
                })?;
                let _ = writeln!(out, "        if {active} {{");
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_sparse_local::<{}, {}>(\n\
                     \x20           {},\n            {residual},\n\
                     \x20           [{}],\n            [{}],\n            [{}],\n            [{}],\n        );",
                    nodes.len(),
                    branches.len(),
                    plan.branch,
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
                out.push_str("        }\n");
            }
            (MirEquationKind::Potential, Reactive::Yes) => {
                let plan = self.potential_equations[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("potential equation {equation} has no physical branch plan"),
                    )
                })?;
                let signed = |value: &str| {
                    if plan.sign < 0 {
                        format!("-({value})")
                    } else {
                        value.to_string()
                    }
                };
                let node_values = node_values
                    .iter()
                    .map(|value| signed(value))
                    .collect::<Vec<_>>();
                let branch_values = branch_values
                    .iter()
                    .map(|value| signed(value))
                    .collect::<Vec<_>>();
                let _ = writeln!(
                    out,
                    "        stamper.stamp_potential_reactive_indexed_dense_local(\n\
                     \x20           {},\n            &[{}],\n            &[{}],\n\
                     \x20           &[{}],\n            &[{}],\n        );",
                    plan.branch,
                    nodes.join(", "),
                    node_values.join(", "),
                    branches.join(", "),
                    branch_values.join(", "),
                );
            }
            (MirEquationKind::Indirect, _) => {}
        }
        Ok(())
    }

    /// Everything an emitted body expects to find in scope.
    ///
    /// Only what the body actually reads: a leaf the function does not carry
    /// would otherwise emit a `ctx` call for a quantity nothing wants, and in
    /// the instance stage there is no bias to read it from.
    fn emit_prologue(
        &self,
        artifact: &CanonicalIrArtifact,
        function: &CfgFunction,
        depth: usize,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        let pad = "    ".repeat(depth);
        let mut wants = Wants::default();
        for value in &function.values {
            wants.observe(&value.kind);
        }
        // Every stamper call scales by it, whether or not the body reads it.
        wants.multiplicity = true;
        // And a stamper call can read a slot the body never touches: an output
        // a coarse stage owns is written straight into the stamp as
        // `staged[..]`. `I(a, c) <+ bias` in a model that also splits is the
        // shape — the Newton body has no staged operand at all, and the slot
        // still has to be in scope.
        wants.staged |= self.slots > 0;

        if wants.parameters {
            let _ = writeln!(out, "{pad}let parameters = &self.params.values;");
        }
        if wants.parameter_given {
            let _ = writeln!(out, "{pad}let parameter_given = &*self.param_given;");
        }
        if wants.event_state {
            let _ = writeln!(out, "{pad}let event_state = &*self.event_state_accepted;");
        }
        if wants.multiplicity {
            let _ = writeln!(out, "{pad}let multiplicity = self.multiplicity;");
        }
        if wants.time {
            let _ = writeln!(out, "{pad}let time = self.time;");
        }
        if wants.temperature {
            let _ = writeln!(out, "{pad}let temperature = ctx.temperature();");
        }
        if wants.thermal_voltage {
            let _ = writeln!(out, "{pad}let thermal_voltage = ctx.thermal_voltage();");
        }
        if wants.staged {
            let _ = writeln!(out, "{pad}let staged = &*self.canonical_staged;");
        }
        if wants.node_potentials {
            let _ = writeln!(
                out,
                "{pad}let node_potentials = [{}];",
                (0..self.node_count)
                    .map(|index| format!("ctx.node_voltage(self.nodes[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.branch_unknown_flows {
            let _ = writeln!(
                out,
                "{pad}let branch_unknown_flows = [{}];",
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| format!("ctx.branch_current(self.branches[{index}])"))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if wants.ddt_scale {
            let _ = writeln!(
                out,
                "{pad}let ddt_scale_value = if ctx.dynamic_operators_enabled() {{ self.ddt_coefficients.derivative_scale }} else {{ 0.0 }};"
            );
            let _ = writeln!(out, "{pad}let ddt_scale = move || ddt_scale_value;");
        }
        if wants.idt_scale {
            // The generalized integration rule defines ddt(y) = input, so the
            // exact idt Jacobian is the reciprocal of the current derivative
            // scale. Candidate evaluation reports malformed active rules; this
            // expression stays finite while that error propagates to the stamp
            // boundary.
            let _ = writeln!(
                out,
                "{pad}let idt_scale_value = if self.ddt_coefficients.active && ctx.dynamic_operators_enabled() && self.ddt_coefficients.derivative_scale.is_finite() && self.ddt_coefficients.derivative_scale != 0.0 {{\n\
                 {pad}    let inverse = 1.0 / self.ddt_coefficients.derivative_scale;\n\
                 {pad}    if inverse.is_finite() {{ inverse }} else {{ 0.0 }}\n\
                 {pad}}} else {{ 0.0 }};\n\
                 {pad}let idt_scale = move || idt_scale_value;"
            );
        }
        // One binding for both operators, and before either closure is built.
        // `idt` used to bind it itself unless `ddt` was going to, which put the
        // binding *after* the closure that read it whenever a model had both —
        // and only PSP-NQS has both, so nothing caught it until the corpus was
        // compiled. The two closures reach disjoint fields through it.
        if wants.ddt || wants.idt {
            let _ = writeln!(out, "{pad}let ddt_state = self.stamp_state.as_mut();");
        }
        if wants.idt {
            for value in &function.values {
                if let CfgValueKind::Idt { operator, .. } = &value.kind {
                    self.idt_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("an idt at {operator} with no generated state slot"),
                        )
                    })?;
                }
            }
            let _ = writeln!(
                out,
                "{pad}let dynamic_operators_enabled = ctx.dynamic_operators_enabled();\n\
                 {pad}let idt_coefficients = self.ddt_coefficients;\n\
                 {pad}let mut idt = |slot: usize, value: f64, ic: f64| -> f64 {{\n\
                 {pad}    if dynamic_operators_enabled {{\n\
                 {pad}        match rspice_eval_idt(\n\
                 {pad}            &mut ddt_state.idt_current,\n\
                 {pad}            &mut ddt_state.idt_candidate_previous,\n\
                 {pad}            &mut ddt_state.idt_input_current,\n\
                 {pad}            &ddt_state.idt_previous,\n\
                 {pad}            &ddt_state.idt_older,\n\
                 {pad}            &ddt_state.idt_input_previous,\n\
                 {pad}            &ddt_state.idt_initialized,\n\
                 {pad}            &mut ddt_state.idt_candidate_valid,\n\
                 {pad}            idt_coefficients,\n\
                 {pad}            slot,\n\
                 {pad}            value,\n\
                 {pad}            ic,\n\
                 {pad}        ) {{\n\
                 {pad}            Ok(candidate) => candidate.value,\n\
                 {pad}            Err(source) => {{\n\
                 {pad}                ctx.report_idt_candidate_error(slot, source);\n\
                 {pad}                0.0\n\
                 {pad}            }}\n\
                 {pad}        }}\n\
                 {pad}    }} else if ddt_state.idt_initialized[slot] {{\n\
                 {pad}        ddt_state.idt_current[slot]\n\
                 {pad}    }} else {{\n\
                 {pad}        ic\n\
                 {pad}    }}\n\
                 {pad}}};"
            );
        }
        if wants.cross {
            for value in &function.values {
                if let CfgValueKind::Cross { operator, .. } = &value.kind {
                    self.cross_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("a cross event at {operator} with no generated detector slot"),
                        )
                    })?;
                }
            }
            out.push_str(&format!(
                "{pad}macro_rules! rspice_cross {{ ($slot:expr, $value:expr, $direction:expr, $time_tol:expr, $expr_tol:expr, $enable:expr) => {{{{\n\
                 {pad}    let slot = $slot;\n\
                 {pad}    match evaluate_generated_cross(self.cross_event_accepted[slot], $value, self.time, $direction, $time_tol, $expr_tol, $enable, ctx.analysis_tran()) {{\n\
                 {pad}        Ok(evaluation) => {{\n\
                 {pad}            self.cross_event_candidate[slot] = evaluation.candidate;\n\
                 {pad}            if let Some(target) = evaluation.refinement_time {{ self.event_refinement_time = self.event_refinement_time.min(target); }}\n\
                 {pad}            evaluation.fired as u8 as f64\n\
                 {pad}        }}\n\
                 {pad}        Err(source) => {{ ctx.report_event_control_error(\"cross\", slot, source); 0.0 }}\n\
                 {pad}    }}\n\
                 {pad}}}}}; }}"
            ));
        }
        if wants.above {
            for value in &function.values {
                if let CfgValueKind::Above { operator, .. } = &value.kind {
                    self.cross_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("an above event at {operator} with no generated detector slot"),
                        )
                    })?;
                }
            }
            out.push_str(&format!(
                "{pad}macro_rules! rspice_above {{ ($slot:expr, $value:expr, $time_tol:expr, $expr_tol:expr, $enable:expr) => {{{{\n\
                 {pad}    let slot = $slot;\n\
                 {pad}    match evaluate_generated_above(self.cross_event_accepted[slot], $value, self.time, $time_tol, $expr_tol, $enable, ctx.analysis_static()) {{\n\
                 {pad}        Ok(evaluation) => {{\n\
                 {pad}            self.cross_event_candidate[slot] = evaluation.candidate;\n\
                 {pad}            if let Some(target) = evaluation.refinement_time {{ self.event_refinement_time = self.event_refinement_time.min(target); }}\n\
                 {pad}            evaluation.fired as u8 as f64\n\
                 {pad}        }}\n\
                 {pad}        Err(source) => {{ ctx.report_event_control_error(\"above\", slot, source); 0.0 }}\n\
                 {pad}    }}\n\
                 {pad}}}}}; }}"
            ));
        }
        if wants.timer {
            out.push_str(&format!(
                "{pad}macro_rules! rspice_timer {{ ($slot:expr, $start:expr, $period:expr, $time_tol:expr, $enable:expr) => {{{{\n\
                 {pad}    let _ = $slot;\n\
                 {pad}    let (fired, next_event) = evaluate_generated_timer($start, $period, $time_tol, $enable, self.time, self.timestep);\n\
                 {pad}    if let Some(target) = next_event {{ self.timer_event_bound_candidate = self.timer_event_bound_candidate.min(target); }}\n\
                 {pad}    fired as u8 as f64\n\
                 {pad}}}}}; }}"
            ));
        }
        if wants.ddt {
            // `ddt` is the one binding that is a call rather than an expression,
            // because it reads and writes per-instance history. Call sites pass
            // the dense slot the backend resolved from the source operator id.
            for value in &function.values {
                if let CfgValueKind::Ddt { operator, .. } = &value.kind {
                    self.ddt_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!("a ddt at {operator} with no generated state slot"),
                        )
                    })?;
                }
            }
            let _ = writeln!(
                out,
                "{pad}let dynamic_operators_enabled = ctx.dynamic_operators_enabled();\n\
                 {pad}let ddt_coefficients = self.ddt_coefficients;\n\
                 {pad}let mut ddt = |slot: usize, value: f64| -> f64 {{\n\
                 {pad}    if dynamic_operators_enabled {{\n\
                 {pad}        match rspice_eval_ddt(\n\
                 {pad}            &mut ddt_state.ddt_current,\n\
                 {pad}            &ddt_state.ddt_previous,\n\
                 {pad}            &ddt_state.ddt_older,\n\
                 {pad}            &ddt_state.ddt_initialized,\n\
                 {pad}            &mut ddt_state.ddt_derivative_current,\n\
                 {pad}            &ddt_state.ddt_derivative_previous,\n\
                 {pad}            &mut ddt_state.ddt_candidate_valid,\n\
                 {pad}            ddt_coefficients,\n\
                 {pad}            slot,\n\
                 {pad}            value,\n\
                 {pad}        ) {{\n\
                 {pad}            Ok(result) => result,\n\
                 {pad}            Err(source) => {{\n\
                 {pad}                ctx.report_ddt_candidate_error(slot, source);\n\
                 {pad}                0.0\n\
                 {pad}            }}\n\
                 {pad}        }}\n\
                 {pad}    }} else {{\n\
                 {pad}        0.0\n\
                 {pad}    }}\n\
                 {pad}}};"
            );
        }
        if wants.limit || wants.limit_previous {
            for value in &function.values {
                let (CfgValueKind::Limit { operator, .. }
                | CfgValueKind::LimitPrevious { operator, .. }) = &value.kind
                else {
                    continue;
                };
                self.limit_slots.get(operator).copied().ok_or_else(|| {
                    unsupported(
                        artifact,
                        format!("a $limit at {operator} with no generated state slot"),
                    )
                })?;
            }
            // The anchors are *copied* out before any write, which is what makes
            // "the value this `$limit` returned on the previous iteration" mean
            // that regardless of emission order. Reading the live array instead
            // would hand a `$limit` its own current value the moment a body read
            // the previous iterate after the limiter had already run, and
            // arrays this small are one register-width memcpy.
            //
            // It also settles the borrow: the reader closes over copies, so the
            // writer can hold the state mutably without the two colliding.
            let _ = writeln!(
                out,
                "{pad}let limiting_enabled = ctx.limiting_enabled();\n\
                 {pad}let limit_state = self.canonical_limit.as_mut();\n\
                 {pad}let limit_anchor = limit_state.previous;\n\
                 {pad}let limit_initialized = limit_state.initialized;"
            );
            if wants.limit_previous {
                let _ = writeln!(
                    out,
                    "{pad}let limit_previous = move |slot: usize, proposed: f64| -> f64 {{\n\
                     {pad}    if limiting_enabled && limit_initialized[slot] {{\n\
                     {pad}        limit_anchor[slot]\n\
                     {pad}    }} else {{\n\
                     {pad}        proposed\n\
                     {pad}    }}\n\
                     {pad}}};"
                );
            }
            if wants.limit {
                let _ = writeln!(
                    out,
                    "{pad}let mut limit = |slot: usize, proposed: f64, candidate: f64| -> f64 {{\n\
                     {pad}    if !limiting_enabled {{\n\
                     {pad}        return proposed;\n\
                     {pad}    }}\n\
                     {pad}    limit_state.active |= candidate != proposed;\n\
                     {pad}    limit_state.previous[slot] = candidate;\n\
                     {pad}    limit_state.initialized[slot] = true;\n\
                     {pad}    candidate\n\
                     {pad}}};"
                );
            }
        }
        Ok(())
    }

    fn state_extensions(&self, artifact: &CanonicalIrArtifact) -> state_file::StateFileExtensions {
        let mut extensions = state_file::StateFileExtensions::default();
        self.push_limit_state_fields(&mut extensions);
        self.push_event_control_state_fields(&mut extensions);
        let reactive = self.reactive.width();
        if reactive > 0 {
            let _ = writeln!(
                extensions.instance_fields,
                "    pub(crate) canonical_reactive: Box<[f64; {reactive}]>,"
            );
            extensions
                .clone_fields
                .push_str("            canonical_reactive: self.canonical_reactive.clone(),\n");
            extensions
                .new_initializers
                .push_str("            canonical_reactive: boxed_zero_f64_array(),\n");
        }
        if self.slots == 0 {
            return extensions;
        }
        let slots = self.slots;
        let shared_model_stage = self.model_stage();
        if let Some(model) = shared_model_stage {
            let width = model.exports.len().max(1);
            let _ = writeln!(
                extensions.support_types,
                "pub(crate) type CanonicalModelValues = [f64; {width}];"
            );
            extensions
                .instance_fields
                .push_str("    pub(crate) canonical_model_values: Option<std::sync::Arc<CanonicalModelValues>>,\n");
            extensions.clone_fields.push_str(
                "            canonical_model_values: self.canonical_model_values.clone(),\n",
            );
            extensions
                .new_initializers
                .push_str("            canonical_model_values: None,\n");
        }
        let _ = write!(
            extensions.instance_fields,
            "    pub(crate) canonical_staged: Box<[f64; {slots}]>,\n\
             \x20   pub(crate) canonical_instance_valid: bool,\n\
             \x20   pub(crate) canonical_temperature_valid: bool,\n\
             \x20   pub(crate) canonical_temperature: f64,\n\
             \x20   pub(crate) canonical_thermal_voltage: f64,\n"
        );
        extensions.clone_fields.push_str(
            "            canonical_staged: self.canonical_staged.clone(),\n\
             \x20           canonical_instance_valid: self.canonical_instance_valid,\n\
             \x20           canonical_temperature_valid: self.canonical_temperature_valid,\n\
             \x20           canonical_temperature: self.canonical_temperature,\n\
             \x20           canonical_thermal_voltage: self.canonical_thermal_voltage,\n",
        );
        extensions.new_initializers.push_str(
            "            canonical_staged: boxed_zero_f64_array(),\n\
             \x20           canonical_instance_valid: false,\n\
             \x20           canonical_temperature_valid: false,\n\
             \x20           canonical_temperature: 0.0,\n\
             \x20           canonical_thermal_voltage: 0.0,\n",
        );
        // Model-card writes invalidate every coarser stage. Per-device geometry
        // cannot affect the model stage, because the schedule proves that
        // boundary from the source parameter attributes.
        if shared_model_stage.is_some() {
            extensions.set_parameter_hook.push_str(
                "if PARAMETER_MODEL_FLAGS[index] {\n    self.canonical_model_values = None;\n}\n",
            );
        }
        extensions.set_parameter_hook.push_str(
            "self.canonical_instance_valid = false;\n\
             self.canonical_temperature_valid = false;\n",
        );
        if artifact.mir.parameters.is_empty() {
            extensions.set_parameter_hook.clear();
        }
        extensions.set_multiplicity_hook.push_str(
            "self.canonical_instance_valid = false;\n\
             self.canonical_temperature_valid = false;\n",
        );
        extensions
    }

    /// Per-instance limiter state: the anchor each `$limit` returned last, and
    /// whether the device was limited at all on this iteration.
    ///
    /// The `active` flag is the engine's, not this backend's — it is how a
    /// device says "do not call this converged, the step I was given was
    /// damped" — so the rollback and checkpoint wiring around it matches the
    /// tier being replaced field for field. Getting that wrong does not fail to
    /// compile; it converges early.
    fn push_limit_state_fields(&self, extensions: &mut state_file::StateFileExtensions) {
        let count = self.limit_slots.len();
        if count == 0 {
            return;
        }
        extensions.support_types.push_str(
            "#[derive(Clone)]\n\
             pub(crate) struct CanonicalLimitState<const N: usize> {\n\
             \x20   pub(crate) previous: [f64; N],\n\
             \x20   pub(crate) initialized: [bool; N],\n\
             \x20   pub(crate) active: bool,\n\
             }\n\n\
             impl<const N: usize> CanonicalLimitState<N> {\n\
             \x20   fn new_box() -> Box<Self> {\n\
             \x20       let mut boxed = Box::<Self>::new_uninit();\n\
             \x20       unsafe {\n\
             \x20           // SAFETY: every field is an f64, a bool, or an array of them; all-zero bytes are valid values.\n\
             \x20           std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);\n\
             \x20           boxed.assume_init()\n\
             \x20       }\n\
             \x20   }\n\
             }\n\n",
        );
        let _ = writeln!(
            extensions.instance_fields,
            "    pub(crate) canonical_limit: Box<CanonicalLimitState<{count}>>,"
        );
        extensions
            .clone_fields
            .push_str("            canonical_limit: self.canonical_limit.clone(),\n");
        extensions
            .new_initializers
            .push_str("            canonical_limit: CanonicalLimitState::new_box(),\n");
        extensions.limiter_converged_expr = "!self.canonical_limit.active".to_string();
        extensions.rollback_value_count = count;
        extensions.rollback_flag_count = count + 1;
        extensions
            .rollback_capture_values
            .push_str("        values.extend_from_slice(&self.canonical_limit.previous);\n");
        extensions.rollback_capture_flags.push_str(
            "        flags.extend_from_slice(&self.canonical_limit.initialized);\n\
             \x20       flags.push(self.canonical_limit.active);\n",
        );
        let _ = write!(
            extensions.rollback_restore_fields,
            "        let (field, remaining) = rollback_values.split_at({count});\n\
             \x20       self.canonical_limit.previous.copy_from_slice(field);\n\
             \x20       rollback_values = remaining;\n\
             \x20       let (field, remaining) = rollback_flags.split_at({count});\n\
             \x20       self.canonical_limit.initialized.copy_from_slice(field);\n\
             \x20       rollback_flags = remaining;\n\
             \x20       let (active, remaining) = rollback_flags.split_first().expect(\"generated limiter rollback active flag\");\n\
             \x20       self.canonical_limit.active = *active;\n\
             \x20       rollback_flags = remaining;\n"
        );
        extensions.checkpoint_capture_fields =
            "            limiter_anchor: self.canonical_limit.previous.to_vec(),\n\
             \x20           limiter_initialized: self.canonical_limit.initialized.to_vec(),\n"
                .to_string();
        let _ = write!(
            extensions.checkpoint_shape_checks,
            "        if state.limiter_anchor.len() != {count} || state.limiter_initialized.len() != {count} {{\n\
             \x20           return Err(format!(\"generated limiter checkpoint shape mismatch: expected {count}, found {{}} / {{}}\", state.limiter_anchor.len(), state.limiter_initialized.len()));\n\
             \x20       }}\n"
        );
        // Restored state is a fresh operating point, not a damped step, so the
        // flag starts clear rather than being carried across.
        extensions.checkpoint_restore_fields.push_str(
            "        self.canonical_limit.previous.copy_from_slice(&state.limiter_anchor);\n\
             \x20       self.canonical_limit.initialized.copy_from_slice(&state.limiter_initialized);\n\
             \x20       self.canonical_limit.active = false;\n",
        );
    }

    fn push_event_control_state_fields(&self, extensions: &mut state_file::StateFileExtensions) {
        let cross_count = self.cross_slots.len();
        let has_timer = !self.timer_slots.is_empty();

        if cross_count > 0 {
            extensions.uses_cross_event_state = true;
            let _ = write!(
                extensions.instance_fields,
                "    pub(crate) cross_event_accepted: Box<[GeneratedCrossState; {cross_count}]>,\n\
                     pub(crate) cross_event_candidate: Box<[GeneratedCrossState; {cross_count}]>,\n\
                     pub(crate) event_refinement_time: f64,\n"
            );
            extensions.clone_fields.push_str(
                "            cross_event_accepted: self.cross_event_accepted.clone(),\n\
                             cross_event_candidate: self.cross_event_candidate.clone(),\n\
                             event_refinement_time: self.event_refinement_time,\n",
            );
            let _ = write!(
                extensions.new_initializers,
                "            cross_event_accepted: Box::new([GeneratedCrossState::INITIAL; {cross_count}]),\n\
                             cross_event_candidate: Box::new([GeneratedCrossState::INITIAL; {cross_count}]),\n\
                             event_refinement_time: f64::INFINITY,\n"
            );
            extensions.begin_event_state_evaluation.push_str(
                "        self.cross_event_candidate.copy_from_slice(&*self.cross_event_accepted);\n\
                         self.event_refinement_time = f64::INFINITY;\n",
            );
            extensions.validate_advance_state.push_str(
                "        for (index, (accepted, candidate)) in self.cross_event_accepted.iter().copied().zip(self.cross_event_candidate.iter().copied()).enumerate() {\n\
                             validate_generated_cross_state(accepted).map_err(|error| format!(\"generated accepted event detector state {index} is invalid: {error}\"))?;\n\
                             validate_generated_cross_state(candidate).map_err(|error| format!(\"generated candidate event detector state {index} is invalid: {error}\"))?;\n\
                             if candidate != accepted && candidate.initialized && candidate.time != self.time {\n\
                                 return Err(format!(\"generated event detector candidate {index} time {} does not equal evaluation time {}\", candidate.time, self.time));\n\
                             }\n\
                         }\n\
                         if self.event_refinement_time != f64::INFINITY && (!self.event_refinement_time.is_finite() || self.event_refinement_time < 0.0 || self.event_refinement_time >= self.time) {\n\
                             return Err(format!(\"generated event refinement {} is not interior to candidate time {}\", self.event_refinement_time, self.time));\n\
                         }\n",
            );
            extensions.apply_advance_state.push_str(
                "        self.cross_event_accepted.copy_from_slice(&*self.cross_event_candidate);\n\
                         self.event_refinement_time = f64::INFINITY;\n",
            );
            let cross_rollback_values = cross_count.saturating_mul(12).saturating_add(1);
            extensions.rollback_value_count = extensions
                .rollback_value_count
                .saturating_add(cross_rollback_values);
            extensions.rollback_capture_values.push_str(
                "        for state in self.cross_event_accepted.iter().copied() { state.append_checkpoint_lanes(&mut values); }\n\
                         for state in self.cross_event_candidate.iter().copied() { state.append_checkpoint_lanes(&mut values); }\n\
                         values.push(self.event_refinement_time);\n",
            );
            let _ = write!(
                extensions.rollback_restore_fields,
                "        for state in self.cross_event_accepted.iter_mut() {{\n\
                             let (lanes, remaining) = rollback_values.split_at(GeneratedCrossState::CHECKPOINT_LANES);\n\
                             *state = GeneratedCrossState::from_checkpoint_lanes(lanes).expect(\"captured generated crossing rollback state\");\n\
                             rollback_values = remaining;\n\
                         }}\n\
                         for state in self.cross_event_candidate.iter_mut() {{\n\
                             let (lanes, remaining) = rollback_values.split_at(GeneratedCrossState::CHECKPOINT_LANES);\n\
                             *state = GeneratedCrossState::from_checkpoint_lanes(lanes).expect(\"captured generated crossing rollback candidate\");\n\
                             rollback_values = remaining;\n\
                         }}\n\
                         let (refinement, remaining) = rollback_values.split_first().expect(\"generated crossing rollback refinement\");\n\
                         self.event_refinement_time = *refinement;\n\
                         rollback_values = remaining;\n"
            );

            let persistent_cross_lanes =
                cross_count.saturating_mul(rspice_veriloga_runtime_checkpoint_lanes());
            extensions.persistent_event_lane_count = extensions
                .persistent_event_lane_count
                .saturating_add(persistent_cross_lanes);
            extensions.checkpoint_event_capture.push_str(
                "        for state in self.cross_event_accepted.iter().copied() { state.append_checkpoint_lanes(&mut event_variables); }\n",
            );
            let _ = write!(
                extensions.checkpoint_event_validate,
                "        let mut generated_event_lanes = &state.event_variables[Self::EVENT_STATE_COUNT..];\n\
                         for index in 0..{cross_count} {{\n\
                             let (lanes, remaining) = generated_event_lanes.split_at(GeneratedCrossState::CHECKPOINT_LANES);\n\
                             GeneratedCrossState::from_checkpoint_lanes(lanes).map_err(|error| format!(\"generated crossing checkpoint slot {{index}}: {{error}}\"))?;\n\
                             generated_event_lanes = remaining;\n\
                         }}\n"
            );
            let _ = write!(
                extensions.checkpoint_event_restore,
                "        let mut generated_event_lanes = &state.event_variables[Self::EVENT_STATE_COUNT..];\n\
                         for target in self.cross_event_accepted.iter_mut() {{\n\
                             let (lanes, remaining) = generated_event_lanes.split_at(GeneratedCrossState::CHECKPOINT_LANES);\n\
                             *target = GeneratedCrossState::from_checkpoint_lanes(lanes)?;\n\
                             generated_event_lanes = remaining;\n\
                         }}\n\
                         self.cross_event_candidate.copy_from_slice(&*self.cross_event_accepted);\n\
                         self.event_refinement_time = f64::INFINITY;\n"
            );
        }

        if has_timer {
            extensions.instance_fields.push_str(
                "    pub(crate) timer_event_bound_accepted: f64,\n\
                     pub(crate) timer_event_bound_candidate: f64,\n",
            );
            extensions.clone_fields.push_str(
                "            timer_event_bound_accepted: self.timer_event_bound_accepted,\n\
                             timer_event_bound_candidate: self.timer_event_bound_candidate,\n",
            );
            extensions.new_initializers.push_str(
                "            timer_event_bound_accepted: f64::INFINITY,\n\
                             timer_event_bound_candidate: f64::INFINITY,\n",
            );
            extensions
                .begin_event_state_evaluation
                .push_str("        self.timer_event_bound_candidate = f64::INFINITY;\n");
            extensions.validate_advance_state.push_str(
                "        if self.timer_event_bound_accepted != f64::INFINITY && (!self.timer_event_bound_accepted.is_finite() || self.timer_event_bound_accepted <= 0.0) {\n\
                             return Err(format!(\"generated accepted timer event bound is invalid: {}\", self.timer_event_bound_accepted));\n\
                         }\n\
                         if self.timer_event_bound_candidate != f64::INFINITY && (!self.timer_event_bound_candidate.is_finite() || self.timer_event_bound_candidate <= self.time) {\n\
                             return Err(format!(\"generated candidate timer event bound {} is not strictly after time {}\", self.timer_event_bound_candidate, self.time));\n\
                         }\n",
            );
            extensions.apply_advance_state.push_str(
                "        self.timer_event_bound_accepted = self.timer_event_bound_candidate;\n",
            );
            extensions.rollback_value_count = extensions.rollback_value_count.saturating_add(2);
            extensions.rollback_capture_values.push_str(
                "        values.push(self.timer_event_bound_accepted);\n\
                                   values.push(self.timer_event_bound_candidate);\n",
            );
            extensions.rollback_restore_fields.push_str(
                "        let (accepted_timer_bound, remaining) = rollback_values.split_first().expect(\"generated accepted timer rollback bound\");\n\
                         self.timer_event_bound_accepted = *accepted_timer_bound;\n\
                         rollback_values = remaining;\n\
                         let (candidate_timer_bound, remaining) = rollback_values.split_first().expect(\"generated candidate timer rollback bound\");\n\
                         self.timer_event_bound_candidate = *candidate_timer_bound;\n\
                         rollback_values = remaining;\n",
            );
            extensions.persistent_event_lane_count =
                extensions.persistent_event_lane_count.saturating_add(1);
            extensions
                .checkpoint_event_capture
                .push_str("        event_variables.push(self.timer_event_bound_accepted);\n");
            extensions.checkpoint_event_validate.push_str(
                "        let timer_bound = *generated_event_lanes.first().expect(\"generated timer checkpoint lane\");\n\
                         if timer_bound != f64::INFINITY && (!timer_bound.is_finite() || timer_bound <= 0.0) { return Err(format!(\"generated timer checkpoint bound is invalid: {timer_bound}\")); }\n\
                         generated_event_lanes = &generated_event_lanes[1..];\n\
                         debug_assert!(generated_event_lanes.is_empty());\n",
            );
            extensions.checkpoint_event_restore.push_str(
                "        self.timer_event_bound_accepted = *generated_event_lanes.first().expect(\"generated timer checkpoint lane\");\n\
                         self.timer_event_bound_candidate = self.timer_event_bound_accepted;\n\
                         generated_event_lanes = &generated_event_lanes[1..];\n\
                         debug_assert!(generated_event_lanes.is_empty());\n",
            );
        } else if cross_count > 0 {
            extensions
                .checkpoint_event_validate
                .push_str("        debug_assert!(generated_event_lanes.is_empty());\n");
            extensions
                .checkpoint_event_restore
                .push_str("        debug_assert!(generated_event_lanes.is_empty());\n");
        }

        if cross_count > 0 {
            extensions.impl_methods.push_str(
                "    #[inline]\n\
                     pub fn transient_event_refinement_time(&self) -> Option<f64> {\n\
                         (self.event_refinement_time.is_finite() && self.event_refinement_time >= 0.0 && self.event_refinement_time < self.time).then_some(self.event_refinement_time)\n\
                     }\n",
            );
        } else {
            extensions.impl_methods.push_str(
                "    #[inline]\n\
                     pub fn transient_event_refinement_time(&self) -> Option<f64> { None }\n",
            );
        }
        if has_timer {
            extensions.impl_methods.push_str(
                "    #[inline]\n\
                     pub fn transient_timer_event_time(&self) -> Option<f64> {\n\
                         (self.timer_event_bound_accepted != f64::INFINITY).then_some(self.timer_event_bound_accepted)\n\
                     }\n\
                     #[inline]\n\
                     pub fn transient_timer_step_bound(&self) -> Option<f64> {\n\
                         let bound = self.timer_event_bound_candidate - self.time;\n\
                         (bound.is_finite() && bound > 0.0).then_some(bound)\n\
                     }\n",
            );
        } else {
            extensions.impl_methods.push_str(
                "    #[inline]\n\
                     pub fn transient_timer_event_time(&self) -> Option<f64> { None }\n\
                     #[inline]\n\
                     pub fn transient_timer_step_bound(&self) -> Option<f64> { None }\n",
            );
        }
    }
}

const fn rspice_veriloga_runtime_checkpoint_lanes() -> usize {
    // Kept as a compiler constant so generated array and checkpoint sizing do
    // not depend on executing emitted-runtime code during compilation.
    6
}

enum StructuralSpecializationError {
    Emit(super::emit::EmitError),
    Cancelled(PipelineCancelled),
}

/// Replace repeated reads of one cached model/instance condition with one
/// dispatch into two complete hot paths.
///
/// This is intentionally not a combinatorial specializer. One condition gives
/// exactly two variants, both outcomes are present, and source growth is capped
/// before the result is accepted. Conditions controlling loops are excluded:
/// turning a loop test into a constant changes the structured shape the emitter
/// relies on and can turn a terminating parameter-bounded loop into an
/// unbounded one.
fn specialize_repeated_static_guards(
    function: &CfgFunction,
    outputs: &[ValueId],
    baseline_body: String,
    baseline_names: Vec<String>,
    bindings: &EmitBindings,
    control: &dyn PipelineControl,
) -> Result<(String, Vec<String>), StructuralSpecializationError> {
    const MIN_REPEATED_BRANCHES: usize = 3;
    const MIN_BODY_BYTES: usize = 8 * 1024;
    const MAX_CANDIDATES: usize = 3;
    const MAX_SOURCE_GROWTH_PERCENT: usize = 2;

    if baseline_body.len() < MIN_BODY_BYTES
        || outputs
            .iter()
            .any(|output| function.lanes_of(*output).is_some())
    {
        return Ok((baseline_body, baseline_names));
    }

    #[derive(Default)]
    struct Candidate {
        branches: usize,
        controls_loop: bool,
    }

    let loop_headers = cfg_loop_headers(function);
    let mut by_slot: HashMap<u32, Candidate> = HashMap::new();
    for block in &function.blocks {
        let CfgTerminator::Branch { condition, .. } = block.terminator else {
            continue;
        };
        let CfgValueKind::Staged { slot } = function.value(condition).kind else {
            continue;
        };
        let candidate = by_slot.entry(slot).or_default();
        candidate.branches = candidate.branches.saturating_add(1);
        candidate.controls_loop |= loop_headers.contains(&block.id);
    }

    let mut candidates: Vec<(u32, usize)> = by_slot
        .into_iter()
        .filter_map(|(slot, candidate)| {
            (!candidate.controls_loop && candidate.branches >= MIN_REPEATED_BRANCHES)
                .then_some((slot, candidate.branches))
        })
        .collect();
    candidates.sort_unstable_by(|(left_slot, left_branches), (right_slot, right_branches)| {
        right_branches
            .cmp(left_branches)
            .then_with(|| left_slot.cmp(right_slot))
    });

    let byte_limit = baseline_body.len().saturating_add(
        baseline_body
            .len()
            .saturating_mul(MAX_SOURCE_GROWTH_PERCENT)
            / 100,
    );
    for (slot, branches) in candidates.into_iter().take(MAX_CANDIDATES) {
        let (true_body, true_names) =
            emit_static_guard_variant(function, outputs, slot, true, bindings, control)?;
        let (false_body, false_names) =
            emit_static_guard_variant(function, outputs, slot, false, bindings, control)?;
        let specialized = render_static_guard_variants(
            slot,
            branches,
            &true_body,
            &true_names,
            &false_body,
            &false_names,
        );
        if specialized.len() <= byte_limit {
            let names = (0..outputs.len())
                .map(|index| format!("canonical_structural_output_{index}"))
                .collect();
            return Ok((specialized, names));
        }
    }

    Ok((baseline_body, baseline_names))
}

fn emit_static_guard_variant(
    function: &CfgFunction,
    outputs: &[ValueId],
    slot: u32,
    outcome: bool,
    bindings: &EmitBindings,
    control: &dyn PipelineControl,
) -> Result<(String, Vec<String>), StructuralSpecializationError> {
    let mut specialized = function.clone();
    let conditions: HashSet<ValueId> = specialized
        .values
        .iter()
        .filter_map(|value| {
            matches!(value.kind, CfgValueKind::Staged { slot: held } if held == slot)
                .then_some(value.id)
        })
        .collect();
    for block in &mut specialized.blocks {
        let CfgTerminator::Branch {
            condition,
            then_target,
            then_args,
            else_target,
            else_args,
        } = block.terminator.clone()
        else {
            continue;
        };
        if !conditions.contains(&condition) {
            continue;
        }
        block.terminator = if outcome {
            CfgTerminator::Jump {
                target: then_target,
                args: then_args,
            }
        } else {
            CfgTerminator::Jump {
                target: else_target,
                args: else_args,
            }
        };
    }
    retain_reachable_blocks(&mut specialized);
    let mut outputs = outputs.to_vec();
    collapse_single_predecessor_parameters(&mut specialized, &mut outputs);
    let (specialized, outputs) = optimize_with_control(&specialized, &outputs, control)
        .map_err(StructuralSpecializationError::Cancelled)?;
    emit_body(&specialized, &outputs, bindings).map_err(StructuralSpecializationError::Emit)
}

fn render_static_guard_variants(
    slot: u32,
    branches: usize,
    true_body: &str,
    true_names: &[String],
    false_body: &str,
    false_names: &[String],
) -> String {
    debug_assert_eq!(true_names.len(), false_names.len());
    let mut out = String::new();
    let _ = writeln!(
        out,
        "    // Bounded structural specialization: one dispatch replaces {branches} \
         repeated static branches without growing this body by more than 2%."
    );
    for index in 0..true_names.len() {
        let _ = writeln!(out, "    let canonical_structural_output_{index}: f64;");
    }
    let _ = writeln!(out, "    if staged[{slot}] != 0.0 {{");
    out.push_str(&indent(true_body, 1));
    for (index, name) in true_names.iter().enumerate() {
        let _ = writeln!(out, "        canonical_structural_output_{index} = {name};");
    }
    out.push_str("    } else {\n");
    out.push_str(&indent(false_body, 1));
    for (index, name) in false_names.iter().enumerate() {
        let _ = writeln!(out, "        canonical_structural_output_{index} = {name};");
    }
    out.push_str("    }\n");
    out
}

fn cfg_loop_headers(function: &CfgFunction) -> HashSet<BlockId> {
    let mut headers = HashSet::new();
    let mut state: HashMap<BlockId, u8> = HashMap::new();
    let mut stack = vec![(function.entry, 0usize)];
    state.insert(function.entry, 1);
    while let Some((block, index)) = stack.pop() {
        let successors = function.block(block).successors();
        if index < successors.len() {
            stack.push((block, index + 1));
            let successor = successors[index];
            match state.get(&successor) {
                Some(1) => {
                    headers.insert(successor);
                }
                Some(_) => {}
                None => {
                    state.insert(successor, 1);
                    stack.push((successor, 0));
                }
            }
        } else {
            state.insert(block, 2);
        }
    }
    headers
}

fn retain_reachable_blocks(function: &mut CfgFunction) {
    let mut reachable = HashSet::from([function.entry]);
    let mut pending = vec![function.entry];
    while let Some(block) = pending.pop() {
        for successor in function.block(block).successors() {
            if reachable.insert(successor) {
                pending.push(successor);
            }
        }
    }

    let mut remap = vec![None; function.blocks.len()];
    let mut blocks = Vec::with_capacity(reachable.len());
    for block in &function.blocks {
        if !reachable.contains(&block.id) {
            continue;
        }
        remap[usize::from(block.id)] = Some(BlockId::from(blocks.len()));
        blocks.push(block.clone());
    }
    for block in &mut blocks {
        block.id = remap[usize::from(block.id)].expect("a reachable block is remapped");
        match &mut block.terminator {
            CfgTerminator::Jump { target, .. } => {
                *target = remap[usize::from(*target)].expect("a reachable target is remapped");
            }
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                *then_target =
                    remap[usize::from(*then_target)].expect("a reachable target is remapped");
                *else_target =
                    remap[usize::from(*else_target)].expect("a reachable target is remapped");
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
    function.entry = remap[usize::from(function.entry)].expect("the entry is reachable");
    function.blocks = blocks;
}

fn collapse_single_predecessor_parameters(function: &mut CfgFunction, outputs: &mut [ValueId]) {
    let mut incoming = vec![0usize; function.blocks.len()];
    for block in &function.blocks {
        for successor in block.successors() {
            incoming[usize::from(successor)] = incoming[usize::from(successor)].saturating_add(1);
        }
    }
    let collapsible: HashSet<BlockId> = function
        .blocks
        .iter()
        .filter(|block| !block.params.is_empty() && incoming[usize::from(block.id)] == 1)
        .map(|block| block.id)
        .collect();
    if collapsible.is_empty() {
        return;
    }

    let mut replacement = vec![None; function.values.len()];
    for source in &function.blocks {
        match &source.terminator {
            CfgTerminator::Jump { target, args } if collapsible.contains(target) => {
                for (param, argument) in function.block(*target).params.iter().zip(args) {
                    replacement[usize::from(*param)] = Some(*argument);
                }
            }
            CfgTerminator::Branch {
                then_target,
                then_args,
                else_target,
                else_args,
                ..
            } => {
                for (target, args) in [
                    (*then_target, then_args.as_slice()),
                    (*else_target, else_args.as_slice()),
                ] {
                    if collapsible.contains(&target) {
                        for (param, argument) in function.block(target).params.iter().zip(args) {
                            replacement[usize::from(*param)] = Some(*argument);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let resolve = |mut value: ValueId| {
        for _ in 0..replacement.len() {
            match replacement[usize::from(value)] {
                Some(next) if next != value => value = next,
                _ => break,
            }
        }
        value
    };

    for value in &mut function.values {
        value.kind.map_operands(&resolve);
    }
    for output in outputs {
        *output = resolve(*output);
    }
    for block in &mut function.blocks {
        if collapsible.contains(&block.id) {
            block.params.clear();
        }
        match &mut block.terminator {
            CfgTerminator::Jump { target, args } => {
                if collapsible.contains(target) {
                    args.clear();
                } else {
                    for argument in args {
                        *argument = resolve(*argument);
                    }
                }
            }
            CfgTerminator::Branch {
                condition,
                then_target,
                then_args,
                else_target,
                else_args,
            } => {
                *condition = resolve(*condition);
                for (target, args) in [(*then_target, then_args), (*else_target, else_args)] {
                    if collapsible.contains(&target) {
                        args.clear();
                    } else {
                        for argument in args {
                            *argument = resolve(*argument);
                        }
                    }
                }
            }
            CfgTerminator::Return | CfgTerminator::Unset => {}
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Reactive {
    No,
    Yes,
}

/// Which bindings a body actually reads.
#[derive(Default)]
struct Wants {
    parameters: bool,
    parameter_given: bool,
    event_state: bool,
    node_potentials: bool,
    branch_unknown_flows: bool,
    temperature: bool,
    thermal_voltage: bool,
    multiplicity: bool,
    time: bool,
    ddt: bool,
    ddt_scale: bool,
    idt: bool,
    idt_scale: bool,
    cross: bool,
    above: bool,
    timer: bool,
    limit: bool,
    limit_previous: bool,
    staged: bool,
}

impl Wants {
    fn observe(&mut self, kind: &CfgValueKind) {
        match kind {
            CfgValueKind::Parameter(_) => self.parameters = true,
            CfgValueKind::ParameterGiven(_) => self.parameter_given = true,
            CfgValueKind::EventState(_) => self.event_state = true,
            CfgValueKind::NodePotential(_) => self.node_potentials = true,
            CfgValueKind::BranchUnknownFlow(_) => self.branch_unknown_flows = true,
            CfgValueKind::Temperature => self.temperature = true,
            CfgValueKind::ThermalVoltage => self.thermal_voltage = true,
            CfgValueKind::Multiplicity => self.multiplicity = true,
            CfgValueKind::Time => self.time = true,
            CfgValueKind::Ddt { .. } => self.ddt = true,
            CfgValueKind::DdtScale => self.ddt_scale = true,
            CfgValueKind::Idt { .. } => self.idt = true,
            CfgValueKind::IdtScale => self.idt_scale = true,
            CfgValueKind::Cross { .. } => self.cross = true,
            CfgValueKind::Above { .. } => self.above = true,
            CfgValueKind::Timer { .. } => self.timer = true,
            CfgValueKind::Limit { .. } => self.limit = true,
            CfgValueKind::LimitPrevious { .. } => self.limit_previous = true,
            CfgValueKind::Staged { .. } => self.staged = true,
            _ => {}
        }
    }
}

/// The charge a contribution stores, if it is a `ddt` and nothing else.
///
/// A residual is an accumulator, so `I(a, b) <+ ddt(q)` arrives as `0 + ddt(q)`
/// and simplification has not run yet — the zero is peeled here rather than
/// relied on. What is deliberately *not* accepted is a residual mixing stored
/// charge with conduction in one statement: separating those needs the reactive
/// part tracked through the arithmetic, and calling the whole expression a
/// charge would put conduction into the reactive matrix.
fn stored_charges(function: &mut CfgFunction, residuals: &[ValueId]) -> Vec<Option<ValueId>> {
    let reaches = values_reaching_a_ddt(function);
    let mut insertions: Vec<(ValueId, ValueId)> = Vec::new();
    let charges: Vec<Option<ValueId>> = residuals
        .iter()
        .map(|residual| {
            resolve_charge(function, &reaches, *residual, 0)
                .and_then(|charge| materialise_charge(function, &charge, &mut insertions))
        })
        .collect();
    // Once, at the end: an instruction list cannot be rebuilt while it is still
    // being read for the next residual, and two residuals routinely share a
    // subexpression.
    apply_insertions(function, &insertions);
    charges
}

/// Splice each built instruction in directly after the one it mirrors.
fn apply_insertions(function: &mut CfgFunction, insertions: &[(ValueId, ValueId)]) {
    if insertions.is_empty() {
        return;
    }
    let mut after: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
    for (anchor, value) in insertions {
        after.entry(*anchor).or_default().push(*value);
    }
    for block in &mut function.blocks {
        if !block
            .instructions
            .iter()
            .any(|instruction| after.contains_key(&instruction.result))
        {
            continue;
        }
        let mut rebuilt = Vec::with_capacity(block.instructions.len() + insertions.len());
        for instruction in std::mem::take(&mut block.instructions) {
            let anchor = instruction.result;
            rebuilt.push(instruction);
            if let Some(values) = after.get(&anchor) {
                rebuilt.extend(
                    values
                        .iter()
                        .map(|result| CfgInstruction { result: *result }),
                );
            }
        }
        block.instructions = rebuilt;
    }
}

impl Charge {
    fn is_nothing(&self) -> bool {
        matches!(self, Self::Nothing)
    }
}

/// How deep a chain of merges to follow before giving up.
///
/// A charge behind more than a handful of nested guards is not a shape any
/// released model has, and a bound is what keeps a loop-carried residual — where
/// a block parameter can reach itself — from recursing forever.
const MAX_CHARGE_MERGE_DEPTH: usize = 8;

/// What a residual stores, worked out before any of it is built.
///
/// Resolution is separated from construction because a merge has to be created
/// bottom-up: the parameter that carries a guarded charge can only be added once
/// every arm's charge exists to be passed on the edges.
enum Charge {
    /// A value the graph already holds — the operand of a `ddt`, or one side of
    /// an operation that carries no charge of its own.
    Value(ValueId),
    /// This path stores nothing. At the top that means the contribution is not
    /// reactive at all; inside a merge it is the arm that was not taken, and
    /// inside a sum it is the conduction half.
    Nothing,
    /// One charge per edge into `block`, in the order [`edges_into`] gives them.
    Merge { block: BlockId, arms: Vec<Charge> },
    /// An operation the charge needs that the graph only has in its `ddt` form.
    ///
    /// `I(db) <+ TYPE * ddt(QD)` stores `TYPE * QD`, and that product exists
    /// nowhere until it is built. It is inserted directly after `anchor` — the
    /// instruction it mirrors — so its operands are in scope exactly where the
    /// original's were, without any dominance question to answer.
    Op {
        anchor: ValueId,
        kind: Box<ChargeOp>,
    },
}

enum ChargeOp {
    Binary {
        op: CfgBinaryOp,
        left: Charge,
        right: Charge,
    },
    Unary {
        op: CfgUnaryOp,
        input: Charge,
    },
}

/// Which values can reach a `ddt` at all.
///
/// A fixed point rather than one pass, because a loop-carried block parameter
/// depends on values computed from itself. It exists so resolution can answer
/// "stores nothing" in O(1) instead of walking a residual's whole expression
/// DAG as a tree — which on a compact model is both exponential and, if it were
/// bounded to stop that, silently wrong for anything deeper than the bound.
fn values_reaching_a_ddt(function: &CfgFunction) -> Vec<bool> {
    let incoming = {
        let mut incoming: HashMap<ValueId, Vec<ValueId>> = HashMap::new();
        for block in &function.blocks {
            for (target, args) in outgoing_edges(block) {
                for (param, argument) in function.block(target).params.iter().zip(args) {
                    incoming.entry(*param).or_default().push(argument);
                }
            }
        }
        incoming
    };
    let mut reaches = vec![false; function.values.len()];
    loop {
        let mut changed = false;
        for value in &function.values {
            let index = usize::from(value.id);
            if reaches[index] {
                continue;
            }
            let reached = match &value.kind {
                CfgValueKind::Ddt { .. } => true,
                CfgValueKind::BlockParameter => incoming
                    .get(&value.id)
                    .is_some_and(|args| args.iter().any(|arg| reaches[usize::from(*arg)])),
                kind => kind
                    .operands()
                    .into_iter()
                    .any(|operand| reaches[usize::from(operand)]),
            };
            if reached {
                reaches[index] = true;
                changed = true;
            }
        }
        if !changed {
            return reaches;
        }
    }
}

fn outgoing_edges(block: &crate::canonical_ir::cfg::CfgBlock) -> Vec<(BlockId, Vec<ValueId>)> {
    match &block.terminator {
        CfgTerminator::Jump { target, args } => vec![(*target, args.clone())],
        CfgTerminator::Branch {
            then_target,
            then_args,
            else_target,
            else_args,
            ..
        } => vec![
            (*then_target, then_args.clone()),
            (*else_target, else_args.clone()),
        ],
        CfgTerminator::Return | CfgTerminator::Unset => Vec::new(),
    }
}

/// `merges` counts only nested merges — arithmetic recurses freely, because in
/// SSA it is a DAG and the only way back to a value already on the stack is
/// through a loop-carried block parameter.
fn resolve_charge(
    function: &CfgFunction,
    reaches: &[bool],
    residual: ValueId,
    merges: usize,
) -> Option<Charge> {
    if !reaches.get(usize::from(residual)).copied().unwrap_or(false) {
        return Some(Charge::Nothing);
    }
    if merges > MAX_CHARGE_MERGE_DEPTH {
        return None;
    }

    match &function.value(residual).kind {
        CfgValueKind::Ddt { input, .. } => Some(Charge::Value(*input)),
        CfgValueKind::RealConstant(constant) if *constant == 0.0 => Some(Charge::Nothing),
        // Linear arithmetic is pushed inside the `ddt`, which is what makes a
        // scaled or summed charge recoverable. `k * ddt(q)` stores `k * q`;
        // `ddt(q1) + ddt(q2)` stores `q1 + q2`. Only the operations that
        // commute with `d/dt` are followed — a product of two charges is not
        // linear in either, so it is refused rather than approximated.
        CfgValueKind::Binary { op, left, right } => {
            let (op, left, right) = (*op, *left, *right);
            let charged_left = resolve_charge(function, reaches, left, merges);
            let charged_right = resolve_charge(function, reaches, right, merges);
            let stores =
                |charge: &Option<Charge>| matches!(charge, Some(charge) if !charge.is_nothing());
            match (op, stores(&charged_left), stores(&charged_right)) {
                // A sum of a conduction term and a charge stores only the
                // charge — the conduction half is already in the residual and
                // the reactive matrix does not want it. That is what the `0 + x`
                // accumulator is, generalised.
                (CfgBinaryOp::Add | CfgBinaryOp::Sub, true, false) => charged_left,
                (CfgBinaryOp::Add, false, true) => charged_right,
                (CfgBinaryOp::Sub, false, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Unary {
                        op: CfgUnaryOp::Neg,
                        input: charged_right?,
                    }),
                }),
                (CfgBinaryOp::Add | CfgBinaryOp::Sub, true, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: charged_right?,
                    }),
                }),
                (CfgBinaryOp::Mul, true, false) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: Charge::Value(right),
                    }),
                }),
                (CfgBinaryOp::Mul, false, true) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: Charge::Value(left),
                        right: charged_right?,
                    }),
                }),
                // Dividing a charge by something that carries none is still
                // linear in the charge; the other way round is not.
                (CfgBinaryOp::Div, true, false) => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Binary {
                        op,
                        left: charged_left?,
                        right: Charge::Value(right),
                    }),
                }),
                (_, false, false) => Some(Charge::Nothing),
                _ => None,
            }
        }
        CfgValueKind::Unary {
            op: op @ CfgUnaryOp::Neg,
            input,
        } => {
            let (op, input) = (*op, *input);
            match resolve_charge(function, reaches, input, merges)? {
                Charge::Nothing => Some(Charge::Nothing),
                charge => Some(Charge::Op {
                    anchor: residual,
                    kind: Box::new(ChargeOp::Unary { op, input: charge }),
                }),
            }
        }
        // A guarded contribution. `I(a, b) <+ ddt(q)` inside an `if` reaches its
        // equation as a merge of the `ddt` from the arm that ran and zero from
        // the arm that did not, so matching the residual against `Ddt` alone
        // finds nothing and the whole reactive contribution disappears — silent
        // in DC, wrong in AC and transient. Self-heating blocks are guarded as a
        // matter of course, so this is the common case rather than an edge one.
        CfgValueKind::BlockParameter => {
            let block = owning_block(function, residual)?;
            let position = function
                .block(block)
                .params
                .iter()
                .position(|param| *param == residual)?;
            let mut arms = Vec::new();
            let mut stores = false;
            for (source, slot) in edges_into(function, block) {
                let argument = *edge_arguments(function, source, slot).get(position)?;
                let arm = resolve_charge(function, reaches, argument, merges + 1)?;
                stores |= !matches!(arm, Charge::Nothing);
                arms.push(arm);
            }
            // Every arm storing nothing is a merge worth building only if some
            // arm stores something; otherwise the contribution is conduction and
            // adding a parameter for it would be noise in the graph.
            stores.then_some(Charge::Merge { block, arms })
        }
        _ => None,
    }
}

fn materialise_charge(
    function: &mut CfgFunction,
    charge: &Charge,
    insertions: &mut Vec<(ValueId, ValueId)>,
) -> Option<ValueId> {
    match charge {
        Charge::Value(value) => Some(*value),
        Charge::Nothing => None,
        Charge::Op { anchor, kind } => {
            let result = match &**kind {
                ChargeOp::Binary { op, left, right } => {
                    let left = materialise_operand(function, left, insertions);
                    let right = materialise_operand(function, right, insertions);
                    push_value(
                        function,
                        CfgValueType::Real,
                        CfgValueKind::Binary {
                            op: *op,
                            left,
                            right,
                        },
                    )
                }
                ChargeOp::Unary { op, input } => {
                    let input = materialise_operand(function, input, insertions);
                    push_value(
                        function,
                        CfgValueType::Real,
                        CfgValueKind::Unary { op: *op, input },
                    )
                }
            };
            insertions.push((*anchor, result));
            Some(result)
        }
        Charge::Merge { block, arms } => {
            // Depth first: an arm that is itself a merge has to own a parameter
            // before this one can name it on an edge.
            let mut arguments = Vec::with_capacity(arms.len());
            for arm in arms {
                let value = match materialise_charge(function, arm, insertions) {
                    Some(value) => value,
                    None => zero_constant(function),
                };
                arguments.push(value);
            }
            let parameter = push_value(function, CfgValueType::Real, CfgValueKind::BlockParameter);
            function.blocks[usize::from(*block)].params.push(parameter);
            for ((source, slot), argument) in
                edges_into(function, *block).into_iter().zip(arguments)
            {
                edge_arguments_mut(function, source, slot).push(argument);
            }
            Some(parameter)
        }
    }
}

/// An operand of a built operation, with "stores nothing" spelled as a zero.
fn materialise_operand(
    function: &mut CfgFunction,
    charge: &Charge,
    insertions: &mut Vec<(ValueId, ValueId)>,
) -> ValueId {
    match materialise_charge(function, charge, insertions) {
        Some(value) => value,
        None => zero_constant(function),
    }
}

/// Which arm of a terminator an edge leaves by.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EdgeSlot {
    Jump,
    Then,
    Else,
}

/// Every edge into `block`, in a deterministic order: by source block id, then
/// then-arm before else-arm. Parameters and arguments are index-aligned, so this
/// order is the contract between resolution and construction.
fn edges_into(function: &CfgFunction, block: BlockId) -> Vec<(BlockId, EdgeSlot)> {
    let mut edges = Vec::new();
    for source in &function.blocks {
        match &source.terminator {
            CfgTerminator::Jump { target, .. } if *target == block => {
                edges.push((source.id, EdgeSlot::Jump));
            }
            CfgTerminator::Branch {
                then_target,
                else_target,
                ..
            } => {
                if *then_target == block {
                    edges.push((source.id, EdgeSlot::Then));
                }
                if *else_target == block {
                    edges.push((source.id, EdgeSlot::Else));
                }
            }
            _ => {}
        }
    }
    edges
}

fn edge_arguments(function: &CfgFunction, source: BlockId, slot: EdgeSlot) -> &[ValueId] {
    match (&function.block(source).terminator, slot) {
        (CfgTerminator::Jump { args, .. }, EdgeSlot::Jump) => args,
        (CfgTerminator::Branch { then_args, .. }, EdgeSlot::Then) => then_args,
        (CfgTerminator::Branch { else_args, .. }, EdgeSlot::Else) => else_args,
        _ => &[],
    }
}

fn edge_arguments_mut(
    function: &mut CfgFunction,
    source: BlockId,
    slot: EdgeSlot,
) -> &mut Vec<ValueId> {
    match (&mut function.blocks[usize::from(source)].terminator, slot) {
        (CfgTerminator::Jump { args, .. }, EdgeSlot::Jump) => args,
        (CfgTerminator::Branch { then_args, .. }, EdgeSlot::Then) => then_args,
        (CfgTerminator::Branch { else_args, .. }, EdgeSlot::Else) => else_args,
        _ => unreachable!("edges_into only reports slots the terminator has"),
    }
}

fn owning_block(function: &CfgFunction, parameter: ValueId) -> Option<BlockId> {
    function
        .blocks
        .iter()
        .find(|block| block.params.contains(&parameter))
        .map(|block| block.id)
}

fn zero_constant(function: &mut CfgFunction) -> ValueId {
    if let Some(existing) = function
        .values
        .iter()
        .find(|value| matches!(value.kind, CfgValueKind::RealConstant(constant) if constant == 0.0))
    {
        return existing.id;
    }
    push_value(
        function,
        CfgValueType::Real,
        CfgValueKind::RealConstant(0.0),
    )
}

fn push_value(function: &mut CfgFunction, value_type: CfgValueType, kind: CfgValueKind) -> ValueId {
    let id = ValueId::from(function.values.len());
    function.values.push(CfgValue {
        id,
        value_type,
        kind,
    });
    id
}

fn reject_unsupported_kinds(
    artifact: &CanonicalIrArtifact,
    function: &CfgFunction,
) -> Result<(), RustBackendError> {
    for value in &function.values {
        if let CfgValueKind::BranchFlow(branch) = &value.kind {
            return Err(unsupported(
                artifact,
                format!("an unresolved flow probe on {branch}"),
            ));
        }
    }
    if artifact
        .mir
        .equations
        .iter()
        .any(|equation| equation.kind == MirEquationKind::Indirect)
    {
        return Err(unsupported(artifact, "an indirect contribution"));
    }
    // Simulator-control tasks, which this backend would otherwise drop in
    // silence rather than refuse.
    //
    // The front end lowers `$bound_step` and `$discontinuity` into hidden
    // variables named after the task, writes them only into the flat statement
    // stream, and the runtime reads them back under those exact names. The
    // structured body carries neither, so the CFG built from it is a faithful
    // graph of a model that has stopped asking for the time step it needs — a
    // model that compiles, runs, and steps wrongly. Refusing sends it to a
    // backend that honours the request instead.
    if let Some(variable) = artifact
        .hir
        .variables
        .iter()
        .find(|variable| SIMULATOR_CONTROL_TASK_VARIABLES.contains(&variable.name.as_str()))
    {
        return Err(unsupported(
            artifact,
            format!("the {} simulator-control task", variable.name),
        ));
    }
    Ok(())
}

/// Hidden variables the front end creates for a simulator-control task, named
/// exactly as `VerilogADevice` reads them back.
const SIMULATOR_CONTROL_TASK_VARIABLES: [&str; 2] = ["$bound_step", "$discontinuity"];

fn stage_fn_name(class: InvalidationClass) -> &'static str {
    match class {
        InvalidationClass::Model => "canonical_model_stage",
        InvalidationClass::Instance => "canonical_instance_stage",
        InvalidationClass::Temperature => "canonical_temperature_stage",
        InvalidationClass::Timestep => "canonical_timestep_stage",
        InvalidationClass::Newton => "canonical_newton_stage",
    }
}

fn preprocess_fn_name(class: InvalidationClass) -> &'static str {
    match class {
        InvalidationClass::Model => "canonical_model_preprocess",
        InvalidationClass::Instance => "canonical_instance_preprocess",
        InvalidationClass::Temperature => "canonical_temperature_preprocess",
        InvalidationClass::Timestep => "canonical_timestep_preprocess",
        InvalidationClass::Newton => "canonical_newton_preprocess",
    }
}

fn stage_slot_table_name(class: InvalidationClass) -> &'static str {
    match class {
        InvalidationClass::Model => "CANONICAL_MODEL_STAGE_SLOTS",
        InvalidationClass::Instance => "CANONICAL_INSTANCE_STAGE_SLOTS",
        InvalidationClass::Temperature => "CANONICAL_TEMPERATURE_STAGE_SLOTS",
        InvalidationClass::Timestep => "CANONICAL_TIMESTEP_STAGE_SLOTS",
        InvalidationClass::Newton => "CANONICAL_NEWTON_STAGE_SLOTS",
    }
}

/// A branch endpoint, as the stamper wants it.
///
/// The *local* ordinal, not `self.nodes[..]`. The stamper resolves a node to a
/// matrix axis through its own per-instance cache, which is keyed by the
/// model's node numbering; handing it the global index would look plausible and
/// address a different node. `ctx.node_voltage`, by contrast, does take the
/// global one — the two are a real distinction and not interchangeable.
fn optional_node(node: Option<crate::canonical_ir::NodeId>) -> String {
    node.map(|node| format!("Some({})", usize::from(node)))
        .unwrap_or_else(|| "None".to_string())
}

/// Reject a magnitude that is not a number before the analysis integrates it.
fn emit_noise_check(out: &mut String, index: usize, quantity: &str, value: &str) {
    let _ = writeln!(
        out,
        "            if !({value}).is_finite() {{ return Err(GeneratedNoiseEvaluationError::NonFinite {{ index: {index}, quantity: {quantity:?}, value: {value} }}); }}"
    );
}

/// Stage caches have an `f64` ABI even when the CFG value they carry is a
/// predicate. Keep Boolean values native inside the generated body and perform
/// the exact Verilog-A numeric conversion only as they cross that ABI boundary.
fn numeric_output_names(
    function: &CfgFunction,
    outputs: &[ValueId],
    names: &[String],
) -> Vec<String> {
    outputs
        .iter()
        .zip(names)
        .map(|(value, name)| {
            if function.value(*value).value_type == CfgValueType::Boolean {
                format!("{name} as u8 as f64")
            } else {
                name.clone()
            }
        })
        .collect()
}

fn truth_output(function: &CfgFunction, value: ValueId, name: &str) -> String {
    if function.value(value).value_type == CfgValueType::Boolean {
        name.to_string()
    } else {
        format!("{name} != 0.0")
    }
}

fn bindings() -> EmitBindings {
    EmitBindings {
        analysis: "ctx.analysis".into(),
        simparam: "ctx.simparam_or".into(),
        cross: "rspice_cross!".into(),
        above: "rspice_above!".into(),
        timer: "rspice_timer!".into(),
        ..EmitBindings::default()
    }
}

fn indent(body: &str, levels: usize) -> String {
    let pad = "\t".repeat(levels);
    let mut out = String::with_capacity(body.len() + body.len() / 8);
    for line in body.lines() {
        if !line.is_empty() {
            out.push_str(&pad);
            out.push_str(line);
        }
        out.push('\n');
    }
    out
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

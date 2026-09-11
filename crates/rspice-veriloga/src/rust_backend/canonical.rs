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
//! ## Small-signal dynamics
//!
//! Linear `ddt` terms share charge derivatives with the transient Jacobian.
//! Nonlinear expressions and nested `ddt`/`idt` operators instead retain the
//! dynamic coefficients of that Jacobian, with primal values fixed at the
//! operating point. Both routes compute their coefficients in the main body;
//! frequency stamping reads the cache without replaying model histories.
//!
//! ## What it refuses
//!
//! Unsupported operators and unresolved flow probes
//! produce explicit diagnostics. Timestep and discontinuity controls retain
//! their trial and accepted state in the generated device contract.

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

use crate::canonical_ir::charge::recover_stored_charges;
pub(crate) use crate::canonical_ir::charge::{stored_charges, values_reaching_a_ddt};

use crate::canonical_ir::ad::{DifferentiationError, differentiate_with_control_for_roots};
use crate::canonical_ir::cfg::{
    CfgBinaryOp, CfgFunction, CfgTerminator, CfgUnaryOp, CfgValueKind, CfgValueType,
};
use crate::canonical_ir::cfg_lower::CfgModel;
use crate::canonical_ir::cfg_opt::{
    optimize_with_control, optimize_with_control_and_tracking, optimize_with_tracking,
};
use crate::canonical_ir::frequency::{self, DynamicPower, FrequencyError};
use crate::canonical_ir::schedule::{
    InvalidationClass, Stage, schedule_with_parameter_scopes, split, structural_guards,
    worth_splitting,
};
use crate::canonical_ir::{
    AdSeed, BlockId, CanonicalIrArtifact, CanonicalValueType, ExprId, HirAnalogOperator,
    HirExprKind, MirEquationKind, NodeId, SourceSpanRef, ValueId, optimize_cfg,
};
use crate::metrics::{
    CfgStructureMetrics, KernelRegionMetric, MetricsRecorder, PipelineCancelled, PipelineControl,
    PipelinePhase,
};

use super::emit::{EmitBindings, emit_body, lane_runtime_types, math_runtime_imports};
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
    let accepted_state_shape_identity = plan.accepted_state_shape_identity(artifact)?;
    let state_extensions = plan.state_extensions(artifact, options);
    let state = state_file::generate_state_file_with_extensions(
        artifact,
        options,
        &parameter_fields,
        plan.ddt_slots.len(),
        plan.idt_slots.len(),
        plan.one_step_dae_split_safe,
        plan.requires_nodeset_phase,
        artifact.mir.branch_unknowns.len(),
        accepted_state_shape_identity,
        &state_extensions,
    )?;
    record_phase(
        artifact,
        measurements,
        PipelinePhase::StateEmission,
        phase_started.elapsed(),
    )?;
    checkpoint_phase(artifact, measurements, PipelinePhase::NoiseEmission)?;
    let phase_started = web_time::Instant::now();
    let noise = plan.noise_file(artifact, options, measurements.control())?;
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
        source_identity: artifact.metadata.source_identity.to_string(),
        accepted_state_shape_identity,
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
                CfgValueKind::Staged { slot } => {
                    let next = staged_indices.len();
                    staged_indices.entry(*slot).or_insert(next);
                }
                _ => {}
            }
            if let Some(site) = function.value(value_id).kind.state_site() {
                let next = operator_indices.len();
                operator_indices.entry(site.0).or_insert(next);
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
            CfgValueKind::AnalogTask(task) => write!(
                out,
                "analog-task:{}",
                serde_json::to_string(task).expect("serializable task")
            ),
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
            CfgValueKind::PortConnected(port) => write!(out, "port-connected:{port}"),
            CfgValueKind::EventState(slot) => write!(out, "event-state:{slot}"),
            CfgValueKind::Temperature => write!(out, "temperature"),
            CfgValueKind::ThermalVoltage => write!(out, "thermal-voltage"),
            CfgValueKind::Multiplicity => write!(out, "multiplicity"),
            CfgValueKind::Time => write!(out, "time"),
            CfgValueKind::Analysis(name) => write!(out, "analysis:{name}"),
            CfgValueKind::SimParamValue(parameter) => {
                write!(out, "simparam-value:{}", parameter.name())
            }
            CfgValueKind::SimParamPresent(parameter) => {
                write!(out, "simparam-present:{}", parameter.name())
            }
            CfgValueKind::NodePotential(node) => write!(out, "node:{}", node_indices[node]),
            CfgValueKind::BranchFlow(branch) => {
                write!(out, "branch:{}", branch_indices[branch])
            }
            CfgValueKind::BranchUnknownFlow(branch) => {
                write!(out, "unknown:{}", unknown_indices[branch])
            }
            // Only the executable lowering emits one, and this signature
            // describes what the generated backend emits. Written out in full
            // anyway, because a signature that is total cannot collide two
            // different values onto one string.
            CfgValueKind::ContributedCurrent { pos, neg, through } => {
                let endpoint = |node: &Option<NodeId>| match node {
                    Some(node) => node_indices[node].to_string(),
                    None => "ground".to_string(),
                };
                write!(
                    out,
                    "contributed-current:{}:{}:{}",
                    endpoint(pos),
                    endpoint(neg),
                    usize::from(*through)
                )
            }
            CfgValueKind::NoiseProcess(process) => write!(out, "noise-process:{process}"),
            CfgValueKind::Ddt { operator, .. } => {
                write!(out, "ddt:{}", operator_indices[operator])
            }
            CfgValueKind::DdtScale => write!(out, "ddt-scale"),
            CfgValueKind::Idt { operator, .. } => {
                write!(out, "idt:{}", operator_indices[operator])
            }
            CfgValueKind::IdtScale => write!(out, "idt-scale"),
            CfgValueKind::IntegralDerivative { operator, wrap, .. } => write!(
                out,
                "integral-derivative {operator} wrapped={}",
                wrap.is_some()
            ),
            CfgValueKind::IdtMod { operator, .. } => {
                write!(out, "idtmod:{}", operator_indices[operator])
            }
            CfgValueKind::AbsDelay { operator, .. } => {
                write!(out, "absdelay:{}", operator_indices[operator])
            }
            CfgValueKind::AbsDelayDerivative {
                operator, order, ..
            } => write!(
                out,
                "absdelay-derivative:{}:{order}",
                operator_indices[operator]
            ),
            CfgValueKind::Slew { operator, .. } => {
                write!(out, "slew:{}", operator_indices[operator])
            }
            CfgValueKind::SlewDerivative { operator, .. } => {
                write!(out, "slew-derivative:{}", operator_indices[operator])
            }
            CfgValueKind::LastCrossing { operator, .. } => {
                write!(out, "last-crossing:{}", operator_indices[operator])
            }
            CfgValueKind::Laplace { operator, .. } => {
                write!(out, "laplace:{}", operator_indices[operator])
            }
            CfgValueKind::LaplaceDerivative { operator, .. } => {
                write!(out, "laplace-derivative:{}", operator_indices[operator])
            }
            CfgValueKind::Zi { operator, .. } => {
                write!(out, "zi:{}", operator_indices[operator])
            }
            CfgValueKind::ZiDerivative { operator, .. } => {
                write!(out, "zi-derivative:{}", operator_indices[operator])
            }
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
            CfgValueKind::Ddx { axis, .. } => match axis {
                crate::canonical_ir::cfg::CfgDdxAxis::Potential { pos_node, neg_node } => write!(
                    out,
                    "ddx:potential:{:?}:{:?}",
                    pos_node.map(|node| node_indices[&node]),
                    neg_node.map(|node| node_indices[&node])
                ),
                crate::canonical_ir::cfg::CfgDdxAxis::BranchFlow { unknown, reversed } => {
                    write!(out, "ddx:flow:{}:{reversed}", unknown.index())
                }
            },
            CfgValueKind::LimitPrevious { operator, .. } => {
                write!(out, "limit-previous:{}", operator_indices[operator])
            }
            CfgValueKind::Unary { op, .. } => write!(out, "unary:{op:?}"),
            CfgValueKind::Binary { op, .. } => write!(out, "binary:{op:?}"),
            CfgValueKind::SumProductsDiv { terms, .. } => {
                write!(out, "sum-products-div:{}", terms.len())
            }
            CfgValueKind::LaneSumProductsDiv { terms, .. } => {
                write!(out, "lane-sum-products-div:{}", terms.len())
            }
            CfgValueKind::Select { .. } => write!(out, "select"),
            CfgValueKind::IntegerArithmetic { op, .. } => write!(out, "integer-arithmetic:{op:?}"),
            CfgValueKind::IntegerBitwise { op, .. } => write!(out, "integer-bitwise:{op:?}"),
            CfgValueKind::IntegerBitwiseNot { .. } => write!(out, "integer-bitwise-not"),
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

            // A digital value never reaches an emitted analog device — the
            // backend refuses a module with processes before it gets here —
            // but the signature is defined anyway, so that a future artifact
            // carrying one hashes to something rather than panicking.
            CfgValueKind::FourStateConstant(value) => {
                write!(out, "four-state:{}", value.spelling())
            }
            CfgValueKind::IntegerConstant(value) => write!(out, "integer:{value}"),
            CfgValueKind::DigitalSignalRead { signal } => {
                write!(out, "digital-read:{signal}")
            }
            CfgValueKind::DigitalRealSignalRead { signal } => {
                write!(out, "digital-real-read:{signal}")
            }
            CfgValueKind::DigitalRepeatCount { signed, .. } => {
                write!(out, "digital-repeat-count:{signed}")
            }
            CfgValueKind::DigitalDelayTicks { input, signed } => {
                write!(out, "digital-delay-ticks:{input:?}:{signed}")
            }
            CfgValueKind::DigitalTime { query } => write!(out, "digital-time:{query:?}"),
            CfgValueKind::DigitalAnalogFlow { probe } => write!(out, "digital-analog-flow:{probe}"),
            CfgValueKind::DigitalAnalogPotential { probe } => {
                write!(out, "digital-analog-potential:{probe}")
            }
            CfgValueKind::DigitalRealArithmetic { op, .. } => {
                write!(out, "digital-real-arithmetic:{op:?}")
            }
            CfgValueKind::DigitalRealCompare { op, .. } => {
                write!(out, "digital-real-compare:{op:?}")
            }
            CfgValueKind::DigitalExpression { .. } => write!(out, "digital-expression"),
            CfgValueKind::DigitalRealSelect { .. } => write!(out, "digital-real-select"),
            CfgValueKind::DigitalRealToBits { .. } => write!(out, "digital-real-to-bits"),
            CfgValueKind::DigitalBitsToReal { .. } => write!(out, "digital-bits-to-real"),
            CfgValueKind::DigitalIntegerToReal { signed, .. } => {
                write!(out, "digital-integer-to-real:{signed}")
            }
            CfgValueKind::DigitalRealToInteger { width, .. } => {
                write!(out, "digital-real-to-integer:{width}")
            }
            CfgValueKind::DigitalBitwise { op, .. } => write!(out, "digital-bitwise:{op:?}"),
            CfgValueKind::DigitalBitwiseNot { .. } => write!(out, "digital-bitwise-not"),
            CfgValueKind::DigitalLogical { op, .. } => write!(out, "digital-logical:{op:?}"),
            CfgValueKind::DigitalLogicalNot { .. } => write!(out, "digital-logical-not"),
            CfgValueKind::DigitalEquality { negate, .. } => {
                write!(out, "digital-equality:{negate}")
            }
            CfgValueKind::DigitalCaseMatch { kind, .. } => {
                write!(out, "digital-case-match:{}", kind.keyword())
            }
            CfgValueKind::DigitalRelational { op, .. } => {
                write!(out, "digital-relational:{op:?}")
            }
            CfgValueKind::DigitalArithmetic { op, .. } => {
                write!(out, "digital-arithmetic:{op:?}")
            }
            CfgValueKind::DigitalShift { op, .. } => write!(out, "digital-shift:{op:?}"),
            CfgValueKind::DigitalBitSelect { bounds, signed, .. } => {
                write!(out, "digital-bit-select:{bounds:?}:{signed}")
            }
            CfgValueKind::DigitalPartSelect { msb, lsb, .. } => {
                write!(out, "digital-part-select:{msb}:{lsb}")
            }
            CfgValueKind::DigitalConcat { parts } => {
                write!(out, "digital-concat:{}", parts.len())
            }
            CfgValueKind::DigitalSelect { .. } => write!(out, "digital-select"),
            CfgValueKind::DigitalDriverWrite { driver, target, .. } => {
                write!(
                    out,
                    "digital-driver-write:{}:{}:{target:?}",
                    usize::from(driver.signal),
                    driver.index
                )
            }
            CfgValueKind::DigitalBlockingWrite { target, .. } => {
                write!(out, "digital-blocking-write:{target:?}")
            }
            CfgValueKind::DigitalNonblockingWrite {
                target,
                region,
                wait,
                ..
            } => {
                write!(
                    out,
                    "digital-nonblocking-write:{target:?}:{}:{wait:?}",
                    region.name()
                )
            }
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
                CfgTerminator::Wait { .. } => "wait",
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
            CfgTerminator::Wait {
                wait,
                resume,
                resume_args,
            } => {
                write!(signature, "wait:{:?}:{}(", wait, target_signature(*resume))
                    .expect("write wait signature");
                for argument in resume_args {
                    write!(signature, "{},", operand_signature(block_index, *argument))
                        .expect("write wait argument signature");
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

/// How one potential or indirect contribution maps onto the solver branch used for
/// its declared identity or unnamed node pair.
#[derive(Debug, Clone, Copy)]
struct BranchEquationPlan {
    branch: usize,
    /// `1` when the contribution uses the group's orientation, `-1` when its
    /// source branch is reversed. Indirect equations use -1 because the shared
    /// row stamper subtracts a source value while their expression is a residual.
    sign: i8,
}

/// Structural KCL coupling and the equations sharing one MIR branch unknown.
struct SourceBranchGroup {
    kind: MirEquationKind,
    pos: Option<NodeId>,
    neg: Option<NodeId>,
    branch: usize,
    equations: Vec<usize>,
}

struct InitializationPlan {
    phase: rspice_veriloga_runtime::AnalogEvaluationPhase,
    function: CfgFunction,
    outputs: Vec<ValueId>,
    state_count: usize,
}

/// Context inputs that can change independently of instance parameter setters.
/// Presence and value are separate keys so an absent simulator parameter keeps
/// the authored fallback, including when an explicit override is zero.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum InitializationInput {
    Temperature,
    // Phase changes within this analysis must not replay analog initial.
    Analysis,
    SimParamPresent(smol_str::SmolStr),
    SimParamValue(smol_str::SmolStr),
}

impl InitializationInput {
    fn expression(&self) -> String {
        match self {
            Self::Temperature => "ctx.temperature()".into(),
            Self::Analysis => "f64::from(ctx.analysis_code())".into(),
            Self::SimParamPresent(name) => format!("ctx.has_simparam({name:?}) as u8 as f64"),
            Self::SimParamValue(name) => format!("ctx.simparam_or({name:?}, 0.0)"),
        }
    }

    fn invalid(&self, value: &str) -> String {
        match self {
            Self::Temperature => format!("!({value}).is_finite() || ({value}) <= 0.0"),
            Self::Analysis => format!(
                "!({value}).is_finite() || !(0.0..=4.0).contains(&({value})) || ({value}).fract() != 0.0"
            ),
            Self::SimParamPresent(_) => {
                format!("({value}) != 0.0 && ({value}) != 1.0")
            }
            Self::SimParamValue(_) => format!("!({value}).is_finite()"),
        }
    }
}

struct ModelPlan {
    initialization: Vec<InitializationPlan>,
    initialization_inputs: Vec<InitializationInput>,
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
    /// The linear-ddt fast path; empty for general frequency expressions.
    reactive: Stamps,
    /// Dynamic Jacobian coefficients for models beyond the linear-ddt fast path.
    frequency: Vec<FrequencyEntry>,
    /// Output positions of the distinct integral-activation flags.
    frequency_activity: Vec<usize>,
    /// The conduction body cut by invalidation class, or empty when the split
    /// was measured not to be worth taking for this model.
    stages: Vec<Stage>,
    slots: usize,
    node_count: usize,
    /// Physical-branch target and orientation per source equation.
    branch_equations: Vec<Option<BranchEquationPlan>>,
    /// Stable source-order groups of equations on the same source branch.
    source_branch_groups: Vec<SourceBranchGroup>,
    /// Output position of each equation's control-flow activation value.
    activation_positions: Vec<Option<usize>>,
    /// Output position of each event-controlled procedural variable candidate,
    /// in dense accepted-state slot order.
    event_state_candidate_positions: Vec<usize>,
    timestep_bound_position: Option<usize>,
    discontinuity_position: Option<usize>,
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
    requires_nodeset_phase: bool,
}

struct FrequencyEntry {
    equation: usize,
    unknown: usize,
    power: DynamicPower,
    value: ValueId,
    position: usize,
    active: Option<ValueId>,
    active_position: Option<usize>,
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
        // This CFG excludes declaration/analog initial phases. Inspect the
        // emitted body once, rather than searching source or scanning it for
        // every runtime instance.
        let requires_nodeset_phase = cfg.function.values.iter().any(|value| {
            matches!(&value.kind, CfgValueKind::Analysis(name) if name.eq_ignore_ascii_case("nodeset"))
        });
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
                CfgValueKind::Cross { operator, .. }
                | CfgValueKind::Above { operator, .. }
                | CfgValueKind::LastCrossing { operator, .. } => {
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
                (equation.kind != MirEquationKind::Current)
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
        let (charges, first_order_complete) = recover_stored_charges(&mut cfg.function, &residuals);
        let general_frequency = !first_order_complete || !idt_slots.is_empty();
        let mut derivative_roots = residuals.clone();
        if !general_frequency {
            derivative_roots.extend(charges.iter().flatten().copied());
        }
        // A task argument can contain ddx even though a task has no derivative.
        // Preserve the numerical preparation required to evaluate that argument.
        for value in &cfg.function.values {
            if let CfgValueKind::AnalogTask(task) = &value.kind {
                derivative_roots.extend(task.expressions().copied());
            }
        }
        // Only numeric stamp values need the preliminary scalar pass: the
        // packed pass differentiates those resolved readbacks again to obtain
        // their Hessian rows. A `ddx` used by an activation, branch predicate,
        // or report-only expression still gets its exact first derivative from
        // `AdBuilder::resolve_ddx`, but predicates are piecewise constant and
        // therefore do not need (or expose) a second derivative. Keeping those
        // readbacks compact is important for large compact-model op-point
        // sections, whose predicates must be correct without inflating the
        // executable stamp with solver-invisible Hessians.
        let mut one_step_dae_split_safe =
            crate::canonical_ir::charge::one_step_dae_split_safe(artifact, false)
                && idt_slots.is_empty()
                && !ddt_controls_flow
                && first_order_complete
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
        let mut differentiated = match differentiate_with_control_for_roots(
            &cfg.function,
            &seeds,
            &derivative_roots,
            measurements.control(),
        ) {
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
                Some(charge) if !general_frequency => differentiated.derivative_row(*charge),
                _ => Vec::new(),
            })
            .collect();
        let mut frequency = Vec::new();
        if general_frequency {
            let roots = conduction_rows
                .iter()
                .enumerate()
                .flat_map(|(equation, row)| {
                    row.iter().enumerate().filter_map(move |(unknown, value)| {
                        (Some(unknown) != correction_lane)
                            .then_some(*value)
                            .flatten()
                            .map(|value| (equation, unknown, value))
                    })
                })
                .collect::<Vec<_>>();
            let coefficients = frequency::expand(
                &mut differentiated.function,
                cfg.function.values.len(),
                &roots.iter().map(|(_, _, value)| *value).collect::<Vec<_>>(),
                measurements.control(),
            )
            .map_err(|error| match error {
                FrequencyError::Cancelled(error) => RustBackendError::cancelled(
                    artifact.metadata.source_package.as_str(),
                    artifact.mir.module_name.as_str(),
                    error,
                ),
                FrequencyError::Unsupported(error) => unsupported(artifact, error),
            })?;
            for ((equation, unknown, _), coefficients) in roots.into_iter().zip(coefficients) {
                frequency.extend(
                    coefficients
                        .into_iter()
                        .map(|(power, value)| FrequencyEntry {
                            equation,
                            unknown,
                            power,
                            value: value.value,
                            position: 0,
                            active: value.active,
                            active_position: None,
                        }),
                );
            }
        }
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
        // Slice the primal first. If a noise power or its branch predicate
        // needs a symbolic readback, use the body that AD has resolved.
        let mut noise = plan_noise(artifact, &cfg, &cfg.function)
            .ok()
            .or_else(|| plan_noise(artifact, &cfg, &differentiated.function).ok());
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
        if !charged || general_frequency {
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
        let conduction_wanted = wanted.len();
        let reactive_wanted = reactive.wanted();
        wanted.extend_from_slice(&reactive_wanted);
        let frequency_start = wanted.len();
        wanted.extend(frequency.iter().map(|entry| entry.value));
        let frequency_end = wanted.len();
        wanted.extend(frequency.iter().filter_map(|entry| entry.active));
        let stamp_wanted = wanted.len();
        wanted.extend(activations.iter().flatten().copied());
        let activation_wanted = activations.iter().flatten().count();
        wanted.extend(cfg.event_state_candidates.iter().copied());
        wanted.extend(cfg.timestep_bound);
        wanted.extend(cfg.discontinuity);
        // Side effects are explicit optimization roots. They remain in their
        // source blocks, including repeated calls on loop back edges.
        wanted.extend(cfg.function.values.iter().filter_map(|value| {
            matches!(value.kind, CfgValueKind::AnalogTask(_)).then_some(value.id)
        }));
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
        conduction.remap(&mapped[..conduction_wanted]);
        reactive.remap(&mapped[conduction_wanted..frequency_start]);
        for (entry, value) in frequency
            .iter_mut()
            .zip(&mapped[frequency_start..frequency_end])
        {
            entry.value = *value;
        }
        let mut mapped_activity = mapped[frequency_end..stamp_wanted].iter().copied();
        for entry in &mut frequency {
            entry.active = entry.active.and_then(|_| {
                let active = mapped_activity.next().expect("frequency activation");
                (!matches!(function.value(active).kind, CfgValueKind::RealConstant(1.0)))
                    .then_some(active)
            });
        }
        frequency.retain(|entry| {
            !entry.active.is_some_and(|active| {
                matches!(function.value(active).kind, CfgValueKind::RealConstant(0.0))
            }) && (entry.power.idt > 0
                || !matches!(
                    function.value(entry.value).kind,
                    CfgValueKind::RealConstant(0.0)
                ))
        });
        let activation_end = stamp_wanted + activation_wanted;
        let mut mapped_activations = mapped[stamp_wanted..activation_end].iter().copied();
        let activations = activations
            .iter()
            .map(|activation| {
                activation.map(|_| mapped_activations.next().expect("mapped activation"))
            })
            .collect::<Vec<_>>();
        debug_assert!(mapped_activations.next().is_none());
        let event_state_end = activation_end + cfg.event_state_candidates.len();
        let event_state_candidates = mapped[activation_end..event_state_end].to_vec();
        let timestep_bound = cfg.timestep_bound.map(|_| mapped[event_state_end]);
        let bound_end = event_state_end + usize::from(timestep_bound.is_some());
        let discontinuity = cfg.discontinuity.map(|_| mapped[bound_end]);
        let task_effects = &mapped[bound_end + usize::from(discontinuity.is_some())..];
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
        let mut frequency_activity = Vec::new();
        let mut activity_indices = HashMap::new();
        for entry in &mut frequency {
            entry.position = outputs.len();
            outputs.push(entry.value);
            entry.active_position = entry.active.map(|active| {
                *activity_indices.entry(active).or_insert_with(|| {
                    let index = frequency_activity.len();
                    frequency_activity.push(outputs.len());
                    outputs.push(active);
                    index
                })
            });
        }
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
        let timestep_bound_position = timestep_bound.map(|bound| {
            outputs.push(bound);
            outputs.len() - 1
        });
        let discontinuity_position = discontinuity.map(|flags| {
            outputs.push(flags);
            outputs.len() - 1
        });
        // These positions keep execution live through scheduling and emission;
        // they produce no numerical stamp or exported device state.
        outputs.extend(task_effects.iter().copied());

        let parameter_scopes: Vec<_> = artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.scope)
            .collect();
        let schedule = schedule_with_parameter_scopes(&function, &parameter_scopes);
        // A coefficient held during AC differentiation can still vary between
        // timepoints or Newton iterates. Its reactive primitive is not a
        // conservative charge suitable for OneStep's F/Q history split.
        one_step_dae_split_safe &= function.values.iter().all(|value| {
            !matches!(
                value.kind,
                CfgValueKind::Unary {
                    op: CfgUnaryOp::FreezeDerivative,
                    ..
                }
            ) || schedule.class(value.id) <= InvalidationClass::Temperature
        });
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

        let (branch_equations, source_branch_groups) = plan_source_branches(artifact)?;
        record_phase(
            artifact,
            measurements,
            PipelinePhase::Scheduling,
            phase_started.elapsed(),
        )?;

        let mut initialization = Vec::new();
        for phase in [
            rspice_veriloga_runtime::AnalogEvaluationPhase::Declarations,
            rspice_veriloga_runtime::AnalogEvaluationPhase::Initialization,
        ] {
            if !artifact.hir.body.iter().any(|region| matches!(region, crate::canonical_ir::hir::HirRegion::Initialization { phase: found, .. } if *found == phase)) {
                continue;
            }
            let cfg = CfgModel::from_hir_for_initialization(&artifact.hir, &artifact.mir, phase)
                .map_err(|diagnostics| {
                    unsupported(
                        artifact,
                        diagnostics
                            .iter()
                            .map(|diagnostic| diagnostic.message.as_str())
                            .collect::<Vec<_>>()
                            .join("; "),
                    )
                })?;
            let state_count = cfg.event_state_candidates.len();
            let mut roots = cfg.event_state_candidates;
            roots.extend(cfg.function.values.iter().filter_map(|value| {
                matches!(value.kind, CfgValueKind::AnalogTask(_)).then_some(value.id)
            }));
            let (function, outputs) =
                optimize_with_control(&cfg.function, &roots, measurements.control()).map_err(
                    |error| {
                        RustBackendError::cancelled(
                            artifact.metadata.source_package.as_str(),
                            artifact.mir.module_name.as_str(),
                            error,
                        )
                    },
                )?;
            initialization.push(InitializationPlan {
                phase,
                function,
                outputs,
                state_count,
            });
        }

        let mut initialization_inputs = std::collections::BTreeSet::new();
        for plan in &initialization {
            for value in &plan.function.values {
                match &value.kind {
                    CfgValueKind::Temperature | CfgValueKind::ThermalVoltage => {
                        initialization_inputs.insert(InitializationInput::Temperature);
                    }
                    CfgValueKind::Analysis(_) => {
                        initialization_inputs.insert(InitializationInput::Analysis);
                    }
                    CfgValueKind::SimParamValue(parameter)
                    | CfgValueKind::SimParamPresent(parameter) => {
                        let name = smol_str::SmolStr::new(parameter.name());
                        initialization_inputs
                            .insert(InitializationInput::SimParamPresent(name.clone()));
                        initialization_inputs
                            .insert(InitializationInput::SimParamValue(name.clone()));
                    }
                    _ => {}
                }
            }
        }
        Ok(Self {
            initialization,
            initialization_inputs: initialization_inputs.into_iter().collect(),
            function,
            outputs,
            conduction,
            reactive,
            frequency,
            frequency_activity,
            stages,
            slots,
            node_count: artifact.mir.nodes.len(),
            branch_equations,
            source_branch_groups,
            activation_positions,
            event_state_candidate_positions,
            timestep_bound_position,
            discontinuity_position,
            noise,
            ddt_slots,
            idt_slots,
            cross_slots,
            timer_slots,
            limit_slots,
            one_step_dae_split_safe,
            requires_nodeset_phase,
        })
    }

    fn accepted_state_shape_identity(
        &self,
        artifact: &CanonicalIrArtifact,
    ) -> Result<[u8; 32], RustBackendError> {
        let ordered = |slots: &HashMap<ExprId, usize>, family: &str| {
            ordered_operator_slots(slots).map_err(|error| {
                accepted_state_shape_error(artifact, format!("{family} state slots: {error}"))
            })
        };
        let expression = |operator: ExprId| {
            artifact
                .hir
                .expressions
                .get(operator.index() as usize)
                .filter(|expression| expression.id == operator)
                .ok_or_else(|| {
                    accepted_state_shape_error(
                        artifact,
                        format!("state operator {operator} is absent from canonical HIR"),
                    )
                })
        };

        let mut shape = AcceptedStateShapeHasher::new();

        let ddt = ordered(&self.ddt_slots, "ddt")?;
        shape.section("ddt", ddt.len());
        for (slot, operator) in ddt.into_iter().enumerate() {
            let expression = expression(operator)?;
            shape.operator(
                "ddt",
                slot,
                expression.span,
                None,
                &[
                    "previous:f64:finite",
                    "older:f64:finite",
                    "derivative_previous:f64:finite",
                    "initialized:bool",
                ],
            );
        }

        let idt = ordered(&self.idt_slots, "idt")?;
        shape.section("idt", idt.len());
        for (slot, operator) in idt.into_iter().enumerate() {
            let expression = expression(operator)?;
            shape.operator(
                "idt",
                slot,
                expression.span,
                None,
                &[
                    "previous:f64:finite",
                    "older:f64:finite",
                    "input_previous:f64:finite",
                    "initialized:bool",
                ],
            );
        }

        let limit = ordered(&self.limit_slots, "limit")?;
        shape.section("limit", limit.len());
        for (slot, operator) in limit.into_iter().enumerate() {
            let expression = expression(operator)?;
            let selector = match &expression.kind {
                HirExprKind::AnalogOperator {
                    op: HirAnalogOperator::Limit { selector, .. },
                } => selector.as_str(),
                HirExprKind::SystemFunction { name, .. } if name == "$limit" => "$default",
                _ => {
                    return Err(accepted_state_shape_error(
                        artifact,
                        format!("limiter state operator {operator} is not a canonical limit"),
                    ));
                }
            };
            shape.operator(
                "limit",
                slot,
                expression.span,
                Some(selector),
                &["anchor:f64:finite", "initialized:bool"],
            );
        }

        let detectors = ordered(&self.cross_slots, "event detector")?;
        shape.section("event_detectors", detectors.len());
        for (slot, operator) in detectors.into_iter().enumerate() {
            let expression = expression(operator)?;
            // Slot allocation precedes optimization. A detector used only by
            // noise metadata may be absent from the reduced stamping CFG.
            let family = match &expression.kind {
                HirExprKind::Call { name, .. } | HirExprKind::SystemFunction { name, .. }
                    if matches!(name.as_str(), "cross" | "above" | "last_crossing") =>
                {
                    name.as_str()
                }
                _ => {
                    return Err(accepted_state_shape_error(
                        artifact,
                        format!(
                            "event detector state operator {operator} is not a canonical detector"
                        ),
                    ));
                }
            };
            shape.runtime_operator(
                family,
                slot,
                expression.span,
                None,
                &rspice_veriloga_runtime::GeneratedCrossState::CHECKPOINT_SCHEMA,
            );
        }

        let event_variables = artifact
            .hir
            .variables
            .iter()
            .filter(|variable| variable.is_state)
            .collect::<Vec<_>>();
        shape.section("event_variables", event_variables.len());
        for (slot, variable) in event_variables.into_iter().enumerate() {
            shape.record("event_variable");
            shape.u64(slot as u64);
            shape.u64(variable.id.index() as u64);
            shape.field(variable.name.as_bytes());
            shape.field(canonical_value_type_tag(variable.value_type).as_bytes());
            shape.field(b"accepted:f64:nan-forbidden");
        }
        shape.section(
            "switch_branch_variables",
            artifact.hir.switch_branch_variables.len(),
        );
        for variable in &artifact.hir.switch_branch_variables {
            shape.u64(u64::from(variable.index()));
            shape.field(b"accepted:f64:zero-or-one");
        }

        let has_timer_bound = !self.timer_slots.is_empty();
        shape.section("timer_bound", usize::from(has_timer_bound));
        if has_timer_bound {
            shape.record("timer_bound");
            shape.field(b"accepted:f64:positive-or-infinity");
        }

        if !self.initialization_inputs.is_empty() {
            shape.section("initialization_context", self.initialization_inputs.len());
            for input in &self.initialization_inputs {
                shape.field(format!("{input:?}").as_bytes());
            }
        }
        shape.section("terminal_currents", artifact.hir.ports.len());
        for (slot, port) in artifact.hir.ports.iter().enumerate() {
            shape.record("terminal_current");
            shape.u64(slot as u64);
            shape.u64(port.id.index() as u64);
            shape.field(port.name.as_bytes());
            shape.field(port.direction.as_bytes());
            shape.field(port.discipline.as_bytes());
            shape.field(b"accepted:f64:finite");
        }

        Ok(shape.finish())
    }
}

fn accepted_state_shape_error(
    artifact: &CanonicalIrArtifact,
    message: impl Into<String>,
) -> RustBackendError {
    RustBackendError::internal(
        artifact.metadata.source_digest.to_string(),
        artifact.mir.module_name.to_string(),
        message,
    )
}

fn ordered_operator_slots(slots: &HashMap<ExprId, usize>) -> Result<Vec<ExprId>, String> {
    let mut ordered = vec![None; slots.len()];
    for (&operator, &slot) in slots {
        let destination = ordered
            .get_mut(slot)
            .ok_or_else(|| format!("slot {slot} is outside 0..{}", slots.len()))?;
        if let Some(previous) = destination.replace(operator) {
            return Err(format!(
                "slot {slot} is assigned to both {previous} and {operator}"
            ));
        }
    }
    ordered
        .into_iter()
        .enumerate()
        .map(|(slot, operator)| operator.ok_or_else(|| format!("slot {slot} is unassigned")))
        .collect()
}

fn canonical_value_type_tag(value_type: CanonicalValueType) -> &'static str {
    match value_type {
        CanonicalValueType::Real => "real",
        CanonicalValueType::Integer => "integer",
        CanonicalValueType::String => "string",
        CanonicalValueType::Boolean => "boolean",
        CanonicalValueType::NatureAccess => "nature-access",
        CanonicalValueType::Void => "void",
        CanonicalValueType::Unknown => "unknown",
        CanonicalValueType::Error => "error",
    }
}

struct AcceptedStateShapeHasher(blake3::Hasher);

impl AcceptedStateShapeHasher {
    fn new() -> Self {
        let mut hasher = blake3::Hasher::new();
        // V3 identifies an accepted-state slot by its stable source location,
        // family, order, selector, and lane schema. Canonical HIR expression
        // numbers are intentionally excluded: inserting an unrelated
        // expression can renumber them without changing persisted state.
        hasher.update(b"rspice-generated-accepted-state-schema-v3\0");
        Self(hasher)
    }

    fn section(&mut self, name: &str, count: usize) {
        self.record("section");
        self.field(name.as_bytes());
        self.u64(count as u64);
    }

    fn operator(
        &mut self,
        family: &str,
        slot: usize,
        span: SourceSpanRef,
        selector: Option<&str>,
        lanes: &[&str],
    ) {
        self.operator_header(family, slot, span, selector);
        self.u64(lanes.len() as u64);
        for lane in lanes {
            self.field(lane.as_bytes());
        }
    }

    fn runtime_operator(
        &mut self,
        family: &str,
        slot: usize,
        span: SourceSpanRef,
        selector: Option<&str>,
        lanes: &[rspice_veriloga_runtime::GeneratedCheckpointLaneDescriptor],
    ) {
        self.operator_header(family, slot, span, selector);
        self.u64(lanes.len() as u64);
        for lane in lanes {
            self.field(lane.name.as_bytes());
            self.field(lane.lane_type.identity_tag().as_bytes());
        }
    }

    fn operator_header(
        &mut self,
        family: &str,
        slot: usize,
        span: SourceSpanRef,
        selector: Option<&str>,
    ) {
        self.record("operator");
        self.field(family.as_bytes());
        self.u64(slot as u64);
        self.u64(u64::from(span.source_file_id));
        self.u64(u64::from(span.start));
        self.u64(u64::from(span.end));
        self.field(selector.unwrap_or("").as_bytes());
    }

    fn record(&mut self, record: &str) {
        self.field(record.as_bytes());
    }

    fn field(&mut self, field: &[u8]) {
        self.0.update(&(field.len() as u64).to_le_bytes());
        self.0.update(field);
    }

    fn u64(&mut self, value: u64) {
        self.0.update(&value.to_le_bytes());
    }

    fn finish(self) -> [u8; 32] {
        *self.0.finalize().as_bytes()
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
            CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
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

/// Why a module's noise slice could not be cut from its CFG, so the flat
/// HIR-driven emitter in [`super::noise`] carries it instead.
///
/// Named rather than counted because the two halves of the answer want
/// different responses: a correspondence that failed is a compiler fault to
/// chase, while a module with no planned sources at all is simply silent and
/// there is nothing to plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NoiseDecline {
    /// The artifact carries no noise-source plan: the module is silent.
    NoPlannedSources,
    /// The body lowered a different number of sources than the plan holds.
    SourceCount { planned: usize, lowered: usize },
    /// A planned source names no equation, or no lowered source carries its
    /// `(contribution, ordinal)` name.
    Unpaired { index: usize },
    /// A paired source disagrees about its kind, whether it has an exponent, or
    /// how wide its table is.
    Shape { index: usize },
    /// A magnitude reads a `ddt` — which would advance per-instance transient
    /// history while a noise analysis merely reads it — or an unresolved `ddx`
    /// that requires the differentiated body.
    LiveStateOperator,
}

/// Match the lowered noise sources to the plan the descriptors come from, and
/// reduce the body to just the magnitudes.
///
/// The correspondence is checked rather than assumed. A source is named by the
/// contribution it was written in and its position among that contribution's
/// sources, because the plan is extracted from a second lowering of the same
/// expressions and shares no expression ids with the body. Where the two
/// disagree this declines rather than guessing: a noise source silently given
/// another source's power would be reported under the wrong mechanism at the
/// wrong branch, which reads as a physics result rather than as a compiler
/// fault. Declining costs the model nothing but the smaller file.
fn plan_noise(
    artifact: &CanonicalIrArtifact,
    cfg: &CfgModel,
    function: &CfgFunction,
) -> Result<NoisePlan, NoiseDecline> {
    let planned = &artifact.noise_sources.sources;
    // A silent model falls back too: the generator being replaced already emits
    // the empty evaluator for one, and this emitter has no reason to.
    if planned.is_empty() {
        return Err(NoiseDecline::NoPlannedSources);
    }
    if cfg.noise.len() != planned.len() {
        return Err(NoiseDecline::SourceCount {
            planned: planned.len(),
            lowered: cfg.noise.len(),
        });
    }

    let mut wanted = Vec::new();
    let mut sources = Vec::with_capacity(planned.len());
    for (index, source) in planned.iter().enumerate() {
        let contribution = artifact
            .mir
            .equations
            .get(usize::from(source.equation))
            .map(|equation| equation.contribution)
            .ok_or(NoiseDecline::Unpaired { index })?;
        let ordinal = planned[..index]
            .iter()
            .filter(|earlier| earlier.equation == source.equation)
            .count();
        let lowered = cfg
            .noise
            .iter()
            .find(|lowered| lowered.contribution == contribution && lowered.ordinal == ordinal)
            .ok_or(NoiseDecline::Unpaired { index })?;

        let table_width = source
            .table
            .as_ref()
            .map_or(0, |table| table.operands.len());
        if lowered.kind != source.kind
            || lowered.exponent.is_some() != source.exponent.is_some()
            || lowered.table.len() != table_width
        {
            return Err(NoiseDecline::Shape { index });
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
    frequency::freeze_noise_primal(&mut function);
    let mut origins = vec![None; function.values.len()];
    for (original, mapped) in tracked.into_iter().zip(mapped) {
        if let Some(mapped) = mapped {
            origins[usize::from(mapped)].get_or_insert(original);
        }
    }

    // A live `ddx` still requires the differentiated body. Primal integrators
    // and derivatives have already been frozen without touching their history.
    let live = reachable(&function, &outputs);
    for (index, value) in function.values.iter_mut().enumerate() {
        if !matches!(value.kind, CfgValueKind::Ddx { .. }) {
            continue;
        }
        if live[index] {
            return Err(NoiseDecline::LiveStateOperator);
        }
        // Dead readbacks cannot affect a magnitude and need no emitted call.
        value.kind = CfgValueKind::RealConstant(0.0);
    }
    Ok(NoisePlan {
        function,
        outputs,
        sources,
        origins,
        prepared_slots: 0,
        shared_through: None,
    })
}

/// Why the CFG noise slice declined a module, or `None` if it did not.
///
/// The one route to that answer from outside this module. Exposed so that the
/// census in `crate::native::cfg_census` can name the reason a shipped model
/// takes the flat emitter instead of re-deriving the correspondence, which
/// would make the census a second copy of the thing it measures.
///
/// Gated to exactly that census's own configuration: it has no other caller,
/// and a hook that compiled into the shipped library with none would be one
/// more thing to keep alive.
#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
pub(crate) fn noise_plan_decline(
    artifact: &CanonicalIrArtifact,
    cfg: &CfgModel,
    function: &CfgFunction,
) -> Option<NoiseDecline> {
    plan_noise(artifact, cfg, function).err()
}

/// Every value the optimized slice can read, including retained branch
/// predicates and block parameters supplied by incoming terminators.
fn reachable(function: &CfgFunction, roots: &[ValueId]) -> Vec<bool> {
    let mut declared: HashMap<ValueId, (usize, usize)> = HashMap::new();
    for (block, data) in function.blocks.iter().enumerate() {
        for (index, parameter) in data.params.iter().enumerate() {
            declared.insert(*parameter, (block, index));
        }
    }

    let mut live = vec![false; function.values.len()];
    let mut work: Vec<ValueId> = roots.to_vec();
    // An arm may jump to a merge whose incoming terminator no longer carries
    // the predicate that selected the arm. All retained predicates affect
    // execution of this optimized slice, even across those intermediate jumps.
    work.extend(function.blocks.iter().filter_map(|block| {
        if let CfgTerminator::Branch { condition, .. } = block.terminator {
            Some(condition)
        } else {
            None
        }
    }));
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

fn plan_source_branches(
    artifact: &CanonicalIrArtifact,
) -> Result<(Vec<Option<BranchEquationPlan>>, Vec<SourceBranchGroup>), RustBackendError> {
    let mut equations = vec![None; artifact.mir.equations.len()];
    let mut groups: Vec<SourceBranchGroup> = Vec::new();
    let mut group_by_branch: Vec<Option<usize>> = vec![None; artifact.mir.branch_unknowns.len()];

    for (equation_index, equation) in artifact.mir.equations.iter().enumerate() {
        if equation.kind == MirEquationKind::Current {
            continue;
        }
        let branch = equation.branch_unknown.map(usize::from).ok_or_else(|| {
            RustBackendError::internal(
                artifact.metadata.source_package.as_str(),
                artifact.mir.module_name.as_str(),
                format!("source equation {equation_index} has no branch unknown"),
            )
        })?;
        let pos = equation.branch.pos_node;
        let neg = equation.branch.neg_node;

        let existing = group_by_branch[branch];
        let (group_index, sign) = match existing {
            Some(group_index) => {
                let group = &mut groups[group_index];
                let sign = if group.pos == pos && group.neg == neg {
                    1
                } else {
                    -1
                };
                group.equations.push(equation_index);
                (group_index, sign)
            }
            None => {
                let group_index = groups.len();
                group_by_branch[branch] = Some(group_index);
                groups.push(SourceBranchGroup {
                    kind: equation.kind,
                    pos,
                    neg,
                    branch,
                    equations: vec![equation_index],
                });
                (group_index, 1)
            }
        };
        equations[equation_index] = Some(BranchEquationPlan {
            branch: groups[group_index].branch,
            sign: if equation.kind == MirEquationKind::Indirect {
                -1
            } else {
                sign
            },
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
        if !self.frequency.is_empty() {
            runtime_support.push("GeneratedDerivative".to_string());
        }
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
        if std::iter::once(&self.function)
            .chain(self.stages.iter().map(|stage| &stage.function))
            .chain(self.initialization.iter().map(|plan| &plan.function))
            .any(uses_checked_operations)
        {
            runtime_support.push("integer".to_string());
        }
        if std::iter::once(&self.function)
            .chain(self.stages.iter().map(|stage| &stage.function))
            .chain(self.initialization.iter().map(|plan| &plan.function))
            .any(|function| {
                function.values.iter().any(|value| {
                    matches!(
                        value.kind,
                        CfgValueKind::SumProductsDiv { .. }
                            | CfgValueKind::LaneSumProductsDiv { .. }
                    )
                })
            })
        {
            runtime_support.extend([
                "arithmetic::product_div".to_string(),
                "arithmetic::product_sum_div".to_string(),
                "arithmetic::sum_products_div".to_string(),
                "arithmetic::sum_products_div_lanes".to_string(),
            ]);
        }
        if self
            .function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::LastCrossing { .. }))
        {
            runtime_support.push("evaluate_generated_last_crossing".to_string());
        }
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
        if self
            .function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::IntegralDerivative { .. }))
        {
            runtime_support.extend([
                "evaluate_generated_idt_derivative".to_string(),
                "GeneratedIdtCandidateError".to_string(),
            ]);
        }
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
        self.emit_initialization(artifact, &mut out)?;
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
        let integer_context = if uses_checked_operations(&stage.function) {
            "    ctx: &GeneratedEvalContext<'_>,\n"
        } else {
            ""
        };
        let _ = writeln!(
            out,
            "pub(super) fn {name}(\n\
             \x20   parameters: &[f64],\n\
             \x20   parameter_given: &[bool],\n\
             \x20   multiplicity: f64,\n\
             \x20   staged: &[f64],\n\
             \x20   temperature: f64,\n\
             \x20   thermal_voltage: f64,\n\
             {integer_context}\
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
            self.emit_prologue(artifact, &stage.function, 3, out, false)?;
            out.push_str(&indent(&body, 3));
            if produced.is_empty() {
                out.push_str("            [0.0]\n");
            } else {
                let names = numeric_output_names(&stage.function, &produced, &names);
                let _ = writeln!(out, "            [{}]", names.join(", "));
            }
            out.push_str("        };\n");
        } else {
            let integer_context = integer_context_argument(&stage.function);
            let _ = writeln!(
                out,
                "        let produced = {}(\n\
                 \x20           &self.params.values,\n\
                 \x20           &self.param_given[..],\n\
                 \x20           self.multiplicity,\n\
                 \x20           &self.canonical_staged[..],\n\
                 \x20           ctx.temperature(),\n\
                 \x20           ctx.thermal_voltage(),\n\
                 {integer_context}\
                 \x20       );",
                preprocess_fn_name(stage.class)
            );
        }
        if uses_checked_operations(&stage.function) {
            out.push_str("        if ctx.evaluation_failed() { return; }\n");
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
             \x20           ctx.thermal_voltage(),\n",
        );
        out.push_str(integer_context_argument(&stage.function));
        out.push_str("        );\n");
        if uses_checked_operations(&stage.function) {
            out.push_str("        if ctx.evaluation_failed() { return; }\n");
        }
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
        if self.timestep_bound_position.is_some() {
            out.push_str("        self.timestep_bound_candidate = f64::NAN;\n");
        }
        if self.discontinuity_position.is_some() {
            out.push_str("        self.discontinuity_candidate = f64::NAN;\n");
        }
        if !self.initialization.is_empty() {
            out.push_str("        self.initialize_analysis(ctx);\n        if ctx.evaluation_failed() || !self.canonical_initialization_valid { return; }\n");
        }
        if self.has_newton_tasks() {
            out.push_str("        if ctx.analog_tasks_enabled() { self.analog_effects.get_or_insert_with(Default::default).begin_evaluation(); }\n");
        }
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
            if uses_checked_operations(&stage.function) {
                out.push_str("        if ctx.evaluation_failed() { return; }\n");
            }
        }

        let newton = self
            .stages
            .iter()
            .find(|stage| stage.class == InvalidationClass::Newton);
        let function = newton.map_or(&self.function, |stage| &stage.function);
        let (body, values) = self.newton_outputs(artifact, newton, control)?;
        self.emit_prologue(artifact, function, 2, out, false)?;
        out.push_str(&indent(&body, 2));

        if let Some(position) = self.timestep_bound_position {
            let _ = writeln!(
                out,
                "        self.timestep_bound_candidate = {};",
                values[position]
            );
        }

        if let Some(position) = self.discontinuity_position {
            let _ = writeln!(
                out,
                "        self.discontinuity_candidate = {};\n        if !matches!(self.discontinuity_candidate, 0.0 | 1.0 | 2.0 | 3.0) {{ ctx.report_discontinuity_degree_error(); return; }}",
                values[position]
            );
        }

        for (slot, position) in self
            .event_state_candidate_positions
            .iter()
            .copied()
            .enumerate()
        {
            let _ = writeln!(
                out,
                "        if ctx.dynamic_operators_enabled() {{ self.event_state_candidate[{slot}] = {}; }}",
                values[position]
            );
        }

        self.emit_source_structure(&values, out);

        for (index, row) in self.conduction.rows.iter().enumerate() {
            let (residual, derivatives) = &self.conduction.positions[index];
            let residual = self.corrected_residual(index, &values, *residual);
            let activation = (row.kind != MirEquationKind::Current)
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
        for (index, entry) in self.frequency.iter().enumerate() {
            let at = self.reactive.width() + index;
            let _ = writeln!(
                out,
                "        self.canonical_reactive[{at}] = {};",
                values[entry.position]
            );
        }
        for (index, &position) in self.frequency_activity.iter().enumerate() {
            let at = self.reactive.width() + self.frequency.len() + index;
            let _ = writeln!(
                out,
                "        self.canonical_reactive[{at}] = {};",
                values[position]
            );
        }
        if self.has_newton_tasks() {
            out.push_str("        if ctx.analog_tasks_enabled() && !ctx.evaluation_failed() { self.analog_effects.as_mut().expect(\"task evaluation began\").complete_evaluation(); }\n");
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    fn emit_initialization(
        &self,
        artifact: &CanonicalIrArtifact,
        out: &mut String,
    ) -> Result<(), RustBackendError> {
        if self.initialization.is_empty() {
            return Ok(());
        }
        out.push_str(
            "    pub(crate) fn initialization_is_ready(&self, ctx: &GeneratedEvalContext<'_>) -> bool {\n",
        );
        out.push_str("        self.canonical_initialization_valid\n");
        for (slot, input) in self.initialization_inputs.iter().enumerate() {
            let _ = writeln!(
                out,
                "            && self.canonical_initialization_context[{slot}] == ({})",
                input.expression()
            );
        }
        out.push_str("    }\n\n");
        out.push_str(
            "    pub fn initialize_analysis(&mut self, ctx: &GeneratedEvalContext<'_>) {\n",
        );
        out.push_str(
            "        if ctx.evaluation_failed() || self.initialization_is_ready(ctx) { return; }\n",
        );
        for (slot, input) in self.initialization_inputs.iter().enumerate() {
            let invalid = input.invalid(&input.expression());
            let _ = writeln!(
                out,
                "        if {invalid} {{ self.canonical_initialization_valid = false; ctx.report_initialization_error({slot}); return; }}"
            );
        }
        out.push_str("        let rollback = self.capture_rollback_state();\n");
        if self.has_analog_tasks() {
            out.push_str("        self.analog_effects.get_or_insert_with(Default::default).begin_evaluation();\n");
        }
        for plan in &self.initialization {
            let _ = writeln!(
                out,
                "        self.canonical_initialize_{}(ctx);",
                plan.phase as u8
            );
            out.push_str("        if ctx.evaluation_failed() { self.restore_rollback_state(&rollback); self.canonical_initialization_valid = false; return; }\n");
        }
        if self.has_analog_tasks() {
            out.push_str("        let journal = self.analog_effects.as_mut().expect(\"initialization journal\");\n        journal.complete_evaluation();\n        if let Err(source) = journal.validate_candidate() { ctx.report_analog_task_error(0, source); self.restore_rollback_state(&rollback); self.canonical_initialization_valid = false; return; }\n        journal.apply_validated_acceptance();\n");
        }
        for (slot, input) in self.initialization_inputs.iter().enumerate() {
            let _ = writeln!(
                out,
                "        self.canonical_initialization_context[{slot}] = {};",
                input.expression()
            );
        }
        out.push_str("        self.canonical_initialization_valid = true;\n    }\n\n");
        for plan in &self.initialization {
            let _ = writeln!(
                out,
                "    fn canonical_initialize_{}(&mut self, ctx: &GeneratedEvalContext<'_>) {{",
                plan.phase as u8
            );
            self.emit_prologue(artifact, &plan.function, 2, out, true)?;
            let (body, values) = emit_body(&plan.function, &plan.outputs, &self.emit_bindings())
                .map_err(|error| unsupported(artifact, format!("initialization body: {error}")))?;
            out.push_str(&indent(&body, 2));
            let values = numeric_output_names(&plan.function, &plan.outputs, &values);
            for (slot, value) in values.iter().take(plan.state_count).enumerate() {
                let _ = writeln!(
                    out,
                    "        if !({value}).is_finite() {{ ctx.report_initialization_error({slot}); return; }}"
                );
            }
            out.push_str("        if ctx.evaluation_failed() { return; }\n");
            for (slot, value) in values.iter().take(plan.state_count).enumerate() {
                let _ = writeln!(
                    out,
                    "        self.event_state_accepted[{slot}] = {value};\n        self.event_state_candidate[{slot}] = {value};"
                );
            }
            out.push_str("    }\n\n");
        }
        Ok(())
    }

    /// The reactive matrix, written from what `stamp` already worked out.
    ///
    /// First-order charge models cache C and stamp jωC. General expressions
    /// cache the coefficients of the differentiated operator chain and compose
    /// its full frequency response here, including real terms such as -ω².
    /// Neither path re-evaluates a primal expression or an operator history.
    fn emit_stamp_reactive(&self, out: &mut String) -> Result<(), RustBackendError> {
        out.push_str(
            "    pub fn stamp_reactive(&mut self, ctx: &GeneratedEvalContext<'_>, stamper: &mut GeneratedReactiveStamper<'_>) {\n",
        );
        if self.reactive.rows.is_empty() && self.frequency.is_empty() {
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
        for (index, entry) in self.frequency.iter().enumerate() {
            let row = &self.conduction.rows[entry.equation];
            let at = self.reactive.width() + index;
            let (axis, unknown) = if entry.unknown < self.node_count {
                ("node", entry.unknown)
            } else {
                ("branch", entry.unknown - self.node_count)
            };
            let real = (entry.power.ddt % 2) == (entry.power.idt % 2);
            let scale = if matches!(row.kind, MirEquationKind::Current) {
                "multiplicity"
            } else {
                "1.0"
            };
            let guard = if let Some(index) = entry.active_position {
                let active_at = self.reactive.width() + self.frequency.len() + index;
                format!("cached[{active_at}] != 0.0 && ")
            } else {
                String::new()
            };
            let _ = writeln!(
                out,
                "        if {guard}let Some(value) = stamper.scaled_frequency_coefficient(ctx, cached[{at}], {scale}, {}, {}) {{",
                entry.power.ddt, entry.power.idt
            );
            match row.kind {
                MirEquationKind::Current => {
                    let _ = writeln!(
                        out,
                        "            stamper.stamp_current_frequency_local::<{real}>({}, {}, GeneratedDerivative::{axis}({unknown}, value));",
                        optional_node(row.pos),
                        optional_node(row.neg)
                    );
                }
                MirEquationKind::Potential | MirEquationKind::Indirect => {
                    let plan = self.branch_equations[entry.equation].expect("source branch plan");
                    let value = if plan.sign < 0 { "-value" } else { "value" };
                    let _ = writeln!(
                        out,
                        "            stamper.stamp_potential_frequency_local::<{real}>({}, GeneratedDerivative::{axis}({unknown}, {value}));",
                        plan.branch
                    );
                }
            }
            out.push_str("        }\n");
        }
        out.push_str("    }\n\n");
        Ok(())
    }

    /// `noise.rs`: the descriptor table, then one body for every magnitude.
    fn noise_file(
        &self,
        artifact: &CanonicalIrArtifact,
        options: &RustTranspileOptions,
        control: &dyn crate::metrics::PipelineControl,
    ) -> Result<GeneratedRustFile, RustBackendError> {
        let Some(noise) = &self.noise else {
            return super::noise::generate_noise_file(
                artifact,
                options,
                &self.cross_slots,
                control,
            );
        };
        let function = &noise.function;
        let mut out = String::new();
        out.push_str(
            "// @generated by rspice-veriloga; do not edit.\n#![allow(dead_code, non_snake_case, unused_parens, unused_variables)]\n\n\
             use super::state::Instance;\n",
        );
        let _ = writeln!(
            out,
            "use {}::GeneratedEvalContext;\npub use {}::{{GeneratedNoiseComplex, GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation, GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseInjectionDescriptor, GeneratedNoiseInjectionEvaluation, GeneratedNoiseKind, GeneratedNoiseProcessDescriptor, GeneratedNoiseProcessEvaluationRef, GeneratedNoiseProcessVisitor, GeneratedNoiseVisitor}};\n",
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
        let (body, values) = emit_body(function, &noise.outputs, &self.emit_bindings())
            .map_err(|error| unsupported(artifact, format!("noise body: {error}")))?;
        let grouped_noise =
            super::noise::grouped_noise_extension(artifact, options, &self.cross_slots, control)?;
        // Import from both emitted evaluators, not only from the compact
        // source-wise noise slice's CFG value kinds. The coherent-process
        // extension is emitted from its own differentiated replay plan; the
        // three HiSIM-HV extensions exposed that boundary by calling the
        // bounded-exponential derivative without importing it. Looking at the
        // completed bodies is exact (so no unused imports) and remains closed
        // over future emitter rewrites.
        let mut runtime_support = Vec::new();
        if body.contains("integer::") {
            runtime_support.push("integer".to_string());
        }
        if !shared_stages.is_empty() {
            runtime_support.push("install_generated_stage_values".to_string());
        }
        runtime_support.extend(
            lane_runtime_types(function)
                .into_iter()
                .filter(|name| body.contains(&format!("{name}("))),
        );
        let math_support = math_runtime_imports(&[&body, &grouped_noise], runtime_support);
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
        if artifact.hir.body.iter().any(|region| {
            matches!(
                region,
                crate::canonical_ir::hir::HirRegion::Initialization { .. }
            )
        }) {
            out.push_str("        if !self.initialization_is_ready(ctx) { return Err(GeneratedNoiseEvaluationError::UninitializedAnalogState); }\n");
        }

        if !shared_stages.is_empty() {
            let _ = writeln!(
                out,
                "        let mut prepared = [0.0; {}];",
                noise.prepared_slots
            );
            for stage in &shared_stages {
                let integer_context = integer_context_argument(&stage.function);
                let _ = writeln!(
                    out,
                    "        let produced = {}(\n\
                     \x20           &self.params.values,\n\
                     \x20           &self.param_given[..],\n\
                     \x20           self.multiplicity,\n\
                     \x20           &prepared[..],\n\
                     \x20           ctx.temperature(),\n\
                     \x20           ctx.thermal_voltage(),\n\
                     {integer_context}\
                     \x20       );\n\
                     \x20       install_generated_stage_values(&mut prepared[..], &produced, &{});",
                    preprocess_fn_name(stage.class),
                    stage_slot_table_name(stage.class),
                );
            }
        }
        self.emit_noise_prologue(artifact, function, &mut out);
        super::noise::emit_frozen_event_bindings(&mut out, function, options);
        out.push_str(&indent(&body, 2));
        if uses_checked_operations(function)
            || shared_stages
                .iter()
                .any(|stage| uses_checked_operations(&stage.function))
        {
            out.push_str("        ctx.check_noise_evaluation()?;\n");
        }

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
        out.push_str(&grouped_noise);

        Ok(GeneratedRustFile {
            relative_path: "noise.rs".to_string(),
            contents: super::emit::compact_generated_indentation(&out),
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
            out.push_str("        let event_state = if ctx.dynamic_operators_enabled() { &*self.event_state_accepted } else { &*self.event_state_candidate };\n");
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
            .expect("only branch-source equations request topology activation");
        truth_output(&self.function, self.outputs[position], &values[position])
    }

    fn emit_source_structure(&self, values: &[String], out: &mut String) {
        for group in &self.source_branch_groups {
            let active = group
                .equations
                .iter()
                .map(|equation| format!("({})", self.activation_expression(*equation, values)))
                .collect::<Vec<_>>()
                .join(" || ");
            if group.kind == MirEquationKind::Indirect {
                let _ = writeln!(
                    out,
                    "        self.equation_active[{}] = {active};",
                    group.branch
                );
            }
            let pos = optional_node(group.pos);
            let neg = optional_node(group.neg);
            let coupling = if group.kind == MirEquationKind::Indirect {
                "stamp_branch_current_local"
            } else {
                "stamp_potential_branch_local"
            };
            let _ = writeln!(
                out,
                "        if {active} {{\n\
                 \x20           stamper.{coupling}({pos}, {neg}, {}, multiplicity);\n\
                 \x20       }} else {{\n\
                 \x20           stamper.stamp_inactive_potential_branch_local({});\n\
                 \x20       }}",
                group.branch, group.branch,
            );
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
            (MirEquationKind::Potential | MirEquationKind::Indirect, Reactive::No) => {
                let plan = self.branch_equations[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("source equation {equation} has no physical branch plan"),
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
                        format!("source equation {equation} has no activation expression"),
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
            (MirEquationKind::Potential | MirEquationKind::Indirect, Reactive::Yes) => {
                let plan = self.branch_equations[equation].ok_or_else(|| {
                    RustBackendError::internal(
                        "",
                        "",
                        format!("source equation {equation} has no physical branch plan"),
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
        initialization: bool,
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
            let _ = writeln!(
                out,
                "{pad}let event_state = if ctx.dynamic_operators_enabled() {{ &*self.event_state_accepted }} else {{ &*self.event_state_candidate }};"
            );
        }
        if wants.multiplicity {
            let _ = writeln!(out, "{pad}let multiplicity = self.multiplicity;");
        }
        if wants.time {
            let source = if initialization { "0.0" } else { "self.time" };
            let _ = writeln!(out, "{pad}let time = {source};");
        }
        if wants.analog_tasks {
            let guard = if initialization {
                ""
            } else {
                "if ctx.analog_tasks_enabled() "
            };
            let _ = writeln!(
                out,
                "{pad}let analog_effects = &mut self.analog_effects;\n\
                 {pad}let mut analog_finish = |site: u32, time: f64, diagnostic: f64| {{\n\
                 {pad}    {guard}{{\n\
                 {pad}        if let Err(source) = analog_effects.get_or_insert_with(Default::default).record_finish(site, time, diagnostic) {{\n\
                 {pad}            ctx.report_analog_task_error(site, source);\n\
                 {pad}        }}\n\
                 {pad}    }}\n\
                 {pad}}};"
            );
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
        if wants.ddt || wants.idt || wants.idt_derivative {
            let _ = writeln!(out, "{pad}let ddt_state = self.stamp_state.as_mut();");
        }
        if wants.idt_derivative {
            let _ = writeln!(
                out,
                "{pad}let idt_derivative_coefficients = self.ddt_coefficients;\n\
                 {pad}let idt_derivative = |slot: usize, primal: f64, input: f64, ic: f64| -> f64 {{\n\
                 {pad}    if !ctx.dynamic_operators_enabled() {{ return 0.0; }}\n\
                 {pad}    let result = if primal.is_finite() {{\n\
                 {pad}        evaluate_generated_idt_derivative(idt_derivative_coefficients, ddt_state.idt_initialized[slot], [input, ic])\n\
                 {pad}    }} else {{ Err(GeneratedIdtCandidateError::NonFiniteResult {{ field: \"primal value\" }}) }};\n\
                 {pad}    match result {{\n\
                 {pad}        Ok(value) => value,\n\
                 {pad}        Err(source) => {{ ctx.report_idt_candidate_error(slot, source); 0.0 }}\n\
                 {pad}    }}\n\
                 {pad}}};"
            );
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
                 {pad}        Ok(evaluation) if ctx.dynamic_operators_enabled() => {{\n\
                 {pad}            self.cross_event_candidate[slot] = evaluation.candidate;\n\
                 {pad}            if let Some(target) = evaluation.refinement_time {{ self.event_refinement_time = self.event_refinement_time.min(target); }}\n\
                 {pad}            evaluation.fired as u8 as f64\n\
                 {pad}        }}\n\
                 {pad}        Ok(_) => 0.0,\n\
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
                 {pad}        Ok(evaluation) if ctx.dynamic_operators_enabled() => {{\n\
                 {pad}            self.cross_event_candidate[slot] = evaluation.candidate;\n\
                 {pad}            if let Some(target) = evaluation.refinement_time {{ self.event_refinement_time = self.event_refinement_time.min(target); }}\n\
                 {pad}            evaluation.fired as u8 as f64\n\
                 {pad}        }}\n\
                 {pad}        Ok(_) => 0.0,\n\
                 {pad}        Err(source) => {{ ctx.report_event_control_error(\"above\", slot, source); 0.0 }}\n\
                 {pad}    }}\n\
                 {pad}}}}}; }}"
            ));
        }
        if wants.last_crossing {
            for value in &function.values {
                if let CfgValueKind::LastCrossing { operator, .. } = &value.kind {
                    self.cross_slots.get(operator).copied().ok_or_else(|| {
                        unsupported(
                            artifact,
                            format!(
                                "a last_crossing at {operator} with no generated detector slot"
                            ),
                        )
                    })?;
                }
            }
            out.push_str(&format!(
                "{pad}macro_rules! rspice_last_crossing {{ ($slot:expr, $value:expr, $direction:expr) => {{{{\n\
                 {pad}    let slot = $slot;\n\
                 {pad}    match evaluate_generated_last_crossing(self.cross_event_accepted[slot], $value, self.time, $direction) {{\n\
                 {pad}        Ok(candidate) => {{\n\
                 {pad}            if ctx.dynamic_operators_enabled() {{ self.cross_event_candidate[slot] = candidate; }}\n\
                 {pad}            if ctx.analysis_tran() {{ candidate.last_crossing_time }} else {{ -1.0 }}\n\
                 {pad}        }}\n\
                 {pad}        Err(source) => {{ ctx.report_event_control_error(\"last_crossing\", slot, source); -1.0 }}\n\
                 {pad}    }}\n\
                 {pad}}}}}; }}"
            ));
        }
        if wants.timer {
            out.push_str(&format!(
                "{pad}macro_rules! rspice_timer {{ ($slot:expr, $start:expr, $period:expr, $time_tol:expr, $enable:expr) => {{{{\n\
                 {pad}    let _ = $slot;\n\
                 {pad}    let (fired, next_event) = evaluate_generated_timer($start, $period, $time_tol, $enable, self.time, self.timestep);\n\
                 {pad}    if ctx.dynamic_operators_enabled() {{\n\
                 {pad}        if let Some(target) = next_event {{ self.timer_event_bound_candidate = self.timer_event_bound_candidate.min(target); }}\n\
                 {pad}        fired as u8 as f64\n\
                 {pad}    }} else {{ 0.0 }}\n\
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
                     {pad}    if !proposed.is_finite() || !candidate.is_finite() {{\n\
                     {pad}        return ctx.checked_derivative_value(proposed, candidate);\n\
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

    fn has_newton_tasks(&self) -> bool {
        self.function
            .values
            .iter()
            .any(|value| matches!(value.kind, CfgValueKind::AnalogTask(_)))
    }

    fn has_analog_tasks(&self) -> bool {
        self.has_newton_tasks()
            || self.initialization.iter().any(|plan| {
                plan.function
                    .values
                    .iter()
                    .any(|value| matches!(value.kind, CfgValueKind::AnalogTask(_)))
            })
    }

    fn state_extensions(
        &self,
        artifact: &CanonicalIrArtifact,
        options: &RustTranspileOptions,
    ) -> state_file::StateFileExtensions {
        let mut extensions = state_file::StateFileExtensions {
            uses_analog_tasks: self.has_analog_tasks(),
            uses_point_analog_tasks: self.has_newton_tasks(),
            uses_initialization: !self.initialization.is_empty(),
            ..Default::default()
        };
        if extensions.uses_analog_tasks {
            let _ = writeln!(
                extensions.instance_fields,
                "    pub(crate) analog_effects: Option<Box<{}::AnalogEffectJournal>>,",
                options.runtime_path
            );
            extensions
                .clone_fields
                .push_str("            analog_effects: self.analog_effects.clone(),\n");
            extensions
                .new_initializers
                .push_str("            analog_effects: None,\n");
        }
        self.push_limit_state_fields(&mut extensions);
        self.push_timestep_bound_state_fields(&mut extensions);
        self.push_discontinuity_state_fields(artifact, &mut extensions);
        self.push_event_control_state_fields(&mut extensions);
        if !self.initialization.is_empty() {
            extensions
                .reset_analysis_state
                .push_str("        self.canonical_initialization_valid = false;\n");
            extensions
                .instance_fields
                .push_str("    pub(crate) canonical_initialization_valid: bool,\n");
            extensions.clone_fields.push_str("            canonical_initialization_valid: self.canonical_initialization_valid,\n");
            extensions
                .new_initializers
                .push_str("            canonical_initialization_valid: false,\n");
            extensions
                .set_parameter_hook
                .push_str("self.canonical_initialization_valid = false;\n");
            extensions
                .set_multiplicity_hook
                .push_str("self.canonical_initialization_valid = false;\n");
            extensions.rollback_flag_count += 1;
            extensions
                .rollback_capture_flags
                .push_str("        flags.push(self.canonical_initialization_valid);\n");
            extensions.rollback_restore_fields.push_str("        let (initialized, remaining) = rollback_flags.split_first().expect(\"initialization rollback flag\");\n        self.canonical_initialization_valid = *initialized;\n        rollback_flags = remaining;\n");
            extensions
                .checkpoint_restore_fields
                .push_str("        self.canonical_initialization_valid = true;\n");
            extensions.validate_advance_state.push_str("        if !self.canonical_initialization_valid { return Err(\"generated analog initialization has not completed\".to_string()); }\n");
            extensions.validate_checkpoint_ready.push_str("        if !self.canonical_initialization_valid { return Err(\"generated analog initialization has not completed\".to_string()); }\n");
        }

        let input_count = self.initialization_inputs.len();
        if input_count != 0 {
            let _ = writeln!(
                extensions.instance_fields,
                "    pub(crate) canonical_initialization_context: Box<[f64; {input_count}]>,"
            );
            extensions.clone_fields.push_str("            canonical_initialization_context: self.canonical_initialization_context.clone(),\n");
            extensions.new_initializers.push_str(
                "            canonical_initialization_context: boxed_zero_f64_array(),\n",
            );
            extensions.rollback_value_count += input_count;
            extensions.rollback_capture_values.push_str(
                "        values.extend_from_slice(&*self.canonical_initialization_context);\n",
            );
            let _ = writeln!(
                extensions.rollback_restore_fields,
                "        let (context, remaining) = rollback_values.split_at({input_count});\n        self.canonical_initialization_context.copy_from_slice(context);\n        rollback_values = remaining;"
            );
            let start = artifact
                .hir
                .variables
                .iter()
                .filter(|variable| variable.is_state)
                .count()
                + extensions.persistent_event_lane_count;
            extensions.persistent_event_lane_count += input_count;
            extensions.checkpoint_event_capture.push_str("        event_variables.extend_from_slice(&*self.canonical_initialization_context);\n");
            for (slot, input) in self.initialization_inputs.iter().enumerate() {
                let invalid = input.invalid(&format!("state.event_variables[{}]", start + slot));
                let _ = writeln!(
                    extensions.checkpoint_event_validate,
                    "        if {invalid} {{ return Err(\"invalid initialization context\".into()); }}"
                );
            }
            let end = start + input_count;
            let _ = writeln!(
                extensions.checkpoint_event_restore,
                "        self.canonical_initialization_context.copy_from_slice(&state.event_variables[{start}..{end}]);"
            );
        }
        extensions.impl_methods.push_str("    /// Restore a trajectory under resolved context without executing initializers.\n    pub fn restore_analysis_continuation_state(&mut self, state: &GeneratedVerilogAPersistentState, ctx: &GeneratedEvalContext<'_>) -> Result<(), String> {\n        if ctx.evaluation_failed() { return Err(\"analysis continuation context has a pending evaluation error\".into()); }\n");
        for (slot, input) in self.initialization_inputs.iter().enumerate() {
            let name = format!("initialization_input_{slot}");
            let _ = writeln!(
                extensions.impl_methods,
                "        let {name} = {};",
                input.expression()
            );
            let invalid = input.invalid(&name);
            let _ = writeln!(
                extensions.impl_methods,
                "        if {invalid} {{ return Err(\"invalid initialization context for analysis continuation\".into()); }}"
            );
        }
        extensions
            .impl_methods
            .push_str("        self.restore_persistent_state(state)?;\n");
        for slot in 0..input_count {
            let _ = writeln!(
                extensions.impl_methods,
                "        self.canonical_initialization_context[{slot}] = initialization_input_{slot};"
            );
        }
        extensions
            .impl_methods
            .push_str("        Ok(())\n    }\n\n");
        let reactive = self.reactive.width() + self.frequency.len() + self.frequency_activity.len();
        if reactive > 0 {
            extensions
                .after_begin_analysis
                .push_str("        self.canonical_reactive.fill(0.0);\n");
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
        extensions.reset_analysis_state.push_str("        self.canonical_limit.previous.fill(0.0);\n        self.canonical_limit.initialized.fill(false);\n        self.canonical_limit.active = false;\n");
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

    fn push_timestep_bound_state_fields(&self, extensions: &mut state_file::StateFileExtensions) {
        if self.timestep_bound_position.is_none() {
            extensions.impl_methods.push_str("    #[inline]\n    pub fn transient_step_bound(&self) -> Result<Option<f64>, String> { Ok(None) }\n");
            return;
        }
        for field in ["timestep_bound_accepted", "timestep_bound_candidate"] {
            let _ = writeln!(extensions.instance_fields, "    pub(crate) {field}: f64,");
            let _ = writeln!(
                extensions.clone_fields,
                "            {field}: self.{field},"
            );
            let _ = writeln!(
                extensions.new_initializers,
                "            {field}: f64::INFINITY,"
            );
            let _ = writeln!(
                extensions.reset_analysis_state,
                "        self.{field} = f64::INFINITY;"
            );
            let _ = writeln!(
                extensions.rollback_capture_values,
                "        values.push(self.{field});"
            );
            let _ = writeln!(
                extensions.rollback_restore_fields,
                "        let (value, remaining) = rollback_values.split_first().expect(\"generated timestep rollback bound\");\n        self.{field} = *value;\n        rollback_values = remaining;"
            );
        }
        extensions.rollback_value_count += 2;
        let lane = Self::event_checkpoint_index(extensions.persistent_event_lane_count);
        extensions.persistent_event_lane_count += 1;
        extensions
            .checkpoint_event_capture
            .push_str("        event_variables.push(self.timestep_bound_accepted);\n");
        let _ = writeln!(
            extensions.checkpoint_event_validate,
            "        let bound = state.event_variables[{lane}];\n        if bound.is_nan() || bound < 0.0 {{ return Err(format!(\"generated $bound_step checkpoint is invalid: {{bound}}\")); }}"
        );
        let _ = writeln!(
            extensions.checkpoint_event_restore,
            "        self.timestep_bound_accepted = state.event_variables[{lane}];\n        self.timestep_bound_candidate = self.timestep_bound_accepted;"
        );
        extensions
            .validate_advance_state
            .push_str("        self.transient_step_bound()?;\n");
        extensions
            .apply_advance_state
            .push_str("        self.timestep_bound_accepted = self.timestep_bound_candidate;\n");
        extensions.impl_methods.push_str(
            "    #[inline]\n\
             pub fn transient_step_bound(&self) -> Result<Option<f64>, String> {\n\
                 let bound = self.timestep_bound_candidate;\n\
                 if bound == f64::INFINITY { return Ok(None); }\n\
                 if !bound.is_finite() || bound < 0.0 { return Err(format!(\"generated $bound_step request is invalid: {bound}\")); }\n\
                 Ok(Some(bound))\n\
             }\n",
        );
    }

    fn push_discontinuity_state_fields(
        &self,
        artifact: &CanonicalIrArtifact,
        extensions: &mut state_file::StateFileExtensions,
    ) {
        let mut conditions = Vec::new();
        for (slot, variable) in artifact
            .hir
            .variables
            .iter()
            .filter(|variable| variable.is_state)
            .enumerate()
        {
            if artifact
                .hir
                .switch_branch_variables
                .binary_search(&variable.id)
                .is_err()
            {
                continue;
            }
            conditions.push(format!(
                "self.event_state_candidate[{slot}] != self.event_state_accepted[{slot}]"
            ));
            let _ = writeln!(
                extensions.validate_advance_state,
                "        if !matches!(self.event_state_candidate[{slot}], 0.0 | 1.0) {{ return Err(\"invalid switch-branch source kind\".to_string()); }}"
            );
            let _ = writeln!(
                extensions.checkpoint_event_validate,
                "        if !matches!(state.event_variables[{slot}], 0.0 | 1.0) {{ return Err(\"invalid checkpoint switch-branch source kind\".to_string()); }}"
            );
        }
        if self.discontinuity_position.is_some() {
            conditions.push(
                "matches!(self.discontinuity_candidate, 1.0 | 3.0) && !self.discontinuity_previous"
                    .into(),
            );
        }
        let condition = if conditions.is_empty() {
            "false".into()
        } else {
            conditions.join(" || ")
        };
        let _ = writeln!(
            extensions.impl_methods,
            "    #[inline]\n    pub fn discontinuity_rising(&self) -> bool {{ {condition} }}"
        );
        if self.discontinuity_position.is_none() {
            return;
        }
        extensions.instance_fields.push_str("    pub(crate) discontinuity_candidate: f64,\n    pub(crate) discontinuity_previous: bool,\n");
        extensions.clone_fields.push_str("            discontinuity_candidate: self.discontinuity_candidate,\n            discontinuity_previous: self.discontinuity_previous,\n");
        extensions.new_initializers.push_str("            discontinuity_candidate: 0.0,\n            discontinuity_previous: false,\n");
        extensions.reset_analysis_state.push_str("        self.discontinuity_candidate = 0.0;\n        self.discontinuity_previous = false;\n");
        extensions.validate_advance_state.push_str("        if !matches!(self.discontinuity_candidate, 0.0 | 1.0 | 2.0 | 3.0) { return Err(\"$discontinuity degree must have a finite integer value >= -1\".to_string()); }\n");
        extensions.apply_advance_state.push_str("        self.discontinuity_previous = matches!(self.discontinuity_candidate, 1.0 | 3.0);\n");
        let converged = "matches!(self.discontinuity_candidate, 0.0 | 1.0)";
        extensions.limiter_converged_expr = if self.limit_slots.is_empty() {
            converged.to_string()
        } else {
            format!("({}) && {converged}", extensions.limiter_converged_expr)
        };
        extensions.rollback_value_count += 1;
        extensions.rollback_flag_count += 1;
        extensions
            .rollback_capture_values
            .push_str("        values.push(self.discontinuity_candidate);\n");
        extensions
            .rollback_capture_flags
            .push_str("        flags.push(self.discontinuity_previous);\n");
        extensions.rollback_restore_fields.push_str("        let (candidate, remaining) = rollback_values.split_first().expect(\"generated discontinuity rollback\");\n        self.discontinuity_candidate = *candidate;\n        rollback_values = remaining;\n        let (previous, remaining) = rollback_flags.split_first().expect(\"generated accepted discontinuity rollback\");\n        self.discontinuity_previous = *previous;\n        rollback_flags = remaining;\n");
        let lane = Self::event_checkpoint_index(extensions.persistent_event_lane_count);
        extensions.persistent_event_lane_count += 1;
        extensions.checkpoint_event_capture.push_str(
            "        event_variables.push(if self.discontinuity_previous { 1.0 } else { 0.0 });\n",
        );
        let _ = writeln!(
            extensions.checkpoint_event_validate,
            "        if !matches!(state.event_variables[{lane}], 0.0 | 1.0) {{ return Err(\"generated discontinuity checkpoint is invalid\".to_string()); }}"
        );
        let _ = writeln!(
            extensions.checkpoint_event_restore,
            "        self.discontinuity_previous = state.event_variables[{lane}] == 1.0;\n        self.discontinuity_candidate = state.event_variables[{lane}];"
        );
    }

    fn event_checkpoint_index(extra_lanes: usize) -> String {
        if extra_lanes == 0 {
            "Self::EVENT_STATE_COUNT".to_string()
        } else {
            format!("Self::EVENT_STATE_COUNT + {extra_lanes}")
        }
    }

    fn push_event_control_state_fields(&self, extensions: &mut state_file::StateFileExtensions) {
        let cross_count = self.cross_slots.len();
        let has_timer = !self.timer_slots.is_empty();
        if cross_count > 0 || has_timer {
            let offset = Self::event_checkpoint_index(extensions.persistent_event_lane_count);
            let prefix = format!(
                "        let mut generated_event_lanes = &state.event_variables[{offset}..];\n"
            );
            extensions.checkpoint_event_validate.push_str(&prefix);
            extensions.checkpoint_event_restore.push_str(&prefix);
        }

        if cross_count > 0 {
            extensions.reset_analysis_state.push_str("        self.cross_event_accepted.fill(GeneratedCrossState::INITIAL);\n        self.cross_event_candidate.fill(GeneratedCrossState::INITIAL);\n        self.event_refinement_time = f64::INFINITY;\n");
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

            let persistent_cross_lanes = cross_count
                .saturating_mul(rspice_veriloga_runtime::GeneratedCrossState::CHECKPOINT_LANES);
            extensions.persistent_event_lane_count = extensions
                .persistent_event_lane_count
                .saturating_add(persistent_cross_lanes);
            extensions.checkpoint_event_capture.push_str(
                "        for state in self.cross_event_accepted.iter().copied() { state.append_checkpoint_lanes(&mut event_variables); }\n",
            );
            let _ = write!(
                extensions.checkpoint_event_validate,
                "        for index in 0..{cross_count} {{\n\
                             let (lanes, remaining) = generated_event_lanes.split_at(GeneratedCrossState::CHECKPOINT_LANES);\n\
                             GeneratedCrossState::from_checkpoint_lanes(lanes).map_err(|error| format!(\"generated crossing checkpoint slot {{index}}: {{error}}\"))?;\n\
                             generated_event_lanes = remaining;\n\
                         }}\n"
            );
            let _ = write!(
                extensions.checkpoint_event_restore,
                "        for target in self.cross_event_accepted.iter_mut() {{\n\
                             let (lanes, remaining) = generated_event_lanes.split_at(GeneratedCrossState::CHECKPOINT_LANES);\n\
                             *target = GeneratedCrossState::from_checkpoint_lanes(lanes)?;\n\
                             generated_event_lanes = remaining;\n\
                         }}\n\
                         self.cross_event_candidate.copy_from_slice(&*self.cross_event_accepted);\n\
                         self.event_refinement_time = f64::INFINITY;\n"
            );
        }

        if has_timer {
            extensions.reset_analysis_state.push_str("        self.timer_event_bound_accepted = f64::INFINITY;\n        self.timer_event_bound_candidate = f64::INFINITY;\n");
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
            CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
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
            CfgTerminator::Return | CfgTerminator::Wait { .. } | CfgTerminator::Unset => {}
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
    analog_tasks: bool,
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
    idt_derivative: bool,
    cross: bool,
    above: bool,
    last_crossing: bool,
    timer: bool,
    limit: bool,
    limit_previous: bool,
    staged: bool,
}

impl Wants {
    fn observe(&mut self, kind: &CfgValueKind) {
        match kind {
            CfgValueKind::AnalogTask(_) => {
                self.analog_tasks = true;
                self.time = true;
            }
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
            CfgValueKind::IntegralDerivative { .. } => self.idt_derivative = true,
            CfgValueKind::Cross { .. } => self.cross = true,
            CfgValueKind::Above { .. } => self.above = true,
            CfgValueKind::LastCrossing { .. } => self.last_crossing = true,
            CfgValueKind::Timer { .. } => self.timer = true,
            CfgValueKind::Limit { .. } => self.limit = true,
            CfgValueKind::LimitPrevious { .. } => self.limit_previous = true,
            CfgValueKind::Staged { .. } => self.staged = true,
            _ => {}
        }
    }
}

fn reject_unsupported_kinds(
    artifact: &CanonicalIrArtifact,
    function: &CfgFunction,
) -> Result<(), RustBackendError> {
    // A process has a canonical form now, but no executable one: this backend
    // emits a device that the analog solver calls, and a process is not called
    // — it suspends and resumes on events. Emitting only the analog half of a
    // module that has processes would produce a device that compiles, runs,
    // and is silently short of what its author wrote.
    if let Some(process) = artifact.digital.processes.first() {
        return Err(unsupported(
            artifact,
            format!(
                "digital process execution: `{}` process {} is lowered but this \
                 backend has no way to run one yet",
                process.kind.keyword(),
                process.id.index()
            ),
        ));
    }
    for value in &function.values {
        match &value.kind {
            CfgValueKind::BranchFlow(branch) => {
                return Err(unsupported(
                    artifact,
                    format!("an unresolved flow probe on {branch}"),
                ));
            }
            // The canonical level represents these; this backend does not run
            // them. Each owns accepted history — a wrapped running total, a
            // transport queue, a rate-limiter state, a crossing detector — that
            // the VM, the native JIT and the WebAssembly JIT keep and this one
            // has no place for. Refusing sends the model to a runtime that
            // does.
            CfgValueKind::IdtMod { .. } => {
                return Err(unsupported(
                    artifact,
                    "stateful idtmod in the direct generated-Rust backend; use the VM, native JIT, or WebAssembly JIT runtime so the wrapped running total is preserved",
                ));
            }
            CfgValueKind::AbsDelay { .. } | CfgValueKind::AbsDelayDerivative { .. } => {
                return Err(unsupported(
                    artifact,
                    "stateful absdelay in the direct generated-Rust backend; use the VM, native JIT, or WebAssembly JIT runtime so the transport history is preserved",
                ));
            }
            CfgValueKind::Slew { .. } | CfgValueKind::SlewDerivative { .. } => {
                return Err(unsupported(
                    artifact,
                    "rate-limited slew in the direct generated-Rust backend; use the VM, native JIT, or WebAssembly JIT runtime so the accepted filter state is preserved",
                ));
            }
            // Named by the spelling the source wrote rather than by the family:
            // a filter's four spellings reduce to two forms here, and a refusal
            // that reported the form would send an author looking for a
            // `laplace_nd` they did not write.
            CfgValueKind::Laplace { operator, .. }
            | CfgValueKind::LaplaceDerivative { operator, .. } => {
                return Err(unsupported(
                    artifact,
                    format!(
                        "a {} filter in the direct generated-Rust backend; use the VM, native JIT, or WebAssembly JIT runtime so the state-space realization and its accepted state are preserved",
                        operator_spelling(artifact, *operator, "laplace")
                    ),
                ));
            }
            CfgValueKind::Zi { operator, .. } | CfgValueKind::ZiDerivative { operator, .. } => {
                return Err(unsupported(
                    artifact,
                    format!(
                        "a {} sampled filter in the direct generated-Rust backend; use the VM, native JIT, or WebAssembly JIT runtime so the sample schedule and its accepted history are preserved",
                        operator_spelling(artifact, *operator, "zi")
                    ),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

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
        integer_result: "ctx.integer_result".into(),
        checked_value: "ctx.checked_derivative_value".into(),
        analysis: "ctx.analysis".into(),
        simparam_required: "ctx.simparam_required".into(),
        simparam_present: "ctx.has_simparam".into(),
        cross: "rspice_cross!".into(),
        above: "rspice_above!".into(),
        last_crossing: "rspice_last_crossing!".into(),
        timer: "rspice_timer!".into(),
        ..EmitBindings::default()
    }
}

fn uses_checked_operations(function: &CfgFunction) -> bool {
    function.values.iter().any(|value| {
        matches!(
            value.kind,
            CfgValueKind::Binary {
                op: CfgBinaryOp::CheckedValue,
                ..
            } | CfgValueKind::IntegerArithmetic { .. }
                | CfgValueKind::IntegerBitwise { .. }
                | CfgValueKind::IntegerBitwiseNot { .. }
                | CfgValueKind::SimParamValue(_)
                | CfgValueKind::SimParamPresent(_)
        )
    })
}

fn integer_context_argument(function: &CfgFunction) -> &'static str {
    if uses_checked_operations(function) {
        "            ctx,\n"
    } else {
        ""
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

/// How the source spelled the operator owning a canonical expression.
///
/// A refusal is read by whoever wrote the model, so it names their operator.
/// The fallback covers the public-AST route, where the operator is a resolved
/// kind rather than a call and there is no spelling to recover.
fn operator_spelling(
    artifact: &CanonicalIrArtifact,
    operator: crate::canonical_ir::ExprId,
    fallback: &'static str,
) -> String {
    match artifact
        .hir
        .expressions
        .get(usize::from(operator))
        .map(|expression| &expression.kind)
    {
        Some(HirExprKind::Call { name, .. } | HirExprKind::SystemFunction { name, .. }) => {
            name.to_ascii_lowercase()
        }
        _ => fallback.to_string(),
    }
}

fn unsupported(artifact: &CanonicalIrArtifact, feature: impl Into<String>) -> RustBackendError {
    RustBackendError::unsupported(
        artifact.metadata.source_package.as_str(),
        artifact.mir.module_name.as_str(),
        feature,
    )
}

#[cfg(test)]
mod accepted_state_shape_tests {
    use super::*;
    use rspice_veriloga_runtime::{GeneratedCheckpointLaneDescriptor, GeneratedCheckpointLaneType};

    const SPAN_A: SourceSpanRef = SourceSpanRef {
        source_file_id: 0,
        start: 10,
        end: 20,
    };
    const SPAN_B: SourceSpanRef = SourceSpanRef {
        source_file_id: 0,
        start: 30,
        end: 40,
    };

    #[test]
    fn ordered_operator_slots_is_exact_and_rejects_malformed_maps() {
        let first = ExprId::from(3usize);
        let second = ExprId::from(9usize);
        let ordered = HashMap::from([(second, 1usize), (first, 0usize)]);
        assert_eq!(ordered_operator_slots(&ordered), Ok(vec![first, second]));

        let duplicate = HashMap::from([(first, 0usize), (second, 0usize)]);
        assert!(
            ordered_operator_slots(&duplicate)
                .expect_err("duplicate slot must fail closed")
                .contains("assigned to both")
        );

        let outside = HashMap::from([(first, 0usize), (second, 2usize)]);
        assert!(
            ordered_operator_slots(&outside)
                .expect_err("out-of-range slot must fail closed")
                .contains("outside")
        );
    }

    fn two_operator_shape(first_family: &str, second_family: &str) -> [u8; 32] {
        let mut shape = AcceptedStateShapeHasher::new();
        shape.section("event_detectors", 2);
        shape.operator(first_family, 0, SPAN_A, None, &["value:f64"]);
        shape.operator(second_family, 1, SPAN_B, None, &["value:f64"]);
        shape.finish()
    }

    #[test]
    fn accepted_state_shape_is_deterministic_and_order_sensitive() {
        let cross_then_above = two_operator_shape("cross", "above");
        assert_eq!(cross_then_above, two_operator_shape("cross", "above"));
        assert_ne!(cross_then_above, two_operator_shape("above", "cross"));
    }

    #[test]
    fn accepted_state_shape_covers_runtime_lane_types() {
        const REAL_LANE: [GeneratedCheckpointLaneDescriptor; 1] =
            [GeneratedCheckpointLaneDescriptor {
                name: "accepted",
                lane_type: GeneratedCheckpointLaneType::NanForbiddenF64,
            }];
        const BOOLEAN_LANE: [GeneratedCheckpointLaneDescriptor; 1] =
            [GeneratedCheckpointLaneDescriptor {
                name: "accepted",
                lane_type: GeneratedCheckpointLaneType::BoolAsF64,
            }];
        let digest = |lanes: &[GeneratedCheckpointLaneDescriptor]| {
            let mut shape = AcceptedStateShapeHasher::new();
            shape.section("probe", 1);
            shape.runtime_operator("cross", 0, SPAN_A, None, lanes);
            shape.finish()
        };

        assert_ne!(digest(&REAL_LANE), digest(&BOOLEAN_LANE));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::canonical_ir::{CfgEvalInputs, evaluate_cfg};
    use crate::{CompilerOptions, VerilogACompiler};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn root_pruning_keeps_ddx_that_controls_a_stamp_path() {
        let source = r#"
module conditional_ddx(p);
    inout p;
    electrical p;
    analog begin
        if (ddx(V(p) * V(p), V(p)) > 1.0)
            I(p) <+ 7.0;
        else
            I(p) <+ 3.0;
    end
endmodule
"#;
        let artifact = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(source)
            .expect("conditional ddx fixture compiles");
        let mut measurements =
            MetricsRecorder::new(0, crate::metrics::PerformanceBudget::default());
        let plan = ModelPlan::build(&artifact, &mut measurements)
            .expect("conditional ddx must survive generated-stamp planning");
        let residual = plan.conduction.rows[0].residual;
        let snapshot = evaluate_cfg(
            &plan.function,
            &CfgEvalInputs {
                parameters: Vec::new(),
                parameter_given: Vec::new(),
                event_state: Vec::new(),
                event_controls: HashMap::new(),
                port_connected: vec![true],
                node_potentials: vec![0.75],
                branch_flows: Vec::new(),
                branch_unknown_flows: Vec::new(),
                temperature: 300.15,
                thermal_voltage: 300.15 * rspice_veriloga_runtime::THERMAL_VOLTAGE_PER_K,
                multiplicity: 1.0,
                time: 0.0,
                analyses: HashSet::new(),
                simparams: Default::default(),
                ddt: 0.0,
                ddt_scale: 0.0,
                idt: 0.0,
                idt_scale: 0.0,
                integral_derivatives: HashMap::new(),
                staged: Vec::new(),
            },
        )
        .expect("optimized conditional ddx CFG evaluates");
        let value = snapshot.value(residual).expect("residual is defined");
        assert_eq!(value, 7.0, "ddx=1.5 must select the true contribution");
    }
}

//! Runtime ABI for build-time generated Verilog-A devices.
//!
//! Generated device modules call this small surface directly from their
//! hand-emitted Rust stamps. Keep it narrow, deterministic, and free of
//! interpreter concepts.

use crate::Value;
use crate::solver::{ComplexMatrix, CscIndex, StaticMatrix};

pub(crate) mod limiting;

#[cfg(feature = "veriloga-builtins")]
#[allow(clippy::all)]
pub mod builtins {
    include!("registry.rs");
}

#[cfg(feature = "veriloga-builtins")]
#[rustfmt::skip]
#[allow(clippy::all)]
pub(crate) mod kernel_runtime;

#[derive(Debug, Clone, Copy)]
pub struct GeneratedDdtCoefficients {
    pub active: bool,
    pub derivative_scale: Value,
    pub previous_value_scale: Value,
    pub older_value_scale: Value,
    pub previous_derivative_scale: Value,
}

impl GeneratedDdtCoefficients {
    #[inline]
    pub const fn inactive() -> Self {
        Self {
            active: false,
            derivative_scale: 0.0,
            previous_value_scale: 0.0,
            older_value_scale: 0.0,
            previous_derivative_scale: 0.0,
        }
    }

    #[inline]
    pub fn from_companion(
        coefficients: &crate::analysis::CompanionCoefficients,
        timestep: Value,
    ) -> Self {
        const DDT_EPSILON: Value = 1.0e-20;
        if !timestep.is_finite() || timestep.abs() <= DDT_EPSILON {
            return Self::inactive();
        }

        let inverse_timestep = 1.0 / timestep;
        Self {
            active: true,
            derivative_scale: coefficients.coeff_g * inverse_timestep,
            previous_value_scale: coefficients.coeff_v_n * inverse_timestep,
            older_value_scale: if coefficients.needs_two_history {
                coefficients.coeff_v_n_minus_1 * inverse_timestep
            } else {
                0.0
            },
            previous_derivative_scale: if coefficients.needs_current_history {
                1.0
            } else {
                0.0
            },
        }
    }
}

impl Default for GeneratedDdtCoefficients {
    #[inline]
    fn default() -> Self {
        Self::inactive()
    }
}

/// Canonical Verilog-A noise primitive represented by generated device code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedNoiseKind {
    White,
    Flicker,
    Table,
}

/// One endpoint of a generated noise contribution in device-local topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoiseEndpoint {
    /// Index into the generated instance's complete node array. `None` is ground.
    pub local_node: Option<usize>,
    pub name: &'static str,
    pub is_internal: bool,
}

/// Immutable metadata emitted from a canonical Verilog-A noise contribution.
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

/// Evaluated, frequency-independent data for one generated noise primitive.
///
/// Table operands retain their canonical flat ordering. Interpretation and
/// interpolation belong to the analysis layer, not the generated-device ABI.
#[derive(Debug, Clone, PartialEq)]
pub struct GeneratedNoiseEvaluation {
    pub active: bool,
    pub psd: Value,
    pub exponent: Option<Value>,
    pub table_operands: Vec<Value>,
}

/// A generated noise evaluator rejected invalid model state or output.
#[derive(Debug, Clone, PartialEq)]
pub enum GeneratedNoiseEvaluationError {
    SourceIndexOutOfRange {
        index: usize,
        count: usize,
    },
    NonFinite {
        index: usize,
        quantity: &'static str,
        value: Value,
    },
    NegativePower {
        index: usize,
        value: Value,
    },
    InvalidMultiplicity {
        value: Value,
    },
}

impl std::fmt::Display for GeneratedNoiseEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceIndexOutOfRange { index, count } => write!(
                f,
                "generated Verilog-A noise source index {index} is outside the {count}-source catalog"
            ),
            Self::NonFinite {
                index,
                quantity,
                value,
            } => write!(
                f,
                "generated Verilog-A noise source {index} produced non-finite {quantity} {value}"
            ),
            Self::NegativePower { index, value } => write!(
                f,
                "generated Verilog-A noise source {index} produced negative power {value}"
            ),
            Self::InvalidMultiplicity { value } => write!(
                f,
                "generated Verilog-A noise evaluation requires a finite positive multiplicity, found {value}"
            ),
        }
    }
}

impl std::error::Error for GeneratedNoiseEvaluationError {}

/// Engine-neutral location at which a generated noise source is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedNoiseInjection {
    Current {
        node_pos: usize,
        node_neg: usize,
    },
    /// A potential-noise EMF applied to the equation for this concrete 1-based
    /// circuit branch ordinal returned by `CircuitData::allocate_branch`. This
    /// is not the descriptor's device-local branch ordinal; the analysis layer
    /// owns its conversion to a matrix axis.
    Potential {
        branch: usize,
    },
}

/// Canonical descriptor paired with topology mapped to a concrete instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedMappedNoiseDescriptor {
    pub descriptor: GeneratedNoiseDescriptor,
    pub injection: GeneratedNoiseInjection,
}

/// Generated noise topology was inconsistent with its concrete instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratedNoiseTopologyError {
    LocalNodeOutOfRange {
        endpoint: &'static str,
        local_node: usize,
        node_count: usize,
    },
    CurrentSourceHasBranch {
        branch_ordinal: usize,
    },
    PotentialSourceMissingBranch,
    BranchOrdinalOutOfRange {
        branch_ordinal: usize,
        branch_count: usize,
    },
}

impl std::fmt::Display for GeneratedNoiseTopologyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalNodeOutOfRange {
                endpoint,
                local_node,
                node_count,
            } => write!(
                f,
                "generated Verilog-A noise {endpoint} endpoint references local node {local_node}, but the instance has {node_count} nodes"
            ),
            Self::CurrentSourceHasBranch { branch_ordinal } => write!(
                f,
                "generated Verilog-A current-noise source unexpectedly references branch ordinal {branch_ordinal}"
            ),
            Self::PotentialSourceMissingBranch => write!(
                f,
                "generated Verilog-A potential-noise source has no branch equation"
            ),
            Self::BranchOrdinalOutOfRange {
                branch_ordinal,
                branch_count,
            } => write!(
                f,
                "generated Verilog-A potential-noise source references branch ordinal {branch_ordinal}, but the instance has {branch_count} branches"
            ),
        }
    }
}

impl std::error::Error for GeneratedNoiseTopologyError {}

impl GeneratedNoiseDescriptor {
    /// Validate and map device-local topology without assuming an engine matrix
    /// layout. Ground is represented by circuit node zero.
    pub fn map_topology(
        self,
        nodes: &[usize],
        branches: &[usize],
    ) -> Result<GeneratedMappedNoiseDescriptor, GeneratedNoiseTopologyError> {
        // Validate both canonical endpoints even when a potential contribution
        // is ultimately injected on its branch-equation row. This keeps corrupt
        // generated metadata from being silently masked by injection topology.
        let node_pos = map_generated_noise_endpoint(self.pos, "positive", nodes)?;
        let node_neg = map_generated_noise_endpoint(self.neg, "negative", nodes)?;
        let injection = if self.is_current {
            if let Some(branch_ordinal) = self.branch_ordinal {
                return Err(GeneratedNoiseTopologyError::CurrentSourceHasBranch { branch_ordinal });
            }
            GeneratedNoiseInjection::Current { node_pos, node_neg }
        } else {
            let branch_ordinal = self
                .branch_ordinal
                .ok_or(GeneratedNoiseTopologyError::PotentialSourceMissingBranch)?;
            let branch = branches.get(branch_ordinal).copied().ok_or(
                GeneratedNoiseTopologyError::BranchOrdinalOutOfRange {
                    branch_ordinal,
                    branch_count: branches.len(),
                },
            )?;
            GeneratedNoiseInjection::Potential { branch }
        };
        Ok(GeneratedMappedNoiseDescriptor {
            descriptor: self,
            injection,
        })
    }
}

fn map_generated_noise_endpoint(
    endpoint: GeneratedNoiseEndpoint,
    endpoint_role: &'static str,
    nodes: &[usize],
) -> Result<usize, GeneratedNoiseTopologyError> {
    endpoint.local_node.map_or(Ok(0), |local_node| {
        nodes
            .get(local_node)
            .copied()
            .ok_or(GeneratedNoiseTopologyError::LocalNodeOutOfRange {
                endpoint: endpoint_role,
                local_node,
                node_count: nodes.len(),
            })
    })
}

#[cfg(feature = "veriloga-builtins")]
#[derive(Clone)]
pub struct BuiltinVerilogAInstance {
    pub model_name: &'static str,
    pub instance_name: String,
    pub nodes: Vec<usize>,
    pub branches: Vec<usize>,
    temperature: Value,
    analysis_initial_step: bool,
    analysis_final_step: bool,
    static_stamp_cache: GeneratedStaticStampCache,
    kind: builtins::GeneratedBuiltinKind,
}

#[cfg(feature = "veriloga-builtins")]
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BuiltinEvaluatedNoiseSource {
    pub mapped: GeneratedMappedNoiseDescriptor,
    pub evaluation: GeneratedNoiseEvaluation,
}

#[cfg(feature = "veriloga-builtins")]
#[derive(Debug)]
pub(crate) enum BuiltinNoiseEvaluationError {
    Topology {
        index: usize,
        mechanism: &'static str,
        source: GeneratedNoiseTopologyError,
    },
    Evaluation {
        index: usize,
        mechanism: &'static str,
        source: GeneratedNoiseEvaluationError,
    },
}

#[cfg(feature = "veriloga-builtins")]
impl std::fmt::Display for BuiltinNoiseEvaluationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Topology {
                index,
                mechanism,
                source,
            } => write!(
                f,
                "noise source {index} ('{mechanism}') has invalid topology: {source}"
            ),
            Self::Evaluation {
                index,
                mechanism,
                source,
            } => write!(
                f,
                "noise source {index} ('{mechanism}') evaluation failed: {source}"
            ),
        }
    }
}

#[cfg(feature = "veriloga-builtins")]
impl std::error::Error for BuiltinNoiseEvaluationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Topology { source, .. } => Some(source),
            Self::Evaluation { source, .. } => Some(source),
        }
    }
}

#[cfg(feature = "veriloga-builtins")]
impl std::fmt::Debug for BuiltinVerilogAInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltinVerilogAInstance")
            .field("model_name", &self.model_name)
            .field("instance_name", &self.instance_name)
            .field("nodes", &self.nodes)
            .field("branches", &self.branches)
            .field("temperature", &self.temperature)
            .finish_non_exhaustive()
    }
}

#[cfg(feature = "veriloga-builtins")]
#[derive(Debug, Clone, Default)]
pub struct BuiltinVerilogADevices {
    devices: Vec<BuiltinVerilogAInstance>,
}

#[cfg(feature = "veriloga-builtins")]
impl BuiltinVerilogADevices {
    #[inline]
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    #[inline]
    pub fn add(&mut self, device: BuiltinVerilogAInstance) {
        self.devices.push(device);
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.devices.len()
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &BuiltinVerilogAInstance> {
        self.devices.iter()
    }

    #[inline]
    pub(crate) fn restore_from_snapshot(&mut self, snapshot: Self) {
        if self.devices.len() == snapshot.devices.len() {
            for (active, snapshot) in self.devices.iter_mut().zip(snapshot.devices) {
                active.restore_from_snapshot(snapshot);
            }
        } else {
            *self = snapshot;
        }
    }

    pub fn link_static_stamps(&mut self, matrix: &StaticMatrix, num_nodes: usize) {
        for device in &mut self.devices {
            device.link_static_stamps(matrix, num_nodes);
        }
    }

    pub fn stamp_all(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        simparams: GeneratedSimulationParameters,
    ) {
        self.stamp_all_with_mode(
            matrix,
            rhs,
            voltages,
            num_nodes,
            analysis,
            simparams,
            GeneratedEvaluationMode::default_for_analysis(analysis),
        );
    }

    pub(crate) fn stamp_all_with_mode(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        simparams: GeneratedSimulationParameters,
        evaluation_mode: GeneratedEvaluationMode,
    ) {
        for device in &mut self.devices {
            device.stamp_with_mode(
                matrix,
                rhs,
                voltages,
                num_nodes,
                analysis,
                simparams,
                evaluation_mode,
            );
        }
    }

    #[inline]
    pub fn all_converged(&self) -> bool {
        self.devices
            .iter()
            .all(BuiltinVerilogAInstance::is_converged)
    }

    #[inline]
    pub fn set_temperature(&mut self, temperature: Value) {
        for device in &mut self.devices {
            device.set_temperature(temperature);
        }
    }

    #[inline]
    pub fn set_timepoint(
        &mut self,
        time: Value,
        timestep: Value,
        ddt_coefficients: GeneratedDdtCoefficients,
    ) {
        for device in &mut self.devices {
            device.set_timepoint(time, timestep, ddt_coefficients);
        }
    }

    #[inline]
    pub fn set_analysis_step(&mut self, initial: bool, final_step: bool) {
        for device in &mut self.devices {
            device.set_analysis_step(initial, final_step);
        }
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        for device in &mut self.devices {
            device.accept_timestep();
        }
    }

    pub fn stamp_ac_real_all(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        simparams: GeneratedSimulationParameters,
    ) {
        for device in &mut self.devices {
            device.stamp_ac_real(matrix, voltages, num_nodes, simparams);
        }
    }

    pub fn stamp_reactive_all(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        omega: Value,
        simparams: GeneratedSimulationParameters,
    ) {
        for device in &mut self.devices {
            device.stamp_reactive(matrix, voltages, num_nodes, omega, simparams);
        }
    }
}

#[cfg(feature = "veriloga-builtins")]
impl BuiltinVerilogAInstance {
    #[inline]
    pub(crate) fn restore_from_snapshot(&mut self, snapshot: Self) {
        debug_assert_eq!(self.model_name, snapshot.model_name);
        debug_assert_eq!(self.instance_name, snapshot.instance_name);
        debug_assert_eq!(self.nodes, snapshot.nodes);
        debug_assert_eq!(self.branches, snapshot.branches);
        self.temperature = snapshot.temperature;
        self.analysis_initial_step = snapshot.analysis_initial_step;
        self.analysis_final_step = snapshot.analysis_final_step;
        self.kind.restore_from_snapshot(snapshot.kind);
    }

    #[inline]
    pub fn is_converged(&self) -> bool {
        self.kind.limiter_converged()
    }

    #[inline]
    pub fn link_static_stamps(&mut self, matrix: &StaticMatrix, num_nodes: usize) {
        self.static_stamp_cache
            .link(matrix, &self.nodes, &self.branches, num_nodes);
    }

    pub(crate) fn evaluate_noise_sources(
        &self,
        voltages: &[Value],
        num_nodes: usize,
        simparams: GeneratedSimulationParameters,
    ) -> Result<Vec<BuiltinEvaluatedNoiseSource>, BuiltinNoiseEvaluationError> {
        let ctx = GeneratedEvalContext::with_analysis_step_and_simparams(
            voltages,
            self.temperature,
            num_nodes,
            GeneratedAnalysisKind::Noise,
            // Noise-source collection begins a new analysis after the DC
            // operating point. Canonical compact models initialize model and
            // temperature state under $analysis("initial_step"), so stale DC
            // step flags must not leak into this evaluation.
            true,
            false,
            simparams,
        );
        self.kind
            .noise_descriptors()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, descriptor)| {
                let mapped = descriptor
                    .map_topology(&self.nodes, &self.branches)
                    .map_err(|source| BuiltinNoiseEvaluationError::Topology {
                        index,
                        mechanism: descriptor.mechanism,
                        source,
                    })?;
                let evaluation =
                    self.kind
                        .evaluate_noise_source(index, &ctx)
                        .map_err(|source| BuiltinNoiseEvaluationError::Evaluation {
                            index,
                            mechanism: descriptor.mechanism,
                            source,
                        })?;
                Ok(BuiltinEvaluatedNoiseSource { mapped, evaluation })
            })
            .collect()
    }

    #[inline]
    pub fn stamp(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        simparams: GeneratedSimulationParameters,
    ) {
        self.stamp_with_mode(
            matrix,
            rhs,
            voltages,
            num_nodes,
            analysis,
            simparams,
            GeneratedEvaluationMode::default_for_analysis(analysis),
        );
    }

    fn stamp_with_mode(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        simparams: GeneratedSimulationParameters,
        evaluation_mode: GeneratedEvaluationMode,
    ) {
        self.static_stamp_cache
            .ensure_axis_indices(&self.nodes, &self.branches, num_nodes);
        let ctx = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            voltages,
            self.temperature,
            num_nodes,
            analysis,
            self.analysis_initial_step,
            self.analysis_final_step,
            simparams,
            evaluation_mode,
        );
        let mut stamper = GeneratedStamper::new_with_static_cache(
            matrix,
            rhs,
            voltages,
            num_nodes,
            &self.static_stamp_cache,
        );
        self.kind.stamp(&ctx, &mut stamper);
    }

    #[inline]
    pub fn set_temperature(&mut self, temperature: Value) {
        if temperature.is_finite() && temperature > 0.0 {
            self.temperature = temperature;
        }
    }

    #[inline]
    pub fn set_timepoint(
        &mut self,
        time: Value,
        timestep: Value,
        ddt_coefficients: GeneratedDdtCoefficients,
    ) {
        self.kind.set_timepoint(time, timestep, ddt_coefficients);
    }

    #[inline]
    pub fn set_analysis_step(&mut self, initial: bool, final_step: bool) {
        self.analysis_initial_step = initial;
        self.analysis_final_step = final_step;
    }

    #[inline]
    pub fn accept_timestep(&mut self) {
        self.kind.accept_timestep();
    }

    #[inline]
    pub fn stamp_ac_real(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        simparams: GeneratedSimulationParameters,
    ) {
        let ctx = GeneratedEvalContext::with_analysis_step_and_simparams(
            voltages,
            self.temperature,
            num_nodes,
            GeneratedAnalysisKind::Ac,
            self.analysis_initial_step,
            self.analysis_final_step,
            simparams,
        );
        self.static_stamp_cache
            .ensure_axis_indices(&self.nodes, &self.branches, num_nodes);
        let mut stamper = GeneratedStamper::new_ac_real_with_static_cache(
            matrix,
            voltages,
            num_nodes,
            &self.static_stamp_cache,
        );
        self.kind.stamp(&ctx, &mut stamper);
    }

    #[inline]
    pub fn stamp_reactive(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        omega: Value,
        simparams: GeneratedSimulationParameters,
    ) {
        let ctx = GeneratedEvalContext::with_analysis_step_and_simparams(
            voltages,
            self.temperature,
            num_nodes,
            GeneratedAnalysisKind::Ac,
            self.analysis_initial_step,
            self.analysis_final_step,
            simparams,
        );
        self.static_stamp_cache
            .ensure_axis_indices(&self.nodes, &self.branches, num_nodes);
        let mut stamper = GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
            matrix,
            &self.nodes,
            &self.branches,
            num_nodes,
            omega,
            &self.static_stamp_cache,
        );
        self.kind.stamp_reactive(&ctx, &mut stamper);
    }
}

#[cfg(feature = "veriloga-builtins")]
pub fn instantiate_builtin(
    model_name: &str,
    instance_name: &str,
    node_names: &[String],
    params: &[(String, crate::netlist::ParametricValue)],
    param_ctx: &crate::netlist::ParamContext,
    circuit: &mut crate::CircuitData,
) -> Result<Option<BuiltinVerilogAInstance>, crate::engine::SimulationError> {
    let Some(descriptor_name) = builtins::builtin_names()
        .iter()
        .find(|name| name.eq_ignore_ascii_case(model_name))
        .copied()
    else {
        return Ok(None);
    };

    let expected_nodes = builtins::node_count(descriptor_name).unwrap_or(0);
    if node_names.len() != expected_nodes {
        return Err(crate::engine::SimulationError::Circuit(format!(
            "Generated Verilog-A instance '{}' expects {} terminals for model '{}', found {}",
            instance_name,
            expected_nodes,
            model_name,
            node_names.len()
        )));
    }

    let internal_node_names = builtins::internal_node_names(descriptor_name).unwrap_or(&[]);
    let total_nodes = builtins::total_node_count(descriptor_name)
        .unwrap_or(expected_nodes + internal_node_names.len());

    let mut nodes = Vec::with_capacity(total_nodes);
    for node_name in node_names {
        nodes.push(if node_name.eq_ignore_ascii_case("0") {
            0
        } else {
            circuit.get_or_create_node(node_name)
        });
    }
    for internal_name in internal_node_names {
        let node_name = format!("{instance_name}.__{internal_name}.internal");
        nodes.push(circuit.get_or_create_node(&node_name));
    }
    debug_assert_eq!(
        nodes.len(),
        total_nodes,
        "generated Verilog-A node metadata is internally inconsistent"
    );

    let mut resolved = Vec::with_capacity(params.len());
    for (name, value) in params {
        let value = match value {
            crate::netlist::ParametricValue::Resolved(value) => *value,
            crate::netlist::ParametricValue::Expression(expr) => {
                crate::netlist::expr::eval_expression(expr, param_ctx).map_err(|error| {
                    crate::engine::SimulationError::Circuit(format!(
                        "Failed to resolve generated Verilog-A parameter '{}': {}",
                        name, error
                    ))
                })?
            }
            crate::netlist::ParametricValue::String(_)
            | crate::netlist::ParametricValue::StringExpression(_) => {
                return Err(crate::engine::SimulationError::Circuit(format!(
                    "Generated Verilog-A parameter '{name}' requires a numeric value"
                )));
            }
        };
        resolved.push((name.clone(), value));
    }

    let branch_count = builtins::branch_count(descriptor_name).unwrap_or(0);
    let mut branches = Vec::with_capacity(branch_count);
    for _ in 0..branch_count {
        branches.push(circuit.allocate_branch());
    }

    let Some(kind) =
        builtins::instantiate(descriptor_name, &nodes, &branches, &resolved).map_err(|error| {
            crate::engine::SimulationError::Circuit(format!(
                "Failed to instantiate generated Verilog-A instance '{}': {}",
                instance_name, error
            ))
        })?
    else {
        return Ok(None);
    };

    Ok(Some(BuiltinVerilogAInstance {
        model_name: descriptor_name,
        instance_name: instance_name.to_string(),
        nodes,
        branches,
        temperature: crate::constants::TEMP_REFERENCE,
        analysis_initial_step: false,
        analysis_final_step: false,
        static_stamp_cache: GeneratedStaticStampCache::default(),
        kind,
    }))
}

#[derive(Debug, Clone, Copy)]
pub enum GeneratedAnalysisKind {
    Dc,
    Ac,
    Tran,
    Noise,
    Ic,
}

/// Controls whether a generated evaluation may apply Newton limiting and
/// advance per-instance limiter history. This is intentionally orthogonal to
/// the physical analysis queried through Verilog-A `$analysis(...)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratedEvaluationMode {
    NewtonLimited,
    StaticProbe,
    SmallSignal,
}

impl GeneratedEvaluationMode {
    #[inline]
    pub const fn default_for_analysis(analysis: GeneratedAnalysisKind) -> Self {
        match analysis {
            GeneratedAnalysisKind::Ac | GeneratedAnalysisKind::Noise => Self::SmallSignal,
            GeneratedAnalysisKind::Dc | GeneratedAnalysisKind::Tran | GeneratedAnalysisKind::Ic => {
                Self::NewtonLimited
            }
        }
    }
}

/// Simulator-owned parameters visible to generated Verilog-A `$simparam` calls.
///
/// `Option` distinguishes an explicitly configured zero from an unavailable
/// parameter, in which case the model-provided fallback remains authoritative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedSimulationParameters {
    gmin: Option<Value>,
    pnjmaxi: Option<Value>,
}

impl GeneratedSimulationParameters {
    #[inline]
    pub const fn new() -> Self {
        Self {
            gmin: Some(crate::constants::GMIN),
            pnjmaxi: None,
        }
    }

    #[inline]
    pub fn set_gmin(&mut self, value: Value) {
        self.gmin = value.is_finite().then_some(value.max(0.0));
    }

    #[inline]
    pub fn set_pnjmaxi(&mut self, value: Option<Value>) {
        self.pnjmaxi = value.filter(|value| value.is_finite() && *value >= 0.0);
    }

    #[inline]
    pub fn get(&self, name: &str) -> Option<Value> {
        if name.eq_ignore_ascii_case("gmin") {
            self.gmin
        } else if name.eq_ignore_ascii_case("pnjmaxi") {
            self.pnjmaxi
        } else {
            None
        }
    }
}

impl Default for GeneratedSimulationParameters {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratedAnalysisKind {
    #[inline]
    fn matches_query(self, query: &str) -> bool {
        match query {
            "dc" | "op" => matches!(self, Self::Dc),
            "ac" => matches!(self, Self::Ac),
            "tran" => matches!(self, Self::Tran),
            "noise" => matches!(self, Self::Noise),
            "ic" => matches!(self, Self::Ic),
            "static" => matches!(self, Self::Dc | Self::Ic),
            "smallsig" => matches!(self, Self::Ac | Self::Noise),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GeneratedEvalContext<'a> {
    voltages: &'a [Value],
    temperature: Value,
    num_nodes: usize,
    analysis: GeneratedAnalysisKind,
    analysis_initial_step: bool,
    analysis_final_step: bool,
    simparams: GeneratedSimulationParameters,
    evaluation_mode: GeneratedEvaluationMode,
}

impl<'a> GeneratedEvalContext<'a> {
    #[inline]
    pub fn new(voltages: &'a [Value], temperature: Value, num_nodes: usize) -> Self {
        Self::with_analysis(voltages, temperature, num_nodes, GeneratedAnalysisKind::Dc)
    }

    #[inline]
    pub fn with_analysis(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
    ) -> Self {
        Self::with_analysis_step(voltages, temperature, num_nodes, analysis, false, false)
    }

    #[inline]
    pub fn with_analysis_step(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
    ) -> Self {
        Self::with_analysis_step_and_simparams(
            voltages,
            temperature,
            num_nodes,
            analysis,
            initial,
            final_step,
            GeneratedSimulationParameters::default(),
        )
    }

    #[inline]
    pub fn with_analysis_step_and_simparams(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
        simparams: GeneratedSimulationParameters,
    ) -> Self {
        let evaluation_mode = GeneratedEvaluationMode::default_for_analysis(analysis);
        Self::with_analysis_step_simparams_and_mode(
            voltages,
            temperature,
            num_nodes,
            analysis,
            initial,
            final_step,
            simparams,
            evaluation_mode,
        )
    }

    #[inline]
    pub fn with_analysis_step_simparams_and_mode(
        voltages: &'a [Value],
        temperature: Value,
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        initial: bool,
        final_step: bool,
        simparams: GeneratedSimulationParameters,
        evaluation_mode: GeneratedEvaluationMode,
    ) -> Self {
        Self {
            voltages,
            temperature,
            num_nodes,
            analysis,
            analysis_initial_step: initial,
            analysis_final_step: final_step,
            simparams,
            evaluation_mode,
        }
    }

    #[inline]
    pub fn limiting_enabled(&self) -> bool {
        matches!(self.evaluation_mode, GeneratedEvaluationMode::NewtonLimited)
    }

    #[inline]
    pub fn simparam_or(&self, name: &str, fallback: Value) -> Value {
        self.simparams.get(name).unwrap_or(fallback)
    }

    #[inline]
    pub fn has_simparam(&self, name: &str) -> bool {
        self.simparams.get(name).is_some()
    }

    #[inline]
    pub fn node_voltage(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn temperature(&self) -> Value {
        self.temperature
    }

    #[inline]
    pub fn thermal_voltage(&self) -> Value {
        crate::constants::thermal_voltage(self.temperature)
    }

    #[inline]
    pub fn branch_current(&self, branch_ordinal: usize) -> Value {
        if branch_ordinal == 0 {
            0.0
        } else {
            self.voltages
                .get(self.num_nodes + branch_ordinal - 1)
                .copied()
                .unwrap_or(0.0)
        }
    }

    #[inline]
    pub fn analysis(&self, query: &str) -> bool {
        self.analysis.matches_query(query)
    }

    #[inline]
    pub fn analysis_dc(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Dc)
    }

    #[inline]
    pub fn analysis_ac(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Ac)
    }

    #[inline]
    pub fn analysis_tran(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Tran)
    }

    #[inline]
    pub fn analysis_noise(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Noise)
    }

    #[inline]
    pub fn analysis_ic(&self) -> bool {
        matches!(self.analysis, GeneratedAnalysisKind::Ic)
    }

    #[inline]
    pub fn analysis_static(&self) -> bool {
        matches!(
            self.analysis,
            GeneratedAnalysisKind::Dc | GeneratedAnalysisKind::Ic
        )
    }

    #[inline]
    pub fn analysis_smallsig(&self) -> bool {
        matches!(
            self.analysis,
            GeneratedAnalysisKind::Ac | GeneratedAnalysisKind::Noise
        )
    }

    #[inline]
    pub fn analysis_initial_step(&self) -> bool {
        self.analysis_initial_step
    }

    #[inline]
    pub fn analysis_final_step(&self) -> bool {
        self.analysis_final_step
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeneratedStaticStampCache {
    node_count: usize,
    branch_count: usize,
    node_axes: Vec<Option<usize>>,
    branch_axes: Vec<Option<usize>>,
    axis_matrix_indices: Vec<Option<usize>>,
    matrix_axis_lookup: Vec<(usize, usize)>,
    slots: Vec<Option<CscIndex>>,
}

impl GeneratedStaticStampCache {
    #[inline]
    pub fn link(
        &mut self,
        matrix: &StaticMatrix,
        nodes: &[usize],
        branches: &[usize],
        num_nodes: usize,
    ) {
        self.rebuild_axis_indices(nodes, branches, num_nodes);
        let width = self.axis_count();
        self.slots.clear();
        self.slots.resize(width * width, None);
        for row_axis in 0..width {
            let Some(row) = self.axis_matrix_indices[row_axis] else {
                continue;
            };
            for col_axis in 0..width {
                let Some(col) = self.axis_matrix_indices[col_axis] else {
                    continue;
                };
                self.slots[row_axis * width + col_axis] = matrix.get_index(row, col);
            }
        }
    }

    #[inline]
    pub fn ensure_axis_indices(&mut self, nodes: &[usize], branches: &[usize], num_nodes: usize) {
        let axis_count = nodes.len() + branches.len();
        let expected_first_branch = branches
            .first()
            .copied()
            .map(|branch| num_nodes + branch - 1);
        let first_branch_axis = nodes.len();
        let branch_index_matches = match expected_first_branch {
            Some(expected) => {
                self.axis_matrix_indices
                    .get(first_branch_axis)
                    .copied()
                    .flatten()
                    == Some(expected)
            }
            None => true,
        };
        if self.axis_matrix_indices.len() != axis_count || !branch_index_matches {
            self.rebuild_axis_indices(nodes, branches, num_nodes);
            self.slots.clear();
        }
    }

    #[inline]
    fn rebuild_axis_indices(&mut self, nodes: &[usize], branches: &[usize], num_nodes: usize) {
        self.node_count = nodes.len();
        self.branch_count = branches.len();
        self.axis_matrix_indices.clear();
        self.axis_matrix_indices
            .reserve(self.node_count + self.branch_count);
        self.axis_matrix_indices
            .extend(nodes.iter().copied().map(Self::node_matrix_index));
        self.axis_matrix_indices.extend(
            branches
                .iter()
                .copied()
                .map(|branch| Self::branch_matrix_index(num_nodes, branch)),
        );

        self.node_axes.clear();
        self.node_axes.reserve(self.node_count);
        self.node_axes.extend(
            self.axis_matrix_indices
                .iter()
                .take(self.node_count)
                .enumerate()
                .map(|(axis, matrix_index)| matrix_index.is_some().then_some(axis)),
        );

        self.branch_axes.clear();
        self.branch_axes.reserve(self.branch_count);
        self.branch_axes.extend(
            self.axis_matrix_indices
                .iter()
                .skip(self.node_count)
                .take(self.branch_count)
                .enumerate()
                .map(|(branch, matrix_index)| {
                    matrix_index.is_some().then_some(self.node_count + branch)
                }),
        );

        self.matrix_axis_lookup.clear();
        self.matrix_axis_lookup.extend(
            self.axis_matrix_indices
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(axis, matrix_index)| matrix_index.map(|index| (index, axis))),
        );
        self.matrix_axis_lookup
            .sort_unstable_by_key(|&(matrix_index, axis)| (matrix_index, axis));
    }

    #[inline]
    fn axis_count(&self) -> usize {
        self.node_count + self.branch_count
    }

    #[inline]
    fn node_axis(&self, node_index: usize) -> Option<usize> {
        self.node_axes.get(node_index).copied().flatten()
    }

    #[inline]
    fn branch_axis(&self, branch_index: usize) -> Option<usize> {
        self.branch_axes.get(branch_index).copied().flatten()
    }

    #[inline]
    fn axis_matrix_index(&self, axis: usize) -> Option<usize> {
        self.axis_matrix_indices.get(axis).copied().flatten()
    }

    #[inline]
    fn slot_for_axes(&self, row_axis: usize, col_axis: usize) -> Option<CscIndex> {
        let width = self.axis_count();
        if width == 0 || self.slots.len() != width * width {
            return None;
        }
        self.slots
            .get(row_axis.checked_mul(width)?.checked_add(col_axis)?)
            .copied()
            .flatten()
    }

    #[inline]
    fn slot_for_matrix_indices(&self, row: usize, col: usize) -> Option<CscIndex> {
        let row_axis = self.axis_for_matrix_index(row)?;
        let col_axis = self.axis_for_matrix_index(col)?;
        self.slot_for_axes(row_axis, col_axis)
    }

    #[inline]
    fn axis_for_matrix_index(&self, matrix_index: usize) -> Option<usize> {
        self.matrix_axis_lookup
            .binary_search_by_key(&matrix_index, |&(index, _axis)| index)
            .ok()
            .map(|idx| self.matrix_axis_lookup[idx].1)
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn branch_matrix_index(num_nodes: usize, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }
}

enum GeneratedMatrixTarget<'a> {
    Static { matrix: &'a mut StaticMatrix },
    AcReal { matrix: &'a mut ComplexMatrix },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GeneratedDerivative {
    axis: GeneratedDerivativeAxis,
    value: Value,
}

impl GeneratedDerivative {
    #[inline]
    pub const fn node(node: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Node(node),
            value,
        }
    }

    #[inline]
    pub const fn branch(branch_ordinal: usize, value: Value) -> Self {
        Self {
            axis: GeneratedDerivativeAxis::Branch(branch_ordinal),
            value,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneratedDerivativeAxis {
    Node(usize),
    Branch(usize),
}

pub struct GeneratedStamper<'a> {
    matrix: GeneratedMatrixTarget<'a>,
    cache: Option<&'a GeneratedStaticStampCache>,
    rhs: Option<&'a mut [Value]>,
    voltages: &'a [Value],
    num_nodes: usize,
}

impl<'a> GeneratedStamper<'a> {
    #[inline]
    pub fn new(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static { matrix },
            cache: None,
            rhs: Some(rhs),
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn new_with_static_cache(
        matrix: &'a mut StaticMatrix,
        rhs: &'a mut [Value],
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::Static { matrix },
            cache: Some(cache),
            rhs: Some(rhs),
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn new_ac_real(
        matrix: &'a mut ComplexMatrix,
        voltages: &'a [Value],
        num_nodes: usize,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::AcReal { matrix },
            cache: None,
            rhs: None,
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn new_ac_real_with_static_cache(
        matrix: &'a mut ComplexMatrix,
        voltages: &'a [Value],
        num_nodes: usize,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix: GeneratedMatrixTarget::AcReal { matrix },
            cache: Some(cache),
            rhs: None,
            voltages,
            num_nodes,
        }
    }

    #[inline]
    pub fn stamp_current_const(&mut self, pos: Option<usize>, neg: Option<usize>, value: Value) {
        if self.rhs.is_none() {
            return;
        }
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        self.add_current_rhs_pair(pos_row, neg_row, value);
    }

    #[inline]
    pub fn stamp_current_node1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0) - derivative1 * self.node_value(node1)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node3(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value(node0)
                - derivative1 * self.node_value(node1)
                - derivative2 * self.node_value(node2)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if let Some(col) = Self::node_matrix_index(node2) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.branch_value(branch0)
                - derivative1 * self.branch_value(branch1)
        } else {
            0.0
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node1_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value(node0) - derivative1 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value(node0)
                - derivative1 * self.node_value(node1)
                - derivative2 * self.branch_value(branch0)
        } else {
            0.0
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_derivative_pair(pos_row, neg_row, col, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for derivative in derivatives {
            if needs_rhs {
                equivalent -= derivative.value * self.axis_value(derivative.axis);
            }
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative.value);
            }
        }

        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_dense(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.node_value(node);
            }
            if let Some(col) = Self::node_matrix_index(node) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.branch_value(branch);
            }
            if let Some(col) = self.branch_matrix_index(branch) {
                self.add_current_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }

        if needs_rhs {
            self.add_current_rhs_pair(pos_row, neg_row, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_const_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
    ) {
        if self.rhs.is_none() {
            return;
        }
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        self.add_current_rhs_axis_pair(pos_axis, neg_axis, value);
    }

    #[inline]
    pub fn stamp_current_node1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.node_value_local(node0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.node_value_local(node1)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node3_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.node_value_local(node1)
                - derivative2 * self.node_value_local(node2)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if let Some(col_axis) = self.node_axis_local(node2) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value - derivative0 * self.branch_value_local(branch0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_branch2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.branch_value_local(branch0)
                - derivative1 * self.branch_value_local(branch1)
        } else {
            0.0
        };
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node1_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.branch_value_local(branch0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_node2_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }
        let needs_rhs = self.rhs.is_some();
        let equivalent = if needs_rhs {
            value
                - derivative0 * self.node_value_local(node0)
                - derivative1 * self.node_value_local(node1)
                - derivative2 * self.branch_value_local(branch0)
        } else {
            0.0
        };
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative1);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative2);
        }
        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let needs_rhs = self.rhs.is_some();
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative.value * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative.value,
                );
            }

            if needs_rhs {
                self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            }
            return;
        }

        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for derivative in derivatives {
            if needs_rhs {
                equivalent -= derivative.value * self.axis_value_local(derivative.axis);
            }
            if let Some(col_axis) = self.derivative_axis_local(derivative.axis) {
                self.add_current_derivative_axis_pair(
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative.value,
                );
            }
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let needs_rhs = self.rhs.is_some();
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }

            if needs_rhs {
                self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            }
            return;
        }

        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.node_value_local(node);
            }
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.branch_value_local(branch);
            }
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_indexed_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        debug_assert_eq!(node_derivative_indices.len(), node_derivatives.len());
        debug_assert_eq!(branch_derivative_indices.len(), branch_derivatives.len());

        let Some(cache) = self.cache else {
            return;
        };
        let pos_axis = pos.and_then(|node| cache.node_axis(node));
        let neg_axis = neg.and_then(|node| cache.node_axis(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
        }
    }

    #[inline(always)]
    pub fn stamp_current_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivative_indices: [usize; NODE_COUNT],
        node_derivatives: [Value; NODE_COUNT],
        branch_derivative_indices: [usize; BRANCH_COUNT],
        branch_derivatives: [Value; BRANCH_COUNT],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let needs_rhs = self.rhs.is_some();
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivative_indices
                .iter()
                .copied()
                .zip(node_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branch_derivative_indices
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                if needs_rhs {
                    equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                }
                self.add_current_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }

            if needs_rhs {
                self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            }
            return;
        }

        let pos_axis = pos.and_then(|node| self.node_axis_local(node));
        let neg_axis = neg.and_then(|node| self.node_axis_local(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.node_value_local(node);
            }
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if needs_rhs {
                equivalent -= derivative * self.branch_value_local(branch);
            }
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_current_derivative_axis_pair(pos_axis, neg_axis, col_axis, derivative);
            }
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair(pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    pub fn stamp_current_indexed_ad_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        debug_assert!(
            node_derivative_indices
                .iter()
                .all(|&index| index < node_derivatives.len())
        );
        debug_assert!(
            branch_derivative_indices
                .iter()
                .all(|&index| index < branch_derivatives.len())
        );

        let Some(cache) = self.cache else {
            return;
        };
        let pos_axis = pos.and_then(|node| cache.node_axis(node));
        let neg_axis = neg.and_then(|node| cache.node_axis(node));
        if pos_axis.is_none() && neg_axis.is_none() {
            return;
        }

        let needs_rhs = self.rhs.is_some();
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for node in node_derivative_indices.iter().copied() {
            let derivative = derivative_scale * node_derivatives[node];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }
        for branch in branch_derivative_indices.iter().copied() {
            let derivative = derivative_scale * branch_derivatives[branch];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            if needs_rhs {
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            }
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
        }

        if needs_rhs {
            self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
        }
    }

    #[inline]
    fn add_current_derivative_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        col: usize,
        derivative: Value,
    ) {
        if let Some(row) = pos_row {
            self.add_real(row, col, derivative);
        }
        if let Some(row) = neg_row {
            self.add_real(row, col, -derivative);
        }
    }

    #[inline]
    fn add_current_rhs_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        equivalent: Value,
    ) {
        if let Some(rhs) = &mut self.rhs {
            if let Some(row) = pos_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot -= equivalent;
            }
            if let Some(row) = neg_row
                && let Some(slot) = rhs.get_mut(row)
            {
                *slot += equivalent;
            }
        }
    }

    #[inline]
    pub fn stamp_potential_branch(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch_ordinal: usize,
        multiplicity: Value,
    ) {
        let Some(branch) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(node) = pos.filter(|node| *node > 0) {
            self.add_real(node - 1, branch, multiplicity);
            self.add_real(branch, node - 1, 1.0);
        }
        if let Some(node) = neg.filter(|node| *node > 0) {
            self.add_real(node - 1, branch, -multiplicity);
            self.add_real(branch, node - 1, -1.0);
        }
    }

    #[inline]
    pub fn stamp_potential_const(&mut self, branch_ordinal: usize, value: Value) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        self.add_potential_rhs(row, value);
    }

    #[inline]
    pub fn stamp_potential_node1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value - derivative0 * self.node_value(node0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent =
            value - derivative0 * self.node_value(node0) - derivative1 * self.node_value(node1);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value - derivative0 * self.branch_value(branch0);
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative0);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch2(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.branch_value(branch0)
            - derivative1 * self.branch_value(branch1);
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node1_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent =
            value - derivative0 * self.node_value(node0) - derivative1 * self.branch_value(branch0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative1);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2_branch1(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value(node0)
            - derivative1 * self.node_value(node1)
            - derivative2 * self.branch_value(branch0);
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_real(row, col, -derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_real(row, col, -derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_real(row, col, -derivative2);
        }
        self.add_potential_rhs(row, equivalent);
    }

    #[inline]
    pub fn stamp_potential(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let mut equivalent = value;
        for derivative in derivatives {
            equivalent -= derivative.value * self.axis_value(derivative.axis);
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_real(row, col, -derivative.value);
            }
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    pub fn stamp_potential_dense(
        &mut self,
        branch_ordinal: usize,
        value: Value,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        let mut equivalent = value;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.axis_value(GeneratedDerivativeAxis::Node(node));
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Node(node)) {
                self.add_real(row, col, -derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.axis_value(GeneratedDerivativeAxis::Branch(branch));
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Branch(branch)) {
                self.add_real(row, col, -derivative);
            }
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    pub fn stamp_potential_branch_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch_index: usize,
        multiplicity: Value,
    ) {
        let Some(branch_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        if let Some(pos_axis) = pos.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(pos_axis, branch_axis, multiplicity);
            self.add_real_axis(branch_axis, pos_axis, 1.0);
        }
        if let Some(neg_axis) = neg.and_then(|node| self.node_axis_local(node)) {
            self.add_real_axis(neg_axis, branch_axis, -multiplicity);
            self.add_real_axis(branch_axis, neg_axis, -1.0);
        }
    }

    #[inline]
    pub fn stamp_potential_const_local(&mut self, branch_index: usize, value: Value) {
        if let Some(row_axis) = self.branch_axis_local(branch_index) {
            self.add_potential_rhs_axis(row_axis, value);
        }
    }

    #[inline]
    pub fn stamp_potential_node1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value - derivative0 * self.node_value_local(node0);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value_local(node0)
            - derivative1 * self.node_value_local(node1);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value - derivative0 * self.branch_value_local(branch0);
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_branch2_local(
        &mut self,
        branch_index: usize,
        value: Value,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.branch_value_local(branch0)
            - derivative1 * self.branch_value_local(branch1);
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch1) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node1_branch1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value_local(node0)
            - derivative1 * self.branch_value_local(branch0);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_node2_branch1_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let equivalent = value
            - derivative0 * self.node_value_local(node0)
            - derivative1 * self.node_value_local(node1)
            - derivative2 * self.branch_value_local(branch0);
        if let Some(col_axis) = self.node_axis_local(node0) {
            self.add_real_axis(row_axis, col_axis, -derivative0);
        }
        if let Some(col_axis) = self.node_axis_local(node1) {
            self.add_real_axis(row_axis, col_axis, -derivative1);
        }
        if let Some(col_axis) = self.branch_axis_local(branch0) {
            self.add_real_axis(row_axis, col_axis, -derivative2);
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_local(
        &mut self,
        branch_index: usize,
        value: Value,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch_index) else {
                return;
            };
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                equivalent -= derivative.value * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative.value,
                );
            }
            self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
            return;
        }

        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let mut equivalent = value;
        for derivative in derivatives {
            equivalent -= derivative.value * self.axis_value_local(derivative.axis);
        }

        for derivative in derivatives {
            if let Some(col_axis) = self.derivative_axis_local(derivative.axis) {
                self.add_real_axis(row_axis, col_axis, -derivative.value);
            }
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_dense_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch_index) else {
                return;
            };
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
            return;
        }

        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let mut equivalent = value;
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.node_value_local(node);
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.branch_value_local(branch);
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_indexed_dense_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
    ) {
        debug_assert_eq!(node_derivative_indices.len(), node_derivatives.len());
        debug_assert_eq!(branch_derivative_indices.len(), branch_derivatives.len());

        let Some(cache) = self.cache else {
            return;
        };
        let Some(row_axis) = cache.branch_axis(branch_index) else {
            return;
        };
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
    }

    #[inline(always)]
    pub fn stamp_potential_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivative_indices: [usize; NODE_COUNT],
        node_derivatives: [Value; NODE_COUNT],
        branch_derivative_indices: [usize; BRANCH_COUNT],
        branch_derivatives: [Value; BRANCH_COUNT],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch_index) else {
                return;
            };
            let mut equivalent = value;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivative_indices
                .iter()
                .copied()
                .zip(node_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            for (branch, derivative) in branch_derivative_indices
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                equivalent -= derivative * self.axis_value_cached(cache, col_axis);
                self.add_real_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -derivative,
                );
            }
            self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
            return;
        }

        let Some(row_axis) = self.branch_axis_local(branch_index) else {
            return;
        };
        let mut equivalent = value;
        for (node, derivative) in node_derivative_indices
            .iter()
            .copied()
            .zip(node_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.node_value_local(node);
            if let Some(col_axis) = self.node_axis_local(node) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        for (branch, derivative) in branch_derivative_indices
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            equivalent -= derivative * self.branch_value_local(branch);
            if let Some(col_axis) = self.branch_axis_local(branch) {
                self.add_real_axis(row_axis, col_axis, -derivative);
            }
        }
        self.add_potential_rhs_axis(row_axis, equivalent);
    }

    #[inline]
    pub fn stamp_potential_indexed_ad_local(
        &mut self,
        branch_index: usize,
        value: Value,
        node_derivative_indices: &[usize],
        node_derivatives: &[Value],
        branch_derivative_indices: &[usize],
        branch_derivatives: &[Value],
    ) {
        debug_assert!(
            node_derivative_indices
                .iter()
                .all(|&index| index < node_derivatives.len())
        );
        debug_assert!(
            branch_derivative_indices
                .iter()
                .all(|&index| index < branch_derivatives.len())
        );

        let Some(cache) = self.cache else {
            return;
        };
        let Some(row_axis) = cache.branch_axis(branch_index) else {
            return;
        };
        let mut equivalent = value;
        let width = cache.axis_count();
        let slots_ready = width != 0 && cache.slots.len() == width * width;
        for node in node_derivative_indices.iter().copied() {
            let derivative = node_derivatives[node];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.node_axis(node) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        for branch in branch_derivative_indices.iter().copied() {
            let derivative = branch_derivatives[branch];
            if derivative == 0.0 {
                continue;
            }
            let Some(col_axis) = cache.branch_axis(branch) else {
                continue;
            };
            equivalent -= derivative * self.axis_value_cached(cache, col_axis);
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
        self.add_potential_rhs_axis_cached(cache, row_axis, equivalent);
    }

    #[inline]
    fn add_potential_rhs(&mut self, row: usize, equivalent: Value) {
        if equivalent == 0.0 {
            return;
        }
        if let Some(rhs) = &mut self.rhs
            && let Some(slot) = rhs.get_mut(row)
        {
            *slot += equivalent;
        }
    }

    #[inline]
    fn add_potential_rhs_axis(&mut self, row_axis: usize, equivalent: Value) {
        if let Some(row) = self.axis_matrix_index_local(row_axis) {
            self.add_potential_rhs(row, equivalent);
        }
    }

    #[inline]
    fn node_value(&self, node: usize) -> Value {
        if node == 0 {
            0.0
        } else {
            self.voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    #[inline]
    fn node_value_local(&self, node_index: usize) -> Value {
        self.node_axis_local(node_index)
            .and_then(|axis| self.axis_matrix_index_local(axis))
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn branch_value_local(&self, branch_index: usize) -> Value {
        self.branch_axis_local(branch_index)
            .and_then(|axis| self.axis_matrix_index_local(axis))
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn branch_value(&self, branch: usize) -> Value {
        self.branch_matrix_index(branch)
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn axis_value(&self, axis: GeneratedDerivativeAxis) -> Value {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_value(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_value(branch),
        }
    }

    #[inline]
    fn axis_value_local(&self, axis: GeneratedDerivativeAxis) -> Value {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_value_local(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_value_local(branch),
        }
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => Self::node_matrix_index(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index(branch),
        }
    }

    #[inline]
    fn derivative_axis_local(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_axis_local(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_axis_local(branch),
        }
    }

    #[inline]
    fn derivative_axis_cached(
        cache: &GeneratedStaticStampCache,
        axis: GeneratedDerivativeAxis,
    ) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => cache.node_axis(node),
            GeneratedDerivativeAxis::Branch(branch) => cache.branch_axis(branch),
        }
    }

    #[inline]
    fn node_axis_local(&self, node_index: usize) -> Option<usize> {
        self.static_cache()?.node_axis(node_index)
    }

    #[inline]
    fn branch_axis_local(&self, branch_index: usize) -> Option<usize> {
        self.static_cache()?.branch_axis(branch_index)
    }

    #[inline]
    fn axis_matrix_index_local(&self, axis: usize) -> Option<usize> {
        self.static_cache()?.axis_matrix_index(axis)
    }

    #[inline]
    fn static_cache(&self) -> Option<&GeneratedStaticStampCache> {
        self.cache
    }

    #[inline]
    fn axis_value_cached(&self, cache: &GeneratedStaticStampCache, axis: usize) -> Value {
        cache
            .axis_matrix_index(axis)
            .and_then(|index| self.voltages.get(index).copied())
            .unwrap_or(0.0)
    }

    #[inline]
    fn add_current_derivative_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_current_rhs_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        equivalent: Value,
    ) {
        let pos_row = pos_axis.and_then(|axis| cache.axis_matrix_index(axis));
        let neg_row = neg_axis.and_then(|axis| cache.axis_matrix_index(axis));
        self.add_current_rhs_pair(pos_row, neg_row, equivalent);
    }

    #[inline]
    fn add_potential_rhs_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        row_axis: usize,
        equivalent: Value,
    ) {
        if let Some(row) = cache.axis_matrix_index(row_axis) {
            self.add_potential_rhs(row, equivalent);
        }
    }

    #[inline]
    fn add_real_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        row_axis: usize,
        col_axis: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let slot = if slots_ready {
            debug_assert!(row_axis < width);
            debug_assert!(col_axis < width);
            cache
                .slots
                .get(row_axis * width + col_axis)
                .copied()
                .flatten()
        } else {
            None
        };
        if let Some(index) = slot {
            match &mut self.matrix {
                GeneratedMatrixTarget::Static { matrix } => matrix.stamp_direct(index, value),
                GeneratedMatrixTarget::AcReal { matrix } => matrix.stamp_direct_real(index, value),
            }
            return;
        }

        let Some(row) = cache.axis_matrix_index(row_axis) else {
            return;
        };
        let Some(col) = cache.axis_matrix_index(col_axis) else {
            return;
        };
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => matrix.add(row, col, value),
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }

    #[inline]
    fn add_current_derivative_axis_pair(
        &mut self,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(cache) = self.cache {
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            self.add_current_derivative_axis_pair_cached(
                cache,
                slots_ready,
                width,
                pos_axis,
                neg_axis,
                col_axis,
                derivative,
            );
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_real_axis(row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_real_axis(row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_current_rhs_axis_pair(
        &mut self,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        equivalent: Value,
    ) {
        if let Some(cache) = self.cache {
            self.add_current_rhs_axis_pair_cached(cache, pos_axis, neg_axis, equivalent);
            return;
        }
        let pos_row = pos_axis.and_then(|axis| self.axis_matrix_index_local(axis));
        let neg_row = neg_axis.and_then(|axis| self.axis_matrix_index_local(axis));
        self.add_current_rhs_pair(pos_row, neg_row, equivalent);
    }

    #[inline]
    fn add_real_axis(&mut self, row_axis: usize, col_axis: usize, value: Value) {
        if value == 0.0 {
            return;
        }
        if let Some(cache) = self.cache {
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            self.add_real_axis_cached(cache, slots_ready, width, row_axis, col_axis, value);
            return;
        }

        let Some(row) = self.axis_matrix_index_local(row_axis) else {
            return;
        };
        let Some(col) = self.axis_matrix_index_local(col_axis) else {
            return;
        };
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => {
                if let Some(index) = self
                    .cache
                    .and_then(|cache| cache.slot_for_axes(row_axis, col_axis))
                {
                    matrix.stamp_direct(index, value);
                } else {
                    matrix.add(row, col, value);
                }
            }
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }

    #[inline]
    fn add_real(&mut self, row: usize, col: usize, value: Value) {
        if value == 0.0 {
            return;
        }
        match &mut self.matrix {
            GeneratedMatrixTarget::Static { matrix } => {
                if let Some(index) = self
                    .cache
                    .and_then(|cache| cache.slot_for_matrix_indices(row, col))
                {
                    matrix.stamp_direct(index, value);
                } else {
                    matrix.add(row, col, value);
                }
            }
            GeneratedMatrixTarget::AcReal { matrix } => matrix.add_real(row, col, value),
        }
    }
}

pub struct GeneratedReactiveStamper<'a> {
    matrix: &'a mut ComplexMatrix,
    cache: Option<&'a GeneratedStaticStampCache>,
    nodes: &'a [usize],
    branches: &'a [usize],
    num_nodes: usize,
    omega: Value,
}

impl<'a> GeneratedReactiveStamper<'a> {
    #[inline]
    pub fn new(matrix: &'a mut ComplexMatrix, num_nodes: usize, omega: Value) -> Self {
        Self {
            matrix,
            cache: None,
            nodes: &[],
            branches: &[],
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn new_with_static_cache(
        matrix: &'a mut ComplexMatrix,
        num_nodes: usize,
        omega: Value,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix,
            cache: Some(cache),
            nodes: &[],
            branches: &[],
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn new_with_local_maps(
        matrix: &'a mut ComplexMatrix,
        nodes: &'a [usize],
        branches: &'a [usize],
        num_nodes: usize,
        omega: Value,
    ) -> Self {
        Self {
            matrix,
            cache: None,
            nodes,
            branches,
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn new_with_local_maps_and_static_cache(
        matrix: &'a mut ComplexMatrix,
        nodes: &'a [usize],
        branches: &'a [usize],
        num_nodes: usize,
        omega: Value,
        cache: &'a GeneratedStaticStampCache,
    ) -> Self {
        Self {
            matrix,
            cache: Some(cache),
            nodes,
            branches,
            num_nodes,
            omega,
        }
    }

    #[inline]
    pub fn stamp_current_reactive(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            let neg_axis = neg
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = self.derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative.value,
                );
            }
            return;
        }

        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_current_reactive_derivative_pair(
                    pos_row,
                    neg_row,
                    col,
                    self.omega * derivative.value,
                );
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_dense(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            let neg_axis = neg
                .and_then(Self::node_matrix_index)
                .and_then(|index| cache.axis_for_matrix_index(index));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let derivative_scale = self.omega * derivative_scale;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = Self::node_matrix_index(node)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = self
                    .branch_matrix_index(branch)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            return;
        }

        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let derivative_scale = self.omega * derivative_scale;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = Self::node_matrix_index(node) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index(branch) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached_local(cache, derivative.axis)
                else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative.value,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index_local(derivative.axis) {
                self.add_current_reactive_derivative_pair(
                    pos_row,
                    neg_row,
                    col,
                    self.omega * derivative.value,
                );
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let derivative_scale = self.omega * derivative_scale;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let derivative_scale = self.omega * derivative_scale;
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_indexed_dense_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
        derivative_scale: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }

            let derivative_scale = self.omega * derivative_scale;
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                let derivative = derivative_scale * derivative;
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    derivative,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }

        let derivative_scale = self.omega * derivative_scale;
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            let derivative = derivative_scale * derivative;
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_current_reactive_derivative_pair(pos_row, neg_row, col, derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node3(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = Self::node_matrix_index(node2) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch2(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2_branch1(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let pos_row = pos.and_then(Self::node_matrix_index);
        let neg_row = neg.and_then(Self::node_matrix_index);
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            if let Some(col_axis) = cache.node_axis(node0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node3_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        node2: usize,
        derivative2: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            if let Some(col_axis) = cache.node_axis(node2) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative2,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node2) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_branch2_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index_local(branch1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node1_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
    }

    #[inline]
    pub fn stamp_current_reactive_node2_branch1_local(
        &mut self,
        pos: Option<usize>,
        neg: Option<usize>,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        if let Some(cache) = self.cache {
            let pos_axis = pos.and_then(|node| cache.node_axis(node));
            let neg_axis = neg.and_then(|node| cache.node_axis(node));
            if pos_axis.is_none() && neg_axis.is_none() {
                return;
            }
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative1,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_current_reactive_derivative_axis_pair_cached(
                    cache,
                    slots_ready,
                    width,
                    pos_axis,
                    neg_axis,
                    col_axis,
                    self.omega * derivative2,
                );
            }
            return;
        }

        let pos_row = pos.and_then(|node| self.node_matrix_index_local(node));
        let neg_row = neg.and_then(|node| self.node_matrix_index_local(node));
        if pos_row.is_none() && neg_row.is_none() {
            return;
        }
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative0,
            );
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative1,
            );
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_current_reactive_derivative_pair(
                pos_row,
                neg_row,
                col,
                self.omega * derivative2,
            );
        }
    }

    #[inline]
    fn add_current_reactive_derivative_pair(
        &mut self,
        pos_row: Option<usize>,
        neg_row: Option<usize>,
        col: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row) = pos_row {
            self.add_imag(row, col, derivative);
        }
        if let Some(row) = neg_row {
            self.add_imag(row, col, -derivative);
        }
    }

    #[inline]
    fn add_current_reactive_derivative_axis_pair_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        pos_axis: Option<usize>,
        neg_axis: Option<usize>,
        col_axis: usize,
        derivative: Value,
    ) {
        if derivative == 0.0 {
            return;
        }
        if let Some(row_axis) = pos_axis {
            self.add_imag_axis_cached(cache, slots_ready, width, row_axis, col_axis, derivative);
        }
        if let Some(row_axis) = neg_axis {
            self.add_imag_axis_cached(cache, slots_ready, width, row_axis, col_axis, -derivative);
        }
    }

    #[inline]
    fn add_imag_axis_cached(
        &mut self,
        cache: &GeneratedStaticStampCache,
        slots_ready: bool,
        width: usize,
        row_axis: usize,
        col_axis: usize,
        value: Value,
    ) {
        if value == 0.0 {
            return;
        }
        let slot = if slots_ready {
            debug_assert!(row_axis < width);
            debug_assert!(col_axis < width);
            cache
                .slots
                .get(row_axis * width + col_axis)
                .copied()
                .flatten()
        } else {
            None
        };
        if let Some(index) = slot {
            self.matrix.stamp_direct_imag(index, value);
            return;
        }

        let Some(row) = cache.axis_matrix_index(row_axis) else {
            return;
        };
        let Some(col) = cache.axis_matrix_index(col_axis) else {
            return;
        };
        self.matrix.add_imag(row, col, value);
    }

    #[inline]
    fn add_imag(&mut self, row: usize, col: usize, value: Value) {
        if value == 0.0 {
            return;
        }
        if let Some(index) = self
            .cache
            .and_then(|cache| cache.slot_for_matrix_indices(row, col))
        {
            self.matrix.stamp_direct_imag(index, value);
        } else {
            self.matrix.add_imag(row, col, value);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch1(
        &mut self,
        branch_ordinal: usize,
        branch0: usize,
        derivative0: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch2(
        &mut self,
        branch_ordinal: usize,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1_branch1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2_branch1(
        &mut self,
        branch_ordinal: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        if let Some(col) = Self::node_matrix_index(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = Self::node_matrix_index(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
        if let Some(col) = self.branch_matrix_index(branch0) {
            self.add_imag(row, col, -self.omega * derivative2);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive(
        &mut self,
        branch_ordinal: usize,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = self
                .branch_matrix_index(branch_ordinal)
                .and_then(|index| cache.axis_for_matrix_index(index))
            else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = self.derivative_axis_cached(cache, derivative.axis) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative.value,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index(derivative.axis) {
                self.add_imag(row, col, -self.omega * derivative.value);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_dense(
        &mut self,
        branch_ordinal: usize,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = self
                .branch_matrix_index(branch_ordinal)
                .and_then(|index| cache.axis_for_matrix_index(index))
            else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = Self::node_matrix_index(node)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = self
                    .branch_matrix_index(branch)
                    .and_then(|index| cache.axis_for_matrix_index(index))
                else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index(branch_ordinal) else {
            return;
        };
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Node(node)) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.axis_matrix_index(GeneratedDerivativeAxis::Branch(branch)) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_local(
        &mut self,
        branch: usize,
        derivatives: &[GeneratedDerivative],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for derivative in derivatives {
                let Some(col_axis) = Self::derivative_axis_cached_local(cache, derivative.axis)
                else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative.value,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        for derivative in derivatives {
            if let Some(col) = self.axis_matrix_index_local(derivative.axis) {
                self.add_imag(row, col, -self.omega * derivative.value);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_dense_local(
        &mut self,
        branch: usize,
        node_derivatives: &[Value],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in node_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        for (node, derivative) in node_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
        for (branch, derivative) in branch_derivatives.iter().copied().enumerate() {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_indexed_dense_local(
        &mut self,
        branch: usize,
        nodes: &[usize],
        node_derivatives: &[Value],
        branches: &[usize],
        branch_derivatives: &[Value],
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.node_axis(node) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            for (branch, derivative) in branches
                .iter()
                .copied()
                .zip(branch_derivatives.iter().copied())
            {
                if derivative == 0.0 {
                    continue;
                }
                let Some(col_axis) = cache.branch_axis(branch) else {
                    continue;
                };
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        for (node, derivative) in nodes.iter().copied().zip(node_derivatives.iter().copied()) {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.node_matrix_index_local(node) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
        for (branch, derivative) in branches
            .iter()
            .copied()
            .zip(branch_derivatives.iter().copied())
        {
            if derivative == 0.0 {
                continue;
            }
            if let Some(col) = self.branch_matrix_index_local(branch) {
                self.add_imag(row, col, -self.omega * derivative);
            }
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            if let Some(col_axis) = cache.node_axis(node0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch1_local(
        &mut self,
        branch: usize,
        branch0: usize,
        derivative0: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            if let Some(col_axis) = cache.branch_axis(branch0) {
                let width = cache.axis_count();
                let slots_ready = width != 0 && cache.slots.len() == width * width;
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_branch2_local(
        &mut self,
        branch: usize,
        branch0: usize,
        derivative0: Value,
        branch1: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch1) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index_local(branch1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node1_branch1_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
        branch0: usize,
        derivative1: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
    }

    #[inline]
    pub fn stamp_potential_reactive_node2_branch1_local(
        &mut self,
        branch: usize,
        node0: usize,
        derivative0: Value,
        node1: usize,
        derivative1: Value,
        branch0: usize,
        derivative2: Value,
    ) {
        if let Some(cache) = self.cache {
            let Some(row_axis) = cache.branch_axis(branch) else {
                return;
            };
            let width = cache.axis_count();
            let slots_ready = width != 0 && cache.slots.len() == width * width;
            if let Some(col_axis) = cache.node_axis(node0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative0,
                );
            }
            if let Some(col_axis) = cache.node_axis(node1) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative1,
                );
            }
            if let Some(col_axis) = cache.branch_axis(branch0) {
                self.add_imag_axis_cached(
                    cache,
                    slots_ready,
                    width,
                    row_axis,
                    col_axis,
                    -self.omega * derivative2,
                );
            }
            return;
        }

        let Some(row) = self.branch_matrix_index_local(branch) else {
            return;
        };
        if let Some(col) = self.node_matrix_index_local(node0) {
            self.add_imag(row, col, -self.omega * derivative0);
        }
        if let Some(col) = self.node_matrix_index_local(node1) {
            self.add_imag(row, col, -self.omega * derivative1);
        }
        if let Some(col) = self.branch_matrix_index_local(branch0) {
            self.add_imag(row, col, -self.omega * derivative2);
        }
    }

    #[inline]
    fn branch_matrix_index(&self, branch_ordinal: usize) -> Option<usize> {
        if branch_ordinal > 0 {
            Some(self.num_nodes + branch_ordinal - 1)
        } else {
            None
        }
    }

    #[inline]
    fn node_matrix_index(node: usize) -> Option<usize> {
        if node > 0 { Some(node - 1) } else { None }
    }

    #[inline]
    fn node_matrix_index_local(&self, node: usize) -> Option<usize> {
        self.nodes
            .get(node)
            .copied()
            .and_then(Self::node_matrix_index)
    }

    #[inline]
    fn branch_matrix_index_local(&self, branch: usize) -> Option<usize> {
        self.branches
            .get(branch)
            .copied()
            .and_then(|branch| self.branch_matrix_index(branch))
    }

    #[inline]
    fn axis_matrix_index(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => Self::node_matrix_index(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index(branch),
        }
    }

    #[inline]
    fn axis_matrix_index_local(&self, axis: GeneratedDerivativeAxis) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => self.node_matrix_index_local(node),
            GeneratedDerivativeAxis::Branch(branch) => self.branch_matrix_index_local(branch),
        }
    }

    #[inline]
    fn derivative_axis_cached(
        &self,
        cache: &GeneratedStaticStampCache,
        axis: GeneratedDerivativeAxis,
    ) -> Option<usize> {
        self.axis_matrix_index(axis)
            .and_then(|index| cache.axis_for_matrix_index(index))
    }

    #[inline]
    fn derivative_axis_cached_local(
        cache: &GeneratedStaticStampCache,
        axis: GeneratedDerivativeAxis,
    ) -> Option<usize> {
        match axis {
            GeneratedDerivativeAxis::Node(node) => cache.node_axis(node),
            GeneratedDerivativeAxis::Branch(branch) => cache.branch_axis(branch),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GeneratedAnalysisKind, GeneratedEvalContext, GeneratedEvaluationMode,
        GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseInjection,
        GeneratedNoiseKind, GeneratedNoiseTopologyError, GeneratedSimulationParameters,
    };

    fn noise_descriptor(
        is_current: bool,
        branch_ordinal: Option<usize>,
    ) -> GeneratedNoiseDescriptor {
        GeneratedNoiseDescriptor {
            mechanism: "WHITE_P_N_SOURCE",
            label: Some("source"),
            kind: GeneratedNoiseKind::White,
            equation: 0,
            is_current,
            branch_ordinal,
            pos: GeneratedNoiseEndpoint {
                local_node: Some(1),
                name: "p",
                is_internal: false,
            },
            neg: GeneratedNoiseEndpoint {
                local_node: None,
                name: "GND",
                is_internal: false,
            },
            table_len: 0,
            table_log_interp: false,
        }
    }

    #[test]
    fn generated_analysis_predicates_match_runtime_contract() {
        let voltages = [0.0];
        let dc = GeneratedEvalContext::with_analysis_step(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Dc,
            true,
            true,
        );
        assert!(dc.analysis_dc());
        assert!(dc.analysis_static());
        assert!(!dc.analysis_smallsig());
        assert!(dc.analysis_initial_step());
        assert!(dc.analysis_final_step());
        assert!(dc.limiting_enabled());

        let ac =
            GeneratedEvalContext::with_analysis(&voltages, 300.15, 1, GeneratedAnalysisKind::Ac);
        assert!(!ac.analysis_static());
        assert!(ac.analysis_smallsig());
        assert!(!ac.analysis_initial_step());
        assert!(!ac.analysis_final_step());
        assert!(!ac.limiting_enabled());

        let dc_probe = GeneratedEvalContext::with_analysis_step_simparams_and_mode(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Dc,
            false,
            false,
            GeneratedSimulationParameters::default(),
            GeneratedEvaluationMode::StaticProbe,
        );
        assert!(dc_probe.analysis_dc());
        assert!(!dc_probe.limiting_enabled());
    }

    #[test]
    fn generated_simparams_distinguish_overrides_from_fallbacks() {
        let voltages = [0.0];
        let mut simparams = GeneratedSimulationParameters::default();
        simparams.set_gmin(0.0);
        let ctx = GeneratedEvalContext::with_analysis_step_and_simparams(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Dc,
            false,
            false,
            simparams,
        );

        assert!(ctx.has_simparam("GMIN"));
        assert_eq!(ctx.simparam_or("gmin", 9.0), 0.0);
        assert!(!ctx.has_simparam("pnjmaxi"));
        assert_eq!(ctx.simparam_or("PNJMAXI", 1.0), 1.0);
        assert_eq!(ctx.simparam_or("unknown", 7.0), 7.0);

        simparams.set_pnjmaxi(Some(2.5));
        let ctx = GeneratedEvalContext::with_analysis_step_and_simparams(
            &voltages,
            300.15,
            1,
            GeneratedAnalysisKind::Dc,
            false,
            false,
            simparams,
        );
        assert_eq!(ctx.simparam_or("pnjmaxi", 1.0), 2.5);
    }

    #[test]
    fn generated_noise_topology_maps_nodes_and_ground() {
        let mapped = noise_descriptor(true, None)
            .map_topology(&[41, 73], &[])
            .expect("map current noise topology");

        assert_eq!(
            mapped.injection,
            GeneratedNoiseInjection::Current {
                node_pos: 73,
                node_neg: 0,
            }
        );
    }

    #[test]
    fn generated_potential_noise_preserves_branch_ordinal_semantics() {
        let mapped = noise_descriptor(false, Some(1))
            .map_topology(&[41, 73], &[3, 9])
            .expect("map potential noise topology");

        assert_eq!(
            mapped.injection,
            GeneratedNoiseInjection::Potential { branch: 9 }
        );
    }

    #[test]
    fn generated_noise_topology_rejects_invalid_local_indices() {
        let error = noise_descriptor(true, None)
            .map_topology(&[41], &[])
            .expect_err("invalid local endpoint must fail");

        assert_eq!(
            error,
            GeneratedNoiseTopologyError::LocalNodeOutOfRange {
                endpoint: "positive",
                local_node: 1,
                node_count: 1,
            }
        );

        let error = noise_descriptor(false, Some(0))
            .map_topology(&[41], &[3])
            .expect_err("potential source metadata must validate endpoints");
        assert_eq!(
            error,
            GeneratedNoiseTopologyError::LocalNodeOutOfRange {
                endpoint: "positive",
                local_node: 1,
                node_count: 1,
            }
        );
    }

    #[test]
    fn generated_noise_topology_rejects_invalid_branch_contracts() {
        assert_eq!(
            noise_descriptor(false, None)
                .map_topology(&[41, 73], &[3])
                .expect_err("potential source without branch must fail"),
            GeneratedNoiseTopologyError::PotentialSourceMissingBranch
        );
        assert_eq!(
            noise_descriptor(false, Some(2))
                .map_topology(&[41, 73], &[3])
                .expect_err("invalid branch ordinal must fail"),
            GeneratedNoiseTopologyError::BranchOrdinalOutOfRange {
                branch_ordinal: 2,
                branch_count: 1,
            }
        );
        assert_eq!(
            noise_descriptor(true, Some(0))
                .map_topology(&[41, 73], &[3])
                .expect_err("current source branch must fail"),
            GeneratedNoiseTopologyError::CurrentSourceHasBranch { branch_ordinal: 0 }
        );
    }
}

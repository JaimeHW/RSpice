//! Engine-side adapter for the precompiled Verilog-A model catalog.
//!
//! Nothing here is generated. The generated devices are 43 independent crates
//! under `rspice-veriloga-models`, each depending only on the engine-neutral
//! `rspice-veriloga-runtime` ABI; this module is what teaches `rspice-core` to
//! drive them. It resolves netlist parameters into an instance, owns the
//! per-instance state the engine checkpoints and rolls back, and stamps the
//! result into the solver's matrices.
//!
//! That direction is the reason the adapter lives here and not in a Verilog-A
//! crate: it speaks `crate::netlist`, `crate::solver` and `crate::CircuitData`.
//! Moving it across would make a generated crate depend on `rspice-core`,
//! closing the cycle `rspice-core -> rspice-veriloga-models -> rspice-core`.
//!
//! What it must *not* name is `crate::engine`. `device` is ranked below it, so
//! instantiation failures leave here as `BuiltinInstantiationError` and the
//! engine converts at the boundary. (Not an intra-doc link: the type is gated
//! behind `veriloga-builtins-base` and this module doc is not.)

#[cfg(feature = "veriloga-builtins-base")]
use crate::solver::{ComplexMatrix, StaticMatrix};
#[cfg(feature = "veriloga-builtins-base")]
use std::sync::Arc;

// The registry compiles inside the catalog crate rather than being pulled
// through the crate boundary by `include!`. The old form hard-coded a sibling
// directory layout and made rspice-core's build depend on another crate's
// source path; the catalog's own feature set already gates the same models,
// because every `veriloga-model-*` here forwards to it.
#[cfg(feature = "veriloga-builtins-base")]
pub use rspice_veriloga_models::registry as builtins;

pub use rspice_veriloga_runtime::{
    GENERATED_PERSISTENT_STATE_VERSION, GENERATED_VERILOGA_DESCRIPTOR_ABI_VERSION,
    GeneratedAnalysisKind, GeneratedDdtCoefficients, GeneratedDerivative, GeneratedEvalContext,
    GeneratedEvaluationError, GeneratedEvaluationMode, GeneratedMappedNoiseDescriptor,
    GeneratedNoiseDescriptor, GeneratedNoiseEndpoint, GeneratedNoiseEvaluation,
    GeneratedNoiseEvaluationError, GeneratedNoiseEvaluationRef, GeneratedNoiseInjection,
    GeneratedNoiseKind, GeneratedNoiseTopologyError, GeneratedReactiveStamper,
    GeneratedSimulationParameters, GeneratedStampLane, GeneratedStamper, GeneratedStaticStampCache,
    GeneratedVerilogAEvaluationError, GeneratedVerilogAInstanceCheckpoint,
    GeneratedVerilogAModelDescriptor, GeneratedVerilogAParameterBound,
    GeneratedVerilogAParameterDescriptor, GeneratedVerilogAParameterScope,
    GeneratedVerilogAPersistentState, GeneratedVerilogARollbackState,
    GeneratedVerilogATerminalDescriptor, GeneratedVerilogATerminalDirection, Value,
};
#[cfg(feature = "veriloga-builtins-base")]
use rspice_veriloga_runtime::{GeneratedParameterAssignment, GeneratedParameterOrigin};

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone)]
pub(crate) struct BuiltinParameterAssignment {
    pub(crate) name: String,
    pub(crate) value: crate::netlist::ParametricValue,
    pub(crate) origin: GeneratedParameterOrigin,
}

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GeneratedTerminalCurrentAlias {
    /// Conventional SPICE lead-current parameter (for example `id`).
    pub(crate) parameter: &'static str,
    /// Canonical Verilog-A terminal that owns the aliased current.
    pub(crate) terminal: &'static str,
}

#[cfg(feature = "veriloga-builtins-base")]
impl BuiltinParameterAssignment {
    #[inline]
    pub(crate) fn new(
        name: impl Into<String>,
        value: crate::netlist::ParametricValue,
        origin: GeneratedParameterOrigin,
    ) -> Self {
        Self {
            name: name.into(),
            value,
            origin,
        }
    }

    #[inline]
    pub(crate) fn model_card(
        name: impl Into<String>,
        value: crate::netlist::ParametricValue,
    ) -> Self {
        Self::new(name, value, GeneratedParameterOrigin::ModelCard)
    }

    #[inline]
    pub(crate) fn instance(
        name: impl Into<String>,
        value: crate::netlist::ParametricValue,
    ) -> Self {
        Self::new(name, value, GeneratedParameterOrigin::Instance)
    }
}

/// Exact catalog contracts compiled into this build.
///
/// An empty slice means no generated-model feature was selected; callers must
/// not present models that the running engine cannot instantiate.
#[cfg(feature = "veriloga-builtins-base")]
pub fn generated_veriloga_model_descriptors() -> &'static [GeneratedVerilogAModelDescriptor] {
    builtins::builtin_descriptors()
}

#[cfg(not(feature = "veriloga-builtins-base"))]
pub fn generated_veriloga_model_descriptors() -> &'static [GeneratedVerilogAModelDescriptor] {
    &[]
}

#[cfg(feature = "veriloga-builtins-base")]
pub fn generated_veriloga_model_descriptor(
    model_name: &str,
) -> Option<&'static GeneratedVerilogAModelDescriptor> {
    builtins::descriptor(model_name)
}

#[cfg(not(feature = "veriloga-builtins-base"))]
pub fn generated_veriloga_model_descriptor(
    _model_name: &str,
) -> Option<&'static GeneratedVerilogAModelDescriptor> {
    None
}

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Clone)]
pub struct BuiltinVerilogAInstance {
    pub model_name: &'static str,
    pub instance_name: String,
    pub nodes: Vec<usize>,
    pub branches: Vec<usize>,
    temperature: Value,
    analysis_initial_step: bool,
    analysis_final_step: bool,
    static_stamp_cache: Arc<GeneratedStaticStampCache>,
    /// Exact current entering each external terminal during the most recent
    /// static evaluation, in canonical module-port order.
    terminal_currents: Vec<Value>,
    /// Canonical external terminal metadata, in the same module-port order.
    external_terminals: &'static [GeneratedVerilogATerminalDescriptor],
    /// Card-semantics aliases installed only by a compatible SPICE route.
    terminal_current_aliases: &'static [GeneratedTerminalCurrentAlias],
    kind: builtins::GeneratedBuiltinKind,
}

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone, PartialEq)]
/// One noise source a generated device contributes at a bias point.
pub struct BuiltinEvaluatedNoiseSource {
    pub mapped: GeneratedMappedNoiseDescriptor,
    pub evaluation: GeneratedNoiseEvaluation,
}

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug)]
/// Why a generated device could not evaluate its noise sources.
pub enum BuiltinNoiseEvaluationError {
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

#[cfg(feature = "veriloga-builtins-base")]
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

#[cfg(feature = "veriloga-builtins-base")]
impl std::error::Error for BuiltinNoiseEvaluationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Topology { source, .. } => Some(source),
            Self::Evaluation { source, .. } => Some(source),
        }
    }
}

#[cfg(feature = "veriloga-builtins-base")]
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

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone, Default)]
pub(crate) struct BuiltinVerilogADevices {
    devices: Vec<BuiltinVerilogAInstance>,
}

#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct BuiltinVerilogADevicesRollback {
    states: Vec<GeneratedVerilogARollbackState>,
    terminal_currents: Vec<Value>,
}

#[cfg(feature = "veriloga-builtins-base")]
impl BuiltinVerilogADevices {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    #[inline]
    pub(crate) fn add(&mut self, device: BuiltinVerilogAInstance) {
        self.devices.push(device);
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    #[inline]
    pub(crate) fn len(&self) -> usize {
        self.devices.len()
    }

    /// Whether every instantiated generated model admits Xyce OneStep's
    /// order-two split between dynamic charge and static residual history.
    #[inline]
    pub(crate) fn all_one_step_dae_split_safe(&self) -> bool {
        self.devices
            .iter()
            .all(|device| device.kind.one_step_dae_split_safe())
    }

    #[inline]
    pub(crate) fn iter(&self) -> impl Iterator<Item = &BuiltinVerilogAInstance> {
        self.devices.iter()
    }

    pub(crate) fn checkpoint_states(&self) -> Vec<GeneratedVerilogAInstanceCheckpoint> {
        self.devices
            .iter()
            .map(BuiltinVerilogAInstance::checkpoint_state)
            .collect()
    }

    pub(crate) fn validate_checkpoint_states(
        &self,
        states: &[GeneratedVerilogAInstanceCheckpoint],
    ) -> Result<(), String> {
        if states.len() != self.devices.len() {
            return Err(format!(
                "generated Verilog-A checkpoint instance count mismatch: captured {}, circuit has {}",
                states.len(),
                self.devices.len()
            ));
        }
        for (index, (device, state)) in self.devices.iter().zip(states).enumerate() {
            device.validate_checkpoint_state(state).map_err(|error| {
                format!("generated Verilog-A checkpoint instance {index}: {error}")
            })?;
        }
        Ok(())
    }

    pub(crate) fn restore_checkpoint_states(
        &mut self,
        states: &[GeneratedVerilogAInstanceCheckpoint],
    ) -> Result<(), String> {
        self.validate_checkpoint_states(states)?;
        let rollback = self.capture_rollback_state();
        for (index, (device, state)) in self.devices.iter_mut().zip(states).enumerate() {
            if let Err(error) = device.restore_checkpoint_state(state) {
                self.restore_rollback_state(rollback);
                return Err(format!(
                    "generated Verilog-A checkpoint instance {index} restore failed: {error}"
                ));
            }
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn capture_rollback_state(&self) -> BuiltinVerilogADevicesRollback {
        BuiltinVerilogADevicesRollback {
            states: self
                .devices
                .iter()
                .map(|device| device.kind.capture_rollback_state())
                .collect(),
            terminal_currents: self
                .devices
                .iter()
                .flat_map(|device| device.terminal_currents.iter().copied())
                .collect(),
        }
    }

    /// Refresh an existing rollback image while retaining its outer storage.
    ///
    /// Transient trial probes capture this state repeatedly after circuit
    /// topology has become immutable. Reusing the per-instance vector avoids
    /// one topology-sized allocation on every rejected Newton point or
    /// timestep while preserving the generated model's exact state packing.
    #[inline]
    pub(crate) fn capture_rollback_state_into(
        &self,
        rollback: &mut BuiltinVerilogADevicesRollback,
    ) {
        let expected_terminal_currents = self
            .devices
            .iter()
            .map(|device| device.terminal_currents.len())
            .sum::<usize>();
        if rollback.states.len() != self.devices.len()
            || rollback.terminal_currents.len() != expected_terminal_currents
        {
            *rollback = self.capture_rollback_state();
            return;
        }

        for (state, device) in rollback.states.iter_mut().zip(&self.devices) {
            let captured = device.kind.capture_rollback_state();
            state.values.clone_from(&captured.values);
            state.flags.clone_from(&captured.flags);
        }
        let mut offset = 0;
        for device in &self.devices {
            let end = offset + device.terminal_currents.len();
            rollback.terminal_currents[offset..end].copy_from_slice(&device.terminal_currents);
            offset = end;
        }
        debug_assert_eq!(offset, rollback.terminal_currents.len());
    }

    #[inline]
    pub(crate) fn restore_rollback_state(&mut self, rollback: BuiltinVerilogADevicesRollback) {
        debug_assert_eq!(self.devices.len(), rollback.states.len());
        let expected_terminal_currents: usize = self
            .devices
            .iter()
            .map(|device| device.terminal_currents.len())
            .sum();
        debug_assert_eq!(expected_terminal_currents, rollback.terminal_currents.len());
        let mut terminal_currents = rollback.terminal_currents.as_slice();
        for (device, state) in self.devices.iter_mut().zip(&rollback.states) {
            device.kind.restore_rollback_state(state);
            let (restored, remaining) = terminal_currents.split_at(device.terminal_currents.len());
            device.terminal_currents.copy_from_slice(restored);
            terminal_currents = remaining;
        }
        debug_assert!(terminal_currents.is_empty());
    }

    pub(crate) fn link_static_stamps(&mut self, matrix: &StaticMatrix, num_nodes: usize) {
        for device in &mut self.devices {
            device.link_static_stamps(matrix, num_nodes);
        }
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
    ) -> Result<(), GeneratedVerilogAEvaluationError> {
        for device in &mut self.devices {
            device
                .stamp_with_mode(
                    matrix,
                    rhs,
                    voltages,
                    num_nodes,
                    analysis,
                    simparams,
                    evaluation_mode,
                )
                .map_err(|source| GeneratedVerilogAEvaluationError {
                    instance_name: device.instance_name.clone(),
                    model_name: device.model_name,
                    source,
                })?;
        }
        Ok(())
    }

    #[inline]
    pub(crate) fn all_converged(&self) -> bool {
        self.devices
            .iter()
            .all(BuiltinVerilogAInstance::is_converged)
    }

    #[inline]
    pub(crate) fn set_timepoint(
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
    pub(crate) fn set_analysis_step(&mut self, initial: bool, final_step: bool) {
        for device in &mut self.devices {
            device.set_analysis_step(initial, final_step);
        }
    }

    #[inline]
    pub(crate) fn accept_timestep(&mut self) {
        for device in &mut self.devices {
            device.accept_timestep();
        }
    }

    pub(crate) fn stamp_ac_real_all(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        simparams: GeneratedSimulationParameters,
    ) -> Result<(), GeneratedVerilogAEvaluationError> {
        for device in &mut self.devices {
            device
                .stamp_ac_real(matrix, voltages, num_nodes, simparams)
                .map_err(|source| GeneratedVerilogAEvaluationError {
                    instance_name: device.instance_name.clone(),
                    model_name: device.model_name,
                    source,
                })?;
        }
        Ok(())
    }

    pub(crate) fn stamp_reactive_all(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        omega: Value,
        simparams: GeneratedSimulationParameters,
    ) -> Result<(), GeneratedVerilogAEvaluationError> {
        for device in &mut self.devices {
            device
                .stamp_reactive(matrix, voltages, num_nodes, omega, simparams)
                .map_err(|source| GeneratedVerilogAEvaluationError {
                    instance_name: device.instance_name.clone(),
                    model_name: device.model_name,
                    source,
                })?;
        }
        Ok(())
    }
}

#[cfg(feature = "veriloga-builtins-base")]
impl BuiltinVerilogAInstance {
    fn checkpoint_state(&self) -> GeneratedVerilogAInstanceCheckpoint {
        GeneratedVerilogAInstanceCheckpoint {
            instance_name: self.instance_name.clone(),
            model_name: self.model_name.to_string(),
            model_identity: self.kind.checkpoint_model_identity().to_string(),
            state_version: GENERATED_PERSISTENT_STATE_VERSION,
            state: self.kind.capture_persistent_state(),
        }
    }

    fn validate_checkpoint_state(
        &self,
        checkpoint: &GeneratedVerilogAInstanceCheckpoint,
    ) -> Result<(), String> {
        if checkpoint.instance_name != self.instance_name {
            return Err(format!(
                "instance name mismatch: captured '{}', circuit has '{}'",
                checkpoint.instance_name, self.instance_name
            ));
        }
        if checkpoint.model_name != self.model_name {
            return Err(format!(
                "model name mismatch for '{}': captured '{}', circuit has '{}'",
                self.instance_name, checkpoint.model_name, self.model_name
            ));
        }
        let model_identity = self.kind.checkpoint_model_identity();
        if checkpoint.model_identity != model_identity {
            return Err(format!(
                "generated model identity mismatch for '{}' ({}): captured '{}', circuit has '{}'",
                self.instance_name, self.model_name, checkpoint.model_identity, model_identity
            ));
        }
        if checkpoint.state_version != GENERATED_PERSISTENT_STATE_VERSION {
            return Err(format!(
                "persistent-state version mismatch for '{}': captured {}, runtime requires {}",
                self.instance_name, checkpoint.state_version, GENERATED_PERSISTENT_STATE_VERSION
            ));
        }
        self.kind.validate_persistent_state_shape(&checkpoint.state)
    }

    fn restore_checkpoint_state(
        &mut self,
        checkpoint: &GeneratedVerilogAInstanceCheckpoint,
    ) -> Result<(), String> {
        self.validate_checkpoint_state(checkpoint)?;
        self.kind.restore_persistent_state(&checkpoint.state)
    }

    /// Instantiate a built-in outside a netlist, against explicit unknown indices.
    ///
    /// [`instantiate_builtin`] is the netlist path and needs a `CircuitData` and
    /// a `ParamContext` to resolve terminals and parameter expressions. Harnesses
    /// that drive one device directly — throughput benchmarks, the numerical
    /// oracle, model-authoring tools — already know their node and branch rows
    /// and have plain numeric overrides, so they take this instead of
    /// assembling a circuit to borrow from.
    ///
    /// `nodes` and `branches` are matrix row indices, in the generated device's
    /// own order; row 0 is ground by the usual convention.
    pub fn standalone(
        model_name: &str,
        instance_name: impl Into<String>,
        nodes: Vec<usize>,
        branches: Vec<usize>,
        temperature: Value,
        overrides: &[(String, Value)],
    ) -> Result<Self, String> {
        let model_name = builtins::builtin_names()
            .iter()
            .find(|name| name.eq_ignore_ascii_case(model_name))
            .copied()
            .ok_or_else(|| format!("'{model_name}' is not a compiled-in generated built-in"))?;
        let descriptor = builtins::descriptor(model_name)
            .ok_or_else(|| format!("'{model_name}' has no canonical model descriptor"))?;
        let external_terminals = descriptor.terminals;
        let total_nodes = descriptor.total_node_count;
        if total_nodes != external_terminals.len() + descriptor.internal_node_names.len() {
            return Err(format!(
                "'{model_name}' canonical node metadata declares {total_nodes} total nodes but has {} external and {} internal nodes",
                external_terminals.len(),
                descriptor.internal_node_names.len()
            ));
        }
        if total_nodes != nodes.len() {
            return Err(format!(
                "'{model_name}' canonical node metadata has {total_nodes} total nodes, instance has {}",
                nodes.len()
            ));
        }
        if descriptor.branch_count != branches.len() {
            return Err(format!(
                "'{model_name}' canonical branch metadata has {} branches, instance has {}",
                descriptor.branch_count,
                branches.len()
            ));
        }
        let kind = builtins::instantiate(model_name, &nodes, &branches, overrides)?
            .ok_or_else(|| format!("'{model_name}' is not compiled into this binary"))?;
        Ok(Self {
            model_name,
            instance_name: instance_name.into(),
            nodes,
            branches,
            temperature,
            analysis_initial_step: false,
            analysis_final_step: false,
            static_stamp_cache: Arc::new(GeneratedStaticStampCache::default()),
            terminal_currents: vec![0.0; external_terminals.len()],
            external_terminals,
            terminal_current_aliases: &[],
            kind,
        })
    }

    /// Noise mechanisms this model declares, in evaluation order.
    #[inline]
    pub fn noise_descriptors(&self) -> &'static [GeneratedNoiseDescriptor] {
        self.kind.noise_descriptors()
    }

    /// CSC locations this instance writes through, after [`Self::link_static_stamps`].
    #[inline]
    pub fn linked_slot_count(&self) -> usize {
        self.static_stamp_cache.linked_slot_count()
    }

    /// Exact currents entering the external terminals, in canonical module
    /// port order, from the most recent static device evaluation.
    #[inline]
    pub(crate) fn terminal_currents(&self) -> &[Value] {
        &self.terminal_currents
    }

    /// Canonical external terminals in Verilog-A module declaration order.
    #[inline]
    pub(crate) fn external_terminals(&self) -> &'static [GeneratedVerilogATerminalDescriptor] {
        self.external_terminals
    }

    /// Conventional SPICE lead aliases admitted by the card route that
    /// created this instance. Direct X-device instantiation has no aliases.
    #[inline]
    pub(crate) fn terminal_current_aliases(&self) -> &'static [GeneratedTerminalCurrentAlias] {
        self.terminal_current_aliases
    }

    pub(crate) fn set_terminal_current_aliases(
        &mut self,
        aliases: &'static [GeneratedTerminalCurrentAlias],
    ) -> Result<(), String> {
        for alias in aliases {
            if !self
                .external_terminals
                .iter()
                .any(|terminal| terminal.name.eq_ignore_ascii_case(alias.terminal))
            {
                return Err(format!(
                    "generated Verilog-A model '{}' cannot map conventional current '{}' to absent terminal '{}'",
                    self.model_name, alias.parameter, alias.terminal
                ));
            }
        }
        self.terminal_current_aliases = aliases;
        Ok(())
    }

    /// Canonical generated internal-node names paired with their circuit node
    /// IDs. External terminals occupy the prefix of `nodes`; the registry's
    /// internal names describe the remaining entries in the same order.
    pub(crate) fn internal_nodes(&self) -> impl Iterator<Item = (&'static str, usize)> + '_ {
        let external_count = self.terminal_currents.len();
        builtins::descriptor(self.model_name)
            .map_or(&[][..], |descriptor| descriptor.internal_node_names)
            .iter()
            .copied()
            .zip(self.nodes.iter().copied().skip(external_count))
    }

    #[inline]
    pub(crate) fn is_converged(&self) -> bool {
        self.kind.limiter_converged()
    }

    #[inline]
    pub fn link_static_stamps(&mut self, matrix: &StaticMatrix, num_nodes: usize) {
        Arc::make_mut(&mut self.static_stamp_cache).link(
            matrix,
            &self.nodes,
            &self.branches,
            num_nodes,
        );
    }

    /// Evaluate this instance's noise sources at a bias point.
    ///
    /// Public for the same reason as the stamp entry points beside it: the
    /// noise power spectral densities are part of what a generated device
    /// computes, so anything auditing a model — a fingerprint capture, a
    /// noise-contribution report — has to be able to observe them without
    /// running an analysis around them.
    pub fn evaluate_noise_sources(
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
        let descriptors = self.kind.noise_descriptors();
        let mut evaluated = Vec::with_capacity(descriptors.len());
        let mut visitor_error = None;
        self.kind
            .evaluate_noise_sources(
                &ctx,
                &mut |index, evaluation: GeneratedNoiseEvaluationRef<'_>| {
                    let Some(descriptor) = descriptors.get(index).copied() else {
                        visitor_error = Some(BuiltinNoiseEvaluationError::Evaluation {
                            index,
                            mechanism: "<missing descriptor>",
                            source: GeneratedNoiseEvaluationError::SourceIndexOutOfRange {
                                index,
                                count: descriptors.len(),
                            },
                        });
                        return false;
                    };
                    let mapped = match descriptor.map_topology(&self.nodes, &self.branches) {
                        Ok(mapped) => mapped,
                        Err(source) => {
                            visitor_error = Some(BuiltinNoiseEvaluationError::Topology {
                                index,
                                mechanism: descriptor.mechanism,
                                source,
                            });
                            return false;
                        }
                    };
                    evaluated.push(BuiltinEvaluatedNoiseSource {
                        mapped,
                        evaluation: evaluation.to_owned(),
                    });
                    true
                },
            )
            .map_err(|source| {
                let (index, mechanism) = match &source {
                    GeneratedNoiseEvaluationError::SourceIndexOutOfRange { index, .. }
                    | GeneratedNoiseEvaluationError::NonFinite { index, .. }
                    | GeneratedNoiseEvaluationError::NegativePower { index, .. } => (
                        *index,
                        descriptors
                            .get(*index)
                            .map_or("<missing descriptor>", |descriptor| descriptor.mechanism),
                    ),
                    GeneratedNoiseEvaluationError::InvalidMultiplicity { .. } => {
                        (0, "<device multiplicity>")
                    }
                    GeneratedNoiseEvaluationError::AnalogLoopLimit { .. } => (0, "<analog loop>"),
                };
                BuiltinNoiseEvaluationError::Evaluation {
                    index,
                    mechanism,
                    source,
                }
            })?;
        // Analog-loop limits are recorded on the context rather than returned,
        // because generated noise schedules are split into helpers that cannot
        // propagate a `Result`. A limit means the operating point the PSDs were
        // evaluated at never settled, so it outranks any per-source problem the
        // visitor saw.
        if let Some(GeneratedEvaluationError::AnalogLoopLimit {
            iterations, limit, ..
        }) = ctx.take_evaluation_error()
        {
            return Err(BuiltinNoiseEvaluationError::Evaluation {
                index: 0,
                mechanism: "<analog loop>",
                source: GeneratedNoiseEvaluationError::AnalogLoopLimit { iterations, limit },
            });
        }
        if let Some(error) = visitor_error {
            return Err(error);
        }
        debug_assert_eq!(evaluated.len(), descriptors.len());
        Ok(evaluated)
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
    ) -> Result<(), GeneratedEvaluationError> {
        self.stamp_with_mode(
            matrix,
            rhs,
            voltages,
            num_nodes,
            analysis,
            simparams,
            GeneratedEvaluationMode::default_for_analysis(analysis),
        )
    }

    /// Stamp with Newton limiting disabled.
    ///
    /// A limited stamp is a function of both the bias and the previous iterate,
    /// which is correct inside Newton and useless to anything that needs the
    /// device's own constitutive relation: a numerical derivative of a limited
    /// stamp differentiates the limiter. Callers that must observe the model
    /// itself — the derivative oracle in `rspice-conformance`'s Verilog-A
    /// suite, small-signal probes — take this entry point instead.
    #[inline]
    pub fn stamp_probe(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        num_nodes: usize,
        analysis: GeneratedAnalysisKind,
        simparams: GeneratedSimulationParameters,
    ) -> Result<(), GeneratedEvaluationError> {
        self.stamp_with_mode(
            matrix,
            rhs,
            voltages,
            num_nodes,
            analysis,
            simparams,
            GeneratedEvaluationMode::StaticProbe,
        )
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
    ) -> Result<(), GeneratedEvaluationError> {
        if !self
            .static_stamp_cache
            .axis_indices_match(&self.nodes, &self.branches, num_nodes)
        {
            let cache = Arc::make_mut(&mut self.static_stamp_cache);
            cache.rebuild_axis_indices(&self.nodes, &self.branches, num_nodes);
            cache.clear_linked_slots();
        }
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
        if evaluation_mode == GeneratedEvaluationMode::StaticDaeProbe {
            // OneStep history capture evaluates only F(x)-B(t), with dynamic
            // operators intentionally suppressed. It is not a physical
            // device operating point and must not replace the complete lead
            // currents retained by the preceding accepted transient stamp.
            let mut stamper = GeneratedStamper::new_with_static_cache(
                matrix,
                rhs,
                voltages,
                num_nodes,
                self.static_stamp_cache.as_ref(),
            );
            self.kind.stamp(&ctx, &mut stamper);
        } else {
            self.terminal_currents.fill(0.0);
            let mut stamper = GeneratedStamper::new_with_static_cache_and_terminal_currents(
                matrix,
                rhs,
                voltages,
                num_nodes,
                self.static_stamp_cache.as_ref(),
                &mut self.terminal_currents,
            );
            self.kind.stamp(&ctx, &mut stamper);
        }
        match ctx.take_evaluation_error() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    #[inline]
    pub(crate) fn set_temperature(&mut self, temperature: Value) {
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
    pub(crate) fn set_analysis_step(&mut self, initial: bool, final_step: bool) {
        self.analysis_initial_step = initial;
        self.analysis_final_step = final_step;
    }

    #[inline]
    pub(crate) fn accept_timestep(&mut self) {
        self.kind.accept_timestep();
    }

    #[inline]
    pub(crate) fn stamp_ac_real(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        simparams: GeneratedSimulationParameters,
    ) -> Result<(), GeneratedEvaluationError> {
        let ctx = GeneratedEvalContext::with_analysis_step_and_simparams(
            voltages,
            self.temperature,
            num_nodes,
            GeneratedAnalysisKind::Ac,
            self.analysis_initial_step,
            self.analysis_final_step,
            simparams,
        );
        if !self
            .static_stamp_cache
            .axis_indices_match(&self.nodes, &self.branches, num_nodes)
        {
            let cache = Arc::make_mut(&mut self.static_stamp_cache);
            cache.rebuild_axis_indices(&self.nodes, &self.branches, num_nodes);
            cache.clear_linked_slots();
        }
        let mut stamper = GeneratedStamper::new_ac_real_with_static_cache(
            matrix,
            voltages,
            num_nodes,
            self.static_stamp_cache.as_ref(),
        );
        self.kind.stamp(&ctx, &mut stamper);
        match ctx.take_evaluation_error() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }

    #[inline]
    pub fn stamp_reactive(
        &mut self,
        matrix: &mut ComplexMatrix,
        voltages: &[Value],
        num_nodes: usize,
        omega: Value,
        simparams: GeneratedSimulationParameters,
    ) -> Result<(), GeneratedEvaluationError> {
        let ctx = GeneratedEvalContext::with_analysis_step_and_simparams(
            voltages,
            self.temperature,
            num_nodes,
            GeneratedAnalysisKind::Ac,
            self.analysis_initial_step,
            self.analysis_final_step,
            simparams,
        );
        if !self
            .static_stamp_cache
            .axis_indices_match(&self.nodes, &self.branches, num_nodes)
        {
            let cache = Arc::make_mut(&mut self.static_stamp_cache);
            cache.rebuild_axis_indices(&self.nodes, &self.branches, num_nodes);
            cache.clear_linked_slots();
        }
        let mut stamper = GeneratedReactiveStamper::new_with_local_maps_and_static_cache(
            matrix,
            &self.nodes,
            &self.branches,
            num_nodes,
            omega,
            self.static_stamp_cache.as_ref(),
        );
        self.kind.stamp_reactive(&ctx, &mut stamper);
        match ctx.take_evaluation_error() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

/// Why a netlist card could not be turned into a generated Verilog-A instance.
///
/// This exists so the adapter does not have to name `engine::SimulationError`.
/// `device` sits at rank 8 in the crate's layer order and `engine` at rank 12,
/// so returning the engine's error type here would be an upward edge — the
/// exact inversion `tests/module_layering.rs` exists to forbid. The engine
/// converts on the way out, which is the direction the ranking allows, and both
/// call sites keep the `?` they already had.
///
/// Every case is a malformed or unresolvable instantiation, so one message
/// carries it; a runtime evaluation failure is [`GeneratedEvaluationError`] and
/// a different thing entirely.
#[cfg(feature = "veriloga-builtins-base")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinInstantiationError(pub String);

#[cfg(feature = "veriloga-builtins-base")]
impl std::fmt::Display for BuiltinInstantiationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(feature = "veriloga-builtins-base")]
impl std::error::Error for BuiltinInstantiationError {}

#[cfg(feature = "veriloga-builtins-base")]
pub fn instantiate_builtin(
    model_name: &str,
    instance_name: &str,
    node_names: &[String],
    params: &[(String, crate::netlist::ParametricValue)],
    param_ctx: &crate::netlist::ParamContext,
    circuit: &mut crate::CircuitData,
) -> Result<Option<BuiltinVerilogAInstance>, BuiltinInstantiationError> {
    let scoped = params
        .iter()
        .map(|(name, value)| BuiltinParameterAssignment::instance(name, value.clone()))
        .collect::<Vec<_>>();
    instantiate_builtin_scoped(
        model_name,
        instance_name,
        node_names,
        &scoped,
        param_ctx,
        circuit,
    )
}

#[cfg(feature = "veriloga-builtins-base")]
pub(crate) fn instantiate_builtin_scoped(
    model_name: &str,
    instance_name: &str,
    node_names: &[String],
    params: &[BuiltinParameterAssignment],
    param_ctx: &crate::netlist::ParamContext,
    circuit: &mut crate::CircuitData,
) -> Result<Option<BuiltinVerilogAInstance>, BuiltinInstantiationError> {
    let Some(descriptor_name) = builtins::builtin_names()
        .iter()
        .find(|name| name.eq_ignore_ascii_case(model_name))
        .copied()
    else {
        return Ok(None);
    };

    let descriptor = builtins::descriptor(descriptor_name).ok_or_else(|| {
        BuiltinInstantiationError(format!(
            "Generated Verilog-A model '{}' has no canonical model descriptor",
            descriptor_name
        ))
    })?;
    let external_terminals = descriptor.terminals;
    let expected_nodes = external_terminals.len();
    if node_names.len() != expected_nodes {
        return Err(BuiltinInstantiationError(format!(
            "Generated Verilog-A instance '{}' expects {} terminals for model '{}', found {}",
            instance_name,
            expected_nodes,
            model_name,
            node_names.len()
        )));
    }
    let internal_node_names = descriptor.internal_node_names;
    let total_nodes = descriptor.total_node_count;
    if total_nodes != expected_nodes + internal_node_names.len() {
        return Err(BuiltinInstantiationError(format!(
            "Generated Verilog-A model '{}' descriptor has {} total nodes but declares {} external and {} internal nodes",
            descriptor_name,
            total_nodes,
            expected_nodes,
            internal_node_names.len()
        )));
    }

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
    for assignment in params {
        let value = match &assignment.value {
            crate::netlist::ParametricValue::Resolved(value) => *value,
            crate::netlist::ParametricValue::Expression(expr) => {
                crate::netlist::expr::eval_expression(expr, param_ctx).map_err(|error| {
                    BuiltinInstantiationError(format!(
                        "Failed to resolve generated Verilog-A parameter '{}': {}",
                        assignment.name, error
                    ))
                })?
            }
            crate::netlist::ParametricValue::String(_)
            | crate::netlist::ParametricValue::StringExpression(_) => {
                return Err(BuiltinInstantiationError(format!(
                    "Generated Verilog-A parameter '{}' requires a numeric value",
                    assignment.name
                )));
            }
        };
        resolved.push(GeneratedParameterAssignment::new(
            assignment.name.as_str(),
            value,
            assignment.origin,
        ));
    }

    let branch_count = descriptor.branch_count;
    let mut branches = Vec::with_capacity(branch_count);
    for _ in 0..branch_count {
        branches.push(circuit.allocate_branch());
    }

    let Some(kind) = builtins::instantiate_scoped(descriptor_name, &nodes, &branches, &resolved)
        .map_err(|error| {
            BuiltinInstantiationError(format!(
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
        static_stamp_cache: Arc::new(GeneratedStaticStampCache::default()),
        terminal_currents: vec![0.0; expected_nodes],
        external_terminals,
        terminal_current_aliases: &[],
        kind,
    }))
}

#[cfg(test)]
mod tests {
    use super::{
        GeneratedAnalysisKind, GeneratedEvalContext, GeneratedEvaluationError,
        GeneratedEvaluationMode, GeneratedNoiseDescriptor, GeneratedNoiseEndpoint,
        GeneratedNoiseInjection, GeneratedNoiseKind, GeneratedNoiseTopologyError,
        GeneratedSimulationParameters,
    };

    #[cfg(feature = "veriloga-builtins-base")]
    use super::{BuiltinVerilogADevices, instantiate_builtin};

    #[cfg(feature = "veriloga-builtins-base")]
    #[test]
    fn generated_catalog_has_complete_canonical_external_terminal_metadata() {
        for model_name in super::builtins::builtin_names() {
            let descriptor = super::builtins::descriptor(model_name)
                .unwrap_or_else(|| panic!("{model_name} has a canonical model descriptor"));
            let terminals = descriptor.terminals;
            assert_eq!(
                terminals.len(),
                super::builtins::node_count(model_name).expect("catalog node count"),
                "{model_name} terminal metadata must match the compiled module"
            );
            assert_eq!(
                descriptor.total_node_count,
                terminals.len() + descriptor.internal_node_names.len(),
                "{model_name} descriptor node partition must be complete"
            );
            let mut current_names = std::collections::HashSet::new();
            for terminal in terminals {
                assert!(!terminal.name.is_empty());
                assert!(current_names.insert(terminal.current_parameter));
            }
        }
    }

    #[cfg(feature = "veriloga-builtins-base")]
    fn checkpoint_test_devices() -> BuiltinVerilogADevices {
        let mut circuit = crate::CircuitData::new();
        let mut devices = BuiltinVerilogADevices::new();
        for (name, node) in [("d1", "n1"), ("d2", "n2")] {
            let instance = instantiate_builtin(
                "DIODE_CMC",
                name,
                &[node.to_string(), "0".to_string()],
                &[],
                &crate::netlist::ParamContext::new(),
                &mut circuit,
            )
            .expect("generated diode instantiates")
            .expect("DIODE_CMC is registered");
            devices.add(instance);
        }
        devices
    }

    /// Stamp one current both ways and compare what reached the matrix.
    ///
    /// `stamp_current_packed` exists so a backend carrying derivatives as an
    /// array does not need a distinct entry point per arity. It is only useful
    /// if it puts exactly what the loose-argument calls put.
    #[cfg(feature = "veriloga-builtins-base")]
    fn stamp_both_ways(
        derivative0: crate::Value,
        derivative1: crate::Value,
        voltages: &[crate::Value],
    ) -> (
        (Vec<crate::Value>, Vec<crate::Value>),
        (Vec<crate::Value>, Vec<crate::Value>),
    ) {
        let nodes = vec![1usize, 2usize];
        let branches: Vec<usize> = Vec::new();
        let size = 2usize;
        let triplets: Vec<(usize, usize, crate::Value)> = (0..size)
            .flat_map(|row| (0..size).map(move |col| (row, col, 0.0)))
            .collect();

        let run = |packed: bool| {
            let mut matrix =
                super::StaticMatrix::from_triplets(size, size, &triplets).expect("matrix");
            let mut cache = super::GeneratedStaticStampCache::default();
            cache.link(&matrix, &nodes, &branches, size);
            let mut rhs = vec![0.0 as crate::Value; size];
            {
                let mut stamper = super::GeneratedStamper::new_with_static_cache(
                    &mut matrix,
                    &mut rhs,
                    voltages,
                    size,
                    &cache,
                );
                if packed {
                    stamper.stamp_current_packed(
                        Some(0),
                        Some(1),
                        0.75,
                        &[
                            super::GeneratedStampLane::Node(0),
                            super::GeneratedStampLane::Node(1),
                        ],
                        &[derivative0, derivative1],
                    );
                } else {
                    stamper.stamp_current_node2_local(
                        Some(0),
                        Some(1),
                        0.75,
                        0,
                        derivative0,
                        1,
                        derivative1,
                    );
                }
            }
            (matrix.values_mut().to_vec(), rhs)
        };

        (run(false), run(true))
    }

    #[cfg(feature = "veriloga-builtins-base")]
    #[test]
    fn packed_stamp_matches_the_loose_argument_stamp() {
        let (loose, packed) = stamp_both_ways(0.25, -0.5, &[0.3, -0.2]);
        assert_eq!(loose.0, packed.0, "matrix entries");
        assert_eq!(loose.1, packed.1, "right-hand side");
    }

    #[cfg(feature = "veriloga-builtins-base")]
    #[test]
    fn a_zero_lane_cannot_poison_the_right_hand_side() {
        // A packed array is sized to the device, so most equations leave most
        // lanes at zero. Multiplying such a lane by its unknown would be
        // harmless right up until Newton diverges and that unknown is NaN --
        // and then `0.0 * NaN` is NaN, which would reach the right-hand side
        // for a lane the equation does not even touch. The loose-argument call
        // never sees the lane, so the packed one must skip it.
        let (loose, packed) = stamp_both_ways(0.25, 0.0, &[0.3, crate::Value::NAN]);
        assert!(
            packed.1.iter().all(|value| value.is_finite()),
            "zero lane poisoned the right-hand side: {:?}",
            packed.1
        );
        // This is the one case where the two deliberately disagree, so record
        // that rather than assert equality. The loose call is handed the lane
        // as an argument and multiplies it out unconditionally; the packed one
        // can see the derivative is zero and decline. Asserting they match here
        // would be asserting the packed form reproduces a defect.
        assert!(
            loose.1.iter().any(|value| !value.is_finite()),
            "fixture no longer reaches the case it exists for: {:?}",
            loose.1
        );
        assert_eq!(
            loose.0, packed.0,
            "the matrix is unaffected either way; only the equivalent source differs"
        );
    }

    #[cfg(feature = "veriloga-builtins-base")]
    #[test]
    fn generated_checkpoint_restore_is_atomic_and_validates_provenance() {
        let mut devices = checkpoint_test_devices();
        let mut accepted = devices.checkpoint_states();
        accepted[0].state.ddt_previous[0] = 1.25;
        accepted[0].state.ddt_older[0] = -0.0;
        accepted[0].state.ddt_derivative_previous[0] = f64::MIN_POSITIVE;
        accepted[0].state.ddt_initialized[0] = true;
        devices
            .restore_checkpoint_states(&accepted)
            .expect("valid persistent state restores");
        assert_eq!(devices.checkpoint_states(), accepted);

        let baseline = devices.checkpoint_states();
        let mut invalid_cases = Vec::new();
        invalid_cases.push(Vec::new());

        let mut reordered = baseline.clone();
        reordered.swap(0, 1);
        invalid_cases.push(reordered);

        let mut wrong_name = baseline.clone();
        wrong_name[0].instance_name = "different".to_string();
        invalid_cases.push(wrong_name);

        let mut wrong_model = baseline.clone();
        wrong_model[0].model_name = "different_model".to_string();
        invalid_cases.push(wrong_model);

        let mut wrong_identity = baseline.clone();
        wrong_identity[0].model_identity.push('0');
        invalid_cases.push(wrong_identity);

        let mut wrong_version = baseline.clone();
        wrong_version[0].state_version += 1;
        invalid_cases.push(wrong_version);

        let mut wrong_shape = baseline.clone();
        wrong_shape[0].state.ddt_previous.push(0.0);
        invalid_cases.push(wrong_shape);

        let mut non_finite = baseline.clone();
        non_finite[0].state.ddt_previous[0] = f64::INFINITY;
        invalid_cases.push(non_finite);

        for invalid in invalid_cases {
            assert!(devices.restore_checkpoint_states(&invalid).is_err());
            assert_eq!(
                devices.checkpoint_states(),
                baseline,
                "failed restore must not partially mutate live generated devices"
            );
        }
    }

    #[cfg(feature = "veriloga-builtins-base")]
    #[test]
    fn generated_rollback_refresh_reuses_the_instance_vector() {
        let mut devices = checkpoint_test_devices();
        devices.devices[0]
            .terminal_currents
            .copy_from_slice(&[1.25, -1.25]);
        devices.devices[1]
            .terminal_currents
            .copy_from_slice(&[-0.5, 0.5]);
        let expected = devices.capture_rollback_state();
        let mut reusable = expected.clone();
        let original_capacity = reusable.states.capacity();
        let original_storage = reusable.states.as_ptr();
        let original_current_capacity = reusable.terminal_currents.capacity();
        let original_current_storage = reusable.terminal_currents.as_ptr();

        devices.capture_rollback_state_into(&mut reusable);

        assert_eq!(reusable, expected);
        assert_eq!(reusable.states.capacity(), original_capacity);
        assert_eq!(reusable.states.as_ptr(), original_storage);
        assert_eq!(
            reusable.terminal_currents.capacity(),
            original_current_capacity
        );
        assert_eq!(
            reusable.terminal_currents.as_ptr(),
            original_current_storage
        );

        devices.devices[0].terminal_currents.fill(0.0);
        devices.devices[1].terminal_currents.fill(0.0);
        devices.restore_rollback_state(expected);
        assert_eq!(devices.devices[0].terminal_currents, [1.25, -1.25]);
        assert_eq!(devices.devices[1].terminal_currents, [-0.5, 0.5]);
    }

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
    fn generated_evaluation_context_preserves_the_first_recoverable_failure() {
        let voltages = [0.0];
        let ctx =
            GeneratedEvalContext::with_analysis(&voltages, 300.15, 1, GeneratedAnalysisKind::Dc);

        ctx.report_analog_loop_limit("transient stamp", 11, 10);
        ctx.report_analog_loop_limit("later helper", 21, 20);

        assert_eq!(
            ctx.take_evaluation_error(),
            Some(GeneratedEvaluationError::AnalogLoopLimit {
                phase: "transient stamp",
                iterations: 11,
                limit: 10,
            })
        );
        assert_eq!(ctx.take_evaluation_error(), None);
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

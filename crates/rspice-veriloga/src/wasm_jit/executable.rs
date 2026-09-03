//! Dense, validated solver lookup table for one emitted model module.

use crate::codegen::CompiledModel;

use super::{WasmJitError, WasmJitModelArtifact, WasmJitResult, WasmJitValueRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WasmJitExecutableEntry {
    ParameterDefault(usize),
    StaticCondition(usize),
    StampValue(usize),
    Jacobian { stamp: usize, entry: usize },
    ReactiveJacobian { stamp: usize, entry: usize },
    NoisePsd(usize),
    NoiseExponent(usize),
}

/// O(1) semantic-entry lookup retained by every device instance.
///
/// Generated module bytes remain worker-cache state; the solver retains only
/// authenticated cache/export identities and never reparses a module in its
/// hot path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WasmJitExecutable {
    cache_key: String,
    assignment_export: String,
    /// The CFG route's assignment pass, run once between the assignment pass
    /// and the first value entry that reads one of its slots.
    prelude_export: Option<String>,
    /// How many slots that pass publishes, which is what the context array has
    /// to hold before any of it runs.
    prelude_slots: usize,
    post_assignment_export: Option<String>,
    /// Whole-model drivers, absent when the shared contribution-ordering rule
    /// forbids fusing this model.
    evaluation_kernel_export: Option<String>,
    stamp_kernel_export: Option<String>,
    parameter_defaults: Vec<Option<String>>,
    static_conditions: Vec<Option<String>>,
    stamp_values: Vec<String>,
    jacobians: Vec<Vec<String>>,
    reactive_jacobians: Vec<Vec<String>>,
    noise_psd: Vec<String>,
    noise_exponents: Vec<Option<String>>,
}

impl WasmJitExecutable {
    pub(crate) fn from_artifact(
        model: &CompiledModel,
        artifact: &WasmJitModelArtifact,
    ) -> WasmJitResult<Self> {
        let mut executable = Self {
            cache_key: artifact.cache_key().to_owned(),
            assignment_export: artifact.assignment_export().to_owned(),
            prelude_export: artifact.prelude_export().map(str::to_owned),
            prelude_slots: artifact.prelude_slots(),
            post_assignment_export: artifact.post_assignment_export().map(str::to_owned),
            evaluation_kernel_export: artifact.evaluation_kernel_export().map(str::to_owned),
            stamp_kernel_export: artifact.stamp_kernel_export().map(str::to_owned),
            parameter_defaults: vec![None; model.parameters.len()],
            static_conditions: vec![None; model.stamp_programs.len()],
            stamp_values: vec![String::new(); model.stamp_programs.len()],
            jacobians: model
                .stamp_programs
                .iter()
                .map(|stamp| vec![String::new(); stamp.jacobian_programs.len()])
                .collect(),
            reactive_jacobians: model
                .stamp_programs
                .iter()
                .map(|stamp| vec![String::new(); stamp.reactive_jacobians.len()])
                .collect(),
            noise_psd: vec![String::new(); model.noise_sources.len()],
            noise_exponents: vec![None; model.noise_sources.len()],
        };

        for entry in artifact.entries() {
            let name = entry.export_name();
            match entry.role() {
                WasmJitValueRole::Assignment { .. } | WasmJitValueRole::Prelude => {}
                WasmJitValueRole::ParameterDefault { parameter_index } => set_optional(
                    &mut executable.parameter_defaults,
                    *parameter_index,
                    name,
                    "parameter-default",
                )?,
                WasmJitValueRole::StaticCondition { stamp_index } => set_optional(
                    &mut executable.static_conditions,
                    *stamp_index,
                    name,
                    "static-condition",
                )?,
                WasmJitValueRole::StampValue { stamp_index } => set_required(
                    &mut executable.stamp_values,
                    *stamp_index,
                    name,
                    "stamp-value",
                )?,
                WasmJitValueRole::Jacobian {
                    stamp_index,
                    entry_index,
                } => set_nested_required(
                    &mut executable.jacobians,
                    *stamp_index,
                    *entry_index,
                    name,
                    "Jacobian",
                )?,
                WasmJitValueRole::ReactiveJacobian {
                    stamp_index,
                    entry_index,
                } => set_nested_required(
                    &mut executable.reactive_jacobians,
                    *stamp_index,
                    *entry_index,
                    name,
                    "reactive-Jacobian",
                )?,
                WasmJitValueRole::NoisePowerSpectralDensity { noise_index } => {
                    set_required(&mut executable.noise_psd, *noise_index, name, "noise-PSD")?
                }
                WasmJitValueRole::NoiseExponent { noise_index } => set_optional(
                    &mut executable.noise_exponents,
                    *noise_index,
                    name,
                    "noise-exponent",
                )?,
            }
        }

        for (index, parameter) in model.parameters.iter().enumerate() {
            require_optional_shape(
                &executable.parameter_defaults,
                index,
                parameter.default_program.is_some(),
                "parameter-default",
            )?;
        }
        for (stamp, program) in model.stamp_programs.iter().enumerate() {
            require_optional_shape(
                &executable.static_conditions,
                stamp,
                program.static_condition.is_some(),
                "static-condition",
            )?;
        }
        require_complete(&executable.stamp_values, "stamp-value")?;
        for entries in &executable.jacobians {
            require_complete(entries, "Jacobian")?;
        }
        for entries in &executable.reactive_jacobians {
            require_complete(entries, "reactive-Jacobian")?;
        }
        require_complete(&executable.noise_psd, "noise-PSD")?;
        for (index, source) in model.noise_sources.iter().enumerate() {
            require_optional_shape(
                &executable.noise_exponents,
                index,
                source.exponent_program.is_some(),
                "noise-exponent",
            )?;
        }
        Ok(executable)
    }

    pub(crate) fn export(&self, entry: WasmJitExecutableEntry) -> Option<&str> {
        match entry {
            WasmJitExecutableEntry::ParameterDefault(index) => {
                self.parameter_defaults.get(index)?.as_deref()
            }
            WasmJitExecutableEntry::StaticCondition(index) => {
                self.static_conditions.get(index)?.as_deref()
            }
            WasmJitExecutableEntry::StampValue(index) => nonempty(self.stamp_values.get(index)?),
            WasmJitExecutableEntry::Jacobian { stamp, entry } => {
                nonempty(self.jacobians.get(stamp)?.get(entry)?)
            }
            WasmJitExecutableEntry::ReactiveJacobian { stamp, entry } => {
                nonempty(self.reactive_jacobians.get(stamp)?.get(entry)?)
            }
            WasmJitExecutableEntry::NoisePsd(index) => nonempty(self.noise_psd.get(index)?),
            WasmJitExecutableEntry::NoiseExponent(index) => {
                self.noise_exponents.get(index)?.as_deref()
            }
        }
    }
}

fn index(value: u32, label: &str) -> WasmJitResult<usize> {
    usize::try_from(value)
        .map_err(|_| WasmJitError::Contract(format!("{label} index exceeds this runtime")))
}

fn set_optional(
    slots: &mut [Option<String>],
    raw_index: u32,
    name: &str,
    label: &str,
) -> WasmJitResult<()> {
    let index = index(raw_index, label)?;
    let slot = slots.get_mut(index).ok_or_else(|| {
        WasmJitError::Contract(format!(
            "{label} export index {index} is outside model shape"
        ))
    })?;
    if slot.is_some() {
        return Err(WasmJitError::Contract(format!(
            "duplicate {label} export index {index}"
        )));
    }
    *slot = Some(name.to_owned());
    Ok(())
}

fn set_required(
    slots: &mut [String],
    raw_index: u32,
    name: &str,
    label: &str,
) -> WasmJitResult<()> {
    let index = index(raw_index, label)?;
    let slot = slots.get_mut(index).ok_or_else(|| {
        WasmJitError::Contract(format!(
            "{label} export index {index} is outside model shape"
        ))
    })?;
    if !slot.is_empty() {
        return Err(WasmJitError::Contract(format!(
            "duplicate {label} export index {index}"
        )));
    }
    *slot = name.to_owned();
    Ok(())
}

fn set_nested_required(
    slots: &mut [Vec<String>],
    raw_outer: u32,
    raw_inner: u32,
    name: &str,
    label: &str,
) -> WasmJitResult<()> {
    let outer = index(raw_outer, label)?;
    let inner = index(raw_inner, label)?;
    let entries = slots.get_mut(outer).ok_or_else(|| {
        WasmJitError::Contract(format!(
            "{label} stamp index {outer} is outside model shape"
        ))
    })?;
    set_required(entries, raw_inner, name, label).map_err(|error| match error {
        WasmJitError::Contract(detail) => {
            WasmJitError::Contract(format!("{label} stamp {outer}, entry {inner}: {detail}"))
        }
        other => other,
    })
}

fn require_optional_shape(
    slots: &[Option<String>],
    index: usize,
    expected: bool,
    label: &str,
) -> WasmJitResult<()> {
    let present = slots.get(index).is_some_and(Option::is_some);
    if present == expected {
        Ok(())
    } else {
        Err(WasmJitError::Contract(format!(
            "{label} export presence at index {index} does not match model shape"
        )))
    }
}

fn require_complete(slots: &[String], label: &str) -> WasmJitResult<()> {
    if let Some(index) = slots.iter().position(String::is_empty) {
        Err(WasmJitError::Contract(format!(
            "missing {label} export at index {index}"
        )))
    } else {
        Ok(())
    }
}

fn nonempty(value: &str) -> Option<&str> {
    (!value.is_empty()).then_some(value)
}

#[cfg(target_arch = "wasm32")]
impl WasmJitExecutable {
    pub(crate) fn run_entry(
        &self,
        entry: WasmJitExecutableEntry,
        context: &mut crate::vm::VmContext,
    ) -> Result<f64, String> {
        let export = self
            .export(entry)
            .ok_or_else(|| format!("model module is missing semantic entry {entry:?}"))?
            .to_owned();
        self.dispatch(&export, context).map(|frame| frame.result)
    }

    pub(crate) fn run_assignments(&self, context: &mut crate::vm::VmContext) -> Result<(), String> {
        self.dispatch(&self.assignment_export, context).map(|_| ())
    }

    /// Run the CFG route's assignment pass, which publishes every value entry's
    /// output into a prelude slot. A postfix plan has none and this is a no-op.
    pub(crate) fn run_prelude(&self, context: &mut crate::vm::VmContext) -> Result<(), String> {
        let Some(export) = self.prelude_export.as_deref() else {
            return Ok(());
        };
        self.dispatch(export, context).map(|_| ())
    }

    /// Run the assignment pass and every stamp value in one dispatch, plus
    /// every Jacobian entry when the caller supplies somewhere to put them.
    ///
    /// Supplying `jacobians` is what selects the stamp driver over the
    /// evaluation driver. Returns `Ok(false)` when this model cannot fuse,
    /// leaving the caller on the per-entry path. Both the activation mirror
    /// and the Jacobian array come from the caller because they are per
    /// instance rather than per model.
    pub(crate) fn run_fused_kernel(
        &self,
        context: &mut crate::vm::VmContext,
        program_active: &[u8],
        jacobians: Option<&mut [f64]>,
    ) -> Result<bool, String> {
        let with_jacobians = jacobians.is_some();
        let export = if with_jacobians {
            self.stamp_kernel_export.as_deref()
        } else {
            self.evaluation_kernel_export.as_deref()
        };
        let Some(export) = export.map(str::to_owned) else {
            return Ok(false);
        };
        self.dispatch_kernel(&export, context, program_active, jacobians)
            .map(|()| true)
    }

    pub(crate) fn evaluation_kernel_is_eligible(&self) -> bool {
        self.evaluation_kernel_export.is_some()
    }

    pub(crate) fn stamp_kernel_is_eligible(&self) -> bool {
        self.stamp_kernel_export.is_some()
    }

    pub(crate) fn run_post_assignments(
        &self,
        context: &mut crate::vm::VmContext,
    ) -> Result<(), String> {
        let Some(export) = self.post_assignment_export.as_deref() else {
            return Ok(());
        };
        self.dispatch(export, context).map(|_| ())
    }

    /// Dispatch a fused driver, additionally publishing the per-instance
    /// activation and Jacobian capabilities into the frame.
    fn dispatch_kernel(
        &self,
        export: &str,
        context: &mut crate::vm::VmContext,
        program_active: &[u8],
        jacobians: Option<&mut [f64]>,
    ) -> Result<(), String> {
        self.ensure_prelude_slots(context);
        let mut frame = evaluation_frame(context)?;
        (frame.program_active_ptr, frame.program_active_len) = slice_capability(program_active)?;
        if let Some(jacobians) = jacobians {
            (frame.jacobians_ptr, frame.jacobians_len) = slice_capability(jacobians)?;
        }
        self.dispatch_frame(export, context, frame).map(|_| ())
    }

    /// Give the context room for every slot this module's prelude publishes.
    ///
    /// The wasm frame carries the array as a capability, so it has to be the
    /// right length before the frame is built rather than before the call.
    /// Zero-length for every postfix plan, which leaves the frame's slot
    /// capability null exactly as it was.
    fn ensure_prelude_slots(&self, context: &mut crate::vm::VmContext) {
        if context.prelude_slots.len() < self.prelude_slots {
            context.prelude_slots.resize(self.prelude_slots, 0.0);
        }
    }

    fn dispatch(
        &self,
        export: &str,
        context: &mut crate::vm::VmContext,
    ) -> Result<super::WasmJitEvalFrame, String> {
        self.ensure_prelude_slots(context);
        let frame = evaluation_frame(context)?;
        self.dispatch_frame(export, context, frame)
    }

    fn dispatch_frame(
        &self,
        export: &str,
        context: &mut crate::vm::VmContext,
        frame: super::WasmJitEvalFrame,
    ) -> Result<super::WasmJitEvalFrame, String> {
        let mut dispatch_frame = super::abi::WasmJitDispatchFrame::new(frame);
        let frame_offset =
            pointer_offset((&mut dispatch_frame.frame as *mut super::WasmJitEvalFrame).cast())?;
        let session = super::WasmJitRuntimeSession::new(std::mem::take(context));
        let (dispatch, mut session) = super::with_runtime_session(frame_offset, session, || {
            super::dispatch_model_entry(&self.cache_key, export, frame_offset)
        });
        let helper_error = session.take_error();
        *context = session.into_context();

        let status = dispatch??;
        if let Some(error) = helper_error {
            return Err(error);
        }
        if status != super::WASM_JIT_STATUS_OK {
            return Err(format!(
                "secondary module export '{export}' returned status {status}"
            ));
        }
        if dispatch_frame.frame.error_status != super::WASM_JIT_STATUS_OK {
            return Err(format!(
                "secondary module export '{export}' recorded runtime status {}",
                dispatch_frame.frame.error_status
            ));
        }
        Ok(dispatch_frame.frame)
    }
}

#[cfg(target_arch = "wasm32")]
fn evaluation_frame(context: &crate::vm::VmContext) -> Result<super::WasmJitEvalFrame, String> {
    let (parameters_ptr, parameters_len) = slice_capability(&context.parameters)?;
    let (parameter_given_ptr, parameter_given_len) = slice_capability(&context.param_given)?;
    let (port_connected_ptr, port_connected_len) = slice_capability(&context.port_connected)?;
    let (terminal_voltages_ptr, terminal_voltages_len) = slice_capability(&context.voltages)?;
    let (internal_voltages_ptr, internal_voltages_len) =
        slice_capability(&context.internal_voltages)?;
    let current_pairs = unsafe {
        std::slice::from_raw_parts(
            context.terminal_pair_currents_ptr(),
            context.terminal_pair_currents_len(),
        )
    };
    let (currents_ptr, currents_len) = slice_capability(current_pairs)?;
    let (prior_currents_ptr, prior_currents_len) = slice_capability(&context.currents)?;
    let (branch_unknowns_ptr, branch_unknowns_len) =
        slice_capability(&context.branch_current_values)?;
    let (variables_ptr, variables_len) = slice_capability(&context.variables)?;
    let (prelude_slots_ptr, prelude_slots_len) = slice_capability(&context.prelude_slots)?;

    Ok(super::WasmJitEvalFrame {
        parameters_ptr,
        parameters_len,
        parameter_given_ptr,
        parameter_given_len,
        port_connected_ptr,
        port_connected_len,
        terminal_voltages_ptr,
        terminal_voltages_len,
        internal_voltages_ptr,
        internal_voltages_len,
        currents_ptr,
        currents_len,
        prior_currents_ptr,
        prior_currents_len,
        branch_unknowns_ptr,
        branch_unknowns_len,
        variables_ptr,
        variables_len,
        prelude_slots_ptr,
        prelude_slots_len,
        analysis_mask: analysis_mask(context),
        temperature: context.temperature,
        thermal_voltage: context.vt(),
        time: context.time,
        m_factor: context.multiplicity,
        ..super::WasmJitEvalFrame::default()
    })
}

#[cfg(target_arch = "wasm32")]
fn slice_capability<T>(slice: &[T]) -> Result<(u32, u32), String> {
    let len = u32::try_from(slice.len())
        .map_err(|_| "WASM JIT array length exceeds wasm32".to_owned())?;
    if slice.is_empty() {
        return Ok((0, 0));
    }
    Ok((pointer_offset(slice.as_ptr().cast())?, len))
}

#[cfg(target_arch = "wasm32")]
fn pointer_offset(pointer: *const u8) -> Result<u32, String> {
    u32::try_from(pointer as usize)
        .map_err(|_| "WASM JIT memory capability is outside wasm32".to_owned())
}

#[cfg(target_arch = "wasm32")]
fn analysis_mask(context: &crate::vm::VmContext) -> u32 {
    let mut mask = 1_u32
        .checked_shl(u32::from(context.analysis_type))
        .unwrap_or(0);
    if matches!(context.analysis_type, 0 | 4) {
        mask |= 1 << 5;
    }
    if matches!(context.analysis_type, 1 | 3) {
        mask |= 1 << 6;
    }
    if context.analysis_initial_step {
        mask |= 1 << 7;
    }
    if context.analysis_final_step {
        mask |= 1 << 8;
    }
    mask
}

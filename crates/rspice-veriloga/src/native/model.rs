use super::runtime::ExecutableMemory;
use super::{EvalContext, JitError, JitResult};
use crate::codegen::{AssignmentStep, BytecodeProgram, CompiledModel, Instruction};
use crate::vm::terminal_pair_current_endpoints;

type AssignmentEntry = unsafe extern "C" fn(*const EvalContext, *mut f64);
type ValueEntry = unsafe extern "C" fn(*const EvalContext, *const f64) -> f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CodeOffset(usize);

impl CodeOffset {
    #[allow(dead_code)]
    pub(crate) fn new(offset: usize) -> Self {
        Self(offset)
    }

    #[allow(dead_code)]
    pub(crate) fn as_usize(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NativeEntryOffsets {
    pub assignment: CodeOffset,
    pub parameter_defaults: Vec<Option<CodeOffset>>,
    pub static_conditions: Vec<Option<CodeOffset>>,
    pub stamp_values: Vec<CodeOffset>,
    pub jacobians: Vec<Vec<CodeOffset>>,
    pub reactive_jacobians: Vec<Vec<CodeOffset>>,
    pub noise_psd: Vec<CodeOffset>,
    pub noise_exponents: Vec<Option<CodeOffset>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct NativeCurrentDependencies {
    pub assignment_current_pairs: Vec<usize>,
    pub assignment_prior_currents: Vec<usize>,
    pub assignment_branch_unknowns: Vec<usize>,
    pub static_condition_branch_unknowns: Vec<Vec<usize>>,
    pub stamp_values: Vec<Vec<usize>>,
    pub stamp_value_prior_currents: Vec<Vec<usize>>,
    pub stamp_value_branch_unknowns: Vec<Vec<usize>>,
    pub jacobians: Vec<Vec<Vec<usize>>>,
    pub jacobian_prior_currents: Vec<Vec<Vec<usize>>>,
    pub jacobian_branch_unknowns: Vec<Vec<Vec<usize>>>,
    pub reactive_jacobians: Vec<Vec<Vec<usize>>>,
    pub reactive_jacobian_prior_currents: Vec<Vec<Vec<usize>>>,
    pub reactive_jacobian_branch_unknowns: Vec<Vec<Vec<usize>>>,
    pub noise_psd: Vec<Vec<usize>>,
    pub noise_psd_prior_currents: Vec<Vec<usize>>,
    pub noise_psd_branch_unknowns: Vec<Vec<usize>>,
    pub noise_exponents: Vec<Vec<usize>>,
    pub noise_exponent_prior_currents: Vec<Vec<usize>>,
    pub noise_exponent_branch_unknowns: Vec<Vec<usize>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) struct NativeRequiredStorage {
    pub state_values: usize,
    pub state_values_prev: usize,
    pub state_initialized: usize,
    pub lookup_tables: usize,
    pub laplace_filters: usize,
    pub zi_filters: usize,
    pub transition_filters: usize,
    pub slew_filters: usize,
    pub delay_buffers: usize,
    pub cross_detectors: usize,
}

impl NativeRequiredStorage {
    pub(crate) fn for_model(model: &CompiledModel) -> Self {
        #[inline]
        fn update_max(max_slot: &mut Option<usize>, index: usize) {
            *max_slot = Some(max_slot.map_or(index, |prev| prev.max(index)));
        }

        let mut max_state = None;
        let mut max_limit_state = None;
        let mut max_transition_filter = None;
        let mut max_slew_filter = None;
        let mut max_delay_buffer = None;
        let mut max_cross_detector = None;

        let mut scan_program = |program: &BytecodeProgram| {
            for instruction in &program.instructions {
                match instruction {
                    Instruction::DdtState(index)
                    | Instruction::IdtState(index)
                    | Instruction::IdtModState(index) => update_max(&mut max_state, *index),
                    Instruction::LimitState(index) => {
                        update_max(&mut max_state, *index);
                        update_max(&mut max_limit_state, *index);
                    }
                    Instruction::TransitionState(index) => {
                        update_max(&mut max_transition_filter, *index);
                    }
                    Instruction::SlewState(index) => update_max(&mut max_slew_filter, *index),
                    Instruction::AbsDelayState(index) => update_max(&mut max_delay_buffer, *index),
                    Instruction::CrossState(index) => update_max(&mut max_cross_detector, *index),
                    _ => {}
                }
            }
        };

        for parameter in &model.parameters {
            if let Some(program) = &parameter.default_program {
                scan_program(program);
            }
        }
        scan_assignment_steps(&model.assignment_steps, &mut scan_program);
        for stamp in &model.stamp_programs {
            if let Some(condition) = &stamp.static_condition {
                scan_program(condition);
            }
            scan_program(&stamp.value_program);
            for jacobian in &stamp.jacobian_programs {
                scan_program(&jacobian.program);
            }
            for jacobian in &stamp.reactive_jacobians {
                scan_program(&jacobian.program);
            }
        }
        for source in &model.noise_sources {
            scan_program(&source.psd_program);
            if let Some(program) = &source.exponent_program {
                scan_program(program);
            }
        }

        let state_values = max_state.map_or(0, |index| index + 1);
        Self {
            state_values,
            state_values_prev: state_values,
            state_initialized: max_limit_state.map_or(0, |index| index + 1),
            lookup_tables: model.lookup_tables.len(),
            laplace_filters: model.laplace_filters.len(),
            zi_filters: model.zi_filters.len(),
            transition_filters: max_transition_filter.map_or(0, |index| index + 1),
            slew_filters: max_slew_filter.map_or(0, |index| index + 1),
            delay_buffers: max_delay_buffer.map_or(0, |index| index + 1),
            cross_detectors: max_cross_detector.map_or(0, |index| index + 1),
        }
    }
}

fn scan_assignment_steps(
    steps: &[AssignmentStep],
    scan_program: &mut impl FnMut(&BytecodeProgram),
) {
    for step in steps {
        match step {
            AssignmentStep::Assign(assignment) => {
                scan_program(&assignment.program);
            }
            AssignmentStep::AssignIndexed { index, value, .. } => {
                scan_program(index);
                scan_program(value);
            }
            AssignmentStep::Loop { condition, body } => {
                scan_program(condition);
                scan_assignment_steps(body, scan_program);
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub struct PlanStats {
    pub assignment_entry_points: usize,
    pub parameter_default_entry_points: usize,
    pub static_condition_entry_points: usize,
    pub stamp_value_entry_points: usize,
    pub jacobian_entry_points: usize,
    pub reactive_jacobian_entry_points: usize,
    pub noise_source_entry_points: usize,
}

pub struct NativeModel {
    pub num_terminals: usize,
    pub num_internal_nodes: usize,
    pub num_variables: usize,
    pub num_parameters: usize,
    image: ExecutableMemory,
    entries: NativeEntryOffsets,
    current_dependencies: NativeCurrentDependencies,
    required_storage: NativeRequiredStorage,
    stats: PlanStats,
}

impl std::fmt::Debug for NativeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeModel")
            .field("num_terminals", &self.num_terminals)
            .field("num_internal_nodes", &self.num_internal_nodes)
            .field("num_variables", &self.num_variables)
            .field("num_parameters", &self.num_parameters)
            .field("image_len", &self.image.len())
            .field("required_storage", &self.required_storage)
            .field("stats", &self.stats)
            .finish()
    }
}

// Safety: NativeModel owns immutable executable memory and immutable entry
// offsets. Calls use only caller-supplied EvalContext/variable pointers.
unsafe impl Send for NativeModel {}
unsafe impl Sync for NativeModel {}

impl NativeModel {
    #[allow(dead_code)]
    pub(crate) fn from_executable_image(
        num_variables: usize,
        num_parameters: usize,
        image: ExecutableMemory,
        entries: NativeEntryOffsets,
    ) -> JitResult<Self> {
        let current_dependencies = Self::empty_current_dependencies(&entries);
        Self::from_executable_image_with_dependencies(
            0,
            0,
            num_variables,
            num_parameters,
            0,
            image,
            entries,
            current_dependencies,
            NativeRequiredStorage::default(),
        )
    }

    pub(crate) fn from_executable_image_with_dependencies(
        num_terminals: usize,
        num_internal_nodes: usize,
        num_variables: usize,
        num_parameters: usize,
        num_branch_unknowns: usize,
        image: ExecutableMemory,
        entries: NativeEntryOffsets,
        current_dependencies: NativeCurrentDependencies,
        required_storage: NativeRequiredStorage,
    ) -> JitResult<Self> {
        Self::validate_entry_offsets(&entries, image.len(), num_parameters)?;
        Self::validate_current_dependencies(
            &entries,
            &current_dependencies,
            num_terminals,
            num_branch_unknowns,
        )?;

        let jacobian_entry_points = entries.jacobians.iter().map(Vec::len).sum();
        let reactive_jacobian_entry_points = entries.reactive_jacobians.iter().map(Vec::len).sum();
        let parameter_default_entry_points = entries.parameter_defaults.iter().flatten().count();
        let static_condition_entry_points = entries.static_conditions.iter().flatten().count();
        let noise_source_entry_points =
            entries.noise_psd.len() + entries.noise_exponents.iter().flatten().count();
        let stats = PlanStats {
            assignment_entry_points: 1,
            parameter_default_entry_points,
            static_condition_entry_points,
            stamp_value_entry_points: entries.stamp_values.len(),
            jacobian_entry_points,
            reactive_jacobian_entry_points,
            noise_source_entry_points,
        };

        Ok(Self {
            num_terminals,
            num_internal_nodes,
            num_variables,
            num_parameters,
            image,
            entries,
            current_dependencies,
            required_storage,
            stats,
        })
    }

    #[cfg(all(test, target_arch = "x86_64"))]
    pub(crate) fn new_for_test(
        num_variables: usize,
        stamp_value_entry_points: usize,
        jacobian_entry_points: Vec<usize>,
        reactive_jacobian_entry_points: Vec<usize>,
    ) -> Self {
        let mut bytes = vec![0xC3]; // assignment: ret
        let stamp_entry = append_test_value_stub(&mut bytes, 1);
        let jacobian_entry = append_test_value_stub(&mut bytes, 2);
        let reactive_jacobian_entry = append_test_value_stub(&mut bytes, 3);
        let image = ExecutableMemory::allocate(&bytes).expect("allocate native test image");
        assert_eq!(
            jacobian_entry_points.len(),
            stamp_value_entry_points,
            "test native model must provide one Jacobian row per stamp"
        );
        assert_eq!(
            reactive_jacobian_entry_points.len(),
            stamp_value_entry_points,
            "test native model must provide one reactive-Jacobian row per stamp"
        );
        let entries = NativeEntryOffsets {
            assignment: CodeOffset::new(0),
            parameter_defaults: vec![],
            static_conditions: vec![Some(stamp_entry); stamp_value_entry_points],
            stamp_values: vec![stamp_entry; stamp_value_entry_points],
            jacobians: jacobian_entry_points
                .into_iter()
                .map(|count| vec![jacobian_entry; count])
                .collect(),
            reactive_jacobians: reactive_jacobian_entry_points
                .into_iter()
                .map(|count| vec![reactive_jacobian_entry; count])
                .collect(),
            noise_psd: vec![],
            noise_exponents: vec![],
        };

        Self::from_executable_image(num_variables, 0, image, entries)
            .expect("publish native test model")
    }

    #[allow(dead_code)]
    fn validate_entry_offsets(
        entries: &NativeEntryOffsets,
        image_len: usize,
        num_parameters: usize,
    ) -> JitResult<()> {
        if entries.parameter_defaults.len() != num_parameters {
            return Err(JitError::InternalCompilerError {
                model: "native-model".into(),
                detail: format!(
                    "parameter-default entry shape {} does not match parameter count {}",
                    entries.parameter_defaults.len(),
                    num_parameters
                )
                .into(),
            });
        }
        if entries.static_conditions.len() != entries.stamp_values.len() {
            return Err(JitError::InternalCompilerError {
                model: "native-model".into(),
                detail: "static-condition entry shape does not match stamp entry shape".into(),
            });
        }
        if entries.jacobians.len() != entries.stamp_values.len() {
            return Err(JitError::InternalCompilerError {
                model: "native-model".into(),
                detail: "jacobian entry shape does not match stamp entry shape".into(),
            });
        }
        if entries.reactive_jacobians.len() != entries.stamp_values.len() {
            return Err(JitError::InternalCompilerError {
                model: "native-model".into(),
                detail: "reactive-jacobian entry shape does not match stamp entry shape".into(),
            });
        }
        if entries.noise_exponents.len() != entries.noise_psd.len() {
            return Err(JitError::InternalCompilerError {
                model: "native-model".into(),
                detail: "noise exponent entry shape does not match noise PSD entry shape".into(),
            });
        }

        Self::validate_entry_offset(entries.assignment, image_len)?;
        for offset in entries.parameter_defaults.iter().flatten() {
            Self::validate_entry_offset(*offset, image_len)?;
        }
        for offset in entries.static_conditions.iter().flatten() {
            Self::validate_entry_offset(*offset, image_len)?;
        }
        for offset in &entries.stamp_values {
            Self::validate_entry_offset(*offset, image_len)?;
        }
        for stamp_entries in &entries.jacobians {
            for offset in stamp_entries {
                Self::validate_entry_offset(*offset, image_len)?;
            }
        }
        for stamp_entries in &entries.reactive_jacobians {
            for offset in stamp_entries {
                Self::validate_entry_offset(*offset, image_len)?;
            }
        }
        for offset in &entries.noise_psd {
            Self::validate_entry_offset(*offset, image_len)?;
        }
        for offset in entries.noise_exponents.iter().flatten() {
            Self::validate_entry_offset(*offset, image_len)?;
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn validate_entry_offset(offset: CodeOffset, image_len: usize) -> JitResult<()> {
        if offset.0 >= image_len {
            return Err(JitError::ExecutableMemory {
                detail: format!(
                    "entry offset {} outside executable image length {}",
                    offset.0, image_len
                )
                .into(),
            });
        }
        Ok(())
    }

    fn empty_current_dependencies(entries: &NativeEntryOffsets) -> NativeCurrentDependencies {
        NativeCurrentDependencies {
            assignment_current_pairs: Vec::new(),
            assignment_prior_currents: Vec::new(),
            assignment_branch_unknowns: Vec::new(),
            static_condition_branch_unknowns: vec![Vec::new(); entries.static_conditions.len()],
            stamp_values: vec![Vec::new(); entries.stamp_values.len()],
            stamp_value_prior_currents: vec![Vec::new(); entries.stamp_values.len()],
            stamp_value_branch_unknowns: vec![Vec::new(); entries.stamp_values.len()],
            jacobians: entries
                .jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
            jacobian_prior_currents: entries
                .jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
            jacobian_branch_unknowns: entries
                .jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
            reactive_jacobians: entries
                .reactive_jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
            reactive_jacobian_prior_currents: entries
                .reactive_jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
            reactive_jacobian_branch_unknowns: entries
                .reactive_jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
            noise_psd: vec![Vec::new(); entries.noise_psd.len()],
            noise_psd_prior_currents: vec![Vec::new(); entries.noise_psd.len()],
            noise_psd_branch_unknowns: vec![Vec::new(); entries.noise_psd.len()],
            noise_exponents: vec![Vec::new(); entries.noise_exponents.len()],
            noise_exponent_prior_currents: vec![Vec::new(); entries.noise_exponents.len()],
            noise_exponent_branch_unknowns: vec![Vec::new(); entries.noise_exponents.len()],
        }
    }

    fn validate_current_dependencies(
        entries: &NativeEntryOffsets,
        dependencies: &NativeCurrentDependencies,
        num_terminals: usize,
        num_branch_unknowns: usize,
    ) -> JitResult<()> {
        if dependencies.static_condition_branch_unknowns.len() != entries.static_conditions.len()
            || dependencies.stamp_values.len() != entries.stamp_values.len()
            || dependencies.stamp_value_prior_currents.len() != entries.stamp_values.len()
            || dependencies.stamp_value_branch_unknowns.len() != entries.stamp_values.len()
            || dependencies.jacobians.len() != entries.jacobians.len()
            || dependencies.jacobian_prior_currents.len() != entries.jacobians.len()
            || dependencies.jacobian_branch_unknowns.len() != entries.jacobians.len()
            || dependencies.reactive_jacobians.len() != entries.reactive_jacobians.len()
            || dependencies.reactive_jacobian_prior_currents.len()
                != entries.reactive_jacobians.len()
            || dependencies.reactive_jacobian_branch_unknowns.len()
                != entries.reactive_jacobians.len()
            || dependencies
                .jacobians
                .iter()
                .zip(&entries.jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
            || dependencies
                .jacobian_prior_currents
                .iter()
                .zip(&entries.jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
            || dependencies
                .jacobian_branch_unknowns
                .iter()
                .zip(&entries.jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
            || dependencies
                .reactive_jacobians
                .iter()
                .zip(&entries.reactive_jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
            || dependencies
                .reactive_jacobian_prior_currents
                .iter()
                .zip(&entries.reactive_jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
            || dependencies
                .reactive_jacobian_branch_unknowns
                .iter()
                .zip(&entries.reactive_jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
            || dependencies.noise_psd.len() != entries.noise_psd.len()
            || dependencies.noise_psd_prior_currents.len() != entries.noise_psd.len()
            || dependencies.noise_psd_branch_unknowns.len() != entries.noise_psd.len()
            || dependencies.noise_exponents.len() != entries.noise_exponents.len()
            || dependencies.noise_exponent_prior_currents.len() != entries.noise_exponents.len()
            || dependencies.noise_exponent_branch_unknowns.len() != entries.noise_exponents.len()
        {
            return Err(JitError::InternalCompilerError {
                model: "native-model".into(),
                detail: "current dependency shape does not match native entry shape".into(),
            });
        }

        Self::validate_current_pair_dependency_list(
            "assignment",
            &dependencies.assignment_current_pairs,
            num_terminals,
        )?;
        Self::validate_current_pair_dependency_table(
            "stamp value",
            &dependencies.stamp_values,
            num_terminals,
        )?;
        Self::validate_current_pair_dependency_nested_table(
            "jacobian",
            &dependencies.jacobians,
            num_terminals,
        )?;
        Self::validate_current_pair_dependency_nested_table(
            "reactive jacobian",
            &dependencies.reactive_jacobians,
            num_terminals,
        )?;
        Self::validate_current_pair_dependency_table(
            "noise psd",
            &dependencies.noise_psd,
            num_terminals,
        )?;
        Self::validate_current_pair_dependency_table(
            "noise exponent",
            &dependencies.noise_exponents,
            num_terminals,
        )?;

        Self::validate_branch_unknown_dependency_list(
            "assignment",
            &dependencies.assignment_branch_unknowns,
            num_branch_unknowns,
        )?;
        Self::validate_branch_unknown_dependency_table(
            "static condition",
            &dependencies.static_condition_branch_unknowns,
            num_branch_unknowns,
        )?;
        Self::validate_branch_unknown_dependency_table(
            "stamp value",
            &dependencies.stamp_value_branch_unknowns,
            num_branch_unknowns,
        )?;
        Self::validate_branch_unknown_dependency_nested_table(
            "jacobian",
            &dependencies.jacobian_branch_unknowns,
            num_branch_unknowns,
        )?;
        Self::validate_branch_unknown_dependency_nested_table(
            "reactive jacobian",
            &dependencies.reactive_jacobian_branch_unknowns,
            num_branch_unknowns,
        )?;
        Self::validate_branch_unknown_dependency_table(
            "noise psd",
            &dependencies.noise_psd_branch_unknowns,
            num_branch_unknowns,
        )?;
        Self::validate_branch_unknown_dependency_table(
            "noise exponent",
            &dependencies.noise_exponent_branch_unknowns,
            num_branch_unknowns,
        )?;

        Ok(())
    }

    fn validate_current_pair_dependency_table(
        table_name: &str,
        dependencies: &[Vec<usize>],
        num_terminals: usize,
    ) -> JitResult<()> {
        for (entry_index, entry_dependencies) in dependencies.iter().enumerate() {
            Self::validate_current_pair_dependency_list(
                &format!("{table_name} {entry_index}"),
                entry_dependencies,
                num_terminals,
            )?;
        }

        Ok(())
    }

    fn validate_current_pair_dependency_nested_table(
        table_name: &str,
        dependencies: &[Vec<Vec<usize>>],
        num_terminals: usize,
    ) -> JitResult<()> {
        for (outer_index, outer_dependencies) in dependencies.iter().enumerate() {
            for (inner_index, entry_dependencies) in outer_dependencies.iter().enumerate() {
                Self::validate_current_pair_dependency_list(
                    &format!("{table_name} {outer_index}.{inner_index}"),
                    entry_dependencies,
                    num_terminals,
                )?;
            }
        }

        Ok(())
    }

    fn validate_current_pair_dependency_list(
        dependency_name: &str,
        dependencies: &[usize],
        num_terminals: usize,
    ) -> JitResult<()> {
        for index in dependencies {
            if terminal_pair_current_endpoints(*index, num_terminals).is_none() {
                return Err(JitError::InternalCompilerError {
                    model: "native-model".into(),
                    detail: format!(
                        "{dependency_name} terminal-pair current dependency {index} invalid for compiled terminal count {num_terminals}"
                    )
                    .into(),
                });
            }
        }

        Ok(())
    }

    fn validate_branch_unknown_dependency_table(
        table_name: &str,
        dependencies: &[Vec<usize>],
        num_branch_unknowns: usize,
    ) -> JitResult<()> {
        for (entry_index, entry_dependencies) in dependencies.iter().enumerate() {
            Self::validate_branch_unknown_dependency_list(
                &format!("{table_name} {entry_index}"),
                entry_dependencies,
                num_branch_unknowns,
            )?;
        }

        Ok(())
    }

    fn validate_branch_unknown_dependency_nested_table(
        table_name: &str,
        dependencies: &[Vec<Vec<usize>>],
        num_branch_unknowns: usize,
    ) -> JitResult<()> {
        for (outer_index, outer_dependencies) in dependencies.iter().enumerate() {
            for (inner_index, entry_dependencies) in outer_dependencies.iter().enumerate() {
                Self::validate_branch_unknown_dependency_list(
                    &format!("{table_name} {outer_index}.{inner_index}"),
                    entry_dependencies,
                    num_branch_unknowns,
                )?;
            }
        }

        Ok(())
    }

    fn validate_branch_unknown_dependency_list(
        dependency_name: &str,
        dependencies: &[usize],
        num_branch_unknowns: usize,
    ) -> JitResult<()> {
        for index in dependencies {
            if *index >= num_branch_unknowns {
                return Err(JitError::InternalCompilerError {
                    model: "native-model".into(),
                    detail: format!(
                        "{dependency_name} branch-current unknown dependency {index} outside compiled branch-current unknown count {num_branch_unknowns}"
                    )
                    .into(),
                });
            }
        }

        Ok(())
    }

    fn entry_ptr(&self, offset: CodeOffset) -> *const u8 {
        self.image
            .ptr_at(offset.0)
            .expect("validated native entry offset")
    }

    pub(crate) fn run_assignments(&self, ctx: &EvalContext, vars: *mut f64) {
        // Safety: from_executable_image validated this offset is inside the
        // executable image owned by self, and the backend records it with the
        // AssignmentEntry ABI.
        let entry: AssignmentEntry =
            unsafe { std::mem::transmute(self.entry_ptr(self.entries.assignment)) };
        // Safety: callers provide pointers matching the native assignment ABI.
        unsafe { entry(ctx as *const EvalContext, vars) };
    }

    pub(crate) fn assignment_current_pairs(&self) -> &[usize] {
        &self.current_dependencies.assignment_current_pairs
    }

    pub(crate) fn assignment_prior_currents(&self) -> &[usize] {
        &self.current_dependencies.assignment_prior_currents
    }

    pub(crate) fn assignment_branch_unknowns(&self) -> &[usize] {
        &self.current_dependencies.assignment_branch_unknowns
    }

    pub(crate) fn required_storage(&self) -> NativeRequiredStorage {
        self.required_storage
    }

    pub(crate) fn run_parameter_default(
        &self,
        index: usize,
        ctx: &EvalContext,
        vars: *const f64,
    ) -> Option<f64> {
        self.entries
            .parameter_defaults
            .get(index)
            .and_then(|offset| offset.map(|offset| self.run_value_entry(offset, ctx, vars)))
    }

    pub(crate) fn run_stamp_value(&self, index: usize, ctx: &EvalContext, vars: *const f64) -> f64 {
        self.run_value_entry(self.entries.stamp_values[index], ctx, vars)
    }

    pub(crate) fn run_static_condition(
        &self,
        index: usize,
        ctx: &EvalContext,
        vars: *const f64,
    ) -> Option<f64> {
        self.entries
            .static_conditions
            .get(index)
            .and_then(|offset| offset.map(|offset| self.run_value_entry(offset, ctx, vars)))
    }

    pub(crate) fn stamp_value_current_pairs(&self, index: usize) -> &[usize] {
        &self.current_dependencies.stamp_values[index]
    }

    pub(crate) fn stamp_value_prior_currents(&self, index: usize) -> &[usize] {
        &self.current_dependencies.stamp_value_prior_currents[index]
    }

    pub(crate) fn static_condition_branch_unknowns(&self, index: usize) -> &[usize] {
        &self.current_dependencies.static_condition_branch_unknowns[index]
    }

    pub(crate) fn stamp_value_branch_unknowns(&self, index: usize) -> &[usize] {
        &self.current_dependencies.stamp_value_branch_unknowns[index]
    }

    pub(crate) fn run_jacobian(
        &self,
        stamp: usize,
        entry: usize,
        ctx: &EvalContext,
        vars: *const f64,
    ) -> f64 {
        self.run_value_entry(self.entries.jacobians[stamp][entry], ctx, vars)
    }

    pub(crate) fn jacobian_current_pairs(&self, stamp: usize, entry: usize) -> &[usize] {
        &self.current_dependencies.jacobians[stamp][entry]
    }

    pub(crate) fn jacobian_prior_currents(&self, stamp: usize, entry: usize) -> &[usize] {
        &self.current_dependencies.jacobian_prior_currents[stamp][entry]
    }

    pub(crate) fn jacobian_branch_unknowns(&self, stamp: usize, entry: usize) -> &[usize] {
        &self.current_dependencies.jacobian_branch_unknowns[stamp][entry]
    }

    pub(crate) fn run_reactive_jacobian(
        &self,
        stamp: usize,
        entry: usize,
        ctx: &EvalContext,
        vars: *const f64,
    ) -> f64 {
        self.run_value_entry(self.entries.reactive_jacobians[stamp][entry], ctx, vars)
    }

    pub(crate) fn reactive_jacobian_current_pairs(&self, stamp: usize, entry: usize) -> &[usize] {
        &self.current_dependencies.reactive_jacobians[stamp][entry]
    }

    pub(crate) fn reactive_jacobian_prior_currents(&self, stamp: usize, entry: usize) -> &[usize] {
        &self.current_dependencies.reactive_jacobian_prior_currents[stamp][entry]
    }

    pub(crate) fn reactive_jacobian_branch_unknowns(&self, stamp: usize, entry: usize) -> &[usize] {
        &self.current_dependencies.reactive_jacobian_branch_unknowns[stamp][entry]
    }

    pub(crate) fn run_noise_psd(&self, index: usize, ctx: &EvalContext, vars: *const f64) -> f64 {
        self.run_value_entry(self.entries.noise_psd[index], ctx, vars)
    }

    pub(crate) fn noise_psd_current_pairs(&self, index: usize) -> &[usize] {
        &self.current_dependencies.noise_psd[index]
    }

    pub(crate) fn noise_psd_prior_currents(&self, index: usize) -> &[usize] {
        &self.current_dependencies.noise_psd_prior_currents[index]
    }

    pub(crate) fn noise_psd_branch_unknowns(&self, index: usize) -> &[usize] {
        &self.current_dependencies.noise_psd_branch_unknowns[index]
    }

    pub(crate) fn run_noise_exponent(
        &self,
        index: usize,
        ctx: &EvalContext,
        vars: *const f64,
    ) -> Option<f64> {
        self.entries
            .noise_exponents
            .get(index)
            .and_then(|offset| offset.map(|offset| self.run_value_entry(offset, ctx, vars)))
    }

    pub(crate) fn has_noise_exponent_entry(&self, index: usize) -> bool {
        self.entries
            .noise_exponents
            .get(index)
            .is_some_and(Option::is_some)
    }

    pub(crate) fn noise_exponent_current_pairs(&self, index: usize) -> &[usize] {
        &self.current_dependencies.noise_exponents[index]
    }

    pub(crate) fn noise_exponent_prior_currents(&self, index: usize) -> &[usize] {
        &self.current_dependencies.noise_exponent_prior_currents[index]
    }

    pub(crate) fn noise_exponent_branch_unknowns(&self, index: usize) -> &[usize] {
        &self.current_dependencies.noise_exponent_branch_unknowns[index]
    }

    fn run_value_entry(&self, offset: CodeOffset, ctx: &EvalContext, vars: *const f64) -> f64 {
        // Safety: from_executable_image validated this offset is inside the
        // executable image owned by self, and the backend records it with the
        // ValueEntry ABI.
        let entry: ValueEntry = unsafe { std::mem::transmute(self.entry_ptr(offset)) };
        // Safety: callers provide pointers matching the native value-entry ABI.
        unsafe { entry(ctx as *const EvalContext, vars) }
    }

    pub fn chunk_count(&self) -> usize {
        self.stats.assignment_entry_points
    }

    pub fn native_stamp_count(&self) -> usize {
        self.stats.stamp_value_entry_points
    }

    pub fn plan_stats(&self) -> PlanStats {
        self.stats
    }
}

#[cfg(all(test, target_arch = "x86_64"))]
fn append_test_value_stub(bytes: &mut Vec<u8>, value: u32) -> CodeOffset {
    let offset = CodeOffset::new(bytes.len());
    // mov eax, imm32; cvtsi2sd xmm0, eax; ret
    bytes.push(0xB8);
    bytes.extend_from_slice(&value.to_le_bytes());
    bytes.extend_from_slice(&[0xF2, 0x0F, 0x2A, 0xC0, 0xC3]);
    offset
}

#[cfg(all(test, feature = "native", target_arch = "x86_64"))]
mod tests {
    use super::super::runtime::ExecutableMemory;
    use super::{
        CodeOffset, EvalContext, NativeEntryOffsets, NativeModel, NativeRequiredStorage,
        append_test_value_stub,
    };
    use std::sync::{Arc, Barrier};

    #[test]
    fn native_model_entry_points_are_not_optional() {
        let model = NativeModel::new_for_test(2, 1, vec![1], vec![0]);
        assert_eq!(model.chunk_count(), 1);
        assert_eq!(model.plan_stats().parameter_default_entry_points, 0);
        assert_eq!(model.plan_stats().static_condition_entry_points, 1);
        assert_eq!(model.native_stamp_count(), 1);
        assert_eq!(model.plan_stats().jacobian_entry_points, 1);
        assert_eq!(model.plan_stats().reactive_jacobian_entry_points, 0);
    }

    #[test]
    fn native_model_tracks_reactive_jacobian_entry_points_separately() {
        let model = NativeModel::new_for_test(2, 1, vec![1], vec![1]);

        assert_eq!(model.plan_stats().reactive_jacobian_entry_points, 1);
        assert_eq!(
            model.run_stamp_value(0, &empty_eval_context(), std::ptr::null()),
            1.0
        );
        assert_eq!(
            model.run_jacobian(0, 0, &empty_eval_context(), std::ptr::null()),
            2.0
        );
        assert_eq!(
            model.run_reactive_jacobian(0, 0, &empty_eval_context(), std::ptr::null()),
            3.0
        );
    }

    #[test]
    fn native_model_calls_entry_points_from_owned_image() {
        let mut bytes = vec![0xC3]; // assignment: ret
        let stamp_entry = append_test_value_stub(&mut bytes, 1);
        let jacobian_entry = append_test_value_stub(&mut bytes, 2);
        let reactive_jacobian_entry = append_test_value_stub(&mut bytes, 3);
        let image = ExecutableMemory::allocate(&bytes).expect("allocate native test image");
        let model = NativeModel::from_executable_image(
            0,
            0,
            image,
            NativeEntryOffsets {
                assignment: CodeOffset::new(0),
                parameter_defaults: vec![],
                static_conditions: vec![Some(stamp_entry)],
                stamp_values: vec![stamp_entry],
                jacobians: vec![vec![jacobian_entry]],
                reactive_jacobians: vec![vec![reactive_jacobian_entry]],
                noise_psd: vec![],
                noise_exponents: vec![],
            },
        )
        .expect("publish owned native model");

        let ctx = empty_eval_context();
        model.run_assignments(&ctx, std::ptr::null_mut());
        assert_eq!(
            model.run_static_condition(0, &ctx, std::ptr::null()),
            Some(1.0)
        );
        assert_eq!(model.run_stamp_value(0, &ctx, std::ptr::null()), 1.0);
        assert_eq!(model.run_jacobian(0, 0, &ctx, std::ptr::null()), 2.0);
        assert_eq!(
            model.run_reactive_jacobian(0, 0, &ctx, std::ptr::null()),
            3.0
        );
        assert_eq!(model.native_stamp_count(), 1);
        assert_eq!(model.plan_stats().jacobian_entry_points, 1);
        assert_eq!(model.plan_stats().reactive_jacobian_entry_points, 1);
    }

    #[test]
    fn native_model_shared_image_survives_parallel_entrypoint_calls() {
        let model = Arc::new(NativeModel::new_for_test(2, 1, vec![1], vec![1]));
        let barrier = Arc::new(Barrier::new(8));
        let mut handles = Vec::new();

        for _ in 0..8 {
            let model = Arc::clone(&model);
            let barrier = Arc::clone(&barrier);
            handles.push(std::thread::spawn(move || {
                let ctx = empty_eval_context();
                barrier.wait();
                for _ in 0..2048 {
                    model.run_assignments(&ctx, std::ptr::null_mut());
                    assert_eq!(model.run_stamp_value(0, &ctx, std::ptr::null()), 1.0);
                    assert_eq!(model.run_jacobian(0, 0, &ctx, std::ptr::null()), 2.0);
                    assert_eq!(
                        model.run_reactive_jacobian(0, 0, &ctx, std::ptr::null()),
                        3.0
                    );
                }
            }));
        }

        for handle in handles {
            handle.join().expect("native entrypoint worker completed");
        }
    }

    #[test]
    fn native_model_rejects_entry_offsets_outside_owned_image() {
        let image = ExecutableMemory::allocate(&[0xC3]).expect("allocate native test image");
        let error = NativeModel::from_executable_image(
            0,
            0,
            image,
            NativeEntryOffsets {
                assignment: CodeOffset::new(1),
                parameter_defaults: vec![],
                static_conditions: vec![],
                stamp_values: vec![],
                jacobians: vec![],
                reactive_jacobians: vec![],
                noise_psd: vec![],
                noise_exponents: vec![],
            },
        )
        .expect_err("entry at image length must be rejected");

        assert!(error.to_string().contains("entry offset"));
        assert!(error.to_string().contains("no interpreter fallback"));
    }

    #[test]
    fn native_model_rejects_parameter_default_shape_mismatch() {
        let image = ExecutableMemory::allocate(&[0xC3]).expect("allocate native test image");
        let error = NativeModel::from_executable_image(
            0,
            1,
            image,
            NativeEntryOffsets {
                assignment: CodeOffset::new(0),
                parameter_defaults: vec![],
                static_conditions: vec![],
                stamp_values: vec![],
                jacobians: vec![],
                reactive_jacobians: vec![],
                noise_psd: vec![],
                noise_exponents: vec![],
            },
        )
        .expect_err("missing parameter-default slot must be rejected");

        assert!(error.to_string().contains("parameter-default entry shape"));
        assert!(error.to_string().contains("no interpreter fallback"));
    }

    #[test]
    fn native_model_rejects_jacobian_stamp_shape_mismatch() {
        let image = ExecutableMemory::allocate(&[0xC3]).expect("allocate native test image");
        let error = NativeModel::from_executable_image(
            0,
            0,
            image,
            NativeEntryOffsets {
                assignment: CodeOffset::new(0),
                parameter_defaults: vec![],
                static_conditions: vec![None],
                stamp_values: vec![CodeOffset::new(0)],
                jacobians: vec![],
                reactive_jacobians: vec![vec![]],
                noise_psd: vec![],
                noise_exponents: vec![],
            },
        )
        .expect_err("jacobian table must match stamp table shape");

        assert!(error.to_string().contains("jacobian entry shape"));
        assert!(error.to_string().contains("no interpreter fallback"));
    }

    #[test]
    fn native_model_rejects_reactive_jacobian_stamp_shape_mismatch() {
        let image = ExecutableMemory::allocate(&[0xC3]).expect("allocate native test image");
        let error = NativeModel::from_executable_image(
            0,
            0,
            image,
            NativeEntryOffsets {
                assignment: CodeOffset::new(0),
                parameter_defaults: vec![],
                static_conditions: vec![None],
                stamp_values: vec![CodeOffset::new(0)],
                jacobians: vec![vec![]],
                reactive_jacobians: vec![],
                noise_psd: vec![],
                noise_exponents: vec![],
            },
        )
        .expect_err("reactive-jacobian table must match stamp table shape");

        assert!(error.to_string().contains("reactive-jacobian entry shape"));
        assert!(error.to_string().contains("no interpreter fallback"));
    }

    #[test]
    fn native_model_rejects_current_pair_dependency_invalid_for_terminal_count() {
        let mut bytes = vec![0xC3]; // assignment: ret
        let stamp_entry = append_test_value_stub(&mut bytes, 1);
        let image = ExecutableMemory::allocate(&bytes).expect("allocate native test image");
        let entries = NativeEntryOffsets {
            assignment: CodeOffset::new(0),
            parameter_defaults: vec![],
            static_conditions: vec![None],
            stamp_values: vec![stamp_entry],
            jacobians: vec![vec![]],
            reactive_jacobians: vec![vec![]],
            noise_psd: vec![],
            noise_exponents: vec![],
        };
        let mut dependencies = NativeModel::empty_current_dependencies(&entries);
        dependencies.stamp_values[0].push(3);

        let error = NativeModel::from_executable_image_with_dependencies(
            1,
            0,
            0,
            0,
            0,
            image,
            entries,
            dependencies,
            NativeRequiredStorage::default(),
        )
        .expect_err("invalid terminal-pair current dependency must be rejected");

        let message = error.to_string();
        assert!(
            message.contains("stamp value 0 terminal-pair current dependency 3"),
            "{message}"
        );
        assert!(message.contains("compiled terminal count 1"), "{message}");
        assert!(message.contains("no interpreter fallback"), "{message}");
    }

    #[test]
    fn native_model_rejects_branch_unknown_dependency_outside_compiled_count() {
        let mut bytes = vec![0xC3]; // assignment: ret
        let stamp_entry = append_test_value_stub(&mut bytes, 1);
        let image = ExecutableMemory::allocate(&bytes).expect("allocate native test image");
        let entries = NativeEntryOffsets {
            assignment: CodeOffset::new(0),
            parameter_defaults: vec![],
            static_conditions: vec![None],
            stamp_values: vec![stamp_entry],
            jacobians: vec![vec![]],
            reactive_jacobians: vec![vec![]],
            noise_psd: vec![],
            noise_exponents: vec![],
        };
        let mut dependencies = NativeModel::empty_current_dependencies(&entries);
        dependencies.stamp_value_branch_unknowns[0].push(1);

        let error = NativeModel::from_executable_image_with_dependencies(
            0,
            0,
            0,
            0,
            1,
            image,
            entries,
            dependencies,
            NativeRequiredStorage::default(),
        )
        .expect_err("out-of-range branch-current unknown dependency must be rejected");

        let message = error.to_string();
        assert!(
            message.contains("stamp value 0 branch-current unknown dependency 1"),
            "{message}"
        );
        assert!(
            message.contains("compiled branch-current unknown count 1"),
            "{message}"
        );
        assert!(message.contains("no interpreter fallback"), "{message}");
    }

    fn empty_eval_context() -> EvalContext {
        EvalContext {
            voltages: std::ptr::null(),
            internal_voltages: std::ptr::null(),
            params: std::ptr::null(),
            branch_currents: std::ptr::null(),
            branch_currents_len: 0,
            currents: std::ptr::null(),
            currents_len: 0,
            num_terminals: 0,
            port_connected: std::ptr::null(),
            port_connected_len: 0,
            temperature: 0.0,
            time: 0.0,
            timestep: 0.0,
            state_prev: std::ptr::null(),
            state_values: std::ptr::null_mut(),
            state_initialized: std::ptr::null_mut(),
            state_initialized_len: 0,
            lookup_tables: std::ptr::null(),
            lookup_tables_len: 0,
            laplace_filters: std::ptr::null_mut(),
            laplace_filters_len: 0,
            param_given: std::ptr::null(),
            param_given_len: 0,
            branch_unknowns: std::ptr::null(),
            analysis_type: 0,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
            transition_filters: std::ptr::null_mut(),
            transition_filters_len: 0,
            slew_filters: std::ptr::null_mut(),
            slew_filters_len: 0,
            delay_buffers: std::ptr::null_mut(),
            delay_buffers_len: 0,
            cross_detectors: std::ptr::null_mut(),
            cross_detectors_len: 0,
            state_prev_len: 0,
            state_values_len: 0,
        }
    }
}

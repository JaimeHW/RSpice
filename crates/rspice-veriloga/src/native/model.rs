use super::runtime::ExecutableMemory;
use super::{EvalContext, JitError, JitResult};

type AssignmentEntry = unsafe extern "C" fn(*const EvalContext, *mut f64);
type ValueEntry = unsafe extern "C" fn(*const EvalContext, *const f64) -> f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CodeOffset(usize);

impl CodeOffset {
    #[allow(dead_code)]
    pub(crate) fn new(offset: usize) -> Self {
        Self(offset)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NativeEntryOffsets {
    pub assignment: CodeOffset,
    pub stamp_values: Vec<CodeOffset>,
    pub jacobians: Vec<Vec<CodeOffset>>,
    pub reactive_jacobians: Vec<Vec<CodeOffset>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct NativeCurrentDependencies {
    pub stamp_values: Vec<Vec<usize>>,
    pub jacobians: Vec<Vec<Vec<usize>>>,
    pub reactive_jacobians: Vec<Vec<Vec<usize>>>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlanStats {
    pub assignment_entry_points: usize,
    pub stamp_value_entry_points: usize,
    pub jacobian_entry_points: usize,
    pub reactive_jacobian_entry_points: usize,
}

pub struct NativeModel {
    pub num_variables: usize,
    image: ExecutableMemory,
    entries: NativeEntryOffsets,
    current_dependencies: NativeCurrentDependencies,
    stats: PlanStats,
}

impl std::fmt::Debug for NativeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeModel")
            .field("num_variables", &self.num_variables)
            .field("image_len", &self.image.len())
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
        image: ExecutableMemory,
        entries: NativeEntryOffsets,
    ) -> JitResult<Self> {
        let current_dependencies = Self::empty_current_dependencies(&entries);
        Self::from_executable_image_with_dependencies(
            num_variables,
            image,
            entries,
            current_dependencies,
        )
    }

    pub(crate) fn from_executable_image_with_dependencies(
        num_variables: usize,
        image: ExecutableMemory,
        entries: NativeEntryOffsets,
        current_dependencies: NativeCurrentDependencies,
    ) -> JitResult<Self> {
        Self::validate_entry_offsets(&entries, image.len())?;
        Self::validate_current_dependencies(&entries, &current_dependencies)?;

        let jacobian_entry_points = entries.jacobians.iter().map(Vec::len).sum();
        let reactive_jacobian_entry_points = entries.reactive_jacobians.iter().map(Vec::len).sum();
        let stats = PlanStats {
            assignment_entry_points: 1,
            stamp_value_entry_points: entries.stamp_values.len(),
            jacobian_entry_points,
            reactive_jacobian_entry_points,
        };

        Ok(Self {
            num_variables,
            image,
            entries,
            current_dependencies,
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
        let entries = NativeEntryOffsets {
            assignment: CodeOffset::new(0),
            stamp_values: vec![stamp_entry; stamp_value_entry_points],
            jacobians: jacobian_entry_points
                .into_iter()
                .map(|count| vec![jacobian_entry; count])
                .collect(),
            reactive_jacobians: reactive_jacobian_entry_points
                .into_iter()
                .map(|count| vec![reactive_jacobian_entry; count])
                .collect(),
        };

        Self::from_executable_image(num_variables, image, entries)
            .expect("publish native test model")
    }

    #[allow(dead_code)]
    fn validate_entry_offsets(entries: &NativeEntryOffsets, image_len: usize) -> JitResult<()> {
        Self::validate_entry_offset(entries.assignment, image_len)?;
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
            stamp_values: vec![Vec::new(); entries.stamp_values.len()],
            jacobians: entries
                .jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
            reactive_jacobians: entries
                .reactive_jacobians
                .iter()
                .map(|entries| vec![Vec::new(); entries.len()])
                .collect(),
        }
    }

    fn validate_current_dependencies(
        entries: &NativeEntryOffsets,
        dependencies: &NativeCurrentDependencies,
    ) -> JitResult<()> {
        if dependencies.stamp_values.len() != entries.stamp_values.len()
            || dependencies.jacobians.len() != entries.jacobians.len()
            || dependencies.reactive_jacobians.len() != entries.reactive_jacobians.len()
            || dependencies
                .jacobians
                .iter()
                .zip(&entries.jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
            || dependencies
                .reactive_jacobians
                .iter()
                .zip(&entries.reactive_jacobians)
                .any(|(dependencies, entries)| dependencies.len() != entries.len())
        {
            return Err(JitError::InternalCompilerError {
                model: "native-model".into(),
                detail: "current dependency shape does not match native entry shape".into(),
            });
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

    pub(crate) fn run_stamp_value(&self, index: usize, ctx: &EvalContext, vars: *const f64) -> f64 {
        self.run_value_entry(self.entries.stamp_values[index], ctx, vars)
    }

    pub(crate) fn stamp_value_current_pairs(&self, index: usize) -> &[usize] {
        &self.current_dependencies.stamp_values[index]
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
    use super::{CodeOffset, EvalContext, NativeEntryOffsets, NativeModel, append_test_value_stub};

    #[test]
    fn native_model_entry_points_are_not_optional() {
        let model = NativeModel::new_for_test(2, 1, vec![1], vec![]);
        assert_eq!(model.chunk_count(), 1);
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
            image,
            NativeEntryOffsets {
                assignment: CodeOffset::new(0),
                stamp_values: vec![stamp_entry],
                jacobians: vec![vec![jacobian_entry]],
                reactive_jacobians: vec![vec![reactive_jacobian_entry]],
            },
        )
        .expect("publish owned native model");

        let ctx = empty_eval_context();
        model.run_assignments(&ctx, std::ptr::null_mut());
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
    fn native_model_rejects_entry_offsets_outside_owned_image() {
        let image = ExecutableMemory::allocate(&[0xC3]).expect("allocate native test image");
        let error = NativeModel::from_executable_image(
            0,
            image,
            NativeEntryOffsets {
                assignment: CodeOffset::new(1),
                stamp_values: vec![],
                jacobians: vec![],
                reactive_jacobians: vec![],
            },
        )
        .expect_err("entry at image length must be rejected");

        assert!(error.to_string().contains("entry offset"));
        assert!(error.to_string().contains("no interpreter fallback"));
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
            branch_unknowns: std::ptr::null(),
            analysis_type: 0,
            multiplicity: 1.0,
            zi_filters: std::ptr::null_mut(),
            zi_filters_len: 0,
        }
    }
}

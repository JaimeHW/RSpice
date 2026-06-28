use super::EvalContext;

pub type AssignmentFn = extern "C" fn(*const EvalContext, *mut f64);
pub type StampFn = extern "C" fn(*const EvalContext, *const f64) -> f64;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlanStats {
    pub assignment_entry_points: usize,
    pub stamp_value_entry_points: usize,
    pub jacobian_entry_points: usize,
}

pub struct NativeModel {
    pub num_variables: usize,
    assignment_fn: AssignmentFn,
    stamp_value_fns: Vec<StampFn>,
    jacobian_fns: Vec<Vec<StampFn>>,
    stats: PlanStats,
}

impl std::fmt::Debug for NativeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeModel")
            .field("num_variables", &self.num_variables)
            .field("stats", &self.stats)
            .finish()
    }
}

// Safety: NativeModel is immutable after construction. Function pointer
// lifetimes are owned by the native image that creates this table; the first
// foundation slice cannot construct NativeModel until that image owner lands.
unsafe impl Send for NativeModel {}
unsafe impl Sync for NativeModel {}

impl NativeModel {
    pub fn new_for_test(
        num_variables: usize,
        assignment_fn: AssignmentFn,
        stamp_value_fns: Vec<StampFn>,
        jacobian_fns: Vec<Vec<StampFn>>,
    ) -> Self {
        let jacobian_entry_points = jacobian_fns.iter().map(Vec::len).sum();
        let stats = PlanStats {
            assignment_entry_points: 1,
            stamp_value_entry_points: stamp_value_fns.len(),
            jacobian_entry_points,
        };
        Self {
            num_variables,
            assignment_fn,
            stamp_value_fns,
            jacobian_fns,
            stats,
        }
    }

    pub fn run_assignments(&self, ctx: &EvalContext, vars: *mut f64) {
        (self.assignment_fn)(ctx as *const EvalContext, vars);
    }

    pub fn stamp_value_fn(&self, index: usize) -> StampFn {
        self.stamp_value_fns[index]
    }

    pub fn jacobian_fn(&self, stamp: usize, entry: usize) -> StampFn {
        self.jacobian_fns[stamp][entry]
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

#[cfg(test)]
mod tests {
    use super::{EvalContext, NativeModel};

    extern "C" fn assign_noop(_ctx: *const EvalContext, _vars: *mut f64) {}
    extern "C" fn stamp_one(_ctx: *const EvalContext, _vars: *const f64) -> f64 {
        1.0
    }

    #[test]
    fn native_model_entry_points_are_not_optional() {
        let model =
            NativeModel::new_for_test(2, assign_noop, vec![stamp_one], vec![vec![stamp_one]]);
        assert_eq!(model.chunk_count(), 1);
        assert_eq!(model.native_stamp_count(), 1);
        assert_eq!(model.plan_stats().jacobian_entry_points, 1);
    }
}

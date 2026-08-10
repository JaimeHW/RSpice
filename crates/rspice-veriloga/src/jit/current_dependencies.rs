//! Dependency metadata shared by every executable JIT backend.

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct JitCurrentDependencies {
    pub assignment_current_pairs: Vec<usize>,
    pub assignment_prior_currents: Vec<usize>,
    pub assignment_branch_unknowns: Vec<usize>,
    pub post_assignment_current_pairs: Vec<usize>,
    pub post_assignment_prior_currents: Vec<usize>,
    pub post_assignment_branch_unknowns: Vec<usize>,
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

impl JitCurrentDependencies {
    /// Whether a fused driver that evaluates every stamp in one pass preserves
    /// contribution-current ordering for assignments and stamp values.
    ///
    /// A fused driver publishes each contribution as it goes, so an expression
    /// that reads a *prior* contribution's current only sees the right value if
    /// that contribution was evaluated earlier in the same pass. An assignment
    /// reading any prior current, or a stamp reading one at or after its own
    /// index, breaks that and forces the per-entry path.
    ///
    /// Shared by every executable backend: the browser and the machine
    /// backends must agree about which models are eligible, or they disagree
    /// about results rather than merely about speed.
    pub(crate) fn evaluation_kernel_order_safe(&self) -> bool {
        self.assignment_prior_currents.is_empty()
            && self
                .stamp_value_prior_currents
                .iter()
                .enumerate()
                .all(|(stamp, dependencies)| {
                    dependencies.iter().all(|dependency| *dependency < stamp)
                })
    }

    /// As [`Self::evaluation_kernel_order_safe`], additionally requiring that
    /// Jacobian entries read no contribution published after their own stamp.
    pub(crate) fn stamp_kernel_order_safe(&self) -> bool {
        self.evaluation_kernel_order_safe()
            && self
                .jacobian_prior_currents
                .iter()
                .enumerate()
                .all(|(stamp, entries)| {
                    entries
                        .iter()
                        .flatten()
                        .all(|dependency| *dependency <= stamp)
                })
    }
}

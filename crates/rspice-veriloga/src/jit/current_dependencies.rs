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

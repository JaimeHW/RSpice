//! Convergence quality of a simulation run: the aids it needed, the points
//! it force-accepted, and the circuit objects a failed solve was attributed
//! to.

use crate::Value;

//=============================================================================
// Attributed Convergence Failures
//=============================================================================

/// Which failure the engine attributed, named as the engine names it.
///
/// The prose a failure carries is written for a person reading a log. This
/// says the same thing in a form a frontend can act on, so a schematic can
/// mark the offending conductors instead of asking the author to read node
/// names out of a paragraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ConvergenceFailureClass {
    /// Nothing in the circuit conducts at DC to the named nodes, so no
    /// operating-point voltage is defined for them.
    NoDcPathToGround,
    /// An operating point was reached, but its bias at the named nodes
    /// survives only because of the simulator's nodal conditioning.
    ConditioningDependentBias,
    /// The assembled MNA system is singular: no equation constrains the
    /// named rows.
    SingularSystem,
    /// Newton-Raphson exhausted its iteration budget. The named nodes are
    /// the KCL equations left worst-violated at the abort iterate.
    NewtonNonConvergence,
}

/// Which MNA unknown a named site is, so a frontend knows what to look up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConvergenceSiteKind {
    /// A circuit node: rows below `num_nodes`, KCL equations.
    Node,
    /// A branch current unknown: rows at or above `num_nodes`.
    Branch,
}

/// One circuit object a failure was attributed to.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvergenceSite {
    /// The node or branch name as the assembled circuit spells it. When the
    /// circuit has no name for the row, this is `#N` with the 1-based
    /// position, matching what the failure's own prose shows.
    pub name: String,
    /// Whether `name` addresses a node or a branch current.
    pub kind: ConvergenceSiteKind,
    /// Scaled residual at this row where the class measures one, normalized
    /// so `1.0` sits exactly at the configured tolerance. `None` for classes
    /// that are structural rather than numerical.
    pub residual: Option<Value>,
}

/// The named circuit objects one failed solve fell on.
///
/// This is recorded beside the failure, never instead of it: the prose in
/// `SimulationError` is unchanged, and [`Self::failure_message`]
/// holds the exact rendering it was built for. A consumer must check
/// [`Self::describes`] against the error it actually received before using
/// the sites, because the engine records an attribution at the moment a
/// solve gives up and a later convergence aid may still rescue the run.
/// Without that check an attribution from a rescued attempt could be shown
/// against an unrelated later failure.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvergenceDiagnostic {
    /// Which failure this attributes.
    pub class: ConvergenceFailureClass,
    /// The named objects, worst first where the class ranks them.
    pub sites: Vec<ConvergenceSite>,
    /// Objects the engine measured but did not name, because attribution is
    /// capped at [`ConvergenceDiagnostic::MAX_NAMED_SITES`]. A frontend
    /// should say the list is partial rather than imply it is the whole set.
    pub elided_sites: usize,
    /// The exact `Display` rendering of the failure this was built for.
    pub failure_message: String,
}

impl ConvergenceDiagnostic {
    /// Upper bound on named sites. A shorted-out deck can float thousands of
    /// nodes; naming all of them is neither useful to read nor free to keep.
    pub const MAX_NAMED_SITES: usize = 32;

    /// Whether this attribution belongs to `rendered_error`.
    ///
    /// `contains` rather than equality because the engine wraps a failure's
    /// prose as it travels — a `.STEP` point prefixes the point it failed at,
    /// and the error type prefixes its own category. Those additions keep the
    /// recorded message as a substring; an unrelated failure does not.
    #[must_use]
    pub fn describes(&self, rendered_error: &str) -> bool {
        !self.failure_message.is_empty() && rendered_error.contains(&self.failure_message)
    }
}

//=============================================================================
// Convergence Quality Metrics
//=============================================================================

/// Detailed convergence quality metrics for a simulation
#[derive(Debug, Clone, Default)]
pub struct ConvergenceQuality {
    /// Total nonlinear Newton Jacobian assemblies across the analysis.
    /// Residual-only verification probes are deliberately excluded. A
    /// transient counts one per linearized Newton solve at each attempted
    /// timepoint, in addition to its operating point's assemblies.
    pub total_iterations: usize,
    /// Number of times GMIN stepping was used
    pub gmin_stepping_count: usize,
    /// Number of times source stepping was used
    pub source_stepping_count: usize,
    /// Number of Newton-converged transient points accepted despite LTE rejection.
    pub force_accepted_points: usize,
    /// Indices of force-accepted LTE-rejected points (for highlighting in plots).
    pub force_accepted_indices: Vec<usize>,
    /// Maximum residual across all converged points
    pub max_residual: Value,
    /// Average iterations per solve
    pub avg_iterations_per_solve: Value,
    /// Number of timestep reductions (transient only)
    pub timestep_reductions: usize,
    /// Number of LTE rejections (transient only)
    pub lte_rejections: usize,
    /// Compact-model evaluations skipped because `.OPTIONS BYPASS` found the
    /// device stationary. Zero whenever bypass is off, which is the default,
    /// and zero on a deck carrying no model family that implements it — so a
    /// run comparison showing no speedup can be told apart from one where the
    /// option never engaged.
    pub bypassed_device_evaluations: u64,
    /// The most recent failure the engine could attribute to named circuit
    /// objects. `None` for a run that never gave up on a solve, which is why
    /// a passing analysis costs nothing here.
    ///
    /// Present does not mean the run failed: a solve that gave up may still
    /// have been rescued by a convergence aid. Consumers must gate on
    /// [`ConvergenceDiagnostic::describes`] against the error they received.
    pub failure_diagnostic: Option<ConvergenceDiagnostic>,
}

impl ConvergenceQuality {
    /// Create a new quality tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Record GMIN stepping usage
    pub fn record_gmin_stepping(&mut self) {
        self.gmin_stepping_count += 1;
    }

    /// Record source stepping usage
    pub fn record_source_stepping(&mut self) {
        self.source_stepping_count += 1;
    }

    /// Record a force-accepted point
    pub fn record_force_accept(&mut self, point_index: usize) {
        self.force_accepted_points += 1;
        self.force_accepted_indices.push(point_index);
    }

    /// Record a timestep reduction
    pub fn record_timestep_reduction(&mut self) {
        self.timestep_reductions += 1;
    }

    /// Record a step rejected on local truncation error. It is a timestep
    /// reduction too, so it counts in both.
    #[doc(hidden)]
    pub fn record_lte_rejection(&mut self) {
        self.timestep_reductions += 1;
        self.lte_rejections += 1;
    }

    /// Record the circuit objects a solve gave up on.
    ///
    /// Last writer wins: the attribution that matters is the one belonging to
    /// the failure the caller ends up seeing, and that is always the most
    /// recent give-up. [`ConvergenceDiagnostic::describes`] is what proves the
    /// pairing; this only keeps the freshest candidate.
    pub fn record_failure_diagnostic(&mut self, diagnostic: ConvergenceDiagnostic) {
        self.failure_diagnostic = Some(diagnostic);
    }

    /// Calculate average iterations per solve
    pub fn finalize(&mut self, total_solves: usize) {
        if total_solves > 0 {
            self.avg_iterations_per_solve = self.total_iterations as Value / total_solves as Value;
        }
    }
}

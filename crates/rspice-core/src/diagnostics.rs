//! Simulation Diagnostics System
//!
//! Provides structured warnings, errors, and quality metrics for simulation runs.
//! This module enables:
//!
//! - Tracking convergence quality across all analysis types
//! - Collecting structured warnings instead of silent failures
//! - Recording force-accept events in transient analysis
//! - Providing detailed simulation quality reports
//!
//! ## Commercial-Grade Philosophy
//!
//! Production simulators never silently fail. Every potential issue is logged
//! and made available to the user for inspection. This module implements that
//! philosophy for RSpice.

use crate::Value;
use std::fmt;

//=============================================================================
// Diagnostic Levels
//=============================================================================

/// Severity level for simulation diagnostics
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DiagnosticLevel {
    /// Informational message (e.g., "Using GMIN stepping")
    Info,
    /// Warning that may affect accuracy (e.g., "Force-accepted LTE-rejected point")
    Warning,
    /// Error that indicates a problem (e.g., "Convergence failed for node X")
    Error,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Info => write!(f, "INFO"),
            DiagnosticLevel::Warning => write!(f, "WARNING"),
            DiagnosticLevel::Error => write!(f, "ERROR"),
        }
    }
}

//=============================================================================
// Diagnostic Categories
//=============================================================================

/// Category of diagnostic for filtering and grouping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagnosticCategory {
    /// Convergence-related issues
    Convergence,
    /// Timestep control issues
    Timestep,
    /// Matrix/solver issues
    Solver,
    /// Device model issues
    Device,
    /// Numerical issues (overflow, underflow)
    Numerical,
    /// Circuit topology issues
    Topology,
    /// General simulation issues
    General,
}

impl fmt::Display for DiagnosticCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticCategory::Convergence => write!(f, "CONVERGENCE"),
            DiagnosticCategory::Timestep => write!(f, "TIMESTEP"),
            DiagnosticCategory::Solver => write!(f, "SOLVER"),
            DiagnosticCategory::Device => write!(f, "DEVICE"),
            DiagnosticCategory::Numerical => write!(f, "NUMERICAL"),
            DiagnosticCategory::Topology => write!(f, "TOPOLOGY"),
            DiagnosticCategory::General => write!(f, "GENERAL"),
        }
    }
}

//=============================================================================
// Diagnostic Entry
//=============================================================================

/// A single diagnostic entry with full context
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Severity level
    pub level: DiagnosticLevel,
    /// Category for grouping
    pub category: DiagnosticCategory,
    /// Human-readable message
    pub message: String,
    /// Optional time point (for transient analysis)
    pub time: Option<Value>,
    /// Optional node or element name
    pub location: Option<String>,
    /// Optional additional context/data
    pub details: Option<String>,
}

impl Diagnostic {
    /// Create a new diagnostic
    pub fn new(
        level: DiagnosticLevel,
        category: DiagnosticCategory,
        message: impl Into<String>,
    ) -> Self {
        Self {
            level,
            category,
            message: message.into(),
            time: None,
            location: None,
            details: None,
        }
    }

    /// Create an info diagnostic
    pub fn info(category: DiagnosticCategory, message: impl Into<String>) -> Self {
        Self::new(DiagnosticLevel::Info, category, message)
    }

    /// Create a warning diagnostic
    pub fn warning(category: DiagnosticCategory, message: impl Into<String>) -> Self {
        Self::new(DiagnosticLevel::Warning, category, message)
    }

    /// Create an error diagnostic
    pub fn error(category: DiagnosticCategory, message: impl Into<String>) -> Self {
        Self::new(DiagnosticLevel::Error, category, message)
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}] {}", self.level, self.category, self.message)?;
        if let Some(time) = self.time {
            write!(f, " (t={:.3e}s)", time)?;
        }
        if let Some(ref loc) = self.location {
            write!(f, " at {}", loc)?;
        }
        if let Some(ref details) = self.details {
            write!(f, " - {}", details)?;
        }
        Ok(())
    }
}

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
/// [`crate::SimulationError`] is unchanged, and [`Self::failure_message`]
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
    /// Residual-only verification probes are deliberately excluded.
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

    /// Check if any quality issues were detected
    pub fn has_issues(&self) -> bool {
        self.force_accepted_points > 0
            || self.gmin_stepping_count > 0
            || self.source_stepping_count > 0
    }

    /// Get a quality summary string
    pub fn summary(&self) -> String {
        let mut parts = Vec::new();

        if self.force_accepted_points > 0 {
            parts.push(format!(
                "{} force-accepted points",
                self.force_accepted_points
            ));
        }
        if self.gmin_stepping_count > 0 {
            parts.push(format!("{} GMIN stepping uses", self.gmin_stepping_count));
        }
        if self.source_stepping_count > 0 {
            parts.push(format!(
                "{} source stepping uses",
                self.source_stepping_count
            ));
        }

        if parts.is_empty() {
            "Clean convergence".to_string()
        } else {
            parts.join(", ")
        }
    }
}

//=============================================================================
// Simulation Diagnostics Container
//=============================================================================

/// Container for all diagnostics collected during simulation
#[derive(Debug, Clone, Default)]
pub struct SimulationDiagnostics {
    /// All diagnostic entries
    diagnostics: Vec<Diagnostic>,
    /// Convergence quality metrics
    pub convergence: ConvergenceQuality,
}

impl SimulationDiagnostics {
    /// Create a new diagnostics container
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a diagnostic entry
    pub fn add(&mut self, diagnostic: Diagnostic) {
        // Also log to the logging system
        match diagnostic.level {
            DiagnosticLevel::Info => log::info!("{}", diagnostic),
            DiagnosticLevel::Warning => log::warn!("{}", diagnostic),
            DiagnosticLevel::Error => log::error!("{}", diagnostic),
        }
        self.diagnostics.push(diagnostic);
    }

    /// Add an info diagnostic
    pub fn info(&mut self, category: DiagnosticCategory, message: impl Into<String>) {
        self.add(Diagnostic::info(category, message));
    }

    /// Add a warning diagnostic
    pub fn warn(&mut self, category: DiagnosticCategory, message: impl Into<String>) {
        self.add(Diagnostic::warning(category, message));
    }

    /// Add an error diagnostic
    pub fn error(&mut self, category: DiagnosticCategory, message: impl Into<String>) {
        self.add(Diagnostic::error(category, message));
    }

    /// Get all diagnostics
    pub fn all(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Get diagnostics filtered by level
    pub fn by_level(&self, level: DiagnosticLevel) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.level == level)
            .collect()
    }

    /// Get all warnings
    pub fn warnings(&self) -> Vec<&Diagnostic> {
        self.by_level(DiagnosticLevel::Warning)
    }

    /// Get all errors
    pub fn errors(&self) -> Vec<&Diagnostic> {
        self.by_level(DiagnosticLevel::Error)
    }

    /// Check if any errors were generated
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Error)
    }

    /// Get total count of diagnostics
    pub fn count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Clear all diagnostics (for reuse)
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.convergence = ConvergenceQuality::new();
    }

    /// Merge another diagnostics container into this one
    pub fn merge(&mut self, other: SimulationDiagnostics) {
        self.diagnostics.extend(other.diagnostics);
        // Merge convergence metrics
        self.convergence.total_iterations += other.convergence.total_iterations;
        self.convergence.gmin_stepping_count += other.convergence.gmin_stepping_count;
        self.convergence.source_stepping_count += other.convergence.source_stepping_count;
        self.convergence.force_accepted_points += other.convergence.force_accepted_points;
        self.convergence
            .force_accepted_indices
            .extend(other.convergence.force_accepted_indices);
        if other.convergence.max_residual > self.convergence.max_residual {
            self.convergence.max_residual = other.convergence.max_residual;
        }
        self.convergence.timestep_reductions += other.convergence.timestep_reductions;
        self.convergence.lte_rejections += other.convergence.lte_rejections;
        // The merged-in container ran later, so its attribution is the fresher
        // one; an absent attribution never erases one that was recorded.
        if other.convergence.failure_diagnostic.is_some() {
            self.convergence.failure_diagnostic = other.convergence.failure_diagnostic;
        }
    }

    /// Generate a summary report
    pub fn report(&self) -> String {
        let mut lines = Vec::new();

        lines.push("=== Simulation Diagnostics Report ===".to_string());
        lines.push(String::new());

        // Convergence summary
        lines.push(format!("Convergence: {}", self.convergence.summary()));
        lines.push(format!(
            "  Total iterations: {}",
            self.convergence.total_iterations
        ));
        if self.convergence.avg_iterations_per_solve > 0.0 {
            lines.push(format!(
                "  Avg iterations/solve: {:.1}",
                self.convergence.avg_iterations_per_solve
            ));
        }

        // Issue counts
        let warning_count = self.warnings().len();
        let error_count = self.errors().len();

        if warning_count > 0 || error_count > 0 {
            lines.push(String::new());
            lines.push(format!(
                "Issues: {} warnings, {} errors",
                warning_count, error_count
            ));

            for diag in &self.diagnostics {
                if diag.level >= DiagnosticLevel::Warning {
                    lines.push(format!("  {}", diag));
                }
            }
        }

        lines.join("\n")
    }
}

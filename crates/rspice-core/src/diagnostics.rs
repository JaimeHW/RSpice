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
    /// Warning that may affect accuracy (e.g., "Force-accepted non-converged point")
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

    /// Add time context
    pub fn at_time(mut self, time: Value) -> Self {
        self.time = Some(time);
        self
    }

    /// Add location context (node name, element name)
    pub fn at_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Add additional details
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
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
// Convergence Quality Metrics
//=============================================================================

/// Detailed convergence quality metrics for a simulation
#[derive(Debug, Clone, Default)]
pub struct ConvergenceQuality {
    /// Total Newton-Raphson iterations across all solves
    pub total_iterations: usize,
    /// Number of times GMIN stepping was used
    pub gmin_stepping_count: usize,
    /// Number of times source stepping was used
    pub source_stepping_count: usize,
    /// Number of force-accepted non-converged points (transient only)
    pub force_accepted_points: usize,
    /// Indices of force-accepted points (for highlighting in plots)
    pub force_accepted_indices: Vec<usize>,
    /// Maximum residual across all converged points
    pub max_residual: Value,
    /// Average iterations per solve
    pub avg_iterations_per_solve: Value,
    /// Number of timestep reductions (transient only)
    pub timestep_reductions: usize,
    /// Number of LTE rejections (transient only)
    pub lte_rejections: usize,
}

impl ConvergenceQuality {
    /// Create a new quality tracker
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a successful solve
    pub fn record_solve(&mut self, iterations: usize, residual: Value) {
        self.total_iterations += iterations;
        if residual > self.max_residual {
            self.max_residual = residual;
        }
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

    /// Record an LTE rejection
    pub fn record_lte_rejection(&mut self) {
        self.lte_rejections += 1;
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

    /// Get diagnostics filtered by category
    pub fn by_category(&self, category: DiagnosticCategory) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.category == category)
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

    /// Check if any warnings were generated
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.level == DiagnosticLevel::Warning)
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


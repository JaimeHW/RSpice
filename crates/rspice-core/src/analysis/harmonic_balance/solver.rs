//! Harmonic Balance Newton Solver
//!
//! Core solver for Harmonic Balance analysis using Newton-Raphson iteration.
//! Solves the frequency-domain circuit equations: G*X + jω*C*X + F_NL(X) = I_S

#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
use super::config::HbConfig;
use super::fft::HbFft;
use super::result::{HbResult, SpectralBranchCurrent, SpectralVoltage};
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
#[cfg(feature = "veriloga")]
use crate::device::veriloga::VerilogADevice;
use num_complex::Complex64;

mod dc;
mod devices;
mod krylov;
mod linear;
mod linear_algebra;
mod newton;
mod nonlinear_api;
mod periodic_ac;
mod result_builder;

pub use periodic_ac::{PeriodicAcExcitation, PeriodicNoiseSource};

/// Conductance used to model an inductor as a DC short across every solve
/// path (full-spectrum residual, Jacobian, DC seed, linear solve). One value
/// everywhere keeps the operating point and the seed solving the same
/// circuit.
pub(crate) const DC_SHORT_CONDUCTANCE: Value = 1e6;

/// Return the DC-short surrogate admittance for one signed inductor-matrix
/// entry. The magnitude of an authored inductance is irrelevant at DC, but
/// the sign still carries the two-terminal topology: diagonal entries are
/// positive and off-diagonal entries are negative. Dropping that sign turns
/// `[+L, -L; -L, +L]` into an all-positive matrix and enforces `V+ = -V-`
/// instead of the physical short-circuit constraint `V+ = V-`.
#[inline]
fn inductor_dc_short_admittance(signed_inductance: Value) -> Value {
    signed_inductance.signum() * DC_SHORT_CONDUCTANCE
}

/// Whether the nonlinear HB kernel may construct a DC operating-point seed
/// before its first full frequency-domain Newton step.
///
/// This is crate-private because netlist semantics are resolved by the engine;
/// direct solver clients retain the historical seeded behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HbDcSeedPolicy {
    Enabled,
    Disabled,
}

/// Error types specific to Harmonic Balance solver
#[derive(Debug, Clone)]
pub enum HbError {
    /// The caller requested cooperative cancellation.
    Aborted,
    /// Newton iteration did not converge
    ConvergenceFailed { iterations: usize, residual: Value },
    /// Matrix is singular
    SingularMatrix,
    /// Invalid circuit configuration
    InvalidCircuit(String),
    /// FFT operation failed
    FftError(String),
}

impl std::fmt::Display for HbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Aborted => write!(f, "Harmonic-balance solve aborted by user"),
            Self::ConvergenceFailed {
                iterations,
                residual,
            } => {
                write!(
                    f,
                    "HB convergence failed after {} iterations (residual: {:.3e})",
                    iterations, residual
                )
            }
            Self::SingularMatrix => write!(f, "Singular Jacobian matrix"),
            Self::InvalidCircuit(msg) => write!(f, "Invalid circuit: {}", msg),
            Self::FftError(msg) => write!(f, "FFT error: {}", msg),
        }
    }
}

impl std::error::Error for HbError {}

/// Harmonic Balance solver state
#[derive(Debug)]
pub struct HbSolverState {
    /// Spectral voltage solution, indexed `[node][harmonic]`.
    pub x: Vec<Vec<Complex64>>,

    /// Residual vector, indexed `[node][harmonic]`.
    pub residual: Vec<Vec<Complex64>>,

    /// Per-row current scale, indexed `[node][harmonic]`: the sum of the magnitudes of
    /// every individual current contribution into the row, accumulated
    /// alongside the residual. Convergence is judged per row against
    /// abstol + reltol * scale (the SPICE KCL criterion), which a global
    /// norm cannot do: a microamp imbalance at a megohm node hides under
    /// the norm of a circuit whose stiff rows carry amps.
    pub residual_scale: Vec<Vec<Value>>,

    /// Current residual norm
    pub residual_norm: Value,

    /// Number of iterations
    pub iteration: usize,

    /// Total harmonic Newton iterations accumulated across every phase of
    /// the convergence strategy (never reset by the ladder), for honest
    /// convergence-cost reporting.
    pub total_iterations: usize,

    /// Converged flag
    pub converged: bool,

    /// Spectra for branch-current unknowns retained by an actual MNA solve.
    /// Rows are in the solver's canonical exact-MNA branch order.
    pub mna_branch_currents: Vec<Vec<Complex64>>,

    /// KVL residual spectra aligned with `mna_branch_currents`.
    pub(crate) mna_branch_residual: Vec<Vec<Complex64>>,

    /// Per-row voltage scale for the KVL convergence certificate.
    pub(crate) mna_branch_residual_scale: Vec<Vec<Value>>,
}

impl HbSolverState {
    /// Create new solver state
    pub fn new(num_nodes: usize, num_harmonics: usize) -> Self {
        Self {
            x: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            residual: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            residual_scale: vec![vec![0.0; num_harmonics + 1]; num_nodes],
            residual_norm: f64::INFINITY,
            iteration: 0,
            total_iterations: 0,
            converged: false,
            mna_branch_currents: Vec::new(),
            mna_branch_residual: Vec::new(),
            mna_branch_residual_scale: Vec::new(),
        }
    }

    /// Fallibly allocate or validate the exact-MNA branch portion of this
    /// state without changing its public node-only construction contract.
    ///
    /// Existing branch currents are preserved so an authenticated operating
    /// point can enter a dependent solve. Empty residual workspaces are
    /// allocated transactionally; any nonempty incompatible shape fails
    /// closed instead of truncating or silently reordering state.
    pub(crate) fn try_prepare_mna_branches(
        &mut self,
        num_branches: usize,
        num_harmonics: usize,
    ) -> Result<(), HbError> {
        let width = num_harmonics.checked_add(1).ok_or_else(|| {
            HbError::InvalidCircuit(
                "HB branch-state harmonic width exceeds this platform".to_string(),
            )
        })?;
        let validate_complex_rows =
            |label: &str, rows: &[Vec<Complex64>], allow_empty: bool| -> Result<(), HbError> {
                if allow_empty && rows.is_empty() {
                    return Ok(());
                }
                if rows.len() != num_branches {
                    return Err(HbError::InvalidCircuit(format!(
                        "HB {label} has {} branch rows; expected {num_branches}",
                        rows.len()
                    )));
                }
                for (branch, row) in rows.iter().enumerate() {
                    if row.len() != width {
                        return Err(HbError::InvalidCircuit(format!(
                            "HB {label} branch {branch} has {} harmonics; expected {width}",
                            row.len()
                        )));
                    }
                    if row
                        .iter()
                        .any(|value| !value.re.is_finite() || !value.im.is_finite())
                    {
                        return Err(HbError::InvalidCircuit(format!(
                            "HB {label} branch {branch} contains a non-finite value"
                        )));
                    }
                    if row.first().is_some_and(|value| value.im != 0.0) {
                        return Err(HbError::InvalidCircuit(format!(
                            "HB {label} branch {branch} has a nonzero imaginary DC component"
                        )));
                    }
                }
                Ok(())
            };
        validate_complex_rows("MNA current state", &self.mna_branch_currents, true)?;
        validate_complex_rows("MNA residual state", &self.mna_branch_residual, true)?;
        if !self.mna_branch_residual_scale.is_empty() {
            if self.mna_branch_residual_scale.len() != num_branches {
                return Err(HbError::InvalidCircuit(format!(
                    "HB MNA residual scale has {} branch rows; expected {num_branches}",
                    self.mna_branch_residual_scale.len()
                )));
            }
            for (branch, row) in self.mna_branch_residual_scale.iter().enumerate() {
                if row.len() != width {
                    return Err(HbError::InvalidCircuit(format!(
                        "HB MNA residual scale branch {branch} has {} harmonics; expected {width}",
                        row.len()
                    )));
                }
                if row.iter().any(|value| !value.is_finite() || *value < 0.0) {
                    return Err(HbError::InvalidCircuit(format!(
                        "HB MNA residual scale branch {branch} contains an invalid value"
                    )));
                }
            }
        }

        let allocate_complex_rows = || -> Result<Vec<Vec<Complex64>>, HbError> {
            let mut rows = Vec::new();
            rows.try_reserve_exact(num_branches).map_err(|error| {
                HbError::InvalidCircuit(format!("HB MNA branch-row allocation failed: {error}"))
            })?;
            for _ in 0..num_branches {
                let mut row = Vec::new();
                row.try_reserve_exact(width).map_err(|error| {
                    HbError::InvalidCircuit(format!(
                        "HB MNA branch-spectrum allocation failed: {error}"
                    ))
                })?;
                row.resize(width, Complex64::new(0.0, 0.0));
                rows.push(row);
            }
            Ok(rows)
        };
        let allocate_scale_rows = || -> Result<Vec<Vec<Value>>, HbError> {
            let mut rows = Vec::new();
            rows.try_reserve_exact(num_branches).map_err(|error| {
                HbError::InvalidCircuit(format!(
                    "HB MNA branch-scale row allocation failed: {error}"
                ))
            })?;
            for _ in 0..num_branches {
                let mut row = Vec::new();
                row.try_reserve_exact(width).map_err(|error| {
                    HbError::InvalidCircuit(format!(
                        "HB MNA branch-scale allocation failed: {error}"
                    ))
                })?;
                row.resize(width, 0.0);
                rows.push(row);
            }
            Ok(rows)
        };

        let currents = self
            .mna_branch_currents
            .is_empty()
            .then(allocate_complex_rows)
            .transpose()?;
        let residual = self
            .mna_branch_residual
            .is_empty()
            .then(allocate_complex_rows)
            .transpose()?;
        let residual_scale = self
            .mna_branch_residual_scale
            .is_empty()
            .then(allocate_scale_rows)
            .transpose()?;
        if let Some(rows) = currents {
            self.mna_branch_currents = rows;
        }
        if let Some(rows) = residual {
            self.mna_branch_residual = rows;
        }
        if let Some(rows) = residual_scale {
            self.mna_branch_residual_scale = rows;
        }
        Ok(())
    }

    /// Compute residual norm (L2 over all nodes and harmonics)
    pub fn compute_residual_norm(&mut self) {
        // Accumulating squared magnitudes can overflow even when every
        // residual component is finite (for example, two values near
        // sqrt(f64::MAX)). `hypot` performs a scale-safe L2 accumulation and
        // still propagates non-finite input to a non-finite diagnostic norm.
        self.residual_norm = self
            .residual
            .iter()
            .chain(self.mna_branch_residual.iter())
            .flat_map(|node| node.iter())
            .fold(0.0, |norm, value| norm.hypot(value.re).hypot(value.im));
    }

    /// SPICE-style per-row KCL convergence: every residual entry must
    /// satisfy |res| <= abstol + reltol * (sum of |contribution| into the
    /// row), using the scale accumulated during residual assembly.
    pub fn rows_converged(&self, reltol: Value, abstol: Value) -> bool {
        self.rows_converged_with_branch_tolerances(reltol, abstol, abstol)
    }

    pub(crate) fn rows_converged_with_branch_tolerances(
        &self,
        reltol: Value,
        current_abstol: Value,
        voltage_abstol: Value,
    ) -> bool {
        if !reltol.is_finite()
            || reltol < 0.0
            || !current_abstol.is_finite()
            || current_abstol < 0.0
            || !voltage_abstol.is_finite()
            || voltage_abstol < 0.0
        {
            return false;
        }
        let current_rows_converged = residual_rows_converged(
            &self.x,
            &self.residual,
            &self.residual_scale,
            reltol,
            current_abstol,
            false,
        );
        current_rows_converged
            && residual_rows_converged(
                &self.mna_branch_currents,
                &self.mna_branch_residual,
                &self.mna_branch_residual_scale,
                reltol,
                voltage_abstol,
                false,
            )
    }

    /// Per-row KCL convergence restricted to the DC (k = 0) entries, for
    /// the DC operating-point pre-solve which only assembles harmonic 0.
    pub fn dc_rows_converged(&self, reltol: Value, abstol: Value) -> bool {
        self.dc_rows_converged_with_branch_tolerances(reltol, abstol, abstol)
    }

    /// DC convergence certificate with dimensionally distinct KCL-current and
    /// KVL-voltage absolute tolerances.
    pub(crate) fn dc_rows_converged_with_branch_tolerances(
        &self,
        reltol: Value,
        current_abstol: Value,
        voltage_abstol: Value,
    ) -> bool {
        if !reltol.is_finite() || reltol < 0.0 {
            return false;
        }
        if !current_abstol.is_finite()
            || current_abstol < 0.0
            || !voltage_abstol.is_finite()
            || voltage_abstol < 0.0
        {
            return false;
        }
        let current_rows_converged = residual_rows_converged(
            &self.x,
            &self.residual,
            &self.residual_scale,
            reltol,
            current_abstol,
            true,
        );
        current_rows_converged
            && residual_rows_converged(
                &self.mna_branch_currents,
                &self.mna_branch_residual,
                &self.mna_branch_residual_scale,
                reltol,
                voltage_abstol,
                true,
            )
    }

    /// Dimensionless worst-row residual certificate shared by DC and
    /// full-spectrum Newton line searches. KCL and KVL rows use independent
    /// absolute tolerances, so amperes and volts are never added or dotted.
    pub(crate) fn certificate_merit(
        &self,
        reltol: Value,
        current_abstol: Value,
        voltage_abstol: Value,
        dc_only: bool,
    ) -> Result<Value, HbError> {
        if !reltol.is_finite()
            || reltol < 0.0
            || !current_abstol.is_finite()
            || current_abstol <= 0.0
            || !voltage_abstol.is_finite()
            || voltage_abstol <= 0.0
        {
            return Err(HbError::InvalidCircuit(
                "HB residual-certificate tolerances must be finite and positive".to_string(),
            ));
        }
        let current_merit = residual_rows_merit(
            "KCL-current",
            &self.x,
            &self.residual,
            &self.residual_scale,
            reltol,
            current_abstol,
            dc_only,
        )?;
        let voltage_merit = residual_rows_merit(
            "KVL-voltage",
            &self.mna_branch_currents,
            &self.mna_branch_residual,
            &self.mna_branch_residual_scale,
            reltol,
            voltage_abstol,
            dc_only,
        )?;
        Ok(current_merit.max(voltage_merit))
    }

    /// Total number of spectral coordinates, or `usize::MAX` when publicly
    /// mutable state is malformed or cannot be represented. Resource and
    /// transport authorization must use the fallible internal qualifier.
    pub fn total_unknowns(&self) -> usize {
        self.try_total_unknowns().unwrap_or(usize::MAX)
    }

    /// Fallible coordinate count for resource authorization and transport
    /// validation. Unlike `total_unknowns`, this never treats a saturated
    /// value as evidence that a state fits a caller's resource budget.
    pub(crate) fn try_total_unknowns(&self) -> Result<usize, HbError> {
        if self.x.is_empty() && self.mna_branch_currents.is_empty() {
            return Ok(0);
        }
        let width = self.x.first().map(|row| row.len()).unwrap_or(0);
        if width == 0
            || self.x.iter().any(|row| row.len() != width)
            || self
                .mna_branch_currents
                .iter()
                .any(|row| row.len() != width)
        {
            return Err(HbError::InvalidCircuit(
                "HB solver state is not a nonempty rectangular spectral grid".to_string(),
            ));
        }
        self.x
            .len()
            .checked_add(self.mna_branch_currents.len())
            .and_then(|rows| rows.checked_mul(width))
            .ok_or_else(|| {
                HbError::InvalidCircuit(
                    "HB solver-state coordinate count exceeds this platform".to_string(),
                )
            })
    }
}

#[inline]
fn residual_entry_converged(
    residual: Complex64,
    scale: Value,
    reltol: Value,
    abstol: Value,
) -> bool {
    if !residual.re.is_finite() || !residual.im.is_finite() || !scale.is_finite() || scale < 0.0 {
        return false;
    }
    let tolerance = abstol + reltol * scale;
    tolerance.is_finite() && residual.norm() <= tolerance
}

fn residual_rows_converged(
    solution: &[Vec<Complex64>],
    residual: &[Vec<Complex64>],
    scale: &[Vec<Value>],
    reltol: Value,
    abstol: Value,
    dc_only: bool,
) -> bool {
    if solution.len() != residual.len() || residual.len() != scale.len() {
        return false;
    }
    solution
        .iter()
        .zip(residual)
        .zip(scale)
        .all(|((solution_row, residual_row), scale_row)| {
            if solution_row.is_empty()
                || solution_row.len() != residual_row.len()
                || residual_row.len() != scale_row.len()
            {
                return false;
            }
            if dc_only {
                residual_entry_converged(residual_row[0], scale_row[0], reltol, abstol)
            } else {
                residual_row
                    .iter()
                    .zip(scale_row)
                    .all(|(&entry, &entry_scale)| {
                        residual_entry_converged(entry, entry_scale, reltol, abstol)
                    })
            }
        })
}

fn residual_rows_merit(
    label: &str,
    solution: &[Vec<Complex64>],
    residual: &[Vec<Complex64>],
    scale: &[Vec<Value>],
    reltol: Value,
    abstol: Value,
    dc_only: bool,
) -> Result<Value, HbError> {
    if solution.len() != residual.len() || residual.len() != scale.len() {
        return Err(HbError::InvalidCircuit(format!(
            "HB {label} certificate row cardinality is inconsistent"
        )));
    }
    let mut merit: Value = 0.0;
    for (row, ((solution_row, residual_row), scale_row)) in
        solution.iter().zip(residual).zip(scale).enumerate()
    {
        if solution_row.is_empty()
            || solution_row.len() != residual_row.len()
            || residual_row.len() != scale_row.len()
        {
            return Err(HbError::InvalidCircuit(format!(
                "HB {label} certificate row {row} has an inconsistent harmonic width"
            )));
        }
        let width = if dc_only { 1 } else { residual_row.len() };
        for harmonic in 0..width {
            let residual = residual_row[harmonic];
            let scale = scale_row[harmonic];
            if !residual.re.is_finite()
                || !residual.im.is_finite()
                || !scale.is_finite()
                || scale < 0.0
            {
                return Err(HbError::InvalidCircuit(format!(
                    "HB {label} certificate row {row}, harmonic {harmonic} is non-finite"
                )));
            }
            if harmonic == 0 && residual.im != 0.0 {
                return Err(HbError::InvalidCircuit(format!(
                    "HB {label} certificate row {row} has a nonzero imaginary DC residual"
                )));
            }
            let tolerance = abstol + reltol * scale;
            if !tolerance.is_finite() || tolerance <= 0.0 {
                return Err(HbError::InvalidCircuit(format!(
                    "HB {label} certificate row {row}, harmonic {harmonic} has an invalid tolerance"
                )));
            }
            let ratio = residual.norm() / tolerance;
            merit = merit.max(if ratio.is_finite() { ratio } else { Value::MAX });
        }
    }
    Ok(merit)
}

#[cfg(test)]
mod solver_state_qualification_tests {
    use super::*;

    #[test]
    fn residual_norm_remains_finite_for_finite_large_components() {
        let mut state = HbSolverState::new(1, 1);
        state.residual[0][0] = Complex64::new(1.0e308, 0.0);
        state.residual[0][1] = Complex64::new(0.0, 1.0e308);

        state.compute_residual_norm();

        assert!(state.residual_norm.is_finite());
        assert!((state.residual_norm / 1.0e308 - 2.0_f64.sqrt()).abs() <= 4.0e-15);
    }

    #[test]
    fn row_certificate_rejects_nonfinite_residuals_scales_and_tolerances() {
        let mut state = HbSolverState::new(1, 0);
        state.residual[0][0] = Complex64::new(1.0, 0.0);
        state.residual_scale[0][0] = Value::INFINITY;
        assert!(!state.rows_converged(1.0, Value::INFINITY));
        assert!(!state.dc_rows_converged(1.0, Value::INFINITY));

        state.residual[0][0] = Complex64::new(Value::INFINITY, 0.0);
        state.residual_scale[0][0] = 1.0;
        assert!(!state.rows_converged(1.0, 0.0));
        assert!(!state.dc_rows_converged(1.0, 0.0));

        state.residual[0][0] = Complex64::new(0.0, 0.0);
        assert!(!state.rows_converged(Value::NAN, 0.0));
        assert!(!state.dc_rows_converged(-1.0, 0.0));
    }

    #[test]
    fn branch_state_preparation_and_certificates_are_shape_strict() {
        let mut state = HbSolverState::new(1, 1);
        state
            .try_prepare_mna_branches(1, 1)
            .expect("one exact branch row is allocated");
        assert_eq!(state.try_total_unknowns().unwrap(), 4);
        assert_eq!(
            state
                .certificate_merit(1.0e-3, 1.0e-12, 1.0e-6, false)
                .unwrap(),
            0.0
        );
        assert!(state.rows_converged_with_branch_tolerances(1.0e-3, 1.0e-12, 1.0e-6));

        state.mna_branch_residual_scale[0].pop();
        assert!(!state.rows_converged_with_branch_tolerances(1.0e-3, 1.0e-12, 1.0e-6));
        assert!(
            state
                .certificate_merit(1.0e-3, 1.0e-12, 1.0e-6, false)
                .is_err()
        );
    }

    #[test]
    fn branch_preparation_rejects_nonreal_dc_current_evidence() {
        let mut state = HbSolverState::new(1, 1);
        state.mna_branch_currents =
            vec![vec![Complex64::new(0.0, 1.0e-30), Complex64::new(0.0, 0.0)]];
        let error = state
            .try_prepare_mna_branches(1, 1)
            .expect_err("harmonic zero must be exactly real");
        assert!(error.to_string().contains("imaginary DC"), "{error}");
    }
}

/// Voltage source branch for MNA
///
/// In Modified Nodal Analysis, voltage sources require branch current
/// variables to properly enforce voltage constraints.
#[derive(Debug, Clone, PartialEq)]
pub struct VoltageSourceBranch {
    /// Positive terminal node (1-indexed, 0 = ground)
    pub node_pos: usize,
    /// Negative terminal node (1-indexed, 0 = ground)
    pub node_neg: usize,
    /// Branch current variable index
    pub branch_idx: usize,
    /// DC voltage value
    pub dc_voltage: Value,
    /// AC harmonic spectrum entries `(harmonic_index, complex_voltage)`.
    ///
    /// Harmonic index `1` is the fundamental of the HB basis frequency.
    pub ac_harmonics: Vec<(usize, Complex64)>,
}

/// One canonical exact MNA branch shared by linear HB and the periodic
/// small-signal operators.
///
/// A large-signal registration retains the authored voltage-source spectrum
/// directly. A PAC/PNoise-only registration has `source == None`, because its
/// independent voltage-source row is a zero-valued small-signal constraint.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExactMnaBranch {
    /// Ideal voltage constraint. `source_index` preserves authored
    /// independent-source ordering for PAC excitation lookup.
    VoltageSource {
        branch_ordinal: usize,
        node_pos: usize,
        node_neg: usize,
        source_index: usize,
        source: Option<VoltageSourceBranch>,
    },
    /// Exact `Vpos - Vneg - j*omega*L*I = 0` branch equation.
    Inductor {
        branch_ordinal: usize,
        node_pos: usize,
        node_neg: usize,
        inductance: Value,
    },
}

impl VoltageSourceBranch {
    /// Create new voltage source branch
    pub fn new(node_pos: usize, node_neg: usize, branch_idx: usize, dc_voltage: Value) -> Self {
        Self {
            node_pos,
            node_neg,
            branch_idx,
            dc_voltage,
            ac_harmonics: Vec::new(),
        }
    }

    /// Set AC parameters for a specific harmonic.
    pub fn with_harmonic(mut self, harmonic: usize, magnitude: Value, phase: Value) -> Self {
        self.set_harmonic_component(harmonic, Complex64::from_polar(magnitude, phase));
        self
    }

    fn set_harmonic_component(&mut self, harmonic: usize, value: Complex64) {
        if harmonic == 0 {
            return;
        }
        if let Some((_, component)) = self
            .ac_harmonics
            .iter_mut()
            .find(|(index, _)| *index == harmonic)
        {
            *component = value;
        } else {
            self.ac_harmonics.push((harmonic, value));
        }
    }
}

/// Harmonic Balance solver
///
/// HB solver supporting:
/// - Linear elements: R, C, L (with proper jωL admittance)
/// - MNA voltage sources with branch currents
/// - Nonlinear device Newton iteration via FFT/IFFT
#[derive(Debug)]
pub struct HbSolver {
    /// Configuration
    config: HbConfig,

    /// FFT processor
    fft: HbFft,

    /// Number of nodes
    num_nodes: usize,

    /// Number of harmonics (including DC)
    num_harmonics: usize,

    /// Number of branch currents (for MNA voltage sources)
    num_branches: usize,

    /// Conductance matrix for each node combination
    /// Stored as sparse: (row, col) -> G
    g_matrix: Vec<(usize, usize, Value)>,

    /// Capacitance matrix for each node combination
    /// Stored as sparse: (row, col) -> C
    c_matrix: Vec<(usize, usize, Value)>,

    /// Inductance matrix for each node combination
    /// Stored as sparse: (row, col) -> L
    /// Admittance Y = 1/(jωL) at each harmonic
    l_matrix: Vec<(usize, usize, Value)>,

    /// Voltage source branches for MNA
    /// Each branch may define AC entries on arbitrary HB harmonics.
    voltage_source_branches: Vec<VoltageSourceBranch>,

    /// Authored names aligned with `voltage_source_branches`.
    voltage_source_branch_names: Vec<String>,

    /// Exact branch equations used by PAC and PNoise conversion systems.
    periodic_mna_branches: Vec<ExactMnaBranch>,

    /// Authored names aligned with `periodic_mna_branches` in the circuit's
    /// canonical one-based MNA branch order.
    periodic_mna_branch_names: Vec<String>,

    /// Node names
    node_names: Vec<String>,

    /// Current source spectra, indexed `[node][harmonic]`.
    source_spectra: Vec<Vec<Complex64>>,

    /// Registered nonlinear devices for Newton iteration
    nonlinear_devices: Vec<NonlinearDeviceInstance>,
    /// Per-device thermal-noise temperature provenance, aligned with
    /// `nonlinear_devices`. Absolute TEMP values are retained directly so an
    /// extreme analysis temperature cannot destroy them through subtraction.
    nonlinear_noise_temperatures: Vec<NonlinearNoiseTemperature>,
    /// Registered Verilog-A devices for Newton iteration.
    #[cfg(feature = "veriloga")]
    veriloga_nonlinear_devices: Vec<HbVerilogADevice>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum NonlinearNoiseTemperature {
    Ambient,
    Offset(Value),
    Absolute(Value),
}

impl NonlinearNoiseTemperature {
    fn resolve(self, ambient: Value) -> Value {
        match self {
            Self::Ambient => ambient,
            Self::Offset(offset) => ambient + offset,
            Self::Absolute(temperature) => temperature,
        }
    }
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
struct HbVerilogADevice {
    device: VerilogADevice,
    rhs_rows: Vec<Vec<(usize, Value)>>,
    jacobian_locs: Vec<Vec<(Option<usize>, Option<usize>)>>,
}

#[cfg(feature = "veriloga")]
impl HbVerilogADevice {
    fn new(device: VerilogADevice) -> Self {
        let rhs_rows = device.mapped_rhs_rows();
        let jacobian_locs = device.mapped_jacobian_locations();
        Self {
            device,
            rhs_rows,
            jacobian_locs,
        }
    }

    fn runtime_error(&self, phase: &str, err: impl std::fmt::Display) -> HbError {
        HbError::InvalidCircuit(format!(
            "Verilog-A device '{}' HB {phase} failed: {err}",
            self.device.name
        ))
    }

    fn try_evaluate(&mut self, phase: &str) -> Result<Vec<Value>, HbError> {
        self.device
            .try_evaluate()
            .map_err(|err| self.runtime_error(phase, err))
    }

    fn try_compute_jacobian(
        &mut self,
        phase: &str,
    ) -> Result<Vec<rspice_veriloga::device::JacobianEntry>, HbError> {
        self.device
            .try_compute_jacobian()
            .map_err(|err| self.runtime_error(phase, err))
    }
}

/// Runtime representation of a nonlinear device for HB Newton iteration
///
/// This wraps device parameters and provides unified current/Jacobian evaluation.
/// Used during the Newton solve to compute nonlinear contributions in time domain.
#[derive(Debug, Clone)]
pub struct NonlinearDeviceInstance {
    /// Device type
    pub device_type: NonlinearDeviceType,
    /// Terminal nodes (0-indexed, device-specific ordering)
    pub terminals: Vec<usize>,
    /// Device parameters (device-specific interpretation)
    pub params: NonlinearDeviceParams,
}

/// Type of nonlinear device
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NonlinearDeviceType {
    /// Two-terminal diode (anode, cathode)
    Diode,
    /// Three-terminal NPN BJT (collector, base, emitter)
    NpnBjt,
    /// Three-terminal PNP BJT (collector, base, emitter)
    PnpBjt,
    /// Four-terminal NMOS (drain, gate, source, bulk)
    Nmos,
    /// Four-terminal PMOS (drain, gate, source, bulk)
    Pmos,
    /// Three-terminal N-channel JFET (drain, gate, source)
    Njfet,
    /// Three-terminal P-channel JFET (drain, gate, source)
    Pjfet,
    /// Four-terminal voltage-controlled switch (p, n, cp, cn)
    VoltageSwitch,
    /// Four-terminal current-controlled switch with sensed control voltage
    /// converted to current (p, n, cp, cn)
    CurrentSwitch,
}

/// Depletion-capacitance parameter set for one junction.
///
/// `cj0 = 0` disables the junction charge entirely; `fc` is the forward-bias
/// linearization knee (SPICE FC, default 0.5).
#[derive(Debug, Clone, Copy)]
pub struct DepletionCap {
    /// Zero-bias junction capacitance (F)
    pub cj0: Value,
    /// Built-in potential (V)
    pub vj: Value,
    /// Grading coefficient
    pub m: Value,
    /// Forward-bias depletion linearization coefficient
    pub fc: Value,
}

impl DepletionCap {
    /// A disabled junction (no charge).
    pub fn none() -> Self {
        Self {
            cj0: 0.0,
            vj: 1.0,
            m: 0.5,
            fc: 0.5,
        }
    }

    /// Junction parameters with SPICE-standard clamping.
    pub fn new(cj0: Value, vj: Value, m: Value, fc: Value) -> Self {
        Self {
            cj0: cj0.max(0.0),
            vj: vj.max(0.01),
            m: m.clamp(0.01, 0.95),
            fc: fc.clamp(0.0, 0.99),
        }
    }
}

impl Default for DepletionCap {
    fn default() -> Self {
        Self::none()
    }
}

/// Device parameters for nonlinear devices
#[derive(Debug, Clone)]
pub struct NonlinearDeviceParams {
    /// Saturation current (Is for diode/BJT)
    pub is: Value,
    /// Ideality factor (n for diode)
    pub n: Value,
    /// Forward emission coefficient (BJT B-E junction)
    pub nf: Value,
    /// Reverse emission coefficient (BJT B-C junction)
    pub nr: Value,
    /// Thermal voltage
    pub vt: Value,
    /// Forward beta (BJT)
    pub bf: Value,
    /// Reverse beta (BJT)
    pub br: Value,
    /// Threshold voltage (MOSFET)
    pub vth: Value,
    /// Transconductance parameter K = μCox W/L (MOSFET)
    pub kp: Value,
    /// Channel length modulation (MOSFET)
    pub lambda: Value,
    /// Body-effect coefficient gamma (MOSFET, V^0.5)
    pub gamma: Value,
    /// Surface potential phi (MOSFET, V)
    pub phi: Value,
    /// Early voltage (BJT)
    pub vaf: Value,
    /// Switch ON resistance
    pub ron: Value,
    /// Switch OFF resistance
    pub roff: Value,
    /// Switch hysteresis voltage parameter (stored, currently not stateful in HB)
    pub vh: Value,
    /// Switch transition smoothness
    pub smooth: Value,
    /// Control conversion gain (e.g. sense conductance A/V)
    pub control_gain: Value,
    /// Primary junction depletion capacitance (diode junction, BJT B-E,
    /// JFET G-S)
    pub cap_a: DepletionCap,
    /// Secondary junction depletion capacitance (BJT B-C, JFET G-D)
    pub cap_b: DepletionCap,
    /// Secondary-junction saturation current (MOS drain-bulk diode; the
    /// source-bulk diode rides on `is`)
    pub is2: Value,
    /// Total intrinsic oxide capacitance Cox' * W * Leff (MOS channel
    /// charge model; zero disables it)
    pub cox_wl: Value,
    /// Forward transit time: diode TT / BJT TF (diffusion charge tau_f * i_f)
    pub tt_f: Value,
    /// Reverse transit time: BJT TR (diffusion charge tau_r * i_r)
    pub tt_r: Value,
}

impl Default for NonlinearDeviceParams {
    fn default() -> Self {
        Self {
            is: 1e-14,
            n: 1.0,
            nf: 1.0,
            nr: 1.0,
            vt: 0.02585,
            bf: 100.0,
            br: 1.0,
            vth: 0.7,
            kp: 2e-5,
            lambda: 0.0,
            gamma: 0.0,
            phi: 0.6,
            vaf: f64::INFINITY,
            ron: 1.0,
            roff: 1e6,
            vh: 0.0,
            smooth: 0.1,
            control_gain: 1.0,
            cap_a: DepletionCap::none(),
            cap_b: DepletionCap::none(),
            is2: 1e-14,
            cox_wl: 0.0,
            tt_f: 0.0,
            tt_r: 0.0,
        }
    }
}

impl HbSolver {}

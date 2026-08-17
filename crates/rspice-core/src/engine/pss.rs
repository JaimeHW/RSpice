//! Periodic Steady-State (PSS) Analysis
//!
//! This module provides PSS analysis using the shooting method.
//!
//! # Overview
//!
//! PSS finds the periodic steady-state solution of a circuit by solving the
//! boundary value problem: x(0) = x(T) where T is the period.
//!
//! # Algorithm
//!
//! 1. **Stabilization phase** (`tstab`): Run transient to approach steady-state
//! 2. **Period detection**: For autonomous oscillators, auto-detect the period
//! 3. **Shooting Newton loop**:
//!    - Simulate one period from current `x0`
//!    - Compute residual `x(T) - x(0)`
//!    - Check convergence
//!    - Compute Jacobian (Monodromy - I) via finite differences
//!    - Solve for Newton step and update `x0`
//! 4. Build final `PssResult` with periodic waveform and harmonics

#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
use super::{Engine, SimulationError, TransientCheckpoint, TransientResult};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::{
    PeriodDetector, PeriodicWaveform, PssConfig, PssResult, ShootingNewtonSolver, ShootingState,
};
use crate::circuit::CircuitData;
use crate::numerics::integration::CompanionCoefficients;
use crate::numerics::integration::IntegrationMethod;
use crate::numerics::integration::{
    BreakpointManager, LteEstimator, TimestepController, TrapGearController,
};
use crate::solver::StaticMatrix;
use crate::{Netlist, Value};

type AutonomousNewtonStep = (Vec<Value>, Value, Vec<Vec<Value>>);

/// Accepted-step timing state for the adaptive PSS trajectory.
///
/// Coefficient construction is deliberately read-only: a rejected Newton
/// trial may change the proposed `dt`, but it must not replace the interval
/// between the two most recent accepted solution points. Only `accept` rotates
/// that history.
#[derive(Debug, Clone, Copy, Default)]
struct PssAcceptedStepHistory {
    previous_accepted_dt: Option<Value>,
}

impl PssAcceptedStepHistory {
    #[inline]
    fn coefficients_for_trial(
        &self,
        method: IntegrationMethod,
        dt: Value,
    ) -> CompanionCoefficients {
        match self.previous_accepted_dt {
            Some(previous_dt) => {
                CompanionCoefficients::for_method_with_previous_step(method, dt, previous_dt)
            }
            // Gear2 cannot form a valid second-order stencil until one real
            // interval has been accepted. PSS normally selects BE explicitly
            // for its first step; this fallback keeps the invariant local if
            // the method-selection policy changes later.
            None if method == IntegrationMethod::Gear2 => CompanionCoefficients::backward_euler(),
            None => CompanionCoefficients::for_method(method),
        }
    }

    #[inline]
    fn accept(&mut self, dt: Value) {
        debug_assert!(dt.is_finite() && dt > 0.0);
        self.previous_accepted_dt = (dt.is_finite() && dt > 0.0).then_some(dt);
    }
}

const PSS_KRYLOV_STATE_THRESHOLD: usize = 12;
const PSS_KRYLOV_REL_TOL: Value = 1e-9;

/// Dense LU of the most recently formed shooting Jacobian.  Newton-Krylov
/// uses it only as a right preconditioner; a singular factor disables the
/// matrix-free attempt and sends the caller to the established full-Jacobian
/// path.
struct PssDenseLu {
    n: usize,
    lu: Vec<Value>,
    pivots: Vec<usize>,
}

impl PssDenseLu {
    fn factor(matrix: &[Vec<Value>]) -> Option<Self> {
        let n = matrix.len();
        if matrix.iter().any(|row| row.len() != n) {
            return None;
        }
        let mut lu = matrix.iter().flatten().copied().collect::<Vec<_>>();
        let mut pivots = vec![0; n];
        for k in 0..n {
            let mut pivot_row = k;
            let mut pivot_abs = lu[k * n + k].abs();
            for row in (k + 1)..n {
                let candidate = lu[row * n + k].abs();
                if candidate > pivot_abs {
                    pivot_abs = candidate;
                    pivot_row = row;
                }
            }
            if !pivot_abs.is_finite() || pivot_abs <= 1e-15 {
                return None;
            }
            pivots[k] = pivot_row;
            if pivot_row != k {
                for column in 0..n {
                    lu.swap(k * n + column, pivot_row * n + column);
                }
            }
            let pivot = lu[k * n + k];
            for row in (k + 1)..n {
                let multiplier = lu[row * n + k] / pivot;
                lu[row * n + k] = multiplier;
                for column in (k + 1)..n {
                    lu[row * n + column] -= multiplier * lu[k * n + column];
                }
            }
        }
        Some(Self { n, lu, pivots })
    }

    fn solve(&self, rhs: &[Value]) -> Vec<Value> {
        debug_assert_eq!(rhs.len(), self.n);
        let mut solution = rhs.to_vec();
        for k in 0..self.n {
            if self.pivots[k] != k {
                solution.swap(k, self.pivots[k]);
            }
        }
        for row in 1..self.n {
            let mut value = solution[row];
            for column in 0..row {
                value -= self.lu[row * self.n + column] * solution[column];
            }
            solution[row] = value;
        }
        for row in (0..self.n).rev() {
            let mut value = solution[row];
            for column in (row + 1)..self.n {
                value -= self.lu[row * self.n + column] * solution[column];
            }
            solution[row] = value / self.lu[row * self.n + row];
        }
        solution
    }
}

#[inline]
fn pss_dot(left: &[Value], right: &[Value]) -> Value {
    left.iter().zip(right).map(|(a, b)| a * b).sum()
}

#[inline]
fn pss_norm(vector: &[Value]) -> Value {
    pss_dot(vector, vector).sqrt()
}

/// Restarted, right-preconditioned real GMRES for shooting Newton systems.
/// The product is fallible because every J*v evaluates two complete period
/// maps.  A non-converged outcome is `None`, which is a correctness-preserving
/// request for the caller to rebuild and directly solve the dense Jacobian.
fn pss_gmres(
    matvec: &mut dyn FnMut(&[Value]) -> Result<Vec<Value>, SimulationError>,
    preconditioner: &PssDenseLu,
    rhs: &[Value],
    restart: usize,
    max_outer: usize,
) -> Result<Option<Vec<Value>>, SimulationError> {
    let size = rhs.len();
    let rhs_norm = pss_norm(rhs);
    if rhs_norm == 0.0 {
        return Ok(Some(vec![0.0; size]));
    }
    let restart = restart.clamp(1, size);
    let mut solution = vec![0.0; size];
    let mut residual = rhs.to_vec();
    let mut beta = rhs_norm;

    for _ in 0..max_outer {
        let mut basis = Vec::with_capacity(restart + 1);
        basis.push(
            residual
                .iter()
                .map(|value| value / beta)
                .collect::<Vec<_>>(),
        );
        let mut hessenberg: Vec<Vec<Value>> = Vec::with_capacity(restart);
        let mut cosines: Vec<Value> = Vec::with_capacity(restart);
        let mut sines: Vec<Value> = Vec::with_capacity(restart);
        let mut projected_rhs = vec![0.0; restart + 1];
        projected_rhs[0] = beta;
        let mut used = 0;
        let mut breakdown = false;

        for column in 0..restart {
            used = column + 1;
            let z = preconditioner.solve(&basis[column]);
            let mut image = matvec(&z)?;
            let mut h_column = vec![0.0; column + 2];
            for row in 0..=column {
                let projection = pss_dot(&basis[row], &image);
                h_column[row] = projection;
                for (value, basis_value) in image.iter_mut().zip(&basis[row]) {
                    *value -= projection * basis_value;
                }
            }
            let image_norm = pss_norm(&image);
            h_column[column + 1] = image_norm;
            for row in 0..column {
                let rotated = cosines[row] * h_column[row] + sines[row] * h_column[row + 1];
                h_column[row + 1] = -sines[row] * h_column[row] + cosines[row] * h_column[row + 1];
                h_column[row] = rotated;
            }
            let diagonal_norm = h_column[column].hypot(h_column[column + 1]);
            let (cosine, sine) = if diagonal_norm > 0.0 {
                (
                    h_column[column] / diagonal_norm,
                    h_column[column + 1] / diagonal_norm,
                )
            } else {
                (1.0, 0.0)
            };
            h_column[column] = diagonal_norm;
            h_column[column + 1] = 0.0;
            let projected = projected_rhs[column];
            projected_rhs[column] = cosine * projected;
            projected_rhs[column + 1] = -sine * projected;
            cosines.push(cosine);
            sines.push(sine);
            hessenberg.push(h_column);

            if projected_rhs[column + 1].abs() / rhs_norm < PSS_KRYLOV_REL_TOL
                || image_norm <= 1e-300
            {
                breakdown = true;
                break;
            }
            basis.push(image.into_iter().map(|value| value / image_norm).collect());
        }

        let mut coefficients = vec![0.0; used];
        for row in (0..used).rev() {
            let mut value = projected_rhs[row];
            for column in (row + 1)..used {
                value -= hessenberg[column][row] * coefficients[column];
            }
            let diagonal = hessenberg[row][row];
            if !diagonal.is_finite() || diagonal.abs() <= 1e-300 {
                return Ok(None);
            }
            coefficients[row] = value / diagonal;
        }
        let mut update = vec![0.0; size];
        for (column, coefficient) in coefficients.iter().enumerate() {
            for (value, basis_value) in update.iter_mut().zip(&basis[column]) {
                *value += coefficient * basis_value;
            }
        }
        let update = preconditioner.solve(&update);
        for (value, step) in solution.iter_mut().zip(update) {
            *value += step;
        }

        let image = matvec(&solution)?;
        for ((value, rhs_value), image_value) in residual.iter_mut().zip(rhs).zip(image) {
            *value = rhs_value - image_value;
        }
        let next_beta = pss_norm(&residual);
        if next_beta / rhs_norm < PSS_KRYLOV_REL_TOL {
            return Ok(Some(solution));
        }
        if breakdown || next_beta >= 0.99 * beta {
            return Ok(None);
        }
        beta = next_beta;
    }
    Ok(None)
}

/// Recover the monodromy matrix from a converged shooting Jacobian:
/// the shooting residual is F(x0) = x(T) - x0, so J = dF/dx0 = M - I and
/// M = J + I. Reusing the Jacobian saves N+1 full period integrations.
fn monodromy_from_newton_jacobian(mut jacobian: Vec<Vec<Value>>) -> Vec<Vec<Value>> {
    for (i, row) in jacobian.iter_mut().enumerate() {
        if i < row.len() {
            row[i] += 1.0;
        }
    }
    jacobian
}

/// Select the PSS integration formula without making the fixed shooting grid
/// depend on state-sensitive TrapGear switching decisions.
fn pss_integration_method(
    first_step: bool,
    fixed_grid: bool,
    requested: Option<IntegrationMethod>,
    adaptive_trapgear_method: IntegrationMethod,
) -> IntegrationMethod {
    if first_step {
        return IntegrationMethod::BackwardEuler;
    }
    match requested {
        Some(IntegrationMethod::TrapGear) if fixed_grid => IntegrationMethod::Trapezoidal,
        Some(IntegrationMethod::TrapGear) => adaptive_trapgear_method,
        Some(method) => method,
        None if fixed_grid => IntegrationMethod::Trapezoidal,
        None => adaptive_trapgear_method,
    }
}

fn ensure_pss_traversal_complete(
    time: Value,
    tstop: Value,
    total_iterations: usize,
    max_iterations: usize,
    fixed_grid: bool,
    fixed_index: usize,
    fixed_steps: usize,
    retained_endpoint: Option<Value>,
) -> Result<(), SimulationError> {
    if time != tstop {
        let reason = if total_iterations >= max_iterations {
            format!("reached the hard {max_iterations}-iteration guard")
        } else if fixed_grid && fixed_index >= fixed_steps {
            format!("exhausted the {fixed_steps}-step fixed grid")
        } else {
            "terminated without an accepted endpoint".to_string()
        };
        return Err(SimulationError::Circuit(format!(
            "PSS transient traversal {reason} at t={time:.17e} before the exact stop t={tstop:.17e}; refusing to publish a partial trajectory"
        )));
    }
    if retained_endpoint != Some(tstop) {
        return Err(SimulationError::Circuit(format!(
            "PSS transient traversal reached tstop={tstop:.17e} without retaining that exact endpoint"
        )));
    }
    Ok(())
}

/// PSS-specific error types
#[derive(Debug, Clone)]
pub enum PssError {
    /// Newton iteration did not converge
    ConvergenceFailed { iterations: usize, residual: Value },
    /// Period detection failed for autonomous oscillator
    PeriodDetectionFailed(String),
    /// Circuit has no reactive elements (no periodic solution possible)
    NoReactiveElements,
    /// Invalid configuration
    InvalidConfig(String),
}

impl std::fmt::Display for PssError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvergenceFailed {
                iterations,
                residual,
            } => {
                write!(
                    f,
                    "PSS convergence failed after {} iterations (residual: {:.3e})",
                    iterations, residual
                )
            }
            Self::PeriodDetectionFailed(msg) => write!(f, "Period detection failed: {}", msg),
            Self::NoReactiveElements => write!(f, "Circuit has no capacitors or inductors"),
            Self::InvalidConfig(msg) => write!(f, "Invalid PSS config: {}", msg),
        }
    }
}

impl std::error::Error for PssError {}

impl From<PssError> for SimulationError {
    fn from(e: PssError) -> Self {
        match e {
            PssError::ConvergenceFailed { iterations, .. } => {
                SimulationError::ConvergenceFailed(iterations)
            }
            _ => SimulationError::Circuit(e.to_string()),
        }
    }
}

/// Reactive-state and node-solution trajectory recorded on the fixed
/// integration grid, used by the oscillator phase-noise (PPV) machinery.
#[derive(Debug, Default, Clone)]
pub(crate) struct PssStateTrace {
    /// Grid times, starting at 0.
    pub times: Vec<Value>,
    /// Reactive state (cap voltages then inductor currents) per grid point.
    pub states: Vec<Vec<Value>>,
    /// Full node/branch solution per grid point.
    pub solutions: Vec<Vec<Value>>,
}

/// PSS analysis result with detailed convergence info
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct PssAnalysisResult {
    /// The periodic steady-state solution
    pub result: PssResult,
    /// Number of shooting Newton iterations
    pub iterations: usize,
    /// Final residual norm
    pub final_residual: Value,
    /// Detected/refined period
    pub period: Value,
    /// Monodromy matrix (state transition over one period)
    pub monodromy: Vec<Vec<Value>>,
    /// Floquet multipliers (for stability analysis)
    pub floquet_multipliers: Vec<num_complex::Complex64>,
    /// Whether circuit is stable (all multipliers inside unit circle)
    pub is_stable: bool,
}

/// Exact DC operating-point state used to initialize shooting PSS.
///
/// The solution uses the core MNA ordering: one value for every non-ground
/// node, followed by one value for every branch-current unknown. Node and
/// branch names are retained in that same order so the seed can be rejected
/// if it is presented to a different or re-elaborated circuit.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct PssDcOperatingPointSeed {
    node_names: Vec<String>,
    branch_names: Vec<String>,
    solution: Vec<Value>,
}

impl PssDcOperatingPointSeed {
    /// Construct a structurally self-consistent PSS DC seed.
    ///
    /// Circuit-specific name and dimension checks are intentionally repeated
    /// when the seed is consumed, after the target netlist has been fully
    /// elaborated. This constructor rejects malformed transport payloads
    /// before they can enter the solver boundary.
    pub fn try_new(
        node_names: Vec<String>,
        branch_names: Vec<String>,
        solution: Vec<Value>,
    ) -> Result<Self, SimulationError> {
        let expected_solution_len = node_names
            .len()
            .checked_add(branch_names.len())
            .ok_or_else(|| {
                SimulationError::Circuit("PSS DC seed dimensions overflow the platform".to_owned())
            })?;
        if solution.len() != expected_solution_len {
            return Err(SimulationError::Circuit(format!(
                "PSS DC seed contains {} MNA values, but its {} node names and {} branch names require {expected_solution_len}",
                solution.len(),
                node_names.len(),
                branch_names.len()
            )));
        }
        if solution.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "PSS DC seed contains a non-finite MNA value".to_owned(),
            ));
        }
        Self::validate_names("node", &node_names)?;
        Self::validate_names("branch", &branch_names)?;

        Ok(Self {
            node_names,
            branch_names,
            solution,
        })
    }

    /// Canonical non-ground node names in MNA order.
    pub fn node_names(&self) -> &[String] {
        &self.node_names
    }

    /// Canonical branch-current unknown names in MNA order.
    pub fn branch_names(&self) -> &[String] {
        &self.branch_names
    }

    /// Full MNA solution: node voltages followed by branch currents.
    pub fn solution(&self) -> &[Value] {
        &self.solution
    }

    fn validate_names(kind: &str, names: &[String]) -> Result<(), SimulationError> {
        let mut seen = std::collections::HashSet::with_capacity(names.len());
        for name in names {
            if name.is_empty() || name.trim() != name {
                return Err(SimulationError::Circuit(format!(
                    "PSS DC seed contains a non-canonical {kind} name"
                )));
            }
            let normalized = name.to_ascii_uppercase();
            if !seen.insert(normalized) {
                return Err(SimulationError::Circuit(format!(
                    "PSS DC seed contains duplicate {kind} name '{name}'"
                )));
            }
        }
        Ok(())
    }

    fn validate_for_circuit(&self, circuit: &CircuitData) -> Result<(), SimulationError> {
        let expected_node_names = circuit.node_names_sorted();
        let expected_branch_names = circuit.branch_names_sorted();
        if self.node_names != expected_node_names {
            return Err(SimulationError::Circuit(format!(
                "PSS DC seed node basis does not match the elaborated circuit: expected {:?}, received {:?}",
                expected_node_names, self.node_names
            )));
        }
        if self.branch_names != expected_branch_names {
            return Err(SimulationError::Circuit(format!(
                "PSS DC seed branch basis does not match the elaborated circuit: expected {:?}, received {:?}",
                expected_branch_names, self.branch_names
            )));
        }
        if self.solution.len() != circuit.matrix_size() {
            return Err(SimulationError::Circuit(format!(
                "PSS DC seed has {} MNA values, but the elaborated circuit requires {}",
                self.solution.len(),
                circuit.matrix_size()
            )));
        }
        if self.solution.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "PSS DC seed contains a non-finite MNA value".to_owned(),
            ));
        }
        Ok(())
    }
}

/// Exact converged shooting-PSS numerical state retained for dependent
/// analyses. This is deliberately distinct from a display waveform: the
/// reactive shooting state and monodromy matrix are part of the contract and
/// must survive task and worker boundaries without being recomputed.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "veriloga", derive(serde::Serialize, serde::Deserialize))]
pub struct PssOperatingPoint {
    config: PssConfig,
    analysis: PssAnalysisResult,
    shooting_state: Vec<Value>,
}

impl PssOperatingPoint {
    /// Fully materialized shooting configuration that produced this state.
    pub fn config(&self) -> &PssConfig {
        &self.config
    }

    /// Converged periodic result and stability data.
    pub fn analysis(&self) -> &PssAnalysisResult {
        &self.analysis
    }

    /// Reactive state at the authenticated phase origin.
    pub fn shooting_state(&self) -> &[Value] {
        &self.shooting_state
    }

    /// Highest Fourier harmonic that can be projected without exceeding the
    /// Nyquist limit of the retained time-domain orbit. Saved-output harmonic
    /// count is intentionally not part of this capacity: dependent analyses
    /// consume the authenticated orbit, not the optional display spectrum.
    pub fn spectral_harmonic_capacity(&self) -> usize {
        self.config
            .points_per_period
            .min(self.analysis.result.time.len().saturating_sub(1))
            / 2
    }

    /// Reconstruct a retained operating point after authenticated transport.
    /// Shape and finiteness checks are repeated here so callers cannot feed a
    /// fabricated partial state into dependent numerical kernels.
    pub fn try_from_parts(
        config: PssConfig,
        analysis: PssAnalysisResult,
        shooting_state: Vec<Value>,
    ) -> Result<Self, SimulationError> {
        config.validate().map_err(PssError::InvalidConfig)?;
        Self::validate_parts(&config, &analysis, &shooting_state)?;
        Ok(Self {
            config,
            analysis,
            shooting_state,
        })
    }

    fn validate_parts(
        config: &PssConfig,
        analysis: &PssAnalysisResult,
        shooting_state: &[Value],
    ) -> Result<(), SimulationError> {
        if !analysis.period.is_finite() || analysis.period <= 0.0 {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has an invalid period".to_owned(),
            ));
        }
        if !analysis.result.period.is_finite()
            || !analysis.result.frequency.is_finite()
            || analysis.result.period <= 0.0
            || analysis.result.frequency <= 0.0
        {
            return Err(SimulationError::Circuit(
                "retained PSS result has an invalid period or frequency".to_owned(),
            ));
        }
        let period_tolerance =
            (64.0 * Value::EPSILON * analysis.period.abs()).max(Value::MIN_POSITIVE);
        if (analysis.result.period - analysis.period).abs() > period_tolerance
            || (analysis.result.frequency - 1.0 / analysis.period).abs()
                > 64.0 * Value::EPSILON * analysis.result.frequency.abs().max(1.0)
        {
            return Err(SimulationError::Circuit(
                "retained PSS result basis is inconsistent with its shooting period".to_owned(),
            ));
        }
        if !config.is_autonomous() {
            let requested_period = config.period();
            let relative_error = ((analysis.period - requested_period) / requested_period).abs();
            if relative_error > 1.0e-9 {
                return Err(SimulationError::Circuit(format!(
                    "retained PSS period {:.16e} s does not match its driven configuration period {:.16e} s",
                    analysis.period, requested_period
                )));
            }
        }
        if analysis.result.time.len() < 2
            || analysis.result.waveforms.is_empty()
            || analysis.result.node_names.len() != analysis.result.waveforms.len()
        {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has an incomplete periodic orbit".to_owned(),
            ));
        }
        let expected_time_samples = config.points_per_period.checked_add(1).ok_or_else(|| {
            SimulationError::Circuit("retained PSS point count overflows the platform".to_owned())
        })?;
        if analysis.result.time.len() != expected_time_samples {
            return Err(SimulationError::Circuit(format!(
                "retained PSS orbit has {} time samples; its configured grid requires {expected_time_samples}",
                analysis.result.time.len()
            )));
        }
        if !analysis.final_residual.is_finite()
            || !analysis.result.residual_norm.is_finite()
            || analysis.final_residual < 0.0
            || analysis.result.residual_norm < 0.0
        {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has an invalid residual".to_owned(),
            ));
        }
        if analysis
            .result
            .floquet_multipliers
            .iter()
            .chain(&analysis.floquet_multipliers)
            .any(|value| !value.re.is_finite() || !value.im.is_finite())
        {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has a non-finite Floquet multiplier".to_owned(),
            ));
        }
        let mut normalized_node_names =
            std::collections::HashSet::with_capacity(analysis.result.node_names.len());
        for node_name in &analysis.result.node_names {
            let normalized = node_name.trim().to_ascii_uppercase();
            if normalized.is_empty() || !normalized_node_names.insert(normalized) {
                return Err(SimulationError::Circuit(
                    "retained PSS operating point has an empty or duplicate node name".to_owned(),
                ));
            }
        }
        if analysis.result.time.iter().any(|value| !value.is_finite())
            || analysis
                .result
                .time
                .windows(2)
                .any(|pair| pair[1] <= pair[0])
        {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has an invalid time grid".to_owned(),
            ));
        }
        let first_time = analysis.result.time[0];
        let last_time = *analysis
            .result
            .time
            .last()
            .expect("validated non-empty PSS time grid");
        if first_time.abs() > period_tolerance
            || (last_time - analysis.period).abs() > period_tolerance
            || analysis
                .result
                .time
                .iter()
                .any(|time| *time < -period_tolerance || *time > analysis.period + period_tolerance)
        {
            return Err(SimulationError::Circuit(
                "retained PSS time grid does not span exactly one shooting period".to_owned(),
            ));
        }
        let sample_count = analysis.result.time.len();
        if analysis.result.waveforms.iter().any(|waveform| {
            waveform.values.len() != sample_count
                || waveform.values.iter().any(|value| !value.is_finite())
        }) {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has an invalid waveform payload".to_owned(),
            ));
        }
        if shooting_state.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has an invalid reactive state".to_owned(),
            ));
        }
        if analysis.monodromy.len() != shooting_state.len()
            || analysis.monodromy.iter().any(|row| {
                row.len() != shooting_state.len() || row.iter().any(|value| !value.is_finite())
            })
        {
            return Err(SimulationError::Circuit(
                "retained PSS operating point has an invalid monodromy matrix".to_owned(),
            ));
        }
        Ok(())
    }
}

/// Phase-consistent state at the end of one converged PSS period, projected
/// onto time zero and ready for
/// the transient integrator's breakpoint-restart continuation contract.
///
/// State families whose accepted histories cannot be projected safely are
/// rejected before this artifact is created. For envelope initialization, the
/// artifact also authenticates the exact independent sources that were frozen
/// at their original time-zero values during PSS and may be reactivated when
/// transient integration starts.
#[derive(Debug, Clone, PartialEq)]
pub struct PssContinuationState {
    period: Value,
    frozen_sources: Vec<String>,
    checkpoint: TransientCheckpoint,
}

impl PssContinuationState {
    /// Converged fundamental period in seconds.
    pub fn period(&self) -> Value {
        self.period
    }

    /// Absolute simulation time represented by this phase-equivalent state.
    pub fn time_origin(&self) -> Value {
        self.checkpoint.time
    }

    /// Canonical independent-source names frozen during the periodic solve.
    pub fn frozen_sources(&self) -> &[String] {
        &self.frozen_sources
    }
}

impl Engine {
    /// Run Periodic Steady-State analysis
    ///
    /// This is the main entry point for PSS simulation. It handles both driven
    /// circuits (known period from source frequency) and autonomous oscillators
    /// (period detected from waveform).
    ///
    /// # Arguments
    ///
    /// * `netlist` - The circuit netlist
    /// * `config` - PSS configuration (frequency, tolerances, etc.)
    ///
    /// # Returns
    ///
    /// * `Ok(PssAnalysisResult)` - Converged periodic solution with waveforms
    /// * `Err(SimulationError)` - If convergence fails or circuit is invalid
    pub fn run_pss(
        &self,
        netlist: &Netlist,
        config: PssConfig,
    ) -> Result<PssAnalysisResult, SimulationError> {
        self.run_pss_with_abort(netlist, config, &NoAbort)
    }

    /// Run PSS with cooperative cancellation across stabilization, shooting,
    /// finite-difference columns, and Floquet analysis.
    pub fn run_pss_with_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<PssAnalysisResult, SimulationError> {
        self.run_pss_with_state_abort(netlist, config, abort)
            .map(|(result, _, _, _)| result)
    }

    /// Run PSS from an exact, externally retained DC operating point.
    ///
    /// The seed must match the fully elaborated circuit's canonical node and
    /// branch ordering exactly. A valid seed replaces the internal DC solve;
    /// it is used directly to initialize reactive state and stabilization.
    pub fn run_pss_with_dc_seed(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        dc_seed: &PssDcOperatingPointSeed,
    ) -> Result<PssAnalysisResult, SimulationError> {
        self.run_pss_with_dc_seed_and_abort(netlist, config, dc_seed, &NoAbort)
    }

    /// Cancellable form of [`Self::run_pss_with_dc_seed`].
    pub fn run_pss_with_dc_seed_and_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        dc_seed: &PssDcOperatingPointSeed,
        abort: &dyn AbortSignal,
    ) -> Result<PssAnalysisResult, SimulationError> {
        self.run_pss_with_state_and_frozen_sources_abort(
            netlist,
            config,
            &std::collections::BTreeSet::new(),
            false,
            Some(dc_seed),
            abort,
        )
        .map(|(result, _, _, _)| result)
    }

    /// Solve PSS and retain the exact numerical state required by dependent
    /// periodic analyses.
    pub fn run_pss_operating_point_with_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<PssOperatingPoint, SimulationError> {
        let retained_config = config.clone();
        let (analysis, _, _, shooting_state) =
            self.run_pss_with_state_abort(netlist, config, abort)?;
        PssOperatingPoint::try_from_parts(retained_config, analysis, shooting_state)
    }

    /// Solve PSS from an exact DC seed and retain the converged numerical
    /// state required by dependent periodic analyses.
    pub fn run_pss_operating_point_with_dc_seed(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        dc_seed: &PssDcOperatingPointSeed,
    ) -> Result<PssOperatingPoint, SimulationError> {
        self.run_pss_operating_point_with_dc_seed_and_abort(netlist, config, dc_seed, &NoAbort)
    }

    /// Cancellable form of [`Self::run_pss_operating_point_with_dc_seed`].
    pub fn run_pss_operating_point_with_dc_seed_and_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        dc_seed: &PssDcOperatingPointSeed,
        abort: &dyn AbortSignal,
    ) -> Result<PssOperatingPoint, SimulationError> {
        let retained_config = config.clone();
        let (analysis, _, _, shooting_state) = self.run_pss_with_state_and_frozen_sources_abort(
            netlist,
            config,
            &std::collections::BTreeSet::new(),
            false,
            Some(dc_seed),
            abort,
        )?;
        PssOperatingPoint::try_from_parts(retained_config, analysis, shooting_state)
    }

    /// Run PSS and materialize a phase-consistent transient continuation
    /// state from one final fixed-grid traversal of the converged orbit.
    ///
    /// This is the warm-start contract for analyses such as envelope
    /// simulation. It is intentionally fail-closed for runtime state families
    /// that shooting PSS does not yet advance with the transient integrator's
    /// accepted-step lifecycle; returning a checkpoint for those circuits
    /// would misrepresent stale internal state as periodic.
    pub fn run_pss_with_continuation_state(
        &self,
        netlist: &Netlist,
        config: PssConfig,
    ) -> Result<(PssAnalysisResult, PssContinuationState), SimulationError> {
        self.run_pss_with_continuation_state_abort(netlist, config, &NoAbort)
    }

    /// Cancellable form of [`Self::run_pss_with_continuation_state`].
    pub fn run_pss_with_continuation_state_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<(PssAnalysisResult, PssContinuationState), SimulationError> {
        self.run_pss_with_frozen_source_continuation_state_abort(netlist, config, &[], abort)
    }

    /// Run PSS with selected independent source waveforms frozen at their
    /// exact time-zero values, then return a continuation artifact authorized
    /// to reactivate only those waveforms against the original netlist.
    ///
    /// This makes a carrier-periodic operating point well-defined when slower
    /// envelope/modulation sources would otherwise make the authored deck
    /// non-periodic. Source names are matched case-insensitively after circuit
    /// elaboration; unknown, empty, or duplicate names are rejected.
    pub fn run_pss_with_frozen_source_continuation_state(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        frozen_source_names: &[String],
    ) -> Result<(PssAnalysisResult, PssContinuationState), SimulationError> {
        self.run_pss_with_frozen_source_continuation_state_abort(
            netlist,
            config,
            frozen_source_names,
            &NoAbort,
        )
    }

    /// Cancellable form of
    /// [`Self::run_pss_with_frozen_source_continuation_state`].
    pub fn run_pss_with_frozen_source_continuation_state_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        frozen_source_names: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<(PssAnalysisResult, PssContinuationState), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        // This identity includes fully elaborated semantics and the bytes of
        // every external dependency (for example PWL FILE data). Capture it
        // before circuit construction reads any of those dependencies, then
        // authenticate the same snapshot after both expensive traversals.
        let authenticated_netlist_identity = Self::pss_continuation_netlist_identity(netlist)?;
        let authenticated_fingerprint = super::transient::netlist_fingerprint(netlist);
        let engine = self.resolved_for_netlist(netlist);
        let continuation_config = config.clone();
        let frozen_source_set = Self::validate_pss_frozen_source_names(frozen_source_names)?;
        let (analysis, mut circuit, mut matrix, shooting_state) = engine
            .run_pss_with_state_and_frozen_sources_abort(
                netlist,
                config,
                &frozen_source_set,
                true,
                None,
                abort,
            )?;
        Self::ensure_pss_continuation_netlist_identity(
            netlist,
            &authenticated_netlist_identity,
            "the periodic solve",
        )?;
        // The pre-solve check in `run_pss_with_state_and_frozen_sources_abort`
        // is the authoritative gate. Repeat it against the returned circuit
        // so a future refactor cannot accidentally bypass the allowlist.
        Self::ensure_pss_continuation_state_supported(&circuit)?;
        let mut frozen_sources = circuit
            .voltage_sources
            .names
            .iter()
            .chain(&circuit.current_sources.names)
            .filter(|name| frozen_source_set.contains(&name.to_ascii_lowercase()))
            .cloned()
            .collect::<Vec<_>>();
        frozen_sources.sort_by_key(|name| name.to_ascii_lowercase());

        let period = analysis.period;
        let max_step = period / continuation_config.points_per_period as Value;
        engine.pss_set_reactive_state(&mut circuit, &shooting_state);
        let seed = engine.pss_initial_node_solution(&mut circuit, &mut matrix, period, abort)?;
        let mut trace = PssStateTrace::default();
        engine.pss_run_tran_internal(
            &mut circuit,
            &mut matrix,
            seed,
            period,
            max_step,
            true,
            Some(&mut trace),
            continuation_config.integration_method,
            abort,
        )?;
        Self::ensure_pss_continuation_netlist_identity(
            netlist,
            &authenticated_netlist_identity,
            "the final converged-orbit traversal",
        )?;
        let endpoint = trace.solutions.last().ok_or_else(|| {
            SimulationError::Circuit(
                "PSS continuation state could not capture the converged orbit endpoint".to_string(),
            )
        })?;
        let lte_reference = engine.config.transient_lte_reference.unwrap_or_else(|| {
            engine
                .config
                .spice_dialect
                .default_transient_lte_reference()
        });
        let mut lte_estimator = LteEstimator::with_tolerances_and_reference(
            engine.transient_lte_reltol(),
            engine.transient_lte_abstol(),
            lte_reference,
        );
        for solution in &trace.solutions {
            lte_estimator.record(solution, max_step);
        }
        let checkpoint = TransientCheckpoint::capture(
            authenticated_fingerprint,
            Some(authenticated_netlist_identity),
            super::transient::simulation_checkpoint_identity(&engine.config),
            0.0,
            endpoint,
            &circuit,
            crate::engine::TransientStartupMode::OperatingPoint,
            Some(&lte_estimator),
        );

        Ok((
            analysis,
            PssContinuationState {
                period,
                frozen_sources,
                checkpoint,
            },
        ))
    }

    /// Continue transient integration for `duration` seconds from a converged
    /// PSS state. Returned sample times remain absolute and start at
    /// `state.time_origin()`.
    pub fn run_tran_from_pss_state(
        &self,
        netlist: &Netlist,
        state: &PssContinuationState,
        duration: Value,
        max_step: Value,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        self.run_tran_from_pss_state_with_abort(netlist, state, duration, max_step, &NoAbort)
    }

    /// Cancellable form of [`Self::run_tran_from_pss_state`].
    pub fn run_tran_from_pss_state_with_abort(
        &self,
        netlist: &Netlist,
        state: &PssContinuationState,
        duration: Value,
        max_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), SimulationError> {
        if !duration.is_finite() || duration <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "PSS continuation duration must be finite and positive, got {duration:e}"
            )));
        }
        let tstop = state.time_origin() + duration;
        if !tstop.is_finite() || tstop <= state.time_origin() {
            return Err(SimulationError::Circuit(format!(
                "PSS continuation stop time overflowed for origin {:e} and duration {duration:e}",
                state.time_origin()
            )));
        }
        self.run_tran_resume_with_abort(netlist, &state.checkpoint, tstop, max_step, abort)
    }

    fn ensure_pss_continuation_state_supported(
        circuit: &CircuitData,
    ) -> Result<(), SimulationError> {
        let mut blockers = Vec::new();
        // Keep this list aligned with transient/residual.rs and
        // transient/state_commit.rs. Shooting PSS currently advances only the
        // ordinary capacitor and inductor companion histories exactly.
        if !circuit.diodes.is_empty() {
            blockers.push("diode junction/diffusion charge history");
        }
        if !circuit.bjts.is_empty() {
            blockers.push("BJT/VBIC charge and internal-state history");
        }
        if !circuit.jfets.is_empty() {
            blockers.push("JFET/MESFET charge and trap history");
        }
        if !circuit.mosfets.is_empty() {
            blockers.push("classic MOSFET charge history");
        }
        if !circuit.b3soi.is_empty() || !circuit.b3soi_fd.is_empty() || !circuit.b3soi_pd.is_empty()
        {
            blockers.push("BSIMSOI charge history");
        }
        if !circuit.bsim3v3.is_empty() {
            blockers.push("BSIM3 charge history");
        }
        if !circuit.bsim4v8.is_empty() {
            blockers.push("BSIM4 charge and NQS history");
        }
        if !circuit.ekv26s.is_empty() {
            blockers.push("EKV 2.6 charge history");
        }
        if !circuit.ekv3s.is_empty() {
            blockers.push("EKV3 charge history");
        }
        if !circuit.vdmoses.is_empty() {
            blockers.push("VDMOS charge history");
        }
        if !circuit.couplings.is_empty() || !circuit.coupled_inductor_pairs.is_empty() {
            blockers.push("coupled-inductor mutual history");
        }
        if !circuit.multi_winding_transformers.is_empty() {
            blockers.push("multi-winding transformer history");
        }
        if !circuit.tlines.is_empty() {
            blockers.push("transmission-line delay history");
        }
        if !circuit.coupled_tlines.is_empty() {
            blockers.push("coupled transmission-line convolution history");
        }
        if !circuit.xspice_instances.is_empty() {
            blockers.push("XSPICE accepted-step and event state");
        }
        if !circuit.generic_switches.is_empty()
            || !circuit.vswitches.is_empty()
            || !circuit.iswitches.is_empty()
        {
            blockers.push("switch hysteresis state");
        }
        if !circuit.xyce_memristors.is_empty() {
            blockers.push("native memristor resistance state");
        }
        if !circuit.jiles_atherton_inductors.is_empty() {
            blockers.push("Jiles-Atherton hysteretic magnetic history");
        }
        if !circuit.xyce_core_groups.is_empty() {
            blockers.push("shared Xyce Core hysteretic magnetic history");
        }
        if !circuit.behavioral_sources.is_empty() {
            blockers.push("behavioral-source accepted-step memory");
        }
        #[cfg(feature = "veriloga")]
        if circuit.has_veriloga_devices() {
            blockers.push("Verilog-A integration state");
        }
        #[cfg(feature = "veriloga-builtins-base")]
        if circuit.has_generated_veriloga_devices() {
            blockers.push("generated Verilog-A integration state");
        }

        if blockers.is_empty() {
            Ok(())
        } else {
            blockers.sort_unstable();
            blockers.dedup();
            Err(SimulationError::Circuit(format!(
                "PSS transient continuation is unavailable because the circuit contains {}; the shooting period map advances only ordinary capacitor and inductor companion history exactly",
                blockers.join(", ")
            )))
        }
    }

    fn pss_continuation_netlist_identity(netlist: &Netlist) -> Result<String, SimulationError> {
        super::transient::netlist_checkpoint_identity(netlist).ok_or_else(|| {
            SimulationError::Circuit(
                "PSS continuation could not authenticate the semantic netlist and its external dependencies before circuit construction"
                    .to_string(),
            )
        })
    }

    fn ensure_pss_continuation_netlist_identity(
        netlist: &Netlist,
        expected: &str,
        phase: &str,
    ) -> Result<(), SimulationError> {
        let current = Self::pss_continuation_netlist_identity(netlist)?;
        if current == expected {
            Ok(())
        } else {
            Err(SimulationError::Circuit(format!(
                "PSS continuation input dependencies changed during {phase}; the semantic netlist or external-file content no longer matches the authenticated pre-build snapshot"
            )))
        }
    }

    fn validate_pss_frozen_source_names(
        frozen_source_names: &[String],
    ) -> Result<std::collections::BTreeSet<String>, SimulationError> {
        let mut normalized = std::collections::BTreeSet::new();
        for source in frozen_source_names {
            let source = source.trim().to_ascii_lowercase();
            if source.is_empty() {
                return Err(SimulationError::Circuit(
                    "PSS frozen-source selection contains an empty source name".to_string(),
                ));
            }
            if !normalized.insert(source.clone()) {
                return Err(SimulationError::Circuit(format!(
                    "PSS frozen-source selection contains duplicate source '{source}'"
                )));
            }
        }
        Ok(normalized)
    }

    fn freeze_pss_independent_sources(
        circuit: &mut CircuitData,
        frozen_sources: &std::collections::BTreeSet<String>,
    ) -> Result<(), SimulationError> {
        for source in frozen_sources {
            let value = circuit
                .voltage_sources
                .freeze_transient_source_at_time(source, 0.0)
                .or_else(|| {
                    circuit
                        .current_sources
                        .freeze_transient_source_at_time(source, 0.0)
                })
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "PSS frozen-source selection references unknown independent source '{source}'"
                    ))
                })?;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "PSS frozen source '{source}' evaluates to non-finite value {value:e} at time zero"
                )));
            }
        }
        Ok(())
    }

    /// `run_pss` plus the converged artifacts the oscillator phase-noise
    /// machinery needs: the prepared circuit/matrix pair and the converged
    /// shooting state x0.
    pub(in crate::engine) fn run_pss_with_state_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<(PssAnalysisResult, CircuitData, StaticMatrix, Vec<Value>), SimulationError> {
        self.run_pss_with_state_and_frozen_sources_abort(
            netlist,
            config,
            &std::collections::BTreeSet::new(),
            false,
            None,
            abort,
        )
    }

    fn run_pss_with_state_and_frozen_sources_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        frozen_sources: &std::collections::BTreeSet<String>,
        require_exact_continuation_state: bool,
        dc_seed: Option<&PssDcOperatingPointSeed>,
        abort: &dyn AbortSignal,
    ) -> Result<(PssAnalysisResult, CircuitData, StaticMatrix, Vec<Value>), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        config.validate().map_err(PssError::InvalidConfig)?;
        self.ensure_analysis_points(config.points_per_period)?;

        // Build and prepare circuit
        let mut circuit = self.build_circuit_with_abort(netlist, abort)?;
        Self::freeze_pss_independent_sources(&mut circuit, frozen_sources)?;
        Self::ensure_supported_xyce_memristor_small_signal(&circuit, "PSS")?;
        if require_exact_continuation_state {
            Self::ensure_pss_continuation_state_supported(&circuit)?;
        }
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Validate circuit has reactive elements
        let state_dimension = circuit.capacitors.len() + circuit.inductors.len();
        if state_dimension == 0 {
            return Err(PssError::NoReactiveElements.into());
        }
        self.ensure_result_values(
            config
                .points_per_period
                .saturating_mul(
                    circuit
                        .matrix_size()
                        .saturating_add(state_dimension)
                        .saturating_add(1),
                )
                .saturating_add(state_dimension.saturating_mul(state_dimension))
                .saturating_add(state_dimension.saturating_mul(2)),
        )?;

        // Use the exact retained operating point when one was supplied. The
        // basis check happens only after full circuit elaboration and matrix
        // construction, so stale or structurally tampered states fail closed.
        // No fresh DC solve is performed on this path.
        self.ensure_dc_paths_to_ground(&circuit)?;
        let dc_solution = match dc_seed {
            Some(seed) => {
                seed.validate_for_circuit(&circuit)?;
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let solution = seed.solution.clone();
                self.ensure_solved_dc_paths_to_ground(&mut circuit, &mut matrix, &solution)?;
                solution
            }
            None => {
                self.solve_dc_operating_point_with_abort(netlist, &mut circuit, &mut matrix, abort)?
            }
        };

        // Initialize capacitor/inductor state from DC
        self.pss_initialize_reactive_state(&mut circuit, &dc_solution);

        // ==================================================================
        // Phase 1: Stabilization (tstab)
        // ==================================================================
        let period = config.period();
        let (stabilized_waveform, current_state) =
            self.pss_run_stabilization(&mut circuit, &mut matrix, &dc_solution, &config, abort)?;

        // ==================================================================
        // Phase 2: Period Detection (for autonomous oscillators)
        // ==================================================================
        let mut detected_period = if config.is_autonomous() && config.auto_period {
            // Detection only seeds the Newton iteration (the period is now a
            // shooting unknown closed by the phase condition), so a low-
            // confidence detection falls back to the configured guess
            // instead of aborting the analysis.
            self.pss_detect_oscillator_period(&stabilized_waveform, &config)
                .unwrap_or_else(|_| config.period())
        } else {
            period
        };

        // ==================================================================
        // Phase 3: Shooting Newton Loop
        // ==================================================================
        // Finite difference step for Jacobian computation
        const FD_STEP: Value = 1e-8;

        let mut solver = ShootingNewtonSolver::new(config.tolerance, config.max_iterations)
            .with_abstol(config.abstol)
            .with_damping(config.damping_factor)
            .with_fd_step(FD_STEP);

        let mut shooting_state = ShootingState::new(current_state.clone(), detected_period);
        let mut final_waveform: Option<TransientResult> = None;
        let mut iteration = 0;
        // The most recent shooting Jacobian (J = M - I). At convergence it
        // doubles as the monodromy source, saving N+1 period integrations.
        let mut last_jacobian: Option<Vec<Vec<Value>>> = None;
        // A previously materialized Jacobian remains useful as a right
        // preconditioner even after a matrix-free step makes it too stale for
        // Floquet reporting.
        let mut preconditioner_jacobian: Option<Vec<Vec<Value>>> = None;

        while iteration < config.max_iterations {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // Simulate one period
            self.pss_set_reactive_state(&mut circuit, &shooting_state.x0);

            let (x_t, waveform) = self.pss_simulate_one_period(
                &mut circuit,
                &mut matrix,
                detected_period,
                &config,
                abort,
            )?;

            shooting_state.x_t = x_t;
            shooting_state.compute_residual();
            if config.verbose {
                log::debug!(
                    "PSS iteration {}: period={:.6e}s residual={:.6e}",
                    iteration,
                    detected_period,
                    shooting_state.residual_norm()
                );
            }

            // Check convergence
            if solver.check_convergence(&shooting_state) {
                final_waveform = Some(waveform);
                break;
            }

            // Compute Newton step using a finite-difference Jacobian whose
            // columns integrate perturbed periods in parallel on per-worker
            // circuit clones (pure per-column work — deterministic).
            if config.is_autonomous() {
                // Oscillators: the period is a Newton unknown alongside the
                // state, closed by a Poincare phase condition.
                let krylov_step = if shooting_state.dimension() >= PSS_KRYLOV_STATE_THRESHOLD {
                    if let Some(jacobian) = preconditioner_jacobian.as_deref() {
                        self.pss_compute_autonomous_newton_step_krylov(
                            &circuit,
                            &shooting_state,
                            detected_period,
                            &config,
                            FD_STEP,
                            jacobian,
                            abort,
                        )?
                    } else {
                        None
                    }
                } else {
                    None
                };
                let (delta, delta_t) = if let Some(step) = krylov_step {
                    // The materialized Jacobian predates this step and cannot
                    // be recycled as the converged monodromy.
                    last_jacobian = None;
                    if config.verbose {
                        log::debug!("PSS autonomous Newton-Krylov step accepted");
                    }
                    step
                } else {
                    let (delta, delta_t, jacobian) = self.pss_compute_autonomous_newton_step(
                        &circuit,
                        &shooting_state,
                        detected_period,
                        &config,
                        FD_STEP,
                        abort,
                    )?;
                    preconditioner_jacobian = Some(jacobian.clone());
                    last_jacobian = Some(jacobian);
                    (delta, delta_t)
                };
                shooting_state.update_x0(&delta, solver.damping);
                let max_dt = config.max_period_change * detected_period;
                detected_period += (solver.damping * delta_t).clamp(-max_dt, max_dt);
                shooting_state.period = detected_period;
            } else {
                let krylov_step = if shooting_state.dimension() >= PSS_KRYLOV_STATE_THRESHOLD {
                    if let Some(jacobian) = preconditioner_jacobian.as_deref() {
                        self.pss_compute_newton_step_krylov(
                            &circuit,
                            &shooting_state,
                            detected_period,
                            &config,
                            FD_STEP,
                            jacobian,
                            abort,
                        )?
                    } else {
                        None
                    }
                } else {
                    None
                };
                let delta = if let Some(delta) = krylov_step {
                    last_jacobian = None;
                    if config.verbose {
                        log::debug!("PSS driven Newton-Krylov step accepted");
                    }
                    delta
                } else {
                    let (delta, jacobian) = self.pss_compute_newton_step(
                        &circuit,
                        &shooting_state,
                        detected_period,
                        &config,
                        FD_STEP,
                        abort,
                    )?;
                    preconditioner_jacobian = Some(jacobian.clone());
                    last_jacobian = Some(jacobian);
                    delta
                };
                shooting_state.update_x0(&delta, solver.damping);
            }
            final_waveform = Some(waveform);

            iteration += 1;
        }

        // Check if we converged
        if !solver.has_converged() && iteration >= config.max_iterations {
            return Err(PssError::ConvergenceFailed {
                iterations: iteration,
                residual: shooting_state.residual_norm(),
            }
            .into());
        }

        // ==================================================================
        // Phase 4: Build Result
        // ==================================================================
        let waveform = final_waveform.unwrap();

        // Compute Floquet multipliers for stability analysis. The converged
        // shooting Jacobian already contains the monodromy (J = M - I), so
        // reuse it instead of re-integrating N+1 periods; the fresh-FD path
        // remains for the zero-iteration case (initial state was already
        // periodic, so no Jacobian was ever built).
        // Autonomous orbits get a FRESH monodromy at the converged (x0, T):
        // the structural unity Floquet multiplier exists only on the closed
        // orbit, so the recycled pre-convergence Jacobian (off-orbit, stale
        // period) is not accurate enough for stability classification there.
        let monodromy = if config.is_autonomous() {
            self.pss_set_reactive_state(&mut circuit, &shooting_state.x0);
            self.pss_compute_monodromy(
                &circuit,
                &shooting_state,
                detected_period,
                &config,
                FD_STEP,
                abort,
            )?
        } else {
            match last_jacobian {
                Some(jacobian) => monodromy_from_newton_jacobian(jacobian),
                None => self.pss_compute_monodromy(
                    &circuit,
                    &shooting_state,
                    detected_period,
                    &config,
                    FD_STEP,
                    abort,
                )?,
            }
        };
        let floquet_multipliers = solver.compute_floquet_multipliers(&monodromy);
        let is_stable = floquet_multipliers.iter().all(|m| m.norm() <= 1.0 + 1e-6);

        // Build PssResult
        let pss_result = self.pss_build_result(
            &waveform,
            detected_period,
            iteration,
            shooting_state.residual_norm(),
        );

        Ok((
            PssAnalysisResult {
                result: pss_result,
                iterations: iteration,
                final_residual: shooting_state.residual_norm(),
                period: detected_period,
                monodromy,
                floquet_multipliers,
                is_stable,
            },
            circuit,
            matrix,
            shooting_state.x0.clone(),
        ))
    }

    /// Initialize reactive element state from DC solution
    fn pss_initialize_reactive_state(&self, circuit: &mut CircuitData, dc_solution: &[Value]) {
        // Initialize capacitor voltages
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_dc = if np == 0 {
                0.0
            } else {
                dc_solution.get(np - 1).copied().unwrap_or(0.0)
            } - if nn == 0 {
                0.0
            } else {
                dc_solution.get(nn - 1).copied().unwrap_or(0.0)
            };
            circuit.capacitors.v_prev[cap_idx] = v_dc;
            circuit.capacitors.v_prev_prev[cap_idx] = v_dc;
        }

        // Initialize inductor currents
        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let br = circuit.inductors.branch_indices[l_idx];

            let v_dc = if np == 0 {
                0.0
            } else {
                dc_solution.get(np - 1).copied().unwrap_or(0.0)
            } - if nn == 0 {
                0.0
            } else {
                dc_solution.get(nn - 1).copied().unwrap_or(0.0)
            };
            circuit.inductors.v_prev[l_idx] = v_dc;

            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_dc = dc_solution.get(br_idx).copied().unwrap_or(0.0);
                circuit.inductors.i_prev[l_idx] = i_dc;
                circuit.inductors.i_prev_prev[l_idx] = i_dc;
                circuit.inductors.i_prev_prev_prev[l_idx] = i_dc;
            }
        }
    }

    /// Extract state vector (capacitor voltages + inductor currents)
    fn pss_extract_reactive_state(&self, circuit: &CircuitData) -> Vec<Value> {
        let mut state = Vec::with_capacity(circuit.capacitors.len() + circuit.inductors.len());

        // Capacitor voltages
        for v in &circuit.capacitors.v_prev {
            state.push(*v);
        }

        // Inductor currents
        for i in &circuit.inductors.i_prev {
            state.push(*i);
        }

        state
    }

    /// Set reactive element state from state vector
    ///
    /// Clears every other piece of companion history (capacitor currents,
    /// inductor voltages): the first step of each period integration runs
    /// backward Euler, which reads neither, so the period map becomes a pure
    /// function of the shooting state. Leaving stale history in place would
    /// leak the previous trajectory into the next one and corrupt both the
    /// shooting residual and the finite-difference Jacobian columns.
    pub(in crate::engine) fn pss_set_reactive_state(
        &self,
        circuit: &mut CircuitData,
        state: &[Value],
    ) {
        let n_caps = circuit.capacitors.len();

        // Set capacitor voltages
        for (i, v) in state.iter().take(n_caps).enumerate() {
            circuit.capacitors.v_prev[i] = *v;
            circuit.capacitors.v_prev_prev[i] = *v;
            circuit.capacitors.i_prev[i] = 0.0;
        }

        // Set inductor currents
        for (i, current) in state.iter().skip(n_caps).enumerate() {
            circuit.inductors.i_prev[i] = *current;
            circuit.inductors.i_prev_prev[i] = *current;
            circuit.inductors.i_prev_prev_prev[i] = *current;
            circuit.inductors.v_prev[i] = 0.0;
        }
    }

    /// Run stabilization phase (`tstab`)
    fn pss_run_stabilization(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        dc_solution: &[Value],
        config: &PssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, Vec<Value>), SimulationError> {
        let period = config.period();
        let tstab = config.effective_tstab();

        if tstab > 0.0 {
            let max_step = period / 50.0;
            let waveform = self.pss_run_tran_internal(
                circuit,
                matrix,
                dc_solution.to_vec(),
                tstab,
                max_step,
                false,
                None,
                config.integration_method,
                abort,
            )?;

            let final_state = self.pss_extract_reactive_state(circuit);
            Ok((waveform, final_state))
        } else {
            let initial_state = self.pss_extract_reactive_state(circuit);

            let waveform = TransientResult {
                time: vec![0.0],
                step_sizes: vec![0.0],
                voltages: (0..circuit.num_nodes())
                    .map(|i| vec![dc_solution.get(i).copied().unwrap_or(0.0)])
                    .collect(),
                branch_currents: Vec::new(),
                num_nodes: circuit.num_nodes(),
                branch_names: Vec::new(),
                node_names: circuit.node_names_sorted(),
                digital_traces: Vec::new(),
                real_traces: Vec::new(),
                device_op_traces: Vec::new(),
                store_traces: Vec::new(),
            };

            Ok((waveform, initial_state))
        }
    }

    /// Detect oscillator period from stabilized waveform
    fn pss_detect_oscillator_period(
        &self,
        waveform: &TransientResult,
        config: &PssConfig,
    ) -> Result<Value, SimulationError> {
        if waveform.voltages.is_empty() || waveform.time.len() < 10 {
            return Err(
                PssError::PeriodDetectionFailed("Insufficient waveform data".to_string()).into(),
            );
        }

        let waveform_index = if let Some(requested_node) = config.oscillator_node.as_deref() {
            waveform
                .node_names
                .iter()
                .position(|node| node.eq_ignore_ascii_case(requested_node.trim()))
                .ok_or_else(|| {
                    PssError::PeriodDetectionFailed(format!(
                        "oscillator node '{requested_node}' is not present in the solved circuit"
                    ))
                })?
        } else {
            0
        };
        let values = waveform.voltages.get(waveform_index).ok_or_else(|| {
            PssError::PeriodDetectionFailed(format!(
                "oscillator node waveform index {waveform_index} is unavailable"
            ))
        })?;
        let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);

        if (max_val - min_val).abs() < 1e-9 {
            return Err(PssError::PeriodDetectionFailed(
                "No oscillation detected (constant voltage)".to_string(),
            )
            .into());
        }

        let detector = PeriodDetector::with_guess(config.period());
        let estimate = detector.detect(&waveform.time, values);

        if estimate.confidence < 0.3 {
            return Err(PssError::PeriodDetectionFailed(format!(
                "Low confidence: {:.1}%",
                estimate.confidence * 100.0
            ))
            .into());
        }

        Ok(estimate.period)
    }

    /// Simulate one complete period
    fn pss_simulate_one_period(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        period: Value,
        config: &PssConfig,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, TransientResult), SimulationError> {
        let max_step = period / config.points_per_period as f64;

        // Node voltages consistent with the frozen reactive state: they seed
        // the first Newton solve and become the genuine t=0 waveform sample.
        let solution = self.pss_initial_node_solution(circuit, matrix, period, abort)?;

        // The period map must vary smoothly with the shooting state: run on
        // the fixed grid (see pss_run_tran_internal).
        let waveform = self.pss_run_tran_internal(
            circuit,
            matrix,
            solution,
            period,
            max_step,
            true,
            None,
            config.integration_method,
            abort,
        )?;

        let final_state = self.pss_extract_reactive_state(circuit);
        Ok((final_state, waveform))
    }

    /// Solve the network at t = 0 with the reactive state held frozen.
    ///
    /// A backward-Euler companion step of `period * 1e-9` pins each capacitor
    /// to its state voltage and each inductor to its state current (the
    /// resulting error equals the state drift over that tiny interval), while
    /// the nonlinear devices settle to node voltages consistent with that
    /// state. Without this, the waveform's first sample and the Newton seed
    /// were all zeros.
    pub(in crate::engine) fn pss_initial_node_solution(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        period: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let dt_freeze = period * 1e-9;
        let coeff = CompanionCoefficients::for_method(
            crate::numerics::integration::IntegrationMethod::BackwardEuler,
        );
        let start = vec![0.0; circuit.matrix_size()];

        match self.pss_newton_solve(circuit, matrix, &coeff, dt_freeze, dt_freeze, &start, abort)? {
            Some(solution) => Ok(solution),
            None => Ok(start),
        }
    }

    /// Compute the shooting sensitivity columns by CENTRAL differences on
    /// the fixed-grid period map: column j is
    /// `(F(x0 + h e_j) - F(x0 - h e_j)) / 2h` with `F = Phi - I` when
    /// `subtract_identity` is set (Newton Jacobian) or `F = Phi` otherwise
    /// (monodromy). The fixed integration grid makes Phi smooth in x0, so
    /// the O(h^2) central difference reaches derivative accuracy the
    /// adaptive-grid forward difference never could.
    ///
    /// Columns are pure functions of `(x0, j)`, so they parallelize across
    /// per-worker circuit clones with deterministic results. `CircuitData`
    /// is Send-but-not-Sync (Cell-based device caches), so the work is
    /// chunked AC-sweep-style: one owned clone per worker chunk, matrix
    /// rebuilt per worker (StaticMatrix holds factorization workspaces and
    /// is intentionally not Clone).
    fn pss_sensitivity_columns(
        &self,
        circuit: &CircuitData,
        x0: &[Value],
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        subtract_identity: bool,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let n = x0.len();
        if n == 0 {
            return Ok(Vec::new());
        }

        let column = |worker_circuit: &mut CircuitData,
                      worker_matrix: &mut StaticMatrix,
                      j: usize|
         -> Result<Vec<Value>, SimulationError> {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let h = fd_step * (1.0 + x0[j].abs());

            let mut x_plus = x0.to_vec();
            x_plus[j] += h;
            self.pss_set_reactive_state(worker_circuit, &x_plus);
            let (x_t_plus, _) =
                self.pss_simulate_one_period(worker_circuit, worker_matrix, period, config, abort)?;

            let mut x_minus = x0.to_vec();
            x_minus[j] -= h;
            self.pss_set_reactive_state(worker_circuit, &x_minus);
            let (x_t_minus, _) =
                self.pss_simulate_one_period(worker_circuit, worker_matrix, period, config, abort)?;

            Ok((0..n)
                .map(|i| {
                    let f_plus = if subtract_identity {
                        x_t_plus[i] - x_plus[i]
                    } else {
                        x_t_plus[i]
                    };
                    let f_minus = if subtract_identity {
                        x_t_minus[i] - x_minus[i]
                    } else {
                        x_t_minus[i]
                    };
                    (f_plus - f_minus) / (2.0 * h)
                })
                .collect())
        };

        #[cfg(feature = "parallel")]
        if n >= 2 {
            use rayon::prelude::*;

            let workers = self.parallel_worker_count(n);
            let chunk_len = n.div_ceil(workers);
            let indices: Vec<usize> = (0..n).collect();
            let work: Vec<(CircuitData, Vec<usize>)> = indices
                .chunks(chunk_len)
                .map(|chunk| (circuit.clone(), chunk.to_vec()))
                .collect();

            let chunk_columns: Result<Vec<Vec<Vec<Value>>>, SimulationError> = self
                .install_parallel(|| {
                    work.into_par_iter()
                        .map(|(mut worker_circuit, chunk)| {
                            let matrix = self.build_matrix(&worker_circuit)?;
                            worker_circuit.link_indices(&matrix);
                            let mut worker_matrix = matrix;
                            chunk
                                .into_iter()
                                .map(|j| column(&mut worker_circuit, &mut worker_matrix, j))
                                .collect()
                        })
                        .collect()
                })?;
            return chunk_columns.map(|chunks| chunks.into_iter().flatten().collect());
        }

        let mut worker_circuit = circuit.clone();
        let matrix = self.build_matrix(&worker_circuit)?;
        worker_circuit.link_indices(&matrix);
        let mut worker_matrix = matrix;
        (0..n)
            .map(|j| column(&mut worker_circuit, &mut worker_matrix, j))
            .collect()
    }

    /// Matrix-free directional derivative of the fixed-grid shooting map.
    /// The perturbation is scaled so its largest normalized state component
    /// matches the same central-difference step used by explicit columns.
    fn pss_directional_jacobian_product(
        &self,
        worker_circuit: &mut CircuitData,
        worker_matrix: &mut StaticMatrix,
        x0: &[Value],
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        direction: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let scaled_direction = direction
            .iter()
            .zip(x0)
            .map(|(value, state)| value.abs() / (1.0 + state.abs()))
            .fold(0.0_f64, Value::max);
        if scaled_direction == 0.0 {
            return Ok(vec![0.0; x0.len()]);
        }
        let epsilon = fd_step / scaled_direction;
        if !epsilon.is_finite() || epsilon <= 0.0 {
            return Err(SimulationError::Circuit(
                "PSS Newton-Krylov produced an invalid directional perturbation".to_string(),
            ));
        }

        let x_plus = x0
            .iter()
            .zip(direction)
            .map(|(state, vector)| state + epsilon * vector)
            .collect::<Vec<_>>();
        self.pss_set_reactive_state(worker_circuit, &x_plus);
        let (phi_plus, _) =
            self.pss_simulate_one_period(worker_circuit, worker_matrix, period, config, abort)?;

        let x_minus = x0
            .iter()
            .zip(direction)
            .map(|(state, vector)| state - epsilon * vector)
            .collect::<Vec<_>>();
        self.pss_set_reactive_state(worker_circuit, &x_minus);
        let (phi_minus, _) =
            self.pss_simulate_one_period(worker_circuit, worker_matrix, period, config, abort)?;

        Ok(phi_plus
            .iter()
            .zip(phi_minus)
            .zip(direction)
            .map(|((plus, minus), vector)| (plus - minus) / (2.0 * epsilon) - vector)
            .collect())
    }

    /// Try a preconditioned Newton-Krylov shooting step.  The exact Jacobian
    /// from the previous Newton point is an excellent local preconditioner;
    /// failure to meet the true inner residual returns `None` for a full
    /// central-column rebuild and direct solve.
    fn pss_compute_newton_step_krylov(
        &self,
        circuit: &CircuitData,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        preconditioner_jacobian: &[Vec<Value>],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        let Some(preconditioner) = PssDenseLu::factor(preconditioner_jacobian) else {
            return Ok(None);
        };
        let mut worker = circuit.clone();
        let worker_matrix = self.build_matrix(&worker)?;
        worker.link_indices(&worker_matrix);
        let mut worker_matrix = worker_matrix;
        let rhs = state
            .residual
            .iter()
            .map(|value| -value)
            .collect::<Vec<_>>();
        let mut matvec = |direction: &[Value]| {
            self.pss_directional_jacobian_product(
                &mut worker,
                &mut worker_matrix,
                &state.x0,
                period,
                config,
                fd_step,
                direction,
                abort,
            )
        };
        pss_gmres(
            &mut matvec,
            &preconditioner,
            &rhs,
            state.dimension().min(24),
            4,
        )
    }

    /// Autonomous counterpart of the matrix-free shooting step.  The period
    /// column and Poincare phase row are formed once; only the state block is
    /// evaluated through directional period-map products.
    fn pss_compute_autonomous_newton_step_krylov(
        &self,
        circuit: &CircuitData,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        preconditioner_jacobian: &[Vec<Value>],
        abort: &dyn AbortSignal,
    ) -> Result<Option<(Vec<Value>, Value)>, SimulationError> {
        let n = state.dimension();
        let h_t = period * 1e-7;
        let mut worker = circuit.clone();
        let worker_matrix = self.build_matrix(&worker)?;
        worker.link_indices(&worker_matrix);
        let mut worker_matrix = worker_matrix;
        self.pss_set_reactive_state(&mut worker, &state.x0);
        let (phi_plus_t, _) = self.pss_simulate_one_period(
            &mut worker,
            &mut worker_matrix,
            period + h_t,
            config,
            abort,
        )?;
        let dphi_dt = (0..n)
            .map(|index| (phi_plus_t[index] - state.x_t[index]) / h_t)
            .collect::<Vec<_>>();

        let mut augmented = vec![vec![0.0; n + 1]; n + 1];
        for row in 0..n {
            for column in 0..n {
                augmented[row][column] = preconditioner_jacobian[row][column];
            }
            augmented[row][n] = dphi_dt[row];
            augmented[n][row] = dphi_dt[row];
        }
        let Some(preconditioner) = PssDenseLu::factor(&augmented) else {
            return Ok(None);
        };
        let mut rhs = state
            .residual
            .iter()
            .map(|value| -value)
            .collect::<Vec<_>>();
        rhs.push(0.0);
        let mut matvec = |direction: &[Value]| {
            let state_direction = &direction[..n];
            let mut image = self.pss_directional_jacobian_product(
                &mut worker,
                &mut worker_matrix,
                &state.x0,
                period,
                config,
                fd_step,
                state_direction,
                abort,
            )?;
            for row in 0..n {
                image[row] += dphi_dt[row] * direction[n];
            }
            image.push(pss_dot(&dphi_dt, state_direction));
            Ok(image)
        };
        Ok(
            pss_gmres(&mut matvec, &preconditioner, &rhs, (n + 1).min(24), 4)?
                .map(|solution| (solution[..n].to_vec(), solution[n])),
        )
    }

    /// Compute Newton step using a finite-difference shooting Jacobian.
    ///
    /// One period integration per state (the unperturbed trajectory is the
    /// shared base point), with columns evaluated in parallel on per-worker
    /// circuit clones. Returns the step together with the Jacobian so the
    /// caller can recycle it as the monodromy at convergence (J = M - I).
    fn pss_compute_newton_step(
        &self,
        circuit: &CircuitData,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(Vec<Value>, Vec<Vec<Value>>), SimulationError> {
        let n = state.dimension();
        let columns =
            self.pss_sensitivity_columns(circuit, &state.x0, period, config, fd_step, true, abort)?;

        let mut jacobian = vec![vec![0.0; n]; n];
        for (j, column) in columns.iter().enumerate() {
            for i in 0..n {
                jacobian[i][j] = column[i];
            }
        }

        // Solve: (J) * delta = -f0 using simple Gaussian elimination
        let delta = self.pss_solve_linear_system(&jacobian, &state.residual)?;

        Ok((delta, jacobian))
    }

    /// Newton step for AUTONOMOUS shooting: the period T joins the state as
    /// an unknown, closed by the Poincare phase condition f(x(T))^T ds = 0
    /// (no update component along the orbit, where the period map is
    /// neutrally stable). The augmented system is
    ///
    /// ```text
    /// [ M - I        dPhi/dT ] [ds]   [-r]
    /// [ f(x(T))^T    0       ] [dT] = [ 0]
    /// ```
    ///
    /// with dPhi/dT differenced in T on the fixed grid (the step count stays
    /// constant, so the map is smooth in T) and f(x(T)) = dPhi/dT itself,
    /// the orbit tangent at the endpoint.
    fn pss_compute_autonomous_newton_step(
        &self,
        circuit: &CircuitData,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<AutonomousNewtonStep, SimulationError> {
        let n = state.dimension();
        let columns =
            self.pss_sensitivity_columns(circuit, &state.x0, period, config, fd_step, true, abort)?;

        // dPhi/dT by forward difference; Phi_T(x0) is already in state.x_t.
        let h_t = period * 1e-7;
        let mut worker = circuit.clone();
        let m = self.build_matrix(&worker)?;
        worker.link_indices(&m);
        let mut worker_matrix = m;
        self.pss_set_reactive_state(&mut worker, &state.x0);
        let (x_t_plus, _) = self.pss_simulate_one_period(
            &mut worker,
            &mut worker_matrix,
            period + h_t,
            config,
            abort,
        )?;
        let dphi_dt: Vec<Value> = (0..n).map(|i| (x_t_plus[i] - state.x_t[i]) / h_t).collect();

        let mut jacobian = vec![vec![0.0; n + 1]; n + 1];
        for (j, column) in columns.iter().enumerate() {
            for i in 0..n {
                jacobian[i][j] = column[i];
            }
        }
        for i in 0..n {
            jacobian[i][n] = dphi_dt[i];
            jacobian[n][i] = dphi_dt[i]; // phase row: orbit tangent
        }

        let mut rhs = state.residual.clone();
        rhs.push(0.0); // phase condition has zero residual by construction

        let solution = self.pss_solve_linear_system(&jacobian, &rhs)?;
        let delta_state = solution[..n].to_vec();
        let delta_t = solution[n];

        // Monodromy reuse: the top-left block is M - I at the current T.
        let mut state_jacobian = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                state_jacobian[i][j] = jacobian[i][j];
            }
        }

        Ok((delta_state, delta_t, state_jacobian))
    }

    /// Compute the monodromy matrix via finite differences — the fallback
    /// for the zero-Newton-iteration case (otherwise the converged Newton
    /// Jacobian is recycled as J + I). The converged trajectory endpoint
    /// `state.x_t` is the shared base; columns run in parallel on private
    /// circuit clones.
    fn pss_compute_monodromy(
        &self,
        circuit: &CircuitData,
        state: &ShootingState,
        period: Value,
        config: &PssConfig,
        fd_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Vec<Value>>, SimulationError> {
        let n = state.dimension();
        let columns = self
            .pss_sensitivity_columns(circuit, &state.x0, period, config, fd_step, false, abort)?;

        let mut monodromy = vec![vec![0.0; n]; n];
        for (j, column) in columns.iter().enumerate() {
            for i in 0..n {
                monodromy[i][j] = column[i];
            }
        }

        Ok(monodromy)
    }

    /// Solve linear system using Gaussian elimination
    pub(in crate::engine) fn pss_solve_linear_system(
        &self,
        a: &[Vec<Value>],
        b: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        let n = b.len();
        if n == 0 {
            return Ok(vec![]);
        }

        // Augmented matrix
        let mut aug: Vec<Vec<Value>> = a
            .iter()
            .zip(b.iter())
            .map(|(row, &bi)| {
                let mut r = row.clone();
                r.push(-bi); // Solve Ax = -b (Newton step)
                r
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            for row in (col + 1)..n {
                if aug[row][col].abs() > aug[max_row][col].abs() {
                    max_row = row;
                }
            }
            aug.swap(col, max_row);

            let pivot = aug[col][col];
            if pivot.abs() < 1e-15 {
                // Near-singular, use regularization
                continue;
            }

            // Eliminate
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for k in col..=n {
                    aug[row][k] -= factor * aug[col][k];
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            if aug[i][i].abs() > 1e-15 {
                x[i] = sum / aug[i][i];
            }
        }

        Ok(x)
    }

    /// One companion-stamped Newton solve at `t_next` with step `dt`.
    ///
    /// Stamps the linear network, time-varying sources, reactive companions
    /// for the given integration coefficients, and nonlinear devices, then
    /// iterates to convergence. Reads — never writes — the companion history,
    /// so callers control when the trajectory actually advances. Returns
    /// `None` when Newton fails to converge at this step size.
    fn pss_newton_solve(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        coeff: &CompanionCoefficients,
        t_next: Value,
        dt: Value,
        start: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Option<Vec<Value>>, SimulationError> {
        let size = circuit.matrix_size();
        let mut new_solution = start.to_vec();
        let mut rhs = vec![0.0; size];
        let mut proposal = Vec::with_capacity(size);

        for _iter in 0..self.config.max_iterations {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            self.pss_stamp_system(circuit, matrix, &mut rhs, coeff, t_next, dt, &new_solution)?;

            match matrix.solve_into(&rhs, &mut proposal) {
                Ok(()) => {
                    let voltage_converged = self.node_voltage_convergence_met(
                        &new_solution,
                        &proposal,
                        circuit.num_nodes(),
                    );
                    let linearized_residual_converged =
                        self.residual_convergence_met(circuit, matrix, &proposal, &rhs);

                    std::mem::swap(&mut new_solution, &mut proposal);

                    if circuit.has_nonlinear_devices() {
                        circuit.update_nonlinear(&new_solution);
                    }

                    let device_converged = !circuit.has_nonlinear_devices()
                        || circuit.nonlinear_converged(self.device_convergence_criteria());

                    if voltage_converged && device_converged && linearized_residual_converged {
                        return Ok(Some(new_solution));
                    }
                }
                Err(_) => return Ok(None),
            }
        }

        Ok(None)
    }

    /// Stamp the full companion-linearized system at one time point:
    /// linear network, time-varying sources, reactive companions for the
    /// given coefficients, and the nonlinear Jacobian linearized at
    /// `linearize_at`. Shared by the Newton iteration and by the
    /// injection-sensitivity solves of the oscillator noise machinery,
    /// which ignore the RHS and solve the stamped matrix against unit
    /// current injections.
    #[allow(clippy::too_many_arguments)]
    pub(in crate::engine) fn pss_stamp_system(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        coeff: &CompanionCoefficients,
        t_next: Value,
        dt: Value,
        linearize_at: &[Value],
    ) -> Result<(), SimulationError> {
        matrix.clear_values();
        rhs.fill(0.0);

        // PSS needs a nodal conductance floor, not a perturbation of dynamic
        // branch equations. In particular, adding 1e-12 to a scaled capacitor
        // branch diagonal can cancel its physical -1/geq coefficient.
        Self::stamp_nodal_gmin(circuit, matrix, 1e-12);

        circuit.stamp_transient_linear_direct(matrix, rhs);

        // Time-varying independent sources: the static stamp wrote DC
        // values; overwrite the source rows with their value at the
        // end of this step. This is what makes driven PSS periodic —
        // without it a SIN drive stamps as its DC offset and the
        // "steady state" collapses to the DC solution.
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, t_next, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, t_next);

        // Reuse the transient capacitor companion so shooting PSS has exactly
        // the same branch-current convention and numerical scaling as TRAN.
        circuit
            .capacitors
            .stamp_transient_companion(matrix, rhs, dt, coeff, num_nodes);

        // Stamp inductors
        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let br = circuit.inductors.branch_indices[l_idx];
            let inductance = circuit.inductors.inductances[l_idx];
            let i_n = circuit.inductors.i_prev[l_idx];
            let i_n_minus_1 = circuit.inductors.i_prev_prev[l_idx];
            let v_n = circuit.inductors.v_prev[l_idx];

            let req = coeff.inductor_req(inductance, dt);
            let veq = coeff.inductor_veq(inductance, dt, i_n, i_n_minus_1, v_n);

            if np > 0 && br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                matrix.add(br_idx, np - 1, 1.0);
                matrix.add(np - 1, br_idx, 1.0);
            }
            if nn > 0 && br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                matrix.add(br_idx, nn - 1, -1.0);
                matrix.add(nn - 1, br_idx, -1.0);
            }
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                matrix.add(br_idx, br_idx, -req);
                // Branch row sign convention: v - r_eq*i = -v_eq (see
                // Inductors::stamp_transient_companion).
                rhs[br_idx] = -veq;
            }
        }

        // Mutual coupling overlays on top of the standalone inductors.
        circuit.stamp_coupled_inductor_pairs_transient(matrix, rhs, dt, coeff);

        if circuit.has_nonlinear_devices() {
            circuit.update_nonlinear(linearize_at);
            circuit
                .stamp_nonlinear(matrix, rhs, linearize_at)
                .map_err(SimulationError::Circuit)?;
            circuit
                .stamp_behavioral(
                    matrix,
                    rhs,
                    linearize_at,
                    t_next,
                    crate::xspice::AnalysisType::Transient,
                )
                .map_err(SimulationError::Circuit)?;
        }
        Ok(())
    }

    /// Internal transient simulation
    /// `fixed_grid` integrates on a uniform time grid with a deterministic
    /// method sequence (backward Euler first step, then the configured method;
    /// TrapGear resolves to trapezoidal on the fixed grid): the
    /// period map then varies SMOOTHLY with the initial state, which is what
    /// makes finite-difference shooting Jacobians and monodromy columns
    /// accurate — an LTE-adaptive grid changes its step decisions
    /// discontinuously under perturbation and floors the achievable
    /// derivative accuracy.
    pub(in crate::engine) fn pss_run_tran_internal(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        mut solution: Vec<Value>,
        tstop: Value,
        max_step: Value,
        fixed_grid: bool,
        mut trace: Option<&mut PssStateTrace>,
        integration_method: Option<IntegrationMethod>,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !tstop.is_finite() || tstop <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "PSS transient traversal stop time must be finite and positive, got {tstop:e}"
            )));
        }
        if !max_step.is_finite() || max_step <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "PSS transient traversal maximum step must be finite and positive, got {max_step:e}"
            )));
        }
        let num_nodes = circuit.num_nodes();

        let fixed_steps = (tstop / max_step).round().max(1.0) as usize;
        let fixed_dt = tstop / fixed_steps as Value;

        let initial_step = (max_step / 10.0).min(tstop / 100.0);
        let mut timestep =
            TimestepController::new(initial_step, self.config.min_timestep, max_step);
        // Register source-waveform breakpoints (PULSE edges, PWL corners,
        // SIN delay starts) so the integrator lands on them instead of
        // stepping across; without this, hard-edged drives shift the PSS
        // orbit by up to one LTE-sized step per edge.
        let mut breakpoints = BreakpointManager::new();
        Self::collect_transient_source_breakpoints(
            circuit,
            tstop,
            max_step,
            self.config.spice_dialect,
            &mut breakpoints,
            abort,
            self.config.resource_limits.max_analysis_points,
        )?;
        let mut lte_estimator =
            LteEstimator::with_tolerances(self.voltage_reltol(), self.voltage_abstol());
        let mut trapgear = TrapGearController::new();

        let node_names = circuit.node_names_sorted();
        let mut result = TransientResult {
            time: vec![0.0],
            step_sizes: vec![0.0],
            voltages: (0..num_nodes)
                .map(|i| vec![solution.get(i).copied().unwrap_or(0.0)])
                .collect(),
            branch_currents: Vec::new(),
            num_nodes,
            branch_names: Vec::new(),
            node_names,
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
        };

        let mut t = 0.0;
        const MAX_ITERATIONS: usize = 100_000;
        let mut total_iterations = 0;
        let mut first_step = true;
        let mut accepted_step_history = PssAcceptedStepHistory::default();

        if let Some(tr) = trace.as_deref_mut() {
            tr.times.push(0.0);
            tr.states.push(self.pss_extract_reactive_state(circuit));
            tr.solutions.push(solution.clone());
        }

        let mut fixed_index = 0usize;
        while t < tstop && total_iterations < MAX_ITERATIONS {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            total_iterations += 1;
            let (dt, t_next) = if fixed_grid {
                if fixed_index >= fixed_steps {
                    return Err(SimulationError::Circuit(format!(
                        "PSS fixed-grid traversal exhausted {fixed_steps} scheduled steps at t={t:.17e} before the exact stop t={tstop:.17e}"
                    )));
                }
                // Anchor each step to the grid so rounding cannot drift the
                // endpoint. The final target is assigned from `tstop`
                // directly rather than reconstructed by multiplication.
                let next_index = fixed_index + 1;
                let t_next = if next_index == fixed_steps {
                    tstop
                } else {
                    next_index as Value * fixed_dt
                };
                (t_next - t, t_next)
            } else {
                let remaining = tstop - t;
                let (limited_dt, _) = breakpoints.limit_step(t, timestep.dt());
                let dt = limited_dt.min(remaining);
                let t_next = if dt == remaining { tstop } else { t + dt };
                (dt, t_next)
            };
            if !dt.is_finite() || dt <= 0.0 || !t_next.is_finite() || t_next <= t {
                return Err(SimulationError::Circuit(format!(
                    "PSS transient traversal produced an invalid step dt={dt:.17e} from t={t:.17e} toward tstop={tstop:.17e}"
                )));
            }

            // First step runs backward Euler: it reads no capacitor-current or
            // inductor-voltage history, so the trajectory depends only on the
            // shooting state that pss_set_reactive_state installed.
            let current_method = pss_integration_method(
                first_step,
                fixed_grid,
                integration_method,
                trapgear.current_method(),
            );
            let coeff = accepted_step_history.coefficients_for_trial(current_method, dt);

            let Some(new_solution) =
                self.pss_newton_solve(circuit, matrix, &coeff, t_next, dt, &solution, abort)?
            else {
                if fixed_grid {
                    // The grid is the contract: a Newton failure on it is a
                    // hard error rather than a silent step change that would
                    // destroy map smoothness.
                    return Err(SimulationError::ConvergenceFailed(total_iterations));
                }
                timestep.force_step(dt * 0.25);
                continue;
            };
            if fixed_grid {
                fixed_index += 1;
            }

            t = t_next;
            first_step = false;

            // Update capacitor history with the same companion that built this
            // step. IC capacitors own a solved physical-current branch;
            // ordinary capacitors retain the Norton reconstruction.
            for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                    - if nn == 0 { 0.0 } else { new_solution[nn - 1] };

                circuit.capacitors.i_prev[cap_idx] =
                    if let Some(branch_ordinal) = circuit.capacitors.ic_branch_indices[cap_idx] {
                        new_solution[num_nodes + branch_ordinal - 1]
                    } else {
                        let capacitance = circuit.capacitors.capacitances[cap_idx];
                        let geq = coeff.capacitor_geq(capacitance, dt);
                        let ieq = coeff.capacitor_ieq(
                            capacitance,
                            dt,
                            circuit.capacitors.v_prev[cap_idx],
                            circuit.capacitors.v_prev_prev[cap_idx],
                            circuit.capacitors.i_prev[cap_idx],
                        );
                        geq * v_new - ieq
                    };
                circuit.capacitors.v_prev_prev[cap_idx] = circuit.capacitors.v_prev[cap_idx];
                circuit.capacitors.v_prev[cap_idx] = v_new;
            }

            // Evaluate the candidate against history from previously accepted
            // points before rotating that history. `recommend_scale` already
            // returns the multiplicative dt ratio; feeding `lte / scale` into
            // `TimestepController::adjust` applies a second, unrelated error
            // controller and can ratchet a smooth RC/RL stabilization down to
            // the hard minimum timestep. PSS stabilization accepts every
            // Newton-converged point, so retain the point and use the LTE only
            // to size the *next* interval.
            let accepted_step_scale = if fixed_grid {
                None
            } else {
                let (lte, _) = lte_estimator.estimate(&new_solution, dt);
                Some(lte_estimator.recommend_scale(lte))
            };
            lte_estimator.record(&new_solution, dt);
            trapgear.update(&new_solution, dt);

            // Update inductor history
            for l_idx in 0..circuit.inductors.names.len() {
                let br = circuit.inductors.branch_indices[l_idx];
                if br > 0 {
                    let br_idx = circuit.num_nodes() + br - 1;
                    let i_new = new_solution[br_idx];
                    circuit.inductors.i_prev_prev_prev[l_idx] = circuit.inductors.i_prev_prev[l_idx];
                    circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                    circuit.inductors.i_prev[l_idx] = i_new;

                    let np = circuit.inductors.node_pos[l_idx];
                    let nn = circuit.inductors.node_neg[l_idx];
                    let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                        - if nn == 0 { 0.0 } else { new_solution[nn - 1] };
                    circuit.inductors.v_prev[l_idx] = v_new;
                }
            }

            solution = new_solution;
            accepted_step_history.accept(dt);

            if let Some(tr) = trace.as_deref_mut() {
                tr.times.push(t);
                tr.states.push(self.pss_extract_reactive_state(circuit));
                tr.solutions.push(solution.clone());
            }

            result.time.push(t);
            for (i, voltages) in result.voltages.iter_mut().enumerate() {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
            }

            if let Some(scale) = accepted_step_scale {
                // The estimator bounds its recommendation to [0.25, 2.0].
                // Anchor the proposal to the interval that was actually
                // accepted, which is important when a source breakpoint made
                // that interval shorter than the controller's prior proposal.
                timestep.force_step(dt * scale);
            }
        }

        ensure_pss_traversal_complete(
            t,
            tstop,
            total_iterations,
            MAX_ITERATIONS,
            fixed_grid,
            fixed_index,
            fixed_steps,
            result.time.last().copied(),
        )?;

        Ok(result)
    }

    /// Build PssResult from transient waveform
    fn pss_build_result(
        &self,
        waveform: &TransientResult,
        period: Value,
        iterations: usize,
        residual_norm: Value,
    ) -> PssResult {
        let n_nodes = waveform.num_nodes;
        let n_points = waveform.time.len();

        let mut result = PssResult::new(period, n_nodes, n_points);
        result.time = waveform.time.clone();
        result.iterations = iterations;
        result.residual_norm = residual_norm;
        result.node_names = waveform.node_names.clone();
        result.period_detected = true;

        for (i, wf) in result.waveforms.iter_mut().enumerate() {
            if i < waveform.voltages.len() {
                *wf = PeriodicWaveform::from_values(waveform.voltages[i].clone());
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shooting_gmres_matches_direct_lu_with_stale_jacobian_preconditioner() {
        let n = 20;
        let mut current = vec![vec![0.0; n]; n];
        let mut previous = vec![vec![0.0; n]; n];
        for row in 0..n {
            current[row][row] = 3.0 + row as Value * 0.02;
            previous[row][row] = current[row][row] * 0.98;
            if row > 0 {
                current[row][row - 1] = -0.35;
                previous[row][row - 1] = -0.34;
            }
            if row + 1 < n {
                current[row][row + 1] = 0.2;
                previous[row][row + 1] = 0.19;
            }
        }
        let preconditioner = PssDenseLu::factor(&previous).unwrap();
        let rhs = (0..n)
            .map(|index| 0.4 - index as Value * 0.013)
            .collect::<Vec<_>>();
        let mut product = |vector: &[Value]| {
            Ok(current
                .iter()
                .map(|row| row.iter().zip(vector).map(|(a, b)| a * b).sum())
                .collect())
        };
        let iterative = pss_gmres(&mut product, &preconditioner, &rhs, 12, 4)
            .unwrap()
            .expect("preconditioned shooting GMRES should converge");
        let direct_factor = PssDenseLu::factor(&current).unwrap();
        let direct = direct_factor.solve(&rhs);
        for (actual, expected) in iterative.into_iter().zip(direct) {
            let scale = actual.abs().max(expected.abs()).max(1.0);
            assert!((actual - expected).abs() <= 2e-9 * scale);
        }
    }

    fn retained_parts() -> (PssConfig, PssAnalysisResult, Vec<Value>) {
        let config = PssConfig::new(1.0)
            .with_harmonics(4)
            .with_points_per_period(16);
        let time = (0..=16)
            .map(|index| index as Value / 16.0)
            .collect::<Vec<_>>();
        let waveform = time
            .iter()
            .map(|time| (2.0 * std::f64::consts::PI * time).sin())
            .collect();
        let result = PssResult {
            period: 1.0,
            frequency: 1.0,
            iterations: 2,
            residual_norm: 1.0e-10,
            time,
            waveforms: vec![PeriodicWaveform::from_values(waveform)],
            node_names: vec!["out".to_owned()],
            period_detected: false,
            floquet_multipliers: Vec::new(),
        };
        (
            config,
            PssAnalysisResult {
                result,
                iterations: 2,
                final_residual: 1.0e-10,
                period: 1.0,
                monodromy: Vec::new(),
                floquet_multipliers: Vec::new(),
                is_stable: true,
            },
            Vec::new(),
        )
    }

    fn assert_close(actual: Value, expected: Value) {
        let tolerance = 32.0 * Value::EPSILON * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "expected {expected:.17e}, got {actual:.17e}"
        );
    }

    #[test]
    fn pss_integration_override_is_honored_without_state_dependent_fixed_grid_switching() {
        assert_eq!(
            pss_integration_method(
                true,
                true,
                Some(IntegrationMethod::Gear2),
                IntegrationMethod::Gear2,
            ),
            IntegrationMethod::BackwardEuler
        );
        assert_eq!(
            pss_integration_method(
                false,
                true,
                Some(IntegrationMethod::Gear2),
                IntegrationMethod::BackwardEuler,
            ),
            IntegrationMethod::Gear2
        );
        assert_eq!(
            pss_integration_method(
                false,
                true,
                Some(IntegrationMethod::TrapGear),
                IntegrationMethod::Gear2,
            ),
            IntegrationMethod::Trapezoidal
        );
        assert_eq!(
            pss_integration_method(
                false,
                false,
                Some(IntegrationMethod::TrapGear),
                IntegrationMethod::Gear2,
            ),
            IntegrationMethod::Gear2
        );
        assert_eq!(
            pss_integration_method(false, true, None, IntegrationMethod::Gear2),
            IntegrationMethod::Trapezoidal
        );
    }

    #[test]
    fn adaptive_pss_gear2_uses_only_the_previous_accepted_timestep() {
        let mut history = PssAcceptedStepHistory::default();

        history.accept(1.0);
        let rejected_trial = history.coefficients_for_trial(IntegrationMethod::Gear2, 2.0);
        assert_close(rejected_trial.coeff_g, 5.0 / 3.0);
        assert_close(rejected_trial.coeff_v_n, 3.0);
        assert_close(rejected_trial.coeff_v_n_minus_1, -4.0 / 3.0);

        // Merely constructing coefficients for the rejected 2x trial cannot
        // rotate the accepted history. Its 0.5x retry still compares against
        // the original accepted 1.0 interval.
        assert_eq!(history.previous_accepted_dt, Some(1.0));
        let retry = history.coefficients_for_trial(IntegrationMethod::Gear2, 0.5);
        assert_close(retry.coeff_g, 4.0 / 3.0);
        assert_close(retry.coeff_v_n, 1.5);
        assert_close(retry.coeff_v_n_minus_1, -1.0 / 6.0);

        history.accept(0.5);
        let next_trial = history.coefficients_for_trial(IntegrationMethod::Gear2, 1.0);
        assert_close(next_trial.coeff_g, 5.0 / 3.0);
        assert_close(next_trial.coeff_v_n, 3.0);
        assert_close(next_trial.coeff_v_n_minus_1, -4.0 / 3.0);
    }

    #[test]
    fn adaptive_pss_gear2_without_accepted_timestep_history_restarts_at_order_one() {
        let history = PssAcceptedStepHistory::default();
        let coefficients = history.coefficients_for_trial(IntegrationMethod::Gear2, 2.0);
        let backward_euler = CompanionCoefficients::backward_euler();

        assert_eq!(coefficients.coeff_g, backward_euler.coeff_g);
        assert_eq!(coefficients.coeff_v_n, backward_euler.coeff_v_n);
        assert_eq!(
            coefficients.coeff_v_n_minus_1,
            backward_euler.coeff_v_n_minus_1
        );
        assert!(!coefficients.needs_two_history);
    }

    #[test]
    fn retained_driven_orbit_without_reactive_state_is_valid_and_reports_nyquist_capacity() {
        let (config, analysis, shooting_state) = retained_parts();
        let operating_point =
            PssOperatingPoint::try_from_parts(config, analysis, shooting_state).unwrap();
        assert!(operating_point.shooting_state().is_empty());
        assert_eq!(operating_point.spectral_harmonic_capacity(), 8);
    }

    #[test]
    fn retained_operating_point_rejects_structurally_tampered_numerical_evidence() {
        let (config, mut analysis, shooting_state) = retained_parts();
        analysis.result.frequency = 2.0;
        assert!(
            PssOperatingPoint::try_from_parts(config.clone(), analysis, shooting_state.clone())
                .is_err()
        );

        let (_, mut analysis, _) = retained_parts();
        analysis.result.node_names[0].clear();
        assert!(
            PssOperatingPoint::try_from_parts(config.clone(), analysis, shooting_state.clone())
                .is_err()
        );

        let (_, mut analysis, _) = retained_parts();
        analysis.result.floquet_multipliers = vec![num_complex::Complex64::new(Value::NAN, 0.0)];
        assert!(
            PssOperatingPoint::try_from_parts(config.clone(), analysis, shooting_state.clone())
                .is_err()
        );

        let (_, mut analysis, _) = retained_parts();
        analysis.result.time[8] = 2.0;
        assert!(PssOperatingPoint::try_from_parts(config, analysis, shooting_state).is_err());
    }

    #[test]
    fn continuation_state_rejects_unadvanced_delay_history() {
        let mut circuit = CircuitData::new();
        circuit.tlines.push(crate::device::TransmissionLine::new(
            "T1".to_string(),
            1,
            0,
            2,
            0,
            50.0,
            1.0e-9,
        ));

        let error = Engine::ensure_pss_continuation_state_supported(&circuit)
            .expect_err("transmission-line continuation must fail closed");
        assert!(
            error
                .to_string()
                .contains("transmission-line delay history")
        );
    }

    #[test]
    fn pss_iteration_guard_refuses_a_partial_trajectory() {
        let error =
            ensure_pss_traversal_complete(0.75, 1.0, 100_000, 100_000, false, 0, 0, Some(0.75))
                .expect_err("the traversal guard must never publish a partial result");
        let message = error.to_string();
        assert!(message.contains("hard 100000-iteration guard"));
        assert!(message.contains("refusing to publish a partial trajectory"));
    }

    #[test]
    fn pss_traversal_requires_the_exact_retained_endpoint() {
        let rounded = Value::from_bits(1.0_f64.to_bits() - 1);
        let error =
            ensure_pss_traversal_complete(rounded, 1.0, 32, 100_000, true, 32, 32, Some(rounded))
                .expect_err("a nearby floating endpoint is not an exact completed traversal");
        assert!(
            error
                .to_string()
                .contains("exhausted the 32-step fixed grid")
        );

        ensure_pss_traversal_complete(1.0, 1.0, 32, 100_000, true, 32, 32, Some(1.0))
            .expect("the exact retained endpoint is complete");
    }

    #[test]
    fn pss_continuation_recheck_detects_changed_external_dependency_bytes() {
        let path = std::env::temp_dir().join(format!(
            "rspice-pss-identity-unit-{}.csv",
            std::process::id()
        ));
        std::fs::write(&path, "0,0\n1e-6,1\n").expect("temporary PWL file is writable");
        let path_text = path.to_string_lossy().replace('\\', "/");
        let netlist = Netlist::parse(&format!(
            "* external identity test\nV1 out 0 PWL FILE=\"{path_text}\"\nC1 out 0 1p\n.end\n"
        ))
        .expect("external PWL dependency parses");
        let authenticated = Engine::pss_continuation_netlist_identity(&netlist)
            .expect("pre-build dependency snapshot is authenticated");

        // Change both bytes and length so this remains robust on filesystems
        // with coarse metadata granularity or aggressive read caching.
        std::fs::write(&path, "0,0\n1e-6,200\n").expect("temporary PWL file can change");
        let error = Engine::ensure_pss_continuation_netlist_identity(
            &netlist,
            &authenticated,
            "the test traversal",
        )
        .expect_err("changed external bytes must invalidate the pre-build snapshot");
        let _ = std::fs::remove_file(path);
        let message = error.to_string();
        assert!(message.contains("changed during the test traversal"));
        assert!(message.contains("external-file content"));
    }
}

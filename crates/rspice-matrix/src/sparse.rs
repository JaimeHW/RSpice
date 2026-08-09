//! High-performance sparse matrix solver using faer
//!
//! Uses faer's sparse LU decomposition for circuit simulation.
//! Provides O(n) scaling for typical circuit matrices.
//!
//! Key optimization: Static structure matrix that caches topology
//! and allows updates to values only, avoiding O(N log N) rebuild.

#![allow(clippy::needless_range_loop)]
use crate::{
    CircuitLuOrientation, CircuitLuRobustness, CircuitLuRowScaling, DivisionPolicy,
    FactorizationRequest, NumericFactorizationPolicy, RealSolverBackend, SolverError,
    SolverOptions, Value,
};
use faer::dyn_stack::{MemBuffer, MemStack};
use faer::sparse::linalg::LuError;
use faer::sparse::linalg::lu as sparse_lu;
use faer::sparse::{FaerError, SparseColMat, SparseColMatRef, SymbolicSparseColMat};
use faer::{Conj, Mat, get_global_parallelism};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[inline]
fn map_faer_error(error: FaerError) -> SolverError {
    match error {
        FaerError::OutOfMemory => SolverError::OutOfMemory,
        FaerError::IndexOverflow => SolverError::InvalidCircuit(
            "sparse matrix indices exceed the numeric backend's supported range".to_string(),
        ),
        _ => SolverError::InvalidCircuit("unsupported sparse factorization failure".to_string()),
    }
}

#[inline]
fn map_faer_lu_error(error: LuError) -> SolverError {
    match error {
        LuError::SymbolicSingular { .. } => SolverError::SingularMatrix,
        LuError::Generic(error) => map_faer_error(error),
    }
}

/// Reusable sparse-LU workspace for the Newton hot path.
///
/// The symbolic analysis is computed once for the frozen sparsity pattern;
/// the numeric factorization, scratch arena, and RHS buffer are reused so
/// repeated solves perform no per-iteration allocations. The symbolic part is
/// `Arc`-shared because LU symbolic analysis depends only on the pattern, not
/// the scalar type — AC's complex matrices reuse it as-is.
struct LuWorkspace {
    symbolic: Arc<sparse_lu::SymbolicLu<usize>>,
    numeric: sparse_lu::NumericLu<usize, Value>,
    factor_mem: MemBuffer,
    solve_mem: MemBuffer,
    transpose_solve_mem: MemBuffer,
    solve_rhs_capacity: usize,
    transpose_rhs_capacity: usize,
    rhs: Mat<Value>,
    /// Numerically equilibrated CSC values for faer's factorization. Circuit
    /// matrices routinely mix conductances, ideal-source rows, and high-gain
    /// controlled sources across many orders of magnitude; factoring the raw
    /// values can lose the pivot quality required by Newton iteration.
    scaled_values: Vec<Value>,
    scaled_rhs: Vec<Value>,
    row_scale: Vec<Value>,
    col_scale: Vec<Value>,
    /// Exact matrix values represented by `numeric`. Repeated RHS solves and
    /// constant-coefficient transient steps reuse the numeric factors when the
    /// freshly stamped CSC values are unchanged.
    factored_values: Vec<Value>,
}

/// Row-major traversal of the immutable CSC pattern used by normal residual
/// checks. Each row retains ascending-column order, exactly matching the
/// arithmetic order of the former column-major scatter walk.
struct ResidualLayout {
    row_ptr: Vec<usize>,
    col_idx: Vec<usize>,
    csc_idx: Vec<usize>,
}

impl ResidualLayout {
    fn from_csc(csc: &SymbolicSparseColMat<usize>) -> Self {
        let nrows = csc.nrows();
        let nnz = csc.row_idx().len();
        let mut row_ptr = vec![0usize; nrows + 1];
        for &row in csc.row_idx() {
            row_ptr[row + 1] += 1;
        }
        for row in 1..=nrows {
            row_ptr[row] += row_ptr[row - 1];
        }

        let mut next = row_ptr[..nrows].to_vec();
        let mut col_idx = vec![0usize; nnz];
        let mut csc_idx = vec![0usize; nnz];
        for col in 0..csc.ncols() {
            for index in csc.col_ptr()[col]..csc.col_ptr()[col + 1] {
                let row = csc.row_idx()[index];
                let position = next[row];
                next[row] += 1;
                col_idx[position] = col;
                csc_idx[position] = index;
            }
        }
        Self {
            row_ptr,
            col_idx,
            csc_idx,
        }
    }
}

//=============================================================================
// Static Structure Matrix - The Key Optimization
//=============================================================================

/// Pre-computed stamp location that maps directly to CSC values array
#[derive(Debug, Clone, Copy)]
pub struct CscIndex(
    /// Numeric offset, public for compatibility with existing stamp code.
    pub usize,
    u64,
);

/// Opaque identity for one frozen CSC sparsity pattern.
///
/// Matrices created by [`StaticMatrix::clone_structure`] intentionally share
/// this token. Unrelated matrices never do, even when their dimensions and
/// numeric offsets happen to match.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CscPatternToken(u64);

impl CscPatternToken {
    /// Bind a numeric offset to this frozen-pattern identity.
    ///
    /// Compact batched stamp plans retain one pattern token and many offsets
    /// instead of duplicating the token in every [`CscIndex`]. The resulting
    /// index remains fully checked by [`StaticMatrix::stamp_direct`], including
    /// both pattern identity and bounds.
    #[inline]
    pub const fn bind_offset(self, offset: usize) -> CscIndex {
        CscIndex(offset, self.0)
    }
}

impl CscIndex {
    /// Numeric offset in the CSC value array.
    ///
    /// Exposed for read-only diagnostics. Stamping should pass the complete
    /// token back to [`StaticMatrix::stamp_direct`] or the corresponding
    /// complex method so the originating pattern can be validated.
    #[inline]
    pub const fn offset(self) -> usize {
        self.0
    }
}

static NEXT_PATTERN_ID: AtomicU64 = AtomicU64::new(1);

#[inline]
fn next_pattern_id() -> Result<u64, SolverError> {
    NEXT_PATTERN_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
            current.checked_add(1)
        })
        .map_err(|_| {
            SolverError::InvalidCircuit("matrix pattern identifier space exhausted".to_string())
        })
}

#[inline]
fn find_csc_offset(csc: &SymbolicSparseColMat<usize>, row: usize, col: usize) -> Option<usize> {
    if row >= csc.nrows() || col >= csc.ncols() {
        return None;
    }
    let begin = csc.col_ptr()[col];
    let end = csc.col_ptr()[col + 1];
    csc.row_idx()[begin..end]
        .binary_search(&row)
        .ok()
        .map(|offset| begin + offset)
}

#[derive(Debug, Clone, Copy)]
struct MissingMatrixPosition {
    method: &'static str,
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, Copy)]
enum MatrixStampError {
    MissingPosition(MissingMatrixPosition),
    InvalidIndex {
        method: &'static str,
        offset: usize,
        index_pattern: u64,
        matrix_pattern: u64,
    },
}

impl MissingMatrixPosition {
    fn into_solver_error(self) -> SolverError {
        SolverError::InvalidCircuit(format!(
            "{} missing matrix position ({}, {})",
            self.method, self.row, self.col
        ))
    }
}

impl MatrixStampError {
    fn into_solver_error(self) -> SolverError {
        match self {
            Self::MissingPosition(missing) => missing.into_solver_error(),
            Self::InvalidIndex {
                method,
                offset,
                index_pattern,
                matrix_pattern,
            } => SolverError::InvalidCircuit(format!(
                "{method} received CSC offset {offset} for pattern {index_pattern}, but the matrix uses pattern {matrix_pattern}"
            )),
        }
    }
}

/// Pre-built matrix structure with static topology
///
/// This is the critical optimization: we build the structure once during
/// circuit setup, then only update the values during Newton-Raphson iterations.
/// This avoids the O(N log N) sort and memory allocation on every solve.
pub struct StaticMatrix {
    /// Matrix row count.
    pub nrows: usize,
    /// Matrix column count.
    pub ncols: usize,
    /// Frozen CSC sparsity pattern, validated once at construction.
    /// Solves borrow it as a view — no per-iteration structure copies.
    csc: Arc<SymbolicSparseColMat<usize>>,
    /// Pre-transposed pattern traversal for cache-local normal residuals.
    residual_layout: Arc<ResidualLayout>,
    /// CSC values (mutable - updated each iteration)
    values: Vec<Value>,
    /// Identity shared by matrices cloned from this exact sparsity pattern.
    /// Precomputed stamp tokens carry the same identity, preventing a valid
    /// offset from one topology from silently corrupting another topology.
    pattern_id: u64,
    /// Reusable LU workspace (lazily initialized on first solve)
    lu: Option<LuWorkspace>,
    /// Default KLU-class real backend: refactors the frozen pattern with a
    /// stored pivot sequence instead of fully re-pivoting every Newton
    /// iteration. Lazily initialized; any failure falls back to the faer path.
    klu: Option<crate::KluSolver>,
    /// Exact matrix values represented by the current Circuit-LU factors.
    /// Comparing one compact CSC value array is far cheaper than replaying a
    /// numeric factorization for constant-coefficient timesteps.
    klu_factored_values: Vec<Value>,
    /// Numeric values in CSC(A^T) order for the Amesos row-CRS compatibility
    /// path. The immutable structure and source-value map live in
    /// `residual_layout`; this vector is reused without per-solve allocation.
    klu_oriented_values: Vec<Value>,
    /// Auto-policy decision retained for the frozen sparsity pattern after a
    /// measured high-fill Circuit LU factorization.
    klu_auto_rejected: bool,
    /// Explicit per-matrix backend policy. This avoids process-global solver
    /// state for commercial embedding while preserving an environment-aware
    /// compatibility constructor.
    solver_options: SolverOptions,
    /// Scratch values + RHS retained between residual probes (see
    /// [`StaticMatrix::with_probe_values`]).
    probe_values: Option<Vec<Value>>,
    probe_rhs: Option<Vec<Value>>,
    /// Scratch for the A*x product inside residual norms.
    residual_scratch: Vec<Value>,
    residual_gross_scratch: Vec<Value>,
    residual_compensation_scratch: Vec<Value>,
    residual_row_nnz_scratch: Vec<usize>,
    /// Retained KLU iterative-refinement correction. This path is uncommon,
    /// but keeping its buffer makes even ill-scaled repeated solves allocation
    /// free after the first accepted system.
    klu_correction_scratch: Vec<Value>,
    /// Low component of the double-double Newton correction RHS. The high
    /// component is written directly into the caller-owned output buffer.
    correction_rhs_lo_scratch: Vec<Value>,
    /// First attempted stamp outside the frozen sparsity pattern.
    stamping_error: Option<MatrixStampError>,
}

struct ProbeValuesGuard<'a> {
    matrix: &'a mut StaticMatrix,
    saved_values: Option<Vec<Value>>,
    rhs: Option<Vec<Value>>,
}

impl Drop for ProbeValuesGuard<'_> {
    fn drop(&mut self) {
        let Some(mut saved_values) = self.saved_values.take() else {
            return;
        };
        std::mem::swap(&mut self.matrix.values, &mut saved_values);
        self.matrix.probe_values = Some(saved_values);
        self.matrix.probe_rhs = self.rhs.take();
    }
}

#[cold]
#[inline(never)]
fn missing_matrix_position(method: &'static str, row: usize, col: usize) -> MissingMatrixPosition {
    MissingMatrixPosition { method, row, col }
}

/// Whether the KLU-class backend handles real solves for this process.
///
/// Default ON: the full ngspice conformance run under the backend
/// reproduces the faer baseline failure set exactly, and the benchmark
/// scoreboard shows 14-15% end-to-end improvement on solver-bound decks
/// (benchmarks/scoreboards/2026-06-11-faer-vs-klu-*.json).
/// `RSPICE_SOLVER=faer` opts out.
pub(crate) fn klu_backend_enabled() -> bool {
    SolverOptions::from_env().real_backend != RealSolverBackend::Faer
}

#[inline]
fn auto_rejects_circuit_lu_fill(n: usize, a_nnz: usize, l_nnz: usize, u_nnz: usize) -> bool {
    n >= 256 && l_nnz.saturating_add(u_nnz) > a_nnz.saturating_mul(8)
}

#[inline]
fn auto_prefers_supernodal_from_pattern(n: usize, a_nnz: usize) -> bool {
    // High-degree unsymmetric patterns are the regime where a supernodal
    // backend amortizes dense frontal work and Circuit LU is likely to create
    // large scalar fill. Avoid paying for a known-bad exploratory factor.
    n >= 512 && a_nnz >= n.saturating_mul(8)
}

#[inline]
fn finite_solution_or_singular(solution: Vec<Value>) -> Result<Vec<Value>, SolverError> {
    if solution.iter().all(|value| value.is_finite()) {
        Ok(solution)
    } else {
        Err(SolverError::SingularMatrix)
    }
}

#[inline]
fn finite_reciprocal_scale(max_abs: Value) -> Value {
    debug_assert!(max_abs.is_finite() && max_abs >= 0.0);
    let small_number = Value::MIN_POSITIVE / Value::EPSILON;
    let large_number = 1.0 / small_number;
    (1.0 / max_abs).clamp(small_number, large_number)
}

/// Build `D_r * A * D_c` and the matching `D_r * b` transforms for faer.
///
/// Column max scaling followed by row max scaling mirrors the equilibration
/// used by production SPICE sparse solvers. The original matrix remains
/// untouched for stamping and residual evaluation; after solving for `y`, the
/// caller recovers the original-coordinate solution as `x = D_c * y`.
fn equilibrate_sparse_system(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Value],
    scaled_values: &mut Vec<Value>,
    row_scale: &mut Vec<Value>,
    col_scale: &mut Vec<Value>,
) -> Result<(), SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() {
        return Err(SolverError::InvalidCircuit(
            "Sparse equilibration dimension mismatch".to_string(),
        ));
    }

    scaled_values.resize(values.len(), 0.0);
    row_scale.resize(nrows, 1.0);
    col_scale.resize(ncols, 1.0);

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        let mut max_abs: Value = 0.0;
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            if !value.is_finite() {
                return Err(SolverError::Overflow);
            }
            max_abs = max_abs.max(value.abs());
        }
        if max_abs == 0.0 {
            return Err(SolverError::SingularMatrix);
        }
        let scale = finite_reciprocal_scale(max_abs);
        col_scale[col] = scale;
        for idx in col_ptr[col]..col_ptr[col + 1] {
            scaled_values[idx] = values[idx] * scale;
        }
    }

    row_scale.fill(0.0);
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let row = row_idx[idx];
            row_scale[row] = row_scale[row].max(scaled_values[idx].abs());
        }
    }
    for row in 0..nrows {
        if row_scale[row] == 0.0 {
            return Err(SolverError::SingularMatrix);
        }
        row_scale[row] = finite_reciprocal_scale(row_scale[row]);
    }
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            scaled_values[idx] *= row_scale[row_idx[idx]];
        }
    }
    if scaled_values.iter().any(|value| !value.is_finite()) {
        return Err(SolverError::Overflow);
    }

    Ok(())
}

#[inline]
fn backward_error_tolerance(row_nnz: usize) -> Value {
    64.0 * Value::EPSILON * (row_nnz.saturating_add(1) as Value)
}

#[derive(Debug, Clone, Copy)]
struct BackwardError {
    componentwise: Value,
    acceptance_ratio: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RealSolveOp {
    Normal,
    Transpose,
}

impl RealSolveOp {
    #[inline]
    fn for_circuit_lu(self, orientation: CircuitLuOrientation) -> Self {
        match orientation {
            CircuitLuOrientation::Native => self,
            CircuitLuOrientation::AmesosRowCrs => match self {
                Self::Normal => Self::Transpose,
                Self::Transpose => Self::Normal,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KluNumericAction {
    Reuse,
    Refactor,
    Factor,
}

#[inline]
fn values_bitwise_equal(left: &[Value], right: &[Value]) -> bool {
    left.len() == right.len()
        && left
            .iter()
            .zip(right)
            .all(|(&left, &right)| left.to_bits() == right.to_bits())
}

fn klu_numeric_action(
    backend: &crate::KluSolver,
    values_current: bool,
    policy: NumericFactorizationPolicy,
    request: FactorizationRequest,
) -> Result<KluNumericAction, SolverError> {
    if request == FactorizationRequest::ReuseExisting {
        return if backend.is_factored() {
            Ok(KluNumericAction::Reuse)
        } else {
            Err(SolverError::InvalidCircuit(
                "circuit-LU factor reuse was requested before numeric factorization".to_string(),
            ))
        };
    }

    if request == FactorizationRequest::Automatic
        && policy == NumericFactorizationPolicy::ReusePivotSequence
        && values_current
        && backend.is_factored()
    {
        return Ok(KluNumericAction::Reuse);
    }

    Ok(match policy {
        NumericFactorizationPolicy::FreshPivotSelection => KluNumericAction::Factor,
        NumericFactorizationPolicy::ReusePivotSequence => KluNumericAction::Refactor,
    })
}

fn configure_klu_backend(
    backend: &mut crate::KluSolver,
    options: SolverOptions,
) -> Result<(), SolverError> {
    backend.set_direct_factorization_division(
        options.factorization_division == DivisionPolicy::DirectDivision,
    );
    backend.set_direct_diagonal_division(options.diagonal_solve == DivisionPolicy::DirectDivision);
    backend.set_row_scaling_enabled(
        options.circuit_lu_row_scaling == CircuitLuRowScaling::AdaptiveExtremeRows,
    );
    backend
        .set_growth_retry_enabled(options.circuit_lu_robustness == CircuitLuRobustness::Enhanced);
    backend.set_pivot_tolerance(options.pivot_tolerance)?;
    backend.set_absolute_pivot_tolerance(options.absolute_pivot_tolerance)
}

fn prepare_klu_input<'a>(
    orientation: CircuitLuOrientation,
    csc: &'a SymbolicSparseColMat<usize>,
    residual_layout: &'a ResidualLayout,
    values: &'a [Value],
    oriented_values: &'a mut Vec<Value>,
    refresh_values: bool,
) -> (&'a [usize], &'a [usize], &'a [Value]) {
    match orientation {
        CircuitLuOrientation::Native => (csc.col_ptr(), csc.row_idx(), values),
        CircuitLuOrientation::AmesosRowCrs => {
            if refresh_values || oriented_values.len() != values.len() {
                oriented_values.resize(values.len(), 0.0);
                for (target, &source) in oriented_values.iter_mut().zip(&residual_layout.csc_idx) {
                    *target = values[source];
                }
            }
            (
                &residual_layout.row_ptr,
                &residual_layout.col_idx,
                oriented_values,
            )
        }
    }
}

fn apply_klu_numeric_action(
    backend: &mut crate::KluSolver,
    values: &[Value],
    action: KluNumericAction,
    robustness: CircuitLuRobustness,
) -> Result<(), SolverError> {
    match action {
        KluNumericAction::Reuse => Ok(()),
        KluNumericAction::Factor => backend.factor(values),
        KluNumericAction::Refactor => match backend.refactor(values) {
            Ok(()) => Ok(()),
            Err(SolverError::PivotGrowth) if robustness == CircuitLuRobustness::Enhanced => {
                backend.factor(values)
            }
            Err(error) => Err(error),
        },
    }
}

impl BackwardError {
    #[inline]
    fn accepted(self) -> bool {
        self.acceptance_ratio <= 1.0
    }
}

/// Compute componentwise backward error in the supplied coordinates.
///
/// The denominator `|b| + |A| |x|` makes the check unit- and scale-invariant.
/// Consequently, checking `D_r*A*D_c`, `y`, and `D_r*b` is mathematically
/// identical to checking the original system with `x = D_c*y`, while avoiding
/// overflow when valid original-coordinate terms approach `f64::MAX`.
/// Rows whose entire signal is near underflow use a safe denominator floor.
/// Unlike LAPACK's forward-error estimator, this solve-acceptance check does
/// not add the floor to the numerator: doing so turns an unavoidable single
/// subnormal rounding bit into an order-one error and rejects otherwise exact
/// sparse solves (notably the inactive tail of a large RC ladder).
/// `residual` is retained as `b - A*x` for iterative refinement.
#[allow(clippy::too_many_arguments)]
#[cfg(test)]
fn componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Value],
    solution: &[Value],
    rhs: &[Value],
    residual: &mut Vec<Value>,
    denominator: &mut Vec<Value>,
    compensation: &mut Vec<Value>,
    row_nnz: &mut Vec<usize>,
    operation: RealSolveOp,
) -> Result<BackwardError, SolverError> {
    componentwise_backward_error_impl(
        csc,
        None,
        values,
        solution,
        rhs,
        residual,
        denominator,
        compensation,
        row_nnz,
        operation,
    )
}

#[allow(clippy::too_many_arguments)]
fn componentwise_backward_error_with_layout(
    csc: &SymbolicSparseColMat<usize>,
    residual_layout: &ResidualLayout,
    values: &[Value],
    solution: &[Value],
    rhs: &[Value],
    residual: &mut Vec<Value>,
    denominator: &mut Vec<Value>,
    compensation: &mut Vec<Value>,
    row_nnz: &mut Vec<usize>,
    operation: RealSolveOp,
) -> Result<BackwardError, SolverError> {
    componentwise_backward_error_impl(
        csc,
        Some(residual_layout),
        values,
        solution,
        rhs,
        residual,
        denominator,
        compensation,
        row_nnz,
        operation,
    )
}

#[allow(clippy::too_many_arguments)]
fn componentwise_backward_error_impl(
    csc: &SymbolicSparseColMat<usize>,
    residual_layout: Option<&ResidualLayout>,
    values: &[Value],
    solution: &[Value],
    rhs: &[Value],
    residual: &mut Vec<Value>,
    denominator: &mut Vec<Value>,
    compensation: &mut Vec<Value>,
    row_nnz: &mut Vec<usize>,
    operation: RealSolveOp,
) -> Result<BackwardError, SolverError> {
    let fast = fast_componentwise_backward_error(
        csc,
        residual_layout,
        values,
        solution,
        rhs,
        residual,
        denominator,
        row_nnz,
        operation,
    )?;
    if fast.accepted() {
        return Ok(fast);
    }
    compensated_componentwise_backward_error(
        csc,
        residual_layout,
        values,
        solution,
        rhs,
        residual,
        denominator,
        compensation,
        row_nnz,
        operation,
    )
}

/// Fast, rigorously bounded first-pass residual. The ordinary accumulation is
/// accepted only when its rounding-error upper bound still fits inside the
/// shared backward-error tolerance; borderline systems fall through to the
/// double-double implementation below.
#[allow(clippy::too_many_arguments)]
fn fast_componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    residual_layout: Option<&ResidualLayout>,
    values: &[Value],
    solution: &[Value],
    rhs: &[Value],
    residual: &mut Vec<Value>,
    denominator: &mut Vec<Value>,
    row_nnz: &mut Vec<usize>,
    operation: RealSolveOp,
) -> Result<BackwardError, SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() || solution.len() != ncols || rhs.len() != nrows {
        return Err(SolverError::InvalidCircuit(
            "Sparse backward-error dimension mismatch".to_string(),
        ));
    }

    if operation == RealSolveOp::Normal
        && let Some(layout) = residual_layout
    {
        if layout.row_ptr.len() != nrows + 1
            || layout.col_idx.len() != values.len()
            || layout.csc_idx.len() != values.len()
        {
            return Err(SolverError::InvalidCircuit(
                "Sparse residual layout dimension mismatch".to_string(),
            ));
        }
        residual.resize(nrows, 0.0);
        denominator.resize(nrows, 0.0);
        row_nnz.resize(nrows, 0);
        let mut error: Value = 0.0;
        let mut acceptance_ratio: Value = 0.0;
        for row in 0..nrows {
            let rhs_value = rhs[row];
            if !rhs_value.is_finite() {
                return Err(SolverError::Overflow);
            }
            let mut row_residual = rhs_value;
            let mut row_denominator = rhs_value.abs();
            let mut nonzeros = 0usize;
            for position in layout.row_ptr[row]..layout.row_ptr[row + 1] {
                let value = values[layout.csc_idx[position]];
                let x = solution[layout.col_idx[position]];
                let term = value * x;
                let magnitude = value.abs() * x.abs();
                if !term.is_finite() || !magnitude.is_finite() {
                    return Err(SolverError::Overflow);
                }
                if value != 0.0 {
                    nonzeros = nonzeros.saturating_add(1);
                }
                row_residual -= term;
                row_denominator = (row_denominator + magnitude).min(Value::MAX);
            }
            residual[row] = row_residual;
            denominator[row] = row_denominator;
            row_nnz[row] = nonzeros;

            let safe1 = (nonzeros.saturating_add(1) as Value) * Value::MIN_POSITIVE;
            let residual_abs = row_residual.abs();
            if !residual_abs.is_finite() || !row_denominator.is_finite() {
                return Err(SolverError::Overflow);
            }
            let rounding_bound = 4.0 * (nonzeros.saturating_add(1) as Value) * Value::EPSILON;
            let row_error = residual_abs / row_denominator.max(safe1);
            let certified_error = row_error + rounding_bound;
            error = error.max(row_error);
            acceptance_ratio =
                acceptance_ratio.max(certified_error / backward_error_tolerance(nonzeros));
        }
        return Ok(BackwardError {
            componentwise: error,
            acceptance_ratio,
        });
    }

    residual.resize(nrows, 0.0);
    denominator.resize(nrows, 0.0);
    row_nnz.resize(nrows, 0);
    row_nnz.fill(0);
    for row in 0..nrows {
        if !rhs[row].is_finite() {
            return Err(SolverError::Overflow);
        }
        residual[row] = rhs[row];
        denominator[row] = rhs[row].abs();
    }

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            let original_row = row_idx[idx];
            let (equation, x_index) = match operation {
                RealSolveOp::Normal => (original_row, col),
                RealSolveOp::Transpose => (col, original_row),
            };
            let x = solution[x_index];
            let term = value * x;
            let magnitude = value.abs() * x.abs();
            if !term.is_finite() || !magnitude.is_finite() {
                return Err(SolverError::Overflow);
            }
            if value != 0.0 {
                row_nnz[equation] = row_nnz[equation].saturating_add(1);
            }
            residual[equation] -= term;
            denominator[equation] = (denominator[equation] + magnitude).min(Value::MAX);
        }
    }

    let mut error: Value = 0.0;
    let mut acceptance_ratio: Value = 0.0;
    for row in 0..nrows {
        let safe1 = (row_nnz[row].saturating_add(1) as Value) * Value::MIN_POSITIVE;
        let residual_abs = residual[row].abs();
        let scale = denominator[row];
        if !residual_abs.is_finite() || !scale.is_finite() {
            return Err(SolverError::Overflow);
        }
        let rounding_bound = 4.0 * (row_nnz[row].saturating_add(1) as Value) * Value::EPSILON;
        let row_error = residual_abs / scale.max(safe1);
        let certified_error = row_error + rounding_bound;
        error = error.max(row_error);
        acceptance_ratio =
            acceptance_ratio.max(certified_error / backward_error_tolerance(row_nnz[row]));
    }
    Ok(BackwardError {
        componentwise: error,
        acceptance_ratio,
    })
}

#[allow(clippy::too_many_arguments)]
fn compensated_componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    residual_layout: Option<&ResidualLayout>,
    values: &[Value],
    solution: &[Value],
    rhs: &[Value],
    residual: &mut Vec<Value>,
    denominator: &mut Vec<Value>,
    compensation: &mut Vec<Value>,
    row_nnz: &mut Vec<usize>,
    operation: RealSolveOp,
) -> Result<BackwardError, SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if operation == RealSolveOp::Normal
        && let Some(layout) = residual_layout
    {
        residual.resize(nrows, 0.0);
        denominator.resize(nrows, 0.0);
        compensation.resize(nrows, 0.0);
        row_nnz.resize(nrows, 0);
        let mut error: Value = 0.0;
        let mut acceptance_ratio: Value = 0.0;
        for row in 0..nrows {
            let mut row_residual = rhs[row];
            let mut row_denominator = rhs[row].abs();
            let mut row_compensation = 0.0;
            let mut nonzeros = 0usize;
            for position in layout.row_ptr[row]..layout.row_ptr[row + 1] {
                let value = values[layout.csc_idx[position]];
                let x = solution[layout.col_idx[position]];
                let term = value * x;
                let magnitude = value.abs() * x.abs();
                if !term.is_finite() || !magnitude.is_finite() {
                    return Err(SolverError::Overflow);
                }
                if value != 0.0 {
                    nonzeros = nonzeros.saturating_add(1);
                }
                let product_hi = -term;
                let product_lo = (-value).mul_add(x, -product_hi);
                let sum = row_residual + product_hi;
                let virtual_addend = sum - row_residual;
                let sum_error =
                    (row_residual - (sum - virtual_addend)) + (product_hi - virtual_addend);
                let tail = row_compensation + product_lo + sum_error;
                let refined = sum + tail;
                row_compensation = tail - (refined - sum);
                row_residual = refined;
                row_denominator = (row_denominator + magnitude).min(Value::MAX);
            }
            row_residual += row_compensation;
            residual[row] = row_residual;
            denominator[row] = row_denominator;
            compensation[row] = row_compensation;
            row_nnz[row] = nonzeros;

            let safe1 = (nonzeros.saturating_add(1) as Value) * Value::MIN_POSITIVE;
            let residual_abs = row_residual.abs();
            if !residual_abs.is_finite() || !row_denominator.is_finite() {
                return Err(SolverError::Overflow);
            }
            let row_error = residual_abs / row_denominator.max(safe1);
            error = error.max(row_error);
            acceptance_ratio = acceptance_ratio.max(row_error / backward_error_tolerance(nonzeros));
        }
        return Ok(BackwardError {
            componentwise: error,
            acceptance_ratio,
        });
    }

    residual.resize(nrows, 0.0);
    denominator.resize(nrows, 0.0);
    compensation.resize(nrows, 0.0);
    compensation.fill(0.0);
    row_nnz.resize(nrows, 0);
    row_nnz.fill(0);
    for row in 0..nrows {
        residual[row] = rhs[row];
        denominator[row] = rhs[row].abs();
    }
    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            let original_row = row_idx[idx];
            let (equation, x_index) = match operation {
                RealSolveOp::Normal => (original_row, col),
                RealSolveOp::Transpose => (col, original_row),
            };
            let x = solution[x_index];
            let term = value * x;
            let magnitude = value.abs() * x.abs();
            if !term.is_finite() || !magnitude.is_finite() {
                return Err(SolverError::Overflow);
            }
            if value != 0.0 {
                row_nnz[equation] = row_nnz[equation].saturating_add(1);
            }
            let product_hi = -term;
            let product_lo = (-value).mul_add(x, -product_hi);
            let sum = residual[equation] + product_hi;
            let virtual_addend = sum - residual[equation];
            let sum_error =
                (residual[equation] - (sum - virtual_addend)) + (product_hi - virtual_addend);
            let tail = compensation[equation] + product_lo + sum_error;
            let refined = sum + tail;
            compensation[equation] = tail - (refined - sum);
            residual[equation] = refined;
            denominator[equation] = (denominator[equation] + magnitude).min(Value::MAX);
        }
    }
    let mut error: Value = 0.0;
    let mut acceptance_ratio: Value = 0.0;
    for row in 0..nrows {
        residual[row] += compensation[row];
        let safe1 = (row_nnz[row].saturating_add(1) as Value) * Value::MIN_POSITIVE;
        let residual_abs = residual[row].abs();
        let scale = denominator[row];
        if !residual_abs.is_finite() || !scale.is_finite() {
            return Err(SolverError::Overflow);
        }
        let row_error = residual_abs / scale.max(safe1);
        error = error.max(row_error);
        acceptance_ratio = acceptance_ratio.max(row_error / backward_error_tolerance(row_nnz[row]));
    }
    Ok(BackwardError {
        componentwise: error,
        acceptance_ratio,
    })
}

struct CorrectionRhsWorkspace<'a> {
    correction: &'a mut Vec<Value>,
    low_components: &'a mut Vec<Value>,
}

fn fill_correction_rhs(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Value],
    nrows: usize,
    ncols: usize,
    rhs: &[Value],
    iterate: &[Value],
    workspace: CorrectionRhsWorkspace<'_>,
) -> Result<(), SolverError> {
    let CorrectionRhsWorkspace {
        correction,
        low_components: correction_lo,
    } = workspace;
    if rhs.len() != nrows || iterate.len() != ncols {
        return Err(SolverError::InvalidCircuit(format!(
            "Correction system requires RHS/iterate dimensions {} and {}, got {} and {}",
            nrows,
            ncols,
            rhs.len(),
            iterate.len()
        )));
    }
    if rhs.iter().chain(iterate).any(|value| !value.is_finite()) {
        return Err(SolverError::Overflow);
    }

    correction.clear();
    correction.extend_from_slice(rhs);
    correction_lo.resize(nrows, 0.0);
    correction_lo.fill(0.0);
    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        let x = iterate[col];
        for index in col_ptr[col]..col_ptr[col + 1] {
            let value = values[index];
            if !value.is_finite() {
                return Err(SolverError::Overflow);
            }
            let row = row_idx[index];
            let product_hi = (-value) * x;
            let product_lo = (-value).mul_add(x, -product_hi);
            let sum = correction[row] + product_hi;
            let virtual_addend = sum - correction[row];
            let sum_error =
                (correction[row] - (sum - virtual_addend)) + (product_hi - virtual_addend);
            let tail = correction_lo[row] + product_lo + sum_error;
            let refined = sum + tail;
            correction_lo[row] = tail - (refined - sum);
            correction[row] = refined;
        }
    }
    for (hi, &lo) in correction.iter_mut().zip(correction_lo.iter()) {
        *hi += lo;
    }
    if correction.iter().any(|value| !value.is_finite()) {
        return Err(SolverError::Overflow);
    }
    Ok(())
}

impl StaticMatrix {
    /// Create a zero-valued matrix with the same sparsity structure.
    ///
    /// This is used for residual probes that must stamp into an independent
    /// numeric workspace while preserving O(1) matrix-entry lookup.
    pub fn clone_structure(&self) -> Self {
        Self {
            nrows: self.nrows,
            ncols: self.ncols,
            csc: self.csc.clone(),
            residual_layout: self.residual_layout.clone(),
            values: vec![0.0; self.values.len()],
            pattern_id: self.pattern_id,
            lu: None,
            klu: None,
            klu_factored_values: Vec::new(),
            klu_oriented_values: Vec::new(),
            klu_auto_rejected: self.klu_auto_rejected,
            solver_options: self.solver_options,
            probe_values: None,
            probe_rhs: None,
            residual_scratch: Vec::new(),
            residual_gross_scratch: Vec::new(),
            residual_compensation_scratch: Vec::new(),
            residual_row_nnz_scratch: Vec::new(),
            klu_correction_scratch: Vec::new(),
            correction_rhs_lo_scratch: Vec::new(),
            stamping_error: None,
        }
    }

    /// Run `f` against this matrix with zeroed scratch values and RHS swapped
    /// in, restoring the live values afterwards.
    ///
    /// Residual probes need to stamp a trial linearization without disturbing
    /// the in-flight Newton system. Since the structure, CSC lookup, and LU
    /// workspace are all valid for any values array, swapping the values
    /// buffer gives the probe a free matrix: no structure clones, no
    /// position-map rehash, and probe solves reuse the cached symbolic
    /// analysis and numeric workspace. (A subsequent live solve refactorizes
    /// from the restored values, so workspace sharing is safe.)
    pub fn with_probe_values<R>(&mut self, f: impl FnOnce(&mut Self, &mut [Value]) -> R) -> R {
        let mut scratch = self.probe_values.take().unwrap_or_default();
        scratch.resize(self.values.len(), 0.0);
        scratch.fill(0.0);
        let mut rhs = self.probe_rhs.take().unwrap_or_default();
        rhs.resize(self.nrows, 0.0);
        rhs.fill(0.0);

        std::mem::swap(&mut self.values, &mut scratch);
        let mut guard = ProbeValuesGuard {
            matrix: self,
            saved_values: Some(scratch),
            rhs: Some(rhs),
        };
        let rhs = guard
            .rhs
            .as_mut()
            .expect("probe RHS is present for the guard lifetime");
        f(guard.matrix, rhs)
    }

    fn to_dense_real(&self) -> Vec<Vec<Value>> {
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut dense = vec![vec![0.0; self.ncols]; self.nrows];
        for col in 0..self.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                dense[row][col] = self.values[idx];
            }
        }
        dense
    }

    /// Build static structure from triplets (called once during setup)
    pub fn from_triplets(
        nrows: usize,
        ncols: usize,
        triplets: &[(usize, usize, Value)],
    ) -> Result<Self, SolverError> {
        Self::from_triplets_with_options(nrows, ncols, triplets, SolverOptions::from_env())
    }

    /// Build a static matrix with an explicit backend policy.
    pub fn from_triplets_with_options(
        nrows: usize,
        ncols: usize,
        triplets: &[(usize, usize, Value)],
        solver_options: SolverOptions,
    ) -> Result<Self, SolverError> {
        if nrows == 0 || ncols == 0 {
            return Err(SolverError::InvalidCircuit("Empty matrix".to_string()));
        }
        if !solver_options.pivot_tolerance.is_finite()
            || solver_options.pivot_tolerance <= 0.0
            || solver_options.pivot_tolerance > 1.0
        {
            return Err(SolverError::InvalidCircuit(format!(
                "relative pivot tolerance must be finite and in (0, 1], got {}",
                solver_options.pivot_tolerance
            )));
        }
        if !solver_options.absolute_pivot_tolerance.is_finite()
            || solver_options.absolute_pivot_tolerance < 0.0
        {
            return Err(SolverError::InvalidCircuit(format!(
                "absolute pivot tolerance must be finite and non-negative, got {}",
                solver_options.absolute_pivot_tolerance
            )));
        }
        for (idx, &(row, col, value)) in triplets.iter().enumerate() {
            if row >= nrows || col >= ncols {
                return Err(SolverError::InvalidCircuit(format!(
                    "Triplet {} index out of bounds: ({}, {}) for matrix {}x{}",
                    idx, row, col, nrows, ncols
                )));
            }
            if !value.is_finite() {
                return Err(SolverError::Overflow);
            }
        }

        // Sort by (col, row) for CSC format
        let mut entries: Vec<(usize, usize, Value)> = triplets.to_vec();
        entries.sort_by_key(|&(r, c, _)| (c, r));

        // Accumulate duplicates while retaining strict CSC row ordering.
        let mut accumulated: Vec<(usize, usize, Value)> = Vec::with_capacity(entries.len());
        for (r, c, v) in entries {
            if let Some(last) = accumulated.last_mut()
                && last.0 == r
                && last.1 == c
            {
                let combined = last.2 + v;
                if !combined.is_finite() {
                    return Err(SolverError::Overflow);
                }
                last.2 = combined;
                continue;
            }
            accumulated.push((r, c, v));
        }

        // Build CSC arrays
        let mut col_ptrs = vec![0usize; ncols + 1];
        let mut row_indices = Vec::with_capacity(accumulated.len());
        let mut values = Vec::with_capacity(accumulated.len());

        for &(r, c, v) in &accumulated {
            col_ptrs[c + 1] += 1;
            row_indices.push(r);
            values.push(v);
        }

        // Cumulative sum for col_ptrs
        for i in 1..=ncols {
            col_ptrs[i] += col_ptrs[i - 1];
        }

        // Validate the pattern once here; every solve afterwards borrows it
        // without re-checking.
        let csc = Arc::new(SymbolicSparseColMat::new_checked(
            nrows,
            ncols,
            col_ptrs,
            None,
            row_indices,
        ));
        let residual_layout = Arc::new(ResidualLayout::from_csc(&csc));

        Ok(Self {
            nrows,
            ncols,
            csc,
            residual_layout,
            values,
            pattern_id: next_pattern_id()?,
            lu: None,
            klu: None,
            klu_factored_values: Vec::new(),
            klu_oriented_values: Vec::new(),
            klu_auto_rejected: false,
            solver_options,
            probe_values: None,
            probe_rhs: None,
            residual_scratch: Vec::new(),
            residual_gross_scratch: Vec::new(),
            residual_compensation_scratch: Vec::new(),
            residual_row_nnz_scratch: Vec::new(),
            klu_correction_scratch: Vec::new(),
            correction_rhs_lo_scratch: Vec::new(),
            stamping_error: None,
        })
    }

    /// Current per-matrix solver policy.
    #[inline]
    pub const fn solver_options(&self) -> SolverOptions {
        self.solver_options
    }

    /// Change the backend used by subsequent solves. Existing workspaces are
    /// retained so switching back does not repeat symbolic setup.
    #[inline]
    pub fn set_solver_options(&mut self, solver_options: SolverOptions) {
        if self.solver_options != solver_options {
            // Auto's fill rejection is a policy decision, not a structural
            // failure. In particular it must never prevent a later explicit
            // Circuit-LU request from being honored.
            self.klu_auto_rejected = false;
            self.klu_factored_values.clear();
            if self.solver_options.circuit_lu_orientation != solver_options.circuit_lu_orientation {
                if let Some(backend) = self.klu.as_mut() {
                    backend.invalidate_analysis();
                }
                self.klu_oriented_values.clear();
            }
        }
        self.solver_options = solver_options;
    }

    /// Zero all values (call before each Newton iteration)
    ///
    /// Slice fill lowers to an optimized bulk clear without coupling this
    /// dependency-neutral matrix crate to the simulator's optional SIMD layer.
    #[inline]
    pub fn clear_values(&mut self) {
        self.values.fill(0.0);
    }

    /// Add a value at `(row, col)` using a cache-local search of that column.
    #[inline]
    pub fn add(&mut self, row: usize, col: usize, value: Value) {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                self.record_missing_position("StaticMatrix::add", row, col);
                return;
            }
        };
        self.values[idx] += value;
    }

    /// Checked add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add(&mut self, row: usize, col: usize, value: Value) -> Result<(), SolverError> {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                let missing = missing_matrix_position("StaticMatrix::try_add", row, col);
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx] + value;
        if !value.is_finite() || !combined.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.values[idx] = combined;
        Ok(())
    }

    #[inline]
    fn record_missing_position(&mut self, method: &'static str, row: usize, col: usize) {
        self.stamping_error.get_or_insert_with(|| {
            MatrixStampError::MissingPosition(missing_matrix_position(method, row, col))
        });
    }

    #[inline]
    fn check_stamping_error(&self) -> Result<(), SolverError> {
        match self.stamping_error {
            Some(error) => Err(error.into_solver_error()),
            None => Ok(()),
        }
    }

    /// Rows whose entries are all exactly zero (or absent): the immediate
    /// structural suspects when factorization reports a singular system.
    pub fn deficient_rows(&self) -> Vec<usize> {
        let mut row_max = vec![0.0f64; self.nrows];
        for col in 0..self.ncols {
            for index in self.csc.col_ptr()[col]..self.csc.col_ptr()[col + 1] {
                let row = self.csc.row_idx()[index];
                let magnitude = self.values[index].abs();
                if magnitude > row_max[row] {
                    row_max[row] = magnitude;
                }
            }
        }
        row_max
            .iter()
            .enumerate()
            .filter(|entry| *entry.1 == 0.0)
            .map(|(row, _)| row)
            .collect()
    }

    /// Get CSC index for (row, col) - for pre-indexed stamping
    #[inline]
    pub fn get_index(&self, row: usize, col: usize) -> Option<CscIndex> {
        find_csc_offset(&self.csc, row, col).map(|offset| CscIndex(offset, self.pattern_id))
    }

    /// Return the opaque identity of this matrix's frozen sparsity pattern.
    #[inline]
    pub const fn pattern_token(&self) -> CscPatternToken {
        CscPatternToken(self.pattern_id)
    }

    /// Borrow numeric storage after validating a prelinked pattern once.
    ///
    /// This supports deterministic batched stamping: callers that resolved all
    /// [`CscIndex`] values from `token` can avoid repeating the same pattern
    /// identity check for every scalar addition. A token from an unrelated
    /// matrix fails closed.
    #[inline]
    pub fn values_mut_for_pattern(&mut self, token: CscPatternToken) -> Option<&mut [Value]> {
        (token.0 == self.pattern_id).then_some(self.values.as_mut_slice())
    }

    /// Direct write to values array using pre-computed index
    #[inline]
    pub fn stamp_direct(&mut self, idx: CscIndex, value: Value) {
        if idx.1 != self.pattern_id || idx.0 >= self.values.len() {
            self.stamping_error
                .get_or_insert(MatrixStampError::InvalidIndex {
                    method: "StaticMatrix::stamp_direct",
                    offset: idx.0,
                    index_pattern: idx.1,
                    matrix_pattern: self.pattern_id,
                });
            return;
        }
        self.values[idx.0] += value;
    }

    /// Replace one existing row with an identity constraint.
    ///
    /// The sparsity pattern remains frozen; callers use this for temporary
    /// operating-point constraints such as `.NODESET` startup solves.
    pub fn force_identity_row(&mut self, row: usize) -> Result<(), SolverError> {
        if row >= self.nrows || row >= self.ncols {
            return Err(SolverError::InvalidCircuit(format!(
                "identity row {} outside {}x{} matrix",
                row, self.nrows, self.ncols
            )));
        }

        let mut diagonal_found = false;
        for col in 0..self.ncols {
            if let Some(index) = find_csc_offset(&self.csc, row, col) {
                self.values[index] = if col == row {
                    diagonal_found = true;
                    1.0
                } else {
                    0.0
                };
            }
        }

        if diagonal_found {
            Ok(())
        } else {
            self.record_missing_position("StaticMatrix::force_identity_row", row, row);
            Err(SolverError::InvalidCircuit(format!(
                "identity row {} has no diagonal matrix position",
                row
            )))
        }
    }

    /// Get mutable access to values slice (for advanced use)
    #[inline]
    pub fn values_mut(&mut self) -> &mut [Value] {
        &mut self.values
    }

    /// Compute infinity norm of the scaled residual `A*x-b`.
    ///
    /// Each row is normalized with a SPICE-like tolerance scale:
    /// `abstol + reltol * max(|A*x|, |b|)`.
    /// Returns `inf` when input vectors contain non-finite values.
    ///
    /// The immutable row layout keeps each equation local and allocation-free;
    /// this is evaluated once or twice per Newton iteration.
    pub fn scaled_residual_inf_norm(
        &self,
        solution: &[Value],
        rhs: &[Value],
        abstol: Value,
        reltol: Value,
    ) -> Result<Value, SolverError> {
        self.scaled_residual_inf_norm_by_row(solution, rhs, reltol, |_| abstol)
    }

    /// Compute infinity norm of the scaled residual `A*x-b` with row-specific
    /// absolute tolerances.
    pub fn scaled_residual_inf_norm_by_row<F>(
        &self,
        solution: &[Value],
        rhs: &[Value],
        reltol: Value,
        mut row_abstol: F,
    ) -> Result<Value, SolverError>
    where
        F: FnMut(usize) -> Value,
    {
        self.check_stamping_error()?;
        if self.nrows != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix rows {} don't match RHS size {}",
                self.nrows,
                rhs.len()
            )));
        }
        if self.ncols != solution.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix cols {} don't match solution size {}",
                self.ncols,
                solution.len()
            )));
        }

        let safe_reltol = if reltol.is_finite() && reltol > 0.0 {
            reltol
        } else {
            1e-3
        };

        if solution.iter().any(|value| !value.is_finite()) {
            return Ok(Value::INFINITY);
        }

        let mut residual_inf: Value = 0.0;
        for row in 0..self.nrows {
            let mut row_ax = 0.0;
            let mut row_ax_gross = 0.0;
            for position in self.residual_layout.row_ptr[row]..self.residual_layout.row_ptr[row + 1]
            {
                let term = self.values[self.residual_layout.csc_idx[position]]
                    * solution[self.residual_layout.col_idx[position]];
                row_ax += term;
                row_ax_gross += term.abs();
            }
            let row_rhs = rhs[row];
            if !row_rhs.is_finite() || !row_ax.is_finite() {
                return Ok(Value::INFINITY);
            }
            let residual = (row_ax - row_rhs).abs();
            let abstol = row_abstol(row);
            let safe_abstol = if abstol.is_finite() && abstol > 0.0 {
                abstol
            } else {
                1e-12
            };
            // The row scale is the NET magnitude max(|Σa_ij·x_j|, |b_i|)
            // plus an explicit floating-point cancellation floor on the
            // GROSS term magnitude Σ|a_ij·x_j|. At a converged KCL row the
            // net cancels to ~0 and the residual that remains is the
            // summation noise of the row's own mA-scale currents — bounded
            // by O(n·ε)·gross — so the floor accepts it without bare-abstol
            // rejections (the 4096-MOSFET array artifact). Scaling by the
            // gross magnitude itself (full Oettli–Prager) is too loose as a
            // NONLINEAR acceptance criterion: on high-gain summing rows with
            // large balanced flows it hides genuine disequilibrium that is a
            // tiny fraction of the gross (a feedback amplifier accepts a
            // wrong-basin operating point), so the relative part stays on
            // the net.
            const CANCELLATION_NOISE_TERMS: Value = 256.0;
            let noise_floor = CANCELLATION_NOISE_TERMS * Value::EPSILON * row_ax_gross;
            let scale = safe_abstol + noise_floor + safe_reltol * row_ax.abs().max(row_rhs.abs());
            let normalized = residual / scale.max(safe_abstol);
            residual_inf = residual_inf.max(normalized);
        }

        Ok(residual_inf)
    }

    /// Compute the unscaled infinity norm of `A*x-b` without allocating.
    pub fn raw_residual_inf_norm(
        &self,
        solution: &[Value],
        rhs: &[Value],
    ) -> Result<Value, SolverError> {
        self.raw_residual_norms(solution, rhs)
            .map(|(inf_norm, _)| inf_norm)
    }

    /// Compute the unscaled infinity and Euclidean norms of `A*x-b` in one
    /// matrix-vector product without allocating.
    ///
    /// The Euclidean norm uses the LAPACK scaled sum-of-squares recurrence so
    /// large and small residual components cannot overflow or underflow an
    /// otherwise finite norm. Xyce's transient NOX status tests use both norms
    /// at every nonlinear iterate.
    pub fn raw_residual_norms(
        &self,
        solution: &[Value],
        rhs: &[Value],
    ) -> Result<(Value, Value), SolverError> {
        self.check_stamping_error()?;
        if self.nrows != rhs.len() || self.ncols != solution.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Residual vector size mismatch: matrix is {}x{}, solution has {}, RHS has {}",
                self.nrows,
                self.ncols,
                solution.len(),
                rhs.len()
            )));
        }

        if solution.iter().any(|value| !value.is_finite()) {
            return Ok((Value::INFINITY, Value::INFINITY));
        }

        let mut inf_norm = 0.0_f64;
        let mut l2_scale = 0.0_f64;
        let mut l2_sum_squares = 1.0_f64;
        for (row, &row_rhs) in rhs.iter().enumerate() {
            let mut row_ax = 0.0;
            for position in self.residual_layout.row_ptr[row]..self.residual_layout.row_ptr[row + 1]
            {
                let x = solution[self.residual_layout.col_idx[position]];
                if x != 0.0 {
                    row_ax += self.values[self.residual_layout.csc_idx[position]] * x;
                }
            }
            let residual = row_ax - row_rhs;
            if !residual.is_finite() {
                return Ok((Value::INFINITY, Value::INFINITY));
            }
            let magnitude = residual.abs();
            inf_norm = inf_norm.max(magnitude);
            if magnitude != 0.0 {
                if l2_scale < magnitude {
                    let ratio = l2_scale / magnitude;
                    l2_sum_squares = 1.0 + l2_sum_squares * ratio * ratio;
                    l2_scale = magnitude;
                } else {
                    let ratio = magnitude / l2_scale;
                    l2_sum_squares += ratio * ratio;
                }
            }
        }
        let l2_norm = if l2_scale == 0.0 {
            0.0
        } else {
            l2_scale * l2_sum_squares.sqrt()
        };
        Ok((inf_norm, l2_norm))
    }

    /// Compute the maximum componentwise backward-error ratio for selected
    /// rows of `A*x=b`.
    ///
    /// The ratio uses the same compensated residual and denominator as the
    /// solver's acceptance test: `|b_i-A_i x| / (|b_i|+|A_i||x|)` divided by
    /// the row-size-aware `64*EPSILON*(nnz_i+1)` bound. Returning the ratio
    /// rather than an absolute residual makes the check invariant under
    /// physical units and the extreme coefficient scales common in coupled
    /// magnetic DAEs. An empty row set returns zero.
    pub fn componentwise_backward_error_by_rows(
        &self,
        solution: &[Value],
        rhs: &[Value],
        rows: &[usize],
    ) -> Result<Value, SolverError> {
        self.check_stamping_error()?;
        if self.nrows != rhs.len() || self.ncols != solution.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Backward-error dimension mismatch: matrix is {}x{}, solution has {}, RHS has {}",
                self.nrows,
                self.ncols,
                solution.len(),
                rhs.len()
            )));
        }
        if solution.iter().any(|value| !value.is_finite()) {
            return Ok(Value::INFINITY);
        }

        let mut maximum_ratio: Value = 0.0;
        for &row in rows {
            if row >= self.nrows {
                return Err(SolverError::InvalidCircuit(format!(
                    "Backward-error row {} is outside matrix with {} rows",
                    row, self.nrows
                )));
            }
            let mut residual = rhs[row];
            if !residual.is_finite() {
                return Ok(Value::INFINITY);
            }
            let mut compensation = 0.0;
            let mut denominator = rhs[row].abs();
            let mut nonzeros = 0usize;
            for position in self.residual_layout.row_ptr[row]..self.residual_layout.row_ptr[row + 1]
            {
                let value = self.values[self.residual_layout.csc_idx[position]];
                let x = solution[self.residual_layout.col_idx[position]];
                let term = value * x;
                let magnitude = value.abs() * x.abs();
                if !term.is_finite() || !magnitude.is_finite() {
                    return Ok(Value::INFINITY);
                }
                if value != 0.0 {
                    nonzeros = nonzeros.saturating_add(1);
                }
                let product_hi = -term;
                let product_lo = (-value).mul_add(x, -product_hi);
                let sum = residual + product_hi;
                let virtual_addend = sum - residual;
                let sum_error = (residual - (sum - virtual_addend)) + (product_hi - virtual_addend);
                let tail = compensation + product_lo + sum_error;
                let refined = sum + tail;
                compensation = tail - (refined - sum);
                residual = refined;
                denominator = (denominator + magnitude).min(Value::MAX);
            }
            residual += compensation;
            let safe_denominator = (nonzeros.saturating_add(1) as Value) * Value::MIN_POSITIVE;
            let row_error = residual.abs() / denominator.max(safe_denominator);
            let ratio = row_error / backward_error_tolerance(nonzeros);
            if !ratio.is_finite() {
                return Ok(Value::INFINITY);
            }
            maximum_ratio = maximum_ratio.max(ratio);
        }
        Ok(maximum_ratio)
    }

    /// Compute raw residual vector `A*x - b`.
    pub fn residual_vector(
        &self,
        solution: &[Value],
        rhs: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        self.check_stamping_error()?;
        if solution.len() != self.ncols || rhs.len() != self.nrows {
            return Err(SolverError::InvalidCircuit(
                "Residual vector size mismatch".to_string(),
            ));
        }

        let mut ax = vec![0.0; self.nrows];
        for (row, row_ax) in ax.iter_mut().enumerate() {
            for position in self.residual_layout.row_ptr[row]..self.residual_layout.row_ptr[row + 1]
            {
                let x = solution[self.residual_layout.col_idx[position]];
                if x != 0.0 {
                    *row_ax += self.values[self.residual_layout.csc_idx[position]] * x;
                }
            }
            *row_ax -= rhs[row];
        }

        Ok(ax)
    }

    /// Compute `A*x` from a saved numeric snapshot into caller-owned storage.
    ///
    /// The sparsity layout remains owned by this matrix. Numeric caches use
    /// this to inspect an earlier values-only state without copying it into
    /// the active factorization buffer or allocating a result per probe.
    pub fn matrix_vector_product_with_values_into(
        &self,
        values: &[Value],
        solution: &[Value],
        product: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.check_stamping_error()?;
        if solution.len() != self.ncols || values.len() != self.values.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix-vector size mismatch: matrix is {}x{} with {} values, solution has {}, snapshot has {}",
                self.nrows,
                self.ncols,
                self.values.len(),
                solution.len(),
                values.len(),
            )));
        }

        product.clear();
        product.resize(self.nrows, 0.0);
        for (row, row_ax) in product.iter_mut().enumerate() {
            for position in self.residual_layout.row_ptr[row]..self.residual_layout.row_ptr[row + 1]
            {
                let x = solution[self.residual_layout.col_idx[position]];
                if x != 0.0 {
                    *row_ax += values[self.residual_layout.csc_idx[position]] * x;
                }
            }
        }
        Ok(())
    }

    /// Convert to an owned faer SparseColMat (legacy/test path; copies)
    fn to_sparse_col_mat(&self) -> SparseColMat<usize, Value> {
        SparseColMat::new(self.csc.as_ref().clone(), self.values.clone())
    }

    /// Initialize the reusable LU workspace if it does not exist yet.
    fn ensure_lu_workspace(&mut self) -> Result<(), SolverError> {
        if self.lu.is_some() {
            return Ok(());
        }
        let par = get_global_parallelism();
        let symbolic =
            sparse_lu::factorize_symbolic_lu(self.csc.as_ref().as_ref(), Default::default())
                .map_err(map_faer_error)?;
        let factor_mem = MemBuffer::try_new(
            symbolic.factorize_numeric_lu_scratch::<Value>(par, Default::default()),
        )
        .map_err(|_| SolverError::OutOfMemory)?;
        let solve_mem = MemBuffer::try_new(symbolic.solve_in_place_scratch::<Value>(1, par))
            .map_err(|_| SolverError::OutOfMemory)?;
        let transpose_solve_mem =
            MemBuffer::try_new(symbolic.solve_transpose_in_place_scratch::<Value>(1, par))
                .map_err(|_| SolverError::OutOfMemory)?;
        self.lu = Some(LuWorkspace {
            symbolic: Arc::new(symbolic),
            numeric: sparse_lu::NumericLu::new(),
            factor_mem,
            solve_mem,
            transpose_solve_mem,
            solve_rhs_capacity: 1,
            transpose_rhs_capacity: 1,
            rhs: Mat::zeros(self.nrows, 1),
            scaled_values: Vec::new(),
            scaled_rhs: Vec::new(),
            row_scale: Vec::new(),
            col_scale: Vec::new(),
            factored_values: Vec::new(),
        });
        Ok(())
    }

    /// Solve Ax = b using cached structure.
    ///
    /// Convenience ownership API over [`Self::solve_into`]. Internal symbolic
    /// and numeric workspaces are reused; returning an owned vector requires
    /// one output allocation per call.
    pub fn solve(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_into(rhs, &mut solution)?;
        Ok(solution)
    }

    /// Solve `A^T x = b` while reusing the matrix's symbolic and numeric
    /// workspaces.
    pub fn solve_transpose(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_transpose_into(rhs, &mut solution)?;
        Ok(solution)
    }

    /// Solve `A^T x = b` into caller-owned storage.
    pub fn solve_transpose_into(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_transpose_into_with_factorization(rhs, solution, FactorizationRequest::Automatic)
    }

    /// Solve `A^T x = b` with an explicit numeric-factor lifecycle request.
    pub fn solve_transpose_into_with_factorization(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
        factorization: FactorizationRequest,
    ) -> Result<(), SolverError> {
        let n = self.nrows;
        self.check_stamping_error()?;
        if n != rhs.len() || self.ncols != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Transpose solve requires a square matrix matching RHS size, got {}x{} with RHS {}",
                n,
                self.ncols,
                rhs.len()
            )));
        }
        if self.solver_options.real_backend != RealSolverBackend::Faer && !self.klu_auto_rejected {
            match self.try_solve_klu_operation_into(
                rhs,
                solution,
                RealSolveOp::Transpose,
                factorization,
            ) {
                Ok(true) => return Ok(()),
                Ok(false) => {}
                Err(error) => return Err(error),
            }
        }
        self.solve_faer_operation_into(rhs, solution, RealSolveOp::Transpose)
    }

    /// Solve multiple real systems in one factorization/triangular pass.
    /// Inputs and outputs are dense column-major blocks with `n` values per
    /// right-hand side.
    pub fn solve_many_into(
        &mut self,
        rhs: &[Value],
        rhs_count: usize,
        solution: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_many_into_with_factorization(
            rhs,
            rhs_count,
            solution,
            FactorizationRequest::Automatic,
        )
    }

    /// Solve multiple real systems with an explicit numeric-factor lifecycle
    /// request shared by the entire right-hand-side block.
    pub fn solve_many_into_with_factorization(
        &mut self,
        rhs: &[Value],
        rhs_count: usize,
        solution: &mut Vec<Value>,
        factorization: FactorizationRequest,
    ) -> Result<(), SolverError> {
        self.solve_many_operation_into(rhs, rhs_count, solution, RealSolveOp::Normal, factorization)
    }

    /// Solve multiple `A^T X = B` systems in one factorization and batched
    /// triangular pass. Inputs and outputs are column-major dense blocks.
    pub fn solve_many_transpose_into(
        &mut self,
        rhs: &[Value],
        rhs_count: usize,
        solution: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_many_transpose_into_with_factorization(
            rhs,
            rhs_count,
            solution,
            FactorizationRequest::Automatic,
        )
    }

    /// Solve multiple transposed systems with an explicit numeric-factor
    /// lifecycle request shared by the entire right-hand-side block.
    pub fn solve_many_transpose_into_with_factorization(
        &mut self,
        rhs: &[Value],
        rhs_count: usize,
        solution: &mut Vec<Value>,
        factorization: FactorizationRequest,
    ) -> Result<(), SolverError> {
        self.solve_many_operation_into(
            rhs,
            rhs_count,
            solution,
            RealSolveOp::Transpose,
            factorization,
        )
    }

    fn solve_many_operation_into(
        &mut self,
        rhs: &[Value],
        rhs_count: usize,
        solution: &mut Vec<Value>,
        operation: RealSolveOp,
        factorization: FactorizationRequest,
    ) -> Result<(), SolverError> {
        self.check_stamping_error()?;
        let n = self.nrows;
        if self.solver_options.real_backend == RealSolverBackend::Auto
            && auto_prefers_supernodal_from_pattern(n, self.values.len())
        {
            self.klu_auto_rejected = true;
        }
        let required = n.checked_mul(rhs_count).ok_or_else(|| {
            SolverError::InvalidCircuit("Real batched RHS size overflow".to_string())
        })?;
        if self.ncols != n || rhs.len() != required {
            return Err(SolverError::InvalidCircuit(format!(
                "Real batched solve requires {}x{} values, got matrix {}x{} and RHS {}",
                n,
                rhs_count,
                self.nrows,
                self.ncols,
                rhs.len()
            )));
        }
        if rhs_count == 0 {
            solution.clear();
            return Ok(());
        }

        if self.solver_options.real_backend != RealSolverBackend::Faer && !self.klu_auto_rejected {
            match self.try_solve_many_klu_operation_into(
                rhs,
                rhs_count,
                solution,
                operation,
                factorization,
            ) {
                Ok(true) => return Ok(()),
                Ok(false) => {}
                Err(error) => return Err(error),
            }
        }
        self.solve_many_faer_operation_into(rhs, rhs_count, solution, operation)
    }

    fn try_solve_many_klu_operation_into(
        &mut self,
        rhs: &[Value],
        rhs_count: usize,
        solution: &mut Vec<Value>,
        operation: RealSolveOp,
        factorization: FactorizationRequest,
    ) -> Result<bool, SolverError> {
        let Self {
            nrows,
            csc,
            residual_layout,
            values,
            klu,
            klu_factored_values,
            klu_oriented_values,
            klu_auto_rejected,
            solver_options,
            residual_scratch,
            residual_gross_scratch,
            residual_compensation_scratch,
            residual_row_nnz_scratch,
            ..
        } = self;
        let n = *nrows;
        let faithful = solver_options.circuit_lu_robustness == CircuitLuRobustness::BackendFaithful;
        let backend = klu.get_or_insert_with(crate::KluSolver::new);
        if let Err(error) = configure_klu_backend(backend, *solver_options) {
            return if faithful { Err(error) } else { Ok(false) };
        }
        let values_current = values_bitwise_equal(klu_factored_values, values);
        let action = klu_numeric_action(
            backend,
            values_current,
            solver_options.numeric_factorization,
            factorization,
        )?;
        let (col_ptr, row_idx, factor_values) = prepare_klu_input(
            solver_options.circuit_lu_orientation,
            csc,
            residual_layout,
            values,
            klu_oriented_values,
            action != KluNumericAction::Reuse,
        );
        if !backend.is_analyzed_for(n)
            && let Err(error) = backend.analyze(n, col_ptr, row_idx)
        {
            klu_factored_values.clear();
            return if faithful { Err(error) } else { Ok(false) };
        }
        if let Err(error) = apply_klu_numeric_action(
            backend,
            factor_values,
            action,
            solver_options.circuit_lu_robustness,
        ) {
            klu_factored_values.clear();
            return if faithful { Err(error) } else { Ok(false) };
        }
        if action != KluNumericAction::Reuse && !values_current {
            klu_factored_values.resize(values.len(), 0.0);
            klu_factored_values.copy_from_slice(values);
        }
        if solver_options.real_backend == RealSolverBackend::Auto {
            let (l_nnz, u_nnz) = backend.factor_nnz();
            if auto_rejects_circuit_lu_fill(n, values.len(), l_nnz, u_nnz) {
                *klu_auto_rejected = true;
                backend.discard_numeric_factorization();
                klu_factored_values.clear();
                return Ok(false);
            }
        }

        let backend_operation = operation.for_circuit_lu(solver_options.circuit_lu_orientation);
        let solve_result = match backend_operation {
            RealSolveOp::Normal => backend.solve_many(rhs, rhs_count, solution),
            RealSolveOp::Transpose => backend.solve_many_transpose(rhs, rhs_count, solution),
        };
        if let Err(error) = solve_result {
            return if faithful { Err(error) } else { Ok(false) };
        }
        if faithful {
            return Ok(true);
        }
        for rhs_index in 0..rhs_count {
            let error = componentwise_backward_error_with_layout(
                csc,
                residual_layout,
                values,
                &solution[rhs_index * n..(rhs_index + 1) * n],
                &rhs[rhs_index * n..(rhs_index + 1) * n],
                residual_scratch,
                residual_gross_scratch,
                residual_compensation_scratch,
                residual_row_nnz_scratch,
                operation,
            );
            if !error.is_ok_and(BackwardError::accepted) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn solve_many_faer_operation_into(
        &mut self,
        rhs: &[Value],
        rhs_count: usize,
        solution: &mut Vec<Value>,
        operation: RealSolveOp,
    ) -> Result<(), SolverError> {
        let n = self.nrows;
        let required = n * rhs_count;
        self.ensure_lu_workspace()?;
        let accepted = {
            let par = get_global_parallelism();
            let Self {
                csc,
                residual_layout,
                values,
                lu,
                residual_scratch,
                residual_gross_scratch,
                residual_compensation_scratch,
                residual_row_nnz_scratch,
                ..
            } = self;
            let Some(ws) = lu.as_mut() else {
                return Err(SolverError::SingularMatrix);
            };
            if ws.factored_values.as_slice() != values.as_slice() {
                equilibrate_sparse_system(
                    csc,
                    values,
                    &mut ws.scaled_values,
                    &mut ws.row_scale,
                    &mut ws.col_scale,
                )?;
                let mat = SparseColMatRef::new(csc.as_ref().as_ref(), ws.scaled_values.as_slice());
                ws.symbolic
                    .factorize_numeric_lu(
                        &mut ws.numeric,
                        mat,
                        par,
                        MemStack::new(&mut ws.factor_mem),
                        Default::default(),
                    )
                    .map_err(map_faer_lu_error)?;
                ws.factored_values.resize(values.len(), 0.0);
                ws.factored_values.copy_from_slice(values);
            }
            // SAFETY: `numeric` was produced by this exact symbolic object.
            // The value cache is empty until the first successful numeric
            // factorization and changes only after another successful one.
            let lu_ref = unsafe { sparse_lu::LuRef::new_unchecked(&ws.symbolic, &ws.numeric) };
            match operation {
                RealSolveOp::Normal if rhs_count > ws.solve_rhs_capacity => {
                    ws.solve_mem = MemBuffer::try_new(
                        ws.symbolic.solve_in_place_scratch::<Value>(rhs_count, par),
                    )
                    .map_err(|_| SolverError::OutOfMemory)?;
                    ws.solve_rhs_capacity = rhs_count;
                }
                RealSolveOp::Transpose if rhs_count > ws.transpose_rhs_capacity => {
                    ws.transpose_solve_mem = MemBuffer::try_new(
                        ws.symbolic
                            .solve_transpose_in_place_scratch::<Value>(rhs_count, par),
                    )
                    .map_err(|_| SolverError::OutOfMemory)?;
                    ws.transpose_rhs_capacity = rhs_count;
                }
                _ => {}
            }
            ws.rhs.resize_with(n, rhs_count, |_, _| 0.0);
            ws.scaled_rhs.resize(required, 0.0);
            let rhs_scale = match operation {
                RealSolveOp::Normal => &ws.row_scale,
                RealSolveOp::Transpose => &ws.col_scale,
            };
            for rhs_index in 0..rhs_count {
                for row in 0..n {
                    let value = rhs[rhs_index * n + row] * rhs_scale[row];
                    if !value.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                    ws.scaled_rhs[rhs_index * n + row] = value;
                    ws.rhs[(row, rhs_index)] = value;
                }
            }
            match operation {
                RealSolveOp::Normal => lu_ref.solve_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.solve_mem),
                ),
                RealSolveOp::Transpose => lu_ref.solve_transpose_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.transpose_solve_mem),
                ),
            }
            solution.resize(required, 0.0);
            let mut all_accepted = true;
            for rhs_index in 0..rhs_count {
                for col in 0..n {
                    solution[rhs_index * n + col] = ws.rhs[(col, rhs_index)];
                }
                let error = componentwise_backward_error_with_layout(
                    csc,
                    residual_layout,
                    &ws.scaled_values,
                    &solution[rhs_index * n..(rhs_index + 1) * n],
                    &ws.scaled_rhs[rhs_index * n..(rhs_index + 1) * n],
                    residual_scratch,
                    residual_gross_scratch,
                    residual_compensation_scratch,
                    residual_row_nnz_scratch,
                    operation,
                )?;
                all_accepted &= error.accepted();
                let solution_scale = match operation {
                    RealSolveOp::Normal => &ws.col_scale,
                    RealSolveOp::Transpose => &ws.row_scale,
                };
                for col in 0..n {
                    let slot = rhs_index * n + col;
                    solution[slot] *= solution_scale[col];
                    if !solution[slot].is_finite() {
                        return Err(SolverError::Overflow);
                    }
                }
            }
            all_accepted
        };
        if accepted {
            return Ok(());
        }
        let mut refined = Vec::with_capacity(required);
        let mut one = Vec::with_capacity(n);
        for rhs_index in 0..rhs_count {
            self.solve_faer_operation_into(
                &rhs[rhs_index * n..(rhs_index + 1) * n],
                &mut one,
                operation,
            )?;
            refined.extend_from_slice(&one);
        }
        *solution = refined;
        Ok(())
    }

    /// Solve `A*x=b` into a caller-owned buffer.
    ///
    /// Reusing `solution` across Newton iterations removes the final allocation
    /// that the convenience [`Self::solve`] API necessarily performs when it
    /// transfers ownership of a fresh vector to its caller.
    pub fn solve_into(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_into_with_factorization(rhs, solution, FactorizationRequest::Automatic)
    }

    /// Solve `A*x=b` with an explicit numeric-factor lifecycle request.
    pub fn solve_into_with_factorization(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
        factorization: FactorizationRequest,
    ) -> Result<(), SolverError> {
        let n = self.nrows;
        self.check_stamping_error()?;

        if self.solver_options.real_backend == RealSolverBackend::Auto
            && auto_prefers_supernodal_from_pattern(n, self.values.len())
        {
            self.klu_auto_rejected = true;
        }

        if n != rhs.len() || self.ncols != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix size {}x{} doesn't match RHS size {}",
                n,
                self.ncols,
                rhs.len()
            )));
        }

        if self.solver_options.real_backend != RealSolverBackend::Faer && !self.klu_auto_rejected {
            match self.try_solve_klu_operation_into(
                rhs,
                solution,
                RealSolveOp::Normal,
                factorization,
            ) {
                Ok(true) => {
                    return if solution.iter().all(|value| value.is_finite()) {
                        Ok(())
                    } else {
                        Err(SolverError::SingularMatrix)
                    };
                }
                Ok(false) => {}
                Err(error) => return Err(error),
            }
        }

        self.solve_faer_into(rhs, solution)
    }

    /// Form the Newton correction right-hand side `b - A*x` without changing
    /// the stamped absolute system.
    ///
    /// Solving for a correction around the current iterate is algebraically
    /// equivalent to solving `A*x=b` directly, but it preserves substantially
    /// more forward accuracy when the iterate is already close to the answer.
    /// Dynamic-device callers may subsequently replace their own rows with a
    /// constitutive residual evaluated from state differences, avoiding loss
    /// that already occurred while forming a large absolute history source.
    pub fn correction_rhs(
        &self,
        rhs: &[Value],
        iterate: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        self.check_stamping_error()?;
        let mut correction = Vec::with_capacity(self.nrows);
        let mut correction_lo = Vec::with_capacity(self.nrows);
        fill_correction_rhs(
            self.csc.as_ref(),
            &self.values,
            self.nrows,
            self.ncols,
            rhs,
            iterate,
            CorrectionRhsWorkspace {
                correction: &mut correction,
                low_components: &mut correction_lo,
            },
        )?;
        Ok(correction)
    }

    /// Allocation-free form of [`Self::correction_rhs`].
    ///
    /// `correction` and the matrix-owned double-double low component retain
    /// their capacity across Newton iterations.
    pub fn correction_rhs_into(
        &mut self,
        rhs: &[Value],
        iterate: &[Value],
        correction: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.check_stamping_error()?;
        // Accumulate every row as a double-double expansion. Newton
        // corrections are most valuable precisely when large KCL terms nearly
        // cancel; ordinary f64 accumulation would round that small residual to
        // the same scale as the forward error we are trying to remove.
        fill_correction_rhs(
            self.csc.as_ref(),
            &self.values,
            self.nrows,
            self.ncols,
            rhs,
            iterate,
            CorrectionRhsWorkspace {
                correction,
                low_components: &mut self.correction_rhs_lo_scratch,
            },
        )
    }

    /// Solve through faer's sparse LU in equilibrated coordinates.
    ///
    /// Kept separate from backend selection so solver tests can exercise this
    /// path without mutating the process-wide `RSPICE_SOLVER` policy.
    #[cfg(test)]
    fn solve_faer(&mut self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_faer_into(rhs, &mut solution)?;
        Ok(solution)
    }

    fn solve_faer_into(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
    ) -> Result<(), SolverError> {
        self.solve_faer_operation_into(rhs, solution, RealSolveOp::Normal)
    }

    fn solve_faer_operation_into(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
        operation: RealSolveOp,
    ) -> Result<(), SolverError> {
        self.ensure_lu_workspace()?;

        let par = get_global_parallelism();
        let Self {
            csc,
            residual_layout,
            values,
            lu,
            residual_scratch,
            residual_gross_scratch,
            residual_compensation_scratch,
            residual_row_nnz_scratch,
            ..
        } = self;
        let Some(ws) = lu.as_mut() else {
            return Err(SolverError::SingularMatrix);
        };

        if ws.factored_values.as_slice() != values.as_slice() {
            equilibrate_sparse_system(
                csc,
                values,
                &mut ws.scaled_values,
                &mut ws.row_scale,
                &mut ws.col_scale,
            )?;

            let mat = SparseColMatRef::new(csc.as_ref().as_ref(), ws.scaled_values.as_slice());
            ws.symbolic
                .factorize_numeric_lu(
                    &mut ws.numeric,
                    mat,
                    par,
                    MemStack::new(&mut ws.factor_mem),
                    Default::default(),
                )
                .map_err(map_faer_lu_error)?;
            ws.factored_values.resize(values.len(), 0.0);
            ws.factored_values.copy_from_slice(values);
        }

        // SAFETY: see the batched path above. A non-empty value cache proves
        // this workspace contains a successful factorization for these values.
        let lu_ref = unsafe { sparse_lu::LuRef::new_unchecked(&ws.symbolic, &ws.numeric) };

        ws.scaled_rhs.resize(rhs.len(), 0.0);
        let rhs_scale = match operation {
            RealSolveOp::Normal => &ws.row_scale,
            RealSolveOp::Transpose => &ws.col_scale,
        };
        for ((scaled_rhs, &rhs_value), &scale) in ws.scaled_rhs.iter_mut().zip(rhs).zip(rhs_scale) {
            *scaled_rhs = rhs_value * scale;
            if !scaled_rhs.is_finite() {
                return Err(SolverError::Overflow);
            }
        }
        ws.rhs.col_as_slice_mut(0).copy_from_slice(&ws.scaled_rhs);
        match operation {
            RealSolveOp::Normal => lu_ref.solve_in_place_with_conj(
                Conj::No,
                ws.rhs.as_mut(),
                par,
                MemStack::new(&mut ws.solve_mem),
            ),
            RealSolveOp::Transpose => lu_ref.solve_transpose_in_place_with_conj(
                Conj::No,
                ws.rhs.as_mut(),
                par,
                MemStack::new(&mut ws.transpose_solve_mem),
            ),
        }

        solution.clear();
        solution.extend_from_slice(ws.rhs.col_as_slice(0));
        if solution.iter().any(|value| !value.is_finite()) {
            return Err(SolverError::SingularMatrix);
        }

        let mut backward_error = componentwise_backward_error_with_layout(
            csc,
            residual_layout,
            &ws.scaled_values,
            solution,
            &ws.scaled_rhs,
            residual_scratch,
            residual_gross_scratch,
            residual_compensation_scratch,
            residual_row_nnz_scratch,
            operation,
        )?;
        if backward_error.accepted() {
            let solution_scale = match operation {
                RealSolveOp::Normal => &ws.col_scale,
                RealSolveOp::Transpose => &ws.row_scale,
            };
            for (value, &scale) in solution.iter_mut().zip(solution_scale) {
                *value *= scale;
                if !value.is_finite() {
                    return Err(SolverError::Overflow);
                }
            }
            return Ok(());
        }

        const MAX_REFINEMENTS: usize = 5;
        const MIN_IMPROVEMENT_FACTOR: Value = 0.5;
        for _ in 0..MAX_REFINEMENTS {
            for (scaled_residual, &residual) in ws
                .rhs
                .col_as_slice_mut(0)
                .iter_mut()
                .zip(residual_scratch.iter())
            {
                *scaled_residual = residual;
            }
            match operation {
                RealSolveOp::Normal => lu_ref.solve_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.solve_mem),
                ),
                RealSolveOp::Transpose => lu_ref.solve_transpose_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.transpose_solve_mem),
                ),
            }

            for (value, &scaled_correction) in solution.iter_mut().zip(ws.rhs.col_as_slice(0)) {
                let refined = *value + scaled_correction;
                if !scaled_correction.is_finite() || !refined.is_finite() {
                    return Err(SolverError::Overflow);
                }
                *value = refined;
            }

            let refined_error = componentwise_backward_error_with_layout(
                csc,
                residual_layout,
                &ws.scaled_values,
                solution,
                &ws.scaled_rhs,
                residual_scratch,
                residual_gross_scratch,
                residual_compensation_scratch,
                residual_row_nnz_scratch,
                operation,
            )?;
            if refined_error.accepted() {
                let solution_scale = match operation {
                    RealSolveOp::Normal => &ws.col_scale,
                    RealSolveOp::Transpose => &ws.row_scale,
                };
                for (value, &scale) in solution.iter_mut().zip(solution_scale) {
                    *value *= scale;
                    if !value.is_finite() {
                        return Err(SolverError::Overflow);
                    }
                }
                return Ok(());
            }
            if refined_error.acceptance_ratio
                >= backward_error.acceptance_ratio * MIN_IMPROVEMENT_FACTOR
            {
                backward_error = refined_error;
                break;
            }
            backward_error = refined_error;
        }

        Err(SolverError::InaccurateSolution(
            backward_error.componentwise,
        ))
    }

    /// Default KLU-class real solve used by focused backend tests.
    #[cfg(test)]
    fn try_solve_klu_into(&mut self, rhs: &[Value], solution: &mut Vec<Value>) -> bool {
        self.try_solve_klu_operation_into(
            rhs,
            solution,
            RealSolveOp::Normal,
            FactorizationRequest::Automatic,
        )
        .is_ok_and(|accepted| accepted)
    }

    fn try_solve_klu_operation_into(
        &mut self,
        rhs: &[Value],
        solution: &mut Vec<Value>,
        operation: RealSolveOp,
        factorization: FactorizationRequest,
    ) -> Result<bool, SolverError> {
        let Self {
            nrows,
            csc,
            residual_layout,
            values,
            klu,
            klu_factored_values,
            klu_oriented_values,
            klu_auto_rejected,
            solver_options,
            residual_scratch,
            residual_gross_scratch,
            residual_compensation_scratch,
            residual_row_nnz_scratch,
            klu_correction_scratch,
            ..
        } = self;
        let n = *nrows;
        let faithful = solver_options.circuit_lu_robustness == CircuitLuRobustness::BackendFaithful;

        let backend = klu.get_or_insert_with(crate::KluSolver::new);
        if let Err(error) = configure_klu_backend(backend, *solver_options) {
            return if faithful { Err(error) } else { Ok(false) };
        }
        let values_current = values_bitwise_equal(klu_factored_values, values);
        let action = klu_numeric_action(
            backend,
            values_current,
            solver_options.numeric_factorization,
            factorization,
        )?;
        let (col_ptr, row_idx, factor_values) = prepare_klu_input(
            solver_options.circuit_lu_orientation,
            csc,
            residual_layout,
            values,
            klu_oriented_values,
            action != KluNumericAction::Reuse,
        );
        if !backend.is_analyzed_for(n)
            && let Err(error) = backend.analyze(n, col_ptr, row_idx)
        {
            klu_factored_values.clear();
            return if faithful { Err(error) } else { Ok(false) };
        }
        if let Err(error) = apply_klu_numeric_action(
            backend,
            factor_values,
            action,
            solver_options.circuit_lu_robustness,
        ) {
            // A numeric failure belongs to this changing Jacobian, not to the
            // frozen sparsity pattern. Auto must retry Circuit LU after
            // faer's per-solve fallback; only the measured fill policy below
            // is a persistent rejection.
            klu_factored_values.clear();
            if faithful {
                return Err(error);
            }
            static FALLBACK_LOGGED: std::sync::Once = std::sync::Once::new();
            FALLBACK_LOGGED.call_once(|| {
                log::warn!("klu backend could not factor this system; using faer fallback");
            });
            return Ok(false);
        }
        if action != KluNumericAction::Reuse && !values_current {
            klu_factored_values.resize(values.len(), 0.0);
            klu_factored_values.copy_from_slice(values);
        }
        if solver_options.real_backend == RealSolverBackend::Auto {
            let (l_nnz, u_nnz) = backend.factor_nnz();
            let factor_nnz = l_nnz.saturating_add(u_nnz);
            let excessive_fill = auto_rejects_circuit_lu_fill(n, values.len(), l_nnz, u_nnz);
            if excessive_fill {
                *klu_auto_rejected = true;
                backend.discard_numeric_factorization();
                klu_factored_values.clear();
                log::debug!(
                    "auto backend retained faer for {}x{} pattern: Circuit LU fill {} / A nnz {}",
                    n,
                    n,
                    factor_nnz,
                    values.len()
                );
                return Ok(false);
            }
        }
        let backend_operation = operation.for_circuit_lu(solver_options.circuit_lu_orientation);
        let solve_result = match backend_operation {
            RealSolveOp::Normal => backend.solve(rhs, solution),
            RealSolveOp::Transpose => backend.solve_transpose(rhs, solution),
        };
        if let Err(error) = solve_result {
            return if faithful { Err(error) } else { Ok(false) };
        }
        if faithful {
            return Ok(true);
        }
        let mut backward_error = match componentwise_backward_error_with_layout(
            csc,
            residual_layout,
            values,
            solution,
            rhs,
            residual_scratch,
            residual_gross_scratch,
            residual_compensation_scratch,
            residual_row_nnz_scratch,
            operation,
        ) {
            Ok(error) => error,
            Err(_) => return Ok(false),
        };
        if backward_error.accepted() {
            return Ok(true);
        }

        // KLU's unscaled factors are substantially faster on the ordinary
        // Newton hot path, but ill-scaled MNA rows (for example a 1e12-ohm
        // bias path beside ideal source constraints) can require refinement.
        // Reuse the same factors to solve A*delta = b-A*x, and accept only a
        // componentwise backward error comparable to the equilibrated faer
        // backend. Failure falls through to faer instead of exposing a
        // finite-but-inaccurate circuit state.
        const MAX_KLU_REFINEMENTS: usize = 5;
        const MIN_IMPROVEMENT_FACTOR: Value = 0.5;
        for _ in 0..MAX_KLU_REFINEMENTS {
            let correction_result = match backend_operation {
                RealSolveOp::Normal => backend.solve(residual_scratch, klu_correction_scratch),
                RealSolveOp::Transpose => {
                    backend.solve_transpose(residual_scratch, klu_correction_scratch)
                }
            };
            if correction_result.is_err() {
                return Ok(false);
            }
            for (value, &delta) in solution.iter_mut().zip(klu_correction_scratch.iter()) {
                let refined = *value + delta;
                if !delta.is_finite() || !refined.is_finite() {
                    return Ok(false);
                }
                *value = refined;
            }
            let refined_error = match componentwise_backward_error_with_layout(
                csc,
                residual_layout,
                values,
                solution,
                rhs,
                residual_scratch,
                residual_gross_scratch,
                residual_compensation_scratch,
                residual_row_nnz_scratch,
                operation,
            ) {
                Ok(error) => error,
                Err(_) => return Ok(false),
            };
            if refined_error.accepted() {
                return Ok(true);
            }
            if refined_error.acceptance_ratio
                >= backward_error.acceptance_ratio * MIN_IMPROVEMENT_FACTOR
            {
                break;
            }
            backward_error = refined_error;
        }
        Ok(false)
    }

    /// Solve Ax = b via dense Gaussian elimination.
    ///
    /// This is used as a high-stability fallback for small linear systems with
    /// strong transformer/coupling fill-in where sparse LU can become noisy.
    pub fn solve_dense(&self, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
        self.check_stamping_error()?;
        if self.nrows != rhs.len() || self.ncols != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Dense solve requires a square matrix matching RHS size, got {}x{} with RHS {}",
                self.nrows,
                self.ncols,
                rhs.len()
            )));
        }

        let solution = solve_gauss(self.to_dense_real(), rhs.to_vec())?;
        let mut residual = Vec::new();
        let mut denominator = Vec::new();
        let mut compensation = Vec::new();
        let mut row_nnz = Vec::new();
        let backward_error = componentwise_backward_error_with_layout(
            &self.csc,
            &self.residual_layout,
            &self.values,
            &solution,
            rhs,
            &mut residual,
            &mut denominator,
            &mut compensation,
            &mut row_nnz,
            RealSolveOp::Normal,
        )?;
        if backward_error.accepted() {
            Ok(solution)
        } else {
            Err(SolverError::InaccurateSolution(
                backward_error.componentwise,
            ))
        }
    }
}

//=============================================================================
// Complex Matrix for AC Analysis
//=============================================================================

use num_complex::Complex64;

/// Reusable complex sparse-LU workspace (see [`LuWorkspace`]).
struct ComplexLuWorkspace {
    symbolic: Arc<sparse_lu::SymbolicLu<usize>>,
    numeric: sparse_lu::NumericLu<usize, Complex64>,
    factor_mem: MemBuffer,
    solve_mem: MemBuffer,
    transpose_solve_mem: MemBuffer,
    solve_rhs_capacity: usize,
    transpose_rhs_capacity: usize,
    rhs: Mat<Complex64>,
    scaled_values: Vec<Complex64>,
    scaled_rhs: Vec<Complex64>,
    scaled_denominator_floor: Vec<Value>,
    row_scale: Vec<Value>,
    col_scale: Vec<Value>,
    residual: Vec<Complex64>,
    denominator: Vec<Value>,
    compensation: Vec<Complex64>,
    row_nnz: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ComplexSolveOp {
    Normal,
    Transpose,
    Adjoint,
}

#[inline]
fn complex_is_finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

#[inline]
fn complex_abs1(value: Complex64) -> Value {
    (value.re.abs() + value.im.abs()).min(Value::MAX)
}

/// Build `D_r * A * D_c` for a complex matrix. Positive real diagonal
/// scalings preserve phase while bringing both axes into a numerically safe
/// range for sparse pivoting.
fn equilibrate_complex_matrix(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Complex64],
    scaled_values: &mut Vec<Complex64>,
    row_scale: &mut Vec<Value>,
    col_scale: &mut Vec<Value>,
) -> Result<(), SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() {
        return Err(SolverError::InvalidCircuit(
            "Complex sparse equilibration dimension mismatch".to_string(),
        ));
    }

    scaled_values.resize(values.len(), Complex64::new(0.0, 0.0));
    row_scale.resize(nrows, 1.0);
    col_scale.resize(ncols, 1.0);

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        let mut max_abs: Value = 0.0;
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let value = values[idx];
            if !complex_is_finite(value) {
                return Err(SolverError::Overflow);
            }
            max_abs = max_abs.max(complex_abs1(value));
        }
        if max_abs == 0.0 {
            return Err(SolverError::SingularMatrix);
        }
        let scale = finite_reciprocal_scale(max_abs);
        col_scale[col] = scale;
        for idx in col_ptr[col]..col_ptr[col + 1] {
            scaled_values[idx] = values[idx] * scale;
        }
    }

    row_scale.fill(0.0);
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let row = row_idx[idx];
            row_scale[row] = row_scale[row].max(complex_abs1(scaled_values[idx]));
        }
    }
    for scale in row_scale.iter_mut() {
        if *scale == 0.0 {
            return Err(SolverError::SingularMatrix);
        }
        *scale = finite_reciprocal_scale(*scale);
    }
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            scaled_values[idx] *= row_scale[row_idx[idx]];
        }
    }
    if scaled_values
        .iter()
        .copied()
        .any(|value| !complex_is_finite(value))
    {
        return Err(SolverError::Overflow);
    }
    Ok(())
}

fn scale_complex_rhs(
    rhs: &[Complex64],
    row_scale: &[Value],
    scaled_rhs: &mut Vec<Complex64>,
) -> Result<(), SolverError> {
    if rhs.len() != row_scale.len() {
        return Err(SolverError::InvalidCircuit(
            "Complex RHS scaling dimension mismatch".to_string(),
        ));
    }
    scaled_rhs.resize(rhs.len(), Complex64::new(0.0, 0.0));
    for ((scaled, &value), &scale) in scaled_rhs.iter_mut().zip(rhs).zip(row_scale) {
        if !complex_is_finite(value) {
            return Err(SolverError::Overflow);
        }
        *scaled = value * scale;
        if !complex_is_finite(*scaled) {
            return Err(SolverError::Overflow);
        }
    }
    Ok(())
}

fn scale_complex_denominator_floor(
    floor: &[Value],
    row_scale: &[Value],
    scaled_floor: &mut Vec<Value>,
) -> Result<(), SolverError> {
    if floor.len() != row_scale.len() {
        return Err(SolverError::InvalidCircuit(
            "Complex denominator-floor scaling dimension mismatch".to_string(),
        ));
    }
    scaled_floor.resize(floor.len(), 0.0);
    for ((scaled, &value), &scale) in scaled_floor.iter_mut().zip(floor).zip(row_scale) {
        if !value.is_finite() || value < 0.0 || !scale.is_finite() || scale <= 0.0 {
            return Err(SolverError::InvalidCircuit(
                "Complex solve denominator floors must be finite and non-negative".to_string(),
            ));
        }
        *scaled = value * scale;
        if !scaled.is_finite() {
            return Err(SolverError::Overflow);
        }
    }
    Ok(())
}

#[inline]
fn dd_add(hi: &mut Value, lo: &mut Value, add_hi: Value, add_lo: Value) {
    let sum = *hi + add_hi;
    let virtual_addend = sum - *hi;
    let sum_error = (*hi - (sum - virtual_addend)) + (add_hi - virtual_addend);
    let tail = *lo + add_lo + sum_error;
    let refined = sum + tail;
    *lo = tail - (refined - sum);
    *hi = refined;
}

#[inline]
fn complex_dd_subtract(
    residual: &mut Complex64,
    compensation: &mut Complex64,
    value: Complex64,
    x: Complex64,
) {
    let product_re_1 = (-value.re) * x.re;
    let product_re_1_lo = (-value.re).mul_add(x.re, -product_re_1);
    let product_re_2 = value.im * x.im;
    let product_re_2_lo = value.im.mul_add(x.im, -product_re_2);
    let product_im_1 = (-value.re) * x.im;
    let product_im_1_lo = (-value.re).mul_add(x.im, -product_im_1);
    let product_im_2 = (-value.im) * x.re;
    let product_im_2_lo = (-value.im).mul_add(x.re, -product_im_2);
    dd_add(
        &mut residual.re,
        &mut compensation.re,
        product_re_1,
        product_re_1_lo,
    );
    dd_add(
        &mut residual.re,
        &mut compensation.re,
        product_re_2,
        product_re_2_lo,
    );
    dd_add(
        &mut residual.im,
        &mut compensation.im,
        product_im_1,
        product_im_1_lo,
    );
    dd_add(
        &mut residual.im,
        &mut compensation.im,
        product_im_2,
        product_im_2_lo,
    );
}

#[allow(clippy::too_many_arguments)]
fn complex_componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Complex64],
    solution: &[Complex64],
    rhs: &[Complex64],
    denominator_floor: Option<&[Value]>,
    residual: &mut Vec<Complex64>,
    denominator: &mut Vec<Value>,
    compensation: &mut Vec<Complex64>,
    row_nnz: &mut Vec<usize>,
    operation: ComplexSolveOp,
) -> Result<BackwardError, SolverError> {
    if let Some(floor) = denominator_floor
        && (floor.len() != rhs.len()
            || floor.iter().any(|value| !value.is_finite() || *value < 0.0))
    {
        return Err(SolverError::InvalidCircuit(
            "Complex backward-error denominator floors must match the RHS and be finite and non-negative"
                .to_string(),
        ));
    }
    let fast = fast_complex_componentwise_backward_error(
        csc,
        values,
        solution,
        rhs,
        denominator_floor,
        residual,
        denominator,
        row_nnz,
        operation,
    )?;
    if fast.accepted() {
        return Ok(fast);
    }
    compensated_complex_componentwise_backward_error(
        csc,
        values,
        solution,
        rhs,
        denominator_floor,
        residual,
        denominator,
        compensation,
        row_nnz,
        operation,
    )
}

#[allow(clippy::too_many_arguments)]
fn fast_complex_componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Complex64],
    solution: &[Complex64],
    rhs: &[Complex64],
    denominator_floor: Option<&[Value]>,
    residual: &mut Vec<Complex64>,
    denominator: &mut Vec<Value>,
    row_nnz: &mut Vec<usize>,
    operation: ComplexSolveOp,
) -> Result<BackwardError, SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() || solution.len() != ncols || rhs.len() != nrows {
        return Err(SolverError::InvalidCircuit(
            "Complex sparse backward-error dimension mismatch".to_string(),
        ));
    }
    residual.resize(nrows, Complex64::new(0.0, 0.0));
    denominator.resize(nrows, 0.0);
    row_nnz.resize(nrows, 0);
    row_nnz.fill(0);
    for row in 0..nrows {
        if !complex_is_finite(rhs[row]) {
            return Err(SolverError::Overflow);
        }
        residual[row] = rhs[row];
        denominator[row] = complex_abs1(rhs[row]);
    }
    for col in 0..ncols {
        for index in csc.col_ptr()[col]..csc.col_ptr()[col + 1] {
            let original_row = csc.row_idx()[index];
            let (equation, x_index, value) = match operation {
                ComplexSolveOp::Normal => (original_row, col, values[index]),
                ComplexSolveOp::Transpose => (col, original_row, values[index]),
                ComplexSolveOp::Adjoint => (col, original_row, values[index].conj()),
            };
            let x = solution[x_index];
            let product = value * x;
            let magnitude = complex_abs1(value) * complex_abs1(x);
            if !complex_is_finite(value)
                || !complex_is_finite(x)
                || !complex_is_finite(product)
                || !magnitude.is_finite()
            {
                return Err(SolverError::Overflow);
            }
            residual[equation] -= product;
            if value != Complex64::new(0.0, 0.0) {
                row_nnz[equation] = row_nnz[equation].saturating_add(1);
            }
            denominator[equation] = (denominator[equation] + magnitude).min(Value::MAX);
        }
    }
    let mut error: Value = 0.0;
    let mut acceptance_ratio: Value = 0.0;
    for row in 0..nrows {
        let safe1 = (row_nnz[row].saturating_add(1) as Value) * Value::MIN_POSITIVE;
        let residual_abs = complex_abs1(residual[row]);
        let scale = denominator[row].max(denominator_floor.map_or(0.0, |floor| floor[row]));
        if !residual_abs.is_finite() || !scale.is_finite() {
            return Err(SolverError::Overflow);
        }
        let row_error = residual_abs / scale.max(safe1);
        // One complex multiply/subtract is bounded by several scalar
        // roundings; sixteen epsilons per active term is conservative.
        let certified_error =
            row_error + 16.0 * (row_nnz[row].saturating_add(1) as Value) * Value::EPSILON;
        error = error.max(row_error);
        acceptance_ratio =
            acceptance_ratio.max(certified_error / backward_error_tolerance(row_nnz[row]));
    }
    Ok(BackwardError {
        componentwise: error,
        acceptance_ratio,
    })
}

#[allow(clippy::too_many_arguments)]
fn compensated_complex_componentwise_backward_error(
    csc: &SymbolicSparseColMat<usize>,
    values: &[Complex64],
    solution: &[Complex64],
    rhs: &[Complex64],
    denominator_floor: Option<&[Value]>,
    residual: &mut Vec<Complex64>,
    denominator: &mut Vec<Value>,
    compensation: &mut Vec<Complex64>,
    row_nnz: &mut Vec<usize>,
    operation: ComplexSolveOp,
) -> Result<BackwardError, SolverError> {
    let nrows = csc.nrows();
    let ncols = csc.ncols();
    if values.len() != csc.row_idx().len() || solution.len() != ncols || rhs.len() != nrows {
        return Err(SolverError::InvalidCircuit(
            "Complex sparse backward-error dimension mismatch".to_string(),
        ));
    }

    residual.resize(nrows, Complex64::new(0.0, 0.0));
    denominator.resize(nrows, 0.0);
    compensation.resize(nrows, Complex64::new(0.0, 0.0));
    compensation.fill(Complex64::new(0.0, 0.0));
    row_nnz.resize(nrows, 0);
    row_nnz.fill(0);
    for row in 0..nrows {
        if !complex_is_finite(rhs[row]) {
            return Err(SolverError::Overflow);
        }
        residual[row] = rhs[row];
        denominator[row] = complex_abs1(rhs[row]);
    }

    let col_ptr = csc.col_ptr();
    let row_idx = csc.row_idx();
    for col in 0..ncols {
        for idx in col_ptr[col]..col_ptr[col + 1] {
            let original_row = row_idx[idx];
            let (equation, x_index, value) = match operation {
                ComplexSolveOp::Normal => (original_row, col, values[idx]),
                ComplexSolveOp::Transpose => (col, original_row, values[idx]),
                ComplexSolveOp::Adjoint => (col, original_row, values[idx].conj()),
            };
            let x = solution[x_index];
            let magnitude = complex_abs1(value) * complex_abs1(x);
            if !complex_is_finite(value) || !complex_is_finite(x) || !magnitude.is_finite() {
                return Err(SolverError::Overflow);
            }
            if value != Complex64::new(0.0, 0.0) {
                row_nnz[equation] = row_nnz[equation].saturating_add(1);
            }
            complex_dd_subtract(
                &mut residual[equation],
                &mut compensation[equation],
                value,
                x,
            );
            denominator[equation] = (denominator[equation] + magnitude).min(Value::MAX);
        }
    }

    let mut error: Value = 0.0;
    let mut acceptance_ratio: Value = 0.0;
    for row in 0..nrows {
        residual[row] += compensation[row];
        let safe1 = (row_nnz[row].saturating_add(1) as Value) * Value::MIN_POSITIVE;
        let residual_abs = complex_abs1(residual[row]);
        let scale = denominator[row].max(denominator_floor.map_or(0.0, |floor| floor[row]));
        if !residual_abs.is_finite() || !scale.is_finite() {
            return Err(SolverError::Overflow);
        }
        let row_error = residual_abs / scale.max(safe1);
        error = error.max(row_error);
        acceptance_ratio = acceptance_ratio.max(row_error / backward_error_tolerance(row_nnz[row]));
    }
    Ok(BackwardError {
        componentwise: error,
        acceptance_ratio,
    })
}

/// ComplexMatrix for AC small-signal analysis
///
/// Shares the same sparsity structure as a StaticMatrix but uses Complex64 values.
/// This enables AC analysis at different frequencies without rebuilding topology.
pub struct ComplexMatrix {
    /// Matrix row count.
    pub nrows: usize,
    /// Matrix column count.
    pub ncols: usize,
    /// Frozen CSC sparsity pattern (cloned from the real matrix once)
    csc: Arc<SymbolicSparseColMat<usize>>,
    /// Complex values (updated for each frequency)
    values: Vec<Complex64>,
    /// Identity of the real sparsity pattern from which this matrix was made.
    pattern_id: u64,
    /// Reusable LU workspace; the symbolic part is shared with the real
    /// matrix when available (symbolic LU is scalar-type independent).
    lu: Option<ComplexLuWorkspace>,
    /// True while `lu.numeric` holds the factorization of the current
    /// `values`. Consecutive solves against unchanged values (noise analysis
    /// solves one matrix against many excitation vectors) skip refactorizing.
    factorization_valid: bool,
    /// First attempted stamp outside the frozen sparsity pattern.
    stamping_error: Option<MatrixStampError>,
}

impl ComplexMatrix {
    /// Create a ComplexMatrix with the same structure as a StaticMatrix
    pub fn from_real_structure(real_matrix: &StaticMatrix) -> Self {
        let nnz = real_matrix.values.len();
        let mut this = Self {
            nrows: real_matrix.nrows,
            ncols: real_matrix.ncols,
            csc: real_matrix.csc.clone(),
            values: vec![Complex64::new(0.0, 0.0); nnz],
            pattern_id: real_matrix.pattern_id,
            lu: None,
            factorization_valid: false,
            stamping_error: None,
        };
        // Reuse the real matrix's symbolic analysis when faer has already been
        // selected or used as a fallback. A successful default KLU-only DC
        // solve deliberately does not pay this independent symbolic cost.
        if let Some(symbolic) = real_matrix.lu.as_ref().map(|ws| ws.symbolic.clone()) {
            this.lu = Self::workspace_from_symbolic(this.nrows, symbolic).ok();
        }
        this
    }

    fn workspace_from_symbolic(
        nrows: usize,
        symbolic: Arc<sparse_lu::SymbolicLu<usize>>,
    ) -> Result<ComplexLuWorkspace, SolverError> {
        let par = get_global_parallelism();
        let factor_mem = MemBuffer::try_new(
            symbolic.factorize_numeric_lu_scratch::<Complex64>(par, Default::default()),
        )
        .map_err(|_| SolverError::OutOfMemory)?;
        let solve_mem = MemBuffer::try_new(symbolic.solve_in_place_scratch::<Complex64>(1, par))
            .map_err(|_| SolverError::OutOfMemory)?;
        let transpose_solve_mem =
            MemBuffer::try_new(symbolic.solve_transpose_in_place_scratch::<Complex64>(1, par))
                .map_err(|_| SolverError::OutOfMemory)?;
        Ok(ComplexLuWorkspace {
            symbolic,
            numeric: sparse_lu::NumericLu::new(),
            factor_mem,
            solve_mem,
            transpose_solve_mem,
            solve_rhs_capacity: 1,
            transpose_rhs_capacity: 1,
            rhs: Mat::zeros(nrows, 1),
            scaled_values: Vec::new(),
            scaled_rhs: Vec::new(),
            scaled_denominator_floor: Vec::new(),
            row_scale: Vec::new(),
            col_scale: Vec::new(),
            residual: Vec::new(),
            denominator: Vec::new(),
            compensation: Vec::new(),
            row_nnz: Vec::new(),
        })
    }

    /// Zero all values
    #[inline]
    pub fn clear_values(&mut self) {
        self.values.fill(Complex64::new(0.0, 0.0));
        self.factorization_valid = false;
    }

    /// Add real value at (row, col)
    #[inline]
    pub fn add_real(&mut self, row: usize, col: usize, value: Value) {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                self.record_missing_position("ComplexMatrix::add_real", row, col);
                return;
            }
        };
        self.values[idx] += Complex64::new(value, 0.0);
        self.factorization_valid = false;
    }

    /// Add complex value at (row, col)
    #[inline]
    pub fn add(&mut self, row: usize, col: usize, value: Complex64) {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                self.record_missing_position("ComplexMatrix::add", row, col);
                return;
            }
        };
        self.values[idx] += value;
        self.factorization_valid = false;
    }

    /// Add imaginary value (for frequency-dependent terms like jwC)
    #[inline]
    pub fn add_imag(&mut self, row: usize, col: usize, value: Value) {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                self.record_missing_position("ComplexMatrix::add_imag", row, col);
                return;
            }
        };
        self.values[idx] += Complex64::new(0.0, value);
        self.factorization_valid = false;
    }

    /// Direct real add using a precomputed CSC index.
    #[inline]
    pub fn stamp_direct_real(&mut self, idx: CscIndex, value: Value) {
        if !self.validate_direct_index("ComplexMatrix::stamp_direct_real", idx) {
            return;
        }
        self.values[idx.0] += Complex64::new(value, 0.0);
        self.factorization_valid = false;
    }

    /// Direct imaginary add using a precomputed CSC index.
    #[inline]
    pub fn stamp_direct_imag(&mut self, idx: CscIndex, value: Value) {
        if !self.validate_direct_index("ComplexMatrix::stamp_direct_imag", idx) {
            return;
        }
        self.values[idx.0] += Complex64::new(0.0, value);
        self.factorization_valid = false;
    }

    /// Checked real add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add_real(
        &mut self,
        row: usize,
        col: usize,
        value: Value,
    ) -> Result<(), SolverError> {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                let missing = missing_matrix_position("ComplexMatrix::try_add_real", row, col);
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx].re + value;
        if !value.is_finite() || !combined.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.values[idx].re = combined;
        self.factorization_valid = false;
        Ok(())
    }

    /// Checked complex add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add(&mut self, row: usize, col: usize, value: Complex64) -> Result<(), SolverError> {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                let missing = missing_matrix_position("ComplexMatrix::try_add", row, col);
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx] + value;
        if !complex_is_finite(value) || !complex_is_finite(combined) {
            return Err(SolverError::Overflow);
        }
        self.values[idx] = combined;
        self.factorization_valid = false;
        Ok(())
    }

    /// Checked imaginary add for callers that want an immediate structural error.
    #[inline]
    pub fn try_add_imag(
        &mut self,
        row: usize,
        col: usize,
        value: Value,
    ) -> Result<(), SolverError> {
        let idx = match find_csc_offset(&self.csc, row, col) {
            Some(idx) => idx,
            None => {
                let missing = missing_matrix_position("ComplexMatrix::try_add_imag", row, col);
                self.stamping_error
                    .get_or_insert(MatrixStampError::MissingPosition(missing));
                return Err(missing.into_solver_error());
            }
        };
        let combined = self.values[idx].im + value;
        if !value.is_finite() || !combined.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.values[idx].im = combined;
        self.factorization_valid = false;
        Ok(())
    }

    #[inline]
    fn record_missing_position(&mut self, method: &'static str, row: usize, col: usize) {
        self.stamping_error.get_or_insert_with(|| {
            MatrixStampError::MissingPosition(missing_matrix_position(method, row, col))
        });
    }

    #[inline]
    fn validate_direct_index(&mut self, method: &'static str, idx: CscIndex) -> bool {
        if idx.1 == self.pattern_id && idx.0 < self.values.len() {
            return true;
        }
        self.stamping_error
            .get_or_insert(MatrixStampError::InvalidIndex {
                method,
                offset: idx.0,
                index_pattern: idx.1,
                matrix_pattern: self.pattern_id,
            });
        false
    }

    #[inline]
    fn check_stamping_error(&self) -> Result<(), SolverError> {
        match self.stamping_error {
            Some(error) => Err(error.into_solver_error()),
            None => Ok(()),
        }
    }

    /// Materialize the real part of the sparse matrix as a dense matrix.
    pub fn to_dense_real(&self) -> Vec<Vec<Value>> {
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut dense = vec![vec![0.0; self.ncols]; self.nrows];
        for col in 0..self.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                dense[row][col] = self.values[idx].re;
            }
        }
        dense
    }

    /// Materialize the imaginary part of the sparse matrix as a dense matrix.
    pub fn to_dense_imag(&self) -> Vec<Vec<Value>> {
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut dense = vec![vec![0.0; self.ncols]; self.nrows];
        for col in 0..self.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let row = row_idx[idx];
                dense[row][col] = self.values[idx].im;
            }
        }
        dense
    }

    /// Visit the frozen CSC entries without materializing a dense matrix.
    /// Entries are yielded in deterministic column-major order, including
    /// structural zeros. Descriptor and diagnostic algorithms can filter the
    /// numeric values they need while preserving the production matrix's
    /// sparse memory footprint.
    pub fn for_each_stored(&self, mut visitor: impl FnMut(usize, usize, Complex64)) {
        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        for col in 0..self.ncols {
            for index in col_ptr[col]..col_ptr[col + 1] {
                visitor(row_idx[index], col, self.values[index]);
            }
        }
    }

    /// Multiply the current sparse matrix by a complex vector without
    /// factorizing it.
    ///
    /// Distortion and sensitivity analyses use this to contract directional
    /// derivatives of the small-signal MNA operator while retaining the
    /// circuit's sparse structure.
    pub fn multiply_vector(&self, vector: &[Complex64]) -> Result<Vec<Complex64>, SolverError> {
        self.check_stamping_error()?;
        if self.ncols != vector.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Matrix column count {} doesn't match vector length {}",
                self.ncols,
                vector.len()
            )));
        }

        let col_ptr = self.csc.col_ptr();
        let row_idx = self.csc.row_idx();
        let mut product = vec![Complex64::new(0.0, 0.0); self.nrows];
        for col in 0..self.ncols {
            let input = vector[col];
            if !complex_is_finite(input) {
                return Err(SolverError::Overflow);
            }
            if input == Complex64::new(0.0, 0.0) {
                continue;
            }
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let term = self.values[idx] * input;
                if !complex_is_finite(self.values[idx]) || !complex_is_finite(term) {
                    return Err(SolverError::Overflow);
                }
                product[row_idx[idx]] += term;
                if !complex_is_finite(product[row_idx[idx]]) {
                    return Err(SolverError::Overflow);
                }
            }
        }
        Ok(product)
    }

    /// Certify a caller-adjusted complex solution against this matrix and RHS.
    ///
    /// This uses the same compensated, componentwise backward-error criterion
    /// as the production solve. It is intended for exact-constraint
    /// canonicalization performed after factorization (for example, restoring
    /// an ideal voltage-source equation after equilibration/unscaling) and
    /// reuses the existing LU residual workspace without steady-state
    /// allocation.
    pub fn certify_solution(
        &mut self,
        solution: &[Complex64],
        rhs: &[Complex64],
    ) -> Result<(), SolverError> {
        self.check_stamping_error()?;
        if self.nrows != rhs.len() || self.ncols != solution.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Complex certification dimension mismatch: matrix is {}x{}, solution has {}, RHS has {}",
                self.nrows,
                self.ncols,
                solution.len(),
                rhs.len()
            )));
        }

        if !self.factorization_valid {
            return Err(SolverError::InvalidCircuit(
                "Complex solution certification requires a successful solve of the current matrix"
                    .to_string(),
            ));
        }
        let workspace = self.lu.as_mut().ok_or_else(|| {
            SolverError::InvalidCircuit(
                "Complex solution certification is missing its solve workspace".to_string(),
            )
        })?;
        let ComplexLuWorkspace {
            rhs: candidate_workspace,
            scaled_values,
            scaled_rhs,
            row_scale,
            col_scale,
            residual,
            denominator,
            compensation,
            row_nnz,
            ..
        } = workspace;
        if col_scale.len() != solution.len() || candidate_workspace.nrows() != solution.len() {
            return Err(SolverError::InvalidCircuit(
                "Complex solution certification scale dimension mismatch".to_string(),
            ));
        }
        scale_complex_rhs(rhs, row_scale, scaled_rhs)?;
        let scaled_candidate = candidate_workspace.col_as_slice_mut(0);
        for ((scaled, &value), &scale) in scaled_candidate
            .iter_mut()
            .zip(solution)
            .zip(col_scale.iter())
        {
            if !complex_is_finite(value) || !scale.is_finite() || scale <= 0.0 {
                return Err(SolverError::Overflow);
            }
            *scaled = value / scale;
            if !complex_is_finite(*scaled) {
                return Err(SolverError::Overflow);
            }
        }
        let error = complex_componentwise_backward_error(
            &self.csc,
            scaled_values,
            scaled_candidate,
            scaled_rhs,
            None,
            residual,
            denominator,
            compensation,
            row_nnz,
            ComplexSolveOp::Normal,
        )?;
        if error.accepted() {
            Ok(())
        } else {
            Err(SolverError::InaccurateSolution(error.componentwise))
        }
    }

    /// Solve Ax = b for complex values.
    ///
    /// The symbolic analysis is computed once per structure (or inherited
    /// from the real matrix) and the numeric factorization is reused across
    /// consecutive solves with unchanged values.
    pub fn solve(&mut self, rhs: &[Complex64]) -> Result<Vec<Complex64>, SolverError> {
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_into(rhs, &mut solution)?;
        Ok(solution)
    }

    /// Solve into a caller-owned complex output buffer, reusing its allocation
    /// across frequency points or multiple noise excitations.
    pub fn solve_into(
        &mut self,
        rhs: &[Complex64],
        solution: &mut Vec<Complex64>,
    ) -> Result<(), SolverError> {
        self.solve_operation_into(rhs, solution, ComplexSolveOp::Normal, None)
    }

    /// Solve `A*x=b` while applying caller-supplied physical denominator
    /// floors to selected rows of the backward-error check.
    ///
    /// A nonzero floor is appropriate only when the caller knows the physical
    /// coordinate scale of a homogeneous equation. The floor is transformed
    /// by the matrix row equilibration and affects solve acceptance only; it
    /// does not modify the matrix, RHS, factorization, or returned solution.
    /// [`Self::certify_solution`] intentionally remains strict and floor-free.
    pub fn solve_with_row_denominator_floors(
        &mut self,
        rhs: &[Complex64],
        denominator_floor: &[Value],
    ) -> Result<Vec<Complex64>, SolverError> {
        let mut solution = Vec::with_capacity(rhs.len());
        self.solve_operation_into(
            rhs,
            &mut solution,
            ComplexSolveOp::Normal,
            Some(denominator_floor),
        )?;
        Ok(solution)
    }

    /// Solve multiple complex systems with one cached factorization and one
    /// batched triangular solve. Inputs and outputs are column-major with `n`
    /// consecutive values per right-hand side.
    pub fn solve_many_into(
        &mut self,
        rhs: &[Complex64],
        rhs_count: usize,
        solution: &mut Vec<Complex64>,
    ) -> Result<(), SolverError> {
        self.solve_many_operation_into(rhs, rhs_count, solution, ComplexSolveOp::Normal)
    }

    /// Solve multiple `A^T X = B` systems in one batched triangular pass.
    pub fn solve_many_transpose_into(
        &mut self,
        rhs: &[Complex64],
        rhs_count: usize,
        solution: &mut Vec<Complex64>,
    ) -> Result<(), SolverError> {
        self.solve_many_operation_into(rhs, rhs_count, solution, ComplexSolveOp::Transpose)
    }

    /// Solve multiple `A^H X = B` systems in one batched triangular pass.
    pub fn solve_many_adjoint_into(
        &mut self,
        rhs: &[Complex64],
        rhs_count: usize,
        solution: &mut Vec<Complex64>,
    ) -> Result<(), SolverError> {
        self.solve_many_operation_into(rhs, rhs_count, solution, ComplexSolveOp::Adjoint)
    }

    fn solve_many_operation_into(
        &mut self,
        rhs: &[Complex64],
        rhs_count: usize,
        solution: &mut Vec<Complex64>,
        operation: ComplexSolveOp,
    ) -> Result<(), SolverError> {
        self.check_stamping_error()?;
        let n = self.nrows;
        let required = n.checked_mul(rhs_count).ok_or_else(|| {
            SolverError::InvalidCircuit("Complex batched RHS size overflow".to_string())
        })?;
        if self.ncols != n || rhs.len() != required {
            return Err(SolverError::InvalidCircuit(format!(
                "Complex batched solve requires {}x{} values, got matrix {}x{} and RHS {}",
                n,
                rhs_count,
                self.nrows,
                self.ncols,
                rhs.len()
            )));
        }
        if rhs_count == 0 {
            solution.clear();
            return Ok(());
        }
        if self.lu.is_none() {
            let symbolic =
                sparse_lu::factorize_symbolic_lu(self.csc.as_ref().as_ref(), Default::default())
                    .map_err(map_faer_error)?;
            self.lu = Some(Self::workspace_from_symbolic(n, Arc::new(symbolic))?);
            self.factorization_valid = false;
        }

        let accepted = {
            let par = get_global_parallelism();
            let Self {
                csc,
                values,
                lu,
                factorization_valid,
                ..
            } = self;
            let Some(ws) = lu.as_mut() else {
                return Err(SolverError::SingularMatrix);
            };
            if !*factorization_valid {
                equilibrate_complex_matrix(
                    csc,
                    values,
                    &mut ws.scaled_values,
                    &mut ws.row_scale,
                    &mut ws.col_scale,
                )?;
                let mat = SparseColMatRef::new(csc.as_ref().as_ref(), ws.scaled_values.as_slice());
                ws.symbolic
                    .factorize_numeric_lu(
                        &mut ws.numeric,
                        mat,
                        par,
                        MemStack::new(&mut ws.factor_mem),
                        Default::default(),
                    )
                    .map_err(map_faer_lu_error)?;
                *factorization_valid = true;
            }
            match operation {
                ComplexSolveOp::Normal if rhs_count > ws.solve_rhs_capacity => {
                    ws.solve_mem = MemBuffer::try_new(
                        ws.symbolic
                            .solve_in_place_scratch::<Complex64>(rhs_count, par),
                    )
                    .map_err(|_| SolverError::OutOfMemory)?;
                    ws.solve_rhs_capacity = rhs_count;
                }
                ComplexSolveOp::Transpose | ComplexSolveOp::Adjoint
                    if rhs_count > ws.transpose_rhs_capacity =>
                {
                    ws.transpose_solve_mem = MemBuffer::try_new(
                        ws.symbolic
                            .solve_transpose_in_place_scratch::<Complex64>(rhs_count, par),
                    )
                    .map_err(|_| SolverError::OutOfMemory)?;
                    ws.transpose_rhs_capacity = rhs_count;
                }
                _ => {}
            }
            let rhs_scale = match operation {
                ComplexSolveOp::Normal => &ws.row_scale,
                ComplexSolveOp::Transpose | ComplexSolveOp::Adjoint => &ws.col_scale,
            };
            ws.rhs
                .resize_with(n, rhs_count, |_, _| Complex64::new(0.0, 0.0));
            for rhs_index in 0..rhs_count {
                for row in 0..n {
                    let value = rhs[rhs_index * n + row] * rhs_scale[row];
                    if !complex_is_finite(value) {
                        return Err(SolverError::Overflow);
                    }
                    ws.rhs[(row, rhs_index)] = value;
                }
            }
            let lu_ref = unsafe { sparse_lu::LuRef::new_unchecked(&ws.symbolic, &ws.numeric) };
            match operation {
                ComplexSolveOp::Normal => lu_ref.solve_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.solve_mem),
                ),
                ComplexSolveOp::Transpose => lu_ref.solve_transpose_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.transpose_solve_mem),
                ),
                ComplexSolveOp::Adjoint => lu_ref.solve_transpose_in_place_with_conj(
                    Conj::Yes,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.transpose_solve_mem),
                ),
            }

            solution.resize(required, Complex64::new(0.0, 0.0));
            let solution_scale = match operation {
                ComplexSolveOp::Normal => &ws.col_scale,
                ComplexSolveOp::Transpose | ComplexSolveOp::Adjoint => &ws.row_scale,
            };
            let mut all_accepted = true;
            for rhs_index in 0..rhs_count {
                for col in 0..n {
                    let value = ws.rhs[(col, rhs_index)] * solution_scale[col];
                    if !complex_is_finite(value) {
                        return Err(SolverError::SingularMatrix);
                    }
                    solution[rhs_index * n + col] = value;
                }
                let error = complex_componentwise_backward_error(
                    csc,
                    values,
                    &solution[rhs_index * n..(rhs_index + 1) * n],
                    &rhs[rhs_index * n..(rhs_index + 1) * n],
                    None,
                    &mut ws.residual,
                    &mut ws.denominator,
                    &mut ws.compensation,
                    &mut ws.row_nnz,
                    operation,
                )?;
                all_accepted &= error.accepted();
            }
            all_accepted
        };
        if accepted {
            return Ok(());
        }

        // Rare ill-conditioned RHS: preserve the batched fast path for the
        // common case, then use the fully refined scalar path only where the
        // shared factor needs correction.
        let mut refined = Vec::with_capacity(required);
        let mut one = Vec::with_capacity(n);
        for rhs_index in 0..rhs_count {
            self.solve_operation_into(
                &rhs[rhs_index * n..(rhs_index + 1) * n],
                &mut one,
                operation,
                None,
            )?;
            refined.extend_from_slice(&one);
        }
        *solution = refined;
        Ok(())
    }

    /// Solve `A^T x = b` without rebuilding or refactorizing the matrix.
    pub fn solve_transpose_into(
        &mut self,
        rhs: &[Complex64],
        solution: &mut Vec<Complex64>,
    ) -> Result<(), SolverError> {
        self.solve_operation_into(rhs, solution, ComplexSolveOp::Transpose, None)
    }

    /// Solve `A^H x = b` without rebuilding or refactorizing the matrix.
    pub fn solve_adjoint_into(
        &mut self,
        rhs: &[Complex64],
        solution: &mut Vec<Complex64>,
    ) -> Result<(), SolverError> {
        self.solve_operation_into(rhs, solution, ComplexSolveOp::Adjoint, None)
    }

    fn solve_operation_into(
        &mut self,
        rhs: &[Complex64],
        solution: &mut Vec<Complex64>,
        operation: ComplexSolveOp,
        denominator_floor: Option<&[Value]>,
    ) -> Result<(), SolverError> {
        let n = self.nrows;
        self.check_stamping_error()?;

        if n != rhs.len() || self.ncols != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Complex solve requires a square matrix matching RHS size, got {}x{} with RHS {}",
                n,
                self.ncols,
                rhs.len()
            )));
        }
        if denominator_floor.is_some() && operation != ComplexSolveOp::Normal {
            return Err(SolverError::InvalidCircuit(
                "Complex row denominator floors are supported only for A*x=b solves".to_string(),
            ));
        }

        if self.lu.is_none() {
            let symbolic =
                sparse_lu::factorize_symbolic_lu(self.csc.as_ref().as_ref(), Default::default())
                    .map_err(map_faer_error)?;
            self.lu = Some(Self::workspace_from_symbolic(n, Arc::new(symbolic))?);
            self.factorization_valid = false;
        }

        let par = get_global_parallelism();
        let Self {
            csc,
            values,
            lu,
            factorization_valid,
            ..
        } = self;
        let Some(ws) = lu.as_mut() else {
            return Err(SolverError::SingularMatrix);
        };

        if !*factorization_valid {
            equilibrate_complex_matrix(
                csc,
                values,
                &mut ws.scaled_values,
                &mut ws.row_scale,
                &mut ws.col_scale,
            )?;
            let mat = SparseColMatRef::new(csc.as_ref().as_ref(), ws.scaled_values.as_slice());
            ws.symbolic
                .factorize_numeric_lu(
                    &mut ws.numeric,
                    mat,
                    par,
                    MemStack::new(&mut ws.factor_mem),
                    Default::default(),
                )
                .map_err(map_faer_lu_error)?;
            *factorization_valid = true;
        }

        let rhs_scale = match operation {
            ComplexSolveOp::Normal => &ws.row_scale,
            ComplexSolveOp::Transpose | ComplexSolveOp::Adjoint => &ws.col_scale,
        };
        scale_complex_rhs(rhs, rhs_scale, &mut ws.scaled_rhs)?;
        if let Some(floor) = denominator_floor {
            scale_complex_denominator_floor(
                floor,
                &ws.row_scale,
                &mut ws.scaled_denominator_floor,
            )?;
        }
        let scaled_denominator_floor =
            denominator_floor.map(|_| ws.scaled_denominator_floor.as_slice());

        // SAFETY: `ws.numeric` was produced by `ws.symbolic.factorize_numeric_lu`
        // on this matrix's pattern, and `factorization_valid` guarantees the
        // values have not been mutated since (every mutator clears the flag).
        let lu_ref = unsafe { sparse_lu::LuRef::new_unchecked(&ws.symbolic, &ws.numeric) };

        ws.rhs.col_as_slice_mut(0).copy_from_slice(&ws.scaled_rhs);
        match operation {
            ComplexSolveOp::Normal => lu_ref.solve_in_place_with_conj(
                Conj::No,
                ws.rhs.as_mut(),
                par,
                MemStack::new(&mut ws.solve_mem),
            ),
            ComplexSolveOp::Transpose => lu_ref.solve_transpose_in_place_with_conj(
                Conj::No,
                ws.rhs.as_mut(),
                par,
                MemStack::new(&mut ws.transpose_solve_mem),
            ),
            ComplexSolveOp::Adjoint => lu_ref.solve_transpose_in_place_with_conj(
                Conj::Yes,
                ws.rhs.as_mut(),
                par,
                MemStack::new(&mut ws.transpose_solve_mem),
            ),
        }

        solution.clear();
        solution.extend_from_slice(ws.rhs.col_as_slice(0));
        if solution
            .iter()
            .copied()
            .any(|value| !complex_is_finite(value))
        {
            return Err(SolverError::SingularMatrix);
        }

        let mut backward_error = complex_componentwise_backward_error(
            csc,
            &ws.scaled_values,
            solution,
            &ws.scaled_rhs,
            scaled_denominator_floor,
            &mut ws.residual,
            &mut ws.denominator,
            &mut ws.compensation,
            &mut ws.row_nnz,
            operation,
        )?;

        const MAX_COMPLEX_REFINEMENTS: usize = 5;
        const MIN_IMPROVEMENT_FACTOR: Value = 0.5;
        for _ in 0..MAX_COMPLEX_REFINEMENTS {
            if backward_error.accepted() {
                let solution_scale = match operation {
                    ComplexSolveOp::Normal => &ws.col_scale,
                    ComplexSolveOp::Transpose | ComplexSolveOp::Adjoint => &ws.row_scale,
                };
                for (value, &scale) in solution.iter_mut().zip(solution_scale) {
                    *value *= scale;
                    if !complex_is_finite(*value) {
                        return Err(SolverError::Overflow);
                    }
                }
                return Ok(());
            }

            ws.rhs.col_as_slice_mut(0).copy_from_slice(&ws.residual);
            match operation {
                ComplexSolveOp::Normal => lu_ref.solve_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.solve_mem),
                ),
                ComplexSolveOp::Transpose => lu_ref.solve_transpose_in_place_with_conj(
                    Conj::No,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.transpose_solve_mem),
                ),
                ComplexSolveOp::Adjoint => lu_ref.solve_transpose_in_place_with_conj(
                    Conj::Yes,
                    ws.rhs.as_mut(),
                    par,
                    MemStack::new(&mut ws.transpose_solve_mem),
                ),
            }
            for (value, &correction) in solution.iter_mut().zip(ws.rhs.col_as_slice(0)) {
                let refined = *value + correction;
                if !complex_is_finite(correction) || !complex_is_finite(refined) {
                    return Err(SolverError::Overflow);
                }
                *value = refined;
            }
            let refined_error = complex_componentwise_backward_error(
                csc,
                &ws.scaled_values,
                solution,
                &ws.scaled_rhs,
                scaled_denominator_floor,
                &mut ws.residual,
                &mut ws.denominator,
                &mut ws.compensation,
                &mut ws.row_nnz,
                operation,
            )?;
            if refined_error.accepted() {
                let solution_scale = match operation {
                    ComplexSolveOp::Normal => &ws.col_scale,
                    ComplexSolveOp::Transpose | ComplexSolveOp::Adjoint => &ws.row_scale,
                };
                for (value, &scale) in solution.iter_mut().zip(solution_scale) {
                    *value *= scale;
                    if !complex_is_finite(*value) {
                        return Err(SolverError::Overflow);
                    }
                }
                return Ok(());
            }
            if refined_error.acceptance_ratio
                >= backward_error.acceptance_ratio * MIN_IMPROVEMENT_FACTOR
            {
                backward_error = refined_error;
                break;
            }
            backward_error = refined_error;
        }

        Err(SolverError::InaccurateSolution(
            backward_error.componentwise,
        ))
    }
}

//=============================================================================
// Legacy TripletMatrix (for initial structure building)
//=============================================================================

/// Sparse matrix in triplet (COO) format for accumulating stamps
#[derive(Debug, Clone)]
pub struct TripletMatrix {
    nrows: usize,
    ncols: usize,
    row_indices: Vec<usize>,
    col_indices: Vec<usize>,
    values: Vec<Value>,
}

impl TripletMatrix {
    /// Create a new triplet matrix
    pub fn new(size: usize) -> Self {
        Self {
            nrows: size,
            ncols: size,
            row_indices: Vec::with_capacity(size * 6),
            col_indices: Vec::with_capacity(size * 6),
            values: Vec::with_capacity(size * 6),
        }
    }

    /// Add a triplet (row, col, value)
    #[inline]
    pub fn push(&mut self, row: usize, col: usize, value: Value) {
        self.row_indices.push(row);
        self.col_indices.push(col);
        self.values.push(value);
    }

    /// Add a checked triplet, returning immediately when its coordinates are
    /// outside the matrix. [`Self::push`] retains deferred validation for the
    /// simulator's topology-building hot path.
    #[inline]
    pub fn try_push(&mut self, row: usize, col: usize, value: Value) -> Result<(), SolverError> {
        if row >= self.nrows || col >= self.ncols {
            return Err(SolverError::InvalidCircuit(format!(
                "Triplet index ({row}, {col}) is outside {}x{} matrix",
                self.nrows, self.ncols
            )));
        }
        if !value.is_finite() {
            return Err(SolverError::Overflow);
        }
        self.push(row, col, value);
        Ok(())
    }

    /// Matrix dimensions.
    #[inline]
    pub const fn shape(&self) -> (usize, usize) {
        (self.nrows, self.ncols)
    }

    /// Matrix row count.
    #[inline]
    pub const fn nrows(&self) -> usize {
        self.nrows
    }

    /// Matrix column count.
    #[inline]
    pub const fn ncols(&self) -> usize {
        self.ncols
    }

    /// Iterate over stored coordinates and values in insertion order.
    pub fn entries(&self) -> impl ExactSizeIterator<Item = (usize, usize, Value)> + '_ {
        debug_assert_eq!(self.row_indices.len(), self.col_indices.len());
        debug_assert_eq!(self.row_indices.len(), self.values.len());
        self.row_indices
            .iter()
            .copied()
            .zip(self.col_indices.iter().copied())
            .zip(self.values.iter().copied())
            .map(|((row, col), value)| (row, col, value))
    }

    /// Clear all entries for reuse
    pub fn clear(&mut self) {
        self.row_indices.clear();
        self.col_indices.clear();
        self.values.clear();
    }

    /// Number of entries
    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    /// Convert to StaticMatrix (freezes structure)
    pub fn to_static(&self) -> Result<StaticMatrix, SolverError> {
        let triplets: Vec<_> = self.entries().collect();

        StaticMatrix::from_triplets(self.nrows, self.ncols, &triplets)
    }

    /// Convert to faer sparse column matrix (legacy path)
    pub fn to_sparse_col_mat(&self) -> Result<SparseColMat<usize, Value>, SolverError> {
        self.to_static().map(|s| s.to_sparse_col_mat())
    }
}

//=============================================================================
// Sparse LU Solver (high-level API)
//=============================================================================

/// High-performance sparse LU solver using faer.
///
/// The frozen pattern, symbolic analysis, numeric storage, scaling vectors,
/// and solve scratch are retained across calls. A different input pattern is
/// detected by exact CSC comparison and replaces the cache; stale symbolic
/// data can therefore never be applied to a same-sized but different matrix.
pub struct SparseLuSolver {
    cached_matrix: Option<StaticMatrix>,
}

impl SparseLuSolver {
    /// Create a sparse LU facade with no cached symbolic pattern.
    pub fn new() -> Self {
        Self {
            cached_matrix: None,
        }
    }

    /// Solve Ax = b using sparse LU decomposition
    pub fn solve(
        &mut self,
        matrix: &SparseColMat<usize, Value>,
        rhs: &[Value],
    ) -> Result<Vec<Value>, SolverError> {
        let n = matrix.nrows();
        let ncols = matrix.ncols();

        if n == 0 {
            return if ncols == 0 && rhs.is_empty() {
                Ok(Vec::new())
            } else {
                Err(SolverError::InvalidCircuit(format!(
                    "Sparse LU requires a square matrix, got {n}x{ncols}"
                )))
            };
        }

        if n != ncols || n != rhs.len() {
            return Err(SolverError::InvalidCircuit(format!(
                "Sparse LU requires a square matrix matching RHS size, got {}x{} with RHS {}",
                n,
                ncols,
                rhs.len()
            )));
        }
        if rhs
            .iter()
            .chain(matrix.val())
            .any(|value| !value.is_finite())
        {
            return Err(SolverError::Overflow);
        }

        let symbolic = matrix.symbolic();
        let pattern_matches = self.cached_matrix.as_ref().is_some_and(|cached| {
            cached.nrows == n
                && cached.ncols == ncols
                && cached.csc.col_ptr() == symbolic.col_ptr()
                && cached.csc.row_idx() == symbolic.row_idx()
        });
        if !pattern_matches {
            let mut triplets = Vec::new();
            triplets
                .try_reserve_exact(matrix.compute_nnz())
                .map_err(|_| SolverError::OutOfMemory)?;
            for col in 0..ncols {
                for index in symbolic.col_ptr()[col]..symbolic.col_ptr()[col + 1] {
                    triplets.push((symbolic.row_idx()[index], col, matrix.val()[index]));
                }
            }
            self.cached_matrix = Some(StaticMatrix::from_triplets_with_options(
                n,
                ncols,
                &triplets,
                SolverOptions {
                    real_backend: RealSolverBackend::Faer,
                    ..SolverOptions::default()
                },
            )?);
        } else if let Some(cached) = self.cached_matrix.as_mut() {
            cached.values.copy_from_slice(matrix.val());
        }

        self.cached_matrix
            .as_mut()
            .ok_or_else(|| {
                SolverError::InvalidCircuit("sparse LU cache was not built".to_string())
            })?
            .solve(rhs)
    }

    /// Clear cached symbolic factorization (call when matrix structure changes)
    pub fn clear_cache(&mut self) {
        self.cached_matrix = None;
    }
}

impl Default for SparseLuSolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Solve a sparse system Ax = b (convenience function)
pub fn solve_sparse(triplets: &TripletMatrix, rhs: &[Value]) -> Result<Vec<Value>, SolverError> {
    let sparse_mat = triplets.to_sparse_col_mat()?;
    let mut solver = SparseLuSolver::new();
    solver.solve(&sparse_mat, rhs)
}

/// Simple Gaussian elimination for small systems or fallback
pub(crate) fn solve_gauss(
    mut a: Vec<Vec<Value>>,
    mut b: Vec<Value>,
) -> Result<Vec<Value>, SolverError> {
    let n = b.len();

    if n == 0 {
        return Ok(Vec::new());
    }
    if a.len() != n || a.iter().any(|row| row.len() != n) {
        return Err(SolverError::InvalidCircuit(format!(
            "Dense solve requires an {n}x{n} coefficient matrix"
        )));
    }
    if b.iter().any(|value| !value.is_finite())
        || a.iter().flatten().any(|value| !value.is_finite())
    {
        return Err(SolverError::Overflow);
    }

    // Forward elimination with partial pivoting
    for k in 0..n {
        // Find pivot
        let mut max_row = k;
        let mut max_val = a[k][k].abs();

        for i in (k + 1)..n {
            if a[i][k].abs() > max_val {
                max_val = a[i][k].abs();
                max_row = i;
            }
        }

        if max_val == 0.0 || !max_val.is_finite() {
            return Err(SolverError::SingularMatrix);
        }

        // Swap rows
        if max_row != k {
            a.swap(k, max_row);
            b.swap(k, max_row);
        }

        // Eliminate column
        for i in (k + 1)..n {
            let factor = a[i][k] / a[k][k];
            if !factor.is_finite() {
                return Err(SolverError::Overflow);
            }
            for j in k..n {
                a[i][j] -= factor * a[k][j];
                if !a[i][j].is_finite() {
                    return Err(SolverError::Overflow);
                }
            }
            b[i] -= factor * b[k];
            if !b[i].is_finite() {
                return Err(SolverError::Overflow);
            }
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = b[i];
        for j in (i + 1)..n {
            sum -= a[i][j] * x[j];
        }
        x[i] = sum / a[i][i];
    }

    finite_solution_or_singular(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    /// The triplet entries, the right-hand side, and the expected solution.
    type ScaledTridiagonalSystem = (Vec<(usize, usize, Value)>, Vec<Value>, Vec<Value>);

    fn scaled_tridiagonal_system(
        row_factors: [Value; 5],
        col_factors: [Value; 5],
    ) -> ScaledTridiagonalSystem {
        let normalized = [
            [4.0, -1.0, 0.0, 0.0, 0.0],
            [-1.0, 4.0, -1.0, 0.0, 0.0],
            [0.0, -1.0, 4.0, -1.0, 0.0],
            [0.0, 0.0, -1.0, 4.0, -1.0],
            [0.0, 0.0, 0.0, -1.0, 4.0],
        ];
        let solution = col_factors.map(|factor| 1.0 / factor);
        let mut triplets = Vec::new();
        let mut rhs = vec![0.0; 5];
        for row in 0..5 {
            for col in 0..5 {
                if normalized[row][col] == 0.0 {
                    continue;
                }
                let value = row_factors[row] * normalized[row][col] * col_factors[col];
                triplets.push((row, col, value));
                rhs[row] += value * solution[col];
            }
        }
        (triplets, rhs, solution.to_vec())
    }

    fn assert_relative_solution(actual: &[Value], expected: &[Value]) {
        assert_eq!(actual.len(), expected.len());
        for (index, (&actual, &expected)) in actual.iter().zip(expected).enumerate() {
            let relative_error =
                (actual - expected).abs() / expected.abs().max(Value::MIN_POSITIVE);
            assert!(
                relative_error <= 1.0e-10,
                "solution[{index}] expected {expected:.17e}, got {actual:.17e} (relative error {relative_error:.3e})"
            );
        }
    }

    #[test]
    fn faer_equilibration_scales_both_axes_and_round_trips() {
        let (triplets, rhs, _) = scaled_tridiagonal_system(
            [1.0e-20, 1.0e-10, 1.0, 1.0e10, 1.0e20],
            [1.0e20, 1.0e10, 1.0, 1.0e-10, 1.0e-20],
        );
        let matrix = StaticMatrix::from_triplets(5, 5, &triplets).unwrap();
        let mut scaled_values = Vec::new();
        let mut row_scale = Vec::new();
        let mut col_scale = Vec::new();

        equilibrate_sparse_system(
            &matrix.csc,
            &matrix.values,
            &mut scaled_values,
            &mut row_scale,
            &mut col_scale,
        )
        .unwrap();

        let col_ptr = matrix.csc.col_ptr();
        let row_idx = matrix.csc.row_idx();
        for col in 0..matrix.ncols {
            for idx in col_ptr[col]..col_ptr[col + 1] {
                let expected = matrix.values[idx] * row_scale[row_idx[idx]] * col_scale[col];
                assert_eq!(scaled_values[idx].to_bits(), expected.to_bits());
            }
        }
        assert!(
            scaled_values
                .iter()
                .all(|value| value.is_finite() && value.abs() <= 1.0)
        );
        assert!(
            rhs.iter()
                .zip(&row_scale)
                .all(|(&value, &scale)| (value * scale).is_finite())
        );
    }

    #[test]
    fn componentwise_backward_error_detects_perturbed_original_solution() {
        let matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)],
        )
        .unwrap();
        let rhs = [6.0, 8.0];
        let mut residual = Vec::new();
        let mut denominator = Vec::new();
        let mut compensation = Vec::new();
        let mut row_nnz = Vec::new();

        let exact_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &[1.0, 2.0],
            &rhs,
            &mut residual,
            &mut denominator,
            &mut compensation,
            &mut row_nnz,
            RealSolveOp::Normal,
        )
        .unwrap();
        assert_eq!(exact_error.componentwise.to_bits(), 0.0_f64.to_bits());
        assert_eq!(residual, vec![0.0, 0.0]);

        let perturbed_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &[1.0 + 1.0e-6, 2.0],
            &rhs,
            &mut residual,
            &mut denominator,
            &mut compensation,
            &mut row_nnz,
            RealSolveOp::Normal,
        )
        .unwrap();
        assert!(!perturbed_error.accepted());
        let expected_residual = residual.clone();
        let expected_denominator = denominator.clone();
        let expected_compensation = compensation.clone();
        let expected_row_nnz = row_nnz.clone();
        let mut row_residual = Vec::new();
        let mut row_denominator = Vec::new();
        let mut row_compensation = Vec::new();
        let mut row_nnz_layout = Vec::new();
        let row_error = componentwise_backward_error_with_layout(
            &matrix.csc,
            &matrix.residual_layout,
            &matrix.values,
            &[1.0 + 1.0e-6, 2.0],
            &rhs,
            &mut row_residual,
            &mut row_denominator,
            &mut row_compensation,
            &mut row_nnz_layout,
            RealSolveOp::Normal,
        )
        .unwrap();
        assert_eq!(
            row_error.componentwise.to_bits(),
            perturbed_error.componentwise.to_bits()
        );
        assert_eq!(
            row_error.acceptance_ratio.to_bits(),
            perturbed_error.acceptance_ratio.to_bits()
        );
        assert_eq!(row_residual, expected_residual);
        assert_eq!(row_denominator, expected_denominator);
        assert_eq!(row_compensation, expected_compensation);
        assert_eq!(row_nnz_layout, expected_row_nnz);

        // A one-ulp subnormal tail is below the arithmetic noise floor. It
        // must not turn into an order-one backward error merely because both
        // the exact row signal and its residual are below MIN_POSITIVE.
        let subnormal_error = componentwise_backward_error(
            &StaticMatrix::from_triplets(1, 1, &[(0, 0, 1.0)])
                .unwrap()
                .csc,
            &[1.0],
            &[Value::from_bits(1)],
            &[0.0],
            &mut residual,
            &mut denominator,
            &mut compensation,
            &mut row_nnz,
            RealSolveOp::Normal,
        )
        .unwrap();
        assert!(subnormal_error.accepted());

        let zero_row = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let zero_error = componentwise_backward_error(
            &zero_row.csc,
            &zero_row.values,
            &[0.0],
            &[0.0],
            &mut residual,
            &mut denominator,
            &mut compensation,
            &mut row_nnz,
            RealSolveOp::Normal,
        )
        .unwrap();
        assert_eq!(zero_error.componentwise.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn correction_rhs_solves_for_an_update_around_the_iterate() {
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)],
        )
        .unwrap();
        let rhs = [6.0, 8.0];
        let iterate = [0.75, 2.25];

        let correction_rhs = matrix.correction_rhs(&rhs, &iterate).unwrap();
        assert_eq!(correction_rhs, vec![0.75, -0.25]);
        let delta = matrix.solve(&correction_rhs).unwrap();
        let corrected = iterate
            .iter()
            .zip(delta)
            .map(|(value, update)| value + update)
            .collect::<Vec<_>>();

        assert!((corrected[0] - 1.0).abs() <= 8.0 * Value::EPSILON);
        assert!((corrected[1] - 2.0).abs() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn correction_rhs_retains_a_small_kcl_residual_between_large_terms() {
        let matrix =
            StaticMatrix::from_triplets(1, 3, &[(0, 0, -1.0e16), (0, 1, 1.0), (0, 2, 1.0e16)])
                .unwrap();

        let correction = matrix.correction_rhs(&[0.0], &[1.0, 1.0, 1.0]).unwrap();

        assert_eq!(correction, vec![-1.0]);
    }

    #[test]
    fn correction_rhs_into_reuses_both_double_double_buffers() {
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)],
        )
        .unwrap();
        let mut correction = Vec::new();

        matrix
            .correction_rhs_into(&[6.0, 8.0], &[0.75, 2.25], &mut correction)
            .unwrap();
        assert_eq!(correction, vec![0.75, -0.25]);
        let output_capacity = correction.capacity();
        let low_capacity = matrix.correction_rhs_lo_scratch.capacity();

        matrix
            .correction_rhs_into(&[12.0, 16.0], &[1.5, 4.5], &mut correction)
            .unwrap();
        assert_eq!(correction, vec![1.5, -0.5]);
        assert_eq!(correction.capacity(), output_capacity);
        assert_eq!(matrix.correction_rhs_lo_scratch.capacity(), low_capacity);
    }

    #[test]
    fn raw_residual_norms_are_exact_and_allocation_free() {
        let matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 2.0), (0, 1, -1.0), (1, 0, 1.0), (1, 1, 3.0)],
        )
        .unwrap();

        assert_eq!(
            matrix
                .raw_residual_inf_norm(&[2.0, 1.0], &[3.0, 5.0])
                .unwrap(),
            0.0
        );
        assert_eq!(
            matrix
                .raw_residual_inf_norm(&[2.0, 1.0], &[2.75, 5.5])
                .unwrap(),
            0.5
        );
        let (inf_norm, l2_norm) = matrix
            .raw_residual_norms(&[2.0, 1.0], &[2.75, 5.5])
            .unwrap();
        assert_eq!(inf_norm.to_bits(), 0.5_f64.to_bits());
        assert_eq!(l2_norm.to_bits(), 0.3125_f64.sqrt().to_bits());
        assert!(
            matrix
                .raw_residual_inf_norm(&[Value::NAN, 1.0], &[3.0, 5.0])
                .unwrap()
                .is_infinite()
        );
    }

    #[test]
    fn row_layout_scaled_residual_matches_csc_scatter_bit_exactly() {
        let matrix = StaticMatrix::from_triplets(
            3,
            3,
            &[
                (0, 0, 1.0e8),
                (0, 1, -1.0e8),
                (0, 2, 3.0),
                (1, 0, -2.0),
                (1, 1, 5.0),
                (2, 1, -7.0),
                (2, 2, 11.0),
            ],
        )
        .unwrap();
        let solution = [1.0 + Value::EPSILON, 1.0, 2.0];
        let rhs = [6.0, 3.0, 15.0];
        let abstols = [1.0e-12, 1.0e-9, 1.0e-15];
        let reltol = 1.0e-6;

        let mut ax = vec![0.0; matrix.nrows];
        let mut gross = vec![0.0; matrix.nrows];
        for col in 0..matrix.ncols {
            let x = solution[col];
            for index in matrix.csc.col_ptr()[col]..matrix.csc.col_ptr()[col + 1] {
                let row = matrix.csc.row_idx()[index];
                let term = matrix.values[index] * x;
                ax[row] += term;
                gross[row] += term.abs();
            }
        }
        let mut expected = 0.0_f64;
        for row in 0..matrix.nrows {
            let residual = (ax[row] - rhs[row]).abs();
            let noise_floor = 256.0 * Value::EPSILON * gross[row];
            let scale = abstols[row] + noise_floor + reltol * ax[row].abs().max(rhs[row].abs());
            expected = expected.max(residual / scale.max(abstols[row]));
        }

        let actual = matrix
            .scaled_residual_inf_norm_by_row(&solution, &rhs, reltol, |row| abstols[row])
            .unwrap();
        assert_eq!(actual.to_bits(), expected.to_bits());
    }

    #[test]
    fn matrix_vector_product_uses_external_values_without_mutating_matrix() {
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 2.0), (0, 1, -1.0), (1, 0, 4.0), (1, 1, 5.0)],
        )
        .unwrap();
        let snapshot = matrix.values_mut().to_vec();
        matrix.clear_values();

        let mut product = Vec::with_capacity(2);
        matrix
            .matrix_vector_product_with_values_into(&snapshot, &[2.0, 3.0], &mut product)
            .unwrap();
        assert_eq!(product, vec![1.0, 23.0]);
        assert!(matrix.values_mut().iter().all(|value| *value == 0.0));

        let capacity = product.capacity();
        matrix
            .matrix_vector_product_with_values_into(&snapshot, &[1.0, -1.0], &mut product)
            .unwrap();
        assert_eq!(product, vec![3.0, -1.0]);
        assert_eq!(product.capacity(), capacity);
        assert!(matches!(
            matrix.matrix_vector_product_with_values_into(
                &snapshot[..3],
                &[1.0, 1.0],
                &mut product
            ),
            Err(SolverError::InvalidCircuit(_))
        ));
    }

    #[test]
    fn raw_residual_l2_norm_is_stable_across_extreme_magnitudes() {
        let matrix = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();

        let (inf_norm, l2_norm) = matrix
            .raw_residual_norms(&[1.0e300, 1.0e-300], &[0.0, 0.0])
            .unwrap();

        assert_eq!(inf_norm.to_bits(), 1.0e300_f64.to_bits());
        assert!(l2_norm.is_finite());
        assert_eq!(l2_norm.to_bits(), 1.0e300_f64.to_bits());
    }

    #[test]
    fn faer_equilibration_solves_ill_scaled_system_and_recomputes_scales() {
        let row_factors = [1.0e-20, 1.0e-10, 1.0, 1.0e10, 1.0e20];
        let col_factors = [1.0e20, 1.0e10, 1.0, 1.0e-10, 1.0e-20];
        let (triplets, rhs, expected) = scaled_tridiagonal_system(row_factors, col_factors);
        let mut matrix = StaticMatrix::from_triplets(5, 5, &triplets).unwrap();

        let first = matrix.solve_faer(&rhs).unwrap();
        let repeated = matrix.solve_faer(&rhs).unwrap();
        assert_relative_solution(&first, &expected);
        let mut residual = Vec::new();
        let mut denominator = Vec::new();
        let mut compensation = Vec::new();
        let mut row_nnz = Vec::new();
        let backward_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &first,
            &rhs,
            &mut residual,
            &mut denominator,
            &mut compensation,
            &mut row_nnz,
            RealSolveOp::Normal,
        )
        .unwrap();
        assert!(backward_error.accepted());
        assert_eq!(
            first
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            repeated
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );

        let (changed_triplets, changed_rhs, changed_expected) =
            scaled_tridiagonal_system(col_factors, row_factors);
        matrix.clear_values();
        for (row, col, value) in changed_triplets {
            matrix.add(row, col, value);
        }
        let changed = matrix.solve_faer(&changed_rhs).unwrap();
        assert_relative_solution(&changed, &changed_expected);
    }

    #[test]
    fn klu_accepts_only_backward_stable_ill_scaled_solutions() {
        let (triplets, rhs, expected) = scaled_tridiagonal_system(
            [1.0e-20, 1.0e-10, 1.0, 1.0e10, 1.0e20],
            [1.0e20, 1.0e10, 1.0, 1.0e-10, 1.0e-20],
        );
        let mut matrix = StaticMatrix::from_triplets(5, 5, &triplets).unwrap();
        let mut solution = Vec::new();
        assert!(
            matrix.try_solve_klu_into(&rhs, &mut solution),
            "KLU refinement must recover this finite ill-scaled system"
        );
        assert_relative_solution(&solution, &expected);

        let backward_error = componentwise_backward_error(
            &matrix.csc,
            &matrix.values,
            &solution,
            &rhs,
            &mut Vec::new(),
            &mut Vec::new(),
            &mut Vec::new(),
            &mut Vec::new(),
            RealSolveOp::Normal,
        )
        .unwrap();
        assert!(backward_error.accepted());
    }

    fn amesos_klu_options() -> SolverOptions {
        SolverOptions {
            real_backend: RealSolverBackend::Klu,
            numeric_factorization: NumericFactorizationPolicy::FreshPivotSelection,
            factorization_division: DivisionPolicy::DirectDivision,
            diagonal_solve: DivisionPolicy::DirectDivision,
            circuit_lu_orientation: CircuitLuOrientation::AmesosRowCrs,
            circuit_lu_row_scaling: CircuitLuRowScaling::Disabled,
            circuit_lu_robustness: CircuitLuRobustness::BackendFaithful,
            ..SolverOptions::default()
        }
    }

    #[test]
    fn amesos_row_crs_orientation_matches_transposed_factor_and_inverted_solves() {
        // Nonsymmetric in both structure and values. Amesos_Klu presents
        // Epetra's row CRS arrays to KLU as CSC, so the numeric object is for
        // A^T and an ordinary A solve uses KLU's transpose solve.
        let triplets = [
            (0, 0, 3.0),
            (1, 0, 5.0),
            (0, 1, 1.0),
            (1, 1, 2.0),
            (1, 2, 7.0),
            (2, 2, 11.0),
        ];
        let rhs = [13.0, 17.0, 19.0];
        let mut matrix =
            StaticMatrix::from_triplets_with_options(3, 3, &triplets, amesos_klu_options())
                .expect("build Amesos-orientation probe");

        let transpose_col_ptr = matrix.residual_layout.row_ptr.clone();
        let transpose_row_idx = matrix.residual_layout.col_idx.clone();
        let transpose_values = matrix
            .residual_layout
            .csc_idx
            .iter()
            .map(|&source| matrix.values[source])
            .collect::<Vec<_>>();
        let mut reference = crate::KluSolver::new();
        reference.set_direct_factorization_division(true);
        reference.set_direct_diagonal_division(true);
        reference.set_row_scaling_enabled(false);
        reference.set_growth_retry_enabled(false);
        reference
            .analyze(3, &transpose_col_ptr, &transpose_row_idx)
            .expect("analyze explicit A^T reference");
        reference
            .factor(&transpose_values)
            .expect("factor explicit A^T reference");

        let mut expected_normal = Vec::new();
        reference
            .solve_transpose(&rhs, &mut expected_normal)
            .expect("reference A solve through factors of A^T");
        let mut actual_normal = Vec::new();
        matrix
            .solve_into(&rhs, &mut actual_normal)
            .expect("Amesos-oriented normal solve");
        assert_eq!(
            actual_normal
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_normal
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );

        let mut expected_transpose = Vec::new();
        reference
            .solve(&rhs, &mut expected_transpose)
            .expect("reference transpose solve through factors of A^T");
        let mut actual_transpose = Vec::new();
        matrix
            .solve_transpose_into_with_factorization(
                &rhs,
                &mut actual_transpose,
                FactorizationRequest::ReuseExisting,
            )
            .expect("Amesos-oriented transpose solve");
        assert_eq!(
            actual_transpose
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected_transpose
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );

        let batch_rhs = [13.0, 17.0, 19.0, 5.0, 23.0, 29.0];
        let mut expected_batch = Vec::new();
        reference
            .solve_many_transpose(&batch_rhs, 2, &mut expected_batch)
            .expect("reference batched A solve through factors of A^T");
        let mut actual_batch = Vec::new();
        matrix
            .solve_many_into_with_factorization(
                &batch_rhs,
                2,
                &mut actual_batch,
                FactorizationRequest::ReuseExisting,
            )
            .expect("Amesos-oriented batched normal solve");
        assert_eq!(
            actual_batch.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            expected_batch
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>()
        );

        let mut native_options = amesos_klu_options();
        native_options.circuit_lu_orientation = CircuitLuOrientation::Native;
        let mut native =
            StaticMatrix::from_triplets_with_options(3, 3, &triplets, native_options).unwrap();
        let mut native_solution = Vec::new();
        native.solve_into(&rhs, &mut native_solution).unwrap();
        assert!(
            native_solution
                .iter()
                .zip(&actual_normal)
                .any(|(native, amesos)| native.to_bits() != amesos.to_bits()),
            "the nonsymmetric probe must distinguish factor(A) from the Amesos factor(A^T) path"
        );
    }

    #[test]
    fn fresh_policy_factors_each_ordinary_call_and_explicit_reuse_does_not() {
        let options = SolverOptions {
            real_backend: RealSolverBackend::Klu,
            numeric_factorization: NumericFactorizationPolicy::FreshPivotSelection,
            circuit_lu_row_scaling: CircuitLuRowScaling::Disabled,
            circuit_lu_robustness: CircuitLuRobustness::BackendFaithful,
            ..SolverOptions::default()
        };
        let mut matrix = StaticMatrix::from_triplets_with_options(
            2,
            2,
            &[(0, 0, 3.0), (1, 0, 5.0), (0, 1, 1.0), (1, 1, 2.0)],
            options,
        )
        .unwrap();
        let rhs = [7.0, 11.0];
        let mut first = Vec::new();
        matrix.solve_into(&rhs, &mut first).unwrap();
        assert_eq!(matrix.klu.as_ref().unwrap().full_factorization_count(), 1);

        let mut second = Vec::new();
        matrix.solve_into(&rhs, &mut second).unwrap();
        assert_eq!(
            matrix.klu.as_ref().unwrap().full_factorization_count(),
            2,
            "FreshPivotSelection must honor an ordinary numeric-solve request even for identical values"
        );
        assert_eq!(
            first.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            second.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );

        let mut reused = Vec::new();
        matrix
            .solve_into_with_factorization(&rhs, &mut reused, FactorizationRequest::ReuseExisting)
            .unwrap();
        assert_eq!(matrix.klu.as_ref().unwrap().full_factorization_count(), 2);
        assert_eq!(
            second.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            reused.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn changing_orientation_invalidates_and_rebuilds_symbolic_analysis() {
        let triplets = [
            (0, 0, 3.0),
            (1, 0, 5.0),
            (0, 1, 1.0),
            (1, 1, 2.0),
            (1, 2, 7.0),
            (2, 2, 11.0),
        ];
        let rhs = [13.0, 17.0, 19.0];
        let mut native_options = amesos_klu_options();
        native_options.circuit_lu_orientation = CircuitLuOrientation::Native;
        let mut switched =
            StaticMatrix::from_triplets_with_options(3, 3, &triplets, native_options).unwrap();
        let mut solution = Vec::new();
        switched.solve_into(&rhs, &mut solution).unwrap();
        assert!(switched.klu.as_ref().unwrap().is_analyzed_for(3));

        switched.set_solver_options(amesos_klu_options());
        assert!(
            !switched.klu.as_ref().unwrap().is_analyzed_for(3),
            "orientation changes must not reuse symbolic analysis for the prior CSC pattern"
        );
        switched.solve_into(&rhs, &mut solution).unwrap();

        let mut direct =
            StaticMatrix::from_triplets_with_options(3, 3, &triplets, amesos_klu_options())
                .unwrap();
        let mut expected = Vec::new();
        direct.solve_into(&rhs, &mut expected).unwrap();
        assert_eq!(
            solution.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            expected.iter().map(|v| v.to_bits()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn backend_faithful_refactor_does_not_escalate_pivot_growth() {
        let mut backend = crate::KluSolver::new();
        backend.set_growth_retry_enabled(false);
        backend.analyze(2, &[0, 2, 4], &[0, 1, 0, 1]).unwrap();
        backend.factor(&[1.0, 1.0, 1.0, 2.0]).unwrap();
        assert_eq!(backend.full_factorization_count(), 1);

        let changed = [1.0e-20, 1.0, 1.0, 1.0e-20];
        assert!(matches!(
            apply_klu_numeric_action(
                &mut backend,
                &changed,
                KluNumericAction::Refactor,
                CircuitLuRobustness::BackendFaithful,
            ),
            Err(SolverError::PivotGrowth)
        ));
        assert_eq!(
            backend.full_factorization_count(),
            1,
            "BackendFaithful must return the refactor failure without a hidden full factor"
        );

        apply_klu_numeric_action(
            &mut backend,
            &changed,
            KluNumericAction::Refactor,
            CircuitLuRobustness::Enhanced,
        )
        .unwrap();
        assert_eq!(backend.full_factorization_count(), 2);
    }

    #[test]
    fn faer_equilibration_rejects_zero_and_nonfinite_numeric_systems() {
        let mut zero_column =
            StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 0.0)]).unwrap();
        assert!(matches!(
            zero_column.solve_faer(&[1.0, 0.0]),
            Err(SolverError::SingularMatrix)
        ));
        let mut zero_row = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 1.0), (1, 1, 1.0)],
        )
        .unwrap();
        assert!(matches!(
            zero_row.solve_faer(&[0.0, 2.0]),
            Err(SolverError::SingularMatrix)
        ));

        let mut nonfinite = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 1.0)]).unwrap();
        nonfinite.add(0, 0, Value::NAN);
        assert!(matches!(
            nonfinite.solve_faer(&[0.0, 1.0]),
            Err(SolverError::Overflow)
        ));
        let mut finite = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();
        assert!(matches!(
            finite.solve_faer(&[Value::INFINITY, 1.0]),
            Err(SolverError::Overflow)
        ));
    }

    #[test]
    fn faer_equilibration_keeps_extreme_finite_scales_finite() {
        let mut matrix =
            StaticMatrix::from_triplets(2, 2, &[(0, 0, Value::MIN_POSITIVE), (1, 1, Value::MAX)])
                .unwrap();

        let solution = matrix
            .solve_faer(&[Value::MIN_POSITIVE, Value::MAX])
            .unwrap();

        assert_relative_solution(&solution, &[1.0, 1.0]);
    }

    #[test]
    fn static_matrix_missing_stamp_returns_solver_error() {
        let mut matrix = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();

        matrix.add(0, 1, 2.0);
        let message = matrix.solve(&[1.0, 1.0]).unwrap_err().to_string();

        assert!(
            message.contains("missing matrix position")
                && message.contains("StaticMatrix::add")
                && message.contains("(0, 1)"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn static_matrix_singular_solve_rejects_non_finite_solution() {
        let mut matrix = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .unwrap();

        matrix.add(0, 0, 1.0);
        matrix.add(0, 1, 1.0);
        matrix.add(1, 0, 2.0);
        matrix.add(1, 1, 2.0);

        let err = matrix.solve(&[1.0, 2.0]).unwrap_err();

        assert!(matches!(err, SolverError::SingularMatrix));
    }

    #[test]
    fn complex_matrix_missing_stamp_returns_solver_error() {
        let real = StaticMatrix::from_triplets(2, 2, &[(0, 0, 1.0), (1, 1, 1.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);

        matrix.add_real(0, 1, 2.0);
        let message = matrix
            .solve(&[Complex64::new(1.0, 0.0), Complex64::new(1.0, 0.0)])
            .unwrap_err()
            .to_string();

        assert!(
            message.contains("missing matrix position")
                && message.contains("ComplexMatrix::add_real")
                && message.contains("(0, 1)"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn explicit_real_backend_policy_produces_accepted_solutions() {
        let triplets = [(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)];
        let rhs = [6.0, 8.0];
        for real_backend in [
            RealSolverBackend::Auto,
            RealSolverBackend::Klu,
            RealSolverBackend::Faer,
        ] {
            let mut matrix = StaticMatrix::from_triplets_with_options(
                2,
                2,
                &triplets,
                SolverOptions {
                    real_backend,
                    ..SolverOptions::default()
                },
            )
            .unwrap();
            assert_eq!(matrix.solver_options().real_backend, real_backend);
            assert_relative_solution(&matrix.solve(&rhs).unwrap(), &[1.0, 2.0]);
        }
        assert!(!auto_rejects_circuit_lu_fill(255, 100, 0, 10_000));
        assert!(!auto_rejects_circuit_lu_fill(1_000, 100, 400, 400));
        assert!(auto_rejects_circuit_lu_fill(1_000, 100, 401, 400));
        assert!(!auto_prefers_supernodal_from_pattern(511, 100_000));
        assert!(!auto_prefers_supernodal_from_pattern(1_000, 7_999));
        assert!(auto_prefers_supernodal_from_pattern(1_000, 8_000));
    }

    #[test]
    fn real_transpose_solve_is_accepted_by_both_backends() {
        let triplets = [
            (0, 0, 4.0),
            (0, 1, -1.0),
            (1, 0, 2.0),
            (1, 1, 5.0),
            (1, 2, 0.75),
            (2, 1, -0.5),
            (2, 2, 3.0),
        ];
        let expected = [0.25, -1.0, 2.0];
        let rhs = [
            4.0 * expected[0] + 2.0 * expected[1],
            -expected[0] + 5.0 * expected[1] - 0.5 * expected[2],
            0.75 * expected[1] + 3.0 * expected[2],
        ];
        for real_backend in [
            RealSolverBackend::Auto,
            RealSolverBackend::Klu,
            RealSolverBackend::Faer,
        ] {
            let mut matrix = StaticMatrix::from_triplets_with_options(
                3,
                3,
                &triplets,
                SolverOptions {
                    real_backend,
                    ..SolverOptions::default()
                },
            )
            .unwrap();
            let actual = matrix.solve_transpose(&rhs).unwrap();
            assert_relative_solution(&actual, &expected);
        }
    }

    #[test]
    fn real_batched_solve_matches_scalar_solves_for_every_policy() {
        let triplets = [(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)];
        let rhs = [6.0, 8.0, 12.0, 16.0, -2.0, 1.0];
        for real_backend in [
            RealSolverBackend::Auto,
            RealSolverBackend::Klu,
            RealSolverBackend::Faer,
        ] {
            let mut matrix = StaticMatrix::from_triplets_with_options(
                2,
                2,
                &triplets,
                SolverOptions {
                    real_backend,
                    ..SolverOptions::default()
                },
            )
            .unwrap();
            let mut actual = Vec::new();
            matrix.solve_many_into(&rhs, 3, &mut actual).unwrap();
            for rhs_index in 0..3 {
                let expected = matrix
                    .solve(&rhs[rhs_index * 2..rhs_index * 2 + 2])
                    .unwrap();
                assert_relative_solution(&actual[rhs_index * 2..rhs_index * 2 + 2], &expected);
            }

            matrix
                .solve_many_transpose_into(&rhs, 3, &mut actual)
                .unwrap();
            for rhs_index in 0..3 {
                let expected = matrix
                    .solve_transpose(&rhs[rhs_index * 2..rhs_index * 2 + 2])
                    .unwrap();
                assert_relative_solution(&actual[rhs_index * 2..rhs_index * 2 + 2], &expected);
            }
        }
    }

    #[test]
    fn sparse_lu_cache_is_replaced_for_a_same_size_different_pattern() {
        let first = StaticMatrix::from_triplets(2, 2, &[(0, 0, 2.0), (0, 1, 1.0), (1, 1, 3.0)])
            .unwrap()
            .to_sparse_col_mat();
        let second = StaticMatrix::from_triplets(2, 2, &[(0, 0, 4.0), (1, 0, -1.0), (1, 1, 2.0)])
            .unwrap()
            .to_sparse_col_mat();
        let mut solver = SparseLuSolver::new();
        assert_relative_solution(&solver.solve(&first, &[4.0, 6.0]).unwrap(), &[1.0, 2.0]);
        assert_relative_solution(&solver.solve(&second, &[4.0, 3.0]).unwrap(), &[1.0, 2.0]);

        let rectangular =
            StaticMatrix::from_triplets(2, 3, &[(0, 0, 1.0), (1, 1, 1.0), (0, 2, 1.0)])
                .unwrap()
                .to_sparse_col_mat();
        assert!(matches!(
            solver.solve(&rectangular, &[1.0, 1.0]),
            Err(SolverError::InvalidCircuit(_))
        ));
    }

    #[test]
    fn probe_values_restore_the_live_matrix_during_unwinding() {
        let mut matrix = StaticMatrix::from_triplets(2, 2, &[(0, 0, 2.0), (1, 1, 4.0)]).unwrap();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            matrix.with_probe_values(|probe, rhs| {
                probe.add(0, 0, 99.0);
                rhs[0] = 42.0;
                panic!("intentional probe unwind");
            });
        }));
        assert!(result.is_err());
        assert_relative_solution(&matrix.solve(&[2.0, 8.0]).unwrap(), &[1.0, 2.0]);
    }

    #[test]
    fn complex_matrix_shares_the_immutable_real_pattern() {
        let real = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();
        let complex = ComplexMatrix::from_real_structure(&real);
        assert!(Arc::ptr_eq(&real.csc, &complex.csc));
    }

    #[test]
    fn solve_into_reuses_the_callers_output_allocation() {
        let triplets = [(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)];
        for real_backend in [
            RealSolverBackend::Auto,
            RealSolverBackend::Klu,
            RealSolverBackend::Faer,
        ] {
            let mut matrix = StaticMatrix::from_triplets_with_options(
                2,
                2,
                &triplets,
                SolverOptions {
                    real_backend,
                    ..SolverOptions::default()
                },
            )
            .unwrap();
            let mut solution = Vec::new();
            matrix.solve_into(&[6.0, 8.0], &mut solution).unwrap();
            let allocation = solution.as_ptr();
            let capacity = solution.capacity();
            matrix.solve_into(&[12.0, 16.0], &mut solution).unwrap();
            assert_eq!(solution.as_ptr(), allocation);
            assert_eq!(solution.capacity(), capacity);
            assert_relative_solution(&solution, &[2.0, 4.0]);
        }
    }

    #[test]
    fn unchanged_real_matrix_reuses_numeric_factorization_cache() {
        let triplets = [(0, 0, 4.0), (0, 1, 1.0), (1, 0, 2.0), (1, 1, 3.0)];
        for real_backend in [RealSolverBackend::Klu, RealSolverBackend::Faer] {
            let mut matrix = StaticMatrix::from_triplets_with_options(
                2,
                2,
                &triplets,
                SolverOptions {
                    real_backend,
                    ..SolverOptions::default()
                },
            )
            .unwrap();
            let mut solution = Vec::new();
            matrix.solve_into(&[6.0, 8.0], &mut solution).unwrap();

            let (cache_ptr, cache_capacity) = match real_backend {
                RealSolverBackend::Klu => (
                    matrix.klu_factored_values.as_ptr(),
                    matrix.klu_factored_values.capacity(),
                ),
                RealSolverBackend::Faer => {
                    let cached = &matrix.lu.as_ref().unwrap().factored_values;
                    (cached.as_ptr(), cached.capacity())
                }
                RealSolverBackend::Auto => unreachable!(),
            };

            // Transient stamping clears and reconstructs the numeric values at
            // every Newton pass. Reconstruct the identical system here and
            // verify that only the RHS changes while cache storage is reused.
            matrix.clear_values();
            for &(row, col, value) in &triplets {
                matrix.add(row, col, value);
            }
            matrix.solve_into(&[12.0, 16.0], &mut solution).unwrap();
            assert_relative_solution(&solution, &[2.0, 4.0]);

            let (reused_ptr, reused_capacity) = match real_backend {
                RealSolverBackend::Klu => (
                    matrix.klu_factored_values.as_ptr(),
                    matrix.klu_factored_values.capacity(),
                ),
                RealSolverBackend::Faer => {
                    let cached = &matrix.lu.as_ref().unwrap().factored_values;
                    (cached.as_ptr(), cached.capacity())
                }
                RealSolverBackend::Auto => unreachable!(),
            };
            assert_eq!(reused_ptr, cache_ptr);
            assert_eq!(reused_capacity, cache_capacity);
        }
    }

    #[test]
    fn direct_stamp_tokens_are_bound_to_their_originating_pattern() {
        let first = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();
        let pattern = first.pattern_token();
        let index = first.get_index(0, 0).unwrap();
        let second_index = first.get_index(1, 1).unwrap();

        let mut unrelated = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();
        assert!(unrelated.values_mut_for_pattern(pattern).is_none());
        unrelated.stamp_direct(pattern.bind_offset(index.offset()), 1.0);
        let message = unrelated.solve(&[1.0, 1.0]).unwrap_err().to_string();
        assert!(message.contains("StaticMatrix::stamp_direct"));
        assert!(message.contains("pattern"));

        let mut clone = first.clone_structure();
        assert!(clone.values_mut_for_pattern(pattern).is_some());
        clone.stamp_direct(pattern.bind_offset(index.offset()), 2.0);
        clone.stamp_direct(clone.get_index(1, 1).unwrap(), 4.0);
        assert_relative_solution(&clone.solve(&[2.0, 4.0]).unwrap(), &[1.0, 1.0]);

        let mut complex = ComplexMatrix::from_real_structure(&first);
        complex.stamp_direct_real(index, 2.0);
        complex.stamp_direct_real(second_index, 4.0);
        let solution = complex
            .solve(&[Complex64::new(2.0, 0.0), Complex64::new(4.0, 0.0)])
            .unwrap();
        assert!((solution[0] - Complex64::new(1.0, 0.0)).norm() <= 8.0 * Value::EPSILON);
        assert!((solution[1] - Complex64::new(1.0, 0.0)).norm() <= 8.0 * Value::EPSILON);
    }

    #[test]
    fn complex_solve_equilibrates_refines_and_reuses_factorization() {
        let real = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        let a = [
            [Complex64::new(4.0, 1.0), Complex64::new(-1.0e-40, 0.5e-40)],
            [Complex64::new(-1.0e40, -0.25e40), Complex64::new(3.0, -0.5)],
        ];
        for (row, row_values) in a.iter().enumerate() {
            for (col, &value) in row_values.iter().enumerate() {
                matrix.add(row, col, value);
            }
        }

        let mut actual = Vec::new();
        let mut allocation = None;
        for expected in [
            [
                Complex64::new(1.0e-20, 0.5e-20),
                Complex64::new(2.0e20, -1.0e20),
            ],
            [
                Complex64::new(-2.0e-20, 1.0e-20),
                Complex64::new(0.5e20, 0.75e20),
            ],
        ] {
            let rhs = [
                a[0][0] * expected[0] + a[0][1] * expected[1],
                a[1][0] * expected[0] + a[1][1] * expected[1],
            ];
            matrix.solve_into(&rhs, &mut actual).unwrap();
            matrix
                .certify_solution(&actual, &rhs)
                .expect("the solved complex system remains certified");
            let mut perturbed = actual.clone();
            perturbed[0] += Complex64::new(1.0e-6, 0.0);
            assert!(matches!(
                matrix.certify_solution(&perturbed, &rhs),
                Err(SolverError::InaccurateSolution(_))
            ));
            if let Some(previous) = allocation {
                assert_eq!(actual.as_ptr(), previous);
            } else {
                allocation = Some(actual.as_ptr());
            }
            for (index, (&actual, &expected)) in actual.iter().zip(&expected).enumerate() {
                let relative_error = (actual - expected).norm() / expected.norm();
                assert!(
                    relative_error <= 1.0e-10,
                    "complex solution[{index}] relative error {relative_error:.3e}"
                );
            }
        }
    }

    #[test]
    fn complex_row_floors_rescue_only_physically_negligible_homogeneous_leakage() {
        let real = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        matrix.add_real(0, 0, 1.0);
        matrix.add_real(1, 1, 1.0);
        let rhs = [Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];
        let mut residual = Vec::new();
        let mut denominator = Vec::new();
        let mut compensation = Vec::new();
        let mut row_nnz = Vec::new();
        let evaluate = |solution: &[Complex64],
                        floor: Option<&[Value]>,
                        residual: &mut Vec<Complex64>,
                        denominator: &mut Vec<Value>,
                        compensation: &mut Vec<Complex64>,
                        row_nnz: &mut Vec<usize>| {
            complex_componentwise_backward_error(
                &matrix.csc,
                &matrix.values,
                solution,
                &rhs,
                floor,
                residual,
                denominator,
                compensation,
                row_nnz,
                ComplexSolveOp::Normal,
            )
            .unwrap()
        };

        let negligible = [Complex64::new(1.0, 0.0), Complex64::new(1.0e-51, 0.0)];
        let strict = evaluate(
            &negligible,
            None,
            &mut residual,
            &mut denominator,
            &mut compensation,
            &mut row_nnz,
        );
        assert_eq!(strict.componentwise, 1.0);
        assert!(!strict.accepted());
        let source_floor = [0.0, 1.0];
        assert!(
            evaluate(
                &negligible,
                Some(&source_floor),
                &mut residual,
                &mut denominator,
                &mut compensation,
                &mut row_nnz,
            )
            .accepted()
        );

        let material_source_error = [Complex64::new(1.0, 0.0), Complex64::new(1.0e-8, 0.0)];
        assert!(
            !evaluate(
                &material_source_error,
                Some(&source_floor),
                &mut residual,
                &mut denominator,
                &mut compensation,
                &mut row_nnz,
            )
            .accepted()
        );
        let material_non_source_error = [
            Complex64::new(1.0 + 1.0e-8, 0.0),
            Complex64::new(1.0e-51, 0.0),
        ];
        assert!(
            !evaluate(
                &material_non_source_error,
                Some(&source_floor),
                &mut residual,
                &mut denominator,
                &mut compensation,
                &mut row_nnz,
            )
            .accepted()
        );

        matrix.solve(&rhs).expect("identity system factors");
        assert!(matches!(
            matrix.certify_solution(&negligible, &rhs),
            Err(SolverError::InaccurateSolution(_))
        ));
    }

    #[test]
    fn complex_row_floors_follow_row_equilibration() {
        let mut scaled = Vec::new();
        scale_complex_denominator_floor(&[0.0, 1.0, 4.0], &[2.0, 0.25, 8.0], &mut scaled).unwrap();
        assert_eq!(scaled, [0.0, 0.25, 32.0]);
        assert!(matches!(
            scale_complex_denominator_floor(&[1.0], &[1.0, 1.0], &mut scaled),
            Err(SolverError::InvalidCircuit(_))
        ));
    }

    #[test]
    fn complex_transpose_and_adjoint_solves_reuse_the_factorization() {
        let real = StaticMatrix::from_triplets(
            3,
            3,
            &[
                (0, 0, 0.0),
                (0, 1, 0.0),
                (1, 0, 0.0),
                (1, 1, 0.0),
                (1, 2, 0.0),
                (2, 1, 0.0),
                (2, 2, 0.0),
            ],
        )
        .unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        let a = [
            [
                Complex64::new(4.0, 1.0),
                Complex64::new(-1.0, 0.5),
                Complex64::new(0.0, 0.0),
            ],
            [
                Complex64::new(2.0, -0.25),
                Complex64::new(5.0, -2.0),
                Complex64::new(0.75, 1.0),
            ],
            [
                Complex64::new(0.0, 0.0),
                Complex64::new(-0.5, 0.2),
                Complex64::new(3.0, 0.5),
            ],
        ];
        for row in 0..3 {
            for col in 0..3 {
                if a[row][col] != Complex64::new(0.0, 0.0) {
                    matrix.add(row, col, a[row][col]);
                }
            }
        }
        let expected = [
            Complex64::new(0.25, -0.5),
            Complex64::new(-1.0, 0.75),
            Complex64::new(2.0, 0.125),
        ];
        let mut transpose_rhs = [Complex64::new(0.0, 0.0); 3];
        let mut adjoint_rhs = [Complex64::new(0.0, 0.0); 3];
        for equation in 0..3 {
            for variable in 0..3 {
                transpose_rhs[equation] += a[variable][equation] * expected[variable];
                adjoint_rhs[equation] += a[variable][equation].conj() * expected[variable];
            }
        }

        let mut actual = Vec::new();
        matrix
            .solve_transpose_into(&transpose_rhs, &mut actual)
            .unwrap();
        for (&x, &want) in actual.iter().zip(&expected) {
            assert!((x - want).norm() <= 1.0e-12);
        }
        matrix
            .solve_adjoint_into(&adjoint_rhs, &mut actual)
            .unwrap();
        for (&x, &want) in actual.iter().zip(&expected) {
            assert!((x - want).norm() <= 1.0e-12);
        }
        let mut batch_rhs = Vec::new();
        batch_rhs.extend_from_slice(&transpose_rhs);
        batch_rhs.extend(transpose_rhs.iter().map(|value| *value * 2.0));
        matrix
            .solve_many_transpose_into(&batch_rhs, 2, &mut actual)
            .unwrap();
        for (&x, &want) in actual[..3].iter().zip(&expected) {
            assert!((x - want).norm() <= 1.0e-12);
        }
        for (&x, &want) in actual[3..].iter().zip(&expected) {
            assert!((x - want * 2.0).norm() <= 2.0e-12);
        }
        assert!(matrix.factorization_valid);
    }

    #[test]
    fn complex_batched_solve_matches_reused_scalar_solves() {
        let real = StaticMatrix::from_triplets(
            2,
            2,
            &[(0, 0, 0.0), (0, 1, 0.0), (1, 0, 0.0), (1, 1, 0.0)],
        )
        .unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        matrix.add(0, 0, Complex64::new(4.0, 1.0));
        matrix.add(0, 1, Complex64::new(-1.0, 0.5));
        matrix.add(1, 0, Complex64::new(2.0, -0.25));
        matrix.add(1, 1, Complex64::new(5.0, -2.0));
        let rhs = [
            Complex64::new(1.0, 2.0),
            Complex64::new(-0.5, 0.75),
            Complex64::new(3.0, -1.0),
            Complex64::new(0.25, 2.5),
            Complex64::new(-2.0, 0.5),
            Complex64::new(1.25, -0.75),
        ];
        let mut actual = Vec::new();
        matrix.solve_many_into(&rhs, 3, &mut actual).unwrap();
        for rhs_index in 0..3 {
            let expected = matrix
                .solve(&rhs[rhs_index * 2..rhs_index * 2 + 2])
                .unwrap();
            for (&x, &want) in actual[rhs_index * 2..rhs_index * 2 + 2]
                .iter()
                .zip(&expected)
            {
                assert!((x - want).norm() <= 1.0e-12);
            }
        }
    }

    #[test]
    fn complex_operations_reject_nonfinite_and_rectangular_systems() {
        let real = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        matrix.add(0, 0, Complex64::new(Value::NAN, 0.0));
        assert!(matches!(
            matrix.solve(&[Complex64::new(1.0, 0.0)]),
            Err(SolverError::Overflow)
        ));

        matrix.clear_values();
        matrix.add_real(0, 0, 1.0);
        assert!(matches!(
            matrix.solve(&[Complex64::new(Value::INFINITY, 0.0)]),
            Err(SolverError::Overflow)
        ));
        assert!(matches!(
            matrix.multiply_vector(&[Complex64::new(0.0, Value::NAN)]),
            Err(SolverError::Overflow)
        ));

        let rectangular = StaticMatrix::from_triplets(1, 2, &[(0, 0, 1.0), (0, 1, 1.0)]).unwrap();
        let mut rectangular = ComplexMatrix::from_real_structure(&rectangular);
        assert!(matches!(
            rectangular.solve(&[Complex64::new(1.0, 0.0)]),
            Err(SolverError::InvalidCircuit(_))
        ));

        let mut checked = ComplexMatrix::from_real_structure(&real);
        checked.try_add_real(0, 0, 2.0).unwrap();
        assert!(matches!(
            checked.try_add(0, 0, Complex64::new(Value::NAN, 0.0)),
            Err(SolverError::Overflow)
        ));
        assert_eq!(
            checked.solve(&[Complex64::new(4.0, 0.0)]).unwrap()[0],
            Complex64::new(2.0, 0.0)
        );
    }

    #[test]
    fn checked_triplet_api_preserves_storage_invariants() {
        let mut matrix = TripletMatrix::new(2);
        matrix.try_push(0, 0, 1.0).unwrap();
        matrix.try_push(1, 1, 2.0).unwrap();
        assert!(matches!(
            matrix.try_push(2, 0, 3.0),
            Err(SolverError::InvalidCircuit(_))
        ));
        assert!(matches!(
            matrix.try_push(0, 1, Value::NAN),
            Err(SolverError::Overflow)
        ));
        assert_eq!(matrix.shape(), (2, 2));
        assert_eq!(
            matrix.entries().collect::<Vec<_>>(),
            vec![(0, 0, 1.0), (1, 1, 2.0)]
        );
        assert_relative_solution(&solve_sparse(&matrix, &[1.0, 2.0]).unwrap(), &[1.0, 1.0]);

        assert!(matches!(
            StaticMatrix::from_triplets(1, 1, &[(0, 0, Value::NAN)]),
            Err(SolverError::Overflow)
        ));
        let mut checked = StaticMatrix::from_triplets(1, 1, &[(0, 0, 0.0)]).unwrap();
        checked.try_add(0, 0, 2.0).unwrap();
        assert!(matches!(
            checked.try_add(0, 0, Value::INFINITY),
            Err(SolverError::Overflow)
        ));
        assert_relative_solution(&checked.solve(&[4.0]).unwrap(), &[2.0]);
    }

    #[test]
    fn complex_stored_entry_visitor_is_deterministic_and_keeps_structural_zeros() {
        let real = StaticMatrix::from_triplets(
            3,
            3,
            &[(2, 0, 0.0), (0, 0, 0.0), (1, 1, 0.0), (0, 2, 0.0)],
        )
        .unwrap();
        let mut matrix = ComplexMatrix::from_real_structure(&real);
        matrix.add(2, 0, Complex64::new(2.0, -1.0));
        matrix.add(0, 2, Complex64::new(-3.0, 0.5));

        let mut visited = Vec::new();
        matrix.for_each_stored(|row, column, value| visited.push((row, column, value)));
        assert_eq!(
            visited,
            vec![
                (0, 0, Complex64::new(0.0, 0.0)),
                (2, 0, Complex64::new(2.0, -1.0)),
                (1, 1, Complex64::new(0.0, 0.0)),
                (0, 2, Complex64::new(-3.0, 0.5)),
            ]
        );
    }

    #[test]
    fn dense_fallback_is_scale_invariant_and_validates_shape() {
        let matrix = StaticMatrix::from_triplets(1, 1, &[(0, 0, 1.0e-300)]).unwrap();
        assert_relative_solution(&matrix.solve_dense(&[2.0e-300]).unwrap(), &[2.0]);

        assert!(matches!(
            solve_gauss(vec![vec![1.0, 0.0]], vec![1.0]),
            Err(SolverError::InvalidCircuit(_))
        ));
        assert!(matches!(
            solve_gauss(vec![vec![Value::NAN]], vec![1.0]),
            Err(SolverError::Overflow)
        ));
    }

    #[test]
    fn auto_retries_klu_after_a_numeric_factorization_failure() {
        let mut matrix = StaticMatrix::from_triplets_with_options(
            1,
            1,
            &[(0, 0, 1.0)],
            SolverOptions {
                real_backend: RealSolverBackend::Auto,
                ..SolverOptions::default()
            },
        )
        .unwrap();

        matrix.clear_values();
        matrix.add(0, 0, Value::NAN);
        assert!(matches!(matrix.solve(&[1.0]), Err(SolverError::Overflow)));
        assert!(!matrix.klu_auto_rejected);

        matrix.clear_values();
        matrix.add(0, 0, 2.0);
        assert_relative_solution(&matrix.solve(&[4.0]).unwrap(), &[2.0]);
    }
}

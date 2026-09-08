//! Portable encoding for the accepted sparse-solver state.

use super::*;
use rspice_matrix::{
    CircuitLuOrientation, CircuitLuRobustness, CircuitLuRowScaling, DivisionPolicy, KluDiagnostics,
    KluNumericCheckpoint, NumericFactorizationPolicy, RealSolverBackend, SolverOptions,
};

fn write_indices<T: std::fmt::Display>(
    out: &mut String,
    name: &str,
    values: &[T],
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!("{name} {}\n", values.len()));
    for (index, value) in values.iter().enumerate() {
        poll_checkpoint_abort(abort, index)?;
        out.push_str(&format!("{value}\n"));
    }
    Ok(())
}

fn read_indices<T: std::str::FromStr>(
    lines: &mut CheckpointLines<'_>,
    name: &str,
    budget: &mut CheckpointParseBudget,
) -> Result<Vec<T>, String> {
    let count = parse_count_header(lines.next().ok_or_else(|| format!("missing {name}"))?, name)?;
    let mut values = allocate_checkpoint_rows(lines, count, name, budget)?;
    for row in 0..count {
        let value = lines.next().ok_or_else(|| format!("truncated {name}"))?;
        values.push(
            value
                .parse()
                .map_err(|_| format!("invalid {name} index at row {row}"))?,
        );
    }
    Ok(values)
}

fn fields<'a, const N: usize>(
    lines: &mut CheckpointLines<'a>,
    name: &str,
) -> Result<[&'a str; N], String> {
    let line = lines.next().ok_or_else(|| format!("missing {name}"))?;
    let mut parts = line.split_whitespace();
    if parts.next() != Some(name) {
        return Err(format!("expected {name}, got {line}"));
    }
    let mut result = [""; N];
    for field in &mut result {
        *field = parts.next().ok_or_else(|| format!("truncated {name}"))?;
    }
    if parts.next().is_some() {
        return Err(format!("extra fields in {name}"));
    }
    Ok(result)
}

fn choice<T: Copy>(field: &str, variants: &[T]) -> Result<T, String> {
    field
        .parse::<usize>()
        .ok()
        .and_then(|index| variants.get(index))
        .copied()
        .ok_or_else(|| format!("unknown sparse-solver policy {field}"))
}

pub(super) fn write_solver_state(
    out: &mut String,
    state: Option<&StaticMatrixSolverCheckpoint>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    check_checkpoint_abort(abort)?;
    let Some(state) = state else {
        out.push_str("accepted_solver_state none\n");
        return Ok(());
    };
    out.push_str(&format!(
        "accepted_solver_state {} {} {}\n",
        state.version,
        state.dimension,
        u8::from(state.klu_auto_rejected)
    ));
    let o = state.options;
    out.push_str(&format!(
        "accepted_solver_options {} {} {} {} {} {} {} {} {}\n",
        match o.real_backend {
            RealSolverBackend::Auto => 0,
            RealSolverBackend::Klu => 1,
            RealSolverBackend::Faer => 2,
        },
        u8::from(o.numeric_factorization == NumericFactorizationPolicy::FreshPivotSelection),
        u8::from(o.factorization_division == DivisionPolicy::DirectDivision),
        u8::from(o.diagonal_solve == DivisionPolicy::DirectDivision),
        u8::from(o.circuit_lu_orientation == CircuitLuOrientation::AmesosRowCrs),
        u8::from(o.circuit_lu_row_scaling == CircuitLuRowScaling::Disabled),
        u8::from(o.circuit_lu_robustness == CircuitLuRobustness::BackendFaithful),
        o.pivot_tolerance,
        o.absolute_pivot_tolerance
    ));
    write_indices(out, "accepted_solver_col_ptr", &state.col_ptr, abort)?;
    write_indices(out, "accepted_solver_row_idx", &state.row_idx, abort)?;
    if let Some(klu) = &state.klu {
        out.push_str("accepted_solver_klu present\n");
        write_indices(out, "accepted_solver_klu_row_perm", &klu.row_perm, abort)?;
        write_indices(out, "accepted_solver_klu_l_col_ptr", &klu.l_col_ptr, abort)?;
        write_indices(out, "accepted_solver_klu_l_rows", &klu.l_rows, abort)?;
        write_value_vector(out, "accepted_solver_klu_l_values", &klu.l_values, abort)?;
        write_indices(out, "accepted_solver_klu_u_col_ptr", &klu.u_col_ptr, abort)?;
        write_indices(out, "accepted_solver_klu_u_rows", &klu.u_rows, abort)?;
        write_value_vector(out, "accepted_solver_klu_u_values", &klu.u_values, abort)?;
        write_value_vector(out, "accepted_solver_klu_row_scale", &klu.row_scale, abort)?;
        let d = klu.diagnostics;
        write_value_vector(
            out,
            "accepted_solver_klu_diagnostics",
            &[
                d.reciprocal_pivot_growth,
                d.diagonal_rcond,
                d.min_abs_pivot,
                d.max_abs_pivot,
            ],
            abort,
        )?;
    } else {
        out.push_str("accepted_solver_klu none\n");
    }
    write_value_vector(
        out,
        "accepted_solver_klu_factored_values",
        &state.klu_factored_values,
        abort,
    )?;
    if let Some(values) = &state.faer_factored_values {
        out.push_str("accepted_solver_faer present\n");
        write_value_vector(out, "accepted_solver_faer_values", values, abort)?;
    } else {
        out.push_str("accepted_solver_faer none\n");
    }
    out.push_str("accepted_solver_end\n");
    Ok(())
}

pub(super) fn read_solver_state(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
) -> Result<Option<StaticMatrixSolverCheckpoint>, String> {
    let line = lines.next().ok_or("missing accepted_solver_state")?;
    if line == "accepted_solver_state none" {
        return Ok(None);
    }
    let mut header = line.split_whitespace();
    if header.next() != Some("accepted_solver_state") {
        return Err("missing accepted_solver_state".into());
    }
    let version = header
        .next()
        .ok_or("missing solver state version")?
        .parse()
        .map_err(|_| "invalid solver state version")?;
    let dimension = header
        .next()
        .ok_or("missing solver dimension")?
        .parse()
        .map_err(|_| "invalid solver dimension")?;
    let klu_auto_rejected = parse_checkpoint_bool(
        header.next().ok_or("missing solver routing state")?,
        "solver routing",
    )?;
    if header.next().is_some() {
        return Err("extra solver state fields".into());
    }
    let [
        backend,
        factorization,
        division,
        diagonal,
        orientation,
        scaling,
        robustness,
        pivot,
        absolute,
    ] = fields(lines, "accepted_solver_options")?;
    let options = SolverOptions {
        real_backend: choice(
            backend,
            &[
                RealSolverBackend::Auto,
                RealSolverBackend::Klu,
                RealSolverBackend::Faer,
            ],
        )?,
        numeric_factorization: choice(
            factorization,
            &[
                NumericFactorizationPolicy::ReusePivotSequence,
                NumericFactorizationPolicy::FreshPivotSelection,
            ],
        )?,
        factorization_division: choice(
            division,
            &[
                DivisionPolicy::ReciprocalMultiplication,
                DivisionPolicy::DirectDivision,
            ],
        )?,
        diagonal_solve: choice(
            diagonal,
            &[
                DivisionPolicy::ReciprocalMultiplication,
                DivisionPolicy::DirectDivision,
            ],
        )?,
        circuit_lu_orientation: choice(
            orientation,
            &[
                CircuitLuOrientation::Native,
                CircuitLuOrientation::AmesosRowCrs,
            ],
        )?,
        circuit_lu_row_scaling: choice(
            scaling,
            &[
                CircuitLuRowScaling::AdaptiveExtremeRows,
                CircuitLuRowScaling::Disabled,
            ],
        )?,
        circuit_lu_robustness: choice(
            robustness,
            &[
                CircuitLuRobustness::Enhanced,
                CircuitLuRobustness::BackendFaithful,
            ],
        )?,
        pivot_tolerance: pivot
            .parse()
            .map_err(|_| "invalid solver pivot tolerance")?,
        absolute_pivot_tolerance: absolute
            .parse()
            .map_err(|_| "invalid solver absolute pivot tolerance")?,
    };
    let col_ptr = read_indices(lines, "accepted_solver_col_ptr", budget)?;
    let row_idx = read_indices(lines, "accepted_solver_row_idx", budget)?;
    let klu = match fields::<1>(lines, "accepted_solver_klu")?[0] {
        "none" => None,
        "present" => {
            let row_perm = read_indices(lines, "accepted_solver_klu_row_perm", budget)?;
            let l_col_ptr = read_indices(lines, "accepted_solver_klu_l_col_ptr", budget)?;
            let l_rows = read_indices(lines, "accepted_solver_klu_l_rows", budget)?;
            let l_values = read_value_vector(lines, "accepted_solver_klu_l_values", budget)?;
            let u_col_ptr = read_indices(lines, "accepted_solver_klu_u_col_ptr", budget)?;
            let u_rows = read_indices(lines, "accepted_solver_klu_u_rows", budget)?;
            let u_values = read_value_vector(lines, "accepted_solver_klu_u_values", budget)?;
            let row_scale = read_value_vector(lines, "accepted_solver_klu_row_scale", budget)?;
            let diagnostics = read_value_vector(lines, "accepted_solver_klu_diagnostics", budget)?;
            let [
                reciprocal_pivot_growth,
                diagonal_rcond,
                min_abs_pivot,
                max_abs_pivot,
            ] = diagnostics.as_slice()
            else {
                return Err("solver diagnostics must have four entries".into());
            };
            Some(KluNumericCheckpoint {
                row_perm,
                l_col_ptr,
                l_rows,
                l_values,
                u_col_ptr,
                u_rows,
                u_values,
                row_scale,
                diagnostics: KluDiagnostics {
                    reciprocal_pivot_growth: *reciprocal_pivot_growth,
                    diagonal_rcond: *diagonal_rcond,
                    min_abs_pivot: *min_abs_pivot,
                    max_abs_pivot: *max_abs_pivot,
                },
            })
        }
        _ => return Err("invalid solver KLU state presence".into()),
    };
    let klu_factored_values =
        read_value_vector(lines, "accepted_solver_klu_factored_values", budget)?;
    let faer_factored_values = match fields::<1>(lines, "accepted_solver_faer")?[0] {
        "none" => None,
        "present" => Some(read_value_vector(
            lines,
            "accepted_solver_faer_values",
            budget,
        )?),
        _ => return Err("invalid solver faer state presence".into()),
    };
    fields::<0>(lines, "accepted_solver_end")?;
    // The enclosing checkpoint validator charges its scratch allocations to
    // the aggregate budget before checking the complete matrix image.
    Ok(Some(StaticMatrixSolverCheckpoint {
        version,
        dimension,
        col_ptr,
        row_idx,
        options,
        klu_auto_rejected,
        klu,
        klu_factored_values,
        faer_factored_values,
    }))
}

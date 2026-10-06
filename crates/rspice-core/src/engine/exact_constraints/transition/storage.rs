//! Exact compatibility of authored charge/flux with the constant E matrix.
use super::*;

pub(super) fn constraint(row: ExactRow<Input>) -> Result<ExactRow<()>> {
    let mut nodes = BTreeMap::new();
    for (input, coefficient) in row.values {
        let Input::Storage(index) = input else {
            return Err(failure("inconsistent prescribed forcing in transition"));
        };
        nodes.insert(index + 1, coefficient);
    }
    Ok(ExactRow {
        nodes,
        ..ExactRow::default()
    })
}

/// Eliminate E*x=q symbolically. Every row without an x pivot is an exact
/// homogeneous constraint on q. This is necessary even when a higher-index
/// block would otherwise let an invalid algebraic-row charge create an impulse.
pub(super) fn prepare(
    e: &Terms,
    limits: ResourceLimits,
    retained: usize,
    abort: &dyn AbortSignal,
) -> Result<ExactElimination<()>> {
    let columns = dimension(e.len().checked_add(1))?;
    ExactElimination::<Input>::ensure_words(
        retained.saturating_add(columns.saturating_mul(4)),
        limits.max_result_values,
    )?;
    let mut image = ExactElimination::<Input>::new(columns, limits)?;
    let mut constraints = ExactElimination::<()>::new(columns, limits)?;
    for (index, entries) in e.iter().enumerate() {
        check_abort(abort)?;
        image.limits.max_result_values = limits
            .max_result_values
            .saturating_sub(retained.saturating_add(constraints.retained_words));
        let mut row = ExactRow::default();
        add(&image, &mut row, entries, 0, abort)?;
        image.check_cost(row.words().saturating_mul(3).saturating_add(160))?;
        row.values
            .insert(Input::Storage(index), integer_coefficient(1.0).unwrap());
        if let Some(row) = image.admit(row, 1, abort)? {
            constraints.limits.max_result_values = limits
                .max_result_values
                .saturating_sub(retained.saturating_add(image.retained_words));
            let row = constraint(row)?;
            // These are homogeneous identities, so a reduced row has no RHS.
            constraints.admit(row, 1, abort)?;
        }
    }
    constraints.limits.max_result_values = limits.max_result_values.saturating_sub(retained);
    Ok(constraints)
}

//! Range-preserving projection of MNA adjoints onto a shared stochastic direction.
use super::*;

pub(super) struct Spectrum {
    pub values: Vec<Complex64>,
    pub exponent: i32,
}
pub(super) fn project(
    grid: &QuasiPeriodicGrid,
    selected: &[usize],
    adjoint: &QuasiPeriodicAdjointSolution,
    injections: &[(usize, Complex64)],
    abort: &dyn AbortSignal,
) -> Result<Spectrum, Error> {
    let mut coefficients = vec![ScaledComplex::ZERO; grid.len()];
    for &k in selected {
        check_abort(abort)?;
        let mut common = None;
        for (i, &(row, direction)) in injections.iter().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let term = scaled_complex_product3(
                direction.conj(),
                adjoint.sensitivities[row][k],
                Complex64::ONE,
                0,
            )
            .map_err(numerical)?;
            if !term.is_zero() {
                common = Some(common.map_or(term.exponent, |e: i32| e.max(term.exponent)));
            }
        }
        let Some(common) = common else {
            continue;
        };
        let mut sum = ScaledComplexAccumulator::new(common);
        for (i, &(row, direction)) in injections.iter().enumerate() {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let term = scaled_complex_product3(
                direction.conj(),
                adjoint.sensitivities[row][k],
                Complex64::ONE,
                0,
            )
            .map_err(numerical)?;
            sum.add(term).map_err(numerical)?;
        }
        let (value, _) = sum.normalized_sum().map_err(numerical)?;
        coefficients[k] = scaled_complex_product3(value, Complex64::ONE, Complex64::ONE, common)
            .map_err(numerical)?;
    }
    let exponent = coefficients
        .iter()
        .filter(|c| !c.is_zero())
        .map(|c| c.exponent)
        .max()
        .unwrap_or(0);
    let values = coefficients
        .into_iter()
        .enumerate()
        .map(|(i, c)| {
            if i.is_multiple_of(256) {
                check_abort(abort)?;
            }
            let shift = c
                .exponent
                .checked_sub(exponent)
                .ok_or_else(|| numerical("source gain scale overflows"))?;
            Ok(Complex64::new(
                scale_complex_component_exactly(c.mantissa.re, shift).map_err(numerical)?,
                scale_complex_component_exactly(c.mantissa.im, shift).map_err(numerical)?,
            ))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(Spectrum { values, exponent })
}

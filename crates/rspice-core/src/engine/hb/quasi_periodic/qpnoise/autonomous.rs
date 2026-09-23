//! Keep interpolation-based noise integrals away from free-phase poles.
use super::*;

pub(super) fn validate_integration(
    request: &QpnoiseRequest,
    point: &QpssOperatingPoint,
    grid: &QuasiPeriodicGrid,
    observations: &[Vec<(usize, Complex64)>],
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let Some(oscillator) = &point.config().oscillator else {
        return Ok(());
    };
    if request.frequencies_hz.len() < 2
        || (request.integration.is_none() && !request.contributor_ranking)
    {
        return Ok(());
    }
    let anchor = request.frequency_anchor();
    for (output, direction) in request.outputs.iter().zip(observations) {
        let [low, high] =
            if let Some([low, high]) = request.integration.as_ref().and_then(|i| i.band_hz) {
                [
                    grid.frequency_relative_to(low, &output.lattice, &anchor)
                        .map_err(numerical_error)?,
                    grid.frequency_relative_to(high, &output.lattice, &anchor)
                        .map_err(numerical_error)?,
                ]
            } else {
                [
                    request.frequencies_hz[0],
                    *request.frequencies_hz.last().unwrap(),
                ]
            };
        for (k, tuple) in grid.indices().iter().enumerate() {
            check_abort(abort)?;
            if tuple[oscillator.tone] == 0 {
                continue;
            }
            let coefficient: Complex64 = direction
                .iter()
                .map(|(row, weight)| weight.conj() * point.complete_spectra()[*row][k])
                .sum();
            if coefficient == Complex64::ZERO {
                continue;
            }
            // Compare on the authored axis: adding a sub-ulp positive
            // offset to its carrier must not move the lower limit to zero.
            let relative: Vec<_> = tuple
                .iter()
                .zip(&anchor)
                .map(|(k, a)| {
                    k.checked_add(*a).ok_or_else(|| {
                        qpnoise_error("autonomous noise frequency anchor overflowed")
                    })
                })
                .collect::<Result<_, _>>()?;
            let pole = grid
                .frequency_relative_to(0.0, &output.lattice, &relative)
                .map_err(numerical_error)?;
            // A clipped band can still interpolate from samples straddling
            // the pole. Check the complete brackets of every used interval.
            let mut crossing = false;
            for pair in request.frequencies_hz.windows(2) {
                check_abort(abort)?;
                crossing |=
                    pair[0].max(low) < pair[1].min(high) && pair[0] <= pole && pole <= pair[1];
                if crossing {
                    break;
                }
            }
            if crossing {
                return Err(qpnoise_error(
                    "autonomous noise integration and contributor ranking cannot interpolate across a retained oscillator spectral line; use samples and a band on one side, or disable integrated noise and contributor ranking",
                ));
            }
        }
    }
    Ok(())
}

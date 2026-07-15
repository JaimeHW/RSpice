use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::noise::NoiseResult;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::error::{ensure_not_aborted, poll_periodically};
use super::{ServiceRunError, ServiceRunResult};

pub(super) fn resolve_pnoise_sideband_stride_with_abort(
    max_sideband: i32,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<usize> {
    ensure_not_aborted(abort)?;
    let non_negative = u64::try_from(max_sideband.max(0)).map_err(|_| {
        ServiceRunError::Failure(format!("PNOISE max sideband '{max_sideband}' is invalid"))
    })?;
    let factor = non_negative
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "PNOISE sideband factor overflow for max sideband '{}'",
                max_sideband
            ))
        })?;
    ensure_not_aborted(abort)?;
    usize::try_from(factor).map_err(|_| {
        ServiceRunError::Failure(format!(
            "PNOISE sideband factor '{}' is unsupported on this platform",
            factor
        ))
    })
}

pub(super) fn build_pnoise_sideband_translated_frequencies_with_abort(
    offset_frequencies: &[Value],
    carrier_frequency: Value,
    max_sideband: i32,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<Value>> {
    ensure_not_aborted(abort)?;
    if !carrier_frequency.is_finite() || carrier_frequency <= 0.0 {
        return Err(ServiceRunError::Failure(format!(
            "PNOISE carrier frequency must be finite and positive, got {}",
            carrier_frequency
        )));
    }

    let sideband_stride = resolve_pnoise_sideband_stride_with_abort(max_sideband, abort)?;
    let total_points = offset_frequencies
        .len()
        .checked_mul(sideband_stride)
        .ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "PNOISE translated-frequency buffer size overflow ({} offsets x {} sidebands)",
                offset_frequencies.len(),
                sideband_stride
            ))
        })?;
    let sideband_max = max_sideband.max(0);
    let mut translated = Vec::with_capacity(total_points);

    let mut poll_index = 0usize;
    for &offset in offset_frequencies {
        poll_periodically(abort, poll_index)?;
        if !offset.is_finite() || offset < 0.0 {
            return Err(ServiceRunError::Failure(format!(
                "PNOISE offset frequencies must be finite and non-negative, got {}",
                offset
            )));
        }
        for sideband in -sideband_max..=sideband_max {
            poll_periodically(abort, poll_index)?;
            poll_index = poll_index.saturating_add(1);
            let translated_freq = (offset + sideband as Value * carrier_frequency)
                .abs()
                .max(1e-30);
            if !translated_freq.is_finite() {
                return Err(ServiceRunError::Failure(format!(
                    "PNOISE translated frequency became non-finite for offset {} and sideband {}",
                    offset, sideband
                )));
            }
            translated.push(translated_freq);
        }
    }

    ensure_not_aborted(abort)?;
    Ok(translated)
}

fn validate_sideband_shape(
    translated_len: usize,
    num_offsets: usize,
    sideband_stride: usize,
    quantity: &str,
) -> Result<(), String> {
    if sideband_stride == 0 {
        return Err(format!(
            "PNOISE {} folding requires a positive sideband stride",
            quantity
        ));
    }

    let expected_len = num_offsets.checked_mul(sideband_stride).ok_or_else(|| {
        format!(
            "PNOISE {} folding overflow ({} offsets x {} sidebands)",
            quantity, num_offsets, sideband_stride
        )
    })?;
    if translated_len != expected_len {
        return Err(format!(
            "PNOISE {} folding expected {} translated points but received {}",
            quantity, expected_len, translated_len
        ));
    }

    Ok(())
}

fn validate_sideband_shape_with_abort(
    translated_len: usize,
    num_offsets: usize,
    sideband_stride: usize,
    quantity: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    ensure_not_aborted(abort)?;
    validate_sideband_shape(translated_len, num_offsets, sideband_stride, quantity)
        .map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)
}

pub(super) fn fold_sideband_noise_results_with_abort<F>(
    translated_results: &[NoiseResult],
    num_offsets: usize,
    sideband_stride: usize,
    quantity: &str,
    abort: &dyn AbortSignal,
    mut sample: F,
) -> ServiceRunResult<Vec<Value>>
where
    F: FnMut(&NoiseResult) -> Value,
{
    validate_sideband_shape_with_abort(
        translated_results.len(),
        num_offsets,
        sideband_stride,
        quantity,
        abort,
    )?;

    let mut folded = Vec::with_capacity(num_offsets);
    let mut poll_index = 0usize;
    for chunk in translated_results.chunks_exact(sideband_stride) {
        let mut total = 0.0;
        for point in chunk {
            poll_periodically(abort, poll_index)?;
            poll_index = poll_index.saturating_add(1);
            total += sample(point);
        }
        folded.push(total);
    }
    ensure_not_aborted(abort)?;
    Ok(folded)
}

pub(super) fn fold_sideband_contributors_with_abort(
    translated_results: &[NoiseResult],
    sideband_stride: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(String, Value)>> {
    ensure_not_aborted(abort)?;
    if translated_results.is_empty() {
        return Ok(Vec::new());
    }
    if sideband_stride == 0 {
        return Err(ServiceRunError::Failure(
            "PNOISE contributor folding requires a positive sideband stride".to_string(),
        ));
    }
    if !crate::utils::numeric::is_multiple_of(translated_results.len(), sideband_stride) {
        return Err(ServiceRunError::Failure(format!(
            "PNOISE contributor folding expected translated results to be divisible by sideband stride (len={}, stride={})",
            translated_results.len(),
            sideband_stride
        )));
    }

    let mut combined: HashMap<String, Value> = HashMap::new();
    let mut poll_index = 0usize;
    for point in translated_results {
        for contrib in &point.contributions {
            poll_periodically(abort, poll_index)?;
            poll_index = poll_index.saturating_add(1);
            if contrib.output_contribution.is_finite() {
                let entry = combined
                    .entry(contrib.identity.device.clone())
                    .or_insert(0.0);
                *entry += contrib.output_contribution.max(0.0);
            }
        }
    }

    let total: Value = combined.values().sum();
    let mut contributors = Vec::with_capacity(combined.len());
    for (index, (name, contribution)) in combined.into_iter().enumerate() {
        poll_periodically(abort, index)?;
        let percentage = if total > 0.0 {
            100.0 * contribution / total
        } else {
            0.0
        };
        contributors.push((name, percentage));
    }
    contributors.sort_by(|lhs, rhs| {
        rhs.1
            .partial_cmp(&lhs.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| lhs.0.cmp(&rhs.0))
    });

    ensure_not_aborted(abort)?;
    Ok(contributors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::CountingAbort;

    #[test]
    fn translated_frequency_expansion_honors_in_loop_abort() {
        let offsets = (0..130).map(|index| index as Value).collect::<Vec<_>>();
        let abort = CountingAbort::new(4);

        let result =
            build_pnoise_sideband_translated_frequencies_with_abort(&offsets, 1e6, 0, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.count() > 4);
    }
}

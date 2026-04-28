use rspice_core::Value;
use rspice_core::analysis::noise::NoiseResult;
use std::cmp::Ordering;
use std::collections::HashMap;

pub(super) fn resolve_pnoise_sideband_stride(max_sideband: i32) -> Result<usize, String> {
    let non_negative = u64::try_from(max_sideband.max(0))
        .map_err(|_| format!("PNOISE max sideband '{}' is invalid", max_sideband))?;
    let factor = non_negative
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| {
            format!(
                "PNOISE sideband factor overflow for max sideband '{}'",
                max_sideband
            )
        })?;
    usize::try_from(factor).map_err(|_| {
        format!(
            "PNOISE sideband factor '{}' is unsupported on this platform",
            factor
        )
    })
}

pub(super) fn build_pnoise_sideband_translated_frequencies(
    offset_frequencies: &[Value],
    carrier_frequency: Value,
    max_sideband: i32,
) -> Result<Vec<Value>, String> {
    if !carrier_frequency.is_finite() || carrier_frequency <= 0.0 {
        return Err(format!(
            "PNOISE carrier frequency must be finite and positive, got {}",
            carrier_frequency
        ));
    }

    let sideband_stride = resolve_pnoise_sideband_stride(max_sideband)?;
    let total_points = offset_frequencies
        .len()
        .checked_mul(sideband_stride)
        .ok_or_else(|| {
            format!(
                "PNOISE translated-frequency buffer size overflow ({} offsets x {} sidebands)",
                offset_frequencies.len(),
                sideband_stride
            )
        })?;
    let sideband_max = max_sideband.max(0);
    let mut translated = Vec::with_capacity(total_points);

    for &offset in offset_frequencies {
        if !offset.is_finite() || offset < 0.0 {
            return Err(format!(
                "PNOISE offset frequencies must be finite and non-negative, got {}",
                offset
            ));
        }
        for sideband in -sideband_max..=sideband_max {
            let translated_freq = (offset + sideband as Value * carrier_frequency)
                .abs()
                .max(1e-30);
            if !translated_freq.is_finite() {
                return Err(format!(
                    "PNOISE translated frequency became non-finite for offset {} and sideband {}",
                    offset, sideband
                ));
            }
            translated.push(translated_freq);
        }
    }

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


pub(super) fn fold_sideband_noise_results<F>(
    translated_results: &[NoiseResult],
    num_offsets: usize,
    sideband_stride: usize,
    quantity: &str,
    mut sample: F,
) -> Result<Vec<Value>, String>
where
    F: FnMut(&NoiseResult) -> Value,
{
    validate_sideband_shape(
        translated_results.len(),
        num_offsets,
        sideband_stride,
        quantity,
    )?;

    Ok(translated_results
        .chunks_exact(sideband_stride)
        .map(|chunk| chunk.iter().map(&mut sample).sum())
        .collect())
}

pub(super) fn fold_sideband_contributors(
    translated_results: &[NoiseResult],
    sideband_stride: usize,
) -> Result<Vec<(String, Value)>, String> {
    if translated_results.is_empty() {
        return Ok(Vec::new());
    }
    if sideband_stride == 0 {
        return Err("PNOISE contributor folding requires a positive sideband stride".to_string());
    }
    if !crate::utils::numeric::is_multiple_of(translated_results.len(), sideband_stride) {
        return Err(format!(
            "PNOISE contributor folding expected translated results to be divisible by sideband stride (len={}, stride={})",
            translated_results.len(),
            sideband_stride
        ));
    }

    let mut combined: HashMap<String, Value> = HashMap::new();
    for point in translated_results {
        for contrib in &point.contributions {
            if contrib.output_contribution.is_finite() {
                let entry = combined.entry(contrib.device_name.clone()).or_insert(0.0);
                *entry += contrib.output_contribution.max(0.0);
            }
        }
    }

    let total: Value = combined.values().sum();
    let mut contributors: Vec<(String, Value)> = combined
        .into_iter()
        .map(|(name, contribution)| {
            let percentage = if total > 0.0 {
                100.0 * contribution / total
            } else {
                0.0
            };
            (name, percentage)
        })
        .collect();
    contributors.sort_by(|lhs, rhs| {
        rhs.1
            .partial_cmp(&lhs.1)
            .unwrap_or(Ordering::Equal)
            .then_with(|| lhs.0.cmp(&rhs.0))
    });

    Ok(contributors)
}


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

pub(super) fn fold_sideband_samples(
    translated_values: &[Value],
    num_offsets: usize,
    sideband_stride: usize,
    quantity: &str,
) -> Result<Vec<Value>, String> {
    validate_sideband_shape(
        translated_values.len(),
        num_offsets,
        sideband_stride,
        quantity,
    )?;

    Ok(translated_values
        .chunks_exact(sideband_stride)
        .map(|chunk| chunk.iter().sum())
        .collect())
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
        .map(|chunk| chunk.iter().map(|point| sample(point)).sum())
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
    if translated_results.len() % sideband_stride != 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::analysis::noise::{NoiseContribution, NoiseSourceType};

    fn build_noise_result(contributions: &[(&str, Value)]) -> NoiseResult {
        NoiseResult {
            frequency: 0.0,
            output_noise_density: 0.0,
            input_referred_density: 0.0,
            contributions: contributions
                .iter()
                .map(|(device, contribution)| NoiseContribution {
                    device_name: (*device).to_string(),
                    noise_type: NoiseSourceType::Thermal,
                    output_contribution: *contribution,
                    percentage: 0.0,
                })
                .collect(),
        }
    }

    #[test]
    fn test_resolve_pnoise_sideband_stride() {
        assert_eq!(
            resolve_pnoise_sideband_stride(0).expect("max_sideband=0 should resolve"),
            1
        );
        assert_eq!(
            resolve_pnoise_sideband_stride(2).expect("max_sideband=2 should resolve"),
            5
        );
        assert_eq!(
            resolve_pnoise_sideband_stride(-3).expect("negative sideband is clamped"),
            1
        );
    }

    #[test]
    fn test_build_translated_frequencies_rejects_invalid_carrier() {
        let err = build_pnoise_sideband_translated_frequencies(&[1.0], 0.0, 1)
            .expect_err("zero carrier should be rejected");
        assert!(err.contains("carrier frequency"));
    }

    #[test]
    fn test_build_translated_frequencies_matches_expected_sideband_order() {
        let translated = build_pnoise_sideband_translated_frequencies(&[1.0, 2.0], 10.0, 1)
            .expect("translated sidebands should be generated");
        assert_eq!(translated, vec![9.0, 1.0, 11.0, 8.0, 2.0, 12.0]);
    }

    #[test]
    fn test_fold_sideband_noise_results_matches_manual_chunk_sum() {
        let points = vec![
            NoiseResult {
                frequency: 1.0,
                output_noise_density: 1.0,
                input_referred_density: 2.0,
                contributions: Vec::new(),
            },
            NoiseResult {
                frequency: 1.0,
                output_noise_density: 3.0,
                input_referred_density: 4.0,
                contributions: Vec::new(),
            },
            NoiseResult {
                frequency: 2.0,
                output_noise_density: 5.0,
                input_referred_density: 6.0,
                contributions: Vec::new(),
            },
            NoiseResult {
                frequency: 2.0,
                output_noise_density: 7.0,
                input_referred_density: 8.0,
                contributions: Vec::new(),
            },
        ];

        let folded = fold_sideband_noise_results(&points, 2, 2, "output-referred", |point| {
            point.output_noise_density
        })
        .expect("folding should succeed");
        assert_eq!(folded, vec![4.0, 12.0]);
    }

    #[test]
    fn test_fold_sideband_contributors_accumulates_all_offsets() {
        let translated_results = vec![
            build_noise_result(&[("R1", 2.0), ("R2", 1.0)]),
            build_noise_result(&[("R1", 1.0)]),
            build_noise_result(&[("R2", 4.0)]),
            build_noise_result(&[("R1", 3.0), ("R3", 2.0)]),
            build_noise_result(&[("R2", 2.0), ("R3", 1.0)]),
            build_noise_result(&[("R1", 2.0)]),
        ];

        let contributors = fold_sideband_contributors(&translated_results, 3)
            .expect("contributor folding should succeed");
        assert_eq!(contributors.len(), 3);
        assert_eq!(contributors[0].0, "R1");
        assert_eq!(contributors[1].0, "R2");
        assert_eq!(contributors[2].0, "R3");

        let total: Value = contributors.iter().map(|(_, percentage)| *percentage).sum();
        assert!((total - 100.0).abs() < 1e-9);
        assert!((contributors[0].1 - (800.0 / 18.0)).abs() < 1e-9);
        assert!((contributors[1].1 - (700.0 / 18.0)).abs() < 1e-9);
        assert!((contributors[2].1 - (300.0 / 18.0)).abs() < 1e-9);
    }

    #[test]
    fn test_fold_sideband_contributors_tie_breaks_by_name() {
        let translated_results = vec![
            build_noise_result(&[("R2", 1.0), ("R1", 1.0)]),
            build_noise_result(&[("R2", 1.0), ("R1", 1.0)]),
        ];

        let contributors = fold_sideband_contributors(&translated_results, 1)
            .expect("contributor folding should succeed");
        assert_eq!(contributors[0].0, "R1");
        assert_eq!(contributors[1].0, "R2");
    }

    #[test]
    fn test_fold_sideband_contributors_rejects_non_divisible_shape() {
        let translated_results = vec![
            build_noise_result(&[("R1", 1.0)]),
            build_noise_result(&[("R1", 1.0)]),
            build_noise_result(&[("R1", 1.0)]),
        ];

        let err = fold_sideband_contributors(&translated_results, 2)
            .expect_err("shape mismatch should be rejected");
        assert!(err.contains("divisible"));
    }
}

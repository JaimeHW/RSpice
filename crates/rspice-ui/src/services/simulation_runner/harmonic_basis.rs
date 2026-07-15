use super::error::ensure_not_aborted;
use super::{HbToneRunConfig, ServiceRunError, ServiceRunResult};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;

#[derive(Debug, Clone)]
pub(super) struct HbMultiToneLayout {
    pub(super) base_frequency: Value,
    pub(super) max_harmonic: usize,
    pub(super) tone_harmonics: Vec<usize>,
}

fn gcd_u64_with_abort(mut a: u64, mut b: u64, abort: &dyn AbortSignal) -> ServiceRunResult<u64> {
    ensure_not_aborted(abort)?;
    while b != 0 {
        ensure_not_aborted(abort)?;
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    ensure_not_aborted(abort)?;
    Ok(a)
}

fn lcm_u64_with_abort(a: u64, b: u64, abort: &dyn AbortSignal) -> ServiceRunResult<Option<u64>> {
    ensure_not_aborted(abort)?;
    if a == 0 || b == 0 {
        return Ok(Some(0));
    }
    let gcd = gcd_u64_with_abort(a, b, abort)?.max(1);
    ensure_not_aborted(abort)?;
    Ok((a / gcd).checked_mul(b))
}

fn approximate_ratio_fraction_with_abort(
    value: Value,
    max_denominator: u32,
    rel_tol: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<(u32, u32)>> {
    ensure_not_aborted(abort)?;
    if !value.is_finite() || value <= 0.0 || max_denominator == 0 {
        return Ok(None);
    }

    let mut best: Option<(u32, u32, Value)> = None;
    for denominator in 1..=max_denominator {
        ensure_not_aborted(abort)?;
        let numerator_f = (value * denominator as Value).round();
        if !numerator_f.is_finite() || numerator_f <= 0.0 {
            continue;
        }
        let numerator = numerator_f as u32;
        if numerator == 0 {
            continue;
        }

        let approximated = numerator as Value / denominator as Value;
        let rel_error = ((approximated - value).abs() / value.abs().max(1.0)).abs();
        if rel_error > rel_tol {
            continue;
        }

        match best {
            Some((_, _, current_error)) if rel_error >= current_error => {}
            _ => best = Some((numerator, denominator, rel_error)),
        }
    }

    ensure_not_aborted(abort)?;
    Ok(best.map(|(numerator, denominator, _)| (numerator, denominator)))
}

pub(super) fn build_multi_tone_hb_layout_with_abort(
    tones: &[HbToneRunConfig],
    max_mixing_order: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HbMultiToneLayout> {
    ensure_not_aborted(abort)?;
    let first_tone = tones
        .first()
        .ok_or_else(|| ServiceRunError::Failure("HB requires at least one tone".to_string()));
    ensure_not_aborted(abort)?;
    let first_tone = first_tone?;
    if tones.len() == 1 {
        ensure_not_aborted(abort)?;
        return Ok(HbMultiToneLayout {
            base_frequency: first_tone.frequency,
            max_harmonic: first_tone.harmonics.max(1),
            tone_harmonics: vec![1],
        });
    }

    let mut min_frequency = Value::INFINITY;
    for tone in tones {
        ensure_not_aborted(abort)?;
        min_frequency = min_frequency.min(tone.frequency);
    }
    if !min_frequency.is_finite() || min_frequency <= 0.0 {
        return Err(ServiceRunError::Failure(
            "HB tone frequencies must be positive".to_string(),
        ));
    }

    let mut reduced_fractions = Vec::with_capacity(tones.len());
    for tone in tones {
        ensure_not_aborted(abort)?;
        let ratio = tone.frequency / min_frequency;
        let fraction = approximate_ratio_fraction_with_abort(ratio, 48, 1e-6, abort)?;
        ensure_not_aborted(abort)?;
        let (numerator, denominator) = fraction.ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "HB tone ratio {} cannot be represented as a stable low-order rational ratio",
                ratio
            ))
        })?;
        let numerator = numerator as u64;
        let denominator = denominator as u64;
        let gcd = gcd_u64_with_abort(numerator, denominator, abort)?.max(1);
        reduced_fractions.push((numerator / gcd, denominator / gcd));
    }

    let mut common_denominator: u64 = 1;
    for (_, denominator) in &reduced_fractions {
        ensure_not_aborted(abort)?;
        common_denominator = lcm_u64_with_abort(common_denominator, *denominator, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure("HB multi-tone ratio harmonization overflowed".to_string())
            })?;
    }
    if common_denominator == 0 {
        return Err(ServiceRunError::Failure(
            "HB multi-tone ratio harmonization produced invalid denominator".to_string(),
        ));
    }

    let base_frequency = min_frequency / common_denominator as Value;
    let mut tone_harmonics = Vec::with_capacity(tones.len());
    for (idx, (numerator, denominator)) in reduced_fractions.iter().enumerate() {
        ensure_not_aborted(abort)?;
        let scale = common_denominator
            .checked_div(*denominator)
            .ok_or_else(|| {
                ServiceRunError::Failure("HB tone denominator harmonization failed".to_string())
            })?;
        let harmonic = numerator
            .checked_mul(scale)
            .ok_or_else(|| ServiceRunError::Failure("HB harmonic index overflowed".to_string()))?;
        let harmonic = usize::try_from(harmonic).map_err(|_| {
            ServiceRunError::Failure("HB harmonic index exceeds this platform".to_string())
        })?;
        if harmonic == 0 {
            return Err(ServiceRunError::Failure(format!(
                "HB tone {} resolved to zero harmonic",
                idx + 1
            )));
        }
        let mapped_frequency = base_frequency * harmonic as Value;
        let rel_error =
            (mapped_frequency - tones[idx].frequency).abs() / tones[idx].frequency.abs().max(1.0);
        if rel_error > 1e-9 {
            return Err(ServiceRunError::Failure(format!(
                "HB tone {} cannot be mapped onto a stable commensurate harmonic basis",
                idx + 1
            )));
        }
        tone_harmonics.push(harmonic);
    }

    let mut max_harmonic = 1;
    for (tone, harmonic) in tones.iter().zip(&tone_harmonics) {
        ensure_not_aborted(abort)?;
        max_harmonic = max_harmonic.max(harmonic.saturating_mul(tone.harmonics.max(1)));
    }
    if tones.len() > 1 {
        let mut basis_peak = 1;
        for harmonic in &tone_harmonics {
            ensure_not_aborted(abort)?;
            basis_peak = basis_peak.max(*harmonic);
        }
        max_harmonic = max_harmonic.max(max_mixing_order.max(1).saturating_mul(basis_peak));
    }
    if max_harmonic > 2048 {
        return Err(ServiceRunError::Failure(format!(
            "HB multi-tone harmonic order {} exceeds supported practical limit",
            max_harmonic
        )));
    }

    ensure_not_aborted(abort)?;
    Ok(HbMultiToneLayout {
        base_frequency,
        max_harmonic,
        tone_harmonics,
    })
}

#[derive(Debug, Clone, Copy)]
pub(super) struct DistoTwoToneHarmonicPlan {
    pub(super) f2_over_f1: Value,
    pub(super) tone1_harmonic: usize,
    pub(super) tone2_harmonic: usize,
    pub(super) max_harmonic: usize,
}

pub(super) fn build_disto_two_tone_harmonic_plan_with_abort(
    f2_over_f1: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<DistoTwoToneHarmonicPlan> {
    ensure_not_aborted(abort)?;
    let tones = vec![
        HbToneRunConfig::new(1.0, 1),
        HbToneRunConfig::new(f2_over_f1, 1),
    ];
    let layout = build_multi_tone_hb_layout_with_abort(&tones, 3, abort).map_err(|error| {
        if error.is_aborted() {
            return error;
        }
        ServiceRunError::Failure(format!(
            "DISTO f2_over_f1={} cannot be represented as a stable low-order rational ratio: {}",
            f2_over_f1, error
        ))
    })?;
    ensure_not_aborted(abort)?;
    let tone1_harmonic = layout.tone_harmonics.first().copied().ok_or_else(|| {
        ServiceRunError::Failure("DISTO harmonic mapping failed for tone 1".to_string())
    })?;
    let tone2_harmonic = layout.tone_harmonics.get(1).copied().ok_or_else(|| {
        ServiceRunError::Failure("DISTO harmonic mapping failed for tone 2".to_string())
    })?;
    if tone2_harmonic <= tone1_harmonic {
        return Err(ServiceRunError::Failure(format!(
            "DISTO f2_over_f1={} must map to tone2 > tone1",
            f2_over_f1
        )));
    }

    let max_harmonic = layout.max_harmonic.max(
        (3 * tone1_harmonic)
            .max(3 * tone2_harmonic)
            .max(tone1_harmonic + tone2_harmonic)
            .max((2 * tone1_harmonic).abs_diff(tone2_harmonic))
            .max((2 * tone2_harmonic).abs_diff(tone1_harmonic)),
    );

    if max_harmonic > 256 {
        return Err(ServiceRunError::Failure(format!(
            "DISTO two-tone HB harmonic order {} exceeds supported practical limit",
            max_harmonic
        )));
    }

    ensure_not_aborted(abort)?;
    Ok(DistoTwoToneHarmonicPlan {
        f2_over_f1,
        tone1_harmonic,
        tone2_harmonic,
        max_harmonic,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn multi_tone_layout_observes_abort_inside_ratio_search() {
        let tones = [
            HbToneRunConfig::new(1.0e6, 3),
            HbToneRunConfig::new(1.5e6, 3),
        ];
        let abort = AbortOnPoll {
            abort_on: 12,
            polls: AtomicUsize::new(0),
        };

        let result = build_multi_tone_hb_layout_with_abort(&tones, 3, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}

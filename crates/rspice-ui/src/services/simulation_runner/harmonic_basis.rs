use super::HbToneRunConfig;
use rspice_core::Value;

#[derive(Debug, Clone)]
pub(super) struct HbMultiToneLayout {
    pub(super) base_frequency: Value,
    pub(super) max_harmonic: usize,
    pub(super) tone_harmonics: Vec<usize>,
}

fn gcd_u64(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm_u64(a: u64, b: u64) -> Option<u64> {
    if a == 0 || b == 0 {
        return Some(0);
    }
    let gcd = gcd_u64(a, b).max(1);
    (a / gcd).checked_mul(b)
}

fn approximate_ratio_fraction(
    value: Value,
    max_denominator: u32,
    rel_tol: Value,
) -> Option<(u32, u32)> {
    if !value.is_finite() || value <= 0.0 || max_denominator == 0 {
        return None;
    }

    let mut best: Option<(u32, u32, Value)> = None;
    for denominator in 1..=max_denominator {
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

    best.map(|(numerator, denominator, _)| (numerator, denominator))
}

pub(super) fn build_multi_tone_hb_layout(
    tones: &[HbToneRunConfig],
    max_mixing_order: usize,
) -> Result<HbMultiToneLayout, String> {
    let first_tone = tones
        .first()
        .ok_or_else(|| "HB requires at least one tone".to_string())?;
    if tones.len() == 1 {
        return Ok(HbMultiToneLayout {
            base_frequency: first_tone.frequency,
            max_harmonic: first_tone.harmonics.max(1),
            tone_harmonics: vec![1],
        });
    }

    let min_frequency = tones
        .iter()
        .map(|tone| tone.frequency)
        .fold(Value::INFINITY, Value::min);
    if !min_frequency.is_finite() || min_frequency <= 0.0 {
        return Err("HB tone frequencies must be positive".to_string());
    }

    let mut reduced_fractions = Vec::with_capacity(tones.len());
    for tone in tones {
        let ratio = tone.frequency / min_frequency;
        let (numerator, denominator) =
            approximate_ratio_fraction(ratio, 48, 1e-6).ok_or_else(|| {
                format!(
                    "HB tone ratio {} cannot be represented as a stable low-order rational ratio",
                    ratio
                )
            })?;
        let numerator = numerator as u64;
        let denominator = denominator as u64;
        let gcd = gcd_u64(numerator, denominator).max(1);
        reduced_fractions.push((numerator / gcd, denominator / gcd));
    }

    let mut common_denominator: u64 = 1;
    for (_, denominator) in &reduced_fractions {
        common_denominator = lcm_u64(common_denominator, *denominator)
            .ok_or_else(|| "HB multi-tone ratio harmonization overflowed".to_string())?;
    }
    if common_denominator == 0 {
        return Err("HB multi-tone ratio harmonization produced invalid denominator".to_string());
    }

    let base_frequency = min_frequency / common_denominator as Value;
    let mut tone_harmonics = Vec::with_capacity(tones.len());
    for (idx, (numerator, denominator)) in reduced_fractions.iter().enumerate() {
        let scale = common_denominator
            .checked_div(*denominator)
            .ok_or_else(|| "HB tone denominator harmonization failed".to_string())?;
        let harmonic = numerator
            .checked_mul(scale)
            .ok_or_else(|| "HB harmonic index overflowed".to_string())?;
        let harmonic = harmonic as usize;
        if harmonic == 0 {
            return Err(format!("HB tone {} resolved to zero harmonic", idx + 1));
        }
        let mapped_frequency = base_frequency * harmonic as Value;
        let rel_error =
            (mapped_frequency - tones[idx].frequency).abs() / tones[idx].frequency.abs().max(1.0);
        if rel_error > 1e-9 {
            return Err(format!(
                "HB tone {} cannot be mapped onto a stable commensurate harmonic basis",
                idx + 1
            ));
        }
        tone_harmonics.push(harmonic);
    }

    let mut max_harmonic = tones
        .iter()
        .zip(tone_harmonics.iter())
        .map(|(tone, harmonic)| harmonic.saturating_mul(tone.harmonics.max(1)))
        .max()
        .unwrap_or(1);
    if tones.len() > 1 {
        let basis_peak = tone_harmonics.iter().copied().max().unwrap_or(1);
        max_harmonic = max_harmonic.max(max_mixing_order.max(1).saturating_mul(basis_peak));
    }
    if max_harmonic > 2048 {
        return Err(format!(
            "HB multi-tone harmonic order {} exceeds supported practical limit",
            max_harmonic
        ));
    }

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

pub(super) fn build_disto_two_tone_harmonic_plan(
    f2_over_f1: Value,
) -> Result<DistoTwoToneHarmonicPlan, String> {
    let tones = vec![
        HbToneRunConfig::new(1.0, 1),
        HbToneRunConfig::new(f2_over_f1, 1),
    ];
    let layout = build_multi_tone_hb_layout(&tones, 3).map_err(|err| {
        format!(
            "DISTO f2_over_f1={} cannot be represented as a stable low-order rational ratio: {}",
            f2_over_f1, err
        )
    })?;
    let tone1_harmonic = layout
        .tone_harmonics
        .first()
        .copied()
        .ok_or_else(|| "DISTO harmonic mapping failed for tone 1".to_string())?;
    let tone2_harmonic = layout
        .tone_harmonics
        .get(1)
        .copied()
        .ok_or_else(|| "DISTO harmonic mapping failed for tone 2".to_string())?;
    if tone2_harmonic <= tone1_harmonic {
        return Err(format!(
            "DISTO f2_over_f1={} must map to tone2 > tone1",
            f2_over_f1
        ));
    }

    let max_harmonic = layout.max_harmonic.max(
        (3 * tone1_harmonic)
            .max(3 * tone2_harmonic)
            .max(tone1_harmonic + tone2_harmonic)
            .max((2 * tone1_harmonic).abs_diff(tone2_harmonic))
            .max((2 * tone2_harmonic).abs_diff(tone1_harmonic)),
    );

    if max_harmonic > 256 {
        return Err(format!(
            "DISTO two-tone HB harmonic order {} exceeds supported practical limit",
            max_harmonic
        ));
    }

    Ok(DistoTwoToneHarmonicPlan {
        f2_over_f1,
        tone1_harmonic,
        tone2_harmonic,
        max_harmonic,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_multi_tone_hb_layout_two_tone_ratio() {
        let tones = vec![HbToneRunConfig::new(2e6, 3), HbToneRunConfig::new(3e6, 2)];
        let layout = build_multi_tone_hb_layout(&tones, 4)
            .expect("2 MHz/3 MHz tones should be commensurate");
        assert!((layout.base_frequency - 1e6).abs() < 1e-9);
        assert_eq!(layout.tone_harmonics, vec![2, 3]);
        assert!(layout.max_harmonic >= 12);
    }

    #[test]
    fn test_build_disto_two_tone_harmonic_plan_basic() {
        let plan = build_disto_two_tone_harmonic_plan(1.5).expect("3/2 ratio should map");
        assert_eq!(plan.tone1_harmonic, 2);
        assert_eq!(plan.tone2_harmonic, 3);
        assert!(plan.max_harmonic >= 9);
    }
}

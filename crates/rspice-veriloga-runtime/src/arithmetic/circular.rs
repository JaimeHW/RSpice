//! Exact accumulated translations for circular integrators.

use super::scalar::{
    BigMagnitude, SmallExact, exact_binary_to_f64, scaled_exact_ratio_to_f64,
    scaled_sum_triple_products, small_exact_ratio_to_f64,
};

const FLOOR: i32 = -1074;
// More than the complete binary64 exponent span plus 1900 carry bits. This
// bounds hostile checkpoint allocations without limiting any realizable run.
const MAX_BITS: usize = 4096;

/// Portable, canonical dyadic representation: signed `words * 2^exponent`.
/// Words are little-endian, without leading zero words or trailing zero bits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdtModOriginCheckpoint {
    pub negative: bool,
    pub exponent: i32,
    pub words: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Origin {
    Small(SmallExact),
    Wide {
        negative: bool,
        magnitude: Box<BigMagnitude>,
    },
}

/// Exact accumulated translation between an integral and its visible phase.
/// Ordinary oscillators use an inline integer. Wide exponent spans use the
/// runtime's existing exact accumulator rather than a rounded running total.
#[derive(Debug, Clone)]
pub struct IdtModOrigin(Origin);

impl Default for IdtModOrigin {
    fn default() -> Self {
        Self(Origin::Small(SmallExact::ZERO))
    }
}

impl PartialEq for IdtModOrigin {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Origin::Small(left), Origin::Small(right)) => left == right,
            _ => self.checkpoint() == other.checkpoint(),
        }
    }
}

impl Eq for IdtModOrigin {}

fn finite_exact(value: f64) -> Result<SmallExact, &'static str> {
    if !value.is_finite() {
        return Err("circular-integrator translation must be finite");
    }
    if value == 0.0 {
        Ok(SmallExact::ZERO)
    } else {
        SmallExact::product(value, 1.0).ok_or("invalid circular-integrator translation")
    }
}

fn shifted_word(value: &BigMagnitude, index: usize, shift: usize) -> u64 {
    let base = index + shift / 64;
    let bits = shift % 64;
    if bits == 0 {
        value.word(base)
    } else {
        (value.word(base) >> bits) | (value.word(base + 1) << (64 - bits))
    }
}

fn power_of_two_mod(mut exponent: u32, modulus: u64) -> u64 {
    let mut value = 1 % modulus;
    let mut factor = 2 % modulus;
    while exponent != 0 {
        if exponent & 1 != 0 {
            value = ((u128::from(value) * u128::from(factor)) % u128::from(modulus)) as u64;
        }
        exponent >>= 1;
        factor = ((u128::from(factor) * u128::from(factor)) % u128::from(modulus)) as u64;
    }
    value
}

fn circular_interval(modulus: f64, offset: f64) -> Result<f64, &'static str> {
    if !offset.is_finite() {
        return Err("offset must be finite");
    }
    if !modulus.is_finite() || modulus <= 0.0 {
        return Err("modulus must be finite and greater than zero");
    }
    let upper = offset + modulus;
    if !upper.is_finite() || upper <= offset {
        return Err("offset and modulus must form a finite, nonempty interval");
    }
    Ok(upper)
}

impl IdtModOrigin {
    /// Maximum serialized mantissa length, checked before checkpoint allocation.
    pub const MAX_CHECKPOINT_WORDS: usize = MAX_BITS / 64;

    pub const ZERO: Self = Self(Origin::Small(SmallExact::ZERO));

    fn wide(&self) -> (bool, BigMagnitude) {
        match &self.0 {
            Origin::Wide {
                negative,
                magnitude,
            } => (*negative, magnitude.as_ref().clone()),
            Origin::Small(value) => {
                let mut magnitude = BigMagnitude::default();
                if value.value != 0 {
                    magnitude.add_shifted(
                        value.value.unsigned_abs(),
                        (value.exponent - FLOOR) as usize,
                    );
                }
                (value.value < 0, magnitude)
            }
        }
    }

    fn from_wide(negative: bool, magnitude: BigMagnitude) -> Result<Self, &'static str> {
        let Some(top) = magnitude.top_bit() else {
            return Ok(Self::default());
        };
        if top >= MAX_BITS {
            return Err("circular-integrator origin exceeds the exact history capacity");
        }
        let first = (0..magnitude.significant_len())
            .find(|&i| magnitude.word(i) != 0)
            .unwrap();
        let trailing = first * 64 + magnitude.word(first).trailing_zeros() as usize;
        if top - trailing < 127 {
            let unsigned = u128::from(shifted_word(&magnitude, 0, trailing))
                | (u128::from(shifted_word(&magnitude, 1, trailing)) << 64);
            let signed = unsigned as i128;
            return Ok(Self(Origin::Small(SmallExact {
                value: if negative { -signed } else { signed },
                exponent: FLOOR + trailing as i32,
            })));
        }
        Ok(Self(Origin::Wide {
            negative,
            magnitude: Box::new(magnitude),
        }))
    }

    fn add(&mut self, value: f64) -> Result<(), &'static str> {
        let term = finite_exact(value)?;
        if term.value == 0 {
            return Ok(());
        }
        if let Origin::Small(previous) = &self.0
            && let Some(sum) = previous.checked_add(term)
        {
            if sum.value == 0 {
                *self = Self::ZERO;
                return Ok(());
            }
            let top =
                sum.exponent - FLOOR + (127 - sum.value.unsigned_abs().leading_zeros()) as i32;
            if top < MAX_BITS as i32 {
                self.0 = Origin::Small(sum);
                return Ok(());
            }
            return Err("circular-integrator origin exceeds the exact history capacity");
        }
        let (negative, mut magnitude) = self.wide();
        let mut other = BigMagnitude::default();
        other.add_shifted(term.value.unsigned_abs(), (term.exponent - FLOOR) as usize);
        let result = if negative == (term.value < 0) {
            for index in 0..other.significant_len() {
                magnitude.add_word(index, other.word(index));
            }
            Self::from_wide(negative, magnitude)?
        } else {
            match magnitude.compare(&other) {
                std::cmp::Ordering::Less => Self::from_wide(!negative, other.subtract(&magnitude))?,
                _ => Self::from_wide(negative, magnitude.subtract(&other))?,
            }
        };
        *self = result;
        Ok(())
    }

    fn remainder(&self, modulus: f64) -> Result<Self, &'static str> {
        let divisor = finite_exact(modulus)?;
        if divisor.value <= 0 {
            return Err("modulus must be finite and greater than zero");
        }
        if let Origin::Small(value) = &self.0 {
            if value.value == 0 {
                return Ok(Self::ZERO);
            }
            let unsigned = value.value.unsigned_abs();
            let difference = value.exponent - divisor.exponent;
            if difference >= 0 {
                let mantissa = divisor.value as u64;
                let residue = (unsigned % u128::from(mantissa)
                    * u128::from(power_of_two_mod(difference as u32, mantissa)))
                    % u128::from(mantissa);
                let residue = if value.value < 0 && residue != 0 {
                    u128::from(mantissa) - residue
                } else {
                    residue
                };
                return Ok(Self(Origin::Small(
                    SmallExact::normalized(residue as i128, divisor.exponent).unwrap(),
                )));
            }
            if let Some(scaled) = u32::try_from(-difference)
                .ok()
                .and_then(|shift| (divisor.value as u128).checked_shl(shift))
                .filter(|&scaled| {
                    scaled >> (-difference as u32) == divisor.value as u128
                        && scaled <= i128::MAX as u128
                })
            {
                let residue = value.value.rem_euclid(scaled as i128);
                return Ok(Self(Origin::Small(
                    SmallExact::normalized(residue, value.exponent).unwrap(),
                )));
            }
        }
        let (negative, magnitude) = self.wide();
        let shift = (divisor.exponent - FLOOR) as usize;
        let mantissa = divisor.value as u64;
        let high_words = magnitude.significant_len().saturating_sub(shift / 64);
        let mut residue = 0_u128;
        for index in (0..high_words).rev() {
            residue = ((residue << 64) | u128::from(shifted_word(&magnitude, index, shift)))
                % u128::from(mantissa);
        }
        let mut remainder = BigMagnitude::default();
        for index in 0..shift / 64 {
            remainder.set_word(index, magnitude.word(index));
        }
        if !shift.is_multiple_of(64) {
            remainder.set_word(
                shift / 64,
                magnitude.word(shift / 64) & ((1_u64 << (shift % 64)) - 1),
            );
        }
        remainder.add_shifted(residue, shift);
        if negative && !remainder.is_zero() {
            let mut whole = BigMagnitude::default();
            whole.add_shifted(u128::from(mantissa), shift);
            remainder = whole.subtract(&remainder);
        }
        Self::from_wide(false, remainder)
    }

    fn rounded(&self) -> Result<f64, &'static str> {
        if let Origin::Small(value) = &self.0
            && let Some(result) = small_exact_ratio_to_f64(
                *value,
                SmallExact {
                    value: 1,
                    exponent: 0,
                },
            )
        {
            return result.map_err(|_| "circular-integrator phase is not finite");
        }
        let (negative, magnitude) = self.wide();
        exact_binary_to_f64(&magnitude, negative, FLOOR)
            .map_err(|_| "circular-integrator phase is not finite")
    }

    /// Fold `self + raw` while retaining phase bits that a rounded total loses.
    pub fn wrapped_value(&self, raw: f64, modulus: f64, offset: f64) -> Result<f64, &'static str> {
        if !raw.is_finite() {
            return Err("integral candidate must be finite");
        }
        let upper = circular_interval(modulus, offset)?;
        if matches!(&self.0, Origin::Small(value) if value.value == 0) {
            if raw >= offset && raw < upper {
                return Ok(raw);
            }
            if offset == 0.0 {
                let phase = raw.rem_euclid(modulus);
                return Ok(if phase >= upper { offset } else { phase });
            }
        }
        let mut total = self.clone();
        total.add(raw)?;
        total.add(-offset)?;
        let mut phase = total.remainder(modulus)?;
        phase.add(offset)?;
        let rounded = phase.rounded()?;
        Ok(if rounded >= upper { offset } else { rounded })
    }

    /// Differentiate the current branch: `dY - floor((Y-offset)/m) * dm`.
    /// `self + phase` retains Y exactly, including translations larger than
    /// binary64. Both terms cancel before the final ratio is rounded.
    pub(crate) fn branch_derivative(
        &self,
        phase: f64,
        modulus: f64,
        offset: f64,
        integral_terms: &[[f64; 2]],
        divisor: f64,
        modulus_derivative: f64,
    ) -> Result<f64, &'static str> {
        circular_interval(modulus, offset)?;
        if !phase.is_finite()
            || !divisor.is_finite()
            || divisor == 0.0
            || !modulus_derivative.is_finite()
        {
            return Err(
                "circular-integrator derivative operands must be finite with a nonzero divisor",
            );
        }
        if modulus_derivative == 0.0 {
            let value = super::sum_products_div(integral_terms, divisor);
            return if value.is_finite() {
                Ok(value)
            } else {
                Err("circular-integrator derivative is not finite")
            };
        }
        let (translation_negative, translation) =
            self.branch_translation(phase, modulus, offset)?;

        let mut numerator =
            scaled_sum_triple_products(integral_terms.iter().map(|&[a, b]| [a, b, modulus]), -3222)
                .map_err(|_| "invalid circular-integrator derivative terms")?;
        let factor = SmallExact::product(modulus_derivative, divisor)
            .ok_or("invalid circular-integrator modulus derivative")?;
        if factor.value != 0 && !translation.is_zero() {
            let shift = (FLOOR + factor.exponent + 3222) as usize;
            let mantissa = factor.value.unsigned_abs();
            let mut action = BigMagnitude::default();
            for index in 0..translation.significant_len() {
                let word = u128::from(translation.word(index));
                action.add_shifted(word * u128::from(mantissa as u64), shift + index * 64);
                action.add_shifted(word * (mantissa >> 64), shift + (index + 1) * 64);
            }
            let negative = !(translation_negative ^ (factor.value < 0));
            if numerator.negative == negative {
                for index in 0..action.significant_len() {
                    numerator.magnitude.add_word(index, action.word(index));
                }
            } else if numerator.magnitude.compare(&action) == std::cmp::Ordering::Less {
                numerator.magnitude = action.subtract(&numerator.magnitude);
                numerator.negative = negative;
            } else {
                numerator.magnitude = numerator.magnitude.subtract(&action);
            }
        }
        if numerator.magnitude.is_zero() {
            return Ok(0.0);
        }
        let denominator = SmallExact::product(divisor, modulus)
            .ok_or("invalid circular-integrator derivative denominator")?;
        let mut magnitude = BigMagnitude::default();
        magnitude.add_shifted(denominator.value.unsigned_abs(), 0);
        scaled_exact_ratio_to_f64(
            &numerator.magnitude,
            &magnitude,
            numerator.negative ^ (denominator.value < 0),
            -3222 - denominator.exponent,
        )
        .map_err(|_| "circular-integrator derivative is not finite")
    }

    fn branch_translation(
        &self,
        phase: f64,
        modulus: f64,
        offset: f64,
    ) -> Result<(bool, BigMagnitude), &'static str> {
        let mut total = self.clone();
        total.add(phase)?;
        total.add(-offset)?;
        let (_, remainder) = total.remainder(modulus)?.wide();
        let (negative, mut translation) = total.wide();
        if negative {
            for index in 0..remainder.significant_len() {
                translation.add_word(index, remainder.word(index));
            }
        } else {
            translation = translation.subtract(&remainder);
        }
        Ok((negative, translation))
    }

    /// Apply the local circular branch to a frequency-domain component without
    /// narrowing its exponent or rounding the wrap count before multiplication.
    #[doc(hidden)]
    pub fn branch_derivative_scaled(
        &self,
        phase: f64,
        modulus: f64,
        offset: f64,
        integral_derivative: super::ScaledValue,
        modulus_derivative: super::ScaledValue,
    ) -> Result<super::ScaledValue, &'static str> {
        use super::ScaledValue;
        circular_interval(modulus, offset)?;
        if !phase.is_finite() || !integral_derivative.is_finite() || !modulus_derivative.is_finite()
        {
            return Err("circular-integrator frequency derivative operands must be finite");
        }
        if modulus_derivative.is_zero() {
            return Ok(integral_derivative);
        }
        let (negative, translation) = self.branch_translation(phase, modulus, offset)?;
        let one = ScaledValue::new(1.0);
        let modulus = ScaledValue::new(modulus);
        let action = if negative {
            modulus_derivative
        } else {
            modulus_derivative.negated()
        };
        // Two 32-bit limbs retain every bit of each word in a binary64 factor.
        let terms = (0..translation.significant_len() * 2).map(|index| {
            let word = translation.word(index / 2);
            let limb = if index % 2 == 0 {
                word as u32
            } else {
                (word >> 32) as u32
            };
            [
                ScaledValue::scaled(f64::from(limb), i64::from(FLOOR) + index as i64 * 32),
                action,
                one,
            ]
        });
        ScaledValue::sum_triple_products_ratio(
            std::iter::once([integral_derivative, modulus, one]).chain(terms),
            [[modulus, one, one]].into_iter(),
        )
        .map_err(|_| "circular-integrator frequency derivative exceeds arithmetic capacity")
    }

    /// Propose the exact new origin after publishing a rounded local phase.
    pub fn rebased(&self, raw: f64, wrapped: f64) -> Result<Self, &'static str> {
        if raw == wrapped && raw.is_finite() {
            return Ok(self.clone());
        }
        let mut candidate = self.clone();
        candidate.add(raw)?;
        candidate.add(-wrapped)?;
        Ok(candidate)
    }

    pub fn checkpoint(&self) -> IdtModOriginCheckpoint {
        let (negative, magnitude) = self.wide();
        let Some(top) = magnitude.top_bit() else {
            return IdtModOriginCheckpoint {
                negative: false,
                exponent: 0,
                words: Vec::new(),
            };
        };
        let first = (0..magnitude.significant_len())
            .find(|&i| magnitude.word(i) != 0)
            .unwrap();
        let trailing = first * 64 + magnitude.word(first).trailing_zeros() as usize;
        IdtModOriginCheckpoint {
            negative,
            exponent: FLOOR + trailing as i32,
            words: (0..=(top - trailing) / 64)
                .map(|i| shifted_word(&magnitude, i, trailing))
                .collect(),
        }
    }

    pub fn from_checkpoint(state: &IdtModOriginCheckpoint) -> Result<Self, &'static str> {
        if state.words.is_empty() {
            return if !state.negative && state.exponent == 0 {
                Ok(Self::default())
            } else {
                Err("noncanonical zero circular-integrator origin")
            };
        }
        if state.words.len() > Self::MAX_CHECKPOINT_WORDS
            || state.exponent < FLOOR
            || state.exponent >= FLOOR + MAX_BITS as i32
            || state.words[0] & 1 == 0
            || state.words.last() == Some(&0)
        {
            return Err("invalid circular-integrator origin checkpoint");
        }
        let shift = (state.exponent - FLOOR) as usize;
        let top = (state.words.len() - 1) * 64 + 63
            - state.words.last().unwrap().leading_zeros() as usize;
        if shift + top >= MAX_BITS {
            return Err("circular-integrator origin exceeds the exact history capacity");
        }
        let mut magnitude = BigMagnitude::default();
        for (index, &word) in state.words.iter().enumerate() {
            magnitude.add_shifted(u128::from(word), shift + index * 64);
        }
        Self::from_wide(state.negative, magnitude)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_origin_trajectories_match_an_independent_dyadic_oracle() {
        for base in [1.0e300, -1.0e300, 2.0_f64.powi(900), -2.0_f64.powi(900)] {
            let mut origin = IdtModOrigin::ZERO.rebased(base, 0.0).unwrap();
            let mut phase = 0.0;
            let mut ticks = 0_i64;
            for step in 0..512 {
                let increment = (step * 17 % 53) - 26;
                ticks += increment;
                let modulus = [0.125, 0.75, 3.0, 13.0][step as usize % 4];
                let offset = [-0.25, 0.0, 0.375][step as usize % 3];
                let raw = phase + increment as f64 / 64.0;
                let expected =
                    offset + ((base % modulus) + ticks as f64 / 64.0 - offset).rem_euclid(modulus);
                phase = origin.wrapped_value(raw, modulus, offset).unwrap();
                assert_eq!(phase, expected, "base={base}, step={step}");
                origin = origin.rebased(raw, phase).unwrap();
                if step % 31 == 0 {
                    origin = IdtModOrigin::from_checkpoint(&origin.checkpoint()).unwrap();
                }
            }
        }
    }

    #[test]
    fn origin_checkpoints_reject_noncanonical_and_unbounded_values() {
        for checkpoint in [
            IdtModOriginCheckpoint {
                negative: true,
                exponent: 0,
                words: vec![],
            },
            IdtModOriginCheckpoint {
                negative: false,
                exponent: 1,
                words: vec![],
            },
            IdtModOriginCheckpoint {
                negative: false,
                exponent: 0,
                words: vec![2],
            },
            IdtModOriginCheckpoint {
                negative: false,
                exponent: 0,
                words: vec![1, 0],
            },
            IdtModOriginCheckpoint {
                negative: false,
                exponent: i32::MIN,
                words: vec![1],
            },
            IdtModOriginCheckpoint {
                negative: false,
                exponent: i32::MAX,
                words: vec![1],
            },
            IdtModOriginCheckpoint {
                negative: false,
                exponent: 3021,
                words: vec![3],
            },
            IdtModOriginCheckpoint {
                negative: false,
                exponent: -1074,
                words: vec![1; 65],
            },
        ] {
            assert!(
                IdtModOrigin::from_checkpoint(&checkpoint).is_err(),
                "{checkpoint:?}"
            );
        }
    }

    #[test]
    fn changing_modulus_preserves_the_original_integral() {
        let origin = IdtModOrigin::default().rebased(5.0, 1.0).unwrap();
        assert_eq!(origin.wrapped_value(1.0, 3.0, 0.0).unwrap(), 2.0);
        let next = origin.rebased(1.0, 2.0).unwrap();
        assert_eq!(next.wrapped_value(2.0, 2.0, 0.0).unwrap(), 1.0);
    }

    #[test]
    fn wide_origins_keep_tiny_phase_and_survive_checkpoints() {
        let origin = IdtModOrigin::default()
            .rebased(1.0e300, 0.0)
            .unwrap()
            .rebased(f64::from_bits(1), 0.0)
            .unwrap();
        assert_eq!(origin.wrapped_value(0.0, 1.0, 0.0).unwrap().to_bits(), 1);
        let restored = IdtModOrigin::from_checkpoint(&origin.checkpoint()).unwrap();
        assert_eq!(origin, restored);
        assert_eq!(restored.wrapped_value(0.25, 1.0, 0.0).unwrap(), 0.25);
        let cancel = origin.rebased(-1.0e300, 0.0).unwrap();
        assert_eq!(cancel.wrapped_value(0.0, 1.0, 0.0).unwrap().to_bits(), 1);
    }

    #[test]
    fn origins_can_exceed_binary64_without_losing_phase() {
        let origin = IdtModOrigin::default()
            .rebased(f64::MAX, 0.0)
            .unwrap()
            .rebased(f64::MAX, 0.0)
            .unwrap();
        assert_eq!(origin.wrapped_value(0.125, 1.0, 0.0).unwrap(), 0.125);
        let checkpoint = origin.checkpoint();
        assert_eq!(IdtModOrigin::from_checkpoint(&checkpoint).unwrap(), origin);
    }
}

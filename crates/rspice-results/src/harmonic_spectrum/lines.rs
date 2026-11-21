//! The lines of a coefficient spectrum and what is read from them: the
//! harmonic each stands for, THD, the intermodulation of two tones and its
//! intercept, and the time waveform the lines add up to.
//!
//! The engine keeps DC as a real value and each line above it as the
//! physical peak-amplitude phasor, so the waveform is
//! `x(t) = c₀ + Σ Re(c_k·e^{j2πf_k t})`.

use super::settings::{CoefficientQuantity, CoefficientSettings};

/// One line of a coefficient spectrum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoefficientLine {
    pub frequency: f64,
    pub re: f64,
    pub im: f64,
}

impl CoefficientLine {
    /// The line's peak amplitude.
    #[must_use]
    pub fn magnitude(&self) -> f64 {
        self.re.hypot(self.im)
    }

    /// The line's phase, in degrees.
    #[must_use]
    pub fn phase_degrees(&self) -> f64 {
        self.im.atan2(self.re).to_degrees()
    }
}

/// How near a line must sit to a frequency, relative to it, to be the line
/// at that frequency: the grid's own rounding, far under any line spacing.
const SAME_FREQUENCY: f64 = 1.0e-9;

fn same_frequency(a: f64, b: f64) -> bool {
    (a - b).abs() <= SAME_FREQUENCY * a.abs().max(b.abs()).max(f64::MIN_POSITIVE)
}

/// The fundamental of a single tone's grid: every line above DC is a
/// harmonic of the lowest, and every harmonic up to the highest is there.
/// The grid of two tones is not one, even when they are commensurate: it
/// holds the mixing products, not every multiple of their spacing.
#[must_use]
pub fn single_tone_fundamental(lines: &[CoefficientLine]) -> Option<f64> {
    let mut positive = lines.iter().map(|line| line.frequency).filter(|f| *f > 0.0);
    let fundamental = positive.next()?;
    let mut order = 1_u32;
    for frequency in positive {
        order += 1;
        if !same_frequency(frequency, f64::from(order) * fundamental) {
            return None;
        }
    }
    Some(fundamental)
}

/// The harmonic order of a line at `frequency` on the grid of `fundamental`.
#[must_use]
pub fn harmonic_order(frequency: f64, fundamental: f64) -> usize {
    // A grid line is a whole multiple of its fundamental to rounding.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let order = (frequency / fundamental).round().max(0.0) as usize;
    order
}

/// Why THD cannot be read off a spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThdAbsence {
    /// The lines are not one tone's harmonics.
    NotOneTone,
    /// The fundamental's amplitude is zero.
    NoFundamental,
    /// The grid keeps no harmonic above the fundamental.
    NoHarmonics,
}

impl ThdAbsence {
    /// The reason, in the few words a details row holds.
    #[must_use]
    pub const fn short(self) -> &'static str {
        match self {
            Self::NotOneTone => "— not one tone",
            Self::NoFundamental => "— no fundamental",
            Self::NoHarmonics => "— no harmonics kept",
        }
    }

    /// The whole reason, for a row's hover.
    #[must_use]
    pub const fn sentence(self) -> &'static str {
        match self {
            Self::NotOneTone => {
                "THD is read against one tone's fundamental. These lines are the mixing products \
                 of several tones; their intermodulation is read instead."
            }
            Self::NoFundamental => "The fundamental's amplitude is zero, so no ratio to it exists.",
            Self::NoHarmonics => {
                "The analysis kept no harmonic above the fundamental. Ask it for more harmonics."
            }
        }
    }
}

/// Total harmonic distortion of one tone's lines, as a ratio: the rms sum of
/// every harmonic kept above the fundamental over the fundamental.
///
/// # Errors
/// The [`ThdAbsence`] that keeps the ratio from being read.
pub fn total_harmonic_distortion(lines: &[CoefficientLine]) -> Result<f64, ThdAbsence> {
    let fundamental = single_tone_fundamental(lines).ok_or(ThdAbsence::NotOneTone)?;
    let carrier = lines
        .iter()
        .find(|line| harmonic_order(line.frequency, fundamental) == 1)
        .map(CoefficientLine::magnitude)
        .filter(|magnitude| *magnitude > 0.0)
        .ok_or(ThdAbsence::NoFundamental)?;
    let harmonics = lines
        .iter()
        .filter(|line| harmonic_order(line.frequency, fundamental) >= 2)
        .map(|line| line.magnitude().powi(2))
        .collect::<Vec<_>>();
    if harmonics.is_empty() {
        return Err(ThdAbsence::NoHarmonics);
    }
    Ok(harmonics.iter().sum::<f64>().sqrt() / carrier)
}

/// The carrier dBc is read against: one tone's fundamental, or the strongest
/// line above DC of several.
#[must_use]
pub fn carrier(lines: &[CoefficientLine]) -> Option<f64> {
    match single_tone_fundamental(lines) {
        Some(fundamental) => lines
            .iter()
            .find(|line| harmonic_order(line.frequency, fundamental) == 1)
            .map(CoefficientLine::magnitude),
        None => lines
            .iter()
            .filter(|line| line.frequency > 0.0)
            .map(CoefficientLine::magnitude)
            .max_by(f64::total_cmp),
    }
    .filter(|magnitude| *magnitude > 0.0)
}

/// One intermodulation product of two tones.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntermodulationProduct {
    /// The product's name: `f2−f1`, `f1+f2`, `2f1−f2`, `2f2−f1`.
    pub name: &'static str,
    /// Its order: 2 or 3.
    pub order: u8,
    pub frequency: f64,
    /// Its peak amplitude.
    pub magnitude: f64,
    /// Its level under the tone beside it, for a third-order product, or
    /// under the stronger tone, for a second-order one, in dB.
    pub dbc: f64,
}

/// The intermodulation of two tones, read off their mixing products.
#[derive(Debug, Clone, PartialEq)]
pub struct Intermodulation {
    /// The tones' lines, lower first.
    pub tones: [CoefficientLine; 2],
    /// Each product the grid keeps.
    pub products: Vec<IntermodulationProduct>,
}

impl Intermodulation {
    /// Read the two tones `tones` names, or with `None` the two strongest
    /// lines above DC, and their second- and third-order products. `None`
    /// for one tone's harmonics, or when either tone is not on the grid.
    #[must_use]
    pub fn read(lines: &[CoefficientLine], tones: Option<[f64; 2]>) -> Option<Self> {
        if single_tone_fundamental(lines).is_some() {
            return None;
        }
        let at = |frequency: f64| {
            lines
                .iter()
                .find(|line| same_frequency(line.frequency, frequency))
                .copied()
        };
        let (low, high) = match tones {
            Some([a, b]) => {
                let (a, b) = (at(a)?, at(b)?);
                if a.frequency <= b.frequency {
                    (a, b)
                } else {
                    (b, a)
                }
            }
            None => {
                let mut strongest = lines
                    .iter()
                    .filter(|line| line.frequency > 0.0 && line.magnitude() > 0.0)
                    .copied()
                    .collect::<Vec<_>>();
                strongest.sort_by(|a, b| b.magnitude().total_cmp(&a.magnitude()));
                let (a, b) = (*strongest.first()?, *strongest.get(1)?);
                if a.frequency <= b.frequency {
                    (a, b)
                } else {
                    (b, a)
                }
            }
        };
        if !(low.magnitude() > 0.0 && high.magnitude() > 0.0)
            || same_frequency(low.frequency, high.frequency)
        {
            return None;
        }
        let (f1, f2) = (low.frequency, high.frequency);
        let stronger = low.magnitude().max(high.magnitude());
        let products = [
            ("f2−f1", 2_u8, f2 - f1, stronger),
            ("f1+f2", 2, f1 + f2, stronger),
            ("2f1−f2", 3, 2.0 * f1 - f2, low.magnitude()),
            ("2f2−f1", 3, 2.0 * f2 - f1, high.magnitude()),
        ]
        .into_iter()
        .filter(|(_, _, frequency, _)| *frequency > 0.0)
        .filter_map(|(name, order, frequency, against)| {
            let line = at(frequency)?;
            Some(IntermodulationProduct {
                name,
                order,
                frequency: line.frequency,
                magnitude: line.magnitude(),
                dbc: 20.0 * (line.magnitude() / against).log10(),
            })
        })
        .collect();
        Some(Self {
            tones: [low, high],
            products,
        })
    }

    /// The output third-order intercept, in dBm into the reference
    /// resistance (dB for a quantity that delivers no power): each tone's
    /// power plus half its height over the third-order product beside it,
    /// the lower of the two sides. `None` when the grid keeps neither
    /// third-order product.
    #[must_use]
    pub fn output_third_order_intercept(
        &self,
        quantity: CoefficientQuantity,
        settings: &CoefficientSettings,
    ) -> Option<f64> {
        let power = |line: &CoefficientLine| match settings.power_dbm(
            line.magnitude(),
            line.frequency,
            quantity,
        ) {
            Some(dbm) => dbm,
            None => 20.0 * line.magnitude().log10(),
        };
        self.products
            .iter()
            .filter(|product| product.order == 3)
            .map(|product| {
                let tone = if product.name == "2f1−f2" {
                    &self.tones[0]
                } else {
                    &self.tones[1]
                };
                power(tone) - product.dbc / 2.0
            })
            .filter(|intercept| intercept.is_finite())
            .min_by(f64::total_cmp)
    }
}

/// How many samples the rebuilt waveform takes per period of its highest
/// line: enough that a line drawn through them shows its shape.
const SAMPLES_PER_HIGHEST_PERIOD: f64 = 16.0;
/// The fewest samples a rebuilt waveform takes, so a waveform of DC and a
/// fundamental alone still draws a smooth curve.
const MIN_REBUILT_SAMPLES: usize = 512;

/// The time waveform `lines` add up to, sampled over `periods` periods of
/// the lowest line above DC (the fundamental of one tone, the beat of two).
/// `None` without a line above DC.
#[must_use]
pub fn rebuild_waveform(lines: &[CoefficientLine], periods: f64) -> Option<(Vec<f64>, Vec<f64>)> {
    let lowest = lines
        .iter()
        .map(|line| line.frequency)
        .filter(|f| *f > 0.0)
        .min_by(f64::total_cmp)?;
    let highest = lines
        .iter()
        .map(|line| line.frequency)
        .max_by(f64::total_cmp)?;
    let span = periods / lowest;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let wanted = (SAMPLES_PER_HIGHEST_PERIOD * highest * span).ceil() as usize;
    let samples = wanted.max(MIN_REBUILT_SAMPLES);
    #[allow(clippy::cast_precision_loss)]
    let step = span / samples as f64;
    let time: Vec<f64> = (0..=samples).map(|n| n as f64 * step).collect();
    let value = time
        .iter()
        .map(|&t| {
            lines
                .iter()
                .map(|line| {
                    if line.frequency > 0.0 {
                        let (sin, cos) = (std::f64::consts::TAU * line.frequency * t).sin_cos();
                        line.re * cos - line.im * sin
                    } else {
                        line.re
                    }
                })
                .sum()
        })
        .collect();
    Some((time, value))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::harmonic_spectrum::settings::CoefficientScale;

    fn line(frequency: f64, magnitude: f64, degrees: f64) -> CoefficientLine {
        let (sin, cos) = degrees.to_radians().sin_cos();
        CoefficientLine {
            frequency,
            re: magnitude * cos,
            im: magnitude * sin,
        }
    }

    /// A 1 MHz tone with harmonics at a tenth, a hundredth and a thousandth
    /// reads THD √(0.1² + 0.01² + 0.001²) and its third harmonic at −40 dBc.
    #[test]
    fn one_tone_reads_its_thd_and_harmonics_in_dbc() {
        let lines = [
            line(0.0, 0.5, 0.0),
            line(1.0e6, 1.0, 0.0),
            line(2.0e6, 0.1, 30.0),
            line(3.0e6, 0.01, -60.0),
            line(4.0e6, 0.001, 90.0),
        ];
        assert_eq!(single_tone_fundamental(&lines), Some(1.0e6));
        let thd = total_harmonic_distortion(&lines).expect("one tone");
        assert!(
            (thd - (0.01_f64 + 1.0e-4 + 1.0e-6).sqrt()).abs() < 1.0e-15,
            "{thd}"
        );
        let dbc = CoefficientSettings {
            scale: CoefficientScale::Dbc,
            ..CoefficientSettings::default()
        };
        let third = dbc.level(
            lines[3].magnitude(),
            3.0e6,
            CoefficientQuantity::Voltage,
            carrier(&lines),
        );
        assert!((third + 40.0).abs() < 1.0e-12, "{third}");
        assert!(Intermodulation::read(&lines, None).is_none());
    }

    /// Two tones at 1.0 and 1.1 MHz, commensurate on a 100 kHz spacing, are
    /// not one tone: THD says so, and the products read their dBc and the
    /// intercept its closed form. Third-order products 60 dB under −10 dBm
    /// tones put OIP3 at 20 dBm.
    #[test]
    fn two_tones_read_their_intermodulation_and_intercept() {
        // 0.1 V peak into 50 Ω is −10 dBm.
        let tone = 0.1;
        let im3 = tone * 1.0e-3;
        let lines = [
            line(0.0, 0.0, 0.0),
            line(0.1e6, 1.0e-4, 0.0),
            line(0.9e6, im3, 0.0),
            line(1.0e6, tone, 0.0),
            line(1.1e6, tone, 0.0),
            line(1.2e6, im3, 0.0),
            line(2.1e6, 2.0e-4, 0.0),
        ];
        assert_eq!(single_tone_fundamental(&lines), None);
        assert_eq!(
            total_harmonic_distortion(&lines),
            Err(ThdAbsence::NotOneTone)
        );
        let im = Intermodulation::read(&lines, None).expect("two tones");
        assert_eq!(im.tones[0].frequency, 1.0e6);
        assert_eq!(im.tones[1].frequency, 1.1e6);
        let names: Vec<&str> = im.products.iter().map(|product| product.name).collect();
        assert_eq!(names, ["f2−f1", "f1+f2", "2f1−f2", "2f2−f1"]);
        assert!((im.products[2].dbc + 60.0).abs() < 1.0e-9);
        let settings = CoefficientSettings::default();
        let oip3 = im
            .output_third_order_intercept(CoefficientQuantity::Voltage, &settings)
            .expect("third-order products");
        assert!((oip3 - 20.0).abs() < 1.0e-9, "{oip3}");
        let named = Intermodulation::read(&lines, Some([1.1e6, 1.0e6])).expect("named tones");
        assert_eq!(named, im);
    }

    /// The rebuilt waveform of DC and a cosine is their sum at every sample.
    #[test]
    fn the_lines_rebuild_their_waveform() {
        let lines = [
            line(0.0, 0.25, 0.0),
            line(1.0e3, 1.0, -90.0),
            line(3.0e3, 0.2, 0.0),
        ];
        let (time, value) = rebuild_waveform(&lines, 2.0).expect("a waveform");
        assert!((time.last().copied().unwrap() - 2.0e-3).abs() < 1.0e-15);
        for (&t, &v) in time.iter().zip(&value) {
            let expected = 0.25
                + (std::f64::consts::TAU * 1.0e3 * t).sin()
                + 0.2 * (std::f64::consts::TAU * 3.0e3 * t).cos();
            assert!(
                (v - expected).abs() < 1.0e-12,
                "{t}: {v} against {expected}"
            );
        }
    }
}

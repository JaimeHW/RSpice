//! How a coefficient spectrum is read: the quantity each line stands for and
//! the level it is drawn at, as ViVA reads an HB or PSS spectrum (voltage,
//! current or power; peak or rms; magnitude, dB or dBm) and ADS draws one in
//! dBm into a stated load.

use serde::{Deserialize, Serialize};

/// What a coefficient spectrum's signal measures, read from its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoefficientQuantity {
    /// A node or branch voltage, `V(…)`.
    Voltage,
    /// A branch or terminal current, `I(…)`.
    Current,
    /// Anything else: a ratio, a derived quantity.
    Other,
}

impl CoefficientQuantity {
    /// The quantity a canonical SPICE signal name measures.
    #[must_use]
    pub fn of(name: &str) -> Self {
        let name = name.trim();
        let starts = |prefix: &str| {
            name.get(..prefix.len())
                .is_some_and(|head| head.eq_ignore_ascii_case(prefix))
        };
        if starts("V(") {
            Self::Voltage
        } else if starts("I(") {
            Self::Current
        } else {
            Self::Other
        }
    }

    /// The unit of the quantity itself.
    #[must_use]
    pub const fn unit(self) -> &'static str {
        match self {
            Self::Voltage => "V",
            Self::Current => "A",
            Self::Other => "",
        }
    }
}

/// The level a line is drawn at.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CoefficientScale {
    /// The amplitude in the quantity's unit.
    Magnitude,
    /// Decibels of one volt or one ampere.
    #[default]
    Db,
    /// The power the line delivers into the reference resistance, in dB of
    /// one milliwatt.
    Dbm,
    /// Decibels under the carrier: the fundamental of one tone, or the
    /// strongest line of several.
    Dbc,
}

impl CoefficientScale {
    pub const ALL: [Self; 4] = [Self::Magnitude, Self::Db, Self::Dbm, Self::Dbc];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Magnitude => "Magnitude",
            Self::Db => "dB",
            Self::Dbm => "dBm",
            Self::Dbc => "dBc",
        }
    }
}

/// Whether an amplitude is read at its peak or as its rms value.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CoefficientAmplitude {
    /// The phasor's peak amplitude, as the engine keeps it.
    #[default]
    Peak,
    /// The rms value: the peak over √2 for a line above DC.
    Rms,
}

impl CoefficientAmplitude {
    pub const ALL: [Self; 2] = [Self::Peak, Self::Rms];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Peak => "Peak",
            Self::Rms => "RMS",
        }
    }
}

/// The resistance dBm is read into when the reader names none, as ADS and
/// ViVA default it.
pub const DEFAULT_REFERENCE_OHMS: f64 = 50.0;

fn default_reference_ohms() -> f64 {
    DEFAULT_REFERENCE_OHMS
}

/// How the reader set a coefficient spectrum's sheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct CoefficientSettings {
    #[serde(default)]
    pub scale: CoefficientScale,
    #[serde(default)]
    pub amplitude: CoefficientAmplitude,
    /// The resistance a voltage or current line delivers its dBm into.
    #[serde(default = "default_reference_ohms")]
    pub reference_ohms: f64,
    /// Each line's phase on a strip of its own.
    #[serde(default)]
    pub phase: bool,
    /// The time waveform the lines add up to, on a strip of its own.
    #[serde(default)]
    pub waveform: bool,
}

impl Default for CoefficientSettings {
    fn default() -> Self {
        Self {
            scale: CoefficientScale::default(),
            amplitude: CoefficientAmplitude::default(),
            reference_ohms: DEFAULT_REFERENCE_OHMS,
            phase: false,
            waveform: false,
        }
    }
}

impl CoefficientSettings {
    /// Why the settings cannot stand, if they cannot.
    ///
    /// # Errors
    /// A reference resistance that is not a positive finite number.
    pub fn validate(&self) -> Result<(), String> {
        if self.reference_ohms.is_finite() && self.reference_ohms > 0.0 {
            Ok(())
        } else {
            Err(format!(
                "the reference resistance must be a positive number of ohms, not {}",
                self.reference_ohms
            ))
        }
    }

    /// The unit `quantity`'s lines are drawn in.
    #[must_use]
    pub const fn unit(&self, quantity: CoefficientQuantity) -> &'static str {
        match (self.scale, quantity) {
            (CoefficientScale::Magnitude, quantity) => quantity.unit(),
            (CoefficientScale::Db, CoefficientQuantity::Voltage) => "dBV",
            // Not "dBA", which every other instrument that prints it means
            // as an A-weighted sound level.
            (CoefficientScale::Db, CoefficientQuantity::Current) => "dB re 1 A",
            (CoefficientScale::Db | CoefficientScale::Dbm, CoefficientQuantity::Other) => "dB",
            (CoefficientScale::Dbm, _) => "dBm",
            (CoefficientScale::Dbc, _) => "dBc",
        }
    }

    /// The level of a line of peak amplitude `peak` at `frequency`, against
    /// the carrier's peak amplitude `carrier` for dBc.
    ///
    /// DC has no peak distinct from its value, so its rms value is itself.
    /// dBm is a power, and a power is read from the rms value whatever the
    /// amplitude convention: `V²/R` for a voltage, `I²R` for a current.
    #[must_use]
    pub fn level(
        &self,
        peak: f64,
        frequency: f64,
        quantity: CoefficientQuantity,
        carrier: Option<f64>,
    ) -> f64 {
        let rms = if frequency > 0.0 {
            peak * std::f64::consts::FRAC_1_SQRT_2
        } else {
            peak
        };
        let amplitude = match self.amplitude {
            CoefficientAmplitude::Peak => peak,
            CoefficientAmplitude::Rms => rms,
        };
        match (self.scale, quantity) {
            (CoefficientScale::Magnitude, _) => amplitude,
            (CoefficientScale::Db, _) | (CoefficientScale::Dbm, CoefficientQuantity::Other) => {
                20.0 * amplitude.log10()
            }
            (CoefficientScale::Dbm, CoefficientQuantity::Voltage) => {
                10.0 * (rms * rms / self.reference_ohms / 1.0e-3).log10()
            }
            (CoefficientScale::Dbm, CoefficientQuantity::Current) => {
                10.0 * (rms * rms * self.reference_ohms / 1.0e-3).log10()
            }
            (CoefficientScale::Dbc, _) => {
                carrier.map_or(f64::NAN, |carrier| 20.0 * (peak / carrier).log10())
            }
        }
    }

    /// The power of a line of peak amplitude `peak` into the reference
    /// resistance, in dBm; `None` for a quantity that delivers no power.
    #[must_use]
    pub fn power_dbm(
        &self,
        peak: f64,
        frequency: f64,
        quantity: CoefficientQuantity,
    ) -> Option<f64> {
        (quantity != CoefficientQuantity::Other).then(|| {
            Self {
                scale: CoefficientScale::Dbm,
                ..self.clone()
            }
            .level(peak, frequency, quantity, None)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One volt peak into 50 Ω is 10 mW, 10 dBm; one ampere peak through it
    /// is 25 W, 43.98 dBm. A dB level reads the convention asked for; a dBc
    /// level reads the ratio of peaks.
    #[test]
    fn levels_read_the_convention_asked_for() {
        let settings = CoefficientSettings {
            scale: CoefficientScale::Dbm,
            ..CoefficientSettings::default()
        };
        let voltage = settings.level(1.0, 1.0e6, CoefficientQuantity::Voltage, None);
        assert!((voltage - 10.0).abs() < 1.0e-12, "{voltage}");
        let current = settings.level(1.0, 1.0e6, CoefficientQuantity::Current, None);
        assert!(
            (current - 10.0 * 25.0e3_f64.log10()).abs() < 1.0e-12,
            "{current}"
        );

        let rms = CoefficientSettings {
            amplitude: CoefficientAmplitude::Rms,
            ..CoefficientSettings::default()
        };
        let level = rms.level(1.0, 1.0e6, CoefficientQuantity::Voltage, None);
        assert!((level + 3.010_299_956_639_812).abs() < 1.0e-12, "{level}");
        assert_eq!(
            rms.level(2.0, 0.0, CoefficientQuantity::Voltage, None),
            20.0 * 2.0_f64.log10()
        );

        let dbc = CoefficientSettings {
            scale: CoefficientScale::Dbc,
            ..CoefficientSettings::default()
        };
        let level = dbc.level(0.01, 3.0e6, CoefficientQuantity::Voltage, Some(1.0));
        assert!((level + 40.0).abs() < 1.0e-12, "{level}");
        assert_eq!(dbc.unit(CoefficientQuantity::Current), "dBc");
    }

    #[test]
    fn a_reference_resistance_is_a_positive_number_of_ohms() {
        for ohms in [0.0, -50.0, f64::NAN, f64::INFINITY] {
            let settings = CoefficientSettings {
                reference_ohms: ohms,
                ..CoefficientSettings::default()
            };
            assert!(settings.validate().is_err(), "{ohms}");
        }
        assert!(CoefficientSettings::default().validate().is_ok());
    }
}

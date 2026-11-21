//! How the reader set a spectrum sheet, as a result view keeps it: the
//! signal it transforms, the record and its samples, the window, and what
//! its figures are measured over. These are the choices ViVA's spectrum
//! assistant and Eldo's FFT offer.

use serde::{Deserialize, Serialize};

use super::window::{KaiserBeta, WindowFunction};

/// The window kinds a spectrum sheet offers; the Kaiser window's β is kept
/// beside it, so choosing another window and coming back keeps the β.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpectrumWindow {
    Rectangular,
    #[default]
    Hanning,
    Hamming,
    Blackman,
    BlackmanHarris,
    FlatTop,
    Kaiser,
    Gaussian,
}

impl SpectrumWindow {
    /// Every kind, in the order a menu lists them.
    pub const ALL: [Self; 8] = [
        Self::Rectangular,
        Self::Hanning,
        Self::Hamming,
        Self::Blackman,
        Self::BlackmanHarris,
        Self::FlatTop,
        Self::Kaiser,
        Self::Gaussian,
    ];

    /// The window this kind is at `beta`.
    #[must_use]
    pub fn with_beta(self, beta: KaiserBeta) -> WindowFunction {
        match self {
            Self::Rectangular => WindowFunction::Rectangular,
            Self::Hanning => WindowFunction::Hanning,
            Self::Hamming => WindowFunction::Hamming,
            Self::Blackman => WindowFunction::Blackman,
            Self::BlackmanHarris => WindowFunction::BlackmanHarris,
            Self::FlatTop => WindowFunction::FlatTop,
            Self::Kaiser => WindowFunction::Kaiser(beta),
            Self::Gaussian => WindowFunction::Gaussian,
        }
    }

    /// The kind as the menu names it.
    #[must_use]
    pub fn label(self) -> &'static str {
        self.with_beta(KaiserBeta::DEFAULT).display_name()
    }
}

/// The amplitude a spectrum's levels read: a tone's peak or its rms.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpectrumAmplitude {
    Peak,
    #[default]
    Rms,
}

impl SpectrumAmplitude {
    pub const ALL: [Self; 2] = [Self::Peak, Self::Rms];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Peak => "Peak",
            Self::Rms => "RMS",
        }
    }
}

/// What the spectrum's ordinate draws.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpectrumScale {
    /// Level in dB of the signal's unit.
    #[default]
    Db,
    /// Magnitude in the signal's unit.
    Magnitude,
    /// Power spectral density, in dB of the unit squared per hertz.
    Psd,
}

impl SpectrumScale {
    pub const ALL: [Self; 3] = [Self::Db, Self::Magnitude, Self::Psd];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Db => "dB",
            Self::Magnitude => "Magnitude",
            Self::Psd => "PSD",
        }
    }
}

/// How many samples the transform takes from its record.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpectrumSampling {
    /// As many as the record holds inside its window.
    #[default]
    Auto,
    /// This many.
    Count(usize),
    /// At this rate, in hertz: the record's length times the rate.
    Rate(f64),
}

/// How samples off the record's own steps are made.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpectrumResampling {
    #[default]
    Interpolate,
    SampleAndHold,
}

/// The smallest and largest harmonic count a sheet measures through.
pub const SPECTRUM_HARMONICS: std::ops::RangeInclusive<usize> = 2..=200;
/// The zero-padding factors a sheet offers.
pub const SPECTRUM_PADDING: [u32; 4] = [1, 2, 4, 8];
/// The most records a sheet averages.
pub const MAX_SPECTRUM_AVERAGES: u32 = 64;

/// How the reader set one spectrum sheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SpectrumSettings {
    /// The signal or expression transformed; `None` is the first plotted
    /// voltage.
    pub source: Option<String>,
    pub window: SpectrumWindow,
    /// The Kaiser window's β.
    pub kaiser_beta: f64,
    pub amplitude: SpectrumAmplitude,
    /// The record, `[start, stop)` in seconds; `None` is the whole record.
    pub record: Option<(f64, f64)>,
    pub sampling: SpectrumSampling,
    pub resampling: SpectrumResampling,
    /// The transform's length over the record's: zeros appended to draw the
    /// spectrum on a finer grid.
    pub padding: u32,
    /// Records of equal length the record is cut into, whose powers are
    /// averaged.
    pub averages: u32,
    /// The harmonic orders THD and SINAD count, HD2 through this one.
    pub harmonics: usize,
    /// The bins either side of a tone claimed for it; `None` is the window's
    /// main lobe.
    pub signal_bins: Option<usize>,
    /// The fundamental, in hertz; `None` is the largest tone.
    pub fundamental: Option<f64>,
    /// The band, in hertz, the figures are measured over; `None` is DC to
    /// Nyquist.
    pub band: Option<(f64, f64)>,
    pub scale: SpectrumScale,
    /// Frequency drawn on a logarithmic axis.
    pub log_frequency: bool,
    /// Phase drawn under the level.
    pub phase: bool,
}

impl Default for SpectrumSettings {
    fn default() -> Self {
        Self {
            source: None,
            window: SpectrumWindow::Hanning,
            kaiser_beta: KaiserBeta::DEFAULT.get(),
            amplitude: SpectrumAmplitude::Rms,
            record: None,
            sampling: SpectrumSampling::Auto,
            resampling: SpectrumResampling::Interpolate,
            padding: 1,
            averages: 1,
            harmonics: 10,
            signal_bins: None,
            fundamental: None,
            band: None,
            scale: SpectrumScale::Db,
            log_frequency: false,
            phase: false,
        }
    }
}

impl SpectrumSettings {
    /// The window the settings name, at their β.
    #[must_use]
    pub fn window_function(&self) -> WindowFunction {
        self.window
            .with_beta(KaiserBeta::new(self.kaiser_beta).unwrap_or_default())
    }

    /// Whether `other` transforms the same record the same way, so a
    /// prepared record serves both: the same source, record, samples and
    /// resampling.
    #[must_use]
    pub fn same_record(&self, other: &Self) -> bool {
        self.source == other.source
            && self.record == other.record
            && self.sampling == other.sampling
            && self.resampling == other.resampling
    }

    /// Why the settings cannot be kept, when they cannot.
    pub fn validate(&self) -> Result<(), String> {
        if let Some(source) = &self.source
            && source.trim().is_empty()
        {
            return Err("the spectrum's source is empty".to_owned());
        }
        if KaiserBeta::new(self.kaiser_beta).is_none() {
            return Err(format!(
                "the Kaiser window's beta must be between 0 and {}, not {}",
                KaiserBeta::MAX,
                self.kaiser_beta
            ));
        }
        if let Some((start, stop)) = self.record
            && !(start.is_finite() && stop.is_finite() && stop > start)
        {
            return Err(format!("the record {start} to {stop} is empty"));
        }
        match self.sampling {
            SpectrumSampling::Count(0) => return Err("the sample count is zero".to_owned()),
            SpectrumSampling::Rate(rate) if !(rate.is_finite() && rate > 0.0) => {
                return Err(format!("the sample rate {rate} is not a positive rate"));
            }
            _ => {}
        }
        if !SPECTRUM_PADDING.contains(&self.padding) {
            return Err(format!("padding by {} is not offered", self.padding));
        }
        if !(1..=MAX_SPECTRUM_AVERAGES).contains(&self.averages) {
            return Err(format!(
                "averaging {} records is not offered",
                self.averages
            ));
        }
        if !SPECTRUM_HARMONICS.contains(&self.harmonics) {
            return Err(format!(
                "the harmonic count {} is outside {} to {}",
                self.harmonics,
                SPECTRUM_HARMONICS.start(),
                SPECTRUM_HARMONICS.end()
            ));
        }
        if let Some(fundamental) = self.fundamental
            && !(fundamental.is_finite() && fundamental > 0.0)
        {
            return Err(format!("the fundamental {fundamental} is not a frequency"));
        }
        if let Some((low, high)) = self.band
            && !(low.is_finite() && high.is_finite() && low >= 0.0 && high > low)
        {
            return Err(format!("the band {low} to {high} is empty"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A view keeps the settings in its stored form, and reads back what it
    /// wrote.
    #[test]
    fn settings_round_trip_through_their_stored_form() {
        let settings = SpectrumSettings {
            source: Some("V(out)".to_owned()),
            window: SpectrumWindow::Kaiser,
            kaiser_beta: 12.5,
            record: Some((1.0e-3, 2.0e-3)),
            sampling: SpectrumSampling::Rate(1.0e6),
            band: Some((0.0, 20.0e3)),
            ..SpectrumSettings::default()
        };
        settings.validate().expect("valid settings");
        let text = serde_json::to_string(&settings).expect("stored");
        let read: SpectrumSettings = serde_json::from_str(&text).expect("read back");
        assert_eq!(read, settings);
        assert_eq!(
            read.window_function(),
            WindowFunction::Kaiser(KaiserBeta::new(12.5).unwrap())
        );
    }

    #[test]
    fn settings_that_name_no_spectrum_are_refused() {
        for settings in [
            SpectrumSettings {
                kaiser_beta: -1.0,
                ..SpectrumSettings::default()
            },
            SpectrumSettings {
                record: Some((2.0, 1.0)),
                ..SpectrumSettings::default()
            },
            SpectrumSettings {
                sampling: SpectrumSampling::Count(0),
                ..SpectrumSettings::default()
            },
            SpectrumSettings {
                padding: 3,
                ..SpectrumSettings::default()
            },
            SpectrumSettings {
                harmonics: 1,
                ..SpectrumSettings::default()
            },
        ] {
            assert!(settings.validate().is_err(), "{settings:?}");
        }
    }
}

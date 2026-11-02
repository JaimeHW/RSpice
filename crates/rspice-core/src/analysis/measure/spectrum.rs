//! Xyce's `.MEASURE FFT` statistics: the ENOB, SFDR, SNR, SNDR and THD of
//! the spectrum of the last `.FFT` of the measured output
//! (N_IO_MeasureFFT.C, N_IO_FFTAnalysis.C:801-990).

use super::MeasureResult;
use crate::Value;
use crate::analysis::transient::TransientFftResult;
use crate::netlist::measure::{FftMetric, MeasureStatement, MeasureType};

/// Xyce's `.FFT` noise floor (N_IO_FFTAnalysis.C:95): SNR reports its
/// reciprocal when no bin counts as noise, and THD in dB never drops below
/// it.
const NOISE_FLOOR: Value = 1.0e-10;

/// The value of one `.MEASURE FFT` statement over the run's spectra.
pub(super) fn evaluate(
    statement: &MeasureStatement,
    spectra: &[TransientFftResult],
) -> MeasureResult {
    let name = &statement.name;
    let MeasureType::FftMetric {
        metric,
        signal,
        bin_size,
        min_freq,
        max_freq,
        harmonics,
    } = &statement.measure_type
    else {
        return MeasureResult::failed(
            name,
            "a .MEASURE FFT reads ENOB, SFDR, SNR, SNDR or THD, or a PARAM or EQN of them",
        );
    };
    // fixupFFTMeasures binds the last `.FFT` whose output is the measure's.
    let Some(spectrum) = spectra
        .iter()
        .rev()
        .find(|spectrum| same_output(&spectrum.output_name, signal))
    else {
        return MeasureResult::failed(name, &format!("no .FFT statement has the output {signal}"));
    };
    if let Err(error) = spectrum.validate_status() {
        return MeasureResult::failed(name, &error);
    }
    // Xyce's `mag_` is indexed by bin, DC first.
    let mut magnitudes = vec![
        0.0;
        spectrum
            .bins
            .iter()
            .map(|bin| bin.index + 1)
            .max()
            .unwrap_or(0)
    ];
    for bin in &spectrum.bins {
        magnitudes[bin.index] = bin.magnitude;
    }
    let bins = Bins {
        magnitudes: &magnitudes,
        first_harmonic: spectrum.fundamental_bin.max(1) as i64,
        points: spectrum.point_count as i64,
    };
    // fixupFFTMeasure: a frequency names its nearest bin, and MAXFREQ
    // defaults to half the points.
    let bin = |frequency: Value| (frequency / spectrum.frequency_resolution).round() as i64;
    let max_index = max_freq.map_or(bins.points / 2, bin);
    let value = match metric {
        FftMetric::Enob => bins.enob(*bin_size as i64),
        FftMetric::Sfdr => bins.sfdr(min_freq.map(bin), max_index, *bin_size as i64),
        FftMetric::Snr => bins.snr(max_index),
        FftMetric::Sndr => bins.sndr(*bin_size as i64),
        FftMetric::Thd => bins.thd_db(*harmonics, max_index),
    };
    MeasureResult::success(name, value)
}

/// Whether a `.FFT` output and a measure's operand name the same signal.
fn same_output(output: &str, signal: &str) -> bool {
    let squeeze = |text: &str| {
        text.chars()
            .filter(|character| !character.is_whitespace())
            .map(|character| character.to_ascii_uppercase())
            .collect::<String>()
    };
    squeeze(output) == squeeze(signal)
}

/// A spectrum's magnitudes from DC up, the bin of its first harmonic and
/// its point count, indexed as Xyce's FFTAnalysis indexes `mag_`.
struct Bins<'a> {
    magnitudes: &'a [Value],
    first_harmonic: i64,
    points: i64,
}

impl Bins<'_> {
    fn magnitude(&self, index: i64) -> Value {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.magnitudes.get(index))
            .copied()
            .unwrap_or(0.0)
    }

    fn signal(&self) -> Value {
        self.magnitude(self.first_harmonic)
    }

    fn within_signal_bins(&self, index: i64, bin_size: i64) -> bool {
        index >= self.first_harmonic - bin_size && index <= self.first_harmonic + bin_size
    }

    /// calculateSNDRforMeasFFT: the bins within `bin_size` of the first
    /// harmonic over every other bin above DC.
    fn sndr(&self, bin_size: i64) -> Value {
        let (mut signal, mut noise) = (0.0, 0.0);
        for index in 1..=self.points / 2 {
            let power = self.magnitude(index).powi(2);
            if self.within_signal_bins(index, bin_size) {
                signal += power;
            } else {
                noise += power;
            }
        }
        20.0 * (signal / noise).sqrt().log10()
    }

    /// calculateENOBforMeasFFT.
    fn enob(&self, bin_size: i64) -> Value {
        (self.sndr(bin_size) - 1.76) / 6.02
    }

    /// calculateSFDRforMeasFFT and getLowerLimforSFDR_: without MINFREQ the
    /// search starts at the first harmonic when MAXFREQ reaches it, and at
    /// the first bin otherwise.
    fn sfdr(&self, min_index: Option<i64>, max_index: i64, bin_size: i64) -> Value {
        let lower = match min_index {
            Some(index) => index,
            None if max_index >= self.first_harmonic => self.first_harmonic,
            None => 1,
        };
        let spur = (lower..=max_index.min(self.points / 2))
            .filter(|index| !self.within_signal_bins(*index, bin_size))
            .map(|index| self.magnitude(index))
            .fold(0.0, Value::max);
        20.0 * (self.signal() / spur).log10()
    }

    /// calculateSNR: the first harmonic over the bins that are neither its
    /// harmonics nor below `max_index`.
    fn snr(&self, max_index: i64) -> Value {
        let limit = max_index.max(self.first_harmonic);
        let noise = (1..=self.points / 2)
            .filter(|index| index % self.first_harmonic != 0 || *index > limit)
            .map(|index| self.magnitude(index).powi(2))
            .collect::<Vec<_>>();
        let ratio = if noise.is_empty() {
            1.0 / NOISE_FLOOR
        } else {
            self.signal() / noise.iter().sum::<Value>().sqrt()
        };
        20.0 * ratio.log10()
    }

    /// THD::getMeasureResult and calculateTHD: the harmonics up to
    /// `NBHARM` of them, or to `max_index` without it, over the first
    /// harmonic, in dB above the noise floor.
    fn thd_db(&self, harmonics: Option<i64>, max_index: i64) -> Value {
        let limit = match harmonics {
            None => max_index,
            Some(count) if count <= 0 || 2 * count * self.first_harmonic >= self.points => {
                self.points / 2
            }
            Some(count) => count * self.first_harmonic,
        };
        let power = (2..)
            .map(|order| order * self.first_harmonic)
            .take_while(|index| *index <= limit.min(self.points / 2))
            .map(|index| self.magnitude(index).powi(2))
            .sum::<Value>();
        let ratio = power.sqrt() / self.signal();
        20.0 * ratio.max(NOISE_FLOOR).log10()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Xyce 7.10's own report for a `.FFT V(4) NP=16 WINDOW=HANN FREQ=2` of
    /// a constant (MEASURE_FFT ENOBTest): the normalized magnitudes it
    /// prints, with the first harmonic in bin 2, give its THD, SNDR, ENOB,
    /// SNR and SFDR lines.
    #[test]
    fn a_second_bin_first_harmonic_gives_xyces_figures() {
        let magnitudes = [
            9.140524e-01,
            1.0,
            4.715437e-02,
            1.650298e-02,
            8.155536e-03,
            4.516010e-03,
            2.487983e-03,
            1.118726e-03,
            0.0,
        ];
        let bins = Bins {
            magnitudes: &magnitudes,
            first_harmonic: 2,
            points: 16,
        };
        let close = |value: Value, xyce: Value| (value - xyce).abs() <= 1.0e-5 * xyce.abs();
        assert!(
            close(bins.thd_db(None, 8), -1.485493e1),
            "{}",
            bins.thd_db(None, 8)
        );
        assert!(close(bins.sndr(0), -2.653115e1), "{}", bins.sndr(0));
        assert!(close(bins.enob(0), -4.699527), "{}", bins.enob(0));
        assert!(close(bins.snr(8), -2.653084e1), "{}", bins.snr(8));
        assert!(
            close(bins.sfdr(None, 8, 0), 9.119191),
            "{}",
            bins.sfdr(None, 8, 0)
        );
        // NBHARM=1 counts no harmonic, so THD sits on the noise floor; a
        // MAXFREQ at the first harmonic makes every other bin noise.
        assert!(close(bins.thd_db(Some(1), 8), -200.0));
        assert!(close(bins.snr(2), bins.sndr(0)));
    }
}

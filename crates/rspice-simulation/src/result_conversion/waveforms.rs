//! Exact shared axes, complex projections, noise ordering and statistical samples.

use super::ResultConversion;
use rspice_results::family_metadata::MonteCarloVariableMetadata;
use rspice_results::waveform::WaveformValues;
use std::collections::HashMap;

type OrderedNoiseSeries = (
    Vec<f64>,
    Vec<f64>,
    Option<Vec<f64>>,
    HashMap<String, Vec<f64>>,
);

impl<Clock: Fn() -> f64> ResultConversion<Clock> {
    pub(super) fn build_waveforms_with_shared_x_owned(
        &self,
        x_values: WaveformValues,
        waveforms: HashMap<String, crate::results::SampledWaveform>,
    ) -> Vec<rspice_results::waveform::RetainedWaveform> {
        self.build_sorted_waveforms_with_shared_x_owned(x_values, waveforms, |name, waveform| {
            (name, waveform.y_values)
        })
    }

    pub(super) fn build_time_waveforms_owned(
        &self,
        time: WaveformValues,
        waveforms: HashMap<String, crate::results::SampledWaveform>,
    ) -> Vec<rspice_results::waveform::RetainedWaveform> {
        let shared_time = time;
        let sample_count = shared_time.len();
        let mut waveforms = waveforms.into_iter().collect::<Vec<_>>();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));
        let mut results = Vec::new();

        for (name, waveform) in waveforms {
            let unit = waveform.y_unit;
            let real = waveform.y_values;
            if !Self::samples_match_shared_axis(real.len(), sample_count) {
                continue;
            }
            if let Some(imag) = waveform.y_imag {
                if !Self::samples_match_shared_axis(imag.len(), sample_count) {
                    continue;
                }
                let magnitude = real
                    .iter()
                    .zip(imag.iter())
                    .map(|(real, imag)| real.hypot(*imag))
                    .collect::<Vec<_>>();
                let phase = real
                    .iter()
                    .zip(imag.iter())
                    .map(|(real, imag)| imag.atan2(*real).to_degrees())
                    .collect::<Vec<_>>();
                results.push(
                    rspice_results::waveform::RetainedWaveform::new(
                        format!("|{name}|"),
                        shared_time.clone(),
                        magnitude,
                    )
                    .with_unit(unit)
                    .with_complex_components(name.clone(), real, imag),
                );
                results.push(
                    rspice_results::waveform::RetainedWaveform::new(
                        format!("phase({name})"),
                        shared_time.clone(),
                        phase,
                    )
                    .with_unit("°"),
                );
            } else {
                results.push(
                    rspice_results::waveform::RetainedWaveform::new(
                        name,
                        shared_time.clone(),
                        real,
                    )
                    .with_unit(unit),
                );
            }
        }
        results
    }

    pub(super) fn build_noise_waveforms_owned(
        &self,
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
        input_quantity: Option<rspice_core::analysis::noise::NoiseInputQuantity>,
    ) -> Vec<rspice_results::waveform::RetainedWaveform> {
        let (frequencies, output_noise, input_noise, contributors) =
            Self::order_noise_series_for_retention(
                frequencies,
                output_noise,
                input_noise,
                contributors,
            );
        let shared_freqs = WaveformValues::from(frequencies);
        let freq_len = shared_freqs.len();
        let mut results = Vec::new();

        // The caller attaches the producer's explicit output unit. Historical
        // data may lack it because this vector also carries dBc/Hz phase noise.
        if Self::samples_match_shared_axis(output_noise.len(), freq_len) {
            results.push(rspice_results::waveform::RetainedWaveform::new(
                "onoise".to_string(),
                shared_freqs.clone(),
                output_noise,
            ));
        }

        if let Some(inoise) = input_noise
            && Self::samples_match_shared_axis(inoise.len(), freq_len)
        {
            let mut waveform = rspice_results::waveform::RetainedWaveform::new(
                "inoise".to_string(),
                shared_freqs.clone(),
                inoise,
            );
            if let Some(quantity) = input_quantity {
                waveform = waveform.with_unit(quantity.density_unit());
            }
            results.push(waveform);
        }

        let mut contributors: Vec<_> = contributors.into_iter().collect();
        contributors.sort_by(|a, b| a.0.cmp(&b.0));
        for (source, values) in contributors {
            if !Self::samples_match_shared_axis(values.len(), freq_len) {
                continue;
            }
            results.push(rspice_results::waveform::RetainedWaveform::new(
                format!("noise({})", source),
                shared_freqs.clone(),
                values,
            ));
        }

        results
    }

    fn order_noise_series_for_retention(
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
    ) -> OrderedNoiseSeries {
        if frequencies
            .windows(2)
            .all(|pair| pair[0].total_cmp(&pair[1]) != std::cmp::Ordering::Greater)
        {
            return (frequencies, output_noise, input_noise, contributors);
        }

        let mut order = (0..frequencies.len()).collect::<Vec<_>>();
        order.sort_by(|left, right| {
            frequencies[*left]
                .total_cmp(&frequencies[*right])
                .then_with(|| left.cmp(right))
        });
        let frequencies = Self::permute_noise_samples(frequencies, &order);
        let output_noise = Self::permute_noise_samples_if_aligned(output_noise, &order);
        let input_noise =
            input_noise.map(|samples| Self::permute_noise_samples_if_aligned(samples, &order));
        let contributors = contributors
            .into_iter()
            .map(|(name, samples)| {
                (
                    name,
                    Self::permute_noise_samples_if_aligned(samples, &order),
                )
            })
            .collect();

        (frequencies, output_noise, input_noise, contributors)
    }

    fn permute_noise_samples(samples: Vec<f64>, order: &[usize]) -> Vec<f64> {
        order.iter().map(|index| samples[*index]).collect()
    }

    fn permute_noise_samples_if_aligned(samples: Vec<f64>, order: &[usize]) -> Vec<f64> {
        if samples.len() == order.len() {
            Self::permute_noise_samples(samples, order)
        } else {
            samples
        }
    }

    pub(super) fn build_monte_carlo_payload_owned(
        &self,
        variables: Vec<crate::results::MonteCarloVariableResult>,
    ) -> (
        Vec<rspice_results::waveform::RetainedWaveform>,
        Vec<MonteCarloVariableMetadata>,
    ) {
        let mut waveforms = Vec::with_capacity(variables.len());
        let mut metadata = Vec::with_capacity(variables.len());
        for variable in variables {
            let crate::results::MonteCarloVariableResult {
                mean_confidence,
                name,
                samples,
                mean,
                std_dev,
                min,
                max,
                histogram,
                bin_edges,
            } = variable;
            if !histogram.is_empty() && bin_edges.len() == histogram.len().saturating_add(1) {
                let x: Vec<f64> = bin_edges
                    .windows(2)
                    .map(|window| window[0].midpoint(window[1]))
                    .collect();
                let y: Vec<f64> = histogram.into_iter().map(|count| count as f64).collect();
                waveforms.push(rspice_results::waveform::RetainedWaveform::new(
                    format!("hist({name})"),
                    x,
                    y,
                ));
            }
            metadata.push(MonteCarloVariableMetadata {
                mean_confidence,
                name,
                samples,
                mean,
                std_dev,
                min,
                max,
            });
        }
        (waveforms, metadata)
    }

    pub(super) fn build_ac_waveforms_owned(
        &self,
        frequencies: Vec<f64>,
        waveforms: HashMap<String, crate::results::SampledWaveform>,
    ) -> Vec<rspice_results::waveform::RetainedWaveform> {
        let shared_freqs = WaveformValues::from(frequencies);
        let freq_len = shared_freqs.len();
        let mut waveforms: Vec<_> = waveforms.into_iter().collect();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));

        let mut results = Vec::new();
        for (name, waveform) in waveforms {
            // A magnitude is the modulus of the source quantity, so it reads
            // in the source's own unit. The phase projection does not: it is
            // produced here, in degrees, and says so itself.
            let unit = waveform.y_unit;
            let waveform_x = waveform.x_values;
            let real = waveform.y_values;
            let x = if Self::samples_match_shared_axis(real.len(), waveform_x.len()) {
                waveform_x
            } else if Self::samples_match_shared_axis(real.len(), freq_len) {
                shared_freqs.clone()
            } else {
                continue;
            };
            match waveform.y_imag {
                Some(imag) => {
                    if !Self::samples_match_shared_axis(imag.len(), x.len()) {
                        continue;
                    }
                    let magnitude_values: Vec<f64> = real
                        .iter()
                        .zip(imag.iter())
                        .map(|(r, i)| r.hypot(*i))
                        .collect();
                    let phase = real
                        .iter()
                        .zip(imag.iter())
                        .map(|(r, i)| i.atan2(*r).to_degrees())
                        .collect::<Vec<_>>();

                    let magnitude = rspice_results::waveform::RetainedWaveform::new(
                        format!("|{}|", name),
                        x.clone(),
                        magnitude_values,
                    )
                    .with_unit(unit)
                    .with_complex_components(name.clone(), real, imag);
                    results.push(magnitude);

                    results.push(
                        rspice_results::waveform::RetainedWaveform::new(
                            format!("phase({})", name),
                            x.clone(),
                            phase,
                        )
                        .with_unit("°"),
                    );
                }
                None => {
                    // A real-valued frequency-domain quantity is not a
                    // complex magnitude. Preserve its producer name and unit
                    // so group delay, stability margin, and Floquet-mode
                    // metrics are never wrapped in |...| or converted to dB
                    // by the viewer.
                    results.push(
                        rspice_results::waveform::RetainedWaveform::new(name, x, real)
                            .with_unit(unit),
                    );
                }
            }
        }

        results
    }

    pub(super) fn build_sorted_waveforms_with_shared_x_owned<F>(
        &self,
        x_values: WaveformValues,
        waveforms: HashMap<String, crate::results::SampledWaveform>,
        value_mapper: F,
    ) -> Vec<rspice_results::waveform::RetainedWaveform>
    where
        F: Fn(String, crate::results::SampledWaveform) -> (String, WaveformValues),
    {
        let shared_x = x_values;
        let mut waveforms: Vec<_> = waveforms.into_iter().collect();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));

        let mut results = Vec::new();
        for (name, waveform) in waveforms {
            // Read the stated unit before the mapper consumes the waveform:
            // it is the producer's, and nothing downstream can recover it
            // from the samples or the name.
            let unit = waveform.y_unit.clone();
            let (display_name, y_values) = value_mapper(name, waveform);
            if !Self::samples_match_shared_axis(y_values.len(), shared_x.len()) {
                continue;
            }
            results.push(
                rspice_results::waveform::RetainedWaveform::new(
                    display_name,
                    shared_x.clone(),
                    y_values,
                )
                .with_unit(unit),
            );
        }

        results
    }

    fn samples_match_shared_axis(samples: usize, axis_len: usize) -> bool {
        axis_len > 0 && samples == axis_len
    }
}

//! Lossless FFT readback feeds the existing publication projections.
use super::*;
use rspice_core::engine::{
    TransientFftBin, TransientFftHarmonic, TransientFftMetrics, TransientFftResult,
};
use rspice_core::netlist::{FftAnalysis, FftFormat, FftOutput, FftWindow, XyceFftMode};

mod delimited;
mod readback;

pub(crate) struct FftBundle {
    parent: String,
    ids: Vec<String>,
    coordinate: Option<super::super::ArtifactCoordinate>,
    results: Vec<TransientFftResult>,
    requests: Vec<FftAnalysis>,
}

impl FftBundle {
    pub(crate) fn from_raw(decoded: DecodedFftRawArtifact) -> Result<Self, String> {
        Self::from_metadata(decoded.metadata, decoded.bins)
    }

    fn from_metadata(
        metadata: FftRawMetadata,
        bins: Vec<DecodedFftRawBin>,
    ) -> Result<Self, String> {
        validate_fft_raw_metadata(&metadata)?;
        let mut bins = bins.into_iter().peekable();
        let mut results = Vec::with_capacity(metadata.results.len());
        for result in metadata.results {
            let mut stored = Hdf5FftResult {
                status: result.status,
                analysis_id: result.analysis_id.clone(),
                ordinal: result.ordinal,
                source_kind: result.source.kind,
                source_text: result.source.text,
                authored_output: result.source.authored_output,
                output_name: result.signal.name,
                physical_type: result.signal.physical_type,
                value_unit: result.signal.unit,
                start_time_s: result.sampling.start_time_s,
                stop_time_s: result.sampling.stop_time_s,
                sample_interval_s: result.sampling.sample_interval_s,
                point_count: result.sampling.point_count,
                accurate_sampling: result.sampling.accurate_sampling,
                format: result.transform.format,
                mode: result.transform.mode,
                window: result.transform.window,
                window_name: result.transform.window_name,
                alpha: result.transform.alpha,
                coherent_gain: result.transform.coherent_gain,
                frequency_resolution_hz: result.transform.frequency_resolution_hz,
                fundamental_bin: result.transform.fundamental_bin,
                minimum_metric_bin: result.transform.minimum_metric_bin,
                maximum_metric_bin: result.transform.maximum_metric_bin,
                sfdr_search_minimum_bin: result.transform.sfdr_search_minimum_bin,
                bin_indices: Vec::new(),
                frequency_hz: Vec::new(),
                real: Vec::new(),
                imaginary: Vec::new(),
                magnitude: Vec::new(),
                phase_degrees: Vec::new(),
                metrics: result.metrics.map(|metrics| Hdf5FftMetrics {
                    fundamental_magnitude: metrics.fundamental_magnitude,
                    thd_ratio: metrics.thd_ratio,
                    thd_db: metrics.thd_db,
                    sndr_db: metrics.sndr_db,
                    enob_bits: metrics.enob_bits,
                    snr_db: metrics.snr_db,
                    sfdr_db: metrics.sfdr_db,
                    sfdr_spur_bin: metrics.sfdr_spur_bin,
                    sfdr_spur_frequency_hz: metrics.sfdr_spur_frequency_hz,
                    largest_harmonics: metrics
                        .largest_harmonics
                        .into_iter()
                        .map(|harmonic| Hdf5FftHarmonic {
                            rank: harmonic.rank,
                            bin: harmonic.bin,
                            frequency_hz: harmonic.frequency_hz,
                            magnitude: harmonic.magnitude,
                            magnitude_db: harmonic.magnitude_db,
                            phase_degrees: harmonic.phase_degrees,
                        })
                        .collect(),
                }),
            };
            while bins
                .peek()
                .is_some_and(|bin| bin.analysis_id == stored.analysis_id)
            {
                let bin = bins.next().expect("peeked FFT bin");
                stored
                    .bin_indices
                    .push(u64::try_from(bin.index).map_err(|_| "FFT index exceeds u64")?);
                stored.frequency_hz.push(bin.frequency_hz);
                stored.real.push(bin.real);
                stored.imaginary.push(bin.imaginary);
                stored.magnitude.push(bin.magnitude);
                stored.phase_degrees.push(bin.phase_degrees);
            }
            results.push(stored);
        }
        if bins.next().is_some() {
            return Err("FFT bins are out of source order or refer to an unknown analysis".into());
        }
        Self::from_section(Hdf5FftSection {
            parent_analysis_id: metadata.parent_analysis_id,
            coordinate: metadata.coordinate.map(|coordinate| Hdf5FftCoordinate {
                coordinate_id: coordinate.coordinate_id,
                ordinal: coordinate.ordinal,
                tag: coordinate.tag,
                assignment: coordinate.assignment,
            }),
            results,
        })
    }

    pub(crate) fn from_section(section: Hdf5FftSection) -> Result<Self, String> {
        section.validate().map_err(|error| error.to_string())?;
        let mut ids = Vec::with_capacity(section.results.len());
        let mut results = Vec::with_capacity(section.results.len());
        let mut requests = Vec::with_capacity(section.results.len());
        for stored in section.results {
            let format = match stored.format.as_str() {
                "normalized" => FftFormat::Normalized,
                "unnormalized" => FftFormat::Unnormalized,
                _ => return Err("unsupported FFT format".into()),
            };
            let mode = match stored.mode.as_str() {
                "hspice_compatible" => XyceFftMode::HspiceCompatible,
                "spectre_compatible" => XyceFftMode::SpectreCompatible,
                _ => return Err("unsupported FFT compatibility mode".into()),
            };
            let physical_type = match stored.physical_type.as_str() {
                "voltage" => "voltage",
                "current" => "current",
                "parameter" => "parameter",
                _ => return Err("unsupported FFT physical quantity".into()),
            };
            let window = [
                FftWindow::Rectangular,
                FftWindow::Bartlett,
                FftWindow::BartlettHann,
                FftWindow::Hamming,
                FftWindow::Hann,
                FftWindow::Blackman67Db,
                FftWindow::Blackman,
                FftWindow::BlackmanHarris,
                FftWindow::Nuttall,
                FftWindow::HalfCycleSine,
                FftWindow::HalfCycleSine3,
                FftWindow::HalfCycleSine6,
                FftWindow::Cosine2,
                FftWindow::Cosine4,
                FftWindow::Gaussian,
                FftWindow::Kaiser,
            ]
            .into_iter()
            .find(|window| fft_window_name(*window) == stored.window)
            .ok_or("unsupported FFT window")?;
            let output = match stored.source_kind.as_str() {
                "probe" => FftOutput::Probe(stored.source_text),
                "expression" => FftOutput::Expression(stored.source_text),
                _ => return Err("unsupported FFT source kind".into()),
            };
            let bins = stored
                .bin_indices
                .into_iter()
                .zip(stored.frequency_hz)
                .zip(stored.real)
                .zip(stored.imaginary)
                .zip(stored.magnitude)
                .zip(stored.phase_degrees)
                .map(
                    |(((((index, frequency), real), imaginary), magnitude), phase_degrees)| {
                        Ok(TransientFftBin {
                            index: usize::try_from(index)
                                .map_err(|_| "FFT bin exceeds this platform")?,
                            frequency,
                            real,
                            imaginary,
                            magnitude,
                            phase_degrees,
                        })
                    },
                )
                .collect::<Result<Vec<_>, String>>()?;
            let metrics = stored.metrics.map(|metrics| TransientFftMetrics {
                fundamental_magnitude: metrics.fundamental_magnitude,
                thd_ratio: metrics.thd_ratio,
                thd_db: metrics.thd_db,
                sndr_db: metrics.sndr_db,
                enob_bits: metrics.enob_bits,
                snr_db: metrics.snr_db,
                sfdr_db: metrics.sfdr_db,
                sfdr_spur_bin: metrics.sfdr_spur_bin,
                sfdr_spur_frequency: metrics.sfdr_spur_frequency_hz,
                largest_harmonics: metrics
                    .largest_harmonics
                    .into_iter()
                    .map(|harmonic| TransientFftHarmonic {
                        rank: harmonic.rank,
                        bin: harmonic.bin,
                        frequency: harmonic.frequency_hz,
                        magnitude: harmonic.magnitude,
                        magnitude_db: harmonic.magnitude_db,
                        phase_degrees: harmonic.phase_degrees,
                    })
                    .collect(),
            });
            // Only FMIN's presence affects re-publication: retain the stored
            // SFDR search policy without claiming to recover its authored text.
            let minimum_frequency = (stored.sfdr_search_minimum_bin == stored.minimum_metric_bin)
                .then_some(stored.minimum_metric_bin as f64 * stored.frequency_resolution_hz);
            requests.push(FftAnalysis {
                output: output.clone(),
                start: Some(stored.start_time_s),
                stop: Some(stored.stop_time_s),
                points: stored.point_count,
                format: Some(format),
                window,
                window_name: stored.window_name.clone(),
                alpha: stored.alpha,
                fundamental_frequency: Some(
                    stored.fundamental_bin as f64 * stored.frequency_resolution_hz,
                ),
                minimum_frequency,
                maximum_frequency: Some(
                    stored.maximum_metric_bin as f64 * stored.frequency_resolution_hz,
                ),
            });
            ids.push(stored.analysis_id);
            results.push(TransientFftResult {
                status: stored.status,
                output,
                output_name: stored.output_name,
                physical_type,
                start_time: stored.start_time_s,
                stop_time: stored.stop_time_s,
                sample_interval: stored.sample_interval_s,
                point_count: stored.point_count,
                accurate_sampling: stored.accurate_sampling,
                format,
                mode,
                window,
                window_name: stored.window_name,
                alpha: stored.alpha,
                coherent_gain: stored.coherent_gain,
                frequency_resolution: stored.frequency_resolution_hz,
                fundamental_bin: stored.fundamental_bin,
                minimum_metric_bin: stored.minimum_metric_bin,
                maximum_metric_bin: stored.maximum_metric_bin,
                bins,
                metrics,
            });
        }
        Ok(Self {
            parent: section.parent_analysis_id,
            coordinate: section
                .coordinate
                .map(|coordinate| super::super::ArtifactCoordinate {
                    id: coordinate.coordinate_id,
                    ordinal: coordinate.ordinal,
                    tag: coordinate.tag,
                    assignment: coordinate.assignment,
                }),
            ids,
            results,
            requests,
        })
    }

    pub(crate) fn comparison_document(&self) -> Result<serde_json::Value, CliError> {
        serde_json::to_value(fft_json_document(
            &self.parent,
            &self.ids,
            self.coordinate.as_ref(),
            &self.results,
            &self.requests,
        ))
        .map_err(|error| CliError::ConversionError {
            message: error.to_string(),
        })
    }

    pub(crate) fn write(&self, path: &Path, format: OutputFormat) -> Result<(), CliError> {
        publish::artifact(path, |writer| {
            write_fft_to_writer(
                writer,
                path,
                format,
                &self.parent,
                &self.ids,
                self.coordinate.as_ref(),
                &self.results,
                &self.requests,
                None,
            )
        })
        .map_err(|error| map_atomic_output_error(path, error))
    }
}

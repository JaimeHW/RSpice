//! CSV encoders over already-selected canonical result records.
//!
//! The application owns view/cache admission, filenames and publication.

mod analysis_stack;
mod noise_report;
mod operating_point_report;
mod typed;
pub use analysis_stack::{AnalysisStackCsv, AnalysisStackCsvError};
pub use noise_report::NoiseReportCsv;
pub use operating_point_report::OperatingPointReportCsv;
pub use typed::{EncodedTypedCsv, TypedCsvSummary, encode_typed_result_csv};

use crate::table::escape_csv_field as csv_text;
use rspice_core::analysis::signal_integrity::EyeMeasurements;
use rspice_results::fft::data::FftData;
use rspice_results::histogram::{Histogram, HistogramDistribution};

/// Encode the selected spectrum with the source identity supplied by its owner.
pub fn encode_spectrum_csv(source: &str, data: &FftData) -> String {
    let mut contents = String::from("field,value,unit\n");
    for (field, value, unit) in [
        ("source", csv_text(source), ""),
        ("window", csv_text(data.window.display_name()), ""),
        ("fft_size", data.fft_size.to_string(), ""),
        ("sample_rate", format!("{:.17e}", data.sample_rate), "Hz"),
        (
            "resolution_bandwidth",
            format!("{:.17e}", data.resolution_bandwidth()),
            "Hz",
        ),
        (
            "normalization",
            csv_text(data.normalization.display_name()),
            "",
        ),
    ] {
        contents.push_str(&format!("{field},{value},{unit}\n"));
    }
    contents.push_str("\nfrequency_hz,magnitude,magnitude_db,phase_rad\n");
    for point in &data.points {
        contents.push_str(&format!(
            "{:.17e},{:.17e},{:.17e},{:.17e}\n",
            point.frequency,
            point.magnitude,
            point.magnitude_db(),
            point.phase
        ));
    }
    contents
}

/// Encode exact bins and the selected distribution's data-space coordinates.
pub fn encode_histogram_csv(histogram: &Histogram, display: &HistogramDistribution) -> String {
    let mut contents = String::from("field,value,unit\n");
    for (field, value) in [
        ("measurement", csv_text(&histogram.name)),
        ("display_mode", csv_text(display.mode.label())),
        ("ordinate_unit", csv_text(display.mode.unit())),
        ("total_count", histogram.total_count.to_string()),
        ("total_weight", format!("{:.17e}", histogram.total_weight)),
        ("underflow", histogram.underflow.to_string()),
        ("overflow", histogram.overflow.to_string()),
        ("data_min", format!("{:.17e}", histogram.data_min)),
        ("data_max", format!("{:.17e}", histogram.data_max)),
    ] {
        contents.push_str(&format!("{field},{value},\n"));
    }
    contents.push_str("\nbin_lower,bin_upper,count,weight\n");
    for bin in &histogram.bins {
        contents.push_str(&format!(
            "{:.17e},{:.17e},{},{:.17e}\n",
            bin.lower, bin.upper, bin.count, bin.weight
        ));
    }
    if display.cdf.is_some() {
        contents.push_str("\nobservation,cumulative_probability\n");
        for (x, y) in display.source_points(histogram) {
            contents.push_str(&format!("{x:.17e},{y:.17e}\n"));
        }
    } else {
        contents.push_str("\nbin_center,display_ordinate\n");
        for (x, y) in display.source_points(histogram) {
            contents.push_str(&format!("{x:.17e},{y:.17e}\n"));
        }
    }
    contents
}

/// A measurement that could not be made exports as an empty cell, not as a
/// zero: a spreadsheet that averages a column of rise times must not be handed
/// a `0 s` that no acquisition contains.
///
/// An *unbounded* measurement keeps its value and exports as `inf`. The Q and
/// SNR of a noiseless eye are unbounded, which is an answer — the sheet prints
/// `∞` for them — and blanking it here would tell a reader the eye had no Q at
/// all, which is the one thing an empty cell in this column means.
fn csv_measurement(value: Option<f64>) -> String {
    value.map_or_else(String::new, |value| format!("{value:.17e}"))
}

/// Encode measurements and the acquisition/timebase context selected by the caller.
pub fn encode_eye_measurements_csv(
    acquisitions: usize,
    unit_intervals: u32,
    unit_interval_source: &str,
    m: &EyeMeasurements,
) -> String {
    let mut contents = String::from("field,value,unit\n");
    for (field, value, unit) in [
        ("acquisitions", acquisitions.to_string(), ""),
        ("unit_intervals", unit_intervals.to_string(), ""),
        ("data_rate", format!("{:.17e}", m.data_rate), "b/s"),
        ("unit_interval", format!("{:.17e}", m.unit_interval), "s"),
        ("unit_interval_source", csv_text(unit_interval_source), ""),
        ("eye_height", format!("{:.17e}", m.eye_height), "V"),
        ("eye_width", format!("{:.17e}", m.eye_width), "UI"),
        ("eye_area", format!("{:.17e}", m.eye_area), ""),
        (
            "vertical_margin",
            format!("{:.17e}", m.vertical_margin),
            "V",
        ),
        (
            "horizontal_margin",
            format!("{:.17e}", m.horizontal_margin),
            "UI",
        ),
        ("rise_time", csv_measurement(m.rise_time), "s"),
        ("fall_time", csv_measurement(m.fall_time), "s"),
        ("jitter_pp", format!("{:.17e}", m.jitter_pp), "s"),
        ("jitter_rms", format!("{:.17e}", m.jitter_rms), "s"),
        ("jitter_dj", format!("{:.17e}", m.jitter_dj), "s"),
        ("crossing_level", csv_measurement(m.crossing_level), "V"),
        (
            "crossing_percentage",
            csv_measurement(m.crossing_percentage),
            "",
        ),
        ("snr", csv_measurement(m.snr_db), "dB"),
        ("q_factor", csv_measurement(m.q_factor), ""),
        ("estimated_ber", csv_measurement(m.estimated_ber), ""),
    ] {
        contents.push_str(&format!("{field},{value},{unit}\n"));
    }
    contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::fft::data::SpectrumNormalization;
    use rspice_results::fft::pipeline::MIN_FFT_SAMPLES;
    use rspice_results::fft::window::WindowFunction;
    use rspice_results::histogram::{HistogramBuilder, HistogramDisplayMode};

    #[test]
    fn derived_documents_retain_exact_sections_units_and_unavailable_measurements() {
        let spectrum = FftData::from_time_domain_with_normalization(
            "record",
            &[0.0; MIN_FFT_SAMPLES],
            8_000.0,
            WindowFunction::Rectangular,
            SpectrumNormalization::Peak,
        )
        .unwrap();
        let text = encode_spectrum_csv("V(out,\"ref\")", &spectrum);
        let records = csv::ReaderBuilder::new()
            .has_headers(false)
            .flexible(true)
            .from_reader(text.as_bytes())
            .records()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(&records[1][1], "V(out,\"ref\")");
        assert!(
            records
                .iter()
                .any(|row| row.get(0) == Some("sample_rate") && row.get(2) == Some("Hz"))
        );
        assert!(text.contains("\nfrequency_hz,magnitude,magnitude_db,phase_rad\n"));
        assert!(
            text.contains(
                "0.00000000000000000e0,0.00000000000000000e0,-inf,0.00000000000000000e0\n"
            )
        );

        let samples = [1.0, 1.0, 3.0];
        let histogram = HistogramBuilder::new().bin_count(1).build(&samples);
        for mode in [HistogramDisplayMode::Count, HistogramDisplayMode::Cdf] {
            let display = HistogramDistribution::new(&histogram, &samples, mode).unwrap();
            let text = encode_histogram_csv(&histogram, &display);
            assert!(text.contains("\nbin_lower,bin_upper,count,weight\n"));
            if mode == HistogramDisplayMode::Cdf {
                assert!(text.contains("\nobservation,cumulative_probability\n"));
                assert!(text.ends_with("3.00000000000000000e0,1.00000000000000000e0\n"));
            } else {
                assert!(text.contains("\nbin_center,display_ordinate\n"));
            }
        }

        let measurements = EyeMeasurements {
            snr_db: Some(f64::INFINITY),
            q_factor: Some(f64::INFINITY),
            estimated_ber: Some(0.0),
            fall_time: Some(-0.0),
            ..Default::default()
        };
        let text =
            encode_eye_measurements_csv(3, 2, "auto from 6 edges (low confidence)", &measurements);
        assert!(text.contains("acquisitions,3,\nunit_intervals,2,\n"));
        assert!(text.contains("unit_interval_source,auto from 6 edges (low confidence),\n"));
        assert!(text.contains("rise_time,,s\nfall_time,-0.00000000000000000e0,s\n"));
        assert!(text.contains("crossing_level,,V\ncrossing_percentage,,\nsnr,inf,dB\nq_factor,inf,\nestimated_ber,0.00000000000000000e0,\n"));
    }
}

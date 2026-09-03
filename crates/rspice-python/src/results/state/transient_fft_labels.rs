//! The stable tag vocabulary of transient `.FFT` state.
//!
//! Every enum a `.FFT` product carries travels as a string label rather than
//! an ordinal, and every label is rejected by name when a later build no
//! longer knows it. Keeping the pairs adjacent is what makes an unbalanced
//! one — a label written but not read back — visible on sight.

use super::*;

pub(super) fn fft_output_state(
    output: &rspice_core::netlist::FftOutput,
) -> TransientFftSourceState {
    match output {
        rspice_core::netlist::FftOutput::Probe(value) => ("probe".to_string(), value.clone()),
        rspice_core::netlist::FftOutput::Expression(value) => {
            ("expression".to_string(), value.clone())
        }
    }
}

pub(super) fn fft_output_from_state(
    state: TransientFftSourceState,
) -> PyResult<rspice_core::netlist::FftOutput> {
    let (kind, value) = state;
    if value.trim().is_empty() {
        return Err(crate::errors::value_error(
            "pickled transient FFT source descriptor cannot be empty",
        ));
    }
    match kind.as_str() {
        "probe" => Ok(rspice_core::netlist::FftOutput::Probe(value)),
        "expression" => Ok(rspice_core::netlist::FftOutput::Expression(value)),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT source kind '{other}' in pickled state"
        ))),
    }
}

pub(super) fn fft_format_label(format: rspice_core::netlist::FftFormat) -> &'static str {
    match format {
        rspice_core::netlist::FftFormat::Normalized => "normalized",
        rspice_core::netlist::FftFormat::Unnormalized => "unnormalized",
    }
}

pub(super) fn fft_format_from_label(label: &str) -> PyResult<rspice_core::netlist::FftFormat> {
    match label {
        "normalized" => Ok(rspice_core::netlist::FftFormat::Normalized),
        "unnormalized" => Ok(rspice_core::netlist::FftFormat::Unnormalized),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT format '{other}' in pickled state"
        ))),
    }
}

pub(super) fn fft_mode_label(mode: rspice_core::netlist::XyceFftMode) -> &'static str {
    match mode {
        rspice_core::netlist::XyceFftMode::HspiceCompatible => "hspice_compatible",
        rspice_core::netlist::XyceFftMode::SpectreCompatible => "spectre_compatible",
    }
}

pub(super) fn fft_mode_from_label(label: &str) -> PyResult<rspice_core::netlist::XyceFftMode> {
    match label {
        "hspice_compatible" => Ok(rspice_core::netlist::XyceFftMode::HspiceCompatible),
        "spectre_compatible" => Ok(rspice_core::netlist::XyceFftMode::SpectreCompatible),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT mode '{other}' in pickled state"
        ))),
    }
}

pub(super) fn fft_window_label(window: rspice_core::netlist::FftWindow) -> &'static str {
    use rspice_core::netlist::FftWindow;
    match window {
        FftWindow::Rectangular => "rectangular",
        FftWindow::Bartlett => "bartlett",
        FftWindow::BartlettHann => "bartlett_hann",
        FftWindow::Hamming => "hamming",
        FftWindow::Hann => "hann",
        FftWindow::Blackman67Db => "blackman_67db",
        FftWindow::Blackman => "blackman",
        FftWindow::BlackmanHarris => "blackman_harris",
        FftWindow::Nuttall => "nuttall",
        FftWindow::HalfCycleSine => "half_cycle_sine",
        FftWindow::HalfCycleSine3 => "half_cycle_sine_3",
        FftWindow::HalfCycleSine6 => "half_cycle_sine_6",
        FftWindow::Cosine2 => "cosine_2",
        FftWindow::Cosine4 => "cosine_4",
    }
}

pub(super) fn fft_window_from_label(label: &str) -> PyResult<rspice_core::netlist::FftWindow> {
    use rspice_core::netlist::FftWindow;
    match label {
        "rectangular" => Ok(FftWindow::Rectangular),
        "bartlett" => Ok(FftWindow::Bartlett),
        "bartlett_hann" => Ok(FftWindow::BartlettHann),
        "hamming" => Ok(FftWindow::Hamming),
        "hann" => Ok(FftWindow::Hann),
        "blackman_67db" => Ok(FftWindow::Blackman67Db),
        "blackman" => Ok(FftWindow::Blackman),
        "blackman_harris" => Ok(FftWindow::BlackmanHarris),
        "nuttall" => Ok(FftWindow::Nuttall),
        "half_cycle_sine" => Ok(FftWindow::HalfCycleSine),
        "half_cycle_sine_3" => Ok(FftWindow::HalfCycleSine3),
        "half_cycle_sine_6" => Ok(FftWindow::HalfCycleSine6),
        "cosine_2" => Ok(FftWindow::Cosine2),
        "cosine_4" => Ok(FftWindow::Cosine4),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT window '{other}' in pickled state"
        ))),
    }
}

pub(super) fn fft_physical_type_from_label(label: &str) -> PyResult<&'static str> {
    match label {
        "voltage" => Ok("voltage"),
        "current" => Ok("current"),
        "parameter" => Ok("parameter"),
        other => Err(crate::errors::value_error(format!(
            "unknown transient FFT physical type '{other}' in pickled state"
        ))),
    }
}

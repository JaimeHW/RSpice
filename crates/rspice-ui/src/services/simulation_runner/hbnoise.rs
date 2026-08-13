//! Harmonic-balance noise analysis from an authenticated retained HB state.
//!
//! This service never computes or approximates a large-signal operating
//! point. It consumes the exact frozen HB coefficients and delegates the
//! cyclostationary adjoint solve to `rspice-core`.

use std::path::Path;

use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{Engine, HbOperatingPoint};

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, generate_freq_points_with_abort,
    is_ground_like, netlist_has_independent_source_named_with_abort,
    parse_runner_netlist_with_abort,
};

/// Frequency sweep type for HBNOISE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbnoiseFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl HbnoiseFrequencySweep {
    const fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Exact retained-HB noise request.
#[derive(Debug, Clone)]
pub struct HbnoiseRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: HbnoiseFrequencySweep,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub input_source: String,
    pub max_sideband: usize,
    pub integrated_noise: bool,
    pub noise_figure: bool,
    pub contributor_ranking: bool,
}

impl HbnoiseRunConfig {
    fn validate(&self) -> Result<(), ServiceRunError> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(ServiceRunError::Failure(
                "HBNOISE start frequency must be finite and positive".to_owned(),
            ));
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err(ServiceRunError::Failure(
                "HBNOISE stop frequency must be finite and >= start frequency".to_owned(),
            ));
        }
        if self.points_per_unit == 0 {
            return Err(ServiceRunError::Failure(
                "HBNOISE points per unit must be greater than zero".to_owned(),
            ));
        }
        if self.output_node.trim().is_empty() {
            return Err(ServiceRunError::Failure(
                "HBNOISE output node must be specified".to_owned(),
            ));
        }
        if self.input_source.trim().is_empty() {
            return Err(ServiceRunError::Failure(
                "HBNOISE input source must be specified".to_owned(),
            ));
        }
        if self.max_sideband == 0 || self.max_sideband > i32::MAX as usize {
            return Err(ServiceRunError::Failure(
                "HBNOISE maximum sideband must be within 1..=2147483647".to_owned(),
            ));
        }
        if self.noise_figure {
            return Err(ServiceRunError::Failure(
                "HBNOISE noise figure requires an explicit source impedance and available-noise temperature contract; disable noise figure until those port references are configured"
                    .to_owned(),
            ));
        }
        Ok(())
    }
}

/// Exact HBNOISE spectra and band-integrated evidence.
#[derive(Debug, Clone)]
pub struct HbnoiseData {
    pub frequencies: Vec<Value>,
    pub output_noise: Vec<Value>,
    pub input_noise: Vec<Value>,
    pub contributors: Vec<(String, Vec<Value>)>,
    pub output_rms: Option<Value>,
    pub input_rms: Option<Value>,
}

/// Run HBNOISE from an immutable retained harmonic-balance state.
pub fn run_hbnoise_analysis_from_hb_with_source_path_and_abort(
    netlist_text: &str,
    config: &HbnoiseRunConfig,
    operating_point: &HbOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HbnoiseData> {
    ensure_not_aborted(abort)?;
    config.validate()?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let source_name = config.input_source.trim();
    if !netlist_has_independent_source_named_with_abort(&netlist, source_name, abort)? {
        return Err(ServiceRunError::Failure(format!(
            "HBNOISE input source '{source_name}' is not an independent voltage/current source in the netlist"
        )));
    }

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;
    if (config.integrated_noise || config.contributor_ranking) && frequencies.len() < 2 {
        return Err(ServiceRunError::Failure(
            "HBNOISE band integration and contributor ranking require at least two distinct frequency points"
                .to_owned(),
        ));
    }
    let output_ref = config
        .output_ref
        .as_deref()
        .map(str::trim)
        .filter(|node| !node.is_empty() && !is_ground_like(node));
    if output_ref.is_some_and(|node| node.eq_ignore_ascii_case(config.output_node.trim())) {
        return Err(ServiceRunError::Failure(
            "HBNOISE output node and output reference cannot be the same node".to_owned(),
        ));
    }

    let engine = Engine::new(build_engine_config(&netlist, None));
    let exact = engine
        .run_pnoise_from_hb_with_abort(
            &netlist,
            &frequencies,
            config.output_node.trim(),
            output_ref,
            Some(source_name),
            config.max_sideband as i32,
            operating_point,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("exact retained-state HBNOISE", error))?;
    let input_noise = exact.input_noise.ok_or_else(|| {
        ServiceRunError::Failure(
            "exact retained-state HBNOISE did not produce its required input-referred spectrum"
                .to_owned(),
        )
    })?;
    validate_psd_series(&frequencies, &exact.output_noise, "output", abort)?;
    validate_psd_series(&frequencies, &input_noise, "input-referred", abort)?;
    for (name, values) in &exact.contributors {
        validate_psd_series(
            &frequencies,
            values,
            &format!("contributor '{name}'"),
            abort,
        )?;
    }

    let (output_rms, input_rms) = if config.integrated_noise {
        (
            Some(integrate_psd(&frequencies, &exact.output_noise, abort)?.sqrt()),
            Some(integrate_psd(&frequencies, &input_noise, abort)?.sqrt()),
        )
    } else {
        (None, None)
    };
    ensure_not_aborted(abort)?;
    Ok(HbnoiseData {
        frequencies,
        output_noise: exact.output_noise,
        input_noise,
        contributors: if config.contributor_ranking {
            exact.contributors
        } else {
            Vec::new()
        },
        output_rms,
        input_rms,
    })
}

fn validate_psd_series(
    frequencies: &[Value],
    values: &[Value],
    label: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    if frequencies.is_empty() || frequencies.len() != values.len() {
        return Err(ServiceRunError::Failure(format!(
            "HBNOISE {label} PSD has {} samples for {} frequency points",
            values.len(),
            frequencies.len()
        )));
    }
    for (index, (&frequency, &value)) in frequencies.iter().zip(values).enumerate() {
        poll_periodically(abort, index)?;
        if !frequency.is_finite() || frequency <= 0.0 || !value.is_finite() || value < 0.0 {
            return Err(ServiceRunError::Failure(format!(
                "HBNOISE {label} PSD contains invalid data at index {index}"
            )));
        }
    }
    if frequencies.windows(2).any(|pair| pair[1] <= pair[0]) {
        return Err(ServiceRunError::Failure(
            "HBNOISE frequency sweep is not strictly increasing".to_owned(),
        ));
    }
    ensure_not_aborted(abort)
}

pub(crate) fn integrate_psd(
    frequencies: &[Value],
    values: &[Value],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    validate_psd_series(frequencies, values, "integration", abort)?;
    if frequencies.len() < 2 {
        return Err(ServiceRunError::Failure(
            "HBNOISE PSD integration requires at least two distinct frequency points".to_owned(),
        ));
    }
    let mut power = 0.0;
    for (index, (frequency_pair, value_pair)) in
        frequencies.windows(2).zip(values.windows(2)).enumerate()
    {
        poll_periodically(abort, index)?;
        power += 0.5 * (value_pair[0] + value_pair[1]) * (frequency_pair[1] - frequency_pair[0]);
    }
    if !power.is_finite() || power < 0.0 {
        return Err(ServiceRunError::Failure(
            "HBNOISE PSD integration produced invalid power".to_owned(),
        ));
    }
    ensure_not_aborted(abort)?;
    Ok(power)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::harmonic_balance::HbConfig;
    use rspice_core::netlist::Netlist;

    fn retained_hb(deck: &str) -> HbOperatingPoint {
        let netlist = Netlist::parse(deck).expect("deck parses");
        Engine::new(Default::default())
            .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(8))
            .expect("HB runs")
            .operating_point
    }

    #[test]
    fn hbnoise_returns_exact_psd_integration_and_ranked_contributors() {
        let deck = "* HBNOISE service fixture\nvin in 0 dc 0\nr1 in out 1k\nr2 out 0 1k\n.end\n";
        let config = HbnoiseRunConfig {
            start_freq: 1.0e3,
            stop_freq: 1.0e4,
            points_per_unit: 3,
            sweep: HbnoiseFrequencySweep::Linear,
            output_node: "out".to_owned(),
            output_ref: None,
            input_source: "vin".to_owned(),
            max_sideband: 3,
            integrated_noise: true,
            noise_figure: false,
            contributor_ranking: true,
        };
        let data = run_hbnoise_analysis_from_hb_with_source_path_and_abort(
            deck,
            &config,
            &retained_hb(deck),
            None,
            &NoAbort,
        )
        .expect("HBNOISE runs");
        assert_eq!(data.frequencies.len(), 3);
        assert!(data.output_noise.iter().all(|value| *value > 0.0));
        assert!(data.input_noise.iter().all(|value| *value > 0.0));
        assert!(data.output_rms.is_some_and(|value| value > 0.0));
        assert!(data.input_rms.is_some_and(|value| value > 0.0));
        assert_eq!(data.contributors.len(), 2);
    }

    #[test]
    fn hbnoise_noise_figure_fails_closed_without_a_port_reference() {
        let config = HbnoiseRunConfig {
            start_freq: 1.0,
            stop_freq: 10.0,
            points_per_unit: 2,
            sweep: HbnoiseFrequencySweep::Linear,
            output_node: "out".to_owned(),
            output_ref: None,
            input_source: "vin".to_owned(),
            max_sideband: 1,
            integrated_noise: false,
            noise_figure: true,
            contributor_ranking: false,
        };
        let error = config.validate().expect_err("NF contract is incomplete");
        assert!(error.to_string().contains("source impedance"));
    }

    #[test]
    fn hbnoise_never_treats_a_spot_psd_as_band_integrated_power() {
        let error = integrate_psd(&[1.0e3], &[2.0e-18], &NoAbort)
            .expect_err("a one-point PSD has no integration bandwidth");
        assert!(error.to_string().contains("at least two"));
    }
}

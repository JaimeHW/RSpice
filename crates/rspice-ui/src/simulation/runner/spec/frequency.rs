//! Dispatch for frequency-domain analyses.

use std::{collections::HashMap, path::Path};

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;
use crate::simulation::execution::ResolvedExecutionDependencies;
use crate::simulation::multi_run::{AnalysisSpec, FrequencySweep, SpPort};
use crate::simulation::results::{SimulationResult, WaveformData};
use crate::simulation::runner::{SimulationError, SpecExecutionOptions};

pub(super) fn run_frequency_spec(
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    super::ensure_not_aborted(abort)?;
    match spec {
        AnalysisSpec::SParameter {
            start_freq,
            stop_freq,
            points_per_unit,
            sweep,
            z0,
            ports,
        } => run_sparameter(
            netlist,
            SParameterRequest {
                start_freq,
                stop_freq,
                points_per_unit,
                sweep,
                z0,
                ports,
            },
            source_path,
            abort,
        ),
        AnalysisSpec::Tf {
            input_source,
            output_expression,
            transfer_gain,
            input_resistance,
            output_resistance,
            normalization,
            accuracy,
        } => run_tf(
            netlist,
            svc_runner::TfRunConfig {
                input_source,
                output_expression,
                transfer_gain,
                input_resistance,
                output_resistance,
                normalization: match normalization {
                    crate::simulation::multi_run::TfNormalization::None => {
                        svc_runner::TfNormalization::None
                    }
                    crate::simulation::multi_run::TfNormalization::RelativeToNominal => {
                        svc_runner::TfNormalization::RelativeToNominal
                    }
                    crate::simulation::multi_run::TfNormalization::PerSourceUnit => {
                        svc_runner::TfNormalization::PerSourceUnit
                    }
                },
                accuracy: match accuracy {
                    crate::simulation::multi_run::TfAccuracy::Fast => svc_runner::TfAccuracy::Fast,
                    crate::simulation::multi_run::TfAccuracy::Balanced => {
                        svc_runner::TfAccuracy::Balanced
                    }
                    crate::simulation::multi_run::TfAccuracy::Accurate => {
                        svc_runner::TfAccuracy::Accurate
                    }
                    crate::simulation::multi_run::TfAccuracy::Robust => {
                        svc_runner::TfAccuracy::Robust
                    }
                },
            },
            normalization,
            accuracy,
            source_path,
            abort,
        ),
        AnalysisSpec::Pac => run_pac(netlist, source_path, options, dependencies, abort),
        AnalysisSpec::Pxf => run_pxf(netlist, source_path, options, dependencies, abort),
        AnalysisSpec::Pnoise => run_pnoise(netlist, source_path, options, dependencies, abort),
        AnalysisSpec::Stb {
            probe_node,
            start_freq,
            stop_freq,
            sweep,
            points_per_decade,
            compute_nyquist,
        } => run_stb(
            netlist,
            probe_node,
            start_freq,
            stop_freq,
            sweep,
            points_per_decade,
            compute_nyquist,
            source_path,
            abort,
        ),
        AnalysisSpec::Pstb => run_pstb(netlist, source_path, options, dependencies, abort),
        other => Err(super::misrouted_spec_error("frequency", &other)),
    }
}

struct SParameterRequest {
    start_freq: f64,
    stop_freq: f64,
    points_per_unit: usize,
    sweep: FrequencySweep,
    z0: f64,
    ports: Vec<SpPort>,
}

fn run_sparameter(
    netlist: &str,
    request: SParameterRequest,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let sweep = match request.sweep {
        FrequencySweep::Decade => svc_runner::SParameterSweep::Decade,
        FrequencySweep::Octave => svc_runner::SParameterSweep::Octave,
        FrequencySweep::Linear => svc_runner::SParameterSweep::Linear,
    };
    let mut configured_ports = Vec::with_capacity(request.ports.len());
    for port in request.ports {
        super::ensure_not_aborted(abort)?;
        configured_ports.push(svc_runner::SParameterPort {
            node_pos: port.node_pos,
            node_neg: port.node_neg,
            z0: port.z0,
        });
    }
    let cfg = svc_runner::SParameterRunConfig {
        start_freq: request.start_freq,
        stop_freq: request.stop_freq,
        points_per_unit: request.points_per_unit,
        sweep,
        z0: request.z0,
        ports: configured_ports,
    };
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_sparameter_analysis_with_source_path_and_abort(
            netlist,
            &cfg,
            source_path,
            abort,
        )
    })?;
    let mut waveforms = HashMap::new();
    for row in 0..data.num_ports {
        super::ensure_not_aborted(abort)?;
        for col in 0..data.num_ports {
            super::ensure_not_aborted(abort)?;
            let name = if data.num_ports <= 9 {
                format!("S{}{}", row + 1, col + 1)
            } else {
                format!("S{}_{}", row + 1, col + 1)
            };
            let trace = &data.s[row][col];
            let waveform = complex_waveform(name.clone(), &data.frequencies, trace, abort)?;
            waveforms.insert(name, waveform);
        }
    }

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_tf(
    netlist: &str,
    config: svc_runner::TfRunConfig,
    normalization: crate::simulation::multi_run::TfNormalization,
    accuracy: crate::simulation::multi_run::TfAccuracy,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_tf_analysis_with_config_and_source_path_and_abort(
            netlist,
            &config,
            source_path,
            abort,
        )
    })?;
    super::ensure_not_aborted(abort)?;

    let input_quantity = transfer_quantity(data.gain_metadata.input_quantity);
    let output_quantity = transfer_quantity(data.gain_metadata.output_quantity);
    Ok(SimulationResult::TransferFunction {
        input_source: data.input_source,
        output_expression: data.output_label,
        input_quantity,
        output_quantity,
        input_unit: quantity_unit(input_quantity).to_owned(),
        output_unit: quantity_unit(output_quantity).to_owned(),
        normalization,
        accuracy,
        gain: data
            .gain
            .map(|value| transfer_scalar(value, "transfer gain"))
            .transpose()?,
        input_resistance: data
            .input_resistance
            .map(|value| transfer_scalar(value, "input resistance"))
            .transpose()?,
        output_resistance: data
            .output_resistance
            .map(|value| transfer_scalar(value, "output resistance"))
            .transpose()?,
        nominal_input: finite_optional(data.nominal_input, "nominal input")?,
        nominal_output: finite_optional(data.nominal_output, "nominal output")?,
    })
}

fn transfer_quantity(
    quantity: svc_runner::TfQuantity,
) -> crate::simulation::results::TransferFunctionQuantity {
    match quantity {
        svc_runner::TfQuantity::Voltage => {
            crate::simulation::results::TransferFunctionQuantity::Voltage
        }
        svc_runner::TfQuantity::Current => {
            crate::simulation::results::TransferFunctionQuantity::Current
        }
    }
}

fn quantity_unit(quantity: crate::simulation::results::TransferFunctionQuantity) -> &'static str {
    match quantity {
        crate::simulation::results::TransferFunctionQuantity::Voltage => "V",
        crate::simulation::results::TransferFunctionQuantity::Current => "A",
    }
}

fn transfer_scalar(
    value: f64,
    label: &str,
) -> Result<crate::simulation::results::TransferFunctionScalar, SimulationError> {
    if value.is_nan() {
        return Err(SimulationError::InvalidConfig(format!(
            "TF {label} is not a number"
        )));
    }
    Ok(crate::simulation::results::TransferFunctionScalar::from_f64(value))
}

fn finite_optional(value: Option<f64>, label: &str) -> Result<Option<f64>, SimulationError> {
    value
        .map(|value| {
            if value.is_finite() {
                Ok(value)
            } else {
                Err(SimulationError::InvalidConfig(format!(
                    "TF {label} is non-finite"
                )))
            }
        })
        .transpose()
}

fn run_pac(
    netlist: &str,
    source_path: Option<&Path>,
    options: SpecExecutionOptions,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let pac_cfg = required_execution_option(
        options.pac,
        "PAC analysis requires explicit PAC execution options",
        abort,
    )?;
    let periodic_state = dependencies.periodic_state().map_err(|error| {
        SimulationError::InvalidConfig(format!("PAC periodic dependency is unavailable: {error}"))
    })?;
    periodic_state
        .validate_consumer_basis(
            "PAC",
            pac_cfg.pss_fundamental_freq,
            pac_cfg.pss_num_harmonics,
            pac_cfg.pss_tolerance,
            false,
        )
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pac_analysis_from_pss_with_source_path_and_abort(
            netlist,
            &pac_cfg,
            periodic_state.operating_point(),
            source_path,
            abort,
        )
    })?;

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms: spectra_to_complex_waveforms(data.spectra, abort)?,
        measurements: Vec::new(),
    })
}

fn run_pxf(
    netlist: &str,
    source_path: Option<&Path>,
    options: SpecExecutionOptions,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let pxf_cfg = required_execution_option(
        options.pxf,
        "PXF analysis requires explicit PXF execution options",
        abort,
    )?;
    let periodic_state = dependencies.periodic_state().map_err(|error| {
        SimulationError::InvalidConfig(format!("PXF periodic dependency is unavailable: {error}"))
    })?;
    periodic_state
        .validate_consumer_basis(
            "PXF",
            pxf_cfg.pss_fundamental_freq,
            pxf_cfg.pss_num_harmonics,
            pxf_cfg.pss_tolerance,
            false,
        )
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pxf_analysis_from_pss_with_source_path_and_abort(
            netlist,
            &pxf_cfg,
            periodic_state.operating_point(),
            source_path,
            abort,
        )
    })?;

    let mut waveforms = HashMap::new();
    let transfer_name = format!(
        "H(sb{}->sb{}, {})",
        data.input_sideband, data.output_sideband, data.output_label
    );
    let transfer = complex_waveform(
        transfer_name.clone(),
        &data.frequencies,
        &data.transfer,
        abort,
    )?;
    waveforms.insert(transfer_name, transfer);
    insert_group_delay(&mut waveforms, data.group_delay, abort)?;

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        waveforms,
        measurements: Vec::new(),
    })
}

fn run_pnoise(
    netlist: &str,
    source_path: Option<&Path>,
    options: SpecExecutionOptions,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let pnoise_cfg = required_execution_option(
        options.pnoise,
        "PNOISE analysis requires explicit PNOISE execution options",
        abort,
    )?;
    let periodic_state = dependencies.periodic_state().map_err(|error| {
        SimulationError::InvalidConfig(format!(
            "PNOISE periodic dependency is unavailable: {error}"
        ))
    })?;
    periodic_state
        .validate_consumer_basis(
            "PNOISE",
            pnoise_cfg.pss_fundamental_freq,
            pnoise_cfg.pss_num_harmonics,
            pnoise_cfg.pss_tolerance,
            pnoise_cfg.noise_ref == svc_runner::PnoiseReference::Phase,
        )
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pnoise_analysis_from_pss_with_source_path_and_abort(
            netlist,
            &pnoise_cfg,
            periodic_state.operating_point(),
            source_path,
            abort,
        )
    })?;

    let freq_len = data.frequencies.len().max(1);
    let mut contributors = HashMap::with_capacity(data.contributors.len());
    for (name, percentage) in data.contributors {
        super::ensure_not_aborted(abort)?;
        contributors.insert(name, vec![percentage; freq_len]);
    }

    Ok(SimulationResult::Noise {
        frequencies: data.frequencies,
        output_noise: data.output_noise,
        input_noise: data.input_noise,
        contributors,
        // PNoise carries cyclostationary percentages, not band-integrated
        // mechanism contributions; no ranked summary for it (yet).
        summary: None,
    })
}

fn run_stb(
    netlist: &str,
    probe_node: String,
    start_freq: f64,
    stop_freq: f64,
    sweep: FrequencySweep,
    points_per_decade: usize,
    compute_nyquist: bool,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_stb_analysis_with_sweep_and_source_path_and_abort(
            netlist,
            &probe_node,
            start_freq,
            stop_freq,
            stb_sweep_type(sweep),
            points_per_decade,
            compute_nyquist,
            source_path,
            abort,
        )
    })?;

    let mut waveforms = HashMap::new();
    super::ensure_not_aborted(abort)?;
    insert_scalar_waveform(
        &mut waveforms,
        "Loop Gain (dB)".to_string(),
        data.frequencies.clone(),
        data.loop_gain_db,
        "dB",
        "Hz",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Loop Phase (deg)".to_string(),
        data.frequencies.clone(),
        data.loop_phase_deg,
        "deg",
        "Hz",
    );
    if let Some(contour) = data.nyquist {
        super::ensure_not_aborted(abort)?;
        waveforms.insert(
            "Nyquist L(jw)".to_string(),
            WaveformData::new_complex(
                "Nyquist L(jw)".to_string(),
                contour.frequencies,
                contour.real,
                contour.imaginary,
            ),
        );
    }

    Ok(SimulationResult::Ac {
        frequencies: data.frequencies,
        measurements: stb_margin_measurements(&data.margins),
        waveforms,
    })
}

/// The margins the loop-gain extraction produced, as measurements.
///
/// Stability analysis exists to answer "how much margin", so the margins are
/// reported as named scalars a specification can be written against — not
/// left inside a waveform for the reader to eyeball. A margin the extraction
/// could not locate is reported as a failed measurement rather than as a
/// number, because "no crossover was found" and "the margin is zero" are
/// different answers.
fn stb_margin_measurements(
    margins: &rspice_core::analysis::stb::StabilityMargins,
) -> Vec<rspice_core::MeasureResult> {
    fn scalar(name: &str, value: f64) -> rspice_core::MeasureResult {
        if value.is_finite() {
            rspice_core::MeasureResult::success(name, value)
        } else {
            rspice_core::MeasureResult::failed(
                name,
                "the loop gain has no such point in the swept band",
            )
        }
    }

    vec![
        scalar("stb_gain_margin_db", margins.gain_margin_db),
        scalar("stb_gain_margin_freq", margins.gain_margin_freq),
        scalar("stb_phase_margin_deg", margins.phase_margin_deg),
        scalar("stb_phase_margin_freq", margins.phase_margin_freq),
        scalar("stb_dc_loop_gain_db", margins.dc_gain_db),
        scalar("stb_unity_gain_bandwidth", margins.unity_gain_bandwidth),
        rspice_core::MeasureResult::success("stb_crossovers", margins.num_crossovers as f64),
        rspice_core::MeasureResult::success(
            "stb_conditionally_stable",
            if margins.conditionally_stable {
                1.0
            } else {
                0.0
            },
        ),
    ]
}

fn stb_sweep_type(sweep: FrequencySweep) -> rspice_core::analysis::stb::StbSweepType {
    match sweep {
        FrequencySweep::Decade => rspice_core::analysis::stb::StbSweepType::Decade,
        FrequencySweep::Octave => rspice_core::analysis::stb::StbSweepType::Octave,
        FrequencySweep::Linear => rspice_core::analysis::stb::StbSweepType::Linear,
    }
}

fn run_pstb(
    netlist: &str,
    source_path: Option<&Path>,
    options: SpecExecutionOptions,
    dependencies: &ResolvedExecutionDependencies,
    abort: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    let pstb_cfg = required_execution_option(
        options.pstb,
        "PSTB analysis requires explicit PSTB execution options",
        abort,
    )?;
    let periodic_state = dependencies.periodic_state().map_err(|error| {
        SimulationError::InvalidConfig(format!("PSTB periodic dependency is unavailable: {error}"))
    })?;
    periodic_state
        .validate_consumer_basis(
            "PSTB",
            pstb_cfg.pss_fundamental_freq,
            pstb_cfg.pss_num_harmonics,
            pstb_cfg.pss_tolerance,
            false,
        )
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;
    let data = super::run_abort_aware_service(abort, || {
        svc_runner::run_pstb_analysis_from_pss_with_source_path_and_abort(
            netlist,
            &pstb_cfg,
            periodic_state.operating_point(),
            source_path,
            abort,
        )
    })?;

    let mut waveforms = HashMap::new();
    super::ensure_not_aborted(abort)?;
    insert_scalar_waveform(
        &mut waveforms,
        "Floquet |lambda|".to_string(),
        data.mode_indices.clone(),
        data.multiplier_magnitude,
        "",
        "mode",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Stability Margin (dB)".to_string(),
        data.mode_indices.clone(),
        data.stability_margin_db,
        "dB",
        "mode",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Mode Damping (1/s)".to_string(),
        data.mode_indices.clone(),
        data.mode_damping,
        "1/s",
        "mode",
    );
    insert_scalar_waveform(
        &mut waveforms,
        "Probe Mode Participation".to_string(),
        data.mode_indices.clone(),
        data.probe_mode_participation,
        "",
        "mode",
    );

    Ok(SimulationResult::Ac {
        frequencies: data.mode_indices,
        waveforms,
        measurements: Vec::new(),
    })
}

fn spectra_to_complex_waveforms(
    spectra: impl IntoIterator<Item = (String, Vec<(f64, f64, f64)>)>,
    abort: &dyn AbortSignal,
) -> Result<HashMap<String, WaveformData>, SimulationError> {
    let mut waveforms = HashMap::new();
    for (name, spectrum) in spectra {
        super::ensure_not_aborted(abort)?;
        let mut frequencies = Vec::with_capacity(spectrum.len());
        let mut real = Vec::with_capacity(spectrum.len());
        let mut imaginary = Vec::with_capacity(spectrum.len());
        for (frequency, magnitude, phase_degrees) in spectrum {
            super::ensure_not_aborted(abort)?;
            let phase = phase_degrees.to_radians();
            frequencies.push(frequency);
            real.push(magnitude * phase.cos());
            imaginary.push(magnitude * phase.sin());
        }
        waveforms.insert(
            name.clone(),
            WaveformData::new_complex(name, frequencies, real, imaginary),
        );
    }
    Ok(waveforms)
}

fn insert_group_delay(
    waveforms: &mut HashMap<String, WaveformData>,
    group_delay: Option<Vec<(f64, f64)>>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    if let Some(gd) = group_delay {
        let mut frequencies = Vec::with_capacity(gd.len());
        let mut delays = Vec::with_capacity(gd.len());
        for (frequency, delay) in gd {
            super::ensure_not_aborted(abort)?;
            frequencies.push(frequency);
            delays.push(delay);
        }
        insert_scalar_waveform(
            waveforms,
            "group_delay".to_string(),
            frequencies,
            delays,
            "s",
            "Hz",
        );
    }
    Ok(())
}

fn complex_waveform(
    name: String,
    frequencies: &[f64],
    values: &[num_complex::Complex64],
    abort: &dyn AbortSignal,
) -> Result<WaveformData, SimulationError> {
    let mut real = Vec::with_capacity(values.len());
    let mut imaginary = Vec::with_capacity(values.len());
    for value in values {
        super::ensure_not_aborted(abort)?;
        real.push(value.re);
        imaginary.push(value.im);
    }
    Ok(WaveformData::new_complex(
        name,
        frequencies.to_vec(),
        real,
        imaginary,
    ))
}

fn required_execution_option<T>(
    option: Option<T>,
    message: &str,
    abort: &dyn AbortSignal,
) -> Result<T, SimulationError> {
    super::ensure_not_aborted(abort)?;
    let option = option.ok_or_else(|| SimulationError::InvalidConfig(message.to_string()));
    super::ensure_not_aborted(abort)?;
    option
}

fn insert_scalar_waveform(
    waveforms: &mut HashMap<String, WaveformData>,
    name: String,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    y_unit: &str,
    _x_unit: &str,
) {
    waveforms.insert(
        name.clone(),
        WaveformData {
            name,
            x_values,
            y_values,
            y_unit: y_unit.to_string(),
            is_complex: false,
            y_imag: None,
        },
    );
}

#[cfg(test)]
mod transfer_function_tests {
    use super::*;
    use crate::simulation::multi_run::{TfAccuracy, TfNormalization};
    use crate::simulation::results::{TransferFunctionQuantity, TransferFunctionScalar};
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    const DIVIDER: &str = "\
structured TF runner
VIN in 0 1
R1 in out 1k
R2 out 0 1k
.end
";

    fn tf_spec() -> AnalysisSpec {
        AnalysisSpec::Tf {
            input_source: "VIN".to_owned(),
            output_expression: "V(out)".to_owned(),
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: TfNormalization::None,
            accuracy: TfAccuracy::Balanced,
        }
    }

    fn assert_finite_scalar_close(
        scalar: Option<TransferFunctionScalar>,
        expected: f64,
        label: &str,
    ) {
        let Some(TransferFunctionScalar::Finite(actual)) = scalar else {
            panic!("{label} must be a retained finite scalar, got {scalar:?}")
        };
        let tolerance = 1.0e-9 * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "{label}: expected {expected:.16e}, got {actual:.16e}"
        );
    }

    #[test]
    fn structured_tf_spec_returns_exact_typed_scalars() {
        let result = run_frequency_spec(
            tf_spec(),
            SpecExecutionOptions::default(),
            DIVIDER,
            None,
            &ResolvedExecutionDependencies::default(),
            &NoAbort,
        )
        .expect("TF runner succeeds");

        let SimulationResult::TransferFunction {
            input_source,
            output_expression,
            input_quantity,
            output_quantity,
            input_unit,
            output_unit,
            normalization,
            accuracy,
            gain,
            input_resistance,
            output_resistance,
            nominal_input,
            nominal_output,
        } = result
        else {
            panic!("expected transfer-function result")
        };
        assert_eq!(input_source, "VIN");
        assert_eq!(output_expression, "V(out)");
        assert_eq!(input_quantity, TransferFunctionQuantity::Voltage);
        assert_eq!(output_quantity, TransferFunctionQuantity::Voltage);
        assert_eq!(input_unit, "V");
        assert_eq!(output_unit, "V");
        assert_eq!(normalization, TfNormalization::None);
        assert_eq!(accuracy, TfAccuracy::Balanced);
        assert_finite_scalar_close(gain, 0.5, "gain");
        assert_finite_scalar_close(input_resistance, 2_000.0, "input resistance");
        assert_finite_scalar_close(output_resistance, 500.0, "output resistance");
        assert_eq!(nominal_input, None);
        assert_eq!(nominal_output, None);
    }

    #[test]
    fn structured_tf_spec_preserves_typed_cancellation() {
        let result = run_frequency_spec(
            tf_spec(),
            SpecExecutionOptions::default(),
            "not a netlist",
            None,
            &ResolvedExecutionDependencies::default(),
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(SimulationError::Aborted)));
    }

    #[test]
    fn scalar_conversion_is_json_safe_for_infinite_resistance() {
        assert_eq!(
            transfer_scalar(f64::INFINITY, "input resistance").unwrap(),
            TransferFunctionScalar::PositiveInfinity
        );
        assert_eq!(
            transfer_scalar(f64::NEG_INFINITY, "output resistance").unwrap(),
            TransferFunctionScalar::NegativeInfinity
        );
        assert!(transfer_scalar(f64::NAN, "input resistance").is_err());
    }
}

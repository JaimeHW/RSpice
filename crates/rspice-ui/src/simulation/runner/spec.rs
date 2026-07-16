use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;

use super::super::engine_bridge::EngineBridge;
use super::super::multi_run::AnalysisSpec;
use super::super::results::SimulationResult;
use super::{SimulationError, SpecExecutionOptions};

mod config;
mod device;
mod frequency;
mod periodic;
mod sweeps;

pub(super) fn run_spec_request(
    bridge: &EngineBridge,
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    abort_flag: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    ensure_not_aborted(abort_flag)?;

    let validation = spec.validate();
    ensure_not_aborted(abort_flag)?;
    validation.map_err(SimulationError::InvalidConfig)?;

    if matches!(spec, AnalysisSpec::Noise { .. }) {
        return Err(SimulationError::InvalidConfig(
            "AnalysisSpec::Noise cannot be executed through the spec runner without losing input source, reference node, sweep type, and temperature; use AnalysisConfig::Noise instead"
                .to_string(),
        ));
    }

    if let Some(config) = config::analysis_config_from_spec(&spec) {
        return bridge.run_with_abort_and_source_path(&config, netlist, source_path, abort_flag);
    }

    match spec {
        AnalysisSpec::MonteCarlo | AnalysisSpec::Parametric | AnalysisSpec::Corner => {
            sweeps::run_sweep_spec(spec, options, netlist, source_path, abort_flag)
        }
        AnalysisSpec::AcData { frequencies, .. } => bridge.run_ac_frequencies_with_source_path(
            netlist,
            source_path,
            frequencies,
            abort_flag,
        ),
        AnalysisSpec::Reliability { .. }
        | AnalysisSpec::Optimization { .. }
        | AnalysisSpec::Soa { .. } => {
            device::run_device_spec(spec, netlist, source_path, abort_flag)
        }
        AnalysisSpec::Pss { .. }
        | AnalysisSpec::HarmonicBalance { .. }
        | AnalysisSpec::Envelope { .. }
        | AnalysisSpec::Fourier { .. }
        | AnalysisSpec::Disto { .. } => {
            periodic::run_periodic_spec(spec, netlist, source_path, abort_flag)
        }
        AnalysisSpec::SParameter { .. }
        | AnalysisSpec::Tf
        | AnalysisSpec::Pac
        | AnalysisSpec::Pxf
        | AnalysisSpec::Pnoise
        | AnalysisSpec::Stb { .. }
        | AnalysisSpec::Pstb => frequency::run_frequency_spec(spec, options, netlist, abort_flag),
        AnalysisSpec::Qpss { .. }
        | AnalysisSpec::Hbsp { .. }
        | AnalysisSpec::Hbnoise { .. }
        | AnalysisSpec::Psp { .. }
        | AnalysisSpec::Qpac { .. }
        | AnalysisSpec::Qpnoise { .. }
        | AnalysisSpec::Qpxf { .. }
        | AnalysisSpec::TransientNoise { .. }
        | AnalysisSpec::DcMismatch { .. } => Err(SimulationError::InvalidConfig(format!(
            "{} execution is unavailable in this engine build; the request was rejected before dispatch",
            spec.run_type().display_name()
        ))),
        AnalysisSpec::DcOp
        | AnalysisSpec::DcSweep { .. }
        | AnalysisSpec::Transient { .. }
        | AnalysisSpec::Ac { .. }
        | AnalysisSpec::Noise { .. }
        | AnalysisSpec::PoleZero { .. }
        | AnalysisSpec::Sensitivity { .. } => Err(config_backed_spec_fallback_error(&spec)),
    }
}

#[inline]
pub(super) fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

/// Execute an abort-aware service without degrading its typed cancellation
/// into an invalid-configuration message.
pub(super) fn run_abort_aware_service<T, F>(
    abort: &dyn AbortSignal,
    run: F,
) -> Result<T, SimulationError>
where
    F: FnOnce() -> svc_runner::ServiceRunResult<T>,
{
    ensure_not_aborted(abort)?;
    let result = run();
    ensure_not_aborted(abort)?;
    result.map_err(translate_service_run_error)
}

fn translate_service_run_error(error: svc_runner::ServiceRunError) -> SimulationError {
    match error {
        svc_runner::ServiceRunError::Aborted => SimulationError::Aborted,
        svc_runner::ServiceRunError::Failure(message) => SimulationError::InvalidConfig(message),
    }
}

fn config_backed_spec_fallback_error(spec: &AnalysisSpec) -> SimulationError {
    SimulationError::InvalidConfig(format!(
        "{} should have been converted to AnalysisConfig before spec dispatch",
        config_backed_spec_name(spec)
    ))
}

fn config_backed_spec_name(spec: &AnalysisSpec) -> &'static str {
    match spec {
        AnalysisSpec::DcOp => "AnalysisSpec::DcOp",
        AnalysisSpec::DcSweep { .. } => "AnalysisSpec::DcSweep",
        AnalysisSpec::Transient { .. } => "AnalysisSpec::Transient",
        AnalysisSpec::Ac { .. } => "AnalysisSpec::Ac",
        AnalysisSpec::Noise { .. } => "AnalysisSpec::Noise",
        AnalysisSpec::PoleZero { .. } => "AnalysisSpec::PoleZero",
        AnalysisSpec::Sensitivity { .. } => "AnalysisSpec::Sensitivity",
        _ => "AnalysisSpec",
    }
}

pub(super) fn misrouted_spec_error(runner: &str, spec: &AnalysisSpec) -> SimulationError {
    SimulationError::InvalidConfig(format!(
        "{} runner received incompatible analysis spec {}",
        runner,
        spec_variant_name(spec)
    ))
}

fn spec_variant_name(spec: &AnalysisSpec) -> &'static str {
    match spec {
        AnalysisSpec::DcOp => "AnalysisSpec::DcOp",
        AnalysisSpec::DcSweep { .. } => "AnalysisSpec::DcSweep",
        AnalysisSpec::Transient { .. } => "AnalysisSpec::Transient",
        AnalysisSpec::Ac { .. } => "AnalysisSpec::Ac",
        AnalysisSpec::AcData { .. } => "AnalysisSpec::AcData",
        AnalysisSpec::Noise { .. } => "AnalysisSpec::Noise",
        AnalysisSpec::PoleZero { .. } => "AnalysisSpec::PoleZero",
        AnalysisSpec::Sensitivity { .. } => "AnalysisSpec::Sensitivity",
        AnalysisSpec::MonteCarlo => "AnalysisSpec::MonteCarlo",
        AnalysisSpec::Parametric => "AnalysisSpec::Parametric",
        AnalysisSpec::Corner => "AnalysisSpec::Corner",
        AnalysisSpec::Reliability { .. } => "AnalysisSpec::Reliability",
        AnalysisSpec::Optimization { .. } => "AnalysisSpec::Optimization",
        AnalysisSpec::Soa { .. } => "AnalysisSpec::Soa",
        AnalysisSpec::Pss { .. } => "AnalysisSpec::Pss",
        AnalysisSpec::HarmonicBalance { .. } => "AnalysisSpec::HarmonicBalance",
        AnalysisSpec::Envelope { .. } => "AnalysisSpec::Envelope",
        AnalysisSpec::Fourier { .. } => "AnalysisSpec::Fourier",
        AnalysisSpec::Disto { .. } => "AnalysisSpec::Disto",
        AnalysisSpec::SParameter { .. } => "AnalysisSpec::SParameter",
        AnalysisSpec::Tf => "AnalysisSpec::Tf",
        AnalysisSpec::Pac => "AnalysisSpec::Pac",
        AnalysisSpec::Pxf => "AnalysisSpec::Pxf",
        AnalysisSpec::Pnoise => "AnalysisSpec::Pnoise",
        AnalysisSpec::Stb { .. } => "AnalysisSpec::Stb",
        AnalysisSpec::Pstb => "AnalysisSpec::Pstb",
        AnalysisSpec::Qpss { .. } => "AnalysisSpec::Qpss",
        AnalysisSpec::Hbsp { .. } => "AnalysisSpec::Hbsp",
        AnalysisSpec::Hbnoise { .. } => "AnalysisSpec::Hbnoise",
        AnalysisSpec::Psp { .. } => "AnalysisSpec::Psp",
        AnalysisSpec::Qpac { .. } => "AnalysisSpec::Qpac",
        AnalysisSpec::Qpnoise { .. } => "AnalysisSpec::Qpnoise",
        AnalysisSpec::Qpxf { .. } => "AnalysisSpec::Qpxf",
        AnalysisSpec::TransientNoise { .. } => "AnalysisSpec::TransientNoise",
        AnalysisSpec::DcMismatch { .. } => "AnalysisSpec::DcMismatch",
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }

        fn poll_count(&self) -> usize {
            self.polls.load(Ordering::Relaxed)
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    struct TempDeckDir {
        path: PathBuf,
    }

    impl TempDeckDir {
        fn new() -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "rspice-ui-spec-source-path-{}-{nonce}",
                std::process::id()
            ));
            fs::create_dir_all(&path).expect("temp deck dir");
            Self { path }
        }

        fn deck_path(&self) -> PathBuf {
            self.path.join("deck.cir")
        }
    }

    impl Drop for TempDeckDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn parametric_spec_resolves_relative_includes_from_source_path() {
        let temp = TempDeckDir::new();
        fs::write(temp.path.join("params.inc"), ".param rload=1k\n").expect("include write");
        fs::write(temp.deck_path(), "").expect("deck path placeholder");
        let netlist = "relative include parametric\n\
.include \"params.inc\"\n\
V1 out 0 1\n\
R1 out 0 {rload}\n\
.step param rload 1k 2k 1k\n\
.end\n";

        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::Parametric,
            SpecExecutionOptions::default(),
            netlist,
            Some(&temp.deck_path()),
            &rspice_core::abort_signal::NoAbort,
        );

        assert!(
            result.is_ok(),
            "relative include should resolve: {result:?}"
        );
    }

    #[test]
    fn spec_noise_request_rejects_lossy_config_fallback() {
        let netlist = "noise spec fallback\n\
V1 in 0 AC 1\n\
R1 in out 1k\n\
R2 out 0 1k\n\
.end\n";

        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::Noise {
                output_node: "out".to_string(),
                start_freq: 1.0,
                stop_freq: 1.0e6,
                points_per_decade: 10,
                temperature: 350.0,
            },
            SpecExecutionOptions::default(),
            netlist,
            None,
            &rspice_core::abort_signal::NoAbort,
        );

        match result {
            Err(SimulationError::InvalidConfig(message)) => {
                assert!(message.contains("AnalysisSpec::Noise"));
                assert!(message.contains("AnalysisConfig::Noise"));
            }
            other => panic!("expected invalid config for lossy noise spec fallback, got {other:?}"),
        }
    }

    #[test]
    fn unavailable_manifest_spec_is_rejected_before_engine_dispatch() {
        let spec = AnalysisSpec::DcMismatch {
            output_expression: "V(out)".to_owned(),
            sigma_multiplier: 3.0,
            contributor_limit: 25,
            include_process: false,
            include_mismatch: true,
            normalized_contributions: true,
        };
        let result = run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            "blocked preview\nV1 out 0 1\n.end\n",
            None,
            &rspice_core::abort_signal::NoAbort,
        );
        match result {
            Err(SimulationError::InvalidConfig(message)) => {
                assert!(message.contains("DC Mismatch Contribution"));
                assert!(message.contains("unavailable"));
                assert!(message.contains("rejected before dispatch"));
            }
            other => panic!("expected fail-closed capability rejection, got {other:?}"),
        }
    }

    #[test]
    fn pole_zero_spec_rejects_invalid_analysis_type_before_config_fallback() {
        let netlist = "invalid pz spec\n\
V1 in 0 DC 1 AC 1\n\
R1 in out 1k\n\
R2 out 0 1k\n\
.end\n";

        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::PoleZero {
                input_node: "in".to_string(),
                input_ref: "0".to_string(),
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                transfer_type: "VOL".to_string(),
                analysis_type: "NOT_A_MODE".to_string(),
            },
            SpecExecutionOptions::default(),
            netlist,
            None,
            &rspice_core::abort_signal::NoAbort,
        );

        match result {
            Err(SimulationError::InvalidConfig(message)) => {
                assert!(message.contains("analysis_type"));
                assert!(message.contains("PZ, POL, or ZER"));
            }
            other => panic!("expected invalid PZ analysis type, got {other:?}"),
        }
    }

    #[test]
    fn config_backed_specs_have_explicit_config_mapping() {
        for spec in config_backed_specs() {
            assert!(
                config::analysis_config_from_spec(&spec).is_some(),
                "config-backed spec must be converted before dispatch: {spec:?}"
            );
        }
    }

    #[test]
    fn config_backed_fallback_reports_invalid_config() {
        let mut specs = config_backed_specs();
        specs.push(AnalysisSpec::Noise {
            output_node: "out".to_string(),
            start_freq: 1.0,
            stop_freq: 1.0e6,
            points_per_decade: 10,
            temperature: 300.0,
        });

        for spec in specs {
            match config_backed_spec_fallback_error(&spec) {
                SimulationError::InvalidConfig(message) => {
                    assert!(
                        message.contains(config_backed_spec_name(&spec)),
                        "fallback error should name the spec variant: {message}"
                    );
                    assert!(
                        message.contains("AnalysisConfig"),
                        "fallback error should tell callers which path to use: {message}"
                    );
                }
                other => panic!("expected InvalidConfig for {spec:?}, got {other:?}"),
            }
        }
    }

    #[test]
    fn specialized_runners_report_invalid_config_for_misrouted_specs() {
        let cases = [
            (
                "sweep",
                sweeps::run_sweep_spec(
                    AnalysisSpec::DcOp,
                    SpecExecutionOptions::default(),
                    "",
                    None,
                    &rspice_core::abort_signal::NoAbort,
                ),
                "AnalysisSpec::DcOp",
            ),
            (
                "device",
                device::run_device_spec(
                    AnalysisSpec::DcOp,
                    "",
                    None,
                    &rspice_core::abort_signal::NoAbort,
                ),
                "AnalysisSpec::DcOp",
            ),
            (
                "periodic",
                periodic::run_periodic_spec(
                    AnalysisSpec::DcOp,
                    "",
                    None,
                    &rspice_core::abort_signal::NoAbort,
                ),
                "AnalysisSpec::DcOp",
            ),
            (
                "frequency",
                frequency::run_frequency_spec(
                    AnalysisSpec::DcOp,
                    SpecExecutionOptions::default(),
                    "",
                    &rspice_core::abort_signal::NoAbort,
                ),
                "AnalysisSpec::DcOp",
            ),
        ];

        for (runner, result, variant) in cases {
            match result {
                Err(SimulationError::InvalidConfig(message)) => {
                    assert!(
                        message.contains(runner),
                        "message should name the runner: {message}"
                    );
                    assert!(
                        message.contains(variant),
                        "message should name the misrouted spec: {message}"
                    );
                }
                other => panic!("expected InvalidConfig from {runner} runner, got {other:?}"),
            }
        }
    }

    #[test]
    fn specialized_spec_families_observe_the_supplied_counter_signal() {
        let cases = [
            ("sweep", AnalysisSpec::MonteCarlo),
            (
                "device",
                AnalysisSpec::Reliability {
                    target_years: vec![1.0],
                    enable_hci: true,
                    enable_nbti: false,
                    enable_em: false,
                    min_stress_voltage: 0.0,
                },
            ),
            (
                "periodic",
                AnalysisSpec::Pss {
                    fundamental_freq: 1.0e6,
                    num_harmonics: 3,
                    tolerance: 1.0e-6,
                },
            ),
            ("frequency", AnalysisSpec::Tf),
        ];

        for (family, spec) in cases {
            let signal = AbortOnPoll::new(3);
            let result = run_spec_request(
                &EngineBridge::new(),
                spec,
                SpecExecutionOptions::default(),
                "cancellation boundary\n.end\n",
                None,
                &signal,
            );

            match result {
                Err(SimulationError::Aborted) => {}
                Err(SimulationError::InvalidConfig(message)) => panic!(
                    "{family} cancellation was incorrectly reported as InvalidConfig: {message}"
                ),
                Err(other) => panic!("{family} expected typed Aborted, got {other}"),
                Ok(_) => panic!("{family} should have observed cancellation"),
            }
            assert!(
                signal.poll_count() >= 3,
                "{family} dispatcher did not poll the supplied signal"
            );
        }
    }

    #[test]
    fn typed_service_abort_maps_to_runner_abort() {
        let result: Result<(), SimulationError> =
            run_abort_aware_service(&rspice_core::abort_signal::NoAbort, || {
                Err(svc_runner::ServiceRunError::Aborted)
            });

        assert!(matches!(result, Err(SimulationError::Aborted)));
    }

    #[test]
    fn typed_service_failure_maps_to_invalid_config() {
        let result: Result<(), SimulationError> =
            run_abort_aware_service(&rspice_core::abort_signal::NoAbort, || {
                Err(svc_runner::ServiceRunError::Failure(
                    "bad input".to_string(),
                ))
            });

        assert!(matches!(
            result,
            Err(SimulationError::InvalidConfig(message)) if message == "bad input"
        ));
    }

    #[test]
    fn ac_data_dispatch_threads_cancellation_into_the_frequency_solver_path() {
        let signal = AbortOnPoll::new(5);
        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::AcData {
                table_name: "measured".to_string(),
                frequencies: vec![1.0, 10.0, 100.0],
            },
            SpecExecutionOptions::default(),
            "AC DATA cancellation\n\
             Vin in 0 AC 1\n\
             R1 in out 1k\n\
             C1 out 0 1n\n\
             .end\n",
            None,
            &signal,
        );

        match result {
            Err(SimulationError::Aborted) => {}
            Err(SimulationError::InvalidConfig(message)) => {
                panic!("AC DATA cancellation became InvalidConfig: {message}")
            }
            Err(other) => panic!("AC DATA expected typed Aborted, got {other}"),
            Ok(_) => panic!("AC DATA should have observed cancellation"),
        }
        assert!(signal.poll_count() >= 5);
    }

    fn config_backed_specs() -> Vec<AnalysisSpec> {
        vec![
            AnalysisSpec::DcOp,
            AnalysisSpec::DcSweep {
                source_name: "V1".to_string(),
                start: 0.0,
                stop: 1.0,
                step: 0.1,
                source2: None,
                start2: None,
                stop2: None,
                step2: None,
            },
            AnalysisSpec::Transient {
                stop_time: 1.0e-6,
                step_time: 1.0e-9,
                start_time: 0.0,
                max_timestep: None,
                uic: false,
            },
            AnalysisSpec::Ac {
                start_freq: 1.0,
                stop_freq: 1.0e6,
                points_per_unit: 10,
                sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            },
            AnalysisSpec::PoleZero {
                input_node: "in".to_string(),
                input_ref: "0".to_string(),
                output_node: "out".to_string(),
                output_ref: "0".to_string(),
                transfer_type: "VOL".to_string(),
                analysis_type: "PZ".to_string(),
            },
            AnalysisSpec::Sensitivity {
                output_var: "V(out)".to_string(),
                ac_mode: false,
                frequency: None,
            },
        ]
    }
}

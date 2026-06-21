use std::path::Path;

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
    abort_flag: &dyn rspice_core::abort_signal::AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    if abort_flag.is_aborted() {
        return Err(SimulationError::Aborted);
    }

    spec.validate().map_err(SimulationError::InvalidConfig)?;

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
            sweeps::run_sweep_spec(spec, options, netlist, source_path)
        }
        AnalysisSpec::Reliability { .. }
        | AnalysisSpec::Optimization { .. }
        | AnalysisSpec::Soa { .. } => device::run_device_spec(spec, netlist),
        AnalysisSpec::Pss { .. }
        | AnalysisSpec::HarmonicBalance { .. }
        | AnalysisSpec::Envelope { .. }
        | AnalysisSpec::Fourier { .. }
        | AnalysisSpec::Disto { .. } => periodic::run_periodic_spec(spec, netlist),
        AnalysisSpec::SParameter { .. }
        | AnalysisSpec::Tf
        | AnalysisSpec::Pac
        | AnalysisSpec::Pxf
        | AnalysisSpec::Pnoise
        | AnalysisSpec::Stb { .. }
        | AnalysisSpec::Pstb => frequency::run_frequency_spec(spec, options, netlist),
        AnalysisSpec::DcOp
        | AnalysisSpec::DcSweep { .. }
        | AnalysisSpec::Transient { .. }
        | AnalysisSpec::Ac { .. }
        | AnalysisSpec::Noise { .. }
        | AnalysisSpec::PoleZero { .. }
        | AnalysisSpec::Sensitivity { .. } => Err(config_backed_spec_fallback_error(&spec)),
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
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

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
                ),
                "AnalysisSpec::DcOp",
            ),
            (
                "device",
                device::run_device_spec(AnalysisSpec::DcOp, ""),
                "AnalysisSpec::DcOp",
            ),
            (
                "periodic",
                periodic::run_periodic_spec(AnalysisSpec::DcOp, ""),
                "AnalysisSpec::DcOp",
            ),
            (
                "frequency",
                frequency::run_frequency_spec(
                    AnalysisSpec::DcOp,
                    SpecExecutionOptions::default(),
                    "",
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

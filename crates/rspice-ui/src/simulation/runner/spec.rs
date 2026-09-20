//! Routing an analysis specification to its runner.
//!
//! Dispatches a validated [`AnalysisSpec`] to the service that executes it,
//! and wraps every call so an abort signal is observed between stages rather
//! than only at the end of a long run.

use std::path::Path;

use rspice_core::abort_signal::AbortSignal;

use crate::services::simulation_runner as svc_runner;

use super::super::engine_bridge::{EngineBridge, SupplyCornerScale};
use super::super::execution::ResolvedExecutionDependencies;
use super::super::multi_run::AnalysisSpec;
use super::super::results::SimulationResult;
use super::{AnalysisExecutionEnvironment, SimulationError, SpecExecutionOptions};

mod config;
mod device;
mod frequency;
mod periodic;
mod recorded_fft;
mod sweeps;

#[cfg(test)]
pub(super) fn run_spec_request(
    bridge: &EngineBridge,
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    dependencies: &ResolvedExecutionDependencies,
    abort_flag: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    run_spec_request_with_environment(
        bridge,
        spec,
        options,
        netlist,
        source_path,
        dependencies,
        None,
        abort_flag,
    )
}

pub(super) fn run_spec_request_with_environment(
    bridge: &EngineBridge,
    spec: AnalysisSpec,
    options: SpecExecutionOptions,
    netlist: &str,
    source_path: Option<&Path>,
    dependencies: &ResolvedExecutionDependencies,
    environment: Option<AnalysisExecutionEnvironment>,
    abort_flag: &dyn AbortSignal,
) -> Result<SimulationResult, SimulationError> {
    ensure_not_aborted(abort_flag)?;

    let validation = spec.validate();
    ensure_not_aborted(abort_flag)?;
    validation.map_err(SimulationError::InvalidConfig)?;
    dependencies
        .validate_for_spec(&spec, &options)
        .map_err(|error| SimulationError::InvalidConfig(error.to_string()))?;

    if let Some(config) = config::analysis_config_from_spec(&spec) {
        return match environment {
            Some(environment) => bridge.run_with_abort_and_source_path_and_environment(
                &config,
                netlist,
                source_path,
                Some(environment),
                abort_flag,
            ),
            None => bridge.run_with_abort_and_source_path(
                &config,
                netlist,
                source_path,
                point_scoped_supply_corner(&spec, &options),
                abort_flag,
            ),
        };
    }

    match spec {
        AnalysisSpec::MonteCarlo { .. } | AnalysisSpec::Parametric | AnalysisSpec::Corner => {
            sweeps::run_sweep_spec(spec, options, netlist, source_path, environment, abort_flag)
        }
        AnalysisSpec::Fft { request } => recorded_fft::run(&request, dependencies, abort_flag),
        AnalysisSpec::AcData {
            table_name,
            frequencies,
        } => bridge.run_ac_data_with_source_path(
            netlist,
            source_path,
            &table_name,
            frequencies,
            abort_flag,
        ),
        AnalysisSpec::Optimization { .. }
        | AnalysisSpec::Soa { .. }
        | AnalysisSpec::DcMismatch { .. } => {
            device::run_device_spec(spec, netlist, source_path, abort_flag)
        }
        AnalysisSpec::Pss { .. }
        | AnalysisSpec::PssSpectrum { .. }
        | AnalysisSpec::HarmonicBalance { .. }
        | AnalysisSpec::Envelope { .. }
        | AnalysisSpec::Fourier { .. }
        | AnalysisSpec::Disto { .. }
        | AnalysisSpec::Hbsp { .. }
        | AnalysisSpec::Hbnoise { .. }
        | AnalysisSpec::Psp { .. } => {
            periodic::run_periodic_spec(spec, netlist, source_path, dependencies, abort_flag)
        }
        AnalysisSpec::SParameter { .. }
        | AnalysisSpec::Tf { .. }
        | AnalysisSpec::Pac
        | AnalysisSpec::Pxf
        | AnalysisSpec::Pnoise
        | AnalysisSpec::Stb { .. }
        | AnalysisSpec::Pstb => frequency::run_frequency_spec(
            spec,
            options,
            netlist,
            source_path,
            dependencies,
            abort_flag,
        ),
        blocked @ (AnalysisSpec::Qpss { .. }
        | AnalysisSpec::Qpac { .. }
        | AnalysisSpec::Qpnoise { .. }
        | AnalysisSpec::Qpxf { .. }
        | AnalysisSpec::Reliability { .. }) => {
            let kind = crate::simulation::execution::canonical_analysis_kind(&blocked);
            let reason = kind
                .execution_blocker()
                .unwrap_or("the selected execution capability is unavailable in this engine build");
            Err(SimulationError::InvalidConfig(format!(
                "{} execution is unavailable; the request was rejected before dispatch: {reason}",
                blocked.run_type().display_name()
            )))
        }
        AnalysisSpec::LegacyDcOp
        | AnalysisSpec::DcOp { .. }
        | AnalysisSpec::DcSweep { .. }
        | AnalysisSpec::Transient { .. }
        | AnalysisSpec::TransientNoise { .. }
        | AnalysisSpec::Ac { .. }
        | AnalysisSpec::Noise { .. }
        | AnalysisSpec::PoleZero { .. }
        | AnalysisSpec::Sensitivity { .. } => Err(config_backed_spec_routing_error(&spec)),
    }
}

/// The supply corner a request scoped to one PVT point must apply.
///
/// A corner contract narrowed to exactly one point is how a corner run's base
/// analysis says which point it is; anything wider is a declared space, not a
/// point, and scaling the deck's supplies by one of its values would be a
/// fabrication. The operating point is excluded because its own run-point
/// contract already carries the supply and applying it here as well would
/// square the scale.
fn point_scoped_supply_corner(
    spec: &AnalysisSpec,
    options: &SpecExecutionOptions,
) -> Option<SupplyCornerScale> {
    if matches!(spec, AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }) {
        return None;
    }
    let contract = options.corner.as_ref()?;
    let [point] = contract.points.as_slice() else {
        return None;
    };
    Some(SupplyCornerScale {
        corner_voltage: point.voltage,
        nominal_voltage: contract.nominal_voltage?,
        supply_source_names: contract.supply_source_names.clone(),
    })
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
        svc_runner::ServiceRunError::ResourceLimit(error) => SimulationError::ResourceLimit {
            resource: error.resource.as_str().to_string(),
            requested: error.requested,
            limit: error.limit,
        },
        svc_runner::ServiceRunError::Failure(message) => SimulationError::InvalidConfig(message),
    }
}

fn config_backed_spec_routing_error(spec: &AnalysisSpec) -> SimulationError {
    SimulationError::InvalidConfig(format!(
        "{} should have been converted to AnalysisConfig before spec dispatch",
        config_backed_spec_name(spec)
    ))
}

fn config_backed_spec_name(spec: &AnalysisSpec) -> &'static str {
    match spec {
        AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. } => "AnalysisSpec::DcOp",
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
        AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. } => "AnalysisSpec::DcOp",
        AnalysisSpec::DcSweep { .. } => "AnalysisSpec::DcSweep",
        AnalysisSpec::Transient { .. } => "AnalysisSpec::Transient",
        AnalysisSpec::Ac { .. } => "AnalysisSpec::Ac",
        AnalysisSpec::AcData { .. } => "AnalysisSpec::AcData",
        AnalysisSpec::Noise { .. } => "AnalysisSpec::Noise",
        AnalysisSpec::PoleZero { .. } => "AnalysisSpec::PoleZero",
        AnalysisSpec::Sensitivity { .. } => "AnalysisSpec::Sensitivity",
        AnalysisSpec::MonteCarlo { .. } => "AnalysisSpec::MonteCarlo",
        AnalysisSpec::Parametric => "AnalysisSpec::Parametric",
        AnalysisSpec::Corner => "AnalysisSpec::Corner",
        AnalysisSpec::Reliability { .. } => "AnalysisSpec::Reliability",
        AnalysisSpec::Optimization { .. } => "AnalysisSpec::Optimization",
        AnalysisSpec::Soa { .. } => "AnalysisSpec::Soa",
        AnalysisSpec::Pss { .. } => "AnalysisSpec::Pss",
        AnalysisSpec::PssSpectrum { .. } => "AnalysisSpec::PssSpectrum",
        AnalysisSpec::HarmonicBalance { .. } => "AnalysisSpec::HarmonicBalance",
        AnalysisSpec::Envelope { .. } => "AnalysisSpec::Envelope",
        AnalysisSpec::Fourier { .. } => "AnalysisSpec::Fourier",
        AnalysisSpec::Fft { .. } => "AnalysisSpec::Fft",
        AnalysisSpec::Disto { .. } => "AnalysisSpec::Disto",
        AnalysisSpec::SParameter { .. } => "AnalysisSpec::SParameter",
        AnalysisSpec::Tf { .. } => "AnalysisSpec::Tf",
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
    mod periodic_port_noise;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;
    use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
    use crate::simulation::execution::{ExecutionArtifactEnvelope, PreparedDependencyBinding};
    use crate::simulation::multi_run::{
        EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve, HbToneSpec,
        SpPort,
    };

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    #[test]
    fn hierarchical_single_port_spec_retains_the_solved_reference() {
        let spec = AnalysisSpec::SParameter {
            do_noise: false,
            start_freq: 1e6,
            stop_freq: 3e6,
            points_per_unit: 3,
            sweep: crate::simulation::multi_run::FrequencySweep::Linear,
            z0: 50.0,
            ports: Vec::new(),
        };
        assert!(spec.validate().is_ok());
        let result = run_spec_request(
            &EngineBridge::new(), spec, SpecExecutionOptions::default(),
            "* Scoped single port\n.subckt generator a b params: reference=75\nP1 a b portnum=1 z0={reference}\n.ends generator\nX1 p 0 generator\nR1 p 0 100\n.end\n",
            None, &ResolvedExecutionDependencies::default(), &rspice_core::NoAbort,
        ).unwrap();
        let SimulationResult::Ac {
            frequencies,
            waveforms,
            reference_impedances_ohm,
            ..
        } = result
        else {
            panic!("SP must retain frequency-domain data");
        };
        assert_eq!(frequencies, [1e6, 2e6, 3e6]);
        assert_eq!(reference_impedances_ohm, Some(vec![75.0]));
        assert_eq!(waveforms.len(), 1);
        assert!(
            waveforms["S11"]
                .y_values
                .iter()
                .all(|value| (*value - 1.0 / 7.0).abs() < 1e-12)
        );
    }

    fn hb_producer_spec() -> AnalysisSpec {
        AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1.0e6, 8)],
            reltol: 2.5e-7,
            abstol: 1.0e-12,
            max_iterations: 100,
            damping: 1.0,
            min_damping: 0.01,
            oversample: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
            use_exact_jacobian: true,
            verbose: false,
        }
    }

    fn transferred_hb_dependencies(netlist: &str) -> ResolvedExecutionDependencies {
        let producer_spec = hb_producer_spec();
        let result = run_spec_request(
            &EngineBridge::new(),
            producer_spec.clone(),
            SpecExecutionOptions::default(),
            netlist,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("the HB producer reaches the periodic dispatcher");
        let producer = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let snapshot = digest(0xb1);
        let config_digest = digest(0xb2);
        let artifact = ExecutionArtifactEnvelope::from_hb_result(
            snapshot,
            producer,
            revision,
            config_digest,
            &producer_spec,
            &result,
        )
        .expect("the HB result forms a typed dependency")
        .expect("HB always retains its numerical state");
        let resolved = ResolvedExecutionDependencies::resolve(
            snapshot,
            vec![PreparedDependencyBinding::hb_state(
                producer,
                revision,
                config_digest,
            )],
            &HashMap::from([(producer, artifact)]),
        )
        .expect("the exact producer binding resolves");
        let (metadata, buffers) = resolved
            .encode_transfer()
            .expect("HB state serializes for worker transport");
        ResolvedExecutionDependencies::decode_transfer(&metadata, buffers)
            .expect("HB state authenticates after worker transport")
    }

    fn pss_producer_spec() -> AnalysisSpec {
        AnalysisSpec::Pss {
            method: crate::simulation::multi_run::PssMethod::Shooting,
            fundamental_freq: 1.0e6,
            tone_sources: vec!["P1".to_owned(), "P2".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            tolerance: 1.0e-7,
            oscillator_mode: false,
            oscillator_node: None,
            num_harmonics: 8,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        }
    }

    fn transferred_pss_dependencies(netlist: &str) -> ResolvedExecutionDependencies {
        let producer_spec = pss_producer_spec();
        let produced = svc_runner::run_pss_analysis_with_source_path_and_abort(
            netlist,
            1.0e6,
            8,
            1.0e-7,
            None,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("the PSS producer converges through the resolved service path");
        let result = SimulationResult::Transient {
            spectra: Vec::new(),
            time: produced.time,
            waveforms: HashMap::new(),
            measurements: Vec::new(),
            periodic_state: Some(produced.operating_point),
            convergence: Default::default(),
            events: Default::default(),
        };
        let producer = AnalysisInstanceId::new();
        let revision = ObjectRevision::INITIAL;
        let snapshot = digest(0xc1);
        let config_digest = digest(0xc2);
        let artifact = ExecutionArtifactEnvelope::from_periodic_result(
            snapshot,
            producer,
            revision,
            config_digest,
            &producer_spec,
            &result,
        )
        .expect("the PSS result forms a typed dependency")
        .expect("PSS retains its numerical periodic state");
        let resolved = ResolvedExecutionDependencies::resolve(
            snapshot,
            vec![PreparedDependencyBinding::periodic_state(
                producer,
                revision,
                config_digest,
            )],
            &HashMap::from([(producer, artifact)]),
        )
        .expect("the exact producer binding resolves");
        let (metadata, buffers) = resolved
            .encode_transfer()
            .expect("PSS state serializes for worker transport");
        ResolvedExecutionDependencies::decode_transfer(&metadata, buffers)
            .expect("PSS state authenticates after worker transport")
    }

    fn periodic_ports() -> Vec<SpPort> {
        vec![
            SpPort {
                node_pos: "p1".to_owned(),
                node_neg: "0".to_owned(),
                z0: Some(50.0),
            },
            SpPort {
                node_pos: "p2".to_owned(),
                node_neg: "0".to_owned(),
                z0: Some(50.0),
            },
        ]
    }

    fn blocked_preview_specs() -> Vec<AnalysisSpec> {
        vec![
            AnalysisSpec::Qpss {
                tones: vec![HbToneSpec::new(1.0e6, 3), HbToneSpec::new(1.1e6, 3)],
                max_iterations: 40,
                relative_tolerance: 1.0e-6,
                autonomous: false,
                oscillator_node: None,
            },
            AnalysisSpec::Qpac {
                start_freq: 1.0e3,
                stop_freq: 2.0e3,
                points_per_unit: 2,
                sweep: crate::simulation::multi_run::FrequencySweep::Linear,
                input_source: "V1".to_owned(),
                output_node: "out".to_owned(),
                output_ref: "0".to_owned(),
                input_lattice: [0, 0],
                output_lattice: [0, 0],
            },
            AnalysisSpec::Qpnoise {
                start_freq: 1.0e3,
                stop_freq: 2.0e3,
                points_per_unit: 2,
                sweep: crate::simulation::multi_run::FrequencySweep::Linear,
                output_node: "out".to_owned(),
                output_ref: "0".to_owned(),
                input_source: "V1".to_owned(),
                lattice_min: [-1, -1],
                lattice_max: [1, 1],
                integrated_noise: true,
                contributor_ranking: true,
            },
            AnalysisSpec::Qpxf {
                start_freq: 1.0e3,
                stop_freq: 2.0e3,
                points_per_unit: 2,
                sweep: crate::simulation::multi_run::FrequencySweep::Linear,
                input_source: "V1".to_owned(),
                output_node: "out".to_owned(),
                output_ref: "0".to_owned(),
                input_lattice: [0, 0],
                output_lattice: [0, 0],
                group_delay: true,
            },
            AnalysisSpec::Reliability {
                target_years: vec![1.0, 10.0],
                enable_hci: true,
                enable_nbti: true,
                enable_em: false,
                min_stress_voltage: 0.1,
            },
        ]
    }

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
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        );

        assert!(
            result.is_ok(),
            "relative include should resolve: {result:?}"
        );
    }

    #[test]
    fn frequency_specs_resolve_relative_includes_from_source_path() {
        let temp = TempDeckDir::new();
        fs::write(temp.deck_path(), "").expect("deck path placeholder");
        let top = "relative include frequency analysis\n\
.include \"network.inc\"\n\
.end\n";
        let run = |spec| {
            run_spec_request(
                &EngineBridge::new(),
                spec,
                SpecExecutionOptions::default(),
                top,
                Some(&temp.deck_path()),
                &ResolvedExecutionDependencies::default(),
                &rspice_core::abort_signal::NoAbort,
            )
        };

        fs::write(temp.path.join("network.inc"), "R1 IN OUT 50\nR2 OUT 0 50\n")
            .expect("S-parameter include write");
        let sparameter = run(AnalysisSpec::SParameter {
            do_noise: false,
            start_freq: 1.0e6,
            stop_freq: 1.0e7,
            points_per_unit: 1,
            sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            z0: 50.0,
            ports: vec![
                crate::simulation::multi_run::SpPort {
                    node_pos: "IN".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                },
                crate::simulation::multi_run::SpPort {
                    node_pos: "OUT".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: None,
                },
            ],
        });
        assert!(
            matches!(sparameter, Ok(SimulationResult::Ac { .. })),
            "S-parameter relative include should resolve: {sparameter:?}"
        );

        fs::write(
            temp.path.join("network.inc"),
            "VIN IN 0 1\nR1 IN OUT 1k\nR2 OUT 0 2k\n",
        )
        .expect("TF include write");
        let transfer = run(AnalysisSpec::Tf {
            input_source: "VIN".to_owned(),
            output_expression: "V(OUT)".to_owned(),
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: crate::simulation::multi_run::TfNormalization::None,
            accuracy: crate::simulation::multi_run::TfAccuracy::Balanced,
        });
        assert!(
            matches!(transfer, Ok(SimulationResult::TransferFunction { .. })),
            "TF relative include should resolve: {transfer:?}"
        );

        fs::write(
            temp.path.join("network.inc"),
            "E1 EO 0 CTRL 0 -1000\n\
VPROBE EO X 0\n\
R1 X CTRL 1k\n\
C1 CTRL 0 159.154943091895n\n",
        )
        .expect("STB include write");
        let stability = run(AnalysisSpec::Stb {
            probe_node: "VPROBE".to_owned(),
            start_freq: 10.0,
            stop_freq: 1.0e4,
            sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            points_per_decade: 2,
            compute_nyquist: false,
        });
        assert!(
            matches!(stability, Ok(SimulationResult::Ac { .. })),
            "STB relative include should resolve: {stability:?}"
        );
    }

    /// A temperature step is solved one declared point at a time, so nothing
    /// solves the declaration. Refusing it here is what keeps the sweep from
    /// costing 2N: an executor that accepted it would quietly solve the whole
    /// space a second time and the duplicate would look like a slow run rather
    /// than a routing fault.
    #[test]
    fn a_temperature_step_is_refused_by_the_executor_rather_than_solved_whole() {
        let netlist = "temperature step routing\n\
V1 in 0 1\n\
R1 in out 1k\n\
R2 out 0 1k\n\
.end\n";

        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::Parametric,
            SpecExecutionOptions {
                temp: Some(crate::services::simulation_runner::TempRunConfig {
                    temperatures_c: vec![-40.0, 27.0],
                    base_mode: crate::services::simulation_runner::CornerBaseMode::Op,
                }),
                ..SpecExecutionOptions::default()
            },
            netlist,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        );

        let error = result.expect_err("a declaration must never reach an executor");
        assert!(
            error.to_string().contains("one declared point at a time"),
            "{error}"
        );
    }

    #[test]
    fn exact_noise_spec_executes_without_live_setup_dependency() {
        let netlist = "exact noise spec\n\
V1 in 0 AC 1\n\
R1 in out 1k\n\
R2 out 0 1k\n\
.end\n";

        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::Noise {
                output_node: "out".to_string(),
                reference_node: "0".to_string(),
                input_source: "V1".to_string(),
                start_freq: 1.0,
                stop_freq: 1.0e6,
                points_per_decade: 10,
                sweep: crate::simulation::config::NoiseSweepType::Decade,
                explicit_frequencies: None,
                data_table_name: None,
                contribution_detail: crate::simulation::config::NoiseContributionDetail::Top50,
                integration_mode: crate::simulation::config::NoiseIntegrationMode::Enabled,
                temperature: 350.0,
            },
            SpecExecutionOptions::default(),
            netlist,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        );

        assert!(matches!(result, Ok(SimulationResult::Noise { .. })));
    }

    #[test]
    fn runnable_preview_envelope_reaches_dispatch_and_returns_complex_slow_time_data() {
        let spec = AnalysisSpec::Envelope {
            initialization: Default::default(),
            fundamental_freq: 1.0e3,
            additional_carrier_tones: Vec::new(),
            stop_time: 2.0e-3,
            num_harmonics: 1,
            envelope_step: Some(2.5e-4),
            modulation_sources: Vec::new(),
            initial_periodic_solve: EnvelopeInitialPeriodicSolve::TransientSpectralEstimate,
            adaptive_mode: EnvelopeAdaptiveMode::FixedEnvelopeStep,
            extraction_path: EnvelopeExtractionPath::Projection,
        };
        assert_eq!(
            crate::simulation::execution::canonical_analysis_kind(&spec).execution_blocker(),
            None
        );

        let result = run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            "preview envelope dispatch\n\
             V1 in 0 SIN(0 1 1k)\n\
             R1 in out 1k\n\
             C1 out 0 100n\n\
             .end\n",
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("the runnable preview Envelope spec reaches its solver");

        let SimulationResult::Transient {
            time, waveforms, ..
        } = result
        else {
            panic!("Envelope must retain a slow-time transient result family");
        };
        assert!(!time.is_empty());
        let envelope = waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("ENV(V(out))"))
            .map(|(_, waveform)| waveform)
            .expect("the dispatched result retains the output envelope");
        assert_eq!(envelope.x_values, time);
        assert_eq!(envelope.y_imag.as_ref().map(Vec::len), Some(time.len()));
    }

    #[test]
    fn runnable_preview_hb_dependents_dispatch_from_authenticated_worker_state() {
        let netlist = "preview HB dependent dispatch\n\
                       .subckt ports a b\n\
                       P1 a 0 PORT=1 Z0=50\n\
                       P2 b 0 PORT=2 Z0=50\n\
                       .ends\n\
                       XP p1 p2 ports\n\
                       R1 p1 p2 50\n\
                       C1 p1 0 1e-18\n\
                       VIN bias 0 0\n\
                       RNOISE bias p2 1k\n\
                       .end\n";
        let dependencies = transferred_hb_dependencies(netlist);

        let hbsp = AnalysisSpec::Hbsp {
            start_freq: 1.0e4,
            stop_freq: 2.0e4,
            points_per_unit: 2,
            sweep: crate::simulation::multi_run::FrequencySweep::Linear,
            ports: periodic_ports(),
            max_sideband: 1,
            mixed_mode: false,
            noise_parameters: false,
            noise_reference: None,
        };
        assert_eq!(
            crate::simulation::execution::canonical_analysis_kind(&hbsp).execution_blocker(),
            None
        );
        let hbsp_result = run_spec_request(
            &EngineBridge::new(),
            hbsp,
            SpecExecutionOptions::default(),
            netlist,
            None,
            &dependencies,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("HBSP consumes the authenticated transferred HB state");
        let SimulationResult::Ac {
            frequencies,
            waveforms,
            reference_impedances_ohm,
            ..
        } = hbsp_result
        else {
            panic!("HBSP must retain the periodic S-parameter result family");
        };
        assert_eq!(frequencies.len(), 2);
        assert_eq!(reference_impedances_ohm, Some(vec![50.0, 50.0]));
        assert!(waveforms.contains_key("S11"));
        assert!(waveforms.contains_key("S21[k=+0,m=+0]"));

        let hbnoise = AnalysisSpec::Hbnoise {
            input_sideband: -1,
            output_sideband: -1,
            noise_reference: Some(svc_runner::HbNoiseReference {
                source_resistor: "RNOISE".into(),
                temperature_kelvin: 290.0,
            }),
            start_freq: 1.0e4,
            stop_freq: 1.0e5,
            points_per_unit: 2,
            sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            output_node: "p2".to_owned(),
            output_ref: "0".to_owned(),
            input_source: "VIN".to_owned(),
            max_sideband: 1,
            integrated_noise: true,
            noise_figure: true,
            contributor_ranking: true,
        };
        assert_eq!(
            crate::simulation::execution::canonical_analysis_kind(&hbnoise).execution_blocker(),
            None
        );
        let hbnoise_result = run_spec_request(
            &EngineBridge::new(),
            hbnoise.clone(),
            SpecExecutionOptions::default(),
            netlist,
            None,
            &dependencies,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("HBNOISE consumes the authenticated transferred HB state");
        let SimulationResult::Noise {
            frequencies,
            output_noise,
            summary,
            ..
        } = hbnoise_result
        else {
            panic!("HBNOISE must retain the noise result family");
        };
        assert!(frequencies.len() >= 2);
        assert!(
            summary
                .as_ref()
                .unwrap()
                .noise_figure
                .as_ref()
                .unwrap()
                .decibels
                .iter()
                .all(|value| value.is_finite())
        );
        let conversion = summary.as_ref().unwrap().conversion.as_ref().unwrap();
        assert_eq!(conversion.input_sideband, -1);
        assert_eq!(conversion.output_sideband, -1);
        assert_eq!(conversion.max_sideband, 1);
        assert!(conversion.carrier_hz > 0.0);
        assert_eq!(output_noise.len(), frequencies.len());
        assert!(
            output_noise
                .iter()
                .all(|value| value.is_finite() && *value >= 0.0)
        );
        assert!(summary.is_some(), "integrated HBNOISE retains its evidence");
        let mut spectrum_only = hbnoise;
        if let AnalysisSpec::Hbnoise {
            integrated_noise,
            noise_figure,
            contributor_ranking,
            ..
        } = &mut spectrum_only
        {
            *integrated_noise = false;
            *noise_figure = false;
            *contributor_ranking = false;
        }
        let spectrum_only = run_spec_request(
            &EngineBridge::new(),
            spectrum_only,
            SpecExecutionOptions::default(),
            netlist,
            None,
            &dependencies,
            &rspice_core::abort_signal::NoAbort,
        )
        .unwrap();
        let SimulationResult::Noise {
            summary: Some(summary),
            ..
        } = spectrum_only
        else {
            panic!(
                "Conversion channels must survive with optional band and figure evidence disabled"
            );
        };
        assert_eq!(summary.conversion.as_ref().unwrap(), conversion);
        assert!(summary.noise_figure.is_none());
        assert!(summary.total_rms.is_none());
        assert!(summary.input_rms.is_none());
        assert!(summary.rows.is_empty());
    }

    #[test]
    fn noise_input_units_survive_execution_worker_transport_and_retention() {
        use crate::simulation::runner::worker_contract::WorkerSimulationResult;
        use rspice_core::analysis::noise::NoiseInputQuantity;
        let netlist = "current-referred noise\nIref 0 out dc 0 ac 1\nR1 out 0 1k\n.end\n";
        let dependencies = transferred_hb_dependencies(netlist);
        let no_dependencies = ResolvedExecutionDependencies::default();
        let ordinary = AnalysisSpec::Noise {
            output_node: "out".into(),
            reference_node: "0".into(),
            input_source: "Iref".into(),
            start_freq: 1e3,
            stop_freq: 1e4,
            points_per_decade: 3,
            sweep: crate::simulation::config::NoiseSweepType::Linear,
            explicit_frequencies: None,
            data_table_name: None,
            contribution_detail: crate::simulation::config::NoiseContributionDetail::Top50,
            integration_mode: crate::simulation::config::NoiseIntegrationMode::Enabled,
            temperature: 300.15,
        };
        let hbnoise = AnalysisSpec::Hbnoise {
            input_sideband: 0,
            output_sideband: 0,
            noise_reference: None,
            start_freq: 1e3,
            stop_freq: 1e4,
            points_per_unit: 3,
            sweep: crate::simulation::multi_run::FrequencySweep::Linear,
            output_node: "out".into(),
            output_ref: "0".into(),
            input_source: "Iref".into(),
            max_sideband: 0,
            integrated_noise: true,
            noise_figure: false,
            contributor_ranking: true,
        };
        let pnoise_options = SpecExecutionOptions {
            pnoise: Some(svc_runner::PnoiseRunConfig {
                output_node: "out".into(),
                input_source: "Iref".into(),
                noise_ref: svc_runner::PnoiseReference::Input,
                start_freq: 1e3,
                stop_freq: 1e4,
                points_per_unit: 3,
                sweep: svc_runner::PnoiseFrequencySweep::Linear,
                pss_num_harmonics: 8,
                max_sideband: 0,
                integrated_noise: true,
                ..Default::default()
            }),
            ..Default::default()
        };
        for (spec, options, kind) in [
            (
                ordinary,
                SpecExecutionOptions::default(),
                crate::state::AnalysisType::Noise,
            ),
            (
                hbnoise,
                SpecExecutionOptions::default(),
                crate::state::AnalysisType::Hbnoise,
            ),
            (
                AnalysisSpec::Pnoise,
                pnoise_options,
                crate::state::AnalysisType::Pnoise,
            ),
        ] {
            let result = run_spec_request(
                &EngineBridge::new(),
                spec,
                options,
                netlist,
                None,
                if kind == crate::state::AnalysisType::Noise {
                    &no_dependencies
                } else {
                    &dependencies
                },
                &rspice_core::abort_signal::NoAbort,
            )
            .unwrap();
            let worker = WorkerSimulationResult::try_from(result).unwrap();
            let json = serde_json::to_string(&worker).unwrap();
            let result = SimulationResult::from(
                serde_json::from_str::<WorkerSimulationResult>(&json).unwrap(),
            );
            let retained = crate::simulation::controller::SimulationController::new()
                .convert_to_analysis_result_with_metadata_owned(
                    result,
                    kind,
                    "current-referred noise",
                );
            let input = retained
                .waveforms
                .iter()
                .find(|wave| wave.name == "inoise")
                .unwrap();
            assert_eq!(input.unit.as_deref(), Some("A²/Hz"));
            let expected = 4.0 * rspice_core::constants::K_BOLTZMANN * 300.15 / 1e3;
            for density in input.y.iter() {
                assert!(
                    (density / expected - 1.0).abs() < 1e-10,
                    "{kind:?}: {density:e}"
                );
            }
            let summary = retained.noise_summary.as_ref().unwrap();
            assert_eq!(summary.input_quantity, Some(NoiseInputQuantity::Current));
            assert_eq!(summary.input_rms_unit(), "A rms");
            assert_eq!(
                input.x.as_slice(),
                &[1e3, 5.5e3, 1e4],
                "{kind:?} frequency grid"
            );
            assert!(
                (summary.input_rms.unwrap().powi(2) / (expected * 9e3) - 1.0).abs() < 1e-10,
                "{kind:?}: RMS {:?}, expected {}, band {:?}",
                summary.input_rms,
                (expected * 9e3).sqrt(),
                summary.band
            );
            assert_eq!(retained.validate_retained_evidence(), Ok(()));
            let mut changed = retained.clone();
            changed.noise_summary.as_mut().unwrap().input_quantity =
                Some(NoiseInputQuantity::Voltage);
            assert_ne!(changed.result_data_digest(), retained.result_data_digest());
            assert!(changed.validate_retained_evidence().is_err());
        }
    }

    #[test]
    fn pnoise_reported_contributors_keep_their_physical_spectra_and_band_powers() {
        let netlist = "PNOISE contributor spectrum\nV1 in 0 0\nR1 in out 1k\nR2 out 0 1k\nC1 out 0 1n\n.end\n";
        let dependencies = transferred_hb_dependencies(netlist);
        let mut config = svc_runner::PnoiseRunConfig {
            pss_num_harmonics: 8,
            max_sideband: 1,
            start_freq: 1e3,
            stop_freq: 1e6,
            points_per_unit: 3,
            sweep: svc_runner::PnoiseFrequencySweep::Linear,
            output_node: "out".into(),
            integrated_noise: true,
            noise_summary: true,
            ..Default::default()
        };
        let execute = |config| {
            run_spec_request(
                &EngineBridge::new(),
                AnalysisSpec::Pnoise,
                SpecExecutionOptions {
                    pnoise: Some(config),
                    ..Default::default()
                },
                netlist,
                None,
                &dependencies,
                &rspice_core::abort_signal::NoAbort,
            )
        };
        let SimulationResult::Noise {
            contributors,
            frequencies,
            output_noise,
            summary: Some(summary),
            measurements,
            ..
        } = execute(config.clone()).unwrap()
        else {
            panic!("noise");
        };
        assert_eq!(contributors.len(), 2);
        assert_eq!(summary.rows.len(), 2);
        for index in 0..frequencies.len() {
            let sum: f64 = contributors.values().map(|values| values[index]).sum();
            assert!((sum / output_noise[index] - 1.0).abs() < 1e-12);
        }
        assert!(
            contributors
                .values()
                .all(|values| values[0] > 5.0 * values[2])
        );
        let power: f64 = summary.rows.iter().map(|row| row.power).sum();
        assert!((power / summary.total_rms.unwrap().powi(2) - 1.0).abs() < 1e-12);
        assert!(
            summary
                .rows
                .iter()
                .all(|row| (row.share_pct - 50.0).abs() < 1e-10)
        );
        assert_eq!(measurements.len(), 2);
        assert!(
            measurements
                .iter()
                .all(|measurement| (measurement.value.unwrap() - 50.0).abs() < 1e-10)
        );
        config.noise_summary = false;
        let SimulationResult::Noise {
            contributors,
            summary: Some(summary),
            measurements,
            ..
        } = execute(config.clone()).unwrap()
        else {
            panic!("noise");
        };
        assert!(contributors.is_empty() && summary.rows.is_empty() && measurements.is_empty());
        config.noise_summary = true;
        config.points_per_unit = 1;
        assert!(
            execute(config.clone())
                .unwrap_err()
                .to_string()
                .contains("at least two distinct")
        );
        config.integrated_noise = false;
        let SimulationResult::Noise {
            contributors,
            summary: Some(summary),
            measurements,
            ..
        } = execute(config).unwrap()
        else {
            panic!("noise");
        };
        assert_eq!(contributors.len(), 2);
        assert!(summary.rows.is_empty() && summary.total_rms.is_none());
        assert_eq!(measurements.len(), 2);
    }

    #[test]
    fn runnable_preview_psp_dispatches_from_authenticated_worker_state() {
        let netlist = "preview PSP dispatch\n\
                       P1 p1 0 SIN(0 0 1Meg) PORT=1 Z0=50\n\
                       R1 p1 p2 50\n\
                       C1 p1 0 1e-18\n\
                       P2 p2 0 SIN(0 0 1Meg) PORT=2 Z0=50\n\
                       .end\n";
        let dependencies = transferred_pss_dependencies(netlist);
        let psp = AnalysisSpec::Psp {
            start_freq: 1.0e4,
            stop_freq: 2.0e4,
            points_per_unit: 2,
            sweep: crate::simulation::multi_run::FrequencySweep::Linear,
            ports: periodic_ports(),
            max_sideband: 1,
            mixed_mode: false,
            noise_parameters: false,
            noise_reference: None,
        };
        assert_eq!(
            crate::simulation::execution::canonical_analysis_kind(&psp).execution_blocker(),
            None
        );

        let result = run_spec_request(
            &EngineBridge::new(),
            psp,
            SpecExecutionOptions::default(),
            netlist,
            None,
            &dependencies,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("PSP consumes the authenticated transferred PSS state");
        let SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } = result
        else {
            panic!("PSP must retain the periodic S-parameter result family");
        };
        assert_eq!(frequencies.len(), 2);
        assert!(waveforms.contains_key("S11"));
        assert!(waveforms.contains_key("S21[k=+0,m=+0]"));
    }

    /// A kind with no solver in this build is refused before the engine is
    /// asked, and refused by its own name.
    ///
    /// The fixture is taken from [`blocked_preview_specs`] rather than
    /// spelled here. It used to be DC mismatch, which now runs; taking the
    /// first still-blocked kind means this test keeps asking the question it
    /// was written to ask instead of having to be rewritten each time a kind
    /// gains a solver.
    #[test]
    fn unavailable_manifest_spec_is_rejected_before_engine_dispatch() {
        let spec = blocked_preview_specs()
            .into_iter()
            .next()
            .expect("this build still has a blocked preview kind");
        let display_name = spec.run_type().display_name().to_owned();
        let result = run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            "blocked preview\nV1 out 0 1\n.end\n",
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        );
        match result {
            Err(SimulationError::InvalidConfig(message)) => {
                assert!(message.contains(&display_name), "{message}");
                assert!(message.contains("unavailable"), "{message}");
                assert!(message.contains("rejected before dispatch"), "{message}");
            }
            other => panic!("expected fail-closed capability rejection, got {other:?}"),
        }
    }

    #[test]
    fn every_blocked_preview_spec_fails_closed_with_its_canonical_reason() {
        let mut seen = Vec::new();
        for spec in blocked_preview_specs() {
            let kind = crate::simulation::execution::canonical_analysis_kind(&spec);
            let expected = kind
                .execution_blocker()
                .unwrap_or_else(|| panic!("{kind:?} must declare why it is blocked"));
            assert!(
                spec.validate().is_ok(),
                "{kind:?} fixture must reach dispatch"
            );

            let result = run_spec_request(
                &EngineBridge::new(),
                spec,
                SpecExecutionOptions::default(),
                "blocked preview exact reason\nV1 out 0 1\n.end\n",
                None,
                &ResolvedExecutionDependencies::default(),
                &rspice_core::abort_signal::NoAbort,
            );
            match result {
                Err(SimulationError::InvalidConfig(message)) => {
                    assert!(message.contains(expected), "{kind:?}: {message}");
                    assert!(message.contains("rejected before dispatch"), "{message}");
                }
                other => panic!("{kind:?} must fail closed, got {other:?}"),
            }
            seen.push(kind);
        }

        assert_eq!(
            seen,
            vec![
                crate::state::CanonicalAnalysisKind::Qpss,
                crate::state::CanonicalAnalysisKind::Qpac,
                crate::state::CanonicalAnalysisKind::Qpnoise,
                crate::state::CanonicalAnalysisKind::Qpxf,
                crate::state::CanonicalAnalysisKind::Reliability,
            ]
        );
    }

    /// A resistor divider run with device noise on, twice from the same seed
    /// and once from another.
    ///
    /// The deck is the smallest circuit with a noise source in it: a resistor
    /// has thermal noise and nothing else, so what the run injects is the one
    /// mechanism under test rather than a device model's whole family.
    fn transient_noise_divider_run(seed: u64) -> Vec<f64> {
        const DECK: &str = "transient noise divider\n\
                            V1 in 0 DC 1\n\
                            R1 in out 10k\n\
                            R2 out 0 10k\n\
                            .end\n";
        let spec = AnalysisSpec::TransientNoise {
            stop_time: 1.0e-6,
            step_time: 1.0e-9,
            start_time: 0.0,
            max_timestep: 1.0e-9,
            seed,
            noise_fmax: 1.0e9,
            noise_fmin: None,
            scale: 1.0,
            uic: false,
        };
        // The deck the run executes is the card the Studio writes, spliced in
        // by the same builder the Analyses page displays. Writing a `.tran`
        // line here by hand would prove the engine can be asked for noise and
        // say nothing about whether the Studio asks for it.
        let card =
            crate::simulation::controller::SimulationController::build_transient_noise_command(
                &spec,
            )
            .expect("the specification writes its card");
        let deck = DECK.replace(".end\n", &format!("{card}\n.end\n"));

        let result = run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            &deck,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("a transient-noise specification reaches the engine");

        let SimulationResult::Transient {
            time, waveforms, ..
        } = result
        else {
            panic!("transient noise retains the transient result family");
        };
        assert!(!time.is_empty(), "the run produced no time axis");
        // A retained node voltage is keyed by the node's own name, as the
        // transient conversion writes it.
        waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("out"))
            .map(|(_, waveform)| waveform.y_values.clone())
            .unwrap_or_else(|| {
                panic!(
                    "the divider's output node is retained; the run returned {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                )
            })
    }

    /// The kind that was refused before dispatch now reaches its solver, and
    /// what comes back is a noisy waveform that the seed reproduces.
    ///
    /// Three properties, because any two of them pass for the wrong reason.
    /// A run that succeeds proves only that the refusal is gone. A waveform
    /// that differs from the deterministic divider proves noise was injected
    /// — a divider with no noise sits at exactly half the supply for the whole
    /// window, so a constant 0.5 V trace is the run silently ignoring the
    /// card. And the same seed reproducing it bit for bit is what makes the
    /// result a measurement rather than a sample: without it the Studio would
    /// be showing a number nobody can get back.
    #[test]
    fn a_transient_noise_run_reaches_the_engine_and_returns_waveforms() {
        let first = transient_noise_divider_run(97);
        let again = transient_noise_divider_run(97);
        let other = transient_noise_divider_run(98);

        assert!(
            first.iter().any(|value| (value - 0.5).abs() > f64::EPSILON),
            "every sample sat at the deterministic half-supply; no noise was injected"
        );
        assert_eq!(
            first.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            again.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            "the same seed must reproduce the same realization bit for bit"
        );
        assert_ne!(
            first.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            other.iter().map(|v| v.to_bits()).collect::<Vec<_>>(),
            "a different seed must play a different realization"
        );
    }

    /// The kind is no longer refused before its solver is asked.
    ///
    /// Replaces this kind's entry in the blocked-kinds walk above. Both halves
    /// are asserted: the blocker is gone from the canonical tag, and the
    /// runner routes the specification instead of returning the
    /// `rejected before dispatch` refusal that walk checks for.
    #[test]
    fn transient_noise_is_no_longer_refused_before_dispatch() {
        let spec = AnalysisSpec::TransientNoise {
            stop_time: 1.0e-6,
            step_time: 1.0e-9,
            start_time: 0.0,
            max_timestep: 1.0e-9,
            seed: 1,
            noise_fmax: 1.0e8,
            noise_fmin: None,
            scale: 1.0,
            uic: false,
        };
        assert_eq!(
            crate::simulation::execution::canonical_analysis_kind(&spec).execution_blocker(),
            None,
            "the transient-noise solver is in this build"
        );
        assert!(
            config::analysis_config_from_spec(&spec).is_some(),
            "transient noise routes with the config-backed group"
        );

        let result = run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            "transient noise dispatch\n\
             V1 in 0 DC 1\n\
             R1 in out 10k\n\
             R2 out 0 10k\n\
             .tran 1n 1u 0 1n NOISEFMAX=100000000 NOISESEED=1\n\
             .end\n",
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        );
        match result {
            Err(SimulationError::InvalidConfig(message))
                if message.contains("rejected before dispatch") =>
            {
                panic!("transient noise is still refused before dispatch: {message}")
            }
            Err(other) => panic!("transient noise must reach its solver, got {other:?}"),
            Ok(_) => {}
        }
    }

    /// An engine refusal is the engine's, word for word.
    ///
    /// The Studio does not pre-judge which devices can be rendered as noise
    /// sources: the mechanisms transient noise cannot inject — a tabulated
    /// density, a correlated BSIM4 channel/gate pair — are refused inside the
    /// engine, against the elaborated circuit, and the Studio has no second
    /// opinion to offer. What it must not do is translate the refusal, so this
    /// checks the engine's own sentence arrives at the run outcome unchanged.
    #[test]
    fn an_engine_noise_refusal_reaches_the_run_outcome_unchanged() {
        // A BSIM4 model card at `tnoiMod=2`: the mechanism is a cross-spectrum
        // between the channel and gate currents, which no pair of independent
        // injected currents reproduces. Nothing about the deck is malformed,
        // and nothing the Studio can see says so — only the elaborated circuit
        // does, which is exactly why the judgement is the engine's.
        let deck = "correlated thermal refusal\n\
                    vdd dd 0 dc 1.0\n\
                    rl dd d 10k\n\
                    vin g 0 dc 0.8\n\
                    m1 d g 0 0 n45 w=1u l=45n\n\
                    .model n45 nmos level=54 version=4.8 fnoimod=1 tnoimod=2\n\
                    .tran 1n 1u 0 1n NOISEFMAX=100000000 NOISESEED=1\n\
                    .end\n";
        let spec = AnalysisSpec::TransientNoise {
            stop_time: 1.0e-6,
            step_time: 1.0e-9,
            start_time: 0.0,
            max_timestep: 1.0e-9,
            seed: 1,
            noise_fmax: 1.0e8,
            noise_fmin: None,
            scale: 1.0,
            uic: false,
        };
        let error = run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            deck,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect_err("a correlated channel/gate pair has no independent-current rendering");
        let message = error.to_string();
        assert!(
            message.contains("correlated channel/gate thermal noise"),
            "the engine's own refusal must survive the Studio: {message}"
        );
        assert!(
            message.contains("tnoiMod=0 or 1"),
            "the engine's own remedy must survive with it: {message}"
        );
        assert!(
            !message.contains("rejected before dispatch"),
            "the refusal is the engine's, not a Studio pre-judgement: {message}"
        );
    }

    #[test]
    fn reliability_rejects_hard_coded_aging_without_a_pdk_model() {
        let result = run_spec_request(
            &EngineBridge::new(),
            AnalysisSpec::Reliability {
                target_years: vec![1.0, 10.0],
                enable_hci: true,
                enable_nbti: true,
                enable_em: false,
                min_stress_voltage: 0.1,
            },
            SpecExecutionOptions::default(),
            "reliability must fail closed\nV1 out 0 1\n.end\n",
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        );

        match result {
            Err(SimulationError::InvalidConfig(message)) => {
                assert!(message.contains("Reliability"));
                assert!(message.contains("unavailable"));
                assert!(message.contains("rejected before dispatch"));
            }
            other => panic!("expected PDK-less reliability refusal, got {other:?}"),
        }
    }

    #[test]
    fn pole_zero_spec_rejects_invalid_analysis_type_before_config_routing() {
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
            &ResolvedExecutionDependencies::default(),
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
    fn config_backed_routing_guard_reports_invalid_config() {
        let mut specs = config_backed_specs();
        specs.push(AnalysisSpec::Noise {
            output_node: "out".to_string(),
            reference_node: "0".to_string(),
            input_source: "V1".to_string(),
            start_freq: 1.0,
            stop_freq: 1.0e6,
            points_per_decade: 10,
            sweep: crate::simulation::config::NoiseSweepType::Decade,
            explicit_frequencies: None,
            data_table_name: None,
            contribution_detail: crate::simulation::config::NoiseContributionDetail::Top50,
            integration_mode: crate::simulation::config::NoiseIntegrationMode::Enabled,
            temperature: 300.0,
        });

        for spec in specs {
            match config_backed_spec_routing_error(&spec) {
                SimulationError::InvalidConfig(message) => {
                    assert!(
                        message.contains(config_backed_spec_name(&spec)),
                        "routing error should name the spec variant: {message}"
                    );
                    assert!(
                        message.contains("AnalysisConfig"),
                        "routing error should tell callers which path to use: {message}"
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
                    AnalysisSpec::dc_op(),
                    SpecExecutionOptions::default(),
                    "",
                    None,
                    None,
                    &rspice_core::abort_signal::NoAbort,
                ),
                "AnalysisSpec::DcOp",
            ),
            (
                "device",
                device::run_device_spec(
                    AnalysisSpec::dc_op(),
                    "",
                    None,
                    &rspice_core::abort_signal::NoAbort,
                ),
                "AnalysisSpec::DcOp",
            ),
            (
                "periodic",
                periodic::run_periodic_spec(
                    AnalysisSpec::dc_op(),
                    "",
                    None,
                    &ResolvedExecutionDependencies::default(),
                    &rspice_core::abort_signal::NoAbort,
                ),
                "AnalysisSpec::DcOp",
            ),
            (
                "frequency",
                frequency::run_frequency_spec(
                    AnalysisSpec::dc_op(),
                    SpecExecutionOptions::default(),
                    "",
                    None,
                    &ResolvedExecutionDependencies::default(),
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
            (
                "sweep",
                AnalysisSpec::MonteCarlo {
                    variation_source: Default::default(),
                    params: Vec::new(),
                },
            ),
            (
                "device",
                AnalysisSpec::Soa {
                    rules: Vec::new(),
                    observation: Default::default(),
                    stop_time: 1.0e-6,
                    step_time: 1.0e-9,
                    check_vgs_max: true,
                    max_vgs: 1.2,
                    check_vds_max: false,
                    max_vds: 1.2,
                    check_vbe_max: false,
                    max_vbe: 0.8,
                    check_vce_max: false,
                    max_vce: 1.2,
                },
            ),
            (
                "periodic",
                AnalysisSpec::HarmonicBalance {
                    tones: vec![crate::simulation::multi_run::HbToneSpec::new(1.0e6, 3)],
                    reltol: 1.0e-6,
                    abstol: 1.0e-12,
                    max_iterations: 40,
                    damping: 0.7,
                    min_damping: 0.01,
                    oversample: 4,
                    collocation_points: Some(7),
                    max_mixing_order: 3,
                    use_krylov: false,
                    gmres_restart: 12,
                    source_stepping: false,
                    use_exact_jacobian: true,
                    verbose: false,
                },
            ),
            (
                "frequency",
                AnalysisSpec::Tf {
                    input_source: "V1".to_owned(),
                    output_expression: "V(out)".to_owned(),
                    transfer_gain: true,
                    input_resistance: true,
                    output_resistance: true,
                    normalization: crate::simulation::multi_run::TfNormalization::None,
                    accuracy: crate::simulation::multi_run::TfAccuracy::Balanced,
                },
            ),
        ];

        for (family, spec) in cases {
            let signal = AbortOnPoll::new(3);
            let result = run_spec_request(
                &EngineBridge::new(),
                spec,
                SpecExecutionOptions::default(),
                "cancellation boundary\n.end\n",
                None,
                &ResolvedExecutionDependencies::default(),
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
    fn typed_service_resource_limit_maps_to_runner_resource_limit() {
        let result: Result<(), SimulationError> =
            run_abort_aware_service(&rspice_core::abort_signal::NoAbort, || {
                Err(svc_runner::ServiceRunError::resource_limit(
                    rspice_core::ResourceKind::BatchRuns,
                    3,
                    2,
                ))
            });

        assert_eq!(
            result,
            Err(SimulationError::ResourceLimit {
                resource: "batch_runs".to_string(),
                requested: 3,
                limit: 2,
            })
        );
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

    /// A stated frequency axis reaches the AC solve, and the answer is the one
    /// the circuit has at exactly those frequencies.
    ///
    /// The deck is the smallest circuit with an exact closed-form response: a
    /// series RC low-pass, whose transfer function is `1/(1 + j·2πfRC)`. Three
    /// frequencies are authored an octave apart and deliberately *not* on any
    /// decade or octave grid a graded `.ac` would land on, so a run that
    /// quietly substituted a graded sweep could not pass.
    ///
    /// The deck the run executes is the pair of cards the Studio writes —
    /// `.ac DATA=` and its `.DATA` table — spliced in by the same builder the
    /// Analyses page displays, because the axis reaches the engine only
    /// through them.
    #[test]
    fn sp_single_point_and_zero_linear_grids_survive_configuration_and_solve() {
        use crate::simulation::dialog::sp::SpSweepType;
        use crate::simulation::dialog::{SpConfig, SpDialogState};
        use crate::simulation::multi_run::FrequencySweep;
        for (kind, sweep, start, stop, count, expected, noise) in [
            (
                SpSweepType::Linear,
                FrequencySweep::Linear,
                0.0,
                0.0,
                1,
                vec![0.0],
                false,
            ),
            (
                SpSweepType::Linear,
                FrequencySweep::Linear,
                0.0,
                1000.0,
                3,
                vec![0.0, 500.0, 1000.0],
                false,
            ),
            (
                SpSweepType::Decade,
                FrequencySweep::Decade,
                1000.0,
                1000.0,
                10,
                vec![1000.0],
                false,
            ),
            (
                SpSweepType::Octave,
                FrequencySweep::Octave,
                1000.0,
                1000.0,
                10,
                vec![1000.0],
                true,
            ),
        ] {
            let config = SpConfig {
                sweep_type: kind,
                start_freq: start,
                stop_freq: stop,
                num_points: count,
                do_noise: noise,
                ports: Vec::new(),
                ..Default::default()
            };
            config.validate().unwrap();
            SpDialogState::from_config(&config).to_config(None).unwrap();
            let result = run_spec_request(
                &EngineBridge::new(),
                AnalysisSpec::SParameter {
                    start_freq: start,
                    stop_freq: stop,
                    points_per_unit: count as usize,
                    sweep,
                    z0: 50.0,
                    ports: Vec::new(),
                    do_noise: noise,
                },
                SpecExecutionOptions::default(),
                "SP grid\nP1 in 0 PORT=1 Z0=50\nP2 out 0 PORT=2 Z0=50\nR1 in out 50\n.end\n",
                None,
                &ResolvedExecutionDependencies::default(),
                &rspice_core::NoAbort,
            )
            .unwrap();
            let SimulationResult::Ac {
                frequencies,
                waveforms,
                ..
            } = result
            else {
                panic!("expected SP")
            };
            assert_eq!(frequencies, expected);
            let reflection = &waveforms["S11"];
            assert!(
                reflection
                    .y_values
                    .iter()
                    .all(|value| (value - 1.0 / 3.0).abs() < 1e-10)
            );
        }
        let config = SpConfig {
            sweep_type: SpSweepType::Linear,
            start_freq: 0.0,
            do_noise: true,
            ..Default::default()
        };
        assert!(
            config.validate().is_err(),
            "noise never evaluates at zero Hz"
        );
    }

    #[test]
    fn an_explicit_frequency_table_reaches_the_ac_solve() {
        const RESISTANCE: f64 = 1.0e3;
        const CAPACITANCE: f64 = 1.0e-6;
        const DECK: &str = "ac frequency table low pass\n\
                            V1 in 0 AC 1\n\
                            R1 in out 1k\n\
                            C1 out 0 1u\n\
                            .end\n";
        let authored = vec![148.0, 0.0, 37.0, 37.0, 74.0];

        let spec = AnalysisSpec::AcData {
            table_name: crate::simulation::config::AC_FREQUENCY_TABLE.to_owned(),
            frequencies: authored.clone(),
        };
        let cards =
            crate::simulation::controller::SimulationController::build_ac_data_command(&spec)
                .expect("the specification writes its card and its table");
        let deck = DECK.replace(".end\n", &format!("{cards}\n.end\n"));

        let result = run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            &deck,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("an AC frequency-table specification reaches the engine");

        let SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } = result
        else {
            panic!("an AC frequency table retains the AC result family");
        };
        assert_eq!(
            frequencies, authored,
            "the solved axis must be exactly the authored one"
        );

        let response = waveforms
            .iter()
            .find(|(name, _)| name.to_ascii_lowercase().contains("out"))
            .map(|(_, waveform)| waveform.clone())
            .unwrap_or_else(|| {
                panic!(
                    "the low-pass output is retained; the run returned {:?}",
                    waveforms.keys().collect::<Vec<_>>()
                )
            });
        let imaginary = response
            .y_imag
            .as_ref()
            .expect("an AC response retains both components");
        assert_eq!(response.y_values.len(), authored.len());

        for (index, frequency) in authored.iter().copied().enumerate() {
            let magnitude = response.y_values[index].hypot(imaginary[index]);
            let omega_rc = std::f64::consts::TAU * frequency * RESISTANCE * CAPACITANCE;
            let exact = 1.0 / (1.0 + omega_rc * omega_rc).sqrt();
            assert!(
                (magnitude - exact).abs() <= 1.0e-9 * exact,
                "at {frequency} Hz the solve returned {magnitude} and the circuit has {exact}"
            );
        }
    }

    #[test]
    fn ac_table_row_parameters_reach_the_studio_result_and_axis_mismatch_is_refused() {
        let deck = "AC table\nV1 in 0 AC 1\n.param load=1k\nR1 in out 1k\nR2 out 0 {load}\n.data pts FREQ load\n1000 1000\n0 2000\n1000 500\n.enddata\n.end\n";
        let run = |frequencies| {
            run_spec_request(
                &EngineBridge::new(),
                AnalysisSpec::AcData {
                    table_name: "pts".into(),
                    frequencies,
                },
                SpecExecutionOptions::default(),
                deck,
                None,
                &ResolvedExecutionDependencies::default(),
                &rspice_core::NoAbort,
            )
        };
        let SimulationResult::Ac {
            frequencies,
            waveforms,
            ..
        } = run(vec![1000.0, 0.0, 1000.0]).unwrap()
        else {
            panic!("expected AC")
        };
        assert_eq!(frequencies, vec![1000.0, 0.0, 1000.0]);
        for (value, expected) in
            waveforms["V(OUT)"]
                .y_values
                .iter()
                .zip([0.5, 2.0 / 3.0, 1.0 / 3.0])
        {
            assert!((value - expected).abs() < 1e-10);
        }
        let error = run(vec![1000.0, 10.0, 1000.0]).unwrap_err();
        assert!(error.to_string().contains("does not match"), "{error}");
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
            &ResolvedExecutionDependencies::default(),
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
            AnalysisSpec::dc_op(),
            AnalysisSpec::DcSweep {
                source_name: "V1".to_string(),
                start: 0.0,
                stop: 1.0,
                step: 0.1,
                source2: None,
                start2: None,
                stop2: None,
                step2: None,
                hysteresis: false,
                modes: Default::default(),
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
                filter: String::new(),
                sweep: None,
            },
        ]
    }

    // ------------------------------------------------------------ DC mismatch
    //
    // `.DCMATCH` has no default spread: a deck with no `statistics` block is
    // refused by name. So the fixture brings one, lowered through the same
    // public Spectre adapter the include expander runs a `.scs` library
    // through — which means these tests need no file system and run on a
    // wasm-shaped target.

    /// Two resistances with independent per-instance mismatch, declared the
    /// way a PDK declares it.
    const DIVIDER_STATISTICS: &str = "\
// Resistor divider mismatch.
parameters r1v=1000 r2v=2000
statistics {
 mismatch {
  vary r1v dist=gauss std=10
  vary r2v dist=gauss std=10
 }
}
";

    const MISMATCH_DIVIDER: &str = "dc mismatch divider\n\
V1 in 0 1\n\
R1 in out {r1v}\n\
R2 out 0 {r2v}\n";

    /// The divider deck with the statistics library lowered into it.
    fn statistical_divider(cards: &str) -> String {
        let lowered = rspice_core::library::adapt_spectre_model_library(
            std::path::Path::new("statistics.scs"),
            DIVIDER_STATISTICS,
        )
        .expect("the statistics library lowers to executable SPICE");
        let (title, body) = MISMATCH_DIVIDER
            .split_once('\n')
            .expect("a deck carries a title line and a body");
        format!("{title}\n{lowered}{body}{cards}.end\n")
    }

    fn dc_mismatch_spec(sigma_multiplier: f64, contributor_limit: usize) -> AnalysisSpec {
        AnalysisSpec::DcMismatch {
            output_expression: "V(out)".to_owned(),
            sigma_multiplier,
            contributor_limit,
            include_process: false,
            include_mismatch: true,
            normalized_contributions: true,
            contribution_threshold: None,
        }
    }

    fn run_dc_mismatch(
        deck: &str,
        spec: AnalysisSpec,
    ) -> Result<SimulationResult, SimulationError> {
        run_spec_request(
            &EngineBridge::new(),
            spec,
            SpecExecutionOptions::default(),
            deck,
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::NoAbort,
        )
    }

    /// The whole route, ending on numbers a hand calculation can check.
    ///
    /// A 1 V source over 1k and 2k: the output sits at 2/3, and one standard
    /// deviation of each resistance displaces it by `V * r_other / (r1+r2)^2`
    /// times that deviation. The two displacements are independent, so the
    /// spread is their root sum of squares and the shares are 4/5 and 1/5.
    /// Nothing here is a golden number — every value is the divider's own.
    #[test]
    fn a_dc_mismatch_run_reaches_the_engine_and_returns_the_analytic_divider_spread() {
        let result = run_dc_mismatch(&statistical_divider(""), dc_mismatch_spec(1.0, 0))
            .expect("a design with statistics runs");
        let SimulationResult::DcMismatch { evidence } = result else {
            panic!("DC mismatch must retain its own evidence family");
        };
        assert_eq!(evidence.validate(), Ok(()));
        assert_eq!(evidence.output_unit, "V");
        assert!(
            (evidence.nominal_value - 2.0 / 3.0).abs() < 1.0e-12,
            "{}",
            evidence.nominal_value
        );

        let expected =
            ((2000.0 * 10.0 / 9.0e6_f64).powi(2) + (1000.0 * 10.0 / 9.0e6_f64).powi(2)).sqrt();
        assert!(
            ((evidence.sigma_total - expected) / expected).abs() < 1.0e-3,
            "{} against {expected}",
            evidence.sigma_total
        );
        // Mismatch only, so the whole spread is the mismatch half.
        assert_eq!(evidence.sigma_process, 0.0);
        assert!((evidence.sigma_mismatch - evidence.sigma_total).abs() < 1.0e-18);
        assert_eq!(evidence.applied_correlations_mismatch, 0);
        assert_eq!(evidence.applied_correlations_process, 0);

        // Every (instance, variable) pair is evaluated; two of them carry the
        // variance, and a limit of zero keeps the rest rather than hiding
        // that the analysis looked at them.
        assert_eq!(evidence.evaluated_contributors, 6);
        assert_eq!(evidence.retained_contributors(), 6);
        let owners: Vec<(&str, &str, f64)> = evidence
            .contributors
            .iter()
            .filter(|row| row.share != 0.0)
            .map(|row| (row.instance.as_str(), row.parameter.as_str(), row.share))
            .collect();
        assert_eq!(owners.len(), 2, "{:?}", evidence.contributors);
        assert_eq!(owners[0].0, "R1");
        assert_eq!(owners[0].1, "R1V");
        assert!((owners[0].2 - 0.8).abs() < 1.0e-6, "{:?}", owners[0]);
        assert_eq!(owners[1].0, "R2");
        assert_eq!(owners[1].1, "R2V");
        assert!((owners[1].2 - 0.2).abs() < 1.0e-6, "{:?}", owners[1]);
    }

    /// The run executes the card the Studio wrote, not another card that
    /// happens to be in the deck.
    ///
    /// A generated deck carries every queued analysis's directive and a
    /// hand-written deck may carry several `.DCMATCH` cards; nothing in the
    /// deck says which of them is this task's. The line the task was
    /// dispatched with does.
    #[test]
    fn a_dc_mismatch_run_executes_the_card_the_studio_wrote_not_another_card_in_the_deck() {
        let deck = statistical_divider(".DCMATCH OUT=V(out) SIGMA=6\n");
        let result = run_dc_mismatch(&deck, dc_mismatch_spec(3.0, 2))
            .expect("the deck's own card does not stop the task's card running");
        let SimulationResult::DcMismatch { evidence } = result else {
            panic!("DC mismatch must retain its own evidence family");
        };
        assert_eq!(evidence.sigma_multiplier, 3.0, "the spec's multiplier ran");
        // And the spec's limit trimmed the list, not the deck's default.
        assert_eq!(evidence.contributor_limit, 2);
        assert_eq!(evidence.retained_contributors(), 2);
        assert!(evidence.is_trimmed());
    }

    /// The kind is no longer refused before the engine is asked.
    #[test]
    fn dc_mismatch_is_no_longer_refused_before_dispatch() {
        assert_eq!(
            crate::state::CanonicalAnalysisKind::DcMismatch.execution_blocker(),
            None
        );
        assert!(
            !blocked_preview_specs()
                .iter()
                .any(|spec| matches!(spec, AnalysisSpec::DcMismatch { .. })),
            "a runnable kind cannot also be on the blocked list"
        );
        let error = run_dc_mismatch(
            "no statistics\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n.end\n",
            dc_mismatch_spec(1.0, 10),
        )
        .expect_err("a design with no statistics is still refused");
        assert!(
            !format!("{error}").contains("rejected before dispatch"),
            "{error}"
        );
    }

    /// A design with no statistics is refused in the engine's own words,
    /// with the Studio's remedy after them.
    #[test]
    fn a_design_without_statistics_is_refused_in_the_engines_words_with_the_studio_remedy() {
        let error = run_dc_mismatch(
            "no statistics\nV1 in 0 1\nR1 in out 1k\nR2 out 0 2k\n.end\n",
            dc_mismatch_spec(1.0, 10),
        )
        .expect_err("a design with no statistics is refused");
        let message = format!("{error}");
        assert!(
            message.contains(
                ".DCMATCH needs a `statistics { mismatch { vary ... } }` block; none is bound to \
                 this design"
            ),
            "{message}"
        );
        assert!(
            message.contains(
                "Attach a Spectre model library whose bound section declares one, or include a \
                 Spectre file that does."
            ),
            "{message}"
        );
        // A design that does declare statistics and fails for another reason
        // must not be told to attach a library it already has.
        let error = run_dc_mismatch(
            &statistical_divider(""),
            AnalysisSpec::DcMismatch {
                output_expression: "V(nowhere)".to_owned(),
                sigma_multiplier: 1.0,
                contributor_limit: 10,
                include_process: false,
                include_mismatch: true,
                normalized_contributions: true,
                contribution_threshold: None,
            },
        )
        .expect_err("a probe that names nothing is refused");
        assert!(
            !format!("{error}").contains("Attach a Spectre model library"),
            "{error}"
        );
    }

    /// A cancelled DC mismatch run stays cancelled rather than degrading to a
    /// configuration failure.
    #[test]
    fn a_cancelled_dc_mismatch_run_stays_cancelled() {
        let error = run_spec_request(
            &EngineBridge::new(),
            dc_mismatch_spec(1.0, 10),
            SpecExecutionOptions::default(),
            &statistical_divider(""),
            None,
            &ResolvedExecutionDependencies::default(),
            &rspice_core::abort_signal::ImmediateAbort,
        )
        .expect_err("an aborted run does not produce evidence");
        assert!(matches!(error, SimulationError::Aborted), "{error:?}");
    }
}

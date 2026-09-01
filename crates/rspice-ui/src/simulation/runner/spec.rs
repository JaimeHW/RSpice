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
        .validate_for_spec(&spec)
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
        AnalysisSpec::AcData { frequencies, .. } => bridge.run_ac_frequencies_with_source_path(
            netlist,
            source_path,
            frequencies,
            abort_flag,
        ),
        AnalysisSpec::Optimization { .. } | AnalysisSpec::Soa { .. } => {
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
        | AnalysisSpec::TransientNoise { .. }
        | AnalysisSpec::DcMismatch { .. }
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

    fn hb_producer_spec() -> AnalysisSpec {
        AnalysisSpec::HarmonicBalance {
            tones: vec![HbToneSpec::new(1.0e6, 8)],
            reltol: 2.5e-7,
            abstol: 1.0e-12,
            max_iterations: 100,
            damping: 1.0,
            oversample: 2,
            collocation_points: None,
            max_mixing_order: 5,
            use_krylov: false,
            gmres_restart: 30,
            source_stepping: false,
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
            AnalysisSpec::TransientNoise {
                stop_time: 1.0e-6,
                step_time: 1.0e-9,
                start_time: 0.0,
                max_timestep: 1.0e-9,
                seed: 1,
                noise_fmax: 1.0e8,
                scale: 1.0,
                uic: false,
            },
            AnalysisSpec::DcMismatch {
                output_expression: "V(out)".to_owned(),
                sigma_multiplier: 3.0,
                contributor_limit: 25,
                include_process: false,
                include_mismatch: true,
                normalized_contributions: true,
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
                       P1 p1 0 PORT=1 Z0=50\n\
                       R1 p1 p2 50\n\
                       C1 p1 0 1e-18\n\
                       P2 p2 0 PORT=2 Z0=50\n\
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
            ..
        } = hbsp_result
        else {
            panic!("HBSP must retain the periodic S-parameter result family");
        };
        assert_eq!(frequencies.len(), 2);
        assert!(waveforms.contains_key("S11"));
        assert!(waveforms.contains_key("S21[k=+0,m=+0]"));

        let hbnoise = AnalysisSpec::Hbnoise {
            start_freq: 1.0e4,
            stop_freq: 1.0e5,
            points_per_unit: 2,
            sweep: crate::simulation::multi_run::FrequencySweep::Decade,
            output_node: "p2".to_owned(),
            output_ref: "0".to_owned(),
            input_source: "VIN".to_owned(),
            max_sideband: 1,
            integrated_noise: true,
            noise_figure: false,
            contributor_ranking: true,
        };
        assert_eq!(
            crate::simulation::execution::canonical_analysis_kind(&hbnoise).execution_blocker(),
            None
        );
        let hbnoise_result = run_spec_request(
            &EngineBridge::new(),
            hbnoise,
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
        assert_eq!(output_noise.len(), frequencies.len());
        assert!(
            output_noise
                .iter()
                .all(|value| value.is_finite() && *value >= 0.0)
        );
        assert!(summary.is_some(), "integrated HBNOISE retains its evidence");
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
            &ResolvedExecutionDependencies::default(),
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
                crate::state::CanonicalAnalysisKind::TransientNoise,
                crate::state::CanonicalAnalysisKind::DcMismatch,
                crate::state::CanonicalAnalysisKind::Reliability,
            ]
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
                },
            ),
            (
                "device",
                AnalysisSpec::Soa {
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
                    oversample: 4,
                    collocation_points: Some(7),
                    max_mixing_order: 3,
                    use_krylov: false,
                    gmres_restart: 12,
                    source_stepping: false,
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

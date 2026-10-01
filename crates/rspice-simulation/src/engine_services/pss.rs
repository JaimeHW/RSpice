//! Periodic steady-state analysis.
//!
//! Finds the periodic operating point a driven circuit settles into, and the
//! autonomous oscillation frequency for an oscillator. Every periodic
//! small-signal analysis linearizes about the result.

#![allow(clippy::type_complexity)]

use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use crate::error::{ensure_not_aborted, poll_periodically};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::{Engine, PssDcOperatingPointSeed};
use std::path::Path;
use std::sync::Arc;

mod output;

/// PSS analysis data
#[derive(Debug, Clone)]
pub struct PssData {
    /// Time points within one period
    pub time: Vec<Value>,
    /// Periodic waveforms: (node_name, values)
    pub waveforms: Vec<(String, Vec<Value>)>,
    /// Exact converged shooting state for dependent periodic analyses.
    pub operating_point: Arc<rspice_core::engine::PssOperatingPoint>,
}

pub use crate::periodic::PssRunConfig;
use crate::periodic::{build_core_pss_config, validate_pss_config};

/// Run PSS analysis with cooperative cancellation and no source path.
///
/// Test-only. PSS ships through
/// [`run_pss_analysis_with_dc_seed_and_source_path_and_abort`], which the
/// periodic spec calls with the operating point its dependency produced.
#[cfg(test)]
pub fn run_pss_analysis_with_abort(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    run_pss_analysis_with_source_path_and_abort(
        netlist_text,
        fundamental_freq,
        num_harmonics,
        tolerance,
        None,
        abort,
    )
}

/// Run PSS analysis with source-path resolution and cooperative cancellation.
///
/// Finds the periodic steady-state solution of a circuit with autonomous or
/// driven oscillations, using the shooting method with Newton iteration.
#[allow(
    dead_code,
    reason = "retained PSS source-path adapter for callers and tests"
)]
pub fn run_pss_analysis_with_source_path_and_abort(
    netlist_text: &str,
    fundamental_freq: Value,
    num_harmonics: usize,
    tolerance: Value,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    ensure_not_aborted(abort)?;
    // The compatibility API predates authored Tones. Its historical semantic
    // is the complete set of circuit drives, so resolve that set explicitly
    // and feed it through the same strict periodic validator as the typed path.
    let parsed = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = Engine::new(build_engine_config(&parsed, None));
    let tone_sources = engine
        .transient_source_names_with_abort(&parsed, abort)
        .map_err(|error| ServiceRunError::from_core("PSS source discovery failed", error))?;
    let config = PssRunConfig::new(fundamental_freq, tone_sources, num_harmonics, tolerance);
    run_pss_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        &config,
        source_path,
        abort,
    )
}

/// Run a fully materialized shooting-PSS request with source-path resolution
/// and cooperative cancellation.
#[allow(dead_code, reason = "retained configured PSS source-path adapter")]
pub fn run_pss_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &PssRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    run_pss_analysis_internal(netlist_text, config, source_path, None, abort)
}

/// Run shooting PSS from the exact operating-point state authenticated by
/// the prepared dependency graph. The source is the producer's exact
/// process-bound source; its voltage corner is applied once before both basis
/// validation and shooting, and its temperature is propagated to the core
/// engine in Kelvin.
pub(crate) fn run_pss_analysis_with_dc_seed_and_source_path_and_abort(
    netlist_text: &str,
    config: &PssRunConfig,
    source_path: Option<&Path>,
    seed_environment: PssSeedEnvironment<'_>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    run_pss_analysis_internal(
        netlist_text,
        config,
        source_path,
        Some(seed_environment),
        abort,
    )
}

pub(crate) struct PssSeedEnvironment<'a> {
    pub(crate) dc_seed: &'a PssDcOperatingPointSeed,
    pub(crate) temperature_celsius: Value,
    pub(crate) supply_voltage: Option<Value>,
    pub(crate) nominal_supply_voltage: Option<Value>,
    pub(crate) supply_source_names: &'a [String],
}

fn run_pss_analysis_internal(
    netlist_text: &str,
    config: &PssRunConfig,
    source_path: Option<&Path>,
    seed_environment: Option<PssSeedEnvironment<'_>>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    ensure_not_aborted(abort)?;
    let validation = validate_pss_config(config);
    ensure_not_aborted(abort)?;
    validation.map_err(ServiceRunError::Failure)?;

    let temperature_source = seed_environment
        .as_ref()
        .map(|environment| {
            crate::netlist_preparation::source_with_run_temperature_with_abort(
                netlist_text,
                environment.temperature_celsius,
                abort,
            )
        })
        .transpose()?;
    let mut netlist = parse_runner_netlist_with_abort(
        temperature_source.as_deref().unwrap_or(netlist_text),
        source_path,
        abort,
    )?;
    netlist.source_text = Some(netlist_text.to_owned());

    let seeded_temperature_kelvin = seed_environment
        .as_ref()
        .map(|environment| {
            apply_seed_environment(
                &mut netlist,
                environment.temperature_celsius,
                environment.supply_voltage,
                environment.nominal_supply_voltage,
                environment.supply_source_names,
                abort,
            )
        })
        .transpose()?;

    run_pss_analysis_on_materialized_with_abort(
        &netlist,
        config,
        seed_environment
            .as_ref()
            .map(|environment| environment.dc_seed),
        seeded_temperature_kelvin,
        abort,
    )
}

/// Execute on one already varied circuit and its freshly solved DC seed.
/// Temperature/supply materialization belongs to the caller at this boundary.
pub(crate) fn run_pss_analysis_on_materialized_with_abort(
    netlist: &rspice_core::Netlist,
    config: &PssRunConfig,
    dc_seed: Option<&PssDcOperatingPointSeed>,
    temperature_kelvin: Option<Value>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    ensure_not_aborted(abort)?;
    validate_pss_config(config).map_err(ServiceRunError::Failure)?;
    let mut sim_config = build_engine_config(netlist, None);
    sim_config.tolerance = config.tolerance;
    if let Some(temperature_kelvin) = temperature_kelvin {
        sim_config.temperature = temperature_kelvin;
    }
    let engine = Engine::try_new_with_resolved_config(sim_config).map_err(|error| {
        ServiceRunError::from_core(
            "PSS resolved engine configuration is invalid",
            rspice_core::SimulationError::Configuration(error),
        )
    })?;
    // The closed periodic-source contract is a *driven* rule, and the draft
    // validator already asks it only of a driven solve
    // (`simulation::plan::config::PssConfigContext::validate_pss_sources`).
    // Asking it here as well made the planner and the runner disagree about
    // the same run: an autonomous draft that the Studio admits, queues and
    // authenticates was then refused the moment it executed. It is
    // unsatisfiable for an oscillator by construction -- the contract demands
    // every elaborated source be named AND be an undelayed drive commensurate
    // with the fundamental, while an oscillator's only source is a one-shot
    // startup kick and its fundamental is the solver's unknown, so neither an
    // empty selection nor a complete one can pass. Core imposes no such rule
    // on the tone list of `PssConfig::autonomous()`. Core separately verifies
    // that the free-running orbit has no changing external drive.
    let pss_config = build_core_pss_config(config);
    if !config.oscillator_mode {
        engine
            .validate_pss_source_contract_with_abort(
                netlist,
                &config.tone_sources,
                &pss_config,
                abort,
            )
            .map_err(|error| {
                ServiceRunError::from_core("PSS tone-source validation failed", error)
            })?;
    }

    let operating_point = match dc_seed {
        Some(seed) => engine
            .run_pss_operating_point_with_dc_seed_and_abort(netlist, pss_config, seed, abort)
            .map_err(|error| ServiceRunError::from_core("PSS error", error))?,
        None => engine
            .run_pss_operating_point_with_abort(netlist, pss_config, abort)
            .map_err(|error| ServiceRunError::from_core("PSS error", error))?,
    };
    let pss_result = operating_point.analysis();

    let period = pss_result.period;
    if !period.is_finite() || period <= 0.0 {
        return Err(ServiceRunError::Failure(
            "PSS solver returned an invalid period".to_string(),
        ));
    }
    let projection = output::projection(
        &pss_result.result.time,
        &netlist.options,
        engine.config().resource_limits,
        pss_result.result.node_names.len() + pss_result.result.branch_names.len(),
        abort,
    )?;
    let mut time = Vec::with_capacity(projection.times().len());
    for (sample_idx, sample) in projection.times().iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        time.push(*sample);
    }

    let mut waveforms: Vec<(String, Vec<Value>)> = Vec::new();
    let result = &pss_result.result;
    for (name, waveform, prefix) in result
        .node_names
        .iter()
        .zip(&result.waveforms)
        .filter(|(name, _)| name.as_str() != "0" && !name.eq_ignore_ascii_case("gnd"))
        .map(|(name, waveform)| (name, waveform, "V"))
        .chain(
            result
                .branch_names
                .iter()
                .zip(&result.branch_waveforms)
                .map(|(name, waveform)| (name, waveform, "I")),
        )
    {
        ensure_not_aborted(abort)?;
        let values = projection
            .project(&waveform.values)
            .map_err(ServiceRunError::Failure)?;
        waveforms.push((format!("{prefix}({name})"), values));
    }
    if waveforms.is_empty() {
        return Err(ServiceRunError::Failure(
            "PSS solver returned no non-ground node waveforms".to_owned(),
        ));
    }

    ensure_not_aborted(abort)?;
    Ok(PssData {
        time,
        waveforms,
        operating_point: Arc::new(operating_point),
    })
}

fn apply_seed_environment(
    netlist: &mut rspice_core::Netlist,
    temperature_celsius: Value,
    supply_voltage: Option<Value>,
    nominal_supply_voltage: Option<Value>,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    if !temperature_celsius.is_finite() || temperature_celsius <= -273.15 {
        return Err(ServiceRunError::Failure(
            "PSS DC seed temperature must be finite and above absolute zero".to_owned(),
        ));
    }
    match (supply_voltage, nominal_supply_voltage) {
        (None, None) => {}
        (Some(supply), Some(nominal)) => {
            crate::netlist_preparation::apply_voltage_corner(
                netlist,
                supply,
                nominal,
                supply_source_names,
                abort,
            )?;
        }
        _ => {
            return Err(ServiceRunError::Failure(
                "PSS DC seed supply and nominal voltage must be present together".to_owned(),
            ));
        }
    }
    netlist.options.temp = Some(temperature_celsius);
    Ok(temperature_celsius + 273.15)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn pss_harmonics_observes_abort_inside_nested_sample_loop() {
        let abort = AbortOnPoll {
            abort_on: 9,
            polls: AtomicUsize::new(0),
        };
        let mut waveform = rspice_core::analysis::pss::PeriodicWaveform::new(128);
        waveform.values = vec![1.0; 128];
        let time = (0..128)
            .map(|index| index as f64 * 1e-6 / 127.0)
            .collect::<Vec<_>>();
        let result = waveform.compute_harmonics_with_abort(&time, 1.0e6, 20, &abort);
        assert!(matches!(
            result,
            Err(rspice_core::analysis::fourier::FourierError::Aborted)
        ));
    }

    #[test]
    fn cancellation_precedes_invalid_pss_parameters() {
        let abort = AbortOnPoll {
            abort_on: 2,
            polls: AtomicUsize::new(0),
        };

        let result = run_pss_analysis_with_abort("invalid", -1.0, 0, -1.0, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn bound_op_environment_scales_supply_once_and_preserves_exact_temperature() {
        let mut netlist =
            rspice_core::Netlist::parse("seed environment\nVDD vdd 0 DC 1\nR1 vdd 0 1k\n.end\n")
                .unwrap();
        let temperature_kelvin = apply_seed_environment(
            &mut netlist,
            125.0,
            Some(1.2),
            Some(1.0),
            &["VDD".to_owned()],
            &NoAbort,
        )
        .unwrap();
        assert_eq!(temperature_kelvin.to_bits(), 398.15_f64.to_bits());
        let supply = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("VDD"))
            .expect("supply remains present");
        let rspice_core::netlist::ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(
            value,
        )) = &supply.kind
        else {
            panic!("expected scalar DC supply")
        };
        assert_eq!(value.to_bits(), 1.2_f64.to_bits());
    }

    #[test]
    fn seeded_pss_preserves_bound_op_temperature_and_tolerance_over_deck_options() {
        let source = "seeded temperature\n\
            V1 drive 0 SIN(1 0.1 1Meg)\n\
            R1 drive out 1k TC1=0.01\n\
            R2 out 0 1k\n\
            C1 out 0 159.154943091895p\n\
            .options temp=25 tnom=25 reltol=0.25\n\
            .end\n";
        let netlist = rspice_core::Netlist::parse(source).unwrap();
        let op_config = rspice_core::engine::SimulationConfig {
            temperature: 125.0 + 273.15,
            tolerance: 1.0e-2,
            ..Default::default()
        };
        let op_engine = Engine::try_new_with_resolved_config(op_config).unwrap();
        let op = op_engine.run_dc_op(&netlist).expect("bound OP solves");
        let seed = PssDcOperatingPointSeed::try_new(
            op.node_names.iter().skip(1).cloned().collect(),
            op.branch_names.clone(),
            op.node_voltages
                .iter()
                .skip(1)
                .copied()
                .chain(op.branch_currents.iter().copied())
                .collect(),
        )
        .unwrap();
        let config = PssRunConfig {
            fundamental_freq: 1.0e6,
            tone_sources: vec!["V1".to_owned()],
            tstab_periods: 0,
            points_per_period: 64,
            num_harmonics: 4,
            tolerance: 1.0e-2,
            oscillator_mode: false,
            oscillator_node: None,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        };
        let result = run_pss_analysis_with_dc_seed_and_source_path_and_abort(
            source,
            &config,
            None,
            PssSeedEnvironment {
                dc_seed: &seed,
                temperature_celsius: 125.0,
                supply_voltage: None,
                nominal_supply_voltage: None,
                supply_source_names: &[],
            },
            &NoAbort,
        )
        .expect("seeded PSS solves with the OP environment");
        assert_eq!(
            result.operating_point.config().tolerance.to_bits(),
            1.0e-2_f64.to_bits()
        );
        let (_, output) = result
            .waveforms
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
            .expect("temperature-sensitive divider waveform is retained");
        assert_eq!(output.len(), 65, "one exact period plus its endpoint");
        let period_mean =
            output[..output.len() - 1].iter().sum::<f64>() / (output.len() - 1) as f64;
        assert!(
            (period_mean - 1.0 / 3.0).abs() < 5.0e-3,
            "seeded PSS must apply the bound 125 C temperature to R1; mean V(out)={period_mean}, expected 1/3 V"
        );
        assert!(
            (period_mean - 0.5).abs() > 0.1,
            "deck TEMP=25 incorrectly overrode the bound 125 C environment; mean V(out)={period_mean}"
        );
    }

    #[test]
    fn pss_rejects_a_dc_only_or_unknown_tone_before_solving() {
        let netlist = "PSS source validation\nVBIAS out 0 1\nR1 out 0 1k\n.end\n";
        let config = PssRunConfig {
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VBIAS".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            num_harmonics: 20,
            tolerance: 1.0e-7,
            oscillator_mode: false,
            oscillator_node: None,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        };

        let error = run_pss_analysis_with_config_and_source_path_and_abort(
            netlist, &config, None, &NoAbort,
        )
        .expect_err("DC-only sources cannot authenticate a periodic solve");
        assert!(error.to_string().contains("tone-source validation"));
    }

    /// The one oscillator in the workspace that self-starts and converges in
    /// autonomous shooting mode. `crates/rspice-core/tests/pss_shooting.rs`
    /// solves this same deck and pins its period against the van der Pol
    /// closed form. `i1` is a one-shot startup kick, not a periodic drive.
    const NEGATIVE_RESISTANCE_OSCILLATOR: &str = "* negative-resistance lc oscillator\n\
         l1 osc 0 1u\n\
         c1 osc 0 1u\n\
         b1 osc 0 i=-0.05*v(osc)+0.025*v(osc)*v(osc)*v(osc)\n\
         i1 0 osc pulse(0 1 10u 10n 10n 1u 1)\n\
         .end\n";

    #[test]
    fn pss_runner_uses_the_configured_grid_in_source_preflight() {
        let source = "source defaults\nV1 in 0 PULSE(0 1 0.7u 0 0 0.28u 1u)\nR1 in out 1k\nC1 out 0 1n\n.end\n";
        for (points, periodic) in [(32, false), (512, true)] {
            let config = PssRunConfig {
                fundamental_freq: 1e6,
                tone_sources: vec!["V1".to_owned()],
                points_per_period: points,
                num_harmonics: 4,
                tstab_periods: 0,
                tolerance: 1e-7,
                oscillator_mode: false,
                oscillator_node: None,
                integration_method: None,
                tstab: 0.0,
                max_iterations: 100,
                abstol: 1.0e-12,
                damping: 1.0,
                max_period_change: 0.1,
                verbose: false,
            };
            let result = run_pss_analysis_with_config_and_source_path_and_abort(
                source, &config, None, &NoAbort,
            );
            assert_eq!(result.is_ok(), periodic, "{points}: {:?}", result.err());
        }
    }

    /// An autonomous solve holds its period as an unknown, so the driven
    /// periodic-source contract cannot be asked of it and the Studio's draft
    /// validator does not ask it. The runner used to ask it anyway, and the
    /// two halves then disagreed about the same run: this deck's only source
    /// is a delayed one-shot kick, which that contract refuses by name, while
    /// omitting it from the selection is refused as an incomplete set. No
    /// oscillator could satisfy both, so no oscillator could be solved from
    /// the Studio at all.
    #[test]
    fn an_autonomous_carrier_solves_from_a_deck_whose_only_source_is_a_startup_kick() {
        let period_guess = 6.3e-6;
        let config = PssRunConfig {
            fundamental_freq: 1.0 / period_guess,
            // What the Studio's own autonomous draft projects: no tones.
            tone_sources: Vec::new(),
            tstab_periods: 30,
            points_per_period: 256,
            num_harmonics: 9,
            tolerance: 1.0e-6,
            oscillator_mode: true,
            oscillator_node: Some("osc".to_owned()),
            integration_method: None,
            tstab: 0.0,
            max_iterations: 100,
            abstol: 1.0e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        };

        let data = run_pss_analysis_with_config_and_source_path_and_abort(
            NEGATIVE_RESISTANCE_OSCILLATOR,
            &config,
            None,
            &NoAbort,
        )
        .expect("the autonomous carrier converges through the Studio's own runner");

        let result = &data.operating_point.analysis().result;
        assert!(
            result.period_detected,
            "an autonomous orbit's period is a solver unknown, not authored input"
        );
        assert_ne!(
            result.frequency.to_bits(),
            config.fundamental_freq.to_bits(),
            "the shooting solver moves the period off the authored guess, which is exactly \
             why a consumer may not compare its own basis against that guess bit for bit"
        );
        let tank = 1.0 / (2.0 * std::f64::consts::PI * 1.0e-6);
        assert!(
            (result.frequency - tank).abs() < 1.0e-3 * tank,
            "the solved carrier is the tank's own frequency {tank}, not the guess: {}",
            result.frequency
        );
    }
}

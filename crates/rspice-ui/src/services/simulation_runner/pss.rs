//! Periodic steady-state analysis.
//!
//! Finds the periodic operating point a driven circuit settles into, and the
//! autonomous oscillation frequency for an oscillator. Every periodic
//! small-signal analysis linearizes about the result.

#![allow(clippy::type_complexity)]

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
#[cfg(test)]
use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, PssDcOperatingPointSeed};
use std::path::Path;
use std::sync::Arc;

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

/// Fully materialized shooting-PSS request used by the execution pipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct PssRunConfig {
    pub fundamental_freq: Value,
    pub tone_sources: Vec<String>,
    pub tstab_periods: usize,
    pub points_per_period: usize,
    pub num_harmonics: usize,
    pub tolerance: Value,
    pub oscillator_mode: bool,
    pub oscillator_node: Option<String>,
}

impl PssRunConfig {
    pub fn new(
        fundamental_freq: Value,
        tone_sources: Vec<String>,
        num_harmonics: usize,
        tolerance: Value,
    ) -> Self {
        Self {
            fundamental_freq,
            tone_sources,
            tstab_periods: 20,
            points_per_period: 512,
            num_harmonics,
            tolerance,
            oscillator_mode: false,
            oscillator_node: None,
        }
    }
}

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
pub fn run_pss_analysis_with_dc_seed_and_source_path_and_abort(
    netlist_text: &str,
    config: &PssRunConfig,
    source_path: Option<&Path>,
    dc_seed: &PssDcOperatingPointSeed,
    temperature_celsius: Value,
    supply_voltage: Option<Value>,
    nominal_supply_voltage: Option<Value>,
    supply_source_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PssData> {
    run_pss_analysis_internal(
        netlist_text,
        config,
        source_path,
        Some(PssSeedEnvironment {
            dc_seed,
            temperature_celsius,
            supply_voltage,
            nominal_supply_voltage,
            supply_source_names,
        }),
        abort,
    )
}

struct PssSeedEnvironment<'a> {
    dc_seed: &'a PssDcOperatingPointSeed,
    temperature_celsius: Value,
    supply_voltage: Option<Value>,
    nominal_supply_voltage: Option<Value>,
    supply_source_names: &'a [String],
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

    let mut netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

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

    let mut sim_config = build_engine_config(&netlist, None);
    sim_config.tolerance = config.tolerance;
    if let Some(temperature_kelvin) = seeded_temperature_kelvin {
        sim_config.temperature = temperature_kelvin;
    }
    let engine = Engine::try_new_with_resolved_config(sim_config).map_err(|error| {
        ServiceRunError::from_core(
            "PSS resolved engine configuration is invalid",
            rspice_core::SimulationError::Configuration(error),
        )
    })?;
    engine
        .validate_periodic_source_contract_with_abort(
            &netlist,
            &config.tone_sources,
            config.fundamental_freq,
            abort,
        )
        .map_err(|error| ServiceRunError::from_core("PSS tone-source validation failed", error))?;

    let pss_config = core_pss_config(config);

    let operating_point = match seed_environment {
        Some(environment) => engine
            .run_pss_operating_point_with_dc_seed_and_abort(
                &netlist,
                pss_config,
                environment.dc_seed,
                abort,
            )
            .map_err(|error| ServiceRunError::from_core("PSS error", error))?,
        None => engine
            .run_pss_operating_point_with_abort(&netlist, pss_config, abort)
            .map_err(|error| ServiceRunError::from_core("PSS error", error))?,
    };
    let pss_result = operating_point.analysis();

    let period = pss_result.period;
    if !period.is_finite() || period <= 0.0 {
        return Err(ServiceRunError::Failure(
            "PSS solver returned an invalid period".to_string(),
        ));
    }
    let mut time = Vec::with_capacity(pss_result.result.time.len());
    for (sample_idx, sample) in pss_result.result.time.iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        time.push(*sample);
    }

    let mut waveforms: Vec<(String, Vec<Value>)> = Vec::new();
    let node_names = &pss_result.result.node_names;
    for (node_name, waveform) in node_names.iter().zip(&pss_result.result.waveforms) {
        ensure_not_aborted(abort)?;
        if node_name == "0" || node_name.eq_ignore_ascii_case("gnd") {
            continue;
        }
        let mut values = Vec::with_capacity(waveform.values.len());
        for (sample_idx, sample) in waveform.values.iter().enumerate() {
            poll_periodically(abort, sample_idx)?;
            values.push(*sample);
        }
        waveforms.push((format!("V({node_name})"), values));
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
            super::apply_voltage_corner(netlist, supply, nominal, supply_source_names, abort)?;
        }
        _ => {
            return Err(ServiceRunError::Failure(
                "PSS DC seed supply and nominal voltage must be present together".to_owned(),
            ));
        }
    }
    Ok(temperature_celsius + 273.15)
}

fn core_pss_config(config: &PssRunConfig) -> PssConfig {
    let mut pss_config = if config.oscillator_mode {
        PssConfig::autonomous().with_period_guess(1.0 / config.fundamental_freq)
    } else {
        PssConfig::new(config.fundamental_freq)
    }
    // Core shooting requires at least one harmonic for its internal result
    // schema. A requested retention count of zero remains exact in the
    // service identity and yields an empty public harmonic payload.
    .with_harmonics(config.num_harmonics.max(1))
    .with_tolerance(config.tolerance)
    .with_max_iterations(100)
    .with_tstab_periods(config.tstab_periods)
    .with_points_per_period(config.points_per_period);
    if let Some(node) = config.oscillator_node.as_deref() {
        pss_config = pss_config.with_oscillator_node(node);
    }
    pss_config
}

fn validate_pss_config(config: &PssRunConfig) -> Result<(), String> {
    if !config.fundamental_freq.is_finite() || config.fundamental_freq <= 0.0 {
        return Err("PSS fundamental frequency must be positive".to_string());
    }
    if !config.oscillator_mode && config.tone_sources.is_empty() {
        return Err("PSS must bind at least one periodic tone source".to_owned());
    }
    for (index, source) in config.tone_sources.iter().enumerate() {
        if source.trim().is_empty() || source.chars().any(char::is_control) {
            return Err(format!("PSS tone source {} is invalid", index + 1));
        }
        if config.tone_sources[..index]
            .iter()
            .any(|prior| prior.eq_ignore_ascii_case(source))
        {
            return Err(format!("PSS tone source '{source}' is duplicated"));
        }
    }
    if config.points_per_period < 16 {
        return Err("PSS points_per_period must be at least 16".to_owned());
    }
    if config
        .num_harmonics
        .max(1)
        .checked_mul(2)
        .is_none_or(|minimum| config.points_per_period < minimum)
    {
        return Err("PSS points_per_period must be at least twice num_harmonics".to_owned());
    }
    if !config.tolerance.is_finite() || config.tolerance <= 0.0 {
        return Err("PSS tolerance must be positive".to_string());
    }
    if config.oscillator_mode
        && config
            .oscillator_node
            .as_deref()
            .is_none_or(|node| node.trim().is_empty())
    {
        return Err("PSS oscillator node is required in oscillator mode".to_string());
    }
    Ok(())
}

/// Harmonic content of one periodic waveform: `(frequency, magnitude,
/// phase in degrees)` per retained harmonic.
///
/// The spectrum analysis derived from a converged PSS state reads this too,
/// so it stays the one owner of the transform rather than each caller
/// growing its own.
pub fn compute_fft_harmonics_with_abort(
    waveform: &[Value],
    fundamental_freq: Value,
    num_harmonics: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(Value, Value, Value)>> {
    use std::f64::consts::PI;

    ensure_not_aborted(abort)?;
    let n = waveform.len();
    if n == 0 {
        return Ok(Vec::new());
    }

    let harmonic_count = num_harmonics.checked_add(1).ok_or_else(|| {
        ServiceRunError::Failure("PSS harmonic count exceeds this platform".to_string())
    })?;
    let mut harmonics = Vec::new();
    harmonics
        .try_reserve_exact(harmonic_count)
        .map_err(|error| {
            ServiceRunError::Failure(format!(
                "PSS harmonic allocation for {harmonic_count} components failed: {error}"
            ))
        })?;
    let mut dc = 0.0;
    for (sample_idx, sample) in waveform.iter().enumerate() {
        poll_periodically(abort, sample_idx)?;
        dc += *sample;
    }
    dc /= n as Value;
    harmonics.push((0.0, dc, 0.0));

    for harmonic in 1..=num_harmonics {
        ensure_not_aborted(abort)?;
        let freq = fundamental_freq * harmonic as Value;
        let mut real = 0.0;
        let mut imag = 0.0;

        for (sample_idx, &sample) in waveform.iter().enumerate() {
            poll_periodically(abort, sample_idx)?;
            let phase = 2.0 * PI * harmonic as Value * sample_idx as Value / n as Value;
            real += sample * phase.cos();
            imag -= sample * phase.sin();
        }

        real *= 2.0 / n as Value;
        imag *= 2.0 / n as Value;

        let magnitude = (real * real + imag * imag).sqrt();
        let phase_deg = imag.atan2(real).to_degrees();
        harmonics.push((freq, magnitude, phase_deg));
    }

    ensure_not_aborted(abort)?;
    Ok(harmonics)
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
    fn pss_fft_observes_abort_inside_nested_sample_loop() {
        let abort = AbortOnPoll {
            abort_on: 9,
            polls: AtomicUsize::new(0),
        };
        let waveform = vec![1.0; 128];

        let result = compute_fft_harmonics_with_abort(&waveform, 1.0e6, 20, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
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
        let mut op_config = rspice_core::engine::SimulationConfig::default();
        op_config.temperature = 125.0 + 273.15;
        op_config.tolerance = 1.0e-2;
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
        };
        let result = run_pss_analysis_with_dc_seed_and_source_path_and_abort(
            source,
            &config,
            None,
            &seed,
            125.0,
            None,
            None,
            &[],
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
    fn configured_shooting_fields_reach_the_core_solver_request() {
        let config = PssRunConfig {
            fundamental_freq: 2.0e6,
            tone_sources: vec!["VOSC".to_owned()],
            tstab_periods: 37,
            points_per_period: 1024,
            num_harmonics: 13,
            tolerance: 2.0e-5,
            oscillator_mode: true,
            oscillator_node: Some("osc".to_owned()),
        };

        let core = core_pss_config(&config);
        assert!(core.auto_period);
        assert_eq!(core.period_guess, 0.5e-6);
        assert_eq!(core.num_harmonics, 13);
        assert_eq!(core.tolerance, 2.0e-5);
        assert_eq!(core.max_iterations, 100);
        assert_eq!(core.tstab_periods, 37);
        assert_eq!(core.points_per_period, 1024);
        assert_eq!(core.oscillator_node.as_deref(), Some("osc"));
    }

    #[test]
    fn zero_harmonic_retention_keeps_a_valid_internal_shooting_contract() {
        let config = PssRunConfig {
            fundamental_freq: 1.0e6,
            tone_sources: vec!["VCLK".to_owned()],
            tstab_periods: 20,
            points_per_period: 512,
            num_harmonics: 0,
            tolerance: 1.0e-7,
            oscillator_mode: false,
            oscillator_node: None,
        };

        validate_pss_config(&config).expect("zero retained harmonics is valid");
        let core = core_pss_config(&config);
        assert_eq!(core.num_harmonics, 1);
        assert_eq!(config.num_harmonics, 0);
        assert!(core.validate().is_ok());
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
        };

        let error = run_pss_analysis_with_config_and_source_path_and_abort(
            netlist, &config, None, &NoAbort,
        )
        .expect_err("DC-only sources cannot authenticate a periodic solve");
        assert!(error.to_string().contains("tone-source validation"));
    }
}

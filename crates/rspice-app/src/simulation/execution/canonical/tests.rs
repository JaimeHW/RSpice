//! Canonical execution identity and numerical control regressions.

use super::*;
use crate::services::drc::{DrcLocation, DrcViolation};
use crate::simulation::config::{NoiseContributionDetail, NoiseIntegrationMode, NoiseSweepType};
use crate::simulation::multi_run::{
    EnvelopeAdaptiveMode, EnvelopeExtractionPath, EnvelopeInitialPeriodicSolve, FrequencySweep,
    TfAccuracy, TfNormalization,
};

#[test]
fn analysis_kind_tags_are_append_only() {
    // These tags are hashed into the config digest, so renumbering one
    // silently redefines every prepared snapshot already on disk. Pin the
    // boundary: PSS keeps 7, and the newest variant took the next free
    // number rather than a gap.
    assert_eq!(
        analysis_kind_tag(&AnalysisSpec::PssSpectrum { num_harmonics: 20 }),
        35
    );
    assert_eq!(analysis_kind_tag(&exact_pss_spec()), 7);
}

#[test]
fn ac_data_table_options_preserve_old_requests_and_separate_new_identities() {
    use crate::simulation::config::AcDataParameterColumn;
    use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;
    let json = serde_json::json!({"AcData": {"table_name": "pts", "frequencies": [1.0, 0.0]}});
    let old: AnalysisSpec = serde_json::from_value(json.clone()).unwrap();
    let worker: WorkerAnalysisSpec = serde_json::from_value(json).unwrap();
    assert_eq!(AnalysisSpec::from(worker), old);
    let digest = |spec: &AnalysisSpec| {
        let mut writer = CanonicalWriter::new("test");
        encode_analysis_spec(&mut writer, spec);
        writer.finish()
    };
    let mut previous = CanonicalWriter::new("test");
    previous.domain("analysis-spec");
    previous.u8(analysis_kind_tag(&old));
    previous.string("pts");
    encode_f64_slice(&mut previous, &[1.0, 0.0]);
    assert_eq!(digest(&old), previous.finish());
    let mut columns = old.clone();
    let AnalysisSpec::AcData { table_options, .. } = &mut columns else {
        unreachable!()
    };
    table_options.parameter_columns.push(AcDataParameterColumn {
        name: "load".into(),
        values: vec![1000.0, 2000.0],
    });
    assert_ne!(digest(&columns), digest(&old));
    let mut changed = columns.clone();
    let AnalysisSpec::AcData { table_options, .. } = &mut changed else {
        unreachable!()
    };
    table_options.parameter_columns[0].values[1] = 3000.0;
    assert_ne!(digest(&changed), digest(&columns));
    let mut reference = old.clone();
    let AnalysisSpec::AcData {
        frequencies,
        table_options,
        ..
    } = &mut reference
    else {
        unreachable!()
    };
    frequencies.clear();
    table_options.from_netlist = true;
    reference.validate().unwrap();
    assert_ne!(digest(&reference), digest(&old));
    for spec in [columns, reference] {
        let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
        let restored: WorkerAnalysisSpec =
            serde_json::from_value(serde_json::to_value(worker).unwrap()).unwrap();
        assert_eq!(AnalysisSpec::from(restored), spec);
    }
}

/// A transient-noise plan with no authored floor digests to exactly the
/// bytes it digested to before the field existed.
///
/// The identity of every prepared snapshot already on disk depends on it.
/// The reference is built from the writer primitives in the order the arm
/// used before the floor was added, rather than pinned as a hex literal:
/// a literal states what the answer is, and this states *why* it is that —
/// the eight fields, nothing appended. A `writer.option` around the new
/// field would tag the `None` and fail here, which is the point.
#[test]
fn an_absent_noise_floor_leaves_the_plan_digest_unchanged() {
    let transient_noise = |noise_fmin| AnalysisSpec::TransientNoise {
        stop_time: 1.0e-6,
        step_time: 1.0e-9,
        start_time: 2.0e-7,
        max_timestep: 2.5e-10,
        seed: Some(97),
        noise_fmax: 5.0e8,
        noise_fmin,
        scale: 0.5,
        uic: true,
    };
    let spec = transient_noise(None);
    let mut encoded = CanonicalWriter::new("test");
    encode_analysis_spec(&mut encoded, &spec);

    let mut before_the_field = CanonicalWriter::new("test");
    before_the_field.domain("analysis-spec");
    before_the_field.u8(analysis_kind_tag(&spec));
    before_the_field.f64(1.0e-6);
    before_the_field.f64(1.0e-9);
    before_the_field.f64(2.0e-7);
    before_the_field.f64(2.5e-10);
    before_the_field.u64(97);
    before_the_field.f64(5.0e8);
    before_the_field.f64(0.5);
    before_the_field.bool(true);

    assert_eq!(
        encoded.finish(),
        before_the_field.finish(),
        "an unauthored noise floor must not move a saved plan's identity"
    );

    // And an authored one must move it, or two plans running different
    // bands would share one identity.
    let mut authored = CanonicalWriter::new("test");
    encode_analysis_spec(&mut authored, &transient_noise(Some(1.0e3)));
    let mut absent = CanonicalWriter::new("test");
    encode_analysis_spec(&mut absent, &spec);
    assert_ne!(authored.finish(), absent.finish());

    for floor in [None, Some(1.0e3)] {
        let mut inherited = transient_noise(floor);
        let AnalysisSpec::TransientNoise { seed, .. } = &mut inherited else {
            unreachable!()
        };
        *seed = None;
        let mut explicit_zero = inherited.clone();
        let AnalysisSpec::TransientNoise { seed, .. } = &mut explicit_zero else {
            unreachable!()
        };
        *seed = Some(0);
        let digest = |spec: &AnalysisSpec| {
            let mut writer = CanonicalWriter::new("test");
            encode_analysis_spec(&mut writer, spec);
            writer.finish()
        };
        assert_ne!(digest(&inherited), digest(&explicit_zero));
    }
}

/// A sensitivity plan restored from a project saved before filters
/// existed digests to exactly the bytes it digested to then.
///
/// Same shape of reference as the two above — the three fields the arm
/// wrote, nothing appended — with one difference that is the whole point:
/// the value that leaves the digest alone is `PARAM:*`, not the empty
/// string. A saved plan computed the deck's design parameters, so that is
/// what it must go on computing, and a plan whose filter was emptied runs
/// a different analysis and has to say so.
#[test]
fn a_design_parameter_filter_at_one_frequency_leaves_the_plan_digest_unchanged() {
    let sensitivity = |filter: &str| AnalysisSpec::Sensitivity {
        output_var: "V(out)".to_owned(),
        ac_mode: true,
        frequency: Some(1.0e6),
        filter: filter.to_owned(),
        sweep: None,
    };
    let spec = sensitivity(crate::simulation::config::DESIGN_PARAMETERS_FILTER);
    let mut encoded = CanonicalWriter::new("test");
    encode_analysis_spec(&mut encoded, &spec);

    let mut before_the_field = CanonicalWriter::new("test");
    before_the_field.domain("analysis-spec");
    before_the_field.u8(analysis_kind_tag(&spec));
    before_the_field.string("V(out)");
    before_the_field.bool(true);
    before_the_field.option(Some(&1.0e6), |writer, value| writer.f64(*value));

    assert_eq!(
        encoded.finish(),
        before_the_field.finish(),
        "a plan restored from before filters existed must keep its identity"
    );
}

/// Emptying the filter is a different analysis, and a different plan.
///
/// The engine reads an empty filter as every device and model parameter
/// and no design parameter — the opposite selection from `PARAM:*`. If
/// the two digested alike, a stored result computed under one would be
/// presented as current for the other.
#[test]
fn an_emptied_filter_is_a_different_plan_from_one_saved_before_filters() {
    let sensitivity = |filter: &str| AnalysisSpec::Sensitivity {
        output_var: "V(out)".to_owned(),
        ac_mode: false,
        frequency: None,
        filter: filter.to_owned(),
        sweep: None,
    };
    let digest = |filter: &str| {
        let mut writer = CanonicalWriter::new("test");
        encode_analysis_spec(&mut writer, &sensitivity(filter));
        writer.finish()
    };
    let legacy = digest(crate::simulation::config::DESIGN_PARAMETERS_FILTER);
    assert_ne!(legacy, digest(""));
    assert_ne!(legacy, digest("R* PARAM:*"));
    assert_ne!(digest(""), digest("R*"));
    assert_eq!(digest("R*"), digest("R*"));
}

/// A DC mismatch plan with no authored share threshold digests to exactly
/// the bytes it digested to before the field existed.
///
/// Same reasoning as the noise floor above, and the same shape of
/// reference: the six fields the arm wrote, nothing appended. `.DCMATCH`'s
/// own default threshold is zero, so a `writer.option` tag — or an
/// unconditional `writer.f64(0.0)` — would redefine the identity of every
/// DC mismatch plan already saved.
#[test]
fn dc_mismatch_moment_controls_preserve_defaults_and_distinguish_policies() {
    let dc_mismatch = |contribution_threshold| AnalysisSpec::DcMismatch {
        moment_options: Default::default(),
        output_expression: "V(out)".to_owned(),
        sigma_multiplier: 1.0,
        contributor_limit: 10,
        include_process: false,
        include_mismatch: true,
        normalized_contributions: true,
        contribution_threshold,
    };
    let spec = dc_mismatch(None);
    let mut encoded = CanonicalWriter::new("test");
    encode_analysis_spec(&mut encoded, &spec);

    let mut before_the_field = CanonicalWriter::new("test");
    before_the_field.domain("analysis-spec");
    before_the_field.u8(analysis_kind_tag(&spec));
    before_the_field.string("V(out)");
    before_the_field.f64(1.0);
    before_the_field.usize(10);
    before_the_field.bool(false);
    before_the_field.bool(true);
    before_the_field.bool(true);

    assert_eq!(
        encoded.finish(),
        before_the_field.finish(),
        "an unauthored share threshold must not move a saved plan's identity"
    );

    // And an authored one must move it, or two plans trimming their
    // contributor lists differently would share one identity.
    let mut authored = CanonicalWriter::new("test");
    encode_analysis_spec(&mut authored, &dc_mismatch(Some(0.05)));
    let mut absent = CanonicalWriter::new("test");
    encode_analysis_spec(&mut absent, &spec);
    assert_ne!(authored.finish(), absent.finish());
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest("", spec, None, &SpecExecutionOptions::default(), None)
    };
    for change_tolerance in [false, true] {
        let mut changed = spec.clone();
        let AnalysisSpec::DcMismatch { moment_options, .. } = &mut changed else {
            unreachable!()
        };
        if change_tolerance {
            moment_options.relative_tolerance *= 2.0;
        } else {
            moment_options.max_points *= 2;
        }
        assert_ne!(digest(&changed), digest(&spec));
    }
}

/// A Monte Carlo plan that named no subset digests to exactly the bytes it
/// digested to when the arm wrote nothing at all.
///
/// The arm was empty, so the reference is the kind tag and nothing after
/// it. That makes the conditional tail load-bearing in both directions: an
/// unnamed subset must not move a saved plan's identity, and two runs that
/// vary different parameters must not share one.
#[test]
fn an_unauthored_vary_only_leaves_the_plan_digest_unchanged() {
    let monte_carlo = |params: Vec<String>| AnalysisSpec::MonteCarlo {
        variation_source: crate::simulation::dialog::McVariationSource::ParameterTolerance,
        params,
    };
    let spec = monte_carlo(Vec::new());
    let mut encoded = CanonicalWriter::new("test");
    encode_analysis_spec(&mut encoded, &spec);

    let mut before_the_field = CanonicalWriter::new("test");
    before_the_field.domain("analysis-spec");
    before_the_field.u8(analysis_kind_tag(&spec));

    assert_eq!(
        encoded.finish(),
        before_the_field.finish(),
        "an unnamed Monte Carlo subset must not move a saved plan's identity"
    );

    let mut named = CanonicalWriter::new("test");
    encode_analysis_spec(&mut named, &monte_carlo(vec!["RLOAD".to_owned()]));
    let mut unnamed = CanonicalWriter::new("test");
    encode_analysis_spec(&mut unnamed, &spec);
    assert_ne!(named.finish(), unnamed.finish());

    // Order is part of the request the card writes, so it is part of the
    // identity too.
    let mut forward = CanonicalWriter::new("test");
    encode_analysis_spec(
        &mut forward,
        &monte_carlo(vec!["RA".to_owned(), "RB".to_owned()]),
    );
    let mut reversed = CanonicalWriter::new("test");
    encode_analysis_spec(
        &mut reversed,
        &monte_carlo(vec!["RB".to_owned(), "RA".to_owned()]),
    );
    assert_ne!(forward.finish(), reversed.finish());
}

#[test]
fn a_pss_spectrum_digest_follows_its_harmonic_count() {
    let twenty = AnalysisSpec::PssSpectrum { num_harmonics: 20 };
    let nine = AnalysisSpec::PssSpectrum { num_harmonics: 9 };
    let mut left = CanonicalWriter::new("test");
    let mut right = CanonicalWriter::new("test");
    encode_analysis_spec(&mut left, &twenty);
    encode_analysis_spec(&mut right, &nine);

    assert_ne!(left.finish(), right.finish());
}

fn exact_pss_spec() -> AnalysisSpec {
    AnalysisSpec::Pss {
        method: crate::simulation::multi_run::PssMethod::Shooting,
        fundamental_freq: 1.0e6,
        tone_sources: vec!["VIN".to_owned()],
        tstab_periods: 20,
        points_per_period: 512,
        tolerance: 1.0e-6,
        oscillator_mode: false,
        oscillator_node: None,
        num_harmonics: 20,
        integration_method: None,
        tstab: 0.0,
        max_iterations: 100,
        abstol: 1.0e-12,
        damping: 1.0,
        max_period_change: 0.1,
        verbose: false,
    }
}

fn exact_noise_spec() -> AnalysisSpec {
    AnalysisSpec::Noise {
        output_node: "out".to_owned(),
        reference_node: "ref".to_owned(),
        input_source: "VIN".to_owned(),
        start_freq: 10.0,
        stop_freq: 1.0e6,
        points_per_decade: 30,
        sweep: NoiseSweepType::Decade,
        explicit_frequencies: None,
        data_table_name: None,
        contribution_detail: NoiseContributionDetail::Top50,
        integration_mode: NoiseIntegrationMode::Enabled,
        temperature: 300.15,
    }
}

/// The solver trace is part of the request's identity.
///
/// It changes no number the solve produces, and it is still encoded, for
/// the reason the whole encoder works on: a digest identifies the request
/// the engine was handed, not the answer it gave back. The byte was
/// already on the end of the harmonic-balance arm before any form could
/// author it, so a request that leaves the switch off digests exactly as
/// it did — which is what lets every sealed manifest still open.
#[test]
fn a_harmonic_balance_request_that_asks_for_a_solver_trace_is_a_different_request() {
    let base = AnalysisSpec::HarmonicBalance {
        tones: vec![crate::simulation::multi_run::HbToneSpec::new(1.0e9, 9)],
        reltol: 1.0e-6,
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
    };
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(".hb", spec, None, &SpecExecutionOptions::default(), None)
    };
    let quiet = digest(&base);
    let mut traced = base.clone();
    let AnalysisSpec::HarmonicBalance { verbose, .. } = &mut traced else {
        unreachable!()
    };
    *verbose = true;
    assert_ne!(quiet, digest(&traced));
}

#[test]
fn sp_noise_changes_identity_even_when_the_directive_is_unchanged() {
    let mut spec = AnalysisSpec::SParameter {
        start_freq: 1e6,
        stop_freq: 3e6,
        points_per_unit: 3,
        sweep: FrequencySweep::Linear,
        z0: 50.0,
        ports: Vec::new(),
        do_noise: false,
    };
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(".sp", spec, None, &SpecExecutionOptions::default(), None)
    };
    let scattering = digest(&spec);
    let AnalysisSpec::SParameter { do_noise, .. } = &mut spec else {
        unreachable!()
    };
    *do_noise = true;
    assert_ne!(scattering, digest(&spec));
}

#[test]
fn noise_digest_changes_for_every_exact_execution_field() {
    let base = exact_noise_spec();
    let digest = |spec: &AnalysisSpec, config: Option<&AnalysisConfig>| {
        analysis_config_digest(
            ".noise",
            spec,
            config,
            &SpecExecutionOptions::default(),
            None,
        )
    };
    let baseline = digest(&base, None);

    let mut variants = Vec::new();
    macro_rules! changed {
        ($field:ident, $value:expr) => {{
            let mut spec = base.clone();
            let AnalysisSpec::Noise { $field, .. } = &mut spec else {
                unreachable!()
            };
            *$field = $value;
            variants.push(spec);
        }};
    }
    changed!(output_node, "other".to_owned());
    changed!(reference_node, "0".to_owned());
    changed!(input_source, "VOTHER".to_owned());
    changed!(start_freq, 11.0);
    changed!(stop_freq, 2.0e6);
    changed!(points_per_decade, 31);
    changed!(sweep, NoiseSweepType::Octave);
    changed!(explicit_frequencies, Some(vec![10.0, 100.0]));
    changed!(data_table_name, Some("points".to_owned()));
    changed!(contribution_detail, NoiseContributionDetail::Top20);
    changed!(integration_mode, NoiseIntegrationMode::OutputNoiseOnly);
    changed!(temperature, 398.15);

    for variant in variants {
        assert_ne!(baseline, digest(&variant, None), "variant: {variant:?}");
    }

    let base_config = crate::simulation::config::NoiseAnalysisConfig::default();
    let mut changed_config = base_config.clone();
    changed_config.contribution_detail = NoiseContributionDetail::SummaryOnly;
    assert_ne!(
        digest(&base, Some(&AnalysisConfig::Noise(base_config)),),
        digest(&base, Some(&AnalysisConfig::Noise(changed_config)),)
    );
}

/// A filtered run space narrows to the same axis values as the full
/// expansion it came from, so the axes alone cannot tell the two apart. If
/// the digest could not either, a filtered run and the full run would share
/// one identity and either's results could be attributed to the other.
#[test]
fn corner_digest_changes_when_points_are_excluded_from_the_same_axes() {
    use crate::services::simulation_runner::{CornerPoint, CornerRunConfig};
    use rspice_app_types::product::ProcessCorner;

    let axes = CornerRunConfig {
        process_corners: vec![ProcessCorner::TT],
        voltages: vec![0.9, 1.1],
        temperatures_c: vec![-40.0, 125.0],
        full_matrix: true,
        nominal_voltage: Some(1.0),
        ..CornerRunConfig::default()
    };
    let point = |voltage: f64, temperature_c: f64| CornerPoint {
        process: ProcessCorner::TT,
        voltage,
        temperature_c,
    };
    // Every axis value is still drawn on, so the two declarations are
    // indistinguishable from their axes alone.
    let filtered = CornerRunConfig {
        points: vec![point(0.9, -40.0), point(1.1, -40.0), point(1.1, 125.0)],
        ..axes.clone()
    };
    let reordered = CornerRunConfig {
        points: vec![point(1.1, 125.0), point(0.9, -40.0), point(1.1, -40.0)],
        ..axes.clone()
    };

    let digest = |corner: &CornerRunConfig| {
        analysis_config_digest(
            ".corner",
            &AnalysisSpec::Corner,
            None,
            &SpecExecutionOptions {
                corner: Some(corner.clone()),
                ..SpecExecutionOptions::default()
            },
            None,
        )
    };

    assert_ne!(digest(&axes), digest(&filtered));
    // Execution order is part of the contract: the manifest labels points
    // by position, so a reordered list is a different run.
    assert_ne!(digest(&filtered), digest(&reordered));
    assert_eq!(digest(&filtered), digest(&filtered.clone()));
}

/// An unauthored carrier leaves the plan digest exactly as it was.
///
/// The digest identifies the encoding as well as the request, so a field
/// appended in the middle of one of these arms would give every saved plan
/// in the family a new identity and detach it from its own results. The
/// carrier is therefore a conditional tail: the card's absent `FROM=` — the
/// only thing any request written before this lane could have meant — adds
/// no bytes, and a named carrier, which binds a different producer and is a
/// different run, adds one.
///
/// The reference is built from the same writer primitives in the order
/// that stood before the tail existed, so this is a check against the
/// encoding rather than against a recorded hash that would be regenerated
/// along with the defect.
#[test]
fn an_unauthored_carrier_leaves_the_plan_digest_unchanged() {
    use crate::services::simulation_runner::{PacRunConfig, PeriodicCarrier};

    let digest = |carrier: PeriodicCarrier| {
        analysis_config_digest(
            ".pac",
            &AnalysisSpec::Pac,
            None,
            &SpecExecutionOptions {
                pac: Some(PacRunConfig {
                    carrier,
                    ..PacRunConfig::default()
                }),
                ..SpecExecutionOptions::default()
            },
            None,
        )
    };

    // The same primitives in the same order `analysis_config_digest` and
    // `encode_spec_options` used before the tail existed.
    let config = PacRunConfig::default();
    let mut writer = CanonicalWriter::new("rspice.analysis-config/v4");
    writer.domain("analysis-line");
    writer.string(".pac");
    encode_analysis_spec(&mut writer, &AnalysisSpec::Pac);
    encode_analysis_config(&mut writer, None);
    writer.domain("spec-execution-options");
    for _ in 0..3 {
        writer.option(None::<&()>, |_, _: &()| unreachable!());
    }
    writer.option(Some(&config), |writer, config| {
        writer.f64(config.pss_fundamental_freq);
        writer.usize(config.pss_num_harmonics);
        writer.f64(config.pss_tolerance);
        writer.f64(config.start_freq);
        writer.f64(config.stop_freq);
        writer.usize(config.points_per_unit);
        writer.u8(pac_sweep_tag(config.sweep));
        writer.i32(config.sideband_max);
        writer.string(&config.input_source);
        writer.string(&config.output_node);
        writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
        writer.f64(config.pac_magnitude);
        writer.bool(config.include_dc);
        writer.f64(config.reltol);
        writer.f64(config.abstol);
    });
    // The `.PXF` slot, the retired TF flag, then `.PNOISE` and `.PSTB`.
    writer.option(None::<&()>, |_, _: &()| unreachable!());
    writer.bool(false);
    for _ in 0..2 {
        writer.option(None::<&()>, |_, _: &()| unreachable!());
    }
    encode_numeric_override(&mut writer, None);

    assert_eq!(
        digest(PeriodicCarrier::Preceding),
        writer.finish(),
        "the absent FROM= keyword must add no bytes to the digest"
    );
    assert_ne!(
        digest(PeriodicCarrier::Preceding),
        digest(PeriodicCarrier::Pss),
        "a named carrier binds a different producer and is a different run"
    );
    assert_ne!(
        digest(PeriodicCarrier::Pss),
        digest(PeriodicCarrier::Hb),
        "the two named carriers are two different runs"
    );
}

/// Giving the harmonic-balance carrier a route changes what a request can
/// *bind*, not what it digests.
///
/// The carrier byte already distinguished the three positions, and a run
/// linearized about a shooting `.PSS` is the same run it was before the
/// other family became routable. If this moved, every saved plan carrying
/// a `FROM=PSS` request would be detached from its own results — and the
/// plumbing that chooses a producer, the prerequisite role, and the
/// service entry are all outside the digest for exactly that reason.
///
/// Reconstructed from the writer primitives, in the order and at the
/// position the tail has always held, so a byte moved anywhere in the arm
/// fails here instead of being regenerated along with the defect. One arm
/// proves it for all three, because all three call the one
/// `encode_periodic_carrier_tail`, and the test above pins that function's
/// three positions.
#[test]
fn a_pss_carried_request_keeps_its_digest() {
    use crate::services::simulation_runner::{PacRunConfig, PeriodicCarrier};

    let config = PacRunConfig {
        carrier: PeriodicCarrier::Pss,
        ..PacRunConfig::default()
    };
    let digest = analysis_config_digest(
        ".pac",
        &AnalysisSpec::Pac,
        None,
        &SpecExecutionOptions {
            pac: Some(config.clone()),
            ..SpecExecutionOptions::default()
        },
        None,
    );

    let mut writer = CanonicalWriter::new("rspice.analysis-config/v4");
    writer.domain("analysis-line");
    writer.string(".pac");
    encode_analysis_spec(&mut writer, &AnalysisSpec::Pac);
    encode_analysis_config(&mut writer, None);
    writer.domain("spec-execution-options");
    for _ in 0..3 {
        writer.option(None::<&()>, |_, _: &()| unreachable!());
    }
    writer.option(Some(&config), |writer, config| {
        writer.f64(config.pss_fundamental_freq);
        writer.usize(config.pss_num_harmonics);
        writer.f64(config.pss_tolerance);
        writer.f64(config.start_freq);
        writer.f64(config.stop_freq);
        writer.usize(config.points_per_unit);
        writer.u8(pac_sweep_tag(config.sweep));
        writer.i32(config.sideband_max);
        writer.string(&config.input_source);
        writer.string(&config.output_node);
        writer.option(config.output_ref.as_ref(), |w, v| w.string(v));
        writer.f64(config.pac_magnitude);
        writer.bool(config.include_dc);
        writer.f64(config.reltol);
        writer.f64(config.abstol);
        // The shooting position's one byte, last in the arm, where the
        // symmetric sideband range adds nothing after it.
        writer.u8(0);
    });
    writer.option(None::<&()>, |_, _: &()| unreachable!());
    writer.bool(false);
    for _ in 0..2 {
        writer.option(None::<&()>, |_, _: &()| unreachable!());
    }
    encode_numeric_override(&mut writer, None);

    assert_eq!(
        digest,
        writer.finish(),
        "a FROM=PSS request digests the bytes it digested before the other family had a route"
    );
}

/// A symmetric sideband range digests exactly as the single bound it
/// replaced, and an asymmetric one earns its own identity.
///
/// The bottom of the range is the second conditional tail on this arm, for
/// the same reason the carrier is the first: every run recorded before the
/// range could be stated asymmetrically was symmetric, and giving those
/// runs a new digest would detach each from its own results.
#[test]
fn a_symmetric_sideband_range_leaves_the_plan_digest_unchanged() {
    use crate::services::simulation_runner::PacRunConfig;

    let digest = |sideband_min: i32, sideband_max: i32| {
        analysis_config_digest(
            ".pac",
            &AnalysisSpec::Pac,
            None,
            &SpecExecutionOptions {
                pac: Some(PacRunConfig {
                    sideband_min,
                    sideband_max,
                    ..PacRunConfig::default()
                }),
                ..SpecExecutionOptions::default()
            },
            None,
        )
    };

    // The default range is symmetric, so it must digest as the arm did
    // before the bottom end existed — which is what the reference in
    // `an_unauthored_carrier_leaves_the_plan_digest_unchanged` builds.
    assert_eq!(digest(-5, 5), digest(-5, 5));
    assert_ne!(
        digest(-5, 5),
        digest(-2, 5),
        "an asymmetric range is a different solve over different sidebands"
    );
    assert_ne!(digest(-2, 5), digest(-2, 7));
    // Zero is its own symmetric case: `-0 == 0`, so it adds no tail.
    assert_eq!(digest(0, 0), digest(0, 0));
}

#[test]
fn pss_digest_changes_for_every_exact_contract_value() {
    let base = AnalysisSpec::Pss {
        method: crate::simulation::multi_run::PssMethod::Shooting,
        fundamental_freq: 1.0e6,
        tone_sources: vec!["VCLK".to_owned()],
        tstab_periods: 20,
        points_per_period: 512,
        tolerance: 1.0e-7,
        oscillator_mode: false,
        oscillator_node: None,
        num_harmonics: 20,
        integration_method: None,
        tstab: 0.0,
        max_iterations: 100,
        abstol: 1.0e-12,
        damping: 1.0,
        max_period_change: 0.1,
        verbose: false,
    };
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(".pss", spec, None, &SpecExecutionOptions::default(), None)
    };
    let baseline = digest(&base);
    let mut variants = Vec::new();
    macro_rules! changed {
        ($field:ident, $value:expr) => {{
            let mut spec = base.clone();
            let AnalysisSpec::Pss { $field, .. } = &mut spec else {
                unreachable!()
            };
            *$field = $value;
            variants.push(spec);
        }};
    }
    changed!(
        method,
        crate::simulation::multi_run::PssMethod::HarmonicBalance
    );
    changed!(fundamental_freq, 2.0e6);
    changed!(tone_sources, vec!["VLO".to_owned(), "VRF".to_owned()]);
    changed!(tstab_periods, 21);
    changed!(points_per_period, 1024);
    changed!(tolerance, 2.0e-8);
    changed!(oscillator_mode, true);
    changed!(oscillator_node, Some("osc".to_owned()));
    changed!(num_harmonics, 21);
    changed!(
        integration_method,
        Some(crate::simulation::dialog::IntegrationMethod::Euler)
    );
    changed!(tstab, 3.0e-9);
    changed!(max_iterations, 250);
    changed!(abstol, 1.0e-15);
    changed!(damping, 0.75);
    changed!(max_period_change, 0.25);
    changed!(verbose, true);

    for variant in variants {
        assert_ne!(baseline, digest(&variant), "variant: {variant:?}");
    }
}

#[test]
fn operating_point_digest_is_sensitive_to_every_contract_field() {
    let base = AnalysisSpec::dc_op();
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(".op", spec, None, &SpecExecutionOptions::default(), None)
    };
    let baseline = digest(&base);

    let mut variants = Vec::new();
    macro_rules! changed {
        ($field:ident, $value:expr) => {{
            let mut spec = base.clone();
            let AnalysisSpec::DcOp { $field, .. } = &mut spec else {
                unreachable!()
            };
            *$field = $value;
            variants.push(spec);
        }};
    }
    changed!(temperature_mode, OpTemperatureMode::Explicit);
    changed!(temperature_celsius, 91.25);
    changed!(initial_guess, OpInitialGuess::ZeroState);
    changed!(initial_guess, OpInitialGuess::PreviousCompatible);
    changed!(node_initialization, OpNodeInitialization::ForceIcValues);
    changed!(homotopy, OpHomotopy::SourceStepping);
    changed!(annotation, OpAnnotation::VoltagesOnly);
    changed!(device_detail, OpDeviceDetail::AllDevices);
    changed!(save_device_op, OpSaveDevice::Disabled);
    changed!(accuracy, OpAccuracy::Accurate);
    changed!(selected_devices, vec!["M1".to_owned()]);
    changed!(
        previous_state,
        Some(OpPreviousState {
            source_content_digest: ContentDigest::from_bytes([1; 32]),
            producer_snapshot_digest: ContentDigest::from_bytes([2; 32]),
            producer_result_digest: ContentDigest::from_bytes([3; 32]),
            node_names: vec!["out".to_owned()],
            branch_names: vec!["V1".to_owned()],
            solution: vec![1.0, -1.0e-3],
        })
    );
    changed!(violation_devices, vec!["M2".to_owned()]);
    changed!(
        violation_source_content_digest,
        Some(ContentDigest::from_bytes([4; 32]))
    );
    changed!(
        run_point,
        OpRunPointContext {
            index: 1,
            count: 2,
            ..OpRunPointContext::default()
        }
    );

    for variant in variants {
        assert_ne!(
            baseline,
            digest(&variant),
            "unchanged digest for {variant:?}"
        );
    }
}

#[test]
fn length_prefixes_prevent_concatenation_collisions() {
    let mut left = CanonicalWriter::new("collision-test");
    left.string("ab");
    left.string("c");
    let mut right = CanonicalWriter::new("collision-test");
    right.string("a");
    right.string("bc");
    assert_ne!(left.finish(), right.finish());
}

#[test]
fn domain_separators_prevent_cross_type_collisions() {
    assert_ne!(
        content_digest("source/v1", b"same"),
        content_digest("model/v1", b"same")
    );
}

#[test]
fn fourier_result_controls_are_bound_into_the_config_digest() {
    let spec = |compute_thd, normalize| AnalysisSpec::Fourier {
        fundamental_freq: 1.0e6,
        num_harmonics: 10,
        num_periods: 1,
        output_node: "out".to_owned(),
        output_ref: "0".to_owned(),
        additional_outputs: Vec::new(),
        start_time: 0.0,
        stop_time: 10.0e-6,
        compute_thd,
        normalize,
    };
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(".four", spec, None, &SpecExecutionOptions::default(), None)
    };

    let baseline = digest(&spec(true, false));
    assert_ne!(baseline, digest(&spec(false, false)));
    assert_ne!(baseline, digest(&spec(true, true)));
}

/// A Fourier plan that names one output digests to exactly the bytes it
/// digested to when the arm ended at `normalize`.
///
/// The output list is appended as a conditional tail for that reason: a
/// one-output plan saved before the list existed must keep its identity,
/// while two runs that decompose different outputs must not share one.
#[test]
fn a_single_output_fourier_leaves_the_plan_digest_unchanged() {
    let fourier = |additional_outputs: Vec<String>| AnalysisSpec::Fourier {
        fundamental_freq: 1.0e6,
        num_harmonics: 10,
        num_periods: 1,
        output_node: "out".to_owned(),
        output_ref: "0".to_owned(),
        additional_outputs,
        start_time: 0.0,
        stop_time: 10.0e-6,
        compute_thd: true,
        normalize: false,
    };
    let spec = fourier(Vec::new());
    let mut encoded = CanonicalWriter::new("test");
    encode_analysis_spec(&mut encoded, &spec);

    let mut before_the_field = CanonicalWriter::new("test");
    before_the_field.domain("analysis-spec");
    before_the_field.u8(analysis_kind_tag(&spec));
    before_the_field.f64(1.0e6);
    before_the_field.usize(10);
    before_the_field.string("out");
    before_the_field.string("0");
    before_the_field.f64(0.0);
    before_the_field.f64(10.0e-6);
    before_the_field.bool(true);
    before_the_field.bool(false);

    assert_eq!(
        encoded.finish(),
        before_the_field.finish(),
        "a one-output Fourier plan must not move a saved plan's identity"
    );

    let mut one = CanonicalWriter::new("test");
    encode_analysis_spec(&mut one, &spec);
    let mut two = CanonicalWriter::new("test");
    encode_analysis_spec(&mut two, &fourier(vec!["V(mid)".to_owned()]));
    assert_ne!(one.finish(), two.finish());

    // The card writes the list in authored order, so the order is part of
    // the identity too.
    let mut forward = CanonicalWriter::new("test");
    encode_analysis_spec(
        &mut forward,
        &fourier(vec!["V(mid)".to_owned(), "I(V1)".to_owned()]),
    );
    let mut reversed = CanonicalWriter::new("test");
    encode_analysis_spec(
        &mut reversed,
        &fourier(vec!["I(V1)".to_owned(), "V(mid)".to_owned()]),
    );
    assert_ne!(forward.finish(), reversed.finish());
}

#[test]
fn every_transfer_function_field_is_bound_into_the_config_digest() {
    let base = AnalysisSpec::Tf {
        input_source: "VIN_DIFF".to_owned(),
        output_expression: "V(afe_out)".to_owned(),
        transfer_gain: true,
        input_resistance: true,
        output_resistance: true,
        normalization: TfNormalization::None,
        accuracy: TfAccuracy::Balanced,
    };
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(".tf", spec, None, &SpecExecutionOptions::default(), None)
    };
    let baseline = digest(&base);
    let mutations: [fn(&mut AnalysisSpec); 7] = [
        |spec| {
            let AnalysisSpec::Tf { input_source, .. } = spec else {
                unreachable!()
            };
            *input_source = "IIN_CAL".to_owned();
        },
        |spec| {
            let AnalysisSpec::Tf {
                output_expression, ..
            } = spec
            else {
                unreachable!()
            };
            *output_expression = "I(VDD)".to_owned();
        },
        |spec| {
            let AnalysisSpec::Tf { transfer_gain, .. } = spec else {
                unreachable!()
            };
            *transfer_gain = false;
        },
        |spec| {
            let AnalysisSpec::Tf {
                input_resistance, ..
            } = spec
            else {
                unreachable!()
            };
            *input_resistance = false;
        },
        |spec| {
            let AnalysisSpec::Tf {
                output_resistance, ..
            } = spec
            else {
                unreachable!()
            };
            *output_resistance = false;
        },
        |spec| {
            let AnalysisSpec::Tf { normalization, .. } = spec else {
                unreachable!()
            };
            *normalization = TfNormalization::RelativeToNominal;
        },
        |spec| {
            let AnalysisSpec::Tf { accuracy, .. } = spec else {
                unreachable!()
            };
            *accuracy = TfAccuracy::Robust;
        },
    ];

    for mutation in mutations {
        let mut changed = base.clone();
        mutation(&mut changed);
        assert_ne!(baseline, digest(&changed));
    }
}

#[test]
fn envelope_owned_controls_are_bound_into_the_config_digest() {
    let base = AnalysisSpec::Envelope {
        multirate: None,
        initialization: Default::default(),
        fundamental_freq: 1.0e6,
        additional_carrier_tones: vec![2.0e6],
        stop_time: 10.0e-3,
        num_harmonics: 9,
        envelope_step: Some(1.0e-6),
        modulation_sources: vec!["VIN_AM".to_owned()],
        initial_periodic_solve: EnvelopeInitialPeriodicSolve::HarmonicBalance,
        adaptive_mode: EnvelopeAdaptiveMode::Enabled,
        extraction_path: EnvelopeExtractionPath::Projection,
    };
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(".envlp", spec, None, &SpecExecutionOptions::default(), None)
    };
    let baseline = digest(&base);

    let mut changed_tones = base.clone();
    if let AnalysisSpec::Envelope {
        additional_carrier_tones,
        ..
    } = &mut changed_tones
    {
        additional_carrier_tones.push(3.0e6);
    }
    assert_ne!(baseline, digest(&changed_tones));

    let mut changed_sources = base.clone();
    if let AnalysisSpec::Envelope {
        modulation_sources, ..
    } = &mut changed_sources
    {
        modulation_sources.push("VCTRL".to_owned());
    }
    assert_ne!(baseline, digest(&changed_sources));

    let mut changed_initial = base.clone();
    if let AnalysisSpec::Envelope {
        initial_periodic_solve,
        ..
    } = &mut changed_initial
    {
        *initial_periodic_solve = EnvelopeInitialPeriodicSolve::PeriodicSteadyState;
    }
    assert_ne!(baseline, digest(&changed_initial));

    let mut changed_adaptive = base.clone();
    if let AnalysisSpec::Envelope { adaptive_mode, .. } = &mut changed_adaptive {
        *adaptive_mode = EnvelopeAdaptiveMode::EventAlignedOnly;
    }
    assert_ne!(baseline, digest(&changed_adaptive));
}

#[test]
fn manual_task_ids_are_reproducible_and_bound_to_source_kind_and_occurrence() {
    let source = content_digest("manual-expanded-source/v1", b"deck\n.op\n.end\n");
    let changed_source = content_digest(
        "manual-expanded-source/v1",
        b"deck\nR1 out 0 1k\n.op\n.end\n",
    );
    let op = AnalysisSpec::dc_op();
    let first = manual_deck_analysis_instance_id(source, &op, 0);

    assert_eq!(first, manual_deck_analysis_instance_id(source, &op, 0));
    assert_eq!(
        first,
        manual_deck_analysis_instance_id_from_tag(source, analysis_kind_tag(&op), 0,)
    );
    assert_ne!(
        first,
        manual_deck_analysis_instance_id(changed_source, &op, 0)
    );
    assert_ne!(first, manual_deck_analysis_instance_id(source, &op, 1));
    assert_ne!(
        first,
        manual_deck_analysis_instance_id(
            source,
            &AnalysisSpec::Transient {
                stop_time: 1.0,
                step_time: 0.1,
                start_time: 0.0,
                max_timestep: None,
                uic: false,
            },
            0,
        )
    );
}

#[test]
fn drc_receipt_is_independent_of_incidental_violation_order() {
    let mut first = DrcResult::new();
    first.add_violation(DrcViolation::new(
        1,
        DrcViolationType::MissingGround,
        "missing ground",
        DrcLocation::Global,
    ));
    first.add_violation(DrcViolation::new(
        2,
        DrcViolationType::UnconnectedPin,
        "pin",
        DrcLocation::Wire { id: 9 },
    ));
    first.completed = true;
    let mut second = DrcResult::new();
    for violation in first.violations().iter().rev() {
        second.add_violation(violation.clone());
    }
    second.completed = true;
    assert_eq!(
        drc_receipt_digest(7, &first),
        drc_receipt_digest(7, &second)
    );
}

#[test]
fn drc_receipt_retains_full_durable_bus_identity() {
    let digest_for = |location| {
        let mut result = DrcResult::new();
        result.add_violation(DrcViolation::new(
            1,
            DrcViolationType::MalformedBus,
            "malformed bus",
            location,
        ));
        result.completed = true;
        drc_receipt_digest(9, &result)
    };

    assert_ne!(
        digest_for(DrcLocation::Bus { id: 1 }),
        digest_for(DrcLocation::Bus {
            id: 1 + (1_u64 << 32),
        })
    );
    assert_ne!(
        digest_for(DrcLocation::BusTap { id: 1 }),
        digest_for(DrcLocation::BusTap {
            id: 1 + (1_u64 << 32),
        })
    );
    assert_ne!(
        digest_for(DrcLocation::Component {
            id: 1,
            name: "R1".to_owned(),
        }),
        digest_for(DrcLocation::Component {
            id: 1 + (1_u64 << 32),
            name: "R1".to_owned(),
        })
    );
    assert_ne!(
        digest_for(DrcLocation::Wire { id: 1 }),
        digest_for(DrcLocation::Wire {
            id: 1 + (1_u64 << 32),
        })
    );
}
#[test]
fn periodic_port_noise_options_have_distinct_execution_identities() {
    use rspice_core::analysis::s_param::PeriodicPortNoiseReference;
    let request = |hb: bool, reference| {
        if hb {
            AnalysisSpec::Hbsp {
                start_freq: 1e4,
                stop_freq: 1e4,
                points_per_unit: 1,
                sweep: crate::simulation::multi_run::FrequencySweep::Linear,
                ports: Vec::new(),
                max_sideband: 1,
                reltol: 1.0e-3,
                abstol: 1.0e-12,
                mixed_mode: false,
                noise_parameters: true,
                noise_reference: reference,
            }
        } else {
            AnalysisSpec::Psp {
                start_freq: 1e4,
                stop_freq: 1e4,
                points_per_unit: 1,
                sweep: crate::simulation::multi_run::FrequencySweep::Linear,
                ports: Vec::new(),
                max_sideband: 1,
                reltol: 1.0e-3,
                abstol: 1.0e-12,
                mixed_mode: false,
                noise_parameters: true,
                noise_reference: reference,
            }
        }
    };
    let baseline = PeriodicPortNoiseReference::default();
    let digest = |spec: &AnalysisSpec| {
        analysis_config_digest(
            "* periodic network",
            spec,
            None,
            &SpecExecutionOptions::default(),
            None,
        )
    };
    for hb in [false, true] {
        let spec = request(hb, Some(baseline.clone()));
        assert!(spec.validate().is_ok());
        let expected = digest(&spec);
        let restored: AnalysisSpec =
            serde_json::from_str(&serde_json::to_string(&spec).unwrap()).unwrap();
        assert_eq!(expected, digest(&restored));
        assert_ne!(expected, digest(&request(hb, None)));
        let changes: [fn(&mut PeriodicPortNoiseReference); 7] = [
            |r| r.input_port = 2,
            |r| r.output_port = 1,
            |r| r.input_sideband = -1,
            |r| r.output_sideband = 1,
            |r| r.reference_temperature_kelvin = 325.0,
            |r| r.termination_temperature_kelvin = 0.0,
            |r| r.image_sideband = Some(-1),
        ];
        for change in changes {
            let mut reference = baseline.clone();
            change(&mut reference);
            assert_ne!(expected, digest(&request(hb, Some(reference))));
        }
        let mut invalid = baseline.clone();
        invalid.input_sideband = 2;
        assert!(request(hb, Some(invalid)).validate().is_err());
        let mut disabled = spec;
        match &mut disabled {
            AnalysisSpec::Psp {
                noise_parameters, ..
            }
            | AnalysisSpec::Hbsp {
                noise_parameters, ..
            } => *noise_parameters = false,
            _ => unreachable!(),
        }
        assert!(disabled.validate().is_err());
    }
}

use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
use rspice_core::netlist::{Netlist, NetlistParseOptions};
use rspice_core::numerics::integration::IntegrationMethod;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const F0: f64 = 1.0e6;

#[test]
fn autonomous_startup_kick_remains_quiet_on_the_orbit_and_reactivates_after_saved_continuation() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    let netlist = Netlist::parse("oscillator startup continuation\nL1 osc 0 1u\nC1 osc 0 1u\nB1 osc 0 I=-0.05*v(osc)+0.025*v(osc)*v(osc)*v(osc)\nI1 0 kick PULSE(0 1 10u 10n 10n 1u 1)\nRkick kick osc 1\n.end\n").unwrap();
    let engine = Engine::default();
    let (analysis, state) = engine
        .run_pss_with_continuation_state(
            &netlist,
            PssConfig::autonomous()
                .with_period_guess(6.3e-6)
                .with_tstab_periods(30)
                .with_tolerance(1e-6)
                .with_max_iterations(60),
        )
        .unwrap();
    let node = |names: &[String], name: &str| {
        names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap()
    };
    let osc = node(&analysis.result.node_names, "osc");
    let kick = node(&analysis.result.node_names, "kick");
    for (&vosc, &vkick) in analysis.result.waveforms[osc]
        .values
        .iter()
        .zip(&analysis.result.waveforms[kick].values)
    {
        assert!(
            (vkick - vosc).abs() < 1e-10,
            "startup must not drive the periodic orbit"
        );
    }
    let (_, checkpoint) = engine
        .run_tran_from_pss_state(&netlist, &state, 8e-6, 1e-8)
        .unwrap();
    let (direct, _) = engine
        .run_tran_resume(&netlist, &checkpoint, 12e-6, 1e-8)
        .unwrap();
    for encoding in [
        TransientCheckpointEncoding::Unpacked,
        TransientCheckpointEncoding::Packed,
    ] {
        let restored =
            TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap()).unwrap();
        let (resumed, _) = engine
            .run_tran_resume(&netlist, &restored, 12e-6, 1e-8)
            .unwrap();
        assert_eq!(resumed.time, direct.time);
        assert_eq!(resumed.voltages, direct.voltages);
        let osc = node(&resumed.node_names, "osc");
        let kick = node(&resumed.node_names, "kick");
        let mut plateau = 0;
        for ((&time, &vosc), &vkick) in resumed
            .time
            .iter()
            .zip(&resumed.voltages[osc])
            .zip(&resumed.voltages[kick])
        {
            if (10.1e-6..10.9e-6).contains(&time) {
                plateau += 1;
                assert!(
                    (vkick - vosc - 1.0).abs() < 1e-9,
                    "{encoding:?}, t={time:e}: startup source must reactivate at its authored time"
                );
            }
        }
        assert!(plateau > 10);
    }
}

#[test]
fn source_defaults_survive_pss_continuation_and_persisted_transient_segments() {
    use rspice_core::engine::{TransientCheckpoint, TransientCheckpointEncoding};
    for (dialect, nox) in [
        (SpiceDialect::Ngspice, false),
        (SpiceDialect::Xyce, false),
        (SpiceDialect::Xyce, true),
    ] {
        let source = if dialect == SpiceDialect::Xyce {
            "SFFM(0 1)"
        } else {
            "SIN(0 1 0)"
        };
        let netlist = Netlist::parse(&format!(
            "PSS source default continuation\nV1 in 0 {source}\nC1 in 0 1p\n.tran 5n 7u\n.end\n",
        ))
        .unwrap();
        for method in [
            IntegrationMethod::BackwardEuler,
            IntegrationMethod::Trapezoidal,
            IntegrationMethod::Gear2,
            IntegrationMethod::TrapGear,
        ] {
            let engine = Engine::new(SimulationConfig {
                spice_dialect: dialect,
                transient_nonlinear_nox: Some(nox),
                integration_method: method,
                ..Default::default()
            });
            let (_, state) = engine
                .run_pss_with_continuation_state(
                    &netlist,
                    PssConfig::new(F0)
                        .with_points_per_period(128)
                        .with_tstab_periods(0),
                )
                .unwrap();
            let (continued, checkpoint) = engine
                .run_tran_from_pss_state(&netlist, &state, 1.31e-6, 3e-9)
                .unwrap();
            for encoding in [
                TransientCheckpointEncoding::Unpacked,
                TransientCheckpointEncoding::Packed,
            ] {
                let restored =
                    TransientCheckpoint::from_bytes(&checkpoint.to_bytes(encoding).unwrap())
                        .unwrap();
                let (resumed, _) = engine
                    .run_tran_resume(&netlist, &restored, 2.57e-6, 2e-9)
                    .unwrap();
                let (direct, _) = engine
                    .run_tran_resume(&netlist, &checkpoint, 2.57e-6, 2e-9)
                    .unwrap();
                assert_eq!(resumed.time, direct.time);
                assert_eq!(resumed.voltages, direct.voltages);
                for result in [&continued, &resumed] {
                    let node = result
                        .node_names
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case("in"))
                        .unwrap();
                    for (&time, &actual) in result.time.iter().zip(&result.voltages[node]) {
                        let expected = (std::f64::consts::TAU * F0 * time).sin();
                        assert!(
                            (actual - expected).abs() < 1e-11,
                            "{dialect:?}, {method:?}, {encoding:?}, t={time:e}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }
}

fn envelope_startup_deck() -> Netlist {
    Netlist::parse(
        "* carrier plus a slower modulation source\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 PULSE(0 1 250n 20n 20n 2u 10u)\n\
         Rcarrier carrier out 1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n",
    )
    .expect("envelope startup deck parses")
}

fn compact_pss_config() -> PssConfig {
    PssConfig::new(F0)
        .with_harmonics(4)
        .with_points_per_period(32)
        // Keep these continuation-contract tests on the deterministic
        // fixed-grid shooting path. Adaptive stabilization has its own guard
        // regression in the PSS unit tests.
        .with_tstab_periods(0)
        .with_tolerance(1.0e-6)
}

struct TemporaryFile(PathBuf);

impl TemporaryFile {
    fn new(label: &str, contents: &str) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice-pss-continuation-{label}-{}-{id}.csv",
            std::process::id()
        ));
        std::fs::write(&path, contents).expect("temporary dependency is writable");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn replace(&self, contents: &str) {
        std::fs::write(&self.0, contents).expect("temporary dependency can be replaced");
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

#[test]
fn frozen_modulation_source_is_authenticated_and_reactivated_at_time_zero() {
    let netlist = envelope_startup_deck();
    let engine = Engine::new(SimulationConfig::default());
    let (pss, state) = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["vMoD".to_string()],
        )
        .expect("frozen-source PSS produces a continuation state");

    assert_eq!(state.time_origin(), 0.0);
    assert!((state.period() - 1.0 / F0).abs() <= 4.0 * f64::EPSILON / F0);
    assert_eq!(pss.result.time.last().copied(), Some(state.period()));
    assert_eq!(state.frozen_sources(), &["VMOD".to_string()]);
    let pss_mod_index = pss
        .result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("mod"))
        .expect("mod node is present in PSS");
    assert!(
        pss.result.waveforms[pss_mod_index]
            .values
            .iter()
            .all(|value| value.abs() < 1.0e-12),
        "the selected modulation source must remain frozen throughout PSS"
    );

    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 600.0e-9, 10.0e-9)
        .expect("original modulation waveform reactivates from the authenticated state");
    assert_eq!(transient.time.first().copied(), Some(0.0));
    let mod_index = transient
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("mod"))
        .expect("mod node is present");
    let mod_waveform = &transient.voltages[mod_index];
    assert!(mod_waveform.first().copied().unwrap_or_default().abs() < 1.0e-12);
    assert!(
        mod_waveform.iter().copied().fold(0.0_f64, f64::max) > 0.99,
        "the original PULSE source must be active after the PSS-to-transient seam"
    );
}

#[test]
fn linear_rl_continuation_retains_the_exact_supported_inductor_path() {
    let netlist = Netlist::parse(
        "* stable driven RL circuit\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in out 10\n\
         L1 out 0 1u\n\
         .end\n",
    )
    .expect("linear RL deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (pss, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .expect("ordinary R/L and independent sources have an exact continuation contract");

    assert_eq!(pss.result.time.last().copied(), Some(state.period()));
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect("the retained inductor history resumes in transient");
    assert_eq!(transient.time.first().copied(), Some(0.0));
    assert_eq!(transient.time.last().copied(), Some(100.0e-9));
}

#[test]
fn continuation_artifact_rejects_different_netlist_or_engine_configuration() {
    let netlist = envelope_startup_deck();
    let engine = Engine::new(SimulationConfig::default());
    let (_, state) = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmod".to_string()],
        )
        .expect("continuation state");

    let changed_deck = Netlist::parse(
        "* changed carrier resistance\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 PULSE(0 1 250n 20n 20n 2u 10u)\n\
         Rcarrier carrier out 1.1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n",
    )
    .expect("changed deck parses");
    let deck_error = engine
        .run_tran_from_pss_state(&changed_deck, &state, 100.0e-9, 10.0e-9)
        .expect_err("artifact must not cross semantic netlist identity");
    assert!(
        deck_error.to_string().contains("different netlist"),
        "unexpected identity error: {deck_error}"
    );

    let mut changed_config = SimulationConfig::default();
    changed_config.temperature += 10.0;
    let changed_engine = Engine::new(changed_config);
    let config_error = changed_engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect_err("artifact must not cross resolved simulation configuration identity");
    assert!(
        config_error
            .to_string()
            .contains("different resolved simulation configuration"),
        "unexpected configuration identity error: {config_error}"
    );
}

#[test]
fn continuation_artifact_authenticates_external_waveform_bytes() {
    let waveform = TemporaryFile::new("pwl", "0,0\n0.000001,1\n");
    let path = waveform.path().to_string_lossy().replace('\\', "/");
    let netlist = Netlist::parse(&format!(
        "* external modulation dependency\n\
         Vcarrier carrier 0 SIN(0 1 1meg)\n\
         Vmod mod 0 PWL FILE=\"{path}\"\n\
         Rcarrier carrier out 1k\n\
         Rmod mod out 2k\n\
         Cout out 0 160p\n\
         .end\n"
    ))
    .expect("PWL FILE deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (_, state) = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmod".to_string()],
        )
        .expect("the original external waveform snapshot is authenticated");

    waveform.replace("0,0\n0.000001,2\n");
    let error = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect_err("changed external waveform bytes must invalidate the artifact");
    assert!(
        error.to_string().contains("different netlist"),
        "unexpected external dependency identity error: {error}"
    );
}

#[test]
fn memoryless_diode_supports_periodic_transient_continuation() {
    let netlist = Netlist::parse(
        "memoryless diode\nV1 in 0 SIN(-1 0.01 1meg)\nR1 in out 1k\n\
         C1 out 0 100p\nD1 out 0 DMOD\n.model DMOD D(CJO=0 TT=0)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let (_, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .unwrap();
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 1e-6, 1e-8)
        .unwrap();
    assert!(
        transient
            .voltages
            .iter()
            .flatten()
            .all(|value| value.is_finite())
    );
}

#[test]
fn charged_diode_continuation_preserves_the_periodic_orbit_from_time_zero() {
    let netlist = Netlist::parse(
        "charged diode continuation\nV1 in 0 SIN(-1 0.01 1meg)\nR1 in out 1k\n\
         D1 out 0 DMOD\n.model DMOD D(IS=1e-30 CJO=1n M=0 TT=0)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let (_, state) = engine
        .run_pss_with_continuation_state(
            &netlist,
            compact_pss_config()
                .with_points_per_period(512)
                .with_tolerance(1e-10),
        )
        .expect("PSS captures the charged diode's accepted state");
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 2e-6, 1e-6 / 1024.0)
        .expect("charged periodic history resumes through the ordinary transient engine");
    let ratio = std::f64::consts::TAU;
    let amplitude = 0.01 / (1.0 + ratio * ratio).sqrt();
    for (&time, &voltage) in transient
        .time
        .iter()
        .zip(transient.try_voltage_waveform_named("OUT").unwrap())
    {
        let expected = -1.0 + amplitude * (std::f64::consts::TAU * F0 * time - ratio.atan()).sin();
        assert!(
            (voltage - expected).abs() < 0.002 * amplitude,
            "charge continuation drift at {time:e}: {voltage:e} versus {expected:e}"
        );
    }
}

#[test]
fn continuation_fails_closed_for_unadvanced_dynamic_state_families() {
    let engine = Engine::new(SimulationConfig::default());

    let coupled = Netlist::parse(
        "* perfect coupling needs independent flux coordinates\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in p 10\n\
         L1 p 0 1u\n\
         L2 out 0 2u\n\
         K1 L1 L2 1\n\
         R2 out 0 100\n\
         .end\n",
    )
    .expect("coupled-inductor deck parses");
    let coupled_error = engine
        .run_pss_with_continuation_state(&coupled, compact_pss_config())
        .expect_err("singular magnetic flux must fail before the periodic solve");
    assert!(
        coupled_error
            .to_string()
            .contains("coupled-inductor flux constraints"),
        "unexpected coupled-inductor diagnostic: {coupled_error}"
    );

    let behavioral = Netlist::parse(
        "* behavioral accepted-step expression memory is not in shooting x\n\
         V1 in 0 SIN(0 1 1meg)\n\
         B1 out 0 V={SDT(V(in))}\n\
         R1 out 0 1k\n\
         C1 out 0 100p\n\
         .end\n",
    )
    .expect("behavioral deck parses");
    let behavioral_error = engine
        .run_pss_with_continuation_state(&behavioral, compact_pss_config())
        .expect_err("behavioral accepted-step memory must fail before solving");
    assert!(
        behavioral_error
            .to_string()
            .contains("behavioral-source accepted-step memory"),
        "unexpected behavioral-state diagnostic: {behavioral_error}"
    );

    let solution_dependent_capacitor = Netlist::parse_with_options(
        "* expression-valued capacitor charge is outside the shooting state\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in out 1k\n\
         C1 out 0 C={100p*(1+0.1*V(out))}\n\
         .end\n",
        NetlistParseOptions {
            expression_dialect: rspice_core::config::ExpressionDialect::Xyce,
            ..Default::default()
        },
    )
    .expect("solution-dependent capacitor deck parses");
    let solution_dependent_error =
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_pss_with_continuation_state(&solution_dependent_capacitor, compact_pss_config())
            .expect_err("solution-dependent capacitor history must fail before solving");
    assert!(
        solution_dependent_error
            .to_string()
            .contains("solution-dependent capacitor charge/expression history"),
        "unexpected solution-dependent capacitor diagnostic: {solution_dependent_error}"
    );

    let thermal_resistor = Netlist::parse(
        "* electrothermal accepted temperature is outside the shooting state\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in out RMOD L=1u A=1u\n\
         C1 out 0 100p\n\
         .MODEL RMOD R (LEVEL=2 RESISTIVITY=1 HEATCAPACITY=1)\n\
         .end\n",
    )
    .expect("thermal resistor deck parses");
    let thermal_error =
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .run_pss_with_continuation_state(&thermal_resistor, compact_pss_config())
            .expect_err("thermal accepted temperature must fail before solving");
    assert!(
        thermal_error
            .to_string()
            .contains("thermal resistor accepted temperature state"),
        "unexpected thermal resistor diagnostic: {thermal_error}"
    );
}

#[test]
fn stateless_behavioral_source_has_an_exact_pss_continuation_path() {
    let netlist = Netlist::parse(
        "* time-only behavioral source has no accepted expression memory\n\
         B1 drive 0 V=sin(2*pi*1meg*time)\n\
         R1 drive out 1k\n\
         C1 out 0 100p\n\
         .end\n",
    )
    .expect("stateless behavioral deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let (pss, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .expect("stateless behavioral source produces an exact continuation state");

    assert_eq!(pss.result.time.last().copied(), Some(state.period()));
    let (transient, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 10.0e-9)
        .expect("stateless behavioral source resumes from the PSS state");
    assert_eq!(transient.time.first().copied(), Some(0.0));
    assert_eq!(transient.time.last().copied(), Some(100.0e-9));
}

#[test]
fn pss_continuation_checkpoint_has_bit_exact_split_run_parity() {
    let netlist = Netlist::parse(
        "* deterministic PSS-to-TRAN split-run fixture\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         .end\n",
    )
    .expect("split-run deck parses");
    let simulation = SimulationConfig {
        integration_method: IntegrationMethod::BackwardEuler,
        transient_initial_timestep: Some(100.0e-9),
        ..SimulationConfig::default()
    };
    let engine = Engine::new(simulation);
    let (_, state) = engine
        .run_pss_with_continuation_state(&netlist, compact_pss_config())
        .expect("PSS continuation state");

    let (uninterrupted, _) = engine
        .run_tran_from_pss_state(&netlist, &state, 200.0e-9, 100.0e-9)
        .expect("uninterrupted PSS continuation");
    let (_, seam_checkpoint) = engine
        .run_tran_from_pss_state(&netlist, &state, 100.0e-9, 100.0e-9)
        .expect("first split segment");
    let (resumed, _) = engine
        .run_tran_resume(&netlist, &seam_checkpoint, 200.0e-9, 100.0e-9)
        .expect("second split segment resumes");

    let seam = uninterrupted
        .time
        .iter()
        .position(|time| time.to_bits() == seam_checkpoint.time.to_bits())
        .expect("uninterrupted trajectory contains the split seam");
    assert_eq!(resumed.time, uninterrupted.time[seam..]);
    assert_eq!(resumed.step_sizes.first().copied(), Some(0.0));
    assert_eq!(
        resumed.step_sizes[1..],
        uninterrupted.step_sizes[seam + 1..]
    );
    assert_eq!(resumed.node_names, uninterrupted.node_names);
    for (resumed_waveform, uninterrupted_waveform) in
        resumed.voltages.iter().zip(&uninterrupted.voltages)
    {
        assert_eq!(resumed_waveform, &uninterrupted_waveform[seam..]);
    }
}

#[test]
fn frozen_source_contract_rejects_ambiguous_or_unknown_names() {
    let netlist = envelope_startup_deck();
    let engine = Engine::new(SimulationConfig::default());

    let duplicate = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmod".to_string(), "vMOD".to_string()],
        )
        .expect_err("case-insensitive duplicates must fail before solving");
    assert!(duplicate.to_string().contains("duplicate source 'vmod'"));

    let unknown = engine
        .run_pss_with_frozen_source_continuation_state(
            &netlist,
            compact_pss_config(),
            &["Vmissing".to_string()],
        )
        .expect_err("unknown sources must fail closed");
    assert!(
        unknown
            .to_string()
            .contains("unknown independent source 'vmissing'")
    );
}

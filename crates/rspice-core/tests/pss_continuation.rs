use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

const F0: f64 = 1.0e6;

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
fn continuation_fails_closed_for_unadvanced_dynamic_state_families() {
    let engine = Engine::new(SimulationConfig::default());

    let diode = Netlist::parse(
        "* diode charge history is outside the shooting state\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         D1 out 0 DMOD\n\
         .model DMOD D(CJO=1p)\n\
         .end\n",
    )
    .expect("diode deck parses");
    let diode_error = engine
        .run_pss_with_continuation_state(&diode, compact_pss_config())
        .expect_err("diode charge state must fail before the periodic solve");
    assert!(
        diode_error
            .to_string()
            .contains("diode junction/diffusion charge history"),
        "unexpected diode-state diagnostic: {diode_error}"
    );

    let coupled = Netlist::parse(
        "* coupled inductor mutual history is not an ordinary L state\n\
         V1 in 0 SIN(0 1 1meg)\n\
         R1 in p 10\n\
         L1 p 0 1u\n\
         L2 out 0 2u\n\
         K1 L1 L2 0.5\n\
         R2 out 0 100\n\
         .end\n",
    )
    .expect("coupled-inductor deck parses");
    let coupled_error = engine
        .run_pss_with_continuation_state(&coupled, compact_pss_config())
        .expect_err("mutual magnetic history must fail before the periodic solve");
    assert!(
        coupled_error
            .to_string()
            .contains("coupled-inductor mutual history"),
        "unexpected coupled-inductor diagnostic: {coupled_error}"
    );

    let behavioral = Netlist::parse(
        "* behavioral accepted-step expression memory is not in shooting x\n\
         B1 out 0 V=sin(2*pi*1meg*time)\n\
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

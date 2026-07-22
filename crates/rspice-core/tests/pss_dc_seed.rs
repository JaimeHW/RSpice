use rspice_core::abort_signal::ImmediateAbort;
use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, PssDcOperatingPointSeed, SimulationConfig, SimulationError};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;
const R: f64 = 1.0e3;
const C: f64 = 159.154_943_091_895e-12;

fn rc_netlist() -> Netlist {
    Netlist::parse(&format!(
        "* seeded shooting PSS\n\
         V1 in 0 SIN(0 1 {F0})\n\
         R1 in out {R}\n\
         C1 out 0 {C}\n\
         .end\n"
    ))
    .expect("RC deck parses")
}

fn compact_config() -> PssConfig {
    PssConfig::new(F0)
        .with_tstab(0.0)
        .with_tstab_periods(0)
        .with_points_per_period(64)
        .with_tolerance(1.0e-2)
}

fn dc_seed(engine: &Engine, netlist: &Netlist) -> PssDcOperatingPointSeed {
    let dc = engine
        .run_dc_op(netlist)
        .expect("DC operating point solves");
    let solution = dc
        .node_voltages
        .iter()
        .skip(1)
        .chain(&dc.branch_currents)
        .copied()
        .collect();
    PssDcOperatingPointSeed::try_new(
        dc.node_names.into_iter().skip(1).collect(),
        dc.branch_names,
        solution,
    )
    .expect("DC result forms an exact seed")
}

#[test]
fn seeded_pss_consumes_exact_dc_state() {
    let netlist = rc_netlist();
    let engine = Engine::new(SimulationConfig::default());
    // Use a deliberately distinct, finite capacitor state and a permissive
    // shooting tolerance so the first accepted orbit exposes the exact
    // initialization instead of Newton-correcting it first. The branch
    // current is present because the contract retains the complete MNA state.
    let periodic_seed = PssDcOperatingPointSeed::try_new(
        vec!["IN".to_owned(), "OUT".to_owned()],
        vec!["V1".to_owned()],
        vec![0.0, 2.0, 2.0 / R],
    )
    .expect("seed is structurally valid");
    let probe_config = compact_config().with_tolerance(10.0);

    let seeded = engine
        .run_pss_with_dc_seed(&netlist, probe_config.clone(), &periodic_seed)
        .expect("seeded PSS converges");
    let automatic = engine
        .run_pss(&netlist, probe_config)
        .expect("ordinary PSS converges");

    let output_index = seeded
        .result
        .node_names
        .iter()
        .position(|name| name == "OUT")
        .expect("OUT waveform is present");
    let seeded_initial = seeded.result.waveforms[output_index].values[0];
    let automatic_initial = automatic.result.waveforms[output_index].values[0];
    assert!(
        (seeded_initial - automatic_initial).abs() > 1.0,
        "the exact seed must control the phase-zero state: seeded={seeded_initial:.6e}, automatic={automatic_initial:.6e}"
    );
}

#[test]
fn seeded_pss_rejects_stale_or_tampered_basis() {
    let netlist = rc_netlist();
    let engine = Engine::new(SimulationConfig::default());
    let wrong_order = PssDcOperatingPointSeed::try_new(
        vec!["out".to_owned(), "in".to_owned()],
        vec!["V1".to_owned()],
        vec![-0.5, 0.0, -0.5 / R],
    )
    .expect("payload is internally shaped but belongs to a different basis");
    let err = engine
        .run_pss_with_dc_seed(&netlist, compact_config(), &wrong_order)
        .expect_err("reordered node basis must be rejected");
    assert!(
        matches!(err, SimulationError::Circuit(message) if message.contains("node basis does not match"))
    );

    let wrong_branch = PssDcOperatingPointSeed::try_new(
        vec!["IN".to_owned(), "OUT".to_owned()],
        vec!["V_STALE".to_owned()],
        vec![0.0, -0.5, -0.5 / R],
    )
    .expect("payload is internally shaped but belongs to a different branch basis");
    let err = engine
        .run_pss_with_dc_seed(&netlist, compact_config(), &wrong_branch)
        .expect_err("stale branch basis must be rejected");
    assert!(
        matches!(err, SimulationError::Circuit(message) if message.contains("branch basis does not match"))
    );

    let err = PssDcOperatingPointSeed::try_new(
        vec!["in".to_owned(), "out".to_owned()],
        vec!["V1".to_owned()],
        vec![0.0, f64::NAN, 0.0],
    )
    .expect_err("non-finite numerical evidence must be rejected");
    assert!(matches!(err, SimulationError::Circuit(message) if message.contains("non-finite")));

    let err = PssDcOperatingPointSeed::try_new(
        vec!["in".to_owned(), "out".to_owned()],
        vec!["V1".to_owned()],
        vec![0.0, -0.5],
    )
    .expect_err("truncated MNA evidence must be rejected");
    assert!(matches!(err, SimulationError::Circuit(message) if message.contains("require 3")));
}

#[test]
fn unseeded_pss_behavior_is_unchanged() {
    let netlist = rc_netlist();
    let engine = Engine::new(SimulationConfig::default());
    let seed = dc_seed(&engine, &netlist);

    let automatic = engine
        .run_pss(&netlist, compact_config())
        .expect("ordinary PSS converges");
    let explicit_same_dc = engine
        .run_pss_with_dc_seed(&netlist, compact_config(), &seed)
        .expect("equivalent seeded PSS converges");

    assert_eq!(automatic, explicit_same_dc);
}

#[test]
fn seeded_pss_honors_cancellation() {
    let netlist = rc_netlist();
    let engine = Engine::new(SimulationConfig::default());
    let seed = dc_seed(&engine, &netlist);

    let err = engine
        .run_pss_with_dc_seed_and_abort(&netlist, compact_config(), &seed, &ImmediateAbort)
        .expect_err("cancelled seeded PSS must stop");
    assert!(matches!(err, SimulationError::Aborted));
}

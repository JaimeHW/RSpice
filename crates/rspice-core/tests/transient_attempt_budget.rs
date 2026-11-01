use rspice_core::{Engine, Netlist, SimulationConfig, SpiceDialect};
use std::sync::Arc;

#[test]
fn valid_locked_run_can_exceed_the_removed_global_attempt_heuristic() {
    // The former DELMAX-derived ceiling stopped at 50,000 outer attempts even
    // though every local Newton step converged and the explicit result budgets
    // still had ample room. One extra prescribed step is the smallest exact
    // counterexample to that accidental global policy.
    const STEP_ATTEMPTS: usize = 50_001;
    let grid = (0..=STEP_ATTEMPTS)
        .map(|index| index as f64 / STEP_ATTEMPTS as f64)
        .collect::<Vec<_>>();
    let netlist = Netlist::parse(
        "bounded transient attempts\n\
         I1 out 0 0\n\
         R1 out 0 1k\n\
         .TRAN 1 1\n\
         .END\n",
    )
    .expect("linear transient deck parses");
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        locked_time_grid: Some(Arc::new(grid.clone())),
        ..SimulationConfig::default()
    });

    let result = engine
        .run_tran(&netlist, 1.0, 1.0)
        .expect("explicit resource budgets, not a derived attempt guess, govern the run");

    assert_eq!(result.time, grid);
    assert_eq!(result.time.last().copied(), Some(1.0));
}

#[test]
fn xyce_classic_mos_transient_converges_on_limiter_activity_alone() {
    // Xyce's MOSFET levels 1-3 report `isConverged = !limitedFlag`
    // (N_DEV_MOSFET1.h:771-774), where `limitedFlag` is pnjlim's `Check` on
    // the bulk junctions (N_DEV_MOSFET3.C:1756-1769). The SPICE current
    // prediction test is not part of Xyce's transient Newton, so applying it
    // in the Xyce dialect rejected steps Xyce accepts. On the first 2 us of
    // ISSUE_239's mos3_test2Ref inverter the strict test rejected 4237 of
    // 17017 attempts and the native status 1453 of 5877; Xyce 7.10 takes 94
    // steps and rejects 8. An explicit ENFORCEDEVICECONV keeps the strict test.
    let netlist = Netlist::parse(
        "mos3 inverter\n\
         vdddev vdd 0 5v\n\
         rin in 1 1k\n\
         vin1 1 0 5v pulse (5v 0v 1.5us 5ns 5ns 1.5us 3us)\n\
         r1 vout 0 10k\n\
         c2 vout 0 0.1p\n\
         mn1 vout in 0 0 nch l=5u w=175u\n\
         mp1 vout in vdd vdd pch l=5u w=270u\n\
         .model pch pmos (level=3 uo=310 vto=-1.6 tox=6e-08 nsub=5.701e+15\n\
         + nss=1e-10 rs=5.359 rd=93.66 rsh=2e-10 is=1e-14 ld=3e-08 kp=1.711e-05\n\
         + lambda=0.02 gamma=0.37 phi=0.65 cbd=0.1p cbs=0.1p pb=0.81 cgso=2p\n\
         + cgbo=4p cgdo=2p cj=2e-4 mj=0.5 cjsw=1e-9 mjsw=0.5 js=1e-8 tpg=0\n\
         + fc=0.5 tnom=27)\n\
         .model nch nmos (level=3 uo=190 vto=1.679 tox=6e-08 nsub=8.601e+15\n\
         + nss=0 rs=13.21 rd=11.59 rsh=0 is=1e-14 ld=8.6e-07 kp=2.161e-05\n\
         + lambda=0.02 gamma=0.37 phi=0.65 cbd=0.1p cbs=0.1p pb=0.81 cgso=2p\n\
         + cgbo=4p cgdo=2p cj=2e-4 mj=0.5 cjsw=1e-9 mjsw=0.5 js=1e-8 tpg=0\n\
         + fc=0.5 tnom=27)\n\
         .tran 20ns 2us\n\
         .end\n",
    )
    .expect("MOS3 inverter deck parses");
    let rejected = |enforce_device_convergence: Option<bool>| {
        let engine = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            transient_enforce_device_convergence: enforce_device_convergence,
            ..SimulationConfig::default()
        });
        let result = engine
            .run_tran(&netlist, 2.0e-6, 0.2e-6)
            .expect("the inverter transient completes");
        assert_eq!(result.time.last().copied(), Some(2.0e-6));
        let quality = engine.convergence_quality();
        assert_eq!(quality.force_accepted_points, 0);
        quality.timestep_reductions
    };

    let native = rejected(None);
    let strict = rejected(Some(true));
    assert!(
        2 * native < strict,
        "native Xyce MOS convergence rejected {native} steps, the strict test {strict}"
    );
}

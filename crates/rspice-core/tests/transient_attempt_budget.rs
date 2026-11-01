use rspice_core::numerics::integration::IntegrationMethod;
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
fn xyce_mos3_inverter_converges_through_vds_zero() {
    // Below VDSAT, Xyce's MOSFET3 applies no channel-length modulation
    // (N_DEV_MOSFET3.C:2201). The Xyce dialect ran ngspice's linear-region
    // CLM continuation through Xyce's derivative assembly, which left the
    // Jacobian with a fraction of the linear-region drain conductance. With
    // the inverter's NMOS on and vds ~ 0, Newton diverged in a growing
    // two-cycle: on the first 2 us of ISSUE_239's mos3_test2Ref inverter it
    // rejected 1396 of 5656 attempts, and 4069 of 16 346 under an explicit
    // ENFORCEDEVICECONV. Xyce 7.10 takes 102 attempts and rejects 8.
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
    let engine = Engine::new(SimulationConfig {
        spice_dialect: SpiceDialect::Xyce,
        ..SimulationConfig::default()
    });
    let result = engine
        .run_tran(&netlist, 2.0e-6, 0.2e-6)
        .expect("the inverter transient completes");
    assert_eq!(result.time.last().copied(), Some(2.0e-6));
    let quality = engine.convergence_quality();
    assert_eq!(quality.force_accepted_points, 0);
    // Every rejection left is a truncation-error one, as Xyce's are.
    assert_eq!(
        quality.timestep_reductions, quality.lte_rejections,
        "the vds ~ 0 Newton solves must converge"
    );
    assert!(
        quality.lte_rejections <= 12,
        "{} truncation-error rejections; Xyce 7.10 rejects 8",
        quality.lte_rejections
    );
    assert!(
        result.time.len() <= 120,
        "{} accepted points; Xyce 7.10 accepts 94",
        result.time.len()
    );
}

#[test]
fn xyce_dialect_integrates_with_one_step_trapezoidal_by_default() {
    // Xyce's default transient integrator is OneStep (N_ANP_Transient.C:147),
    // a trapezoidal rule with order selection that never switches to Gear on
    // its own. The adaptive TrapGear default used to swap in Gear2 when its
    // ringing detector fired, which Xyce never does: on Xyce_Regression's
    // BUG_372 M=2 MOS2 inverter the swap at the first input edge (1.505 us)
    // moved RSpice off Xyce's step sequence, and the M=2 run then failed
    // xyce_verify against the explicit-parallel one (v(in) 7.4% integrated
    // error at a 2% tolerance). Run as Xyce does, both match Xyce's 4224 and
    // 4350 steps exactly.
    let netlist = Netlist::parse(
        "mos2 inverter, M=2 devices\n\
         .options timeint reltol=1e-4\n\
         vdddev vdd 0 5v\n\
         rin in 1 1k\n\
         vin1 1 0 5v pulse (5v 0v 1.5us 5ns 5ns 1.5us 3us)\n\
         r1 vout 0 10k\n\
         c2 vout 0 0.1p\n\
         mn1 vout in 0 0 nch l=5u w=175u m=2\n\
         mp1 vout in vdd vdd pch l=5u w=270u m=2\n\
         .model pch pmos (level=2 uo=310 vto=-1.6 tox=6e-08 nsub=5.701e+15\n\
         + nss=1e-10 rs=5.359 rd=93.66 rsh=2e-10 is=1e-14 ld=3e-08 kp=1.711e-05\n\
         + lambda=0.02 gamma=0.37 phi=0.65 cbd=0.1p cbs=0.1p pb=0.81 cgso=2p\n\
         + cgbo=4p cgdo=2p cj=2e-4 mj=0.5 cjsw=1e-9 mjsw=0.5 js=1e-8 tpg=0\n\
         + fc=0.5 tnom=27)\n\
         .model nch nmos (level=2 uo=190 vto=1.679 tox=6e-08 nsub=8.601e+15\n\
         + nss=0 rs=13.21 rd=11.59 rsh=0 is=1e-14 ld=8.6e-07 kp=2.161e-05\n\
         + lambda=0.02 gamma=0.37 phi=0.65 cbd=0.1p cbs=0.1p pb=0.81 cgso=2p\n\
         + cgbo=4p cgdo=2p cj=2e-4 mj=0.5 cjsw=1e-9 mjsw=0.5 js=1e-8 tpg=0\n\
         + fc=0.5 tnom=27)\n\
         .tran 20ns 2us\n\
         .end\n",
    )
    .expect("MOS2 inverter deck parses");
    let run = |integration_method: IntegrationMethod| {
        let engine = Engine::new(rspice_core::engine::resolve_simulation_config(
            &SimulationConfig {
                spice_dialect: SpiceDialect::Xyce,
                integration_method,
                ..SimulationConfig::default()
            },
            Some(&netlist.options),
            &Default::default(),
        ));
        let result = engine
            .run_tran(&netlist, 2.0e-6, 0.2e-6)
            .expect("the inverter transient completes");
        assert_eq!(engine.convergence_quality().force_accepted_points, 0);
        result
    };

    let default = run(IntegrationMethod::TrapGear);
    let trapezoidal = run(IntegrationMethod::Trapezoidal);
    assert_eq!(default.time, trapezoidal.time);
    assert_eq!(default.voltages, trapezoidal.voltages);
}

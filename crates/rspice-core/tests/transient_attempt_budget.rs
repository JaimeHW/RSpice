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

/// ngspice, Xyce, HSPICE and Spectre all integrate with the trapezoidal rule
/// unless a deck asks otherwise, and so does RSpice: the trap/Gear-2 switch
/// is an explicit choice. On this MOS2 inverter the switch's ringing detector
/// fires at the first input edge (1.505 us), so the deck tells the two apart:
/// a default run has to reproduce trapezoidal exactly, and a requested
/// TrapGear has to take a different step sequence.
fn assert_integrates_trapezoidally_by_default(spice_dialect: SpiceDialect) {
    let netlist = Netlist::parse(
        "mos2 inverter, M=2 devices\n\
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
    let run = |integration_method: Option<IntegrationMethod>| {
        let mut config = SimulationConfig {
            spice_dialect,
            ..SimulationConfig::default()
        };
        if let Some(method) = integration_method {
            config.integration_method = method;
        }
        let engine = Engine::new(rspice_core::engine::resolve_simulation_config(
            &config,
            Some(&netlist.options),
            &Default::default(),
        ));
        engine
            .run_tran(&netlist, 2.0e-6, 0.2e-6)
            .expect("the inverter transient completes")
    };

    let default = run(None);
    let trapezoidal = run(Some(IntegrationMethod::Trapezoidal));
    let switched = run(Some(IntegrationMethod::TrapGear));
    assert_eq!(default.time, trapezoidal.time);
    assert_eq!(default.voltages, trapezoidal.voltages);
    assert_ne!(
        switched.time, trapezoidal.time,
        "a requested TrapGear still switches to Gear-2 at the input edge"
    );
}

#[test]
fn ngspice_dialect_integrates_trapezoidally_by_default() {
    assert_integrates_trapezoidally_by_default(SpiceDialect::Ngspice);
}

#[test]
fn best_dialect_integrates_trapezoidally_by_default() {
    assert_integrates_trapezoidally_by_default(SpiceDialect::BestAvailable);
}

#[test]
fn xyce_breakpoint_restart_keeps_a_landing_step_below_a_tenth_of_the_interrupted_one() {
    // Xyce's completeStep leaves the step that landed on a breakpoint as it
    // was (N_TIA_OneStep.C:2242), and the restart takes the smaller of that
    // step and a tenth of the one the breakpoint interrupted
    // (N_TIA_OneStep.C:1719, :1738-1741). Here 0.1 ns steps land 2.54 ps
    // short of the PULSE corner at 2.27 ns. Xyce 7.10 restarts with those
    // 2.54 ps, to 2.27253906e-9; the grown 5.08 ps proposal for the landing
    // step used to restart at 2.27507812e-9 instead.
    let netlist = Netlist::parse(
        "rc breakpoint restart
         v1 in 0 pulse(0 1 1n 1.27n 1n 5n 20n)
         r1 in out 1k
         c1 out 0 1p
         .tran 0.1ns 4ns 0 0.1ns
         .end
",
    )
    .expect("RC deck parses");
    let engine = Engine::new(rspice_core::engine::resolve_simulation_config(
        &SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        },
        Some(&netlist.options),
        &Default::default(),
    ));
    let result = engine
        .run_tran(&netlist, 4.0e-9, 1.0e-10)
        .expect("the RC transient completes");
    let corner = result
        .time
        .iter()
        .position(|time| (time - 2.27e-9).abs() < 1.0e-18)
        .expect("the run lands on the PULSE corner");
    let landing = result.time[corner] - result.time[corner - 1];
    let restart = result.time[corner + 1] - result.time[corner];
    assert!(
        (restart - landing).abs() <= 1.0e-24,
        "landing {landing:e}, restart {restart:e}"
    );
    assert!(
        (result.time[corner + 1] - 2.27253906e-9).abs() < 5.0e-18,
        "{:e}",
        result.time[corner + 1]
    );
}

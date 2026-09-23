//! Independent junction-charge and transport oracle at the carrier handoff.
use num_complex::Complex64;
use rspice_core::analysis::HbConfig;
use rspice_core::config::{ConvergenceConfig, SpiceDialect};
use rspice_core::engine::{
    Engine, HbEnvelopeStateGuarantee, SimulationConfig, TransientCheckpoint,
};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::{FRAC_PI_4, TAU};

#[test]
fn envelope_bjt_restores_transport_depletion_and_substrate_currents() {
    for (kind, polarity, method) in [
        ("NPN", 1.0, IntegrationMethod::Trapezoidal),
        ("PNP", -1.0, IntegrationMethod::Gear2),
    ] {
        let deck = Netlist::parse(&format!(
            "BJT carrier\nVC c 0 SIN({} {} 1k 0 0 90)\n\
            VB b 0 SIN({} {} 1k)\nVE e 0 0\n\
            VS s 0 SIN({} {} 1k 0 0 45)\n\
            Q1 c b e s QM IC=0,0 OFF\n\
            .model QM {kind}(IS=1e-10 BF=100 BR=2 CJE=100n CJC=200n CJS=50n \
            VJE=1 VJC=1 VJS=1 MJE=.5 MJC=.5 MJS=.5 FC=.5 XCJC=.4 TF=20u TR=10u)\n\
            .temp 27\n.options tnom=27\n.options hbint tahb=0\n\
            .print tran IC(Q1) IB(Q1) IE(Q1) IS(Q1)\n.end\n",
            0.8 * polarity,
            0.08 * polarity,
            0.4 * polarity,
            0.015 * polarity,
            -0.2 * polarity,
            0.04 * polarity,
        ))
        .unwrap();
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            convergence_config: ConvergenceConfig::default()
                .with_voltage_tolerances(1e-8, 1e-11)
                .with_residual_reltol(1e-8),
            ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice)
        });
        let mut config = HbConfig::new(1e3)
            .with_harmonics(10)
            .with_collocation_points(65);
        config.tolerance = 1e-9;
        config.abstol = 1e-13;
        let result = engine
            .run_envelope_with_abort(&deck, config, &[], 0.35e-3, 0.25e-6, &NoAbort)
            .unwrap();
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExactJunctionRlcMnaV1
        );
        let thermal = 300.15 * 1.38064852e-23 / 1.6021766208e-19;
        let expected = |time: f64| {
            let angle = TAU * 1e3 * time;
            let vb = 0.4 + 0.015 * angle.sin();
            let vc = 0.8 + 0.08 * angle.cos();
            let vs = -0.2 + 0.04 * (angle + FRAC_PI_4).sin();
            let db = 0.015 * TAU * 1e3 * angle.cos();
            let dc = -0.08 * TAU * 1e3 * angle.sin();
            let ds = 0.04 * TAU * 1e3 * (angle + FRAC_PI_4).cos();
            let ef = (vb / thermal).exp();
            let er = ((vb - vc) / thermal).exp();
            let forward = 1e-10 * (ef - 1.0);
            let reverse = 1e-10 * (er - 1.0);
            let be = (100e-9 / (1.0 - vb).sqrt() + 20e-6 * 1e-10 / thermal * ef) * db;
            let bc = (200e-9 / (1.0 - vb + vc).sqrt() + 10e-6 * 1e-10 / thermal * er) * (db - dc);
            // Native ngspice defaults are vertical NPN (collector/substrate)
            // and lateral PNP (base/substrate). The latter is forward biased
            // here, using the substrate's FC=0 linear capacitance extension.
            let lateral = kind == "PNP";
            let substrate = if lateral {
                50e-9 * (1.0 + 0.5 * (vb - vs)) * (db - ds)
            } else {
                50e-9 / (1.0 + vc - vs).sqrt() * (dc - ds)
            };
            [
                polarity
                    * (forward - reverse - reverse / 2.0 - bc
                        + if lateral { 0.0 } else { substrate }),
                polarity
                    * (forward / 100.0
                        + reverse / 2.0
                        + be
                        + bc
                        + if lateral { substrate } else { 0.0 }),
                polarity * (-forward + reverse - forward / 100.0 - be),
                -polarity * substrate,
            ]
        };
        let transient = result.continued_transient();
        for (terminal, (parameter, source)) in
            [("IC", "VC"), ("IB", "VB"), ("IE", "VE"), ("IS", "VS")]
                .into_iter()
                .enumerate()
        {
            let current = transient
                .try_device_op_waveform_named("Q1", parameter)
                .unwrap();
            let source_current = transient.try_branch_current_waveform_named(source).unwrap();
            assert!(
                (current[0] - expected(0.0)[terminal]).abs() < 2e-10,
                "{kind} initial {parameter}: {} vs {}",
                current[0],
                expected(0.0)[terminal]
            );
            for ((&time, &value), &source_value) in
                transient.time.iter().zip(current).zip(source_current)
            {
                assert!(
                    (value - expected(time)[terminal]).abs() < 2e-7,
                    "{kind} {parameter} at {time}: {value} vs {}",
                    expected(time)[terminal]
                );
                assert!(
                    (value + source_value).abs() < 2e-9,
                    "{kind} {parameter} KCL at {time}: {value} + {source_value}"
                );
            }
        }
        let checkpoint =
            TransientCheckpoint::from_text(&result.final_checkpoint().to_text()).unwrap();
        assert!(checkpoint.capability().is_resumable());
    }
}

#[test]
fn envelope_bjt_restores_private_base_state_and_external_bc_charge() {
    // Negligible transport makes this an independently solvable linear RC
    // network. RB != RBM retains the promoted native base-resistance state;
    // at zero transport current its value is exactly RB.
    let deck = Netlist::parse(
        "Private base carrier\nVB b 0 SIN(.2 .03 1k)\n\
        VC c 0 SIN(.5 .03 1k 0 0 90)\nVE e 0 0\n\
        Q1 c b e QM OFF\n\
        .model QM NPN(IS=1e-30 RB=20 RBM=10 IRB=1 CJE=100n CJC=50n \
        MJE=0 MJC=0 XCJC=.4)\n\
        .print tran IC(Q1) IB(Q1) IE(Q1)\n.options hbint tahb=0\n.end\n",
    )
    .unwrap();
    let engine = Engine::new(SimulationConfig {
        convergence_config: ConvergenceConfig::default()
            .with_voltage_tolerances(1e-8, 1e-11)
            .with_residual_reltol(1e-8),
        ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice)
    });
    let mut config = HbConfig::new(1e3).with_harmonics(3);
    config.tolerance = 1e-9;
    config.abstol = 1e-13;
    let result = engine
        .run_envelope_with_abort(&deck, config, &[], 0.3e-3, 0.5e-6, &NoAbort)
        .unwrap();
    let omega = TAU * 1e3;
    let jomega = Complex64::new(0.0, omega);
    let base = Complex64::new(0.0, -0.03);
    let collector = Complex64::new(0.03, 0.0);
    let intrinsic = (base / 20.0 + jomega * 20e-9 * collector) / (1.0 / 20.0 + jomega * 120e-9);
    let cb = jomega * 20e-9 * (collector - intrinsic) + jomega * 30e-9 * (collector - base);
    let be = -jomega * 100e-9 * intrinsic;
    let currents = [cb, -cb - be, be];
    let transient = result.continued_transient();
    for (i, (parameter, source)) in [("IC", "VC"), ("IB", "VB"), ("IE", "VE")]
        .into_iter()
        .enumerate()
    {
        let measured = transient
            .try_device_op_waveform_named("Q1", parameter)
            .unwrap();
        let supplied = transient.try_branch_current_waveform_named(source).unwrap();
        assert!((measured[0] - currents[i].re).abs() < 2e-10);
        for ((&time, &current), &supply) in transient.time.iter().zip(measured).zip(supplied) {
            let expected = (currents[i] * Complex64::from_polar(1.0, omega * time)).re;
            assert!(
                (current - expected).abs() < 1e-7,
                "{parameter} at {time}: {current} vs {expected}"
            );
            assert!(
                (current + supply).abs() < 2e-9,
                "{parameter} KCL at {time}: {current} + {supply}"
            );
        }
    }
    assert!(
        TransientCheckpoint::from_text(&result.final_checkpoint().to_text())
            .unwrap()
            .capability()
            .is_resumable()
    );
}

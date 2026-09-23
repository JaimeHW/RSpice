//! HB carrier charge and physical lead currents for classic N/P JFETs.
use rspice_core::analysis::HbConfig;
use rspice_core::config::{ConvergenceConfig, SpiceDialect};
use rspice_core::engine::{Engine, HbEnvelopeStateGuarantee, SimulationConfig};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn envelope_jfet_preserves_both_gate_charges_and_all_terminal_currents() {
    for (kind, polarity, method) in [
        ("NJF", 1.0, IntegrationMethod::Trapezoidal),
        ("PJF", -1.0, IntegrationMethod::Gear2),
    ] {
        let deck = Netlist::parse(&format!(
            "JFET carrier\nVD d 0 SIN({} {} 1k 0 0 90)\n\
            VG g 0 SIN({} {} 1k)\nVS s 0 0\n\
            J1 d g s JM IC=0,0 OFF\n\
            .model JM {kind}(VTO=-2 BETA=1m LAMBDA=.02 IS=1e-14 CGS=100n CGD=200n PB=1 M=.5 FC=.5)\n\
            .print tran ID(J1) IG(J1) IS(J1)\n.options hbint tahb=0\n.end\n",
            0.7*polarity, 0.15*polarity, -0.4*polarity, 0.1*polarity
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            convergence_config: ConvergenceConfig::default()
                .with_voltage_tolerances(1e-8, 1e-11)
                .with_residual_reltol(1e-8),
            ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice)
        });
        let mut config = HbConfig::new(1e3)
            .with_harmonics(8)
            .with_collocation_points(65);
        config.tolerance = 1e-9;
        config.abstol = 1e-13;
        let result = engine
            .run_envelope_with_abort(&deck, config, &[], 0.4e-3, 0.5e-6, &NoAbort)
            .unwrap();
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExactJunctionRlcMnaV1
        );
        let transient = result.continued_transient();
        let expected = |time: f64| {
            let angle = TAU * 1e3 * time;
            let vg = -0.4 + 0.1 * angle.sin();
            let vd = 0.7 + 0.15 * angle.cos();
            let channel = polarity * 1e-3 * (2.0 * (vg + 2.0) * vd - vd * vd) * (1.0 + 0.02 * vd);
            let gs = polarity * 100e-9 / (1.0 - vg).sqrt() * 0.1 * TAU * 1e3 * angle.cos();
            let gd = polarity * 200e-9 / (1.0 - vg + vd).sqrt()
                * TAU
                * 1e3
                * (0.1 * angle.cos() + 0.15 * angle.sin());
            [channel - gd, gs + gd, -channel - gs]
        };
        for (terminal, (parameter, source)) in [("ID", "VD"), ("IG", "VG"), ("IS", "VS")]
            .into_iter()
            .enumerate()
        {
            let current = transient
                .try_device_op_waveform_named("J1", parameter)
                .unwrap();
            let source_current = transient.try_branch_current_waveform_named(source).unwrap();
            assert!(
                (current[0] - expected(0.0)[terminal]).abs() < 1e-10,
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
        assert!(result.final_checkpoint().capability().is_resumable());
    }
}

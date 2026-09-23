//! Junction charge and accepted current survive the HB-to-TRAN origin.
use rspice_core::analysis::HbConfig;
use rspice_core::config::SpiceDialect;
use rspice_core::engine::{
    Engine, HbEnvelopeStateGuarantee, SimulationConfig, TransientCheckpoint,
};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::{PI, TAU};

#[test]
fn envelope_restores_depletion_diffusion_and_limiter_history() {
    let deck = Netlist::parse(
        "Diode carrier handoff\n\
        Vref reference 0 SIN(-.2 .1 1k 0 0 30)\n\
        Bdrive 0 out I=V(reference)/1k+1u/sqrt(1-V(reference))*.1*2*pi*1k*cos(2*pi*1k*time+pi/6)\n\
        Rload out 0 1k\nDcharge out 0 DM IC=.9 OFF\n\
        .model DM D(IS=1e-14 CJO=1u VJ=1 M=.5 FC=.5 TT=0)\n\
        Vforward forward 0 SIN(.5 .01 1k)\nDforward forward 0 DF IC=-1 OFF\n\
        .model DF D(IS=1e-12 CJO=1n VJ=1 M=.5 FC=.9 TT=1u)\n\
        .temp 27\n.options tnom=27\n.options hbint tahb=0\n.end\n",
    )
    .unwrap();
    for (dialect, method, thermal) in [
        (
            SpiceDialect::Ngspice,
            IntegrationMethod::Trapezoidal,
            300.15 * 1.38064852e-23 / 1.6021766208e-19,
        ),
        (
            SpiceDialect::Xyce,
            IntegrationMethod::Gear2,
            300.15 * 1.3806226e-23 / 1.6021918e-19,
        ),
    ] {
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            ..SimulationConfig::default().with_spice_dialect(dialect)
        });
        let mut config = HbConfig::new(1e3)
            .with_harmonics(8)
            .with_collocation_points(65);
        config.tolerance = 1e-9;
        config.abstol = 1e-13;
        let result = engine
            .run_envelope_with_abort(&deck, config, &[], 0.35e-3, 0.5e-6, &NoAbort)
            .unwrap();
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExactJunctionRlcMnaV1
        );
        let transient = result.continued_transient();
        let out = transient.try_voltage_waveform_named("out").unwrap();
        let diode = transient
            .try_branch_current_waveform_named("Dcharge")
            .unwrap();
        let forward = transient
            .try_branch_current_waveform_named("Dforward")
            .unwrap();
        let expected = |time: f64| {
            let angle = TAU * 1e3 * time + PI / 6.0;
            let voltage = -0.2 + 0.1 * angle.sin();
            let depletion = 1e-6 / (1.0 - voltage).sqrt() * 0.1 * TAU * 1e3 * angle.cos();
            let forward_voltage = 0.5 + 0.01 * (TAU * 1e3 * time).sin();
            let exponential = (forward_voltage / thermal).exp();
            let conduction = 1e-12 * (exponential - 1.0);
            let capacitance =
                1e-9 / (1.0 - forward_voltage).sqrt() + 1e-6 * 1e-12 * exponential / thermal;
            let total = conduction + capacitance * 0.01 * TAU * 1e3 * (TAU * 1e3 * time).cos();
            (voltage, depletion, total)
        };
        let origin = expected(0.0);
        assert!(
            (out[0] - origin.0).abs() < 1e-8,
            "{dialect:?}: initial voltage {}",
            out[0]
        );
        assert!(
            (diode[0] - origin.1).abs() < 1e-10,
            "{dialect:?}: initial depletion {} vs {}",
            diode[0],
            origin.1
        );
        assert!(
            (forward[0] - origin.2).abs() < 1e-10,
            "{dialect:?}: initial diffusion {} vs {}",
            forward[0],
            origin.2
        );
        for (((&time, &voltage), &current), &forward_current) in
            transient.time.iter().zip(out).zip(diode).zip(forward)
        {
            let (expected_voltage, expected_current, expected_forward) = expected(time);
            assert!(
                (voltage - expected_voltage).abs() < 2e-5,
                "{dialect:?} voltage at {time}: {voltage} vs {expected_voltage}"
            );
            assert!(
                (current - expected_current).abs() < 2e-7,
                "{dialect:?} depletion at {time}: {current} vs {expected_current}"
            );
            assert!(
                (forward_current - expected_forward).abs() < 2e-8,
                "{dialect:?} diffusion at {time}: {forward_current} vs {expected_forward}"
            );
        }
        let checkpoint =
            TransientCheckpoint::from_text(&result.final_checkpoint().to_text()).unwrap();
        assert!(checkpoint.capability().is_resumable());
        let tag =
            rspice_core::execution::result_document::EnvelopeGuaranteeTag::from(result.guarantee());
        assert_eq!(
            serde_json::to_string(&tag).unwrap(),
            "\"exact-junction-rlc-mna-v1\""
        );
    }
}

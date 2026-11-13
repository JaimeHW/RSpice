//! Envelope warm starts retain linear MNA and mutually coupled winding state.
use rspice_core::analysis::HbConfig;
use rspice_core::config::SpiceDialect;
use rspice_core::engine::{Engine, HbEnvelopeStateGuarantee, SimulationConfig};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Complex64, Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn envelope_rlc_restores_coupled_currents_ic_branches_and_controlled_sources() {
    let deck = Netlist::parse(
        "RLC envelope\n\
        Vcarrier carrier 0 SIN(.4 .2 1k)\nRref carrier 0 2k\n\
        Vmod mod 0 PWL(0 0 2m 0 2.1m .1 3m .1)\n\
        Edrive drive 0 carrier 0 2\nRdrive drive a 100\n\
        L1 a out 10m IC=.5\nL2 sec 0 2.5m IC=.1\nK1 L1 L2 .8\nRsec sec 0 50\n\
        C1 out 0 1u IC=3.5\nRload out 0 1k\nRmod mod out 1k\n\
        Rzero z out 0\nRz z 0 2k\n\
        Gcopy g 0 carrier 0 1m\nRg g 0 1k\n\
        Fcopy f 0 Vcarrier 2\nRf f 0 1k\n\
        Hcopy h 0 Vcarrier 100\nRh h 0 1k\n.end\n",
    )
    .unwrap();
    // Independent frequency-domain circuit solution. The secondary load
    // reflects through M into the primary series impedance.
    let jw = Complex64::new(0.0, TAU * 1e3);
    let mutual = 0.8 * (10e-3_f64 * 2.5e-3).sqrt();
    let primary = jw * 10e-3 - (jw * mutual).powu(2) / (50.0 + jw * 2.5e-3);
    let load = 1.0 / (1.0 / 400.0 + jw * 1e-6);
    let current = Complex64::new(0.0, -0.4) / (100.0 + primary + load);
    let out = load * current;
    let secondary = -jw * mutual * current / (50.0 + jw * 2.5e-3);
    let expected = |phasor: Complex64, dc: f64, time: f64| {
        dc + (phasor * Complex64::from_polar(1.0, TAU * 1e3 * time)).re
    };
    for method in [IntegrationMethod::Trapezoidal, IntegrationMethod::Gear2] {
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            ..SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce)
        });
        let result = engine
            .run_envelope_with_abort(
                &deck,
                HbConfig::new(1e3).with_harmonics(3),
                &["Vmod".into()],
                0.4e-3,
                2e-6,
                &NoAbort,
            )
            .unwrap();
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExactLinearRlcMnaV1
        );
        let transient = result.continued_transient();
        let waveform = transient.try_voltage_waveform_named("out").unwrap();
        assert!((waveform[0] - expected(out, 0.64, 0.0)).abs() < 1e-11);
        // Authored IC clamps seed OP only; the continuation carries the solved
        // periodic voltage and winding/lead currents through the zero origin.
        for (&time, &value) in transient.time.iter().zip(waveform) {
            assert!(
                (value - expected(out, 0.64, time)).abs() < 5e-4,
                "{method:?} {time}: {value}"
            );
        }
        for (name, phasor, dc) in [
            ("L1", current, 0.8 / 500.0),
            ("L2", secondary, 0.0),
            ("C1", jw * 1e-6 * out, 0.0),
            ("Rzero", -out / 2e3, -0.64 / 2e3),
        ] {
            let values = transient.try_branch_current_waveform_named(name).unwrap();
            assert!(
                (values[0] - expected(phasor, dc, 0.0)).abs() < 1e-12,
                "initial {name}"
            );
            for (&time, &value) in transient.time.iter().zip(values) {
                assert!(
                    (value - expected(phasor, dc, time)).abs() < 1e-5,
                    "{method:?} {name} {time}: {value}"
                );
            }
        }
        for (name, gain) in [("g", -1.0), ("f", 1.0), ("h", -0.05)] {
            for (&time, &value) in transient
                .time
                .iter()
                .zip(transient.try_voltage_waveform_named(name).unwrap())
            {
                assert!((value - gain * (0.4 + 0.2 * (TAU * 1e3 * time).sin())).abs() < 1e-10);
            }
        }
        // The completeness contract survives portable result serialization.
        let tag =
            rspice_core::execution::result_document::EnvelopeGuaranteeTag::from(result.guarantee());
        let encoded = serde_json::to_string(&tag).unwrap();
        assert_eq!(encoded, "\"exact-linear-rlc-mna-v1\"");
        let restored: rspice_core::execution::result_document::EnvelopeGuaranteeTag =
            serde_json::from_str(&encoded).unwrap();
        assert_eq!(HbEnvelopeStateGuarantee::from(restored), result.guarantee());
    }
}

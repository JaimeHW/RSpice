//! Behavioral memory must survive the HB carrier to Envelope handoff.
use rspice_core::analysis::HbConfig;
use rspice_core::engine::{Engine, HbEnvelopeStateGuarantee, SimulationConfig};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Complex64, Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn envelope_restores_feedback_and_nested_integrals_and_reactivates_modulation() {
    let deck = Netlist::parse(
        "Behavioral envelope\n\
        Vin input 0 SIN(.7 .2 1k 0 0 90)\n\
        Vcos cosine 0 SIN(0 1 1k 0 0 90)\n\
        Vmod mod 0 PWL(0 .1 .1m .1 .2m .2 1m .2)\n\
        BV out 0 V=.2+1k*SDT(V(input)-V(out))\nRout out 0 1k\n\
        BI sink 0 I=-SDT(V(out)-V(sink))\nRsink sink 0 1k\n\
        BN nested 0 V=(2*pi*1k)^2*SDT(SDT(V(cosine)))\nRn nested 0 1k\n\
        BP power 0 V=V(input)^2+V(mod)\nRp power 0 1k\nCp power 0 1n\n\
        .options hbint tahb=0\n.end\n",
    )
    .unwrap();
    let h = 1e3 / Complex64::new(1e3, TAU * 1e3);
    for method in [IntegrationMethod::Trapezoidal, IntegrationMethod::Gear2] {
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            ..SimulationConfig::default()
        });
        let result = engine
            .run_envelope_with_abort(
                &deck,
                HbConfig::new(1e3).with_harmonics(3),
                &["Vmod".into()],
                0.4e-3,
                1e-6,
                &NoAbort,
            )
            .unwrap();
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExactBehavioralRlcMnaV1
        );
        let transient = result.continued_transient();
        for (name, transfer) in [("out", h), ("sink", h * h)] {
            let values = transient.try_voltage_waveform_named(name).unwrap();
            // In particular, neither integral's nonzero solved constant may
            // be replaced by the authored zero-memory transient startup.
            assert!((values[0] - (0.7 + 0.2 * transfer.re)).abs() < 1e-9);
            for (&time, &value) in transient.time.iter().zip(values) {
                let expected =
                    0.7 + 0.2 * (transfer * Complex64::from_polar(1.0, TAU * 1e3 * time)).re;
                assert!(
                    (value - expected).abs() < 2e-6,
                    "{method:?} {name} at {time}: {value} vs {expected}"
                );
            }
        }
        for (&time, &value) in transient
            .time
            .iter()
            .zip(transient.try_voltage_waveform_named("nested").unwrap())
        {
            let expected = 1.0 - (TAU * 1e3 * time).cos();
            assert!(
                (value - expected).abs() < 2e-5,
                "{method:?} nested at {time}: {value} vs {expected}"
            );
        }
        let currents = transient.try_branch_current_waveform_named("BP").unwrap();
        for ((&time, &value), &current) in transient
            .time
            .iter()
            .zip(transient.try_voltage_waveform_named("power").unwrap())
            .zip(currents)
        {
            let input = 0.7 + 0.2 * (TAU * 1e3 * time).cos();
            let modulation = 0.1 + ((time - 0.1e-3) * 1e3).clamp(0.0, 0.1);
            let expected = input * input + modulation;
            assert!((value - expected).abs() < 1e-9, "power at {time}");
            // Branch current must include the capacitor contribution after
            // the modulation waveform is reactivated at slow-time zero.
            if time < 0.09e-3 || time > 0.21e-3 {
                let derivative = -2.0 * input * 0.2 * TAU * 1e3 * (TAU * 1e3 * time).sin();
                assert!(
                    (current + expected / 1e3 + 1e-9 * derivative).abs() < 2e-8,
                    "{method:?} BP at {time}: {current}"
                );
            }
        }
        let tag =
            rspice_core::execution::result_document::EnvelopeGuaranteeTag::from(result.guarantee());
        let encoded = serde_json::to_string(&tag).unwrap();
        assert_eq!(encoded, "\"exact-behavioral-rlc-mna-v1\"");
        let restored: rspice_core::execution::result_document::EnvelopeGuaranteeTag =
            serde_json::from_str(&encoded).unwrap();
        assert_eq!(HbEnvelopeStateGuarantee::from(restored), result.guarantee());
    }
}

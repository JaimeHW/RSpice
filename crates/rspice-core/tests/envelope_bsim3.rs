//! Native BSIM3 carrier-to-transient continuation, including NQS storage.
use rspice_core::analysis::HbConfig;
use rspice_core::engine::{
    Engine, HbEnvelopeStateGuarantee, SimulationConfig, TransientCheckpoint,
};
use rspice_core::execution::result_document::EnvelopeGuaranteeTag;
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn bsim3_continuation_preserves_the_carrier_orbit_and_portable_checkpoint() {
    for (nqs, kind, polarity, method) in [
        (0, "NMOS", 1.0, IntegrationMethod::Gear2),
        (1, "PMOS", -1.0, IntegrationMethod::Trapezoidal),
    ] {
        let deck = Netlist::parse(&format!(
            "BSIM3 Envelope\nVDD supply 0 {}\nVIN in 0 SIN({} {} 1G)\nRD supply out 500\nRG in gate 100\nRS source 0 10\nM1 out gate source 0 mm L=.18u W=10u AD=4p AS=4p PD=20u PS=20u M=2 OFF\n.model mm {kind}(LEVEL=49 TOX=4.1n VTH0={} U0=270 K1=.59 K2=.0026 CAPMOD=2 NQSMOD={nqs} RSH=10 CGDO=7.9e-10 CGSO=6.3e-10 CJ=9.5e-4 CJSW=2.4e-10)\n.options hbint tahb=0\n.end\n",
            polarity * 1.8, polarity * 0.7, polarity * 0.01, polarity * 0.37,
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            ..Default::default()
        });
        let config = HbConfig::new(1e9).with_harmonics(5).with_tolerance(1e-10);
        let result = engine
            .run_envelope_with_abort(&deck, config, &[], 0.25e-9, 0.5e-12, &NoAbort)
            .unwrap_or_else(|error| panic!("{kind} NQS={nqs}: {error}"));
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExactBsim3RlcMnaV1
        );
        let transient = result.continued_transient();
        for (name, values) in transient.node_names.iter().zip(&transient.voltages) {
            let spectrum = result
                .carrier()
                .spectral_voltages
                .iter()
                .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            let amplitude = spectrum
                .coefficients
                .iter()
                .skip(1)
                .map(|v| v.norm())
                .sum::<f64>();
            for (&time, &actual) in transient.time.iter().zip(values) {
                let expected = spectrum
                    .coefficients
                    .iter()
                    .enumerate()
                    .map(|(harmonic, coefficient)| {
                        (coefficient
                            * num_complex::Complex64::from_polar(
                                1.0,
                                TAU * 1e9 * time * harmonic as f64,
                            ))
                        .re
                    })
                    .sum::<f64>();
                assert!(
                    (actual - expected).abs() < 1e-9 + 2e-3 * amplitude,
                    "{kind} NQS={nqs} {name} at {time}: {actual} vs {expected}"
                );
            }
        }
        let checkpoint =
            TransientCheckpoint::from_text(&result.final_checkpoint().to_text()).unwrap();
        assert!(checkpoint.capability().is_resumable());
        engine
            .run_tran_resume(&deck, &checkpoint, 0.26e-9, 0.5e-12)
            .unwrap();
        let tag = EnvelopeGuaranteeTag::from(result.guarantee());
        let encoded = serde_json::to_string(&tag).unwrap();
        assert_eq!(encoded, "\"exact-bsim3-rlc-mna-v1\"");
        let decoded: EnvelopeGuaranteeTag = serde_json::from_str(&encoded).unwrap();
        assert_eq!(HbEnvelopeStateGuarantee::from(decoded), result.guarantee());
    }
}

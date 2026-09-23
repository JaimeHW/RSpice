//! Carrier-to-transient transfer of both waves in a mismatched delay line.
use num_complex::Complex64;
use rspice_core::analysis::harmonic_balance::HbContinuationLimitation;
use rspice_core::analysis::{HbConfig, HbResult};
use rspice_core::engine::{
    Engine, HbEnvelopeStateGuarantee, SimulationConfig, SpiceDialect, TransientCheckpoint,
    TransientResult,
};
use rspice_core::execution::result_document::{EnvelopeGuaranteeTag, HbContinuationLimitationTag};
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

const FREQ: f64 = 1e6;

fn compare_carrier(carrier: &HbResult, transient: &TransientResult) {
    let compare = |name: &str, coefficients: &[Complex64], values: &[f64], absolute: f64| {
        let amplitude: f64 = coefficients.iter().map(|c| c.norm()).sum();
        for (&time, &actual) in transient.time.iter().zip(values) {
            let expected: f64 = coefficients
                .iter()
                .enumerate()
                .map(|(k, c)| (c * Complex64::from_polar(1.0, TAU * FREQ * k as f64 * time)).re)
                .sum();
            assert!(
                (actual - expected).abs() < absolute + 2e-4 * amplitude,
                "{name} at {time:e}: {actual:e} vs {expected:e}"
            );
        }
    };
    for (name, values) in transient.node_names.iter().zip(&transient.voltages) {
        let spectrum = carrier
            .spectral_voltages
            .iter()
            .find(|row| row.node_name.eq_ignore_ascii_case(name))
            .unwrap();
        compare(name, &spectrum.coefficients, values, 1e-7);
    }
    for name in ["vdrive", "vharm", "vref"] {
        let spectrum = carrier
            .mna_branch_currents
            .iter()
            .find(|row| row.device_name.eq_ignore_ascii_case(name))
            .unwrap();
        compare(
            name,
            &spectrum.coefficients,
            transient.try_branch_current_waveform_named(name).unwrap(),
            1e-9,
        );
    }
}

#[test]
fn envelope_line_history_crosses_origin_and_round_trips_for_short_and_long_delays() {
    for (delay, dialect) in [
        (137e-9, SpiceDialect::Ngspice),
        (2.137e-6, SpiceDialect::Xyce),
    ] {
        let deck = Netlist::parse(&format!(
            "Envelope lossless history\nVREF ref 0 0.35\nVDRIVE drive mid SIN(0.2 0.8 1meg 0 0 27)\nVHARM mid ref SIN(0.1 0.07 3meg 0 0 13)\nRS drive near 33\nT1 near ref far ref Z0=50 TD={delay:e}\nRL far ref 75\n.save all\n.end\n"
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            ..Default::default()
        });
        let result = engine
            .run_envelope_with_abort(
                &deck,
                HbConfig::new(FREQ).with_harmonics(3),
                &[],
                delay * 0.11,
                1.0 / FREQ / 1024.0,
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("TD={delay} {dialect:?}: {error}"));
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::SampledDelayHistoryV1
        );
        assert!(
            result
                .carrier()
                .continuation_limitations
                .contains(&HbContinuationLimitation::TransmissionLineHistoryNotRetained)
        );
        compare_carrier(result.carrier(), result.continued_transient());
        let checkpoint =
            TransientCheckpoint::from_text(&result.final_checkpoint().to_text()).unwrap();
        assert!(checkpoint.capability().is_resumable());
        let (resumed, _) = engine
            .run_tran_resume(
                &deck,
                &checkpoint,
                2.0 * delay + 0.4 / FREQ,
                1.0 / FREQ / 1024.0,
            )
            .unwrap();
        compare_carrier(result.carrier(), &resumed);
        let encoded =
            serde_json::to_string(&EnvelopeGuaranteeTag::from(result.guarantee())).unwrap();
        assert_eq!(encoded, "\"sampled-delay-history-v1\"");
        let decoded: EnvelopeGuaranteeTag = serde_json::from_str(&encoded).unwrap();
        assert_eq!(HbEnvelopeStateGuarantee::from(decoded), result.guarantee());
        let tag = HbContinuationLimitationTag::from(
            &HbContinuationLimitation::TransmissionLineHistoryNotRetained,
        );
        let tag: HbContinuationLimitationTag =
            serde_json::from_str(&serde_json::to_string(&tag).unwrap()).unwrap();
        assert_eq!(
            HbContinuationLimitation::from(tag),
            HbContinuationLimitation::TransmissionLineHistoryNotRetained
        );
    }
}

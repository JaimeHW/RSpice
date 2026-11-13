//! Native Meyer/legacy BSIM carrier history and transient continuation.
use rspice_core::analysis::HbConfig;
use rspice_core::engine::{
    Engine, HbEnvelopeStateGuarantee, SimulationConfig, SpiceDialect, TransientCheckpoint,
};
use rspice_core::execution::result_document::EnvelopeGuaranteeTag;
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn classic_mos_envelope_reconstructs_native_history_and_portable_continuation() {
    for (level, p, dialect, method) in [
        (
            1,
            1.0,
            SpiceDialect::Ngspice,
            IntegrationMethod::Trapezoidal,
        ),
        (3, 1.0, SpiceDialect::Xyce, IntegrationMethod::Gear2),
        (
            4,
            -1.0,
            SpiceDialect::Ngspice,
            IntegrationMethod::Trapezoidal,
        ),
        (5, 1.0, SpiceDialect::Ngspice, IntegrationMethod::Gear2),
    ] {
        let kind = if p < 0.0 { "PMOS" } else { "NMOS" };
        let model = if matches!(level, 4 | 5) {
            "VFB=-0.8 PHI=0.7 K1=0.5 TOX=0.02".to_string()
        } else {
            format!("VTO={} KP=50u GAMMA=0.5 PHI=0.7 TOX=20n", p * 0.6)
        };
        let terminals = if level == 3 {
            "source gate out bulk"
        } else {
            "out gate source bulk"
        };
        let deck = Netlist::parse(&format!(
            "Classic MOS Envelope\nVDD supply 0 {}\nVIN in 0 SIN({} {} 100meg)\nVB bulk 0 SIN({} {} 100meg 0 0 35)\nRD supply out 500\nRG in gate 10000\nRS source 0 10\nM1 {terminals} mm L=1u W=10u AD=4p AS=5p PD=20u PS=22u M=2 OFF\n.model mm {kind} LEVEL={level} {model} CGSO=1e-9 CGDO=2e-9 CGBO=1e-9 CJ=0.001 CJSW=1e-10 MJ=0.4 MJSW=0.23 RSH=10\n.options TEMP=60 hbint tahb=0\n.save all\n.print tran ID(M1) IG(M1) IS(M1) IB(M1)\n.end\n",
            p*0.4,p*1.2,p*0.1,-p*0.2,p*0.03,
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            spice_dialect: dialect,
            integration_method: method,
            ..Default::default()
        });
        let result = engine
            .run_envelope_with_abort(
                &deck,
                HbConfig::new(1e8).with_harmonics(7).with_tolerance(1e-10),
                &[],
                10e-9,
                10e-9 / 1024.0,
                &NoAbort,
            )
            .unwrap_or_else(|error| panic!("level {level} {kind} {dialect:?} {method:?}: {error}"));
        assert_eq!(
            result.guarantee(),
            HbEnvelopeStateGuarantee::ExactClassicMosRlcMnaV1
        );
        let tran = result.continued_transient();
        for (name, values) in tran.node_names.iter().zip(&tran.voltages) {
            let spectrum = result
                .carrier()
                .spectral_voltages
                .iter()
                .find(|row| row.node_name.eq_ignore_ascii_case(name))
                .unwrap();
            let amplitude: f64 = spectrum.coefficients.iter().skip(1).map(|v| v.norm()).sum();
            for (&time, &actual) in tran.time.iter().zip(values) {
                let expected: f64 = spectrum
                    .coefficients
                    .iter()
                    .enumerate()
                    .map(|(k, c)| {
                        (c * num_complex::Complex64::from_polar(1.0, TAU * 1e8 * k as f64 * time))
                            .re
                    })
                    .sum();
                assert!(
                    (actual - expected).abs() < 1e-8 + 2e-3 * amplitude,
                    "level {level} {kind} {dialect:?} {name} t={time}: {actual} vs {expected}"
                );
            }
        }
        // The synthetic origin must retain actual displacement lead currents,
        // not replace them with DC reports before the first accepted step.
        let lead = |parameter: &str| tran.try_device_op_waveform_named("m1", parameter).unwrap()[0];
        let branch = |source: &str| tran.try_branch_current_waveform_named(source).unwrap()[0];
        for (parameter, source) in [
            ("ig", "vin"),
            ("ib", "vb"),
            (if level == 3 { "is" } else { "id" }, "vdd"),
        ] {
            let actual = lead(parameter);
            assert!(
                (actual + branch(source)).abs() < 1e-10 + 1e-4 * actual.abs(),
                "level {level} initial {parameter}={} vs source={}",
                actual,
                branch(source)
            );
        }
        assert!(
            ["id", "ig", "is", "ib"]
                .into_iter()
                .map(lead)
                .sum::<f64>()
                .abs()
                < 1e-12
        );
        let checkpoint =
            TransientCheckpoint::from_text(&result.final_checkpoint().to_text()).unwrap();
        assert!(checkpoint.capability().is_resumable());
        engine
            .run_tran_resume(&deck, &checkpoint, 10.1e-9, 10e-9 / 1024.0)
            .unwrap();
        let encoded =
            serde_json::to_string(&EnvelopeGuaranteeTag::from(result.guarantee())).unwrap();
        assert_eq!(encoded, "\"exact-classic-mos-rlc-mna-v1\"");
        let decoded: EnvelopeGuaranteeTag = serde_json::from_str(&encoded).unwrap();
        assert_eq!(HbEnvelopeStateGuarantee::from(decoded), result.guarantee());
    }
}

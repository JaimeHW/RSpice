//! BSIM3 shooting state, coupled charge, and ordinary transient handoff.
use rspice_core::analysis::{HbConfig, PssConfig};
use rspice_core::engine::{Engine, SimulationConfig, TransientCheckpoint};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn bsim3_pss_overlap_storage_matches_rc_waveform_and_physical_mode() {
    let netlist = Netlist::parse(
        "BSIM3 overlap RC\nVIN in 0 SIN(0 .1 1meg)\nR1 in out 1k\nM1 0 out 0 0 mm L=1u W=10u M=2 OFF\n.model mm NMOS LEVEL=49 VTH0=3 CAPMOD=0 XPART=-1 CGSO=1e-5 CGDO=0 CGBO=0 CF=0\n.end\n"
    ).unwrap();
    let point = Engine::default()
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::new(1e6)
                .with_points_per_period(64)
                .with_tstab_periods(0),
            &NoAbort,
        )
        .unwrap();
    assert_eq!(
        point.shooting_state_basis().len(),
        1,
        "one physical gate charge, no drain/body state"
    );
    let result = &point.analysis().result;
    let output = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap();
    let amplitude = num_complex::Complex64::new(0.0, -0.1)
        / num_complex::Complex64::new(1.0, TAU * 1e6 * 200e-9);
    for (&time, &actual) in result.time.iter().zip(&result.waveforms[output].values) {
        let expected = (amplitude * num_complex::Complex64::from_polar(1.0, TAU * 1e6 * time)).re;
        assert!(
            (actual - expected).abs() < 4e-5,
            "{time}: {actual} vs {expected}"
        );
    }
    assert_eq!(point.analysis().monodromy.len(), 1);
    assert!((point.analysis().monodromy[0][0] - (-5.0_f64).exp()).abs() < 4e-5);
}

#[test]
fn bsim3_pss_coupled_native_charge_matches_hb_and_resumes() {
    for (kind, polarity, capmod, nqs, method) in [
        ("NMOS", 1.0, 0, 0, IntegrationMethod::Trapezoidal),
        ("PMOS", -1.0, 3, 0, IntegrationMethod::Gear2),
        ("NMOS", 1.0, 2, 1, IntegrationMethod::Trapezoidal),
        ("PMOS", -1.0, 3, 1, IntegrationMethod::Gear2),
    ] {
        let storage = if nqs == 1 && polarity > 0.0 {
            "CGDO=0 CGSO=0 CGBO=0 CF=0 CGDL=0 CGSL=0 CJ=0 CJSW=0 CJSWG=0"
        } else {
            "CGDO=7.9e-10 CGSO=6.3e-10 CJ=9.5e-4 CJSW=2.4e-10"
        };
        let netlist = Netlist::parse(&format!(
            "BSIM3 PSS\nVDD supply 0 {}\nVIN in 0 SIN({} {} 1G)\nRD supply out 500\nRG in gate 100\nRS source 0 10\nM1 out gate source 0 mm L=.18u W=10u AD=4p AS=4p PD=20u PS=20u M=2 OFF\n.model mm {kind}(LEVEL=49 TOX=4.1n VTH0={} U0=270 K1=.59 K2=.0026 CAPMOD={capmod} NQSMOD={nqs} RSH=10 {storage})\n.options hbint tahb=0\n.end\n",
            polarity * 1.8, polarity * 0.7, polarity * 0.01, polarity * 0.37,
        )).unwrap();
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            ..Default::default()
        });
        let hb = engine
            .run_hb(
                &netlist,
                HbConfig::new(1e9).with_harmonics(5).with_tolerance(1e-10),
            )
            .unwrap();
        let mut config = PssConfig::new(1e9)
            .with_points_per_period(128)
            .with_tstab_periods(0);
        config.integration_method = Some(method);
        let (analysis, state) = engine
            .run_pss_with_continuation_state(&netlist, config)
            .unwrap_or_else(|error| panic!("{kind} CAPMOD={capmod} NQSMOD={nqs}: {error}"));
        if nqs == 1 && polarity > 0.0 {
            assert_eq!(
                analysis.monodromy.len(),
                1,
                "only the stored channel charge is dynamic"
            );
        }
        for (name, values) in analysis
            .result
            .node_names
            .iter()
            .zip(&analysis.result.waveforms)
        {
            let spectrum = hb
                .result
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
            for (&time, &actual) in analysis.result.time.iter().zip(&values.values) {
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
                    (actual - expected).abs() < 2e-7 + 0.015 * amplitude,
                    "{kind} {name} at {time}: {actual} vs {expected}"
                );
            }
        }
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&netlist, &state, 0.1e-9, 1e-12)
            .unwrap();
        assert!(continued.time.len() > 2);
        let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
        assert!(checkpoint.capability().is_resumable());
        engine
            .run_tran_resume(&netlist, &checkpoint, 0.11e-9, 1e-12)
            .unwrap();
    }
}

//! Classic MOS shooting uses native Meyer or legacy BSIM charge laws.
use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig, TransientCheckpoint};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

#[test]
fn classic_mos_pss_overlap_storage_matches_rc_waveform_and_physical_mode() {
    let netlist = Netlist::parse(
        "Classic MOS overlap RC\nVIN in 0 SIN(0 .1 1meg)\nR1 in out 1k\nM1 0 out 0 0 mm L=1u W=10u M=2 OFF\n.model mm NMOS LEVEL=1 VTO=3 TOX=1 CGSO=1e-5 CGDO=0 CGBO=0\n.end\n"
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

fn interpolate(time: &[f64], values: &[f64], at: f64) -> f64 {
    let hi = time.partition_point(|t| *t < at).clamp(1, time.len() - 1);
    let lo = hi - 1;
    values[lo] + (values[hi] - values[lo]) * ((at - time[lo]) / (time[hi] - time[lo]))
}

#[test]
fn classic_mos_pss_native_models_match_transient_and_resume() {
    use rspice_core::engine::SpiceDialect;
    for (level, dialect) in [
        (1, SpiceDialect::Ngspice),
        (2, SpiceDialect::Ngspice),
        (3, SpiceDialect::Ngspice),
        (4, SpiceDialect::Ngspice),
        (5, SpiceDialect::Ngspice),
        (6, SpiceDialect::Ngspice),
        (9, SpiceDialect::Ngspice),
        (1, SpiceDialect::Xyce),
        (3, SpiceDialect::Xyce),
    ] {
        let polarity = if level % 2 == 0 { -1.0 } else { 1.0 };
        let kind = if polarity > 0.0 { "NMOS" } else { "PMOS" };
        let method = if level % 2 == 0 {
            IntegrationMethod::Gear2
        } else {
            IntegrationMethod::Trapezoidal
        };
        let model = if level == 4 || level == 5 {
            "VFB=-0.8 PHI=0.7 K1=0.5 TOX=0.02".to_string()
        } else {
            format!("VTO={} KP=50u GAMMA=0.5 PHI=0.7 TOX=20n", polarity * 0.6)
        };
        let terminals = if level == 3 {
            "source gate out 0"
        } else {
            "out gate source 0"
        };
        let netlist = Netlist::parse(&format!(
            "Classic MOS shooting
VDD supply 0 {}
VIN in 0 SIN({} {} 100meg)
RD supply out 500
RG in gate 10000
RS source 0 10
M1 {terminals} mm L=1u W=10u AD=4p AS=5p PD=20u PS=22u M=2 OFF
.model mm {kind} LEVEL={level} {model} CGSO=1e-8 CGDO=2e-8 CGBO=1e-8 CJ=0.001 CJSW=1e-10 RSH=10
.options TEMP=60
.end
",
            polarity * 1.8,
            polarity * 0.9,
            polarity * 0.15
        ))
        .unwrap();
        let engine = Engine::new(SimulationConfig {
            integration_method: method,
            spice_dialect: dialect,
            ..Default::default()
        });
        let mut config = PssConfig::new(1e8)
            .with_points_per_period(128)
            .with_tstab_periods(0);
        config.integration_method = Some(method);
        let label = format!("{dialect:?} level {level} {kind} {method:?}");
        let (analysis, state) = engine
            .run_pss_with_continuation_state(&netlist, config)
            .unwrap_or_else(|error| panic!("{label}: {error}"));
        let reference = engine.run_tran(&netlist, 200e-9, 10e-9 / 512.0).unwrap();
        let result = &analysis.result;
        for (name, wave) in result.node_names.iter().zip(&result.waveforms) {
            let reference_wave = reference.try_voltage_waveform_named(name).unwrap();
            for (&time, &actual) in result.time.iter().zip(&wave.values) {
                let expected = interpolate(&reference.time, &reference_wave, 190e-9 + time);
                assert!(
                    (actual - expected).abs() < 4e-4,
                    "{label} {name} at {time}: {actual} vs {expected}"
                );
            }
        }
        let (continued, checkpoint) = engine
            .run_tran_from_pss_state(&netlist, &state, 10e-9, 10e-9 / 512.0)
            .unwrap();
        for name in ["gate", "out"] {
            let actual = continued.try_voltage_waveform_named(name).unwrap();
            let reference_wave = reference.try_voltage_waveform_named(name).unwrap();
            for (&time, &value) in continued.time.iter().zip(actual) {
                let expected = interpolate(&reference.time, &reference_wave, 190e-9 + time);
                assert!(
                    (value - expected).abs() < 4e-4,
                    "{label} handoff {name} at {time}: {value} vs {expected}"
                );
            }
        }
        let checkpoint = TransientCheckpoint::from_text(&checkpoint.to_text()).unwrap();
        engine
            .run_tran_resume(&netlist, &checkpoint, 10.1e-9, 10e-9 / 512.0)
            .unwrap();
    }
}

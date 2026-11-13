//! Native VBIC electrical, thermal and excess-phase carrier state.
use num_complex::Complex64;
use rspice_core::analysis::HbConfig;
use rspice_core::config::{ConvergenceConfig, SpiceDialect};
use rspice_core::engine::{Engine, SimulationConfig, TransientCheckpoint};
use rspice_core::numerics::integration::IntegrationMethod;
use rspice_core::{Netlist, NoAbort};
use std::f64::consts::TAU;

fn engine(dialect: SpiceDialect) -> Engine {
    Engine::new(SimulationConfig {
        integration_method: IntegrationMethod::Gear2,
        transient_nonlinear_reltol: Some(1e-8),
        transient_nonlinear_rhstol: Some(1e-13),
        convergence_config: ConvergenceConfig::default()
            .with_voltage_tolerances(1e-8, 1e-11)
            .with_residual_reltol(1e-8),
        ..SimulationConfig::default().with_spice_dialect(dialect)
    })
}

#[test]
fn envelope_vbic_lead_currents_match_rc_charge_on_all_native_levels() {
    for (level, kind, dialect, substrate) in [
        (4, "PNP", SpiceDialect::Ngspice, "0"),
        (11, "NPN", SpiceDialect::Xyce, ""),
        (12, "PNP", SpiceDialect::Xyce, "0"),
    ] {
        let deck = Netlist::parse(&format!(
            "VBIC RC carrier\nVdrive drive 0 SIN(0 .1 1meg)\nRdrive drive b 1k\n\
            VC c 0 0\nVE e 0 0\nQ1 c b e {substrate} qm OFF\n\
            .model qm {kind}(LEVEL={level} IS=1e-40 IBEI=1e-40 IBCI=1e-40 \
            CJE=100p CJC=20p MJE=0 MJC=0 TF=0 TR=0 CBEO=30p CBCO=9p \
            RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 CJEP=0 CJCP=0 CCSO=0 QCO=0 GAMM=0 ISP=0)\n\
            .print tran IC(Q1) IB(Q1) IE(Q1)\n.options hbint tahb=0\n.end\n"
        ))
        .unwrap();
        let mut config = HbConfig::new(1e6).with_harmonics(3);
        config.tolerance = 1e-9;
        config.abstol = 1e-14;
        let result = engine(dialect)
            .run_envelope_with_abort(&deck, config, &[], 0.3e-6, 0.5e-9, &NoAbort)
            .unwrap_or_else(|error| panic!("LEVEL={level}: {error}"));
        let transient = result.continued_transient();
        let jomega = Complex64::new(0.0, TAU * 1e6);
        let base = Complex64::new(0.0, -0.1) / (1.0 + jomega * 1e3 * 159e-12);
        let currents = [-29e-12, 159e-12, -130e-12].map(|c| jomega * c * base);
        for (i, (parameter, source)) in [("IC", "VC"), ("IB", "Vdrive"), ("IE", "VE")]
            .into_iter()
            .enumerate()
        {
            let measured = transient
                .try_device_op_waveform_named("Q1", parameter)
                .unwrap();
            let supplied = transient.try_branch_current_waveform_named(source).unwrap();
            assert!(
                (measured[0] - currents[i].re).abs() < 1e-9,
                "LEVEL={level} initial {parameter}: {} vs {}",
                measured[0],
                currents[i].re
            );
            for ((&time, &current), &supply) in transient.time.iter().zip(measured).zip(supplied) {
                let expected = (currents[i] * Complex64::from_polar(1.0, TAU * 1e6 * time)).re;
                assert!(
                    (current - expected).abs() < 5e-8,
                    "LEVEL={level} {parameter} at {time}: {current} vs {expected}"
                );
                assert!(
                    (current + supply).abs() < 2e-9,
                    "LEVEL={level} {parameter} KCL at {time}: {current} + {supply}"
                );
            }
        }
    }
}

#[test]
fn envelope_vbic_retains_self_heating_and_delay_against_ngspice_orbit() {
    // ngspice 46, identical NPN deck: .tran .00625n 20u 19u .00625n,
    // RELTOL=1e-7, ABSTOL=1e-15, CHGTOL=1e-20, GMIN=0, TEMP=TNOM=27.
    // Settled samples [rise K, XF1 A, XF2 A, I(VC), I(VB)] at eight phases.
    // Same independently qualified reference as pss_continuation.rs.
    let reference = [
        [
            1.041075569976005,
            6.148529746246692e-4,
            5.512146980012821e-4,
            -5.501029480052405e-4,
            -4.613058613910093e-5,
        ],
        [
            1.066517966765304,
            1.030781687499789e-3,
            9.071911850267481e-4,
            -9.059719097573484e-4,
            -6.058156544801507e-5,
        ],
        [
            1.160116913345152,
            1.52598429312555e-3,
            1.403160675769665e-3,
            -1.402444715621389e-3,
            -3.148194306449638e-5,
        ],
        [
            1.262103901382114,
            1.666931504525963e-3,
            1.66694458706596e-3,
            -1.667458099232046e-3,
            2.758779477732575e-5,
        ],
        [
            1.289809611401592,
            1.24936325490962e-3,
            1.383995190930887e-3,
            -1.385546378654575e-3,
            4.088935445331497e-5,
        ],
        [
            1.237623085814133,
            7.230633244770615e-4,
            8.50230695212558e-4,
            -8.514968594782904e-4,
            1.688881392015541e-5,
        ],
        [
            1.155258758854126,
            4.552662403430866e-4,
            5.094134876644744e-4,
            -5.09712259017482e-4,
            -2.633663883801036e-6,
        ],
        [
            1.080244740037929,
            4.295707356929894e-4,
            4.236695847765118e-4,
            -4.230871653264675e-4,
            -2.176034333311161e-5,
        ],
    ];
    let deck = Netlist::parse(
        "VBIC thermal delay carrier\nVC c 0 1.2\nVB b 0 SIN(.65 .02 1meg)\n\
        Q1 c b 0 0 th qm OFF\n\
        .model qm NPN(LEVEL=4 IS=1e-14 IBEI=1e-16 IBCI=1e-16 RCX=10 RCI=20 \
        RBX=10 RBI=40 RE=1 RBP=10 RS=1 CJE=10p CJC=5p CJEP=3p CJCP=2p \
        TF=10n TR=2n QCO=10f GAMM=1e-9 ISP=1e-16 WBE=.8 SELFT=1 RTH=1000 CTH=1n TD=100n)\n\
        .options GMIN=0\n.options hbint tahb=0\n.temp 27\n\
        .save all\n.print tran IC(Q1) IB(Q1) I(VC) I(VB) V(th)\n.end\n",
    )
    .unwrap();
    let engine = engine(SpiceDialect::Ngspice);
    let mut config = HbConfig::new(1e6)
        .with_harmonics(12)
        .with_collocation_points(65);
    config.tolerance = 1e-9;
    config.abstol = 1e-14;
    let result = engine
        .run_envelope_with_abort(&deck, config, &[], 1e-6, 0.5e-9, &NoAbort)
        .unwrap();
    let continued = result.continued_transient();
    let traces = [
        continued.try_voltage_waveform_named("th").unwrap(),
        continued
            .try_voltage_waveform_named("Q1.__xf1.internal")
            .unwrap(),
        continued
            .try_voltage_waveform_named("Q1.__xf2.internal")
            .unwrap(),
        continued.try_branch_current_waveform_named("VC").unwrap(),
        continued.try_branch_current_waveform_named("VB").unwrap(),
    ];
    for (phase, expected) in reference.iter().enumerate() {
        let time = phase as f64 / 8e6;
        let hi = continued.time.partition_point(|&t| t < time);
        let lo = hi.saturating_sub(1);
        let fraction = if lo == hi {
            0.0
        } else {
            (time - continued.time[lo]) / (continued.time[hi] - continued.time[lo])
        };
        for (column, tolerance) in [1e-4, 3e-7, 3e-7, 3e-7, 5e-8].into_iter().enumerate() {
            let actual = traces[column][lo] + fraction * (traces[column][hi] - traces[column][lo]);
            assert!(
                (actual - expected[column]).abs() < tolerance,
                "phase={phase} column={column}: {actual} vs {}",
                expected[column]
            );
        }
    }
    for (parameter, source) in [("IC", "VC"), ("IB", "VB")] {
        let device = continued
            .try_device_op_waveform_named("Q1", parameter)
            .unwrap();
        let supplied = continued.try_branch_current_waveform_named(source).unwrap();
        assert!(
            device
                .iter()
                .zip(supplied)
                .all(|(a, b)| (a + b).abs() < 2e-9)
        );
    }
    let checkpoint = TransientCheckpoint::from_text(&result.final_checkpoint().to_text()).unwrap();
    let (resumed, _) = engine
        .run_tran_resume(&deck, &checkpoint, 1.125e-6, 0.5e-9)
        .unwrap();
    let end = resumed.time.len() - 1;
    let thermal = resumed.try_voltage_waveform_named("th").unwrap()[end];
    assert!((thermal - reference[1][0]).abs() < 1e-4);
}

#[test]
fn envelope_vbic_collapsed_delay_leads_match_two_pole_transport() {
    // Prescribe an exactly sinusoidal forward transport current through its
    // inverse exponential law. The independent two-state delay transfer is
    // 1 / (1 + s*TD + s*s*TD*TD/3).
    for (kind, p) in [("NPN", 1.0), ("PNP", -1.0)] {
        let thermal = 300.15 * 1.380662e-23 / 1.602189e-19;
        let deck = Netlist::parse(&format!(
            "VBIC collapsed delay\nVC c 0 {p}\nVE e 0 0\n\
            Bbase b 0 V={p}*{thermal}*ln(1+(1m+.2m*sin(2*pi*1meg*time))/1e-14)\n\
            Q1 c b e 0 qm OFF\n\
            .model qm {kind}(LEVEL=4 IS=1e-14 IBEI=1e-16 IBCI=0 \
            RCX=0 RCI=0 RBX=0 RBI=0 RE=0 RBP=0 RS=0 \
            CJE=0 CJC=0 CJEP=0 CJCP=0 TF=0 TR=0 TD=100n)\n\
            .temp 27\n.options tnom=27 gmin=0\n.options hbint tahb=0\n\
            .print tran IC(Q1) IB(Q1) IE(Q1) I(VC) I(VE) I(Bbase)\n.end\n"
        ))
        .unwrap();
        let mut config = HbConfig::new(1e6)
            .with_harmonics(10)
            .with_collocation_points(65);
        config.tolerance = 1e-9;
        config.abstol = 1e-14;
        let result = engine(SpiceDialect::Ngspice)
            .run_envelope_with_abort(&deck, config, &[], 0.3e-6, 0.5e-9, &NoAbort)
            .unwrap();
        let sdt = Complex64::new(0.0, TAU * 1e6 * 100e-9);
        let collector = Complex64::new(0.0, -0.2e-3) / (1.0 + sdt + sdt * sdt / 3.0);
        let base = Complex64::new(0.0, -0.2e-5);
        let spectra = [collector, base, -collector - base];
        let dc = [1e-3, 1e-5, -1.01e-3];
        let transient = result.continued_transient();
        for (i, (parameter, source)) in [("IC", "VC"), ("IB", "Bbase"), ("IE", "VE")]
            .into_iter()
            .enumerate()
        {
            let current = transient
                .try_device_op_waveform_named("Q1", parameter)
                .unwrap();
            let supply = transient.try_branch_current_waveform_named(source).unwrap();
            for ((&time, &value), &source_value) in transient.time.iter().zip(current).zip(supply) {
                let expected =
                    p * (dc[i] + (spectra[i] * Complex64::from_polar(1.0, TAU * 1e6 * time)).re);
                assert!(
                    (value - expected).abs() < 5e-8,
                    "{kind} {parameter} at {time}: {value} vs {expected}"
                );
                assert!(
                    (value + source_value).abs() < 2e-9,
                    "{kind} {parameter} KCL at {time}: {value} + {source_value}"
                );
            }
        }
    }
}

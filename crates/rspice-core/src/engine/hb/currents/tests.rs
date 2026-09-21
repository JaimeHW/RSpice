use super::*;

fn run(source: &str, frequency: Value) -> HbAnalysisResult {
    let deck = Netlist::parse(source).unwrap();
    let engine = Engine::new(crate::SimulationConfig {
        convergence_config: crate::config::ConvergenceConfig {
            gmin_target: 0.0,
            junction_gmin_target: 0.0,
            voltage_reltol: 1e-9,
            current_abstol: 1e-13,
            ..Default::default()
        },
        ..Default::default()
    });
    let result = engine
        .run_hb(
            &deck,
            HbConfig::new(frequency)
                .with_harmonics(5)
                .with_tolerance(1e-9),
        )
        .unwrap();
    assert!(result.converged);
    result
}

fn current(result: &HbAnalysisResult, name: &str, harmonic: usize) -> Complex64 {
    result
        .device_currents
        .iter()
        .find(|row| row.probe.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("missing {name}"))
        .coefficients[harmonic]
}
fn branch(result: &HbAnalysisResult, name: &str, harmonic: usize) -> Complex64 {
    result
        .result
        .mna_branch_currents
        .iter()
        .find(|row| row.device_name.eq_ignore_ascii_case(name))
        .unwrap()
        .coefficients[harmonic]
}
fn close(actual: Complex64, expected: Complex64) {
    assert!(
        (actual - expected).norm() < 2e-11 + expected.norm() * 2e-6,
        "{actual:?} != {expected:?}"
    );
}

#[test]
fn hb_device_current_linear_sources_and_resistors_preserve_signed_phasors() {
    let result = run(
        "HB currents\nV1 in 0 SIN(1 2 1k)\nR1 in out 1k AC=9k\nR2 out 0 1k\nI1 0 out SIN(0 .001 1k 0 0 90)\nG1 other 0 out 0 .001\nR3 other 0 1k\nF1 mirror 0 V1 2\nR4 mirror 0 1k\n.end\n",
        1e3,
    );
    for harmonic in 0..=5 {
        close(
            current(&result, "I(R1)", harmonic),
            -branch(&result, "V1", harmonic),
        );
        close(
            current(&result, "I(R1)", harmonic) + current(&result, "I(I1)", harmonic),
            current(&result, "I(R2)", harmonic),
        );
        close(
            current(&result, "I(G1)", harmonic),
            -current(&result, "I(R3)", harmonic),
        );
        close(
            current(&result, "I(F1)", harmonic),
            branch(&result, "V1", harmonic) * 2.0,
        );
        close(
            current(&result, "I(F1)", harmonic),
            -current(&result, "I(R4)", harmonic),
        );
    }
    close(current(&result, "I(I1)", 1), Complex64::new(0.001, 0.0));
    close(current(&result, "I(R1)", 0), Complex64::new(0.0005, 0.0));
}

#[test]
fn hb_device_current_nonlinear_charge_and_tied_mos_leads_match_kcl() {
    let diode = run(
        "Diode lead current\nV1 in 0 SIN(5 .05 1meg)\nR1 in out 1k\nD1 0 out dm\n.model dm D IS=1e-16 CJ0=10p VJ=.7 M=.5\n.end\n",
        1e6,
    );
    assert!(current(&diode, "I(D1)", 1).norm() > 1e-7);
    for h in 0..=5 {
        close(current(&diode, "I(D1)", h), branch(&diode, "V1", h));
    }

    let mos = run(
        "Tied MOS leads\nVG g 0 SIN(-1 .001 8meg)\nM1 0 g 0 0 nm L=1u W=10u M=3\n.model nm NMOS LEVEL=1 VTO=.7 KP=2e-5 TOX=20n CGSO=1e-10 CGDO=1e-10 CGBO=1e-11 PHI=.7\n.end\n",
        8e6,
    );
    let capacitance =
        3.0 * (3.9 * 8.854214871e-12 / 20e-9 * 10e-6 * 1e-6 + 2.0 * 1e-10 * 10e-6 + 1e-11 * 1e-6);
    close(
        current(&mos, "@M1[ig]", 1),
        Complex64::new(std::f64::consts::TAU * 8e6 * capacitance * 0.001, 0.0),
    );
    close(
        current(&mos, "@M1[id]", 1),
        Complex64::new(
            -std::f64::consts::TAU * 8e6 * 3.0 * 1e-10 * 10e-6 * 0.001,
            0.0,
        ),
    );
    for h in 0..=5 {
        close(current(&mos, "@M1[ig]", h), -branch(&mos, "VG", h));
        close(
            ["d", "g", "s", "b"]
                .iter()
                .map(|lead| current(&mos, &format!("@M1[i{lead}]"), h))
                .sum(),
            Complex64::ZERO,
        );
        close(current(&mos, "I(M1)", h), current(&mos, "@M1[id]", h));
    }
}

#[test]
fn hb_device_current_bjt_authored_leads_include_charge_and_series_networks() {
    for model in [
        "NPN IS=1e-15 BF=100 RB=100 RC=10 RE=1 CJE=10p CJC=5p XCJC=.3 TF=10n TR=2n",
        "NPN LEVEL=4 IS=1e-15 IBEI=1e-17 IBCI=1e-17 IBEIP=0 IBENP=0 IBCIP=0 IBCNP=0 ISP=0 RCX=10 RCI=20 RBX=10 RBI=40 RE=1 CJE=10p CJC=5p TF=10n TR=2n TD=10n",
    ] {
        let result = run(
            &format!(
                "BJT lead current\nVC c 0 2\nVB b 0 SIN(.65 .005 1meg)\nVE e 0 0\nVS s 0 0\nQ1 c b e s qm\n.model qm {model}\n.end\n"
            ),
            1e6,
        );
        for h in 0..=5 {
            for (lead, source) in [("c", "VC"), ("b", "VB"), ("e", "VE"), ("s", "VS")] {
                close(
                    current(&result, &format!("@Q1[i{lead}]"), h),
                    -branch(&result, source, h),
                );
            }
        }
    }
}

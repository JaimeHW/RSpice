//! Exact descriptor constraints must not become spurious finite fast modes.
use rspice_core::analysis::pole_zero::{
    Matrix, PoleZeroAnalysisError, PoleZeroAnalyzer, PoleZeroConfig,
};
use rspice_core::{Engine, Netlist, NoAbort};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unexported_behavioral_dynamics_cannot_claim_a_qualified_spectrum() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::engine::{SimulationConfig, SpiceDialect};
    use rspice_core::netlist::NetlistParseOptions;
    let engine = Engine::new_with_resolved_config(
        SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce),
    );
    let mut incorrectly_admitted = Vec::new();
    for device in [
        "B1 out 0 I={1m*(1+FREQ)*V(out)}",
        "B1 out 0 I={1m*SDT(V(out))}",
        "B1 out 0 I={1m*(1+TIME)*V(out)}",
        "B1 aux 0 V={(1+HERTZ)*V(out)}\nR2 aux out 1k",
        "B1 aux 0 V={SDT(V(out))}\nR2 aux out 1k",
        "C1 out 0 C={1u*(1+FREQ)}",
        "C1 out 0 C={1u*(1+SDT(V(out)))}",
        "C1 out 0 C={1u*(1+TIME)}",
        ".FUNC gain(x) {(1+FREQ)*x}\nB1 out 0 I={1m*gain(V(out))}",
        ".FUNC rate(x) {SDT(x)}\nB1 out 0 I={1m*rate(V(out))}",
    ] {
        let netlist = Netlist::parse_with_options(
            &format!("Unexported state\nR1 out 0 1k\n{device}\n.pz out 0 out 0 cur pol\n.end\n"),
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .unwrap();
        match engine.run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort) {
            Err(rspice_core::SimulationError::UnsupportedCapability(_)) => {}
            result => incorrectly_admitted.push(format!("{device}: {result:?}")),
        }
    }
    assert!(
        incorrectly_admitted.is_empty(),
        "{}",
        incorrectly_admitted.join("\n")
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn descriptor_admission_keeps_static_laws_and_independent_forcing() {
    for source in [
        "B1 out 0 I={1m*V(out)}",
        "B1 out 0 I={sin(TIME)+FREQ}",
        "V1 FREQ 0 0\nB1 out 0 I={1m*V(FREQ)}",
    ] {
        let netlist = Netlist::parse(&format!(
            "Memoryless descriptor\nR1 out 0 1k\nC1 out 0 1u\n{source}\n.pz out 0 out 0 cur pol\n.end\n"
        )).unwrap();
        let result = Engine::default()
            .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
            .unwrap();
        assert_eq!(result.poles.len(), 1, "{source}: {result:?}");
        let expected = if source == "B1 out 0 I={1m*V(out)}" {
            -2000.0
        } else {
            -1000.0
        };
        assert!((result.poles[0].re / expected - 1.0).abs() < 1e-10);
        assert!(result.pole_evidence.is_qualified());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sparse_certificates_retain_the_original_algebraic_multiplicity() {
    let netlist = Netlist::parse(
        "RC descriptor\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.pz in 0 out 0 vol pz\n.end\n",
    ).unwrap();
    let result = Engine::default()
        .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
        .unwrap();
    let poles = result.pole_evidence.certificate().unwrap();
    let zeros = result.zero_evidence.certificate().unwrap();
    // Two node voltages and one ideal-source branch; the zero pencil adds
    // one transfer constraint. Eliminating them does not erase multiplicity.
    assert_eq!((poles.problem_order, poles.infinite_count), (3, 2));
    assert_eq!((zeros.problem_order, zeros.infinite_count), (4, 4));
    assert_eq!(result.poles.len(), 1);
    assert!((result.poles[0].re + 1000.0).abs() < 1e-10);
    assert!(result.zeros.is_empty());
    assert!(result.has_consistent_root_evidence());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn expression_capacitor_poles_use_the_accepted_bias_and_control_derivatives() {
    use rspice_core::config::ExpressionDialect;
    use rspice_core::engine::{SimulationConfig, SpiceDialect};
    use rspice_core::netlist::NetlistParseOptions;

    for (devices, expected) in [
        ("C1 out 0 C={1u*(1+V(out))}\n", -500.0),
        (
            "Econtrol ctrl 0 out 0 2\nC1 out 0 C={1u*(1+V(ctrl))}\n",
            -200.0,
        ),
    ] {
        let netlist = Netlist::parse_with_options(
            &format!(
                "Capacitor bias\nI1 0 out DC 1m AC 1\nR1 out 0 1k\n{devices}.pz out 0 out 0 cur pol\n.end\n"
            ),
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .unwrap();
        let mut config = SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        config.convergence_config.gmin_target = 0.0;
        let result = Engine::new_with_resolved_config(config)
            .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
            .unwrap();
        // Vout=1. Terminal control gives C(op)=2u. External control adds
        // Vout*dC/dVctrl*dVctrl/dVout=2u to C(op)=3u, for a total 5u.
        assert_eq!(result.poles.len(), 1, "{result:?}");
        assert!(
            (result.poles[0].re / expected - 1.0).abs() < 1e-10 && result.poles[0].im == 0.0,
            "expected {expected}, got {result:?}"
        );
        assert!(result.pole_evidence.is_qualified());
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn schur_cancellation_does_not_erase_or_shift_a_natural_pole() {
    // Exact binary64-rational oracle for -(g11*g22-g12*g21)/(g22*C),
    // with g12=0.1, g21=0.2, g22=0.3, C=1e-20. Floating subtraction
    // after the algebraic solve instead gives 0, +1387.779, -1387.779.
    for (g11, expected) in [
        (0.06666666666666668, -308.395284618099),
        (0.06666666666666667, 1079.3834961633468),
        (0.0666666666666667, -1696.1740653995448),
    ] {
        for exponent in [-100, 0, 100] {
            let scale = 2.0_f64.powi(exponent);
            for algebraic_first in [false, true] {
                let capacitor = format!("C1 n1 0 {:.17e}\n", 1e-20 * scale);
                let algebraic = format!("G22 n2 0 n2 0 {:.17e}\n", 0.3 * scale);
                let source = format!(
                    "Schur cancellation\n{}{}G11 n1 0 n1 0 {:.17e}\nG12 n1 0 n2 0 {:.17e}\nG21 n2 0 n1 0 {:.17e}\n.pz n1 0 n1 0 cur pol\n.end\n",
                    if algebraic_first {
                        &algebraic
                    } else {
                        &capacitor
                    },
                    if algebraic_first {
                        &capacitor
                    } else {
                        &algebraic
                    },
                    g11 * scale,
                    0.1 * scale,
                    0.2 * scale,
                );
                let netlist = Netlist::parse(&source).unwrap();
                let result = Engine::default()
                    .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
                    .unwrap();
                assert_eq!(result.poles.len(), 1, "{result:?}");
                let pole = result.poles[0];
                assert!(
                    (pole.re / expected - 1.0).abs() < 1e-12 && pole.im == 0.0,
                    "G11={g11}, scale=2^{exponent}, algebraic_first={algebraic_first}: expected {expected}, got {result:?}"
                );
                assert_eq!(result.is_stable(), expected < 0.0);
                assert!(result.pole_evidence.is_qualified());
                let natural = Engine::default().run_pole_spectrum(&netlist).unwrap();
                assert_eq!(natural.poles.len(), 1);
                assert!((natural.poles[0].re / expected - 1.0).abs() < 1e-12);
                assert!(natural.evidence.is_qualified());
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn differential_rc_has_one_stable_pole_and_no_finite_zero() {
    let netlist = Netlist::parse(
        "Hierarchy\n.subckt filter p n\nR1 p mid 1k\nC1 mid n 1u\n.ends\nX1 in ref filter\nV1 in ref 1\nRref ref 0 1k\n.pz in ref x1.mid ref vol pz\n.end\n",
    ).unwrap();
    let result = Engine::default()
        .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
        .unwrap();
    // H(s) = 1 / (1 + 0.001*s), independently of the reference shunt.
    assert_eq!(result.poles.len(), 1, "{result:?}");
    assert!((result.poles[0].re + 1000.0).abs() < 1e-7);
    assert!(result.poles[0].im.abs() < 1e-12);
    assert!(result.zeros.is_empty(), "{result:?}");
    assert!(result.is_stable());
    assert!(result.has_consistent_root_evidence());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn singular_floating_capacitor_has_its_analytical_pole_and_zero() {
    let result = PoleZeroAnalyzer::new(
        Matrix::from_dense(vec![vec![0.001, 0.0], vec![0.0, 0.001]]),
        Matrix::from_dense(vec![vec![1e-6, -1e-6], vec![-1e-6, 1e-6]]),
    )
    .analyze(&PoleZeroConfig::poles_and_zeros(0, 0))
    .unwrap();
    // Z11(s) = (0.001 + s*1e-6) / (1e-6 + s*2e-9).
    assert_eq!(result.poles.len(), 1, "{result:?}");
    assert_eq!(result.zeros.len(), 1, "{result:?}");
    assert!((result.poles[0].re + 500.0).abs() < 1e-8);
    assert!((result.zeros[0].re + 1000.0).abs() < 1e-8);
    let certificate = result.pole_evidence.certificate().unwrap();
    assert_eq!(certificate.problem_order, 2);
    assert_eq!(certificate.infinite_count, 1);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn descriptor_rank_keeps_a_genuine_fast_mode_under_row_and_column_permutations() {
    let epsilon = 2.0_f64.powi(-80);
    let c = [
        [1.0, -1.0, 0.0, 0.0],
        [-1.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, epsilon, -epsilon],
        [0.0, 0.0, -epsilon, epsilon],
    ];
    for rows in [[0, 1, 2, 3], [3, 2, 1, 0], [1, 3, 0, 2]] {
        for columns in [[0, 1, 2, 3], [3, 2, 1, 0], [2, 0, 3, 1]] {
            let g = rows
                .iter()
                .map(|r| columns.iter().map(|s| f64::from(r == s)).collect())
                .collect();
            let c = rows
                .iter()
                .map(|r| columns.iter().map(|s| c[*r][*s]).collect())
                .collect();
            let mut config = PoleZeroConfig::poles_and_zeros(0, 0);
            config.compute_zeros = false;
            let result = PoleZeroAnalyzer::new(Matrix::from_dense(g), Matrix::from_dense(c))
                .analyze(&config)
                .unwrap();
            assert_eq!(result.poles.len(), 2, "{rows:?} / {columns:?}: {result:?}");
            for (root, expected) in result.poles.iter().zip([-0.5, -0.5 / epsilon]) {
                assert!(
                    (root.re / expected - 1.0).abs() < 1e-12 && root.im == 0.0,
                    "{result:?}"
                );
            }
            assert_eq!(
                result.pole_evidence.certificate().unwrap().infinite_count,
                2
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn higher_index_descriptor_retains_finite_zeros_and_infinite_pole_multiplicity() {
    let result = PoleZeroAnalyzer::new(
        Matrix::identity(3),
        Matrix::from_dense(vec![vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0], vec![0.0; 3]]),
    )
    .analyze(&PoleZeroConfig::poles_and_zeros(2, 0))
    .unwrap();
    // (I+s*N)^-1 = I-s*N+s^2*N^2, so H02(s)=s^2.
    assert!(result.poles.is_empty(), "{result:?}");
    assert_eq!(
        result.pole_evidence.certificate().unwrap().infinite_count,
        3
    );
    assert_eq!(result.zeros.len(), 2, "{result:?}");
    assert!(result.zeros.iter().all(|root| root.norm() == 0.0));
    assert_eq!(
        result.zero_evidence.certificate().unwrap().infinite_count,
        2
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn exact_reduction_rejects_an_irregular_pencil() {
    let matrix = || Matrix::from_dense(vec![vec![1.0, 1.0], vec![2.0, 2.0]]);
    let result =
        PoleZeroAnalyzer::new(matrix(), matrix()).analyze(&PoleZeroConfig::poles_and_zeros(0, 0));
    assert!(
        matches!(
            result,
            Err(PoleZeroAnalysisError::IrregularDescriptor { .. })
        ),
        "{result:?}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn descriptor_preparation_obeys_inner_cancellation() {
    use rspice_core::AbortSignal;
    use std::sync::atomic::{AtomicUsize, Ordering};
    struct StopAfter(AtomicUsize);
    impl AbortSignal for StopAfter {
        fn is_aborted(&self) -> bool {
            self.0.fetch_add(1, Ordering::Relaxed) + 1 >= 25
        }
    }
    let abort = StopAfter(AtomicUsize::new(0));
    let result = PoleZeroAnalyzer::new(
        Matrix::identity(2),
        Matrix::from_dense(vec![vec![1.0, -1.0], vec![-1.0, 1.0]]),
    )
    .analyze_with_abort(&PoleZeroConfig::poles_and_zeros(0, 0), &abort);
    assert!(
        matches!(result, Err(PoleZeroAnalysisError::Aborted)),
        "{result:?}"
    );
    assert_eq!(abort.0.load(Ordering::Relaxed), 25);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn engine_keeps_exact_workspace_failure_typed() {
    use rspice_core::{ResourceKind, ResourceLimits, SimulationConfig, SimulationError};
    let netlist = Netlist::parse(
        "Bounded\nV1 in ref 1\nR1 in out 1k\nC1 out ref 1u\nRref ref 0 1k\n.pz in ref out ref vol pz\n.end\n",
    ).unwrap();
    let mut limits = ResourceLimits::default();
    limits.max_result_values = 512;
    let engine = Engine::new(SimulationConfig {
        resource_limits: limits,
        ..SimulationConfig::default()
    });
    let result = engine.run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort);
    let Err(SimulationError::ResourceLimit(error)) = result else {
        panic!("{result:?}");
    };
    assert_eq!(error.resource, ResourceKind::ResultValues);
    assert_eq!(error.limit, 512);
    // The admitted four-row floating matrices fit. The next exact coefficient
    // reservation is what exhausts the shared workspace allowance.
    assert_eq!(error.requested, 560);
}

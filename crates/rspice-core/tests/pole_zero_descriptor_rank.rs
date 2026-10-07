//! Exact descriptor constraints must not become spurious finite fast modes.
use rspice_core::analysis::pole_zero::{
    Matrix, PoleZeroAnalysisError, PoleZeroAnalyzer, PoleZeroConfig,
};
use rspice_core::{Engine, Netlist, NoAbort};

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

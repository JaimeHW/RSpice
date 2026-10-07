//! Analytical high-frequency limits, including hidden improper coordinates.
use rspice_core::analysis::pole_zero::{
    Matrix, PoleZeroAnalysisError, PoleZeroAnalyzer, PoleZeroConfig,
};
use rspice_core::execution::{
    AnalysisInstanceId, AnalysisKind, AnalysisResultDocument, ResultPayload,
};
use rspice_core::{Engine, Netlist, NoAbort};

fn finite_limit(source: &str, expected: f64) {
    let netlist = Netlist::parse(source).unwrap();
    let result = Engine::default()
        .run_pz_from_card_with_abort(&netlist, &netlist.analyses[0], &NoAbort)
        .unwrap();
    let actual = result
        .hf_gain
        .expect("the analytical high-frequency limit is finite");
    assert!(
        (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
        "{result:?}"
    );
    let document = AnalysisResultDocument::from_pole_zero(
        AnalysisInstanceId::new(AnalysisKind::PoleZero, 0),
        &result,
    )
    .unwrap()
    .build()
    .unwrap();
    let ResultPayload::PoleZero(payload) = document.payload() else {
        panic!("PZ");
    };
    assert_eq!(payload.high_frequency_gain, Some(actual));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn static_current_and_voltage_transfer_limits_retain_their_signed_gain() {
    finite_limit("R\nR1 out 0 1k\n.pz out 0 out 0 cur pz\n.end\n", 1000.0);
    for voltage in ["V1 in ref 1", "V1 ref in -1"] {
        finite_limit(
            &format!(
                "Divider\n{voltage}\nR1 in out 1k\nR2 out ref 2k\nRref ref 0 1k\n.pz in ref out ref vol pz\n.end\n"
            ),
            2.0 / 3.0,
        );
        finite_limit(
            &format!(
                "Reverse\n{voltage}\nR1 in out 1k\nR2 out ref 2k\nRref ref 0 1k\n.pz in ref ref out vol pz\n.end\n"
            ),
            -2.0 / 3.0,
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn floating_reference_lowpass_and_highpass_have_zero_and_unity_limits() {
    finite_limit(
        "LP\nV1 in ref 1\nR1 in out 1k\nC1 out ref 1u\nRref ref 0 1k\n.pz in ref out ref vol pz\n.end\n",
        0.0,
    );
    finite_limit(
        "HP\nV1 in ref 1\nC1 in out 1u\nR1 out ref 1k\nRref ref 0 1k\n.pz in ref out ref vol pz\n.end\n",
        1.0,
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn floating_capacitor_transimpedance_has_a_finite_nonzero_limit() {
    // Capacitor shorts the two equal resistors at infinity: 1k || 1k.
    finite_limit(
        "Floating C\nR1 a 0 1k\nR2 b 0 1k\nC1 a b 1u\n.pz a 0 a 0 cur pz\n.end\n",
        500.0,
    );
}

fn hidden_improper(delta: f64) -> (PoleZeroAnalyzer, PoleZeroConfig) {
    // x2=u, x0=-(1+s)*u, x1=-(1+delta)*s*u.
    // Therefore x0-x1=-1+delta*s, despite both internal signals growing.
    let analyzer = PoleZeroAnalyzer::new(
        Matrix::from_dense(vec![
            vec![1.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ]),
        Matrix::from_dense(vec![
            vec![0.0, 0.0, 1.0],
            vec![0.0, 0.0, 1.0 + delta],
            vec![0.0; 3],
        ]),
    );
    let mut config = PoleZeroConfig::poles_and_zeros(2, 0);
    config.output_neg = Some(1);
    (analyzer, config)
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn observed_improper_terms_cancel_exactly_before_classifying_the_limit() {
    let (analyzer, config) = hidden_improper(0.0);
    let result = analyzer.analyze(&config).unwrap();
    assert_eq!(result.hf_gain, Some(-1.0), "{result:?}");
    assert!(result.poles.is_empty() && result.zeros.is_empty());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_small_nonzero_improper_term_is_not_replaced_by_its_constant_part() {
    let (analyzer, config) = hidden_improper(f64::EPSILON);
    let result = analyzer.analyze(&config).unwrap();
    assert_eq!(
        result.hf_gain, None,
        "-1 + epsilon*s has no finite limit: {result:?}"
    );
    assert_eq!(result.zeros.len(), 1);
    assert!((result.zeros[0].re * f64::EPSILON - 1.0).abs() < 1e-12);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn higher_order_improper_transfer_has_no_finite_limit() {
    let result = PoleZeroAnalyzer::new(
        Matrix::identity(3),
        Matrix::from_dense(vec![vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0], vec![0.0; 3]]),
    )
    .analyze(&PoleZeroConfig::poles_and_zeros(2, 0))
    .unwrap();
    // H02(s)=s^2: neither a finite approximation nor zero is its limit.
    assert_eq!(result.hf_gain, None);
    assert_eq!(result.zeros.len(), 2);
}

fn scaled_drive(gain: f64) -> PoleZeroConfig {
    let mut config = PoleZeroConfig::poles_and_zeros(1, 0);
    config.input_is_current = false;
    config.input_voltage_branch = Some(1);
    config.input_voltage_gain = gain;
    // These range tests concern the observation, not numerator eigenvalues.
    config.compute_zeros = false;
    config
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn an_underflowing_improper_coefficient_still_proves_an_unbounded_limit() {
    let result = PoleZeroAnalyzer::new(
        Matrix::identity(2),
        Matrix::from_dense(vec![vec![0.0, 1e-300], vec![0.0; 2]]),
    )
    .analyze(&scaled_drive(1e-300))
    .unwrap();
    // H(s)=-1e-600*s. Projecting its slope to f64 first would fabricate zero.
    assert_eq!(result.hf_gain, None);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn smallest_subnormal_limit_is_retained_and_finite_underflow_is_explicit() {
    let analyzer = PoleZeroAnalyzer::new(
        Matrix::from_dense(vec![vec![2.0_f64.powi(1023), -1.0], vec![0.0, 1.0]]),
        Matrix::zeros(2, 2),
    );
    // x1=2^-51*u, x0=2^-1023*x1: H=2^-1074, the smallest subnormal.
    let result = analyzer.analyze(&scaled_drive(2.0_f64.powi(-51))).unwrap();
    assert_eq!(result.hf_gain.unwrap().to_bits(), 1);
    // Half of that value is nonzero in the exact equations but rounds to zero.
    let error = analyzer
        .analyze(&scaled_drive(2.0_f64.powi(-52)))
        .unwrap_err();
    assert!(
        matches!(
            error,
            PoleZeroAnalysisError::UnrepresentableGain {
                quantity: "high-frequency"
            }
        ),
        "{error:?}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn finite_limit_overflow_is_distinct_from_an_improper_transfer() {
    let mut config = PoleZeroConfig::poles_and_zeros(0, 0);
    config.compute_zeros = false;
    // Exact constant H=2^1024 is finite, but no finite binary64 represents it.
    let error = PoleZeroAnalyzer::new(
        Matrix::from_dense(vec![vec![f64::from_bits(1_u64 << 50)]]),
        Matrix::zeros(1, 1),
    )
    .analyze(&config)
    .unwrap_err();
    assert!(
        matches!(
            error,
            PoleZeroAnalysisError::UnrepresentableGain {
                quantity: "high-frequency"
            }
        ),
        "{error:?}"
    );
}

use super::*;

fn node_state() -> Vec<Vec<Complex64>> {
    vec![vec![Complex64::new(1.0, 0.0), Complex64::new(0.25, -0.5)]]
}

fn branch_state() -> Vec<Vec<Complex64>> {
    vec![vec![
        Complex64::new(-2.0e-3, 0.0),
        Complex64::new(3.0e-3, 4.0e-3),
    ]]
}

#[test]
fn pre_apft_source_transform_identity_version_is_rejected() {
    let obsolete = HbOperatingPointIdentity {
        version: 1,
        semantic_netlist_identity: "0".repeat(64),
        resolved_simulation_identity: "1".repeat(64),
        hb_source_transform_identity: "2".repeat(64),
        retained_state_identity: "3".repeat(64),
    };

    let error = obsolete
        .validate()
        .expect_err("an analytic-source identity must not authenticate APFT state");
    assert!(
        error
            .to_string()
            .contains("producer identity version 1 is unsupported; expected 2"),
        "{error}"
    );
}

#[test]
fn legacy_node_only_state_is_reusable_only_without_mna_branches() {
    let point = HbOperatingPoint::try_from_parts(
        HbConfig::new(1.0e3).with_harmonics(1),
        vec!["out".to_owned()],
        node_state(),
        3,
        1.0e-12,
    )
    .expect("legacy node-only state is structurally valid");

    let state = point
        .to_solver_state(&["out".to_owned()], &[])
        .expect("a branch-free circuit may reuse legacy state");
    assert!(state.mna_branch_currents.is_empty());

    let error = point
        .to_solver_state(&["out".to_owned()], &["V1".to_owned()])
        .expect_err("a circuit MNA branch requires authenticated current state");
    assert!(error.to_string().contains("node-only"), "{error}");
}

#[test]
fn exact_mna_state_round_trips_only_in_canonical_branch_order() {
    let currents = branch_state();
    let point = HbOperatingPoint::try_from_parts_with_mna_branches(
        HbConfig::new(1.0e3).with_harmonics(1),
        vec!["out".to_owned()],
        node_state(),
        vec!["V1".to_owned()],
        currents.clone(),
        3,
        1.0e-12,
    )
    .expect("exact branch evidence is structurally valid");

    assert_eq!(point.mna_branch_names(), ["V1"]);
    assert_eq!(point.mna_branch_spectral_state(), currents);
    let state = point
        .to_solver_state(&["out".to_owned()], &["V1".to_owned()])
        .expect("matching canonical branch identity is accepted");
    assert_eq!(state.mna_branch_currents, branch_state());

    let error = point
        .to_solver_state(&["out".to_owned()], &["L1".to_owned()])
        .expect_err("a different branch identity must fail closed");
    assert!(error.to_string().contains("branch basis"), "{error}");
}

#[test]
fn transported_mna_evidence_rejects_malformed_names_rows_and_values() {
    let config = HbConfig::new(1.0e3).with_harmonics(1);
    let construct = |names: Vec<String>, currents: Vec<Vec<Complex64>>| {
        HbOperatingPoint::try_from_parts_with_mna_branches(
            config.clone(),
            vec!["out".to_owned()],
            node_state(),
            names,
            currents,
            3,
            1.0e-12,
        )
    };

    assert!(
        construct(vec!["V1".to_owned()], Vec::new())
            .expect_err("name/current cardinality mismatch is invalid")
            .to_string()
            .contains("spectral row")
    );
    assert!(
        construct(
            vec!["V1".to_owned(), "v1".to_owned()],
            vec![branch_state()[0].clone(), branch_state()[0].clone()],
        )
        .expect_err("branch names are case-insensitively unique")
        .to_string()
        .contains("duplicate")
    );
    assert!(
        construct(vec![" V1".to_owned()], branch_state())
            .expect_err("non-canonical whitespace is invalid")
            .to_string()
            .contains("non-canonical")
    );
    assert!(
        construct(vec!["V1".to_owned()], vec![vec![Complex64::new(0.0, 0.0)]],)
            .expect_err("a truncated branch spectrum is invalid")
            .to_string()
            .contains("frozen basis")
    );
    assert!(
        construct(
            vec!["V1".to_owned()],
            vec![vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(Value::NAN, 0.0),
            ]],
        )
        .expect_err("a non-finite branch coefficient is invalid")
        .to_string()
        .contains("non-finite")
    );
}

#[test]
fn retained_real_waveform_state_rejects_imaginary_dc_coefficients() {
    let config = HbConfig::new(1.0e3).with_harmonics(1);
    let mut invalid_nodes = node_state();
    invalid_nodes[0][0].im = 1.0e-30;
    let node_error = HbOperatingPoint::try_from_parts(
        config.clone(),
        vec!["out".to_owned()],
        invalid_nodes,
        0,
        0.0,
    )
    .expect_err("a one-sided real-waveform basis has no imaginary DC state");
    assert!(
        node_error.to_string().contains("imaginary DC"),
        "{node_error}"
    );

    let mut invalid_branches = branch_state();
    invalid_branches[0][0].im = -1.0e-30;
    let branch_error = HbOperatingPoint::try_from_parts_with_mna_branches(
        config,
        vec!["out".to_owned()],
        node_state(),
        vec!["V1".to_owned()],
        invalid_branches,
        0,
        0.0,
    )
    .expect_err("a retained MNA branch has no imaginary DC state");
    assert!(
        branch_error.to_string().contains("imaginary DC"),
        "{branch_error}"
    );
}

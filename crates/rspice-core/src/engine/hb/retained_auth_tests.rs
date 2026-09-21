use super::*;

fn integral_point() -> HbOperatingPoint {
    let config = HbConfig::new(1.0e3).with_harmonics(1);
    let integrals = vec![
        HbIntegralSpectrum {
            name: "B:B1:sdt:0".to_owned(),
            coefficients: vec![Complex64::new(5.0e-4, 0.0), Complex64::new(1.0e-4, -2.0e-4)],
        },
        HbIntegralSpectrum {
            name: "B:B1:sdt:1".to_owned(),
            coefficients: vec![Complex64::new(7.0e-7, 0.0), Complex64::new(3.0e-7, -4.0e-7)],
        },
    ];
    let nodes = vec!["out".to_owned()];
    let branches = vec!["V1".to_owned()];
    let identity = HbOperatingPointIdentity::bind(
        HbOperatingPointProducerInputs {
            semantic_netlist_identity: "0".repeat(64),
            resolved_simulation_identity: "1".repeat(64),
            hb_source_transform_identity: "2".repeat(64),
        },
        &config,
        &nodes,
        &node_state(),
        &branches,
        &branch_state(),
        &integrals,
        3,
        1.0e-12,
    );
    HbOperatingPoint::try_from_complete_parts(
        config,
        nodes,
        node_state(),
        branches,
        branch_state(),
        integrals,
        3,
        1.0e-12,
        Some(identity),
    )
    .unwrap()
}

#[test]
fn retained_integral_state_restores_typed_basis_and_authenticates_every_coordinate() {
    let point = integral_point();
    let integral_names = point
        .integral_spectra()
        .iter()
        .map(|s| s.name.clone())
        .collect::<Vec<_>>();
    let state = point
        .to_solver_state(
            point.node_names(),
            point.mna_branch_names(),
            &integral_names,
        )
        .unwrap();
    assert_eq!(point.mna_branch_names(), ["V1"]);
    assert_eq!(point.mna_branch_spectral_state(), branch_state());
    assert_eq!(state.mna_branch_currents[0], branch_state()[0]);
    assert_eq!(
        state.mna_branch_currents[1],
        point.integral_spectra()[0].coefficients
    );
    assert_eq!(
        state.mna_branch_currents[2],
        point.integral_spectra()[1].coefficients
    );
    assert!(
        point
            .to_solver_state(point.node_names(), point.mna_branch_names(), &[])
            .unwrap_err()
            .to_string()
            .contains("integral basis")
    );

    for change in 0..5 {
        let mut changed = point.clone();
        match change {
            0 => changed.integral_spectra.clear(),
            1 => changed.integral_spectra[0].coefficients[0].re += 1.0e-9,
            2 => changed.integral_spectra[0].coefficients[1].im += 1.0e-9,
            3 => changed.integral_spectra.swap(0, 1),
            _ => changed.integral_spectra[0].name = "B:OTHER:sdt:0".to_owned(),
        }
        assert!(
            changed
                .validate()
                .unwrap_err()
                .to_string()
                .contains("authenticated producer identity")
        );
    }

    // Old constructors must not silently discard an authenticated integral tail.
    assert!(
        HbOperatingPoint::try_from_authenticated_parts_with_mna_branches(
            point.producer_identity().unwrap().clone(),
            point.config().clone(),
            point.node_names().to_vec(),
            point.spectral_state().to_vec(),
            point.mna_branch_names().to_vec(),
            point.mna_branch_spectral_state().to_vec(),
            point.iterations(),
            point.residual_norm(),
        )
        .is_err()
    );
    let legacy = HbOperatingPoint::try_from_parts_with_mna_branches(
        point.config().clone(),
        point.node_names().to_vec(),
        point.spectral_state().to_vec(),
        point.mna_branch_names().to_vec(),
        point.mna_branch_spectral_state().to_vec(),
        3,
        1.0e-12,
    )
    .unwrap();
    assert!(
        legacy
            .to_solver_state(
                point.node_names(),
                point.mna_branch_names(),
                &integral_names
            )
            .unwrap_err()
            .to_string()
            .contains("integral basis")
    );
}

#[test]
fn retained_integral_state_rejects_malformed_spectra_before_reuse() {
    for (change, diagnostic) in [
        (0, "non-canonical"),
        (1, "duplicate"),
        (2, "frozen basis"),
        (3, "non-finite"),
        (4, "imaginary DC"),
    ] {
        let mut changed = integral_point();
        changed.producer_identity = None;
        match change {
            0 => changed.integral_spectra[0].name = " B:B1:sdt:0".to_owned(),
            1 => changed.integral_spectra[1].name = "b:b1:SDT:0".to_owned(),
            2 => {
                changed.integral_spectra[0].coefficients.pop();
            }
            3 => changed.integral_spectra[0].coefficients[1].re = f64::NAN,
            _ => changed.integral_spectra[0].coefficients[0].im = 1.0e-30,
        }
        assert!(
            changed
                .validate()
                .unwrap_err()
                .to_string()
                .contains(diagnostic)
        );
    }
}

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
            .contains(&format!(
                "producer identity version 1 is unsupported; expected {HB_OPERATING_POINT_IDENTITY_VERSION}"
            )),
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
        .to_solver_state(&["out".to_owned()], &[], &[])
        .expect("a branch-free circuit may reuse legacy state");
    assert!(state.mna_branch_currents.is_empty());

    let error = point
        .to_solver_state(&["out".to_owned()], &["V1".to_owned()], &[])
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
        .to_solver_state(&["out".to_owned()], &["V1".to_owned()], &[])
        .expect("matching canonical branch identity is accepted");
    assert_eq!(state.mna_branch_currents, branch_state());

    let error = point
        .to_solver_state(&["out".to_owned()], &["L1".to_owned()], &[])
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

//! Quasi-periodic CSV integration over application-retained result fixtures.

#[test]
fn qpac_csv_preserves_every_signed_tuple_translated_axis_and_complex_unit() {
    let retained = crate::simulation::results::qpac_retained_test_fixture();
    let prepared = super::super::prepare_typed_result_csv(&retained).unwrap();
    let mut reader = csv::Reader::from_reader(prepared.contents.as_bytes());
    let headers = reader.headers().unwrap().clone();
    let records = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
    let field = |name| headers.iter().position(|h| h == name).unwrap();
    // 2 nodes + 1 voltage-source branch, 9 full signed tuples, 3 offsets,
    // and one differential result per offset.
    assert_eq!(records.len(), 3 * (3 * 9 + 1));
    assert!(records.iter().any(|r| &r[field("output_tuple")] == "-1;1"));
    let row = records
        .iter()
        .find(|r| &r[field("kind")] == "differential")
        .unwrap();
    assert_eq!(&row[field("gain_unit")], "Ω");
    assert_eq!(&row[field("response_unit")], "V");
    let offset: f64 = row[field("probe_offset_hz")].parse().unwrap();
    let frequency: f64 = row[field("output_frequency_hz")].parse().unwrap();
    assert_eq!(offset, -37.0);
    assert!((frequency - (offset + 1000.0 - 1414.213562373095)).abs() < 1e-12);
    let unit = rspice_core::Complex64::new(
        row[field("unit_response_real")].parse().unwrap(),
        row[field("unit_response_imaginary")].parse().unwrap(),
    );
    let response = rspice_core::Complex64::new(
        row[field("response_real")].parse().unwrap(),
        row[field("response_imaginary")].parse().unwrap(),
    );
    assert!(
        (response - unit * rspice_core::Complex64::from_polar(0.002, 73.0_f64.to_radians())).norm()
            < 1e-14
    );
}

#[test]
fn qpnoise_result_csv_retains_measurements_statuses_covariance_and_primary_evidence() {
    let result = crate::simulation::results::qpnoise_retained_test_fixture();
    let export = super::super::prepare_typed_result_csv(&result).unwrap();
    let mut reader = csv::Reader::from_reader(export.contents.as_bytes());
    let headers = reader.headers().unwrap().clone();
    let rows = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
    let index = |name| headers.iter().position(|h| h == name).unwrap();
    for kind in [
        "metadata",
        "source_law",
        "source_injection",
        "source_density",
        "measurement",
        "input_transfer",
        "covariance",
        "adjoint",
        "integrated",
        "ranking",
    ] {
        assert!(rows.iter().any(|r| &r[0] == kind), "missing {kind}");
    }
    assert!(
        rows.iter().any(|r| &r[index("quantity")] == "input_psd"
            && &r[index("status")] == "zero_input_transfer")
    );
    assert!(rows.iter().any(|r| &r[index("quantity")] == "output_rms"
        && &r[index("status")] == "negative_integration_frequency"));
    assert!(
        rows.iter()
            .any(|r| &r[index("unit")] == "V·A/Hz" && &r[0] == "covariance")
    );
    assert!(
        rows.iter()
            .any(|r| &r[0] == "adjoint" && r[index("coordinate")].starts_with("branch_equation:"))
    );
    assert!(
        rows.iter()
            .all(|r| !r[index("qpss_identity")].is_empty()
                && !r[index("result_identity")].is_empty())
    );
}

#[test]
fn qpxf_csv_preserves_transfer_axes_delay_statuses_and_dual_coordinates() {
    let retained = crate::simulation::results::qpxf_retained_test_fixture();
    let prepared = super::super::prepare_typed_result_csv(&retained).unwrap();
    let mut reader = csv::Reader::from_reader(prepared.contents.as_bytes());
    let headers = reader.headers().unwrap().clone();
    let records = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
    let f = |name| headers.iter().position(|h| h == name).unwrap();
    assert_eq!(records.len(), 4 * 3 + 3 * 3 * 9);
    let transfers: Vec<_> = records
        .iter()
        .filter(|r| &r[f("kind")] == "transfer")
        .collect();
    assert_eq!(transfers.len(), 12);
    assert!(transfers.iter().any(|r| &r[f("unit")] == "Ω"));
    assert!(transfers.iter().any(|r| &r[f("unit")] == "1"));
    for row in &transfers {
        assert_eq!(&row[f("authored_axis")], "output");
        assert_eq!(
            &row[f("authored_frequency_hz")],
            &row[f("output_frequency_hz")]
        );
        assert_eq!(&row[f("frequency_anchor")], "1;-1");
        assert_eq!(&row[f("output_tuple")], "1;-1");
    }
    let finite = transfers
        .iter()
        .find(|r| &r[f("group_delay_status")] == "finite")
        .unwrap();
    assert!(
        finite[f("group_delay_seconds")]
            .parse::<f64>()
            .unwrap()
            .is_finite()
    );
    let absent = transfers
        .iter()
        .find(|r| &r[f("group_delay_status")] == "below_magnitude_floor")
        .unwrap();
    assert!(absent[f("group_delay_seconds")].is_empty());
    let adjoint = records.iter().find(|r| &r[f("kind")] == "adjoint").unwrap();
    assert_eq!(&adjoint[f("unit")], "V / equation RHS");
    assert_eq!(&adjoint[f("group_delay_status")], "not_applicable");
    assert_eq!(adjoint[f("qpss_identity")].len(), 64);
    assert_eq!(adjoint[f("result_identity")].len(), 64);
}

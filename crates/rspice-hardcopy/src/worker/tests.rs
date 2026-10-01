use super::*;

#[test]
fn request_rejects_protocol_counter_and_unknown_field_drift() {
    let valid = HardcopyWorkerRequest::try_new(
        17,
        0,
        29,
        HardcopyWorkerCommand::ResolveSource {
            source_key: "retained:test-source".to_owned(),
            scope: HardcopyScope::ActiveDocument,
        },
    )
    .expect("fixture request is valid");
    assert_eq!(valid.epoch, "0");
    assert_eq!(valid.generation, "29");
    valid.validate().expect("canonical request validates");
    let maximum = HardcopyWorkerRequest::try_new(
        u32::MAX,
        u64::MAX,
        u64::MAX,
        HardcopyWorkerCommand::ResolveSource {
            source_key: "retained:test-source".to_owned(),
            scope: HardcopyScope::ActiveDocument,
        },
    )
    .expect("fixture request is valid");
    assert_eq!(maximum.epoch, u64::MAX.to_string());
    assert_eq!(maximum.generation, u64::MAX.to_string());

    let mut wrong_protocol = valid.clone();
    wrong_protocol.protocol_version = HARDCOPY_WORKER_PROTOCOL_VERSION + 1;
    assert!(wrong_protocol.validate().is_err());

    let mut zero_id = valid.clone();
    zero_id.id = 0;
    assert!(zero_id.validate().is_err());

    for epoch in ["00", "01", "+1", "-1", " 1"] {
        let mut invalid = valid.clone();
        invalid.epoch = epoch.to_owned();
        assert!(
            invalid.validate().is_err(),
            "noncanonical epoch {epoch:?} must fail"
        );
    }
    for generation in ["0", "00", "01", "+1", "-1", " 1"] {
        let mut invalid = valid.clone();
        invalid.generation = generation.to_owned();
        assert!(
            invalid.validate().is_err(),
            "noncanonical generation {generation:?} must fail"
        );
    }

    let mut unknown_request = serde_json::to_value(&valid).expect("request serializes");
    unknown_request
        .as_object_mut()
        .expect("request is an object")
        .insert("futureField".to_owned(), serde_json::json!(true));
    assert!(
        serde_json::from_value::<HardcopyWorkerRequest>(unknown_request).is_err(),
        "unknown request metadata must fail closed"
    );

    let mut unknown_command = serde_json::to_value(&valid).expect("request serializes");
    unknown_command["command"]
        .as_object_mut()
        .expect("command is an object")
        .insert("future-field".to_owned(), serde_json::json!(42));
    assert!(
        serde_json::from_value::<HardcopyWorkerRequest>(unknown_command).is_err(),
        "unknown operation metadata must fail closed"
    );
}

#[test]
fn metadata_and_request_buffer_budgets_are_exact_and_overflow_safe() {
    assert!(validate_metadata_length(0).is_ok());
    assert!(validate_metadata_length(MAX_REQUEST_METADATA_BYTES).is_ok());
    assert!(validate_metadata_length(MAX_REQUEST_METADATA_BYTES + 1).is_err());

    assert!(validate_request_buffer_lengths([]).is_ok());
    assert!(validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES]).is_ok());
    assert!(
        validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES.saturating_sub(1), 1,]).is_ok()
    );
    assert!(
        validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES.saturating_sub(1), 2,]).is_err()
    );
    assert!(validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES + 1]).is_err());
    assert!(validate_request_buffer_lengths([0, 0, 0]).is_err());
    assert!(validate_request_buffer_lengths([usize::MAX, 1]).is_err());
}

#[test]
fn response_preflight_enforces_operation_cardinality_and_transport_budgets() {
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::ResolveSource,
            Some(1),
            [MAX_WORKER_SNAPSHOT_BYTES],
        )
        .is_ok()
    );
    assert!(
        validate_response_buffer_lengths(HardcopyWorkerOperation::ResolveSource, Some(1), [],)
            .is_err()
    );
    assert!(
        validate_response_buffer_lengths(HardcopyWorkerOperation::ResolveSource, Some(1), [1, 1],)
            .is_err()
    );
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::ResolveSource,
            Some(1),
            [MAX_WORKER_SNAPSHOT_BYTES + 1],
        )
        .is_err()
    );

    let one_preview = [
        MAX_PREVIEW_WORKER_MANIFEST_BYTES,
        MAX_PREVIEW_WORKER_RGBA_BYTES,
    ];
    let two_previews = [
        MAX_PREVIEW_WORKER_MANIFEST_BYTES,
        MAX_PREVIEW_WORKER_RGBA_BYTES,
        MAX_PREVIEW_WORKER_MANIFEST_BYTES,
        MAX_PREVIEW_WORKER_RGBA_BYTES,
    ];
    assert!(
        validate_response_buffer_lengths(HardcopyWorkerOperation::Preview, Some(2), one_preview,)
            .is_ok()
    );
    assert!(
        validate_response_buffer_lengths(HardcopyWorkerOperation::Preview, Some(4), two_previews,)
            .is_ok()
    );
    assert!(
        validate_response_buffer_lengths(HardcopyWorkerOperation::Preview, Some(4), one_preview,)
            .is_err()
    );
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::Preview,
            Some(2),
            [MAX_PREVIEW_WORKER_MANIFEST_BYTES + 1, 0],
        )
        .is_err()
    );
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::Preview,
            Some(2),
            [0, MAX_PREVIEW_WORKER_RGBA_BYTES + 1],
        )
        .is_err()
    );
    assert!(
        validate_response_buffer_lengths(HardcopyWorkerOperation::Preview, Some(3), [0, 0, 0],)
            .is_err()
    );

    let publication_bytes = MAX_PUBLICATION_BYTES as usize;
    let second_maximum_part = publication_bytes
        .checked_sub(MAX_ARTIFACT_BYTES)
        .expect("publication aggregate exceeds one artifact");
    assert!(second_maximum_part <= MAX_ARTIFACT_BYTES);
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::Publication,
            Some(3),
            [
                MAX_PUBLICATION_WORKER_MANIFEST_BYTES,
                MAX_ARTIFACT_BYTES,
                second_maximum_part,
            ],
        )
        .is_ok()
    );
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::Publication,
            Some(2),
            [MAX_PUBLICATION_WORKER_MANIFEST_BYTES + 1, 0],
        )
        .is_err()
    );
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::Publication,
            Some(2),
            [0, MAX_ARTIFACT_BYTES + 1],
        )
        .is_err()
    );
    assert!(
        validate_response_buffer_lengths(
            HardcopyWorkerOperation::Publication,
            Some(4),
            [0, MAX_ARTIFACT_BYTES, second_maximum_part, 1,],
        )
        .is_err()
    );
    assert!(
        validate_response_buffer_lengths(HardcopyWorkerOperation::Publication, Some(1), [0],)
            .is_err()
    );
}

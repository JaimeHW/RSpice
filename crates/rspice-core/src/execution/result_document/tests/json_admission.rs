use super::*;

fn admitted(json: &str, count: usize) {
    let limits = crate::ResourceLimits {
        max_result_values: count,
        max_external_data_values: count,
        ..Default::default()
    };
    assert_eq!(
        super::super::json_admission::check(json, &limits, &NoAbort).unwrap(),
        count
    );
    if count == 0 {
        return;
    }
    for external in [false, true] {
        let mut lower = limits;
        let resource = if external {
            lower.max_external_data_values -= 1;
            crate::ResourceKind::ExternalDataValues
        } else {
            lower.max_result_values -= 1;
            crate::ResourceKind::ResultValues
        };
        let error = AnalysisResultDocument::from_json_with_limits_and_abort(
            json,
            &lower,
            &NoAbort,
            u64::MAX,
        )
        .unwrap_err();
        assert!(
            matches!(error, ResultDocumentError::ResourceLimit(ref error)
            if error.resource == resource && error.limit == count - 1 && error.requested == count),
            "{error}"
        );
    }
}

#[test]
fn every_family_admission_matches_its_exact_retained_value_budget() {
    for kind in AnalysisResultKind::ALL {
        let document = document_for(kind);
        let count = document.total_value_count();
        let json = document.to_json().unwrap();
        admitted(&json, count);
        let limits = crate::ResourceLimits {
            max_result_values: count,
            max_external_data_values: count,
            ..Default::default()
        };
        let decoded = AnalysisResultDocument::from_json_with_limits_and_abort(
            &json,
            &limits,
            &NoAbort,
            u64::MAX,
        );
        // QP evidence validation also reconstructs a bounded working grid. Its
        // temporary storage can exceed the retained document; admission must
        // preserve that existing validation result, not bypass the grid budget.
        match document.validate_with_limits_and_abort(&limits, &NoAbort) {
            Ok(()) => assert_eq!(decoded.unwrap(), document, "{}", kind.tag()),
            Err(expected) => assert_eq!(
                decoded.unwrap_err().to_string(),
                expected.to_string(),
                "{}",
                kind.tag()
            ),
        }
        // Object key order changes when the document goes through a JSON map.
        let reordered =
            serde_json::to_string(&serde_json::from_str::<serde_json::Value>(&json).unwrap())
                .unwrap();
        admitted(&reordered, count);
    }
}

#[test]
fn optional_payload_data_and_missing_complex_samples_are_charged() {
    let mut documents = [
        document_for(AnalysisResultKind::MonteCarlo),
        document_for(AnalysisResultKind::PortNoise),
        document_for(AnalysisResultKind::Transient),
        document_for(AnalysisResultKind::Ac),
    ];
    if let ResultPayload::MonteCarlo(payload) = &mut documents[0].payload {
        payload.successful_trial_indices = Some(vec![0, 1, 2]);
    }
    if let ResultPayload::PortNoise(payload) = &mut documents[1].payload {
        payload.two_port.push(TwoPortNoiseEntry {
            frequency: 1e9,
            noise_resistance_ohm: 50.0,
            noise_factor: 2.0,
            minimum_noise_factor: 1.0,
            optimum_source_reflection: ComplexSample {
                real: 0.1,
                imaginary: 0.2,
            },
        });
    }
    if let ResultPayload::Tran(payload) = &mut documents[2].payload {
        payload.current_impulses = Some(vec![crate::CurrentImpulseTrace {
            owner: crate::CurrentImpulseOwner::DeviceLead {
                device_name: "München \\\"".into(),
                parameter: "drain_current".into(),
            },
            complete: true,
            points: vec![crate::CurrentImpulsePoint {
                time: 0.0,
                charge_coulombs: 1e-9,
            }],
            derivatives: vec![crate::CurrentImpulseDerivative {
                time: 0.0,
                order: 1,
                coefficient: 1e-12,
            }],
        }]);
        payload.voltage_impulses = Some(vec![crate::VoltageImpulseTrace {
            node_name: "node α".into(),
            complete: true,
            points: vec![crate::VoltageImpulsePoint {
                time: 0.0,
                volt_seconds: 1e-9,
            }],
            derivatives: vec![],
        }]);
    }
    if let SeriesValues::Complex { samples } = &mut documents[3].signals[0].values {
        samples.fill(None);
    }
    for document in documents {
        // Some synthetic evidence is deliberately invalid; admission must still
        // bound its actual storage before the evidence validator rejects it.
        admitted(
            &serde_json::to_string(&document).unwrap(),
            document.total_value_count(),
        );
    }
}

#[test]
fn over_budget_arrays_are_refused_before_numeric_or_typed_decoding() {
    let schema = format!(
        r#""schema":"{ANALYSIS_RESULT_DOCUMENT_SCHEMA}","schemaVersion":{ANALYSIS_RESULT_DOCUMENT_VERSION}"#
    );
    let cases = [
        r#""axes":[{"values":{"values":[0,0,0,0,0,0,0,0,0,0,1e-999]}}]"#,
        r#""signals":[{"values":{"samples":[null,null,null,null,null,null,1e-999],"representation":"complex"}}]"#,
        r#""payload":{"statistics":[{"samples":[0,0,0,0,0,0,0,0,0,0,1e-999]}],"family":"monte-carlo"}"#,
        r#""payload":{"result":{"nested":[0,0,0,0,0,0,0,0,0,0,1e-999]},"family":"qpnoise"}"#,
    ];
    for fields in cases {
        for external in [false, true] {
            let mut limits = crate::ResourceLimits::default();
            let resource = if external {
                limits.max_external_data_values = 5;
                crate::ResourceKind::ExternalDataValues
            } else {
                limits.max_result_values = 5;
                crate::ResourceKind::ResultValues
            };
            let error = AnalysisResultDocument::from_json_with_limits_and_abort(
                &format!("{{{schema},{fields}}}"),
                &limits,
                &NoAbort,
                u64::MAX,
            )
            .unwrap_err();
            assert!(
                matches!(error, ResultDocumentError::ResourceLimit(ref error)
                if error.resource == resource && error.requested == 6 && error.limit == 5),
                "{fields}: {error}"
            );
        }
    }
}

#[test]
fn admission_polls_cancellation_while_walking_large_arrays() {
    let json = format!(
        r#"{{"axes":[{{"values":{{"values":[{}]}}}}]}}"#,
        vec!["0"; 4096].join(",")
    );
    let error = super::super::json_admission::check(
        &json,
        &crate::ResourceLimits::default(),
        &CountingAbort::new(3),
    )
    .unwrap_err();
    assert!(matches!(error, ResultDocumentError::Aborted), "{error}");
}

#[test]
fn escaped_field_names_and_late_discriminators_do_not_bypass_admission() {
    let json = r#"{"signals":[{"values":{"samp\u006ces":[null,null],"representat\u0069on":"complex"}}],"payload":{"angularFrequenci\u0065s":[1,2],"ports":[{},{}],"family":"sp"}}"#;
    assert_eq!(
        super::super::json_admission::check(json, &crate::ResourceLimits::default(), &NoAbort)
            .unwrap(),
        10
    );
}

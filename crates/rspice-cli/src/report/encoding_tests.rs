use super::tests::report_with_measurement;
use super::*;

#[test]
fn tap_names_cannot_introduce_directives_or_extra_results() {
    let mut report = report_with_measurement(Some(1.0));
    report.name = "deck # TODO ignored\r\nnot ok 9 - injected".into();
    report.measurements[0].name = "measurement # SKIP ignored\nBail out!".into();
    report.measurements[0].record_index = Some(0);
    report.measurements[0].aggregate_policy = Some("all\nnot ok 10 - injected".into());
    let mut bytes = Vec::new();
    write_tap_report(&mut bytes, Path::new("report.tap"), &[report]).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let results: Vec<_> = text
        .lines()
        .filter(|line| line.starts_with("ok ") || line.starts_with("not ok "))
        .collect();
    assert_eq!(results.len(), 2, "{text}");
    assert!(results.iter().all(|line| !line.contains('#')), "{text}");
    assert!(
        !text.lines().any(|line| line.starts_with("Bail out!")),
        "{text}"
    );
    assert!(text.lines().all(|line| !line.contains('\r')), "{text}");
}

#[test]
fn tap_diagnostics_round_trip_quotes_controls_and_line_breaks() {
    let error = "can't read 'V(out)': \"bad\"\\path\r\n\t\0\u{1b}\u{7f}\u{85}\u{9f}\u{2028}\u{2029}\u{fffe}\u{ffff} Ω 😀";
    let mut report = report_with_measurement(None);
    report.passed = false;
    report.error = Some(error.into());
    report.measurements[0].error = Some(error.into());
    let expected = [
        run_failure_message(&report).unwrap(),
        measurement_failure_diagnostics(&report.measurements[0]),
    ];
    let mut bytes = Vec::new();
    write_tap_report(&mut bytes, Path::new("report.tap"), &[report]).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let messages: Vec<_> = text
        .lines()
        .filter_map(|line| line.strip_prefix("  message: "))
        .collect();
    assert_eq!(messages.len(), expected.len(), "{text}");
    for (encoded, expected) in messages.into_iter().zip(expected) {
        // JSON double-quoted strings are also YAML scalars. Decoding with an
        // independent parser checks escaping and exact diagnostic retention.
        assert_eq!(serde_json::from_str::<String>(encoded).unwrap(), expected);
        assert!(
            !encoded.chars().any(|ch| ch.is_control()
                || matches!(ch, '\u{2028}' | '\u{2029}' | '\u{fffe}' | '\u{ffff}'))
        );
    }
}

use super::tests::report_with_measurement;
use super::*;

#[test]
fn junit_preserves_legal_whitespace_in_names_and_diagnostics() {
    let text = "quotes ' \" & < >\t\r\nΩ 😀";
    let mut report = report_with_measurement(None);
    report.name = text.into();
    report.netlist = text.into();
    report.measurements[0].name = text.into();
    report.measurements[0].error = Some(text.into());
    let mut bytes = Vec::new();
    write_junit_report(&mut bytes, Path::new("report.xml"), &[report]).unwrap();
    let xml = String::from_utf8(bytes).unwrap();
    let document = roxmltree::Document::parse(&xml).unwrap();
    let suite = document
        .descendants()
        .find(|node| node.has_tag_name("testsuite"))
        .unwrap();
    assert_eq!(suite.attribute("name"), Some(text));
    let case = document
        .descendants()
        .filter(|node| node.has_tag_name("testcase"))
        .nth(1)
        .unwrap();
    assert_eq!(case.attribute("name"), Some(text));
    assert_eq!(case.attribute("classname"), Some(text));
    assert_eq!(
        case.children()
            .find(|node| node.has_tag_name("failure"))
            .unwrap()
            .text(),
        Some(text)
    );
}

#[test]
fn junit_forbidden_characters_cannot_make_the_report_unreadable() {
    let mut report = report_with_measurement(None);
    let forbidden = "bad\0\u{1}\u{b}\u{1f}\u{fffe}\u{ffff}text";
    report.name = forbidden.into();
    report.netlist = forbidden.into();
    report.passed = false;
    report.error = Some(forbidden.into());
    report.measurements[0].name = forbidden.into();
    report.measurements[0].error = Some(forbidden.into());
    let mut bytes = Vec::new();
    write_junit_report(&mut bytes, Path::new("report.xml"), &[report]).unwrap();
    let xml = String::from_utf8(bytes).unwrap();
    let document = roxmltree::Document::parse(&xml).unwrap();
    let suite = document
        .descendants()
        .find(|node| node.has_tag_name("testsuite"))
        .unwrap();
    assert_eq!(suite.attribute("failures"), Some("2"));
    assert_eq!(
        suite.attribute("name"),
        Some(r"bad\u{0}\u{1}\u{b}\u{1f}\u{fffe}\u{ffff}text")
    );
    assert_eq!(
        document
            .descendants()
            .filter(|node| node.has_tag_name("failure"))
            .count(),
        2
    );
}

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

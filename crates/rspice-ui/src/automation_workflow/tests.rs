use sha2::{Digest, Sha256};

use super::*;

const SOURCE: &str = "plan = project.plan(\"Lab characterization\")\n\
run = plan.with_corners(\"all\").execute(target=\"local\")\n\
run.require(specs=\"release\")\n\
run.compare(baseline=\"main\", waveforms=True)\n\
run.export([\"junit\", \"summary.json\", \"report.pdf\"])";

fn plan() -> AutomationPlan {
    compile_workflow(SOURCE).expect("canonical workflow compiles")
}

fn evidence(plan: &AutomationPlan) -> CompletedEvidence {
    let checks = vec![
        CheckEvidence::try_new(
            "gain<&\"'",
            "Closed-loop gain < 1% & stable",
            CheckOutcome::Passed,
            12,
            "Measured at TT & 27 °C",
        )
        .expect("valid passed check"),
        CheckEvidence::try_new(
            "noise",
            "Input noise",
            CheckOutcome::Failed,
            1_004,
            "Limit exceeded: 12 < 13 & review \"required\"",
        )
        .expect("valid failed check"),
    ];
    CompletedEvidence::try_new(
        plan.source_digest(),
        "rev-α",
        "run-42",
        checks,
        ComparisonEvidence::try_new("main", 27, 1).expect("valid comparison"),
    )
    .expect("valid completed evidence")
}

#[test]
fn canonical_source_compiles_to_exact_immutable_plan() {
    let plan = plan();
    let expected: [u8; 32] = Sha256::digest(SOURCE.as_bytes()).into();
    assert_eq!(plan.source_digest().as_bytes(), &expected);
    assert_eq!(plan.project_name(), "Lab characterization");
    assert_eq!(plan.corners(), "all");
    assert_eq!(plan.target(), "local");
    assert_eq!(plan.required_specs(), "release");
    assert_eq!(plan.baseline(), "main");
    assert!(plan.compare_waveforms());
    assert_eq!(
        plan.artifacts().collect::<Vec<_>>(),
        vec![
            ArtifactKind::JunitXml,
            ArtifactKind::SummaryJson,
            ArtifactKind::VerificationPdf
        ]
    );
}

#[test]
fn unicode_and_crlf_have_correct_byte_spans_and_one_based_locations() {
    let source = SOURCE
        .replace("Lab characterization", "測定 characterization")
        .replace('\n', "\r\n")
        .replace("target=\"local\"", "target=\"remote\"");
    let diagnostics = compile_workflow(&source).expect_err("remote target is rejected");
    let diagnostic = diagnostics
        .as_slice()
        .iter()
        .find(|diagnostic| diagnostic.message.contains("remote targets"))
        .expect("remote-target diagnostic");
    assert_eq!(diagnostic.start.line, 2);
    assert_eq!(diagnostic.start.column, 47);
    assert_eq!(
        &source[diagnostic.span.start..diagnostic.span.end],
        "\"remote\""
    );
}

#[test]
fn parser_rejects_missing_duplicate_unknown_and_out_of_order_stages() {
    let missing = SOURCE.lines().take(4).collect::<Vec<_>>().join("\n");
    assert!(has_code(&missing, DiagnosticCode::MissingStage));

    let duplicate = format!("{SOURCE}\nrun.require(specs=\"release\")");
    assert!(has_code(&duplicate, DiagnosticCode::DuplicateStage));

    let unknown = SOURCE.replace(
        "run.require(specs=\"release\")",
        "run.shell(command=\"rm -rf /\")",
    );
    assert!(has_code(&unknown, DiagnosticCode::UnknownStage));

    let mut lines = SOURCE.lines().collect::<Vec<_>>();
    lines.swap(2, 3);
    assert!(has_code(&lines.join("\n"), DiagnosticCode::OutOfOrderStage));
}

#[test]
fn parser_rejects_unknown_missing_and_duplicate_arguments() {
    let unknown = SOURCE.replace("target=\"local\"", "host=\"local\"");
    assert!(has_code(&unknown, DiagnosticCode::UnknownArgument));

    let missing = SOURCE.replace("target=\"local\"", "");
    assert!(has_code(&missing, DiagnosticCode::MissingArgument));

    let duplicate = SOURCE.replace("target=\"local\"", "target=\"local\", target=\"local\"");
    assert!(has_code(&duplicate, DiagnosticCode::DuplicateArgument));
}

#[test]
fn parser_rejects_remote_and_arbitrary_execution_syntax() {
    let remote = SOURCE.replace("target=\"local\"", "target=\"cluster\"");
    assert!(has_code(&remote, DiagnosticCode::UnsupportedValue));

    for forbidden in [
        "run.eval(\"1 + 1\")",
        "run.shell(\"echo secret\")",
        "run.network(\"https://example.com\")",
        "run.fs(\"../secret\")",
        "import os",
    ] {
        let source = format!("{SOURCE}\n{forbidden}");
        assert!(parse_workflow(&source).is_err(), "accepted {forbidden:?}");
    }
}

#[test]
fn parser_rejects_malformed_oversized_and_duplicate_artifacts() {
    assert!(parse_workflow("plan = project.plan(\"unterminated)").is_err());
    assert!(has_code(
        &"x".repeat(super::parser::MAX_SOURCE_BYTES + 1),
        DiagnosticCode::SourceTooLarge
    ));
    let duplicate = SOURCE.replace(
        "\"junit\", \"summary.json\"",
        "\"junit\", \"junit\", \"summary.json\"",
    );
    assert!(has_code(&duplicate, DiagnosticCode::DuplicateArtifact));

    let unknown = SOURCE.replace("\"report.pdf\"", "\"raw-results.zip\"");
    assert!(has_code(&unknown, DiagnosticCode::UnsupportedValue));

    let empty = SOURCE.replace("[\"junit\", \"summary.json\", \"report.pdf\"]", "[]");
    assert!(has_code(&empty, DiagnosticCode::EmptyArtifactSet));

    let incomplete_comparison = SOURCE.replace("waveforms=True", "waveforms=False");
    assert!(has_code(
        &incomplete_comparison,
        DiagnosticCode::UnsupportedValue
    ));
}

#[test]
fn exact_source_bytes_produce_distinct_digests() {
    let lf = plan();
    let crlf = compile_workflow(&SOURCE.replace('\n', "\r\n")).expect("CRLF source compiles");
    assert_ne!(lf.source_digest(), crlf.source_digest());
}

#[test]
fn completed_evidence_is_fail_closed() {
    let plan = plan();
    let comparison = ComparisonEvidence::try_new("main", 1, 0).expect("valid comparison");
    assert!(matches!(
        CompletedEvidence::try_new(plan.source_digest(), "rev", "run", vec![], comparison),
        Err(EvidenceError::NoChecks)
    ));
    assert!(matches!(
        CheckEvidence::try_new("id", "failed", CheckOutcome::Failed, 0, ""),
        Err(EvidenceError::MissingFailureDetail { .. })
    ));
    assert!(matches!(
        ComparisonEvidence::try_new("main", 1, 2),
        Err(EvidenceError::InvalidComparisonCounts { .. })
    ));
    assert!(matches!(
        ComparisonEvidence::try_new_complete("main", 2, 1, 0, 0, "policy", "coverage mismatch"),
        Err(EvidenceError::InvalidComparisonCoverage { .. })
    ));
    assert!(matches!(
        ComparisonEvidence::try_new_complete("main", 1, 1, 1, 0, "policy", ""),
        Err(EvidenceError::MissingComparisonFailureDetail)
    ));

    let first = CheckEvidence::try_new("duplicate", "first", CheckOutcome::Passed, u64::MAX, "")
        .expect("valid check");
    let second = CheckEvidence::try_new("other", "second", CheckOutcome::Passed, 1, "")
        .expect("valid check");
    assert!(matches!(
        CompletedEvidence::try_new(
            plan.source_digest(),
            "rev",
            "run",
            vec![first, second],
            ComparisonEvidence::try_new("main", 0, 0).expect("valid comparison")
        ),
        Err(EvidenceError::DurationOverflow)
    ));

    let first = CheckEvidence::try_new("duplicate", "first", CheckOutcome::Passed, 0, "")
        .expect("valid check");
    let second = CheckEvidence::try_new("duplicate", "second", CheckOutcome::Passed, 0, "")
        .expect("valid check");
    assert!(matches!(
        CompletedEvidence::try_new(
            plan.source_digest(),
            "rev",
            "run",
            vec![first, second],
            ComparisonEvidence::try_new("main", 0, 0).expect("valid comparison")
        ),
        Err(EvidenceError::DuplicateCheckId { .. })
    ));
}

#[test]
fn artifacts_are_deterministic_and_xml_is_escaped() {
    let plan = plan();
    let evidence = evidence(&plan);
    let first = render_requested_artifacts(&plan, &evidence).expect("artifacts render");
    let second = render_requested_artifacts(&plan, &evidence).expect("artifacts render again");
    assert_eq!(first, second);

    let junit = first
        .iter()
        .find(|artifact| artifact.kind() == ArtifactKind::JunitXml)
        .expect("JUnit artifact");
    let xml = std::str::from_utf8(junit.bytes()).expect("UTF-8 XML");
    assert!(xml.contains("id=\"gain&lt;&amp;&quot;&apos;\""));
    assert!(xml.contains("Closed-loop gain &lt; 1% &amp; stable"));
    assert!(xml.contains("12 &lt; 13 &amp; review \"required\""));
    assert!(xml.contains("tests=\"3\" failures=\"2\""));
    assert!(xml.contains("id=\"rspice.automation.baseline-comparison\""));
    assert!(xml.contains("baseline comparison failed"));
    assert_eq!(junit.file_name(), "junit.xml");
    assert_eq!(junit.media_type(), "application/xml");
}

#[test]
fn summary_is_typed_json_and_pdf_is_parseable() {
    let plan = plan();
    let evidence = evidence(&plan);
    let artifacts = render_requested_artifacts(&plan, &evidence).expect("artifacts render");

    let summary = artifacts
        .iter()
        .find(|artifact| artifact.kind() == ArtifactKind::SummaryJson)
        .expect("summary artifact");
    let json: serde_json::Value = serde_json::from_slice(summary.bytes()).expect("valid JSON");
    assert_eq!(json["schemaVersion"], "rspice.automation.summary/1");
    assert_eq!(json["outcome"], "failed");
    assert_eq!(json["checks"]["total"], 2);
    assert_eq!(json["comparison"]["waveformTargets"], 27);
    assert_eq!(json["comparison"]["evaluatedWaveforms"], 27);
    assert_eq!(json["comparison"]["differingWaveforms"], 1);
    assert_eq!(json["comparison"]["missingWaveforms"], 0);
    assert_eq!(
        json["comparison"]["tolerancePolicyDigest"],
        "rspice-exact-waveform-comparison/v1"
    );

    let pdf = artifacts
        .iter()
        .find(|artifact| artifact.kind() == ArtifactKind::VerificationPdf)
        .expect("PDF artifact");
    assert!(pdf.bytes().starts_with(b"%PDF-1.7"));
    assert!(pdf.bytes().ends_with(b"%%EOF\n"));
    let document = lopdf::Document::load_mem(pdf.bytes()).expect("standards-valid PDF");
    assert_eq!(document.get_pages().len(), 1);
}

#[test]
fn pdf_renderer_retains_multipage_evidence() {
    let plan = plan();
    let checks = (0..100)
        .map(|index| {
            CheckEvidence::try_new(
                format!("check-{index:03}"),
                format!("Verification check {index:03}"),
                CheckOutcome::Passed,
                index,
                "",
            )
            .expect("valid check")
        })
        .collect();
    let evidence = CompletedEvidence::try_new(
        plan.source_digest(),
        "rev-multipage",
        "run-multipage",
        checks,
        ComparisonEvidence::try_new("main", 100, 0).expect("valid comparison"),
    )
    .expect("valid completed evidence");
    let artifacts = render_requested_artifacts(&plan, &evidence).expect("artifacts render");
    let pdf = artifacts
        .iter()
        .find(|artifact| artifact.kind() == ArtifactKind::VerificationPdf)
        .expect("PDF artifact");
    let document = lopdf::Document::load_mem(pdf.bytes()).expect("standards-valid PDF");
    assert_eq!(document.get_pages().len(), 3);
}

#[test]
fn renderer_rejects_evidence_from_another_plan_or_baseline() {
    let plan = plan();
    let other = compile_workflow(&SOURCE.replace("Lab characterization", "Other"))
        .expect("other plan compiles");
    let evidence = evidence(&other);
    assert!(matches!(
        render_requested_artifacts(&plan, &evidence),
        Err(ArtifactRenderError::PlanDigestMismatch { .. })
    ));

    let check =
        CheckEvidence::try_new("id", "name", CheckOutcome::Passed, 0, "").expect("valid check");
    let wrong_baseline = CompletedEvidence::try_new(
        plan.source_digest(),
        "rev",
        "run",
        vec![check],
        ComparisonEvidence::try_new("release", 1, 0).expect("valid comparison shape"),
    )
    .expect("valid evidence shape");
    assert!(matches!(
        render_requested_artifacts(&plan, &wrong_baseline),
        Err(ArtifactRenderError::BaselineMismatch { .. })
    ));
}

fn has_code(source: &str, code: DiagnosticCode) -> bool {
    parse_workflow(source)
        .err()
        .or_else(|| compile_workflow(source).err())
        .is_some_and(|diagnostics| {
            diagnostics
                .as_slice()
                .iter()
                .any(|diagnostic| diagnostic.code == code)
        })
}

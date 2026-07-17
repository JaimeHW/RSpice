use std::{error::Error, fmt};

use serde::Serialize;

use super::parser::{ArtifactKind, AutomationPlan, SourceDigest};

const MAX_EVIDENCE_TEXT_BYTES: usize = 16 * 1024;
const MAX_TOTAL_EVIDENCE_BYTES: usize = 16 * 1024 * 1024;
const MAX_CHECKS: usize = 100_000;
const PDF_LINES_PER_PAGE: usize = 44;
const PDF_WRAP_COLUMNS: usize = 88;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckOutcome {
    Passed,
    Failed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckEvidence {
    id: String,
    name: String,
    outcome: CheckOutcome,
    duration_ms: u64,
    detail: String,
}

impl CheckEvidence {
    pub fn try_new(
        id: impl Into<String>,
        name: impl Into<String>,
        outcome: CheckOutcome,
        duration_ms: u64,
        detail: impl Into<String>,
    ) -> Result<Self, EvidenceError> {
        let id = id.into();
        let name = name.into();
        let detail = detail.into();
        validate_text("check ID", &id, false, false)?;
        validate_text("check name", &name, false, false)?;
        validate_text("check detail", &detail, true, true)?;
        if outcome == CheckOutcome::Failed && detail.trim().is_empty() {
            return Err(EvidenceError::MissingFailureDetail { check_id: id });
        }
        Ok(Self {
            id,
            name,
            outcome,
            duration_ms,
            detail,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn outcome(&self) -> CheckOutcome {
        self.outcome
    }

    pub const fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComparisonEvidence {
    baseline: String,
    /// Complete union of governed waveform targets in the baseline, candidate,
    /// and sealed tolerance policy.
    waveform_count: u64,
    evaluated_waveform_count: u64,
    differing_waveform_count: u64,
    missing_waveform_count: u64,
    tolerance_policy_digest: String,
    detail: String,
}

impl ComparisonEvidence {
    pub fn try_new(
        baseline: impl Into<String>,
        waveform_count: u64,
        differing_waveform_count: u64,
    ) -> Result<Self, EvidenceError> {
        let detail = if differing_waveform_count == 0 {
            String::new()
        } else {
            format!(
                "{differing_waveform_count} of {waveform_count} exact waveform comparisons differ"
            )
        };
        Self::try_new_complete(
            baseline,
            waveform_count,
            waveform_count,
            differing_waveform_count,
            0,
            "rspice-exact-waveform-comparison/v1",
            detail,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_new_complete(
        baseline: impl Into<String>,
        waveform_count: u64,
        evaluated_waveform_count: u64,
        differing_waveform_count: u64,
        missing_waveform_count: u64,
        tolerance_policy_digest: impl Into<String>,
        detail: impl Into<String>,
    ) -> Result<Self, EvidenceError> {
        let baseline = baseline.into();
        let tolerance_policy_digest = tolerance_policy_digest.into();
        let detail = detail.into();
        validate_text("comparison baseline", &baseline, false, false)?;
        validate_text(
            "comparison tolerance policy digest",
            &tolerance_policy_digest,
            false,
            false,
        )?;
        validate_text("comparison detail", &detail, true, true)?;
        let covered = evaluated_waveform_count
            .checked_add(missing_waveform_count)
            .ok_or(EvidenceError::InvalidComparisonCoverage {
                waveform_count,
                evaluated_waveform_count,
                missing_waveform_count,
            })?;
        if covered != waveform_count {
            return Err(EvidenceError::InvalidComparisonCoverage {
                waveform_count,
                evaluated_waveform_count,
                missing_waveform_count,
            });
        }
        if differing_waveform_count > evaluated_waveform_count {
            return Err(EvidenceError::InvalidComparisonCounts {
                waveform_count: evaluated_waveform_count,
                differing_waveform_count,
            });
        }
        if (differing_waveform_count != 0 || missing_waveform_count != 0)
            && detail.trim().is_empty()
        {
            return Err(EvidenceError::MissingComparisonFailureDetail);
        }
        Ok(Self {
            baseline,
            waveform_count,
            evaluated_waveform_count,
            differing_waveform_count,
            missing_waveform_count,
            tolerance_policy_digest,
            detail,
        })
    }

    pub fn baseline(&self) -> &str {
        &self.baseline
    }

    pub const fn waveform_count(&self) -> u64 {
        self.waveform_count
    }

    pub const fn evaluated_waveform_count(&self) -> u64 {
        self.evaluated_waveform_count
    }

    pub const fn differing_waveform_count(&self) -> u64 {
        self.differing_waveform_count
    }

    pub const fn missing_waveform_count(&self) -> u64 {
        self.missing_waveform_count
    }

    pub fn tolerance_policy_digest(&self) -> &str {
        &self.tolerance_policy_digest
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn passed(&self) -> bool {
        self.evaluated_waveform_count == self.waveform_count
            && self.missing_waveform_count == 0
            && self.differing_waveform_count == 0
    }
}

/// Immutable evidence from a completed execution. Construction validates
/// completeness and uniqueness; it never performs or fabricates a run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletedEvidence {
    plan_digest: SourceDigest,
    project_revision: String,
    run_id: String,
    checks: Vec<CheckEvidence>,
    comparison: ComparisonEvidence,
}

impl CompletedEvidence {
    pub fn try_new(
        plan_digest: SourceDigest,
        project_revision: impl Into<String>,
        run_id: impl Into<String>,
        checks: Vec<CheckEvidence>,
        comparison: ComparisonEvidence,
    ) -> Result<Self, EvidenceError> {
        let project_revision = project_revision.into();
        let run_id = run_id.into();
        validate_text("project revision", &project_revision, false, false)?;
        validate_text("run ID", &run_id, false, false)?;
        if checks.is_empty() {
            return Err(EvidenceError::NoChecks);
        }
        if checks.len() > MAX_CHECKS {
            return Err(EvidenceError::TooManyChecks {
                count: checks.len(),
                maximum: MAX_CHECKS,
            });
        }

        let comparison_text_bytes = comparison
            .baseline
            .len()
            .saturating_add(comparison.tolerance_policy_digest.len())
            .saturating_add(comparison.detail.len());
        let total_text_bytes = checks.iter().try_fold(
            project_revision
                .len()
                .saturating_add(run_id.len())
                .saturating_add(comparison_text_bytes),
            |total, check| {
                total
                    .checked_add(check.id.len())?
                    .checked_add(check.name.len())?
                    .checked_add(check.detail.len())
            },
        );
        if total_text_bytes.is_none_or(|bytes| bytes > MAX_TOTAL_EVIDENCE_BYTES) {
            return Err(EvidenceError::EvidenceTooLarge {
                maximum: MAX_TOTAL_EVIDENCE_BYTES,
            });
        }
        if checks
            .iter()
            .try_fold(0_u64, |total, check| total.checked_add(check.duration_ms))
            .is_none()
        {
            return Err(EvidenceError::DurationOverflow);
        }

        let mut ids = std::collections::BTreeSet::new();
        for check in &checks {
            if !ids.insert(check.id.clone()) {
                return Err(EvidenceError::DuplicateCheckId {
                    check_id: check.id.clone(),
                });
            }
        }
        Ok(Self {
            plan_digest,
            project_revision,
            run_id,
            checks,
            comparison,
        })
    }

    pub const fn plan_digest(&self) -> SourceDigest {
        self.plan_digest
    }

    pub fn project_revision(&self) -> &str {
        &self.project_revision
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn checks(&self) -> &[CheckEvidence] {
        &self.checks
    }

    pub const fn comparison(&self) -> &ComparisonEvidence {
        &self.comparison
    }

    pub fn passed(&self) -> bool {
        self.checks
            .iter()
            .all(|check| check.outcome == CheckOutcome::Passed)
            && self.comparison.passed()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvidenceError {
    EmptyField {
        field: &'static str,
    },
    FieldTooLarge {
        field: &'static str,
        bytes: usize,
        maximum: usize,
    },
    ControlCharacter {
        field: &'static str,
    },
    MissingFailureDetail {
        check_id: String,
    },
    NoChecks,
    TooManyChecks {
        count: usize,
        maximum: usize,
    },
    EvidenceTooLarge {
        maximum: usize,
    },
    DurationOverflow,
    DuplicateCheckId {
        check_id: String,
    },
    InvalidComparisonCounts {
        waveform_count: u64,
        differing_waveform_count: u64,
    },
    InvalidComparisonCoverage {
        waveform_count: u64,
        evaluated_waveform_count: u64,
        missing_waveform_count: u64,
    },
    MissingComparisonFailureDetail,
}

impl fmt::Display for EvidenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { field } => write!(formatter, "{field} must not be empty"),
            Self::FieldTooLarge {
                field,
                bytes,
                maximum,
            } => write!(
                formatter,
                "{field} contains {bytes} bytes; the maximum is {maximum}"
            ),
            Self::ControlCharacter { field } => {
                write!(
                    formatter,
                    "{field} contains an unsupported control character"
                )
            }
            Self::MissingFailureDetail { check_id } => {
                write!(formatter, "failed check {check_id:?} requires detail")
            }
            Self::NoChecks => formatter.write_str("completed evidence contains no checks"),
            Self::TooManyChecks { count, maximum } => {
                write!(
                    formatter,
                    "evidence contains {count} checks; maximum is {maximum}"
                )
            }
            Self::EvidenceTooLarge { maximum } => write!(
                formatter,
                "completed evidence text exceeds the {maximum}-byte aggregate limit"
            ),
            Self::DurationOverflow => formatter.write_str(
                "the aggregate check duration exceeds the supported 64-bit millisecond range",
            ),
            Self::DuplicateCheckId { check_id } => {
                write!(formatter, "duplicate check ID {check_id:?}")
            }
            Self::InvalidComparisonCounts {
                waveform_count,
                differing_waveform_count,
            } => write!(
                formatter,
                "comparison reports {differing_waveform_count} differences for only {waveform_count} waveforms"
            ),
            Self::InvalidComparisonCoverage {
                waveform_count,
                evaluated_waveform_count,
                missing_waveform_count,
            } => write!(
                formatter,
                "comparison expects {waveform_count} waveforms but reports {evaluated_waveform_count} evaluated and {missing_waveform_count} missing"
            ),
            Self::MissingComparisonFailureDetail => formatter.write_str(
                "a differing or missing waveform comparison requires retained failure detail",
            ),
        }
    }
}

impl Error for EvidenceError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderedArtifact {
    kind: ArtifactKind,
    bytes: Vec<u8>,
}

impl RenderedArtifact {
    pub const fn kind(&self) -> ArtifactKind {
        self.kind
    }

    pub const fn file_name(&self) -> &'static str {
        self.kind.file_name()
    }

    pub const fn media_type(&self) -> &'static str {
        self.kind.media_type()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactRenderError {
    PlanDigestMismatch {
        expected: SourceDigest,
        actual: SourceDigest,
    },
    BaselineMismatch {
        expected: &'static str,
        actual: String,
    },
    Json(String),
    DocumentTooLarge,
}

impl fmt::Display for ArtifactRenderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlanDigestMismatch { expected, actual } => write!(
                formatter,
                "evidence plan digest {actual} does not match compiled plan {expected}"
            ),
            Self::BaselineMismatch { expected, actual } => write!(
                formatter,
                "evidence baseline {actual:?} does not match plan baseline {expected:?}"
            ),
            Self::Json(message) => write!(formatter, "could not render summary JSON: {message}"),
            Self::DocumentTooLarge => {
                formatter.write_str("verification document exceeds PDF object limits")
            }
        }
    }
}

impl Error for ArtifactRenderError {}

/// Render exactly the artifacts requested by `plan`, in stable artifact order.
/// Evidence identity and comparison binding are checked before any bytes are
/// produced.
pub fn render_requested_artifacts(
    plan: &AutomationPlan,
    evidence: &CompletedEvidence,
) -> Result<Vec<RenderedArtifact>, ArtifactRenderError> {
    if evidence.plan_digest != plan.source_digest() {
        return Err(ArtifactRenderError::PlanDigestMismatch {
            expected: plan.source_digest(),
            actual: evidence.plan_digest,
        });
    }
    if evidence.comparison.baseline != plan.baseline() {
        return Err(ArtifactRenderError::BaselineMismatch {
            expected: plan.baseline(),
            actual: evidence.comparison.baseline.clone(),
        });
    }

    plan.artifacts()
        .map(|kind| {
            let bytes = match kind {
                ArtifactKind::JunitXml => render_junit(plan, evidence).into_bytes(),
                ArtifactKind::SummaryJson => render_summary_json(plan, evidence)?,
                ArtifactKind::VerificationPdf => render_verification_pdf(plan, evidence)?,
            };
            Ok(RenderedArtifact { kind, bytes })
        })
        .collect()
}

fn validate_text(
    field: &'static str,
    value: &str,
    allow_empty: bool,
    allow_multiline: bool,
) -> Result<(), EvidenceError> {
    if !allow_empty && value.trim().is_empty() {
        return Err(EvidenceError::EmptyField { field });
    }
    if value.len() > MAX_EVIDENCE_TEXT_BYTES {
        return Err(EvidenceError::FieldTooLarge {
            field,
            bytes: value.len(),
            maximum: MAX_EVIDENCE_TEXT_BYTES,
        });
    }
    if value.chars().any(|character| {
        character.is_control() && !(allow_multiline && matches!(character, '\n' | '\r' | '\t'))
    }) {
        return Err(EvidenceError::ControlCharacter { field });
    }
    Ok(())
}

fn render_junit(plan: &AutomationPlan, evidence: &CompletedEvidence) -> String {
    let check_failures = evidence
        .checks
        .iter()
        .filter(|check| check.outcome == CheckOutcome::Failed)
        .count();
    let comparison_failed = !evidence.comparison.passed();
    let failures = check_failures + usize::from(comparison_failed);
    let duration_ms = evidence
        .checks
        .iter()
        .map(|check| check.duration_ms)
        .sum::<u64>();
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str(&format!(
        "<testsuite name=\"{}\" tests=\"{}\" failures=\"{}\" errors=\"0\" time=\"{}\">\n",
        escape_xml_attribute(plan.project_name()),
        evidence.checks.len() + 1,
        failures,
        format_duration(duration_ms)
    ));
    xml.push_str("  <properties>\n");
    xml.push_str(&format!(
        "    <property name=\"rspice.plan_digest\" value=\"{}\"/>\n",
        plan.source_digest()
    ));
    xml.push_str(&format!(
        "    <property name=\"rspice.project_revision\" value=\"{}\"/>\n",
        escape_xml_attribute(&evidence.project_revision)
    ));
    xml.push_str(&format!(
        "    <property name=\"rspice.run_id\" value=\"{}\"/>\n",
        escape_xml_attribute(&evidence.run_id)
    ));
    xml.push_str(&format!(
        "    <property name=\"rspice.tolerance_policy_digest\" value=\"{}\"/>\n",
        escape_xml_attribute(&evidence.comparison.tolerance_policy_digest)
    ));
    xml.push_str("  </properties>\n");
    for check in &evidence.checks {
        xml.push_str(&format!(
            "  <testcase classname=\"{}\" name=\"{}\" id=\"{}\" time=\"{}\">",
            escape_xml_attribute(plan.project_name()),
            escape_xml_attribute(&check.name),
            escape_xml_attribute(&check.id),
            format_duration(check.duration_ms)
        ));
        match check.outcome {
            CheckOutcome::Passed if check.detail.is_empty() => xml.push_str("</testcase>\n"),
            CheckOutcome::Passed => {
                xml.push_str("<system-out>");
                xml.push_str(&escape_xml_text(&check.detail));
                xml.push_str("</system-out></testcase>\n");
            }
            CheckOutcome::Failed => {
                xml.push_str("<failure message=\"verification check failed\">");
                xml.push_str(&escape_xml_text(&check.detail));
                xml.push_str("</failure></testcase>\n");
            }
        }
    }
    xml.push_str(&format!(
        "  <testcase classname=\"{}\" name=\"Golden waveform comparison\" id=\"rspice.automation.baseline-comparison\" time=\"0.000\">",
        escape_xml_attribute(plan.project_name())
    ));
    if comparison_failed {
        xml.push_str("<failure message=\"baseline comparison failed\">");
        xml.push_str(&escape_xml_text(&evidence.comparison.detail));
        xml.push_str("</failure></testcase>\n");
    } else if evidence.comparison.detail.is_empty() {
        xml.push_str("</testcase>\n");
    } else {
        xml.push_str("<system-out>");
        xml.push_str(&escape_xml_text(&evidence.comparison.detail));
        xml.push_str("</system-out></testcase>\n");
    }
    xml.push_str("</testsuite>\n");
    xml
}

fn format_duration(milliseconds: u64) -> String {
    format!("{}.{:03}", milliseconds / 1_000, milliseconds % 1_000)
}

fn escape_xml_attribute(value: &str) -> String {
    escape_xml(value, true)
}

fn escape_xml_text(value: &str) -> String {
    escape_xml(value, false)
}

fn escape_xml(value: &str, attribute: bool) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' if attribute => output.push_str("&quot;"),
            '\'' if attribute => output.push_str("&apos;"),
            '\r' => output.push_str("&#13;"),
            _ => output.push(character),
        }
    }
    output
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Summary<'evidence> {
    schema_version: &'static str,
    plan_digest: String,
    project: &'evidence str,
    project_revision: &'evidence str,
    run_id: &'evidence str,
    outcome: &'static str,
    workflow: SummaryWorkflow,
    checks: SummaryChecks<'evidence>,
    comparison: SummaryComparison<'evidence>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SummaryWorkflow {
    corners: &'static str,
    target: &'static str,
    required_specs: &'static str,
    compare_waveforms: bool,
    requested_artifacts: Vec<&'static str>,
}

#[derive(Serialize)]
struct SummaryChecks<'evidence> {
    total: usize,
    passed: usize,
    failed: usize,
    items: Vec<SummaryCheck<'evidence>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SummaryCheck<'evidence> {
    id: &'evidence str,
    name: &'evidence str,
    outcome: CheckOutcome,
    duration_ms: u64,
    detail: &'evidence str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SummaryComparison<'evidence> {
    baseline: &'evidence str,
    waveform_targets: u64,
    evaluated_waveforms: u64,
    differing_waveforms: u64,
    missing_waveforms: u64,
    tolerance_policy_digest: &'evidence str,
    detail: &'evidence str,
}

fn render_summary_json(
    plan: &AutomationPlan,
    evidence: &CompletedEvidence,
) -> Result<Vec<u8>, ArtifactRenderError> {
    let passed = evidence
        .checks
        .iter()
        .filter(|check| check.outcome == CheckOutcome::Passed)
        .count();
    let failed = evidence.checks.len() - passed;
    let summary = Summary {
        schema_version: "rspice.automation.summary/1",
        plan_digest: plan.source_digest().to_hex(),
        project: plan.project_name(),
        project_revision: &evidence.project_revision,
        run_id: &evidence.run_id,
        outcome: if evidence.passed() {
            "passed"
        } else {
            "failed"
        },
        workflow: SummaryWorkflow {
            corners: plan.corners(),
            target: plan.target(),
            required_specs: plan.required_specs(),
            compare_waveforms: plan.compare_waveforms(),
            requested_artifacts: plan.artifacts().map(ArtifactKind::file_name).collect(),
        },
        checks: SummaryChecks {
            total: evidence.checks.len(),
            passed,
            failed,
            items: evidence
                .checks
                .iter()
                .map(|check| SummaryCheck {
                    id: &check.id,
                    name: &check.name,
                    outcome: check.outcome,
                    duration_ms: check.duration_ms,
                    detail: &check.detail,
                })
                .collect(),
        },
        comparison: SummaryComparison {
            baseline: &evidence.comparison.baseline,
            waveform_targets: evidence.comparison.waveform_count,
            evaluated_waveforms: evidence.comparison.evaluated_waveform_count,
            differing_waveforms: evidence.comparison.differing_waveform_count,
            missing_waveforms: evidence.comparison.missing_waveform_count,
            tolerance_policy_digest: &evidence.comparison.tolerance_policy_digest,
            detail: &evidence.comparison.detail,
        },
    };
    let mut bytes = serde_json::to_vec_pretty(&summary)
        .map_err(|error| ArtifactRenderError::Json(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn render_verification_pdf(
    plan: &AutomationPlan,
    evidence: &CompletedEvidence,
) -> Result<Vec<u8>, ArtifactRenderError> {
    let mut lines = vec![
        "RSpice Verification Report".to_owned(),
        String::new(),
        format!("Project: {}", plan.project_name()),
        format!("Project revision: {}", evidence.project_revision),
        format!("Run ID: {}", evidence.run_id),
        format!("Plan SHA-256: {}", plan.source_digest()),
        format!(
            "Result: {}",
            if evidence.passed() {
                "PASSED"
            } else {
                "FAILED"
            }
        ),
        format!("Required specifications: {}", plan.required_specs()),
        format!("Execution target: {}", plan.target()),
        format!("Comparison baseline: {}", evidence.comparison.baseline),
        format!(
            "Waveform targets: {} ({} evaluated, {} differing, {} missing)",
            evidence.comparison.waveform_count,
            evidence.comparison.evaluated_waveform_count,
            evidence.comparison.differing_waveform_count,
            evidence.comparison.missing_waveform_count
        ),
        format!(
            "Tolerance policy SHA-256: {}",
            evidence.comparison.tolerance_policy_digest
        ),
        format!("Comparison detail: {}", evidence.comparison.detail),
        String::new(),
        "Verification checks".to_owned(),
    ];
    for check in &evidence.checks {
        let outcome = match check.outcome {
            CheckOutcome::Passed => "PASS",
            CheckOutcome::Failed => "FAIL",
        };
        lines.push(format!(
            "[{outcome}] {} - {} ({} ms)",
            check.id, check.name, check.duration_ms
        ));
        if !check.detail.is_empty() {
            lines.push(format!("  {}", check.detail));
        }
    }

    let display_lines = lines
        .into_iter()
        .flat_map(|line| wrap_pdf_line(&ascii_pdf_display(&line), PDF_WRAP_COLUMNS))
        .collect::<Vec<_>>();
    build_pdf(&display_lines)
}

fn ascii_pdf_display(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '\n' | '\r' | '\t' => output.push(' '),
            _ if character.is_ascii_graphic() || character == ' ' => output.push(character),
            _ => {
                use fmt::Write as _;
                let _ = write!(output, "\\u{{{:X}}}", u32::from(character));
            }
        }
    }
    output
}

fn wrap_pdf_line(value: &str, columns: usize) -> Vec<String> {
    if value.is_empty() {
        return vec![String::new()];
    }
    let characters = value.chars().collect::<Vec<_>>();
    characters
        .chunks(columns)
        .map(|chunk| chunk.iter().collect())
        .collect()
}

fn escape_pdf_literal(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'(' => output.push_str("\\("),
            b')' => output.push_str("\\)"),
            b'\\' => output.push_str("\\\\"),
            0x20..=0x7e => output.push(char::from(byte)),
            _ => {
                use fmt::Write as _;
                let _ = write!(output, "\\{byte:03o}");
            }
        }
    }
    output
}

fn build_pdf(lines: &[String]) -> Result<Vec<u8>, ArtifactRenderError> {
    let page_count = lines.len().max(1).div_ceil(PDF_LINES_PER_PAGE);
    let object_count = 3_usize
        .checked_add(
            page_count
                .checked_mul(2)
                .ok_or(ArtifactRenderError::DocumentTooLarge)?,
        )
        .ok_or(ArtifactRenderError::DocumentTooLarge)?;
    if object_count > 9_999_999 {
        return Err(ArtifactRenderError::DocumentTooLarge);
    }

    let mut objects = Vec::<Vec<u8>>::with_capacity(object_count);
    objects.push(b"<< /Type /Catalog /Pages 2 0 R >>".to_vec());
    let kids = (0..page_count)
        .map(|page| format!("{} 0 R", 4 + page * 2))
        .collect::<Vec<_>>()
        .join(" ");
    objects.push(format!("<< /Type /Pages /Kids [{kids}] /Count {page_count} >>").into_bytes());
    objects.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());

    for page_index in 0..page_count {
        let page_object = 4 + page_index * 2;
        let content_object = page_object + 1;
        objects.push(
            format!(
                "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 3 0 R >> >> /Contents {content_object} 0 R >>"
            )
            .into_bytes(),
        );
        let start = page_index * PDF_LINES_PER_PAGE;
        let end = (start + PDF_LINES_PER_PAGE).min(lines.len());
        let mut stream = String::from("BT\n/F1 10 Tf\n");
        let mut y = 752;
        for line in &lines[start..end] {
            stream.push_str(&format!(
                "1 0 0 1 48 {y} Tm\n({}) Tj\n",
                escape_pdf_literal(line)
            ));
            y -= 16;
        }
        if start == end {
            stream.push_str("1 0 0 1 48 752 Tm\n() Tj\n");
        }
        stream.push_str("ET\n");
        objects.push(
            format!(
                "<< /Length {} >>\nstream\n{}endstream",
                stream.len(),
                stream
            )
            .into_bytes(),
        );
    }

    let mut pdf = b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n".to_vec();
    let mut offsets = Vec::with_capacity(objects.len() + 1);
    offsets.push(0);
    for (index, object) in objects.iter().enumerate() {
        offsets.push(pdf.len());
        pdf.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
        pdf.extend_from_slice(object);
        pdf.extend_from_slice(b"\nendobj\n");
    }
    let xref = pdf.len();
    pdf.extend_from_slice(format!("xref\n0 {}\n", objects.len() + 1).as_bytes());
    pdf.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.into_iter().skip(1) {
        pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    pdf.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n",
            objects.len() + 1
        )
        .as_bytes(),
    );
    Ok(pdf)
}

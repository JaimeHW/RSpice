//! The sign-off report: what Check Design found in the configured design, as
//! a plain-text summary in the spirit of a verification run's `.summary` and
//! a CSV with one row per finding.
//!
//! It is built from a check of the design as it stands when it is built,
//! never from findings the design has moved away from: building it runs
//! Check Design wherever no current check describes the design. Its verdict
//! is the run gate's: it fails on any Error or Critical finding no waiver
//! covers, and on nothing else.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use super::{AppState, CellViewCheckReceipt, CheckScope};
use crate::state::{CanonicalCellViewOwnerKey, CellViewRef};
use rspice_design::drc::{DrcLocation, DrcSeverity, DrcViolation, waiver_site};

/// One sign-off report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SignoffReport {
    /// The plain-text summary.
    pub(crate) summary: String,
    /// A header row, then one row per finding, as RFC 4180 CSV.
    pub(crate) detail: String,
    /// Whether no finding that counts is an Error or Critical.
    pub(crate) passed: bool,
}

/// The CSV detail's header row.
const DETAIL_HEADER: [&str; 10] = [
    "rule",
    "severity",
    "cellview",
    "occurrence",
    "object",
    "message",
    "waived",
    "waiver_reason",
    "waiver_author",
    "waiver_date",
];

impl AppState {
    /// Check Design where no current check describes the configured design,
    /// and the sign-off report of what it found, dated `date`.
    pub(crate) fn signoff_report(&mut self, date: &str) -> Result<SignoffReport, String> {
        let root = self.workspace.content.simulation_root_reference();
        self.ensure_design_check(root.clone(), CheckScope::Design)?;
        let receipt = self
            .design_checks
            .receipt(&root)
            .filter(|receipt| receipt.scope == CheckScope::Design)
            .ok_or_else(|| format!("{} has no Check Design result", root.display_path()))?;
        let reached = super::reached_cellviews(self, &root)?
            .iter()
            .map(super::owner_key)
            .collect();
        Ok(SignoffReport {
            summary: summary(self, &root, receipt, &reached, date),
            detail: detail(&root, receipt),
            passed: !receipt.result.has_errors(),
        })
    }
}

/// How many findings of one rule count against the design, and how many a
/// waiver covers.
#[derive(Debug, Clone, Copy, Default)]
struct RuleCounts {
    errors: usize,
    errors_waived: usize,
    warnings: usize,
    warnings_waived: usize,
}

/// The summary of `receipt`, Check Design's pass over `root`, which reached
/// the cellviews `reached` holds.
fn summary(
    state: &AppState,
    root: &CellViewRef,
    receipt: &CellViewCheckReceipt,
    reached: &BTreeSet<CanonicalCellViewOwnerKey>,
    date: &str,
) -> String {
    let result = &receipt.result;
    let mut text = String::new();
    let _ = writeln!(text, "RSpice design sign-off summary");
    let _ = writeln!(text);
    for (label, value) in [
        ("Design root", root.display_path()),
        ("Configuration", configuration(state)),
        (
            "Project revision",
            state.workspace.content.project.revision().get().to_string(),
        ),
        ("Input digest", receipt.input_digest.to_string()),
        ("Date", date.to_owned()),
        ("RSpice version", env!("CARGO_PKG_VERSION").to_owned()),
    ] {
        let _ = writeln!(text, "{label:<18}{value}");
    }

    let _ = writeln!(text);
    let _ = writeln!(text, "Rule deck overrides");
    let mut overrides = state
        .workspace
        .content
        .design_rules
        .severity_overrides()
        .into_iter()
        .filter(|(kind, severity)| *severity != kind.rule().default_severity)
        .map(|(kind, severity)| (kind.rule_code(), kind.rule().default_severity, severity))
        .collect::<Vec<_>>();
    overrides.sort_by_key(|(code, _, _)| *code);
    if overrides.is_empty() {
        let _ = writeln!(text, "  none: every rule at its default severity");
    }
    for (code, default, severity) in overrides {
        let _ = writeln!(
            text,
            "  {code:<32}{} -> {}",
            default.display_name(),
            severity.display_name()
        );
    }

    let mut rules = BTreeMap::<&'static str, RuleCounts>::new();
    for violation in result.violations() {
        let counts = rules
            .entry(violation.violation_type.rule_code())
            .or_default();
        let waived = !violation.counts();
        match violation.severity {
            DrcSeverity::Critical | DrcSeverity::Error if waived => counts.errors_waived += 1,
            DrcSeverity::Critical | DrcSeverity::Error => counts.errors += 1,
            DrcSeverity::Warning if waived => counts.warnings_waived += 1,
            DrcSeverity::Warning => counts.warnings += 1,
            DrcSeverity::Info => {}
        }
    }
    rules.retain(|_, counts| {
        counts.errors + counts.errors_waived + counts.warnings + counts.warnings_waived > 0
    });
    let _ = writeln!(text);
    let _ = writeln!(
        text,
        "  {:<32}{:>8}{:>8}{:>10}{:>8}",
        "Rule", "Errors", "Waived", "Warnings", "Waived"
    );
    let mut total = RuleCounts::default();
    for (code, counts) in &rules {
        let _ = writeln!(
            text,
            "  {code:<32}{:>8}{:>8}{:>10}{:>8}",
            counts.errors, counts.errors_waived, counts.warnings, counts.warnings_waived
        );
        total.errors += counts.errors;
        total.errors_waived += counts.errors_waived;
        total.warnings += counts.warnings;
        total.warnings_waived += counts.warnings_waived;
    }
    let _ = writeln!(
        text,
        "  {:<32}{:>8}{:>8}{:>10}{:>8}",
        "Total", total.errors, total.errors_waived, total.warnings, total.warnings_waived
    );

    let _ = writeln!(text);
    let _ = writeln!(text, "Unmatched waivers");
    let waivers = state.design_check_config(root).waivers;
    // Only where the check looked: a waiver stored with a schematic the
    // design never reaches — another testbench, a cell nothing places — is
    // about findings this check does not compute, and says nothing stale.
    let mut unmatched = waivers.unmatched_anywhere(result);
    unmatched.retain(|(stored_with, _)| stored_with.is_none_or(|key| reached.contains(key)));
    if unmatched.is_empty() {
        let _ = writeln!(text, "  none");
    }
    for (stored_with, waiver) in unmatched {
        let cellview = stored_with.map_or_else(|| root.display_path(), ToString::to_string);
        let _ = writeln!(
            text,
            "  {cellview}  {}  {}  \"{}\" ({}, {})",
            waiver.rule,
            waiver.object,
            waiver.reason,
            waiver.author,
            utc_date(waiver.waived_at_ms)
        );
    }

    let _ = writeln!(text);
    if total.errors == 0 {
        let _ = writeln!(text, "RESULT: PASS");
    } else {
        let _ = writeln!(
            text,
            "RESULT: FAIL ({} unwaived error{})",
            total.errors,
            if total.errors == 1 { "" } else { "s" }
        );
    }
    text
}

/// The configuration the root's hierarchy is resolved under.
fn configuration(state: &AppState) -> String {
    let resolution = state.workspace.resolve_hierarchy(&state.library_manager);
    resolution
        .configuration_id
        .and_then(|id| state.workspace.content.configuration_sets.find(id))
        .map_or_else(
            || "none: each instance bound as its own drawing binds it".to_owned(),
            |configuration| {
                format!(
                    "{}, revision {}",
                    configuration.name(),
                    configuration.revision()
                )
            },
        )
}

fn detail(root: &CellViewRef, receipt: &CellViewCheckReceipt) -> String {
    let mut text = String::new();
    push_row(
        &mut text,
        DETAIL_HEADER.iter().map(|field| (*field).to_owned()),
    );
    for violation in receipt.result.violations() {
        push_row(&mut text, detail_row(root, violation));
    }
    text
}

fn detail_row(root: &CellViewRef, violation: &DrcViolation) -> [String; 10] {
    let cellview = waiver_site(&violation.location)
        .0
        .unwrap_or(root)
        .display_path();
    let (occurrence, object) = match &violation.location {
        DrcLocation::InsideInstance { path, inner, .. } => {
            (path.to_string(), leaf(inner).display())
        }
        location => (String::new(), location.display()),
    };
    let waiver = violation.waiver.as_ref();
    [
        violation.violation_type.rule_code().to_owned(),
        violation.severity.display_name().to_owned(),
        cellview,
        occurrence,
        object,
        violation.message.clone(),
        if waiver.is_some() { "yes" } else { "no" }.to_owned(),
        waiver.map_or_else(String::new, |waiver| waiver.reason.clone()),
        waiver.map_or_else(String::new, |waiver| waiver.author.clone()),
        waiver.map_or_else(String::new, |waiver| utc_date(waiver.waived_at_ms)),
    ]
}

/// The object a location names inside every occurrence it is stated in.
fn leaf(location: &DrcLocation) -> &DrcLocation {
    match location {
        DrcLocation::InsideInstance { inner, .. } => leaf(inner),
        location => location,
    }
}

fn push_row(text: &mut String, fields: impl IntoIterator<Item = String>) {
    for (index, field) in fields.into_iter().enumerate() {
        if index > 0 {
            text.push(',');
        }
        text.push_str(&csv_field(&field));
    }
    text.push_str("\r\n");
}

/// One field as RFC 4180 writes it: quoted, with its quotes doubled, where
/// it holds a separator, a quote or a line break.
fn csv_field(field: &str) -> std::borrow::Cow<'_, str> {
    if field.contains([',', '"', '\r', '\n']) {
        std::borrow::Cow::Owned(format!("\"{}\"", field.replace('"', "\"\"")))
    } else {
        std::borrow::Cow::Borrowed(field)
    }
}

/// A waiver's recording time, in UTC.
fn utc_date(milliseconds: u64) -> String {
    rspice_hardcopy::render::HardcopyPublicationTimestamp::from_unix_seconds(milliseconds / 1000)
        .map_or_else(
            |_| String::new(),
            |stamp| {
                format!(
                    "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                    stamp.year, stamp.month, stamp.day, stamp.hour, stamp.minute, stamp.second
                )
            },
        )
}

#[cfg(test)]
mod tests;

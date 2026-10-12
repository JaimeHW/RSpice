//! One cell's outcome: `fail`, `refused`, `wrong`, `pass` or `unverified`,
//! decided in that order (design §2.7).
//!
//! A refusal is an outcome only when it is precise. It must carry a capability
//! token (or an elaboration/parse kind), a reason beyond the token, the
//! instance or module it is about, and the analysis it refuses — by structured
//! identity, by a label that names *this* card's analysis, or, for an
//! elaboration or parse refusal, by the source location that refuses every
//! analysis of the variant. A refusal missing any of the four is
//! `wrong(malformed-refusal:<missing>)`; one whose label names a different
//! analysis is `wrong(misnamed-analysis)`.
//!
//! [`REQUIRE_STRUCTURED_REFUSAL`] is off at P0.5 (amendment A2): an instance
//! or analysis found only in the prose counts, recorded as `form=prose`. It
//! flips at P1.2's exit, after which prose identities are malformed.

use super::Outcome;
use super::case::{CaseResult, ErrorRecord};
use super::compare::Verdict;

/// Flip at the exit of P1.2 (design §7 D2 / amendment A2).
pub const REQUIRE_STRUCTURED_REFUSAL: bool = false;

/// The fixed analysis labels a refusal may use, per card kind.
pub fn analysis_labels(kind: &str) -> &'static [&'static str] {
    match kind {
        "op" => &["operating point", "operating-point", ".op"],
        "dc" => &["dc sweep", ".dc"],
        "ac" => &["ac analysis", ".ac"],
        "tf" => &["transfer function", ".tf"],
        "noise" => &["noise analysis", ".noise"],
        "stb" => &["stb analysis", ".stb"],
        "pz" => &["pole-zero analysis", ".pz"],
        "sens" => &["sensitivity analysis", ".sens"],
        "pss" => &["pss analysis", ".pss"],
        "pac" => &["pac analysis", ".pac"],
        "pnoise" => &["pnoise analysis", "pss noise analysis", ".pnoise"],
        "tran" | "trnoise" | "ckpt" | "mc-tran" | "step-tran" => &["transient", ".tran"],
        _ => &[],
    }
}

const LABELLED_KINDS: [&str; 12] = [
    "op", "dc", "ac", "tf", "noise", "stb", "pz", "sens", "pss", "pac", "pnoise", "tran",
];

/// How the case process ended, before any result file is read.
#[derive(Debug, Clone)]
pub enum CaseEnd {
    Completed(Box<CaseResult>),
    Crash(String),
    Timeout(f64),
}

/// The verdicts of one quantity against one oracle.
#[derive(Debug, Clone)]
pub struct QuantityVerdict {
    pub quantity: String,
    pub oracle: String,
    pub outcome: QuantityOutcome,
}

#[derive(Debug, Clone)]
pub enum QuantityOutcome {
    Compared(Verdict),
    /// The oracle's reference is not available (O2/O3 before P0.3).
    OracleAbsent(String),
    /// RSpice delivered no value for a declared signal: silent omission.
    Missing(String),
    /// The quantity could not be evaluated on RSpice's result.
    Unmeasurable(String),
    /// The oracle side could not be evaluated: a corpus defect, not RSpice's.
    OracleError(String),
}

/// The classified cell.
#[derive(Debug, Clone, PartialEq)]
pub struct Classification {
    pub outcome: Outcome,
    pub detail: String,
}

impl Classification {
    fn new(outcome: Outcome, detail: impl Into<String>) -> Self {
        Self {
            outcome,
            detail: detail.into(),
        }
    }
}

/// Classify one cell. `kind` is the analysis kind; `instances` are the deck's
/// instance names (lower case), used to recognise an instance named in prose.
pub fn classify(
    end: &CaseEnd,
    kind: &str,
    instances: &[String],
    verdicts: &[QuantityVerdict],
) -> Classification {
    let result = match end {
        CaseEnd::Crash(why) => return Classification::new(Outcome::Fail, format!("crash: {why}")),
        CaseEnd::Timeout(seconds) => {
            return Classification::new(Outcome::Fail, format!("timeout after {seconds:.0} s"));
        }
        CaseEnd::Completed(result) => result,
    };
    if result.outcome != "completed" {
        let Some(error) = &result.error else {
            return Classification::new(Outcome::Fail, "error outcome without an error record");
        };
        return classify_error(error, kind, instances);
    }
    if result.dispatch == "none" {
        return Classification::new(Outcome::Fail, "no-dispatch");
    }
    // Completed: non-finite or silently omitted values are wrong outright.
    for observation in &result.results {
        if let Some(name) = observation.nonfinite.first().or_else(|| {
            observation
                .trials
                .iter()
                .find_map(|trial| trial.nonfinite.first())
        }) {
            return Classification::new(Outcome::Wrong, format!("non-finite: {name}"));
        }
    }
    let mut worst_failed: Option<(&QuantityVerdict, &Verdict)> = None;
    let mut worst_passed: Option<(&QuantityVerdict, &Verdict)> = None;
    let mut compared = 0usize;
    let mut absent = Vec::new();
    for verdict in verdicts {
        match &verdict.outcome {
            QuantityOutcome::Missing(what) => {
                return Classification::new(
                    Outcome::Wrong,
                    format!(
                        "silent-omission: {} ({}) {what}",
                        verdict.quantity, verdict.oracle
                    ),
                );
            }
            QuantityOutcome::Unmeasurable(why) => {
                return Classification::new(
                    Outcome::Wrong,
                    format!(
                        "unmeasurable: {} ({}) {why}",
                        verdict.quantity, verdict.oracle
                    ),
                );
            }
            QuantityOutcome::OracleError(why) => {
                return Classification::new(
                    Outcome::Fail,
                    format!(
                        "oracle-error: {} ({}) {why}",
                        verdict.quantity, verdict.oracle
                    ),
                );
            }
            QuantityOutcome::OracleAbsent(why) => {
                absent.push(format!("{}:{} {why}", verdict.quantity, verdict.oracle))
            }
            QuantityOutcome::Compared(compared_verdict) => {
                compared += 1;
                let slot = if compared_verdict.pass {
                    &mut worst_passed
                } else {
                    &mut worst_failed
                };
                if slot.is_none_or(|(_, worst)| compared_verdict.severity > worst.severity) {
                    *slot = Some((verdict, compared_verdict));
                }
            }
        }
    }
    if let Some((quantity, verdict)) = worst_failed {
        return Classification::new(
            Outcome::Wrong,
            format!(
                "{} ({}) {}",
                quantity.quantity, quantity.oracle, verdict.detail
            ),
        );
    }
    if compared > 0 {
        let detail = worst_passed
            .map(|(quantity, verdict)| {
                format!(
                    "{} ({}) {}",
                    quantity.quantity, quantity.oracle, verdict.detail
                )
            })
            .unwrap_or_default();
        let detail = if absent.is_empty() {
            detail
        } else {
            format!("{detail}; pending {}", absent.len())
        };
        return Classification::new(Outcome::Pass, detail);
    }
    let detail = if verdicts.is_empty() {
        "no quantity declared for this cell".to_owned()
    } else {
        format!("oracle absent: {}", absent.join("; "))
    };
    Classification::new(Outcome::Unverified, detail)
}

fn classify_error(error: &ErrorRecord, kind: &str, instances: &[String]) -> Classification {
    let refusal_category = matches!(error.category.as_str(), "capability" | "elaboration")
        || (error.category == "parse" && error.token.is_some());
    if !refusal_category {
        let code = error.code.as_deref().unwrap_or(&error.category);
        return Classification::new(
            Outcome::Fail,
            format!("{}: {code}: {}", error.category, first_line(&error.display)),
        );
    }
    let token = error.token.as_deref().unwrap_or_default();
    let mut missing = Vec::new();
    if token.is_empty() {
        missing.push("token");
    }
    let reason = error.detail.trim();
    if reason.is_empty() || reason.eq_ignore_ascii_case(token) {
        missing.push("reason");
    }
    // The refusal's own detail, not the message a wrapper composed around it:
    // a frontend prefix such as "DC transfer-function analysis error:" names
    // the request the user made, not the analysis the refusal says it refused.
    let lower_detail = error.detail.to_ascii_lowercase();
    let (instance, instance_form) = if let Some(instance) =
        error.instance.as_ref().or(error.module.as_ref())
    {
        (Some(instance.clone()), "structured")
    } else if let Some(instance) = instances.iter().find(|name| quoted_in(&lower_detail, name)) {
        (Some(instance.clone()), "prose")
    } else {
        (None, "")
    };
    if instance.is_none() || (REQUIRE_STRUCTURED_REFUSAL && instance_form == "prose") {
        missing.push("instance");
    }
    let stage_refusal = matches!(error.category.as_str(), "elaboration" | "parse");
    let (analysis_form, label) = if let Some(analysis) = &error.analysis {
        ("structured", analysis.clone())
    } else if stage_refusal && error.location.is_some() {
        ("location", error.location.clone().unwrap_or_default())
    } else if let Some(label) = analysis_labels(kind)
        .iter()
        .find(|label| contains_label(&lower_detail, label))
    {
        ("prose", (*label).to_owned())
    } else {
        let other = LABELLED_KINDS
            .iter()
            .filter(|other| **other != kind && !analysis_labels(kind).is_empty())
            .find_map(|other| {
                analysis_labels(other)
                    .iter()
                    .find(|label| !label.starts_with('.') && contains_label(&lower_detail, label))
                    .map(|label| (*label).to_owned())
            });
        if let Some(other) = other {
            return Classification::new(
                Outcome::Wrong,
                format!("misnamed-analysis: {token}; label=\"{other}\" on a {kind} card"),
            );
        }
        ("", String::new())
    };
    if analysis_form.is_empty() || (REQUIRE_STRUCTURED_REFUSAL && analysis_form == "prose") {
        missing.push("analysis");
    }
    if !missing.is_empty() {
        return Classification::new(
            Outcome::Wrong,
            format!(
                "malformed-refusal:{}; {token}: {}",
                missing.join(","),
                first_line(&error.display)
            ),
        );
    }
    let token_form = error.token_form.as_deref().unwrap_or("structured");
    Classification::new(
        Outcome::Refused,
        format!(
            "{token}; label=\"{label}\"; instance={} form={instance_form}; token-form={token_form}; analysis-form={analysis_form}{}",
            instance.unwrap_or_default(),
            error
                .location
                .as_ref()
                .map(|location| format!("; site={location}"))
                .unwrap_or_default()
        ),
    )
}

/// Whether `label` starts a word in `text` ("AC analysis" is not inside
/// "PAC analysis").
fn contains_label(text: &str, label: &str) -> bool {
    text.match_indices(label).any(|(start, _)| {
        text[..start]
            .chars()
            .next_back()
            .is_none_or(|before| !before.is_ascii_alphanumeric())
    })
}

/// Whether `name` appears in `text` as a whole token in quotes.
fn quoted_in(text: &str, name: &str) -> bool {
    ["'", "\"", "`"]
        .iter()
        .any(|quote| text.contains(&format!("{quote}{name}{quote}")))
        || ["'", "\"", "`"].iter().any(|quote| {
            // Hierarchical names quote the full path; accept the instance as the
            // final or first path segment of a quoted name.
            text.match_indices(quote).any(|(start, _)| {
                let rest = &text[start + 1..];
                let Some(end) = rest.find(quote) else {
                    return false;
                };
                let quoted = &rest[..end];
                quoted.split(['.', ':']).any(|segment| segment == name)
            })
        })
}

fn first_line(text: &str) -> String {
    let line = text.lines().next().unwrap_or_default();
    let mut line: String = line.chars().take(240).collect();
    line = line.replace('\t', " ");
    line
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suites::mixed::case::Observation;

    fn refusal(display: &str) -> CaseEnd {
        let mut result = CaseResult::new("interp", "simulation");
        result = result.failed(ErrorRecord {
            category: "capability".into(),
            token: Some("analysis.mixed_signal.discrete_state".into()),
            token_form: Some("prose".into()),
            detail: display.into(),
            display: display.into(),
            stage: Some("execution".into()),
            ..Default::default()
        });
        CaseEnd::Completed(Box::new(result))
    }

    #[test]
    fn a_precise_prose_refusal_is_refused_at_p05() {
        let end = refusal(
            "AC analysis cannot run with mixed-signal instance 'xq' holding discrete state",
        );
        let class = classify(&end, "ac", &["xq".into()], &[]);
        assert_eq!(class.outcome, Outcome::Refused, "{}", class.detail);
        assert!(class.detail.contains("form=prose"), "{}", class.detail);
    }

    #[test]
    fn a_refusal_naming_another_analysis_is_misnamed() {
        let end = refusal(
            "AC analysis cannot run with mixed-signal instance 'xq' holding discrete state",
        );
        let class = classify(&end, "tf", &["xq".into()], &[]);
        assert_eq!(class.outcome, Outcome::Wrong);
        assert!(
            class.detail.starts_with("misnamed-analysis"),
            "{}",
            class.detail
        );
    }

    #[test]
    fn a_refusal_without_an_instance_is_malformed() {
        let end = refusal("AC analysis cannot run with mixed-signal discrete state");
        let class = classify(&end, "ac", &["xq".into()], &[]);
        assert_eq!(class.outcome, Outcome::Wrong);
        assert!(
            class.detail.starts_with("malformed-refusal:instance"),
            "{}",
            class.detail
        );
    }

    #[test]
    fn non_capability_errors_and_crashes_fail() {
        let mut result = CaseResult::new("interp", "simulation");
        result = result.failed(ErrorRecord {
            category: "convergence".into(),
            detail: "Newton gave up".into(),
            display: "Convergence failed".into(),
            ..Default::default()
        });
        assert_eq!(
            classify(&CaseEnd::Completed(Box::new(result)), "op", &[], &[]).outcome,
            Outcome::Fail
        );
        assert_eq!(
            classify(&CaseEnd::Crash("no file".into()), "op", &[], &[]).outcome,
            Outcome::Fail
        );
        assert_eq!(
            classify(&CaseEnd::Timeout(900.0), "op", &[], &[]).outcome,
            Outcome::Fail
        );
    }

    #[test]
    fn completed_cells_pass_fail_or_stay_unverified() {
        let mut result = CaseResult::new("interp", "simulation");
        result.results.push(Observation::default());
        let end = CaseEnd::Completed(Box::new(result.clone()));
        let passed = QuantityVerdict {
            quantity: "code".into(),
            oracle: "O6".into(),
            outcome: QuantityOutcome::Compared(Verdict {
                pass: true,
                severity: 0.5,
                detail: "ok".into(),
            }),
        };
        let absent = QuantityVerdict {
            quantity: "code".into(),
            oracle: "O2".into(),
            outcome: QuantityOutcome::OracleAbsent("P0.3".into()),
        };
        assert_eq!(
            classify(&end, "tran", &[], &[passed.clone(), absent.clone()]).outcome,
            Outcome::Pass
        );
        assert_eq!(
            classify(&end, "tran", &[], std::slice::from_ref(&absent)).outcome,
            Outcome::Unverified
        );
        let failed = QuantityVerdict {
            outcome: QuantityOutcome::Compared(Verdict {
                pass: false,
                severity: 3.0,
                detail: "off".into(),
            }),
            ..passed.clone()
        };
        assert_eq!(
            classify(&end, "tran", &[], &[passed, failed]).outcome,
            Outcome::Wrong
        );
        let omitted = QuantityVerdict {
            outcome: QuantityOutcome::Missing("v(p)".into()),
            ..absent
        };
        assert_eq!(
            classify(&end, "tran", &[], &[omitted]).outcome,
            Outcome::Wrong
        );
        result.results[0].nonfinite.push("v(a)".into());
        assert_eq!(
            classify(&CaseEnd::Completed(Box::new(result)), "tran", &[], &[]).outcome,
            Outcome::Wrong
        );
    }

    #[test]
    fn quoted_hierarchical_names_identify_the_instance() {
        assert!(quoted_in("instance 'xq' refused", "xq"));
        assert!(quoted_in("instance \"x1.xq\" refused", "xq"));
        assert!(!quoted_in("instance xq refused", "xq"));
        assert!(!quoted_in("instance 'xqq' refused", "xq"));
    }
}

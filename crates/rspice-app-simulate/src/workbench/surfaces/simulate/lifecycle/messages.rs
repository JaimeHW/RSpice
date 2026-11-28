//! The words a lifecycle outcome is announced in: the line for a committed change,
//! the line for a refused one, and a removal refusal in the names analyses show under.

use super::*;
use crate::simulation::plan::AnalysisNumericOverride;

/// Restate a refusal in the names the analyses show under.
///
/// The backstop refusal is reached when the plan changed underneath a staged
/// review, or when a caller went straight to the transaction — rare, and
/// exactly when a reader has the least context to spend on decoding
/// `Analysis 4f2a1b3c-… is still required by 9d10e2f4-…`. Only the identities
/// are rewritten: the sentence, and the fail-closed guarantee it reports, stay
/// the plan's own.
pub(super) fn removal_refusal_naming_instances(model: &Model, error: &AnalysisPlanError) -> String {
    let AnalysisPlanError::ReferencedBy { target, dependents } = error else {
        return error.to_string();
    };
    let named = |id: AnalysisInstanceId| format!("{} ({id})", analysis_display_name(model, id));
    format!(
        "Analysis {} is still required by {}.",
        named(*target),
        dependents
            .iter()
            .map(|dependent| named(*dependent))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// Both of these only announce an outcome — the lifecycle status line and the
/// Console record of it, through the funnel that writes the pair — so they
/// take the simulate feature's context rather than the application. A handler
/// that took the whole app could mutate every subsystem to announce an outcome.
pub(super) fn record_receipt(cx: &mut Cx<'_, SimulateFeature>, receipt: &AnalysisLifecycleReceipt) {
    cx.model.record_plan_receipt(receipt_line(receipt));
}

pub(crate) fn record_failure(cx: &mut Cx<'_, SimulateFeature>, action: &str, error: &str) {
    record_plan_refusal(cx, failure_line(action, error));
}

/// The line [`record_receipt`] announces for `receipt`: the name the analysis
/// shows under, then what changed, as "Transient: disabled".
pub(crate) fn receipt_line(receipt: &AnalysisLifecycleReceipt) -> String {
    receipt.detail().to_owned()
}

/// The line the Console logs when the analysis named `name` takes the
/// settings `after` in place of `before`: its name, then what changed, as
/// "Transient: stop time 100n". `field` is the label of the field the value
/// was typed in, when it was; otherwise each setting that changed is named
/// after itself.
pub(crate) fn edit_line(
    name: &str,
    field: Option<&str>,
    before: &AnalysisDraft,
    after: &AnalysisDraft,
) -> String {
    let changes = changed_settings(before, after);
    let change = match (field, changes.first()) {
        (Some(label), Some((_, value))) => with_value(&in_a_sentence(label), value),
        (Some(label), None) => in_a_sentence(label),
        (None, None) => "edited".to_owned(),
        (None, Some(_)) => changes
            .iter()
            .map(|(key, value)| with_value(&key.replace('_', " "), value))
            .collect::<Vec<_>>()
            .join(", "),
    };
    format!("{name}: {change}")
}

/// The line the Console logs when the analysis named `name` takes the solver
/// options of its own `after` in place of `before`: its name, then each
/// option that changed, as "Transient: TRTOL 3".
pub(crate) fn options_line(
    name: &str,
    before: Option<&AnalysisNumericOverride>,
    after: Option<&AnalysisNumericOverride>,
) -> String {
    let entries = |record: Option<&AnalysisNumericOverride>| {
        record.map_or_else(Vec::new, AnalysisNumericOverride::entries)
    };
    let (before, after) = (entries(before), entries(after));
    let mut changes = after
        .iter()
        .filter(|entry| !before.contains(entry))
        .map(|(option, value)| format!("{} {value}", option.key()))
        .collect::<Vec<_>>();
    changes.extend(
        before
            .iter()
            .filter(|(option, _)| after.iter().all(|(kept, _)| kept != option))
            .map(|(option, _)| format!("{} from the solver options", option.key())),
    );
    if changes.is_empty() {
        format!("{name}: solver options")
    } else {
        format!("{name}: {}", changes.join(", "))
    }
}

/// Each setting `after` holds that `before` does not, by its key, with its
/// new value in words: empty for a list or a group of settings.
fn changed_settings(before: &AnalysisDraft, after: &AnalysisDraft) -> Vec<(String, String)> {
    let settings = |draft: &AnalysisDraft| match serde_json::to_value(draft) {
        Ok(serde_json::Value::Object(mut fields)) => match fields.remove("draft") {
            Some(serde_json::Value::Object(settings)) => settings,
            _ => serde_json::Map::new(),
        },
        _ => serde_json::Map::new(),
    };
    let before = settings(before);
    settings(after)
        .into_iter()
        .filter(|(key, value)| before.get(key) != Some(value))
        .map(|(key, value)| {
            let words = match value {
                serde_json::Value::String(text) => text,
                serde_json::Value::Bool(true) => "on".to_owned(),
                serde_json::Value::Bool(false) => "off".to_owned(),
                serde_json::Value::Number(number) => number.to_string(),
                serde_json::Value::Null => "none".to_owned(),
                serde_json::Value::Array(_) | serde_json::Value::Object(_) => String::new(),
            };
            (key, words)
        })
        .collect()
}

/// `words` followed by `value`, or alone when there is no value to state.
fn with_value(words: &str, value: &str) -> String {
    if value.is_empty() {
        words.to_owned()
    } else {
        format!("{words} {value}")
    }
}

/// A field's label as it reads inside a sentence: "Stop time" as "stop
/// time", and "TRTOL" as it is.
fn in_a_sentence(label: &str) -> String {
    let mut characters = label.chars();
    match (characters.next(), characters.next()) {
        (Some(first), Some(second)) if first.is_uppercase() && !second.is_uppercase() => {
            first.to_lowercase().chain(label.chars().skip(1)).collect()
        }
        _ => label.to_owned(),
    }
}

/// The line [`record_failure`] announces for `action` refused with `error`.
pub(crate) fn failure_line(action: &str, error: &str) -> String {
    format!("{action} rejected fail-closed: {error}. The stable plan is unchanged.")
}

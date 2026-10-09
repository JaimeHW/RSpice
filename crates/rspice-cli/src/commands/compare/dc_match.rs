//! Validate mismatch report identities independently of selected numeric values.

use super::{WaveformData, evidence::true_indicator, strip_outer_call};
use crate::commands::report_identity::decode_folded_part;
use std::collections::HashSet;

pub(super) fn problems(data: &WaveformData) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut problems = Vec::new();
    for (column, name) in data.variables.iter().enumerate() {
        let component = strip_outer_call(name, "Re").or_else(|| strip_outer_call(name, "Im"));
        let claim = component.unwrap_or(name).trim().to_ascii_lowercase();
        if !claim.starts_with("dcmatch:") {
            continue;
        }
        let identity = claim
            .strip_prefix("dcmatch:output(")
            .and_then(|value| value.strip_suffix(')'))
            .filter(|value| !value.trim().is_empty())
            .map(|_| vec!["output".to_owned()])
            .or_else(|| {
                let value = claim
                    .strip_prefix("dcmatch:contributor(")?
                    .strip_suffix(')')?;
                let fields = value
                    .split(',')
                    .map(decode_folded_part)
                    .collect::<Option<Vec<_>>>()?;
                let [scope, instance, parameter] = fields.as_slice() else {
                    return None;
                };
                (matches!(scope.as_str(), "process" | "mismatch")
                    && !instance.trim().is_empty()
                    && !parameter.trim().is_empty())
                .then_some(fields)
            });
        if component.is_some()
            || !true_indicator(data, column)
            || identity.is_none_or(|identity| !seen.insert(identity))
        {
            problems.push(format!(
                "'{name}': invalid or conflicting DC mismatch identity"
            ));
        }
    }
    problems
}

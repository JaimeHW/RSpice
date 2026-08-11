//! Composing the exact executable deck an owned source strategy names.
//!
//! A narrow override stays a small, separately owned file. Nothing here
//! rewrites the frozen generated base it composes against.

use super::*;

/// Materialize the exact executable deck represented by an owned source
/// strategy. Narrow override documents remain small, separately owned files;
/// validation and execution deterministically compose them with their frozen
/// generated base.
pub(crate) fn compose_owned_netlist_execution_source(
    state: &AppState,
    authored_source: &str,
) -> Result<String, String> {
    let Some(descriptor) = state.workspace.netlist_descriptor.as_ref() else {
        return Ok(authored_source.to_owned());
    };
    if descriptor.strategy == crate::state::OwnedNetlistEditStrategy::OwnedSource {
        return Ok(authored_source.to_owned());
    }
    let base = state
        .workspace
        .netlist_document
        .as_ref()
        .map(|document| document.generated_artifact().source())
        .ok_or_else(|| "Narrow override has no retained generated base artifact.".to_owned())?;

    let select: fn(&str) -> bool = match descriptor.strategy {
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => {
            |head| matches!(head, ".param" | ".option" | ".options" | ".temp")
        }
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride => {
            |head| matches!(head, ".include" | ".inc" | ".lib" | ".veriloga")
        }
        crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => is_analysis_directive,
        crate::state::OwnedNetlistEditStrategy::OwnedSource => {
            return Ok(authored_source.to_owned());
        }
    };
    validate_narrow_override(authored_source, select)?;
    let base = match descriptor.strategy {
        crate::state::OwnedNetlistEditStrategy::ParameterOptionOverride => base.to_owned(),
        crate::state::OwnedNetlistEditStrategy::IncludeOrderOverride
        | crate::state::OwnedNetlistEditStrategy::AnalysisOnlyDeck => {
            strip_selected_cards(base, select)
        }
        crate::state::OwnedNetlistEditStrategy::OwnedSource => base.to_owned(),
    };
    insert_before_end(&base, authored_source)
}

pub(super) fn validate_narrow_override(
    source: &str,
    allowed: impl Fn(&str) -> bool,
) -> Result<(), String> {
    let mut continuation_allowed = false;
    for (index, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }
        if trimmed.starts_with('+') {
            if continuation_allowed {
                continue;
            }
            return Err(format!(
                "Override line {} is a continuation without an allowed owning card.",
                index + 1
            ));
        }
        let head = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        continuation_allowed = allowed(&head);
        if !continuation_allowed {
            return Err(format!(
                "Directive '{}' at override line {} is outside the selected ownership strategy.",
                head,
                index + 1
            ));
        }
    }
    Ok(())
}

pub(super) fn strip_selected_cards(source: &str, remove: impl Fn(&str) -> bool) -> String {
    let mut kept = Vec::new();
    let mut removing_continuation = false;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with('+') {
            if !removing_continuation {
                kept.push(line);
            }
            continue;
        }
        let head = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        removing_continuation = remove(&head);
        if !removing_continuation {
            kept.push(line);
        }
    }
    kept.join("\n")
}

pub(super) fn insert_before_end(base: &str, override_source: &str) -> Result<String, String> {
    let lines = base.lines().collect::<Vec<_>>();
    let end = lines
        .iter()
        .rposition(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|head| head.eq_ignore_ascii_case(".end"))
        })
        .ok_or_else(|| "Retained generated base has no .end terminator.".to_owned())?;
    let mut result = lines[..end].join("\n");
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(override_source.trim_end());
    result.push('\n');
    result.push_str(&lines[end..].join("\n"));
    Ok(result)
}

pub(super) fn is_analysis_directive(head: &str) -> bool {
    matches!(
        head,
        ".op"
            | ".tran"
            | ".ac"
            | ".dc"
            | ".noise"
            | ".tf"
            | ".pz"
            | ".sens"
            | ".four"
            | ".sp"
            | ".hb"
            | ".pss"
            | ".pac"
            | ".pnoise"
            | ".stb"
            | ".measure"
            | ".meas"
            | ".save"
            | ".probe"
    )
}

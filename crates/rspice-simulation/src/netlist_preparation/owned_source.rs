//! Composing the exact executable deck an owned source strategy names.
//!
//! A narrow override stays a small, separately owned file. Nothing here
//! rewrites the frozen generated base it composes against.

use rspice_core::netlist::Netlist;
use rspice_design::netlist_document::GeneratedArtifact;
use rspice_design::owned_netlist::{
    NetlistExecutionProfile, OwnedNetlistDescriptor, OwnedNetlistEditStrategy,
};

/// Materialize the exact executable deck represented by an owned source
/// strategy. Narrow override documents remain small, separately owned files;
/// validation and execution deterministically compose them with their frozen
/// generated base.
pub fn compose_owned_netlist_execution_source(
    descriptor: Option<&OwnedNetlistDescriptor>,
    generated: Option<&GeneratedArtifact>,
    authored_source: &str,
) -> Result<String, String> {
    let Some(descriptor) = descriptor else {
        return Ok(authored_source.to_owned());
    };
    if descriptor.strategy == OwnedNetlistEditStrategy::OwnedSource {
        return Ok(authored_source.to_owned());
    }
    let base = generated
        .map(GeneratedArtifact::source)
        .ok_or_else(|| "Narrow override has no retained generated base artifact.".to_owned())?;

    let select: fn(&str) -> bool = match descriptor.strategy {
        OwnedNetlistEditStrategy::ParameterOptionOverride => {
            |head| matches!(head, ".param" | ".option" | ".options" | ".temp")
        }
        OwnedNetlistEditStrategy::IncludeOrderOverride => {
            |head| matches!(head, ".include" | ".inc" | ".lib" | ".veriloga")
        }
        OwnedNetlistEditStrategy::AnalysisOnlyDeck => is_analysis_directive,
        OwnedNetlistEditStrategy::OwnedSource => {
            return Ok(authored_source.to_owned());
        }
    };
    validate_narrow_override(authored_source, select)?;
    let base = match descriptor.strategy {
        OwnedNetlistEditStrategy::ParameterOptionOverride => base.to_owned(),
        OwnedNetlistEditStrategy::IncludeOrderOverride
        | OwnedNetlistEditStrategy::AnalysisOnlyDeck => strip_selected_cards(base, select),
        OwnedNetlistEditStrategy::OwnedSource => base.to_owned(),
    };
    insert_before_end(&base, authored_source)
}

fn validate_narrow_override(source: &str, allowed: impl Fn(&str) -> bool) -> Result<(), String> {
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

fn strip_selected_cards(source: &str, remove: impl Fn(&str) -> bool) -> String {
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

fn insert_before_end(base: &str, override_source: &str) -> Result<String, String> {
    if crate::netlist_preparation::terminal_end_card_offset(base).is_none() {
        return Err("Retained generated base has no .end terminator.".to_owned());
    }
    Ok(crate::netlist_preparation::splice_before_terminal_end_card(
        base,
        override_source,
    ))
}

fn is_analysis_directive(head: &str) -> bool {
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

/// Apply the reviewed source contract before model binding or include parsing.
pub fn adapt_owned_execution_profile<'a>(
    descriptor: Option<&OwnedNetlistDescriptor>,
    source: &'a str,
) -> Result<std::borrow::Cow<'a, str>, String> {
    let Some(descriptor) = descriptor else {
        return Ok(std::borrow::Cow::Borrowed(source));
    };
    if descriptor.execution_profile_review_required() {
        return Err("Review this deck's execution profile before running it.".to_owned());
    }
    let Some(profile) = descriptor.execution_profile else {
        return Ok(std::borrow::Cow::Borrowed(source));
    };
    if profile.source_dialect() != descriptor.imported_dialect.unwrap_or_default() {
        return Err(
            "The deck's execution profile does not match its reviewed source dialect.".to_owned(),
        );
    }
    let adapted = profile.adapt_source(source)?;
    profile.validate_executable_source(&adapted)?;
    Ok(adapted)
}

/// Validate the sealed dependency closure and bind compatibility into the
/// executable source. All analysis drivers and workers resolve these options
/// through the same core configuration pipeline; the authored deck is retained.
pub fn bind_execution_profile(
    profile: Option<NetlistExecutionProfile>,
    source: String,
) -> Result<String, String> {
    let Some(profile) = profile else {
        return Ok(source);
    };
    profile.validate_resolved_source(&source)?;
    let parsed = Netlist::parse(&source).map_err(|error| error.to_string())?;
    profile.validate_parsed_netlist(&parsed)?;
    if let Some(diagnostic) = parsed
        .diagnostics
        .iter()
        .find(|diagnostic| NetlistExecutionProfile::diagnostic_is_semantic_loss(&diagnostic.code))
    {
        return Err(format!(
            "Parser diagnostic {} at line {}: {}",
            diagnostic.code, diagnostic.line, diagnostic.message
        ));
    }
    rspice_core::netlist::validate_output_symbols(&parsed).map_err(|error| error.to_string())?;
    if profile == NetlistExecutionProfile::RSpiceCanonicalV1 {
        return Ok(source);
    }
    let dialect = match profile.spice_dialect() {
        rspice_core::SpiceDialect::BestAvailable => "BEST_AVAILABLE",
        rspice_core::SpiceDialect::Ngspice => "NGSPICE",
        rspice_core::SpiceDialect::Xyce => "XYCE",
    };
    Ok(crate::netlist_preparation::splice_before_terminal_end_card(
        &source,
        &format!(
            "* RSpice execution profile: {}\n.OPTIONS RSPICE_DIALECT={dialect}",
            profile.id()
        ),
    ))
}

#[cfg(test)]
mod tests;

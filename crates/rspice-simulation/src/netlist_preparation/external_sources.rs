//! Which lines of an executable deck still open something outside it.
//!
//! A prepared run executes sealed sources only, so preparation refuses a line
//! that would read a file, a library or a co-simulation runtime at solve time.
//! The judgment is lexical and owns no state.

use crate::preparation::{PreparationError, PreparationStage};
use crate::project_veriloga::preparation::project_veriloga_directive_matches_exact_identity;
use rspice_core::netlist::{parse_include_directive, parse_lib_directive};

pub fn contains_external_include_directive(source: &str) -> bool {
    source
        .lines()
        .any(|line| parse_include_directive(line).is_some() || parse_lib_directive(line).is_some())
}

pub fn deferred_external_source_reason(line: &str) -> Option<&'static str> {
    let line = executable_source_portion(line);
    if line.is_empty() {
        return None;
    }
    if parse_include_directive(line).is_some() || parse_lib_directive(line).is_some() {
        return Some("include/library directive");
    }
    let lower = line.to_ascii_lowercase();
    let directive = lower.split_whitespace().next().unwrap_or_default();
    if matches!(
        directive,
        ".spef_include" | ".veriloga" | ".va" | ".ahdl_include" | ".hdl" | ".verilog" | ".load"
    ) {
        return Some("external source directive");
    }

    let without_whitespace = lower
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect::<String>();
    // File/provider assignments belong to code-model declarations or instances;
    // identically named numeric parameters do not open external resources.
    let code_model = directive == ".model" || directive.starts_with('a');
    if code_model
        && ["file", "input_file", "state_file", "process_file"]
            .iter()
            .any(|name| contains_parameter_assignment(&lower, name))
    {
        return Some("file-backed element or code-model parameter");
    }
    let tokens = lower
        .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    if matches!(directive, ".measure" | ".meas") && tokens.contains(&"file") {
        return Some("file-backed measurement reference");
    }
    // `simulation` is the d_cosim shared-library/provider selector and may be
    // supplied either on its model or as an instance override.
    if code_model && contains_parameter_assignment(&lower, "simulation") {
        return Some("external co-simulation runtime");
    }
    const FILE_LOOKUPS: [&str; 16] = [
        "table",
        "tablefile",
        "fasttable",
        "fasttablefile",
        "cubic",
        "cubicfile",
        "akima",
        "akimafile",
        "spline",
        "splinefile",
        "wodicka",
        "wodickafile",
        "bli",
        "blifile",
        "barycentric",
        "barycentricfile",
    ];
    if FILE_LOOKUPS.iter().any(|function| {
        [format!("{function}(\""), format!("{function}('")]
            .iter()
            .any(|needle| without_whitespace.contains(needle))
    }) {
        return Some("file-backed behavioral lookup");
    }
    None
}

fn executable_source_portion(line: &str) -> &str {
    rspice_core::netlist::strip_spice_inline_comment(
        line,
        rspice_core::config::ExpressionDialect::Ngspice,
    )
    .trim()
}

fn contains_parameter_assignment(line: &str, parameter: &str) -> bool {
    line.match_indices(parameter).any(|(index, _)| {
        let has_identifier_boundary = index == 0
            || line[..index]
                .chars()
                .next_back()
                .is_some_and(|character| !(character.is_ascii_alphanumeric() || character == '_'));
        has_identifier_boundary
            && line[index + parameter.len()..]
                .trim_start()
                .starts_with('=')
    })
}

pub fn reject_deferred_external_sources_with_project_runtimes(
    netlist: &str,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    measurement_references: &crate::measurement_references::PreparedMeasurementReferences,
) -> Result<(), PreparationError> {
    measurement_references
        .validate_source(netlist)
        .map_err(|error| {
            PreparationError::new(
                PreparationStage::SourceChecks,
                format!("Executable netlist contains an unsealed external dependency: {error}"),
            )
        })?;
    for (line_number, logical_line) in executable_logical_lines(netlist) {
        if project_runtimes.sources().any(|runtime| {
            project_veriloga_directive_matches_exact_identity(
                &logical_line,
                runtime.source_key(),
                runtime.netlist_alias(),
            )
        }) {
            continue;
        }
        if let Some(reason) = deferred_external_source_reason(&logical_line) {
            if reason == "file-backed measurement reference" {
                continue;
            }
            return Err(PreparationError::new(
                PreparationStage::SourceChecks,
                format!(
                    "Executable netlist contains an unsealed external dependency ({reason}) at line {}: {}",
                    line_number, logical_line
                ),
            ));
        }
    }
    Ok(())
}

/// Fold physical SPICE continuation records exactly as the core parser does
/// for executable lines: comment removal and trimming happen per physical
/// line, then a leading `+` appends to the preceding logical record. Auditing
/// the folded form prevents an external path or parameter name from being
/// split across continuation boundaries after authorization.
pub fn executable_logical_lines(source: &str) -> Vec<(usize, String)> {
    let mut logical_lines = Vec::new();
    let mut pending: Option<(usize, String)> = None;

    for (index, physical_line) in source.lines().enumerate() {
        let trimmed = executable_source_portion(physical_line).trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix('+') {
            let (_, logical) = pending.get_or_insert_with(|| (index + 1, String::new()));
            logical.push(' ');
            logical.push_str(rest);
            continue;
        }

        if let Some(previous) = pending.replace((index + 1, trimmed.to_owned())) {
            logical_lines.push(previous);
        }
    }

    if let Some(previous) = pending {
        logical_lines.push(previous);
    }
    logical_lines
}

#[cfg(test)]
mod tests;

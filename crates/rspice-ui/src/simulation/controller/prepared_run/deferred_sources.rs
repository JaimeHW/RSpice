//! Which lines of an executable deck still open something outside it.
//!
//! A prepared run executes sealed sources only, so preparation refuses a line
//! that would read a file, a library or a co-simulation runtime at solve time.
//! The judgment is lexical and owns no state.

use rspice_core::netlist::{parse_include_directive, parse_lib_directive};

pub(super) fn deferred_external_source_reason(line: &str) -> Option<&'static str> {
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

pub(super) fn executable_source_portion(line: &str) -> &str {
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

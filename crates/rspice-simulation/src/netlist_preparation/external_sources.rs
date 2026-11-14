//! Which lines of an executable deck still open something outside it.
//!
//! A prepared run executes sealed sources only, so preparation refuses a line
//! that would read a file, a library or a co-simulation runtime at solve time.
//! The judgment is lexical and owns no state.

use super::process_programs;
use crate::preparation::{PreparationError, PreparationStage};
use crate::project_veriloga::preparation::project_veriloga_directive_matches_exact_identity;
use rspice_core::netlist::{parse_include_directive, parse_lib_directive};

pub fn contains_external_include_directive(source: &str) -> bool {
    source
        .lines()
        .any(|line| parse_include_directive(line).is_some() || parse_lib_directive(line).is_some())
}

/// The reason a code model's data-file parameter gives. A run's own deck
/// seals those files at preparation (`data_files`), which reads them through
/// the engine's parameter resolution, defaults included; a source nothing
/// seals them from, such as a corner's model cards, is refused for them.
const CODE_MODEL_DATA_FILE: &str = "code-model data file";

/// The reason a `d_process` program gives. Preparation writes a relative
/// program path as the root deck's directory resolves it, as the CLI does,
/// so a run starts a program it names by an absolute path; a host that
/// cannot start programs refuses each instance by name.
const PROCESS_PROGRAM: &str = "program a d_process instance starts";

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
        ".spef_include"
            | ".veriloga"
            | ".va"
            | ".ahdl_include"
            | ".hdl"
            | ".vams"
            | ".verilog"
            | ".load"
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
    if code_model && contains_parameter_assignment(&lower, "process_file") {
        return Some(PROCESS_PROGRAM);
    }
    if code_model
        && ["file", "input_file", "state_file"]
            .iter()
            .any(|name| contains_parameter_assignment(&lower, name))
    {
        return Some(CODE_MODEL_DATA_FILE);
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

/// Refuse a deck line that would open something outside the prepared run.
/// The project's sealed Verilog-A and measurement tables pass; so do code-model
/// data files, which the prepared snapshot seals and checks deck by deck, and
/// a `d_process` program named by an absolute path on a host that can start
/// it.
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
    reject_deferred_external_sources_on(
        netlist,
        project_runtimes,
        process_programs::STARTS_PROGRAMS,
        std::env::var_os("PATH").as_deref(),
    )
}

/// [`reject_deferred_external_sources_with_project_runtimes`] on a host that
/// starts programs or not, looking a bare program name up on `search_path`.
fn reject_deferred_external_sources_on(
    netlist: &str,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    starts_programs: bool,
    search_path: Option<&std::ffi::OsStr>,
) -> Result<(), PreparationError> {
    let records = executable_logical_lines(netlist);
    for (line_number, logical_line) in &records {
        let (line_number, logical_line) = (*line_number, logical_line.as_str());
        if project_runtimes.sources().any(|runtime| {
            project_veriloga_directive_matches_exact_identity(
                logical_line,
                runtime.source_key(),
                runtime.netlist_alias(),
            )
        }) {
            continue;
        }
        if let Some(reason) = deferred_external_source_reason(logical_line) {
            if reason == "file-backed measurement reference" || reason == CODE_MODEL_DATA_FILE {
                continue;
            }
            if reason == PROCESS_PROGRAM {
                admit_process_programs(
                    &records,
                    line_number,
                    logical_line,
                    starts_programs,
                    search_path,
                )?;
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

/// Admit a record's `d_process` programs when this host starts each where
/// the CLI starts it: by an absolute path, or by a bare name `search_path`
/// finds. Otherwise refuse, naming the program, or naming the instances when
/// the host cannot start programs at all. A model no instance names starts
/// nothing and passes.
fn admit_process_programs(
    records: &[(usize, String)],
    line_number: usize,
    record: &str,
    starts_programs: bool,
    search_path: Option<&std::ffi::OsStr>,
) -> Result<(), PreparationError> {
    let instances = process_programs::process_instances(records, record);
    if instances.is_empty() {
        // A model no instance names starts nothing.
        return Ok(());
    }
    let programs = process_programs::process_programs(record);
    if !starts_programs {
        let named = instances
            .iter()
            .map(|instance| format!("'{instance}'"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(PreparationError::new(
            PreparationStage::SourceChecks,
            format!(
                "d_process instance {named} starts the program '{}' (line {line_number}: {record}), \
                 and a run in the browser cannot start a program; run the deck in the desktop \
                 application or the CLI",
                programs.first().copied().unwrap_or_default()
            ),
        ));
    }
    match programs.iter().find(|program| {
        !process_programs::names_its_program_absolutely(program)
            && process_programs::program_on_path(program, search_path).is_none()
    }) {
        None => Ok(()),
        Some(program) if process_programs::is_bare_program(program) => Err(PreparationError::new(
            PreparationStage::SourceChecks,
            format!(
                "Executable netlist contains an unsealed external dependency ({PROCESS_PROGRAM}) at \
                 line {line_number}: {record}; no file beside the deck and no directory on PATH \
                 holds the program '{program}'"
            ),
        )),
        Some(program) => Err(PreparationError::new(
            PreparationStage::SourceChecks,
            format!(
                "Executable netlist contains an unsealed external dependency ({PROCESS_PROGRAM}) at \
                 line {line_number}: {record}; the program '{program}' is named relative to no \
                 deck directory, so name it by an absolute path"
            ),
        )),
    }
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

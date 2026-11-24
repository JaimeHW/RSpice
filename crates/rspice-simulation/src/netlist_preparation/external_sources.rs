//! Which lines of an executable deck still open something outside it.
//!
//! A prepared run executes sealed sources only, so preparation refuses a line
//! that would read a file, a library or a co-simulation runtime at solve time.
//! The judgment is lexical and owns no state, except that a line the parser
//! never reads, such as one in a false `.if` branch, opens nothing and is
//! skipped as the run skips it ([`UnreadLines`]).

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
///
/// The measurement tables are bound to the deck read under `limits`, the
/// limits its run executes under.
pub fn reject_deferred_external_sources_with_project_runtimes(
    netlist: &str,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    measurement_references: &crate::measurement_references::PreparedMeasurementReferences,
    limits: rspice_core::ResourceLimits,
) -> Result<(), PreparationError> {
    reject_deferred_external_sources_for_host(
        netlist,
        project_runtimes,
        measurement_references,
        process_programs::STARTS_PROGRAMS,
        limits,
    )
}

/// [`reject_deferred_external_sources_with_project_runtimes`] for a deck its
/// run's preparation parsed as `parsed`, stopped once `abort` is raised: it
/// polls `abort` as it reads the deck's lines, and reads the lines the
/// parser skips off `parsed` rather than parsing the deck again.
pub fn reject_deferred_external_sources_with_abort(
    netlist: &str,
    parsed: &rspice_core::Netlist,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    measurement_references: &crate::measurement_references::PreparedMeasurementReferences,
    limits: rspice_core::ResourceLimits,
    abort: &dyn rspice_core::abort_signal::AbortSignal,
) -> Result<(), PreparationError> {
    reject_deferred_external_sources_for_host_with_abort(
        netlist,
        Some(parsed),
        &UnreadLines::of_parsed(parsed),
        project_runtimes,
        measurement_references,
        process_programs::STARTS_PROGRAMS,
        limits,
        abort,
    )
}

/// [`reject_deferred_external_sources_with_project_runtimes`] for a run
/// prepared here for a host that starts programs or not: a run prepared for
/// the browser worker refuses every `d_process` instance's program, as the
/// browser itself does.
pub(crate) fn reject_deferred_external_sources_for_host(
    netlist: &str,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    measurement_references: &crate::measurement_references::PreparedMeasurementReferences,
    starts_programs: bool,
    limits: rspice_core::ResourceLimits,
) -> Result<(), PreparationError> {
    reject_deferred_external_sources_for_host_with_abort(
        netlist,
        None,
        &UnreadLines::new(netlist, limits),
        project_runtimes,
        measurement_references,
        starts_programs,
        limits,
        &rspice_core::NoAbort,
    )
}

/// [`reject_deferred_external_sources_for_host`], stopped once `abort` is
/// raised, with `unread` naming the lines the parser skips, for a deck its
/// run's preparation parsed as `parsed`, where it did.
#[allow(clippy::too_many_arguments)]
fn reject_deferred_external_sources_for_host_with_abort(
    netlist: &str,
    parsed: Option<&rspice_core::Netlist>,
    unread: &UnreadLines<'_>,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    measurement_references: &crate::measurement_references::PreparedMeasurementReferences,
    starts_programs: bool,
    limits: rspice_core::ResourceLimits,
    abort: &dyn rspice_core::abort_signal::AbortSignal,
) -> Result<(), PreparationError> {
    match parsed {
        Some(parsed) => measurement_references.validate_parsed(netlist, parsed, limits),
        None => measurement_references.validate_source(netlist, limits),
    }
    .map_err(|error| {
        PreparationError::new(
            PreparationStage::SourceChecks,
            format!("The netlist reads a file that is not kept with the run: {error}"),
        )
    })?;
    PreparationError::check_abort(abort)?;
    scan_deferred_external_sources(
        netlist,
        project_runtimes,
        starts_programs,
        std::env::var_os("PATH").as_deref(),
        unread,
        abort,
    )
}

/// The capability a host that cannot start a program lacks for a
/// `d_process` instance.
pub(crate) const PROCESS_PROGRAM_CAPABILITY: &str = "xspice.d_process.program";

/// [`reject_deferred_external_sources_with_project_runtimes`] on a host that
/// starts programs or not, looking a bare program name up on `search_path`.
#[cfg(test)]
fn reject_deferred_external_sources_on(
    netlist: &str,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    starts_programs: bool,
    search_path: Option<&std::ffi::OsStr>,
) -> Result<(), PreparationError> {
    scan_deferred_external_sources(
        netlist,
        project_runtimes,
        starts_programs,
        search_path,
        &UnreadLines::new(netlist, rspice_core::ResourceLimits::default()),
        &rspice_core::NoAbort,
    )
}

/// How many of a deck's records are read between two polls of the abort
/// signal: a few milliseconds of reading.
const RECORDS_PER_ABORT_POLL: usize = 4096;

/// Whether a deck's record, as [`for_each_logical_line`] folds it, can name
/// anything [`deferred_external_source_reason`] refuses. Each reason is a
/// directive, a code model's card (an instance `A…`, or its `.model`), or a
/// lookup reading a quoted file, so a record that starts with none of `.`,
/// `a` and `A` and quotes nothing names nothing, and is passed over without
/// the reasons being read.
fn may_name_an_external_source(record: &str) -> bool {
    matches!(record.as_bytes().first(), Some(b'.' | b'a' | b'A'))
        || record.bytes().any(|byte| byte == b'"' || byte == b'\'')
}

/// [`reject_deferred_external_sources_with_project_runtimes`] on a host that
/// starts programs or not, looking a bare program name up on `search_path`,
/// stopped once `abort` is raised. `unread` names the lines the parser skips.
fn scan_deferred_external_sources(
    netlist: &str,
    project_runtimes: &crate::veriloga::PreparedVerilogARuntimeSet,
    starts_programs: bool,
    search_path: Option<&std::ffi::OsStr>,
    unread: &UnreadLines<'_>,
    abort: &dyn rspice_core::abort_signal::AbortSignal,
) -> Result<(), PreparationError> {
    // The deck's records are listed whole only for a `d_process` program,
    // whose instances are looked for among them.
    let records = std::cell::OnceCell::new();
    let mut read = 0usize;
    PreparationError::check_abort(abort)?;
    for_each_logical_line(netlist, |line_number, logical_line| {
        read += 1;
        if read.is_multiple_of(RECORDS_PER_ABORT_POLL) {
            PreparationError::check_abort(abort)?;
        }
        if !may_name_an_external_source(logical_line) {
            return Ok(());
        }
        if project_runtimes.sources().any(|runtime| {
            project_veriloga_directive_matches_exact_identity(
                logical_line,
                runtime.source_key(),
                runtime.netlist_alias(),
            )
        }) {
            return Ok(());
        }
        let Some(reason) = deferred_external_source_reason(logical_line) else {
            return Ok(());
        };
        if reason == "file-backed measurement reference" || reason == CODE_MODEL_DATA_FILE {
            return Ok(());
        }
        if reason == PROCESS_PROGRAM {
            return match admit_process_programs(
                records.get_or_init(|| executable_logical_lines(netlist)),
                line_number,
                logical_line,
                starts_programs,
                search_path,
            ) {
                Err(refusal) if !unread.contains(line_number) => Err(refusal),
                _ => Ok(()),
            };
        }
        if unread.contains(line_number) {
            return Ok(());
        }
        Err(PreparationError::new(
            PreparationStage::SourceChecks,
            format!(
                "Line {} reads a file that is not kept with the run ({reason}): {}",
                line_number, logical_line
            ),
        ))
    })
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
        )
        .at_line(Some(line_number))
        .refusing(
            PROCESS_PROGRAM_CAPABILITY,
            instances.into_iter().map(str::to_owned).collect(),
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
                "Line {line_number} names a program that is not kept with the run \
                 ({PROCESS_PROGRAM}): {record}; no file beside the deck and no folder on PATH \
                 holds the program '{program}'"
            ),
        )),
        Some(program) => Err(PreparationError::new(
            PreparationStage::SourceChecks,
            format!(
                "Line {line_number} names a program that is not kept with the run \
                 ({PROCESS_PROGRAM}): {record}; the program '{program}' is named relative to no \
                 deck folder, so name it by an absolute path"
            ),
        )),
    }
}

/// The lines of a deck the parser skips without reading them: an untaken
/// `.if`/`.elseif`/`.else` branch, an inline `.lib` section no `.lib` card
/// selects. A run never reads such a line, so a check skips it as ngspice
/// skips every line of a false branch: it comments them out once the deck
/// and its includes are read (`inp.c` `dotifeval`).
///
/// A deck that is not parsed yet is parsed on the first question only, so a
/// deck no check would refuse a line of costs nothing, and under the limits
/// its run executes under, so a deck the run reads is read here too; a deck
/// its run's preparation parsed is not parsed again
/// ([`UnreadLines::of_parsed`]). A deck that does not parse has no line the
/// parser skips, and every line is checked.
pub(crate) struct UnreadLines<'a> {
    deck: Option<(&'a str, rspice_core::ResourceLimits)>,
    regions: std::cell::OnceCell<Vec<std::ops::RangeInclusive<usize>>>,
}

impl<'a> UnreadLines<'a> {
    /// The lines `deck` skips, read under `limits`, the limits its run
    /// executes under.
    pub(crate) fn new(deck: &'a str, limits: rspice_core::ResourceLimits) -> Self {
        Self {
            deck: Some((deck, limits)),
            regions: std::cell::OnceCell::new(),
        }
    }

    /// The lines `parsed`, a deck as parsed, skips: read off the parse.
    pub(crate) fn of_parsed(parsed: &rspice_core::Netlist) -> Self {
        Self {
            deck: None,
            regions: std::cell::OnceCell::from(unread_regions(parsed)),
        }
    }

    /// Whether one-based physical line `line` of the deck is one the parser
    /// skips.
    pub(crate) fn contains(&self, line: usize) -> bool {
        self.regions
            .get_or_init(|| {
                self.deck
                    .and_then(|(deck, limits)| super::parse_under(deck, limits).ok())
                    .as_ref()
                    .map_or_else(Vec::new, unread_regions)
            })
            .iter()
            .any(|lines| lines.contains(&line))
    }
}

/// The lines of its root source the parse `parsed` skips.
fn unread_regions(parsed: &rspice_core::Netlist) -> Vec<std::ops::RangeInclusive<usize>> {
    parsed
        .unread_root_source()
        .map(|region| region.lines.clone())
        .collect()
}

/// Fold physical SPICE continuation records exactly as the core parser does
/// for executable lines: comment removal and trimming happen per physical
/// line, then a leading `+` appends to the preceding logical record. Auditing
/// the folded form prevents an external path or parameter name from being
/// split across continuation boundaries after authorization.
pub fn executable_logical_lines(source: &str) -> Vec<(usize, String)> {
    let mut logical_lines = Vec::new();
    let _ = for_each_logical_line(source, |line, record| {
        logical_lines.push((line, record.to_owned()));
        Ok::<(), std::convert::Infallible>(())
    });
    logical_lines
}

/// Each of `source`'s records, as [`executable_logical_lines`] folds them,
/// with the number of the physical line it starts on, until `each` refuses
/// one. A record on one physical line is lent where it stands; only a record
/// that continues is copied, to fold it.
pub(crate) fn for_each_logical_line<'a, E>(
    source: &'a str,
    mut each: impl FnMut(usize, &str) -> Result<(), E>,
) -> Result<(), E> {
    // The record being read: in one piece while it stands on one line,
    // folded once a continuation follows it. At most one is held.
    let mut standing: Option<(usize, &'a str)> = None;
    let mut folded: Option<(usize, String)> = None;
    for (index, physical_line) in source.lines().enumerate() {
        let trimmed = executable_source_portion(physical_line).trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }
        if let Some(rest) = trimmed.strip_prefix('+') {
            let (_, record) = folded.get_or_insert_with(|| match standing.take() {
                Some((line, record)) => (line, record.to_owned()),
                None => (index + 1, String::new()),
            });
            record.push(' ');
            record.push_str(rest);
            continue;
        }
        if let Some((line, record)) = folded.take() {
            each(line, &record)?;
        } else if let Some((line, record)) = standing.take() {
            each(line, record)?;
        }
        standing = Some((index + 1, trimmed));
    }
    if let Some((line, record)) = folded {
        each(line, &record)?;
    } else if let Some((line, record)) = standing {
        each(line, record)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests;

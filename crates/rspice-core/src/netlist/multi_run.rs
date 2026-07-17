//! HSPICE-style multi-run preprocessing: `.ALTER` blocks and `.DATA` sweeps.
//!
//! Both constructs multiply one deck into several concrete runs, so they
//! are resolved *textually, before parsing* — the model HSPICE itself uses
//! (every alter re-reads the modified deck). [`expand_multi_run`] turns raw
//! deck text into self-contained single-run decks:
//!
//! * `.ALTER [title]` — each block edits the deck cumulatively: element and
//!   `.model` statements replace the statement with the same name (taking
//!   its continuation lines along), `.param` assignments override the
//!   existing top-level assignment in place (preserving in-order parameter
//!   evaluation), and anything else appends.
//! * `.DATA name p1 p2 … / rows … / .ENDDATA` — an analysis referencing
//!   `DATA=name` (`.dc`, or `SWEEP DATA=name` on `.ac`/`.tran`) expands to
//!   one run per table row with the row's parameter values bound. A bare
//!   `.dc data=name` is one operating point per row, so it rewrites to
//!   `.op`.
//!
//! The netlist parser itself only skips these blocks (the base deck stays
//! parseable everywhere); the run orchestration lives in the CLI, which
//! loops the expanded decks.

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::netlist::lexer::parse_spice_value;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};

/// One concrete, self-contained run produced by the expansion.
#[derive(Debug, Clone)]
pub struct RunDeck {
    /// Human-readable run label (`None` for a plain single-run deck).
    pub label: Option<String>,
    /// Complete deck text, parseable on its own.
    pub source: String,
}

/// Error raised while expanding HSPICE-style `.ALTER` / `.DATA` constructs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiRunError {
    message: String,
    resource_limit: Option<ResourceLimitError>,
    aborted: bool,
}

impl MultiRunError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            resource_limit: None,
            aborted: false,
        }
    }

    fn resource_limit(error: ResourceLimitError) -> Self {
        Self {
            message: error.to_string(),
            resource_limit: Some(error),
            aborted: false,
        }
    }

    fn aborted() -> Self {
        Self {
            message: "multi-run expansion aborted".to_owned(),
            resource_limit: None,
            aborted: true,
        }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    /// Typed resource-limit details, when expansion was rejected by policy.
    pub fn resource_limit_error(&self) -> Option<ResourceLimitError> {
        self.resource_limit
    }

    /// Whether cooperative cancellation stopped the expansion.
    pub fn is_aborted(&self) -> bool {
        self.aborted
    }
}

impl std::fmt::Display for MultiRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MultiRunError {}

/// One `.ALTER` block: its title and raw body lines.
#[derive(Debug, Clone)]
struct AlterBlock {
    title: String,
    lines: Vec<String>,
}

/// One parsed `.DATA` table.
#[derive(Debug, Clone)]
struct DataTable {
    name: String,
    params: Vec<String>,
    rows: Vec<Vec<Value>>,
}

/// Expand a deck into its concrete runs. Decks without `.ALTER`/`.DATA`
/// constructs pass through untouched as a single unlabeled run.
pub fn expand_multi_run(source: &str) -> Vec<RunDeck> {
    match try_expand_multi_run(source) {
        Ok(decks) => decks,
        Err(error) => {
            log::warn!("multi-run expansion failed: {error}");
            vec![RunDeck {
                label: None,
                source: source.to_owned(),
            }]
        }
    }
}

/// Checked expansion for production callers. Malformed `.DATA` constructs
/// return an error instead of silently dropping values or running the base deck.
pub fn try_expand_multi_run(source: &str) -> Result<Vec<RunDeck>, MultiRunError> {
    try_expand_multi_run_with_limits(source, ResourceLimits::default())
}

/// Checked expansion under an explicit resource policy.
pub fn try_expand_multi_run_with_limits(
    source: &str,
    resource_limits: ResourceLimits,
) -> Result<Vec<RunDeck>, MultiRunError> {
    try_expand_multi_run_with_limits_and_abort(source, resource_limits, &NoAbort)
}

/// Checked expansion under an explicit resource policy with cooperative
/// cancellation. The retained concrete decks are bounded both by run count
/// and by their aggregate source bytes.
pub fn try_expand_multi_run_with_limits_and_abort(
    source: &str,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<RunDeck>, MultiRunError> {
    ensure_not_aborted(abort)?;
    ensure_resource(
        ResourceKind::NetlistBytes,
        source.len(),
        resource_limits.max_netlist_bytes,
    )?;

    let mut has_multi_run = false;
    for (index, line) in source.lines().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;
        ensure_resource(
            ResourceKind::NetlistLines,
            index.saturating_add(1),
            resource_limits.max_netlist_lines,
        )?;
        let token = first_token(line);
        has_multi_run |= token.eq_ignore_ascii_case(".alter")
            || token.eq_ignore_ascii_case(".data")
            || token.eq_ignore_ascii_case(".enddata")
            || references_data_table(line);
    }
    if !has_multi_run {
        let mut decks = Vec::new();
        let mut retained_source_bytes = 0;
        push_source_deck(
            &mut decks,
            &mut retained_source_bytes,
            None,
            source,
            resource_limits,
            abort,
        )?;
        return Ok(decks);
    }

    let (mut current_lines, alters) = split_alters(source, abort)?;
    ensure_resource(
        ResourceKind::BatchRuns,
        alters.len().saturating_add(1),
        resource_limits.max_batch_runs,
    )?;

    let mut decks = Vec::new();
    let mut retained_source_bytes = 0usize;
    let base_label = (!alters.is_empty()).then(|| "base".to_owned());
    expand_variant(
        base_label,
        &current_lines,
        &mut decks,
        &mut retained_source_bytes,
        resource_limits,
        abort,
    )?;

    // Apply each alter cumulatively and expand it immediately. Keeping only
    // the current variant avoids an O(alters * deck-size) intermediate.
    for (index, block) in alters.iter().enumerate() {
        poll_abort(abort, index)?;
        apply_alter(&mut current_lines, block, abort)?;
        let label = if block.title.is_empty() {
            format!("alter {}", index + 1)
        } else {
            block.title.clone()
        };
        expand_variant(
            Some(label),
            &current_lines,
            &mut decks,
            &mut retained_source_bytes,
            resource_limits,
            abort,
        )?;
    }
    ensure_not_aborted(abort)?;
    Ok(decks)
}

fn expand_variant(
    label: Option<String>,
    source_lines: &[String],
    decks: &mut Vec<RunDeck>,
    retained_source_bytes: &mut usize,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    ensure_not_aborted(abort)?;
    let (tables, mut lines) = extract_data_tables(source_lines.to_vec(), resource_limits, abort)?;
    let reference = find_data_reference(&lines, &tables, abort)?;

    match reference {
        Some((table_index, _)) => {
            let table = &tables[table_index];
            if table.rows.is_empty() {
                return Err(MultiRunError::new(format!(
                    ".data {} has no rows",
                    table.name
                )));
            }
            ensure_resource(
                ResourceKind::BatchRuns,
                decks.len().saturating_add(table.rows.len()),
                resource_limits.max_batch_runs,
            )?;
            strip_data_tokens(&mut lines, abort)?;
            for (row_index, row) in table.rows.iter().enumerate() {
                poll_abort(abort, row_index)?;
                let mut run_lines = lines.clone();
                for (param_index, (param, value)) in table.params.iter().zip(row).enumerate() {
                    poll_abort(abort, param_index)?;
                    override_param_with_abort(&mut run_lines, param, &format_value(*value), abort)?;
                }
                let row_label = format!("{} row {}", table.name, row_index + 1);
                push_assembled_deck(
                    decks,
                    retained_source_bytes,
                    Some(match &label {
                        Some(label) => format!("{label} · {row_label}"),
                        None => row_label,
                    }),
                    &run_lines,
                    resource_limits,
                    abort,
                )?;
            }
        }
        None => push_assembled_deck(
            decks,
            retained_source_bytes,
            label,
            &lines,
            resource_limits,
            abort,
        )?,
    }
    Ok(())
}

fn push_assembled_deck(
    decks: &mut Vec<RunDeck>,
    retained_source_bytes: &mut usize,
    label: Option<String>,
    lines: &[String],
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    let requested_bytes = assembled_len(lines);
    let requested_total = retained_source_bytes.saturating_add(requested_bytes);
    ensure_resource(
        ResourceKind::ExpandedSourceBytes,
        requested_total,
        resource_limits.max_expanded_source_bytes,
    )?;
    ensure_resource(
        ResourceKind::BatchRuns,
        decks.len().saturating_add(1),
        resource_limits.max_batch_runs,
    )?;
    let source = assemble(lines, requested_bytes, abort)?;
    push_owned_deck(decks, retained_source_bytes, label, source)
}

fn push_source_deck(
    decks: &mut Vec<RunDeck>,
    retained_source_bytes: &mut usize,
    label: Option<String>,
    source: &str,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    ensure_resource(
        ResourceKind::ExpandedSourceBytes,
        retained_source_bytes.saturating_add(source.len()),
        resource_limits.max_expanded_source_bytes,
    )?;
    ensure_resource(
        ResourceKind::BatchRuns,
        decks.len().saturating_add(1),
        resource_limits.max_batch_runs,
    )?;
    poll_text_abort(abort, source)?;
    let mut owned = String::new();
    owned
        .try_reserve_exact(source.len())
        .map_err(|error| allocation_error("plain run deck source", error))?;
    owned.push_str(source);
    push_owned_deck(decks, retained_source_bytes, label, owned)
}

fn push_owned_deck(
    decks: &mut Vec<RunDeck>,
    retained_source_bytes: &mut usize,
    label: Option<String>,
    source: String,
) -> Result<(), MultiRunError> {
    decks
        .try_reserve(1)
        .map_err(|error| allocation_error("run deck list", error))?;
    *retained_source_bytes = retained_source_bytes.saturating_add(source.len());
    decks.push(RunDeck { label, source });
    Ok(())
}

fn ensure_resource(
    resource: ResourceKind,
    requested: usize,
    limit: usize,
) -> Result<(), MultiRunError> {
    ResourceLimitError::ensure(resource, requested, limit).map_err(MultiRunError::resource_limit)
}

#[inline]
fn ensure_not_aborted(abort: &dyn AbortSignal) -> Result<(), MultiRunError> {
    if abort.is_aborted() {
        Err(MultiRunError::aborted())
    } else {
        Ok(())
    }
}

#[inline]
fn poll_abort(abort: &dyn AbortSignal, index: usize) -> Result<(), MultiRunError> {
    const POLL_STRIDE: usize = 64;
    if index.is_multiple_of(POLL_STRIDE) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

fn poll_text_abort(abort: &dyn AbortSignal, text: &str) -> Result<(), MultiRunError> {
    const TEXT_CHUNK_BYTES: usize = 4096;
    for _ in text.as_bytes().chunks(TEXT_CHUNK_BYTES) {
        ensure_not_aborted(abort)?;
    }
    Ok(())
}

fn allocation_error(resource: &str, error: std::collections::TryReserveError) -> MultiRunError {
    MultiRunError::new(format!("unable to reserve memory for {resource}: {error}"))
}

/// Reassemble deck lines with a terminating `.end`.
fn assemble(
    lines: &[String],
    capacity: usize,
    abort: &dyn AbortSignal,
) -> Result<String, MultiRunError> {
    let mut out = String::new();
    out.try_reserve_exact(capacity)
        .map_err(|error| allocation_error("expanded run deck source", error))?;
    for (index, line) in lines.iter().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;
        out.push_str(line);
        out.push('\n');
    }
    out.push_str(".end\n");
    Ok(out)
}

fn assembled_len(lines: &[String]) -> usize {
    lines.iter().fold(".end\n".len(), |total, line| {
        total.saturating_add(line.len()).saturating_add(1)
    })
}

/// First whitespace-delimited token of a line (empty for blank lines).
fn first_token(line: &str) -> &str {
    line.split_whitespace().next().unwrap_or("")
}

/// Split the deck at its `.ALTER` markers. The returned base and blocks
/// carry raw lines without the final `.END`.
fn split_alters(
    source: &str,
    abort: &dyn AbortSignal,
) -> Result<(Vec<String>, Vec<AlterBlock>), MultiRunError> {
    let mut base = Vec::new();
    let mut alters: Vec<AlterBlock> = Vec::new();
    let mut in_alter = false;

    for (index, line) in source.lines().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;
        let token = first_token(line);
        if token.eq_ignore_ascii_case(".alter") {
            let title = line
                .split_whitespace()
                .skip(1)
                .collect::<Vec<_>>()
                .join(" ");
            alters.push(AlterBlock {
                title,
                lines: Vec::new(),
            });
            in_alter = true;
            continue;
        }
        if token.eq_ignore_ascii_case(".end") {
            break;
        }
        if in_alter {
            if let Some(block) = alters.last_mut() {
                block.lines.push(line.to_owned());
            }
        } else {
            base.push(line.to_owned());
        }
    }
    Ok((base, alters))
}

/// Apply one alter block to the deck lines (HSPICE substitution rules).
fn apply_alter(
    lines: &mut Vec<String>,
    block: &AlterBlock,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    for (statement_index, statement) in statements(&block.lines, abort)?.into_iter().enumerate() {
        poll_abort(abort, statement_index)?;
        let head = first_token(&statement[0]);
        if head.is_empty() || head.starts_with('*') {
            continue;
        }
        if head.starts_with('.') {
            if head.eq_ignore_ascii_case(".param") {
                for (line_index, line) in statement.iter().enumerate() {
                    poll_abort(abort, line_index)?;
                    poll_text_abort(abort, line)?;
                    for (name, value) in line_assignments(line) {
                        override_param_with_abort(lines, &name, &value, abort)?;
                    }
                }
            } else if head.eq_ignore_ascii_case(".model") {
                let name = statement[0].split_whitespace().nth(1).unwrap_or("");
                replace_statement(
                    lines,
                    &statement,
                    |first| {
                        first_token(first).eq_ignore_ascii_case(".model")
                            && first
                                .split_whitespace()
                                .nth(1)
                                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
                    },
                    abort,
                )?;
            } else {
                lines.extend(statement);
            }
        } else {
            // Element statement: replace the element with the same name.
            replace_statement(
                lines,
                &statement,
                |first| first_token(first).eq_ignore_ascii_case(head),
                abort,
            )?;
        }
    }
    ensure_not_aborted(abort)
}

/// Group raw lines into statements: each statement is a line plus its
/// `+` continuation lines.
fn statements(
    lines: &[String],
    abort: &dyn AbortSignal,
) -> Result<Vec<Vec<String>>, MultiRunError> {
    let mut out: Vec<Vec<String>> = Vec::new();
    for (index, line) in lines.iter().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;
        if line.trim_start().starts_with('+')
            && let Some(last) = out.last_mut()
        {
            last.push(line.clone());
            continue;
        }
        out.push(vec![line.clone()]);
    }
    Ok(out)
}

/// Replace the first statement whose head line matches `matches` with
/// `replacement` (taking the old statement's continuation lines along);
/// append when nothing matches.
fn replace_statement(
    lines: &mut Vec<String>,
    replacement: &[String],
    matches: impl Fn(&str) -> bool,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    let mut index = 0;
    while index < lines.len() {
        poll_abort(abort, index)?;
        let line = &lines[index];
        if !line.trim_start().starts_with('+') && matches(line) {
            let mut end = index + 1;
            while end < lines.len() && lines[end].trim_start().starts_with('+') {
                poll_abort(abort, end)?;
                end += 1;
            }
            lines.splice(index..end, replacement.iter().cloned());
            return Ok(());
        }
        index += 1;
    }
    lines.extend(replacement.iter().cloned());
    Ok(())
}

/// Override every top-level `.param` assignment of `name` in place; insert
/// a fresh assignment after the title when the deck never assigns it.
/// In-place replacement preserves the deck's in-order parameter-evaluation
/// semantics (an appended line would lose to later defaults).
#[cfg(test)]
fn override_param(lines: &mut Vec<String>, name: &str, value: &str) {
    override_param_with_abort(lines, name, value, &NoAbort)
        .expect("NoAbort cannot cancel parameter override");
}

fn override_param_with_abort(
    lines: &mut Vec<String>,
    name: &str,
    value: &str,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    let mut replaced = false;
    let mut subckt_depth = 0usize;
    let mut in_param_statement = false;

    for (index, line) in lines.iter_mut().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;
        let token = first_token(line).to_ascii_lowercase();
        match token.as_str() {
            ".subckt" => subckt_depth += 1,
            ".ends" => subckt_depth = subckt_depth.saturating_sub(1),
            _ => {}
        }
        let continues = in_param_statement && line.trim_start().starts_with('+');
        if token == ".param" || continues {
            in_param_statement = true;
            if subckt_depth == 0 && rewrite_assignment(line, name, value) {
                replaced = true;
            }
        } else if !line.trim().is_empty() && !line.trim_start().starts_with('*') {
            in_param_statement = false;
        }
    }

    if !replaced {
        let insert_at = if lines.is_empty() { 0 } else { 1 };
        lines.insert(insert_at, format!(".param {name}={value}"));
    }
    Ok(())
}

/// Rewrite `name`'s assignment value inside one physical line. Returns
/// whether a rewrite happened.
fn rewrite_assignment(line: &mut String, name: &str, value: &str) -> bool {
    let Some((start, end)) = assignment_value_span(line, name) else {
        return false;
    };
    let mut next = String::with_capacity(line.len() + value.len());
    next.push_str(&line[..start]);
    next.push_str(value);
    next.push_str(&line[end..]);
    *line = next;
    true
}

/// All `name=value` assignments on one line as owned pairs.
fn line_assignments(line: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    for (name, start, end) in scan_assignments(line) {
        out.push((name, line[start..end].to_owned()));
    }
    out
}

/// Byte span of the value assigned to `name` on this line, if present.
fn assignment_value_span(line: &str, name: &str) -> Option<(usize, usize)> {
    scan_assignments(line)
        .into_iter()
        .find(|(candidate, _, _)| candidate.eq_ignore_ascii_case(name))
        .map(|(_, start, end)| (start, end))
}

/// Scan `name = value` pairs on a line, returning `(name, value_start,
/// value_end)` byte spans. Values may be `{expressions}` (brace-balanced)
/// or bare tokens; whitespace around `=` is tolerated.
fn scan_assignments(line: &str) -> Vec<(String, usize, usize)> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let name_start = i;
        while i < bytes.len() {
            let ch = bytes[i] as char;
            if ch.is_whitespace() || ch == '=' {
                break;
            }
            i += 1;
        }
        let name = &line[name_start..i];
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] != b'=' {
            continue;
        }
        i += 1;
        while i < bytes.len() && (bytes[i] as char).is_whitespace() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let value_start = i;
        if bytes[i] == b'{' || bytes[i] == b'\'' {
            let close = if bytes[i] == b'{' { b'}' } else { b'\'' };
            let mut depth = 0i32;
            while i < bytes.len() {
                if bytes[i] == b'{' && close == b'}' {
                    depth += 1;
                } else if bytes[i] == close {
                    depth -= 1;
                    if depth <= 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
        } else {
            while i < bytes.len() && !(bytes[i] as char).is_whitespace() {
                i += 1;
            }
        }
        out.push((name.to_owned(), value_start, i));
    }
    out
}

/// Pull `.DATA … .ENDDATA` blocks out of the deck, returning the parsed
/// tables and the deck lines with the blocks removed.
fn extract_data_tables(
    lines: Vec<String>,
    resource_limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(Vec<DataTable>, Vec<String>), MultiRunError> {
    let mut tables: Vec<DataTable> = Vec::new();
    let mut kept = Vec::with_capacity(lines.len());
    let mut current: Option<DataTable> = None;
    let mut flat_values: Vec<Value> = Vec::new();

    for (line_index, line) in lines.into_iter().enumerate() {
        poll_abort(abort, line_index)?;
        poll_text_abort(abort, &line)?;
        let line_number = line_index + 1;
        let token = first_token(&line);
        if let Some(table) = current.as_mut() {
            if token.eq_ignore_ascii_case(".enddata") {
                if table.params.is_empty() {
                    return Err(MultiRunError::new(format!(
                        ".data {} has no parameter columns",
                        table.name
                    )));
                }
                let columns = table.params.len();
                if !flat_values.len().is_multiple_of(columns) {
                    return Err(MultiRunError::new(format!(
                        ".data {} has {} values, which does not fill {} columns",
                        table.name,
                        flat_values.len(),
                        columns
                    )));
                }
                let row_count = flat_values.len() / columns;
                ensure_resource(
                    ResourceKind::AnalysisPoints,
                    row_count,
                    resource_limits.max_analysis_points,
                )?;
                table
                    .rows
                    .try_reserve_exact(row_count)
                    .map_err(|error| allocation_error(".DATA rows", error))?;
                for (row_index, chunk) in flat_values.chunks_exact(columns).enumerate() {
                    poll_abort(abort, row_index)?;
                    table.rows.push(chunk.to_vec());
                }
                flat_values.clear();
                tables.push(current.take().unwrap());
            } else {
                for (value_index, raw) in line.split_whitespace().enumerate() {
                    poll_abort(abort, value_index)?;
                    let raw = raw.trim_start_matches('+');
                    if raw.is_empty() {
                        continue;
                    }
                    match parse_spice_value(raw) {
                        Ok(value) => {
                            ensure_resource(
                                ResourceKind::ResultValues,
                                flat_values.len().saturating_add(1),
                                resource_limits.max_result_values,
                            )?;
                            flat_values.push(value);
                            if !table.params.is_empty() {
                                ensure_resource(
                                    ResourceKind::AnalysisPoints,
                                    flat_values.len().div_ceil(table.params.len()),
                                    resource_limits.max_analysis_points,
                                )?;
                            }
                        }
                        Err(_) => {
                            return Err(MultiRunError::new(format!(
                                ".data {} line {} contains non-numeric token `{raw}`",
                                table.name, line_number
                            )));
                        }
                    }
                }
            }
            continue;
        }
        if token.eq_ignore_ascii_case(".enddata") {
            return Err(MultiRunError::new(format!(
                ".enddata without matching .data at line {line_number}"
            )));
        }
        if token.eq_ignore_ascii_case(".data") {
            let mut fields = line.split_whitespace().skip(1);
            let Some(name) = fields.next() else {
                return Err(MultiRunError::new(format!(
                    ".data at line {line_number} is missing a table name"
                )));
            };
            current = Some(DataTable {
                name: name.to_owned(),
                params: fields.map(|field| field.to_owned()).collect(),
                rows: Vec::new(),
            });
            continue;
        }
        kept.push(line);
    }

    if let Some(table) = current {
        return Err(MultiRunError::new(format!(
            ".data {} not closed by .enddata",
            table.name
        )));
    }
    ensure_not_aborted(abort)?;
    Ok((tables, kept))
}

/// Find the first analysis line referencing `DATA=<table>`, resolved
/// against the extracted tables. Returns `(table_index, line_index)`.
fn find_data_reference(
    lines: &[String],
    tables: &[DataTable],
    abort: &dyn AbortSignal,
) -> Result<Option<(usize, usize)>, MultiRunError> {
    for (line_index, line) in lines.iter().enumerate() {
        poll_abort(abort, line_index)?;
        poll_text_abort(abort, line)?;
        if !is_sweep_analysis(line) {
            continue;
        }
        let Some(name) = data_reference_name(line) else {
            continue;
        };
        match tables
            .iter()
            .position(|table| table.name.eq_ignore_ascii_case(&name))
        {
            Some(table_index) => return Ok(Some((table_index, line_index))),
            None => {
                return Err(MultiRunError::new(format!(
                    "analysis line {} references unknown .data table `{name}`",
                    line_index + 1
                )));
            }
        }
    }
    Ok(None)
}

/// Whether a statement is an analysis line referencing a `.DATA` table —
/// the parser skips such lines (with a warning) when a deck is parsed
/// without going through multi-run expansion first.
pub(crate) fn references_data_table(line: &str) -> bool {
    is_sweep_analysis(line) && data_reference_name(line).is_some()
}

fn is_sweep_analysis(line: &str) -> bool {
    let token = first_token(line);
    token.eq_ignore_ascii_case(".dc") || token.eq_ignore_ascii_case(".tran")
}

/// The table name referenced by `DATA=<name>` on this line, if any.
fn data_reference_name(line: &str) -> Option<String> {
    scan_assignments(line)
        .into_iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case("data"))
        .map(|(_, start, end)| line[start..end].to_owned())
}

/// Remove `[SWEEP] DATA=<name>` from every analysis line; a `.dc` left
/// with no sweep arguments becomes `.op` (one point per data row).
fn strip_data_tokens(lines: &mut [String], abort: &dyn AbortSignal) -> Result<(), MultiRunError> {
    for (index, line) in lines.iter_mut().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;
        if !is_sweep_analysis(line) || data_reference_name(line).is_none() {
            continue;
        }
        let is_dc = first_token(line).eq_ignore_ascii_case(".dc");
        let mut kept: Vec<&str> = Vec::new();
        let normalized = normalize_assignment_spacing(line);
        for field in normalized.split_whitespace() {
            let lower = field.to_ascii_lowercase();
            if lower.starts_with("data=") {
                if kept
                    .last()
                    .is_some_and(|prev| prev.eq_ignore_ascii_case("sweep"))
                {
                    kept.pop();
                }
                continue;
            }
            kept.push(field);
        }
        *line = if is_dc && kept.len() <= 1 {
            ".op".to_owned()
        } else {
            kept.join(" ")
        };
    }
    Ok(())
}

/// Collapse whitespace around `=` so `data = name` tokenizes as one field.
fn normalize_assignment_spacing(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            // Drop the whitespace run when it borders an '='.
            while chars.peek().is_some_and(|next| next.is_whitespace()) {
                chars.next();
            }
            if chars.peek() == Some(&'=') || out.ends_with('=') {
                continue;
            }
            out.push(' ');
            continue;
        }
        out.push(ch);
    }
    out
}

/// Format a row value as a SPICE literal (round-trippable, no suffix).
fn format_value(value: Value) -> String {
    if value == value.trunc() && value.abs() < 1e15 {
        format!("{}", value)
    } else {
        format!("{value:e}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "test deck\n\
        .param rload=1k\n\
        V1 in 0 DC 5\n\
        R1 in out {rload}\n\
        + m=1\n\
        C1 out 0 1u\n\
        .model d1 d is=1e-14\n\
        .tran 1u 1m\n\
        .end\n";

    #[test]
    fn plain_deck_passes_through_untouched() {
        let decks = expand_multi_run(BASE);
        assert_eq!(decks.len(), 1);
        assert!(decks[0].label.is_none());
        assert_eq!(decks[0].source, BASE);
    }

    #[test]
    fn alter_replaces_elements_models_and_params_cumulatively() {
        let source = "test deck\n\
            .param rload=1k\n\
            V1 in 0 DC 5\n\
            R1 in out {rload}\n\
            + m=1\n\
            .model d1 d is=1e-14\n\
            .tran 1u 1m\n\
            .alter hot\n\
            .param rload=2k\n\
            .temp 125\n\
            .alter swap source\n\
            V1 in 0 DC 3\n\
            .model d1 d is=2e-14 n=1.5\n\
            .end\n";

        let decks = expand_multi_run(source);
        assert_eq!(decks.len(), 3);
        assert_eq!(decks[0].label.as_deref(), Some("base"));
        assert_eq!(decks[1].label.as_deref(), Some("hot"));
        assert_eq!(decks[2].label.as_deref(), Some("swap source"));

        // Alter 1: the param overrides in place; .temp appends.
        assert!(decks[1].source.contains(".param rload=2k"));
        assert!(!decks[1].source.contains("rload=1k"));
        assert!(decks[1].source.contains(".temp 125"));
        // The replaced element keeps only the replacement (continuation
        // lines of the old statement must not survive).
        assert!(decks[1].source.contains("R1 in out {rload}"));

        // Alter 2 is cumulative: it still carries alter 1's edits.
        assert!(decks[2].source.contains(".param rload=2k"));
        assert!(decks[2].source.contains(".temp 125"));
        assert!(decks[2].source.contains("V1 in 0 DC 3"));
        assert!(!decks[2].source.contains("V1 in 0 DC 5"));
        assert!(decks[2].source.contains(".model d1 d is=2e-14 n=1.5"));
        assert!(!decks[2].source.contains("is=1e-14"));
        // Every deck terminates.
        for deck in &decks {
            assert!(deck.source.trim_end().ends_with(".end"));
        }
    }

    #[test]
    fn element_replacement_takes_continuations_along() {
        let source = "t\n\
            R1 a b 1k\n\
            + temp=27\n\
            .alter\n\
            R1 a b 2k\n\
            .end\n";
        let decks = expand_multi_run(source);
        assert!(decks[1].source.contains("R1 a b 2k"));
        assert!(!decks[1].source.contains("temp=27"));
    }

    #[test]
    fn alter_appends_new_statements() {
        let source = "t\nR1 a 0 1k\n.alter\nR2 a 0 2k\n.end\n";
        let decks = expand_multi_run(source);
        assert!(decks[1].source.contains("R1 a 0 1k"));
        assert!(decks[1].source.contains("R2 a 0 2k"));
    }

    #[test]
    fn data_sweep_expands_rows_and_binds_params() {
        let source = "t\n\
            .param vdd=1 rl=1k\n\
            V1 in 0 {vdd}\n\
            R1 in 0 {rl}\n\
            .data tbl vdd rl\n\
            1.6 900\n\
            1.8 1k\n\
            2.0 1.1k\n\
            .enddata\n\
            .dc data=tbl\n\
            .end\n";
        let decks = expand_multi_run(source);
        assert_eq!(decks.len(), 3);
        assert_eq!(decks[0].label.as_deref(), Some("tbl row 1"));
        // Row values override the .param assignments in place.
        assert!(decks[0].source.contains("vdd=1.6"));
        assert!(decks[0].source.contains("rl=900"));
        assert!(decks[2].source.contains("vdd=2"));
        assert!(decks[2].source.contains("rl=1100"));
        // `.dc data=` is one operating point per row.
        assert!(decks[0].source.contains(".op"));
        assert!(!decks[0].source.to_lowercase().contains("data=tbl"));
        // The table block itself is stripped from the emitted decks.
        assert!(!decks[0].source.to_lowercase().contains(".enddata"));
    }

    #[test]
    fn checked_data_sweep_rejects_malformed_tables() {
        let cases = [
            (
                "ragged",
                "t\n.data tbl vdd rl\n1.0 1k\n2.0\n.enddata\n.dc data=tbl\n.end\n",
                "does not fill 2 columns",
            ),
            (
                "non_numeric",
                "t\n.data tbl vdd\n1.0\nbad\n.enddata\n.dc data=tbl\n.end\n",
                "non-numeric token `bad`",
            ),
            (
                "unclosed",
                "t\n.data tbl vdd\n1.0\n",
                "not closed by .enddata",
            ),
            (
                "missing_name",
                "t\n.data\n1.0\n.enddata\n.dc data=tbl\n.end\n",
                "missing a table name",
            ),
            (
                "empty",
                "t\n.data tbl vdd\n.enddata\n.dc data=tbl\n.end\n",
                ".data tbl has no rows",
            ),
            (
                "unknown_reference",
                "t\nV1 a 0 1\n.dc data=missing\n.end\n",
                "unknown .data table `missing`",
            ),
            (
                "dangling_enddata",
                "t\nV1 a 0 1\n.enddata\n.op\n.end\n",
                ".enddata without matching .data",
            ),
        ];

        for (label, source, needle) in cases {
            let err = try_expand_multi_run(source).expect_err(&format!("{label} should reject"));
            assert!(
                err.to_string().contains(needle),
                "{label}: expected `{needle}`, got `{err}`"
            );
        }
    }

    #[test]
    fn tran_sweep_data_keeps_the_analysis() {
        let source = "t\n\
            .param cap=1p\n\
            C1 a 0 {cap}\n\
            R1 a 0 1k\n\
            V1 a 0 PULSE(0 1 0 1n 1n 1u 2u)\n\
            .data pts cap\n\
            1p\n\
            2p\n\
            .enddata\n\
            .tran 1n 4u sweep data=pts\n\
            .end\n";
        let decks = expand_multi_run(source);
        assert_eq!(decks.len(), 2);
        assert!(decks[0].source.contains(".tran 1n 4u"));
        assert!(!decks[0].source.to_lowercase().contains("sweep"));
        assert!(decks[1].source.contains("cap=2e-12"));
    }

    #[test]
    fn alters_cross_with_data_rows() {
        let source = "t\n\
            .param vdd=1\n\
            V1 a 0 {vdd}\n\
            R1 a 0 1k\n\
            .data tbl vdd\n\
            1.0\n\
            2.0\n\
            .enddata\n\
            .dc data=tbl\n\
            .alter big load\n\
            R1 a 0 10k\n\
            .end\n";
        let decks = expand_multi_run(source);
        assert_eq!(decks.len(), 4);
        assert_eq!(decks[0].label.as_deref(), Some("base · tbl row 1"));
        assert_eq!(decks[3].label.as_deref(), Some("big load · tbl row 2"));
        assert!(decks[3].source.contains("R1 a 0 10k"));
        assert!(decks[3].source.contains("vdd=2"));
    }

    #[test]
    fn expansion_enforces_total_batch_runs_before_retaining_cross_product() {
        let source = "t\n\
            .param vdd=1\n\
            V1 a 0 {vdd}\n\
            .data tbl vdd\n\
            1\n\
            2\n\
            3\n\
            .enddata\n\
            .dc data=tbl\n\
            .end\n";
        let limits = ResourceLimits {
            max_batch_runs: 2,
            ..ResourceLimits::default()
        };

        let error = try_expand_multi_run_with_limits(source, limits)
            .expect_err("three rows must exceed the two-run policy");

        assert_eq!(
            error.resource_limit_error(),
            Some(ResourceLimitError {
                resource: ResourceKind::BatchRuns,
                requested: 3,
                limit: 2,
            })
        );
    }

    #[test]
    fn expansion_bounds_aggregate_retained_deck_source_bytes() {
        let source = "t\n\
            .param vdd=1\n\
            V1 a 0 {vdd}\n\
            .data tbl vdd\n\
            1\n\
            2\n\
            .enddata\n\
            .dc data=tbl\n\
            .end\n";
        let expanded = try_expand_multi_run(source).expect("fixture expands");
        let retained_bytes = expanded.iter().map(|deck| deck.source.len()).sum::<usize>();
        let limits = ResourceLimits {
            max_expanded_source_bytes: retained_bytes - 1,
            ..ResourceLimits::default()
        };

        let error = try_expand_multi_run_with_limits(source, limits)
            .expect_err("aggregate concrete deck bytes must be bounded");
        let resource = error
            .resource_limit_error()
            .expect("failure must retain typed resource details");
        assert_eq!(resource.resource, ResourceKind::ExpandedSourceBytes);
        assert_eq!(resource.requested, retained_bytes);
        assert_eq!(resource.limit, retained_bytes - 1);
    }

    #[test]
    fn alter_variants_are_preflighted_against_batch_limit() {
        let source = "t\nR1 a 0 1k\n.alter one\nR1 a 0 2k\n.alter two\nR1 a 0 3k\n.end\n";
        let limits = ResourceLimits {
            max_batch_runs: 2,
            ..ResourceLimits::default()
        };

        let error = try_expand_multi_run_with_limits(source, limits)
            .expect_err("base plus two alters must exceed a two-run policy");

        assert_eq!(
            error.resource_limit_error(),
            Some(ResourceLimitError {
                resource: ResourceKind::BatchRuns,
                requested: 3,
                limit: 2,
            })
        );
    }

    #[test]
    fn expansion_observes_cooperative_cancellation_during_text_work() {
        let mut source = String::from("cancel expansion\n");
        for index in 0..256 {
            source.push_str(&format!("R{index} n{index} 0 1k\n"));
        }
        source.push_str(".alter second\nR0 n0 0 2k\n.end\n");
        let abort = crate::abort_signal::CountingAbort::new(12);

        let error =
            try_expand_multi_run_with_limits_and_abort(&source, ResourceLimits::default(), &abort)
                .expect_err("counting abort must stop expansion");

        assert!(error.is_aborted());
        assert!(abort.count() > 12, "expansion must poll during text work");
    }

    #[test]
    fn param_override_inserts_when_missing_and_skips_subckt_scope() {
        let mut lines: Vec<String> = vec![
            "title".into(),
            ".subckt cell a b".into(),
            ".param w=1u".into(),
            ".ends".into(),
        ];
        override_param(&mut lines, "w", "2u");
        // The subckt-scoped assignment is untouched; a top-level insert
        // lands after the title.
        assert_eq!(lines[1], ".param w=2u");
        assert!(lines.iter().any(|l| l == ".param w=1u"));
    }

    #[test]
    fn assignment_scanner_handles_braces_and_spacing() {
        let spans = scan_assignments(".param a = {x*2} b=3k data = tbl");
        let names: Vec<&str> = spans.iter().map(|(n, _, _)| n.as_str()).collect();
        assert_eq!(names, ["a", "b", "data"]);
        let line = ".param a = {x*2} b=3k data = tbl";
        assert_eq!(&line[spans[0].1..spans[0].2], "{x*2}");
        assert_eq!(&line[spans[2].1..spans[2].2], "tbl");
    }
}

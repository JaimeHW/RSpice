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
use crate::netlist::data_table::data_table_parameter_name_is_valid;
use crate::netlist::lexer::parse_spice_value;
use crate::resource::{ResourceKind, ResourceLimitError, ResourceLimits};
use std::collections::BTreeSet;

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
pub(crate) fn try_expand_multi_run_with_limits(
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
    let has_step_data = has_step_data_reference(source_lines);
    let has_textual_data_sweep = source_lines.iter().any(|line| references_data_table(line));
    if has_step_data {
        if has_textual_data_sweep {
            return Err(MultiRunError::new(
                ".STEP DATA cannot be combined with textual .DC/.TRAN DATA expansion in one deck",
            ));
        }
        // `.STEP DATA` is expanded by the typed engine planner. Preserve the
        // complete table block so the parser can attach it to the netlist;
        // textual multi-run expansion must not consume its ownership first.
        return push_assembled_deck(
            decks,
            retained_source_bytes,
            label,
            source_lines,
            resource_limits,
            abort,
        );
    }
    let (tables, mut lines) = extract_data_tables(source_lines.to_vec(), resource_limits, abort)?;
    let references = find_data_references(&lines, &tables, abort)?;

    match references.as_slice() {
        [] => push_assembled_deck(
            decks,
            retained_source_bytes,
            label,
            &lines,
            resource_limits,
            abort,
        )?,
        references => {
            let first_table = &tables[references[0].0];
            if first_table.rows.is_empty() {
                return Err(MultiRunError::new(format!(
                    ".data {} has no rows",
                    first_table.name
                )));
            }
            let row_count = first_table.rows.len();
            let mut seen_columns = BTreeSet::new();
            for (table_index, _) in references {
                let table = &tables[*table_index];
                if table.rows.len() != row_count {
                    return Err(MultiRunError::new(format!(
                        ".data {} has {} rows, expected {row_count} to match the other table-driven analysis columns",
                        table.name,
                        table.rows.len()
                    )));
                }
                for column in &table.params {
                    if !seen_columns.insert(column.to_ascii_uppercase()) {
                        return Err(MultiRunError::new(format!(
                            ".data column `{column}` is specified more than once across the active tables"
                        )));
                    }
                }
            }
            ensure_resource(
                ResourceKind::BatchRuns,
                decks.len().saturating_add(row_count),
                resource_limits.max_batch_runs,
            )?;
            prepare_data_analysis(&mut lines, references[0].1, abort)?;
            let table_label = references
                .iter()
                .map(|(table_index, _)| tables[*table_index].name.as_str())
                .collect::<Vec<_>>()
                .join("+");
            for row_index in 0..row_count {
                poll_abort(abort, row_index)?;
                let mut run_lines = lines.clone();
                let mut column_index = 0usize;
                for (table_index, _) in references {
                    let table = &tables[*table_index];
                    for (param, value) in table.params.iter().zip(&table.rows[row_index]) {
                        poll_abort(abort, column_index)?;
                        column_index = column_index.saturating_add(1);
                        apply_data_column_override_with_abort(
                            &mut run_lines,
                            param,
                            &format_value(*value),
                            abort,
                        )?;
                    }
                }
                let row_label = format!("{table_label} row {}", row_index + 1);
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
    if rewrite_top_level_param_assignments_with_abort(lines, name, value, abort)? {
        return Ok(());
    }

    let insert_at = if lines.is_empty() { 0 } else { 1 };
    lines.insert(insert_at, format!(".param {name}={value}"));
    Ok(())
}

/// Apply one `.DATA` column value using Xyce's sweep-target precedence.
/// Declared top-level parameters win; otherwise a matching device target is
/// overridden through its canonical named instance parameter. A name with no
/// declared parameter or device remains a parameter binding so expressions
/// that reference an otherwise undefined table parameter can resolve when the
/// concrete row is parsed.
fn apply_data_column_override_with_abort(
    lines: &mut Vec<String>,
    name: &str,
    value: &str,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    if rewrite_top_level_param_assignments_with_abort(lines, name, value, abort)? {
        return Ok(());
    }
    if append_device_parameter_override_with_abort(lines, name, value, abort)? {
        return Ok(());
    }

    let insert_at = if lines.is_empty() { 0 } else { 1 };
    lines.insert(insert_at, format!(".param {name}={value}"));
    Ok(())
}

fn rewrite_top_level_param_assignments_with_abort(
    lines: &mut [String],
    name: &str,
    value: &str,
    abort: &dyn AbortSignal,
) -> Result<bool, MultiRunError> {
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
        if matches!(
            token.as_str(),
            ".param" | ".params" | ".csparam" | ".global_param"
        ) || continues
        {
            in_param_statement = true;
            if subckt_depth == 0 && rewrite_assignment(line, name, value) {
                replaced = true;
            }
        } else if !line.trim().is_empty() && !line.trim_start().starts_with('*') {
            in_param_statement = false;
        }
    }
    Ok(replaced)
}

/// Append a named instance override for one top-level `.DATA` device column.
/// Bare passive names select their primary value (`R`, `C`, or `L`); an
/// explicit `DEVICE:PARAM` spelling retains the requested parameter. Appending
/// the named value is intentional: the ordinary parser's last-write semantics
/// replace an earlier positional or named value without lossy token surgery.
fn append_device_parameter_override_with_abort(
    lines: &mut [String],
    target: &str,
    value: &str,
    abort: &dyn AbortSignal,
) -> Result<bool, MultiRunError> {
    let (device_name, explicit_parameter) = target
        .rsplit_once(':')
        .map_or((target, None), |(device, parameter)| {
            (device, Some(parameter))
        });
    if device_name.is_empty()
        || explicit_parameter.is_some_and(|parameter| {
            parameter.is_empty() || !data_table_parameter_name_is_valid(parameter)
        })
    {
        return Ok(false);
    }

    let mut subckt_depth = 0usize;
    for (index, line) in lines.iter_mut().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;
        let head = first_token(line);
        if head.eq_ignore_ascii_case(".subckt") {
            subckt_depth = subckt_depth.saturating_add(1);
            continue;
        }
        if head.eq_ignore_ascii_case(".ends") {
            subckt_depth = subckt_depth.saturating_sub(1);
            continue;
        }
        if subckt_depth != 0 || !head.eq_ignore_ascii_case(device_name) {
            continue;
        }

        let parameter = match explicit_parameter {
            Some(parameter) => parameter.to_ascii_uppercase(),
            None => match head.as_bytes().first().map(u8::to_ascii_uppercase) {
                Some(b'R') => "R".to_owned(),
                Some(b'C') => "C".to_owned(),
                Some(b'L') => "L".to_owned(),
                _ => return Ok(false),
            },
        };
        let insertion = format!(" {parameter}={value}");
        if let Some(comment_start) = line.find(';') {
            line.insert_str(comment_start, &insertion);
        } else {
            line.push_str(&insertion);
        }
        return Ok(true);
    }
    Ok(false)
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
                let body = line
                    .split_once(';')
                    .map_or(line.as_str(), |(before, _)| before)
                    .trim();
                if body.is_empty() || body.starts_with('*') {
                    continue;
                }
                let body = body.strip_prefix('+').unwrap_or(body).trim();
                if body.is_empty() {
                    continue;
                }

                if table.params.is_empty() {
                    for (param_index, param) in body.split_whitespace().enumerate() {
                        poll_abort(abort, param_index)?;
                        if !data_table_parameter_name_is_valid(param) {
                            return Err(MultiRunError::new(format!(
                                ".data {} line {} parameter column `{param}` is not a valid parameter name",
                                table.name, line_number
                            )));
                        }
                        table.params.push(param.to_owned());
                    }
                    continue;
                }

                for (value_index, raw) in body.split_whitespace().enumerate() {
                    poll_abort(abort, value_index)?;
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
            let params = fields.map(|field| field.to_owned()).collect::<Vec<_>>();
            for param in &params {
                if !data_table_parameter_name_is_valid(param) {
                    return Err(MultiRunError::new(format!(
                        ".data {name} line {line_number} parameter column `{param}` is not a valid parameter name"
                    )));
                }
            }
            current = Some(DataTable {
                name: name.to_owned(),
                params,
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

/// Find the table references belonging to one table-driven analysis. Xyce
/// combines multiple `DATA=<table>` cards of the same analysis kind by row,
/// with every table contributing columns. Different analysis kinds would
/// imply an unrepresented cross-product and therefore fail closed.
fn find_data_references(
    lines: &[String],
    tables: &[DataTable],
    abort: &dyn AbortSignal,
) -> Result<Vec<(usize, usize)>, MultiRunError> {
    let mut references = Vec::new();
    let mut analysis_kind: Option<String> = None;
    for (line_index, line) in lines.iter().enumerate() {
        poll_abort(abort, line_index)?;
        poll_text_abort(abort, line)?;
        if !is_sweep_analysis(line) {
            continue;
        }
        let Some(name) = data_reference_name(line) else {
            continue;
        };
        let table_index = match tables
            .iter()
            .position(|table| table.name.eq_ignore_ascii_case(&name))
        {
            Some(table_index) => table_index,
            None => {
                return Err(MultiRunError::new(format!(
                    "analysis line {} references unknown .data table `{name}`",
                    line_index + 1
                )));
            }
        };
        let kind = first_token(line).to_ascii_lowercase();
        if let Some(previous_kind) = analysis_kind.as_deref()
            && previous_kind != kind
        {
            return Err(MultiRunError::new(format!(
                "table-driven analyses of kinds `{previous_kind}` and `{kind}` cannot be combined into one textual multi-run expansion"
            )));
        }
        analysis_kind = Some(kind);
        references.push((table_index, line_index));
    }
    Ok(references)
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

/// Whether any logical `.STEP` statement owns a `.DATA` table. Blank and
/// full-line comment cards do not terminate a pending logical statement, and
/// every `+` fragment is assembled before assignment scanning. Inside a table
/// body, a leading `+` remains row data rather than netlist continuation.
fn has_step_data_reference(lines: &[String]) -> bool {
    let mut in_data_table = false;
    let mut logical_statement = String::new();

    for line in lines {
        let trimmed = strip_multi_run_inline_comment(line).trim();
        if trimmed.is_empty()
            || trimmed.starts_with('*')
            || trimmed.starts_with('$')
            || trimmed.starts_with("//")
        {
            continue;
        }
        let token = first_token(trimmed);

        if in_data_table {
            if token.eq_ignore_ascii_case(".enddata") {
                in_data_table = false;
            }
            continue;
        }

        if token.eq_ignore_ascii_case(".data") {
            if logical_step_references_data(&logical_statement) {
                return true;
            }
            logical_statement.clear();
            in_data_table = true;
            continue;
        }

        if let Some(continuation) = trimmed.strip_prefix('+') {
            if !logical_statement.is_empty() {
                logical_statement.push(' ');
                logical_statement.push_str(continuation.trim());
            }
            continue;
        }

        if logical_step_references_data(&logical_statement) {
            return true;
        }
        logical_statement.clear();
        logical_statement.push_str(trimmed);
    }

    logical_step_references_data(&logical_statement)
}

fn logical_step_references_data(statement: &str) -> bool {
    first_token(statement).eq_ignore_ascii_case(".step") && data_reference_name(statement).is_some()
}

/// Strip semicolon comments without treating markers inside authored strings
/// as comments. Multi-run expansion precedes dialect selection, so full-line
/// `*`, `$`, and `//` handling stays in the logical-line owner above.
fn strip_multi_run_inline_comment(line: &str) -> &str {
    let mut single_quoted = false;
    let mut double_quoted = false;
    let mut escaped = false;
    for (index, character) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match character {
            '\\' if single_quoted || double_quoted => escaped = true,
            '\'' if !double_quoted => single_quoted = !single_quoted,
            '"' if !single_quoted => double_quoted = !double_quoted,
            ';' if !single_quoted && !double_quoted => return &line[..index],
            _ => {}
        }
    }
    line
}

/// The table name referenced by `DATA=<name>` on this line, if any.
fn data_reference_name(line: &str) -> Option<String> {
    scan_assignments(line)
        .into_iter()
        .find(|(name, _, _)| name.eq_ignore_ascii_case("data"))
        .map(|(_, start, end)| line[start..end].to_owned())
}

/// Prepare the selected table-driven analysis for one concrete row. A bare
/// `.DC DATA=<name>` becomes `.OP`; other analyses lose only their
/// `[SWEEP] DATA=<name>` selector. Competing cards of the same analysis kind
/// are suppressed because Xyce gives the table-driven card precedence.
fn prepare_data_analysis(
    lines: &mut [String],
    selected_line_index: usize,
    abort: &dyn AbortSignal,
) -> Result<(), MultiRunError> {
    let selected_kind = first_token(&lines[selected_line_index]).to_ascii_lowercase();
    let mut suppress_continuations = false;
    for (index, line) in lines.iter_mut().enumerate() {
        poll_abort(abort, index)?;
        poll_text_abort(abort, line)?;

        if suppress_continuations && line.trim_start().starts_with('+') {
            line.clear();
            continue;
        }
        suppress_continuations = false;

        if index != selected_line_index && first_token(line).eq_ignore_ascii_case(&selected_kind) {
            line.clear();
            suppress_continuations = true;
            continue;
        }
        if index != selected_line_index {
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
    fn step_data_preserves_its_table_for_typed_cartesian_planning() {
        let source = "typed STEP DATA\n\
            .param rval=1k\n\
            V1 in 0 1\n\
            R1 in 0 {rval}\n\
            .data coordinates rval\n\
            1k\n\
            2k\n\
            .enddata\n\
            .step data=coordinates\n\
            .op\n\
            .end\n";
        let decks = try_expand_multi_run(source).expect("STEP DATA ownership is preserved");
        assert_eq!(decks.len(), 1);
        assert!(decks[0].label.is_none());
        assert!(decks[0].source.contains(".data coordinates rval"));
        assert!(decks[0].source.contains(".enddata"));
        assert!(decks[0].source.contains(".step data=coordinates"));

        let parsed = crate::Netlist::parse(&decks[0].source).expect("preserved deck parses");
        assert_eq!(parsed.data_tables.len(), 1);
        assert!(matches!(
            parsed.analyses.as_slice(),
            [
                crate::netlist::AnalysisCommand::Step(_),
                crate::netlist::AnalysisCommand::Op
            ]
        ));
    }

    #[test]
    fn continued_step_data_preserves_its_table_for_typed_cartesian_planning() {
        let source = "continued typed STEP DATA\n\
            .param rval=1k\n\
            V1 in 0 1\n\
            R1 in 0 {rval}\n\
            .data coordinates rval\n\
            1k\n\
            2k\n\
            .enddata\n\
            .step\n\
            + data=coordinates\n\
            .op\n\
            .end\n";
        let decks = try_expand_multi_run(source).expect("continued STEP DATA is preserved");
        assert_eq!(decks.len(), 1);
        assert!(decks[0].source.contains(".data coordinates rval"));
        assert!(decks[0].source.contains(".enddata"));
        assert!(decks[0].source.contains(".step\n"));
        assert!(decks[0].source.contains("+ data=coordinates"));

        let parsed = crate::Netlist::parse(&decks[0].source).expect("preserved deck parses");
        assert_eq!(parsed.data_tables.len(), 1);
        assert!(matches!(
            parsed.analyses.as_slice(),
            [
                crate::netlist::AnalysisCommand::Step(_),
                crate::netlist::AnalysisCommand::Op
            ]
        ));
    }

    #[test]
    fn fragmented_step_data_continuations_cross_comments_and_blank_lines() {
        let source = "fragmented typed STEP DATA\n\
            .param rval=1k\n\
            V1 in 0 1\n\
            R1 in 0 {rval}\n\
            .data coordinates rval\n\
            1k\n\
            2k\n\
            .enddata\n\
            .step\n\
            * continuation comment\n\
            \n\
            + data\n\
            $ another continuation comment\n\
            + = coordinates\n\
            .op\n\
            .end\n";
        let decks = try_expand_multi_run(source).expect("fragmented STEP DATA is preserved");
        assert_eq!(decks.len(), 1);
        assert!(decks[0].source.contains(".data coordinates rval"));
        assert!(decks[0].source.contains("+ data"));
        assert!(decks[0].source.contains("+ = coordinates"));

        let parsed = crate::Netlist::parse(&decks[0].source).expect("preserved deck parses");
        assert_eq!(parsed.data_tables.len(), 1);
        assert!(matches!(
            parsed.analyses.as_slice(),
            [
                crate::netlist::AnalysisCommand::Step(_),
                crate::netlist::AnalysisCommand::Op
            ]
        ));
    }

    #[test]
    fn step_data_and_textual_data_expansion_fail_before_consuming_the_table() {
        let source = "ambiguous DATA ownership\n\
            V1 in 0 1\n\
            .data coordinates rval\n\
            1k\n\
            .enddata\n\
            .step data=coordinates\n\
            .dc data=coordinates\n\
            .end\n";
        let error = try_expand_multi_run(source).expect_err("mixed DATA owners reject");
        assert!(error.to_string().contains(".STEP DATA cannot be combined"));
    }

    #[test]
    fn data_global_parameter_binding_precedes_same_named_device() {
        let source = "DATA global precedence\n\
                      .GLOBAL_PARAM R1=1\n\
                      V1 n 0 1\n\
                      R1 n 0 1k\n\
                      .DATA values R1\n\
                      2\n\
                      .ENDDATA\n\
                      .DC DATA=values\n\
                      .END\n";
        let decks = try_expand_multi_run(source).expect("DATA row expands");
        assert_eq!(decks.len(), 1);
        assert!(decks[0].source.contains(".GLOBAL_PARAM R1=2"));
        assert!(decks[0].source.contains("R1 n 0 1k"));
        assert!(!decks[0].source.contains(" R=2"));
    }

    // What a deck expands into is asserted here. That an expanded row then
    // *solves* is asserted in `tests/xyce_data_sweep.rs`, because this layer
    // may not name the engine.

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
                "missing_continuation_header",
                "t\n.data tbl\n1.0\n.enddata\n.dc data=tbl\n.end\n",
                "parameter column `1.0` is not a valid parameter name",
            ),
            (
                "mixed_continuation_header",
                "t\n.data tbl\n+ vdd 1\n+ 1 2\n.enddata\n.dc data=tbl\n.end\n",
                "parameter column `1` is not a valid parameter name",
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
    fn multiple_dc_data_tables_combine_columns_rowwise() {
        let source = "t\n\
            V1 1 0 10\n\
            R1 1 2 1\n\
            R2 2 0 1\n\
            .data left R1\n\
            1\n\
            2\n\
            .enddata\n\
            .data right R2\n\
            3\n\
            4\n\
            .enddata\n\
            .dc data=left\n\
            .dc data=right\n\
            .end\n";

        let decks = try_expand_multi_run(source).expect("same-kind DATA tables combine");
        assert_eq!(decks.len(), 2);
        assert_eq!(decks[0].label.as_deref(), Some("left+right row 1"));
        for (deck, expected_r1, expected_r2) in
            [(&decks[0], 1.0_f64, 3.0_f64), (&decks[1], 2.0, 4.0)]
        {
            let netlist = crate::netlist::Netlist::parse(&deck.source).expect("row parses");
            for (name, expected) in [("R1", expected_r1), ("R2", expected_r2)] {
                let actual = netlist
                    .elements
                    .iter()
                    .find(|element| element.name.eq_ignore_ascii_case(name))
                    .and_then(|element| match element.kind {
                        crate::netlist::ElementKind::Resistor { value, .. } => Some(value),
                        _ => None,
                    })
                    .expect("resistor retained");
                assert_eq!(actual.to_bits(), expected.to_bits());
            }
        }
    }

    #[test]
    fn different_data_driven_analysis_kinds_fail_closed() {
        let source = "t\n\
            .param a=1\n\
            V1 1 0 {a}\n\
            .data values a\n\
            1\n\
            .enddata\n\
            .dc data=values\n\
            .tran 1n 2n sweep data=values\n\
            .end\n";

        let error = try_expand_multi_run(source)
            .expect_err("an implicit cross-analysis expansion must not be guessed");
        assert!(error.to_string().contains("cannot be combined"));
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

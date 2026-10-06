//! Parse active source cards and enforce their lexical boundaries.
use super::*;

pub(super) fn parse(
    lines: &[&str],
    body_start: usize,
    options: &NetlistParseOptions,
    mut source_schedule: Option<SourceEventSchedule>,
    state: &mut ParseState,
    abort: &dyn AbortSignal,
) -> Result<Option<NetlistSourceLocation>, ParseWithAbortError> {
    let xyce_syntax = options.expression_dialect == ExpressionDialect::Xyce;
    let allow_non_semicolon_comments = !xyce_syntax;
    let mut line_num = body_start;
    let mut continuation = String::new();
    let mut continuation_line = None;
    let mut continuation_origin = None;
    let mut data_table: Option<DataTableBuilder> = None;
    let mut active_sources = Vec::new();
    let mut deferred_source_boundaries = Vec::new();
    let mut termination = None;
    let mut root_eof = None;
    // The line a Verilog-A source directive was taken from, while it is still
    // the line immediately before. A directive is consumed whole and leaves no
    // card open, so a `+` line here continues nothing.
    let mut veriloga_directive_line: Option<usize> = None;

    process_source_events_at(
        source_schedule.as_mut(),
        0,
        &mut active_sources,
        &mut deferred_source_boundaries,
        &mut continuation,
        &mut continuation_line,
        &mut continuation_origin,
        state,
        abort,
    )?;

    for (zero_based_line, line) in lines.iter().enumerate().skip(body_start) {
        process_source_events_at(
            source_schedule.as_mut(),
            zero_based_line,
            &mut active_sources,
            &mut deferred_source_boundaries,
            &mut continuation,
            &mut continuation_line,
            &mut continuation_origin,
            state,
            abort,
        )?;
        poll_parse_abort(abort, zero_based_line)?;
        poll_parse_text(abort, line)?;
        line_num += 1;
        let origin = source_schedule
            .as_ref()
            .and_then(|schedule| schedule.origin(zero_based_line))
            .cloned()
            .unwrap_or_else(|| NetlistSourceLocation::in_memory(line_num));

        if xyce_syntax && xyce_physical_line_is_comment(line) {
            continue;
        }

        // Strip inline comments (common SPICE syntax), then trim.
        // We intentionally keep this simple and treat these markers as comment
        // starts only when they appear outside quoted strings.
        let no_inline_comment = strip_inline_semicolon_comment_with_non_semicolon_comments(
            line,
            allow_non_semicolon_comments,
        );
        let trimmed = no_inline_comment.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        let ordinary_continuation = data_table.is_none() && trimmed.starts_with('+');
        if !ordinary_continuation {
            flush_pending_logical_line(
                &mut continuation,
                &mut continuation_line,
                &mut continuation_origin,
                state,
                active_sources.len() > 1,
                abort,
            )?;
            apply_deferred_source_boundaries(
                &mut deferred_source_boundaries,
                false,
                &mut active_sources,
                state,
            )?;
        }

        let head = trimmed.split_whitespace().next().unwrap_or("");
        if let Some(table) = data_table.as_mut() {
            if head.eq_ignore_ascii_case(".enddata") {
                let table = data_table
                    .take()
                    .expect(".DATA builder exists while inside data block")
                    .finish(line_num, abort)?;
                state.data_tables.push(table);
            } else if head.eq_ignore_ascii_case(".data") {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".DATA cannot be nested inside another .DATA block".to_string(),
                }
                .into());
            } else if is_dot_command_head(head) {
                return Err(ParseError::Syntax {
                    line: table.opened_at_line,
                    message: ".DATA without a matching .ENDDATA".to_string(),
                }
                .into());
            } else {
                table.push_line(line_num, trimmed, &state.params, abort)?;
            }
            continue;
        }

        // Handle line continuation (+ at start of line)
        if let Some(rest) = trimmed.strip_prefix('+') {
            // A Verilog-A source directive is taken whole and closes no card,
            // so the continuation machinery has nothing to attach this to: it
            // would open a fresh logical line whose first character is the
            // continuation's own text, and the deck would silently gain a card
            // the author never wrote.
            if let Some(directive_line) = veriloga_directive_line {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "continuation line has nothing to continue: the Verilog-A source \
                         directive on line {directive_line} takes no '+' continuation"
                    ),
                }
                .into());
            }
            continuation_line.get_or_insert(line_num);
            continuation_origin.get_or_insert_with(|| origin.clone());
            continuation.push(' ');
            continuation.push_str(rest);
            continue;
        }
        veriloga_directive_line = None;

        // Check for .END
        if trimmed.eq_ignore_ascii_case(".end") {
            termination = Some((MissingSubcircuitEndsBoundary::EndCard, origin));
            break;
        }

        // `.ALTER` ends the base deck; the variants expand textually
        // before parsing (multi-run), so this parse stops here.
        if head.eq_ignore_ascii_case(".alter") {
            log::info!(
                "line {line_num}: .ALTER present; this parse covers the base deck - \
                 run multi-run expansion for the alter variants"
            );
            termination = Some((MissingSubcircuitEndsBoundary::AlterCard, origin));
            break;
        }
        if head.eq_ignore_ascii_case(".data") {
            data_table = Some(DataTableBuilder::new(
                line_num,
                trimmed,
                options.resource_limits,
                abort,
            )?);
            continue;
        }
        if head.eq_ignore_ascii_case(".enddata") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".ENDDATA without matching .DATA".to_string(),
            }
            .into());
        }

        // Handle a Verilog-A source directive directly, in any of its
        // spellings, before continuation handling.
        if is_veriloga_source_command(head) {
            // The directive is taken here rather than through `process_line`,
            // so the conditional stack has to be honoured explicitly. A card
            // inside a false `.IF` branch does not exist, and a source
            // directive is no different: collecting it would bind masters the
            // deck deliberately switched off, and an instance beside it would
            // resolve against a module the author excluded.
            if state.conditionals_suppress() {
                veriloga_directive_line = Some(line_num);
                continue;
            }
            let mut include = parse_veriloga_directive(trimmed).ok_or_else(|| ParseError::Syntax {
                line: line_num,
                message: "Invalid Verilog-A include; expected .VERILOGA filename [MODELNAME] [module=MODULE] with closed quotes and no extra fields".to_owned(),
            })?;
            // A Verilog-A source names a global instance master, the way
            // Spectre's `ahdl_include` does, so writing one inside a
            // `.SUBCKT` body does not scope it to that subcircuit. Keep the
            // global scope — a deck that relies on it still works — and say
            // once, naming the subcircuit, that the scope is not what the
            // placement suggests.
            let enclosing_subcircuit = state
                .subckt_stack
                .last()
                .map(|frame| frame.qualified_name.clone());
            if let Some(subcircuit) = enclosing_subcircuit {
                state.diagnostics.push(ParseDiagnostic::warning_at(
                    origin.clone(),
                    "veriloga-directive-hoisted",
                    format!(
                        "Verilog-A source directive inside subcircuit '{subcircuit}' at line \
                         {line_num} is hoisted to the top level; the master it defines is \
                         visible to the whole deck"
                    ),
                ));
            }
            // The file that wrote the directive owns the directory a relative
            // source path resolves against, exactly as it does for `.include`.
            include.origin = origin.clone();
            log::debug!("Found .VERILOGA include: {:?}", include.file_path);
            state.push_veriloga_include(include);
            veriloga_directive_line = Some(line_num);
            continue; // Skip normal processing
        }

        // Start new continuation or process line
        continuation = trimmed.to_string();
        continuation_line = Some(line_num);
        continuation_origin = Some(origin);
    }

    if termination.is_none() {
        process_source_events_at(
            source_schedule.as_mut(),
            lines.len(),
            &mut active_sources,
            &mut deferred_source_boundaries,
            &mut continuation,
            &mut continuation_line,
            &mut continuation_origin,
            state,
            abort,
        )?;
    }

    if let Some(table) = data_table {
        return Err(ParseError::Syntax {
            line: table.opened_at_line,
            message: ".DATA without a matching .ENDDATA".to_string(),
        }
        .into());
    }

    // Process final line
    if !continuation.is_empty() {
        flush_pending_logical_line(
            &mut continuation,
            &mut continuation_line,
            &mut continuation_origin,
            state,
            active_sources.len() > 1,
            abort,
        )?;
    }
    if termination.is_none() {
        root_eof = apply_deferred_source_boundaries(
            &mut deferred_source_boundaries,
            true,
            &mut active_sources,
            state,
        )?;
    }

    if let Some(frame) = state.conditional_stack.last() {
        return Err(ParseError::Syntax {
            line: frame.opened_at_line,
            message: ".if without a matching .endif".to_string(),
        }
        .into());
    }

    if let Some((boundary, detected_at)) = termination {
        if let Some(error) = state.missing_subcircuit_ends(detected_at, boundary) {
            return Err(error.into());
        }
    } else {
        let detected_at = root_eof
            .clone()
            .unwrap_or_else(|| NetlistSourceLocation::in_memory(lines.len() + 1));
        if let Some(error) = state.missing_subcircuit_ends(
            detected_at.clone(),
            MissingSubcircuitEndsBoundary::EndOfSource,
        ) {
            return Err(error.into());
        }
    }

    Ok(root_eof)
}

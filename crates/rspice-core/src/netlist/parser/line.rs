//! Top-level line normalization and dispatch.

use super::*;

pub(super) fn strip_inline_semicolon_comment(line: &str) -> &str {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;

    for (idx, ch) in line.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }

        match ch {
            '\\' if in_single_quote || in_double_quote => {
                escaped = true;
            }
            '\'' if !in_double_quote => {
                in_single_quote = !in_single_quote;
            }
            '"' if !in_single_quote => {
                in_double_quote = !in_double_quote;
            }
            ';' | '$' if !in_single_quote && !in_double_quote => {
                return &line[..idx];
            }
            _ => {}
        }
    }
    line
}

pub(super) fn parse_veriloga_directive(line: &str) -> Option<VerilogAInclude> {
    let mut parts = line.trim().splitn(2, char::is_whitespace);
    let command = parts.next()?;
    if !command.eq_ignore_ascii_case(".veriloga") && !command.eq_ignore_ascii_case(".va") {
        return None;
    }

    let remainder = parts.next()?.trim();
    if remainder.is_empty() {
        return None;
    }

    let (raw_path, rest) = consume_quoted_or_token(remainder)?;
    let path = raw_path.trim();
    if path.is_empty() {
        return None;
    }

    let model_name = rest
        .split_whitespace()
        .next()
        .map(|s| s.trim_matches(|c| c == '"' || c == '\'').to_string())
        .filter(|s| !s.is_empty());

    Some(VerilogAInclude {
        file_path: std::path::PathBuf::from(path),
        model_name,
    })
}

pub(super) fn consume_quoted_or_token(input: &str) -> Option<(String, &str)> {
    let trimmed = input.trim_start();
    let first = trimmed.chars().next()?;

    if first == '"' || first == '\'' {
        let quote = first;
        let mut escaped = false;
        let mut value = String::new();
        for (idx, ch) in trimmed.char_indices().skip(1) {
            if escaped {
                value.push(ch);
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == quote {
                let rest = trimmed[idx + ch.len_utf8()..].trim_start();
                return Some((value, rest));
            }
            value.push(ch);
        }
        // Unclosed quote: consume remaining text as path body.
        return Some((value, ""));
    }

    let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
    let token = trimmed[..end].to_string();
    let rest = trimmed[end..].trim_start();
    Some((token, rest))
}

pub(super) fn process_line(
    line: &str,
    line_num: usize,
    state: &mut ParseState,
) -> Result<(), ParseError> {
    // HSPICE table-driven sweeps resolve through multi-run expansion
    // (`netlist::multi_run`); in a direct parse the analysis is skipped
    // with a warning instead of failing, so unexpanded decks still load.
    if crate::netlist::multi_run::references_data_table(line) {
        log::warn!(
            "line {line_num}: analysis references a .DATA table; table-driven sweeps \
             run via multi-run expansion - analysis skipped in this parse"
        );
        return Ok(());
    }

    let head = line.split_whitespace().next().unwrap_or("");
    let upper = line.to_uppercase();

    // Check for .SUBCKT start
    if head.eq_ignore_ascii_case(".subckt") {
        let subckt = parse_subckt_def(line, line_num, state.condition_scope())?;
        let parent_scope = state
            .subckt_stack
            .last()
            .map(|frame| frame.qualified_name.as_str());
        let qualified_name = qualify_nested_subckt_name(parent_scope, &subckt.name);
        let mut local_params = state
            .subckt_stack
            .last()
            .map(|frame| frame.local_params.clone())
            .unwrap_or_else(|| state.params.clone());
        for (name, value) in &subckt.params {
            local_params.set(name, *value);
        }
        for (name, value) in &subckt.string_params {
            local_params.set_string(name, value.clone());
        }
        state.subckt_stack.push(SubcktFrame {
            def: subckt,
            qualified_name,
            local_params,
            nested_aliases: HashMap::new(),
            local_model_aliases: HashMap::new(),
        });
        return Ok(());
    }

    // Check for .ENDS
    if head.eq_ignore_ascii_case(".ends") {
        let mut fields = line.split_whitespace();
        fields.next();
        let end_name = fields.next();
        if let Some(extra) = fields.next() {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(".ENDS has unexpected trailing token `{extra}`"),
            });
        }

        let Some(open_frame) = state.subckt_stack.last() else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".ENDS without matching .SUBCKT".to_string(),
            });
        };

        if let Some(end_name) = end_name
            && !end_name.eq_ignore_ascii_case(&open_frame.def.name)
            && !end_name.eq_ignore_ascii_case(&open_frame.qualified_name)
        {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".ENDS `{end_name}` does not match open .SUBCKT `{}`",
                    open_frame.def.name
                ),
            });
        }

        let mut frame = state
            .subckt_stack
            .pop()
            .expect("open .SUBCKT frame checked before pop");
        let mut visible_model_aliases = HashMap::new();
        for ancestor in &state.subckt_stack {
            for (alias, qualified) in &ancestor.local_model_aliases {
                visible_model_aliases.insert(alias.clone(), qualified.clone());
            }
        }
        for (alias, qualified) in &frame.local_model_aliases {
            visible_model_aliases.insert(alias.clone(), qualified.clone());
        }

        rewrite_scoped_references(
            &mut frame.def.elements,
            &frame.nested_aliases,
            &visible_model_aliases,
        );

        let original_name = frame.def.name.to_ascii_uppercase();
        frame.def.name = frame.qualified_name.clone();
        let finalized = frame.def;

        if let Some(parent) = state.subckt_stack.last_mut() {
            parent
                .nested_aliases
                .insert(original_name, finalized.name.clone());
            parent.def.nested_subcircuits.push(finalized.clone());
        }

        state.subcircuits.push(finalized);
        return Ok(());
    }

    // If inside subcircuit, add elements to subcircuit
    if state.subckt_stack.last().is_some() {
        if upper.starts_with(".MODEL") {
            let models = &mut state.models;
            let frame = state
                .subckt_stack
                .last_mut()
                .expect("subcircuit presence already checked");
            let tokens = tokenize(line).map_err(|e| lex_to_parse_error(e, line_num))?;
            let mut stream = TokenStream::new(tokens);
            stream.advance(); // skip .MODEL
            let mut model =
                parse_model_definition(&mut stream, line_num, &frame.local_params, models, true)?;
            let local_name = model.name.clone();
            let qualified_name = qualify_local_model_name(&frame.qualified_name, &local_name);
            frame
                .local_model_aliases
                .insert(local_name.to_ascii_uppercase(), qualified_name.clone());
            model.name = qualified_name;
            models.push(model);
            return Ok(());
        }

        {
            let analyses = &mut state.analyses;
            let unknown_warned = &mut state.unknown_warned;
            let models = &mut state.models;
            let initial_conditions = &mut state.initial_conditions;
            let node_sets = &mut state.node_sets;
            let global_nodes = &mut state.global_nodes;
            let saves = &mut state.saves;
            let options = &mut state.options;
            let diagnostics = &mut state.diagnostics;
            let spef_includes = &mut state.spef_includes;
            let frame = state
                .subckt_stack
                .last_mut()
                .expect("subcircuit presence already checked");
            let mut subckt_elements = Vec::new();
            // Subcircuits don't get standalone measurements parsing
            let mut dummy_measurements = Vec::new();
            parse_line(
                line,
                line_num,
                &mut subckt_elements,
                true,
                &mut frame.local_params,
                &mut dummy_measurements,
                ParseLineContext {
                    analyses,
                    unknown_warned,
                    models,
                    initial_conditions,
                    node_sets,
                    global_nodes,
                    saves,
                    options,
                    diagnostics,
                    spef_includes,
                },
            )?;
            frame.def.elements.extend(subckt_elements);
        }
        return Ok(());
    }

    // Normal element/command parsing
    parse_line(
        line,
        line_num,
        &mut state.elements,
        false,
        &mut state.params,
        &mut state.measurements,
        ParseLineContext {
            analyses: &mut state.analyses,
            unknown_warned: &mut state.unknown_warned,
            models: &mut state.models,
            initial_conditions: &mut state.initial_conditions,
            node_sets: &mut state.node_sets,
            global_nodes: &mut state.global_nodes,
            saves: &mut state.saves,
            options: &mut state.options,
            diagnostics: &mut state.diagnostics,
            spef_includes: &mut state.spef_includes,
        },
    )
}

pub(super) fn parse_line(
    line: &str,
    line_num: usize,
    elements: &mut Vec<Element>,
    defer_simple_param_refs: bool,
    params: &mut ParamContext,
    measurements: &mut Vec<MeasureStatement>,
    context: ParseLineContext<'_>,
) -> Result<(), ParseError> {
    let ParseLineContext {
        analyses,
        unknown_warned,
        models,
        initial_conditions,
        node_sets,
        global_nodes,
        saves,
        options,
        diagnostics,
        spef_includes,
    } = context;

    // Tokenize the line
    let tokens = tokenize(line).map_err(|e| lex_to_parse_error(e, line_num))?;
    let mut stream = TokenStream::new(tokens);

    // Skip leading whitespace/newlines
    stream.skip_newlines();

    if stream.is_eof() {
        return Ok(());
    }

    let first = match &stream.peek().kind {
        TokenKind::Ident(s) => s.clone(),
        _ => {
            return Err(ParseError::Syntax {
                line: line_num,
                message: "Expected identifier at start of line".to_string(),
            });
        }
    };

    let first_char = first.chars().next().unwrap_or(' ');

    match first_char {
        '.' => parse_command(
            &mut stream,
            line_num,
            ParseCommandContext {
                analyses,
                unknown_warned,
                models,
                params,
                initial_conditions,
                node_sets,
                global_nodes,
                measurements,
                saves,
                options,
                diagnostics,
                spef_includes,
            },
        ),
        'R' => parse_resistor(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'C' => parse_capacitor(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'L' => parse_inductor(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'V' => parse_voltage_source(&mut stream, line_num, elements, params),
        'I' => parse_current_source(&mut stream, line_num, elements, params),
        'D' => parse_diode(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'Q' => parse_bjt(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'M' => parse_mosfet(
            &mut stream,
            line_num,
            elements,
            params,
            diagnostics,
            defer_simple_param_refs,
        ),
        'J' => parse_jfet(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'X' => parse_subcircuit_instance(line, line_num, elements, params),
        'E' => parse_vcvs(&mut stream, line_num, elements, params),
        'F' => parse_cccs(&mut stream, line_num, elements, params),
        'G' => parse_vccs(&mut stream, line_num, elements, params),
        'H' => parse_ccvs(&mut stream, line_num, elements, params),
        'B' => parse_behavioral(&mut stream, line_num, elements, params),
        // Coupling and switches
        'K' => parse_coupling(&mut stream, line_num, elements, params),
        'S' => parse_vswitch(&mut stream, line_num, elements),
        'W' => parse_iswitch(&mut stream, line_num, elements),
        // Transmission lines
        'T' => parse_transmission_line(&mut stream, line_num, elements, params),
        'O' => parse_lossless_tline(&mut stream, line_num, elements, params),
        'Y' => parse_lossy_tline(&mut stream, line_num, elements, params),
        'P' => parse_coupled_tlines(&mut stream, line_num, elements),
        // MESFET (Z element) - treat like JFET with model
        'Z' => parse_mesfet(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        // XSPICE code model instance
        'A' => {
            let name = expect_ident(&mut stream, line_num)?;
            xspice_parser::parse_xspice(
                &mut stream,
                line_num,
                name,
                elements,
                params,
                defer_simple_param_refs,
            )
        }
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Unknown element type: {}", first_char),
        }),
    }
}

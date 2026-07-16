//! Top-level line normalization and dispatch.

use super::*;

pub(super) fn strip_inline_semicolon_comment(line: &str) -> &str {
    let mut in_single_quote = false;
    let mut in_double_quote = false;
    let mut escaped = false;
    let mut chars = line.char_indices().peekable();
    let mut prev_char = None;

    while let Some((idx, ch)) = chars.next() {
        if escaped {
            escaped = false;
            prev_char = Some(ch);
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
            ';' if !in_single_quote && !in_double_quote => {
                return &line[..idx];
            }
            '$' if !in_single_quote && !in_double_quote => {
                if chars.peek().is_none_or(|(_, next)| next.is_whitespace()) {
                    return &line[..idx];
                }
            }
            '/' if !in_single_quote && !in_double_quote => {
                if matches!(chars.peek(), Some((_, '/')))
                    && prev_char.map_or(true, |prev: char| prev.is_whitespace())
                {
                    return &line[..idx];
                }
            }
            _ => {}
        }
        prev_char = Some(ch);
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

fn expect_xspice_instance_name(
    stream: &mut TokenStream,
    line_num: usize,
) -> Result<String, ParseError> {
    skip_commas(stream);

    let first = stream.peek().clone();
    let TokenKind::Ident(head) = first.kind else {
        return Err(ParseError::Syntax {
            line: line_num,
            message: format!("Expected identifier, found {:?}", first.kind),
        });
    };

    let mut name = head;
    let mut end = first.span.end;
    stream.advance();

    while stream.peek().span.start == end {
        let token = stream.peek().clone();
        let fragment = match &token.kind {
            TokenKind::Ident(s) => Some(s.clone()),
            TokenKind::Number(_) => Some(token.lexeme.to_ascii_uppercase()),
            TokenKind::Plus => Some("+".to_string()),
            TokenKind::Minus => Some("-".to_string()),
            TokenKind::Star => Some("*".to_string()),
            TokenKind::Slash => Some("/".to_string()),
            TokenKind::AtSign => Some("@".to_string()),
            TokenKind::Tilde => Some("~".to_string()),
            TokenKind::Other(c) => Some(c.to_string()),
            _ => None,
        };
        let Some(fragment) = fragment else {
            break;
        };
        name.push_str(&fragment);
        end = token.span.end;
        stream.advance();
    }

    Ok(name)
}

pub(super) fn process_line(
    line: &str,
    line_num: usize,
    origin: &NetlistSourceLocation,
    state: &mut ParseState,
) -> Result<(), ParseError> {
    let authored_element_name = line
        .split_whitespace()
        .next()
        .map(|name| name.trim_end_matches(','))
        .filter(|name| !name.starts_with('.'));

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
        local_params.begin_child_definition_scope();
        for (name, value) in &subckt.params {
            local_params.set(name, *value);
        }
        for (name, value) in &subckt.string_params {
            local_params.set_string(name, value.clone());
        }
        let mut formal_names = HashSet::new();
        for name in subckt
            .params
            .iter()
            .map(|(name, _)| name)
            .chain(subckt.expr_params.iter().map(|(name, _)| name))
            .chain(subckt.string_params.iter().map(|(name, _)| name))
        {
            if formal_names.insert(name.to_ascii_uppercase()) {
                debug_assert!(local_params.accepts_parameter_definition(name, false));
            }
        }
        state.subckt_stack.push(SubcktFrame {
            def: subckt,
            qualified_name,
            opened_at: origin.clone(),
            local_params,
            nested_aliases: HashMap::new(),
            local_model_aliases: HashMap::new(),
            element_names: ElementNameRegistry::default(),
        });
        return Ok(());
    }

    // Check for .ENDS
    if head.eq_ignore_ascii_case(".ends") {
        let mut fields = line.split_whitespace();
        fields.next();
        let end_name = fields.collect::<Vec<_>>();

        let Some(open_frame) = state.subckt_stack.last() else {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".ENDS without matching .SUBCKT".to_string(),
            });
        };

        if !end_name.is_empty() {
            let end_name = end_name.join("");
            if !end_name.eq_ignore_ascii_case(&open_frame.def.name)
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
            let fft_analyses = &mut state.fft_analyses;
            let unknown_warned = &mut state.unknown_warned;
            let models = &mut state.models;
            let device_initial_conditions = &mut state.device_initial_conditions;
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
            let mut subckt_initial_conditions = Vec::new();
            let mut subckt_node_sets = Vec::new();
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
                    fft_analyses,
                    unknown_warned,
                    models,
                    initial_conditions: &mut subckt_initial_conditions,
                    device_initial_conditions,
                    node_sets: &mut subckt_node_sets,
                    global_nodes,
                    saves,
                    options,
                    diagnostics,
                    spef_includes,
                    origin,
                    deferred_body_params: Some(&mut frame.def.body_expr_params),
                },
            )?;
            let scope = format!("SUBCIRCUIT:{}", frame.qualified_name.to_ascii_uppercase());
            frame.element_names.register(
                &subckt_elements,
                authored_element_name,
                &scope,
                line_num,
            )?;
            capture_subckt_body_scope(line, &mut frame.def, &frame.local_params);
            frame.def.elements.extend(subckt_elements);
            frame
                .def
                .initial_conditions
                .extend(subckt_initial_conditions);
            frame.def.node_sets.extend(subckt_node_sets);
        }
        return Ok(());
    }

    // Normal element/command parsing
    let first_new_element = state.elements.len();
    parse_line(
        line,
        line_num,
        &mut state.elements,
        false,
        &mut state.params,
        &mut state.measurements,
        ParseLineContext {
            analyses: &mut state.analyses,
            fft_analyses: &mut state.fft_analyses,
            unknown_warned: &mut state.unknown_warned,
            models: &mut state.models,
            initial_conditions: &mut state.initial_conditions,
            device_initial_conditions: &mut state.device_initial_conditions,
            node_sets: &mut state.node_sets,
            global_nodes: &mut state.global_nodes,
            saves: &mut state.saves,
            options: &mut state.options,
            diagnostics: &mut state.diagnostics,
            spef_includes: &mut state.spef_includes,
            origin,
            deferred_body_params: None,
        },
    )?;
    state.element_names.register(
        &state.elements[first_new_element..],
        authored_element_name,
        "TOP_LEVEL",
        line_num,
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
        fft_analyses,
        unknown_warned,
        models,
        initial_conditions,
        device_initial_conditions,
        node_sets,
        global_nodes,
        saves,
        options,
        diagnostics,
        spef_includes,
        origin,
        deferred_body_params,
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
                fft_analyses,
                unknown_warned,
                models,
                params,
                initial_conditions,
                device_initial_conditions,
                node_sets,
                global_nodes,
                measurements,
                saves,
                options,
                diagnostics,
                spef_includes,
                origin,
                defer_scoped_values: defer_simple_param_refs,
                deferred_body_params,
            },
        ),
        'R' => parse_resistor(
            &mut stream,
            line_num,
            elements,
            params,
            diagnostics,
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
        'V' => parse_voltage_source(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'I' => parse_current_source(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
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
        'E' => parse_vcvs(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'F' => parse_cccs(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'G' => parse_vccs(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'H' => parse_ccvs(
            &mut stream,
            line_num,
            elements,
            params,
            defer_simple_param_refs,
        ),
        'B' => parse_behavioral(&mut stream, line_num, elements, params),
        // Coupling and switches
        'K' => parse_coupling(&mut stream, line_num, elements, params),
        'S' => parse_vswitch(&mut stream, line_num, elements),
        'W' => parse_iswitch(&mut stream, line_num, elements),
        // Transmission lines
        'T' => parse_transmission_line(&mut stream, line_num, elements, params),
        'O' => parse_lossless_tline(&mut stream, line_num, elements, params),
        'Y' => parse_lossy_tline(&mut stream, line_num, elements, params),
        'P' => parse_coupled_tlines(&mut stream, line_num, elements, params),
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
            let name = expect_xspice_instance_name(&mut stream, line_num)?;
            xspice_parser::parse_xspice(
                &mut stream,
                line_num,
                name,
                elements,
                params,
                defer_simple_param_refs,
            )
        }
        'U' => parse_pspice_u_device(line, line_num, elements, params),
        _ => Err(ParseError::Syntax {
            line: line_num,
            message: format!("Unknown element type: {}", first_char),
        }),
    }
}

fn capture_subckt_body_scope(line: &str, def: &mut SubcircuitDef, params: &ParamContext) {
    let fields = split_spice_fields(line);
    let Some(command) = fields.first() else {
        return;
    };

    if command.eq_ignore_ascii_case(".PARAM")
        || command.eq_ignore_ascii_case(".CSPARAM")
        || command.eq_ignore_ascii_case(".GLOBAL_PARAM")
    {
        for name in subckt_body_param_names(&fields) {
            if params.parameter_redefinition_policy() == ParameterRedefinitionPolicy::UseFirst
                && subckt_formal_param_contains(def, &name)
            {
                continue;
            }
            if def
                .body_expr_params
                .iter()
                .any(|(existing, _)| existing.eq_ignore_ascii_case(&name))
            {
                def.body_params
                    .retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
                def.body_string_params
                    .retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
            } else if let Some(value) = params.get(&name) {
                upsert_case_insensitive(&mut def.body_params, name.clone(), value);
                def.body_string_params
                    .retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
            } else if let Some(value) = params.get_string(&name) {
                upsert_case_insensitive(
                    &mut def.body_string_params,
                    name.clone(),
                    value.to_string(),
                );
                def.body_params
                    .retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
            }
        }
        return;
    }

    if command.eq_ignore_ascii_case(".FUNC")
        && let Some(name) = fields.get(1).map(|field| function_name_from_field(field))
        && let Some(function) = params.get_function(name).cloned()
    {
        def.body_functions
            .retain(|existing| !existing.name.eq_ignore_ascii_case(name));
        def.body_functions.push(function);
    }
}

fn subckt_formal_param_contains(def: &SubcircuitDef, name: &str) -> bool {
    def.params
        .iter()
        .any(|(existing, _)| existing.eq_ignore_ascii_case(name))
        || def
            .expr_params
            .iter()
            .any(|(existing, _)| existing.eq_ignore_ascii_case(name))
        || def
            .string_params
            .iter()
            .any(|(existing, _)| existing.eq_ignore_ascii_case(name))
}

fn function_name_from_field(field: &str) -> &str {
    field.split_once('(').map_or(field, |(name, _)| name)
}

fn subckt_body_param_names(fields: &[String]) -> Vec<String> {
    let mut names = Vec::new();
    let mut idx = 1usize;
    while idx < fields.len() {
        let field = &fields[idx];
        if field.eq_ignore_ascii_case("PARAMS") || field.eq_ignore_ascii_case("PARAMS:") {
            idx += 1;
            continue;
        }

        if let Some((name, _)) = field.split_once('=') {
            if !name.is_empty() {
                names.push(name.to_string());
            }
            idx += 1;
            continue;
        }

        if matches!(fields.get(idx + 1).map(String::as_str), Some("=")) {
            names.push(field.clone());
            idx += 3;
            continue;
        }

        idx += 1;
    }
    names
}

fn upsert_case_insensitive<T>(items: &mut Vec<(String, T)>, name: String, value: T) {
    if let Some((_, existing_value)) = items
        .iter_mut()
        .find(|(existing, _)| existing.eq_ignore_ascii_case(&name))
    {
        *existing_value = value;
    } else {
        items.push((name, value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slash_comments_do_not_strip_urls() {
        assert_eq!(
            strip_inline_semicolon_comment(".ends // SUBCKT sar_adc").trim_end(),
            ".ends"
        );
        assert_eq!(
            strip_inline_semicolon_comment("A1 m file=https://example.test/model // comment")
                .trim_end(),
            "A1 m file=https://example.test/model"
        );
        assert_eq!(
            strip_inline_semicolon_comment(
                "A1 m file=\"https://example.test/model // not a comment\" // comment",
            )
            .trim_end(),
            "A1 m file=\"https://example.test/model // not a comment\""
        );
    }

    #[test]
    fn dollar_global_nodes_do_not_start_inline_comments() {
        assert_eq!(
            strip_inline_semicolon_comment("U1 NAND(2) $G_DPWR $G_DGND a b y").trim_end(),
            "U1 NAND(2) $G_DPWR $G_DGND a b y"
        );
        assert_eq!(
            strip_inline_semicolon_comment("R1 a b 1k $ comment").trim_end(),
            "R1 a b 1k"
        );
    }

    #[test]
    fn process_line_registers_authored_and_synthesized_rf_port_names() {
        let mut state = ParseState::new();
        process_line(
            "P1 OUT 0 DC 2 PORT=1 Z0=75",
            2,
            &NetlistSourceLocation::in_memory(2),
            &mut state,
        )
        .expect("real Xyce RF port parses");

        assert_eq!(state.elements.len(), 2);
        assert!(
            state.element_names.contains_canonical("P1"),
            "authored RF-port source name is registered"
        );
        assert!(
            state.element_names.contains_canonical("__RSPICE_P1_Z0"),
            "synthesized RF-port termination name is registered from the same append batch"
        );
    }
}

//! LTspice special-function `A` devices.
//!
//! LTspice writes every special function with eight terminals and a model
//! token naming one of its built-in devices:
//!
//! ```text
//! Annn n001 n002 n003 n004 n005 n006 n007 n008 OTA G=1m Iout=10u linear
//! ```
//!
//! An `A` line is one of these exactly when its first nine fields after the
//! name are eight plain node names followed by a special-function name, and
//! no `.model` card seen so far carries that name. LTspice reads only those
//! names in that position and never binds them to a `.model` card, while an
//! XSPICE line names its own `.model` card and marks vector and typed ports
//! with brackets or `%`, so every XSPICE deck keeps its XSPICE reading.

use super::*;
use crate::netlist::{SpecialFunction, XspicePort};

/// Characters that never occur in a plain node name and always mark an
/// XSPICE port, parameter or expression field.
const NON_NODE_CHARACTERS: &[char] = &['[', ']', '%', '=', '~', '(', ')', '{', '}', '\'', '"'];

/// The special function an `A` line names, when it has the LTspice shape.
pub(super) fn special_function_of_line(line: &str, models: &[ModelDef]) -> Option<SpecialFunction> {
    let mut fields = line.split_whitespace();
    let _name = fields.next()?;
    for _ in 0..8 {
        let node = fields.next()?;
        if node.contains(NON_NODE_CHARACTERS) {
            return None;
        }
    }
    let token = fields.next()?;
    let function = SpecialFunction::from_model_token(token)?;
    let shadowed = models
        .iter()
        .any(|model| model.name.eq_ignore_ascii_case(token));
    (!shadowed).then_some(function)
}

/// Parse an LTspice special-function line whose shape
/// [`special_function_of_line`] accepted. The stream starts at the name.
pub(super) fn parse_special_function(
    stream: &mut TokenStream,
    line_num: usize,
    function: SpecialFunction,
    elements: &mut Vec<Element>,
    params: &ParamContext,
    defer_simple_param_refs: bool,
) -> Result<(), ParseError> {
    let name = expect_element_name(stream, line_num)?;
    let mut nodes = Vec::with_capacity(8);
    for _ in 0..8 {
        nodes.push(expect_node(stream, line_num)?);
    }
    // The model token itself, already identified from the raw fields.
    expect_ident(stream, line_num)?;

    let mut instance_params: Vec<(String, Value)> = Vec::new();
    let mut deferred_params: Vec<(String, String)> = Vec::new();
    let mut assign = |name: String, value: Option<DeferrableValue>| {
        instance_params.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
        deferred_params.retain(|(existing, _)| !existing.eq_ignore_ascii_case(&name));
        match value {
            Some(DeferrableValue::Resolved(value)) => instance_params.push((name, value)),
            Some(DeferrableValue::Deferred(expression)) => deferred_params.push((name, expression)),
            None => instance_params.push((name, 1.0)),
        }
    };
    loop {
        skip_commas(stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }
        let raw_name = expect_ident(stream, line_num)?;
        let param_name = raw_name.to_ascii_uppercase();
        if !stream.consume(&TokenKind::Equals) {
            assign(param_name, None);
            continue;
        }
        match take_value(stream, params, defer_simple_param_refs) {
            Some(value) => assign(param_name, Some(value)),
            None => {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "Expected value for {} parameter '{}' of {}",
                        function.ltspice_name(),
                        raw_name,
                        name
                    ),
                });
            }
        }
    }

    elements.push(Element {
        name,
        kind: ElementKind::SpecialFunction {
            function,
            instance_params,
            deferred_params,
        },
        nodes,
        provenance: crate::netlist::ElementProvenance::Authored,
    });
    Ok(())
}

/// One parameter value. LTspice reads a value up to the blank that separates
/// it from the next assignment or flag; a blank inside parentheses or beside
/// an operator continues it, so `en=1n+ 2n/ (1+freq) Vlow=0` assigns one
/// expression to `en`.
fn take_value(
    stream: &mut TokenStream,
    params: &ParamContext,
    defer: bool,
) -> Option<DeferrableValue> {
    let operator = |kind: &TokenKind| {
        matches!(
            kind,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash
        ) || matches!(kind, TokenKind::Other(c) if "^<>!&|".contains(*c))
    };
    let mut text = String::new();
    let mut previous_end = None;
    let mut after_operator = false;
    let mut depth = 0usize;
    loop {
        let token = stream.peek();
        if let Some(end) = previous_end
            && token.span.start != end
        {
            if depth == 0 && !after_operator && !operator(&token.kind) {
                break;
            }
            text.push(' ');
        }
        match &token.kind {
            TokenKind::LParen => depth += 1,
            TokenKind::RParen if depth > 0 => depth -= 1,
            TokenKind::Comma | TokenKind::Equals if depth > 0 => {}
            TokenKind::Ident(_)
            | TokenKind::Number(_)
            | TokenKind::Expression(_)
            | TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Other(_) => {}
            _ => break,
        }
        after_operator = operator(&token.kind);
        previous_end = Some(token.span.end);
        text.push_str(&crate::netlist::lexer::contiguous_expression_piece(token));
        stream.advance();
    }
    if text.is_empty() || depth != 0 {
        return None;
    }
    if let Ok(value) = crate::netlist::lexer::parse_spice_value_complete(&text) {
        return Some(DeferrableValue::Resolved(value));
    }
    if defer {
        return Some(DeferrableValue::Deferred(text));
    }
    Some(
        match stream.evaluate_value_capturing_direction(&text, params, None) {
            Ok(value) => DeferrableValue::Resolved(value),
            Err(_) => DeferrableValue::Deferred(text),
        },
    )
}

/// Give back its XSPICE reading to every special-function element whose
/// model token names a `.model` card declared after it. LTspice never binds
/// these names to a card, so such a card makes the line an XSPICE instance.
pub(super) fn restore_xspice_readings(
    elements: &mut [Element],
    subcircuits: &mut [SubcircuitDef],
    models: &[ModelDef],
) {
    for element in elements.iter_mut() {
        let ElementKind::SpecialFunction {
            function,
            instance_params,
            deferred_params,
        } = &mut element.kind
        else {
            continue;
        };
        let Some(model) = models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(function.ltspice_name()))
        else {
            continue;
        };
        element.kind = ElementKind::Xspice {
            model: model.name.clone(),
            pspice_u_timing: None,
            ports: std::mem::take(&mut element.nodes)
                .into_iter()
                .map(XspicePort::Analog)
                .collect(),
            params: std::mem::take(instance_params),
            expr_params: std::mem::take(deferred_params),
            string_params: Vec::new(),
            string_expr_params: Vec::new(),
            string_vector_params: Vec::new(),
            string_vector_expr_params: Vec::new(),
            real_vector_params: Vec::new(),
            real_vector_expr_params: Vec::new(),
        };
    }
    for subcircuit in subcircuits {
        restore_xspice_readings(
            &mut subcircuit.elements,
            &mut subcircuit.nested_subcircuits,
            models,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn model(name: &str) -> ModelDef {
        ModelDef {
            name: name.to_string(),
            model_type: "d_and".to_string(),
            params: Vec::new(),
            expr_params: Vec::new(),
            string_params: Vec::new(),
            string_vector_params: Vec::new(),
            real_vector_params: Vec::new(),
            real_vector_expr_params: Vec::new(),
            integer_vector_params: Vec::new(),
        }
    }

    #[test]
    fn eight_plain_nodes_and_a_function_name_route_to_the_native_device() {
        assert_eq!(
            special_function_of_line("A1 0 in 0 0 0 0 out 0 ota g=1m linear", &[]),
            Some(SpecialFunction::Ota)
        );
        assert_eq!(
            special_function_of_line("a2 in 0 0 0 0 qb q 0 Schmitt vt=1 vh=0.1", &[]),
            Some(SpecialFunction::Schmitt)
        );
    }

    #[test]
    fn a_value_runs_across_blanks_inside_parentheses_and_beside_operators() {
        let netlist = crate::netlist::Netlist::parse(
            "* ota\nA1 0 in 0 0 0 0 out 0 OTA g=2m en=1n+ 2n*min(freq, 1k)/ (1k+ freq) \
             linear Vlow=-1\n.end\n",
        )
        .unwrap();
        let ElementKind::SpecialFunction {
            instance_params,
            deferred_params,
            ..
        } = &netlist.elements[0].kind
        else {
            panic!("not a special function: {:?}", netlist.elements[0].kind);
        };
        assert_eq!(
            deferred_params,
            &[(
                "EN".to_string(),
                "1n+ 2n*min(freq, 1k)/ (1k+ freq)".to_string()
            )]
        );
        for expected in [("G", 2e-3), ("LINEAR", 1.0), ("VLOW", -1.0)] {
            assert!(
                instance_params.contains(&(expected.0.to_string(), expected.1)),
                "{expected:?} in {instance_params:?}"
            );
        }
    }

    #[test]
    fn xspice_shapes_keep_their_xspice_reading() {
        // Bracketed and typed ports.
        assert_eq!(
            special_function_of_line("A1 [a b c d e f g] h and", &[]),
            None
        );
        assert_eq!(
            special_function_of_line("A1 %vd(a b) c d e f g h i buf", &[]),
            None
        );
        // Too few terminals: the model token sits in the eighth position.
        assert_eq!(special_function_of_line("A1 a b c d e f g and", &[]), None);
        // A symbol name is not a model token.
        assert_eq!(
            special_function_of_line("A1 a 0 0 0 0 b c 0 INV", &[]),
            None
        );
        // A `.model` card of that name makes it an XSPICE model reference.
        assert_eq!(
            special_function_of_line("A1 a b c d e f g h and", &[model("AND")]),
            None
        );
    }
}

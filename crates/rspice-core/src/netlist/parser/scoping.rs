//! Subcircuit and model-name scoping helpers.

use super::*;

pub(super) fn qualify_nested_subckt_name(parent_scope: Option<&str>, local_name: &str) -> String {
    match parent_scope {
        Some(scope) if !scope.is_empty() => format!("{scope}.{local_name}"),
        _ => local_name.to_string(),
    }
}

pub(super) fn qualify_local_model_name(scope: &str, local_name: &str) -> String {
    format!("{scope}::{local_name}")
}

pub(super) fn parse_model_definition(
    stream: &mut TokenStream,
    line_num: usize,
    params: &ParamContext,
    known_models: &[ModelDef],
    defer_expression_params: bool,
) -> Result<ModelDef, ParseError> {
    let name = expect_ident(stream, line_num)?;
    let second = expect_ident(stream, line_num)?;

    // PSpice `AKO:` inheritance: `.model X AKO:BASE [type] (overrides)`.
    // The base must be defined earlier (PSpice's own in-order rule); the
    // derived model starts from the base's type and parameters and the
    // overrides replace by name. `AKO: BASE` (split tokens) also parses.
    let (ako_base, model_type) = if second.to_ascii_uppercase().starts_with("AKO:") {
        let base = if second.len() > 4 {
            second[4..].to_string()
        } else {
            expect_ident(stream, line_num)?
        };
        // An explicit type may follow; otherwise it inherits.
        let explicit_type = match &stream.peek().kind {
            TokenKind::Ident(t) => {
                let t = t.clone();
                stream.advance();
                Some(t)
            }
            _ => None,
        };
        (Some(base), explicit_type)
    } else {
        (None, Some(second))
    };

    if ako_base.is_none()
        && matches!(model_type.as_deref(), Some(kind) if kind.eq_ignore_ascii_case("CPL"))
    {
        // CPL model cards carry RLGC matrix/list payloads that are validated
        // from raw source text by the transmission-line model resolver.
        stream.skip_to_eol();
        return Ok(ModelDef {
            name,
            model_type: model_type.expect("CPL model carries an explicit type"),
            params: Vec::new(),
            expr_params: Vec::new(),
            string_params: Vec::new(),
            string_vector_params: Vec::new(),
            real_vector_params: Vec::new(),
            integer_vector_params: Vec::new(),
        });
    }

    let ako_base_model = if let Some(base_name) = ako_base.as_ref() {
        Some(
            known_models
                .iter()
                .rev()
                .find(|m| m.name.eq_ignore_ascii_case(base_name))
                .ok_or_else(|| ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "AKO base model `{base_name}` is not defined before `{name}` \
                         (PSpice requires the base earlier in the deck)"
                    ),
                })?,
        )
    } else {
        None
    };
    let model_type_hint = model_type
        .as_deref()
        .or_else(|| ako_base_model.map(|base| base.model_type.as_str()));
    let model_params = parse_model_params(
        stream,
        line_num,
        params,
        defer_expression_params,
        model_type_hint,
    )?;

    let Some(base_name) = ako_base else {
        return Ok(ModelDef {
            name,
            model_type: model_type.expect("non-AKO models carry an explicit type"),
            params: model_params.numeric,
            expr_params: model_params.expr,
            string_params: model_params.string,
            string_vector_params: model_params.string_vector,
            real_vector_params: model_params.real_vector,
            integer_vector_params: model_params.integer_vector,
        });
    };

    let base = ako_base_model.expect("AKO base model was resolved before parsing params");

    let model_type = match model_type {
        Some(explicit) => {
            if !explicit.eq_ignore_ascii_case(&base.model_type) {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "AKO model `{name}` declares type `{explicit}` but base \
                         `{base_name}` is `{}`",
                        base.model_type
                    ),
                });
            }
            explicit
        }
        None => base.model_type.clone(),
    };

    // Start from the base's parameters; overrides replace by name.
    let mut numeric = base.params.clone();
    let mut expr = base.expr_params.clone();
    let mut string = base.string_params.clone();
    let mut string_vector = base.string_vector_params.clone();
    let mut real_vector = base.real_vector_params.clone();
    let mut integer_vector = base.integer_vector_params.clone();
    for (key, value) in model_params.numeric {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        numeric.push((key, value));
    }
    for (key, value) in model_params.expr {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.push((key, value));
    }
    for (key, value) in model_params.string {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.push((key, value));
    }
    for (key, value) in model_params.string_vector {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.push((key, value));
    }
    for (key, value) in model_params.real_vector {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.push((key, value));
    }
    for (key, value) in model_params.integer_vector {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.push((key, value));
    }

    Ok(ModelDef {
        name,
        model_type,
        params: numeric,
        expr_params: expr,
        string_params: string,
        string_vector_params: string_vector,
        real_vector_params: real_vector,
        integer_vector_params: integer_vector,
    })
}

pub(super) fn rewrite_scoped_references(
    elements: &mut [Element],
    nested_aliases: &HashMap<String, String>,
    visible_model_aliases: &HashMap<String, String>,
) {
    for element in elements {
        if let ElementKind::Subcircuit { subckt_name, .. } = &mut element.kind
            && let Some(qualified) = nested_aliases.get(&subckt_name.to_ascii_uppercase())
        {
            *subckt_name = qualified.clone();
        }

        let model_ref = match &mut element.kind {
            ElementKind::Resistor { model, .. }
            | ElementKind::Capacitor { model, .. }
            | ElementKind::Inductor { model, .. } => model.as_mut(),
            ElementKind::JilesAthertonInductor { model, .. } => Some(model),
            ElementKind::Diode { model, .. }
            | ElementKind::Bjt { model, .. }
            | ElementKind::Mosfet { model, .. }
            | ElementKind::Jfet { model, .. }
            | ElementKind::Mesfet { model, .. }
            | ElementKind::VSwitch { model, .. }
            | ElementKind::ISwitch { model, .. }
            | ElementKind::Xspice { model, .. } => Some(model),
            ElementKind::TransmissionLine { model, .. } => model.as_mut(),
            _ => None,
        };

        if let Some(model_name) = model_ref
            && let Some(qualified) = visible_model_aliases.get(&model_name.to_ascii_uppercase())
        {
            *model_name = qualified.clone();
        }
    }
}

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
) -> Result<ModelDef, ParseError> {
    let name = expect_ident(stream, line_num)?;
    let model_type = expect_ident(stream, line_num)?;
    let model_params = parse_model_params(stream, params)?;

    Ok(ModelDef {
        name,
        model_type,
        params: model_params.numeric,
        expr_params: model_params.expr,
        string_params: model_params.string,
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
            ElementKind::Resistor { model, .. } => model.as_mut(),
            ElementKind::JilesAthertonInductor { model, .. } => Some(model),
            ElementKind::Diode { model }
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

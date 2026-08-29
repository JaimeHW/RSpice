//! Subcircuit and model-name scoping helpers.

use super::*;

#[derive(Debug)]
pub(super) struct PendingXyceDiodeModelWarning {
    pub(super) model_index: usize,
    /// Authored model spelling used by Xyce's warning payload. Local model
    /// qualification is an internal lookup detail and must not leak here.
    pub(super) model_name: String,
    pub(super) origin: NetlistSourceLocation,
    /// Effective parameter order after AKO inheritance/overrides. ModelDef's
    /// typed stores group by representation and therefore cannot recover this
    /// user-visible order later.
    pub(super) parameter_order: Vec<String>,
}

pub(super) fn qualify_nested_subckt_name(parent_scope: Option<&str>, local_name: &str) -> String {
    match parent_scope {
        Some(scope) if !scope.is_empty() => format!("{scope}.{local_name}"),
        _ => local_name.to_string(),
    }
}

pub(super) fn qualify_local_model_name(scope: &str, local_name: &str) -> String {
    format!("{scope}::{local_name}")
}

/// Parse a `.model` card.
///
/// `bare_ident_deferrals` collects parameters whose value was an unresolvable
/// bare identifier, with the line that wrote them. Those are forward
/// references until the deck ends and typos afterwards, and only end-of-parse
/// validation can tell which — see
/// `resolve_static_model_expression_params_with_abort`.
pub(super) fn parse_model_definition(
    stream: &mut TokenStream,
    line_num: usize,
    origin: &NetlistSourceLocation,
    params: &ParamContext,
    known_models: &[ModelDef],
    defer_expression_params: bool,
    bare_ident_deferrals: &mut Vec<(String, String, usize)>,
    pending_xyce_diode_model_warnings: &mut Vec<PendingXyceDiodeModelWarning>,
) -> Result<ModelDef, ParseError> {
    let model_index = known_models.len();
    let name = expect_model_name(stream, line_num)?;
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
            real_vector_expr_params: Vec::new(),
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
    let mut model_params = parse_model_params(
        stream,
        line_num,
        params,
        defer_expression_params,
        model_type_hint,
        &name,
        origin,
    )?;
    let authored_parameter_order = model_params.authored_parameter_order.clone();
    bare_ident_deferrals.append(&mut model_params.bare_ident_deferrals);

    let Some(base_name) = ako_base else {
        let model = ModelDef {
            name,
            model_type: model_type.expect("non-AKO models carry an explicit type"),
            params: model_params.numeric,
            expr_params: model_params.expr,
            string_params: model_params.string,
            string_vector_params: model_params.string_vector,
            real_vector_params: model_params.real_vector,
            real_vector_expr_params: model_params.real_vector_expr,
            integer_vector_params: model_params.integer_vector,
        };
        queue_unknown_xyce_diode_model_parameter_warnings(
            params,
            &model,
            model_index,
            authored_parameter_order,
            origin,
            pending_xyce_diode_model_warnings,
        );
        return Ok(model);
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
    let mut real_vector_expr = base.real_vector_expr_params.clone();
    let mut integer_vector = base.integer_vector_params.clone();
    for (key, value) in model_params.numeric {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        numeric.push((key, value));
    }
    for (key, value) in model_params.expr {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.push((key, value));
    }
    for (key, value) in model_params.string {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.push((key, value));
    }
    for (key, value) in model_params.string_vector {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.push((key, value));
    }
    for (key, value) in model_params.real_vector {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.push((key, value));
    }
    for (key, value) in model_params.real_vector_expr {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.push((key, value));
    }
    for (key, value) in model_params.integer_vector {
        numeric.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        string_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        real_vector_expr.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.retain(|(k, _)| !k.eq_ignore_ascii_case(&key));
        integer_vector.push((key, value));
    }

    let mut parameter_order = ako_base_model
        .and_then(|base| {
            known_models
                .iter()
                .position(|known| std::ptr::eq(known, base))
        })
        .and_then(|base_index| {
            pending_xyce_diode_model_warnings
                .iter()
                .find(|warning| warning.model_index == base_index)
        })
        .map(|warning| warning.parameter_order.clone())
        .unwrap_or_else(|| model_parameter_names_in_storage_order(base));
    for name in &authored_parameter_order {
        parameter_order.retain(|inherited| !inherited.eq_ignore_ascii_case(name));
        parameter_order.push(name.clone());
    }

    let model = ModelDef {
        name,
        model_type,
        params: numeric,
        expr_params: expr,
        string_params: string,
        string_vector_params: string_vector,
        real_vector_params: real_vector,
        real_vector_expr_params: real_vector_expr,
        integer_vector_params: integer_vector,
    };
    queue_unknown_xyce_diode_model_parameter_warnings(
        params,
        &model,
        model_index,
        parameter_order,
        origin,
        pending_xyce_diode_model_warnings,
    );
    Ok(model)
}

fn model_parameter_names_in_storage_order(model: &ModelDef) -> Vec<String> {
    model
        .params
        .iter()
        .map(|(name, _)| name)
        .chain(model.expr_params.iter().map(|(name, _)| name))
        .chain(model.string_params.iter().map(|(name, _)| name))
        .chain(model.string_vector_params.iter().map(|(name, _)| name))
        .chain(model.real_vector_params.iter().map(|(name, _)| name))
        .chain(model.real_vector_expr_params.iter().map(|(name, _)| name))
        .chain(model.integer_vector_params.iter().map(|(name, _)| name))
        .map(|name| name.to_ascii_uppercase())
        .collect()
}

fn queue_unknown_xyce_diode_model_parameter_warnings(
    params: &ParamContext,
    model: &ModelDef,
    model_index: usize,
    parameter_order: Vec<String>,
    origin: &NetlistSourceLocation,
    pending: &mut Vec<PendingXyceDiodeModelWarning>,
) {
    if params.expression_dialect() != ExpressionDialect::Xyce
        || !(model.model_type.eq_ignore_ascii_case("D")
            || model.model_type.eq_ignore_ascii_case("DIODE"))
    {
        return;
    }

    // Delay every Xyce diode namespace decision until global expressions have
    // resolved. This keeps diagnostics in authored model-card order and lets a
    // subcircuit-local LEVEL be decided against each concrete caller scope.
    pending.push(PendingXyceDiodeModelWarning {
        model_index,
        model_name: model.name.clone(),
        origin: origin.clone(),
        parameter_order,
    });
}

fn uses_generated_xyce_diode_model(model: &ModelDef) -> bool {
    model
        .params
        .iter()
        .rev()
        .find(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
        .is_some_and(|(_, level)| {
            let rounded = level.round();
            level.is_finite()
                && (*level - rounded).abs() <= 1e-9
                && matches!(rounded as i32, 200 | 2002)
        })
}

fn push_unknown_xyce_diode_model_parameter_warnings(
    model: &ModelDef,
    model_name: &str,
    parameter_order: &[String],
    origin: &NetlistSourceLocation,
    diagnostics: &mut Vec<ParseDiagnostic>,
) {
    if uses_generated_xyce_diode_model(model) {
        return;
    }

    let mut seen = HashSet::new();
    for name in parameter_order {
        if !model_has_parameter(model, name) {
            continue;
        }
        if crate::device::Diode::supports_xyce_legacy_model_parameter(name) {
            continue;
        }
        let canonical_name = name.to_ascii_uppercase();
        if !seen.insert(canonical_name.clone()) {
            continue;
        }
        let message = format!(
            "No model parameter {canonical_name} found for model {model_name} of type D, parameter ignored."
        );
        diagnostics.push(ParseDiagnostic::warning_at(
            origin.clone(),
            "xyce-unknown-diode-model-parameter",
            message,
        ));
    }
}

fn model_has_parameter(model: &ModelDef, expected: &str) -> bool {
    model
        .params
        .iter()
        .map(|(name, _)| name)
        .chain(model.expr_params.iter().map(|(name, _)| name))
        .chain(model.string_params.iter().map(|(name, _)| name))
        .chain(model.string_vector_params.iter().map(|(name, _)| name))
        .chain(model.real_vector_params.iter().map(|(name, _)| name))
        .chain(model.real_vector_expr_params.iter().map(|(name, _)| name))
        .chain(model.integer_vector_params.iter().map(|(name, _)| name))
        .any(|name| name.eq_ignore_ascii_case(expected))
}

pub(super) fn emit_pending_xyce_diode_model_parameter_warnings_with_abort(
    netlist: &mut Netlist,
    pending: Vec<PendingXyceDiodeModelWarning>,
    resource_limits: crate::resource::ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let needs_scoped_resolution = pending.iter().any(|warning| {
        netlist
            .models
            .get(warning.model_index)
            .is_some_and(|model| {
                model
                    .expr_params
                    .iter()
                    .any(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
            })
    });
    let mut native_scoped_models = HashSet::new();
    if needs_scoped_resolution {
        let config = crate::netlist::flattener::FlattenerConfig {
            max_depth: resource_limits.max_hierarchy_depth,
            max_elements: resource_limits.max_flattened_elements,
            ..crate::netlist::flattener::FlattenerConfig::default()
        };
        let mut flattener = crate::netlist::flattener::Flattener::with_models_config(
            &netlist.subcircuits,
            &netlist.models,
            config,
        );
        match flattener.collect_scoped_models_for_diagnostics_with_abort(netlist, abort) {
            Ok(()) => {
                for (scoped_model, source_model_index) in flattener.scoped_models_with_sources() {
                    if scoped_model
                        .expr_params
                        .iter()
                        .any(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
                        || uses_generated_xyce_diode_model(scoped_model)
                    {
                        continue;
                    }
                    native_scoped_models.insert(source_model_index);
                }
            }
            Err(ParseWithAbortError::Aborted) => return Err(ParseWithAbortError::Aborted),
            Err(error @ ParseWithAbortError::Parse(ParseError::ResourceLimit(_))) => {
                return Err(error);
            }
            Err(ParseWithAbortError::Parse(error)) => {
                // The diagnostic collector isolates ordinary branch failures.
                // Keep this defensive fallback so future setup-only errors do
                // not move circuit-construction validation into parsing.
                log::debug!(
                    "could not elaborate deferred Xyce diode warnings during parsing: {error}"
                );
            }
        }
    }

    let (models, diagnostics) = (&netlist.models, &mut netlist.diagnostics);
    for (index, warning) in pending.into_iter().enumerate() {
        poll_parse_abort(abort, index)?;
        let Some(model) = models.get(warning.model_index) else {
            continue;
        };
        let level_is_deferred = model
            .expr_params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("LEVEL"));
        if level_is_deferred && !native_scoped_models.contains(&warning.model_index) {
            continue;
        }
        push_unknown_xyce_diode_model_parameter_warnings(
            model,
            &warning.model_name,
            &warning.parameter_order,
            &warning.origin,
            diagnostics,
        );
    }
    ensure_parse_not_aborted(abort)
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
            | ElementKind::XyceMemristor { model, .. }
            | ElementKind::VSwitch { model, .. }
            | ElementKind::ISwitch { model, .. }
            | ElementKind::Xspice { model, .. }
            | ElementKind::Coupling {
                model: Some(model), ..
            } => Some(model),
            ElementKind::TransmissionLine { model, .. } => model.as_mut(),
            _ => None,
        };

        if let Some(model_name) = model_ref
            && let Some(qualified) = visible_model_aliases.get(&model_name.to_ascii_uppercase())
        {
            *model_name = qualified.clone();
        }

        if let ElementKind::Xspice {
            pspice_u_timing: Some(timing),
            ..
        } = &mut element.kind
            && let Some(qualified) =
                visible_model_aliases.get(&timing.timing_model.to_ascii_uppercase())
        {
            timing.timing_model = qualified.clone();
        }
    }
}

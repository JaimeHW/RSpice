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

/// The device families model cards serve, each named by its first type.
const MODEL_FAMILIES: &[&[&str]] = &[
    &["D"],
    &["NPN", "PNP", "LPNP"],
    &["NMOS", "PMOS", "VDMOS"],
    &["NJF", "PJF"],
    &["NMF", "PMF", "NHFET", "PHFET"],
    &["SW", "VSWITCH", "VSW"],
    &["CSW", "ISWITCH", "ISW"],
];

/// The family a model type serves, by the family's first type.
pub(super) fn model_family(model_type: &str) -> Option<&'static str> {
    MODEL_FAMILIES
        .iter()
        .find(|family| {
            family
                .iter()
                .any(|kind| kind.eq_ignore_ascii_case(model_type))
        })
        .map(|family| family[0])
}

/// The alias key of a local model name, upper case, for one family.
pub(super) fn typed_model_key(local_key: &str, family: &str) -> String {
    format!("{local_key}@{family}")
}

/// The model family an element names its model from, when cards of several
/// families may share a name.
fn element_model_family(kind: &ElementKind) -> Option<&'static str> {
    match kind {
        ElementKind::Diode { .. } => Some("D"),
        ElementKind::Bjt { .. } => Some("NPN"),
        ElementKind::Mosfet { .. } => Some("NMOS"),
        ElementKind::Jfet { .. } => Some("NJF"),
        ElementKind::Mesfet { .. } => Some("NMF"),
        ElementKind::VSwitch { .. } => Some("SW"),
        ElementKind::ISwitch { .. } => Some("CSW"),
        _ => None,
    }
}

/// One unresolved bare reference and the exact model card that owns it.
#[derive(Debug)]
pub(super) struct PendingModelParameterReference {
    pub(super) model_index: usize,
    pub(super) parameter: String,
    pub(super) reference: String,
    pub(super) line: usize,
}

/// Parse a `.model` card.
///
/// `bare_ident_deferrals` collects parameters whose value was an unresolvable
/// bare identifier, with the line that wrote them. Those are forward
/// references until the deck ends and typos afterwards, and only end-of-parse
/// validation can tell which — see
/// `resolve_static_model_expression_params_with_abort`.
/// The deferral sinks a `.MODEL` line writes into: bare identifiers whose
/// value is not known until the parameter scope closes, the Xyce diode
/// warnings that can only be emitted once the card is complete, and `AKO:`
/// cards whose base is not yet visible.
pub(super) struct ModelDefinitionDeferrals<'a> {
    pub bare_ident_deferrals: &'a mut Vec<PendingModelParameterReference>,
    pub pending_xyce_diode_model_warnings: &'a mut Vec<PendingXyceDiodeModelWarning>,
    pub pending_ako_models: &'a mut Vec<PendingAkoModel>,
    /// Qualified names of the subcircuits enclosing the card, innermost
    /// first; empty at the top level.
    pub enclosing_scopes: &'a [String],
}

/// An `AKO:` model card whose base is bound once the whole deck has been
/// read: LTspice and QSPICE find the base wherever the deck defines it,
/// before or after the derived card.
#[derive(Debug)]
pub(super) struct PendingAkoModel {
    pub(super) model_index: usize,
    pub(super) base_name: String,
    /// The type the derived card wrote, which must match its base's.
    pub(super) explicit_type: Option<String>,
    pub(super) enclosing_scopes: Vec<String>,
    pub(super) authored_parameter_order: Vec<String>,
    pub(super) origin: NetlistSourceLocation,
}

/// The model a scoped `AKO:` base name names. A subcircuit's own models
/// shadow its parents', which shadow the top level's. While the deck is
/// still being read, `local_only` refuses a top-level match from inside a
/// subcircuit, because a local model of that name may still follow.
fn find_ako_base(
    models: &[ModelDef],
    enclosing_scopes: &[String],
    base_name: &str,
    local_only: bool,
) -> Option<usize> {
    let find = |name: &str| {
        models
            .iter()
            .rposition(|model| model.name.eq_ignore_ascii_case(name))
    };
    enclosing_scopes
        .iter()
        .find_map(|scope| find(&qualify_local_model_name(scope, base_name)))
        .or_else(|| {
            if enclosing_scopes.is_empty() || !local_only {
                find(base_name)
            } else {
                None
            }
        })
}

/// Give an `AKO:` model its base's type and parameters; the parameters it
/// writes itself replace the base's of the same name.
fn inherit_ako_base(
    model: &mut ModelDef,
    explicit_type: Option<&str>,
    base: &ModelDef,
    base_name: &str,
    line: usize,
) -> Result<(), ParseError> {
    fn overlay<T: Clone>(
        inherited: &[(String, T)],
        own: &mut Vec<(String, T)>,
        written: &HashSet<String>,
    ) {
        let mut merged: Vec<(String, T)> = inherited
            .iter()
            .filter(|(name, _)| !written.contains(&name.to_ascii_uppercase()))
            .cloned()
            .collect();
        merged.append(own);
        *own = merged;
    }

    if let Some(explicit) = explicit_type
        && !explicit.eq_ignore_ascii_case(&base.model_type)
    {
        return Err(ParseError::Syntax {
            line,
            message: format!(
                "AKO model `{}` declares type `{explicit}` but base `{base_name}` is `{}`",
                model.name, base.model_type
            ),
        });
    }
    let written: HashSet<String> = model_parameter_names_in_storage_order(model)
        .into_iter()
        .collect();
    overlay(&base.params, &mut model.params, &written);
    overlay(&base.expr_params, &mut model.expr_params, &written);
    overlay(&base.string_params, &mut model.string_params, &written);
    overlay(
        &base.string_vector_params,
        &mut model.string_vector_params,
        &written,
    );
    overlay(
        &base.real_vector_params,
        &mut model.real_vector_params,
        &written,
    );
    overlay(
        &base.real_vector_expr_params,
        &mut model.real_vector_expr_params,
        &written,
    );
    overlay(
        &base.integer_vector_params,
        &mut model.integer_vector_params,
        &written,
    );
    model.model_type = explicit_type.map_or_else(|| base.model_type.clone(), str::to_string);
    Ok(())
}

/// The user-visible parameter order of an `AKO:` model: its base's, then the
/// names it writes itself.
fn ako_parameter_order(
    base_index: usize,
    base: &ModelDef,
    authored_parameter_order: &[String],
    pending_xyce_diode_model_warnings: &[PendingXyceDiodeModelWarning],
) -> Vec<String> {
    let mut parameter_order = pending_xyce_diode_model_warnings
        .iter()
        .find(|warning| warning.model_index == base_index)
        .map(|warning| warning.parameter_order.clone())
        .unwrap_or_else(|| model_parameter_names_in_storage_order(base));
    for name in authored_parameter_order {
        parameter_order.retain(|inherited| !inherited.eq_ignore_ascii_case(name));
        parameter_order.push(name.clone());
    }
    parameter_order
}

/// Bind every deferred `AKO:` card to its base once the deck has been read.
/// A chain binds base first whatever order its cards were written in; a base
/// nothing defines, or a chain that returns to itself, is an error.
pub(super) fn resolve_pending_ako_models(
    params: &ParamContext,
    models: &mut [ModelDef],
    pending: Vec<PendingAkoModel>,
    pending_xyce_diode_model_warnings: &mut Vec<PendingXyceDiodeModelWarning>,
) -> Result<(), ParseError> {
    let located = |card: &PendingAkoModel, message: String| ParseError::Syntax {
        line: card.origin.line,
        message: if card.origin.path.is_some() {
            format!("{}: {message}", card.origin)
        } else {
            message
        },
    };
    let queued_warnings = pending_xyce_diode_model_warnings.len();
    let mut unresolved = pending;
    while !unresolved.is_empty() {
        let before = unresolved.len();
        let bases = unresolved
            .iter()
            .map(|card| {
                find_ako_base(models, &card.enclosing_scopes, &card.base_name, false).ok_or_else(
                    || {
                        located(
                            card,
                            format!(
                                "AKO base model `{}` of `{}` is not defined",
                                card.base_name, models[card.model_index].name
                            ),
                        )
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let unbound: HashSet<usize> = unresolved.iter().map(|card| card.model_index).collect();
        let mut waiting = Vec::new();
        for (card, base_index) in unresolved.into_iter().zip(bases) {
            if unbound.contains(&base_index) {
                waiting.push(card);
                continue;
            }
            let base = models[base_index].clone();
            let model = &mut models[card.model_index];
            inherit_ako_base(
                model,
                card.explicit_type.as_deref(),
                &base,
                &card.base_name,
                card.origin.line,
            )
            .map_err(|error| match error {
                ParseError::Syntax { message, .. } => located(&card, message),
                other => other,
            })?;
            let parameter_order = ako_parameter_order(
                base_index,
                &base,
                &card.authored_parameter_order,
                pending_xyce_diode_model_warnings,
            );
            queue_unknown_xyce_diode_model_parameter_warnings(
                params,
                model,
                card.model_index,
                parameter_order,
                &card.origin,
                pending_xyce_diode_model_warnings,
            );
        }
        if waiting.len() == before {
            let card = &waiting[0];
            return Err(located(
                card,
                format!(
                    "AKO model `{}` inherits from itself through base `{}`",
                    models[card.model_index].name, card.base_name
                ),
            ));
        }
        unresolved = waiting;
    }
    if pending_xyce_diode_model_warnings.len() > queued_warnings {
        pending_xyce_diode_model_warnings.sort_by_key(|warning| warning.model_index);
    }
    Ok(())
}

pub(super) fn parse_model_definition(
    stream: &mut TokenStream,
    line_num: usize,
    origin: &NetlistSourceLocation,
    params: &ParamContext,
    known_models: &[ModelDef],
    defer_expression_params: bool,
    deferrals: ModelDefinitionDeferrals<'_>,
) -> Result<ModelDef, ParseError> {
    let ModelDefinitionDeferrals {
        bare_ident_deferrals,
        pending_xyce_diode_model_warnings,
        pending_ako_models,
        enclosing_scopes,
    } = deferrals;
    let model_index = known_models.len();
    let name = expect_model_name(stream, line_num)?;
    let second = expect_ident(stream, line_num)?;

    // PSpice `AKO:` inheritance: `.model X AKO:BASE [type] (overrides)`.
    // The derived model starts from the base's type and parameters and the
    // overrides replace by name. `AKO: BASE` (split tokens) also parses.
    let (ako_base, model_type) = if second.to_ascii_uppercase().starts_with("AKO:") {
        let base = if second.len() > 4 {
            second[4..].to_string()
        } else {
            expect_ident(stream, line_num)?
        };
        // An explicit type may follow; otherwise it inherits. A name followed
        // by `=` is the first override, not a type.
        let explicit_type = match &stream.peek().kind {
            TokenKind::Ident(t) if !matches!(stream.peek_n(1).kind, TokenKind::Equals) => {
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

    // A base already visible from the card's scope, and itself complete,
    // binds now so the overrides parse against its type; any other binds
    // once the deck has been read.
    let visible_base = ako_base
        .as_deref()
        .and_then(|base_name| find_ako_base(known_models, enclosing_scopes, base_name, true))
        .filter(|base_index| {
            !pending_ako_models
                .iter()
                .any(|pending| pending.model_index == *base_index)
        });
    let model_type_hint = model_type.as_deref().or_else(|| {
        visible_base
            .or_else(|| {
                ako_base.as_deref().and_then(|base_name| {
                    find_ako_base(known_models, enclosing_scopes, base_name, false)
                })
            })
            .map(|index| known_models[index].model_type.as_str())
    });
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
    bare_ident_deferrals.extend(model_params.bare_ident_deferrals.drain(..).map(
        |(parameter, reference, line)| PendingModelParameterReference {
            model_index,
            parameter,
            reference,
            line,
        },
    ));

    let mut model = ModelDef {
        name,
        model_type: model_type_hint.unwrap_or_default().to_string(),
        params: model_params.numeric,
        expr_params: model_params.expr,
        string_params: model_params.string,
        string_vector_params: model_params.string_vector,
        real_vector_params: model_params.real_vector,
        real_vector_expr_params: model_params.real_vector_expr,
        integer_vector_params: model_params.integer_vector,
    };
    let parameter_order = match (ako_base, visible_base) {
        (None, _) => authored_parameter_order,
        (Some(base_name), Some(base_index)) => {
            let base = &known_models[base_index];
            inherit_ako_base(
                &mut model,
                model_type.as_deref(),
                base,
                &base_name,
                line_num,
            )?;
            ako_parameter_order(
                base_index,
                base,
                &authored_parameter_order,
                pending_xyce_diode_model_warnings,
            )
        }
        (Some(base_name), None) => {
            pending_ako_models.push(PendingAkoModel {
                model_index,
                base_name,
                explicit_type: model_type,
                enclosing_scopes: enclosing_scopes.to_vec(),
                authored_parameter_order,
                origin: origin.clone(),
            });
            return Ok(model);
        }
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
        || !(is_xyce_diode_model(model) || is_xyce_resistor_model(model))
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

fn is_xyce_diode_model(model: &ModelDef) -> bool {
    model.model_type.eq_ignore_ascii_case("D") || model.model_type.eq_ignore_ascii_case("DIODE")
}

fn is_xyce_resistor_model(model: &ModelDef) -> bool {
    model.model_type.eq_ignore_ascii_case("R") || model.model_type.eq_ignore_ascii_case("RES")
}

/// Warn for each resistance spelling Xyce's resistor model does not have.
///
/// Xyce names the model multiplier R and nothing else (N_DEV_Resistor.C:254),
/// so it ignores RES, R0, VALUE and RESISTANCE with this warning, at every
/// LEVEL; the other dialects read them as R.
fn push_unknown_xyce_resistor_model_parameter_warnings(
    model: &ModelDef,
    model_name: &str,
    parameter_order: &[String],
    origin: &NetlistSourceLocation,
    diagnostics: &mut Vec<ParseDiagnostic>,
) {
    let mut seen = HashSet::new();
    for name in parameter_order {
        let canonical_name = name.to_ascii_uppercase();
        if !matches!(
            canonical_name.as_str(),
            "RES" | "R0" | "VALUE" | "RESISTANCE"
        ) || !model_has_parameter(model, name)
            || !seen.insert(canonical_name.clone())
        {
            continue;
        }
        diagnostics.push(ParseDiagnostic::warning_at(
            origin.clone(),
            "xyce-unknown-resistor-model-parameter",
            format!(
                "No model parameter {canonical_name} found for model {model_name} of type R, parameter ignored."
            ),
        ));
    }
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

/// Return whether Xyce 7.10's legacy `D` model registry accepts a model
/// parameter name.
///
/// RSpice's native diode also supports ngspice-compatible and geometry
/// fields, while Xyce's parser diagnoses those names when they are authored
/// on a legacy diode model card. The entries below are the exact
/// `Traits::loadModelParameters` names from Xyce 7.10 `N_DEV_Diode.C`, plus
/// the framework-owned `LEVEL` selector.
fn supports_xyce_legacy_diode_model_parameter(name: &str) -> bool {
    matches!(
        name.to_ascii_uppercase().as_str(),
        "LEVEL"
            | "IS"
            | "JS"
            | "JSW"
            | "RS"
            | "N"
            | "NS"
            | "ISR"
            | "NR"
            | "IKF"
            | "TT"
            | "CJO"
            | "CJ"
            | "CJ0"
            | "VJ"
            | "M"
            | "CJSW"
            | "CJP"
            | "PHP"
            | "VJSW"
            | "MJSW"
            | "EG"
            | "XTI"
            | "TIKF"
            | "TBV1"
            | "TBV2"
            | "TRS1"
            | "TRS"
            | "TRS2"
            | "FC"
            | "FCS"
            | "BV"
            | "VB"
            | "IBV"
            | "NBV"
            | "IBVL"
            | "NBVL"
            | "TNOM"
            | "KF"
            | "AF"
    )
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
        if supports_xyce_legacy_diode_model_parameter(name) {
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
                is_xyce_diode_model(model)
                    && model
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
        if is_xyce_resistor_model(model) {
            push_unknown_xyce_resistor_model_parameter_warnings(
                model,
                &warning.model_name,
                &warning.parameter_order,
                &warning.origin,
                diagnostics,
            );
            continue;
        }
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

        let family = element_model_family(&element.kind);
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

        if let Some(model_name) = model_ref {
            let key = model_name.to_ascii_uppercase();
            if let Some(qualified) = family
                .and_then(|family| visible_model_aliases.get(&typed_model_key(&key, family)))
                .or_else(|| visible_model_aliases.get(&key))
            {
                *model_name = qualified.clone();
            }
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

/// Which namespace defines an instance master, ordered the way the engine
/// resolves them.
///
/// Hierarchy expansion takes the deck's own `.subckt` first
/// (`netlist::flattener`'s `find_subcircuit`), before an unclaimed master is
/// preserved for external binding. What survives is bound to a Verilog-A
/// artifact next — explicit alias, then `module=` selection, then file stem
/// (`engine::builder`'s `VerilogABindingPriority`) — and only a name no
/// artifact claims reaches the build-time generated catalog.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MasterNamespace {
    DeckSubcircuit,
    VerilogAAlias,
    VerilogAModuleSelection,
    VerilogAFileStem,
    GeneratedBuiltin,
}

impl MasterNamespace {
    /// How the warning names the definition the deck will actually get.
    fn winner(self) -> &'static str {
        match self {
            Self::DeckSubcircuit => "the deck's .subckt",
            Self::VerilogAAlias => "the .VERILOGA alias",
            Self::VerilogAModuleSelection => "the .VERILOGA module= selection",
            Self::VerilogAFileStem => "the .VERILOGA file stem",
            Self::GeneratedBuiltin => "the generated built-in model",
        }
    }
}

/// One definition of one instance-master name.
#[derive(Clone)]
struct MasterClaim {
    namespace: MasterNamespace,
    /// Upper-case spelling the namespaces are compared on.
    canonical: String,
    /// Spelling as authored, so the warning points at the source token.
    authored: String,
    /// The Verilog-A source this definition came from and the directive that
    /// named it, when it came from one.
    veriloga: Option<(std::path::PathBuf, NetlistSourceLocation)>,
}

/// `.subckt`, `.VERILOGA` and the build-time generated catalog are three
/// independent instance-master namespaces, and a name defined in two of them
/// resolves silently by precedence. Spectre warns when a master is defined
/// twice; so does RSpice, naming both definitions and which one the deck gets.
///
/// Every Verilog-A spelling of one include is one namespace: an alias and a
/// file stem that agree name the same artifact, which is not a shadow.
///
/// A Verilog-A definition carries the directive's own origin. A `.subckt` does
/// not, so its line comes from a scan of `source`, which runs only for a name
/// that really is defined twice.
pub(super) fn shadowed_instance_master_diagnostics(
    netlist: &Netlist,
    source: &str,
) -> Vec<ParseDiagnostic> {
    let builtins = crate::netlist::generated_builtin_masters();
    if netlist.veriloga_includes.is_empty() && builtins.is_empty() {
        return Vec::new();
    }

    // Without the feature there is no Verilog-A binding to shadow: an include
    // is carried through parsing and refused at construction.
    let mut veriloga: Vec<MasterClaim> = Vec::new();
    if cfg!(feature = "veriloga") {
        for include in &netlist.veriloga_includes {
            let path = include.file_path.as_path();
            let spellings = [
                (
                    include.model_name.as_deref(),
                    MasterNamespace::VerilogAAlias,
                ),
                (
                    include.selected_module.as_deref(),
                    MasterNamespace::VerilogAModuleSelection,
                ),
                (
                    path.file_stem().and_then(|stem| stem.to_str()),
                    MasterNamespace::VerilogAFileStem,
                ),
            ];
            for (authored, namespace) in spellings {
                let Some(authored) = authored else {
                    continue;
                };
                let claim = MasterClaim {
                    namespace,
                    canonical: authored.to_ascii_uppercase(),
                    authored: authored.to_owned(),
                    veriloga: Some((path.to_path_buf(), include.origin.clone())),
                };
                match veriloga
                    .iter()
                    .position(|existing| existing.canonical == claim.canonical)
                {
                    Some(index) if veriloga[index].namespace <= claim.namespace => {}
                    Some(index) => veriloga[index] = claim,
                    None => veriloga.push(claim),
                }
            }
        }
    }

    let builtin_claim = |canonical: &str| {
        builtins.get(canonical).map(|authored| MasterClaim {
            namespace: MasterNamespace::GeneratedBuiltin,
            canonical: canonical.to_owned(),
            authored: (*authored).to_owned(),
            veriloga: None,
        })
    };

    let mut diagnostics = Vec::new();
    let mut reported: Vec<String> = Vec::new();
    for subckt in &netlist.subcircuits {
        let canonical = subckt.name.to_ascii_uppercase();
        let mut definitions = vec![MasterClaim {
            namespace: MasterNamespace::DeckSubcircuit,
            canonical: canonical.clone(),
            authored: subckt.name.clone(),
            veriloga: None,
        }];
        definitions.extend(
            veriloga
                .iter()
                .find(|claim| claim.canonical == canonical)
                .cloned(),
        );
        definitions.extend(builtin_claim(&canonical));
        if definitions.len() < 2 || reported.contains(&canonical) {
            continue;
        }
        reported.push(canonical);
        diagnostics.push(shadowed_master_warning(&definitions, source));
    }
    for claim in &veriloga {
        if reported.contains(&claim.canonical) {
            continue;
        }
        let Some(builtin) = builtin_claim(&claim.canonical) else {
            continue;
        };
        reported.push(claim.canonical.clone());
        diagnostics.push(shadowed_master_warning(&[claim.clone(), builtin], source));
    }
    diagnostics
}

/// The one warning for one twice-defined master. `definitions` is in
/// resolution order, so its first entry is the definition that is used.
fn shadowed_master_warning(definitions: &[MasterClaim], source: &str) -> ParseDiagnostic {
    let described: Vec<(String, usize)> = definitions
        .iter()
        .map(|claim| describe_master_claim(claim, source))
        .collect();
    let spellings: Vec<&str> = described
        .iter()
        .map(|(description, _)| description.as_str())
        .collect();
    let message = format!(
        "Instance master {} is defined in more than one namespace: {}; {} is used",
        definitions[0].canonical,
        join_master_definitions(&spellings),
        definitions[0].namespace.winner()
    );
    // The message says "line N" whichever source wrote the definition, so the
    // two dialects word one clash identically; the exact physical location
    // rides on the diagnostic, where a frontend reads it, and a Verilog-A
    // directive is the only definition here that records one.
    match definitions[0].veriloga.as_ref() {
        Some((_, origin)) => {
            ParseDiagnostic::warning_at(origin.clone(), "shadowed-instance-master", message)
        }
        None => ParseDiagnostic::warning(described[0].1, "shadowed-instance-master", message),
    }
}

/// One definition as the warning spells it, with the source line it was
/// written on — `0` when nothing records one, which is what
/// [`ParseDiagnostic::line`] reserves for a diagnostic with no single line.
fn describe_master_claim(claim: &MasterClaim, source: &str) -> (String, usize) {
    let at = |line: usize| {
        if line == 0 {
            String::new()
        } else {
            format!(" at line {line}")
        }
    };
    match claim.namespace {
        MasterNamespace::DeckSubcircuit => {
            let line = deck_subcircuit_line(source, &claim.authored);
            (format!(".subckt {}{}", claim.authored, at(line)), line)
        }
        MasterNamespace::GeneratedBuiltin => (
            format!("the generated built-in model {}", claim.authored),
            0,
        ),
        namespace => match claim.veriloga.as_ref() {
            Some((path, origin)) => {
                let spelling = match namespace {
                    MasterNamespace::VerilogAAlias => format!(".VERILOGA alias {}", claim.authored),
                    MasterNamespace::VerilogAModuleSelection => {
                        format!(".VERILOGA module={}", claim.authored)
                    }
                    _ => ".VERILOGA file stem".to_owned(),
                };
                (
                    format!(
                        "the {spelling} of '{}'{}",
                        display_source_path(path),
                        at(origin.line)
                    ),
                    origin.line,
                )
            }
            // Unreachable: a Verilog-A definition is built from the directive
            // that declared it. Naming the spelling alone is still true.
            None => (namespace.winner().to_owned(), 0),
        },
    }
}

fn join_master_definitions(definitions: &[&str]) -> String {
    match definitions {
        [] => String::new(),
        [only] => (*only).to_owned(),
        [head @ .., last] => format!("{} and {last}", head.join(", ")),
    }
}

/// The line a `.subckt` of this name is declared on, or `0`.
fn deck_subcircuit_line(source: &str, name: &str) -> usize {
    source
        .lines()
        .position(|line| {
            strip_leading_directive(line.trim_start(), ".subckt").is_some_and(|rest| {
                rest.split_whitespace()
                    .next()
                    .is_some_and(|declared| declared.eq_ignore_ascii_case(name))
            })
        })
        .map_or(0, |index| index + 1)
}

/// What follows `directive` when `line` opens with it as a whole word.
fn strip_leading_directive<'a>(line: &'a str, directive: &str) -> Option<&'a str> {
    let head = line.get(..directive.len())?;
    if !head.eq_ignore_ascii_case(directive) {
        return None;
    }
    let rest = &line[directive.len()..];
    rest.starts_with(char::is_whitespace)
        .then(|| rest.trim_start())
}

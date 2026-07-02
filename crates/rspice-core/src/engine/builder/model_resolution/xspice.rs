use super::*;

pub(in crate::engine::builder) struct ResolvedXspiceModel {
    pub(in crate::engine::builder) code_model: std::sync::Arc<dyn crate::xspice::CodeModel>,
    pub(in crate::engine::builder) numeric_params: Vec<(String, f64)>,
    pub(in crate::engine::builder) string_params: Vec<(String, String)>,
    pub(in crate::engine::builder) string_vector_params: Vec<(String, Vec<String>)>,
    pub(in crate::engine::builder) real_vector_params: Vec<(String, Vec<f64>)>,
    pub(in crate::engine::builder) integer_vector_params: Vec<(String, Vec<i64>)>,
}

pub(in crate::engine::builder) enum NativeXtradevReactiveModel {
    Capacitor {
        capacitance: f64,
        initial_voltage: Option<f64>,
    },
    Inductor {
        inductance: f64,
    },
}

fn merge_numeric_params(base: &[(String, f64)], overrides: &[(String, f64)]) -> Vec<(String, f64)> {
    let mut merged = base.to_vec();

    for (name, value) in overrides {
        if let Some(existing) = merged
            .iter_mut()
            .find(|(existing_name, _)| existing_name.eq_ignore_ascii_case(name))
        {
            existing.1 = *value;
        } else {
            merged.push((name.clone(), *value));
        }
    }

    merged
}

fn merge_string_params(
    base: &[(String, String)],
    overrides: &[(String, String)],
) -> Vec<(String, String)> {
    let mut merged = base.to_vec();

    for (name, value) in overrides {
        if let Some(existing) = merged
            .iter_mut()
            .find(|(existing_name, _)| existing_name.eq_ignore_ascii_case(name))
        {
            existing.1 = value.clone();
        } else {
            merged.push((name.clone(), value.clone()));
        }
    }

    merged
}

fn merge_vector_params<T: Clone>(
    base: &[(String, Vec<T>)],
    overrides: &[(String, Vec<T>)],
) -> Vec<(String, Vec<T>)> {
    let mut merged = base.to_vec();

    for (name, values) in overrides {
        if let Some(existing) = merged
            .iter_mut()
            .find(|(existing_name, _)| existing_name.eq_ignore_ascii_case(name))
        {
            existing.1 = values.clone();
        } else {
            merged.push((name.clone(), values.clone()));
        }
    }

    merged
}

fn reject_param_names_for_vector_specs<'a>(
    code_model: &dyn crate::xspice::CodeModel,
    names: impl IntoIterator<Item = &'a str>,
    channel: &str,
) -> Result<(), SimulationError> {
    for name in names {
        if let Some(spec) = code_model
            .parameters()
            .iter()
            .find(|spec| spec.name.eq_ignore_ascii_case(name))
            && matches!(
                spec.param_type,
                crate::xspice::ParamType::StringVector
                    | crate::xspice::ParamType::RealVector
                    | crate::xspice::ParamType::IntegerVector
                    | crate::xspice::ParamType::ComplexVector
            )
        {
            return Err(SimulationError::Circuit(format!(
                "XSPICE model '{}' vector parameter '{}' was given a {} value",
                code_model.name(),
                name,
                channel
            )));
        }
    }
    Ok(())
}

fn reject_scalar_params_for_vector_specs(
    code_model: &dyn crate::xspice::CodeModel,
    params: &[(String, f64)],
) -> Result<(), SimulationError> {
    reject_param_names_for_vector_specs(
        code_model,
        params.iter().map(|(name, _)| name.as_str()),
        "scalar",
    )
}

fn resolve_scalar_expression_params(
    netlist: &Netlist,
    model_def: &crate::netlist::ModelDef,
    code_model: &dyn crate::xspice::CodeModel,
) -> Result<Vec<(String, f64)>, SimulationError> {
    let current_temp_c = netlist.options.temp.unwrap_or(27.0);
    let tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let ctx = build_model_eval_context(netlist, model_def, current_temp_c, tnom_c);
    let mut resolved = Vec::with_capacity(model_def.expr_params.len());

    for (name, expr) in &model_def.expr_params {
        if let Some(spec) = code_model
            .parameters()
            .iter()
            .find(|spec| spec.name.eq_ignore_ascii_case(name))
        {
            match spec.param_type {
                crate::xspice::ParamType::Real
                | crate::xspice::ParamType::Integer
                | crate::xspice::ParamType::Boolean => {}
                crate::xspice::ParamType::Complex => {
                    return Err(SimulationError::Circuit(format!(
                        "XSPICE model '{}' parameter '{}' expects Complex, got expression value",
                        code_model.name(),
                        name
                    )));
                }
                crate::xspice::ParamType::String => {
                    return Err(SimulationError::Circuit(format!(
                        "XSPICE model '{}' parameter '{}' expects String, got expression value",
                        code_model.name(),
                        name
                    )));
                }
                crate::xspice::ParamType::StringVector
                | crate::xspice::ParamType::RealVector
                | crate::xspice::ParamType::IntegerVector
                | crate::xspice::ParamType::ComplexVector => {
                    return Err(SimulationError::Circuit(format!(
                        "XSPICE model '{}' vector parameter '{}' was given an expression value",
                        code_model.name(),
                        name
                    )));
                }
            }
        }

        let value = crate::netlist::expr::eval_expression(expr, &ctx).map_err(|err| {
            SimulationError::Circuit(format!(
                "XSPICE model '{}' expression parameter '{}' could not be resolved: {}",
                code_model.name(),
                name,
                err
            ))
        })?;
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE model '{}' expression parameter '{}' resolved to non-finite value {}",
                code_model.name(),
                name,
                value
            )));
        }
        resolved.push((name.clone(), value));
    }

    Ok(resolved)
}

fn build_instance_eval_context(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    instance_params: &[(String, f64)],
) -> crate::netlist::ParamContext {
    let current_temp_c = netlist.options.temp.unwrap_or(27.0);
    let tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let mut ctx = match model_def {
        Some(model_def) => build_model_eval_context(netlist, model_def, current_temp_c, tnom_c),
        None => {
            let mut ctx = base_eval_context(netlist);
            ctx.set("TEMP", current_temp_c);
            ctx.set("TEMPER", current_temp_c);
            ctx.set("TNOM", tnom_c);
            ctx
        }
    };
    for (name, value) in instance_params {
        ctx.set(name, *value);
    }
    ctx
}

fn validate_xspice_expression_param_type(
    code_model: &dyn crate::xspice::CodeModel,
    name: &str,
) -> Result<(), SimulationError> {
    if let Some(spec) = code_model
        .parameters()
        .iter()
        .find(|spec| spec.name.eq_ignore_ascii_case(name))
    {
        match spec.param_type {
            crate::xspice::ParamType::Real
            | crate::xspice::ParamType::Integer
            | crate::xspice::ParamType::Boolean => {}
            crate::xspice::ParamType::Complex => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' parameter '{}' expects Complex, got expression value",
                    code_model.name(),
                    name
                )));
            }
            crate::xspice::ParamType::String => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' parameter '{}' expects String, got expression value",
                    code_model.name(),
                    name
                )));
            }
            crate::xspice::ParamType::StringVector
            | crate::xspice::ParamType::RealVector
            | crate::xspice::ParamType::IntegerVector
            | crate::xspice::ParamType::ComplexVector => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' vector parameter '{}' was given an expression value",
                    code_model.name(),
                    name
                )));
            }
        }
    }
    Ok(())
}

fn resolve_scalar_instance_expression_params(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    code_model: &dyn crate::xspice::CodeModel,
    model_name: &str,
    instance_params: &[(String, f64)],
    instance_expr_params: &[(String, String)],
) -> Result<Vec<(String, f64)>, SimulationError> {
    if instance_expr_params.is_empty() {
        return Ok(Vec::new());
    }

    for (name, _) in instance_expr_params {
        validate_xspice_expression_param_type(code_model, name)?;
    }

    let mut ctx = build_instance_eval_context(netlist, model_def, instance_params);
    let mut pending = instance_expr_params.to_vec();
    let mut resolved = Vec::with_capacity(instance_expr_params.len());

    while !pending.is_empty() {
        let mut progress = false;
        let mut unresolved = Vec::new();
        let mut first_error = None;

        for (name, expr) in pending {
            match crate::netlist::expr::eval_expression(&expr, &ctx) {
                Ok(value) if value.is_finite() => {
                    ctx.set(&name, value);
                    resolved.push((name, value));
                    progress = true;
                }
                Ok(value) => {
                    return Err(SimulationError::Circuit(format!(
                        "XSPICE model '{}' instance expression parameter '{}' resolved to non-finite value {}",
                        model_name, name, value
                    )));
                }
                Err(err) => {
                    if first_error.is_none() {
                        first_error = Some((name.clone(), err.to_string()));
                    }
                    unresolved.push((name, expr));
                }
            }
        }

        if !progress {
            let (name, err) = first_error.expect("unresolved expression has an error");
            return Err(SimulationError::Circuit(format!(
                "XSPICE model '{}' instance expression parameter '{}' could not be resolved: {}",
                model_name, name, err
            )));
        }

        pending = unresolved;
    }

    Ok(resolved)
}

fn resolve_instance_string_expression_params(
    netlist: &Netlist,
    model_name: &str,
    instance_string_expr_params: &[(String, String)],
) -> Result<Vec<(String, String)>, SimulationError> {
    let mut resolved = Vec::with_capacity(instance_string_expr_params.len());

    for (name, expr) in instance_string_expr_params {
        let value = netlist.params.get_string(expr).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "XSPICE model '{}' instance string parameter '{}' could not resolve string parameter '{}'",
                model_name, name, expr
            ))
        })?;
        resolved.push((name.clone(), value.to_string()));
    }

    Ok(resolved)
}

fn resolve_instance_real_vector_expression_params(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    model_name: &str,
    instance_params: &[(String, f64)],
    instance_vector_expr_params: &[(String, Vec<String>)],
) -> Result<Vec<(String, Vec<f64>)>, SimulationError> {
    if instance_vector_expr_params.is_empty() {
        return Ok(Vec::new());
    }

    let ctx = build_instance_eval_context(netlist, model_def, instance_params);
    let mut resolved = Vec::with_capacity(instance_vector_expr_params.len());

    for (name, exprs) in instance_vector_expr_params {
        let mut values = Vec::with_capacity(exprs.len());
        for expr in exprs {
            let value = crate::netlist::expr::eval_expression(expr, &ctx).map_err(|err| {
                SimulationError::Circuit(format!(
                    "XSPICE model '{}' instance vector parameter '{}' could not resolve expression '{}': {}",
                    model_name, name, expr, err
                ))
            })?;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' instance vector parameter '{}' expression '{}' resolved to non-finite value {}",
                    model_name, name, expr, value
                )));
            }
            values.push(value);
        }
        resolved.push((name.clone(), values));
    }

    Ok(resolved)
}

fn resolve_model_real_vector_expression_params(
    netlist: &Netlist,
    model_def: &crate::netlist::ModelDef,
    code_model: &dyn crate::xspice::CodeModel,
) -> Result<Vec<(String, Vec<f64>)>, SimulationError> {
    if model_def.real_vector_expr_params.is_empty() {
        return Ok(Vec::new());
    }

    let current_temp_c = netlist.options.temp.unwrap_or(27.0);
    let tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let ctx = build_model_eval_context(netlist, model_def, current_temp_c, tnom_c);
    let mut resolved = Vec::with_capacity(model_def.real_vector_expr_params.len());

    for (name, exprs) in &model_def.real_vector_expr_params {
        let mut values = Vec::with_capacity(exprs.len());
        for expr in exprs {
            let value = crate::netlist::expr::eval_expression(expr, &ctx).map_err(|err| {
                SimulationError::Circuit(format!(
                    "XSPICE model '{}' vector parameter '{}' could not resolve expression '{}': {}",
                    code_model.name(),
                    name,
                    expr,
                    err
                ))
            })?;
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' vector parameter '{}' expression '{}' resolved to non-finite value {}",
                    code_model.name(),
                    name,
                    expr,
                    value
                )));
            }
            values.push(value);
        }
        resolved.push((name.clone(), values));
    }

    Ok(resolved)
}

fn resolve_instance_string_vector_expression_params(
    netlist: &Netlist,
    model_name: &str,
    instance_string_vector_expr_params: &[(String, String)],
) -> Result<Vec<(String, Vec<String>)>, SimulationError> {
    let mut resolved = Vec::with_capacity(instance_string_vector_expr_params.len());

    for (name, expr) in instance_string_vector_expr_params {
        let value = netlist.params.get_string(expr).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "XSPICE model '{}' instance string-vector parameter '{}' could not resolve string parameter '{}'",
                model_name, name, expr
            ))
        })?;
        let values = crate::netlist::parse_xspice_string_vector_literal(value, 1, name).map_err(|err| {
            SimulationError::Circuit(format!(
                "XSPICE model '{}' instance string-vector parameter '{}' could not parse '{}': {}",
                model_name, name, expr, err
            ))
        })?;
        resolved.push((name.clone(), values));
    }

    Ok(resolved)
}

fn resolve_vector_params(
    code_model: &dyn crate::xspice::CodeModel,
    vectors: &[(String, Vec<f64>)],
) -> Result<(Vec<(String, Vec<f64>)>, Vec<(String, Vec<i64>)>), SimulationError> {
    let param_specs: Vec<&crate::xspice::ParamSpec> = code_model.parameters().iter().collect();
    let mut real_vectors = Vec::new();
    let mut integer_vectors = Vec::new();

    for (name, values) in vectors {
        match param_specs
            .iter()
            .copied()
            .find(|spec| spec.name.eq_ignore_ascii_case(name))
            .map(|spec| spec.param_type)
        {
            Some(crate::xspice::ParamType::IntegerVector) => {
                let mut integer_values = Vec::with_capacity(values.len());
                for value in values {
                    if !value.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "XSPICE model '{}' integer-vector parameter '{}' expected finite value, got {}",
                            code_model.name(),
                            name,
                            value
                        )));
                    }
                    integer_values.push(value.round() as i64);
                }
                integer_vectors.push((name.clone(), integer_values));
            }
            Some(crate::xspice::ParamType::RealVector) | None => {
                real_vectors.push((name.clone(), values.clone()));
            }
            Some(other) => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' parameter '{}' expects {:?}, got real-vector value",
                    code_model.name(),
                    name,
                    other
                )));
            }
        }
    }

    Ok((real_vectors, integer_vectors))
}

fn resolve_string_vector_params(
    code_model: &dyn crate::xspice::CodeModel,
    vectors: &[(String, Vec<String>)],
) -> Result<Vec<(String, Vec<String>)>, SimulationError> {
    let param_specs: Vec<&crate::xspice::ParamSpec> = code_model.parameters().iter().collect();
    let mut string_vectors = Vec::new();

    for (name, values) in vectors {
        match param_specs
            .iter()
            .copied()
            .find(|spec| spec.name.eq_ignore_ascii_case(name))
            .map(|spec| spec.param_type)
        {
            Some(crate::xspice::ParamType::StringVector)
            | Some(crate::xspice::ParamType::ComplexVector)
            | None => {
                string_vectors.push((name.clone(), values.clone()));
            }
            Some(other) => {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE model '{}' parameter '{}' expects {:?}, got string-vector value",
                    code_model.name(),
                    name,
                    other
                )));
            }
        }
    }

    Ok(string_vectors)
}

fn native_xtradev_kind(model_type: &str) -> Option<&'static str> {
    if model_type.eq_ignore_ascii_case("capacitor")
        || model_type.eq_ignore_ascii_case("capacitoric")
    {
        Some("capacitoric")
    } else if model_type.eq_ignore_ascii_case("inductor")
        || model_type.eq_ignore_ascii_case("inductoric")
    {
        Some("inductoric")
    } else {
        None
    }
}

fn native_xtradev_supported_params(kind: &str) -> &'static [&'static str] {
    match kind {
        "capacitoric" => &["C", "IC"],
        "inductoric" => &["L", "IC"],
        _ => &[],
    }
}

fn native_xtradev_param_list(kind: &str) -> &'static str {
    match kind {
        "capacitoric" => "C, IC",
        "inductoric" => "L, IC",
        _ => "",
    }
}

fn validate_native_xtradev_param_name(
    kind: &str,
    element_name: &str,
    model_name: &str,
    name: &str,
) -> Result<(), SimulationError> {
    if native_xtradev_supported_params(kind)
        .iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
    {
        Ok(())
    } else {
        Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' uses \
             unsupported parameter '{name}' (supported: {})",
            native_xtradev_param_list(kind)
        )))
    }
}

fn validate_native_xtradev_numeric_params(
    kind: &str,
    element_name: &str,
    model_name: &str,
    params: &[(String, f64)],
) -> Result<(), SimulationError> {
    for (name, value) in params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
                 '{name}' resolved to non-finite value {value}"
            )));
        }
    }
    Ok(())
}

fn reject_native_xtradev_non_scalar_params(
    kind: &str,
    element_name: &str,
    model_name: &str,
    model_def: &crate::netlist::ModelDef,
) -> Result<(), SimulationError> {
    for (name, _) in &model_def.string_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in &model_def.string_vector_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in &model_def.real_vector_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in &model_def.real_vector_expr_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in &model_def.integer_vector_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    Ok(())
}

fn reject_native_xtradev_instance_string_params(
    kind: &str,
    element_name: &str,
    model_name: &str,
    string_params: &[(String, String)],
    string_expr_params: &[(String, String)],
    string_vector_params: &[(String, Vec<String>)],
    string_vector_expr_params: &[(String, String)],
    real_vector_params: &[(String, Vec<f64>)],
    real_vector_expr_params: &[(String, Vec<String>)],
) -> Result<(), SimulationError> {
    for (name, _) in string_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in string_expr_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in string_vector_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in string_vector_expr_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in real_vector_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    for (name, _) in real_vector_expr_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        return Err(SimulationError::Circuit(format!(
            "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
             '{name}' must be a scalar numeric value"
        )));
    }
    Ok(())
}

fn resolve_native_xtradev_expr_params(
    netlist: &Netlist,
    model_def: &crate::netlist::ModelDef,
    kind: &str,
    element_name: &str,
    model_name: &str,
) -> Result<Vec<(String, f64)>, SimulationError> {
    let current_temp_c = netlist.options.temp.unwrap_or(27.0);
    let tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let ctx = build_model_eval_context(netlist, model_def, current_temp_c, tnom_c);
    let mut resolved = Vec::with_capacity(model_def.expr_params.len());

    for (name, expr) in &model_def.expr_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
        let value = crate::netlist::expr::eval_expression(expr, &ctx).map_err(|err| {
            SimulationError::Circuit(format!(
                "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
                 '{name}' could not be resolved: {err}"
            ))
        })?;
        if !value.is_finite() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
                 '{name}' resolved to non-finite value {value}"
            )));
        }
        resolved.push((name.clone(), value));
    }

    Ok(resolved)
}

fn resolve_native_xtradev_instance_expr_params(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    kind: &str,
    element_name: &str,
    model_name: &str,
    instance_params: &[(String, f64)],
    instance_expr_params: &[(String, String)],
) -> Result<Vec<(String, f64)>, SimulationError> {
    if instance_expr_params.is_empty() {
        return Ok(Vec::new());
    }

    let mut ctx = build_instance_eval_context(netlist, model_def, instance_params);
    let mut pending = instance_expr_params.to_vec();
    let mut resolved = Vec::with_capacity(instance_expr_params.len());

    while !pending.is_empty() {
        let mut progress = false;
        let mut unresolved = Vec::new();
        let mut first_error = None;

        for (name, expr) in pending {
            match crate::netlist::expr::eval_expression(&expr, &ctx) {
                Ok(value) if value.is_finite() => {
                    ctx.set(&name, value);
                    resolved.push((name, value));
                    progress = true;
                }
                Ok(value) => {
                    return Err(SimulationError::Circuit(format!(
                        "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' \
                         parameter '{name}' resolved to non-finite value {value}"
                    )));
                }
                Err(err) => {
                    if first_error.is_none() {
                        first_error = Some((name.clone(), err.to_string()));
                    }
                    unresolved.push((name, expr));
                }
            }
        }

        if !progress {
            let (name, err) = first_error.expect("unresolved expression has an error");
            return Err(SimulationError::Circuit(format!(
                "XSPICE xtradev {kind} instance '{element_name}' model '{model_name}' parameter \
                 '{name}' could not be resolved: {err}"
            )));
        }

        pending = unresolved;
    }

    Ok(resolved)
}

fn resolve_native_xtradev_params(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    kind: &str,
    element_name: &str,
    model_name: &str,
    instance_params: &[(String, f64)],
    instance_expr_params: &[(String, String)],
    instance_string_params: &[(String, String)],
    instance_string_expr_params: &[(String, String)],
    instance_string_vector_params: &[(String, Vec<String>)],
    instance_string_vector_expr_params: &[(String, String)],
    instance_real_vector_params: &[(String, Vec<f64>)],
    instance_real_vector_expr_params: &[(String, Vec<String>)],
) -> Result<Vec<(String, f64)>, SimulationError> {
    validate_native_xtradev_numeric_params(kind, element_name, model_name, instance_params)?;
    reject_native_xtradev_instance_string_params(
        kind,
        element_name,
        model_name,
        instance_string_params,
        instance_string_expr_params,
        instance_string_vector_params,
        instance_string_vector_expr_params,
        instance_real_vector_params,
        instance_real_vector_expr_params,
    )?;
    for (name, _) in instance_expr_params {
        validate_native_xtradev_param_name(kind, element_name, model_name, name)?;
    }
    let instance_expr_params = resolve_native_xtradev_instance_expr_params(
        netlist,
        model_def,
        kind,
        element_name,
        model_name,
        instance_params,
        instance_expr_params,
    )?;
    let instance_numeric_params = merge_numeric_params(instance_params, &instance_expr_params);

    let Some(model_def) = model_def else {
        return Ok(instance_numeric_params);
    };

    reject_native_xtradev_non_scalar_params(kind, element_name, model_name, model_def)?;
    validate_native_xtradev_numeric_params(kind, element_name, model_name, &model_def.params)?;
    let expr_params =
        resolve_native_xtradev_expr_params(netlist, model_def, kind, element_name, model_name)?;
    let model_numeric_params = merge_numeric_params(&model_def.params, &expr_params);
    Ok(merge_numeric_params(
        &model_numeric_params,
        &instance_numeric_params,
    ))
}

pub(in crate::engine::builder) fn resolve_native_xtradev_reactive_model(
    netlist: &Netlist,
    model_name: &str,
    element_name: &str,
    instance_params: &[(String, f64)],
    instance_expr_params: &[(String, String)],
    instance_string_params: &[(String, String)],
    instance_string_expr_params: &[(String, String)],
    instance_string_vector_params: &[(String, Vec<String>)],
    instance_string_vector_expr_params: &[(String, String)],
    instance_real_vector_params: &[(String, Vec<f64>)],
    instance_real_vector_expr_params: &[(String, Vec<String>)],
) -> Result<Option<NativeXtradevReactiveModel>, SimulationError> {
    let model_def = find_model_def(netlist, model_name);
    let model_type = model_def
        .map(|model_def| model_def.model_type.as_str())
        .unwrap_or(model_name);
    let Some(kind) = native_xtradev_kind(model_type) else {
        return Ok(None);
    };

    let params = resolve_native_xtradev_params(
        netlist,
        model_def,
        kind,
        element_name,
        model_name,
        instance_params,
        instance_expr_params,
        instance_string_params,
        instance_string_expr_params,
        instance_string_vector_params,
        instance_string_vector_expr_params,
        instance_real_vector_params,
        instance_real_vector_expr_params,
    )?;

    match kind {
        "capacitoric" => {
            let capacitance = model_param(&params, &["C"]).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "XSPICE xtradev capacitoric instance '{element_name}' model '{model_name}' \
                     requires C"
                ))
            })?;
            if !capacitance.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE xtradev capacitoric instance '{element_name}' model '{model_name}' \
                     resolved to invalid capacitance C={capacitance}"
                )));
            }
            let initial_voltage = model_param(&params, &["IC"]);
            Ok(Some(NativeXtradevReactiveModel::Capacitor {
                capacitance,
                initial_voltage,
            }))
        }
        "inductoric" => {
            let inductance = model_param(&params, &["L"]).ok_or_else(|| {
                SimulationError::Circuit(format!(
                    "XSPICE xtradev inductoric instance '{element_name}' model '{model_name}' \
                     requires L"
                ))
            })?;
            if !inductance.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "XSPICE xtradev inductoric instance '{element_name}' model '{model_name}' \
                     resolved to invalid inductance L={inductance}"
                )));
            }
            Ok(Some(NativeXtradevReactiveModel::Inductor { inductance }))
        }
        _ => Ok(None),
    }
}

pub(in crate::engine::builder) fn resolve_xspice_model_instance(
    netlist: &Netlist,
    registry: &crate::xspice::CodeModelRegistry,
    model_name: &str,
    instance_params: &[(String, f64)],
    instance_expr_params: &[(String, String)],
    instance_string_params: &[(String, String)],
    instance_string_expr_params: &[(String, String)],
    instance_string_vector_params: &[(String, Vec<String>)],
    instance_string_vector_expr_params: &[(String, String)],
    instance_real_vector_params: &[(String, Vec<f64>)],
    instance_real_vector_expr_params: &[(String, Vec<String>)],
) -> Result<ResolvedXspiceModel, SimulationError> {
    let model_def = find_model_def(netlist, model_name);

    if model_def.is_none() {
        if let Some(code_model) = registry.get(model_name) {
            reject_scalar_params_for_vector_specs(code_model.as_ref(), instance_params)?;
            reject_param_names_for_vector_specs(
                code_model.as_ref(),
                instance_string_params
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .chain(
                        instance_string_expr_params
                            .iter()
                            .map(|(name, _)| name.as_str()),
                    ),
                "string",
            )?;
            let resolved_instance_real_vector_expr_params =
                resolve_instance_real_vector_expression_params(
                    netlist,
                    None,
                    model_name,
                    instance_params,
                    instance_real_vector_expr_params,
                )?;
            let resolved_instance_string_vector_expr_params =
                resolve_instance_string_vector_expression_params(
                    netlist,
                    model_name,
                    instance_string_vector_expr_params,
                )?;
            let instance_real_vector_params = merge_vector_params(
                instance_real_vector_params,
                &resolved_instance_real_vector_expr_params,
            );
            let instance_string_vector_params = merge_vector_params(
                instance_string_vector_params,
                &resolved_instance_string_vector_expr_params,
            );
            let (real_vector_params, integer_vector_params) =
                resolve_vector_params(code_model.as_ref(), &instance_real_vector_params)?;
            let string_vector_params =
                resolve_string_vector_params(code_model.as_ref(), &instance_string_vector_params)?;
            let instance_expr_params = resolve_scalar_instance_expression_params(
                netlist,
                None,
                code_model.as_ref(),
                model_name,
                instance_params,
                instance_expr_params,
            )?;
            let numeric_params = merge_numeric_params(instance_params, &instance_expr_params);
            let resolved_instance_string_expr_params = resolve_instance_string_expression_params(
                netlist,
                model_name,
                instance_string_expr_params,
            )?;
            let string_params = merge_string_params(
                instance_string_params,
                &resolved_instance_string_expr_params,
            );
            return Ok(ResolvedXspiceModel {
                code_model,
                numeric_params,
                string_params,
                string_vector_params,
                real_vector_params,
                integer_vector_params,
            });
        }
    }

    let model_def = model_def.ok_or_else(|| {
        SimulationError::Circuit(format!("Unknown XSPICE model '{}'", model_name))
    })?;

    let code_model = registry.get(&model_def.model_type).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Unknown XSPICE model '{}' (alias '{}' resolves to unregistered code model '{}')",
            model_name, model_def.name, model_def.model_type
        ))
    })?;

    reject_scalar_params_for_vector_specs(code_model.as_ref(), &model_def.params)?;
    reject_param_names_for_vector_specs(
        code_model.as_ref(),
        model_def
            .string_params
            .iter()
            .map(|(name, _)| name.as_str()),
        "string",
    )?;
    reject_scalar_params_for_vector_specs(code_model.as_ref(), instance_params)?;
    reject_param_names_for_vector_specs(
        code_model.as_ref(),
        instance_string_params
            .iter()
            .map(|(name, _)| name.as_str())
            .chain(
                instance_string_expr_params
                    .iter()
                    .map(|(name, _)| name.as_str()),
            ),
        "string",
    )?;
    let resolved_instance_real_vector_expr_params = resolve_instance_real_vector_expression_params(
        netlist,
        Some(model_def),
        model_name,
        instance_params,
        instance_real_vector_expr_params,
    )?;
    let resolved_instance_string_vector_expr_params =
        resolve_instance_string_vector_expression_params(
            netlist,
            model_name,
            instance_string_vector_expr_params,
        )?;
    let instance_real_vector_params = merge_vector_params(
        instance_real_vector_params,
        &resolved_instance_real_vector_expr_params,
    );
    let instance_string_vector_params = merge_vector_params(
        instance_string_vector_params,
        &resolved_instance_string_vector_expr_params,
    );

    let expr_params = resolve_scalar_expression_params(netlist, model_def, code_model.as_ref())?;
    let model_real_vector_expr_params =
        resolve_model_real_vector_expression_params(netlist, model_def, code_model.as_ref())?;
    let instance_expr_params = resolve_scalar_instance_expression_params(
        netlist,
        Some(model_def),
        code_model.as_ref(),
        model_name,
        instance_params,
        instance_expr_params,
    )?;
    let resolved_instance_string_expr_params = resolve_instance_string_expression_params(
        netlist,
        model_name,
        instance_string_expr_params,
    )?;
    let string_vector_params =
        resolve_string_vector_params(code_model.as_ref(), &model_def.string_vector_params)?;
    let model_real_vector_params = merge_vector_params(
        &model_def.real_vector_params,
        &model_real_vector_expr_params,
    );
    let (real_vector_params, mut integer_vector_params) =
        resolve_vector_params(code_model.as_ref(), &model_real_vector_params)?;
    let instance_string_vector_params =
        resolve_string_vector_params(code_model.as_ref(), &instance_string_vector_params)?;
    let (instance_real_vector_params, instance_integer_vector_params) =
        resolve_vector_params(code_model.as_ref(), &instance_real_vector_params)?;
    integer_vector_params.extend(model_def.integer_vector_params.clone());
    let model_numeric_params = merge_numeric_params(&model_def.params, &expr_params);
    let instance_numeric_params = merge_numeric_params(instance_params, &instance_expr_params);
    let instance_string_params = merge_string_params(
        instance_string_params,
        &resolved_instance_string_expr_params,
    );

    Ok(ResolvedXspiceModel {
        code_model,
        numeric_params: merge_numeric_params(&model_numeric_params, &instance_numeric_params),
        string_params: merge_string_params(&model_def.string_params, &instance_string_params),
        string_vector_params: merge_vector_params(
            &string_vector_params,
            &instance_string_vector_params,
        ),
        real_vector_params: merge_vector_params(&real_vector_params, &instance_real_vector_params),
        integer_vector_params: merge_vector_params(
            &integer_vector_params,
            &instance_integer_vector_params,
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::{CmContext, CmResult, CodeModel, ParamSpec, PortSpec};
    use std::sync::Arc;

    struct ParamOnlyModel {
        name: &'static str,
        params: Vec<ParamSpec>,
        ports: Vec<PortSpec>,
    }

    impl ParamOnlyModel {
        fn new(name: &'static str, params: Vec<ParamSpec>) -> Self {
            Self {
                name,
                params,
                ports: Vec::new(),
            }
        }
    }

    impl CodeModel for ParamOnlyModel {
        fn name(&self) -> &str {
            self.name
        }

        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }

        fn parameters(&self) -> &[ParamSpec] {
            &self.params
        }

        fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }

        fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
            Ok(())
        }
    }

    #[test]
    fn xspice_resolver_prefers_model_card_alias_over_registered_code_model_name() {
        let netlist = Netlist::parse(
            "xspice alias collision\n\
             .model divider d_fdiv (div_factor=4)\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "divider",
            vec![ParamSpec::real("scale", 1.0)],
        )));
        registry.register(Arc::new(ParamOnlyModel::new(
            "d_fdiv",
            vec![ParamSpec::integer("div_factor", 2)],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "divider",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice alias resolves before direct model name");

        assert_eq!(resolved.code_model.name(), "d_fdiv");
        assert_eq!(
            resolved
                .numeric_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("div_factor"))
                .map(|(_, value)| *value),
            Some(4.0)
        );
    }

    #[test]
    fn xspice_resolver_carries_real_vector_model_params() {
        let netlist = Netlist::parse(
            "xspice vector model\n\
             .model vp vector_probe (points=[1 2.5 4])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "vp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice model resolves");

        assert_eq!(
            resolved
                .real_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("points"))
                .map(|(_, values)| values.as_slice()),
            Some(&[1.0, 2.5, 4.0][..])
        );
    }

    #[test]
    fn xspice_resolver_converts_integer_vector_model_params() {
        let netlist = Netlist::parse(
            "xspice integer vector model\n\
             .model bp bit_probe (bits=[1 0 1 1])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "bit_probe",
            vec![ParamSpec::integer_vector("bits", vec![0])],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "bp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice model resolves");

        assert_eq!(
            resolved
                .integer_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("bits"))
                .map(|(_, values)| values.as_slice()),
            Some(&[1, 0, 1, 1][..])
        );
        assert!(
            resolved
                .real_vector_params
                .iter()
                .all(|(name, _)| !name.eq_ignore_ascii_case("bits"))
        );
    }

    #[test]
    fn xspice_resolver_carries_string_vector_model_params() {
        let netlist = Netlist::parse(
            "xspice string vector model\n\
             .model pp process_probe (process_params=[\"--mode\" \"fast\" \"count=2\"])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "process_probe",
            vec![ParamSpec::string_vector("process_params", Vec::new())],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "pp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice model resolves");

        assert_eq!(
            resolved
                .string_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("process_params"))
                .map(|(_, values)| values.as_slice()),
            Some(
                &[
                    "--mode".to_string(),
                    "fast".to_string(),
                    "count=2".to_string()
                ][..]
            )
        );
    }

    #[test]
    fn xspice_resolver_rounds_fractional_integer_vector_model_params_like_ngspice() {
        let netlist = Netlist::parse(
            "xspice fractional integer vector model\n\
             .model bp bit_probe (bits=[1 0.5 1.9])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "bit_probe",
            vec![ParamSpec::integer_vector("bits", vec![0])],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "bp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("fractional integer vector elements round like ngspice");

        assert_eq!(
            resolved
                .integer_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("bits"))
                .map(|(_, values)| values.as_slice()),
            Some(&[1, 1, 2][..])
        );
    }

    #[test]
    fn xspice_resolver_rejects_scalar_model_param_for_known_vector_param() {
        let netlist = Netlist::parse(
            "xspice scalar where vector expected\n\
             .model vp vector_probe (points=3)\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let err = match resolve_xspice_model_instance(
            &netlist,
            &registry,
            "vp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        ) {
            Ok(_) => panic!("scalar value for known vector parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && lowered.contains("scalar"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_resolver_rejects_string_model_param_for_known_vector_param() {
        let netlist = Netlist::parse(
            "xspice string where vector expected\n\
             .model vp vector_probe (points=\"bad\")\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let err = match resolve_xspice_model_instance(
            &netlist,
            &registry,
            "vp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        ) {
            Ok(_) => panic!("string value for known vector parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && lowered.contains("string"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_resolver_rejects_expr_model_param_for_known_vector_param() {
        let netlist = Netlist::parse(
            "xspice expression where vector expected\n\
             .model vp vector_probe (points={missing_param})\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let err = match resolve_xspice_model_instance(
            &netlist,
            &registry,
            "vp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        ) {
            Ok(_) => panic!("expression value for known vector parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && lowered.contains("expression"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_resolver_resolves_scalar_expression_model_params() {
        let netlist = Netlist::parse(
            "xspice scalar expression model param\n\
             .model gp gain_probe (base=2 gain={base*3})\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "gain_probe",
            vec![ParamSpec::real("gain", 1.0)],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "gp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice model resolves");

        assert_eq!(
            resolved
                .numeric_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("gain"))
                .map(|(_, value)| *value),
            Some(6.0)
        );
    }

    #[test]
    fn xspice_resolver_resolves_scalar_expression_instance_params() {
        let netlist = Netlist::parse(
            "xspice scalar expression instance param\n\
             .param scale=2\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "gain_probe",
            vec![ParamSpec::real("gain", 1.0)],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "gain_probe",
            &[("base".to_string(), 3.0)],
            &[("gain".to_string(), "scale*base".to_string())],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice instance expression param resolves");

        assert_eq!(
            resolved
                .numeric_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("gain"))
                .map(|(_, value)| *value),
            Some(6.0)
        );
    }

    #[test]
    fn xspice_resolver_merges_string_instance_params() {
        let netlist = Netlist::parse(
            "xspice string instance param\n\
             .model fp file_probe (file=\"default.tbl\")\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "file_probe",
            vec![ParamSpec::string("file", "")],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "fp",
            &[],
            &[],
            &[("file".to_string(), "instance.tbl".to_string())],
            &[],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice string instance param resolves");

        assert_eq!(
            resolved
                .string_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("file"))
                .map(|(_, value)| value.as_str()),
            Some("instance.tbl")
        );
    }

    #[test]
    fn xspice_resolver_resolves_string_expression_instance_params() {
        let netlist = Netlist::parse(
            "xspice string expression instance param\n\
             .param fname=\"late.tbl\"\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "file_probe",
            vec![ParamSpec::string("file", "")],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "file_probe",
            &[],
            &[],
            &[],
            &[("file".to_string(), "fname".to_string())],
            &[],
            &[],
            &[],
            &[],
        )
        .expect("xspice string expression instance param resolves");

        assert_eq!(
            resolved
                .string_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("file"))
                .map(|(_, value)| value.as_str()),
            Some("late.tbl")
        );
    }

    #[test]
    fn xspice_resolver_merges_real_vector_instance_params() {
        let netlist = Netlist::parse(
            "xspice real vector instance param\n\
             .model vp vector_probe (points=[1 2])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", Vec::new())],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "vp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[("points".to_string(), vec![3.0, 4.0])],
            &[],
        )
        .expect("xspice real-vector instance param resolves");

        assert_eq!(
            resolved
                .real_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("points"))
                .map(|(_, values)| values.as_slice()),
            Some(&[3.0, 4.0][..])
        );
    }

    #[test]
    fn xspice_resolver_converts_integer_vector_instance_params() {
        let netlist = Netlist::parse(
            "xspice integer vector instance param\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "bit_probe",
            vec![ParamSpec::integer_vector("bits", Vec::new())],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "bit_probe",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[("bits".to_string(), vec![1.0, 0.5, 2.2])],
            &[],
        )
        .expect("xspice integer-vector instance param resolves");

        assert_eq!(
            resolved
                .integer_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("bits"))
                .map(|(_, values)| values.as_slice()),
            Some(&[1, 1, 2][..])
        );
    }

    #[test]
    fn xspice_resolver_merges_string_vector_instance_params() {
        let netlist = Netlist::parse(
            "xspice string vector instance param\n\
             .model pp process_probe (process_params=[\"--old\"])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "process_probe",
            vec![ParamSpec::string_vector("process_params", Vec::new())],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "pp",
            &[],
            &[],
            &[],
            &[],
            &[(
                "process_params".to_string(),
                vec!["--mode".to_string(), "fast".to_string()],
            )],
            &[],
            &[],
            &[],
        )
        .expect("xspice string-vector instance param resolves");

        assert_eq!(
            resolved
                .string_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("process_params"))
                .map(|(_, values)| values.as_slice()),
            Some(&["--mode".to_string(), "fast".to_string()][..])
        );
    }

    #[test]
    fn xspice_resolver_resolves_string_vector_expression_instance_params() {
        let netlist = Netlist::parse(
            "xspice string vector expression instance param\n\
             .param args=\"[1e3 --mode -gTarget=4500]\"\n\
             .model pp process_probe (process_params=[\"--old\"])\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "process_probe",
            vec![ParamSpec::string_vector("process_params", Vec::new())],
        )));

        let resolved = resolve_xspice_model_instance(
            &netlist,
            &registry,
            "pp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[("process_params".to_string(), "args".to_string())],
            &[],
            &[],
        )
        .expect("xspice string-vector expression instance param resolves");

        assert_eq!(
            resolved
                .string_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("process_params"))
                .map(|(_, values)| values.as_slice()),
            Some(
                &[
                    "1e3".to_string(),
                    "--mode".to_string(),
                    "-gTarget=4500".to_string(),
                ][..]
            )
        );
    }

    #[test]
    fn xspice_resolver_rejects_expression_instance_param_for_known_vector_param() {
        let netlist = Netlist::parse(
            "xspice expression instance param where vector expected\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "vector_probe",
            vec![ParamSpec::real_vector("points", vec![0.0])],
        )));

        let err = match resolve_xspice_model_instance(
            &netlist,
            &registry,
            "vector_probe",
            &[],
            &[("points".to_string(), "2".to_string())],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        ) {
            Ok(_) => panic!("expression value for known vector parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && lowered.contains("expression"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_resolver_rejects_unresolved_scalar_expression_model_params() {
        let netlist = Netlist::parse(
            "xspice unresolved scalar expression model param\n\
             .model gp gain_probe (gain={missing_param})\n\
             .end\n",
        )
        .expect("netlist parses");
        let mut registry = crate::xspice::CodeModelRegistry::new();
        registry.register(Arc::new(ParamOnlyModel::new(
            "gain_probe",
            vec![ParamSpec::real("gain", 1.0)],
        )));

        let err = match resolve_xspice_model_instance(
            &netlist,
            &registry,
            "gp",
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        ) {
            Ok(_) => panic!("unresolved scalar expression parameter must be rejected"),
            Err(err) => err,
        };

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("gain") && lowered.contains("missing_param"),
            "unexpected error: {message}"
        );
    }
}

#![allow(clippy::needless_range_loop)]
use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TransmissionLineModelParams {
    pub(super) z0: Option<f64>,
    pub(super) td: Option<f64>,
    pub(super) freq: Option<f64>,
    pub(super) nl: Option<f64>,
    pub(super) r: Option<f64>,
    pub(super) g: Option<f64>,
    pub(super) len: Option<f64>,
    pub(super) alpha: Option<f64>,
    pub(super) atten: Option<f64>,
}

#[derive(Debug, Clone)]
pub(super) struct CplModelParams {
    pub(super) r: Vec<Vec<f64>>,
    pub(super) l: Vec<Vec<f64>>,
    pub(super) c: Vec<Vec<f64>>,
    pub(super) g: Vec<Vec<f64>>,
    pub(super) length: f64,
}

pub(super) fn model_param(params: &[(String, f64)], names: &[&str]) -> Option<f64> {
    params.iter().find_map(|(name, value)| {
        if names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            Some(*value)
        } else {
            None
        }
    })
}

pub(super) fn instance_param(params: &[(String, f64)], names: &[&str]) -> Option<f64> {
    params.iter().find_map(|(name, value)| {
        if names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            Some(*value)
        } else {
            None
        }
    })
}

fn normalize_temperature_param_to_celsius(value: f64) -> f64 {
    if value > 200.0 {
        crate::analysis::temperature::kelvin_to_celsius(value)
    } else {
        value
    }
}

fn build_model_eval_context(
    netlist: &Netlist,
    model_def: &crate::netlist::ModelDef,
    current_temp_c: f64,
    tnom_c: f64,
) -> crate::netlist::ParamContext {
    let mut ctx = netlist.params.clone();
    ctx.set("TEMP", current_temp_c);
    ctx.set("TEMPER", current_temp_c);
    ctx.set("TNOM", tnom_c);

    for (name, value) in &model_def.params {
        ctx.set(name, *value);
    }

    let mut pending = model_def.expr_params.clone();
    let mut progress = true;
    while !pending.is_empty() && progress {
        progress = false;
        let mut unresolved = Vec::new();
        for (name, expr) in pending {
            if let Ok(value) = crate::netlist::expr::eval_expression(&expr, &ctx) {
                ctx.set(&name, value);
                progress = true;
            } else {
                unresolved.push((name, expr));
            }
        }
        pending = unresolved;
    }

    ctx
}

fn resolve_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    ctx: &crate::netlist::ParamContext,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        return Ok(Some(value));
    }

    for candidate in names {
        if let Some((name, expr)) = model_def
            .expr_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(candidate))
        {
            let value = crate::netlist::expr::eval_expression(expr, ctx).map_err(|e| {
                SimulationError::Circuit(format!(
                    "Model '{}' parameter '{}' could not be resolved: {}",
                    model_def.name, name, e
                ))
            })?;
            return Ok(Some(value));
        }
    }

    Ok(None)
}

fn resolve_resistor_eval_context(
    netlist: &Netlist,
    model_def: Option<&crate::netlist::ModelDef>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> Result<(crate::netlist::ParamContext, f64, f64), SimulationError> {
    let mut current_temp_c = crate::analysis::temperature::kelvin_to_celsius(temperature_kelvin);
    if let Some(temp) = instance_param(instance_params, &["TEMP"]) {
        current_temp_c = normalize_temperature_param_to_celsius(temp);
    } else if let Some(dtemp) = instance_param(instance_params, &["DTEMP"]) {
        current_temp_c += dtemp;
    }

    let base_tnom_c = netlist.options.tnom.unwrap_or(27.0);
    let Some(model_def) = model_def else {
        let mut ctx = netlist.params.clone();
        ctx.set("TEMP", current_temp_c);
        ctx.set("TEMPER", current_temp_c);
        ctx.set("TNOM", base_tnom_c);
        return Ok((ctx, current_temp_c, base_tnom_c));
    };

    let initial_ctx = build_model_eval_context(netlist, model_def, current_temp_c, base_tnom_c);
    let model_tnom_c = resolve_model_param(model_def, &["TNOM"], &initial_ctx)?
        .map(normalize_temperature_param_to_celsius)
        .unwrap_or(base_tnom_c);

    let ctx = if (model_tnom_c - base_tnom_c).abs() > f64::EPSILON {
        build_model_eval_context(netlist, model_def, current_temp_c, model_tnom_c)
    } else {
        initial_ctx
    };

    Ok((ctx, current_temp_c, model_tnom_c))
}

fn model_binning_param(
    model_def: &crate::netlist::ModelDef,
    name: &str,
) -> Option<f64> {
    model_param(&model_def.params, &[name])
}

fn model_matches_geometry(model_def: &crate::netlist::ModelDef, width: f64, length: f64) -> bool {
    let lmin = model_binning_param(model_def, "LMIN");
    let lmax = model_binning_param(model_def, "LMAX");
    let wmin = model_binning_param(model_def, "WMIN");
    let wmax = model_binning_param(model_def, "WMAX");

    if lmin.is_none() && lmax.is_none() && wmin.is_none() && wmax.is_none() {
        return false;
    }

    let within_l = lmin.is_none_or(|min| length >= min) && lmax.is_none_or(|max| length <= max);
    let within_w = wmin.is_none_or(|min| width >= min) && wmax.is_none_or(|max| width <= max);
    within_l && within_w
}

fn model_bin_range_size(model_def: &crate::netlist::ModelDef) -> f64 {
    let l_range = match (
        model_binning_param(model_def, "LMIN"),
        model_binning_param(model_def, "LMAX"),
    ) {
        (Some(min), Some(max)) => max - min,
        _ => f64::MAX,
    };
    let w_range = match (
        model_binning_param(model_def, "WMIN"),
        model_binning_param(model_def, "WMAX"),
    ) {
        (Some(min), Some(max)) => max - min,
        _ => f64::MAX,
    };
    l_range + w_range
}

pub(super) fn resolve_resistor_instance_value(
    netlist: &Netlist,
    element_name: &str,
    value: f64,
    value_expr: Option<&str>,
    model_name: Option<&str>,
    instance_params: &[(String, f64)],
    temperature_kelvin: f64,
) -> Result<f64, SimulationError> {
    let model_def = if let Some(model_name) = model_name {
        let model_def = find_model_def(netlist, model_name).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "Resistor '{}' references unknown model '{}'",
                element_name, model_name
            ))
        })?;
        ensure_model_type(
            "Resistor",
            element_name,
            model_name,
            model_def,
            &["R", "RES", "RESISTOR"],
        )?;
        Some(model_def)
    } else {
        None
    };

    let (eval_ctx, current_temp_c, tnom_c) =
        resolve_resistor_eval_context(netlist, model_def, instance_params, temperature_kelvin)?;
    let mut resistance = instance_param(instance_params, &["R", "VALUE"]);
    if resistance.is_none() && value.is_finite() && value > 0.0 {
        resistance = Some(value);
    }
    if resistance.is_none() {
        if let Some(expr) = value_expr {
            resistance = Some(
                crate::netlist::expr::eval_expression(expr, &eval_ctx).map_err(|e| {
                    SimulationError::Circuit(format!(
                        "Resistor '{}' value expression could not be resolved: {}",
                        element_name, e
                    ))
                })?,
            );
        }
    }

    if let Some(model_def) = model_def {
        if resistance.is_none() {
            resistance = resolve_model_param(
                model_def,
                &["R", "RES", "R0", "VALUE", "RESISTANCE"],
                &eval_ctx,
            )?;
        }

        if resistance.is_none() {
            let rsh = resolve_model_param(model_def, &["RSH", "SHEETRES", "SHEETR"], &eval_ctx)?
                .ok_or_else(|| {
                    SimulationError::Circuit(format!(
                        "Resistor '{}' model '{}' requires R/RES or RSH with geometry",
                        element_name,
                        model_name.unwrap_or_default()
                    ))
                })?;

            if !rsh.is_finite() || rsh <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' model '{}' has invalid RSH={} (must be finite and > 0)",
                    element_name,
                    model_name.unwrap_or_default(),
                    rsh
                )));
            }

            let squares = if let Some(nsq) =
                instance_param(instance_params, &["NRS", "NRSQ", "NSQ", "SQUARES"])
            {
                nsq
            } else if let Some(nsq) = resolve_model_param(
                model_def,
                &["NRS", "NRSQ", "NSQ", "SQUARES"],
                &eval_ctx,
            )? {
                nsq
            } else {
                let l = instance_param(instance_params, &["L", "LENGTH"])
                    .or_else(|| resolve_model_param(model_def, &["L", "LENGTH"], &eval_ctx).ok().flatten())
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Resistor '{}' model '{}' requires L/LENGTH when using RSH",
                            element_name,
                            model_name.unwrap_or_default()
                        ))
                    })?;
                let w = instance_param(instance_params, &["W", "WIDTH"])
                    .or_else(|| {
                        resolve_model_param(model_def, &["W", "WIDTH", "DEFW"], &eval_ctx)
                            .ok()
                            .flatten()
                    })
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Resistor '{}' model '{}' requires W/WIDTH (or DEFW) when using RSH",
                            element_name,
                            model_name.unwrap_or_default()
                        ))
                    })?;
                let narrow = resolve_model_param(model_def, &["NARROW"], &eval_ctx)?.unwrap_or(0.0);
                let l_eff = l - narrow;
                let w_eff = w - narrow;
                if !l_eff.is_finite() || !w_eff.is_finite() || l_eff <= 0.0 || w_eff <= 0.0 {
                    return Err(SimulationError::Circuit(format!(
                        "Resistor '{}' has invalid effective geometry (L={}, W={}, NARROW={})",
                        element_name, l, w, narrow
                    )));
                }
                l_eff / w_eff
            };

            if !squares.is_finite() || squares <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "Resistor '{}' has invalid number of squares {}",
                    element_name, squares
                )));
            }
            resistance = Some(rsh * squares);
        }
    }

    let mut resolved = resistance.ok_or_else(|| {
        if model_name.is_some() {
            SimulationError::Circuit(format!(
                "Resistor '{}' model-based value could not be resolved",
                element_name
            ))
        } else {
            SimulationError::Circuit(format!(
                "Resistor '{}' has no valid resistance value",
                element_name
            ))
        }
    })?;

    let tc1 = instance_param(instance_params, &["TC1"])
        .or_else(|| model_def.and_then(|model_def| resolve_model_param(model_def, &["TC1"], &eval_ctx).ok().flatten()))
        .unwrap_or(0.0);
    let tc2 = instance_param(instance_params, &["TC2"])
        .or_else(|| model_def.and_then(|model_def| resolve_model_param(model_def, &["TC2"], &eval_ctx).ok().flatten()))
        .unwrap_or(0.0);
    if tc1 != 0.0 || tc2 != 0.0 {
        let temp_ctx = crate::analysis::TemperatureContext::from_celsius(current_temp_c, tnom_c);
        resolved =
            crate::analysis::ResistorTempCoeffs::new(tc1, tc2).scale_resistance(resolved, &temp_ctx);
    }

    if let Some(mult) = instance_param(instance_params, &["M", "MULT"]) {
        if !mult.is_finite() || mult <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Resistor '{}' has invalid multiplicity M={} (must be finite and > 0)",
                element_name, mult
            )));
        }
        resolved /= mult;
    }

    if !resolved.is_finite() || resolved <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "Resistor '{}' resolved to invalid resistance {}",
            element_name, resolved
        )));
    }

    Ok(resolved)
}

pub(super) fn resolve_tline_model_params(
    netlist: &Netlist,
    model_name: &str,
) -> Option<TransmissionLineModelParams> {
    let model = netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))?;

    let mut params = TransmissionLineModelParams {
        z0: model_param(&model.params, &["Z0", "ZO"]),
        td: model_param(&model.params, &["TD", "TDELAY"]),
        freq: model_param(&model.params, &["F", "FREQ"]),
        nl: model_param(&model.params, &["NL"]),
        r: model_param(&model.params, &["R", "R0"]),
        g: model_param(&model.params, &["G", "G0"]),
        len: model_param(&model.params, &["LEN", "LENGTH"]),
        alpha: model_param(&model.params, &["ALPHA"]),
        atten: model_param(&model.params, &["ATTEN", "ATTENDB", "LOSSDB"]),
    };

    let l = model_param(&model.params, &["L", "L0"]);
    let c = model_param(&model.params, &["C", "C0"]);
    let len = params.len;

    if params.z0.is_none() {
        if let (Some(l), Some(c)) = (l, c) {
            if l > 0.0 && c > 0.0 {
                params.z0 = Some((l / c).sqrt());
            }
        }
    }

    if params.td.is_none() {
        if let (Some(f), Some(nl)) = (params.freq, params.nl) {
            if f > 0.0 {
                params.td = Some(nl / f);
            }
        }
    }

    if params.td.is_none() {
        if let (Some(l), Some(c), Some(len)) = (l, c, len) {
            if l > 0.0 && c > 0.0 && len > 0.0 {
                params.td = Some(len * (l * c).sqrt());
            }
        }
    }

    Some(params)
}

fn strip_netlist_comment(line: &str) -> &str {
    let mut end = line.len();
    for (idx, ch) in line.char_indices() {
        if ch == ';' || ch == '$' {
            end = idx;
            break;
        }
    }
    &line[..end]
}

fn extract_cpl_model_body(source: &str, model_name: &str) -> Option<Vec<String>> {
    let mut lines = source.lines().peekable();
    while let Some(line) = lines.next() {
        let trimmed = strip_netlist_comment(line).trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.len() >= 3
            && tokens[0].eq_ignore_ascii_case(".model")
            && tokens[1].eq_ignore_ascii_case(model_name)
            && tokens[2].eq_ignore_ascii_case("cpl")
        {
            let mut body = Vec::new();
            let inline = tokens.iter().skip(3).copied().collect::<Vec<_>>().join(" ");
            if !inline.trim().is_empty() {
                body.push(inline);
            }

            while let Some(next_line) = lines.peek() {
                let trimmed = strip_netlist_comment(next_line).trim();
                let Some(stripped) = trimmed.strip_prefix('+') else {
                    break;
                };
                body.push(stripped.trim().to_string());
                lines.next();
            }

            return Some(body);
        }
    }

    None
}

#[derive(Default)]
struct ParsedCplEntries {
    r: Vec<f64>,
    l: Vec<f64>,
    c: Vec<f64>,
    g: Vec<f64>,
    length: Option<f64>,
}

fn parse_cpl_entries(
    model_name: &str,
    body: &[String],
) -> Result<ParsedCplEntries, SimulationError> {
    let mut parsed = ParsedCplEntries::default();
    let mut current_key: Option<String> = None;

    for line in body {
        let normalized = line.replace(['(', ')', ','], " ");
        let trimmed = normalized.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (key, values_str) = if let Some((lhs, rhs)) = trimmed.split_once('=') {
            let key = lhs.trim().to_ascii_uppercase();
            current_key = Some(key.clone());
            (key, rhs)
        } else {
            let Some(key) = current_key.clone() else {
                continue;
            };
            (key, trimmed)
        };

        let values = values_str
            .split_whitespace()
            .filter_map(|token| token.parse::<f64>().ok())
            .collect::<Vec<_>>();

        match key.as_str() {
            "R" => parsed.r.extend(values),
            "L" => parsed.l.extend(values),
            "C" => parsed.c.extend(values),
            "G" => parsed.g.extend(values),
            "LEN" | "LENGTH" => {
                parsed.length = values.first().copied();
            }
            _ => {}
        }
    }

    if parsed.length.is_none() {
        return Err(SimulationError::Circuit(format!(
            "CPL model '{}' is missing LENGTH/LEN",
            model_name
        )));
    }

    Ok(parsed)
}

fn symmetric_matrix_from_upper_triangle(
    model_name: &str,
    label: &str,
    values: &[f64],
    dimension: usize,
) -> Result<Vec<Vec<f64>>, SimulationError> {
    let expected = dimension * (dimension + 1) / 2;
    if values.len() != expected {
        return Err(SimulationError::Circuit(format!(
            "CPL model '{}' has {} {} entries, expected {} for {} conductors",
            model_name,
            values.len(),
            label,
            expected,
            dimension
        )));
    }

    let mut matrix = vec![vec![0.0; dimension]; dimension];
    let mut idx = 0usize;
    for row in 0..dimension {
        for col in row..dimension {
            let value = values[idx];
            matrix[row][col] = value;
            matrix[col][row] = value;
            idx += 1;
        }
    }
    Ok(matrix)
}

pub(super) fn resolve_cpl_model_params(
    netlist: &Netlist,
    model_name: &str,
    conductors: usize,
) -> Result<Option<CplModelParams>, SimulationError> {
    let model = netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name));
    let Some(model) = model else {
        return Ok(None);
    };
    if !model.model_type.eq_ignore_ascii_case("CPL") {
        return Ok(None);
    }

    let source = netlist.source_text.as_deref().ok_or_else(|| {
        SimulationError::Circuit(format!(
            "CPL model '{}' requires raw source text for RLGC matrix resolution",
            model_name
        ))
    })?;
    let body = extract_cpl_model_body(source, model_name).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Unable to locate raw .MODEL block for CPL model '{}'",
            model_name
        ))
    })?;
    let parsed = parse_cpl_entries(model_name, &body)?;
    let expected = conductors * (conductors + 1) / 2;

    let r_entries = if parsed.r.is_empty() {
        vec![0.0; expected]
    } else {
        parsed.r
    };
    let g_entries = if parsed.g.is_empty() {
        vec![0.0; expected]
    } else {
        parsed.g
    };

    let length = parsed.length.unwrap_or(0.0);
    if !length.is_finite() || length <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "CPL model '{}' has invalid LENGTH={} (must be finite and > 0)",
            model_name, length
        )));
    }

    Ok(Some(CplModelParams {
        r: symmetric_matrix_from_upper_triangle(model_name, "R", &r_entries, conductors)?,
        l: symmetric_matrix_from_upper_triangle(model_name, "L", &parsed.l, conductors)?,
        c: symmetric_matrix_from_upper_triangle(model_name, "C", &parsed.c, conductors)?,
        g: symmetric_matrix_from_upper_triangle(model_name, "G", &g_entries, conductors)?,
        length,
    }))
}

pub(super) fn tline_model_attenuation(params: TransmissionLineModelParams, z0: f64) -> Option<f64> {
    let len = params.len.unwrap_or(1.0).max(0.0);

    // Explicit alpha (Np/unit length) takes precedence.
    if let Some(alpha) = params.alpha {
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    // ATTEN/ATTENDB: interpret <=1 as linear ratio, otherwise as dB.
    if let Some(atten) = params.atten {
        if atten.is_finite() && atten >= 0.0 {
            if atten <= 1.0 {
                return Some(atten);
            }
            let db_total = if params.len.is_some() {
                atten * len
            } else {
                atten
            };
            return Some(10_f64.powf(-db_total / 20.0));
        }
    }

    // Derive from primary RLGC line loss when available.
    let r = params.r.unwrap_or(0.0).max(0.0);
    let g = params.g.unwrap_or(0.0).max(0.0);
    if (r > 0.0 || g > 0.0) && z0.is_finite() && z0 > 0.0 {
        let alpha = r / (2.0 * z0) + g * z0 / 2.0;
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    None
}

pub(super) fn positive_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    param_label: &str,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        if !value.is_finite() || value <= 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Jiles-Atherton model '{}' has invalid {}={} (must be finite and > 0)",
                model_def.name, param_label, value
            )));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub(super) fn unit_interval_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    param_label: &str,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        if !value.is_finite() || !(0.0..=1.0).contains(&value) {
            return Err(SimulationError::Circuit(format!(
                "Jiles-Atherton model '{}' has invalid {}={} (must be finite and within [0, 1])",
                model_def.name, param_label, value
            )));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub(super) fn nonnegative_model_param(
    model_def: &crate::netlist::ModelDef,
    names: &[&str],
    param_label: &str,
) -> Result<Option<f64>, SimulationError> {
    if let Some(value) = model_param(&model_def.params, names) {
        if !value.is_finite() || value < 0.0 {
            return Err(SimulationError::Circuit(format!(
                "Jiles-Atherton model '{}' has invalid {}={} (must be finite and >= 0)",
                model_def.name, param_label, value
            )));
        }
        Ok(Some(value))
    } else {
        Ok(None)
    }
}

pub(super) fn resolve_jiles_atherton_model_params(
    model_def: &crate::netlist::ModelDef,
    nominal_inductance: f64,
) -> Result<crate::device::passive::JilesAthertonParams, SimulationError> {
    let mut params = crate::device::passive::JilesAthertonParams::default();
    let mut explicit_turns = false;

    if let Some(ms) = positive_model_param(model_def, &["MS"], "MS")? {
        params.ms = ms;
    }
    if let Some(a) = positive_model_param(model_def, &["A"], "A")? {
        params.a = a;
    }
    if let Some(k) = positive_model_param(model_def, &["K"], "K")? {
        params.k = k;
    }
    if let Some(c) = unit_interval_model_param(model_def, &["C"], "C")? {
        params.c = c;
    }
    if let Some(alpha) = nonnegative_model_param(model_def, &["ALPHA"], "ALPHA")? {
        params.alpha = alpha;
    }
    if let Some(area) = positive_model_param(model_def, &["AREA", "ACORE", "COREAREA"], "AREA")? {
        params.area = area;
    }
    if let Some(length) = positive_model_param(model_def, &["LENGTH", "LEN", "PATHLEN"], "LENGTH")?
    {
        params.length = length;
    }
    if let Some(n_turns) = positive_model_param(model_def, &["N", "NT", "NTURNS", "TURNS"], "N")? {
        params.n_turns = n_turns;
        explicit_turns = true;
    }

    if !explicit_turns && nominal_inductance.is_finite() && nominal_inductance > 0.0 {
        let base = params.base_inductance();
        if base.is_finite() && base > 0.0 {
            let turns_scale = (nominal_inductance / base).sqrt();
            if turns_scale.is_finite() && turns_scale > 0.0 {
                params.n_turns *= turns_scale;
            }
        }
    }

    Ok(params)
}

pub(super) fn resolve_bjt_type_from_model(model_type: &str) -> Option<crate::netlist::BjtType> {
    if model_type.eq_ignore_ascii_case("NPN") {
        Some(crate::netlist::BjtType::Npn)
    } else if model_type.eq_ignore_ascii_case("PNP") {
        Some(crate::netlist::BjtType::Pnp)
    } else {
        None
    }
}

pub(super) fn resolve_mos_type_from_model(model_type: &str) -> Option<crate::netlist::MosType> {
    if model_type.eq_ignore_ascii_case("NMOS") {
        Some(crate::netlist::MosType::Nmos)
    } else if model_type.eq_ignore_ascii_case("PMOS") {
        Some(crate::netlist::MosType::Pmos)
    } else {
        None
    }
}

pub(super) fn resolve_jfet_type_from_model(model_type: &str) -> Option<crate::netlist::JfetType> {
    if model_type.eq_ignore_ascii_case("NJF") {
        Some(crate::netlist::JfetType::Njf)
    } else if model_type.eq_ignore_ascii_case("PJF") {
        Some(crate::netlist::JfetType::Pjf)
    } else {
        None
    }
}

pub(super) fn resolve_mesfet_type_from_model(
    model_type: &str,
) -> Option<crate::netlist::MesfetType> {
    if model_type.eq_ignore_ascii_case("NMF") || model_type.eq_ignore_ascii_case("NHFET") {
        Some(crate::netlist::MesfetType::Nmf)
    } else if model_type.eq_ignore_ascii_case("PMF") || model_type.eq_ignore_ascii_case("PHFET") {
        Some(crate::netlist::MesfetType::Pmf)
    } else {
        None
    }
}

pub(super) fn find_model_def<'a>(
    netlist: &'a Netlist,
    model_name: &str,
) -> Option<&'a crate::netlist::ModelDef> {
    netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))
}

pub(super) fn find_binned_model_def<'a>(
    netlist: &'a Netlist,
    model_name: &str,
    instance_params: &[(String, f64)],
) -> Option<&'a crate::netlist::ModelDef> {
    let exact = find_model_def(netlist, model_name);
    if exact.is_some() {
        return exact;
    }

    let width = instance_param(instance_params, &["W", "WIDTH"])?;
    let length = instance_param(instance_params, &["L", "LENGTH"])?;
    let prefix = format!("{model_name}.");

    netlist
        .models
        .iter()
        .filter(|model_def| {
            model_def.name.len() > prefix.len()
                && model_def.name[..prefix.len()].eq_ignore_ascii_case(&prefix)
                && model_matches_geometry(model_def, width, length)
        })
        .min_by(|left, right| {
            model_bin_range_size(left)
                .partial_cmp(&model_bin_range_size(right))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

pub(super) fn expected_model_type_text(expected_types: &[&str]) -> String {
    match expected_types {
        [] => String::new(),
        [single] => (*single).to_string(),
        [left, right] => format!("{left} or {right}"),
        _ => expected_types.join(", "),
    }
}

pub(super) fn ensure_model_type(
    element_kind: &str,
    element_name: &str,
    model_name: &str,
    model_def: &crate::netlist::ModelDef,
    expected_types: &[&str],
) -> Result<(), SimulationError> {
    if expected_types
        .iter()
        .any(|kind| model_def.model_type.eq_ignore_ascii_case(kind))
    {
        return Ok(());
    }

    let expected = expected_model_type_text(expected_types);
    Err(SimulationError::Circuit(format!(
        "{} '{}' references model '{}' with incompatible type '{}'; expected {}",
        element_kind, element_name, model_name, model_def.model_type, expected
    )))
}

pub(super) fn map_switch_state(state: crate::netlist::SwitchState) -> crate::device::SwitchState {
    match state {
        crate::netlist::SwitchState::On => crate::device::SwitchState::On,
        crate::netlist::SwitchState::Off => crate::device::SwitchState::Off,
    }
}

pub(super) struct ResolvedXspiceModel {
    pub(super) code_model: std::sync::Arc<dyn crate::xspice::CodeModel>,
    pub(super) numeric_params: Vec<(String, f64)>,
    pub(super) string_params: Vec<(String, String)>,
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

pub(super) fn resolve_xspice_model_instance(
    netlist: &Netlist,
    registry: &crate::xspice::CodeModelRegistry,
    model_name: &str,
    instance_params: &[(String, f64)],
) -> Result<ResolvedXspiceModel, SimulationError> {
    if let Some(code_model) = registry.get(model_name) {
        return Ok(ResolvedXspiceModel {
            code_model,
            numeric_params: instance_params.to_vec(),
            string_params: Vec::new(),
        });
    }

    let model_def = find_model_def(netlist, model_name).ok_or_else(|| {
        SimulationError::Circuit(format!("Unknown XSPICE model '{}'", model_name))
    })?;

    let code_model = registry.get(&model_def.model_type).ok_or_else(|| {
        SimulationError::Circuit(format!(
            "Unknown XSPICE model '{}' (alias '{}' resolves to unregistered code model '{}')",
            model_name, model_def.name, model_def.model_type
        ))
    })?;

    Ok(ResolvedXspiceModel {
        code_model,
        numeric_params: merge_numeric_params(&model_def.params, instance_params),
        string_params: model_def.string_params.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::{merge_numeric_params, resolve_cpl_model_params, resolve_mesfet_type_from_model};
    use crate::Netlist;

    #[test]
    fn resolve_mesfet_type_accepts_hfet_aliases() {
        assert_eq!(
            resolve_mesfet_type_from_model("NHFET"),
            Some(crate::netlist::MesfetType::Nmf)
        );
        assert_eq!(
            resolve_mesfet_type_from_model("PHFET"),
            Some(crate::netlist::MesfetType::Pmf)
        );
        assert_eq!(
            resolve_mesfet_type_from_model("NMF"),
            Some(crate::netlist::MesfetType::Nmf)
        );
        assert_eq!(
            resolve_mesfet_type_from_model("PMF"),
            Some(crate::netlist::MesfetType::Pmf)
        );
        assert_eq!(resolve_mesfet_type_from_model("UNKNOWN"), None);
    }

    #[test]
    fn merge_numeric_params_overrides_case_insensitively() {
        let merged = merge_numeric_params(
            &[("GAIN".to_string(), 2.0), ("OFFSET".to_string(), 1.0)],
            &[("gain".to_string(), 4.0), ("DELAY".to_string(), 5.0)],
        );

        assert_eq!(
            merged
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("gain"))
                .map(|(_, value)| *value),
            Some(4.0)
        );
        assert_eq!(
            merged
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("offset"))
                .map(|(_, value)| *value),
            Some(1.0)
        );
        assert_eq!(
            merged
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("delay"))
                .map(|(_, value)| *value),
            Some(5.0)
        );
    }

    #[test]
    fn resolve_cpl_model_params_parses_upper_triangular_matrices() {
        let netlist = Netlist::parse(
            r#"
P1 V1 V2 0 V3 V4 0 CPL1
.model cpl1 cpl
+R = 0.5 0 0.5
+L = 247.3e-9 31.65e-9
+            247.3e-9
+C = 31.4e-12 -2.45e-12
+            31.4e-12
+G = 0 0 0
+length = 0.3048
.end
"#,
        )
        .expect("netlist should parse");

        let cpl = resolve_cpl_model_params(&netlist, "cpl1", 2)
            .expect("CPL model should resolve")
            .expect("CPL model should exist");

        assert!((cpl.length - 0.3048).abs() < 1e-15);
        assert!((cpl.r[0][0] - 0.5).abs() < 1e-15);
        assert!((cpl.r[0][1] - 0.0).abs() < 1e-15);
        assert!((cpl.r[1][1] - 0.5).abs() < 1e-15);
        assert!((cpl.l[0][1] - 31.65e-9).abs() < 1e-21);
        assert!((cpl.c[0][1] + 2.45e-12).abs() < 1e-24);
        assert!((cpl.g[0][0] - 0.0).abs() < 1e-18);
    }

    #[test]
    fn resolve_cpl_model_params_defaults_optional_r_and_g_to_zero() {
        let netlist = Netlist::parse(
            r#"
P1 V1 V2 0 V3 V4 0 CPL1
.model cpl1 cpl
+L = 247.3e-9  31.65e-9
+              247.3e-9
+C = 31.4e-12 -2.45e-12
+              31.4e-12
+length = 0.3048
.end
"#,
        )
        .expect("netlist should parse");

        let cpl = resolve_cpl_model_params(&netlist, "cpl1", 2)
            .expect("CPL model should resolve")
            .expect("CPL model should exist");

        assert!(cpl.r.iter().flatten().all(|value| value.abs() < 1e-18));
        assert!(cpl.g.iter().flatten().all(|value| value.abs() < 1e-18));
    }

    #[test]
    fn resolve_cpl_model_params_requires_length() {
        let netlist = Netlist::parse(
            r#"
P1 V1 V2 0 V3 V4 0 CPL1
.model cpl1 cpl
+R = 0.5 0 0.5
+L = 247.3e-9  31.65e-9
+              247.3e-9
+C = 31.4e-12 -2.45e-12
+              31.4e-12
.end
"#,
        )
        .expect("netlist should parse");

        let err =
            resolve_cpl_model_params(&netlist, "cpl1", 2).expect_err("missing LENGTH should error");
        assert!(
            err.to_string().contains("missing LENGTH"),
            "expected missing LENGTH error, got {}",
            err
        );
    }
}

use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::engine::builder) struct TransmissionLineModelParams {
    pub(in crate::engine::builder) z0: Option<f64>,
    pub(in crate::engine::builder) td: Option<f64>,
    pub(in crate::engine::builder) freq: Option<f64>,
    pub(in crate::engine::builder) nl: Option<f64>,
    pub(in crate::engine::builder) r: Option<f64>,
    pub(in crate::engine::builder) l: Option<f64>,
    pub(in crate::engine::builder) g: Option<f64>,
    pub(in crate::engine::builder) c: Option<f64>,
    pub(in crate::engine::builder) len: Option<f64>,
    pub(in crate::engine::builder) alpha: Option<f64>,
    pub(in crate::engine::builder) atten: Option<f64>,
    pub(in crate::engine::builder) compactrel: Option<f64>,
    pub(in crate::engine::builder) compactabs: Option<f64>,
}

#[derive(Debug, Clone)]
pub(in crate::engine::builder) struct CplModelParams {
    pub(in crate::engine::builder) r: Vec<Vec<f64>>,
    pub(in crate::engine::builder) l: Vec<Vec<f64>>,
    pub(in crate::engine::builder) c: Vec<Vec<f64>>,
    pub(in crate::engine::builder) g: Vec<Vec<f64>>,
    pub(in crate::engine::builder) length: f64,
}

pub(in crate::engine::builder) fn resolve_tline_model_params(
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
        l: model_param(&model.params, &["L", "L0"]),
        g: model_param(&model.params, &["G", "G0"]),
        c: model_param(&model.params, &["C", "C0"]),
        len: model_param(&model.params, &["LEN", "LENGTH"]),
        alpha: model_param(&model.params, &["ALPHA"]),
        atten: model_param(&model.params, &["ATTEN", "ATTENDB", "LOSSDB"]),
        compactrel: model_param(&model.params, &["COMPACTREL"]),
        compactabs: model_param(&model.params, &["COMPACTABS"]),
    };

    let l = params.l;
    let c = params.c;
    let len = params.len;

    if params.z0.is_none()
        && let (Some(l), Some(c)) = (l, c)
        && l > 0.0
        && c > 0.0
    {
        params.z0 = Some((l / c).sqrt());
    }

    if params.td.is_none()
        && let (Some(f), Some(nl)) = (params.freq, params.nl)
        && f > 0.0
    {
        params.td = Some(nl / f);
    }

    if params.td.is_none()
        && let (Some(l), Some(c), Some(len)) = (l, c, len)
        && l > 0.0
        && c > 0.0
        && len > 0.0
    {
        params.td = Some(len * (l * c).sqrt());
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

pub(in crate::engine::builder) fn resolve_cpl_model_params(
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

pub(in crate::engine::builder) fn tline_model_attenuation(
    params: TransmissionLineModelParams,
    z0: f64,
) -> Option<f64> {
    let len = params.len.unwrap_or(1.0).max(0.0);

    // Explicit alpha (Np/unit length) takes precedence.
    if let Some(alpha) = params.alpha
        && alpha.is_finite()
        && alpha >= 0.0
    {
        return Some((-alpha * len).exp());
    }

    // ATTEN/ATTENDB: interpret <=1 as linear ratio, otherwise as dB.
    if let Some(atten) = params.atten
        && atten.is_finite()
        && atten >= 0.0
    {
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

#[allow(dead_code)]
pub(in crate::engine::builder) fn tline_model_loss_time_constant(
    params: TransmissionLineModelParams,
) -> Option<f64> {
    let len = params.len?;
    if !len.is_finite() || len <= 0.0 {
        return None;
    }

    let rc_term = params.r.unwrap_or(0.0).max(0.0) * params.c.unwrap_or(0.0).max(0.0);
    let lg_term = params.l.unwrap_or(0.0).max(0.0) * params.g.unwrap_or(0.0).max(0.0);
    let tau = 0.5 * (rc_term + lg_term) * len * len;
    if tau.is_finite() && tau > 0.0 {
        Some(tau)
    } else {
        None
    }
}

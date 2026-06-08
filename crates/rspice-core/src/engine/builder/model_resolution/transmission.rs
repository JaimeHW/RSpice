use super::*;

const TXL_MIN_INDUCTANCE: f64 = 1.0e-12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::engine::builder) enum TransmissionLineModelKind {
    Ltra,
    Txl,
}

impl Default for TransmissionLineModelKind {
    fn default() -> Self {
        Self::Ltra
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(in crate::engine::builder) struct TransmissionLineModelParams {
    pub(in crate::engine::builder) kind: TransmissionLineModelKind,
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
    pub(in crate::engine::builder) rel: Option<f64>,
    pub(in crate::engine::builder) abs: Option<f64>,
    pub(in crate::engine::builder) compactrel: Option<f64>,
    pub(in crate::engine::builder) compactabs: Option<f64>,
}

impl TransmissionLineModelParams {
    #[inline]
    pub(in crate::engine::builder) fn is_txl(self) -> bool {
        self.kind == TransmissionLineModelKind::Txl
    }

    #[inline]
    pub(in crate::engine::builder) fn uses_txl_lossless_branch(self) -> bool {
        if self.kind != TransmissionLineModelKind::Txl {
            return false;
        }

        let (Some(r), Some(l)) = (self.r, self.l) else {
            return false;
        };
        if !r.is_finite() || !l.is_finite() || l <= 0.0 {
            return false;
        }

        let g = self.g.unwrap_or(0.0);
        g.is_finite() && g < 1.0e-2 && r / l < 5.0e5
    }
}

#[derive(Debug, Clone)]
pub(in crate::engine::builder) struct CplModelParams {
    pub(in crate::engine::builder) r: Vec<Vec<f64>>,
    pub(in crate::engine::builder) l: Vec<Vec<f64>>,
    pub(in crate::engine::builder) c: Vec<Vec<f64>>,
    pub(in crate::engine::builder) g: Vec<Vec<f64>>,
    pub(in crate::engine::builder) length: f64,
}

pub(in crate::engine::builder) const CPL_MIN_SERIES_RESISTANCE_PER_LENGTH: f64 = 1.0e-4;

pub(in crate::engine::builder) fn resolve_tline_model_params(
    netlist: &Netlist,
    model_name: &str,
) -> Result<Option<TransmissionLineModelParams>, SimulationError> {
    let model = netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name));
    let Some(model) = model else {
        return Ok(None);
    };

    let mut params = TransmissionLineModelParams {
        kind: if model.model_type.eq_ignore_ascii_case("TXL") {
            TransmissionLineModelKind::Txl
        } else {
            TransmissionLineModelKind::Ltra
        },
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
        rel: model_param(&model.params, &["REL", "RELTOL"]),
        abs: model_param(&model.params, &["ABS", "ABSTOL"]),
        compactrel: model_param(&model.params, &["COMPACTREL"]),
        compactabs: model_param(&model.params, &["COMPACTABS"]),
    };

    if params.kind == TransmissionLineModelKind::Txl {
        params = finalize_txl_model_params(model_name, params)?;
    }

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

    Ok(Some(params))
}

fn require_txl_param(
    model_name: &str,
    param_name: &str,
    value: Option<f64>,
) -> Result<f64, SimulationError> {
    let value = value.ok_or_else(|| {
        SimulationError::Circuit(format!(
            "TXL model '{}' is missing required {} parameter",
            model_name, param_name
        ))
    })?;
    if !value.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "TXL model '{}' has non-finite {}={}",
            model_name, param_name, value
        )));
    }
    Ok(value)
}

fn finalize_txl_model_params(
    model_name: &str,
    mut params: TransmissionLineModelParams,
) -> Result<TransmissionLineModelParams, SimulationError> {
    let r = require_txl_param(model_name, "R", params.r)?;
    let l = require_txl_param(model_name, "L", params.l)?;
    let g = require_txl_param(model_name, "G", params.g)?;
    let c = require_txl_param(model_name, "C", params.c)?;
    let len = require_txl_param(model_name, "LENGTH", params.len)?;

    if r < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "TXL model '{}' has invalid R={} (must be >= 0)",
            model_name, r
        )));
    }
    if g < 0.0 {
        return Err(SimulationError::Circuit(format!(
            "TXL model '{}' has invalid G={} (must be >= 0)",
            model_name, g
        )));
    }
    if c <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "TXL model '{}' has invalid C={} (must be > 0)",
            model_name, c
        )));
    }
    if len <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "TXL model '{}' has invalid LENGTH={} (must be > 0)",
            model_name, len
        )));
    }

    params.l = Some(l.max(TXL_MIN_INDUCTANCE));
    Ok(params)
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

        if !matches!(key.as_str(), "R" | "L" | "C" | "G" | "LEN" | "LENGTH") {
            continue;
        }

        match key.as_str() {
            "R" => parsed
                .r
                .extend(parse_cpl_value_list(model_name, &key, values_str)?),
            "L" => parsed
                .l
                .extend(parse_cpl_value_list(model_name, &key, values_str)?),
            "C" => parsed
                .c
                .extend(parse_cpl_value_list(model_name, &key, values_str)?),
            "G" => parsed
                .g
                .extend(parse_cpl_value_list(model_name, &key, values_str)?),
            "LEN" | "LENGTH" => {
                let values = parse_cpl_value_list(model_name, &key, values_str)?;
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

fn parse_cpl_value_list(
    model_name: &str,
    key: &str,
    values_str: &str,
) -> Result<Vec<f64>, SimulationError> {
    values_str
        .split_whitespace()
        .map(|token| {
            crate::netlist::lexer::parse_spice_value(token).map_err(|err| {
                SimulationError::Circuit(format!(
                    "CPL model '{}' has invalid {} value '{}': {}",
                    model_name, key, token, err
                ))
            })
        })
        .collect()
}

fn symmetric_matrix_from_upper_triangle(
    model_name: &str,
    label: &str,
    values: &[f64],
    dimension: usize,
    floor_abs_below: Option<f64>,
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
            let mut value = values[idx];
            if !value.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "CPL model '{}' has non-finite {}[{},{}]",
                    model_name,
                    label,
                    row + 1,
                    col + 1
                )));
            }
            if let Some(min_value) = floor_abs_below
                && value.abs() < min_value
            {
                value = min_value;
            }
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

    let length = parsed.length.unwrap_or(0.0);
    if !length.is_finite() || length <= 0.0 {
        return Err(SimulationError::Circuit(format!(
            "CPL model '{}' has invalid LENGTH={} (must be finite and > 0)",
            model_name, length
        )));
    }

    Ok(Some(CplModelParams {
        r: symmetric_matrix_from_upper_triangle(
            model_name,
            "R",
            &parsed.r,
            conductors,
            Some(CPL_MIN_SERIES_RESISTANCE_PER_LENGTH),
        )?,
        l: symmetric_matrix_from_upper_triangle(model_name, "L", &parsed.l, conductors, None)?,
        c: symmetric_matrix_from_upper_triangle(model_name, "C", &parsed.c, conductors, None)?,
        g: symmetric_matrix_from_upper_triangle(model_name, "G", &parsed.g, conductors, None)?,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn resolve_test_cpl(source: &str, conductors: usize) -> CplModelParams {
        let netlist = crate::netlist::Netlist::parse(source).expect("test netlist parses");
        resolve_cpl_model_params(&netlist, "m", conductors)
            .expect("CPL model resolves")
            .expect("CPL model exists")
    }

    fn resolve_test_txl(source: &str) -> TransmissionLineModelParams {
        let netlist = crate::netlist::Netlist::parse(source).expect("test netlist parses");
        resolve_tline_model_params(&netlist, "y")
            .expect("TXL model resolves")
            .expect("TXL model exists")
    }

    #[test]
    fn txl_clamps_l_before_derived_delay_and_impedance() {
        let params = resolve_test_txl(
            r#"title
.model y txl r=0 l=1e-15 g=0 c=1e-12 length=2
.end
"#,
        );

        assert_eq!(params.l, Some(TXL_MIN_INDUCTANCE));
        assert_eq!(params.z0, Some(1.0));
        assert_eq!(params.td, Some(2.0e-12));
    }

    #[test]
    fn txl_requires_primary_rlgc_and_length() {
        let netlist = crate::netlist::Netlist::parse(
            r#"title
.model y txl r=1 l=1n c=1p length=1
.end
"#,
        )
        .expect("test netlist parses");

        let err = resolve_tline_model_params(&netlist, "y").expect_err("missing G is rejected");
        assert!(
            err.to_string().contains("missing required G parameter"),
            "{err}"
        );
    }

    #[test]
    fn cpl_r_entries_below_ngspice_floor_are_mirrored_to_minimum() {
        let params = resolve_test_cpl(
            r#"title
.model m cpl
+ r = 0 0 2e-4
+ l = 1n 0 1n
+ c = 1p -0.1p 1p
+ g = 0 0 0
+ length = 1
.end
"#,
            2,
        );

        assert_eq!(params.r[0][0], CPL_MIN_SERIES_RESISTANCE_PER_LENGTH);
        assert_eq!(params.r[0][1], CPL_MIN_SERIES_RESISTANCE_PER_LENGTH);
        assert_eq!(params.r[1][0], CPL_MIN_SERIES_RESISTANCE_PER_LENGTH);
        assert_eq!(params.r[1][1], 2e-4);
        assert_eq!(params.c[0][1], -0.1e-12);
        assert_eq!(params.c[1][0], -0.1e-12);
    }

    #[test]
    fn cpl_ibm2_r_matches_ngspice_off_diagonal_floor() {
        let params = resolve_test_cpl(
            r#"title
.model m cpl
+ r = 0.5 0 0.5
+ l = 1n 0.1n 1n
+ c = 1p -0.1p 1p
+ g = 0 0 0
+ length = 1
.end
"#,
            2,
        );

        assert_eq!(params.r[0][0], 0.5);
        assert_eq!(params.r[0][1], CPL_MIN_SERIES_RESISTANCE_PER_LENGTH);
        assert_eq!(params.r[1][0], CPL_MIN_SERIES_RESISTANCE_PER_LENGTH);
        assert_eq!(params.r[1][1], 0.5);
    }

    #[test]
    fn cpl3_4_line_r_matches_ngspice_off_diagonal_floor() {
        let params = resolve_test_cpl(
            r#"title
.model m cpl
+ r = 0.3 0 0 0 0.3 0 0 0.3 0 0.3
+ l = 1n 0.1n 0.1n 0.1n 1n 0.1n 0.1n 1n 0.1n 1n
+ c = 1p -0.1p -0.1p -0.1p 1p -0.1p -0.1p 1p -0.1p 1p
+ g = 0 0 0 0 0 0 0 0 0 0
+ length = 1
.end
"#,
            4,
        );

        for row in 0..4 {
            for col in 0..4 {
                let expected = if row == col {
                    0.3
                } else {
                    CPL_MIN_SERIES_RESISTANCE_PER_LENGTH
                };
                assert_eq!(params.r[row][col], expected);
            }
        }
    }

    #[test]
    fn cpl_requires_full_upper_triangle_matrices() {
        let netlist = crate::netlist::Netlist::parse(
            r#"title
.model m cpl
+ r = 0 0 0
+ l = 1n 0
+ c = 1p 0 1p
+ g = 0 0 0
+ length = 1
.end
"#,
        )
        .expect("test netlist parses");

        let err = resolve_cpl_model_params(&netlist, "m", 2)
            .expect_err("short upper triangle is rejected");
        assert!(
            err.to_string().contains("has 2 L entries, expected 3"),
            "{err}"
        );
    }

    #[test]
    fn cpl_requires_g_matrix_like_ngspice_setup() {
        let netlist = crate::netlist::Netlist::parse(
            r#"title
.model m cpl
+ r = 0 0 0
+ l = 1n 0 1n
+ c = 1p 0 1p
+ length = 1
.end
"#,
        )
        .expect("test netlist parses");

        let err = resolve_cpl_model_params(&netlist, "m", 2).expect_err("missing G is rejected");
        assert!(
            err.to_string().contains("has 0 G entries, expected 3"),
            "{err}"
        );
    }
}

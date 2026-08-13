//! Periodic-analysis directives authored in a manual deck.
//!
//! The core parser deliberately reports these as unsupported until their AST
//! is shared by every frontend. The Netlist workspace still executes them
//! through the UI's typed analysis pipeline: this parser accepts the exact
//! source card, validates every operand, and freezes all execution options on
//! the queued task. Nothing is borrowed from the Simulation Studio dialogs.

use std::collections::{HashMap, HashSet};

use rspice_core::netlist::expr::eval_expression;
use rspice_core::netlist::{Netlist, ParamContext};

use super::*;
use crate::services::simulation_runner::{
    PacFrequencySweep, PacRunConfig, PnoiseFrequencySweep, PnoiseReference, PnoiseRunConfig,
    PstbRunConfig, PxfFrequencySweep, PxfRunConfig,
};
use crate::simulation::multi_run::{AnalysisSpec, PssMethod};
use crate::simulation::runner::SpecExecutionOptions;

#[derive(Debug)]
struct ParsedCard {
    source: String,
    positional: Vec<String>,
    keyed: HashMap<String, String>,
}

pub(super) fn parse_periodic_tasks(
    netlist: &Netlist,
    source: &str,
) -> Result<Vec<QueuedAnalysis>, Vec<String>> {
    let cards = logical_cards(source);
    let mut parsed = Vec::new();
    let mut errors = Vec::new();
    for (line, card) in cards {
        let Some(head) = card.split_whitespace().next() else {
            continue;
        };
        if !matches_ignore_ascii_case(head, &[".pss", ".pac", ".pnoise", ".pxf", ".pstb"]) {
            continue;
        }
        match parse_card(&card) {
            Ok(card) => parsed.push((line, head.to_ascii_lowercase(), card)),
            Err(error) => errors.push(format!("line {line}: {error}")),
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }

    let pss_cards = parsed
        .iter()
        .filter(|(_, head, _)| head == ".pss")
        .collect::<Vec<_>>();
    if pss_cards.len() > 1 {
        return Err(vec![format!(
            "Manual decks support one authenticated .PSS operating point per run; found {}.",
            pss_cards.len()
        )]);
    }
    let pss_dependent_count = parsed
        .iter()
        .filter(|(_, head, _)| matches!(head.as_str(), ".pac" | ".pnoise" | ".pxf" | ".pstb"))
        .count();
    if pss_dependent_count > 0 && pss_cards.is_empty() {
        return Err(vec![
            ".PAC, .PNOISE, .PXF, and .PSTB require one .PSS directive in the same manual deck so the exact periodic operating point can be bound."
                .to_owned(),
        ]);
    }

    let pss_spec = pss_cards
        .first()
        .map(|(line, _, card)| {
            parse_pss(card, &netlist.params).map_err(|error| format!("line {line}: {error}"))
        })
        .transpose()
        .map_err(|error| vec![error])?;
    let (pss_fundamental, pss_harmonics, pss_tolerance) = match &pss_spec {
        Some(AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
            ..
        }) => (*fundamental_freq, *num_harmonics, *tolerance),
        _ => (0.0, 0, 0.0),
    };
    if pss_dependent_count > 0 && pss_harmonics == 0 {
        return Err(vec![
            "Periodic small-signal analyses require .PSS save_harmonics to be greater than zero."
                .to_owned(),
        ]);
    }
    let reltol = netlist
        .options
        .reltol
        .unwrap_or(rspice_core::constants::RELTOL);
    let abstol = netlist
        .options
        .abstol
        .unwrap_or(rspice_core::constants::ABSTOL);

    let mut tasks = Vec::new();
    for (line, head, card) in parsed {
        let task = match head.as_str() {
            ".pss" => QueuedAnalysis {
                numeric_override: None,
                spec: pss_spec.clone().expect("one parsed PSS spec exists"),
                config: None,
                spec_options: SpecExecutionOptions::default(),
                analysis_line: card.source.clone(),
            },
            ".pac" => {
                let config = parse_pac(
                    &card,
                    &netlist.params,
                    pss_fundamental,
                    pss_harmonics,
                    pss_tolerance,
                    reltol,
                    abstol,
                )
                .map_err(|error| vec![format!("line {line}: {error}")])?;
                QueuedAnalysis {
                    numeric_override: None,
                    spec: AnalysisSpec::Pac,
                    config: None,
                    spec_options: SpecExecutionOptions {
                        pac: Some(config),
                        ..SpecExecutionOptions::default()
                    },
                    analysis_line: card.source.clone(),
                }
            }
            ".pnoise" => {
                let config = parse_pnoise(
                    &card,
                    &netlist.params,
                    pss_fundamental,
                    pss_harmonics,
                    pss_tolerance,
                    reltol,
                    abstol,
                )
                .map_err(|error| vec![format!("line {line}: {error}")])?;
                QueuedAnalysis {
                    numeric_override: None,
                    spec: AnalysisSpec::Pnoise,
                    config: None,
                    spec_options: SpecExecutionOptions {
                        pnoise: Some(config),
                        ..SpecExecutionOptions::default()
                    },
                    analysis_line: card.source.clone(),
                }
            }
            ".pxf" => {
                let config = parse_pxf(
                    &card,
                    &netlist.params,
                    pss_fundamental,
                    pss_harmonics,
                    pss_tolerance,
                    reltol,
                    abstol,
                )
                .map_err(|error| vec![format!("line {line}: {error}")])?;
                QueuedAnalysis {
                    numeric_override: None,
                    spec: AnalysisSpec::Pxf,
                    config: None,
                    spec_options: SpecExecutionOptions {
                        pxf: Some(config),
                        ..SpecExecutionOptions::default()
                    },
                    analysis_line: card.source.clone(),
                }
            }
            ".pstb" => {
                let config = parse_pstb(
                    &card,
                    &netlist.params,
                    pss_fundamental,
                    pss_harmonics,
                    pss_tolerance,
                )
                .map_err(|error| vec![format!("line {line}: {error}")])?;
                QueuedAnalysis {
                    numeric_override: None,
                    spec: AnalysisSpec::Pstb,
                    config: None,
                    spec_options: SpecExecutionOptions {
                        pstb: Some(config),
                        ..SpecExecutionOptions::default()
                    },
                    analysis_line: card.source.clone(),
                }
            }
            _ => unreachable!("filtered periodic directive"),
        };
        tasks.push(task);
        if let AnalysisSpec::Pss { num_harmonics, .. } = &tasks.last().unwrap().spec
            && *num_harmonics > 0
        {
            tasks.push(QueuedAnalysis {
                numeric_override: None,
                spec: AnalysisSpec::PssSpectrum {
                    num_harmonics: *num_harmonics,
                },
                config: None,
                spec_options: SpecExecutionOptions::default(),
                analysis_line: format!("{} (spectrum)", card.source),
            });
        }
    }
    Ok(tasks)
}

fn parse_pss(card: &ParsedCard, params: &ParamContext) -> Result<AnalysisSpec, String> {
    reject_unknown_keys(
        card,
        &[
            "fund",
            "fundamental",
            "mode",
            "tones",
            "tstab_periods",
            "points_per_period",
            "tolerance",
            "autonomous",
            "oscnode",
            "save_harmonics",
        ],
        ".PSS",
    )?;
    let fundamental = card
        .positional
        .first()
        .or_else(|| card.keyed.get("fund"))
        .or_else(|| card.keyed.get("fundamental"))
        .ok_or_else(|| ".PSS requires a fundamental frequency".to_owned())?;
    let fundamental_freq = numeric_value(fundamental, ".PSS fundamental frequency", params)?;
    let mode = card.keyed.get("mode").map_or("shooting", String::as_str);
    if !mode.eq_ignore_ascii_case("shooting") {
        return Err(".PSS mode must be SHOOTING; HB is a separate .HB analysis".to_owned());
    }
    let oscillator_mode = optional_bool(card, "autonomous", false)?;
    let tone_sources = card
        .keyed
        .get("tones")
        .map(|value| split_names(value))
        .unwrap_or_default();
    let oscillator_node = card
        .keyed
        .get("oscnode")
        .map(|value| unquote(value).trim().to_owned())
        .filter(|value| value != "-");
    let spec = AnalysisSpec::Pss {
        method: PssMethod::Shooting,
        fundamental_freq,
        tone_sources,
        tstab_periods: optional_usize(card, "tstab_periods", 20, params)?,
        points_per_period: optional_usize(card, "points_per_period", 512, params)?,
        tolerance: optional_value(card, "tolerance", 1.0e-7, params)?,
        oscillator_mode,
        oscillator_node,
        num_harmonics: optional_usize(card, "save_harmonics", 20, params)?,
    };
    spec.validate()
        .map_err(|error| format!("invalid .PSS: {error}"))?;
    Ok(spec)
}

#[allow(clippy::too_many_arguments)]
fn parse_pac(
    card: &ParsedCard,
    params: &ParamContext,
    pss_fundamental_freq: f64,
    pss_num_harmonics: usize,
    pss_tolerance: f64,
    reltol: f64,
    abstol: f64,
) -> Result<PacRunConfig, String> {
    reject_unknown_keys(
        card,
        &["maxsideband", "input", "output", "pacmag", "includedc"],
        ".PAC",
    )?;
    let (sweep, points_per_unit, start_freq, stop_freq) = frequency_sweep(card, ".PAC", params)?;
    let input_source = required_text(card, "input", ".PAC")?;
    let (output_node, output_ref) = required_output(card, ".PAC")?;
    let config = PacRunConfig {
        pss_fundamental_freq,
        pss_num_harmonics,
        pss_tolerance,
        start_freq,
        stop_freq,
        points_per_unit,
        sweep: match sweep {
            FrequencySweepKind::Decade => PacFrequencySweep::Decade,
            FrequencySweepKind::Octave => PacFrequencySweep::Octave,
            FrequencySweepKind::Linear => PacFrequencySweep::Linear,
        },
        max_sideband: optional_i32(card, "maxsideband", 5, params)?,
        input_source,
        output_node,
        output_ref,
        pac_magnitude: optional_value(card, "pacmag", 1.0, params)?,
        include_dc: optional_bool(card, "includedc", true)?,
        reltol,
        abstol,
    };
    validate_frequency_contract(
        ".PAC",
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.max_sideband,
    )?;
    if config.input_source.trim().is_empty()
        || config.output_node.trim().is_empty()
        || !config.pac_magnitude.is_finite()
        || config.pac_magnitude <= 0.0
    {
        return Err(
            ".PAC requires non-empty input/output names and a positive PAC magnitude".to_owned(),
        );
    }
    Ok(config)
}

#[allow(clippy::too_many_arguments)]
fn parse_pnoise(
    card: &ParsedCard,
    params: &ParamContext,
    pss_fundamental_freq: f64,
    pss_num_harmonics: usize,
    pss_tolerance: f64,
    reltol: f64,
    abstol: f64,
) -> Result<PnoiseRunConfig, String> {
    reject_unknown_keys(
        card,
        &[
            "maxsideband",
            "input",
            "output",
            "noiseref",
            "integratednoise",
            "noisesummary",
        ],
        ".PNOISE",
    )?;
    let (sweep, points_per_unit, start_freq, stop_freq) = frequency_sweep(card, ".PNOISE", params)?;
    let (output_node, output_ref) = required_output(card, ".PNOISE")?;
    let noise_ref = match card
        .keyed
        .get("noiseref")
        .map_or("output", String::as_str)
        .to_ascii_lowercase()
        .as_str()
    {
        "output" => PnoiseReference::Output,
        "input" => PnoiseReference::Input,
        "phase" => PnoiseReference::Phase,
        value => {
            return Err(format!(
                ".PNOISE noiseref={value:?} must be output, input, or phase"
            ));
        }
    };
    let input_source = card
        .keyed
        .get("input")
        .map(|value| unquote(value).trim().to_owned())
        .unwrap_or_default();
    if noise_ref == PnoiseReference::Input && input_source.is_empty() {
        return Err(".PNOISE noiseref=input requires input=<source>".to_owned());
    }
    let config = PnoiseRunConfig {
        pss_fundamental_freq,
        pss_num_harmonics,
        pss_tolerance,
        start_freq,
        stop_freq,
        points_per_unit,
        sweep: match sweep {
            FrequencySweepKind::Decade => PnoiseFrequencySweep::Decade,
            FrequencySweepKind::Octave => PnoiseFrequencySweep::Octave,
            FrequencySweepKind::Linear => PnoiseFrequencySweep::Linear,
        },
        max_sideband: optional_i32(card, "maxsideband", 5, params)?,
        output_node,
        output_ref,
        input_source,
        noise_ref,
        integrated_noise: optional_bool(card, "integratednoise", false)?,
        noise_summary: optional_bool(card, "noisesummary", true)?,
        reltol,
        abstol,
    };
    validate_frequency_contract(
        ".PNOISE",
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.max_sideband,
    )?;
    Ok(config)
}

#[allow(clippy::too_many_arguments)]
fn parse_pxf(
    card: &ParsedCard,
    params: &ParamContext,
    pss_fundamental_freq: f64,
    pss_num_harmonics: usize,
    pss_tolerance: f64,
    reltol: f64,
    abstol: f64,
) -> Result<PxfRunConfig, String> {
    reject_unknown_keys(
        card,
        &[
            "maxsideband",
            "inputsideband",
            "outsideband",
            "input",
            "output",
        ],
        ".PXF",
    )?;
    let (sweep, points_per_unit, start_freq, stop_freq) = frequency_sweep(card, ".PXF", params)?;
    let input_source = required_text(card, "input", ".PXF")?;
    let (output_node, output_ref) = required_output(card, ".PXF")?;
    let config = PxfRunConfig {
        pss_fundamental_freq,
        pss_num_harmonics,
        pss_tolerance,
        start_freq,
        stop_freq,
        points_per_unit,
        sweep: match sweep {
            FrequencySweepKind::Decade => PxfFrequencySweep::Decade,
            FrequencySweepKind::Octave => PxfFrequencySweep::Octave,
            FrequencySweepKind::Linear => PxfFrequencySweep::Linear,
        },
        input_source,
        input_sideband: optional_i32(card, "inputsideband", 1, params)?,
        output_node,
        output_ref,
        output_sideband: optional_i32(card, "outsideband", 1, params)?,
        max_sideband: optional_i32(card, "maxsideband", 5, params)?,
        reltol,
        abstol,
    };
    validate_frequency_contract(
        ".PXF",
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.max_sideband,
    )?;
    if config.input_sideband.unsigned_abs() > config.max_sideband as u32
        || config.output_sideband.unsigned_abs() > config.max_sideband as u32
    {
        return Err(".PXF input/output sidebands must not exceed maxsideband".to_owned());
    }
    if config
        .output_ref
        .as_deref()
        .is_some_and(|reference| reference.eq_ignore_ascii_case(&config.output_node))
    {
        return Err(".PXF output node and reference node must differ".to_owned());
    }
    Ok(config)
}

fn parse_pstb(
    card: &ParsedCard,
    params: &ParamContext,
    pss_fundamental_freq: f64,
    pss_num_harmonics: usize,
    pss_tolerance: f64,
) -> Result<PstbRunConfig, String> {
    reject_unknown_keys(
        card,
        &[
            "probe",
            "maxharm",
            "nmults",
            "stabilitythreshold",
            "detectsubharmonics",
            "eigentol",
        ],
        ".PSTB",
    )?;
    if !card.positional.is_empty() {
        return Err(".PSTB accepts keyed options only".to_owned());
    }
    let config = PstbRunConfig {
        pss_fundamental_freq,
        pss_num_harmonics,
        pss_tolerance,
        probe_instance: required_text(card, "probe", ".PSTB")?,
        max_harmonics: optional_usize(card, "maxharm", 10, params)?,
        num_multipliers: optional_usize(card, "nmults", 10, params)?,
        stability_threshold: optional_value(card, "stabilitythreshold", 1.0 + 1.0e-6, params)?,
        detect_subharmonics: optional_bool(card, "detectsubharmonics", true)?,
        eigenvalue_tolerance: optional_value(card, "eigentol", 1.0e-10, params)?,
    };
    if config.max_harmonics == 0
        || config.num_multipliers == 0
        || !config.stability_threshold.is_finite()
        || config.stability_threshold <= 0.0
        || !config.eigenvalue_tolerance.is_finite()
        || config.eigenvalue_tolerance <= 0.0
    {
        return Err(
            ".PSTB requires positive maxharm, nmults, stabilitythreshold, and eigentol".to_owned(),
        );
    }
    Ok(config)
}

fn logical_cards(source: &str) -> Vec<(usize, String)> {
    let mut cards = Vec::<(usize, String)>::new();
    for (zero_line, raw) in source.lines().enumerate() {
        let trimmed = raw.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('*')
            || trimmed.starts_with(';')
            || trimmed.starts_with("//")
        {
            continue;
        }
        if let Some(continuation) = trimmed.strip_prefix('+') {
            if let Some((_, card)) = cards.last_mut() {
                card.push(' ');
                card.push_str(continuation.trim());
            }
        } else {
            cards.push((zero_line + 1, trimmed.to_owned()));
        }
    }
    cards
}

fn parse_card(source: &str) -> Result<ParsedCard, String> {
    let tokens = source.split_whitespace().collect::<Vec<_>>();
    let mut positional = Vec::new();
    let mut keyed = HashMap::new();
    let mut index = 1usize;
    while index < tokens.len() {
        let token = tokens[index].trim_end_matches(',');
        if let Some((key, value)) = token.split_once('=') {
            if key.is_empty() {
                return Err(format!("invalid assignment token {token:?}"));
            }
            let value = if value.is_empty() {
                index += 1;
                tokens
                    .get(index)
                    .ok_or_else(|| format!("{key}= requires a value"))?
            } else {
                value
            };
            insert_key(&mut keyed, key, value)?;
        } else if tokens.get(index + 1) == Some(&"=") {
            let value = tokens
                .get(index + 2)
                .ok_or_else(|| format!("{token} = requires a value"))?;
            insert_key(&mut keyed, token, value)?;
            index += 2;
        } else {
            positional.push(token.to_owned());
        }
        index += 1;
    }
    Ok(ParsedCard {
        source: source.to_owned(),
        positional,
        keyed,
    })
}

fn insert_key(keyed: &mut HashMap<String, String>, key: &str, value: &str) -> Result<(), String> {
    let key = key.trim().to_ascii_lowercase();
    if keyed
        .insert(key.clone(), value.trim().trim_end_matches(',').to_owned())
        .is_some()
    {
        return Err(format!("duplicate option {key:?}"));
    }
    Ok(())
}

fn reject_unknown_keys(
    card: &ParsedCard,
    accepted: &[&str],
    directive: &str,
) -> Result<(), String> {
    let accepted = accepted.iter().copied().collect::<HashSet<_>>();
    if let Some(key) = card
        .keyed
        .keys()
        .find(|key| !accepted.contains(key.as_str()))
    {
        return Err(format!("{directive} does not support option {key:?}"));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum FrequencySweepKind {
    Decade,
    Octave,
    Linear,
}

fn frequency_sweep(
    card: &ParsedCard,
    directive: &str,
    params: &ParamContext,
) -> Result<(FrequencySweepKind, usize, f64, f64), String> {
    if card.positional.len() != 4 {
        return Err(format!(
            "{directive} requires SWEEP POINTS START STOP before keyed options"
        ));
    }
    let sweep = match card.positional[0].to_ascii_lowercase().as_str() {
        "dec" => FrequencySweepKind::Decade,
        "oct" => FrequencySweepKind::Octave,
        "lin" => FrequencySweepKind::Linear,
        value => {
            return Err(format!(
                "{directive} sweep {value:?} must be DEC, OCT, or LIN"
            ));
        }
    };
    let points = usize_value(&card.positional[1], &format!("{directive} points"), params)?;
    let start = numeric_value(
        &card.positional[2],
        &format!("{directive} start frequency"),
        params,
    )?;
    let stop = numeric_value(
        &card.positional[3],
        &format!("{directive} stop frequency"),
        params,
    )?;
    Ok((sweep, points, start, stop))
}

fn validate_frequency_contract(
    directive: &str,
    start: f64,
    stop: f64,
    points: usize,
    max_sideband: i32,
) -> Result<(), String> {
    if !start.is_finite() || !stop.is_finite() || start <= 0.0 || stop <= start {
        return Err(format!(
            "{directive} frequencies must be finite and satisfy 0 < START < STOP"
        ));
    }
    if points == 0 || max_sideband < 0 {
        return Err(format!(
            "{directive} points must be positive and maxsideband must be non-negative"
        ));
    }
    Ok(())
}

fn required_text(card: &ParsedCard, key: &str, directive: &str) -> Result<String, String> {
    card.keyed
        .get(key)
        .map(|value| unquote(value).trim().to_owned())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{directive} requires {key}=<name>"))
}

fn required_output(card: &ParsedCard, directive: &str) -> Result<(String, Option<String>), String> {
    let raw = required_text(card, "output", directive)?;
    let inner = raw
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .unwrap_or(&raw);
    let nodes = inner.split(',').map(str::trim).collect::<Vec<_>>();
    match nodes.as_slice() {
        [node] if !node.is_empty() => Ok(((*node).to_owned(), None)),
        [node, reference] if !node.is_empty() && !reference.is_empty() => {
            Ok(((*node).to_owned(), Some((*reference).to_owned())))
        }
        _ => Err(format!(
            "{directive} output must be NODE or (NODE,REFERENCE)"
        )),
    }
}

fn optional_value(
    card: &ParsedCard,
    key: &str,
    default: f64,
    params: &ParamContext,
) -> Result<f64, String> {
    card.keyed
        .get(key)
        .map_or(Ok(default), |value| numeric_value(value, key, params))
}

fn optional_usize(
    card: &ParsedCard,
    key: &str,
    default: usize,
    params: &ParamContext,
) -> Result<usize, String> {
    card.keyed
        .get(key)
        .map_or(Ok(default), |value| usize_value(value, key, params))
}

fn optional_i32(
    card: &ParsedCard,
    key: &str,
    default: i32,
    params: &ParamContext,
) -> Result<i32, String> {
    card.keyed
        .get(key)
        .map_or(Ok(default), |value| i32_value(value, key, params))
}

fn optional_bool(card: &ParsedCard, key: &str, default: bool) -> Result<bool, String> {
    card.keyed.get(key).map_or(Ok(default), |value| {
        match unquote(value).to_ascii_lowercase().as_str() {
            "yes" | "true" | "on" | "1" => Ok(true),
            "no" | "false" | "off" | "0" => Ok(false),
            _ => Err(format!("{key} must be yes/no, true/false, on/off, or 1/0")),
        }
    })
}

fn numeric_value(value: &str, label: &str, params: &ParamContext) -> Result<f64, String> {
    let value = unquote(value);
    let expression = value.trim_matches(['{', '}']);
    rspice_core::netlist::lexer::parse_spice_value_complete(expression)
        .or_else(|_| eval_expression(expression, params))
        .map_err(|error| format!("{label} has invalid numeric expression {value:?}: {error}"))
}

fn usize_value(value: &str, label: &str, params: &ParamContext) -> Result<usize, String> {
    let value = numeric_value(value, label, params)?;
    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 || value > usize::MAX as f64 {
        return Err(format!("{label} must be a non-negative integer"));
    }
    Ok(value as usize)
}

fn i32_value(value: &str, label: &str, params: &ParamContext) -> Result<i32, String> {
    let value = numeric_value(value, label, params)?;
    if !value.is_finite()
        || value.fract() != 0.0
        || value < i32::MIN as f64
        || value > i32::MAX as f64
    {
        return Err(format!("{label} must be a 32-bit integer"));
    }
    Ok(value as i32)
}

fn split_names(value: &str) -> Vec<String> {
    unquote(value)
        .split([',', ';'])
        .flat_map(str::split_whitespace)
        .map(str::trim)
        .filter(|value| !value.is_empty() && *value != "-")
        .map(str::to_owned)
        .collect()
}

fn unquote(value: &str) -> &str {
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .unwrap_or(value)
}

fn matches_ignore_ascii_case(value: &str, candidates: &[&str]) -> bool {
    candidates
        .iter()
        .any(|candidate| value.eq_ignore_ascii_case(candidate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn periodic_cards_accept_continuations_and_freeze_exact_options() {
        let source = "periodic\n.param F0=1Meg NH=12 NPTS=20\nV1 in 0 SIN(0 1 {F0})\nR1 in out 1k\nC1 out 0 1n\n\
.pss {F0} tones=V1 tstab_periods=8 points_per_period=128 tolerance=1e-6 save_harmonics={NH}\n\
.pac dec {NPTS} 1k 100Meg maxsideband=7 input=V1 output=(out,0)\n\
.pnoise dec 10 1 1Meg output=out maxsideband=9\n+ noiseref=phase noiseSummary=no\n\
.pxf dec 15 1k 10Meg input=V1 output=out inputsideband=-1 outsideband=2 maxsideband=5\n\
.pstb probe=LPROBE maxharm=8 nmults=6 stabilitythreshold=1.0001 detectsubharmonics=no eigentol=1e-9\n.end\n";
        let netlist = Netlist::parse(source).unwrap();

        let tasks = parse_periodic_tasks(&netlist, source).unwrap();

        assert_eq!(tasks.len(), 6);
        assert!(matches!(tasks[0].spec, AnalysisSpec::Pss { .. }));
        assert!(matches!(
            tasks[1].spec,
            AnalysisSpec::PssSpectrum { num_harmonics: 12 }
        ));
        let pac = tasks[2].spec_options.pac.as_ref().unwrap();
        assert_eq!(pac.points_per_unit, 20);
        assert_eq!(pac.max_sideband, 7);
        assert_eq!(pac.output_ref.as_deref(), Some("0"));
        let pnoise = tasks[3].spec_options.pnoise.as_ref().unwrap();
        assert_eq!(pnoise.max_sideband, 9);
        assert_eq!(pnoise.noise_ref, PnoiseReference::Phase);
        assert!(!pnoise.noise_summary);
        let pxf = tasks[4].spec_options.pxf.as_ref().unwrap();
        assert_eq!(pxf.input_sideband, -1);
        assert_eq!(pxf.output_sideband, 2);
        let pstb = tasks[5].spec_options.pstb.as_ref().unwrap();
        assert_eq!(pstb.probe_instance, "LPROBE");
        assert_eq!(pstb.num_multipliers, 6);
        assert!(!pstb.detect_subharmonics);
    }

    #[test]
    fn periodic_dependents_fail_closed_without_pss() {
        let source = "periodic\nV1 in 0 0 AC 1\nR1 in out 1k\n.pac dec 10 1k 1Meg input=V1 output=out\n.end\n";
        let netlist = Netlist::parse(source).unwrap();

        let errors = parse_periodic_tasks(&netlist, source).unwrap_err();

        assert!(
            errors
                .iter()
                .any(|error| error.contains("require one .PSS"))
        );
    }
}

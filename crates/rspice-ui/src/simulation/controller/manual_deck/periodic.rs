//! Periodic-analysis directives authored in a manual deck.
//!
//! The Netlist workspace executes the periodic family through the UI's typed
//! analysis pipeline: this reader accepts the exact source card, validates
//! every operand, and freezes all execution options on the queued task.
//! Nothing is borrowed from the Simulation Studio dialogs.
//!
//! There is one grammar, and it is the engine's. `.PSS`, `.PAC` and `.PNOISE`
//! are cards the netlist parser owns
//! (`rspice-core/src/netlist/parser/periodic_cards.rs`), so a manual deck is
//! parsed by that parser before it reaches this one and every malformed card
//! is refused there, in the engine's own words, with the engine's own line
//! number. What is left for this reader is the part the engine's AST does not
//! carry: binding each dependent card to the deck's one `.PSS` operating
//! point, and turning the cards into the typed run configurations the studio
//! dispatches.
//!
//! So the key sets below are the engine's key sets, not a second dialect. Two
//! consequences follow, and both are stated rather than absorbed:
//!
//! - A key the engine accepts but the studio's typed run configuration has no
//!   field for is refused by name — [`PSS_KEYS_THE_STUDIO_CANNOT_HONOUR`] —
//!   rather than parsed and dropped. That is the rule `periodic_cards.rs`
//!   states for the ngspice fields RSpice cannot honour, applied one layer up.
//! - `.PXF` and `.PSTB` are *not* engine cards: the parser records them as
//!   unsupported dot-commands and ignores them, so this reader owns their
//!   whole grammar. They spell their output probe `out=` like the rest of the
//!   family.

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
            parse_pss(card, &netlist.params, || driven_tone_sources(netlist))
                .map_err(|error| format!("line {line}: {error}"))
        })
        .transpose()
        .map_err(|error| vec![error])?;
    // A dependent card without a `.PSS` was refused above, and a `.PSS` that
    // retains no harmonic is refused by `parse_pss` — the engine's card has no
    // `HARMS=0` — so the fallbacks here are only reached when the deck holds
    // no periodic analysis at all, and nothing reads them.
    let (pss_fundamental, pss_harmonics, pss_tolerance) = match &pss_spec {
        Some(AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
            ..
        }) => (*fundamental_freq, *num_harmonics, *tolerance),
        _ => (0.0, 0, 0.0),
    };
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
                spec: pss_spec.clone().ok_or_else(|| {
                    vec![format!(
                        "line {line}: the parsed .PSS directive lost its authenticated analysis specification"
                    )]
                })?,
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
            _ => {
                return Err(vec![format!(
                    "line {line}: unsupported periodic directive {head} reached task construction"
                )]);
            }
        };
        let pss_spectrum_harmonics = match &task.spec {
            AnalysisSpec::Pss { num_harmonics, .. } if *num_harmonics > 0 => Some(*num_harmonics),
            _ => None,
        };
        tasks.push(task);
        if let Some(num_harmonics) = pss_spectrum_harmonics {
            tasks.push(QueuedAnalysis {
                numeric_override: None,
                spec: AnalysisSpec::PssSpectrum { num_harmonics },
                config: None,
                spec_options: SpecExecutionOptions::default(),
                analysis_line: format!("{} (spectrum)", card.source),
            });
        }
    }
    Ok(tasks)
}

/// The complete elaborated set of time-varying sources the deck drives.
///
/// The engine's `.PSS` card carries no tone list, and it needs none:
/// [`Engine::validate_periodic_source_contract`] accepts exactly the deck's
/// complete elaborated source set and refuses any proper subset, so the deck's
/// own sources *are* the tone list. Reading them here is the same resolution
/// the compatibility PSS entry point makes for the same reason
/// (`services::simulation_runner::pss::run_pss_analysis_with_source_path_and_abort`).
///
/// [`Engine::validate_periodic_source_contract`]: rspice_core::Engine::validate_periodic_source_contract
fn driven_tone_sources(netlist: &Netlist) -> Result<Vec<String>, String> {
    rspice_core::Engine::new(rspice_core::SimulationConfig::default())
        .transient_source_names(netlist)
        .map_err(|error| format!("the deck's periodic sources could not be read: {error}"))
}

/// `.PSS` keywords the engine accepts that the studio's typed request cannot
/// hold, and what the studio does instead.
///
/// The engine's card carries the whole shooting configuration; the studio's
/// `PssRunConfig` carries the seven fields its form owns. Each entry is
/// refused by name rather than parsed and dropped, so a deck is never accepted
/// under a configuration the run will not honour.
const PSS_KEYS_THE_STUDIO_CANNOT_HONOUR: &[(&str, &str)] = &[
    (
        "periodguess",
        "the studio seeds an autonomous period from the fundamental; author fund=",
    ),
    (
        "tstab",
        "the studio's stabilization window is a period count; author tstabperiods=",
    ),
    (
        "maxiter",
        "the studio's shooting run is fixed at 100 Newton iterations",
    ),
    (
        "abstol",
        "the studio's shooting run converges on the relative periodicity norm alone; author tol=",
    ),
    (
        "damping",
        "the studio's shooting run takes the engine's default Newton damping",
    ),
    (
        "maxperiodchange",
        "the studio's autonomous run takes the engine's default relative period bound",
    ),
    (
        "method",
        "the studio's shooting run takes the engine's default integration method",
    ),
    (
        "verbose",
        "the studio does not route solver logging through the deck",
    ),
];

/// Read `.PSS FUND=<hz> [KEY=VALUE ...]`, the engine's keyword form.
///
/// Every default here is the engine's card default, so a key the deck omits
/// means the same thing to both readers.
///
/// The tone list is resolved from the deck rather than from the card, because
/// the card has no tone field; `driven_tone_sources` says why that is the
/// complete answer rather than a guess. It is only asked for on a driven card:
/// an autonomous solve takes its period from the oscillator node and reads no
/// tone list at all.
fn parse_pss(
    card: &ParsedCard,
    params: &ParamContext,
    driven_tone_sources: impl FnOnce() -> Result<Vec<String>, String>,
) -> Result<AnalysisSpec, String> {
    if !card.positional.is_empty() {
        return Err(
            ".PSS in a manual deck is the keyword card `.PSS fund=<frequency> [key=value ...]`; \
             ngspice's positional oscillator card carries a stabilization time in seconds and a \
             shooting-iteration limit the studio's periodic pipeline does not hold"
                .to_owned(),
        );
    }
    reject_unsupported_keys(
        card,
        &[
            "fund",
            "autonomous",
            "oscnode",
            "harms",
            "tstabperiods",
            "points",
            "tol",
        ],
        PSS_KEYS_THE_STUDIO_CANNOT_HONOUR,
        ".PSS",
    )?;
    let fundamental = card
        .keyed
        .get("fund")
        .ok_or_else(|| ".PSS requires fund=<frequency>".to_owned())?;
    let fundamental_freq = numeric_value(fundamental, ".PSS fund", params)?;
    let oscillator_node = card
        .keyed
        .get("oscnode")
        .map(|value| unquote(value).trim().to_owned())
        .filter(|value| !value.is_empty());
    let authored_autonomous = card
        .keyed
        .contains_key("autonomous")
        .then(|| optional_bool(card, "autonomous", false))
        .transpose()?;
    // The engine's rule, and its refusal: naming an oscillator node is itself
    // a request for period detection, so `autonomous=no` beside one is two
    // cards at once rather than one.
    if authored_autonomous == Some(false) && oscillator_node.is_some() {
        return Err(".PSS oscnode= and autonomous=no state opposite solves".to_owned());
    }
    let oscillator_mode = authored_autonomous.unwrap_or(false) || oscillator_node.is_some();
    let tone_sources = if oscillator_mode {
        Vec::new()
    } else {
        driven_tone_sources()?
    };
    // Detecting a period needs more startup than tracking a known one, which
    // is why the engine's card defaults the two modes apart.
    let default_tstab_periods = if oscillator_mode { 20 } else { 10 };
    let tstab_periods = optional_usize(card, "tstabperiods", default_tstab_periods, params)?;
    let num_harmonics = optional_usize(card, "harms", 9, params)?;
    // Two bounds the typed specification does not carry, because a spec built
    // from a form cannot reach them: `harms=0` is the studio's own "retain no
    // spectrum", which is not a thing the card can ask for, and a zero-period
    // stabilization window is not a thing the card can ask for either.
    if num_harmonics == 0 {
        return Err(".PSS harms must be at least 1".to_owned());
    }
    if tstab_periods == 0 {
        return Err(".PSS tstabperiods must be at least 1".to_owned());
    }
    let spec = AnalysisSpec::Pss {
        method: PssMethod::Shooting,
        fundamental_freq,
        tone_sources,
        tstab_periods,
        points_per_period: optional_usize(card, "points", 256, params)?,
        tolerance: optional_value(card, "tol", 1.0e-6, params)?,
        oscillator_mode,
        oscillator_node,
        num_harmonics,
    };
    spec.validate()
        .map_err(|error| format!("invalid .PSS: {error}"))?;
    Ok(spec)
}

fn parse_pac(
    card: &ParsedCard,
    params: &ParamContext,
    pss_fundamental_freq: f64,
    pss_num_harmonics: usize,
    pss_tolerance: f64,
    reltol: f64,
    abstol: f64,
) -> Result<PacRunConfig, String> {
    reject_unsupported_keys(
        card,
        &[
            "input",
            "out",
            "maxsideband",
            "sidebandmin",
            "sidebandmax",
            "reltol",
            "abstol",
            "from",
        ],
        &[],
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
        max_sideband: sideband_bound(card, ".PAC", 5, params)?,
        input_source,
        output_node,
        output_ref,
        // The engine's `.PAC` has no field for either, so neither is authored:
        // both reach a studio-configured run through the typed execution
        // options instead, and a manual deck takes the same values the direct
        // periodic AC entry point defaults to.
        pac_magnitude: 1.0,
        include_dc: true,
        // The card may state the frequency-domain tolerances; a deck that does
        // not falls back to its own `.options`, which is a sharper answer than
        // the card constant and the one this reader has always given.
        reltol: optional_value(card, "reltol", reltol, params)?,
        abstol: optional_value(card, "abstol", abstol, params)?,
    };
    periodic_source_selector(card, ".PAC")?;
    validate_frequency_contract(
        ".PAC",
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.max_sideband,
    )?;
    if !config.reltol.is_finite()
        || config.reltol <= 0.0
        || !config.abstol.is_finite()
        || config.abstol <= 0.0
    {
        return Err(".PAC reltol and abstol must be finite and positive".to_owned());
    }
    Ok(config)
}

fn parse_pnoise(
    card: &ParsedCard,
    params: &ParamContext,
    pss_fundamental_freq: f64,
    pss_num_harmonics: usize,
    pss_tolerance: f64,
    reltol: f64,
    abstol: f64,
) -> Result<PnoiseRunConfig, String> {
    reject_unsupported_keys(
        card,
        &["out", "input", "maxsideband", "from"],
        &[],
        ".PNOISE",
    )?;
    let (sweep, points_per_unit, start_freq, stop_freq) = frequency_sweep(card, ".PNOISE", params)?;
    let (output_node, output_ref) = required_output(card, ".PNOISE")?;
    let input_source = card
        .keyed
        .get("input")
        .map(|value| unquote(value).trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_default();
    // The engine's card has one field for the noise reference and it is
    // `INPUT=`, described there as "the independent source used for
    // input-referred noise, when authored". A card that names one asks for
    // input-referred noise; a card that does not asks for output-referred.
    // Phase noise has no spelling on the card and so no manual-deck route: a
    // studio-configured PNoise still selects it, through the typed execution
    // options rather than through the deck.
    let noise_ref = if input_source.is_empty() {
        PnoiseReference::Output
    } else {
        PnoiseReference::Input
    };
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
        // The engine's own default folded bound for a card that does not say.
        max_sideband: optional_i32(card, "maxsideband", 6, params)?,
        output_node,
        output_ref,
        input_source,
        noise_ref,
        // Neither has a field on the engine's card. Both keep the value the
        // direct periodic-noise entry point defaults to, and both remain
        // authorable from the studio through the typed execution options.
        integrated_noise: false,
        noise_summary: true,
        reltol,
        abstol,
    };
    periodic_source_selector(card, ".PNOISE")?;
    if config.max_sideband < 1 {
        return Err(".PNOISE maxsideband must be at least 1".to_owned());
    }
    validate_frequency_contract(
        ".PNOISE",
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.max_sideband,
    )?;
    Ok(config)
}

fn parse_pxf(
    card: &ParsedCard,
    params: &ParamContext,
    pss_fundamental_freq: f64,
    pss_num_harmonics: usize,
    pss_tolerance: f64,
    reltol: f64,
    abstol: f64,
) -> Result<PxfRunConfig, String> {
    reject_unsupported_keys(
        card,
        &[
            "maxsideband",
            "inputsideband",
            "outsideband",
            "input",
            "out",
        ],
        &[],
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
    reject_unsupported_keys(
        card,
        &[
            "probe",
            "maxharm",
            "nmults",
            "stabilitythreshold",
            "detectsubharmonics",
            "eigentol",
        ],
        &[],
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

/// Refuse every key the card carries that this reader does not honour.
///
/// `unhonourable` names keys the *engine* accepts on the same card but whose
/// value the studio's typed run configuration has no field for. They are
/// refused with the reason and the remedy rather than with the generic
/// "unknown option", because a deck that carries one is well formed — it is
/// asking for something this pipeline would silently not do.
fn reject_unsupported_keys(
    card: &ParsedCard,
    accepted: &[&str],
    unhonourable: &[(&str, &str)],
    directive: &str,
) -> Result<(), String> {
    for (key, reason) in unhonourable {
        if card.keyed.contains_key(*key) {
            return Err(format!(
                "{directive} {key}= is not honoured by the studio's periodic pipeline: {reason}"
            ));
        }
    }
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

/// Read the engine's `FROM=PSS|HB` selector.
///
/// A manual deck binds its dependent cards to the one `.PSS` the deck is
/// required to carry, so `FROM=PSS` is the selector the run already
/// implements and `FROM=HB` names a binding this pipeline does not have.
fn periodic_source_selector(card: &ParsedCard, directive: &str) -> Result<(), String> {
    let Some(value) = card.keyed.get("from") else {
        return Ok(());
    };
    match unquote(value).trim().to_ascii_lowercase().as_str() {
        "pss" => Ok(()),
        "hb" => Err(format!(
            "{directive} from=hb has no manual-deck route: a manual-deck periodic analysis binds \
             to the .PSS operating point in the same deck"
        )),
        other => Err(format!("{directive} from={other:?} must be PSS")),
    }
}

/// Read the sideband bound the card states, in either of the engine's two
/// spellings.
///
/// `MAXSIDEBAND=n` is the symmetric range `-n..=n`, and the studio's typed
/// periodic configurations hold exactly that one number. `SIDEBANDMIN`/
/// `SIDEBANDMAX` can state an asymmetric range, which is refused rather than
/// widened to the enclosing symmetric one: a run over sidebands the deck did
/// not ask for is not the analysis that was authored.
fn sideband_bound(
    card: &ParsedCard,
    directive: &str,
    default: i32,
    params: &ParamContext,
) -> Result<i32, String> {
    let minimum = card
        .keyed
        .contains_key("sidebandmin")
        .then(|| i32_value_for(card, "sidebandmin", params))
        .transpose()?;
    let maximum = card
        .keyed
        .contains_key("sidebandmax")
        .then(|| i32_value_for(card, "sidebandmax", params))
        .transpose()?;
    let symmetric = card
        .keyed
        .contains_key("maxsideband")
        .then(|| i32_value_for(card, "maxsideband", params))
        .transpose()?;

    match (symmetric, minimum, maximum) {
        (Some(_), Some(_), _) | (Some(_), _, Some(_)) => Err(format!(
            "{directive} states both maxsideband= and sidebandmin=/sidebandmax="
        )),
        (Some(bound), None, None) => Ok(bound),
        (None, None, None) => Ok(default),
        (None, minimum, maximum) => {
            let minimum = minimum.unwrap_or(-default);
            let maximum = maximum.unwrap_or(default);
            if minimum > maximum {
                return Err(format!("{directive} sidebandmin= exceeds sidebandmax="));
            }
            if minimum != -maximum {
                return Err(format!(
                    "{directive} runs a symmetric sideband range; author maxsideband={maximum} \
                     rather than sidebandmin={minimum} sidebandmax={maximum}"
                ));
            }
            Ok(maximum)
        }
    }
}

fn i32_value_for(card: &ParsedCard, key: &str, params: &ParamContext) -> Result<i32, String> {
    let value = card
        .keyed
        .get(key)
        .ok_or_else(|| format!("{key}= requires a value"))?;
    i32_value(value, key, params)
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

/// Read `out=`, the engine's output probe.
///
/// The engine's `card_output_probe` accepts `V(node)`, `V(node,ref)` or a bare
/// node name, so those are what this accepts.
fn required_output(card: &ParsedCard, directive: &str) -> Result<(String, Option<String>), String> {
    let raw = required_text(card, "out", directive)?;
    let probe = raw
        .strip_prefix('V')
        .or_else(|| raw.strip_prefix('v'))
        .unwrap_or(&raw);
    let inner = probe
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
            "{directive} out must be a node name, V(node), or V(node,reference)"
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

    /// Every card here is written in the engine's grammar, and the engine
    /// parses this exact source first: `Netlist::parse` is not a fixture step,
    /// it is the same parse the run makes, so a card spelled in any other
    /// dialect would fail before this reader saw it.
    #[test]
    fn periodic_cards_accept_continuations_and_freeze_exact_options() {
        let source = "periodic\n.param F0=1Meg NH=12 NPTS=20\nV1 in 0 SIN(0 1 {F0})\nR1 in out 1k\nC1 out 0 1n\n\
.pss fund={F0} tstabperiods=8 points=128 tol=1e-6 harms={NH}\n\
.pac dec {NPTS} 1k 100Meg maxsideband=7 input=V1 out=V(out,0)\n\
.pnoise dec 10 1 1Meg out=out maxsideband=9\n+ input=V1 from=pss\n\
.pxf dec 15 1k 10Meg input=V1 out=out inputsideband=-1 outsideband=2 maxsideband=5\n\
.pstb probe=LPROBE maxharm=8 nmults=6 stabilitythreshold=1.0001 detectsubharmonics=no eigentol=1e-9\n.end\n";
        let netlist = Netlist::parse(source).unwrap();

        let tasks = parse_periodic_tasks(&netlist, source).unwrap();

        assert_eq!(tasks.len(), 6);
        let AnalysisSpec::Pss {
            ref tone_sources,
            tstab_periods,
            points_per_period,
            ..
        } = tasks[0].spec
        else {
            panic!("the first task is the PSS: {:?}", tasks[0].spec);
        };
        assert_eq!(tstab_periods, 8);
        assert_eq!(points_per_period, 128);
        // The card names no tones because the engine's card has no tone field.
        // The deck's own one driven source is the complete set, which is
        // exactly what the periodic source contract will demand at preflight.
        assert_eq!(tone_sources, &["V1".to_owned()]);
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
        // `INPUT=` is the card's one noise-reference field, and naming a source
        // is what asks for input-referred noise.
        assert_eq!(pnoise.noise_ref, PnoiseReference::Input);
        assert_eq!(pnoise.input_source, "V1");
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
        let source =
            "periodic\nV1 in 0 0 AC 1\nR1 in out 1k\n.pac dec 10 1k 1Meg input=V1 out=out\n.end\n";
        let netlist = Netlist::parse(source).unwrap();

        let errors = parse_periodic_tasks(&netlist, source).unwrap_err();

        assert!(
            errors
                .iter()
                .any(|error| error.contains("require one .PSS"))
        );
    }

    /// The studio and the engine refuse the same cards.
    ///
    /// The reader above proves the two agree on what is accepted. This proves
    /// they agree on what is not — which is the half that decides whether a
    /// deck can be authored at all, because a card only one of them refuses
    /// either stops a runnable deck or admits one the engine will reject at
    /// preparation. Each card is put through the engine's parser and this
    /// reader, and both must say no.
    #[test]
    fn a_periodic_card_the_engine_refuses_is_a_card_the_studio_refuses() {
        const CIRCUIT: &str = "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        const SEED: &str = ".pss fund=1Meg\n";

        for card in [
            // Keywords neither reader knows.
            ".pss fund=1Meg errpreset=conservative",
            ".pss fund=1Meg tones=V1",
            ".pac dec 10 1k 1Meg input=V1 out=out sidebands=3",
            ".pnoise dec 10 1 1Meg out=out noisetype=pm",
            // Required fields neither reader defaults.
            ".pss harms=9",
            ".pac dec 10 1k 1Meg out=out",
            ".pac dec 10 1k 1Meg input=V1",
            ".pnoise dec 10 1 1Meg",
            // Values outside the range the card states.
            ".pss fund=1Meg harms=0",
            ".pss fund=1Meg points=8",
            ".pss fund=1Meg harms=9 points=16",
            ".pnoise dec 10 1 1Meg out=out maxsideband=0",
            ".pac lin 10 1Meg 1k input=V1 out=out",
            // Two spellings of one quantity on one card.
            ".pac dec 10 1k 1Meg input=V1 out=out maxsideband=2 sidebandmin=-1",
            ".pss fund=1Meg oscnode=out autonomous=no",
        ] {
            let seed = if card.starts_with(".pss") { "" } else { SEED };
            let source = format!("{CIRCUIT}{seed}{card}\n.end\n");
            let engine = Netlist::parse(&source);
            assert!(
                engine.is_err(),
                "the engine must refuse `{card}`; this case no longer tests what it claims"
            );
            let studio = match Netlist::parse(&format!("{CIRCUIT}{seed}.end\n")) {
                Ok(netlist) => parse_periodic_tasks(&netlist, &source),
                Err(error) => panic!("the fixture circuit must parse: {error}"),
            };
            assert!(
                studio.is_err(),
                "the studio accepted `{card}`, which the engine refuses: {studio:?}"
            );
        }
    }
}

//! Periodic-analysis directives authored in a manual deck.
//!
//! The Netlist workspace executes the periodic family through the UI's typed
//! analysis pipeline: this reader accepts the exact source card, validates
//! every operand, and freezes all execution options on the queued task.
//! Nothing is borrowed from the Simulation Studio dialogs.
//!
//! There is one grammar, and it is the engine's. All five of `.PSS`, `.PAC`,
//! `.PNOISE`, `.PXF` and `.PSTB` are cards the netlist parser owns
//! (`rspice-core/src/netlist/parser/periodic_cards.rs`), so a manual deck is
//! parsed by that parser before it reaches this one and every malformed card
//! is refused there, in the engine's own words, with the engine's own line
//! number. What is left for this reader is the part the engine's AST does not
//! carry: binding each dependent card to the deck's one `.PSS` operating
//! point, and turning the cards into the typed run configurations the studio
//! dispatches.
//!
//! So the key sets below are the engine's key sets, not a second dialect: a
//! key the engine's card accepts is a key this reader accepts, at the same
//! default. `.PXF` used to refuse `RELTOL=`, `ABSTOL=` and `FROM=` by name
//! while the engine's parser took all three — and since the deck is parsed by
//! the engine before it reaches this reader, that refusal fired on a deck the
//! engine had already read. The two key sets are asserted equal by
//! `the_reader_and_the_engine_accept_the_same_periodic_key_set` and, for
//! `.PSS`, against the engine's own keyword arms by
//! `the_reader_and_the_engine_accept_the_same_pss_key_set`.
//!
//! `.PSS` carried the last exception and no longer does. Eight of the engine's
//! keywords were refused here by name, with a reason and a remedy, because the
//! studio's typed request had no field for them: the integration method, the
//! stabilization time, the Newton limit, the absolute tolerance, the damping
//! factor, the period bound, the period guess and the solver log. The request
//! holds all of them now, so the table and the parameter that carried it are
//! gone.
//!
//! `.PXF` and `.PSTB` were the exception until the engine gained cards for
//! them: the parser used to record both as unsupported dot-commands, warn that
//! whatever they requested would not run, and ignore them, so this reader
//! owned their whole grammar. It no longer does. They spell their output probe
//! `out=` like the rest of the family, because the engine does.

use std::collections::{HashMap, HashSet};

use rspice_core::netlist::expr::eval_expression;
use rspice_core::netlist::{Netlist, ParamContext};

use super::*;
// Aliased: this module's own `PeriodicCarrier` is the *solved basis* a
// dependent card binds to — four numbers off the deck's one `.PSS` — while the
// runner's is the `FROM=` selector that says which solve that is. Two
// different facts about one carrier, and a reader of this file needs to see
// which one each site means.
use crate::services::simulation_runner::PeriodicCarrier as CarrierSelector;
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

/// What the deck's one `.PSS` card settled, as every dependent card needs it.
///
/// The four numbers travel together because they describe one operating point:
/// splitting them across argument lists let a dependent card be bound to three
/// of them and not the fourth, which is how `noiseref=phase` used to reach a
/// driven carrier.
#[derive(Debug, Clone, Copy, Default)]
struct PeriodicCarrier {
    fundamental_freq: f64,
    num_harmonics: usize,
    tolerance: f64,
    /// Whether the period is a solver unknown rather than authored input.
    autonomous: bool,
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

    // The carrier selector is read before the `.PSS` precondition below, and
    // not inside each card's parser, because a card that names a
    // harmonic-balance carrier is not a card that forgot its `.PSS`. Read in
    // the old order, the deck the engine actually runs -- an `.HB` with a
    // `FROM=HB` card and no `.PSS` at all -- was refused here for the one
    // reason that is untrue of it, while the same card beside a `.PSS` was
    // refused for the real one. One card, two answers, neither the whole
    // story. `.PSTB` is deliberately absent: it has no `FROM=` key in either
    // reader, so `from=` on one is an unknown keyword and stays one.
    for (line, head, card) in &parsed {
        if matches!(head.as_str(), ".pac" | ".pnoise" | ".pxf")
            && let Err(error) = periodic_source_selector(card, &head.to_ascii_uppercase())
        {
            errors.push(format!("line {line}: {error}"));
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
    let carrier = match &pss_spec {
        Some(AnalysisSpec::Pss {
            fundamental_freq,
            num_harmonics,
            tolerance,
            oscillator_mode,
            ..
        }) => PeriodicCarrier {
            fundamental_freq: *fundamental_freq,
            num_harmonics: *num_harmonics,
            tolerance: *tolerance,
            autonomous: *oscillator_mode,
        },
        _ => PeriodicCarrier::default(),
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
                    carrier,
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
                    carrier,
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
                    carrier,
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
                    carrier,
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

/// Every `.PSS` keyword this reader accepts.
///
/// The engine's key set, key for key, asserted equal to it by
/// [`tests::the_reader_and_the_engine_accept_the_same_pss_key_set`], which
/// reads the engine's own keyword arms out of `periodic_cards.rs` rather than
/// trusting this list to have been kept up to date.
///
/// It was a shorter list beside a refusal table: eight of the engine's
/// keywords were rejected by name because the studio's typed request had no
/// field for them. It has fields for all of them now.
const PSS_KEYS: &[&str] = &[
    "fund",
    "periodguess",
    "autonomous",
    "oscnode",
    "harms",
    "tstab",
    "tstabperiods",
    "maxiter",
    "tol",
    "abstol",
    "damping",
    "maxperiodchange",
    "points",
    "method",
    "verbose",
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
///
/// `PERIODGUESS=` is not a field of its own here, for the reason the engine's
/// own card gives: it refuses `FUND=` and `PERIODGUESS=` on one card, and an
/// autonomous card's period guess *is* its fundamental — `parse_pss_command`
/// sets each from the other, and `PssConfig::with_period_guess` does the same.
/// So the two spellings resolve onto one number here as well, and a deck may
/// use either.
fn parse_pss(
    card: &ParsedCard,
    params: &ParamContext,
    driven_tone_sources: impl FnOnce() -> Result<Vec<String>, String>,
) -> Result<AnalysisSpec, String> {
    if !card.positional.is_empty() {
        return Err(
            ".PSS in a manual deck is the keyword card `.PSS fund=<frequency> [key=value ...]`; \
             ngspice's positional oscillator card names the node its period is detected on, so a \
             driven solve cannot be written in it at all"
                .to_owned(),
        );
    }
    reject_unsupported_keys(card, PSS_KEYS, ".PSS")?;
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
    let integration_method = match card.keyed.get("method") {
        None => None,
        Some(spelling) => {
            let spelling = unquote(spelling).trim().to_owned();
            Some(
                crate::simulation::dialog::IntegrationMethod::from_spice_name(&spelling)
                    .ok_or_else(|| {
                        format!(".PSS method={spelling:?} is not TRAP, GEAR, EULER or TRAPGEAR")
                    })?,
            )
        }
    };
    let spec = AnalysisSpec::Pss {
        method: PssMethod::Shooting,
        fundamental_freq: pss_fundamental(card, oscillator_mode, params)?,
        tone_sources,
        tstab_periods,
        points_per_period: optional_usize(card, "points", 256, params)?,
        tolerance: optional_value(card, "tol", 1.0e-6, params)?,
        oscillator_mode,
        oscillator_node,
        num_harmonics,
        integration_method,
        // Every default is the engine's own card default, so a key the deck
        // omits means the same thing to both readers.
        tstab: optional_value(card, "tstab", 0.0, params)?,
        max_iterations: optional_usize(card, "maxiter", 100, params)?,
        abstol: optional_value(card, "abstol", 1.0e-12, params)?,
        damping: optional_value(card, "damping", 1.0, params)?,
        max_period_change: optional_value(card, "maxperiodchange", 0.1, params)?,
        verbose: optional_bool(card, "verbose", false)?,
    };
    spec.validate()
        .map_err(|error| format!("invalid .PSS: {error}"))?;
    Ok(spec)
}

/// The fundamental the card states, in whichever of its two spellings.
///
/// `FUND=` is a frequency and `PERIODGUESS=` is its reciprocal, and the engine
/// refuses both on one card because they are one quantity. `PERIODGUESS=`
/// belongs only to an autonomous card — the driven form has a known period,
/// not an estimate of one — and `parse_pss_command` refuses it there in the
/// same words.
fn pss_fundamental(
    card: &ParsedCard,
    oscillator_mode: bool,
    params: &ParamContext,
) -> Result<f64, String> {
    let fundamental = card
        .keyed
        .contains_key("fund")
        .then(|| numeric_value(&card.keyed["fund"], ".PSS fund", params))
        .transpose()?;
    let period_guess = card
        .keyed
        .contains_key("periodguess")
        .then(|| numeric_value(&card.keyed["periodguess"], ".PSS periodguess", params))
        .transpose()?;
    match (fundamental, period_guess) {
        (Some(_), Some(_)) => Err(
            ".PSS states both fund= and periodguess=, which are one period spelled twice"
                .to_owned(),
        ),
        (Some(frequency), None) => Ok(frequency),
        (None, Some(period)) if !oscillator_mode => {
            let _ = period;
            Err(
                ".PSS periodguess= estimates an oscillator's own period; author autonomous=yes, \
                 or state the driven period as fund="
                    .to_owned(),
            )
        }
        (None, Some(period)) => {
            if !period.is_finite() || period <= 0.0 {
                return Err(".PSS periodguess must be a positive period in seconds".to_owned());
            }
            Ok(1.0 / period)
        }
        (None, None) => Err(".PSS requires fund=<frequency>".to_owned()),
    }
}

fn parse_pac(
    card: &ParsedCard,
    params: &ParamContext,
    carrier: PeriodicCarrier,
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
            "pacmag",
            "includedc",
            "from",
        ],
        ".PAC",
    )?;
    let (sweep, points_per_unit, start_freq, stop_freq) = frequency_sweep(card, ".PAC", params)?;
    let input_source = required_text(card, "input", ".PAC")?;
    let (output_node, output_ref) = required_output(card, ".PAC")?;
    let config = PacRunConfig {
        pss_fundamental_freq: carrier.fundamental_freq,
        pss_num_harmonics: carrier.num_harmonics,
        pss_tolerance: carrier.tolerance,
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
        // The engine's card carries both, at these defaults, so a deck that
        // says nothing means what a studio run with the untouched options
        // means.
        pac_magnitude: optional_value(card, "pacmag", 1.0, params)?,
        include_dc: optional_bool(card, "includedc", true)?,
        // The card may state the frequency-domain tolerances; a deck that does
        // not falls back to its own `.options`, which is a sharper answer than
        // the card constant and the one this reader has always given.
        reltol: optional_value(card, "reltol", reltol, params)?,
        abstol: optional_value(card, "abstol", abstol, params)?,
        carrier: CarrierSelector::Preceding,
    };
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
    if !config.pac_magnitude.is_finite() || config.pac_magnitude <= 0.0 {
        return Err(".PAC pacmag must be a finite positive drive amplitude".to_owned());
    }
    // The engine's card refuses this pairing where it is written; so does the
    // reader, rather than letting the run fail with nothing to publish.
    if !config.include_dc && config.max_sideband == 0 {
        return Err(".PAC includedc=no withholds the only sideband this card analyses".to_owned());
    }
    Ok(config)
}

fn parse_pnoise(
    card: &ParsedCard,
    params: &ParamContext,
    carrier: PeriodicCarrier,
    reltol: f64,
    abstol: f64,
) -> Result<PnoiseRunConfig, String> {
    reject_unsupported_keys(
        card,
        &[
            "out",
            "input",
            "maxsideband",
            "noiseref",
            "integratednoise",
            "noisesummary",
            "from",
        ],
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
    // `NOISEREF=` states the measurement and `INPUT=` alone implies
    // input-referred, which is the spelling that predates the keyword. The
    // engine's card refuses the two when they disagree; so does this reader,
    // in the same words, so a deck means one thing to both.
    let noise_ref = match card.keyed.get("noiseref") {
        Some(spelling) => match unquote(spelling).trim().to_ascii_lowercase().as_str() {
            "output" | "out" => PnoiseReference::Output,
            "input" | "in" => PnoiseReference::Input,
            "phase" | "pm" => PnoiseReference::Phase,
            other => {
                return Err(format!(
                    ".PNOISE noiseref={other:?} is not one of output, input or phase"
                ));
            }
        },
        None if input_source.is_empty() => PnoiseReference::Output,
        None => PnoiseReference::Input,
    };
    if noise_ref == PnoiseReference::Input && input_source.is_empty() {
        return Err(".PNOISE noiseref=input requires input=<source>".to_owned());
    }
    if noise_ref != PnoiseReference::Input && !input_source.is_empty() {
        return Err(
            ".PNOISE names an input source and then measures something other than \
             input-referred noise"
                .to_owned(),
        );
    }
    // The engine refuses the same two pairings when it runs the card; refusing
    // them here means the deck is told which line is wrong.
    if noise_ref == PnoiseReference::Phase && !carrier.autonomous {
        return Err(
            ".PNOISE noiseref=phase needs an autonomous carrier: a driven orbit has no free \
             phase to diffuse, so author .PSS autonomous=yes"
                .to_owned(),
        );
    }
    if noise_ref == PnoiseReference::Input && carrier.autonomous {
        return Err(
            ".PNOISE input= refers noise to a driving source, and an autonomous .PSS has none; \
             author noiseref=phase for an oscillator's phase noise"
                .to_owned(),
        );
    }
    let config = PnoiseRunConfig {
        pss_fundamental_freq: carrier.fundamental_freq,
        pss_num_harmonics: carrier.num_harmonics,
        pss_tolerance: carrier.tolerance,
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
        // The engine's card carries both, at these defaults.
        integrated_noise: optional_bool(card, "integratednoise", false)?,
        noise_summary: optional_bool(card, "noisesummary", true)?,
        reltol,
        abstol,
        carrier: CarrierSelector::Preceding,
    };
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
    carrier: PeriodicCarrier,
    reltol: f64,
    abstol: f64,
) -> Result<PxfRunConfig, String> {
    // `parse_pxf_command`'s key set, complete. `RELTOL=`, `ABSTOL=` and
    // `FROM=` were missing, so a `.pxf` line the engine's parser accepts was
    // refused here by name — and since the deck is parsed by the engine before
    // it reaches this reader, the deck that failed was one the engine had
    // already read. They are handled the way `.PAC` handles the same three
    // keys, which is the card these two share their tolerance defaults with.
    reject_unsupported_keys(
        card,
        &[
            "maxsideband",
            "inputsideband",
            "outsideband",
            "input",
            "out",
            "reltol",
            "abstol",
            "from",
        ],
        ".PXF",
    )?;
    let (sweep, points_per_unit, start_freq, stop_freq) = frequency_sweep(card, ".PXF", params)?;
    let input_source = required_text(card, "input", ".PXF")?;
    let (output_node, output_ref) = required_output(card, ".PXF")?;
    let config = PxfRunConfig {
        pss_fundamental_freq: carrier.fundamental_freq,
        pss_num_harmonics: carrier.num_harmonics,
        pss_tolerance: carrier.tolerance,
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
        // As `.PAC`: the card may state the frequency-domain tolerances, and a
        // deck that does not falls back to its own `.options` rather than to
        // the card's constant.
        reltol: optional_value(card, "reltol", reltol, params)?,
        abstol: optional_value(card, "abstol", abstol, params)?,
        carrier: CarrierSelector::Preceding,
    };
    validate_frequency_contract(
        ".PXF",
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
        return Err(".PXF reltol and abstol must be finite and positive".to_owned());
    }
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
    carrier: PeriodicCarrier,
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
        ".PSTB",
    )?;
    if !card.positional.is_empty() {
        return Err(".PSTB accepts keyed options only".to_owned());
    }
    let config = PstbRunConfig {
        pss_fundamental_freq: carrier.fundamental_freq,
        pss_num_harmonics: carrier.num_harmonics,
        pss_tolerance: carrier.tolerance,
        probe_instance: required_text(card, "probe", ".PSTB")?,
        max_harmonics: optional_usize(card, "maxharm", 10, params)?,
        num_multipliers: optional_usize(card, "nmults", 10, params)?,
        stability_threshold: optional_value(card, "stabilitythreshold", 1.0 + 1.0e-6, params)?,
        detect_subharmonics: optional_bool(card, "detectsubharmonics", true)?,
        eigenvalue_tolerance: optional_value(card, "eigentol", 1.0e-10, params)?,
    };
    // `rspice_core::Engine::run_pstb_card_from_pss_with_abort` validates its
    // card before it resolves a probe, and these are its three refusals in its
    // own words. One refusal per field, and the field's value in the sentence:
    // the single combined message this replaced named four keys and told the
    // operator nothing about which of them the card got wrong.
    //
    // The threshold bound is the one that mattered. It admitted anything above
    // zero, while the engine, and both copies of the check in
    // `services::simulation_runner::pstb`, require a finite magnitude of at
    // least one — the physical boundary is |lambda| = 1, so a threshold below
    // it would call a mode sitting exactly on the unit circle unstable. So the
    // Studio queued a run its own engine rejects, and the operator found out
    // after the solve had started rather than when the deck was read.
    if config.max_harmonics == 0 || config.num_multipliers == 0 {
        return Err(
            ".PSTB requires at least one carrier harmonic and one reported multiplier".to_owned(),
        );
    }
    if !config.stability_threshold.is_finite() || config.stability_threshold < 1.0 {
        return Err(format!(
            ".PSTB requires a finite stability threshold of at least one, got {}",
            config.stability_threshold
        ));
    }
    if !config.eigenvalue_tolerance.is_finite() || config.eigenvalue_tolerance <= 0.0 {
        return Err(format!(
            ".PSTB requires a positive eigenvalue tolerance, got {}",
            config.eigenvalue_tolerance
        ));
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
/// It took a second list until this lane: keys the *engine* accepted on the
/// same card but whose value the studio's typed run configuration had no field
/// for, refused by name with a reason and a remedy. `.PSS` was its only user,
/// and the studio now holds all eight of those controls, so both the table and
/// the parameter that carried it are gone. An unknown key is the one refusal
/// left, which is the one the engine's own parser gives.
fn reject_unsupported_keys(
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
            "{directive} from=hb has no route in the Studio: a manual-deck periodic analysis \
             binds to the .PSS operating point in the same deck, and no Studio runner linearizes \
             a harmonic-balance carrier for this card. The engine does, so a deck carrying it \
             runs on the command line; author from=pss to run it here"
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

    /// The reader's `.PSS` key set is the engine's, read out of the engine.
    ///
    /// The sibling test below proves the family's readers agree by authoring
    /// each key and watching both accept it, which can only catch a key this
    /// reader refuses. It cannot catch the other direction — a keyword the
    /// engine's card grew that nobody taught this reader — because nothing in
    /// this crate knows what that set is. So this reads it: the keyword arms
    /// of `parse_pss_command` itself, out of the engine's own source.
    ///
    /// That is the same shape as the guards in `source_guard`, and it carries
    /// the same obligation: a scan that stops matching passes forever, so the
    /// function's text has to be found or this fails, and the set it yields
    /// has to be big enough to be the real one.
    #[test]
    fn the_reader_and_the_engine_accept_the_same_pss_key_set() {
        let engine_source = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("this crate sits in the workspace's crates/ directory")
            .join("rspice-core/src/netlist/parser/periodic_cards.rs");
        let source = std::fs::read_to_string(&engine_source)
            .unwrap_or_else(|error| panic!("read {}: {error}", engine_source.display()));
        let start = source
            .find("fn parse_pss_command(")
            .expect("the engine still spells the .PSS parser this way");
        // The keyword loop ends where the next item starts, and `rustfmt`
        // writes every top-level item at column zero.
        let body = &source[start..];
        let end = body[1..]
            .find("\nfn ")
            .map(|offset| offset + 1)
            .expect("the .PSS parser is followed by another function");
        let body = &body[..end];

        let mut engine_keys = body
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim_start();
                let rest = trimmed.strip_prefix('"')?;
                let (keyword, tail) = rest.split_once('"')?;
                // An arm, not a field name or a message: `"KEY" =>`.
                tail.trim_start()
                    .starts_with("=>")
                    .then(|| keyword.to_ascii_lowercase())
            })
            .collect::<Vec<_>>();
        engine_keys.sort();
        engine_keys.dedup();
        assert!(
            engine_keys.len() >= 15,
            "the scan found only {} `.PSS` keyword arms; a scan that reaches nothing passes \
             forever: {engine_keys:?}",
            engine_keys.len()
        );

        let mut reader_keys = PSS_KEYS
            .iter()
            .map(|key| (*key).to_owned())
            .collect::<Vec<_>>();
        reader_keys.sort();
        assert_eq!(
            reader_keys, engine_keys,
            "the deck reader and the engine's `.PSS` card must accept the same keywords"
        );
    }

    /// Every solver control the form can author survives the deck.
    ///
    /// The card is written by the dialog's own emitter and read back by this
    /// reader, so what is proved is the round trip a saved project takes: the
    /// studio writes the deck, a reader hands it back as a typed request, and
    /// the two requests are the same run.
    #[test]
    fn pss_solver_controls_round_trip_through_the_deck_reader() {
        use crate::simulation::dialog::{IntegrationMethod, PssConfig};

        const CIRCUIT: &str =
            "pss solver controls\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        let authored = PssConfig {
            integration_method: Some(IntegrationMethod::Gear2),
            fund_freq: 1.0e6,
            tone_sources: vec!["V1".to_owned()],
            tstab_periods: 7,
            tstab: 3.0e-9,
            max_iterations: 250,
            abstol: 1.0e-15,
            damping: 0.75,
            max_period_change: 0.25,
            points_per_period: 1024,
            tolerance: 1.0e-8,
            osc_mode: false,
            osc_node: String::new(),
            num_harmonics: 15,
        };
        authored.validate().expect("the authored request is valid");
        let directive = authored.to_spice();
        let deck = format!("{CIRCUIT}{directive}\n.end\n");

        // The engine reads the card first, exactly as a run would.
        let netlist = Netlist::parse(&deck)
            .unwrap_or_else(|error| panic!("the engine reads `{directive}`: {error}"));
        let tasks = parse_periodic_tasks(&netlist, &deck)
            .unwrap_or_else(|errors| panic!("the deck reader refused: {}", errors.join("; ")));
        let AnalysisSpec::Pss {
            fundamental_freq,
            tstab_periods,
            points_per_period,
            tolerance,
            num_harmonics,
            integration_method,
            tstab,
            max_iterations,
            abstol,
            damping,
            max_period_change,
            verbose,
            ..
        } = tasks
            .iter()
            .map(|task| &task.spec)
            .find(|spec| matches!(spec, AnalysisSpec::Pss { .. }))
            .expect("the deck reader recovers the PSS")
            .clone()
        else {
            unreachable!("filtered to the PSS spec");
        };
        assert_eq!(fundamental_freq, 1.0e6);
        assert_eq!(tstab_periods, 7);
        assert_eq!(points_per_period, 1024);
        assert_eq!(tolerance, 1.0e-8);
        assert_eq!(num_harmonics, 15);
        assert_eq!(integration_method, Some(IntegrationMethod::Gear2));
        assert_eq!(tstab, 3.0e-9);
        assert_eq!(max_iterations, 250);
        assert_eq!(abstol, 1.0e-15);
        assert_eq!(damping, 0.75);
        assert_eq!(max_period_change, 0.25);
        assert!(!verbose, "the form authors no solver log");
    }

    /// `VERBOSE=` is the one control only a deck can state, and it reaches the
    /// engine from one.
    #[test]
    fn a_deck_authors_the_solver_log_the_form_does_not_offer() {
        const CIRCUIT: &str = "pss verbose\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        let deck = format!("{CIRCUIT}.pss fund=1Meg verbose=yes\n.end\n");
        let netlist = Netlist::parse(&deck).expect("the engine reads verbose=");
        let tasks = parse_periodic_tasks(&netlist, &deck)
            .unwrap_or_else(|errors| panic!("the deck reader refused: {}", errors.join("; ")));
        assert!(matches!(
            tasks.first().map(|task| &task.spec),
            Some(AnalysisSpec::Pss { verbose: true, .. })
        ));
    }

    /// A deck may spell the autonomous period estimate either way, and not
    /// both.
    #[test]
    fn an_autonomous_period_estimate_is_one_quantity_in_two_spellings() {
        const CIRCUIT: &str =
            "pss period guess\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";

        let deck = format!("{CIRCUIT}.pss autonomous=yes oscnode=out periodguess=1n\n.end\n");
        let netlist = Netlist::parse(&deck).expect("the engine reads periodguess=");
        let tasks = parse_periodic_tasks(&netlist, &deck)
            .unwrap_or_else(|errors| panic!("the deck reader refused: {}", errors.join("; ")));
        assert!(matches!(
            tasks.first().map(|task| &task.spec),
            Some(AnalysisSpec::Pss {
                oscillator_mode: true,
                fundamental_freq,
                ..
            }) if (*fundamental_freq - 1.0e9).abs() <= 1.0
        ));

        // Both spellings on one card is the engine's own refusal, so the
        // reader must refuse the deck the engine will not read either.
        let conflicting =
            format!("{CIRCUIT}.pss autonomous=yes oscnode=out fund=1Meg periodguess=1n\n.end\n");
        assert!(
            Netlist::parse(&conflicting).is_err(),
            "the premise is that the engine refuses one card stating both"
        );
        let seeded = Netlist::parse(&format!("{CIRCUIT}.end\n")).expect("the fixture parses");
        let errors = parse_periodic_tasks(&seeded, &conflicting)
            .expect_err("one period spelled twice is not one card");
        assert!(
            errors.iter().any(|error| error.contains("spelled twice")),
            "{errors:?}"
        );
    }

    /// A deck `METHOD=` spelling means the same integrator to both readers.
    ///
    /// [`crate::simulation::dialog::IntegrationMethod::from_spice_name`] is
    /// this crate's copy of a table that lives in the engine, so it is held to
    /// the engine's by running every spelling through the engine's own `.PSS`
    /// parser. A spelling neither accepts is checked too: a table that said
    /// yes to everything would pass the first half alone.
    #[test]
    fn a_deck_method_spelling_means_the_same_thing_to_both_readers() {
        use crate::simulation::dialog::IntegrationMethod;
        use rspice_core::netlist::AnalysisCommand;

        const CIRCUIT: &str = "pss method\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        let engine_method = |spelling: &str| {
            let deck = format!("{CIRCUIT}.pss fund=1Meg method={spelling}\n.end\n");
            let netlist = Netlist::parse(&deck).ok()?;
            match netlist.analyses.into_iter().next() {
                Some(AnalysisCommand::Pss(card)) => card.integration_method,
                other => panic!("expected a .PSS card, got {other:?}"),
            }
        };

        for spelling in IntegrationMethod::spice_spellings() {
            let reader = IntegrationMethod::from_spice_name(spelling)
                .unwrap_or_else(|| panic!("this reader accepts {spelling}"));
            assert_eq!(
                engine_method(spelling),
                Some(reader.core()),
                "`method={spelling}` names a different integrator to each reader"
            );
        }
        for refused in ["bdf2", "gear3", "spectre", ""] {
            assert_eq!(
                IntegrationMethod::from_spice_name(refused),
                None,
                "`method={refused}` is not a method the engine integrates under"
            );
            assert_eq!(engine_method(refused), None, "{refused}");
        }
    }

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

    /// The five options that used to reach a run only through the Studio's
    /// typed request now reach it from the deck, and mean the same thing.
    #[test]
    fn periodic_cards_carry_the_drive_amplitude_and_the_noise_reporting_options() {
        let source = "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n\
.pss fund=1Meg\n\
.pac dec 10 1k 100Meg maxsideband=7 input=V1 out=out pacmag=25m includedc=no\n\
.pnoise dec 10 1 1Meg out=out maxsideband=9 integratednoise=yes noisesummary=no\n.end\n";
        let netlist = Netlist::parse(source).unwrap();

        let tasks = parse_periodic_tasks(&netlist, source).unwrap();

        let pac = tasks
            .iter()
            .find_map(|task| task.spec_options.pac.as_ref())
            .expect("the .PAC card is queued");
        assert!(
            (pac.pac_magnitude - 25e-3).abs() <= 1e-18,
            "pacmag was {}",
            pac.pac_magnitude
        );
        assert!(!pac.include_dc);
        let pnoise = tasks
            .iter()
            .find_map(|task| task.spec_options.pnoise.as_ref())
            .expect("the .PNOISE card is queued");
        assert!(pnoise.integrated_noise);
        assert!(!pnoise.noise_summary);
        assert_eq!(pnoise.noise_ref, PnoiseReference::Output);
    }

    /// Phase noise is authorable in a deck, and only around the carrier that
    /// has a phase to diffuse.
    #[test]
    fn a_deck_authors_phase_noise_only_around_an_autonomous_carrier() {
        const CIRCUIT: &str = "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        let source = format!(
            "{CIRCUIT}.pss fund=1Meg autonomous=yes oscnode=out\n\
             .pnoise dec 10 1 1Meg out=out noiseref=phase\n.end\n"
        );
        let netlist = Netlist::parse(&source).expect("the autonomous deck parses");
        let tasks = parse_periodic_tasks(&netlist, &source).expect("the deck queues");
        let pnoise = tasks
            .iter()
            .find_map(|task| task.spec_options.pnoise.as_ref())
            .expect("the .PNOISE card is queued");
        assert_eq!(pnoise.noise_ref, PnoiseReference::Phase);

        let driven = format!(
            "{CIRCUIT}.pss fund=1Meg\n.pnoise dec 10 1 1Meg out=out noiseref=phase\n.end\n"
        );
        let netlist = Netlist::parse(&driven).expect("the driven deck parses");
        let errors = parse_periodic_tasks(&netlist, &driven)
            .expect_err("phase noise around a driven carrier is refused");
        assert!(
            errors.iter().any(|error| error.contains("autonomous")),
            "unexpected refusal: {errors:?}"
        );
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
            // A drive amplitude that is not a positive number, a card that
            // withholds the only sideband it analyses, and a noise reference
            // that disagrees with the source the card names.
            ".pac dec 10 1k 1Meg input=V1 out=out pacmag=0",
            ".pac dec 10 1k 1Meg input=V1 out=out maxsideband=0 includedc=no",
            ".pnoise dec 10 1 1Meg out=out noiseref=input",
            ".pnoise dec 10 1 1Meg out=out input=V1 noiseref=output",
            ".pnoise dec 10 1 1Meg out=out noiseref=amplitude",
            // `.PXF`: keywords the card does not carry, ends of the path it
            // cannot default, a conversion measured outside the depth it
            // states, a probe measured against itself, a tolerance that is not
            // a tolerance, one quantity spelled twice, and a carrier the
            // family has no selector for.
            ".pxf dec 10 1k 1Meg input=V1 out=out sidebands=3",
            ".pxf dec 10 1k 1Meg out=out",
            ".pxf dec 10 1k 1Meg input=V1",
            ".pxf dec 10 1k 1Meg input=V1 out=out maxsideband=1 outsideband=3",
            ".pxf dec 10 1k 1Meg input=V1 out=out maxsideband=1 inputsideband=-4",
            ".pxf dec 10 1k 1Meg input=V1 out=V(out,out)",
            ".pxf dec 10 1k 1Meg input=V1 out=out maxsideband=-1",
            ".pxf dec 10 1k 1Meg input=V1 out=out reltol=0",
            ".pxf dec 10 1k 1Meg input=V1 out=out abstol=0",
            ".pxf dec 10 1k 1Meg input=V1 input=VLO out=out",
            ".pxf dec 10 1k 1Meg input=V1 out=out from=tran",
            ".pxf lin 10 1Meg 1k input=V1 out=out",
            // `.PSTB`: the probe it cannot default, a threshold inside the
            // unit circle, counts of nothing, a tolerance of zero, a switch
            // that is neither state, a keyword the card does not carry, a
            // carrier selector it deliberately does not offer, one quantity
            // spelled twice, and a bare positional where a key belongs.
            ".pstb maxharm=6",
            ".pstb probe=l1 stabilitythreshold=0.5",
            ".pstb probe=l1 maxharm=0",
            ".pstb probe=l1 nmults=0",
            ".pstb probe=l1 eigentol=0",
            ".pstb probe=l1 detectsubharmonics=sometimes",
            ".pstb probe=l1 sweeptype=dec",
            ".pstb probe=l1 from=hb",
            ".pstb probe=l1 probe=l2",
            ".pstb l1",
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

    /// `FROM=HB` is the one card in the family the two readers answer
    /// differently, and the difference is a limitation rather than a drift.
    ///
    /// The agreement test above has no case for it because it cannot: the
    /// engine *accepts* `FROM=HB`. Core's `.PAC`, `.PNOISE` and `.PXF`
    /// grammars all admit `FROM=PSS|HB`, the plan binds such a card to the
    /// deck's preceding `.HB`, and the CLI runs it through
    /// `Engine::run_pxf_card_from_hb_with_abort`. The Studio has no runner
    /// that takes a harmonic-balance operating point for any of the three, and
    /// a manual deck binds them to the `.PSS` in the same deck, so it refuses.
    ///
    /// Pinned from both sides, and in both deck shapes, because the refusal
    /// has to say the same thing whether or not a `.PSS` happens to be present
    /// — the deck the engine actually runs is the one with no `.PSS` at all,
    /// and that is the shape that used to be told it was missing one.
    #[test]
    fn a_harmonic_balance_carrier_the_engine_accepts_is_refused_by_the_studio_alone() {
        const CIRCUIT: &str = "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        const HB: &str = ".hb 1Meg\n";

        for card in [
            ".pxf dec 10 1k 1Meg input=V1 out=out from=hb",
            ".pac dec 10 1k 1Meg input=V1 out=out from=hb",
            ".pnoise dec 10 1 1Meg out=out from=hb",
        ] {
            for seed in ["", ".pss fund=1Meg\n"] {
                let source = format!("{CIRCUIT}{HB}{seed}{card}\n.end\n");
                let netlist = Netlist::parse(&source).unwrap_or_else(|error| {
                    panic!("the engine accepts `{card}`; this case no longer tests what it claims: {error}")
                });
                rspice_core::execution::DeckPlan::from_netlist(
                    &netlist,
                    &rspice_core::resource::ResourceLimits::default(),
                )
                .unwrap_or_else(|error| {
                    panic!("the engine binds `{card}` to the deck's .HB carrier: {error}")
                });

                let studio_circuit = Netlist::parse(&format!("{CIRCUIT}.end\n"))
                    .expect("the fixture circuit must parse");
                let errors = parse_periodic_tasks(&studio_circuit, &source)
                    .expect_err("the Studio has no route for a harmonic-balance carrier");
                assert!(
                    errors
                        .iter()
                        .any(|error| error.contains("from=hb") && error.contains("command line")),
                    "`{card}` must be refused as the carrier it names, in every deck shape, and \
                     must say where it does run: {errors:?}"
                );
            }
        }
    }

    /// The reader's key set is the engine's key set, key for key.
    ///
    /// The two tests above prove the readers agree on whole cards. This proves
    /// they agree on the *vocabulary*, which is the half that actually
    /// drifted: `.PXF` accepted five of the engine's eight keys and refused
    /// `RELTOL=`, `ABSTOL=` and `FROM=` by name. Because a manual deck is
    /// parsed by the engine before it reaches this reader, that refusal fired
    /// on a line the engine had already read — the card was well formed, and
    /// the Studio would not run the deck that held it.
    ///
    /// Each key is authored alone, so a failure names the key rather than the
    /// card, and then all of them together, so a key accepted only in
    /// isolation is caught as well.
    #[test]
    fn the_reader_and_the_engine_accept_the_same_periodic_key_set() {
        const CIRCUIT: &str = "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        const SEED: &str = ".pss fund=1Meg\n";
        const PXF_BASE: &str = ".pxf dec 10 1k 1Meg input=V1 out=out";
        const PSTB_BASE: &str = ".pstb probe=l1";

        const PXF_EVERY_KEY: &str = ".pxf dec 10 1k 1Meg input=V1 out=out inputsideband=-1 \
             outsideband=2 maxsideband=4 reltol=1e-4 abstol=1e-14 from=pss";
        const PSTB_EVERY_KEY: &str = ".pstb probe=l1 maxharm=8 nmults=6 stabilitythreshold=1.5 \
             detectsubharmonics=no eigentol=1e-9";

        let mut cards = vec![PXF_EVERY_KEY.to_owned(), PSTB_EVERY_KEY.to_owned()];
        // One line per arm of `parse_pxf_command` and `parse_pstb_command`.
        // `INPUT=`/`OUT=`/`PROBE=` are in every base above because neither
        // card defaults them.
        for (base, key, value) in [
            (PXF_BASE, "inputsideband", "-1"),
            (PXF_BASE, "outsideband", "2"),
            (PXF_BASE, "maxsideband", "4"),
            (PXF_BASE, "reltol", "1e-4"),
            (PXF_BASE, "abstol", "1e-14"),
            (PXF_BASE, "from", "pss"),
            (PSTB_BASE, "maxharm", "8"),
            (PSTB_BASE, "nmults", "6"),
            (PSTB_BASE, "stabilitythreshold", "1.5"),
            (PSTB_BASE, "detectsubharmonics", "no"),
            (PSTB_BASE, "eigentol", "1e-9"),
        ] {
            cards.push(format!("{base} {key}={value}"));
        }

        for card in &cards {
            let source = format!("{CIRCUIT}{SEED}{card}\n.end\n");
            let netlist = Netlist::parse(&source)
                .unwrap_or_else(|error| panic!("the engine must accept `{card}`: {error}"));
            parse_periodic_tasks(&netlist, &source).unwrap_or_else(|errors| {
                panic!(
                    "the engine accepts `{card}` and the studio does not: {}",
                    errors.join("; ")
                )
            });
        }

        // Accepting a key is not carrying it. The two tolerances were the
        // reason the key set diverged at all, so they are read back rather
        // than merely tolerated.
        let source = format!("{CIRCUIT}{SEED}{PXF_EVERY_KEY}\n.end\n");
        let netlist = Netlist::parse(&source).expect("the fully keyed .PXF parses");
        let tasks = parse_periodic_tasks(&netlist, &source).expect("the deck queues");
        let pxf = tasks
            .iter()
            .find_map(|task| task.spec_options.pxf.as_ref())
            .expect("the .PXF card is queued");
        assert!(
            (pxf.reltol - 1.0e-4).abs() <= 1.0e-19,
            "reltol was {}",
            pxf.reltol
        );
        assert!(
            (pxf.abstol - 1.0e-14).abs() <= 1.0e-29,
            "abstol was {}",
            pxf.abstol
        );
        assert_eq!(pxf.input_sideband, -1);
        assert_eq!(pxf.output_sideband, 2);
        assert_eq!(pxf.max_sideband, 4);
    }

    /// A `.PSTB` card the engine's `validate_card` refuses is refused here, in
    /// the engine's own sentence, before anything is queued.
    ///
    /// This reader admitted any threshold above zero. The engine entry
    /// requires a finite magnitude of at least one, and so do both copies of
    /// the check in `services::simulation_runner::pstb` — the physical
    /// boundary is |lambda| = 1, and a threshold below it would call a mode
    /// sitting exactly on the unit circle unstable. So the Studio queued a run
    /// its own engine rejects, and the operator learned of it after the solve
    /// had started rather than when the deck was read.
    #[test]
    fn a_sub_unit_stability_threshold_is_refused_before_the_run_is_queued() {
        const CIRCUIT: &str = "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";

        // The engine's parser refuses the card outright, so the reader is
        // exercised against the deck the engine could not read — which is what
        // makes it a second line of defence rather than the only one.
        let refused =
            format!("{CIRCUIT}.pss fund=1Meg\n.pstb probe=l1 stabilitythreshold=0.5\n.end\n");
        assert!(
            Netlist::parse(&refused).is_err(),
            "the premise of this test is that the engine refuses the card"
        );
        let seeded = Netlist::parse(&format!("{CIRCUIT}.pss fund=1Meg\n.end\n"))
            .expect("the fixture circuit parses");
        let errors = parse_periodic_tasks(&seeded, &refused)
            .expect_err("a threshold inside the unit circle is not a stability contract");
        assert!(
            errors.iter().any(|error| {
                error.contains("requires a finite stability threshold of at least one, got 0.5")
            }),
            "the refusal must be the engine's own sentence, with the value: {errors:?}"
        );

        // Unity itself is admissible, at both ends: it is the boundary, not a
        // value inside it.
        let boundary =
            format!("{CIRCUIT}.pss fund=1Meg\n.pstb probe=l1 stabilitythreshold=1\n.end\n");
        let netlist = Netlist::parse(&boundary).expect("the engine accepts the boundary itself");
        let tasks = parse_periodic_tasks(&netlist, &boundary).expect("and so does the studio");
        let pstb = tasks
            .iter()
            .find_map(|task| task.spec_options.pstb.as_ref())
            .expect("the .PSTB card is queued");
        assert_eq!(pstb.stability_threshold, 1.0);
    }
}

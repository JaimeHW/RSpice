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

/// What the deck's periodic solve settled, as every dependent card needs it.
///
/// The four numbers travel together because they describe one operating point:
/// splitting them across argument lists let a dependent card be bound to three
/// of them and not the fourth, which is how `noiseref=phase` used to reach a
/// driven carrier.
///
/// Read off whichever family the dependent card's `FROM=` names — the deck's
/// one `.PSS`, or its `.HB`. For a harmonic-balance carrier the three numbers
/// are the authored basis rather than the solved one: `prepare_periodic_ac`
/// replaces a dependent's fundamental with the retained state's own before it
/// solves anything, and the retained state is what the Studio hands over.
#[derive(Debug, Clone, Copy, Default)]
struct PeriodicCarrier {
    fundamental_freq: f64,
    num_harmonics: usize,
    tolerance: f64,
    /// Whether the period is a solver unknown rather than authored input.
    /// Never true of a harmonic-balance carrier: its period is the authored
    /// tone.
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

    // The carrier selector is read before the precondition below, and not
    // inside each card's parser, because a malformed selector is not a card
    // that forgot its carrier. `.PSTB` is deliberately absent: it has no
    // `FROM=` key in either reader, so `from=` on one is an unknown keyword
    // and stays one.
    let mut selectors = HashMap::new();
    for (line, head, card) in &parsed {
        if !matches!(head.as_str(), ".pac" | ".pnoise" | ".pxf") {
            continue;
        }
        match periodic_source_selector(card, &head.to_ascii_uppercase()) {
            Ok(selector) => {
                selectors.insert(*line, selector);
            }
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
    // The deck's harmonic-balance carrier, read off the command the engine
    // parsed rather than off the card text: `.HB` belongs to the manual
    // reader's own dispatch, which is the one place that knows how the card's
    // positional tones and the deck's `.OPTIONS HBINT NUMFREQ=` combine. The
    // line it was written on comes from the source, so a card without `FROM=`
    // can be bound the way the engine binds it.
    let hb_carrier = harmonic_balance_carrier(netlist);
    let hb_lines = harmonic_balance_card_lines(source);
    let pss_line = pss_cards.first().map(|(line, _, _)| *line);
    // Which family each dependent card reads, resolved once so the
    // precondition below and the basis each card is bound to cannot disagree.
    let mut families = HashMap::new();
    for (line, head, _) in &parsed {
        let family = match head.as_str() {
            // `.PSTB` reads a monodromy matrix and only a shooting solve
            // produces one, so it has no family to resolve.
            ".pstb" => CarrierSelector::Pss,
            ".pac" | ".pnoise" | ".pxf" => {
                match selectors.get(line).copied().unwrap_or_default() {
                    CarrierSelector::Pss => CarrierSelector::Pss,
                    CarrierSelector::Hb => CarrierSelector::Hb,
                    // The absent keyword: the nearest *preceding* periodic
                    // solve of either family, which is
                    // `resolve_periodic_source`'s rule. A card written above
                    // its own solve keeps today's acceptance rather than
                    // gaining a refusal — manual-deck analysis directives are
                    // declarative, and `prepare_manual_tasks` orders the
                    // producer before its consumer whatever order the deck
                    // wrote them in.
                    CarrierSelector::Preceding => {
                        let nearest_hb = hb_lines.iter().copied().filter(|hb| hb < line).max();
                        let nearest_pss = pss_line.filter(|pss| pss < line);
                        match (nearest_pss, nearest_hb) {
                            (Some(pss), Some(hb)) if hb > pss => CarrierSelector::Hb,
                            (Some(_), _) => CarrierSelector::Pss,
                            (None, Some(_)) => CarrierSelector::Hb,
                            (None, None) if pss_line.is_none() && hb_carrier.is_some() => {
                                CarrierSelector::Hb
                            }
                            (None, None) => CarrierSelector::Pss,
                        }
                    }
                }
            }
            _ => continue,
        };
        families.insert(*line, family);
    }
    let dependents_needing_shooting = families
        .values()
        .filter(|family| matches!(family, CarrierSelector::Pss))
        .count();
    if dependents_needing_shooting > 0 && pss_cards.is_empty() {
        // `.PSTB` reads a monodromy matrix and only a shooting solve produces
        // one, so a deck whose nearest carrier is `.HB` is answered in the
        // engine's own words rather than told it forgot a `.PSS` it may have
        // meant to leave out. `DeckPlan::from_netlist` states the same thing
        // for the same deck.
        if hb_carrier.is_some() && parsed.iter().any(|(_, head, _)| head == ".pstb") {
            return Err(vec![
                ".PSTB requires a preceding .PSS; a harmonic-balance carrier has no monodromy matrix in the same deck."
                    .to_owned(),
            ]);
        }
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
    // A dependent card with no carrier at all was refused above, and a `.PSS`
    // that retains no harmonic is refused by `parse_pss` — the engine's card
    // has no `HARMS=0` — so the fallbacks here are only reached when the deck
    // holds no periodic analysis at all, and nothing reads them.
    let shooting_carrier = match &pss_spec {
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
    let mut pnoise_cards = netlist.analyses.iter().filter_map(|command| match command {
        rspice_core::netlist::AnalysisCommand::Pnoise(card) => Some(card),
        _ => None,
    });
    for (line, head, card) in parsed {
        // The basis a dependent card binds is the basis of the family it
        // reads. A card bound to the `.HB` and handed the `.PSS`'s four
        // numbers would be authenticated against a solve it never reads.
        let carrier = match families.get(&line).copied().unwrap_or_default() {
            CarrierSelector::Hb => hb_carrier.unwrap_or(shooting_carrier),
            _ => shooting_carrier,
        };
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
                let sampling = pnoise_cards.next().ok_or_else(|| vec![format!("line {line}: missing parsed PNOISE card")])?.sampling.clone();
                let config = parse_pnoise(
                    &card,
                    sampling,
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
    let (sideband_min, sideband_max) = sideband_range(card, ".PAC", 5, params)?;
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
        sideband_min,
        sideband_max,
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
        carrier: periodic_source_selector(card, ".PAC")?,
    };
    validate_frequency_contract(
        ".PAC",
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sideband_max,
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
    if !config.include_dc && config.sideband_min == 0 && config.sideband_max == 0 {
        return Err(".PAC includedc=no withholds the only sideband this card analyses".to_owned());
    }
    Ok(config)
}

fn parse_pnoise(
    card: &ParsedCard,
    sampling: Option<rspice_core::analysis::pnoise::PeriodicNoiseSampling>,
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
            "inputsideband",
            "outsideband",
            "noiseref",
            "integratednoise",
            "noisesummary",
            "sampling",
            "samplephase",
            "threshold",
            "direction",
            "occurrence",
            "phasetol",
            "minslew",
            "refout",
            "refthreshold",
            "refdirection",
            "refoccurrence",
            "refphasetol",
            "refminslew",
            "periods",
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
    if noise_ref == PnoiseReference::Input && carrier.autonomous && sampling.is_none() {
        return Err(
            ".PNOISE input= refers noise to a driving source, and an autonomous .PSS has none; \
             author noiseref=phase for an oscillator's phase noise"
                .to_owned(),
        );
    }
    let config = PnoiseRunConfig {
        sampling,
        input_sideband: optional_i32(card, "inputsideband", 0, params)?,
        output_sideband: optional_i32(card, "outsideband", 0, params)?,
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
        carrier: periodic_source_selector(card, ".PNOISE")?,
    };
    config.validate_conversion_channels()?;
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
        carrier: periodic_source_selector(card, ".PXF")?,
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

/// Read the engine's `FROM=PSS|HB` selector into the carrier the run states.
///
/// The value the card carries, not a yes/no: the selector is part of the
/// request now, so a deck that names its carrier reaches the engine having
/// named it, and a deck that does not writes no keyword and binds to the
/// preceding periodic solve — the same three positions the form offers.
///
/// Every spelling the engine accepts is accepted here: the two families both
/// have a runner in this crate, so the only refusal left is the engine's own
/// `InvalidChoice`.
fn periodic_source_selector(card: &ParsedCard, directive: &str) -> Result<CarrierSelector, String> {
    let Some(value) = card.keyed.get("from") else {
        return Ok(CarrierSelector::Preceding);
    };
    let spelling = unquote(value).trim();
    CarrierSelector::from_spice_name(spelling)
        // The engine's own `InvalidChoice` on this field, in its own words.
        .ok_or_else(|| format!("{directive} from={spelling:?} must be PSS or HB"))
}

/// The basis the deck's `.HB` card settled, or `None` for a deck without one.
///
/// Read off `AnalysisCommand::Hb` — the command the engine's parser produced —
/// and the deck's own `.OPTIONS HBINT NUMFREQ=`, exactly as the manual
/// reader's `.HB` dispatch builds the harmonic-balance specification it
/// queues. A second reading of the card here could bind a dependent to a basis
/// the queued `.HB` task does not solve on.
fn harmonic_balance_carrier(netlist: &Netlist) -> Option<PeriodicCarrier> {
    use rspice_core::netlist::AnalysisCommand;

    let card = netlist.analyses.iter().find_map(|command| match command {
        AnalysisCommand::Hb(card) => Some(card),
        _ => None,
    })?;
    let config = rspice_core::analysis::HbConfig::from_hb_card(card, &netlist.options).ok()?;
    Some(PeriodicCarrier {
        fundamental_freq: config.fundamental_freq,
        num_harmonics: config.num_harmonics,
        tolerance: config.tolerance,
        // A harmonic-balance orbit's period is the authored tone, never a
        // solver unknown.
        autonomous: false,
    })
}

/// Which lines of the deck carry a `.HB` card.
///
/// The basis above comes from the engine's parsed command, which carries no
/// line; the lines come from the same logical-card scan every other card in
/// this reader is found by. Both are needed: a card without `FROM=` binds to
/// the nearest *preceding* periodic solve, and "preceding" is a statement
/// about where the cards were written.
fn harmonic_balance_card_lines(source: &str) -> Vec<usize> {
    logical_cards(source)
        .into_iter()
        .filter_map(|(line, card)| {
            let head = card.split_whitespace().next()?;
            matches_ignore_ascii_case(head, &[".hb"]).then_some(line)
        })
        .collect()
}

/// Read the sideband range the card states, in either of the engine's two
/// spellings.
///
/// `MAXSIDEBAND=n` is the symmetric range `-n..=n`; `SIDEBANDMIN=`/
/// `SIDEBANDMAX=` state the two ends independently, and the card refuses the
/// two spellings together. An asymmetric range used to be refused here and
/// widened nowhere, because the typed configuration held one symmetric
/// number; it holds both ends now, so the deck's own range is what runs.
///
/// The resolution is `parse_pac_command`'s, line for line: a stated symmetric
/// bound is `(-n, n)`, and otherwise each end is its own authored value or
/// the card's default for that end.
fn sideband_range(
    card: &ParsedCard,
    directive: &str,
    default: i32,
    params: &ParamContext,
) -> Result<(i32, i32), String> {
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
        (Some(bound), None, None) => Ok((-bound, bound)),
        (None, None, None) => Ok((-default, default)),
        (None, minimum, maximum) => {
            let minimum = minimum.unwrap_or(-default);
            let maximum = maximum.unwrap_or(default);
            if minimum > maximum {
                return Err(format!("{directive} sidebandmin= exceeds sidebandmax="));
            }
            Ok((minimum, maximum))
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
            verbose: true,
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
        assert!(verbose, "the form's solver log survives the deck");
    }

    /// `VERBOSE=` reaches the engine from a hand-written deck too.
    #[test]
    fn a_deck_authors_the_solver_log_the_form_now_also_offers() {
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

    /// Every card here is written in the engine's grammar, and the engine
    /// parses this exact source first: `Netlist::parse` is not a fixture step,
    /// it is the same parse the run makes, so a card spelled in any other
    /// dialect would fail before this reader saw it.
    #[test]
    fn periodic_cards_accept_continuations_and_freeze_exact_options() {
        let source = "periodic\n.param F0=1Meg NH=12 NPTS=20\nV1 in 0 SIN(0 1 {F0})\nR1 in out 1k\nC1 out 0 1n\n\
.pss fund={F0} tstabperiods=8 points=128 tol=1e-6 harms={NH}\n\
.pac dec {NPTS} 1k 100Meg maxsideband=7 input=V1 out=V(out,0)\n\
.pnoise dec 10 1 1Meg out=out maxsideband=9\n+ input=V1 from=pss inputsideband=-2 outsideband=1\n\
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
        assert_eq!((pac.sideband_min, pac.sideband_max), (-7, 7));
        assert_eq!(pac.output_ref.as_deref(), Some("0"));
        let pnoise = tasks[3].spec_options.pnoise.as_ref().unwrap();
        assert_eq!(pnoise.max_sideband, 9);
        assert_eq!((pnoise.input_sideband, pnoise.output_sideband), (-2, 1));
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

    /// A harmonic-balance carrier is read, bound and run, on every card of the
    /// family that can read one.
    ///
    /// Core's `.PAC`, `.PNOISE` and `.PXF` grammars all admit `FROM=PSS|HB`,
    /// the plan binds such a card to the deck's preceding `.HB`, and the
    /// engine runs it through `Engine::run_pac_from_hb_with_abort` and its two
    /// siblings. This reader used to refuse all three by name with "no route
    /// in the Studio" — a limitation of this crate stated as a fact about the
    /// card. The route exists now, so the test that pinned the refusal is
    /// replaced by one that drives it: the same three cards, in both deck
    /// shapes, read into a queue whose dependent carries a harmonic-balance
    /// basis, and then run through the three service entries against a real
    /// converged `.HB` operating point.
    #[test]
    fn an_hb_carrier_accepts_the_periodic_dependents_that_can_read_it() {
        use crate::services::simulation_runner::{
            self as svc, HbRunConfig, HbToneRunConfig, run_hb_analysis_with_source_path_and_abort,
        };
        use rspice_core::abort_signal::NoAbort;

        const CIRCUIT: &str =
            "periodic\nV1 in 0 SIN(0 0.001 1Meg) AC 1\nR1 in out 1k\nC1 out 0 159.154943091895p\n";
        const HB: &str = ".hb 1Meg\n";
        const FUNDAMENTAL: f64 = 1.0e6;

        // One converged carrier for all three runs, as the plan hands the one
        // artifact to every dependent bound to that instance.
        let carrier_deck = format!("{CIRCUIT}.end\n");
        let operating_point = run_hb_analysis_with_source_path_and_abort(
            &carrier_deck,
            &HbRunConfig {
                tones: vec![HbToneRunConfig::new(FUNDAMENTAL, 12)],
                reltol: 1.0e-10,
                ..HbRunConfig::default()
            },
            None,
            &NoAbort,
        )
        .expect("the harmonic-balance carrier converges")
        .operating_point;

        for card in [
            ".pxf dec 10 1k 1Meg input=V1 out=out from=hb",
            ".pac dec 10 1k 1Meg input=V1 out=out from=hb",
            ".pnoise dec 10 1k 1Meg out=out from=hb",
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

                let tasks = parse_periodic_tasks(&netlist, &source)
                    .unwrap_or_else(|errors| panic!("`{card}` must read: {errors:?}"));
                let dependent = tasks
                    .iter()
                    .find(|task| {
                        matches!(
                            task.spec,
                            AnalysisSpec::Pac | AnalysisSpec::Pxf | AnalysisSpec::Pnoise
                        )
                    })
                    .unwrap_or_else(|| panic!("`{card}` queues a dependent task"));
                let options = &dependent.spec_options;
                // Bound to the harmonic-balance basis, not to the `.PSS`
                // that may be sitting beside it.
                let basis = options
                    .pac
                    .as_ref()
                    .map(|config| (config.carrier, config.pss_fundamental_freq))
                    .or_else(|| {
                        options
                            .pxf
                            .as_ref()
                            .map(|config| (config.carrier, config.pss_fundamental_freq))
                    })
                    .or_else(|| {
                        options
                            .pnoise
                            .as_ref()
                            .map(|config| (config.carrier, config.pss_fundamental_freq))
                    })
                    .unwrap_or_else(|| panic!("`{card}` freezes its run configuration"));
                assert_eq!(basis.0, CarrierSelector::Hb, "`{card}`");
                assert!(
                    (basis.1 - FUNDAMENTAL).abs() < 1.0,
                    "`{card}` must carry the .HB fundamental, got {}",
                    basis.1
                );

                // And it runs. The service entries below are the ones the
                // dispatch calls once the plan has handed over the artifact.
                if let Some(config) = options.pac.as_ref() {
                    let data = svc::run_pac_analysis_from_hb_with_source_path_and_abort(
                        &carrier_deck,
                        config,
                        operating_point.as_ref(),
                        None,
                        &NoAbort,
                    )
                    .unwrap_or_else(|error| panic!("`{card}` must run: {error}"));
                    assert!(!data.frequencies.is_empty(), "`{card}`");
                    assert!(!data.traces.is_empty(), "`{card}`");
                } else if let Some(config) = options.pxf.as_ref() {
                    let data = svc::run_pxf_analysis_from_hb_with_source_path_and_abort(
                        &carrier_deck,
                        config,
                        operating_point.as_ref(),
                        None,
                        &NoAbort,
                    )
                    .unwrap_or_else(|error| panic!("`{card}` must run: {error}"));
                    assert!(!data.transfer.is_empty(), "`{card}`");
                } else if let Some(config) = options.pnoise.as_ref() {
                    let data = svc::run_pnoise_analysis_from_hb_with_source_path_and_abort(
                        &carrier_deck,
                        config,
                        operating_point.as_ref(),
                        None,
                        &NoAbort,
                    )
                    .unwrap_or_else(|error| panic!("`{card}` must run: {error}"));
                    assert!(
                        data.output_noise.iter().all(|value| *value > 0.0),
                        "`{card}` must publish a positive spectrum"
                    );
                } else {
                    panic!("`{card}` queues one of the three typed configurations");
                }
            }
        }
    }

    /// A card without `FROM=` follows the deck, exactly as the engine does.
    ///
    /// `resolve_periodic_source` binds such a card to the nearest preceding
    /// `.PSS` **or** `.HB`. A deck holding only an `.HB` therefore carries its
    /// dependents on that `.HB`, and a deck that writes a `.PSS` after it
    /// carries them on the `.PSS`.
    #[test]
    fn a_hand_written_hb_then_pac_deck_runs_around_the_harmonic_balance_solution() {
        const CIRCUIT: &str =
            "periodic\nV1 in 0 SIN(0 0.001 1Meg) AC 1\nR1 in out 1k\nC1 out 0 1n\n";
        const PAC: &str = ".pac dec 10 1k 1Meg input=V1 out=out";

        for (solves, expected_fundamental) in [
            (".hb 2Meg\n", 2.0e6),
            (".hb 2Meg\n.pss fund=1Meg\n", 1.0e6),
            (".pss fund=1Meg\n.hb 2Meg\n", 2.0e6),
        ] {
            let source = format!("{CIRCUIT}{solves}{PAC}\n.end\n");
            let netlist = Netlist::parse(&source)
                .unwrap_or_else(|error| panic!("the engine reads `{solves}`: {error}"));
            rspice_core::execution::DeckPlan::from_netlist(
                &netlist,
                &rspice_core::resource::ResourceLimits::default(),
            )
            .unwrap_or_else(|error| panic!("the engine binds the card in `{solves}`: {error}"));

            let tasks = parse_periodic_tasks(&netlist, &source)
                .unwrap_or_else(|errors| panic!("`{solves}` must read: {errors:?}"));
            let config = tasks
                .iter()
                .find_map(|task| task.spec_options.pac.as_ref())
                .unwrap_or_else(|| panic!("`{solves}` queues a .PAC configuration"));
            assert_eq!(config.carrier, CarrierSelector::Preceding, "`{solves}`");
            assert!(
                (config.pss_fundamental_freq - expected_fundamental).abs() < 1.0,
                "`{solves}` must bind the periodic solve written last, got {}",
                config.pss_fundamental_freq
            );
        }
    }

    /// `.PSTB` after a harmonic balance is refused in the engine's own words.
    ///
    /// The engine's `resolve_periodic_source` never sees this card: the plan
    /// binds `.PSTB` to the preceding `.PSS` unconditionally, because
    /// `PssAnalysisResult` is the only thing carrying a monodromy matrix. A
    /// deck whose only periodic solve is an `.HB` is refused there, and the
    /// clause this asserts is the engine's own — taken off
    /// `DeckPlan::from_netlist` rather than written out a second time.
    #[test]
    fn a_pstb_after_harmonic_balance_is_refused_in_the_engines_words() {
        const CIRCUIT: &str = "periodic\nV1 in 0 SIN(0 1 1Meg)\nL1 in out 1u\nR1 out 0 1k\n";
        let source = format!("{CIRCUIT}.hb 1Meg\n.pstb probe=L1\n.end\n");
        let netlist = Netlist::parse(&source).expect("the engine reads the deck");
        let engine_error = rspice_core::execution::DeckPlan::from_netlist(
            &netlist,
            &rspice_core::resource::ResourceLimits::default(),
        )
        .expect_err("a harmonic-balance carrier has no monodromy matrix")
        .to_string();
        assert!(
            engine_error.contains("monodromy matrix"),
            "the engine's own refusal must name the missing object: {engine_error}"
        );

        let errors = parse_periodic_tasks(&netlist, &source)
            .expect_err("the Studio refuses the deck the engine refuses");
        assert!(
            errors
                .iter()
                .any(|error| error.contains("monodromy matrix") && error.contains(".PSS")),
            "the Studio must answer in the engine's words: {errors:?}"
        );
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
        const PAC_BASE: &str = ".pac dec 10 1k 1Meg input=V1 out=out";
        const PNOISE_BASE: &str = ".pnoise dec 10 1 1Meg out=out";

        const PXF_EVERY_KEY: &str = ".pxf dec 10 1k 1Meg input=V1 out=out inputsideband=-1 \
             outsideband=2 maxsideband=4 reltol=1e-4 abstol=1e-14 from=pss";
        const PSTB_EVERY_KEY: &str = ".pstb probe=l1 maxharm=8 nmults=6 stabilitythreshold=1.5 \
             detectsubharmonics=no eigentol=1e-9";
        // `MAXSIDEBAND=` is deliberately absent beside the two ends: the card
        // refuses the two spellings together, so "every key" on `.PAC` is
        // every key one line may carry at once.
        const PAC_EVERY_KEY: &str = ".pac dec 10 1k 1Meg input=V1 out=out sidebandmin=-2 \
             sidebandmax=4 reltol=1e-5 abstol=1e-15 pacmag=0.05 includedc=no from=pss";
        const PNOISE_EVERY_KEY: &str = ".pnoise dec 10 1 1Meg out=out maxsideband=3 \
             noiseref=output integratednoise=yes noisesummary=no from=pss";

        let mut cards = vec![
            PXF_EVERY_KEY.to_owned(),
            PSTB_EVERY_KEY.to_owned(),
            PAC_EVERY_KEY.to_owned(),
            PNOISE_EVERY_KEY.to_owned(),
        ];
        // One line per arm of `parse_pac_command`, `parse_pnoise_command`,
        // `parse_pxf_command` and `parse_pstb_command`. `INPUT=`/`OUT=`/
        // `PROBE=` are in the bases above because no card defaults them.
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
            (PAC_BASE, "maxsideband", "3"),
            (PAC_BASE, "sidebandmin", "-2"),
            (PAC_BASE, "sidebandmax", "4"),
            (PAC_BASE, "reltol", "1e-5"),
            (PAC_BASE, "abstol", "1e-15"),
            (PAC_BASE, "pacmag", "0.05"),
            (PAC_BASE, "includedc", "no"),
            (PAC_BASE, "from", "pss"),
            (PNOISE_BASE, "maxsideband", "3"),
            (PNOISE_BASE, "noiseref", "output"),
            (PNOISE_BASE, "integratednoise", "yes"),
            (PNOISE_BASE, "noisesummary", "no"),
            (PNOISE_BASE, "from", "pss"),
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
        assert_eq!(pxf.carrier, CarrierSelector::Pss);
    }

    #[test]
    fn autonomous_sampled_pnoise_studio_manual_deck_retains_timing_and_input_reference() {
        let source = "sampled oscillator\nIprobe 0 osc dc 0\nR1 osc 0 1k\n.pss autonomous=yes oscnode=osc periodguess=1m\n\
            .pnoise lin 3 10 100 out=osc sampling=edge input=Iprobe inputsideband=1 maxsideband=4 integratednoise=yes\n.end\n";
        let netlist = Netlist::parse(source).unwrap();
        let tasks = parse_periodic_tasks(&netlist, source).unwrap();
        let config = tasks
            .iter()
            .find_map(|task| task.spec_options.pnoise.as_ref())
            .unwrap();
        assert_eq!(config.noise_ref, PnoiseReference::Input);
        assert_eq!(config.input_source, "Iprobe");
        assert_eq!(config.input_sideband, 1);
        assert!(config.integrated_noise);
        assert!(matches!(
            &config.sampling,
            Some(rspice_core::analysis::pnoise::PeriodicNoiseSampling::Edge { .. })
        ));
    }

    #[test]
    fn sampled_pnoise_studio_manual_deck_preserves_each_request_in_order() {
        let source = "sampled manual\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.pss fund=1k\n\
                      .pnoise lin 3 10 100 out=out sampling=phase samplephase=37\n\
                      .pnoise lin 3 10 100 out=out sampling=delay threshold=0.3 direction=falling occurrence=2 phasetol=1u minslew=2k refout=V(clk,ref) refthreshold=0.7 refdirection=either refoccurrence=3 refphasetol=2u refminslew=3k periods=4\n.end\n";
        let netlist = Netlist::parse(source).unwrap();
        let expected: Vec<_> = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                rspice_core::netlist::AnalysisCommand::Pnoise(card) => Some(card.sampling.clone()),
                _ => None,
            })
            .collect();
        let tasks = parse_periodic_tasks(&netlist, source).unwrap();
        let actual: Vec<_> = tasks
            .iter()
            .filter_map(|task| {
                task.spec_options
                    .pnoise
                    .as_ref()
                    .map(|config| config.sampling.clone())
            })
            .collect();
        assert_eq!(actual, expected);
        assert_eq!(actual.len(), 2);
    }

    /// Every keyword this lane taught the form reaches the same typed request
    /// from a hand-written deck, valued as the deck authored it.
    ///
    /// Accepting a key is not carrying it, and three of these were accepted
    /// and dropped: `FROM=` was read only to decide whether to refuse the
    /// card, and an asymmetric `SIDEBANDMIN=`/`SIDEBANDMAX=` pair was refused
    /// outright because the typed request held one symmetric number. So each
    /// is read back off the queued configuration rather than merely tolerated.
    #[test]
    fn the_reader_carries_the_carrier_the_sideband_ends_and_the_tolerances() {
        const CIRCUIT: &str = "periodic\nV1 in 0 SIN(0 1 1Meg)\nR1 in out 1k\nC1 out 0 1n\n";
        const DECK: &str = ".pss fund=1Meg\n\
             .pac dec 10 1k 1Meg input=V1 out=out sidebandmin=-2 sidebandmax=4 reltol=1e-5 \
             abstol=1e-15 from=pss\n\
             .pxf dec 10 1k 1Meg input=V1 out=out reltol=2e-5 abstol=2e-15 from=pss\n\
             .pnoise dec 10 1 1Meg out=out from=pss\n";

        let source = format!("{CIRCUIT}{DECK}.end\n");
        let netlist = Netlist::parse(&source).expect("the engine reads the deck");
        let tasks = parse_periodic_tasks(&netlist, &source).expect("and so does the studio");

        let pac = tasks
            .iter()
            .find_map(|task| task.spec_options.pac.as_ref())
            .expect("the .PAC card is queued");
        assert_eq!(
            (pac.sideband_min, pac.sideband_max),
            (-2, 4),
            "an asymmetric range is the range the deck asked for"
        );
        assert_eq!(pac.carrier, CarrierSelector::Pss);
        assert!((pac.reltol - 1.0e-5).abs() <= 1.0e-20, "{}", pac.reltol);
        assert!((pac.abstol - 1.0e-15).abs() <= 1.0e-30, "{}", pac.abstol);

        let pxf = tasks
            .iter()
            .find_map(|task| task.spec_options.pxf.as_ref())
            .expect("the .PXF card is queued");
        assert_eq!(pxf.carrier, CarrierSelector::Pss);
        assert!((pxf.reltol - 2.0e-5).abs() <= 1.0e-20, "{}", pxf.reltol);

        let pnoise = tasks
            .iter()
            .find_map(|task| task.spec_options.pnoise.as_ref())
            .expect("the .PNOISE card is queued");
        assert_eq!(pnoise.carrier, CarrierSelector::Pss);

        // A deck that names no carrier binds to the preceding periodic solve,
        // which is the third position rather than a default spelling of the
        // one above.
        let unnamed =
            format!("{CIRCUIT}.pss fund=1Meg\n.pac dec 10 1k 1Meg input=V1 out=out\n.end\n");
        let netlist = Netlist::parse(&unnamed).expect("the engine reads the unnamed deck");
        let tasks = parse_periodic_tasks(&netlist, &unnamed).expect("and so does the studio");
        let pac = tasks
            .iter()
            .find_map(|task| task.spec_options.pac.as_ref())
            .expect("the .PAC card is queued");
        assert_eq!(pac.carrier, CarrierSelector::Preceding);
        assert_eq!((pac.sideband_min, pac.sideband_max), (-5, 5));
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

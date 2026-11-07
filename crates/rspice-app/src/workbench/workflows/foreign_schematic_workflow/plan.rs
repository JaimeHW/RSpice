//! A drawing's directive cards as simulation-plan content.
//!
//! Each analysis card becomes the plan's draft for that analysis, written in
//! the draft's own fields so the plan validates and emits it like one typed
//! by hand. A constant `.param` is a design variable; a parameter expression,
//! `.model`, `.func` and subcircuit text is model-library text. Solver
//! options and a single `.temp` set the plan's options. A card none of these
//! can hold stays on the sheet, with the reason.

use rspice_design::foreign_schematic::{ForeignCard, ForeignCardRole};
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_simulation_contract::config::NoiseSweepType;
use rspice_simulation_contract::options::{IntegrationMethod, SimulationOptions, format_si_value};
use rspice_simulation_contract::spice_value::parse_spice_value_checked;

/// What a drawing's directives become.
#[derive(Debug, Default)]
pub(super) struct ForeignPlan {
    pub(super) analyses: Vec<(ForeignCard, AnalysisDraft)>,
    /// Constant parameters, as design variables: name and value.
    pub(super) variables: Vec<(String, String)>,
    pub(super) options: SimulationOptions,
    pub(super) temperature: Option<f64>,
    /// Cards for the project's model library.
    pub(super) model_text: Vec<String>,
    /// `.include` and `.lib` cards, for the files they name.
    pub(super) includes: Vec<ForeignCard>,
    /// Cards that stay on the sheet, and why.
    pub(super) kept: Vec<(ForeignCard, String)>,
    pub(super) notices: Vec<String>,
}

/// LTspice's and QSPICE's options for their own waveform files and printed
/// numbers, which say nothing about the circuit or its solution.
const OUTPUT_OPTIONS: [&str; 10] = [
    "plotwinsize",
    "numdgt",
    "measdgt",
    "nomarch",
    "plotreltol",
    "plotvntol",
    "plotabstol",
    "nopage",
    "noacct",
    "binary",
];

pub(super) fn plan_from_cards(cards: Vec<ForeignCard>) -> ForeignPlan {
    let mut plan = ForeignPlan::default();
    for card in cards {
        match card.role {
            ForeignCardRole::Analysis => match analysis_draft(&card) {
                Ok((draft, ignored)) => {
                    if !ignored.is_empty() {
                        plan.notices.push(format!(
                            "`{}`: `{}` has no RSpice equivalent and was not applied.",
                            card.text,
                            ignored.join(" ")
                        ));
                    }
                    plan.analyses.push((card, draft));
                }
                Err(reason) => plan.kept.push((card, reason)),
            },
            ForeignCardRole::Parameter => parameters(&mut plan, card),
            ForeignCardRole::Options => options(&mut plan, card),
            ForeignCardRole::Temperature => {
                let values = card
                    .tokens
                    .iter()
                    .skip(1)
                    .map(|word| parse_spice_value_checked(word))
                    .collect::<Result<Vec<_>, _>>();
                match values.as_deref() {
                    Ok([value]) => plan.temperature = Some(*value),
                    _ => plan.kept.push((
                        card,
                        "a temperature list sweeps every analysis, which the plan states as a \
                         temperature analysis of its own"
                            .to_owned(),
                    )),
                }
            }
            ForeignCardRole::Step => plan.kept.push((
                card,
                "a stepped run is a sweep of the plan's run set, set on the Analyses page"
                    .to_owned(),
            )),
            ForeignCardRole::Measurement => plan.kept.push((
                card,
                "a measurement is a specification of the plan, set on the Specs page".to_owned(),
            )),
            ForeignCardRole::ModelText => plan.model_text.push(card.text),
            ForeignCardRole::Include => plan.includes.push(card),
            // `.backanno` and `.end` tell the source program alone something.
            ForeignCardRole::SourceOnly => {}
            ForeignCardRole::Other => {
                let reason = match card.keyword().as_str() {
                    ".plot" | ".probe" | ".save" | ".print" => {
                        "RSpice keeps every waveform, and a probe chooses what is plotted"
                            .to_owned()
                    }
                    keyword => format!("RSpice has no project place for `{keyword}`"),
                };
                plan.kept.push((card, reason));
            }
        }
    }
    plan
}

fn parameters(plan: &mut ForeignPlan, card: ForeignCard) {
    let mut expressions = Vec::new();
    for word in card.tokens.iter().skip(1) {
        let Some((name, value)) = word.split_once('=') else {
            plan.kept.push((
                card.clone(),
                format!("`{word}` is not a name=value assignment"),
            ));
            return;
        };
        let value = value.trim();
        if parse_spice_value_checked(value).is_ok() {
            plan.variables
                .push((name.trim().to_owned(), value.to_owned()));
        } else {
            expressions.push(format!("{}={value}", name.trim()));
        }
    }
    if !expressions.is_empty() {
        plan.model_text
            .push(format!(".param {}", expressions.join(" ")));
    }
}

fn options(plan: &mut ForeignPlan, card: ForeignCard) {
    let mut unknown = Vec::new();
    let mut output = Vec::new();
    for word in card.tokens.iter().skip(1) {
        let (key, value) = match word.split_once('=') {
            Some((key, value)) => (key.trim().to_ascii_lowercase(), Some(value.trim())),
            None => (word.to_ascii_lowercase(), None),
        };
        if OUTPUT_OPTIONS.contains(&key.as_str()) {
            output.push(word.clone());
            continue;
        }
        let number = value.and_then(|value| parse_spice_value_checked(value).ok());
        let options = &mut plan.options;
        let applied = match (key.as_str(), number, value) {
            ("reltol", Some(number), _) => {
                options.reltol = number;
                true
            }
            ("abstol", Some(number), _) => {
                options.abstol = number;
                true
            }
            ("vntol", Some(number), _) => {
                options.vntol = number;
                true
            }
            ("chgtol", Some(number), _) => {
                options.chgtol = number;
                true
            }
            ("gmin", Some(number), _) => {
                options.gmin = number;
                true
            }
            ("trtol", Some(number), _) => {
                options.trtol = number;
                true
            }
            ("tnom", Some(number), _) => {
                options.tnom = number;
                true
            }
            ("itl1", Some(number), _) if number >= 1.0 => {
                options.itl1 = number as usize;
                true
            }
            ("itl4", Some(number), _) if number >= 1.0 => {
                options.itl4 = number as usize;
                true
            }
            ("method", _, Some(method)) => match method.to_ascii_lowercase().as_str() {
                "trap" | "trapezoidal" => {
                    options.method = IntegrationMethod::Trap;
                    true
                }
                "gear" => {
                    options.method = IntegrationMethod::Gear2;
                    true
                }
                "euler" => {
                    options.method = IntegrationMethod::Euler;
                    true
                }
                _ => false,
            },
            _ => false,
        };
        if !applied {
            unknown.push(word.clone());
        }
    }
    if !output.is_empty() {
        plan.notices.push(format!(
            "`{}` sets how the source program writes its own files and has no meaning in RSpice.",
            output.join(" ")
        ));
    }
    if !unknown.is_empty() {
        let mut kept = card;
        kept.text = format!("{} {}", kept.tokens[0], unknown.join(" "));
        plan.kept.push((
            kept,
            "the plan's options have no field for these settings".to_owned(),
        ));
    }
}

/// The plan draft for one analysis card, and the card's words RSpice has no
/// equivalent for.
fn analysis_draft(card: &ForeignCard) -> Result<(AnalysisDraft, Vec<String>), String> {
    let words = &card.tokens[1..];
    let number = |word: &str| {
        parse_spice_value_checked(word).map_err(|_| format!("`{word}` is not a number"))
    };
    let mut ignored = Vec::new();
    let draft = match card.keyword().as_str() {
        ".op" => AnalysisDraft::for_kind(AnalysisKind::OperatingPoint),
        ".tran" => {
            let mut uic = false;
            let mut values = Vec::new();
            for word in words {
                match word.to_ascii_lowercase().as_str() {
                    "uic" => uic = true,
                    "startup" | "steady" | "nodiscard" | "step" => ignored.push(word.clone()),
                    _ => values.push(word.as_str()),
                }
            }
            let (step, stop, start, max_step) = match values.as_slice() {
                [stop] => (None, *stop, None, None),
                [step, stop] => (Some(*step), *stop, None, None),
                [step, stop, start] => (Some(*step), *stop, Some(*start), None),
                [step, stop, start, max_step] => {
                    (Some(*step), *stop, Some(*start), Some(*max_step))
                }
                _ => return Err("a transient card names one to four times".to_owned()),
            };
            let stop_time = number(stop)?;
            let step = match step {
                Some(step) if number(step)? > 0.0 => step.to_owned(),
                // The source program reads a zero print step as "choose one".
                _ => format_si_value(stop_time / 1000.0),
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::Transient);
            if let AnalysisDraft::Transient(tran) = &mut draft {
                tran.stop = stop.to_owned();
                tran.step = step;
                tran.start = start.unwrap_or("0").to_owned();
                tran.max_step = match max_step {
                    Some(max_step) if number(max_step)? > 0.0 => max_step.to_owned(),
                    _ => "auto".to_owned(),
                };
                tran.uic = uic;
            }
            draft
        }
        ".ac" => {
            let [mode, points, start, stop] = words else {
                return Err("only a dec, oct or lin frequency sweep is a plan AC analysis".into());
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::Ac);
            if let AnalysisDraft::Ac(ac) = &mut draft {
                ac.sweep = sweep_index(mode)?;
                ac.points = points.clone();
                ac.fstart = start.clone();
                ac.fstop = stop.clone();
            }
            draft
        }
        ".dc" => {
            let (mode, axes) = match words.first().map(|word| word.to_ascii_lowercase()) {
                Some(word) if word == "lin" => (0, &words[1..]),
                Some(word) if word == "dec" => (2, &words[1..]),
                Some(word) if word == "oct" => (3, &words[1..]),
                _ => (0, words),
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::DcSweep);
            if let AnalysisDraft::DcSweep(dc) = &mut draft {
                dc.mode = mode;
                match axes {
                    [source, start, stop, step, rest @ ..] => {
                        dc.source = source.clone();
                        dc.start = start.clone();
                        dc.stop = stop.clone();
                        dc.step = step.clone();
                        dc.points = step.clone();
                        match rest {
                            [] => dc.nested = false,
                            [source2, start2, stop2, step2] => {
                                dc.nested = true;
                                dc.source2 = source2.clone();
                                dc.start2 = start2.clone();
                                dc.stop2 = stop2.clone();
                                dc.step2 = step2.clone();
                            }
                            _ => return Err("a DC sweep card has one or two sweeps".into()),
                        }
                    }
                    _ => return Err("a DC sweep card names a source, start, stop and step".into()),
                }
            }
            draft
        }
        ".noise" => {
            let [output, input, mode, points, start, stop] = words else {
                return Err("a noise card names an output, a source and a frequency sweep".into());
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::Noise);
            if let AnalysisDraft::Noise(noise) = &mut draft {
                noise.output = output.clone();
                noise.input = input.clone();
                noise.sweep = match sweep_index(mode)? {
                    0 => NoiseSweepType::Decade,
                    1 => NoiseSweepType::Octave,
                    _ => NoiseSweepType::Linear,
                };
                noise.points = points.clone();
                noise.fstart = start.clone();
                noise.fstop = stop.clone();
            }
            draft
        }
        ".tf" => {
            let [output, input] = words else {
                return Err("a transfer-function card names an output and a source".into());
            };
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::TransferFunction);
            if let AnalysisDraft::TransferFunction(xf) = &mut draft {
                xf.output_expression = output.clone();
                xf.input_source = input.clone();
            }
            draft
        }
        other => return Err(format!("`{other}` is not a plan analysis")),
    };
    Ok((draft, ignored))
}

fn sweep_index(mode: &str) -> Result<usize, String> {
    match mode.to_ascii_lowercase().as_str() {
        "dec" => Ok(0),
        "oct" => Ok(1),
        "lin" => Ok(2),
        _ => Err(format!("`{mode}` is not a dec, oct or lin sweep")),
    }
}

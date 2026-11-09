//! A drawing's directive cards as simulation-plan content.
//!
//! Each analysis card becomes the plan's draft for that analysis, written in
//! the draft's own fields so the plan validates and emits it like one typed
//! by hand. A constant `.param` is a design variable; a parameter expression,
//! `.model`, `.func` and subcircuit text is model-library text. Solver
//! options and a single `.temp` set the plan's options. A stepped run
//! (`.step`, or a `.temp` list) is an axis of the plan's run set, and a
//! `.meas` card a measurement the plan defines. A card none of these can
//! hold stays on the sheet, with the reason.

use rspice_design::foreign_schematic::{ForeignCard, ForeignCardRole, ForeignTool};
use rspice_design::schematic::component_type::ComponentType;
use rspice_design::schematic::document::SchematicDocument;
use rspice_results::specification::{
    SpecEntry, SpecPointScope, SpecificationDefinition, SpecificationRole,
};
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_simulation_contract::config::NoiseSweepType;
use rspice_simulation_contract::options::{IntegrationMethod, SimulationOptions, format_si_value};
use rspice_simulation_contract::run_set::{
    DESIGN_VARIABLE_SOURCE_PREFIX, NETLIST_SUPPLY_SOURCE_PREFIX, RunSetDimension,
    RunSetDimensionKind,
};
use rspice_simulation_contract::spice_value::parse_spice_value_checked;

/// What a drawing's directives become.
#[derive(Debug, Default)]
pub(super) struct ForeignPlan {
    pub(super) analyses: Vec<(ForeignCard, AnalysisDraft)>,
    /// Constant parameters, as design variables: name and value.
    pub(super) variables: Vec<(String, String)>,
    pub(super) options: SimulationOptions,
    pub(super) temperature: Option<f64>,
    /// Axes of the plan's run set, each with the card that stated it.
    pub(super) sweeps: Vec<(ForeignCard, RunSetDimension)>,
    /// Measurements the plan defines, each with its card.
    pub(super) measurements: Vec<(ForeignCard, SpecificationDefinition)>,
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

/// The plan a drawing's cards state. `tool` wrote the drawing; `document` is
/// the imported drawing, whose sources a stepped source names; `invalid` says
/// why the plan would refuse an analysis draft.
pub(super) fn plan_from_cards(
    cards: Vec<ForeignCard>,
    tool: ForeignTool,
    document: &SchematicDocument,
    invalid: impl Fn(&AnalysisDraft) -> Option<String>,
) -> ForeignPlan {
    let mut plan = ForeignPlan::default();
    let mut steps = Vec::new();
    let mut measurements = Vec::new();
    for card in cards {
        match card.role {
            ForeignCardRole::Analysis => match analysis_draft(&card, tool) {
                Ok((draft, ignored)) => {
                    if !ignored.is_empty() {
                        plan.notices.push(format!(
                            "`{}`: `{}` has no RSpice equivalent and was not applied.",
                            card.text,
                            ignored.join(" ")
                        ));
                    }
                    match invalid(&draft) {
                        Some(error) => plan.kept.push((card, error)),
                        None => plan.analyses.push((card, draft)),
                    }
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
                    Ok([]) | Err(_) => plan.kept.push((
                        card,
                        "it does not list temperatures in degrees Celsius".to_owned(),
                    )),
                    // A list runs every analysis at each temperature, as a
                    // stepped temperature does.
                    Ok(_) => steps.push(card),
                }
            }
            ForeignCardRole::Step => steps.push(card),
            ForeignCardRole::Measurement => measurements.push(card),
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
    for card in steps {
        match sweep(&card, document) {
            Ok(dimension) => add_sweep(&mut plan, card, dimension),
            Err(reason) => plan.kept.push((card, reason)),
        }
    }
    for card in measurements {
        match measurement(&plan, &card) {
            Ok(definition) => plan.measurements.push((card, definition)),
            Err(reason) => plan.kept.push((card, reason)),
        }
    }
    // A sweep or a measurement applies to the plan's analyses; with none,
    // the source program would run nothing either.
    if plan.analyses.is_empty() {
        let reason = || "the drawing states no analysis it applies to".to_owned();
        let sweeps = std::mem::take(&mut plan.sweeps);
        let measured = std::mem::take(&mut plan.measurements);
        plan.kept
            .extend(sweeps.into_iter().map(|(card, _)| (card, reason())));
        plan.kept
            .extend(measured.into_iter().map(|(card, _)| (card, reason())));
    }
    plan
}

/// A card's text on one line, continuation marks and `;` comments removed.
fn one_line(card: &ForeignCard) -> String {
    card.text
        .lines()
        .map(|line| {
            let line = line.trim();
            let line = line.strip_prefix('+').unwrap_or(line);
            line.split(';').next().unwrap_or_default().trim()
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// The run-set axis a `.step` card, or a `.temp` list, states.
fn sweep(card: &ForeignCard, document: &SchematicDocument) -> Result<RunSetDimension, String> {
    let text = one_line(card);
    let words = text.split_whitespace().skip(1).collect::<Vec<_>>();
    if card.keyword() == ".temp" {
        return Ok(axis(RunSetDimensionKind::Temperature, "", &words));
    }
    let lower = |index: usize| words.get(index).map(|word| word.to_ascii_lowercase());
    let (scale, words) = match lower(0).as_deref() {
        Some(scale @ ("lin" | "dec" | "oct")) => (Scale::from(scale), &words[1..]),
        _ => (Scale::Linear, &words[..]),
    };
    let (kind, name, values) = match words {
        [param, name, values @ ..] if param.eq_ignore_ascii_case("param") => {
            (RunSetDimensionKind::Parameter, *name, values)
        }
        [temp, values @ ..] if temp.eq_ignore_ascii_case("temp") => {
            (RunSetDimensionKind::Temperature, "", values)
        }
        [source, values @ ..]
            if values.first().is_some_and(|word| !word.contains('('))
                && independent_source(document, source) =>
        {
            (RunSetDimensionKind::Source, *source, values)
        }
        [source, ..] if is_source_name(source) => {
            return Err(format!(
                "a stepped source sets its DC value, and `{source}` is not a drawn source with one"
            ));
        }
        _ => {
            return Err(
                "a stepped model parameter has no run-set axis; RSpice steps design variables, \
                 source values and temperature"
                    .to_owned(),
            );
        }
    };
    let values = match values {
        [list, values @ ..] if list.eq_ignore_ascii_case("list") && !values.is_empty() => {
            values.iter().map(|value| (*value).to_owned()).collect()
        }
        [start, stop, increment] => scale.values(start, stop, increment)?,
        _ => {
            return Err("it names neither a value list nor a start, stop and increment".to_owned());
        }
    };
    let values = values.iter().map(String::as_str).collect::<Vec<_>>();
    Ok(axis(kind, name, &values))
}

fn axis(kind: RunSetDimensionKind, name: &str, values: &[&str]) -> RunSetDimension {
    let mut dimension = RunSetDimension::new(String::new(), kind, values, 1);
    match kind {
        RunSetDimensionKind::Parameter => {
            dimension.name = name.to_owned();
            dimension.source = format!("{DESIGN_VARIABLE_SOURCE_PREFIX}{name}");
        }
        RunSetDimensionKind::Source => {
            dimension.name = name.to_owned();
            dimension.source = format!("{NETLIST_SUPPLY_SOURCE_PREFIX}{name}");
        }
        _ => {}
    }
    dimension
}

fn is_source_name(name: &str) -> bool {
    name.chars()
        .next()
        .is_some_and(|first| matches!(first.to_ascii_uppercase(), 'V' | 'I'))
}

/// Whether the drawing holds an independent source of that name whose value
/// is a DC level, the value a stepped source replaces.
fn independent_source(document: &SchematicDocument, name: &str) -> bool {
    document.components.iter().any(|component| {
        matches!(
            component.kind,
            ComponentType::VoltageSource | ComponentType::CurrentSource
        ) && component.name.eq_ignore_ascii_case(name)
            && {
                let mut words = component.value.split_whitespace();
                let first = words.next().unwrap_or_default();
                let level = if first.eq_ignore_ascii_case("dc") {
                    words.next().unwrap_or_default()
                } else {
                    first
                };
                parse_spice_value_checked(level).is_ok()
            }
    })
}

#[derive(Debug, Clone, Copy)]
enum Scale {
    Linear,
    Decade,
    Octave,
}

impl From<&str> for Scale {
    fn from(word: &str) -> Self {
        match word {
            "dec" => Self::Decade,
            "oct" => Self::Octave,
            _ => Self::Linear,
        }
    }
}

impl Scale {
    /// Most values one stepped axis expands to.
    const MAXIMUM_VALUES: usize = 10_000;

    /// The values from `start` to `stop`, the source program's way: by
    /// `increment` on a linear scale, `increment` points per decade or
    /// octave on a logarithmic one.
    fn values(self, start: &str, stop: &str, increment: &str) -> Result<Vec<String>, String> {
        let number = |word: &str| {
            parse_spice_value_checked(word).map_err(|_| format!("`{word}` is not a number"))
        };
        let (start, stop, increment) = (number(start)?, number(stop)?, number(increment)?);
        let mut values = Vec::new();
        match self {
            Self::Linear => {
                if increment == 0.0 || (stop - start) * increment < 0.0 {
                    return Err("its increment does not lead from start to stop".to_owned());
                }
                let count = ((stop - start) / increment + 1e-9).floor() as usize + 1;
                if count > Self::MAXIMUM_VALUES {
                    return Err(format!("it steps {count} values"));
                }
                values.extend((0..count).map(|index| start + index as f64 * increment));
            }
            Self::Decade | Self::Octave => {
                let base: f64 = if matches!(self, Self::Decade) {
                    10.0
                } else {
                    2.0
                };
                if start <= 0.0 || stop < start || increment < 1.0 {
                    return Err(
                        "a logarithmic step runs upward from a positive start, one point or more \
                         per interval"
                            .to_owned(),
                    );
                }
                let count = ((stop / start).log(base) * increment + 1e-9).floor() as usize + 1;
                if count > Self::MAXIMUM_VALUES {
                    return Err(format!("it steps {count} values"));
                }
                values.extend((0..count).map(|index| start * base.powf(index as f64 / increment)));
            }
        }
        // Thirteen significant digits drop the binary residue of the sums.
        Ok(values
            .into_iter()
            .map(|value| {
                let rounded = format!("{value:.12e}").parse().unwrap_or(value);
                format_si_value(rounded)
            })
            .collect())
    }
}

/// Adds an axis unless another already states its parameter, source or
/// temperature, and makes a stepped parameter the drawing never sets a
/// design variable at its first value.
fn add_sweep(plan: &mut ForeignPlan, card: ForeignCard, mut dimension: RunSetDimension) {
    let taken = plan.sweeps.iter().any(|(_, existing)| {
        existing.kind == dimension.kind && existing.source.eq_ignore_ascii_case(&dimension.source)
    });
    if taken {
        plan.kept.push((
            card,
            "another card already steps the same quantity".to_owned(),
        ));
        return;
    }
    if dimension.kind == RunSetDimensionKind::Parameter {
        let name = dimension.name.clone();
        let defined = plan
            .variables
            .iter()
            .any(|(variable, _)| variable.eq_ignore_ascii_case(&name))
            || plan
                .model_text
                .iter()
                .any(|text| defines_parameter(text, &name));
        if !defined && let Some(first) = dimension.values.first() {
            plan.variables.push((name, first.lexical.clone()));
        }
    }
    dimension.id = format!("dimension-step-{}", plan.sweeps.len() + 1);
    let id = dimension.id.clone();
    for (index, value) in dimension.values.iter_mut().enumerate() {
        value.id = rspice_simulation_contract::run_set::value_id(&id, index);
    }
    plan.sweeps.push((card, dimension));
}

fn defines_parameter(text: &str, name: &str) -> bool {
    let mut words = text.split_whitespace();
    words
        .next()
        .is_some_and(|word| word.eq_ignore_ascii_case(".param"))
        && words.any(|word| {
            word.split('=')
                .next()
                .is_some_and(|defined| defined.trim().eq_ignore_ascii_case(name))
        })
}

/// The analysis families whose results a `.MEAS` card reads.
const MEASURED_FAMILIES: [&str; 4] = ["TRAN", "AC", "DC", "NOISE"];

/// The plan measurement a `.meas` card defines. A card that names no
/// analysis measures the plan's one measured analysis, as the source
/// program measures whichever analysis it runs.
fn measurement(plan: &ForeignPlan, card: &ForeignCard) -> Result<SpecificationDefinition, String> {
    let text = one_line(card);
    let (_, rest) = text
        .split_once(char::is_whitespace)
        .ok_or_else(|| "it names no measurement".to_owned())?;
    let (first, after_first) = rest
        .trim_start()
        .split_once(char::is_whitespace)
        .ok_or_else(|| "it states no measurement".to_owned())?;
    let named = ["AC", "DC", "OP", "TRAN", "TF", "NOISE"]
        .iter()
        .find(|family| family.eq_ignore_ascii_case(first));
    let (family, body) = match named {
        Some(family) => ((*family).to_owned(), after_first.trim_start()),
        None => match measured_families(plan).as_slice() {
            [family] => ((*family).to_owned(), rest.trim_start()),
            [] => return Err("the plan has no analysis it measures".to_owned()),
            _ => {
                return Err(
                    "it names no analysis and the plan runs several it could measure".to_owned(),
                );
            }
        },
    };
    if !MEASURED_FAMILIES.contains(&family.as_str()) {
        return Err(format!(
            "RSpice measures transient, AC, DC and noise results, not {family}"
        ));
    }
    if !measured_families(plan).contains(&family.as_str()) {
        return Err(format!("the plan runs no {family} analysis"));
    }
    let name = body
        .split_whitespace()
        .next()
        .ok_or_else(|| "it names no measurement".to_owned())?;
    let statement = format!(".MEAS {family} {body}");
    check_measurement(plan, &statement)?;
    let mut definition = SpecificationDefinition::new_from_projection(&SpecEntry {
        measurement: name.to_owned(),
        expression: statement,
        min: None,
        max: None,
        unit: String::new(),
        scope: SpecPointScope::AllPoints,
    });
    definition.define_measurement = true;
    definition.role = SpecificationRole::Informational;
    definition.validate()?;
    Ok(definition)
}

/// The measured families of the plan's analyses, each once.
fn measured_families(plan: &ForeignPlan) -> Vec<&'static str> {
    let mut families = plan
        .analyses
        .iter()
        .filter_map(|(_, draft)| measured_family(draft))
        .collect::<Vec<_>>();
    families.sort_unstable();
    families.dedup();
    families
}

fn measured_family(draft: &AnalysisDraft) -> Option<&'static str> {
    match draft {
        AnalysisDraft::Transient(_) => Some("TRAN"),
        AnalysisDraft::Ac(_) => Some("AC"),
        AnalysisDraft::DcSweep(_) => Some("DC"),
        AnalysisDraft::Noise(_) => Some("NOISE"),
        _ => None,
    }
}

/// Whether the engine reads the statement, beside the plan's parameters and
/// the measurements accepted before it, so a run never meets a card it
/// refuses.
fn check_measurement(plan: &ForeignPlan, statement: &str) -> Result<(), String> {
    let mut deck = String::from("* measurement\nR1 1 0 1\n");
    for (name, value) in &plan.variables {
        deck.push_str(&format!(".param {name}={value}\n"));
    }
    for (_, dimension) in &plan.sweeps {
        if dimension.kind == RunSetDimensionKind::Parameter
            && let Some(first) = dimension.values.first()
        {
            deck.push_str(&format!(".param {}={}\n", dimension.name, first.lexical));
        }
    }
    let base = deck.clone();
    for text in &plan.model_text {
        let keyword = text
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if keyword == ".param" || keyword == ".func" {
            deck.push_str(text);
            deck.push('\n');
        }
    }
    // Parameter text the engine reads only in its full deck does not decide
    // a measurement.
    if rspice_core::Netlist::parse(&format!("{deck}.end\n")).is_err() {
        deck = base;
    }
    for (_, accepted) in &plan.measurements {
        deck.push_str(&accepted.expression);
        deck.push('\n');
    }
    deck.push_str(statement);
    deck.push_str("\n.end\n");
    rspice_core::Netlist::parse(&deck)
        .map(|_| ())
        .map_err(|error| format!("RSpice does not read it: {error}"))
}

fn parameters(plan: &mut ForeignPlan, card: ForeignCard) {
    let text = one_line(&card);
    let body = text
        .split_once(char::is_whitespace)
        .map_or("", |(_, body)| body);
    let assignments = match assignments(body) {
        Ok(assignments) => assignments,
        Err(reason) => {
            plan.kept.push((card, reason));
            return;
        }
    };
    let mut expressions = Vec::new();
    for (name, value) in assignments {
        if parse_spice_value_checked(&value).is_ok() {
            // The same assignment written twice is one variable; two that
            // differ both stay, in source order.
            let repeated = plan.variables.iter().any(|(written, was)| {
                written.eq_ignore_ascii_case(&name) && was.eq_ignore_ascii_case(&value)
            });
            if !repeated {
                plan.variables.push((name, value));
            }
        } else {
            expressions.push(format!("{name}={value}"));
        }
    }
    if !expressions.is_empty() {
        plan.model_text
            .push(format!(".param {}", expressions.join(" ")));
    }
}

/// The name and value pairs of a `.param` card, written `name=value`,
/// `name = value` or, as LTspice also reads, `name value`. A value in braces,
/// quotes or parentheses runs to its closing mark, and an expression runs
/// across the spaces around its operators (`Vb = Vas / 2`).
fn assignments(text: &str) -> Result<Vec<(String, String)>, String> {
    let mut pairs = Vec::new();
    let mut rest = text.trim_start();
    while !rest.is_empty() {
        let end = rest
            .find(|character: char| character.is_whitespace() || character == '=')
            .unwrap_or(rest.len());
        let name = &rest[..end];
        if name.is_empty()
            || !name.starts_with(|first: char| first.is_ascii_alphabetic() || first == '_')
        {
            return Err(format!("`{rest}` does not start with a parameter name"));
        }
        rest = rest[end..].trim_start();
        rest = rest.strip_prefix('=').unwrap_or(rest).trim_start();
        let length = value_length(rest);
        if length == 0 {
            return Err(format!("`{name}` is given no value"));
        }
        pairs.push((name.to_owned(), rest[..length].to_owned()));
        rest = rest[length..].trim_start();
    }
    Ok(pairs)
}

/// The length of the value at the start of `text`: through the bracket or
/// quote that closes an opening one, and to the next space that no operator
/// spans otherwise.
fn value_length(text: &str) -> usize {
    const OPERATORS: [char; 13] = [
        '+', '-', '*', '/', '^', '%', '<', '>', '&', '|', '!', '?', ':',
    ];
    let mut depth = 0_usize;
    let mut quoted = false;
    let mut previous = None;
    for (index, character) in text.char_indices() {
        match character {
            '"' => quoted = !quoted,
            '{' | '(' if !quoted => depth += 1,
            '}' | ')' if !quoted => depth = depth.saturating_sub(1),
            _ if character.is_whitespace() && depth == 0 && !quoted => {
                let next = text[index..].trim_start();
                let spanned = previous.is_some_and(|last| OPERATORS.contains(&last) || last == '=')
                    || next.starts_with(OPERATORS)
                    || next.starts_with("==");
                if !spanned {
                    return index;
                }
                continue;
            }
            _ => {}
        }
        previous = Some(character);
    }
    text.len()
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
fn analysis_draft(
    card: &ForeignCard,
    tool: ForeignTool,
) -> Result<(AnalysisDraft, Vec<String>), String> {
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
            if let Some(print_step) = step {
                number(print_step)?;
            }
            // LTspice and QSPICE accept the same steps and keep every one of
            // them whatever print step the card states. A plan's outputs need
            // a bounded grid, so this one is fine enough to keep the peaks of
            // those steps, and the solver takes the ceiling the source program
            // would: a 1024th of the stop time, which a stated maximum replaces
            // in LTspice and can only lower in QSPICE.
            let ceiling = stop_time / 1024.0;
            let mut draft = AnalysisDraft::for_kind(AnalysisKind::Transient);
            if let AnalysisDraft::Transient(tran) = &mut draft {
                tran.stop = stop.to_owned();
                tran.step = format_si_value(stop_time / 100_000.0);
                tran.start = start.unwrap_or("0").to_owned();
                tran.max_step = match max_step {
                    Some(stated)
                        if number(stated)? > 0.0
                            && (tool == ForeignTool::Ltspice || number(stated)? < ceiling) =>
                    {
                        stated.to_owned()
                    }
                    _ => format_si_value(ceiling),
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

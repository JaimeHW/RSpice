//! The measurement functions a reader can pick, with their arguments.
//!
//! One list feeds every place that offers measurements — the calculator's
//! function reference and the Add measurement form — so a function, its
//! argument order and what each omitted argument means are stated once. Each
//! entry names the function as ViVA does where ViVA has it.
//!
//! [`MeasureSpec::call`] turns a form's argument fields into the expression
//! the evaluator reads, and [`split_call`] reads such an expression back into
//! fields, so the expression a reader edits by hand and the fields a reader
//! fills stay one thing.

/// Where a measurement belongs in a picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureGroup {
    Levels,
    Timing,
    StepResponse,
    FrequencyResponse,
    Spectrum,
}

impl MeasureGroup {
    pub const ALL: [MeasureGroup; 5] = [
        MeasureGroup::Levels,
        MeasureGroup::Timing,
        MeasureGroup::StepResponse,
        MeasureGroup::FrequencyResponse,
        MeasureGroup::Spectrum,
    ];

    pub fn label(self) -> &'static str {
        match self {
            MeasureGroup::Levels => "Levels",
            MeasureGroup::Timing => "Edges and timing",
            MeasureGroup::StepResponse => "Step response",
            MeasureGroup::FrequencyResponse => "Frequency response",
            MeasureGroup::Spectrum => "Spectrum",
        }
    }
}

/// What an argument holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentKind {
    /// A signal: a trace of the pane, or any expression over them.
    Signal,
    /// A number, in the unit the label states: a count, a percentage,
    /// decibels.
    Number,
    /// A point on the x-axis, in the x-axis unit: a time on a transient
    /// pane, a frequency on an AC one.
    AxisPoint,
    /// A frequency of a signal along the x-axis, in the reciprocal of the
    /// x-axis unit: hertz on a transient pane.
    AxisFrequency,
    /// A level of the signal argument at this index, in that signal's unit.
    Level(usize),
    /// One of these option words, written quoted in the expression.
    Choice(&'static [&'static str]),
}

/// What omitting an argument means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentDefault {
    /// The call cannot omit it.
    Required,
    /// This value, which the form spells out when a later argument is given.
    Literal(&'static str),
    /// A value the function derives from the signal, described here. It
    /// cannot be spelled, so a later argument cannot be given without it.
    Derived(&'static str),
    /// Whatever the argument at this index holds.
    SameAs(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgumentSpec {
    /// The form's label for the field.
    pub label: &'static str,
    /// The argument's name in a signature: `x`, `level`, `n`.
    pub short: &'static str,
    pub kind: ArgumentKind,
    pub default: ArgumentDefault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasureSpec {
    /// The name as the catalogue spells it; the evaluator ignores case.
    pub name: &'static str,
    /// Other spellings the evaluator accepts for the same function.
    pub aliases: &'static [&'static str],
    pub group: MeasureGroup,
    /// A few words for a reference row.
    pub summary: &'static str,
    /// What it measures and how, for the form.
    pub description: &'static str,
    pub arguments: &'static [ArgumentSpec],
}

const fn arg(
    label: &'static str,
    short: &'static str,
    kind: ArgumentKind,
    default: ArgumentDefault,
) -> ArgumentSpec {
    ArgumentSpec {
        label,
        short,
        kind,
        default,
    }
}

use ArgumentDefault::{Derived, Literal, Required, SameAs};
use ArgumentKind::{AxisFrequency, AxisPoint, Choice, Level, Number, Signal};

const EDGES: &[&str] = &["rising", "falling", "either"];
const SIGNAL: ArgumentSpec = arg("Signal", "x", Signal, Required);

/// `(x, from, to, n, initial, final)`, which `rise`, `fall` and `slewRate`
/// share; only the derived levels differ.
const fn edge_arguments(initial: &'static str, last: &'static str) -> [ArgumentSpec; 6] {
    [
        SIGNAL,
        arg("From (% of edge)", "from", Number, Literal("10")),
        arg("To (% of edge)", "to", Number, Literal("90")),
        arg("Edge number", "n", Number, Literal("1")),
        arg("0 % level", "initial", Level(0), Derived(initial)),
        arg("100 % level", "final", Level(0), Derived(last)),
    ]
}

/// `(x, from, to)`, which the level measures share: the window is the whole
/// trace unless both ends are given.
const LEVEL_ARGUMENTS: [ArgumentSpec; 3] = [
    SIGNAL,
    arg(
        "Window from",
        "from",
        Number,
        Derived("the start of the trace"),
    ),
    arg("Window to", "to", Number, Derived("the end of the trace")),
];

const RISE_ARGUMENTS: [ArgumentSpec; 6] = edge_arguments("signal minimum", "signal maximum");
const FALL_ARGUMENTS: [ArgumentSpec; 6] = edge_arguments("signal maximum", "signal minimum");

pub const MEASUREMENTS: &[MeasureSpec] = &[
    MeasureSpec {
        name: "avg",
        aliases: &["average"],
        group: MeasureGroup::Levels,
        summary: "mean over the window",
        description: "The mean of the signal, integrated over the window and divided by its width.",
        arguments: &LEVEL_ARGUMENTS,
    },
    MeasureSpec {
        name: "rms",
        aliases: &[],
        group: MeasureGroup::Levels,
        summary: "root mean square",
        description: "The root of the mean square, integrated over the window.",
        arguments: &LEVEL_ARGUMENTS,
    },
    MeasureSpec {
        name: "min",
        aliases: &[],
        group: MeasureGroup::Levels,
        summary: "lowest sample",
        description: "The lowest value the signal takes in the window.",
        arguments: &LEVEL_ARGUMENTS,
    },
    MeasureSpec {
        name: "max",
        aliases: &[],
        group: MeasureGroup::Levels,
        summary: "highest sample",
        description: "The highest value the signal takes in the window.",
        arguments: &LEVEL_ARGUMENTS,
    },
    MeasureSpec {
        name: "pp",
        aliases: &[],
        group: MeasureGroup::Levels,
        summary: "peak to peak",
        description: "The highest value less the lowest.",
        arguments: &LEVEL_ARGUMENTS,
    },
    MeasureSpec {
        name: "yval",
        aliases: &[],
        group: MeasureGroup::Levels,
        summary: "value at an x",
        description: "The signal's value at one point of the x-axis, interpolated between samples.",
        arguments: &[SIGNAL, arg("At x", "at", AxisPoint, Required)],
    },
    MeasureSpec {
        name: "cross",
        aliases: &[],
        group: MeasureGroup::Timing,
        summary: "x of the nth crossing",
        description: "Where the signal crosses a level for the nth time, counting only crossings \
                      of the edge type.",
        arguments: &[
            SIGNAL,
            arg("Level", "level", Level(0), Required),
            arg("Crossing number", "n", Number, Literal("1")),
            arg("Edge type", "edge", Choice(EDGES), Literal("either")),
        ],
    },
    MeasureSpec {
        name: "rise",
        aliases: &["riseTime"],
        group: MeasureGroup::Timing,
        summary: "rise time of an edge",
        description: "How long the nth rising edge takes from one percentage of its transition \
                      to another, 10 % to 90 % unless stated. The 0 % and 100 % levels are the \
                      signal's own excursion unless given.",
        arguments: &RISE_ARGUMENTS,
    },
    MeasureSpec {
        name: "fall",
        aliases: &["fallTime"],
        group: MeasureGroup::Timing,
        summary: "fall time of an edge",
        description: "How long the nth falling edge takes from one percentage of its transition \
                      to another, 10 % to 90 % of the way down unless stated.",
        arguments: &FALL_ARGUMENTS,
    },
    MeasureSpec {
        name: "slewRate",
        aliases: &[],
        group: MeasureGroup::Timing,
        summary: "slope of an edge",
        description: "The average slope of the nth edge between two percentages of its \
                      transition. The edge runs from the 0 % level to the 100 % level, so give \
                      them high to low for a falling edge, whose slope is negative.",
        arguments: &RISE_ARGUMENTS,
    },
    MeasureSpec {
        name: "delay",
        aliases: &[],
        group: MeasureGroup::Timing,
        summary: "between two edges",
        description: "From the nth edge of the first signal through its level to the nth edge \
                      of the second through its own. Edges are counted from the start of the \
                      window; the second signal's settings default to the first's.",
        arguments: &[
            arg("Signal 1", "a", Signal, Required),
            arg("Signal 2", "b", Signal, Required),
            arg(
                "Level 1",
                "level1",
                Level(0),
                Derived("mid level of each signal"),
            ),
            arg("Edge type 1", "edge1", Choice(EDGES), Literal("either")),
            arg("Edge number 1", "n1", Number, Literal("1")),
            arg("Level 2", "level2", Level(1), SameAs(2)),
            arg("Edge type 2", "edge2", Choice(EDGES), SameAs(3)),
            arg("Edge number 2", "n2", Number, SameAs(4)),
        ],
    },
    MeasureSpec {
        name: "pulseWidth",
        aliases: &[],
        group: MeasureGroup::Timing,
        summary: "width of the nth pulse",
        description: "How long the nth high pulse stays above the level, or the nth low pulse \
                      below it. A pulse already under way when the window opens is not counted.",
        arguments: &[
            SIGNAL,
            arg("Level", "level", Level(0), Derived("mid level")),
            arg(
                "Polarity",
                "polarity",
                Choice(&["high", "low"]),
                Literal("high"),
            ),
            arg("Pulse number", "n", Number, Literal("1")),
        ],
    },
    MeasureSpec {
        name: "dutyCycle",
        aliases: &["duty"],
        group: MeasureGroup::Timing,
        summary: "% of each period above a level",
        description: "The percentage of each period the signal spends above the level, averaged \
                      over the whole cycles in the window.",
        arguments: &[
            SIGNAL,
            arg("Level", "level", Level(0), Derived("mid level")),
        ],
    },
    MeasureSpec {
        name: "period",
        aliases: &[],
        group: MeasureGroup::Timing,
        summary: "mean period",
        description: "The mean time between rising crossings of the level, over every whole \
                      cycle in the window or over the first n.",
        arguments: &[
            SIGNAL,
            arg("Cycles", "n", Number, Derived("every whole cycle")),
            arg("Level", "level", Level(0), Derived("mid level")),
        ],
    },
    MeasureSpec {
        name: "frequency",
        aliases: &["freq"],
        group: MeasureGroup::Timing,
        summary: "1 / period",
        description: "The reciprocal of the mean period, over every whole cycle in the window or \
                      over the first n.",
        arguments: &[
            SIGNAL,
            arg("Cycles", "n", Number, Derived("every whole cycle")),
            arg("Level", "level", Level(0), Derived("mid level")),
        ],
    },
    MeasureSpec {
        name: "overshoot",
        aliases: &[],
        group: MeasureGroup::StepResponse,
        summary: "% past the final value",
        description: "The percentage of the step by which the response passes its final value.",
        arguments: &[
            SIGNAL,
            arg(
                "Initial value",
                "initial",
                Level(0),
                Derived("first sample"),
            ),
            arg("Final value", "final", Level(0), Derived("last sample")),
        ],
    },
    MeasureSpec {
        name: "undershoot",
        aliases: &[],
        group: MeasureGroup::StepResponse,
        summary: "% the wrong way past the start",
        description: "The percentage of the step by which the response first moves the wrong \
                      way, past its initial value.",
        arguments: &[
            SIGNAL,
            arg(
                "Initial value",
                "initial",
                Level(0),
                Derived("first sample"),
            ),
            arg("Final value", "final", Level(0), Derived("last sample")),
        ],
    },
    MeasureSpec {
        name: "settlingTime",
        aliases: &["settling"],
        group: MeasureGroup::StepResponse,
        summary: "time into the band",
        description: "The time from the start until the response enters and stays within a \
                      band about its final value, as a percentage of the step.",
        arguments: &[
            SIGNAL,
            arg("Band (% of step)", "band", Number, Required),
            arg("Final value", "final", Level(0), Derived("last sample")),
            arg("Start", "start", AxisPoint, Derived("window start")),
        ],
    },
    MeasureSpec {
        name: "bandwidth",
        aliases: &[],
        group: MeasureGroup::FrequencyResponse,
        summary: "band within n dB",
        description: "For a low-pass response, the frequency at which the gain falls the given \
                      dB below its low-frequency value; for a high-pass, the frequency at which \
                      it rises to within it of its high-frequency value; for a band-pass, the \
                      width between the two frequencies that far below the peak.",
        arguments: &[
            arg("Response", "x", Signal, Required),
            arg("Drop (dB)", "dB", Number, Literal("3")),
            arg(
                "Shape",
                "shape",
                Choice(&["low", "high", "band"]),
                Literal("low"),
            ),
        ],
    },
    MeasureSpec {
        name: "gainMargin",
        aliases: &[],
        group: MeasureGroup::FrequencyResponse,
        summary: "dB below unity at -180°",
        description: "The gain margin of a complex loop gain, as the Bode sheet reports it: how \
                      far below 0 dB the gain is where the phase reaches -180°.",
        arguments: &[arg("Loop gain", "loop", Signal, Required)],
    },
    MeasureSpec {
        name: "phaseMargin",
        aliases: &[],
        group: MeasureGroup::FrequencyResponse,
        summary: "180° + phase at 0 dB",
        description: "The phase margin of a complex loop gain, as the Bode sheet reports it: \
                      180° plus its phase where its gain crosses 0 dB.",
        arguments: &[arg("Loop gain", "loop", Signal, Required)],
    },
    MeasureSpec {
        name: "thd",
        aliases: &[],
        group: MeasureGroup::Spectrum,
        summary: "% harmonic distortion",
        description: "Total harmonic distortion of a periodic signal in percent, over the given \
                      number of harmonics, at a fundamental found from the signal unless stated.",
        arguments: &[
            SIGNAL,
            arg("Harmonics", "n", Number, Literal("10")),
            arg(
                "Fundamental",
                "f0",
                AxisFrequency,
                Derived("found from the signal"),
            ),
        ],
    },
];

/// The catalogue entry the evaluator would run for `name`, by its own
/// spelling or an alias, ignoring case.
pub fn find(name: &str) -> Option<&'static MeasureSpec> {
    MEASUREMENTS.iter().find(|spec| {
        spec.name.eq_ignore_ascii_case(name)
            || spec
                .aliases
                .iter()
                .any(|alias| alias.eq_ignore_ascii_case(name))
    })
}

impl MeasureSpec {
    /// The full signature: `rise(x, from, to, n, initial, final)`.
    pub fn signature(&self) -> String {
        let shorts: Vec<&str> = self.arguments.iter().map(|arg| arg.short).collect();
        format!("{}({})", self.name, shorts.join(", "))
    }

    /// The signature with its optional arguments folded: `rise(x, …)`.
    pub fn short_signature(&self) -> String {
        let required: Vec<&str> = self
            .arguments
            .iter()
            .filter(|arg| arg.default == Required)
            .map(|arg| arg.short)
            .collect();
        let folded = if required.len() < self.arguments.len() {
            ", …"
        } else {
            ""
        };
        format!("{}({}{folded})", self.name, required.join(", "))
    }

    /// The call `values` spell, one per argument in order. Trailing empty
    /// fields are left out; an empty field before a filled one is spelled
    /// as its default, or refused when the default cannot be spelled.
    pub fn call(&self, values: &[String]) -> Result<String, String> {
        let values: Vec<&str> = (0..self.arguments.len())
            .map(|index| values.get(index).map_or("", |value| value.trim()))
            .collect();
        let last = values.iter().rposition(|value| !value.is_empty());
        let mut spelled: Vec<String> = Vec::new();
        for (index, argument) in self.arguments.iter().enumerate() {
            let value = values[index];
            let given = !value.is_empty();
            if !given && last.is_none_or(|last| index > last) {
                if argument.default == Required {
                    return Err(format!("{} is required.", argument.label));
                }
                break;
            }
            let text = if given {
                match argument.kind {
                    Choice(_) => quote(value),
                    _ => value.to_owned(),
                }
            } else {
                match argument.default {
                    Required => return Err(format!("{} is required.", argument.label)),
                    Literal(word) => match argument.kind {
                        Choice(_) => quote(word),
                        _ => word.to_owned(),
                    },
                    Derived(_) => {
                        let later = self.arguments[last.unwrap_or(index)].label;
                        return Err(format!(
                            "Give {} to set {later}.",
                            argument.label.to_lowercase()
                        ));
                    }
                    SameAs(other) => spelled[other].clone(),
                }
            };
            spelled.push(text);
        }
        Ok(format!("{}({})", self.name, spelled.join(", ")))
    }
}

fn quote(word: &str) -> String {
    let bare = word.trim_matches('"');
    format!("\"{bare}\"")
}

/// Read a call to a catalogued measurement back into its argument fields:
/// `cross(V(out), 2.5, 1, "rising")` reads as `cross` with `V(out)`,
/// `2.5`, `1` and `rising`. `None` when the expression is not one such call
/// with at most the catalogued number of arguments.
pub fn split_call(expression: &str) -> Option<(&'static MeasureSpec, Vec<String>)> {
    let expression = expression.trim();
    let open = expression.find('(')?;
    let spec = find(expression[..open].trim())?;
    let inner = expression[open + 1..].strip_suffix(')')?;
    let mut arguments = Vec::new();
    let mut depth = 0usize;
    let mut quoted = false;
    let mut current = String::new();
    for c in inner.chars() {
        match c {
            '"' => quoted = !quoted,
            '(' if !quoted => depth += 1,
            ')' if !quoted => depth = depth.checked_sub(1)?,
            ',' if !quoted && depth == 0 => {
                arguments.push(std::mem::take(&mut current));
                continue;
            }
            _ => {}
        }
        current.push(c);
    }
    if depth != 0 || quoted {
        return None;
    }
    if !(inner.trim().is_empty() && arguments.is_empty()) {
        arguments.push(current);
    }
    if arguments.len() > spec.arguments.len() {
        return None;
    }
    let fields = arguments
        .iter()
        .zip(spec.arguments)
        .map(|(text, argument)| match argument.kind {
            Choice(_) => text.trim().trim_matches('"').to_ascii_lowercase(),
            _ => text.trim().to_owned(),
        })
        .collect();
    Some((spec, fields))
}

#[cfg(test)]
mod tests;

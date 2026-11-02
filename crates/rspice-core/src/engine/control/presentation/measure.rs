//! `meas`: a measurement of the current plot, as ngspice's com_meas
//! (frontend/measure.c:37) runs it through get_measure2
//! (frontend/com_measure2.c:1623), the code that also runs a deck's
//! `.meas` cards. The measurement is this host's `.meas` engine over the
//! plot's vectors; the script binds the result.
use super::*;
use crate::analysis::measure_signals::evaluate_tran_measurements_with_signals_and_abort;
use crate::engine::control::shell::percent_e;
use crate::netlist::lexer::parse_spice_value;
use std::collections::HashMap;

/// The measurement functions get_measure2 runs (com_measure2.c:208).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Function {
    Trig,
    Find,
    When,
    Average,
    Rms,
    Integral,
    Min,
    Max,
    MinAt,
    MaxAt,
    PeakToPeak,
}

impl Function {
    fn parse(word: &str) -> Option<Self> {
        Some(match word.to_ascii_uppercase().as_str() {
            "TRIG" | "DELAY" => Self::Trig,
            "FIND" => Self::Find,
            "WHEN" => Self::When,
            "AVG" => Self::Average,
            "RMS" => Self::Rms,
            "INTEG" => Self::Integral,
            "MIN" => Self::Min,
            "MAX" => Self::Max,
            "MIN_AT" => Self::MinAt,
            "MAX_AT" => Self::MaxAt,
            "PP" => Self::PeakToPeak,
            _ => return None,
        })
    }

    /// The `.meas` card's word; MIN_AT and MAX_AT read MIN's and MAX's axis.
    const fn card_word(self) -> &'static str {
        match self {
            Self::Trig => "TRIG",
            Self::Find => "FIND",
            Self::When => "WHEN",
            Self::Average => "AVG",
            Self::Rms => "RMS",
            Self::Integral => "INTEG",
            Self::Min | Self::MinAt => "MIN",
            Self::Max | Self::MaxAt => "MAX",
            Self::PeakToPeak => "PP",
        }
    }
}

/// Words that open a clause rather than name a vector.
const KEYWORDS: [&str; 13] = [
    "TRIG", "TARG", "FIND", "WHEN", "AVG", "RMS", "INTEG", "MIN", "MAX", "MIN_AT", "MAX_AT", "PP",
    "DELAY",
];

/// `name=value` settings whose value is a number (measure_parse_stdParams).
const SETTINGS: [&str; 8] = ["VAL", "AT", "FROM", "TO", "TD", "RISE", "FALL", "CROSS"];

/// INPevaluate's number, a scale factor and unit letters included. A word
/// with anything but letters after its number, `0.9*v(2)`, is an
/// expression.
fn number(text: &str) -> Option<Value> {
    let unit = text.trim_start_matches(|c: char| {
        c.is_ascii_digit() || matches!(c, '+' | '-' | '.' | 'e' | 'E')
    });
    (text.starts_with(|c: char| c.is_ascii_digit() || matches!(c, '+' | '-' | '.'))
        && unit.chars().all(|c| c.is_ascii_alphabetic()))
    .then(|| parse_spice_value(text).ok())
    .flatten()
}

fn is_literal(text: &str) -> bool {
    number(text).is_some() || text.eq_ignore_ascii_case("last")
}

/// The value of a `key=` word among `words`, if one is given.
fn given<'a>(words: &'a [String], key: &str) -> Option<&'a String> {
    words.iter().find(|word| is_key(word, key))
}

fn setting(words: &[String], key: &str) -> Option<Value> {
    number(given(words, key)?.split_once('=')?.1)
}

/// A meas command's measured operands, read as this host's vectors and
/// handed to the measurement engine under names of their own.
struct Operands {
    axis: Vec<Value>,
    signals: Vec<(String, Vec<Value>)>,
}

impl Operands {
    /// Hand `samples` over as a signal of their own; returns its name.
    fn push(&mut self, samples: Vec<Value>) -> String {
        let name = format!("v(meas_operand_{})", self.signals.len() + 1);
        self.signals.push((name.clone(), samples));
        name
    }

    /// The signal `left - right` of two signals already handed over.
    fn difference(&mut self, left: &str, right: &str) -> String {
        let samples = |name: &str| {
            self.signals
                .iter()
                .find(|(signal, _)| signal == name)
                .map(|(_, samples)| samples.as_slice())
                .unwrap_or_default()
        };
        let difference = samples(left)
            .iter()
            .zip(samples(right))
            .map(|(left, right)| left - right)
            .collect();
        self.push(difference)
    }

    fn map(&self) -> HashMap<String, &[Value]> {
        let mut map = HashMap::new();
        for (name, samples) in &self.signals {
            for key in [
                name.clone(),
                name.to_ascii_lowercase(),
                name.to_ascii_uppercase(),
            ] {
                map.insert(key, samples.as_slice());
            }
        }
        map
    }
}

/// What every operand of one meas command is evaluated with.
#[derive(Clone, Copy)]
struct Scope<'a> {
    variables: &'a ParamContext,
    line: usize,
    abort: &'a dyn AbortSignal,
}

fn is_key(word: &str, key: &str) -> bool {
    word.split_once('=')
        .is_some_and(|(name, _)| name.eq_ignore_ascii_case(key))
}

impl ControlCircuit {
    /// `meas analysis name function ...`. Returns the text ngspice prints
    /// and whether the measurement succeeded: a failure prints
    /// ` meas <line> failed!` on the terminal itself (measure.c:131), not
    /// where a redirection points.
    pub(in crate::engine) fn measure(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(String, bool), ControlExecutionError> {
        let line = command.line;
        let mut words: Vec<String> = crate::engine::control::shell::words(&command.arguments)
            .into_iter()
            .map(|word| word.text)
            .collect();
        self.substitute_one_element_vectors(&mut words, variables, line, abort)?;
        // ngspice reads its control lines in lower case.
        let failed = |words: &[String]| {
            (
                format!(" meas {} failed!\n\n", words.join(" ").to_ascii_lowercase()),
                false,
            )
        };
        // `a=` `b` and `a` `=b` are one word to ngspice's parsers.
        let mut joined: Vec<String> = Vec::new();
        for word in &words {
            match joined.last_mut() {
                Some(last) if last.ends_with('=') || word.starts_with('=') => last.push_str(word),
                _ => joined.push(word.clone()),
            }
        }
        let [analysis, name, function, rest @ ..] = joined.as_slice() else {
            return Ok(failed(&words));
        };
        let (Some(function), Some(axis)) =
            (Function::parse(function), self.measured_axis(analysis))
        else {
            return Ok(failed(&words));
        };
        let name = name.to_ascii_lowercase();
        let mut operands = Operands {
            axis,
            signals: Vec::new(),
        };
        let scope = Scope {
            variables,
            line,
            abort,
        };
        let Ok(cards) = self.measure_cards(function, &name, rest, &mut operands, &scope) else {
            check_abort(abort, line)?;
            return Ok(failed(&words));
        };
        let mut statements = Vec::new();
        for card in cards.iter().flatten() {
            let Some(statement) =
                crate::netlist::Netlist::parse(&format!("* meas\n{card}\n.end\n"))
                    .ok()
                    .and_then(|parsed| parsed.measurements.into_iter().next())
            else {
                return Ok(failed(&words));
            };
            statements.push(statement);
        }
        let mut netlist = self.netlist.clone();
        netlist.measurements = statements;
        let signals = operands.map();
        let results = evaluate_tran_measurements_with_signals_and_abort(
            &netlist,
            &operands.axis,
            &signals,
            None,
            abort,
        )
        .map_err(|error| simulation_error(line, error))?;
        let mut results = results.iter();
        let Some((text, result)) = measured_text(
            function,
            &name,
            rest,
            &cards,
            &mut results,
            &operands.axis,
            analysis,
        ) else {
            return Ok(failed(&words));
        };
        // measure.c:140: `let name = %e`, the result to seven digits.
        let bound = percent_e(result, 6).parse().unwrap_or(result);
        self.bind_vector(
            &name,
            VectorValue {
                samples: vec![ComplexValue::new(bound, 0.0)],
                complex: false,
                scale: None,
                unit: SignalUnit::Unspecified,
            },
            line,
        )?;
        Ok((text, true))
    }

    /// measure.c:56: the right side of an `=` that names a one-element
    /// vector is replaced by its value, before anything is measured.
    fn substitute_one_element_vectors(
        &self,
        words: &mut [String],
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        for index in 0..words.len() {
            let (target, value) = match words[index].split_once('=') {
                Some((_, "")) if index + 1 < words.len() => (index + 1, words[index + 1].clone()),
                Some((_, value)) if !value.is_empty() => (index, value.to_owned()),
                _ => continue,
            };
            if is_literal(&value) {
                continue;
            }
            let Ok(parsed @ Expr::Param(_)) = parse_control_expression_with_abort(&value, abort)
            else {
                continue;
            };
            if let Ok(vector) = self.evaluate_vector(&parsed, variables, line, abort)
                && let [sample] = vector.samples.as_slice()
            {
                let number = percent_e(sample.re, 6);
                let word = &mut words[target];
                *word = match word.split_once('=') {
                    Some((key, _)) => format!("{key}={number}"),
                    None => number,
                };
            }
        }
        check_abort(abort, line)
    }

    /// The current plot's scale, if it is the plot kind `analysis` names:
    /// get_measure2 measures only tran, ac, dc and sp plots.
    fn measured_axis(&self, analysis: &str) -> Option<Vec<Value>> {
        let dataset = self.current_dataset()?;
        let matches = matches!(
            (&dataset.result, analysis.to_ascii_lowercase().as_str()),
            (ControlAnalysisResult::Transient(_), "tran")
                | (
                    ControlAnalysisResult::Ac(_) | ControlAnalysisResult::AcTable(_),
                    "ac"
                )
                | (ControlAnalysisResult::DcSweep(_), "dc")
                | (ControlAnalysisResult::SParameters(_), "sp")
        );
        matches.then(|| {
            (0..dataset.length())
                .map_while(|row| dataset.scale_value(row))
                .collect()
        })
    }

    /// The `.meas` cards a meas command runs as: the card itself, or for
    /// TRIG/TARG one WHEN card per side, as com_measure2.c:1735 measures
    /// each side on its own (`None` for a side given `AT=`).
    fn measure_cards(
        &self,
        function: Function,
        name: &str,
        rest: &[String],
        operands: &mut Operands,
        scope: &Scope<'_>,
    ) -> Result<Vec<Option<String>>, ControlError> {
        let line = scope.line;
        let card =
            |name: &str, words: Vec<String>| format!(".meas tran {name} {}", words.join(" "));
        if function != Function::Trig {
            let mut words = vec![function.card_word().to_owned()];
            words.extend(rest.iter().cloned());
            let words = self.measure_clause(&words, operands, scope)?;
            return Ok(vec![Some(card(name, words))]);
        }
        let (trig, targ) =
            trig_targ(rest).ok_or_else(|| command_error(line, "meas TRIG needs TARG"))?;
        // A FROM on one side applies to both (com_measure2.c:1787).
        let from = given(trig, "from").or_else(|| given(targ, "from")).cloned();
        let mut cards = Vec::new();
        for side in [trig, targ] {
            if given(side, "at").is_some() {
                cards.push(None);
                continue;
            }
            let [signal, settings @ ..] = side else {
                return Err(command_error(line, "meas TRIG and TARG name a vector"));
            };
            let value = given(settings, "val")
                .and_then(|word| word.split_once('='))
                .map(|(_, value)| value)
                .ok_or_else(|| command_error(line, "meas TRIG and TARG need VAL="))?;
            let mut words = vec!["WHEN".to_owned(), format!("{signal}={value}")];
            words.extend(settings.iter().filter(|word| !is_key(word, "val")).cloned());
            if let Some(from) = &from
                && given(side, "from").is_none()
            {
                words.push(from.clone());
            }
            let words = self.measure_clause(&words, operands, scope)?;
            cards.push(Some(card("meas_side", words)));
        }
        Ok(cards)
    }

    /// One meas clause rewritten for a `.meas` card: settings evaluated to
    /// numbers, vectors named as engine signals.
    fn measure_clause(
        &self,
        words: &[String],
        operands: &mut Operands,
        scope: &Scope<'_>,
    ) -> Result<Vec<String>, ControlError> {
        let line = scope.line;
        let mut card = Vec::new();
        for word in words {
            let upper = word.to_ascii_uppercase();
            if let Some((key, value)) = word.split_once('=') {
                let key_upper = key.to_ascii_uppercase();
                if SETTINGS.contains(&key_upper.as_str()) {
                    let value = match self.measure_operand(value, operands, scope)? {
                        Operand::Literal(value) => value,
                        Operand::Signal(_) => {
                            return Err(command_error(
                                line,
                                format!("meas {key}= takes one value"),
                            ));
                        }
                    };
                    card.push(format!("{key_upper}={value}"));
                } else {
                    // A WHEN condition: a vector equal to a number or to a
                    // vector (com_measure2.c:1510 measure_valid_vector).
                    let Operand::Signal(left) = self.measure_operand(key, operands, scope)? else {
                        return Err(command_error(line, format!("meas WHEN {key} is no vector")));
                    };
                    card.push(match self.measure_operand(value, operands, scope)? {
                        Operand::Literal(value) => format!("{left}={value}"),
                        // ngspice counts the crossings of two vectors by
                        // their difference (com_measure2.c:563), where this
                        // host's engine reads the left one's slope, as Xyce
                        // does: the difference is measured against zero.
                        Operand::Signal(right) => {
                            format!("{}=0", operands.difference(&left, &right))
                        }
                    });
                }
            } else if KEYWORDS.contains(&upper.as_str()) {
                card.push(upper);
            } else {
                let Operand::Signal(signal) = self.measure_operand(word, operands, scope)? else {
                    return Err(command_error(line, format!("meas {word} is no vector")));
                };
                card.push(signal);
            }
        }
        Ok(card)
    }

    /// A number as written, a one-element vector as its `%e` value, or a
    /// vector of the plot as the engine signal it is handed over as. A
    /// complex vector measures by its real part (com_measure2.c:138).
    fn measure_operand(
        &self,
        text: &str,
        operands: &mut Operands,
        scope: &Scope<'_>,
    ) -> Result<Operand, ControlError> {
        let Scope {
            variables,
            line,
            abort,
        } = *scope;
        if is_literal(text) {
            return Ok(Operand::Literal(text.to_owned()));
        }
        let parsed = parse_control_expression_with_abort(text, abort)
            .map_err(|error| ControlError::evaluation(line, error.into()))?;
        let vector = self.evaluate_vector(&parsed, variables, line, abort)?;
        if let [sample] = vector.samples.as_slice() {
            return Ok(Operand::Literal(percent_e(sample.re, 6)));
        }
        if vector.samples.len() != operands.axis.len() {
            return Err(command_error(
                line,
                format!("meas operand '{text}' is not a vector of the current plot"),
            ));
        }
        let samples = vector.samples.iter().map(|sample| sample.re).collect();
        Ok(Operand::Signal(operands.push(samples)))
    }
}

enum Operand {
    Literal(String),
    Signal(String),
}

/// The words of a TRIG clause and of its TARG clause.
fn trig_targ(rest: &[String]) -> Option<(&[String], &[String])> {
    let targ = rest
        .iter()
        .position(|word| word.eq_ignore_ascii_case("targ"))?;
    Some((&rest[..targ], &rest[targ + 1..]))
}

/// get_measure2's printed line (com_measure2.c:1818-2140, `%.5e` fields) and
/// the value com_meas binds.
fn measured_text<'a>(
    function: Function,
    name: &str,
    rest: &[String],
    cards: &[Option<String>],
    results: &mut impl Iterator<Item = &'a crate::analysis::MeasureResult>,
    axis: &[Value],
    analysis: &str,
) -> Option<(String, Value)> {
    let e = |value: Value| percent_e(value, 5);
    if function == Function::Trig {
        let (trig, targ) = trig_targ(rest)?;
        let mut side = |words: &[String], card: &Option<String>| match card {
            None => setting(words, "at"),
            Some(_) => results.next()?.value,
        };
        let trig = side(trig, cards.first()?)?;
        let targ = side(targ, cards.get(1)?)?;
        let delay = targ - trig;
        let text = format!(
            "{name:<20}=  {} targ=  {} trig=  {}\n",
            e(delay),
            e(targ),
            e(trig)
        );
        return Some((text, delay));
    }
    let result = results.next()?;
    let value = result.value?;
    let (from, to) = (setting(rest, "from"), setting(rest, "to"));
    Some(match function {
        Function::Trig | Function::Find | Function::When => {
            (format!("{name:<20}=  {}\n", e(value)), value)
        }
        Function::PeakToPeak => {
            // PP prints the window as given: 0 to 0, or the whole DC sweep
            // (com_measure2.c:1415).
            let (low, high) = if analysis.eq_ignore_ascii_case("dc") {
                (-1e99, 1e99)
            } else {
                (0.0, 0.0)
            };
            let text = format!(
                "{name:<20}=  {} from=  {} to=  {}\n",
                e(value),
                e(from.unwrap_or(low)),
                e(to.unwrap_or(high))
            );
            (text, value)
        }
        Function::Average => {
            // AVG prints the last point it averaged: the first at or after
            // TO, or the plot's last.
            let to = to.map_or(axis.last().copied(), |to| {
                axis.iter()
                    .copied()
                    .find(|&x| x >= to)
                    .or(axis.last().copied())
            });
            let text = format!(
                "{name:<20}=  {} from=  {} to=  {}\n",
                e(value),
                e(from.or(axis.first().copied())?),
                e(to?)
            );
            (text, value)
        }
        Function::Rms | Function::Integral => {
            let text = format!(
                "{name:<20}=  {} from=  {} to=  {}\n",
                e(value),
                e(from.or(axis.first().copied())?),
                e(to.or(axis.last().copied())?)
            );
            (text, value)
        }
        Function::Min | Function::Max => {
            let at = result.event_axis?;
            (format!("{name:<20}=  {} at=  {}\n", e(value), e(at)), value)
        }
        Function::MinAt | Function::MaxAt => {
            let at = result.event_axis?;
            (format!("{name:<20}=  {} with=  {}\n", e(at), e(value)), at)
        }
    })
}

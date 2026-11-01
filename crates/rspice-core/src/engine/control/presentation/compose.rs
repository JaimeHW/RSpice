//! `compose`: a vector of listed values or of a generated sweep, as ngspice
//! frontend/com_compose.c builds it.

use super::*;
use crate::execution::control::shell::{SetValue, assignments};

impl ControlCircuit {
    /// `compose name values expr...` or `compose name parameter=value...`.
    /// The vector replaces any of its name in the current plot.
    pub(in crate::engine::control) fn compose(
        &mut self,
        command: &ControlCommand,
        variables: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ControlExecutionError> {
        let line = command.line;
        let mut input = command.arguments.trim();
        let name = word(&mut input, line)?
            .trim_matches(|character| matches!(character, '"' | '\''))
            .to_owned();
        if name.is_empty() {
            return Err(command_error(line, "compose requires a vector name").into());
        }
        let value = if first_word(input).eq_ignore_ascii_case("values") {
            word(&mut input, line)?;
            self.composed_values(input, variables, line, abort)?
        } else {
            sweep(input, variables, line, abort)?
        };
        self.compose_vector(&name, value, line)?;
        Ok(())
    }

    /// One element from each value expression; the vector is complex when
    /// any value is (com_compose.c:126).
    fn composed_values(
        &self,
        mut input: &str,
        variables: &ParamContext,
        line: usize,
        abort: &dyn AbortSignal,
    ) -> Result<VectorValue, ControlExecutionError> {
        let mut samples = Vec::new();
        let mut complex = false;
        loop {
            input = input.trim_start_matches(',').trim_start();
            if input.is_empty() {
                break;
            }
            let (expression, label) = expression(&mut input, line, abort)?;
            let value = self.evaluate_vector(&expression, variables, line, abort)?;
            let [sample] = value.samples.as_slice() else {
                return Err(command_error(
                    line,
                    format!(
                        "compose values takes one element from each value; '{label}' has {}",
                        value.samples.len()
                    ),
                )
                .into());
            };
            complex |= value.complex;
            samples.push(*sample);
        }
        if samples.is_empty() {
            return Err(command_error(line, "compose values requires a value").into());
        }
        Ok(VectorValue {
            samples,
            complex,
            scale: None,
            unit: SignalUnit::Unspecified,
        })
    }
}

/// A linear or logarithmic sweep (com_compose.c:239). The random `unif` and
/// `gauss` forms are not supported.
fn sweep(
    input: &str,
    variables: &ParamContext,
    line: usize,
    abort: &dyn AbortSignal,
) -> Result<VectorValue, ControlExecutionError> {
    let error = |message: String| -> ControlExecutionError { command_error(line, message).into() };
    let mut given: BTreeMap<String, Value> = BTreeMap::new();
    for assignment in assignments(input, line)? {
        let name = assignment.name.to_ascii_lowercase();
        let SetValue::Word(value) = &assignment.value else {
            return Err(error(format!(
                "compose -> bad syntax at {}",
                assignment.name
            )));
        };
        if !matches!(
            name.as_str(),
            "start" | "stop" | "step" | "center" | "span" | "lin" | "log" | "dec" | "oct"
        ) {
            return Err(error(format!("compose -> bad parm {name}")));
        }
        given.insert(name, scalar(&value.text, variables, line, abort)?);
    }
    let get = |name: &str| given.get(name).copied();
    let (mut start, mut stop, mut step) = (get("start"), get("stop"), get("step"));
    if let (Some(center), Some(span), None, None) = (get("center"), get("span"), start, stop) {
        start = Some(center - span / 2.0);
        stop = Some(center + span / 2.0);
    }
    if step == Some(0.0) {
        return Err(error("compose -> step cannot = 0.0".into()));
    }
    let kinds = ["lin", "log", "dec", "oct"]
        .iter()
        .filter(|kind| given.contains_key(**kind))
        .count();
    if kinds > 1 {
        return Err(error(
            "compose -> can have at most one of (lin, log, dec, oct)".into(),
        ));
    }
    let mut lin = get("lin");
    if kinds == 0 {
        let (Some(start), Some(stop), Some(given_step)) = (start, stop, step) else {
            return Err(error(
                "compose -> either one of (lin, log, dec, oct) or all of (start, stop, and step) \
                 must be given"
                    .into(),
            ));
        };
        let step_value = if (stop - start > 0.0) != (given_step > 0.0) {
            -given_step
        } else {
            given_step
        };
        lin = Some((stop - start) / step_value + 1.0);
        step = None;
    }
    let limit = |length: usize| -> Result<usize, ControlExecutionError> {
        if length > crate::resource::ResourceLimits::default().max_result_values / 2 {
            Err(error(format!(
                "compose of {length} elements exceeds the result values limit"
            )))
        } else {
            Ok(length)
        }
    };
    let samples: Vec<Value> = if let Some(lin) = lin {
        if lin <= 0.0 {
            return Err(error("compose -> can't have lin <= 0".into()));
        }
        let length = limit(lin as usize)?;
        if let (Some(given_step), Some(start), Some(stop)) = (step, start, stop)
            && given_step != (stop - start) / (lin - 1.0)
        {
            // com_compose.c:381 warns and keeps start, stop and lin.
            step = None;
        }
        let start = match start {
            Some(start) => start,
            None => match (stop, step) {
                (Some(stop), Some(step)) => stop - step * (lin - 1.0),
                (Some(stop), None) => stop - lin + 1.0,
                _ => 0.0,
            },
        };
        let stop = stop.unwrap_or(match step {
            Some(step) => start + step * (lin - 1.0),
            None => start + lin - 1.0,
        });
        let step = step.unwrap_or((stop - start) / (lin - 1.0));
        let mut value = start;
        (0..length)
            .map(|_| {
                let current = value;
                value += step;
                current
            })
            .collect()
    } else {
        let (Some(start), Some(stop)) = (start, stop) else {
            return Err(error("compose -> start and stop must be given".into()));
        };
        if start <= 0.0 || stop <= 0.0 {
            return Err(error("compose -> start and stop must be positive".into()));
        }
        let count = if let Some(dec) = get("dec") {
            (dec.trunc() * (stop / start).log10()).round() + 1.0
        } else if let Some(oct) = get("oct") {
            (oct.trunc() * (stop / start).log10() / 2f64.log10()).round() + 1.0
        } else {
            get("log").unwrap_or_default().trunc()
        };
        if count < 1.0 {
            return Err(error("compose -> the sweep has no points".into()));
        }
        let length = limit(count as usize)?;
        if length == 1 {
            vec![start]
        } else {
            (0..length)
                .map(|index| start * (stop / start).powf(index as Value / (count - 1.0)))
                .collect()
        }
    };
    Ok(VectorValue {
        samples: samples.into_iter().map(ComplexValue::from).collect(),
        complex: false,
        scale: None,
        unit: SignalUnit::Unspecified,
    })
}

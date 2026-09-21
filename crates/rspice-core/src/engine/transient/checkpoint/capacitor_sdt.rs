//! Accepted integration memory of behavioral capacitance expressions.
use super::*;
use crate::expr::AcceptedSdtState;

#[derive(Debug, Clone, PartialEq)]
pub(super) struct State {
    pub name: String,
    pub integrals: Vec<AcceptedSdtState>,
}

pub(super) fn capture(circuit: &CircuitData) -> Vec<State> {
    circuit
        .capacitors
        .value_expressions
        .iter()
        .flatten()
        .filter(|expression| expression.program.sdt_count != 0)
        .map(|expression| State {
            name: expression.name.clone(),
            integrals: expression.accepted_sdt_history(),
        })
        .collect()
}

pub(super) fn validate_target(
    circuit: &CircuitData,
    states: Option<&[State]>,
) -> Result<(), String> {
    let Some(states) = states else {
        return if circuit.capacitors.has_stateful_value_expressions() {
            Err("legacy transient checkpoint does not contain capacitor SDT history; re-run the transient from t=0".into())
        } else {
            Ok(())
        };
    };
    let expressions = || {
        circuit
            .capacitors
            .value_expressions
            .iter()
            .flatten()
            .filter(|expression| expression.program.sdt_count != 0)
    };
    if expressions().count() != states.len() {
        return Err("capacitor SDT checkpoint expression count does not match the circuit".into());
    }
    for (expression, state) in expressions().zip(states) {
        if expression.name != state.name || expression.program.sdt_count != state.integrals.len() {
            return Err(format!(
                "capacitor SDT checkpoint identity or operator count does not match '{}'",
                expression.name
            ));
        }
    }
    Ok(())
}

/// The whole target layout is validated before any checkpoint state is injected.
pub(super) fn restore(circuit: &mut CircuitData, states: Option<&[State]>) {
    let Some(states) = states else {
        return;
    };
    for (expression, state) in circuit
        .capacitors
        .value_expressions
        .iter_mut()
        .flatten()
        .filter(|expression| expression.program.sdt_count != 0)
        .zip(states)
    {
        expression.restore_sdt_history(&state.integrals);
    }
}

pub(super) fn write(
    out: &mut String,
    states: Option<&[State]>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!(
        "capacitor_sdt_state_available {}\ncapacitor_sdt_states {}\n",
        u8::from(states.is_some()),
        states.map_or(0, <[_]>::len)
    ));
    for (index, state) in states.unwrap_or_default().iter().enumerate() {
        poll_checkpoint_abort(abort, index)?;
        out.push_str(&format!(
            "capacitor_sdt_state {} {}\n",
            state.name,
            state.integrals.len()
        ));
        for (index, integral) in state.integrals.iter().enumerate() {
            poll_checkpoint_abort(abort, index)?;
            out.push_str(&format!(
                "capacitor_sdt {} {} {}\n",
                integral.time, integral.input, integral.integral
            ));
        }
    }
    Ok(())
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
) -> Result<Option<Vec<State>>, String> {
    let available = match lines.next().ok_or("missing capacitor SDT availability")? {
        "capacitor_sdt_state_available 0" => false,
        "capacitor_sdt_state_available 1" => true,
        _ => return Err("invalid capacitor SDT availability".into()),
    };
    let count = parse_count_header(
        lines.next().ok_or("missing capacitor SDT count")?,
        "capacitor_sdt_states",
    )?;
    if !available && count != 0 {
        return Err("unavailable capacitor SDT history cannot contain states".into());
    }
    let mut states = allocate_checkpoint_rows(lines, count, "capacitor SDT states", budget)?;
    for _ in 0..count {
        let mut fields = lines
            .next()
            .ok_or("missing capacitor SDT state")?
            .split_whitespace();
        if fields.next() != Some("capacitor_sdt_state") {
            return Err("invalid capacitor SDT state header".into());
        }
        let name = copy_checkpoint_string(
            fields.next().ok_or("missing capacitor SDT name")?,
            "capacitor SDT name",
            budget,
        )?;
        let count = fields
            .next()
            .ok_or("missing capacitor SDT operator count")?
            .parse::<usize>()
            .map_err(|_| "invalid capacitor SDT operator count")?;
        if fields.next().is_some() || count == 0 {
            return Err("invalid capacitor SDT state layout".into());
        }
        let mut integrals =
            allocate_checkpoint_rows(lines, count, "capacitor SDT integrals", budget)?;
        for index in 0..count {
            let mut fields = lines
                .next()
                .ok_or("missing capacitor SDT integral")?
                .split_whitespace();
            if fields.next() != Some("capacitor_sdt") {
                return Err("invalid capacitor SDT integral header".into());
            }
            let [time, input, integral] =
                read_fixed_finite_values(&mut fields, "capacitor", index, "SDT")?;
            if fields.next().is_some() {
                return Err("extra capacitor SDT integral fields".into());
            }
            integrals.push(AcceptedSdtState {
                time,
                input,
                integral,
            });
        }
        states.push(State { name, integrals });
    }
    Ok(available.then_some(states))
}

pub(super) fn validate(
    states: Option<&[State]>,
    time: Value,
    budget: &mut Option<&mut CheckpointParseBudget>,
) -> Result<(), String> {
    let states = states.unwrap_or_default();
    let mut names = match budget.as_deref_mut() {
        Some(budget) => {
            allocate_checkpoint_capacity(states.len(), "capacitor SDT validation names", budget)?
        }
        None => {
            let mut values = Vec::new();
            values
                .try_reserve_exact(states.len())
                .map_err(|_| "capacitor SDT validation names exceed allocation limits")?;
            values
        }
    };
    for state in states {
        if state.name.is_empty()
            || state.name.chars().any(char::is_whitespace)
            || state.integrals.is_empty()
        {
            return Err("invalid capacitor SDT checkpoint identity or operator count".into());
        }
        if state
            .integrals
            .iter()
            .any(|s| s.time != time || !s.input.is_finite() || !s.integral.is_finite())
        {
            return Err(format!(
                "capacitor SDT checkpoint for '{}' has invalid accepted history",
                state.name
            ));
        }
        names.push(state.name.as_str());
    }
    names.sort_unstable();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("duplicate capacitor SDT checkpoint source".into());
    }
    Ok(())
}

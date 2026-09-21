//! Bounded wire format for accepted behavioral integration state.
use super::*;
use crate::device::behavioral::BehavioralAcceptedState;
use crate::expr::AcceptedSdtState;

pub(super) fn write(
    out: &mut String,
    states: Option<&[BehavioralAcceptedState]>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!(
        "behavioral_state_available {}\nbehavioral_states {}\n",
        u8::from(states.is_some()),
        states.map_or(0, <[_]>::len)
    ));
    for (index, state) in states.unwrap_or_default().iter().enumerate() {
        poll_checkpoint_abort(abort, index)?;
        out.push_str(&format!(
            "behavioral_state {} {} {}\n",
            if state.voltage { "V" } else { "I" },
            state.name,
            state.integrals.len()
        ));
        for (index, integral) in state.integrals.iter().enumerate() {
            poll_checkpoint_abort(abort, index)?;
            out.push_str(&format!(
                "behavioral_sdt {} {} {}\n",
                integral.time, integral.input, integral.integral
            ));
        }
    }
    Ok(())
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
) -> Result<Option<Vec<BehavioralAcceptedState>>, String> {
    let available = match lines
        .next()
        .ok_or("missing behavioral state availability")?
    {
        "behavioral_state_available 0" => false,
        "behavioral_state_available 1" => true,
        _ => return Err("invalid behavioral state availability".into()),
    };
    let count = parse_count_header(
        lines.next().ok_or("missing behavioral state count")?,
        "behavioral_states",
    )?;
    if !available && count != 0 {
        return Err("unavailable behavioral history cannot contain states".into());
    }
    let mut states = allocate_checkpoint_rows(lines, count, "behavioral states", budget)?;
    for _ in 0..count {
        let mut fields = lines
            .next()
            .ok_or("missing behavioral source state")?
            .split_whitespace();
        if fields.next() != Some("behavioral_state") {
            return Err("invalid behavioral source state header".into());
        }
        let voltage = match fields.next() {
            Some("V") => true,
            Some("I") => false,
            _ => return Err("invalid behavioral source state kind".into()),
        };
        let name = copy_checkpoint_string(
            fields.next().ok_or("missing behavioral source name")?,
            "behavioral source name",
            budget,
        )?;
        let count = fields
            .next()
            .ok_or("missing behavioral SDT count")?
            .parse::<usize>()
            .map_err(|_| "invalid behavioral SDT count")?;
        if fields.next().is_some() || count == 0 {
            return Err("invalid behavioral SDT source layout".into());
        }
        let mut integrals = allocate_checkpoint_rows(lines, count, "behavioral integrals", budget)?;
        for index in 0..count {
            let mut fields = lines
                .next()
                .ok_or("missing behavioral integral")?
                .split_whitespace();
            if fields.next() != Some("behavioral_sdt") {
                return Err("invalid behavioral integral header".into());
            }
            let [time, input, integral] =
                read_fixed_finite_values(&mut fields, "behavioral", index, "SDT")?;
            if fields.next().is_some() {
                return Err("extra behavioral integral fields".into());
            }
            integrals.push(AcceptedSdtState {
                time,
                input,
                integral,
            });
        }
        states.push(BehavioralAcceptedState {
            voltage,
            name,
            integrals,
        });
    }
    Ok(available.then_some(states))
}

pub(super) fn validate(
    states: Option<&[BehavioralAcceptedState]>,
    time: Value,
    budget: &mut Option<&mut CheckpointParseBudget>,
) -> Result<(), String> {
    let states = states.unwrap_or_default();
    let mut names = match budget.as_deref_mut() {
        Some(budget) => {
            allocate_checkpoint_capacity(states.len(), "behavioral validation names", budget)?
        }
        None => {
            let mut values = Vec::new();
            values
                .try_reserve_exact(states.len())
                .map_err(|_| "behavioral validation names exceed allocation limits")?;
            values
        }
    };
    for state in states {
        if state.name.is_empty()
            || state.name.chars().any(char::is_whitespace)
            || state.integrals.is_empty()
        {
            return Err("invalid behavioral SDT checkpoint identity or operator count".into());
        }
        if state
            .integrals
            .iter()
            .any(|s| s.time != time || !s.input.is_finite() || !s.integral.is_finite())
        {
            return Err(format!(
                "behavioral SDT checkpoint for '{}' has invalid accepted history",
                state.name
            ));
        }
        names.push(state.name.as_str());
    }
    names.sort_unstable();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("duplicate behavioral SDT checkpoint source".into());
    }
    Ok(())
}

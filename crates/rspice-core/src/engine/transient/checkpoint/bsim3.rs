//! Versioned BSIM3 accepted device and integration histories.
use super::*;
use crate::device::mosfet::bsim3v3::device::checkpoint::AcceptedBsim3NonlinearCheckpoint;

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
    checkpoint: &mut AcceptedJunctionTransientHistoryCheckpoint,
) -> Result<(), String> {
    let count = parse_count_header(
        lines.next().ok_or("missing accepted BSIM3 states")?,
        "accepted_bsim3_states",
    )?;
    if count > lines.remaining() {
        return Err("accepted BSIM3 state count exceeds remaining rows".to_string());
    }
    checkpoint.bsim3_states = allocate_checkpoint_capacity(count, "BSIM3 accepted states", budget)?;
    for (name, values) in checkpoint.bsim3_history.columns_mut() {
        *values = allocate_checkpoint_capacity(count, name, budget)?;
    }
    for row in 0..count {
        let mut fields = lines
            .next()
            .ok_or("truncated accepted BSIM3 states")?
            .split_whitespace();
        if fields.next() != Some("accepted_bsim3_state") {
            return Err("malformed accepted BSIM3 state row".to_string());
        }
        let instance_name = copy_checkpoint_string(
            fields.next().ok_or("missing BSIM3 instance")?,
            "BSIM3 instance",
            budget,
        )?;
        let runtime_tag = copy_checkpoint_string(
            fields.next().ok_or("missing BSIM3 runtime tag")?,
            "BSIM3 runtime tag",
            budget,
        )?;
        let mode = fields
            .next()
            .ok_or("missing BSIM3 mode")?
            .parse::<i32>()
            .map_err(|_| "invalid BSIM3 mode")?;
        let seed_evaluations = fields
            .next()
            .ok_or("missing BSIM3 startup count")?
            .parse::<u8>()
            .map_err(|_| "invalid BSIM3 startup count")?;
        let mut flags = [false; 7];
        for flag in &mut flags {
            *flag = read_history_bool(&mut fields, "BSIM3", row, "nonlinear flag")?;
        }
        let values = read_fixed_finite_values(&mut fields, "BSIM3", row, "nonlinear")?;
        let state = AcceptedBsim3NonlinearCheckpoint {
            instance_name,
            runtime_tag,
            values,
            flags,
            mode,
            seed_evaluations,
        };
        state.validate_numeric_state()?;
        checkpoint.bsim3_states.push(state);
        for (name, values) in checkpoint.bsim3_history.columns_mut() {
            values.push(read_finite_history_value(&mut fields, "BSIM3", row, name)?);
        }
        if fields.next().is_some() {
            return Err("extra field in accepted BSIM3 state".to_string());
        }
    }
    let mut fields = lines
        .next()
        .ok_or("missing BSIM3 transient timestep history")?
        .split_whitespace();
    if fields.next() != Some("accepted_bsim3_transient_dt") {
        return Err("malformed BSIM3 transient timestep history".to_string());
    }
    checkpoint.bsim3_history.accepted_dt_prev =
        read_finite_history_value(&mut fields, "BSIM3", count, "accepted_dt_prev")?;
    checkpoint.bsim3_history.accepted_dt_prev_prev =
        read_finite_history_value(&mut fields, "BSIM3", count, "accepted_dt_prev_prev")?;
    if fields.next().is_some() {
        return Err("extra field in BSIM3 transient timestep history".to_string());
    }
    Ok(())
}

pub(super) fn validate(
    checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    budget: &mut Option<&mut CheckpointParseBudget>,
) -> Result<(), String> {
    checkpoint
        .bsim3_history
        .validate(checkpoint.bsim3_states.len())?;
    let mut names = if let Some(budget) = budget.as_deref_mut() {
        allocate_checkpoint_capacity(
            checkpoint.bsim3_states.len(),
            "BSIM3 validation names",
            budget,
        )?
    } else {
        let mut names = Vec::new();
        names
            .try_reserve_exact(checkpoint.bsim3_states.len())
            .map_err(|_| "BSIM3 validation names exceed allocation limits")?;
        names
    };
    for state in &checkpoint.bsim3_states {
        state.validate_numeric_state()?;
        names.push(state.instance_name.as_str());
    }
    names.sort_unstable();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("accepted BSIM3 state has duplicate instance names".to_string());
    }
    Ok(())
}

pub(super) fn write(
    out: &mut String,
    checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!(
        "accepted_bsim3_states {}\n",
        checkpoint.bsim3_states.len()
    ));
    for (row, state) in checkpoint.bsim3_states.iter().enumerate() {
        poll_checkpoint_abort(abort, row)?;
        out.push_str(&format!(
            "accepted_bsim3_state {} {} {} {}",
            state.instance_name, state.runtime_tag, state.mode, state.seed_evaluations
        ));
        for flag in state.flags {
            out.push(' ');
            out.push(if flag { '1' } else { '0' });
        }
        for value in state.values {
            out.push(' ');
            out.push_str(&value.to_string());
        }
        for (_, values) in checkpoint.bsim3_history.columns() {
            out.push(' ');
            out.push_str(&values[row].to_string());
        }
        out.push('\n');
    }
    out.push_str(&format!(
        "accepted_bsim3_transient_dt {} {}\n",
        checkpoint.bsim3_history.accepted_dt_prev, checkpoint.bsim3_history.accepted_dt_prev_prev
    ));
    Ok(())
}

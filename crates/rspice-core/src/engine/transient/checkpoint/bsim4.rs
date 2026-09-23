//! Versioned BSIM4 accepted device and integration histories.
use super::*;
use crate::device::mosfet::bsim4v8::device::checkpoint::AcceptedBsim4NonlinearCheckpoint;

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
    checkpoint: &mut AcceptedJunctionTransientHistoryCheckpoint,
) -> Result<(), String> {
    let count = parse_count_header(
        lines.next().ok_or("missing accepted BSIM4 states")?,
        "accepted_bsim4_states",
    )?;
    if count > lines.remaining() {
        return Err("accepted BSIM4 state count exceeds remaining rows".to_string());
    }
    checkpoint.bsim4_states = allocate_checkpoint_capacity(count, "BSIM4 accepted states", budget)?;
    for (name, values) in checkpoint.bsim4_history.columns_mut() {
        *values = allocate_checkpoint_capacity(count, name, budget)?;
    }
    for row in 0..count {
        let mut fields = lines
            .next()
            .ok_or("truncated accepted BSIM4 states")?
            .split_whitespace();
        if fields.next() != Some("accepted_bsim4_state") {
            return Err("malformed accepted BSIM4 state row".to_string());
        }
        let instance_name = copy_checkpoint_string(
            fields.next().ok_or("missing BSIM4 instance")?,
            "BSIM4 instance",
            budget,
        )?;
        let runtime_tag = copy_checkpoint_string(
            fields.next().ok_or("missing BSIM4 runtime tag")?,
            "BSIM4 runtime tag",
            budget,
        )?;
        let mode = fields
            .next()
            .ok_or("missing BSIM4 mode")?
            .parse::<i32>()
            .map_err(|_| "invalid BSIM4 mode")?;
        let seed_evaluations = fields
            .next()
            .ok_or("missing BSIM4 startup count")?
            .parse::<u8>()
            .map_err(|_| "invalid BSIM4 startup count")?;
        let mut flags = [false; 7];
        for flag in &mut flags {
            *flag = read_history_bool(&mut fields, "BSIM4", row, "nonlinear flag")?;
        }
        let values = read_fixed_finite_values(&mut fields, "BSIM4", row, "nonlinear")?;
        let state = AcceptedBsim4NonlinearCheckpoint {
            instance_name,
            runtime_tag,
            values,
            flags,
            mode,
            seed_evaluations,
        };
        state.validate_numeric_state()?;
        checkpoint.bsim4_states.push(state);
        for (name, values) in checkpoint.bsim4_history.columns_mut() {
            values.push(read_finite_history_value(&mut fields, "BSIM4", row, name)?);
        }
        if fields.next().is_some() {
            return Err("extra field in accepted BSIM4 state".to_string());
        }
    }
    let mut fields = lines
        .next()
        .ok_or("missing BSIM4 transient timestep history")?
        .split_whitespace();
    if fields.next() != Some("accepted_bsim4_transient_dt") {
        return Err("malformed BSIM4 transient timestep history".to_string());
    }
    checkpoint.bsim4_history.accepted_dt_prev =
        read_finite_history_value(&mut fields, "BSIM4", count, "accepted_dt_prev")?;
    checkpoint.bsim4_history.accepted_dt_prev_prev =
        read_finite_history_value(&mut fields, "BSIM4", count, "accepted_dt_prev_prev")?;
    if fields.next().is_some() {
        return Err("extra field in BSIM4 transient timestep history".to_string());
    }
    Ok(())
}

pub(super) fn validate(
    checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    budget: &mut Option<&mut CheckpointParseBudget>,
) -> Result<(), String> {
    checkpoint
        .bsim4_history
        .validate(checkpoint.bsim4_states.len())?;
    let mut names = if let Some(budget) = budget.as_deref_mut() {
        allocate_checkpoint_capacity(
            checkpoint.bsim4_states.len(),
            "BSIM4 validation names",
            budget,
        )?
    } else {
        let mut names = Vec::new();
        names
            .try_reserve_exact(checkpoint.bsim4_states.len())
            .map_err(|_| "BSIM4 validation names exceed allocation limits")?;
        names
    };
    for state in &checkpoint.bsim4_states {
        state.validate_numeric_state()?;
        names.push(state.instance_name.as_str());
    }
    names.sort_unstable();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("accepted BSIM4 state has duplicate instance names".to_string());
    }
    Ok(())
}

pub(super) fn write(
    out: &mut String,
    checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!(
        "accepted_bsim4_states {}\n",
        checkpoint.bsim4_states.len()
    ));
    for (row, state) in checkpoint.bsim4_states.iter().enumerate() {
        poll_checkpoint_abort(abort, row)?;
        out.push_str(&format!(
            "accepted_bsim4_state {} {} {} {}",
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
        for (_, values) in checkpoint.bsim4_history.columns() {
            out.push(' ');
            out.push_str(&values[row].to_string());
        }
        out.push('\n');
    }
    out.push_str(&format!(
        "accepted_bsim4_transient_dt {} {}\n",
        checkpoint.bsim4_history.accepted_dt_prev, checkpoint.bsim4_history.accepted_dt_prev_prev
    ));
    Ok(())
}

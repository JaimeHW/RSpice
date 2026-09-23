//! Versioned MOSFET accepted device and integration histories.
use super::*;
use crate::device::mosfet::AcceptedMosfetNonlinearCheckpoint;

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
    checkpoint: &mut AcceptedJunctionTransientHistoryCheckpoint,
) -> Result<(), String> {
    let count = parse_count_header(
        lines.next().ok_or("missing accepted MOSFET states")?,
        "accepted_mosfet_states",
    )?;
    if count > lines.remaining() {
        return Err("accepted MOSFET state count exceeds remaining rows".to_string());
    }
    let mut fields = lines
        .next()
        .ok_or("missing MOSFET extended history flag")?
        .split_whitespace();
    if fields.next() != Some("accepted_mosfet_extended_junction_history") {
        return Err("malformed MOSFET extended history flag".into());
    }
    let extended = read_history_bool(&mut fields, "MOSFET", 0, "extended junction history")?;
    if fields.next().is_some() {
        return Err("extra field in MOSFET extended history flag".into());
    }
    checkpoint.mosfet_history.accepted_displacement_currents =
        allocate_checkpoint_capacity(count, "MOSFET displacement currents", budget)?;
    checkpoint.mosfet_states =
        allocate_checkpoint_capacity(count, "MOSFET accepted states", budget)?;
    for (name, values) in checkpoint.mosfet_history.columns_mut() {
        if extended || !MosfetTransientHistory::is_extended_column(name) {
            *values = allocate_checkpoint_capacity(count, name, budget)?;
        }
    }
    for row in 0..count {
        let mut fields = lines
            .next()
            .ok_or("truncated accepted MOSFET states")?
            .split_whitespace();
        if fields.next() != Some("accepted_mosfet_state") {
            return Err("malformed accepted MOSFET state row".to_string());
        }
        let instance_name = copy_checkpoint_string(
            fields.next().ok_or("missing MOSFET instance")?,
            "MOSFET instance",
            budget,
        )?;
        let runtime_tag = copy_checkpoint_string(
            fields.next().ok_or("missing MOSFET runtime tag")?,
            "MOSFET runtime tag",
            budget,
        )?;
        let level = fields
            .next()
            .ok_or("missing MOSFET level")?
            .parse::<i32>()
            .map_err(|_| "invalid MOSFET level")?;
        let junction_model = fields
            .next()
            .ok_or("missing MOSFET junction model")?
            .parse::<u8>()
            .map_err(|_| "invalid MOSFET junction model")?;
        let region = fields
            .next()
            .ok_or("missing MOSFET region")?
            .parse::<u8>()
            .map_err(|_| "invalid MOSFET region")?;
        let seed_evaluations = fields
            .next()
            .ok_or("missing MOSFET startup count")?
            .parse::<u8>()
            .map_err(|_| "invalid MOSFET startup count")?;
        let mut flags = [false; 5];
        for flag in &mut flags {
            *flag = read_history_bool(&mut fields, "MOSFET", row, "nonlinear flag")?;
        }
        let values = read_fixed_finite_values(&mut fields, "MOSFET", row, "nonlinear")?;
        let state = AcceptedMosfetNonlinearCheckpoint {
            instance_name,
            runtime_tag,
            values,
            flags,
            level,
            junction_model,
            region,
            seed_evaluations,
        };
        state.validate_numeric_state()?;
        checkpoint.mosfet_states.push(state);
        for (name, values) in checkpoint.mosfet_history.columns_mut() {
            if extended || !MosfetTransientHistory::is_extended_column(name) {
                values.push(read_finite_history_value(&mut fields, "MOSFET", row, name)?);
            }
        }
        checkpoint
            .mosfet_history
            .accepted_displacement_currents
            .push(read_fixed_finite_values(
                &mut fields,
                "MOSFET",
                row,
                "displacement currents",
            )?);
        if fields.next().is_some() {
            return Err("extra field in accepted MOSFET state".to_string());
        }
    }
    let mut fields = lines
        .next()
        .ok_or("missing MOSFET transient timestep history")?
        .split_whitespace();
    if fields.next() != Some("accepted_mosfet_transient_dt") {
        return Err("malformed MOSFET transient timestep history".to_string());
    }
    checkpoint.mosfet_history.accepted_dt_prev =
        read_finite_history_value(&mut fields, "MOSFET", count, "accepted_dt_prev")?;
    checkpoint.mosfet_history.accepted_dt_prev_prev =
        read_finite_history_value(&mut fields, "MOSFET", count, "accepted_dt_prev_prev")?;
    if fields.next().is_some() {
        return Err("extra field in MOSFET transient timestep history".to_string());
    }
    let expected_extended = checkpoint.mosfet_states.iter().any(|state| state.flags[4]);
    if extended != expected_extended {
        return Err("MOSFET extended history presence does not match the captured models".into());
    }
    Ok(())
}

pub(super) fn validate(
    checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    budget: &mut Option<&mut CheckpointParseBudget>,
) -> Result<(), String> {
    checkpoint.mosfet_history.validate(
        checkpoint.mosfet_states.len(),
        checkpoint.mosfet_states.iter().any(|state| state.flags[4]),
    )?;
    let mut names = if let Some(budget) = budget.as_deref_mut() {
        allocate_checkpoint_capacity(
            checkpoint.mosfet_states.len(),
            "MOSFET validation names",
            budget,
        )?
    } else {
        let mut names = Vec::new();
        names
            .try_reserve_exact(checkpoint.mosfet_states.len())
            .map_err(|_| "MOSFET validation names exceed allocation limits")?;
        names
    };
    for state in &checkpoint.mosfet_states {
        state.validate_numeric_state()?;
        names.push(state.instance_name.as_str());
    }
    names.sort_unstable();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("accepted MOSFET state has duplicate instance names".to_string());
    }
    Ok(())
}

pub(super) fn write(
    out: &mut String,
    checkpoint: &AcceptedJunctionTransientHistoryCheckpoint,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!(
        "accepted_mosfet_states {}\n",
        checkpoint.mosfet_states.len()
    ));
    let extended = checkpoint.mosfet_states.iter().any(|state| state.flags[4]);
    out.push_str(&format!(
        "accepted_mosfet_extended_junction_history {}\n",
        usize::from(extended)
    ));
    for (row, state) in checkpoint.mosfet_states.iter().enumerate() {
        poll_checkpoint_abort(abort, row)?;
        out.push_str(&format!(
            "accepted_mosfet_state {} {} {} {} {} {}",
            state.instance_name,
            state.runtime_tag,
            state.level,
            state.junction_model,
            state.region,
            state.seed_evaluations
        ));
        for flag in state.flags {
            out.push(' ');
            out.push(if flag { '1' } else { '0' });
        }
        for value in state.values {
            out.push(' ');
            out.push_str(&value.to_string());
        }
        for (name, values) in checkpoint.mosfet_history.columns() {
            if !extended && MosfetTransientHistory::is_extended_column(name) {
                continue;
            }
            out.push(' ');
            out.push_str(&values[row].to_string());
        }
        for value in checkpoint.mosfet_history.accepted_displacement_currents[row] {
            out.push(' ');
            out.push_str(&value.to_string());
        }
        out.push('\n');
    }
    out.push_str(&format!(
        "accepted_mosfet_transient_dt {} {}\n",
        checkpoint.mosfet_history.accepted_dt_prev, checkpoint.mosfet_history.accepted_dt_prev_prev
    ));
    Ok(())
}

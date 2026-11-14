//! Accepted voltage- and current-controlled switch state.
//!
//! A switch's hysteresis is history: whether it conducts at a control inside
//! its band depends on which side it last left, so a resume that rebuilt the
//! switch from its card would continue a different trajectory. Every
//! `S`/`W` switch therefore carries its complete accepted runtime here, in
//! circuit order: the voltage-controlled switches, then the current-controlled
//! ones. The expression-controlled Xyce switches keep their own store vectors
//! (`generic_switch_stores`).
use super::*;
use crate::device::{AcceptedSwitchCheckpoint, SwitchCheckpointKind};

pub(super) fn capture(circuit: &CircuitData) -> Vec<AcceptedSwitchCheckpoint> {
    circuit
        .vswitches
        .iter()
        .map(crate::device::VoltageSwitch::accepted_checkpoint)
        .chain(
            circuit
                .iswitches
                .iter()
                .map(crate::device::CurrentSwitch::accepted_checkpoint),
        )
        .collect()
}

fn switch_count(circuit: &CircuitData) -> usize {
    circuit.vswitches.len() + circuit.iswitches.len()
}

/// Prove every row belongs to the target's switch at its position before any
/// state is injected.
pub(super) fn validate_target(
    circuit: &CircuitData,
    states: &[AcceptedSwitchCheckpoint],
) -> Result<(), String> {
    if states.len() != switch_count(circuit) {
        return Err(format!(
            "checkpoint switch state shape mismatch: captured {}, circuit has {}",
            states.len(),
            switch_count(circuit)
        ));
    }
    let (voltage, current) = states.split_at(circuit.vswitches.len());
    for (switch, state) in circuit.vswitches.iter().zip(voltage) {
        switch.validate_accepted_checkpoint(state)?;
    }
    for (switch, state) in circuit.iswitches.iter().zip(current) {
        switch.validate_accepted_checkpoint(state)?;
    }
    Ok(())
}

/// Inject the validated rows.
pub(super) fn restore(
    circuit: &mut CircuitData,
    states: &[AcceptedSwitchCheckpoint],
) -> Result<(), String> {
    validate_target(circuit, states)?;
    let (voltage, current) = states.split_at(circuit.vswitches.len());
    for (switch, state) in circuit.vswitches.iter_mut().zip(voltage) {
        switch.restore_accepted_checkpoint(state)?;
    }
    for (switch, state) in circuit.iswitches.iter_mut().zip(current) {
        switch.restore_accepted_checkpoint(state)?;
    }
    Ok(())
}

pub(super) fn validate(states: &[AcceptedSwitchCheckpoint]) -> Result<(), String> {
    states
        .iter()
        .try_for_each(AcceptedSwitchCheckpoint::validate_numeric_state)
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(states: &[AcceptedSwitchCheckpoint]) -> usize {
    states.len().saturating_mul(6)
}

pub(super) fn write(
    out: &mut String,
    states: &[AcceptedSwitchCheckpoint],
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    out.push_str(&format!("accepted_switch_states {}\n", states.len()));
    for (index, state) in states.iter().enumerate() {
        poll_checkpoint_abort(abort, index)?;
        out.push_str("accepted_switch_state ");
        out.push_str(state.kind.tag());
        out.push(' ');
        out.push_str(&state.instance_name);
        for value in state.values {
            out.push(' ');
            out.push_str(&value.to_string());
        }
        for code in state.codes {
            out.push(' ');
            out.push_str(&code.to_string());
        }
        out.push('\n');
    }
    Ok(())
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
) -> Result<Vec<AcceptedSwitchCheckpoint>, String> {
    let count = parse_count_header(
        lines.next().ok_or("missing accepted switch state count")?,
        "accepted_switch_states",
    )?;
    let mut states = allocate_checkpoint_rows(lines, count, "accepted switch states", budget)?;
    for row in 0..count {
        let mut fields = lines
            .next()
            .ok_or("missing accepted switch state row")?
            .split_whitespace();
        if fields.next() != Some("accepted_switch_state") {
            return Err(format!(
                "accepted switch state row {row} has an invalid header"
            ));
        }
        let kind = fields
            .next()
            .and_then(SwitchCheckpointKind::from_tag)
            .ok_or_else(|| format!("accepted switch state row {row} has an invalid family"))?;
        let instance_name = copy_checkpoint_string(
            fields
                .next()
                .ok_or_else(|| format!("accepted switch state row {row} names no instance"))?,
            "accepted switch instance name",
            budget,
        )?;
        let values = read_fixed_finite_values(&mut fields, "switch", row, "values")?;
        let mut codes = [0_u8; 12];
        for (index, code) in codes.iter_mut().enumerate() {
            *code = fields
                .next()
                .and_then(|field| field.parse::<u8>().ok())
                .ok_or_else(|| {
                    format!("accepted switch state row {row} has an invalid code {index}")
                })?;
        }
        if fields.next().is_some() {
            return Err(format!("accepted switch state row {row} has extra fields"));
        }
        let state = AcceptedSwitchCheckpoint {
            instance_name,
            kind,
            values,
            codes,
        };
        state.validate_numeric_state()?;
        states.push(state);
    }
    Ok(states)
}

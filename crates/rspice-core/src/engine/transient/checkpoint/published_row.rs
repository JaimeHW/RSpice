//! The row a run published at the checkpoint time, where it did not come
//! from the restart state.
//!
//! A physical startup publishes the operating point (or the UIC point) at
//! t = 0 and leaves t = 0 from its outgoing state; an event step publishes
//! the left limit, the step's own solution, and the integration leaves the
//! event from the outgoing state. The checkpoint's `solution` is that restart
//! state, so a resumed run that rebuilt its first row from it published a
//! right limit where the uninterrupted run published the left one. Such a
//! checkpoint carries the published row: every retained node voltage, branch
//! current and device operating-point value at its time.
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::engine::transient) struct PublishedRow {
    voltages: Vec<(usize, Value)>,
    branch_currents: Vec<(usize, Value)>,
    device_values: Vec<(String, String, Value)>,
}

/// Where a published row is read from: the last row the run wrote to its
/// result sink, and the result that holds the device operating-point traces.
#[derive(Clone, Copy)]
pub(in crate::engine::transient) struct PublishedSource<'a> {
    pub(in crate::engine::transient) result: &'a crate::analysis::transient::TransientResult,
    pub(in crate::engine::transient) row: crate::abort_signal::TransientRow<'a>,
    /// Accepted samples the run has written, this one included.
    pub(in crate::engine::transient) points: usize,
    /// The device values a result sink keeps in the row, by `(device, parameter)`, and the last
    /// row's values of them; empty when the result holds the device traces.
    pub(in crate::engine::transient) device_columns: &'a [(String, String)],
    pub(in crate::engine::transient) device_row: &'a [Value],
}

impl PublishedRow {
    /// The last row the run wrote, in every trace that retains a value per
    /// sample.
    pub(in crate::engine::transient) fn last_of(
        source: PublishedSource<'_>,
    ) -> Result<Self, String> {
        let PublishedSource {
            result,
            row,
            points: samples,
            device_columns,
            device_row,
        } = source;
        if samples == 0 {
            return Err("a published checkpoint row needs a sample".into());
        }
        let row = Self {
            voltages: row.node_voltages().collect(),
            branch_currents: row.branch_currents().collect(),
            device_values: if device_columns.is_empty() {
                result
                    .device_op_traces
                    .iter()
                    .filter(|trace| trace.values.len() == samples)
                    .map(|trace| {
                        (
                            trace.device_name.clone(),
                            trace.parameter.clone(),
                            trace.values[samples - 1],
                        )
                    })
                    .collect()
            } else {
                device_columns
                    .iter()
                    .zip(device_row)
                    .map(|((device, parameter), value)| (device.clone(), parameter.clone(), *value))
                    .collect()
            },
        };
        row.validate()?;
        Ok(row)
    }

    /// Overwrite the first row of a resumed run with this one: the row its
    /// sink holds open, and the device traces of its result.
    pub(in crate::engine::transient) fn publish_as_first_row(
        &self,
        result: &mut crate::analysis::transient::TransientResult,
        rows: &mut super::super::result_rows::TransientRows<'_>,
    ) -> Result<(), crate::SimulationError> {
        let missing = |kind: &str, index: usize| {
            crate::SimulationError::Circuit(format!(
                "checkpoint published {kind} {index} is not in this run"
            ))
        };
        for &(index, value) in &self.voltages {
            if index >= rows.layout().num_nodes() {
                return Err(missing("voltage", index));
            }
            rows.amend_first_voltage(index, value)?;
        }
        for &(index, value) in &self.branch_currents {
            if index >= rows.layout().num_branches() {
                return Err(missing("branch current", index));
            }
            rows.amend_first_current(index, value)?;
        }
        for (device, parameter, value) in &self.device_values {
            let column =
                rows.layout()
                    .device_ops()
                    .iter()
                    .position(|(known_device, known_parameter)| {
                        known_device.eq_ignore_ascii_case(device)
                            && known_parameter.eq_ignore_ascii_case(parameter)
                    });
            if let Some(column) = column {
                rows.amend_first_device_op(column, *value)?;
                continue;
            }
            if let Some(first) = result
                .device_op_traces
                .iter_mut()
                .find(|trace| {
                    trace.device_name.eq_ignore_ascii_case(device)
                        && trace.parameter.eq_ignore_ascii_case(parameter)
                })
                .and_then(|trace| trace.values.first_mut())
            {
                *first = *value;
            }
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        let ascending = |values: &[(usize, Value)]| {
            values.windows(2).all(|pair| pair[0].0 < pair[1].0)
                && values.iter().all(|(_, value)| value.is_finite())
        };
        if !ascending(&self.voltages) || !ascending(&self.branch_currents) {
            return Err(
                "checkpoint published row traces must be ascending, unique and finite".into(),
            );
        }
        let name = |name: &str| !name.is_empty() && !name.contains(char::is_whitespace);
        if self.device_values.iter().any(|(device, parameter, value)| {
            !name(device) || !name(parameter) || !value.is_finite()
        }) {
            return Err(
                "checkpoint published device values need whitespace-free names and finite values"
                    .into(),
            );
        }
        Ok(())
    }
}

pub(super) fn validate(row: Option<&PublishedRow>) -> Result<(), String> {
    row.map_or(Ok(()), PublishedRow::validate)
}

/// Values a parsed image retains, for the checkpoint memory budget.
pub(super) fn retained_values(row: Option<&PublishedRow>) -> usize {
    row.map_or(0, |row| {
        row.voltages
            .len()
            .saturating_add(row.branch_currents.len())
            .saturating_add(row.device_values.len().saturating_mul(3))
    })
    .saturating_add(1)
}

pub(super) fn write(
    out: &mut String,
    row: Option<&PublishedRow>,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let Some(row) = row else {
        out.push_str("published_row none\n");
        return Ok(());
    };
    out.push_str(&format!(
        "published_row {} {} {}\n",
        row.voltages.len(),
        row.branch_currents.len(),
        row.device_values.len()
    ));
    let mut written = 0usize;
    for (kind, values) in [
        ("published_voltage", &row.voltages),
        ("published_current", &row.branch_currents),
    ] {
        for (index, value) in values {
            poll_checkpoint_abort(abort, written)?;
            written += 1;
            out.push_str(&format!("{kind} {index} {value}\n"));
        }
    }
    for (device, parameter, value) in &row.device_values {
        poll_checkpoint_abort(abort, written)?;
        written += 1;
        out.push_str(&format!("published_device {device} {parameter} {value}\n"));
    }
    Ok(())
}

pub(super) fn read(
    lines: &mut CheckpointLines<'_>,
    budget: &mut CheckpointParseBudget,
) -> Result<Option<PublishedRow>, String> {
    let line = lines.next().ok_or("missing published row line")?;
    let malformed = |line: &str| format!("malformed published row line: '{line}'");
    let fields = line.split_whitespace().collect::<Vec<_>>();
    let counts = match fields.as_slice() {
        ["published_row", "none"] => return Ok(None),
        ["published_row", voltages, currents, devices] => [voltages, currents, devices]
            .map(|field| field.parse::<usize>().map_err(|_| malformed(line))),
        _ => return Err(malformed(line)),
    };
    let [voltages, currents, devices] = counts;
    let (voltages, currents, devices) = (voltages?, currents?, devices?);
    let mut indexed = |kind: &str, count: usize, budget: &mut CheckpointParseBudget| {
        let mut values = allocate_checkpoint_capacity(count, kind, budget)?;
        for _ in 0..count {
            let line = lines.next().ok_or_else(|| format!("missing {kind} line"))?;
            match line.split_whitespace().collect::<Vec<_>>().as_slice() {
                [name, index, value] if *name == kind => values.push((
                    index.parse::<usize>().map_err(|_| malformed(line))?,
                    value.parse::<Value>().map_err(|_| malformed(line))?,
                )),
                _ => return Err(malformed(line)),
            }
        }
        Ok::<_, String>(values)
    };
    let voltages = indexed("published_voltage", voltages, budget)?;
    let branch_currents = indexed("published_current", currents, budget)?;
    let mut device_values = allocate_checkpoint_capacity::<(String, String, Value)>(
        devices,
        "published_device",
        budget,
    )?;
    for _ in 0..devices {
        let line = lines.next().ok_or("missing published_device line")?;
        match line.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["published_device", device, parameter, value] => device_values.push((
                (*device).to_owned(),
                (*parameter).to_owned(),
                value.parse::<Value>().map_err(|_| malformed(line))?,
            )),
            _ => return Err(malformed(line)),
        }
    }
    let row = PublishedRow {
        voltages,
        branch_currents,
        device_values,
    };
    row.validate()?;
    Ok(Some(row))
}

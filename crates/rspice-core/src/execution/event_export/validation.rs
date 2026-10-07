//! Validate the event carrier before decoding or coalescing any timeline.
use super::*;

pub(super) fn validate(plot: &RawWaveformData, kind: RawEventKind) -> Result<(), EventPlotError> {
    let invalid = |message: String| EventPlotError::InvalidData {
        plot: plot.header.plotname.clone(),
        message,
    };
    let columns = plot.variables.len();
    if kind == RawEventKind::Bus && columns < 2 {
        return Err(EventPlotError::BusVariableCount {
            title: plot.header.title.clone(),
            variables: columns,
        });
    }
    if kind != RawEventKind::Bus && columns != 2 {
        return Err(EventPlotError::VariableCount {
            plot: plot.header.plotname.clone(),
            variables: columns,
        });
    }
    if plot.header.no_variables != columns
        || plot.waveforms.len() != columns
        || plot
            .waveforms
            .iter()
            .any(|waveform| waveform.y.len() != plot.header.no_points)
    {
        return Err(invalid(
            "column lengths must match the declared event dimensions".into(),
        ));
    }
    if plot.header.is_complex
        || plot
            .waveforms
            .iter()
            .any(|waveform| waveform.y_imag.is_some())
    {
        return Err(invalid(
            "event histories require real columns; complex components cannot be discarded".into(),
        ));
    }
    let (Some(axis), Some(time_column)) = (plot.variables.first(), plot.waveforms.first()) else {
        return Err(invalid("event history has no time column".into()));
    };
    if !axis.var_type.eq_ignore_ascii_case("time") {
        return Err(invalid(
            "event histories require a time coordinate in seconds".into(),
        ));
    }
    // Version 1 event plots encode SI seconds. Table provenance must not
    // silently reinterpret that coordinate; scaled tables can use the ordinary
    // table conversion path instead of declaring themselves event histories.
    let units = crate::io::ltspice_raw::raw_table_units(&plot.header)
        .map_err(|error| invalid(error.to_string()))?;
    if let Some(unit) = units
        .as_ref()
        .and_then(|units| units.first())
        .and_then(Option::as_deref)
        && !matches!(unit.trim(), "s" | "sec" | "second" | "seconds")
    {
        return Err(invalid(format!(
            "version 1 event time must be in seconds, not '{unit}'"
        )));
    }
    let times = &time_column.y;
    let mut previous = None;
    for (index, &time) in times.iter().enumerate() {
        if !time.is_finite() || time < 0.0 {
            return Err(invalid(format!(
                "event time at row {index} must be finite and non-negative"
            )));
        }
        if previous.is_some_and(|previous| time < previous) {
            return Err(invalid(format!("event time decreases at row {index}")));
        }
        previous = Some(time);
    }
    // The scalar digital decoder checks every event code as it materializes
    // the history. Bus columns may be skipped, and real values need admission.
    if kind == RawEventKind::Digital {
        return Ok(());
    }
    for (variable, waveform) in plot.variables.iter().zip(&plot.waveforms).skip(1) {
        for (&time, &value) in times.iter().zip(&waveform.y) {
            // Validate bus columns even when a scalar member plot supplies
            // the authoritative history and this column will be skipped.
            if kind == RawEventKind::Bus && event_code(value).is_none() {
                return Err(EventPlotError::DigitalCode {
                    node: kind
                        .node_name(&variable.name)
                        .unwrap_or(&variable.name)
                        .to_owned(),
                    time,
                    value,
                });
            }
            if kind == RawEventKind::Real && !value.is_finite() {
                return Err(invalid(format!(
                    "event value for '{}' at t={time} must be finite",
                    variable.name
                )));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;

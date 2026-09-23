//! Exact lifetime plots from retained calibration and electrical observations.

use super::*;
use rspice_core::engine::ReliabilityRunResult;
use std::sync::Arc;

impl AnalysisResultPayload {
    pub(crate) fn reliability_display_traces(
        response: &ReliabilityRunResult,
    ) -> Result<Vec<WaveformData>, String> {
        response
            .validate_retained_payload_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort,
            )
            .map_err(|e| e.to_string())?;
        let years = Arc::new(response.stress.request.target_years.clone());
        let mut traces = Vec::new();
        let mut names = std::collections::HashSet::new();
        let mut values = 0usize;
        let mut push = |name: String, unit: &str, y: Vec<f64>| -> Result<(), String> {
            values = values.saturating_add(y.len());
            if values > rspice_core::ResourceLimits::default().max_result_values {
                return Err("Reliability display exceeds the retained scalar limit".into());
            }
            if y.len() != years.len() || !names.insert(name.clone()) {
                return Err("Reliability display shape or identity is invalid".into());
            }
            traces.push(WaveformData::new(name, years.clone(), y, "#f5b700").with_unit(unit));
            Ok(())
        };
        for (device_index, device) in response.stress.checkpoints[0].devices.iter().enumerate() {
            for (contribution_index, contribution) in device.contributions.iter().enumerate() {
                let rows: Vec<_> = response
                    .stress
                    .checkpoints
                    .iter()
                    .map(|c| &c.devices[device_index].contributions[contribution_index])
                    .collect();
                let label = format!("{} / {}", device.device, contribution.model_id);
                push(
                    if contribution.trap_occupancies.is_empty() {
                        format!("Equivalent age({label})")
                    } else {
                        format!("Elapsed history({label})")
                    },
                    "s",
                    rows.iter().map(|c| c.equivalent_seconds).collect(),
                )?;
                for (index, trap) in contribution.trap_occupancies.iter().enumerate() {
                    push(
                        format!("Occupancy({label} / {})", trap.trap_id),
                        "1",
                        rows.iter()
                            .map(|c| c.trap_occupancies[index].occupancy)
                            .collect(),
                    )?;
                }
                if contribution.electromigration_lifetime_fraction.is_some() {
                    push(
                        format!("EM consumed lifetime({label})"),
                        "1",
                        rows.iter()
                            .map(|c| c.electromigration_lifetime_fraction.unwrap())
                            .collect(),
                    )?;
                }
                for (parameter_index, parameter) in contribution.parameters.iter().enumerate() {
                    let unit = if parameter.update
                        == rspice_core::analysis::reliability::AgingParameterUpdate::Relative
                    {
                        "1"
                    } else {
                        parameter_unit(&parameter.parameter)
                    };
                    push(
                        format!("Shift({label} / {})", parameter.parameter),
                        unit,
                        rows.iter()
                            .map(|c| c.parameters[parameter_index].shift)
                            .collect(),
                    )?;
                }
            }
        }
        for (phase_index, phase) in response.stress.request.study.mission.iter().enumerate() {
            let points = &response.aged[phase_index * years.len()..(phase_index + 1) * years.len()];
            let label = format!("phase {} {}", phase_index + 1, phase.name);
            for (parameter_index, parameter) in points[0].parameters.iter().enumerate() {
                let name = format!("{}:{} / {label}", parameter.device, parameter.parameter);
                push(
                    format!("Aged({name})"),
                    parameter_unit(&parameter.parameter),
                    points
                        .iter()
                        .map(|p| p.parameters[parameter_index].aged_value)
                        .collect(),
                )?;
                push(
                    format!("Fresh({name})"),
                    parameter_unit(&parameter.parameter),
                    points
                        .iter()
                        .map(|p| p.parameters[parameter_index].fresh_value)
                        .collect(),
                )?;
            }
            let fresh = response.stress.phases[phase_index]
                .fresh_operating_point
                .as_ref()
                .unwrap();
            for (kind, unit, baseline) in [
                (0, "V", &fresh.voltages),
                (1, "A", &fresh.branch_currents),
                (2, "device units", &fresh.device_observables),
            ] {
                for (name, value) in baseline {
                    let probe = match kind {
                        0 => format!("V({name})"),
                        1 => format!("I({name})"),
                        _ => format!("OP({name})"),
                    };
                    let sampled = points
                        .iter()
                        .map(|p| match kind {
                            0 => p.operating_point.voltages[name],
                            1 => p.operating_point.branch_currents[name],
                            _ => p.operating_point.device_observables[name],
                        })
                        .collect();
                    push(format!("Aged({probe} / {label})"), unit, sampled)?;
                    push(
                        format!("Fresh({probe} / {label})"),
                        unit,
                        vec![*value; years.len()],
                    )?;
                }
            }
        }
        traces.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(traces)
    }

    pub(crate) fn reliability_response_bytes(response: &ReliabilityRunResult) -> usize {
        // Count serialization without building a JSON allocation. This includes
        // the full metadata; numeric bytes are also counted explicitly.
        struct Counter(usize);
        impl std::io::Write for Counter {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                self.0 = self.0.saturating_add(bytes.len());
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut counter = Counter(0);
        if serde_json::to_writer(&mut counter, response).is_err() {
            return usize::MAX;
        }
        let point_count = |p: &rspice_core::engine::ReliabilityOperatingPoint| {
            p.voltages
                .len()
                .saturating_add(p.branch_currents.len())
                .saturating_add(p.device_observables.len())
        };
        let mut values = 0usize;
        for phase in &response.stress.phases {
            values = values
                .saturating_add(
                    phase
                        .time_s
                        .len()
                        .saturating_mul(1 + 4 * phase.devices.len()),
                )
                .saturating_add(phase.fresh_operating_point.as_ref().map_or(0, point_count));
        }
        for point in &response.aged {
            values = values
                .saturating_add(3 * point.parameters.len())
                .saturating_add(point_count(&point.operating_point));
        }
        counter.0.saturating_add(values.saturating_mul(8))
    }
}

fn parameter_unit(parameter: &str) -> &'static str {
    match parameter.to_ascii_uppercase().as_str() {
        "VTO" | "PHI" => "V",
        "KP" => "A/V²",
        "GAMMA" => "√V",
        _ => "model units",
    }
}

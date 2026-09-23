//! Reconstruct the full pre-origin delay window from exact HB port spectra.
use super::*;

impl Engine {
    pub(super) fn hb_initialize_transmission_line_history(
        &self,
        circuit: &mut CircuitData,
        result: &HbResult,
        history_step: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let branch_names = circuit.branch_names_sorted();
        let period = 1.0 / result.fundamental_freq;
        let mut total_samples = 0usize;
        for line in &mut circuit.tlines {
            line.reset();
            if line.is_memoryless_two_port() {
                continue;
            }
            let current_names = if let Some((one, two)) = line.ltra_branch_ordinals() {
                [one, two].map(|ordinal| branch_names[ordinal - 1].clone())
            } else {
                [
                    format!("{}#port1", line.name),
                    format!("{}#port2", line.name),
                ]
            };
            let mut currents = Vec::with_capacity(2);
            for name in &current_names {
                let current = result
                    .mna_branch_currents
                    .iter()
                    .find(|row| row.device_name.eq_ignore_ascii_case(name))
                    .ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "HB Envelope omitted transmission-line port current '{name}'"
                        ))
                    })?;
                currents.push(current.coefficients.clone());
            }
            let voltages = [
                Self::hb_terminal_voltage_spectrum(result, line.node1_pos, line.node1_neg),
                Self::hb_terminal_voltage_spectrum(result, line.node2_pos, line.node2_neg),
            ];
            let impedance = line.impedance();
            let mut step = history_step.min(line.delay() / 16.0);
            for port in 0..2 {
                // The native interpolator acts on the combined travelling wave,
                // not independently on terminal voltage and current. Bounds on
                // its second/third phase derivatives cover every phase, including
                // peaks that a finite set of midpoint probes could miss.
                let mut amplitude = 0.0;
                let mut second = 0.0;
                let mut third = 0.0;
                for (k, (voltage, current)) in
                    voltages[port].iter().zip(&currents[port]).enumerate()
                {
                    let magnitude = (voltage + impedance * current).norm();
                    amplitude += magnitude;
                    second += magnitude * (k as Value).powi(2);
                    third += magnitude * (k as Value).powi(3);
                }
                let tolerance = self.voltage_abstol()
                    + impedance * self.current_abstol()
                    + self.voltage_reltol() * amplitude;
                if !tolerance.is_finite() || !second.is_finite() || !third.is_finite() {
                    return Err(SimulationError::Circuit(format!(
                        "HB Envelope line '{}' has non-finite travelling-wave bounds",
                        line.name
                    )));
                }
                // Linear interpolation error is <= M2*h^2/8 and the native
                // three-point quadratic error <= M3*h^3/6 on a uniform grid.
                // Requiring each unscaled bound <= tolerance/8 leaves margin
                // for rounding and covers linear/quadratic/mixed/Xyce modes.
                if second > 0.0 {
                    step = step.min(period / TAU * (tolerance / (8.0 * second)).sqrt());
                }
                if third > 0.0 {
                    step = step.min(period / TAU * (tolerance / (8.0 * third)).cbrt());
                }
            }
            if !step.is_finite() || step <= 0.0 {
                return Err(SimulationError::Circuit(format!(
                    "HB Envelope line '{}' has an unrepresentable history interval",
                    line.name
                )));
            }
            // Keep the full physical delay, even when TD exceeds the carrier
            // period. Three predecessors preserve the interpolation stencil at
            // -TD; TD/16 also keeps them inside the native 1.5*TD retention span.
            let required_intervals = (line.delay() / step).ceil();
            if !required_intervals.is_finite()
                || required_intervals >= (usize::MAX / 11 - 4) as Value
            {
                return Err(SimulationError::Circuit(format!(
                    "HB Envelope line '{}' delay history is too large to represent",
                    line.name
                )));
            }
            let intervals = required_intervals as usize + 3;
            total_samples = total_samples.saturating_add(intervals.saturating_add(1));
            crate::ResourceLimitError::ensure(
                crate::ResourceKind::AnalysisPoints,
                total_samples,
                self.config.resource_limits.max_analysis_points,
            )?;
            crate::ResourceLimitError::ensure(
                crate::ResourceKind::ResultValues,
                total_samples.saturating_mul(11),
                self.config.resource_limits.max_result_values,
            )?;
            let mut previous_time = Value::NEG_INFINITY;
            for index in (0..=intervals).rev() {
                if index % 128 == 0 && abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let time = if index == 0 {
                    0.0
                } else {
                    -(index as Value) * step
                };
                if !time.is_finite() || time <= previous_time {
                    return Err(SimulationError::Circuit(format!(
                        "HB Envelope line '{}' history times cannot be resolved",
                        line.name
                    )));
                }
                previous_time = time;
                let phase = TAU * (time % period) / period;
                let evaluate = |coefficients: &[Complex64]| {
                    coefficients
                        .iter()
                        .enumerate()
                        .map(|(k, coefficient)| {
                            (coefficient * Complex64::from_polar(1.0, k as Value * phase)).re
                        })
                        .sum::<Value>()
                };
                let [v1, v2] = voltages.each_ref().map(|row| evaluate(row));
                let i1 = evaluate(&currents[0]);
                let i2 = evaluate(&currents[1]);
                if [v1, i1, v2, i2].iter().any(|value| !value.is_finite()) {
                    return Err(SimulationError::Circuit(format!(
                        "HB Envelope line '{}' has non-finite periodic history",
                        line.name
                    )));
                }
                line.update_history(time, v1, i1, v2, i2);
            }
            // Check exactly the state that checkpoint restore will consume.
            line.checkpoint_state().map_err(SimulationError::Circuit)?;
        }
        Ok(())
    }
}

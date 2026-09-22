//! Continuous variable-capacitance law shared by carrier and response solves.
use super::*;
use crate::circuit::Capacitors;
use crate::device::{MatrixStamper, passive::SolutionDependentCapacitor};

#[derive(Debug, Clone)]
pub(super) struct PeriodicCapacitor {
    pub expression: SolutionDependentCapacitor,
    pub pos: usize,
    pub neg: usize,
    pub multiplier: Value,
}

impl PeriodicCapacitor {
    pub(super) fn rate_name(&self) -> String {
        format!("C:{}:voltage_rate", self.expression.name)
    }

    pub(super) fn stamp(
        &mut self,
        point: crate::device::behavioral::BehavioralFqPoint<'_>,
        frequency: Value,
        coordinates: (usize, usize),
        f: &mut impl MatrixStamper,
        q: &mut impl MatrixStamper,
    ) -> Result<(), String> {
        let (integral_start, rate) = coordinates;
        let solution = point.inputs;
        self.expression
            .stamp_periodic_integrals(point, integral_start, f, q)?;
        let sample = self
            .expression
            .sample_periodic_capacitance(point, integral_start)?;
        let capacitance = self.multiplier * sample.value;
        let current = capacitance * frequency * solution[rate];
        for (node, sign) in [(self.pos, 1.0), (self.neg, -1.0)] {
            f.stamp_rhs(node, -sign * current);
            f.stamp(node, rate + 1, sign * capacitance * frequency);
            for &(column, partial) in &sample.partials {
                f.stamp(
                    node,
                    column + 1,
                    sign * self.multiplier * partial * frequency * solution[rate],
                );
            }
        }
        // r = dV/dt / f0. This voltage-valued descriptor coordinate avoids
        // substituting d(C*V)/dt for C*dV/dt. In a response solve the Q column
        // supplies the perturbed input frequency, including its sideband.
        f.stamp_rhs(rate + 1, solution[rate]);
        f.stamp(rate + 1, rate + 1, -1.0);
        for (node, sign) in [(self.pos, 1.0), (self.neg, -1.0)] {
            let voltage = node.checked_sub(1).map_or(0.0, |index| solution[index]);
            q.stamp_rhs(rate + 1, -sign * voltage / frequency);
            q.stamp(rate + 1, node, sign / frequency);
        }
        Ok(())
    }
}

impl HbSolver {
    /// Configure before behavioral primitives are prepared: their independent
    /// input discovery must include capacitor dependencies from the start.
    pub(crate) fn set_periodic_capacitors(
        &mut self,
        capacitors: &Capacitors,
        autonomous: bool,
        quasi_periodic: bool,
        retained: bool,
    ) -> Result<(), HbError> {
        if self.physical_branch_count() != self.exact_mna_branches().len() {
            return Err(HbError::InvalidCircuit(
                "capacitors must precede periodic auxiliary registration".into(),
            ));
        }
        let mut devices = Vec::new();
        let period = self.config.fundamental_freq.recip();
        for (index, expression) in capacitors.value_expressions.iter().enumerate() {
            let Some(expression) = expression else {
                continue;
            };
            let fail = |reason| {
                HbError::InvalidCircuit(format!("capacitor '{}': {reason}", expression.name))
            };
            expression
                .validate_periodic_integral_rates()
                .map_err(HbError::InvalidCircuit)?;
            if capacitors.ic_branch_indices[index].is_some() {
                return Err(fail("periodic IC branch equations are unavailable"));
            }
            // QPSS validates clocks while lifting onto its actual tone grid,
            // after all physical and integral coordinates have been counted.
            if !quasi_periodic && !expression.has_periodic_shooting_equation(period, autonomous) {
                return Err(fail(
                    "capacitance is not certified periodic over the configured period",
                ));
            }
            if retained && !expression.has_periodic_response_context() {
                return Err(fail(
                    "live-frequency capacitance has no periodic response equation",
                ));
            }
            if expression
                .bound_solution_indices()
                .any(|column| column >= self.num_nodes + self.exact_mna_branches().len())
            {
                return Err(fail("expression references an unavailable MNA coordinate"));
            }
            if !quasi_periodic && !autonomous {
                let cycles = expression.max_authored_tone_cycles(period);
                let interval = expression.minimum_periodic_collocation_interval();
                let points = (2.0 * cycles.round() + 1.0)
                    .max(interval.map_or(0.0, |interval| (2.0 * period / interval).ceil()));
                if !cycles.is_finite()
                    || cycles.round() > self.num_harmonics as Value
                    || !points.is_finite()
                    || points > self.fft.size() as Value
                    || interval.is_some_and(|value| !value.is_finite() || value <= 0.0)
                {
                    return Err(fail(
                        "configured harmonics or collocation points cannot resolve the capacitance expression",
                    ));
                }
            }
            devices.push(PeriodicCapacitor {
                expression: expression.clone(),
                pos: capacitors.stamps[index].pp.row,
                neg: capacitors.stamps[index].nn.row,
                multiplier: capacitors.capacitances[index],
            });
        }
        self.periodic_capacitors = devices;
        Ok(())
    }

    pub(crate) fn capacitor_rate_start(&self) -> usize {
        self.physical_branch_count()
            + self.behavioral_sources.integral_count()
            + self
                .periodic_capacitors
                .iter()
                .map(|cap| cap.expression.program.sdt_count)
                .sum::<usize>()
    }
}

//! Instantaneous forcing for oscillator phase-noise projection.
use super::*;
use crate::device::behavioral::integrals::BehavioralFqPoint;

impl Capacitors {
    pub(crate) fn integral_rate_directions(
        &self,
        solution: &[Value],
        direction: &[Value],
        integrals: &[Value],
        time: Value,
    ) -> Result<Vec<Value>, String> {
        if integrals.len() != self.integral_count() || solution.len() != direction.len() {
            return Err("capacitor integral noise direction has inconsistent dimensions".into());
        }
        let mut output = Vec::with_capacity(integrals.len());
        for expression in self.value_expressions.iter().flatten() {
            expression.append_integral_rate_directions(
                solution,
                direction,
                &integrals[output.len()..],
                time,
                &mut output,
            )?;
        }
        Ok(output)
    }

    /// At fixed integral state, i=C(x,z)*v' has tangent
    /// delta_i = C*s*delta_v + v'*C_x*delta_x. Accumulated transient
    /// charge-history derivatives instead describe changes to a past orbit;
    /// using them here would introduce a false control-node mass term.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn stamp_frozen_noise(
        &mut self,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        voltage_rates: &[Value],
        time: Value,
        frozen_rate: Value,
        num_nodes: usize,
    ) -> Result<(), String> {
        if voltage_rates.len() != self.len() || !frozen_rate.is_finite() || frozen_rate <= 0.0 {
            return Err("capacitor noise linearization has invalid rates".into());
        }
        let mut inputs = solution.to_vec();
        inputs.extend(self.accepted_integrals());
        let point = BehavioralFqPoint {
            inputs: &inputs,
            time,
            num_nodes,
            unknowns: inputs.len(),
            integral_start: solution.len(),
            prescribed_integrals: &[],
        };
        let mut state_start = solution.len();
        for (index, expression) in self.value_expressions.iter_mut().enumerate() {
            let Some(expression) = expression else {
                continue;
            };
            let sample = expression.sample_periodic_capacitance(point, state_start)?;
            state_start += expression.program.sdt_count;
            let scale = self.capacitances[index];
            let capacitance = scale * sample.value;
            let velocity = voltage_rates[index];
            if !capacitance.is_finite() || capacitance < 0.0 || !velocity.is_finite() {
                return Err(format!(
                    "capacitor '{}' has invalid noise capacitance or voltage rate",
                    self.names[index]
                ));
            }
            let stamp = self.stamps[index];
            let mut terms: Vec<_> = sample
                .partials
                .into_iter()
                .filter(|(column, _)| *column < solution.len())
                .map(|(column, partial)| (column, scale * velocity * partial))
                .collect();
            let conductance = frozen_rate * capacitance;
            if stamp.pp.row > 0 {
                terms.push((stamp.pp.row - 1, conductance));
            }
            if stamp.nn.row > 0 {
                terms.push((stamp.nn.row - 1, -conductance));
            }
            if !conductance.is_finite() || terms.iter().any(|(_, value)| !value.is_finite()) {
                return Err(format!(
                    "capacitor '{}' has a non-finite noise derivative",
                    self.names[index]
                ));
            }
            if let Some(ordinal) = self.ic_branch_indices[index] {
                let branch = num_nodes + ordinal - 1;
                if stamp.pp.row != stamp.nn.row {
                    if stamp.pp.row > 0 {
                        matrix.add(stamp.pp.row - 1, branch, 1.0);
                    }
                    if stamp.nn.row > 0 {
                        matrix.add(stamp.nn.row - 1, branch, -1.0);
                    }
                }
                // Match transient row scaling without changing physical I(C).
                let row_scale = if conductance > 1.0 {
                    -1.0 / conductance
                } else {
                    1.0
                };
                matrix.add(branch, branch, row_scale);
                for (column, value) in terms {
                    matrix.add(branch, column, -row_scale * value);
                }
            } else if stamp.pp.row != stamp.nn.row {
                for (column, value) in terms {
                    if stamp.pp.row > 0 {
                        matrix.add(stamp.pp.row - 1, column, value);
                    }
                    if stamp.nn.row > 0 {
                        matrix.add(stamp.nn.row - 1, column, -value);
                    }
                }
            }
        }
        Ok(())
    }
}

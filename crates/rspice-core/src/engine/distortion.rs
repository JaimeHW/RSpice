//! Third-order small-signal Volterra distortion analysis.
//!
//! The nonlinear forcing terms are obtained by directionally differentiating
//! the complete small-signal MNA operator around the converged bias point.
//! This keeps static device currents and nonlinear charge/flux terms on the
//! same circuit-wide path and includes mixed derivatives across coupled
//! device terminals.

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::{
    AcResult, DistortionAnalysisResult, DistortionPointResult, DistortionProduct,
    DistortionProductResult,
};
use crate::solver::StaticMatrix;
use crate::{CircuitData, Complex64, Netlist, Value};
use std::f64::consts::PI;

impl Engine {
    /// Run harmonic (single-tone) or intermodulation (two-tone) small-signal
    /// distortion analysis through third Volterra order.
    ///
    /// When `f2_over_f1` is present, F2 is fixed at
    /// `f2_over_f1 * frequencies[0]` while F1 is swept, matching SPICE.
    pub fn run_distortion(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
        f2_over_f1: Option<Value>,
    ) -> Result<DistortionAnalysisResult, SimulationError> {
        self.run_distortion_with_abort(netlist, frequencies, f2_over_f1, &NoAbort)
    }

    /// Distortion analysis with cooperative cancellation between every
    /// nonlinear operator evaluation and sparse solve.
    pub fn run_distortion_with_abort(
        &self,
        netlist: &Netlist,
        frequencies: &[Value],
        f2_over_f1: Option<Value>,
        abort: &dyn AbortSignal,
    ) -> Result<DistortionAnalysisResult, SimulationError> {
        validate_distortion_request(frequencies, f2_over_f1)?;
        check_abort(abort)?;

        let engine = self.resolved_for_netlist(netlist);
        let mut circuit = engine.build_circuit(netlist)?;
        if !circuit.coupled_tlines.is_empty() {
            return Err(SimulationError::Circuit(
                "Distortion analysis does not support coupled multiconductor (CPL) transmission lines"
                    .to_string(),
            ));
        }
        Self::ensure_supported_dynamic_charges(&circuit, "Distortion")?;

        let mut matrix = engine.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);
        let dc_solution = engine.solve_dc_operating_point_with_abort(
            netlist,
            &mut circuit,
            &mut matrix,
            abort,
        )?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        Self::prepare_small_signal_state(&mut circuit, &dc_solution);

        let rhs_f1 = build_distortion_rhs(&circuit, DistortionInputTone::F1)?;
        if rhs_f1.iter().all(is_zero_complex) {
            return Err(SimulationError::Circuit(
                "Distortion analysis requires at least one non-zero DISTOF1 source excitation"
                    .to_string(),
            ));
        }

        let two_tone = f2_over_f1.is_some();
        let rhs_f2 = if two_tone {
            let rhs = build_distortion_rhs(&circuit, DistortionInputTone::F2)?;
            if rhs.iter().all(is_zero_complex) {
                return Err(SimulationError::Circuit(
                    "Two-tone distortion analysis requires at least one non-zero DISTOF2 source excitation"
                        .to_string(),
                ));
            }
            Some(rhs)
        } else {
            None
        };

        let f2 = f2_over_f1.map(|ratio| ratio * frequencies[0]);
        let num_nodes = circuit.num_nodes();
        let node_names = circuit.node_names_sorted();
        let branch_names = circuit.branch_names_sorted();
        let mut points = Vec::with_capacity(frequencies.len());

        for (index, &f1) in frequencies.iter().enumerate() {
            check_abort(abort)?;
            let mut context = VolterraContext {
                circuit: &mut circuit,
                matrix: &matrix,
                operating_state: &dc_solution,
                num_nodes,
                abort,
            };
            let h1 = context.solve(f1, &rhs_f1)?;
            let fundamental_f1 = make_ac_result(
                context.circuit,
                f1,
                &h1,
                2.0,
                num_nodes,
                &node_names,
                &branch_names,
            );

            let point = if let (Some(f2), Some(rhs_f2)) = (f2, rhs_f2.as_ref()) {
                context.two_tone_point(
                    f1,
                    f2,
                    &h1,
                    rhs_f2,
                    fundamental_f1,
                    &node_names,
                    &branch_names,
                )?
            } else {
                context.harmonic_point(f1, &h1, fundamental_f1, &node_names, &branch_names)?
            };
            points.push(point);
            abort.observe_progress((index + 1) as Value / frequencies.len() as Value);
        }

        Ok(DistortionAnalysisResult { f2_over_f1, points })
    }
}

#[derive(Clone, Copy)]
enum DistortionInputTone {
    F1,
    F2,
}

fn build_distortion_rhs(
    circuit: &CircuitData,
    tone: DistortionInputTone,
) -> Result<Vec<Complex64>, SimulationError> {
    let mut rhs = vec![Complex64::new(0.0, 0.0); circuit.matrix_size()];

    for index in 0..circuit.voltage_sources.len() {
        let Some(spec) = circuit.voltage_sources.source_specs[index].as_ref() else {
            continue;
        };
        let excitation = match tone {
            DistortionInputTone::F1 => spec.distortion_f1(),
            DistortionInputTone::F2 => spec.distortion_f2(),
        };
        let Some(excitation) = excitation else {
            continue;
        };
        validate_source_tone(&circuit.voltage_sources.names[index], excitation)?;
        let branch = circuit.get_branch_matrix_index(circuit.voltage_sources.branch_indices[index]);
        rhs[branch - 1] += Complex64::from_polar(0.5 * excitation.magnitude, excitation.phase);
    }

    for index in 0..circuit.current_sources.len() {
        let Some(spec) = circuit.current_sources.source_specs[index].as_ref() else {
            continue;
        };
        let excitation = match tone {
            DistortionInputTone::F1 => spec.distortion_f1(),
            DistortionInputTone::F2 => spec.distortion_f2(),
        };
        let Some(excitation) = excitation else {
            continue;
        };
        validate_source_tone(&circuit.current_sources.names[index], excitation)?;
        let current = Complex64::from_polar(0.5 * excitation.magnitude, excitation.phase);
        let positive = circuit.current_sources.node_pos[index];
        let negative = circuit.current_sources.node_neg[index];
        if positive > 0 {
            rhs[positive - 1] -= current;
        }
        if negative > 0 {
            rhs[negative - 1] += current;
        }
    }

    Ok(rhs)
}

fn validate_source_tone(
    source_name: &str,
    tone: crate::netlist::SourceDistortionTone,
) -> Result<(), SimulationError> {
    if !tone.magnitude.is_finite() || !tone.phase.is_finite() {
        return Err(SimulationError::Circuit(format!(
            "Distortion excitation on source '{source_name}' must have finite magnitude and phase"
        )));
    }
    Ok(())
}

struct VolterraContext<'a> {
    circuit: &'a mut CircuitData,
    matrix: &'a StaticMatrix,
    operating_state: &'a [Value],
    num_nodes: usize,
    abort: &'a dyn AbortSignal,
}

impl VolterraContext<'_> {
    fn solve(
        &mut self,
        frequency: Value,
        rhs: &[Complex64],
    ) -> Result<Vec<Complex64>, SimulationError> {
        check_abort(self.abort)?;
        self.circuit
            .prepare_behavioral_small_signal_at_frequency(self.operating_state, frequency);
        let mut operator = Engine::try_build_small_signal_ac_matrix(
            self.circuit,
            self.matrix,
            self.operating_state,
            2.0 * PI * frequency,
        )?;
        let solution = operator.solve(rhs)?;
        ensure_finite_vector(&solution, "distortion solve")?;
        check_abort(self.abort)?;
        Ok(solution)
    }

    fn solve_forcing(
        &mut self,
        frequency: Value,
        forcing: &[Complex64],
    ) -> Result<Vec<Complex64>, SimulationError> {
        let rhs: Vec<_> = forcing.iter().map(|value| -*value).collect();
        self.solve(frequency, &rhs)
    }

    fn harmonic_point(
        &mut self,
        f1: Value,
        h1: &[Complex64],
        fundamental_f1: AcResult,
        node_names: &[String],
        branch_names: &[String],
    ) -> Result<DistortionPointResult, SimulationError> {
        let forcing_2f1 = self.second_order_forcing(2.0 * f1, h1, h1)?;
        let h11 = self.solve_forcing(2.0 * f1, &forcing_2f1)?;

        let mut forcing_3f1 = self.second_order_forcing(3.0 * f1, h1, &h11)?;
        scale_vector_in_place(&mut forcing_3f1, 2.0);
        let third = self.third_order_forcing(3.0 * f1, h1, h1, h1)?;
        add_scaled_vector(&mut forcing_3f1, &third, 1.0);
        let h111 = self.solve_forcing(3.0 * f1, &forcing_3f1)?;

        Ok(DistortionPointResult {
            fundamental_f1,
            fundamental_f2: None,
            products: vec![
                make_product_result(
                    self.circuit,
                    DistortionProduct::SecondHarmonic,
                    2.0 * f1,
                    &h11,
                    2.0,
                    self.num_nodes,
                    node_names,
                    branch_names,
                ),
                make_product_result(
                    self.circuit,
                    DistortionProduct::ThirdHarmonic,
                    3.0 * f1,
                    &h111,
                    2.0,
                    self.num_nodes,
                    node_names,
                    branch_names,
                ),
            ],
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn two_tone_point(
        &mut self,
        f1: Value,
        f2: Value,
        h1: &[Complex64],
        rhs_f2: &[Complex64],
        fundamental_f1: AcResult,
        node_names: &[String],
        branch_names: &[String],
    ) -> Result<DistortionPointResult, SimulationError> {
        let h2 = self.solve(f2, rhs_f2)?;
        let h2_conjugate = conjugate_vector(&h2);

        let forcing_sum = self.second_order_forcing(f1 + f2, h1, &h2)?;
        let h12 = self.solve_forcing(f1 + f2, &forcing_sum)?;

        let forcing_difference = self.second_order_forcing(f1 - f2, h1, &h2_conjugate)?;
        let h1m2 = self.solve_forcing(f1 - f2, &forcing_difference)?;

        // The third-order 2F1-F2 recurrence also needs H2(F1,F1).
        let forcing_2f1 = self.second_order_forcing(2.0 * f1, h1, h1)?;
        let h11 = self.solve_forcing(2.0 * f1, &forcing_2f1)?;
        let output_frequency = 2.0 * f1 - f2;

        let mut forcing_third = self.second_order_forcing(output_frequency, h1, &h1m2)?;
        scale_vector_in_place(&mut forcing_third, 4.0 / 3.0);
        let second_partition = self.second_order_forcing(output_frequency, &h2_conjugate, &h11)?;
        add_scaled_vector(&mut forcing_third, &second_partition, 2.0 / 3.0);
        let cubic = self.third_order_forcing(output_frequency, h1, h1, &h2_conjugate)?;
        add_scaled_vector(&mut forcing_third, &cubic, 1.0);
        let h11m2 = self.solve_forcing(output_frequency, &forcing_third)?;

        Ok(DistortionPointResult {
            fundamental_f1,
            fundamental_f2: Some(make_ac_result(
                self.circuit,
                f2,
                &h2,
                2.0,
                self.num_nodes,
                node_names,
                branch_names,
            )),
            products: vec![
                make_product_result(
                    self.circuit,
                    DistortionProduct::Sum,
                    f1 + f2,
                    &h12,
                    4.0,
                    self.num_nodes,
                    node_names,
                    branch_names,
                ),
                make_product_result(
                    self.circuit,
                    DistortionProduct::Difference,
                    f1 - f2,
                    &h1m2,
                    4.0,
                    self.num_nodes,
                    node_names,
                    branch_names,
                ),
                make_product_result(
                    self.circuit,
                    DistortionProduct::ThirdOrderDifference,
                    output_frequency,
                    &h11m2,
                    6.0,
                    self.num_nodes,
                    node_names,
                    branch_names,
                ),
            ],
        })
    }

    /// Taylor coefficient `(F'' + jw*Q'')/2` contracted with two complex
    /// Volterra state vectors.
    fn second_order_forcing(
        &mut self,
        output_frequency: Value,
        left: &[Complex64],
        right: &[Complex64],
    ) -> Result<Vec<Complex64>, SimulationError> {
        let (left_real, left_imag) = split_complex_vector(left);
        let mut result =
            self.real_first_operator_derivative(output_frequency, &left_real, right)?;
        let imaginary_direction =
            self.real_first_operator_derivative(output_frequency, &left_imag, right)?;
        add_i_scaled_vector(&mut result, &imaginary_direction, 1.0);
        scale_vector_in_place(&mut result, 0.5);
        ensure_finite_vector(&result, "second-order Volterra forcing")?;
        Ok(result)
    }

    /// Taylor coefficient `(F''' + jw*Q''')/6` contracted with three complex
    /// Volterra state vectors.
    fn third_order_forcing(
        &mut self,
        output_frequency: Value,
        first: &[Complex64],
        second: &[Complex64],
        third: &[Complex64],
    ) -> Result<Vec<Complex64>, SimulationError> {
        let (first_real, first_imag) = split_complex_vector(first);
        let (second_real, second_imag) = split_complex_vector(second);

        let mut result = self.real_second_operator_derivative(
            output_frequency,
            &first_real,
            &second_real,
            third,
        )?;
        let imag_imag = self.real_second_operator_derivative(
            output_frequency,
            &first_imag,
            &second_imag,
            third,
        )?;
        add_scaled_vector(&mut result, &imag_imag, -1.0);

        let real_imag = self.real_second_operator_derivative(
            output_frequency,
            &first_real,
            &second_imag,
            third,
        )?;
        let imag_real = self.real_second_operator_derivative(
            output_frequency,
            &first_imag,
            &second_real,
            third,
        )?;
        add_i_scaled_vector(&mut result, &real_imag, 1.0);
        add_i_scaled_vector(&mut result, &imag_real, 1.0);
        scale_vector_in_place(&mut result, 1.0 / 6.0);
        ensure_finite_vector(&result, "third-order Volterra forcing")?;
        Ok(result)
    }

    fn real_first_operator_derivative(
        &mut self,
        output_frequency: Value,
        direction: &[Value],
        vector: &[Complex64],
    ) -> Result<Vec<Complex64>, SimulationError> {
        let Some((normalized, direction_scale)) =
            normalize_real_direction(self.operating_state, direction, self.num_nodes)?
        else {
            return Ok(zero_vector(vector.len()));
        };
        let h = Value::EPSILON.cbrt();
        let coarse = self.central_first_difference(output_frequency, &normalized, vector, h)?;
        let fine = self.central_first_difference(output_frequency, &normalized, vector, 0.5 * h)?;
        Ok(coarse
            .iter()
            .zip(fine.iter())
            .map(|(coarse, fine)| direction_scale * (4.0 * *fine - *coarse) / 3.0)
            .collect())
    }

    fn central_first_difference(
        &mut self,
        output_frequency: Value,
        direction: &[Value],
        vector: &[Complex64],
        h: Value,
    ) -> Result<Vec<Complex64>, SimulationError> {
        let plus = self.operator_product_at_offset(output_frequency, &[(direction, h)], vector)?;
        let minus =
            self.operator_product_at_offset(output_frequency, &[(direction, -h)], vector)?;
        Ok(plus
            .iter()
            .zip(minus.iter())
            .map(|(plus, minus)| (*plus - *minus) / (2.0 * h))
            .collect())
    }

    fn real_second_operator_derivative(
        &mut self,
        output_frequency: Value,
        first: &[Value],
        second: &[Value],
        vector: &[Complex64],
    ) -> Result<Vec<Complex64>, SimulationError> {
        let Some((first, first_scale)) =
            normalize_real_direction(self.operating_state, first, self.num_nodes)?
        else {
            return Ok(zero_vector(vector.len()));
        };
        let Some((second, second_scale)) =
            normalize_real_direction(self.operating_state, second, self.num_nodes)?
        else {
            return Ok(zero_vector(vector.len()));
        };
        let h = Value::EPSILON.sqrt().sqrt();
        let pp = self.operator_product_at_offset(
            output_frequency,
            &[(&first, h), (&second, h)],
            vector,
        )?;
        let pm = self.operator_product_at_offset(
            output_frequency,
            &[(&first, h), (&second, -h)],
            vector,
        )?;
        let mp = self.operator_product_at_offset(
            output_frequency,
            &[(&first, -h), (&second, h)],
            vector,
        )?;
        let mm = self.operator_product_at_offset(
            output_frequency,
            &[(&first, -h), (&second, -h)],
            vector,
        )?;
        let scale = first_scale * second_scale / (4.0 * h * h);
        Ok((0..vector.len())
            .map(|index| scale * (pp[index] - pm[index] - mp[index] + mm[index]))
            .collect())
    }

    fn operator_product_at_offset(
        &mut self,
        output_frequency: Value,
        offsets: &[(&[Value], Value)],
        vector: &[Complex64],
    ) -> Result<Vec<Complex64>, SimulationError> {
        check_abort(self.abort)?;
        let mut state = self.operating_state.to_vec();
        for &(direction, coefficient) in offsets {
            if direction.len() != state.len() {
                return Err(SimulationError::Circuit(format!(
                    "Volterra direction length {} does not match MNA state length {}",
                    direction.len(),
                    state.len()
                )));
            }
            for (value, delta) in state.iter_mut().zip(direction.iter()) {
                *value += coefficient * *delta;
            }
        }
        if state.iter().any(|value| !value.is_finite()) {
            return Err(SimulationError::Circuit(
                "Volterra perturbation produced a non-finite operating state".to_string(),
            ));
        }
        let operator = Engine::try_build_small_signal_ac_matrix_at_state(
            self.circuit,
            self.matrix,
            &state,
            2.0 * PI * output_frequency,
        )?;
        let product = operator.multiply_vector(vector)?;
        ensure_finite_vector(&product, "perturbed small-signal operator product")?;
        check_abort(self.abort)?;
        Ok(product)
    }
}

fn validate_distortion_request(
    frequencies: &[Value],
    f2_over_f1: Option<Value>,
) -> Result<(), SimulationError> {
    if frequencies.is_empty() {
        return Err(SimulationError::Circuit(
            "Distortion analysis requires at least one F1 frequency".to_string(),
        ));
    }
    if let Some((index, frequency)) = frequencies
        .iter()
        .enumerate()
        .find(|(_, frequency)| !frequency.is_finite() || **frequency <= 0.0)
    {
        return Err(SimulationError::Circuit(format!(
            "Distortion F1 frequency at index {index} must be finite and positive, got {frequency}"
        )));
    }
    if let Some(ratio) = f2_over_f1
        && (!ratio.is_finite() || ratio <= 0.0 || ratio >= 1.0)
    {
        return Err(SimulationError::Circuit(format!(
            "f2_over_f1 must be finite and strictly between 0 and 1, got {ratio}"
        )));
    }
    if let Some(ratio) = f2_over_f1 {
        let f2 = ratio * frequencies[0];
        if let Some((index, frequency)) = frequencies
            .iter()
            .enumerate()
            .find(|(_, frequency)| **frequency <= f2)
        {
            return Err(SimulationError::Circuit(format!(
                "Distortion F1 frequency at index {index} ({frequency}) must be greater than the fixed F2 frequency ({f2})"
            )));
        }
    }
    Ok(())
}

fn normalize_real_direction(
    operating_state: &[Value],
    direction: &[Value],
    num_nodes: usize,
) -> Result<Option<(Vec<Value>, Value)>, SimulationError> {
    if operating_state.len() != direction.len() {
        return Err(SimulationError::Circuit(format!(
            "Volterra direction length {} does not match MNA state length {}",
            direction.len(),
            operating_state.len()
        )));
    }
    if direction.iter().any(|value| !value.is_finite()) {
        return Err(SimulationError::Circuit(
            "Volterra direction contains a non-finite value".to_string(),
        ));
    }

    let direction_scale = direction
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let floor = if index < num_nodes { 1.0 } else { 1.0e-3 };
            let state_scale = operating_state[index].abs().max(floor);
            value.abs() / state_scale
        })
        .fold(0.0, Value::max);
    if direction_scale == 0.0 {
        return Ok(None);
    }
    Ok(Some((
        direction
            .iter()
            .map(|value| *value / direction_scale)
            .collect(),
        direction_scale,
    )))
}

fn make_product_result(
    circuit: &CircuitData,
    product: DistortionProduct,
    frequency: Value,
    kernel: &[Complex64],
    sinusoid_scale: Value,
    num_nodes: usize,
    node_names: &[String],
    branch_names: &[String],
) -> DistortionProductResult {
    DistortionProductResult {
        product,
        response: make_ac_result(
            circuit,
            frequency,
            kernel,
            sinusoid_scale,
            num_nodes,
            node_names,
            branch_names,
        ),
    }
}

fn make_ac_result(
    circuit: &CircuitData,
    frequency: Value,
    kernel: &[Complex64],
    sinusoid_scale: Value,
    num_nodes: usize,
    node_names: &[String],
    branch_names: &[String],
) -> AcResult {
    let voltages = kernel[..num_nodes]
        .iter()
        .map(|value| sinusoid_scale * *value)
        .collect::<Vec<_>>();
    let mut currents = kernel[num_nodes..]
        .iter()
        .map(|value| sinusoid_scale * *value)
        .collect::<Vec<_>>();
    let mut scaled_solution = Vec::with_capacity(voltages.len() + currents.len());
    scaled_solution.extend_from_slice(&voltages);
    scaled_solution.extend_from_slice(&currents);
    circuit.capacitors.project_complex_ic_branch_currents(
        &scaled_solution,
        &mut currents,
        2.0 * PI * frequency,
    );
    AcResult {
        frequency,
        node_names: node_names.to_vec(),
        branch_names: branch_names.to_vec(),
        voltages,
        currents,
    }
}

fn ensure_finite_vector(values: &[Complex64], context: &str) -> Result<(), SimulationError> {
    if let Some((index, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.re.is_finite() || !value.im.is_finite())
    {
        return Err(SimulationError::Circuit(format!(
            "{context} produced non-finite value {value} at MNA index {index}"
        )));
    }
    Ok(())
}

fn split_complex_vector(values: &[Complex64]) -> (Vec<Value>, Vec<Value>) {
    (
        values.iter().map(|value| value.re).collect(),
        values.iter().map(|value| value.im).collect(),
    )
}

fn conjugate_vector(values: &[Complex64]) -> Vec<Complex64> {
    values.iter().map(|value| value.conj()).collect()
}

fn add_scaled_vector(target: &mut [Complex64], addend: &[Complex64], scale: Value) {
    debug_assert_eq!(target.len(), addend.len());
    for (target, addend) in target.iter_mut().zip(addend.iter()) {
        *target += scale * *addend;
    }
}

fn add_i_scaled_vector(target: &mut [Complex64], addend: &[Complex64], scale: Value) {
    debug_assert_eq!(target.len(), addend.len());
    let multiplier = Complex64::new(0.0, scale);
    for (target, addend) in target.iter_mut().zip(addend.iter()) {
        *target += multiplier * *addend;
    }
}

fn scale_vector_in_place(values: &mut [Complex64], scale: Value) {
    for value in values {
        *value *= scale;
    }
}

fn zero_vector(length: usize) -> Vec<Complex64> {
    vec![Complex64::new(0.0, 0.0); length]
}

fn is_zero_complex(value: &Complex64) -> bool {
    *value == Complex64::new(0.0, 0.0)
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), SimulationError> {
    if abort.is_aborted() {
        Err(SimulationError::Aborted)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::ImmediateAbort;

    #[test]
    fn distortion_refreshes_frequency_dependent_behavioral_conductance() {
        let netlist = Netlist::parse_with_options(
            "live FREQ distortion operator\n\
             .PARAM RUNTIME_R={FREQ}\n\
             I1 out 0 DISTOF1 1 0\n\
             R1 out 0 {RUNTIME_R}\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::netlist::ExpressionDialect::Xyce,
                ..crate::netlist::NetlistParseOptions::default()
            },
        )
        .expect("frequency-dependent distortion deck parses");
        let engine = Engine::new(
            crate::engine::SimulationConfig::default()
                .with_spice_dialect(crate::engine::SpiceDialect::Xyce),
        );
        let results = engine
            .run_distortion(&netlist, &[10.0, 100.0], None)
            .expect("frequency-dependent distortion operators solve");
        for (point, expected) in results.points.iter().zip([10.0, 100.0]) {
            let actual = point.fundamental_f1.voltages[0].norm();
            assert!(
                (actual - expected).abs() <= 1.0e-10 * expected,
                "distortion operator retained stale FREQ conductance: actual={actual:e}, expected={expected:e}"
            );
        }
    }

    #[test]
    fn request_validation_rejects_invalid_frequencies_and_ratio() {
        assert!(validate_distortion_request(&[], None).is_err());
        assert!(validate_distortion_request(&[0.0], None).is_err());
        assert!(validate_distortion_request(&[1.0], Some(0.0)).is_err());
        assert!(validate_distortion_request(&[1.0], Some(1.0)).is_err());
        assert!(validate_distortion_request(&[1.0], Some(0.9)).is_ok());
        assert!(validate_distortion_request(&[1.0, 0.5], Some(0.9)).is_err());
    }

    #[test]
    fn distortion_honors_an_immediate_abort() {
        let netlist = Netlist::parse(
            "aborted distortion\n\
             V1 out 0 DISTOF1 1 0\n\
             R1 out 0 1k\n\
             .end\n",
        )
        .expect("abort deck parses");
        let error = Engine::default()
            .run_distortion_with_abort(&netlist, &[1.0e3], None, &ImmediateAbort)
            .expect_err("an immediate abort must stop distortion analysis");
        assert!(matches!(error, SimulationError::Aborted));
    }

    #[test]
    fn linear_circuit_has_zero_harmonic_products() {
        let netlist = Netlist::parse(
            "linear distortion\n\
             V1 in 0 DISTOF1 1 0\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .end\n",
        )
        .expect("linear deck parses");
        let result = Engine::default()
            .run_distortion(&netlist, &[1.0e3], None)
            .expect("linear distortion solve succeeds");
        let point = &result.points[0];
        assert_eq!(point.fundamental_f1.voltages[0], Complex64::new(1.0, 0.0));
        for product in &point.products {
            assert!(product.response.voltages.iter().all(is_zero_complex));
            assert!(product.response.currents.iter().all(is_zero_complex));
        }
    }

    #[test]
    fn diode_harmonics_match_closed_form_taylor_series() {
        const BIAS: Value = 0.5;
        const AMPLITUDE: Value = 1.0e-3;
        const IS: Value = 1.0e-12;
        let netlist = Netlist::parse(
            "diode distortion oracle\n\
             V1 out 0 DC 0.5 DISTOF1 1m 0\n\
             D1 out 0 DM\n\
             .model DM D(IS=1e-12 N=1 CJO=0 TT=0)\n\
             .end\n",
        )
        .expect("diode deck parses");
        let result = Engine::default()
            .run_distortion(&netlist, &[1.0e3], None)
            .expect("diode distortion solve succeeds");
        let point = &result.points[0];
        let second = point
            .product(DistortionProduct::SecondHarmonic)
            .expect("second harmonic")
            .response
            .currents[0]
            .norm();
        let third = point
            .product(DistortionProduct::ThirdHarmonic)
            .expect("third harmonic")
            .response
            .currents[0]
            .norm();

        let vt = crate::constants::thermal_voltage(crate::constants::TEMP_REFERENCE);
        let exponential_current = IS * (BIAS / vt).exp();
        let expected_second = exponential_current * AMPLITUDE.powi(2) / (4.0 * vt.powi(2));
        let expected_third = exponential_current * AMPLITUDE.powi(3) / (24.0 * vt.powi(3));
        let second_relative_error = (second - expected_second).abs() / expected_second;
        let third_relative_error = (third - expected_third).abs() / expected_third;
        assert!(
            second_relative_error < 2.0e-5,
            "HD2 current {second:.12e}, expected {expected_second:.12e}, relerr={second_relative_error:.3e}"
        );
        assert!(
            third_relative_error < 2.0e-3,
            "HD3 current {third:.12e}, expected {expected_third:.12e}, relerr={third_relative_error:.3e}"
        );
    }

    #[test]
    fn diode_two_tone_products_match_closed_form_and_fixed_f2_contract() {
        const BIAS: Value = 0.5;
        const A1: Value = 1.0e-3;
        const A2: Value = 2.0e-3;
        const IS: Value = 1.0e-12;
        let netlist = Netlist::parse(
            "diode intermodulation oracle\n\
             V1 out 0 DC 0.5 DISTOF1 1m 0 DISTOF2 2m 0\n\
             D1 out 0 DM\n\
             .model DM D(IS=1e-12 N=1 CJO=0 TT=0)\n\
             .end\n",
        )
        .expect("two-tone diode deck parses");
        let result = Engine::default()
            .run_distortion(&netlist, &[1.0e3, 2.0e3], Some(0.9))
            .expect("two-tone diode distortion solve succeeds");
        assert!(result.is_two_tone());
        for (point, f1) in result.points.iter().zip([1.0e3, 2.0e3]) {
            let fundamental_f2 = point.fundamental_f2.as_ref().expect("F2 response");
            assert_eq!(fundamental_f2.frequency, 900.0);
            let sum = point.product(DistortionProduct::Sum).expect("sum product");
            let difference = point
                .product(DistortionProduct::Difference)
                .expect("difference product");
            let im3 = point
                .product(DistortionProduct::ThirdOrderDifference)
                .expect("third-order difference product");
            assert_eq!(sum.response.frequency, f1 + 900.0);
            assert_eq!(difference.response.frequency, f1 - 900.0);
            assert_eq!(im3.response.frequency, 2.0 * f1 - 900.0);

            let vt = crate::constants::thermal_voltage(crate::constants::TEMP_REFERENCE);
            let exponential_current = IS * (BIAS / vt).exp();
            let expected_second = exponential_current * A1 * A2 / (2.0 * vt.powi(2));
            let expected_third = exponential_current * A1.powi(2) * A2 / (8.0 * vt.powi(3));
            for actual in [
                sum.response.currents[0].norm(),
                difference.response.currents[0].norm(),
            ] {
                let relative_error = (actual - expected_second).abs() / expected_second;
                assert!(
                    relative_error < 2.0e-5,
                    "IM2 current {actual:.12e}, expected {expected_second:.12e}, relerr={relative_error:.3e}"
                );
            }
            let actual_third = im3.response.currents[0].norm();
            let third_relative_error = (actual_third - expected_third).abs() / expected_third;
            assert!(
                third_relative_error < 2.0e-3,
                "IM3 current {actual_third:.12e}, expected {expected_third:.12e}, relerr={third_relative_error:.3e}"
            );
        }
    }

    #[test]
    fn diode_nonlinear_depletion_charge_is_included_in_volterra_products() {
        const BIAS: Value = 0.2;
        const AMPLITUDE: Value = 1.0e-3;
        const IS: Value = 1.0e-12;
        const CJO: Value = 1.0e-12;
        const VJ: Value = 1.0;
        const M: Value = 0.5;
        const F1: Value = 1.0e6;
        let netlist = Netlist::parse(
            "diode charge distortion oracle\n\
             V1 out 0 DC 0.2 DISTOF1 1m 0\n\
             D1 out 0 DM\n\
             .model DM D(IS=1e-12 N=1 CJO=1p VJ=1 M=0.5 FC=0.5 TT=0)\n\
             .end\n",
        )
        .expect("capacitive diode deck parses");
        let result = Engine::default()
            .run_distortion(&netlist, &[F1], None)
            .expect("capacitive diode distortion solve succeeds");
        let point = &result.points[0];
        let second = point
            .product(DistortionProduct::SecondHarmonic)
            .expect("second harmonic")
            .response
            .currents[0]
            .norm();
        let third = point
            .product(DistortionProduct::ThirdHarmonic)
            .expect("third harmonic")
            .response
            .currents[0]
            .norm();

        let vt = crate::constants::thermal_voltage(crate::constants::TEMP_REFERENCE);
        let exponential_current = IS * (BIAS / vt).exp();
        let current_second = exponential_current * AMPLITUDE.powi(2) / (4.0 * vt.powi(2));
        let current_third = exponential_current * AMPLITUDE.powi(3) / (24.0 * vt.powi(3));
        let charge_second_derivative = CJO * M / VJ * (1.0 - BIAS / VJ).powf(-M - 1.0);
        let charge_third_derivative =
            CJO * M * (M + 1.0) / VJ.powi(2) * (1.0 - BIAS / VJ).powf(-M - 2.0);
        let displacement_second =
            2.0 * PI * (2.0 * F1) * charge_second_derivative * AMPLITUDE.powi(2) / 4.0;
        let displacement_third =
            2.0 * PI * (3.0 * F1) * charge_third_derivative * AMPLITUDE.powi(3) / 24.0;
        let expected_second = current_second.hypot(displacement_second);
        let expected_third = current_third.hypot(displacement_third);
        let second_relative_error = (second - expected_second).abs() / expected_second;
        let third_relative_error = (third - expected_third).abs() / expected_third;
        assert!(
            second_relative_error < 3.0e-5,
            "charged-diode HD2 {second:.12e}, expected {expected_second:.12e}, relerr={second_relative_error:.3e}"
        );
        assert!(
            third_relative_error < 3.0e-3,
            "charged-diode HD3 {third:.12e}, expected {expected_third:.12e}, relerr={third_relative_error:.3e}"
        );
    }

    #[test]
    fn nonlinear_feedback_harmonics_match_implicit_closed_form() {
        const BIAS: Value = 0.4;
        const INPUT_AMPLITUDE: Value = 1.0e-3;
        const IS: Value = 1.0e-12;
        const RESISTANCE: Value = 1.0e3;
        let vt = crate::constants::thermal_voltage(crate::constants::TEMP_REFERENCE);
        let exponential_current = IS * (BIAS / vt).exp();
        let input_bias = BIAS + RESISTANCE * (exponential_current - IS);
        let deck = format!(
            "nonlinear feedback distortion oracle\n\
             V1 in 0 DC {input_bias:.17e} DISTOF1 1m 0\n\
             R1 in out 1k\n\
             D1 out 0 DM\n\
             .model DM D(IS=1e-12 N=1 CJO=0 TT=0)\n\
             .end\n"
        );
        let netlist = Netlist::parse(&deck).expect("feedback diode deck parses");
        let result = Engine::default()
            .run_distortion(&netlist, &[1.0e3], None)
            .expect("feedback diode distortion solve succeeds");
        let point = &result.points[0];
        let output_index = point
            .fundamental_f1
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("output node exists");
        let actual_fundamental = point.fundamental_f1.voltages[output_index].re;
        let actual_second = point
            .product(DistortionProduct::SecondHarmonic)
            .expect("second harmonic")
            .response
            .voltages[output_index]
            .re;
        let actual_third = point
            .product(DistortionProduct::ThirdHarmonic)
            .expect("third harmonic")
            .response
            .voltages[output_index]
            .re;

        let linear = 1.0 / RESISTANCE + exponential_current / vt;
        let expected_fundamental = (INPUT_AMPLITUDE / RESISTANCE) / linear;
        let quadratic = exponential_current / (2.0 * vt.powi(2));
        let cubic = exponential_current / (6.0 * vt.powi(3));
        let expected_second = -quadratic * expected_fundamental.powi(2) / (2.0 * linear);
        let expected_third = -(quadratic * expected_fundamental * expected_second
            + cubic * expected_fundamental.powi(3) / 4.0)
            / linear;

        for (label, actual, expected, tolerance) in [
            ("F1", actual_fundamental, expected_fundamental, 2.0e-6),
            ("2F1", actual_second, expected_second, 3.0e-5),
            ("3F1", actual_third, expected_third, 3.0e-3),
        ] {
            let relative_error = (actual - expected).abs() / expected.abs();
            assert!(
                relative_error < tolerance,
                "{label} voltage {actual:.12e}, expected {expected:.12e}, relerr={relative_error:.3e}"
            );
        }
    }
}

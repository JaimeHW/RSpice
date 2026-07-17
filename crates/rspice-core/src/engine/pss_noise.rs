//! Oscillator phase noise via the perturbation projection vector (PPV).
//!
//! Implements Demir, Mehrotra, Roychowdhury, "Phase Noise in Oscillators: A
//! Unifying Theory and Numerical Methods for Characterisation" (DAC 1998 /
//! IEEE TCAS-I 47(5), 2000). The driven-circuit conversion-matrix noise
//! analysis degenerates at an oscillator's carrier (the matrix turns
//! singular along the neutrally stable oscillation mode); the correct object
//! is the scalar phase diffusion constant, Eq. (16) of the paper:
//!
//! ```text
//! c = (1/T) * integral_0^T  v1^T(t) B(t) B^T(t) v1(t) dt
//! ```
//!
//! where v1(t) is the adjoint Floquet mode of the unity multiplier
//! normalized so v1^T(t) * dx_s/dt = 1 for all t (the paper's Remark 4.3),
//! and B maps the white-noise intensities onto the state equations. The
//! oscillator output spectrum is then a sum of Lorentzians (Eq. (23)); the
//! carrier-normalized single-sideband density around the fundamental is
//!
//! ```text
//! L(f_m) = f0^2 c / (pi^2 f0^4 c^2 + f_m^2)
//! ```
//!
//! whose corner sits at f_c = pi f0^2 c (the paper's Section 10 example) and
//! whose integral preserves the carrier power exactly.

#![allow(clippy::needless_range_loop)]

use super::{Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::analysis::transient::CompanionCoefficients;
use crate::analysis::{IntegrationMethod, NoiseSource, NoiseSourceType, PssConfig};
use crate::{Netlist, Value};
use std::collections::HashMap;

/// Result of oscillator phase-noise analysis.
#[derive(Debug, Clone)]
pub struct OscPnoiseResult {
    /// Offset frequencies from the carrier (Hz).
    pub frequencies: Vec<Value>,
    /// Single-sideband phase noise L(f_m) in dBc/Hz.
    pub phase_noise_dbc: Vec<Value>,
    /// The zero-frequency-equivalent phase diffusion constant c (Demir
    /// Eq. 16 for white sources, including finite-DC colored sources).
    pub diffusion_constant: Value,
    /// Solved oscillation period (s).
    pub period: Value,
    /// Lorentzian corner pi*f0^2*c (Hz); below it L flattens to the finite
    /// carrier-preserving value.
    pub corner_hz: Value,
}

impl Engine {
    /// Compute oscillator single-sideband phase noise at the given carrier
    /// offsets.
    ///
    /// Runs autonomous PSS (period as a shooting unknown), extracts the
    /// adjoint unity Floquet mode along the orbit, projects the complete
    /// device-noise source set through the instantaneous linearized network,
    /// and evaluates Demir's white- and colored-noise coefficients. This
    /// includes disabled/noisy resistor semantics, temperature offsets,
    /// diode and BJT shot noise, MOS/JFET/EKV/BSIM channel and flicker noise,
    /// tabulated and Verilog-A sources, and correlated BSIM4 thermal noise.
    pub fn run_pnoise_oscillator(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        offsets: &[Value],
    ) -> Result<OscPnoiseResult, SimulationError> {
        self.run_pnoise_oscillator_with_abort(netlist, config, offsets, &NoAbort)
    }

    /// Compute oscillator phase noise with cooperative cancellation across
    /// PSS, finite-difference trajectories, adjoint propagation, and source
    /// projection.
    pub fn run_pnoise_oscillator_with_abort(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        offsets: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<OscPnoiseResult, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if offsets.is_empty() {
            return Err(SimulationError::Circuit(
                "oscillator pnoise needs at least one offset frequency".to_string(),
            ));
        }
        self.ensure_analysis_points(offsets.len())?;
        if let Some(offset) = offsets
            .iter()
            .copied()
            .find(|offset| !offset.is_finite() || *offset <= 0.0)
        {
            return Err(SimulationError::Circuit(format!(
                "oscillator pnoise offset frequencies must be finite and strictly positive, got {offset}"
            )));
        }
        if !config.is_autonomous() {
            return Err(SimulationError::Circuit(
                "oscillator pnoise requires an autonomous PSS configuration".to_string(),
            ));
        }

        let (pss, mut circuit, mut matrix, x0) =
            self.run_pss_with_state_abort(netlist, config.clone(), abort)?;
        let period = pss.period;
        let f0 = 1.0 / period;

        // ------------------------------------------------------------------
        // Base trajectory on the fixed grid, with state and node solutions.
        // ------------------------------------------------------------------
        let mut base = super::pss::PssStateTrace::default();
        self.pss_set_reactive_state(&mut circuit, &x0);
        let max_step = period / config.points_per_period as f64;
        let seed = self.pss_initial_node_solution(&mut circuit, &mut matrix, period, abort)?;
        self.pss_run_tran_internal(
            &mut circuit,
            &mut matrix,
            seed,
            period,
            max_step,
            true,
            Some(&mut base),
            config.integration_method,
            abort,
        )?;

        let n_state = x0.len();
        let n_grid = base.times.len();
        if n_grid < 8 || n_state == 0 {
            return Err(SimulationError::Circuit(
                "oscillator pnoise: trajectory too short".to_string(),
            ));
        }
        self.ensure_result_values(
            n_grid
                .saturating_mul(n_state)
                .saturating_mul(n_state)
                .saturating_add(offsets.len().saturating_mul(4)),
        )?;

        // ------------------------------------------------------------------
        // State-transition matrices Phi(t_k, 0) by central differences on
        // the smooth fixed-grid map: one traced pair of integrations per
        // state column.
        // ------------------------------------------------------------------
        let fd_step = 1e-8;
        let mut phi: Vec<Vec<Vec<Value>>> = vec![vec![vec![0.0; n_state]; n_state]; n_grid];
        for k in 0..n_grid {
            for i in 0..n_state {
                phi[k][i][i] = 0.0; // filled below
            }
        }
        for j in 0..n_state {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let h = fd_step * (1.0 + x0[j].abs());

            let mut x_plus = x0.clone();
            x_plus[j] += h;
            let mut tr_plus = super::pss::PssStateTrace::default();
            self.pss_set_reactive_state(&mut circuit, &x_plus);
            let seed_p =
                self.pss_initial_node_solution(&mut circuit, &mut matrix, period, abort)?;
            self.pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed_p,
                period,
                max_step,
                true,
                Some(&mut tr_plus),
                config.integration_method,
                abort,
            )?;

            let mut x_minus = x0.clone();
            x_minus[j] -= h;
            let mut tr_minus = super::pss::PssStateTrace::default();
            self.pss_set_reactive_state(&mut circuit, &x_minus);
            let seed_m =
                self.pss_initial_node_solution(&mut circuit, &mut matrix, period, abort)?;
            self.pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed_m,
                period,
                max_step,
                true,
                Some(&mut tr_minus),
                config.integration_method,
                abort,
            )?;

            for k in 0..n_grid {
                for i in 0..n_state {
                    phi[k][i][j] = (tr_plus.states[k][i] - tr_minus.states[k][i]) / (2.0 * h);
                }
            }
        }

        // ------------------------------------------------------------------
        // Orbit tangent ds/dt on the grid (periodic central differences) and
        // the adjoint unity mode v1(0) from the monodromy Phi(T, 0).
        // ------------------------------------------------------------------
        let dt = period / (n_grid - 1) as Value;
        let tangent = |k: usize| -> Vec<Value> {
            // s(0) == s(T) on the converged orbit, so wrap periodically.
            let prev = if k == 0 { n_grid - 2 } else { k - 1 };
            let next = if k + 1 >= n_grid { 1 } else { k + 1 };
            (0..n_state)
                .map(|i| (base.states[next][i] - base.states[prev][i]) / (2.0 * dt))
                .collect()
        };

        let monodromy = &phi[n_grid - 1];
        let mut v1_0 = vec![1.0; n_state];
        // Inverse iteration on (M^T - (1+eps) I) for the unity left mode.
        for _ in 0..50 {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let mut shifted = vec![vec![0.0; n_state]; n_state];
            for r in 0..n_state {
                for cidx in 0..n_state {
                    shifted[r][cidx] = monodromy[cidx][r]; // transpose
                }
                shifted[r][r] -= 1.0 + 1e-9;
            }
            let next = self.pss_solve_linear_system(&shifted, &v1_0)?;
            let norm: Value = next.iter().map(|v| v * v).sum::<Value>().sqrt();
            if !norm.is_finite() || norm <= 0.0 {
                break;
            }
            v1_0 = next.iter().map(|v| v / norm).collect();
        }
        // Demir Remark 4.3 normalization: v1^T(0) * ds/dt(0) = 1.
        let t0 = tangent(0);
        let proj: Value = v1_0.iter().zip(&t0).map(|(a, b)| a * b).sum();
        if proj.abs() < 1e-30 {
            return Err(SimulationError::Circuit(
                "oscillator pnoise: degenerate phase normalization (orbit tangent \
                 orthogonal to the unity adjoint mode)"
                    .to_string(),
            ));
        }
        let v1_0: Vec<Value> = v1_0.iter().map(|v| v / proj).collect();

        // v1(t_k): adjoint propagation via Phi(t_k,0)^T v1(t_k) = v1(0),
        // renormalized against the local tangent (exact in exact arithmetic;
        // numerically this absorbs grid-level drift).
        let mut v1: Vec<Vec<Value>> = Vec::with_capacity(n_grid);
        for k in 0..n_grid {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let v1_k = if k == 0 {
                v1_0.clone()
            } else {
                let mut phit = vec![vec![0.0; n_state]; n_state];
                for r in 0..n_state {
                    for cidx in 0..n_state {
                        phit[r][cidx] = phi[k][cidx][r];
                    }
                }
                // pss_solve_linear_system solves A x = -b, so negate.
                let neg: Vec<Value> = v1_0.iter().map(|v| -v).collect();
                self.pss_solve_linear_system(&phit, &neg)?
            };
            let tk = tangent(k);
            let p: Value = v1_k.iter().zip(&tk).map(|(a, b)| a * b).sum();
            if p.abs() < 1e-30 {
                return Err(SimulationError::Circuit(
                    "oscillator pnoise: adjoint mode lost tangent normalization".to_string(),
                ));
            }
            v1.push(v1_k.iter().map(|v| v / p).collect());
        }

        // ------------------------------------------------------------------
        // Noise projection: per grid point, solve the instantaneously frozen
        // linearized network for each source's unit current injection; the
        // state-equation entries are dv_cap/dt_freeze and di_branch/dt_freeze.
        // ------------------------------------------------------------------
        let dt_freeze = period * 1e-9;
        let coeff = CompanionCoefficients::for_method(IntegrationMethod::BackwardEuler);
        let temperature = self.config.temperature;
        let mut evaluation_frequencies = Vec::with_capacity(offsets.len() + 1);
        evaluation_frequencies.push(0.0);
        evaluation_frequencies.extend_from_slice(offsets);

        let n_caps = circuit.capacitors.len();
        let size = circuit.matrix_size();
        let mut white_integrals = vec![0.0; evaluation_frequencies.len()];
        let mut previous_white = vec![0.0; evaluation_frequencies.len()];
        let mut colored_integrals: HashMap<PssNoiseKey, Vec<Value>> = HashMap::new();
        let mut previous_colored: HashMap<PssNoiseKey, Vec<Value>> = HashMap::new();
        let mut found_noise_source = false;
        for k in 0..n_grid {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // Restore the traced reactive state and linearize at the traced
            // node solution: pss_stamp_system reads cap/inductor history.
            self.pss_set_reactive_state(&mut circuit, &base.states[k]);
            let solution = base.solutions[k].clone();
            let mut rhs_scratch = vec![0.0; size];
            self.pss_stamp_system(
                &mut circuit,
                &mut matrix,
                &mut rhs_scratch,
                &coeff,
                base.times[k] + dt_freeze,
                dt_freeze,
                &solution,
            )?;

            let mut projection = |node_pos: usize,
                                  node_neg: usize|
             -> Result<Value, SimulationError> {
                if node_pos > size || node_neg > size {
                    return Err(SimulationError::Circuit(format!(
                        "oscillator pnoise source injection ({node_pos}, {node_neg}) exceeds matrix size {size}"
                    )));
                }
                let mut injection = vec![0.0; size];
                if node_pos > 0 {
                    injection[node_pos - 1] += 1.0;
                }
                if node_neg > 0 {
                    injection[node_neg - 1] -= 1.0;
                }
                let delta = matrix.solve(&injection).map_err(SimulationError::Solver)?;

                // b entries: capacitor states first, then inductor branch
                // states. The frozen companion step maps a unit source into
                // the state derivative used by the PPV projection.
                let mut value = 0.0;
                for (alpha, cap) in circuit.capacitors.stamps.iter().enumerate() {
                    let np = cap.pp.row;
                    let nn = cap.nn.row;
                    let dv = if np == 0 { 0.0 } else { delta[np - 1] }
                        - if nn == 0 { 0.0 } else { delta[nn - 1] };
                    value += v1[k][alpha] * dv / dt_freeze;
                }
                for l_idx in 0..circuit.inductors.names.len() {
                    let branch = circuit.inductors.branch_indices[l_idx];
                    if branch > 0 {
                        let branch_index = circuit.num_nodes() + branch - 1;
                        value += v1[k][n_caps + l_idx] * delta[branch_index] / dt_freeze;
                    }
                }
                Ok(value)
            };

            let (sources, correlated_sources) =
                Self::try_collect_noise_sources(&circuit, &solution)?;
            found_noise_source |= !sources.is_empty() || !correlated_sources.is_empty();
            let mut white = vec![0.0; evaluation_frequencies.len()];
            let mut colored: HashMap<PssNoiseKey, Vec<Value>> = HashMap::new();
            let mut colored_occurrences: HashMap<PssNoiseIdentity, usize> = HashMap::new();
            for source in &sources {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let projected = projection(source.node_pos, source.node_neg)?;
                if pss_noise_is_colored(source.noise_type) {
                    let amplitudes = evaluation_frequencies
                        .iter()
                        .map(|&frequency| {
                            let density = source.spectral_density(frequency, temperature);
                            if density.is_finite() && density > 0.0 {
                                projected * density.sqrt()
                            } else {
                                0.0
                            }
                        })
                        .collect();
                    let identity = PssNoiseIdentity::from(source);
                    let occurrence = colored_occurrences.entry(identity.clone()).or_default();
                    let key = PssNoiseKey {
                        identity,
                        occurrence: *occurrence,
                    };
                    *occurrence += 1;
                    colored.insert(key, amplitudes);
                } else {
                    let density = source.spectral_density(0.0, temperature);
                    if density.is_finite() && density > 0.0 {
                        let contribution = density * projected * projected;
                        for value in &mut white {
                            *value += contribution;
                        }
                    }
                }
            }

            // BSIM4 tnoiMod=2 is a single correlated thermal mechanism at
            // two current-injection ports. Preserve its relative phase before
            // taking the squared magnitude; treating the ports independently
            // would double count or discard their covariance.
            for source in &correlated_sources {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let first = projection(source.first.node_pos, source.first.node_neg)?;
                let second = projection(source.second.node_pos, source.second.node_neg)?;
                for (index, &frequency) in evaluation_frequencies.iter().enumerate() {
                    let Some(densities) = source.spectral_densities(frequency, temperature) else {
                        continue;
                    };
                    if !densities.first_psd.is_finite()
                        || !densities.second_psd.is_finite()
                        || densities.first_psd < 0.0
                        || densities.second_psd < 0.0
                    {
                        continue;
                    }
                    let first_amplitude = first * densities.first_psd.sqrt();
                    let second_amplitude = second * densities.second_psd.sqrt();
                    white[index] += first_amplitude * first_amplitude
                        + second_amplitude * second_amplitude
                        + 2.0 * first_amplitude * second_amplitude * densities.phase_rad.cos();
                }
            }

            if k > 0 {
                for index in 0..white_integrals.len() {
                    white_integrals[index] += 0.5 * (white[index] + previous_white[index]) * dt;
                }

                for (key, amplitudes) in &colored {
                    let previous = previous_colored.get(key);
                    let integral = colored_integrals
                        .entry(key.clone())
                        .or_insert_with(|| vec![0.0; evaluation_frequencies.len()]);
                    for index in 0..integral.len() {
                        integral[index] += 0.5
                            * (amplitudes[index] + previous.map_or(0.0, |values| values[index]))
                            * dt;
                    }
                }
                for (key, amplitudes) in &previous_colored {
                    if colored.contains_key(key) {
                        continue;
                    }
                    let integral = colored_integrals
                        .entry(key.clone())
                        .or_insert_with(|| vec![0.0; evaluation_frequencies.len()]);
                    for index in 0..integral.len() {
                        integral[index] += 0.5 * amplitudes[index] * dt;
                    }
                }
            }
            previous_white = white;
            previous_colored = colored;
        }
        if !found_noise_source {
            return Err(SimulationError::Circuit(
                "oscillator pnoise: no modeled noise sources in the circuit".to_string(),
            ));
        }

        let mut coefficients: Vec<Value> = white_integrals
            .into_iter()
            .map(|integral| integral / period)
            .collect();
        for integrals in colored_integrals.values() {
            for (coefficient, integral) in coefficients.iter_mut().zip(integrals) {
                // Demir's colored-noise coefficient is |V0|^2, where V0 is
                // the period-average signed PPV/source-amplitude projection;
                // it is not the average of the squared projection used for
                // white noise (ICCAD 1998, Eqs. 89-93).
                *coefficient += (integral / period).powi(2);
            }
        }
        let c = coefficients[0];
        if !c.is_finite() || c < 0.0 {
            return Err(SimulationError::Circuit(
                "oscillator pnoise produced a non-finite diffusion coefficient".to_string(),
            ));
        }

        let corner_hz = std::f64::consts::PI * f0 * f0 * c;
        let phase_noise_dbc: Vec<Value> = offsets
            .iter()
            .zip(coefficients.iter().skip(1))
            .map(|(&fm, &coefficient)| {
                // This smooth form is exact for the white-noise Lorentzian
                // and has Demir's colored-noise limiting forms: the finite-DC
                // coefficient controls the carrier linewidth, while away
                // from the carrier L(fm) -> f0^2*c(fm)/fm^2.
                let l = f0 * f0 * coefficient / (corner_hz * corner_hz + fm * fm);
                if l > 0.0 {
                    10.0 * l.log10()
                } else {
                    Value::NEG_INFINITY
                }
            })
            .collect();

        Ok(OscPnoiseResult {
            frequencies: offsets.to_vec(),
            phase_noise_dbc,
            diffusion_constant: c,
            period,
            corner_hz,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PssNoiseKey {
    identity: PssNoiseIdentity,
    occurrence: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PssNoiseIdentity {
    device_name: String,
    noise_type: NoiseSourceType,
    node_pos: usize,
    node_neg: usize,
}

impl From<&NoiseSource> for PssNoiseIdentity {
    fn from(source: &NoiseSource) -> Self {
        Self {
            device_name: source.identity.device.clone(),
            noise_type: source.noise_type,
            node_pos: source.node_pos,
            node_neg: source.node_neg,
        }
    }
}

fn pss_noise_is_colored(noise_type: NoiseSourceType) -> bool {
    matches!(
        noise_type,
        NoiseSourceType::Flicker
            | NoiseSourceType::Burst
            | NoiseSourceType::Table
            | NoiseSourceType::Bsim4Flicker
            | NoiseSourceType::Bsim3Flicker
    )
}

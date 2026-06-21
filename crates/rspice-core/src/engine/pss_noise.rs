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
use crate::analysis::transient::CompanionCoefficients;
use crate::analysis::{IntegrationMethod, PssConfig};
use crate::circuit::Circuit;
use crate::{Netlist, Value};

/// Result of oscillator phase-noise analysis.
#[derive(Debug, Clone)]
pub struct OscPnoiseResult {
    /// Offset frequencies from the carrier (Hz).
    pub frequencies: Vec<Value>,
    /// Single-sideband phase noise L(f_m) in dBc/Hz.
    pub phase_noise_dbc: Vec<Value>,
    /// The scalar phase diffusion constant c (Demir Eq. 16).
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
    /// adjoint unity Floquet mode along the orbit, projects every white
    /// noise source onto the state equations through the linearized
    /// instantaneous network, and evaluates Demir's diffusion constant.
    /// Modeled sources: resistor thermal 4kT/R (stationary) and diode shot
    /// 2q|Id(t)| (modulated along the orbit). FET channel noise and flicker
    /// are not yet projected; circuits whose noise is dominated by those
    /// sources will read optimistic.
    pub fn run_pnoise_oscillator(
        &self,
        netlist: &Netlist,
        config: PssConfig,
        offsets: &[Value],
    ) -> Result<OscPnoiseResult, SimulationError> {
        if offsets.is_empty() {
            return Err(SimulationError::Circuit(
                "oscillator pnoise needs at least one offset frequency".to_string(),
            ));
        }
        if !config.is_autonomous() {
            return Err(SimulationError::Circuit(
                "oscillator pnoise requires an autonomous PSS configuration".to_string(),
            ));
        }

        let (pss, mut circuit, mut matrix, x0) =
            self.run_pss_with_state(netlist, config.clone())?;
        let period = pss.period;
        let f0 = 1.0 / period;

        // ------------------------------------------------------------------
        // Base trajectory on the fixed grid, with state and node solutions.
        // ------------------------------------------------------------------
        let mut base = super::pss::PssStateTrace::default();
        self.pss_set_reactive_state(&mut circuit, &x0);
        let max_step = period / config.points_per_period as f64;
        let seed = self.pss_initial_node_solution(&mut circuit, &mut matrix, period)?;
        self.pss_run_tran_internal(
            &mut circuit,
            &mut matrix,
            seed,
            period,
            max_step,
            true,
            Some(&mut base),
        )?;

        let n_state = x0.len();
        let n_grid = base.times.len();
        if n_grid < 8 || n_state == 0 {
            return Err(SimulationError::Circuit(
                "oscillator pnoise: trajectory too short".to_string(),
            ));
        }

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
            let h = fd_step * (1.0 + x0[j].abs());

            let mut x_plus = x0.clone();
            x_plus[j] += h;
            let mut tr_plus = super::pss::PssStateTrace::default();
            self.pss_set_reactive_state(&mut circuit, &x_plus);
            let seed_p = self.pss_initial_node_solution(&mut circuit, &mut matrix, period)?;
            self.pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed_p,
                period,
                max_step,
                true,
                Some(&mut tr_plus),
            )?;

            let mut x_minus = x0.clone();
            x_minus[j] -= h;
            let mut tr_minus = super::pss::PssStateTrace::default();
            self.pss_set_reactive_state(&mut circuit, &x_minus);
            let seed_m = self.pss_initial_node_solution(&mut circuit, &mut matrix, period)?;
            self.pss_run_tran_internal(
                &mut circuit,
                &mut matrix,
                seed_m,
                period,
                max_step,
                true,
                Some(&mut tr_minus),
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
        let sources = Self::pss_noise_source_list(&circuit);
        if sources.is_empty() {
            return Err(SimulationError::Circuit(
                "oscillator pnoise: no modeled noise sources in the circuit".to_string(),
            ));
        }
        let temperature = self.config.temperature;

        let n_caps = circuit.capacitors.len();
        let size = circuit.matrix_size();
        let mut c_integral = 0.0;
        let mut prev_integrand = 0.0;
        for k in 0..n_grid {
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
            );

            let mut integrand = 0.0;
            for source in &sources {
                let intensity = source.intensity(&circuit, &solution, temperature);
                if !intensity.is_finite() || intensity <= 0.0 {
                    continue;
                }
                let mut inj = vec![0.0; size];
                if source.node_pos > 0 {
                    inj[source.node_pos - 1] += 1.0;
                }
                if source.node_neg > 0 {
                    inj[source.node_neg - 1] -= 1.0;
                }
                let delta = match matrix.solve(&inj) {
                    Ok(d) => d,
                    Err(e) => return Err(SimulationError::Solver(e)),
                };

                // b entries: cap states first, then inductor branch states.
                let mut proj = 0.0;
                for (alpha, cap) in circuit.capacitors.stamps.iter().enumerate() {
                    let np = cap.pp.row;
                    let nn = cap.nn.row;
                    let dv = if np == 0 { 0.0 } else { delta[np - 1] }
                        - if nn == 0 { 0.0 } else { delta[nn - 1] };
                    proj += v1[k][alpha] * dv / dt_freeze;
                }
                for l_idx in 0..circuit.inductors.names.len() {
                    let br = circuit.inductors.branch_indices[l_idx];
                    if br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        proj += v1[k][n_caps + l_idx] * delta[br_idx] / dt_freeze;
                    }
                }
                integrand += intensity * proj * proj;
            }

            if k > 0 {
                c_integral += 0.5 * (integrand + prev_integrand) * dt;
            }
            prev_integrand = integrand;
        }
        let c = c_integral / period;

        let corner_hz = std::f64::consts::PI * f0 * f0 * c;
        let phase_noise_dbc: Vec<Value> = offsets
            .iter()
            .map(|&fm| {
                let l = f0 * f0 * c / (corner_hz * corner_hz + fm * fm);
                10.0 * l.max(1e-300).log10()
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

    /// Enumerate the time-domain noise sources the PPV projection models:
    /// resistor thermal (stationary 4kT G) and diode shot (2q|Id(v(t))|,
    /// modulated along the orbit).
    fn pss_noise_source_list(circuit: &Circuit) -> Vec<PssNoiseSource> {
        let mut sources = Vec::new();
        for i in 0..circuit.resistors.len() {
            let g = circuit.resistors.conductances[i];
            if g.is_finite() && g > 0.0 {
                sources.push(PssNoiseSource {
                    node_pos: circuit.resistors.stamps[i].pp.row,
                    node_neg: circuit.resistors.stamps[i].nn.row,
                    kind: PssNoiseKind::Thermal { g },
                });
            }
        }
        for diode in &circuit.diodes.devices {
            sources.push(PssNoiseSource {
                node_pos: diode.node_anode,
                node_neg: diode.node_cathode,
                kind: PssNoiseKind::DiodeShot {
                    is: diode.is,
                    n_vt: diode.n * diode.vt,
                },
            });
        }
        sources
    }
}

/// One projected white-noise source in the time-domain (PSS) circuit.
struct PssNoiseSource {
    node_pos: usize,
    node_neg: usize,
    kind: PssNoiseKind,
}

enum PssNoiseKind {
    Thermal { g: Value },
    DiodeShot { is: Value, n_vt: Value },
}

impl PssNoiseSource {
    /// Instantaneous intensity in A^2/Hz at the given node solution.
    fn intensity(&self, _circuit: &Circuit, solution: &[Value], temperature: Value) -> Value {
        const K_B: Value = 1.380649e-23;
        const Q_E: Value = 1.602176634e-19;
        match self.kind {
            PssNoiseKind::Thermal { g } => 4.0 * K_B * temperature * g,
            PssNoiseKind::DiodeShot { is, n_vt } => {
                let vp = if self.node_pos == 0 {
                    0.0
                } else {
                    solution.get(self.node_pos - 1).copied().unwrap_or(0.0)
                };
                let vn = if self.node_neg == 0 {
                    0.0
                } else {
                    solution.get(self.node_neg - 1).copied().unwrap_or(0.0)
                };
                let vd = vp - vn;
                let arg = (vd / n_vt).min(40.0);
                let id = is * (arg.exp() - 1.0);
                2.0 * Q_E * id.abs()
            }
        }
    }
}

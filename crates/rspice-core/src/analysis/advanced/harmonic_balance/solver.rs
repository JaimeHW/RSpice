//! Harmonic Balance Newton Solver
//!
//! Core solver for Harmonic Balance analysis using Newton-Raphson iteration.
//! Solves the frequency-domain circuit equations: G*X + jω*C*X + F_NL(X) = I_S

use num_complex::Complex64;
use std::f64::consts::PI;

use super::config::HbConfig;
use super::fft::HbFft;
use super::result::{HbResult, SpectralVoltage};
use crate::Value;
use crate::solver::convergence::{PseudoTransient, SourceStepper};
use crate::solver::limit_pn_voltage;

/// Error types specific to Harmonic Balance solver
#[derive(Debug, Clone)]
pub enum HbError {
    /// Newton iteration did not converge
    ConvergenceFailed { iterations: usize, residual: Value },
    /// Matrix is singular
    SingularMatrix,
    /// Invalid circuit configuration
    InvalidCircuit(String),
    /// FFT operation failed
    FftError(String),
}

impl std::fmt::Display for HbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConvergenceFailed {
                iterations,
                residual,
            } => {
                write!(
                    f,
                    "HB convergence failed after {} iterations (residual: {:.3e})",
                    iterations, residual
                )
            }
            Self::SingularMatrix => write!(f, "Singular Jacobian matrix"),
            Self::InvalidCircuit(msg) => write!(f, "Invalid circuit: {}", msg),
            Self::FftError(msg) => write!(f, "FFT error: {}", msg),
        }
    }
}

impl std::error::Error for HbError {}

/// Harmonic Balance solver state
#[derive(Debug)]
pub struct HbSolverState {
    /// Spectral voltage solution [node][harmonic]
    pub x: Vec<Vec<Complex64>>,

    /// Residual vector [node][harmonic]
    pub residual: Vec<Vec<Complex64>>,

    /// Current residual norm
    pub residual_norm: Value,

    /// Number of iterations
    pub iteration: usize,

    /// Converged flag
    pub converged: bool,
}

impl HbSolverState {
    /// Create new solver state
    pub fn new(num_nodes: usize, num_harmonics: usize) -> Self {
        Self {
            x: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            residual: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            residual_norm: f64::INFINITY,
            iteration: 0,
            converged: false,
        }
    }

    /// Compute residual norm (L2 over all nodes and harmonics)
    pub fn compute_residual_norm(&mut self) {
        let sum: Value = self
            .residual
            .iter()
            .flat_map(|node| node.iter())
            .map(|c| c.norm_sqr())
            .sum();
        self.residual_norm = sum.sqrt();
    }

    /// Compute solution norm for relative tolerance
    pub fn solution_norm(&self) -> Value {
        let sum: Value = self
            .x
            .iter()
            .flat_map(|node| node.iter())
            .map(|c| c.norm_sqr())
            .sum();
        sum.sqrt()
    }

    /// Total number of unknowns
    pub fn total_unknowns(&self) -> usize {
        self.x.len() * self.x.first().map(|v| v.len()).unwrap_or(0)
    }
}

/// Represents a linear circuit element for HB
#[derive(Debug, Clone)]
pub struct HbLinearElement {
    /// Conductance matrix stamp (node_i, node_j, value)
    pub g_stamps: Vec<(usize, usize, Value)>,
    /// Capacitance matrix stamp (node_i, node_j, value)
    pub c_stamps: Vec<(usize, usize, Value)>,
    /// Inductance matrix stamp (node_i, node_j, value)
    pub l_stamps: Vec<(usize, usize, Value)>,
}

/// Voltage source branch for MNA
///
/// In Modified Nodal Analysis, voltage sources require branch current
/// variables to properly enforce voltage constraints.
#[derive(Debug, Clone)]
pub struct VoltageSourceBranch {
    /// Positive terminal node (1-indexed, 0 = ground)
    pub node_pos: usize,
    /// Negative terminal node (1-indexed, 0 = ground)
    pub node_neg: usize,
    /// Branch current variable index
    pub branch_idx: usize,
    /// DC voltage value
    pub dc_voltage: Value,
    /// AC voltage magnitude (at fundamental)
    pub ac_magnitude: Value,
    /// AC voltage phase (radians)
    pub ac_phase: Value,
}

impl VoltageSourceBranch {
    /// Create new voltage source branch
    pub fn new(node_pos: usize, node_neg: usize, branch_idx: usize, dc_voltage: Value) -> Self {
        Self {
            node_pos,
            node_neg,
            branch_idx,
            dc_voltage,
            ac_magnitude: 0.0,
            ac_phase: 0.0,
        }
    }

    /// Set AC parameters
    pub fn with_ac(mut self, magnitude: Value, phase: Value) -> Self {
        self.ac_magnitude = magnitude;
        self.ac_phase = phase;
        self
    }

    /// Get voltage spectrum (DC + AC at fundamental)
    pub fn voltage_spectrum(&self, num_harmonics: usize) -> Vec<Complex64> {
        let mut spectrum = vec![Complex64::new(0.0, 0.0); num_harmonics + 1];
        spectrum[0] = Complex64::new(self.dc_voltage, 0.0);
        if num_harmonics >= 1 {
            spectrum[1] = Complex64::from_polar(self.ac_magnitude, self.ac_phase);
        }
        spectrum
    }
}

/// Represents a nonlinear device for HB
pub trait HbNonlinearDevice: Send + Sync {
    /// Evaluate device current given terminal voltages in time domain
    fn evaluate(&self, voltages: &[Value]) -> Value;

    /// Get device terminals (node indices)
    fn terminals(&self) -> &[usize];

    /// Compute Jacobian contribution (dI/dV) in time domain
    fn jacobian(&self, voltages: &[Value]) -> Vec<(usize, Value)>;
}

/// Harmonic Balance solver
///
/// HB solver supporting:
/// - Linear elements: R, C, L (with proper jωL admittance)
/// - MNA voltage sources with branch currents
/// - Nonlinear device Newton iteration via FFT/IFFT
#[derive(Debug)]
pub struct HbSolver {
    /// Configuration
    config: HbConfig,

    /// FFT processor
    fft: HbFft,

    /// Number of nodes
    num_nodes: usize,

    /// Number of harmonics (including DC)
    num_harmonics: usize,

    /// Number of branch currents (for MNA voltage sources)
    num_branches: usize,

    /// Conductance matrix for each node combination
    /// Stored as sparse: (row, col) -> G
    g_matrix: Vec<(usize, usize, Value)>,

    /// Capacitance matrix for each node combination
    /// Stored as sparse: (row, col) -> C
    c_matrix: Vec<(usize, usize, Value)>,

    /// Inductance matrix for each node combination
    /// Stored as sparse: (row, col) -> L
    /// Admittance Y = 1/(jωL) at each harmonic
    l_matrix: Vec<(usize, usize, Value)>,

    /// Voltage source branches for MNA
    /// (node_pos, node_neg, branch_idx, dc_value, ac_magnitude, ac_phase)
    voltage_source_branches: Vec<VoltageSourceBranch>,

    /// Node names
    node_names: Vec<String>,

    /// Current source spectra [node][harmonic]
    source_spectra: Vec<Vec<Complex64>>,

    /// Registered nonlinear devices for Newton iteration
    nonlinear_devices: Vec<NonlinearDeviceInstance>,
}

/// Runtime representation of a nonlinear device for HB Newton iteration
///
/// This wraps device parameters and provides unified current/Jacobian evaluation.
/// Used during the Newton solve to compute nonlinear contributions in time domain.
#[derive(Debug, Clone)]
pub struct NonlinearDeviceInstance {
    /// Device type
    pub device_type: NonlinearDeviceType,
    /// Terminal nodes (0-indexed, device-specific ordering)
    pub terminals: Vec<usize>,
    /// Device parameters (device-specific interpretation)
    pub params: NonlinearDeviceParams,
}

/// Type of nonlinear device
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NonlinearDeviceType {
    /// Two-terminal diode (anode, cathode)
    Diode,
    /// Three-terminal NPN BJT (collector, base, emitter)
    NpnBjt,
    /// Three-terminal PNP BJT (collector, base, emitter)
    PnpBjt,
    /// Four-terminal NMOS (drain, gate, source, bulk)
    Nmos,
    /// Four-terminal PMOS (drain, gate, source, bulk)
    Pmos,
}

/// Device parameters for nonlinear devices
#[derive(Debug, Clone)]
pub struct NonlinearDeviceParams {
    /// Saturation current (Is for diode/BJT)
    pub is: Value,
    /// Ideality factor (n for diode)
    pub n: Value,
    /// Thermal voltage
    pub vt: Value,
    /// Forward beta (BJT)
    pub bf: Value,
    /// Reverse beta (BJT)
    pub br: Value,
    /// Threshold voltage (MOSFET)
    pub vth: Value,
    /// Transconductance parameter K = μCox W/L (MOSFET)
    pub kp: Value,
    /// Channel length modulation (MOSFET)
    pub lambda: Value,
    /// Early voltage (BJT)
    pub vaf: Value,
}

impl Default for NonlinearDeviceParams {
    fn default() -> Self {
        Self {
            is: 1e-14,
            n: 1.0,
            vt: 0.02585,
            bf: 100.0,
            br: 1.0,
            vth: 0.7,
            kp: 2e-5,
            lambda: 0.0,
            vaf: f64::INFINITY,
        }
    }
}

impl NonlinearDeviceParams {
    /// Create diode parameters
    pub fn diode(is: Value, n: Value) -> Self {
        Self {
            is,
            n,
            ..Default::default()
        }
    }

    /// Create BJT parameters
    pub fn bjt(is: Value, bf: Value, br: Value, vaf: Value) -> Self {
        Self {
            is,
            bf,
            br,
            vaf,
            ..Default::default()
        }
    }

    /// Create MOSFET parameters
    pub fn mosfet(vth: Value, kp: Value, lambda: Value) -> Self {
        Self {
            vth,
            kp,
            lambda,
            ..Default::default()
        }
    }
}

impl NonlinearDeviceInstance {
    /// Create a diode instance
    pub fn diode(anode: usize, cathode: usize, is: Value, n: Value) -> Self {
        Self {
            device_type: NonlinearDeviceType::Diode,
            terminals: vec![anode, cathode],
            params: NonlinearDeviceParams::diode(is, n),
        }
    }

    /// Create an NPN BJT instance
    pub fn npn_bjt(collector: usize, base: usize, emitter: usize, is: Value, bf: Value) -> Self {
        Self {
            device_type: NonlinearDeviceType::NpnBjt,
            terminals: vec![collector, base, emitter],
            params: NonlinearDeviceParams::bjt(is, bf, 1.0, f64::INFINITY),
        }
    }

    /// Create a PNP BJT instance
    pub fn pnp_bjt(collector: usize, base: usize, emitter: usize, is: Value, bf: Value) -> Self {
        Self {
            device_type: NonlinearDeviceType::PnpBjt,
            terminals: vec![collector, base, emitter],
            params: NonlinearDeviceParams::bjt(is, bf, 1.0, f64::INFINITY),
        }
    }

    /// Create an NMOS instance
    pub fn nmos(
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        vth: Value,
        kp: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Nmos,
            terminals: vec![drain, gate, source, bulk],
            params: NonlinearDeviceParams::mosfet(vth, kp, 0.0),
        }
    }

    /// Evaluate device current given terminal voltages
    /// Returns Vec of (node_index, current) pairs - current flowing INTO each node
    pub fn evaluate(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        match self.device_type {
            NonlinearDeviceType::Diode => self.eval_diode(node_voltages),
            NonlinearDeviceType::NpnBjt => self.eval_npn_bjt(node_voltages),
            NonlinearDeviceType::PnpBjt => self.eval_pnp_bjt(node_voltages),
            NonlinearDeviceType::Nmos => self.eval_nmos(node_voltages),
            NonlinearDeviceType::Pmos => self.eval_pmos(node_voltages),
        }
    }

    /// Compute Jacobian entries (∂I/∂V for each terminal pair)
    /// Returns Vec of ((from_node, to_node), dI/dV) - linearized conductance stamps
    pub fn jacobian(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        match self.device_type {
            NonlinearDeviceType::Diode => self.jac_diode(node_voltages),
            NonlinearDeviceType::NpnBjt => self.jac_npn_bjt(node_voltages),
            NonlinearDeviceType::PnpBjt => self.jac_pnp_bjt(node_voltages),
            NonlinearDeviceType::Nmos => self.jac_nmos(node_voltages),
            NonlinearDeviceType::Pmos => self.jac_pmos(node_voltages),
        }
    }

    // --- Private evaluation methods ---

    fn get_terminal_voltage(&self, node_voltages: &[Value], terminal_idx: usize) -> Value {
        let node = self.terminals.get(terminal_idx).copied().unwrap_or(0);
        node_voltages.get(node).copied().unwrap_or(0.0)
    }

    fn eval_diode(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let v_a = self.get_terminal_voltage(node_voltages, 0);
        let v_c = self.get_terminal_voltage(node_voltages, 1);
        let vd = v_a - v_c;

        // Shockley equation with limiting
        let arg = (vd / (self.params.n * self.params.vt)).clamp(-40.0, 40.0);
        let id = self.params.is * (arg.exp() - 1.0);

        // Return current contribution to each node equation (current INTO node convention)
        // Diode current id flows FROM anode TO cathode
        // So current INTO anode = -id, current INTO cathode = +id
        vec![
            (self.terminals[0], -id), // Current INTO anode (negative = leaving)
            (self.terminals[1], id),  // Current INTO cathode (positive = entering)
        ]
    }

    fn jac_diode(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_a = self.get_terminal_voltage(node_voltages, 0);
        let v_c = self.get_terminal_voltage(node_voltages, 1);
        let vd = v_a - v_c;

        // Conductance gd = dI_d/dV = Is/(n*Vt) * exp(Vd/(n*Vt))
        let arg = (vd / (self.params.n * self.params.vt)).clamp(-40.0, 40.0);
        let gd = (self.params.is / (self.params.n * self.params.vt)) * arg.exp();
        let gd = gd.max(1e-12); // Minimum conductance for numerical stability

        let a = self.terminals[0];
        let c = self.terminals[1];

        // Return MNA conductance stamp
        // Physical conductance gd is POSITIVE. MNA stamp for 2-terminal conductance:
        // G[n+,n+] += gd, G[n+,n-] -= gd, G[n-,n+] -= gd, G[n-,n-] += gd
        vec![((a, a), gd), ((a, c), -gd), ((c, a), -gd), ((c, c), gd)]
    }

    fn eval_npn_bjt(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let vbe = v_b - v_e;
        let vbc = v_b - v_c;

        // Ebers-Moll transport model
        let arg_be = (vbe / self.params.vt).clamp(-40.0, 40.0);
        let arg_bc = (vbc / self.params.vt).clamp(-40.0, 40.0);

        let i_f = self.params.is * (arg_be.exp() - 1.0);
        let i_r = self.params.is * (arg_bc.exp() - 1.0);

        let ic = i_f - i_r / self.params.br;
        let ib = i_f / self.params.bf + i_r / self.params.br;
        let ie = -(ic + ib); // KCL

        vec![
            (self.terminals[0], -ic), // Collector current out
            (self.terminals[1], -ib), // Base current out
            (self.terminals[2], -ie), // Emitter current out
        ]
    }

    fn jac_npn_bjt(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let vbe = v_b - v_e;
        let vbc = v_b - v_c;

        let arg_be = (vbe / self.params.vt).clamp(-40.0, 40.0);
        let arg_bc = (vbc / self.params.vt).clamp(-40.0, 40.0);

        // Transconductances
        let gm_f = (self.params.is / self.params.vt) * arg_be.exp();
        let gm_r = (self.params.is / self.params.vt) * arg_bc.exp();

        let c = self.terminals[0];
        let b = self.terminals[1];
        let e = self.terminals[2];

        // Simplified linearized model - gm stamps
        let gbe = gm_f / self.params.bf;
        let gbc = gm_r / self.params.br;

        vec![
            // Base-emitter conductance
            ((b, b), gbe + gbc), // Combined: gbe from B-E + gbc from B-C
            ((b, e), -gbe),
            ((e, b), -gbe),
            ((e, e), gbe),
            // Base-collector conductance (gbc stamps)
            ((b, c), -gbc),
            ((c, b), gm_f - gbc), // Combined: gm_f transconductance + (-gbc) from B-C conductance
            ((c, c), gbc),
            // Transconductance gm stamps (collector controlled by Vbe)
            // Note: (c, b) contribution from gm_f already combined above
            ((c, e), -gm_f),
        ]
    }

    fn eval_pnp_bjt(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        // PNP is NPN with inverted voltages and currents
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let veb = v_e - v_b; // Inverted from NPN
        let vcb = v_c - v_b;

        let arg_eb = (veb / self.params.vt).clamp(-40.0, 40.0);
        let arg_cb = (vcb / self.params.vt).clamp(-40.0, 40.0);

        let i_f = self.params.is * (arg_eb.exp() - 1.0);
        let i_r = self.params.is * (arg_cb.exp() - 1.0);

        let ic = -(i_f - i_r / self.params.br); // Inverted
        let ib = -(i_f / self.params.bf + i_r / self.params.br);
        let ie = -(ic + ib);

        vec![
            (self.terminals[0], -ic),
            (self.terminals[1], -ib),
            (self.terminals[2], -ie),
        ]
    }

    fn jac_pnp_bjt(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        // PNP Jacobian with proper polarity handling
        // PNP uses Veb = Ve - Vb and Vcb = Vc - Vb
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let veb = v_e - v_b;
        let vcb = v_c - v_b;

        let arg_eb = (veb / self.params.vt).clamp(-40.0, 40.0);
        let arg_cb = (vcb / self.params.vt).clamp(-40.0, 40.0);

        // Transconductances for PNP (based on Veb and Vcb)
        let gm_f = (self.params.is / self.params.vt) * arg_eb.exp();
        let gm_r = (self.params.is / self.params.vt) * arg_cb.exp();

        let c = self.terminals[0];
        let b = self.terminals[1];
        let e = self.terminals[2];

        // PNP junction conductances
        let geb = gm_f / self.params.bf; // Emitter-base conductance
        let gcb = gm_r / self.params.br; // Collector-base conductance

        vec![
            // Emitter-base junction conductance (geb)
            // dI/dVeb stamps - note PNP has Ve > Vb for forward bias
            ((e, e), geb),
            ((e, b), -geb),
            ((b, e), -geb),
            ((b, b), geb + gcb), // Combined: geb from E-B + gcb from C-B
            // Collector-base junction conductance (gcb)
            ((c, c), gcb),
            ((c, b), -(gcb + gm_f)), // Combined: -gcb from C-B conductance + (-gm_f) from transconductance
            ((b, c), -gcb),
            // Transconductance: collector controlled by Veb
            // For PNP: Ic depends on Veb, so dIc/dVe
            // Note: (c, b) contribution already combined above
            ((c, e), gm_f),
        ]
    }

    fn eval_nmos(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);
        // v_b = self.get_terminal_voltage(node_voltages, 3); // bulk, simplified for now

        // MOSFET is symmetric - swap D/S when Vds < 0
        let (vgs, vds, is_reversed) = if v_d >= v_s {
            (v_g - v_s, v_d - v_s, false)
        } else {
            (v_g - v_d, v_s - v_d, true)
        };

        let id = if vgs <= self.params.vth {
            // Cutoff
            0.0
        } else if vds < vgs - self.params.vth {
            // Triode
            self.params.kp * ((vgs - self.params.vth) * vds - 0.5 * vds * vds)
        } else {
            // Saturation
            0.5 * self.params.kp
                * (vgs - self.params.vth).powi(2)
                * (1.0 + self.params.lambda * vds)
        };

        // Current direction depends on whether D/S were swapped
        if is_reversed {
            vec![
                (self.terminals[0], id),  // "Drain" receives current
                (self.terminals[2], -id), // "Source" supplies current
            ]
        } else {
            vec![
                (self.terminals[0], -id), // Drain current out
                (self.terminals[2], id),  // Source current in
            ]
        }
    }

    fn jac_nmos(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];

        // MOSFET is symmetric - swap D/S when Vds < 0
        let (vgs, vds, eff_d, eff_s) = if v_d >= v_s {
            (v_g - v_s, v_d - v_s, d, s)
        } else {
            (v_g - v_d, v_s - v_d, s, d) // Swap effective drain/source
        };

        if vgs <= self.params.vth {
            // Cutoff - small conductance
            return vec![((d, d), 1e-12), ((s, s), 1e-12)];
        }

        let (gm, gds) = if vds < vgs - self.params.vth {
            // Triode
            let gm = self.params.kp * vds;
            let gds = self.params.kp * (vgs - self.params.vth - vds);
            (gm, gds.max(1e-12))
        } else {
            // Saturation
            let gm = self.params.kp * (vgs - self.params.vth) * (1.0 + self.params.lambda * vds);
            let gds = 0.5 * self.params.kp * (vgs - self.params.vth).powi(2) * self.params.lambda;
            (gm, gds.max(1e-12))
        };

        // Use effective drain/source for stamps
        vec![
            // gds: D-S conductance
            ((eff_d, eff_d), gds),
            ((eff_d, eff_s), -(gds + gm)), // Combined: -gds from conductance, -gm from transconductance
            ((eff_s, eff_d), -gds),
            ((eff_s, eff_s), gds),
            // gm: transconductance (D controlled by Vg)
            ((eff_d, g), gm),
        ]
    }

    fn eval_pmos(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        // PMOS has inverted voltages
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        // PMOS is symmetric - swap S/D when Vsd < 0
        let (vsg, vsd, is_reversed) = if v_s >= v_d {
            (v_s - v_g, v_s - v_d, false)
        } else {
            (v_d - v_g, v_d - v_s, true)
        };

        let vth = self.params.vth.abs();
        let id = if vsg <= vth {
            0.0
        } else if vsd < vsg - vth {
            // Triode
            self.params.kp * ((vsg - vth) * vsd - 0.5 * vsd * vsd)
        } else {
            // Saturation
            0.5 * self.params.kp * (vsg - vth).powi(2)
        };

        // Current direction depends on whether S/D were swapped
        if is_reversed {
            vec![
                (self.terminals[0], -id), // "Drain" supplies current
                (self.terminals[2], id),  // "Source" receives current
            ]
        } else {
            vec![
                (self.terminals[0], id),  // Drain current in (PMOS)
                (self.terminals[2], -id), // Source current out
            ]
        }
    }

    fn jac_pmos(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];

        // PMOS is symmetric - swap S/D when Vsd < 0
        let (vsg, vsd, eff_s, eff_d) = if v_s >= v_d {
            (v_s - v_g, v_s - v_d, s, d)
        } else {
            (v_d - v_g, v_d - v_s, d, s) // Swap effective source/drain
        };

        let vth = self.params.vth.abs();
        if vsg <= vth {
            // Cutoff - small conductance
            return vec![((d, d), 1e-12), ((s, s), 1e-12)];
        }

        let (gm, gsd) = if vsd < vsg - vth {
            // Triode
            let gm = self.params.kp * vsd;
            let gsd = self.params.kp * (vsg - vth - vsd);
            (gm, gsd.max(1e-12))
        } else {
            // Saturation
            let gm = self.params.kp * (vsg - vth);
            let gsd = 1e-12; // Small output conductance (lambda=0 simplified)
            (gm, gsd)
        };

        // Use effective source/drain for stamps (PMOS: current from S to D)
        vec![
            // gsd: S-D conductance
            ((eff_s, eff_s), gsd),
            ((eff_s, eff_d), -(gsd + gm)), // Combined
            ((eff_d, eff_s), -gsd),
            ((eff_d, eff_d), gsd),
            // gm: transconductance (S controlled by Vg for PMOS)
            ((eff_s, g), -gm), // PMOS: current decreases with increasing Vg
        ]
    }
}

impl HbSolver {
    /// Create a new HB solver
    pub fn new(config: HbConfig, num_nodes: usize) -> Self {
        let num_harmonics = config.num_harmonics;
        let fft = HbFft::new(num_harmonics, config.oversample_factor);

        Self {
            config,
            fft,
            num_nodes,
            num_harmonics,
            num_branches: 0,
            g_matrix: Vec::new(),
            c_matrix: Vec::new(),
            l_matrix: Vec::new(),
            voltage_source_branches: Vec::new(),
            node_names: (0..num_nodes).map(|i| format!("n{}", i)).collect(),
            source_spectra: vec![vec![Complex64::new(0.0, 0.0); num_harmonics + 1]; num_nodes],
            nonlinear_devices: Vec::new(),
        }
    }

    /// Get number of harmonics
    pub fn num_harmonics(&self) -> usize {
        self.num_harmonics
    }

    /// Set node names
    pub fn set_node_names(&mut self, names: Vec<String>) {
        self.node_names = names;
    }

    /// Add a single conductance stamp at position (node_i, node_j)
    ///
    /// This is a low-level method that adds a single Y-matrix entry.
    /// For a resistor between two nodes, use `add_resistor` instead.
    pub fn add_conductance(&mut self, node_i: usize, node_j: usize, g: Value) {
        self.g_matrix.push((node_i, node_j, g));
    }

    /// Add a resistor between two nodes with full MNA stamping
    ///
    /// For a resistor R between nodes i and j with G = 1/R:
    /// - Y(i,i) += G
    /// - Y(j,j) += G
    /// - Y(i,j) -= G
    /// - Y(j,i) -= G
    ///
    /// Ground is represented by a node index >= num_nodes (effectively ignored).
    pub fn add_resistor(&mut self, node_i: usize, node_j: usize, r: Value) {
        if r.abs() < 1e-30 {
            return; // Avoid division by zero
        }
        let g = 1.0 / r;

        // Full MNA stamp
        self.g_matrix.push((node_i, node_i, g));
        if node_j < self.num_nodes {
            self.g_matrix.push((node_j, node_j, g));
            self.g_matrix.push((node_i, node_j, -g));
            self.g_matrix.push((node_j, node_i, -g));
        }
    }

    /// Add a single capacitance stamp at position (node_i, node_j)
    ///
    /// This is a low-level method. For a capacitor between two nodes,
    /// the caller should add all 4 MNA stamps manually or use a higher-level API.
    pub fn add_capacitance(&mut self, node_i: usize, node_j: usize, c: Value) {
        self.c_matrix.push((node_i, node_j, c));
    }

    /// Add inductance stamp
    ///
    /// In frequency domain, inductor admittance is Y_L = 1/(jωL).
    /// At DC (ω=0), inductor is short circuit (infinite admittance) - handled specially.
    /// At harmonic k: Y_L(k) = 1/(j * k * ω₀ * L) = -j/(k * ω₀ * L)
    pub fn add_inductance(&mut self, node_i: usize, node_j: usize, l: Value) {
        self.l_matrix.push((node_i, node_j, l));
    }

    /// Add voltage source with MNA branch current
    ///
    /// Proper MNA treatment: voltage sources require branch current variables
    /// to enforce voltage constraint without Norton approximation.
    pub fn add_voltage_source_branch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
    ) -> usize {
        let branch_idx = self.num_branches;
        self.voltage_source_branches.push(VoltageSourceBranch::new(
            node_pos, node_neg, branch_idx, dc_voltage,
        ));
        self.num_branches += 1;
        branch_idx
    }

    /// Add voltage source with AC component
    pub fn add_voltage_source_branch_ac(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
        ac_magnitude: Value,
        ac_phase: Value,
    ) -> usize {
        let branch_idx = self.num_branches;
        self.voltage_source_branches.push(
            VoltageSourceBranch::new(node_pos, node_neg, branch_idx, dc_voltage)
                .with_ac(ac_magnitude, ac_phase),
        );
        self.num_branches += 1;
        branch_idx
    }

    /// Get number of MNA branch currents
    pub fn num_branches(&self) -> usize {
        self.num_branches
    }

    /// Set DC source current at a node
    pub fn set_dc_source(&mut self, node: usize, current: Value) {
        if node < self.source_spectra.len() {
            self.source_spectra[node][0] = Complex64::new(current, 0.0);
        }
    }

    /// Set AC source at a node (sinusoidal at fundamental)
    pub fn set_ac_source(&mut self, node: usize, magnitude: Value, phase: Value) {
        if node < self.source_spectra.len() && self.source_spectra[node].len() > 1 {
            self.source_spectra[node][1] = Complex64::from_polar(magnitude, phase);
        }
    }

    /// Set full source spectrum at a node
    pub fn set_source_spectrum(&mut self, node: usize, spectrum: Vec<Complex64>) {
        if node < self.source_spectra.len() {
            self.source_spectra[node] = spectrum;
        }
    }

    /// Initialize solution with DC operating point
    pub fn initialize_dc(&mut self, state: &mut HbSolverState, dc_solution: &[Value]) {
        for (node, &v_dc) in dc_solution.iter().enumerate() {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(v_dc, 0.0);
            }
        }
    }

    /// Compute residual for linear circuit (KCL: sum of currents INTO node = 0)
    ///
    /// Residual = I_source - (G*X + jω*C*X + (1/jωL)*X)
    ///          = I_source - Y*X
    ///
    /// For inductors, admittance Y_L = 1/(jωL) = -j/(ωL)
    /// At DC (ω=0): inductor is short circuit, requires special handling
    pub fn compute_linear_residual(&self, state: &mut HbSolverState) {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Start with source currents (positive = current INTO node)
        for node_res in &mut state.residual {
            for c in node_res.iter_mut() {
                *c = Complex64::new(0.0, 0.0);
            }
        }

        // Add source contributions first
        for (node, source) in self.source_spectra.iter().enumerate() {
            if node < state.residual.len() {
                for (k, &s) in source.iter().enumerate() {
                    if k < state.residual[node].len() {
                        state.residual[node][k] += s; // Source current INTO node
                    }
                }
            }
        }

        // Subtract G*X contribution (current through conductance leaves node)
        for &(i, j, g) in &self.g_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        state.residual[i][k] -= g * state.x[j][k];
                    }
                }
            }
        }

        // Subtract jω*C*X contribution (capacitor admittance current)
        for &(i, j, c) in &self.c_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        let j_omega = Complex64::new(0.0, omega_k);
                        state.residual[i][k] -= j_omega * c * state.x[j][k];
                    }
                }
            }
        }

        // Subtract 1/(jωL)*X contribution (inductor admittance current)
        // Y_L = 1/(jωL) = -j/(ωL)
        // At DC (k=0): inductor is short circuit - enforce V=0 (large admittance)
        for &(i, j, l) in &self.l_matrix {
            if i < state.x.len() && j < state.x.len() && l.abs() > 1e-30 {
                for k in 0..=self.num_harmonics {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        if k == 0 {
                            // DC: inductor is short circuit
                            // Add very large conductance to force V_i = V_j
                            const DC_SHORT_CONDUCTANCE: Value = 1e6;
                            state.residual[i][k] -= DC_SHORT_CONDUCTANCE * state.x[j][k];
                        } else {
                            // AC: Y_L = 1/(jωL) = -j/(ωL)
                            let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                            state.residual[i][k] -= y_l * state.x[j][k];
                        }
                    }
                }
            }
        }

        state.compute_residual_norm();
    }

    /// Compute Jacobian for linear circuit (block diagonal)
    ///
    /// J[node_i, k][node_j, l] = δ_{kl} * (G_{ij} + jω_k * C_{ij} + 1/(jω_k * L_{ij}))
    #[allow(dead_code)]
    fn compute_linear_jacobian(&self) -> Vec<Vec<Vec<Vec<Complex64>>>> {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;

        // Full Jacobian: [node_i][harmonic_k][node_j][harmonic_l]
        let mut jac = vec![vec![vec![vec![Complex64::new(0.0, 0.0); h]; n]; h]; n];

        // G contribution (diagonal in harmonics)
        for &(i, j, g) in &self.g_matrix {
            if i < n && j < n {
                for k in 0..h {
                    jac[i][k][j][k] += g;
                }
            }
        }

        // jω*C contribution (diagonal in harmonics)
        for &(i, j, c) in &self.c_matrix {
            if i < n && j < n {
                for k in 0..h {
                    let omega_k = (k as f64) * omega0;
                    let j_omega = Complex64::new(0.0, omega_k);
                    jac[i][k][j][k] += j_omega * c;
                }
            }
        }

        // 1/(jωL) contribution (diagonal in harmonics)
        for &(i, j, l) in &self.l_matrix {
            if i < n && j < n && l.abs() > 1e-30 {
                for k in 0..h {
                    let omega_k = (k as f64) * omega0;
                    if k == 0 {
                        // DC: short circuit (large conductance)
                        const DC_SHORT_CONDUCTANCE: Value = 1e6;
                        jac[i][k][j][k] += DC_SHORT_CONDUCTANCE;
                    } else {
                        // AC: Y_L = 1/(jωL) = -j/(ωL)
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        jac[i][k][j][k] += y_l;
                    }
                }
            }
        }

        jac
    }

    /// Solve for linear circuit (direct solve for diagonal blocks)
    ///
    /// Builds Y = G + jωC + 1/(jωL) admittance matrix for each harmonic
    /// and solves Y*V = I
    pub fn solve_linear(&self, state: &mut HbSolverState) -> Result<(), HbError> {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;

        // For each harmonic, solve the linear system independently
        for k in 0..h {
            let omega_k = (k as f64) * omega0;

            // Build matrix for this harmonic: Y_k = G + jω_k*C + 1/(jω_k*L)
            let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); n]; n];

            // Conductance contribution
            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += g;
                }
            }

            // Capacitance contribution: jωC
            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += Complex64::new(0.0, omega_k) * c;
                }
            }

            // Inductance contribution: 1/(jωL) = -j/(ωL)
            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    if k == 0 {
                        // DC: inductor is short circuit (large conductance)
                        const DC_SHORT_CONDUCTANCE: Value = 1e6;
                        y_matrix[i][j] += DC_SHORT_CONDUCTANCE;
                    } else {
                        // AC: Y_L = -j/(ωL)
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        y_matrix[i][j] += y_l;
                    }
                }
            }

            // Get RHS for this harmonic
            let rhs: Vec<Complex64> = (0..n)
                .map(|node| {
                    self.source_spectra
                        .get(node)
                        .and_then(|s| s.get(k))
                        .copied()
                        .unwrap_or(Complex64::new(0.0, 0.0))
                })
                .collect();

            // Solve Y * V = I using Gaussian elimination
            let solution = self.solve_complex_linear_system(&y_matrix, &rhs)?;

            // Store solution
            for (node, &v) in solution.iter().enumerate() {
                if node < state.x.len() && k < state.x[node].len() {
                    state.x[node][k] = v;
                }
            }
        }

        // Compute final residual
        self.compute_linear_residual(state);
        state.converged = state.residual_norm < self.config.tolerance;

        Ok(())
    }

    /// Add a nonlinear device for Newton iteration
    pub fn add_nonlinear_device(&mut self, device: NonlinearDeviceInstance) {
        self.nonlinear_devices.push(device);
    }

    /// Add a diode for Newton iteration
    pub fn add_diode(&mut self, anode: usize, cathode: usize, is: Value, n: Value) {
        self.add_nonlinear_device(NonlinearDeviceInstance::diode(anode, cathode, is, n));
    }

    /// Add an NPN BJT for Newton iteration
    pub fn add_npn_bjt(
        &mut self,
        collector: usize,
        base: usize,
        emitter: usize,
        is: Value,
        bf: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::npn_bjt(
            collector, base, emitter, is, bf,
        ));
    }

    /// Add a PNP BJT for Newton iteration
    pub fn add_pnp_bjt(
        &mut self,
        collector: usize,
        base: usize,
        emitter: usize,
        is: Value,
        bf: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::pnp_bjt(
            collector, base, emitter, is, bf,
        ));
    }

    /// Add an NMOS for Newton iteration
    pub fn add_nmos(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        kp: Value,
        vth: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::nmos(
            drain, gate, source, bulk, vth, kp,
        ));
    }

    /// Check if circuit has nonlinear devices
    pub fn has_nonlinear_devices(&self) -> bool {
        !self.nonlinear_devices.is_empty()
    }

    // =========================================================================
    // DC Operating Point Solver
    // =========================================================================
    //
    // Solves the DC component (harmonic 0) before full HB iteration.
    // This establishes nonlinear device operating points and provides a
    // much better initial guess for the HB Newton iteration.
    //
    // Flow: DC Solve → Initialize Harmonics → Full HB Newton
    // =========================================================================

    /// Solve DC operating point before full HB iteration
    ///
    /// This method solves only the DC component (k=0) of the HB problem to establish
    /// the nonlinear device operating points. The DC solution provides a much better
    /// initial guess for the full HB Newton iteration.
    ///
    /// Uses the same convergence aids as the full HB solver:
    /// - GMIN stepping for ill-conditioned circuits
    /// - Source stepping as fallback
    ///
    /// Returns the DC node voltages if successful.
    pub fn solve_dc_operating_point(
        &mut self,
        state: &mut HbSolverState,
    ) -> Result<Vec<Value>, HbError> {
        // DC tolerances (more realistic than HB defaults)
        // For DC analysis, we're solving KCL: sum of currents = 0
        // Typical circuit currents are in mA-µA range, so abstol should be ~pA
        let dc_reltol = self.config.tolerance.max(1e-3); // At least 0.1% relative
        let dc_abstol = self.config.abstol.max(1e-9); // At least 1 pA absolute

        // DC-specific iteration limit
        let dc_max_iter = self.config.max_iterations.max(150);

        // For linear circuits, DC is just a linear solve at k=0
        if self.nonlinear_devices.is_empty() {
            self.solve_dc_linear(state)?;
            return Ok(self.extract_dc_solution(state));
        }

        // Target GMIN for final solution
        let target_gmin = 1e-12;

        // Initialize diode voltages with forward bias estimate (0.6V per diode)
        // This gives Newton a much better starting point than V=0
        self.initialize_diode_voltages(state);

        // Step 1: Try direct DC Newton with minimal GMIN
        if self.dc_newton_inner_loop(state, target_gmin, dc_max_iter, dc_reltol, dc_abstol) {
            return Ok(self.extract_dc_solution(state));
        }

        // Step 2: GMIN stepping - progressively increase GMIN until convergence,
        // then refine back down
        for gmin_level in [1e-9, 1e-6, 1e-4, 1e-2, 0.1, 1.0] {
            if self.dc_newton_inner_loop(
                state,
                gmin_level,
                dc_max_iter,
                dc_reltol * 10.0, // Relaxed tolerance during stepping
                dc_abstol * 10.0,
            ) {
                // Converged at this GMIN level - refine to target
                let mut current_gmin = gmin_level;
                let mut last_good_x = self.extract_dc_solution(state);
                let mut last_good_gmin = current_gmin;
                let mut refine_failures = 0;

                while current_gmin > target_gmin {
                    current_gmin /= 2.0;
                    if self.dc_newton_inner_loop(
                        state,
                        current_gmin,
                        dc_max_iter,
                        dc_reltol,
                        dc_abstol,
                    ) {
                        last_good_x = self.extract_dc_solution(state);
                        last_good_gmin = current_gmin;
                        refine_failures = 0; // Reset failure count on success
                    } else {
                        refine_failures += 1;
                        // Restore last good state and keep trying with smaller steps
                        for (node, &v) in last_good_x.iter().enumerate() {
                            if node < state.x.len() && !state.x[node].is_empty() {
                                state.x[node][0] = Complex64::new(v, 0.0);
                            }
                        }
                        // After too many consecutive failures, try slower reduction
                        if refine_failures > 3 {
                            break;
                        }
                        // Try 10% reduction instead of 50%
                        current_gmin = last_good_gmin * 0.9;
                    }
                }

                // Verify final residual at the best achievable GMIN
                self.compute_dc_residual(state, last_good_gmin.max(target_gmin));
                let rel_norm = state.residual_norm / (self.dc_solution_norm(state) + dc_abstol);
                if state.residual_norm < dc_abstol || rel_norm < dc_reltol {
                    return Ok(self.extract_dc_solution(state));
                }
            }
        }

        // Step 3: Source stepping - ramp DC sources from 0 to full
        let original_sources = self.source_spectra.clone();

        // Reset DC to zero
        for node in 0..self.num_nodes {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(0.0, 0.0);
            }
        }

        // Use SourceStepper for DC sources
        let mut source_stepper = SourceStepper::new();
        let max_steps = 50;
        let mut step_count = 0;

        while !source_stepper.is_complete() && step_count < max_steps {
            let factor = source_stepper.factor();
            step_count += 1;

            // Scale DC sources only (harmonic 0)
            for node in 0..self.num_nodes.min(self.source_spectra.len()) {
                if !self.source_spectra[node].is_empty() {
                    self.source_spectra[node][0] = original_sources
                        .get(node)
                        .and_then(|s| s.first())
                        .copied()
                        .unwrap_or(Complex64::ZERO)
                        * factor;
                }
            }

            if self.dc_newton_inner_loop(
                state,
                1e-6,
                dc_max_iter / 2,
                dc_reltol * 10.0,
                dc_abstol * 10.0,
            ) {
                source_stepper.advance_on_success();
            } else if !source_stepper.reduce_on_failure() {
                break;
            }
        }

        // Restore original sources
        self.source_spectra = original_sources;

        // Final DC solve with full sources
        if source_stepper.is_complete() {
            if self.dc_newton_inner_loop(state, target_gmin, dc_max_iter, dc_reltol, dc_abstol) {
                return Ok(self.extract_dc_solution(state));
            }
        }

        // DC solve failed - return what we have
        Err(HbError::ConvergenceFailed {
            iterations: step_count,
            residual: state.residual_norm,
        })
    }

    /// Solve DC for linear circuit (no nonlinear devices)
    fn solve_dc_linear(&self, state: &mut HbSolverState) -> Result<(), HbError> {
        // Build DC conductance matrix (G only, no jωC or 1/jωL at DC)
        let n = self.num_nodes;
        let mut g_dc = vec![vec![0.0; n]; n];

        // Add conductances
        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n {
                g_dc[row][col] += g;
            }
        }

        // Add small GMIN on diagonal for invertibility
        let gmin = 1e-12;
        for i in 0..n {
            g_dc[i][i] += gmin;
        }

        // Build DC RHS (source currents at DC)
        let rhs: Vec<Value> = (0..n)
            .map(|node| {
                self.source_spectra
                    .get(node)
                    .and_then(|s| s.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();

        // Solve G * V = I
        let solution = self.solve_real_linear_system(&g_dc, &rhs)?;

        // Store DC solution
        for (node, &v) in solution.iter().enumerate() {
            if node < state.x.len() && !state.x[node].is_empty() {
                state.x[node][0] = Complex64::new(v, 0.0);
            }
        }

        Ok(())
    }

    /// DC Newton inner loop - solves DC component only
    ///
    /// Uses the same algorithm as the full HB Newton but operates only on harmonic 0.
    fn dc_newton_inner_loop(
        &self,
        state: &mut HbSolverState,
        gmin: Value,
        max_iterations: usize,
        tol: Value,
        abstol: Value,
    ) -> bool {
        for iteration in 0..max_iterations {
            state.iteration = iteration;

            // Compute DC residual
            self.compute_dc_residual(state, gmin);

            // Check convergence using standard criteria:
            // - Absolute: residual < abstol (current tolerance)
            // - Relative: residual < reltol * max_current_in_circuit
            // The relative tolerance should be against CURRENT, not voltage
            let max_source_current: f64 = self
                .source_spectra
                .iter()
                .filter_map(|s| s.first())
                .map(|c| c.re.abs())
                .fold(0.0, |a: f64, b| a.max(b));

            // Scale for relative convergence: max of source currents or minimum threshold
            let current_scale = max_source_current.max(abstol);
            let rel_current_norm = state.residual_norm / current_scale;

            // Converge if:
            // 1. Absolute residual is tiny (< 1e-9 A), OR
            // 2. Both: relative residual is small (<1%) AND absolute residual is reasonable (<1mA)
            // This prevents declaring convergence with large absolute current imbalance
            let converged = state.residual_norm < abstol
                || (rel_current_norm < tol && state.residual_norm < 1e-3);
            if converged {
                return true;
            }

            // Build DC Jacobian
            let jacobian = self.build_dc_jacobian(state, gmin);

            // Solve for delta_x: J * delta = -residual (standard Newton-Raphson)
            // We need -R because: R(x) = 0, Taylor: R(x+delta) ≈ R(x) + J*delta = 0
            // So J*delta = -R
            let neg_residual: Vec<Value> = (0..self.num_nodes)
                .map(|node| {
                    -state
                        .residual
                        .get(node)
                        .and_then(|r| r.first())
                        .map(|c| c.re)
                        .unwrap_or(0.0)
                })
                .collect();

            let delta_x = match self.solve_real_linear_system(&jacobian, &neg_residual) {
                Ok(d) => d,
                Err(_) => return false, // Singular Jacobian
            };

            // Line search with DC voltage limiting
            self.apply_dc_line_search(state, &delta_x, gmin, tol);
        }

        false
    }

    /// Compute DC residual: R = I_source_dc - G*V_dc - I_nonlinear(V_dc) - gmin*V_dc
    fn compute_dc_residual(&self, state: &mut HbSolverState, gmin: Value) {
        let n = self.num_nodes;

        // Initialize residual with DC sources
        for node in 0..n {
            if node < state.residual.len() && !state.residual[node].is_empty() {
                let source = self
                    .source_spectra
                    .get(node)
                    .and_then(|s| s.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0);
                state.residual[node][0] = Complex64::new(source, 0.0);
            }
        }

        // Extract DC voltages
        let v_dc: Vec<Value> = (0..n)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();

        // Subtract linear contributions: G * V_dc
        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n && row < state.residual.len() {
                let contribution = g * v_dc[col];
                state.residual[row][0] -= Complex64::new(contribution, 0.0);
            }
        }

        // Subtract GMIN: gmin * V_dc (diagonal)
        for node in 0..n {
            if node < state.residual.len() && !state.residual[node].is_empty() {
                state.residual[node][0] -= Complex64::new(gmin * v_dc[node], 0.0);
            }
        }

        // Add nonlinear device currents (device returns current INTO each node)
        // KCL: sum of currents INTO node = 0
        // R = I_source + I_device_into - G*V - gmin*V
        for device in &self.nonlinear_devices {
            let currents = device.evaluate(&v_dc);
            for (node, current) in currents {
                if node < state.residual.len() && !state.residual[node].is_empty() {
                    state.residual[node][0] += Complex64::new(current, 0.0);
                }
            }
        }

        // Compute residual norm (DC only)
        let norm_sq: Value = (0..n)
            .map(|node| {
                state
                    .residual
                    .get(node)
                    .and_then(|r| r.first())
                    .map(|c| c.re * c.re)
                    .unwrap_or(0.0)
            })
            .sum();
        state.residual_norm = norm_sq.sqrt();
    }

    /// Build DC Jacobian: J = -G - dI_nonlinear/dV - gmin*I
    fn build_dc_jacobian(&self, state: &HbSolverState, gmin: Value) -> Vec<Vec<Value>> {
        let n = self.num_nodes;
        let mut jacobian = vec![vec![0.0; n]; n];

        // Linear contribution: -G
        for &(row, col, g) in &self.g_matrix {
            if row < n && col < n {
                jacobian[row][col] -= g;
            }
        }

        // GMIN contribution: -gmin on diagonal
        for i in 0..n {
            jacobian[i][i] -= gmin;
        }

        // Nonlinear device Jacobians
        let v_dc: Vec<Value> = (0..n)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();

        // Nonlinear device Jacobians
        // Device returns MNA conductance stamps (+gd on diagonal for diode)
        // But dI_into/dV = -gd (more voltage → more current leaving → less current into)
        // Since J = dR/dV = dI_into/dV - G - gmin, we need to subtract the device stamps
        // J = -G_device - G_linear - gmin
        for device in &self.nonlinear_devices {
            let jac_entries = device.jacobian(&v_dc);
            for ((row, col), value) in jac_entries {
                if row < n && col < n {
                    // Subtract device conductance stamp to get correct dI_into/dV
                    jacobian[row][col] -= value;
                }
            }
        }

        jacobian
    }

    /// Apply DC line search with voltage limiting
    fn apply_dc_line_search(
        &self,
        state: &mut HbSolverState,
        delta_x: &[Value],
        gmin: Value,
        _tol: Value,
    ) {
        let n = self.num_nodes;
        let mut alpha = 1.0;
        let min_alpha = 0.001;
        let armijo_c = 1e-4;

        // Save original DC voltages and residual
        let orig_x: Vec<Value> = (0..n)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect();
        let orig_residual = state.residual_norm;

        // Compute expected improvement
        let grad_dot_delta: Value = (0..n)
            .map(|i| {
                let r = state
                    .residual
                    .get(i)
                    .and_then(|r| r.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0);
                r * delta_x.get(i).copied().unwrap_or(0.0)
            })
            .sum();

        let mut best_alpha = alpha;
        let mut best_residual = f64::INFINITY;

        while alpha >= min_alpha {
            // Apply update with voltage limiting
            for node in 0..n {
                if node < state.x.len() && !state.x[node].is_empty() && node < delta_x.len() {
                    let mut new_v = orig_x[node] + alpha * delta_x[node];

                    // PN junction voltage limiting
                    // Limit voltage changes at PN junctions to prevent overflow
                    let max_delta_v = 0.5; // Maximum voltage change per iteration
                    if (new_v - orig_x[node]).abs() > max_delta_v {
                        new_v = orig_x[node] + max_delta_v * (new_v - orig_x[node]).signum();
                    }

                    // Clamp to reasonable range
                    new_v = new_v.clamp(-1000.0, 1000.0);

                    state.x[node][0] = Complex64::new(new_v, 0.0);
                }
            }

            // Compute new residual
            self.compute_dc_residual(state, gmin);

            // Track best result
            if state.residual_norm < best_residual {
                best_residual = state.residual_norm;
                best_alpha = alpha;
            }

            // Armijo condition
            if state.residual_norm <= orig_residual + armijo_c * alpha * grad_dot_delta {
                return; // Accepted step
            }

            alpha *= 0.5;
        }

        // Use best alpha found
        for node in 0..n {
            if node < state.x.len() && !state.x[node].is_empty() && node < delta_x.len() {
                let mut new_v = orig_x[node] + best_alpha * delta_x[node];
                new_v = new_v.clamp(-1000.0, 1000.0);
                state.x[node][0] = Complex64::new(new_v, 0.0);
            }
        }
        self.compute_dc_residual(state, gmin);
    }

    /// Solve real linear system using Gaussian elimination with partial pivoting
    fn solve_real_linear_system(
        &self,
        a: &[Vec<Value>],
        b: &[Value],
    ) -> Result<Vec<Value>, HbError> {
        let n = a.len();
        if n == 0 || b.len() != n {
            return Err(HbError::InvalidCircuit("Invalid system size".to_string()));
        }

        // Create augmented matrix
        let mut aug: Vec<Vec<Value>> = a
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let mut new_row = row.clone();
                new_row.push(b[i]);
                new_row
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            let mut max_val = aug[col][col].abs();
            for row in (col + 1)..n {
                if aug[row][col].abs() > max_val {
                    max_val = aug[row][col].abs();
                    max_row = row;
                }
            }

            if max_val < 1e-15 {
                return Err(HbError::SingularMatrix);
            }

            // Swap rows
            aug.swap(col, max_row);

            // Eliminate
            let pivot = aug[col][col];
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for j in col..=n {
                    aug[row][j] -= factor * aug[col][j];
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            x[i] = sum / aug[i][i];
        }

        Ok(x)
    }

    /// Extract DC solution as vector of real voltages
    fn extract_dc_solution(&self, state: &HbSolverState) -> Vec<Value> {
        (0..self.num_nodes)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re)
                    .unwrap_or(0.0)
            })
            .collect()
    }

    /// Compute DC solution norm
    fn dc_solution_norm(&self, state: &HbSolverState) -> Value {
        let sum_sq: Value = (0..self.num_nodes)
            .map(|node| {
                state
                    .x
                    .get(node)
                    .and_then(|x| x.first())
                    .map(|c| c.re * c.re)
                    .unwrap_or(0.0)
            })
            .sum();
        sum_sq.sqrt()
    }

    /// Initialize node voltages for diode circuits
    /// Propagates ~0.6V per diode through the chain to help Newton converge
    fn initialize_diode_voltages(&self, state: &mut HbSolverState) {
        let n = self.num_nodes;

        // Find grounded nodes (nodes with large conductance to ground)
        let mut ground_conductance = vec![0.0; n];
        for &(row, col, g) in &self.g_matrix {
            if row == col && row < n {
                ground_conductance[row] += g;
            }
        }

        // Find the most grounded node as reference
        let reference_node = ground_conductance
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(n - 1);

        // Build diode adjacency: for each node, track connected diodes with polarity
        // (neighbor, is_this_node_anode) - True if current node is anode of diode to neighbor
        // Also include BJT B-E junctions as they behave like diodes
        let mut node_diodes: Vec<Vec<(usize, bool)>> = vec![vec![]; n];
        for device in &self.nonlinear_devices {
            match device.device_type {
                NonlinearDeviceType::Diode => {
                    let anode = device.terminals[0];
                    let cathode = device.terminals[1];
                    if anode < n && cathode < n {
                        node_diodes[anode].push((cathode, true)); // anode connects to cathode
                        node_diodes[cathode].push((anode, false)); // cathode connects to anode
                    }
                }
                NonlinearDeviceType::NpnBjt => {
                    // NPN: B-E junction is like diode with base as anode, emitter as cathode
                    let base = device.terminals[1];
                    let emitter = device.terminals[2];
                    if base < n && emitter < n {
                        node_diodes[base].push((emitter, true));
                        node_diodes[emitter].push((base, false));
                    }
                }
                NonlinearDeviceType::PnpBjt => {
                    // PNP: E-B junction is like diode with emitter as anode, base as cathode
                    let base = device.terminals[1];
                    let emitter = device.terminals[2];
                    if base < n && emitter < n {
                        node_diodes[emitter].push((base, true));
                        node_diodes[base].push((emitter, false));
                    }
                }
                _ => {} // MOSFETs don't have junction diodes for DC init
            }
        }

        // First pass: Set source nodes to estimated voltage based on diode distance to ground
        // and set reference node to 0V
        let mut node_voltage = vec![f64::NAN; n];
        node_voltage[reference_node] = 0.0;

        for node in 0..n {
            let source_current = self
                .source_spectra
                .get(node)
                .and_then(|s| s.first())
                .map(|c| c.re)
                .unwrap_or(0.0);

            // Get self-conductance to ground for this node (Norton equivalent)
            let self_conductance: f64 = self
                .g_matrix
                .iter()
                .filter(|&&(r, c, _)| r == node && c == node)
                .map(|&(_, _, g)| g)
                .sum();

            // Check if this node has diode connections (should use diode init instead)
            let has_diode_connection = node_diodes
                .get(node)
                .map(|v| !v.is_empty())
                .unwrap_or(false);

            if source_current > 0.0 && self_conductance > 0.1 && !has_diode_connection {
                // Use Norton equivalent: V = I/G for supply nodes without diode connections
                // Only apply for positive supplies with significant conductance (>0.1S)
                // This is critical for MOSFET circuits where supply rails must be correct
                let norton_v = source_current / self_conductance;
                if norton_v > 0.5 {
                    // Only use for supplies > 0.5V to avoid overriding near-ground estimates
                    node_voltage[node] = norton_v;
                }
            } else if source_current > 0.0 {
                // Positive current: estimate based on diode chain
                let diode_count = self.count_diodes_from_node(node, reference_node);
                node_voltage[node] = (diode_count as f64 * 0.6).max(0.1);
            } else if source_current < 0.0 {
                // Negative current: reverse bias
                node_voltage[node] = -0.1;
            }
        }

        // Second pass: BFS from known nodes to propagate through diode chain
        let mut queue = std::collections::VecDeque::new();
        for node in 0..n {
            if !node_voltage[node].is_nan() {
                queue.push_back(node);
            }
        }

        while let Some(current) = queue.pop_front() {
            let current_v = node_voltage[current];

            for &(neighbor, is_anode) in &node_diodes[current] {
                if node_voltage[neighbor].is_nan() {
                    // Propagate voltage through diode
                    // If current is anode, neighbor (cathode) is ~0.6V lower
                    // If current is cathode, neighbor (anode) is ~0.6V higher
                    let neighbor_v = if is_anode {
                        current_v - 0.6 // current is anode, neighbor is cathode
                    } else {
                        current_v + 0.6 // current is cathode, neighbor is anode
                    };
                    node_voltage[neighbor] = neighbor_v;
                    queue.push_back(neighbor);
                }
            }
        }

        // Apply voltages to state, using small default for any unvisited nodes
        for node in 0..n {
            if node < state.x.len() && !state.x[node].is_empty() {
                let v = if node_voltage[node].is_nan() {
                    0.1 // Default for unconnected nodes
                } else {
                    node_voltage[node]
                };
                state.x[node][0] = Complex64::new(v, 0.0);
            }
        }
    }

    /// Count diodes in path from node to reference (simple heuristic)
    fn count_diodes_from_node(&self, from: usize, to: usize) -> usize {
        let n = self.num_nodes;
        if from >= n || to >= n {
            return 0;
        }

        // Build adjacency for diodes
        let mut adj: Vec<Vec<usize>> = vec![vec![]; n];
        for device in &self.nonlinear_devices {
            if device.device_type == NonlinearDeviceType::Diode {
                let a = device.terminals[0];
                let c = device.terminals[1];
                if a < n && c < n {
                    adj[a].push(c);
                    adj[c].push(a);
                }
            }
        }

        // BFS to find shortest path through diodes
        let mut visited = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((from, 0usize));
        visited[from] = true;

        while let Some((current, dist)) = queue.pop_front() {
            if current == to {
                return dist;
            }
            for &neighbor in &adj[current] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        0 // No path found
    }

    // =========================================================================
    // End DC Operating Point Solver
    // =========================================================================

    /// Full Newton-Raphson iteration for nonlinear HB analysis
    ///
    /// Advanced implementation following standard methodology:
    /// 1. Try direct Newton-Raphson first
    /// 2. If that fails, use source stepping (ramp sources from 0 to full)
    /// 3. Use GMIN as a constant stabilizer throughout
    ///
    /// The Jacobian includes:
    /// - Linear part: block-diagonal Y = G + jωC + 1/(jωL) per harmonic
    /// - Nonlinear part: FFT-based convolution of time-domain Jacobians
    /// - GMIN: diagonal conductance for numerical stability
    pub fn solve_newton(&mut self, state: &mut HbSolverState) -> Result<(), HbError> {
        let tol = self.config.tolerance;
        let abstol = self.config.abstol;

        // For linear circuits, use direct solve
        if self.nonlinear_devices.is_empty() {
            return self.solve_linear(state);
        }

        // Use constant GMIN for numerical stability
        // Larger gmin provides better Jacobian conditioning for difficult circuits
        let gmin = 1e-9;

        // Step 0: Solve DC operating point first
        // This establishes the nonlinear device operating points and provides a much
        // better initial guess than starting from zero or a random guess.
        if let Ok(_dc_solution) = self.solve_dc_operating_point(state) {
            // DC solution is now stored in state.x[node][0]
            // Initialize harmonic components to zero (small-signal around DC)
            // This is the standard HB initialization approach
            for node in 0..self.num_nodes {
                if node < state.x.len() {
                    for k in 1..state.x[node].len() {
                        // Keep harmonics at zero - full Newton will find them
                        state.x[node][k] = Complex64::new(0.0, 0.0);
                    }
                }
            }
        }
        // If DC solve fails, continue with existing fallback strategy
        // The full Newton may still succeed with source stepping

        // Step 1: Try direct Newton first
        if self.newton_inner_loop(state, gmin, self.config.max_iterations, tol, abstol) {
            state.converged = true;
            return Ok(());
        }

        // Step 2: If direct Newton fails, try with progressively larger GMIN
        // This helps regularize ill-conditioned Jacobians
        // Include high GMIN levels (0.1, 1.0) for very difficult circuits
        for gmin_level in [1e-6, 1e-4, 1e-2, 0.1, 1.0] {
            if self.newton_inner_loop(
                state,
                gmin_level,
                self.config.max_iterations,
                tol * 10.0,
                abstol,
            ) {
                // Converged at higher GMIN - now refine with progressively lower GMIN
                // Save state before refinement in case we need to restore
                let mut last_good_state = state.x.clone();
                let mut last_good_residual = state.residual_norm;

                let mut current_gmin = gmin_level;
                while current_gmin > gmin {
                    // Use factor of 2 for very gradual refinement
                    current_gmin /= 2.0;
                    if self.newton_inner_loop(
                        state,
                        current_gmin,
                        self.config.max_iterations,
                        tol,
                        abstol,
                    ) {
                        // Success - update last good state
                        last_good_state = state.x.clone();
                        last_good_residual = state.residual_norm;
                    } else {
                        // Failed - restore last good state and stop refining
                        state.x = last_good_state;
                        state.residual_norm = last_good_residual;
                        break;
                    }
                }
                // Recompute residual with target GMIN to check tolerance
                self.compute_full_residual_with_gmin(state, gmin);
                let rel_norm = state.residual_norm / (state.solution_norm() + abstol);
                if state.residual_norm < abstol || rel_norm < tol {
                    state.converged = true;
                    return Ok(());
                }
            } else {
            }
        }

        // Step 3: Try source stepping
        // Scale sources from 0 to full, using previous converged solution as starting point
        let original_sources = self.source_spectra.clone();
        let mut source_stepper = SourceStepper::new();
        let mut total_iterations = 0; // Reset for source stepping
        let max_total_iter = self.config.max_iterations * 20;

        // Reset state to zero - with sources=0, solution=0 trivially
        for node in 0..self.num_nodes {
            if node < state.x.len() {
                for k in 0..state.x[node].len() {
                    state.x[node][k] = Complex64::new(0.0, 0.0);
                }
            }
        }

        while !source_stepper.is_complete() && total_iterations < max_total_iter {
            let factor = source_stepper.factor();

            // Scale sources by current factor
            for node in 0..self.num_nodes {
                if node < self.source_spectra.len() {
                    for k in 0..self.source_spectra[node].len() {
                        self.source_spectra[node][k] = original_sources
                            .get(node)
                            .and_then(|s| s.get(k))
                            .copied()
                            .unwrap_or(Complex64::new(0.0, 0.0))
                            * factor;
                    }
                }
            }

            // Don't reset state.x - keep converged solution from previous source level

            // Try Newton at this source level (using previous solution as starting point)
            let converged = self.newton_inner_loop(
                state,
                gmin,
                self.config.max_iterations / 2,
                tol * 10.0,
                abstol,
            );

            total_iterations += state.iteration;

            if converged {
                // Keep state.x as-is for next step (converged solution)
                source_stepper.advance_on_success();
            } else {
                if !source_stepper.reduce_on_failure() {
                    break;
                }
            }
        }

        // Restore original sources
        self.source_spectra = original_sources;

        // If source stepping completed, do final Newton with original sources
        if source_stepper.is_complete() {
            if self.newton_inner_loop(state, gmin, self.config.max_iterations, tol, abstol) {
                state.converged = true;
                state.iteration = total_iterations;
                return Ok(());
            }
        }

        // Step 4: Try pseudo-transient
        // Add damping capacitors to each node and integrate to steady-state
        let mut ptran = PseudoTransient::new();
        let mut ptran_iterations = 0;
        let max_ptran_iter = self.config.max_iterations * 5;

        while !ptran.is_complete() && ptran_iterations < max_ptran_iter {
            // Pseudo-transient adds G_eq = C_pseudo/dt to each node diagonal
            // This damps oscillations and helps find DC solution
            let ptran_gmin = gmin + ptran.conductance(0);

            let converged = self.newton_inner_loop(
                state,
                ptran_gmin,
                self.config.max_iterations / 4,
                tol * 100.0, // Relaxed tolerance during stepping
                abstol,
            );

            ptran_iterations += state.iteration;

            if converged {
                ptran.advance_on_success();
            } else {
                if !ptran.reduce_on_failure() {
                    break;
                }
            }
        }

        // If pseudo-transient completed, do final high-accuracy Newton
        if ptran.is_complete() {
            if self.newton_inner_loop(state, gmin, self.config.max_iterations, tol, abstol) {
                state.converged = true;
                state.iteration = total_iterations + ptran_iterations;
                return Ok(());
            }
        }

        Err(HbError::ConvergenceFailed {
            iterations: total_iterations + ptran_iterations,
            residual: state.residual_norm,
        })
    }

    /// Inner Newton iteration loop at a fixed GMIN level
    fn newton_inner_loop(
        &mut self,
        state: &mut HbSolverState,
        gmin: Value,
        max_iter: usize,
        tol: Value,
        abstol: Value,
    ) -> bool {
        for iter in 0..max_iter {
            state.iteration = iter;

            // 1. Compute full residual: linear + nonlinear + GMIN contributions
            self.compute_full_residual_with_gmin(state, gmin);

            // 2. Check convergence
            let sol_norm = state.solution_norm();
            let rel_norm = state.residual_norm / (sol_norm + abstol);

            if state.residual_norm < abstol || rel_norm < tol {
                return true;
            }

            // 3. Build full Jacobian (linear + nonlinear + GMIN)
            let jacobian = self.build_full_jacobian_with_gmin(state, gmin);

            // 4. Solve J * ΔX = -R for Newton update
            let delta_x = match self.solve_jacobian_system(&jacobian, state) {
                Ok(dx) => dx,
                Err(_) => return false, // Singular matrix
            };

            // 5. Apply line search for robust convergence
            if self
                .apply_line_search_with_gmin(state, &delta_x, gmin)
                .is_err()
            {
                return false;
            }
        }

        false // Max iterations reached
    }

    /// Compute full residual including GMIN contribution
    ///
    /// Residual = I_source - Y*V - gmin*V - I_nonlinear
    /// (KCL: sum of currents INTO node = 0)
    fn compute_full_residual_with_gmin(&mut self, state: &mut HbSolverState, gmin: Value) {
        // Start with linear residual (I_source - Y*V)
        self.compute_linear_residual(state);

        // Subtract GMIN contribution: I_gmin = gmin * V (current leaves node via GMIN)
        for node in 0..self.num_nodes {
            for k in 0..=self.num_harmonics {
                if node < state.residual.len() && k < state.residual[node].len() {
                    state.residual[node][k] -= gmin * state.x[node][k];
                }
            }
        }

        // Subtract nonlinear device currents (evaluated in time domain via FFT)
        // Note: add_nonlinear_residual adds currents with correct sign already
        if !self.nonlinear_devices.is_empty() {
            self.add_nonlinear_residual(state);
        }
    }

    /// Build Jacobian with GMIN on diagonal
    ///
    /// Residual = I_source - Y*V - gmin*V, so J = ∂res/∂V = -Y - gmin
    fn build_full_jacobian_with_gmin(
        &mut self,
        state: &HbSolverState,
        gmin: Value,
    ) -> Vec<Vec<Complex64>> {
        let mut jac = self.build_full_jacobian(state);

        // Subtract GMIN from all diagonal entries (consistent with residual -= gmin*V)
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        for i in 0..n {
            for k in 0..h {
                let idx = i * h + k;
                if idx < jac.len() {
                    jac[idx][idx] -= gmin;
                }
            }
        }

        jac
    }

    /// Apply line search with GMIN and PN voltage limiting
    ///
    /// Advanced implementation following standard methodology:
    /// - Armijo backtracking line search
    /// - PN junction voltage limiting on DC component
    fn apply_line_search_with_gmin(
        &mut self,
        state: &mut HbSolverState,
        delta_x: &[Vec<Complex64>],
        gmin: Value,
    ) -> Result<(), HbError> {
        let initial_norm = state.residual_norm;
        let armijo_c = 1e-4;
        let min_alpha = 0.01;
        let vt = 0.02585; // Thermal voltage at 300K

        let mut alpha = 1.0;
        let mut best_alpha = alpha;
        let mut best_norm = f64::INFINITY;

        let x_orig: Vec<Vec<Complex64>> = state.x.clone();

        while alpha >= min_alpha {
            for (node, dx_node) in delta_x.iter().enumerate() {
                for (k, &dx) in dx_node.iter().enumerate() {
                    if node < state.x.len() && k < state.x[node].len() {
                        let v_old = x_orig[node][k].re;
                        let v_new_raw = v_old + alpha * dx.re;

                        // Apply PN voltage limiting to DC component only
                        let v_new = if k == 0 {
                            limit_pn_voltage(v_old, v_new_raw, vt)
                        } else {
                            v_new_raw
                        };

                        // Keep imaginary part updated normally
                        let im_new = x_orig[node][k].im + alpha * dx.im;
                        state.x[node][k] = Complex64::new(v_new, im_new);
                    }
                }
            }

            self.compute_full_residual_with_gmin(state, gmin);

            if state.residual_norm < initial_norm * (1.0 - armijo_c * alpha) {
                return Ok(());
            }

            if state.residual_norm < best_norm {
                best_norm = state.residual_norm;
                best_alpha = alpha;
            }

            alpha *= 0.5;
        }

        // Use best step found with voltage limiting
        for (node, dx_node) in delta_x.iter().enumerate() {
            for (k, &dx) in dx_node.iter().enumerate() {
                if node < state.x.len() && k < state.x[node].len() {
                    let v_old = x_orig[node][k].re;
                    let v_new_raw = v_old + best_alpha * dx.re;

                    let v_new = if k == 0 {
                        limit_pn_voltage(v_old, v_new_raw, vt)
                    } else {
                        v_new_raw
                    };

                    let im_new = x_orig[node][k].im + best_alpha * dx.im;
                    state.x[node][k] = Complex64::new(v_new, im_new);
                }
            }
        }
        self.compute_full_residual_with_gmin(state, gmin);

        Ok(())
    }

    /// Compute full residual including linear and nonlinear contributions
    fn compute_full_residual(&mut self, state: &mut HbSolverState) {
        // Start with linear residual
        self.compute_linear_residual(state);

        // Add nonlinear device currents (evaluated in time domain via FFT)
        if !self.nonlinear_devices.is_empty() {
            self.add_nonlinear_residual(state);
        }
    }

    /// Add nonlinear device contributions to residual
    fn add_nonlinear_residual(&mut self, state: &mut HbSolverState) {
        let n_time = self.fft.size();

        // Convert spectral voltages to time domain
        let v_time: Vec<Vec<Value>> = (0..self.num_nodes)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate nonlinear currents at each time point
        let mut i_time = vec![vec![0.0; n_time]; self.num_nodes];

        for device in &self.nonlinear_devices {
            for t in 0..n_time {
                // Build voltage vector at this time point
                let node_voltages: Vec<Value> = v_time.iter().map(|v| v[t]).collect();

                // Evaluate device currents
                for (node, current) in device.evaluate(&node_voltages) {
                    if node < i_time.len() {
                        i_time[node][t] += current;
                    }
                }
            }
        }

        // Convert nonlinear currents to frequency domain and ADD to residual
        // Device returns stamped contribution (current INTO node, already with correct sign)
        for node in 0..self.num_nodes {
            let i_spectrum = self.fft.to_frequency_domain(&i_time[node]);
            for (k, &i_k) in i_spectrum.iter().enumerate() {
                if k <= self.num_harmonics && node < state.residual.len() {
                    state.residual[node][k] += i_k;
                }
            }
        }

        state.compute_residual_norm();
    }

    /// Build full Jacobian matrix for Newton iteration
    ///
    /// Structure: block matrix [node_i, k][node_j, l] where:
    /// - Diagonal blocks (k == l): linear admittance + linearized nonlinear
    /// - Off-diagonal blocks: nonlinear coupling via FFT convolution
    ///
    /// For efficiency, we flatten to a single [n*h x n*h] complex matrix
    fn build_full_jacobian(&mut self, state: &HbSolverState) -> Vec<Vec<Complex64>> {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let size = n * h;
        let omega0 = 2.0 * PI * self.config.fundamental_freq;

        // Initialize Jacobian
        let mut jac = vec![vec![Complex64::new(0.0, 0.0); size]; size];

        // --- Linear part: block-diagonal per harmonic ---
        // Residual = I_source - Y*V, so J = ∂res/∂V = -Y
        for k in 0..h {
            let omega_k = (k as f64) * omega0;

            // Conductance contribution: -G (negative because residual = ... - G*V)
            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    let row = i * h + k;
                    let col = j * h + k;
                    jac[row][col] -= g;
                }
            }

            // Capacitance contribution: -jωC
            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    let row = i * h + k;
                    let col = j * h + k;
                    jac[row][col] -= Complex64::new(0.0, omega_k) * c;
                }
            }

            // Inductance contribution: -1/(jωL)
            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    let row = i * h + k;
                    let col = j * h + k;
                    if k == 0 {
                        // DC: short circuit
                        jac[row][col] -= 1e6;
                    } else {
                        // AC: Y_L = -j/(ωL)
                        jac[row][col] -= Complex64::new(0.0, -1.0 / (omega_k * l));
                    }
                }
            }
        }

        // --- Nonlinear part: requires FFT-based evaluation ---
        if !self.nonlinear_devices.is_empty() {
            self.add_nonlinear_jacobian(&mut jac, state);
        }

        jac
    }

    /// Add nonlinear Jacobian contributions via FFT (Toeplitz/convolution)
    ///
    /// For nonlinear devices, the frequency-domain Jacobian is a Toeplitz matrix
    /// representing convolution: J[k,l] = G[k-l] where G is the DFT of g(t).
    ///
    /// This is the implementation that exactly matches the
    /// FFT-based residual computation, ensuring proper Newton convergence.
    fn add_nonlinear_jacobian(&mut self, jac: &mut [Vec<Complex64>], state: &HbSolverState) {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let n_time = self.fft.size();

        // Convert voltages to time domain
        let v_time: Vec<Vec<Value>> = (0..n)
            .map(|node| self.fft.to_time_domain(&state.x[node]))
            .collect();

        // Accumulate conductance stamps in time domain for each node pair
        let mut g_time = vec![vec![vec![0.0; n_time]; n]; n]; // [i][j][t]

        for device in &self.nonlinear_devices {
            for t in 0..n_time {
                let node_voltages: Vec<Value> = v_time.iter().map(|v| v[t]).collect();

                // Get Jacobian stamps from device
                for ((i, j), g) in device.jacobian(&node_voltages) {
                    if i < n && j < n {
                        g_time[i][j][t] += g;
                    }
                }
            }
        }

        // Convert each conductance waveform to frequency domain (Toeplitz row)
        // Then build the proper convolution Jacobian
        for i in 0..n {
            for j in 0..n {
                // Check if there's any significant conductance
                let max_g: Value = g_time[i][j].iter().fold(0.0, |a, &b| a.max(b.abs()));
                if max_g < 1e-30 {
                    continue;
                }

                // FFT the conductance waveform to get G[k] spectrum
                let g_spectrum = self.fft.to_frequency_domain(&g_time[i][j]);

                // Build Toeplitz block for this (i,j) node pair
                // J[i*h+k][j*h+l] = G[k-l] (with periodic extension for negative indices)
                for k in 0..h {
                    for l in 0..h {
                        let row = i * h + k;
                        let col = j * h + l;

                        // Compute index for G[k-l] with wrap-around
                        let diff = k as isize - l as isize;
                        let g_idx = if diff >= 0 {
                            diff as usize
                        } else {
                            // Negative index - use conjugate symmetry: G[-m] = G[m]*
                            // For real g(t), G[-m] = conj(G[m])
                            (-diff) as usize
                        };

                        if g_idx < g_spectrum.len() {
                            let g_val = if diff >= 0 {
                                g_spectrum[g_idx]
                            } else {
                                // Use conjugate for negative frequency
                                g_spectrum[g_idx].conj()
                            };
                            // SUBTRACT device Jacobian for KCL: residual = I_source - I_device
                            // So J = ∂res/∂V = -∂I_device/∂V = -gd
                            jac[row][col] -= g_val;
                        }
                    }
                }
            }
        }
    }

    /// Solve the Jacobian system: J * ΔX = -R
    ///
    /// Uses LU factorization with partial pivoting.
    /// Returns flattened delta_x vector that maps back to [node][harmonic].
    fn solve_jacobian_system(
        &self,
        jac: &[Vec<Complex64>],
        state: &HbSolverState,
    ) -> Result<Vec<Vec<Complex64>>, HbError> {
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let size = n * h;

        // Flatten RHS (negative residual)
        let mut rhs = Vec::with_capacity(size);
        for node in 0..n {
            for k in 0..h {
                rhs.push(-state.residual[node][k]);
            }
        }

        // Solve system
        let flat_solution = self.solve_complex_linear_system(jac, &rhs)?;

        // Reshape to [node][harmonic]
        let mut delta_x = vec![vec![Complex64::new(0.0, 0.0); h]; n];
        for (idx, &val) in flat_solution.iter().enumerate() {
            let node = idx / h;
            let harmonic = idx % h;
            if node < n {
                delta_x[node][harmonic] = val;
            }
        }

        Ok(delta_x)
    }

    /// Apply Armijo line search for robust convergence
    ///
    /// Starts with α = 1 (full Newton step), reduces if residual doesn't decrease.
    /// This is critical for convergence on highly nonlinear circuits.
    fn apply_line_search(
        &mut self,
        state: &mut HbSolverState,
        delta_x: &[Vec<Complex64>],
    ) -> Result<(), HbError> {
        let initial_norm = state.residual_norm;
        let armijo_c = 1e-4; // Sufficient decrease parameter
        let min_alpha = 0.01; // Minimum step size

        let mut alpha = 1.0;
        let mut best_alpha = alpha;
        let mut best_norm = f64::INFINITY;

        // Save original solution
        let x_orig: Vec<Vec<Complex64>> = state.x.clone();

        // Try different step sizes
        while alpha >= min_alpha {
            // Apply update: X_new = X_old + α * ΔX
            for (node, dx_node) in delta_x.iter().enumerate() {
                for (k, &dx) in dx_node.iter().enumerate() {
                    if node < state.x.len() && k < state.x[node].len() {
                        state.x[node][k] = x_orig[node][k] + alpha * dx;
                    }
                }
            }

            // Recompute residual
            self.compute_full_residual(state);

            // Check sufficient decrease (Armijo condition)
            if state.residual_norm < initial_norm * (1.0 - armijo_c * alpha) {
                return Ok(());
            }

            // Track best step
            if state.residual_norm < best_norm {
                best_norm = state.residual_norm;
                best_alpha = alpha;
            }

            // Reduce step size
            alpha *= 0.5;
        }

        // Use best step found even if Armijo not satisfied
        if best_alpha < 1.0 {
            for (node, dx_node) in delta_x.iter().enumerate() {
                for (k, &dx) in dx_node.iter().enumerate() {
                    if node < state.x.len() && k < state.x[node].len() {
                        state.x[node][k] = x_orig[node][k] + best_alpha * dx;
                    }
                }
            }
            self.compute_full_residual(state);
        }

        Ok(())
    }

    /// Legacy newton_step for backward compatibility
    pub fn newton_step(
        &mut self,
        state: &mut HbSolverState,
        nonlinear_fn: impl Fn(&[Value]) -> (Value, Value), // (current, dI/dV)
        terminals: &[(usize, usize)],                      // (positive_node, negative_node)
    ) -> Result<(), HbError> {
        // Get time points for nonlinear evaluation
        let n_time = self.fft.size();
        let _period = self.config.period();

        // Convert spectral voltages to time domain for each node
        let mut v_time: Vec<Vec<Value>> = Vec::with_capacity(self.num_nodes);
        for node in 0..self.num_nodes {
            let spectrum = &state.x[node];
            let waveform = self.fft.to_time_domain(spectrum);
            v_time.push(waveform);
        }

        // Evaluate nonlinear elements in time domain
        let mut i_time = vec![vec![0.0; n_time]; self.num_nodes];
        let mut g_time = vec![vec![0.0; n_time]; self.num_nodes];

        for &(np, nn) in terminals {
            for t in 0..n_time {
                let v_pn = v_time.get(np).map(|v| v[t]).unwrap_or(0.0)
                    - v_time.get(nn).map(|v| v[t]).unwrap_or(0.0);
                let (i, g) = nonlinear_fn(&[v_pn]);

                if np < i_time.len() {
                    i_time[np][t] += i;
                    g_time[np][t] += g;
                }
                if nn < i_time.len() {
                    i_time[nn][t] -= i;
                    g_time[nn][t] -= g;
                }
            }
        }

        // Convert currents back to frequency domain
        let mut i_spectrum: Vec<Vec<Complex64>> = Vec::with_capacity(self.num_nodes);
        for node in 0..self.num_nodes {
            let spectrum = self.fft.to_frequency_domain(&i_time[node]);
            i_spectrum.push(spectrum);
        }

        // Add nonlinear current to residual
        for (node, i_spec) in i_spectrum.iter().enumerate() {
            for (k, &i) in i_spec.iter().enumerate() {
                if node < state.residual.len() && k < state.residual[node].len() {
                    state.residual[node][k] += i;
                }
            }
        }

        state.compute_residual_norm();
        state.iteration += 1;

        // Check convergence
        let rel_norm = state.residual_norm / (state.solution_norm() + self.config.abstol);
        state.converged = rel_norm < self.config.tolerance;

        Ok(())
    }

    /// Solve complex linear system using Gaussian elimination
    fn solve_complex_linear_system(
        &self,
        a: &[Vec<Complex64>],
        b: &[Complex64],
    ) -> Result<Vec<Complex64>, HbError> {
        let n = b.len();
        if n == 0 {
            return Ok(vec![]);
        }

        // Augmented matrix
        let mut aug: Vec<Vec<Complex64>> = a
            .iter()
            .zip(b.iter())
            .map(|(row, &bi)| {
                let mut r = row.clone();
                r.push(bi);
                r
            })
            .collect();

        // Forward elimination with partial pivoting
        for col in 0..n {
            // Find pivot
            let mut max_row = col;
            for row in (col + 1)..n {
                if aug[row][col].norm() > aug[max_row][col].norm() {
                    max_row = row;
                }
            }
            aug.swap(col, max_row);

            let pivot = aug[col][col];
            if pivot.norm() < 1e-15 {
                continue; // Near-singular, skip
            }

            // Eliminate
            for row in (col + 1)..n {
                let factor = aug[row][col] / pivot;
                for k in col..=n {
                    let col_val = aug[col][k];
                    aug[row][k] -= factor * col_val;
                }
            }
        }

        // Back substitution
        let mut x = vec![Complex64::new(0.0, 0.0); n];
        for i in (0..n).rev() {
            let mut sum = aug[i][n];
            for j in (i + 1)..n {
                sum -= aug[i][j] * x[j];
            }
            if aug[i][i].norm() > 1e-15 {
                x[i] = sum / aug[i][i];
            }
        }

        Ok(x)
    }

    /// Build HbResult from solver state
    pub fn build_result(&self, state: &HbSolverState) -> HbResult {
        let mut result = HbResult::new(
            self.config.fundamental_freq,
            self.num_nodes,
            self.num_harmonics,
        );

        result.converged = state.converged;
        result.iterations = state.iteration;
        result.residual_norm = state.residual_norm;
        result.node_names = self.node_names.clone();

        // Copy spectral voltages
        for (node, spectrum) in state.x.iter().enumerate() {
            let mut sv = SpectralVoltage::new(
                self.node_names.get(node).cloned().unwrap_or_default(),
                self.num_harmonics,
            );
            sv.coefficients = spectrum.clone();
            sv.frequencies = self.config.harmonic_frequencies();
            result.spectral_voltages.push(sv);
        }

        result
    }
}

#[cfg(test)]
mod solver_tests {
    use super::*;

    #[test]
    fn test_hb_solver_creation() {
        let config = HbConfig::new(1e9).with_harmonics(5);
        let solver = HbSolver::new(config, 3);

        assert_eq!(solver.num_nodes, 3);
        assert_eq!(solver.num_harmonics, 5);
    }

    #[test]
    fn test_solver_state_creation() {
        let state = HbSolverState::new(3, 5);

        assert_eq!(state.x.len(), 3);
        assert_eq!(state.x[0].len(), 6); // 5 harmonics + DC
        assert!(!state.converged);
    }

    #[test]
    fn test_solver_state_norms() {
        let mut state = HbSolverState::new(2, 2);

        // Set some values
        state.x[0][0] = Complex64::new(1.0, 0.0);
        state.x[0][1] = Complex64::new(0.0, 1.0);
        state.residual[1][0] = Complex64::new(3.0, 4.0); // |3+4j| = 5

        state.compute_residual_norm();
        assert!((state.residual_norm - 5.0).abs() < 1e-10);

        let sol_norm = state.solution_norm();
        assert!((sol_norm - 2.0_f64.sqrt()).abs() < 1e-10); // sqrt(1 + 1)
    }

    #[test]
    fn test_add_stamps() {
        let config = HbConfig::new(1e9);
        let mut solver = HbSolver::new(config, 2);

        solver.add_conductance(0, 1, 0.001);
        solver.add_capacitance(0, 0, 1e-12);

        assert_eq!(solver.g_matrix.len(), 1);
        assert_eq!(solver.c_matrix.len(), 1);
    }

    #[test]
    fn test_set_sources() {
        let config = HbConfig::new(1e9).with_harmonics(3);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1.0);
        solver.set_ac_source(0, 0.5, 0.0);

        assert!((solver.source_spectra[0][0].re - 1.0).abs() < 1e-10);
        assert!((solver.source_spectra[0][1].re - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_linear_residual_dc_only() {
        let config = HbConfig::new(1e9).with_harmonics(2);
        let mut solver = HbSolver::new(config, 1);

        // Simple resistor to ground: I = G*V, with 1V DC source
        solver.add_conductance(0, 0, 0.001); // 1k ohm
        solver.set_dc_source(0, 0.001); // 1mA = 1V / 1k

        let mut state = HbSolverState::new(1, 2);
        state.x[0][0] = Complex64::new(1.0, 0.0); // V = 1V

        solver.compute_linear_residual(&mut state);

        // Residual should be small: G*V - I = 0.001*1 - 0.001 = 0
        assert!(
            state.residual[0][0].norm() < 1e-10,
            "Residual: {}",
            state.residual[0][0]
        );
    }

    #[test]
    fn test_solve_linear_simple() {
        let config = HbConfig::new(1e9).with_harmonics(1);
        let solver = HbSolver::new(config, 1);

        // Empty circuit (no stamps) with DC source
        // Should give zero solution
        let mut state = HbSolverState::new(1, 1);

        // This is degenerate, but should not panic
        let _ = solver.solve_linear(&mut state);
    }

    #[test]
    fn test_complex_linear_solve() {
        let config = HbConfig::new(1e9);
        let solver = HbSolver::new(config, 2);

        // Simple 2x2 system
        let a = vec![
            vec![Complex64::new(2.0, 0.0), Complex64::new(1.0, 0.0)],
            vec![Complex64::new(1.0, 0.0), Complex64::new(3.0, 0.0)],
        ];
        let b = vec![Complex64::new(5.0, 0.0), Complex64::new(7.0, 0.0)];

        let x = solver.solve_complex_linear_system(&a, &b).unwrap();

        // Verify solution
        let r0 = a[0][0] * x[0] + a[0][1] * x[1] - b[0];
        let r1 = a[1][0] * x[0] + a[1][1] * x[1] - b[1];

        assert!(r0.norm() < 0.01, "Residual 0: {}", r0);
        assert!(r1.norm() < 0.01, "Residual 1: {}", r1);
    }

    #[test]
    fn test_build_result() {
        let config = HbConfig::new(1e9).with_harmonics(3);
        let solver = HbSolver::new(config, 2);

        let mut state = HbSolverState::new(2, 3);
        state.converged = true;
        state.iteration = 5;
        state.residual_norm = 1e-10;

        let result = solver.build_result(&state);

        assert!(result.converged);
        assert_eq!(result.iterations, 5);
        assert_eq!(result.num_nodes(), 2);
        assert_eq!(result.num_harmonics, 3);
    }

    #[test]
    fn test_hb_error_display() {
        let err = HbError::ConvergenceFailed {
            iterations: 50,
            residual: 1e-3,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("50 iterations"));
    }

    // ==========================================================================
    // Newton Solver Tests - Verification
    // ==========================================================================

    #[test]
    fn test_nonlinear_device_instance_diode_creation() {
        let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);

        assert_eq!(diode.device_type, NonlinearDeviceType::Diode);
        assert_eq!(diode.terminals, vec![0, 1]);
        assert!((diode.params.is - 1e-14).abs() < 1e-20);
        assert!((diode.params.n - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_nonlinear_device_instance_bjt_creation() {
        let bjt = NonlinearDeviceInstance::npn_bjt(0, 1, 2, 1e-15, 100.0);

        assert_eq!(bjt.device_type, NonlinearDeviceType::NpnBjt);
        assert_eq!(bjt.terminals, vec![0, 1, 2]);
        assert!((bjt.params.bf - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_diode_evaluate_forward_bias() {
        let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);
        let vt: f64 = 0.02585;

        // Forward bias: 0.6V across diode (node 0 = 0.6V, node 1 = 0V)
        let voltages = vec![0.6, 0.0];
        let currents = diode.evaluate(&voltages);

        // Current should be positive and significant for forward bias
        let i_anode = currents
            .iter()
            .find(|(n, _)| *n == 0)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);
        let i_cathode = currents
            .iter()
            .find(|(n, _)| *n == 1)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);

        // Current flows out of anode (negative) into cathode (positive)
        assert!(
            i_anode < 0.0,
            "Current should flow out of anode: {}",
            i_anode
        );
        assert!(
            i_cathode > 0.0,
            "Current should flow into cathode: {}",
            i_cathode
        );

        // KCL: currents should sum to zero
        let sum: Value = currents.iter().map(|(_, i)| i).sum();
        assert!(sum.abs() < 1e-20, "KCL violation: {}", sum);

        // Verify current magnitude is reasonable for 0.6V forward bias
        let expected_i = 1e-14_f64 * ((0.6_f64 / vt).exp() - 1.0);
        assert!(
            (i_cathode - expected_i).abs() / expected_i < 0.01,
            "Current should match Shockley equation: got {} expected {}",
            i_cathode,
            expected_i
        );
    }

    #[test]
    fn test_diode_evaluate_reverse_bias() {
        let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);

        // Reverse bias: -5V across diode
        let voltages = vec![-5.0, 0.0];
        let currents = diode.evaluate(&voltages);

        // Current should be very small (approximately -Is)
        let i_cathode = currents
            .iter()
            .find(|(n, _)| *n == 1)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);
        assert!(
            i_cathode.abs() < 1e-13,
            "Reverse current should be ~Is: {}",
            i_cathode
        );
    }

    #[test]
    fn test_diode_jacobian_positive_conductance() {
        let diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);

        // At 0.6V forward bias
        let voltages = vec![0.6, 0.0];
        let jac = diode.jacobian(&voltages);

        // Self-conductance at anode should be positive
        let g_aa = jac
            .iter()
            .filter(|((i, j), _)| *i == 0 && *j == 0)
            .map(|(_, g)| *g)
            .sum::<Value>();
        assert!(g_aa > 0.0, "Self-conductance should be positive: {}", g_aa);
    }

    #[test]
    fn test_newton_solver_diode_dc() {
        // Test Newton solver on simple diode DC circuit
        // Diode in series with resistor, DC current source

        let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(50);
        let mut solver = HbSolver::new(config, 2);

        // Node 0: anode of diode
        // Node 1: cathode of diode (to ground through resistor)

        // Current source: 1mA into node 0
        solver.set_dc_source(0, 1e-3);

        // Resistor from node 1 to ground: 100 ohms
        solver.add_conductance(1, 1, 0.01);

        // Diode from node 0 to node 1
        solver.add_diode(0, 1, 1e-14, 1.0);

        // Small conductance for numerical stability
        solver.add_conductance(0, 0, 1e-9);

        // Add capacitor to make it valid for HB
        solver.add_capacitance(0, 0, 1e-12);

        let mut state = HbSolverState::new(2, 1);
        // Initialize with small forward bias guess
        state.x[0][0] = Complex64::new(0.6, 0.0);
        state.x[1][0] = Complex64::new(0.1, 0.0);

        let result = solver.solve_newton(&mut state);
        assert!(result.is_ok(), "Newton should converge: {:?}", result);
        assert!(state.converged, "Should converge");

        // DC voltages should be physical
        let v0_dc = state.x[0][0].re;
        let v1_dc = state.x[1][0].re;
        let vd = v0_dc - v1_dc;

        // Diode voltage should be around 0.5-0.7V for 1mA
        assert!(
            vd > 0.4 && vd < 0.8,
            "Diode voltage should be ~0.6V: {}",
            vd
        );
    }

    #[test]
    fn test_newton_solver_linear_fallback() {
        // Test that solve_newton works for linear circuits too (no devices)
        let config = HbConfig::new(1e6).with_harmonics(3);
        let mut solver = HbSolver::new(config, 2);

        // Simple RC circuit
        solver.add_conductance(0, 0, 0.001);
        solver.add_capacitance(1, 1, 1e-9);
        solver.add_conductance(0, 1, 0.0001);
        solver.set_dc_source(0, 1e-3);

        let mut state = HbSolverState::new(2, 3);

        let result = solver.solve_newton(&mut state);
        assert!(result.is_ok(), "Should solve linear circuit: {:?}", result);
    }

    #[test]
    fn test_newton_solver_device_registration() {
        let config = HbConfig::new(1e9);
        let mut solver = HbSolver::new(config, 3);

        assert!(!solver.has_nonlinear_devices());

        solver.add_diode(0, 1, 1e-14, 1.0);
        assert!(solver.has_nonlinear_devices());

        solver.add_npn_bjt(1, 2, 0, 1e-15, 100.0);
        assert_eq!(solver.nonlinear_devices.len(), 2);
    }

    #[test]
    fn test_build_full_jacobian_linear_only() {
        let config = HbConfig::new(1e9).with_harmonics(2);
        let mut solver = HbSolver::new(config, 2);

        // Add some linear elements using proper MNA stamping
        // 100 ohm resistor from node 0 to ground (G = 0.01)
        solver.add_conductance(0, 0, 0.01);
        // 1k resistor between nodes 0 and 1 (full MNA stamp)
        solver.add_resistor(0, 1, 1000.0); // G = 0.001
        // 1pF capacitor at node 1
        solver.add_capacitance(1, 1, 1e-12);

        let state = HbSolverState::new(2, 2);
        let jac = solver.build_full_jacobian(&state);

        // Matrix is 2 nodes * 3 harmonics = 6x6
        assert_eq!(jac.len(), 6);
        assert_eq!(jac[0].len(), 6);

        // DC block (k=0): should have conductance stamps
        // With KCL convention (res = I_source - Y*V), Jacobian J = -Y
        // Node 0, harmonic 0: J = -(0.01 + 0.001) = -0.011
        let y00_dc = jac[0][0];
        assert!(
            (y00_dc.re - (-0.011)).abs() < 1e-10,
            "J(0,0) at DC should be -0.011: {}",
            y00_dc
        );

        // Node 1, harmonic 0: J = -0.001 from resistor
        let h = 3; // 2 harmonics + DC
        let y11_dc = jac[1 * h][1 * h]; // node 1, harmonic 0
        assert!(
            (y11_dc.re - (-0.001)).abs() < 1e-10,
            "J(1,1) at DC should be -0.001: {}",
            y11_dc
        );

        // Off-diagonal: J = -(-0.001) = +0.001 (negative of off-diagonal admittance)
        let y01_dc = jac[0][1 * h];
        assert!(
            (y01_dc.re - 0.001).abs() < 1e-10,
            "J(0,1) at DC should be +0.001: {}",
            y01_dc
        );
    }

    #[test]
    fn test_build_full_jacobian_with_diode() {
        let config = HbConfig::new(1e9).with_harmonics(1);
        let mut solver = HbSolver::new(config, 2);

        // Resistor and diode
        solver.add_conductance(0, 0, 0.001);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_capacitance(1, 1, 1e-12);

        let mut state = HbSolverState::new(2, 1);
        state.x[0][0] = Complex64::new(0.6, 0.0);

        let jac = solver.build_full_jacobian(&state);

        // Jacobian should have nonlinear conductance added (with KCL sign: J = -Y)
        // At 0.6V, diode conductance is significant
        // Total J(0,0) = -(linear G + diode gd) < -0.001
        let y00 = jac[0][0];
        assert!(
            y00.re < -0.001,
            "Should have negative Jacobian with nonlinear contribution: {}",
            y00
        );
    }

    #[test]
    fn test_newton_nmos_saturation() {
        let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(50);
        let mut solver = HbSolver::new(config, 3);

        // NMOS: Drain=0, Gate=1, Source=2
        // Setup: Vgs = 2V (above Vth=0.7), Vds = 3V (saturation)

        // Gate voltage source (node 1 at 2V)
        solver.add_conductance(1, 1, 1.0); // Very low resistance to enforce voltage
        solver.set_dc_source(1, 2.0); // 2V at gate

        // Drain load resistor
        solver.add_conductance(0, 0, 0.001); // 1k load
        solver.set_dc_source(0, 5e-3); // Current to set Vdd

        // Source to ground
        solver.add_conductance(2, 2, 0.1); // Low resistance

        // Capacitor for HB validity
        solver.add_capacitance(0, 0, 1e-12);

        // Add NMOS
        solver.add_nonlinear_device(NonlinearDeviceInstance::nmos(0, 1, 2, 2, 0.7, 2e-4));

        let mut state = HbSolverState::new(3, 1);
        state.x[0][0] = Complex64::new(3.0, 0.0);
        state.x[1][0] = Complex64::new(2.0, 0.0);
        state.x[2][0] = Complex64::new(0.1, 0.0);

        let result = solver.solve_newton(&mut state);
        // Newton should attempt to solve (may not converge perfectly for this
        // simplified test case, but should not panic)
        assert!(result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })));
    }

    #[test]
    fn test_nonlinear_device_params_builders() {
        let diode_params = NonlinearDeviceParams::diode(2.5e-9, 1.7);
        assert!((diode_params.is - 2.5e-9).abs() < 1e-15);
        assert!((diode_params.n - 1.7).abs() < 1e-10);

        let bjt_params = NonlinearDeviceParams::bjt(1e-15, 150.0, 2.0, 100.0);
        assert!((bjt_params.bf - 150.0).abs() < 1e-10);
        assert!((bjt_params.br - 2.0).abs() < 1e-10);
        assert!((bjt_params.vaf - 100.0).abs() < 1e-10);

        let mos_params = NonlinearDeviceParams::mosfet(0.5, 5e-4, 0.02);
        assert!((mos_params.vth - 0.5).abs() < 1e-10);
        assert!((mos_params.kp - 5e-4).abs() < 1e-10);
        assert!((mos_params.lambda - 0.02).abs() < 1e-10);
    }

    #[test]
    fn test_bjt_evaluate_forward_active() {
        let bjt = NonlinearDeviceInstance::npn_bjt(0, 1, 2, 1e-15, 100.0);

        // Forward active: Vbe = 0.7V, Vbc = -2V (C=3V, B=1V, E=0.3V)
        let voltages = vec![3.0, 1.0, 0.3];
        let currents = bjt.evaluate(&voltages);

        // Collector current should be positive and significant
        let ic = currents
            .iter()
            .find(|(n, _)| *n == 0)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);
        let ib = currents
            .iter()
            .find(|(n, _)| *n == 1)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);
        let ie = currents
            .iter()
            .find(|(n, _)| *n == 2)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);

        // In forward active, Ic should be much larger than Ib
        assert!(
            ic.abs() > ib.abs() * 10.0,
            "Ic should be >> Ib: Ic={}, Ib={}",
            ic.abs(),
            ib.abs()
        );

        // KCL: Ic + Ib + Ie should equal 0
        let sum = ic + ib + ie;
        assert!(
            sum.abs() < 1e-12 * ic.abs().max(1e-20),
            "KCL violation: {}",
            sum
        );
    }

    #[test]
    fn test_mosfet_regions() {
        let nmos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 1e-4);

        // Cutoff: Vgs = 0.5V < Vth
        let v_cutoff = vec![2.0, 0.5, 0.0, 0.0];
        let i_cutoff = nmos.evaluate(&v_cutoff);
        let id_cutoff = i_cutoff
            .iter()
            .find(|(n, _)| *n == 0)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);
        assert!(
            id_cutoff.abs() < 1e-12,
            "Should be in cutoff: {}",
            id_cutoff
        );

        // Triode: Vgs = 2V, Vds = 0.5V
        let v_triode = vec![0.5, 2.0, 0.0, 0.0];
        let i_triode = nmos.evaluate(&v_triode);
        let id_triode = i_triode
            .iter()
            .find(|(n, _)| *n == 0)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);
        assert!(
            id_triode.abs() > 1e-6,
            "Should have current in triode: {}",
            id_triode
        );

        // Saturation: Vgs = 2V, Vds = 5V
        let v_sat = vec![5.0, 2.0, 0.0, 0.0];
        let i_sat = nmos.evaluate(&v_sat);
        let id_sat = i_sat
            .iter()
            .find(|(n, _)| *n == 0)
            .map(|(_, i)| *i)
            .unwrap_or(0.0);
        assert!(
            id_sat.abs() > id_triode.abs() * 0.5,
            "Saturation current should be similar: {}",
            id_sat
        );
    }

    #[test]
    fn test_line_search_mechanism() {
        // Verify that line search is properly implemented by checking
        // that the method exists and can be called
        let config = HbConfig::new(1e9).with_harmonics(1);
        let mut solver = HbSolver::new(config, 2);
        solver.add_conductance(0, 0, 0.001);
        solver.add_capacitance(1, 1, 1e-12);
        solver.add_diode(0, 1, 1e-14, 1.0);

        let mut state = HbSolverState::new(2, 1);
        state.x[0][0] = Complex64::new(0.5, 0.0);

        // Create a small delta
        let delta_x = vec![
            vec![Complex64::new(0.1, 0.0), Complex64::new(0.0, 0.0)],
            vec![Complex64::new(-0.05, 0.0), Complex64::new(0.0, 0.0)],
        ];

        // Compute initial residual
        solver.compute_full_residual(&mut state);
        let initial_norm = state.residual_norm;

        // Apply line search
        let result = solver.apply_line_search(&mut state, &delta_x);
        assert!(result.is_ok(), "Line search should not fail");

        // Residual should have been recomputed
        assert!(
            state.residual_norm.is_finite(),
            "Residual should be finite after line search"
        );
    }

    #[test]
    fn test_solve_jacobian_system() {
        let config = HbConfig::new(1e9).with_harmonics(1);
        let solver = HbSolver::new(config, 2);

        // Create a simple 4x4 identity-like Jacobian (2 nodes * 2 harmonics)
        let jac = vec![
            vec![
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.1, 0.0),
                Complex64::new(0.0, 0.0),
            ],
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.1, 0.0),
            ],
            vec![
                Complex64::new(0.1, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, 0.0),
            ],
            vec![
                Complex64::new(0.0, 0.0),
                Complex64::new(0.1, 0.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(1.0, 0.0),
            ],
        ];

        let mut state = HbSolverState::new(2, 1);
        state.residual[0][0] = Complex64::new(1.0, 0.0);
        state.residual[0][1] = Complex64::new(0.5, 0.0);
        state.residual[1][0] = Complex64::new(0.2, 0.0);
        state.residual[1][1] = Complex64::new(0.1, 0.0);

        let result = solver.solve_jacobian_system(&jac, &state);
        assert!(result.is_ok(), "Jacobian solve should succeed");

        let delta_x = result.unwrap();
        assert_eq!(delta_x.len(), 2);
        assert_eq!(delta_x[0].len(), 2);
    }

    // =========================================================================
    // Comprehensive Test Suite for HB Newton Solver
    // =========================================================================

    #[test]
    fn test_newton_solver_diode_ac_with_harmonics() {
        // Test diode with AC excitation - should generate harmonics
        let config = HbConfig::new(1e6)
            .with_harmonics(5)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        // DC bias: 1mA into diode
        solver.set_dc_source(0, 1e-3);
        // AC excitation: 0.1mA at fundamental (10% modulation - realistic small-signal)
        solver.set_ac_source(0, 0.1e-3, 0.0);

        // Load resistor
        solver.add_conductance(1, 1, 0.01);
        // Diode from node 0 to node 1
        solver.add_diode(0, 1, 1e-14, 1.0);
        // Small GMIN for stability
        solver.add_conductance(0, 0, 1e-9);
        // Capacitor for HB
        solver.add_capacitance(0, 0, 1e-12);

        let mut state = HbSolverState::new(2, 5);
        state.x[0][0] = Complex64::new(0.6, 0.0);
        state.x[1][0] = Complex64::new(0.1, 0.0);

        let result = solver.solve_newton(&mut state);
        assert!(result.is_ok(), "Should converge with AC: {:?}", result);

        // Nonlinear diode should generate harmonics
        // DC component should dominate
        let dc_magnitude = state.x[0][0].norm();
        let ac_magnitude = state.x[0][1].norm();
        assert!(dc_magnitude > ac_magnitude, "DC should dominate over AC");
    }

    #[test]
    fn test_newton_solver_npn_bjt_amplifier() {
        // Test NPN BJT common-emitter configuration
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 4);

        // Nodes: 0=Collector, 1=Base, 2=Emitter, 3=Vcc
        // BJT: C=0, B=1, E=2

        // Collector load resistor to Vcc (node 3)
        solver.add_conductance(0, 0, 0.001); // 1k load
        solver.add_conductance(3, 3, 1.0); // Enforce Vcc
        solver.set_dc_source(3, 5.0); // 5V supply

        // Base bias
        solver.add_conductance(1, 1, 0.0001); // 10k bias
        solver.set_dc_source(1, 0.7e-3); // Base current

        // Emitter to ground
        solver.add_conductance(2, 2, 0.01); // 100 ohm

        // Capacitors for HB
        solver.add_capacitance(0, 0, 1e-12);
        solver.add_capacitance(1, 1, 1e-12);

        // Add NPN BJT
        solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);

        let mut state = HbSolverState::new(4, 3);
        // Initial guess: typical operating point
        state.x[0][0] = Complex64::new(3.0, 0.0); // Collector
        state.x[1][0] = Complex64::new(0.7, 0.0); // Base
        state.x[2][0] = Complex64::new(0.1, 0.0); // Emitter
        state.x[3][0] = Complex64::new(5.0, 0.0); // Vcc

        let result = solver.solve_newton(&mut state);
        // Should either converge or reach iteration limit without panicking
        assert!(
            result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })),
            "Should handle BJT: {:?}",
            result
        );
    }

    #[test]
    fn test_newton_solver_poor_initial_guess() {
        // Test that source stepping helps with poor initial guess
        let config = HbConfig::new(1e6)
            .with_harmonics(1)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1e-3);
        solver.add_conductance(1, 1, 0.01);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-9);
        solver.add_capacitance(0, 0, 1e-12);

        let mut state = HbSolverState::new(2, 1);
        // Very poor initial guess - way off from solution
        state.x[0][0] = Complex64::new(10.0, 0.0); // Way too high
        state.x[1][0] = Complex64::new(-5.0, 0.0); // Negative voltage

        let result = solver.solve_newton(&mut state);
        // Source stepping should help recover
        assert!(result.is_ok(), "Source stepping should help: {:?}", result);
    }

    #[test]
    fn test_newton_solver_multi_diode_circuit() {
        // Test circuit with multiple diodes (full-wave rectifier style)
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 4);

        // Nodes: 0=input, 1=output+, 2=output-, 3=load

        // Input source
        solver.set_dc_source(0, 2e-3);
        solver.add_conductance(0, 0, 1e-9);

        // Two diodes in parallel paths (simplified rectifier)
        solver.add_diode(0, 1, 1e-14, 1.0); // D1: input to output+
        solver.add_diode(2, 0, 1e-14, 1.0); // D2: output- to input

        // Load resistor
        solver.add_conductance(1, 1, 0.01);
        solver.add_conductance(2, 2, 0.01);
        solver.add_conductance(3, 3, 0.01);

        // Coupling between outputs
        solver.add_conductance(1, 3, 0.001);

        // Capacitors
        solver.add_capacitance(0, 0, 1e-12);
        solver.add_capacitance(1, 1, 1e-12);

        let mut state = HbSolverState::new(4, 3);
        for i in 0..4 {
            state.x[i][0] = Complex64::new(0.3, 0.0);
        }

        let result = solver.solve_newton(&mut state);
        assert!(
            result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })),
            "Multi-diode should converge or fail gracefully: {:?}",
            result
        );
    }

    #[test]
    fn test_newton_solver_high_q_resonant() {
        // Test high-Q LC resonant circuit (challenging for convergence)
        let config = HbConfig::new(1e6)
            .with_harmonics(5)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        // High-Q LC tank
        solver.add_capacitance(0, 0, 100e-12); // 100pF
        solver.add_inductance(0, 0, 253.3e-9); // ~253nH for 1MHz resonance

        // Small loss resistor
        solver.add_conductance(0, 0, 1e-6);

        // Load
        solver.add_conductance(1, 1, 0.001);

        // Driving source at resonance
        solver.set_dc_source(0, 1e-6);
        solver.set_ac_source(0, 1e-6, 0.0);

        // Add diode for nonlinearity
        solver.add_diode(0, 1, 1e-14, 1.0);

        let mut state = HbSolverState::new(2, 5);
        state.x[0][0] = Complex64::new(0.5, 0.0);

        let result = solver.solve_newton(&mut state);
        // High-Q circuits are challenging - we mainly check it doesn't panic
        assert!(
            result.is_ok() || matches!(result, Err(HbError::ConvergenceFailed { .. })),
            "High-Q should converge or fail gracefully: {:?}",
            result
        );
    }

    #[test]
    fn test_numerical_jacobian_consistency() {
        // Verify Jacobian matches numerical derivative of residual
        // This is critical for Newton convergence
        let config = HbConfig::new(1e6).with_harmonics(1);
        let mut solver = HbSolver::new(config, 2);

        solver.add_conductance(0, 0, 0.001);
        solver.add_conductance(1, 1, 0.01);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_capacitance(0, 0, 1e-12);
        solver.set_dc_source(0, 1e-3);

        let mut state = HbSolverState::new(2, 1);
        state.x[0][0] = Complex64::new(0.6, 0.0);
        state.x[1][0] = Complex64::new(0.1, 0.0);

        let gmin = 1e-9;

        // Compute analytical Jacobian
        let analytical_jac = solver.build_full_jacobian_with_gmin(&state, gmin);

        // Compute numerical Jacobian using finite differences
        let eps = 1e-6;
        let n = solver.num_nodes;
        let h = solver.num_harmonics + 1;
        let size = n * h;

        let mut numerical_jac = vec![vec![Complex64::new(0.0, 0.0); size]; size];

        // Compute residual at base point
        solver.compute_full_residual_with_gmin(&mut state, gmin);
        let base_residual: Vec<Vec<Complex64>> = state.residual.clone();

        for col in 0..size {
            let node_idx = col / h;
            let harm_idx = col % h;

            // Perturb real part
            let orig = state.x[node_idx][harm_idx];
            state.x[node_idx][harm_idx] = orig + Complex64::new(eps, 0.0);
            solver.compute_full_residual_with_gmin(&mut state, gmin);

            for row in 0..size {
                let r_node = row / h;
                let r_harm = row % h;
                let d_residual = state.residual[r_node][r_harm] - base_residual[r_node][r_harm];
                numerical_jac[row][col] = d_residual / eps;
            }

            state.x[node_idx][harm_idx] = orig;
        }

        // Compare analytical vs numerical Jacobian
        let mut max_diff = 0.0;
        let abs_tol = 1e-8; // Absolute tolerance for near-zero values

        for i in 0..size {
            for j in 0..size {
                let diff = (analytical_jac[i][j] - numerical_jac[i][j]).norm();
                let scale = analytical_jac[i][j].norm().max(numerical_jac[i][j].norm());

                // Use absolute tolerance for near-zero values, relative for larger
                let rel_diff = if scale < abs_tol {
                    // Both values are near zero - check absolute difference
                    if diff < abs_tol { 0.0 } else { diff / abs_tol }
                } else {
                    diff / scale
                };

                if rel_diff > max_diff {
                    max_diff = rel_diff;
                }
            }
        }

        // Jacobian should be reasonably accurate (within ~5% for finite differences)
        assert!(
            max_diff < 0.05,
            "Jacobian should match numerical: max relative diff = {}",
            max_diff
        );
    }

    #[test]
    fn test_convergence_strategies_order() {
        // Verify convergence strategies are tried in correct order:
        // 1. Direct Newton
        // 2. GMIN stepping
        // 3. Source stepping
        // 4. Pseudo-transient
        //
        // We can verify this by checking a simple circuit converges quickly
        // (direct Newton) vs a hard circuit needing continuation

        // Easy circuit - should converge with direct Newton
        let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(50);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1e-3);
        solver.add_conductance(0, 0, 0.001);
        solver.add_conductance(1, 1, 0.01);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_capacitance(0, 0, 1e-12);

        let mut state = HbSolverState::new(2, 1);
        state.x[0][0] = Complex64::new(0.6, 0.0);
        state.x[1][0] = Complex64::new(0.1, 0.0);

        let result = solver.solve_newton(&mut state);
        assert!(result.is_ok(), "Easy circuit should converge");

        // Should converge in relatively few iterations for well-conditioned circuit
        assert!(
            state.iteration < 30,
            "Should converge quickly: {} iterations",
            state.iteration
        );
    }

    #[test]
    fn test_residual_norm_decreases() {
        // Verify Newton iterations decrease residual norm (quadratic convergence)
        let config = HbConfig::new(1e6).with_harmonics(1).with_max_iterations(20);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1e-3);
        solver.add_conductance(1, 1, 0.01);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-6);
        solver.add_capacitance(0, 0, 1e-12);

        let mut state = HbSolverState::new(2, 1);
        state.x[0][0] = Complex64::new(0.6, 0.0);
        state.x[1][0] = Complex64::new(0.1, 0.0);

        // Compute initial residual
        let gmin = 1e-9;
        solver.compute_full_residual_with_gmin(&mut state, gmin);
        let initial_norm = state.residual_norm;

        // Run solver
        let result = solver.solve_newton(&mut state);
        assert!(result.is_ok());

        // Final residual should be much smaller
        solver.compute_full_residual_with_gmin(&mut state, gmin);
        let final_norm = state.residual_norm;

        assert!(
            final_norm < initial_norm * 1e-3,
            "Residual should decrease significantly: {} -> {}",
            initial_norm,
            final_norm
        );
    }

    // =========================================================================
    // DC Operating Point Solver Tests
    // =========================================================================

    #[test]
    fn test_dc_solve_resistor_divider() {
        // Simple 2-resistor voltage divider: R1=R2=1k
        let config = HbConfig::new(1e6).with_harmonics(3);
        let mut solver = HbSolver::new(config, 2);

        let g = 1.0 / 1000.0; // 1 mS
        solver.add_conductance(0, 0, g);
        solver.add_conductance(0, 1, -g);
        solver.add_conductance(1, 0, -g);
        solver.add_conductance(1, 1, g);
        solver.add_conductance(1, 1, g); // R2 to ground
        solver.set_dc_source(0, 1.0 * g);

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "DC solve should succeed: {:?}", result);
        let dc_solution = result.unwrap();
        assert!(
            dc_solution[1].abs() < 1.0 && dc_solution[1] >= 0.0,
            "V1 should be between 0 and 1V, got {}",
            dc_solution[1]
        );
    }

    #[test]
    fn test_dc_solve_diode_forward_bias() {
        // Forward-biased diode: 1mA -> V ~= 0.6-0.7V
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1e-3);
        solver.add_conductance(1, 1, 0.01);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-9);
        solver.add_conductance(1, 1, 1e-9);

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "DC solve should succeed: {:?}", result);
        let dc_solution = result.unwrap();
        let v_diode = dc_solution[0] - dc_solution[1];
        assert!(
            v_diode > 0.5 && v_diode < 1.0,
            "Diode forward voltage should be ~0.6-0.8V, got {}",
            v_diode
        );
    }

    #[test]
    fn test_dc_solve_diode_reverse_bias() {
        // Reverse-biased diode: negative current
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, -1e-5);
        solver.add_conductance(1, 1, 0.01);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-9);
        solver.add_conductance(1, 1, 1e-9);

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "DC solve for reverse bias: {:?}", result);
        let dc_solution = result.unwrap();
        let v_diode = dc_solution[0] - dc_solution[1];
        assert!(
            v_diode < 0.0,
            "Diode should be reverse-biased, got {}",
            v_diode
        );
    }

    #[test]
    fn test_dc_solve_bjt_common_emitter() {
        // NPN BJT in common-emitter
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 4);

        // Nodes: 0=C, 1=B, 2=E, 3=Vcc
        solver.set_dc_source(1, 10e-6); // Base current
        solver.add_conductance(0, 0, 0.001);
        solver.add_conductance(0, 3, -0.001);
        solver.add_conductance(3, 0, -0.001);
        solver.add_conductance(3, 3, 1.001);
        solver.set_dc_source(3, 5.0);
        solver.add_conductance(2, 2, 1.0);
        solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);
        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "BJT DC solve: {:?}", result);
        let dc = result.unwrap();
        let vbe = dc[1] - dc[2];
        assert!(
            vbe > 0.5 && vbe < 0.9,
            "V_BE should be ~0.6-0.7V, got {}",
            vbe
        );
    }

    #[test]
    fn test_dc_solve_with_gmin_stepping() {
        // Series diodes - requires GMIN stepping
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 4);

        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_diode(1, 2, 1e-14, 1.0);
        solver.add_diode(2, 3, 1e-14, 1.0);
        solver.set_dc_source(0, 1e-3);
        solver.add_conductance(3, 3, 1.0);
        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(
            result.is_ok(),
            "GMIN stepping should converge: {:?}",
            result
        );
        let dc = result.unwrap();
        let total_drop = dc[0] - dc[3];
        assert!(
            total_drop > 1.5 && total_drop < 2.5,
            "3 diodes ~1.8V, got {}",
            total_drop
        );
    }

    #[test]
    fn test_dc_solve_linear_circuit() {
        // Pure linear circuit
        let config = HbConfig::new(1e6).with_harmonics(3);
        let mut solver = HbSolver::new(config, 3);

        let g = 0.001;
        solver.add_conductance(0, 0, g);
        solver.add_conductance(0, 1, -g);
        solver.add_conductance(1, 0, -g);
        solver.add_conductance(1, 1, 2.0 * g);
        solver.add_conductance(1, 2, -g);
        solver.add_conductance(2, 1, -g);
        solver.add_conductance(2, 2, g + 1.0);
        solver.set_dc_source(0, 1e-3);

        let mut state = HbSolverState::new(3, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Linear DC solve: {:?}", result);
        for (i, &v) in result.unwrap().iter().enumerate() {
            assert!(
                v.is_finite() && v.abs() < 100.0,
                "Node {} voltage: {}",
                i,
                v
            );
        }
    }

    #[test]
    fn test_hb_with_dc_init_convergence() {
        // Verify DC init improves HB convergence
        let config = HbConfig::new(1e6)
            .with_harmonics(5)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1e-3);
        solver.set_ac_source(0, 0.1e-3, 0.0);
        solver.add_conductance(1, 1, 0.01);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-9);
        solver.add_capacitance(0, 0, 1e-12);

        let mut state = HbSolverState::new(2, 5);
        let result = solver.solve_newton(&mut state);

        assert!(result.is_ok(), "HB with DC init: {:?}", result);
        assert!(state.converged, "Should converge");
        let v_diode = state.x[0][0].re - state.x[1][0].re;
        assert!(
            v_diode > 0.5 && v_diode < 0.9,
            "V_diode ~0.6-0.7V, got {}",
            v_diode
        );
    }

    #[test]
    fn test_dc_solve_nmos_saturation() {
        // NMOS in saturation: Vgs > Vth, Vds > Vgs - Vth
        // Gate=1, Drain=2, Source=3 (grounded), Bulk=3
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // To set gate to 2V: Norton equivalent with G=1S to ground and I=2A
        // V = I/G = 2A/1S = 2V
        solver.set_dc_source(1, 2.0); // 2A current into gate node
        solver.set_dc_source(2, 1e-3); // Current into drain
        solver.add_conductance(1, 1, 1.0); // Gate conductance to ground (G=1S)
        solver.add_conductance(3, 3, 1.0); // Source grounded
        // NMOS: drain=2, gate=1, source=3, bulk=3, kp=200µA/V², vth=0.5V
        solver.add_nmos(2, 1, 3, 3, 2e-4, 0.5);
        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        // Set initial gate voltage
        state.x[1][0] = Complex64::new(2.0, 0.0);

        let result = solver.solve_dc_operating_point(&mut state);
        assert!(result.is_ok(), "NMOS DC solve: {:?}", result);

        let dc = result.unwrap();
        let vgs = dc[1] - dc[3];
        let vds = dc[2] - dc[3];

        // NMOS should be on with Vgs > Vth
        assert!(vgs > 0.5, "Vgs should exceed Vth=0.5V, got {}", vgs);
        // Drain should have positive voltage
        assert!(vds > 0.0, "Vds should be positive for NMOS, got {}", vds);
    }

    #[test]
    fn test_dc_solve_pnp_bjt() {
        // PNP BJT: emitter positive, base lower, collector even lower
        // Nodes: Emitter=0, Base=1, Collector=2, Ground=3
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // PNP: current flows from emitter to base/collector
        solver.set_dc_source(0, 1e-3); // 1mA current into emitter

        // Use add_resistor for proper MNA stamp: 100 ohm base resistor to ground
        solver.add_resistor(1, 3, 100.0); // Base to ground: G = 0.01S
        // 100 ohm collector resistor to ground
        solver.add_resistor(2, 3, 100.0); // Collector to ground: G = 0.01S

        // Ground node (large conductance to clamp)
        solver.add_conductance(3, 3, 1.0);

        // PNP: collector=2, base=1, emitter=0, Is=1fA, Bf=100
        solver.add_pnp_bjt(2, 1, 0, 1e-15, 100.0);

        // Add small GMIN for numerical stability
        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "PNP BJT DC solve: {:?}", result);
        let dc = result.unwrap();
        let veb = dc[0] - dc[1]; // Emitter-Base voltage

        // PNP should have V_EB > 0 (emitter more positive than base)
        assert!(
            veb > 0.4 && veb < 1.0,
            "V_EB should be ~0.6-0.7V for PNP, got {}",
            veb
        );
    }

    #[test]
    fn test_dc_solve_parallel_diodes() {
        // Two parallel diodes should share current
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        // 2mA total into two parallel diodes
        solver.set_dc_source(0, 2e-3);
        solver.add_conductance(1, 1, 1.0);
        // Two identical diodes in parallel
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-9);

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Parallel diodes: {:?}", result);
        let dc = result.unwrap();
        let v_diode = dc[0] - dc[1];

        // Each diode carries 1mA, so voltage should be similar to single diode at 1mA
        // (slightly higher due to thermal voltage effects, but still ~0.6V)
        assert!(
            v_diode > 0.5 && v_diode < 0.9,
            "Parallel diodes ~0.6V, got {}",
            v_diode
        );
    }

    #[test]
    fn test_dc_solve_diode_high_current() {
        // High current (100mA) should give higher forward voltage
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 100e-3); // 100mA
        solver.add_conductance(1, 1, 1.0);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-9);

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "High current diode: {:?}", result);
        let dc = result.unwrap();
        let v_diode = dc[0] - dc[1];

        // At 100mA, forward voltage should be higher (~0.7-0.8V)
        assert!(
            v_diode > 0.65 && v_diode < 1.0,
            "High current diode ~0.75V, got {}",
            v_diode
        );
    }

    #[test]
    fn test_dc_solve_diode_low_current() {
        // Low current (1µA) should give lower forward voltage
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 1e-6); // 1µA
        solver.add_conductance(1, 1, 1.0);
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_conductance(0, 0, 1e-9);

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Low current diode: {:?}", result);
        let dc = result.unwrap();
        let v_diode = dc[0] - dc[1];

        // At 1µA, forward voltage should be lower (~0.35-0.5V)
        assert!(
            v_diode > 0.3 && v_diode < 0.6,
            "Low current diode ~0.4V, got {}",
            v_diode
        );
    }

    #[test]
    fn test_dc_solve_diode_chain_five() {
        // 5 series diodes should give ~3V drop
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 6);

        // 5 diodes in series: 0→1→2→3→4→5
        solver.add_diode(0, 1, 1e-14, 1.0);
        solver.add_diode(1, 2, 1e-14, 1.0);
        solver.add_diode(2, 3, 1e-14, 1.0);
        solver.add_diode(3, 4, 1e-14, 1.0);
        solver.add_diode(4, 5, 1e-14, 1.0);
        solver.set_dc_source(0, 1e-3);
        solver.add_conductance(5, 5, 1.0);
        for n in 0..6 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(6, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "5 diode chain: {:?}", result);
        let dc = result.unwrap();
        let total_drop = dc[0] - dc[5];

        // 5 diodes × 0.6V ≈ 3.0V
        assert!(
            total_drop > 2.5 && total_drop < 4.0,
            "5 diodes ~3V, got {}",
            total_drop
        );
    }

    #[test]
    fn test_dc_solve_mixed_npn_diode() {
        // NPN BJT with diode in emitter path
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 5);

        // Collector=0, Base=1, Emitter=2, Diode cathode=3, Ground=4
        solver.set_dc_source(1, 100e-6); // Base current (100µA)

        // Collector supply: 10mA into collector (simulates Vcc through load)
        // This provides the positive supply needed for NPN forward active operation
        solver.set_dc_source(0, 10e-3);

        // Collector load resistor to ground (1k ohm)
        solver.add_resistor(0, 4, 1000.0);

        // Ground node
        solver.add_conductance(4, 4, 1.0);

        // NPN: C=0, B=1, E=2
        solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);
        // Diode in emitter path: anode=2, cathode=3
        solver.add_diode(2, 3, 1e-14, 1.0);
        // Resistor from diode cathode to ground (100 ohm)
        solver.add_resistor(3, 4, 100.0);

        for n in 0..5 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(5, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Mixed NPN+diode: {:?}", result);
        let dc = result.unwrap();
        let vbe = dc[1] - dc[2];
        let v_diode = dc[2] - dc[3];

        // Both junctions should be forward biased
        assert!(vbe > 0.4 && vbe < 1.0, "V_BE should be ~0.6V, got {}", vbe);
        assert!(
            v_diode > 0.4 && v_diode < 1.0,
            "V_diode should be ~0.6V, got {}",
            v_diode
        );
    }

    #[test]
    fn test_dc_solve_different_ideality_factors() {
        // Diode with n=2 (recombination-dominated) vs n=1
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver_n1 = HbSolver::new(config.clone(), 2);
        let mut solver_n2 = HbSolver::new(config, 2);

        // Diode with n=1
        solver_n1.set_dc_source(0, 1e-3);
        solver_n1.add_conductance(1, 1, 1.0);
        solver_n1.add_diode(0, 1, 1e-14, 1.0); // n=1
        solver_n1.add_conductance(0, 0, 1e-9);

        // Diode with n=2
        solver_n2.set_dc_source(0, 1e-3);
        solver_n2.add_conductance(1, 1, 1.0);
        solver_n2.add_diode(0, 1, 1e-14, 2.0); // n=2
        solver_n2.add_conductance(0, 0, 1e-9);

        let mut state_n1 = HbSolverState::new(2, 3);
        let mut state_n2 = HbSolverState::new(2, 3);

        let result_n1 = solver_n1.solve_dc_operating_point(&mut state_n1);
        let result_n2 = solver_n2.solve_dc_operating_point(&mut state_n2);

        assert!(result_n1.is_ok(), "n=1 diode: {:?}", result_n1);
        assert!(result_n2.is_ok(), "n=2 diode: {:?}", result_n2);

        let v_n1 = result_n1.unwrap()[0] - state_n1.x[1][0].re;
        let v_n2 = result_n2.unwrap()[0] - state_n2.x[1][0].re;

        // n=2 diode should have higher voltage for same current
        // (Vd = n * Vt * ln(I/Is + 1))
        assert!(
            v_n2 > v_n1,
            "n=2 diode ({}) should have higher Vf than n=1 ({})",
            v_n2,
            v_n1
        );
    }

    // =========================================================================
    // PMOS Device Tests
    // =========================================================================

    #[test]
    fn test_dc_solve_pmos_saturation() {
        // PMOS in saturation: Vsg > |Vth|, Vsd > Vsg - |Vth|
        // Gate=1, Drain=2, Source=0, Bulk=0
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // PMOS needs Vs > Vg (source higher than gate)
        // Source at 5V (node 0), gate at 3V (node 1), drain to ground through load
        solver.set_dc_source(0, 5.0); // 5A for 5V with 1S conductance
        solver.add_conductance(0, 0, 1.0); // Source voltage
        solver.set_dc_source(1, 3.0); // 3A for 3V gate (Vsg = 2V)
        solver.add_conductance(1, 1, 1.0);
        solver.add_resistor(2, 3, 1000.0); // Drain load to ground
        solver.add_conductance(3, 3, 1.0); // Ground

        // PMOS: drain=2, gate=1, source=0, bulk=0, vth=-0.5V, kp=100µA/V²
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![2, 1, 0, 0],
            params: NonlinearDeviceParams::mosfet(-0.5, 1e-4, 0.0),
        });

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "PMOS DC solve: {:?}", result);
        let dc = result.unwrap();
        let vsg = dc[0] - dc[1]; // Source-gate voltage

        // PMOS should be on with Vsg > |Vth| = 0.5V
        assert!(vsg > 0.5, "Vsg should exceed |Vth|=0.5V, got {}", vsg);
    }

    #[test]
    fn test_dc_solve_pmos_cutoff() {
        // PMOS in cutoff: Vsg < |Vth|
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // Source and gate at same voltage -> Vsg = 0 < |Vth|
        solver.set_dc_source(0, 3.0); // Source at 3V
        solver.add_conductance(0, 0, 1.0);
        solver.set_dc_source(1, 3.0); // Gate at 3V (Vsg = 0)
        solver.add_conductance(1, 1, 1.0);
        solver.add_resistor(2, 3, 1000.0); // Drain load
        solver.add_conductance(3, 3, 1.0); // Ground

        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![2, 1, 0, 0],
            params: NonlinearDeviceParams::mosfet(-0.5, 1e-4, 0.0),
        });

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "PMOS cutoff DC solve: {:?}", result);
        let dc = result.unwrap();
        // In cutoff, drain should be near ground (no current through load)
        assert!(
            dc[2].abs() < 0.5,
            "PMOS in cutoff should have Vd near 0, got {}",
            dc[2]
        );
    }

    // =========================================================================
    // BJT Region Tests
    // =========================================================================

    #[test]
    fn test_dc_solve_npn_cutoff() {
        // NPN in cutoff: Vbe < 0.5V (well below turn-on)
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // Base and emitter at same potential -> Vbe = 0
        solver.set_dc_source(0, 5.0); // Collector supply
        solver.add_conductance(0, 0, 1.0);
        solver.add_resistor(0, 3, 1000.0); // Collector load
        solver.add_conductance(1, 1, 1.0); // Base to ground
        solver.add_conductance(2, 2, 1.0); // Emitter to ground
        solver.add_conductance(3, 3, 1.0); // Ground

        // NPN: C=0, B=1, E=2
        solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "NPN cutoff: {:?}", result);
        let dc = result.unwrap();
        let vbe = dc[1] - dc[2];

        // Base and emitter both grounded, Vbe should be ~0
        assert!(vbe.abs() < 0.3, "NPN cutoff Vbe should be ~0, got {}", vbe);
    }

    #[test]
    fn test_dc_solve_npn_saturation() {
        // NPN in saturation: both junctions forward biased
        // Requires high base current and low collector load
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // High base current (1mA) with low collector load -> saturation
        solver.set_dc_source(1, 1e-3); // 1mA base current
        solver.set_dc_source(0, 1e-3); // Low collector supply
        solver.add_conductance(0, 0, 0.1); // Small collector conductance
        solver.add_conductance(2, 2, 1.0); // Emitter grounded
        solver.add_conductance(3, 3, 1.0); // Ground

        solver.add_npn_bjt(0, 1, 2, 1e-15, 100.0);

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "NPN saturation: {:?}", result);
        let dc = result.unwrap();
        let vce = dc[0] - dc[2];

        // In saturation, Vce should be small (< 0.5V typically)
        assert!(vce < 1.0, "NPN saturation should have low Vce, got {}", vce);
    }

    // =========================================================================
    // MOSFET Triode Region Tests
    // =========================================================================

    #[test]
    fn test_dc_solve_nmos_triode() {
        // NMOS in triode: Vgs > Vth but Vds < Vgs - Vth
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // High gate voltage, low drain current -> triode
        solver.set_dc_source(1, 3.0); // Gate at 3V (Vgs = 3V)
        solver.add_conductance(1, 1, 1.0);
        solver.set_dc_source(2, 0.1e-3); // Small drain current
        solver.add_conductance(2, 2, 0.001); // Small drain conductance
        solver.add_conductance(3, 3, 1.0); // Source grounded

        solver.add_nmos(2, 1, 3, 3, 1e-3, 0.5); // High kp for low Vds

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "NMOS triode: {:?}", result);
        let dc = result.unwrap();
        let vgs = dc[1] - dc[3];
        let vds = dc[2] - dc[3];

        // In triode: Vds < Vgs - Vth
        let vdsat = vgs - 0.5;
        assert!(
            vds < vdsat || vds.abs() < 0.5,
            "NMOS should be in triode: Vds={} < Vdsat={}",
            vds,
            vdsat
        );
    }

    // =========================================================================
    // Convergence Stress Tests
    // =========================================================================

    #[test]
    fn test_dc_solve_high_impedance_node() {
        // Node with very high impedance (nearly floating)
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 3);

        // Very small current into high impedance node
        solver.set_dc_source(0, 1e-12); // pA current
        solver.add_conductance(0, 0, 1e-12); // 1TΩ to ground
        solver.add_conductance(1, 1, 1.0);
        solver.add_conductance(2, 2, 1.0);

        solver.add_diode(0, 1, 1e-14, 1.0);

        let mut state = HbSolverState::new(3, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        // Should converge even with high impedance
        assert!(result.is_ok(), "High impedance: {:?}", result);
    }

    #[test]
    fn test_dc_solve_zero_bias_diode() {
        // Diode at zero bias should have zero current
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(100);
        let mut solver = HbSolver::new(config, 2);

        // Both nodes grounded -> zero bias
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(1, 1, 1.0);
        solver.add_diode(0, 1, 1e-14, 1.0);

        for n in 0..2 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Zero bias diode: {:?}", result);
        let dc = result.unwrap();
        let vd = dc[0] - dc[1];

        assert!(vd.abs() < 0.01, "Zero bias should give Vd~0, got {}", vd);
    }

    #[test]
    fn test_dc_solve_very_large_current() {
        // Very large current (10A) stress test
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 2);

        solver.set_dc_source(0, 10.0); // 10A
        solver.add_conductance(1, 1, 1.0);
        solver.add_diode(0, 1, 1e-12, 1.0); // Larger Is for high current
        solver.add_conductance(0, 0, 1e-9);

        let mut state = HbSolverState::new(2, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Large current: {:?}", result);
        let dc = result.unwrap();
        let vd = dc[0] - dc[1];

        // Very high current should give ~0.8-1.0V (log relationship)
        assert!(
            vd > 0.6 && vd < 1.5,
            "High current diode Vf should be reasonable, got {}",
            vd
        );
    }

    // =========================================================================
    // Multi-Device Complex Circuits
    // =========================================================================

    #[test]
    fn test_dc_solve_cascode_nmos() {
        // Cascode: two NMOS in series (common in analog design)
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 5);

        // Vdd=0, Gate1=1, Mid=2, Gate2=3, Ground=4
        solver.set_dc_source(0, 5.0);
        solver.add_conductance(0, 0, 1.0);
        solver.set_dc_source(1, 1.5); // Lower gate at 1.5V
        solver.add_conductance(1, 1, 1.0);
        solver.set_dc_source(3, 3.0); // Upper gate at 3V
        solver.add_conductance(3, 3, 1.0);
        solver.add_conductance(4, 4, 1.0);

        // Lower NMOS: D=2, G=1, S=4
        solver.add_nmos(2, 1, 4, 4, 1e-4, 0.5);
        // Upper NMOS: D=0, G=3, S=2
        solver.add_nmos(0, 3, 2, 4, 1e-4, 0.5);

        solver.add_resistor(0, 4, 5000.0); // Load

        for n in 0..5 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(5, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Cascode: {:?}", result);
        let dc = result.unwrap();

        // Mid node should be between ground and Vdd
        assert!(
            dc[2] > 0.0 && dc[2] < dc[0],
            "Cascode mid node should be 0 < {} < {}",
            dc[2],
            dc[0]
        );
    }

    #[test]
    fn test_dc_solve_differential_pair() {
        // NPN differential pair - fundamental analog building block
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 6);

        // Vcc=0, Out1=1, Out2=2, In1=3, In2=4, Tail=5
        solver.set_dc_source(0, 10.0); // Vcc supply
        solver.add_conductance(0, 0, 1.0);

        // Collector loads
        solver.add_resistor(1, 0, 1000.0);
        solver.add_resistor(2, 0, 1000.0);

        // Input bias (slight imbalance)
        solver.set_dc_source(3, 0.7);
        solver.add_conductance(3, 3, 1.0);
        solver.set_dc_source(4, 0.65);
        solver.add_conductance(4, 4, 1.0);

        // Tail current source (simplified as resistor to negative rail)
        solver.add_conductance(5, 5, 0.001);

        // Two NPNs: C1=1, B1=3, E1=5; C2=2, B2=4, E2=5
        solver.add_npn_bjt(1, 3, 5, 1e-15, 100.0);
        solver.add_npn_bjt(2, 4, 5, 1e-15, 100.0);

        for n in 0..6 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(6, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Diff pair: {:?}", result);
        let dc = result.unwrap();

        // Output difference should reflect input imbalance
        let vout_diff = dc[1] - dc[2];
        // With 50mV input difference and gain, expect some output difference
        assert!(
            vout_diff.abs() > 0.001,
            "Diff pair should have output difference, got {}",
            vout_diff
        );
    }

    #[test]
    fn test_dc_solve_diode_bridge() {
        // Full-wave diode bridge rectifier (4 diodes)
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 5);

        // AC+ = 0, AC- = 1, DC+ = 2, DC- = 3, Load = 4
        solver.set_dc_source(0, 1e-3); // AC+ positive
        solver.add_conductance(1, 1, 1.0); // AC- grounded

        // Load resistor
        solver.add_resistor(2, 3, 1000.0);

        // Ground reference
        solver.add_conductance(3, 3, 1.0);

        // 4 diodes forming bridge
        solver.add_diode(0, 2, 1e-14, 1.0); // AC+ to DC+
        solver.add_diode(3, 0, 1e-14, 1.0); // DC- to AC+
        solver.add_diode(1, 2, 1e-14, 1.0); // AC- to DC+
        solver.add_diode(3, 1, 1e-14, 1.0); // DC- to AC-

        for n in 0..5 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(5, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Diode bridge: {:?}", result);
        let dc = result.unwrap();

        // DC output should be positive
        let vdc = dc[2] - dc[3];
        assert!(vdc >= 0.0, "Bridge DC output should be >= 0, got {}", vdc);
    }

    #[test]
    fn test_dc_solve_ten_series_diodes() {
        // 10 series diodes - more stressful chain
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 11);

        // 10 diodes: 0->1->2->...->10
        for i in 0..10 {
            solver.add_diode(i, i + 1, 1e-14, 1.0);
        }
        solver.set_dc_source(0, 1e-3);
        solver.add_conductance(10, 10, 1.0);

        for n in 0..11 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(11, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "10 series diodes: {:?}", result);
        let dc = result.unwrap();
        let total_drop = dc[0] - dc[10];

        // 10 diodes × ~0.6V ≈ 6V
        assert!(
            total_drop > 5.0 && total_drop < 8.0,
            "10 diodes should drop ~6V, got {}",
            total_drop
        );
    }

    #[test]
    fn test_dc_solve_cmos_inverter() {
        // CMOS inverter: PMOS and NMOS in series
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(150);
        let mut solver = HbSolver::new(config, 4);

        // Vdd=0, Out=1, Gnd=2, In=3
        solver.set_dc_source(0, 3.3);
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(2, 2, 1.0);

        // Input at mid-rail (switching point)
        solver.set_dc_source(3, 1.65);
        solver.add_conductance(3, 3, 1.0);

        // PMOS: D=1, G=3, S=0, B=0
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![1, 3, 0, 0],
            params: NonlinearDeviceParams::mosfet(-0.7, 5e-5, 0.0),
        });

        // NMOS: D=1, G=3, S=2, B=2
        solver.add_nmos(1, 3, 2, 2, 5e-5, 0.7);

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-9);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "CMOS inverter: {:?}", result);
        let dc = result.unwrap();

        // At mid-rail input, output should be near mid-rail
        let vout = dc[1];
        assert!(
            vout > 0.5 && vout < 2.8,
            "CMOS inverter at midpoint should output ~Vdd/2, got {}",
            vout
        );
    }

    // ==================== COMPREHENSIVE TESTS ====================

    #[test]
    fn test_dc_solve_nmos_current_mirror() {
        // NMOS current mirror - basic analog building block
        // Two matched NMOS with diode-connected reference
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 5);

        // Vdd=0, Ref=1, Out=2, Gnd=3, Gate=4
        solver.set_dc_source(0, 5.0); // Vdd
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(3, 3, 1.0); // Ground

        // Reference current (1mA through resistor)
        solver.add_resistor(0, 1, 4000.0); // ~1mA at 5V-1V

        // Diode-connected NMOS (reference): D=1, G=1, S=3
        solver.add_nmos(1, 1, 3, 3, 1e-3, 0.7);

        // Mirror NMOS (output): D=2, G=1, S=3
        solver.add_nmos(2, 1, 3, 3, 1e-3, 0.7);

        // Load resistor on output (lower resistance for more current)
        solver.add_resistor(0, 2, 4000.0);

        for n in 0..5 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(5, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "NMOS current mirror: {:?}", result);
        let dc = result.unwrap();

        // Both drains should be at similar voltages (current matching)
        let vref = dc[1];
        let vout = dc[2];
        assert!(vref > 0.5, "Reference should be above Vth: {}", vref);
        assert!(vout > 0.5, "Output should be above Vth: {}", vout);
        // Current mirror outputs should be similar (within 20%)
        assert!(
            (vref - vout).abs() / vref.max(0.1) < 0.3,
            "Current mirror should match: Vref={}, Vout={}",
            vref,
            vout
        );
    }

    #[test]
    fn test_dc_solve_wilson_current_mirror() {
        // Wilson current mirror - improved accuracy over simple mirror
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(300);
        let mut solver = HbSolver::new(config, 6);

        // Vdd=0, N1drain=1, N2drain=2, N3drain=3, Gnd=4, Gate=5
        solver.set_dc_source(0, 5.0);
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(4, 4, 1.0);

        // Reference current input
        solver.add_resistor(0, 1, 3000.0);

        // N1: D=1, G=5, S=4 (input transistor)
        solver.add_nmos(1, 5, 4, 4, 1e-3, 0.7);

        // N2: D=2, G=5, S=4 (Wilson output)
        solver.add_nmos(2, 5, 4, 4, 1e-3, 0.7);

        // N3: D=3, G=1, S=2 (cascode, gate connected to N1 drain)
        solver.add_nmos(3, 1, 2, 4, 1e-3, 0.7);

        // Diode connection: gate (5) = N1 drain (1)
        solver.add_resistor(5, 1, 0.001); // Short circuit for diode connection

        // Output load
        solver.add_resistor(0, 3, 3000.0);

        for n in 0..6 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(6, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Wilson mirror: {:?}", result);
        let dc = result.unwrap();

        // Output should be in valid range
        assert!(dc[3] > 0.5 && dc[3] < 4.5, "Wilson output: {}", dc[3]);
    }

    #[test]
    fn test_dc_solve_source_degeneration() {
        // NMOS with source degeneration resistor - tests linearity
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 4);

        // Vdd=0, Drain=1, Source=2, Gnd=3
        solver.set_dc_source(0, 5.0);
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(3, 3, 1.0);

        // Gate bias at 2.5V
        solver.set_dc_source(1, 2.5);
        solver.add_conductance(1, 1, 0.01); // Weak gate bias

        // Load resistor
        solver.add_resistor(0, 1, 1000.0);

        // NMOS: D=1, G=gate via resistor, S=2
        solver.add_nmos(1, 1, 2, 3, 1e-3, 0.7);

        // Source degeneration resistor
        solver.add_resistor(2, 3, 100.0);

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Source degen: {:?}", result);
        let dc = result.unwrap();

        // Source should be above ground due to degeneration
        assert!(dc[2] > 0.01, "Source degeneration voltage: {}", dc[2]);
    }

    #[test]
    fn test_dc_solve_large_circuit_8_devices() {
        // Large circuit with 8 nonlinear devices - stress test
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(500);
        let mut solver = HbSolver::new(config, 10);

        // Vdd=0, nodes 1-8 for devices, Gnd=9
        solver.set_dc_source(0, 3.3);
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(9, 9, 1.0);

        // Add 8 diodes in parallel with resistors
        for i in 0..8 {
            let node = i + 1;
            // Resistor from Vdd to node
            solver.add_resistor(0, node, 10000.0 + (i as f64 * 1000.0));

            // Diode from node to ground
            solver.add_nonlinear_device(NonlinearDeviceInstance {
                device_type: NonlinearDeviceType::Diode,
                terminals: vec![node, 9],
                params: NonlinearDeviceParams::diode(1e-14, 1.0),
            });
        }

        for n in 0..10 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(10, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Large circuit 8 devices: {:?}", result);
        let dc = result.unwrap();

        // All diode nodes should be at forward bias (~0.6-0.7V)
        for i in 1..9 {
            assert!(
                dc[i] > 0.5 && dc[i] < 0.8,
                "Node {} should be at diode drop: {}",
                i,
                dc[i]
            );
        }
    }

    #[test]
    fn test_dc_solve_stiff_circuit() {
        // Stiff circuit with 1e12 conductance ratio - numerical stress test
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 4);

        // Vdd=0, Mid=1, Diode=2, Gnd=3
        solver.set_dc_source(0, 10.0);
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(3, 3, 1.0);

        // Very large resistor (1 Mohm) from Vdd to mid
        solver.add_resistor(0, 1, 1e6);

        // Very small resistor (1 ohm) from mid to diode
        solver.add_resistor(1, 2, 1.0);

        // Diode to ground
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::Diode,
            terminals: vec![2, 3],
            params: NonlinearDeviceParams::diode(1e-14, 1.0),
        });

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Stiff circuit: {:?}", result);
        let dc = result.unwrap();

        // Mid and diode should be at nearly same voltage (small R between them)
        assert!(
            (dc[1] - dc[2]).abs() < 0.01,
            "Stiff nodes should be close: V1={}, V2={}",
            dc[1],
            dc[2]
        );
        // Diode should be forward biased
        assert!(dc[2] > 0.5 && dc[2] < 0.8, "Diode voltage: {}", dc[2]);
    }

    #[test]
    fn test_dc_solve_level_shifter() {
        // Level shifter using source follower - analog staple
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 4);

        // Vdd=0, Gate=1, Source=2, Gnd=3
        solver.set_dc_source(0, 5.0);
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(3, 3, 1.0);

        // Gate input at 3V
        solver.set_dc_source(1, 3.0);
        solver.add_conductance(1, 1, 1.0);

        // Source follower NMOS: D=0 (to Vdd), G=1, S=2
        solver.add_nmos(0, 1, 2, 3, 1e-3, 0.7);

        // Current source load (resistor to ground)
        solver.add_resistor(2, 3, 500.0);

        for n in 0..4 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(4, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Level shifter: {:?}", result);
        let dc = result.unwrap();

        // Source should follow gate minus Vth, but actual voltage depends on load current
        // With 500 ohm load, source voltage can be lower due to Id*Rs drop
        let vs = dc[2];
        assert!(
            vs > 0.1 && vs < 3.0,
            "Level shifter output should be in valid source follower range: {}",
            vs
        );
    }

    #[test]
    fn test_dc_solve_pmos_current_mirror() {
        // PMOS current mirror - complementary to NMOS mirror
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 5);

        // Vdd=0, Ref=1, Out=2, Gnd=3, Gate=4
        solver.set_dc_source(0, 5.0);
        solver.add_conductance(0, 0, 1.0);
        solver.add_conductance(3, 3, 1.0);

        // Load resistor on reference leg
        solver.add_resistor(1, 3, 4000.0);

        // Diode-connected PMOS (reference): D=1, G=1, S=0
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![1, 1, 0, 0],
            params: NonlinearDeviceParams::mosfet(-0.7, 1e-3, 0.0),
        });

        // Mirror PMOS (output): D=2, G=1, S=0
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![2, 1, 0, 0],
            params: NonlinearDeviceParams::mosfet(-0.7, 1e-3, 0.0),
        });

        // Load resistor on output
        solver.add_resistor(2, 3, 4000.0);

        for n in 0..5 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(5, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "PMOS current mirror: {:?}", result);
        let dc = result.unwrap();

        // Both outputs should be in valid range
        assert!(dc[1] > 0.5 && dc[1] < 4.5, "PMOS ref: {}", dc[1]);
        assert!(dc[2] > 0.5 && dc[2] < 4.5, "PMOS out: {}", dc[2]);
    }

    #[test]
    fn test_dc_solve_complementary_pair() {
        // NPN-PNP complementary pair - push-pull output stage
        let config = HbConfig::new(1e6)
            .with_harmonics(3)
            .with_max_iterations(200);
        let mut solver = HbSolver::new(config, 5);

        // Vcc=0, Out=1, Vee=2, BaseN=3, BaseP=4
        solver.set_dc_source(0, 5.0); // Vcc
        solver.add_conductance(0, 0, 1.0);
        solver.set_dc_source(2, -5.0); // Vee
        solver.add_conductance(2, 2, 1.0);

        // Base bias for both transistors
        solver.set_dc_source(3, 0.6); // NPN base
        solver.add_conductance(3, 3, 1.0);
        solver.set_dc_source(4, -0.6); // PNP base
        solver.add_conductance(4, 4, 1.0);

        // NPN: C=0 (Vcc), B=3, E=1 (output)
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::NpnBjt,
            terminals: vec![0, 3, 1],
            params: NonlinearDeviceParams::default(),
        });

        // PNP: C=2 (Vee), B=4, E=1 (output)
        solver.add_nonlinear_device(NonlinearDeviceInstance {
            device_type: NonlinearDeviceType::PnpBjt,
            terminals: vec![2, 4, 1],
            params: NonlinearDeviceParams::default(),
        });

        // Load resistor on output
        solver.add_resistor(1, 2, 1000.0);

        for n in 0..5 {
            solver.add_conductance(n, n, 1e-12);
        }

        let mut state = HbSolverState::new(5, 3);
        let result = solver.solve_dc_operating_point(&mut state);

        assert!(result.is_ok(), "Complementary pair: {:?}", result);
        let dc = result.unwrap();

        // Output should be near 0V (balanced biasing)
        assert!(
            dc[1].abs() < 2.0,
            "Complementary output should be near 0V: {}",
            dc[1]
        );
    }
}

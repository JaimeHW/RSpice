//! Harmonic Balance Newton Solver
//!
//! Core solver for Harmonic Balance analysis using Newton-Raphson iteration.
//! Solves the frequency-domain circuit equations: G*X + jω*C*X + F_NL(X) = I_S

use num_complex::Complex64;
use std::f64::consts::PI;

use super::config::HbConfig;
use super::fft::HbFft;
use super::result::{HbResult, SpectralVoltage};
#[cfg(feature = "veriloga")]
use crate::device::veriloga::VerilogADevice;
use crate::solver::convergence::{PseudoTransient, SourceStepper};
use crate::solver::limit_pn_voltage;
use crate::Value;

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
    /// AC harmonic spectrum entries `(harmonic_index, complex_voltage)`.
    ///
    /// Harmonic index `1` is the fundamental of the HB basis frequency.
    pub ac_harmonics: Vec<(usize, Complex64)>,
}

impl VoltageSourceBranch {
    /// Create new voltage source branch
    pub fn new(node_pos: usize, node_neg: usize, branch_idx: usize, dc_voltage: Value) -> Self {
        Self {
            node_pos,
            node_neg,
            branch_idx,
            dc_voltage,
            ac_harmonics: Vec::new(),
        }
    }

    /// Set AC parameters at the fundamental harmonic (k=1).
    pub fn with_ac(mut self, magnitude: Value, phase: Value) -> Self {
        self.set_harmonic_component(1, Complex64::from_polar(magnitude, phase));
        self
    }

    /// Set AC parameters for a specific harmonic.
    pub fn with_harmonic(mut self, harmonic: usize, magnitude: Value, phase: Value) -> Self {
        self.set_harmonic_component(harmonic, Complex64::from_polar(magnitude, phase));
        self
    }

    fn set_harmonic_component(&mut self, harmonic: usize, value: Complex64) {
        if harmonic == 0 {
            return;
        }
        if let Some((_, component)) = self
            .ac_harmonics
            .iter_mut()
            .find(|(index, _)| *index == harmonic)
        {
            *component = value;
        } else {
            self.ac_harmonics.push((harmonic, value));
        }
    }
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
    /// Each branch may define AC entries on arbitrary HB harmonics.
    voltage_source_branches: Vec<VoltageSourceBranch>,

    /// Node names
    node_names: Vec<String>,

    /// Current source spectra [node][harmonic]
    source_spectra: Vec<Vec<Complex64>>,

    /// Registered nonlinear devices for Newton iteration
    nonlinear_devices: Vec<NonlinearDeviceInstance>,
    /// Registered Verilog-A devices for Newton iteration.
    #[cfg(feature = "veriloga")]
    veriloga_nonlinear_devices: Vec<HbVerilogADevice>,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
struct HbVerilogADevice {
    device: VerilogADevice,
    rhs_rows: Vec<Vec<(usize, Value)>>,
    jacobian_locs: Vec<Vec<(Option<usize>, Option<usize>)>>,
}

#[cfg(feature = "veriloga")]
impl HbVerilogADevice {
    fn new(device: VerilogADevice) -> Self {
        let rhs_rows = device.mapped_rhs_rows();
        let jacobian_locs = device.mapped_jacobian_locations();
        Self {
            device,
            rhs_rows,
            jacobian_locs,
        }
    }
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
    /// Three-terminal N-channel JFET (drain, gate, source)
    Njfet,
    /// Three-terminal P-channel JFET (drain, gate, source)
    Pjfet,
    /// Four-terminal voltage-controlled switch (p, n, cp, cn)
    VoltageSwitch,
    /// Four-terminal current-controlled switch with sensed control voltage
    /// converted to current (p, n, cp, cn)
    CurrentSwitch,
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
    /// Switch ON resistance
    pub ron: Value,
    /// Switch OFF resistance
    pub roff: Value,
    /// Switch hysteresis voltage parameter (stored, currently not stateful in HB)
    pub vh: Value,
    /// Switch transition smoothness
    pub smooth: Value,
    /// Control conversion gain (e.g. sense conductance A/V)
    pub control_gain: Value,
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
            ron: 1.0,
            roff: 1e6,
            vh: 0.0,
            smooth: 0.1,
            control_gain: 1.0,
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

    /// Create JFET parameters
    pub fn jfet(vto: Value, beta: Value, lambda: Value) -> Self {
        Self {
            vth: vto,
            kp: beta,
            lambda,
            ..Default::default()
        }
    }

    /// Create voltage-controlled switch parameters
    pub fn voltage_switch(vt: Value, vh: Value, ron: Value, roff: Value, smooth: Value) -> Self {
        Self {
            vth: vt,
            vh: vh.abs(),
            ron: ron.max(1e-6),
            roff: roff.max(1e-6),
            smooth: smooth.max(1e-9),
            ..Default::default()
        }
    }

    /// Create current-controlled switch parameters.
    pub fn current_switch(
        it: Value,
        ih: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
        control_gain: Value,
    ) -> Self {
        Self {
            vth: it,
            vh: ih.abs(),
            ron: ron.max(1e-6),
            roff: roff.max(1e-6),
            smooth: smooth.max(1e-12),
            control_gain: control_gain.max(1e-18),
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

    /// Create a PMOS instance
    pub fn pmos(
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        vth: Value,
        kp: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![drain, gate, source, bulk],
            params: NonlinearDeviceParams::mosfet(vth, kp, 0.0),
        }
    }

    /// Create an N-channel JFET instance
    pub fn njfet(
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Njfet,
            terminals: vec![drain, gate, source],
            params: NonlinearDeviceParams::jfet(vto, beta, lambda),
        }
    }

    /// Create a P-channel JFET instance
    pub fn pjfet(
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Pjfet,
            terminals: vec![drain, gate, source],
            params: NonlinearDeviceParams::jfet(vto, beta, lambda),
        }
    }

    /// Create a voltage-controlled switch instance
    pub fn voltage_switch(
        node_pos: usize,
        node_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        vt: Value,
        vh: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::VoltageSwitch,
            terminals: vec![node_pos, node_neg, ctrl_pos, ctrl_neg],
            params: NonlinearDeviceParams::voltage_switch(vt, vh, ron, roff, smooth),
        }
    }

    /// Create a current-controlled switch instance.
    pub fn current_switch(
        node_pos: usize,
        node_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        it: Value,
        ih: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
        control_gain: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::CurrentSwitch,
            terminals: vec![node_pos, node_neg, ctrl_pos, ctrl_neg],
            params: NonlinearDeviceParams::current_switch(it, ih, ron, roff, smooth, control_gain),
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
            NonlinearDeviceType::Njfet => self.eval_njfet(node_voltages),
            NonlinearDeviceType::Pjfet => self.eval_pjfet(node_voltages),
            NonlinearDeviceType::VoltageSwitch => self.eval_voltage_switch(node_voltages),
            NonlinearDeviceType::CurrentSwitch => self.eval_current_switch(node_voltages),
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
            NonlinearDeviceType::Njfet => self.jac_njfet(node_voltages),
            NonlinearDeviceType::Pjfet => self.jac_pjfet(node_voltages),
            NonlinearDeviceType::VoltageSwitch => self.jac_voltage_switch(node_voltages),
            NonlinearDeviceType::CurrentSwitch => self.jac_current_switch(node_voltages),
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

    fn jfet_ids_gm_gds(&self, node_voltages: &[Value], polarity: Value) -> (Value, Value, Value) {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let vgs = v_g - v_s;
        let vds = v_d - v_s;
        let vgs_int = polarity * vgs;
        let vds_int = polarity * vds;
        let vto = self.params.vth;
        let beta = self.params.kp.max(1e-18);
        let lambda = self.params.lambda.max(0.0);
        let vgst = vgs_int - vto;

        let (ids_int, gm, gds) = if vgst <= 0.0 {
            (0.0, 0.0, 0.0)
        } else if vds_int < 0.0 {
            // Reverse operation: swap effective drain/source orientation.
            let vds_rev = -vds_int;
            let vgs_rev = vgs_int - vds_int;
            let vgst_rev = vgs_rev - vto;

            if vgst_rev <= 0.0 {
                (0.0, 0.0, 0.0)
            } else if vds_rev <= vgst_rev {
                let ids_fwd = beta
                    * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev)
                    * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vds_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * 2.0 * (vgst_rev - vds_rev) * (1.0 + lambda * vds_rev)
                    + beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            } else {
                let ids_fwd = beta * vgst_rev * vgst_rev * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vgst_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * vgst_rev * vgst_rev * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            }
        } else if vds_int <= vgst {
            let ids = beta * (2.0 * vgst * vds_int - vds_int * vds_int) * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vds_int * (1.0 + lambda * vds_int);
            let gds = beta * 2.0 * (vgst - vds_int) * (1.0 + lambda * vds_int)
                + beta * (2.0 * vgst * vds_int - vds_int * vds_int) * lambda;
            (ids, gm, gds)
        } else {
            let ids = beta * vgst * vgst * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vgst * (1.0 + lambda * vds_int);
            let gds = beta * vgst * vgst * lambda;
            (ids, gm, gds)
        };

        (polarity * ids_int, gm, gds.max(1e-12))
    }

    fn eval_njfet(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (id, _, _) = self.jfet_ids_gm_gds(node_voltages, 1.0);
        vec![
            (self.terminals[0], -id), // Drain current leaving
            (self.terminals[2], id),  // Source current entering
        ]
    }

    fn jac_njfet(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (_, gm, gds) = self.jfet_ids_gm_gds(node_voltages, 1.0);
        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];
        vec![
            ((d, d), gds),
            ((d, s), -(gds + gm)),
            ((s, d), -gds),
            ((s, s), gds),
            ((d, g), gm),
            ((s, g), -gm),
        ]
    }

    fn eval_pjfet(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (id, _, _) = self.jfet_ids_gm_gds(node_voltages, -1.0);
        vec![
            (self.terminals[0], -id), // Drain current leaving
            (self.terminals[2], id),  // Source current entering
        ]
    }

    fn jac_pjfet(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (_, gm, gds) = self.jfet_ids_gm_gds(node_voltages, -1.0);
        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];
        vec![
            ((d, d), gds),
            ((d, s), -(gds + gm)),
            ((s, d), -gds),
            ((s, s), gds),
            ((d, g), gm),
            ((s, g), -gm),
        ]
    }

    fn switch_conductance_and_derivative(&self, vctrl: Value) -> (Value, Value) {
        // HB uses a memoryless smooth switch characteristic. Hysteresis parameter
        // is retained in params for compatibility with shared model cards.
        let smooth = self.params.smooth.max(1e-9);
        let x = (vctrl - self.params.vth) / smooth;
        let tanh_x = x.tanh();
        let f = 0.5 * (1.0 - tanh_x);

        let ron = self.params.ron.max(1e-6);
        let roff = self.params.roff.max(ron);
        let log_ron = ron.ln();
        let log_roff = roff.ln();
        let dlog_r = log_roff - log_ron;
        let log_r = log_ron + dlog_r * f;
        let g = (-log_r).exp();

        let sech2 = 1.0 - tanh_x * tanh_x;
        let df_dvctrl = -0.5 * sech2 / smooth;
        let dlogr_dvctrl = dlog_r * df_dvctrl;
        let dg_dvctrl = -g * dlogr_dvctrl;

        (g.max(1e-12), dg_dvctrl)
    }

    fn eval_voltage_switch(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vctrl = vcp - vcn;
        let vmain = vp - vn;
        let (g, _) = self.switch_conductance_and_derivative(vctrl);
        let i = g * vmain;
        vec![(self.terminals[0], -i), (self.terminals[1], i)]
    }

    fn jac_voltage_switch(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vctrl = vcp - vcn;
        let vmain = vp - vn;
        let (g, dg_dvctrl) = self.switch_conductance_and_derivative(vctrl);
        let g_ctrl = dg_dvctrl * vmain;

        let p = self.terminals[0];
        let n = self.terminals[1];
        let cp = self.terminals[2];
        let cn = self.terminals[3];

        vec![
            ((p, p), g),
            ((p, n), -g),
            ((n, p), -g),
            ((n, n), g),
            ((p, cp), g_ctrl),
            ((p, cn), -g_ctrl),
            ((n, cp), -g_ctrl),
            ((n, cn), g_ctrl),
        ]
    }

    fn eval_current_switch(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vmain = vp - vn;
        let ictrl = self.params.control_gain * (vcp - vcn);
        let (g, _) = self.switch_conductance_and_derivative(ictrl);
        let i = g * vmain;
        vec![(self.terminals[0], -i), (self.terminals[1], i)]
    }

    fn jac_current_switch(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vmain = vp - vn;
        let ictrl = self.params.control_gain * (vcp - vcn);
        let (g, dg_dictrl) = self.switch_conductance_and_derivative(ictrl);
        let g_ctrl = dg_dictrl * self.params.control_gain * vmain;

        let p = self.terminals[0];
        let n = self.terminals[1];
        let cp = self.terminals[2];
        let cn = self.terminals[3];

        vec![
            ((p, p), g),
            ((p, n), -g),
            ((n, p), -g),
            ((n, n), g),
            ((p, cp), g_ctrl),
            ((p, cn), -g_ctrl),
            ((n, cp), -g_ctrl),
            ((n, cn), g_ctrl),
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
            #[cfg(feature = "veriloga")]
            veriloga_nonlinear_devices: Vec::new(),
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

    /// Add voltage source with arbitrary AC harmonic entries.
    pub fn add_voltage_source_branch_harmonics(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        dc_voltage: Value,
        harmonics: &[(usize, Value, Value)],
    ) -> usize {
        let branch_idx = self.num_branches;
        let mut branch = VoltageSourceBranch::new(node_pos, node_neg, branch_idx, dc_voltage);
        for (harmonic, magnitude, phase) in harmonics {
            branch = branch.with_harmonic(*harmonic, *magnitude, *phase);
        }
        self.voltage_source_branches.push(branch);
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

    /// Add DC source current contribution at a node
    pub fn add_dc_source(&mut self, node: usize, current: Value) {
        if node < self.source_spectra.len() {
            self.source_spectra[node][0] += Complex64::new(current, 0.0);
        }
    }

    /// Set AC source at a node (sinusoidal at fundamental)
    pub fn set_ac_source(&mut self, node: usize, magnitude: Value, phase: Value) {
        self.set_harmonic_source(node, 1, magnitude, phase);
    }

    /// Add AC source contribution at the fundamental harmonic for a node
    pub fn add_ac_source(&mut self, node: usize, magnitude: Value, phase: Value) {
        self.add_harmonic_source(node, 1, magnitude, phase);
    }

    /// Set AC source contribution at an arbitrary harmonic for a node.
    pub fn set_harmonic_source(
        &mut self,
        node: usize,
        harmonic: usize,
        magnitude: Value,
        phase: Value,
    ) {
        if node < self.source_spectra.len() && harmonic < self.source_spectra[node].len() {
            self.source_spectra[node][harmonic] = Complex64::from_polar(magnitude, phase);
        }
    }

    /// Add AC source contribution at an arbitrary harmonic for a node.
    pub fn add_harmonic_source(
        &mut self,
        node: usize,
        harmonic: usize,
        magnitude: Value,
        phase: Value,
    ) {
        if node < self.source_spectra.len() && harmonic < self.source_spectra[node].len() {
            self.source_spectra[node][harmonic] += Complex64::from_polar(magnitude, phase);
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

    fn voltage_source_value_at_harmonic(
        branch: &VoltageSourceBranch,
        harmonic: usize,
    ) -> Complex64 {
        if harmonic == 0 {
            Complex64::new(branch.dc_voltage, 0.0)
        } else {
            branch
                .ac_harmonics
                .iter()
                .find_map(|(index, value)| (*index == harmonic).then_some(*value))
                .unwrap_or_else(|| Complex64::new(0.0, 0.0))
        }
    }

    fn compute_linear_residual_with_branches(
        &self,
        state: &mut HbSolverState,
        branch_currents: &[Vec<Complex64>],
    ) -> Value {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let h = self.num_harmonics + 1;

        for node_res in &mut state.residual {
            for c in node_res.iter_mut() {
                *c = Complex64::new(0.0, 0.0);
            }
        }

        // Start with nodal current source spectra.
        for (node, source) in self.source_spectra.iter().enumerate() {
            if node < state.residual.len() {
                for (k, &s) in source.iter().enumerate() {
                    if k < state.residual[node].len() {
                        state.residual[node][k] += s;
                    }
                }
            }
        }

        // Subtract linear passive contributions.
        for &(i, j, g) in &self.g_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..h {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        state.residual[i][k] -= g * state.x[j][k];
                    }
                }
            }
        }
        for &(i, j, c) in &self.c_matrix {
            if i < state.x.len() && j < state.x.len() {
                for k in 0..h {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        let omega_k = (k as f64) * omega0;
                        state.residual[i][k] -= Complex64::new(0.0, omega_k) * c * state.x[j][k];
                    }
                }
            }
        }
        for &(i, j, l) in &self.l_matrix {
            if i < state.x.len() && j < state.x.len() && l.abs() > 1e-30 {
                for k in 0..h {
                    if k < state.x[j].len() && k < state.residual[i].len() {
                        if k == 0 {
                            const DC_SHORT_CONDUCTANCE: Value = 1e6;
                            state.residual[i][k] -= DC_SHORT_CONDUCTANCE * state.x[j][k];
                        } else {
                            let omega_k = (k as f64) * omega0;
                            let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                            state.residual[i][k] -= y_l * state.x[j][k];
                        }
                    }
                }
            }
        }

        // Subtract MNA branch current coupling in nodal equations.
        for branch in &self.voltage_source_branches {
            let Some(currents) = branch_currents.get(branch.branch_idx) else {
                continue;
            };
            for k in 0..h {
                let ib = currents.get(k).copied().unwrap_or_default();
                if branch.node_pos > 0 && branch.node_pos - 1 < state.residual.len() {
                    state.residual[branch.node_pos - 1][k] -= ib;
                }
                if branch.node_neg > 0 && branch.node_neg - 1 < state.residual.len() {
                    state.residual[branch.node_neg - 1][k] += ib;
                }
            }
        }

        let mut residual_sum: Value = state
            .residual
            .iter()
            .flat_map(|node| node.iter())
            .map(|c| c.norm_sqr())
            .sum();

        // Include branch KVL residuals in the overall convergence norm.
        for branch in &self.voltage_source_branches {
            for k in 0..h {
                let mut v_drop = Complex64::new(0.0, 0.0);
                if branch.node_pos > 0 && branch.node_pos - 1 < state.x.len() {
                    v_drop += state.x[branch.node_pos - 1][k];
                }
                if branch.node_neg > 0 && branch.node_neg - 1 < state.x.len() {
                    v_drop -= state.x[branch.node_neg - 1][k];
                }
                let source_v = Self::voltage_source_value_at_harmonic(branch, k);
                let branch_residual = source_v - v_drop;
                residual_sum += branch_residual.norm_sqr();
            }
        }

        residual_sum.sqrt()
    }

    /// Solve for linear circuit (direct solve for diagonal harmonic blocks).
    ///
    /// Builds Y = G + jωC + 1/(jωL) and augments with MNA branch equations for
    /// ideal voltage sources when present.
    pub fn solve_linear(&self, state: &mut HbSolverState) -> Result<(), HbError> {
        let omega0 = 2.0 * PI * self.config.fundamental_freq;
        let n = self.num_nodes;
        let h = self.num_harmonics + 1;
        let m = self.num_branches;
        let total_unknowns = n + m;

        let mut branch_currents = vec![vec![Complex64::new(0.0, 0.0); h]; m];

        // For each harmonic, solve an independent linear system.
        for k in 0..h {
            let omega_k = (k as f64) * omega0;
            let mut y_matrix = vec![vec![Complex64::new(0.0, 0.0); total_unknowns]; total_unknowns];
            let mut rhs = vec![Complex64::new(0.0, 0.0); total_unknowns];

            for &(i, j, g) in &self.g_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += g;
                }
            }

            for &(i, j, c) in &self.c_matrix {
                if i < n && j < n {
                    y_matrix[i][j] += Complex64::new(0.0, omega_k) * c;
                }
            }

            for &(i, j, l) in &self.l_matrix {
                if i < n && j < n && l.abs() > 1e-30 {
                    if k == 0 {
                        const DC_SHORT_CONDUCTANCE: Value = 1e6;
                        y_matrix[i][j] += DC_SHORT_CONDUCTANCE;
                    } else {
                        let y_l = Complex64::new(0.0, -1.0 / (omega_k * l));
                        y_matrix[i][j] += y_l;
                    }
                }
            }

            for node in 0..n {
                rhs[node] = self
                    .source_spectra
                    .get(node)
                    .and_then(|s| s.get(k))
                    .copied()
                    .unwrap_or_default();
            }

            // MNA branch equations for ideal voltage sources.
            for branch in &self.voltage_source_branches {
                let row = n + branch.branch_idx;
                if row >= total_unknowns {
                    continue;
                }

                if branch.node_pos > 0 && branch.node_pos - 1 < n {
                    let np = branch.node_pos - 1;
                    y_matrix[np][row] += Complex64::new(1.0, 0.0);
                    y_matrix[row][np] += Complex64::new(1.0, 0.0);
                }
                if branch.node_neg > 0 && branch.node_neg - 1 < n {
                    let nn = branch.node_neg - 1;
                    y_matrix[nn][row] -= Complex64::new(1.0, 0.0);
                    y_matrix[row][nn] -= Complex64::new(1.0, 0.0);
                }

                rhs[row] = Self::voltage_source_value_at_harmonic(branch, k);
            }

            let solution = self.solve_complex_linear_system(&y_matrix, &rhs)?;

            for node in 0..n {
                if node < state.x.len() && k < state.x[node].len() {
                    state.x[node][k] = solution[node];
                }
            }
            for branch_idx in 0..m {
                let col = n + branch_idx;
                if col < solution.len() && branch_idx < branch_currents.len() {
                    branch_currents[branch_idx][k] = solution[col];
                }
            }
        }

        state.residual_norm = if m == 0 {
            self.compute_linear_residual(state);
            state.residual_norm
        } else {
            self.compute_linear_residual_with_branches(state, &branch_currents)
        };
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

    /// Add a PMOS for Newton iteration
    pub fn add_pmos(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        kp: Value,
        vth: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::pmos(
            drain, gate, source, bulk, vth, kp,
        ));
    }

    /// Add an N-channel JFET for Newton iteration
    pub fn add_njfet(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::njfet(
            drain, gate, source, vto, beta, lambda,
        ));
    }

    /// Add a P-channel JFET for Newton iteration
    pub fn add_pjfet(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::pjfet(
            drain, gate, source, vto, beta, lambda,
        ));
    }

    /// Add a voltage-controlled switch for Newton iteration
    pub fn add_voltage_switch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        vt: Value,
        vh: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::voltage_switch(
            node_pos, node_neg, ctrl_pos, ctrl_neg, vt, vh, ron, roff, smooth,
        ));
    }

    /// Add a current-controlled switch for Newton iteration.
    pub fn add_current_switch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        it: Value,
        ih: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
        control_gain: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::current_switch(
            node_pos,
            node_neg,
            ctrl_pos,
            ctrl_neg,
            it,
            ih,
            ron,
            roff,
            smooth,
            control_gain,
        ));
    }

    /// Add a Verilog-A nonlinear device for Newton iteration.
    #[cfg(feature = "veriloga")]
    pub fn add_veriloga_device(&mut self, device: VerilogADevice) {
        self.veriloga_nonlinear_devices
            .push(HbVerilogADevice::new(device));
    }

    /// Check if circuit has nonlinear devices
    pub fn has_nonlinear_devices(&self) -> bool {
        if !self.nonlinear_devices.is_empty() {
            return true;
        }
        #[cfg(feature = "veriloga")]
        {
            if !self.veriloga_nonlinear_devices.is_empty() {
                return true;
            }
        }
        false
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
        if !self.has_nonlinear_devices() {
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
            .max_by(|a, b| {
                let a_val = if a.1.is_finite() {
                    *a.1
                } else {
                    f64::NEG_INFINITY
                };
                let b_val = if b.1.is_finite() {
                    *b.1
                } else {
                    f64::NEG_INFINITY
                };
                a_val.total_cmp(&b_val)
            })
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
        if !self.has_nonlinear_devices() {
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
        if self.has_nonlinear_devices() {
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
    #[cfg(test)]
    fn compute_full_residual(&mut self, state: &mut HbSolverState) {
        // Start with linear residual
        self.compute_linear_residual(state);

        // Add nonlinear device currents (evaluated in time domain via FFT)
        if self.has_nonlinear_devices() {
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

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for (node, current) in device.evaluate(&node_voltages) {
                        if node < i_time.len() {
                            i_time[node][t] += current;
                        }
                    }
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            let mut circuit_voltages = vec![0.0; self.num_nodes];
            for t in 0..n_time {
                for node in 0..self.num_nodes {
                    circuit_voltages[node] = v_time[node][t];
                }
                for device in &mut self.veriloga_nonlinear_devices {
                    device.device.update_all_voltages(&circuit_voltages);
                    let values = device.device.evaluate();
                    for (program_idx, value) in values.iter().enumerate() {
                        let Some(rows) = device.rhs_rows.get(program_idx) else {
                            continue;
                        };
                        for &(row, sign) in rows {
                            if row < self.num_nodes {
                                i_time[row][t] += sign * *value;
                            }
                        }
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
        if self.has_nonlinear_devices() {
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

        if !self.nonlinear_devices.is_empty() {
            let mut node_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    node_voltages[node] = v_time[node][t];
                }
                for device in &self.nonlinear_devices {
                    for ((i, j), g) in device.jacobian(&node_voltages) {
                        if i < n && j < n {
                            g_time[i][j][t] += g;
                        }
                    }
                }
            }
        }

        #[cfg(feature = "veriloga")]
        if !self.veriloga_nonlinear_devices.is_empty() {
            let mut circuit_voltages = vec![0.0; n];
            for t in 0..n_time {
                for node in 0..n {
                    circuit_voltages[node] = v_time[node][t];
                }
                for device in &mut self.veriloga_nonlinear_devices {
                    device.device.update_all_voltages(&circuit_voltages);
                    let jac_entries = device.device.compute_jacobian();
                    for entry in jac_entries {
                        let Some(prog_locs) = device.jacobian_locs.get(entry.program_idx) else {
                            continue;
                        };
                        let Some(&(row, col)) = prog_locs.get(entry.jacobian_idx) else {
                            continue;
                        };
                        if let (Some(i), Some(j)) = (row, col) {
                            if i < n && j < n {
                                g_time[i][j][t] += entry.value;
                            }
                        }
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
    #[cfg(test)]
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
mod solver_tests;

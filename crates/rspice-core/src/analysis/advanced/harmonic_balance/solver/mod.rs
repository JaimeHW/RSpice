//! Harmonic Balance Newton Solver
//!
//! Core solver for Harmonic Balance analysis using Newton-Raphson iteration.
//! Solves the frequency-domain circuit equations: G*X + jÏ‰*C*X + F_NL(X) = I_S

#![allow(clippy::needless_range_loop, clippy::too_many_arguments)]
use super::config::HbConfig;
use super::fft::HbFft;
use super::result::{HbResult, SpectralVoltage};
use crate::Value;
#[cfg(feature = "veriloga")]
use crate::device::veriloga::VerilogADevice;
use num_complex::Complex64;

mod dc;
mod devices;
mod krylov;
mod linear;
mod linear_algebra;
mod newton;
mod nonlinear_api;
mod periodic_ac;
mod result_builder;

pub use periodic_ac::{PeriodicAcExcitation, PeriodicNoiseSource};

/// Conductance used to model an inductor as a DC short across every solve
/// path (full-spectrum residual, Jacobian, DC seed, linear solve). One value
/// everywhere keeps the operating point and the seed solving the same
/// circuit.
pub(super) const DC_SHORT_CONDUCTANCE: Value = 1e6;

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
/// - Linear elements: R, C, L (with proper jÏ‰L admittance)
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
    /// Admittance Y = 1/(jÏ‰L) at each harmonic
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

/// Depletion-capacitance parameter set for one junction.
///
/// `cj0 = 0` disables the junction charge entirely; `fc` is the forward-bias
/// linearization knee (SPICE FC, default 0.5).
#[derive(Debug, Clone, Copy)]
pub struct DepletionCap {
    /// Zero-bias junction capacitance (F)
    pub cj0: Value,
    /// Built-in potential (V)
    pub vj: Value,
    /// Grading coefficient
    pub m: Value,
    /// Forward-bias depletion linearization coefficient
    pub fc: Value,
}

impl DepletionCap {
    /// A disabled junction (no charge).
    pub fn none() -> Self {
        Self {
            cj0: 0.0,
            vj: 1.0,
            m: 0.5,
            fc: 0.5,
        }
    }

    /// Junction parameters with SPICE-standard clamping.
    pub fn new(cj0: Value, vj: Value, m: Value, fc: Value) -> Self {
        Self {
            cj0: cj0.max(0.0),
            vj: vj.max(0.01),
            m: m.clamp(0.01, 0.95),
            fc: fc.clamp(0.0, 0.99),
        }
    }
}

impl Default for DepletionCap {
    fn default() -> Self {
        Self::none()
    }
}

/// Device parameters for nonlinear devices
#[derive(Debug, Clone)]
pub struct NonlinearDeviceParams {
    /// Saturation current (Is for diode/BJT)
    pub is: Value,
    /// Ideality factor (n for diode)
    pub n: Value,
    /// Forward emission coefficient (BJT B-E junction)
    pub nf: Value,
    /// Reverse emission coefficient (BJT B-C junction)
    pub nr: Value,
    /// Thermal voltage
    pub vt: Value,
    /// Forward beta (BJT)
    pub bf: Value,
    /// Reverse beta (BJT)
    pub br: Value,
    /// Threshold voltage (MOSFET)
    pub vth: Value,
    /// Transconductance parameter K = Î¼Cox W/L (MOSFET)
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
    /// Primary junction depletion capacitance (diode junction, BJT B-E,
    /// JFET G-S)
    pub cap_a: DepletionCap,
    /// Secondary junction depletion capacitance (BJT B-C, JFET G-D)
    pub cap_b: DepletionCap,
    /// Forward transit time: diode TT / BJT TF (diffusion charge tau_f * i_f)
    pub tt_f: Value,
    /// Reverse transit time: BJT TR (diffusion charge tau_r * i_r)
    pub tt_r: Value,
}

impl Default for NonlinearDeviceParams {
    fn default() -> Self {
        Self {
            is: 1e-14,
            n: 1.0,
            nf: 1.0,
            nr: 1.0,
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
            cap_a: DepletionCap::none(),
            cap_b: DepletionCap::none(),
            tt_f: 0.0,
            tt_r: 0.0,
        }
    }
}

impl HbSolver {}

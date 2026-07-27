//! Jiles-Atherton Hysteresis Model for Magnetic Cores
//!
//! Implements the industry-standard Jiles-Atherton (J-A) model for magnetic
//! hysteresis, essential for accurate simulation of:
//! - Transformer cores under varying load
//! - Inductor cores with DC bias
//! - Magnetic sensing elements
//! - Power electronics with significant core loss
//!
//! # Model Description
//!
//! The J-A model describes hysteresis through the physical mechanisms of:
//! 1. Domain wall motion (reversible and irreversible)
//! 2. Domain wall pinning at defects
//! 3. Anhysteretic (ideal) magnetization via Langevin function
//!
//! ## Governing Equations
//!
//! **Anhysteretic magnetization (Langevin function):**
//! ```text
//! Man = Ms * [coth(He/a) - a/He]
//! ```
//! where `He = H + αM` is the effective field.
//!
//! **Differential susceptibility:**
//! ```text
//! dM/dH = (1-c) * (Man - Mirr) / (k*δ - α*(Man - Mirr)) + c * dMan/dH
//! ```
//!
//! ## Parameters
//!
//! | Parameter | Description | Typical Range |
//! |-----------|-------------|---------------|
//! | Ms | Saturation magnetization | 1.0-2.0 T for iron |
//! | a | Domain wall density | 100-1000 A/m |
//! | k | Pinning site density | 100-500 A/m |
//! | c | Reversibility coefficient | 0.01-0.2 |
//! | α | Inter-domain coupling | 1e-5 to 1e-3 |
//!
//! # References
//!
//! - Jiles, D.C. and Atherton, D.L., "Theory of ferromagnetic hysteresis"
//!   Journal of Magnetism and Magnetic Materials, 1986
//! - Hauser, H., "Energetic model of ferromagnetic hysteresis"
//!   Journal of Applied Physics, 1994

use crate::device::traits::{
    DynamicDevice, MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice,
};
use crate::{Value, circuit::NodeId};
use std::f64::consts::PI;

//=============================================================================
// Jiles-Atherton Model Parameters
//=============================================================================

/// Parameters for the Jiles-Atherton hysteresis model
#[derive(Debug, Clone)]
pub struct JilesAthertonParams {
    /// Saturation magnetization Ms (T or A/m depending on convention)
    pub ms: Value,
    /// Domain wall density parameter 'a' (A/m)
    /// Controls the slope of the anhysteretic curve
    pub a: Value,
    /// Pinning site density parameter 'k' (A/m)  
    /// Controls hysteresis loop width - higher k = wider loop
    pub k: Value,
    /// Reversibility coefficient 'c' (dimensionless, 0-1)
    /// Fraction of reversible domain wall motion
    pub c: Value,
    /// Inter-domain coupling parameter 'α' (dimensionless)
    /// Represents mean field coupling between domains
    pub alpha: Value,
    /// Cross-sectional area of the core (m²)
    pub area: Value,
    /// Mean magnetic path length (m)
    pub length: Value,
    /// Number of turns in the winding
    pub n_turns: Value,
    /// Effective air gap (m), used by Xyce's nonlinear magnetic-core form.
    pub gap: Value,
    /// Select Xyce's nonlinear mutual-inductor constitutive law.
    pub xyce_core: bool,
    /// Select Xyce's LEVEL=2 Core state update.  LEVEL=1 and LEVEL=2 use
    /// different state equations in Xyce even though their constitutive
    /// expressions share the same anhysteretic curve.
    pub xyce_core_level2: bool,
    /// Xyce LEVEL=2 voltage scaling used by the smooth irreversible branch.
    pub delta_v: Value,
    /// Xyce LEVEL=2 voltage normalization.
    pub v_inf: Value,
    /// Xyce LEVEL=1 voltage scaling.
    pub delta_v_scaling: Value,
    /// Xyce CORE anhysteretic-curve modeling constant.
    pub beta_h: Value,
    /// Xyce CORE irreversible-domain modeling constant.
    pub beta_m: Value,
}

/// Pure endpoint evaluation of Xyce's nonlinear Core constitutive state.
///
/// The transient solver uses this value while assembling a Newton trial. It
/// deliberately contains no accepted-state mutation; the accepted step calls
/// `integrate_xyce_core` to commit the same constitutive evaluation.
#[derive(Debug, Clone, Copy)]
pub struct XyceCoreTrial {
    /// Endpoint magnetization in A/m.
    pub magnetization: Value,
    /// Differential constitutive factor P used by MutIndNonLin.
    pub p: Value,
    /// `1 + (1-gap/path) * P`, the normalized branch factor.
    pub mid: Value,
    /// Vacuum inductance multiplied by `mid`.
    pub effective_inductance: Value,
    /// Applied field H before optional gap/turning-point display filtering.
    pub applied_field: Value,
    /// Forward-Euler magnetic update produced by this Newton evaluation.
    pub magnetization_update: Value,
    /// Xyce's `latestMag` argument used while evaluating `P`.
    pub latest_magnetization: Value,
    /// LEVEL=1's hidden R variable at this trial endpoint.  LEVEL=2 does not
    /// have the hidden magnetic equations and leaves this at zero.
    pub level1_rate: Value,
    /// Residual of LEVEL=1's eliminated hidden M equation at this endpoint.
    /// A nonzero finite value represents an inexact Newton trial; the
    /// magnetic stamp uses its Schur-complement contribution so the global
    /// electrical solve can move the coupled variables together.
    pub level1_residual: Value,
    /// Residual of LEVEL=1's explicit hidden R equation. R is the physical
    /// derivative of the aggregate ampere-turn source.
    pub level1_rate_residual: Value,
}

impl Default for JilesAthertonParams {
    fn default() -> Self {
        Self {
            // Default values for typical power ferrite
            ms: 0.4,        // Saturation magnetization (T)
            a: 200.0,       // Domain wall density (A/m)
            k: 150.0,       // Pinning site density (A/m)
            c: 0.1,         // Reversibility coefficient
            alpha: 1e-4,    // Inter-domain coupling
            area: 1e-4,     // 1 cm² cross-section
            length: 0.1,    // 10 cm path length
            n_turns: 100.0, // 100 turns
            gap: 0.0,
            xyce_core: false,
            xyce_core_level2: false,
            delta_v: 0.1,
            v_inf: 1.0,
            delta_v_scaling: 1.0e3,
            beta_h: 0.0001,
            beta_m: 3.125e-5,
        }
    }
}

impl JilesAthertonParams {
    /// Create parameters for a typical power ferrite core
    pub fn power_ferrite() -> Self {
        Self {
            ms: 0.4,
            a: 200.0,
            k: 150.0,
            c: 0.1,
            alpha: 1e-4,
            area: 1e-4,
            length: 0.08,
            n_turns: 50.0,
            gap: 0.0,
            xyce_core: false,
            xyce_core_level2: false,
            delta_v: 0.1,
            v_inf: 1.0,
            delta_v_scaling: 1.0e3,
            beta_h: 0.0001,
            beta_m: 3.125e-5,
        }
    }

    /// Create parameters for silicon steel (transformer grade)
    pub fn silicon_steel() -> Self {
        Self {
            ms: 1.6,
            a: 500.0,
            k: 300.0,
            c: 0.05,
            alpha: 5e-4,
            area: 2e-4,
            length: 0.2,
            n_turns: 100.0,
            gap: 0.0,
            xyce_core: false,
            xyce_core_level2: false,
            delta_v: 0.1,
            v_inf: 1.0,
            delta_v_scaling: 1.0e3,
            beta_h: 0.0001,
            beta_m: 3.125e-5,
        }
    }

    /// Create parameters for nickel-iron (permalloy-like)
    pub fn permalloy() -> Self {
        Self {
            ms: 0.8,
            a: 50.0,
            k: 30.0,
            c: 0.2,
            alpha: 1e-5,
            area: 1e-4,
            length: 0.1,
            n_turns: 100.0,
            gap: 0.0,
            xyce_core: false,
            xyce_core_level2: false,
            delta_v: 0.1,
            v_inf: 1.0,
            delta_v_scaling: 1.0e3,
            beta_h: 0.0001,
            beta_m: 3.125e-5,
        }
    }

    /// Calculate reluctance factor for inductance calculation
    pub fn reluctance_factor(&self) -> Value {
        self.length / (self.area * self.n_turns * self.n_turns)
    }

    /// Calculate base inductance (at Ms = 0)
    /// L = μ₀ * N² * A / l
    pub fn base_inductance(&self) -> Value {
        const MU_0: Value = 4.0 * PI * 1e-7;
        MU_0 * self.n_turns * self.n_turns * self.area / self.length
    }
}

//=============================================================================
// Jiles-Atherton State
//=============================================================================

/// Internal state variables for J-A model
#[derive(Debug, Clone)]
struct JaState {
    /// Current magnetization M (A/m)
    m: Value,
    /// Previous magnetization
    m_prev: Value,
    /// Current magnetic field H (A/m)
    h: Value,
    /// Previous H field
    h_prev: Value,
    /// Current flux B (T)
    b: Value,
    /// Previous flux
    b_prev: Value,
    /// Direction of H change (1=increasing, -1=decreasing)
    delta: Value,
    /// Irreversible magnetization component
    m_irr: Value,
    /// Xyce's reported H field.  With a nonzero gap Xyce can suppress the
    /// magnetization contribution at a nonphysical turning point while the
    /// constitutive H remains gap-corrected.
    reported_h: Value,
    /// Largest accepted LEVEL=1 voltage drop used by DELVSCALING.
    max_voltage_drop: Value,
}

impl Default for JaState {
    fn default() -> Self {
        Self {
            m: 0.0,
            m_prev: 0.0,
            h: 0.0,
            h_prev: 0.0,
            b: 0.0,
            b_prev: 0.0,
            delta: 1.0,
            m_irr: 0.0,
            reported_h: 0.0,
            max_voltage_drop: 1.0e-10,
        }
    }
}

//=============================================================================
// Jiles-Atherton Inductor
//=============================================================================

/// Inductor with Jiles-Atherton hysteresis model
///
/// This provides an accurate B-H loop simulation including:
/// - Major and minor loops
/// - First magnetization curve
/// - Proper handling of field reversals
/// - Core loss estimation
#[derive(Debug, Clone)]
pub struct JilesAthertonInductor {
    /// Device instance name
    pub name: String,
    /// Positive terminal node
    pub node_pos: NodeId,
    /// Negative terminal node  
    pub node_neg: NodeId,
    /// Branch index for MNA current variable
    pub branch_index: Option<NodeId>,

    /// Model parameters
    params: JilesAthertonParams,

    /// Internal state
    state: JaState,

    /// Current through inductor
    current: Value,
    /// Previous current
    current_prev: Value,
    /// Flux linkage (Wb-turns)
    flux_linkage: Value,
    /// Previous flux linkage
    flux_linkage_prev: Value,
    /// Voltage across inductor
    voltage_prev: Value,
    /// Effective inductance at current operating point
    l_eff: Value,

    /// Core loss accumulator (W)
    core_loss: Value,

    /// Xyce MutIndNonLin's trial update carried between Newton evaluations.
    /// It is included in the nonlinear snapshot and rolled back with the
    /// rest of the circuit when a transient attempt is rejected.
    xyce_mag_update: Value,
    /// Most recent pure endpoint produced while stamping a Newton iterate.
    /// The endpoint is committed verbatim when that iterate is accepted; this
    /// preserves Xyce's ordering where P is evaluated with the carried update
    /// before the newly computed `MagVarUpdate` replaces it.
    xyce_trial: Option<(Value, Value, XyceCoreTrial)>,
    /// Normalized `mid(P)` from the last accepted Xyce Core F evaluation.
    /// Xyce advances `MagVar` after `P` has been evaluated, so recomputing
    /// this factor from the committed magnetization would not reproduce the
    /// static voltage history used by the next trapezoidal step.
    xyce_accepted_mid: Value,
    /// Running ten-step average of the accepted magnetization derivative used
    /// by Xyce's `updateSecondaryState` output-path hysteresis filter.
    xyce_dmdt_average: Value,
    /// Fixed-length history backing `xyce_dmdt_average`.  Xyce initializes
    /// this queue with ten zero entries, so the first accepted derivatives
    /// enter the average gradually rather than changing the reported H path
    /// immediately.
    xyce_dmdt_history: [Value; 10],
    /// Number of accepted derivatives inserted into the history queue.
    xyce_dmdt_history_len: usize,
    /// Ring-buffer cursor for the next derivative replacement.
    xyce_dmdt_history_cursor: usize,
    /// Accepted LEVEL=1 hidden R variable, in physical (unscaled) units.
    /// The reduced Rust stamp eliminates Xyce's R equation algebraically, so
    /// this history is retained on the constitutive device instead of in an
    /// extra MNA unknown.
    xyce_level1_rate: Value,
    /// Accepted LEVEL=1 constitutive P used by the previous hidden M equation.
    xyce_level1_p: Value,
}

impl JilesAthertonInductor {
    #[inline]
    fn xyce_endpoint_matches(lhs: Value, rhs: Value) -> bool {
        if lhs == rhs {
            return true;
        }
        if !lhs.is_finite() || !rhs.is_finite() {
            return false;
        }
        let scale = lhs.abs().max(rhs.abs()).max(1.0);
        (lhs - rhs).abs() <= 128.0 * Value::EPSILON * scale
    }

    /// Create a new Jiles-Atherton inductor
    pub fn new(name: String, node_pos: NodeId, node_neg: NodeId) -> Self {
        let params = JilesAthertonParams::default();
        let l_eff = params.base_inductance() * 1000.0; // Initial high permeability

        Self {
            name,
            node_pos,
            node_neg,
            branch_index: None,
            params,
            state: JaState::default(),
            current: 0.0,
            current_prev: 0.0,
            flux_linkage: 0.0,
            flux_linkage_prev: 0.0,
            voltage_prev: 0.0,
            l_eff,
            core_loss: 0.0,
            xyce_mag_update: 0.0,
            xyce_trial: None,
            xyce_accepted_mid: 1.0,
            xyce_dmdt_average: 0.0,
            xyce_dmdt_history: [0.0; 10],
            xyce_dmdt_history_len: 0,
            xyce_dmdt_history_cursor: 0,
            xyce_level1_rate: 0.0,
            xyce_level1_p: 0.0,
        }
    }

    /// Create with specific parameters
    pub fn with_params(mut self, params: JilesAthertonParams) -> Self {
        self.l_eff = params.base_inductance() * 1000.0;
        self.params = params;
        self
    }

    /// Set branch index for MNA
    pub fn set_branch_index(&mut self, index: NodeId) {
        self.branch_index = Some(index);
    }

    /// Whether this instance follows Xyce's nonlinear mutual-inductor Core
    /// contract.  The transient companion freezes this state during Newton
    /// trial evaluations and advances it only after an accepted step.
    pub fn is_xyce_core(&self) -> bool {
        self.params.xyce_core
    }

    /// Whether this core uses Xyce's LEVEL=2 state update.
    pub fn is_xyce_core_level2(&self) -> bool {
        self.params.xyce_core && self.params.xyce_core_level2
    }

    /// Set initial current
    pub fn set_initial_current(&mut self, current: Value) {
        self.current = current;
        self.current_prev = current;
        // Calculate initial H field and magnetization
        let h = current * self.params.n_turns / self.params.length;
        self.state.h = h;
        self.state.h_prev = h;
        // Start on anhysteretic curve
        self.state.m = self.anhysteretic(h);
        if self.params.xyce_core && self.params.gap > 0.0 {
            self.state.h = (current * self.params.n_turns - self.params.gap * self.state.m)
                / self.params.length;
        }
        self.state.reported_h = current * self.params.n_turns / self.params.length;
        self.state.max_voltage_drop = 1.0e-10;
        self.state.m_prev = self.state.m;
        self.state.m_irr = self.state.m;
        // Calculate B and flux linkage
        let output_h = if self.params.xyce_core {
            self.state.reported_h
        } else {
            h
        };
        self.state.b = Self::mu_0() * (output_h + self.state.m);
        self.state.b_prev = self.state.b;
        self.flux_linkage = self.state.b * self.params.area * self.params.n_turns;
        self.flux_linkage_prev = self.flux_linkage;
    }

    /// Permeability of free space
    const fn mu_0() -> Value {
        4.0 * PI * 1e-7
    }

    /// Calculate anhysteretic magnetization using modified Langevin function
    ///
    /// Man = Ms * L(He/a) where L(x) = coth(x) - 1/x (Langevin function)
    fn anhysteretic(&self, h: Value) -> Value {
        if self.params.xyce_core {
            return self.xyce_anhysteretic(h, self.state.m);
        }

        let he = h + self.params.alpha * self.state.m; // Effective field

        let x = he / self.params.a;

        if x.abs() < 1e-6 {
            // Taylor series for small x: L(x) ≈ x/3 - x³/45
            self.params.ms * (x / 3.0 - x.powi(3) / 45.0)
        } else {
            // Full Langevin function: coth(x) - 1/x
            let coth_x = 1.0 / x.tanh();
            self.params.ms * (coth_x - 1.0 / x)
        }
    }

    /// Calculate derivative of anhysteretic magnetization dMan/dH
    fn anhysteretic_derivative(&self, h: Value) -> Value {
        let he = h + self.params.alpha * self.state.m;

        if self.params.xyce_core {
            let heo = self.params.beta_h * self.params.a;
            let root = (heo * heo + he * he).sqrt();
            let denominator = self.params.a + root;
            return if denominator.is_finite() && denominator > 0.0 && root > 0.0 {
                self.params.ms * (self.params.a + heo * heo / root) / (denominator * denominator)
            } else {
                0.0
            };
        }

        let x = he / self.params.a;

        if x.abs() < 1e-6 {
            // Taylor series derivative: dL/dx ≈ 1/3 - x²/15
            self.params.ms / (3.0 * self.params.a)
        } else {
            // dL/dx = 1/x² - 1/sinh²(x)
            let sinh_x = x.sinh();
            let dl_dx = 1.0 / (x * x) - 1.0 / (sinh_x * sinh_x);
            self.params.ms * dl_dx / self.params.a
        }
    }

    /// Calculate differential susceptibility dM/dH using J-A equation
    ///
    /// dM/dH = (1-c) * (Man - Mirr) / (k*δ - α*(Man - Mirr)) + c * dMan/dH
    fn differential_susceptibility(&self, h: Value, delta: Value) -> Value {
        let man = self.anhysteretic(h);

        if self.params.xyce_core {
            return self.xyce_differential_susceptibility(h, self.state.m, delta);
        }

        let m_diff = man - self.state.m_irr;

        // Prevent negative denominator (ensures physical behavior)
        let denom = self.params.k * delta - self.params.alpha * m_diff;

        // Irreversible component
        let dm_dh_irr = if denom.abs() > 1e-12 && delta * m_diff >= 0.0 {
            (1.0 - self.params.c) * m_diff / denom
        } else {
            0.0
        };

        // Reversible component
        let dm_dh_rev = self.params.c * self.anhysteretic_derivative(h);

        dm_dh_irr + dm_dh_rev
    }

    /// Integrate magnetization for a change in H
    ///
    /// Uses 4th-order Runge-Kutta for accurate hysteresis tracking
    fn integrate_magnetization(&mut self, h_new: Value) {
        let h = self.state.h;
        let dh = h_new - h;

        if dh.abs() < 1e-12 {
            return;
        }

        // Update direction
        self.state.delta = dh.signum();

        // RK4 integration for accuracy.  The CORE constitutive derivative is
        // evaluated against each intermediate magnetization, rather than the
        // stale accepted value, so reversals follow Xyce's implicit M/R
        // trajectory instead of accumulating a branch-dependent offset.
        let k1 = self.core_or_ja_differential_susceptibility(h, self.state.m, self.state.delta);
        let m2 = self.state.m + 0.5 * dh * k1;
        let k2 = self.core_or_ja_differential_susceptibility(h + 0.5 * dh, m2, self.state.delta);
        let m3 = self.state.m + 0.5 * dh * k2;
        let k3 = self.core_or_ja_differential_susceptibility(h + 0.5 * dh, m3, self.state.delta);
        let m4 = self.state.m + dh * k3;
        let k4 = self.core_or_ja_differential_susceptibility(h_new, m4, self.state.delta);

        let dm = (dh / 6.0) * (k1 + 2.0 * k2 + 2.0 * k3 + k4);

        // Update magnetization
        self.state.m_prev = self.state.m;
        self.state.m += dm;

        // Clamp to physical limits
        self.state.m = self.state.m.clamp(-self.params.ms, self.params.ms);

        // Update irreversible component
        let man = self.anhysteretic(h_new);
        let m_diff = man - self.state.m_irr;
        if self.state.delta * m_diff >= 0.0 {
            self.state.m_irr += dm * (1.0 - self.params.c);
        }

        // Update H
        self.state.h_prev = self.state.h;
        self.state.h = h_new;

        // Calculate B = μ₀(H + M)
        self.state.b_prev = self.state.b;
        self.state.b = Self::mu_0() * (h_new + self.state.m);
    }

    /// Xyce's CORE anhysteretic magnetization evaluated for an explicit
    /// `(H, M)` pair.  Keeping the trial magnetization as an argument makes
    /// RK4 stages consistent with the constitutive differential equation.
    fn xyce_anhysteretic(&self, h: Value, m: Value) -> Value {
        let he = h + self.params.alpha * m;
        let heo = self.params.beta_h * self.params.a;
        let root = (heo * heo + he * he).sqrt();
        let denominator = self.params.a + root;
        if denominator.is_finite() && denominator > 0.0 {
            self.params.ms * he / denominator
        } else {
            0.0
        }
    }

    /// Xyce's CORE differential susceptibility for an explicit `(H, M)`
    /// trial state and signed field direction.  This compatibility wrapper is
    /// retained for the ordinary J-A path; the native Core update supplies
    /// Xyce's voltage-derived `tanh(qV)` factor directly.
    fn xyce_differential_susceptibility(&self, h: Value, m: Value, delta: Value) -> Value {
        self.xyce_core_p(h, m, delta.signum())
    }

    /// Evaluate Xyce's P constitutive factor for an explicit `(H, M)` state.
    /// `tanh_qv` is the voltage-smoothed irreversible direction term used by
    /// both MutIndNonLin levels.
    fn xyce_core_p(&self, h: Value, m: Value, tanh_qv: Value) -> Value {
        let man = self.xyce_anhysteretic(h, m);
        let he = h + self.params.alpha * m;
        let heo = self.params.beta_h * self.params.a;
        let root_he = (heo * heo + he * he).sqrt();
        let denominator = self.params.a + root_he;
        let man_prime = if denominator.is_finite() && denominator > 0.0 && root_he > 0.0 {
            self.params.ms * (self.params.a + heo * heo / root_he) / (denominator * denominator)
        } else {
            0.0
        };

        let del_m = man - m;
        let del_m0 = self.params.beta_m * self.params.ms;
        let root_m = (del_m0 * del_m0 + del_m * del_m).sqrt();
        let mirr_denominator = 2.0 * (self.params.k - self.params.alpha * root_m);
        let mirr_prime = if mirr_denominator.abs() > 1.0e-18 {
            (del_m * tanh_qv + root_m) / mirr_denominator
        } else {
            0.0
        };

        let gap_path = self.params.gap / self.params.length;
        let denominator = 1.0
            + (gap_path - self.params.alpha) * self.params.c * man_prime
            + gap_path * (1.0 - self.params.c) * mirr_prime;
        if denominator.abs() > 1.0e-18 {
            (self.params.c * man_prime + (1.0 - self.params.c) * mirr_prime) / denominator
        } else {
            0.0
        }
    }

    /// Convert the accepted voltage drop to Xyce's smooth irreversible branch
    /// direction.  LEVEL=1 uses the adaptive `DELVSCALING/maxVoltageDrop`
    /// form; LEVEL=2 uses its explicit `DELV/VINF` ratio.
    fn xyce_tanh_qv(&self, voltage: Value) -> Value {
        let qv = if self.params.xyce_core_level2 {
            let denominator = if self.params.v_inf.abs() > 1.0e-30 {
                self.params.v_inf
            } else {
                1.0
            };
            self.params.delta_v * voltage / denominator
        } else {
            let denominator = self.state.max_voltage_drop.max(1.0e-10);
            self.params.delta_v_scaling * voltage / denominator
        };
        if qv.abs() < 40.0 {
            qv.tanh()
        } else {
            qv.signum()
        }
    }

    /// Evaluate the accepted-step constitutive state for a Newton trial.
    ///
    /// Xyce's `MutIndNonLin` devices solve the magnetization update together
    /// with the electrical branch equation. Keeping this operation pure lets
    /// the transient assembler linearize that coupled equation without
    /// contaminating rejected Newton attempts.
    pub fn xyce_core_trial(&self, current: Value, voltage: Value) -> Option<XyceCoreTrial> {
        if !self.params.xyce_core {
            return None;
        }

        self.xyce_core_trial_with_update(current, voltage, self.xyce_mag_update)
    }

    /// Evaluate one source-ordered MutIndNonLin update using an explicit
    /// `MagVarUpdate` value.  Keeping the carried update as an argument makes
    /// trial assembly pure while allowing the transient assembler to commit
    /// the resulting update transactionally after the evaluation.
    pub(crate) fn xyce_core_trial_with_update(
        &self,
        current: Value,
        voltage: Value,
        mag_update: Value,
    ) -> Option<XyceCoreTrial> {
        if !self.params.xyce_core {
            return None;
        }

        let old_current = self.current;
        let delta_happ = (current - old_current) * self.params.n_turns / self.params.length;
        let happ = current * self.params.n_turns / self.params.length;
        self.xyce_core_trial_from_happ_with_update(happ, delta_happ, voltage, mag_update)
    }

    /// Evaluate the Core constitutive endpoint from an aggregate applied
    /// field.  Multi-winding Xyce devices form `Happ` from the sum of
    /// `turns[i] * I[i]`; keeping this primitive independent of one winding's
    /// turn count preserves the canonical shared-state equation.
    pub(crate) fn xyce_core_trial_from_happ_with_update(
        &self,
        happ: Value,
        delta_happ: Value,
        voltage: Value,
        mag_update: Value,
    ) -> Option<XyceCoreTrial> {
        if !self.params.xyce_core {
            return None;
        }

        let old_m = self.state.m;
        let tanh_qv = self.xyce_tanh_qv(voltage);
        let latest_m = old_m + mag_update;
        let h = happ - (self.params.gap / self.params.length) * latest_m;
        let p = self.xyce_core_p(h, latest_m, tanh_qv);
        let magnetization_update = if delta_happ.abs() > 1.0e-18 {
            p * delta_happ
        } else {
            0.0
        };
        let m_new = old_m + magnetization_update;
        let mid = 1.0 + (1.0 - self.params.gap / self.params.length) * p;
        let effective_inductance = self.params.base_inductance() * mid;
        Some(XyceCoreTrial {
            magnetization: m_new,
            p,
            mid,
            effective_inductance,
            applied_field: happ,
            magnetization_update,
            latest_magnetization: latest_m,
            level1_rate: 0.0,
            level1_residual: 0.0,
            level1_rate_residual: 0.0,
        })
    }

    /// Evaluate Xyce's LEVEL=1 hidden M/R equations after eliminating their
    /// two internal unknowns from the Rust MNA system.  MutIndNonLin solves
    ///
    ///     R = (S-Sprev)/dt                         (BE)
    ///     R = 2(S-Sprev)/dt - Rprev                (OneStep order 2)
    ///     M = Mprev + a (P(M) R + Pprev Rprev)     (OneStep order 2)
    ///
    /// where `S = sum(turns[i] * I[i])`, `a = dt/(2*Path)`, and the previous
    /// term is omitted for the order-one equation.  Solving the scalar M
    /// equation here preserves the exact constitutive endpoint while keeping
    /// the public branch matrix compact.
    pub(crate) fn xyce_core_level1_trial_from_happ(
        &self,
        happ: Value,
        delta_happ: Value,
        voltage: Value,
        dt: Value,
        one_step_order2: bool,
    ) -> Option<XyceCoreTrial> {
        if !self.params.xyce_core
            || self.params.xyce_core_level2
            || !dt.is_finite()
            || dt <= 0.0
            || !happ.is_finite()
            || !delta_happ.is_finite()
        {
            return None;
        }
        let gap_path = self.params.gap / self.params.length;
        if !gap_path.is_finite() || !self.params.length.is_finite() || self.params.length <= 0.0 {
            return None;
        }
        let rate_factor = if one_step_order2 { 2.0 } else { 1.0 };
        // `delta_happ` is ΔS/Path, while Xyce's hidden R variable is dS/dt.
        // Keep the stored rate in that physical source-equation coordinate;
        // the M equation then contributes dt/Path * P * R.
        let rate = rate_factor * delta_happ * self.params.length / dt
            - if one_step_order2 {
                self.xyce_level1_rate
            } else {
                0.0
            };
        let integration_scale = if one_step_order2 { 0.5 } else { 1.0 } * dt / self.params.length;
        let previous_product = if one_step_order2 {
            self.xyce_level1_p * self.xyce_level1_rate
        } else {
            0.0
        };
        let old_m = self.state.m;
        let carried = if self.xyce_mag_update.is_finite() {
            self.xyce_mag_update
        } else {
            0.0
        };
        let mut latest_m = old_m + carried;
        if !latest_m.is_finite() {
            // Preserve the Newton predictor carried from the previous
            // assembly.  The branch stamp may subsequently advance this
            // predictor by the hidden M-row Newton correction; resetting it
            // to the accepted state here would make the eliminated unknown
            // path-independent and prevent reversal through a constitutive
            // turning point.
            latest_m = old_m + carried;
        }

        let residual = |m: Value| -> Option<(Value, Value)> {
            let h = happ - gap_path * m;
            let p = self.xyce_core_p(h, m, self.xyce_tanh_qv(voltage));
            let d_mid_d_m = self.xyce_core_dmid_d_magnetization(happ, voltage, m)?;
            let d_p_d_m = if (1.0 - gap_path).abs() > 1.0e-18 {
                d_mid_d_m / (1.0 - gap_path)
            } else {
                0.0
            };
            let f = m - old_m - integration_scale * (p * rate + previous_product);
            let df = 1.0 - integration_scale * rate * d_p_d_m;
            Some((f, df))
        };

        let scale = self
            .params
            .ms
            .abs()
            .max(old_m.abs())
            .max(latest_m.abs())
            .max(1.0);
        let tolerance = 1.0e-12 * scale;
        // Once the compact stamp has taken a hidden M-row Newton correction,
        // keep that value as the explicit Newton iterate.  Re-running a
        // scalar damped solve at the same electrical point can jump back to a
        // different constitutive pole and erase the coupled-state progress.
        let predictor_only = carried.abs() > 1.0e-12;
        let mut converged = false;
        for _ in 0..(if predictor_only { 0 } else { 40 }) {
            let Some((f, df)) = residual(latest_m) else {
                return None;
            };
            if !f.is_finite() || !df.is_finite() || df.abs() <= 1.0e-18 {
                break;
            }
            if f.abs() <= tolerance {
                converged = true;
                break;
            }
            let step = f / df;
            let mut damping = 1.0;
            let current_abs = f.abs();
            let mut accepted = false;
            while damping >= 1.0 / 128.0 {
                let candidate = latest_m - damping * step;
                if !candidate.is_finite() {
                    damping *= 0.5;
                    continue;
                }
                let candidate_abs = residual(candidate)
                    .map(|(candidate_f, _)| candidate_f.abs())
                    .unwrap_or(Value::INFINITY);
                if candidate_abs <= current_abs || damping <= 1.0 / 128.0 {
                    latest_m = candidate;
                    accepted = true;
                    break;
                }
                damping *= 0.5;
            }
            if !accepted {
                break;
            }
        }
        if !converged {
            let Some((f, _)) = residual(latest_m) else {
                return None;
            };
            converged = f.is_finite() && f.abs() <= 1.0e-9 * scale;
        }
        if !converged {
            // The Jiles-Atherton implicit equation can pass through a very
            // flat constitutive turning point.  A damped Newton iteration can
            // then alternate across that point even though the physical
            // branch has a well-defined root farther along the continuation
            // path.  Bracket the root around the accepted magnetization and
            // finish with a safeguarded bisection so the hidden Xyce state is
            // solved rather than silently approximated.
            let Some((f_center, _)) = residual(old_m) else {
                return None;
            };
            if !predictor_only && f_center.is_finite() && f_center.abs() > tolerance {
                let mut bracket = None;
                // Continue from the accepted magnetization branch.  A
                // sign change across one of the Jiles--Atherton constitutive
                // poles is not a physical M-equation root; searching an
                // unrestricted interval can otherwise jump to the opposite
                // hysteresis branch in a single Newton trial.  Bound the
                // safeguarded search to the same finite excursion envelope
                // used by the transient step controller and search first in
                // the implicit predictor direction.
                let predicted_delta = -f_center;
                let branch_limit = (0.5 * scale).max(1.0);
                let span = branch_limit;
                let preferred_direction = if predicted_delta.is_sign_negative() {
                    -1.0
                } else {
                    1.0
                };
                for direction in [preferred_direction, -preferred_direction] {
                    let mut previous = Some((old_m, f_center));
                    for sample in 1..=512 {
                        let offset = span * sample as Value / 512.0;
                        let candidate = old_m + direction * offset;
                        let Some((candidate_f, _)) = residual(candidate) else {
                            previous = None;
                            continue;
                        };
                        if !candidate_f.is_finite() {
                            previous = None;
                            continue;
                        }
                        if let Some((previous_m, previous_f)) = previous
                            && ((previous_f <= 0.0 && candidate_f >= 0.0)
                                || (previous_f >= 0.0 && candidate_f <= 0.0))
                        {
                            bracket = Some((previous_m, previous_f, candidate, candidate_f));
                            break;
                        }
                        previous = Some((candidate, candidate_f));
                    }
                    if bracket.is_some() {
                        break;
                    }
                }
                if let Some((mut lo, mut f_lo, mut hi, mut f_hi)) = bracket {
                    for _ in 0..128 {
                        let mid = lo + 0.5 * (hi - lo);
                        let Some((f_mid, _)) = residual(mid) else {
                            break;
                        };
                        if !f_mid.is_finite() {
                            break;
                        }
                        latest_m = mid;
                        if f_mid.abs() <= tolerance {
                            converged = true;
                            break;
                        }
                        if (hi - lo).abs() <= 1.0e-12 * scale {
                            converged = f_mid.abs() <= 1.0e-9 * scale;
                            break;
                        }
                        if (f_lo <= 0.0 && f_mid >= 0.0) || (f_lo >= 0.0 && f_mid <= 0.0) {
                            hi = mid;
                            f_hi = f_mid;
                        } else {
                            lo = mid;
                            f_lo = f_mid;
                        }
                        let _ = f_hi;
                    }
                }
            }
        }
        if !converged {
            // The hidden M equation is a coupled Newton unknown in Xyce,
            // not a scalar constitutive projection.  A fixed-electrical-state
            // solve can legitimately have no root (or encounter a
            // constitutive pole) even though the coupled MNA system does.
            // Return a finite accepted-state predictor and retain its hidden
            // residual so the branch stamp can eliminate M with the exact
            // Schur complement.  The Newton solve then moves the electrical
            // variables and M together instead of rejecting the iterate on a
            // scalar subproblem that Xyce never solves in isolation.
            // Preserve the carried Newton predictor.  The magnetic stamp
            // advances this predictor with the hidden M-row correction; it
            // must not be reset to the accepted state at every failed scalar
            // projection or reversal cannot traverse the coupled branch.
            latest_m = old_m + carried;
        }

        if !latest_m.is_finite() {
            return None;
        }
        let level1_residual = residual(latest_m)?.0;
        if !level1_residual.is_finite() {
            return None;
        }
        let h = happ - gap_path * latest_m;
        let p = self.xyce_core_p(h, latest_m, self.xyce_tanh_qv(voltage));
        let mid = 1.0 + (1.0 - gap_path) * p;
        let effective_inductance = self.params.base_inductance() * mid;
        if !p.is_finite() || !mid.is_finite() || !effective_inductance.is_finite() {
            return None;
        }
        Some(XyceCoreTrial {
            magnetization: latest_m,
            p,
            mid,
            effective_inductance,
            applied_field: happ,
            magnetization_update: latest_m - old_m,
            latest_magnetization: latest_m,
            level1_rate: rate,
            level1_residual,
            level1_rate_residual: 0.0,
        })
    }

    /// Return the partial derivatives of LEVEL=1's explicit hidden M equation
    /// `g(M, Happ, V, R)=0` while holding the endpoint M/R fixed for the
    /// electrical derivatives.  The four terms are derivatives with respect
    /// to physical M, applied field, first-winding voltage, and physical R.
    pub(crate) fn xyce_core_level1_hidden_partials(
        &self,
        trial: XyceCoreTrial,
        voltage: Value,
        dt: Value,
        one_step_order2: bool,
    ) -> Option<(Value, Value, Value, Value)> {
        if !self.params.xyce_core || self.params.xyce_core_level2 || !dt.is_finite() || dt <= 0.0 {
            return None;
        }
        let gap_path = self.params.gap / self.params.length;
        if !gap_path.is_finite() || !self.params.length.is_finite() {
            return None;
        }
        let p_scale = 1.0 - gap_path;
        let (fixed_mid_m, fixed_mid_happ, fixed_mid_voltage) = (
            self.xyce_core_dmid_d_magnetization(
                trial.applied_field,
                voltage,
                trial.latest_magnetization,
            )?,
            self.xyce_core_dmid_d_happ(
                trial.applied_field,
                voltage,
                trial.latest_magnetization,
                1.0,
            )?,
            self.xyce_core_dmid_d_voltage(trial.applied_field, voltage, trial.latest_magnetization)
                .unwrap_or(0.0),
        );
        let (dp_dm, dp_dhapp, dp_dvoltage) = if p_scale.abs() > 1.0e-18 {
            (
                fixed_mid_m / p_scale,
                fixed_mid_happ / p_scale,
                fixed_mid_voltage / p_scale,
            )
        } else {
            (0.0, 0.0, 0.0)
        };
        let integration_scale = (if one_step_order2 { 0.5 } else { 1.0 }) * dt / self.params.length;
        let g_m = 1.0 - integration_scale * trial.level1_rate * dp_dm;
        let g_happ = -integration_scale * trial.level1_rate * dp_dhapp;
        let g_voltage = -integration_scale * trial.level1_rate * dp_dvoltage;
        let g_rate = -integration_scale * trial.p;
        if g_m.is_finite() && g_happ.is_finite() && g_voltage.is_finite() && g_rate.is_finite() {
            Some((g_m, g_happ, g_voltage, g_rate))
        } else {
            None
        }
    }

    /// Return Xyce's physical hidden-R target for an aggregate applied-field
    /// increment. `delta_happ` is ΔS/Path, while R is dS/dt.
    pub(crate) fn xyce_core_level1_rate_target(
        &self,
        delta_happ: Value,
        dt: Value,
        one_step_order2: bool,
    ) -> Option<Value> {
        if !self.params.xyce_core
            || self.params.xyce_core_level2
            || !delta_happ.is_finite()
            || !dt.is_finite()
            || dt <= 0.0
            || !self.params.length.is_finite()
            || self.params.length <= 0.0
        {
            return None;
        }
        let rate = (if one_step_order2 { 2.0 } else { 1.0 }) * delta_happ * self.params.length / dt
            - if one_step_order2 {
                self.xyce_level1_rate
            } else {
                0.0
            };
        rate.is_finite().then_some(rate)
    }

    /// Evaluate LEVEL=1 at explicit hidden-M and hidden-R Newton unknowns.
    /// The supplied rate is physical dS/dt; the returned residual retains the
    /// R equation so the circuit assembler can stamp both coupled rows.
    pub(crate) fn xyce_core_level1_trial_at_magnetization_and_rate(
        &self,
        happ: Value,
        delta_happ: Value,
        voltage: Value,
        dt: Value,
        one_step_order2: bool,
        magnetization: Value,
        rate: Value,
    ) -> Option<XyceCoreTrial> {
        if !self.params.xyce_core
            || self.params.xyce_core_level2
            || !dt.is_finite()
            || dt <= 0.0
            || !happ.is_finite()
            || !delta_happ.is_finite()
            || !magnetization.is_finite()
            || !rate.is_finite()
        {
            return None;
        }
        let gap_path = self.params.gap / self.params.length;
        if !gap_path.is_finite() || !self.params.length.is_finite() || self.params.length <= 0.0 {
            return None;
        }
        let integration_scale = (if one_step_order2 { 0.5 } else { 1.0 }) * dt / self.params.length;
        let previous_product = if one_step_order2 {
            self.xyce_level1_p * self.xyce_level1_rate
        } else {
            0.0
        };
        let p = self.xyce_core_p(
            happ - gap_path * magnetization,
            magnetization,
            self.xyce_tanh_qv(voltage),
        );
        let residual =
            magnetization - self.state.m - integration_scale * (p * rate + previous_product);
        let rate_target = self.xyce_core_level1_rate_target(delta_happ, dt, one_step_order2)?;
        let rate_residual = rate - rate_target;
        let mid = 1.0 + (1.0 - gap_path) * p;
        let effective_inductance = self.params.base_inductance() * mid;
        if !p.is_finite()
            || !residual.is_finite()
            || !mid.is_finite()
            || !effective_inductance.is_finite()
        {
            return None;
        }
        Some(XyceCoreTrial {
            magnetization,
            p,
            mid,
            effective_inductance,
            applied_field: happ,
            magnetization_update: magnetization - self.state.m,
            latest_magnetization: magnetization,
            level1_rate: rate,
            level1_residual: residual,
            level1_rate_residual: rate_residual,
        })
    }

    /// Xyce MutIndNonLin's analytic partial derivative of the branch factor
    /// with respect to the winding current while holding `latestMag` fixed.
    /// This is the exact `dP_dI` path used by `loadDAEdFdx`; the magnetic state
    /// update itself remains explicit in the carried `MagVarUpdate` value.
    pub(crate) fn xyce_core_dmid_d_current(
        &self,
        current: Value,
        voltage: Value,
        latest_m: Value,
    ) -> Option<Value> {
        if !self.params.xyce_core {
            return None;
        }
        let happ = current * self.params.n_turns / self.params.length;
        let d_happ_d_current = self.params.n_turns / self.params.length;
        self.xyce_core_dmid_d_happ(happ, voltage, latest_m, d_happ_d_current)
    }

    /// Analytic LEVEL=1 partial of the normalized branch factor with respect
    /// to the solved magnetization, holding applied field and voltage fixed.
    /// This is the missing term needed when the hidden Xyce M equation is
    /// eliminated by a Schur complement in the compact Rust stamp.
    pub(crate) fn xyce_core_dmid_d_magnetization(
        &self,
        happ: Value,
        voltage: Value,
        latest_m: Value,
    ) -> Option<Value> {
        if !self.params.xyce_core {
            return None;
        }
        let gap_path = self.params.gap / self.params.length;
        let h = happ - gap_path * latest_m;
        let he = h + self.params.alpha * latest_m;
        let heo = self.params.beta_h * self.params.a;
        let he2 = he * he;
        let heo2 = heo * heo;
        let root_he = (heo2 + he2).sqrt();
        let denominator = self.params.a + root_he;
        if !root_he.is_finite() || root_he <= 0.0 || !denominator.is_finite() {
            return None;
        }
        let man_prime =
            self.params.ms * (self.params.a + heo2 / root_he) / (denominator * denominator);
        let man = self.params.ms * he / denominator;
        let del_m = man - latest_m;
        let del_m0 = self.params.beta_m * self.params.ms;
        let root_m = (del_m0 * del_m0 + del_m * del_m).sqrt();
        let mirr_denominator = 2.0 * (self.params.k - self.params.alpha * root_m);
        if !root_m.is_finite() || !mirr_denominator.is_finite() || mirr_denominator.abs() <= 1.0e-18
        {
            return None;
        }
        let tanh_qv = self.xyce_tanh_qv(voltage);
        let mirr_prime = (del_m * tanh_qv + root_m) / mirr_denominator;
        let p_denominator = 1.0
            + (gap_path - self.params.alpha) * self.params.c * man_prime
            + gap_path * (1.0 - self.params.c) * mirr_prime;
        if !p_denominator.is_finite() || p_denominator.abs() <= 1.0e-18 {
            return None;
        }

        // These are the physical-field derivatives from MutIndNonLin's
        // updateIntermediateVars().  dHe/dM includes both the mean-field and
        // air-gap contributions; d(delM)/dM also contains the explicit -1.
        let d_he_d_m = self.params.alpha - gap_path;
        let d_man_prime_d_m = (-self.params.ms * he / (denominator * denominator * root_he))
            * (heo2 / (heo2 + he2) + 2.0 * (self.params.a + heo2 / root_he) / denominator)
            * d_he_d_m;
        let d_del_m_d_m =
            (d_he_d_m * self.params.ms / denominator) * (1.0 - he2 / (denominator * root_he)) - 1.0;
        let d_mirr_prime_d_m = (1.0 / mirr_denominator)
            * (tanh_qv
                + del_m / root_m
                + (2.0 * self.params.alpha * del_m * (del_m * tanh_qv + root_m)
                    / (mirr_denominator * root_m)))
            * d_del_m_d_m;
        let numerator_slope =
            self.params.c * d_man_prime_d_m + (1.0 - self.params.c) * d_mirr_prime_d_m;
        let denominator_slope = (gap_path - self.params.alpha) * self.params.c * d_man_prime_d_m
            + gap_path * (1.0 - self.params.c) * d_mirr_prime_d_m;
        let d_p_d_m = numerator_slope / p_denominator
            - (self.params.c * man_prime + (1.0 - self.params.c) * mirr_prime) * denominator_slope
                / (p_denominator * p_denominator);
        let d_mid_d_m = (1.0 - gap_path) * d_p_d_m;
        d_mid_d_m.is_finite().then_some(d_mid_d_m)
    }

    /// Xyce MutIndNonLin's analytic partial derivative of the branch factor
    /// with respect to an arbitrary winding current.  `happ` is the
    /// aggregate applied field formed by all windings and
    /// `d_happ_d_current` is that winding's contribution to the field slope
    /// (normally `turns / path`).  Keeping the aggregate field explicit is
    /// essential for a shared multi-winding Core: every branch sees the same
    /// constitutive state, while each branch's Jacobian column has its own
    /// turns-dependent slope.
    pub(crate) fn xyce_core_dmid_d_happ(
        &self,
        happ: Value,
        voltage: Value,
        latest_m: Value,
        d_happ_d_current: Value,
    ) -> Option<Value> {
        if !self.params.xyce_core {
            return None;
        }
        let gap_path = self.params.gap / self.params.length;
        let h = happ - gap_path * latest_m;
        let he = h + self.params.alpha * latest_m;
        let heo = self.params.beta_h * self.params.a;
        let he2 = he * he;
        let heo2 = heo * heo;
        let root_he = (heo2 + he2).sqrt();
        let denominator = self.params.a + root_he;
        if !root_he.is_finite() || root_he <= 0.0 || !denominator.is_finite() {
            return None;
        }

        let man = self.params.ms * he / denominator;
        let del_m = man - latest_m;
        let del_m0 = self.params.beta_m * self.params.ms;
        let root_m = (del_m0 * del_m0 + del_m * del_m).sqrt();
        let mirr_denominator = 2.0 * (self.params.k - self.params.alpha * root_m);
        if !root_m.is_finite() || !mirr_denominator.is_finite() || mirr_denominator.abs() <= 1.0e-18
        {
            return None;
        }
        let tanh_qv = self.xyce_tanh_qv(voltage);
        let mirr_prime = (del_m * tanh_qv + root_m) / mirr_denominator;
        let man_prime =
            self.params.ms * (self.params.a + heo2 / root_he) / (denominator * denominator);
        let p_denominator = 1.0
            + (gap_path - self.params.alpha) * self.params.c * man_prime
            + gap_path * (1.0 - self.params.c) * mirr_prime;
        if !p_denominator.is_finite() || p_denominator.abs() <= 1.0e-18 {
            return None;
        }

        let d_he_d_current = d_happ_d_current;
        let d_man_prime_d_current = (-self.params.ms * he / (denominator * denominator * root_he))
            * (heo2 / (heo2 + he2) + 2.0 * (self.params.a + heo2 / root_he) / denominator)
            * d_he_d_current;
        let d_del_m_d_current =
            (self.params.ms / denominator) * (1.0 - he2 / (denominator * root_he)) * d_he_d_current;
        let d_mirr_prime_d_current = (1.0 / mirr_denominator)
            * (tanh_qv
                + del_m / root_m
                + (2.0 * self.params.alpha * del_m * (del_m * tanh_qv + root_m)
                    / (mirr_denominator * root_m)))
            * d_del_m_d_current;
        let numerator_slope =
            self.params.c * d_man_prime_d_current + (1.0 - self.params.c) * d_mirr_prime_d_current;
        let denominator_slope =
            (gap_path - self.params.alpha) * self.params.c * d_man_prime_d_current
                + gap_path * (1.0 - self.params.c) * d_mirr_prime_d_current;
        let d_p_d_current = numerator_slope / p_denominator
            - (self.params.c * (man_prime - mirr_prime) + mirr_prime) * denominator_slope
                / (p_denominator * p_denominator);
        let d_mid_d_current = (1.0 - gap_path) * d_p_d_current;
        d_mid_d_current.is_finite().then_some(d_mid_d_current)
    }

    /// Return the voltage partial of Xyce LEVEL=1's normalized branch
    /// factor.  MutIndNonLin includes this term in the branch Jacobian
    /// because its irreversible direction uses the adaptive
    /// `DELVSCALING/maxVoltageDrop` normalization.  LEVEL=2 deliberately
    /// omits the voltage partial in Xyce's `loadDAEdFdx` path.
    pub(crate) fn xyce_core_dmid_d_voltage(
        &self,
        happ: Value,
        voltage: Value,
        latest_m: Value,
    ) -> Option<Value> {
        if !self.params.xyce_core || self.params.xyce_core_level2 {
            return None;
        }

        let gap_path = self.params.gap / self.params.length;
        let h = happ - gap_path * latest_m;
        let he = h + self.params.alpha * latest_m;
        let heo = self.params.beta_h * self.params.a;
        let he2 = he * he;
        let heo2 = heo * heo;
        let root_he = (heo2 + he2).sqrt();
        let denominator = self.params.a + root_he;
        if !root_he.is_finite() || root_he <= 0.0 || !denominator.is_finite() {
            return None;
        }

        let man = self.params.ms * he / denominator;
        let del_m = man - latest_m;
        let del_m0 = self.params.beta_m * self.params.ms;
        let root_m = (del_m0 * del_m0 + del_m * del_m).sqrt();
        let mirr_denominator = 2.0 * (self.params.k - self.params.alpha * root_m);
        if !root_m.is_finite() || !mirr_denominator.is_finite() || mirr_denominator.abs() <= 1.0e-18
        {
            return None;
        }

        let tanh_qv = self.xyce_tanh_qv(voltage);
        let denominator_p = 1.0
            + (gap_path - self.params.alpha)
                * self.params.c
                * (self.params.ms * (self.params.a + heo2 / root_he) / (denominator * denominator))
            + gap_path * (1.0 - self.params.c) * ((del_m * tanh_qv + root_m) / mirr_denominator);
        if !denominator_p.is_finite() || denominator_p.abs() <= 1.0e-18 {
            return None;
        }

        let d_tanh_d_voltage =
            if self.params.delta_v_scaling.is_finite() && self.params.delta_v_scaling != 0.0 {
                let scale_denominator = self.state.max_voltage_drop.max(1.0e-10);
                self.params.delta_v_scaling / scale_denominator * (1.0 - tanh_qv * tanh_qv)
            } else {
                0.0
            };
        let d_mirr_d_tanh = del_m / mirr_denominator;
        let man_prime =
            self.params.ms * (self.params.a + heo2 / root_he) / (denominator * denominator);
        let mirr_prime = (del_m * tanh_qv + root_m) / mirr_denominator;
        let numerator = self.params.c * man_prime + (1.0 - self.params.c) * mirr_prime;
        let d_p_d_tanh =
            (1.0 - self.params.c) * d_mirr_d_tanh * (denominator_p - gap_path * numerator)
                / (denominator_p * denominator_p);
        let d_mid_d_voltage = (1.0 - gap_path) * d_p_d_tanh * d_tanh_d_voltage;
        d_mid_d_voltage.is_finite().then_some(d_mid_d_voltage)
    }

    /// Advance a Xyce Core state after an accepted electrical solution.
    ///
    /// Xyce evaluates `P` with the latest magnetization and then solves
    /// `M_new = M_old + P * delta(Happ)` as part of the coupled DAE.  The
    /// native runtime keeps the same accepted-step boundary, so solve that
    /// scalar constitutive equation to convergence instead of integrating a
    /// separate sign-based J-A approximation.
    fn integrate_xyce_core(
        &mut self,
        current: Value,
        voltage: Value,
        dt: Value,
        one_step_order2: bool,
    ) {
        let old_current = self.current;
        let delta_happ = (current - old_current) * self.params.n_turns / self.params.length;
        let happ = current * self.params.n_turns / self.params.length;
        self.integrate_xyce_core_from_happ(current, happ, delta_happ, voltage, dt, one_step_order2);
    }

    /// Commit an accepted Core endpoint expressed in aggregate magnetic-field
    /// coordinates.  A shared multi-winding Core has one constitutive state,
    /// so its magnetic update is driven by `Happ` and `delta_happ` assembled
    /// from all branch currents rather than by a single branch current.
    fn integrate_xyce_core_from_happ(
        &mut self,
        current: Value,
        happ: Value,
        delta_happ: Value,
        voltage: Value,
        dt: Value,
        one_step_order2: bool,
    ) {
        if voltage.abs() > self.state.max_voltage_drop {
            self.state.max_voltage_drop = voltage.abs();
        }
        let old_current = self.current;
        let old_m = self.state.m;
        // A Newton stamp computes P from the carried update and then replaces
        // that update with the newly predicted increment.  Recomputing here
        // from the replaced value would shift `latestMag` by one Newton
        // evaluation. Consume the exact endpoint produced by the accepted
        // stamp when available; direct DynamicDevice callers fall back to a
        // fresh pure evaluation.
        let cached_trial = self.xyce_trial.take();
        let trial = match cached_trial {
            Some((trial_current, trial_voltage, trial))
                if Self::xyce_endpoint_matches(trial_current, current)
                    && Self::xyce_endpoint_matches(trial_voltage, voltage) =>
            {
                // MutIndNonLin2 retains the MagVarUpdate produced by the
                // accepted updateIntermediateVars call.  The cached trial is
                // that exact endpoint, so carry its predictor across the
                // accepted-step boundary instead of leaving the previous
                // Newton iterate installed.
                if self.params.xyce_core_level2 {
                    self.xyce_mag_update = trial.magnetization_update;
                }
                trial
            }
            Some((_, _, _)) => {
                let trial = if !self.params.xyce_core_level2 {
                    let Some(trial) = self.xyce_core_level1_trial_from_happ(
                        happ,
                        delta_happ,
                        voltage,
                        dt,
                        one_step_order2,
                    ) else {
                        return;
                    };
                    trial
                } else {
                    let Some(trial) = self.xyce_core_trial_from_happ_with_update(
                        happ,
                        delta_happ,
                        voltage,
                        self.xyce_mag_update,
                    ) else {
                        return;
                    };
                    trial
                };
                self.xyce_mag_update = trial.magnetization_update;
                trial
            }
            None => {
                let trial = if !self.params.xyce_core_level2 {
                    let Some(trial) = self.xyce_core_level1_trial_from_happ(
                        happ,
                        delta_happ,
                        voltage,
                        dt,
                        one_step_order2,
                    ) else {
                        return;
                    };
                    trial
                } else {
                    let Some(trial) = self.xyce_core_trial_from_happ_with_update(
                        happ,
                        delta_happ,
                        voltage,
                        self.xyce_mag_update,
                    ) else {
                        return;
                    };
                    trial
                };
                // Xyce retains the endpoint MagVarUpdate produced by the
                // accepted updateIntermediateVars call.  The cached Newton
                // probe normally already installed this value, but a final
                // projection can produce an endpoint that is not bitwise
                // identical to the cached iterate.  Preserve the fallback
                // result as the carried update for the next timepoint.
                self.xyce_mag_update = trial.magnetization_update;
                trial
            }
        };
        let m_new = trial.magnetization;
        self.xyce_accepted_mid = if trial.mid.is_finite() && trial.mid.abs() > 1.0e-12 {
            trial.mid
        } else {
            1.0
        };
        let happ = trial.applied_field;
        // MutIndNonLin LEVEL=1 has no accepted-state saturation reset: its
        // hidden M equation is allowed to traverse beyond +/-2*Ms.  The
        // LEVEL=2 device has a separate MagVarUpdate limiter during its
        // transient update, so do not apply that LEVEL=2 behavior here.
        if m_new.is_finite() {
            self.state.m_prev = old_m;
            self.state.m = m_new;
        }
        if !self.params.xyce_core_level2 {
            self.xyce_level1_rate = trial.level1_rate;
            self.xyce_level1_p = trial.p;
            self.xyce_mag_update = trial.magnetization_update;
        }
        self.state.m_irr = self.state.m;

        self.state.h_prev = self.state.h;
        self.state.h = happ - (self.params.gap / self.params.length) * self.state.m;
        if delta_happ != 0.0 {
            self.state.delta = delta_happ.signum();
        }

        // MutIndNonLin's reported H/B stores deliberately retain an
        // artificial hysteresis path at nonphysical turning points.  This is
        // an output-only filter; the constitutive H used by P and the branch
        // equation remains the gap-corrected value above.
        let gap_factor = -(self.params.gap / self.params.length) * self.state.m;
        let calculated_h = happ + gap_factor;
        let dmdt = if dt.is_finite() && dt > 0.0 {
            (self.state.m - old_m) / dt
        } else {
            0.0
        };
        let d_happ_dt = if dt.is_finite() && dt > 0.0 {
            delta_happ / dt
        } else {
            0.0
        };
        let d_h_dt = d_happ_dt - (self.params.gap / self.params.length) * dmdt;
        let hold_h = if self.params.gap <= 0.0 {
            (self.xyce_dmdt_average < 0.0 && d_h_dt > 0.0)
                || (self.xyce_dmdt_average > 0.0 && d_h_dt < 0.0)
                || (self.xyce_dmdt_average < 0.0 && self.state.reported_h < calculated_h)
                || (self.xyce_dmdt_average > 0.0 && self.state.reported_h > calculated_h)
        } else {
            false
        };
        self.state.reported_h = if hold_h {
            self.state.reported_h
        } else if self.params.gap > 0.0
            && gap_factor.abs() < happ.abs()
            && gap_factor.signum() == happ.signum()
        {
            calculated_h
        } else {
            happ
        };
        self.state.b_prev = self.state.b;
        self.state.b = Self::mu_0() * (self.state.reported_h + self.state.m);

        // Xyce updates the ten-entry derivative average in acceptStep, after
        // computing this timepoint's secondary output state.  Preserve that
        // ordering so a rejected Newton attempt cannot perturb the filter.
        let cursor = self.xyce_dmdt_history_cursor;
        if self.xyce_dmdt_history_len < self.xyce_dmdt_history.len() {
            self.xyce_dmdt_history[cursor] = dmdt;
            self.xyce_dmdt_history_len += 1;
            self.xyce_dmdt_history_cursor = (cursor + 1) % self.xyce_dmdt_history.len();
        } else {
            self.xyce_dmdt_history[cursor] = dmdt;
            self.xyce_dmdt_history_cursor = (cursor + 1) % self.xyce_dmdt_history.len();
        }
        let history_sum: Value = self.xyce_dmdt_history.iter().sum();
        // The Xyce FixedQueue is initialized with ten zeros and always
        // averages all ten slots, including those initial zeros.
        self.xyce_dmdt_average = history_sum / self.xyce_dmdt_history.len() as Value;
        self.current_prev = old_current;
        self.current = current;
        self.voltage_prev = voltage;
        self.l_eff = self.effective_inductance();
    }

    fn core_or_ja_differential_susceptibility(&self, h: Value, m: Value, delta: Value) -> Value {
        if self.params.xyce_core {
            self.xyce_differential_susceptibility(h, m, delta)
        } else {
            self.differential_susceptibility(h, delta)
        }
    }

    /// Calculate effective inductance at current operating point
    /// L = N * dΦ/dI = N² * A * dB/dH * 1/(N/l) = N * A * l * dB/dH / l = N² * A / l * μr
    pub fn effective_inductance(&self) -> Value {
        if self.params.xyce_core {
            let p = self.xyce_core_p(
                self.state.h,
                self.state.m,
                self.xyce_tanh_qv(self.voltage_prev),
            );
            let mid = 1.0 + (1.0 - self.params.gap / self.params.length) * p;
            let l = self.params.base_inductance() * mid;
            if l.is_finite() && l.abs() > self.params.base_inductance() * 1.0e-12 {
                return l;
            }
            return self.params.base_inductance() * 1.0e-12;
        }
        let h = self.state.h;
        let dm_dh = self.differential_susceptibility(h, self.state.delta);
        let mu_r = 1.0 + dm_dh.max(0.0);

        const MU_0: Value = 4.0 * PI * 1e-7;
        let l = MU_0 * mu_r * self.params.n_turns * self.params.n_turns * self.params.area
            / self.params.length;

        // Ensure minimum inductance for numerical stability
        l.max(self.params.base_inductance() * 0.01)
    }

    /// Get current magnetization M (A/m)
    pub fn magnetization(&self) -> Value {
        self.state.m
    }

    pub(crate) fn xyce_core_mag_update(&self) -> Value {
        self.xyce_mag_update
    }

    #[doc(hidden)]
    pub(crate) fn xyce_core_level1_rate_debug(&self) -> Value {
        self.xyce_level1_rate
    }

    pub(crate) fn xyce_core_accepted_mid(&self) -> Value {
        self.xyce_accepted_mid
    }

    /// Return the constitutive factor from the final Newton endpoint when it
    /// is still cached, otherwise use the last accepted endpoint.  Xyce's
    /// OneStep `qHistory[2]` stores the static vector assembled before
    /// `acceptStep()` mutates the Core state, so an accepted-history probe must
    /// retain that pre-commit factor.
    pub(crate) fn xyce_core_static_mid(&self, current: Value, voltage: Value) -> Value {
        if let Some((trial_current, trial_voltage, trial)) = self.xyce_trial.as_ref()
            && Self::xyce_endpoint_matches(*trial_current, current)
            && Self::xyce_endpoint_matches(*trial_voltage, voltage)
            && trial.mid.is_finite()
            && trial.mid.abs() > 1.0e-12
        {
            return trial.mid;
        }
        self.xyce_accepted_mid
    }

    /// Cache the pure endpoint used by the current Newton stamp. The
    /// transient commit consumes this cache only when the accepted solution
    /// has the same branch current and winding voltage.
    pub(crate) fn cache_xyce_core_trial(
        &mut self,
        current: Value,
        voltage: Value,
        trial: XyceCoreTrial,
    ) {
        self.xyce_trial = Some((current, voltage, trial));
    }

    /// Check the scaled residuals of LEVEL=1's explicit hidden M/R equations
    /// for the most recently assembled Newton endpoint. Xyce scales each row
    /// by `1e-3`; in physical units the same relative threshold applies to M
    /// and R. LEVEL=2 has no hidden equations and is always converged here.
    pub(crate) fn xyce_core_trial_converged(&self) -> bool {
        if !self.params.xyce_core || self.params.xyce_core_level2 {
            return true;
        }
        let Some((_, _, trial)) = self.xyce_trial.as_ref() else {
            return false;
        };
        let scale = self
            .params
            .ms
            .abs()
            .max(self.state.m.abs())
            .max(trial.magnetization.abs())
            .max(1.0);
        let rate_scale = self
            .xyce_level1_rate
            .abs()
            .max(trial.level1_rate.abs())
            .max(1.0);
        trial.level1_residual.is_finite()
            && trial.level1_residual.abs() <= 1.0e-8 * scale
            && trial.level1_rate_residual.is_finite()
            && trial.level1_rate_residual.abs() <= 1.0e-8 * rate_scale
    }

    pub fn current_value(&self) -> Value {
        self.current
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get current magnetic field H (A/m)
    pub fn magnetic_field(&self) -> Value {
        if self.params.xyce_core {
            self.state.reported_h
        } else {
            self.state.h
        }
    }

    /// Get current flux density B (T)
    pub fn flux_density(&self) -> Value {
        self.state.b
    }

    /// Get flux linkage (Wb-turns)
    pub fn flux_linkage(&self) -> Value {
        self.flux_linkage
    }

    /// Geometry-only vacuum inductance used by Xyce's Core DAE as the
    /// constant flux-linkage coefficient before the constitutive `mid`
    /// factor is applied.
    pub fn nominal_inductance(&self) -> Value {
        self.params.base_inductance()
    }

    /// Return a constant vacuum mutual-inductance coefficient for two
    /// windings on this Core geometry.  Xyce's nonlinear mutual-inductor DAE
    /// uses this geometry-only matrix and applies the constitutive `mid`
    /// factor separately.  `turns_i` and `turns_j` may therefore differ from
    /// the representative winding stored in this device instance.
    pub(crate) fn xyce_core_vacuum_mutual_inductance(
        &self,
        turns_i: Value,
        turns_j: Value,
        coupling: Value,
    ) -> Value {
        Self::mu_0() * self.params.area / self.params.length * turns_i * turns_j * coupling
    }

    /// Convert aggregate ampere-turns to Xyce's applied magnetic field.
    pub(crate) fn xyce_core_happ_from_ampere_turns(&self, ampere_turns: Value) -> Value {
        ampere_turns / self.params.length
    }

    pub(crate) fn xyce_core_happ_from_current(&self, current: Value) -> Value {
        self.xyce_core_happ_from_ampere_turns(current * self.params.n_turns)
    }

    pub(crate) fn n_turns_for_xyce_core(&self) -> Value {
        self.params.n_turns
    }

    /// Return the applied-field slope contributed by one winding current.
    pub(crate) fn xyce_core_happ_slope_for_turns(&self, turns: Value) -> Value {
        turns / self.params.length
    }

    /// Commit an accepted transient Core solution with the actual interval
    /// length.  Xyce updates the hidden magnetic state at the accepted-step
    /// boundary, after Newton has selected the electrical solution; keeping
    /// this operation separate from the dt-free `NonlinearDevice::update`
    /// hook prevents rejected probes from advancing the output filter.
    pub(crate) fn commit_xyce_core_solution(
        &mut self,
        voltages: &[Value],
        dt: Value,
        one_step_order2: bool,
    ) {
        if !self.params.xyce_core {
            return;
        }
        let branch = self.branch_index.unwrap_or(0);
        let current = voltages
            .get(branch.saturating_sub(1))
            .copied()
            .unwrap_or(self.current);
        let v_pos = if self.node_pos == 0 {
            0.0
        } else {
            voltages.get(self.node_pos - 1).copied().unwrap_or(0.0)
        };
        let v_neg = if self.node_neg == 0 {
            0.0
        } else {
            voltages.get(self.node_neg - 1).copied().unwrap_or(0.0)
        };
        self.integrate_xyce_core(current, v_pos - v_neg, dt, one_step_order2);
    }

    /// Commit an accepted solution for a shared multi-winding Xyce Core.
    ///
    /// `happ` and `delta_happ` are assembled by the circuit from the signed
    /// sum of all winding ampere-turns.  The representative current retained
    /// by this device is only a state-bookkeeping coordinate used to match a
    /// cached Newton endpoint and to expose the same current-based APIs as a
    /// single-winding device.
    pub(crate) fn commit_xyce_core_group_solution(
        &mut self,
        happ: Value,
        delta_happ: Value,
        voltage: Value,
        dt: Value,
        one_step_order2: bool,
    ) {
        if !self.params.xyce_core {
            return;
        }
        let current = if self.params.n_turns.is_finite()
            && self.params.n_turns.abs() > 1.0e-30
            && self.params.length.is_finite()
        {
            happ * self.params.length / self.params.n_turns
        } else {
            self.current
        };
        self.integrate_xyce_core_from_happ(current, happ, delta_happ, voltage, dt, one_step_order2);
    }

    /// Get hysteresis loop area (approximates core loss per cycle)
    pub fn core_loss_per_cycle(&self) -> Value {
        self.core_loss
    }

    /// Calculate equivalent resistance for trapezoidal integration
    fn req(&self, dt: Value) -> Value {
        2.0 * self.l_eff / dt
    }

    /// Reset to demagnetized state
    pub fn reset(&mut self) {
        self.state = JaState::default();
        self.current = 0.0;
        self.current_prev = 0.0;
        self.flux_linkage = 0.0;
        self.flux_linkage_prev = 0.0;
        self.voltage_prev = 0.0;
        self.l_eff = self.params.base_inductance() * 1000.0;
        self.core_loss = 0.0;
        self.xyce_mag_update = 0.0;
        self.xyce_trial = None;
        self.xyce_accepted_mid = 1.0;
        self.xyce_dmdt_average = 0.0;
        self.xyce_dmdt_history = [0.0; 10];
        self.xyce_dmdt_history_len = 0;
        self.xyce_dmdt_history_cursor = 0;
        self.xyce_level1_rate = 0.0;
        self.xyce_level1_p = 0.0;
    }

    /// Stamp this inductor after verifying its MNA branch has been assigned.
    ///
    /// Public direct-device callers should prefer this checked API. The
    /// legacy [`DynamicDevice`] implementation logs and returns on this error
    /// because that trait cannot report failures.
    pub fn try_stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) -> Result<(), String> {
        let Some(branch) = self.branch_index else {
            return Err(format!(
                "Jiles-Atherton inductor '{}' cannot be stamped before its MNA branch index is assigned",
                self.name
            ));
        };
        let req = self.req(dt);

        // MNA stamp for inductor (voltage source with series resistance)
        matrix.stamp(branch, self.node_pos, 1.0);
        matrix.stamp(branch, self.node_neg, -1.0);
        matrix.stamp(branch, branch, -req);

        matrix.stamp(self.node_pos, branch, 1.0);
        matrix.stamp(self.node_neg, branch, -1.0);

        // Equivalent voltage source
        let veq = req * self.current_prev + self.voltage_prev;
        matrix.stamp_rhs(branch, veq);
        Ok(())
    }
}

impl DynamicDevice for JilesAthertonInductor {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    ) {
        if let Err(message) = self.try_stamp_transient(_voltages, dt, matrix, rhs) {
            log::error!("{message}");
        }
    }

    fn step(&mut self, voltages: &[Value], dt: Value) {
        let v_pos = if self.node_pos == 0 {
            0.0
        } else {
            voltages[self.node_pos - 1]
        };
        let v_neg = if self.node_neg == 0 {
            0.0
        } else {
            voltages[self.node_neg - 1]
        };
        let v = v_pos - v_neg;

        // Update flux linkage: Ψ = ∫v dt
        self.flux_linkage_prev = self.flux_linkage;
        self.flux_linkage += (dt / 2.0) * (v + self.voltage_prev);

        // Calculate B from flux linkage: B = Ψ / (N * A)
        let b_new = self.flux_linkage / (self.params.n_turns * self.params.area);

        // Calculate H from current and advance the canonical Core state only
        // at this accepted dynamic step.  Ordinary J-A devices retain their
        // standalone B-H integrator.
        let h_new = self.current * self.params.n_turns / self.params.length;
        if self.params.xyce_core {
            self.integrate_xyce_core(self.current, v, dt, false);
        } else {
            self.integrate_magnetization(h_new);
        }

        // Accumulate core loss (area of B-H loop segment)
        let db = b_new - self.state.b_prev;
        self.core_loss += (self.state.h * db).abs();

        // Update effective inductance
        self.l_eff = self.effective_inductance();

        // Update current using trapezoidal rule with current effective inductance
        let l_eff = self.l_eff.max(self.params.base_inductance() * 0.01);
        self.current_prev = self.current;
        self.current = self.current_prev + (dt / (2.0 * l_eff)) * (v + self.voltage_prev);

        self.voltage_prev = v;
    }
}

impl NonlinearDevice for JilesAthertonInductor {
    fn update(&mut self, voltages: &[Value]) {
        let branch = self.branch_index.unwrap_or(0);
        let new_current = if branch > 0 && branch <= voltages.len() {
            voltages[branch - 1]
        } else {
            self.current
        };
        if self.params.xyce_core {
            // This hook is called while probing a Newton iterate, but it has
            // no accepted-step context.  Xyce's Core model keeps
            // `oldBranchCurrentSum` fixed throughout all probes and advances
            // it only from the accepted endpoint.  Leave the accepted
            // current untouched; `commit_xyce_core_solution` owns that
            // lifecycle for transient analyses.
            return;
        }

        self.current = new_current;
        let h_new = self.current * self.params.n_turns / self.params.length;
        if (h_new - self.state.h).abs() > 1e-6 {
            self.integrate_magnetization(h_new);
            self.l_eff = self.effective_inductance();
        }
    }

    fn stamp_nonlinear(
        &self,
        _voltages: &[Value],
        _matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        // Nonlinear behavior is captured in effective inductance update
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        let tolerance = criteria.voltage_tolerance();
        // Check if magnetization has stabilized
        let dm = (self.state.m - self.state.m_prev).abs();
        dm / (self.state.m.abs().max(1e-12)) < tolerance
    }
}

//=============================================================================
// Tests
//=============================================================================

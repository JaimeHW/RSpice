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
}

impl JilesAthertonInductor {
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
        let old_m = self.state.m;
        let delta_happ = (current - old_current) * self.params.n_turns / self.params.length;
        let happ = current * self.params.n_turns / self.params.length;
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
        let gap_path = self.params.gap / self.params.length;
        let happ = current * self.params.n_turns / self.params.length;
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

        let d_he_d_current = self.params.n_turns / self.params.length;
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

    /// Advance a Xyce Core state after an accepted electrical solution.
    ///
    /// Xyce evaluates `P` with the latest magnetization and then solves
    /// `M_new = M_old + P * delta(Happ)` as part of the coupled DAE.  The
    /// native runtime keeps the same accepted-step boundary, so solve that
    /// scalar constitutive equation to convergence instead of integrating a
    /// separate sign-based J-A approximation.
    fn integrate_xyce_core(&mut self, current: Value, voltage: Value, dt: Value) {
        if voltage.abs() > self.state.max_voltage_drop {
            self.state.max_voltage_drop = voltage.abs();
        }
        let old_current = self.current;
        let old_m = self.state.m;
        let delta_happ = (current - old_current) * self.params.n_turns / self.params.length;
        // A Newton stamp computes P from the carried update and then replaces
        // that update with the newly predicted increment.  Recomputing here
        // from the replaced value would shift `latestMag` by one Newton
        // evaluation. Consume the exact endpoint produced by the accepted
        // stamp when available; direct DynamicDevice callers fall back to a
        // fresh pure evaluation.
        let trial = match self.xyce_trial.take() {
            Some((trial_current, trial_voltage, trial))
                if trial_current == current && trial_voltage == voltage =>
            {
                trial
            }
            _ => {
                let trial = self
                    .xyce_core_trial(current, voltage)
                    .expect("xyce core trial requires a Core model");
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
        let mut m_new = trial.magnetization;
        self.xyce_accepted_mid = if trial.mid.is_finite() && trial.mid.abs() > 1.0e-12 {
            trial.mid
        } else {
            1.0
        };
        let happ = trial.applied_field;
        // Xyce permits a transient excursion beyond Ms, but resets an
        // unbounded state after two saturation values.  Keep the same guard;
        // do not impose the ordinary J-A +/-Ms clamp on Core models.
        // Xyce applies its two-saturation reset only at the accepted-step
        // boundary.  Newton endpoint probes must remain continuous so the
        // finite-difference branch Jacobian does not see a trial-only jump.
        if m_new.is_finite() && m_new.abs() > 2.0 * self.params.ms {
            m_new = 0.0;
        }
        if m_new.is_finite() {
            self.state.m_prev = old_m;
            self.state.m = m_new;
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
            (happ - old_current * self.params.n_turns / self.params.length) / dt
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
            && *trial_current == current
            && *trial_voltage == voltage
            && trial.mid.is_finite()
            && trial.mid.abs() > 1.0e-12
        {
            return trial.mid;
        }
        self.xyce_accepted_mid
    }

    pub(crate) fn set_xyce_core_mag_update(&mut self, value: Value) {
        self.xyce_mag_update = if value.is_finite() { value } else { 0.0 };
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

    /// Commit an accepted transient Core solution with the actual interval
    /// length.  Xyce updates the hidden magnetic state at the accepted-step
    /// boundary, after Newton has selected the electrical solution; keeping
    /// this operation separate from the dt-free `NonlinearDevice::update`
    /// hook prevents rejected probes from advancing the output filter.
    pub(crate) fn commit_xyce_core_solution(&mut self, voltages: &[Value], dt: Value) {
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
        self.integrate_xyce_core(current, v_pos - v_neg, dt);
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
            self.integrate_xyce_core(self.current, v, dt);
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

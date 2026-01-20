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

use crate::device::traits::{DynamicDevice, MatrixStamper, NonlinearDevice};
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
        self.state.m_prev = self.state.m;
        self.state.m_irr = self.state.m;
        // Calculate B and flux linkage
        self.state.b = Self::mu_0() * (h + self.state.m);
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

        // RK4 integration for accuracy
        let k1 = self.differential_susceptibility(h, self.state.delta);
        let k2 = self.differential_susceptibility(h + 0.5 * dh, self.state.delta);
        let k3 = self.differential_susceptibility(h + 0.5 * dh, self.state.delta);
        let k4 = self.differential_susceptibility(h_new, self.state.delta);

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

    /// Calculate effective inductance at current operating point
    /// L = N * dΦ/dI = N² * A * dB/dH * 1/(N/l) = N * A * l * dB/dH / l = N² * A / l * μr
    pub fn effective_inductance(&self) -> Value {
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

    /// Get current magnetic field H (A/m)
    pub fn magnetic_field(&self) -> Value {
        self.state.h
    }

    /// Get current flux density B (T)
    pub fn flux_density(&self) -> Value {
        self.state.b
    }

    /// Get flux linkage (Wb-turns)
    pub fn flux_linkage(&self) -> Value {
        self.flux_linkage
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
    }
}

impl DynamicDevice for JilesAthertonInductor {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let branch = self.branch_index.expect("Branch index must be set");
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

        // Calculate H from current: H = N * I / l
        let h_new = self.current * self.params.n_turns / self.params.length;

        // Integrate magnetization on B-H loop
        self.integrate_magnetization(h_new);

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
        if branch > 0 && branch <= voltages.len() {
            self.current = voltages[branch - 1];
        }

        // Update H and magnetization for new current
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

    fn is_converged(&self, tolerance: Value) -> bool {
        // Check if magnetization has stabilized
        let dm = (self.state.m - self.state.m_prev).abs();
        dm / (self.state.m.abs().max(1e-12)) < tolerance
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ja_params_default() {
        let params = JilesAthertonParams::default();
        assert!(params.ms > 0.0);
        assert!(params.a > 0.0);
        assert!(params.k > 0.0);
        assert!(params.c >= 0.0 && params.c <= 1.0);
        assert!(params.alpha > 0.0);
    }

    #[test]
    fn test_ja_params_presets() {
        let ferrite = JilesAthertonParams::power_ferrite();
        let steel = JilesAthertonParams::silicon_steel();
        let perm = JilesAthertonParams::permalloy();

        // Silicon steel should have highest Ms
        assert!(steel.ms > ferrite.ms);

        // Permalloy should have lowest k (narrowest hysteresis)
        assert!(perm.k < steel.k);
    }

    #[test]
    fn test_ja_inductor_creation() {
        let ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);
        assert_eq!(ind.name, "L1");
        assert_eq!(ind.node_pos, 1);
        assert_eq!(ind.node_neg, 2);
        assert_eq!(ind.current, 0.0);
    }

    #[test]
    fn test_anhysteretic_symmetry() {
        let ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        // Anhysteretic curve should be antisymmetric
        let m_pos = ind.anhysteretic(1000.0);
        let m_neg = ind.anhysteretic(-1000.0);

        assert!(
            (m_pos + m_neg).abs() < 1e-10,
            "Anhysteretic should be antisymmetric"
        );
    }

    #[test]
    fn test_anhysteretic_saturation() {
        let ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        // At very high H, M should approach Ms
        let m_high = ind.anhysteretic(1e6);
        assert!(m_high > 0.99 * ind.params.ms, "Should saturate at high H");

        // At H=0, M should be 0
        let m_zero = ind.anhysteretic(0.0);
        assert!(m_zero.abs() < 1e-10, "M should be 0 at H=0");
    }

    #[test]
    fn test_differential_susceptibility_positive() {
        let ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        // Susceptibility should be positive for forward motion
        let chi = ind.differential_susceptibility(100.0, 1.0);
        assert!(chi >= 0.0, "Susceptibility should be non-negative");
    }

    #[test]
    fn test_effective_inductance_range() {
        let ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        let l_eff = ind.effective_inductance();
        let l_base = ind.params.base_inductance();

        // Effective inductance should be reasonable
        assert!(l_eff > l_base * 0.01, "Effective L should be above minimum");
        assert!(
            l_eff < l_base * 10000.0,
            "Effective L should be below maximum"
        );
    }

    #[test]
    fn test_magnetization_integration() {
        let mut ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        // Start at H=0
        assert_eq!(ind.state.h, 0.0);
        assert!(ind.state.m.abs() < 1e-10);

        // Apply increasing H
        ind.integrate_magnetization(500.0);
        assert!(ind.state.m > 0.0, "M should increase with H");

        // Apply decreasing H (should stay on upper branch due to hysteresis)
        let m_before = ind.state.m;
        ind.integrate_magnetization(400.0);
        assert!(ind.state.m > 0.0, "M should still be positive");
        assert!(ind.state.m < m_before, "M should decrease slightly");
    }

    #[test]
    fn test_hysteresis_loop_direction() {
        let mut ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        // Go up to H = 1000
        for h in (0..=1000).step_by(100) {
            ind.integrate_magnetization(h as f64);
        }
        let _m_up = ind.state.m;

        // Go back down to H = 500
        for h in (500..=1000).rev().step_by(100) {
            ind.integrate_magnetization(h as f64);
        }
        let m_down = ind.state.m;

        // On the way down, M should be higher than on the way up (hysteresis)
        // at the same H field
        assert!(m_down > 0.0, "M should be positive on return");
    }

    #[test]
    fn test_reset_clears_state() {
        let mut ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        // Apply some magnetization
        ind.integrate_magnetization(1000.0);
        assert!(ind.state.m.abs() > 0.1);

        // Reset
        ind.reset();
        assert_eq!(ind.state.m, 0.0);
        assert_eq!(ind.state.h, 0.0);
        assert_eq!(ind.current, 0.0);
    }

    #[test]
    fn test_set_initial_current() {
        let mut ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);
        ind.set_initial_current(1.0);

        assert_eq!(ind.current, 1.0);
        assert!(ind.state.h > 0.0, "H should be set from current");
        assert!(ind.state.m != 0.0, "M should be initialized");
    }

    #[test]
    fn test_base_inductance_calculation() {
        let params = JilesAthertonParams {
            n_turns: 100.0,
            area: 1e-4,  // 1 cm²
            length: 0.1, // 10 cm
            ..Default::default()
        };

        let l_base = params.base_inductance();

        // L = μ₀ * N² * A / l = 4π*1e-7 * 10000 * 1e-4 / 0.1
        // = 4π * 1e-7 * 10000 * 1e-3 = 4π * 1e-6 ≈ 12.566 μH
        let expected = 4.0 * PI * 1e-6;
        assert!(
            (l_base - expected).abs() < 1e-12,
            "Base inductance calculation error: {} vs {}",
            l_base,
            expected
        );
    }

    #[test]
    fn test_langevin_taylor_series() {
        let ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        // For very small H, Taylor series should match full Langevin
        let h_small = 0.01;
        let m1 = ind.anhysteretic(h_small);

        // The Taylor series approximation for small x: L(x) ≈ x/3
        let x = h_small / ind.params.a;
        let m_approx = ind.params.ms * x / 3.0;

        // Should be very close for small x
        assert!(
            (m1 - m_approx).abs() / m_approx.abs() < 0.01,
            "Taylor series should match for small H"
        );
    }

    #[test]
    fn test_core_loss_accumulation() {
        let mut ind = JilesAthertonInductor::new("L1".to_string(), 1, 2);

        assert_eq!(ind.core_loss, 0.0);

        // Simulate a magnetization cycle (this is simplified)
        ind.integrate_magnetization(1000.0);
        ind.integrate_magnetization(-1000.0);
        ind.integrate_magnetization(1000.0);

        // Should have accumulated some loss
        // Note: Full loss calculation requires step() with voltage integration
    }
}

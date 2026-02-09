//! GaN HEMT (High Electron Mobility Transistor) Device Model
//!
//! Physics-based compact model for Gallium Nitride HEMTs, critical for
//! power electronics, RF amplifiers, and high-frequency switching applications.
//!
//! # Model Physics
//!
//! GaN HEMTs are based on AlGaN/GaN heterostructures with a 2D electron gas (2DEG).
//! Key features include:
//! - High breakdown voltage (>600V possible)
//! - High electron mobility (~2000 cm²/V·s)
//! - Low on-resistance
//! - High temperature operation
//! - Self-heating effects
//!
//! # Model Equations
//!
//! The drain current uses a hyperbolic tangent model with velocity saturation:
//! Ids = β × (Vgs - Vth)² × tanh(αVds) × (1 + λVds) × Θ(T)
//!
//! Where:
//! - β = transconductance parameter
//! - Vth = threshold voltage (typically negative for enhancement mode)
//! - α = saturation parameter
//! - λ = channel length modulation
//! - Θ(T) = temperature scaling factor

use crate::Value;

//=============================================================================
// Physical Constants
//=============================================================================

/// Boltzmann constant (J/K)
const KB: Value = 1.380649e-23;
/// Elementary charge (C)
const Q: Value = 1.602176634e-19;
/// Reference temperature (K)
const TREF: Value = 300.15;

//=============================================================================
// GaN HEMT Model Parameters
//=============================================================================

/// GaN HEMT model parameters
#[derive(Debug, Clone)]
pub struct GanHemtParams {
    // Threshold voltage parameters
    /// Zero-bias threshold voltage (V) - typically negative for depletion mode
    pub vth0: Value,
    /// Threshold voltage temperature coefficient (V/K)
    pub vth_temp_coeff: Value,

    // Transconductance parameters
    /// Transconductance parameter (A/V²)
    pub beta: Value,
    /// Beta temperature exponent
    pub beta_temp_exp: Value,

    // Saturation parameters
    /// Saturation parameter (1/V)
    pub alpha: Value,
    /// Saturation velocity (m/s)
    pub vsat: Value,

    // Output conductance
    /// Channel length modulation (1/V)
    pub lambda: Value,
    /// DIBL coefficient (V/V)
    pub dibl: Value,

    // Subthreshold parameters
    /// Subthreshold swing ideality factor
    pub n_sub: Value,
    /// Subthreshold slope temperature coefficient
    pub n_sub_temp: Value,

    // Capacitance parameters
    /// Gate-source capacitance (F)
    pub cgs: Value,
    /// Gate-drain capacitance (F)
    pub cgd: Value,
    /// Drain-source capacitance (F)
    pub cds: Value,

    // Parasitic resistances
    /// Source resistance (Ω)
    pub rs: Value,
    /// Drain resistance (Ω)
    pub rd: Value,
    /// Gate resistance (Ω)
    pub rg: Value,

    // Thermal parameters
    /// Thermal resistance junction-to-case (K/W)
    pub rth: Value,
    /// Thermal time constant (s)
    pub tau_th: Value,
    /// Max junction temperature (K)
    pub tj_max: Value,

    // Breakdown parameters
    /// Breakdown voltage (V)
    pub bv: Value,
    /// Breakdown current (A)
    pub ibv: Value,

    // Trap parameters (for dynamic effects)
    /// Trap time constant (s)
    pub tau_trap: Value,
    /// Trap density factor
    pub trap_density: Value,
}

impl Default for GanHemtParams {
    fn default() -> Self {
        Self {
            // Threshold voltage - typical enhancement mode GaN
            vth0: 1.5,
            vth_temp_coeff: -2.5e-3,

            // Transconductance - typical 650V/30A GaN power device
            // At Vgs=5V, Vds=10V: Ids = β × (5-1.5)² × tanh(5) × 1.2 ≈ β × 14.7
            // For ~10A nominal, β ≈ 0.7 A/V²
            beta: 0.7,
            beta_temp_exp: -1.5,

            // Saturation
            alpha: 0.5,
            vsat: 1.5e5,

            // Output conductance
            lambda: 0.02,
            dibl: 0.01,

            // Subthreshold
            n_sub: 1.5,
            n_sub_temp: 0.001,

            // Capacitances (typical 30A device)
            cgs: 300e-12,
            cgd: 30e-12,
            cds: 150e-12,

            // Parasitic resistances
            rs: 0.003,
            rd: 0.003,
            rg: 0.5,

            // Thermal - low Rth for good cooling (1.5 K/W typical for TO-247)
            rth: 1.5,
            tau_th: 1e-3,
            tj_max: 448.15, // 175°C

            // Breakdown
            bv: 650.0,
            ibv: 1e-6,

            // Traps
            tau_trap: 1e-6,
            trap_density: 0.1,
        }
    }
}

impl GanHemtParams {
    /// Create parameters for power switching GaN
    pub fn power_gan(voltage_rating: Value, current_rating: Value) -> Self {
        let beta = current_rating / 25.0; // Approximate scaling
        let cgs = current_rating * 10e-12; // Scale capacitance with current

        Self {
            vth0: 1.5,
            beta,
            bv: voltage_rating * 1.2, // 20% margin
            cgs,
            cgd: cgs * 0.1,
            cds: cgs * 0.5,
            rs: 0.1 / current_rating,
            rd: 0.1 / current_rating,
            rth: 1.0 / current_rating.sqrt(),
            ..Default::default()
        }
    }

    /// Create parameters for RF GaN
    pub fn rf_gan(ft_ghz: Value) -> Self {
        // Estimate capacitance from target ft
        let cgs = 1e-12 / ft_ghz; // Simplified

        Self {
            vth0: -3.0, // Depletion mode
            beta: 100e-3,
            alpha: 1.0,
            cgs,
            cgd: cgs * 0.2,
            cds: cgs * 0.1,
            lambda: 0.05,
            ..Default::default()
        }
    }

    /// Get temperature-adjusted threshold voltage
    pub fn vth_at_temp(&self, temp: Value) -> Value {
        self.vth0 + self.vth_temp_coeff * (temp - TREF)
    }

    /// Get temperature-adjusted beta
    pub fn beta_at_temp(&self, temp: Value) -> Value {
        self.beta * (temp / TREF).powf(self.beta_temp_exp)
    }

    /// Get thermal voltage at temperature
    pub fn vt(&self, temp: Value) -> Value {
        KB * temp / Q
    }
}

//=============================================================================
// GaN HEMT Model State
//=============================================================================

/// Operating state of GaN HEMT
#[derive(Debug, Clone)]
pub struct GanHemtState {
    /// Gate-source voltage (V)
    pub vgs: Value,
    /// Drain-source voltage (V)
    pub vds: Value,
    /// Junction temperature (K)
    pub tj: Value,
    /// Power dissipation (W)
    pub power: Value,
    /// Trap state (normalized 0-1)
    pub trap_state: Value,
}

impl Default for GanHemtState {
    fn default() -> Self {
        Self {
            vgs: 0.0,
            vds: 0.0,
            tj: TREF,
            power: 0.0,
            trap_state: 0.0,
        }
    }
}

impl GanHemtState {
    /// Create state with bias voltages
    pub fn with_bias(vgs: Value, vds: Value) -> Self {
        Self {
            vgs,
            vds,
            ..Default::default()
        }
    }
}

//=============================================================================
// GaN HEMT Model
//=============================================================================

/// GaN HEMT device model
#[derive(Debug, Clone)]
pub struct GanHemt {
    /// Model parameters
    pub params: GanHemtParams,
    /// Current state
    pub state: GanHemtState,
}

impl GanHemt {
    /// Create new GaN HEMT with default parameters
    pub fn new() -> Self {
        Self {
            params: GanHemtParams::default(),
            state: GanHemtState::default(),
        }
    }

    /// Create with specific parameters
    pub fn with_params(params: GanHemtParams) -> Self {
        Self {
            params,
            state: GanHemtState::default(),
        }
    }

    /// Calculate drain current
    ///
    /// Uses hyperbolic tangent model with temperature effects
    pub fn ids(&self, vgs: Value, vds: Value, tj: Value) -> Value {
        let params = &self.params;

        // Temperature-adjusted parameters
        let vth = params.vth_at_temp(tj);
        let beta = params.beta_at_temp(tj);
        let vt = params.vt(tj);

        // Gate overdrive
        let vgt = vgs - vth;

        // Subthreshold region
        if vgt < 0.0 {
            let n = params.n_sub + params.n_sub_temp * (tj - TREF);
            let i_sub = beta * (n * vt).powi(2) * (vgt / (n * vt)).exp();
            return i_sub.max(0.0) * (1.0 - (-vds / vt).exp());
        }

        // Above threshold - hyperbolic tangent saturation model
        let alpha = params.alpha;
        let lambda = params.lambda;

        // DIBL effect
        let vth_dibl = vth - params.dibl * vds;
        let vgt_dibl = vgs - vth_dibl;

        if vgt_dibl <= 0.0 {
            return 0.0;
        }

        // Core I-V relationship
        let ids_lin = beta * vgt_dibl.powi(2);
        let tanh_sat = (alpha * vds).tanh();
        let chlm = 1.0 + lambda * vds;

        // Trap effect (reduces current slightly)
        let trap_factor = 1.0 - self.state.trap_state * params.trap_density;

        (ids_lin * tanh_sat * chlm * trap_factor).max(0.0)
    }

    /// Calculate transconductance gm = dIds/dVgs
    pub fn gm(&self, vgs: Value, vds: Value, tj: Value) -> Value {
        let delta = 1e-6;
        let ids_plus = self.ids(vgs + delta, vds, tj);
        let ids_minus = self.ids(vgs - delta, vds, tj);
        (ids_plus - ids_minus) / (2.0 * delta)
    }

    /// Calculate output conductance gds = dIds/dVds
    pub fn gds(&self, vgs: Value, vds: Value, tj: Value) -> Value {
        let delta = 1e-6;
        let ids_plus = self.ids(vgs, vds + delta, tj);
        let ids_minus = self.ids(vgs, vds - delta, tj);
        (ids_plus - ids_minus) / (2.0 * delta)
    }

    /// Calculate power dissipation
    pub fn power(&self, vgs: Value, vds: Value, tj: Value) -> Value {
        let ids = self.ids(vgs, vds, tj);
        let vds_eff = vds - ids * (self.params.rs + self.params.rd);
        ids * vds_eff.max(0.0)
    }

    /// Calculate gate charge
    pub fn qg(&self, vgs: Value, vds: Value) -> Value {
        // Simplified gate charge model
        self.params.cgs * vgs + self.params.cgd * (vgs - vds)
    }

    /// Calculate drain charge
    pub fn qd(&self, vgs: Value, vds: Value) -> Value {
        self.params.cgd * (vds - vgs) + self.params.cds * vds
    }

    /// Get small-signal capacitances at operating point
    pub fn capacitances(&self, _vgs: Value, _vds: Value) -> (Value, Value, Value) {
        // For now, use constant capacitances
        // A more sophisticated model would have voltage-dependent caps
        (self.params.cgs, self.params.cgd, self.params.cds)
    }

    /// Calculate ft (unity current gain frequency)
    pub fn ft(&self, vgs: Value, vds: Value, tj: Value) -> Value {
        let gm = self.gm(vgs, vds, tj);
        let cgs = self.params.cgs;
        let cgd = self.params.cgd;

        gm / (2.0 * std::f64::consts::PI * (cgs + cgd))
    }

    /// Calculate fmax (maximum oscillation frequency)
    pub fn fmax(&self, vgs: Value, vds: Value, tj: Value) -> Value {
        let ft = self.ft(vgs, vds, tj);
        let gds = self.gds(vgs, vds, tj);
        let rg = self.params.rg;
        let cgd = self.params.cgd;

        ft / (2.0 * (rg * gds + 2.0 * std::f64::consts::PI * ft * rg * cgd).sqrt())
    }

    /// Update thermal state (for transient)
    pub fn update_thermal(&mut self, power: Value, ambient_temp: Value, dt: Value) {
        let tj_target = ambient_temp + power * self.params.rth;
        let tau = self.params.tau_th;

        // First-order thermal response
        let alpha = 1.0 - (-dt / tau).exp();
        self.state.tj = self.state.tj + alpha * (tj_target - self.state.tj);
        self.state.power = power;
    }

    /// Check if device is in safe operating area
    pub fn is_soa_ok(&self, _vgs: Value, vds: Value, ids: Value) -> bool {
        let tj = self.state.tj;

        // Voltage limit
        if vds > self.params.bv * 0.9 {
            return false;
        }

        // Temperature limit
        if tj > self.params.tj_max {
            return false;
        }

        // Power limit (thermal)
        let power = ids * vds;
        let tj_estimated = TREF + power * self.params.rth;
        if tj_estimated > self.params.tj_max {
            return false;
        }

        true
    }
}

impl Default for GanHemt {
    fn default() -> Self {
        Self::new()
    }
}

//=============================================================================
// Process Corners
//=============================================================================

/// Process corner definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProcessCorner {
    /// Typical parameters (TT)
    Typical,
    /// Fast NMOS, Fast PMOS (FF)
    FastFast,
    /// Slow NMOS, Slow PMOS (SS)
    SlowSlow,
    /// Fast NMOS, Slow PMOS (FS)
    FastSlow,
    /// Slow NMOS, Fast PMOS (SF)
    SlowFast,
}

impl Default for ProcessCorner {
    fn default() -> Self {
        Self::Typical
    }
}

impl ProcessCorner {
    /// Get all standard corners
    pub fn all_standard() -> Vec<Self> {
        vec![
            Self::Typical,
            Self::FastFast,
            Self::SlowSlow,
            Self::FastSlow,
            Self::SlowFast,
        ]
    }

    /// Get corner description
    pub fn name(&self) -> &'static str {
        match self {
            Self::Typical => "TT",
            Self::FastFast => "FF",
            Self::SlowSlow => "SS",
            Self::FastSlow => "FS",
            Self::SlowFast => "SF",
        }
    }

    /// Get Vth scaling factor (relative to typical)
    pub fn vth_scale(&self) -> Value {
        match self {
            Self::Typical => 1.0,
            Self::FastFast => 0.9, // Lower Vth = faster
            Self::SlowSlow => 1.1, // Higher Vth = slower
            Self::FastSlow => 0.95,
            Self::SlowFast => 1.05,
        }
    }

    /// Get mobility scaling factor (relative to typical)
    pub fn mobility_scale(&self) -> Value {
        match self {
            Self::Typical => 1.0,
            Self::FastFast => 1.15,
            Self::SlowSlow => 0.85,
            Self::FastSlow => 1.0,
            Self::SlowFast => 1.0,
        }
    }

    /// Get capacitance scaling factor
    pub fn cap_scale(&self) -> Value {
        match self {
            Self::Typical => 1.0,
            Self::FastFast => 0.95,
            Self::SlowSlow => 1.05,
            Self::FastSlow => 1.0,
            Self::SlowFast => 1.0,
        }
    }
}

/// Apply process corner to GaN HEMT parameters
pub fn apply_corner(params: &GanHemtParams, corner: ProcessCorner) -> GanHemtParams {
    let mut result = params.clone();

    result.vth0 *= corner.vth_scale();
    result.beta *= corner.mobility_scale();
    result.cgs *= corner.cap_scale();
    result.cgd *= corner.cap_scale();
    result.cds *= corner.cap_scale();

    result
}

//=============================================================================
// Statistical Variability
//=============================================================================

/// Statistical variation configuration
#[derive(Debug, Clone)]
pub struct StatisticalVariation {
    /// Threshold voltage sigma (V)
    pub vth_sigma: Value,
    /// Beta sigma (relative)
    pub beta_sigma: Value,
    /// Capacitance sigma (relative)
    pub c_sigma: Value,
    /// Resistance sigma (relative)
    pub r_sigma: Value,
}

impl Default for StatisticalVariation {
    fn default() -> Self {
        Self {
            vth_sigma: 0.05,  // 50mV 1-sigma
            beta_sigma: 0.05, // 5% 1-sigma
            c_sigma: 0.03,    // 3% 1-sigma
            r_sigma: 0.10,    // 10% 1-sigma
        }
    }
}

impl StatisticalVariation {
    /// Apply random variation to parameters
    ///
    /// # Arguments
    /// * `params` - Base parameters
    /// * `normal_samples` - Vector of standard normal samples [vth, beta, c, r]
    pub fn apply(&self, params: &GanHemtParams, normal_samples: &[Value]) -> GanHemtParams {
        let mut result = params.clone();

        if normal_samples.len() >= 4 {
            result.vth0 += self.vth_sigma * normal_samples[0];
            result.beta *= 1.0 + self.beta_sigma * normal_samples[1];
            result.cgs *= 1.0 + self.c_sigma * normal_samples[2];
            result.cgd *= 1.0 + self.c_sigma * normal_samples[2];
            result.cds *= 1.0 + self.c_sigma * normal_samples[2];
            result.rs *= 1.0 + self.r_sigma * normal_samples[3];
            result.rd *= 1.0 + self.r_sigma * normal_samples[3];
        }

        result
    }

    /// Generate parameter set for Monte Carlo
    ///
    /// Returns parameters with 3-sigma variations applied
    pub fn three_sigma_samples(&self, base: &GanHemtParams) -> Vec<(&'static str, GanHemtParams)> {
        vec![
            ("Vth+3σ", self.apply(base, &[3.0, 0.0, 0.0, 0.0])),
            ("Vth-3σ", self.apply(base, &[-3.0, 0.0, 0.0, 0.0])),
            ("β+3σ", self.apply(base, &[0.0, 3.0, 0.0, 0.0])),
            ("β-3σ", self.apply(base, &[0.0, -3.0, 0.0, 0.0])),
            ("C+3σ", self.apply(base, &[0.0, 0.0, 3.0, 0.0])),
            ("C-3σ", self.apply(base, &[0.0, 0.0, -3.0, 0.0])),
            ("R+3σ", self.apply(base, &[0.0, 0.0, 0.0, 3.0])),
            ("R-3σ", self.apply(base, &[0.0, 0.0, 0.0, -3.0])),
        ]
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gan_hemt_default_params() {
        let params = GanHemtParams::default();
        assert!(params.vth0 > 0.0); // Enhancement mode
        assert!(params.beta > 0.0);
        assert!(params.bv > 0.0);
    }

    #[test]
    fn test_gan_hemt_power_params() {
        let params = GanHemtParams::power_gan(650.0, 30.0);
        assert!(params.bv > 650.0);
        assert!(params.beta > 0.0);
    }

    #[test]
    fn test_gan_hemt_rf_params() {
        let params = GanHemtParams::rf_gan(10.0);
        assert!(params.vth0 < 0.0); // Depletion mode
        assert!(params.cgs > 0.0);
    }

    #[test]
    fn test_gan_hemt_temp_scaling() {
        let params = GanHemtParams::default();

        let vth_cold = params.vth_at_temp(250.0);
        let vth_hot = params.vth_at_temp(400.0);

        // Vth should decrease with temperature
        assert!(vth_cold > vth_hot);

        let beta_cold = params.beta_at_temp(250.0);
        let beta_hot = params.beta_at_temp(400.0);

        // Beta should decrease with temperature (negative exponent)
        assert!(beta_cold > beta_hot);
    }

    #[test]
    fn test_gan_hemt_ids_off() {
        let hemt = GanHemt::new();

        // Should be off when Vgs < Vth
        let ids = hemt.ids(0.0, 10.0, TREF);
        assert!(ids < 1e-6);
    }

    #[test]
    fn test_gan_hemt_ids_on() {
        let hemt = GanHemt::new();

        // Should conduct when Vgs > Vth
        let ids = hemt.ids(5.0, 10.0, TREF);
        assert!(ids > 1.0);
    }

    #[test]
    fn test_gan_hemt_ids_saturation() {
        let hemt = GanHemt::new();

        // Current should saturate with Vds
        let ids_1v = hemt.ids(5.0, 1.0, TREF);
        let ids_10v = hemt.ids(5.0, 10.0, TREF);
        let ids_100v = hemt.ids(5.0, 100.0, TREF);

        // Current should increase but with diminishing returns
        assert!(ids_10v > ids_1v);
        assert!(ids_100v > ids_10v);

        // But the increase from 10V to 100V should be less than 10x
        assert!(ids_100v < ids_10v * 5.0);
    }

    #[test]
    fn test_gan_hemt_gm() {
        let hemt = GanHemt::new();

        let gm = hemt.gm(5.0, 10.0, TREF);
        assert!(gm > 0.0);
    }

    #[test]
    fn test_gan_hemt_gds() {
        let hemt = GanHemt::new();

        let gds = hemt.gds(5.0, 10.0, TREF);
        assert!(gds > 0.0);
    }

    #[test]
    fn test_gan_hemt_ft() {
        let hemt = GanHemt::new();

        let ft = hemt.ft(5.0, 10.0, TREF);
        assert!(ft > 0.0);
    }

    #[test]
    fn test_gan_hemt_thermal_update() {
        let mut hemt = GanHemt::new();

        hemt.update_thermal(100.0, TREF, 1e-3);
        assert!(hemt.state.tj > TREF);

        // After long time, should approach steady state
        for _ in 0..100 {
            hemt.update_thermal(100.0, TREF, 1e-3);
        }
        let ss_temp = TREF + 100.0 * hemt.params.rth;
        assert!((hemt.state.tj - ss_temp).abs() < 1.0);
    }

    #[test]
    fn test_gan_hemt_soa() {
        let hemt = GanHemt::new();

        // Normal operation: 5A at 20V = 100W, with Rth=1.5 K/W gives 150°C rise
        // Tj = 27°C + 150°C = 177°C > Tjmax (175°C) - borderline but let's use lower power
        // 3A at 20V = 60W, Tj = 27 + 90 = 117°C - well within SOA
        assert!(hemt.is_soa_ok(5.0, 20.0, 3.0));

        // Excessive voltage should fail (near breakdown)
        assert!(!hemt.is_soa_ok(5.0, 600.0, 3.0));

        // Excessive power should fail (thermal)
        // 10A at 50V = 500W, Tj = 27 + 750 = 777°C - way over
        assert!(!hemt.is_soa_ok(5.0, 50.0, 10.0));
    }

    #[test]
    fn test_process_corner_all() {
        let corners = ProcessCorner::all_standard();
        assert_eq!(corners.len(), 5);
    }

    #[test]
    fn test_process_corner_names() {
        assert_eq!(ProcessCorner::Typical.name(), "TT");
        assert_eq!(ProcessCorner::FastFast.name(), "FF");
        assert_eq!(ProcessCorner::SlowSlow.name(), "SS");
    }

    #[test]
    fn test_process_corner_vth_scaling() {
        let vth_ff = ProcessCorner::FastFast.vth_scale();
        let vth_tt = ProcessCorner::Typical.vth_scale();
        let vth_ss = ProcessCorner::SlowSlow.vth_scale();

        // Fast should have lower Vth
        assert!(vth_ff < vth_tt);
        assert!(vth_tt < vth_ss);
    }

    #[test]
    fn test_process_corner_apply() {
        let base = GanHemtParams::default();

        let ff = apply_corner(&base, ProcessCorner::FastFast);
        let ss = apply_corner(&base, ProcessCorner::SlowSlow);

        // Fast corner should have lower Vth and higher beta
        assert!(ff.vth0 < base.vth0);
        assert!(ff.beta > base.beta);

        // Slow corner should have higher Vth and lower beta
        assert!(ss.vth0 > base.vth0);
        assert!(ss.beta < base.beta);
    }

    #[test]
    fn test_process_corner_current() {
        let base = GanHemtParams::default();
        let base_hemt = GanHemt::with_params(base.clone());

        let ff_params = apply_corner(&base, ProcessCorner::FastFast);
        let ff_hemt = GanHemt::with_params(ff_params);

        let ss_params = apply_corner(&base, ProcessCorner::SlowSlow);
        let ss_hemt = GanHemt::with_params(ss_params);

        let vgs = 5.0;
        let vds = 10.0;

        let ids_tt = base_hemt.ids(vgs, vds, TREF);
        let ids_ff = ff_hemt.ids(vgs, vds, TREF);
        let ids_ss = ss_hemt.ids(vgs, vds, TREF);

        // FF should conduct more, SS should conduct less
        assert!(ids_ff > ids_tt);
        assert!(ids_tt > ids_ss);
    }

    #[test]
    fn test_statistical_variation_default() {
        let var = StatisticalVariation::default();
        assert!(var.vth_sigma > 0.0);
        assert!(var.beta_sigma > 0.0);
    }

    #[test]
    fn test_statistical_variation_apply() {
        let base = GanHemtParams::default();
        let var = StatisticalVariation::default();

        // Apply +3 sigma to Vth
        let high_vth = var.apply(&base, &[3.0, 0.0, 0.0, 0.0]);
        assert!(high_vth.vth0 > base.vth0);

        // Apply -3 sigma to Vth
        let low_vth = var.apply(&base, &[-3.0, 0.0, 0.0, 0.0]);
        assert!(low_vth.vth0 < base.vth0);
    }

    #[test]
    fn test_statistical_three_sigma_samples() {
        let base = GanHemtParams::default();
        let var = StatisticalVariation::default();

        let samples = var.three_sigma_samples(&base);
        assert_eq!(samples.len(), 8);

        // Check names
        assert_eq!(samples[0].0, "Vth+3σ");
        assert_eq!(samples[1].0, "Vth-3σ");
    }

    #[test]
    fn test_gan_hemt_state_default() {
        let state = GanHemtState::default();
        assert_eq!(state.vgs, 0.0);
        assert_eq!(state.vds, 0.0);
        assert!((state.tj - TREF).abs() < 0.01);
    }

    #[test]
    fn test_gan_hemt_state_with_bias() {
        let state = GanHemtState::with_bias(5.0, 10.0);
        assert_eq!(state.vgs, 5.0);
        assert_eq!(state.vds, 10.0);
    }
}

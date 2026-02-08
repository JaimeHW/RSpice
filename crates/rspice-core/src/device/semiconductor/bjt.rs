//! BJT (Bipolar Junction Transistor) device model
//!
//! Implements the Ebers-Moll model for NPN and PNP transistors.
//! Supports both large-signal DC and small-signal AC analysis.

use crate::device::traits::{MatrixStamper, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

/// BJT transistor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BjtType {
    Npn,
    Pnp,
}

/// Pre-computed stamp indices for O(1) matrix access (3-terminal device)
/// Layout: [row][col] where row/col are C, B, E
#[derive(Debug, Clone, Default)]
pub struct BjtIndices {
    // Collector row
    pub cc: Option<CscIndex>,
    pub cb: Option<CscIndex>,
    pub ce: Option<CscIndex>,
    // Base row
    pub bc: Option<CscIndex>,
    pub bb: Option<CscIndex>,
    pub be: Option<CscIndex>,
    // Emitter row
    pub ec: Option<CscIndex>,
    pub eb: Option<CscIndex>,
    pub ee: Option<CscIndex>,
}

/// BJT device using the Ebers-Moll model
///
/// Terminal connections:
/// - Collector (C)
/// - Base (B)
/// - Emitter (E)
#[derive(Debug, Clone)]
pub struct Bjt {
    pub name: String,
    pub bjt_type: BjtType,

    // Node connections
    pub node_collector: NodeId,
    pub node_base: NodeId,
    pub node_emitter: NodeId,

    // Model parameters (Ebers-Moll)
    /// Saturation current (IS)
    pub is: Value,
    /// Forward current gain (BF)
    pub bf: Value,
    /// Reverse current gain (BR)
    pub br: Value,
    /// Forward emission coefficient (NF)
    pub nf: Value,
    /// Reverse emission coefficient (NR)
    pub nr: Value,
    /// Thermal voltage (VT = kT/q, ~26mV at 300K)
    pub vt: Value,
    /// Base-emitter built-in potential
    pub vje: Value,
    /// Base-collector built-in potential
    pub vjc: Value,
    /// Forward Early voltage (VAF)
    pub vaf: Value,
    /// Reverse Early voltage (VAR)
    pub var: Value,
    /// Base resistance
    pub rb: Value,
    /// Collector resistance
    pub rc: Value,
    /// Emitter resistance
    pub re: Value,

    // Gummel-Poon charge model parameters
    /// Zero-bias B-E junction capacitance (CJE)
    pub cje: Value,
    /// B-E built-in potential (VJE)
    pub mje: Value,
    /// Zero-bias B-C junction capacitance (CJC)
    pub cjc: Value,
    /// B-C grading coefficient (MJC)
    pub mjc: Value,
    /// Forward transit time (TF)
    pub tf: Value,
    /// Reverse transit time (TR)
    pub tr: Value,
    /// Knee current for high-level injection (IKF)
    pub ikf: Value,
    /// Reverse knee current (IKR)
    pub ikr: Value,

    // Operating point values (for linearization)
    vbe: Value,
    vbc: Value,
    ic: Value,
    ib: Value,
    ie: Value,

    // Previous iteration values (for convergence)
    vbe_prev: Value,
    vbc_prev: Value,

    /// Pre-computed matrix indices for O(1) stamping
    pub indices: BjtIndices,
}

impl Bjt {
    /// Create a new NPN BJT with default 2N2222 parameters
    pub fn new_npn(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Npn, collector, base, emitter)
    }

    /// Create a new PNP BJT with default 2N2907 parameters
    pub fn new_pnp(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Pnp, collector, base, emitter)
    }

    fn new(
        name: String,
        bjt_type: BjtType,
        collector: NodeId,
        base: NodeId,
        emitter: NodeId,
    ) -> Self {
        Self {
            name,
            bjt_type,
            node_collector: collector,
            node_base: base,
            node_emitter: emitter,

            // Default parameters (2N2222-like for NPN)
            is: 1e-14,          // Saturation current
            bf: 200.0,          // Forward current gain
            br: 1.0,            // Reverse current gain
            nf: 1.0,            // Forward emission coefficient
            nr: 1.0,            // Reverse emission coefficient
            vt: 0.02585,        // Thermal voltage at 300K
            vje: 0.75,          // B-E built-in potential
            vjc: 0.75,          // B-C built-in potential
            vaf: 100.0,         // Forward Early voltage
            var: f64::INFINITY, // Reverse Early voltage
            rb: 10.0,           // Base resistance
            rc: 1.0,            // Collector resistance
            re: 0.1,            // Emitter resistance

            // Gummel-Poon parameters
            cje: 1e-12,   // B-E junction capacitance
            mje: 0.33,    // B-E grading coefficient
            cjc: 0.5e-12, // B-C junction capacitance
            mjc: 0.33,    // B-C grading coefficient
            tf: 4e-10,    // Forward transit time (400ps)
            tr: 5e-9,     // Reverse transit time (5ns)
            ikf: 0.1,     // Knee current (100mA)
            ikr: 0.01,    // Reverse knee

            vbe: 0.0,
            vbc: 0.0,
            ic: 0.0,
            ib: 0.0,
            ie: 0.0,
            vbe_prev: 0.0,
            vbc_prev: 0.0,
            indices: BjtIndices::default(),
        }
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        // DC parameters
        if let Some(&v) = params.get("IS") {
            self.is = v;
        }
        if let Some(&v) = params.get("BF") {
            self.bf = v;
        }
        if let Some(&v) = params.get("BR") {
            self.br = v;
        }
        if let Some(&v) = params.get("NF") {
            self.nf = v;
        }
        if let Some(&v) = params.get("NR") {
            self.nr = v;
        }
        if let Some(&v) = params.get("VAF") {
            self.vaf = v;
        }
        if let Some(&v) = params.get("VAR") {
            self.var = v;
        }
        if let Some(&v) = params.get("RB") {
            self.rb = v;
        }
        if let Some(&v) = params.get("RC") {
            self.rc = v;
        }
        if let Some(&v) = params.get("RE") {
            self.re = v;
        }
        // Gummel-Poon charge parameters
        if let Some(&v) = params.get("CJE") {
            self.cje = v;
        }
        if let Some(&v) = params.get("MJE") {
            self.mje = v;
        }
        if let Some(&v) = params.get("CJC") {
            self.cjc = v;
        }
        if let Some(&v) = params.get("MJC") {
            self.mjc = v;
        }
        if let Some(&v) = params.get("TF") {
            self.tf = v;
        }
        if let Some(&v) = params.get("TR") {
            self.tr = v;
        }
        if let Some(&v) = params.get("IKF") {
            self.ikf = v;
        }
        if let Some(&v) = params.get("IKR") {
            self.ikr = v;
        }
        self
    }

    /// Calculate base-emitter junction capacitance
    /// Cbe = CJE / (1 - Vbe/VJE)^MJE + gm * TF
    pub fn cbe(&self, vbe: Value, gm: Value) -> Value {
        let p = self.polarity();
        let v = (p * vbe).min(0.9 * self.vje); // Clamp to avoid singularity
        let cj = self.cje / (1.0 - v / self.vje).powf(self.mje);
        let cd = gm * self.tf; // Diffusion capacitance
        cj + cd
    }

    /// Calculate base-collector junction capacitance
    /// Cbc = CJC / (1 - Vbc/VJC)^MJC
    pub fn cbc(&self, vbc: Value) -> Value {
        let p = self.polarity();
        let v = (p * vbc).min(0.9 * self.vjc); // Clamp to avoid singularity
        self.cjc / (1.0 - v / self.vjc).powf(self.mjc)
    }

    /// Calculate total capacitances for transient analysis
    /// Returns (Cbe, Cbc)
    pub fn junction_capacitances(&self, vbe: Value, vbc: Value) -> (Value, Value) {
        let gm = self.gm(vbe);
        (self.cbe(vbe, gm), self.cbc(vbc))
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let c = self.node_collector;
        let b = self.node_base;
        let e = self.node_emitter;

        // Collector row
        if c > 0 && c > 0 {
            self.indices.cc = matrix.get_index(c - 1, c - 1);
        }
        if c > 0 && b > 0 {
            self.indices.cb = matrix.get_index(c - 1, b - 1);
        }
        if c > 0 && e > 0 {
            self.indices.ce = matrix.get_index(c - 1, e - 1);
        }
        // Base row
        if b > 0 && c > 0 {
            self.indices.bc = matrix.get_index(b - 1, c - 1);
        }
        if b > 0 && b > 0 {
            self.indices.bb = matrix.get_index(b - 1, b - 1);
        }
        if b > 0 && e > 0 {
            self.indices.be = matrix.get_index(b - 1, e - 1);
        }
        // Emitter row
        if e > 0 && c > 0 {
            self.indices.ec = matrix.get_index(e - 1, c - 1);
        }
        if e > 0 && b > 0 {
            self.indices.eb = matrix.get_index(e - 1, b - 1);
        }
        if e > 0 && e > 0 {
            self.indices.ee = matrix.get_index(e - 1, e - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let vc = if self.node_collector == 0 {
            0.0
        } else {
            voltages[self.node_collector - 1]
        };
        let vb = if self.node_base == 0 {
            0.0
        } else {
            voltages[self.node_base - 1]
        };
        let ve = if self.node_emitter == 0 {
            0.0
        } else {
            voltages[self.node_emitter - 1]
        };

        // CRITICAL: Use LIMITED junction voltages from update(), not raw!
        // This is essential for Newton convergence - raw voltages can cause
        // exponential current blowup if Vbe changes too much between iterations.
        // Nagel's algorithm limits the change to prevent divergence.
        let vbe = self.vbe; // Limited in update() via limit_junction_voltage
        let vbc = self.vbc; // Limited in update() via limit_junction_voltage

        // Linearized conductances
        // Calculate currents FIRST so we can use fresh ic for go (fixes lag issue)
        let (ic, ib, _ie) = self.calculate_currents(vbe, vbc);

        let gm = self.gm(vbe);
        let go = self.go(ic); // Use fresh ic, not lagged self.ic
        let gbe = self.gbe(vbe);
        let gbc = self.gbc(vbc);

        // Equivalent currents (companion model for Newton-Raphson)
        let ic_eq = ic - gm * vbe - go * (vc - ve);
        let ib_eq = ib - gbe * vbe - gbc * vbc;

        // Debug logging for Newton convergence analysis (commented for performance)
        // log::trace!(
        //     "BJT {}: Vc={:.3} Vb={:.3} Ve={:.3} | VBE={:.3} VBC={:.3} | Ic={:.2e} Ib={:.2e} | gm={:.2e} go={:.2e} | ic_eq={:.2e} ib_eq={:.2e}",
        //     self.name, vc, vb, ve, vbe, vbc, ic, ib, gm, go, ic_eq, ib_eq
        // );

        // Stamp matrix using direct indexing
        // Collector row
        if let Some(idx) = self.indices.cc {
            matrix.stamp_direct(idx, go + gbc);
        }
        if let Some(idx) = self.indices.cb {
            matrix.stamp_direct(idx, gm - gbc);
        }
        if let Some(idx) = self.indices.ce {
            matrix.stamp_direct(idx, -gm - go);
        }
        // Base row
        if let Some(idx) = self.indices.bc {
            matrix.stamp_direct(idx, -gbc);
        }
        if let Some(idx) = self.indices.bb {
            matrix.stamp_direct(idx, gbe + gbc);
        }
        if let Some(idx) = self.indices.be {
            matrix.stamp_direct(idx, -gbe);
        }
        // Emitter row
        if let Some(idx) = self.indices.ec {
            matrix.stamp_direct(idx, -go);
        }
        if let Some(idx) = self.indices.eb {
            matrix.stamp_direct(idx, -gm - gbe);
        }
        if let Some(idx) = self.indices.ee {
            matrix.stamp_direct(idx, gm + go + gbe);
        }

        // Stamp RHS - current flowing OUT of node is positive in the equation
        // BJT collector current flows from collector to emitter
        if self.node_collector > 0 {
            rhs[self.node_collector - 1] -= ic_eq;
        }
        if self.node_base > 0 {
            rhs[self.node_base - 1] -= ib_eq;
        }
        if self.node_emitter > 0 {
            rhs[self.node_emitter - 1] += ic_eq + ib_eq;
        }
    }

    /// Get polarity multiplier (+1 for NPN, -1 for PNP)
    fn polarity(&self) -> Value {
        match self.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        }
    }

    /// Diode current: I = Is * (exp(V / (n * Vt)) - 1)
    ///
    /// SPICE-style voltage limiting:
    /// - Forward: limit to 80*n*Vt to prevent exp overflow
    /// - Reverse: for V < -5*n*Vt, use linear extrapolation (negligible current)
    fn diode_current(&self, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt; // Forward limit
        let v_rev = -5.0 * nvt; // Reverse limit (around -0.13V at room temp)

        if v > v_crit {
            // Forward saturation - linear extrapolation
            let i_crit = self.is * ((v_crit / nvt).exp() - 1.0);
            let g_crit = (self.is / nvt) * (v_crit / nvt).exp();
            i_crit + g_crit * (v - v_crit)
        } else if v < v_rev {
            // Deep reverse bias - essentially just -Is (negligible)
            -self.is
        } else {
            // Normal operating region
            self.is * ((v / nvt).exp() - 1.0)
        }
    }

    /// Diode conductance: g = Is / (n * Vt) * exp(V / (n * Vt))
    ///
    /// SPICE-style limiting with minimum conductance floor for numerical stability
    fn diode_conductance(&self, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt;
        let v_rev = -5.0 * nvt;

        let g = if v > v_crit {
            // Forward saturation - constant high conductance
            (self.is / nvt) * (v_crit / nvt).exp()
        } else if v < v_rev {
            // Deep reverse bias - minimum conductance
            1e-15
        } else {
            // Normal region
            (self.is / nvt) * (v / nvt).exp()
        };

        // Apply minimum conductance floor
        g.max(1e-15)
    }

    /// Calculate BJT currents using Ebers-Moll with Gummel-Poon enhancements
    ///
    /// Base model is Ebers-Moll for stability. Early voltage and high-injection
    /// effects are applied via go() output conductance and base charge modulation.
    fn calculate_currents(&self, vbe: Value, vbc: Value) -> (Value, Value, Value) {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;

        // Forward and reverse diode currents
        let if_diode = self.diode_current(vbe_eff, self.nf);
        let ir_diode = self.diode_current(vbc_eff, self.nr);

        // High-injection correction (Gummel-Poon)
        // At high currents (If >> IKF), effective beta is reduced
        let ikf_ratio = if_diode.max(0.0) / self.ikf.max(1e-6);
        let ikr_ratio = ir_diode.max(0.0) / self.ikr.max(1e-6);

        // Smooth high-injection factor: approaches 1/sqrt(I/IK) at high currents
        let hf_factor = 1.0 / (1.0 + ikf_ratio.sqrt()).max(0.1);
        let hr_factor = 1.0 / (1.0 + ikr_ratio.sqrt()).max(0.1);

        // Ebers-Moll with high-injection modification
        let ic = p * (if_diode * hf_factor - ir_diode * hr_factor - ir_diode / self.br);
        let ib = p * (if_diode / self.bf + ir_diode / self.br);
        let ie = -(ic + ib); // KCL: Ic + Ib + Ie = 0

        (ic, ib, ie)
    }

    /// Get transconductance gm = dIc/dVbe with Gummel-Poon high-injection
    ///
    /// Includes the reduction in gm at high currents due to high-injection.
    fn gm(&self, vbe: Value) -> Value {
        let p = self.polarity();
        let vbe_eff = p * vbe;

        // Base diode conductance
        let g_diode = self.diode_conductance(vbe_eff, self.nf);
        let if_diode = self.diode_current(vbe_eff, self.nf);

        // High-injection correction factor and its derivative
        let ikf_ratio = if_diode.max(0.0) / self.ikf.max(1e-6);
        let hf = 1.0 / (1.0 + ikf_ratio.sqrt()).max(0.1);

        // d(hf)/dVbe approx for smooth behavior (simplified)
        // At low currents: gm ≈ g_diode
        // At high currents: gm ≈ g_diode * hf (reduced)
        // Apply minimum conductance floor for numerical stability
        (g_diode * hf).max(1e-15)
    }

    /// Get output conductance go = dIc/dVce (Early effect)
    fn go(&self, ic: Value) -> Value {
        if self.vaf.is_finite() {
            ic.abs() / self.vaf
        } else {
            1e-12 // Minimum conductance
        }
    }

    /// Get base-emitter junction conductance
    /// Includes minimum conductance floor for numerical stability
    fn gbe(&self, vbe: Value) -> Value {
        let g = self.diode_conductance(self.polarity() * vbe, self.nf) / self.bf;
        g.max(1e-15) // Minimum floor prevents singular matrix
    }

    /// Get base-collector junction conductance
    /// Includes minimum conductance floor for numerical stability
    fn gbc(&self, vbc: Value) -> Value {
        let g = self.diode_conductance(self.polarity() * vbc, self.nr) / self.br;
        g.max(1e-15) // Minimum floor prevents singular matrix
    }

    /// Junction voltage limiting (Nagel's algorithm from SPICE)
    ///
    /// This is critical for Newton-Raphson convergence with BJTs. The exponential
    /// I-V characteristic means that large voltage changes can cause currents to
    /// blow up, diverging NR. This function limits how much a junction voltage
    /// can change between iterations.
    ///
    /// Algorithm from: L.W. Nagel, "SPICE2: A Computer Program to Simulate
    /// Semiconductor Circuits", UCB/ERL M520, 1975
    ///
    /// Used by commercial simulators: Spectre, HSPICE, PSpice, etc.
    fn limit_junction_voltage(vnew: Value, vold: Value, vt: Value) -> Value {
        // Critical voltage: above this, exponential becomes problematic
        let vcrit = vt * (vt / (core::f64::consts::SQRT_2 * 1e-14)).ln();

        // If new voltage is below critical, accept it (reverse bias is stable)
        if vnew < vcrit {
            return vnew;
        }

        // For forward bias above critical voltage, use logarithmic limiting
        // This prevents huge jumps that would cause exp() overflow
        let delta = vnew - vold;

        if delta.abs() <= 2.0 * vt {
            // Small change - accept as-is
            vnew
        } else if vold >= 0.0 {
            // Forward bias case: limit using logarithmic function
            // New voltage = old + Vt * (1 + ln((delta/Vt - 1).max(1e-10)))
            let arg = (delta / vt - 1.0).abs().max(1e-10);
            if delta > 0.0 {
                vold + vt * (1.0 + arg.ln())
            } else {
                vold - vt * (1.0 + arg.ln())
            }
        } else {
            // Transition from reverse to forward - be conservative
            // Limit to 2*Vt step toward forward bias
            if vnew > 0.0 {
                vt.min(vnew) // Don't exceed Vt on first forward step
            } else {
                vnew.max(vold - 2.0 * vt) // Limit reverse-to-less-reverse
            }
        }
    }
}

impl NonlinearDevice for Bjt {
    fn update(&mut self, voltages: &[Value]) {
        let vc = if self.node_collector == 0 {
            0.0
        } else {
            voltages[self.node_collector - 1]
        };
        let vb = if self.node_base == 0 {
            0.0
        } else {
            voltages[self.node_base - 1]
        };
        let ve = if self.node_emitter == 0 {
            0.0
        } else {
            voltages[self.node_emitter - 1]
        };

        self.vbe_prev = self.vbe;
        self.vbc_prev = self.vbc;

        // Calculate raw junction voltages
        let vbe_raw = vb - ve;
        let vbc_raw = vb - vc;

        // Apply junction voltage limiting (standard SPICE technique)
        // This is critical for BJT convergence - limits how much Vbe/Vbc can
        // change per Newton iteration to prevent exponential current blowup
        self.vbe = Self::limit_junction_voltage(vbe_raw, self.vbe_prev, self.vt);
        self.vbc = Self::limit_junction_voltage(vbc_raw, self.vbc_prev, self.vt);

        let (ic, ib, ie) = self.calculate_currents(self.vbe, self.vbc);
        self.ic = ic;
        self.ib = ib;
        self.ie = ie;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vc = if self.node_collector == 0 {
            0.0
        } else {
            voltages[self.node_collector - 1]
        };
        let _vb = if self.node_base == 0 {
            0.0
        } else {
            voltages[self.node_base - 1]
        };
        let ve = if self.node_emitter == 0 {
            0.0
        } else {
            voltages[self.node_emitter - 1]
        };

        // CRITICAL: Use LIMITED junction voltages from update(), not raw!
        // This is essential for Newton convergence - raw voltages can cause
        // exponential current blowup if Vbe changes too much between iterations.
        // Nagel's algorithm limits the change to prevent divergence.
        let vbe = self.vbe; // Limited in update() via limit_junction_voltage
        let vbc = self.vbc; // Limited in update() via limit_junction_voltage

        // Calculate currents FIRST so we can use fresh ic for go (matches stamp_direct)
        // Using lagged self.ic would cause inconsistent linearization during Newton-Raphson
        let (ic, ib, _ie) = self.calculate_currents(vbe, vbc);

        // Linearized conductances using fresh values
        let gm = self.gm(vbe);
        let go = self.go(ic); // Use fresh ic, not lagged self.ic
        let gbe = self.gbe(vbe);
        let gbc = self.gbc(vbc);

        // Equivalent currents for linearization (companion model)
        let ic_eq = ic - gm * vbe - go * (vc - ve);
        let ib_eq = ib - gbe * vbe - gbc * vbc;

        // Stamp the linearized model
        // Collector node equation
        matrix.stamp(self.node_collector, self.node_collector, go + gbc);
        matrix.stamp(self.node_collector, self.node_base, gm - gbc);
        matrix.stamp(self.node_collector, self.node_emitter, -gm - go);

        // Base node equation
        matrix.stamp(self.node_base, self.node_collector, -gbc);
        matrix.stamp(self.node_base, self.node_base, gbe + gbc);
        matrix.stamp(self.node_base, self.node_emitter, -gbe);

        // Emitter node equation
        matrix.stamp(self.node_emitter, self.node_collector, -go);
        matrix.stamp(self.node_emitter, self.node_base, -gm - gbe);
        matrix.stamp(self.node_emitter, self.node_emitter, gm + go + gbe);

        // Stamp equivalent current sources
        matrix.stamp_rhs(self.node_collector, -ic_eq);
        matrix.stamp_rhs(self.node_base, -ib_eq);
        matrix.stamp_rhs(self.node_emitter, ic_eq + ib_eq);
    }

    /// Check Newton-Raphson convergence using SPICE-style voltage criteria.
    ///
    /// Uses the standard SPICE convergence test:
    ///   |delta(V)| < RELTOL * max(|V_new|, |V_old|) + VNTOL
    ///
    /// `tolerance` is VNTOL from solver configuration.
    fn is_converged(&self, tolerance: Value) -> bool {
        // Industry-standard SPICE convergence parameter
        const RELTOL: Value = 1e-3; // 0.1% relative tolerance

        let vbe_diff = (self.vbe - self.vbe_prev).abs();
        let vbc_diff = (self.vbc - self.vbc_prev).abs();

        // SPICE criterion: |delta(V)| < RELTOL * max(|V_new|, |V_old|) + VNTOL
        let vbe_tol = RELTOL * self.vbe.abs().max(self.vbe_prev.abs()) + tolerance;
        let vbc_tol = RELTOL * self.vbc.abs().max(self.vbc_prev.abs()) + tolerance;

        vbe_diff < vbe_tol && vbc_diff < vbc_tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct CaptureMatrix {
        entries: HashMap<(NodeId, NodeId), Value>,
        rhs: HashMap<NodeId, Value>,
    }

    impl CaptureMatrix {
        fn g(&self, row: NodeId, col: NodeId) -> Value {
            *self.entries.get(&(row, col)).unwrap_or(&0.0)
        }

        fn i(&self, node: NodeId) -> Value {
            *self.rhs.get(&node).unwrap_or(&0.0)
        }
    }

    impl MatrixStamper for CaptureMatrix {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            *self.entries.entry((row, col)).or_insert(0.0) += value;
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            *self.rhs.entry(index).or_insert(0.0) += value;
        }
    }

    // =========================================================================
    // Creation and Configuration Tests
    // =========================================================================

    #[test]
    fn test_bjt_creation() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        assert_eq!(q.bjt_type, BjtType::Npn);
        assert_eq!(q.node_collector, 2);
        assert_eq!(q.node_base, 1);
        assert_eq!(q.node_emitter, 0);
    }

    #[test]
    fn test_pnp_polarity() {
        let npn = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let pnp = Bjt::new_pnp("Q2".to_string(), 2, 1, 0);

        assert_eq!(npn.polarity(), 1.0);
        assert_eq!(pnp.polarity(), -1.0);
    }

    #[test]
    fn test_bjt_default_params() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // 2N2222-like defaults
        assert!((q.is - 1e-14).abs() < 1e-15);
        assert!((q.bf - 200.0).abs() < 1.0);
        assert!((q.br - 1.0).abs() < 0.1);
        assert!((q.vt - 0.02585).abs() < 0.001);
        assert!((q.vaf - 100.0).abs() < 1.0);
    }

    #[test]
    fn test_bjt_gummel_poon_params() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("CJE".to_string(), 2e-12);
        params.insert("CJC".to_string(), 1e-12);
        params.insert("TF".to_string(), 1e-9);
        params.insert("TR".to_string(), 10e-9);
        params.insert("IKF".to_string(), 0.05);

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);

        assert_eq!(q.cje, 2e-12);
        assert_eq!(q.cjc, 1e-12);
        assert_eq!(q.tf, 1e-9);
        assert_eq!(q.tr, 10e-9);
        assert_eq!(q.ikf, 0.05);
    }

    // =========================================================================
    // Operating Region Tests
    // =========================================================================

    #[test]
    fn test_bjt_forward_active() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Typical forward active: Vbe ~ 0.7V, Vbc < 0
        let (ic, ib, ie) = q.calculate_currents(0.7, -5.0);

        // Ic should be positive and >> Ib
        assert!(ic > 0.0);
        assert!(ib > 0.0);
        assert!(ic > ib * 10.0); // Beta > 10

        // KCL check
        assert!((ic + ib + ie).abs() < 1e-12);
    }

    #[test]
    fn test_bjt_cutoff() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Cutoff: Vbe << 0, Vbc << 0
        let (ic, ib, ie) = q.calculate_currents(-0.5, -5.0);

        // All currents should be essentially zero
        assert!(ic.abs() < 1e-12, "Ic should be ~0 in cutoff, got {}", ic);
        assert!(ib.abs() < 1e-12, "Ib should be ~0 in cutoff, got {}", ib);
        assert!(ie.abs() < 1e-12, "Ie should be ~0 in cutoff, got {}", ie);
    }

    #[test]
    fn test_bjt_saturation() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Saturation: Vbe > 0, Vbc > 0 (both junctions forward biased)
        let (ic, ib, _ie) = q.calculate_currents(0.75, 0.65);

        // In saturation, beta is reduced
        let beta_sat = ic.abs() / ib.abs();
        assert!(beta_sat < 50.0, "Beta should be reduced in saturation");
    }

    #[test]
    fn test_bjt_reverse_active() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Reverse active: Vbe < 0, Vbc > 0
        let (ic, ib, ie) = q.calculate_currents(-0.5, 0.7);

        // Current should flow, but Ic negative (emitter acts as collector)
        assert!(ic < 0.0, "Ic should be negative in reverse active");
        assert!((ic + ib + ie).abs() < 1e-12); // KCL
    }

    // =========================================================================
    // Small-Signal Parameter Tests
    // =========================================================================

    #[test]
    fn test_bjt_gm_positive() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let gm = q.gm(0.7);
        assert!(gm > 0.0, "gm should be positive");

        // gm ≈ Ic / Vt for ideal BJT
        // At Vbe=0.7V, Ic is in mA range, so gm should be tens of mS
        assert!(gm > 1e-3, "gm should be > 1mS at Vbe=0.7V");
    }

    #[test]
    fn test_bjt_gm_increases_with_vbe() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let gm1 = q.gm(0.6);
        let gm2 = q.gm(0.7);
        let gm3 = q.gm(0.8);

        assert!(gm2 > gm1, "gm should increase with Vbe");
        assert!(gm3 > gm2, "gm should increase with Vbe");
    }

    #[test]
    fn test_bjt_go_early_effect() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // go = |Ic| / VAF
        let ic = 1e-3; // 1mA
        let go = q.go(ic);

        // With VAF=100V, go = 1mA/100V = 10μS
        assert!((go - 1e-5).abs() < 1e-6, "go should be ~10μS, got {}", go);
    }

    #[test]
    fn test_bjt_gbe_gbc() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let gbe = q.gbe(0.7);
        let gbc = q.gbc(-5.0);

        // gbe should be gm / beta (approximately)
        assert!(gbe > 0.0);

        // gbc should be very small for reverse biased junction
        assert!(
            gbc < gbe * 0.01,
            "gbc << gbe for reverse biased BC junction"
        );
    }

    // =========================================================================
    // Capacitance Tests
    // =========================================================================

    #[test]
    fn test_bjt_junction_capacitances() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Forward active: Vbe=0.7V, Vbc=-5V
        let (cbe, cbc) = q.junction_capacitances(0.7, -5.0);

        // Cbe should be larger (forward biased + diffusion cap from TF*gm)
        assert!(cbe > 1e-12, "Expected Cbe > 1pF, got {:.2e}", cbe);

        // Cbc should be small (reverse biased)
        assert!(
            cbc > 0.1e-12 && cbc < 5e-12,
            "Expected Cbc ~0.5pF, got {:.2e}",
            cbc
        );

        // Cbe should be larger than Cbc in forward active
        assert!(cbe > cbc, "Expected Cbe > Cbc in forward active");
    }

    #[test]
    fn test_bjt_cbe_includes_diffusion() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Diffusion capacitance = gm * TF
        let gm = q.gm(0.7);
        let cbe = q.cbe(0.7, gm);

        // Cd = gm * TF, with TF=400ps and gm~40mS, Cd~16pF
        // Total Cbe should be >> CJE due to diffusion
        assert!(
            cbe > q.cje * 5.0,
            "Cbe should include significant diffusion cap"
        );
    }

    #[test]
    fn test_bjt_cbc_reverse_bias() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Reverse biased BC junction
        let cbc_rev = q.cbc(-10.0);
        let cbc_zero = q.cbc(0.0);

        // Capacitance should decrease with reverse bias
        assert!(cbc_rev < cbc_zero, "Cbc should decrease with reverse bias");
    }

    // =========================================================================
    // PNP Operation Tests
    // =========================================================================

    #[test]
    fn test_pnp_forward_active() {
        let q = Bjt::new_pnp("Q1".to_string(), 2, 1, 0);

        // PNP forward active: Vbe=-0.7V, Vbc=+5V
        let (ic, ib, ie) = q.calculate_currents(-0.7, 5.0);

        // Ic should be negative (current flows out of collector)
        assert!(ic < 0.0, "PNP Ic should be negative, got {}", ic);
        assert!(ib < 0.0, "PNP Ib should be negative, got {}", ib);

        // KCL check
        assert!((ic + ib + ie).abs() < 1e-12);
    }

    #[test]
    fn test_pnp_gm() {
        let q = Bjt::new_pnp("Q1".to_string(), 2, 1, 0);

        // gm should still be positive (magnitude of transconductance)
        let gm = q.gm(-0.7);
        assert!(gm > 0.0, "gm magnitude should be positive for PNP");
    }

    // =========================================================================
    // High-Injection Tests
    // =========================================================================

    #[test]
    fn test_bjt_high_injection_beta_reduction() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("IKF".to_string(), 0.01); // Low knee current for easy testing

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);

        // At low current
        let (ic_low, ib_low, _) = q.calculate_currents(0.6, -5.0);
        let beta_low = ic_low / ib_low;

        // At high current (above IKF)
        let (ic_high, ib_high, _) = q.calculate_currents(0.85, -5.0);
        let beta_high = ic_high / ib_high;

        // Beta should be lower at high current due to high-injection effects
        assert!(
            beta_high < beta_low,
            "Beta should decrease at high injection"
        );
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_bjt_zero_voltages() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        let (ic, ib, ie) = q.calculate_currents(0.0, 0.0);

        // All currents should be near zero
        assert!(ic.abs() < 1e-12);
        assert!(ib.abs() < 1e-12);
        assert!(ie.abs() < 1e-12);
    }

    #[test]
    fn test_bjt_large_reverse_bias() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Large reverse biases should not cause overflow
        let (ic, ib, ie) = q.calculate_currents(-10.0, -100.0);

        assert!(ic.is_finite());
        assert!(ib.is_finite());
        assert!(ie.is_finite());
    }

    #[test]
    fn test_bjt_convergence_check() {
        let mut q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Simulate convergence
        q.vbe = 0.7;
        q.vbc = -5.0;
        q.vbe_prev = 0.7001;
        q.vbc_prev = -5.001;

        // Should be converged within 1mV tolerance
        // With RELTOL=1e-3: tol_vbe = 0.001*0.7 + 0.01 ≈ 0.0107, Δ=0.0001 < 0.0107 ✓
        assert!(q.is_converged(0.01));

        // Even with very tight VNTOL, RELTOL provides adequate tolerance for
        // junction voltages. This is correct SPICE behavior.
        // tol_vbe = 0.001*0.7 + 1e-6 ≈ 0.0007, Δ=0.0001 < 0.0007 ✓
        assert!(q.is_converged(1e-6));

        // To fail convergence, delta must exceed RELTOL*|V| + VNTOL
        q.vbe_prev = 0.71; // Δ=0.01, tol=0.001*0.71+1e-6≈0.00071, 0.01 > 0.00071
        assert!(!q.is_converged(1e-6));
    }

    #[test]
    fn test_bjt_stamp_preserves_kcl_in_jacobian_and_rhs() {
        // Nodes are 1-based in stamps (0 is ground): C=3, B=2, E=1
        let mut q = Bjt::new_npn("Q1".to_string(), 3, 2, 1);
        let voltages = vec![0.2, 0.9, 5.0]; // E, B, C
        q.update(&voltages);

        let mut matrix = CaptureMatrix::default();
        let mut rhs = vec![0.0; 3];
        q.stamp_nonlinear(&voltages, &mut matrix, &mut rhs);

        let cc = matrix.g(3, 3);
        let cb = matrix.g(3, 2);
        let ce = matrix.g(3, 1);
        let bc = matrix.g(2, 3);
        let bb = matrix.g(2, 2);
        let be = matrix.g(2, 1);
        let ec = matrix.g(1, 3);
        let eb = matrix.g(1, 2);
        let ee = matrix.g(1, 1);

        // Emitter row must be the negative of collector+base rows (charge conservation)
        assert!((ec + cc + bc).abs() < 1e-12);
        assert!((eb + cb + bb).abs() < 1e-12);
        assert!((ee + ce + be).abs() < 1e-12);

        // Current-source linearization must also satisfy KCL
        let i_sum = matrix.i(3) + matrix.i(2) + matrix.i(1);
        assert!(i_sum.abs() < 1e-12);
    }
}

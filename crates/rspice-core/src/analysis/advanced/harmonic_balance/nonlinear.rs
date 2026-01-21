//! Nonlinear device handling for Harmonic Balance
//!
//! This module provides HB-specific wrappers for nonlinear semiconductor devices.
//! Devices are evaluated in the time domain (via IFFT) and their currents are
//! transformed back to frequency domain (via FFT) for the Newton iteration.
//!
//! Commercial-grade implementation following Spectre/ADS methodology:
//! - Time-domain device evaluation for accuracy
//! - Analytical Jacobians for convergence
//! - Proper handling of multi-terminal devices

use crate::Value;

/// Thermal voltage at room temperature (300K)
pub const VT_NOMINAL: Value = 0.02585;

//=============================================================================
// HB Diode Wrapper
//=============================================================================

/// Diode device wrapper for Harmonic Balance analysis
///
/// Implements the Shockley diode equation in a form suitable for
/// time-domain evaluation during HB Newton iteration.
#[derive(Debug, Clone)]
pub struct HbDiode {
    /// Device name
    pub name: String,
    /// Anode node index (0-indexed for HB solver)
    pub anode: usize,
    /// Cathode node index (0-indexed for HB solver)
    pub cathode: usize,
    /// Saturation current (A)
    pub is: Value,
    /// Ideality factor
    pub n: Value,
    /// Thermal voltage (V)
    pub vt: Value,
    /// Series resistance (Ω)
    pub rs: Value,
    /// Junction capacitance at zero bias (F)
    pub cj0: Value,
    /// Built-in potential (V)
    pub vj: Value,
    /// Grading coefficient
    pub m: Value,
    /// Transit time (s)
    pub tt: Value,
}

impl Default for HbDiode {
    fn default() -> Self {
        Self {
            name: String::new(),
            anode: 0,
            cathode: 0,
            is: 1e-14,
            n: 1.0,
            vt: VT_NOMINAL,
            rs: 0.0,
            cj0: 0.0,
            vj: 1.0,
            m: 0.5,
            tt: 0.0,
        }
    }
}

impl HbDiode {
    /// Create a new HB diode with default 1N4148 parameters
    pub fn new(name: impl Into<String>, anode: usize, cathode: usize) -> Self {
        Self {
            name: name.into(),
            anode,
            cathode,
            is: 2.52e-9, // 1N4148
            n: 1.752,
            vt: VT_NOMINAL,
            rs: 0.568,
            cj0: 4e-12,
            vj: 0.65,
            m: 0.5,
            tt: 5.6e-9,
        }
    }

    /// Set saturation current
    pub fn with_is(mut self, is: Value) -> Self {
        self.is = is;
        self
    }

    /// Set ideality factor
    pub fn with_n(mut self, n: Value) -> Self {
        self.n = n;
        self
    }

    /// Compute diode current using Shockley equation
    ///
    /// I = Is * (exp(Vd / (n * Vt)) - 1)
    ///
    /// Includes voltage limiting for numerical stability
    #[inline]
    pub fn current(&self, vd: Value) -> Value {
        let vd_limited = vd.min(40.0 * self.n * self.vt);
        let arg = vd_limited / (self.n * self.vt);

        if arg > 40.0 {
            // Linear extrapolation for large forward bias
            self.is * (40.0_f64.exp() + 40.0_f64.exp() * (arg - 40.0))
        } else if arg < -40.0 {
            // Reverse bias: essentially -Is
            -self.is
        } else {
            self.is * (arg.exp() - 1.0)
        }
    }

    /// Compute diode conductance (dI/dV)
    ///
    /// g = Is / (n * Vt) * exp(Vd / (n * Vt))
    #[inline]
    pub fn conductance(&self, vd: Value) -> Value {
        let vd_limited = vd.min(40.0 * self.n * self.vt);
        let arg = vd_limited / (self.n * self.vt);
        let gd = (self.is / (self.n * self.vt)) * arg.exp();

        // Minimum conductance for numerical stability
        gd.max(1e-12)
    }

    /// Compute junction capacitance
    ///
    /// Cj = CJ0 / (1 - Vd/VJ)^M for Vd < FC*VJ
    /// Includes diffusion capacitance: Cd = TT * gd
    pub fn capacitance(&self, vd: Value, gd: Value) -> Value {
        let fc = 0.5; // Forward bias coefficient

        let cj = if vd < fc * self.vj {
            self.cj0 / (1.0 - vd / self.vj).powf(self.m)
        } else {
            // Linear extrapolation for forward bias
            let cj_fc = self.cj0 / (1.0 - fc).powf(self.m);
            cj_fc * (1.0 + self.m * (vd - fc * self.vj) / (self.vj * (1.0 - fc)))
        };

        // Diffusion capacitance
        let cd = self.tt * gd;

        cj + cd
    }

    /// Evaluate diode current for all time samples
    ///
    /// Input: voltage waveform V(t) at N time points
    /// Output: current waveform I(t) at N time points
    pub fn evaluate_time_domain(&self, v_waveform: &[Value]) -> Vec<Value> {
        v_waveform.iter().map(|&vd| self.current(vd)).collect()
    }

    /// Evaluate Jacobian (dI/dV) for all time samples
    pub fn jacobian_time_domain(&self, v_waveform: &[Value]) -> Vec<Value> {
        v_waveform.iter().map(|&vd| self.conductance(vd)).collect()
    }
}

//=============================================================================
// HB BJT Wrapper
//=============================================================================

/// BJT polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbBjtType {
    Npn,
    Pnp,
}

/// BJT device wrapper for Harmonic Balance analysis
///
/// Implements Ebers-Moll model with Gummel-Poon enhancements
/// for time-domain evaluation in HB Newton iteration.
#[derive(Debug, Clone)]
pub struct HbBjt {
    /// Device name
    pub name: String,
    /// Collector node index (0-indexed)
    pub collector: usize,
    /// Base node index (0-indexed)
    pub base: usize,
    /// Emitter node index (0-indexed)
    pub emitter: usize,
    /// Device type
    pub bjt_type: HbBjtType,
    /// Saturation current (A)
    pub is: Value,
    /// Forward current gain
    pub bf: Value,
    /// Reverse current gain
    pub br: Value,
    /// Forward ideality factor
    pub nf: Value,
    /// Reverse ideality factor
    pub nr: Value,
    /// Thermal voltage
    pub vt: Value,
    /// Forward Early voltage (V)
    pub vaf: Value,
    /// Reverse Early voltage (V)
    pub var: Value,
    /// B-E zero-bias capacitance (F)
    pub cje: Value,
    /// B-E built-in potential (V)
    pub vje: Value,
    /// B-E grading coefficient
    pub mje: Value,
    /// B-C zero-bias capacitance (F)
    pub cjc: Value,
    /// B-C built-in potential (V)
    pub vjc: Value,
    /// B-C grading coefficient
    pub mjc: Value,
    /// Forward transit time (s)
    pub tf: Value,
    /// Reverse transit time (s)
    pub tr: Value,
}

impl Default for HbBjt {
    fn default() -> Self {
        Self {
            name: String::new(),
            collector: 0,
            base: 0,
            emitter: 0,
            bjt_type: HbBjtType::Npn,
            is: 1e-14,
            bf: 100.0,
            br: 1.0,
            nf: 1.0,
            nr: 1.0,
            vt: VT_NOMINAL,
            vaf: 100.0,
            var: 100.0,
            cje: 0.0,
            vje: 0.75,
            mje: 0.33,
            cjc: 0.0,
            vjc: 0.75,
            mjc: 0.33,
            tf: 0.0,
            tr: 0.0,
        }
    }
}

impl HbBjt {
    /// Create new NPN BJT with default 2N2222 parameters
    pub fn new_npn(name: impl Into<String>, collector: usize, base: usize, emitter: usize) -> Self {
        Self {
            name: name.into(),
            collector,
            base,
            emitter,
            bjt_type: HbBjtType::Npn,
            is: 3.295e-14,
            bf: 293.1,
            br: 13.72,
            nf: 1.0,
            nr: 1.0,
            vt: VT_NOMINAL,
            vaf: 74.03,
            var: 28.11,
            cje: 22.01e-12,
            vje: 0.632,
            mje: 0.377,
            cjc: 7.306e-12,
            vjc: 0.3958,
            mjc: 0.4509,
            tf: 347.3e-12,
            tr: 10.03e-9,
        }
    }

    /// Create new PNP BJT with default 2N2907 parameters
    pub fn new_pnp(name: impl Into<String>, collector: usize, base: usize, emitter: usize) -> Self {
        Self {
            name: name.into(),
            collector,
            base,
            emitter,
            bjt_type: HbBjtType::Pnp,
            is: 2.2e-14,
            bf: 200.0,
            br: 4.0,
            nf: 1.0,
            nr: 1.0,
            vt: VT_NOMINAL,
            vaf: 50.0,
            var: 30.0,
            cje: 20e-12,
            vje: 0.65,
            mje: 0.33,
            cjc: 10e-12,
            vjc: 0.65,
            mjc: 0.33,
            tf: 500e-12,
            tr: 20e-9,
        }
    }

    /// Get polarity multiplier (+1 for NPN, -1 for PNP)
    #[inline]
    pub fn polarity(&self) -> Value {
        match self.bjt_type {
            HbBjtType::Npn => 1.0,
            HbBjtType::Pnp => -1.0,
        }
    }

    /// Compute diode current
    #[inline]
    fn diode_current(&self, v: Value, n: Value) -> Value {
        let p = self.polarity();
        let vd = p * v;
        let arg = (vd / (n * self.vt)).min(40.0);

        if arg > -40.0 {
            self.is * (arg.exp() - 1.0)
        } else {
            -self.is
        }
    }

    /// Compute diode conductance
    #[inline]
    fn diode_conductance(&self, v: Value, n: Value) -> Value {
        let p = self.polarity();
        let vd = p * v;
        let arg = (vd / (n * self.vt)).min(40.0);
        (self.is / (n * self.vt)) * arg.exp().max(1e-12)
    }

    /// Calculate BJT currents using Ebers-Moll transport model
    ///
    /// Returns (Ic, Ib, Ie) terminal currents
    /// Sign convention: positive current flows INTO terminal
    /// For NPN in forward active: Ic > 0, Ib > 0, Ie < 0
    /// KCL is satisfied by construction: Ic + Ib + Ie = 0
    pub fn calculate_currents(&self, vbe: Value, vbc: Value) -> (Value, Value, Value) {
        let p = self.polarity();

        // Effective junction voltages
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;

        // Junction exponential terms with limiting
        let arg_be = (vbe_eff / (self.nf * self.vt)).clamp(-40.0, 40.0);
        let arg_bc = (vbc_eff / (self.nr * self.vt)).clamp(-40.0, 40.0);

        let exp_be = arg_be.exp();
        let exp_bc = arg_bc.exp();

        // Ebers-Moll transport model currents
        // IF = Is * (exp(Vbe/Vt) - 1): forward BE junction current
        // IR = Is * (exp(Vbc/Vt) - 1): reverse BC junction current
        let i_f = self.is * (exp_be - 1.0);
        let i_r = self.is * (exp_bc - 1.0);

        // Transport current (the current that gets amplified)
        let i_ct = self.is * (exp_be - exp_bc);

        // Early effect on collector current
        let vce_eff = vbe_eff - vbc_eff;
        let early_factor = if self.vaf > 0.0 && vce_eff > 0.0 {
            1.0 + vce_eff / self.vaf
        } else {
            1.0
        };

        // Terminal currents (Ebers-Moll with Early effect):
        // Ic = ICT - IR/βR = Is*(exp(Vbe/Vt) - exp(Vbc/Vt)) - Is*(exp(Vbc/Vt)-1)/βR
        // Ib = IF/βF + IR/βR = Is*(exp(Vbe/Vt)-1)/βF + Is*(exp(Vbc/Vt)-1)/βR
        // Ie = -(Ic + Ib) from KCL
        let ic = p * (i_ct * early_factor - i_r / self.br);
        let ib = p * (i_f / self.bf + i_r / self.br);
        let ie = -(ic + ib); // KCL: Ic + Ib + Ie = 0

        (ic, ib, ie)
    }

    /// Get transconductance gm = dIc/dVbe
    pub fn gm(&self, vbe: Value) -> Value {
        let gbe = self.diode_conductance(vbe, self.nf);
        gbe * self.bf / (1.0 + self.bf)
    }

    /// Get output conductance go = dIc/dVce (Early effect)
    pub fn go(&self, ic: Value) -> Value {
        if self.vaf > 0.0 && ic.abs() > 1e-15 {
            ic.abs() / self.vaf
        } else {
            1e-12
        }
    }

    /// Get base-emitter junction conductance
    pub fn gbe(&self, vbe: Value) -> Value {
        self.diode_conductance(vbe, self.nf) / self.bf
    }

    /// Get base-collector junction conductance
    pub fn gbc(&self, vbc: Value) -> Value {
        self.diode_conductance(vbc, self.nr) / self.br
    }

    /// Evaluate BJT currents for all time samples
    ///
    /// Input: vbe and vbc waveforms at N time points
    /// Output: (ic, ib, ie) current waveforms
    pub fn evaluate_time_domain(
        &self,
        vbe_waveform: &[Value],
        vbc_waveform: &[Value],
    ) -> (Vec<Value>, Vec<Value>, Vec<Value>) {
        let n = vbe_waveform.len();
        let mut ic = Vec::with_capacity(n);
        let mut ib = Vec::with_capacity(n);
        let mut ie = Vec::with_capacity(n);

        for i in 0..n {
            let (ic_t, ib_t, ie_t) = self.calculate_currents(vbe_waveform[i], vbc_waveform[i]);
            ic.push(ic_t);
            ib.push(ib_t);
            ie.push(ie_t);
        }

        (ic, ib, ie)
    }
}

//=============================================================================
// HB MOSFET Wrapper
//=============================================================================

/// MOSFET type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HbMosType {
    Nmos,
    Pmos,
}

/// MOSFET device wrapper for Harmonic Balance analysis
///
/// Implements Level 1 square-law model for time-domain HB evaluation.
/// Higher-level models (BSIM4) could be added as needed.
#[derive(Debug, Clone)]
pub struct HbMosfet {
    /// Device name
    pub name: String,
    /// Drain node index (0-indexed)
    pub drain: usize,
    /// Gate node index (0-indexed)
    pub gate: usize,
    /// Source node index (0-indexed)
    pub source: usize,
    /// Bulk node index (0-indexed)
    pub bulk: usize,
    /// Device type
    pub mos_type: HbMosType,
    /// Threshold voltage (V)
    pub vth0: Value,
    /// Transconductance parameter (A/V²)
    pub kp: Value,
    /// Channel length modulation (1/V)
    pub lambda: Value,
    /// Gate-source overlap capacitance (F)
    pub cgso: Value,
    /// Gate-drain overlap capacitance (F)
    pub cgdo: Value,
    /// Gate-bulk capacitance (F)
    pub cgbo: Value,
    /// Channel width (m)
    pub w: Value,
    /// Channel length (m)
    pub l: Value,
}

impl Default for HbMosfet {
    fn default() -> Self {
        Self {
            name: String::new(),
            drain: 0,
            gate: 0,
            source: 0,
            bulk: 0,
            mos_type: HbMosType::Nmos,
            vth0: 0.7,
            kp: 110e-6, // µA/V² for NMOS
            lambda: 0.04,
            cgso: 0.0,
            cgdo: 0.0,
            cgbo: 0.0,
            w: 10e-6,
            l: 1e-6,
        }
    }
}

impl HbMosfet {
    /// Create new NMOS with typical parameters
    pub fn new_nmos(
        name: impl Into<String>,
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
    ) -> Self {
        Self {
            name: name.into(),
            drain,
            gate,
            source,
            bulk,
            mos_type: HbMosType::Nmos,
            vth0: 0.7,
            kp: 110e-6,
            lambda: 0.04,
            w: 10e-6,
            l: 1e-6,
            ..Default::default()
        }
    }

    /// Create new PMOS with typical parameters
    pub fn new_pmos(
        name: impl Into<String>,
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
    ) -> Self {
        Self {
            name: name.into(),
            drain,
            gate,
            source,
            bulk,
            mos_type: HbMosType::Pmos,
            vth0: -0.7,
            kp: 50e-6, // Lower mobility for PMOS
            lambda: 0.04,
            w: 10e-6,
            l: 1e-6,
            ..Default::default()
        }
    }

    /// Get polarity multiplier
    #[inline]
    pub fn polarity(&self) -> Value {
        match self.mos_type {
            HbMosType::Nmos => 1.0,
            HbMosType::Pmos => -1.0,
        }
    }

    /// Compute drain current using Level 1 model
    ///
    /// Cutoff: Vgs < Vth → Id = 0
    /// Linear: Vds < Vgs - Vth → Id = β[(Vgs-Vth)Vds - Vds²/2](1 + λVds)
    /// Saturation: Vds ≥ Vgs - Vth → Id = β(Vgs-Vth)²/2 (1 + λVds)
    pub fn drain_current(&self, vgs: Value, vds: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;

        let vth = p * self.vth0;
        let beta = self.kp * self.w / self.l;

        let vov = vgs_eff - vth; // Overdrive voltage

        if vov <= 0.0 {
            // Cutoff region
            0.0
        } else if vds_eff < vov {
            // Linear/triode region
            let id =
                beta * (vov * vds_eff - 0.5 * vds_eff * vds_eff) * (1.0 + self.lambda * vds_eff);
            p * id
        } else {
            // Saturation region
            let id = 0.5 * beta * vov * vov * (1.0 + self.lambda * vds_eff);
            p * id
        }
    }

    /// Compute transconductance gm = dId/dVgs
    pub fn gm(&self, vgs: Value, vds: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;
        let vth = p * self.vth0;
        let beta = self.kp * self.w / self.l;
        let vov = vgs_eff - vth;

        if vov <= 0.0 {
            1e-12
        } else if vds_eff < vov {
            // Linear region
            beta * vds_eff * (1.0 + self.lambda * vds_eff)
        } else {
            // Saturation region
            beta * vov * (1.0 + self.lambda * vds_eff)
        }
    }

    /// Compute output conductance gds = dId/dVds
    pub fn gds(&self, vgs: Value, vds: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;
        let vth = p * self.vth0;
        let beta = self.kp * self.w / self.l;
        let vov = vgs_eff - vth;

        if vov <= 0.0 {
            1e-12
        } else if vds_eff < vov {
            // Linear region
            beta * (vov - vds_eff) * (1.0 + self.lambda * vds_eff)
                + beta * (vov * vds_eff - 0.5 * vds_eff * vds_eff) * self.lambda
        } else {
            // Saturation region (λ effect)
            0.5 * beta * vov * vov * self.lambda
        }
        .max(1e-12)
    }

    /// Evaluate MOSFET currents for all time samples
    pub fn evaluate_time_domain(
        &self,
        vgs_waveform: &[Value],
        vds_waveform: &[Value],
    ) -> Vec<Value> {
        vgs_waveform
            .iter()
            .zip(vds_waveform.iter())
            .map(|(&vgs, &vds)| self.drain_current(vgs, vds))
            .collect()
    }
}

//=============================================================================
// Unified HB Device Enum
//=============================================================================

/// Unified enum for all nonlinear devices in HB analysis
#[derive(Debug, Clone)]
pub enum HbDevice {
    Diode(HbDiode),
    Bjt(HbBjt),
    Mosfet(HbMosfet),
}

impl HbDevice {
    /// Get device name
    pub fn name(&self) -> &str {
        match self {
            HbDevice::Diode(d) => &d.name,
            HbDevice::Bjt(b) => &b.name,
            HbDevice::Mosfet(m) => &m.name,
        }
    }

    /// Get number of terminals
    pub fn num_terminals(&self) -> usize {
        match self {
            HbDevice::Diode(_) => 2,
            HbDevice::Bjt(_) => 3,
            HbDevice::Mosfet(_) => 4,
        }
    }

    /// Get terminal node indices
    pub fn terminals(&self) -> Vec<usize> {
        match self {
            HbDevice::Diode(d) => vec![d.anode, d.cathode],
            HbDevice::Bjt(b) => vec![b.collector, b.base, b.emitter],
            HbDevice::Mosfet(m) => vec![m.drain, m.gate, m.source, m.bulk],
        }
    }
}

//=============================================================================
// Nonlinear Device Collection
//=============================================================================

/// Collection of nonlinear devices for HB analysis
#[derive(Debug, Default)]
pub struct HbDeviceCollection {
    pub diodes: Vec<HbDiode>,
    pub bjts: Vec<HbBjt>,
    pub mosfets: Vec<HbMosfet>,
}

impl HbDeviceCollection {
    /// Create new empty collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a diode
    pub fn add_diode(&mut self, diode: HbDiode) {
        self.diodes.push(diode);
    }

    /// Add a BJT
    pub fn add_bjt(&mut self, bjt: HbBjt) {
        self.bjts.push(bjt);
    }

    /// Add a MOSFET
    pub fn add_mosfet(&mut self, mosfet: HbMosfet) {
        self.mosfets.push(mosfet);
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.diodes.is_empty() && self.bjts.is_empty() && self.mosfets.is_empty()
    }

    /// Total device count
    pub fn len(&self) -> usize {
        self.diodes.len() + self.bjts.len() + self.mosfets.len()
    }

    /// Convert to unified device vector
    pub fn to_devices(&self) -> Vec<HbDevice> {
        let mut devices = Vec::with_capacity(self.len());
        for d in &self.diodes {
            devices.push(HbDevice::Diode(d.clone()));
        }
        for b in &self.bjts {
            devices.push(HbDevice::Bjt(b.clone()));
        }
        for m in &self.mosfets {
            devices.push(HbDevice::Mosfet(m.clone()));
        }
        devices
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    // =========================================================================
    // Diode Tests
    // =========================================================================

    #[test]
    fn test_hb_diode_creation() {
        let diode = HbDiode::new("D1", 0, 1);
        assert_eq!(diode.name, "D1");
        assert_eq!(diode.anode, 0);
        assert_eq!(diode.cathode, 1);
        assert!(diode.is > 0.0);
    }

    #[test]
    fn test_hb_diode_forward_bias() {
        let diode = HbDiode::new("D1", 0, 1);

        // Forward bias should produce positive current
        let i_0v = diode.current(0.0);
        let i_0_6v = diode.current(0.6);
        let i_0_7v = diode.current(0.7);

        assert!(i_0v.abs() < 1e-10, "Zero bias should have ~zero current");
        assert!(i_0_6v > 1e-6, "0.6V should have significant current");
        assert!(i_0_7v > i_0_6v, "Higher voltage should have higher current");
    }

    #[test]
    fn test_hb_diode_reverse_bias() {
        let diode = HbDiode::new("D1", 0, 1);

        // Reverse bias should have ~-Is current
        let i_rev = diode.current(-1.0);
        assert!(i_rev < 0.0, "Reverse current should be negative");
        assert!(i_rev.abs() < 1e-8, "Reverse current should be small");
    }

    #[test]
    fn test_hb_diode_conductance_positive() {
        let diode = HbDiode::new("D1", 0, 1);

        // Conductance should always be positive
        assert!(diode.conductance(0.0) > 0.0);
        assert!(diode.conductance(0.6) > 0.0);
        assert!(diode.conductance(-1.0) > 0.0);

        // Higher bias = higher conductance
        assert!(diode.conductance(0.6) > diode.conductance(0.3));
    }

    #[test]
    fn test_hb_diode_time_domain_evaluation() {
        let diode = HbDiode::new("D1", 0, 1);

        // Create a sinusoidal voltage waveform
        let n_samples = 64;
        let v_waveform: Vec<Value> = (0..n_samples)
            .map(|i| 0.3 * (2.0 * PI * i as f64 / n_samples as f64).sin())
            .collect();

        let i_waveform = diode.evaluate_time_domain(&v_waveform);

        assert_eq!(i_waveform.len(), n_samples);

        // Positive half-cycle should have positive current
        // Negative half-cycle should have ~zero current (reverse bias)
        let i_max = i_waveform.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let i_min = i_waveform.iter().cloned().fold(f64::INFINITY, f64::min);

        assert!(i_max > 0.0, "Should have positive current in forward bias");
        assert!(i_min.abs() < i_max, "Reverse current should be smaller");
    }

    #[test]
    fn test_hb_diode_jacobian_time_domain() {
        let diode = HbDiode::new("D1", 0, 1);

        let v_waveform = vec![0.0, 0.3, 0.6, -0.3, -0.6];
        let g_waveform = diode.jacobian_time_domain(&v_waveform);

        assert_eq!(g_waveform.len(), v_waveform.len());

        // All conductances should be positive
        for g in &g_waveform {
            assert!(*g > 0.0);
        }
    }

    #[test]
    fn test_hb_diode_capacitance() {
        let diode = HbDiode::new("D1", 0, 1).with_is(1e-14);

        let gd = diode.conductance(0.5);
        let cj = diode.capacitance(0.5, gd);

        // Capacitance should be positive
        assert!(cj > 0.0);
    }

    // =========================================================================
    // BJT Tests
    // =========================================================================

    #[test]
    fn test_hb_bjt_creation() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);
        assert_eq!(bjt.name, "Q1");
        assert_eq!(bjt.collector, 0);
        assert_eq!(bjt.base, 1);
        assert_eq!(bjt.emitter, 2);
        assert_eq!(bjt.bjt_type, HbBjtType::Npn);
    }

    #[test]
    fn test_hb_bjt_pnp_creation() {
        let bjt = HbBjt::new_pnp("Q2", 0, 1, 2);
        assert_eq!(bjt.bjt_type, HbBjtType::Pnp);
        assert_eq!(bjt.polarity(), -1.0);
    }

    #[test]
    fn test_hb_bjt_forward_active() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // Forward active: Vbe > 0, Vbc < 0
        let (ic, ib, ie) = bjt.calculate_currents(0.65, -5.0);

        assert!(ic > 0.0, "Collector current should be positive");
        assert!(ib > 0.0, "Base current should be positive");
        assert!(ie < 0.0, "Emitter current should be negative (into device)");

        // KCL: Ic + Ib + Ie = 0
        let kcl = ic + ib + ie;
        assert!(kcl.abs() < 1e-15, "KCL should be satisfied");
    }

    #[test]
    fn test_hb_bjt_cutoff() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // Cutoff: Vbe < 0
        let (ic, ib, ie) = bjt.calculate_currents(-0.1, -5.0);

        assert!(ic.abs() < 1e-10, "Cutoff Ic should be very small");
        assert!(ib.abs() < 1e-10, "Cutoff Ib should be very small");
    }

    #[test]
    fn test_hb_bjt_gm() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        let gm_low = bjt.gm(0.5);
        let gm_high = bjt.gm(0.65);

        assert!(gm_high > gm_low, "Higher Vbe should give higher gm");
        assert!(gm_high > 0.0);
    }

    #[test]
    fn test_hb_bjt_time_domain_evaluation() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // Simple waveforms
        let vbe = vec![0.0, 0.3, 0.6, 0.65, 0.5];
        let vbc = vec![-5.0, -5.0, -5.0, -5.0, -5.0];

        let (ic, ib, ie) = bjt.evaluate_time_domain(&vbe, &vbc);

        assert_eq!(ic.len(), vbe.len());
        assert_eq!(ib.len(), vbe.len());
        assert_eq!(ie.len(), vbe.len());
    }

    // =========================================================================
    // MOSFET Tests
    // =========================================================================

    #[test]
    fn test_hb_mosfet_creation() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);
        assert_eq!(mos.name, "M1");
        assert_eq!(mos.drain, 0);
        assert_eq!(mos.gate, 1);
        assert_eq!(mos.source, 2);
        assert_eq!(mos.bulk, 3);
        assert_eq!(mos.mos_type, HbMosType::Nmos);
    }

    #[test]
    fn test_hb_mosfet_cutoff() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        // Vgs < Vth should give zero current
        let id = mos.drain_current(0.3, 1.0);
        assert!(id.abs() < 1e-15, "Cutoff Id should be zero");
    }

    #[test]
    fn test_hb_mosfet_saturation() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        // Vgs = 1.5V, Vds = 2V (saturation)
        let vgs = 1.5;
        let vds = 2.0;
        let vth = 0.7;

        // Should be in saturation
        assert!(vds > vgs - vth);

        let id = mos.drain_current(vgs, vds);
        assert!(id > 0.0, "Saturation Id should be positive");

        // Verify square-law behavior: Id ∝ (Vgs - Vth)²
        let id2 = mos.drain_current(vgs + 0.1, vds);
        assert!(id2 > id, "Higher Vgs should give higher Id");
    }

    #[test]
    fn test_hb_mosfet_linear() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        // Vgs = 2V, Vds = 0.5V (linear)
        let vgs = 2.0;
        let vds = 0.5;
        let vth = 0.7;

        // Should be in linear
        assert!(vds < vgs - vth);

        let id = mos.drain_current(vgs, vds);
        assert!(id > 0.0, "Linear Id should be positive");
    }

    #[test]
    fn test_hb_mosfet_gm_gds() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        let gm = mos.gm(1.5, 2.0);
        let gds = mos.gds(1.5, 2.0);

        assert!(gm > 0.0, "gm should be positive");
        assert!(gds > 0.0, "gds should be positive");
        assert!(gm > gds, "gm should typically be larger than gds");
    }

    #[test]
    fn test_hb_mosfet_pmos() {
        let mos = HbMosfet::new_pmos("M1", 0, 1, 2, 3);

        // PMOS: negative Vgs, negative Vds
        let id = mos.drain_current(-1.5, -2.0);
        assert!(id < 0.0, "PMOS Id should be negative");
    }

    #[test]
    fn test_hb_mosfet_time_domain() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        let vgs = vec![0.5, 1.0, 1.5, 2.0, 1.5];
        let vds = vec![2.0, 2.0, 2.0, 2.0, 2.0];

        let id = mos.evaluate_time_domain(&vgs, &vds);
        assert_eq!(id.len(), vgs.len());
    }

    // =========================================================================
    // Device Collection Tests
    // =========================================================================

    #[test]
    fn test_hb_device_collection() {
        let mut devices = HbDeviceCollection::new();
        assert!(devices.is_empty());

        devices.add_diode(HbDiode::new("D1", 0, 1));
        devices.add_bjt(HbBjt::new_npn("Q1", 2, 3, 4));
        devices.add_mosfet(HbMosfet::new_nmos("M1", 5, 6, 7, 8));

        assert!(!devices.is_empty());
        assert_eq!(devices.len(), 3);
    }

    #[test]
    fn test_hb_device_enum() {
        let diode = HbDevice::Diode(HbDiode::new("D1", 0, 1));
        assert_eq!(diode.name(), "D1");
        assert_eq!(diode.num_terminals(), 2);
        assert_eq!(diode.terminals(), vec![0, 1]);

        let bjt = HbDevice::Bjt(HbBjt::new_npn("Q1", 0, 1, 2));
        assert_eq!(bjt.num_terminals(), 3);

        let mos = HbDevice::Mosfet(HbMosfet::new_nmos("M1", 0, 1, 2, 3));
        assert_eq!(mos.num_terminals(), 4);
    }

    // =========================================================================
    // Numerical Accuracy Tests
    // =========================================================================

    #[test]
    fn test_diode_conductance_is_derivative() {
        let diode = HbDiode::new("D1", 0, 1).with_is(1e-14).with_n(1.0);

        // Numerical derivative should match analytical
        let vd = 0.5;
        let h = 1e-6;

        let g_analytical = diode.conductance(vd);
        let g_numerical = (diode.current(vd + h) - diode.current(vd - h)) / (2.0 * h);

        let error = (g_analytical - g_numerical).abs() / g_analytical;
        assert!(error < 0.01, "Conductance error: {}", error);
    }

    #[test]
    fn test_mosfet_gm_is_derivative() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        let vgs = 1.5;
        let vds = 2.0;
        let h = 1e-6;

        let gm_analytical = mos.gm(vgs, vds);
        let gm_numerical =
            (mos.drain_current(vgs + h, vds) - mos.drain_current(vgs - h, vds)) / (2.0 * h);

        let error = (gm_analytical - gm_numerical).abs() / gm_analytical.max(1e-12);
        assert!(error < 0.01, "gm error: {}", error);
    }

    #[test]
    fn test_mosfet_gds_is_derivative() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        let vgs = 1.5;
        let vds = 2.0;
        let h = 1e-6;

        let gds_analytical = mos.gds(vgs, vds);
        let gds_numerical =
            (mos.drain_current(vgs, vds + h) - mos.drain_current(vgs, vds - h)) / (2.0 * h);

        let error = (gds_analytical - gds_numerical).abs() / gds_analytical.max(1e-12);
        assert!(error < 0.05, "gds error: {}", error); // Looser tolerance due to nonlinearity
    }

    // =========================================================================
    // Additional Comprehensive Tests
    // =========================================================================

    #[test]
    fn test_bjt_saturation_region() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // Saturation: both junctions forward biased (Vbe > 0, Vbc > 0)
        let (ic, ib, ie) = bjt.calculate_currents(0.65, 0.5);

        // In saturation, Ic should be smaller relative to Ib
        // KCL must still hold
        let kcl = ic + ib + ie;
        assert!(kcl.abs() < 1e-15, "KCL should be satisfied in saturation");
        assert!(ib > 0.0, "Base current should be positive in saturation");
    }

    #[test]
    fn test_bjt_reverse_active() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // Reverse active: Vbe < 0, Vbc > 0
        let (ic, ib, ie) = bjt.calculate_currents(-0.1, 0.65);

        // KCL must hold
        let kcl = ic + ib + ie;
        assert!(
            kcl.abs() < 1e-15,
            "KCL should be satisfied in reverse active"
        );
    }

    #[test]
    fn test_pnp_forward_active() {
        let bjt = HbBjt::new_pnp("Q1", 0, 1, 2);

        // PNP forward active: Vbe < 0 (from base to emitter, opposite polarity)
        let (ic, ib, ie) = bjt.calculate_currents(-0.65, 5.0);

        // For PNP: currents have opposite sign to NPN
        assert!(ic < 0.0, "PNP Ic should be negative");
        assert!(ib < 0.0, "PNP Ib should be negative");
        assert!(ie > 0.0, "PNP Ie should be positive");

        // KCL must hold
        let kcl = ic + ib + ie;
        assert!(kcl.abs() < 1e-15, "KCL should be satisfied for PNP");
    }

    #[test]
    fn test_bjt_current_gain() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // In forward active, Ic/Ib should be approximately beta
        let (ic, ib, _ie) = bjt.calculate_currents(0.65, -5.0);

        let beta_measured = ic / ib;
        // Should be close to bf (but not exact due to Early effect, etc.)
        assert!(
            beta_measured > 100.0 && beta_measured < 500.0,
            "Beta should be in reasonable range: got {}",
            beta_measured
        );
    }

    #[test]
    fn test_diode_large_forward_bias() {
        let diode = HbDiode::new("D1", 0, 1);

        // Test at high forward bias (should be limited to prevent overflow)
        let i_high = diode.current(2.0);
        assert!(
            i_high.is_finite(),
            "Current should be finite at high forward bias"
        );
        assert!(i_high > 0.0, "Current should be positive");

        let g_high = diode.conductance(2.0);
        assert!(g_high.is_finite(), "Conductance should be finite");
    }

    #[test]
    fn test_diode_large_reverse_bias() {
        let diode = HbDiode::new("D1", 0, 1);

        // Test at high reverse bias
        let i_rev = diode.current(-100.0);
        assert!(i_rev.is_finite(), "Reverse current should be finite");
        assert!(
            (i_rev + diode.is).abs() < 1e-10,
            "Reverse current should be ~ -Is"
        );
    }

    #[test]
    fn test_mosfet_body_effect_simulation() {
        // Test that MOSFET works with different threshold voltages
        // (simulating body effect by changing Vth)
        let mut mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        let id_nominal = mos.drain_current(1.5, 2.0);

        // Increase threshold (simulating body effect)
        mos.vth0 = 0.9;
        let id_body_effect = mos.drain_current(1.5, 2.0);

        assert!(
            id_body_effect < id_nominal,
            "Higher Vth should reduce drain current"
        );
    }

    #[test]
    fn test_mosfet_triode_to_saturation_transition() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        let vgs = 2.0;
        let vth = 0.7;
        let vov = vgs - vth; // 1.3V

        // Just below saturation boundary
        let id_triode = mos.drain_current(vgs, vov - 0.1);

        // Just above saturation boundary
        let id_sat = mos.drain_current(vgs, vov + 0.1);

        // Currents should be close at the boundary
        let diff = (id_sat - id_triode).abs() / id_sat;
        assert!(
            diff < 0.2,
            "Current should be continuous at boundary: diff = {}",
            diff
        );
    }

    #[test]
    fn test_diode_many_bias_points() {
        let diode = HbDiode::new("D1", 0, 1);

        // Test many bias points to verify monotonicity
        let mut prev_i = diode.current(-2.0);
        for v in (-20..=10).map(|x| x as f64 * 0.1) {
            let i = diode.current(v);
            assert!(
                i >= prev_i || (i - prev_i).abs() < 1e-15,
                "Diode current should be monotonic: V={}, I={}, prev={}",
                v,
                i,
                prev_i
            );
            prev_i = i;
        }
    }

    #[test]
    fn test_mosfet_many_bias_points() {
        let mos = HbMosfet::new_nmos("M1", 0, 1, 2, 3);

        // Test that Id is monotonic in Vgs (for fixed Vds in saturation)
        let vds = 2.0;
        let mut prev_id = 0.0;
        for vgs in (0..=30).map(|x| x as f64 * 0.1) {
            let id = mos.drain_current(vgs, vds);
            assert!(
                id >= prev_id || (id - prev_id).abs() < 1e-15,
                "MOSFET Id should be monotonic in Vgs"
            );
            prev_id = id;
        }
    }

    #[test]
    fn test_bjt_gm_go_positive() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // All small-signal parameters should be positive in forward active
        let (ic, _ib, _ie) = bjt.calculate_currents(0.65, -5.0);
        let gm = bjt.gm(0.65);
        let go = bjt.go(ic);
        let gbe = bjt.gbe(0.65);
        let gbc = bjt.gbc(-5.0);

        assert!(gm > 0.0, "gm should be positive: {}", gm);
        assert!(go > 0.0, "go should be positive: {}", go);
        assert!(gbe > 0.0, "gbe should be positive: {}", gbe);
        assert!(gbc > 0.0, "gbc should be positive: {}", gbc);
    }

    #[test]
    fn test_device_collection_to_devices() {
        let mut collection = HbDeviceCollection::new();
        collection.add_diode(HbDiode::new("D1", 0, 1));
        collection.add_diode(HbDiode::new("D2", 1, 2));
        collection.add_bjt(HbBjt::new_npn("Q1", 3, 4, 5));
        collection.add_mosfet(HbMosfet::new_nmos("M1", 6, 7, 8, 9));

        let devices = collection.to_devices();
        assert_eq!(devices.len(), 4);

        // Check order: diodes first, then BJTs, then MOSFETs
        assert!(matches!(devices[0], HbDevice::Diode(_)));
        assert!(matches!(devices[1], HbDevice::Diode(_)));
        assert!(matches!(devices[2], HbDevice::Bjt(_)));
        assert!(matches!(devices[3], HbDevice::Mosfet(_)));
    }

    #[test]
    fn test_diode_current_symmetry() {
        // Two diodes with same parameters should give same current
        let d1 = HbDiode::new("D1", 0, 1);
        let d2 = HbDiode::new("D2", 2, 3);

        let v = 0.6;
        assert_eq!(d1.current(v), d2.current(v));
        assert_eq!(d1.conductance(v), d2.conductance(v));
    }

    #[test]
    fn test_bjt_kcl_many_conditions() {
        let bjt = HbBjt::new_npn("Q1", 0, 1, 2);

        // Test KCL across many operating points
        for vbe in (-5..=10).map(|x| x as f64 * 0.1) {
            for vbc in (-10..=5).map(|x| x as f64 * 0.5) {
                let (ic, ib, ie) = bjt.calculate_currents(vbe, vbc);
                let kcl = ic + ib + ie;
                assert!(
                    kcl.abs() < 1e-14,
                    "KCL violated at Vbe={}, Vbc={}: sum={}",
                    vbe,
                    vbc,
                    kcl
                );
            }
        }
    }
}

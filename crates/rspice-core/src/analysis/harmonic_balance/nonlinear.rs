//! Nonlinear device handling for Harmonic Balance
//!
//! This module provides HB-specific wrappers for nonlinear semiconductor devices.
//! Devices are evaluated in the time domain (via IFFT) and their currents are
//! transformed back to frequency domain (via FFT) for the Newton iteration.
//!
//! Advanced implementation following standard methodology:
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

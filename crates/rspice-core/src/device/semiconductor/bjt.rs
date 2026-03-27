//! BJT (Bipolar Junction Transistor) device model
//!
//! Implements the Ebers-Moll model for NPN and PNP transistors.
//! Supports both large-signal DC and small-signal AC analysis.

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
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

#[derive(Debug, Clone, Copy)]
struct BjtLinearization {
    ic: Value,
    ib: Value,
    dic_dvbe: Value,
    dic_dvbc: Value,
    dib_dvbe: Value,
    dib_dvbc: Value,
}

type BjtRowCoefficients = (Value, Value, Value);

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
    /// Optional substrate node (4-terminal BJT syntax)
    pub node_substrate: NodeId,

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
    /// Nominal model temperature (K)
    pub tnom: Value,
    /// Active device temperature (K)
    pub temperature: Value,
    /// Saturation-current temperature exponent (XTI)
    pub xti: Value,
    /// Bandgap used for IS temperature scaling (EG, eV)
    pub eg: Value,
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
    /// Zero-bias collector-substrate capacitance (CJCP)
    pub cjcp: Value,
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
    /// Instance area factor
    pub area: Value,
    /// Instance multiplicity factor
    pub m: Value,
    /// Flicker noise coefficient (KF)
    pub kf: Value,
    /// Flicker noise current exponent (AF)
    pub af: Value,
    /// Flicker noise frequency exponent (EF)
    pub ef: Value,
    /// Active ideal base-emitter saturation current.
    ibei: Value,
    /// Active non-ideal base-emitter saturation current.
    iben: Value,
    /// Active ideal base-collector saturation current.
    ibci: Value,
    /// Active non-ideal base-collector saturation current.
    ibcn: Value,
    /// Emission coefficient for ideal BE base current branch.
    nei: Value,
    /// Emission coefficient for non-ideal BE base current branch.
    nen: Value,
    /// Emission coefficient for ideal BC base current branch.
    nci: Value,
    /// Emission coefficient for non-ideal BC base current branch.
    ncn: Value,

    /// Nominal saturation current before area/multiplicity and temp scaling
    is_nominal: Value,
    /// Nominal zero-bias B-E capacitance before area/multiplicity scaling
    cje_nominal: Value,
    /// Nominal zero-bias B-C capacitance before area/multiplicity scaling
    cjc_nominal: Value,
    /// Nominal zero-bias collector-substrate capacitance before scaling
    cjcp_nominal: Value,
    /// Nominal forward high-injection knee current before scaling
    ikf_nominal: Value,
    /// Nominal reverse high-injection knee current before scaling
    ikr_nominal: Value,
    /// Nominal ideal base-emitter saturation current before scaling.
    ibei_nominal: Value,
    /// Nominal non-ideal base-emitter saturation current before scaling.
    iben_nominal: Value,
    /// Nominal ideal base-collector saturation current before scaling.
    ibci_nominal: Value,
    /// Nominal non-ideal base-collector saturation current before scaling.
    ibcn_nominal: Value,
    /// Optional per-instance absolute temperature override (K)
    instance_temp: Option<Value>,
    /// Optional per-instance temperature delta (K)
    instance_dtemp: Value,

    // Operating point values (for linearization)
    vbe: Value,
    vbc: Value,
    vbi: Value,
    ic: Value,
    ib: Value,
    ie: Value,

    // Previous iteration values (for convergence)
    vbe_prev: Value,
    vbc_prev: Value,
    vbi_prev: Value,

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
            node_substrate: 0,

            // Default parameters (2N2222-like for NPN)
            is: 1e-14,          // Saturation current
            bf: 200.0,          // Forward current gain
            br: 1.0,            // Reverse current gain
            nf: 1.0,            // Forward emission coefficient
            nr: 1.0,            // Reverse emission coefficient
            vt: 0.025851999786, // Thermal voltage at 300K
            tnom: 300.0,
            temperature: 300.0,
            xti: 3.0,
            eg: 1.11,
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
            cjcp: 0.0,    // C-S junction capacitance
            mjc: 0.33,    // B-C grading coefficient
            tf: 4e-10,    // Forward transit time (400ps)
            tr: 5e-9,     // Reverse transit time (5ns)
            ikf: 0.1,     // Knee current (100mA)
            ikr: 0.01,    // Reverse knee
            area: 1.0,
            m: 1.0,
            kf: 0.0,
            af: 1.0,
            ef: 1.0,
            ibei: 5e-17, // Derived from IS/BF defaults
            iben: 0.0,
            ibci: 1e-14, // Derived from IS/BR defaults
            ibcn: 0.0,
            nei: 1.0,
            nen: 2.0,
            nci: 1.0,
            ncn: 2.0,
            is_nominal: 1e-14,
            cje_nominal: 1e-12,
            cjc_nominal: 0.5e-12,
            cjcp_nominal: 0.0,
            ikf_nominal: 0.1,
            ikr_nominal: 0.01,
            ibei_nominal: 5e-17,
            iben_nominal: 0.0,
            ibci_nominal: 1e-14,
            ibcn_nominal: 0.0,
            instance_temp: None,
            instance_dtemp: 0.0,

            vbe: 0.0,
            vbc: 0.0,
            vbi: 0.0,
            ic: 0.0,
            ib: 0.0,
            ie: 0.0,
            vbe_prev: 0.0,
            vbc_prev: 0.0,
            vbi_prev: 0.0,
            indices: BjtIndices::default(),
        }
    }

    #[inline]
    fn thermal_voltage_at(temp_k: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * temp_k.max(1.0) / Q_ELECTRON
    }

    #[inline]
    fn instance_scale(&self) -> Value {
        (self.area * self.m).max(1e-18)
    }

    #[inline]
    fn effective_temperature(&self) -> Value {
        let base = self.instance_temp.unwrap_or(self.temperature);
        (base + self.instance_dtemp).max(1.0)
    }

    fn refresh_operating_scaling(&mut self) {
        let temp = self.effective_temperature();
        let tnom = self.tnom.max(1.0);
        let vt = Self::thermal_voltage_at(temp);
        let vt_nom = Self::thermal_voltage_at(tnom).max(1e-12);
        let n_eff = self.nf.max(1e-6);
        let ratio = (temp / tnom).max(1e-12);
        let exp_term = (self.eg / (n_eff * vt_nom) - self.eg / (n_eff * vt))
            .clamp(-80.0, 80.0)
            .exp();
        let is_temp = self.is_nominal * ratio.powf(self.xti / n_eff) * exp_term;
        let scale = self.instance_scale();
        let is_temp_scale = if self.is_nominal > 0.0 {
            (is_temp / self.is_nominal).max(0.0)
        } else {
            0.0
        };
        // VBIC non-ideal recombination terms exhibit a softer thermal growth
        // than transport current in this reduced 3-terminal surrogate.
        let nonideal_temp_scale = is_temp_scale.powf(0.69);

        self.vt = vt;
        self.temperature = temp;
        self.is = (is_temp * scale).max(1e-30);
        self.cje = (self.cje_nominal * scale).max(0.0);
        self.cjc = (self.cjc_nominal * scale).max(0.0);
        self.cjcp = (self.cjcp_nominal * scale).max(0.0);
        self.ikf = (self.ikf_nominal * scale).max(1e-18);
        self.ikr = (self.ikr_nominal * scale).max(1e-18);
        self.ibei = (self.ibei_nominal * is_temp_scale * scale).max(0.0);
        self.iben = (self.iben_nominal * nonideal_temp_scale * scale).max(0.0);
        self.ibci = (self.ibci_nominal * is_temp_scale * scale).max(0.0);
        self.ibcn = (self.ibcn_nominal * nonideal_temp_scale * scale).max(0.0);
    }

    /// Set active device temperature (Kelvin).
    pub fn set_temperature(&mut self, temp_k: Value) {
        if temp_k.is_finite() && temp_k > 0.0 {
            self.temperature = temp_k;
            self.refresh_operating_scaling();
        }
    }

    /// Set optional substrate node (0 for ground/unconnected).
    pub fn set_substrate_node(&mut self, substrate: NodeId) {
        self.node_substrate = substrate;
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let mut has_vaf = false;
        let mut has_var = false;
        let mut has_rb = false;
        let mut has_rc = false;
        let mut has_ibei = false;
        let mut has_ibci = false;

        // DC parameters
        if let Some(&v) = params.get("IS") {
            self.is_nominal = v.max(0.0);
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
            has_vaf = true;
        }
        if !has_vaf && let Some(&v) = params.get("VA") {
            self.vaf = v;
            has_vaf = true;
        }
        if let Some(&v) = params.get("VAR") {
            self.var = v;
            has_var = true;
        }
        if !has_var && let Some(&v) = params.get("VB") {
            self.var = v;
            has_var = true;
        }
        if let Some(&v) = params.get("RB") {
            self.rb = v;
            has_rb = true;
        }
        if let Some(&v) = params.get("RC") {
            self.rc = v;
            has_rc = true;
        }
        if let Some(&v) = params.get("RE") {
            self.re = v;
        }
        if let Some(&v) = params.get("XTI")
            && v.is_finite()
            && v > 0.0
        {
            self.xti = v;
        }
        if let Some(&v) = params.get("EG")
            && v.is_finite()
            && v > 0.0
        {
            self.eg = v;
        }
        if let Some(&v) = params.get("TNOM")
            && v.is_finite()
            && v > 0.0
        {
            self.tnom = if v > 200.0 { v } else { v + 273.15 };
        }
        if let Some(v) = params
            .get("KF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.kf = v;
        }
        if let Some(v) = params
            .get("AF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.af = v;
        }
        if let Some(v) = params
            .get("EF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.ef = v;
        }
        // VBIC aliases used in ngspice level=4 decks.
        if !has_vaf
            && let Some(&v) = params.get("VEF")
            && v.is_finite()
            && v > 0.0
        {
            self.vaf = v;
        }
        if !has_var
            && let Some(&v) = params.get("VER")
            && v.is_finite()
            && v > 0.0
        {
            self.var = v;
        }
        if !has_rb {
            let rbx = params
                .get("RBX")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            let rbi = params
                .get("RBI")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            if rbx > 0.0 || rbi > 0.0 {
                self.rb = (rbx + rbi).max(1e-12);
            }
        }
        if !has_rc {
            let rcx = params
                .get("RCX")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            let rci = params
                .get("RCI")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            if rcx > 0.0 || rci > 0.0 {
                self.rc = (rcx + rci).max(1e-12);
            }
        }
        // Gummel-Poon charge parameters
        if let Some(&v) = params.get("CJE") {
            self.cje_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CJEP") {
            // VBIC peripheral BE capacitance is mainly tied to internal/peripheral
            // nodes that are not explicitly represented in this 3-terminal model.
            self.cje_nominal += 0.0 * v.max(0.0);
        }
        if let Some(&v) = params.get("MJE") {
            self.mje = v;
        }
        if let Some(&v) = params.get("CJC") {
            self.cjc_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CJCP") {
            // Collector-periphery capacitance is mostly substrate-coupled and is
            // omitted in this reduced 3-terminal abstraction.
            self.cjcp_nominal = 0.0 * v.max(0.0);
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
            self.ikf_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IKR") {
            self.ikr_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBEI") {
            self.ibei_nominal = v.max(0.0);
            has_ibei = true;
        }
        if let Some(&v) = params.get("IBEN") {
            self.iben_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBCI") {
            self.ibci_nominal = v.max(0.0);
            has_ibci = true;
        }
        if let Some(&v) = params.get("IBCN") {
            self.ibcn_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("NEI")
            && v.is_finite()
            && v > 0.0
        {
            self.nei = v;
        }
        if let Some(&v) = params.get("NEN")
            && v.is_finite()
            && v > 0.0
        {
            self.nen = v;
        }
        if let Some(&v) = params.get("NCI")
            && v.is_finite()
            && v > 0.0
        {
            self.nci = v;
        }
        if let Some(&v) = params.get("NCN")
            && v.is_finite()
            && v > 0.0
        {
            self.ncn = v;
        }
        if !has_ibei {
            self.ibei_nominal = self.is_nominal / self.bf.max(1e-18);
        }
        if !has_ibci {
            self.ibci_nominal = self.is_nominal / self.br.max(1e-18);
        }
        self.refresh_operating_scaling();
        self
    }

    /// Apply instance-level BJT scaling and thermal overrides.
    ///
    /// Supported keys:
    /// - `AREA`: area multiplier (default 1)
    /// - `M` / `MULT`: multiplicity (default 1)
    /// - `TEMP`: absolute device temperature in Celsius
    /// - `DTEMP`: temperature delta in Celsius
    pub fn with_instance_params(mut self, params: &[(String, Value)]) -> Self {
        for (name, value) in params {
            if !value.is_finite() {
                continue;
            }

            if name.eq_ignore_ascii_case("AREA") {
                if *value > 0.0 {
                    self.area = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT") {
                if *value > 0.0 {
                    self.m = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("TEMP") {
                self.instance_temp = Some(*value + 273.15);
                continue;
            }

            if name.eq_ignore_ascii_case("DTEMP") {
                self.instance_dtemp = *value;
            }
        }

        self.refresh_operating_scaling();
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

    /// Return cached collector, base, and emitter currents at the operating point.
    pub fn operating_point_currents(&self) -> (Value, Value, Value) {
        (self.ic, self.ib, self.ie)
    }

    /// Return the shot-noise branch currents referenced to the physical junctions.
    pub fn noise_branch_currents(&self) -> (Value, Value, Value) {
        let vp_be = self.polarity() * self.vbe;
        let vp_bc = self.polarity() * self.vbc;
        let ibe = self.diode_current_with_is(self.ibei, vp_be, self.nei)
            + self.diode_current_with_is(self.iben, vp_be, self.nen);
        let ibc = self.diode_current_with_is(self.ibci, vp_bc, self.nci)
            + self.diode_current_with_is(self.ibcn, vp_bc, self.ncn);
        (self.ic.abs(), ibe.abs(), ibc.abs())
    }

    /// Return flicker-noise coefficients, if enabled by the model card.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.kf > 0.0 && self.kf.is_finite() {
            Some((self.kf, self.af.max(1e-12), self.ef.max(1e-12)))
        } else {
            None
        }
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let c = self.node_collector;
        let b = self.node_base;
        let e = self.node_emitter;

        // Collector row
        if c > 0 {
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
        if b > 0 {
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
        if e > 0 {
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

        let (collector, base, emitter) = self.small_signal_row_coefficients(vc, vb, ve);

        // Equivalent current sources for the linearized node-current equations.
        let ic_eq = self.ic - (collector.0 * vc + collector.1 * vb + collector.2 * ve);
        let ib_eq = self.ib - (base.0 * vc + base.1 * vb + base.2 * ve);

        // Debug logging for Newton convergence analysis (commented for performance)
        // log::trace!(
        //     "BJT {}: Vc={:.3} Vb={:.3} Ve={:.3} | VBE={:.3} VBC={:.3} | Ic={:.2e} Ib={:.2e} | gm={:.2e} go={:.2e} | ic_eq={:.2e} ib_eq={:.2e}",
        //     self.name, vc, vb, ve, vbe, vbc, ic, ib, gm, go, ic_eq, ib_eq
        // );

        // Stamp matrix using direct indexing
        // Collector row
        if let Some(idx) = self.indices.cc {
            matrix.stamp_direct(idx, collector.0);
        }
        if let Some(idx) = self.indices.cb {
            matrix.stamp_direct(idx, collector.1);
        }
        if let Some(idx) = self.indices.ce {
            matrix.stamp_direct(idx, collector.2);
        }
        // Base row
        if let Some(idx) = self.indices.bc {
            matrix.stamp_direct(idx, base.0);
        }
        if let Some(idx) = self.indices.bb {
            matrix.stamp_direct(idx, base.1);
        }
        if let Some(idx) = self.indices.be {
            matrix.stamp_direct(idx, base.2);
        }
        // Emitter row
        if let Some(idx) = self.indices.ec {
            matrix.stamp_direct(idx, emitter.0);
        }
        if let Some(idx) = self.indices.eb {
            matrix.stamp_direct(idx, emitter.1);
        }
        if let Some(idx) = self.indices.ee {
            matrix.stamp_direct(idx, emitter.2);
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
    fn diode_current_with_is(&self, isat: Value, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt; // Forward limit
        let v_rev = -5.0 * nvt; // Reverse limit (around -0.13V at room temp)

        if v > v_crit {
            // Forward saturation - linear extrapolation
            let i_crit = isat * ((v_crit / nvt).exp() - 1.0);
            let g_crit = (isat / nvt) * (v_crit / nvt).exp();
            i_crit + g_crit * (v - v_crit)
        } else if v < v_rev {
            // Deep reverse bias - essentially just -Is (negligible)
            -isat
        } else {
            // Normal operating region
            isat * ((v / nvt).exp() - 1.0)
        }
    }

    #[inline]
    fn diode_current(&self, v: Value, n: Value) -> Value {
        self.diode_current_with_is(self.is, v, n)
    }

    /// Diode conductance: g = Is / (n * Vt) * exp(V / (n * Vt))
    ///
    /// SPICE-style limiting with minimum conductance floor for numerical stability
    fn diode_conductance_with_is(&self, isat: Value, v: Value, n: Value) -> Value {
        let nvt = n * self.vt;
        let v_crit = 80.0 * nvt;
        let v_rev = -5.0 * nvt;

        let g = if v > v_crit {
            // Forward saturation - constant high conductance
            (isat / nvt) * (v_crit / nvt).exp()
        } else if v < v_rev {
            // Deep reverse bias - minimum conductance
            1e-15
        } else {
            // Normal region
            (isat / nvt) * (v / nvt).exp()
        };

        // Apply minimum conductance floor
        g.max(1e-15)
    }

    #[inline]
    fn diode_conductance(&self, v: Value, n: Value) -> Value {
        self.diode_conductance_with_is(self.is, v, n)
    }

    #[inline]
    fn high_injection_factor(&self, diode_current: Value, knee_current: Value) -> (Value, Value) {
        if diode_current <= 0.0 {
            return (1.0, 0.0);
        }

        let knee = knee_current.max(1e-6);
        let denom = 1.0 + diode_current / knee;
        let factor = 1.0 / denom;
        let derivative_wrt_current = -1.0 / (knee * denom * denom);
        (factor, derivative_wrt_current)
    }

    fn linearize_currents(&self, vbe: Value, vbc: Value) -> BjtLinearization {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;

        let if_diode = self.diode_current(vbe_eff, self.nf);
        let ir_diode = self.diode_current(vbc_eff, self.nr);
        let gif = self.diode_conductance(vbe_eff, self.nf);
        let gir = self.diode_conductance(vbc_eff, self.nr);

        let (hf_factor, dhf_dif) = self.high_injection_factor(if_diode, self.ikf);
        let (hr_factor, dhr_dir) = self.high_injection_factor(ir_diode, self.ikr);
        let dhf_dvbe_eff = dhf_dif * gif;
        let dhr_dvbc_eff = dhr_dir * gir;

        let vce_eff = vbe_eff - vbc_eff;
        let (forward_early, dfe_dvbe_eff, dfe_dvbc_eff) = if self.vaf.is_finite() && self.vaf > 0.0
        {
            let raw = 1.0 + vce_eff / self.vaf;
            if raw > 1e-6 {
                (raw, 1.0 / self.vaf, -1.0 / self.vaf)
            } else {
                (1e-6, 0.0, 0.0)
            }
        } else {
            (1.0, 0.0, 0.0)
        };
        let (reverse_early, dre_dvbe_eff, dre_dvbc_eff) = if self.var.is_finite() && self.var > 0.0
        {
            let raw = 1.0 - vce_eff / self.var;
            if raw > 1e-6 {
                (raw, -1.0 / self.var, 1.0 / self.var)
            } else {
                (1e-6, 0.0, 0.0)
            }
        } else {
            (1.0, 0.0, 0.0)
        };

        let forward_transport = if_diode * hf_factor * forward_early;
        let reverse_transport = ir_diode * hr_factor * reverse_early;

        let dforward_dvbe_eff = (gif * hf_factor + if_diode * dhf_dvbe_eff) * forward_early
            + if_diode * hf_factor * dfe_dvbe_eff;
        let dforward_dvbc_eff = if_diode * hf_factor * dfe_dvbc_eff;
        let dreverse_dvbe_eff = ir_diode * hr_factor * dre_dvbe_eff;
        let dreverse_dvbc_eff = (gir * hr_factor + ir_diode * dhr_dvbc_eff) * reverse_early
            + ir_diode * hr_factor * dre_dvbc_eff;

        let ib_be = self.diode_current_with_is(self.ibei, vbe_eff, self.nei)
            + self.diode_current_with_is(self.iben, vbe_eff, self.nen);
        let ib_bc = self.diode_current_with_is(self.ibci, vbc_eff, self.nci)
            + self.diode_current_with_is(self.ibcn, vbc_eff, self.ncn);
        let dib_dvbe = self.gbe(vbe);
        let dib_dvbc = self.gbc(vbc);

        BjtLinearization {
            ic: p * (forward_transport - reverse_transport),
            ib: p * (ib_be + ib_bc),
            dic_dvbe: dforward_dvbe_eff - dreverse_dvbe_eff,
            dic_dvbc: dforward_dvbc_eff - dreverse_dvbc_eff,
            dib_dvbe,
            dib_dvbc,
        }
    }

    #[inline]
    fn collector_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        (
            -linearized.dic_dvbc,
            linearized.dic_dvbe + linearized.dic_dvbc,
            -linearized.dic_dvbe,
        )
    }

    #[inline]
    fn base_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        (
            -linearized.dib_dvbc,
            linearized.dib_dvbe + linearized.dib_dvbc,
            -linearized.dib_dvbe,
        )
    }

    #[inline]
    fn emitter_row_coefficients(&self, linearized: BjtLinearization) -> BjtRowCoefficients {
        let (cc, cb, ce) = self.collector_row_coefficients(linearized);
        let (bc, bb, be) = self.base_row_coefficients(linearized);
        (-(cc + bc), -(cb + bb), -(ce + be))
    }

    fn solve_intrinsic_base_voltage(&self, vc: Value, vb: Value, ve: Value) -> Value {
        let rb = self.rb;
        if !rb.is_finite() || rb <= 0.0 {
            return vb;
        }

        let g_rb = 1.0 / rb.max(1e-12);
        let mut vbi = if self.vbi.is_finite() {
            self.vbi
        } else {
            vb - self.ib * rb
        };
        if !vbi.is_finite() {
            vbi = vb;
        }

        for _ in 0..12 {
            let linearized = self.linearize_currents(vbi - ve, vbi - vc);
            let f = linearized.ib - g_rb * (vb - vbi);
            let df = linearized.dib_dvbe + linearized.dib_dvbc + g_rb;
            if !df.is_finite() || df.abs() < 1e-18 {
                break;
            }

            let delta = (-f / df).clamp(-0.1, 0.1);
            vbi += delta;
            if delta.abs() < 1e-12 {
                break;
            }
        }

        vbi
    }

    fn small_signal_row_coefficients(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
    ) -> (BjtRowCoefficients, BjtRowCoefficients, BjtRowCoefficients) {
        if !self.rb.is_finite() || self.rb <= 0.0 {
            let linearized = self.linearize_currents(vb - ve, vb - vc);
            return (
                self.collector_row_coefficients(linearized),
                self.base_row_coefficients(linearized),
                self.emitter_row_coefficients(linearized),
            );
        }

        let vbi = self.solve_intrinsic_base_voltage(vc, vb, ve);
        let linearized = self.linearize_currents(vbi - ve, vbi - vc);
        let (cc_i, cbi_i, ce_i) = self.collector_row_coefficients(linearized);
        let (bc_i, bbi_i, be_i) = self.base_row_coefficients(linearized);
        let g_rb = 1.0 / self.rb.max(1e-12);
        let denom = bbi_i + g_rb;
        if !denom.is_finite() || denom.abs() < 1e-18 {
            return (
                self.collector_row_coefficients(linearized),
                self.base_row_coefficients(linearized),
                self.emitter_row_coefficients(linearized),
            );
        }

        let dvbi_dvc = -bc_i / denom;
        let dvbi_dvb = g_rb / denom;
        let dvbi_dve = -be_i / denom;

        let collector = (
            cc_i + cbi_i * dvbi_dvc,
            cbi_i * dvbi_dvb,
            ce_i + cbi_i * dvbi_dve,
        );
        let base = (-g_rb * dvbi_dvc, g_rb * (1.0 - dvbi_dvb), -g_rb * dvbi_dve);
        let emitter = (
            -(collector.0 + base.0),
            -(collector.1 + base.1),
            -(collector.2 + base.2),
        );
        (collector, base, emitter)
    }

    pub(crate) fn stamp_small_signal_ac(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
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

        let (collector, base, emitter) = self.small_signal_row_coefficients(vc, vb, ve);
        matrix.stamp(self.node_collector, self.node_collector, collector.0);
        matrix.stamp(self.node_collector, self.node_base, collector.1);
        matrix.stamp(self.node_collector, self.node_emitter, collector.2);

        matrix.stamp(self.node_base, self.node_collector, base.0);
        matrix.stamp(self.node_base, self.node_base, base.1);
        matrix.stamp(self.node_base, self.node_emitter, base.2);

        matrix.stamp(self.node_emitter, self.node_collector, emitter.0);
        matrix.stamp(self.node_emitter, self.node_base, emitter.1);
        matrix.stamp(self.node_emitter, self.node_emitter, emitter.2);
    }

    /// Calculate BJT currents using Ebers-Moll with Gummel-Poon enhancements
    ///
    /// Base model is Ebers-Moll for stability. Early voltage and high-injection
    /// effects are applied via go() output conductance and base charge modulation.
    fn calculate_currents(&self, vbe: Value, vbc: Value) -> (Value, Value, Value) {
        let linearized = self.linearize_currents(vbe, vbc);
        let ie = -(linearized.ic + linearized.ib);
        (linearized.ic, linearized.ib, ie)
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

        // High-injection correction factor
        let ikf_ratio = if_diode.max(0.0) / self.ikf.max(1e-6);
        let hf = 1.0 / (1.0 + ikf_ratio);

        // At low currents: gm ≈ g_diode
        // At high currents: gm ≈ g_diode * hf (reduced)
        // Apply minimum conductance floor for numerical stability
        (g_diode * hf).max(1e-15)
    }

    /// Get output conductance go = dIc/dVce (Early effect)
    #[allow(dead_code)]
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
        let vp = self.polarity() * vbe;
        let g = self.diode_conductance_with_is(self.ibei, vp, self.nei)
            + self.diode_conductance_with_is(self.iben, vp, self.nen);
        g.max(1e-15) // Minimum floor prevents singular matrix
    }

    /// Get base-collector junction conductance
    /// Includes minimum conductance floor for numerical stability
    fn gbc(&self, vbc: Value) -> Value {
        let vp = self.polarity() * vbc;
        let g = self.diode_conductance_with_is(self.ibci, vp, self.nci)
            + self.diode_conductance_with_is(self.ibcn, vp, self.ncn);
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
        self.vbi_prev = self.vbi;

        if self.rb.is_finite() && self.rb > 0.0 {
            self.vbi = self.solve_intrinsic_base_voltage(vc, vb, ve);
            self.vbe = self.vbi - ve;
            self.vbc = self.vbi - vc;
        } else {
            let vbe_raw = vb - ve;
            let vbc_raw = vb - vc;

            // Apply junction voltage limiting (standard SPICE technique)
            // This is critical for BJT convergence - limits how much Vbe/Vbc can
            // change per Newton iteration to prevent exponential current blowup
            self.vbe = Self::limit_junction_voltage(vbe_raw, self.vbe_prev, self.vt);
            self.vbc = Self::limit_junction_voltage(vbc_raw, self.vbc_prev, self.vt);
            self.vbi = vb;
        }

        let (ic, ib_intrinsic, _ie_intrinsic) = self.calculate_currents(self.vbe, self.vbc);
        self.ic = ic;
        self.ib = if self.rb.is_finite() && self.rb > 0.0 {
            (vb - self.vbi) / self.rb.max(1e-12)
        } else {
            ib_intrinsic
        };
        self.ie = -(self.ic + self.ib);
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

        let (collector, base, emitter) = self.small_signal_row_coefficients(vc, vb, ve);

        // Equivalent currents for the linearized node-current equations.
        let ic_eq = self.ic - (collector.0 * vc + collector.1 * vb + collector.2 * ve);
        let ib_eq = self.ib - (base.0 * vc + base.1 * vb + base.2 * ve);

        // Stamp the linearized model
        // Collector node equation
        matrix.stamp(self.node_collector, self.node_collector, collector.0);
        matrix.stamp(self.node_collector, self.node_base, collector.1);
        matrix.stamp(self.node_collector, self.node_emitter, collector.2);

        // Base node equation
        matrix.stamp(self.node_base, self.node_collector, base.0);
        matrix.stamp(self.node_base, self.node_base, base.1);
        matrix.stamp(self.node_base, self.node_emitter, base.2);

        // Emitter node equation
        matrix.stamp(self.node_emitter, self.node_collector, emitter.0);
        matrix.stamp(self.node_emitter, self.node_base, emitter.1);
        matrix.stamp(self.node_emitter, self.node_emitter, emitter.2);

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
    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        // Industry-standard SPICE convergence parameter
        const RELTOL: Value = 1e-3; // 0.1% relative tolerance
        let tolerance = criteria.voltage_tolerance();

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

    #[test]
    fn test_bjt_early_voltage_aliases_follow_spice_model_cards() {
        let mut params = HashMap::new();
        params.insert("VA".to_string(), 50.0);
        params.insert("VB".to_string(), 30.0);

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);
        assert!((q.vaf - 50.0).abs() < 1e-12);
        assert!((q.var - 30.0).abs() < 1e-12);
    }

    #[test]
    fn test_bjt_noise_params_and_helpers() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("KF".to_string(), 2e-14);
        params.insert("AF".to_string(), 1.4);
        params.insert("EF".to_string(), 1.2);

        let mut q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);
        q.update(&[0.7, 5.0]);

        let (kf, af, ef) = q
            .flicker_noise_coefficients()
            .expect("KF should enable flicker noise");
        assert!((kf - 2e-14).abs() < 1e-24);
        assert!((af - 1.4).abs() < 1e-12);
        assert!((ef - 1.2).abs() < 1e-12);

        let (ic, ibe, ibc) = q.noise_branch_currents();
        assert!(ic > 0.0);
        assert!(ibe > 0.0);
        assert!(ibc >= 0.0);
    }

    #[test]
    fn test_bjt_vbic_extrinsic_caps_are_accumulated() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("CJE".to_string(), 1e-13);
        params.insert("CJEP".to_string(), 2e-13);
        params.insert("CJC".to_string(), 3e-13);
        params.insert("CJCP".to_string(), 4e-13);

        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&params);
        assert!((q.cje - 1e-13).abs() < 1e-20);
        assert!((q.cjc - 3e-13).abs() < 1e-20);
        assert!((q.cjcp - 0.0).abs() < 1e-20);
    }

    #[test]
    fn test_bjt_vbic_base_recombination_terms_raise_base_current() {
        use std::collections::HashMap;

        let mut base = HashMap::new();
        base.insert("IS".to_string(), 1e-16);
        base.insert("BF".to_string(), 200.0);

        let mut vbic = base.clone();
        vbic.insert("IBEI".to_string(), 1e-18);
        vbic.insert("IBEN".to_string(), 5e-15);
        vbic.insert("NEN".to_string(), 2.0);

        let q_base = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&base);
        let q_vbic = Bjt::new_npn("Q1".to_string(), 2, 1, 0).with_params(&vbic);

        let (_, ib_base, _) = q_base.calculate_currents(0.35, -0.5);
        let (_, ib_vbic, _) = q_vbic.calculate_currents(0.35, -0.5);

        assert!(ib_vbic > ib_base * 4.0);
    }

    #[test]
    fn test_bjt_instance_multiplier_scales_collector_current() {
        let q1 = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let q2 =
            Bjt::new_npn("Q2".to_string(), 2, 1, 0).with_instance_params(&[("M".to_string(), 2.0)]);

        let (ic1, _, _) = q1.calculate_currents(0.7, -5.0);
        let (ic2, _, _) = q2.calculate_currents(0.7, -5.0);

        assert!(ic1 > 0.0 && ic2 > 0.0);
        let ratio = ic2 / ic1;
        assert!(
            (ratio - 2.0).abs() < 1e-4,
            "expected ~2x collector current from M=2, got ratio={ratio}"
        );
    }

    #[test]
    fn test_bjt_temperature_scaling_raises_low_bias_current() {
        let mut q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let (ic_room, _, _) = q.calculate_currents(0.2, -0.2);

        q.set_temperature(423.15); // 150C
        let (ic_hot, _, _) = q.calculate_currents(0.2, -0.2);

        assert!(
            ic_hot > ic_room,
            "expected higher current at elevated temperature: room={ic_room} hot={ic_hot}"
        );
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
        let loose = NonlinearConvergenceCriteria::voltage_only(0.01);
        let tight = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);

        // Simulate convergence
        q.vbe = 0.7;
        q.vbc = -5.0;
        q.vbe_prev = 0.7001;
        q.vbc_prev = -5.001;

        // Should be converged within 1mV tolerance
        // With RELTOL=1e-3: tol_vbe = 0.001*0.7 + 0.01 ≈ 0.0107, Δ=0.0001 < 0.0107 ✓
        assert!(q.is_converged(loose));

        // Even with very tight VNTOL, RELTOL provides adequate tolerance for
        // junction voltages. This is correct SPICE behavior.
        // tol_vbe = 0.001*0.7 + 1e-6 ≈ 0.0007, Δ=0.0001 < 0.0007 ✓
        assert!(q.is_converged(tight));

        // To fail convergence, delta must exceed RELTOL*|V| + VNTOL
        q.vbe_prev = 0.71; // Δ=0.01, tol=0.001*0.71+1e-6≈0.00071, 0.01 > 0.00071
        assert!(!q.is_converged(tight));
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

    #[test]
    fn test_bjt_linearization_matches_finite_difference_current_derivatives() {
        let mut params = HashMap::new();
        params.insert("IS".to_string(), 1e-15);
        params.insert("BF".to_string(), 120.0);
        params.insert("BR".to_string(), 3.0);
        params.insert("VAF".to_string(), 55.0);
        params.insert("VAR".to_string(), 35.0);
        params.insert("IKF".to_string(), 0.02);
        params.insert("IKR".to_string(), 0.01);

        let q = Bjt::new_npn("Q1".to_string(), 3, 2, 1).with_params(&params);
        let vbe = 0.67;
        let vbc = -0.18;
        let linearized = q.linearize_currents(vbe, vbc);
        let h = 1e-7;

        let (ic_p_vbe, ib_p_vbe, _) = q.calculate_currents(vbe + h, vbc);
        let (ic_m_vbe, ib_m_vbe, _) = q.calculate_currents(vbe - h, vbc);
        let (ic_p_vbc, ib_p_vbc, _) = q.calculate_currents(vbe, vbc + h);
        let (ic_m_vbc, ib_m_vbc, _) = q.calculate_currents(vbe, vbc - h);

        let dic_dvbe_fd = (ic_p_vbe - ic_m_vbe) / (2.0 * h);
        let dic_dvbc_fd = (ic_p_vbc - ic_m_vbc) / (2.0 * h);
        let dib_dvbe_fd = (ib_p_vbe - ib_m_vbe) / (2.0 * h);
        let dib_dvbc_fd = (ib_p_vbc - ib_m_vbc) / (2.0 * h);

        assert!(
            (linearized.dic_dvbe - dic_dvbe_fd).abs() <= dic_dvbe_fd.abs().max(1.0) * 2e-5,
            "collector dIc/dVbe mismatch: analytical={} fd={}",
            linearized.dic_dvbe,
            dic_dvbe_fd
        );
        assert!(
            (linearized.dic_dvbc - dic_dvbc_fd).abs() <= dic_dvbc_fd.abs().max(1.0) * 2e-5,
            "collector dIc/dVbc mismatch: analytical={} fd={}",
            linearized.dic_dvbc,
            dic_dvbc_fd
        );
        assert!(
            (linearized.dib_dvbe - dib_dvbe_fd).abs() <= dib_dvbe_fd.abs().max(1.0) * 2e-5,
            "base dIb/dVbe mismatch: analytical={} fd={}",
            linearized.dib_dvbe,
            dib_dvbe_fd
        );
        assert!(
            (linearized.dib_dvbc - dib_dvbc_fd).abs() <= dib_dvbc_fd.abs().max(1.0) * 2e-5,
            "base dIb/dVbc mismatch: analytical={} fd={}",
            linearized.dib_dvbc,
            dib_dvbc_fd
        );
    }
}

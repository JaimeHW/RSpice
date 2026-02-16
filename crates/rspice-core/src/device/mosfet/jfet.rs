//! JFET (Junction Field-Effect Transistor) Device Model
//!
//! Implements the Shichman-Hodges model for N-channel and P-channel JFETs.
//!
//! # Model Equations
//!
//! For N-JFET (P-JFET uses opposite polarities):
//!
//! **Cutoff** (Vgs - Vto ≤ 0):
//! ```text
//! Ids = 0
//! ```
//!
//! **Linear** (Vds < Vgs - Vto):
//! ```text
//! Ids = Beta * (2*(Vgs-Vto)*Vds - Vds²) * (1 + Lambda*Vds)
//! ```
//!
//! **Saturation** (Vds ≥ Vgs - Vto):
//! ```text
//! Ids = Beta * (Vgs - Vto)² * (1 + Lambda*Vds)
//! ```
//!
//! where Beta is typically derived from IDSS: `Beta = IDSS / Vto²`
//!
//! # Example
//!
//! ```ignore
//! J1 drain gate source JMOD
//! .MODEL JMOD NJF(VTO=-2 BETA=1E-3 LAMBDA=0.01)
//! ```

use crate::Value;
use crate::circuit::NodeId;
use crate::device::traits::{MatrixStamper, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};

//=============================================================================
// JFET Type
//=============================================================================

/// JFET channel type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JfetType {
    /// N-channel JFET (current flows drain to source)
    NJF,
    /// P-channel JFET (current flows source to drain)
    PJF,
}

impl JfetType {
    /// Get polarity multiplier (+1 for NJF, -1 for PJF)
    pub fn polarity(&self) -> Value {
        match self {
            JfetType::NJF => 1.0,
            JfetType::PJF => -1.0,
        }
    }
}

//=============================================================================
// JFET Parameters
//=============================================================================

/// JFET model parameters (Shichman-Hodges level 1)
#[derive(Debug, Clone)]
pub struct JfetParams {
    /// Threshold voltage (V) - negative for N-JFET depletion mode
    pub vto: Value,
    /// Transconductance coefficient (A/V²)
    pub beta: Value,
    /// Channel-length modulation (1/V)
    pub lambda: Value,
    /// Gate junction saturation current (A)
    pub is: Value,
    /// Gate-source zero-bias capacitance (F)
    pub cgs: Value,
    /// Gate-drain zero-bias capacitance (F)
    pub cgd: Value,
    /// Gate junction potential (V)
    pub pb: Value,
    /// Capacitance grading coefficient
    pub m: Value,
    /// Drain ohmic resistance (Ω)
    pub rd: Value,
    /// Source ohmic resistance (Ω)
    pub rs: Value,
    /// Forward bias junction coefficient
    pub fc: Value,
    /// Gate junction emission coefficient
    pub n: Value,
    /// Nominal temperature (K)
    pub tnom: Value,
}

impl Default for JfetParams {
    fn default() -> Self {
        Self {
            vto: -2.0,    // Threshold voltage (depletion mode)
            beta: 1e-4,   // Transconductance coefficient
            lambda: 0.0,  // Channel-length modulation
            is: 1e-14,    // Gate saturation current
            cgs: 0.0,     // Gate-source capacitance
            cgd: 0.0,     // Gate-drain capacitance
            pb: 1.0,      // Junction potential
            m: 0.5,       // Grading coefficient
            rd: 0.0,      // Drain resistance
            rs: 0.0,      // Source resistance
            fc: 0.5,      // Forward bias coefficient
            n: 1.0,       // Emission coefficient
            tnom: 300.15, // 27°C nominal
        }
    }
}

impl JfetParams {
    /// Create parameters from IDSS and VTO
    ///
    /// IDSS is the drain current at Vgs=0, Vds >> Vgs-Vto (saturation)
    /// Beta = IDSS / Vto²
    pub fn from_idss(idss: Value, vto: Value) -> Self {
        let beta = idss / (vto * vto);
        Self {
            vto,
            beta,
            ..Default::default()
        }
    }

    /// Create with specified parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Set VTO
    pub fn with_vto(mut self, vto: Value) -> Self {
        self.vto = vto;
        self
    }

    /// Set BETA
    pub fn with_beta(mut self, beta: Value) -> Self {
        self.beta = beta;
        self
    }

    /// Set LAMBDA
    pub fn with_lambda(mut self, lambda: Value) -> Self {
        self.lambda = lambda;
        self
    }

    /// Set capacitances
    pub fn with_capacitances(mut self, cgs: Value, cgd: Value) -> Self {
        self.cgs = cgs;
        self.cgd = cgd;
        self
    }

    /// Set junction parameters
    pub fn with_junction(mut self, is: Value, pb: Value) -> Self {
        self.is = is;
        self.pb = pb;
        self
    }
}

//=============================================================================
// JFET Device
//=============================================================================

/// Pre-computed stamp indices for O(1) matrix access.
#[derive(Debug, Clone, Default)]
pub struct JfetIndices {
    // Drain row
    pub dd: Option<CscIndex>,
    pub dg: Option<CscIndex>,
    pub ds: Option<CscIndex>,
    // Gate row
    pub gd: Option<CscIndex>,
    pub gg: Option<CscIndex>,
    pub gs: Option<CscIndex>,
    // Source row
    pub sd: Option<CscIndex>,
    pub sg: Option<CscIndex>,
    pub ss: Option<CscIndex>,
}

/// JFET device instance
#[derive(Debug, Clone)]
pub struct Jfet {
    /// Instance name
    pub name: String,
    /// JFET type (NJF or PJF)
    pub jfet_type: JfetType,
    /// Drain node index
    pub drain: NodeId,
    /// Gate node index
    pub gate: NodeId,
    /// Source node index
    pub source: NodeId,
    /// Model parameters
    pub params: JfetParams,
    /// Device multiplier
    pub m: Value,
    /// Area factor
    pub area: Value,
    /// Previous/current iteration state for convergence checks
    vgs: Value,
    vds: Value,
    vgs_prev: Value,
    vds_prev: Value,
    /// Pre-computed matrix indices for O(1) direct stamping
    pub indices: JfetIndices,
}

impl Jfet {
    /// Create a new N-JFET
    pub fn njf(name: &str, drain: NodeId, gate: NodeId, source: NodeId) -> Self {
        Self {
            name: name.to_string(),
            jfet_type: JfetType::NJF,
            drain,
            gate,
            source,
            params: JfetParams::default(),
            m: 1.0,
            area: 1.0,
            vgs: 0.0,
            vds: 0.0,
            vgs_prev: 0.0,
            vds_prev: 0.0,
            indices: JfetIndices::default(),
        }
    }

    /// Create a new P-JFET
    pub fn pjf(name: &str, drain: NodeId, gate: NodeId, source: NodeId) -> Self {
        Self {
            name: name.to_string(),
            jfet_type: JfetType::PJF,
            drain,
            gate,
            source,
            params: JfetParams::default(),
            m: 1.0,
            area: 1.0,
            vgs: 0.0,
            vds: 0.0,
            vgs_prev: 0.0,
            vds_prev: 0.0,
            indices: JfetIndices::default(),
        }
    }

    /// Set model parameters
    pub fn with_params(mut self, params: JfetParams) -> Self {
        self.params = params;
        self
    }

    /// Set device multiplier
    pub fn with_multiplier(mut self, m: Value) -> Self {
        self.m = m;
        self
    }

    /// Set area factor
    pub fn with_area(mut self, area: Value) -> Self {
        self.area = area;
        self
    }

    /// Set model parameters from a HashMap (for .MODEL statement parsing)
    pub fn with_model_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let mut p = self.params.clone();

        if let Some(v) = params
            .get("VTO")
            .or_else(|| params.get("VT0"))
            .copied()
            .filter(|v| v.is_finite())
        {
            p.vto = v;
        }

        let beta_from_card = params
            .get("BETA")
            .or_else(|| params.get("B"))
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0);
        let idss_from_card = params
            .get("IDSS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0);
        if let Some(beta) = beta_from_card {
            p.beta = beta;
        } else if let Some(idss) = idss_from_card {
            let vto2 = p.vto * p.vto;
            if vto2 > 1e-30 {
                p.beta = idss / vto2;
            }
        }

        if let Some(v) = params
            .get("LAMBDA")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.lambda = v;
        }
        if let Some(v) = params
            .get("IS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.is = v;
        }
        if let Some(v) = params
            .get("CGS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.cgs = v;
        }
        if let Some(v) = params
            .get("CGD")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.cgd = v;
        }
        if let Some(v) = params
            .get("PB")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.pb = v;
        }
        if let Some(v) = params
            .get("M")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            if v < 1.0 {
                p.m = v;
            } else {
                // HFET/MESFET cards often use M as multiplicity.
                self.m *= v;
            }
        }
        if let Some(v) = params
            .get("RD")
            .or_else(|| params.get("RDI"))
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.rd = v;
        }
        if let Some(v) = params
            .get("RS")
            .or_else(|| params.get("RSI"))
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.rs = v;
        }
        if let Some(v) = params
            .get("FC")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0 && *v < 1.0)
        {
            p.fc = v;
        }
        if let Some(v) = params
            .get("N")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.n = v;
        }
        if let Some(v) = params
            .get("TNOM")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.tnom = v;
        }
        self.params = p;
        self
    }

    /// Thermal voltage at given temperature
    fn thermal_voltage(&self, temp: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * temp / Q_ELECTRON
    }

    #[inline]
    fn junction_scale(&self) -> Value {
        self.area * self.m
    }

    /// Gate junction diode current for internal anode-cathode voltage.
    ///
    /// Uses SPICE-style limiting to keep Newton iterations finite under strong
    /// forward or reverse bias.
    fn junction_diode_current(&self, v_ak: Value, temp: Value) -> Value {
        let nvt = self.params.n * self.thermal_voltage(temp);
        let isat = self.params.is * self.junction_scale();
        let v_crit = 80.0 * nvt;
        let v_rev = -5.0 * nvt;

        if v_ak > v_crit {
            let exp_crit = (v_crit / nvt).exp();
            let i_crit = isat * (exp_crit - 1.0);
            let g_crit = (isat / nvt) * exp_crit;
            i_crit + g_crit * (v_ak - v_crit)
        } else if v_ak < v_rev {
            -isat
        } else {
            isat * ((v_ak / nvt).exp() - 1.0)
        }
    }

    /// Gate junction diode small-signal conductance for internal anode-cathode voltage.
    fn junction_diode_conductance(&self, v_ak: Value, temp: Value) -> Value {
        let nvt = self.params.n * self.thermal_voltage(temp);
        let isat = self.params.is * self.junction_scale();
        let v_crit = 80.0 * nvt;
        let v_rev = -5.0 * nvt;

        let g = if v_ak > v_crit {
            (isat / nvt) * (v_crit / nvt).exp()
        } else if v_ak < v_rev {
            1e-15
        } else {
            (isat / nvt) * (v_ak / nvt).exp()
        };
        g.max(1e-15)
    }

    #[inline]
    fn node_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    /// Calculate drain current and conductances
    ///
    /// Returns (Ids, gm, gds) where:
    /// - Ids: drain-source current
    /// - gm: transconductance ∂Ids/∂Vgs
    /// - gds: output conductance ∂Ids/∂Vds
    pub fn calculate(&self, vgs: Value, vds: Value, _temp: Value) -> (Value, Value, Value) {
        let pol = self.jfet_type.polarity();

        // Apply polarity for P-JFET
        let vgs_int = pol * vgs;
        let vds_int = pol * vds;

        let vto = self.params.vto;
        let beta = self.params.beta * self.area * self.m;
        let lambda = self.params.lambda;

        // Effective Vgs (gate-source overdrive)
        let vgst = vgs_int - vto;

        let (ids, gm, gds) = if vgst <= 0.0 {
            // Cutoff region
            (0.0, 0.0, 0.0)
        } else if vds_int < 0.0 {
            // Reverse operation - swap drain and source
            // This handles the symmetric JFET behavior
            let vds_rev = -vds_int;
            let vgs_rev = vgs_int - vds_int;
            let vgst_rev = vgs_rev - vto;

            if vgst_rev <= 0.0 {
                (0.0, 0.0, 0.0)
            } else if vds_rev <= vgst_rev {
                // Linear (reversed)
                // Evaluate forward current from swapped terminals, then map back
                // to the original drain-source orientation.
                let ids_fwd = beta
                    * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev)
                    * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vds_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * 2.0 * (vgst_rev - vds_rev) * (1.0 + lambda * vds_rev)
                    + beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            } else {
                // Saturation (reversed)
                let ids_fwd = beta * vgst_rev * vgst_rev * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vgst_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * vgst_rev * vgst_rev * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            }
        } else if vds_int <= vgst {
            // Linear (triode) region: Vds < Vgs - Vto
            let ids = beta * (2.0 * vgst * vds_int - vds_int * vds_int) * (1.0 + lambda * vds_int);

            // gm = ∂Ids/∂Vgs = 2 * beta * Vds * (1 + lambda * Vds)
            let gm = 2.0 * beta * vds_int * (1.0 + lambda * vds_int);

            // gds = ∂Ids/∂Vds = beta * 2 * (Vgst - Vds) * (1 + lambda*Vds)
            //                   + beta * (2*Vgst*Vds - Vds²) * lambda
            let gds = beta * 2.0 * (vgst - vds_int) * (1.0 + lambda * vds_int)
                + beta * (2.0 * vgst * vds_int - vds_int * vds_int) * lambda;

            (ids, gm, gds)
        } else {
            // Saturation region: Vds >= Vgs - Vto
            let ids = beta * vgst * vgst * (1.0 + lambda * vds_int);

            // gm = ∂Ids/∂Vgs = 2 * beta * Vgst * (1 + lambda * Vds)
            let gm = 2.0 * beta * vgst * (1.0 + lambda * vds_int);

            // gds = ∂Ids/∂Vds = beta * Vgst² * lambda
            let gds = beta * vgst * vgst * lambda;

            (ids, gm, gds)
        };

        // Apply polarity for output current
        (pol * ids, gm, gds)
    }

    /// Calculate gate junction current (reverse-biased diodes)
    ///
    /// Returns (Igs, Igd) - gate-source and gate-drain junction currents
    pub fn gate_current(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        let (igs, igd, _, _) = self.gate_junctions(vgs, vgd, temp);
        (igs, igd)
    }

    /// Calculate gate junction currents and conductances.
    ///
    /// Returned currents are defined in external terminal orientation:
    /// - `igs`: current from gate to source
    /// - `igd`: current from gate to drain
    fn gate_junctions(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value, Value, Value) {
        let pol = self.jfet_type.polarity();
        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;

        let igs = pol * self.junction_diode_current(vgs_int, temp);
        let igd = pol * self.junction_diode_current(vgd_int, temp);
        let ggs = self.junction_diode_conductance(vgs_int, temp);
        let ggd = self.junction_diode_conductance(vgd_int, temp);

        (igs, igd, ggs, ggd)
    }

    /// Calculate junction capacitances
    ///
    /// Returns (Cgs, Cgd) - gate-source and gate-drain capacitances
    pub fn capacitances(&self, vgs: Value, vgd: Value) -> (Value, Value) {
        let scale = self.junction_scale();
        let cgs0 = self.params.cgs * scale;
        let cgd0 = self.params.cgd * scale;
        let pb = self.params.pb;
        let m = self.params.m;
        let fc = self.params.fc;

        // Depletion capacitance model
        let cgs = if vgs <= fc * pb {
            cgs0 / (1.0 - vgs / pb).powf(m)
        } else {
            // Forward bias region - use linear extrapolation
            let f1 = (1.0 - fc).powf(1.0 + m);
            let f2 = 1.0 + m * fc;
            cgs0 / f1 * (f2 + m * vgs / pb)
        };

        let cgd = if vgd <= fc * pb {
            cgd0 / (1.0 - vgd / pb).powf(m)
        } else {
            let f1 = (1.0 - fc).powf(1.0 + m);
            let f2 = 1.0 + m * fc;
            cgd0 / f1 * (f2 + m * vgd / pb)
        };

        (cgs.max(cgs0 * 0.01), cgd.max(cgd0 * 0.01))
    }

    /// Get IDSS (drain current at Vgs=0 in saturation)
    pub fn idss(&self) -> Value {
        self.params.beta * self.params.vto * self.params.vto * self.area * self.m
    }

    /// Link this device to a StaticMatrix for O(1) direct stamping.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.drain;
        let g = self.gate;
        let s = self.source;

        if d > 0 && d > 0 {
            self.indices.dd = matrix.get_index(d - 1, d - 1);
        }
        if d > 0 && g > 0 {
            self.indices.dg = matrix.get_index(d - 1, g - 1);
        }
        if d > 0 && s > 0 {
            self.indices.ds = matrix.get_index(d - 1, s - 1);
        }

        if g > 0 && d > 0 {
            self.indices.gd = matrix.get_index(g - 1, d - 1);
        }
        if g > 0 && g > 0 {
            self.indices.gg = matrix.get_index(g - 1, g - 1);
        }
        if g > 0 && s > 0 {
            self.indices.gs = matrix.get_index(g - 1, s - 1);
        }

        if s > 0 && d > 0 {
            self.indices.sd = matrix.get_index(s - 1, d - 1);
        }
        if s > 0 && g > 0 {
            self.indices.sg = matrix.get_index(s - 1, g - 1);
        }
        if s > 0 && s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after `link`).
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);

        let vgs = vg - vs;
        let vds = vd - vs;
        let vgd = vg - vd;

        let (ids, gm, gds_raw) = self.calculate(vgs, vds, self.params.tnom);
        let gds = gds_raw.max(1e-12);
        let ids_eq = ids - gm * vgs - gds * vds;
        let (igs, igd, ggs, ggd) = self.gate_junctions(vgs, vgd, self.params.tnom);
        let igs_eq = igs - ggs * vgs;
        let igd_eq = igd - ggd * vgd;

        // Drain row
        if let Some(idx) = self.indices.dd {
            matrix.stamp_direct(idx, gds + ggd);
        }
        if let Some(idx) = self.indices.dg {
            matrix.stamp_direct(idx, gm - ggd);
        }
        if let Some(idx) = self.indices.ds {
            matrix.stamp_direct(idx, -gm - gds);
        }

        // Gate row
        if let Some(idx) = self.indices.gd {
            matrix.stamp_direct(idx, -ggd);
        }
        if let Some(idx) = self.indices.gg {
            matrix.stamp_direct(idx, ggs + ggd);
        }
        if let Some(idx) = self.indices.gs {
            matrix.stamp_direct(idx, -ggs);
        }

        // Source row
        if let Some(idx) = self.indices.sd {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.sg {
            matrix.stamp_direct(idx, -gm - ggs);
        }
        if let Some(idx) = self.indices.ss {
            matrix.stamp_direct(idx, gm + gds + ggs);
        }

        if self.drain > 0 {
            rhs[self.drain - 1] -= ids_eq - igd_eq;
        }
        if self.gate > 0 {
            rhs[self.gate - 1] -= igs_eq + igd_eq;
        }
        if self.source > 0 {
            rhs[self.source - 1] += ids_eq + igs_eq;
        }
    }
}

impl NonlinearDevice for Jfet {
    fn update(&mut self, voltages: &[Value]) {
        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);

        self.vgs_prev = self.vgs;
        self.vds_prev = self.vds;
        self.vgs = vg - vs;
        self.vds = vd - vs;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);

        let vgs = vg - vs;
        let vds = vd - vs;
        let vgd = vg - vd;

        let (ids, gm, gds_raw) = self.calculate(vgs, vds, self.params.tnom);
        let gds = gds_raw.max(1e-12);
        let ids_eq = ids - gm * vgs - gds * vds;
        let (igs, igd, ggs, ggd) = self.gate_junctions(vgs, vgd, self.params.tnom);
        let igs_eq = igs - ggs * vgs;
        let igd_eq = igd - ggd * vgd;

        matrix.stamp(self.drain, self.drain, gds + ggd);
        matrix.stamp(self.drain, self.gate, gm - ggd);
        matrix.stamp(self.drain, self.source, -gm - gds);

        matrix.stamp(self.gate, self.drain, -ggd);
        matrix.stamp(self.gate, self.gate, ggs + ggd);
        matrix.stamp(self.gate, self.source, -ggs);

        matrix.stamp(self.source, self.drain, -gds);
        matrix.stamp(self.source, self.gate, -gm - ggs);
        matrix.stamp(self.source, self.source, gm + gds + ggs);

        matrix.stamp_rhs(self.drain, -ids_eq + igd_eq);
        matrix.stamp_rhs(self.gate, -igs_eq - igd_eq);
        matrix.stamp_rhs(self.source, ids_eq + igs_eq);
    }

    fn is_converged(&self, tolerance: Value) -> bool {
        (self.vgs - self.vgs_prev).abs() < tolerance && (self.vds - self.vds_prev).abs() < tolerance
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jfet_creation() {
        let jfet = Jfet::njf("J1", 1, 2, 3);
        assert_eq!(jfet.name, "J1");
        assert_eq!(jfet.jfet_type, JfetType::NJF);
        assert_eq!(jfet.drain, 1);
        assert_eq!(jfet.gate, 2);
        assert_eq!(jfet.source, 3);
    }

    #[test]
    fn test_pjf_creation() {
        let jfet = Jfet::pjf("J2", 1, 2, 3);
        assert_eq!(jfet.jfet_type, JfetType::PJF);
        assert_eq!(jfet.jfet_type.polarity(), -1.0);
    }

    #[test]
    fn test_cutoff_region() {
        let params = JfetParams::default(); // VTO = -2V
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Vgs = -3V < Vto = -2V → cutoff
        let (ids, gm, gds) = jfet.calculate(-3.0, 5.0, 300.0);

        assert!(ids.abs() < 1e-15, "Ids should be ~0 in cutoff, got {}", ids);
        assert!(gm.abs() < 1e-15, "gm should be 0 in cutoff");
        assert!(gds.abs() < 1e-15, "gds should be 0 in cutoff");
    }

    #[test]
    fn test_saturation_region() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Vgs = 0V, Vds = 5V → saturation (Vds > Vgs - Vto = 0 - (-2) = 2)
        let (ids, gm, gds) = jfet.calculate(0.0, 5.0, 300.0);

        // Ids = beta * (Vgs - Vto)² = 1e-3 * (0 - (-2))² = 1e-3 * 4 = 4mA
        assert!((ids - 4e-3).abs() < 1e-6, "Expected Ids=4mA, got {}", ids);

        // gm = 2 * beta * (Vgs - Vto) = 2 * 1e-3 * 2 = 4mS
        assert!((gm - 4e-3).abs() < 1e-6, "Expected gm=4mS, got {}", gm);

        // gds = 0 when lambda = 0
        assert!(gds.abs() < 1e-10, "gds should be ~0 with lambda=0");
    }

    #[test]
    fn test_linear_region() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Vgs = 0V, Vds = 1V → linear (Vds < Vgs - Vto = 2)
        let (ids, gm, gds) = jfet.calculate(0.0, 1.0, 300.0);

        // Ids = beta * (2*(Vgs-Vto)*Vds - Vds²) = 1e-3 * (2*2*1 - 1) = 1e-3 * 3 = 3mA
        assert!((ids - 3e-3).abs() < 1e-6, "Expected Ids=3mA, got {}", ids);

        // gm = 2 * beta * Vds = 2 * 1e-3 * 1 = 2mS
        assert!((gm - 2e-3).abs() < 1e-6, "Expected gm=2mS, got {}", gm);

        // gds = beta * 2 * (Vgst - Vds) = 1e-3 * 2 * (2 - 1) = 2mS
        assert!((gds - 2e-3).abs() < 1e-6, "Expected gds=2mS, got {}", gds);
    }

    #[test]
    fn test_reverse_vds_changes_current_and_gm_sign_for_njf() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        let (ids_fwd, gm_fwd, gds_fwd) = jfet.calculate(0.0, 1.0, 300.0);
        let (ids_rev, gm_rev, gds_rev) = jfet.calculate(0.0, -1.0, 300.0);

        assert!(
            ids_fwd > 0.0,
            "forward Ids should be positive, got {}",
            ids_fwd
        );
        assert!(
            gm_fwd > 0.0,
            "forward gm should be positive, got {}",
            gm_fwd
        );
        assert!(
            gds_fwd > 0.0,
            "forward gds should be positive, got {}",
            gds_fwd
        );

        assert!(
            ids_rev < 0.0,
            "reverse Vds should invert drain current direction, got {}",
            ids_rev
        );
        assert!(
            gm_rev < 0.0,
            "reverse Vds should invert gm sign in original terminal orientation, got {}",
            gm_rev
        );
        assert!(gds_rev > 0.0, "gds should remain positive, got {}", gds_rev);
    }

    #[test]
    fn test_channel_length_modulation() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.01);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Saturation with lambda > 0
        let (ids1, _, gds1) = jfet.calculate(0.0, 5.0, 300.0);
        let (ids2, _, _) = jfet.calculate(0.0, 10.0, 300.0);

        // Ids should increase with Vds due to lambda
        assert!(ids2 > ids1, "Ids should increase with Vds when lambda > 0");

        // gds = beta * Vgst² * lambda = 1e-3 * 4 * 0.01 = 40µS
        assert!(
            (gds1 - 40e-6).abs() < 1e-9,
            "Expected gds=40µS, got {}",
            gds1
        );
    }

    #[test]
    fn test_pjf_polarity() {
        // P-JFET uses same VTO sign convention as N-JFET in the model
        // The polarity multiplier handles the sign transformation
        let params = JfetParams::new()
            .with_vto(-2.0) // VTO=-2 (same as N-JFET)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::pjf("J1", 1, 2, 0).with_params(params);

        // P-JFET: Vgs = 0, Vds = -5V
        // Internal: vgs_int = -1*0 = 0, vds_int = -1*(-5) = 5
        // vgst = 0 - (-2) = 2 > 0, so device is ON
        // Saturation: vds_int=5 > vgst=2
        let (ids, _gm, _) = jfet.calculate(0.0, -5.0, 300.0);

        // Current flows opposite direction (negative Ids for P-JFET)
        // Internal ids = beta * vgst^2 = 1e-3 * 4 = 4mA
        // Output: pol * ids = -1 * 4mA = -4mA
        assert!(ids < 0.0, "P-JFET Ids should be negative, got {}", ids);
        assert!(
            (ids.abs() - 4e-3).abs() < 1e-6,
            "Expected |Ids|=4mA, got {}",
            ids.abs()
        );
    }

    #[test]
    fn test_idss_calculation() {
        let params = JfetParams::from_idss(10e-3, -2.0); // 10mA IDSS
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // IDSS = Ids at Vgs=0, saturation
        let (ids, _, _) = jfet.calculate(0.0, 10.0, 300.0);

        // Should be close to 10mA (exactly 10mA with lambda=0)
        assert!(
            (ids - 10e-3).abs() < 1e-6,
            "Expected Ids≈IDSS=10mA, got {}",
            ids
        );
    }

    #[test]
    fn test_gate_current() {
        let params = JfetParams::new().with_junction(1e-14, 0.8);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Reverse biased gate (normal operation)
        let (igs, igd) = jfet.gate_current(-1.0, -6.0, 300.0);

        // Should be very small (reverse saturation)
        assert!(
            igs.abs() < 1e-12,
            "Gate current should be tiny reverse biased"
        );
        assert!(igd.abs() < 1e-12, "Gate-drain current should be tiny");
    }

    #[test]
    fn test_gate_current_pjf_forward_bias_has_negative_gate_current() {
        let params = JfetParams::new().with_junction(1e-12, 0.8);
        let jfet = Jfet::pjf("J1", 1, 2, 0).with_params(params);

        // For P-JFET, source-to-gate forward bias means gate current is negative
        // when defined as current flowing from gate to source/drain.
        let (igs, igd) = jfet.gate_current(-0.6, -0.6, 300.0);
        assert!(
            igs < 0.0,
            "P-JFET gate-source current should be negative in forward bias, got {}",
            igs
        );
        assert!(
            igd < 0.0,
            "P-JFET gate-drain current should be negative in forward bias, got {}",
            igd
        );
    }

    #[test]
    fn test_gate_current_large_forward_is_finite() {
        let params = JfetParams::new().with_junction(1e-12, 0.8);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        let (igs, igd) = jfet.gate_current(100.0, 80.0, 300.0);

        assert!(
            igs.is_finite(),
            "Igs must remain finite under large forward bias"
        );
        assert!(
            igd.is_finite(),
            "Igd must remain finite under large forward bias"
        );
        assert!(
            igs > 0.0,
            "Igs should be positive for strongly forward-biased NJF"
        );
        assert!(
            igd > 0.0,
            "Igd should be positive for strongly forward-biased NJF"
        );
    }

    #[test]
    fn test_capacitances() {
        let params = JfetParams::new().with_capacitances(5e-12, 2e-12); // 5pF CGS, 2pF CGD
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Zero bias capacitances
        let (cgs, cgd) = jfet.capacitances(0.0, 0.0);

        assert!(
            (cgs - 5e-12).abs() < 1e-15,
            "CGS at zero bias should be 5pF"
        );
        assert!(
            (cgd - 2e-12).abs() < 1e-15,
            "CGD at zero bias should be 2pF"
        );

        // Reverse bias increases depletion width, decreases capacitance
        let (cgs_rev, _) = jfet.capacitances(-2.0, -5.0);
        assert!(cgs_rev < cgs, "CGS should decrease with reverse bias");
    }

    #[test]
    fn test_capacitances_scale_with_area_and_multiplier() {
        let params = JfetParams::new().with_capacitances(1e-12, 0.5e-12);
        let jfet = Jfet::njf("J1", 1, 2, 0)
            .with_params(params)
            .with_area(2.0)
            .with_multiplier(3.0);

        let (cgs, cgd) = jfet.capacitances(0.0, 0.0);
        assert!(
            (cgs - 6e-12).abs() < 1e-18,
            "expected CGS to scale with area*m, got {}",
            cgs
        );
        assert!(
            (cgd - 3e-12).abs() < 1e-18,
            "expected CGD to scale with area*m, got {}",
            cgd
        );
    }

    #[test]
    fn test_params_builder() {
        let params = JfetParams::new()
            .with_vto(-3.0)
            .with_beta(2e-3)
            .with_lambda(0.02)
            .with_capacitances(10e-12, 5e-12)
            .with_junction(1e-15, 0.9);

        assert_eq!(params.vto, -3.0);
        assert_eq!(params.beta, 2e-3);
        assert_eq!(params.lambda, 0.02);
        assert_eq!(params.cgs, 10e-12);
        assert_eq!(params.cgd, 5e-12);
        assert_eq!(params.is, 1e-15);
        assert_eq!(params.pb, 0.9);
    }

    #[test]
    fn test_with_model_params_derives_beta_from_idss_when_beta_absent() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VTO".to_string(), -3.0);
        model.insert("IDSS".to_string(), 12e-3);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        let expected = 12e-3 / 9.0;
        assert!(
            (jfet.params.beta - expected).abs() < 1e-15,
            "expected beta={} from IDSS/VTO^2, got {}",
            expected,
            jfet.params.beta
        );
    }

    #[test]
    fn test_with_model_params_explicit_beta_overrides_idss() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VTO".to_string(), -3.0);
        model.insert("IDSS".to_string(), 12e-3);
        model.insert("BETA".to_string(), 2.5e-3);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!(
            (jfet.params.beta - 2.5e-3).abs() < 1e-18,
            "explicit BETA should override IDSS-derived beta, got {}",
            jfet.params.beta
        );
    }

    #[test]
    fn test_with_model_params_applies_tnom_and_grading_coefficient() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("TNOM".to_string(), 325.0);
        model.insert("M".to_string(), 0.35);
        model.insert("FC".to_string(), 0.4);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!((jfet.params.tnom - 325.0).abs() < 1e-12);
        assert!((jfet.params.m - 0.35).abs() < 1e-12);
        assert!((jfet.params.fc - 0.4).abs() < 1e-12);
    }

    #[test]
    fn test_with_model_params_rejects_nonphysical_values() {
        use std::collections::HashMap;

        let baseline = Jfet::njf("J1", 1, 2, 0);
        let mut model = HashMap::new();
        model.insert("BETA".to_string(), -1e-3);
        model.insert("LAMBDA".to_string(), -0.1);
        model.insert("IS".to_string(), -1e-9);
        model.insert("PB".to_string(), 0.0);
        model.insert("FC".to_string(), 1.2);
        model.insert("N".to_string(), 0.0);
        model.insert("TNOM".to_string(), -10.0);
        model.insert("RD".to_string(), -10.0);
        model.insert("RS".to_string(), -12.0);

        let jfet = baseline.clone().with_model_params(&model);
        assert!(
            (jfet.params.beta - baseline.params.beta).abs() < 1e-30,
            "invalid BETA should be ignored"
        );
        assert!(
            (jfet.params.lambda - baseline.params.lambda).abs() < 1e-30,
            "invalid LAMBDA should be ignored"
        );
        assert!(
            (jfet.params.is - baseline.params.is).abs() < 1e-30,
            "invalid IS should be ignored"
        );
        assert!(
            (jfet.params.pb - baseline.params.pb).abs() < 1e-30,
            "invalid PB should be ignored"
        );
        assert!(
            (jfet.params.fc - baseline.params.fc).abs() < 1e-30,
            "invalid FC should be ignored"
        );
        assert!(
            (jfet.params.n - baseline.params.n).abs() < 1e-30,
            "invalid N should be ignored"
        );
        assert!(
            (jfet.params.tnom - baseline.params.tnom).abs() < 1e-30,
            "invalid TNOM should be ignored"
        );
        assert!(
            (jfet.params.rd - baseline.params.rd).abs() < 1e-30,
            "invalid RD should be ignored"
        );
        assert!(
            (jfet.params.rs - baseline.params.rs).abs() < 1e-30,
            "invalid RS should be ignored"
        );

        let (cgs, cgd) = jfet.capacitances(0.0, 0.0);
        assert!(cgs.is_finite() && cgd.is_finite());
    }

    #[test]
    fn test_with_model_params_accepts_hfet_aliases() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VT0".to_string(), 0.3);
        model.insert("RDI".to_string(), 12.0);
        model.insert("RSI".to_string(), 8.0);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!((jfet.params.vto - 0.3).abs() < 1e-15);
        assert!((jfet.params.rd - 12.0).abs() < 1e-15);
        assert!((jfet.params.rs - 8.0).abs() < 1e-15);
    }

    #[test]
    fn test_with_model_params_uses_m_greater_than_one_as_multiplier() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("M".to_string(), 2.5);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!((jfet.m - 2.5).abs() < 1e-15);
        assert!(
            (jfet.params.m - JfetParams::default().m).abs() < 1e-15,
            "grading coefficient should remain default when M is treated as multiplicity"
        );
    }
}

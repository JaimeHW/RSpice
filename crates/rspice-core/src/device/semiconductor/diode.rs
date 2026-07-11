//! Diode device model

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

/// Boltzmann over charge with ngspice-46's CODATA constants
/// (const.h: CONSTboltz / CHARGE).
const KOVERQ: Value = 1.38064852e-23 / 1.6021766208e-19;
/// Legacy physical constants retained by Xyce 7.x's SPICE diode model.
const XYCE_7_KOVERQ: Value = 1.3806226e-23 / 1.6021918e-19;
/// SPICE reference temperature, 27C in Kelvin (ngspice REFTEMP).
const REFTEMP: Value = 300.15;
const EPSMIN: Value = 1.0e-28;
const MAX_EXP_ARG: Value = 100.0;
const BREAKDOWN_EXP_ARG_MAX: Value = 40.0;

/// Pre-computed stamp indices for O(1) matrix access (2-terminal device)
#[derive(Debug, Clone, Default)]
pub struct DiodeIndices {
    /// Matrix stamps: (anode,anode), (anode,cathode), (cathode,anode), (cathode,cathode)
    pub aa: Option<CscIndex>,
    pub ac: Option<CscIndex>,
    pub ca: Option<CscIndex>,
    pub cc: Option<CscIndex>,
}

/// Semiconductor diode with Shockley equation model
#[derive(Debug, Clone)]
pub struct Diode {
    pub name: String,
    pub node_anode: NodeId,
    pub node_cathode: NodeId,
    /// Saturation current (Is)
    pub is: Value,
    /// Emission coefficient (N)
    pub n: Value,
    /// Thermal voltage (Vt = kT/q, ~26mV at room temp)
    pub vt: Value,
    /// Series resistance
    pub rs: Value,
    /// Reverse breakdown voltage. `None` disables the breakdown branch.
    pub bv: Option<Value>,
    /// Reverse current at breakdown knee.
    pub ibv: Value,
    /// Forward high-injection knee current (IKF/IK).
    pub forward_knee_current: Value,
    /// Reverse high-injection knee current (IKR).
    pub reverse_knee_current: Value,
    /// Recombination saturation current (ISR).
    pub recombination_saturation_current: Value,
    /// Recombination-current emission coefficient (NR).
    pub recombination_emission_coefficient: Value,
    /// Sidewall perimeter instance parameter (PJ).
    pub sidewall_perimeter: Value,
    /// Sidewall saturation current density/current parameter (JSW).
    pub sidewall_saturation_current: Value,
    /// Whether JSW was explicitly given on the model card.
    pub sidewall_current_given: bool,
    /// Sidewall emission coefficient (NS).
    pub sidewall_emission_coefficient: Value,
    /// Whether NS was explicitly given on the model card.
    pub sidewall_emission_given: bool,

    // Junction capacitance parameters
    /// Zero-bias junction capacitance (CJ0)
    pub cj0: Value,
    /// Built-in potential (VJ)
    pub vj: Value,
    /// Grading coefficient (M)
    pub m: Value,
    /// Transit time for diffusion capacitance (TT)
    pub tt: Value,
    /// Forward-bias depletion capacitance coefficient (FC)
    pub fc: Value,
    /// Sidewall zero-bias junction capacitance (CJSW/CJP)
    pub sidewall_cj0: Value,
    /// Sidewall built-in potential (PHP/VJSW)
    pub sidewall_vj: Value,
    /// Sidewall grading coefficient (MJSW)
    pub sidewall_m: Value,
    /// Sidewall forward-bias depletion capacitance coefficient (FCS)
    pub sidewall_fc: Value,
    /// Reverse breakdown ideality factor (NBV).
    pub breakdown_emission_coefficient: Value,
    /// Whether NBV was explicitly given on the model card.
    pub breakdown_emission_given: bool,

    // Temperature parameters
    /// Saturation-current temperature exponent (XTI)
    pub xti: Value,
    /// Activation energy in eV (EG)
    pub eg: Value,
    /// Model nominal temperature override in Celsius (TNOM)
    pub tnom_c: Option<Value>,
    /// Select the PSpice-compatible Xyce/HSPICE LEVEL=2 junction-temperature law.
    pub pspice_level2: bool,

    // Noise parameters
    /// Flicker noise coefficient (KF)
    pub kf: Value,
    /// Flicker noise current exponent (AF)
    pub af: Value,
    /// Instance multiplicity (M), kept apart from the folded junction
    /// scaling because dionoise.c rides flicker on `m·KF·|Id/m|^AF`.
    pub multiplicity: Value,
    /// Previous iteration voltage (for convergence check)
    prev_vd: Value,
    /// Previous voltage for convergence
    prev_vd_old: Value,
    /// Previous iteration current
    prev_id: Value,
    /// Engine-supplied junction shunt conductance (ngspice `CKTgmin`):
    /// zero in plain solves, raised by gmin-stepping/rescue ladders so the
    /// continuation can deform the diode system like every other junction.
    junction_gmin: Value,
    /// Junction voltage the last stamp linearized at — the `vold` of the
    /// pnjlim iteration-limiting history.
    last_limited_vd: std::cell::Cell<Value>,
    /// The last stamp's pnjlim clamped the junction; a limited step forces
    /// another Newton iteration (ngspice `Check` semantics).
    limited: std::cell::Cell<bool>,
    /// Junction voltage used by the last nonlinear stamp.
    last_stamp_vd: std::cell::Cell<Value>,
    /// Current used by the last nonlinear stamp, with junction gmin folded in.
    last_stamp_id: std::cell::Cell<Value>,
    /// Conductance used by the last nonlinear stamp, with junction gmin folded in.
    last_stamp_gd: std::cell::Cell<Value>,
    /// Folded AREA * M multiplier applied to the bottom junction.
    junction_scale: Value,
    /// Temperature-adjusted matched breakdown voltage (Xyce/ngspice tBrkdwnV).
    temperature_breakdown_voltage: Option<Value>,
    /// Pre-computed matrix indices for O(1) stamping
    pub indices: DiodeIndices,
}

impl Diode {
    /// Create a new diode with default 1N4148 parameters
    pub fn new(name: String, node_anode: NodeId, node_cathode: NodeId) -> Self {
        Self {
            name,
            node_anode,
            node_cathode,
            is: 2.52e-9,          // Saturation current
            n: 1.752,             // Emission coefficient
            vt: KOVERQ * REFTEMP, // Thermal voltage at 27C (ngspice REFTEMP)
            rs: 0.568,            // Series resistance
            bv: None,
            ibv: 1e-6,
            forward_knee_current: 0.0,
            reverse_knee_current: 0.0,
            recombination_saturation_current: 0.0,
            recombination_emission_coefficient: 2.0,
            sidewall_perimeter: 0.0,
            sidewall_saturation_current: 0.0,
            sidewall_current_given: false,
            sidewall_emission_coefficient: 1.0,
            sidewall_emission_given: false,

            // Junction capacitance (1N4148-like)
            cj0: 4e-12, // Zero-bias junction capacitance (4pF)
            vj: 0.7,    // Built-in potential
            m: 0.5,     // Grading coefficient
            tt: 8e-9,   // Transit time (8ns)
            fc: 0.5,    // Forward-bias depletion coefficient (SPICE default)
            sidewall_cj0: 0.0,
            sidewall_vj: 1.0,
            sidewall_m: 0.33,
            sidewall_fc: 0.5,
            breakdown_emission_coefficient: 1.752,
            breakdown_emission_given: false,

            // SPICE temperature defaults
            xti: 3.0,
            eg: 1.11,
            tnom_c: None,
            pspice_level2: false,

            // Flicker noise off by default (diosetup.c: fNcoef 0, fNexp 1).
            kf: 0.0,
            af: 1.0,
            multiplicity: 1.0,

            prev_vd: 0.0,
            prev_vd_old: 0.0,
            prev_id: 0.0,
            junction_gmin: 0.0,
            last_limited_vd: std::cell::Cell::new(0.0),
            limited: std::cell::Cell::new(false),
            last_stamp_vd: std::cell::Cell::new(0.0),
            last_stamp_id: std::cell::Cell::new(0.0),
            last_stamp_gd: std::cell::Cell::new(0.0),
            junction_scale: 1.0,
            temperature_breakdown_voltage: None,
            indices: DiodeIndices::default(),
        }
    }

    /// Engine hook: junction gmin for continuation ladders (mirrors the
    /// MOSFET/JFET/BJT `set_junction_gmin` convention).
    pub fn set_junction_gmin(&mut self, gmin: Value) {
        self.junction_gmin = if gmin.is_finite() { gmin.max(0.0) } else { 0.0 };
    }

    /// ngspice `DEVpnjlim` (devsup.c): junction-voltage iteration limiting.
    /// Returns the limited voltage and whether limiting engaged. The same
    /// math as the validated JFET port; the guard `|vnew-vold| > 2·vte`
    /// keeps the log arguments positive.
    fn pnjlim(vnew: Value, vold: Value, vte: Value, vcrit: Value) -> (Value, bool) {
        if vnew > vcrit && (vnew - vold).abs() > vte + vte {
            if vold > 0.0 {
                let arg = (vnew - vold) / vte;
                if arg > 0.0 {
                    return (vold + vte * (2.0 + (arg - 2.0).ln()), true);
                }
                return (vold - vte * (2.0 + (2.0 - arg).ln()), true);
            }
            return (vte * (vnew / vte).ln(), true);
        }
        if vnew < 0.0 {
            let arg = if vold > 0.0 {
                -vold - 1.0
            } else {
                2.0 * vold - 1.0
            };
            if vnew < arg {
                return (arg, true);
            }
        }
        (vnew, false)
    }

    /// Limit the junction voltage against the previous iterate and
    /// linearize there, folding the engine junction gmin in (dioload.c:
    /// `gd += CKTgmin; cd += CKTgmin·vd`). Returns `(vd, id, gd)`.
    ///
    /// Breakdown diodes keep the raw voltage: ngspice limits those through
    /// a dedicated branch around `-BV`, and clamping them with the plain
    /// forward law would fight the breakdown exponential.
    fn limited_linearization(&self, vd_raw: Value) -> (Value, Value, Value) {
        let vte = self.n * self.vt;
        let vcrit = vte
            * (vte
                / (std::f64::consts::SQRT_2
                    * self.effective_bottom_saturation_current().max(1e-300)))
            .ln();

        let (vd, limited) = if let Some(bv) = self.bv.filter(|bv| bv.is_finite() && *bv > 0.0) {
            let vtebrk = self.breakdown_emission_coefficient.max(EPSMIN) * self.vt;
            if vd_raw < 0.0_f64.min(-bv + 10.0 * vtebrk) {
                let transformed = -(vd_raw + bv);
                let old_transformed = -(self.last_limited_vd.get() + bv);
                let (limited_transformed, limited) =
                    Self::pnjlim(transformed, old_transformed, vtebrk, vcrit);
                (-(limited_transformed + bv), limited)
            } else {
                Self::pnjlim(vd_raw, self.last_limited_vd.get(), vte, vcrit)
            }
        } else {
            Self::pnjlim(vd_raw, self.last_limited_vd.get(), vte, vcrit)
        };
        self.limited.set(limited);
        self.last_limited_vd.set(vd);

        let (id, gd) = self.current_and_conductance(vd);
        let stamped_id = id + self.junction_gmin * vd;
        let stamped_gd = gd + self.junction_gmin;
        self.last_stamp_vd.set(vd);
        self.last_stamp_id.set(stamped_id);
        self.last_stamp_gd.set(stamped_gd);
        (vd, stamped_id, stamped_gd)
    }

    /// Return the flicker-noise coefficients `(KF, AF)`, if enabled by the
    /// model card. dionoise.c rides the source on the junction current as
    /// `m·KF·|Id/m|^AF / f`, so the caller folds `self.multiplicity` into
    /// the coefficient.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value)> {
        if self.kf > 0.0 && self.kf.is_finite() {
            Some((self.kf, self.af.max(1e-12)))
        } else {
            None
        }
    }

    /// Create a diode carrying ngspice's model-card defaults.
    ///
    /// `.model` parsing must start from these (IS=1e-14, N=1, RS=0, CJO=0,
    /// VJ=1, M=0.5, TT=0, IBV=1mA) so a card that omits a parameter gets
    /// SPICE semantics; `new()` keeps its 1N4148-like values as a standalone
    /// convenience, but seeding model cards from it silently gave every bare
    /// card 4 pF of junction capacitance, 8 ns of transit time, and half an
    /// ohm of series resistance that ngspice does not have.
    pub fn spice_defaults(name: String, node_anode: usize, node_cathode: usize) -> Self {
        let mut diode = Self::new(name, node_anode, node_cathode);
        diode.is = 1e-14;
        diode.n = 1.0;
        diode.rs = 0.0;
        diode.ibv = 1e-3;
        diode.cj0 = 0.0;
        diode.vj = 1.0;
        diode.m = 0.5;
        diode.tt = 0.0;
        diode.sidewall_saturation_current = 0.0;
        diode.sidewall_current_given = false;
        diode.sidewall_emission_coefficient = 1.0;
        diode.sidewall_emission_given = false;
        diode.sidewall_cj0 = 0.0;
        diode.sidewall_vj = 1.0;
        diode.sidewall_m = 0.33;
        diode.sidewall_fc = 0.5;
        diode.breakdown_emission_coefficient = diode.n;
        diode.breakdown_emission_given = false;
        diode.recombination_saturation_current = 0.0;
        diode.recombination_emission_coefficient = 2.0;
        diode
    }

    /// Create diode with custom DC model parameters
    pub fn with_params(mut self, is: Value, n: Value, rs: Value) -> Self {
        self.is = is;
        self.n = n;
        self.rs = rs;
        self
    }

    /// Set model parameters from a HashMap (for .MODEL statement parsing)
    pub fn with_model_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        self.pspice_level2 = params
            .get("LEVEL")
            .is_some_and(|level| level.is_finite() && (*level - 2.0).abs() <= 1.0e-9);
        if let Some(&v) = params.get("IS").or_else(|| params.get("JS")) {
            self.is = v;
        }
        if let Some(&v) = params.get("N") {
            self.n = v;
        }
        if let Some(&v) = params.get("RS") {
            self.rs = v;
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
        if let Some(&v) = params.get("BV").or_else(|| params.get("VB"))
            && v.is_finite()
            && v > 0.0
        {
            self.bv = Some(v);
        }
        if let Some(&v) = params.get("IBV")
            && v.is_finite()
            && v > 0.0
        {
            self.ibv = v;
        }
        if let Some(v) = params.get("IKF").or_else(|| params.get("IK")).copied() {
            self.forward_knee_current = if v.is_finite() && v >= EPSMIN { v } else { 0.0 };
        }
        if let Some(&v) = params.get("IKR") {
            self.reverse_knee_current = if v.is_finite() && v >= EPSMIN { v } else { 0.0 };
        }
        if let Some(&v) = params.get("ISR")
            && v.is_finite()
            && v >= 0.0
        {
            self.recombination_saturation_current = v;
        }
        if let Some(&v) = params.get("NR")
            && v.is_finite()
            && v > 0.0
        {
            self.recombination_emission_coefficient = v;
        }
        if let Some(&v) = params
            .get("CJO")
            .or_else(|| params.get("CJ0"))
            .or_else(|| params.get("CJ"))
        {
            self.cj0 = v;
        }
        if let Some(&v) = params.get("VJ") {
            self.vj = v;
        }
        if let Some(&v) = params.get("M") {
            self.m = v;
        }
        if let Some(&v) = params.get("TT") {
            self.tt = v;
        }
        if let Some(&v) = params.get("FC")
            && v.is_finite()
            && v >= 0.0
        {
            self.fc = v;
        }
        if let Some(&v) = params.get("JSW")
            && v.is_finite()
            && v >= 0.0
        {
            self.sidewall_saturation_current = v;
            self.sidewall_current_given = true;
        }
        if let Some(&v) = params.get("NS")
            && v.is_finite()
            && v > 0.0
        {
            self.sidewall_emission_coefficient = v;
            self.sidewall_emission_given = true;
        }
        if let Some(&v) = params.get("CJSW").or_else(|| params.get("CJP"))
            && v.is_finite()
            && v >= 0.0
        {
            self.sidewall_cj0 = v;
        }
        if let Some(&v) = params.get("PHP").or_else(|| params.get("VJSW"))
            && v.is_finite()
            && v > 0.0
        {
            self.sidewall_vj = v;
        }
        if let Some(&v) = params.get("MJSW")
            && v.is_finite()
        {
            self.sidewall_m = v;
        }
        if let Some(&v) = params.get("FCS")
            && v.is_finite()
            && v >= 0.0
        {
            self.sidewall_fc = v;
        }
        if let Some(&v) = params.get("NBV")
            && v.is_finite()
            && v > 0.0
        {
            self.breakdown_emission_coefficient = v;
            self.breakdown_emission_given = true;
        }
        if let Some(&v) = params.get("XTI") {
            self.xti = v;
        }
        if let Some(&v) = params.get("EG") {
            self.eg = v;
        }
        if let Some(&v) = params.get("TNOM") {
            self.tnom_c = Some(v);
        }
        if !self.breakdown_emission_given {
            self.breakdown_emission_coefficient = self.n;
        }
        self
    }

    pub(crate) fn remap_nodes(&mut self, old_node_id: NodeId) {
        fn remap_node_id(id: NodeId, old_id: NodeId) -> NodeId {
            if id == old_id {
                0
            } else if id > old_id {
                id - 1
            } else {
                id
            }
        }
        self.node_anode = remap_node_id(self.node_anode, old_node_id);
        self.node_cathode = remap_node_id(self.node_cathode, old_node_id);
    }

    /// Scale the junction to the operating temperature.
    ///
    /// Mirrors ngspice's diotemp.c and Xyce's `Instance::updateTemperature`
    /// at the SPICE3 default temperature level: IS/JSW follow the
    /// activation-energy / XTI law, VJ/CJ0 and PHP/CJSW follow the
    /// bandgap-shift mapping, and BV is converted to the matched
    /// breakdown voltage used by the reverse branch.
    ///
    /// Call once after model parameters and junction scaling are applied.
    pub fn set_temperature(&mut self, temp_kelvin: Value, default_tnom_kelvin: Value) {
        self.set_temperature_with_k_over_q(temp_kelvin, default_tnom_kelvin, KOVERQ);
    }

    /// Scale the junction using Xyce 7.x's legacy Boltzmann/charge ratio.
    ///
    /// Xyce's open SPICE models intentionally retain their historical
    /// physical constants. The small difference is observable in precision
    /// regression work, so the Xyce compatibility dialect selects this path
    /// while native/ngspice operation keeps the modern default above.
    pub fn set_temperature_xyce_7(&mut self, temp_kelvin: Value, default_tnom_kelvin: Value) {
        self.set_temperature_with_k_over_q(temp_kelvin, default_tnom_kelvin, XYCE_7_KOVERQ);
    }

    fn set_temperature_with_k_over_q(
        &mut self,
        temp_kelvin: Value,
        default_tnom_kelvin: Value,
        k_over_q: Value,
    ) {
        let tnom = self
            .tnom_c
            .map(|c| c + 273.15)
            .unwrap_or(default_tnom_kelvin);
        let temp = temp_kelvin;
        if !temp.is_finite() || temp <= 0.0 || !tnom.is_finite() || tnom <= 0.0 {
            return;
        }

        let vt = k_over_q * temp;
        let vtnom = k_over_q * tnom;

        // Silicon bandgap at both temperatures (TLEV 0/1 form).
        let egfet = 1.16 - (7.02e-4 * temp * temp) / (temp + 1108.0);
        let egfet1 = 1.16 - (7.02e-4 * tnom * tnom) / (tnom + 1108.0);
        let fact1 = tnom / REFTEMP;
        let fact2 = temp / REFTEMP;
        // ngspice's CHARGE*arg terms, folded through k/q so everything
        // stays in volts.
        let arg = -egfet / (2.0 * vt) + 1.1150877 / (2.0 * k_over_q * REFTEMP);
        let arg1 = -egfet1 / (2.0 * vtnom) + 1.1150877 / (2.0 * k_over_q * REFTEMP);
        let pbfact = -2.0 * vt * (1.5 * fact2.ln() + arg);
        let pbfact1 = -2.0 * vtnom * (1.5 * fact1.ln() + arg1);

        // IS(T) = IS * exp(((T/TNOM)-1)*EG/(N*vt) + (XTI/N)*ln(T/TNOM))
        let vte = self.n * vt;
        let is_factor = (((temp / tnom) - 1.0) * self.eg / vte
            + (self.xti / self.n) * (temp / tnom).ln())
        .exp();
        if is_factor.is_finite() && is_factor > 0.0 {
            self.is *= is_factor;
        }
        let ns = self.sidewall_emission_coefficient.max(EPSMIN);
        let sidewall_factor = (((temp / tnom) - 1.0) * self.eg / (ns * vt)
            + (self.xti / ns) * (temp / tnom).ln())
        .exp();
        if sidewall_factor.is_finite() && sidewall_factor > 0.0 {
            self.sidewall_saturation_current *= sidewall_factor;
        }
        let nr = self.recombination_emission_coefficient.max(EPSMIN);
        let recombination_factor = (((temp / tnom) - 1.0) * self.eg / (nr * vt)
            + (self.xti / nr) * (temp / tnom).ln())
        .exp();
        if recombination_factor.is_finite() && recombination_factor > 0.0 {
            self.recombination_saturation_current *= recombination_factor;
        }

        // VJ(T) and CJ0(T), TLEVC=0 bandgap mapping.
        if self.pspice_level2 && self.vj > 0.0 {
            // Xyce's LEVEL=2 branch is the PSpice temperature law. Keep this
            // profile explicit: it differs materially from the SPICE3 law
            // when TNOM is not the 27 C reference temperature.
            let nominal_vj = self.vj;
            let t_jct_pot = (nominal_vj - egfet1) * fact2 - 3.0 * vt * fact2.ln() + egfet;
            let denominator =
                1.0 + self.m * (400e-6 * (temp - tnom) + (1.0 - t_jct_pot / nominal_vj));
            if t_jct_pot.is_finite()
                && t_jct_pot > 0.0
                && denominator.is_finite()
                && denominator > 0.0
            {
                let cj = self.cj0 / denominator;
                if cj.is_finite() && cj >= 0.0 {
                    self.vj = t_jct_pot;
                    self.cj0 = cj;
                }
            }
        } else if self.vj > 0.0 {
            let pbo = (self.vj - pbfact1) / fact1;
            if pbo > 0.0 {
                let gmaold = (self.vj - pbo) / pbo;
                let denom = 1.0 + self.m * (400e-6 * (tnom - REFTEMP) - gmaold);
                let t_jct_pot = pbfact + fact2 * pbo;
                if denom != 0.0 && t_jct_pot > 0.0 {
                    let mut cj = self.cj0 / denom;
                    let gmanew = (t_jct_pot - pbo) / pbo;
                    cj *= 1.0 + self.m * (400e-6 * (temp - REFTEMP) - gmanew);
                    if cj.is_finite() && cj >= 0.0 {
                        self.vj = t_jct_pot;
                        self.cj0 = cj;
                    }
                }
            }
        }
        if self.sidewall_vj > 0.0 {
            let pbo = (self.sidewall_vj - pbfact1) / fact1;
            if pbo > 0.0 {
                let gmaold = (self.sidewall_vj - pbo) / pbo;
                let denom = 1.0 + self.sidewall_m * (400e-6 * (tnom - REFTEMP) - gmaold);
                let t_jct_pot = pbfact + fact2 * pbo;
                if denom != 0.0 && t_jct_pot > 0.0 {
                    let mut cj = self.sidewall_cj0 / denom;
                    let gmanew = (t_jct_pot - pbo) / pbo;
                    cj *= 1.0 + self.sidewall_m * (400e-6 * (temp - REFTEMP) - gmanew);
                    if cj.is_finite() && cj >= 0.0 {
                        self.sidewall_vj = t_jct_pot;
                        self.sidewall_cj0 = cj;
                    }
                }
            }
        }

        self.vt = vt;
        self.temperature_breakdown_voltage = self
            .bv
            .and_then(|bv| self.xbv_matched_breakdown_voltage(bv));
    }

    /// Cached operating-point values from the last accepted Newton solution:
    /// `(vd, id, gd)` — junction voltage, current, and conductance.
    pub fn op_values(&self) -> (Value, Value, Value, Value) {
        let (id, gd) = self.current_and_conductance(self.prev_vd);
        let (_, cd) = self.junction_charge_and_capacitance(self.prev_vd);
        (self.prev_vd, id, gd, cd)
    }

    /// Apply a parallel-junction scale factor (instance `AREA` × `M`).
    ///
    /// Saturation and breakdown-knee currents and the zero-bias depletion
    /// capacitance scale with the number of parallel junctions; series
    /// resistance scales inversely. Mirrors ngspice's AREA/M handling for
    /// the lumped diode.
    pub fn apply_junction_scaling(&mut self, scale: Value) {
        self.junction_scale *= scale;
        self.is *= scale;
        self.ibv *= scale;
        self.forward_knee_current *= scale;
        self.reverse_knee_current *= scale;
        self.recombination_saturation_current *= scale;
        self.cj0 *= scale;
        if self.rs > 0.0 {
            self.rs /= scale;
        }
    }

    /// Set the sidewall perimeter scale (instance `PJ`, multiplied by `M`).
    pub fn set_sidewall_perimeter(&mut self, perimeter: Value) {
        self.sidewall_perimeter = if perimeter.is_finite() && perimeter > 0.0 {
            perimeter
        } else {
            0.0
        };
    }

    /// Set junction capacitance parameters
    pub fn with_capacitance(mut self, cj0: Value, vj: Value, m: Value, tt: Value) -> Self {
        self.cj0 = cj0;
        self.vj = vj;
        self.m = m;
        self.tt = tt;
        self
    }

    /// Junction charge and capacitance at `vd` for transient integration
    /// (dioload.c): depletion charge with the F1/F2/F3 polynomial
    /// continuation above `FC·VJ`, plus diffusion charge `TT·id` riding the
    /// conduction current. Returns `(qd, capd)`.
    ///
    /// `vj`/`cj0` are already temperature-adjusted by `set_temperature`, so
    /// the F-coefficients computed here match ngspice's `DIOtF1`/`DIOf2`/
    /// `DIOf3` (diotemp.c evaluates them from the adjusted junction
    /// potential).
    pub fn junction_charge_and_capacitance(&self, vd: Value) -> (Value, Value) {
        let mut qd = 0.0;
        let mut capd = 0.0;
        let (bottom_q, bottom_c) =
            Self::depletion_charge_and_capacitance(vd, self.cj0, self.vj, self.m, self.fc);
        qd += bottom_q;
        capd += bottom_c;

        let sidewall_cj0 = self.sidewall_cj0 * self.sidewall_perimeter;
        let (sidewall_q, sidewall_c) = Self::depletion_charge_and_capacitance(
            vd,
            sidewall_cj0,
            self.sidewall_vj,
            self.sidewall_m,
            self.sidewall_fc,
        );
        qd += sidewall_q;
        capd += sidewall_c;

        if self.tt > 0.0 {
            let (id, gd) = self.current_and_conductance(vd);
            qd += self.tt * id;
            capd += self.tt * gd;
        }
        (qd, capd)
    }

    /// Calculate junction capacitance: Cj = CJ0 / (1 - Vd/VJ)^M
    /// Includes depletion (junction) and diffusion capacitance
    pub fn junction_capacitance(&self, vd: Value) -> Value {
        self.junction_charge_and_capacitance(vd).1
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let a = self.node_anode;
        let c = self.node_cathode;

        if a > 0 {
            self.indices.aa = matrix.get_index(a - 1, a - 1);
        }
        if a > 0 && c > 0 {
            self.indices.ac = matrix.get_index(a - 1, c - 1);
        }
        if c > 0 && a > 0 {
            self.indices.ca = matrix.get_index(c - 1, a - 1);
        }
        if c > 0 {
            self.indices.cc = matrix.get_index(c - 1, c - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let vd_raw = self.terminal_voltage(voltages);
        // Iteration-limited linearization with junction gmin folded in.
        let (vd, id, gd) = self.limited_linearization(vd_raw);
        self.stamp_linearized_direct(matrix, rhs, vd, id, gd);
    }

    /// Stamp a static residual probe at the candidate voltage itself.
    ///
    /// Newton iteration uses `limited_linearization` to protect live steps.
    /// Residual validation must instead rebuild the companion at the actual
    /// candidate bias, otherwise a limiter companion can masquerade as a
    /// converged nonlinear solution.
    pub(crate) fn stamp_static_probe_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        let vd = self.terminal_voltage(voltages);
        let (id, gd) = self.current_and_conductance(vd);
        let stamped_id = id + self.junction_gmin * vd;
        let stamped_gd = gd + self.junction_gmin;
        self.stamp_linearized_direct(matrix, rhs, vd, stamped_id, stamped_gd);
    }

    fn terminal_voltage(&self, voltages: &[Value]) -> Value {
        let va = if self.node_anode == 0 {
            0.0
        } else {
            voltages[self.node_anode - 1]
        };
        let vc = if self.node_cathode == 0 {
            0.0
        } else {
            voltages[self.node_cathode - 1]
        };
        va - vc
    }

    fn stamp_linearized_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        vd: Value,
        id: Value,
        gd: Value,
    ) {
        // Equivalent current source: ieq = id - gd * vd
        let ieq = id - gd * vd;

        // Stamp conductance using direct indexing
        if let Some(idx) = self.indices.aa {
            matrix.stamp_direct(idx, gd);
        }
        if let Some(idx) = self.indices.ac {
            matrix.stamp_direct(idx, -gd);
        }
        if let Some(idx) = self.indices.ca {
            matrix.stamp_direct(idx, -gd);
        }
        if let Some(idx) = self.indices.cc {
            matrix.stamp_direct(idx, gd);
        }

        // Stamp RHS (still needs bounds check for ground nodes)
        if self.node_anode > 0 {
            rhs[self.node_anode - 1] -= ieq;
        }
        if self.node_cathode > 0 {
            rhs[self.node_cathode - 1] += ieq;
        }
    }

    /// Shockley diode equation: I = Is * (exp(Vd / (N * Vt)) - 1)
    /// Public for noise analysis (shot noise = 2qI)
    pub fn current(&self, vd: Value) -> Value {
        self.current_and_conductance(vd).0
    }

    /// Junction current and conductance in one evaluation.
    ///
    /// Xyce treats JSW without NS as extra saturation current on the bottom
    /// diode, while JSW with NS is a distinct sidewall diode on the same
    /// junction voltage. That distinction matters for high-injection limiting
    /// and for the sidewall current shape.
    fn current_and_conductance(&self, vd: Value) -> (Value, Value) {
        let (forward_i, forward_g) = self.bottom_current_and_conductance(vd);
        let (recombination_i, recombination_g) = self.recombination_current_and_conductance(vd);
        let (sidewall_i, sidewall_g) = self.separate_sidewall_current_and_conductance(vd);
        (
            forward_i + recombination_i + sidewall_i,
            forward_g + recombination_g + sidewall_g,
        )
    }

    fn bottom_current_and_conductance(&self, vd: Value) -> (Value, Value) {
        let isat = self.effective_bottom_saturation_current();
        let (current, conductance) = self.exponential_current_and_conductance(vd, isat, self.n);
        self.apply_high_injection_knee(vd, current, conductance)
    }

    fn separate_sidewall_current_and_conductance(&self, vd: Value) -> (Value, Value) {
        if !(self.sidewall_current_given && self.sidewall_emission_given) {
            return (0.0, 0.0);
        }

        let isat = self.sidewall_saturation_current * self.sidewall_perimeter;
        self.exponential_current_and_conductance(vd, isat, self.sidewall_emission_coefficient)
    }

    fn effective_bottom_saturation_current(&self) -> Value {
        let sidewall = if self.sidewall_current_given && !self.sidewall_emission_given {
            self.sidewall_saturation_current * self.sidewall_perimeter
        } else {
            0.0
        };
        self.is + sidewall
    }

    /// Xyce/PSpice depletion-region recombination current.
    ///
    /// Xyce evaluates this branch only in its forward/near-zero region,
    /// selected by the ordinary emission voltage `N*Vt`. The exponential
    /// itself uses `NR*Vt`, then the depletion generation factor shapes both
    /// the current and its analytic Jacobian. Linear reverse bias and
    /// avalanche breakdown deliberately omit ISR in Xyce 7.10.
    fn recombination_current_and_conductance(&self, vd: Value) -> (Value, Value) {
        let isr = self.recombination_saturation_current;
        let nr = self.recombination_emission_coefficient;
        let forward_boundary = -3.0 * self.n.max(EPSMIN) * self.vt;
        if !(isr.is_finite()
            && isr > 0.0
            && nr.is_finite()
            && nr > 0.0
            && self.vt.is_finite()
            && self.vt > 0.0
            && self.vj.is_finite()
            && self.vj > 0.0
            && vd >= forward_boundary)
        {
            return (0.0, 0.0);
        }

        let nr_vt = nr * self.vt;
        let (exponential, exponential_derivative) = Self::limited_exp(vd / nr_vt, MAX_EXP_ARG);
        let base_current = isr * (exponential - 1.0);
        let base_conductance = (isr / nr_vt) * exponential_derivative;

        let normalized_depletion = 1.0 - vd / self.vj;
        let generation_base = normalized_depletion * normalized_depletion + 0.005;
        let generation_exponent = 0.5 * self.m;
        let generation_factor = generation_base.powf(generation_exponent);
        let generation_derivative = -self.m * normalized_depletion / self.vj
            * generation_base.powf(generation_exponent - 1.0);
        if !(generation_factor.is_finite() && generation_derivative.is_finite()) {
            return (0.0, 0.0);
        }

        (
            base_current * generation_factor,
            base_conductance * generation_factor + base_current * generation_derivative,
        )
    }

    fn exponential_current_and_conductance(
        &self,
        vd: Value,
        isat: Value,
        emission_coefficient: Value,
    ) -> (Value, Value) {
        if !(isat.is_finite() && isat > 0.0 && emission_coefficient.is_finite()) {
            return (0.0, 0.0);
        }

        let n_vt = emission_coefficient.max(EPSMIN) * self.vt;
        if vd >= -3.0 * n_vt {
            let (e, de_darg) = Self::limited_exp(vd / n_vt, MAX_EXP_ARG);
            return (isat * (e - 1.0), (isat / n_vt) * de_darg);
        }

        if let Some(brkdwn_v) = self.active_breakdown_voltage()
            && vd < -brkdwn_v
        {
            let vtebrk = self.breakdown_emission_coefficient.max(EPSMIN) * self.vt;
            let (e, de_darg) = Self::limited_exp(-(brkdwn_v + vd) / vtebrk, BREAKDOWN_EXP_ARG_MAX);
            return (-isat * e, (isat / vtebrk) * de_darg);
        }

        let mut arg = 3.0 * n_vt / (vd * std::f64::consts::E);
        arg = arg * arg * arg;
        (-isat * (1.0 + arg), isat * 3.0 * arg / vd)
    }

    fn active_breakdown_voltage(&self) -> Option<Value> {
        if let Some(brkdwn_v) = self.temperature_breakdown_voltage
            && brkdwn_v.is_finite()
            && brkdwn_v > 0.0
        {
            return Some(brkdwn_v);
        }

        self.bv.and_then(|bv| {
            if bv.is_finite() && bv > 0.0 {
                self.xbv_matched_breakdown_voltage(bv).or(Some(bv))
            } else {
                None
            }
        })
    }

    fn xbv_matched_breakdown_voltage(&self, bv: Value) -> Option<Value> {
        let vt = self.vt;
        let isat = self.is.max(1e-300);
        let cbv = self.ibv;
        let nbv = self.breakdown_emission_coefficient.max(EPSMIN);
        if !(bv.is_finite()
            && bv > 0.0
            && vt.is_finite()
            && vt > 0.0
            && cbv.is_finite()
            && cbv > 0.0)
        {
            return None;
        }

        if cbv < isat * bv / vt {
            return Some(bv);
        }

        let reltol = 1.0e-3;
        let tol = reltol * cbv;
        let mut xbv = bv - nbv * vt * (1.0 + cbv / isat).ln();
        for _ in 0..25 {
            xbv = bv - nbv * vt * (cbv / isat + 1.0 - xbv / vt).ln();
            let xcbv = isat * (((bv - xbv) / (nbv * vt)).exp() - 1.0 + xbv / vt);
            if (xcbv - cbv).abs() <= tol {
                break;
            }
        }
        if xbv.is_finite() && xbv > 0.0 {
            Some(xbv)
        } else {
            Some(bv)
        }
    }

    fn limited_exp(arg: Value, max_arg: Value) -> (Value, Value) {
        if arg <= max_arg {
            let exp_arg = arg.exp();
            return (exp_arg, exp_arg);
        }

        let exp_max = max_arg.exp();
        (exp_max * (1.0 + arg - max_arg), exp_max)
    }

    fn depletion_charge_and_capacitance(
        vd: Value,
        cj0: Value,
        vj: Value,
        grading: Value,
        fc: Value,
    ) -> (Value, Value) {
        if !(cj0 > 0.0 && vj > 0.0 && grading < 1.0) {
            return (0.0, 0.0);
        }

        let fc = fc.clamp(0.0, 0.95);
        let dep_cap_knee = fc * vj;
        if vd < dep_cap_knee {
            let arg = 1.0 - vd / vj;
            let sarg_arg = (-grading * arg.ln()).min(MAX_EXP_ARG);
            let sarg = sarg_arg.exp();
            let charge = vj * cj0 * (1.0 - arg * sarg) / (1.0 - grading);
            let capacitance = cj0 * sarg;
            return (charge, capacitance);
        }

        let f1 = vj * (1.0 - (1.0 - fc).powf(1.0 - grading)) / (1.0 - grading);
        let f2 = (1.0 - fc).powf(1.0 + grading);
        let f3 = 1.0 - fc * (1.0 + grading);
        let czof2 = cj0 / f2;
        let charge = cj0 * f1
            + czof2
                * (f3 * (vd - dep_cap_knee)
                    + (grading / (2.0 * vj)) * (vd * vd - dep_cap_knee * dep_cap_knee));
        let capacitance = czof2 * (f3 + grading * vd / vj);
        (charge, capacitance)
    }

    fn apply_high_injection_knee(
        &self,
        vd: Value,
        current: Value,
        conductance: Value,
    ) -> (Value, Value) {
        let n_vt = self.n * self.vt;
        if vd >= -3.0 * n_vt {
            return Self::apply_forward_knee(current, conductance, self.forward_knee_current);
        }
        Self::apply_reverse_knee(current, conductance, self.reverse_knee_current)
    }

    fn apply_forward_knee(
        current: Value,
        conductance: Value,
        knee_current: Value,
    ) -> (Value, Value) {
        if !(knee_current > 0.0 && current > 1.0e-18) {
            return (current, conductance);
        }
        let sqrt_knee = (current / knee_current).sqrt();
        let denominator = 1.0 + 2.0 * sqrt_knee + current / knee_current;
        let limited_conductance = ((1.0 + sqrt_knee) * conductance
            - current * conductance / (2.0 * sqrt_knee * knee_current))
            / denominator;
        (current / (1.0 + sqrt_knee), limited_conductance)
    }

    fn apply_reverse_knee(
        current: Value,
        conductance: Value,
        knee_current: Value,
    ) -> (Value, Value) {
        if !(knee_current > 0.0 && current < -1.0e-18) {
            return (current, conductance);
        }
        let sqrt_knee = (current / -knee_current).sqrt();
        let denominator = 1.0 + 2.0 * sqrt_knee - current / knee_current;
        let limited_conductance = ((1.0 + sqrt_knee) * conductance
            + current * conductance / (2.0 * sqrt_knee * knee_current))
            / denominator;
        (current / (1.0 + sqrt_knee), limited_conductance)
    }

    fn linearized_current_matches_candidate(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        let candidate_current = self.current(self.prev_vd) + self.junction_gmin * self.prev_vd;
        let predicted_current = self.last_stamp_id.get()
            + self.last_stamp_gd.get() * (self.prev_vd - self.last_stamp_vd.get());
        let tolerance = criteria.current_tolerance()
            + criteria.relative_tolerance() * candidate_current.abs().max(predicted_current.abs());
        (candidate_current - predicted_current).abs() <= tolerance
    }
}

impl NonlinearDevice for Diode {
    fn update(&mut self, voltages: &[Value]) {
        let va = if self.node_anode == 0 {
            0.0
        } else {
            voltages[self.node_anode - 1]
        };
        let vc = if self.node_cathode == 0 {
            0.0
        } else {
            voltages[self.node_cathode - 1]
        };
        self.prev_vd_old = self.prev_vd;
        self.prev_vd = va - vc;
        self.prev_id = self.current(self.prev_vd);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let va = if self.node_anode == 0 {
            0.0
        } else {
            voltages[self.node_anode - 1]
        };
        let vc = if self.node_cathode == 0 {
            0.0
        } else {
            voltages[self.node_cathode - 1]
        };
        // Iteration-limited linearization with junction gmin folded in.
        let (vd, id, gd) = self.limited_linearization(va - vc);

        // Equivalent current source: ieq = id - gd * vd
        let ieq = id - gd * vd;

        // Stamp conductance
        matrix.stamp(self.node_anode, self.node_anode, gd);
        matrix.stamp(self.node_anode, self.node_cathode, -gd);
        matrix.stamp(self.node_cathode, self.node_anode, -gd);
        matrix.stamp(self.node_cathode, self.node_cathode, gd);

        // Stamp equivalent current source
        matrix.stamp_rhs(self.node_anode, -ieq);
        matrix.stamp_rhs(self.node_cathode, ieq);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        let tolerance = criteria.voltage_tolerance();
        // A pnjlim-clamped step must iterate again regardless of the
        // voltage delta (ngspice `Check` semantics).
        !self.limited.get()
            && (self.prev_vd - self.prev_vd_old).abs() < tolerance
            && self.linearized_current_matches_candidate(criteria)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_diode() -> Diode {
        let mut d = Diode::new("d1".to_string(), 1, 2);
        d.is = 1e-14;
        d.n = 1.0;
        d.vj = 0.8;
        d.cj0 = 2e-12;
        d.m = 0.4;
        d
    }

    #[test]
    fn temperature_scaling_is_identity_at_tnom() {
        let mut d = test_diode();
        let (is0, vj0, cj0) = (d.is, d.vj, d.cj0);
        d.set_temperature(REFTEMP, REFTEMP);
        assert_eq!(d.vt, KOVERQ * REFTEMP);
        assert!((d.is - is0).abs() <= is0 * 1e-12);
        assert!((d.vj - vj0).abs() <= vj0 * 1e-12, "vj={} vs {}", d.vj, vj0);
        assert!((d.cj0 - cj0).abs() <= cj0 * 1e-12);
    }

    #[test]
    fn xyce_7_compatibility_uses_its_published_legacy_thermal_constant() {
        let mut d = test_diode();
        d.set_temperature_xyce_7(REFTEMP, REFTEMP);
        assert_eq!(d.vt, XYCE_7_KOVERQ * REFTEMP);
        assert!(
            d.vt < KOVERQ * REFTEMP,
            "Xyce 7's historical k/q must remain distinct from the native constant"
        );
    }

    #[test]
    fn saturation_current_follows_ngspice_diotemp_law() {
        // IS(T) = IS * exp(((T/TNOM)-1)*EG/(N*vt) + (XTI/N)*ln(T/TNOM))
        let mut d = test_diode();
        let temp = 273.15 + 85.0;
        let tnom = REFTEMP;
        d.set_temperature(temp, tnom);

        let vt = KOVERQ * temp;
        let expected = 1e-14 * (((temp / tnom) - 1.0) * 1.11 / vt + 3.0 * (temp / tnom).ln()).exp();
        assert!(
            (d.is - expected).abs() <= expected * 1e-12,
            "is={:e} expected={:e}",
            d.is,
            expected
        );
        assert_eq!(d.vt, vt);
    }

    #[test]
    fn junction_potential_drops_with_temperature() {
        // The bandgap mapping lowers VJ and raises CJ0 as silicon heats up.
        let mut d = test_diode();
        d.set_temperature(273.15 + 85.0, REFTEMP);
        assert!(d.vj < 0.8, "vj={} should drop at 85C", d.vj);
        assert!(d.cj0 > 2e-12, "cj0={} should rise at 85C", d.cj0);
    }
}

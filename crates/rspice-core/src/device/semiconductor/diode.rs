//! Diode device model

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

/// Boltzmann over charge with ngspice-46's CODATA constants
/// (const.h: CONSTboltz / CHARGE).
const KOVERQ: Value = 1.38064852e-23 / 1.6021766208e-19;
/// SPICE reference temperature, 27C in Kelvin (ngspice REFTEMP).
const REFTEMP: Value = 300.15;

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

    // Temperature parameters
    /// Saturation-current temperature exponent (XTI)
    pub xti: Value,
    /// Activation energy in eV (EG)
    pub eg: Value,
    /// Model nominal temperature override in Celsius (TNOM)
    pub tnom_c: Option<Value>,

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
            is: 2.52e-9, // Saturation current
            n: 1.752,    // Emission coefficient
            vt: KOVERQ * REFTEMP, // Thermal voltage at 27C (ngspice REFTEMP)
            rs: 0.568,   // Series resistance
            bv: None,
            ibv: 1e-6,

            // Junction capacitance (1N4148-like)
            cj0: 4e-12, // Zero-bias junction capacitance (4pF)
            vj: 0.7,    // Built-in potential
            m: 0.5,     // Grading coefficient
            tt: 8e-9,   // Transit time (8ns)
            fc: 0.5,    // Forward-bias depletion coefficient (SPICE default)

            // SPICE temperature defaults
            xti: 3.0,
            eg: 1.11,
            tnom_c: None,

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
        let vd = if self.bv.is_none() {
            let vte = self.n * self.vt;
            let vcrit = vte * (vte / (std::f64::consts::SQRT_2 * self.is.max(1e-300))).ln();
            let (vd, limited) = Self::pnjlim(vd_raw, self.last_limited_vd.get(), vte, vcrit);
            self.limited.set(limited);
            self.last_limited_vd.set(vd);
            vd
        } else {
            self.limited.set(false);
            self.last_limited_vd.set(vd_raw);
            vd_raw
        };
        let (id, gd) = self.current_and_conductance(vd);
        (vd, id + self.junction_gmin * vd, gd + self.junction_gmin)
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
        if let Some(&v) = params.get("IS") {
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
        if let Some(&v) = params.get("BV")
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
        if let Some(&v) = params.get("CJO") {
            self.cj0 = v;
        }
        if let Some(&v) = params.get("CJ0") {
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
        if let Some(&v) = params.get("XTI") {
            self.xti = v;
        }
        if let Some(&v) = params.get("EG") {
            self.eg = v;
        }
        if let Some(&v) = params.get("TNOM") {
            self.tnom_c = Some(v);
        }
        self
    }

    /// Scale the junction to the operating temperature.
    ///
    /// Mirrors ngspice's diotemp.c at TLEV=0 / TLEVC=0 (the SPICE3
    /// default): IS follows the activation-energy / XTI law, and VJ and
    /// CJ0 follow the bandgap-shift mapping. The formulas reduce to
    /// identity at `temp == tnom`. The soft breakdown branch keeps its
    /// nominal knee (the exact ngspice IBV/BV matching is not modeled).
    ///
    /// Call once after model parameters and junction scaling are applied.
    pub fn set_temperature(&mut self, temp_kelvin: Value, default_tnom_kelvin: Value) {
        let tnom = self
            .tnom_c
            .map(|c| c + 273.15)
            .unwrap_or(default_tnom_kelvin);
        let temp = temp_kelvin;
        if !(temp > 0.0) || !(tnom > 0.0) {
            return;
        }

        let vt = KOVERQ * temp;
        let vtnom = KOVERQ * tnom;

        // Silicon bandgap at both temperatures (TLEV 0/1 form).
        let egfet = 1.16 - (7.02e-4 * temp * temp) / (temp + 1108.0);
        let egfet1 = 1.16 - (7.02e-4 * tnom * tnom) / (tnom + 1108.0);
        let fact1 = tnom / REFTEMP;
        let fact2 = temp / REFTEMP;
        // ngspice's CHARGE*arg terms, folded through k/q so everything
        // stays in volts.
        let arg = -egfet / (2.0 * vt) + 1.1150877 / (2.0 * KOVERQ * REFTEMP);
        let arg1 = -egfet1 / (2.0 * vtnom) + 1.1150877 / (2.0 * KOVERQ * REFTEMP);
        let pbfact = -2.0 * vt * (1.5 * fact2.ln() + arg);
        let pbfact1 = -2.0 * vtnom * (1.5 * fact1.ln() + arg1);

        // IS(T) = IS * exp(((T/TNOM)-1)*EG/(N*vt) + (XTI/N)*ln(T/TNOM))
        let vte = self.n * vt;
        let is_factor =
            (((temp / tnom) - 1.0) * self.eg / vte + (self.xti / self.n) * (temp / tnom).ln())
                .exp();
        if is_factor.is_finite() && is_factor > 0.0 {
            self.is *= is_factor;
        }

        // VJ(T) and CJ0(T), TLEVC=0 bandgap mapping.
        if self.vj > 0.0 {
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

        self.vt = vt;
    }

    /// Cached operating-point values from the last accepted Newton solution:
    /// `(vd, id, gd)` — junction voltage, current, and conductance.
    pub fn op_values(&self) -> (Value, Value, Value) {
        let (id, gd) = self.current_and_conductance(self.prev_vd);
        (self.prev_vd, id, gd)
    }

    /// Apply a parallel-junction scale factor (instance `AREA` × `M`).
    ///
    /// Saturation and breakdown-knee currents and the zero-bias depletion
    /// capacitance scale with the number of parallel junctions; series
    /// resistance scales inversely. Mirrors ngspice's AREA/M handling for
    /// the lumped diode.
    pub fn apply_junction_scaling(&mut self, scale: Value) {
        self.is *= scale;
        self.ibv *= scale;
        self.cj0 *= scale;
        if self.rs > 0.0 {
            self.rs /= scale;
        }
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
        if self.cj0 > 0.0 && self.vj > 0.0 && self.m < 1.0 {
            let fc = self.fc.clamp(0.0, 0.95);
            let dep_cap_knee = fc * self.vj;
            if vd < dep_cap_knee {
                let arg = 1.0 - vd / self.vj;
                let sarg = (-self.m * arg.ln()).exp();
                qd += self.vj * self.cj0 * (1.0 - arg * sarg) / (1.0 - self.m);
                capd += self.cj0 * sarg;
            } else {
                let f1 = self.vj * (1.0 - (1.0 - fc).powf(1.0 - self.m)) / (1.0 - self.m);
                let f2 = (1.0 - fc).powf(1.0 + self.m);
                let f3 = 1.0 - fc * (1.0 + self.m);
                let czof2 = self.cj0 / f2;
                qd += self.cj0 * f1
                    + czof2
                        * (f3 * (vd - dep_cap_knee)
                            + (self.m / (2.0 * self.vj))
                                * (vd * vd - dep_cap_knee * dep_cap_knee));
                capd += czof2 * (f3 + self.m * vd / self.vj);
            }
        }
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
        // Clamp voltage to avoid singularity at forward bias > VJ
        let v_clamped = vd.min(0.9 * self.vj);

        // Junction (depletion) capacitance
        let cj = if v_clamped < 0.0 {
            // Reverse bias: standard formula
            self.cj0 / (1.0 - v_clamped / self.vj).powf(self.m)
        } else {
            // Forward bias: linear approximation above VJ/2
            let fc = 0.5; // Where to switch to linear
            if v_clamped < fc * self.vj {
                self.cj0 / (1.0 - v_clamped / self.vj).powf(self.m)
            } else {
                // Linear extrapolation
                let cj_fc = self.cj0 / (1.0 - fc).powf(self.m);
                let dcj = cj_fc * self.m / (self.vj * (1.0 - fc));
                cj_fc + dcj * (v_clamped - fc * self.vj)
            }
        };

        // Diffusion capacitance: Cd = TT * gd (where gd is conductance)
        let gd = self.diode_conductance(vd);
        let cd = self.tt * gd;

        cj + cd
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
        let forward = self.forward_current(vd);
        let breakdown = self.breakdown_current(vd);
        forward + breakdown
    }

    /// Diode conductance (derivative of current): gd = Is / (N * Vt) * exp(Vd / (N * Vt))
    fn diode_conductance(&self, vd: Value) -> Value {
        let forward = self.forward_conductance(vd);
        let breakdown = self.breakdown_conductance(vd);
        forward + breakdown
    }

    /// Junction current and conductance in one evaluation.
    ///
    /// `current` + `diode_conductance` share the same forward and breakdown
    /// exponentials; the stamp path calls this fused form to evaluate each
    /// exp() once. The expression shapes mirror the individual methods
    /// exactly so results stay bit-identical.
    fn current_and_conductance(&self, vd: Value) -> (Value, Value) {
        let n_vt = self.n * self.vt;
        let vd_limited = vd.min(80.0 * self.n * self.vt);
        let e = (vd_limited / n_vt).exp();
        let forward_i = self.is * (e - 1.0);
        let forward_g = (self.is / n_vt) * e;
        let (breakdown_i, breakdown_g) = match self.bv {
            None => (0.0, 0.0),
            Some(bv) => {
                let scale = self.breakdown_softness(bv);
                let exponent = ((-vd - bv) / scale).clamp(-80.0, 40.0);
                let be = exponent.exp();
                (-self.ibv * be, (self.ibv / scale) * be)
            }
        };
        (forward_i + breakdown_i, forward_g + breakdown_g)
    }

    fn forward_current(&self, vd: Value) -> Value {
        // Limit voltage to prevent overflow
        let vd_limited = vd.min(80.0 * self.n * self.vt);
        self.is * ((vd_limited / (self.n * self.vt)).exp() - 1.0)
    }

    fn forward_conductance(&self, vd: Value) -> Value {
        let vd_limited = vd.min(80.0 * self.n * self.vt);
        (self.is / (self.n * self.vt)) * (vd_limited / (self.n * self.vt)).exp()
    }

    fn breakdown_softness(&self, bv: Value) -> Value {
        let thermal_knee = (self.n * self.vt).abs().max(0.05);
        let scaled_knee = (0.02 * bv.abs()).clamp(0.05, 1.0);
        thermal_knee.max(scaled_knee)
    }

    fn breakdown_current(&self, vd: Value) -> Value {
        let Some(bv) = self.bv else {
            return 0.0;
        };
        let scale = self.breakdown_softness(bv);
        let exponent = ((-vd - bv) / scale).clamp(-80.0, 40.0);
        -self.ibv * exponent.exp()
    }

    fn breakdown_conductance(&self, vd: Value) -> Value {
        let Some(bv) = self.bv else {
            return 0.0;
        };
        let scale = self.breakdown_softness(bv);
        let exponent = ((-vd - bv) / scale).clamp(-80.0, 40.0);
        (self.ibv / scale) * exponent.exp()
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
        !self.limited.get() && (self.prev_vd - self.prev_vd_old).abs() < tolerance
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
    fn saturation_current_follows_ngspice_diotemp_law() {
        // IS(T) = IS * exp(((T/TNOM)-1)*EG/(N*vt) + (XTI/N)*ln(T/TNOM))
        let mut d = test_diode();
        let temp = 273.15 + 85.0;
        let tnom = REFTEMP;
        d.set_temperature(temp, tnom);

        let vt = KOVERQ * temp;
        let expected = 1e-14
            * (((temp / tnom) - 1.0) * 1.11 / vt + 3.0 * (temp / tnom).ln()).exp();
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

//! Diode device model

use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{NodeId, Value};

/// Boltzmann over charge with ngspice-46's CODATA constants
/// (const.h: CONSTboltz / CHARGE).
const KOVERQ: Value = 1.38064852e-23 / 1.6021766208e-19;
/// Legacy physical constants retained by Xyce 7.x's SPICE diode model.
const XYCE_7_KOVERQ: Value = 1.3806226e-23 / 1.6021918e-19;
/// SPICE reference temperature, 27C in Kelvin (ngspice REFTEMP).
const REFTEMP: Value = 300.15;
const EPSMIN: Value = 1.0e-28;
const MAX_EXP_ARG: Value = 100.0;
// Xyce's diode model uses the same 100.0 exponent ceiling for reverse
// breakdown as for the forward junction.  A lower ceiling changes the
// breakdown I/V curve itself (the linear continuation becomes the model),
// which is observable in cold, low-Is Level=2 sweeps.
const BREAKDOWN_EXP_ARG_MAX: Value = 100.0;

/// Oxide permittivity used by the LEVEL=3 metal/poly overlap capacitances
/// (ngspice const.h `CONSTepsSiO2`).
const EPS_SIO2: Value = 3.4531479969e-11;

/// Which diode formulation a `.model` card selected.
///
/// ngspice implements LEVEL 1 and 3 in the same `dio` device and rejects
/// LEVEL 2; Xyce implements 1 and 2 and has no LEVEL 3. RSpice carries all
/// three because its conformance corpora include decks from both.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiodeLevel {
    /// Legacy SPICE junction diode (no LEVEL, LEVEL=0, or LEVEL=1).
    Legacy,
    /// Xyce/PSpice LEVEL=2, which differs in its junction-temperature law.
    Pspice,
    /// ngspice LEVEL=3: the geometric foundry diode.
    ///
    /// Same evaluator as `Legacy` — the parameters that foundry cards lean on
    /// (JSW, CJP, JTUN, TLEV/TLEVC) are level-independent in ngspice. Only
    /// three behaviours actually branch on the level: W/L derives AREA and PJ,
    /// the breakdown knee current scales with area rather than multiplicity
    /// alone, and metal/poly overlap capacitances exist at all.
    Geometric,
}

impl DiodeLevel {
    /// Classify a numeric `LEVEL=` selector. Unknown levels are rejected
    /// earlier by `validate_diode_model_level`.
    fn from_selector(level: Value) -> Self {
        if !level.is_finite() {
            return Self::Legacy;
        }
        if (level - 2.0).abs() <= 1.0e-9 {
            Self::Pspice
        } else if (level - 3.0).abs() <= 1.0e-9 {
            Self::Geometric
        } else {
            Self::Legacy
        }
    }
}

/// HSPICE-style temperature-equation coefficients (ngspice `dio` TLEV/TLEVC).
///
/// `tlev` selects the saturation-current and breakdown-voltage laws; `tlevc`
/// selects the junction-potential and depletion-capacitance laws. The two are
/// independent, and foundry cards routinely set `TLEVC=1` to replace the
/// bandgap-derived capacitance shift with measured linear coefficients.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DiodeTemperatureModel {
    /// Saturation-current / breakdown temperature-equation selector.
    pub tlev: u8,
    /// Junction-capacitance / potential temperature-equation selector.
    pub tlevc: u8,
    /// First bandgap correction factor (TLEV=2 only).
    pub gap1: Value,
    /// Second bandgap correction factor (TLEV=2 only).
    pub gap2: Value,
    /// Breakdown-voltage temperature coefficient (ngspice TCV).
    pub tcv: Value,
    /// Whether TCV was authored, selecting the ngspice breakdown law.
    pub tcv_given: bool,
    /// Bottom junction-potential temperature coefficient (TLEVC=1).
    pub tpb: Value,
    /// Sidewall junction-potential temperature coefficient (TLEVC=1).
    pub tphp: Value,
    /// Bottom junction-capacitance temperature coefficient (TLEVC=1).
    pub cta: Value,
    /// Sidewall junction-capacitance temperature coefficient (TLEVC=1).
    pub ctp: Value,
    /// Series-resistance linear temperature coefficient (TRS/TRS1).
    pub trs1: Value,
    /// Series-resistance quadratic temperature coefficient (TRS2).
    pub trs2: Value,
    /// Grading-coefficient linear temperature coefficient (TM1).
    pub tm1: Value,
    /// Grading-coefficient quadratic temperature coefficient (TM2).
    pub tm2: Value,
    /// Transit-time linear temperature coefficient (TTT1).
    pub ttt1: Value,
    /// Transit-time quadratic temperature coefficient (TTT2).
    pub ttt2: Value,
}

impl Default for DiodeTemperatureModel {
    fn default() -> Self {
        Self {
            tlev: 0,
            tlevc: 0,
            gap1: 7.02e-4,
            gap2: 1108.0,
            tcv: 0.0,
            tcv_given: false,
            tpb: 0.0,
            tphp: 0.0,
            cta: 0.0,
            ctp: 0.0,
            trs1: 0.0,
            trs2: 0.0,
            tm1: 0.0,
            tm2: 0.0,
            ttt1: 0.0,
            ttt2: 0.0,
        }
    }
}

impl DiodeTemperatureModel {
    /// The `1 + c1·dt + c2·dt²` polynomial ngspice applies to RS, MJ and TT.
    fn quadratic_factor(first: Value, second: Value, delta_t: Value) -> Value {
        let factor = 1.0 + first * delta_t + second * delta_t * delta_t;
        if factor.is_finite() && factor > 0.0 {
            factor
        } else {
            1.0
        }
    }
}

/// Band-to-band tunneling branch (ngspice `dio` JTUN/JTUNSW family).
///
/// This is the reverse-bias mechanism foundry junction diodes are actually
/// characterised on. For the GF180MCU PDK it outranks avalanche breakdown by
/// an order of magnitude — at −12.13 V an `np_3p3` sources 1.61 A of tunneling
/// current against 0.19 A of breakdown current — so a diode that models
/// breakdown but not tunneling is not merely imprecise in reverse, it is
/// wrong by orders of magnitude.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DiodeTunneling {
    /// Bottom tunneling saturation current, scaled by AREA (JTUN).
    pub bottom: Value,
    /// Sidewall tunneling saturation current, scaled by PJ (JTUNSW).
    pub sidewall: Value,
    /// Whether JTUN was authored; ngspice gates the branch on this.
    pub bottom_given: bool,
    /// Whether JTUNSW was authored.
    pub sidewall_given: bool,
    /// Tunneling emission coefficient (NTUN).
    pub emission: Value,
    /// Tunneling saturation-current temperature exponent (XTITUN).
    pub exponent: Value,
    /// Bandgap correction applied to the tunneling temperature law (KEG).
    pub bandgap_factor: Value,
}

impl Default for DiodeTunneling {
    fn default() -> Self {
        Self {
            bottom: 0.0,
            sidewall: 0.0,
            bottom_given: false,
            sidewall_given: false,
            emission: 30.0,
            exponent: 3.0,
            bandgap_factor: 1.0,
        }
    }
}

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
    /// Sidewall high-injection knee current (IKP).
    pub sidewall_knee_current: Value,
    /// Band-to-band tunneling branch (JTUN/JTUNSW/NTUN/XTITUN/KEG).
    pub(crate) tunneling: DiodeTunneling,
    /// Bias-independent metal and poly overlap capacitance (LEVEL=3
    /// CMETAL + CPOLY), already scaled by the instance multiplicity.
    pub overlap_capacitance: Value,

    // Temperature parameters
    /// Saturation-current temperature exponent (XTI)
    pub xti: Value,
    /// Activation energy in eV (EG)
    pub eg: Value,
    /// Model nominal temperature override in Celsius (TNOM/TREF/T_MEASURED)
    pub tnom_c: Option<Value>,
    /// Which formulation the `.model` card's LEVEL selector chose.
    pub level: DiodeLevel,
    /// HSPICE-style TLEV/TLEVC temperature-equation coefficients.
    pub(crate) temperature_model: DiodeTemperatureModel,
    /// Evaluate as Xyce's native diode rather than ngspice's.
    ///
    /// The two references genuinely disagree about this device, so the flag
    /// selects between them rather than tuning a tolerance:
    ///
    /// - **Iteration limiter.** Xyce keeps the original
    ///   `DeviceSupport::pnjlim`; ngspice uses the later negative-voltage-safe
    ///   `DEVpnjlim`.
    /// - **Sidewall without NS.** Xyce merges `JSW·PJ` into the bottom
    ///   saturation current (`Isat = Isat + IsatSW`), so it shares the
    ///   high-injection knee and the breakdown region. ngspice keeps it a
    ///   separate current that skips both.
    /// - **Breakdown matching.** Xyce solves the knee against the bottom
    ///   saturation current alone; ngspice solves it against `totalSatCur`.
    xyce_dialect: bool,
    /// Evaluate the diode's implementation-defined extensions exactly as
    /// ngspice rather than using the best-available reference equations.
    ///
    /// In particular, ngspice clamps the `ISR` contribution at `-3*N*Vt` in
    /// reverse bias, while Cadence's PSpice reference equation evaluates it at
    /// the actual junction voltage.
    ngspice_dialect: bool,
    /// Use Xyce's native transient device status semantics for this circuit.
    ///
    /// Xyce's classic diode reports only its limiter/origFlag status to the
    /// transient `ENFORCEDEVICECONV` test.  The transient engine enables this
    /// for an implicit Xyce run after the DC operating point is established;
    /// explicit strict overrides and all non-transient analyses leave it off.
    native_xyce_transient_convergence: bool,
    /// Linear temperature coefficient for the reverse breakdown voltage
    /// (Xyce TBV1, in 1/C).
    pub tbv1: Value,
    /// Quadratic temperature coefficient for the reverse breakdown voltage
    /// (Xyce TBV2, in 1/C^2).
    pub tbv2: Value,

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
    /// Conductance paired with `prev_id` at exactly `prev_vd`.
    prev_gd: Value,
    /// Whether the paired candidate evaluation has been populated by
    /// `NonlinearDevice::update`.  A separate validity bit prevents the
    /// all-zero construction state from masquerading as an evaluated diode.
    candidate_eval_valid: bool,
    /// Engine-supplied junction shunt conductance (ngspice `CKTgmin`):
    /// zero in plain solves, raised by gmin-stepping/rescue ladders so the
    /// continuation can deform the diode system like every other junction.
    junction_gmin: Value,
    /// The deck marked this instance `OFF`.
    initial_off: bool,
    /// Instance `IC=` junction voltage.  The diode is the one family whose
    /// `IC` is a scalar in both references: `dio/dio.c:16` declares
    /// `IOPAU("ic", DIO_IC, IF_REAL, "Initial device voltage")` and
    /// `N_DEV_Diode.C:79` a plain `addPar("IC", 0.0, &Instance::InitCond)`.
    initial_condition: Option<Value>,
    /// No stamp has established a pnjlim history yet, so the next
    /// linearization is the operating point's MODEINITJCT evaluation.
    junction_history_valid: std::cell::Cell<bool>,
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

/// Mutable Newton and junction-limiter state for a diode rollback point.
/// Static model, topology, temperature, and linked-index fields remain on the
/// live device and are not recopied for every transient timestep.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DiodeNonlinearState {
    pub(crate) prev_vd: Value,
    pub(crate) prev_vd_old: Value,
    pub(crate) prev_id: Value,
    pub(crate) prev_gd: Value,
    pub(crate) candidate_eval_valid: bool,
    pub(crate) junction_gmin: Value,
    pub(crate) junction_history_valid: bool,
    pub(crate) last_limited_vd: Value,
    pub(crate) limited: bool,
    pub(crate) last_stamp_vd: Value,
    pub(crate) last_stamp_id: Value,
    pub(crate) last_stamp_gd: Value,
}

pub(crate) const DIODE_ACCEPTED_NONLINEAR_RUNTIME_TAG: &str = "native-diode-v1";

/// Accepted native-diode Newton/limiter state carried across a transient seam.
///
/// The instance name and runtime tag make ordinal checkpoint storage fail
/// closed if a future elaboration reorders devices or changes the runtime that
/// owns the state.  The numeric payload is exactly the same compact state used
/// by rejected-trial rollback; model parameters, topology and linked matrix
/// indices remain owned by the freshly elaborated live device.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct AcceptedDiodeNonlinearCheckpoint {
    pub(crate) instance_name: String,
    pub(crate) runtime_tag: String,
    pub(crate) state: DiodeNonlinearState,
}

impl Diode {
    pub(crate) fn nonlinear_state_snapshot(&self) -> DiodeNonlinearState {
        DiodeNonlinearState {
            prev_vd: self.prev_vd,
            prev_vd_old: self.prev_vd_old,
            prev_id: self.prev_id,
            prev_gd: self.prev_gd,
            candidate_eval_valid: self.candidate_eval_valid,
            junction_gmin: self.junction_gmin,
            junction_history_valid: self.junction_history_valid.get(),
            last_limited_vd: self.last_limited_vd.get(),
            limited: self.limited.get(),
            last_stamp_vd: self.last_stamp_vd.get(),
            last_stamp_id: self.last_stamp_id.get(),
            last_stamp_gd: self.last_stamp_gd.get(),
        }
    }

    pub(crate) fn restore_nonlinear_state(&mut self, state: DiodeNonlinearState) {
        self.prev_vd = state.prev_vd;
        self.prev_vd_old = state.prev_vd_old;
        self.prev_id = state.prev_id;
        self.prev_gd = state.prev_gd;
        self.candidate_eval_valid = state.candidate_eval_valid;
        self.junction_gmin = state.junction_gmin;
        self.junction_history_valid
            .set(state.junction_history_valid);
        self.last_limited_vd.set(state.last_limited_vd);
        self.limited.set(state.limited);
        self.last_stamp_vd.set(state.last_stamp_vd);
        self.last_stamp_id.set(state.last_stamp_id);
        self.last_stamp_gd.set(state.last_stamp_gd);
    }

    pub(crate) fn accepted_nonlinear_checkpoint(
        &self,
    ) -> Result<AcceptedDiodeNonlinearCheckpoint, String> {
        let checkpoint = AcceptedDiodeNonlinearCheckpoint {
            instance_name: self.name.clone(),
            runtime_tag: DIODE_ACCEPTED_NONLINEAR_RUNTIME_TAG.to_string(),
            state: self.nonlinear_state_snapshot(),
        };
        self.validate_accepted_nonlinear_checkpoint(&checkpoint)?;
        Ok(checkpoint)
    }

    pub(crate) fn validate_accepted_nonlinear_checkpoint(
        &self,
        checkpoint: &AcceptedDiodeNonlinearCheckpoint,
    ) -> Result<(), String> {
        if checkpoint.instance_name != self.name {
            return Err(format!(
                "diode instance name mismatch: captured '{}', circuit has '{}'",
                checkpoint.instance_name, self.name
            ));
        }
        if checkpoint.runtime_tag != DIODE_ACCEPTED_NONLINEAR_RUNTIME_TAG {
            return Err(format!(
                "diode '{}' runtime mismatch: captured '{}', runtime requires '{}'",
                self.name, checkpoint.runtime_tag, DIODE_ACCEPTED_NONLINEAR_RUNTIME_TAG
            ));
        }
        let state = checkpoint.state;
        let numeric = [
            state.prev_vd,
            state.prev_vd_old,
            state.prev_id,
            state.prev_gd,
            state.junction_gmin,
            state.last_limited_vd,
            state.last_stamp_vd,
            state.last_stamp_id,
            state.last_stamp_gd,
        ];
        if numeric.iter().any(|value| !value.is_finite()) {
            return Err(format!(
                "diode '{}' accepted nonlinear checkpoint contains a non-finite value",
                self.name
            ));
        }
        if state.junction_gmin < 0.0 {
            return Err(format!(
                "diode '{}' accepted nonlinear checkpoint has negative junction gmin",
                self.name
            ));
        }
        Ok(())
    }

    pub(crate) fn restore_accepted_nonlinear_checkpoint(
        &mut self,
        checkpoint: &AcceptedDiodeNonlinearCheckpoint,
    ) -> Result<(), String> {
        self.validate_accepted_nonlinear_checkpoint(checkpoint)?;
        self.restore_nonlinear_state(checkpoint.state);
        Ok(())
    }

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
            sidewall_knee_current: 0.0,
            tunneling: DiodeTunneling::default(),
            overlap_capacitance: 0.0,

            // SPICE temperature defaults
            xti: 3.0,
            eg: 1.11,
            tnom_c: None,
            level: DiodeLevel::Legacy,
            temperature_model: DiodeTemperatureModel::default(),
            xyce_dialect: false,
            ngspice_dialect: false,
            native_xyce_transient_convergence: false,
            tbv1: 0.0,
            tbv2: 0.0,

            // Flicker noise off by default (diosetup.c: fNcoef 0, fNexp 1).
            kf: 0.0,
            af: 1.0,
            multiplicity: 1.0,

            prev_vd: 0.0,
            prev_vd_old: 0.0,
            prev_id: 0.0,
            prev_gd: 0.0,
            candidate_eval_valid: false,
            junction_gmin: 0.0,
            initial_off: false,
            initial_condition: None,
            junction_history_valid: std::cell::Cell::new(false),
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

    /// The deck marked this instance `OFF`, so its first stamped
    /// linearization is dioload.c's zero-bias MODEINITJCT state.
    pub fn set_initially_off(&mut self, off: bool) {
        self.initial_off = off;
    }

    /// True when the deck marked this instance `OFF`.
    pub fn is_initially_off(&self) -> bool {
        self.initial_off
    }

    /// The instance `IC=` junction voltage the deck authored.
    pub fn set_transient_initial_condition(&mut self, ic: Option<Value>) {
        self.initial_condition = ic.filter(|value| value.is_finite());
    }

    /// The instance `IC=` junction voltage, read only by the `UIC` transient
    /// startup: `dioload.c:153-157` requires
    /// `MODEINITJCT && MODETRANOP && MODEUIC`.
    pub(crate) fn transient_initial_condition(&self) -> Option<Value> {
        self.initial_condition
    }

    /// Engine hook: junction gmin for continuation ladders (mirrors the
    /// MOSFET/JFET/BJT `set_junction_gmin` convention).
    pub fn set_junction_gmin(&mut self, gmin: Value) {
        self.junction_gmin = if gmin.is_finite() { gmin.max(0.0) } else { 0.0 };
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
            * (vte / (std::f64::consts::SQRT_2 * self.vcrit_saturation_current().max(1e-300))).ln();

        // dioload.c's MODEINITJCT arms assign the junction voltage outright
        // rather than handing a reference to pnjlim: an instance the deck
        // marked `OFF` opens at exactly `vd = 0` (dioload.c:158-161) and an
        // unmarked one opens forward-biased at `vd = tVcrit`
        // (dioload.c:162-166). Neither arm is gated on a compatibility mode,
        // and Xyce takes the same two branches on its first Newton iterate of
        // an operating point — `N_DEV_Diode.C:1156-1159` for `off`,
        // `N_DEV_Diode.C:1176-1177` for the rest, both writing `Vd_old = Vd`
        // (line 1184) so the pnjlim that follows is a no-op on the value.
        // So this is not a dialect choice; it is what a SPICE junction's
        // first load is.
        //
        // Assigning is not the same thing as limiting toward the same value.
        // `tVcrit` is by construction the bias where `gd = 1/√2 S`, so the
        // first Jacobian carries a conducting junction and an equivalent
        // current source of order half an amp, and Newton descends onto the
        // forward root from above. Limiting a zero-referenced raw bias
        // instead starts the junction at cutoff, where `gd` is the saturation
        // current over `vte` — a dozen orders of magnitude smaller — and the
        // first step is decided by whatever else is in the row.
        //
        // A non-finite `tVcrit` is the one case the assignment cannot take:
        // an unlimited pnjlim reference degrades to no limiting, but an
        // unlimited *bias* is a NaN in the matrix. Such an instance keeps the
        // ordinary path.
        if !self.junction_history_valid.get() && (self.initial_off || vcrit.is_finite()) {
            self.junction_history_valid.set(true);
            self.limited.set(false);
            let vd = if self.initial_off { 0.0 } else { vcrit };
            self.last_limited_vd.set(vd);
            let (id, gd) = self.candidate_current_and_conductance(vd);
            let stamped_id = id + self.junction_gmin * vd;
            let stamped_gd = gd + self.junction_gmin;
            self.last_stamp_vd.set(vd);
            self.last_stamp_id.set(stamped_id);
            self.last_stamp_gd.set(stamped_gd);
            return (vd, stamped_id, stamped_gd);
        }

        let limit_junction =
            |candidate: Value, previous: Value, thermal: Value, critical: Value| {
                if self.xyce_dialect {
                    Self::limit_xyce_pnjlim(candidate, previous, thermal, critical)
                } else {
                    super::limiting::pnjlim_new(candidate, previous, thermal, critical)
                }
            };

        // Pivoted on the *matched* breakdown voltage, which is what both
        // ngspice (`here->DIOtBrkdwnV`) and the evaluator above use. Limiting
        // around the raw card value instead would fold the junction about a
        // different point than the branch it is protecting.
        let (vd, limited) = if let Some(bv) = self.active_breakdown_voltage() {
            let vtebrk = self.breakdown_emission_coefficient.max(EPSMIN) * self.vt;
            if vd_raw < 0.0_f64.min(-bv + 10.0 * vtebrk) {
                let transformed = -(vd_raw + bv);
                let old_transformed = -(self.last_limited_vd.get() + bv);
                let (limited_transformed, limited) =
                    limit_junction(transformed, old_transformed, vtebrk, vcrit);
                (-(limited_transformed + bv), limited)
            } else {
                limit_junction(vd_raw, self.last_limited_vd.get(), vte, vcrit)
            }
        } else {
            limit_junction(vd_raw, self.last_limited_vd.get(), vte, vcrit)
        };
        self.limited.set(limited);
        self.last_limited_vd.set(vd);
        self.junction_history_valid.set(true);

        let (id, gd) = self.candidate_current_and_conductance(vd);
        let stamped_id = id + self.junction_gmin * vd;
        let stamped_gd = gd + self.junction_gmin;
        self.last_stamp_vd.set(vd);
        self.last_stamp_id.set(stamped_id);
        self.last_stamp_gd.set(stamped_gd);
        (vd, stamped_id, stamped_gd)
    }

    /// Xyce 7.10's historical `DeviceSupport::pnjlim` implementation.
    ///
    /// The diode model intentionally differs from `pnjlim_new`: the classic
    /// Xyce routine has no negative-voltage branch and uses
    /// `1 + (vnew-vold)/vt` in its forward-bias logarithm.  Keeping this
    /// equation local to the native diode avoids changing the Verilog-A
    /// generated limiter used by generated devices.
    #[inline]
    fn limit_xyce_pnjlim(
        candidate: Value,
        previous: Value,
        thermal_voltage: Value,
        critical_voltage: Value,
    ) -> (Value, bool) {
        let thermal_voltage = thermal_voltage.max(EPSMIN);
        if !candidate.is_finite() {
            return (previous, false);
        }
        if !previous.is_finite() {
            return (candidate, false);
        }
        if candidate <= critical_voltage
            || (candidate - previous).abs() <= thermal_voltage + thermal_voltage
        {
            return (candidate, false);
        }

        let limited = if previous > 0.0 {
            let arg = 1.0 + (candidate - previous) / thermal_voltage;
            if arg > 0.0 {
                previous + thermal_voltage * arg.ln()
            } else {
                critical_voltage
            }
        } else {
            thermal_voltage * (candidate / thermal_voltage).ln()
        };
        (limited, true)
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

    /// Return whether Xyce 7.10's legacy `D` model registry accepts a model
    /// parameter name.
    ///
    /// RSpice's native diode also supports ngspice-compatible and geometry
    /// fields, while Xyce's parser diagnoses those names when they are authored
    /// on a legacy diode model card. The entries below are the exact
    /// `Traits::loadModelParameters` names from Xyce 7.10 `N_DEV_Diode.C`, plus
    /// the framework-owned `LEVEL` selector.
    pub(crate) fn supports_xyce_legacy_model_parameter(name: &str) -> bool {
        matches!(
            name.to_ascii_uppercase().as_str(),
            "LEVEL"
                | "IS"
                | "JS"
                | "JSW"
                | "RS"
                | "N"
                | "NS"
                | "ISR"
                | "NR"
                | "IKF"
                | "TT"
                | "CJO"
                | "CJ"
                | "CJ0"
                | "VJ"
                | "M"
                | "CJSW"
                | "CJP"
                | "PHP"
                | "VJSW"
                | "MJSW"
                | "EG"
                | "XTI"
                | "TIKF"
                | "TBV1"
                | "TBV2"
                | "TRS1"
                | "TRS"
                | "TRS2"
                | "FC"
                | "FCS"
                | "BV"
                | "VB"
                | "IBV"
                | "NBV"
                | "IBVL"
                | "NBVL"
                | "TNOM"
                | "KF"
                | "AF"
        )
    }

    /// Set model parameters from a HashMap (for .MODEL statement parsing)
    pub fn with_model_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        self.level = params
            .get("LEVEL")
            .copied()
            .map_or(DiodeLevel::Legacy, DiodeLevel::from_selector);
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
        if let Some(&v) = params
            .get("BV")
            .or_else(|| params.get("VB"))
            .or_else(|| params.get("VRB"))
            .or_else(|| params.get("VAR"))
            && v.is_finite()
            && v >= 0.0
        {
            // Presence, not positivity. Both ngspice (`DIObreakdownVoltageGiven`)
            // and Xyce (`BVGiven`) gate the breakdown branch on whether the card
            // named BV at all, and `BV=0` is a real, distinct setting rather
            // than a way to disable breakdown: the matching loop then solves for
            // a *negative* effective breakdown voltage, which puts every reverse
            // bias inside the breakdown exponential. GF180MCU's `nwp_3p3` relies
            // on exactly this — at −12.13 V it draws 5.65 A with `bv=0` against
            // 81 µA with BV omitted.
            self.bv = Some(v);
        }
        if let Some(&v) = params.get("IBV").or_else(|| params.get("IB"))
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
        if let Some(&v) = params.get("VJ").or_else(|| params.get("PB")) {
            self.vj = v;
        }
        if let Some(&v) = params.get("M").or_else(|| params.get("MJ")) {
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
        if let Some(&v) = params.get("JSW").or_else(|| params.get("ISW"))
            && v.is_finite()
            && v >= 0.0
        {
            self.sidewall_saturation_current = v;
            self.sidewall_current_given = true;
        }
        if let Some(&v) = params.get("IKP") {
            self.sidewall_knee_current = if v.is_finite() && v >= EPSMIN { v } else { 0.0 };
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
        if let Some(&v) = params.get("NBV").or_else(|| params.get("NZ"))
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
        if let Some(&v) = params
            .get("TNOM")
            .or_else(|| params.get("TREF"))
            .or_else(|| params.get("T_MEASURED"))
        {
            self.tnom_c = Some(v);
        }

        // Band-to-band tunneling (JTUN/JTUNSW/NTUN/XTITUN/KEG). ngspice gates
        // each branch on the density having been authored rather than on it
        // being non-zero, so an explicit `jtun=0` still selects the branch and
        // contributes nothing — which is the same answer, but keeps the
        // "given" flags meaning what the card said.
        if let Some(&v) = params.get("JTUN")
            && v.is_finite()
        {
            self.tunneling.bottom = v;
            self.tunneling.bottom_given = true;
        }
        if let Some(&v) = params.get("JTUNSW")
            && v.is_finite()
        {
            self.tunneling.sidewall = v;
            self.tunneling.sidewall_given = true;
        }
        if let Some(&v) = params.get("NTUN")
            && v.is_finite()
            && v > 0.0
        {
            self.tunneling.emission = v;
        }
        if let Some(&v) = params.get("XTITUN")
            && v.is_finite()
        {
            self.tunneling.exponent = v;
        }
        if let Some(&v) = params.get("KEG")
            && v.is_finite()
        {
            self.tunneling.bandgap_factor = v;
        }

        // TLEV/TLEVC temperature-equation family.
        if let Some(&v) = params.get("TLEV")
            && v.is_finite()
            && v >= 0.0
        {
            self.temperature_model.tlev = v.round() as u8;
        }
        if let Some(&v) = params.get("TLEVC")
            && v.is_finite()
            && v >= 0.0
        {
            self.temperature_model.tlevc = v.round() as u8;
        }
        if let Some(&v) = params.get("GAP1")
            && v.is_finite()
        {
            self.temperature_model.gap1 = v;
        }
        if let Some(&v) = params.get("GAP2")
            && v.is_finite()
        {
            self.temperature_model.gap2 = v;
        }
        // TCV selects ngspice's breakdown law; TBV1/TBV2 below select Xyce's.
        // They are deliberately separate rather than aliased: ngspice's `tbv1`
        // is a synonym for TCV and enters as `BV·(1 − TCV·dt)`, while Xyce's
        // TBV1 enters as `BV·(1 + TBV1·dt)`. Folding them together would flip
        // the sign of every card that names the other dialect's parameter.
        if let Some(&v) = params.get("TCV")
            && v.is_finite()
        {
            self.temperature_model.tcv = v;
            self.temperature_model.tcv_given = true;
        }
        if let Some(&v) = params.get("TPB").or_else(|| params.get("TVJ"))
            && v.is_finite()
        {
            self.temperature_model.tpb = v;
        }
        if let Some(&v) = params.get("TPHP")
            && v.is_finite()
        {
            self.temperature_model.tphp = v;
        }
        if let Some(&v) = params.get("CTA").or_else(|| params.get("CTC"))
            && v.is_finite()
        {
            self.temperature_model.cta = v;
        }
        if let Some(&v) = params.get("CTP")
            && v.is_finite()
        {
            self.temperature_model.ctp = v;
        }
        if let Some(&v) = params.get("TRS").or_else(|| params.get("TRS1"))
            && v.is_finite()
        {
            self.temperature_model.trs1 = v;
        }
        if let Some(&v) = params.get("TRS2")
            && v.is_finite()
        {
            self.temperature_model.trs2 = v;
        }
        if let Some(&v) = params.get("TM1")
            && v.is_finite()
        {
            self.temperature_model.tm1 = v;
        }
        if let Some(&v) = params.get("TM2")
            && v.is_finite()
        {
            self.temperature_model.tm2 = v;
        }
        if let Some(&v) = params.get("TTT1")
            && v.is_finite()
        {
            self.temperature_model.ttt1 = v;
        }
        if let Some(&v) = params.get("TTT2")
            && v.is_finite()
        {
            self.temperature_model.ttt2 = v;
        }

        if let Some(&v) = params.get("TBV1")
            && v.is_finite()
        {
            self.tbv1 = v;
        }
        if let Some(&v) = params.get("TBV2")
            && v.is_finite()
        {
            self.tbv2 = v;
        }
        if !self.breakdown_emission_given {
            self.breakdown_emission_coefficient = self.n;
        }
        // ngspice defaults the activation energy against TLEV, not against the
        // level: the TLEV=2 bandgap law is written around 1.16 eV while the
        // TLEV 0/1 laws are written around 1.11 eV.
        if !params.contains_key("EG") && self.temperature_model.tlev == 2 {
            self.eg = 1.16;
        }
        self
    }

    /// Apply Xyce device-level minimum defaults to model parameters that were
    /// omitted from a legacy diode card.
    ///
    /// Xyce marks `RS` as `MIN_RES` and `CJO`/`CJSW` as `MIN_CAP` in its
    /// parameter metadata.  The global `MINRES`/`MINCAP` values therefore
    /// replace only the model-card defaults; an explicitly authored zero (or
    /// any other finite value) remains authoritative.
    pub fn apply_xyce_device_minimums(
        &mut self,
        min_resistance: Option<Value>,
        min_capacitance: Option<Value>,
        rs_given: bool,
        cj0_given: bool,
        sidewall_cj0_given: bool,
    ) {
        if !rs_given {
            if let Some(value) = min_resistance.filter(|value| value.is_finite() && *value >= 0.0) {
                self.rs = value;
            }
        }
        if let Some(value) = min_capacitance.filter(|value| value.is_finite() && *value >= 0.0) {
            if !cj0_given {
                self.cj0 = value;
            }
            if !sidewall_cj0_given {
                self.sidewall_cj0 = value;
            }
        }
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

    /// Select the native Xyce diode iteration limiter.
    ///
    /// Xyce 7.10's diode model calls the historical `DeviceSupport::pnjlim`
    /// routine.  Other dialects retain ngspice's `pnjlim_new` behavior.
    pub fn set_xyce_compatibility(&mut self, enabled: bool) {
        self.xyce_dialect = enabled;
        if enabled {
            self.ngspice_dialect = false;
        }
    }

    /// Select ngspice's native diode extension semantics.
    pub(crate) fn set_ngspice_compatibility(&mut self, enabled: bool) {
        self.ngspice_dialect = enabled;
        if enabled {
            self.xyce_dialect = false;
        }
    }

    /// Select Xyce's native transient device-convergence status policy for
    /// this run.  This is deliberately separate from the dialect selector so
    /// DC startup and explicit `ENFORCEDEVICECONV=1` remain strict.
    pub(crate) fn set_native_xyce_transient_convergence(&mut self, enabled: bool) {
        self.native_xyce_transient_convergence = enabled;
    }

    /// Evaluate the junction limiter for a candidate without mutating the
    /// Newton history. Xyce evaluates both conduction and charge from this
    /// same pnjlim-limited voltage during a Newton load; transient charge
    /// companions are assembled before the conduction stamp, so they need a
    /// side-effect-free preview of that load.
    #[inline]
    pub(crate) fn limited_junction_voltage_for(&self, vd_raw: Value) -> Value {
        let vte = self.n * self.vt;
        let vcrit = vte
            * (vte / (std::f64::consts::SQRT_2 * self.vcrit_saturation_current().max(1e-300))).ln();
        let limit_junction =
            |candidate: Value, previous: Value, thermal: Value, critical: Value| {
                if self.xyce_dialect {
                    Self::limit_xyce_pnjlim(candidate, previous, thermal, critical)
                } else {
                    super::limiting::pnjlim_new(candidate, previous, thermal, critical)
                }
            };
        if let Some(bv) = self.active_breakdown_voltage() {
            let vtebrk = self.breakdown_emission_coefficient.max(EPSMIN) * self.vt;
            if vd_raw < 0.0_f64.min(-bv + 10.0 * vtebrk) {
                let transformed = -(vd_raw + bv);
                let old_transformed = -(self.last_limited_vd.get() + bv);
                let (limited_transformed, _) =
                    limit_junction(transformed, old_transformed, vtebrk, vcrit);
                -(limited_transformed + bv)
            } else {
                let (vd, _) = limit_junction(vd_raw, self.last_limited_vd.get(), vte, vcrit);
                vd
            }
        } else {
            let (vd, _) = limit_junction(vd_raw, self.last_limited_vd.get(), vte, vcrit);
            vd
        }
    }

    /// Evaluate the voltage used by the native Xyce transient charge load.
    /// Xyce forms `Qd` from the same pnjlim-limited junction voltage as the
    /// conduction Jacobian during a Newton load; other dialects retain the
    /// raw companion voltage used by their existing transient contract.
    #[inline]
    pub(crate) fn transient_charge_voltage(&self, vd_raw: Value) -> Value {
        if self.xyce_dialect {
            self.limited_junction_voltage_for(vd_raw)
        } else {
            vd_raw
        }
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
        let delta_t = temp - tnom;
        let log_t_ratio = (temp / tnom).ln();
        let temperature = self.temperature_model;

        // Silicon bandgap at both temperatures. TLEV 0 and 1 pin the classic
        // 1.16 eV / 7.02e-4 / 1108 silicon fit; TLEV 2 lets the card supply
        // its own activation energy and correction factors through EG, GAP1
        // and GAP2.
        let (egfet, egfet1) = if temperature.tlev == 2 {
            (
                self.eg - (temperature.gap1 * temp * temp) / (temp + temperature.gap2),
                self.eg - (temperature.gap1 * tnom * tnom) / (tnom + temperature.gap2),
            )
        } else {
            (
                1.16 - (7.02e-4 * temp * temp) / (temp + 1108.0),
                1.16 - (7.02e-4 * tnom * tnom) / (tnom + 1108.0),
            )
        };
        let fact1 = tnom / REFTEMP;
        let fact2 = temp / REFTEMP;
        // ngspice's CHARGE*arg terms, folded through k/q so everything
        // stays in volts.
        let arg = -egfet / (2.0 * vt) + 1.1150877 / (2.0 * k_over_q * REFTEMP);
        let arg1 = -egfet1 / (2.0 * vtnom) + 1.1150877 / (2.0 * k_over_q * REFTEMP);
        let pbfact = -2.0 * vt * (1.5 * fact2.ln() + arg);
        let pbfact1 = -2.0 * vtnom * (1.5 * fact1.ln() + arg1);

        // Saturation-current scaling, per emission coefficient.
        //
        // TLEV 0/1: IS(T) = IS·exp(((T/TNOM)−1)·EG/(N·vt) + (XTI/N)·ln(T/TNOM))
        // TLEV 2:   IS(T) = IS·exp(EG(TNOM)/(N·vtnom) − EG(T)/(N·vt)
        //                          + (XTI/N)·ln(T/TNOM))
        //
        // The two agree to first order but diverge across a wide temperature
        // span, which is exactly the span a foundry corner sweep covers.
        let saturation_factor = |emission: Value, exponent: Value| -> Value {
            let emission = emission.max(EPSMIN);
            let thermal = emission * vt;
            let factor = if temperature.tlev == 2 {
                (egfet1 / (emission * vtnom) - egfet / thermal
                    + (exponent / emission) * log_t_ratio)
                    .exp()
            } else {
                (((temp / tnom) - 1.0) * self.eg / thermal + (exponent / emission) * log_t_ratio)
                    .exp()
            };
            if factor.is_finite() && factor > 0.0 {
                factor
            } else {
                1.0
            }
        };

        self.is *= saturation_factor(self.n, self.xti);
        self.sidewall_saturation_current *=
            saturation_factor(self.sidewall_emission_coefficient, self.xti);
        self.recombination_saturation_current *=
            saturation_factor(self.recombination_emission_coefficient, self.xti);

        // Tunneling saturation currents ride the same law, but scaled by the
        // KEG bandgap correction and their own XTITUN exponent. GF180MCU's
        // cards carry XTITUN between −12 and −46, so this term moves the
        // reverse branch by orders of magnitude across a corner sweep — it is
        // not a second-order correction.
        let tunnel_factor = {
            let emission = self.tunneling.emission.max(EPSMIN);
            let thermal = emission * vt;
            let keg = self.tunneling.bandgap_factor;
            let factor = if temperature.tlev == 2 {
                (keg * egfet1 / (emission * vtnom) - keg * egfet / thermal
                    + (self.tunneling.exponent / emission) * log_t_ratio)
                    .exp()
            } else {
                (((temp / tnom) - 1.0) * keg * self.eg / thermal
                    + (self.tunneling.exponent / emission) * log_t_ratio)
                    .exp()
            };
            if factor.is_finite() && factor > 0.0 {
                factor
            } else {
                1.0
            }
        };
        self.tunneling.bottom *= tunnel_factor;
        self.tunneling.sidewall *= tunnel_factor;

        // Grading coefficient, transit time and series resistance all take
        // the same `1 + c1·dt + c2·dt²` polynomial.
        self.m *=
            DiodeTemperatureModel::quadratic_factor(temperature.tm1, temperature.tm2, delta_t);
        self.tt *=
            DiodeTemperatureModel::quadratic_factor(temperature.ttt1, temperature.ttt2, delta_t);
        if self.rs > 0.0 {
            self.rs *= DiodeTemperatureModel::quadratic_factor(
                temperature.trs1,
                temperature.trs2,
                delta_t,
            );
        }

        // TLEVC=1 replaces the bandgap-derived junction shift with the
        // measured linear coefficients TPB/CTA (bottom) and TPHP/CTP
        // (sidewall), referred to the 27 C SPICE reference rather than to
        // TNOM. Foundry cards overwhelmingly use this form.
        if temperature.tlevc == 1 {
            if self.vj > 0.0 {
                let t_jct_pot = self.vj - temperature.tpb * (temp - REFTEMP);
                let cj = self.cj0 * (1.0 + temperature.cta * (temp - REFTEMP));
                if t_jct_pot.is_finite() && t_jct_pot > 0.0 && cj.is_finite() && cj >= 0.0 {
                    self.vj = t_jct_pot;
                    self.cj0 = cj;
                }
            }
            if self.sidewall_vj > 0.0 {
                let t_jct_pot = self.sidewall_vj - temperature.tphp * (temp - REFTEMP);
                let cj = self.sidewall_cj0 * (1.0 + temperature.ctp * (temp - REFTEMP));
                if t_jct_pot.is_finite() && t_jct_pot > 0.0 && cj.is_finite() && cj >= 0.0 {
                    self.sidewall_vj = t_jct_pot;
                    self.sidewall_cj0 = cj;
                }
            }
            self.vt = vt;
            self.temperature_breakdown_voltage = self.matched_breakdown_voltage(delta_t);
            return;
        }

        // VJ(T) and CJ0(T), TLEVC=0 bandgap mapping.
        if self.level == DiodeLevel::Pspice && self.vj > 0.0 {
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
        self.temperature_breakdown_voltage = self.matched_breakdown_voltage(delta_t);
    }

    /// Temperature-shifted breakdown voltage, run through the forward/reverse
    /// matching loop.
    ///
    /// Two shift laws live here because two dialects spell the coefficient
    /// differently. ngspice's TCV subtracts (`BV − TCV·dt`) at TLEV=0 and
    /// scales (`BV·(1 − TCV·dt)`) otherwise; Xyce's TBV1/TBV2 scale with the
    /// opposite sign (`BV·(1 + TBV1·dt + TBV2·dt²)`). A card that names TCV
    /// gets ngspice's; anything else gets Xyce's, which degenerates to no
    /// shift when TBV1 and TBV2 are absent.
    fn matched_breakdown_voltage(&self, delta_t: Value) -> Option<Value> {
        let bv = self.bv?;
        let shifted = if self.temperature_model.tcv_given {
            if self.temperature_model.tlev == 0 {
                bv - self.temperature_model.tcv * delta_t
            } else {
                bv * (1.0 - self.temperature_model.tcv * delta_t)
            }
        } else {
            bv * (1.0 + delta_t * (self.tbv1 + self.tbv2 * delta_t))
        };
        shifted
            .is_finite()
            .then_some(shifted)
            .filter(|value| *value >= 0.0)
            .and_then(|value| self.xbv_matched_breakdown_voltage(value))
    }

    /// Cached operating-point values from the last accepted Newton solution:
    /// `(vd, id, gd)` — junction voltage, current, and conductance.
    pub fn op_values(&self) -> (Value, Value, Value, Value) {
        let (id, gd) = self.current_and_conductance(self.prev_vd);
        let (_, cd) = self.junction_charge_and_capacitance(self.prev_vd);
        (self.prev_vd, id, gd, cd)
    }

    /// Apply the instance's area and multiplicity to every area-referred
    /// quantity (ngspice's `DIOarea` × `DIOm` handling).
    ///
    /// Saturation, tunneling and knee currents and the zero-bias depletion
    /// capacitance scale with the junction area; series resistance scales
    /// inversely.
    ///
    /// The breakdown knee current is the one place the LEVEL matters:
    /// `diotemp.c` scales IBV by multiplicity alone at LEVEL=1 and by area ×
    /// multiplicity at LEVEL=3. IBV feeds the forward/reverse matching loop
    /// that sets the effective breakdown voltage, so getting this wrong moves
    /// the knee rather than just its height.
    pub(crate) fn apply_instance_scaling(&mut self, area: Value, multiplicity: Value) {
        let scale = area * multiplicity;
        self.junction_scale *= scale;
        self.is *= scale;
        self.ibv *= match self.level {
            DiodeLevel::Geometric => scale,
            DiodeLevel::Legacy | DiodeLevel::Pspice => multiplicity,
        };
        self.forward_knee_current *= scale;
        self.reverse_knee_current *= scale;
        self.recombination_saturation_current *= scale;
        self.tunneling.bottom *= scale;
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
            // Both reference implementations form diffusion charge from the
            // complete junction F/G contribution, including the active
            // continuation conductance. Excluding gmin here would stamp its
            // static current but omit d/dt(TT*gmin*vd) from the same physical
            // lead current.
            let (model_current, model_conductance) = self.current_and_conductance(vd);
            let id = model_current + self.junction_gmin * vd;
            let gd = model_conductance + self.junction_gmin;
            qd += self.tt * id;
            capd += self.tt * gd;
        }

        // LEVEL=3 metal and poly overlap capacitance: bias-independent, so it
        // contributes a linear charge and a constant capacitance.
        if self.overlap_capacitance > 0.0 {
            qd += self.overlap_capacitance * vd;
            capd += self.overlap_capacitance;
        }
        (qd, capd)
    }

    /// Resolve the LEVEL=3 geometric instance parameters into the AREA and PJ
    /// factors the rest of the model is written against.
    ///
    /// ngspice derives both from the drawn rectangle when the instance gives
    /// `W` and `L`: `AREA = (W+XW)·(L+XW)·M·scale²` and
    /// `PJ = 2·((W+XW)+(L+XW))·M·scale`. Note that `M` lands here *and* again
    /// in the per-instance scaling, so a W/L-specified LEVEL=3 diode with
    /// `M>1` scales its area by `M²`. That is what ngspice-46 computes
    /// (`diosetup.c` folds `DIOm` into `DIOarea`, and `diotemp.c` multiplies
    /// by `DIOm` a second time), and the conformance corpora are pinned to it,
    /// so it is reproduced deliberately rather than quietly corrected.
    ///
    /// Returns `(area, perimeter)` — both still un-multiplied by `M`, which
    /// `apply_instance_scaling` and `set_sidewall_perimeter` apply.
    pub(crate) fn geometric_area_and_perimeter(
        width: Value,
        length: Value,
        multiplicity: Value,
        mask_offset: Value,
        scale: Value,
    ) -> (Value, Value) {
        let w = width + mask_offset;
        let l = length + mask_offset;
        (
            w * l * multiplicity * scale * scale,
            2.0 * (w + l) * multiplicity * scale,
        )
    }

    /// Set the LEVEL=3 metal and poly overlap capacitances.
    ///
    /// `CMETAL = eps_SiO2/XOM · M · (WM·scale + XM) · (LM·scale + XM)` and the
    /// same shape for poly through `XOI`/`XP`. Both are plain parallel-plate
    /// overlaps between the diode's routing and the bulk, so they add a fixed
    /// capacitance rather than a junction one.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn set_overlap_capacitance(
        &mut self,
        multiplicity: Value,
        width_metal: Value,
        length_metal: Value,
        width_poly: Value,
        length_poly: Value,
        metal_oxide_thickness: Value,
        poly_oxide_thickness: Value,
        metal_mask_offset: Value,
        poly_mask_offset: Value,
        scale: Value,
    ) {
        let plate = |width: Value, length: Value, thickness: Value, offset: Value| -> Value {
            if !(thickness.is_finite() && thickness > 0.0) {
                return 0.0;
            }
            let value = EPS_SIO2 / thickness
                * multiplicity
                * (width * scale + offset)
                * (length * scale + offset);
            if value.is_finite() && value > 0.0 {
                value
            } else {
                0.0
            }
        };
        self.overlap_capacitance = plate(
            width_metal,
            length_metal,
            metal_oxide_thickness,
            metal_mask_offset,
        ) + plate(
            width_poly,
            length_poly,
            poly_oxide_thickness,
            poly_mask_offset,
        );
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
        let (id, gd) = self.candidate_current_and_conductance(vd);
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

    /// Physical static current stamped from anode to cathode at `vd`.
    ///
    /// Junction continuation conductance is part of the device's nonlinear
    /// F contribution in both Xyce and ngspice. Transient lead-current
    /// publication must therefore retain it alongside the model conduction
    /// current before the integration-owned dQ/dt term is added.
    #[inline]
    pub(crate) fn stamped_conduction_current(&self, vd: Value) -> Value {
        self.current(vd) + self.junction_gmin * vd
    }

    /// Junction current and conductance in one evaluation.
    ///
    /// Xyce treats JSW without NS as extra saturation current on the bottom
    /// diode, while JSW with NS is a distinct sidewall diode on the same
    /// junction voltage. That distinction matters for high-injection limiting
    /// and for the sidewall current shape.
    fn current_and_conductance(&self, vd: Value) -> (Value, Value) {
        // Bottom junction. ngspice applies IKF/IKR after summing its bottom
        // mechanisms; the Cadence PSpice law and Xyce apply the knee to the
        // normal current and add recombination separately.
        let (mut bottom_i, mut bottom_g) =
            self.exponential_current_and_conductance(vd, self.bottom_saturation_current(), self.n);
        let (recombination_i, recombination_g) = self.recombination_current_and_conductance(vd);
        if self.tunneling.bottom_given {
            let (tunnel_i, tunnel_g) =
                self.tunnel_current_and_conductance(vd, self.tunnel_bottom());
            bottom_i += tunnel_i;
            bottom_g += tunnel_g;
        }
        if self.ngspice_dialect {
            bottom_i += recombination_i;
            bottom_g += recombination_g;
            (bottom_i, bottom_g) = self.apply_high_injection_knee(vd, bottom_i, bottom_g);
        } else {
            (bottom_i, bottom_g) = self.apply_high_injection_knee(vd, bottom_i, bottom_g);
            bottom_i += recombination_i;
            bottom_g += recombination_g;
        }

        // Sidewall junction: its own exponential plus sidewall tunneling,
        // then the IKP knee.
        let (mut sidewall_i, mut sidewall_g) = self.sidewall_current_and_conductance(vd);
        if self.tunneling.sidewall_given {
            let (tunnel_i, tunnel_g) =
                self.tunnel_current_and_conductance(vd, self.tunnel_sidewall());
            sidewall_i += tunnel_i;
            sidewall_g += tunnel_g;
        }
        let (sidewall_i, sidewall_g) = Self::apply_forward_knee(
            sidewall_i,
            sidewall_g,
            self.sidewall_knee_current * self.sidewall_perimeter,
        );

        (bottom_i + sidewall_i, bottom_g + sidewall_g)
    }

    /// Reuse the evaluation populated by `update` only at the exact same
    /// terminal bias.  This is a semantic candidate cache, independent of
    /// circuit identity, topology size, or analysis deck; every mismatch
    /// falls through to the canonical model law.
    #[inline]
    fn candidate_current_and_conductance(&self, vd: Value) -> (Value, Value) {
        if self.candidate_eval_valid && vd.to_bits() == self.prev_vd.to_bits() {
            (self.prev_id, self.prev_gd)
        } else {
            self.current_and_conductance(vd)
        }
    }

    /// Sidewall junction current, whether or not the card gave it its own
    /// emission coefficient.
    ///
    /// JSW alone puts the sidewall on the bottom junction's characteristic —
    /// same emission voltage, same region boundaries — but it stays a
    /// *separate* current, because ngspice applies the IKF/IKR knee to the
    /// bottom branch only. Folding JSW into IS instead would drag the
    /// sidewall through high-injection limiting that ngspice never applies to
    /// it. JSW with NS gives the sidewall its own characteristic outright.
    ///
    /// A shared-characteristic sidewall has no breakdown region. In
    /// `dioload.c` the breakdown arm evaluates it from `vdsw`, which is
    /// declared `vdsw = 0.0` and only ever assigned when the model gives a
    /// separate sidewall resistance RSW — so without RSW the sidewall
    /// breakdown term is `exp(-BV/vtebrk)`, which underflows to zero. The
    /// author's intent was plainly the common voltage `vd`; the slip is
    /// reproduced anyway, because foundry cards are *extracted against this
    /// implementation*. GF180MCU's JSW and BV were fitted to curves in which
    /// the sidewall does not break down, so evaluating the physically
    /// intended equation with those parameters would overstate the knee by
    /// exactly `(IS + JSW·PJ)/IS` — 3.6x on the corpus's high-perimeter
    /// geometries. Matching the equations the parameters were measured with
    /// is what makes the parameters mean anything.
    fn sidewall_current_and_conductance(&self, vd: Value) -> (Value, Value) {
        if !self.sidewall_current_given {
            return (0.0, 0.0);
        }
        let isat = self.sidewall_saturation_current * self.sidewall_perimeter;
        if self.sidewall_emission_given {
            return self.junction_branch(vd, isat, self.sidewall_emission_coefficient, true);
        }
        if self.xyce_dialect {
            // Already merged into the bottom junction; see
            // `bottom_saturation_current`.
            return (0.0, 0.0);
        }
        self.junction_branch(vd, isat, self.n, false)
    }

    /// Saturation current driving the bottom junction's exponential.
    ///
    /// Xyce merges a characteristic-less sidewall in here so it shares the
    /// bottom's high-injection knee and breakdown region; ngspice keeps it
    /// separate.
    fn bottom_saturation_current(&self) -> Value {
        if self.xyce_dialect {
            self.is + self.merged_sidewall_saturation_current()
        } else {
            self.is
        }
    }

    /// The sidewall saturation current Xyce folds onto the bottom junction:
    /// non-zero only when JSW was given without its own NS.
    fn merged_sidewall_saturation_current(&self) -> Value {
        if self.sidewall_current_given && !self.sidewall_emission_given {
            self.sidewall_saturation_current * self.sidewall_perimeter
        } else {
            0.0
        }
    }

    /// Saturation current the junction limiter's `Vcrit` is written against.
    fn vcrit_saturation_current(&self) -> Value {
        if self.xyce_dialect {
            self.bottom_saturation_current()
        } else {
            self.total_saturation_current()
        }
    }

    /// Saturation current the breakdown-matching loop is written against.
    ///
    /// ngspice matches against `totalSatCur`; Xyce matches against the bottom
    /// `tSatCur` alone, even when it has merged the sidewall into the current
    /// it actually evaluates.
    fn matching_saturation_current(&self) -> Value {
        if self.xyce_dialect {
            self.is
        } else {
            self.total_saturation_current()
        }
    }

    /// Temperature-scaled bottom tunneling saturation current.
    fn tunnel_bottom(&self) -> Value {
        self.tunneling.bottom
    }

    /// Temperature-scaled sidewall tunneling saturation current, referred to
    /// the instance perimeter.
    fn tunnel_sidewall(&self) -> Value {
        self.tunneling.sidewall * self.sidewall_perimeter
    }

    /// Band-to-band tunneling: `−Jtun·(exp(−vd/(NTUN·vt)) − 1)`.
    ///
    /// Reverse-biased, so the exponent grows as `vd` falls. The conductance is
    /// the exact derivative, `+Jtun·exp(−vd/(NTUN·vt))/(NTUN·vt)`, which stays
    /// positive: tunneling makes the junction *more* conductive in reverse,
    /// which is the whole point of the mechanism.
    fn tunnel_current_and_conductance(&self, vd: Value, saturation: Value) -> (Value, Value) {
        if !(saturation.is_finite() && saturation != 0.0) {
            return (0.0, 0.0);
        }
        let thermal = self.tunneling.emission.max(EPSMIN) * self.vt;
        if !(thermal.is_finite() && thermal > 0.0) {
            return (0.0, 0.0);
        }
        let (exponential, derivative) = Self::limited_exp(-vd / thermal, MAX_EXP_ARG);
        (
            -saturation * (exponential - 1.0),
            saturation * derivative / thermal,
        )
    }

    /// Total saturation current across both junctions.
    ///
    /// ngspice's `totalSatCur`: the quantity `Vcrit` and the breakdown-voltage
    /// matching loop are both written against, regardless of whether the
    /// sidewall carries its own emission coefficient.
    fn total_saturation_current(&self) -> Value {
        let sidewall = if self.sidewall_current_given {
            self.sidewall_saturation_current * self.sidewall_perimeter
        } else {
            0.0
        };
        self.is + sidewall
    }

    /// PSpice depletion-region recombination current.
    ///
    /// Cadence's PSpice reference law adds `Irec*Kgen` to the diode current at
    /// the actual junction voltage without a reverse-bias cutoff. ngspice
    /// instead freezes that contribution at `-3*N*Vt` in reverse, while Xyce
    /// 7.10 omits it there. Explicit dialect selection retains each published
    /// behavior; best-available mode follows the Cadence equation. The
    /// exponential uses `NR*Vt`, then the depletion generation factor shapes
    /// both the current and its analytic Jacobian.
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
            && (!self.xyce_dialect || vd >= forward_boundary))
        {
            return (0.0, 0.0);
        }

        // ngspice freezes the recombination contribution at the ordinary
        // diode's -3*N*Vt boundary throughout reverse and breakdown. Cadence's
        // PSpice equation uses the actual junction voltage; Xyce was handled by
        // the early return above.
        let evaluation_vd = if self.ngspice_dialect && vd < forward_boundary {
            forward_boundary
        } else {
            vd
        };
        let nr_vt = nr * self.vt;
        let (exponential, exponential_derivative) =
            Self::limited_exp(evaluation_vd / nr_vt, MAX_EXP_ARG);
        let base_current = isr * (exponential - 1.0);
        let base_conductance = (isr / nr_vt) * exponential_derivative;

        let normalized_depletion = 1.0 - evaluation_vd / self.vj;
        let generation_base = normalized_depletion * normalized_depletion + 0.005;
        let generation_exponent = 0.5 * self.m;
        let generation_factor = generation_base.powf(generation_exponent);
        let generation_derivative = -self.m * normalized_depletion / self.vj
            * generation_base.powf(generation_exponent - 1.0);
        if !(generation_factor.is_finite() && generation_derivative.is_finite()) {
            return (0.0, 0.0);
        }

        let current = base_current * generation_factor;
        let conductance = if self.ngspice_dialect && vd < forward_boundary {
            0.0
        } else {
            base_conductance * generation_factor + base_current * generation_derivative
        };
        (current, conductance)
    }

    fn exponential_current_and_conductance(
        &self,
        vd: Value,
        isat: Value,
        emission_coefficient: Value,
    ) -> (Value, Value) {
        self.junction_branch(vd, isat, emission_coefficient, true)
    }

    /// Forward / reverse / breakdown junction branches for one exponential.
    ///
    /// `breakdown` selects whether this junction has a breakdown region at
    /// all. When it does not, biases past `-BV` produce nothing rather than
    /// continuing the reverse formula — see
    /// `sidewall_current_and_conductance` for why one junction ends up
    /// without one.
    fn junction_branch(
        &self,
        vd: Value,
        isat: Value,
        emission_coefficient: Value,
        breakdown: bool,
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
            if !breakdown {
                return (0.0, 0.0);
            }
            let vtebrk = self.breakdown_emission_coefficient.max(EPSMIN) * self.vt;
            let (e, de_darg) = Self::limited_exp(-(brkdwn_v + vd) / vtebrk, BREAKDOWN_EXP_ARG_MAX);
            return (-isat * e, (isat / vtebrk) * de_darg);
        }

        let mut arg = 3.0 * n_vt / (vd * std::f64::consts::E);
        arg = arg * arg * arg;
        (-isat * (1.0 + arg), isat * 3.0 * arg / vd)
    }

    /// The breakdown voltage the reverse branch is actually written against.
    ///
    /// May be negative: the matching loop solves for the voltage at which the
    /// forward and reverse regions meet, and a card with a small BV and a
    /// comparatively large IBV drives that meeting point below zero. When it
    /// does, `vd < -brkdwn_v` is satisfied everywhere in reverse, which is the
    /// intended reading — the junction has no ordinary reverse region left.
    fn active_breakdown_voltage(&self) -> Option<Value> {
        if let Some(brkdwn_v) = self.temperature_breakdown_voltage
            && brkdwn_v.is_finite()
        {
            return Some(brkdwn_v);
        }

        self.bv.and_then(|bv| {
            if bv.is_finite() && bv >= 0.0 {
                self.xbv_matched_breakdown_voltage(bv).or(Some(bv))
            } else {
                None
            }
        })
    }

    fn xbv_matched_breakdown_voltage(&self, bv: Value) -> Option<Value> {
        let vt = self.vt;
        let isat = self.matching_saturation_current().max(1e-300);
        let cbv = self.ibv;
        let nbv = self.breakdown_emission_coefficient.max(EPSMIN);
        if !(bv.is_finite()
            && bv >= 0.0
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
        if xbv.is_finite() { Some(xbv) } else { Some(bv) }
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
        if !(cj0 > 0.0 && vj > 0.0 && grading.is_finite()) {
            return (0.0, 0.0);
        }

        let fc = fc.clamp(0.0, 0.95);
        let dep_cap_knee = fc * vj;
        if vd < dep_cap_knee {
            let arg = 1.0 - vd / vj;
            let sarg_arg = (-grading * arg.ln()).min(MAX_EXP_ARG);
            let sarg = sarg_arg.exp();
            let charge = if (grading - 1.0).abs() < 1.0e-12 {
                -vj * cj0 * arg.ln()
            } else {
                vj * cj0 * (1.0 - arg * sarg) / (1.0 - grading)
            };
            let capacitance = cj0 * sarg;
            return (charge, capacitance);
        }

        let f1 = if (grading - 1.0).abs() < 1.0e-12 {
            -vj * (1.0 - fc).ln()
        } else {
            vj * (1.0 - (1.0 - fc).powf(1.0 - grading)) / (1.0 - grading)
        };
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
        let candidate_current = if self.candidate_eval_valid {
            self.prev_id
        } else {
            self.current(self.prev_vd)
        } + self.junction_gmin * self.prev_vd;
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
        (self.prev_id, self.prev_gd) = self.current_and_conductance(self.prev_vd);
        self.candidate_eval_valid = true;
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
        let native_xyce_transient = self.xyce_dialect && self.native_xyce_transient_convergence;
        // A pnjlim-clamped step must iterate again regardless of the
        // voltage delta (ngspice `Check` semantics).
        !self.limited.get()
            && (native_xyce_transient || (self.prev_vd - self.prev_vd_old).abs() < tolerance)
            && (native_xyce_transient || self.linearized_current_matches_candidate(criteria))
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
    fn off_instance_starts_its_first_linearization_at_zero_bias() {
        // dioload.c evaluates an OFF instance at exactly `vd = 0` on
        // MODEINITJCT. A junction whose saturation current is large enough for
        // pnjlim's zero reference to still conduct is where that differs from
        // merely limiting the raw bias, so use one: the active instance below
        // stamps milliamps on its first load, the OFF instance stamps nothing.
        let forward_bias = 5.0;

        let mut active = test_diode();
        active.is = 1.0e-3;
        let (vd, id, _) = active.limited_linearization(forward_bias);
        assert!(
            vd > 0.0 && id > 1.0e-3,
            "an active diode opens forward-biased: vd={vd} id={id}"
        );

        let mut off = test_diode();
        off.is = 1.0e-3;
        off.set_initially_off(true);
        assert!(off.is_initially_off());
        let (vd, id, _) = off.limited_linearization(forward_bias);
        assert_eq!(vd, 0.0, "OFF must load the junction at zero bias");
        assert_eq!(id, 0.0, "a zero-bias junction carries no current");

        // OFF owns the first load only: the pnjlim history it leaves behind is
        // zero, and the next iterate limits away from it like any other.
        let (next_vd, _, _) = off.limited_linearization(forward_bias);
        assert!(
            next_vd > 0.0,
            "the second iterate must track the bias again: vd={next_vd}"
        );
    }

    #[test]
    fn unmarked_instance_starts_its_first_linearization_at_tvcrit() {
        // dioload.c:162-166 opens an unmarked junction at `vd = tVcrit`
        // whatever the terminals say, and `tVcrit = vte·ln(vte/(√2·Isat))` is
        // by construction the bias where `gd = Isat·exp(vd/vte)/vte` is
        // exactly `1/√2 S` — independent of Isat, temperature or area. That
        // identity is the whole check: it pins both the voltage and the
        // conductance the first Jacobian carries.
        let sqrt2_conductance = 1.0 / std::f64::consts::SQRT_2;

        for (label, is, n) in [
            ("small-signal", 1.0e-14, 1.0),
            ("power rectifier", 1.0e-9, 1.8),
            ("high injection", 1.0e-3, 1.0),
        ] {
            for raw in [-40.0_f64, 0.0, 5.0] {
                let mut diode = test_diode();
                diode.is = is;
                diode.n = n;
                let vte = diode.n * diode.vt;
                let expected = vte
                    * (vte / (std::f64::consts::SQRT_2 * diode.total_saturation_current())).ln();

                let (vd, _id, gd) = diode.limited_linearization(raw);
                assert!(
                    (vd - expected).abs() <= 1.0e-12,
                    "{label} at raw={raw} must open at tVcrit={expected}, found {vd}"
                );
                assert!(
                    (gd - sqrt2_conductance).abs() <= 1.0e-9,
                    "{label} at raw={raw} must open with gd=1/√2 S, found {gd}"
                );
            }
        }

        // The startup arm owns the first load only. It leaves tVcrit behind as
        // the pnjlim history, so the second iterate limits against it: a raw
        // bias far past tVcrit is clamped short of itself rather than taken.
        let diode = test_diode();
        let vte = diode.n * diode.vt;
        let vcrit =
            vte * (vte / (std::f64::consts::SQRT_2 * diode.total_saturation_current())).ln();
        diode.limited_linearization(5.0);
        let (second, _, _) = diode.limited_linearization(5.0);
        assert!(
            second > vcrit && second < 5.0,
            "the second iterate must limit away from tVcrit={vcrit}, found {second}"
        );
    }

    #[test]
    fn xyce_transient_status_policy_is_explicit_and_limiter_owned() {
        let mut diode = test_diode();
        diode.set_xyce_compatibility(true);
        diode.prev_vd = 0.8;
        diode.prev_vd_old = 0.0;
        diode.limited.set(false);

        let criteria = NonlinearConvergenceCriteria::default();
        assert!(!diode.is_converged(criteria));

        diode.set_native_xyce_transient_convergence(true);
        assert!(diode.is_converged(criteria));

        diode.limited.set(true);
        assert!(!diode.is_converged(criteria));
    }

    #[test]
    fn candidate_evaluation_cache_is_exact_bias_keyed_and_rollback_safe() {
        let assert_pair_bits = |actual: (Value, Value), expected: (Value, Value)| {
            assert_eq!(actual.0.to_bits(), expected.0.to_bits());
            assert_eq!(actual.1.to_bits(), expected.1.to_bits());
        };
        let mut diode = test_diode();
        let candidate = 0.413_25;
        let expected = diode.current_and_conductance(candidate);
        diode.update(&[candidate, 0.0]);
        assert_pair_bits(diode.candidate_current_and_conductance(candidate), expected);

        let mismatch = candidate + 1.0e-6;
        let mismatch_expected = diode.current_and_conductance(mismatch);
        assert_pair_bits(
            diode.candidate_current_and_conductance(mismatch),
            mismatch_expected,
        );

        let snapshot = diode.nonlinear_state_snapshot();
        diode.update(&[0.62, 0.0]);
        diode.restore_nonlinear_state(snapshot);
        assert_pair_bits(diode.candidate_current_and_conductance(candidate), expected);
    }

    /// GF180MCU's `np_3p3`, the card that drove the LEVEL=3 work.
    ///
    /// Values are the `typical` corner of `sm141064.ngspice` with the corner
    /// multipliers folded in, so the numbers here are the ones ngspice-46
    /// evaluates for the vendored `np_3p3_typical_*` cases.
    fn gf180_np_3p3() -> std::collections::HashMap<String, Value> {
        [
            ("LEVEL", 3.0),
            ("TREF", 25.0),
            ("IS", 2.2959e-7),
            ("JSW", 2.1207e-13),
            ("IK", 300000.0),
            ("BV", 11.0),
            ("IBV", 0.001),
            ("N", 1.01),
            ("RS", 2e-10),
            ("JTUN", 1.1223e-5),
            ("JTUNSW", 6.4125e-12),
            ("NTUN", 10.0),
            ("CJ", 0.00096797),
            ("CJP", 1.5663e-10),
            ("PB", 0.70172),
            ("PHP", 0.8062),
            ("MJ", 0.32071),
            ("MJSW", 0.1),
            ("TLEV", 1.0),
            ("TLEVC", 1.0),
            ("TRS", 4.5778e-5),
            ("XTI", 3.0),
            ("XTITUN", -25.0),
            ("CTA", 0.0009438),
            ("CTP", 0.00060474),
            ("EG", 1.17),
            ("TPB", 0.0018129),
            ("TPHP", 5e-5),
        ]
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect()
    }

    /// A GF180MCU diode as the builder assembles it: model card, then the
    /// instance's `AREA=100p PJ=40u`, then temperature.
    fn gf180_instance(params: &std::collections::HashMap<String, Value>) -> Diode {
        let mut d = Diode::spice_defaults("dn1".to_string(), 1, 0).with_model_params(params);
        d.apply_instance_scaling(100e-12, 1.0);
        d.set_sidewall_perimeter(40e-6);
        d.set_temperature(REFTEMP, REFTEMP);
        d
    }

    #[test]
    fn level_three_selects_the_geometric_model() {
        let d = Diode::spice_defaults("d1".to_string(), 1, 0).with_model_params(&gf180_np_3p3());
        assert_eq!(d.level, DiodeLevel::Geometric);
    }

    /// The parameter aliases foundry cards actually use. Each of these
    /// silently defaulted before LEVEL=3 support, which is worse than
    /// rejecting the card: `PB` and `MJ` alone would leave every junction at
    /// `VJ=1.0, M=0.5`.
    #[test]
    fn foundry_parameter_aliases_reach_their_canonical_fields() {
        let params: std::collections::HashMap<String, Value> = [
            ("PB", 0.70172),
            ("MJ", 0.32071),
            ("TREF", 25.0),
            ("ISW", 3.5e-13),
            ("IB", 2e-3),
            ("NZ", 1.4),
            ("VRB", 7.5),
            ("CTC", 1.5e-3),
            ("TVJ", 2.5e-3),
            ("TRS1", 4e-5),
        ]
        .into_iter()
        .map(|(name, value)| (name.to_string(), value))
        .collect();
        let d = Diode::spice_defaults("d1".to_string(), 1, 0).with_model_params(&params);

        assert_eq!(d.vj, 0.70172);
        assert_eq!(d.m, 0.32071);
        assert_eq!(d.tnom_c, Some(25.0));
        assert_eq!(d.sidewall_saturation_current, 3.5e-13);
        assert!(d.sidewall_current_given);
        assert_eq!(d.ibv, 2e-3);
        assert_eq!(d.breakdown_emission_coefficient, 1.4);
        assert!(d.breakdown_emission_given);
        assert_eq!(d.bv, Some(7.5));
        assert_eq!(d.temperature_model.cta, 1.5e-3);
        assert_eq!(d.temperature_model.tpb, 2.5e-3);
        assert_eq!(d.temperature_model.trs1, 4e-5);
    }

    #[test]
    fn vishay_t_measured_alias_sets_the_model_nominal_temperature() {
        let params: std::collections::HashMap<String, Value> = [("T_MEASURED", 27.0)]
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect();
        let d = Diode::spice_defaults("d1".to_string(), 1, 0).with_model_params(&params);

        assert_eq!(d.tnom_c, Some(27.0));
    }

    #[test]
    fn pspice_recombination_extends_through_reverse_bias_but_xyce_retains_its_cutoff() {
        let mut diode = test_diode();
        diode.n = 1.0136;
        diode.vj = 1.0542;
        diode.m = 0.55916;
        diode.recombination_saturation_current = 564.09e-9;
        diode.recombination_emission_coefficient = 4.995;

        let vd = -10.0;
        let nr_vt = diode.recombination_emission_coefficient * diode.vt;
        let exponential = (vd / nr_vt).exp();
        let base_current = diode.recombination_saturation_current * (exponential - 1.0);
        let base_conductance = diode.recombination_saturation_current * exponential / nr_vt;
        let normalized_depletion = 1.0 - vd / diode.vj;
        let generation_base = normalized_depletion * normalized_depletion + 0.005;
        let generation_exponent = 0.5 * diode.m;
        let generation_factor = generation_base.powf(generation_exponent);
        let generation_derivative = -diode.m * normalized_depletion / diode.vj
            * generation_base.powf(generation_exponent - 1.0);
        let expected_current = base_current * generation_factor;
        let expected_conductance =
            base_conductance * generation_factor + base_current * generation_derivative;

        let (native_current, native_conductance) = diode.recombination_current_and_conductance(vd);
        assert!((native_current - expected_current).abs() <= expected_current.abs() * 1.0e-12);
        assert!(
            (native_conductance - expected_conductance).abs()
                <= expected_conductance.abs() * 1.0e-12
        );

        diode.set_ngspice_compatibility(true);
        let boundary = -3.0 * diode.n * diode.vt;
        let boundary_exponential = (boundary / nr_vt).exp();
        let boundary_base_current =
            diode.recombination_saturation_current * (boundary_exponential - 1.0);
        let boundary_normalized = 1.0 - boundary / diode.vj;
        let boundary_generation =
            (boundary_normalized * boundary_normalized + 0.005).powf(generation_exponent);
        let (ngspice_current, ngspice_conductance) =
            diode.recombination_current_and_conductance(vd);
        let ngspice_expected = boundary_base_current * boundary_generation;
        assert!((ngspice_current - ngspice_expected).abs() <= ngspice_expected.abs() * 1.0e-12);
        assert_eq!(ngspice_conductance, 0.0);

        diode.set_xyce_compatibility(true);
        assert_eq!(diode.recombination_current_and_conductance(vd), (0.0, 0.0));
    }

    #[test]
    fn pspice_high_injection_limits_normal_current_before_adding_recombination() {
        let mut diode = test_diode();
        diode.is = 1.0e-6;
        diode.n = 1.0;
        diode.forward_knee_current = 1.0e-4;
        diode.recombination_saturation_current = 1.0e-6;
        diode.recombination_emission_coefficient = 2.0;
        let vd = 0.2;

        let normal = diode.exponential_current_and_conductance(vd, diode.is, diode.n);
        let recombination = diode.recombination_current_and_conductance(vd);
        let limited_normal = diode.apply_high_injection_knee(vd, normal.0, normal.1);
        let expected_pspice = (
            limited_normal.0 + recombination.0,
            limited_normal.1 + recombination.1,
        );
        let actual_pspice = diode.current_and_conductance(vd);
        assert_eq!(actual_pspice.0.to_bits(), expected_pspice.0.to_bits());
        assert_eq!(actual_pspice.1.to_bits(), expected_pspice.1.to_bits());

        diode.set_ngspice_compatibility(true);
        let combined = (normal.0 + recombination.0, normal.1 + recombination.1);
        let expected_ngspice = diode.apply_high_injection_knee(vd, combined.0, combined.1);
        let actual_ngspice = diode.current_and_conductance(vd);
        assert_eq!(actual_ngspice.0.to_bits(), expected_ngspice.0.to_bits());
        assert_eq!(actual_ngspice.1.to_bits(), expected_ngspice.1.to_bits());
    }

    /// Tunneling is the dominant reverse mechanism in a foundry junction
    /// diode, not a correction to it.
    ///
    /// Measured against ngspice-46 on this exact card: at −12.13 V an
    /// `np_3p3` sources 1.61 A with JTUN present and 0.19 A with it removed.
    /// The junction voltage here is well short of that, because the deck's
    /// 2 Ω series resistance absorbs most of the applied bias — so the test
    /// pins the ratio rather than an absolute current.
    #[test]
    fn tunneling_dominates_the_reverse_branch() {
        let with_tunneling = gf180_instance(&gf180_np_3p3());
        let mut without = gf180_np_3p3();
        without.remove("JTUN");
        without.remove("JTUNSW");
        let without_tunneling = gf180_instance(&without);

        let vd = -8.0;
        let tunneling = with_tunneling.current(vd).abs();
        let plain = without_tunneling.current(vd).abs();

        assert!(
            tunneling > plain * 1e3,
            "tunneling must dominate: {tunneling:e} vs {plain:e}"
        );
    }

    /// `BV=0` is a setting, not an absence.
    ///
    /// ngspice and Xyce both gate breakdown on whether the card named BV. With
    /// `BV=0` the matching loop solves for a negative effective breakdown
    /// voltage, so `vd < -tBrkdwnV` holds everywhere in reverse and the whole
    /// region becomes the breakdown exponential. GF180MCU's `nwp_3p3` depends
    /// on it: ngspice-46 gives 5.65 A at −12.13 V with `bv=0` against 81 µA
    /// with BV omitted.
    #[test]
    fn zero_breakdown_voltage_is_given_rather_than_absent() {
        let mut params = gf180_np_3p3();
        params.insert("BV".to_string(), 0.0);
        let zero_bv = gf180_instance(&params);

        params.remove("BV");
        let no_bv = gf180_instance(&params);

        assert_eq!(zero_bv.bv, Some(0.0));
        assert_eq!(no_bv.bv, None);
        let matched = zero_bv
            .active_breakdown_voltage()
            .expect("BV=0 still yields a matched breakdown voltage");
        assert!(
            matched < 0.0,
            "matching a zero breakdown voltage lands below zero, got {matched}"
        );
        assert!(
            zero_bv.current(-2.0).abs() > no_bv.current(-2.0).abs() * 1e3,
            "BV=0 must open the breakdown branch across the whole reverse region"
        );
    }

    /// ngspice evaluates a characteristic-less sidewall from `vdsw`, which it
    /// only assigns when RSW is given — so the sidewall contributes nothing in
    /// breakdown. Reproduced deliberately; see
    /// `sidewall_current_and_conductance`.
    #[test]
    fn ngspice_sidewall_without_ns_has_no_breakdown_region() {
        let d = gf180_instance(&gf180_np_3p3());
        let brkdwn = d
            .active_breakdown_voltage()
            .expect("card gives BV, so a matched voltage exists");
        let deep = -(brkdwn + 1.0);

        let (sidewall_i, sidewall_g) = d.sidewall_current_and_conductance(deep);
        assert_eq!((sidewall_i, sidewall_g), (0.0, 0.0));

        // Still present either side of the breakdown knee.
        assert!(d.sidewall_current_and_conductance(0.4).0 > 0.0);
        assert!(d.sidewall_current_and_conductance(-1.0).0 < 0.0);
    }

    /// Xyce takes the opposite route on the same card: `Isat = Isat + IsatSW`
    /// merges the sidewall into the bottom junction, so it shares the knee and
    /// the breakdown region rather than skipping both.
    #[test]
    fn xyce_merges_a_characteristic_less_sidewall_into_the_bottom_junction() {
        let mut d = gf180_instance(&gf180_np_3p3());
        d.set_xyce_compatibility(true);

        assert_eq!(d.sidewall_current_and_conductance(0.4), (0.0, 0.0));
        assert!(
            d.bottom_saturation_current() > d.is,
            "Xyce folds JSW·PJ into the bottom saturation current"
        );
        // ngspice keeps them apart, so the same card splits differently.
        d.set_xyce_compatibility(false);
        assert_eq!(d.bottom_saturation_current(), d.is);
        assert!(d.sidewall_current_and_conductance(0.4).0 > 0.0);
    }

    /// TLEVC=1 replaces the bandgap-derived junction shift with measured
    /// linear coefficients, referred to 27 C rather than to TNOM.
    #[test]
    fn tlevc_one_uses_the_measured_linear_capacitance_law() {
        let params = gf180_np_3p3();
        let mut d = Diode::spice_defaults("d1".to_string(), 1, 0).with_model_params(&params);
        let (cj0, vj, sidewall_cj0, sidewall_vj) = (d.cj0, d.vj, d.sidewall_cj0, d.sidewall_vj);
        let temp = 273.15 + 125.0;
        d.set_temperature(temp, REFTEMP);

        let dt = temp - REFTEMP;
        assert!((d.cj0 - cj0 * (1.0 + 0.0009438 * dt)).abs() <= cj0 * 1e-12);
        assert!((d.vj - (vj - 0.0018129 * dt)).abs() <= vj * 1e-12);
        assert!((d.sidewall_cj0 - sidewall_cj0 * (1.0 + 0.00060474 * dt)).abs() <= 1e-24);
        assert!((d.sidewall_vj - (sidewall_vj - 5e-5 * dt)).abs() <= sidewall_vj * 1e-12);
    }

    /// TRS moves the series resistance, and the GF180MCU decks are sensitive
    /// to it: the reverse branch runs at amps through a 2 Ω series path, so a
    /// 0.3% shift in RS moves the junction bias enough to matter.
    #[test]
    fn trs_scales_series_resistance_against_tnom() {
        let mut params = gf180_np_3p3();
        params.insert("RS".to_string(), 4.0);
        let mut d = Diode::spice_defaults("d1".to_string(), 1, 0).with_model_params(&params);
        let temp = 273.15 - 40.0;
        d.set_temperature(temp, 273.15 + 25.0);

        let dt = temp - (273.15 + 25.0);
        assert!((d.rs - 4.0 * (1.0 + 4.5778e-5 * dt)).abs() <= 1e-12);
        assert!(d.rs < 4.0, "a cold junction lowers RS on a positive TRS");
    }

    /// The tunneling saturation current rides its own XTITUN exponent and KEG
    /// bandgap correction, not the junction's XTI.
    #[test]
    fn tunneling_temperature_law_uses_xtitun_and_keg() {
        let params = gf180_np_3p3();
        let mut d = Diode::spice_defaults("d1".to_string(), 1, 0).with_model_params(&params);
        let (jtun, tnom) = (d.tunneling.bottom, 273.15 + 25.0);
        let temp = 273.15 + 125.0;
        d.set_temperature(temp, tnom);

        let vtt = 10.0 * KOVERQ * temp;
        let expected =
            jtun * (((temp / tnom) - 1.0) * 1.17 / vtt + (-25.0 / 10.0) * (temp / tnom).ln()).exp();
        assert!(
            (d.tunneling.bottom - expected).abs() <= expected * 1e-12,
            "{} vs {expected}",
            d.tunneling.bottom
        );
    }

    /// LEVEL=1 scales the breakdown knee current by multiplicity alone;
    /// LEVEL=3 scales it by area as well. IBV feeds the matching loop, so this
    /// moves the knee rather than just its height.
    #[test]
    fn breakdown_knee_current_scales_by_area_only_at_level_three() {
        let scale_ibv = |level: Value| {
            let params: std::collections::HashMap<String, Value> =
                [("LEVEL", level), ("IBV", 1e-3), ("BV", 10.0)]
                    .into_iter()
                    .map(|(name, value)| (name.to_string(), value))
                    .collect();
            let mut d = Diode::spice_defaults("d1".to_string(), 1, 0).with_model_params(&params);
            d.apply_instance_scaling(4.0, 3.0);
            d.ibv
        };

        assert!((scale_ibv(1.0) - 1e-3 * 3.0).abs() <= 1e-18);
        assert!((scale_ibv(3.0) - 1e-3 * 12.0).abs() <= 1e-18);
    }

    /// LEVEL=3 derives AREA and PJ from the drawn rectangle. The multiplicity
    /// appears here and again in the instance scaling, which is what
    /// ngspice-46 computes; see `geometric_area_and_perimeter`.
    #[test]
    fn geometric_area_and_perimeter_come_from_the_drawn_rectangle() {
        let (area, perimeter) = Diode::geometric_area_and_perimeter(2e-6, 5e-6, 2.0, 1e-7, 1.0);
        let (w, l) = (2e-6 + 1e-7, 5e-6 + 1e-7);
        assert!((area - w * l * 2.0).abs() <= area * 1e-12);
        assert!((perimeter - 2.0 * (w + l) * 2.0).abs() <= perimeter * 1e-12);

        // `.options scale` multiplies each dimension, so it enters area squared.
        let (scaled_area, scaled_perimeter) =
            Diode::geometric_area_and_perimeter(2e-6, 5e-6, 2.0, 1e-7, 2.0);
        assert!((scaled_area - area * 4.0).abs() <= scaled_area * 1e-12);
        assert!((scaled_perimeter - perimeter * 2.0).abs() <= scaled_perimeter * 1e-12);
    }

    /// The LEVEL=3 metal and poly overlaps are plain parallel plates, so they
    /// add a bias-independent capacitance on top of the junction's.
    #[test]
    fn overlap_capacitance_is_bias_independent() {
        let mut d = test_diode();
        d.set_temperature(REFTEMP, REFTEMP);
        let (bare_near, bare_far) = (d.junction_capacitance(-1.0), d.junction_capacitance(-3.0));

        // XOM is divided in raw, as ngspice does; a thin oxide keeps the
        // overlap comparable to the junction capacitance so the difference
        // below is well conditioned in f64.
        d.set_overlap_capacitance(1.0, 3e-6, 4e-6, 0.0, 0.0, 1e-4, 1e-4, 0.0, 0.0, 1.0);
        let expected = EPS_SIO2 / 1e-4 * 3e-6 * 4e-6;
        assert!((d.overlap_capacitance - expected).abs() <= expected * 1e-12);

        // The same increment at both biases: the overlap does not deplete.
        let tolerance = bare_near * 1e-12;
        assert!((d.junction_capacitance(-1.0) - bare_near - expected).abs() <= tolerance);
        assert!((d.junction_capacitance(-3.0) - bare_far - expected).abs() <= tolerance);
    }

    /// SPICE diode cards are not restricted to the textbook `M < 1` range.
    /// In reverse bias the depletion law remains well defined for `M > 1`;
    /// several production cards rely on it. The former guard silently
    /// discarded the entire junction charge and capacitance for those cards.
    #[test]
    fn depletion_capacitance_supports_grading_above_one() {
        let (cj0, vj, grading, fc) = (463.53e-12, 9.99, 1.2861, 0.5);

        for vd in [0.0, -4.0, -20.0] {
            let (charge, capacitance) =
                Diode::depletion_charge_and_capacitance(vd, cj0, vj, grading, fc);
            let arg = 1.0 - vd / vj;
            let expected_capacitance = cj0 * arg.powf(-grading);
            let expected_charge = vj * cj0 * (1.0 - arg.powf(1.0 - grading)) / (1.0 - grading);

            assert!((capacitance - expected_capacitance).abs() <= cj0 * 1.0e-12);
            assert!((charge - expected_charge).abs() <= (cj0 * vj) * 1.0e-12);

            let step = 1.0e-5;
            let q_lo = Diode::depletion_charge_and_capacitance(vd - step, cj0, vj, grading, fc).0;
            let q_hi = Diode::depletion_charge_and_capacitance(vd + step, cj0, vj, grading, fc).0;
            let numerical_capacitance = (q_hi - q_lo) / (2.0 * step);
            assert!(
                (numerical_capacitance - capacitance).abs() <= capacitance * 1.0e-8,
                "dQ/dV={numerical_capacitance:e}, C={capacitance:e} at Vd={vd}"
            );
        }
    }

    /// `M=1` is the logarithmic limit of the depletion-charge integral. It
    /// needs an explicit expression to avoid the removable `0/0` singularity
    /// in both the depletion region and its forward-bias continuation.
    #[test]
    fn unit_grading_uses_the_logarithmic_charge_limit() {
        let (cj0, vj, grading, fc) = (20e-12, 0.8, 1.0, 0.5);
        let vd = -2.0;
        let arg = 1.0 - vd / vj;
        let (charge, capacitance) =
            Diode::depletion_charge_and_capacitance(vd, cj0, vj, grading, fc);

        let expected_charge = -vj * cj0 * arg.ln();
        let expected_capacitance = cj0 / arg;
        assert!((charge - expected_charge).abs() <= (cj0 * vj) * 1.0e-12);
        assert!((capacitance - expected_capacitance).abs() <= cj0 * 1.0e-12);

        for forward_bias in [fc * vj, 0.7] {
            let (forward_charge, forward_capacitance) =
                Diode::depletion_charge_and_capacitance(forward_bias, cj0, vj, grading, fc);
            assert!(forward_charge.is_finite());
            assert!(forward_capacitance.is_finite() && forward_capacitance > 0.0);
        }
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
    fn xyce_diode_limiter_matches_legacy_devicesupport_pnjlim() {
        let vt = XYCE_7_KOVERQ * REFTEMP;
        let previous = 0.7;
        let candidate = 5.0;
        let expected = previous + vt * (1.0 + (candidate - previous) / vt).ln();
        let (actual, limited) = Diode::limit_xyce_pnjlim(candidate, previous, vt, 1.0);

        assert!(limited);
        assert!((actual - expected).abs() <= 4.0 * f64::EPSILON * expected.abs().max(1.0));
    }

    #[test]
    fn xyce_diode_limiter_leaves_negative_candidates_unclamped() {
        let vt = XYCE_7_KOVERQ * REFTEMP;
        let (actual, limited) = Diode::limit_xyce_pnjlim(-2.0, 0.0, vt, 1.0);

        assert_eq!(actual, -2.0);
        assert!(!limited);
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

    #[test]
    fn xyce_level2_breakdown_voltage_applies_tbv1_and_tbv2_before_matching() {
        let mut params = std::collections::HashMap::new();
        params.insert("LEVEL".to_string(), 2.0);
        params.insert("BV".to_string(), 7.255);
        params.insert("IBV".to_string(), 1.0e-20);
        params.insert("TBV1".to_string(), 1.3e-4);
        params.insert("TBV2".to_string(), -5.0e-8);

        let mut diode = Diode::spice_defaults("d1".to_string(), 1, 2).with_model_params(&params);
        let tnom = REFTEMP;
        let temp = 273.15 - 55.0;
        diode.set_temperature_xyce_7(temp, tnom);

        let delta_t = temp - tnom;
        let expected = 7.255 * (1.0 + delta_t * (1.3e-4 - 5.0e-8 * delta_t));
        let actual = diode
            .active_breakdown_voltage()
            .expect("TBV-adjusted breakdown voltage is active");
        assert!((actual - expected).abs() <= expected * 1.0e-12);
    }

    #[test]
    fn xyce_breakdown_exponential_retains_the_full_model_exponent_range() {
        let mut params = std::collections::HashMap::new();
        params.insert("BV".to_string(), 7.255);
        params.insert("IBV".to_string(), 1.0e-3);

        let mut diode = Diode::spice_defaults("d1".to_string(), 1, 2).with_model_params(&params);
        diode.set_temperature_xyce_7(273.15 - 55.0, REFTEMP);
        let breakdown = diode
            .active_breakdown_voltage()
            .expect("matched breakdown voltage is active");
        let argument = 50.0;
        let vd = -(breakdown + argument * diode.vt);
        let expected = -diode.is * argument.exp();
        let actual = diode.current(vd);

        assert!(
            (actual - expected).abs() <= expected.abs() * 1.0e-12,
            "reverse breakdown current {actual:e} does not match Xyce exponential {expected:e}"
        );
    }

    #[test]
    fn xyce_device_minimum_defaults_only_fill_omitted_model_parameters() {
        let mut diode = Diode::spice_defaults("d1".to_string(), 1, 2);
        diode.apply_xyce_device_minimums(Some(1.0), Some(1.0e-9), false, false, false);
        assert_eq!(diode.rs, 1.0);
        assert_eq!(diode.cj0, 1.0e-9);
        assert_eq!(diode.sidewall_cj0, 1.0e-9);

        let mut explicit = Diode::spice_defaults("d2".to_string(), 1, 2);
        explicit.rs = 5.0;
        explicit.cj0 = 2.0e-12;
        explicit.sidewall_cj0 = 3.0e-12;
        explicit.apply_xyce_device_minimums(Some(1.0), Some(1.0e-9), true, true, true);
        assert_eq!(explicit.rs, 5.0);
        assert_eq!(explicit.cj0, 2.0e-12);
        assert_eq!(explicit.sidewall_cj0, 3.0e-12);
    }

    #[test]
    fn junction_gmin_contributes_to_stamped_current_and_tt_diffusion_charge() {
        let mut diode = test_diode();
        diode.cj0 = 0.0;
        diode.sidewall_cj0 = 0.0;
        diode.overlap_capacitance = 0.0;
        diode.tt = 4.0e-6;
        diode.set_junction_gmin(2.5e-3);

        let vd = 0.2;
        let (model_current, model_conductance) = diode.current_and_conductance(vd);
        let expected_stamped_current = model_current + diode.junction_gmin * vd;
        let expected_diffusion_charge = diode.tt * expected_stamped_current;
        let expected_diffusion_capacitance = diode.tt * (model_conductance + diode.junction_gmin);
        let (charge, capacitance) = diode.junction_charge_and_capacitance(vd);

        let current_tolerance = 16.0 * Value::EPSILON * expected_stamped_current.abs();
        let charge_tolerance = 16.0 * Value::EPSILON * expected_diffusion_charge.abs();
        let capacitance_tolerance = 16.0 * Value::EPSILON * expected_diffusion_capacitance.abs();
        assert!(
            (diode.stamped_conduction_current(vd) - expected_stamped_current).abs()
                <= current_tolerance,
            "stamped current omitted the junction-gmin branch"
        );
        assert!(
            (charge - expected_diffusion_charge).abs() <= charge_tolerance,
            "TT diffusion charge {charge:.17e} did not equal TT times stamped current {expected_diffusion_charge:.17e}"
        );
        assert!(
            (capacitance - expected_diffusion_capacitance).abs() <= capacitance_tolerance,
            "TT diffusion capacitance {capacitance:.17e} did not include TT*gmin {expected_diffusion_capacitance:.17e}"
        );
    }

    #[test]
    fn accepted_nonlinear_checkpoint_round_trips_without_replacing_the_device() {
        let mut source = test_diode();
        source.set_junction_gmin(3.0e-12);
        source.update(&[0.71, 0.02]);
        source.limited_linearization(0.69);
        let checkpoint = source
            .accepted_nonlinear_checkpoint()
            .expect("finite accepted diode state captures");

        let mut restored = test_diode();
        restored.update(&[-0.4, 0.3]);
        restored.node_anode = 7;
        restored
            .restore_accepted_nonlinear_checkpoint(&checkpoint)
            .expect("accepted diode state restores");

        assert_eq!(
            restored
                .accepted_nonlinear_checkpoint()
                .expect("restored state captures"),
            checkpoint
        );
        assert_eq!(
            restored.node_anode, 7,
            "restore must preserve live topology"
        );
    }
}

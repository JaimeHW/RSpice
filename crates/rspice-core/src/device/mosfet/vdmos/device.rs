use super::*;

//=============================================================================
// Stamp Indices for O(1) Matrix Access
//=============================================================================

/// Pre-computed stamp indices for O(1) matrix access
#[derive(Debug, Clone, Default)]
pub struct VdmosIndices {
    // Internal drain node (after Rd)
    pub d_d: Option<CscIndex>,
    pub d_di: Option<CscIndex>,
    // Internal source node (after Rs)
    pub s_s: Option<CscIndex>,
    pub s_si: Option<CscIndex>,
    // MOS channel stamps (between di and si)
    pub di_di: Option<CscIndex>,
    pub di_si: Option<CscIndex>,
    pub si_di: Option<CscIndex>,
    pub si_si: Option<CscIndex>,
    // Resistance stamps
    pub di_d: Option<CscIndex>,
    pub si_s: Option<CscIndex>,
    // RHS indices
    pub rhs_d: Option<usize>,
    pub rhs_s: Option<usize>,
    pub rhs_di: Option<usize>,
    pub rhs_si: Option<usize>,
}

//=============================================================================
// VDMOS Device
//=============================================================================

/// VDMOS Power MOSFET device
///
/// Terminal connections:
/// - Drain (D): External drain terminal
/// - Gate (G): Gate terminal
/// - Source (S): External source terminal
///
/// Internal nodes:
/// - Di: Internal drain (after Rd)
/// - Si: Internal source (after Rs)
#[derive(Debug, Clone)]
pub struct Vdmos {
    /// Device instance name
    pub name: String,
    /// Device type (N or P channel)
    pub vdmos_type: VdmosType,

    // Terminal node IDs
    pub drain: NodeId,
    pub gate: NodeId,
    pub source: NodeId,
    pub bulk: NodeId,

    // Internal node IDs (assigned during circuit elaboration)
    pub drain_int: Option<NodeId>,
    pub drain_drift: Option<NodeId>,
    pub source_int: Option<NodeId>,
    pub d1_prime: Option<NodeId>,

    //=========================================================================
    // Model Parameters
    //=========================================================================
    /// Threshold voltage (V)
    pub vth: Value,
    /// Transconductance coefficient (A/V²)
    pub kp: Value,
    /// Drain resistance (Ω)
    pub rd: Value,
    /// Source resistance (Ω)
    pub rs: Value,
    /// Gate resistance (Ω)
    pub rg: Value,
    /// Channel length modulation (V⁻¹)
    pub lambda: Value,
    /// Triode region exponent
    pub mtriode: Value,
    /// Quasi-saturation resistance (Ω)
    pub rq: Value,
    /// Quasi-saturation onset voltage (V)
    pub vq: Value,
    /// Velocity-saturation critical voltage derived from VMAX, UO, and L.
    pub velocity_saturation_voltage: Value,
    /// Use Xyce's VDMOS LEVEL=18 UCCM current equations.
    pub xyce_level18: bool,
    /// Effective device width used by the Xyce/UCCM model (m), including instance M.
    pub xyce_width: Value,
    /// Effective channel length used by the Xyce/UCCM model (m).
    pub xyce_length: Value,
    /// Surface mobility at the active temperature (m^2/V/s).
    pub xyce_surface_mobility: Value,
    /// Gate oxide thickness (m).
    pub xyce_oxide_thickness: Value,
    /// Xyce/UCCM subthreshold ideality factor.
    pub xyce_eta: Value,
    /// Xyce/UCCM transition width parameter.
    pub xyce_delta: Value,
    /// Xyce/UCCM DIBL coefficient.
    pub xyce_sigma0: Value,
    /// Xyce/UCCM DIBL transition voltage.
    pub xyce_vsigmat: Value,
    /// Xyce/UCCM DIBL smoothing voltage.
    pub xyce_vsigma: Value,
    /// Xyce/UCCM mobility degradation coefficient.
    pub xyce_theta: Value,
    /// Xyce/UCCM maximum drift velocity (m/s).
    pub xyce_max_drift_velocity: Value,
    /// Xyce drain-drift resistance intercept (ohms).
    pub xyce_drift_param_a: Value,
    /// Xyce drain-drift resistance slope (ohms/volt).
    pub xyce_drift_param_b: Value,
    /// Xyce body-effect surface potential.
    pub xyce_phi: Value,
    /// Xyce body-effect square-root coefficient after geometry scaling.
    pub xyce_gammas: Value,
    /// Xyce body-effect linear coefficient after geometry scaling.
    pub xyce_gammal: Value,

    //=========================================================================
    // Nonlinear Capacitance Parameters
    // Uses junction capacitance model: C = C0 / (1 - V/Pb)^M
    //=========================================================================
    /// Zero-bias gate-source capacitance (F)
    pub cgs0: Value,
    /// Zero-bias gate-drain (Miller) capacitance (F)
    pub cgd0: Value,
    /// Gate-source junction potential (V)
    pub cgs_pb: Value,
    /// Gate-drain junction potential (V)
    pub cgd_pb: Value,
    /// Gate-source grading coefficient
    pub cgs_m: Value,
    /// Gate-drain grading coefficient (typically 0.5 for abrupt, 0.33 for graded)
    pub cgd_m: Value,
    /// Forward bias coefficient (capacitance model limit)
    pub fc: Value,
    /// Drain-source capacitance (F) - typically fixed
    pub cds: Value,
    /// Zero-bias bulk-source p-n capacitance (F).
    pub cbs0: Value,
    /// Zero-bias bulk-drain p-n capacitance (F).
    pub cbd0: Value,
    /// True when CBS was explicitly provided on the model card.
    pub cbs_given: bool,
    /// True when CBD was explicitly provided on the model card.
    pub cbd_given: bool,
    /// Bulk bottom junction zero-bias capacitance density.
    pub body_cj: Value,
    /// Bulk sidewall junction zero-bias capacitance density.
    pub body_cjsw: Value,
    /// Effective drain diffusion area for geometry-derived CBD.
    pub drain_area: Value,
    /// Effective source diffusion area for geometry-derived CBS.
    pub source_area: Value,
    /// Effective drain diffusion perimeter for geometry-derived sidewall CBD.
    pub drain_perimeter: Value,
    /// Effective source diffusion perimeter for geometry-derived sidewall CBS.
    pub source_perimeter: Value,
    /// Bulk junction built-in potential (V).
    pub body_pb: Value,
    /// Bulk junction grading coefficient.
    pub body_m: Value,
    /// Bulk sidewall junction grading coefficient.
    pub body_mjsw: Value,
    /// Xyce/ngspice gate-source overlap capacitance after geometry scaling (F).
    pub cgs_overlap: Value,
    /// Xyce/ngspice gate-drain overlap capacitance after geometry scaling (F).
    pub cgd_overlap: Value,
    /// Xyce gate-bulk overlap capacitance after effective-length scaling (F).
    pub cgb_overlap: Value,
    /// D1 drain-source diode saturation current (A).
    pub d1_is: Value,
    /// D1 drain-source diode ohmic series resistance (ohms).
    pub d1_rs: Value,
    /// D1 drain-source diode emission coefficient.
    pub d1_n: Value,
    /// D1 drain-source diode recombination saturation current (A).
    pub d1_isr: Value,
    /// D1 drain-source diode recombination emission coefficient.
    pub d1_nr: Value,
    /// D1 drain-source diode high-injection knee current (A).
    pub d1_ikf: Value,
    /// D1 drain-source diode activation energy (eV).
    pub d1_eg: Value,
    /// D1 drain-source diode saturation-current temperature exponent.
    pub d1_xti: Value,
    /// Active D1 drain-source diode nominal temperature (K), sourced from TNOM.
    pub d1_tnom_kelvin: Value,
    /// Active D1 drain-source diode temperature (K).
    pub d1_temperature_kelvin: Value,
    /// D1 drain-source diode reverse breakdown voltage (V).
    pub d1_bv: Value,
    /// D1 drain-source diode current at the specified breakdown voltage (A).
    pub d1_ibv: Value,
    /// True when D1BV was explicitly supplied on the model card.
    pub d1_bv_given: bool,
    /// D1 drain-source diode transit time (s).
    pub d1_tt: Value,
    /// D1 drain-source diode zero-bias junction capacitance (F).
    pub d1_cjo: Value,
    /// D1 drain-source diode junction potential (V).
    pub d1_vj: Value,
    /// D1 drain-source diode grading coefficient.
    pub d1_m: Value,
    /// D1 drain-source diode forward-bias depletion coefficient.
    pub d1_fc: Value,
    /// True when a D1 current or diffusion-charge parameter was provided.
    pub d1_current_enabled: bool,

    // Body diode parameters
    /// Saturation current (A)
    pub is: Value,
    /// Ideality factor
    pub n: Value,
    /// Transit time (s)
    pub tt: Value,
    /// Breakdown voltage (V)
    pub bv: Value,

    //=========================================================================
    // Thermal Model (Self-Heating)
    //=========================================================================
    /// Thermal network for electro-thermal simulation
    pub thermal: ThermalNetwork,

    //=========================================================================
    // Body Diode Reverse Recovery
    //=========================================================================
    /// Diode recovery model for switching transients
    pub recovery: DiodeRecovery,

    //=========================================================================
    // Operating State
    //=========================================================================
    /// Current operating region
    pub region: VdmosRegion,
    /// Drain current
    pub id: Value,
    /// Body diode current
    pub id_diode: Value,
    /// Instantaneous power dissipation (W)
    pub power: Value,
    /// Previous voltages for convergence
    prev_vgs: Value,
    prev_vds: Value,
    prev_vbs: Value,
    vgs_prev: Value,
    vds_prev: Value,
    vbs_prev: Value,
    eval_vgs: Value,
    eval_vds: Value,
    eval_vbs: Value,
    eval_vgs_prev: Value,
    eval_vds_prev: Value,
    eval_vbs_prev: Value,
    id_prev: Value,
    gm: Value,
    gds: Value,
    gmb: Value,
    gm_prev: Value,
    gds_prev: Value,
    gmb_prev: Value,
    id_eq: Value,
    limiter_applied: bool,
    has_branch_history: bool,

    /// Pre-computed stamp indices
    indices: VdmosIndices,
}

impl Vdmos {
    /// Create a new N-channel VDMOS
    pub fn new_nvdmos(name: String, drain: NodeId, gate: NodeId, source: NodeId) -> Self {
        Self::new(name, VdmosType::NVdmos, drain, gate, source)
    }

    /// Create a new P-channel VDMOS
    pub fn new_pvdmos(name: String, drain: NodeId, gate: NodeId, source: NodeId) -> Self {
        Self::new(name, VdmosType::PVdmos, drain, gate, source)
    }

    /// Create a new VDMOS with specified type
    pub fn new(
        name: String,
        vdmos_type: VdmosType,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
    ) -> Self {
        Self {
            name,
            vdmos_type,
            drain,
            gate,
            source,
            bulk: source,
            drain_int: None,
            drain_drift: None,
            source_int: None,
            d1_prime: None,

            // Default power MOSFET parameters (typical values)
            vth: 2.0,
            kp: 2.0,
            rd: 0.1,
            rs: 0.01,
            rg: 1.0,
            lambda: 0.01,
            mtriode: 1.5,
            rq: 0.0,
            vq: 5.0,
            velocity_saturation_voltage: Value::INFINITY,
            xyce_level18: false,
            xyce_width: 1.0,
            xyce_length: 1.0,
            xyce_surface_mobility: 280.0e-4,
            xyce_oxide_thickness: 1.0e-7,
            xyce_eta: 1.32,
            xyce_delta: 5.0,
            xyce_sigma0: 0.048,
            xyce_vsigmat: 1.7,
            xyce_vsigma: 0.2,
            xyce_theta: 0.0,
            xyce_max_drift_velocity: 4.0e4,
            xyce_drift_param_a: 0.08,
            xyce_drift_param_b: 0.013,
            xyce_phi: 0.6,
            xyce_gammas: 0.5,
            xyce_gammal: 0.0,

            // Nonlinear capacitance defaults (typical power MOSFET)
            cgs0: 1e-9,    // 1nF zero-bias Cgs
            cgd0: 100e-12, // 100pF zero-bias Cgd (Miller)
            cgs_pb: 0.8,   // Gate-source junction potential
            cgd_pb: 0.8,   // Gate-drain junction potential
            cgs_m: 0.5,    // Grading coefficient
            cgd_m: 0.5,    // Grading coefficient
            fc: 0.5,       // Forward bias limit coefficient
            cds: 50e-12,   // Drain-source capacitance (fixed)
            cbs0: 0.0,
            cbd0: 0.0,
            cbs_given: false,
            cbd_given: false,
            body_cj: 0.0,
            body_cjsw: 0.0,
            drain_area: 0.0,
            source_area: 0.0,
            drain_perimeter: 0.0,
            source_perimeter: 0.0,
            body_pb: 0.8,
            body_m: 0.5,
            body_mjsw: 0.5,
            cgs_overlap: 0.0,
            cgd_overlap: 0.0,
            cgb_overlap: 0.0,
            d1_is: 1.0e-14,
            d1_rs: 0.0,
            d1_n: 1.0,
            d1_isr: 0.0,
            d1_nr: 2.0,
            d1_ikf: 0.0,
            d1_eg: 1.11,
            d1_xti: 3.0,
            d1_tnom_kelvin: 300.15,
            d1_temperature_kelvin: 300.15,
            d1_bv: 1.0e99,
            d1_ibv: 1.0e-3,
            d1_bv_given: false,
            d1_tt: 0.0,
            d1_cjo: 0.0,
            d1_vj: 1.0,
            d1_m: 0.5,
            d1_fc: 0.5,
            d1_current_enabled: false,

            is: 1e-14,
            n: 1.5,
            tt: 50e-9,
            bv: 100.0,

            // Thermal and recovery models (disabled by default)
            thermal: ThermalNetwork::default(),
            recovery: DiodeRecovery::default(),

            region: VdmosRegion::Cutoff,
            id: 0.0,
            id_diode: 0.0,
            power: 0.0,
            prev_vgs: 0.0,
            prev_vds: 0.0,
            prev_vbs: 0.0,
            vgs_prev: Value::NAN,
            vds_prev: Value::NAN,
            vbs_prev: Value::NAN,
            eval_vgs: Value::NAN,
            eval_vds: Value::NAN,
            eval_vbs: Value::NAN,
            eval_vgs_prev: Value::NAN,
            eval_vds_prev: Value::NAN,
            eval_vbs_prev: Value::NAN,
            id_prev: Value::NAN,
            gm: Value::NAN,
            gds: Value::NAN,
            gmb: Value::NAN,
            gm_prev: Value::NAN,
            gds_prev: Value::NAN,
            gmb_prev: Value::NAN,
            id_eq: Value::NAN,
            limiter_applied: false,
            has_branch_history: false,

            indices: VdmosIndices::default(),
        }
    }

    /// Set model parameters from a parameter map
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let is_xyce_level18 = params
            .get("LEVEL")
            .is_some_and(|level| level.is_finite() && (*level - 18.0).abs() <= 1.0e-9);
        if is_xyce_level18 {
            self.xyce_level18 = true;
            // Xyce LEVEL=18 defaults differ from RSpice's generic VDMOS
            // defaults; honor the UCCM model-card baseline before overrides.
            self.lambda = 0.048;
            self.cgs0 = 0.0;
            self.cgd0 = 0.0;
            self.cds = 0.0;
        }

        if let Some(&v) = params.get("VTH") {
            self.vth = self.normalized_threshold(v);
        }
        if let Some(&v) = params.get("VTO") {
            self.vth = self.normalized_threshold(v);
        } // Alias
        if let Some(&v) = params.get("KP") {
            self.kp = v;
        }
        if let Some(&v) = params.get("RD") {
            self.rd = v;
        }
        if let Some(&v) = params.get("RS") {
            self.rs = v;
        }
        if let Some(&v) = params.get("RG") {
            self.rg = v;
        }
        if let Some(&v) = params.get("LAMBDA") {
            self.lambda = v;
        }
        if let Some(&v) = params.get("MTRIODE") {
            self.mtriode = v;
        }
        if let Some(&v) = params.get("RQ") {
            self.rq = v;
        }
        if let Some(&v) = params.get("VQ") {
            self.vq = v;
        }
        if let Some(&v) = params.get("VSATV") {
            self.velocity_saturation_voltage = v;
        }
        if let Some(&v) = params.get("TOX")
            && v.is_finite()
            && v > 0.0
        {
            self.xyce_oxide_thickness = v;
        }
        if let Some(&v) = params.get("UO").or_else(|| params.get("U0"))
            && v.is_finite()
            && v > 0.0
        {
            self.xyce_surface_mobility = v * 1.0e-4;
        }
        if let Some(&v) = params.get("VMAX").or_else(|| params.get("VSAT"))
            && v.is_finite()
            && v > 0.0
        {
            self.xyce_max_drift_velocity = v;
        }
        if let Some(&v) = params.get("ETA")
            && v.is_finite()
            && v > 0.0
        {
            self.xyce_eta = v;
        }
        if let Some(&v) = params.get("DELTA")
            && v.is_finite()
            && v >= 0.0
        {
            self.xyce_delta = v;
        }
        if let Some(&v) = params.get("SIGMA0")
            && v.is_finite()
        {
            self.xyce_sigma0 = v;
        }
        if let Some(&v) = params.get("VSIGMAT")
            && v.is_finite()
        {
            self.xyce_vsigmat = v;
        }
        if let Some(&v) = params.get("VSIGMA")
            && v.is_finite()
            && v > 0.0
        {
            self.xyce_vsigma = v;
        }
        if let Some(&v) = params.get("THETA")
            && v.is_finite()
        {
            self.xyce_theta = v;
        }
        if let Some(&v) = params.get("DRIFTPARAMA")
            && v.is_finite()
            && v >= 0.0
        {
            self.xyce_drift_param_a = v;
        }
        if let Some(&v) = params.get("DRIFTPARAMB")
            && v.is_finite()
            && v >= 0.0
        {
            self.xyce_drift_param_b = v;
        }
        if let Some(&v) = params.get("PHI")
            && v.is_finite()
            && v > 0.0
        {
            self.xyce_phi = v;
        }
        // Capacitance parameters - support both old and new names
        if let Some(&v) = params.get("CGS") {
            self.cgs0 = v;
        }
        if let Some(&v) = params.get("CGS0") {
            self.cgs0 = v;
        }
        if let Some(&v) = params.get("CGD") {
            self.cgd0 = v;
        }
        if let Some(&v) = params.get("CGD0") {
            self.cgd0 = v;
        }
        if let Some(&v) = params.get("CGSO") {
            self.cgs_overlap = v;
        }
        if let Some(&v) = params.get("CGDO") {
            self.cgd_overlap = v;
        }
        if let Some(&v) = params.get("CGBO") {
            self.cgb_overlap = v;
        }
        if let Some(&v) = params.get("PB") {
            self.cgs_pb = v;
            self.cgd_pb = v;
            self.body_pb = v;
        }
        if let Some(&v) = params.get("CGSPB") {
            self.cgs_pb = v;
        }
        if let Some(&v) = params.get("CGDPB") {
            self.cgd_pb = v;
        }
        if let Some(&v) = params.get("M")
            && !is_xyce_level18
        {
            self.cgs_m = v;
            self.cgd_m = v;
        }
        if let Some(&v) = params.get("CGSM") {
            self.cgs_m = v;
        }
        if let Some(&v) = params.get("CGDM") {
            self.cgd_m = v;
        }
        if let Some(&v) = params.get("FC") {
            self.fc = v;
        }
        if let Some(&v) = params.get("CDS") {
            self.cds = v;
        }
        if let Some(&v) = params.get("CBS") {
            self.cbs0 = v;
            self.cbs_given = true;
        }
        if let Some(&v) = params.get("CBD") {
            self.cbd0 = v;
            self.cbd_given = true;
        }
        if let Some(&v) = params.get("CJ") {
            self.body_cj = v;
        }
        if let Some(&v) = params.get("CJSW") {
            self.body_cjsw = v;
        }
        if let Some(&v) = params.get("MJ") {
            self.body_m = v;
        }
        if let Some(&v) = params.get("MJSW") {
            self.body_mjsw = v;
        }
        if let Some(&v) = params.get("D1IS") {
            self.d1_is = v;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1RS") {
            self.d1_rs = v.max(0.0);
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1N") {
            self.d1_n = v;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1ISR") {
            self.d1_isr = v;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1NR") {
            self.d1_nr = v;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1IKF") {
            self.d1_ikf = v;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1EG") {
            self.d1_eg = v.max(0.1);
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1XTI") {
            self.d1_xti = v;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1BV") {
            self.d1_bv = v;
            self.d1_bv_given = true;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1IBV") {
            self.d1_ibv = v;
            self.d1_current_enabled = true;
        }
        if let Some(&v) = params.get("D1TT") {
            self.d1_tt = v;
            if v != 0.0 {
                self.d1_current_enabled = true;
            }
        }
        if let Some(&v) = params.get("D1CJO") {
            self.d1_cjo = v;
        }
        if let Some(&v) = params.get("D1VJ") {
            self.d1_vj = v;
        }
        if let Some(&v) = params.get("D1M") {
            self.d1_m = v.min(0.9);
        }
        if let Some(&v) = params.get("D1FC") {
            self.d1_fc = v.min(0.95);
        }
        if let Some(&v) = params.get("IS") {
            self.is = v;
        }
        if let Some(&v) = params.get("N") {
            self.n = v;
        }
        if let Some(&v) = params.get("TT") {
            self.tt = v;
        }
        if let Some(&v) = params.get("BV") {
            self.bv = v;
        }

        // Thermal model parameters
        if let Some(&v) = params.get("RTH") {
            self.thermal.rth = v;
        }
        if let Some(&v) = params.get("CTH") {
            self.thermal.cth = v;
        }
        if let Some(&v) = params.get("TAMB") {
            // Convert Celsius to Kelvin
            self.thermal.t_ambient = v + 273.15;
            self.thermal.t_junction = self.thermal.t_ambient;
        }

        // Body diode reverse recovery parameters
        if let Some(&v) = params.get("QRR") {
            self.recovery.qrr = v;
        }
        if let Some(&v) = params.get("TRR") {
            self.recovery.trr = v;
        }
        if let Some(&v) = params.get("SOFTNESS") {
            self.recovery.softness = v.clamp(0.0, 1.0);
        }

        self
    }

    /// Set instance-dependent sizing parameters.
    ///
    /// Xyce VDMOS Level 18 commonly specifies drive strength through
    /// `UO`, `TOX`, and the instance `W/L` ratio rather than an explicit
    /// SPICE `KP`. Translate that physical form to the native channel
    /// transconductance while still honoring an explicitly supplied `KP`.
    pub fn with_instance_params(
        mut self,
        model_params: &std::collections::HashMap<String, Value>,
        instance_params: &[(String, Value)],
    ) -> Self {
        fn instance_value(params: &[(String, Value)], name: &str) -> Option<Value> {
            params.iter().find_map(|(candidate, value)| {
                candidate.eq_ignore_ascii_case(name).then_some(*value)
            })
        }

        let width = instance_value(instance_params, "W")
            .or_else(|| instance_value(instance_params, "WIDTH"))
            .or_else(|| model_params.get("W0").copied().filter(|v| *v > 0.0))
            .unwrap_or(1.0);
        let length = instance_value(instance_params, "L")
            .or_else(|| instance_value(instance_params, "LENGTH"))
            .or_else(|| model_params.get("L0").copied().filter(|v| *v > 0.0))
            .unwrap_or(1.0);
        let multiplier = instance_value(instance_params, "M")
            .or_else(|| instance_value(instance_params, "MULT"))
            .or_else(|| instance_value(instance_params, "PARALLEL"))
            .filter(|value| value.is_finite() && *value > 0.0)
            .unwrap_or(1.0);
        let source_area = instance_value(instance_params, "AS")
            .filter(|value| value.is_finite() && *value >= 0.0)
            .unwrap_or(0.0);
        let drain_area = instance_value(instance_params, "AD")
            .filter(|value| value.is_finite() && *value >= 0.0)
            .unwrap_or(0.0);
        let source_perimeter = instance_value(instance_params, "PS")
            .filter(|value| value.is_finite() && *value >= 0.0)
            .unwrap_or(0.0);
        let drain_perimeter = instance_value(instance_params, "PD")
            .filter(|value| value.is_finite() && *value >= 0.0)
            .unwrap_or(0.0);

        if model_params.contains_key("UO")
            || model_params.contains_key("U0")
            || model_params.contains_key("TOX")
            || model_params.contains_key("VMAX")
            || model_params.contains_key("VSAT")
        {
            self.xyce_level18 = true;
        }

        let lateral_diffusion = model_params.get("LD").copied().unwrap_or(0.0);
        let effective_length = (length - 2.0 * lateral_diffusion).max(1.0e-12);
        if width.is_finite() && width > 0.0 {
            self.xyce_width = width * multiplier;
        }
        if effective_length.is_finite() && effective_length > 0.0 {
            self.xyce_length = effective_length;
        }
        let l0 = model_params.get("L0").copied().unwrap_or(0.0);
        let w0 = model_params.get("W0").copied().unwrap_or(0.0);
        let gammas0 = model_params
            .get("GAMMAS0")
            .copied()
            .or_else(|| {
                model_params.get("NSUB").copied().and_then(|nsub| {
                    Self::xyce_derived_gammas0_from_nsub(nsub, self.xyce_oxide_thickness)
                })
            })
            .unwrap_or(0.5);
        let gammal0 = model_params.get("GAMMAL0").copied().unwrap_or(0.0);
        let lgammas = model_params.get("LGAMMAS").copied().unwrap_or(0.0);
        let wgammas = model_params.get("WGAMMAS").copied().unwrap_or(0.0);
        let lgammal = model_params.get("LGAMMAL").copied().unwrap_or(0.0);
        let wgammal = model_params.get("WGAMMAL").copied().unwrap_or(0.0);
        self.xyce_gammas = gammas0
            + lgammas * (1.0 - l0 / effective_length)
            + wgammas * (1.0 - w0 / width.max(1.0e-12));
        self.xyce_gammal = gammal0
            + lgammal * (1.0 - l0 / effective_length)
            + wgammal * (1.0 - w0 / width.max(1.0e-12));

        if !model_params.contains_key("KP") {
            let mobility_cm2 = model_params
                .get("UO")
                .or_else(|| model_params.get("U0"))
                .copied();
            let tox = model_params.get("TOX").copied();
            if let (Some(mobility_cm2), Some(tox)) = (mobility_cm2, tox)
                && mobility_cm2.is_finite()
                && mobility_cm2 > 0.0
                && tox.is_finite()
                && tox > 0.0
                && width.is_finite()
                && width > 0.0
                && length.is_finite()
                && length > 0.0
            {
                const EPS_OX: Value = 3.453_133e-11;
                let mobility_m2 = mobility_cm2 * 1.0e-4;
                self.kp = mobility_m2 * EPS_OX / tox * (width / effective_length);
            }
        }

        let mobility_cm2 = model_params
            .get("UO")
            .or_else(|| model_params.get("U0"))
            .copied();
        let max_drift_velocity = model_params
            .get("VMAX")
            .or_else(|| model_params.get("VSAT"))
            .copied();
        if let (Some(mobility_cm2), Some(max_drift_velocity)) = (mobility_cm2, max_drift_velocity)
            && mobility_cm2.is_finite()
            && mobility_cm2 > 0.0
            && max_drift_velocity.is_finite()
            && max_drift_velocity > 0.0
            && effective_length.is_finite()
            && effective_length > 0.0
        {
            let mobility_m2 = mobility_cm2 * 1.0e-4;
            self.velocity_saturation_voltage = max_drift_velocity * effective_length / mobility_m2;
        }

        if multiplier > 0.0 {
            self.kp *= multiplier;
            self.cgs0 *= multiplier;
            self.cgd0 *= multiplier;
            self.cds *= multiplier;
            self.cbs0 *= multiplier;
            self.cbd0 *= multiplier;
            if !self.xyce_level18 {
                self.d1_is *= multiplier;
                self.d1_isr *= multiplier;
                self.d1_ikf *= multiplier;
                self.d1_cjo *= multiplier;
            }
            self.is *= multiplier;
            self.source_area = source_area * multiplier;
            self.drain_area = drain_area * multiplier;
            self.source_perimeter = source_perimeter * multiplier;
            self.drain_perimeter = drain_perimeter * multiplier;
            self.cgs_overlap *= width.max(0.0) * multiplier;
            self.cgd_overlap *= width.max(0.0) * multiplier;
            self.cgb_overlap *= effective_length.max(0.0) * multiplier;
        }

        self
    }

    /// Set internal node IDs (called during circuit elaboration)
    pub fn set_internal_nodes(&mut self, drain_int: NodeId, source_int: NodeId) {
        self.drain_int = Some(drain_int);
        self.source_int = Some(source_int);
    }

    pub fn set_drain_drift_node(&mut self, drain_drift: NodeId) {
        self.drain_drift = Some(drain_drift);
    }

    pub fn set_d1_prime_node(&mut self, d1_prime: NodeId) {
        self.d1_prime = Some(d1_prime);
    }

    pub fn set_temperature(
        &mut self,
        temperature_kelvin: Value,
        nominal_temperature_kelvin: Value,
    ) {
        if temperature_kelvin.is_finite() && temperature_kelvin > 0.0 {
            self.d1_temperature_kelvin = temperature_kelvin;
        }
        if nominal_temperature_kelvin.is_finite() && nominal_temperature_kelvin > 0.0 {
            self.d1_tnom_kelvin = nominal_temperature_kelvin;
        }
    }

    pub fn set_bulk_node(&mut self, bulk: NodeId) {
        self.bulk = bulk;
    }

    /// Get polarity multiplier (+1 for N-channel, -1 for P-channel)
    #[inline]
    pub fn polarity(&self) -> Value {
        match self.vdmos_type {
            VdmosType::NVdmos => 1.0,
            VdmosType::PVdmos => -1.0,
        }
    }

    #[inline]
    fn normalized_threshold(&self, value: Value) -> Value {
        match self.vdmos_type {
            VdmosType::NVdmos => value,
            VdmosType::PVdmos => value.abs(),
        }
    }

    /// Operating-point quantities cached by the last accepted update.
    pub fn op_values(&self) -> (Value, Value, Value, Value, Value, &'static str) {
        (
            self.id,
            self.prev_vgs,
            self.prev_vds,
            self.id_diode,
            self.power,
            Self::region_name(self.region),
        )
    }

    fn region_name(region: VdmosRegion) -> &'static str {
        match region {
            VdmosRegion::Cutoff => "cutoff",
            VdmosRegion::Triode => "triode",
            VdmosRegion::Saturation => "saturation",
            VdmosRegion::QuasiSaturation => "quasi-saturation",
            VdmosRegion::BodyDiode => "body-diode",
        }
    }

    #[inline]
    fn limited_branch_voltages_for_eval(&self, vgs: Value, vds: Value) -> (Value, Value, bool) {
        (vgs, vds, false)
    }

    fn xyce_derived_gammas0_from_nsub(nsub_cm3: Value, oxide_thickness: Value) -> Option<Value> {
        const CONST_Q: Value = 1.602_191_8e-19;
        const EPS0: Value = 8.854_214_871e-12;
        const EPS_SI_REL: Value = 11.70;
        const EPS_OX_REL: Value = 3.9;
        const MIN_NSUB_M3: Value = 1.45e16;

        if !nsub_cm3.is_finite() || !oxide_thickness.is_finite() || oxide_thickness <= 0.0 {
            return None;
        }
        let nsub_m3 = nsub_cm3 * 1.0e6;
        if nsub_m3 <= MIN_NSUB_M3 {
            return None;
        }

        let oxide_cap_factor = EPS_OX_REL * EPS0 / oxide_thickness;
        Some((2.0 * EPS_SI_REL * EPS0 * CONST_Q * nsub_m3).sqrt() / oxide_cap_factor)
    }

    #[inline]
    fn xyce_drift_enabled(&self) -> bool {
        self.xyce_level18 && (self.xyce_drift_param_a > 0.0 || self.xyce_drift_param_b > 0.0)
    }

    #[inline]
    fn drain_drift_node(&self) -> NodeId {
        if self.xyce_drift_enabled() {
            self.drain_drift
                .unwrap_or_else(|| self.drain_int.unwrap_or(self.drain))
        } else {
            self.drain
        }
    }

    #[inline]
    fn uses_topological_drift(&self) -> bool {
        self.xyce_drift_enabled() && self.drain_drift_node() != self.drain
    }

    #[inline]
    fn xyce_drift_current_and_conductance(&self, vdrop: Value) -> (Value, Value) {
        let resistance =
            (self.xyce_drift_param_a + self.xyce_drift_param_b * vdrop.abs()).max(1.0e-30);
        let conductance = 1.0 / resistance;
        let current = conductance * vdrop;
        let d_abs = if vdrop > 0.0 {
            1.0
        } else if vdrop < 0.0 {
            -1.0
        } else {
            0.0
        };
        let derivative =
            conductance - vdrop * conductance * conductance * self.xyce_drift_param_b * d_abs;
        (current, derivative.max(1.0e-12))
    }

    #[inline]
    fn linearized_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, VdmosRegion, Value, Value, Value, Value) {
        let (id, region) = self.calculate_id_with_body(vgs, vds, vbs);
        let gm = self.gm(vgs, vds, vbs);
        let gds = self.gds(vgs, vds, vbs);
        let gmb = self.gmb(vgs, vds, vbs);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
        (id, region, gm, gds, gmb, id_eq)
    }

    fn xyce_level18_von(&self, vbs_eff: Value) -> Value {
        let phi = self.xyce_phi.max(1.0e-12);
        let phi_min_vbs = phi - vbs_eff;
        let sarg = if phi_min_vbs > 0.0 {
            phi_min_vbs.sqrt()
        } else {
            0.0
        };
        let vtoo = self.vth + self.xyce_gammal * phi - self.xyce_gammas * phi.sqrt();
        vtoo + self.xyce_gammas * sarg - self.xyce_gammal * phi_min_vbs
    }

    fn calculate_id_xyce_level18(
        &self,
        vgs_eff: Value,
        vds_eff: Value,
        vbs_eff: Value,
    ) -> (Value, VdmosRegion) {
        let (xvgs, xvdds, xvbs, mode) = if vds_eff >= 0.0 {
            (vgs_eff, vds_eff, vbs_eff, 1.0)
        } else {
            (vgs_eff - vds_eff, -vds_eff, vbs_eff - vds_eff, -1.0)
        };

        let (id, region) = if self.uses_topological_drift() {
            self.calculate_id_xyce_level18_forward(xvgs, xvdds, xvbs)
        } else {
            self.calculate_id_xyce_level18_with_drift(xvgs, xvdds, xvbs)
        };
        (mode * id, region)
    }

    fn calculate_id_xyce_level18_with_drift(
        &self,
        xvgs: Value,
        xvdds: Value,
        xvbs: Value,
    ) -> (Value, VdmosRegion) {
        let drift_a = self.xyce_drift_param_a.max(0.0);
        let drift_b = self.xyce_drift_param_b.max(0.0);
        if xvdds <= 0.0 || (drift_a == 0.0 && drift_b == 0.0) {
            return self.calculate_id_xyce_level18_forward(xvgs, xvdds, xvbs);
        }

        let (intrinsic_id, intrinsic_region) =
            self.calculate_id_xyce_level18_forward(xvgs, xvdds, xvbs);
        if intrinsic_id <= 0.0 {
            return (intrinsic_id, intrinsic_region);
        }

        let resistor_limited = xvdds / (drift_a + drift_b * xvdds).max(1.0e-30);
        let mut lo = 0.0;
        let mut hi = intrinsic_id.min(resistor_limited).max(0.0);
        if hi <= 0.0 {
            return (0.0, VdmosRegion::Cutoff);
        }

        let current_to_channel_vds = |current: Value| -> Value {
            let denominator = (1.0 - drift_b * current).max(1.0e-12);
            let drift_drop = current * drift_a / denominator;
            (xvdds - drift_drop).max(0.0)
        };

        for _ in 0..80 {
            let mid = 0.5 * (lo + hi);
            let channel_vds = current_to_channel_vds(mid);
            let (channel_id, _) = self.calculate_id_xyce_level18_forward(xvgs, channel_vds, xvbs);
            if mid <= channel_id {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        let id = 0.5 * (lo + hi);
        let channel_vds = current_to_channel_vds(id);
        let (_, region) = self.calculate_id_xyce_level18_forward(xvgs, channel_vds, xvbs);
        (id, region)
    }

    fn calculate_id_xyce_level18_forward(
        &self,
        xvgs: Value,
        xvdds: Value,
        xvbs: Value,
    ) -> (Value, VdmosRegion) {
        const CONST_Q: Value = 1.602_191_8e-19;
        const CONST_BOLTZ: Value = 1.380_622_6e-23;
        const CONST_REF_TEMP: Value = 300.15;
        const CONST_EPS_OX: Value = 3.453_133e-11;
        const EXP_LIMIT: Value = 150.0;

        let vt = (CONST_BOLTZ / CONST_Q) * CONST_REF_TEMP;
        let eta = self.xyce_eta.max(1.0e-12);
        let etavt = eta * vt;
        let tox = self.xyce_oxide_thickness.max(1.0e-12);
        let length = self.xyce_length.max(1.0e-12);
        let width = self.xyce_width.max(0.0);
        let surface_mobility = self.xyce_surface_mobility.max(1.0e-12);
        let vsigma = self.xyce_vsigma.max(1.0e-12);

        let von = self.xyce_level18_von(xvbs);
        let vgt0 = xvgs - von;
        let dibl_exp = ((vgt0 - self.xyce_vsigmat) / vsigma).clamp(-EXP_LIMIT, EXP_LIMIT);
        let sigma = self.xyce_sigma0 / (1.0 + dibl_exp.exp());
        let vgt = vgt0 + sigma * xvdds;
        let b = 0.5 * vgt / vt - 1.0;
        let q = (self.xyce_delta * self.xyce_delta + b * b).sqrt();
        let vgte = vt * (2.0 + b + q);
        let mobility_denom = (1.0 + self.xyce_theta * (vgte + 2.0 * von) / tox).max(1.0e-12);
        let mobility = surface_mobility / mobility_denom;

        let x = vgt / etavt;
        let n0 = CONST_EPS_OX * eta * vt / (2.0 * CONST_Q * tox);
        let ns = if x > 50.0 {
            n0 * 2.0 * x
        } else if x < -30.0 {
            n0 * x.exp()
        } else {
            2.0 * n0 * (1.0 + 0.5 * x.exp()).ln()
        };

        if ns < 1.0e-38 || width <= 0.0 {
            return (0.0, VdmosRegion::Cutoff);
        }

        let gchi0 = CONST_Q * width / length;
        let gchi = gchi0 * mobility * ns;
        let rt = (self.rs + self.rd).max(0.0);
        let gch_denom = 1.0 + gchi * rt;
        if gch_denom <= 0.0 {
            return (0.0, VdmosRegion::Cutoff);
        }
        let gch = gchi / gch_denom;
        if gch <= 1.0e-30 {
            return (0.0, VdmosRegion::Cutoff);
        }

        let vl = self.xyce_max_drift_velocity.max(1.0e-12) * length / surface_mobility;
        let vl2 = vl * vl;
        let d = (1.0 + 2.0 * gchi * self.rs.max(0.0) + vgte * vgte / vl2).sqrt();
        let h = 1.0 + gchi * self.rs.max(0.0) + d;
        let isat = gchi * vgte / h;
        let vsate = isat / gch;
        if vsate <= 1.0e-30 {
            return (0.0, VdmosRegion::Cutoff);
        }

        let y = xvdds / vsate;
        let tanh_y = if y.abs() > EXP_LIMIT {
            y.signum()
        } else {
            y.tanh()
        };
        let id = isat * (1.0 + self.lambda * xvdds) * tanh_y;

        let region = if vgt0 <= 0.0 {
            VdmosRegion::Cutoff
        } else if xvdds < vsate {
            VdmosRegion::Triode
        } else {
            VdmosRegion::Saturation
        };

        (id, region)
    }

    /// Calculate drain current and region for given terminal voltages
    pub fn calculate_id(&self, vgs: Value, vds: Value) -> (Value, VdmosRegion) {
        self.calculate_id_with_body(vgs, vds, 0.0)
    }

    fn calculate_id_with_body(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, VdmosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;
        let vbs_eff = p * vbs;

        if self.xyce_level18 {
            let (channel_id, region) = self.calculate_id_xyce_level18(vgs_eff, vds_eff, vbs_eff);
            return (p * channel_id, region);
        }

        // Check for body diode conduction (reverse Vds)
        if vds_eff < -0.3 {
            // Body diode conducting
            let vd = -vds_eff; // Forward diode voltage
            let vt = 0.0259; // Thermal voltage at 300K
            let id_diode = self.is * ((vd / (self.n * vt)).exp() - 1.0);
            return (p * (-id_diode), VdmosRegion::BodyDiode);
        }

        let vgt = vgs_eff - self.vth;

        // Cutoff region
        if vgt <= 0.0 {
            // Subthreshold leakage (simplified)
            let vt = 0.0259;
            let i_sub = 1e-12 * (vgt / (1.5 * vt)).exp().min(1e-6);
            return (p * i_sub, VdmosRegion::Cutoff);
        }

        // Saturation voltage. Xyce Level 18 supplies VMAX; translate it to
        // a continuous velocity-saturated Vdsat rather than a discontinuous
        // current clamp.
        let vdsat = if self.velocity_saturation_voltage.is_finite()
            && self.velocity_saturation_voltage > 0.0
        {
            vgt / (1.0 + vgt / self.velocity_saturation_voltage)
        } else {
            vgt
        };

        if vds_eff < vdsat {
            // Triode (linear) region. Use the standard square-law MOS
            // expression so conductance is finite and well-conditioned at
            // Vds ~= 0.
            let id =
                self.kp * (vgt * vds_eff - 0.5 * vds_eff * vds_eff) * (1.0 + self.lambda * vds_eff);
            (p * id, VdmosRegion::Triode)
        } else {
            // Saturation or quasi-saturation
            let id_sat =
                self.kp * (vgt * vdsat - 0.5 * vdsat * vdsat) * (1.0 + self.lambda * vds_eff);

            // Check for quasi-saturation (drift region limiting)
            if vds_eff > self.vq && self.rq > 0.0 {
                // In quasi-saturation, current is limited by drift region
                // Solve: Id = Id_sat / (1 + Id * Rq / (Vds - Vq))
                // This requires iteration, use simplified model:
                let vexcess = vds_eff - self.vq;
                let _id_drift = vexcess / self.rq;
                let id = id_sat.min(id_sat / (1.0 + id_sat * self.rq / vexcess.max(0.001)));
                (p * id, VdmosRegion::QuasiSaturation)
            } else {
                (p * id_sat, VdmosRegion::Saturation)
            }
        }
    }

    /// Calculate transconductance gm = dId/dVgs
    pub fn gm(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let delta = 1e-6;
        let (id_plus, _) = self.calculate_id_with_body(vgs + delta, vds, vbs);
        let (id_minus, _) = self.calculate_id_with_body(vgs - delta, vds, vbs);
        (id_plus - id_minus) / (2.0 * delta)
    }

    /// Calculate output conductance gds = dId/dVds
    pub fn gds(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let delta = 1e-6;
        let (id_plus, _) = self.calculate_id_with_body(vgs, vds + delta, vbs);
        let (id_minus, _) = self.calculate_id_with_body(vgs, vds - delta, vbs);
        ((id_plus - id_minus) / (2.0 * delta)).max(1e-12)
    }

    pub fn gmb(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let delta = 1e-6;
        let (id_plus, _) = self.calculate_id_with_body(vgs, vds, vbs + delta);
        let (id_minus, _) = self.calculate_id_with_body(vgs, vds, vbs - delta);
        (id_plus - id_minus) / (2.0 * delta)
    }

    /// Calculate body diode conductance
    pub fn gdiode(&self, vds: Value) -> Value {
        if vds < 0.0 {
            let vt = 0.0259;
            let vd = -vds;
            (self.is / (self.n * vt)) * (vd / (self.n * vt)).exp()
        } else {
            1e-12
        }
    }

    //=========================================================================
    // Voltage-Dependent Capacitance Calculations
    // Uses standard SPICE junction model: C = C0 / (1 - V/Pb)^M
    // With linear extrapolation for forward bias V > Fc*Pb
    //=========================================================================

    /// Calculate gate-source capacitance Cgs(Vgs)
    ///
    /// Uses junction capacitance model with forward bias extrapolation.
    /// For VDMOS, Cgs increases slightly with positive Vgs as the channel
    /// inverts and the effective oxide area increases.
    #[inline]
    pub fn cgs_effective(&self, vgs: Value) -> Value {
        self.junction_capacitance(self.cgs0, vgs, self.cgs_pb, self.cgs_m, self.fc)
    }

    /// Calculate gate-drain (Miller) capacitance Cgd(Vgd)
    ///
    /// This is the critical nonlinear capacitance in power MOSFETs.
    /// - When Vgd > 0 (drain below gate): Large capacitance (overlap + depletion)
    /// - When Vgd < 0 (drain above gate): Capacitance drops dramatically
    ///   as the depletion region extends into the lightly-doped drift region
    ///
    /// The "Miller plateau" during switching occurs when Cgd is charging/discharging.
    #[inline]
    pub fn cgd_effective(&self, vgd: Value) -> Value {
        self.junction_capacitance(self.cgd0, vgd, self.cgd_pb, self.cgd_m, self.fc)
    }

    /// Calculate all three capacitances for given terminal voltages
    ///
    /// Returns (Cgs, Cgd, Cds) for transient analysis stamping.
    pub fn capacitances(&self, vgs: Value, vds: Value) -> (Value, Value, Value) {
        let vgd = vgs - vds; // Gate-to-drain voltage

        let cgs = self.cgs_effective(vgs) + self.cgs_overlap.max(0.0);
        let cgd = self.cgd_effective(vgd) + self.cgd_overlap.max(0.0);
        let cds = self.cds; // Fixed for now, could be voltage-dependent

        (cgs, cgd, cds)
    }

    #[inline]
    pub fn gate_bulk_capacitance(&self) -> Value {
        self.cgb_overlap.max(0.0)
    }

    #[inline]
    pub fn body_source_charge_and_capacitance_at(&self, vbs: Value) -> (Value, Value) {
        let (bottom, sidewall) = self.body_source_zero_bias_caps();
        self.body_junction_charge_and_capacitance(bottom, sidewall, vbs)
    }

    #[inline]
    pub fn body_drain_charge_and_capacitance_at(&self, vbd: Value) -> (Value, Value) {
        let (bottom, sidewall) = self.body_drain_zero_bias_caps();
        self.body_junction_charge_and_capacitance(bottom, sidewall, vbd)
    }

    #[inline]
    fn body_source_zero_bias_caps(&self) -> (Value, Value) {
        let bottom = if self.cbs_given {
            self.cbs0
        } else {
            self.body_cj * self.source_area
        };
        let sidewall = self.body_cjsw * self.source_perimeter;
        (bottom, sidewall)
    }

    #[inline]
    fn body_drain_zero_bias_caps(&self) -> (Value, Value) {
        let bottom = if self.cbd_given {
            self.cbd0
        } else {
            self.body_cj * self.drain_area
        };
        let sidewall = self.body_cjsw * self.drain_perimeter;
        (bottom, sidewall)
    }

    #[inline]
    fn body_transient_charge_factor(&self) -> Value {
        // Xyce LEVEL=18 loads VDMOS body-junction derivatives through both
        // its legacy F-vector current state and DAE Q-vector charge state.
        if self.xyce_level18 { 2.0 } else { 1.0 }
    }

    #[inline]
    pub(crate) fn body_source_transient_charge_and_capacitance_at(
        &self,
        vbs: Value,
    ) -> (Value, Value) {
        let (charge, capacitance) = self.body_source_charge_and_capacitance_at(vbs);
        let factor = self.body_transient_charge_factor();
        (factor * charge, factor * capacitance)
    }

    #[inline]
    pub(crate) fn body_drain_transient_charge_and_capacitance_at(
        &self,
        vbd: Value,
    ) -> (Value, Value) {
        let (charge, capacitance) = self.body_drain_charge_and_capacitance_at(vbd);
        let factor = self.body_transient_charge_factor();
        (factor * charge, factor * capacitance)
    }

    #[inline]
    pub(crate) fn d1_charge_and_capacitance_at(&self, vds: Value) -> (Value, Value) {
        let (charge, capacitance) = self.d1_raw_charge_and_capacitance(-vds);
        (-charge, capacitance)
    }

    #[inline]
    pub(crate) fn d1_charge_nodes(&self) -> (NodeId, NodeId) {
        let d1p = self.d1_prime_node();
        match self.vdmos_type {
            VdmosType::NVdmos => (self.drain, d1p),
            VdmosType::PVdmos => (d1p, self.drain),
        }
    }

    #[inline]
    pub(crate) fn d1_charge_branch_voltage_at(&self, voltages: &[Value]) -> Value {
        let vd = if self.drain > 0 {
            voltages[self.drain - 1]
        } else {
            0.0
        };
        let d1p = self.d1_prime_node();
        let vp = if d1p > 0 { voltages[d1p - 1] } else { 0.0 };
        self.polarity() * (vd - vp)
    }

    #[inline]
    fn d1_prime_node(&self) -> NodeId {
        self.d1_prime.unwrap_or(self.source)
    }

    #[inline]
    fn d1_series_resistance_enabled(&self) -> bool {
        self.d1_rs.is_finite() && self.d1_rs > 1.0e-12
    }

    #[inline]
    fn d1_current_branch_terms_at(
        &self,
        voltages: &[Value],
    ) -> Option<(NodeId, NodeId, Value, Value)> {
        if !self.xyce_level18 || !self.d1_current_enabled {
            return None;
        }

        let d = self.drain;
        let pnode = self.d1_prime_node();
        let vd = if d > 0 { voltages[d - 1] } else { 0.0 };
        let vp = if pnode > 0 { voltages[pnode - 1] } else { 0.0 };
        let p = self.polarity();
        let normalized_vds = p * (vd - vp);
        let (diode_current, diode_conductance) = self.d1_current_and_conductance(-normalized_vds);
        let current = p * (-diode_current);
        let conductance = diode_conductance.max(0.0);
        let vdiff = vd - vp;
        let ieq = current - conductance * vdiff;
        Some((d, pnode, conductance, ieq))
    }

    #[inline]
    pub(crate) fn gate_source_charge_nodes(&self) -> (NodeId, NodeId) {
        let si = self.source_int.unwrap_or(self.source);
        match self.vdmos_type {
            VdmosType::NVdmos => (self.gate, si),
            VdmosType::PVdmos => (si, self.gate),
        }
    }

    #[inline]
    pub(crate) fn gate_drain_charge_nodes(&self) -> (NodeId, NodeId) {
        let di = self.drain_int.unwrap_or(self.drain);
        match self.vdmos_type {
            VdmosType::NVdmos => (self.gate, di),
            VdmosType::PVdmos => (di, self.gate),
        }
    }

    #[inline]
    pub(crate) fn gate_bulk_charge_nodes(&self) -> (NodeId, NodeId) {
        match self.vdmos_type {
            VdmosType::NVdmos => (self.gate, self.bulk),
            VdmosType::PVdmos => (self.bulk, self.gate),
        }
    }

    #[inline]
    pub(crate) fn drain_source_charge_nodes(&self) -> (NodeId, NodeId) {
        let di = self.drain_int.unwrap_or(self.drain);
        let si = self.source_int.unwrap_or(self.source);
        match self.vdmos_type {
            VdmosType::NVdmos => (di, si),
            VdmosType::PVdmos => (si, di),
        }
    }

    #[inline]
    pub(crate) fn body_source_charge_nodes(&self) -> (NodeId, NodeId) {
        let si = self.source_int.unwrap_or(self.source);
        match self.vdmos_type {
            VdmosType::NVdmos => (self.bulk, si),
            VdmosType::PVdmos => (si, self.bulk),
        }
    }

    #[inline]
    pub(crate) fn body_drain_charge_nodes(&self) -> (NodeId, NodeId) {
        let di = self.drain_int.unwrap_or(self.drain);
        match self.vdmos_type {
            VdmosType::NVdmos => (self.bulk, di),
            VdmosType::PVdmos => (di, self.bulk),
        }
    }

    #[inline]
    pub(crate) fn body_charge_branch_voltages_at(&self, voltages: &[Value]) -> (Value, Value) {
        let di = self.drain_int.unwrap_or(self.drain);
        let si = self.source_int.unwrap_or(self.source);
        let b = self.bulk;

        let vd = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vs = if si > 0 { voltages[si - 1] } else { 0.0 };
        let vb = if b > 0 { voltages[b - 1] } else { 0.0 };
        let p = self.polarity();
        (p * (vb - vs), p * (vb - vd))
    }

    pub(crate) fn transient_charge_branch_voltages_at(
        &self,
        voltages: &[Value],
    ) -> (Value, Value, Value, Value) {
        let g = self.gate;
        let di = self.drain_int.unwrap_or(self.drain);
        let si = self.source_int.unwrap_or(self.source);
        let b = self.bulk;

        let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
        let vd = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vs = if si > 0 { voltages[si - 1] } else { 0.0 };
        let vb = if b > 0 { voltages[b - 1] } else { 0.0 };

        let p = self.polarity();
        let vgs = p * (vg - vs);
        let vgd = p * (vg - vd);
        let vgb = p * (vg - vb);
        let vds = p * (vd - vs);
        (vgs, vgd, vgb, vds)
    }

    /// Generic junction capacitance calculation
    ///
    /// Implements the standard SPICE model:
    /// - For V ≤ Fc*Pb: C = C0 / (1 - V/Pb)^M
    /// - For V > Fc*Pb: Linear extrapolation to avoid infinity
    ///
    /// # Arguments
    /// * `c0` - Zero-bias capacitance
    /// * `v` - Junction voltage (positive = forward bias)
    /// * `pb` - Junction built-in potential (typically 0.8V)
    /// * `m` - Grading coefficient (0.5 for abrupt, 0.33 for graded)
    /// * `fc` - Forward bias coefficient (typically 0.5)
    #[inline]
    fn junction_capacitance(&self, c0: Value, v: Value, pb: Value, m: Value, fc: Value) -> Value {
        if pb <= 0.0 {
            return c0; // Fallback for invalid parameters
        }

        let fc_pb = fc * pb;

        if v <= fc_pb {
            // Standard reverse/low-forward bias model
            let denominator = (1.0 - v / pb).max(1e-6);
            c0 / denominator.powf(m)
        } else {
            // Forward bias linear extrapolation (prevents blow-up)
            // C(Fc*Pb) = C0 / (1-Fc)^M
            // dC/dV at Fc*Pb = C0 * M / (Pb * (1-Fc)^(M+1))
            let one_minus_fc = 1.0 - fc;
            let c_fc = c0 / one_minus_fc.powf(m);
            let dc_dv = c0 * m / (pb * one_minus_fc.powf(m + 1.0));
            c_fc + dc_dv * (v - fc_pb)
        }
    }

    #[inline]
    fn junction_depletion_scaling(arg: Value, grading: Value) -> Value {
        if (grading - 0.5).abs() < 1e-12 {
            1.0 / arg.sqrt()
        } else {
            (-grading * arg.ln()).exp()
        }
    }

    #[inline]
    fn junction_depletion_charge_term(
        c0: Value,
        potential: Value,
        grading: Value,
        arg: Value,
        scaling: Value,
    ) -> Value {
        if (1.0 - grading).abs() < 1e-12 {
            -c0 * potential * arg.ln()
        } else {
            c0 * potential * (1.0 - arg * scaling) / (1.0 - grading)
        }
    }

    #[inline]
    fn body_junction_component_charge_and_capacitance(
        &self,
        c0: Value,
        v: Value,
        grading: Value,
    ) -> (Value, Value) {
        Self::junction_component_charge_and_capacitance(c0, self.body_pb, grading, self.fc, v)
    }

    #[inline]
    fn d1_raw_charge_and_capacitance(&self, vd: Value) -> (Value, Value) {
        let (current, conductance) = if self.d1_current_enabled {
            self.d1_current_and_conductance(vd)
        } else {
            (0.0, 0.0)
        };
        let (depletion_charge, depletion_capacitance) =
            self.d1_depletion_charge_and_capacitance(vd);
        let transit_time = self.d1_tt.max(0.0);
        (
            transit_time * current + depletion_charge,
            transit_time * conductance + depletion_capacitance,
        )
    }

    #[inline]
    fn d1_current_and_conductance(&self, vd: Value) -> (Value, Value) {
        let thermal_voltage = self.d1_thermal_voltage();
        let isat = self.d1_temperature_adjusted_saturation_current(thermal_voltage);
        if isat == 0.0 && self.d1_isr.max(0.0) == 0.0 {
            return (0.0, 0.0);
        }

        let nvt = (self.d1_n.max(1.0e-12) * thermal_voltage).max(1.0e-12);
        let (recombination_current, recombination_conductance) =
            self.d1_recombination_current_and_conductance(vd, thermal_voltage);
        if vd >= -3.0 * nvt {
            let evd = (vd / nvt).clamp(-120.0, 120.0).exp();
            let current = isat * (evd - 1.0) + recombination_current;
            let conductance = isat * evd / nvt + recombination_conductance;
            self.d1_high_injection_corrected(current, conductance)
        } else if let Some(breakdown_voltage) =
            self.d1_effective_breakdown_voltage(isat, thermal_voltage)
            && vd < -breakdown_voltage
        {
            let evrev = (-(breakdown_voltage + vd) / nvt).clamp(-120.0, 120.0).exp();
            (
                -isat * evrev + recombination_current,
                isat * evrev / nvt + recombination_conductance,
            )
        } else {
            let arg = 3.0 * nvt / (vd * std::f64::consts::E);
            let arg3 = arg * arg * arg;
            (
                -isat * (1.0 + arg3) + recombination_current,
                isat * 3.0 * arg3 / vd + recombination_conductance,
            )
        }
    }

    #[inline]
    fn d1_thermal_voltage(&self) -> Value {
        const K_OVER_Q: Value = 1.380_622_6e-23 / 1.602_191_8e-19;
        K_OVER_Q * self.d1_temperature_kelvin.max(1.0e-12)
    }

    #[inline]
    fn d1_temperature_adjusted_saturation_current(&self, thermal_voltage: Value) -> Value {
        let nominal_is = self.d1_is.max(0.0);
        if nominal_is == 0.0 {
            return 0.0;
        }

        let temp = self.d1_temperature_kelvin.max(1.0e-12);
        let nominal_temp = self.d1_tnom_kelvin.max(1.0e-12);
        let emission = self.d1_n.max(1.0e-12);
        let ratio = temp / nominal_temp;
        let exponent = ((ratio - 1.0) * self.d1_eg.max(0.1) / (emission * thermal_voltage))
            + (self.d1_xti / emission) * ratio.ln();
        nominal_is * exponent.clamp(-120.0, 120.0).exp()
    }

    #[inline]
    fn d1_temperature_adjusted_junction_terms(&self) -> (Value, Value) {
        const REFERENCE_TEMPERATURE: Value = 300.15;
        const BOLTZMANN: Value = 1.380_622_6e-23;
        const ELECTRON_CHARGE: Value = 1.602_191_8e-19;
        const K_OVER_Q: Value = BOLTZMANN / ELECTRON_CHARGE;
        const SILICON_BANDGAP_300K: Value = 1.115_087_7;

        let fallback_potential = self.d1_vj.max(1.0e-12);
        let fallback_capacitance = self.d1_cjo.max(0.0);
        let temperature = self.d1_temperature_kelvin.max(1.0e-12);
        let nominal_temperature = self.d1_tnom_kelvin.max(1.0e-12);
        let temperature_ratio = temperature / REFERENCE_TEMPERATURE;
        let nominal_ratio = nominal_temperature / REFERENCE_TEMPERATURE;
        if !temperature_ratio.is_finite()
            || !nominal_ratio.is_finite()
            || temperature_ratio <= 0.0
            || nominal_ratio <= 0.0
        {
            return (fallback_potential, fallback_capacitance);
        }

        let bandgap = 1.16 - (7.02e-4 * temperature * temperature) / (temperature + 1108.0);
        let temperature_arg = -bandgap / (2.0 * BOLTZMANN * temperature)
            + SILICON_BANDGAP_300K / (BOLTZMANN * (REFERENCE_TEMPERATURE + REFERENCE_TEMPERATURE));
        let pbfact = -2.0
            * K_OVER_Q
            * temperature
            * (1.5 * temperature_ratio.ln() + ELECTRON_CHARGE * temperature_arg);

        let nominal_bandgap = 1.16
            - (7.02e-4 * nominal_temperature * nominal_temperature)
                / (nominal_temperature + 1108.0);
        let nominal_arg = -nominal_bandgap / (2.0 * BOLTZMANN * nominal_temperature)
            + SILICON_BANDGAP_300K / (2.0 * BOLTZMANN * REFERENCE_TEMPERATURE);
        let pbfact_nominal = -2.0
            * K_OVER_Q
            * nominal_temperature
            * (1.5 * nominal_ratio.ln() + ELECTRON_CHARGE * nominal_arg);
        let zero_temperature_potential = (self.d1_vj - pbfact_nominal) / nominal_ratio;
        let adjusted = pbfact + temperature_ratio * zero_temperature_potential;
        if !zero_temperature_potential.is_finite()
            || zero_temperature_potential.abs() <= 1.0e-30
            || !adjusted.is_finite()
            || adjusted <= 0.0
        {
            return (fallback_potential, fallback_capacitance);
        }

        let grading = self.d1_m;
        let old_gamma = (self.d1_vj - zero_temperature_potential) / zero_temperature_potential;
        let cap_denominator =
            1.0 + grading * (400.0e-6 * (nominal_temperature - REFERENCE_TEMPERATURE) - old_gamma);
        let new_gamma = (adjusted - zero_temperature_potential) / zero_temperature_potential;
        let cap_factor =
            1.0 + grading * (400.0e-6 * (temperature - REFERENCE_TEMPERATURE) - new_gamma);
        let adjusted_capacitance = fallback_capacitance / cap_denominator * cap_factor;
        if adjusted_capacitance.is_finite()
            && cap_denominator.is_finite()
            && cap_denominator.abs() > 1.0e-30
            && cap_factor.is_finite()
            && adjusted_capacitance >= 0.0
        {
            (adjusted.max(1.0e-12), adjusted_capacitance)
        } else {
            (adjusted.max(1.0e-12), fallback_capacitance)
        }
    }

    #[inline]
    fn d1_temperature_adjusted_junction_potential(&self) -> Value {
        self.d1_temperature_adjusted_junction_terms().0
    }

    #[inline]
    fn d1_depletion_charge_and_capacitance(&self, vd: Value) -> (Value, Value) {
        let (adjusted_potential, adjusted_capacitance) =
            self.d1_temperature_adjusted_junction_terms();
        let c0 = adjusted_capacitance.max(0.0);
        if c0 == 0.0 {
            return (0.0, 0.0);
        }

        let nominal_potential = self.d1_vj.max(1.0e-12);
        let grading = self.d1_m;
        let fc = self.d1_fc.clamp(0.0, 0.999_999_999_999);
        let depletion_corner = fc * adjusted_potential;
        if vd < depletion_corner {
            let arg = (1.0 - vd / nominal_potential).max(1.0e-18);
            let scaling = Self::junction_depletion_scaling(arg, grading);
            let charge =
                Self::junction_depletion_charge_term(c0, nominal_potential, grading, arg, scaling);
            return (charge, (c0 * scaling).max(0.0));
        }

        let f2_denominator = (1.0 - fc).powf(1.0 + grading).max(1.0e-18);
        let f3 = 1.0 - fc * (1.0 + grading);
        let t_f1 = if (1.0 - grading).abs() < 1.0e-12 {
            -adjusted_potential * (1.0 - fc).max(1.0e-18).ln()
        } else {
            adjusted_potential * (1.0 - (1.0 - fc).powf(1.0 - grading)) / (1.0 - grading)
        };
        let c0_over_f2 = c0 / f2_denominator;
        let charge = c0 * t_f1
            + c0_over_f2
                * (f3 * (vd - depletion_corner)
                    + (grading / (nominal_potential + nominal_potential))
                        * (vd * vd - depletion_corner * depletion_corner));
        let capacitance = c0_over_f2 * (f3 + grading * vd / nominal_potential);
        (charge, capacitance.max(0.0))
    }

    #[inline]
    fn d1_high_injection_corrected(&self, current: Value, conductance: Value) -> (Value, Value) {
        let ikf = self.d1_ikf.max(0.0);
        let denominator = ikf + current;
        if ikf == 0.0 || denominator <= 0.0 {
            return (current, conductance);
        }

        let attenuation = (ikf / denominator).sqrt();
        let corrected_conductance = attenuation * conductance * (1.0 - 0.5 * current / denominator);
        (attenuation * current, corrected_conductance)
    }

    #[inline]
    fn d1_recombination_current_and_conductance(
        &self,
        vd: Value,
        thermal_voltage: Value,
    ) -> (Value, Value) {
        let isr = self.d1_isr.max(0.0);
        if isr == 0.0 {
            return (0.0, 0.0);
        }

        let recombination_nvt = (self.d1_nr.max(1.0e-12) * thermal_voltage).max(1.0e-12);
        let junction_potential = self.d1_temperature_adjusted_junction_potential();
        let grading = self.d1_m;
        let evr = (vd / recombination_nvt).clamp(-120.0, 120.0).exp();
        let temp = 1.0 - vd / junction_potential;
        let arg = temp * temp;
        let denominator = arg + 0.001;
        let recombination_is = isr * denominator.powf(0.5 * grading);
        let current = recombination_is * (evr - 1.0);
        let conductance = -current * grading * temp / (junction_potential * denominator)
            + recombination_is * evr / recombination_nvt;
        (current, conductance)
    }

    #[inline]
    fn d1_effective_breakdown_voltage(&self, isat: Value, thermal_voltage: Value) -> Option<Value> {
        if !self.d1_bv_given || isat <= 0.0 || thermal_voltage <= 0.0 {
            return None;
        }

        let specified_bv = self.d1_bv;
        if !specified_bv.is_finite() || specified_bv <= 0.0 {
            return None;
        }

        let breakdown_current = self.d1_ibv.max(0.0);
        let minimum_current = isat * specified_bv / thermal_voltage;
        if breakdown_current < minimum_current {
            return Some(specified_bv);
        }

        let current_ratio = breakdown_current / isat;
        let mut xbv = specified_bv - thermal_voltage * (1.0 + current_ratio).ln();
        for _ in 0..25 {
            let log_arg = (current_ratio + 1.0 - xbv / thermal_voltage).max(1.0e-300);
            xbv = specified_bv - thermal_voltage * log_arg.ln();
        }

        xbv.is_finite().then_some(xbv.max(0.0))
    }

    #[inline]
    fn junction_component_charge_and_capacitance(
        c0: Value,
        potential: Value,
        grading: Value,
        fc: Value,
        v: Value,
    ) -> (Value, Value) {
        let c0 = c0.max(0.0);
        if c0 == 0.0 {
            return (0.0, 0.0);
        }

        let potential = potential.max(1e-12);
        let fc = fc.clamp(0.0, 0.999_999_999_999);
        let depletion_corner = fc * potential;

        if v < depletion_corner {
            let arg = (1.0 - v / potential).max(1e-18);
            let scaling = Self::junction_depletion_scaling(arg, grading);
            let charge = Self::junction_depletion_charge_term(c0, potential, grading, arg, scaling);
            return (charge, (c0 * scaling).max(0.0));
        }

        let arg = (1.0 - fc).max(1e-18);
        let scaling = Self::junction_depletion_scaling(arg, grading);
        let f2 = c0 * (1.0 - fc * (1.0 + grading)) * scaling / arg;
        let f3 = c0 * grading * scaling / arg / potential;
        let edge_charge =
            Self::junction_depletion_charge_term(c0, potential, grading, arg, scaling);
        let f4 =
            edge_charge - 0.5 * f3 * depletion_corner * depletion_corner - depletion_corner * f2;
        let charge = f4 + v * (f2 + 0.5 * v * f3);
        let capacitance = (f2 + v * f3).max(0.0);
        (charge, capacitance)
    }

    #[inline]
    fn body_junction_charge_and_capacitance(
        &self,
        bottom_c0: Value,
        sidewall_c0: Value,
        v: Value,
    ) -> (Value, Value) {
        let (bottom_charge, bottom_cap) =
            self.body_junction_component_charge_and_capacitance(bottom_c0, v, self.body_m);
        let (sidewall_charge, sidewall_cap) =
            self.body_junction_component_charge_and_capacitance(sidewall_c0, v, self.body_mjsw);
        (bottom_charge + sidewall_charge, bottom_cap + sidewall_cap)
    }

    /// Link to static matrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.drain;
        let s = self.source;
        let di = self.drain_int.unwrap_or(d);
        let si = self.source_int.unwrap_or(s);

        // Drain resistance stamps
        if di != d {
            self.indices.d_d = matrix.get_index(d, d);
            self.indices.d_di = matrix.get_index(d, di);
            self.indices.di_d = matrix.get_index(di, d);
            self.indices.di_di = matrix.get_index(di, di);
        }

        // Source resistance stamps
        if si != s {
            self.indices.s_s = matrix.get_index(s, s);
            self.indices.s_si = matrix.get_index(s, si);
            self.indices.si_s = matrix.get_index(si, s);
            self.indices.si_si = matrix.get_index(si, si);
        }

        // MOS channel stamps (between internal nodes)
        self.indices.di_di = matrix.get_index(di, di);
        self.indices.di_si = matrix.get_index(di, si);
        self.indices.si_di = matrix.get_index(si, di);
        self.indices.si_si = matrix.get_index(si, si);

        self.indices.rhs_d = Some(d);
        self.indices.rhs_s = Some(s);
        self.indices.rhs_di = Some(di);
        self.indices.rhs_si = Some(si);
    }

    #[inline]
    fn stamp_direct_conductance(
        matrix: &mut StaticMatrix,
        pos: NodeId,
        neg: NodeId,
        conductance: Value,
    ) {
        if conductance == 0.0 {
            return;
        }
        if pos > 0 {
            matrix.add(pos - 1, pos - 1, conductance);
        }
        if pos > 0 && neg > 0 {
            matrix.add(pos - 1, neg - 1, -conductance);
            matrix.add(neg - 1, pos - 1, -conductance);
        }
        if neg > 0 {
            matrix.add(neg - 1, neg - 1, conductance);
        }
    }

    #[inline]
    fn stamp_direct_rhs(rhs: &mut [Value], pos: NodeId, neg: NodeId, ieq: Value) {
        if pos > 0 {
            rhs[pos - 1] -= ieq;
        }
        if neg > 0 {
            rhs[neg - 1] += ieq;
        }
    }

    /// Stamp using direct indexing
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let d = self.drain;
        let s = self.source;
        let g = self.gate;
        let b = self.bulk;
        let di = self.drain_int.unwrap_or(d);
        let dd = self.drain_drift_node();
        let si = self.source_int.unwrap_or(s);

        // Get voltages
        let vd = if d > 0 { voltages[d - 1] } else { 0.0 };
        let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
        let vb = if b > 0 { voltages[b - 1] } else { 0.0 };
        let _vs = if s > 0 { voltages[s - 1] } else { 0.0 };
        let vdd = if dd > 0 { voltages[dd - 1] } else { 0.0 };
        let vdi = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vsi = if si > 0 { voltages[si - 1] } else { 0.0 };

        let vgs = vg - vsi;
        let vds_int = vdi - vsi;
        let vbs = vb - vsi;

        // Calculate operating point
        let (id, _region) = self.calculate_id_with_body(vgs, vds_int, vbs);
        let gm = self.gm(vgs, vds_int, vbs);
        let gds = self.gds(vgs, vds_int, vbs);
        let gmb = self.gmb(vgs, vds_int, vbs);

        // Stamp nonlinear drain drift and optional explicit RD as separate
        // Xyce LEVEL=18 branches: drain -> drain_drift -> drain_prime.
        if self.xyce_drift_enabled() && dd != d {
            let vdrop = vd - vdd;
            let (current, conductance) = self.xyce_drift_current_and_conductance(vdrop);
            let ieq = current - conductance * vdrop;
            Self::stamp_direct_conductance(matrix, d, dd, conductance);
            Self::stamp_direct_rhs(rhs, d, dd, ieq);
        }
        if self.rd > 1e-12 && di != dd {
            let gd = 1.0 / self.rd;
            Self::stamp_direct_conductance(matrix, dd, di, gd);
        }

        // Stamp source resistance (between s and si)
        if self.rs > 1e-12 && si != s {
            let gs = 1.0 / self.rs;
            if let Some(idx) = self.indices.s_s {
                matrix.stamp_direct(idx, gs);
            }
            if let Some(idx) = self.indices.s_si {
                matrix.stamp_direct(idx, -gs);
            }
            if let Some(idx) = self.indices.si_s {
                matrix.stamp_direct(idx, -gs);
            }
            if let Some(idx) = self.indices.si_si {
                matrix.stamp_direct(idx, gs);
            }
        }

        if self.d1_series_resistance_enabled() {
            let d1p = self.d1_prime_node();
            Self::stamp_direct_conductance(matrix, d1p, s, 1.0 / self.d1_rs);
        }

        if let Some((pos, neg, conductance, ieq)) = self.d1_current_branch_terms_at(voltages) {
            Self::stamp_direct_conductance(matrix, pos, neg, conductance);
            Self::stamp_direct_rhs(rhs, pos, neg, ieq);
        }

        // Stamp MOS channel (linearized: Id = Id0 + gm*(Vgs-Vgs0) + gds*(Vds-Vds0))
        // Companion model current: Ieq = Id - gm*Vgs - gds*Vds
        let ieq = id - gm * vgs - gds * vds_int - gmb * vbs;

        // Stamp conductances
        // gds stamps (di-si)
        if let Some(idx) = self.indices.di_di {
            matrix.stamp_direct(idx, gds);
        }
        if let Some(idx) = self.indices.di_si {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.si_di {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.si_si {
            matrix.stamp_direct(idx, gds);
        }

        // gm stamps (controlled by Vgs = Vg - Vsi)
        // Id increases when Vg increases (for NMOS), so positive gm to di from g
        // Current into di: +gm contribution
        // Using Norton equivalent: need to stamp gm between proper nodes
        // For MOSFET: Id = f(Vgs, Vds), gm = dId/dVgs
        // We stamp gm such that change in Vg causes current change at drain

        // RHS contributions
        if let Some(di_idx) = self.indices.rhs_di
            && di_idx > 0
        {
            rhs[di_idx - 1] -= ieq;
        }
        if let Some(si_idx) = self.indices.rhs_si
            && si_idx > 0
        {
            rhs[si_idx - 1] += ieq;
        }
    }
}

impl NonlinearDevice for Vdmos {
    fn update(&mut self, voltages: &[Value]) {
        self.vgs_prev = self.prev_vgs;
        self.vds_prev = self.prev_vds;
        self.vbs_prev = self.prev_vbs;
        self.eval_vgs_prev = self.eval_vgs;
        self.eval_vds_prev = self.eval_vds;
        self.eval_vbs_prev = self.eval_vbs;
        self.id_prev = self.id;
        self.gm_prev = self.gm;
        self.gds_prev = self.gds;
        self.gmb_prev = self.gmb;

        let g = self.gate;
        let b = self.bulk;
        let di = self.drain_int.unwrap_or(self.drain);
        let si = self.source_int.unwrap_or(self.source);

        let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
        let vb = if b > 0 { voltages[b - 1] } else { 0.0 };
        let vdi = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vsi = if si > 0 { voltages[si - 1] } else { 0.0 };

        let vgs = vg - vsi;
        let vds = vdi - vsi;
        let vbs = vb - vsi;
        let (eval_vgs, eval_vds, limited) = self.limited_branch_voltages_for_eval(vgs, vds);
        let eval_vbs = vbs;

        self.prev_vgs = vgs;
        self.prev_vds = vds;
        self.prev_vbs = vbs;
        self.eval_vgs = eval_vgs;
        self.eval_vds = eval_vds;
        self.eval_vbs = eval_vbs;
        self.limiter_applied = limited;

        let (id, region, gm, gds, gmb, id_eq) =
            self.linearized_operating_point(eval_vgs, eval_vds, eval_vbs);
        self.id = id;
        self.region = region;
        self.gm = gm;
        self.gds = gds;
        self.gmb = gmb;
        self.id_eq = id_eq;
        self.power = id * eval_vds;
        self.has_branch_history = true;

        // Update body diode current
        if eval_vds < 0.0 {
            let vt = 0.0259;
            let vd = -eval_vds;
            self.id_diode = self.is * ((vd / (self.n * vt)).exp() - 1.0);
        } else {
            self.id_diode = 0.0;
        }
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let d = self.drain;
        let s = self.source;
        let g = self.gate;
        let b = self.bulk;
        let di = self.drain_int.unwrap_or(d);
        let dd = self.drain_drift_node();
        let si = self.source_int.unwrap_or(s);

        let vd = if d > 0 { voltages[d - 1] } else { 0.0 };
        let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
        let vb = if b > 0 { voltages[b - 1] } else { 0.0 };
        let vdd = if dd > 0 { voltages[dd - 1] } else { 0.0 };
        let vdi = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vsi = if si > 0 { voltages[si - 1] } else { 0.0 };

        let vgs = vg - vsi;
        let vds_int = vdi - vsi;
        let vbs = vb - vsi;

        let (eval_vgs, eval_vds, _) = if self.prev_vgs == vgs && self.prev_vds == vds_int {
            (self.eval_vgs, self.eval_vds, self.limiter_applied)
        } else {
            self.limited_branch_voltages_for_eval(vgs, vds_int)
        };
        let eval_vbs = vbs;
        let (gm, gds, gmb, ieq) = if self.prev_vgs == vgs
            && self.prev_vds == vds_int
            && self.prev_vbs == vbs
            && self.eval_vgs == eval_vgs
            && self.eval_vds == eval_vds
            && self.eval_vbs == eval_vbs
            && self.gm.is_finite()
            && self.gds.is_finite()
            && self.gmb.is_finite()
            && self.id_eq.is_finite()
        {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) =
                self.linearized_operating_point(eval_vgs, eval_vds, eval_vbs);
            (gm, gds, gmb, id_eq)
        };

        // Stamp Xyce LEVEL=18 drain path. The nonlinear drift branch sits
        // between the external drain and DrainDrift; explicit RD sits between
        // DrainDrift and DrainPrime, which is the MOS channel drain.
        if self.xyce_drift_enabled() && dd != d {
            let vdrop = vd - vdd;
            let (current, conductance) = self.xyce_drift_current_and_conductance(vdrop);
            let ieq = current - conductance * vdrop;
            matrix.stamp(d, d, conductance);
            matrix.stamp(d, dd, -conductance);
            matrix.stamp(dd, d, -conductance);
            matrix.stamp(dd, dd, conductance);
            matrix.stamp_rhs(d, -ieq);
            matrix.stamp_rhs(dd, ieq);
        }
        if self.rd > 1e-12 && di != dd {
            let gd = 1.0 / self.rd;
            matrix.stamp(dd, dd, gd);
            matrix.stamp(dd, di, -gd);
            matrix.stamp(di, dd, -gd);
            matrix.stamp(di, di, gd);
        }

        if self.rs > 1e-12 && si != s {
            let _vs = if s > 0 { voltages[s - 1] } else { 0.0 };
            let gs = 1.0 / self.rs;
            matrix.stamp(s, s, gs);
            matrix.stamp(s, si, -gs);
            matrix.stamp(si, s, -gs);
            matrix.stamp(si, si, gs);
        }

        if self.d1_series_resistance_enabled() {
            let d1p = self.d1_prime_node();
            let gs = 1.0 / self.d1_rs;
            matrix.stamp(d1p, d1p, gs);
            matrix.stamp(d1p, s, -gs);
            matrix.stamp(s, d1p, -gs);
            matrix.stamp(s, s, gs);
        }

        if let Some((pos, neg, conductance, ieq)) = self.d1_current_branch_terms_at(voltages) {
            matrix.stamp(pos, pos, conductance);
            matrix.stamp(pos, neg, -conductance);
            matrix.stamp(neg, pos, -conductance);
            matrix.stamp(neg, neg, conductance);
            matrix.stamp_rhs(pos, -ieq);
            matrix.stamp_rhs(neg, ieq);
        }

        // Stamp MOS channel

        // gds stamps
        matrix.stamp(di, di, gds);
        matrix.stamp(di, si, -gds);
        matrix.stamp(si, di, -gds);
        matrix.stamp(si, si, gds);

        // gm stamps (transconductance from gate)
        // For controlled source: Id has gm contribution from Vgs
        // Stamp as: +gm into di from g, -gm into di from si
        matrix.stamp(di, g, gm);
        matrix.stamp(di, si, -gm);
        matrix.stamp(si, g, -gm);
        matrix.stamp(si, si, gm);

        // Body transconductance.
        matrix.stamp(di, b, gmb);
        matrix.stamp(di, si, -gmb);
        matrix.stamp(si, b, -gmb);
        matrix.stamp(si, si, gmb);

        // RHS
        matrix.stamp_rhs(di, -ieq);
        matrix.stamp_rhs(si, ieq);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if self.limiter_applied {
            return false;
        }

        if !self.prev_vgs.is_finite()
            || !self.prev_vds.is_finite()
            || !self.prev_vbs.is_finite()
            || !self.vgs_prev.is_finite()
            || !self.vds_prev.is_finite()
            || !self.vbs_prev.is_finite()
            || !self.eval_vgs.is_finite()
            || !self.eval_vds.is_finite()
            || !self.eval_vbs.is_finite()
            || !self.eval_vgs_prev.is_finite()
            || !self.eval_vds_prev.is_finite()
            || !self.eval_vbs_prev.is_finite()
            || !self.id.is_finite()
            || !self.id_prev.is_finite()
            || !self.gm_prev.is_finite()
            || !self.gds_prev.is_finite()
            || !self.gmb_prev.is_finite()
        {
            return false;
        }

        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();

        let vgs_diff = (self.prev_vgs - self.vgs_prev).abs();
        let vds_diff = (self.prev_vds - self.vds_prev).abs();
        let vbs_diff = (self.prev_vbs - self.vbs_prev).abs();
        let vgs_tol = reltol * self.prev_vgs.abs().max(self.vgs_prev.abs()) + voltage_tol;
        let vds_tol = reltol * self.prev_vds.abs().max(self.vds_prev.abs()) + voltage_tol;
        let vbs_tol = reltol * self.prev_vbs.abs().max(self.vbs_prev.abs()) + voltage_tol;
        vgs_diff < vgs_tol && vds_diff < vds_tol && vbs_diff < vbs_tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1_charge_branch_voltage_uses_external_nodes_not_channel_internal_nodes() {
        let mut vdmos = Vdmos::new_nvdmos("m1".to_string(), 1, 2, 3);
        vdmos.set_bulk_node(4);
        vdmos.set_internal_nodes(5, 6);
        let voltages = [-0.5, 0.0, 0.0, 0.0, -0.25, 0.1];

        let d1_vds = vdmos.d1_charge_branch_voltage_at(&voltages);

        assert_eq!(d1_vds, -0.5);
    }

    #[test]
    fn pmos_d1_charge_branch_voltage_is_polarity_normalized() {
        let mut vdmos = Vdmos::new_pvdmos("m1".to_string(), 1, 2, 3);
        vdmos.set_bulk_node(4);
        vdmos.set_internal_nodes(5, 6);
        let voltages = [0.5, 0.0, 0.0, 0.0, 0.25, -0.1];

        let d1_vds = vdmos.d1_charge_branch_voltage_at(&voltages);

        assert_eq!(d1_vds, -0.5);
    }
}

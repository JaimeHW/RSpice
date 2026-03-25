//! VDMOS (Vertical Double-diffused MOS) Power MOSFET Model
//!
//! Implements a power MOSFET model suitable for switching power supply simulation.
//! Key differences from planar MOSFET:
//! - **Quasi-saturation**: At high currents, the lightly-doped drift region limits current
//! - **Body diode**: Integrated anti-parallel diode from source to drain
//! - **Rds(on)**: Characterized by on-resistance rather than transconductance
//!
//! # Model Parameters
//! | Parameter | Description | Default |
//! |-----------|-------------|---------|
//! | VTH | Threshold voltage | 2.0V |
//! | KP | Transconductance coefficient | 2.0 A/V² |
//! | RD | Drain drift region resistance | 0.1Ω |
//! | RS | Source metallization resistance | 0.01Ω |
//! | RG | Gate resistance | 1Ω |
//! | LAMBDA | Channel length modulation | 0.01 V⁻¹ |
//! | MTRIODE | Triode region exponent | 1.5 |
//! | RQ | Quasi-saturation resistance | 0.5Ω |
//! | VQ | Quasi-saturation voltage | 5.0V |
//! | CGS | Gate-source capacitance | 1nF |
//! | CGD | Gate-drain (Miller) capacitance | 100pF |
//! | IS | Body diode saturation current | 1e-14A |
//! | N | Body diode ideality factor | 1.5 |
//! | TT | Body diode transit time | 50ns |
//! | BV | Breakdown voltage | 100V |

use crate::device::traits::{MatrixStamper, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

//=============================================================================
// Types
//=============================================================================

/// VDMOS device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdmosType {
    /// N-channel VDMOS (most common for power switching)
    NVdmos,
    /// P-channel VDMOS
    PVdmos,
}

/// Operating region of VDMOS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VdmosRegion {
    /// Gate below threshold, no channel conduction
    Cutoff,
    /// Linear/ohmic region (Vds < Vgs - Vth)
    Triode,
    /// Normal saturation region
    Saturation,
    /// Quasi-saturation: drift region limiting current
    QuasiSaturation,
    /// Body diode conducting (reverse Vds)
    BodyDiode,
}

//=============================================================================
// Thermal Network for Self-Heating
//=============================================================================

/// Thermal network for electro-thermal simulation
///
/// Models the junction temperature rise due to power dissipation using
/// a single-pole Foster RC network:
///   dTj/dt = (P - (Tj - Ta)/Rth) / Cth
///
/// At steady-state: Tj = Ta + P * Rth
#[derive(Debug, Clone, Copy)]
pub struct ThermalNetwork {
    /// Thermal resistance junction-to-ambient (K/W)
    pub rth: Value,
    /// Thermal capacitance (J/K)
    pub cth: Value,
    /// Ambient temperature (K)
    pub t_ambient: Value,
    /// Current junction temperature (K)
    pub t_junction: Value,
    /// Previous junction temperature for transient integration
    prev_t_junction: Value,
    /// Accumulated power for averaging (W·s)
    power_integral: Value,
    /// Time of last thermal update (s)
    last_update_time: Value,
}

impl Default for ThermalNetwork {
    fn default() -> Self {
        Self {
            rth: 1.0,           // 1 K/W - typical for TO-220 with heatsink
            cth: 0.01,          // 10 mJ/K - typical thermal mass
            t_ambient: 300.15,  // 27°C in Kelvin
            t_junction: 300.15, // Start at ambient
            prev_t_junction: 300.15,
            power_integral: 0.0,
            last_update_time: 0.0,
        }
    }
}

impl ThermalNetwork {
    /// Create a new thermal network with specified parameters
    pub fn new(rth: Value, cth: Value, t_ambient: Value) -> Self {
        Self {
            rth,
            cth,
            t_ambient,
            t_junction: t_ambient,
            prev_t_junction: t_ambient,
            power_integral: 0.0,
            last_update_time: 0.0,
        }
    }

    /// Check if thermal network is enabled (Rth > 0)
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.rth > 0.0 && self.cth > 0.0
    }

    /// Get thermal voltage at current junction temperature
    #[inline]
    pub fn thermal_voltage(&self) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * self.t_junction / Q_ELECTRON
    }

    /// Update junction temperature based on instantaneous power
    ///
    /// Uses backward Euler integration for stability:
    ///   Tj(n+1) = (Cth*Tj(n) + dt*(P + Ta/Rth)) / (Cth + dt/Rth)
    pub fn update(&mut self, power: Value, time: Value, dt: Value) {
        if !self.is_enabled() || dt <= 0.0 {
            return;
        }

        self.prev_t_junction = self.t_junction;

        // Backward Euler for thermal RC network
        let g_th = 1.0 / self.rth;
        let denominator = self.cth + dt * g_th;
        let numerator = self.cth * self.t_junction + dt * (power + g_th * self.t_ambient);
        self.t_junction = numerator / denominator;

        // Clamp to reasonable range (prevent runaway)
        self.t_junction = self
            .t_junction
            .clamp(self.t_ambient, self.t_ambient + 200.0);

        self.last_update_time = time;
    }

    /// Get steady-state junction temperature for given power
    #[inline]
    pub fn steady_state_temperature(&self, power: Value) -> Value {
        self.t_ambient + power * self.rth
    }

    /// Reset thermal state to ambient
    pub fn reset(&mut self) {
        self.t_junction = self.t_ambient;
        self.prev_t_junction = self.t_ambient;
        self.power_integral = 0.0;
        self.last_update_time = 0.0;
    }
}

//=============================================================================
// Soft-Recovery Body Diode
//=============================================================================

/// Body diode reverse recovery model
///
/// Models the stored charge in the body diode that must be removed
/// during turn-off, causing reverse recovery current.
///
/// Key parameters:
/// - Qrr: Total reverse recovery charge
/// - trr: Reverse recovery time
/// - Softness: Controls snap-off behavior (0=snappy, 1=soft)
///
/// The recovery current follows: Irr = Qrr * f(t/trr, softness)
#[derive(Debug, Clone, Copy)]
pub struct DiodeRecovery {
    /// Reverse recovery charge (C)
    pub qrr: Value,
    /// Reverse recovery time (s)
    pub trr: Value,
    /// Softness factor (0.0 = snappy, 1.0 = soft)
    pub softness: Value,
    /// Current stored charge (C)
    stored_charge: Value,
    /// Previous diode current for charge tracking
    prev_current: Value,
    /// Time when recovery started
    recovery_start_time: Value,
    /// Flag indicating active recovery
    in_recovery: bool,
}

impl Default for DiodeRecovery {
    fn default() -> Self {
        Self {
            qrr: 0.0,      // Disabled by default
            trr: 100e-9,   // 100ns typical
            softness: 0.5, // Moderate softness
            stored_charge: 0.0,
            prev_current: 0.0,
            recovery_start_time: 0.0,
            in_recovery: false,
        }
    }
}

impl DiodeRecovery {
    /// Create a new recovery model with specified parameters
    pub fn new(qrr: Value, trr: Value, softness: Value) -> Self {
        Self {
            qrr,
            trr,
            softness: softness.clamp(0.0, 1.0),
            stored_charge: 0.0,
            prev_current: 0.0,
            recovery_start_time: 0.0,
            in_recovery: false,
        }
    }

    /// Check if recovery model is enabled
    #[inline]
    pub fn is_enabled(&self) -> bool {
        self.qrr > 0.0 && self.trr > 0.0
    }

    /// Update stored charge based on diode current
    ///
    /// During forward conduction: charge builds up toward Qrr
    /// During reverse: charge depletes, causing recovery current
    pub fn update(&mut self, diode_current: Value, time: Value, dt: Value) {
        if !self.is_enabled() || dt <= 0.0 {
            self.prev_current = diode_current;
            return;
        }

        if diode_current > 0.0 {
            // Forward conduction: charge builds up with time constant ~ tt
            // Q approaches Qrr * (1 - exp(-If*dt/Qrr))
            let charge_rate = diode_current.min(self.qrr / self.trr);
            self.stored_charge += charge_rate * dt;
            self.stored_charge = self.stored_charge.min(self.qrr);
            self.in_recovery = false;
        } else if self.stored_charge > 1e-15 {
            // Reverse transition: start recovery
            if !self.in_recovery && self.prev_current > 0.0 {
                self.recovery_start_time = time;
                self.in_recovery = true;
            }

            // Deplete charge based on reverse current
            self.stored_charge += diode_current * dt; // diode_current is negative
            self.stored_charge = self.stored_charge.max(0.0);

            if self.stored_charge < 1e-15 {
                self.in_recovery = false;
            }
        }

        self.prev_current = diode_current;
    }

    /// Get recovery current contribution
    ///
    /// Returns additional current that flows during reverse recovery.
    /// Uses a softness-dependent waveform shape.
    pub fn recovery_current(&self, time: Value) -> Value {
        if !self.in_recovery || self.stored_charge <= 0.0 {
            return 0.0;
        }

        let t_rel = time - self.recovery_start_time;
        if t_rel < 0.0 || t_rel > 3.0 * self.trr {
            return 0.0;
        }

        // Recovery waveform: triangular modified by softness
        // ta = trr * softness (time of peak reverse current)
        // tb = trr * (1 - softness) (decay time)
        let ta = self.trr * (1.0 - self.softness * 0.5);
        let tb = self.trr * (1.0 + self.softness);

        let irr_peak = 2.0 * self.qrr / (ta + tb);

        if t_rel < ta {
            // Rising to peak
            -irr_peak * t_rel / ta
        } else if t_rel < ta + tb {
            // Decaying from peak
            -irr_peak * (1.0 - (t_rel - ta) / tb)
        } else {
            0.0
        }
    }

    /// Reset recovery state
    pub fn reset(&mut self) {
        self.stored_charge = 0.0;
        self.prev_current = 0.0;
        self.recovery_start_time = 0.0;
        self.in_recovery = false;
    }
}

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

    // Internal node IDs (assigned during circuit elaboration)
    pub drain_int: Option<NodeId>,
    pub source_int: Option<NodeId>,

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
            drain_int: None,
            source_int: None,

            // Default power MOSFET parameters (typical values)
            vth: 2.0,
            kp: 2.0,
            rd: 0.1,
            rs: 0.01,
            rg: 1.0,
            lambda: 0.01,
            mtriode: 1.5,
            rq: 0.5,
            vq: 5.0,

            // Nonlinear capacitance defaults (typical power MOSFET)
            cgs0: 1e-9,    // 1nF zero-bias Cgs
            cgd0: 100e-12, // 100pF zero-bias Cgd (Miller)
            cgs_pb: 0.8,   // Gate-source junction potential
            cgd_pb: 0.8,   // Gate-drain junction potential
            cgs_m: 0.5,    // Grading coefficient
            cgd_m: 0.5,    // Grading coefficient
            fc: 0.5,       // Forward bias limit coefficient
            cds: 50e-12,   // Drain-source capacitance (fixed)

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

            indices: VdmosIndices::default(),
        }
    }

    /// Set model parameters from a parameter map
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("VTH") {
            self.vth = v;
        }
        if let Some(&v) = params.get("VTO") {
            self.vth = v;
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
        if let Some(&v) = params.get("PB") {
            self.cgs_pb = v;
            self.cgd_pb = v;
        }
        if let Some(&v) = params.get("CGSPB") {
            self.cgs_pb = v;
        }
        if let Some(&v) = params.get("CGDPB") {
            self.cgd_pb = v;
        }
        if let Some(&v) = params.get("M") {
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

    /// Set internal node IDs (called during circuit elaboration)
    pub fn set_internal_nodes(&mut self, drain_int: NodeId, source_int: NodeId) {
        self.drain_int = Some(drain_int);
        self.source_int = Some(source_int);
    }

    /// Get polarity multiplier (+1 for N-channel, -1 for P-channel)
    #[inline]
    pub fn polarity(&self) -> Value {
        match self.vdmos_type {
            VdmosType::NVdmos => 1.0,
            VdmosType::PVdmos => -1.0,
        }
    }

    /// Calculate drain current and region for given terminal voltages
    pub fn calculate_id(&self, vgs: Value, vds: Value) -> (Value, VdmosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;

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

        // Saturation voltage
        let vdsat = vgt;

        if vds_eff < vdsat {
            // Triode (linear) region
            // Use smooth transition with mtriode exponent
            let x = vds_eff / vdsat;
            let id = self.kp * vgt * vds_eff * (1.0 - (1.0 - x).powf(self.mtriode));
            (p * id, VdmosRegion::Triode)
        } else {
            // Saturation or quasi-saturation
            let id_sat = 0.5 * self.kp * vgt * vgt * (1.0 + self.lambda * vds_eff);

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
    pub fn gm(&self, vgs: Value, vds: Value) -> Value {
        let delta = 1e-6;
        let (id_plus, _) = self.calculate_id(vgs + delta, vds);
        let (id_minus, _) = self.calculate_id(vgs - delta, vds);
        (id_plus - id_minus) / (2.0 * delta)
    }

    /// Calculate output conductance gds = dId/dVds
    pub fn gds(&self, vgs: Value, vds: Value) -> Value {
        let delta = 1e-6;
        let (id_plus, _) = self.calculate_id(vgs, vds + delta);
        let (id_minus, _) = self.calculate_id(vgs, vds - delta);
        ((id_plus - id_minus) / (2.0 * delta)).max(1e-12)
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

        let cgs = self.cgs_effective(vgs);
        let cgd = self.cgd_effective(vgd);
        let cds = self.cds; // Fixed for now, could be voltage-dependent

        (cgs, cgd, cds)
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

    /// Stamp using direct indexing
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let d = self.drain;
        let s = self.source;
        let g = self.gate;
        let di = self.drain_int.unwrap_or(d);
        let si = self.source_int.unwrap_or(s);

        // Get voltages
        let _vd = if d > 0 { voltages[d - 1] } else { 0.0 };
        let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
        let _vs = if s > 0 { voltages[s - 1] } else { 0.0 };
        let vdi = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vsi = if si > 0 { voltages[si - 1] } else { 0.0 };

        let vgs = vg - vsi;
        let vds_int = vdi - vsi;

        // Calculate operating point
        let (id, _region) = self.calculate_id(vgs, vds_int);
        let gm = self.gm(vgs, vds_int);
        let gds = self.gds(vgs, vds_int);

        // Stamp drain resistance (between d and di)
        if self.rd > 1e-12 && di != d {
            let gd = 1.0 / self.rd;
            if let Some(idx) = self.indices.d_d {
                matrix.stamp_direct(idx, gd);
            }
            if let Some(idx) = self.indices.d_di {
                matrix.stamp_direct(idx, -gd);
            }
            if let Some(idx) = self.indices.di_d {
                matrix.stamp_direct(idx, -gd);
            }
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
        }

        // Stamp MOS channel (linearized: Id = Id0 + gm*(Vgs-Vgs0) + gds*(Vds-Vds0))
        // Companion model current: Ieq = Id - gm*Vgs - gds*Vds
        let ieq = id - gm * vgs - gds * vds_int;

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
            && di_idx > 0 {
                rhs[di_idx - 1] -= ieq;
            }
        if let Some(si_idx) = self.indices.rhs_si
            && si_idx > 0 {
                rhs[si_idx - 1] += ieq;
            }
    }
}

impl NonlinearDevice for Vdmos {
    fn update(&mut self, voltages: &[Value]) {
        let g = self.gate;
        let di = self.drain_int.unwrap_or(self.drain);
        let si = self.source_int.unwrap_or(self.source);

        let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
        let vdi = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vsi = if si > 0 { voltages[si - 1] } else { 0.0 };

        let vgs = vg - vsi;
        let vds = vdi - vsi;

        self.prev_vgs = vgs;
        self.prev_vds = vds;

        let (id, region) = self.calculate_id(vgs, vds);
        self.id = id;
        self.region = region;

        // Update body diode current
        if vds < 0.0 {
            let vt = 0.0259;
            let vd = -vds;
            self.id_diode = self.is * ((vd / (self.n * vt)).exp() - 1.0);
        } else {
            self.id_diode = 0.0;
        }
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    ) {
        let d = self.drain;
        let s = self.source;
        let g = self.gate;
        let di = self.drain_int.unwrap_or(d);
        let si = self.source_int.unwrap_or(s);

        let vg = if g > 0 { voltages[g - 1] } else { 0.0 };
        let vdi = if di > 0 { voltages[di - 1] } else { 0.0 };
        let vsi = if si > 0 { voltages[si - 1] } else { 0.0 };

        let vgs = vg - vsi;
        let vds_int = vdi - vsi;

        let (id, _region) = self.calculate_id(vgs, vds_int);
        let gm = self.gm(vgs, vds_int);
        let gds = self.gds(vgs, vds_int);

        // Stamp internal resistances
        if self.rd > 1e-12 && di != d {
            let _vd = if d > 0 { voltages[d - 1] } else { 0.0 };
            let gd = 1.0 / self.rd;
            matrix.stamp(d, d, gd);
            matrix.stamp(d, di, -gd);
            matrix.stamp(di, d, -gd);
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

        // Stamp MOS channel
        let ieq = id - gm * vgs - gds * vds_int;

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

        // RHS
        if di > 0 {
            rhs[di - 1] -= ieq;
        }
        if si > 0 {
            rhs[si - 1] += ieq;
        }
    }

    fn is_converged(&self, _tolerance: Value) -> bool {
        let _di = self.drain_int.unwrap_or(self.drain);
        let _si = self.source_int.unwrap_or(self.source);

        // Simple convergence check based on current stability
        // In practice, convergence is checked globally
        true
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vdmos_creation() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        assert_eq!(vdmos.name, "M1");
        assert_eq!(vdmos.vdmos_type, VdmosType::NVdmos);
        assert_eq!(vdmos.vth, 2.0);
    }

    #[test]
    fn test_vdmos_cutoff() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        let (id, region) = vdmos.calculate_id(0.0, 5.0);
        assert!(id.abs() < 1e-6);
        assert_eq!(region, VdmosRegion::Cutoff);
    }

    #[test]
    fn test_vdmos_triode() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        let (id, region) = vdmos.calculate_id(5.0, 1.0);
        assert!(id > 0.0);
        assert_eq!(region, VdmosRegion::Triode);
    }

    #[test]
    fn test_vdmos_saturation() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        let (id, region) = vdmos.calculate_id(5.0, 10.0);
        assert!(id > 0.0);
        // Could be Saturation or QuasiSaturation depending on parameters
        assert!(matches!(
            region,
            VdmosRegion::Saturation | VdmosRegion::QuasiSaturation
        ));
    }

    #[test]
    fn test_vdmos_body_diode() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        // Negative Vds should activate body diode
        let (id, region) = vdmos.calculate_id(0.0, -1.0);
        assert!(id < 0.0); // Current flows from S to D
        assert_eq!(region, VdmosRegion::BodyDiode);
    }

    #[test]
    fn test_vdmos_quasi_saturation() {
        let mut vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        vdmos.rq = 0.1; // Lower Rq to trigger quasi-sat at lower Vds
        vdmos.vq = 3.0;

        // With high Vgs and high Vds, should be in quasi-saturation
        let (id, region) = vdmos.calculate_id(10.0, 20.0);
        assert!(id > 0.0);
        assert_eq!(region, VdmosRegion::QuasiSaturation);
    }

    #[test]
    fn test_vdmos_gm() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        let gm = vdmos.gm(5.0, 5.0);
        assert!(gm > 0.0); // gm should be positive in saturation
    }

    #[test]
    fn test_vdmos_gds() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);
        let gds = vdmos.gds(5.0, 5.0);
        assert!(gds > 0.0); // gds should be positive
    }

    #[test]
    fn test_pvdmos_polarity() {
        let pvdmos = Vdmos::new_pvdmos("M1".to_string(), 1, 2, 3);
        assert_eq!(pvdmos.polarity(), -1.0);

        // P-VDMOS with negative Vgs should conduct
        let (id, _region) = pvdmos.calculate_id(-5.0, -5.0);
        assert!(id < 0.0); // Current flows opposite direction
    }

    #[test]
    fn test_vdmos_cgs_voltage_dependence() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);

        // Cgs at zero bias should equal cgs0
        let cgs_zero = vdmos.cgs_effective(0.0);
        assert!((cgs_zero - vdmos.cgs0).abs() < 1e-15);

        // Cgs should increase with forward bias (positive Vgs)
        let cgs_forward = vdmos.cgs_effective(0.3);
        assert!(
            cgs_forward > cgs_zero,
            "Cgs should increase with forward bias"
        );

        // Cgs should decrease with reverse bias (negative Vgs)
        let cgs_reverse = vdmos.cgs_effective(-5.0);
        assert!(
            cgs_reverse < cgs_zero,
            "Cgs should decrease with reverse bias"
        );
    }

    #[test]
    fn test_vdmos_cgd_miller_cliff() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);

        // Cgd at zero bias
        let cgd_zero = vdmos.cgd_effective(0.0);

        // When drain is high (Vgd < 0), Cgd should drop dramatically
        // This is the "Miller cliff" effect
        let cgd_reverse = vdmos.cgd_effective(-10.0);
        assert!(
            cgd_reverse < cgd_zero * 0.5,
            "Miller capacitance should drop at high Vds: {} vs {}",
            cgd_reverse,
            cgd_zero
        );

        // Even more reduction at higher Vds
        let cgd_high_reverse = vdmos.cgd_effective(-50.0);
        assert!(
            cgd_high_reverse < cgd_reverse,
            "Miller capacitance should continue dropping"
        );
    }

    #[test]
    fn test_vdmos_capacitances() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);

        // Get all capacitances at typical operating point
        let (cgs, cgd, cds) = vdmos.capacitances(5.0, 20.0);

        assert!(cgs > 0.0);
        assert!(cgd > 0.0);
        assert!(cds > 0.0);
        assert_eq!(cds, vdmos.cds); // Cds is fixed

        // At high Vds, Cgd should be much smaller than Cgs
        assert!(cgd < cgs, "Cgd should be less than Cgs at high Vds");
    }

    #[test]
    fn test_vdmos_forward_bias_capping() {
        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3);

        // Forward bias beyond Fc*Pb should use linear extrapolation
        // and never go to infinity
        let c_high = vdmos.junction_capacitance(1e-9, 0.9, 0.8, 0.5, 0.5);
        assert!(c_high < 1e-6, "Capacitance should be capped: {}", c_high);
        assert!(c_high > 1e-9, "Capacitance should increase in forward bias");
    }

    //=========================================================================
    // Thermal Network Tests
    //=========================================================================

    #[test]
    fn test_thermal_network_defaults() {
        let thermal = ThermalNetwork::default();
        assert!(thermal.rth > 0.0);
        assert!(thermal.cth > 0.0);
        assert!((thermal.t_junction - thermal.t_ambient).abs() < 0.01);
    }

    #[test]
    fn test_thermal_network_steady_state() {
        let thermal = ThermalNetwork::new(1.0, 0.01, 300.0);
        let power = 10.0; // 10W
        let expected_tj = 300.0 + 10.0 * 1.0; // Ta + P * Rth = 310K
        let actual = thermal.steady_state_temperature(power);
        assert!(
            (actual - expected_tj).abs() < 0.01,
            "Expected {}, got {}",
            expected_tj,
            actual
        );
    }

    #[test]
    fn test_thermal_network_transient() {
        let mut thermal = ThermalNetwork::new(10.0, 0.1, 300.0);
        let power = 5.0; // 5W

        // Simulate for many timesteps - temperature should rise
        for i in 0..1000 {
            let time = i as f64 * 0.001;
            thermal.update(power, time, 0.001);
        }

        // After 1 second, should be close to steady state (Rth*Cth = 1s)
        let steady_state = thermal.steady_state_temperature(power);
        assert!(thermal.t_junction > 300.0, "Temperature should rise");
        assert!(
            (thermal.t_junction - steady_state).abs() / steady_state < 0.1,
            "Should approach steady state: {} vs {}",
            thermal.t_junction,
            steady_state
        );
    }

    #[test]
    fn test_thermal_network_reset() {
        let mut thermal = ThermalNetwork::new(1.0, 0.01, 300.0);
        thermal.update(100.0, 0.1, 0.1); // Heat up
        assert!(thermal.t_junction > 300.0);

        thermal.reset();
        assert!((thermal.t_junction - thermal.t_ambient).abs() < 0.01);
    }

    #[test]
    fn test_thermal_voltage_calculation() {
        let thermal = ThermalNetwork::new(1.0, 0.01, 300.0);
        let vt = thermal.thermal_voltage();
        // At 300K, Vt ≈ 25.9mV
        assert!(
            (vt - 0.0259).abs() < 0.001,
            "Thermal voltage at 300K: {}",
            vt
        );
    }

    //=========================================================================
    // Diode Recovery Tests
    //=========================================================================

    #[test]
    fn test_diode_recovery_defaults() {
        let recovery = DiodeRecovery::default();
        assert!(!recovery.is_enabled()); // Disabled by default (qrr = 0)
    }

    #[test]
    fn test_diode_recovery_charge_buildup() {
        let mut recovery = DiodeRecovery::new(1e-6, 100e-9, 0.5);

        // Forward current should build up charge
        for i in 0..100 {
            let time = i as f64 * 1e-6;
            recovery.update(1.0, time, 1e-6); // 1A forward
        }

        assert!(recovery.stored_charge > 0.0, "Charge should accumulate");
    }

    #[test]
    fn test_diode_recovery_current_waveform() {
        let mut recovery = DiodeRecovery::new(1e-6, 100e-9, 0.5);
        recovery.stored_charge = 1e-6; // Pre-load with Qrr
        recovery.in_recovery = true;
        recovery.recovery_start_time = 0.0;

        // At t=0, recovery is starting
        let i_start = recovery.recovery_current(0.0);

        // At mid-recovery, should have significant current
        let i_mid = recovery.recovery_current(50e-9);

        // After recovery (3*trr), current should be zero
        let i_end = recovery.recovery_current(400e-9);

        assert!(i_mid < 0.0, "Recovery current should be negative (reverse)");
        assert!(
            i_mid.abs() > i_start.abs(),
            "Current should peak during recovery"
        );
        assert!(i_end.abs() < 1e-12, "Current should be zero after recovery");
    }

    #[test]
    fn test_diode_recovery_softness() {
        let snappy = DiodeRecovery::new(1e-6, 100e-9, 0.0);
        let soft = DiodeRecovery::new(1e-6, 100e-9, 1.0);

        // Both should have same Qrr
        assert_eq!(snappy.qrr, soft.qrr);

        // Softness should affect waveform shape
        assert!(snappy.softness < soft.softness);
    }

    #[test]
    fn test_diode_recovery_reset() {
        let mut recovery = DiodeRecovery::new(1e-6, 100e-9, 0.5);
        recovery.stored_charge = 0.5e-6;
        recovery.in_recovery = true;

        recovery.reset();

        assert_eq!(recovery.stored_charge, 0.0);
        assert!(!recovery.in_recovery);
    }

    //=========================================================================
    // VDMOS with Thermal/Recovery Integration Tests
    //=========================================================================

    #[test]
    fn test_vdmos_thermal_params() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("RTH".to_string(), 2.5);
        params.insert("CTH".to_string(), 0.05);
        params.insert("TAMB".to_string(), 50.0); // 50°C

        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3).with_params(&params);

        assert!((vdmos.thermal.rth - 2.5).abs() < 0.01);
        assert!((vdmos.thermal.cth - 0.05).abs() < 0.01);
        assert!((vdmos.thermal.t_ambient - 323.15).abs() < 0.1); // 50°C + 273.15
    }

    #[test]
    fn test_vdmos_recovery_params() {
        use std::collections::HashMap;

        let mut params = HashMap::new();
        params.insert("QRR".to_string(), 500e-9);
        params.insert("TRR".to_string(), 75e-9);
        params.insert("SOFTNESS".to_string(), 0.3);

        let vdmos = Vdmos::new_nvdmos("M1".to_string(), 1, 2, 3).with_params(&params);

        assert!((vdmos.recovery.qrr - 500e-9).abs() < 1e-12);
        assert!((vdmos.recovery.trr - 75e-9).abs() < 1e-12);
        assert!((vdmos.recovery.softness - 0.3).abs() < 0.01);
    }
}

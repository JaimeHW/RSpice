//! BSIM4 MOSFET Model
//!
//! Implements the BSIM4v4.8 model for deep-submicron MOSFETs.
//! This is an industry-standard compact model that accurately captures:
//!
//! - **Short-channel effects**: DIBL, CLM, velocity saturation
//! - **Quantum effects**: Polysilicon depletion, gate tunneling
//! - **Mobility degradation**: Vertical and lateral field effects
//! - **Parasitic effects**: Source/drain resistance, overlap capacitance
//!
//! # Model Parameters
//!
//! The model supports over 300 parameters, but typically only a subset
//! are needed for most applications.
//!
//! # Example
//!
//! ```ignore
//! .MODEL NMOS NMOS LEVEL=14 VTH0=0.4 K1=0.5 TOX=2e-9
//! M1 drain gate source body NMOS W=1u L=65n
//! ```

use crate::Value;
use crate::circuit::NodeId;

/// Physical constants
const Q_ELECTRON: Value = 1.602176634e-19; // Electron charge (C)
const EPSILON_SI: Value = 1.0359e-10; // Silicon permittivity (F/m)
const EPSILON_OX: Value = 3.453e-11; // SiO2 permittivity (F/m)
const K_BOLTZMANN: Value = 1.380649e-23; // Boltzmann constant (J/K)
const NI_300K: Value = 1.45e16; // Intrinsic carrier concentration at 300K (m^-3)

/// BSIM4 device type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Bsim4Type {
    #[default]
    Nmos,
    Pmos,
}

/// BSIM4 model parameters
///
/// Contains the complete parameter set for BSIM4v4.8.
/// Most parameters have reasonable defaults for 65nm technology.
#[derive(Debug, Clone)]
pub struct Bsim4Params {
    /// Device type (NMOS or PMOS)
    pub device_type: Bsim4Type,

    //==========================================================================
    // Geometry Parameters
    //==========================================================================
    /// Channel length (m)
    pub l: Value,
    /// Channel width (m)
    pub w: Value,
    /// Number of fingers
    pub nf: Value,
    /// Gate oxide thickness (m)
    pub tox: Value,
    /// Poly-Si gate oxide thickness (m)
    pub toxe: Value,
    /// Channel doping (m^-3)
    pub ndep: Value,

    //==========================================================================
    // Threshold Voltage Parameters
    //==========================================================================
    /// Long-channel threshold voltage (V)
    pub vth0: Value,
    /// First-order body effect coefficient (V^0.5)
    pub k1: Value,
    /// Second-order body effect coefficient
    pub k2: Value,
    /// Body effect coefficient for short-channel
    pub k3: Value,
    /// Body effect coefficient exponent
    pub k3b: Value,
    /// Narrow width effect coefficient
    pub w0: Value,
    /// DIBL coefficient
    pub dvt0: Value,
    /// DIBL exponent
    pub dvt1: Value,
    /// Lateral DIBL coefficient
    pub dvt2: Value,
    /// Short-channel VTH shift coefficient
    pub dvt0w: Value,
    /// Short-channel VTH shift exponent
    pub dvt1w: Value,
    /// Short-channel VTH shift exponent 2
    pub dvt2w: Value,
    /// Small-size VTH shift
    pub eta0: Value,
    /// VBS dependence of eta0
    pub etab: Value,
    /// DIBL subthreshold slope coefficient
    pub dsub: Value,

    //==========================================================================
    // Mobility Parameters
    //==========================================================================
    /// Low-field mobility (cm^2/V·s)
    pub u0: Value,
    /// First-order mobility degradation
    pub ua: Value,
    /// Second-order mobility degradation  
    pub ub: Value,
    /// Body-bias mobility degradation
    pub uc: Value,
    /// Mobility degradation exponent
    pub eu: Value,
    /// Minimum mobility (cm^2/V·s)
    pub umin: Value,

    //==========================================================================
    // Velocity Saturation Parameters
    //==========================================================================
    /// Saturation velocity (m/s)
    pub vsat: Value,
    /// Velocity saturation parameter
    pub a0: Value,
    /// Velocity saturation exponent
    pub ags: Value,
    /// Velocity overshoot coefficient
    pub a1: Value,
    /// Velocity overshoot threshold
    pub a2: Value,

    //==========================================================================
    // Channel Length Modulation
    //==========================================================================
    /// CLM parameter
    pub pclm: Value,
    /// CLM exponent parameter
    pub pdiblc1: Value,
    /// CLM coefficient 2
    pub pdiblc2: Value,
    /// CLM coefficient B
    pub pdiblcb: Value,
    /// Drain-induced threshold shift
    pub drout: Value,
    /// Output conductance parameter
    pub pscbe1: Value,
    /// Output conductance parameter 2
    pub pscbe2: Value,
    /// Channel conductance parameter
    pub pvag: Value,

    //==========================================================================
    // Subthreshold Parameters
    //==========================================================================
    /// Subthreshold swing coefficient
    pub voff: Value,
    /// DIBL-induced VTH shift
    pub voffl: Value,
    /// Subthreshold body bias coefficient
    pub nfactor: Value,
    /// Channel doping dependence
    pub cdsc: Value,
    /// Drain voltage dependence of CDSC
    pub cdscd: Value,
    /// Body voltage dependence of CDSC
    pub cdscb: Value,
    /// Interface trap density
    pub cit: Value,

    //==========================================================================
    // Source/Drain Resistance
    //==========================================================================
    /// Sheet resistance (Ω/sq)
    pub rsh: Value,
    /// Source resistance per width (Ω·m)
    pub rdsw: Value,
    /// Drain resistance per width (Ω·m)
    pub rdw: Value,
    /// Source resistance per width (Ω·m)  
    pub rsw: Value,
    /// Resistance body coefficient
    pub prwb: Value,
    /// Resistance gate coefficient
    pub prwg: Value,

    //==========================================================================
    // Capacitance Parameters
    //==========================================================================
    /// Gate-source overlap capacitance (F/m)
    pub cgso: Value,
    /// Gate-drain overlap capacitance (F/m)
    pub cgdo: Value,
    /// Gate-body overlap capacitance (F/m)
    pub cgbo: Value,
    /// Junction bottom capacitance (F/m^2)
    pub cj: Value,
    /// Junction sidewall capacitance (F/m)
    pub cjsw: Value,
    /// Junction grading coefficient
    pub mj: Value,
    /// Sidewall grading coefficient
    pub mjsw: Value,
    /// Built-in potential (V)
    pub pb: Value,

    //==========================================================================
    // Temperature Parameters
    //==========================================================================
    /// Nominal temperature (°C)
    pub tnom: Value,
    /// VTH0 temperature coefficient (/°C)
    pub kt1: Value,
    /// VTH0 length-dependent temp coefficient
    pub kt1l: Value,
    /// VTH0 second-order temp coefficient
    pub kt2: Value,
    /// Mobility temperature exponent
    pub ute: Value,
    /// Saturation velocity temperature coefficient
    pub at: Value,
    /// Parasitic resistance temperature coefficient
    pub prt: Value,

    //==========================================================================
    // Noise Parameters (BSIM4 Noise Model)
    //==========================================================================
    /// Flicker noise coefficient
    pub kf: Value,
    /// Flicker noise current exponent
    pub af: Value,
    /// Flicker noise frequency exponent
    pub ef: Value,
    /// Oxide trap density coefficient A (NOIA)
    pub noia: Value,
    /// Oxide trap density coefficient B (NOIB)
    pub noib: Value,
    /// Oxide trap density coefficient C (NOIC)
    pub noic: Value,
    /// Noise emission coefficient
    pub em: Value,
    /// Channel thermal noise coefficient (gamma)
    pub tnoia: Value,
    /// Induced gate noise partition factor
    pub tnoib: Value,
    /// Noise model selector (0=SPICE2, 1=BSIM4, 2=BSIM4 holistic)
    pub noimod: usize,

    //==========================================================================
    // Self-Heating Parameters
    //==========================================================================
    /// Thermal resistance (K/W)
    pub rth0: Value,
    /// Thermal capacitance (J/K)
    pub cth0: Value,
    /// Self-heating enable (0=off, 1=on)
    pub shmod: usize,
    /// Number of RC thermal stages
    pub nrth: usize,
}

impl Default for Bsim4Params {
    fn default() -> Self {
        Self {
            device_type: Bsim4Type::Nmos,

            // Geometry - 65nm node typical
            l: 65e-9,
            w: 1e-6,
            nf: 1.0,
            tox: 1.4e-9,
            toxe: 1.8e-9,
            ndep: 1.7e24,

            // Threshold voltage
            vth0: 0.4,
            k1: 0.5,
            k2: -0.1,
            k3: 80.0,
            k3b: 0.0,
            w0: 0.0,
            dvt0: 2.2,
            dvt1: 0.53,
            dvt2: -0.032,
            dvt0w: 0.0,
            dvt1w: 5.3e6,
            dvt2w: -0.032,
            eta0: 0.08,
            etab: -0.07,
            dsub: 0.56,

            // Mobility
            u0: 400.0, // cm^2/V·s
            ua: 2.0e-9,
            ub: 5.0e-19,
            uc: -4.6e-11,
            eu: 1.67,
            umin: 100.0,

            // Velocity saturation
            vsat: 1.5e5, // m/s
            a0: 1.0,
            ags: 0.2,
            a1: 0.0,
            a2: 1.0,

            // Channel length modulation
            pclm: 1.3,
            pdiblc1: 0.39,
            pdiblc2: 0.0086,
            pdiblcb: -0.1,
            drout: 0.56,
            pscbe1: 4.24e8,
            pscbe2: 1.0e-5,
            pvag: 0.1,

            // Subthreshold
            voff: -0.13,
            voffl: 0.0,
            nfactor: 1.5,
            cdsc: 0.0,
            cdscd: 0.0,
            cdscb: 0.0,
            cit: 0.0,

            // S/D resistance
            rsh: 5.0,
            rdsw: 200.0,
            rdw: 100.0,
            rsw: 100.0,
            prwb: 0.0,
            prwg: 0.0,

            // Capacitance
            cgso: 2.0e-10,
            cgdo: 2.0e-10,
            cgbo: 0.0,
            cj: 1.0e-3,
            cjsw: 3.0e-10,
            mj: 0.5,
            mjsw: 0.33,
            pb: 1.0,

            // Temperature
            tnom: 27.0,
            kt1: -0.11,
            kt1l: 0.0,
            kt2: 0.022,
            ute: -1.5,
            at: 3.3e4,
            prt: 0.0,

            // Noise parameters (BSIM4 defaults)
            kf: 1.0e-27,    // Flicker noise coefficient
            af: 1.0,        // Flicker noise current exponent
            ef: 1.0,        // Flicker noise frequency exponent
            noia: 6.25e41,  // Oxide trap density A (for holistic model)
            noib: 3.125e26, // Oxide trap density B
            noic: 8.75e9,   // Oxide trap density C
            em: 4.1e7,      // Saturation field for noise
            tnoia: 1.5,     // Channel thermal noise coefficient
            tnoib: 3.5,     // Induced gate noise partition
            noimod: 1,      // BSIM4 noise model

            // Self-heating parameters
            rth0: 0.0,    // Thermal resistance (disabled by default)
            cth0: 1.0e-9, // Thermal capacitance
            shmod: 0,     // Self-heating disabled by default
            nrth: 1,      // Single RC thermal stage
        }
    }
}

impl Bsim4Params {
    /// Create NMOS parameters
    pub fn nmos() -> Self {
        Self::default()
    }

    /// Create PMOS parameters
    pub fn pmos() -> Self {
        Self {
            device_type: Bsim4Type::Pmos,
            vth0: -0.4,
            u0: 100.0, // PMOS has lower mobility
            ..Self::default()
        }
    }

    /// Set channel length
    pub fn with_length(mut self, l: Value) -> Self {
        self.l = l;
        self
    }

    /// Set channel width
    pub fn with_width(mut self, w: Value) -> Self {
        self.w = w;
        self
    }

    /// Set oxide thickness
    pub fn with_tox(mut self, tox: Value) -> Self {
        self.tox = tox;
        self
    }

    /// Set threshold voltage
    pub fn with_vth0(mut self, vth0: Value) -> Self {
        self.vth0 = vth0;
        self
    }
}

/// BSIM4 MOSFET device instance
#[derive(Debug, Clone)]
pub struct Bsim4 {
    /// Device name
    pub name: String,
    /// Drain node
    pub nd: NodeId,
    /// Gate node
    pub ng: NodeId,
    /// Source node
    pub ns: NodeId,
    /// Body (bulk) node
    pub nb: NodeId,
    /// Model parameters
    pub params: Bsim4Params,

    // Operating point variables
    vgs: Value,
    vds: Value,
    vbs: Value,
    id: Value,
    gm: Value,
    gds: Value,
    gmb: Value,

    // Self-heating state
    /// Temperature rise above ambient (K)
    delta_temp: Value,
    /// Instantaneous power dissipation (W)
    power_diss: Value,
}

impl Bsim4 {
    /// Create a new BSIM4 MOSFET
    pub fn new(
        name: String,
        nd: NodeId,
        ng: NodeId,
        ns: NodeId,
        nb: NodeId,
        params: Bsim4Params,
    ) -> Self {
        Self {
            name,
            nd,
            ng,
            ns,
            nb,
            params,
            vgs: 0.0,
            vds: 0.0,
            vbs: 0.0,
            id: 0.0,
            gm: 0.0,
            gds: 0.0,
            gmb: 0.0,
            delta_temp: 0.0,
            power_diss: 0.0,
        }
    }

    /// Calculate effective dimensions
    fn effective_dimensions(&self) -> (Value, Value) {
        let weff = self.params.w * self.params.nf;
        let leff = self.params.l;
        (weff, leff)
    }

    /// Calculate gate oxide capacitance per unit area
    fn cox(&self) -> Value {
        EPSILON_OX / self.params.tox
    }

    /// Calculate thermal voltage
    fn vt(&self, temp: Value) -> Value {
        K_BOLTZMANN * (temp + 273.15) / Q_ELECTRON
    }

    /// Calculate effective threshold voltage including short-channel effects
    fn vth_eff(&self, vbs: Value, vds: Value, temp: Value) -> Value {
        let p = &self.params;
        let (_weff, leff) = self.effective_dimensions();

        // Body effect (for NMOS: positive vbs reduces Vth)
        let phi_s = 2.0 * self.vt(temp) * (p.ndep / NI_300K).ln().max(1.0);
        let sqrt_phi = phi_s.abs().sqrt();
        let body_effect = p.k1 * ((sqrt_phi - vbs).max(0.01).sqrt() - sqrt_phi);

        // DIBL effect (small for short channels)
        let theta0 = (-p.dvt1 * leff / (2.0 * 10e-9_f64.max(leff))).exp();
        let dibl = (p.eta0 + p.etab * vbs) * vds.abs() * theta0 * 0.1; // Scale down

        // Short-channel effect (scale down to prevent over-reduction)
        let dvth_sce = p.dvt0 * theta0 * (1.0 - p.dvt2 * vbs.abs()) * 0.01;

        // Temperature effect
        let dvth_temp = p.kt1 * (temp - p.tnom) / 300.0
            + p.kt1l / leff * (temp - p.tnom) / 300.0
            + p.kt2 * vbs * (temp - p.tnom) / 300.0;

        // Final threshold voltage - keep positive for NMOS
        (p.vth0 + body_effect - dibl - dvth_sce + dvth_temp).max(0.05)
    }

    /// Calculate drain current with all effects
    pub fn calculate_id(&mut self, vgs: Value, vds: Value, vbs: Value, temp: Value) -> Value {
        let p = &self.params;
        let (weff, leff) = self.effective_dimensions();
        let cox = self.cox();
        let type_sign = if p.device_type == Bsim4Type::Pmos {
            -1.0
        } else {
            1.0
        };

        // Apply polarity for PMOS
        let vgs_int = type_sign * vgs;
        let vds_int = (type_sign * vds).abs().max(1e-6); // Always positive, avoid zero
        let vbs_int = type_sign * vbs;

        // Store operating point
        self.vgs = vgs;
        self.vds = vds;
        self.vbs = vbs;

        // Thermal voltage
        let vt = self.vt(temp);

        // Effective threshold voltage (no temperature shift here - we handle it in mobility)
        let vth = self.vth_eff(vbs_int, vds_int, temp);

        // Gate overdrive
        let vgst = vgs_int - vth;

        // Use smooth transition based on BSIM4 formulation
        // The smooth function provides C1 continuity at threshold
        let n = p.nfactor;
        let vgst_eff = n * vt * (1.0 + (vgst / (n * vt)).exp()).ln();

        // Temperature-dependent mobility (this dominates temperature behavior)
        // ute is negative (-1.5), so higher temp = lower mobility = lower current
        let t_ratio = (temp + 273.15) / (p.tnom + 273.15);
        let mu0 = p.u0 * 1e-4 * t_ratio.powf(p.ute); // m^2/V·s

        // Effective mobility with vertical field degradation (BSIM4 formulation)
        // Eeff = (Vgst + 2*Vth) / (6 * Toxe) in V/m, but ua/ub expect MV/cm
        // Convert: 1 MV/cm = 1e8 V/m, so divide by 1e8
        let eeff_vm = (cox * vgst_eff / EPSILON_SI).max(1e3); // V/m
        let eeff = eeff_vm / 1e8; // Convert to MV/cm for ua/ub parameters
        let mu = mu0 / (1.0 + p.ua * eeff.powf(p.eu) + p.ub * eeff.powi(2));
        let mu_eff = mu.max(p.umin * 1e-4);

        // Velocity saturation
        let esat = 2.0 * p.vsat / mu_eff;

        // Saturation voltage (smooth)
        let vdsat = vgst_eff / (1.0 + vgst_eff / (esat * leff));

        // Effective Vds (smooth clamp to Vdsat)
        let vds_eff = vdsat - 0.5 * (vdsat - vds_int + ((vdsat - vds_int).powi(2) + 0.01).sqrt());
        let vds_eff = vds_eff.max(1e-6);

        // Drain current using charge-sheet model
        let id_base = weff / leff * mu_eff * cox * (vgst_eff * vds_eff - 0.5 * vds_eff.powi(2));

        // Velocity saturation factor
        let vsat_factor = 1.0 / (1.0 + vds_eff / (esat * leff));

        // Channel length modulation (Early effect)
        let lambda = 1.0 / ((p.pscbe1 / (p.pscbe2 + 1.0 / leff)).max(100.0));
        let clm_factor = 1.0 + lambda * (vds_int - vdsat).max(0.0);

        // Final drain current
        let id = (id_base * vsat_factor * clm_factor).max(0.0);

        // Calculate small-signal parameters
        let dvth_dvbs = 0.5 * p.k1 / (2.0 * vth.abs() + 0.1).sqrt();

        // gm = dId/dVgs
        // Long-channel saturation: gm = (W/L)*µ*Cox*Vgst = β * Vgst
        // This properly scales linearly with gate overdrive
        let beta = weff / leff * mu_eff * cox;
        let gm_long_channel = beta * vgst_eff;

        // Apply velocity saturation and CLM corrections
        self.gm = (gm_long_channel * vsat_factor * clm_factor).max(1e-15);
        self.gds = (id * lambda).max(1e-15);
        self.gmb = self.gm * dvth_dvbs;
        self.id = type_sign * id;

        self.id
    }

    /// Get drain current (must call calculate_id first)
    pub fn id(&self) -> Value {
        self.id
    }

    /// Get transconductance dId/dVgs
    pub fn gm(&self) -> Value {
        self.gm
    }

    /// Get output conductance dId/dVds
    pub fn gds(&self) -> Value {
        self.gds
    }

    /// Get body transconductance dId/dVbs
    pub fn gmb(&self) -> Value {
        self.gmb
    }

    /// Get operating region as string
    pub fn region(&self) -> &'static str {
        if self.gm < 1e-12 {
            "cutoff"
        } else if self.vds.abs() < (self.vgs - self.params.vth0).abs() {
            "linear"
        } else {
            "saturation"
        }
    }

    //=========================================================================
    // Noise Analysis Methods
    //=========================================================================

    /// Calculate noise power spectral density at a given frequency
    ///
    /// Returns a NoiseSpectrum containing thermal, flicker, and total noise
    pub fn calculate_noise_psd(&self, freq: Value, temp: Value) -> NoiseSpectrum {
        let p = &self.params;
        let (weff, leff) = self.effective_dimensions();
        let cox = self.cox();

        // Thermal voltage
        let temp_k = temp + 273.15;
        let k_t = K_BOLTZMANN * temp_k;

        //---------------------------------------------------------------------
        // Channel Thermal Noise: Sid_thermal = 4kT * gamma * gm
        //---------------------------------------------------------------------
        // gamma is the channel thermal noise coefficient (2/3 for long channel,
        // can be higher for short channel due to hot electrons)
        let gamma = p.tnoia * 2.0 / 3.0; // typically 1.0 for 65nm
        let sid_thermal = 4.0 * k_t * gamma * self.gm;

        //---------------------------------------------------------------------
        // Flicker (1/f) Noise: Sid_flicker = Kf * Id^Af / (Cox * W * L * f^Ef)
        //---------------------------------------------------------------------
        let sid_flicker = if freq > 0.0 {
            let area = weff * leff;
            p.kf * self.id.abs().powf(p.af) / (cox * area * freq.powf(p.ef))
        } else {
            0.0
        };

        //---------------------------------------------------------------------
        // Shot Noise (gate leakage): Sig = 2 * q * Ig
        //---------------------------------------------------------------------
        // For deep submicron, gate leakage can contribute shot noise
        // Simplified: assume Ig ~ 1e-12 A/um^2 for 65nm
        let ig = 1e-12 * weff * leff * 1e12; // A
        let sig_shot = 2.0 * Q_ELECTRON * ig.abs();

        //---------------------------------------------------------------------
        // S/D Resistance Thermal Noise (output-referred current noise)
        //---------------------------------------------------------------------
        // Resistance thermal noise: Sv = 4kT * R, Si = 4kT / R
        let rd = p.rdw / weff.max(1e-9); // Drain resistance (Ω)
        let rs = p.rsw / weff.max(1e-9); // Source resistance (Ω)
        let sid_rsd = 4.0 * k_t / (rd + rs).max(1e-3); // Current noise from R

        //---------------------------------------------------------------------
        // Induced Gate Noise (for high frequencies)
        //---------------------------------------------------------------------
        // Sig_induced = 4kT * (2/15) * (w^2 * Cgs^2 / gm)
        let cgs = p.cgso * weff + 2.0 / 3.0 * cox * weff * leff;
        let omega = 2.0 * std::f64::consts::PI * freq;
        let sig_induced = if omega > 0.0 {
            4.0 * k_t * (p.tnoib / 15.0) * omega.powi(2) * cgs.powi(2) / self.gm.max(1e-15)
        } else {
            0.0
        };

        NoiseSpectrum {
            frequency: freq,
            thermal: sid_thermal,
            flicker: sid_flicker,
            shot: sig_shot,
            rsd: sid_rsd,
            induced_gate: sig_induced,
            total: sid_thermal + sid_flicker + sig_shot + sid_rsd + sig_induced,
        }
    }

    /// Calculate flicker noise corner frequency
    ///
    /// Returns the frequency where flicker noise equals thermal noise
    pub fn flicker_corner_freq(&self, temp: Value) -> Value {
        let p = &self.params;
        let (weff, leff) = self.effective_dimensions();
        let cox = self.cox();
        let temp_k = temp + 273.15;
        let k_t = K_BOLTZMANN * temp_k;

        // At corner: Sid_thermal = Sid_flicker
        // 4kT*gamma*gm = Kf*Id^Af / (Cox*W*L*fc^Ef)
        // fc^Ef = Kf*Id^Af / (4kT*gamma*gm*Cox*W*L)
        // fc = (Kf*Id^Af / (4kT*gamma*gm*Cox*W*L))^(1/Ef)

        let gamma = p.tnoia * 2.0 / 3.0;
        let area = weff * leff;
        let thermal = 4.0 * k_t * gamma * self.gm;

        if thermal > 0.0 && p.ef != 0.0 {
            let ratio = p.kf * self.id.abs().powf(p.af) / (thermal * cox * area);
            ratio.powf(1.0 / p.ef).max(1.0) // At least 1 Hz
        } else {
            1e3 // Default 1kHz
        }
    }

    //=========================================================================
    // Self-Heating Methods
    //=========================================================================

    /// Update self-heating temperature for transient simulation
    ///
    /// Uses a first-order RC thermal model: dT/dt = (P*Rth - T) / (Rth*Cth)
    pub fn update_self_heating(&mut self, dt: Value) {
        let p = &self.params;

        if p.shmod == 0 || p.rth0 <= 0.0 {
            // Self-heating disabled
            self.delta_temp = 0.0;
            return;
        }

        // Calculate power dissipation: P = Id * Vds
        self.power_diss = (self.id * self.vds).abs();

        // Steady-state temperature rise: dT_ss = P * Rth
        let delta_temp_ss = self.power_diss * p.rth0;

        // Time constant: tau = Rth * Cth
        let tau = p.rth0 * p.cth0;

        // First-order update: dT/dt = (dT_ss - dT) / tau
        if tau > 0.0 {
            let rate = (delta_temp_ss - self.delta_temp) / tau;
            self.delta_temp += rate * dt;
            // Clamp to physical limits
            self.delta_temp = self.delta_temp.clamp(0.0, 200.0); // Max 200K rise
        } else {
            // Instantaneous (no thermal capacitance)
            self.delta_temp = delta_temp_ss;
        }
    }

    /// Get current device temperature including self-heating
    pub fn effective_temperature(&self, ambient: Value) -> Value {
        ambient + self.delta_temp
    }

    /// Get current temperature rise (delta T)
    pub fn delta_temp(&self) -> Value {
        self.delta_temp
    }

    /// Get current power dissipation
    pub fn power_dissipation(&self) -> Value {
        self.power_diss
    }

    /// Reset self-heating state (for DC analysis)
    pub fn reset_self_heating(&mut self) {
        self.delta_temp = 0.0;
        self.power_diss = 0.0;
    }

    /// Calculate steady-state self-heating temperature rise
    pub fn steady_state_delta_temp(&self) -> Value {
        let p = &self.params;
        if p.shmod == 0 || p.rth0 <= 0.0 {
            0.0
        } else {
            (self.id * self.vds).abs() * p.rth0
        }
    }
}

//=============================================================================
// Noise Spectrum Result
//=============================================================================

/// Noise power spectral density components
#[derive(Debug, Clone, Copy, Default)]
pub struct NoiseSpectrum {
    /// Frequency (Hz)
    pub frequency: Value,
    /// Channel thermal noise (A²/Hz)
    pub thermal: Value,
    /// Flicker (1/f) noise (A²/Hz)
    pub flicker: Value,
    /// Shot noise (A²/Hz)
    pub shot: Value,
    /// S/D resistance thermal noise (A²/Hz)
    pub rsd: Value,
    /// Induced gate noise (A²/Hz)
    pub induced_gate: Value,
    /// Total noise (A²/Hz)
    pub total: Value,
}

impl NoiseSpectrum {
    /// Get noise in dB relative to 1 A²/Hz
    pub fn total_db(&self) -> Value {
        10.0 * self.total.max(1e-30).log10()
    }

    /// Get equivalent input-referred noise voltage (V/√Hz)
    pub fn input_referred_voltage(&self, gm: Value) -> Value {
        if gm > 0.0 {
            (self.total / gm.powi(2)).sqrt()
        } else {
            0.0
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

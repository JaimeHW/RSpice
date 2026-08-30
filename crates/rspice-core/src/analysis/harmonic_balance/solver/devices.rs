//! Nonlinear device parameter builders and time-domain evaluation helpers.

use super::*;

/// Largest junction exponent evaluated exactly; beyond it the exponential is
/// continued linearly so the current keeps responding to the voltage.
const MAX_EXP_ARG: Value = 40.0;

/// A non-negative value represented as `mantissa * 2^exponent` without
/// forcing the physical value through binary64's direct exponent range.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct ScaledNonnegative {
    pub(super) mantissa: Value,
    pub(super) exponent: i32,
}

impl ScaledNonnegative {
    pub(super) const ZERO: Self = Self {
        mantissa: 0.0,
        exponent: 0,
    };

    fn checked_product(factors: &[Value]) -> Result<Self, &'static str> {
        if factors
            .iter()
            .any(|factor| !factor.is_finite() || *factor < 0.0)
        {
            return Err("a noise-intensity factor is non-finite or negative");
        }
        if factors.contains(&0.0) {
            return Ok(Self::ZERO);
        }

        let mut value = Self {
            mantissa: 1.0,
            exponent: 0,
        };
        value.checked_multiply(factors)?;
        Ok(value)
    }

    fn checked_multiply(&mut self, factors: &[Value]) -> Result<(), &'static str> {
        if factors
            .iter()
            .any(|factor| !factor.is_finite() || *factor < 0.0)
        {
            return Err("a noise-intensity factor is non-finite or negative");
        }
        if self.mantissa == 0.0 || factors.contains(&0.0) {
            *self = Self::ZERO;
            return Ok(());
        }

        for &factor in factors {
            let factor_exponent = libm::ilogb(factor);
            let factor_mantissa = libm::scalbn(factor, -factor_exponent);
            self.mantissa *= factor_mantissa;
            let mantissa_exponent = libm::ilogb(self.mantissa);
            self.mantissa = libm::scalbn(self.mantissa, -mantissa_exponent);
            self.exponent = self
                .exponent
                .checked_add(factor_exponent)
                .and_then(|value| value.checked_add(mantissa_exponent))
                .ok_or("a noise-intensity product exponent exceeds this platform")?;
        }
        if !self.mantissa.is_finite() || !(1.0..2.0).contains(&self.mantissa) {
            return Err("a normalized noise-intensity product mantissa is invalid");
        }
        Ok(())
    }
}

/// Ideal-junction current and conductance `Is*(exp(v/nvt) - 1)`.
///
/// Above `MAX_EXP_ARG` the exponential is replaced by its tangent line, which
/// keeps current and conductance C1-continuous and mutually consistent; a hard
/// clamp would flatten the current while the Jacobian still reported the full
/// exponential slope, stalling Newton far from the solution. Deep reverse bias
/// needs no guard: `exp` underflows gracefully.
fn junction_current(is: Value, v: Value, nvt: Value) -> (Value, Value) {
    let arg = v / nvt;
    if arg > MAX_EXP_ARG {
        let e = MAX_EXP_ARG.exp();
        let g = is * e / nvt;
        let i = is * (e - 1.0) + g * (v - MAX_EXP_ARG * nvt);
        (i, g)
    } else {
        let e = arg.exp();
        (is * (e - 1.0), is * e / nvt)
    }
}

/// Depletion charge and capacitance of a graded junction.
///
/// SPICE convention: below the forward knee `fc*vj` the textbook power-law
/// holds; above it both charge and capacitance continue with the standard
/// linearized form, C1-continuous at the knee.
fn depletion_charge(cap: &DepletionCap, v: Value) -> (Value, Value) {
    if cap.cj0 <= 0.0 {
        return (0.0, 0.0);
    }
    let (cj0, vj, m, fc) = (cap.cj0, cap.vj, cap.m, cap.fc);
    let knee = fc * vj;

    if v < knee {
        let x = 1.0 - v / vj;
        let q = if m == 1.0 {
            -cj0 * vj * x.ln()
        } else {
            cj0 * vj / (1.0 - m) * (1.0 - x.powf(1.0 - m))
        };
        let c = cj0 * x.powf(-m);
        (q, c)
    } else {
        let f1 = if m == 1.0 {
            -vj * (1.0 - fc).ln()
        } else {
            vj / (1.0 - m) * (1.0 - (1.0 - fc).powf(1.0 - m))
        };
        let f2 = (1.0 - fc).powf(1.0 + m);
        let f3 = 1.0 - fc * (1.0 + m);
        let q = cj0 * (f1 + (f3 * (v - knee) + m / (2.0 * vj) * (v * v - knee * knee)) / f2);
        let c = cj0 * (f3 + m * v / vj) / f2;
        (q, c)
    }
}

impl NonlinearDeviceParams {
    /// Create diode parameters
    pub fn diode(is: Value, n: Value) -> Self {
        Self {
            is,
            n,
            ..Default::default()
        }
    }

    /// Create MOSFET parameters
    ///
    /// `vth` is the effective polarity-frame threshold: pass VTO for NMOS and
    /// -VTO for PMOS so depletion devices keep their sign.
    pub fn mosfet(vth: Value, kp: Value, lambda: Value) -> Self {
        Self {
            vth,
            kp,
            lambda,
            ..Default::default()
        }
    }

    /// Create JFET parameters
    pub fn jfet(vto: Value, beta: Value, lambda: Value, is: Value) -> Self {
        Self {
            vth: vto,
            kp: beta,
            lambda,
            is,
            ..Default::default()
        }
    }

    /// Create voltage-controlled switch parameters
    pub(crate) fn voltage_switch(
        vt: Value,
        vh: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
    ) -> Self {
        Self {
            vth: vt,
            vh,
            ron,
            roff,
            smooth,
            ..Default::default()
        }
    }
}

impl NonlinearDeviceInstance {
    /// Create a diode instance
    pub fn diode(anode: usize, cathode: usize, is: Value, n: Value) -> Self {
        Self {
            device_type: NonlinearDeviceType::Diode,
            terminals: vec![anode, cathode],
            params: NonlinearDeviceParams::diode(is, n),
        }
    }

    /// Create an NMOS instance (`vth` is VTO)
    pub fn nmos(
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        vth: Value,
        kp: Value,
        lambda: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Nmos,
            terminals: vec![drain, gate, source, bulk],
            params: NonlinearDeviceParams::mosfet(vth, kp, lambda),
        }
    }

    /// Create a PMOS instance (`vth` is the effective threshold, i.e. -VTO)
    pub fn pmos(
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        vth: Value,
        kp: Value,
        lambda: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![drain, gate, source, bulk],
            params: NonlinearDeviceParams::mosfet(vth, kp, lambda),
        }
    }

    /// Create an N-channel JFET instance
    pub fn njfet(
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
        is: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Njfet,
            terminals: vec![drain, gate, source],
            params: NonlinearDeviceParams::jfet(vto, beta, lambda, is),
        }
    }

    /// Create a P-channel JFET instance
    pub fn pjfet(
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
        is: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Pjfet,
            terminals: vec![drain, gate, source],
            params: NonlinearDeviceParams::jfet(vto, beta, lambda, is),
        }
    }

    /// Create a voltage-controlled switch instance
    pub(crate) fn voltage_switch(
        node_pos: usize,
        node_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        vt: Value,
        vh: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::VoltageSwitch,
            terminals: vec![node_pos, node_neg, ctrl_pos, ctrl_neg],
            params: NonlinearDeviceParams::voltage_switch(vt, vh, ron, roff, smooth),
        }
    }

    /// Evaluate device current given terminal voltages
    /// Returns Vec of (node_index, current) pairs - current flowing INTO each node
    pub fn evaluate(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        match self.device_type {
            NonlinearDeviceType::Diode => self.eval_diode(node_voltages),
            NonlinearDeviceType::Nmos => self.eval_nmos(node_voltages),
            NonlinearDeviceType::Pmos => self.eval_pmos(node_voltages),
            NonlinearDeviceType::Njfet => self.eval_njfet(node_voltages),
            NonlinearDeviceType::Pjfet => self.eval_pjfet(node_voltages),
            NonlinearDeviceType::VoltageSwitch => self.eval_voltage_switch(node_voltages),
        }
    }

    /// Compute Jacobian entries (∂I/∂V for each terminal pair)
    /// Returns Vec of ((from_node, to_node), dI/dV) - linearized conductance stamps
    pub fn jacobian(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        match self.device_type {
            NonlinearDeviceType::Diode => self.jac_diode(node_voltages),
            NonlinearDeviceType::Nmos => self.jac_nmos(node_voltages),
            NonlinearDeviceType::Pmos => self.jac_pmos(node_voltages),
            NonlinearDeviceType::Njfet => self.jac_njfet(node_voltages),
            NonlinearDeviceType::Pjfet => self.jac_pjfet(node_voltages),
            NonlinearDeviceType::VoltageSwitch => self.jac_voltage_switch(node_voltages),
        }
    }

    /// Noise branch terminal pairs, ordered to match `noise_intensities`:
    /// the device injects a white current-noise source between each pair
    /// whose intensity is periodically modulated by the operating point.
    pub(crate) fn noise_branches(&self) -> Vec<(usize, usize)> {
        match self.device_type {
            NonlinearDeviceType::Diode => vec![(self.terminals[0], self.terminals[1])],
            NonlinearDeviceType::Nmos | NonlinearDeviceType::Pmos => {
                vec![(self.terminals[0], self.terminals[2])] // channel thermal
            }
            NonlinearDeviceType::Njfet | NonlinearDeviceType::Pjfet => {
                vec![(self.terminals[0], self.terminals[2])] // channel thermal
            }
            NonlinearDeviceType::VoltageSwitch => {
                vec![(self.terminals[0], self.terminals[1])] // ON-resistance thermal
            }
        }
    }

    /// Human-readable mechanism label for one `noise_branches` entry.
    pub(crate) fn noise_branch_label(&self, _branch: usize) -> &'static str {
        match self.device_type {
            NonlinearDeviceType::Diode => "shot",
            NonlinearDeviceType::Nmos
            | NonlinearDeviceType::Pmos
            | NonlinearDeviceType::Njfet
            | NonlinearDeviceType::Pjfet => "channel thermal",
            NonlinearDeviceType::VoltageSwitch => "ron thermal",
        }
    }

    /// Instantaneous white-noise intensities s(t) >= 0 in A^2/Hz for each
    /// branch of `noise_branches`, evaluated at one time sample: shot noise
    /// `2q|I|` for junction and transport currents, channel thermal
    /// `(8/3)kT|gm|` for legacy MOS/JFET channels, and `4kT g(t)` for switch
    /// resistance. Numerical conductance floors and output conductance are not
    /// physical channel-noise generators in these Level-1 models.
    pub(super) fn noise_intensities(
        &self,
        node_voltages: &[Value],
        temperature: Value,
        q_e: Value,
        k_b: Value,
    ) -> Result<Vec<ScaledNonnegative>, &'static str> {
        match self.device_type {
            NonlinearDeviceType::Diode => {
                let v_a = self.get_terminal_voltage(node_voltages, 0);
                let v_c = self.get_terminal_voltage(node_voltages, 1);
                let (id, _) =
                    junction_current(self.params.is, v_a - v_c, self.params.n * self.params.vt);
                Ok(vec![ScaledNonnegative::checked_product(&[
                    2.0,
                    q_e,
                    id.abs(),
                ])?])
            }
            NonlinearDeviceType::Nmos => {
                let (_, _, _, gm, _, _) = self.mos_operating_point(1.0, node_voltages);
                Ok(vec![ScaledNonnegative::checked_product(&[
                    8.0 / 3.0,
                    k_b,
                    temperature,
                    gm.abs(),
                ])?])
            }
            NonlinearDeviceType::Pmos => {
                let (_, _, _, gm, _, _) = self.mos_operating_point(-1.0, node_voltages);
                Ok(vec![ScaledNonnegative::checked_product(&[
                    8.0 / 3.0,
                    k_b,
                    temperature,
                    gm.abs(),
                ])?])
            }
            NonlinearDeviceType::Njfet => {
                let (_, gm, _) = self.jfet_ids_gm_gds(node_voltages, 1.0);
                Ok(vec![ScaledNonnegative::checked_product(&[
                    8.0 / 3.0,
                    k_b,
                    temperature,
                    gm.abs(),
                ])?])
            }
            NonlinearDeviceType::Pjfet => {
                let (_, gm, _) = self.jfet_ids_gm_gds(node_voltages, -1.0);
                Ok(vec![ScaledNonnegative::checked_product(&[
                    8.0 / 3.0,
                    k_b,
                    temperature,
                    gm.abs(),
                ])?])
            }
            NonlinearDeviceType::VoltageSwitch => {
                let vcp = self.get_terminal_voltage(node_voltages, 2);
                let vcn = self.get_terminal_voltage(node_voltages, 3);
                let (g, _) = self.switch_conductance_and_derivative(vcp - vcn);
                Ok(vec![ScaledNonnegative::checked_product(&[
                    4.0,
                    k_b,
                    temperature,
                    g,
                ])?])
            }
        }
    }

    /// Configure the MOSFET intrinsic gate capacitance: `cox_wl` is the
    /// total oxide capacitance Cox' * W * Leff. Enables the
    /// charge-conserving square-law channel charge (Ward-Dutton partition)
    /// and the gate-bulk accumulation/depletion wedge.
    pub(crate) fn with_intrinsic_gate(mut self, cox_wl: Value) -> Self {
        self.params.cox_wl = cox_wl;
        self
    }

    /// Configure the MOSFET bulk junctions: source-bulk on `cap_a`/`is`,
    /// drain-bulk on `cap_b`/`is2`. The junctions conduct (Shockley with the
    /// shared linear continuation) and store depletion charge; sidewall
    /// capacitance is folded into the zero-bias values at the bottom
    /// grading coefficient (separate MJSW treatment pending).
    pub(crate) fn with_bulk_junctions(
        mut self,
        cap_sb: DepletionCap,
        cap_db: DepletionCap,
        is_s: Value,
        is_d: Value,
    ) -> Self {
        self.params.cap_a = cap_sb;
        self.params.cap_b = cap_db;
        self.params.is = is_s;
        self.params.is2 = is_d;
        self
    }

    /// Set the MOSFET body-effect parameters: threshold shifts by
    /// `gamma*(sqrt(phi + vsb) - sqrt(phi))` with the source-bulk voltage
    /// measured in the polarity frame from the effective source.
    pub(crate) fn with_body_effect(mut self, gamma: Value, phi: Value) -> Self {
        self.params.gamma = gamma;
        self.params.phi = phi;
        self
    }

    /// Set the thermal voltage kT/q the junction laws evaluate at; device
    /// structs carry it temperature-adjusted, so passing it through keeps HB
    /// at the same operating temperature as the rest of the engine.
    pub(crate) fn with_thermal_voltage(mut self, vt: Value) -> Self {
        if vt.is_finite() && vt > 0.0 {
            self.params.vt = vt;
        }
        self
    }

    /// Attach junction charge parameters: `cap_a` is the diode or JFET G-S
    /// junction, `cap_b` is the optional JFET G-D junction, and
    /// `transit_time` is diode diffusion transit time.
    pub(crate) fn with_junction_caps(
        mut self,
        cap_a: DepletionCap,
        cap_b: DepletionCap,
        transit_time: Value,
    ) -> Self {
        self.params.cap_a = cap_a;
        self.params.cap_b = cap_b;
        self.params.tt_f = transit_time;
        self
    }

    /// Whether this device stores charge (junction or diffusion capacitance).
    pub(crate) fn has_charge_storage(&self) -> bool {
        self.params.cap_a.cj0 > 0.0
            || self.params.cap_b.cj0 > 0.0
            || self.params.tt_f > 0.0
            || self.params.cox_wl > 0.0
    }

    /// Stored charge delivered INTO each node.
    ///
    /// Same sign convention as `evaluate`: the returned value is minus the
    /// charge absorbed at the terminal, so the capacitive current delivered
    /// into the node is d/dt of the returned charge. Devices without charge
    /// storage return an empty vector.
    pub fn charge(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        if !self.has_charge_storage() {
            return Vec::new();
        }
        match self.device_type {
            NonlinearDeviceType::Diode => self.charge_diode(node_voltages),
            NonlinearDeviceType::Nmos => self.charge_mos(1.0, node_voltages),
            NonlinearDeviceType::Pmos => self.charge_mos(-1.0, node_voltages),
            NonlinearDeviceType::Njfet => self.charge_jfet(1.0, node_voltages),
            NonlinearDeviceType::Pjfet => self.charge_jfet(-1.0, node_voltages),
            _ => Vec::new(),
        }
    }

    /// Capacitance stamps: derivative of the charge ABSORBED at each
    /// terminal with respect to node voltage, mirroring the `jacobian`
    /// conductance-stamp convention.
    pub(crate) fn charge_jacobian(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        if !self.has_charge_storage() {
            return Vec::new();
        }
        match self.device_type {
            NonlinearDeviceType::Diode => self.cap_diode(node_voltages),
            NonlinearDeviceType::Nmos => self.cap_mos(1.0, node_voltages),
            NonlinearDeviceType::Pmos => self.cap_mos(-1.0, node_voltages),
            NonlinearDeviceType::Njfet => self.cap_jfet(1.0, node_voltages),
            NonlinearDeviceType::Pjfet => self.cap_jfet(-1.0, node_voltages),
            _ => Vec::new(),
        }
    }

    fn validate_depletion_cap(cap: DepletionCap) -> Result<(), &'static str> {
        if !cap.cj0.is_finite() || cap.cj0 < 0.0 {
            return Err("junction CJ0 must be finite and nonnegative");
        }
        if !cap.vj.is_finite() || cap.vj <= 0.0 {
            return Err("junction VJ must be finite and positive");
        }
        if !cap.m.is_finite() || !(0.0..=1.0).contains(&cap.m) {
            return Err("junction grading coefficient must be finite in [0, 1]");
        }
        if !cap.fc.is_finite() || !(0.0..1.0).contains(&cap.fc) {
            return Err("junction FC must be finite in [0, 1)");
        }
        Ok(())
    }

    fn physical_parameter_error(&self) -> Option<&'static str> {
        if let Err(reason) = Self::validate_depletion_cap(self.params.cap_a) {
            return Some(reason);
        }
        if let Err(reason) = Self::validate_depletion_cap(self.params.cap_b) {
            return Some(reason);
        }
        if !self.params.cox_wl.is_finite() || self.params.cox_wl < 0.0 {
            return Some("intrinsic gate capacitance must be finite and nonnegative");
        }
        if !self.params.tt_f.is_finite() || self.params.tt_f < 0.0 {
            return Some("transit time must be finite and nonnegative");
        }
        match self.device_type {
            NonlinearDeviceType::Diode => {
                if !self.params.is.is_finite() || self.params.is < 0.0 {
                    return Some("diode IS must be finite and nonnegative");
                }
                if !self.params.n.is_finite() || self.params.n <= 0.0 {
                    return Some("diode N must be finite and positive");
                }
                if !self.params.vt.is_finite() || self.params.vt <= 0.0 {
                    return Some("diode thermal voltage must be finite and positive");
                }
                let nvt = self.params.n * self.params.vt;
                if !nvt.is_finite() || nvt <= 0.0 {
                    return Some("diode N*VT must be finite and positive");
                }
            }
            NonlinearDeviceType::Nmos | NonlinearDeviceType::Pmos => {
                if !self.params.vth.is_finite() {
                    return Some("MOS threshold must be finite");
                }
                if !self.params.kp.is_finite() || self.params.kp < 0.0 {
                    return Some("MOS KP must be finite and nonnegative");
                }
                if !self.params.lambda.is_finite() || self.params.lambda < 0.0 {
                    return Some("MOS LAMBDA must be finite and nonnegative");
                }
                if !self.params.gamma.is_finite() || self.params.gamma < 0.0 {
                    return Some("MOS GAMMA must be finite and nonnegative");
                }
                if !self.params.phi.is_finite() || self.params.phi <= 0.0 {
                    return Some("MOS PHI must be finite and positive");
                }
                if !self.params.is.is_finite() || self.params.is < 0.0 {
                    return Some("MOS source-bulk IS must be finite and nonnegative");
                }
                if !self.params.is2.is_finite() || self.params.is2 < 0.0 {
                    return Some("MOS drain-bulk IS must be finite and nonnegative");
                }
                if !self.params.vt.is_finite() || self.params.vt <= 0.0 {
                    return Some("MOS thermal voltage must be finite and positive");
                }
            }
            NonlinearDeviceType::Njfet | NonlinearDeviceType::Pjfet => {
                if !self.params.vth.is_finite() {
                    return Some("JFET VTO must be finite");
                }
                if !self.params.kp.is_finite() || self.params.kp < 0.0 {
                    return Some("JFET BETA must be finite and nonnegative");
                }
                if !self.params.lambda.is_finite() || self.params.lambda < 0.0 {
                    return Some("JFET LAMBDA must be finite and nonnegative");
                }
                if !self.params.is.is_finite() || self.params.is < 0.0 {
                    return Some("JFET IS must be finite and nonnegative");
                }
                if !self.params.vt.is_finite() || self.params.vt <= 0.0 {
                    return Some("JFET thermal voltage must be finite and positive");
                }
            }
            NonlinearDeviceType::VoltageSwitch => {
                if let Some(reason) = self.voltage_switch_parameter_error() {
                    return Some(reason);
                }
            }
        }
        None
    }

    fn voltage_switch_parameter_error(&self) -> Option<&'static str> {
        let p = &self.params;
        if !p.vth.is_finite() {
            return Some("voltage-switch threshold must be finite");
        }
        if !p.vh.is_finite() || p.vh != 0.0 {
            return Some("voltage-switch exact HB requires zero finite hysteresis");
        }
        if !p.smooth.is_finite() || p.smooth <= 0.0 {
            return Some("voltage-switch SMOOTH must be finite and positive");
        }
        for (resistance, resistance_error, conductance_error) in [
            (
                p.ron,
                "voltage-switch RON must be finite and positive",
                "voltage-switch 1/RON must be finite and positive",
            ),
            (
                p.roff,
                "voltage-switch ROFF must be finite and positive",
                "voltage-switch 1/ROFF must be finite and positive",
            ),
        ] {
            if !resistance.is_finite() || resistance <= 0.0 {
                return Some(resistance_error);
            }
            let conductance = 1.0 / resistance;
            if !conductance.is_finite() || conductance <= 0.0 {
                return Some(conductance_error);
            }
        }

        let log_ron = p.ron.ln();
        let log_roff = p.roff.ln();
        let m = 0.5 * log_ron + 0.5 * log_roff;
        let h = 0.5 * (log_roff - log_ron).abs();
        if h > 0.0 {
            let z = h / (h.hypot(1.0) + 1.0);
            let log_max_slope = -m + h * z + h.ln() - p.smooth.ln() + (-z * z).ln_1p();
            let max_slope = log_max_slope.exp();
            if !log_max_slope.is_finite()
                || log_max_slope > Value::MAX.ln()
                || !max_slope.is_finite()
                || max_slope <= 0.0
            {
                return Some("voltage-switch maximum |dg/dVctrl| must be finite and representable");
            }
        }
        None
    }

    // --- Private evaluation methods ---

    fn get_terminal_voltage(&self, node_voltages: &[Value], terminal_idx: usize) -> Value {
        let node = self.terminals.get(terminal_idx).copied().unwrap_or(0);
        node_voltages.get(node).copied().unwrap_or(0.0)
    }

    fn eval_diode(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let v_a = self.get_terminal_voltage(node_voltages, 0);
        let v_c = self.get_terminal_voltage(node_voltages, 1);
        let vd = v_a - v_c;

        let (id, _) = junction_current(self.params.is, vd, self.params.n * self.params.vt);

        // Return current contribution to each node equation (current INTO node convention)
        // Diode current id flows FROM anode TO cathode
        // So current INTO anode = -id, current INTO cathode = +id
        vec![
            (self.terminals[0], -id), // Current INTO anode (negative = leaving)
            (self.terminals[1], id),  // Current INTO cathode (positive = entering)
        ]
    }

    fn jac_diode(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_a = self.get_terminal_voltage(node_voltages, 0);
        let v_c = self.get_terminal_voltage(node_voltages, 1);
        let vd = v_a - v_c;

        let (_, gd) = junction_current(self.params.is, vd, self.params.n * self.params.vt);

        let a = self.terminals[0];
        let c = self.terminals[1];

        // Return MNA conductance stamp
        // Physical conductance gd is POSITIVE. MNA stamp for 2-terminal conductance:
        // G[n+,n+] += gd, G[n+,n-] -= gd, G[n-,n+] -= gd, G[n-,n-] += gd
        vec![((a, a), gd), ((a, c), -gd), ((c, a), -gd), ((c, c), gd)]
    }

    /// Level-1 MOSFET core in the polarity frame after drain/source swap.
    ///
    /// `vgs`/`vds` are effective (polarity-resolved, `vds >= 0`) values;
    /// channel-length modulation applies in triode and saturation so the
    /// current and its derivatives are continuous across the region boundary
    /// (ngspice MOS1 convention).
    fn mos_ids(&self, vgs: Value, vds: Value, vth: Value) -> (Value, Value, Value) {
        let kp = self.params.kp;
        let lambda = self.params.lambda;
        let vov = vgs - vth;

        if vov <= 0.0 {
            (0.0, 0.0, 0.0)
        } else if vds < vov {
            // Triode
            let clm = 1.0 + lambda * vds;
            let ids = kp * (vov * vds - 0.5 * vds * vds) * clm;
            let gm = kp * vds * clm;
            let gds = kp * (vov - vds) * clm + kp * (vov * vds - 0.5 * vds * vds) * lambda;
            (ids, gm, gds)
        } else {
            // Saturation
            let clm = 1.0 + lambda * vds;
            let ids = 0.5 * kp * vov * vov * clm;
            let gm = kp * vov * clm;
            let gds = 0.5 * kp * vov * vov * lambda;
            (ids, gm, gds)
        }
    }

    /// Resolve the effective drain/source orientation and operating point.
    ///
    /// Returns `(eff_d, eff_s, ids, gm, gds, gmbs)` in the swapped polarity
    /// frame; the current absorbed at `eff_d` is `p * ids`. The threshold
    /// carries the body effect `vth = vto + gamma*(sqrt(phi + vsb) -
    /// sqrt(phi))` with `vsb` measured from the EFFECTIVE source (the swap
    /// keeps the device symmetric), clamped at full depletion; `gmbs =
    /// -dIds/dVth * dVth/dVsb` is the bulk transconductance.
    fn mos_operating_point(
        &self,
        p: Value,
        node_voltages: &[Value],
    ) -> (usize, usize, Value, Value, Value, Value) {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);
        let v_b = self.get_terminal_voltage(node_voltages, 3);

        let d = self.terminals[0];
        let s = self.terminals[2];

        // Symmetric device: swap drain/source when the effective Vds is negative.
        let (vgs, vds, vsb, eff_d, eff_s) = if p * (v_d - v_s) >= 0.0 {
            (p * (v_g - v_s), p * (v_d - v_s), p * (v_s - v_b), d, s)
        } else {
            (p * (v_g - v_d), p * (v_s - v_d), p * (v_d - v_b), s, d)
        };

        let (vth, dvth_dvsb) = self.mos_threshold(vsb);

        let (ids, gm, gds) = self.mos_ids(vgs, vds, vth);
        let gmbs = gm * dvth_dvsb;
        (eff_d, eff_s, ids, gm, gds, gmbs)
    }

    /// Body-effect threshold law and its derivative with respect to the
    /// effective source-bulk voltage.
    fn mos_threshold(&self, vsb: Value) -> (Value, Value) {
        let gamma = self.params.gamma;
        let phi = self.params.phi;
        if gamma > 0.0 {
            let arg = phi + vsb;
            if arg > 0.0 {
                let sqrt_arg = arg.sqrt();
                (
                    self.params.vth + gamma * (sqrt_arg - phi.sqrt()),
                    gamma / (2.0 * sqrt_arg),
                )
            } else {
                // Full depletion clamp: threshold pinned, no bulk control.
                (self.params.vth - gamma * phi.sqrt(), 0.0)
            }
        } else {
            (self.params.vth, 0.0)
        }
    }

    /// Bulk junction voltages in the polarity frame (forward when the bulk
    /// diode conducts): v_j = p * (v_b - v_terminal), tied to the PHYSICAL
    /// source/drain terminals (the channel swap does not move diffusions).
    fn mos_bulk_junctions(&self, p: Value, node_voltages: &[Value]) -> (Value, Value) {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_s = self.get_terminal_voltage(node_voltages, 2);
        let v_b = self.get_terminal_voltage(node_voltages, 3);
        (p * (v_b - v_s), p * (v_b - v_d))
    }

    fn eval_mos(&self, p: Value, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (eff_d, eff_s, ids, _, _, _) = self.mos_operating_point(p, node_voltages);
        let absorbed = p * ids; // Current absorbed at the effective drain.
        let mut out = vec![(eff_d, -absorbed), (eff_s, absorbed)];

        // Bulk diode conduction (normally reverse biased; matters when the
        // body forward-biases). Current flows bulk -> terminal when v_j > 0.
        let (vj_sb, vj_db) = self.mos_bulk_junctions(p, node_voltages);
        let (i_sb, _) = junction_current(self.params.is, vj_sb, self.params.vt);
        let (i_db, _) = junction_current(self.params.is2, vj_db, self.params.vt);
        let b = self.terminals[3];
        let d = self.terminals[0];
        let s = self.terminals[2];
        out.push((b, -p * (i_sb + i_db)));
        out.push((s, p * i_sb));
        out.push((d, p * i_db));
        out
    }

    fn jac_mos(&self, p: Value, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (eff_d, eff_s, _, gm, gds, gmbs) = self.mos_operating_point(p, node_voltages);
        let g = self.terminals[1];
        let b = self.terminals[3];

        // The polarity factors cancel (p^2 = 1): the node-space stamps are the
        // textbook MOS pattern in the effective frame for NMOS and PMOS alike.
        // Ids falls as Vsb rises (gmbs acts like a back-gate gm controlled by
        // the effective source-bulk voltage), so the source column carries
        // the full -(gm + gds + gmbs) dependence and the bulk column +gmbs.
        let mut stamps = vec![
            ((eff_d, eff_d), gds),
            ((eff_d, g), gm),
            ((eff_d, b), gmbs),
            ((eff_d, eff_s), -(gm + gds + gmbs)),
            ((eff_s, eff_d), -gds),
            ((eff_s, g), -gm),
            ((eff_s, b), -gmbs),
            ((eff_s, eff_s), gm + gds + gmbs),
        ];

        // Bulk diode conductances on the physical terminals.
        let (vj_sb, vj_db) = self.mos_bulk_junctions(p, node_voltages);
        let (_, g_sb) = junction_current(self.params.is, vj_sb, self.params.vt);
        let (_, g_db) = junction_current(self.params.is2, vj_db, self.params.vt);
        let d = self.terminals[0];
        let s = self.terminals[2];
        stamps.extend_from_slice(&[
            ((b, b), g_sb + g_db),
            ((b, s), -g_sb),
            ((s, b), -g_sb),
            ((s, s), g_sb),
            ((b, d), -g_db),
            ((d, b), -g_db),
            ((d, d), g_db),
        ]);
        stamps
    }

    fn eval_nmos(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        self.eval_mos(1.0, node_voltages)
    }

    fn jac_nmos(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        self.jac_mos(1.0, node_voltages)
    }

    fn eval_pmos(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        self.eval_mos(-1.0, node_voltages)
    }

    fn jac_pmos(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        self.jac_mos(-1.0, node_voltages)
    }

    fn jfet_ids_gm_gds(&self, node_voltages: &[Value], polarity: Value) -> (Value, Value, Value) {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let vgs = v_g - v_s;
        let vds = v_d - v_s;
        let vgs_int = polarity * vgs;
        let vds_int = polarity * vds;
        let vto = self.params.vth;
        let beta = self.params.kp;
        let lambda = self.params.lambda;
        let vgst = vgs_int - vto;

        let (ids_int, gm, gds) = if vgst <= 0.0 {
            (0.0, 0.0, 0.0)
        } else if vds_int < 0.0 {
            // Reverse operation: swap effective drain/source orientation.
            let vds_rev = -vds_int;
            let vgs_rev = vgs_int - vds_int;
            let vgst_rev = vgs_rev - vto;

            if vgst_rev <= 0.0 {
                (0.0, 0.0, 0.0)
            } else if vds_rev <= vgst_rev {
                let ids_fwd = beta
                    * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev)
                    * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vds_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * 2.0 * (vgst_rev - vds_rev) * (1.0 + lambda * vds_rev)
                    + beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            } else {
                let ids_fwd = beta * vgst_rev * vgst_rev * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vgst_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * vgst_rev * vgst_rev * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            }
        } else if vds_int <= vgst {
            let ids = beta * (2.0 * vgst * vds_int - vds_int * vds_int) * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vds_int * (1.0 + lambda * vds_int);
            let gds = beta * 2.0 * (vgst - vds_int) * (1.0 + lambda * vds_int)
                + beta * (2.0 * vgst * vds_int - vds_int * vds_int) * lambda;
            (ids, gm, gds)
        } else {
            let ids = beta * vgst * vgst * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vgst * (1.0 + lambda * vds_int);
            let gds = beta * vgst * vgst * lambda;
            (ids, gm, gds)
        };

        (polarity * ids_int, gm, gds)
    }

    /// Gate junction currents `(igs, ggs, igd, ggd)` in the polarity frame.
    ///
    /// The SPICE JFET model conducts through the gate-source and gate-drain
    /// junction diodes once they are driven into forward bias; ignoring them
    /// leaves the gate node floating in self-biased large-signal circuits.
    fn jfet_gate_junctions(
        &self,
        node_voltages: &[Value],
        polarity: Value,
    ) -> (Value, Value, Value, Value) {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let (igs, ggs) = junction_current(self.params.is, polarity * (v_g - v_s), self.params.vt);
        let (igd, ggd) = junction_current(self.params.is, polarity * (v_g - v_d), self.params.vt);
        (igs, ggs, igd, ggd)
    }

    fn eval_jfet(&self, polarity: Value, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (id, _, _) = self.jfet_ids_gm_gds(node_voltages, polarity);
        let (igs, _, igd, _) = self.jfet_gate_junctions(node_voltages, polarity);
        vec![
            (self.terminals[0], -id + polarity * igd), // Channel out, gate-drain junction in
            (self.terminals[1], -polarity * (igs + igd)), // Gate junction current out
            (self.terminals[2], id + polarity * igs),  // Channel in, gate-source junction in
        ]
    }

    fn jac_jfet(&self, polarity: Value, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (_, gm, gds) = self.jfet_ids_gm_gds(node_voltages, polarity);
        let (_, ggs, _, ggd) = self.jfet_gate_junctions(node_voltages, polarity);
        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];
        vec![
            // Channel: textbook FET stamps; the source row carries the full
            // -(gm + gds) dependence mirrored from the drain row.
            ((d, d), gds),
            ((d, g), gm),
            ((d, s), -(gds + gm)),
            ((s, d), -gds),
            ((s, g), -gm),
            ((s, s), gds + gm),
            // Gate-source junction (polarity factors cancel in node space).
            ((g, g), ggs),
            ((g, s), -ggs),
            ((s, g), -ggs),
            ((s, s), ggs),
            // Gate-drain junction.
            ((g, g), ggd),
            ((g, d), -ggd),
            ((d, g), -ggd),
            ((d, d), ggd),
        ]
    }

    fn eval_njfet(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        self.eval_jfet(1.0, node_voltages)
    }

    fn jac_njfet(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        self.jac_jfet(1.0, node_voltages)
    }

    fn eval_pjfet(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        self.eval_jfet(-1.0, node_voltages)
    }

    fn jac_pjfet(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        self.jac_jfet(-1.0, node_voltages)
    }

    fn switch_conductance_and_derivative(&self, vctrl: Value) -> (Value, Value) {
        // HB uses a memoryless smooth switch characteristic. Hysteresis parameter
        // is retained in params for compatibility with shared model cards.
        let smooth = self.params.smooth;
        let ron = self.params.ron;
        let roff = self.params.roff;
        if !vctrl.is_finite()
            || !smooth.is_finite()
            || smooth <= 0.0
            || !ron.is_finite()
            || ron <= 0.0
            || !roff.is_finite()
            || roff <= 0.0
        {
            // Native HB residual/Jacobian validation turns these sentinels into
            // a typed solve error. Do not replace invalid authored parameters
            // with a different physical switch law.
            return (Value::NAN, Value::NAN);
        }
        let x = (vctrl - self.params.vth) / smooth;
        let tanh_x = x.tanh();
        let f = 0.5 * (1.0 - tanh_x);

        let log_ron = ron.ln();
        let log_roff = roff.ln();
        let dlog_r = log_roff - log_ron;
        let log_r = log_ron + dlog_r * f;
        let g = (-log_r).exp();

        let sech2 = 1.0 - tanh_x * tanh_x;
        let dg_dvctrl = if dlog_r == 0.0 || sech2 == 0.0 {
            0.0
        } else {
            let log_abs_derivative =
                -log_r + dlog_r.abs().ln() + sech2.ln() - std::f64::consts::LN_2 - smooth.ln();
            log_abs_derivative.exp().copysign(dlog_r)
        };

        (g, dg_dvctrl)
    }

    fn eval_voltage_switch(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vctrl = vcp - vcn;
        let vmain = vp - vn;
        let (g, _) = self.switch_conductance_and_derivative(vctrl);
        let i = g * vmain;
        vec![(self.terminals[0], -i), (self.terminals[1], i)]
    }

    fn jac_voltage_switch(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vctrl = vcp - vcn;
        let vmain = vp - vn;
        let (g, dg_dvctrl) = self.switch_conductance_and_derivative(vctrl);
        let g_ctrl = dg_dvctrl * vmain;

        let p = self.terminals[0];
        let n = self.terminals[1];
        let cp = self.terminals[2];
        let cn = self.terminals[3];

        vec![
            ((p, p), g),
            ((p, n), -g),
            ((n, p), -g),
            ((n, n), g),
            ((p, cp), g_ctrl),
            ((p, cn), -g_ctrl),
            ((n, cp), -g_ctrl),
            ((n, cn), g_ctrl),
        ]
    }

    /// Diode stored charge: depletion plus diffusion `TT * Id`.
    fn charge_diode(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let v_a = self.get_terminal_voltage(node_voltages, 0);
        let v_c = self.get_terminal_voltage(node_voltages, 1);
        let vd = v_a - v_c;

        let (q_dep, _) = depletion_charge(&self.params.cap_a, vd);
        let (id, _) = junction_current(self.params.is, vd, self.params.n * self.params.vt);
        let q = q_dep + self.params.tt_f * id;

        vec![(self.terminals[0], -q), (self.terminals[1], q)]
    }

    fn cap_diode(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_a = self.get_terminal_voltage(node_voltages, 0);
        let v_c = self.get_terminal_voltage(node_voltages, 1);
        let vd = v_a - v_c;

        let (_, c_dep) = depletion_charge(&self.params.cap_a, vd);
        let (_, gd) = junction_current(self.params.is, vd, self.params.n * self.params.vt);
        let c = c_dep + self.params.tt_f * gd;

        let a = self.terminals[0];
        let k = self.terminals[1];
        vec![((a, a), c), ((a, k), -c), ((k, a), -c), ((k, k), c)]
    }

    /// Smoothing half-width (volts) for the effective overdrive in the
    /// channel-charge model: keeps the charges C-infinity through threshold
    /// and is asymptotically exact a few hundred microvolts above it.
    const MOS_QSMOOTH: Value = 1e-4;

    /// Charge-conserving square-law channel charge (Ward-Dutton partition)
    /// plus the Meyer-style gate-bulk accumulation/depletion wedge.
    ///
    /// Integrating the gradual-channel inversion charge exactly, with
    /// a = smoothed vgst and b = a - min(vds, a):
    ///
    ///   Qg = (2/3) Cox (a^2 + ab + b^2) / (a + b)
    ///   Qd = -(2 Cox / 15) (2a^3 + 4a^2 b + 6ab^2 + 3b^3) / (a + b)^2
    ///   Qs = -Qg - Qd
    ///
    /// recovers Cox*vgst at vds = 0, (2/3) Cox*vgst and the 40/60
    /// Ward-Dutton drain/source split in saturation, and is C1 across the
    /// saturation boundary (dQd/db vanishes at b = 0). The transient engine
    /// integrates the Meyer capacitances instead (ngspice MOS1); both agree
    /// in the canonical strong-inversion and accumulation limits, but HB's
    /// spectral residual requires true charges, so the non-integrable Meyer
    /// model cannot be used here.
    ///
    /// Returns the node ids `[eff_d, g, eff_s, b]`, the ABSORBED charge at
    /// each, and the exact Jacobian d(absorbed at row)/d(node voltage at
    /// column) in the same ordering.
    fn mos_intrinsic_charge_state(
        &self,
        p: Value,
        node_voltages: &[Value],
    ) -> ([usize; 4], [Value; 4], [[Value; 4]; 4]) {
        let cox = self.params.cox_wl;
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);
        let v_b = self.get_terminal_voltage(node_voltages, 3);

        // Channel swap mirrors mos_operating_point: the charge model lives
        // in the effective frame, so the partition follows the carrier flow.
        let (v_ed, v_es, ed, es) = if p * (v_d - v_s) >= 0.0 {
            (v_d, v_s, self.terminals[0], self.terminals[2])
        } else {
            (v_s, v_d, self.terminals[2], self.terminals[0])
        };
        let g = self.terminals[1];
        let b = self.terminals[3];
        let nodes = [ed, g, es, b];

        let vgs_eff = p * (v_g - v_es);
        let vds_eff = p * (v_ed - v_es);
        let vsb_eff = p * (v_es - v_b);
        let (vth, dvth) = self.mos_threshold(vsb_eff);
        let t = vgs_eff - vth;

        // Frame-variable partials per column [eff_d, g, eff_s, b].
        let dt = [0.0, p, -p * (1.0 + dvth), p * dvth];
        let dvds = [p, 0.0, -p, 0.0];

        let delta = Self::MOS_QSMOOTH;
        let r = (t * t + 4.0 * delta * delta).sqrt();
        let a = 0.5 * (t + r);
        let da_dt = 0.5 * (1.0 + t / r);
        let da: [Value; 4] = core::array::from_fn(|i| da_dt * dt[i]);

        // Saturation clamp: b = max(a - vds, 0). The charges are C1 across
        // the clamp, so the one-sided derivative switch is consistent.
        let (bq, db): (Value, [Value; 4]) = if vds_eff >= a {
            (0.0, [0.0; 4])
        } else {
            (a - vds_eff, core::array::from_fn(|i| da[i] - dvds[i]))
        };

        // a >= delta^2/|t| > 0 always, so s = a + b never vanishes.
        let s = a + bq;
        let m = a * a + a * bq + bq * bq;
        let qg = (2.0 / 3.0) * cox * m / s;
        let qg_a = (2.0 / 3.0) * cox * ((2.0 * a + bq) * s - m) / (s * s);
        let qg_b = (2.0 / 3.0) * cox * ((a + 2.0 * bq) * s - m) / (s * s);

        let s2 = s * s;
        let k = 2.0 * cox / 15.0;
        let qd =
            -k * (2.0 * a * a * a + 4.0 * a * a * bq + 6.0 * a * bq * bq + 3.0 * bq * bq * bq) / s2;
        let qd_a = -k * 2.0 * a * (a * a + 3.0 * a * bq + bq * bq) / (s2 * s);
        let qd_b = -k * bq * (8.0 * a * a + 9.0 * a * bq + 3.0 * bq * bq) / (s2 * s);

        // Gate-bulk accumulation/depletion wedge on the raw overdrive
        // (Meyer's integrable single-variable piece): capacitance Cox in
        // deep accumulation, falling linearly to zero at threshold.
        let phi = self.params.phi;
        let (qgb, cgb_t) = if t >= 0.0 {
            (0.0, 0.0)
        } else if t >= -phi {
            (-cox * t * t / (2.0 * phi), -cox * t / phi)
        } else {
            (cox * (t + 0.5 * phi), cox)
        };

        // Absorbed charges per node row [eff_d, g, eff_s, b].
        let q = [p * qd, p * (qg + qgb), p * (-qg - qd), -p * qgb];
        let mut dq = [[0.0; 4]; 4];
        for (c, (&da_c, &db_c)) in da.iter().zip(db.iter()).enumerate() {
            let dqg = qg_a * da_c + qg_b * db_c;
            let dqd = qd_a * da_c + qd_b * db_c;
            let dqgb = cgb_t * dt[c];
            dq[0][c] = p * dqd;
            dq[1][c] = p * (dqg + dqgb);
            dq[2][c] = p * (-dqg - dqd);
            dq[3][c] = -p * dqgb;
        }
        (nodes, q, dq)
    }

    /// MOSFET stored charge: bulk junction depletion (source-bulk on
    /// `cap_a`, drain-bulk on `cap_b`) plus the intrinsic channel charge.
    fn charge_mos(&self, p: Value, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (vj_sb, vj_db) = self.mos_bulk_junctions(p, node_voltages);
        let (q_sb, _) = depletion_charge(&self.params.cap_a, vj_sb);
        let (q_db, _) = depletion_charge(&self.params.cap_b, vj_db);
        let b = self.terminals[3];
        let d = self.terminals[0];
        let s = self.terminals[2];
        let mut out = vec![(b, -p * (q_sb + q_db)), (s, p * q_sb), (d, p * q_db)];

        if self.params.cox_wl > 0.0 {
            let (nodes, q, _) = self.mos_intrinsic_charge_state(p, node_voltages);
            for (node, absorbed) in nodes.iter().zip(q.iter()) {
                // Delivered-into-node convention: minus the absorbed charge.
                out.push((*node, -absorbed));
            }
        }
        out
    }

    fn cap_mos(&self, p: Value, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (vj_sb, vj_db) = self.mos_bulk_junctions(p, node_voltages);
        let (_, c_sb) = depletion_charge(&self.params.cap_a, vj_sb);
        let (_, c_db) = depletion_charge(&self.params.cap_b, vj_db);
        let b = self.terminals[3];
        let d = self.terminals[0];
        let s = self.terminals[2];
        let mut out = vec![
            ((b, b), c_sb + c_db),
            ((b, s), -c_sb),
            ((s, b), -c_sb),
            ((s, s), c_sb),
            ((b, d), -c_db),
            ((d, b), -c_db),
            ((d, d), c_db),
        ];

        if self.params.cox_wl > 0.0 {
            let (nodes, _, dq) = self.mos_intrinsic_charge_state(p, node_voltages);
            for (i, row) in dq.iter().enumerate() {
                for (j, &c) in row.iter().enumerate() {
                    if c != 0.0 {
                        out.push(((nodes[i], nodes[j]), c));
                    }
                }
            }
        }
        out
    }

    /// JFET gate junction depletion charges in the polarity frame.
    fn jfet_junction_charges(
        &self,
        p: Value,
        node_voltages: &[Value],
    ) -> (Value, Value, Value, Value) {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let (q_gs, c_gs) = depletion_charge(&self.params.cap_a, p * (v_g - v_s));
        let (q_gd, c_gd) = depletion_charge(&self.params.cap_b, p * (v_g - v_d));
        (q_gs, c_gs, q_gd, c_gd)
    }

    fn charge_jfet(&self, p: Value, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (q_gs, _, q_gd, _) = self.jfet_junction_charges(p, node_voltages);
        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];
        vec![(g, -p * (q_gs + q_gd)), (s, p * q_gs), (d, p * q_gd)]
    }

    fn cap_jfet(&self, p: Value, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (_, c_gs, _, c_gd) = self.jfet_junction_charges(p, node_voltages);
        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];
        vec![
            ((g, g), c_gs + c_gd),
            ((g, s), -c_gs),
            ((s, g), -c_gs),
            ((s, s), c_gs),
            ((g, d), -c_gd),
            ((d, g), -c_gd),
            ((d, d), c_gd),
        ]
    }
}

impl HbSolver {
    pub(super) fn validate_nonlinear_device_parameters(&self) -> Result<(), HbError> {
        for (index, device) in self.nonlinear_devices.iter().enumerate() {
            let expected_terminals = match device.device_type {
                NonlinearDeviceType::Diode => 2,
                NonlinearDeviceType::Nmos | NonlinearDeviceType::Pmos => 4,
                NonlinearDeviceType::Njfet | NonlinearDeviceType::Pjfet => 3,
                NonlinearDeviceType::VoltageSwitch => 4,
            };
            if device.terminals.len() != expected_terminals {
                return Err(HbError::InvalidCircuit(format!(
                    "nonlinear HB device {:?}#{index} has {} terminals, expected {expected_terminals}",
                    device.device_type,
                    device.terminals.len()
                )));
            }
            if let Some((terminal, node)) = device
                .terminals
                .iter()
                .copied()
                .enumerate()
                .find(|(_, node)| *node > self.num_nodes)
            {
                return Err(HbError::InvalidCircuit(format!(
                    "nonlinear HB device {:?}#{index} terminal {terminal} node index {node} exceeds {} nodes plus the ground sentinel",
                    device.device_type, self.num_nodes
                )));
            }
            if let Some(reason) = device.physical_parameter_error() {
                return Err(HbError::InvalidCircuit(format!(
                    "nonlinear HB device {:?}#{index} has invalid physical parameters: {reason}",
                    device.device_type
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type DeliveredNodeQuantityFn<'a> = dyn Fn(&[Value]) -> Vec<(usize, Value)> + 'a;
    type StampEntriesFn<'a> = dyn Fn(&[Value]) -> Vec<((usize, usize), Value)> + 'a;

    #[test]
    fn nonlinear_noise_scaling_preserves_extremes_and_rejects_invalid_factors() {
        let below_range = ScaledNonnegative::checked_product(&[Value::from_bits(1), 0.5]).unwrap();
        assert_eq!(below_range.mantissa.to_bits(), 1.0_f64.to_bits());
        assert_eq!(below_range.exponent, -1075);

        let above_range = ScaledNonnegative::checked_product(&[Value::MAX, Value::MAX]).unwrap();
        assert!(above_range.mantissa >= 1.0 && above_range.mantissa < 2.0);
        assert!(above_range.exponent > 1023);

        let exact_zero = ScaledNonnegative::checked_product(&[0.0, Value::MAX]).unwrap();
        assert_eq!(exact_zero, ScaledNonnegative::ZERO);
        for invalid in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY, -1.0] {
            assert!(ScaledNonnegative::checked_product(&[0.0, invalid]).is_err());
        }
    }

    #[test]
    fn legacy_fet_channel_noise_uses_transconductance_without_gds_floors() {
        const TEMPERATURE: Value = 300.15;
        const K_B: Value = 1.380_649e-23;

        // Deep triode operation makes gds about one thousand times gm. The
        // Level-1 channel source is nevertheless (8/3)kT*gm, matching
        // mos1noi.c rather than treating output conductance as another source.
        let mos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.0, 1.0, 0.0);
        let mos_noise = mos
            .noise_intensities(&[1.0e-3, 1.0, 0.0, 0.0], TEMPERATURE, 1.0, K_B)
            .unwrap()[0];
        let actual = libm::scalbn(mos_noise.mantissa, mos_noise.exponent);
        let expected = (8.0 / 3.0) * K_B * TEMPERATURE * 1.0e-3;
        assert!((actual - expected).abs() <= 8.0 * Value::EPSILON * expected);

        // Cutoff has gm=0 and must therefore retain an exact zero noise source.
        let jfet = NonlinearDeviceInstance::njfet(0, 1, 2, -2.0, 1.0e-3, 0.0, 1.0e-14);
        let cutoff = jfet
            .noise_intensities(&[1.0, -3.0, 0.0], TEMPERATURE, 1.0, K_B)
            .unwrap();
        assert_eq!(cutoff, vec![ScaledNonnegative::ZERO]);
    }

    #[test]
    fn cutoff_device_jacobians_do_not_invent_physical_conductance() {
        let diode = NonlinearDeviceInstance::diode(0, 1, 0.0, 1.0);
        assert!(
            diode
                .jacobian(&[-1.0, 0.0])
                .iter()
                .all(|(_, conductance)| *conductance == 0.0)
        );

        let mos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.0).with_bulk_junctions(
            DepletionCap::none(),
            DepletionCap::none(),
            0.0,
            0.0,
        );
        assert!(
            mos.jacobian(&[1.0, -1.0, 0.0, 0.0])
                .iter()
                .all(|(_, conductance)| *conductance == 0.0)
        );

        let jfet = NonlinearDeviceInstance::njfet(0, 1, 2, -2.0, 1.0e-3, 0.0, 0.0);
        assert!(
            jfet.jacobian(&[1.0, -3.0, 0.0])
                .iter()
                .all(|(_, conductance)| *conductance == 0.0)
        );
    }

    #[test]
    fn zero_lambda_mos_saturation_has_exact_zero_output_conductance() {
        let mos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2.0e-5, 0.0).with_bulk_junctions(
            DepletionCap::none(),
            DepletionCap::none(),
            0.0,
            0.0,
        );
        let voltages = [2.0, 1.5, 0.0, 0.0];
        let (_, _, _, gm, gds, gmbs) = mos.mos_operating_point(1.0, &voltages);
        assert!(gm > 0.0);
        assert_eq!(gds, 0.0);
        assert_eq!(gmbs, 0.0);

        let drain_diagonal = mos
            .jacobian(&voltages)
            .iter()
            .filter(|((row, column), _)| *row == 0 && *column == 0)
            .map(|(_, value)| value)
            .sum::<Value>();
        assert_eq!(drain_diagonal, 0.0);
    }

    #[test]
    fn voltage_switch_preserves_high_off_resistance_without_a_conductance_floor() {
        let switch =
            NonlinearDeviceInstance::voltage_switch(0, 1, 2, 3, 0.0, 0.0, 1.0, 1.0e15, 0.1);
        let (conductance, derivative) = switch.switch_conductance_and_derivative(-100.0);
        let expected = 1.0e-15;
        assert!((conductance - expected).abs() <= 16.0 * Value::EPSILON * expected);
        assert_eq!(derivative, 0.0);
        assert!(conductance < 1.0e-12);
    }

    #[test]
    fn voltage_switch_invalid_physical_parameters_fail_closed() {
        for (ron, roff, smooth) in [
            (0.0, 1.0e6, 0.1),
            (1.0, 0.0, 0.1),
            (1.0, 1.0e6, 0.0),
            (Value::NAN, 1.0e6, 0.1),
        ] {
            let switch =
                NonlinearDeviceInstance::voltage_switch(0, 1, 2, 3, 0.0, 0.0, ron, roff, smooth);
            let (conductance, derivative) = switch.switch_conductance_and_derivative(0.0);
            assert!(conductance.is_nan());
            assert!(derivative.is_nan());
        }
    }

    /// Deterministic uniform sample in [lo, hi).
    fn lcg(seed: &mut u64, lo: Value, hi: Value) -> Value {
        *seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let u = ((*seed >> 11) as f64) / ((1u64 << 53) as f64);
        lo + u * (hi - lo)
    }

    /// Every stamp must be the derivative of the evaluated per-node
    /// quantity: stamp(i, j) == -d(quantity delivered into node i)/dV_j.
    /// Central differences over deterministic operating points catch any
    /// missing, mis-signed, or region-inconsistent entry. Shared by the
    /// conductance (current) and capacitance (charge) layers.
    #[allow(clippy::too_many_arguments)]
    fn assert_stamps_match_finite_difference(
        label: &str,
        num_nodes: usize,
        v_range: (Value, Value),
        samples: usize,
        seed: u64,
        tol_abs: Value,
        deliver: &DeliveredNodeQuantityFn<'_>,
        stamp_fn: &StampEntriesFn<'_>,
    ) {
        let mut seed = seed;
        let h = 1e-7;
        let tol_rel = 1e-4;

        for sample in 0..samples {
            let v: Vec<Value> = (0..num_nodes)
                .map(|_| lcg(&mut seed, v_range.0, v_range.1))
                .collect();

            let mut stamps = vec![vec![0.0; num_nodes]; num_nodes];
            for ((i, j), g) in stamp_fn(&v) {
                if i < num_nodes && j < num_nodes {
                    stamps[i][j] += g;
                }
            }

            for j in 0..num_nodes {
                let mut vp = v.clone();
                vp[j] += h;
                let mut vm = v.clone();
                vm[j] -= h;

                let mut into_p = vec![0.0; num_nodes];
                let mut into_m = vec![0.0; num_nodes];
                for (n, c) in deliver(&vp) {
                    if n < num_nodes {
                        into_p[n] += c;
                    }
                }
                for (n, c) in deliver(&vm) {
                    if n < num_nodes {
                        into_m[n] += c;
                    }
                }

                for i in 0..num_nodes {
                    let fd = (into_p[i] - into_m[i]) / (2.0 * h);
                    let expected = -fd;
                    let got = stamps[i][j];
                    let scale = expected.abs().max(got.abs());
                    // Central differences cannot resolve derivatives below the
                    // rounding floor of the evaluated currents themselves; the
                    // junction linear-continuation region reaches ampere scales
                    // where small cross-junction terms cancel out of f64 sums.
                    let noise_floor = 8.0 * f64::EPSILON * into_p[i].abs().max(into_m[i].abs()) / h;
                    assert!(
                        (got - expected).abs() <= tol_rel * scale + tol_abs + noise_floor,
                        "{} sample {} stamp ({}, {}): jacobian {:.6e} vs finite-difference {:.6e} at V={:?}",
                        label,
                        sample,
                        i,
                        j,
                        got,
                        expected,
                        v
                    );
                }
            }
        }
    }

    fn assert_jacobian_matches_finite_difference(
        device: &NonlinearDeviceInstance,
        num_nodes: usize,
        v_range: (Value, Value),
        samples: usize,
        seed: u64,
    ) {
        assert_stamps_match_finite_difference(
            &format!("{:?} current", device.device_type),
            num_nodes,
            v_range,
            samples,
            seed,
            1e-8,
            &|v| device.evaluate(v),
            &|v| device.jacobian(v),
        );
    }

    /// Capacitance stamps live at the 1e-12 F scale, so the absolute floor
    /// shrinks accordingly.
    fn assert_charge_jacobian_matches_finite_difference(
        device: &NonlinearDeviceInstance,
        num_nodes: usize,
        v_range: (Value, Value),
        samples: usize,
        seed: u64,
    ) {
        assert_stamps_match_finite_difference(
            &format!("{:?} charge", device.device_type),
            num_nodes,
            v_range,
            samples,
            seed,
            1e-20,
            &|v| device.charge(v),
            &|v| device.charge_jacobian(v),
        );
    }

    #[test]
    fn diode_jacobian_matches_finite_difference() {
        let device = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);
        assert_jacobian_matches_finite_difference(&device, 2, (-1.0, 1.0), 40, 11);

        let device = NonlinearDeviceInstance::diode(0, 1, 2.5e-9, 1.8);
        assert_jacobian_matches_finite_difference(&device, 2, (-1.0, 1.0), 40, 13);
    }

    #[test]
    fn mosfet_jacobians_match_finite_difference() {
        let nmos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.04);
        assert_jacobian_matches_finite_difference(&nmos, 4, (-3.0, 3.0), 80, 23);

        let pmos = NonlinearDeviceInstance::pmos(0, 1, 2, 3, 0.7, 1.2e-5, 0.05);
        assert_jacobian_matches_finite_difference(&pmos, 4, (-3.0, 3.0), 80, 29);

        // Body effect engaged: the gmbs stamps on the bulk column must be
        // the derivative of the threshold law everywhere, including the
        // drain/source swap region and the full-depletion clamp.
        let nmos_body =
            NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.04).with_body_effect(0.6, 0.7);
        assert_jacobian_matches_finite_difference(&nmos_body, 4, (-3.0, 3.0), 80, 31);

        let pmos_body = NonlinearDeviceInstance::pmos(0, 1, 2, 3, 0.7, 1.2e-5, 0.05)
            .with_body_effect(0.5, 0.65);
        assert_jacobian_matches_finite_difference(&pmos_body, 4, (-3.0, 3.0), 80, 37);

        // Bulk diodes conducting: the source-bulk and drain-bulk junction
        // conductances ride on the physical terminals through the
        // drain/source swap region.
        let nmos_junc = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.04)
            .with_body_effect(0.6, 0.7)
            .with_bulk_junctions(DepletionCap::none(), DepletionCap::none(), 1e-14, 2e-14);
        assert_jacobian_matches_finite_difference(&nmos_junc, 4, (-1.0, 1.0), 80, 67);

        let pmos_junc = NonlinearDeviceInstance::pmos(0, 1, 2, 3, 0.7, 1.2e-5, 0.05)
            .with_body_effect(0.5, 0.65)
            .with_bulk_junctions(DepletionCap::none(), DepletionCap::none(), 2e-14, 1e-14);
        assert_jacobian_matches_finite_difference(&pmos_junc, 4, (-1.0, 1.0), 80, 71);
    }

    /// The body-effect threshold law itself: at fixed bias the saturation
    /// current must follow 0.5*kp*(vgs - vth(vsb))^2 with
    /// vth = vto + gamma*(sqrt(phi + vsb) - sqrt(phi)).
    #[test]
    fn body_effect_shifts_the_threshold_by_the_textbook_law() {
        let gamma = 0.5;
        let phi = 0.7;
        let nmos =
            NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.0).with_body_effect(gamma, phi);

        // Vd=5, Vg=2, Vs=0, Vb=-1: saturation with vsb = 1.
        let v = vec![5.0, 2.0, 0.0, -1.0];
        let into_source: Value = nmos
            .evaluate(&v)
            .iter()
            .filter(|(n, _)| *n == 2)
            .map(|(_, c)| c)
            .sum();

        let vth = 0.7 + gamma * ((phi + 1.0_f64).sqrt() - phi.sqrt());
        let expected = 0.5 * 2e-5 * (2.0 - vth) * (2.0 - vth);
        assert!(
            (into_source - expected).abs() < 1e-9 * expected,
            "saturation current must follow the shifted threshold: got {into_source:.6e}, want {expected:.6e}"
        );
    }

    #[test]
    fn jfet_jacobians_match_finite_difference() {
        let njf = NonlinearDeviceInstance::njfet(0, 1, 2, -2.0, 1e-3, 0.02, 1e-14);
        assert_jacobian_matches_finite_difference(&njf, 3, (-2.5, 2.5), 80, 31);

        let pjf = NonlinearDeviceInstance::pjfet(0, 1, 2, -2.0, 1e-3, 0.02, 1e-14);
        assert_jacobian_matches_finite_difference(&pjf, 3, (-2.5, 2.5), 80, 37);
    }

    #[test]
    fn switch_jacobians_match_finite_difference() {
        let vsw = NonlinearDeviceInstance::voltage_switch(0, 1, 2, 3, 0.5, 0.1, 1.0, 1e6, 0.1);
        assert_jacobian_matches_finite_difference(&vsw, 4, (-2.0, 2.0), 60, 41);
    }

    #[test]
    fn charge_jacobians_match_finite_difference() {
        let mut diode = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);
        diode.params.cap_a = DepletionCap::new(10e-12, 0.7, 0.5, 0.5);
        diode.params.tt_f = 5e-9;
        assert_charge_jacobian_matches_finite_difference(&diode, 2, (-5.0, 0.8), 60, 51);

        let mut njf = NonlinearDeviceInstance::njfet(0, 1, 2, -2.0, 1e-3, 0.02, 1e-14);
        njf.params.cap_a = DepletionCap::new(4e-12, 0.8, 0.5, 0.5);
        njf.params.cap_b = DepletionCap::new(4e-12, 0.8, 0.5, 0.5);
        assert_charge_jacobian_matches_finite_difference(&njf, 3, (-2.5, 0.6), 60, 59);

        let mut pjf = NonlinearDeviceInstance::pjfet(0, 1, 2, -2.0, 1e-3, 0.02, 1e-14);
        pjf.params.cap_a = DepletionCap::new(4e-12, 0.8, 0.5, 0.5);
        pjf.params.cap_b = DepletionCap::new(4e-12, 0.8, 0.5, 0.5);
        assert_charge_jacobian_matches_finite_difference(&pjf, 3, (-2.5, 0.6), 60, 61);

        let nmos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.04)
            .with_body_effect(0.6, 0.7)
            .with_intrinsic_gate(80e-15)
            .with_bulk_junctions(
                DepletionCap::new(3e-12, 0.8, 0.5, 0.5),
                DepletionCap::new(2e-12, 0.8, 0.5, 0.5),
                1e-14,
                1e-14,
            );
        assert_charge_jacobian_matches_finite_difference(&nmos, 4, (-2.5, 2.5), 60, 63);

        let pmos = NonlinearDeviceInstance::pmos(0, 1, 2, 3, 0.7, 1.2e-5, 0.05)
            .with_body_effect(0.5, 0.65)
            .with_intrinsic_gate(120e-15)
            .with_bulk_junctions(
                DepletionCap::new(2e-12, 0.8, 0.5, 0.5),
                DepletionCap::new(3e-12, 0.8, 0.5, 0.5),
                1e-14,
                1e-14,
            );
        assert_charge_jacobian_matches_finite_difference(&pmos, 4, (-2.5, 2.5), 60, 69);
    }

    /// The intrinsic channel charge must hit the square-law textbook
    /// limits: gate capacitance Cox at vds = 0, (2/3) Cox in saturation
    /// with the 40/60 Ward-Dutton drain/source charge split, and the
    /// gate-bulk wedge must deliver Cox in deep accumulation.
    #[test]
    fn mos_intrinsic_gate_charge_matches_the_square_law_limits() {
        let cox = 1e-12;
        let nmos =
            NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.0).with_intrinsic_gate(cox);
        let c_gg = |v: &[Value]| -> Value {
            nmos.charge_jacobian(v)
                .iter()
                .filter(|((i, j), _)| *i == 1 && *j == 1)
                .map(|(_, c)| c)
                .sum()
        };

        // Saturation (vds > vgst): cgg = (2/3) Cox.
        let v_sat = vec![3.0, 2.0, 0.0, 0.0];
        let c_sat = c_gg(&v_sat);
        assert!(
            (c_sat - 2.0 / 3.0 * cox).abs() < 1e-3 * cox,
            "saturation gate capacitance must be (2/3) Cox: got {c_sat:.6e}"
        );

        // Triode at vds -> 0: cgg = Cox.
        let v_lin = vec![1e-12, 2.0, 0.0, 0.0];
        let c_lin = c_gg(&v_lin);
        assert!(
            (c_lin - cox).abs() < 1e-3 * cox,
            "vds = 0 gate capacitance must be Cox: got {c_lin:.6e}"
        );

        // Deep accumulation (vgst << -phi): the gate-bulk wedge gives Cox.
        let v_acc = vec![0.0, -3.0, 0.0, 0.0];
        let c_acc = c_gg(&v_acc);
        assert!(
            (c_acc - cox).abs() < 1e-3 * cox,
            "deep-accumulation gate capacitance must be Cox: got {c_acc:.6e}"
        );

        // Ward-Dutton saturation partition: |Qd| / |Qs| = (4/15) / (6/15).
        let absorbed = |v: &[Value], node: usize| -> Value {
            nmos.charge(v)
                .iter()
                .filter(|(n, _)| *n == node)
                .map(|(_, q)| -q)
                .sum()
        };
        let qd = absorbed(&v_sat, 0);
        let qs = absorbed(&v_sat, 2);
        let vgst = 2.0 - 0.7;
        assert!(
            (qd + 4.0 / 15.0 * cox * vgst).abs() < 1e-3 * cox * vgst,
            "saturation drain charge must be -(4/15) Cox vgst: got {qd:.6e}"
        );
        assert!(
            (qs + 6.0 / 15.0 * cox * vgst).abs() < 1e-3 * cox * vgst,
            "saturation source charge must be -(6/15) Cox vgst: got {qs:.6e}"
        );
    }

    /// The bulk junction capacitance must follow the textbook power law
    /// cj0 / (1 - vj/pb)^mj in reverse bias, on the bulk row/column.
    #[test]
    fn mos_bulk_junction_capacitance_follows_the_power_law() {
        let cj0 = 10e-12;
        let (pb, mj) = (0.8, 0.5);
        let nmos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.0).with_bulk_junctions(
            DepletionCap::new(cj0, pb, mj, 0.5),
            DepletionCap::none(),
            1e-14,
            1e-14,
        );

        // Vd=Vg=Vs=0, Vb=-2: source-bulk junction at vj = -2 (reverse).
        let v = vec![0.0, 0.0, 0.0, -2.0];
        let c_bb: Value = nmos
            .charge_jacobian(&v)
            .iter()
            .filter(|((i, j), _)| *i == 3 && *j == 3)
            .map(|(_, c)| c)
            .sum();
        let expected = cj0 / (1.0 + 2.0 / pb).powf(mj);
        assert!(
            (c_bb - expected).abs() < 1e-6 * expected,
            "reverse-bias bulk capacitance must follow the power law: \
             got {c_bb:.6e}, want {expected:.6e}"
        );
    }

    /// Depletion charge and capacitance must be C1-continuous at the
    /// forward-bias linearization knee fc*vj, and the capacitance must match
    /// the textbook power law in reverse bias.
    #[test]
    fn depletion_charge_is_continuous_at_the_knee_and_exact_in_reverse() {
        let cap = DepletionCap::new(10e-12, 0.7, 0.5, 0.5);
        let knee = cap.fc * cap.vj;

        for eps in [1e-9, 1e-7] {
            let (q_below, c_below) = depletion_charge(&cap, knee - eps);
            let (q_above, c_above) = depletion_charge(&cap, knee + eps);
            assert!(
                (q_below - q_above).abs() < 1e-6 * q_below.abs().max(1e-15),
                "depletion charge continuous at the knee: {q_below:.6e} vs {q_above:.6e}"
            );
            assert!(
                (c_below - c_above).abs() < 1e-4 * c_below.abs(),
                "depletion capacitance continuous at the knee: {c_below:.6e} vs {c_above:.6e}"
            );
        }

        // Reverse bias: C = CJ0*(1 - v/vj)^-m exactly.
        for v in [-5.0, -2.0, -0.5] {
            let (_, c) = depletion_charge(&cap, v);
            let expected = cap.cj0 * (1.0 - v / cap.vj).powf(-cap.m);
            assert!(
                (c - expected).abs() < 1e-12 * expected,
                "reverse-bias capacitance at {v} V: got {c:.6e}, want {expected:.6e}"
            );
        }
    }

    #[test]
    fn unit_grading_coefficient_uses_the_exact_logarithmic_limit() {
        let cap = DepletionCap::new(10e-12, 0.7, 1.0, 0.5);
        for voltage in [-2.0, -0.5, 0.1] {
            let (charge, capacitance) = depletion_charge(&cap, voltage);
            let x = 1.0 - voltage / cap.vj;
            let expected_charge = -cap.cj0 * cap.vj * x.ln();
            let expected_capacitance = cap.cj0 / x;
            assert!((charge - expected_charge).abs() <= 8.0 * Value::EPSILON * charge.abs());
            assert!(
                (capacitance - expected_capacitance).abs()
                    <= 8.0 * Value::EPSILON * capacitance.abs()
            );
        }
    }

    #[test]
    fn retained_charge_parameter_boundaries_are_preserved_and_accepted() {
        let primary = DepletionCap::new(0.0, Value::MIN_POSITIVE, 0.0, 0.0);
        let secondary = DepletionCap::new(Value::from_bits(1), 1.0, 1.0, 0.999_999_999_999);
        assert_eq!(primary.cj0.to_bits(), 0.0_f64.to_bits());
        assert_eq!(primary.vj.to_bits(), Value::MIN_POSITIVE.to_bits());
        assert_eq!(primary.m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(primary.fc.to_bits(), 0.0_f64.to_bits());
        assert_eq!(secondary.cj0.to_bits(), Value::from_bits(1).to_bits());
        assert_eq!(secondary.m.to_bits(), 1.0_f64.to_bits());

        let transit_time = Value::from_bits(1);
        let diode = NonlinearDeviceInstance::diode(0, 0, 0.0, 1.0).with_junction_caps(
            primary,
            secondary,
            transit_time,
        );
        assert_eq!(diode.params.tt_f.to_bits(), transit_time.to_bits());

        let intrinsic_gate = Value::from_bits(1);
        let mos = NonlinearDeviceInstance::nmos(0, 0, 0, 0, 0.7, 0.0, 0.0)
            .with_intrinsic_gate(intrinsic_gate);
        assert_eq!(mos.params.cox_wl.to_bits(), intrinsic_gate.to_bits());

        let mut solver = HbSolver::new(HbConfig::new(1.0e6).with_harmonics(1), 1);
        solver.add_nonlinear_device(diode);
        solver.add_nonlinear_device(mos);
        solver
            .validate_nonlinear_device_parameters()
            .expect("exact retained boundary values are valid");
    }

    /// Level-1 channel-length modulation applies in triode and saturation;
    /// the drain current must be continuous across the region boundary.
    #[test]
    fn mosfet_current_is_continuous_across_the_saturation_boundary() {
        let nmos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.08);
        let vov = 1.3; // vgs = 2.0
        for eps in [1e-9, 1e-6] {
            let below = nmos.evaluate(&[vov - eps, 2.0, 0.0, 0.0]);
            let above = nmos.evaluate(&[vov + eps, 2.0, 0.0, 0.0]);
            let id_below: Value = below.iter().filter(|(n, _)| *n == 2).map(|(_, c)| c).sum();
            let id_above: Value = above.iter().filter(|(n, _)| *n == 2).map(|(_, c)| c).sum();
            assert!(
                (id_below - id_above).abs() <= 1e-6 * id_below.abs().max(1e-12),
                "drain current must be continuous at the triode/saturation boundary: {id_below:.9e} vs {id_above:.9e}"
            );
        }
    }
}

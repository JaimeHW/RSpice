//! Nonlinear device parameter builders and time-domain evaluation helpers.

use super::*;

/// Largest junction exponent evaluated exactly; beyond it the exponential is
/// continued linearly so the current keeps responding to the voltage.
const MAX_EXP_ARG: Value = 40.0;

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

/// Currents and junction-frame partials of the Ebers-Moll transport core.
struct BjtOperatingPoint {
    /// Current absorbed at the collector terminal.
    ic: Value,
    /// Current absorbed at the base terminal.
    ib: Value,
    d_ic_d_vbe: Value,
    d_ic_d_vbc: Value,
    d_ib_d_vbe: Value,
    d_ib_d_vbc: Value,
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

    /// Create BJT parameters
    pub fn bjt(is: Value, bf: Value, br: Value, nf: Value, nr: Value, vaf: Value) -> Self {
        Self {
            is,
            bf,
            br: br.max(1e-6),
            nf: nf.max(1e-3),
            nr: nr.max(1e-3),
            vaf,
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
            is: is.max(1e-30),
            ..Default::default()
        }
    }

    /// Create voltage-controlled switch parameters
    pub fn voltage_switch(vt: Value, vh: Value, ron: Value, roff: Value, smooth: Value) -> Self {
        Self {
            vth: vt,
            vh: vh.abs(),
            ron: ron.max(1e-6),
            roff: roff.max(1e-6),
            smooth: smooth.max(1e-9),
            ..Default::default()
        }
    }

    /// Create current-controlled switch parameters.
    pub fn current_switch(
        it: Value,
        ih: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
        control_gain: Value,
    ) -> Self {
        Self {
            vth: it,
            vh: ih.abs(),
            ron: ron.max(1e-6),
            roff: roff.max(1e-6),
            smooth: smooth.max(1e-12),
            control_gain: control_gain.max(1e-18),
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

    /// Create an NPN BJT instance
    pub fn npn_bjt(
        collector: usize,
        base: usize,
        emitter: usize,
        is: Value,
        bf: Value,
        br: Value,
        nf: Value,
        nr: Value,
        vaf: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::NpnBjt,
            terminals: vec![collector, base, emitter],
            params: NonlinearDeviceParams::bjt(is, bf, br, nf, nr, vaf),
        }
    }

    /// Create a PNP BJT instance
    pub fn pnp_bjt(
        collector: usize,
        base: usize,
        emitter: usize,
        is: Value,
        bf: Value,
        br: Value,
        nf: Value,
        nr: Value,
        vaf: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::PnpBjt,
            terminals: vec![collector, base, emitter],
            params: NonlinearDeviceParams::bjt(is, bf, br, nf, nr, vaf),
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
    pub fn voltage_switch(
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

    /// Create a current-controlled switch instance.
    pub fn current_switch(
        node_pos: usize,
        node_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        it: Value,
        ih: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
        control_gain: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::CurrentSwitch,
            terminals: vec![node_pos, node_neg, ctrl_pos, ctrl_neg],
            params: NonlinearDeviceParams::current_switch(it, ih, ron, roff, smooth, control_gain),
        }
    }

    /// Evaluate device current given terminal voltages
    /// Returns Vec of (node_index, current) pairs - current flowing INTO each node
    pub fn evaluate(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        match self.device_type {
            NonlinearDeviceType::Diode => self.eval_diode(node_voltages),
            NonlinearDeviceType::NpnBjt => self.eval_npn_bjt(node_voltages),
            NonlinearDeviceType::PnpBjt => self.eval_pnp_bjt(node_voltages),
            NonlinearDeviceType::Nmos => self.eval_nmos(node_voltages),
            NonlinearDeviceType::Pmos => self.eval_pmos(node_voltages),
            NonlinearDeviceType::Njfet => self.eval_njfet(node_voltages),
            NonlinearDeviceType::Pjfet => self.eval_pjfet(node_voltages),
            NonlinearDeviceType::VoltageSwitch => self.eval_voltage_switch(node_voltages),
            NonlinearDeviceType::CurrentSwitch => self.eval_current_switch(node_voltages),
        }
    }

    /// Compute Jacobian entries (âˆ‚I/âˆ‚V for each terminal pair)
    /// Returns Vec of ((from_node, to_node), dI/dV) - linearized conductance stamps
    pub fn jacobian(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        match self.device_type {
            NonlinearDeviceType::Diode => self.jac_diode(node_voltages),
            NonlinearDeviceType::NpnBjt => self.jac_npn_bjt(node_voltages),
            NonlinearDeviceType::PnpBjt => self.jac_pnp_bjt(node_voltages),
            NonlinearDeviceType::Nmos => self.jac_nmos(node_voltages),
            NonlinearDeviceType::Pmos => self.jac_pmos(node_voltages),
            NonlinearDeviceType::Njfet => self.jac_njfet(node_voltages),
            NonlinearDeviceType::Pjfet => self.jac_pjfet(node_voltages),
            NonlinearDeviceType::VoltageSwitch => self.jac_voltage_switch(node_voltages),
            NonlinearDeviceType::CurrentSwitch => self.jac_current_switch(node_voltages),
        }
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
        let gd = gd.max(1e-12); // Minimum conductance for numerical stability

        let a = self.terminals[0];
        let c = self.terminals[1];

        // Return MNA conductance stamp
        // Physical conductance gd is POSITIVE. MNA stamp for 2-terminal conductance:
        // G[n+,n+] += gd, G[n+,n-] -= gd, G[n-,n+] -= gd, G[n-,n-] += gd
        vec![((a, a), gd), ((a, c), -gd), ((c, a), -gd), ((c, c), gd)]
    }

    /// Ebers-Moll transport core shared by NPN and PNP.
    ///
    /// Works in the polarity frame (`vbe_eff = p*(Vb - Ve)`, `vbc_eff =
    /// p*(Vb - Vc)`); because the polarity enters both the junction voltages
    /// and the terminal currents, the node-space Jacobian is polarity-free.
    ///
    /// Returns `(ic, ib)` terminal currents (current absorbed at collector and
    /// base) plus the four partials of the polarity-frame currents with respect
    /// to the effective junction voltages.
    fn bjt_core(&self, p: Value, node_voltages: &[Value]) -> BjtOperatingPoint {
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let vbe = p * (v_b - v_e);
        let vbc = p * (v_b - v_c);

        let (i_f, gf) = junction_current(self.params.is, vbe, self.params.nf * self.params.vt);
        let (i_r, gr) = junction_current(self.params.is, vbc, self.params.nr * self.params.vt);

        // Forward Early effect on the transport current (SPICE level-1 form).
        let (early, d_early_d_vbc) =
            if self.params.vaf.is_finite() && self.params.vaf > 0.0 {
                ((1.0 - vbc / self.params.vaf).max(0.01), -1.0 / self.params.vaf)
            } else {
                (1.0, 0.0)
            };

        // Transport current Ict = Is*(exp(Vbe/NfVt) - exp(Vbc/NrVt)).
        let i_ct = i_f - i_r;

        let ic_int = i_ct * early - i_r / self.params.br;
        let ib_int = i_f / self.params.bf + i_r / self.params.br;

        BjtOperatingPoint {
            ic: p * ic_int,
            ib: p * ib_int,
            d_ic_d_vbe: gf * early,
            d_ic_d_vbc: -gr * early + i_ct * d_early_d_vbc - gr / self.params.br,
            d_ib_d_vbe: gf / self.params.bf,
            d_ib_d_vbc: gr / self.params.br,
        }
    }

    fn eval_bjt(&self, p: Value, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let op = self.bjt_core(p, node_voltages);
        let ie = -(op.ic + op.ib); // KCL

        vec![
            (self.terminals[0], -op.ic), // Collector current out
            (self.terminals[1], -op.ib), // Base current out
            (self.terminals[2], -ie),    // Emitter current out
        ]
    }

    /// BJT conductance stamps from the exact partials of the transport core.
    ///
    /// Stamp convention: entry `((i, j), g)` is the derivative of the current
    /// absorbed at terminal i with respect to node voltage j. In the polarity
    /// frame the chain rule contributes p twice, so the stamps below hold for
    /// NPN and PNP alike.
    fn jac_bjt(&self, p: Value, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let op = self.bjt_core(p, node_voltages);

        let c = self.terminals[0];
        let b = self.terminals[1];
        let e = self.terminals[2];

        // Node-space partials: Vbe_eff and Vbc_eff both increase with Vb,
        // decrease with Ve and Vc respectively.
        let d_ic_d_vb = op.d_ic_d_vbe + op.d_ic_d_vbc;
        let d_ic_d_vc = -op.d_ic_d_vbc;
        let d_ic_d_ve = -op.d_ic_d_vbe;

        let d_ib_d_vb = op.d_ib_d_vbe + op.d_ib_d_vbc;
        let d_ib_d_vc = -op.d_ib_d_vbc;
        let d_ib_d_ve = -op.d_ib_d_vbe;

        vec![
            ((c, b), d_ic_d_vb),
            ((c, c), d_ic_d_vc),
            ((c, e), d_ic_d_ve),
            ((b, b), d_ib_d_vb),
            ((b, c), d_ib_d_vc),
            ((b, e), d_ib_d_ve),
            // Emitter absorbs -(ic + ib).
            ((e, b), -(d_ic_d_vb + d_ib_d_vb)),
            ((e, c), -(d_ic_d_vc + d_ib_d_vc)),
            ((e, e), -(d_ic_d_ve + d_ib_d_ve)),
        ]
    }

    fn eval_npn_bjt(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        self.eval_bjt(1.0, node_voltages)
    }

    fn jac_npn_bjt(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        self.jac_bjt(1.0, node_voltages)
    }

    fn eval_pnp_bjt(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        self.eval_bjt(-1.0, node_voltages)
    }

    fn jac_pnp_bjt(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        self.jac_bjt(-1.0, node_voltages)
    }

    /// Level-1 MOSFET core in the polarity frame after drain/source swap.
    ///
    /// `vgs`/`vds` are effective (polarity-resolved, `vds >= 0`) values;
    /// channel-length modulation applies in triode and saturation so the
    /// current and its derivatives are continuous across the region boundary
    /// (ngspice MOS1 convention).
    fn mos_ids(&self, vgs: Value, vds: Value) -> (Value, Value, Value) {
        let vth = self.params.vth;
        let kp = self.params.kp;
        let lambda = self.params.lambda.max(0.0);
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
    /// Returns `(eff_d, eff_s, ids, gm, gds)` where `ids`, `gm`, `gds` are in
    /// the swapped polarity frame and the current absorbed at `eff_d` is
    /// `p * ids`.
    fn mos_operating_point(
        &self,
        p: Value,
        node_voltages: &[Value],
    ) -> (usize, usize, Value, Value, Value) {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let d = self.terminals[0];
        let s = self.terminals[2];

        // Symmetric device: swap drain/source when the effective Vds is negative.
        let (vgs, vds, eff_d, eff_s) = if p * (v_d - v_s) >= 0.0 {
            (p * (v_g - v_s), p * (v_d - v_s), d, s)
        } else {
            (p * (v_g - v_d), p * (v_s - v_d), s, d)
        };

        let (ids, gm, gds) = self.mos_ids(vgs, vds);
        (eff_d, eff_s, ids, gm, gds)
    }

    fn eval_mos(&self, p: Value, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (eff_d, eff_s, ids, _, _) = self.mos_operating_point(p, node_voltages);
        let absorbed = p * ids; // Current absorbed at the effective drain.
        vec![(eff_d, -absorbed), (eff_s, absorbed)]
    }

    fn jac_mos(&self, p: Value, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (eff_d, eff_s, _, gm, gds) = self.mos_operating_point(p, node_voltages);
        let g = self.terminals[1];

        if gm == 0.0 && gds == 0.0 {
            // Cutoff: tiny drain-source leak keeps the Jacobian regular without
            // introducing a phantom path to ground.
            let g_leak = 1e-12;
            return vec![
                ((eff_d, eff_d), g_leak),
                ((eff_d, eff_s), -g_leak),
                ((eff_s, eff_d), -g_leak),
                ((eff_s, eff_s), g_leak),
            ];
        }

        // The polarity factors cancel (p^2 = 1): the node-space stamps are the
        // textbook MOS pattern in the effective frame for NMOS and PMOS alike.
        vec![
            ((eff_d, eff_d), gds),
            ((eff_d, g), gm),
            ((eff_d, eff_s), -(gm + gds)),
            ((eff_s, eff_d), -gds),
            ((eff_s, g), -gm),
            ((eff_s, eff_s), gm + gds),
        ]
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
        let beta = self.params.kp.max(1e-18);
        let lambda = self.params.lambda.max(0.0);
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

        (polarity * ids_int, gm, gds.max(1e-12))
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

        let (igs, ggs) =
            junction_current(self.params.is, polarity * (v_g - v_s), self.params.vt);
        let (igd, ggd) =
            junction_current(self.params.is, polarity * (v_g - v_d), self.params.vt);
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
        let smooth = self.params.smooth.max(1e-9);
        let x = (vctrl - self.params.vth) / smooth;
        let tanh_x = x.tanh();
        let f = 0.5 * (1.0 - tanh_x);

        let ron = self.params.ron.max(1e-6);
        let roff = self.params.roff.max(ron);
        let log_ron = ron.ln();
        let log_roff = roff.ln();
        let dlog_r = log_roff - log_ron;
        let log_r = log_ron + dlog_r * f;
        let g = (-log_r).exp();

        let sech2 = 1.0 - tanh_x * tanh_x;
        let df_dvctrl = -0.5 * sech2 / smooth;
        let dlogr_dvctrl = dlog_r * df_dvctrl;
        let dg_dvctrl = -g * dlogr_dvctrl;

        (g.max(1e-12), dg_dvctrl)
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

    fn eval_current_switch(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vmain = vp - vn;
        let ictrl = self.params.control_gain * (vcp - vcn);
        let (g, _) = self.switch_conductance_and_derivative(ictrl);
        let i = g * vmain;
        vec![(self.terminals[0], -i), (self.terminals[1], i)]
    }

    fn jac_current_switch(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let vp = self.get_terminal_voltage(node_voltages, 0);
        let vn = self.get_terminal_voltage(node_voltages, 1);
        let vcp = self.get_terminal_voltage(node_voltages, 2);
        let vcn = self.get_terminal_voltage(node_voltages, 3);
        let vmain = vp - vn;
        let ictrl = self.params.control_gain * (vcp - vcn);
        let (g, dg_dictrl) = self.switch_conductance_and_derivative(ictrl);
        let g_ctrl = dg_dictrl * self.params.control_gain * vmain;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic uniform sample in [lo, hi).
    fn lcg(seed: &mut u64, lo: Value, hi: Value) -> Value {
        *seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let u = ((*seed >> 11) as f64) / ((1u64 << 53) as f64);
        lo + u * (hi - lo)
    }

    /// Every conductance stamp must be the derivative of the evaluated
    /// currents: stamp(i, j) == -d(current delivered into node i)/dV_j.
    /// Central differences over deterministic operating points catch any
    /// missing, mis-signed, or region-inconsistent entry.
    fn assert_jacobian_matches_finite_difference(
        device: &NonlinearDeviceInstance,
        num_nodes: usize,
        v_range: (Value, Value),
        samples: usize,
        seed: u64,
    ) {
        let mut seed = seed;
        let h = 1e-7;
        let tol_rel = 1e-4;
        let tol_abs = 1e-8;

        for sample in 0..samples {
            let v: Vec<Value> = (0..num_nodes)
                .map(|_| lcg(&mut seed, v_range.0, v_range.1))
                .collect();

            let mut stamps = vec![vec![0.0; num_nodes]; num_nodes];
            for ((i, j), g) in device.jacobian(&v) {
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
                for (n, c) in device.evaluate(&vp) {
                    if n < num_nodes {
                        into_p[n] += c;
                    }
                }
                for (n, c) in device.evaluate(&vm) {
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
                    let noise_floor =
                        8.0 * f64::EPSILON * into_p[i].abs().max(into_m[i].abs()) / h;
                    assert!(
                        (got - expected).abs() <= tol_rel * scale + tol_abs + noise_floor,
                        "{:?} sample {} stamp ({}, {}): jacobian {:.6e} vs finite-difference {:.6e} at V={:?}",
                        device.device_type,
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

    #[test]
    fn diode_jacobian_matches_finite_difference() {
        let device = NonlinearDeviceInstance::diode(0, 1, 1e-14, 1.0);
        assert_jacobian_matches_finite_difference(&device, 2, (-1.0, 1.0), 40, 11);

        let device = NonlinearDeviceInstance::diode(0, 1, 2.5e-9, 1.8);
        assert_jacobian_matches_finite_difference(&device, 2, (-1.0, 1.0), 40, 13);
    }

    #[test]
    fn bjt_jacobians_match_finite_difference() {
        let npn = NonlinearDeviceInstance::npn_bjt(0, 1, 2, 1e-14, 120.0, 3.0, 1.1, 1.05, 80.0);
        assert_jacobian_matches_finite_difference(&npn, 3, (-0.9, 0.9), 60, 17);

        let pnp = NonlinearDeviceInstance::pnp_bjt(0, 1, 2, 2e-14, 80.0, 2.0, 1.0, 1.2, 60.0);
        assert_jacobian_matches_finite_difference(&pnp, 3, (-0.9, 0.9), 60, 19);
    }

    #[test]
    fn mosfet_jacobians_match_finite_difference() {
        let nmos = NonlinearDeviceInstance::nmos(0, 1, 2, 3, 0.7, 2e-5, 0.04);
        assert_jacobian_matches_finite_difference(&nmos, 4, (-3.0, 3.0), 80, 23);

        let pmos = NonlinearDeviceInstance::pmos(0, 1, 2, 3, 0.7, 1.2e-5, 0.05);
        assert_jacobian_matches_finite_difference(&pmos, 4, (-3.0, 3.0), 80, 29);
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

        let isw =
            NonlinearDeviceInstance::current_switch(0, 1, 2, 3, 1e-3, 0.0, 1.0, 1e6, 1e-4, 1e-2);
        assert_jacobian_matches_finite_difference(&isw, 4, (-2.0, 2.0), 60, 43);
    }

    /// At Vce = 0 the forward and reverse transport terms cancel exactly, so
    /// the collector current must reduce to the reverse-recombination term
    /// -Is*(exp(Vbe/Vt) - 1)/BR. The injection-style formula (i_f - i_r/br)
    /// instead predicts a large positive residue, so this pins the transport
    /// formulation through the saturation region.
    #[test]
    fn collector_current_vanishing_vce_reduces_to_recombination_term() {
        let br = 2.0;
        let is = 1e-14;
        let npn = NonlinearDeviceInstance::npn_bjt(0, 1, 2, is, 100.0, br, 1.0, 1.0, f64::INFINITY);

        // Vc = Ve = 0, Vb = 0.7: both junctions equally forward biased.
        let v = vec![0.0, 0.7, 0.0];
        let currents = npn.evaluate(&v);
        let into_collector = currents
            .iter()
            .filter(|(node, _)| *node == 0)
            .map(|(_, c)| *c)
            .sum::<Value>();

        let vt = 0.02585;
        let i_r = is * ((0.7_f64 / vt).exp() - 1.0);
        let expected_into_collector = i_r / br; // -ic with ic = -i_r/br

        let err = (into_collector - expected_into_collector).abs();
        assert!(
            err <= 1e-9 * expected_into_collector.abs(),
            "collector current at Vce=0 must be the recombination term: got {into_collector:.6e}, want {expected_into_collector:.6e}"
        );
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

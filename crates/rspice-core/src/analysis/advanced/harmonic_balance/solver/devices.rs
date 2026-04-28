//! Nonlinear device parameter builders and time-domain evaluation helpers.

use super::*;

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
    pub fn bjt(is: Value, bf: Value, br: Value, vaf: Value) -> Self {
        Self {
            is,
            bf,
            br,
            vaf,
            ..Default::default()
        }
    }

    /// Create MOSFET parameters
    pub fn mosfet(vth: Value, kp: Value, lambda: Value) -> Self {
        Self {
            vth,
            kp,
            lambda,
            ..Default::default()
        }
    }

    /// Create JFET parameters
    pub fn jfet(vto: Value, beta: Value, lambda: Value) -> Self {
        Self {
            vth: vto,
            kp: beta,
            lambda,
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
    pub fn npn_bjt(collector: usize, base: usize, emitter: usize, is: Value, bf: Value) -> Self {
        Self {
            device_type: NonlinearDeviceType::NpnBjt,
            terminals: vec![collector, base, emitter],
            params: NonlinearDeviceParams::bjt(is, bf, 1.0, f64::INFINITY),
        }
    }

    /// Create a PNP BJT instance
    pub fn pnp_bjt(collector: usize, base: usize, emitter: usize, is: Value, bf: Value) -> Self {
        Self {
            device_type: NonlinearDeviceType::PnpBjt,
            terminals: vec![collector, base, emitter],
            params: NonlinearDeviceParams::bjt(is, bf, 1.0, f64::INFINITY),
        }
    }

    /// Create an NMOS instance
    pub fn nmos(
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        vth: Value,
        kp: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Nmos,
            terminals: vec![drain, gate, source, bulk],
            params: NonlinearDeviceParams::mosfet(vth, kp, 0.0),
        }
    }

    /// Create a PMOS instance
    pub fn pmos(
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        vth: Value,
        kp: Value,
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Pmos,
            terminals: vec![drain, gate, source, bulk],
            params: NonlinearDeviceParams::mosfet(vth, kp, 0.0),
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
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Njfet,
            terminals: vec![drain, gate, source],
            params: NonlinearDeviceParams::jfet(vto, beta, lambda),
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
    ) -> Self {
        Self {
            device_type: NonlinearDeviceType::Pjfet,
            terminals: vec![drain, gate, source],
            params: NonlinearDeviceParams::jfet(vto, beta, lambda),
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

        // Shockley equation with limiting
        let arg = (vd / (self.params.n * self.params.vt)).clamp(-40.0, 40.0);
        let id = self.params.is * (arg.exp() - 1.0);

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

        // Conductance gd = dI_d/dV = Is/(n*Vt) * exp(Vd/(n*Vt))
        let arg = (vd / (self.params.n * self.params.vt)).clamp(-40.0, 40.0);
        let gd = (self.params.is / (self.params.n * self.params.vt)) * arg.exp();
        let gd = gd.max(1e-12); // Minimum conductance for numerical stability

        let a = self.terminals[0];
        let c = self.terminals[1];

        // Return MNA conductance stamp
        // Physical conductance gd is POSITIVE. MNA stamp for 2-terminal conductance:
        // G[n+,n+] += gd, G[n+,n-] -= gd, G[n-,n+] -= gd, G[n-,n-] += gd
        vec![((a, a), gd), ((a, c), -gd), ((c, a), -gd), ((c, c), gd)]
    }

    fn eval_npn_bjt(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let vbe = v_b - v_e;
        let vbc = v_b - v_c;

        // Ebers-Moll transport model
        let arg_be = (vbe / self.params.vt).clamp(-40.0, 40.0);
        let arg_bc = (vbc / self.params.vt).clamp(-40.0, 40.0);

        let i_f = self.params.is * (arg_be.exp() - 1.0);
        let i_r = self.params.is * (arg_bc.exp() - 1.0);

        let ic = i_f - i_r / self.params.br;
        let ib = i_f / self.params.bf + i_r / self.params.br;
        let ie = -(ic + ib); // KCL

        vec![
            (self.terminals[0], -ic), // Collector current out
            (self.terminals[1], -ib), // Base current out
            (self.terminals[2], -ie), // Emitter current out
        ]
    }

    fn jac_npn_bjt(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let vbe = v_b - v_e;
        let vbc = v_b - v_c;

        let arg_be = (vbe / self.params.vt).clamp(-40.0, 40.0);
        let arg_bc = (vbc / self.params.vt).clamp(-40.0, 40.0);

        // Transconductances
        let gm_f = (self.params.is / self.params.vt) * arg_be.exp();
        let gm_r = (self.params.is / self.params.vt) * arg_bc.exp();

        let c = self.terminals[0];
        let b = self.terminals[1];
        let e = self.terminals[2];

        // Simplified linearized model - gm stamps
        let gbe = gm_f / self.params.bf;
        let gbc = gm_r / self.params.br;

        vec![
            // Base-emitter conductance
            ((b, b), gbe + gbc), // Combined: gbe from B-E + gbc from B-C
            ((b, e), -gbe),
            ((e, b), -gbe),
            ((e, e), gbe),
            // Base-collector conductance (gbc stamps)
            ((b, c), -gbc),
            ((c, b), gm_f - gbc), // Combined: gm_f transconductance + (-gbc) from B-C conductance
            ((c, c), gbc),
            // Transconductance gm stamps (collector controlled by Vbe)
            // Note: (c, b) contribution from gm_f already combined above
            ((c, e), -gm_f),
        ]
    }

    fn eval_pnp_bjt(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        // PNP is NPN with inverted voltages and currents
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let veb = v_e - v_b; // Inverted from NPN
        let vcb = v_c - v_b;

        let arg_eb = (veb / self.params.vt).clamp(-40.0, 40.0);
        let arg_cb = (vcb / self.params.vt).clamp(-40.0, 40.0);

        let i_f = self.params.is * (arg_eb.exp() - 1.0);
        let i_r = self.params.is * (arg_cb.exp() - 1.0);

        let ic = -(i_f - i_r / self.params.br); // Inverted
        let ib = -(i_f / self.params.bf + i_r / self.params.br);
        let ie = -(ic + ib);

        vec![
            (self.terminals[0], -ic),
            (self.terminals[1], -ib),
            (self.terminals[2], -ie),
        ]
    }

    fn jac_pnp_bjt(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        // PNP Jacobian with proper polarity handling
        // PNP uses Veb = Ve - Vb and Vcb = Vc - Vb
        let v_c = self.get_terminal_voltage(node_voltages, 0);
        let v_b = self.get_terminal_voltage(node_voltages, 1);
        let v_e = self.get_terminal_voltage(node_voltages, 2);

        let veb = v_e - v_b;
        let vcb = v_c - v_b;

        let arg_eb = (veb / self.params.vt).clamp(-40.0, 40.0);
        let arg_cb = (vcb / self.params.vt).clamp(-40.0, 40.0);

        // Transconductances for PNP (based on Veb and Vcb)
        let gm_f = (self.params.is / self.params.vt) * arg_eb.exp();
        let gm_r = (self.params.is / self.params.vt) * arg_cb.exp();

        let c = self.terminals[0];
        let b = self.terminals[1];
        let e = self.terminals[2];

        // PNP junction conductances
        let geb = gm_f / self.params.bf; // Emitter-base conductance
        let gcb = gm_r / self.params.br; // Collector-base conductance

        vec![
            // Emitter-base junction conductance (geb)
            // dI/dVeb stamps - note PNP has Ve > Vb for forward bias
            ((e, e), geb),
            ((e, b), -geb),
            ((b, e), -geb),
            ((b, b), geb + gcb), // Combined: geb from E-B + gcb from C-B
            // Collector-base junction conductance (gcb)
            ((c, c), gcb),
            ((c, b), -(gcb + gm_f)), // Combined: -gcb from C-B conductance + (-gm_f) from transconductance
            ((b, c), -gcb),
            // Transconductance: collector controlled by Veb
            // For PNP: Ic depends on Veb, so dIc/dVe
            // Note: (c, b) contribution already combined above
            ((c, e), gm_f),
        ]
    }

    fn eval_nmos(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);
        // v_b = self.get_terminal_voltage(node_voltages, 3); // bulk, simplified for now

        // MOSFET is symmetric - swap D/S when Vds < 0
        let (vgs, vds, is_reversed) = if v_d >= v_s {
            (v_g - v_s, v_d - v_s, false)
        } else {
            (v_g - v_d, v_s - v_d, true)
        };

        let id = if vgs <= self.params.vth {
            // Cutoff
            0.0
        } else if vds < vgs - self.params.vth {
            // Triode
            self.params.kp * ((vgs - self.params.vth) * vds - 0.5 * vds * vds)
        } else {
            // Saturation
            0.5 * self.params.kp
                * (vgs - self.params.vth).powi(2)
                * (1.0 + self.params.lambda * vds)
        };

        // Current direction depends on whether D/S were swapped
        if is_reversed {
            vec![
                (self.terminals[0], id),  // "Drain" receives current
                (self.terminals[2], -id), // "Source" supplies current
            ]
        } else {
            vec![
                (self.terminals[0], -id), // Drain current out
                (self.terminals[2], id),  // Source current in
            ]
        }
    }

    fn jac_nmos(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];

        // MOSFET is symmetric - swap D/S when Vds < 0
        let (vgs, vds, eff_d, eff_s) = if v_d >= v_s {
            (v_g - v_s, v_d - v_s, d, s)
        } else {
            (v_g - v_d, v_s - v_d, s, d) // Swap effective drain/source
        };

        if vgs <= self.params.vth {
            // Cutoff - small conductance
            return vec![((d, d), 1e-12), ((s, s), 1e-12)];
        }

        let (gm, gds) = if vds < vgs - self.params.vth {
            // Triode
            let gm = self.params.kp * vds;
            let gds = self.params.kp * (vgs - self.params.vth - vds);
            (gm, gds.max(1e-12))
        } else {
            // Saturation
            let gm = self.params.kp * (vgs - self.params.vth) * (1.0 + self.params.lambda * vds);
            let gds = 0.5 * self.params.kp * (vgs - self.params.vth).powi(2) * self.params.lambda;
            (gm, gds.max(1e-12))
        };

        // Use effective drain/source for stamps
        vec![
            // gds: D-S conductance
            ((eff_d, eff_d), gds),
            ((eff_d, eff_s), -(gds + gm)), // Combined: -gds from conductance, -gm from transconductance
            ((eff_s, eff_d), -gds),
            ((eff_s, eff_s), gds),
            // gm: transconductance (D controlled by Vg)
            ((eff_d, g), gm),
        ]
    }

    fn eval_pmos(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        // PMOS has inverted voltages
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        // PMOS is symmetric - swap S/D when Vsd < 0
        let (vsg, vsd, is_reversed) = if v_s >= v_d {
            (v_s - v_g, v_s - v_d, false)
        } else {
            (v_d - v_g, v_d - v_s, true)
        };

        let vth = self.params.vth.abs();
        let id = if vsg <= vth {
            0.0
        } else if vsd < vsg - vth {
            // Triode
            self.params.kp * ((vsg - vth) * vsd - 0.5 * vsd * vsd)
        } else {
            // Saturation
            0.5 * self.params.kp * (vsg - vth).powi(2)
        };

        // Current direction depends on whether S/D were swapped
        if is_reversed {
            vec![
                (self.terminals[0], -id), // "Drain" supplies current
                (self.terminals[2], id),  // "Source" receives current
            ]
        } else {
            vec![
                (self.terminals[0], id),  // Drain current in (PMOS)
                (self.terminals[2], -id), // Source current out
            ]
        }
    }

    fn jac_pmos(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let v_d = self.get_terminal_voltage(node_voltages, 0);
        let v_g = self.get_terminal_voltage(node_voltages, 1);
        let v_s = self.get_terminal_voltage(node_voltages, 2);

        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];

        // PMOS is symmetric - swap S/D when Vsd < 0
        let (vsg, vsd, eff_s, eff_d) = if v_s >= v_d {
            (v_s - v_g, v_s - v_d, s, d)
        } else {
            (v_d - v_g, v_d - v_s, d, s) // Swap effective source/drain
        };

        let vth = self.params.vth.abs();
        if vsg <= vth {
            // Cutoff - small conductance
            return vec![((d, d), 1e-12), ((s, s), 1e-12)];
        }

        let (gm, gsd) = if vsd < vsg - vth {
            // Triode
            let gm = self.params.kp * vsd;
            let gsd = self.params.kp * (vsg - vth - vsd);
            (gm, gsd.max(1e-12))
        } else {
            // Saturation
            let gm = self.params.kp * (vsg - vth);
            let gsd = 1e-12; // Small output conductance (lambda=0 simplified)
            (gm, gsd)
        };

        // Use effective source/drain for stamps (PMOS: current from S to D)
        vec![
            // gsd: S-D conductance
            ((eff_s, eff_s), gsd),
            ((eff_s, eff_d), -(gsd + gm)), // Combined
            ((eff_d, eff_s), -gsd),
            ((eff_d, eff_d), gsd),
            // gm: transconductance (S controlled by Vg for PMOS)
            ((eff_s, g), -gm), // PMOS: current decreases with increasing Vg
        ]
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

    fn eval_njfet(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (id, _, _) = self.jfet_ids_gm_gds(node_voltages, 1.0);
        vec![
            (self.terminals[0], -id), // Drain current leaving
            (self.terminals[2], id),  // Source current entering
        ]
    }

    fn jac_njfet(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (_, gm, gds) = self.jfet_ids_gm_gds(node_voltages, 1.0);
        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];
        vec![
            ((d, d), gds),
            ((d, s), -(gds + gm)),
            ((s, d), -gds),
            ((s, s), gds),
            ((d, g), gm),
            ((s, g), -gm),
        ]
    }

    fn eval_pjfet(&self, node_voltages: &[Value]) -> Vec<(usize, Value)> {
        let (id, _, _) = self.jfet_ids_gm_gds(node_voltages, -1.0);
        vec![
            (self.terminals[0], -id), // Drain current leaving
            (self.terminals[2], id),  // Source current entering
        ]
    }

    fn jac_pjfet(&self, node_voltages: &[Value]) -> Vec<((usize, usize), Value)> {
        let (_, gm, gds) = self.jfet_ids_gm_gds(node_voltages, -1.0);
        let d = self.terminals[0];
        let g = self.terminals[1];
        let s = self.terminals[2];
        vec![
            ((d, d), gds),
            ((d, s), -(gds + gm)),
            ((s, d), -gds),
            ((s, s), gds),
            ((d, g), gm),
            ((s, g), -gm),
        ]
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

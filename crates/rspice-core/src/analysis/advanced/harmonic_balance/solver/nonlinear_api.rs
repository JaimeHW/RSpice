//! Public nonlinear-device registration helpers for the HB solver.

use super::*;
impl HbSolver {
    /// Add a nonlinear device for Newton iteration
    pub fn add_nonlinear_device(&mut self, device: NonlinearDeviceInstance) {
        self.nonlinear_devices.push(device);
    }

    /// Add a diode for Newton iteration
    pub fn add_diode(&mut self, anode: usize, cathode: usize, is: Value, n: Value) {
        self.add_nonlinear_device(NonlinearDeviceInstance::diode(anode, cathode, is, n));
    }

    /// Add an NPN BJT for Newton iteration
    pub fn add_npn_bjt(
        &mut self,
        collector: usize,
        base: usize,
        emitter: usize,
        is: Value,
        bf: Value,
        br: Value,
        nf: Value,
        nr: Value,
        vaf: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::npn_bjt(
            collector, base, emitter, is, bf, br, nf, nr, vaf,
        ));
    }

    /// Add a PNP BJT for Newton iteration
    pub fn add_pnp_bjt(
        &mut self,
        collector: usize,
        base: usize,
        emitter: usize,
        is: Value,
        bf: Value,
        br: Value,
        nf: Value,
        nr: Value,
        vaf: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::pnp_bjt(
            collector, base, emitter, is, bf, br, nf, nr, vaf,
        ));
    }

    /// Add an NMOS for Newton iteration
    pub fn add_nmos(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        kp: Value,
        vth: Value,
        lambda: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::nmos(
            drain, gate, source, bulk, vth, kp, lambda,
        ));
    }

    /// Add a PMOS for Newton iteration (`vth` is the effective threshold, -VTO)
    pub fn add_pmos(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        bulk: usize,
        kp: Value,
        vth: Value,
        lambda: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::pmos(
            drain, gate, source, bulk, vth, kp, lambda,
        ));
    }

    /// Add an N-channel JFET for Newton iteration
    pub fn add_njfet(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
        is: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::njfet(
            drain, gate, source, vto, beta, lambda, is,
        ));
    }

    /// Add a P-channel JFET for Newton iteration
    pub fn add_pjfet(
        &mut self,
        drain: usize,
        gate: usize,
        source: usize,
        vto: Value,
        beta: Value,
        lambda: Value,
        is: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::pjfet(
            drain, gate, source, vto, beta, lambda, is,
        ));
    }

    /// Add a voltage-controlled switch for Newton iteration
    pub fn add_voltage_switch(
        &mut self,
        node_pos: usize,
        node_neg: usize,
        ctrl_pos: usize,
        ctrl_neg: usize,
        vt: Value,
        vh: Value,
        ron: Value,
        roff: Value,
        smooth: Value,
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::voltage_switch(
            node_pos, node_neg, ctrl_pos, ctrl_neg, vt, vh, ron, roff, smooth,
        ));
    }

    /// Add a current-controlled switch for Newton iteration.
    pub fn add_current_switch(
        &mut self,
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
    ) {
        self.add_nonlinear_device(NonlinearDeviceInstance::current_switch(
            node_pos,
            node_neg,
            ctrl_pos,
            ctrl_neg,
            it,
            ih,
            ron,
            roff,
            smooth,
            control_gain,
        ));
    }

    /// Add a Verilog-A nonlinear device for Newton iteration.
    #[cfg(feature = "veriloga")]
    pub fn add_veriloga_device(&mut self, device: VerilogADevice) {
        self.veriloga_nonlinear_devices
            .push(HbVerilogADevice::new(device));
    }

    /// Check if circuit has nonlinear devices
    pub fn has_nonlinear_devices(&self) -> bool {
        if !self.nonlinear_devices.is_empty() {
            return true;
        }
        #[cfg(feature = "veriloga")]
        {
            if !self.veriloga_nonlinear_devices.is_empty() {
                return true;
            }
        }
        false
    }

    // =========================================================================
    // DC Operating Point Solver
    // =========================================================================
    //
    // Solves the DC component (harmonic 0) before full HB iteration.
    // This establishes nonlinear device operating points and provides a
    // much better initial guess for the HB Newton iteration.
    //
    // Flow: DC Solve â†’ Initialize Harmonics â†’ Full HB Newton
    // =========================================================================
}

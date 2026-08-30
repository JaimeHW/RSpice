//! Public nonlinear-device registration helpers for the HB solver.

use super::*;
impl HbSolver {
    /// Add a nonlinear device for Newton iteration
    pub fn add_nonlinear_device(&mut self, device: NonlinearDeviceInstance) {
        self.add_nonlinear_device_with_noise_temperature(
            device,
            NonlinearNoiseTemperature::Ambient,
        );
    }

    /// Add an engine-resolved nonlinear device while retaining its
    /// instance-specific thermal-noise temperature offset.
    pub(crate) fn add_nonlinear_device_with_noise_temperature_offset(
        &mut self,
        device: NonlinearDeviceInstance,
        noise_temperature_offset: Value,
    ) {
        self.add_nonlinear_device_with_noise_temperature(
            device,
            NonlinearNoiseTemperature::Offset(noise_temperature_offset),
        );
    }

    /// Add an engine-resolved nonlinear device with an absolute instance TEMP.
    pub(crate) fn add_nonlinear_device_with_absolute_noise_temperature(
        &mut self,
        device: NonlinearDeviceInstance,
        noise_temperature: Value,
    ) {
        self.add_nonlinear_device_with_noise_temperature(
            device,
            NonlinearNoiseTemperature::Absolute(noise_temperature),
        );
    }

    fn add_nonlinear_device_with_noise_temperature(
        &mut self,
        device: NonlinearDeviceInstance,
        noise_temperature: NonlinearNoiseTemperature,
    ) {
        self.nonlinear_devices.push(device);
        self.nonlinear_noise_temperatures.push(noise_temperature);
    }

    /// Add a diode for Newton iteration
    pub fn add_diode(&mut self, anode: usize, cathode: usize, is: Value, n: Value) {
        self.add_nonlinear_device(NonlinearDeviceInstance::diode(anode, cathode, is, n));
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
    // Flow: DC Solve → Initialize Harmonics → Full HB Newton
    // =========================================================================
}

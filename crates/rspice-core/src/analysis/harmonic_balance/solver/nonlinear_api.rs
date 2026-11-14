//! Public nonlinear-device registration helpers for the HB solver.

use super::*;
impl HbSolver {
    /// Add a nonlinear device for Newton iteration
    pub fn add_nonlinear_device(&mut self, device: NonlinearDeviceInstance) {
        let fallback_name = format!("{:?}#{}", device.device_type, self.nonlinear_devices.len());
        self.add_wrapped_nonlinear_device_with_noise_temperature(
            fallback_name,
            HbNonlinearDevice {
                device,
                resolved_diode_junction: None,
            },
            NonlinearNoiseTemperature::Ambient,
        );
    }

    /// Register an engine-resolved nonlinear device under its authored name.
    pub(crate) fn add_named_nonlinear_device(
        &mut self,
        name: impl Into<String>,
        device: NonlinearDeviceInstance,
    ) {
        self.add_wrapped_nonlinear_device_with_noise_temperature(
            name,
            HbNonlinearDevice {
                device,
                resolved_diode_junction: None,
            },
            NonlinearNoiseTemperature::Ambient,
        );
    }

    /// Register an engine-resolved Level-1 diode without changing the public
    /// direct-solver device representation.
    pub(crate) fn add_named_resolved_diode(
        &mut self,
        name: impl Into<String>,
        device: NonlinearDeviceInstance,
        junction: crate::device::semiconductor::ResolvedDiodeJunction,
    ) {
        self.add_wrapped_nonlinear_device_with_noise_temperature(
            name,
            HbNonlinearDevice {
                device,
                resolved_diode_junction: Some(junction),
            },
            NonlinearNoiseTemperature::Ambient,
        );
    }

    /// Add an engine-resolved nonlinear device while retaining its
    /// instance-specific thermal-noise temperature offset.
    pub(crate) fn add_named_nonlinear_device_with_noise_temperature_offset(
        &mut self,
        name: impl Into<String>,
        device: NonlinearDeviceInstance,
        noise_temperature_offset: Value,
    ) {
        self.add_wrapped_nonlinear_device_with_noise_temperature(
            name,
            HbNonlinearDevice {
                device,
                resolved_diode_junction: None,
            },
            NonlinearNoiseTemperature::Offset(noise_temperature_offset),
        );
    }

    /// Add an engine-resolved nonlinear device with an absolute instance TEMP.
    pub(crate) fn add_named_nonlinear_device_with_absolute_noise_temperature(
        &mut self,
        name: impl Into<String>,
        device: NonlinearDeviceInstance,
        noise_temperature: Value,
    ) {
        self.add_wrapped_nonlinear_device_with_noise_temperature(
            name,
            HbNonlinearDevice {
                device,
                resolved_diode_junction: None,
            },
            NonlinearNoiseTemperature::Absolute(noise_temperature),
        );
    }

    fn add_wrapped_nonlinear_device_with_noise_temperature(
        &mut self,
        name: impl Into<String>,
        device: HbNonlinearDevice,
        noise_temperature: NonlinearNoiseTemperature,
    ) {
        self.nonlinear_devices.push(device);
        self.nonlinear_device_names.push(name.into());
        self.nonlinear_noise_temperatures.push(noise_temperature);
    }

    /// Add a diode for Newton iteration
    #[cfg(test)]
    pub fn add_diode(&mut self, anode: usize, cathode: usize, is: Value, n: Value) {
        self.add_nonlinear_device(NonlinearDeviceInstance::diode(anode, cathode, is, n));
    }

    /// Add a Verilog-A instance to the F/Q sampler. Its residual must be
    /// `F(x) + dQ(x)/dt`, which the harmonic-balance capability gate decides
    /// before the solver is built.
    #[cfg(feature = "veriloga")]
    pub(crate) fn try_add_veriloga_device(
        &mut self,
        device: crate::device::veriloga::VerilogADevice,
    ) -> Result<(), HbError> {
        // Branch currents and mathematical states are unknowns, not node
        // voltages: no shunt conductance may enter their equations.
        for node in device.non_electrical_node_indices() {
            if node > self.num_nodes {
                return Err(HbError::InvalidCircuit(format!(
                    "Verilog-A device '{}' has unregistered periodic unknowns",
                    device.name
                )));
            }
            if let Err(index) = self.non_electrical_nodes.binary_search(&(node - 1)) {
                self.non_electrical_nodes.insert(index, node - 1);
            }
        }
        // Each operator site with state carries its input and its output as
        // unknowns of its own; a network ties them at every harmonic.
        let mut ports = Vec::new();
        for site in device.periodic_operator_sites() {
            let mut rows = [0; 2];
            for (row, role) in rows.iter_mut().zip(["input", "output"]) {
                let branch_ordinal = self.exact_mna_branches().len() + 1;
                *row = self.num_nodes + self.exact_mna_branches().len();
                self.try_push_periodic_mna_branch(
                    ExactMnaBranch::AuxiliaryState { branch_ordinal },
                    &format!("{}:{:?}{}:{role}", device.name, site.kind, site.slot),
                )?;
            }
            ports.push((rows[0], rows[1]));
        }
        let unknowns = self.num_nodes + self.exact_mna_branches().len();
        let (device, networks) =
            HbVerilogADevice::try_new(device, self.config.fundamental_freq, unknowns, &ports)?;
        self.veriloga_nonlinear_devices.push(device);
        for network in networks {
            self.try_add_exact_periodic_network(ExactPeriodicNetwork::VerilogAOperator {
                name: network.name,
                input: network.input,
                output: network.output,
                transfer: network.transfer,
            })?;
        }
        Ok(())
    }

    /// Check if circuit has nonlinear devices
    pub fn has_nonlinear_devices(&self) -> bool {
        !self.nonlinear_devices.is_empty() || self.has_native_periodic_devices()
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

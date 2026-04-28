use super::*;

impl CircuitData {
    /// Check if circuit has any nonlinear devices requiring Newton-Raphson
    pub fn has_nonlinear_devices(&self) -> bool {
        !self.diodes.is_empty()
            || !self.bjts.is_empty()
            || !self.mosfets.is_empty()
            || !self.jfets.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || !self.behavioral_sources.is_empty()
            || self.has_xspice_devices()
            || {
                #[cfg(feature = "veriloga")]
                {
                    self.has_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    false
                }
            }
    }

    /// Check whether circuit contains strongly-coupled physical nonlinearities
    /// that benefit from conservative Newton damping (e.g., voltage limiting).
    #[inline]
    pub fn has_physical_nonlinear_devices(&self) -> bool {
        !self.diodes.is_empty()
            || !self.bjts.is_empty()
            || !self.mosfets.is_empty()
            || !self.jfets.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || self.has_xspice_devices()
            || {
                #[cfg(feature = "veriloga")]
                {
                    self.has_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga"))]
                {
                    false
                }
            }
    }

    /// Update all nonlinear devices with current solution
    pub fn update_nonlinear(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        self.diodes.update_all(voltages);
        self.bjts.update_all(voltages);
        self.mosfets.update_all(voltages);
        let mut order: Vec<usize> = (0..self.jfets.len()).collect();
        order.sort_by_key(|&idx| (self.jfets[idx].model_order(), idx));
        let mut hfet_inverse_latched = false;
        for idx in order {
            let jfet = &mut self.jfets[idx];
            let uses_hfet_legacy_inverse = jfet.uses_hfet_legacy_inverse_mode();
            jfet.set_hfet_legacy_inverse_active(uses_hfet_legacy_inverse && hfet_inverse_latched);
            jfet.update(voltages);
            if uses_hfet_legacy_inverse && jfet.internal_vds_limited_state() < 0.0 {
                hfet_inverse_latched = true;
            }
        }
        for vswitch in &mut self.vswitches {
            vswitch.update(voltages);
        }
        for iswitch in &mut self.iswitches {
            iswitch.update(voltages);
        }
        #[cfg(feature = "veriloga")]
        {
            self.veriloga_devices_mut().update_all_voltages(voltages);
        }
    }

    /// Stamp all nonlinear devices into matrix using O(1) direct indexing
    pub fn stamp_nonlinear(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        use crate::device::NonlinearDevice;
        self.diodes.stamp_all_direct(matrix, rhs, voltages);
        self.bjts.stamp_all_direct(matrix, rhs, voltages);
        self.mosfets.stamp_all_direct(matrix, rhs, voltages);
        for jfet in &self.jfets {
            jfet.stamp_direct(matrix, rhs, voltages);
        }
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for vswitch in &self.vswitches {
            vswitch.stamp_nonlinear(voltages, &mut stamper, &mut []);
        }
        for iswitch in &self.iswitches {
            iswitch.stamp_nonlinear(voltages, &mut stamper, &mut []);
        }
        #[cfg(feature = "veriloga")]
        {
            let veriloga_devices = self.veriloga_devices_mut();
            veriloga_devices.update_all_voltages(voltages);
            veriloga_devices.stamp_all(
                voltages,
                |row, col, value| matrix.add(row, col, value),
                |index, value| {
                    if let Some(slot) = rhs.get_mut(index) {
                        *slot += value;
                    }
                },
            );
        }
    }

    /// Stamp behavioral sources with the given analysis time.
    pub fn stamp_behavioral(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) {
        self.behavioral_sources
            .stamp_all(matrix, rhs, solution, self.num_nodes, time);
    }

    /// Check if all nonlinear devices have converged
    pub fn nonlinear_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        self.diodes.all_converged(criteria)
            && self.bjts.all_converged(criteria)
            && self.mosfets.all_converged(criteria)
            && self.jfets.iter().all(|jfet| jfet.is_converged(criteria))
            && self.vswitches.iter().all(|sw| sw.is_converged(criteria))
            && self.iswitches.iter().all(|sw| sw.is_converged(criteria))
            && self.xspice_converged(criteria.voltage_tolerance())
    }
}

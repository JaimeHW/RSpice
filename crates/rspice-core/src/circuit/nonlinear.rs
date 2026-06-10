use super::*;

#[derive(Debug, Clone)]
pub(crate) struct NonlinearDeviceStateSnapshot {
    diodes: Diodes,
    bjts: Bjts,
    mosfets: Mosfets,
    b3soi: B3SoiDds,
    b3soi_fd: B3SoiFds,
    b3soi_pd: B3SoiPds,
    jfets: Vec<crate::device::Jfet>,
    vswitches: Vec<crate::device::VoltageSwitch>,
    iswitches: Vec<crate::device::CurrentSwitch>,
    behavioral_sources: BehavioralSources,
    xspice_instances: Vec<XspiceInstance>,
    #[cfg(feature = "veriloga")]
    veriloga_devices: crate::device::veriloga::VerilogADevices,
}

impl CircuitData {
    /// Check if circuit has any nonlinear devices requiring Newton-Raphson
    pub fn has_nonlinear_devices(&self) -> bool {
        !self.diodes.is_empty()
            || !self.bjts.is_empty()
            || !self.mosfets.is_empty()
            || !self.b3soi.is_empty()
            || !self.b3soi_fd.is_empty()
            || !self.b3soi_pd.is_empty()
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
            || !self.b3soi.is_empty()
            || !self.b3soi_fd.is_empty()
            || !self.b3soi_pd.is_empty()
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

    /// Check whether the nonlinear solve should apply global nodal damping.
    ///
    /// Classic JFETs already use ngspice-style local gate-branch limiting
    /// (`DEVpnjlim`/`DEVfetlim`). For circuits made only from those devices,
    /// global line-search damping adds substantial cost without improving the
    /// Newton path. Keep it enabled for other compact models and mixed
    /// nonlinear circuits where device-local limiting is not sufficient.
    #[inline]
    pub fn requires_conservative_solution_damping(&self) -> bool {
        if !self.diodes.is_empty()
            || !self.bjts.is_empty()
            || !self.mosfets.is_empty()
            || !self.b3soi.is_empty()
            || !self.b3soi_fd.is_empty()
            || !self.b3soi_pd.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || self.has_xspice_devices()
        {
            return true;
        }

        #[cfg(feature = "veriloga")]
        {
            if self.has_veriloga_devices() {
                return true;
            }
        }

        self.jfets.iter().any(|jfet| {
            !matches!(
                jfet.params.channel_model,
                crate::device::JfetChannelModel::ShichmanHodges
            )
        })
    }

    /// Set the circuit-level semiconductor junction GMIN seen by compact models.
    ///
    /// ngspice passes the active `CKTgmin` into device junction branches during
    /// gmin stepping, not only as a nodal shunt. Keep model-local gate/body
    /// diode loading aligned with the continuation stage.
    pub fn set_semiconductor_junction_gmin(&mut self, gmin: Value) {
        let gmin = if gmin.is_finite() && gmin > 0.0 {
            gmin
        } else {
            0.0
        };
        for mos in &mut self.mosfets.devices {
            mos.set_junction_gmin(gmin);
        }
        for jfet in &mut self.jfets {
            jfet.set_junction_gmin(gmin);
        }
    }

    /// Return true when any JFET-family compact model exposes a stiff gate
    /// generation-recombination branch that can benefit from homotopy.
    pub(crate) fn has_jfet_gate_generation_branches(&self) -> bool {
        self.jfets
            .iter()
            .any(crate::device::Jfet::has_gate_generation_branch)
    }

    /// Scale JFET-family gate generation-recombination branches during
    /// continuation. A scale of 1.0 restores the physical model equations.
    pub(crate) fn set_jfet_gate_generation_scale(&mut self, scale: Value) {
        for jfet in &mut self.jfets {
            if jfet.has_gate_generation_branch() {
                jfet.set_gate_generation_scale(scale);
            }
        }
    }

    /// Capture mutable nonlinear evaluation state before probing trial residuals.
    ///
    /// Newton line-search and fallback merit functions evaluate rejected trial
    /// points. Those probes must not commit device limiter caches, previous
    /// voltages, behavioral-source linearization scratch, or code-model context.
    pub(crate) fn nonlinear_state_snapshot(&self) -> NonlinearDeviceStateSnapshot {
        NonlinearDeviceStateSnapshot {
            diodes: self.diodes.clone(),
            bjts: self.bjts.clone(),
            mosfets: self.mosfets.clone(),
            b3soi: self.b3soi.clone(),
            b3soi_fd: self.b3soi_fd.clone(),
            b3soi_pd: self.b3soi_pd.clone(),
            jfets: self.jfets.clone(),
            vswitches: self.vswitches.clone(),
            iswitches: self.iswitches.clone(),
            behavioral_sources: self.behavioral_sources.clone(),
            xspice_instances: self.xspice_instances.clone(),
            #[cfg(feature = "veriloga")]
            veriloga_devices: self.veriloga_devices.clone(),
        }
    }

    /// Restore mutable nonlinear evaluation state after a trial residual probe.
    pub(crate) fn restore_nonlinear_state(&mut self, snapshot: NonlinearDeviceStateSnapshot) {
        self.diodes = snapshot.diodes;
        self.bjts = snapshot.bjts;
        self.mosfets = snapshot.mosfets;
        self.b3soi = snapshot.b3soi;
        self.b3soi_fd = snapshot.b3soi_fd;
        self.b3soi_pd = snapshot.b3soi_pd;
        self.jfets = snapshot.jfets;
        self.vswitches = snapshot.vswitches;
        self.iswitches = snapshot.iswitches;
        self.behavioral_sources = snapshot.behavioral_sources;
        self.xspice_instances = snapshot.xspice_instances;
        #[cfg(feature = "veriloga")]
        {
            self.veriloga_devices = snapshot.veriloga_devices;
        }
    }

    /// Update all nonlinear devices with current solution
    pub fn update_nonlinear(&mut self, voltages: &[Value]) {
        use crate::device::NonlinearDevice;
        self.diodes.update_all(voltages);
        self.bjts.update_all(voltages);
        self.mosfets.update_all(voltages);
        self.b3soi.update_all(voltages);
        self.b3soi_fd.update_all(voltages);
        self.b3soi_pd.update_all(voltages);
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

    /// Re-linearize JFET/MESFET devices directly at a static probe solution.
    ///
    /// Normal nonlinear updates intentionally apply ngspice-style branch
    /// limiting to protect Newton iterations. Residual and fallback validation
    /// probes, however, need the JFET stamp at the candidate voltage itself so
    /// they can measure the actual operating-point equation error.
    pub(crate) fn update_jfet_static_linearizations(&mut self, voltages: &[Value]) {
        let mut order: Vec<usize> = (0..self.jfets.len()).collect();
        order.sort_by_key(|&idx| (self.jfets[idx].model_order(), idx));
        let mut hfet_inverse_latched = false;
        for idx in order {
            let jfet = &mut self.jfets[idx];
            let uses_hfet_legacy_inverse = jfet.uses_hfet_legacy_inverse_mode();
            jfet.set_hfet_legacy_inverse_active(uses_hfet_legacy_inverse && hfet_inverse_latched);
            jfet.update_static_linearization(voltages);
            if uses_hfet_legacy_inverse && jfet.internal_vds_limited_state() < 0.0 {
                hfet_inverse_latched = true;
            }
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
        // B3SOIDD devices use the generic stamper path (1-indexed -> 0-indexed
        // handled by StaticMatrixStamper). Their DC conductance/current stamp is
        // applied here; the transient charge companion is added by the engine's
        // dedicated B3SOI transient pass.
        self.b3soi.stamp_all(&mut stamper, &mut [], voltages);
        self.b3soi_fd.stamp_all(&mut stamper, &mut [], voltages);
        self.b3soi_pd.stamp_all(&mut stamper, &mut [], voltages);
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
            && self.b3soi.all_converged(criteria)
            && self.b3soi_fd.all_converged(criteria)
            && self.b3soi_pd.all_converged(criteria)
            && self.jfets.iter().all(|jfet| jfet.is_converged(criteria))
            && self.vswitches.iter().all(|sw| sw.is_converged(criteria))
            && self.iswitches.iter().all(|sw| sw.is_converged(criteria))
            && self.xspice_converged(criteria.voltage_tolerance())
    }
}

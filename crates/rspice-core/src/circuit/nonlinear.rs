use super::*;

#[derive(Debug, Clone)]
pub(crate) struct NonlinearDeviceStateSnapshot {
    diodes: Diodes,
    bjts: Bjts,
    mosfets: Mosfets,
    b3soi: B3SoiDds,
    b3soi_fd: B3SoiFds,
    b3soi_pd: B3SoiPds,
    bsim3v3: Bsim3v3s,
    bsim4v8: Bsim4v8s,
    ekv26s: EkvMosfets,
    ekv3s: Ekv3Mosfets,
    vdmoses: Vdmoses,
    jfets: Vec<crate::device::Jfet>,
    vswitches: Vec<crate::device::VoltageSwitch>,
    iswitches: Vec<crate::device::CurrentSwitch>,
    generic_switches: Vec<crate::device::GenericSwitch>,
    behavioral_sources: BehavioralSources,
    xspice_instances: Vec<XspiceInstance>,
    xspice_digital_values: HashMap<NodeId, DigitalValue>,
    xspice_digital_drivers: HashMap<(NodeId, String, String), DigitalValue>,
    xspice_digital_event_times: HashMap<NodeId, Value>,
    xspice_real_values: HashMap<NodeId, Value>,
    xspice_real_drivers: HashMap<(NodeId, String, String), Value>,
    xspice_real_event_times: HashMap<NodeId, Value>,
    xspice_event_queue: EventQueue,
    #[cfg(feature = "veriloga")]
    veriloga_devices: crate::device::veriloga::VerilogADevices,
    #[cfg(feature = "veriloga-builtins")]
    generated_veriloga_devices: crate::device::veriloga_generated::BuiltinVerilogADevices,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::EkvMosfet;

    #[test]
    fn ekv26_consumes_circuit_junction_gmin() {
        let mut circuit = CircuitData::new();
        circuit
            .ekv26s
            .add(EkvMosfet::new_nmos("mekv".to_string(), 1, 2, 0, 0));

        circuit.set_semiconductor_junction_gmin(1.0e-8);

        assert_eq!(circuit.ekv26s.devices[0].eval_gmin(), 1.0e-8);

        circuit.set_semiconductor_junction_gmin(-1.0);

        assert_eq!(circuit.ekv26s.devices[0].eval_gmin(), 0.0);
    }
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
            || !self.bsim3v3.is_empty()
            || !self.bsim4v8.is_empty()
            || !self.ekv26s.is_empty()
            || !self.ekv3s.is_empty()
            || !self.vdmoses.is_empty()
            || !self.jfets.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || !self.behavioral_sources.is_empty()
            || self.has_xspice_devices()
            || {
                #[cfg(feature = "veriloga-builtins")]
                {
                    self.has_generated_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga-builtins"))]
                {
                    false
                }
            }
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

    /// Check if circuit has any BSIM3SOI device (DD, FD, or PD variant).
    ///
    /// The three variants share one transient charge-history pipeline
    /// (companion stamping, history commit, and charge LTE), indexed DD
    /// devices first, then FD, then PD.
    #[inline]
    pub fn has_b3soi_devices(&self) -> bool {
        !self.b3soi.is_empty() || !self.b3soi_fd.is_empty() || !self.b3soi_pd.is_empty()
    }

    /// Check if circuit has any native BSIM3v3.3 (level 8/49) device.
    ///
    /// Their coupled charge companion runs through a dedicated transient
    /// pipeline (companion stamping, history commit, charge LTE) parallel to
    /// the B3SOI one.
    #[inline]
    pub fn has_bsim3v3_devices(&self) -> bool {
        !self.bsim3v3.is_empty()
    }

    /// Check if circuit has any native BSIM4 v4.8 (level 14/54) device.
    ///
    /// Same dedicated transient pipeline shape as the BSIM3 one, over the
    /// BSIM4 composite charge states.
    #[inline]
    pub fn has_bsim4v8_devices(&self) -> bool {
        !self.bsim4v8.is_empty()
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
            || !self.bsim3v3.is_empty()
            || !self.bsim4v8.is_empty()
            || !self.ekv26s.is_empty()
            || !self.ekv3s.is_empty()
            || !self.vdmoses.is_empty()
            || !self.jfets.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || self
                .xspice_instances
                .iter()
                .any(|instance| instance.requires_conservative_newton_damping())
            || {
                #[cfg(feature = "veriloga-builtins")]
                {
                    self.has_generated_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga-builtins"))]
                {
                    false
                }
            }
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
            || !self.bsim3v3.is_empty()
            || !self.bsim4v8.is_empty()
            || !self.ekv26s.is_empty()
            || !self.ekv3s.is_empty()
            || !self.vdmoses.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || self
                .xspice_instances
                .iter()
                .any(|instance| instance.requires_conservative_newton_damping())
        {
            return true;
        }

        #[cfg(feature = "veriloga")]
        {
            if self.has_veriloga_devices() {
                return true;
            }
        }
        #[cfg(feature = "veriloga-builtins")]
        {
            if self.has_generated_veriloga_devices() {
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
        for bjt in &mut self.bjts.devices {
            bjt.set_junction_gmin(gmin);
        }
        for diode in &mut self.diodes.devices {
            diode.set_junction_gmin(gmin);
        }
        // BSIM3 consumes CKTgmin inside its diode equations (b3ld.c forms
        // `gbs = ... + gmin` directly); there is no separate shunt path.
        for dev in &mut self.bsim3v3.devices {
            dev.set_eval_gmin(gmin);
        }
        // BSIM4 follows the same discipline (b4ld.c diode/TAT terms).
        for dev in &mut self.bsim4v8.devices {
            dev.set_eval_gmin(gmin);
        }
        // Xyce BSIMSOI3 carries CKTgmin as two terminal conductances
        // (body-source and gate-drain). Its device option scales that GMIN by
        // 1e-6 by default, but decks can disable the scaling.
        let b3soi_gmin = gmin * self.b3soi_gmin_scale.max(0.0);
        for dev in &mut self.b3soi.devices {
            dev.set_eval_gmin(b3soi_gmin);
        }
        for dev in &mut self.b3soi_fd.devices {
            dev.set_eval_gmin(b3soi_gmin);
        }
        for dev in &mut self.b3soi_pd.devices {
            dev.set_eval_gmin(b3soi_gmin);
        }
        for dev in &mut self.ekv26s.devices {
            dev.set_eval_gmin(gmin);
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
            bsim3v3: self.bsim3v3.clone(),
            bsim4v8: self.bsim4v8.clone(),
            ekv26s: self.ekv26s.clone(),
            ekv3s: self.ekv3s.clone(),
            vdmoses: self.vdmoses.clone(),
            jfets: self.jfets.clone(),
            vswitches: self.vswitches.clone(),
            iswitches: self.iswitches.clone(),
            generic_switches: self.generic_switches.clone(),
            behavioral_sources: self.behavioral_sources.clone(),
            xspice_instances: self.xspice_instances.clone(),
            xspice_digital_values: self.xspice_digital_values.clone(),
            xspice_digital_drivers: self.xspice_digital_drivers.clone(),
            xspice_digital_event_times: self.xspice_digital_event_times.clone(),
            xspice_real_values: self.xspice_real_values.clone(),
            xspice_real_drivers: self.xspice_real_drivers.clone(),
            xspice_real_event_times: self.xspice_real_event_times.clone(),
            xspice_event_queue: self.xspice_event_queue.clone(),
            #[cfg(feature = "veriloga")]
            veriloga_devices: self.veriloga_devices.clone(),
            #[cfg(feature = "veriloga-builtins")]
            generated_veriloga_devices: self.generated_veriloga_devices.clone(),
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
        self.bsim3v3 = snapshot.bsim3v3;
        self.bsim4v8 = snapshot.bsim4v8;
        self.ekv26s = snapshot.ekv26s;
        self.ekv3s = snapshot.ekv3s;
        self.vdmoses = snapshot.vdmoses;
        self.jfets = snapshot.jfets;
        self.vswitches = snapshot.vswitches;
        self.iswitches = snapshot.iswitches;
        self.generic_switches = snapshot.generic_switches;
        self.behavioral_sources = snapshot.behavioral_sources;
        self.xspice_instances = snapshot.xspice_instances;
        self.xspice_digital_values = snapshot.xspice_digital_values;
        self.xspice_digital_drivers = snapshot.xspice_digital_drivers;
        self.xspice_digital_event_times = snapshot.xspice_digital_event_times;
        self.xspice_real_values = snapshot.xspice_real_values;
        self.xspice_real_drivers = snapshot.xspice_real_drivers;
        self.xspice_real_event_times = snapshot.xspice_real_event_times;
        self.xspice_event_queue = snapshot.xspice_event_queue;
        #[cfg(feature = "veriloga")]
        {
            self.veriloga_devices = snapshot.veriloga_devices;
        }
        #[cfg(feature = "veriloga-builtins")]
        {
            self.generated_veriloga_devices
                .restore_from_snapshot(snapshot.generated_veriloga_devices);
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
        self.bsim3v3.update_all(voltages);
        self.bsim4v8.update_all(voltages);
        self.ekv26s.update_all(voltages);
        self.ekv3s.update_all(voltages);
        self.vdmoses.update_all(voltages);
        let mut order: Vec<usize> = (0..self.jfets.len()).collect();
        order.sort_by_key(|&idx| (self.jfets[idx].model_order(), std::cmp::Reverse(idx)));
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
        order.sort_by_key(|&idx| (self.jfets[idx].model_order(), std::cmp::Reverse(idx)));
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

    /// Re-linearize BSIMSOI devices directly at a static probe solution.
    ///
    /// Normal nonlinear updates intentionally apply BSIMSOI branch/body
    /// limiters to protect Newton iterations. Residual and fallback validation
    /// probes must evaluate the compact-model equations at the candidate
    /// voltage itself so false limiter-history roots are rejected.
    pub(crate) fn update_b3soi_static_linearizations(&mut self, voltages: &[Value]) {
        for dev in &mut self.b3soi.devices {
            dev.update_static_linearization(voltages);
        }
        for dev in &mut self.b3soi_fd.devices {
            dev.update_static_linearization(voltages);
        }
        for dev in &mut self.b3soi_pd.devices {
            dev.update_static_linearization(voltages);
        }
    }

    /// Stamp all nonlinear devices into matrix using O(1) direct indexing
    pub fn stamp_nonlinear(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) {
        if let Err(err) = self.try_stamp_nonlinear(matrix, rhs, voltages) {
            panic!("{err}");
        }
    }

    /// Checked nonlinear stamping path for simulator analyses that can report
    /// external model diagnostics instead of unwinding.
    pub fn try_stamp_nonlinear(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) -> Result<(), String> {
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
        // BSIM3 stamps its equivalent currents through the stamper's RHS
        // hook (the b3ld.c cdreq/ceqbd/ceqbs rows), not the legacy `rhs`
        // slice, so the same path serves DC and transient assembly.
        self.bsim3v3.stamp_all(&mut stamper, &mut [], voltages);
        // BSIM4 rides the identical path (b4ld.c ceqdrn/ceqbd/ceqbs/ceqj*
        // rows through the stamper's RHS hook).
        self.bsim4v8.stamp_all(&mut stamper, &mut [], voltages);
        self.ekv26s.stamp_all(&mut stamper, &mut [], voltages);
        self.ekv3s.stamp_all(&mut stamper, &mut [], voltages);
        self.vdmoses.stamp_all(&mut stamper, &mut [], voltages);
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
            veriloga_devices.try_stamp_all(
                voltages,
                |row, col, value| matrix.add(row, col, value),
                |index, value| {
                    if let Some(slot) = rhs.get_mut(index) {
                        *slot += value;
                    }
                },
            )?;
        }
        Ok(())
    }

    /// Stamp expression-controlled generic switches for the given analysis time.
    pub fn stamp_generic_switches(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        time: Value,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for switch in &mut self.generic_switches {
            switch.stamp_time_dependent(time, &mut stamper);
        }
    }

    /// Stamp behavioral sources with the given analysis time.
    pub fn stamp_behavioral(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        _analysis: crate::xspice::AnalysisType,
    ) {
        self.behavioral_sources
            .stamp_all(matrix, rhs, solution, self.num_nodes, time);
        #[cfg(feature = "veriloga-builtins")]
        {
            let generated_analysis = match _analysis {
                crate::xspice::AnalysisType::Ac => {
                    crate::device::veriloga_generated::GeneratedAnalysisKind::Ac
                }
                crate::xspice::AnalysisType::Transient => {
                    crate::device::veriloga_generated::GeneratedAnalysisKind::Tran
                }
                crate::xspice::AnalysisType::Noise => {
                    crate::device::veriloga_generated::GeneratedAnalysisKind::Noise
                }
                _ => crate::device::veriloga_generated::GeneratedAnalysisKind::Dc,
            };
            let num_nodes = self.num_nodes;
            self.generated_veriloga_devices_mut().stamp_all(
                matrix,
                rhs,
                solution,
                num_nodes,
                generated_analysis,
            );
        }
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
            && self.bsim3v3.all_converged(criteria)
            && self.bsim4v8.all_converged(criteria)
            && self.ekv26s.all_converged(criteria)
            && self.ekv3s.all_converged(criteria)
            && self.vdmoses.all_converged(criteria)
            && self.jfets.iter().all(|jfet| jfet.is_converged(criteria))
            && self.vswitches.iter().all(|sw| sw.is_converged(criteria))
            && self.iswitches.iter().all(|sw| sw.is_converged(criteria))
            && self.xspice_converged(criteria.voltage_tolerance())
    }
}

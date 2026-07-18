use super::*;

#[derive(Debug, Clone)]
pub(crate) struct NonlinearDeviceStateSnapshot {
    inductors: Inductors,
    jiles_atherton_inductors: Vec<JilesAthertonBinding>,
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
    xyce_team_memristors: Vec<XyceTeamMemristorBinding>,
    vswitches: Vec<crate::device::VoltageSwitch>,
    iswitches: Vec<crate::device::CurrentSwitch>,
    generic_switches: Vec<crate::device::GenericSwitch>,
    behavioral_sources: BehavioralSources,
    xspice_instances: Vec<XspiceInstance>,
    xspice_digital_values: HashMap<NodeId, DigitalValue>,
    xspice_digital_drivers: XspiceDigitalDrivers,
    xspice_digital_event_times: HashMap<NodeId, Value>,
    xspice_real_values: HashMap<NodeId, Value>,
    xspice_real_drivers: XspiceRealDrivers,
    xspice_real_event_times: HashMap<NodeId, Value>,
    xspice_event_queue: EventQueue,
    #[cfg(feature = "veriloga")]
    veriloga_devices: crate::device::veriloga::VerilogADevices,
    #[cfg(feature = "veriloga-builtins")]
    generated_veriloga_devices: crate::device::veriloga_generated::BuiltinVerilogADevices,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DiodeStampMode {
    LimitedNewton,
    StaticProbe,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{BehavioralVoltageSource, EkvMosfet};

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

    #[test]
    fn behavioral_gmin_does_not_follow_junction_continuation() {
        let mut source = BehavioralVoltageSource::new("B1".to_string(), 1, 0, 1, "GMIN")
            .expect("GMIN behavioral expression parses");
        source.set_gmin(2.5e-8);

        let mut circuit = CircuitData::new();
        circuit.behavioral_sources.add_voltage(source);
        circuit.set_semiconductor_junction_gmin(1.0e-3);

        assert_eq!(
            circuit.behavioral_sources.voltage_sources[0].evaluate(&[], 0.0),
            2.5e-8,
            "expression-visible GMIN is the fixed resolved device option, not the active continuation conductance"
        );
    }

    #[cfg(feature = "veriloga-builtins")]
    #[test]
    fn generated_simparam_gmin_is_solver_controlled_and_not_rolled_back() {
        let mut circuit = CircuitData::new();
        circuit.set_semiconductor_junction_gmin(1.0e-6);
        let snapshot = circuit.nonlinear_state_snapshot();

        circuit.set_semiconductor_junction_gmin(0.0);
        circuit.restore_nonlinear_state(snapshot);

        assert_eq!(
            circuit.generated_simulation_parameters.get("gmin"),
            Some(0.0)
        );
    }

    #[test]
    fn nonlinear_snapshot_restores_dynamic_inductor_coefficients() {
        let mut circuit = CircuitData::new();
        circuit.inductors.add("lcore".to_string(), 1, 0, 1, 2.0e-3);
        let snapshot = circuit.nonlinear_state_snapshot();

        circuit.inductors.inductances[0] = 7.0e-3;
        circuit.restore_nonlinear_state(snapshot);

        assert_eq!(circuit.inductors.inductances, vec![2.0e-3]);
    }

    #[test]
    fn time_only_behavioral_source_does_not_make_circuit_nonlinear() {
        let mut circuit = CircuitData::new();
        circuit.behavioral_sources.add_voltage(
            BehavioralVoltageSource::new("B1".to_string(), 1, 0, 1, "time")
                .expect("time-only behavioral source parses"),
        );

        assert!(!circuit.has_nonlinear_devices());
    }

    #[test]
    fn solution_dependent_behavioral_source_makes_circuit_nonlinear() {
        let mut circuit = CircuitData::new();
        circuit.behavioral_sources.add_voltage(
            BehavioralVoltageSource::new("B1".to_string(), 1, 0, 1, "V(1)*V(1)")
                .expect("solution-dependent behavioral source parses"),
        );

        assert!(circuit.has_nonlinear_devices());
    }
}

impl CircuitData {
    fn has_non_xspice_nonlinear_devices(&self) -> bool {
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
            || !self.xyce_team_memristors.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || !self.generic_switches.is_empty()
            || self.behavioral_sources.has_solution_dependent_sources()
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

    pub(crate) fn can_use_zero_bias_for_explicit_xspice_ac(&self) -> bool {
        !self.xspice_instances.is_empty()
            && !self.has_non_xspice_nonlinear_devices()
            && self.xspice_instances.iter().all(|instance| {
                matches!(
                    instance.model_name(),
                    "tline" | "mlin" | "cpline" | "cpmlin"
                )
            })
    }

    /// Check if circuit has any nonlinear devices requiring Newton-Raphson
    pub fn has_nonlinear_devices(&self) -> bool {
        self.has_non_xspice_nonlinear_devices() || self.has_xspice_devices()
    }

    /// Whether all nonlinear-looking transient participants are XSPICE models
    /// whose stamps are affine in the current solution and stateless across
    /// trial iterations.
    pub(crate) fn has_only_memoryless_linear_xspice_nonlinearity(&self) -> bool {
        !self.xspice_instances.is_empty()
            && !self.has_non_xspice_nonlinear_devices()
            && self
                .xspice_instances
                .iter()
                .all(XspiceInstance::has_memoryless_linear_transient_stamp)
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

    /// Select whether native BSIMSOI devices are being stamped for an
    /// operating-point solve. Xyce-style instance IC branch constraints are
    /// active only in this mode; real transient timesteps keep the internal IC
    /// branch unknowns isolated with identity rows.
    pub(crate) fn set_b3soi_operating_point_mode(&mut self, operating_point: bool) {
        for dev in &self.b3soi.devices {
            dev.set_dc_mode(operating_point);
        }
        for dev in &self.b3soi_fd.devices {
            dev.set_dc_mode(operating_point);
        }
        for dev in &self.b3soi_pd.devices {
            dev.set_dc_mode(operating_point);
        }
    }

    /// Select whether a rank-deficient TEAM state equation may receive its
    /// deterministic operating-point gauge. This never replaces an active
    /// physical state equation and is disabled for real transient steps,
    /// where the private Q(x) companion supplies the dynamic row.
    pub(crate) fn set_xyce_team_operating_point_mode(&mut self, operating_point: bool) {
        self.xyce_team_operating_point_mode = operating_point;
    }

    pub(crate) fn reset_b3soi_operating_point_history(&mut self) {
        for dev in &mut self.b3soi.devices {
            dev.reset_operating_point_history();
        }
        for dev in &mut self.b3soi_pd.devices {
            dev.reset_operating_point_history();
        }
    }

    pub(crate) fn seed_b3soi_self_heating_temperature_guess(&self, solution: &mut [Value]) {
        for dev in &self.b3soi.devices {
            dev.seed_self_heating_temperature_from_power(solution);
        }
        for dev in &self.b3soi_pd.devices {
            dev.seed_self_heating_temperature_from_power(solution);
        }
    }

    pub(crate) fn zero_b3soi_self_heating_temperature_guess(&self, solution: &mut [Value]) {
        for node in self
            .b3soi
            .devices
            .iter()
            .filter(|dev| dev.has_self_heating_node())
            .map(|dev| dev.node_temp)
            .chain(
                self.b3soi_pd
                    .devices
                    .iter()
                    .filter(|dev| dev.has_self_heating_node())
                    .map(|dev| dev.node_temp),
            )
        {
            if let Some(slot) = solution.get_mut(node - 1) {
                *slot = 0.0;
            }
        }
    }

    pub(crate) fn prime_b3soi_operating_point_from_solution(&mut self, solution: &[Value]) {
        for dev in &mut self.b3soi.devices {
            dev.prime_operating_point_from_solution(solution);
        }
        for dev in &mut self.b3soi_pd.devices {
            dev.prime_operating_point_from_solution(solution);
        }
    }

    pub(crate) fn has_b3soi_self_heating(&self) -> bool {
        self.b3soi
            .devices
            .iter()
            .any(|dev| dev.has_self_heating_node())
            || self
                .b3soi_pd
                .devices
                .iter()
                .any(|dev| dev.has_self_heating_node())
    }

    pub(crate) fn set_b3soi_self_heating_startup_disabled(&self, disabled: bool) {
        for dev in &self.b3soi.devices {
            dev.set_self_heating_startup_disabled(disabled);
        }
        for dev in &self.b3soi_pd.devices {
            dev.set_self_heating_startup_disabled(disabled);
        }
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
            || !self.xyce_team_memristors.is_empty()
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
            || !self.xyce_team_memristors.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || self
                .xspice_instances
                .iter()
                .any(|instance| instance.requires_conservative_newton_damping())
        {
            return true;
        }

        // External Verilog-A can model ideal branch constraints whose exact
        // Newton step must not be rejected by global line-search damping.
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
        #[cfg(feature = "veriloga-builtins")]
        self.generated_simulation_parameters.set_gmin(gmin);
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
            inductors: self.inductors.clone(),
            jiles_atherton_inductors: self.jiles_atherton_inductors.clone(),
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
            xyce_team_memristors: self.xyce_team_memristors.clone(),
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
        self.inductors = snapshot.inductors;
        self.jiles_atherton_inductors = snapshot.jiles_atherton_inductors;
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
        self.xyce_team_memristors = snapshot.xyce_team_memristors;
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

    /// Commit nonlinear hysteresis state that must advance only after a
    /// transient timestep has been accepted.
    pub(crate) fn commit_accepted_nonlinear_state(&mut self) {
        for vswitch in &mut self.vswitches {
            vswitch.commit_transient_hysteresis();
        }
        for iswitch in &mut self.iswitches {
            iswitch.commit_transient_hysteresis();
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

    /// Re-linearize promoted native VBIC BJTs directly at a static probe
    /// solution. Regular Newton updates keep VBIC's junction limiter; residual
    /// validation probes need the true candidate equations.
    pub(crate) fn update_bjt_static_linearizations(&mut self, voltages: &[Value]) {
        for bjt in &mut self.bjts.devices {
            bjt.update_static_linearization(voltages);
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
    ) -> Result<(), String> {
        self.try_stamp_nonlinear(matrix, rhs, voltages)
    }

    /// Checked nonlinear stamping path for simulator analyses that can report
    /// external model diagnostics instead of unwinding.
    pub fn try_stamp_nonlinear(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) -> Result<(), String> {
        self.try_stamp_nonlinear_with_diode_mode(
            matrix,
            rhs,
            voltages,
            DiodeStampMode::LimitedNewton,
        )
    }

    /// Checked nonlinear stamping path for residual probes.
    ///
    /// Diodes are stamped at the candidate bias rather than through pnjlim,
    /// so residual validation measures the physical nonlinear equations. The
    /// live Newton path still uses limited diode companions.
    pub fn try_stamp_static_probe_nonlinear(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
    ) -> Result<(), String> {
        self.try_stamp_nonlinear_with_diode_mode(matrix, rhs, voltages, DiodeStampMode::StaticProbe)
    }

    fn try_stamp_nonlinear_with_diode_mode(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        voltages: &[Value],
        diode_stamp_mode: DiodeStampMode,
    ) -> Result<(), String> {
        use crate::device::NonlinearDevice;
        match diode_stamp_mode {
            DiodeStampMode::LimitedNewton => self.diodes.stamp_all_direct(matrix, rhs, voltages),
            DiodeStampMode::StaticProbe => {
                self.diodes
                    .stamp_static_probe_all_direct(matrix, rhs, voltages);
            }
        }
        self.bjts.stamp_all_direct(matrix, rhs, voltages);
        match diode_stamp_mode {
            DiodeStampMode::LimitedNewton => {
                self.mosfets.stamp_all_direct(matrix, rhs, voltages);
            }
            DiodeStampMode::StaticProbe => {
                self.mosfets
                    .stamp_all_static_probe_direct(matrix, rhs, voltages);
            }
        }
        for jfet in &self.jfets {
            jfet.stamp_direct(matrix, rhs, voltages);
        }
        self.stamp_xyce_team_memristors(matrix, rhs, voltages)?;
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
            let evaluation_mode = match diode_stamp_mode {
                DiodeStampMode::LimitedNewton => {
                    crate::device::veriloga::VerilogAEvaluationMode::NewtonLimited
                }
                DiodeStampMode::StaticProbe => {
                    crate::device::veriloga::VerilogAEvaluationMode::StaticProbe
                }
            };
            veriloga_devices.try_stamp_all_with_mode(
                voltages,
                |row, col, value| matrix.add(row, col, value),
                |index, value| {
                    if let Some(slot) = rhs.get_mut(index) {
                        *slot += value;
                    }
                },
                evaluation_mode,
            )?;
        }
        Ok(())
    }

    fn stamp_xyce_team_memristors(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
    ) -> Result<(), String> {
        #[inline]
        fn value_at(solution: &[Value], node: NodeId) -> Result<Value, String> {
            solution_node_voltage(solution, node)
                .ok_or_else(|| format!("TEAM memristor node {node} is outside the solution vector"))
        }

        #[inline]
        fn stamp_row(
            matrix: &mut StaticMatrix,
            rhs: &mut [Value],
            row: NodeId,
            columns: &[(NodeId, Value)],
            equivalent_rhs: Value,
        ) {
            if row == 0 {
                return;
            }
            for &(column, derivative) in columns {
                if column > 0 && derivative != 0.0 {
                    matrix.add(row - 1, column - 1, derivative);
                }
            }
            rhs[row - 1] += equivalent_rhs;
        }

        for binding in &self.xyce_team_memristors {
            let v_pos = value_at(solution, binding.node_pos)?;
            let v_neg = value_at(solution, binding.node_neg)?;
            let x = value_at(solution, binding.node_x)?;
            let cache = binding
                .device
                .evaluate(v_pos, v_neg, x)
                .map_err(|error| format!("TEAM memristor '{}': {error}", binding.name))?;
            let variables = [v_pos, v_neg, x];
            let nodes = [binding.node_pos, binding.node_neg, binding.node_x];

            for row_index in 0..2 {
                let row_jacobian = cache.jacobian[row_index];
                let equivalent_rhs = row_jacobian
                    .iter()
                    .zip(variables)
                    .map(|(derivative, value)| derivative * value)
                    .sum::<Value>()
                    - cache.residual[row_index];
                let columns = [
                    (nodes[0], row_jacobian[0]),
                    (nodes[1], row_jacobian[1]),
                    (nodes[2], row_jacobian[2]),
                ];
                stamp_row(matrix, rhs, nodes[row_index], &columns, equivalent_rhs);
            }

            let mut row_jacobian = cache.jacobian[2];
            let mut state_residual = cache.residual[2];
            if self.xyce_team_operating_point_mode
                && state_residual == 0.0
                && row_jacobian.iter().all(|derivative| *derivative == 0.0)
            {
                // Within the threshold deadband, F_x is identically zero and
                // a DC operating point does not determine x. Select x=0 as a
                // gauge only for that rank-deficient row. Transient stamping
                // disables this mode so dQ/dt remains the governing equation.
                row_jacobian = [0.0, 0.0, 1.0];
                state_residual = x;
            }
            let equivalent_rhs = row_jacobian
                .iter()
                .zip(variables)
                .map(|(derivative, value)| derivative * value)
                .sum::<Value>()
                - state_residual;
            let columns = [
                (nodes[0], row_jacobian[0]),
                (nodes[1], row_jacobian[1]),
                (nodes[2], row_jacobian[2]),
            ];
            stamp_row(matrix, rhs, binding.node_x, &columns, equivalent_rhs);
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
    pub fn stamp_behavioral_sources(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) {
        self.behavioral_sources
            .stamp_all(matrix, rhs, solution, self.num_nodes, time);
    }

    /// Stamp behavioral sources and generated Verilog-A builtins with the
    /// given analysis time.
    pub fn stamp_behavioral(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
        self.stamp_behavioral_with_generated_mode(
            matrix,
            rhs,
            solution,
            time,
            analysis,
            crate::device::veriloga_generated::GeneratedEvaluationMode::NewtonLimited,
        );
    }

    /// Stamp behavioral devices for a physical residual probe. Generated
    /// Verilog-A limiters are bypassed and their Newton history is untouched.
    pub fn stamp_behavioral_static_probe(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
        self.stamp_behavioral_with_generated_mode(
            matrix,
            rhs,
            solution,
            time,
            analysis,
            crate::device::veriloga_generated::GeneratedEvaluationMode::StaticProbe,
        );
    }

    pub(crate) fn stamp_behavioral_with_generated_mode(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        _analysis: crate::xspice::AnalysisType,
        _evaluation_mode: crate::device::veriloga_generated::GeneratedEvaluationMode,
    ) {
        self.stamp_behavioral_sources(matrix, rhs, solution, time);
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
            let simparams = self.generated_simulation_parameters;
            self.generated_veriloga_devices_mut().stamp_all_with_mode(
                matrix,
                rhs,
                solution,
                num_nodes,
                generated_analysis,
                simparams,
                _evaluation_mode,
            );
        }
    }

    /// Check if all nonlinear devices have converged
    pub fn nonlinear_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        #[cfg(feature = "veriloga-builtins")]
        let generated_veriloga_converged = self.generated_veriloga_devices.all_converged();
        #[cfg(not(feature = "veriloga-builtins"))]
        let generated_veriloga_converged = true;
        #[cfg(feature = "veriloga")]
        let dynamic_veriloga_converged = self.veriloga_devices.all_converged();
        #[cfg(not(feature = "veriloga"))]
        let dynamic_veriloga_converged = true;
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
            && dynamic_veriloga_converged
            && generated_veriloga_converged
    }

    pub fn behavioral_linearizations_converged(
        &mut self,
        solution: &[Value],
        time: Value,
        reltol: Value,
        voltage_abstol: Value,
        current_abstol: Value,
    ) -> bool {
        self.behavioral_sources.linearizations_converged(
            solution,
            time,
            reltol,
            voltage_abstol,
            current_abstol,
        )
    }
}

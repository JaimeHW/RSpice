//! Bias-dependent device state across the Newton and timestep loops.
//!
//! Two jobs. First, per-family capability queries (`has_*_devices`,
//! self-heating and event-driven presence) that let the engine skip work for
//! circuits that do not contain a given device class. Second, the state
//! lifecycle: snapshotting nonlinear device state so a rejected Newton
//! iteration or timestep can be rolled back, and committing it once a step is
//! accepted. Solver-controlled quantities such as GMIN continuation are
//! deliberately excluded from rollback.

use super::*;

#[derive(Debug, Clone)]
pub(crate) struct NonlinearDeviceStateSnapshot {
    // Fixed-value reactive devices are immutable during Newton/residual
    // probes.  Keeping them out of the rollback image avoids copying their
    // topology and accepted history once per attempted timestep.  The full
    // containers remain necessary for solution-dependent capacitances and
    // nonlinear magnetic devices because their evaluators/linearizations are
    // mutated while a trial point is assembled.
    capacitors: Option<Capacitors>,
    inductors: Option<Inductors>,
    jiles_atherton_inductors: Vec<JilesAthertonBinding>,
    xyce_core_groups: Vec<XyceCoreGroupBinding>,
    diodes: Vec<crate::device::semiconductor::DiodeNonlinearState>,
    bjts: Bjts,
    mosfets: Vec<crate::device::mosfet::MosfetNonlinearState>,
    b3soi: B3SoiDds,
    b3soi_fd: B3SoiFds,
    b3soi_pd: B3SoiPds,
    bsim3v3: Bsim3v3s,
    bsim4v8: Bsim4v8s,
    ekv26s: EkvMosfets,
    ekv3s: Ekv3Mosfets,
    vdmoses: Vdmoses,
    jfets: Vec<crate::device::Jfet>,
    xyce_memristors: Vec<XyceMemristorBinding>,
    xyce_memristor_operating_point_mode: bool,
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
    xspice_event_queue: XspiceEventScheduler,
    #[cfg(feature = "veriloga")]
    veriloga_devices: crate::device::veriloga::VerilogADevices,
    #[cfg(feature = "veriloga-builtins-base")]
    generated_veriloga_devices: crate::device::veriloga_builtins::BuiltinVerilogADevicesRollback,
}

/// Accepted native diode/BJT state plus any runtime families that deliberately
/// block resume. Capture is infallible so checkpoint producers can persist a
/// diagnostic image even when a circuit contains an unsupported VBIC runtime;
/// validation/restore fail closed on a non-empty blocker list.
#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct AcceptedNativeNonlinearCheckpointStates {
    pub(crate) diodes: Vec<crate::device::semiconductor::AcceptedDiodeNonlinearCheckpoint>,
    pub(crate) bjts: Vec<crate::device::semiconductor::AcceptedBjtNonlinearCheckpoint>,
    pub(crate) resume_blockers: Vec<String>,
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
            circuit.behavioral_sources.voltage_sources[0]
                .evaluate(&[], 0.0)
                .expect("finite GMIN source"),
            2.5e-8,
            "expression-visible GMIN is the fixed resolved device option, not the active continuation conductance"
        );
    }

    /// **D5 clause 1**, the half `tests/sync_contract.rs` cannot reach.
    ///
    /// Conservative lockstep says the digital world executes only at analog
    /// timepoints the integrator accepts. Nothing in the event path enforces
    /// that directly — the settle loop drains at whatever bound it is handed,
    /// including a Newton trial's candidate time. What makes the rejected
    /// trial harmless is that the scheduler rides in this snapshot, so a
    /// discarded attempt's event execution is undone with the rest of the
    /// device state.
    ///
    /// That is a one-line entry in a struct with forty fields, and a
    /// refactor that dropped it would leave every existing test passing while
    /// silently breaking D5: a rejected step's events would stay executed.
    /// This is the assertion that fails instead.
    ///
    /// Both snapshot flavours are checked. `transient_trial_state_snapshot` is
    /// the one the transient loop actually takes per attempt; it omits the
    /// fixed reactive stores, and the event queue must *not* be omitted with
    /// them.
    #[test]
    fn xspice_event_queue_survives_the_nonlinear_state_round_trip() {
        fn queue_two_events(circuit: &mut CircuitData) {
            circuit.xspice_event_queue.schedule(
                2.0e-9,
                1,
                "out",
                "u1",
                0,
                crate::xspice::EventValue::Digital(DigitalValue::one()),
            );
            circuit.xspice_event_queue.schedule(
                4.0e-9,
                2,
                "out",
                "u2",
                0,
                crate::xspice::EventValue::Digital(DigitalValue::zero()),
            );
        }

        for (label, take_snapshot) in [
            (
                "nonlinear_state_snapshot",
                CircuitData::nonlinear_state_snapshot
                    as fn(&CircuitData) -> NonlinearDeviceStateSnapshot,
            ),
            (
                "transient_trial_state_snapshot",
                CircuitData::transient_trial_state_snapshot
                    as fn(&CircuitData) -> NonlinearDeviceStateSnapshot,
            ),
        ] {
            let mut circuit = CircuitData::new();
            queue_two_events(&mut circuit);

            let accepted = take_snapshot(&circuit);

            // The rejected attempt executes both events.
            circuit
                .xspice_event_queue
                .run_due_events(4.0e-9, |_| {})
                .expect("a queue nothing feeds back into settles");
            assert!(
                circuit.xspice_event_queue.is_empty(),
                "{label}: the attempt must actually consume the queue, or this proves nothing"
            );

            circuit.restore_nonlinear_state(accepted);

            assert_eq!(
                circuit.xspice_event_queue.len(),
                2,
                "{label}: a rejected step must leave every event pending again"
            );
            assert_eq!(
                circuit.xspice_event_queue.next_event_time(),
                Some(2.0e-9),
                "{label}: the restored queue must present the same next event time, \
                 which is what the retry's breakpoint is placed from"
            );
        }
    }

    #[cfg(feature = "veriloga-builtins-base")]
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
    fn nonlinear_snapshot_restores_xyce_memristor_equation_mode() {
        let mut circuit = CircuitData::new();
        circuit.set_xyce_memristor_operating_point_mode(false);
        let snapshot = circuit.nonlinear_state_snapshot();

        circuit.set_xyce_memristor_operating_point_mode(true);
        circuit.restore_nonlinear_state(snapshot);

        assert!(!circuit.xyce_memristor_operating_point_mode);
    }

    #[test]
    fn nonlinear_snapshot_restores_compact_classic_mos_state() {
        let mut circuit = CircuitData::new();
        circuit.mosfets.add(crate::device::Mosfet::new_nmos(
            "m1".to_string(),
            1,
            2,
            0,
            0,
        ));
        let expected = circuit.mosfets.nonlinear_state_snapshot();
        let snapshot = circuit.nonlinear_state_snapshot();

        circuit.mosfets.devices[0].set_junction_gmin(3.0e-6);
        circuit.mosfets.update_all(&[1.0, 2.0]);
        assert_ne!(circuit.mosfets.nonlinear_state_snapshot(), expected);

        circuit.restore_nonlinear_state(snapshot);

        assert_eq!(circuit.mosfets.nonlinear_state_snapshot(), expected);
        assert_eq!(circuit.mosfets.devices[0].name, "m1");
    }

    #[test]
    fn classic_mos_only_transient_capability_is_fail_closed() {
        let mut circuit = CircuitData::new();
        assert!(!circuit.has_classic_mos_only_transient_nonlinearity());

        circuit.mosfets.add(crate::device::Mosfet::new_nmos(
            "m1".to_string(),
            1,
            2,
            0,
            0,
        ));
        assert!(circuit.has_classic_mos_only_transient_nonlinearity());
        assert!(circuit.has_cacheable_classic_mos_transient_base());

        circuit
            .diodes
            .add(crate::device::Diode::new("d1".to_string(), 1, 0));
        assert!(!circuit.has_classic_mos_only_transient_nonlinearity());
        assert!(!circuit.has_cacheable_classic_mos_transient_base());

        let mut with_inductor = CircuitData::new();
        with_inductor.mosfets.add(crate::device::Mosfet::new_nmos(
            "m1".to_string(),
            1,
            2,
            0,
            0,
        ));
        with_inductor
            .inductors
            .add("l1".to_string(), 1, 0, 1, 1.0e-3);
        assert!(with_inductor.has_classic_mos_only_transient_nonlinearity());
        assert!(!with_inductor.has_cacheable_classic_mos_transient_base());
    }

    #[test]
    fn diode_only_transient_base_capability_is_fail_closed() {
        let mut circuit = CircuitData::new();
        assert!(!circuit.has_cacheable_diode_transient_base());

        circuit
            .diodes
            .add(crate::device::Diode::new("d1".to_string(), 1, 0));
        assert!(circuit.has_cacheable_diode_transient_base());

        circuit.mosfets.add(crate::device::Mosfet::new_nmos(
            "m1".to_string(),
            1,
            2,
            0,
            0,
        ));
        assert!(!circuit.has_cacheable_diode_transient_base());

        let mut with_inductor = CircuitData::new();
        with_inductor
            .diodes
            .add(crate::device::Diode::new("d1".to_string(), 1, 0));
        with_inductor
            .inductors
            .add("l1".to_string(), 1, 0, 1, 1.0e-3);
        assert!(!with_inductor.has_cacheable_diode_transient_base());
    }

    #[test]
    fn nonlinear_snapshot_restores_compact_diode_state() {
        let mut circuit = CircuitData::new();
        circuit
            .diodes
            .add(crate::device::Diode::new("d1".to_string(), 1, 0));
        let expected = circuit.diodes.nonlinear_state_snapshot();
        let snapshot = circuit.nonlinear_state_snapshot();

        circuit.diodes.devices[0].set_junction_gmin(4.0e-6);
        circuit.diodes.update_all(&[0.7]);
        assert_ne!(circuit.diodes.nonlinear_state_snapshot(), expected);

        circuit.restore_nonlinear_state(snapshot);

        assert_eq!(circuit.diodes.nonlinear_state_snapshot(), expected);
        assert_eq!(circuit.diodes.devices[0].name, "d1");
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
    /// Capture every supported accepted native nonlinear runtime without
    /// changing the infallible checkpoint-capture contract. Unsupported or
    /// invalid device state is represented by an explicit, named blocker.
    pub(crate) fn capture_accepted_native_nonlinear_checkpoint_states(
        &self,
    ) -> AcceptedNativeNonlinearCheckpointStates {
        let mut captured = AcceptedNativeNonlinearCheckpointStates {
            diodes: Vec::with_capacity(self.diodes.devices.len()),
            bjts: Vec::with_capacity(self.bjts.devices.len()),
            resume_blockers: Vec::new(),
        };
        for diode in &self.diodes.devices {
            match diode.accepted_nonlinear_checkpoint() {
                Ok(state) => captured.diodes.push(state),
                Err(blocker) => captured.resume_blockers.push(blocker),
            }
        }
        for bjt in &self.bjts.devices {
            match bjt.accepted_nonlinear_checkpoint() {
                Ok(state) => captured.bjts.push(state),
                Err(blocker) => captured.resume_blockers.push(blocker),
            }
        }
        captured
    }

    /// Validate all names, runtime tags, fixed payload shapes and finite values
    /// before any live device is mutated.
    pub(crate) fn validate_accepted_native_nonlinear_checkpoint_states(
        &self,
        captured: &AcceptedNativeNonlinearCheckpointStates,
    ) -> Result<(), String> {
        if !captured.resume_blockers.is_empty() {
            return Err(format!(
                "transient checkpoint does not contain restorable native nonlinear state: {}",
                captured.resume_blockers.join("; ")
            ));
        }
        if captured.diodes.len() != self.diodes.devices.len() {
            return Err(format!(
                "checkpoint diode accepted nonlinear state shape mismatch: captured {}, circuit has {}",
                captured.diodes.len(),
                self.diodes.devices.len()
            ));
        }
        if captured.bjts.len() != self.bjts.devices.len() {
            return Err(format!(
                "checkpoint BJT accepted nonlinear state shape mismatch: captured {}, circuit has {}",
                captured.bjts.len(),
                self.bjts.devices.len()
            ));
        }
        for (index, (diode, state)) in self.diodes.devices.iter().zip(&captured.diodes).enumerate()
        {
            diode
                .validate_accepted_nonlinear_checkpoint(state)
                .map_err(|error| format!("checkpoint diode instance {index}: {error}"))?;
        }
        for (index, (bjt, state)) in self.bjts.devices.iter().zip(&captured.bjts).enumerate() {
            bjt.validate_accepted_nonlinear_checkpoint(state)
                .map_err(|error| format!("checkpoint BJT instance {index}: {error}"))?;
        }
        Ok(())
    }

    /// Restore compact accepted state into the current elaboration. Validation
    /// is deliberately completed for every device before the first write.
    pub(crate) fn restore_accepted_native_nonlinear_checkpoint_states(
        &mut self,
        captured: &AcceptedNativeNonlinearCheckpointStates,
    ) -> Result<(), String> {
        self.validate_accepted_native_nonlinear_checkpoint_states(captured)?;
        for (diode, state) in self.diodes.devices.iter_mut().zip(&captured.diodes) {
            diode.restore_accepted_nonlinear_checkpoint(state)?;
        }
        for (bjt, state) in self.bjts.devices.iter_mut().zip(&captured.bjts) {
            bjt.restore_accepted_nonlinear_checkpoint(state)?;
        }
        Ok(())
    }

    /// Whether transient nonlinear trial state consists solely of classic
    /// MOS devices. Their physical residual stamp and charge companions are
    /// pure functions of the candidate solution, so a proof restamp can
    /// reuse the candidate update instead of evaluating the family twice.
    pub(crate) fn has_classic_mos_only_transient_nonlinearity(&self) -> bool {
        #[cfg(feature = "veriloga-builtins-base")]
        let has_generated_veriloga = self.has_generated_veriloga_devices();
        #[cfg(not(feature = "veriloga-builtins-base"))]
        let has_generated_veriloga = false;
        #[cfg(feature = "veriloga")]
        let has_dynamic_veriloga = self.has_veriloga_devices();
        #[cfg(not(feature = "veriloga"))]
        let has_dynamic_veriloga = false;

        !self.mosfets.is_empty()
            && !self.capacitors.has_solution_dependent_values()
            && self.diodes.is_empty()
            && self.bjts.is_empty()
            && self.b3soi.is_empty()
            && self.b3soi_fd.is_empty()
            && self.b3soi_pd.is_empty()
            && self.bsim3v3.is_empty()
            && self.bsim4v8.is_empty()
            && self.ekv26s.is_empty()
            && self.ekv3s.is_empty()
            && self.vdmoses.is_empty()
            && self.jfets.is_empty()
            && self.xyce_memristors.is_empty()
            && self.vswitches.is_empty()
            && self.iswitches.is_empty()
            && self.generic_switches.is_empty()
            && self.jiles_atherton_inductors.is_empty()
            && self.xyce_core_groups.is_empty()
            && self.behavioral_sources.is_empty()
            && !self.has_xspice_devices()
            && !has_generated_veriloga
            && !has_dynamic_veriloga
    }

    /// Whether a transient attempt can reuse one solution-independent base
    /// matrix for every classic-MOS Newton and residual assembly.
    ///
    /// This deliberately fails closed around every stateful linear companion
    /// other than an ordinary capacitor. The excluded families are legal in a
    /// classic-MOS-only nonlinear circuit, but some refresh trial state from
    /// the candidate solution or carry analysis-specific stamping rules.
    pub(crate) fn has_cacheable_classic_mos_transient_base(&self) -> bool {
        self.has_classic_mos_only_transient_nonlinearity()
            && self.resistors.thermal.iter().all(Option::is_none)
            && self.inductors.is_empty()
            && self.tlines.is_empty()
            && self.coupled_tlines.is_empty()
            && self.couplings.is_empty()
            && self.coupled_inductor_pairs.is_empty()
            && self.multi_winding_transformers.is_empty()
    }

    /// Whether a diode/ordinary-RC transient can restore its invariant and
    /// per-attempt linear base instead of rebuilding it for every Newton and
    /// physical-residual stamp.
    ///
    /// The capability is intentionally fail-closed around every other
    /// nonlinear, behavioral, event-driven, or stateful linear family.
    pub(crate) fn has_cacheable_diode_transient_base(&self) -> bool {
        #[cfg(feature = "veriloga-builtins-base")]
        let has_generated_veriloga = self.has_generated_veriloga_devices();
        #[cfg(not(feature = "veriloga-builtins-base"))]
        let has_generated_veriloga = false;
        #[cfg(feature = "veriloga")]
        let has_dynamic_veriloga = self.has_veriloga_devices();
        #[cfg(not(feature = "veriloga"))]
        let has_dynamic_veriloga = false;

        !self.diodes.is_empty()
            && !self.capacitors.has_solution_dependent_values()
            && self.mosfets.is_empty()
            && self.bjts.is_empty()
            && self.b3soi.is_empty()
            && self.b3soi_fd.is_empty()
            && self.b3soi_pd.is_empty()
            && self.bsim3v3.is_empty()
            && self.bsim4v8.is_empty()
            && self.ekv26s.is_empty()
            && self.ekv3s.is_empty()
            && self.vdmoses.is_empty()
            && self.jfets.is_empty()
            && self.xyce_memristors.is_empty()
            && self.vswitches.is_empty()
            && self.iswitches.is_empty()
            && self.generic_switches.is_empty()
            && self.jiles_atherton_inductors.is_empty()
            && self.xyce_core_groups.is_empty()
            && self.behavioral_sources.is_empty()
            && self.resistors.thermal.iter().all(Option::is_none)
            && self.inductors.is_empty()
            && self.tlines.is_empty()
            && self.coupled_tlines.is_empty()
            && self.couplings.is_empty()
            && self.coupled_inductor_pairs.is_empty()
            && self.multi_winding_transformers.is_empty()
            && !self.has_xspice_devices()
            && !has_generated_veriloga
            && !has_dynamic_veriloga
    }

    fn has_non_xspice_nonlinear_devices(&self) -> bool {
        self.capacitors.has_solution_dependent_values()
            || !self.diodes.is_empty()
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
            || !self.xyce_memristors.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || !self.generic_switches.is_empty()
            || self
                .jiles_atherton_inductors
                .iter()
                .any(|binding| binding.device.is_xyce_core())
            || self
                .xyce_core_groups
                .iter()
                .any(|binding| binding.device.is_xyce_core())
            || self.behavioral_sources.has_solution_dependent_sources()
            || {
                #[cfg(feature = "veriloga-builtins-base")]
                {
                    self.has_generated_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga-builtins-base"))]
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

    /// Select the family-specific operating-point state equation. This is
    /// disabled for real transient steps, where the private Q(x) companion
    /// supplies the dynamic row.
    pub(crate) fn set_xyce_memristor_operating_point_mode(&mut self, operating_point: bool) {
        self.xyce_memristor_operating_point_mode = operating_point;
    }

    pub(crate) fn reset_b3soi_operating_point_history(&mut self) {
        for dev in &mut self.b3soi.devices {
            dev.reset_operating_point_history();
        }
        for dev in &mut self.b3soi_pd.devices {
            dev.reset_operating_point_history();
        }
    }

    pub(crate) fn reset_legacy_bjt_operating_point_history(&mut self) {
        for dev in &mut self.bjts.devices {
            dev.reset_legacy_operating_point_history();
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

    /// Hand every native BSIMSOI instance the engine's bypass tolerance triple,
    /// or `None` to keep bypass off.
    ///
    /// Circuit construction is the only point at which a resolved
    /// `SimulationConfig` and the device instances it governs are both in hand;
    /// past that, an analysis receives a built circuit and never sees the
    /// config that built it. The devices decide per iterate whether the freeze
    /// applies, so the engine states the policy once and never again.
    pub(crate) fn set_b3soi_bypass_tolerances(&self, tolerances: Option<(Value, Value, Value)>) {
        for dev in &self.b3soi.devices {
            dev.set_bypass_tolerances(tolerances);
        }
        for dev in &self.b3soi_fd.devices {
            dev.set_bypass_tolerances(tolerances);
        }
        for dev in &self.b3soi_pd.devices {
            dev.set_bypass_tolerances(tolerances);
        }
    }

    /// Newton iterates the native BSIMSOI instances answered from a frozen
    /// linearization, summed over the circuit.
    pub fn b3soi_bypass_hits(&self) -> u64 {
        self.b3soi
            .devices
            .iter()
            .map(|dev| dev.bypass_hits())
            .chain(self.b3soi_fd.devices.iter().map(|dev| dev.bypass_hits()))
            .chain(self.b3soi_pd.devices.iter().map(|dev| dev.bypass_hits()))
            .sum()
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
            || !self.xyce_memristors.is_empty()
            || !self.vswitches.is_empty()
            || !self.iswitches.is_empty()
            || self
                .jiles_atherton_inductors
                .iter()
                .any(|binding| binding.device.is_xyce_core())
            || self
                .xyce_core_groups
                .iter()
                .any(|binding| binding.device.is_xyce_core())
            || self
                .xspice_instances
                .iter()
                .any(|instance| instance.requires_conservative_newton_damping())
            || {
                #[cfg(feature = "veriloga-builtins-base")]
                {
                    self.has_generated_veriloga_devices()
                }
                #[cfg(not(feature = "veriloga-builtins-base"))]
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
    /// Newton path. Voltage/current switches are likewise bounded
    /// conductances: globally clipping their ideal-source node updates hides
    /// the authored initial-junction state behind artificial 0.5 V steps.
    /// Keep damping enabled for compact models and mixed nonlinear circuits
    /// where device-local limiting is not sufficient.
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
            || !self.xyce_memristors.is_empty()
            || self
                .xspice_instances
                .iter()
                .any(|instance| instance.requires_conservative_newton_damping())
        {
            return true;
        }

        // External Verilog-A can model ideal branch constraints whose exact
        // Newton step must not be rejected by global line-search damping.
        #[cfg(feature = "veriloga-builtins-base")]
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
        #[cfg(feature = "veriloga-builtins-base")]
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
        self.nonlinear_state_snapshot_impl(true)
    }

    fn nonlinear_state_snapshot_impl(
        &self,
        include_fixed_reactive_stores: bool,
    ) -> NonlinearDeviceStateSnapshot {
        NonlinearDeviceStateSnapshot {
            capacitors: (include_fixed_reactive_stores
                || self.capacitors.has_solution_dependent_values())
            .then(|| self.capacitors.clone()),
            inductors: (include_fixed_reactive_stores
                || !self.jiles_atherton_inductors.is_empty()
                || !self.xyce_core_groups.is_empty())
            .then(|| self.inductors.clone()),
            jiles_atherton_inductors: self.jiles_atherton_inductors.clone(),
            xyce_core_groups: self.xyce_core_groups.clone(),
            diodes: self.diodes.nonlinear_state_snapshot(),
            bjts: self.bjts.clone(),
            mosfets: self.mosfets.nonlinear_state_snapshot(),
            b3soi: self.b3soi.clone(),
            b3soi_fd: self.b3soi_fd.clone(),
            b3soi_pd: self.b3soi_pd.clone(),
            bsim3v3: self.bsim3v3.clone(),
            bsim4v8: self.bsim4v8.clone(),
            ekv26s: self.ekv26s.clone(),
            ekv3s: self.ekv3s.clone(),
            vdmoses: self.vdmoses.clone(),
            jfets: self.jfets.clone(),
            xyce_memristors: self.xyce_memristors.clone(),
            xyce_memristor_operating_point_mode: self.xyce_memristor_operating_point_mode,
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
            #[cfg(feature = "veriloga-builtins-base")]
            generated_veriloga_devices: self.generated_veriloga_devices.capture_rollback_state(),
        }
    }

    /// Capture the mutable state touched while assembling a transient trial.
    ///
    /// Fixed-value capacitors and ordinary inductors retain accepted history
    /// until the step-commit phase, so copying their topology-sized stores at
    /// every attempted timestep is unnecessary.  Solution-dependent
    /// capacitor evaluators and nonlinear magnetic devices do mutate during
    /// trial assembly and therefore retain the exact full rollback behavior.
    pub(crate) fn transient_trial_state_snapshot(&self) -> NonlinearDeviceStateSnapshot {
        self.nonlinear_state_snapshot_impl(false)
    }

    /// Refresh an existing rollback snapshot without reallocating its
    /// topology-sized buffers.
    ///
    /// Transient integration takes these snapshots at every attempted step
    /// and merit checkpoint. Circuit topology is immutable during an
    /// analysis, so `clone_from` and the compact state-vector writers retain
    /// all backing allocations after the first capture.
    fn refresh_nonlinear_state_snapshot_impl(
        &self,
        snapshot: &mut NonlinearDeviceStateSnapshot,
        include_fixed_reactive_stores: bool,
    ) {
        if include_fixed_reactive_stores || self.capacitors.has_solution_dependent_values() {
            if let Some(capacitors) = snapshot.capacitors.as_mut() {
                capacitors.clone_from(&self.capacitors);
            } else {
                snapshot.capacitors = Some(self.capacitors.clone());
            }
        } else {
            snapshot.capacitors = None;
        }
        if include_fixed_reactive_stores
            || !self.jiles_atherton_inductors.is_empty()
            || !self.xyce_core_groups.is_empty()
        {
            if let Some(inductors) = snapshot.inductors.as_mut() {
                inductors.clone_from(&self.inductors);
            } else {
                snapshot.inductors = Some(self.inductors.clone());
            }
        } else {
            snapshot.inductors = None;
        }
        snapshot
            .jiles_atherton_inductors
            .clone_from(&self.jiles_atherton_inductors);
        snapshot.xyce_core_groups.clone_from(&self.xyce_core_groups);
        self.diodes
            .nonlinear_state_snapshot_into(&mut snapshot.diodes);
        snapshot.bjts.clone_from(&self.bjts);
        self.mosfets
            .nonlinear_state_snapshot_into(&mut snapshot.mosfets);
        snapshot.b3soi.clone_from(&self.b3soi);
        snapshot.b3soi_fd.clone_from(&self.b3soi_fd);
        snapshot.b3soi_pd.clone_from(&self.b3soi_pd);
        snapshot.bsim3v3.clone_from(&self.bsim3v3);
        snapshot.bsim4v8.clone_from(&self.bsim4v8);
        snapshot.ekv26s.clone_from(&self.ekv26s);
        snapshot.ekv3s.clone_from(&self.ekv3s);
        snapshot.vdmoses.clone_from(&self.vdmoses);
        snapshot.jfets.clone_from(&self.jfets);
        snapshot.xyce_memristors.clone_from(&self.xyce_memristors);
        snapshot.xyce_memristor_operating_point_mode = self.xyce_memristor_operating_point_mode;
        snapshot.vswitches.clone_from(&self.vswitches);
        snapshot.iswitches.clone_from(&self.iswitches);
        snapshot.generic_switches.clone_from(&self.generic_switches);
        snapshot
            .behavioral_sources
            .clone_from(&self.behavioral_sources);
        snapshot.xspice_instances.clone_from(&self.xspice_instances);
        snapshot
            .xspice_digital_values
            .clone_from(&self.xspice_digital_values);
        snapshot
            .xspice_digital_drivers
            .clone_from(&self.xspice_digital_drivers);
        snapshot
            .xspice_digital_event_times
            .clone_from(&self.xspice_digital_event_times);
        snapshot
            .xspice_real_values
            .clone_from(&self.xspice_real_values);
        snapshot
            .xspice_real_drivers
            .clone_from(&self.xspice_real_drivers);
        snapshot
            .xspice_real_event_times
            .clone_from(&self.xspice_real_event_times);
        snapshot
            .xspice_event_queue
            .clone_from(&self.xspice_event_queue);
        #[cfg(feature = "veriloga")]
        snapshot.veriloga_devices.clone_from(&self.veriloga_devices);
        #[cfg(feature = "veriloga-builtins-base")]
        self.generated_veriloga_devices
            .capture_rollback_state_into(&mut snapshot.generated_veriloga_devices);
    }

    /// Refresh a reusable transient-trial snapshot while preserving the
    /// elision of immutable fixed-value reactive-device stores.
    pub(crate) fn refresh_transient_trial_state_snapshot(
        &self,
        snapshot: &mut NonlinearDeviceStateSnapshot,
    ) {
        self.refresh_nonlinear_state_snapshot_impl(snapshot, false);
    }

    /// Restore mutable nonlinear evaluation state after a trial residual probe.
    pub(crate) fn restore_nonlinear_state(&mut self, snapshot: NonlinearDeviceStateSnapshot) {
        self.restore_nonlinear_state_with_xyce_core_carry(snapshot, false);
    }

    /// Restore a rejected adaptive timestep while retaining Xyce LEVEL=2's
    /// latest `MagVarUpdate` evaluation member. Xyce does not roll that
    /// member back on timestep rejection, although merit/line-search probes
    /// do require the ordinary strict rollback above.
    pub(crate) fn restore_nonlinear_state_preserving_xyce_core_level2_carry(
        &mut self,
        snapshot: NonlinearDeviceStateSnapshot,
    ) {
        self.restore_nonlinear_state_with_xyce_core_carry(snapshot, true);
    }

    fn restore_nonlinear_state_with_xyce_core_carry(
        &mut self,
        snapshot: NonlinearDeviceStateSnapshot,
        preserve_xyce_core_carry: bool,
    ) {
        let vswitch_initial_loads = self
            .vswitches
            .iter()
            .map(crate::device::VoltageSwitch::initial_load_phase)
            .collect::<Vec<_>>();
        let iswitch_initial_loads = self
            .iswitches
            .iter()
            .map(crate::device::CurrentSwitch::initial_load_phase)
            .collect::<Vec<_>>();
        let generic_switch_initial_loads = self
            .generic_switches
            .iter()
            .map(crate::device::GenericSwitch::initial_load_phase)
            .collect::<Vec<_>>();
        let core_mag_updates = if preserve_xyce_core_carry {
            self.jiles_atherton_inductors
                .iter()
                .map(|binding| {
                    binding
                        .device
                        .is_xyce_core_level2()
                        .then(|| binding.device.xyce_core_mag_update())
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let grouped_core_mag_updates = if preserve_xyce_core_carry {
            self.xyce_core_groups
                .iter()
                .map(|group| {
                    group
                        .device
                        .is_xyce_core_level2()
                        .then(|| group.device.xyce_core_mag_update())
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        if let Some(capacitors) = snapshot.capacitors {
            self.capacitors = capacitors;
        }
        if let Some(inductors) = snapshot.inductors {
            self.inductors = inductors;
        }
        self.jiles_atherton_inductors = snapshot.jiles_atherton_inductors;
        self.xyce_core_groups = snapshot.xyce_core_groups;
        if preserve_xyce_core_carry {
            for (binding, update) in self
                .jiles_atherton_inductors
                .iter_mut()
                .zip(core_mag_updates.into_iter())
            {
                if let Some(update) = update {
                    binding.device.restore_xyce_core_mag_update(update);
                }
                binding.device.invalidate_xyce_core_trial();
            }
            for (group, update) in self
                .xyce_core_groups
                .iter_mut()
                .zip(grouped_core_mag_updates.into_iter())
            {
                if let Some(update) = update {
                    group.device.restore_xyce_core_mag_update(update);
                }
                group.device.invalidate_xyce_core_trial();
            }
        }
        self.diodes.restore_nonlinear_state(snapshot.diodes);
        self.bjts = snapshot.bjts;
        self.mosfets.restore_nonlinear_state(snapshot.mosfets);
        self.b3soi = snapshot.b3soi;
        self.b3soi_fd = snapshot.b3soi_fd;
        self.b3soi_pd = snapshot.b3soi_pd;
        self.bsim3v3 = snapshot.bsim3v3;
        self.bsim4v8 = snapshot.bsim4v8;
        self.ekv26s = snapshot.ekv26s;
        self.ekv3s = snapshot.ekv3s;
        self.vdmoses = snapshot.vdmoses;
        self.jfets = snapshot.jfets;
        self.xyce_memristors = snapshot.xyce_memristors;
        self.xyce_memristor_operating_point_mode = snapshot.xyce_memristor_operating_point_mode;
        self.vswitches = snapshot.vswitches;
        self.iswitches = snapshot.iswitches;
        self.generic_switches = snapshot.generic_switches;
        for (switch, phase) in self.vswitches.iter_mut().zip(vswitch_initial_loads) {
            switch.retain_initial_load_progress(phase);
        }
        for (switch, phase) in self.iswitches.iter_mut().zip(iswitch_initial_loads) {
            switch.retain_initial_load_progress(phase);
        }
        for (switch, phase) in self
            .generic_switches
            .iter_mut()
            .zip(generic_switch_initial_loads)
        {
            switch.retain_initial_load_progress(phase);
        }
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
        #[cfg(feature = "veriloga-builtins-base")]
        {
            self.generated_veriloga_devices
                .restore_rollback_state(snapshot.generated_veriloga_devices);
        }
    }

    /// Update all nonlinear devices with current solution
    pub fn update_nonlinear(&mut self, voltages: &[Value]) {
        self.update_nonlinear_impl(voltages, None, false);
    }

    /// Prime device history from a solver seed while preserving the one-shot
    /// switch initial-junction load. Preparatory seed evaluation is not a
    /// matrix load and therefore must not consume instance `ON`/`OFF`.
    pub(crate) fn prime_nonlinear_operating_point(&mut self, voltages: &[Value]) {
        self.update_nonlinear_impl(voltages, None, true);
    }

    /// Enter the one-shot SPICE initial-junction phase for every switch in a
    /// freshly built circuit. Each device ignores repeated calls after the
    /// phase starts, so continuation retries and sweep points cannot replay
    /// an authored initial state.
    pub(crate) fn begin_switch_initial_junction_load(&mut self) {
        for switch in &mut self.vswitches {
            switch.begin_initial_junction_load();
        }
        for switch in &mut self.iswitches {
            switch.begin_initial_junction_load();
        }
        for switch in &mut self.generic_switches {
            switch.begin_initial_junction_load();
        }
    }

    #[cfg(feature = "parallel")]
    pub(crate) fn update_nonlinear_parallel_classic_mos(
        &mut self,
        voltages: &[Value],
        worker_count: usize,
    ) {
        self.update_nonlinear_impl(voltages, Some(worker_count), false);
    }

    fn update_nonlinear_impl(
        &mut self,
        voltages: &[Value],
        parallel_classic_mos_workers: Option<usize>,
        preserve_switch_initial_load: bool,
    ) {
        use crate::device::NonlinearDevice;
        self.diodes.update_all(voltages);
        self.bjts.update_all(voltages);
        #[cfg(feature = "parallel")]
        if let Some(worker_count) = parallel_classic_mos_workers {
            self.mosfets.update_all_parallel(voltages, worker_count);
        } else {
            self.mosfets.update_all(voltages);
        }
        #[cfg(not(feature = "parallel"))]
        {
            let _ = parallel_classic_mos_workers;
            self.mosfets.update_all(voltages);
        }
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
            if preserve_switch_initial_load {
                vswitch.prime_operating_point_seed(voltages);
            } else {
                vswitch.update(voltages);
            }
        }
        for iswitch in &mut self.iswitches {
            if preserve_switch_initial_load {
                iswitch.prime_operating_point_seed(voltages);
            } else {
                iswitch.update(voltages);
            }
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
        self.stamp_xyce_memristors(matrix, rhs, voltages)?;
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

    fn stamp_xyce_memristors(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
    ) -> Result<(), String> {
        #[inline]
        fn value_at(solution: &[Value], node: NodeId) -> Result<Value, String> {
            solution_node_voltage(solution, node)
                .ok_or_else(|| format!("Xyce memristor node {node} is outside the solution vector"))
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

        for binding in &self.xyce_memristors {
            let v_pos = value_at(solution, binding.node_pos)?;
            let v_neg = value_at(solution, binding.node_neg)?;
            let x = value_at(solution, binding.node_x)?;
            let cache = binding
                .device
                .evaluate(v_pos, v_neg, x, self.xyce_memristor_operating_point_mode)
                .map_err(|error| {
                    format!(
                        "{} memristor '{}': {error}",
                        binding.device.family_name(),
                        binding.name
                    )
                })?;
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

            let row_jacobian = cache.jacobian[2];
            let state_residual = cache.residual[2];
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
        self.stamp_generic_switches_with_solution(matrix, rhs, &[], time);
    }

    /// Stamp generic switches using the current Newton solution so CONTROL
    /// expressions containing V(...) or I(...) receive their Jacobian
    /// coupling. The zero-solution compatibility wrapper above remains useful
    /// for the purely time-dependent linear path.
    pub fn stamp_generic_switches_with_solution(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for switch in &mut self.generic_switches {
            switch.stamp_time_dependent_with_solution(time, solution, &mut stamper);
        }
    }

    /// Commit generic-switch store vectors after a transient timepoint is
    /// accepted. Trial Newton stamps intentionally remain rollback-free and
    /// continue to observe the previous accepted hysteresis state.
    pub(crate) fn accept_generic_switch_transient_step(&mut self) {
        for switch in &mut self.generic_switches {
            switch.accept_transient_step();
        }
    }

    /// Seed generic-switch accepted store history from the operating point,
    /// mirroring Xyce's constant-history initialization before transient time
    /// integration begins.
    pub(crate) fn initialize_generic_switch_transient_history(&mut self) {
        for switch in &mut self.generic_switches {
            switch.initialize_transient_history();
        }
    }

    /// Capture generic-switch last/current/next store vectors and accepted
    /// conductance in stable circuit insertion order.
    pub(crate) fn generic_switch_transient_store_snapshots(&self) -> Vec<[Value; 4]> {
        self.generic_switches
            .iter()
            .map(crate::device::GenericSwitch::transient_store_snapshot)
            .collect()
    }

    /// Restore generic-switch transient stores after checkpoint shape and
    /// numeric validation has completed.
    pub(crate) fn restore_generic_switch_transient_store_snapshots(
        &mut self,
        snapshots: &[[Value; 4]],
    ) {
        debug_assert_eq!(self.generic_switches.len(), snapshots.len());
        for (switch, &snapshot) in self.generic_switches.iter_mut().zip(snapshots) {
            switch.restore_transient_store_snapshot(snapshot);
        }
    }

    pub(crate) fn generic_switch_count(&self) -> usize {
        self.generic_switches.len()
    }

    /// Stamp behavioral sources with the given analysis time.
    pub fn stamp_behavioral_sources(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
    ) -> Result<(), String> {
        self.behavioral_sources
            .stamp_all(matrix, rhs, solution, self.num_nodes, time)
            .map_err(|error| error.to_string())
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
    ) -> Result<(), String> {
        self.stamp_behavioral_with_generated_mode(
            matrix,
            rhs,
            solution,
            time,
            analysis,
            crate::device::veriloga_builtins::GeneratedEvaluationMode::NewtonLimited,
        )
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
    ) -> Result<(), String> {
        self.stamp_behavioral_with_generated_mode(
            matrix,
            rhs,
            solution,
            time,
            analysis,
            crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticProbe,
        )
    }

    /// Stamp the transient model's physical static DAE contribution while
    /// suppressing generated dynamic operators and preserving their history.
    pub(crate) fn stamp_behavioral_static_dae_probe(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) -> Result<(), String> {
        self.stamp_behavioral_with_generated_mode(
            matrix,
            rhs,
            solution,
            time,
            analysis,
            crate::device::veriloga_builtins::GeneratedEvaluationMode::StaticDaeProbe,
        )
    }

    pub(crate) fn stamp_behavioral_with_generated_mode(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        _analysis: crate::xspice::AnalysisType,
        _evaluation_mode: crate::device::veriloga_builtins::GeneratedEvaluationMode,
    ) -> Result<(), String> {
        self.stamp_behavioral_sources(matrix, rhs, solution, time)?;
        #[cfg(feature = "veriloga-builtins-base")]
        {
            let generated_analysis = match _analysis {
                crate::xspice::AnalysisType::Ac => {
                    crate::device::veriloga_builtins::GeneratedAnalysisKind::Ac
                }
                crate::xspice::AnalysisType::Transient => {
                    crate::device::veriloga_builtins::GeneratedAnalysisKind::Tran
                }
                crate::xspice::AnalysisType::Noise => {
                    crate::device::veriloga_builtins::GeneratedAnalysisKind::Noise
                }
                _ => crate::device::veriloga_builtins::GeneratedAnalysisKind::Dc,
            };
            let num_nodes = self.num_nodes;
            let simparams = self.generated_simulation_parameters;
            self.generated_veriloga_devices_mut()
                .stamp_all_with_mode(
                    matrix,
                    rhs,
                    solution,
                    num_nodes,
                    generated_analysis,
                    simparams,
                    _evaluation_mode,
                )
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }

    /// Check if all nonlinear devices have converged
    pub fn nonlinear_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        use crate::device::NonlinearDevice;
        #[cfg(feature = "veriloga-builtins-base")]
        let generated_veriloga_converged = self.generated_veriloga_devices.all_converged();
        #[cfg(not(feature = "veriloga-builtins-base"))]
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
            && !self.xyce_core_trial_invalid
    }

    /// Return whether the current transient assembly could not evaluate a
    /// physically valid Xyce Core constitutive endpoint.  This is an
    /// assembly-local status, not a persistent device convergence flag.
    #[inline]
    pub(crate) fn xyce_core_trial_invalid(&self) -> bool {
        self.xyce_core_trial_invalid
    }

    pub fn behavioral_linearizations_converged(
        &mut self,
        solution: &[Value],
        time: Value,
        reltol: Value,
        voltage_abstol: Value,
        current_abstol: Value,
    ) -> Result<bool, String> {
        self.behavioral_sources
            .linearizations_converged(solution, time, reltol, voltage_abstol, current_abstol)
            .map_err(|error| error.to_string())
    }
}

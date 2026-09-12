//! The model acceptance barrier for native SPICE, XSPICE and HDL participants.
//! Solver-controller acceptance, observation and task publication remain in the
//! parent stepper. Reversible external resources join through registered undo images.

use super::*;

/// Borrowed native state carried through the same validation barrier as HDL.
/// Preparation owns only new values; accepted history storage stays in place.
pub(super) struct NativeHistoryAcceptance<'state, 'inputs> {
    pub histories: TransientDeviceHistories<'state>,
    pub bsim4_trnqs_coeff: &'inputs CompanionCoefficients,
    pub snapshots: AcceptedReactiveSnapshots<'inputs>,
    pub scheduling: ReactiveBreakpointScheduling<'state>,
    pub sink: DynamicBreakpointSink<'state>,
}

impl Engine {
    #[cfg(all(test, feature = "veriloga"))]
    #[allow(clippy::too_many_arguments)]
    pub(super) fn accept_external_transient_models(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &mut [Value],
        time: Value,
        dt: Value,
        coefficients: &CompanionCoefficients,
        xyce_one_step_order2: bool,
        capture_static_history: bool,
        baseline_diag_gmin: Value,
        initial_step: bool,
        final_step: bool,
    ) -> Result<(bool, Option<Vec<Value>>), SimulationError> {
        self.accept_transient_models(
            circuit,
            matrix,
            solution,
            time,
            dt,
            coefficients,
            xyce_one_step_order2,
            capture_static_history,
            baseline_diag_gmin,
            initial_step,
            final_step,
            None,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn accept_transient_models(
        &self,
        circuit: &mut crate::circuit::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &mut [Value],
        time: Value,
        dt: Value,
        coefficients: &CompanionCoefficients,
        xyce_one_step_order2: bool,
        capture_static_history: bool,
        baseline_diag_gmin: Value,
        initial_step: bool,
        final_step: bool,
        mut native: Option<NativeHistoryAcceptance<'_, '_>>,
    ) -> Result<(bool, Option<Vec<Value>>), SimulationError> {
        if let Some(error) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(error));
        }
        // The warning de-dup is scoped to the point it was measured at: the
        // step this accepts is over, so the next one starts from silence and
        // a failure that repeats at the next timepoint is reported again.
        circuit.clear_xspice_evaluation_warning();
        let has_xspice = circuit.has_xspice_devices();
        let rollback = has_xspice.then(|| circuit.capture_xspice_acceptance());
        let mut projected = Vec::new();
        let result = (|| {
            let finish = |circuit: &mut crate::CircuitData,
                          solution: &mut [Value],
                          projected: &[(usize, Value)],
                          mixed_static: Option<&[Value]>| {
                if !projected.is_empty()
                    && let Some(native) = native.as_mut()
                {
                    native.snapshots.vbic_snapshots = None;
                    native.snapshots.capacitor_accepted_states = None;
                    native.snapshots.mosfet_caps = None;
                    native.snapshots.mosfet_gate_companion_charges = None;
                }
                // Refresh before final analog evaluation: updating device voltages
                // invalidates numerical candidates, while static observation must
                // retain the complete final candidate for subsequent acceptance.
                if capture_static_history
                    || (!projected.is_empty() && circuit.has_nonlinear_devices())
                {
                    self.update_transient_nonlinear_devices(circuit, solution)?;
                }
                #[cfg(feature = "veriloga")]
                if circuit.has_veriloga_devices() {
                    circuit
                        .evaluate_veriloga_timepoint(solution)
                        .map_err(SimulationError::Circuit)?;
                }
                #[cfg(feature = "veriloga-builtins-base")]
                if circuit.has_generated_veriloga_devices() {
                    circuit
                        .evaluate_generated_veriloga_timepoint(matrix, solution)
                        .map_err(SimulationError::Circuit)?;
                }
                let static_history = if capture_static_history {
                    let mut history = self.capture_xyce_static_residual(
                        circuit,
                        matrix,
                        solution,
                        time,
                        baseline_diag_gmin,
                    )?;
                    if let Some(mixed) = mixed_static {
                        if history.len() != mixed.len() {
                            return Err(SimulationError::Circuit(
                                "mixed static history has the wrong circuit dimension".into(),
                            ));
                        }
                        for (value, mixed) in history.iter_mut().zip(mixed) {
                            *value += mixed;
                            if !value.is_finite() {
                                return Err(SimulationError::Circuit(
                                    "combined static history is not finite".into(),
                                ));
                            }
                        }
                    }
                    Some(history)
                } else {
                    None
                };
                // Use the final projected electrical candidate, but retain the
                // preceding material/load state until every HDL participant agrees.
                let thermal = circuit
                    .resistors
                    .prepare_thermal_step(solution, dt)
                    .map_err(SimulationError::Circuit)?;
                let prepared_native = native
                    .as_ref()
                    .map(|native| {
                        self.prepare_reactive_history(
                            circuit,
                            AcceptedReactiveStep {
                                accepted_solution: solution,
                                accepted_time: time,
                                dt,
                                coeff: coefficients,
                                bsim4_trnqs_coeff: native.bsim4_trnqs_coeff,
                            },
                            &native.histories,
                            native.snapshots,
                        )
                    })
                    .transpose()?;
                let discontinuity = circuit.accept_model_transient_timestep(
                    time,
                    dt,
                    solution,
                    coefficients,
                    initial_step,
                    final_step,
                )?;
                if let (Some(native), Some(prepared)) = (native, prepared_native) {
                    self.commit_reactive_history(
                        circuit,
                        AcceptedReactiveStep {
                            accepted_solution: solution,
                            accepted_time: time,
                            dt,
                            coeff: coefficients,
                            bsim4_trnqs_coeff: native.bsim4_trnqs_coeff,
                        },
                        native.histories,
                        native.snapshots,
                        native.scheduling,
                        native.sink,
                        prepared,
                    );
                }
                circuit.resistors.commit_thermal_step(thermal);
                // The joint HDL barrier has completed all fallible operations.
                // XSPICE promotion only swaps/copies already evaluated histories.
                if has_xspice {
                    circuit.accept_xspice_timestep();
                }
                Ok((discontinuity, static_history))
            };
            #[cfg(feature = "veriloga")]
            let coupled = circuit.has_coupled_event_nets();
            #[cfg(not(feature = "veriloga"))]
            let coupled = false;
            if has_xspice && !coupled {
                circuit
                    .evaluate_xspice_transient_timestep_with_coefficients(
                        time,
                        dt,
                        solution,
                        XspiceCompanionPolicy {
                            coefficients,
                            xyce_one_step_order2,
                        },
                        rollback.as_ref().map(|snapshot| snapshot.resources()),
                    )
                    .map_err(|error| {
                        SimulationError::Circuit(format!(
                            "XSPICE candidate acceptance failed: {error}"
                        ))
                    })?;
                let num_nodes = circuit.num_nodes();
                projected = circuit.project_xspice_voltage_outputs(solution, num_nodes);
            }
            #[cfg(feature = "veriloga")]
            if circuit.has_mixed_signal_hosts() {
                let (mixed_discontinuity, (discontinuity, history)) = circuit
                    .accept_mixed_transient_with(
                        time,
                        dt,
                        solution,
                        XspiceCompanionPolicy {
                            coefficients,
                            xyce_one_step_order2,
                        },
                        initial_step,
                        final_step,
                        rollback.as_ref().map(|snapshot| snapshot.resources()),
                        capture_static_history,
                        &mut projected,
                        finish,
                    )?;
                return Ok((discontinuity || mixed_discontinuity, history));
            }
            finish(circuit, solution, &projected, None)
        })();
        match (result, rollback) {
            (Ok(value), rollback) => {
                if let Some(rollback) = rollback {
                    rollback.resources().commit();
                }
                Ok(value)
            }
            (Err(error), rollback) => {
                for (index, value) in projected.into_iter().rev() {
                    solution[index] = value;
                }
                if let Some(rollback) = rollback
                    && let Err(restore) = circuit.restore_xspice_acceptance(rollback)
                {
                    return Err(SimulationError::Circuit(format!("{error}; {restore}")));
                }
                Err(error)
            }
        }
    }
}

#[cfg(all(test, feature = "veriloga"))]
mod tests {
    use super::*;
    use crate::xspice::event_scheduler::SchedulerLimits;
    use crate::xspice::verilog::MixedSignalHost;
    use crate::xspice::{
        CmContext, CmError, CmResult, CodeModel, ParamSpec, PortConnection, PortSpec, PortType,
        TransactionalContextResource, XspiceInstance,
    };
    use rspice_veriloga::{VerilogACompiler, device::VerilogADevice};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct ProbeResource {
        value: Mutex<u64>,
        captures: AtomicUsize,
        fail_restore: AtomicBool,
    }

    impl TransactionalContextResource for ProbeResource {
        fn capture_transaction_state(&self) -> CmResult<Vec<u8>> {
            self.captures.fetch_add(1, Ordering::Relaxed);
            Ok(self.value.lock().unwrap().to_le_bytes().to_vec())
        }
        fn restore_transaction_state(&self, state: &[u8]) -> CmResult<()> {
            if self.fail_restore.load(Ordering::Relaxed) {
                return Err(CmError::EvaluationError(
                    "injected external restore failure".into(),
                ));
            }
            *self.value.lock().unwrap() = u64::from_le_bytes(state.try_into().unwrap());
            Ok(())
        }
    }

    struct StatefulProbe {
        resource: Arc<ProbeResource>,
        fail: bool,
        voltage: bool,
        ports: Vec<PortSpec>,
    }
    impl CodeModel for StatefulProbe {
        fn name(&self) -> &str {
            "acceptance_probe"
        }
        fn ports(&self) -> &[PortSpec] {
            &self.ports
        }
        fn parameters(&self) -> &[ParamSpec] {
            &[]
        }
        fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
            ctx.allocate_states(1);
            Ok(())
        }
        fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
            if ctx.evaluation_phase() == crate::xspice::EvaluationPhase::AcceptedStep {
                if ctx
                    .transactional_resource::<ProbeResource>("external")?
                    .is_none()
                {
                    // Exercise registration during a candidate, after the
                    // owned context rollback snapshot has already been taken.
                    ctx.set_transactional_resource("external", self.resource.clone());
                }
                let resource = ctx
                    .transactional_resource::<ProbeResource>("external")?
                    .unwrap();
                *resource.value.lock().unwrap() += 1;
                // Multiple accesses must retain the original undo image.
                ctx.transactional_resource::<ProbeResource>("external")?;
            }
            ctx.set_state(0, ctx.state_prev(0) + 1.0);
            ctx.set_output(
                "out",
                if self.voltage && ctx.time > 0.0 {
                    0.75
                } else {
                    0.0
                },
            );
            ctx.request_breakpoint(ctx.time + 7e-9);
            if self.fail && ctx.input("fail") > 0.5 {
                return Err(CmError::EvaluationError(
                    "candidate deliberately refused".into(),
                ));
            }
            Ok(())
        }
    }

    fn fixture(
        fail_xspice: bool,
    ) -> (
        Engine,
        crate::CircuitData,
        crate::solver::StaticMatrix,
        Vec<Value>,
        Vec<Arc<ProbeResource>>,
    ) {
        fixture_with_deck(fail_xspice, "")
    }

    fn fixture_with_deck(
        fail_xspice: bool,
        extra_deck: &str,
    ) -> (
        Engine,
        crate::CircuitData,
        crate::solver::StaticMatrix,
        Vec<Value>,
        Vec<Arc<ProbeResource>>,
    ) {
        let engine = Engine::new(crate::SimulationConfig::default());
        let deck = Netlist::parse(&format!(
            "barrier\nRload p 0 1k\nRadc adc 0 1k\nRbad bad 0 1k\n{extra_deck}\n.end\n"
        ))
        .unwrap();
        let mut circuit = engine.build_circuit(&deck).unwrap();
        let p = circuit.get_node_by_name("p").unwrap();
        let adc = circuit.get_node_by_name("adc").unwrap();
        let bad = circuit.get_node_by_name("bad").unwrap();
        let worker =
            "module worker(p,adc); inout p; electrical p; input adc; wire adc; reg q,later;
            initial begin q=0; later=0; later<=#2 1; end
            always @(posedge adc) q=~q;
            analog I(p)<+q*1e-3;
            endmodule";
        for (name, source, nodes) in [
            ("first", worker.to_string(), vec![p]),
            (
                "second",
                worker
                    .replace("worker(p,adc)", "worker(p,bad,adc)")
                    .replace("inout p; electrical p;", "inout p,bad; electrical p,bad;")
                    .replace("q*1e-3", "q*1e-3 + sqrt(1-V(bad))*1e-6"),
                vec![p, bad],
            ),
        ] {
            let mut host =
                MixedSignalHost::compile(&source, None, name, &nodes, SchedulerLimits::default())
                    .unwrap();
            host.add_adc_bridge("adc", 0, (adc, 0), 0.4, 0.6).unwrap();
            circuit.add_mixed_signal_host(host).unwrap();
        }
        let compiled=VerilogACompiler::default().compile_runtime(
            "module observer(p); inout p; electrical p; analog begin I(p)<+V(p)*1e-3; if(V(p)>0.5) $finish(0); end endmodule",None).unwrap();
        circuit.add_veriloga_device(
            VerilogADevice::try_new_with_canonical_ir(
                "observer",
                compiled.model,
                &compiled.canonical_ir,
                &[p],
            )
            .unwrap(),
        );
        circuit.begin_veriloga_analysis(2).unwrap();
        circuit.start_mixed_digital_execution().unwrap();
        let mut resources = Vec::new();
        for index in 0..2 {
            let resource = Arc::new(ProbeResource::default());
            resources.push(resource.clone());
            let model = StatefulProbe {
                resource,
                fail: fail_xspice && index == 1,
                voltage: index == 0,
                ports: vec![
                    PortSpec::input("fail", PortType::Voltage),
                    PortSpec::output(
                        "out",
                        if index == 0 {
                            PortType::Voltage
                        } else {
                            PortType::Current
                        },
                    ),
                ],
            };
            let mut instance = XspiceInstance::new(
                &format!("A{index}"),
                Arc::new(model),
                vec![PortConnection::Analog(bad), PortConnection::Analog(p)],
                &[],
                &[],
                &[],
                &[],
            )
            .unwrap();
            if index == 0 {
                let branch = circuit.allocate_branch_named("A0#out");
                instance.set_output_branch(1, branch).unwrap();
            }
            circuit.add_xspice_instance(instance);
        }
        let n = circuit.matrix_size();
        let entries: Vec<_> = (0..n).map(|i| (i, i, 1e-3)).collect();
        let matrix = crate::solver::StaticMatrix::from_triplets(n, n, &entries).unwrap();
        (engine, circuit, matrix, vec![0.0; n], resources)
    }

    fn step(
        engine: &Engine,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &mut [Value],
        time: Value,
        dt: Value,
    ) -> Result<(bool, Option<Vec<Value>>), SimulationError> {
        step_with_policy(engine, circuit, matrix, solution, time, dt, false)
    }

    fn step_with_policy(
        engine: &Engine,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &mut [Value],
        time: Value,
        dt: Value,
        weighted: bool,
    ) -> Result<(bool, Option<Vec<Value>>), SimulationError> {
        let coefficients = CompanionCoefficients::backward_euler();
        circuit
            .prepare_veriloga_timepoint_with_policy(
                time,
                dt,
                XspiceCompanionPolicy {
                    coefficients: &coefficients,
                    xyce_one_step_order2: weighted,
                },
                time == 0.0,
                false,
            )
            .unwrap();
        engine.accept_external_transient_models(
            circuit,
            matrix,
            solution,
            time,
            dt,
            &coefficients,
            weighted,
            true,
            0.0,
            time == 0.0,
            false,
        )
    }

    #[test]
    fn accepted_static_history_includes_settled_mixed_runtime_and_dac_terms() {
        let engine = Engine::new(crate::SimulationConfig::default());
        let deck = Netlist::parse("static history\nRnative native 0 2\nRva va 0 2\nRmix mixed 0 2\nRdac bridge 0 2\n.end\n").unwrap();
        let mut circuit = engine.build_circuit(&deck).unwrap();
        let va = circuit.get_node_by_name("va").unwrap();
        let mixed = circuit.get_node_by_name("mixed").unwrap();
        let bridge = circuit.get_node_by_name("bridge").unwrap();
        let native = circuit.get_node_by_name("native").unwrap();
        let compiled = VerilogACompiler::default()
            .compile_runtime(
                "module analog_only(p); inout p; electrical p;
             analog I(p)<+2*V(p)+ddt(3e-9*V(p))+idt(1e9*V(p),4); endmodule",
                None,
            )
            .unwrap();
        circuit.add_veriloga_device(
            VerilogADevice::try_new_with_canonical_ir(
                "analog_only",
                compiled.model,
                &compiled.canonical_ir,
                &[va],
            )
            .unwrap(),
        );
        let mut host = MixedSignalHost::compile(
            "module mixed_history(p,q); inout p; electrical p; output q; reg q;
             initial begin q=0; #1 q=1; #1 q=0; end
             analog I(p)<+2*(q+1)*V(p)+ddt(3e-9*V(p))+idt(1e9*V(p),4); endmodule",
            None,
            "mixed_history",
            &[mixed],
            SchedulerLimits::default(),
        )
        .unwrap();
        host.add_dac_bridge("q", 0, (bridge, 0), 0.0, 1.0, 2.0)
            .unwrap();
        circuit.add_mixed_signal_host(host).unwrap();
        assert!(circuit.veriloga_one_step_dae_split_safe());
        circuit.begin_veriloga_analysis(2).unwrap();
        circuit.start_mixed_digital_execution().unwrap();
        let size = circuit.matrix_size();
        let entries: Vec<_> = (0..size).map(|i| (i, i, 1.0)).collect();
        let mut matrix = crate::solver::StaticMatrix::from_triplets(size, size, &entries).unwrap();
        circuit.link_indices(&matrix);
        let mut solution = vec![0.0; size];
        solution[native - 1] = 0.5;
        solution[bridge - 1] = 0.25;
        let (_, initial) =
            step(&engine, &mut circuit, &mut matrix, &mut solution, 0.0, 0.0).unwrap();
        let initial = initial.unwrap();
        assert_eq!(initial[va - 1], 4.0);
        assert_eq!(initial[mixed - 1], 4.0);
        assert_eq!(initial[bridge - 1], 0.25);
        assert_eq!(initial[native - 1], 0.25);
        let checkpoint = circuit.clone();
        for weighted in [false, true] {
            for _replay in 0..2 {
                circuit = checkpoint.clone();
                for (time, voltage, integral, digital) in
                    [(1e-9, 2.0, 6.0, 1.0), (2e-9, 3.0, 9.0, 0.0)]
                {
                    solution[va - 1] = voltage;
                    solution[mixed - 1] = voltage;
                    // The probe uses the derivative bank, while acceptance below
                    // must preserve the full internal integral and undo probe state.
                    let integral = if weighted {
                        if time == 1e-9 { 5.0 } else { 7.5 }
                    } else {
                        integral
                    };
                    let prior_voltage = if time == 1e-9 { 0.0 } else { 2.0 };
                    let ddt_gain = if weighted { 6.0 } else { 3.0 };
                    let idt_gain = if weighted { 0.5 } else { 1.0 };
                    let digital_before = circuit.mixed_signal_hosts[0].read_digital("q").unwrap();
                    let mut probe_rhs = vec![0.0; size];
                    matrix.values_mut().fill(0.0);
                    circuit
                        .stamp_mixed_transient_trial(
                            &mut matrix,
                            &mut probe_rhs,
                            time,
                            1e-9,
                            &solution,
                            XspiceCompanionPolicy {
                                coefficients: &CompanionCoefficients::backward_euler(),
                                xyce_one_step_order2: weighted,
                            },
                            false,
                            false,
                        )
                        .unwrap();
                    let slot = matrix.get_index(mixed - 1, mixed - 1).unwrap();
                    let jacobian = matrix.values_mut()[slot.offset()];
                    let weight = if weighted { 0.5 } else { 1.0 };
                    assert!(
                        (jacobian - weight * (2.0 * (digital + 1.0) + ddt_gain + idt_gain)).abs()
                            < 1e-12
                    );
                    let expected = 2.0 * (digital + 1.0) * voltage
                        + ddt_gain * (voltage - prior_voltage)
                        + integral;
                    assert!(
                        (jacobian * voltage - probe_rhs[mixed - 1] - weight * expected).abs()
                            < 1e-12
                    );
                    let bridge_slot = matrix.get_index(bridge - 1, bridge - 1).unwrap();
                    assert_eq!(matrix.values_mut()[bridge_slot.offset()], weight * 0.5);
                    assert_eq!(probe_rhs[bridge - 1], weight * 0.5 * digital);
                    assert_eq!(
                        circuit.mixed_signal_hosts[0].read_digital("q").unwrap(),
                        digital_before
                    );
                    let (_, history) = step_with_policy(
                        &engine,
                        &mut circuit,
                        &mut matrix,
                        &mut solution,
                        time,
                        1e-9,
                        weighted,
                    )
                    .unwrap();
                    let history = history.unwrap();
                    assert!(
                        (history[va - 1] - (2.5 * voltage + integral)).abs() < 1e-12,
                        "runtime history={history:?}"
                    );
                    assert!(
                        (history[mixed - 1] - ((2.0 * (digital + 1.0) + 0.5) * voltage + integral))
                            .abs()
                            < 1e-12,
                        "mixed history={history:?}"
                    );
                    assert_eq!(history[bridge - 1], 0.25 - 0.5 * digital);
                    assert_eq!(history[native - 1], 0.25);
                    assert_eq!(
                        circuit.mixed_signal_hosts[0].read_digital("q").unwrap(),
                        if digital == 1.0 { "1" } else { "0" }
                    );
                    assert!(!circuit.mixed_signal_hosts[0].trial_active());
                }
            }
        }
    }

    #[test]
    fn acceptance_barrier_restores_earlier_models_when_a_later_mixed_candidate_fails() {
        let (engine, mut circuit, mut matrix, mut solution, resources) = fixture(false);
        step(&engine, &mut circuit, &mut matrix, &mut solution, 0.0, 0.0).unwrap();
        let checkpoint = circuit.clone();
        let before: Vec<_> = circuit
            .xspice_instances
            .iter()
            .map(|instance| instance.checkpoint_state())
            .collect();
        let p = circuit.get_node_by_name("p").unwrap() - 1;
        let adc = circuit.get_node_by_name("adc").unwrap() - 1;
        let resource_before: Vec<_> = resources
            .iter()
            .map(|resource| *resource.value.lock().unwrap())
            .collect();
        let bad = circuit.get_node_by_name("bad").unwrap() - 1;
        solution[p] = 1.0;
        solution[adc] = 0.6;
        solution[bad] = 2.0;
        let error = step(
            &engine,
            &mut circuit,
            &mut matrix,
            &mut solution,
            0.65e-9,
            0.65e-9,
        )
        .unwrap_err();
        assert!(error.to_string().contains("second"), "{error}");
        assert_eq!(
            solution[p], 1.0,
            "failed voltage-output projection must restore the candidate"
        );
        for host in &circuit.mixed_signal_hosts {
            assert!(!host.trial_active());
            assert_eq!(host.read_digital("q").unwrap(), "0");
            assert_eq!(host.read_digital("later").unwrap(), "0");
        }
        assert!(
            (circuit.next_mixed_event_time().unwrap().unwrap() - 2e-9).abs() < 1e-20,
            "the circuit owner retains the pending event after rollback"
        );
        assert_eq!(
            before,
            circuit
                .xspice_instances
                .iter()
                .map(|instance| instance.checkpoint_state())
                .collect::<Vec<_>>()
        );
        let mut calls = 0;
        circuit
            .visit_accepted_analog_tasks(&mut |_| calls += 1)
            .unwrap();
        assert_eq!(
            calls, 0,
            "a failed barrier must not publish the observer's pending finish"
        );
        assert_eq!(
            resource_before,
            resources
                .iter()
                .map(|resource| *resource.value.lock().unwrap())
                .collect::<Vec<_>>(),
            "external resources must restore with the owned context"
        );
        for resource in &resources {
            assert_eq!(
                resource.captures.load(Ordering::Relaxed),
                2,
                "each barrier captures once even with multiple accesses"
            );
        }
        solution[bad] = 0.0;
        for replay in 0..2 {
            if replay == 1 {
                circuit = checkpoint.clone();
            }
            let (_, history) = step(
                &engine,
                &mut circuit,
                &mut matrix,
                &mut solution,
                0.65e-9,
                0.65e-9,
            )
            .unwrap();
            assert!(history.unwrap().iter().all(|value| value.is_finite()));
            for host in &circuit.mixed_signal_hosts {
                assert_eq!(host.read_digital("q").unwrap(), "1");
                assert_eq!(host.read_digital("later").unwrap(), "0");
            }
            calls = 0;
            circuit
                .visit_accepted_analog_tasks(&mut |_| calls += 1)
                .unwrap();
            assert_eq!(
                calls, 1,
                "the successful retry promotes the observer exactly once"
            );
        }
    }

    #[test]
    fn acceptance_barrier_restores_xspice_state_and_events_before_reporting_failure() {
        let (
            initial_engine,
            mut initial_circuit,
            mut initial_matrix,
            mut initial_solution,
            initial_resources,
        ) = fixture(true);
        let bad = initial_circuit.get_node_by_name("bad").unwrap() - 1;
        initial_solution[bad] = 2.0;
        step(
            &initial_engine,
            &mut initial_circuit,
            &mut initial_matrix,
            &mut initial_solution,
            0.0,
            0.0,
        )
        .unwrap_err();
        assert!(
            initial_resources
                .iter()
                .all(|resource| *resource.value.lock().unwrap() == 0),
            "new resources enlisted after the initial snapshot must also roll back"
        );
        // A failed restore of a newly registered resource must remain fatal
        // even after the registration itself disappears in owned-state undo.
        initial_resources[0]
            .fail_restore
            .store(true, Ordering::Relaxed);
        let mut earlier_clone = initial_circuit.clone();
        let error = step(
            &initial_engine,
            &mut initial_circuit,
            &mut initial_matrix,
            &mut initial_solution,
            0.0,
            0.0,
        )
        .unwrap_err();
        let message = error.to_string();
        assert!(
            message.contains("A1")
                && message.contains("A0 resource 'external'")
                && message.contains("injected external restore failure"),
            "{message}"
        );
        assert_eq!(
            *initial_resources[1].value.lock().unwrap(),
            0,
            "a failed provider restore cannot prevent another resource's restoration"
        );
        for circuit in [&mut initial_circuit, &mut earlier_clone] {
            for _ in 0..2 {
                assert!(
                    circuit
                        .take_xspice_evaluation_error()
                        .unwrap()
                        .contains("A0 resource 'external'")
                );
            }
            assert!(
                step(
                    &initial_engine,
                    circuit,
                    &mut initial_matrix,
                    &mut initial_solution,
                    0.0,
                    0.0
                )
                .unwrap_err()
                .to_string()
                .contains("resource rollback failed")
            );
        }
        let (engine, mut circuit, mut matrix, mut solution, resources) = fixture(true);
        step(&engine, &mut circuit, &mut matrix, &mut solution, 0.0, 0.0).unwrap();
        let before: Vec<_> = circuit
            .xspice_instances
            .iter()
            .map(|instance| instance.checkpoint_state())
            .collect();
        let resource_before: Vec<_> = resources
            .iter()
            .map(|resource| *resource.value.lock().unwrap())
            .collect();
        let bad = circuit.get_node_by_name("bad").unwrap() - 1;
        solution[bad] = 2.0;
        let error = step(
            &engine,
            &mut circuit,
            &mut matrix,
            &mut solution,
            0.65e-9,
            0.65e-9,
        )
        .unwrap_err();
        assert!(
            error.to_string().contains("A1") && error.to_string().contains("deliberately refused"),
            "{error}"
        );
        assert_eq!(
            before,
            circuit
                .xspice_instances
                .iter()
                .map(|instance| instance.checkpoint_state())
                .collect::<Vec<_>>()
        );
        for host in &circuit.mixed_signal_hosts {
            assert_eq!(host.read_digital("q").unwrap(), "0");
            assert!(!host.trial_active());
        }
        assert_eq!(
            resource_before,
            resources
                .iter()
                .map(|resource| *resource.value.lock().unwrap())
                .collect::<Vec<_>>(),
            "external resources must restore with the owned context"
        );
        for resource in &resources {
            assert_eq!(
                resource.captures.load(Ordering::Relaxed),
                2,
                "each barrier captures once even with multiple accesses"
            );
        }
        solution[bad] = 0.0;
        step(
            &engine,
            &mut circuit,
            &mut matrix,
            &mut solution,
            0.65e-9,
            0.65e-9,
        )
        .unwrap();
        assert!(circuit.take_xspice_evaluation_error().is_none());
        for (resource, before) in resources.iter().zip(resource_before) {
            assert_eq!(*resource.value.lock().unwrap(), before + 1);
        }
    }
    fn thermal_values(circuit: &crate::CircuitData) -> (Vec<Value>, Vec<Option<[Value; 7]>>) {
        (
            circuit.resistors.conductances.clone(),
            circuit
                .resistors
                .thermal
                .iter()
                .map(|state| {
                    state.as_ref().map(|state| {
                        [
                            state.temperature_celsius,
                            state.resistivity,
                            state.heat_capacity,
                            state.thermal_heat_capacity,
                            state.reported_resistance,
                            state.output_resistance,
                            state.output_conductance,
                        ]
                    })
                })
                .collect(),
        )
    }

    #[test]
    fn thermal_acceptance_prepares_every_resistor_before_mixed_promotion() {
        let (engine, mut circuit, mut matrix, mut solution, resources) = fixture(false);
        let thermal_deck = Netlist::parse("thermal setup\nRhot a 0 hot L=1 A=1\n.model hot R(LEVEL=2 RESISTIVITY=1000 HEATCAPACITY=1e-12)\n.end\n").unwrap();
        let thermal_circuit = engine.build_circuit(&thermal_deck).unwrap();
        let thermal = thermal_circuit.resistors.thermal[0].as_ref().unwrap();
        let load = circuit
            .resistors
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("rload"))
            .unwrap();
        let later = circuit
            .resistors
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("rbad"))
            .unwrap();
        for index in [load, later] {
            circuit.resistors.thermal[index] = Some(thermal.clone());
        }
        step(&engine, &mut circuit, &mut matrix, &mut solution, 0.0, 0.0).unwrap();
        let accepted = thermal_values(&circuit);
        let p = circuit.get_node_by_name("p").unwrap() - 1;
        let adc = circuit.get_node_by_name("adc").unwrap() - 1;
        let bad = circuit.get_node_by_name("bad").unwrap() - 1;
        solution[p] = 1.0;
        solution[adc] = 0.6;
        solution[bad] = 2.0;
        let dt = 0.65e-9;
        let error = step(&engine, &mut circuit, &mut matrix, &mut solution, dt, dt).unwrap_err();
        assert!(error.to_string().contains("second"), "{error}");
        assert_eq!(
            thermal_values(&circuit),
            accepted,
            "a later HDL refusal must discard prepared heating"
        );
        assert_eq!(solution[p], 1.0);

        solution[bad] = 0.0;
        circuit.resistors.thermal[later]
            .as_mut()
            .unwrap()
            .instance_resistivity = Some(-1.0);
        let error = circuit
            .resistors
            .advance_thermal_states(&solution, dt)
            .unwrap_err();
        assert!(error.to_ascii_lowercase().contains("rbad"), "{error}");
        assert_eq!(
            thermal_values(&circuit),
            accepted,
            "standalone advancement must not partially update earlier resistors"
        );
        let error = step(&engine, &mut circuit, &mut matrix, &mut solution, dt, dt).unwrap_err();
        assert!(
            error.to_string().to_ascii_lowercase().contains("rbad"),
            "{error}"
        );
        assert_eq!(thermal_values(&circuit), accepted);
        assert_eq!(
            solution[p], 1.0,
            "the XSPICE projection must undo with refused material values"
        );
        assert!(
            resources
                .iter()
                .all(|resource| *resource.value.lock().unwrap() == 1)
        );
        for host in &circuit.mixed_signal_hosts {
            assert_eq!(host.read_digital("q").unwrap(), "0");
        }
        let mut tasks = 0;
        circuit
            .visit_accepted_analog_tasks(&mut |_| tasks += 1)
            .unwrap();
        assert_eq!(
            tasks, 0,
            "material failure cannot publish the observer's finish"
        );

        circuit.resistors.thermal[later]
            .as_mut()
            .unwrap()
            .instance_resistivity = None;
        step(&engine, &mut circuit, &mut matrix, &mut solution, dt, dt).unwrap();
        assert_eq!(solution[p], 0.75);
        let expected =
            thermal.temperature_celsius + (0.75_f64 / 1000.0).powi(2) * 1000.0 * dt / 1e-12;
        let state = circuit.resistors.thermal[load].as_ref().unwrap();
        assert!(
            (state.temperature_celsius - expected).abs() < 1e-12,
            "heating must use the final projected voltage exactly once: {} versus {expected}",
            state.temperature_celsius
        );
        assert_eq!(state.output_resistance, 1000.0);
        assert_eq!(state.output_conductance, 1e-3);
        assert!(
            resources
                .iter()
                .all(|resource| *resource.value.lock().unwrap() == 2)
        );
        for host in &circuit.mixed_signal_hosts {
            assert_eq!(host.read_digital("q").unwrap(), "1");
        }
        tasks = 0;
        circuit
            .visit_accepted_analog_tasks(&mut |_| tasks += 1)
            .unwrap();
        assert_eq!(tasks, 1);
    }

    #[test]
    fn thermal_acceptance_engine_paths_preserve_the_constant_power_oracle() {
        for source in [
            "V1 in 0 1",
            "V1 source 0 1\nA1 source in amp\n.model amp gain(gain=1)",
        ] {
            let deck = Netlist::parse(&format!("constant power\n{source}\nRhot in 0 hot L=1 A=1\n.model hot R(LEVEL=2 RESISTIVITY=1000 HEATCAPACITY=1e-12)\n.save @rhot[temp] @rhot[r]\n.end\n")).unwrap();
            let result = Engine::default().run_tran(&deck, 2e-9, 0.5e-9).unwrap();
            let temperature = result
                .device_op_traces
                .iter()
                .find(|trace| {
                    trace.device_name.eq_ignore_ascii_case("rhot")
                        && trace.parameter.eq_ignore_ascii_case("temp")
                })
                .unwrap();
            assert_eq!(temperature.values.len(), result.time.len());
            assert!(temperature.values.last().unwrap() > &temperature.values[0]);
            for (&time, &temperature) in result.time.iter().zip(&temperature.values) {
                let expected = 27.0 + time * 1e9;
                assert!(
                    (temperature - expected).abs() < 1e-8,
                    "{source}: thermal state at {time:e} is {temperature}, expected {expected}"
                );
            }
        }
    }
    fn joint_native_step(
        engine: &Engine,
        circuit: &mut crate::CircuitData,
        matrix: &mut crate::solver::StaticMatrix,
        solution: &mut [Value],
        bjt: &mut BjtTransientHistory,
        time: Value,
    ) -> Result<(bool, Option<Vec<Value>>), SimulationError> {
        let dt = time;
        let coefficients = CompanionCoefficients::backward_euler();
        circuit
            .prepare_veriloga_timepoint(time, dt, &coefficients, false, false)
            .unwrap();
        // A legitimate capacitor cache from the pre-projection 1 V candidate.
        let capacitor_candidate = [CapacitorAcceptedState {
            voltage: 1.0,
            current: 1e-9 / dt,
        }];
        engine.accept_transient_models(
            circuit,
            matrix,
            solution,
            time,
            dt,
            &coefficients,
            false,
            false,
            0.0,
            false,
            false,
            Some(NativeHistoryAcceptance {
                histories: TransientDeviceHistories {
                    bjt,
                    jfet: &mut JfetTransientHistory::default(),
                    diode: &mut DiodeTransientHistory::default(),
                    mosfet: &mut MosfetTransientHistory::default(),
                    vdmos: &mut VdmosTransientHistory::default(),
                    b3soi: &mut B3SoiTransientHistory::default(),
                    bsim3: &mut Bsim3TransientHistory::default(),
                    bsim4: &mut Bsim4TransientHistory::default(),
                    ekv26: &mut Ekv26TransientHistory::default(),
                },
                bsim4_trnqs_coeff: &coefficients,
                snapshots: AcceptedReactiveSnapshots {
                    xyce_one_step_order2: false,
                    vbic_snapshots: None,
                    capacitor_accepted_states: Some(&capacitor_candidate),
                    mosfet_caps: None,
                    mosfet_gate_companion_charges: None,
                    suppress_gate_charge_history: false,
                    tline_dc_refs: &[],
                    coupled_tline_refs: &[],
                },
                scheduling: ReactiveBreakpointScheduling {
                    breakpoints: &mut BreakpointManager::new(),
                    tstop: 3e-9,
                    voltage_reltol: 1e-3,
                    voltage_abstol: 1e-6,
                    current_abstol: 1e-12,
                },
                sink: DynamicBreakpointSink {
                    dynamic_breakpoints_added: &mut 0,
                    warned_dynamic_breakpoint_cap: &mut false,
                    pending_dynamic_breakpoints: &mut Vec::new(),
                },
            }),
        )
    }

    #[test]
    fn joint_native_acceptance_preserves_histories_across_external_and_native_refusals() {
        for fail_xspice in [false, true] {
            let (engine, mut circuit, mut matrix, mut solution, resources) = fixture_with_deck(
                fail_xspice,
                "Cmemory p 0 1n\nLmemory p 0 1u\nQmemory p qb 0 qm\nRbase qb 0 1k\n.model qm NPN(IS=1e-14 CJE=1p CJC=2p TF=1n)\nBmemory integral 0 V=sdt(V(p))\nRintegral integral 0 1k\nBcheck checked 0 I={exp(1000*V(failnative))}\nRchecked checked 0 1k\nRfailnative failnative 0 1k",
            );
            let coeff = CompanionCoefficients::backward_euler();
            circuit
                .behavioral_sources
                .accept_transient_step(&solution, 0.0)
                .unwrap();
            let mut bjt = Engine::initialize_bjt_history(
                &circuit,
                &solution,
                ReactiveHistorySeed::SolvedBias,
            );
            circuit
                .prepare_veriloga_timepoint(0.0, 0.0, &coeff, true, false)
                .unwrap();
            engine
                .accept_transient_models(
                    &mut circuit,
                    &mut matrix,
                    &mut solution,
                    0.0,
                    0.0,
                    &coeff,
                    false,
                    false,
                    0.0,
                    true,
                    false,
                    None,
                )
                .unwrap();
            let before = bjt.clone();
            let capacitors = format!("{:?}", circuit.capacitors);
            let inductors = format!("{:?}", circuit.inductors);
            let p = circuit.get_node_by_name("p").unwrap() - 1;
            let bad = circuit.get_node_by_name("bad").unwrap() - 1;
            let adc = circuit.get_node_by_name("adc").unwrap() - 1;
            let failnative = circuit.get_node_by_name("failnative").unwrap() - 1;
            let qb = circuit.bjts.devices[0].node_base - 1;
            let branch = circuit.num_nodes() + circuit.inductors.branch_indices[0] - 1;
            solution[p] = 1.0;
            solution[bad] = 2.0;
            solution[adc] = 0.6;
            solution[qb] = 0.6;
            solution[branch] = 2e-3;
            let time = 0.65e-9;
            for native_failure in [false, true] {
                if native_failure {
                    solution[bad] = 0.0;
                    solution[failnative] = 1.0;
                }
                let error = joint_native_step(
                    &engine,
                    &mut circuit,
                    &mut matrix,
                    &mut solution,
                    &mut bjt,
                    time,
                )
                .unwrap_err();
                let expected = if native_failure {
                    "bcheck"
                } else if fail_xspice {
                    "a1"
                } else {
                    "second"
                };
                assert!(
                    error.to_string().to_ascii_lowercase().contains(expected),
                    "{error}"
                );
                assert_eq!(bjt, before);
                assert_eq!(format!("{:?}", circuit.capacitors), capacitors);
                assert_eq!(format!("{:?}", circuit.inductors), inductors);
                assert_eq!(solution[p], 1.0);
                assert!(
                    resources
                        .iter()
                        .all(|resource| *resource.value.lock().unwrap() == 1)
                );
                assert!(
                    circuit
                        .mixed_signal_hosts
                        .iter()
                        .all(|host| host.read_digital("q").unwrap() == "0")
                );
                let mut effects = 0;
                circuit
                    .visit_accepted_analog_tasks(&mut |_| effects += 1)
                    .unwrap();
                assert_eq!(effects, 0);
            }
            solution[failnative] = 0.0;
            joint_native_step(
                &engine,
                &mut circuit,
                &mut matrix,
                &mut solution,
                &mut bjt,
                time,
            )
            .unwrap();
            assert_ne!(bjt, before);
            assert_eq!(
                circuit.capacitors.v_prev,
                [0.75],
                "the pre-projection capacitor cache must be invalidated"
            );
            assert!((circuit.capacitors.i_prev[0] - 1e-9 * 0.75 / time).abs() < 1e-12);
            assert_eq!(circuit.inductors.i_prev, [2e-3]);
            assert_eq!(circuit.inductors.v_prev, [0.75]);
            let integral = circuit.behavioral_sources.voltage_sources[0]
                .evaluate(&solution, time)
                .unwrap();
            assert!((integral - 0.5 * 0.75 * time).abs() < 1e-24);
            assert!(
                resources
                    .iter()
                    .all(|resource| *resource.value.lock().unwrap() == 2)
            );
            assert!(
                circuit
                    .mixed_signal_hosts
                    .iter()
                    .all(|host| host.read_digital("q").unwrap() == "1")
            );
        }
    }
}

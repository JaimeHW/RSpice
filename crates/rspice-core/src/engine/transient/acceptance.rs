//! The acceptance barrier for XSPICE and compiled analog/mixed participants.
//! Native SPICE history and solver-controller acceptance remain in the parent
//! stepper, along with observation and analog-task publication. External
//! reversible resources join this barrier through registered undo images.

use super::*;

impl Engine {
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
        if let Some(error) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(error));
        }
        let has_xspice = circuit.has_xspice_devices();
        let rollback = has_xspice.then(|| circuit.capture_xspice_acceptance());
        let mut projected = Vec::new();
        let result = (|| {
            if has_xspice {
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
                // Contributions are available from the evaluated candidate;
                // accepted model histories need not advance to project them.
                projected = circuit.project_xspice_voltage_outputs(solution, circuit.num_nodes());
            }
            let static_history = if has_xspice && capture_static_history {
                Some(self.capture_xyce_static_residual(
                    circuit,
                    matrix,
                    solution,
                    time,
                    baseline_diag_gmin,
                )?)
            } else {
                None
            };
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
            let discontinuity = circuit.accept_model_transient_timestep(
                time,
                dt,
                solution,
                coefficients,
                initial_step,
                final_step,
            )?;
            // The joint HDL barrier has completed all fallible operations.
            // XSPICE promotion only swaps/copies already evaluated histories.
            if has_xspice {
                circuit.accept_xspice_timestep();
            }
            Ok((discontinuity, static_history))
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
        let engine = Engine::new(crate::SimulationConfig::default());
        let deck =
            Netlist::parse("barrier\nRload p 0 1k\nRadc adc 0 1k\nRbad bad 0 1k\n.end\n").unwrap();
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
            circuit.add_mixed_signal_host(host);
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
        let coefficients = CompanionCoefficients::backward_euler();
        circuit
            .prepare_veriloga_timepoint(time, dt, &coefficients, time == 0.0, false)
            .unwrap();
        engine.accept_external_transient_models(
            circuit,
            matrix,
            solution,
            time,
            dt,
            &coefficients,
            false,
            true,
            0.0,
            time == 0.0,
            false,
        )
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
            assert!((host.next_event_time().unwrap().unwrap() - 2e-9).abs() < 1e-20);
        }
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
}

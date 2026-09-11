use super::*;
use crate::xspice::event_scheduler::SchedulerLimits;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortConnection, PortSpec, PortType,
    TransactionalContextResource, XspiceInstance,
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};

#[derive(Default)]
struct ProbeResource {
    value: Mutex<u64>,
    captures: AtomicUsize,
}
impl TransactionalContextResource for ProbeResource {
    fn capture_transaction_state(&self) -> CmResult<Vec<u8>> {
        self.captures.fetch_add(1, Ordering::Relaxed);
        Ok(self.value.lock().unwrap().to_le_bytes().to_vec())
    }
    fn restore_transaction_state(&self, bytes: &[u8]) -> CmResult<()> {
        *self.value.lock().unwrap() = u64::from_le_bytes(bytes.try_into().unwrap());
        Ok(())
    }
}
struct ResourceProbe {
    resource: Arc<ProbeResource>,
    ports: Vec<PortSpec>,
    phase: crate::xspice::EvaluationPhase,
}
impl CodeModel for ResourceProbe {
    fn name(&self) -> &str {
        "candidate_resource"
    }
    fn ports(&self) -> &[PortSpec] {
        &self.ports
    }
    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }
    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.set_transactional_resource("counter", self.resource.clone());
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.evaluation_phase() != self.phase {
            return Err(CmError::EvaluationError(
                "unexpected circuit evaluation phase".into(),
            ));
        }
        let resource = ctx
            .transactional_resource::<ProbeResource>("counter")?
            .unwrap();
        *resource.value.lock().unwrap() += 1;
        ctx.set_output("out", 0.0);
        Ok(())
    }
}

fn compile_unstarted(
    source: &str,
    module: Option<&str>,
    instance: &str,
    nodes: &[usize],
    limits: SchedulerLimits,
) -> Result<MixedSignalHost, MixedSignalError> {
    let compiled = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
        enable_ams: true,
        ..Default::default()
    })
    .compile_runtime(source, module)
    .unwrap();
    MixedSignalHost::from_compiled(
        instance,
        Arc::new(compiled.model),
        &compiled.canonical_ir,
        nodes,
        limits,
        &rspice_veriloga::NoPipelineControl,
    )
}

fn fixture(
    phase: crate::xspice::EvaluationPhase,
) -> (
    crate::CircuitData,
    crate::solver::StaticMatrix,
    Vec<f64>,
    Arc<ProbeResource>,
) {
    let engine = crate::Engine::new(crate::SimulationConfig::default());
    let deck = crate::Netlist::parse("joint probe\nRp p 0 1k\nRtap tap 0 1k\nRin stimulus 0 1k\nRload loaded 0 1k\nRbad bad 0 1k\n.end\n").unwrap();
    let mut circuit = engine.build_circuit(&deck).unwrap();
    let p = circuit.get_node_by_name("p").unwrap();
    let tap = circuit.get_node_by_name("tap").unwrap();
    let stimulus = circuit.get_node_by_name("stimulus").unwrap();
    let loaded = circuit.get_node_by_name("loaded").unwrap();
    let bad = circuit.get_node_by_name("bad").unwrap();
    let trigger = circuit.get_or_create_node("trigger");
    let command = circuit.get_or_create_node("command");
    let bus = circuit.get_or_create_node("bus");
    let response = circuit.get_or_create_node("response");
    let mut first = compile_unstarted(
        r#"
module controller(p,trigger,response,command);
 inout p; electrical p; input trigger,response; wire trigger,response;
 output command; reg command,sampled;
 initial begin command=0; sampled=1; end
 always @(posedge trigger) begin command=1; #0 sampled=response; command<=0; end
 analog I(p)<+sampled*1e-3;
endmodule
"#,
        None,
        "controller",
        &[p],
        SchedulerLimits::default(),
    )
    .unwrap();
    first
        .add_adc_bridge("trigger", 0, (trigger, 0), 0.4, 0.6)
        .unwrap();
    first
        .add_adc_bridge("response", 0, (response, 0), 0.4, 0.6)
        .unwrap();
    first
        .add_dac_bridge("command", 0, (command, 0), 0.0, 1.0, 20.0)
        .unwrap();
    circuit.add_mixed_signal_host(first).unwrap();
    let mut second = compile_unstarted(
        r#"
module relay(tap,bad,input_bit,output_bit);
 inout tap,bad; electrical tap,bad;
 input input_bit; wire input_bit; output output_bit; wire output_bit;
 assign output_bit=input_bit;
 analog I(tap)<+V(tap)*1e-6+sqrt(1-V(bad))*1e-6;
endmodule
"#,
        None,
        "relay",
        &[tap, bad],
        SchedulerLimits::default(),
    )
    .unwrap();
    second
        .add_adc_bridge("input_bit", 0, (bus, 0), 0.4, 0.6)
        .unwrap();
    second
        .add_dac_bridge("output_bit", 0, (response, 0), 0.0, 1.0, 20.0)
        .unwrap();
    second
        .add_dac_bridge("output_bit", 0, (loaded, 0), 0.0, 1.0, 20.0)
        .unwrap();
    circuit.add_mixed_signal_host(second).unwrap();
    let resource = Arc::new(ProbeResource::default());
    let instances: Vec<(&str, Arc<dyn CodeModel>, Vec<PortConnection>)> = vec![
        (
            "Aadc",
            Arc::new(crate::xspice::models::AdcBridge),
            vec![
                PortConnection::AnalogVector(vec![stimulus]),
                PortConnection::DigitalVector(vec![trigger]),
            ],
        ),
        (
            "Ainv",
            Arc::new(crate::xspice::models::DigitalInverter),
            vec![
                PortConnection::Digital(command),
                PortConnection::Digital(bus),
            ],
        ),
        (
            "Aresource",
            Arc::new(ResourceProbe {
                resource: resource.clone(),
                phase,
                ports: vec![PortSpec::output("out", PortType::Current)],
            }),
            vec![PortConnection::Analog(p)],
        ),
    ];
    for (name, model, connections) in instances {
        let mut instance =
            XspiceInstance::new(name, model, connections, &[], &[], &[], &[]).unwrap();
        instance.init().unwrap();
        circuit.add_xspice_instance(instance);
    }
    circuit
        .finalize_mixed_digital(
            &[trigger, command, bus, response].into_iter().collect(),
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
    assert!(circuit.mixed_xspice_bindings.is_some());
    circuit.begin_veriloga_analysis(2).unwrap();
    circuit.start_mixed_digital_execution().unwrap();
    let size = circuit.matrix_size();
    let entries: Vec<_> = (0..size).map(|i| (i, i, 1.0)).collect();
    let matrix = crate::solver::StaticMatrix::from_triplets(size, size, &entries).unwrap();
    circuit.link_indices(&matrix);
    (circuit, matrix, vec![0.0; size], resource)
}

fn stamp(
    circuit: &mut crate::CircuitData,
    matrix: &mut crate::solver::StaticMatrix,
    solution: &[f64],
) -> Result<Vec<f64>, SimulationError> {
    matrix.values_mut().fill(0.0);
    let mut rhs = vec![0.0; solution.len()];
    circuit.stamp_transient_linear_direct(matrix, &mut rhs);
    circuit.stamp_mixed_transient_trial(
        matrix,
        &mut rhs,
        0.0,
        0.0,
        solution,
        XspiceCompanionPolicy {
            coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(),
            xyce_one_step_order2: false,
        },
        true,
        false,
    )?;
    Ok(rhs)
}

#[test]
fn coupled_circuit_probe_retains_adc_gate_feedback_then_restores_all_owners_and_resources() {
    let (mut circuit, mut matrix, mut solution, resource) =
        fixture(crate::xspice::EvaluationPhase::CircuitTrial);
    let p = circuit.get_node_by_name("p").unwrap() - 1;
    let stimulus = circuit.get_node_by_name("stimulus").unwrap() - 1;
    let loaded = circuit.get_node_by_name("loaded").unwrap() - 1;
    let bad = circuit.get_node_by_name("bad").unwrap() - 1;
    let before = circuit.mixed_signal_hosts[0]
        .read_digital("sampled")
        .unwrap();
    for (input, expected_current) in [(0.0, -1e-3), (1.0, 0.0), (0.0, -1e-3), (1.0, 0.0)] {
        solution[stimulus] = input;
        let captures = resource.captures.load(Ordering::Relaxed);
        let rhs = stamp(&mut circuit, &mut matrix, &solution).unwrap();
        assert!(
            (rhs[p] - expected_current).abs() < 1e-15,
            "input={input}, rhs={:?}",
            rhs
        );
        let index = matrix.get_index(loaded, loaded).unwrap().offset();
        let conductance = matrix.values_mut()[index];
        assert!(
            (conductance - 0.051).abs() < 1e-15,
            "native 1k load and 20 ohm D/A remain physical"
        );
        assert!((rhs[loaded] / conductance - 1000.0 / 1020.0).abs() < 1e-14);
        assert_eq!(*resource.value.lock().unwrap(), 0);
        assert_eq!(
            resource.captures.load(Ordering::Relaxed),
            captures + 1,
            "one undo image covers all XSPICE Active waves"
        );
        assert_eq!(
            circuit.mixed_signal_hosts[0]
                .read_digital("sampled")
                .unwrap(),
            before
        );
        assert!(circuit.xspice_event_values.digital_drivers.is_empty());
        assert!(circuit.xspice_event_queue.next_event_time().is_none());
        assert!(circuit.mixed_digital_coordinator.is_some());
    }
    solution[bad] = 2.0;
    assert!(
        stamp(&mut circuit, &mut matrix, &solution).is_err(),
        "late analog stamping refuses the candidate"
    );
    assert_eq!(*resource.value.lock().unwrap(), 0);
    assert_eq!(circuit.mixed_signal_hosts.len(), 2);
    assert_eq!(
        circuit.mixed_signal_hosts[0]
            .read_digital("sampled")
            .unwrap(),
        before
    );
    assert!(circuit.xspice_event_values.digital_drivers.is_empty());
    solution[bad] = 0.0;
    let rhs = stamp(&mut circuit, &mut matrix, &solution).unwrap();
    assert!(rhs[p].abs() < 1e-15);
}

#[test]
fn coupled_acceptance_refusal_restores_shared_drivers_models_and_resources_before_retry() {
    let (mut circuit, _, mut solution, resource) =
        fixture(crate::xspice::EvaluationPhase::AcceptedStep);
    let stimulus = circuit.get_node_by_name("stimulus").unwrap() - 1;
    solution[stimulus] = 1.0;
    let before = circuit.mixed_signal_hosts[0]
        .read_digital("sampled")
        .unwrap();
    let coeff = crate::numerics::integration::CompanionCoefficients::backward_euler();
    for refuse in [true, true, false] {
        let rollback = circuit.capture_xspice_acceptance();
        let captures = resource.captures.load(Ordering::Relaxed);
        let mut projected = Vec::new();
        let result = circuit.accept_mixed_transient_with(
            0.0,
            0.0,
            &mut solution,
            XspiceCompanionPolicy {
                coefficients: &coeff,
                xyce_one_step_order2: false,
            },
            true,
            false,
            Some(rollback.resources()),
            true,
            &mut projected,
            |_, _, _, history| {
                assert!(history.unwrap().iter().all(|value| value.is_finite()));
                assert!(
                    *resource.value.lock().unwrap() > 0,
                    "external candidate must have advanced before native preparation"
                );
                if refuse {
                    Err(SimulationError::Circuit(
                        "injected late native preparation refusal".into(),
                    ))
                } else {
                    Ok(())
                }
            },
        );
        assert_eq!(resource.captures.load(Ordering::Relaxed), captures + 1);
        assert_eq!(circuit.mixed_signal_hosts.len(), 2);
        assert!(circuit.mixed_digital_coordinator.is_some());
        if refuse {
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("late native preparation refusal")
            );
            for (index, value) in projected.into_iter().rev() {
                solution[index] = value;
            }
            circuit.restore_xspice_acceptance(rollback).unwrap();
            assert_eq!(*resource.value.lock().unwrap(), 0);
            assert_eq!(
                circuit.mixed_signal_hosts[0]
                    .read_digital("sampled")
                    .unwrap(),
                before
            );
            assert!(circuit.xspice_event_values.digital_drivers.is_empty());
            assert!(circuit.xspice_event_queue.next_event_time().is_none());
        } else {
            result.unwrap();
            rollback.resources().commit();
            assert!(*resource.value.lock().unwrap() > 0);
            assert_eq!(
                circuit.mixed_signal_hosts[0]
                    .read_digital("sampled")
                    .unwrap(),
                "0"
            );
            let mut snapshot = Vec::new();
            circuit.fill_xspice_digital_snapshot(&mut snapshot);
            assert!(
                snapshot.windows(2).all(|pair| pair[0].0 < pair[1].0),
                "shared nodes have exactly one snapshot owner: {snapshot:?}"
            );
            let bus = circuit.get_node_by_name("bus").unwrap();
            assert_eq!(
                snapshot
                    .iter()
                    .find(|(node, _)| *node == bus)
                    .unwrap()
                    .1
                    .state,
                crate::xspice::DigitalState::One
            );
        }
    }
}

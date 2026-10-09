use super::*;
use crate::circuit::XspiceCompanionPolicy;
use crate::xspice::{PortConnection, XspiceInstance};
use std::sync::Arc;

fn fixture(started: bool) -> (CircuitData, Vec<f64>) {
    let engine = crate::Engine::new(crate::SimulationConfig::default());
    let deck = crate::Netlist::parse("mixed restart\nRp p 0 1k\n.end\n").unwrap();
    let mut circuit = engine.build_circuit(&deck).unwrap();
    let p = circuit.get_node_by_name("p").unwrap();
    let q = circuit.get_or_create_node("q");
    let back = circuit.get_or_create_node("back");
    let sources = [
        (
            "ticker",
            r#"module ticker(p,q,back);
            inout p; electrical p; output q; reg q; input back; wire back;
            integer seen; initial begin seen=0; q=0; #3 q=1; #7 q=0; end
            always @(posedge back) seen<=seen+1;
            analog I(p)<+V(p)/1000.0+idt(V(p),0.0)*1e-6+ddt(1e-12*V(p));
            endmodule"#,
        ),
        (
            "observer",
            r#"module observer(p,back);
            inout p; electrical p; input back; wire back;
            integer seen; initial seen=0; always @(negedge back) seen<=seen+1;
            analog I(p)<+V(p)*1e-6+seen*1e-9;
            endmodule"#,
        ),
    ];
    for (name, source) in sources {
        let compiled = rspice_veriloga::VerilogACompiler::new(rspice_veriloga::CompilerOptions {
            enable_ams: true,
            ..Default::default()
        })
        .compile_runtime(source, None)
        .unwrap();
        let mut host = MixedSignalHost::from_compiled(
            name,
            Arc::new(compiled.model),
            &compiled.canonical_ir,
            &[p],
            Default::default(),
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
        if name == "ticker" {
            host.add_dac_bridge("q", 0, (q, 0), 0.0, 3.3, 20.0).unwrap();
        }
        host.add_adc_bridge("back", 0, (back, 0), 0.4, 0.6).unwrap();
        circuit.add_mixed_signal_host(host).unwrap();
    }
    let mut instance = XspiceInstance::new(
        "ainv",
        Arc::new(crate::xspice::models::DigitalInverter),
        vec![PortConnection::Digital(q), PortConnection::Digital(back)],
        &[("rise_delay".into(), 4e-9), ("fall_delay".into(), 4e-9)],
        &[],
        &[],
        &[],
    )
    .unwrap();
    instance.init().unwrap();
    circuit.add_xspice_instance(instance);
    circuit
        .finalize_mixed_digital(
            &[q, back].into_iter().collect(),
            &rspice_veriloga::NoPipelineControl,
        )
        .unwrap();
    if started {
        circuit.begin_veriloga_analysis(2).unwrap();
        circuit.start_mixed_digital_execution().unwrap();
    }
    let size = circuit.matrix_size();
    let entries: Vec<_> = (0..size).map(|i| (i, i, 1.0)).collect();
    let matrix = crate::solver::StaticMatrix::from_triplets(size, size, &entries).unwrap();
    circuit.link_indices(&matrix);
    let mut solution = vec![0.0; size];
    solution[p - 1] = 0.75;
    if started {
        step(&mut circuit, &mut solution, 0.0, 0.0);
    }
    (circuit, solution)
}

fn step(circuit: &mut CircuitData, solution: &mut [f64], time: f64, dt: f64) {
    let rollback = circuit.capture_xspice_acceptance();
    circuit
        .accept_mixed_transient_with(
            &crate::abort_signal::NoAbort,
            time,
            dt,
            solution,
            XspiceCompanionPolicy {
                coefficients: &crate::numerics::integration::CompanionCoefficients::backward_euler(
                ),
                xyce_one_step_order2: false,
            },
            time == 0.0,
            false,
            Some(rollback.resources()),
            false,
            &mut Vec::new(),
            |circuit, _, _, _| {
                for instance in &mut circuit.xspice_instances {
                    instance.make_mut().accept_timestep();
                }
                Ok(())
            },
        )
        .unwrap();
    rollback.resources().commit();
}

fn next_hdl_time(circuit: &CircuitData) -> f64 {
    circuit
        .scheduler
        .mixed_digital_coordinator
        .as_ref()
        .unwrap()
        .next_event_time()
        .unwrap()
        .unwrap()
        .1
}

#[test]
fn mixed_circuit_checkpoint_restores_both_event_lanes_and_all_participants_together() {
    let (mut continuous, mut solution) = fixture(true);
    let cut = next_hdl_time(&continuous);
    step(&mut continuous, &mut solution, cut, cut);
    assert!(
        continuous
            .scheduler
            .xspice_event_queue
            .next_event_time()
            .is_some()
    );
    let saved = continuous
        .mixed_runtime_checkpoint(cut, Default::default())
        .unwrap();
    let bytes = serde_json::to_vec(&saved).unwrap();
    let transported = MixedCircuitCheckpoint::decode(&bytes, Default::default()).unwrap();
    let (mut resumed, _) = fixture(true);
    let origin = resumed
        .mixed_runtime_checkpoint(0.0, Default::default())
        .unwrap();
    let prepared = transported
        .restore(&resumed, cut, Default::default())
        .unwrap();
    assert_eq!(
        origin,
        resumed
            .mixed_runtime_checkpoint(0.0, Default::default())
            .unwrap()
    );
    prepared.install(&mut resumed);
    assert_eq!(
        saved,
        resumed
            .mixed_runtime_checkpoint(cut, Default::default())
            .unwrap()
    );
    // Public resume rebuilds an elaborated design without executing initial
    // blocks. Restore the accepted state directly into that receiving design.
    let (mut unstarted, _) = fixture(false);
    transported
        .restore(&unstarted, cut, Default::default())
        .unwrap()
        .install(&mut unstarted);
    assert_eq!(
        saved,
        unstarted
            .mixed_runtime_checkpoint(cut, Default::default())
            .unwrap()
    );
    let mut resumed_solution = solution.clone();
    let mut previous = cut;
    while previous < 16e-9 {
        let mut time = 16e-9_f64;
        if let Some(at) = continuous.scheduler.xspice_event_queue.next_event_time() {
            time = time.min(at);
        }
        if let Some((_, at)) = continuous
            .scheduler
            .mixed_digital_coordinator
            .as_ref()
            .unwrap()
            .next_event_time()
            .unwrap()
        {
            time = time.min(at);
        }
        assert!(time > previous);
        step(&mut continuous, &mut solution, time, time - previous);
        step(&mut resumed, &mut resumed_solution, time, time - previous);
        assert_eq!(solution, resumed_solution);
        assert_eq!(
            continuous
                .mixed_runtime_checkpoint(time, Default::default())
                .unwrap(),
            resumed
                .mixed_runtime_checkpoint(time, Default::default())
                .unwrap(),
        );
        previous = time;
    }
}

#[test]
fn mixed_circuit_checkpoint_rejects_partial_images_budgets_and_failed_owners_atomically() {
    let (mut source, mut solution) = fixture(true);
    let cut = next_hdl_time(&source);
    step(&mut source, &mut solution, cut, cut);
    let saved = source
        .mixed_runtime_checkpoint(cut, Default::default())
        .unwrap();
    let (mut receiver, _) = fixture(true);
    let before = receiver
        .mixed_runtime_checkpoint(0.0, Default::default())
        .unwrap();
    let mut lent = receiver.clone();
    lent.scheduler.mixed_digital_coordinator = None;
    lent.mixed_signal_hosts.clear();
    assert!(
        lent.validate_mixed_checkpoint_idle()
            .unwrap_err()
            .contains("open trial")
    );
    let mut invalid = Vec::new();
    let mut bad = saved.clone();
    bad.version += 1;
    invalid.push(bad);
    let mut bad = saved.clone();
    bad.matrix_size += 1;
    invalid.push(bad);
    let mut bad = saved.clone();
    bad.participants.pop();
    invalid.push(bad);
    let mut bad = saved.clone();
    bad.instances.clear();
    invalid.push(bad);
    let mut bad = saved.clone();
    bad.coordinator = None;
    invalid.push(bad);
    // A late participant failure must not install the already rebuilt owner.
    let mut json = serde_json::to_value(&saved).unwrap();
    json["participants"][1]["analog_identity"][0] = "different instance".into();
    invalid.push(serde_json::from_value(json).unwrap());
    for bad in invalid {
        assert!(bad.restore(&receiver, cut, Default::default()).is_err());
        assert_eq!(
            before,
            receiver
                .mixed_runtime_checkpoint(0.0, Default::default())
                .unwrap()
        );
    }
    assert!(saved.restore(&receiver, 4e-9, Default::default()).is_err());
    let bytes = serde_json::to_vec(&saved).unwrap();
    let limits = MixedCheckpointLimits {
        max_bytes: bytes.len() - 1,
        ..Default::default()
    };
    assert!(MixedCircuitCheckpoint::decode(&bytes, limits).is_err());
    assert!(saved.restore(&receiver, cut, limits).is_err());
    assert!(source.mixed_runtime_checkpoint(cut, limits).is_err());
    let limits = MixedCheckpointLimits {
        max_instances: 2,
        ..Default::default()
    };
    assert!(saved.restore(&receiver, cut, limits).is_err());
    assert!(source.mixed_runtime_checkpoint(cut, limits).is_err());
    receiver.xspice_evaluation_error = Some("earlier failure".into());
    assert!(saved.restore(&receiver, cut, Default::default()).is_err());
    assert!(
        receiver
            .mixed_runtime_checkpoint(0.0, Default::default())
            .is_err()
    );
}

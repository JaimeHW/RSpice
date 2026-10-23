//! A project Verilog-A runtime compiled under a deck's min:typ:max selection
//! crosses the simulation worker with it, and the worker runs it only for a
//! deck keeping the same values.

use super::*;
use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;

/// A design whose conductance is a min:typ:max expression: 1, 2 or 3 mS.
const HELD: &str = "module held(p, n); inout p, n; electrical p, n;\n\
                    parameter real g = 1.0e-3:2.0e-3:3.0e-3;\n\
                    analog I(p, n) <+ g * V(p, n);\nendmodule\n";

/// The worker's transient request for `held` sealed under `min`, in a deck
/// selecting `options`.
fn request(options: &str) -> WorkerRequest {
    let project_id = rspice_vocabulary::product::ProjectId::new();
    let bundle = rspice_design::project_sources::ProjectSourceBundle::try_new(
        rspice_design::project_sources::ProjectSourceOwner::code_workspace(
            rspice_design::project_sources::ProjectSourceLanguage::VerilogA,
        ),
        rspice_design::project_sources::ProjectSourceLanguage::VerilogA,
        "held.va",
        HELD,
        [],
        [],
    )
    .unwrap();
    let (receipt, _) = crate::project_veriloga::receipt::compile_project_bundle_receipt_against(
        project_id,
        &bundle,
        Some("held"),
        None,
        DigitalDelaySelect::Min,
    )
    .unwrap();
    let runtime = receipt.prepare_runtime(project_id, &bundle).unwrap();
    assert_eq!(runtime.delay_select(), DigitalDelaySelect::Min);
    let directive = crate::netlist_preparation::project_veriloga_directive(
        runtime.source_key(),
        runtime.module_name(),
    );
    WorkerRequest {
        measurement_references: Default::default(),
        data_files: Default::default(),
        id: 41,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::Transient {
            stop_time: 2e-9,
            step_time: 0.2e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        })),
        netlist: format!(
            "* held under a selection\n{options}{directive}\n\
             v1 in 0 1\nr1 in p 1k\nx1 p 0 held\n.end\n"
        ),
        source_path: None,
        project_veriloga_runtimes: crate::veriloga::PreparedVerilogARuntimeSet::try_new(vec![
            runtime,
        ])
        .unwrap(),
        dependencies: Default::default(),
        environment: None,
        stream_transient_samples: false,
        execution_limits: rspice_core::ResourceLimits::default(),
    }
}

/// `request` carried through the worker's transport and run there.
fn through_worker(request: WorkerRequest) -> Result<SimulationResult, String> {
    let mut transport = WorkerRequestTransport::from_request(request.clone()).unwrap();
    let metadata = serde_json::to_vec(&transport.request).unwrap();
    transport.request = serde_json::from_slice(&metadata).unwrap();
    let restored = transport.into_request().unwrap();
    assert_eq!(restored, request);
    let runtime = restored
        .project_veriloga_runtimes
        .device_runtimes()
        .next()
        .unwrap();
    assert_eq!(runtime.delay_select(), DigitalDelaySelect::Min);
    runtime.validate().unwrap();
    worker_response_from_request(restored)
        .into_result()
        .map_err(|error| error.to_string())
}

/// The minimum conductance, 1 mS, against 1 kOhm halves the drive: the
/// worker runs the minimum artifact for a minimum deck, and refuses it for a
/// typical deck by naming both selections.
#[test]
fn the_worker_runs_a_minimum_artifact_only_for_a_minimum_deck() {
    let result = through_worker(request(".options hdl_delay_select=min\n"))
        .expect("the minimum deck runs its artifact");
    let SimulationResult::Transient { waveforms, .. } = result else {
        panic!("transient result required")
    };
    let (_, p) = waveforms
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("p"))
        .unwrap_or_else(|| panic!("p voltage waveform missing from {:?}", waveforms.keys()));
    assert!(!p.y_values.is_empty());
    assert!(
        p.y_values.iter().all(|volts| (volts - 0.5).abs() < 1e-9),
        "{:?}",
        p.y_values
    );

    let refused = through_worker(request("")).expect_err("a typical deck refuses it");
    for expected in ["`hdl_delay_select=min`", "`hdl_delay_select=typ`"] {
        assert!(refused.contains(expected), "{expected}: {refused}");
    }
}

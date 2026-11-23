//! A browser run's tasks are analyses of the one run: the run's record of the
//! files its analyses wrote crosses to the worker with each request and back
//! with each answer, so a later task appends where an earlier one wrote, as
//! on the desktop (LRM 2.4 section 9.5.1.1).

use super::*;
use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;

/// A design that writes one line to `log.txt`, opened `"w"`, in each
/// analysis it takes part in.
const LOGGER: &str = "module logger(p); inout p; electrical p; integer fd;\n\
                      analog begin\n@(initial_step) begin fd = $fopen(\"log.txt\", \"w\");\n\
                      $fdisplay(fd, \"analysis\"); end\nI(p) <+ V(p) / 1k;\nend\nendmodule\n";

/// The worker's transient request for `logger`, its run directory
/// `directory`, as an analysis of the run whose record is `files`.
fn request(directory: &std::path::Path, files: rspice_core::config::HdlRunFiles) -> WorkerRequest {
    let project_id = rspice_vocabulary::product::ProjectId::new();
    let bundle = rspice_design::project_sources::ProjectSourceBundle::try_new(
        rspice_design::project_sources::ProjectSourceOwner::ProjectWide,
        "logger.va",
        LOGGER,
        [],
        [],
    )
    .unwrap();
    let (receipt, _) = crate::project_veriloga::receipt::compile_project_bundle_receipt_against(
        project_id,
        &bundle,
        Some("logger"),
        None,
        DigitalDelaySelect::Typ,
    )
    .unwrap();
    let runtime = receipt.prepare_runtime(project_id, &bundle).unwrap();
    let directive = crate::netlist_preparation::project_veriloga_directive(
        runtime.source_key(),
        runtime.module_name(),
    );
    WorkerRequest {
        measurement_references: Default::default(),
        data_files: Default::default(),
        id: 7,
        request: WorkerSimulationRequest::Config(Box::new(WorkerAnalysisConfig::Transient {
            stop_time: 1e-9,
            step_time: 0.1e-9,
            start_time: 0.0,
            max_timestep: None,
            uic: false,
        })),
        netlist: format!(
            "* logger\n.options hdl_rundir=\"{}\"\n{directive}\nv1 p 0 1\nx1 p logger\n.end\n",
            directory.display()
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
        transient_states: Default::default(),
        hdl_run_files: Default::default(),
    }
    .with_hdl_run_files(files)
}

/// `request` across the worker's transport, run there, and its answer back
/// across: the record the answer carries.
fn through_worker(request: WorkerRequest) -> rspice_core::config::HdlRunFiles {
    let mut transport = WorkerRequestTransport::from_request(request.clone()).unwrap();
    transport.request = serde_json::from_slice(&serde_json::to_vec(&transport.request).unwrap())
        .expect("the request metadata reads back");
    let restored = transport.into_request().unwrap();
    assert_eq!(restored, request);
    let mut answer = WorkerResponseTransport::from_response_with_backends(
        worker_response_from_request(restored),
        None,
    )
    .unwrap();
    answer.response = serde_json::from_slice(&serde_json::to_vec(&answer.response).unwrap())
        .expect("the response metadata reads back");
    let (response, _) = answer.into_response_and_backends().unwrap();
    response
        .hdl_run_files
        .clone()
        .expect("the answer carries the record")
}

/// Two tasks of one run, each its own worker request: the runner takes back
/// the record each answer carries, so the second appends to what the first
/// wrote; a new run's record starts the file over.
#[test]
fn a_runs_worker_tasks_append_where_its_earlier_tasks_wrote() {
    let directory = std::env::temp_dir().join(format!(
        "rspice-worker-hdl-run-files-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    let read = || std::fs::read_to_string(directory.join("log.txt")).unwrap();
    let run = rspice_core::config::HdlRunFiles::default();
    for _ in 0..2 {
        let returned = through_worker(request(&directory, run.clone()));
        assert_ne!(returned, run, "the worker's analysis advanced its copy");
        run.adopt(&returned);
    }
    assert_eq!(read(), "analysis\nanalysis\n");
    through_worker(request(&directory, Default::default()));
    assert_eq!(read(), "analysis\n");
}

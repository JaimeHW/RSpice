//! Background project compilation and typed native/browser completions.
//!
//! Hosts select the exact source and check publication currentness. This service
//! owns compilation, bounded worker transport, and structured diagnostic outcomes.

use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::{ExternalCell, RuntimeCompileReport};
use rspice_vocabulary::diagnostics::{DiagnosticSeverity, SourceDiagnostic};
use std::sync::{Arc, mpsc};

#[cfg(any(target_arch = "wasm32", test))]
mod worker;
#[cfg(target_arch = "wasm32")]
pub use worker::{cancel as cancel_compile, run_worker_request_value};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum VerilogACompileOutcome {
    Success(Box<rspice_veriloga::RuntimeCompileReport>),
    Failure(Vec<SourceDiagnostic>),
}

/// Compile `selected_module` of `bundle` in-process, offering the design
/// `cells` as leaves: those of the netlist the project's run builds. A name
/// neither the bundle nor `cells` defines is still an undefined module.
pub fn compile_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
    cells: Option<&[ExternalCell]>,
) -> VerilogACompileOutcome {
    project_compile_outcome(
        bundle,
        super::compile_project_bundle_source_against(bundle, selected_module, cells),
    )
}

fn project_compile_outcome(
    bundle: &ProjectSourceBundle,
    outcome: Result<Box<RuntimeCompileReport>, super::ProjectVerilogACompileError>,
) -> VerilogACompileOutcome {
    match super::diagnostics::project_compile_outcome(bundle, outcome) {
        Ok(report) => VerilogACompileOutcome::Success(report),
        Err(diagnostics) => VerilogACompileOutcome::Failure(
            diagnostics
                .into_iter()
                .map(SourceDiagnostic::from)
                .collect(),
        ),
    }
}

pub fn transport_failure_outcome(message: String) -> VerilogACompileOutcome {
    VerilogACompileOutcome::Failure(vec![SourceDiagnostic::current(
        "rspice.veriloga.browser-worker",
        "VA-WORKER-TRANSPORT",
        DiagnosticSeverity::Error,
        "Browser compiler worker failed",
        message,
        None,
        None,
        None,
        None,
        None,
    )])
}

/// Compile as [`compile_source`] off the editor's thread: on a native thread,
/// or in the browser's compile worker, which receives `cells` with the bundle.
pub fn start_compile(
    bundle: ProjectSourceBundle,
    selected_module: Option<String>,
    cells: Option<Arc<[ExternalCell]>>,
    sender: mpsc::Sender<VerilogACompileOutcome>,
    wake: impl Fn() + Send + Sync + 'static,
) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            let outcome = compile_source(&bundle, selected_module.as_deref(), cells.as_deref());
            let _ = sender.send(outcome);
            wake();
        });
        Ok(())
    }
    #[cfg(target_arch = "wasm32")]
    worker::start(
        &bundle,
        selected_module.as_deref(),
        cells.as_deref(),
        sender,
        std::rc::Rc::new(wake),
    )
}

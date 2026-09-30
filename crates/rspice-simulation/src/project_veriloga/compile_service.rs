//! Background project compilation and typed native/browser completions.
//!
//! Hosts select the exact source and check publication currentness. This service
//! owns compilation, bounded worker transport, and structured diagnostic outcomes.

use rspice_app_types::diagnostics::{DiagnosticSeverity, SourceDiagnostic};
use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::RuntimeCompileReport;
use std::sync::mpsc;

#[cfg(any(target_arch = "wasm32", test))]
mod worker;
#[cfg(target_arch = "wasm32")]
pub use worker::{cancel as cancel_compile, run_worker_request_value};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum VerilogACompileOutcome {
    Success(Box<rspice_veriloga::RuntimeCompileReport>),
    Failure(Vec<SourceDiagnostic>),
}

pub fn compile_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
) -> VerilogACompileOutcome {
    project_compile_outcome(
        bundle,
        super::compile_project_bundle_source(bundle, selected_module),
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

pub fn start_compile(
    bundle: ProjectSourceBundle,
    selected_module: Option<String>,
    sender: mpsc::Sender<VerilogACompileOutcome>,
    wake: impl Fn() + Send + Sync + 'static,
) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            let outcome = compile_source(&bundle, selected_module.as_deref());
            let _ = sender.send(outcome);
            wake();
        });
        Ok(())
    }
    #[cfg(target_arch = "wasm32")]
    worker::start(
        &bundle,
        selected_module.as_deref(),
        sender,
        std::rc::Rc::new(wake),
    )
}

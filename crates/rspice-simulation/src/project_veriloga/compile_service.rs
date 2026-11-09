//! Background project compilation and typed native/browser completions.
//!
//! Hosts select the exact source and check publication currentness. This service
//! owns compilation, bounded worker transport, and structured diagnostic outcomes.

use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
use rspice_veriloga::{ExternalCell, PipelineControl, RuntimeCompileReport};
use rspice_vocabulary::diagnostics::{DiagnosticSeverity, SourceDiagnostic};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};

#[cfg(any(target_arch = "wasm32", test))]
mod worker;
#[cfg(target_arch = "wasm32")]
pub use worker::run_worker_request_value;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum VerilogACompileOutcome {
    Success(Box<rspice_veriloga::RuntimeCompileReport>),
    Failure(Vec<SourceDiagnostic>),
}

/// Stops one compile running off the editor's thread: a native compile at the
/// compiler's next step, the browser's by ending its worker.
#[derive(Debug, Clone, Default)]
pub struct CompileCancellation(Arc<AtomicBool>);

impl CompileCancellation {
    pub fn cancel(&self) {
        self.0.store(true, Ordering::Relaxed);
        #[cfg(target_arch = "wasm32")]
        worker::cancel();
    }
}

impl PipelineControl for CompileCancellation {
    fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Compile `selected_module` of `bundle` in-process, offering the design
/// `cells` as leaves and keeping the `delay_select` value of each
/// min:typ:max expression: those of the netlist the project's run builds. A
/// name neither the bundle nor `cells` defines is still an undefined module.
pub fn compile_source(
    bundle: &ProjectSourceBundle,
    selected_module: Option<&str>,
    cells: Option<&[ExternalCell]>,
    delay_select: DigitalDelaySelect,
) -> VerilogACompileOutcome {
    project_compile_outcome(
        bundle,
        super::compile_project_bundle_source_against(bundle, selected_module, cells, delay_select),
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
/// or in the browser's compile worker, which receives `cells` and
/// `delay_select` with the bundle.
pub fn start_compile(
    bundle: ProjectSourceBundle,
    selected_module: Option<String>,
    cells: Option<Arc<[ExternalCell]>>,
    delay_select: DigitalDelaySelect,
    cancellation: CompileCancellation,
    sender: mpsc::Sender<VerilogACompileOutcome>,
    wake: impl Fn() + Send + Sync + 'static,
) -> Result<(), String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::thread::spawn(move || {
            let outcome = project_compile_outcome(
                &bundle,
                super::compile_project_bundle_source_with_control(
                    &bundle,
                    selected_module.as_deref(),
                    cells.as_deref(),
                    delay_select,
                    &cancellation,
                ),
            );
            let _ = sender.send(outcome);
            wake();
        });
        Ok(())
    }
    #[cfg(target_arch = "wasm32")]
    {
        // The worker is ended from the outside; nothing inside it listens.
        let _ = cancellation;
        worker::start(
            &bundle,
            selected_module.as_deref(),
            cells.as_deref(),
            delay_select,
            sender,
            std::rc::Rc::new(wake),
        )
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rspice_design::project_sources::{ProjectSourceLanguage, ProjectSourceOwner};
    use rspice_design_model::cell_view::CellViewRef;

    fn module() -> ProjectSourceBundle {
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(CellViewRef::new("lib", "cell", "veriloga")),
            ProjectSourceLanguage::VerilogA,
            "m.va",
            "`include \"disciplines.vams\"\nmodule m(a); inout a; electrical a;\nanalog V(a) <+ 1.0;\nendmodule\n",
            [],
            [],
        )
        .expect("the bundle is valid")
    }

    fn compiled(cancellation: CompileCancellation) -> VerilogACompileOutcome {
        let (sender, receiver) = mpsc::channel();
        start_compile(
            module(),
            Some("m".to_owned()),
            None,
            DigitalDelaySelect::default(),
            cancellation,
            sender,
            || {},
        )
        .expect("a native compile always starts");
        receiver
            .recv_timeout(std::time::Duration::from_secs(120))
            .expect("the compile answers")
    }

    #[test]
    fn a_compile_left_alone_finishes() {
        assert!(matches!(
            compiled(CompileCancellation::default()),
            VerilogACompileOutcome::Success(_)
        ));
    }

    #[test]
    fn a_cancelled_compile_stops_short_of_a_result() {
        let cancellation = CompileCancellation::default();
        cancellation.cancel();
        assert!(matches!(
            compiled(cancellation),
            VerilogACompileOutcome::Failure(_)
        ));
    }
}

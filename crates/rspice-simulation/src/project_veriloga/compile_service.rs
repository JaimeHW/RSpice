//! Background project compilation and typed native/browser completions.
//!
//! Hosts select the exact source and check publication currentness. This service
//! owns compilation, bounded worker transport, and structured diagnostic outcomes.

use rspice_design::project_sources::ProjectSourceBundle;
use rspice_veriloga::canonical_ir::digital::DigitalDelaySelect;
use rspice_veriloga::{ExternalCell, PipelineControl, RuntimeCompileReport};
use rspice_vocabulary::diagnostics::{DiagnosticSeverity, SourceDiagnostic};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Weak, mpsc};

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
/// compiler's next step, or inside a constant function or loop it is
/// evaluating, the browser's by ending its worker.
///
/// The requester keeps it (and its clones) for as long as it wants the
/// result. A compile whose requester has dropped every clone (the project was
/// closed, or the compile was let go) stops as if cancelled: nothing is left
/// to read what it would find. A native compile sees that through its weak
/// hold; the browser's worker is ended when the last clone drops.
#[derive(Debug, Clone, Default)]
pub struct CompileCancellation {
    stopped: Arc<AtomicBool>,
    requester: Arc<Requester>,
}

/// The requester's interest in one compile. In the browser it names the
/// worker running the compile, so letting go of it ends that worker and no
/// later one.
#[derive(Debug, Default)]
struct Requester {
    /// The browser worker's request id, once it has started; 0 before.
    #[cfg(target_arch = "wasm32")]
    worker: std::sync::atomic::AtomicU32,
}

#[cfg(target_arch = "wasm32")]
impl Requester {
    /// The worker running the compile, once it has started.
    fn worker(&self) -> Option<u32> {
        Some(self.worker.load(Ordering::Relaxed)).filter(|&id| id != 0)
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for Requester {
    fn drop(&mut self) {
        if let Some(id) = self.worker() {
            worker::finish(Some(id));
        }
    }
}

impl CompileCancellation {
    pub fn cancel(&self) {
        self.stopped.store(true, Ordering::Relaxed);
        // The worker this compile started, or, before it has one, whichever
        // is running.
        #[cfg(target_arch = "wasm32")]
        worker::finish(self.requester.worker());
    }

    /// What the compile itself holds: the cancellation, without keeping the
    /// requester's interest in the result alive.
    pub fn job(&self) -> CompileJob {
        CompileJob {
            stopped: Arc::clone(&self.stopped),
            requester: Arc::downgrade(&self.requester),
        }
    }
}

impl PipelineControl for CompileCancellation {
    fn is_cancelled(&self) -> bool {
        self.stopped.load(Ordering::Relaxed)
    }
}

/// A compile's own hold on its [`CompileCancellation`]: cancelled once the
/// requester cancels or keeps no clone of it.
#[derive(Debug)]
pub struct CompileJob {
    stopped: Arc<AtomicBool>,
    requester: Weak<Requester>,
}

impl CompileJob {
    /// Run `body`, the job's work, with this job as the thread's loop-abort
    /// hook, so a constant function or loop it evaluates (a generate bound, a
    /// parameter's value) stops with the job instead of holding the thread.
    pub fn run<R>(&self, body: impl FnOnce() -> R) -> R {
        rspice_core::engine::with_loop_abort(self, body)
    }
}

impl PipelineControl for CompileJob {
    fn is_cancelled(&self) -> bool {
        self.stopped.load(Ordering::Relaxed) || self.requester.strong_count() == 0
    }
}

impl rspice_core::abort_signal::AbortSignal for CompileJob {
    fn is_aborted(&self) -> bool {
        PipelineControl::is_cancelled(self)
    }
}

/// Compile `selected_module` of `bundle` in-process, offering the design
/// `cells` as leaves and keeping the `delay_select` value of each
/// min:typ:max expression: those of the netlist the project's run builds. A
/// name neither the bundle nor `cells` defines is still an undefined module.
#[cfg(any(test, feature = "test-support"))]
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
        let job = cancellation.job();
        drop(cancellation);
        std::thread::spawn(move || {
            let outcome = job.run(|| {
                project_compile_outcome(
                    &bundle,
                    super::compile_project_bundle_source_with_control(
                        &bundle,
                        selected_module.as_deref(),
                        cells.as_deref(),
                        delay_select,
                        &job,
                    ),
                )
            });
            let _ = sender.send(outcome);
            wake();
        });
        Ok(())
    }
    #[cfg(target_arch = "wasm32")]
    {
        // The worker is ended from the outside, by a cancel or by the
        // requester letting go; nothing inside it listens.
        let id = worker::start(
            &bundle,
            selected_module.as_deref(),
            cells.as_deref(),
            delay_select,
            sender,
            std::rc::Rc::new(wake),
        )?;
        cancellation.requester.worker.store(id, Ordering::Relaxed);
        Ok(())
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use rspice_design::project_sources::{ProjectSourceLanguage, ProjectSourceOwner};
    use rspice_design_model::cell_view::CellViewRef;

    fn bundle(source: &str) -> ProjectSourceBundle {
        ProjectSourceBundle::try_new(
            ProjectSourceOwner::cell_view(CellViewRef::new("lib", "cell", "veriloga")),
            ProjectSourceLanguage::VerilogA,
            "m.va",
            source,
            [],
            [],
        )
        .expect("the bundle is valid")
    }

    fn module() -> ProjectSourceBundle {
        bundle(
            "`include \"disciplines.vams\"\nmodule m(a); inout a; electrical a;\nanalog V(a) <+ 1.0;\nendmodule\n",
        )
    }

    /// A module whose generate loop is bounded by a constant function that
    /// never returns: its compile runs until it is stopped.
    fn spinning_module() -> ProjectSourceBundle {
        bundle(
            "`include \"disciplines.vams\"\n\
             module m(a); inout a; electrical a;\n\
             \x20 function integer spin(input integer n);\n\
             \x20   begin spin = n; while (spin > 0) spin = spin + 0; end\n\
             \x20 endfunction\n\
             \x20 localparam integer N = spin(1);\n\
             \x20 genvar i;\n\
             \x20 generate for (i = 0; i < N; i = i + 1) begin : g end endgenerate\n\
             \x20 analog V(a) <+ 1.0;\n\
             endmodule\n",
        )
    }

    fn start(
        bundle: ProjectSourceBundle,
        cancellation: CompileCancellation,
    ) -> mpsc::Receiver<VerilogACompileOutcome> {
        let (sender, receiver) = mpsc::channel();
        start_compile(
            bundle,
            Some("m".to_owned()),
            None,
            DigitalDelaySelect::default(),
            cancellation,
            sender,
            || {},
        )
        .expect("a native compile always starts");
        receiver
    }

    fn answer(receiver: &mpsc::Receiver<VerilogACompileOutcome>) -> VerilogACompileOutcome {
        receiver
            .recv_timeout(std::time::Duration::from_secs(120))
            .expect("the compile answers")
    }

    /// The compile's answer, its requester keeping `cancellation` meanwhile.
    fn compiled(cancellation: CompileCancellation) -> VerilogACompileOutcome {
        answer(&start(module(), cancellation.clone()))
    }

    #[test]
    fn a_compile_left_alone_finishes() {
        assert!(matches!(
            compiled(CompileCancellation::default()),
            VerilogACompileOutcome::Success(_)
        ));
    }

    /// A compile held in a constant function stops when it is cancelled, and
    /// when its requester lets go of it, as when the project closes.
    #[test]
    fn a_compile_held_in_a_constant_function_stops_with_its_job() {
        let cancellation = CompileCancellation::default();
        let cancelled = start(spinning_module(), cancellation.clone());
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert!(matches!(
            cancelled.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));
        cancellation.cancel();
        assert!(matches!(
            answer(&cancelled),
            VerilogACompileOutcome::Failure(_)
        ));

        let abandoned = start(spinning_module(), CompileCancellation::default());
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert!(matches!(
            answer(&abandoned),
            VerilogACompileOutcome::Failure(_)
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

//! Owned native hardcopy rendering outside the UI thread.
//!
//! This runtime deliberately permits one operation at a time. The immutable
//! plan and resolved source are moved into a named worker thread, while the UI
//! retains an exact digest-bound ticket. Cancellation is observed before and
//! after the renderer's non-interruptible calls; a late result is discarded
//! rather than being allowed to publish through a cancelled or stale dialog.

#![cfg(not(target_arch = "wasm32"))]

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, mpsc};

use crate::hardcopy::{HardcopyPlan, OutputFormat};
use crate::product::ContentDigest;
use crate::workbench::hardcopy_adapters::render::{
    HardcopyRenderer, HardcopySceneMetadata, RenderedHardcopyPublication, RenderedPrinterPages,
};
use crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument;

const NATIVE_EXECUTION_THREAD_NAME: &str = "rspice-hardcopy-render";
const CANCELLED_MESSAGE: &str = "Native hardcopy rendering was cancelled.";

/// The exact renderer entry point owned by a native execution ticket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExecutionOperation {
    Publication,
    NativePrinter { dpi: u16 },
}

/// Immutable identity retained by the UI while native work is in flight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ExecutionTicket {
    pub(crate) generation: u64,
    pub(crate) plan_digest: ContentDigest,
    pub(crate) source_digest: ContentDigest,
    pub(crate) operation: ExecutionOperation,
}

/// The owned renderer result. The payload is never returned without its exact
/// ticket in [`ExecutionCompletion`].
#[derive(Debug)]
pub(crate) enum ExecutionPayload {
    Publication(RenderedHardcopyPublication),
    NativePrinter(RenderedPrinterPages),
}

/// A successful worker result together with the identity that produced it.
#[derive(Debug)]
pub(crate) struct ExecutionCompletion {
    pub(crate) ticket: ExecutionTicket,
    pub(crate) payload: ExecutionPayload,
}

struct WorkerEnvelope {
    ticket: ExecutionTicket,
    result: Result<ExecutionPayload, String>,
}

struct ActiveExecution {
    ticket: ExecutionTicket,
    cancelled: Arc<AtomicBool>,
    receiver: mpsc::Receiver<WorkerEnvelope>,
}

static ACTIVE_EXECUTION: OnceLock<Mutex<Option<ActiveExecution>>> = OnceLock::new();

fn active_execution() -> &'static Mutex<Option<ActiveExecution>> {
    ACTIVE_EXECUTION.get_or_init(|| Mutex::new(None))
}

/// Start one owned native render operation.
///
/// The caller must retain the returned ticket and pass that exact value to
/// [`poll`]. A completed result remains owned by the runtime until polled, so a
/// second request cannot silently replace an unobserved artifact.
pub(crate) fn start(
    plan: Arc<HardcopyPlan>,
    source: Arc<ResolvedHardcopyDocument>,
    metadata: HardcopySceneMetadata,
    generation: u64,
    operation: ExecutionOperation,
    repaint: egui::Context,
) -> Result<ExecutionTicket, String> {
    validate_start(&plan, &source, generation, operation)?;

    let ticket = ExecutionTicket {
        generation,
        plan_digest: plan.content_digest(),
        source_digest: source.authority().content_digest(),
        operation,
    };
    let mut active = active_execution()
        .lock()
        .map_err(|_| "The native hardcopy execution owner is unavailable.".to_owned())?;
    ensure_execution_slot_available(&mut active)?;

    let cancelled = Arc::new(AtomicBool::new(false));
    let worker_cancelled = Arc::clone(&cancelled);
    let (sender, receiver) = mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name(NATIVE_EXECUTION_THREAD_NAME.to_owned())
        .spawn(move || {
            let result = catch_worker_panic(|| {
                execute_owned(&plan, &source, metadata, ticket, worker_cancelled.as_ref())
            });
            // The UI may have cancelled and released a completed operation.
            // Failing to deliver in that case is expected and must not panic.
            let _ = sender.send(WorkerEnvelope { ticket, result });
            repaint.request_repaint();
        })
        .map_err(|error| format!("Could not start native hardcopy rendering: {error}"))?;

    *active = Some(ActiveExecution {
        ticket,
        cancelled,
        receiver,
    });
    Ok(ticket)
}

/// Poll only the operation identified by `expected`.
///
/// A stale ticket cannot consume or release the active result. Channel
/// disconnection is terminal and releases the one-active slot with an error.
pub(crate) fn poll(expected: ExecutionTicket) -> Option<Result<ExecutionCompletion, String>> {
    let mut active = match active_execution().lock() {
        Ok(active) => active,
        Err(_) => {
            return Some(Err(
                "The native hardcopy execution owner is unavailable.".to_owned()
            ));
        }
    };
    let current = active.as_ref()?;
    if !tickets_match(expected, current.ticket) {
        return None;
    }

    match current.receiver.try_recv() {
        Ok(envelope) => {
            let was_cancelled = current.cancelled.load(Ordering::Acquire);
            active.take();
            if !tickets_match(expected, envelope.ticket) {
                return Some(Err(
                    "Native hardcopy rendering returned a stale completion identity.".to_owned(),
                ));
            }
            if was_cancelled {
                return Some(Err(CANCELLED_MESSAGE.to_owned()));
            }
            Some(envelope.result.map(|payload| ExecutionCompletion {
                ticket: envelope.ticket,
                payload,
            }))
        }
        Err(mpsc::TryRecvError::Empty) => None,
        Err(mpsc::TryRecvError::Disconnected) => {
            active.take();
            Some(Err(
                "Native hardcopy rendering disconnected before returning a result.".to_owned(),
            ))
        }
    }
}

/// Request cancellation of the active operation.
///
/// Publication and printer rendering currently have no page-boundary callback,
/// so their worker safely discards the result after the renderer returns. The
/// one-active slot remains owned until [`poll`] observes terminal completion.
pub(crate) fn cancel() {
    if let Ok(active) = active_execution().lock()
        && let Some(active) = active.as_ref()
    {
        active.cancelled.store(true, Ordering::Release);
    }
}

#[must_use]
pub(crate) fn is_active() -> bool {
    active_execution()
        .lock()
        .is_ok_and(|active| active.is_some())
}

fn validate_start(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    generation: u64,
    operation: ExecutionOperation,
) -> Result<(), String> {
    validate_generation(generation)?;
    if plan.source() != source.authority() {
        return Err(
            "The native hardcopy source authority does not match the immutable plan.".to_owned(),
        );
    }
    if plan.content_extent() != source.content_extent() {
        return Err(
            "The native hardcopy source extent does not match the immutable plan.".to_owned(),
        );
    }
    validate_operation(plan.setup().render().format(), operation)
}

fn validate_generation(generation: u64) -> Result<(), String> {
    if generation == 0 {
        return Err("Native hardcopy generation must be non-zero.".to_owned());
    }
    Ok(())
}

fn validate_operation(format: OutputFormat, operation: ExecutionOperation) -> Result<(), String> {
    match operation {
        ExecutionOperation::Publication if format == OutputFormat::NativePrinter => {
            Err("A native-printer plan must use native printer-page rendering.".to_owned())
        }
        ExecutionOperation::NativePrinter { .. } if format != OutputFormat::NativePrinter => {
            Err("Native printer-page rendering requires a native-printer plan.".to_owned())
        }
        ExecutionOperation::NativePrinter { dpi } if !(72..=9_600).contains(&dpi) => Err(format!(
            "Native printer resolution {dpi} DPI is outside the supported 72-9600 DPI range."
        )),
        _ => Ok(()),
    }
}

fn execute_owned(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    metadata: HardcopySceneMetadata,
    ticket: ExecutionTicket,
    cancelled: &AtomicBool,
) -> Result<ExecutionPayload, String> {
    if cancelled.load(Ordering::Acquire) {
        return Err(CANCELLED_MESSAGE.to_owned());
    }
    validate_worker_identity(plan, source, ticket)?;

    let payload = match ticket.operation {
        ExecutionOperation::Publication => {
            HardcopyRenderer::render_resolved(plan, source, metadata)
                .map(ExecutionPayload::Publication)
        }
        ExecutionOperation::NativePrinter { dpi } => {
            HardcopyRenderer::render_printer_pages_resolved(plan, source, metadata, dpi)
                .map(ExecutionPayload::NativePrinter)
        }
    }
    .map_err(|error| error.to_string())?;

    if cancelled.load(Ordering::Acquire) {
        return Err(CANCELLED_MESSAGE.to_owned());
    }
    validate_worker_identity(plan, source, ticket)?;
    Ok(payload)
}

fn validate_worker_identity(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    ticket: ExecutionTicket,
) -> Result<(), String> {
    validate_generation(ticket.generation).map_err(|_| {
        "Native hardcopy rendering rejected a stale plan or source identity.".to_owned()
    })?;
    if ticket.plan_digest != plan.content_digest()
        || ticket.source_digest != source.authority().content_digest()
        || plan.source() != source.authority()
        || plan.content_extent() != source.content_extent()
    {
        return Err(
            "Native hardcopy rendering rejected a stale plan or source identity.".to_owned(),
        );
    }
    Ok(())
}

fn tickets_match(expected: ExecutionTicket, actual: ExecutionTicket) -> bool {
    expected == actual
}

fn catch_worker_panic(
    execute: impl FnOnce() -> Result<ExecutionPayload, String>,
) -> Result<ExecutionPayload, String> {
    catch_unwind(AssertUnwindSafe(execute))
        .unwrap_or_else(|_| Err("Native hardcopy rendering terminated unexpectedly.".to_owned()))
}

fn ensure_execution_slot_available(active: &mut Option<ActiveExecution>) -> Result<(), String> {
    reap_finished_cancellation(active);
    if active.is_some() {
        return Err("A native hardcopy render operation is already active.".to_owned());
    }
    Ok(())
}

fn reap_finished_cancellation(active: &mut Option<ActiveExecution>) {
    let Some(current) = active.as_ref() else {
        return;
    };
    if !current.cancelled.load(Ordering::Acquire) {
        return;
    }
    match current.receiver.try_recv() {
        Ok(_) | Err(mpsc::TryRecvError::Disconnected) => {
            active.take();
        }
        Err(mpsc::TryRecvError::Empty) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_RUNTIME_SERIAL: Mutex<()> = Mutex::new(());

    struct RuntimeGuard {
        _serial: std::sync::MutexGuard<'static, ()>,
    }

    impl RuntimeGuard {
        fn acquire() -> Self {
            let serial = TEST_RUNTIME_SERIAL
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            reset_runtime();
            Self { _serial: serial }
        }
    }

    impl Drop for RuntimeGuard {
        fn drop(&mut self) {
            reset_runtime();
        }
    }

    fn digest(value: u8) -> ContentDigest {
        ContentDigest::from_bytes([value; 32])
    }

    fn ticket(generation: u64, operation: ExecutionOperation) -> ExecutionTicket {
        ExecutionTicket {
            generation,
            plan_digest: digest(0x11),
            source_digest: digest(0x22),
            operation,
        }
    }

    fn reset_runtime() {
        active_execution()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
    }

    fn install_active(
        active_ticket: ExecutionTicket,
    ) -> (Arc<AtomicBool>, mpsc::SyncSender<WorkerEnvelope>) {
        let cancelled = Arc::new(AtomicBool::new(false));
        let (sender, receiver) = mpsc::sync_channel(1);
        *active_execution()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActiveExecution {
            ticket: active_ticket,
            cancelled: Arc::clone(&cancelled),
            receiver,
        });
        (cancelled, sender)
    }

    fn envelope(completion_ticket: ExecutionTicket, error: &str) -> WorkerEnvelope {
        WorkerEnvelope {
            ticket: completion_ticket,
            result: Err(error.to_owned()),
        }
    }

    #[test]
    fn exact_ticket_guard_rejects_every_stale_dimension() {
        let expected = ticket(7, ExecutionOperation::Publication);
        assert!(tickets_match(expected, expected));

        let mut changed = expected;
        changed.generation = 8;
        assert!(!tickets_match(expected, changed));
        changed = expected;
        changed.plan_digest = digest(0x33);
        assert!(!tickets_match(expected, changed));
        changed = expected;
        changed.source_digest = digest(0x44);
        assert!(!tickets_match(expected, changed));
        changed = expected;
        changed.operation = ExecutionOperation::NativePrinter { dpi: 600 };
        assert!(!tickets_match(expected, changed));
    }

    #[test]
    fn printer_resolution_is_part_of_operation_identity() {
        assert!(!tickets_match(
            ticket(3, ExecutionOperation::NativePrinter { dpi: 300 }),
            ticket(3, ExecutionOperation::NativePrinter { dpi: 600 }),
        ));
    }

    #[test]
    fn one_active_owner_is_retained_until_terminal_observation() {
        let _guard = RuntimeGuard::acquire();
        let active_ticket = ticket(11, ExecutionOperation::Publication);
        let (_cancelled, sender) = install_active(active_ticket);
        sender
            .send(envelope(active_ticket, "unobserved completion"))
            .expect("active receiver must retain the completion");

        let mut active = active_execution()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(
            ensure_execution_slot_available(&mut active),
            Err("A native hardcopy render operation is already active.".to_owned())
        );
        assert_eq!(
            active.as_ref().map(|current| current.ticket),
            Some(active_ticket)
        );
    }

    #[test]
    fn stale_ticket_cannot_consume_exact_completion() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(21, ExecutionOperation::Publication);
        let (_cancelled, sender) = install_active(exact);
        sender
            .send(envelope(exact, "exact completion"))
            .expect("active receiver must accept the completion");

        let mut stale = exact;
        stale.generation += 1;
        assert!(poll(stale).is_none());
        assert!(is_active());

        let completion = poll(exact).expect("the exact ticket must observe completion");
        assert!(matches!(
            completion,
            Err(ref error) if error == "exact completion"
        ));
        assert!(!is_active());
    }

    #[test]
    fn cancellation_retains_ownership_then_discards_late_completion() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(31, ExecutionOperation::Publication);
        let (cancelled, sender) = install_active(exact);

        cancel();
        assert!(cancelled.load(Ordering::Acquire));
        assert!(is_active());
        sender
            .send(envelope(exact, "late renderer result"))
            .expect("active receiver must accept the late completion");

        let completion = poll(exact).expect("cancelled completion must be terminal");
        assert!(matches!(
            completion,
            Err(ref error) if error == CANCELLED_MESSAGE
        ));
        assert!(!is_active());
    }

    #[test]
    fn completed_cancelled_owner_is_reaped_before_a_new_claim() {
        let exact = ticket(36, ExecutionOperation::Publication);
        let cancelled = Arc::new(AtomicBool::new(true));
        let (sender, receiver) = mpsc::sync_channel(1);
        sender
            .send(envelope(exact, "late renderer result"))
            .expect("the local receiver must retain the late completion");
        let mut active = Some(ActiveExecution {
            ticket: exact,
            cancelled,
            receiver,
        });

        assert_eq!(ensure_execution_slot_available(&mut active), Ok(()));
        assert!(active.is_none());
    }

    #[test]
    fn disconnected_worker_is_terminal_and_releases_owner() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(41, ExecutionOperation::Publication);
        let (_cancelled, sender) = install_active(exact);
        drop(sender);

        let completion = poll(exact).expect("disconnection must be terminal");
        assert!(matches!(
            completion,
            Err(ref error)
                if error
                    == "Native hardcopy rendering disconnected before returning a result."
        ));
        assert!(!is_active());
    }

    #[test]
    fn worker_panic_is_caught_as_a_bounded_error() {
        let result = catch_worker_panic(|| -> Result<ExecutionPayload, String> {
            panic!("synthetic renderer panic")
        });
        assert!(matches!(
            result,
            Err(ref error) if error == "Native hardcopy rendering terminated unexpectedly."
        ));
    }

    #[test]
    fn generation_and_operation_contracts_fail_closed() {
        assert_eq!(
            validate_generation(0),
            Err("Native hardcopy generation must be non-zero.".to_owned())
        );
        assert_eq!(validate_generation(1), Ok(()));

        assert_eq!(
            validate_operation(OutputFormat::NativePrinter, ExecutionOperation::Publication),
            Err("A native-printer plan must use native printer-page rendering.".to_owned())
        );
        assert_eq!(
            validate_operation(
                OutputFormat::PdfVector,
                ExecutionOperation::NativePrinter { dpi: 600 }
            ),
            Err("Native printer-page rendering requires a native-printer plan.".to_owned())
        );
        assert_eq!(
            validate_operation(
                OutputFormat::NativePrinter,
                ExecutionOperation::NativePrinter { dpi: 71 }
            ),
            Err(
                "Native printer resolution 71 DPI is outside the supported 72-9600 DPI range."
                    .to_owned()
            )
        );
        assert_eq!(
            validate_operation(
                OutputFormat::NativePrinter,
                ExecutionOperation::NativePrinter { dpi: 9_601 }
            ),
            Err(
                "Native printer resolution 9601 DPI is outside the supported 72-9600 DPI range."
                    .to_owned()
            )
        );
        assert_eq!(
            validate_operation(OutputFormat::PdfVector, ExecutionOperation::Publication),
            Ok(())
        );
        assert_eq!(
            validate_operation(
                OutputFormat::NativePrinter,
                ExecutionOperation::NativePrinter { dpi: 9_600 }
            ),
            Ok(())
        );
    }
}

//! Owned native hardcopy publication outside the UI thread.
//!
//! Rendering and finalization are deliberately separate owners. This runtime
//! receives an already-rendered, immutable artifact and performs the remaining
//! blocking work: deterministic package assembly, durable filesystem commit,
//! printer rediscovery, and native spooling. Exactly one finalization may be
//! active. Its completion remains retained until the caller presents the
//! digest-bound ticket that started it.

#![cfg(not(target_arch = "wasm32"))]

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock, mpsc};

use sha2::{Digest as _, Sha256};

use crate::hardcopy::{
    CancellationPhase, HardcopyArtifactIdentity, HardcopyFailureCode, HardcopyOutcome,
    HardcopyPlan, OutputFormat, RenderTarget,
};
use crate::product::ContentDigest;
use crate::workbench::workflows::export_workflow::{
    ExportWorkflowIo, NativeExportWorkflowIo, ObservedExportDestination, deterministic_stored_zip,
};
use crate::workbench::hardcopy_adapters::print::{
    HardcopyCancellationToken, HardcopyPrintError, discover_native_printers, spool_native_hardcopy,
};
use crate::workbench::hardcopy_adapters::render::{RenderedHardcopyPublication, RenderedPrinterPages};

const NATIVE_FINALIZATION_THREAD_NAME: &str = "rspice-hardcopy-finalize";
const CANCELLED_MESSAGE: &str = "Native hardcopy finalization was cancelled.";
const MAX_MEDIA_TYPE_BYTES: usize = 128;
const MAX_PRINTER_ID_BYTES: usize = 256;

/// Complete, owned input for the post-render publication boundary.
#[derive(Debug)]
pub(crate) enum FinalizationOperation {
    Export {
        publication: RenderedHardcopyPublication,
        destination: ObservedExportDestination,
        media_type: &'static str,
        multi_part: bool,
    },
    Print {
        pages: RenderedPrinterPages,
        printer_id: String,
        capabilities_digest: ContentDigest,
        cancellation: HardcopyCancellationToken,
    },
}

impl FinalizationOperation {
    fn cancellation(&self) -> Option<&HardcopyCancellationToken> {
        match self {
            Self::Export { .. } => None,
            Self::Print { cancellation, .. } => Some(cancellation),
        }
    }
}

/// Exact immutable identity retained by the UI for one finalization.
///
/// `operation_digest` binds the rendered payload identity and every destination
/// property that can change what is published. The mutable cancellation bit is
/// intentionally excluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct FinalizationTicket {
    pub(crate) generation: u64,
    pub(crate) plan_digest: ContentDigest,
    pub(crate) operation_digest: ContentDigest,
}

/// Successful result returned after the irreversible boundary completes.
#[derive(Debug)]
pub(crate) enum FinalizationPayload {
    Export {
        artifact: HardcopyArtifactIdentity,
        page_count: u32,
        format: OutputFormat,
    },
    Print {
        outcome: HardcopyOutcome,
        accepted: usize,
        display_name: String,
    },
}

/// A successful payload together with the exact identity that produced it.
#[derive(Debug)]
pub(crate) struct FinalizationCompletion {
    pub(crate) ticket: FinalizationTicket,
    pub(crate) payload: FinalizationPayload,
}

/// Typed terminal failure retained by the native finalization owner.
///
/// Spool failures remain structured so the UI can record the exact
/// `HardcopyOutcome::Failed` code, retryability, and accepted page count. No
/// error at the irreversible printer boundary is flattened into a generic
/// renderer or internal-failure message.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
pub(crate) enum FinalizationFailure {
    #[error("{0}")]
    Message(String),
    #[error("native hardcopy finalization was cancelled after {pages_completed} completed page(s)")]
    Cancelled {
        phase: CancellationPhase,
        pages_completed: u32,
        reason: Option<String>,
    },
    #[error("{error} after {pages_completed} completed page(s)")]
    Print {
        error: HardcopyPrintError,
        pages_completed: u32,
    },
    #[error("the selected printer {printer_id:?} is no longer available")]
    DeviceUnavailable {
        printer_id: String,
        pages_completed: u32,
    },
}

impl FinalizationFailure {
    /// Convert every typed print-side failure into the exact receipt outcome
    /// that the UI must record. Generic export/controller messages deliberately
    /// return `None` and remain owned by their original workflow.
    #[must_use]
    pub(crate) fn hardcopy_outcome(&self) -> Option<HardcopyOutcome> {
        match self {
            Self::Cancelled {
                phase,
                pages_completed,
                reason,
            } => Some(HardcopyOutcome::Cancelled {
                phase: *phase,
                pages_completed: *pages_completed,
                reason: reason.clone(),
            }),
            Self::Print {
                error,
                pages_completed,
            } => Some(error.failure_outcome(*pages_completed)),
            Self::DeviceUnavailable {
                printer_id,
                pages_completed,
            } => Some(HardcopyOutcome::Failed {
                code: HardcopyFailureCode::DeviceUnavailable,
                message: format!("The selected printer {printer_id:?} is no longer available."),
                pages_completed: *pages_completed,
                retryable: true,
            }),
            Self::Message(_) => None,
        }
    }
}

impl From<String> for FinalizationFailure {
    fn from(message: String) -> Self {
        Self::Message(message)
    }
}

struct WorkerEnvelope {
    ticket: FinalizationTicket,
    result: Result<FinalizationPayload, FinalizationFailure>,
}

struct ActiveFinalization {
    ticket: FinalizationTicket,
    cancelled: Arc<AtomicBool>,
    boundary_started: Arc<AtomicBool>,
    cancellation_phase: CancellationPhase,
    print_cancellation: Option<HardcopyCancellationToken>,
    receiver: mpsc::Receiver<WorkerEnvelope>,
}

static ACTIVE_FINALIZATION: OnceLock<Mutex<Option<ActiveFinalization>>> = OnceLock::new();

fn active_finalization() -> &'static Mutex<Option<ActiveFinalization>> {
    ACTIVE_FINALIZATION.get_or_init(|| Mutex::new(None))
}

/// Start one native finalization without blocking the UI thread.
///
/// A completed result still owns the global slot until it is polled with the
/// exact returned ticket. This prevents a second publication from replacing an
/// unobserved durable-write or spool result.
pub(crate) fn start(
    plan: Arc<HardcopyPlan>,
    generation: u64,
    operation: FinalizationOperation,
    repaint: egui::Context,
) -> Result<FinalizationTicket, FinalizationFailure> {
    validate_generation(generation).map_err(FinalizationFailure::Message)?;
    validate_operation(&plan, &operation)?;

    let ticket = FinalizationTicket {
        generation,
        plan_digest: plan.content_digest(),
        operation_digest: operation_digest(&operation),
    };
    let cancellation_phase = pre_boundary_cancellation_phase(&operation);
    let print_cancellation = operation.cancellation().cloned();
    let mut active = active_finalization().lock().map_err(|_| {
        FinalizationFailure::Message(
            "The native hardcopy finalization owner is unavailable.".to_owned(),
        )
    })?;
    ensure_finalization_slot_available(&mut active).map_err(FinalizationFailure::Message)?;

    let cancelled = Arc::new(AtomicBool::new(false));
    let boundary_started = Arc::new(AtomicBool::new(false));
    let worker_cancelled = Arc::clone(&cancelled);
    let worker_boundary = Arc::clone(&boundary_started);
    let (sender, receiver) = mpsc::sync_channel(1);
    std::thread::Builder::new()
        .name(NATIVE_FINALIZATION_THREAD_NAME.to_owned())
        .spawn(move || {
            let result = catch_worker_panic(|| {
                execute_owned(
                    &plan,
                    ticket,
                    operation,
                    worker_cancelled.as_ref(),
                    worker_boundary.as_ref(),
                )
            });
            // Cancellation may release a completed pre-boundary operation.
            // Delivery failure is therefore expected and must not panic.
            let _ = sender.send(WorkerEnvelope { ticket, result });
            repaint.request_repaint();
        })
        .map_err(|error| {
            FinalizationFailure::Message(format!(
                "Could not start native hardcopy finalization: {error}"
            ))
        })?;

    *active = Some(ActiveFinalization {
        ticket,
        cancelled,
        boundary_started,
        cancellation_phase,
        print_cancellation,
        receiver,
    });
    Ok(ticket)
}

/// Poll only the finalization identified by `expected`.
///
/// A stale ticket cannot consume or release the active result. Cancellation
/// suppresses only work that never entered the irreversible filesystem/spool
/// boundary; once that boundary starts, the actual outcome remains observable.
pub(crate) fn poll(
    expected: FinalizationTicket,
) -> Option<Result<FinalizationCompletion, FinalizationFailure>> {
    let mut active = match active_finalization().lock() {
        Ok(active) => active,
        Err(_) => {
            return Some(Err(FinalizationFailure::Message(
                "The native hardcopy finalization owner is unavailable.".to_owned(),
            )));
        }
    };
    let current = active.as_ref()?;
    if !tickets_match(expected, current.ticket) {
        return None;
    }

    match current.receiver.try_recv() {
        Ok(envelope) => {
            let cancelled = current.cancelled.load(Ordering::Acquire);
            let boundary_started = current.boundary_started.load(Ordering::Acquire);
            let cancellation_phase = current.cancellation_phase;
            active.take();
            if !tickets_match(expected, envelope.ticket) {
                return Some(Err(FinalizationFailure::Message(
                    "Native hardcopy finalization returned a stale completion identity.".to_owned(),
                )));
            }
            if cancelled && !boundary_started {
                return Some(Err(cancelled_failure(
                    cancellation_phase,
                    CANCELLED_MESSAGE,
                )));
            }
            Some(envelope.result.map(|payload| FinalizationCompletion {
                ticket: envelope.ticket,
                payload,
            }))
        }
        Err(mpsc::TryRecvError::Empty) => None,
        Err(mpsc::TryRecvError::Disconnected) => {
            active.take();
            Some(Err(FinalizationFailure::Message(
                "Native hardcopy finalization disconnected before returning a result.".to_owned(),
            )))
        }
    }
}

/// Request cancellation of the active finalization.
///
/// Native spooling receives the same token, allowing it to abort at its exact
/// page boundary. A filesystem write already in progress is allowed to report
/// its truthful completion rather than being mislabeled as cancelled.
pub(crate) fn cancel() {
    if let Ok(active) = active_finalization().lock()
        && let Some(active) = active.as_ref()
    {
        active.cancelled.store(true, Ordering::Release);
        if let Some(cancellation) = &active.print_cancellation {
            cancellation.cancel();
        }
    }
}

#[must_use]
pub(crate) fn is_active() -> bool {
    active_finalization()
        .lock()
        .is_ok_and(|active| active.is_some())
}

fn execute_owned(
    plan: &HardcopyPlan,
    ticket: FinalizationTicket,
    operation: FinalizationOperation,
    cancelled: &AtomicBool,
    boundary_started: &AtomicBool,
) -> Result<FinalizationPayload, FinalizationFailure> {
    ensure_not_cancelled(
        cancelled,
        operation.cancellation(),
        pre_boundary_cancellation_phase(&operation),
        "Cancelled before native hardcopy finalization began",
    )?;
    validate_worker_identity(plan, ticket, &operation)?;

    match operation {
        FinalizationOperation::Export {
            publication,
            destination,
            media_type,
            multi_part,
        } => {
            let page_count = publication.page_count();
            let format = publication.format();
            let (bytes, artifact) = export_bytes_and_identity(&publication, plan, multi_part)?;
            ensure_not_cancelled(
                cancelled,
                None,
                CancellationPhase::CommittingArtifact,
                "Cancelled before the hardcopy artifact was committed",
            )?;
            validate_export_metadata(
                plan,
                publication.format(),
                publication.page_count(),
                publication.parts().len(),
                &destination,
                media_type,
                multi_part,
            )?;
            validate_worker_ticket(
                plan,
                ticket,
                operation_digest_for_export(&publication, &destination, media_type, multi_part),
            )?;

            boundary_started.store(true, Ordering::Release);
            write_export_bytes(&destination, &bytes, media_type)?;
            Ok(FinalizationPayload::Export {
                artifact,
                page_count,
                format,
            })
        }
        FinalizationOperation::Print {
            pages,
            printer_id,
            capabilities_digest,
            cancellation,
        } => {
            // Discovery is blocking but reversible, so it intentionally occurs
            // before the irreversible marker is raised.
            let report =
                discover_native_printers().map_err(|error| FinalizationFailure::Print {
                    error,
                    pages_completed: 0,
                })?;
            let capabilities = report
                .printers()
                .iter()
                .find(|entry| entry.capabilities().device_id() == printer_id)
                .map(|entry| entry.capabilities().clone())
                .ok_or_else(|| FinalizationFailure::DeviceUnavailable {
                    printer_id: printer_id.clone(),
                    pages_completed: 0,
                })?;
            validate_capabilities_digest(capabilities_digest, capabilities.content_digest())
                .map_err(|error| FinalizationFailure::Print {
                    error,
                    pages_completed: 0,
                })?;
            ensure_not_cancelled(
                cancelled,
                Some(&cancellation),
                CancellationPhase::Preparing,
                "Cancelled before the native spool job was opened",
            )?;
            validate_print_metadata(
                plan,
                &printer_id,
                capabilities_digest,
                pages.pages().len(),
                cancellation.is_cancelled(),
            )?;
            validate_worker_ticket(
                plan,
                ticket,
                operation_digest_for_print(&pages, &printer_id, capabilities_digest),
            )?;

            let display_name = capabilities.display_name().to_owned();
            boundary_started.store(true, Ordering::Release);
            let outcome = spool_native_hardcopy(plan, &pages, &capabilities, &cancellation)
                .map_err(|failure| {
                    let (error, pages_completed) = failure.into_parts();
                    FinalizationFailure::Print {
                        error,
                        pages_completed,
                    }
                })?;
            let accepted = match &outcome {
                HardcopyOutcome::SpoolAccepted { pages_accepted, .. } => *pages_accepted as usize,
                HardcopyOutcome::Cancelled {
                    pages_completed, ..
                } => *pages_completed as usize,
                _ => 0,
            };
            Ok(FinalizationPayload::Print {
                outcome,
                accepted,
                display_name,
            })
        }
    }
}

fn validate_operation(
    plan: &HardcopyPlan,
    operation: &FinalizationOperation,
) -> Result<(), FinalizationFailure> {
    match operation {
        FinalizationOperation::Export {
            publication,
            destination,
            media_type,
            multi_part,
        } => validate_export_metadata(
            plan,
            publication.format(),
            publication.page_count(),
            publication.parts().len(),
            destination,
            media_type,
            *multi_part,
        )
        .map_err(FinalizationFailure::Message),
        FinalizationOperation::Print {
            pages,
            printer_id,
            capabilities_digest,
            cancellation,
        } => validate_print_metadata(
            plan,
            printer_id,
            *capabilities_digest,
            pages.pages().len(),
            cancellation.is_cancelled(),
        ),
    }
}

fn validate_export_metadata(
    plan: &HardcopyPlan,
    publication_format: OutputFormat,
    page_count: u32,
    part_count: usize,
    destination: &ObservedExportDestination,
    media_type: &str,
    multi_part: bool,
) -> Result<(), String> {
    if !matches!(plan.setup().render().target(), RenderTarget::ExportArtifact) {
        return Err("Hardcopy export finalization requires an export-artifact plan.".to_owned());
    }
    let format = plan.setup().render().format();
    if matches!(
        format,
        OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument
    ) || publication_format != format
    {
        return Err(
            "The rendered hardcopy publication format does not match the export plan.".to_owned(),
        );
    }
    let planned_pages = plan.pagination().pages().len() as u32;
    if page_count != planned_pages || page_count == 0 {
        return Err(
            "The rendered hardcopy publication page count does not match the export plan."
                .to_owned(),
        );
    }
    let expected_multi_part =
        matches!(format, OutputFormat::SvgVector | OutputFormat::Png { .. }) && planned_pages > 1;
    if multi_part != expected_multi_part
        || (multi_part && part_count != planned_pages as usize)
        || (!multi_part && part_count != 1)
    {
        return Err(
            "The rendered hardcopy publication part contract does not match the export plan."
                .to_owned(),
        );
    }
    if destination.path().as_os_str().is_empty() {
        return Err("Hardcopy export destination must not be empty.".to_owned());
    }
    validate_media_type(media_type, format, multi_part)
}

fn validate_print_metadata(
    plan: &HardcopyPlan,
    printer_id: &str,
    capabilities_digest: ContentDigest,
    page_count: usize,
    already_cancelled: bool,
) -> Result<(), FinalizationFailure> {
    if already_cancelled {
        return Err(cancelled_failure(
            CancellationPhase::Preparing,
            "Cancelled before native hardcopy finalization began",
        ));
    }
    if printer_id.is_empty()
        || printer_id.len() > MAX_PRINTER_ID_BYTES
        || printer_id.trim() != printer_id
    {
        return Err(FinalizationFailure::Print {
            error: HardcopyPrintError::InvalidCapabilitySnapshot(
                "native hardcopy printer identity is invalid".to_owned(),
            ),
            pages_completed: 0,
        });
    }
    let RenderTarget::SystemPrinter {
        printer_id: planned_printer,
        job,
    } = plan.setup().render().target()
    else {
        return Err(FinalizationFailure::Print {
            error: HardcopyPrintError::NativePrinterPlanRequired,
            pages_completed: 0,
        });
    };
    if plan.setup().render().format() != OutputFormat::NativePrinter {
        return Err(FinalizationFailure::Print {
            error: HardcopyPrintError::NativePrinterPlanRequired,
            pages_completed: 0,
        });
    }
    if planned_printer != printer_id {
        return Err(FinalizationFailure::Print {
            error: HardcopyPrintError::PrinterIdentityMismatch {
                expected: planned_printer.clone(),
                observed: printer_id.to_owned(),
            },
            pages_completed: 0,
        });
    }
    if job.capabilities_digest() != capabilities_digest {
        return Err(FinalizationFailure::Print {
            error: HardcopyPrintError::PrinterCapabilitiesChanged,
            pages_completed: 0,
        });
    }
    if page_count == 0 || page_count != plan.pagination().pages().len() {
        return Err(FinalizationFailure::Print {
            error: HardcopyPrintError::PrinterPublicationMismatch(
                "rendered page count does not match the immutable print plan".to_owned(),
            ),
            pages_completed: 0,
        });
    }
    Ok(())
}

fn validate_media_type(
    media_type: &str,
    format: OutputFormat,
    multi_part: bool,
) -> Result<(), String> {
    if media_type.is_empty()
        || media_type.len() > MAX_MEDIA_TYPE_BYTES
        || !media_type.is_ascii()
        || media_type.trim() != media_type
    {
        return Err("Hardcopy export media type is invalid.".to_owned());
    }
    let expected = if multi_part {
        "application/zip"
    } else {
        match format {
            OutputFormat::PdfVector | OutputFormat::PdfA => "application/pdf",
            OutputFormat::SvgVector => "image/svg+xml",
            OutputFormat::Png { .. } => "image/png",
            OutputFormat::Tiff { .. } => "image/tiff",
            OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument => {
                return Err("Printer formats cannot be exported as files.".to_owned());
            }
        }
    };
    if media_type != expected {
        return Err(format!(
            "Hardcopy export media type {media_type:?} does not match the required {expected:?}."
        ));
    }
    Ok(())
}

fn validate_capabilities_digest(
    expected: ContentDigest,
    observed: ContentDigest,
) -> Result<(), HardcopyPrintError> {
    if expected != observed {
        Err(HardcopyPrintError::PrinterCapabilitiesChanged)
    } else {
        Ok(())
    }
}

fn ensure_not_cancelled(
    cancelled: &AtomicBool,
    print_cancellation: Option<&HardcopyCancellationToken>,
    phase: CancellationPhase,
    reason: &'static str,
) -> Result<(), FinalizationFailure> {
    if cancelled.load(Ordering::Acquire)
        || print_cancellation.is_some_and(HardcopyCancellationToken::is_cancelled)
    {
        Err(cancelled_failure(phase, reason))
    } else {
        Ok(())
    }
}

fn pre_boundary_cancellation_phase(operation: &FinalizationOperation) -> CancellationPhase {
    match operation {
        FinalizationOperation::Export { .. } => CancellationPhase::CommittingArtifact,
        FinalizationOperation::Print { .. } => CancellationPhase::Preparing,
    }
}

fn cancelled_failure(phase: CancellationPhase, reason: impl Into<String>) -> FinalizationFailure {
    FinalizationFailure::Cancelled {
        phase,
        pages_completed: 0,
        reason: Some(reason.into()),
    }
}

fn validate_generation(generation: u64) -> Result<(), String> {
    if generation == 0 {
        Err("Native hardcopy finalization generation must be non-zero.".to_owned())
    } else {
        Ok(())
    }
}

fn validate_worker_identity(
    plan: &HardcopyPlan,
    ticket: FinalizationTicket,
    operation: &FinalizationOperation,
) -> Result<(), String> {
    validate_worker_ticket(plan, ticket, operation_digest(operation))
}

fn validate_worker_ticket(
    plan: &HardcopyPlan,
    ticket: FinalizationTicket,
    observed_operation_digest: ContentDigest,
) -> Result<(), String> {
    if ticket.generation == 0
        || ticket.plan_digest != plan.content_digest()
        || ticket.operation_digest != observed_operation_digest
    {
        Err("Native hardcopy finalization rejected a stale operation identity.".to_owned())
    } else {
        Ok(())
    }
}

fn operation_digest(operation: &FinalizationOperation) -> ContentDigest {
    match operation {
        FinalizationOperation::Export {
            publication,
            destination,
            media_type,
            multi_part,
        } => operation_digest_for_export(publication, destination, media_type, *multi_part),
        FinalizationOperation::Print {
            pages,
            printer_id,
            capabilities_digest,
            ..
        } => operation_digest_for_print(pages, printer_id, *capabilities_digest),
    }
}

fn operation_digest_for_export(
    publication: &RenderedHardcopyPublication,
    destination: &ObservedExportDestination,
    media_type: &str,
    multi_part: bool,
) -> ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-native-hardcopy-export-v1");
    digest.update(publication.digest().as_bytes());
    digest.update([u8::from(multi_part)]);
    update_digest_field(&mut digest, media_type.as_bytes());
    update_digest_field(&mut digest, format!("{destination:?}").as_bytes());
    ContentDigest::from_bytes(digest.finalize().into())
}

fn operation_digest_for_print(
    pages: &RenderedPrinterPages,
    printer_id: &str,
    capabilities_digest: ContentDigest,
) -> ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-native-hardcopy-print-v1");
    digest.update(pages.digest().as_bytes());
    digest.update(capabilities_digest.as_bytes());
    update_digest_field(&mut digest, printer_id.as_bytes());
    ContentDigest::from_bytes(digest.finalize().into())
}

fn update_digest_field(digest: &mut Sha256, bytes: &[u8]) {
    digest.update((bytes.len() as u64).to_le_bytes());
    digest.update(bytes);
}

fn export_bytes_and_identity(
    publication: &RenderedHardcopyPublication,
    plan: &HardcopyPlan,
    multi_part: bool,
) -> Result<(Vec<u8>, HardcopyArtifactIdentity), String> {
    if !multi_part {
        let part = publication.single_part().ok_or_else(|| {
            "The renderer returned multiple files for a single-file format.".to_owned()
        })?;
        return Ok((
            part.bytes().to_vec(),
            publication.identity().map_err(|error| error.to_string())?,
        ));
    }
    let entries = publication
        .parts()
        .iter()
        .map(|part| (part.suggested_filename(), part.bytes()))
        .collect::<Vec<_>>();
    let bytes = deterministic_stored_zip(&entries)?;
    let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
    let artifact = HardcopyArtifactIdentity::try_new(
        digest,
        bytes.len() as u64,
        plan.pagination().pages().len() as u32,
        plan.setup().render().format(),
    )
    .map_err(|error| error.to_string())?;
    Ok((bytes, artifact))
}

fn write_export_bytes(
    destination: &ObservedExportDestination,
    bytes: &[u8],
    media_type: &str,
) -> Result<(), String> {
    NativeExportWorkflowIo.write_bytes_file_observed(destination, bytes, media_type)
}

fn tickets_match(expected: FinalizationTicket, actual: FinalizationTicket) -> bool {
    expected == actual
}

fn catch_worker_panic(
    execute: impl FnOnce() -> Result<FinalizationPayload, FinalizationFailure>,
) -> Result<FinalizationPayload, FinalizationFailure> {
    catch_unwind(AssertUnwindSafe(execute)).unwrap_or_else(|_| {
        Err(FinalizationFailure::Message(
            "Native hardcopy finalization terminated unexpectedly.".to_owned(),
        ))
    })
}

fn ensure_finalization_slot_available(
    active: &mut Option<ActiveFinalization>,
) -> Result<(), String> {
    reap_finished_pre_boundary_cancellation(active);
    if active.is_some() {
        Err("A native hardcopy finalization is already active.".to_owned())
    } else {
        Ok(())
    }
}

fn reap_finished_pre_boundary_cancellation(active: &mut Option<ActiveFinalization>) {
    let Some(current) = active.as_ref() else {
        return;
    };
    if !current.cancelled.load(Ordering::Acquire)
        || current.boundary_started.load(Ordering::Acquire)
    {
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
    use std::path::PathBuf;
    use std::sync::atomic::AtomicU64;

    use crate::hardcopy::{
        ActiveHardcopySource, ContentExtent, HardcopyDocumentId, HardcopyDocumentKind,
        HardcopyScope, HardcopySetup, Length,
    };
    use crate::product::ObjectRevision;

    use super::*;

    static TEST_RUNTIME_SERIAL: Mutex<()> = Mutex::new(());
    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(1);

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

    fn ticket(generation: u64) -> FinalizationTicket {
        FinalizationTicket {
            generation,
            plan_digest: digest(0x11),
            operation_digest: digest(0x22),
        }
    }

    fn export_plan() -> HardcopyPlan {
        let source = ActiveHardcopySource::try_new(
            HardcopyDocumentId::new(),
            ObjectRevision::INITIAL,
            digest(0x44),
            "finalization fixture",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::CurrentSheet,
        )
        .expect("fixture source");
        HardcopyPlan::compile(
            source,
            HardcopySetup::default(),
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(100_000),
            )
            .expect("fixture extent"),
        )
        .expect("fixture plan")
    }

    fn reset_runtime() {
        active_finalization()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
    }

    fn install_active(
        active_ticket: FinalizationTicket,
        boundary_started: bool,
    ) -> (
        Arc<AtomicBool>,
        Arc<AtomicBool>,
        mpsc::SyncSender<WorkerEnvelope>,
    ) {
        let cancelled = Arc::new(AtomicBool::new(false));
        let boundary = Arc::new(AtomicBool::new(boundary_started));
        let (sender, receiver) = mpsc::sync_channel(1);
        *active_finalization()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(ActiveFinalization {
            ticket: active_ticket,
            cancelled: Arc::clone(&cancelled),
            boundary_started: Arc::clone(&boundary),
            cancellation_phase: CancellationPhase::Preparing,
            print_cancellation: None,
            receiver,
        });
        (cancelled, boundary, sender)
    }

    fn envelope(completion_ticket: FinalizationTicket, error: &str) -> WorkerEnvelope {
        WorkerEnvelope {
            ticket: completion_ticket,
            result: Err(FinalizationFailure::Message(error.to_owned())),
        }
    }

    fn unique_temp_path(label: &str) -> PathBuf {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "rspice-hardcopy-finalize-{label}-{}-{sequence}.bin",
            std::process::id()
        ))
    }

    #[test]
    fn exact_ticket_guard_rejects_every_stale_dimension() {
        let expected = ticket(7);
        assert!(tickets_match(expected, expected));

        let mut changed = expected;
        changed.generation = 8;
        assert!(!tickets_match(expected, changed));
        changed = expected;
        changed.plan_digest = digest(0x33);
        assert!(!tickets_match(expected, changed));
        changed = expected;
        changed.operation_digest = digest(0x44);
        assert!(!tickets_match(expected, changed));
    }

    #[test]
    fn one_active_owner_retains_an_unobserved_completion() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(11);
        let (_cancelled, _boundary, sender) = install_active(exact, false);
        sender
            .send(envelope(exact, "unobserved completion"))
            .expect("active receiver must retain completion");

        let mut active = active_finalization()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        assert_eq!(
            ensure_finalization_slot_available(&mut active),
            Err("A native hardcopy finalization is already active.".to_owned())
        );
        assert_eq!(active.as_ref().map(|current| current.ticket), Some(exact));
    }

    #[test]
    fn stale_ticket_cannot_consume_exact_completion() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(21);
        let (_cancelled, _boundary, sender) = install_active(exact, false);
        sender
            .send(envelope(exact, "exact completion"))
            .expect("active receiver must accept completion");

        let mut stale = exact;
        stale.operation_digest = digest(0x77);
        assert!(poll(stale).is_none());
        assert!(is_active());
        assert!(matches!(
            poll(exact),
            Some(Err(ref error)) if error.to_string() == "exact completion"
        ));
        assert!(!is_active());
    }

    #[test]
    fn cancellation_discards_only_a_pre_boundary_late_completion() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(31);
        let (cancelled, _boundary, sender) = install_active(exact, false);

        cancel();
        assert!(cancelled.load(Ordering::Acquire));
        sender
            .send(envelope(exact, "late pre-boundary result"))
            .expect("active receiver must accept late completion");
        let failure = poll(exact)
            .expect("cancelled completion must be terminal")
            .expect_err("cancelled completion must not publish");
        assert!(matches!(
            failure.hardcopy_outcome(),
            Some(HardcopyOutcome::Cancelled {
                phase: CancellationPhase::Preparing,
                pages_completed: 0,
                ..
            })
        ));
        assert!(!is_active());
    }

    #[test]
    fn cancellation_preserves_a_post_boundary_completion() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(32);
        let (_cancelled, _boundary, sender) = install_active(exact, true);

        cancel();
        sender
            .send(envelope(exact, "durable boundary failure"))
            .expect("active receiver must accept completion");
        assert!(matches!(
            poll(exact),
            Some(Err(ref error)) if error.to_string() == "durable boundary failure"
        ));
    }

    #[test]
    fn completed_cancelled_pre_boundary_owner_is_reaped_before_new_claim() {
        let exact = ticket(36);
        let cancelled = Arc::new(AtomicBool::new(true));
        let boundary_started = Arc::new(AtomicBool::new(false));
        let (sender, receiver) = mpsc::sync_channel(1);
        sender
            .send(envelope(exact, "late completion"))
            .expect("local receiver must retain completion");
        let mut active = Some(ActiveFinalization {
            ticket: exact,
            cancelled,
            boundary_started,
            cancellation_phase: CancellationPhase::Preparing,
            print_cancellation: None,
            receiver,
        });

        assert_eq!(ensure_finalization_slot_available(&mut active), Ok(()));
        assert!(active.is_none());
    }

    #[test]
    fn completed_post_boundary_owner_is_never_reaped_without_observation() {
        let exact = ticket(37);
        let cancelled = Arc::new(AtomicBool::new(true));
        let boundary_started = Arc::new(AtomicBool::new(true));
        let (sender, receiver) = mpsc::sync_channel(1);
        sender
            .send(envelope(exact, "committed completion"))
            .expect("local receiver must retain completion");
        let mut active = Some(ActiveFinalization {
            ticket: exact,
            cancelled,
            boundary_started,
            cancellation_phase: CancellationPhase::Preparing,
            print_cancellation: None,
            receiver,
        });

        assert_eq!(
            ensure_finalization_slot_available(&mut active),
            Err("A native hardcopy finalization is already active.".to_owned())
        );
        assert!(active.is_some());
    }

    #[test]
    fn disconnected_worker_is_terminal_and_releases_owner() {
        let _guard = RuntimeGuard::acquire();
        let exact = ticket(41);
        let (_cancelled, _boundary, sender) = install_active(exact, false);
        drop(sender);

        assert!(matches!(
            poll(exact),
            Some(Err(ref error))
                if error.to_string()
                    == "Native hardcopy finalization disconnected before returning a result."
        ));
        assert!(!is_active());
    }

    #[test]
    fn worker_panic_is_contained_as_a_bounded_error() {
        let result = catch_worker_panic(|| -> Result<FinalizationPayload, FinalizationFailure> {
            panic!("synthetic finalizer panic")
        });
        assert!(matches!(
            result,
            Err(ref error)
                if error.to_string()
                    == "Native hardcopy finalization terminated unexpectedly."
        ));
    }

    #[test]
    fn spool_failure_preserves_typed_error_pages_and_failure_outcome() {
        let failure = FinalizationFailure::Print {
            error: HardcopyPrintError::UnsupportedResolution(1_200),
            pages_completed: 3,
        };
        let FinalizationFailure::Print {
            error,
            pages_completed,
        } = &failure
        else {
            panic!("spool failure must remain structured");
        };
        assert_eq!(*pages_completed, 3);
        assert_eq!(error, &HardcopyPrintError::UnsupportedResolution(1_200));
        assert!(matches!(
            failure.hardcopy_outcome(),
            Some(HardcopyOutcome::Failed {
                code: HardcopyFailureCode::InvalidPrinterConfiguration,
                pages_completed: 3,
                retryable: false,
                ..
            })
        ));
        assert!(matches!(
            error.failure_outcome(*pages_completed),
            HardcopyOutcome::Failed {
                code: HardcopyFailureCode::InvalidPrinterConfiguration,
                pages_completed: 3,
                retryable: false,
                ..
            }
        ));

        let FinalizationFailure::Print {
            error,
            pages_completed,
        } = failure
        else {
            panic!("caller must be able to consume the typed spool failure");
        };
        let owned = (error, pages_completed);
        assert_eq!(owned, (HardcopyPrintError::UnsupportedResolution(1_200), 3));
    }

    #[test]
    fn pre_spool_cancellation_is_a_typed_zero_page_receipt_outcome() {
        let cancellation = HardcopyCancellationToken::default();
        cancellation.cancel();
        let failure = ensure_not_cancelled(
            &AtomicBool::new(false),
            Some(&cancellation),
            CancellationPhase::Preparing,
            "Cancelled before the native spool job was opened",
        )
        .expect_err("cancelled print token must fail closed");
        assert!(matches!(
            failure,
            FinalizationFailure::Cancelled {
                phase: CancellationPhase::Preparing,
                pages_completed: 0,
                ..
            }
        ));

        let failure = cancelled_failure(
            CancellationPhase::Preparing,
            "Cancelled before the native spool job was opened",
        );
        assert!(matches!(
            &failure,
            FinalizationFailure::Cancelled {
                phase: CancellationPhase::Preparing,
                pages_completed: 0,
                reason: Some(reason),
            } if reason == "Cancelled before the native spool job was opened"
        ));
        assert!(matches!(
            failure.hardcopy_outcome(),
            Some(HardcopyOutcome::Cancelled {
                phase: CancellationPhase::Preparing,
                pages_completed: 0,
                reason: Some(reason),
            }) if reason == "Cancelled before the native spool job was opened"
        ));
    }

    #[test]
    fn discovery_and_missing_device_failures_have_truthful_print_outcomes() {
        let discovery = FinalizationFailure::Print {
            error: HardcopyPrintError::PlatformUnavailable(
                crate::workbench::hardcopy_adapters::print::HardcopyPlatformUnavailableReason::
                    NativePrintingIsWindowsOnly,
            ),
            pages_completed: 0,
        };
        assert!(matches!(
            discovery.hardcopy_outcome(),
            Some(HardcopyOutcome::Failed {
                code: HardcopyFailureCode::DeviceUnavailable,
                pages_completed: 0,
                retryable: true,
                ..
            })
        ));

        let missing = FinalizationFailure::DeviceUnavailable {
            printer_id: "lab-printer-07".to_owned(),
            pages_completed: 0,
        };
        assert!(matches!(
            missing.hardcopy_outcome(),
            Some(HardcopyOutcome::Failed {
                code: HardcopyFailureCode::DeviceUnavailable,
                pages_completed: 0,
                retryable: true,
                ..
            })
        ));
        assert!(missing.to_string().contains("lab-printer-07"));
    }

    #[test]
    fn generation_export_contract_and_media_type_fail_closed() {
        assert_eq!(
            validate_generation(0),
            Err("Native hardcopy finalization generation must be non-zero.".to_owned())
        );
        assert_eq!(validate_generation(1), Ok(()));
        assert_eq!(
            validate_media_type("image/png", OutputFormat::PdfVector, false),
            Err(
                "Hardcopy export media type \"image/png\" does not match the required \"application/pdf\"."
                    .to_owned()
            )
        );
        assert_eq!(
            validate_media_type("application/zip", OutputFormat::SvgVector, true),
            Ok(())
        );

        let plan = export_plan();
        assert!(matches!(
            validate_print_metadata(&plan, "printer-a", digest(1), 1, true),
            Err(FinalizationFailure::Cancelled {
                phase: CancellationPhase::Preparing,
                pages_completed: 0,
                ..
            })
        ));
        assert!(matches!(
            validate_print_metadata(&plan, "printer-a", digest(1), 1, false),
            Err(FinalizationFailure::Print {
                error: HardcopyPrintError::NativePrinterPlanRequired,
                pages_completed: 0,
            })
        ));

        let path = unique_temp_path("invalid-contract");
        let destination = NativeExportWorkflowIo
            .observe_destination(&path)
            .expect("observe fixture destination");
        assert!(matches!(
            validate_export_metadata(
                &plan,
                OutputFormat::Png { dpi: 300 },
                1,
                1,
                &destination,
                "image/png",
                false,
            ),
            Err(ref error)
                if error
                    == "The rendered hardcopy publication format does not match the export plan."
        ));
        assert!(matches!(
            validate_export_metadata(
                &plan,
                OutputFormat::PdfVector,
                0,
                1,
                &destination,
                "application/pdf",
                false,
            ),
            Err(ref error)
                if error
                    == "The rendered hardcopy publication page count does not match the export plan."
        ));
    }

    #[test]
    fn printer_capability_digest_mismatch_is_rejected_exactly() {
        let error = validate_capabilities_digest(digest(0x51), digest(0x52))
            .expect_err("changed capabilities must fail closed");
        assert_eq!(error, HardcopyPrintError::PrinterCapabilitiesChanged);
        let failure = FinalizationFailure::Print {
            error,
            pages_completed: 0,
        };
        assert!(matches!(
            failure.hardcopy_outcome(),
            Some(HardcopyOutcome::Failed {
                code: HardcopyFailureCode::InvalidPrinterConfiguration,
                pages_completed: 0,
                retryable: false,
                ..
            })
        ));
        assert_eq!(
            validate_capabilities_digest(digest(0x51), digest(0x51)),
            Ok(())
        );
    }

    #[test]
    fn native_export_uses_observed_durable_destination_state() {
        let path = unique_temp_path("durability");
        let io = NativeExportWorkflowIo;
        let destination = io
            .observe_destination(&path)
            .expect("observe missing destination");
        let bytes = b"authenticated hardcopy bytes";
        write_export_bytes(&destination, bytes, "application/pdf").expect("durable write");
        assert_eq!(std::fs::read(&path).expect("read durable result"), bytes);
        std::fs::remove_file(path).expect("remove durable fixture");
    }

    #[test]
    fn native_export_rejects_destination_changed_after_observation() {
        let path = unique_temp_path("race");
        let io = NativeExportWorkflowIo;
        let destination = io
            .observe_destination(&path)
            .expect("observe missing destination");
        std::fs::write(&path, b"other writer").expect("create competing file");

        assert!(write_export_bytes(&destination, b"hardcopy", "application/pdf").is_err());
        assert_eq!(
            std::fs::read(&path).expect("read competing file"),
            b"other writer"
        );
        std::fs::remove_file(path).expect("remove durable fixture");
    }
}

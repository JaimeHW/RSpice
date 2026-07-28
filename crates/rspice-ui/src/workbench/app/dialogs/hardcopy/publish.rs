//! Hardcopy workflow controller and irreversible publication boundary.
//!
//! The dialog owns only a mutable draft. Every primary action resolves the
//! active semantic document again, verifies it still matches the preview
//! authority, compiles a fresh immutable plan, and only then crosses the file,
//! browser-print, or native-spool boundary.

#[cfg(target_arch = "wasm32")]
use sha2::{Digest as _, Sha256};

use super::HardcopyWorkflow;
use crate::diagnostics::ConsoleMessage;
use crate::workbench::app::RSpiceApp;

thread_local! {
    static REPAINT_CONTEXT: std::cell::RefCell<Option<egui::Context>> =
        const { std::cell::RefCell::new(None) };
    static PENDING_PAGE_SETUP: std::cell::RefCell<Option<PendingPageSetup>> =
        const { std::cell::RefCell::new(None) };
}

struct PendingPageSetup {
    opened_source: std::sync::Arc<ResolvedHardcopyDocument>,
    setup: crate::hardcopy::HardcopySetup,
    staged_mapping: StagedPrintMappingPersistence,
}

pub(super) fn register_repaint_context(context: &egui::Context) {
    REPAINT_CONTEXT.with(|slot| *slot.borrow_mut() = Some(context.clone()));
}

fn repaint_context() -> egui::Context {
    REPAINT_CONTEXT
        .with(|slot| slot.borrow().clone())
        .unwrap_or_default()
}
#[cfg(target_arch = "wasm32")]
use crate::hardcopy::HardcopyArtifactIdentity;
use crate::hardcopy::{
    DuplexMode, HardcopyFailureCode, HardcopyOutcome, HardcopyPlan, HardcopyReceipt, Orientation,
    OutputFormat, PrinterJobSettings, PrinterMediaSource, ResolvedOrientation,
};
use crate::product::ContentDigest;
#[cfg(target_arch = "wasm32")]
use crate::workbench::workflows::export_workflow::deterministic_stored_zip;
use crate::workbench::workflows::export_workflow::{
    ObservedExportDestination, SaveDialogConfig, export_completion_message,
};
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::hardcopy_adapters::print::discover_native_printers;
#[cfg(target_arch = "wasm32")]
use crate::workbench::hardcopy_adapters::print::{
    BrowserPrintReservation, finalize_browser_print, reserve_browser_print_window,
};
use crate::workbench::hardcopy_adapters::print::{HardcopyCancellationToken, PrinterCapabilitySnapshot};
#[cfg(test)]
use crate::workbench::hardcopy_adapters::render::HardcopyRenderer;
#[cfg(target_arch = "wasm32")]
use crate::workbench::hardcopy_adapters::render::RenderedHardcopyPublication;
use crate::workbench::hardcopy_adapters::render::{HardcopyPublicationTimestamp, HardcopySceneMetadata};
use crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument;

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static PRINTER_DISCOVERY: std::cell::RefCell<Option<
        std::sync::mpsc::Receiver<Result<
            crate::workbench::hardcopy_adapters::print::PrinterDiscoveryReport,
            String,
        >>,
    >> = const { std::cell::RefCell::new(None) };
    static SOURCE_RESOLUTION: std::cell::RefCell<Option<ActiveSourceResolution>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static SOURCE_RESOLUTION: std::cell::RefCell<Option<ActiveBrowserSourceResolution>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
struct ActiveBrowserSourceResolution {
    ticket: super::worker::HardcopyWorkerTicket,
    purpose: SourceResolutionPurpose,
    source_key: String,
    scope: crate::hardcopy::HardcopyScope,
    expected: Option<ResolvedHardcopyDocument>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static PUBLICATION: std::cell::RefCell<Option<ActiveBrowserPublication>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
struct ActiveBrowserPublication {
    ticket: super::worker::HardcopyWorkerTicket,
    plan: std::sync::Arc<HardcopyPlan>,
    source: std::sync::Arc<ResolvedHardcopyDocument>,
    staged_mapping: StagedPrintMappingPersistence,
    destination: BrowserPublicationDestination,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static PENDING_PUBLICATION: std::cell::RefCell<Option<PendingBrowserPublication>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(target_arch = "wasm32")]
struct PendingBrowserPublication {
    plan: std::sync::Arc<HardcopyPlan>,
    opened_source: std::sync::Arc<ResolvedHardcopyDocument>,
    metadata: HardcopySceneMetadata,
    staged_mapping: StagedPrintMappingPersistence,
    destination: BrowserPublicationDestination,
}

#[cfg(target_arch = "wasm32")]
enum BrowserPublicationDestination {
    Print {
        reservation: BrowserPrintReservation,
    },
    Export {
        path: std::path::PathBuf,
        destination: ObservedExportDestination,
        media_type: &'static str,
        multi_part: bool,
    },
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static PUBLICATION: std::cell::RefCell<Option<ActiveNativePublication>> =
        const { std::cell::RefCell::new(None) };
    static FINALIZATION: std::cell::RefCell<Option<ActiveNativeFinalization>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(not(target_arch = "wasm32"))]
struct ActiveNativePublication {
    ticket: super::execution::ExecutionTicket,
    plan: std::sync::Arc<HardcopyPlan>,
    staged_mapping: StagedPrintMappingPersistence,
    destination: NativePublicationDestination,
}

#[cfg(not(target_arch = "wasm32"))]
struct ActiveNativeFinalization {
    ticket: super::finalize::FinalizationTicket,
    plan: std::sync::Arc<HardcopyPlan>,
    source_digest: ContentDigest,
    staged_mapping: StagedPrintMappingPersistence,
    destination: NativePublicationDestination,
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static PENDING_PUBLICATION: std::cell::RefCell<Option<PendingNativePublication>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(not(target_arch = "wasm32"))]
struct PendingNativePublication {
    plan: std::sync::Arc<HardcopyPlan>,
    opened_source: std::sync::Arc<ResolvedHardcopyDocument>,
    metadata: HardcopySceneMetadata,
    staged_mapping: StagedPrintMappingPersistence,
    operation: super::execution::ExecutionOperation,
    destination: NativePublicationDestination,
}

#[cfg(not(target_arch = "wasm32"))]
enum NativePublicationDestination {
    Print {
        printer_id: String,
        capabilities_digest: ContentDigest,
        cancellation: HardcopyCancellationToken,
    },
    Export {
        path: std::path::PathBuf,
        destination: ObservedExportDestination,
        media_type: &'static str,
        multi_part: bool,
    },
}

#[cfg(not(target_arch = "wasm32"))]
struct ActiveSourceResolution {
    generation: u64,
    purpose: SourceResolutionPurpose,
    source_key: String,
    scope: crate::hardcopy::HardcopyScope,
    expected: Option<ResolvedHardcopyDocument>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
    receiver: std::sync::mpsc::Receiver<Result<ResolvedHardcopyDocument, String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceResolutionPurpose {
    Open(HardcopyWorkflow),
    Select,
    Publish,
    SavePageSetup,
}

pub(crate) fn open_hardcopy_workflow(app: &mut RSpiceApp, workflow: HardcopyWorkflow) {
    let (source_key, scope) = match active_retained_source_selection(app) {
        Ok(selection) => selection,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::warning(format!(
                "{} is unavailable: {error}.",
                workflow.title()
            )));
            return;
        }
    };
    let prepared = match crate::workbench::hardcopy_adapters::sources::prepare_retained_hardcopy_resolution(
        &app.state,
        &source_key,
        scope.clone(),
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            app.state.push_user_message(ConsoleMessage::warning(format!(
                "{} is unavailable: {error}.",
                workflow.title()
            )));
            return;
        }
    };
    let candidates =
        crate::workbench::hardcopy_adapters::sources::enumerate_retained_hardcopy_sources(&app.state);
    app.state.dialogs.hardcopy.begin_open(workflow, candidates);
    let generation = app
        .state
        .dialogs
        .hardcopy
        .next_source_resolution_generation();
    if let Err(error) = start_source_resolution(
        prepared,
        SourceResolutionPurpose::Open(workflow),
        generation,
        source_key,
        scope,
        None,
    ) {
        app.state.dialogs.hardcopy.busy = false;
        app.state.dialogs.hardcopy.error = Some(error);
    }
}

pub(super) fn select_retained_source(
    app: &mut RSpiceApp,
    source_key: &str,
    scope: crate::hardcopy::HardcopyScope,
) {
    let prepared = match crate::workbench::hardcopy_adapters::sources::prepare_retained_hardcopy_resolution(
        &app.state,
        source_key,
        scope.clone(),
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            app.state.dialogs.hardcopy.error = Some(error.to_string());
            return;
        }
    };
    app.state.dialogs.hardcopy.busy = true;
    app.state.dialogs.hardcopy.error = None;
    let generation = app
        .state
        .dialogs
        .hardcopy
        .next_source_resolution_generation();
    if let Err(error) = start_source_resolution(
        prepared,
        SourceResolutionPurpose::Select,
        generation,
        source_key.to_owned(),
        scope,
        None,
    ) {
        app.state.dialogs.hardcopy.busy = false;
        app.state.dialogs.hardcopy.error = Some(error);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn start_source_resolution(
    prepared: crate::workbench::hardcopy_adapters::sources::PreparedRetainedHardcopyResolution,
    purpose: SourceResolutionPurpose,
    generation: u64,
    source_key: String,
    scope: crate::hardcopy::HardcopyScope,
    expected: Option<ResolvedHardcopyDocument>,
) -> Result<(), String> {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let worker_cancelled = cancelled.clone();
    std::thread::Builder::new()
        .name("rspice-hardcopy-source".to_owned())
        .spawn(move || {
            if worker_cancelled.load(std::sync::atomic::Ordering::Acquire) {
                return;
            }
            let result = prepared.resolve_owned().map_err(|error| error.to_string());
            if !worker_cancelled.load(std::sync::atomic::Ordering::Acquire) {
                let _ = sender.send(result);
            }
        })
        .map_err(|error| format!("Could not start hardcopy source worker: {error}"))?;
    SOURCE_RESOLUTION.with(|active| {
        debug_assert!(active.borrow().is_none());
        *active.borrow_mut() = Some(ActiveSourceResolution {
            generation,
            purpose,
            source_key,
            scope,
            expected,
            cancelled,
            receiver,
        });
    });
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn start_source_resolution(
    prepared: crate::workbench::hardcopy_adapters::sources::PreparedRetainedHardcopyResolution,
    purpose: SourceResolutionPurpose,
    generation: u64,
    source_key: String,
    scope: crate::hardcopy::HardcopyScope,
    expected: Option<ResolvedHardcopyDocument>,
) -> Result<(), String> {
    let ticket = super::worker::start_source_resolution(
        prepared,
        source_key.clone(),
        scope.clone(),
        generation,
        generation,
        repaint_context(),
    )?;
    SOURCE_RESOLUTION.with(|active| {
        debug_assert!(active.borrow().is_none());
        *active.borrow_mut() = Some(ActiveBrowserSourceResolution {
            ticket,
            purpose,
            source_key,
            scope,
            expected,
        });
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn poll_source_resolution(app: &mut RSpiceApp) {
    let completed = SOURCE_RESOLUTION.with(|active| {
        let mut active = active.borrow_mut();
        let result = active
            .as_ref()
            .and_then(|worker| match worker.receiver.try_recv() {
                Ok(result) => Some((
                    worker.generation,
                    worker.purpose,
                    worker.source_key.clone(),
                    worker.scope.clone(),
                    worker.expected.clone(),
                    result,
                )),
                Err(std::sync::mpsc::TryRecvError::Disconnected) => Some((
                    worker.generation,
                    worker.purpose,
                    worker.source_key.clone(),
                    worker.scope.clone(),
                    worker.expected.clone(),
                    Err("The hardcopy source-resolution worker disconnected.".to_owned()),
                )),
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
            });
        if result.is_some() {
            active.take();
        }
        result
    });
    let Some((generation, purpose, source_key, scope, expected, result)) = completed else {
        return;
    };
    if !app.state.dialogs.hardcopy.open
        || app.state.dialogs.hardcopy.source_resolution_generation != generation
    {
        clear_pending_authentication(purpose);
        if app.state.dialogs.hardcopy.open {
            app.state.dialogs.hardcopy.busy = false;
        }
        return;
    }
    match result {
        Ok(resolved) if expected.is_none() => {
            let prepared =
                match crate::workbench::hardcopy_adapters::sources::prepare_retained_hardcopy_resolution(
                    &app.state,
                    &source_key,
                    scope.clone(),
                ) {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        app.state.dialogs.hardcopy.busy = false;
                        clear_pending_authentication(purpose);
                        app.state.dialogs.hardcopy.error = Some(error.to_string());
                        return;
                    }
                };
            if let Err(error) = start_source_resolution(
                prepared,
                purpose,
                generation,
                source_key,
                scope,
                Some(resolved),
            ) {
                app.state.dialogs.hardcopy.busy = false;
                clear_pending_authentication(purpose);
                app.state.dialogs.hardcopy.error = Some(error);
            }
        }
        Ok(resolved) => {
            if matches!(
                purpose,
                SourceResolutionPurpose::Open(_) | SourceResolutionPurpose::Select
            ) {
                app.state.dialogs.hardcopy.busy = false;
            }
            if expected.as_ref() != Some(&resolved) {
                app.state.dialogs.hardcopy.busy = false;
                clear_pending_authentication(purpose);
                app.state.dialogs.hardcopy.error = Some(
                    "The retained hardcopy source changed while it was being authenticated."
                        .to_owned(),
                );
                return;
            }
            match purpose {
                SourceResolutionPurpose::Open(workflow) => {
                    if let Err(error) = apply_open_resolved(app, workflow, resolved) {
                        app.state.dialogs.hardcopy.error = Some(error);
                    }
                }
                SourceResolutionPurpose::Select => {
                    if let Err(error) = apply_selected_resolved(app, resolved) {
                        app.state.dialogs.hardcopy.error = Some(error);
                    }
                }
                SourceResolutionPurpose::Publish => {
                    if let Err(error) = finish_publication_authentication(app, resolved) {
                        app.state.dialogs.hardcopy.busy = false;
                        clear_pending_publication();
                        app.state.dialogs.hardcopy.error = Some(error);
                    }
                }
                SourceResolutionPurpose::SavePageSetup => {
                    match finish_page_setup_authentication(app, resolved) {
                        Ok(message) => {
                            app.state.dialogs.hardcopy.busy = false;
                            app.state.push_user_message(ConsoleMessage::info(message));
                            app.state.dialogs.hardcopy.close();
                        }
                        Err(error) => {
                            app.state.dialogs.hardcopy.busy = false;
                            clear_pending_authentication(purpose);
                            app.state.dialogs.hardcopy.error = Some(error);
                        }
                    }
                }
            }
        }
        Err(error) => {
            app.state.dialogs.hardcopy.busy = false;
            clear_pending_authentication(purpose);
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn poll_source_resolution(app: &mut RSpiceApp) {
    let completed = SOURCE_RESOLUTION.with(|active| {
        let mut active = active.borrow_mut();
        let result = active.as_ref().and_then(|worker| {
            super::worker::poll(worker.ticket).map(|result| {
                (
                    worker.ticket.generation,
                    worker.purpose,
                    worker.source_key.clone(),
                    worker.scope.clone(),
                    worker.expected.clone(),
                    result.and_then(|buffers| {
                        if buffers.len() != 1 {
                            return Err(
                                "Browser hardcopy source resolution returned the wrong buffer count."
                                    .to_owned(),
                            );
                        }
                        ResolvedHardcopyDocument::from_worker_snapshot_json(&buffers[0])
                            .map_err(|error| error.to_string())
                    }),
                )
            })
        });
        if result.is_some() {
            active.take();
        }
        result
    });
    let Some((generation, purpose, source_key, scope, expected, result)) = completed else {
        return;
    };
    if !app.state.dialogs.hardcopy.open
        || app.state.dialogs.hardcopy.source_resolution_generation != generation
    {
        clear_pending_authentication(purpose);
        if app.state.dialogs.hardcopy.open {
            app.state.dialogs.hardcopy.busy = false;
        }
        return;
    }
    match result {
        Ok(resolved) if expected.is_none() => {
            let prepared =
                match crate::workbench::hardcopy_adapters::sources::prepare_retained_hardcopy_resolution(
                    &app.state,
                    &source_key,
                    scope.clone(),
                ) {
                    Ok(prepared) => prepared,
                    Err(error) => {
                        app.state.dialogs.hardcopy.busy = false;
                        clear_pending_authentication(purpose);
                        app.state.dialogs.hardcopy.error = Some(error.to_string());
                        return;
                    }
                };
            if let Err(error) = start_source_resolution(
                prepared,
                purpose,
                generation,
                source_key,
                scope,
                Some(resolved),
            ) {
                app.state.dialogs.hardcopy.busy = false;
                clear_pending_authentication(purpose);
                app.state.dialogs.hardcopy.error = Some(error);
            }
        }
        Ok(resolved) => {
            if matches!(
                purpose,
                SourceResolutionPurpose::Open(_) | SourceResolutionPurpose::Select
            ) {
                app.state.dialogs.hardcopy.busy = false;
            }
            if expected.as_ref() != Some(&resolved) {
                app.state.dialogs.hardcopy.busy = false;
                clear_pending_authentication(purpose);
                app.state.dialogs.hardcopy.error = Some(
                    "The retained hardcopy source changed while it was being authenticated."
                        .to_owned(),
                );
                return;
            }
            match purpose {
                SourceResolutionPurpose::Open(workflow) => {
                    if let Err(error) = apply_open_resolved(app, workflow, resolved) {
                        app.state.dialogs.hardcopy.error = Some(error);
                    }
                }
                SourceResolutionPurpose::Select => {
                    if let Err(error) = apply_selected_resolved(app, resolved) {
                        app.state.dialogs.hardcopy.error = Some(error);
                    }
                }
                SourceResolutionPurpose::Publish => {
                    if let Err(error) = finish_publication_authentication(app, resolved) {
                        app.state.dialogs.hardcopy.busy = false;
                        clear_pending_publication();
                        app.state.dialogs.hardcopy.error = Some(error);
                    }
                }
                SourceResolutionPurpose::SavePageSetup => {
                    match finish_page_setup_authentication(app, resolved) {
                        Ok(message) => {
                            app.state.dialogs.hardcopy.busy = false;
                            app.state.push_user_message(ConsoleMessage::info(message));
                            app.state.dialogs.hardcopy.close();
                        }
                        Err(error) => {
                            app.state.dialogs.hardcopy.busy = false;
                            clear_pending_authentication(purpose);
                            app.state.dialogs.hardcopy.error = Some(error);
                        }
                    }
                }
            }
        }
        Err(error) => {
            app.state.dialogs.hardcopy.busy = false;
            clear_pending_authentication(purpose);
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}

pub(super) fn cancel_source_resolution() {
    #[cfg(not(target_arch = "wasm32"))]
    SOURCE_RESOLUTION.with(|active| {
        if let Some(worker) = active.borrow_mut().take() {
            worker
                .cancelled
                .store(true, std::sync::atomic::Ordering::Release);
        }
    });
    #[cfg(target_arch = "wasm32")]
    SOURCE_RESOLUTION.with(|active| {
        if active.borrow_mut().take().is_some() {
            super::worker::cancel();
        }
    });
    PENDING_PAGE_SETUP.with(|slot| {
        slot.borrow_mut().take();
    });
}

fn apply_open_resolved(
    app: &mut RSpiceApp,
    workflow: HardcopyWorkflow,
    resolved: ResolvedHardcopyDocument,
) -> Result<(), String> {
    let saved = app
        .state
        .workspace
        .hardcopy_setups
        .setup_for(resolved.authority())
        .map_err(|error| error.to_string())?
        .map(|saved| saved.setup().clone());
    let metadata = metadata_for(app, &resolved).map_err(|error| error.to_string())?;
    let source_candidates = app.state.dialogs.hardcopy.source_candidates.clone();
    app.state
        .dialogs
        .hardcopy
        .open_resolved(workflow, resolved, saved.as_ref())
        .map_err(|error| error.to_string())?;
    app.state.dialogs.hardcopy.metadata = Some(metadata);
    app.state.dialogs.hardcopy.source_candidates = source_candidates;
    #[cfg(not(target_arch = "wasm32"))]
    if (workflow == HardcopyWorkflow::Print
        || app.state.dialogs.hardcopy.format == OutputFormat::NativePrinter)
        && crate::workbench::hardcopy_adapters::print::native_print_platform_support()
            == crate::workbench::hardcopy_adapters::print::NativePrintPlatformSupport::Available
    {
        refresh_printer_catalog(app);
    }
    Ok(())
}

fn apply_selected_resolved(
    app: &mut RSpiceApp,
    resolved: ResolvedHardcopyDocument,
) -> Result<(), String> {
    let metadata = metadata_for(app, &resolved).map_err(|error| error.to_string())?;
    let mapping = super::merge_print_mapping(
        resolved.default_print_mapping(),
        &app.state.dialogs.hardcopy.print_mapping,
    )
    .map_err(|error| error.to_string())?;
    let dialog = &mut app.state.dialogs.hardcopy;
    dialog.source = Some(resolved.authority().clone());
    dialog.content_extent = Some(resolved.content_extent());
    dialog.resolved_document = Some(std::sync::Arc::new(resolved));
    dialog.metadata = Some(metadata);
    dialog.print_mapping = mapping;
    dialog.preview_page = 0;
    dialog.error = None;
    dialog.refresh_preview();
    Ok(())
}

fn active_retained_source_selection(
    app: &RSpiceApp,
) -> Result<(String, crate::hardcopy::HardcopyScope), String> {
    use crate::hardcopy::HardcopyScope;
    use crate::workbench::SurfaceId;

    let candidates =
        crate::workbench::hardcopy_adapters::sources::enumerate_retained_hardcopy_sources(&app.state);
    let selected = match app.state.workbench.current_route().surface_id() {
        SurfaceId::Design => candidates.iter().find(|candidate| {
            candidate.source_key.contains(":cell-view:")
                && !candidate.source_key.contains(":sheet:")
        }),
        SurfaceId::Results => candidates
            .iter()
            .find(|candidate| candidate.source_key.contains(":result-dataset:")),
        SurfaceId::VisualizationStudio => {
            let pane_id = app
                .state
                .workbench
                .visualization_studio
                .active_pane
                .ok_or_else(|| "No visualization pane is active.".to_owned())?;
            let suffix = format!(":visualization-pane:{pane_id}");
            candidates
                .iter()
                .find(|candidate| candidate.source_key.ends_with(&suffix))
        }
        SurfaceId::ReportAuthoring => candidates
            .iter()
            .find(|candidate| candidate.source_key.contains(":report:")),
        _ => None,
    }
    .ok_or_else(|| "The active surface has no retained hardcopy source.".to_owned())?;
    if !selected.availability.is_available() {
        return Err("The active hardcopy source is not currently available.".to_owned());
    }
    let scope = selected
        .allowed_scopes
        .iter()
        .find(|scope| {
            matches!(
                scope,
                HardcopyScope::CurrentSheet
                    | HardcopyScope::VisibleHierarchy
                    | HardcopyScope::ActivePlotDocument
                    | HardcopyScope::CompleteReport
                    | HardcopyScope::ActiveDocument
            )
        })
        .cloned()
        .ok_or_else(|| "The active source has no publishable active extent.".to_owned())?;
    Ok((selected.source_key.clone(), scope))
}

fn metadata_for(
    app: &RSpiceApp,
    resolved: &ResolvedHardcopyDocument,
) -> Result<HardcopySceneMetadata, crate::workbench::hardcopy_adapters::render::HardcopyRenderError> {
    let mut metadata =
        HardcopySceneMetadata::try_new(resolved.authority().display_name(), "RSpice")?;
    metadata.set_publication_timestamp(HardcopyPublicationTimestamp::from_unix_seconds(
        crate::time_compat::unix_epoch().as_secs(),
    )?);
    let (header, provenance) = identity_lines(
        app.state.workspace.project.display_name(),
        &app.state.workspace.project.revision().get().to_string(),
        resolved.authority().display_name(),
        resolved.source_key(),
        &resolved.authority().revision().get().to_string(),
        &resolved.authority().content_digest().to_string(),
    );
    // The renderer reserves one physical line for each band. Keep all
    // identity fields on those exact single lines so enabled decorations
    // cannot overflow an otherwise valid page.
    metadata.set_header_lines(vec![header])?;
    metadata.set_provenance_lines(vec![provenance])?;
    Ok(metadata)
}

fn identity_lines(
    project: &str,
    project_revision: &str,
    source_name: &str,
    source_key: &str,
    source_revision: &str,
    source_digest: &str,
) -> (String, String) {
    (
        format!("{project} \u{00b7} revision {project_revision} \u{00b7} {source_name}"),
        format!(
            "Source {source_key} \u{00b7} revision {source_revision} \u{00b7} content {source_digest}"
        ),
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn refresh_printer_catalog(app: &mut RSpiceApp) {
    if crate::workbench::hardcopy_adapters::print::native_print_platform_support()
        != crate::workbench::hardcopy_adapters::print::NativePrintPlatformSupport::Available
    {
        app.state.dialogs.hardcopy.printer_report = None;
        app.state.dialogs.hardcopy.printer_capabilities = None;
        app.state.dialogs.hardcopy.clear_printer();
        app.state.dialogs.hardcopy.error =
            Some("Native printer discovery is available on Windows only.".to_owned());
        return;
    }
    if app.state.dialogs.hardcopy.printer_discovery_busy {
        return;
    }
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    match std::thread::Builder::new()
        .name("rspice-printer-discovery".to_owned())
        .spawn(move || {
            let _ = sender.send(discover_native_printers().map_err(|error| error.to_string()));
        }) {
        Ok(_) => {
            app.state.dialogs.hardcopy.printer_discovery_busy = true;
            PRINTER_DISCOVERY.with(|slot| *slot.borrow_mut() = Some(receiver));
        }
        Err(error) => {
            app.state.dialogs.hardcopy.error =
                Some(format!("Could not start printer discovery: {error}"));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn poll_printer_catalog(app: &mut RSpiceApp) {
    let result = PRINTER_DISCOVERY.with(|slot| {
        let mut slot = slot.borrow_mut();
        let result = slot
            .as_ref()
            .and_then(|receiver| match receiver.try_recv() {
                Ok(result) => Some(result),
                Err(std::sync::mpsc::TryRecvError::Disconnected) => Some(Err(
                    "The printer discovery worker stopped unexpectedly.".to_owned(),
                )),
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
            });
        if result.is_some() {
            *slot = None;
        }
        result
    });
    let Some(result) = result else {
        return;
    };
    app.state.dialogs.hardcopy.printer_discovery_busy = false;
    match result {
        Ok(report) => {
            let preferred = report
                .printers()
                .iter()
                .find(|entry| {
                    entry.capabilities().device_id() == app.state.dialogs.hardcopy.printer_id
                })
                .or_else(|| report.printers().iter().find(|entry| entry.is_default()))
                .or_else(|| report.printers().first())
                .map(|entry| entry.capabilities().clone());
            app.state.dialogs.hardcopy.printer_report = Some(report);
            match preferred {
                Some(capabilities) => select_printer_capabilities(app, capabilities, None),
                None => {
                    app.state.dialogs.hardcopy.clear_printer();
                    app.state.dialogs.hardcopy.printer_capabilities = None;
                    app.state.dialogs.hardcopy.error = Some(
                        "Windows did not report an available printer with an authenticated capability snapshot."
                            .to_owned(),
                    );
                }
            }
        }
        Err(error) => {
            app.state.dialogs.hardcopy.printer_report = None;
            app.state.dialogs.hardcopy.printer_capabilities = None;
            app.state.dialogs.hardcopy.clear_printer();
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn poll_printer_catalog(_app: &mut RSpiceApp) {}

pub(super) fn select_printer_capabilities(
    app: &mut RSpiceApp,
    capabilities: PrinterCapabilitySnapshot,
    suggested: Option<crate::workbench::hardcopy_adapters::print::PrinterDriverSettingsSuggestion>,
) {
    let previous = app.state.dialogs.hardcopy.printer_job.clone();
    let media_source = suggested
        .as_ref()
        .map(|value| value.media_source.clone())
        .or_else(|| previous.as_ref().map(|value| value.media_source().clone()))
        .filter(|source| media_supported(source, &capabilities))
        .unwrap_or(PrinterMediaSource::AutomaticCompatibleTray);
    let resolution = suggested
        .as_ref()
        .and_then(|value| value.resolution_dpi)
        .or_else(|| previous.as_ref().map(PrinterJobSettings::resolution_dpi))
        .filter(|dpi| {
            capabilities.resolutions().iter().any(|resolution| {
                resolution.horizontal_dpi() == *dpi && resolution.vertical_dpi() == *dpi
            })
        })
        .or_else(|| preferred_default_resolution(&capabilities))
        .unwrap_or(300);
    let duplex = suggested
        .as_ref()
        .map(|value| value.duplex)
        .or_else(|| previous.as_ref().map(PrinterJobSettings::duplex))
        .filter(|duplex| capabilities.duplex_modes().contains(duplex))
        .unwrap_or(DuplexMode::Off);
    let copies = suggested
        .as_ref()
        .map(|value| value.copies)
        .or_else(|| previous.as_ref().map(PrinterJobSettings::copies))
        .unwrap_or(1)
        .clamp(1, capabilities.maximum_copies());
    let collate = copies > 1
        && capabilities.supports_collation()
        && suggested
            .as_ref()
            .map(|value| value.collate)
            .or_else(|| previous.as_ref().map(PrinterJobSettings::collate))
            .unwrap_or(false);
    let selected_paper_id = suggested
        .as_ref()
        .and_then(|value| value.paper_platform_id)
        .map(|value| value.to_string())
        .or_else(|| selected_printer_paper_id(app, &capabilities));
    let Some(selected_paper_id) = selected_paper_id else {
        app.state.dialogs.hardcopy.printer_capabilities = Some(capabilities);
        app.state.dialogs.hardcopy.clear_printer();
        app.state.dialogs.hardcopy.error = Some(
            "The selected printer does not report the page size configured in Custom paper and margins."
                .to_owned(),
        );
        return;
    };
    let orientation = resolved_orientation(app);
    let raster_geometry = match crate::workbench::hardcopy_adapters::print::resolve_native_printer_mode(
        &capabilities,
        &selected_paper_id,
        resolution,
        orientation,
    ) {
        Ok(geometry) => geometry,
        Err(error) => {
            app.state.dialogs.hardcopy.printer_capabilities = Some(capabilities);
            app.state.dialogs.hardcopy.clear_printer();
            app.state.dialogs.hardcopy.error = Some(error.to_string());
            return;
        }
    };
    match PrinterJobSettings::try_new(
        capabilities.content_digest(),
        selected_paper_id,
        raster_geometry,
        media_source,
        resolution,
        duplex,
        copies,
        collate,
    ) {
        Ok(job) => {
            let id = capabilities.device_id().to_owned();
            app.state.dialogs.hardcopy.printer_capabilities = Some(capabilities);
            app.state.dialogs.hardcopy.set_printer(id, job);
        }
        Err(error) => {
            app.state.dialogs.hardcopy.printer_capabilities = None;
            app.state.dialogs.hardcopy.clear_printer();
            app.state.dialogs.hardcopy.error = Some(error.to_string());
        }
    }
}

fn preferred_default_resolution(capabilities: &PrinterCapabilitySnapshot) -> Option<u16> {
    preferred_resolution(
        capabilities
            .resolutions()
            .iter()
            .filter(|resolution| resolution.horizontal_dpi() == resolution.vertical_dpi())
            .map(|resolution| resolution.horizontal_dpi())
            .collect(),
    )
}

fn preferred_resolution(mut square: Vec<u16>) -> Option<u16> {
    square.sort_unstable();
    square.dedup();
    if square.binary_search(&600).is_ok() {
        Some(600)
    } else {
        square
            .iter()
            .copied()
            .filter(|dpi| *dpi <= 1_200)
            .max()
            .or_else(|| square.first().copied())
    }
}

fn selected_printer_paper_id(
    app: &RSpiceApp,
    capabilities: &PrinterCapabilitySnapshot,
) -> Option<String> {
    let paper = app
        .state
        .dialogs
        .hardcopy
        .paper
        .build(app.state.dialogs.hardcopy.display_unit)
        .ok()?;
    let (width, height) = paper.portrait_dimensions();
    capabilities
        .papers()
        .iter()
        .find(|candidate| {
            let (candidate_width, candidate_height) = candidate.portrait_dimensions_um();
            candidate_width.abs_diff(width.micrometres()) <= 100
                && candidate_height.abs_diff(height.micrometres()) <= 100
        })
        .map(|paper| paper.platform_id().to_string())
}

fn resolved_orientation(app: &RSpiceApp) -> ResolvedOrientation {
    match app.state.dialogs.hardcopy.orientation {
        Orientation::Portrait => ResolvedOrientation::Portrait,
        Orientation::Landscape => ResolvedOrientation::Landscape,
        Orientation::AutomaticPerPage => {
            if app
                .state
                .dialogs
                .hardcopy
                .content_extent
                .is_some_and(|extent| extent.width() > extent.height())
            {
                ResolvedOrientation::Landscape
            } else {
                ResolvedOrientation::Portrait
            }
        }
    }
}

fn media_supported(source: &PrinterMediaSource, capabilities: &PrinterCapabilitySnapshot) -> bool {
    match source {
        PrinterMediaSource::AutomaticCompatibleTray => true,
        PrinterMediaSource::NamedTray(name) => capabilities
            .trays()
            .iter()
            .any(|tray| tray.display_name() == name),
        PrinterMediaSource::ManualFeed => capabilities
            .trays()
            .iter()
            .any(|tray| matches!(tray.platform_id(), 4 | 6)),
        PrinterMediaSource::Roll { width } => capabilities.papers().iter().any(|paper| {
            let (paper_width, _) = paper.portrait_dimensions_um();
            paper_width.abs_diff(width.micrometres()) <= 100
        }),
    }
}

pub(super) fn commit_hardcopy_workflow(app: &mut RSpiceApp) {
    if app.state.dialogs.hardcopy.busy {
        return;
    }
    app.state.dialogs.hardcopy.busy = true;
    app.state.dialogs.hardcopy.error = None;
    app.state.dialogs.hardcopy.cancellation = HardcopyCancellationToken::default();
    let workflow = app.state.dialogs.hardcopy.workflow;
    #[cfg(target_arch = "wasm32")]
    let browser_print_reservation = if workflow == HardcopyWorkflow::Print
        && app.state.dialogs.hardcopy.format == OutputFormat::BrowserPrintDocument
    {
        match reserve_browser_print_window() {
            Ok(reservation) => Some(reservation),
            Err(error) => {
                app.state.dialogs.hardcopy.busy = false;
                app.state.dialogs.hardcopy.error = Some(error.to_string());
                return;
            }
        }
    } else {
        None
    };
    let result = match workflow {
        HardcopyWorkflow::PageSetup => save_page_setup(app),
        HardcopyWorkflow::Print => {
            #[cfg(target_arch = "wasm32")]
            {
                publish_print(app, browser_print_reservation)
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                publish_print(app)
            }
        }
        HardcopyWorkflow::Export => publish_export(app),
    };
    match result {
        Ok(PublicationCompletion::Pending) => {}
        Ok(PublicationCompletion::Cancelled) => {
            app.state.dialogs.hardcopy.busy = false;
        }
        Err(error) => {
            app.state.dialogs.hardcopy.busy = false;
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}

enum PublicationCompletion {
    Cancelled,
    Pending,
}

enum StagedPrintMappingPersistence {
    Document,
    Project {
        catalog: crate::hardcopy::PrintMappingPresetCatalog,
        changed: bool,
    },
    Personal(crate::hardcopy::PrintMappingPresetCatalog),
}

fn stage_print_mapping_persistence(
    app: &RSpiceApp,
    mapping: &crate::hardcopy::PrintMappingTable,
) -> Result<StagedPrintMappingPersistence, String> {
    match mapping.save_scope() {
        crate::hardcopy::PrintMappingSaveScope::Document => {
            Ok(StagedPrintMappingPersistence::Document)
        }
        crate::hardcopy::PrintMappingSaveScope::ProjectPrintSet(_) => {
            let mut catalog = app.state.workspace.project_print_mappings.clone();
            let receipt = catalog
                .save(mapping.clone())
                .map_err(|error| error.to_string())?;
            Ok(StagedPrintMappingPersistence::Project {
                catalog,
                changed: receipt.disposition()
                    != crate::hardcopy::PrintMappingSaveDisposition::Unchanged,
            })
        }
        crate::hardcopy::PrintMappingSaveScope::PortablePersonalPreset(_) => {
            let mut catalog = app
                .state
                .ui
                .preferences
                .personal_print_mapping_presets()
                .cloned()
                .ok_or_else(|| {
                    "Personal print mappings were written by an incompatible build.".to_owned()
                })?;
            catalog
                .save(mapping.clone())
                .map_err(|error| error.to_string())?;
            Ok(StagedPrintMappingPersistence::Personal(catalog))
        }
    }
}

fn commit_print_mapping_persistence(
    app: &mut RSpiceApp,
    staged: StagedPrintMappingPersistence,
) -> Result<(), String> {
    match staged {
        StagedPrintMappingPersistence::Document => Ok(()),
        StagedPrintMappingPersistence::Project { catalog, changed } => {
            app.state.workspace.project_print_mappings = catalog;
            app.state.workspace.project_print_mappings_dirty |= changed;
            Ok(())
        }
        StagedPrintMappingPersistence::Personal(catalog) => app
            .state
            .ui
            .preferences
            .replace_personal_print_mapping_presets(catalog)
            .map_err(|error| error.to_string()),
    }
}

fn current_plan(
    app: &RSpiceApp,
    source: &ResolvedHardcopyDocument,
) -> Result<std::sync::Arc<HardcopyPlan>, String> {
    let setup = app
        .state
        .dialogs
        .hardcopy
        .build_setup()
        .map_err(|error| error.to_string())?;
    let preview_plan = app
        .state
        .dialogs
        .hardcopy
        .preview_plan
        .as_ref()
        .ok_or_else(|| "The hardcopy preview has no sealed publication plan.".to_owned())?;
    let sections = source
        .hardcopy_sections()
        .map_err(|error| error.to_string())?;
    let candidate = if sections.is_empty() {
        HardcopyPlan::compile_with_id(
            preview_plan.id(),
            source.authority().clone(),
            setup,
            source.content_extent(),
        )
    } else {
        HardcopyPlan::compile_with_id_and_sections(
            preview_plan.id(),
            source.authority().clone(),
            setup,
            source.content_extent(),
            sections,
        )
    }
    .map_err(|error| error.to_string())?;
    if candidate != **preview_plan {
        return Err(
            "The hardcopy source or settings no longer match the sealed preview plan. Review the updated preview before publishing."
                .to_owned(),
        );
    }
    Ok(preview_plan.clone())
}

fn current_metadata(app: &RSpiceApp) -> Result<HardcopySceneMetadata, String> {
    app.state
        .dialogs
        .hardcopy
        .metadata
        .clone()
        .ok_or_else(|| "The dialog has no validated hardcopy identity block.".to_owned())
}

#[cfg(target_arch = "wasm32")]
fn begin_publication_authentication(
    app: &mut RSpiceApp,
    pending: PendingBrowserPublication,
) -> Result<(), String> {
    if PENDING_PUBLICATION.with(|slot| slot.borrow().is_some()) {
        return Err("A browser hardcopy publication is already being authenticated.".to_owned());
    }
    let prepared = crate::workbench::hardcopy_adapters::sources::prepare_retained_hardcopy_resolution(
        &app.state,
        pending.opened_source.source_key(),
        pending.opened_source.authority().scope().clone(),
    )
    .map_err(|error| error.to_string())?;
    let generation = app
        .state
        .dialogs
        .hardcopy
        .next_source_resolution_generation();
    start_source_resolution(
        prepared,
        SourceResolutionPurpose::Publish,
        generation,
        pending.opened_source.source_key().to_owned(),
        pending.opened_source.authority().scope().clone(),
        None,
    )?;
    PENDING_PUBLICATION.with(|slot| *slot.borrow_mut() = Some(pending));
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn begin_publication_authentication(
    app: &mut RSpiceApp,
    pending: PendingNativePublication,
) -> Result<(), String> {
    if PENDING_PUBLICATION.with(|slot| slot.borrow().is_some()) {
        return Err("A native hardcopy publication is already being authenticated.".to_owned());
    }
    let prepared = crate::workbench::hardcopy_adapters::sources::prepare_retained_hardcopy_resolution(
        &app.state,
        pending.opened_source.source_key(),
        pending.opened_source.authority().scope().clone(),
    )
    .map_err(|error| error.to_string())?;
    let generation = app
        .state
        .dialogs
        .hardcopy
        .next_source_resolution_generation();
    start_source_resolution(
        prepared,
        SourceResolutionPurpose::Publish,
        generation,
        pending.opened_source.source_key().to_owned(),
        pending.opened_source.authority().scope().clone(),
        None,
    )?;
    PENDING_PUBLICATION.with(|slot| *slot.borrow_mut() = Some(pending));
    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn finish_publication_authentication(
    app: &RSpiceApp,
    resolved: ResolvedHardcopyDocument,
) -> Result<(), String> {
    let pending = PENDING_PUBLICATION
        .with(|slot| slot.borrow_mut().take())
        .ok_or_else(|| "The browser publication authentication owner was lost.".to_owned())?;
    if resolved != *pending.opened_source {
        return Err(
            "The retained hardcopy source changed while publication was being authenticated."
                .to_owned(),
        );
    }
    let plan = current_plan(app, &resolved)?;
    if plan != pending.plan {
        return Err(
            "The hardcopy plan changed while publication was being authenticated.".to_owned(),
        );
    }
    start_browser_publication(
        app,
        plan,
        std::sync::Arc::new(resolved),
        pending.metadata,
        pending.staged_mapping,
        pending.destination,
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn finish_publication_authentication(
    app: &RSpiceApp,
    resolved: ResolvedHardcopyDocument,
) -> Result<(), String> {
    let pending = PENDING_PUBLICATION
        .with(|slot| slot.borrow_mut().take())
        .ok_or_else(|| "The native publication authentication owner was lost.".to_owned())?;
    if resolved != *pending.opened_source {
        return Err(
            "The retained hardcopy source changed while publication was being authenticated."
                .to_owned(),
        );
    }
    let plan = current_plan(app, &resolved)?;
    if plan != pending.plan {
        return Err(
            "The hardcopy plan changed while publication was being authenticated.".to_owned(),
        );
    }
    start_native_publication(
        app,
        plan,
        std::sync::Arc::new(resolved),
        pending.metadata,
        pending.staged_mapping,
        pending.operation,
        pending.destination,
    )
}

fn clear_pending_publication() {
    PENDING_PUBLICATION.with(|slot| {
        slot.borrow_mut().take();
    });
}

fn clear_pending_authentication(purpose: SourceResolutionPurpose) {
    match purpose {
        SourceResolutionPurpose::Publish => clear_pending_publication(),
        SourceResolutionPurpose::SavePageSetup => {
            PENDING_PAGE_SETUP.with(|slot| {
                slot.borrow_mut().take();
            });
        }
        SourceResolutionPurpose::Open(_) | SourceResolutionPurpose::Select => {}
    }
}

fn save_page_setup(app: &mut RSpiceApp) -> Result<PublicationCompletion, String> {
    if PENDING_PAGE_SETUP.with(|slot| slot.borrow().is_some()) {
        return Err("A page setup is already being authenticated.".to_owned());
    }
    let source = app
        .state
        .dialogs
        .hardcopy
        .resolved_document
        .clone()
        .ok_or_else(|| "The dialog has no authenticated semantic source.".to_owned())?;
    let setup = app
        .state
        .dialogs
        .hardcopy
        .build_setup()
        .map_err(|error| error.to_string())?;
    let staged_mapping = stage_print_mapping_persistence(app, setup.print_mapping())?;
    let prepared = crate::workbench::hardcopy_adapters::sources::prepare_retained_hardcopy_resolution(
        &app.state,
        source.source_key(),
        source.authority().scope().clone(),
    )
    .map_err(|error| error.to_string())?;
    let generation = app
        .state
        .dialogs
        .hardcopy
        .next_source_resolution_generation();
    start_source_resolution(
        prepared,
        SourceResolutionPurpose::SavePageSetup,
        generation,
        source.source_key().to_owned(),
        source.authority().scope().clone(),
        None,
    )?;
    PENDING_PAGE_SETUP.with(|slot| {
        *slot.borrow_mut() = Some(PendingPageSetup {
            opened_source: source,
            setup,
            staged_mapping,
        });
    });
    Ok(PublicationCompletion::Pending)
}

fn finish_page_setup_authentication(
    app: &mut RSpiceApp,
    source: ResolvedHardcopyDocument,
) -> Result<String, String> {
    let pending = PENDING_PAGE_SETUP
        .with(|slot| slot.borrow_mut().take())
        .ok_or_else(|| "The page-setup authentication owner was lost.".to_owned())?;
    if source != *pending.opened_source {
        return Err(
            "The retained hardcopy source changed while page setup was being authenticated."
                .to_owned(),
        );
    }
    let mut staged_setups = app.state.workspace.hardcopy_setups.clone();
    let outcome = staged_setups
        .save(source.authority(), pending.setup)
        .map_err(|error| error.to_string())?;
    commit_print_mapping_persistence(app, pending.staged_mapping)?;
    app.state.workspace.hardcopy_setups = staged_setups;
    if outcome.disposition() != crate::hardcopy::SetupSaveDisposition::Unchanged {
        app.state.workspace.hardcopy_setups_dirty = true;
    }
    Ok(format!(
        "Page setup {} for {}.",
        match outcome.disposition() {
            crate::hardcopy::SetupSaveDisposition::Inserted => "saved",
            crate::hardcopy::SetupSaveDisposition::Updated => "updated",
            crate::hardcopy::SetupSaveDisposition::Unchanged => "already matched",
        },
        source.authority().display_name()
    ))
}

fn publish_print(
    app: &mut RSpiceApp,
    #[cfg(target_arch = "wasm32")] reservation: Option<BrowserPrintReservation>,
) -> Result<PublicationCompletion, String> {
    if !matches!(
        app.state.dialogs.hardcopy.format,
        OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument
    ) {
        return publish_export(app);
    }
    let source = app
        .state
        .dialogs
        .hardcopy
        .resolved_document
        .clone()
        .ok_or_else(|| "The dialog has no authenticated semantic source.".to_owned())?;
    let plan = current_plan(app, &source)?;
    let metadata = current_metadata(app)?;
    let staged_mapping = stage_print_mapping_persistence(app, plan.setup().print_mapping())?;
    #[cfg(target_arch = "wasm32")]
    {
        let reservation = reservation.ok_or_else(|| {
            "The browser print window was not reserved by the user action.".to_owned()
        })?;
        begin_publication_authentication(
            app,
            PendingBrowserPublication {
                plan,
                opened_source: source,
                metadata,
                staged_mapping,
                destination: BrowserPublicationDestination::Print { reservation },
            },
        )?;
        return Ok(PublicationCompletion::Pending);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Rendering uses the exact capability snapshot already authenticated
        // by the dialog. The finalizer re-discovers and digest-checks the
        // device off the UI thread immediately before the spool boundary.
        let capabilities = app
            .state
            .dialogs
            .hardcopy
            .printer_capabilities
            .clone()
            .ok_or_else(|| "The selected printer has no capability snapshot.".to_owned())?;
        if capabilities.device_id() != app.state.dialogs.hardcopy.printer_id {
            return Err(
                "The selected printer identity does not match its authenticated capabilities."
                    .to_owned(),
            );
        }
        let dpi = app
            .state
            .dialogs
            .hardcopy
            .printer_job
            .as_ref()
            .ok_or_else(|| "Select printer properties before printing.".to_owned())?
            .resolution_dpi();
        let cancellation = app.state.dialogs.hardcopy.cancellation.clone();
        begin_publication_authentication(
            app,
            PendingNativePublication {
                plan,
                opened_source: source,
                metadata,
                staged_mapping,
                operation: super::execution::ExecutionOperation::NativePrinter { dpi },
                destination: NativePublicationDestination::Print {
                    printer_id: capabilities.device_id().to_owned(),
                    capabilities_digest: capabilities.content_digest(),
                    cancellation,
                },
            },
        )?;
        Ok(PublicationCompletion::Pending)
    }
}

fn publish_export(app: &mut RSpiceApp) -> Result<PublicationCompletion, String> {
    let initial_source = app
        .state
        .dialogs
        .hardcopy
        .resolved_document
        .clone()
        .ok_or_else(|| "The dialog has no authenticated semantic source.".to_owned())?;
    let draft_plan = current_plan(app, &initial_source)?;
    let format = draft_plan.setup().render().format();
    let multi_part = matches!(format, OutputFormat::SvgVector | OutputFormat::Png { .. })
        && draft_plan.pagination().pages().len() > 1;
    let (extension, media_type, filter_name) = if multi_part {
        ("zip", "application/zip", "ZIP hardcopy package")
    } else {
        format_file_contract(format)?
    };
    let stem = safe_filename(initial_source.authority().display_name());
    let default_name = format!("{stem}.{extension}");
    let Some(mut path) = app.export_workflow_io.show_save_dialog(SaveDialogConfig {
        title: "Export active view",
        default_name: &default_name,
        filter_name,
        filter_extensions: &[extension],
    })?
    else {
        return Ok(PublicationCompletion::Cancelled);
    };
    crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, extension);
    let destination = app.export_workflow_io.observe_destination(&path)?;

    // The picker may have yielded. The exact current source is authenticated
    // twice off-thread before rendering; the original preview plan remains
    // the immutable authority for that continuation.
    let staged_mapping = stage_print_mapping_persistence(app, draft_plan.setup().print_mapping())?;
    #[cfg(target_arch = "wasm32")]
    {
        let metadata = current_metadata(app)?;
        begin_publication_authentication(
            app,
            PendingBrowserPublication {
                plan: draft_plan,
                opened_source: initial_source,
                metadata,
                staged_mapping,
                destination: BrowserPublicationDestination::Export {
                    path,
                    destination,
                    media_type,
                    multi_part,
                },
            },
        )?;
        return Ok(PublicationCompletion::Pending);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let metadata = current_metadata(app)?;
        begin_publication_authentication(
            app,
            PendingNativePublication {
                plan: draft_plan,
                opened_source: initial_source,
                metadata,
                staged_mapping,
                operation: super::execution::ExecutionOperation::Publication,
                destination: NativePublicationDestination::Export {
                    path,
                    destination,
                    media_type,
                    multi_part,
                },
            },
        )?;
        Ok(PublicationCompletion::Pending)
    }
}

#[cfg(target_arch = "wasm32")]
fn start_browser_publication(
    app: &RSpiceApp,
    plan: std::sync::Arc<HardcopyPlan>,
    source: std::sync::Arc<ResolvedHardcopyDocument>,
    metadata: HardcopySceneMetadata,
    staged_mapping: StagedPrintMappingPersistence,
    destination: BrowserPublicationDestination,
) -> Result<(), String> {
    if PUBLICATION.with(|active| active.borrow().is_some()) || super::worker::is_active() {
        return Err("A browser hardcopy operation is already active.".to_owned());
    }
    let package_multi_part = matches!(
        &destination,
        BrowserPublicationDestination::Export {
            multi_part: true,
            ..
        }
    );
    let ticket = super::worker::start_publication(
        &plan,
        &source,
        metadata,
        package_multi_part,
        app.state.dialogs.hardcopy.source_resolution_generation,
        app.state.dialogs.hardcopy.preview_generation,
        repaint_context(),
    )?;
    PUBLICATION.with(|active| {
        *active.borrow_mut() = Some(ActiveBrowserPublication {
            ticket,
            plan,
            source,
            staged_mapping,
            destination,
        });
    });
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub(super) fn poll_publication(app: &mut RSpiceApp) {
    let completed = PUBLICATION.with(|active| {
        let mut active = active.borrow_mut();
        let result = active
            .as_ref()
            .and_then(|publication| super::worker::poll(publication.ticket).map(|result| result));
        result.map(|result| {
            (
                active
                    .take()
                    .expect("completed browser publication remains owned"),
                result,
            )
        })
    });
    let Some((publication, result)) = completed else {
        return;
    };
    let current = &app.state.dialogs.hardcopy;
    if !current.open
        || current.source_resolution_generation != publication.ticket.epoch
        || current.preview_generation != publication.ticket.generation
        || current
            .preview_plan
            .as_ref()
            .map(|plan| plan.content_digest())
            != Some(publication.ticket_plan_digest())
        || current
            .resolved_document
            .as_ref()
            .map(|source| source.authority().content_digest())
            != Some(publication.ticket_source_digest())
    {
        app.state.dialogs.hardcopy.busy = false;
        if app.state.dialogs.hardcopy.open {
            app.state.dialogs.hardcopy.error = Some(
                "The hardcopy source or plan changed before browser publication completed."
                    .to_owned(),
            );
        }
        return;
    }
    let plan = publication.plan;
    let source = publication.source;
    let buffers = match result {
        Ok(buffers) => buffers,
        Err(error) => {
            app.state.dialogs.hardcopy.busy = false;
            app.state.dialogs.hardcopy.error = Some(record_render_failure(app, &plan, error));
            return;
        }
    };

    let completion = match publication.destination {
        BrowserPublicationDestination::Print { reservation } => (|| {
            let rendered = decode_browser_publication(&plan, &source, buffers)
                .map_err(|error| record_render_failure(app, &plan, error))?;
            let outcome = finalize_browser_print(reservation, &plan, &rendered)
                .map_err(|error| record_print_failure(app, &plan, error, 0));
            outcome.and_then(|outcome| {
                let receipt =
                    HardcopyReceipt::record(&plan, outcome).map_err(|error| error.to_string())?;
                app.state.dialogs.hardcopy.last_receipt = Some(receipt);
                commit_print_mapping_persistence(app, publication.staged_mapping)?;
                let pages = rendered.page_count();
                Ok(format!(
                    "Browser print handoff accepted for {pages} page{}; confirm the browser print dialog.",
                    if pages == 1 { "" } else { "s" }
                ))
            })
        })(),
        BrowserPublicationDestination::Export {
            path,
            destination,
            media_type,
            multi_part,
        } => {
            if multi_part {
                super::worker::decode_packaged_publication(&plan, &source, buffers)
                    .map_err(|error| record_render_failure(app, &plan, error))
                    .and_then(|packaged| {
                        app.export_workflow_io
                            .write_bytes_file_observed(&destination, &packaged.bytes, media_type)
                            .map_err(|error| {
                                record_export_failure(
                                    app,
                                    &plan,
                                    format!("Could not publish hardcopy: {error}"),
                                )
                            })?;
                        let receipt = HardcopyReceipt::record(
                            &plan,
                            HardcopyOutcome::ArtifactExported {
                                artifact: packaged.artifact,
                            },
                        )
                        .map_err(|error| error.to_string())?;
                        app.state.dialogs.hardcopy.last_receipt = Some(receipt);
                        commit_print_mapping_persistence(app, publication.staged_mapping)?;
                        Ok(export_completion_message(
                            "hardcopy",
                            &path,
                            Some(format!(
                                "{} page{} \u{00b7} deterministic ZIP package",
                                packaged.page_count,
                                if packaged.page_count == 1 { "" } else { "s" },
                            )),
                            app.export_workflow_io.as_ref(),
                        ))
                    })
            } else {
                decode_browser_publication(&plan, &source, buffers)
                    .map_err(|error| record_render_failure(app, &plan, error))
                    .and_then(|rendered| {
                        export_bytes_and_identity(&rendered, &plan, false).and_then(
                            |(bytes, artifact)| {
                                app.export_workflow_io
                                    .write_bytes_file_observed(&destination, &bytes, media_type)
                                    .map_err(|error| {
                                        record_export_failure(
                                            app,
                                            &plan,
                                            format!("Could not publish hardcopy: {error}"),
                                        )
                                    })?;
                                let receipt = HardcopyReceipt::record(
                                    &plan,
                                    HardcopyOutcome::ArtifactExported { artifact },
                                )
                                .map_err(|error| error.to_string())?;
                                app.state.dialogs.hardcopy.last_receipt = Some(receipt);
                                commit_print_mapping_persistence(app, publication.staged_mapping)?;
                                Ok(export_completion_message(
                                    "hardcopy",
                                    &path,
                                    Some(format!(
                                        "{} page{} \u{00b7} {}",
                                        rendered.page_count(),
                                        if rendered.page_count() == 1 { "" } else { "s" },
                                        if rendered.format().is_vector() {
                                            "vector"
                                        } else {
                                            "raster"
                                        }
                                    )),
                                    app.export_workflow_io.as_ref(),
                                ))
                            },
                        )
                    })
            }
        }
    };
    app.state.dialogs.hardcopy.busy = false;
    match completion {
        Ok(message) => {
            app.state.push_user_message(ConsoleMessage::info(message));
            app.state.dialogs.hardcopy.close();
        }
        Err(error) => {
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn decode_browser_publication(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    mut buffers: Vec<Vec<u8>>,
) -> Result<RenderedHardcopyPublication, String> {
    if buffers.len() < 2 {
        return Err("Browser publication returned no artifact payload.".to_owned());
    }
    let manifest = buffers.remove(0);
    RenderedHardcopyPublication::from_worker_transfer(plan, source, &manifest, buffers)
        .map_err(|error| error.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn start_native_publication(
    app: &RSpiceApp,
    plan: std::sync::Arc<HardcopyPlan>,
    source: std::sync::Arc<ResolvedHardcopyDocument>,
    metadata: HardcopySceneMetadata,
    staged_mapping: StagedPrintMappingPersistence,
    operation: super::execution::ExecutionOperation,
    destination: NativePublicationDestination,
) -> Result<(), String> {
    if PUBLICATION.with(|active| active.borrow().is_some())
        || FINALIZATION.with(|active| active.borrow().is_some())
        || super::execution::is_active()
        || super::finalize::is_active()
    {
        return Err("A native hardcopy operation is already active.".to_owned());
    }
    let ticket = super::execution::start(
        plan.clone(),
        source.clone(),
        metadata,
        app.state.dialogs.hardcopy.preview_generation,
        operation,
        repaint_context(),
    )?;
    PUBLICATION.with(|active| {
        *active.borrow_mut() = Some(ActiveNativePublication {
            ticket,
            plan,
            staged_mapping,
            destination,
        });
    });
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn poll_publication(app: &mut RSpiceApp) {
    if poll_native_finalization(app) {
        return;
    }
    let completed = PUBLICATION.with(|active| {
        let mut active = active.borrow_mut();
        let result = active
            .as_ref()
            .and_then(|publication| super::execution::poll(publication.ticket));
        result.map(|result| {
            (
                active
                    .take()
                    .expect("completed native publication remains owned"),
                result,
            )
        })
    });
    let Some((publication, result)) = completed else {
        return;
    };
    let current = &app.state.dialogs.hardcopy;
    if current.open
        && (current.preview_generation != publication.ticket.generation
            || current
                .preview_plan
                .as_ref()
                .map(|plan| plan.content_digest())
                != Some(publication.ticket.plan_digest)
            || current
                .resolved_document
                .as_ref()
                .map(|source| source.authority().content_digest())
                != Some(publication.ticket.source_digest))
    {
        app.state.dialogs.hardcopy.busy = false;
        if app.state.dialogs.hardcopy.open {
            app.state.dialogs.hardcopy.error = Some(
                "The hardcopy source or plan changed before native rendering completed.".to_owned(),
            );
        }
        return;
    }
    let plan = publication.plan;
    let operation = match result {
        Err(error) => Err(record_render_failure(app, &plan, error)),
        Ok(completion) if completion.ticket != publication.ticket => Err(record_render_failure(
            app,
            &plan,
            "Native hardcopy rendering returned a stale completion ticket.".to_owned(),
        )),
        Ok(completion) => match (publication.destination, completion.payload) {
            (
                NativePublicationDestination::Print {
                    printer_id,
                    capabilities_digest,
                    cancellation,
                },
                super::execution::ExecutionPayload::NativePrinter(pages),
            ) => Ok((
                super::finalize::FinalizationOperation::Print {
                    pages,
                    printer_id: printer_id.clone(),
                    capabilities_digest,
                    cancellation: cancellation.clone(),
                },
                NativePublicationDestination::Print {
                    printer_id,
                    capabilities_digest,
                    cancellation,
                },
            )),
            (
                NativePublicationDestination::Export {
                    path,
                    destination,
                    media_type,
                    multi_part,
                },
                super::execution::ExecutionPayload::Publication(rendered),
            ) => Ok((
                super::finalize::FinalizationOperation::Export {
                    publication: rendered,
                    destination: destination.clone(),
                    media_type,
                    multi_part,
                },
                NativePublicationDestination::Export {
                    path,
                    destination,
                    media_type,
                    multi_part,
                },
            )),
            _ => Err(record_render_failure(
                app,
                &plan,
                "Native hardcopy rendering returned the wrong payload type.".to_owned(),
            )),
        },
    };
    match operation {
        Ok((operation, destination)) => {
            let ticket =
                match super::finalize::start(
                    plan.clone(),
                    publication.ticket.generation,
                    operation,
                    repaint_context(),
                ) {
                    Ok(ticket) => ticket,
                    Err(error) => {
                        app.state.dialogs.hardcopy.busy = false;
                        app.state.dialogs.hardcopy.error = Some(
                            record_native_finalization_failure(app, &plan, &destination, error),
                        );
                        return;
                    }
                };
            FINALIZATION.with(|active| {
                *active.borrow_mut() = Some(ActiveNativeFinalization {
                    ticket,
                    plan,
                    source_digest: publication.ticket.source_digest,
                    staged_mapping: publication.staged_mapping,
                    destination,
                });
            });
        }
        Err(error) => {
            app.state.dialogs.hardcopy.busy = false;
            app.state.dialogs.hardcopy.error = Some(error);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn poll_native_finalization(app: &mut RSpiceApp) -> bool {
    let completed = FINALIZATION.with(|active| {
        let mut active = active.borrow_mut();
        let result = active
            .as_ref()
            .and_then(|publication| super::finalize::poll(publication.ticket));
        result.map(|result| {
            (
                active
                    .take()
                    .expect("completed native finalization remains owned"),
                result,
            )
        })
    });
    let Some((publication, result)) = completed else {
        return false;
    };
    let current = &app.state.dialogs.hardcopy;
    if current.open
        && (current.preview_generation != publication.ticket.generation
            || current
                .preview_plan
                .as_ref()
                .map(|plan| plan.content_digest())
                != Some(publication.ticket.plan_digest)
            || current
                .resolved_document
                .as_ref()
                .map(|source| source.authority().content_digest())
                != Some(publication.source_digest))
    {
        app.state.dialogs.hardcopy.busy = false;
        if app.state.dialogs.hardcopy.open {
            app.state.dialogs.hardcopy.error = Some(
                "The hardcopy source or plan changed before native finalization completed."
                    .to_owned(),
            );
        }
        return true;
    }
    let plan = publication.plan;
    let completion = match result {
        Err(error) => Err(record_native_finalization_failure(
            app,
            &plan,
            &publication.destination,
            error,
        )),
        Ok(completion) if completion.ticket != publication.ticket => Err(record_render_failure(
            app,
            &plan,
            "Native hardcopy finalization returned a stale completion ticket.".to_owned(),
        )),
        Ok(completion) => match (publication.destination, completion.payload) {
            (
                NativePublicationDestination::Print { .. },
                super::finalize::FinalizationPayload::Print {
                    outcome,
                    accepted,
                    display_name,
                },
            ) => (|| {
                let (message, commit_mapping) = match &outcome {
                    HardcopyOutcome::SpoolAccepted { pages_accepted, .. }
                        if *pages_accepted as usize == accepted =>
                    {
                        (
                            format!(
                                "The Windows spooler accepted {accepted} page{} for {display_name}.",
                                if accepted == 1 { "" } else { "s" }
                            ),
                            true,
                        )
                    }
                    HardcopyOutcome::Cancelled {
                        pages_completed, ..
                    } if *pages_completed as usize == accepted => (
                        format!(
                            "Printing to {display_name} was cancelled after {accepted} completed page{}.",
                            if accepted == 1 { "" } else { "s" }
                        ),
                        false,
                    ),
                    _ => {
                        return Err(
                            "Native hardcopy finalization returned an inconsistent print outcome."
                                .to_owned(),
                        );
                    }
                };
                let receipt =
                    HardcopyReceipt::record(&plan, outcome).map_err(|error| error.to_string())?;
                app.state.dialogs.hardcopy.last_receipt = Some(receipt);
                if commit_mapping {
                    commit_print_mapping_persistence(app, publication.staged_mapping)?;
                }
                Ok(message)
            })(),
            (
                NativePublicationDestination::Export {
                    path, multi_part, ..
                },
                super::finalize::FinalizationPayload::Export {
                    artifact,
                    page_count,
                    format,
                },
            ) => (|| {
                let receipt =
                    HardcopyReceipt::record(&plan, HardcopyOutcome::ArtifactExported { artifact })
                        .map_err(|error| error.to_string())?;
                app.state.dialogs.hardcopy.last_receipt = Some(receipt);
                commit_print_mapping_persistence(app, publication.staged_mapping)?;
                Ok(export_completion_message(
                    "hardcopy",
                    &path,
                    Some(format!(
                        "{page_count} page{} \u{00b7} {}",
                        if page_count == 1 { "" } else { "s" },
                        if multi_part {
                            "deterministic ZIP package"
                        } else if format.is_vector() {
                            "vector"
                        } else {
                            "raster"
                        }
                    )),
                    app.export_workflow_io.as_ref(),
                ))
            })(),
            _ => Err(record_render_failure(
                app,
                &plan,
                "Native hardcopy finalization returned the wrong payload type.".to_owned(),
            )),
        },
    };
    app.state.dialogs.hardcopy.busy = false;
    match completion {
        Ok(message) => {
            app.state.push_user_message(ConsoleMessage::info(message));
            if app.state.dialogs.hardcopy.open {
                app.state.dialogs.hardcopy.close();
            }
        }
        Err(error) => {
            if app.state.dialogs.hardcopy.open {
                app.state.dialogs.hardcopy.error = Some(error);
            } else if !error.contains("was cancelled") {
                app.state.push_user_message(ConsoleMessage::warning(error));
            }
        }
    }
    true
}

#[cfg(not(target_arch = "wasm32"))]
fn record_native_finalization_failure(
    app: &mut RSpiceApp,
    plan: &HardcopyPlan,
    destination: &NativePublicationDestination,
    error: super::finalize::FinalizationFailure,
) -> String {
    if matches!(destination, NativePublicationDestination::Print { .. })
        && let Some(outcome) = error.hardcopy_outcome()
    {
        return match HardcopyReceipt::record(plan, outcome) {
            Ok(receipt) => {
                app.state.dialogs.hardcopy.last_receipt = Some(receipt);
                error.to_string()
            }
            Err(receipt_error) => {
                format!("{error}; the typed print outcome could not be recorded: {receipt_error}")
            }
        };
    }
    match destination {
        NativePublicationDestination::Print { .. } => {
            record_render_failure(app, plan, error.to_string())
        }
        NativePublicationDestination::Export { .. } => {
            record_export_failure(app, plan, error.to_string())
        }
    }
}

pub(super) fn cancel_publication() {
    clear_pending_publication();
    PENDING_PAGE_SETUP.with(|slot| {
        slot.borrow_mut().take();
    });
    #[cfg(target_arch = "wasm32")]
    PUBLICATION.with(|active| {
        if active.borrow_mut().take().is_some() {
            super::worker::cancel();
        }
    });
    #[cfg(not(target_arch = "wasm32"))]
    {
        PUBLICATION.with(|active| {
            if let Some(publication) = active.borrow_mut().take()
                && let NativePublicationDestination::Print { cancellation, .. } =
                    publication.destination
            {
                cancellation.cancel();
            }
        });
        super::execution::cancel();
        super::finalize::cancel();
    }
}

#[cfg(target_arch = "wasm32")]
impl ActiveBrowserPublication {
    fn ticket_plan_digest(&self) -> ContentDigest {
        self.plan.content_digest()
    }

    fn ticket_source_digest(&self) -> ContentDigest {
        self.source.authority().content_digest()
    }
}

#[cfg(target_arch = "wasm32")]
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

fn format_file_contract(
    format: OutputFormat,
) -> Result<(&'static str, &'static str, &'static str), String> {
    match format {
        OutputFormat::PdfVector | OutputFormat::PdfA => {
            Ok(("pdf", "application/pdf", "PDF document"))
        }
        OutputFormat::SvgVector => Ok(("svg", "image/svg+xml", "SVG image")),
        OutputFormat::Png { .. } => Ok(("png", "image/png", "PNG image")),
        OutputFormat::Tiff { .. } => Ok(("tiff", "image/tiff", "TIFF image")),
        OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument => Err(
            "Printer formats cannot be exported as files; choose an artifact output format."
                .to_owned(),
        ),
    }
}

fn safe_filename(value: &str) -> String {
    let mut name = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '-' | '_') {
                character
            } else {
                '-'
            }
        })
        .collect::<String>();
    while name.contains("--") {
        name = name.replace("--", "-");
    }
    let name = name.trim_matches('-');
    if name.is_empty() {
        "rspice-hardcopy".to_owned()
    } else {
        name.to_owned()
    }
}

fn record_render_failure(app: &mut RSpiceApp, plan: &HardcopyPlan, message: String) -> String {
    record_failure(
        app,
        plan,
        HardcopyFailureCode::RenderFailure,
        message,
        false,
    )
}

#[cfg(target_arch = "wasm32")]
fn record_print_failure(
    app: &mut RSpiceApp,
    plan: &HardcopyPlan,
    error: crate::workbench::hardcopy_adapters::print::HardcopyPrintError,
    pages_completed: u32,
) -> String {
    let message = error.to_string();
    if let Ok(receipt) = HardcopyReceipt::record(plan, error.failure_outcome(pages_completed)) {
        app.state.dialogs.hardcopy.last_receipt = Some(receipt);
    }
    message
}

fn record_export_failure(app: &mut RSpiceApp, plan: &HardcopyPlan, message: String) -> String {
    record_failure(
        app,
        plan,
        HardcopyFailureCode::DestinationWriteFailed,
        message,
        true,
    )
}

fn record_failure(
    app: &mut RSpiceApp,
    plan: &HardcopyPlan,
    code: HardcopyFailureCode,
    message: String,
    retryable: bool,
) -> String {
    let outcome = HardcopyOutcome::Failed {
        code,
        message: message.clone(),
        pages_completed: 0,
        retryable,
    };
    if let Ok(receipt) = HardcopyReceipt::record(plan, outcome) {
        app.state.dialogs.hardcopy.last_receipt = Some(receipt);
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn publication_reuses_only_the_exact_sealed_preview_plan() {
        use crate::state::{Point, Wire};
        use crate::workbench::state::WorkspaceDocumentId;

        let mut app = RSpiceApp::test_instance();
        app.state
            .schematic
            .wires
            .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
        let reference = app.state.workspace.active_view.clone();
        app.state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(reference));
        let resolved =
            crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(&app.state)
                .expect("active schematic resolves");
        app.state
            .dialogs
            .hardcopy
            .open_resolved(HardcopyWorkflow::Export, resolved.clone(), None)
            .expect("dialog opens");

        let preview = app
            .state
            .dialogs
            .hardcopy
            .preview_plan
            .as_ref()
            .expect("sealed preview")
            .clone();
        let publication = current_plan(&app, &resolved).expect("unchanged plan");
        assert!(std::sync::Arc::ptr_eq(&preview, &publication));

        app.state.dialogs.hardcopy.margin_left = "0.5".to_owned();
        assert!(
            current_plan(&app, &resolved)
                .expect_err("unpreviewed settings must fail closed")
                .contains("sealed preview plan")
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn app_state_clone_drops_runtime_hardcopy_authority_and_payloads() {
        use crate::hardcopy::{HardcopyOutcome, PrinterRasterGeometry};
        use crate::state::{Point, Wire};
        use crate::workbench::state::WorkspaceDocumentId;

        let mut app = RSpiceApp::test_instance();
        app.state
            .schematic
            .wires
            .push(Wire::segment(71, Point::new(0, 0), Point::new(20, 0)));
        let reference = app.state.workspace.active_view.clone();
        app.state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(reference));
        let resolved =
            crate::workbench::hardcopy_adapters::sources::resolve_active_app_hardcopy_source(&app.state)
                .expect("active schematic resolves");
        app.state
            .dialogs
            .hardcopy
            .open_resolved(HardcopyWorkflow::Export, resolved.clone(), None)
            .expect("dialog opens");
        let plan = app
            .state
            .dialogs
            .hardcopy
            .preview_plan
            .as_ref()
            .expect("sealed preview")
            .clone();
        let metadata =
            HardcopySceneMetadata::try_new(resolved.authority().display_name(), "RSpice")
                .expect("metadata");
        app.state.dialogs.hardcopy.preview = Some(std::sync::Arc::new(
            HardcopyRenderer::render_preview_page_resolved(&plan, &resolved, metadata, 0, 72)
                .expect("preview"),
        ));
        app.state.dialogs.hardcopy.source_resolution_generation = 19;
        app.state.dialogs.hardcopy.printer_report =
            Some(crate::workbench::hardcopy_adapters::print::PrinterDiscoveryReport::default());
        app.state.dialogs.hardcopy.printer_job = Some(
            PrinterJobSettings::try_new(
                ContentDigest::from_bytes([0x50; 32]),
                "1",
                PrinterRasterGeometry::try_new(1_000, 800, 0, 0, 1_000, 800).unwrap(),
                PrinterMediaSource::AutomaticCompatibleTray,
                600,
                DuplexMode::Off,
                1,
                false,
            )
            .unwrap(),
        );
        app.state.dialogs.hardcopy.last_receipt = Some(
            HardcopyReceipt::record(
                &plan,
                HardcopyOutcome::Failed {
                    code: HardcopyFailureCode::InternalFailure,
                    message: "test failure".to_owned(),
                    pages_completed: 0,
                    retryable: false,
                },
            )
            .expect("receipt"),
        );
        app.state.dialogs.hardcopy.busy = true;
        app.state.dialogs.hardcopy.error = Some("test error".to_owned());

        let cloned = app.state.clone();
        let hardcopy = &cloned.dialogs.hardcopy;
        assert!(!hardcopy.open);
        assert!(hardcopy.source.is_none());
        assert!(hardcopy.resolved_document.is_none());
        assert!(hardcopy.preview_plan.is_none());
        assert!(hardcopy.preview.is_none());
        assert!(hardcopy.preview_adjacent.is_none());
        assert_eq!(hardcopy.source_resolution_generation, 0);
        assert!(hardcopy.printer_report.is_none());
        assert!(hardcopy.printer_job.is_none());
        assert!(hardcopy.last_receipt.is_none());
        assert!(!hardcopy.busy);
        assert!(hardcopy.error.is_none());
    }

    #[test]
    fn artifact_names_are_portable_and_nonempty() {
        assert_eq!(safe_filename("top / schematic"), "top-schematic");
        assert_eq!(safe_filename("***"), "rspice-hardcopy");
        assert_eq!(safe_filename("afe--out"), "afe-out");
    }

    #[test]
    fn file_contract_does_not_misrepresent_print_targets() {
        assert_eq!(format_file_contract(OutputFormat::PdfA).unwrap().0, "pdf");
        assert!(format_file_contract(OutputFormat::NativePrinter).is_err());
        assert!(format_file_contract(OutputFormat::BrowserPrintDocument).is_err());
    }

    #[test]
    fn project_print_mapping_stage_is_transactional_until_publication_commit() {
        use crate::hardcopy::{PrintMappingSaveScope, PrintMappingTable};

        let mut app = RSpiceApp::test_instance();
        let before = app.state.workspace.project_print_mappings.clone();
        let mapping = PrintMappingTable::try_new(
            PrintMappingSaveScope::ProjectPrintSet("release-proof".to_owned()),
            Vec::new(),
        )
        .unwrap();

        let staged = stage_print_mapping_persistence(&app, &mapping).unwrap();
        assert_eq!(app.state.workspace.project_print_mappings, before);
        assert!(!app.state.workspace.project_print_mappings_dirty);
        drop(staged);
        assert_eq!(app.state.workspace.project_print_mappings, before);
        assert!(!app.state.workspace.project_print_mappings_dirty);

        let staged = stage_print_mapping_persistence(&app, &mapping).unwrap();
        commit_print_mapping_persistence(&mut app, staged).unwrap();
        assert_eq!(
            app.state
                .workspace
                .project_print_mappings
                .get("release-proof")
                .unwrap(),
            &mapping
        );
        assert!(app.state.workspace.project_print_mappings_dirty);
    }

    #[test]
    fn identity_bands_are_exactly_one_line_each() {
        let (header, provenance) =
            identity_lines("P", "7", "top / schematic", "source:key", "3", "0123");
        assert!(!header.contains('\n'));
        assert!(!provenance.contains('\n'));
        assert!(header.contains("top / schematic"));
        assert!(provenance.contains("source:key"));
    }

    #[test]
    fn selected_export_name_receives_the_contract_extension() {
        let mut path = std::path::PathBuf::from("review-output");
        crate::workbench::workflows::file_actions::ensure_file_extension(&mut path, "pdf");
        assert_eq!(path, std::path::PathBuf::from("review-output.pdf"));
    }

    #[test]
    fn default_printer_resolution_prefers_600_then_a_bounded_highest_mode() {
        assert_eq!(preferred_resolution(vec![300, 600, 1_200]), Some(600));
        assert_eq!(
            preferred_resolution(vec![300, 720, 1_200, 2_400]),
            Some(1_200)
        );
        assert_eq!(preferred_resolution(vec![2_400, 4_800]), Some(2_400));
        assert_eq!(preferred_resolution(Vec::new()), None);
    }
}

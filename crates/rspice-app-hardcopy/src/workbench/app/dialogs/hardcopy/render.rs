//! Mockup-owned hardcopy dialogs.
//!
//! This surface deliberately renders the renderer-owned page raster. It never
//! takes a screenshot of the viewport, so preview, export, and print all share
//! one semantic source and one immutable pagination plan.

mod bodies;
pub mod content_options;
mod labels;
mod preview;
mod settings;

use bodies::*;
use content_options::*;
use labels::*;
use preview::*;
use settings::*;

use egui::{
    Align, ColorImage, Context, Frame, Layout, Margin, Rect, Sense, Stroke, TextureHandle,
    TextureOptions, Ui, Vec2, vec2,
};

use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, DialogTransactionTone, IconButton, SegmentedWidth,
    segmented,
};
#[cfg(not(target_arch = "wasm32"))]
use rspice_hardcopy::render::HardcopyRenderer;
use rspice_hardcopy::render::{HardcopyPreviewPage, HardcopyRenderError, max_raster_dpi};
use rspice_hardcopy_contract::{
    AuthoredSheetMedia, BackgroundMode, ColorMapping, HardcopyDocumentKind, HardcopyScope, Length,
    LengthUnit, MAX_RASTER_DPI, MIN_RASTER_DPI, Orientation, OutputFormat,
    OutsideSheetContentPolicy, PaperSize, PrintColor, PrintMappingEntry, PrintMappingTable,
    PrintRedundancy, PrinterJobSettings, ResolvedOrientation, ScaleMode, SchematicHardcopyExtent,
    SchematicHardcopySetup, StandardPaper, TilingMode, Watermark,
};
#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
use rspice_hardcopy_contract::{DuplexMode, PrinterMediaSource};

use super::{
    BlockedSetup, DEFAULT_RASTER_DPI, HardcopyDialogPage, HardcopyDialogState, HardcopyParts,
    HardcopyWorkflow, PaperDraft, publish,
};

#[derive(Clone)]
struct PreviewTexture {
    digest: crate::product::ContentDigest,
    handle: TextureHandle,
}

#[cfg(not(target_arch = "wasm32"))]
type PreviewWorkerPayload = Result<
    (
        HardcopyPreviewPage,
        Option<HardcopyPreviewPage>,
        crate::product::ContentDigest,
    ),
    BlockedSetup,
>;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct PreviewWorkerRuntime {
    active: Option<ActivePreviewWorker>,
}

#[cfg(not(target_arch = "wasm32"))]
struct ActivePreviewWorker {
    generation: u64,
    receiver: std::sync::mpsc::Receiver<PreviewWorkerPayload>,
    cancelled: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PreviewScheduleDecision {
    Launch,
    Wait,
    CancelAndWait,
}

#[cfg(not(target_arch = "wasm32"))]
fn preview_schedule_decision(
    active_generation: Option<u64>,
    requested_generation: u64,
) -> PreviewScheduleDecision {
    match active_generation {
        None => PreviewScheduleDecision::Launch,
        Some(active) if active == requested_generation => PreviewScheduleDecision::Wait,
        Some(_) => PreviewScheduleDecision::CancelAndWait,
    }
}

#[cfg(not(target_arch = "wasm32"))]
thread_local! {
    static PREVIEW_WORKER: std::cell::RefCell<PreviewWorkerRuntime> =
        std::cell::RefCell::new(PreviewWorkerRuntime::default());
}

#[cfg(target_arch = "wasm32")]
struct ActiveBrowserPreview {
    ticket: super::worker::HardcopyWorkerTicket,
    page_indices: Vec<usize>,
    dpi: u16,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static PREVIEW_WORKER: std::cell::RefCell<Option<ActiveBrowserPreview>> =
        const { std::cell::RefCell::new(None) };
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
pub enum BodyAction {
    None,
    PrinterProperties,
    CustomPaper,
    PrintMapping,
    SelectPrinter(String),
    DriverProperties,
    SelectSource {
        source_key: String,
        scope: HardcopyScope,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ContentInputs {
    paper: PaperDraft,
    display_unit: LengthUnit,
    orientation: Orientation,
    margins: [String; 4],
    bleed: String,
    scale: ScaleMode,
    custom_scale_percent: String,
    tiling: TilingMode,
    manual_columns: String,
    manual_rows: String,
    overlap: String,
    registration_marks: bool,
    schematic: SchematicHardcopySetup,
    printer_id: String,
    printer_job: Option<PrinterJobSettings>,
    page_range: Option<(u32, u32)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RenderingInputs {
    format: OutputFormat,
    color_mapping: ColorMapping,
    background: BackgroundMode,
    embed_fonts: bool,
    searchable_text: bool,
    soft_proof: bool,
    decorations: (bool, bool, bool, Watermark),
    plot_overlays: (bool, bool),
}

/// Draw the hardcopy dialog for this frame over its `parts`. It runs every
/// frame, open or not: closed, it lets go of the work the dialog started.
pub fn render_hardcopy_dialog(parts: &mut HardcopyParts<'_>, ctx: &Context) {
    publish::register_repaint_context(ctx);
    if !parts.dialog.open {
        publish::cancel_source_resolution();
        publish::cancel_publication();
        publish::poll_publication(parts);
        cancel_preview_worker();
        clear_preview_textures(ctx);
        return;
    }
    publish::poll_source_resolution(parts);
    publish::poll_printer_catalog(parts);
    publish::poll_publication(parts);
    if !parts.dialog.open {
        clear_preview_textures(ctx);
        return;
    }
    // The dialog appears once it has read its source, rather than over a
    // frame or two claiming there is none.
    if parts.dialog.opening_source_pending() {
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
        return;
    }
    if parts.dialog.printer_discovery_busy || parts.dialog.busy {
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
    }
    let page = parts.dialog.page;
    let (title, primary, secondary): (&str, &str, Option<&str>) = match page {
        HardcopyDialogPage::Main => (
            parts.dialog.workflow.title(),
            parts.dialog.workflow.primary_label(),
            None,
        ),
        HardcopyDialogPage::PrinterProperties => {
            ("Printer properties", "Apply printer properties", None)
        }
        HardcopyDialogPage::CustomPaperMargins => {
            ("Custom paper and margins", "Use custom page", None)
        }
        HardcopyDialogPage::PrintMapping => (
            "Layer, trace, and marker print mapping",
            "Save print mapping",
            None,
        ),
    };
    let error = parts.dialog.error.clone();
    let busy = parts.dialog.busy;
    let plan_valid = parts.dialog.preview_plan.is_some();
    let description = match page {
        HardcopyDialogPage::Main => None,
        HardcopyDialogPage::PrinterProperties => {
            Some("Check the paper, resolution and options the selected printer offers.")
        }
        HardcopyDialogPage::CustomPaperMargins => Some("Set a paper size and margins of your own."),
        HardcopyDialogPage::PrintMapping => {
            Some("Choose how layers, traces and markers look on paper.")
        }
    };
    let mut body_scroll_offset = parts.dialog.body_scroll_offset;
    let mut body_action = BodyAction::None;
    // One page: the settings in groups beside the page they lay out. The
    // surface holds one size whatever is chosen, so nothing moves as an
    // option shows or hides, and it never grows to the window.
    let mut dialog = Dialog::new(title, primary)
        .size(DialogSize::DrawingSheetWorkflow)
        .fixed_height(DIALOG_HEIGHT)
        .flush_body()
        .primary_on_enter(false)
        .primary_enabled(!busy && (plan_valid || page != HardcopyDialogPage::Main))
        .secondary("Cancel");
    if let Some(description) = description {
        dialog = dialog.description(description);
    }
    if page == HardcopyDialogPage::Main
        && let Some(warning) = parts.dialog.printer_margin_warning()
    {
        dialog = dialog
            .hint(warning)
            .hint_tone(crate::ui::widgets::DialogHintTone::Warn);
    }
    // The body divides its track itself and owns whatever scrolling any
    // region of it ever needs.
    dialog = if page == HardcopyDialogPage::Main {
        dialog.manual_body_scroll()
    } else {
        dialog.body_scroll_offset(&mut body_scroll_offset)
    };
    if let Some(label) = secondary {
        dialog = dialog.secondary(label);
    }
    if let Some(error) = error.as_deref() {
        // The strip is the only thing on screen that explains a disabled
        // primary.
        let title = match parts.dialog.workflow {
            HardcopyWorkflow::Print => "Cannot print with these settings",
            HardcopyWorkflow::Export => "Cannot export with these settings",
        };
        dialog = dialog.transaction_state(
            DialogTransactionTone::Error,
            title.to_owned(),
            sentence_case(error),
        );
    }
    let choice = dialog.show(ctx, |ui| {
        ui.add_enabled_ui(!busy, |ui| {
            body_action = match page {
                HardcopyDialogPage::Main => main_body(ui, parts.dialog),
                HardcopyDialogPage::PrinterProperties => printer_properties_body(ui, parts.dialog),
                HardcopyDialogPage::CustomPaperMargins => {
                    custom_paper_body(ui, parts.dialog);
                    BodyAction::None
                }
                HardcopyDialogPage::PrintMapping => {
                    print_mapping_body(ui, parts.dialog);
                    BodyAction::None
                }
            };
        });
    });
    parts.dialog.body_scroll_offset = body_scroll_offset;

    match body_action {
        BodyAction::None => {}
        BodyAction::PrinterProperties => parts
            .dialog
            .open_subflow(HardcopyDialogPage::PrinterProperties),
        BodyAction::CustomPaper => parts
            .dialog
            .open_subflow(HardcopyDialogPage::CustomPaperMargins),
        BodyAction::PrintMapping => parts.dialog.open_subflow(HardcopyDialogPage::PrintMapping),
        BodyAction::SelectPrinter(device_id) => {
            let capabilities = parts
                .dialog
                .printer_report
                .as_ref()
                .and_then(|report| {
                    report
                        .printers()
                        .iter()
                        .find(|entry| entry.capabilities().device_id() == device_id)
                })
                .map(|entry| entry.capabilities().clone());
            if let Some(capabilities) = capabilities {
                publish::select_printer_capabilities(parts, capabilities, None);
            }
        }
        BodyAction::DriverProperties => {
            #[cfg(target_os = "windows")]
            {
                let printer_id = parts.dialog.printer_id.clone();
                match crate::workbench::hardcopy_adapters::print::show_native_printer_properties(
                    &printer_id,
                    None,
                ) {
                    Ok(crate::workbench::hardcopy_adapters::print::PrinterDriverPropertiesOutcome::Accepted {
                        capabilities,
                        suggestion,
                    }) => {
                        apply_suggested_paper(
                            parts.dialog,
                            &capabilities,
                            suggestion.paper_platform_id,
                        );
                        publish::select_printer_capabilities(
                            parts,
                            *capabilities,
                            Some(suggestion),
                        );
                    }
                    Ok(crate::workbench::hardcopy_adapters::print::PrinterDriverPropertiesOutcome::Cancelled) => {}
                    Err(error) => {
                        parts.dialog.error = Some(error.to_string());
                    }
                }
            }
        }
        BodyAction::SelectSource { source_key, scope } => {
            publish::select_retained_source(parts, &source_key, scope);
        }
    }
    // After the body, so the page is rendered at the resolution the desk
    // measured for it this pass rather than at a guess redone a pass later.
    ensure_preview(ctx, parts.dialog);
    match choice {
        DialogChoice::Primary if page == HardcopyDialogPage::Main => {
            publish::commit_hardcopy_workflow(parts);
        }
        DialogChoice::Primary => {
            if page == HardcopyDialogPage::PrinterProperties
                && parts.dialog.format == OutputFormat::NativePrinter
                && !reconcile_native_printer_job(parts.dialog)
            {
                parts.dialog.error = Some(
                    "The selected paper, orientation, and resolution are not supported by the current printer."
                        .to_owned(),
                );
                return;
            }
            if let Err(error) = parts.dialog.accept_subflow() {
                parts.dialog.error = Some(error.to_string());
            }
        }
        DialogChoice::Secondary | DialogChoice::Cancelled if page != HardcopyDialogPage::Main => {
            parts.dialog.cancel_subflow();
        }
        DialogChoice::Secondary | DialogChoice::Cancelled => {
            publish::cancel_source_resolution();
            publish::cancel_publication();
            cancel_preview_worker();
            clear_preview_textures(ctx);
            parts.dialog.close();
        }
        DialogChoice::Ghost | DialogChoice::None => {}
    }
}

/// A renderer refusal read as a refusal of the thing the operator was looking
/// at. The sentence gains its context; the section it was attributed to is
/// carried through unchanged, because the rail has to mark the same place
/// whichever render raised it.
fn refused_preview(error: HardcopyRenderError) -> BlockedSetup {
    let BlockedSetup { message, section } = BlockedSetup::from(error);
    BlockedSetup {
        message: format!("The preview could not be rendered: {message}"),
        section,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn ensure_preview(ctx: &Context, draft: &mut HardcopyDialogState) {
    // A page rendered for another zoom stays on the desk, stretched, until
    // the one for this zoom arrives.
    let dpi = draft.preview_render_dpi();
    if draft
        .preview
        .as_ref()
        .is_some_and(|preview| preview.dpi() == dpi)
        || draft.preview_plan.is_none()
        || draft.preview_failed_generation == Some(draft.preview_generation)
    {
        return;
    }
    let generation = draft.preview_generation;
    let completed = PREVIEW_WORKER.with(|runtime| {
        let mut runtime = runtime.borrow_mut();
        let polled = runtime
            .active
            .as_ref()
            .and_then(|active| match active.receiver.try_recv() {
                Ok(result) => Some((active.generation, result)),
                Err(std::sync::mpsc::TryRecvError::Disconnected) => Some((
                    active.generation,
                    Err(BlockedSetup::dialog(
                        "The preview worker stopped before returning a result.",
                    )),
                )),
                Err(std::sync::mpsc::TryRecvError::Empty) => None,
            });
        if polled.is_some() {
            runtime.active = None;
        }
        polled
    });
    if let Some((completed_generation, result)) = completed
        && completed_generation == generation
    {
        match result {
            Ok((preview, adjacent, plan_digest))
                if draft
                    .preview_plan
                    .as_ref()
                    .is_some_and(|plan| plan.content_digest() == plan_digest) =>
            {
                draft.preview = Some(std::sync::Arc::new(preview));
                draft.preview_adjacent = adjacent.map(std::sync::Arc::new);
                draft.preview_failed_generation = None;
                draft.error = None;
                draft.error_section = None;
            }
            Ok(_) => {}
            Err(blocked) => {
                draft.preview_failed_generation = Some(completed_generation);
                draft.error = Some(blocked.message);
                draft.error_section = blocked.section;
            }
        }
        // The pass has already been drawn; the next one shows the result.
        ctx.request_repaint();
        return;
    }
    let schedule = PREVIEW_WORKER.with(|runtime| {
        let runtime = runtime.borrow();
        let decision = preview_schedule_decision(
            runtime.active.as_ref().map(|active| active.generation),
            generation,
        );
        if decision == PreviewScheduleDecision::CancelAndWait
            && let Some(active) = runtime.active.as_ref()
        {
            active
                .cancelled
                .store(true, std::sync::atomic::Ordering::Release);
        }
        decision
    });
    if schedule != PreviewScheduleDecision::Launch {
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
        return;
    }
    let Some(document) = draft.resolved_document.clone() else {
        return;
    };
    let Some(metadata) = draft.metadata.clone() else {
        return;
    };
    let Some(plan) = draft.preview_plan.clone() else {
        return;
    };
    let preview_page = draft.preview_page;
    let plan_digest = plan.content_digest();
    let repaint = ctx.clone();
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let worker_cancelled = cancelled.clone();
    std::thread::Builder::new()
        .name("rspice-hardcopy-preview".to_owned())
        .spawn(move || {
            let page_count = plan.pagination().pages().len();
            let adjacent_index = if page_count > 1 {
                if (preview_page as usize) + 1 < page_count {
                    Some(preview_page as usize + 1)
                } else {
                    Some(preview_page.saturating_sub(1) as usize)
                }
            } else {
                None
            };
            let mut page_indices = vec![preview_page as usize];
            if let Some(adjacent) = adjacent_index {
                page_indices.push(adjacent);
            }
            let result = HardcopyRenderer::render_preview_pages_resolved(
                &plan,
                &document,
                metadata,
                &page_indices,
                dpi,
                || worker_cancelled.load(std::sync::atomic::Ordering::Acquire),
            )
            .map_err(refused_preview)
            .and_then(|mut pages| {
                if pages.is_empty() {
                    return Err(BlockedSetup::dialog(
                        "The preview worker returned no selected page.",
                    ));
                }
                let preview = pages.remove(0);
                let adjacent = pages.into_iter().next();
                Ok((preview, adjacent, plan_digest))
            });
            let _ = sender.send(result);
            repaint.request_repaint();
        })
        .map_err(|error| error.to_string())
        .map_or_else(
            |error| {
                draft.preview_failed_generation = Some(generation);
                draft.error = Some(format!("Could not start preview worker: {error}"));
                draft.error_section = None;
            },
            |_| {
                PREVIEW_WORKER.with(|runtime| {
                    runtime.borrow_mut().active = Some(ActivePreviewWorker {
                        generation,
                        receiver,
                        cancelled,
                    });
                });
            },
        );
}

fn cancel_preview_worker() {
    #[cfg(not(target_arch = "wasm32"))]
    PREVIEW_WORKER.with(|runtime| {
        if let Some(active) = runtime.borrow_mut().active.take() {
            active
                .cancelled
                .store(true, std::sync::atomic::Ordering::Release);
        }
    });
    #[cfg(target_arch = "wasm32")]
    PREVIEW_WORKER.with(|runtime| {
        if runtime.borrow_mut().take().is_some() {
            super::worker::cancel();
        }
    });
}

fn clear_preview_textures(ctx: &Context) {
    ctx.data_mut(|data| {
        data.remove::<PreviewTexture>(preview_texture_slot_id(0));
        data.remove::<PreviewTexture>(preview_texture_slot_id(1));
    });
}

#[cfg(target_arch = "wasm32")]
fn ensure_preview(ctx: &Context, draft: &mut HardcopyDialogState) {
    let dpi = draft.preview_render_dpi();
    if draft
        .preview
        .as_ref()
        .is_some_and(|preview| preview.dpi() == dpi)
        || draft.preview_plan.is_none()
        || draft.preview_failed_generation == Some(draft.preview_generation)
    {
        return;
    }
    let Some((document, metadata, plan)) = draft
        .resolved_document
        .clone()
        .zip(draft.metadata.clone())
        .zip(draft.preview_plan.clone())
        .map(|((document, metadata), plan)| (document, metadata, plan))
    else {
        return;
    };

    let completed = PREVIEW_WORKER.with(|active| {
        let mut active = active.borrow_mut();
        let result = active.as_ref().and_then(|worker| {
            super::worker::poll(worker.ticket).map(|result| {
                (
                    worker.ticket,
                    worker.page_indices.clone(),
                    worker.dpi,
                    result,
                )
            })
        });
        if result.is_some() {
            active.take();
        }
        result
    });
    if let Some((ticket, page_indices, dpi, result)) = completed {
        if ticket.epoch != draft.source_resolution_generation
            || ticket.generation != draft.preview_generation
        {
            return;
        }
        let decoded = result.map_err(BlockedSetup::dialog).and_then(|buffers| {
            if buffers.len() != page_indices.len() * 2 {
                return Err(BlockedSetup::dialog(
                    "Browser preview returned the wrong buffer count.",
                ));
            }
            let mut previews = Vec::with_capacity(page_indices.len());
            for (pair, page_index) in buffers.chunks_exact(2).zip(page_indices) {
                previews.push(
                    HardcopyPreviewPage::from_worker_transfer(
                        &plan,
                        &document,
                        page_index,
                        dpi,
                        &pair[0],
                        pair[1].clone(),
                    )
                    .map_err(refused_preview)?,
                );
            }
            Ok(previews)
        });
        match decoded {
            Ok(mut previews) if !previews.is_empty() => {
                draft.preview = Some(std::sync::Arc::new(previews.remove(0)));
                draft.preview_adjacent = previews.into_iter().next().map(std::sync::Arc::new);
                draft.preview_failed_generation = None;
                draft.error = None;
                draft.error_section = None;
            }
            Ok(_) => {
                draft.preview_failed_generation = Some(ticket.generation);
                draft.error = Some("Browser preview returned no selected page.".to_owned());
                draft.error_section = None;
            }
            Err(blocked) => {
                draft.preview_failed_generation = Some(ticket.generation);
                draft.error = Some(blocked.message);
                draft.error_section = blocked.section;
            }
        }
        ctx.request_repaint();
        return;
    }

    let active_generation = PREVIEW_WORKER.with(|active| {
        active
            .borrow()
            .as_ref()
            .map(|worker| worker.ticket.generation)
    });
    if let Some(active_generation) = active_generation {
        if active_generation != draft.preview_generation {
            cancel_preview_worker();
        } else {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
            return;
        }
    }
    if super::worker::is_active() {
        ctx.request_repaint_after(std::time::Duration::from_millis(16));
        return;
    }

    let page_count = plan.pagination().pages().len();
    let selected = draft.preview_page as usize;
    let adjacent = if page_count > 1 {
        if selected + 1 < page_count {
            Some(selected + 1)
        } else {
            Some(selected.saturating_sub(1))
        }
    } else {
        None
    };
    let mut page_indices = vec![selected];
    if let Some(adjacent) = adjacent {
        page_indices.push(adjacent);
    }
    match super::worker::start_preview(
        &plan,
        &document,
        metadata,
        page_indices.clone(),
        dpi,
        draft.source_resolution_generation,
        draft.preview_generation,
        ctx.clone(),
    ) {
        Ok(ticket) => PREVIEW_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveBrowserPreview {
                ticket,
                page_indices,
                dpi,
            });
        }),
        Err(error) => {
            draft.preview_failed_generation = Some(draft.preview_generation);
            draft.error = Some(format!("Could not start browser preview worker: {error}"));
            draft.error_section = None;
        }
    }
}

/// The dialog's height on every page: the settings column's groups, held
/// whole, over the footer.
const DIALOG_HEIGHT: f32 = 760.0;

fn main_body(ui: &mut Ui, draft: &mut HardcopyDialogState) -> BodyAction {
    // One before/after comparison for the whole surface, so a control
    // invalidates the preview wherever it is drawn.
    let before_content = content_inputs(draft);
    let before_render = rendering_inputs(draft);
    // A field the operator typed into and then left is settled here, on the
    // one path every pass takes. It has to come after the snapshot above: a
    // resolution settled outside that comparison would never invalidate the
    // plan it is meant to publish.
    settle_raster_resolution(ui, draft);
    // What a file keeps of its words follows from what the file is: a PDF or
    // an SVG carries its typefaces and its text, pixels carry neither.
    draft.embed_fonts = draft.format.is_vector();
    draft.searchable_text = draft.format.is_vector();
    draft.fit_fonts_to_format();
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    let available = ui.available_size();
    let height = available.y.max(1.0);
    let t = Tokens::get(ui.ctx());
    let mut action = BodyAction::None;
    ui.allocate_ui_with_layout(
        vec2(available.x, height),
        Layout::left_to_right(Align::Min),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let (settings, settings_action) = settings_column(ui, draft, height);
            action = settings_action;
            // The page takes exactly what the settings leave, clipped, so a
            // long name in either can never widen the dialog.
            let width = ui.available_width().max(1.0);
            let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
            let mut page = ui.new_child(
                egui::UiBuilder::new()
                    .id_salt("hardcopy-preview-column")
                    .max_rect(rect)
                    .layout(Layout::top_down(Align::Min)),
            );
            page.set_clip_rect(rect.intersect(ui.clip_rect()));
            preview_column(&mut page, draft, height);
            ui.painter().vline(
                settings.right(),
                settings.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        },
    );

    if draft.format == OutputFormat::NativePrinter
        && (before_content.paper != draft.paper || before_content.orientation != draft.orientation)
    {
        reconcile_native_printer_job(draft);
    }
    if content_inputs(draft) != before_content || rendering_inputs(draft) != before_render {
        draft.refresh_preview();
    }
    action
}

/// `Some(true)` when resolved schematic content crosses the authored sheet,
/// `Some(false)` when it does not, `None` when this source has no authored
/// sheet at all.
pub(super) fn schematic_sheet_relationship(draft: &HardcopyDialogState) -> Option<bool> {
    draft.resolved_document.as_ref().and_then(|document| {
        document
            .schematic_drawing_sheet_has_outside_content()
            .ok()
            .flatten()
    })
}

/// The target in one phrase. A raster format is nothing without its
/// resolution, so it never appears without one.
pub fn output_summary(format: OutputFormat) -> String {
    format.raster_dpi().map_or_else(
        || format_label(format).to_owned(),
        |dpi| {
            format!(
                "{} · {dpi} dpi",
                if matches!(format, OutputFormat::Png { .. }) {
                    "PNG"
                } else {
                    "TIFF"
                }
            )
        },
    )
}

/// A refusal as the strip states it, in sentence case. The errors behind it
/// are written as Rust errors are, in lower case, and the strip read
/// "choose a printer to print to".
fn sentence_case(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(chars).collect()
    })
}

/// The track a two-up form row needs before it stops stacking.
const PAIRED_TRACK_MINIMUM: f32 = 430.0;
fn dialog_split(ui: &mut Ui, mut render_column: impl FnMut(&mut Ui, usize)) {
    let t = Tokens::get(ui.ctx());
    let stacked = ui.ctx().content_rect().width() <= 760.0;
    let mut horizontal_divider = None;
    let surface = Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .outer_margin(Margin {
            left: 0,
            right: 0,
            top: 10,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            if stacked {
                let first = Frame::NONE.inner_margin(Margin::same(10)).show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;
                    render_column(ui, 0);
                });
                horizontal_divider = Some(first.response.rect.bottom());
                Frame::NONE.inner_margin(Margin::same(10)).show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;
                    render_column(ui, 1);
                });
            } else {
                ui.columns(2, |columns| {
                    split_column(&mut columns[0], |ui| render_column(ui, 0));
                    split_column(&mut columns[1], |ui| render_column(ui, 1));
                });
            }
        });
    if let Some(y) = horizontal_divider {
        ui.painter().hline(
            surface.response.rect.x_range(),
            y,
            Stroke::new(1.0, t.color.border_strong),
        );
    } else {
        ui.painter().vline(
            surface.response.rect.center().x,
            surface.response.rect.y_range(),
            Stroke::new(1.0, t.color.border_strong),
        );
    }
}

fn mapping_cell<R>(ui: &mut Ui, height: f32, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    Frame::NONE
        .inner_margin(Margin::symmetric(8, 0))
        .show(ui, |ui| {
            ui.set_min_height(height);
            ui.with_layout(Layout::left_to_right(Align::Center), add_contents)
                .inner
        })
        .inner
}

/// What the mapping page cannot show as a control: why the redundancy columns
/// exist at all. Where a mapping is saved is not one of these — the scope
/// control above decides it, and a note restating a live control only invites
/// the reader to look for a second place to set it.
fn mapping_note_grid(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .outer_margin(Margin {
            left: 0,
            right: 0,
            top: 10,
            bottom: 0,
        })
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = 4.0;
            note_text(
                ui,
                "Color-safe output",
                "Dash, marker, hatch, and label redundancy keeps traces and layers distinguishable in grayscale and common color-vision deficiencies.",
            );
        });
}

fn content_inputs(draft: &HardcopyDialogState) -> ContentInputs {
    ContentInputs {
        paper: draft.paper.clone(),
        display_unit: draft.display_unit,
        orientation: draft.orientation,
        margins: [
            draft.margin_top.clone(),
            draft.margin_right.clone(),
            draft.margin_bottom.clone(),
            draft.margin_left.clone(),
        ],
        bleed: draft.bleed.clone(),
        scale: draft.scale,
        custom_scale_percent: draft.custom_scale_percent.clone(),
        tiling: draft.tiling,
        manual_columns: draft.manual_columns.clone(),
        manual_rows: draft.manual_rows.clone(),
        overlap: draft.overlap.clone(),
        registration_marks: draft.registration_marks,
        schematic: SchematicHardcopySetup::new(
            draft.schematic_extent,
            draft.outside_sheet_content,
            draft.crop_marks,
            draft.include_sheet_paper,
            draft.include_sheet_border,
            draft.include_sheet_title_block,
            draft.include_sheet_zones,
            draft.include_schematic_grid,
        ),
        printer_id: draft.printer_id.clone(),
        printer_job: draft.printer_job.clone(),
        page_range: draft.page_range,
    }
}

fn rendering_inputs(draft: &HardcopyDialogState) -> RenderingInputs {
    RenderingInputs {
        format: draft.format,
        color_mapping: draft.color_mapping,
        background: draft.background,
        embed_fonts: draft.embed_fonts,
        searchable_text: draft.searchable_text,
        soft_proof: draft.soft_proof,
        decorations: (
            draft.include_legends,
            draft.include_header,
            draft.include_provenance,
            draft.watermark.clone(),
        ),
        plot_overlays: (draft.include_cursors_markers, draft.include_notes),
    }
}

fn field(ui: &mut Ui, label: &str, add: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.label(egui::RichText::new(label).size(tokens::FS_0));
        add(ui);
    });
}

/// One row of the settings form: two fields side by side where the track can
/// hold them, stacked where it cannot.
///
/// This exists because `horizontal_wrapped` cannot do the second case
/// cleanly. A pair that overflows its track wraps, and the wrapped row still
/// reserves height for the tallest thing that was *going* to be in it, so the
/// column grows an empty band under every other field. Stacking is a layout
/// decision, so the layout makes it rather than discovering it.
pub(super) struct FormRow<'a> {
    ui: &'a mut Ui,
    stacked: bool,
    narrow: f32,
    wide: f32,
}

impl FormRow<'_> {
    fn cell(&mut self, width: f32, add: impl FnOnce(&mut Ui)) {
        if self.stacked {
            add(self.ui);
            return;
        }
        self.ui
            .allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
                // The requested track is not binding on its own — a Ui shrinks
                // to its content and the neighbour then slides inward.
                ui.set_width(width);
                add(ui);
            });
    }

    pub(super) fn narrow(&mut self, add: impl FnOnce(&mut Ui)) {
        self.cell(self.narrow, add);
    }

    pub(super) fn wide(&mut self, add: impl FnOnce(&mut Ui)) {
        self.cell(self.wide, add);
    }
}

pub(super) fn form_row(ui: &mut Ui, add: impl FnOnce(&mut FormRow<'_>)) {
    let (narrow, wide) = form_grid_widths(ui);
    let stacked = narrow + wide + 11.0 > ui.available_width() + 0.5;
    if stacked {
        add(&mut FormRow {
            ui,
            stacked,
            narrow,
            wide,
        });
    } else {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 11.0;
            add(&mut FormRow {
                ui,
                stacked,
                narrow,
                wide,
            });
        });
    }
}

fn form_grid_widths(ui: &Ui) -> (f32, f32) {
    let track = ui.available_width();
    if track < PAIRED_TRACK_MINIMUM {
        let width = track.max(120.0);
        return (width, width);
    }
    let available = (track - 11.0).max(240.0);
    let narrow = (available * 0.4).max(110.0);
    (narrow, (available - narrow).max(130.0))
}

fn length_field(ui: &mut Ui, label: &str, value: &mut String, unit: LengthUnit) {
    let unit_label = match unit {
        LengthUnit::Inches => "in",
        LengthUnit::Millimetres => "mm",
    };
    field(ui, label, |ui| {
        let visuals = ui.style().visuals.widgets.inactive;
        let available_width = ui.available_width();
        let control_height = ui.spacing().interact_size.y;
        Frame::NONE
            .fill(visuals.bg_fill)
            .stroke(visuals.bg_stroke)
            .corner_radius(visuals.corner_radius)
            .inner_margin(Margin::symmetric(6, 0))
            .show(ui, |ui| {
                // The frame's margin and its stroke both sit outside this
                // width; counting only the margin made the field two points
                // wider than its column and widened the dialog on that step.
                let border = 2.0 * visuals.bg_stroke.width;
                ui.set_min_size(vec2(
                    (available_width - 12.0 - border).max(76.0),
                    control_height,
                ));
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let suffix_width = 28.0;
                    ui.add_sized(
                        vec2(
                            (ui.available_width() - suffix_width).max(48.0),
                            control_height,
                        ),
                        egui::TextEdit::singleline(value)
                            .font(theme::field_mono())
                            .frame(Frame::NONE),
                    );
                    let suffix = ui.allocate_ui_with_layout(
                        vec2(suffix_width, control_height),
                        Layout::centered_and_justified(egui::Direction::LeftToRight),
                        |ui| {
                            ui.label(
                                egui::RichText::new(unit_label)
                                    .monospace()
                                    .color(ui.visuals().weak_text_color()),
                            );
                        },
                    );
                    ui.painter().vline(
                        suffix.response.rect.left(),
                        suffix.response.rect.y_range().shrink(4.0),
                        visuals.bg_stroke,
                    );
                });
            });
    });
}

fn enum_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    id: impl std::hash::Hash + std::fmt::Debug,
    value: &mut T,
    options: &[(T, &'static str)],
) {
    let selected = options
        .iter()
        .find(|(candidate, _)| candidate == value)
        .map_or("Custom", |(_, label)| *label);
    field(ui, label, |ui| {
        crate::ui::widgets::Combo::from_id_salt(id)
            .selected_text(selected)
            .width(ui.available_width())
            .show_ui(ui, |ui| {
                for (candidate, label) in options {
                    ui.selectable_value(value, *candidate, *label);
                }
            });
    });
}

fn property(ui: &mut Ui, label: &str, value: &str) {
    let width = ui.available_width();
    let content_width = (width - 8.0).max(0.0);
    ui.allocate_ui_with_layout(
        vec2(width, 29.0),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;
            ui.allocate_ui_with_layout(
                vec2(content_width * 0.4, 29.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(label)
                                .size(11.0)
                                .color(ui.visuals().weak_text_color()),
                        )
                        .truncate(),
                    );
                },
            );
            ui.allocate_ui_with_layout(
                vec2(content_width * 0.6, 29.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.add(
                        egui::Label::new(egui::RichText::new(value).monospace().size(11.0))
                            .truncate(),
                    );
                },
            );
        },
    );
}

fn property_list(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::NONE
        .inner_margin(Margin {
            left: 10,
            right: 10,
            top: 7,
            bottom: 10,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            add_contents(ui);
        });
}

fn status_grid(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let status = Frame::NONE
        .inner_margin(Margin::symmetric(10, 8))
        .show(ui, |ui| {
            ui.set_width((width - 20.0).max(0.0));
            add_contents(ui);
        });
    ui.painter().hline(
        status.response.rect.x_range(),
        status.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

fn status_grid_content(ui: &mut Ui, valid: bool, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = 7.0;
        let (dot, _) = ui.allocate_exact_size(vec2(10.0, 16.0), Sense::hover());
        ui.painter().circle_filled(
            dot.center(),
            3.0,
            if valid { t.color.ok } else { t.color.err },
        );
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            ui.label(egui::RichText::new(title).strong().size(tokens::FS_0));
            ui.label(
                egui::RichText::new(detail)
                    .size(tokens::FS_0)
                    .color(t.color.text_dim),
            );
        });
    });
}

fn note_block(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::same(8))
        .show(ui, |ui| {
            ui.strong(title);
            ui.label(detail);
        });
}

fn note_text(ui: &mut Ui, title: &str, detail: &str) {
    ui.strong(title);
    ui.label(
        egui::RichText::new(detail)
            .size(tokens::FS_0)
            .color(ui.visuals().weak_text_color()),
    );
}

fn workflow_status(ui: &mut Ui, valid: bool, title: &str, detail: &str) {
    status_grid(ui, |ui| status_grid_content(ui, valid, title, detail));
}

pub(super) fn standard_paper_label(paper: StandardPaper) -> &'static str {
    match paper {
        StandardPaper::Letter => "US Letter · 8.5 × 11 in",
        StandardPaper::Legal => "Legal · 8.5 × 14 in",
        StandardPaper::Tabloid => "Tabloid · 11 × 17 in",
        StandardPaper::A4 => "A4 · 210 × 297 mm",
        StandardPaper::A3 => "A3 · 297 × 420 mm",
        StandardPaper::A2 => "A2 · 420 × 594 mm",
        StandardPaper::A1 => "A1 · 594 × 841 mm",
        StandardPaper::A0 => "A0 · 841 × 1189 mm",
    }
}

fn source_choice_for_active_extent(
    candidates: &[crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor],
    candidate: Option<
        &crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor,
    >,
) -> Option<(String, HardcopyScope)> {
    let active_kind = candidate.map(|candidate| candidate.document_kind);
    candidate
        .into_iter()
        .chain(candidates.iter().filter(|choice| {
            Some(choice.document_kind) == active_kind
                && candidate.is_none_or(|active| active.source_key != choice.source_key)
        }))
        .filter(|choice| choice.availability.is_available())
        .find_map(|choice| {
            choice
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
                .map(|scope| (choice.source_key.clone(), scope))
        })
}

fn source_choice_for_scope(
    candidates: &[crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor],
    candidate: Option<
        &crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor,
    >,
    scope: HardcopyScope,
) -> Option<(String, HardcopyScope)> {
    let active_kind = candidate.map(|candidate| candidate.document_kind);
    candidate
        .into_iter()
        .chain(candidates.iter().filter(|choice| {
            Some(choice.document_kind) == active_kind
                && candidate.is_none_or(|active| active.source_key != choice.source_key)
        }))
        .find(|choice| choice.availability.is_available() && choice.supports_scope(&scope))
        .map(|choice| (choice.source_key.clone(), scope))
}

fn named_source_choices(
    candidates: &[crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor],
    candidate: Option<
        &crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor,
    >,
) -> Vec<(String, HardcopyScope, String)> {
    let active_kind = candidate.map(|candidate| candidate.document_kind);
    candidates
        .iter()
        .filter(|choice| {
            choice.availability.is_available() && Some(choice.document_kind) == active_kind
        })
        .flat_map(|choice| {
            choice.allowed_scopes.iter().filter_map(|scope| {
                let HardcopyScope::NamedPrintSet(name) = scope else {
                    return None;
                };
                Some((choice.source_key.clone(), scope.clone(), name.clone()))
            })
        })
        .collect()
}

fn custom_from_current(draft: &HardcopyDialogState) -> PaperDraft {
    let authored_media = draft
        .resolved_document
        .as_ref()
        .and_then(|document| document.authored_sheet_output_media().ok());
    let (width, height) = match draft.paper.build(draft.display_unit, authored_media) {
        Ok(PaperSize::Standard(paper)) => paper.portrait_dimensions(),
        Ok(PaperSize::Custom(custom)) => custom.dimensions(),
        Ok(PaperSize::MatchAuthoredSheets(media)) => media.first().map_or_else(
            || StandardPaper::Letter.portrait_dimensions(),
            AuthoredSheetMedia::portrait_dimensions,
        ),
        Err(_) => StandardPaper::Letter.portrait_dimensions(),
    };
    PaperDraft::Custom {
        name: "Custom".to_owned(),
        width: format_length_local(width, draft.display_unit),
        height: format_length_local(height, draft.display_unit),
    }
}

fn format_length_local(length: Length, unit: LengthUnit) -> String {
    super::format_length(length, unit)
}

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
fn apply_printer_job(
    draft: &mut HardcopyDialogState,
    capabilities: &crate::workbench::hardcopy_adapters::print::PrinterCapabilitySnapshot,
    paper_id: &str,
    media: PrinterMediaSource,
    dpi: u16,
    duplex: DuplexMode,
    copies: u16,
    collate: bool,
) {
    let orientation = match draft.orientation {
        Orientation::Portrait => ResolvedOrientation::Portrait,
        Orientation::Landscape => ResolvedOrientation::Landscape,
        Orientation::AutomaticPerPage => {
            if draft
                .content_extent
                .is_some_and(|extent| extent.width() > extent.height())
            {
                ResolvedOrientation::Landscape
            } else {
                ResolvedOrientation::Portrait
            }
        }
    };
    let geometry = crate::workbench::hardcopy_adapters::print::resolve_native_printer_mode(
        capabilities,
        paper_id,
        dpi,
        orientation,
    );
    let job = geometry
        .map_err(|error| error.to_string())
        .and_then(|geometry| {
            PrinterJobSettings::try_new(
                capabilities.content_digest(),
                paper_id,
                geometry,
                media,
                dpi,
                duplex,
                copies,
                collate && copies > 1 && capabilities.supports_collation(),
            )
            .map_err(|error| error.to_string())
        });
    match job {
        Ok(job) => {
            apply_suggested_paper(draft, capabilities, paper_id.parse::<i16>().ok());
            draft.set_printer(capabilities.device_id().to_owned(), job);
        }
        Err(error) => draft.error = Some(error.to_string()),
    }
}

fn reconcile_native_printer_job(draft: &mut HardcopyDialogState) -> bool {
    if draft.format != OutputFormat::NativePrinter {
        return true;
    }
    let (Some(capabilities), Some(current_job)) = (
        draft.printer_capabilities.as_ref(),
        draft.printer_job.as_ref(),
    ) else {
        return false;
    };
    let authored_media = draft
        .resolved_document
        .as_ref()
        .and_then(|document| document.authored_sheet_output_media().ok());
    let Ok(paper) = draft.paper.build(draft.display_unit, authored_media) else {
        draft.printer_job = None;
        return false;
    };
    let (width, height) = paper.portrait_dimensions();
    let Some(capability_paper) = capabilities.papers().iter().find(|candidate| {
        let (candidate_width, candidate_height) = candidate.portrait_dimensions_um();
        candidate_width.abs_diff(width.micrometres()) <= 100
            && candidate_height.abs_diff(height.micrometres()) <= 100
    }) else {
        draft.printer_job = None;
        return false;
    };
    let orientation = match draft.orientation {
        Orientation::Portrait => ResolvedOrientation::Portrait,
        Orientation::Landscape => ResolvedOrientation::Landscape,
        Orientation::AutomaticPerPage => {
            if draft
                .content_extent
                .is_some_and(|extent| extent.width() > extent.height())
            {
                ResolvedOrientation::Landscape
            } else {
                ResolvedOrientation::Portrait
            }
        }
    };
    let paper_id = capability_paper.platform_id().to_string();
    let Ok(geometry) = crate::workbench::hardcopy_adapters::print::resolve_native_printer_mode(
        capabilities,
        &paper_id,
        current_job.resolution_dpi(),
        orientation,
    ) else {
        draft.printer_job = None;
        return false;
    };
    let rebuilt = PrinterJobSettings::try_new(
        capabilities.content_digest(),
        paper_id,
        geometry,
        current_job.media_source().clone(),
        current_job.resolution_dpi(),
        current_job.duplex(),
        current_job.copies(),
        current_job.collate(),
    );
    match rebuilt {
        Ok(job) => {
            draft.printer_job = Some(job);
            true
        }
        Err(_) => {
            draft.printer_job = None;
            false
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
fn apply_suggested_paper(
    draft: &mut HardcopyDialogState,
    capabilities: &crate::workbench::hardcopy_adapters::print::PrinterCapabilitySnapshot,
    paper_platform_id: Option<i16>,
) {
    let Some(paper) = paper_platform_id.and_then(|platform_id| {
        capabilities
            .papers()
            .iter()
            .find(|paper| paper.platform_id() == platform_id)
    }) else {
        return;
    };
    let (width, height) = paper.portrait_dimensions_um();
    draft.paper = [
        StandardPaper::Letter,
        StandardPaper::Legal,
        StandardPaper::Tabloid,
        StandardPaper::A4,
        StandardPaper::A3,
        StandardPaper::A2,
        StandardPaper::A1,
        StandardPaper::A0,
    ]
    .into_iter()
    .find(|standard| {
        let (standard_width, standard_height) = standard.portrait_dimensions();
        standard_width.micrometres().abs_diff(width) <= 100
            && standard_height.micrometres().abs_diff(height) <= 100
    })
    .map_or_else(
        || PaperDraft::Custom {
            name: paper.display_name().to_owned(),
            width: format_length_local(Length::from_micrometres(width), draft.display_unit),
            height: format_length_local(Length::from_micrometres(height), draft.display_unit),
        },
        PaperDraft::Standard,
    );
    draft.refresh_preview();
}

#[cfg(test)]
mod tests;

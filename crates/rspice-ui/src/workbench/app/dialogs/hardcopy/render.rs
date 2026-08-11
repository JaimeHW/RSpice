//! Mockup-owned hardcopy dialogs.
//!
//! This surface deliberately renders the renderer-owned page raster. It never
//! takes a screenshot of the viewport, so preview, export, and print all share
//! one semantic source and one immutable pagination plan.

mod bodies;
mod content_options;

use bodies::*;
use content_options::*;

use egui::{
    Align, ColorImage, Context, Frame, Layout, Margin, Rect, Sense, Stroke, TextureHandle,
    TextureOptions, Ui, Vec2, vec2,
};

use crate::hardcopy::{
    AuthoredSheetMedia, BackgroundMode, ColorMapping, Length, LengthUnit, MAX_RASTER_DPI,
    MIN_RASTER_DPI, Orientation, OutputFormat, OutsideSheetContentPolicy, PaperSize, PrintColor,
    PrintMappingEntry, PrintMappingTable, PrintRedundancy, ScaleMode, SchematicHardcopyExtent,
    StandardPaper, TilingMode, Watermark,
};
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, Dialog, DialogChoice, DialogSize, DialogTransactionTone, IconButton,
};
use crate::workbench::app::RSpiceApp;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::hardcopy_adapters::render::HardcopyRenderer;
use crate::workbench::hardcopy_adapters::render::{HardcopyPreviewPage, max_raster_dpi};

use super::{
    DEFAULT_RASTER_DPI, HardcopyDialogPage, HardcopyDialogState, HardcopySection, HardcopyWorkflow,
    PaperDraft, publish,
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
    String,
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
enum BodyAction {
    None,
    PrinterProperties,
    CustomPaper,
    PrintMapping,
    SelectPrinter(String),
    DriverProperties,
    SelectSource {
        source_key: String,
        scope: crate::hardcopy::HardcopyScope,
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
    schematic: crate::hardcopy::SchematicHardcopySetup,
    printer_id: String,
    printer_job: Option<crate::hardcopy::PrinterJobSettings>,
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
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_hardcopy_dialog(&mut self, ctx: &Context) {
        publish::register_repaint_context(ctx);
        if !self.state.dialogs.hardcopy.open {
            publish::cancel_source_resolution();
            publish::cancel_publication();
            publish::poll_publication(self);
            cancel_preview_worker();
            clear_preview_textures(ctx);
            return;
        }
        publish::poll_source_resolution(self);
        publish::poll_printer_catalog(self);
        publish::poll_publication(self);
        if !self.state.dialogs.hardcopy.open {
            clear_preview_textures(ctx);
            return;
        }
        if self.state.dialogs.hardcopy.printer_discovery_busy || self.state.dialogs.hardcopy.busy {
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
        ensure_preview(ctx, &mut self.state.dialogs.hardcopy);
        let page = self.state.dialogs.hardcopy.page;
        let main_eyebrow = format!(
            "FILE · {} · OUTPUT CONTRACT",
            self.state
                .dialogs
                .hardcopy
                .source
                .as_ref()
                .map_or("ENGINEERING DOCUMENT", |source| {
                    document_kind_label(source.document_kind())
                })
                .to_uppercase()
        );
        let (eyebrow, title, primary, secondary): (&str, &str, &str, Option<&str>) = match page {
            HardcopyDialogPage::Main => (
                main_eyebrow.as_str(),
                self.state.dialogs.hardcopy.workflow.title(),
                self.state.dialogs.hardcopy.workflow.primary_label(),
                None,
            ),
            HardcopyDialogPage::PrinterProperties => (
                "HARDCOPY · DEVICE CAPABILITIES · DRIVER BOUNDARY",
                "Printer properties",
                "Apply printer properties",
                None,
            ),
            HardcopyDialogPage::CustomPaperMargins => (
                "HARDCOPY · PHYSICAL PAGE · SAFE PRINTABLE AREA",
                "Custom paper and margins",
                "Use custom page",
                None,
            ),
            HardcopyDialogPage::PrintMapping => (
                "HARDCOPY · SEMANTIC STYLE · ACCESSIBLE REDUNDANCY",
                "Layer, trace, and marker print mapping",
                "Save print mapping",
                None,
            ),
        };
        let error = self.state.dialogs.hardcopy.error.clone();
        let busy = self.state.dialogs.hardcopy.busy;
        let plan_valid = self.state.dialogs.hardcopy.preview_plan.is_some();
        let description = match page {
            HardcopyDialogPage::Main => {
                "Configure the exact hardcopy source, physical page, semantic styling, and publication target before previewing or publishing."
            }
            HardcopyDialogPage::PrinterProperties => {
                "Review and apply the capabilities reported by the selected physical printer."
            }
            HardcopyDialogPage::CustomPaperMargins => {
                "Define a custom physical page and safe printable margins for this hardcopy operation."
            }
            HardcopyDialogPage::PrintMapping => {
                "Map schematic layers, traces, and markers to accessible hardcopy styles."
            }
        };
        let mut body_scroll_offset = self.state.dialogs.hardcopy.body_scroll_offset;
        let mut body_action = BodyAction::None;
        // The main page is a studio, not a form: a fixed section rail, one
        // section's fields, and a full-height preview beside them. Showing
        // every control at once could only ever fit the surface by accident,
        // and did not — the settings ran off the bottom and took the preview
        // and the command row with them. Showing one section at a time fits by
        // construction, so the dialog never scrolls no matter how many
        // controls a hardcopy grows.
        let mut dialog = Dialog::new(eyebrow, title, primary)
            .description(description)
            .size(match page {
                HardcopyDialogPage::Main => DialogSize::SchematicPageSetup,
                _ => DialogSize::SimulationWorkflow,
            })
            .flush_body()
            .primary_on_enter(false)
            .primary_enabled(!busy && (plan_valid || page != HardcopyDialogPage::Main))
            .ghost("Cancel");
        dialog = if page == HardcopyDialogPage::Main {
            dialog.fixed_height(760.0).manual_body_scroll()
        } else {
            dialog.body_scroll_offset(&mut body_scroll_offset)
        };
        if let Some(label) = secondary {
            dialog = dialog.secondary(label);
        }
        if let Some(error) = error.as_deref() {
            dialog = dialog.transaction_state(
                DialogTransactionTone::Error,
                "Hardcopy configuration blocked",
                error,
            );
        }
        let choice = dialog.show(ctx, |ui| {
            ui.add_enabled_ui(!busy, |ui| {
                body_action = match page {
                    HardcopyDialogPage::Main => main_body(ui, &mut self.state.dialogs.hardcopy),
                    HardcopyDialogPage::PrinterProperties => {
                        printer_properties_body(ui, &mut self.state.dialogs.hardcopy)
                    }
                    HardcopyDialogPage::CustomPaperMargins => {
                        custom_paper_body(ui, &mut self.state.dialogs.hardcopy);
                        BodyAction::None
                    }
                    HardcopyDialogPage::PrintMapping => {
                        print_mapping_body(ui, &mut self.state.dialogs.hardcopy);
                        BodyAction::None
                    }
                };
            });
        });
        self.state.dialogs.hardcopy.body_scroll_offset = body_scroll_offset;

        match body_action {
            BodyAction::None => {}
            BodyAction::PrinterProperties => self
                .state
                .dialogs
                .hardcopy
                .open_subflow(HardcopyDialogPage::PrinterProperties),
            BodyAction::CustomPaper => self
                .state
                .dialogs
                .hardcopy
                .open_subflow(HardcopyDialogPage::CustomPaperMargins),
            BodyAction::PrintMapping => self
                .state
                .dialogs
                .hardcopy
                .open_subflow(HardcopyDialogPage::PrintMapping),
            BodyAction::SelectPrinter(device_id) => {
                let capabilities = self
                    .state
                    .dialogs
                    .hardcopy
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
                    publish::select_printer_capabilities(self, capabilities, None);
                }
            }
            BodyAction::DriverProperties => {
                #[cfg(target_os = "windows")]
                {
                    let printer_id = self.state.dialogs.hardcopy.printer_id.clone();
                    match crate::workbench::hardcopy_adapters::print::show_native_printer_properties(
                        &printer_id,
                        None,
                    ) {
                        Ok(crate::workbench::hardcopy_adapters::print::PrinterDriverPropertiesOutcome::Accepted {
                            capabilities,
                            suggestion,
                        }) => {
                            apply_suggested_paper(
                                &mut self.state.dialogs.hardcopy,
                                &capabilities,
                                suggestion.paper_platform_id,
                            );
                            publish::select_printer_capabilities(
                                self,
                                *capabilities,
                                Some(suggestion),
                            );
                        }
                        Ok(crate::workbench::hardcopy_adapters::print::PrinterDriverPropertiesOutcome::Cancelled) => {}
                        Err(error) => {
                            self.state.dialogs.hardcopy.error = Some(error.to_string());
                        }
                    }
                }
            }
            BodyAction::SelectSource { source_key, scope } => {
                publish::select_retained_source(self, &source_key, scope);
            }
        }
        match choice {
            DialogChoice::Primary if page == HardcopyDialogPage::Main => {
                publish::commit_hardcopy_workflow(self);
            }
            DialogChoice::Primary => {
                if page == HardcopyDialogPage::PrinterProperties
                    && self.state.dialogs.hardcopy.format == OutputFormat::NativePrinter
                    && !reconcile_native_printer_job(&mut self.state.dialogs.hardcopy)
                {
                    self.state.dialogs.hardcopy.error = Some(
                        "The selected paper, orientation, and resolution are not supported by the current printer."
                            .to_owned(),
                    );
                    return;
                }
                if let Err(error) = self.state.dialogs.hardcopy.accept_subflow() {
                    self.state.dialogs.hardcopy.error = Some(error.to_string());
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled if page != HardcopyDialogPage::Main => {
                self.state.dialogs.hardcopy.cancel_subflow();
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                publish::cancel_source_resolution();
                publish::cancel_publication();
                cancel_preview_worker();
                clear_preview_textures(ctx);
                self.state.dialogs.hardcopy.close();
            }
            DialogChoice::Secondary | DialogChoice::None => {}
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn ensure_preview(ctx: &Context, draft: &mut HardcopyDialogState) {
    if draft.preview.is_some()
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
                    Err("The preview worker stopped before returning a result.".to_owned()),
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
            }
            Ok(_) => {}
            Err(error) => {
                draft.preview_failed_generation = Some(completed_generation);
                draft.error = Some(error);
            }
        }
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
                72,
                || worker_cancelled.load(std::sync::atomic::Ordering::Acquire),
            )
            .map_err(|error| format!("The exact preview could not be rendered: {error}"))
            .and_then(|mut pages| {
                if pages.is_empty() {
                    return Err("The preview worker returned no selected page.".to_owned());
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
    if draft.preview.is_some()
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
        let decoded = result.and_then(|buffers| {
            if buffers.len() != page_indices.len() * 2 {
                return Err("Browser preview returned the wrong buffer count.".to_owned());
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
                    .map_err(|error| error.to_string())?,
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
            }
            Ok(_) => {
                draft.preview_failed_generation = Some(ticket.generation);
                draft.error = Some("Browser preview returned no selected page.".to_owned());
            }
            Err(error) => {
                draft.preview_failed_generation = Some(ticket.generation);
                draft.error = Some(format!("The exact preview could not be rendered: {error}"));
            }
        }
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
        72,
        draft.source_resolution_generation,
        draft.preview_generation,
        ctx.clone(),
    ) {
        Ok(ticket) => PREVIEW_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveBrowserPreview {
                ticket,
                page_indices,
                dpi: 72,
            });
        }),
        Err(error) => {
            draft.preview_failed_generation = Some(draft.preview_generation);
            draft.error = Some(format!("Could not start browser preview worker: {error}"));
        }
    }
}

/// The section rail's fixed track, matched to the drawing-sheet studio so the
/// two surfaces a hardcopy passes through do not shift under the pointer.
const NAV_WIDTH: f32 = 230.0;
/// Below this the rail, the editor and the preview cannot all hold the width
/// their contents need, so the surface stacks and scrolls deliberately rather
/// than clipping a form nobody can reach.
const STUDIO_STACK_WIDTH: f32 = 980.0;
const PANEL_INSET_X: i8 = 18;
const PANEL_INSET_Y: i8 = 16;
/// Height of one row of the preview's estimate strip. The strip's total height
/// is reserved before it is drawn, so this is the contract the rows keep: a row
/// that laid itself out freely made the column taller than the height it was
/// assigned, and the whole studio then hung 40 pt below the rail beside it.
const FACT_ROW_HEIGHT: f32 = 20.0;
const FACT_STRIP_INSET: f32 = 20.0;

fn main_body(ui: &mut Ui, draft: &mut HardcopyDialogState) -> BodyAction {
    // One before/after comparison for the whole surface. Each form used to run
    // its own, so a control could only invalidate the preview if it happened to
    // be declared in the same block as the fields it changed — and with the
    // controls now spread across five sections, that is no longer true of any
    // of them.
    let before_content = content_inputs(draft);
    let before_render = rendering_inputs(draft);
    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

    let sections = available_sections(draft);
    if !sections.contains(&draft.section) {
        draft.section = sections[0];
    }
    let available = ui.available_size();
    let body_height = available.y.max(1.0);
    let mut action = BodyAction::None;
    if available.x < STUDIO_STACK_WIDTH {
        action = stacked_body(ui, draft, &sections, body_height);
    } else {
        let t = Tokens::get(ui.ctx());
        ui.allocate_ui_with_layout(
            vec2(available.x, body_height),
            Layout::left_to_right(Align::Min),
            |ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                let nav = ui.allocate_ui_with_layout(
                    vec2(NAV_WIDTH, body_height),
                    Layout::top_down(Align::Min),
                    |ui| section_navigation(ui, draft, &sections, body_height),
                );
                ui.painter().vline(
                    nav.response.rect.right(),
                    nav.response.rect.y_range(),
                    Stroke::new(1.0, t.color.border),
                );
                // The preview takes 0.82 of every 1.82 units left over, with
                // both columns floored at the width their contents need: a
                // preview too small to read is not worth the space it costs
                // the form, and vice versa.
                let remaining = (available.x - NAV_WIDTH).max(720.0);
                let preview_width = (remaining * (0.82 / 1.82))
                    .max(330.0)
                    .min((remaining - 390.0).max(330.0));
                let editor_width = (remaining - preview_width).max(390.0);
                let editor = ui.allocate_ui_with_layout(
                    vec2(editor_width, body_height),
                    Layout::top_down(Align::Min),
                    |ui| editor_column(ui, draft, Some(body_height)),
                );
                action = editor.inner;
                ui.painter().vline(
                    editor.response.rect.right(),
                    editor.response.rect.y_range(),
                    Stroke::new(1.0, t.color.border),
                );
                ui.allocate_ui_with_layout(
                    vec2(preview_width, body_height),
                    Layout::top_down(Align::Min),
                    |ui| preview_column(ui, draft, body_height),
                );
            },
        );
    }

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

/// The sections this source actually has. A document with no governed drawing
/// sheet has no sheet furniture to configure, so the section is absent rather
/// than present and inert — a rail entry that can never do anything is worse
/// than one that is not there.
fn available_sections(draft: &HardcopyDialogState) -> Vec<HardcopySection> {
    let sheet = schematic_sheet_relationship(draft).is_some();
    HardcopySection::ALL
        .into_iter()
        .filter(|section| sheet || *section != HardcopySection::DrawingSheet)
        .collect()
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

fn section_navigation(
    ui: &mut Ui,
    draft: &mut HardcopyDialogState,
    sections: &[HardcopySection],
    height: f32,
) {
    let t = Tokens::get(ui.ctx());
    // Paint the rail over the column it was assigned rather than over the
    // entries it happens to contain. A Frame would only cover its content, and
    // a rail that stops short of the editor beside it reads as a broken panel.
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, t.color.bg_panel);
    {
        ui.set_width(NAV_WIDTH);
        ui.spacing_mut().item_spacing.y = 0.0;
        for section in sections.iter().copied() {
            let selected = draft.section == section;
            let detail = section_detail(draft, section);
            let entry = Frame::NONE
                .fill(if selected {
                    t.color.bg_active
                } else {
                    t.color.bg_panel
                })
                .inner_margin(Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    let width = ui.available_width();
                    ui.allocate_ui_with_layout(
                        vec2(width, 34.0),
                        Layout::left_to_right(Align::Min),
                        |ui| {
                            ui.set_min_width(width);
                            ui.label(
                                egui::RichText::new(section.number())
                                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                                    .color(if selected {
                                        t.color.accent
                                    } else {
                                        t.color.text_faint
                                    }),
                            );
                            paint_section_icon(ui, section, selected);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(section.label())
                                        .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                        .color(t.color.text),
                                );
                                // Truncated, never wrapped: a two-line summary
                                // would push the entry past the height it was
                                // allocated and ripple down the whole rail.
                                ui.add(
                                    egui::Label::new(
                                        egui::RichText::new(detail)
                                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                            .color(t.color.text_dim),
                                    )
                                    .truncate(),
                                );
                            });
                        },
                    );
                });
            let rect = entry.response.rect;
            if selected {
                ui.painter().rect_filled(
                    Rect::from_min_max(rect.min, egui::pos2(rect.left() + 3.0, rect.bottom())),
                    0.0,
                    t.color.accent,
                );
            }
            let response = entry.response.interact(Sense::click());
            response.widget_info(|| {
                egui::WidgetInfo::selected(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    selected,
                    section.label(),
                )
            });
            theme::paint_focus_ring(ui, &response, rect);
            if response
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                draft.section = section;
            }
            ui.painter().hline(
                rect.x_range(),
                rect.bottom(),
                Stroke::new(1.0, t.color.border),
            );
        }
        // Dock the sealed source identity on the rail's bottom edge with
        // whatever the entries left over. It belongs here rather than in a
        // strip across the top: it is the one fact every section is about, and
        // the rail is the only region that is never replaced.
        let remaining = (height - ui.min_rect().height()).max(12.0);
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), remaining),
            Layout::bottom_up(Align::Min),
            |ui| {
                // Claim the whole leftover, or the bottom-up region collapses
                // to the note's own height and the rail ends above the columns.
                ui.set_min_height(remaining);
                Frame::NONE
                    .fill(t.color.bg_inset)
                    .inner_margin(Margin::same(12))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        // The frame inherits the bottom-up layout, so the note
                        // is added in reverse reading order.
                        let kind = draft.source.as_ref().map_or("unresolved", |source| {
                            document_kind_label(source.document_kind())
                        });
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(kind)
                                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text_dim),
                            )
                            .truncate(),
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(
                                    draft
                                        .source
                                        .as_ref()
                                        .map_or("No active publication source", |source| {
                                            source.display_name()
                                        }),
                                )
                                .font(theme::sans(tokens::FS_1, FontWeight::SemiBold))
                                .color(t.color.text),
                            )
                            .truncate(),
                        );
                        ui.add_space(5.0);
                        ui.label(
                            egui::RichText::new("SEALED PUBLICATION SOURCE")
                                .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                                .color(t.color.accent),
                        );
                    });
            },
        );
    }
}

fn paint_section_icon(ui: &mut Ui, section: HardcopySection, selected: bool) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
    let color = if selected {
        t.color.accent
    } else {
        t.color.text_dim
    };
    let stroke = Stroke::new(1.0, color);
    let page = Rect::from_center_size(rect.center(), vec2(13.0, 15.0));
    let painter = ui.painter();
    painter.rect_stroke(page, 0.5, stroke, egui::StrokeKind::Inside);
    match section {
        // A turned corner: the retained document, sealed as it was read.
        HardcopySection::Source => {
            painter.line_segment(
                [
                    egui::pos2(page.right() - 4.5, page.top()),
                    egui::pos2(page.right(), page.top() + 4.5),
                ],
                Stroke::new(0.8, color),
            );
        }
        // Tile seams across the sheet: pagination.
        HardcopySection::Page => {
            painter.vline(page.center().x, page.y_range(), Stroke::new(0.6, color));
            painter.hline(page.x_range(), page.center().y, Stroke::new(0.6, color));
        }
        // Border and title block: the authored drawing sheet's furniture.
        HardcopySection::DrawingSheet => {
            painter.rect_stroke(
                page.shrink(2.5),
                0.0,
                Stroke::new(0.6, color),
                egui::StrokeKind::Inside,
            );
            painter.rect_stroke(
                Rect::from_min_max(
                    egui::pos2(page.center().x, page.bottom() - 4.5),
                    page.right_bottom(),
                ),
                0.0,
                Stroke::new(0.8, color),
                egui::StrokeKind::Inside,
            );
        }
        // A page leaving the surface: the artifact or the spool.
        HardcopySection::Output => {
            let tip = egui::pos2(page.center().x, page.top() + 3.0);
            painter.line_segment(
                [egui::pos2(page.center().x, page.bottom() - 3.0), tip],
                Stroke::new(0.9, color),
            );
            painter.line_segment(
                [tip, egui::pos2(tip.x - 3.0, tip.y + 3.5)],
                Stroke::new(0.9, color),
            );
            painter.line_segment(
                [tip, egui::pos2(tip.x + 3.0, tip.y + 3.5)],
                Stroke::new(0.9, color),
            );
        }
        // Ruled lines: the legends, header and provenance a page carries.
        HardcopySection::Identity => {
            for offset in [4.5, 8.0, 11.5] {
                painter.hline(
                    egui::Rangef::new(page.left() + 3.0, page.right() - 3.0),
                    page.top() + offset,
                    Stroke::new(0.7, color),
                );
            }
        }
    }
}

/// The one-line summary a section shows while another is open. Without these
/// the rail would be a filing cabinet; with them nothing is hidden, only
/// deferred.
fn section_detail(draft: &HardcopyDialogState, section: HardcopySection) -> String {
    match section {
        HardcopySection::Source => draft.source.as_ref().map_or_else(
            || "unresolved".to_owned(),
            |source| detailed_scope_label(source.scope()).to_lowercase(),
        ),
        HardcopySection::Page => format!(
            "{} · {}",
            short_paper_label(&draft.paper),
            orientation_word(draft.orientation)
        ),
        HardcopySection::DrawingSheet => match draft.schematic_extent {
            SchematicHardcopyExtent::CompleteSchematicContent => "complete content".to_owned(),
            // The outside-content policy only decides anything when there is
            // outside content. Reporting "undecided" for a drawing that fits
            // its sheet would name a decision nobody has to make.
            SchematicHardcopyExtent::AuthoredDrawingSheet
                if schematic_sheet_relationship(draft) == Some(false) =>
            {
                "authored sheet".to_owned()
            }
            SchematicHardcopyExtent::AuthoredDrawingSheet => format!(
                "authored · {}",
                match draft.outside_sheet_content {
                    OutsideSheetContentPolicy::Ask => "undecided",
                    OutsideSheetContentPolicy::ClipToAuthoredSheet => "clipped",
                    OutsideSheetContentPolicy::ExtendOutput => "extended",
                }
            ),
        },
        HardcopySection::Output => output_summary(draft.format),
        HardcopySection::Identity => {
            let included = usize::from(draft.include_legends)
                + usize::from(draft.include_header)
                + usize::from(draft.include_provenance)
                + usize::from(!matches!(draft.watermark, Watermark::None));
            format!("{included} of 4 marks")
        }
    }
}

fn short_paper_label(paper: &PaperDraft) -> String {
    match paper {
        PaperDraft::Standard(paper) => match paper {
            StandardPaper::Letter => "US Letter",
            StandardPaper::Legal => "Legal",
            StandardPaper::Tabloid => "US Ledger",
            StandardPaper::A4 => "A4",
            StandardPaper::A3 => "A3",
            StandardPaper::A2 => "A2",
            StandardPaper::A1 => "A1",
            StandardPaper::A0 => "A0",
        }
        .to_owned(),
        PaperDraft::MatchAuthoredSheets => "per authored sheet".to_owned(),
        PaperDraft::Custom { name, .. } => name.clone(),
    }
}

const fn orientation_word(orientation: Orientation) -> &'static str {
    match orientation {
        Orientation::Landscape => "landscape",
        Orientation::Portrait => "portrait",
        Orientation::AutomaticPerPage => "auto per page",
    }
}

/// The target in one phrase. A raster format is nothing without its
/// resolution, so it never appears without one.
fn output_summary(format: OutputFormat) -> String {
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

/// The editor column. `height` fills the studio's assigned track; `None` sizes
/// to the section's own content, which is what the stacked layout needs — a
/// fixed track there would strand the preview a screen below the form.
fn editor_column(ui: &mut Ui, draft: &mut HardcopyDialogState, height: Option<f32>) -> BodyAction {
    let t = Tokens::get(ui.ctx());
    let mut action = BodyAction::None;
    Frame::NONE.fill(t.color.bg_app).show(ui, |ui| {
        let Some(height) = height else {
            action = editor_panel(ui, draft);
            return;
        };
        ui.set_min_height(height);
        ui.allocate_ui(vec2(ui.available_width(), height.max(120.0)), |ui| {
            // Every section is authored to fit this column outright. The scroll
            // area is the fallback for a hostile font scale, not the layout.
            egui::ScrollArea::vertical()
                .id_salt(("hardcopy-editor", draft.section))
                .auto_shrink([false, false])
                .show(ui, |ui| action = editor_panel(ui, draft));
        });
    });
    action
}

fn editor_panel(ui: &mut Ui, draft: &mut HardcopyDialogState) -> BodyAction {
    section_panel_header(ui, draft.section, draft.workflow);
    Frame::NONE
        .inner_margin(Margin::symmetric(PANEL_INSET_X, PANEL_INSET_Y))
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 6.0;
            match draft.section {
                HardcopySection::Source => source_panel(ui, draft),
                HardcopySection::Page => page_panel(ui, draft),
                HardcopySection::DrawingSheet => {
                    drawing_sheet_panel(ui, draft);
                    BodyAction::None
                }
                HardcopySection::Output => output_panel(ui, draft),
                HardcopySection::Identity => {
                    identity_panel(ui, draft);
                    BodyAction::None
                }
            }
        })
        .inner
}

fn section_panel_header(ui: &mut Ui, section: HardcopySection, workflow: HardcopyWorkflow) {
    let (eyebrow, description) = match section {
        HardcopySection::Source => (
            "SEALED INPUT",
            "Choose the retained document and the extent of it this hardcopy publishes. Both are resolved again before execution.",
        ),
        HardcopySection::Page => (
            "OUTPUT MEDIA",
            "Physical media, scale and tiling for the output. None of it rewrites the authored drawing sheet.",
        ),
        HardcopySection::DrawingSheet => (
            "SHEET FURNITURE",
            "Decide how much of the authored sheet is drawn, and what becomes of content that crosses it.",
        ),
        HardcopySection::Output => (
            if workflow == HardcopyWorkflow::Export {
                "ARTIFACT TARGET"
            } else {
                "PUBLICATION TARGET"
            },
            "Format, resolution and color. Each option is bounded by what this host can actually produce.",
        ),
        HardcopySection::Identity => (
            "PROVENANCE",
            "Legends, headers, provenance and watermark carried on every published page.",
        ),
    };
    panel_header(ui, eyebrow, section.label(), description);
}

fn panel_header(ui: &mut Ui, eyebrow: &str, title: &str, description: &str) {
    let t = Tokens::get(ui.ctx());
    let header = Frame::NONE
        .fill(t.color.bg_elevated)
        .inner_margin(Margin::symmetric(18, 14))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.label(
                egui::RichText::new(eyebrow)
                    .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
                    .color(t.color.accent),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(title)
                    .font(theme::sans(tokens::FS_3, FontWeight::SemiBold))
                    .color(t.color.text),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new(description)
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
            );
        });
    ui.painter().hline(
        header.response.rect.x_range(),
        header.response.rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
}

/// A supporting flow, offered where the settings it owns are edited rather
/// than from a command strip that belonged to no section in particular.
pub(super) fn action_row(
    ui: &mut Ui,
    label: &str,
    detail: &str,
    enabled: bool,
    disabled_hint: &str,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let mut clicked = false;
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(10, 9))
        .show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                let response = Button::new(label).enabled(enabled).show(ui);
                clicked = response.clicked();
                if !enabled {
                    response.on_disabled_hover_text(disabled_hint.to_owned());
                }
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(detail)
                            .size(tokens::FS_0)
                            .color(t.color.text_dim),
                    )
                    .truncate(),
                );
            });
        });
    clicked
}

/// A bordered label/value readout for facts a section is about but does not
/// edit. Values are monospaced and truncated, never wrapped, so the block's
/// height is a function of how many facts there are and nothing else.
pub(super) fn fact_list(ui: &mut Ui, facts: &[(&'static str, String)]) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE
        .fill(t.color.bg_inset)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            let content_width = ui.available_width();
            ui.set_min_width(content_width);
            ui.spacing_mut().item_spacing = vec2(10.0, 0.0);
            for (label, value) in facts {
                ui.allocate_ui_with_layout(
                    vec2(content_width, FACT_ROW_HEIGHT),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.allocate_ui_with_layout(
                            vec2(118.0, FACT_ROW_HEIGHT),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(label.to_uppercase())
                                        .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                                        .color(t.color.text_faint),
                                );
                            },
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(value)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text),
                            )
                            .truncate(),
                        );
                    },
                );
            }
        });
}

/// The leading bytes of a content digest, which is as much as anyone reads and
/// still far more than enough to tell two revisions apart by eye.
pub(super) fn short_digest(digest: &crate::product::ContentDigest) -> String {
    digest
        .as_bytes()
        .iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn stacked_body(
    ui: &mut Ui,
    draft: &mut HardcopyDialogState,
    sections: &[HardcopySection],
    height: f32,
) -> BodyAction {
    let mut action = BodyAction::None;
    egui::ScrollArea::vertical()
        .id_salt("hardcopy-studio-stacked")
        .show(ui, |ui| {
            Frame::NONE
                .fill(Tokens::get(ui.ctx()).color.bg_panel)
                .inner_margin(Margin::same(10))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        for section in sections.iter().copied() {
                            let label = format!("{}  {}", section.number(), section.label());
                            if ui
                                .selectable_label(draft.section == section, label)
                                .clicked()
                            {
                                draft.section = section;
                            }
                        }
                    });
                });
            action = editor_column(ui, draft, None);
            preview_column(ui, draft, height.clamp(300.0, 460.0));
        });
    action
}

fn preview_column(ui: &mut Ui, draft: &mut HardcopyDialogState, height: f32) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    Frame::NONE.fill(t.color.bg_panel).show(ui, |ui| {
        ui.set_min_size(vec2(width, height));
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        // The three bands divide the column's exact assigned height between
        // them. The page desk takes what the toolbar and the estimate do not,
        // so the preview grows with the surface instead of staying a thumbnail.
        const TOOLBAR_HEIGHT: f32 = 38.0;
        let facts = preview_facts(draft);
        let facts_height = facts.len() as f32 * FACT_ROW_HEIGHT + FACT_STRIP_INSET;
        let desk_height = (height - facts_height - TOOLBAR_HEIGHT).max(160.0);
        preview_desk(ui, draft, width, desk_height);
        preview_toolbar(ui, draft, width, TOOLBAR_HEIGHT);
        preview_facts_strip(ui, &facts, width, facts_height);
    });
}

fn preview_desk(ui: &mut Ui, draft: &mut HardcopyDialogState, width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    Frame::NONE.fill(t.color.bg_inset).show(ui, |ui| {
        ui.set_min_size(vec2(width, height));
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        let Some(count) = draft
            .preview_plan
            .as_ref()
            .map(|plan| plan.pagination().pages().len().clamp(1, 2))
        else {
            ui.allocate_ui_with_layout(
                vec2(width, height),
                Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.add(
                        egui::Label::new("Resolve the configuration to render an exact page.")
                            .wrap(),
                    );
                },
            );
            return;
        };
        // The card is sized from the page, not the other way round: the white
        // area is the sheet plus its margin, so the desk never shows a
        // letterboxed page inside a card of some other proportion.
        let aspect = draft.preview.as_ref().map_or_else(
            || match draft.orientation {
                Orientation::Portrait => 1.0 / std::f32::consts::SQRT_2,
                _ => std::f32::consts::SQRT_2,
            },
            |preview| preview.width() as f32 / preview.height() as f32,
        );
        let columns = count as f32;
        let max_card = vec2(
            ((width - 40.0) - 16.0 * (columns - 1.0)) / columns,
            height - 40.0,
        );
        let image = fit_aspect(
            vec2((max_card.x - 20.0).max(40.0), (max_card.y - 38.0).max(40.0)),
            aspect,
        );
        let card = vec2(image.x + 20.0, image.y + 38.0);
        let group_width = card.x * columns + 16.0 * (columns - 1.0);
        let mut selected_adjacent = None;
        ui.allocate_ui_with_layout(vec2(width, height), Layout::top_down(Align::Min), |ui| {
            ui.set_width(width);
            ui.add_space(((height - card.y) * 0.5).max(0.0));
            ui.horizontal_top(|ui| {
                ui.spacing_mut().item_spacing.x = 16.0;
                ui.add_space(((width - group_width) * 0.5).max(0.0));
                let pages = [draft.preview.as_ref(), draft.preview_adjacent.as_ref()]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>();
                for slot in 0..count {
                    if let Some(preview) = pages.get(slot).copied() {
                        let selected =
                            preview.page_number() == draft.preview_page.saturating_add(1);
                        if preview_card(
                            ui,
                            preview,
                            selected,
                            draft.preview_zoom_percent,
                            slot,
                            card,
                        ) && !selected
                        {
                            selected_adjacent = Some(preview.page_number().saturating_sub(1));
                        }
                    } else {
                        preview_placeholder(ui, slot == 0, card);
                    }
                }
            });
        });
        if let Some(page) = selected_adjacent {
            draft.preview_page = page;
            draft.invalidate_preview_raster();
        }
    });
}

fn preview_toolbar(ui: &mut Ui, draft: &mut HardcopyDialogState, width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let content_height = (height - 12.0).max(1.0);
    let toolbar = Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.set_width((width - 20.0).max(0.0));
            ui.set_min_height(content_height);
            let count = draft
                .preview_plan
                .as_ref()
                .map_or(0, |plan| plan.pagination().pages().len() as u32);
            if count == 0 {
                ui.with_layout(
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.monospace("No valid page");
                    },
                );
                return;
            }
            ui.allocate_ui_with_layout(
                vec2(ui.available_width(), content_height),
                Layout::centered_and_justified(egui::Direction::LeftToRight),
                |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 6.0;
                        if IconButton::new(Icon::ChevronLeft)
                            .tooltip("Previous page")
                            .enabled(draft.preview_page > 0)
                            .show(ui)
                            .clicked()
                        {
                            draft.preview_page -= 1;
                            draft.invalidate_preview_raster();
                        }
                        ui.monospace(format!(
                            "Page {} / {}",
                            draft.preview_page.saturating_add(1),
                            count
                        ));
                        if IconButton::new(Icon::ChevronRight)
                            .tooltip("Next page")
                            .enabled(draft.preview_page + 1 < count)
                            .show(ui)
                            .clicked()
                        {
                            draft.preview_page += 1;
                            draft.invalidate_preview_raster();
                        }
                        ui.separator();
                        if IconButton::new(Icon::ZoomOut)
                            .tooltip("Zoom out")
                            .show(ui)
                            .clicked()
                        {
                            draft.preview_zoom_percent =
                                draft.preview_zoom_percent.saturating_sub(10).max(25);
                        }
                        ui.monospace(format!("{}%", draft.preview_zoom_percent));
                        if IconButton::new(Icon::ZoomIn)
                            .tooltip("Zoom in")
                            .show(ui)
                            .clicked()
                        {
                            draft.preview_zoom_percent =
                                draft.preview_zoom_percent.saturating_add(10).min(200);
                        }
                        if Button::new("Fit page").show(ui).clicked() {
                            draft.preview_zoom_percent = 85;
                        }
                    })
                },
            );
        });
    ui.painter().hline(
        toolbar.response.rect.x_range(),
        toolbar.response.rect.top(),
        Stroke::new(1.0, t.color.border_strong),
    );
}

/// What this configuration will actually produce, as facts rather than as the
/// settings that produced them.
fn preview_facts(draft: &HardcopyDialogState) -> Vec<(&'static str, String)> {
    let pages = draft.preview_plan.as_ref().map_or_else(
        || "no valid page".to_owned(),
        |plan| {
            let pagination = plan.pagination();
            let count = pagination.pages().len();
            let arrangement = if pagination.sections().is_empty() {
                format!(" · {} × {} tiles", pagination.columns(), pagination.rows())
            } else {
                " · section-aware".to_owned()
            };
            format!(
                "{count} page{}{arrangement}",
                if count == 1 { "" } else { "s" }
            )
        },
    );
    vec![
        ("Pages", pages),
        (
            "Media",
            format!(
                "{} · {}",
                short_paper_label(&draft.paper),
                orientation_word(draft.orientation)
            ),
        ),
        ("Target", output_summary(draft.format)),
        ("Scale", scale_label(scale_key(draft.scale)).to_owned()),
        ("Color", color_mapping_label(draft.color_mapping).to_owned()),
    ]
}

fn preview_facts_strip(ui: &mut Ui, facts: &[(&'static str, String)], width: f32, height: f32) {
    let t = Tokens::get(ui.ctx());
    let strip = Frame::NONE
        .fill(t.color.bg_inset)
        .inner_margin(Margin::symmetric(12, 10))
        .show(ui, |ui| {
            let content_width = (width - 24.0).max(0.0);
            ui.set_width(content_width);
            ui.set_min_height((height - FACT_STRIP_INSET).max(0.0));
            ui.spacing_mut().item_spacing = vec2(10.0, 0.0);
            let label_width = 74.0_f32.min(content_width * 0.4);
            for (label, value) in facts {
                // Each row takes exactly the height the strip reserved for it.
                ui.allocate_ui_with_layout(
                    vec2(content_width, FACT_ROW_HEIGHT),
                    Layout::left_to_right(Align::Center),
                    |ui| {
                        ui.allocate_ui_with_layout(
                            vec2(label_width, FACT_ROW_HEIGHT),
                            Layout::left_to_right(Align::Center),
                            |ui| {
                                ui.label(
                                    egui::RichText::new(label.to_uppercase())
                                        .font(theme::sans(tokens::FS_MICRO, FontWeight::Regular))
                                        .color(t.color.text_faint),
                                );
                            },
                        );
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(value)
                                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                    .color(t.color.text),
                            )
                            .truncate(),
                        );
                    },
                );
            }
        });
    ui.painter().hline(
        strip.response.rect.x_range(),
        strip.response.rect.top(),
        Stroke::new(1.0, t.color.border),
    );
}

fn preview_card(
    ui: &mut Ui,
    preview: &HardcopyPreviewPage,
    selected: bool,
    zoom_percent: u16,
    texture_slot: usize,
    size: Vec2,
) -> bool {
    let texture = preview_texture(ui.ctx(), preview, texture_slot);
    let t = Tokens::get(ui.ctx());
    let inner_width = (size.x - 20.0).max(20.0);
    let image_box = vec2(inner_width, (size.y - 38.0).max(20.0));
    let card = Frame::NONE
        .fill(egui::Color32::from_rgb(245, 244, 239))
        .stroke(Stroke::new(1.0, egui::Color32::from_rgb(155, 155, 150)))
        .shadow(egui::epaint::Shadow {
            offset: [0, 4],
            blur: 12,
            spread: 0,
            color: egui::Color32::from_black_alpha(71),
        })
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(inner_width);
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                let (rect, _) = ui.allocate_exact_size(image_box, Sense::hover());
                let aspect = preview.width() as f32 / preview.height() as f32;
                let fitted = fit_aspect(rect.size(), aspect);
                let image_rect = egui::Rect::from_center_size(
                    rect.center(),
                    fitted * (zoom_percent as f32 / 85.0),
                );
                ui.painter().with_clip_rect(rect).image(
                    texture.id(),
                    image_rect,
                    egui::Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
                ui.add_sized(
                    vec2(inner_width, 18.0),
                    egui::Label::new(
                        egui::RichText::new(format!(
                            "Page {} · {}",
                            preview.page_number(),
                            preview.coordinate()
                        ))
                        .monospace()
                        .size(tokens::FS_0)
                        .color(egui::Color32::from_rgb(44, 44, 42)),
                    )
                    .truncate(),
                );
            });
        });
    if selected {
        ui.painter().rect_stroke(
            card.response.rect,
            0.0,
            Stroke::new(2.0, t.color.accent),
            egui::StrokeKind::Outside,
        );
    }
    let response = card.response.interact(Sense::click());
    let label = format!("Page {} · {}", preview.page_number(), preview.coordinate());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Button,
            ui.is_enabled(),
            selected,
            label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, response.rect);
    response.clicked()
}

fn preview_placeholder(ui: &mut Ui, selected: bool, size: Vec2) {
    let t = Tokens::get(ui.ctx());
    let inner_width = (size.x - 20.0).max(20.0);
    let card = Frame::NONE
        .fill(egui::Color32::from_rgb(245, 244, 239))
        .stroke(Stroke::new(1.0, egui::Color32::from_rgb(155, 155, 150)))
        .shadow(egui::epaint::Shadow {
            offset: [0, 4],
            blur: 12,
            spread: 0,
            color: egui::Color32::from_black_alpha(71),
        })
        .inner_margin(Margin::same(10))
        .show(ui, |ui| {
            ui.set_width(inner_width);
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                ui.allocate_ui_with_layout(
                    vec2(inner_width, (size.y - 38.0).max(20.0)),
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.spinner();
                    },
                );
                ui.allocate_ui_with_layout(
                    vec2(inner_width, 18.0),
                    Layout::centered_and_justified(egui::Direction::LeftToRight),
                    |ui| {
                        ui.label(
                            egui::RichText::new("Rendering…")
                                .monospace()
                                .size(tokens::FS_0)
                                .color(egui::Color32::from_rgb(44, 44, 42)),
                        );
                    },
                );
            });
        });
    if selected {
        ui.painter().rect_stroke(
            card.response.rect,
            0.0,
            Stroke::new(2.0, t.color.accent),
            egui::StrokeKind::Outside,
        );
    }
}

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

fn mapping_note_grid(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let stacked = ui.ctx().content_rect().width() <= 760.0;
    let mut divider = None;
    let notes = Frame::NONE
        .stroke(Stroke::new(1.0, t.color.border_strong))
        .outer_margin(Margin {
            left: 0,
            right: 0,
            top: 10,
            bottom: 0,
        })
        .show(ui, |ui| {
            let note = |ui: &mut Ui, title: &str, detail: &str| {
                Frame::NONE
                    .inner_margin(Margin::same(10))
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 4.0;
                        note_text(ui, title, detail)
                    })
            };
            if stacked {
                let first = note(
                    ui,
                    "Color-safe output",
                    "Dash, marker, hatch, and label redundancy keeps traces and layers distinguishable in grayscale and common color-vision deficiencies.",
                );
                divider = Some(first.response.rect.bottom());
                note(
                    ui,
                    "Scope",
                    "Mappings may be saved per document, project print set, or portable personal preset.",
                );
            } else {
                ui.columns(2, |columns| {
                    note(
                        &mut columns[0],
                        "Color-safe output",
                        "Dash, marker, hatch, and label redundancy keeps traces and layers distinguishable in grayscale and common color-vision deficiencies.",
                    );
                    note(
                        &mut columns[1],
                        "Scope",
                        "Mappings may be saved per document, project print set, or portable personal preset.",
                    );
                });
            }
        });
    if let Some(y) = divider {
        ui.painter().hline(
            notes.response.rect.x_range(),
            y,
            Stroke::new(1.0, t.color.border_strong),
        );
    } else {
        ui.painter().vline(
            notes.response.rect.center().x,
            notes.response.rect.y_range(),
            Stroke::new(1.0, t.color.border_strong),
        );
    }
}

fn preview_texture(ctx: &Context, preview: &HardcopyPreviewPage, slot: usize) -> TextureHandle {
    let id = preview_texture_slot_id(slot);
    if let Some(cache) = ctx.data(|data| data.get_temp::<PreviewTexture>(id))
        && cache.digest == preview.digest()
    {
        return cache.handle;
    }
    let image = ColorImage::from_rgba_unmultiplied(
        [preview.width() as usize, preview.height() as usize],
        preview.rgba(),
    );
    let handle = ctx.load_texture(
        "RSpice exact hardcopy preview",
        image,
        TextureOptions::LINEAR,
    );
    ctx.data_mut(|data| {
        data.insert_temp(
            id,
            PreviewTexture {
                digest: preview.digest(),
                handle: handle.clone(),
            },
        );
    });
    handle
}

fn preview_texture_slot_id(slot: usize) -> egui::Id {
    egui::Id::new(("rspice-hardcopy-preview-texture", slot.min(1)))
}

fn fit_aspect(max: egui::Vec2, aspect: f32) -> egui::Vec2 {
    if max.x / max.y > aspect {
        vec2(max.y * aspect, max.y)
    } else {
        vec2(max.x, max.x / aspect)
    }
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
        schematic: crate::hardcopy::SchematicHardcopySetup::new(
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
    }
}

fn field(ui: &mut Ui, label: &str, add: impl FnOnce(&mut Ui)) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 5.0;
        ui.label(egui::RichText::new(label).size(tokens::FS_0));
        add(ui);
    });
}

/// Widths for the two-up form rows.
///
/// Measured against the **track**, not the viewport. Keying off the viewport
/// was the layout's central defect: inside a column each pair was still sized
/// as though it had the whole dialog, never fit, and wrapped — and a wrapped
/// row still reserves its full height, which is where the dead band under
/// every other field came from. When a pair cannot share its track, both
/// halves take the whole of it and stack deliberately instead.
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
    const PAIRED_TRACK_MINIMUM: f32 = 430.0;
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
                ui.set_min_size(vec2((available_width - 12.0).max(76.0), control_height));
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    let suffix_width = 28.0;
                    ui.add_sized(
                        vec2(
                            (ui.available_width() - suffix_width).max(48.0),
                            control_height,
                        ),
                        egui::TextEdit::singleline(value)
                            .font(egui::TextStyle::Monospace)
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
        egui::ComboBox::from_id_salt(id)
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
        StandardPaper::Tabloid => "US Ledger · 11 × 17 in",
        StandardPaper::A4 => "A4 · 210 × 297 mm",
        StandardPaper::A3 => "A3 · 297 × 420 mm",
        StandardPaper::A2 => "A2 · 420 × 594 mm",
        StandardPaper::A1 => "A1 · 594 × 841 mm",
        StandardPaper::A0 => "A0 · 841 × 1189 mm",
    }
}

fn document_kind_label(kind: crate::hardcopy::HardcopyDocumentKind) -> &'static str {
    match kind {
        crate::hardcopy::HardcopyDocumentKind::SchematicOrSymbol => "Schematic / symbol",
        crate::hardcopy::HardcopyDocumentKind::LayoutWithLayerLegend => "Layout + layer legend",
        crate::hardcopy::HardcopyDocumentKind::PlotOrWorksheet => "Plot / worksheet",
        crate::hardcopy::HardcopyDocumentKind::Report => "Report",
        crate::hardcopy::HardcopyDocumentKind::EngineeringDocument => "Engineering document",
    }
}

fn scope_label(scope: &crate::hardcopy::HardcopyScope) -> &'static str {
    use crate::hardcopy::HardcopyScope;
    match scope {
        HardcopyScope::Selection => "Selection",
        HardcopyScope::CurrentSheet => "Current sheet",
        HardcopyScope::VisibleHierarchy => "Visible hierarchy",
        HardcopyScope::ActivePlotDocument => "Active plot document",
        HardcopyScope::CompleteReport => "Complete report",
        HardcopyScope::ActiveDocument => "Active document",
        HardcopyScope::AllSheetsOrPanes => "All sheets / panes",
        HardcopyScope::NamedPrintSet(_) => "Named print set",
    }
}

fn detailed_scope_label(scope: &crate::hardcopy::HardcopyScope) -> &str {
    use crate::hardcopy::HardcopyScope;
    match scope {
        HardcopyScope::Selection => "Selection",
        HardcopyScope::CurrentSheet => "Current sheet",
        HardcopyScope::VisibleHierarchy => "Visible hierarchy",
        HardcopyScope::ActivePlotDocument => "Active plot document",
        HardcopyScope::CompleteReport => "Complete report",
        HardcopyScope::ActiveDocument => "Active document",
        HardcopyScope::AllSheetsOrPanes => "All sheets / panes",
        HardcopyScope::NamedPrintSet(name) => name,
    }
}

fn source_choice_for_active_extent(
    candidates: &[crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor],
    candidate: Option<
        &crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor,
    >,
) -> Option<(String, crate::hardcopy::HardcopyScope)> {
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
                        crate::hardcopy::HardcopyScope::CurrentSheet
                            | crate::hardcopy::HardcopyScope::VisibleHierarchy
                            | crate::hardcopy::HardcopyScope::ActivePlotDocument
                            | crate::hardcopy::HardcopyScope::CompleteReport
                            | crate::hardcopy::HardcopyScope::ActiveDocument
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
    scope: crate::hardcopy::HardcopyScope,
) -> Option<(String, crate::hardcopy::HardcopyScope)> {
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
) -> Vec<(String, crate::hardcopy::HardcopyScope, String)> {
    let active_kind = candidate.map(|candidate| candidate.document_kind);
    candidates
        .iter()
        .filter(|choice| {
            choice.availability.is_available() && Some(choice.document_kind) == active_kind
        })
        .flat_map(|choice| {
            choice.allowed_scopes.iter().filter_map(|scope| {
                let crate::hardcopy::HardcopyScope::NamedPrintSet(name) = scope else {
                    return None;
                };
                Some((choice.source_key.clone(), scope.clone(), name.clone()))
            })
        })
        .collect()
}

fn paper_label(paper: &PaperDraft) -> String {
    match paper {
        PaperDraft::Standard(paper) => standard_paper_label(*paper).to_owned(),
        PaperDraft::MatchAuthoredSheets => "Match each authored sheet".to_owned(),
        PaperDraft::Custom { name, .. } => format!("Custom · {name}"),
    }
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

fn scale_key(value: ScaleMode) -> u8 {
    match value {
        ScaleMode::FitPrintableArea => 0,
        ScaleMode::EngineeringOneToOne => 1,
        ScaleMode::CustomPercent { .. } => 2,
        ScaleMode::FitWidth => 3,
    }
}

fn scale_label(value: u8) -> &'static str {
    match value {
        0 => "Fit to printable area",
        1 => "1:1 engineering scale",
        2 => "Custom",
        _ => "Fit width",
    }
}

fn tiling_key(value: TilingMode) -> u8 {
    match value {
        TilingMode::Automatic => 0,
        TilingMode::SinglePage => 1,
        TilingMode::Manual { .. } => 2,
    }
}

fn format_label(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::NativePrinter => "System printer",
        OutputFormat::BrowserPrintDocument => {
            if cfg!(target_arch = "wasm32") {
                "Browser print dialog"
            } else {
                "Print-ready document handoff"
            }
        }
        OutputFormat::PdfVector => "PDF · vector",
        OutputFormat::PdfA => "PDF/A · vector",
        OutputFormat::SvgVector => "SVG · vector",
        // The resolution is its own field, so the format no longer spells one
        // out. It used to read "PNG · 600 dpi" — a number nobody could change,
        // and on a browser-sized raster budget one nothing could deliver.
        OutputFormat::Png { .. } => "PNG · raster",
        OutputFormat::Tiff { .. } => "TIFF · raster",
    }
}

fn background_label(background: BackgroundMode) -> &'static str {
    match background {
        BackgroundMode::White => "White",
        BackgroundMode::Transparent => "Transparent · vector export",
        BackgroundMode::WorkspaceBackground => "Workspace background",
    }
}

fn color_mapping_label(mapping: ColorMapping) -> &'static str {
    match mapping {
        ColorMapping::PrintSafeEngineeringPalette => "Print-safe engineering palette",
        ColorMapping::ScreenColors => "Screen colors",
        ColorMapping::GrayscaleWithDashMarkerRedundancy => "Grayscale with dash/marker redundancy",
        ColorMapping::Monochrome => "Monochrome",
    }
}

fn print_color_label(value: PrintColor) -> String {
    match value {
        PrintColor::Black => "Black".to_owned(),
        PrintColor::GrayPercent(percent) => format!("{percent}% black"),
        PrintColor::Rgb { red, green, blue } => {
            if let Some(name) = named_print_color(value) {
                format!("{name} · #{red:02X}{green:02X}{blue:02X}")
            } else {
                format!("RGB · #{red:02X}{green:02X}{blue:02X}")
            }
        }
    }
}

fn print_color_options(current: PrintColor) -> Vec<(PrintColor, String)> {
    let mut values = vec![
        PrintColor::Black,
        PrintColor::GrayPercent(40),
        PrintColor::GrayPercent(60),
        PrintColor::GrayPercent(70),
        PrintColor::Rgb {
            red: 0,
            green: 174,
            blue: 199,
        },
        PrintColor::Rgb {
            red: 0,
            green: 128,
            blue: 72,
        },
        PrintColor::Rgb {
            red: 232,
            green: 185,
            blue: 35,
        },
        PrintColor::Rgb {
            red: 210,
            green: 56,
            blue: 56,
        },
        PrintColor::Rgb {
            red: 226,
            green: 126,
            blue: 34,
        },
        PrintColor::Rgb {
            red: 0,
            green: 95,
            blue: 180,
        },
    ];
    if !values.contains(&current) {
        values.insert(0, current);
    }
    values
        .into_iter()
        .map(|value| (value, print_color_label(value)))
        .collect()
}

fn named_print_color(value: PrintColor) -> Option<&'static str> {
    match value {
        PrintColor::Rgb {
            red: 0,
            green: 174,
            blue: 199,
        } => Some("Cyan"),
        PrintColor::Rgb {
            red: 0,
            green: 128,
            blue: 72,
        } => Some("Green"),
        PrintColor::Rgb {
            red: 232,
            green: 185,
            blue: 35,
        } => Some("Yellow"),
        PrintColor::Rgb {
            red: 210,
            green: 56,
            blue: 56,
        } => Some("Red"),
        PrintColor::Rgb {
            red: 226,
            green: 126,
            blue: 34,
        } => Some("Orange"),
        PrintColor::Rgb {
            red: 0,
            green: 95,
            blue: 180,
        } => Some("Engineering blue"),
        _ => None,
    }
}

fn redundancy_options(current: PrintRedundancy) -> Vec<PrintRedundancy> {
    let thin = Length::from_micrometres(200);
    let mut values = vec![
        PrintRedundancy::SourceStyle,
        PrintRedundancy::SolidLine {
            width: Length::from_micrometres(350),
        },
        PrintRedundancy::DashedLine {
            width: Length::from_micrometres(300),
            dash: Length::from_micrometres(2_000),
            gap: Length::from_micrometres(1_000),
        },
        PrintRedundancy::DottedLeader {
            width: thin,
            spacing: Length::from_micrometres(1_250),
        },
        PrintRedundancy::SolidFill,
        PrintRedundancy::CrossHatch {
            line_width: thin,
            spacing: Length::from_micrometres(1_500),
        },
        PrintRedundancy::TriangleWithId {
            size: Length::from_micrometres(3_000),
        },
    ];
    if !values.contains(&current) {
        values.insert(0, current);
    }
    values
}

fn redundancy_label(value: PrintRedundancy) -> String {
    match value {
        PrintRedundancy::SourceStyle => "Source style".to_owned(),
        PrintRedundancy::SolidLine { width } => {
            format!("Solid · {} mm", format_millimetres(width))
        }
        PrintRedundancy::DashedLine { width, dash, gap } => format!(
            "Dash · {} mm · {}/{} mm",
            format_millimetres(width),
            format_millimetres(dash),
            format_millimetres(gap)
        ),
        PrintRedundancy::DottedLeader { width, spacing } => format!(
            "Dotted leader · {} mm · {} mm spacing",
            format_millimetres(width),
            format_millimetres(spacing)
        ),
        PrintRedundancy::SolidFill => "Solid fill".to_owned(),
        PrintRedundancy::CrossHatch {
            line_width,
            spacing,
        } => format!(
            "Cross hatch · {}/{} mm",
            format_millimetres(line_width),
            format_millimetres(spacing)
        ),
        PrintRedundancy::TriangleWithId { size } => {
            format!("Triangle + ID · {} mm", format_millimetres(size))
        }
    }
}

fn format_millimetres(value: Length) -> String {
    let micrometres = value.micrometres();
    let mut value = format!("{}.{:03}", micrometres / 1_000, micrometres % 1_000);
    if value.ends_with('0') {
        value.pop();
    }
    value
}

#[cfg(target_os = "windows")]
fn duplex_label(value: crate::hardcopy::DuplexMode) -> &'static str {
    match value {
        crate::hardcopy::DuplexMode::Off => "Off",
        crate::hardcopy::DuplexMode::LongEdge => "Long edge",
        crate::hardcopy::DuplexMode::ShortEdge => "Short edge",
    }
}

#[cfg(target_os = "windows")]
fn media_label(value: &crate::hardcopy::PrinterMediaSource) -> String {
    match value {
        crate::hardcopy::PrinterMediaSource::AutomaticCompatibleTray => {
            "Automatic compatible tray".to_owned()
        }
        crate::hardcopy::PrinterMediaSource::NamedTray(name) => name.clone(),
        crate::hardcopy::PrinterMediaSource::ManualFeed => "Manual feed".to_owned(),
        crate::hardcopy::PrinterMediaSource::Roll { width } => {
            format!("Roll · {} μm", width.micrometres())
        }
    }
}

#[cfg(target_os = "windows")]
fn apply_printer_job(
    draft: &mut HardcopyDialogState,
    capabilities: &crate::workbench::hardcopy_adapters::print::PrinterCapabilitySnapshot,
    paper_id: &str,
    media: crate::hardcopy::PrinterMediaSource,
    dpi: u16,
    duplex: crate::hardcopy::DuplexMode,
    copies: u16,
    collate: bool,
) {
    let orientation = match draft.orientation {
        Orientation::Portrait => crate::hardcopy::ResolvedOrientation::Portrait,
        Orientation::Landscape => crate::hardcopy::ResolvedOrientation::Landscape,
        Orientation::AutomaticPerPage => {
            if draft
                .content_extent
                .is_some_and(|extent| extent.width() > extent.height())
            {
                crate::hardcopy::ResolvedOrientation::Landscape
            } else {
                crate::hardcopy::ResolvedOrientation::Portrait
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
            crate::hardcopy::PrinterJobSettings::try_new(
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
        Orientation::Portrait => crate::hardcopy::ResolvedOrientation::Portrait,
        Orientation::Landscape => crate::hardcopy::ResolvedOrientation::Landscape,
        Orientation::AutomaticPerPage => {
            if draft
                .content_extent
                .is_some_and(|extent| extent.width() > extent.height())
            {
                crate::hardcopy::ResolvedOrientation::Landscape
            } else {
                crate::hardcopy::ResolvedOrientation::Portrait
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
    let rebuilt = crate::hardcopy::PrinterJobSettings::try_new(
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

#[cfg(target_os = "windows")]
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
mod tests {
    use super::*;

    /// A source with no governed drawing sheet has no sheet furniture to
    /// configure, so the rail must not offer the section — and a selection
    /// left pointing at it must land somewhere real rather than on a blank
    /// editor.
    #[test]
    fn a_source_without_an_authored_sheet_offers_no_sheet_section() {
        let draft = HardcopyDialogState::default();
        let sections = available_sections(&draft);

        assert!(!sections.contains(&HardcopySection::DrawingSheet));
        assert_eq!(sections.first(), Some(&HardcopySection::Source));
        assert!(sections.contains(&HardcopySection::Output));
    }

    /// The rail summarises the sections nobody is looking at, so a summary that
    /// names a decision the document does not pose is worse than none.
    #[test]
    fn the_sheet_summary_only_reports_a_decision_there_is_one_to_make() {
        let mut draft = HardcopyDialogState::default();
        draft.outside_sheet_content = OutsideSheetContentPolicy::Ask;
        // No resolved document: nothing crosses a sheet that does not exist.
        assert_eq!(
            section_detail(&draft, HardcopySection::DrawingSheet),
            "authored · undecided"
        );
        draft.schematic_extent = SchematicHardcopyExtent::CompleteSchematicContent;
        assert_eq!(
            section_detail(&draft, HardcopySection::DrawingSheet),
            "complete content"
        );
    }

    /// A raster target is meaningless without its resolution, and the rail is
    /// where it is read while another section is open.
    #[test]
    fn the_output_summary_carries_the_resolution_of_a_raster_target() {
        assert_eq!(
            output_summary(OutputFormat::Png { dpi: 300 }),
            "PNG · 300 dpi"
        );
        assert_eq!(
            output_summary(OutputFormat::Tiff { dpi: 1_200 }),
            "TIFF · 1200 dpi"
        );
        assert_eq!(output_summary(OutputFormat::PdfA), "PDF/A · vector");
    }

    /// The field must refuse what the publisher would refuse, and for the
    /// same reason. A ceiling of `None` means the plan has not compiled yet,
    /// so only the contract's own range is known.
    #[test]
    fn a_resolution_outside_the_offered_range_is_refused() {
        assert_eq!(parse_raster_dpi("300", Some(600)), Ok(300));
        assert_eq!(parse_raster_dpi("  600 ", Some(600)), Ok(600));
        assert!(parse_raster_dpi("601", Some(600)).is_err());
        assert!(parse_raster_dpi("71", Some(600)).is_err());
        assert!(parse_raster_dpi("", Some(600)).is_err());
        assert!(parse_raster_dpi("600dpi", Some(600)).is_err());
        assert!(parse_raster_dpi("-300", Some(600)).is_err());
        // Without a compiled plan the contract is the only authority.
        assert_eq!(parse_raster_dpi("9600", None), Ok(MAX_RASTER_DPI));
        assert!(parse_raster_dpi("9601", None).is_err());
    }

    #[test]
    fn preview_texture_cache_is_bounded_to_two_rotating_slots() {
        assert_eq!(preview_texture_slot_id(0), preview_texture_slot_id(0));
        assert_eq!(preview_texture_slot_id(1), preview_texture_slot_id(8));
        assert_ne!(preview_texture_slot_id(0), preview_texture_slot_id(1));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn rapid_preview_invalidation_never_launches_over_an_active_worker() {
        assert_eq!(
            preview_schedule_decision(None, 10),
            PreviewScheduleDecision::Launch
        );
        assert_eq!(
            preview_schedule_decision(Some(10), 10),
            PreviewScheduleDecision::Wait
        );
        for generation in 11..1_000 {
            assert_eq!(
                preview_schedule_decision(Some(10), generation),
                PreviewScheduleDecision::CancelAndWait
            );
        }
        assert_eq!(
            preview_schedule_decision(None, 999),
            PreviewScheduleDecision::Launch
        );
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn closing_preview_cancels_and_clears_worker_before_reopen() {
        let (_sender, receiver) = std::sync::mpsc::sync_channel(1);
        let cancelled = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        PREVIEW_WORKER.with(|runtime| {
            runtime.borrow_mut().active = Some(ActivePreviewWorker {
                generation: 41,
                receiver,
                cancelled: cancelled.clone(),
            });
        });

        cancel_preview_worker();

        assert!(cancelled.load(std::sync::atomic::Ordering::Acquire));
        PREVIEW_WORKER.with(|runtime| {
            assert!(runtime.borrow().active.is_none());
        });
        assert_eq!(
            preview_schedule_decision(None, 42),
            PreviewScheduleDecision::Launch
        );
    }

    #[test]
    fn document_type_choice_prefers_active_extent_over_selection() {
        use crate::hardcopy::{HardcopyDocumentKind, HardcopyScope};
        use crate::workbench::hardcopy_adapters::sources::{
            RetainedHardcopySourceAvailability, RetainedHardcopySourceDescriptor,
        };

        let candidate = RetainedHardcopySourceDescriptor {
            source_key: "project:test:schematic".to_owned(),
            display_name: "top / schematic".to_owned(),
            document_kind: HardcopyDocumentKind::SchematicOrSymbol,
            allowed_scopes: vec![
                HardcopyScope::Selection,
                HardcopyScope::CurrentSheet,
                HardcopyScope::AllSheetsOrPanes,
            ],
            availability: RetainedHardcopySourceAvailability::Available,
        };
        let candidates = vec![candidate];
        let choice = source_choice_for_active_extent(&candidates, candidates.first()).unwrap();
        assert_eq!(choice.0, "project:test:schematic");
        assert_eq!(choice.1, HardcopyScope::CurrentSheet);
    }
}

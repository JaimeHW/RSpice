//! Runtime state and exact mockup-owned Page Setup / Print / Export workflow.
//!
//! Draft controls live here; only a validated [`HardcopySetup`] is committed
//! to the project. Preview and publication are resolved from the same sealed
//! plan so the dialog cannot preview one layout and emit another.

use crate::hardcopy::{
    ActiveHardcopySource, BackgroundMode, Bleed, ColorMapping, ContentExtent, CustomPaper,
    DecorationSetup, FontPolicy, HardcopyError, HardcopyPlan, HardcopyReceipt, HardcopySetup,
    Length, LengthUnit, Orientation, OutputFormat, PageMargins, PaperSize, PhysicalPageSetup,
    PrintMappingSaveScope, PrintMappingTable, PrinterJobSettings, RenderSetup, RenderTarget,
    ScaleMode, StandardPaper, TilingMode, TilingSetup, Watermark,
};
#[cfg(test)]
use crate::hardcopy::{DuplexMode, PrinterMediaSource, PrinterRasterGeometry};
use crate::workbench::hardcopy_sources::ResolvedHardcopyDocument;

#[cfg(not(target_arch = "wasm32"))]
mod execution;
#[cfg(not(target_arch = "wasm32"))]
mod finalize;
mod publish;
mod render;
#[cfg(any(test, target_arch = "wasm32"))]
mod worker;
pub(crate) use publish::open_hardcopy_workflow;
#[cfg(target_arch = "wasm32")]
pub(crate) use worker::run_worker_request_value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HardcopyDialogPage {
    #[default]
    Main,
    PrinterProperties,
    CustomPaperMargins,
    PrintMapping,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum HardcopyDialogError {
    #[error(transparent)]
    Contract(#[from] HardcopyError),
    #[error("select a printer and resolve its current capabilities before printing")]
    #[cfg(not(target_arch = "wasm32"))]
    MissingPrinterCapabilities,
    #[error("{field} must be an unsigned integer from 1 through 65535")]
    InvalidUnsignedInteger { field: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HardcopyWorkflow {
    PageSetup,
    Print,
    Export,
}

impl HardcopyWorkflow {
    pub(crate) const fn title(self) -> &'static str {
        match self {
            Self::PageSetup => "Page setup",
            Self::Print => "Print and hardcopy",
            Self::Export => "Export active view",
        }
    }

    pub(crate) const fn primary_label(self) -> &'static str {
        match self {
            Self::PageSetup => "Save page setup",
            Self::Print => "Print",
            Self::Export => "Export hardcopy",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PaperDraft {
    Standard(StandardPaper),
    Custom {
        name: String,
        width: String,
        height: String,
    },
}

impl PaperDraft {
    fn from_paper(paper: &PaperSize, unit: LengthUnit) -> Self {
        match paper {
            PaperSize::Standard(paper) => Self::Standard(*paper),
            PaperSize::Custom(custom) => {
                let (width, height) = custom.dimensions();
                Self::Custom {
                    name: custom.name().to_owned(),
                    width: format_length(width, unit),
                    height: format_length(height, unit),
                }
            }
        }
    }

    fn build(&self, unit: LengthUnit) -> Result<PaperSize, HardcopyError> {
        match self {
            Self::Standard(paper) => Ok(PaperSize::Standard(*paper)),
            Self::Custom {
                name,
                width,
                height,
            } => Ok(PaperSize::Custom(CustomPaper::try_new(
                name.clone(),
                Length::parse_decimal(width, unit)?,
                Length::parse_decimal(height, unit)?,
                unit,
            )?)),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) enum HardcopySubflowSnapshot {
    Printer {
        printer_id: String,
        printer_job: Option<PrinterJobSettings>,
        printer_capabilities: Option<crate::workbench::hardcopy_print::PrinterCapabilitySnapshot>,
        paper: PaperDraft,
    },
    CustomPaper {
        paper: PaperDraft,
        display_unit: LengthUnit,
        orientation: Orientation,
        margin_top: String,
        margin_right: String,
        margin_bottom: String,
        margin_left: String,
        bleed: String,
        overlap: String,
        printer_id: String,
        printer_job: Option<PrinterJobSettings>,
        printer_capabilities: Option<crate::workbench::hardcopy_print::PrinterCapabilitySnapshot>,
    },
    PrintMapping {
        print_mapping: PrintMappingTable,
        mapping_scope_draft: PrintMappingSaveScope,
        mapping_name_draft: String,
    },
}

/// Runtime-only dialog draft. Source identity and content revision are sealed
/// when the workflow opens; publication re-resolves them before execution and
/// fails closed if the active document changed underneath the preview.
#[derive(Debug)]
pub(crate) struct HardcopyDialogState {
    pub(crate) open: bool,
    pub(crate) page: HardcopyDialogPage,
    pub(crate) subflow_snapshot: Option<HardcopySubflowSnapshot>,
    pub(crate) workflow: HardcopyWorkflow,
    pub(crate) source: Option<ActiveHardcopySource>,
    pub(crate) content_extent: Option<ContentExtent>,
    pub(crate) resolved_document: Option<std::sync::Arc<ResolvedHardcopyDocument>>,
    pub(crate) source_candidates:
        Vec<crate::workbench::hardcopy_sources::RetainedHardcopySourceDescriptor>,
    pub(crate) paper: PaperDraft,
    pub(crate) display_unit: LengthUnit,
    pub(crate) orientation: Orientation,
    pub(crate) margin_top: String,
    pub(crate) margin_right: String,
    pub(crate) margin_bottom: String,
    pub(crate) margin_left: String,
    pub(crate) bleed: String,
    pub(crate) scale: ScaleMode,
    pub(crate) custom_scale_percent: String,
    pub(crate) tiling: TilingMode,
    pub(crate) manual_columns: String,
    pub(crate) manual_rows: String,
    pub(crate) overlap: String,
    pub(crate) registration_marks: bool,
    pub(crate) printer_id: String,
    pub(crate) printer_job: Option<PrinterJobSettings>,
    pub(crate) printer_report: Option<crate::workbench::hardcopy_print::PrinterDiscoveryReport>,
    pub(crate) printer_capabilities:
        Option<crate::workbench::hardcopy_print::PrinterCapabilitySnapshot>,
    pub(crate) printer_discovery_busy: bool,
    pub(crate) format: OutputFormat,
    pub(crate) color_mapping: ColorMapping,
    pub(crate) background: BackgroundMode,
    pub(crate) embed_fonts: bool,
    pub(crate) searchable_text: bool,
    pub(crate) soft_proof: bool,
    pub(crate) include_legends: bool,
    pub(crate) include_header: bool,
    pub(crate) include_provenance: bool,
    pub(crate) watermark: Watermark,
    pub(crate) print_mapping: PrintMappingTable,
    pub(crate) mapping_scope_draft: PrintMappingSaveScope,
    pub(crate) mapping_name_draft: String,
    pub(crate) preview_page: u32,
    pub(crate) preview_zoom_percent: u16,
    pub(crate) preview_generation: u64,
    pub(crate) preview_failed_generation: Option<u64>,
    pub(crate) source_resolution_generation: u64,
    pub(crate) preview_plan: Option<std::sync::Arc<HardcopyPlan>>,
    pub(crate) preview:
        Option<std::sync::Arc<crate::workbench::hardcopy_render::HardcopyPreviewPage>>,
    pub(crate) preview_adjacent:
        Option<std::sync::Arc<crate::workbench::hardcopy_render::HardcopyPreviewPage>>,
    pub(crate) metadata: Option<crate::workbench::hardcopy_render::HardcopySceneMetadata>,
    pub(crate) body_scroll_offset: f32,
    pub(crate) busy: bool,
    pub(crate) cancellation: crate::workbench::hardcopy_print::HardcopyCancellationToken,
    pub(crate) last_receipt: Option<HardcopyReceipt>,
    pub(crate) error: Option<String>,
}

impl Default for HardcopyDialogState {
    fn default() -> Self {
        Self::from_setup(HardcopySetup::default())
    }
}

/// Project transactions clone [`crate::workbench::app::AppState`]. A modal
/// hardcopy session is runtime authority, not project data, so it must never
/// cross that transaction boundary or duplicate retained source/preview
/// payloads.
impl Clone for HardcopyDialogState {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl HardcopyDialogState {
    pub(crate) fn begin_open(
        &mut self,
        workflow: HardcopyWorkflow,
        source_candidates: Vec<
            crate::workbench::hardcopy_sources::RetainedHardcopySourceDescriptor,
        >,
    ) {
        let mut draft = Self::default();
        draft.open = true;
        draft.workflow = workflow;
        draft.source_candidates = source_candidates;
        draft.busy = true;
        *self = draft;
    }

    fn from_setup(setup: HardcopySetup) -> Self {
        let display_unit = LengthUnit::Inches;
        let physical = setup.physical_page();
        let margins = physical.margins();
        let tiling = setup.tiling();
        let render = setup.render();
        let decorations = setup.decorations();
        let custom_scale_percent = match setup.scale() {
            ScaleMode::CustomPercent { hundredths_percent } => format_percent(hundredths_percent),
            _ => "100".to_owned(),
        };
        let (printer_id, printer_job, format) = match render.target() {
            RenderTarget::SystemPrinter { printer_id, job } => (
                printer_id.clone(),
                Some(job.clone()),
                OutputFormat::NativePrinter,
            ),
            RenderTarget::ExportArtifact => (String::new(), None, render.format()),
            RenderTarget::BrowserPrintDialog => {
                (String::new(), None, OutputFormat::BrowserPrintDocument)
            }
        };
        Self {
            open: false,
            page: HardcopyDialogPage::Main,
            subflow_snapshot: None,
            workflow: HardcopyWorkflow::PageSetup,
            source: None,
            content_extent: None,
            resolved_document: None,
            source_candidates: Vec::new(),
            paper: PaperDraft::from_paper(physical.paper(), display_unit),
            display_unit,
            orientation: physical.orientation(),
            margin_top: format_length(margins.top, display_unit),
            margin_right: format_length(margins.right, display_unit),
            margin_bottom: format_length(margins.bottom, display_unit),
            margin_left: format_length(margins.left, display_unit),
            bleed: match physical.bleed() {
                Bleed::None => "0".to_owned(),
                Bleed::Uniform(value) => format_length(value, display_unit),
            },
            scale: setup.scale(),
            custom_scale_percent,
            tiling: tiling.mode(),
            manual_columns: match tiling.mode() {
                TilingMode::Manual { columns, .. } => columns.to_string(),
                _ => "2".to_owned(),
            },
            manual_rows: match tiling.mode() {
                TilingMode::Manual { rows, .. } => rows.to_string(),
                _ => "1".to_owned(),
            },
            overlap: format_length(tiling.overlap(), display_unit),
            registration_marks: tiling.registration_marks_and_coordinates(),
            printer_id,
            printer_job,
            printer_report: None,
            printer_capabilities: None,
            printer_discovery_busy: false,
            format,
            color_mapping: render.color_mapping(),
            background: render.background(),
            embed_fonts: render.fonts().embed_fonts(),
            searchable_text: render.fonts().preserve_searchable_text(),
            soft_proof: render.soft_proof_print_safe_colors(),
            include_legends: decorations.includes_legends(),
            include_header: decorations.includes_header(),
            include_provenance: decorations.includes_provenance(),
            watermark: decorations.watermark().clone(),
            print_mapping: setup.print_mapping().clone(),
            mapping_scope_draft: setup.print_mapping().save_scope().clone(),
            mapping_name_draft: match setup.print_mapping().save_scope() {
                PrintMappingSaveScope::Document => String::new(),
                PrintMappingSaveScope::ProjectPrintSet(name)
                | PrintMappingSaveScope::PortablePersonalPreset(name) => name.clone(),
            },
            preview_page: 0,
            preview_zoom_percent: 85,
            preview_generation: 1,
            preview_failed_generation: None,
            source_resolution_generation: 0,
            preview_plan: None,
            preview: None,
            preview_adjacent: None,
            metadata: None,
            body_scroll_offset: 0.0,
            busy: false,
            cancellation: crate::workbench::hardcopy_print::HardcopyCancellationToken::default(),
            last_receipt: None,
            error: None,
        }
    }

    pub(crate) fn open(
        &mut self,
        workflow: HardcopyWorkflow,
        source: ActiveHardcopySource,
        extent: ContentExtent,
        saved: Option<&HardcopySetup>,
    ) {
        let mut draft = saved.cloned().map_or_else(Self::default, Self::from_setup);
        draft.open = true;
        draft.page = HardcopyDialogPage::Main;
        draft.subflow_snapshot = None;
        draft.workflow = workflow;
        draft.source = Some(source);
        draft.content_extent = Some(extent);
        draft.resolved_document = None;
        draft.source_candidates.clear();
        draft.preview = None;
        draft.preview_adjacent = None;
        draft.metadata = None;
        draft.body_scroll_offset = 0.0;
        draft.busy = false;
        draft.cancellation = crate::workbench::hardcopy_print::HardcopyCancellationToken::default();
        if workflow == HardcopyWorkflow::Print {
            draft.format = runtime_print_format();
        } else if workflow == HardcopyWorkflow::Export
            && matches!(
                draft.format,
                OutputFormat::NativePrinter | OutputFormat::BrowserPrintDocument
            )
        {
            draft.format = OutputFormat::PdfVector;
        }
        #[cfg(all(not(target_arch = "wasm32"), not(target_os = "windows")))]
        if draft.format == OutputFormat::NativePrinter {
            draft.format = OutputFormat::PdfVector;
        }
        draft.refresh_preview();
        *self = draft;
    }

    pub(crate) fn open_resolved(
        &mut self,
        workflow: HardcopyWorkflow,
        document: ResolvedHardcopyDocument,
        saved: Option<&HardcopySetup>,
    ) -> Result<(), HardcopyError> {
        let authority = document.authority().clone();
        let extent = document.content_extent();
        let mapping = saved.map_or_else(
            || Ok(document.default_print_mapping().clone()),
            |setup| merge_print_mapping(document.default_print_mapping(), setup.print_mapping()),
        )?;
        self.open(workflow, authority, extent, saved);
        self.resolved_document = Some(std::sync::Arc::new(document));
        self.print_mapping = mapping;
        self.refresh_preview();
        Ok(())
    }

    pub(crate) fn close(&mut self) {
        self.open = false;
        self.page = HardcopyDialogPage::Main;
        self.subflow_snapshot = None;
        self.preview_plan = None;
        self.preview = None;
        self.preview_adjacent = None;
        self.source = None;
        self.content_extent = None;
        self.resolved_document = None;
        self.source_candidates.clear();
        self.error = None;
        self.busy = false;
        self.printer_discovery_busy = false;
        self.cancellation.cancel();
    }

    /// Atomically bind a driver capability snapshot to its printer identity.
    /// Callers must never update the visible printer name independently from
    /// the authenticated job settings.
    pub(crate) fn set_printer(&mut self, printer_id: String, job: PrinterJobSettings) {
        self.printer_id = printer_id;
        self.printer_job = Some(job);
        self.refresh_preview();
    }

    pub(crate) fn clear_printer(&mut self) {
        self.printer_id.clear();
        self.printer_job = None;
        self.refresh_preview();
    }

    pub(crate) fn open_subflow(&mut self, page: HardcopyDialogPage) {
        self.subflow_snapshot = Some(match page {
            HardcopyDialogPage::PrinterProperties => HardcopySubflowSnapshot::Printer {
                printer_id: self.printer_id.clone(),
                printer_job: self.printer_job.clone(),
                printer_capabilities: self.printer_capabilities.clone(),
                paper: self.paper.clone(),
            },
            HardcopyDialogPage::CustomPaperMargins => HardcopySubflowSnapshot::CustomPaper {
                paper: self.paper.clone(),
                display_unit: self.display_unit,
                orientation: self.orientation,
                margin_top: self.margin_top.clone(),
                margin_right: self.margin_right.clone(),
                margin_bottom: self.margin_bottom.clone(),
                margin_left: self.margin_left.clone(),
                bleed: self.bleed.clone(),
                overlap: self.overlap.clone(),
                printer_id: self.printer_id.clone(),
                printer_job: self.printer_job.clone(),
                printer_capabilities: self.printer_capabilities.clone(),
            },
            HardcopyDialogPage::PrintMapping => HardcopySubflowSnapshot::PrintMapping {
                print_mapping: self.print_mapping.clone(),
                mapping_scope_draft: self.mapping_scope_draft.clone(),
                mapping_name_draft: self.mapping_name_draft.clone(),
            },
            HardcopyDialogPage::Main => {
                self.page = HardcopyDialogPage::Main;
                self.subflow_snapshot = None;
                return;
            }
        });
        self.page = page;
        if page == HardcopyDialogPage::PrintMapping {
            self.mapping_scope_draft = self.print_mapping.save_scope().clone();
            self.mapping_name_draft = match self.print_mapping.save_scope() {
                PrintMappingSaveScope::Document => String::new(),
                PrintMappingSaveScope::ProjectPrintSet(name)
                | PrintMappingSaveScope::PortablePersonalPreset(name) => name.clone(),
            };
        }
        self.body_scroll_offset = 0.0;
        self.error = None;
    }

    pub(crate) fn accept_subflow(&mut self) -> Result<(), HardcopyDialogError> {
        let mapping = if self.page == HardcopyDialogPage::PrintMapping {
            let scope = match &self.mapping_scope_draft {
                PrintMappingSaveScope::Document => PrintMappingSaveScope::Document,
                PrintMappingSaveScope::ProjectPrintSet(_) => {
                    PrintMappingSaveScope::ProjectPrintSet(self.mapping_name_draft.clone())
                }
                PrintMappingSaveScope::PortablePersonalPreset(_) => {
                    PrintMappingSaveScope::PortablePersonalPreset(self.mapping_name_draft.clone())
                }
            };
            Some(PrintMappingTable::try_new(
                scope,
                self.print_mapping.entries().to_vec(),
            )?)
        } else {
            None
        };
        self.build_setup_with_print_mapping(mapping.as_ref().unwrap_or(&self.print_mapping))?;
        if let Some(mapping) = mapping {
            self.print_mapping = mapping;
        }
        self.page = HardcopyDialogPage::Main;
        self.subflow_snapshot = None;
        self.error = None;
        self.refresh_preview();
        Ok(())
    }

    pub(crate) fn cancel_subflow(&mut self) {
        if let Some(snapshot) = self.subflow_snapshot.take() {
            match snapshot {
                HardcopySubflowSnapshot::Printer {
                    printer_id,
                    printer_job,
                    printer_capabilities,
                    paper,
                } => {
                    self.printer_id = printer_id;
                    self.printer_job = printer_job;
                    self.printer_capabilities = printer_capabilities;
                    self.paper = paper;
                }
                HardcopySubflowSnapshot::CustomPaper {
                    paper,
                    display_unit,
                    orientation,
                    margin_top,
                    margin_right,
                    margin_bottom,
                    margin_left,
                    bleed,
                    overlap,
                    printer_id,
                    printer_job,
                    printer_capabilities,
                } => {
                    self.paper = paper;
                    self.display_unit = display_unit;
                    self.orientation = orientation;
                    self.margin_top = margin_top;
                    self.margin_right = margin_right;
                    self.margin_bottom = margin_bottom;
                    self.margin_left = margin_left;
                    self.bleed = bleed;
                    self.overlap = overlap;
                    self.printer_id = printer_id;
                    self.printer_job = printer_job;
                    self.printer_capabilities = printer_capabilities;
                }
                HardcopySubflowSnapshot::PrintMapping {
                    print_mapping,
                    mapping_scope_draft,
                    mapping_name_draft,
                } => {
                    self.print_mapping = print_mapping;
                    self.mapping_scope_draft = mapping_scope_draft;
                    self.mapping_name_draft = mapping_name_draft;
                }
            }
        }
        self.page = HardcopyDialogPage::Main;
        self.error = None;
        self.body_scroll_offset = 0.0;
        self.refresh_preview();
    }

    /// Change display units without changing any physical value. The update
    /// is transactional so a partially edited field cannot cause the other
    /// controls to jump or silently reinterpret their contents.
    pub(crate) fn set_display_unit(&mut self, unit: LengthUnit) -> Result<(), HardcopyError> {
        if unit == self.display_unit {
            return Ok(());
        }
        let current = self.display_unit;
        let top = Length::parse_decimal(&self.margin_top, current)?;
        let right = Length::parse_decimal(&self.margin_right, current)?;
        let bottom = Length::parse_decimal(&self.margin_bottom, current)?;
        let left = Length::parse_decimal(&self.margin_left, current)?;
        let bleed = Length::parse_decimal(&self.bleed, current)?;
        let overlap = Length::parse_decimal(&self.overlap, current)?;
        let custom_dimensions = match &self.paper {
            PaperDraft::Custom { width, height, .. } => Some((
                Length::parse_decimal(width, current)?,
                Length::parse_decimal(height, current)?,
            )),
            PaperDraft::Standard(_) => None,
        };

        self.display_unit = unit;
        self.margin_top = format_length(top, unit);
        self.margin_right = format_length(right, unit);
        self.margin_bottom = format_length(bottom, unit);
        self.margin_left = format_length(left, unit);
        self.bleed = format_length(bleed, unit);
        self.overlap = format_length(overlap, unit);
        if let (PaperDraft::Custom { width, height, .. }, Some((physical_width, physical_height))) =
            (&mut self.paper, custom_dimensions)
        {
            *width = format_length(physical_width, unit);
            *height = format_length(physical_height, unit);
        }
        self.refresh_preview();
        Ok(())
    }

    pub(crate) fn build_setup(&self) -> Result<HardcopySetup, HardcopyDialogError> {
        self.build_setup_with_print_mapping(&self.print_mapping)
    }

    fn build_setup_with_print_mapping(
        &self,
        print_mapping: &PrintMappingTable,
    ) -> Result<HardcopySetup, HardcopyDialogError> {
        let parse = |value: &str| Length::parse_decimal(value, self.display_unit);
        let bleed = match parse(&self.bleed)? {
            Length::ZERO => Bleed::None,
            value => Bleed::Uniform(value),
        };
        let scale = if matches!(self.scale, ScaleMode::CustomPercent { .. }) {
            let hundredths_percent = parse_percent_hundredths(&self.custom_scale_percent)?;
            ScaleMode::CustomPercent { hundredths_percent }
        } else {
            self.scale
        };
        let tiling_mode = if matches!(self.tiling, TilingMode::Manual { .. }) {
            TilingMode::Manual {
                columns: parse_u16(&self.manual_columns, "manual columns")?,
                rows: parse_u16(&self.manual_rows, "manual rows")?,
            }
        } else {
            self.tiling
        };
        let is_platform_print = self.format == runtime_print_format();
        let target = if is_platform_print {
            runtime_print_target(&self.printer_id, self.printer_job.as_ref())?
        } else {
            RenderTarget::ExportArtifact
        };
        Ok(HardcopySetup::try_new(
            PhysicalPageSetup::try_new(
                self.paper.build(self.display_unit)?,
                PageMargins {
                    top: parse(&self.margin_top)?,
                    right: parse(&self.margin_right)?,
                    bottom: parse(&self.margin_bottom)?,
                    left: parse(&self.margin_left)?,
                },
                bleed,
                self.orientation,
            )?,
            scale,
            TilingSetup::try_new(tiling_mode, parse(&self.overlap)?, self.registration_marks)?,
            RenderSetup::try_new(
                target,
                self.format,
                self.color_mapping,
                self.background,
                FontPolicy::new(self.embed_fonts, self.searchable_text),
                self.soft_proof,
            )?,
            DecorationSetup::try_new(
                self.include_legends,
                self.include_header,
                self.include_provenance,
                self.watermark.clone(),
            )?,
            print_mapping.clone(),
        )?)
    }

    pub(crate) fn refresh_preview(&mut self) {
        self.invalidate_preview_raster();
        let sections = self.resolved_document.as_ref().map_or_else(
            || Ok(Vec::new()),
            |document| {
                document
                    .hardcopy_sections()
                    .map_err(|error| error.to_string())
            },
        );
        let plan = self
            .source
            .clone()
            .zip(self.content_extent)
            .ok_or_else(|| "The active document has no hardcopy adapter.".to_owned())
            .and_then(|(source, extent)| {
                self.build_setup()
                    .map_err(|error| error.to_string())
                    .and_then(|setup| match sections {
                        Ok(sections) if !sections.is_empty() => {
                            HardcopyPlan::compile_with_sections(source, setup, extent, sections)
                                .map_err(|error| error.to_string())
                        }
                        Ok(_) => HardcopyPlan::compile(source, setup, extent)
                            .map_err(|error| error.to_string()),
                        Err(error) => Err(error),
                    })
            });
        match plan {
            Ok(plan) => {
                self.preview_page = self
                    .preview_page
                    .min((plan.pagination().pages().len() as u32).saturating_sub(1));
                self.preview_plan = Some(std::sync::Arc::new(plan));
                self.error = None;
            }
            Err(error) => {
                self.preview_plan = None;
                self.error = Some(error);
            }
        }
    }

    pub(crate) fn invalidate_preview_raster(&mut self) {
        self.preview_generation = self.preview_generation.saturating_add(1);
        self.preview_failed_generation = None;
        self.preview = None;
        self.preview_adjacent = None;
    }

    pub(crate) fn next_source_resolution_generation(&mut self) -> u64 {
        self.source_resolution_generation = self.source_resolution_generation.saturating_add(1);
        self.source_resolution_generation
    }
}

fn merge_print_mapping(
    current: &PrintMappingTable,
    saved: &PrintMappingTable,
) -> Result<PrintMappingTable, HardcopyError> {
    let saved_by_identity = saved
        .entries()
        .iter()
        .map(|entry| {
            (
                (entry.object().kind(), entry.object().stable_id().to_owned()),
                entry.clone(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let entries = current
        .entries()
        .iter()
        .map(|entry| {
            saved_by_identity
                .get(&(entry.object().kind(), entry.object().stable_id().to_owned()))
                .cloned()
                .unwrap_or_else(|| entry.clone())
        })
        .collect();
    PrintMappingTable::try_new(saved.save_scope().clone(), entries)
}

#[cfg(not(target_arch = "wasm32"))]
fn runtime_print_target(
    printer_id: &str,
    job: Option<&PrinterJobSettings>,
) -> Result<RenderTarget, HardcopyDialogError> {
    Ok(RenderTarget::SystemPrinter {
        printer_id: printer_id.trim().to_owned(),
        job: job
            .cloned()
            .ok_or(HardcopyDialogError::MissingPrinterCapabilities)?,
    })
}

#[cfg(target_arch = "wasm32")]
fn runtime_print_target(
    _printer_id: &str,
    _job: Option<&PrinterJobSettings>,
) -> Result<RenderTarget, HardcopyDialogError> {
    Ok(RenderTarget::BrowserPrintDialog)
}

#[cfg(not(target_arch = "wasm32"))]
const fn runtime_print_format() -> OutputFormat {
    OutputFormat::NativePrinter
}

#[cfg(target_arch = "wasm32")]
const fn runtime_print_format() -> OutputFormat {
    OutputFormat::BrowserPrintDocument
}

fn parse_u16(value: &str, field: &'static str) -> Result<u16, HardcopyDialogError> {
    value
        .trim()
        .parse::<u16>()
        .ok()
        .filter(|value| *value > 0)
        .ok_or(HardcopyDialogError::InvalidUnsignedInteger { field })
}

fn parse_percent_hundredths(value: &str) -> Result<u32, HardcopyError> {
    let value = value.trim();
    let mut parts = value.split('.');
    let whole = parts.next().unwrap_or_default();
    let fraction = parts.next().unwrap_or_default();
    if value.is_empty()
        || parts.next().is_some()
        || !whole.bytes().all(|byte| byte.is_ascii_digit())
        || !fraction.bytes().all(|byte| byte.is_ascii_digit())
        || fraction.len() > 9
    {
        return Err(HardcopyError::InvalidLength(format!(
            "custom scale percentage {value:?}"
        )));
    }
    let denominator = 10_u128.pow(fraction.len() as u32);
    let whole = whole
        .parse::<u128>()
        .map_err(|_| HardcopyError::InvalidScale(u32::MAX))?;
    let fraction = if fraction.is_empty() {
        0
    } else {
        fraction
            .parse::<u128>()
            .map_err(|_| HardcopyError::InvalidScale(u32::MAX))?
    };
    let scaled = whole
        .checked_mul(denominator)
        .and_then(|number| number.checked_add(fraction))
        .and_then(|number| number.checked_mul(100))
        .and_then(|number| number.checked_add(denominator / 2))
        .ok_or(HardcopyError::InvalidScale(u32::MAX))?
        / denominator;
    u32::try_from(scaled).map_err(|_| HardcopyError::InvalidScale(u32::MAX))
}

fn format_percent(hundredths_percent: u32) -> String {
    let whole = hundredths_percent / 100;
    let fraction = hundredths_percent % 100;
    if fraction == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{fraction:02}")
            .trim_end_matches('0')
            .to_owned()
    }
}

fn format_length(value: Length, unit: LengthUnit) -> String {
    let (divisor, fractional_digits) = match unit {
        LengthUnit::Inches => (25_400_u64, 6_u32),
        LengthUnit::Millimetres => (1_000_u64, 3_u32),
    };
    let whole = value.micrometres() / divisor;
    let remainder = value.micrometres() % divisor;
    if remainder == 0 {
        return whole.to_string();
    }
    let scale = 10_u128.pow(fractional_digits);
    let fractional =
        (u128::from(remainder) * scale + u128::from(divisor / 2)) / u128::from(divisor);
    if fractional == scale {
        return (whole + 1).to_string();
    }
    format!(
        "{whole}.{fractional:0width$}",
        width = fractional_digits as usize
    )
    .trim_end_matches('0')
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardcopy::{HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope};
    use crate::product::{ContentDigest, ObjectRevision};

    fn source() -> ActiveHardcopySource {
        ActiveHardcopySource::try_new(
            HardcopyDocumentId::try_from_uuid(uuid::Uuid::from_u128(0x4852_4450_5902)).unwrap(),
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x52; 32]),
            "top / schematic",
            HardcopyDocumentKind::SchematicOrSymbol,
            HardcopyScope::CurrentSheet,
        )
        .unwrap()
    }

    #[test]
    fn default_draft_compiles_the_same_plan_used_by_preview() {
        let mut draft = HardcopyDialogState::default();
        draft.open(
            HardcopyWorkflow::Export,
            source(),
            ContentExtent::try_new(
                Length::from_micrometres(250_000),
                Length::from_micrometres(160_000),
            )
            .unwrap(),
            None,
        );

        let preview = draft.preview_plan.as_ref().expect("valid preview");
        assert_eq!(preview.setup(), &draft.build_setup().unwrap());
        assert!(draft.error.is_none());
    }

    #[test]
    fn invalid_geometry_removes_preview_instead_of_retaining_stale_pages() {
        let mut draft = HardcopyDialogState::default();
        draft.open(
            HardcopyWorkflow::Export,
            source(),
            ContentExtent::try_new(
                Length::from_micrometres(100_000),
                Length::from_micrometres(100_000),
            )
            .unwrap(),
            None,
        );
        assert!(draft.preview_plan.is_some());

        draft.margin_left = "100".to_owned();
        draft.refresh_preview();
        assert!(draft.preview_plan.is_none());
        assert!(draft.error.is_some());
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn print_workflow_requires_a_real_printer_identity() {
        let mut draft = HardcopyDialogState::default();
        draft.workflow = HardcopyWorkflow::Print;
        draft.format = OutputFormat::NativePrinter;
        // Switching to a raster printer through the dialog clears the vector-only
        // searchable-text contract before the setup is compiled. Mirror that
        // user-visible transition in this direct state-level fixture.
        draft.embed_fonts = false;
        draft.searchable_text = false;
        draft.printer_id.clear();
        assert!(draft.build_setup().is_err());
        draft.printer_id = "Microsoft Print to PDF".to_owned();
        draft.printer_job = Some(
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
        assert!(draft.build_setup().is_ok());
    }

    #[test]
    fn print_dialog_file_target_compiles_as_an_export_artifact() {
        let mut draft = HardcopyDialogState::default();
        draft.workflow = HardcopyWorkflow::Print;
        draft.format = OutputFormat::PdfVector;
        let setup = draft.build_setup().expect("PDF target is a file artifact");
        assert_eq!(setup.render().target(), &RenderTarget::ExportArtifact);
        assert_eq!(setup.render().format(), OutputFormat::PdfVector);
    }

    #[test]
    #[cfg(target_arch = "wasm32")]
    fn browser_print_uses_the_browser_handoff_contract() {
        let mut draft = HardcopyDialogState::default();
        draft.workflow = HardcopyWorkflow::Print;
        let setup = draft
            .build_setup()
            .expect("browser target needs no printer id");
        assert_eq!(setup.render().target(), &RenderTarget::BrowserPrintDialog);
        assert_eq!(setup.render().format(), OutputFormat::BrowserPrintDocument);
    }

    #[test]
    fn changing_display_units_preserves_exact_physical_geometry() {
        let mut draft = HardcopyDialogState::default();
        let before = draft.build_setup().unwrap();

        draft
            .set_display_unit(LengthUnit::Millimetres)
            .expect("valid draft converts");
        let after = draft.build_setup().unwrap();

        assert_eq!(before, after);
        assert_eq!(draft.margin_top, "8.89");
        assert_eq!(draft.overlap, "6.35");
    }

    #[test]
    fn cancelling_custom_page_restores_units_overlap_and_exact_setup() {
        let mut draft = HardcopyDialogState::default();
        draft.open(
            HardcopyWorkflow::Export,
            source(),
            ContentExtent::try_new(
                Length::from_micrometres(250_000),
                Length::from_micrometres(160_000),
            )
            .unwrap(),
            None,
        );
        let before = draft.build_setup().unwrap();
        let original_job = PrinterJobSettings::try_new(
            ContentDigest::from_bytes([0x51; 32]),
            "9",
            PrinterRasterGeometry::try_new(1_000, 800, 0, 0, 1_000, 800).unwrap(),
            PrinterMediaSource::AutomaticCompatibleTray,
            600,
            DuplexMode::Off,
            1,
            false,
        )
        .unwrap();
        draft.printer_id = "test-printer".to_owned();
        draft.printer_job = Some(original_job.clone());
        draft.open_subflow(HardcopyDialogPage::CustomPaperMargins);

        draft
            .set_display_unit(LengthUnit::Millimetres)
            .expect("valid unit conversion");
        draft.overlap = "12.345".to_owned();
        draft.margin_left = "23.456".to_owned();
        draft.paper = PaperDraft::Custom {
            name: "Temporary page".to_owned(),
            width: "420".to_owned(),
            height: "297".to_owned(),
        };
        draft.printer_id.clear();
        draft.printer_job = None;
        draft.cancel_subflow();

        assert_eq!(draft.page, HardcopyDialogPage::Main);
        assert_eq!(draft.display_unit, LengthUnit::Inches);
        assert_eq!(draft.overlap, "0.25");
        assert_eq!(draft.printer_id, "test-printer");
        assert_eq!(draft.printer_job, Some(original_job));
        assert_eq!(draft.build_setup().unwrap(), before);
        assert!(draft.preview_plan.is_some());
    }

    #[test]
    fn source_resolution_generations_are_strictly_monotonic() {
        let mut draft = HardcopyDialogState::default();
        assert_eq!(draft.next_source_resolution_generation(), 1);
        assert_eq!(draft.next_source_resolution_generation(), 2);
        assert_eq!(draft.next_source_resolution_generation(), 3);
    }

    #[test]
    fn length_format_round_trips_arbitrary_integer_micrometres() {
        for micrometres in (0_u64..=100_000).chain([
            215_899,
            279_401,
            431_799,
            1_234_567,
            10_000_001,
            u32::MAX as u64,
        ]) {
            let length = Length::from_micrometres(micrometres);
            for unit in [LengthUnit::Inches, LengthUnit::Millimetres] {
                let text = format_length(length, unit);
                assert_eq!(
                    Length::parse_decimal(&text, unit).unwrap(),
                    length,
                    "{micrometres} µm did not round-trip through {unit:?} as {text:?}"
                );
            }
        }
    }
}

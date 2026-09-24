//! Runtime state and exact mockup-owned Page Setup / Print / Export workflow.
//!
//! Draft controls live here; only a validated [`HardcopySetup`] is committed
//! to the project. Preview and publication are resolved from the same sealed
//! plan so the dialog cannot preview one layout and emit another.

use crate::hardcopy::{
    ActiveHardcopySource, AuthoredSheetMedia, BackgroundMode, Bleed, ColorMapping, ContentExtent,
    CustomPaper, DecorationSetup, FontPolicy, HardcopyError, HardcopyPlan, HardcopyReceipt,
    HardcopySetup, Length, LengthUnit, Orientation, OutputFormat, OutsideSheetContentPolicy,
    PageMargins, PaperSize, PhysicalPageSetup, PrintMappingSaveScope, PrintMappingTable,
    PrinterJobSettings, RenderSetup, RenderTarget, ScaleMode, SchematicHardcopyExtent,
    SchematicHardcopySetup, StandardPaper, TilingMode, TilingSetup, Watermark,
};
#[cfg(test)]
use crate::hardcopy::{DuplexMode, PrinterMediaSource, PrinterRasterGeometry};
use crate::workbench::hardcopy_adapters::render::HardcopyRenderError;
use crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument;

#[cfg(not(target_arch = "wasm32"))]
mod execution;
#[cfg(not(target_arch = "wasm32"))]
mod finalize;
mod publish;
mod render;
#[cfg(test)]
mod setting_effects;
#[cfg(any(test, target_arch = "wasm32"))]
mod worker;
/// Resolution a raster export starts at when the setup carries none. It is a
/// starting point, not a ceiling: what the host can actually afford for the
/// chosen page comes from `render::max_raster_dpi`, and on a small enough
/// budget the field opens already showing a violation rather than a lie.
pub(crate) const DEFAULT_RASTER_DPI: u16 = 600;

pub(crate) use publish::open_hardcopy_workflow;
#[cfg(all(target_arch = "wasm32", feature = "browser-worker"))]
pub(crate) use worker::run_worker_request_value;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HardcopyDialogPage {
    #[default]
    Main,
    PrinterProperties,
    CustomPaperMargins,
    PrintMapping,
}

/// Which region a surface too narrow to hold both is showing.
///
/// View state: it is a property of how this hardcopy is being looked at, never
/// of the hardcopy itself, so it stays out of the setup the workflow commits
/// and out of the comparisons that invalidate the preview.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum HardcopyRegion {
    #[default]
    Setup,
    Preview,
}

/// The one concern the main page is editing right now.
///
/// The surface shows exactly one of these at a time, which is what keeps it
/// scroll-free: the number of controls a hardcopy carries is not bounded, but
/// the number visible at once is. The rail keeps the others legible by
/// summarising each in a line, so nothing is hidden — only deferred.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub(crate) enum HardcopySection {
    #[default]
    Source,
    Page,
    DrawingSheet,
    Output,
    Identity,
}

impl HardcopySection {
    pub(crate) const ALL: [Self; 5] = [
        Self::Source,
        Self::Page,
        Self::DrawingSheet,
        Self::Output,
        Self::Identity,
    ];

    pub(crate) const fn number(self) -> &'static str {
        match self {
            Self::Source => "01",
            Self::Page => "02",
            Self::DrawingSheet => "03",
            Self::Output => "04",
            Self::Identity => "05",
        }
    }

    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Source => "Source and scope",
            Self::Page => "Page and pagination",
            Self::DrawingSheet => "Drawing sheet",
            Self::Output => "Output",
            Self::Identity => "Identity",
        }
    }
}

/// The section whose fields a refusal is about, where exactly one owns it.
///
/// Attribution is by contract variant, and by the typed field a variant names
/// where it carries one. It is never taken from the rendered message: that is
/// prose, and a reworded sentence would silently move a marker. A refusal that
/// more than one section's controls can cause — or that no control causes at
/// all — stays at the dialog level, because a marker on a section with nothing
/// wrong in it costs the operator more than no marker does.
fn contract_section(error: &HardcopyError) -> Option<HardcopySection> {
    match error {
        // Paper, orientation, margins, bleed, scale and tiling are authored in
        // one section, and every length or count this dialog parses is one of
        // them. The pagination refusals belong here too: what they name is the
        // page grid those same controls ask for.
        HardcopyError::InvalidLength(_)
        | HardcopyError::InvalidPageDimension { .. }
        | HardcopyError::MarginsExhaustPage
        | HardcopyError::BleedExceedsMargins
        | HardcopyError::InvalidScale(_)
        | HardcopyError::ZeroScaleRatio
        | HardcopyError::InvalidManualTiling { .. }
        | HardcopyError::ManualTilingDoesNotCover { .. }
        | HardcopyError::SinglePageOverflow { .. }
        | HardcopyError::OverlapExhaustsPage
        | HardcopyError::TooManyPages(_)
        | HardcopyError::InvalidAuthoredSheetMedia(_)
        | HardcopyError::AuthoredMediaRequiresSchematicSource => Some(HardcopySection::Page),
        // The target and what it is able to carry.
        HardcopyError::InvalidRasterResolution(_)
        | HardcopyError::IncompatibleRenderTarget
        | HardcopyError::TransparentBackgroundRequiresVectorExport
        | HardcopyError::SearchableTextRequiresVectorOutput
        | HardcopyError::SearchableTextRequiresEmbeddedFonts
        | HardcopyError::PdfARequiresEmbeddedFonts
        | HardcopyError::InvalidPrinterRollWidth
        | HardcopyError::InvalidPrinterResolution(_)
        | HardcopyError::InvalidPrinterRasterGeometry
        | HardcopyError::InvalidCopyCount(_)
        | HardcopyError::CollationRequiresMultipleCopies => Some(HardcopySection::Output),
        // The sealed document and the extent of it this hardcopy publishes.
        HardcopyError::IncompatibleScope { .. } | HardcopyError::EmptyContentExtent => {
            Some(HardcopySection::Source)
        }
        // One refusal, one field, several owners. An unrecognised field is
        // reported at the dialog level rather than guessed at.
        HardcopyError::InvalidText { field, .. } => match *field {
            "custom paper name" => Some(HardcopySection::Page),
            "printer identity" => Some(HardcopySection::Output),
            "custom watermark" => Some(HardcopySection::Identity),
            _ => None,
        },
        _ => None,
    }
}

/// The section a refusal raised by the renderer belongs to.
///
/// Same rule as [`contract_section`], and for the same reason: by variant, so
/// that rewording a sentence cannot move a marker. A resource limit is the
/// refusal a browser meets in ordinary use — its raster budget is a quarter of
/// the desktop's — and every budget it names is charged against the artifact
/// the target section asks for, which is where an operator goes to ask for a
/// smaller one. The rest of what the renderer can refuse is about the sealed
/// document or about the publication as a whole, and stays at the dialog level
/// rather than marking a section that cannot repair it.
fn render_section(error: &HardcopyRenderError) -> Option<HardcopySection> {
    match error {
        HardcopyRenderError::ResourceLimit { .. } => Some(HardcopySection::Output),
        HardcopyRenderError::SchematicOutsideContentDecisionRequired => {
            Some(HardcopySection::DrawingSheet)
        }
        _ => None,
    }
}

/// A refused configuration: what the operator is told, and where to go about
/// it. The two travel together so a marker cannot outlive the message it
/// belongs to.
struct BlockedSetup {
    message: String,
    section: Option<HardcopySection>,
}

impl BlockedSetup {
    /// A refusal no section owns: it is about the workflow, the host, or the
    /// retained document rather than about a field.
    fn dialog(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            section: None,
        }
    }
}

impl From<HardcopyDialogError> for BlockedSetup {
    fn from(error: HardcopyDialogError) -> Self {
        Self {
            message: error.to_string(),
            section: error.section(),
        }
    }
}

impl From<HardcopyError> for BlockedSetup {
    fn from(error: HardcopyError) -> Self {
        Self {
            message: error.to_string(),
            section: contract_section(&error),
        }
    }
}

impl From<HardcopyRenderError> for BlockedSetup {
    fn from(error: HardcopyRenderError) -> Self {
        Self {
            message: error.to_string(),
            section: render_section(&error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(crate) enum HardcopyDialogError {
    #[error(transparent)]
    Contract(#[from] HardcopyError),
    #[error("select a printer and resolve its current capabilities before printing")]
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    MissingPrinterCapabilities,
    #[error("{field} must be an unsigned integer from 1 through 65535")]
    InvalidUnsignedInteger { field: &'static str },
    #[error(
        "schematic content crosses the authored drawing sheet; choose Clip to authored drawing sheet or Extend output to include content"
    )]
    OutsideSheetContentDecisionRequired,
    #[error("could not validate schematic drawing-sheet output: {0}")]
    SchematicOutputSource(String),
}

impl HardcopyDialogError {
    fn section(&self) -> Option<HardcopySection> {
        match self {
            Self::Contract(error) => contract_section(error),
            #[cfg(not(target_arch = "wasm32"))]
            Self::MissingPrinterCapabilities => Some(HardcopySection::Output),
            // The only unsigned integers the draft parses are the manual tile
            // counts. A field added later is reported at the dialog level until
            // it is given an owner here.
            Self::InvalidUnsignedInteger { field } => match *field {
                "manual columns" | "manual rows" => Some(HardcopySection::Page),
                _ => None,
            },
            Self::OutsideSheetContentDecisionRequired => Some(HardcopySection::DrawingSheet),
            // Raised when the retained document cannot answer a question about
            // its own drawing sheet, which is a property of the source rather
            // than of anything the sheet section can be set to.
            Self::SchematicOutputSource(_) => None,
        }
    }
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
    MatchAuthoredSheets,
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
            PaperSize::MatchAuthoredSheets(_) => Self::MatchAuthoredSheets,
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

    fn build(
        &self,
        unit: LengthUnit,
        authored_media: Option<Vec<AuthoredSheetMedia>>,
    ) -> Result<PaperSize, HardcopyError> {
        match self {
            Self::Standard(paper) => Ok(PaperSize::Standard(*paper)),
            Self::MatchAuthoredSheets => authored_media.map(PaperSize::MatchAuthoredSheets).ok_or(
                HardcopyError::InvalidAuthoredSheetMedia(
                    "the selected source has no governed authored drawing sheets",
                ),
            ),
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
        printer_capabilities:
            Option<crate::workbench::hardcopy_adapters::print::PrinterCapabilitySnapshot>,
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
        printer_capabilities:
            Option<crate::workbench::hardcopy_adapters::print::PrinterCapabilitySnapshot>,
    },
    PrintMapping {
        print_mapping: PrintMappingTable,
        mapping_scope_draft: PrintMappingSaveScope,
        mapping_name_draft: String,
    },
}

#[derive(Debug, Clone)]
pub(crate) struct GovernedSheetPageAuthority {
    pub(crate) cell_view_key: String,
    pub(crate) catalog_revision: u64,
    pub(crate) sheet_id: crate::state::SheetId,
    pub(crate) sheet_revision: u64,
}

#[derive(Debug, Clone)]
pub(crate) struct SchematicPageSetupAuthority {
    pub(crate) edit: crate::workbench::app::SchematicEditAuthority,
    pub(crate) governed_sheet: Option<GovernedSheetPageAuthority>,
}

/// Runtime-only dialog draft. Source identity and content revision are sealed
/// when the workflow opens; publication re-resolves them before execution and
/// fails closed if the active document changed underneath the preview.
#[derive(Debug)]
pub(crate) struct HardcopyDialogState {
    pub(crate) open: bool,
    pub(crate) page: HardcopyDialogPage,
    pub(crate) section: HardcopySection,
    /// Which region the one-at-a-time surface is showing. Meaningless where
    /// the setup and the page are both on screen, which is why nothing else
    /// reads it.
    pub(crate) region: HardcopyRegion,
    pub(crate) subflow_snapshot: Option<HardcopySubflowSnapshot>,
    pub(crate) workflow: HardcopyWorkflow,
    pub(crate) source: Option<ActiveHardcopySource>,
    pub(crate) content_extent: Option<ContentExtent>,
    pub(crate) resolved_document: Option<std::sync::Arc<ResolvedHardcopyDocument>>,
    pub(crate) schematic_page_authority: Option<SchematicPageSetupAuthority>,
    pub(crate) source_candidates:
        Vec<crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor>,
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
    pub(crate) schematic_extent: SchematicHardcopyExtent,
    pub(crate) outside_sheet_content: OutsideSheetContentPolicy,
    pub(crate) crop_marks: bool,
    pub(crate) include_sheet_paper: bool,
    pub(crate) include_sheet_border: bool,
    pub(crate) include_sheet_title_block: bool,
    pub(crate) include_sheet_zones: bool,
    pub(crate) include_schematic_grid: bool,
    pub(crate) printer_id: String,
    pub(crate) printer_job: Option<PrinterJobSettings>,
    pub(crate) printer_report:
        Option<crate::workbench::hardcopy_adapters::print::PrinterDiscoveryReport>,
    pub(crate) printer_capabilities:
        Option<crate::workbench::hardcopy_adapters::print::PrinterCapabilitySnapshot>,
    pub(crate) printer_discovery_busy: bool,
    pub(crate) format: OutputFormat,
    /// Editable resolution for the raster formats. `format` stays the
    /// authority; this is the text being typed, committed on Enter or focus
    /// loss, because `60` is a legal prefix of `600` and refusing it mid-word
    /// would fight the typist.
    pub(crate) raster_dpi_draft: String,
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
        Option<std::sync::Arc<crate::workbench::hardcopy_adapters::render::HardcopyPreviewPage>>,
    pub(crate) preview_adjacent:
        Option<std::sync::Arc<crate::workbench::hardcopy_adapters::render::HardcopyPreviewPage>>,
    pub(crate) metadata: Option<crate::workbench::hardcopy_adapters::render::HardcopySceneMetadata>,
    pub(crate) body_scroll_offset: f32,
    pub(crate) busy: bool,
    pub(crate) cancellation: crate::workbench::hardcopy_adapters::print::HardcopyCancellationToken,
    pub(crate) last_receipt: Option<HardcopyReceipt>,
    pub(crate) error: Option<String>,
    /// The rail entry [`Self::error`] belongs to, where the contract named one.
    /// Written only where the draft is validated, so a section is never marked
    /// while its fields compile.
    pub(crate) error_section: Option<HardcopySection>,
}

impl Default for HardcopyDialogState {
    fn default() -> Self {
        Self::from_setup(HardcopySetup::default())
    }
}

/// Project transactions clone [`crate::workbench::app_state::AppState`]. A modal
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
            crate::workbench::hardcopy_adapters::sources::RetainedHardcopySourceDescriptor,
        >,
    ) {
        *self = Self {
            open: true,
            workflow,
            source_candidates,
            busy: true,
            ..Default::default()
        };
    }

    fn from_setup(setup: HardcopySetup) -> Self {
        let display_unit = LengthUnit::Inches;
        let physical = setup.physical_page();
        let margins = physical.margins();
        let tiling = setup.tiling();
        let render = setup.render();
        let decorations = setup.decorations();
        let schematic = setup.schematic();
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
            section: HardcopySection::default(),
            region: HardcopyRegion::default(),
            subflow_snapshot: None,
            workflow: HardcopyWorkflow::PageSetup,
            source: None,
            content_extent: None,
            resolved_document: None,
            schematic_page_authority: None,
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
            schematic_extent: schematic.extent(),
            outside_sheet_content: schematic.outside_content(),
            crop_marks: schematic.crop_marks(),
            include_sheet_paper: schematic.includes_paper(),
            include_sheet_border: schematic.includes_border(),
            include_sheet_title_block: schematic.includes_title_block(),
            include_sheet_zones: schematic.includes_zones(),
            include_schematic_grid: schematic.includes_grid(),
            printer_id,
            printer_job,
            printer_report: None,
            printer_capabilities: None,
            printer_discovery_busy: false,
            format,
            raster_dpi_draft: format
                .raster_dpi()
                .unwrap_or(DEFAULT_RASTER_DPI)
                .to_string(),
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
            cancellation:
                crate::workbench::hardcopy_adapters::print::HardcopyCancellationToken::default(),
            last_receipt: None,
            error: None,
            error_section: None,
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
        draft.section = HardcopySection::default();
        draft.region = HardcopyRegion::default();
        draft.subflow_snapshot = None;
        draft.workflow = workflow;
        draft.source = Some(source);
        draft.content_extent = Some(extent);
        draft.resolved_document = None;
        draft.schematic_page_authority = None;
        draft.source_candidates.clear();
        draft.preview = None;
        draft.preview_adjacent = None;
        draft.metadata = None;
        draft.body_scroll_offset = 0.0;
        draft.busy = false;
        draft.cancellation =
            crate::workbench::hardcopy_adapters::print::HardcopyCancellationToken::default();
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
        self.schematic_page_authority = None;
        self.source_candidates.clear();
        self.error = None;
        self.error_section = None;
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
        self.error_section = None;
    }

    /// The scope the print-mapping page will commit: the selected kind
    /// carrying the name currently typed. The name is not normalised, because
    /// a project catalog keys its presets by exactly this text.
    pub(crate) fn edited_mapping_scope(&self) -> PrintMappingSaveScope {
        match &self.mapping_scope_draft {
            PrintMappingSaveScope::Document => PrintMappingSaveScope::Document,
            PrintMappingSaveScope::ProjectPrintSet(_) => {
                PrintMappingSaveScope::ProjectPrintSet(self.mapping_name_draft.clone())
            }
            PrintMappingSaveScope::PortablePersonalPreset(_) => {
                PrintMappingSaveScope::PortablePersonalPreset(self.mapping_name_draft.clone())
            }
        }
    }

    /// Why the contract would refuse that scope, if it would. The field asks
    /// the contract rather than restating its rule, so what the name control
    /// reports and what the save refuses can never disagree.
    pub(crate) fn mapping_scope_refusal(&self) -> Option<String> {
        PrintMappingTable::try_new(self.edited_mapping_scope(), Vec::new())
            .err()
            .map(|error| error.to_string())
    }

    pub(crate) fn accept_subflow(&mut self) -> Result<(), HardcopyDialogError> {
        let mapping = if self.page == HardcopyDialogPage::PrintMapping {
            Some(PrintMappingTable::try_new(
                self.edited_mapping_scope(),
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
        self.error_section = None;
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
        self.error_section = None;
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
            PaperDraft::Standard(_) | PaperDraft::MatchAuthoredSheets => None,
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
        self.validate_schematic_output_contract()?;
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
        let authored_media = if matches!(self.paper, PaperDraft::MatchAuthoredSheets) {
            Some(
                self.resolved_document
                    .as_ref()
                    .ok_or(HardcopyDialogError::SchematicOutputSource(
                        "the selected source has not resolved".to_owned(),
                    ))?
                    .authored_sheet_output_media()
                    .map_err(|error| {
                        HardcopyDialogError::SchematicOutputSource(error.to_string())
                    })?,
            )
        } else {
            None
        };
        Ok(HardcopySetup::try_new_with_schematic(
            PhysicalPageSetup::try_new(
                self.paper.build(self.display_unit, authored_media)?,
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
            SchematicHardcopySetup::new(
                self.schematic_extent,
                self.outside_sheet_content,
                self.crop_marks,
                self.include_sheet_paper,
                self.include_sheet_border,
                self.include_sheet_title_block,
                self.include_sheet_zones,
                self.include_schematic_grid,
            ),
            print_mapping.clone(),
        )?)
    }

    fn validate_schematic_output_contract(&self) -> Result<(), HardcopyDialogError> {
        let Some(document) = self.resolved_document.as_ref() else {
            return Ok(());
        };
        let Some(has_outside_content) = document
            .schematic_drawing_sheet_has_outside_content()
            .map_err(|error| HardcopyDialogError::SchematicOutputSource(error.to_string()))?
        else {
            return Ok(());
        };
        if self.schematic_extent == SchematicHardcopyExtent::CompleteSchematicContent {
            return Ok(());
        }
        if has_outside_content && self.outside_sheet_content == OutsideSheetContentPolicy::Ask {
            return Err(HardcopyDialogError::OutsideSheetContentDecisionRequired);
        }
        Ok(())
    }

    pub(crate) fn refresh_preview(&mut self) {
        self.invalidate_preview_raster();
        match self.compile_preview_plan() {
            Ok(plan) => {
                self.preview_page = self
                    .preview_page
                    .min((plan.pagination().pages().len() as u32).saturating_sub(1));
                self.preview_plan = Some(std::sync::Arc::new(plan));
                self.error = None;
                self.error_section = None;
            }
            Err(blocked) => {
                self.preview_plan = None;
                self.error = Some(blocked.message);
                self.error_section = blocked.section;
            }
        }
    }

    /// The plan the preview and the publication share. Every refusal on the way
    /// keeps its type until this returns, so the studio can say which section a
    /// blocked configuration is about instead of only that it is blocked.
    fn compile_preview_plan(&self) -> Result<HardcopyPlan, BlockedSetup> {
        let (source, fallback_extent) = self
            .source
            .clone()
            .zip(self.content_extent)
            .ok_or_else(|| BlockedSetup::dialog("The active document has no hardcopy adapter."))?;
        let setup = self.build_setup()?;
        let extent = self.resolved_document.as_ref().map_or_else(
            || Ok(fallback_extent),
            |document| {
                document
                    .content_extent_for_setup(setup.schematic())
                    .map_err(|error| BlockedSetup::dialog(error.to_string()))
            },
        )?;
        let sections = self.resolved_document.as_ref().map_or_else(
            || Ok(Vec::new()),
            |document| {
                document
                    .hardcopy_sections_for_setup(setup.schematic())
                    .map_err(|error| BlockedSetup::dialog(error.to_string()))
            },
        )?;
        if sections.is_empty() {
            HardcopyPlan::compile(source, setup, extent)
        } else {
            HardcopyPlan::compile_with_sections(source, setup, extent, sections)
        }
        .map_err(BlockedSetup::from)
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

#[cfg(target_os = "windows")]
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

#[cfg(not(target_os = "windows"))]
fn runtime_print_target(
    _printer_id: &str,
    _job: Option<&PrinterJobSettings>,
) -> Result<RenderTarget, HardcopyDialogError> {
    Ok(RenderTarget::BrowserPrintDialog)
}

#[cfg(target_os = "windows")]
const fn runtime_print_format() -> OutputFormat {
    OutputFormat::NativePrinter
}

#[cfg(not(target_os = "windows"))]
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
    use crate::hardcopy::{
        HardcopyDocumentId, HardcopyDocumentKind, HardcopyScope, HardcopySetupStore,
        SetupSaveDisposition,
    };
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

    /// The printer identity this asserts is a Windows contract.
    ///
    /// `RenderTarget::SystemPrinter` is produced by the `target_os = "windows"`
    /// `runtime_print_target` alone; every other host returns
    /// `BrowserPrintDialog` and reads neither the identity nor the job. And
    /// `NativePrinter` is only that host's `runtime_print_format`, so run
    /// anywhere else this fixture never reaches the contract at all: both
    /// `build_setup` calls fail as an incompatible export target, and the
    /// refusal half passes for a reason that has nothing to do with printers.
    #[test]
    #[cfg(target_os = "windows")]
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

    /// The counterpart contract on every host without native printing.
    ///
    /// macOS, Linux and wasm32 all take the `not(target_os = "windows")`
    /// `runtime_print_target`, which hands off to the browser print dialog and
    /// reads neither the printer identity nor the job. So this asserts the
    /// inverse of the Windows fixture above: printing compiles with no printer
    /// identity at all, and demanding one would be the defect.
    ///
    /// `format` is set explicitly rather than taken from
    /// `runtime_print_format()` or left at the `HardcopySetup::default()`
    /// value. That default is `PdfVector`, which makes `is_platform_print`
    /// false and quietly routes the fixture down the `ExportArtifact` path
    /// already covered above — passing the target assertion is what proves the
    /// print branch was reached.
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn browser_print_uses_the_browser_handoff_contract() {
        let mut draft = HardcopyDialogState::default();
        draft.workflow = HardcopyWorkflow::Print;
        draft.format = OutputFormat::BrowserPrintDocument;
        // Mirror the Windows fixture: the dialog clears the vector-only
        // searchable-text contract as part of the same transition, so the
        // font policy is not what this fixture is testing.
        draft.embed_fonts = false;
        draft.searchable_text = false;
        // No printer was ever selected, and none is required.
        draft.printer_id.clear();
        draft.printer_job = None;
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

    /// A refused field is invisible from any other section, so the refusal has
    /// to carry the section it belongs to — and it has to come from the
    /// contract's own answer rather than from the sentence it renders.
    #[test]
    fn a_refused_field_names_the_section_that_owns_it() {
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
        assert_eq!(draft.error_section, None);

        draft.margin_left = "100".to_owned();
        draft.refresh_preview();
        assert_eq!(draft.error_section, Some(HardcopySection::Page));

        draft.margin_left = "0.35".to_owned();
        draft.manual_columns = "0".to_owned();
        draft.tiling = TilingMode::Manual {
            columns: 2,
            rows: 1,
        };
        draft.refresh_preview();
        assert_eq!(draft.error_section, Some(HardcopySection::Page));

        draft.tiling = TilingMode::Automatic;
        draft.format = OutputFormat::Png { dpi: 300 };
        draft.searchable_text = true;
        draft.refresh_preview();
        assert_eq!(draft.error_section, Some(HardcopySection::Output));

        draft.searchable_text = false;
        draft.watermark = Watermark::Custom(" untrimmed ".to_owned());
        draft.refresh_preview();
        assert_eq!(draft.error_section, Some(HardcopySection::Identity));

        draft.watermark = Watermark::None;
        draft.refresh_preview();
        assert!(draft.error.is_none());
        assert_eq!(draft.error_section, None);
    }

    /// The refusal a browser meets first is the renderer's, not the contract's:
    /// its raster working set is a quarter of the desktop's, so a page the
    /// desktop publishes at 600 dpi is refused there. It arrives after a render
    /// round trip rather than from the draft, and it has to reach the rail
    /// carrying the section whose controls set the budget it names.
    #[test]
    fn a_renderer_resource_limit_is_attributed_to_the_target_that_set_it() {
        let refused = HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            maximum: 268_435_456,
        };
        assert_eq!(render_section(&refused), Some(HardcopySection::Output));

        let message = refused.to_string();
        let blocked = BlockedSetup::from(refused);
        assert_eq!(blocked.section, Some(HardcopySection::Output));
        assert_eq!(blocked.message, message);

        // The variant decides the section. The scope is a word in the sentence,
        // not a second key into the table, so no wording of it moves the marker.
        for scope in ["scene primitives", "render work units", "pages"] {
            assert_eq!(
                render_section(&HardcopyRenderError::ResourceLimit {
                    scope,
                    maximum: 1_024,
                }),
                Some(HardcopySection::Output),
                "{scope}"
            );
        }
        assert_eq!(
            render_section(&HardcopyRenderError::SchematicOutsideContentDecisionRequired),
            Some(HardcopySection::DrawingSheet)
        );
        // Nothing any section can be set to repairs a source that will not
        // convert, so it stays at the dialog level.
        assert_eq!(
            render_section(&HardcopyRenderError::SourceConversion(
                "unreadable".to_owned()
            )),
            None
        );
    }

    /// Attribution is a whitelist. A refusal more than one section's controls
    /// can cause, or that no control causes at all, has to stay at the dialog
    /// level: a marker on a section with nothing wrong in it sends the operator
    /// to a form that cannot help them.
    #[test]
    fn a_refusal_no_single_section_owns_stays_at_the_dialog_level() {
        for error in [
            // Page geometry and the Identity bands both decide this one.
            HardcopyError::NoPrintableContentArea,
            HardcopyError::GeometryUnderflow("header, legend, and provenance bands"),
            // The print mapping is edited on a page of its own, not a section.
            HardcopyError::TooManyPrintMappings(4_096),
            HardcopyError::InvalidPrintGrayPercent(0),
            HardcopyError::InvalidText {
                field: "project print set mapping name",
                maximum: 128,
            },
            // Nothing in the dialog can be set to repair a broken artifact,
            // receipt, or persisted store.
            HardcopyError::EmptyArtifact,
            HardcopyError::UnsupportedSetupStoreSchema(0),
        ] {
            assert_eq!(contract_section(&error), None, "{error}");
        }
        assert_eq!(
            HardcopyDialogError::SchematicOutputSource("unresolved".to_owned()).section(),
            None
        );
        assert_eq!(
            HardcopyDialogError::OutsideSheetContentDecisionRequired.section(),
            Some(HardcopySection::DrawingSheet)
        );
        assert_eq!(
            HardcopyDialogError::InvalidUnsignedInteger {
                field: "manual rows"
            }
            .section(),
            Some(HardcopySection::Page)
        );
    }

    /// Which section is open and which region a narrow surface is showing are
    /// properties of the looking, not of the hardcopy. If either reached the
    /// setup, reading a different section would rewrite the document.
    #[test]
    fn looking_at_a_different_section_or_region_changes_no_setup() {
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
        let baseline = draft.build_setup().unwrap();
        let mut store = HardcopySetupStore::default();
        let first = store
            .save(draft.source.as_ref().unwrap(), baseline.clone())
            .unwrap();
        assert_eq!(first.disposition(), SetupSaveDisposition::Inserted);
        let digest = first.saved().content_digest();

        for section in HardcopySection::ALL {
            for region in [HardcopyRegion::Setup, HardcopyRegion::Preview] {
                draft.section = section;
                draft.region = region;
                draft.refresh_preview();
                let setup = draft.build_setup().unwrap();
                assert_eq!(setup, baseline, "{section:?} / {region:?}");
                let outcome = store.save(draft.source.as_ref().unwrap(), setup).unwrap();
                assert_eq!(
                    outcome.disposition(),
                    SetupSaveDisposition::Unchanged,
                    "{section:?} / {region:?}"
                );
                assert_eq!(outcome.saved().content_digest(), digest);
                assert_eq!(outcome.saved().revision(), ObjectRevision::INITIAL);
            }
        }
    }

    /// Reopening a saved setup has to give back the setup that was saved. A
    /// draft that reopens even slightly different would report a change nobody
    /// made, and every reopen would dirty the project.
    #[test]
    fn a_saved_setup_reopens_as_itself() {
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
        draft.paper = PaperDraft::Standard(StandardPaper::A3);
        draft.orientation = Orientation::Portrait;
        draft.margin_top = "0.75".to_owned();
        draft.margin_left = "0.6".to_owned();
        draft.scale = ScaleMode::CustomPercent {
            hundredths_percent: 7_550,
        };
        draft.custom_scale_percent = "75.5".to_owned();
        draft.tiling = TilingMode::Manual {
            columns: 2,
            rows: 2,
        };
        draft.manual_columns = "2".to_owned();
        draft.manual_rows = "2".to_owned();
        draft.overlap = "0.125".to_owned();
        draft.registration_marks = true;
        draft.format = OutputFormat::PdfA;
        draft.embed_fonts = true;
        draft.searchable_text = true;
        draft.include_legends = true;
        draft.include_provenance = true;
        draft.watermark = Watermark::Custom("REVIEW ONLY".to_owned());
        draft.refresh_preview();
        let configured = draft.build_setup().expect("the authored draft compiles");
        assert_ne!(configured, HardcopySetup::default());

        let mut store = HardcopySetupStore::default();
        let saved = store
            .save(draft.source.as_ref().unwrap(), configured.clone())
            .unwrap();
        assert_eq!(saved.disposition(), SetupSaveDisposition::Inserted);
        let digest = saved.saved().content_digest();

        let mut reopened = HardcopyDialogState::default();
        reopened.open(
            HardcopyWorkflow::Export,
            source(),
            ContentExtent::try_new(
                Length::from_micrometres(250_000),
                Length::from_micrometres(160_000),
            )
            .unwrap(),
            Some(
                store
                    .setup_for(&source())
                    .unwrap()
                    .expect("the setup was inserted")
                    .setup(),
            ),
        );
        let round_tripped = reopened.build_setup().expect("a saved setup reopens valid");
        assert_eq!(round_tripped, configured);

        let again = store
            .save(reopened.source.as_ref().unwrap(), round_tripped)
            .unwrap();
        assert_eq!(again.disposition(), SetupSaveDisposition::Unchanged);
        assert_eq!(again.saved().content_digest(), digest);
        assert_eq!(again.saved().revision(), ObjectRevision::INITIAL);
    }

    /// A receipt is bound to the plan that was executed, not to whatever the
    /// dialog is showing when it lands. The two can differ — the draft stays
    /// live while a publication is in flight — so the binding is measured.
    #[test]
    fn a_receipt_records_the_plan_that_was_published() {
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
        let published = draft.preview_plan.clone().expect("a compiled plan");
        let pages = published.pagination().pages().len() as u32;
        let receipt = HardcopyReceipt::record(
            &published,
            crate::hardcopy::HardcopyOutcome::ArtifactExported {
                artifact: crate::hardcopy::HardcopyArtifactIdentity::try_new(
                    crate::product::ContentDigest::from_bytes([0x77; 32]),
                    4_096,
                    pages,
                    published.setup().render().format(),
                )
                .unwrap(),
            },
        )
        .expect("an outcome that matches its plan is recordable");

        // The operator keeps configuring while the artifact is written.
        draft.orientation = Orientation::Portrait;
        draft.paper = PaperDraft::Standard(StandardPaper::A3);
        draft.refresh_preview();
        let configured = draft
            .preview_plan
            .clone()
            .expect("the edited draft compiles");
        assert_ne!(configured.content_digest(), published.content_digest());

        assert_eq!(receipt.plan_id(), published.id());
        assert_eq!(receipt.plan_content_digest(), published.content_digest());
        assert_ne!(receipt.plan_content_digest(), configured.content_digest());
        assert_eq!(
            receipt.source_content_digest(),
            published.source().content_digest()
        );
        // The ledger re-derives every receipt's digest from its own material,
        // so a receipt that had been edited to name another plan is refused.
        crate::hardcopy::HardcopyReceiptLedger::default()
            .append(receipt)
            .expect("the receipt digests the plan it names");
    }

    /// The outcome a receipt carries is checked against the plan it names, so a
    /// receipt cannot claim a page count the publication did not produce.
    #[test]
    fn a_receipt_cannot_claim_more_pages_than_the_plan_has() {
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
        let plan = draft.preview_plan.clone().expect("a compiled plan");
        let pages = plan.pagination().pages().len() as u32;
        let overclaimed = HardcopyReceipt::record(
            &plan,
            crate::hardcopy::HardcopyOutcome::ArtifactExported {
                artifact: crate::hardcopy::HardcopyArtifactIdentity::try_new(
                    crate::product::ContentDigest::from_bytes([0x78; 32]),
                    4_096,
                    pages + 1,
                    plan.setup().render().format(),
                )
                .unwrap(),
            },
        );
        assert!(matches!(
            overclaimed,
            Err(HardcopyError::ArtifactPageCountMismatch { .. })
        ));
    }

    #[test]
    fn outside_sheet_decision_error_names_both_valid_publication_choices() {
        let message = HardcopyDialogError::OutsideSheetContentDecisionRequired.to_string();
        assert!(message.contains("Clip"));
        assert!(message.contains("Extend"));
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

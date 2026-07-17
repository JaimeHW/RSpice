//! Standards-enforced PDF/A-2b publication for canonical reports.
//!
//! This module has no UI or filesystem dependencies. Callers provide an exact
//! UTC publication timestamp, which makes repeated exports deterministic and
//! supplies the document date required by PDF/A. Krilla's PDF/A-2b validator
//! is active during serialization, so invalid archival output is never
//! returned as a publication artifact.

use krilla::color::rgb;
use krilla::configure::{Archival, ConfigurationBuilder};
use krilla::geom::{Point, Size, Transform};
use krilla::image::Image;
use krilla::metadata::{DateTime, Metadata, PageLayout};
use krilla::page::PageSettings;
use krilla::paint::Fill;
use krilla::text::{Font, TextDirection};
use krilla::{Document, SerializeSettings};
use sha2::{Digest as _, Sha256};
use unicode_segmentation::UnicodeSegmentation;

use super::report_document::{
    FigureSizing, PlotFigureBlock, ProseStyle, ReportBlockId, ReportBlockKind, ReportDocument,
    ReportPage, ReportReferenceMode, ReportSourceId, RequirementDisposition, ReviewNoteStatus,
    SpecificationDisposition, TableCell,
};
use crate::product::{ContentDigest, ObjectRevision};

const PAGE_WIDTH: f32 = 595.28;
const PAGE_HEIGHT: f32 = 841.89;
const MARGIN_X: f32 = 42.0;
const BODY_TOP: f32 = 66.0;
const BODY_BOTTOM: f32 = 802.0;
const CONTENT_WIDTH: f32 = PAGE_WIDTH - 2.0 * MARGIN_X;
const MAX_AUTHORS: usize = 64;
const MAX_AUTHOR_BYTES: usize = 512;

/// Maximum aggregate source text accepted by one PDF/A publication.
pub const MAX_REPORT_PDFA_SOURCE_TEXT_BYTES: usize = 4 * 1_048_576;
/// Maximum aggregate table cells traversed by one PDF/A publication.
pub const MAX_REPORT_PDFA_TABLE_CELLS: usize = 250_000;
/// Maximum aggregate decoded raster pixels accepted by one PDF/A publication.
pub const MAX_REPORT_PDFA_RASTER_PIXELS: usize = 100_000_000;
/// Maximum serialized size of one PDF/A publication artifact.
pub const MAX_REPORT_PDFA_ARTIFACT_BYTES: usize = 64 * 1_048_576;

const MAX_FIGURE_HEIGHT: f32 = 560.0;
const NATURAL_RASTER_POINTS_PER_PIXEL: f32 = 0.75;

// IBM Plex is licensed for embedding under the SIL Open Font License 1.1;
// the license is retained at assets/fonts/OFL-IBMPlex.txt.
const IBM_PLEX_SANS_REGULAR: &[u8] = include_bytes!("../../assets/fonts/IBMPlexSans-Regular.ttf");
const IBM_PLEX_SANS_SEMIBOLD: &[u8] = include_bytes!("../../assets/fonts/IBMPlexSans-SemiBold.ttf");
const IBM_PLEX_MONO_REGULAR: &[u8] = include_bytes!("../../assets/fonts/IBMPlexMono-Regular.ttf");

/// PDF archival conformance guaranteed by this writer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportPdfAConformance {
    PdfA2b,
}

impl ReportPdfAConformance {
    #[must_use]
    pub const fn label(self) -> &'static str {
        "PDF/A-2b"
    }
}

/// Strict UTC publication timestamp written to PDF Info and XMP metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReportPublicationDate {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl ReportPublicationDate {
    pub fn new(
        year: u16,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, ReportPdfAError> {
        let maximum_day = days_in_month(year, month).ok_or(ReportPdfAError::InvalidDate)?;
        if year == 0
            || year > 9999
            || day == 0
            || day > maximum_day
            || hour > 23
            || minute > 59
            || second > 59
        {
            return Err(ReportPdfAError::InvalidDate);
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    fn as_krilla(self) -> DateTime {
        DateTime::new(self.year)
            .month(self.month)
            .day(self.day)
            .hour(self.hour)
            .minute(self.minute)
            .second(self.second)
            .utc_offset_hour(0)
            .utc_offset_minute(0)
    }

    fn display_utc(self) -> String {
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

const fn days_in_month(year: u16, month: u8) -> Option<u8> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
        4 | 6 | 9 | 11 => Some(30),
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            Some(29)
        }
        2 => Some(28),
        _ => None,
    }
}

/// Immutable publication inputs that are not report-domain state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportPdfAOptions {
    pub publication_date: ReportPublicationDate,
    pub authors: Vec<String>,
}

impl ReportPdfAOptions {
    #[must_use]
    pub const fn new(publication_date: ReportPublicationDate) -> Self {
        Self {
            publication_date,
            authors: Vec::new(),
        }
    }

    fn validate(&self) -> Result<(), ReportPdfAError> {
        let invalid_author = self.authors.iter().any(|author| {
            author.is_empty()
                || author != author.trim()
                || author.len() > MAX_AUTHOR_BYTES
                || author.chars().any(char::is_control)
        });
        if self.authors.len() > MAX_AUTHORS || invalid_author {
            return Err(ReportPdfAError::InvalidAuthors);
        }
        Ok(())
    }
}

/// Complete, authenticated in-memory archival publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportPdfAArtifact {
    bytes: Vec<u8>,
    digest: ContentDigest,
}

impl ReportPdfAArtifact {
    pub const MEDIA_TYPE: &'static str = "application/pdf";

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    #[must_use]
    pub const fn conformance(&self) -> ReportPdfAConformance {
        ReportPdfAConformance::PdfA2b
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReportPdfAError {
    #[error("publication date must be a valid UTC Gregorian date and time")]
    InvalidDate,
    #[error("authors must be trimmed printable names of at most 512 bytes (maximum 64 authors)")]
    InvalidAuthors,
    #[error("report document is invalid: {0}")]
    InvalidReport(String),
    #[error(
        "plot figure block {block_id} cannot be published until its exact vector or raster artwork is resolved"
    )]
    UnresolvedPlotFigure { block_id: ReportBlockId },
    #[error(
        "plot figure block {block_id} uses unsupported PDF/A artwork media type `{media_type}`; use an opaque PNG or JPEG"
    )]
    UnsupportedPlotFigureMediaType {
        block_id: ReportBlockId,
        media_type: String,
    },
    #[error("plot figure block {block_id} contains invalid raster artwork: {detail}")]
    InvalidPlotFigureArtwork {
        block_id: ReportBlockId,
        detail: String,
    },
    #[error("embedded report font {0} is invalid")]
    InvalidEmbeddedFont(&'static str),
    #[error("could not configure PDF/A-2b validation: {0}")]
    InvalidPdfAConfiguration(String),
    #[error("could not serialize a conforming PDF/A-2b report: {0}")]
    Serialization(String),
    #[error("PDF/A publication exceeds the {scope} limit of {maximum} units")]
    ResourceLimit { scope: &'static str, maximum: usize },
}

/// Serialize a canonical report under krilla's PDF/A-2b validator.
///
/// Plot figures require resolved artwork. Until a caller can provide that
/// artwork, this writer fails before serialization instead of omitting a figure
/// or inserting a non-evidentiary placeholder.
pub fn serialize_report_pdfa_2b(
    report: &ReportDocument,
    options: &ReportPdfAOptions,
) -> Result<ReportPdfAArtifact, ReportPdfAError> {
    report
        .validate()
        .map_err(|error| ReportPdfAError::InvalidReport(error.to_string()))?;
    options.validate()?;
    preflight(report)?;

    let configuration = ConfigurationBuilder::new()
        .with_archival_validator(Archival::A2_B)
        .finish()
        .map_err(|error| ReportPdfAError::InvalidPdfAConfiguration(format!("{error:?}")))?;
    let settings = SerializeSettings {
        pretty: true,
        compress_content_streams: false,
        no_device_cs: true,
        xmp_metadata: true,
        configuration,
        enable_tagging: false,
        ..SerializeSettings::default()
    };

    let fonts = Fonts::load()?;
    let page_settings = PageSettings::from_wh(PAGE_WIDTH, PAGE_HEIGHT)
        .ok_or_else(|| ReportPdfAError::Serialization("invalid A4 page dimensions".to_owned()))?;
    let mut document = Document::new_with(settings);
    let mut metadata = Metadata::new()
        .title(report.title().to_owned())
        .description(format!(
            "RSpice governed engineering report, document {} revision {}",
            report.id(),
            report.revision().get()
        ))
        .creator("RSpice Report Composer".to_owned())
        .producer("RSpice standards-enforced PDF/A writer".to_owned())
        .document_id(format!("rspice-report-{}", report.id()))
        .language("en-US".to_owned())
        .creation_date(options.publication_date.as_krilla())
        .page_layout(PageLayout::OneColumn);
    if !options.authors.is_empty() {
        metadata = metadata.authors(options.authors.clone());
    }
    document.set_metadata(metadata);

    {
        let mut paginator = Paginator::new(
            &mut document,
            &fonts,
            page_settings,
            report.title(),
            options.publication_date,
        );
        render_cover(&mut paginator, report);
        for page in report.pages() {
            paginator.begin_logical_page(page.title());
            render_report_page(&mut paginator, page)?;
        }
        paginator.finish();
    }

    let bytes = document
        .finish()
        .map_err(|error| ReportPdfAError::Serialization(error.to_string()))?;
    validate_output_size(bytes.len())?;
    let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
    Ok(ReportPdfAArtifact { bytes, digest })
}

fn preflight(report: &ReportDocument) -> Result<(), ReportPdfAError> {
    let mut workload = PublicationWorkload::default();
    workload.add_text(report.title())?;
    for page in report.pages() {
        workload.add_text(page.title())?;
        for section in page.sections() {
            workload.add_text(section.title())?;
            for block in section.blocks() {
                workload.add_block(block.id(), block.kind())?;
            }
        }
    }
    Ok(())
}

#[derive(Default)]
struct PublicationWorkload {
    source_text_bytes: usize,
    table_cells: usize,
    raster_pixels: usize,
}

impl PublicationWorkload {
    fn add_text(&mut self, text: &str) -> Result<(), ReportPdfAError> {
        self.source_text_bytes = self.source_text_bytes.checked_add(text.len()).ok_or(
            ReportPdfAError::ResourceLimit {
                scope: "source text bytes",
                maximum: MAX_REPORT_PDFA_SOURCE_TEXT_BYTES,
            },
        )?;
        if self.source_text_bytes > MAX_REPORT_PDFA_SOURCE_TEXT_BYTES {
            return Err(ReportPdfAError::ResourceLimit {
                scope: "source text bytes",
                maximum: MAX_REPORT_PDFA_SOURCE_TEXT_BYTES,
            });
        }
        Ok(())
    }

    fn add_cells(&mut self, count: usize) -> Result<(), ReportPdfAError> {
        self.table_cells =
            self.table_cells
                .checked_add(count)
                .ok_or(ReportPdfAError::ResourceLimit {
                    scope: "table cells",
                    maximum: MAX_REPORT_PDFA_TABLE_CELLS,
                })?;
        if self.table_cells > MAX_REPORT_PDFA_TABLE_CELLS {
            return Err(ReportPdfAError::ResourceLimit {
                scope: "table cells",
                maximum: MAX_REPORT_PDFA_TABLE_CELLS,
            });
        }
        Ok(())
    }

    fn add_reference(&mut self, reference: &ReportReferenceMode) -> Result<(), ReportPdfAError> {
        if let ReportSourceId::ExternalRecord { namespace, key } = &reference.snapshot().source {
            self.add_text(namespace)?;
            self.add_text(key)?;
        }
        if let Some(artifact) = reference.frozen_artifact() {
            self.add_text(artifact.media_type())?;
        }
        Ok(())
    }

    fn add_raster_pixels(&mut self, count: u64) -> Result<(), ReportPdfAError> {
        let count = usize::try_from(count).map_err(|_| ReportPdfAError::ResourceLimit {
            scope: "decoded raster pixels",
            maximum: MAX_REPORT_PDFA_RASTER_PIXELS,
        })?;
        self.raster_pixels =
            self.raster_pixels
                .checked_add(count)
                .ok_or(ReportPdfAError::ResourceLimit {
                    scope: "decoded raster pixels",
                    maximum: MAX_REPORT_PDFA_RASTER_PIXELS,
                })?;
        if self.raster_pixels > MAX_REPORT_PDFA_RASTER_PIXELS {
            return Err(ReportPdfAError::ResourceLimit {
                scope: "decoded raster pixels",
                maximum: MAX_REPORT_PDFA_RASTER_PIXELS,
            });
        }
        Ok(())
    }

    fn add_block(
        &mut self,
        block_id: ReportBlockId,
        kind: &ReportBlockKind,
    ) -> Result<(), ReportPdfAError> {
        match kind {
            ReportBlockKind::PlotFigure(block) => {
                self.add_text(&block.caption)?;
                self.add_text(&block.alternative_text)?;
                self.add_reference(&block.reference)?;
                let resolved = resolve_plot_figure(block_id, block)?;
                self.add_raster_pixels(resolved.pixel_count())?;
            }
            ReportBlockKind::DataTable(block) => {
                self.add_text(&block.title)?;
                self.add_cells(block.columns.len().checked_mul(block.rows.len()).ok_or(
                    ReportPdfAError::ResourceLimit {
                        scope: "table cells",
                        maximum: MAX_REPORT_PDFA_TABLE_CELLS,
                    },
                )?)?;
                for column in &block.columns {
                    self.add_text(&column.key)?;
                    self.add_text(&column.heading)?;
                    if let Some(unit) = &column.unit {
                        self.add_text(unit)?;
                    }
                }
                for cell in block.rows.iter().flatten() {
                    match cell {
                        TableCell::Text(text) => self.add_text(text)?,
                        TableCell::Number {
                            unit: Some(unit), ..
                        } => self.add_text(unit)?,
                        _ => {}
                    }
                }
                self.add_reference(&block.reference)?;
            }
            ReportBlockKind::Datasheet(block) => {
                self.add_text(&block.title)?;
                for field in &block.fields {
                    self.add_text(&field.key)?;
                    self.add_text(&field.label)?;
                    self.add_text(&field.value)?;
                    if let Some(unit) = &field.unit {
                        self.add_text(unit)?;
                    }
                }
                self.add_reference(&block.reference)?;
            }
            ReportBlockKind::Requirements(block) => {
                self.add_text(&block.title)?;
                for entry in &block.entries {
                    self.add_text(&entry.requirement_id)?;
                    self.add_text(&entry.statement)?;
                    if let Some(label) = &entry.evidence_label {
                        self.add_text(label)?;
                    }
                }
                self.add_reference(&block.reference)?;
            }
            ReportBlockKind::Specifications(block) => {
                self.add_text(&block.title)?;
                for entry in &block.entries {
                    self.add_text(&entry.expression)?;
                    self.add_text(&entry.limit)?;
                    if let Some(measured) = &entry.measured {
                        self.add_text(measured)?;
                    }
                }
                self.add_reference(&block.reference)?;
            }
            ReportBlockKind::Prose(block) => self.add_text(&block.markdown)?,
            ReportBlockKind::ReviewNote(block) => {
                self.add_text(&block.author)?;
                self.add_text(&block.message)?;
            }
            ReportBlockKind::Evidence(block) => {
                self.add_text(&block.title)?;
                self.add_text(&block.summary)?;
                self.add_reference(&block.reference)?;
            }
        }
        Ok(())
    }
}

fn validate_output_size(bytes: usize) -> Result<(), ReportPdfAError> {
    if bytes == 0 || bytes > MAX_REPORT_PDFA_ARTIFACT_BYTES {
        return Err(ReportPdfAError::ResourceLimit {
            scope: "serialized artifact bytes",
            maximum: MAX_REPORT_PDFA_ARTIFACT_BYTES,
        });
    }
    Ok(())
}

#[derive(Clone)]
struct Fonts {
    regular: Font,
    semibold: Font,
    mono: Font,
}

impl Fonts {
    fn load() -> Result<Self, ReportPdfAError> {
        Ok(Self {
            regular: load_font(IBM_PLEX_SANS_REGULAR, "IBM Plex Sans Regular")?,
            semibold: load_font(IBM_PLEX_SANS_SEMIBOLD, "IBM Plex Sans Semibold")?,
            mono: load_font(IBM_PLEX_MONO_REGULAR, "IBM Plex Mono Regular")?,
        })
    }
}

fn load_font(bytes: &'static [u8], name: &'static str) -> Result<Font, ReportPdfAError> {
    Font::new(bytes.into(), 0).ok_or(ReportPdfAError::InvalidEmbeddedFont(name))
}

#[derive(Clone, Copy)]
enum LineStyle {
    Title,
    PageTitle,
    Section,
    Body,
    Detail,
    Mono,
    Warning,
}

impl LineStyle {
    const fn size(self) -> f32 {
        match self {
            Self::Title => 22.0,
            Self::PageTitle => 15.0,
            Self::Section => 10.5,
            Self::Body | Self::Warning => 9.0,
            Self::Detail | Self::Mono => 7.6,
        }
    }

    const fn line_height(self) -> f32 {
        match self {
            Self::Title => 30.0,
            Self::PageTitle => 23.0,
            Self::Section => 17.0,
            Self::Body | Self::Warning => 13.5,
            Self::Detail | Self::Mono => 11.5,
        }
    }

    const fn top_gap(self) -> f32 {
        match self {
            Self::Title => 12.0,
            Self::PageTitle | Self::Section => 8.0,
            _ => 0.0,
        }
    }

    const fn mono(self) -> bool {
        matches!(self, Self::Mono)
    }
}

struct PlacedLine {
    text: String,
    style: LineStyle,
    y: f32,
}

struct PlacedImage {
    image: Image,
    size: Size,
    y: f32,
}

struct ResolvedPlotFigure {
    image: Image,
    width_pixels: u32,
    height_pixels: u32,
    size: Size,
}

impl ResolvedPlotFigure {
    fn pixel_count(&self) -> u64 {
        u64::from(self.width_pixels) * u64::from(self.height_pixels)
    }
}

struct Paginator<'a> {
    document: &'a mut Document,
    fonts: &'a Fonts,
    page_settings: PageSettings,
    report_title: &'a str,
    logical_page_title: String,
    publication_date: ReportPublicationDate,
    lines: Vec<PlacedLine>,
    images: Vec<PlacedImage>,
    next_y: f32,
    page_number: usize,
}

impl<'a> Paginator<'a> {
    fn new(
        document: &'a mut Document,
        fonts: &'a Fonts,
        page_settings: PageSettings,
        report_title: &'a str,
        publication_date: ReportPublicationDate,
    ) -> Self {
        Self {
            document,
            fonts,
            page_settings,
            report_title,
            logical_page_title: "Cover".to_owned(),
            publication_date,
            lines: Vec::new(),
            images: Vec::new(),
            next_y: BODY_TOP,
            page_number: 0,
        }
    }

    fn begin_logical_page(&mut self, title: &str) {
        self.flush();
        self.logical_page_title = title.to_owned();
        self.emit(title, LineStyle::PageTitle);
    }

    fn emit(&mut self, text: &str, style: LineStyle) {
        for line in wrap_text(text, CONTENT_WIDTH, style.size(), style.mono()) {
            let needed = style.top_gap() + style.line_height();
            if self.next_y + needed > BODY_BOTTOM && !self.lines.is_empty() {
                self.flush();
            }
            self.next_y += style.top_gap();
            self.lines.push(PlacedLine {
                text: line,
                style,
                y: self.next_y + style.size(),
            });
            self.next_y += style.line_height();
        }
    }

    fn spacer(&mut self, points: f32) {
        if self.next_y + points > BODY_BOTTOM && self.has_content() {
            self.flush();
        } else {
            self.next_y += points;
        }
    }

    fn emit_image(&mut self, image: Image, size: Size) {
        const TOP_GAP: f32 = 8.0;
        let needed = TOP_GAP + size.height();
        if self.next_y + needed > BODY_BOTTOM && self.has_content() {
            self.flush();
        }
        self.next_y += TOP_GAP;
        self.images.push(PlacedImage {
            image,
            size,
            y: self.next_y,
        });
        self.next_y += size.height();
    }

    fn has_content(&self) -> bool {
        !self.lines.is_empty() || !self.images.is_empty()
    }

    fn flush(&mut self) {
        if !self.has_content() {
            return;
        }
        self.page_number += 1;
        let mut page = self.document.start_page_with(self.page_settings.clone());
        let mut surface = page.surface();
        draw_text(
            &mut surface,
            &self.fonts.semibold,
            7.2,
            MARGIN_X,
            35.0,
            self.report_title,
            Ink::Muted,
        );
        draw_text(
            &mut surface,
            &self.fonts.regular,
            7.2,
            MARGIN_X,
            48.0,
            &self.logical_page_title,
            Ink::Muted,
        );
        for placed in &self.images {
            surface.push_transform(&Transform::from_translate(MARGIN_X, placed.y));
            surface.draw_image(placed.image.clone(), placed.size);
            surface.pop();
        }
        for line in &self.lines {
            let (font, ink) = match line.style {
                LineStyle::Title | LineStyle::PageTitle | LineStyle::Section => {
                    (&self.fonts.semibold, Ink::Text)
                }
                LineStyle::Mono => (&self.fonts.mono, Ink::Text),
                LineStyle::Warning => (&self.fonts.regular, Ink::Warning),
                LineStyle::Body | LineStyle::Detail => (&self.fonts.regular, Ink::Text),
            };
            draw_text(
                &mut surface,
                font,
                line.style.size(),
                MARGIN_X,
                line.y,
                &line.text,
                ink,
            );
        }
        draw_text(
            &mut surface,
            &self.fonts.regular,
            7.0,
            MARGIN_X,
            823.0,
            &format!(
                "{} · {}",
                ReportPdfAConformance::PdfA2b.label(),
                self.publication_date.display_utc()
            ),
            Ink::Muted,
        );
        draw_text(
            &mut surface,
            &self.fonts.mono,
            7.0,
            PAGE_WIDTH - MARGIN_X - 55.0,
            823.0,
            &format!("Page {}", self.page_number),
            Ink::Muted,
        );
        surface.finish();
        page.finish();
        self.lines.clear();
        self.images.clear();
        self.next_y = BODY_TOP;
    }

    fn finish(mut self) {
        self.flush();
    }
}

fn render_cover(paginator: &mut Paginator<'_>, report: &ReportDocument) {
    paginator.emit(report.title(), LineStyle::Title);
    paginator.spacer(12.0);
    paginator.emit("GOVERNED ENGINEERING REPORT", LineStyle::Section);
    paginator.emit(
        &format!("Document identity: {}", report.id()),
        LineStyle::Mono,
    );
    paginator.emit(
        &format!("Document revision: {}", report.revision().get()),
        LineStyle::Mono,
    );
    paginator.emit(
        &format!("Schema version: {}", report.schema_version()),
        LineStyle::Mono,
    );
    paginator.emit(
        &format!("Template: {:?}", report.template()),
        LineStyle::Detail,
    );
    paginator.emit(
        &format!("Report pages: {}", report.pages().len()),
        LineStyle::Detail,
    );
}

fn render_report_page(
    paginator: &mut Paginator<'_>,
    page: &ReportPage,
) -> Result<(), ReportPdfAError> {
    paginator.emit(
        &format!(
            "Page identity {} · revision {} · update policy {:?}",
            page.id(),
            page.revision().get(),
            page.update_policy()
        ),
        LineStyle::Detail,
    );
    if page.sections().is_empty() {
        paginator.emit(
            "No report sections in this governed revision.",
            LineStyle::Body,
        );
        return Ok(());
    }
    for section in page.sections() {
        paginator.emit(section.title(), LineStyle::Section);
        paginator.emit(
            &format!(
                "Section {} · revision {}",
                section.id(),
                section.revision().get()
            ),
            LineStyle::Detail,
        );
        if section.blocks().is_empty() {
            paginator.emit(
                "No content blocks in this governed revision.",
                LineStyle::Body,
            );
        }
        for block in section.blocks() {
            render_block(paginator, block.id(), block.kind())?;
            paginator.emit(
                &format!("Block {} · revision {}", block.id(), block.revision().get()),
                LineStyle::Detail,
            );
            paginator.spacer(4.0);
        }
    }
    Ok(())
}

fn render_block(
    paginator: &mut Paginator<'_>,
    block_id: ReportBlockId,
    kind: &ReportBlockKind,
) -> Result<(), ReportPdfAError> {
    match kind {
        ReportBlockKind::PlotFigure(block) => {
            let resolved = resolve_plot_figure(block_id, block)?;
            paginator.emit(&block.caption, LineStyle::Section);
            paginator.emit_image(resolved.image, resolved.size);
            paginator.emit(&block.alternative_text, LineStyle::Detail);
            render_reference(paginator, &block.reference);
        }
        ReportBlockKind::DataTable(block) => {
            paginator.emit(&block.title, LineStyle::Section);
            let headings = block
                .columns
                .iter()
                .map(|column| match &column.unit {
                    Some(unit) => format!("{} [{}]", column.heading, unit),
                    None => column.heading.clone(),
                })
                .collect::<Vec<_>>()
                .join(" | ");
            paginator.emit(&headings, LineStyle::Mono);
            for row in &block.rows {
                paginator.emit(
                    &row.iter().map(table_cell).collect::<Vec<_>>().join(" | "),
                    LineStyle::Mono,
                );
            }
            render_reference(paginator, &block.reference);
        }
        ReportBlockKind::Datasheet(block) => {
            paginator.emit(&block.title, LineStyle::Section);
            for field in &block.fields {
                paginator.emit(
                    &format!(
                        "{}: {}{}",
                        field.label,
                        field.value,
                        field
                            .unit
                            .as_ref()
                            .map_or(String::new(), |unit| format!(" {unit}"))
                    ),
                    LineStyle::Body,
                );
            }
            render_reference(paginator, &block.reference);
        }
        ReportBlockKind::Requirements(block) => {
            paginator.emit(&block.title, LineStyle::Section);
            for entry in &block.entries {
                paginator.emit(
                    &format!(
                        "{} [{}] {}{}",
                        entry.requirement_id,
                        requirement_disposition(entry.disposition),
                        entry.statement,
                        entry
                            .evidence_label
                            .as_ref()
                            .map_or(String::new(), |label| format!(" · Evidence: {label}"))
                    ),
                    if entry.disposition == RequirementDisposition::Failed {
                        LineStyle::Warning
                    } else {
                        LineStyle::Body
                    },
                );
            }
            render_reference(paginator, &block.reference);
        }
        ReportBlockKind::Specifications(block) => {
            paginator.emit(&block.title, LineStyle::Section);
            for entry in &block.entries {
                paginator.emit(
                    &format!(
                        "{} · limit {} · measured {} · {}",
                        entry.expression,
                        entry.limit,
                        entry.measured.as_deref().unwrap_or("not evaluated"),
                        specification_disposition(entry.disposition)
                    ),
                    if entry.disposition == SpecificationDisposition::OutOfSpecification {
                        LineStyle::Warning
                    } else {
                        LineStyle::Mono
                    },
                );
            }
            render_reference(paginator, &block.reference);
        }
        ReportBlockKind::Prose(block) => {
            paginator.emit(prose_style(block.style), LineStyle::Section);
            for line in block.markdown.lines() {
                paginator.emit(line, LineStyle::Body);
            }
        }
        ReportBlockKind::ReviewNote(block) => {
            paginator.emit("Review note", LineStyle::Section);
            paginator.emit(
                &format!(
                    "{} · {} · opened {}{}",
                    block.author,
                    review_status(block.status),
                    block.created_at_unix_ms,
                    block
                        .resolved_at_unix_ms
                        .map_or(String::new(), |resolved| format!(" · resolved {resolved}"))
                ),
                LineStyle::Detail,
            );
            paginator.emit(&block.message, LineStyle::Body);
        }
        ReportBlockKind::Evidence(block) => {
            paginator.emit(&block.title, LineStyle::Section);
            paginator.emit(&block.summary, LineStyle::Body);
            render_reference(paginator, &block.reference);
        }
    }
    Ok(())
}

fn resolve_plot_figure(
    block_id: ReportBlockId,
    block: &PlotFigureBlock,
) -> Result<ResolvedPlotFigure, ReportPdfAError> {
    let artifact = block
        .reference
        .frozen_artifact()
        .ok_or(ReportPdfAError::UnresolvedPlotFigure { block_id })?;
    let payload = artifact.payload().to_vec();
    let image = match artifact.media_type() {
        "image/png" => {
            let reader = png::Decoder::new(std::io::Cursor::new(payload.as_slice()))
                .read_info()
                .map_err(|error| ReportPdfAError::InvalidPlotFigureArtwork {
                    block_id,
                    detail: error.to_string(),
                })?;
            let info = reader.info();
            if matches!(
                info.color_type,
                png::ColorType::GrayscaleAlpha | png::ColorType::Rgba
            ) || info.trns.is_some()
            {
                return Err(ReportPdfAError::InvalidPlotFigureArtwork {
                    block_id,
                    detail: "PDF/A-2b plot artwork must not use a PNG alpha channel".to_owned(),
                });
            }
            if info.bit_depth == png::BitDepth::Sixteen {
                return Err(ReportPdfAError::InvalidPlotFigureArtwork {
                    block_id,
                    detail: "PDF/A-2b plot artwork must use at most 8 bits per component"
                        .to_owned(),
                });
            }
            Image::from_png(payload.into(), false)
        }
        "image/jpeg" => Image::from_jpeg(payload.into(), false),
        media_type => {
            return Err(ReportPdfAError::UnsupportedPlotFigureMediaType {
                block_id,
                media_type: media_type.to_owned(),
            });
        }
    }
    .map_err(|detail| ReportPdfAError::InvalidPlotFigureArtwork { block_id, detail })?;
    let (width_pixels, height_pixels) = image.size();
    let (natural_width, natural_height) = match block.sizing {
        FigureSizing::Natural => (
            width_pixels as f32 * NATURAL_RASTER_POINTS_PER_PIXEL,
            height_pixels as f32 * NATURAL_RASTER_POINTS_PER_PIXEL,
        ),
        FigureSizing::FitWidth | FigureSizing::FitPage => {
            (width_pixels as f32, height_pixels as f32)
        }
    };
    let width_scale = CONTENT_WIDTH / natural_width;
    let height_scale = MAX_FIGURE_HEIGHT / natural_height;
    let scale = match block.sizing {
        FigureSizing::Natural => 1.0_f32.min(width_scale).min(height_scale),
        FigureSizing::FitWidth => width_scale.min(height_scale),
        FigureSizing::FitPage => width_scale.min(height_scale).min(1.0),
    };
    let width = natural_width * scale;
    let height = natural_height * scale;
    let size =
        Size::from_wh(width, height).ok_or_else(|| ReportPdfAError::InvalidPlotFigureArtwork {
            block_id,
            detail: "decoded image dimensions cannot be represented on the PDF page".to_owned(),
        })?;
    Ok(ResolvedPlotFigure {
        image,
        width_pixels,
        height_pixels,
        size,
    })
}

fn render_reference(paginator: &mut Paginator<'_>, reference: &ReportReferenceMode) {
    let snapshot = reference.snapshot();
    paginator.emit(
        &format!(
            "Source: {} · mode {} · revision {} · digest {}",
            source_label(&snapshot.source),
            if reference.is_frozen() {
                "frozen"
            } else {
                "linked"
            },
            snapshot
                .source_revision
                .map_or_else(|| "n/a".to_owned(), revision_label),
            snapshot.content_digest
        ),
        LineStyle::Detail,
    );
    for binding in &snapshot.dataset_bindings {
        paginator.emit(
            &format!(
                "Dataset binding: {} · {}",
                binding.dataset_id, binding.content_digest
            ),
            LineStyle::Detail,
        );
    }
    if let Some(artifact) = reference.frozen_artifact() {
        paginator.emit(
            &format!(
                "Frozen artifact: {} · {} bytes · digest {}",
                artifact.media_type(),
                artifact.payload().len(),
                artifact.content_digest()
            ),
            LineStyle::Detail,
        );
    }
}

fn revision_label(revision: ObjectRevision) -> String {
    revision.get().to_string()
}

fn source_label(source: &ReportSourceId) -> String {
    match source {
        ReportSourceId::VisualizationDocument { document_id } => {
            format!("visualization-document:{document_id}")
        }
        ReportSourceId::Dataset { dataset_id } => format!("dataset:{dataset_id}"),
        ReportSourceId::VerificationEvidence { evidence_id } => {
            format!("verification-evidence:{evidence_id}")
        }
        ReportSourceId::ExternalRecord { namespace, key } => format!("external:{namespace}:{key}"),
    }
}

fn table_cell(cell: &TableCell) -> String {
    match cell {
        TableCell::Empty => String::new(),
        TableCell::Text(value) => value.clone(),
        TableCell::Number { value, unit } => format!(
            "{value}{}",
            unit.as_ref()
                .map_or(String::new(), |unit| format!(" {unit}"))
        ),
        TableCell::Integer(value) => value.to_string(),
        TableCell::Boolean(value) => value.to_string(),
    }
}

const fn requirement_disposition(value: RequirementDisposition) -> &'static str {
    match value {
        RequirementDisposition::NotEvaluated => "not evaluated",
        RequirementDisposition::Passed => "passed",
        RequirementDisposition::Failed => "failed",
        RequirementDisposition::Waived => "waived",
    }
}

const fn specification_disposition(value: SpecificationDisposition) -> &'static str {
    match value {
        SpecificationDisposition::NotEvaluated => "not evaluated",
        SpecificationDisposition::InSpecification => "in specification",
        SpecificationDisposition::OutOfSpecification => "out of specification",
        SpecificationDisposition::Informational => "informational",
    }
}

const fn prose_style(value: ProseStyle) -> &'static str {
    match value {
        ProseStyle::Body => "Narrative",
        ProseStyle::ExecutiveSummary => "Executive summary",
        ProseStyle::Method => "Method",
        ProseStyle::Conclusion => "Conclusion",
        ProseStyle::Warning => "Warning",
    }
}

const fn review_status(value: ReviewNoteStatus) -> &'static str {
    match value {
        ReviewNoteStatus::Open => "open",
        ReviewNoteStatus::Addressed => "addressed",
        ReviewNoteStatus::Accepted => "accepted",
    }
}

#[derive(Clone, Copy)]
enum Ink {
    Text,
    Muted,
    Warning,
}

impl Ink {
    fn rgb(self) -> rgb::Color {
        match self {
            Self::Text => rgb::Color::new(25, 32, 38),
            Self::Muted => rgb::Color::new(88, 99, 108),
            Self::Warning => rgb::Color::new(153, 45, 35),
        }
    }
}

fn draw_text(
    surface: &mut krilla::surface::Surface<'_>,
    font: &Font,
    size: f32,
    x: f32,
    y: f32,
    text: &str,
    ink: Ink,
) {
    surface.set_stroke(None);
    surface.set_fill(Some(Fill {
        paint: ink.rgb().into(),
        ..Fill::default()
    }));
    surface.draw_text(
        Point::from_xy(x, y),
        font.clone(),
        size,
        text,
        false,
        TextDirection::LeftToRight,
    );
}

fn wrap_text(value: &str, max_width: f32, font_size: f32, mono: bool) -> Vec<String> {
    let cleaned = clean_text(value);
    if cleaned.is_empty() {
        return vec!["-".to_owned()];
    }
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_width = 0.0_f32;
    let space_width = grapheme_width(" ", font_size, mono);
    for word in cleaned.split_whitespace() {
        let word_width = text_width(word, font_size, mono);
        if !current.is_empty() && current_width + space_width + word_width <= max_width {
            current.push(' ');
            current.push_str(word);
            current_width += space_width + word_width;
            continue;
        }
        if !current.is_empty() {
            lines.push(std::mem::take(&mut current));
        }
        if word_width <= max_width {
            current.push_str(word);
            current_width = word_width;
            continue;
        }
        let mut segment = String::new();
        let mut segment_width = 0.0_f32;
        for grapheme in word.graphemes(true) {
            let width = grapheme_width(grapheme, font_size, mono);
            if !segment.is_empty() && segment_width + width > max_width {
                lines.push(std::mem::take(&mut segment));
                segment_width = 0.0;
            }
            segment.push_str(grapheme);
            segment_width += width;
        }
        current = segment;
        current_width = segment_width;
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn clean_text(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            '\r' | '\n' | '\t' => ' ',
            character if character.is_control() => ' ',
            character => character,
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn text_width(value: &str, font_size: f32, mono: bool) -> f32 {
    value
        .graphemes(true)
        .map(|grapheme| grapheme_width(grapheme, font_size, mono))
        .sum::<f32>()
}

fn grapheme_width(grapheme: &str, font_size: f32, mono: bool) -> f32 {
    let factor = if mono {
        0.6
    } else {
        match grapheme {
            " " => 0.28,
            "i" | "l" | "I" | "." | "," | ":" | ";" | "'" | "!" | "|" => 0.29,
            "m" | "w" | "M" | "W" | "@" | "%" | "&" => 0.82,
            _ if grapheme.is_ascii() => 0.54,
            _ => 0.64,
        }
    };
    factor * font_size
}

#[cfg(test)]
mod tests {
    use krilla::configure::ValidationError;
    use krilla::error::KrillaError;
    use lopdf::{Document as ParsedPdf, Object};

    use super::*;
    use crate::product::ResultDocumentId;
    use crate::results::report_document::{
        FrozenReportArtifact, PlotFigureBlock, ProseBlock, ReportEdit, ReportEntityRef,
        ReportPageUpdatePolicy, ReportReferenceSnapshot,
    };

    fn publication_options() -> ReportPdfAOptions {
        let mut options =
            ReportPdfAOptions::new(ReportPublicationDate::new(2026, 7, 17, 14, 30, 0).unwrap());
        options.authors = vec!["RSpice Verification Team".to_owned()];
        options
    }

    fn prose_report() -> ReportDocument {
        let mut report = ReportDocument::new("Precision sensor AFE verification").unwrap();
        let receipt = report
            .transact(
                report.revision(),
                vec![ReportEdit::AddPage {
                    title: "Executive summary".to_owned(),
                }],
                1,
            )
            .unwrap();
        let page_id = match receipt.created[0] {
            ReportEntityRef::Page(id) => id,
            _ => unreachable!(),
        };
        let page_revision = report.page(page_id).unwrap().revision();
        let receipt = report
            .transact(
                report.revision(),
                vec![
                    ReportEdit::SetPageUpdatePolicy {
                        page_id,
                        expected_page_revision: page_revision,
                        update_policy: ReportPageUpdatePolicy::FreezeSelectedRevision,
                    },
                    ReportEdit::AddSection {
                        page_id,
                        title: "Release decision".to_owned(),
                    },
                ],
                2,
            )
            .unwrap();
        let section_id = receipt
            .created
            .iter()
            .find_map(|entity| match entity {
                ReportEntityRef::Section(id) => Some(*id),
                _ => None,
            })
            .unwrap();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: ReportBlockKind::Prose(ProseBlock {
                        style: ProseStyle::ExecutiveSummary,
                        markdown: "All governed specifications passed at the retained revision."
                            .to_owned(),
                    }),
                }],
                3,
            )
            .unwrap();
        report
    }

    fn reference_snapshot() -> ReportReferenceSnapshot {
        ReportReferenceSnapshot::new(
            ReportSourceId::VisualizationDocument {
                document_id: ResultDocumentId::new(),
            },
            Some(ObjectRevision::INITIAL),
            ContentDigest::from_bytes([0xA5; 32]),
            Vec::new(),
        )
        .unwrap()
    }

    fn figure_report(reference: ReportReferenceMode) -> (ReportDocument, ReportBlockId) {
        let mut report = prose_report();
        let section_id = report.pages()[0].sections()[0].id();
        let receipt = report
            .transact(
                report.revision(),
                vec![ReportEdit::AddBlock {
                    section_id,
                    kind: ReportBlockKind::PlotFigure(PlotFigureBlock {
                        caption: "Closed-loop gain and phase".to_owned(),
                        alternative_text:
                            "Gain remains above 20 dB through the retained measurement band."
                                .to_owned(),
                        sizing: FigureSizing::FitWidth,
                        reference,
                    }),
                }],
                4,
            )
            .unwrap();
        let block_id = receipt
            .created
            .iter()
            .find_map(|entity| match entity {
                ReportEntityRef::Block(id) => Some(*id),
                _ => None,
            })
            .unwrap();
        (report, block_id)
    }

    fn encoded_png(color_type: png::ColorType) -> Vec<u8> {
        let channels = match color_type {
            png::ColorType::Rgb => 3,
            png::ColorType::Rgba => 4,
            _ => panic!("test helper only supports RGB and RGBA"),
        };
        let mut pixels = Vec::with_capacity(8 * channels);
        for index in 0..8_u8 {
            pixels.extend_from_slice(&[16 + index, 48 + index, 96 + index]);
            if channels == 4 {
                pixels.push(u8::MAX);
            }
        }
        let mut encoded = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut encoded, 4, 2);
            encoder.set_color(color_type);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().unwrap();
            writer.write_image_data(&pixels).unwrap();
            writer.finish().unwrap();
        }
        encoded
    }

    fn catalog(pdf: &ParsedPdf) -> &lopdf::Dictionary {
        let root_id = pdf.trailer.get(b"Root").unwrap().as_reference().unwrap();
        pdf.get_object(root_id).unwrap().as_dict().unwrap()
    }

    fn dereference<'a>(pdf: &'a ParsedPdf, object: &'a Object) -> &'a Object {
        match object {
            Object::Reference(id) => pdf.get_object(*id).unwrap(),
            object => object,
        }
    }

    #[test]
    fn writer_emits_pdfa_2b_xmp_output_intent_and_icc_profile() {
        let artifact = serialize_report_pdfa_2b(&prose_report(), &publication_options()).unwrap();
        assert_eq!(artifact.conformance(), ReportPdfAConformance::PdfA2b);
        assert_eq!(artifact.bytes().get(..8), Some(b"%PDF-1.7".as_slice()));

        let parsed = ParsedPdf::load_mem(artifact.bytes()).unwrap();
        let catalog = catalog(&parsed);
        let metadata = dereference(&parsed, catalog.get(b"Metadata").unwrap())
            .as_stream()
            .unwrap();
        let xmp = String::from_utf8_lossy(&metadata.content);
        assert!(xmp.contains("<pdfaid:part>2</pdfaid:part>"), "{xmp}");
        assert!(
            xmp.contains("<pdfaid:conformance>B</pdfaid:conformance>"),
            "{xmp}"
        );
        assert!(xmp.contains("2026-07-17T14:30:00+00:00"), "{xmp}");

        let output_intents = dereference(&parsed, catalog.get(b"OutputIntents").unwrap())
            .as_array()
            .unwrap();
        assert_eq!(output_intents.len(), 1);
        let intent = dereference(&parsed, &output_intents[0]).as_dict().unwrap();
        assert_eq!(
            intent.get(b"S").unwrap(),
            &Object::Name(b"GTS_PDFA1".to_vec())
        );
        let profile = dereference(&parsed, intent.get(b"DestOutputProfile").unwrap())
            .as_stream()
            .unwrap();
        assert_eq!(profile.dict.get(b"N").unwrap().as_i64().unwrap(), 3);
        let profile_bytes = profile.decompressed_content().unwrap();
        assert_eq!(profile_bytes.get(36..40), Some(b"acsp".as_slice()));
    }

    #[test]
    fn writer_returns_only_after_validator_accepts_and_embeds_fonts() {
        let artifact = serialize_report_pdfa_2b(&prose_report(), &publication_options()).unwrap();
        let parsed = ParsedPdf::load_mem(artifact.bytes()).unwrap();
        let pages = parsed.get_pages();
        let text = parsed
            .extract_text(&pages.keys().copied().collect::<Vec<_>>())
            .unwrap();
        assert!(text.contains("Precision sensor AFE verification"));
        assert!(text.contains("All governed specifications passed"));
        let raw = String::from_utf8_lossy(artifact.bytes());
        assert!(raw.contains("/FontFile2"));
        assert!(raw.contains("IBMPlexSans"));

        let expected = ContentDigest::from_bytes(Sha256::digest(artifact.bytes()).into());
        assert_eq!(artifact.digest(), expected);
    }

    #[test]
    fn writer_embeds_authenticated_opaque_plot_artwork_and_visible_description() {
        let reference = ReportReferenceMode::Frozen {
            snapshot: reference_snapshot(),
            artifact: FrozenReportArtifact::new("image/png", encoded_png(png::ColorType::Rgb))
                .unwrap(),
        };
        let (report, _) = figure_report(reference);
        let artifact = serialize_report_pdfa_2b(&report, &publication_options()).unwrap();
        let parsed = ParsedPdf::load_mem(artifact.bytes()).unwrap();
        let pages = parsed.get_pages();
        let text = parsed
            .extract_text(&pages.keys().copied().collect::<Vec<_>>())
            .unwrap();
        assert!(text.contains("Closed-loop gain and phase"));
        assert!(text.contains("Gain remains above 20 dB"));
        assert!(String::from_utf8_lossy(artifact.bytes()).contains("/Subtype /Image"));

        let repeated = serialize_report_pdfa_2b(&report, &publication_options()).unwrap();
        assert_eq!(artifact, repeated);
    }

    #[test]
    fn plot_artwork_fails_closed_when_linked_unsupported_invalid_or_transparent() {
        let (linked, linked_id) = figure_report(ReportReferenceMode::Linked {
            snapshot: reference_snapshot(),
        });
        assert!(matches!(
            serialize_report_pdfa_2b(&linked, &publication_options()),
            Err(ReportPdfAError::UnresolvedPlotFigure { block_id }) if block_id == linked_id
        ));

        let (unsupported, unsupported_id) = figure_report(ReportReferenceMode::Frozen {
            snapshot: reference_snapshot(),
            artifact: FrozenReportArtifact::new("image/svg+xml", b"<svg/>".to_vec()).unwrap(),
        });
        assert!(matches!(
            serialize_report_pdfa_2b(&unsupported, &publication_options()),
            Err(ReportPdfAError::UnsupportedPlotFigureMediaType { block_id, .. })
                if block_id == unsupported_id
        ));

        let (invalid, invalid_id) = figure_report(ReportReferenceMode::Frozen {
            snapshot: reference_snapshot(),
            artifact: FrozenReportArtifact::new("image/png", b"not a PNG".to_vec()).unwrap(),
        });
        assert!(matches!(
            serialize_report_pdfa_2b(&invalid, &publication_options()),
            Err(ReportPdfAError::InvalidPlotFigureArtwork { block_id, .. })
                if block_id == invalid_id
        ));

        let (transparent, transparent_id) = figure_report(ReportReferenceMode::Frozen {
            snapshot: reference_snapshot(),
            artifact: FrozenReportArtifact::new("image/png", encoded_png(png::ColorType::Rgba))
                .unwrap(),
        });
        assert!(matches!(
            serialize_report_pdfa_2b(&transparent, &publication_options()),
            Err(ReportPdfAError::InvalidPlotFigureArtwork { block_id, detail })
                if block_id == transparent_id && detail.contains("alpha channel")
        ));
    }

    #[test]
    fn archival_validator_rejects_missing_required_document_date() {
        let configuration = ConfigurationBuilder::new()
            .with_archival_validator(Archival::A2_B)
            .finish()
            .unwrap();
        let mut document = Document::new_with(SerializeSettings {
            configuration,
            ..SerializeSettings::default()
        });
        document.set_metadata(Metadata::new().title("Missing date".to_owned()));
        let settings = PageSettings::from_wh(PAGE_WIDTH, PAGE_HEIGHT).unwrap();
        document.start_page_with(settings).finish();
        let error = document.finish().unwrap_err();
        let KrillaError::Validation(errors) = error else {
            panic!("expected PDF/A validation errors");
        };
        assert!(
            errors
                .iter()
                .any(|(error, _)| { *error == ValidationError::MissingDocumentDate })
        );
    }

    #[test]
    fn identical_inputs_are_byte_deterministic() {
        let report = prose_report();
        let options = publication_options();
        let first = serialize_report_pdfa_2b(&report, &options).unwrap();
        let second = serialize_report_pdfa_2b(&report, &options).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn strict_publication_date_rejects_invalid_calendar_values() {
        assert!(ReportPublicationDate::new(2025, 2, 29, 0, 0, 0).is_err());
        assert!(ReportPublicationDate::new(2024, 2, 29, 23, 59, 59).is_ok());
        assert!(ReportPublicationDate::new(2024, 13, 1, 0, 0, 0).is_err());
        assert!(ReportPublicationDate::new(10_000, 1, 1, 0, 0, 0).is_err());
    }

    #[test]
    fn empty_report_still_produces_a_real_cover_page() {
        let report = ReportDocument::new("Empty governed report").unwrap();
        let artifact = serialize_report_pdfa_2b(&report, &publication_options()).unwrap();
        let parsed = ParsedPdf::load_mem(artifact.bytes()).unwrap();
        assert_eq!(parsed.get_pages().len(), 1);
    }

    #[test]
    fn publication_workload_enforces_text_and_cell_budgets_without_allocation() {
        let mut text_workload = PublicationWorkload {
            source_text_bytes: MAX_REPORT_PDFA_SOURCE_TEXT_BYTES,
            ..PublicationWorkload::default()
        };
        assert!(matches!(
            text_workload.add_text("x"),
            Err(ReportPdfAError::ResourceLimit {
                scope: "source text bytes",
                maximum: MAX_REPORT_PDFA_SOURCE_TEXT_BYTES,
            })
        ));

        let mut table_workload = PublicationWorkload::default();
        assert!(matches!(
            table_workload.add_cells(MAX_REPORT_PDFA_TABLE_CELLS + 1),
            Err(ReportPdfAError::ResourceLimit {
                scope: "table cells",
                maximum: MAX_REPORT_PDFA_TABLE_CELLS,
            })
        ));

        let mut raster_workload = PublicationWorkload::default();
        assert!(
            raster_workload
                .add_raster_pixels(MAX_REPORT_PDFA_RASTER_PIXELS as u64)
                .is_ok()
        );
        assert!(matches!(
            raster_workload.add_raster_pixels(1),
            Err(ReportPdfAError::ResourceLimit {
                scope: "decoded raster pixels",
                maximum: MAX_REPORT_PDFA_RASTER_PIXELS,
            })
        ));
    }

    #[test]
    fn serialized_artifact_size_is_bounded() {
        assert!(validate_output_size(1).is_ok());
        assert!(validate_output_size(MAX_REPORT_PDFA_ARTIFACT_BYTES).is_ok());
        assert!(matches!(
            validate_output_size(MAX_REPORT_PDFA_ARTIFACT_BYTES + 1),
            Err(ReportPdfAError::ResourceLimit {
                scope: "serialized artifact bytes",
                maximum: MAX_REPORT_PDFA_ARTIFACT_BYTES,
            })
        ));
    }
}

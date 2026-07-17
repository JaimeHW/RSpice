//! Deterministic, UI-independent publication writers for report documents.
//!
//! Writers in this module perform no filesystem or browser operations. They
//! produce authenticated byte artifacts that the native and WASM front ends
//! can hand to their respective durable-publication services. A writer never
//! resolves a linked reference implicitly: the exact captured snapshot is
//! retained as provenance, while a frozen reference carries its authenticated
//! embedded payload.

use std::collections::HashSet;
use std::io::{self, Write as _};

use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use sha2::{Digest as _, Sha256};

use crate::product::{ContentDigest, ObjectRevision, ResultDocumentId};

use super::report_document::{
    DataTableBlock, FigureSizing, ProseStyle, ReportBlock, ReportBlockId, ReportBlockKind,
    ReportDocument, ReportError, ReportPageUpdatePolicy, ReportReferenceMode,
    ReportReferenceSnapshot, ReportSourceId, ReportTemplate, RequirementDisposition,
    ReviewNoteStatus, SpecificationDisposition, TableCell,
};

/// Maximum size of one materialized publication artifact.
///
/// The report domain permits larger archival frozen payload sets, but desktop
/// and browser publication is deliberately bounded so a single export cannot
/// exhaust a client process. Oversized reports fail before publication and can
/// be split by the caller without mutating their source document.
pub const MAX_REPORT_ARTIFACT_BYTES: usize = 64 * 1_048_576;

/// Maximum aggregate size returned by one selected-table CSV request.
pub const MAX_REPORT_CSV_BUNDLE_BYTES: usize = 64 * 1_048_576;

/// Maximum number of tables accepted by one CSV publication request.
pub const MAX_REPORT_CSV_TABLES: usize = 1_024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReportPublicationFormat {
    CanonicalJson,
    StandaloneHtml,
    TableCsv,
}

impl ReportPublicationFormat {
    #[must_use]
    pub const fn media_type(self) -> &'static str {
        match self {
            Self::CanonicalJson => "application/vnd.rspice.report+json",
            Self::StandaloneHtml => "text/html; charset=utf-8",
            Self::TableCsv => "text/csv; charset=utf-8",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishedReferenceMode {
    Linked,
    Frozen {
        artifact_media_type: String,
        artifact_digest: ContentDigest,
    },
}

/// Exact reference state associated with a publication artifact.
///
/// JSON and HTML carry this data in their bytes as well. CSV deliberately
/// keeps the tabular wire format free of non-standard preamble rows, so this
/// metadata is the authoritative side-channel used by the publication
/// manifest and release workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedReferenceProvenance {
    snapshot: ReportReferenceSnapshot,
    mode: PublishedReferenceMode,
}

impl PublishedReferenceProvenance {
    fn from_reference(reference: &ReportReferenceMode) -> Self {
        let mode = match reference {
            ReportReferenceMode::Linked { .. } => PublishedReferenceMode::Linked,
            ReportReferenceMode::Frozen { artifact, .. } => PublishedReferenceMode::Frozen {
                artifact_media_type: artifact.media_type().to_owned(),
                artifact_digest: artifact.content_digest(),
            },
        };
        Self {
            snapshot: reference.snapshot().clone(),
            mode,
        }
    }

    #[must_use]
    pub const fn snapshot(&self) -> &ReportReferenceSnapshot {
        &self.snapshot
    }

    #[must_use]
    pub const fn mode(&self) -> &PublishedReferenceMode {
        &self.mode
    }
}

/// Complete in-memory publication candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportPublicationArtifact {
    format: ReportPublicationFormat,
    file_name: String,
    bytes: Vec<u8>,
    content_digest: ContentDigest,
    document_id: ResultDocumentId,
    document_revision: ObjectRevision,
    block_id: Option<ReportBlockId>,
    reference: Option<PublishedReferenceProvenance>,
}

impl ReportPublicationArtifact {
    fn new(
        format: ReportPublicationFormat,
        file_name: String,
        bytes: Vec<u8>,
        document: &ReportDocument,
        block_id: Option<ReportBlockId>,
        reference: Option<PublishedReferenceProvenance>,
    ) -> Result<Self, ReportPublicationError> {
        if bytes.is_empty() || bytes.len() > MAX_REPORT_ARTIFACT_BYTES {
            return Err(ReportPublicationError::ResourceLimit {
                scope: "publication artifact",
                maximum_bytes: MAX_REPORT_ARTIFACT_BYTES,
            });
        }
        let content_digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
        Ok(Self {
            format,
            file_name,
            bytes,
            content_digest,
            document_id: document.id(),
            document_revision: document.revision(),
            block_id,
            reference,
        })
    }

    #[must_use]
    pub const fn format(&self) -> ReportPublicationFormat {
        self.format
    }

    #[must_use]
    pub const fn media_type(&self) -> &'static str {
        self.format.media_type()
    }

    #[must_use]
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }

    #[must_use]
    pub const fn document_id(&self) -> ResultDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn document_revision(&self) -> ObjectRevision {
        self.document_revision
    }

    #[must_use]
    pub const fn block_id(&self) -> Option<ReportBlockId> {
        self.block_id
    }

    #[must_use]
    pub const fn reference(&self) -> Option<&PublishedReferenceProvenance> {
        self.reference.as_ref()
    }
}

/// Serialize a validated report using its stable serde wire schema.
///
/// The wire contains no maps, uses fixed struct-field order, compact JSON,
/// lowercase UUIDs and lowercase SHA-256 digests. The writer verifies a full
/// domain round trip before returning the bytes.
pub fn publish_canonical_json(
    document: &ReportDocument,
) -> Result<ReportPublicationArtifact, ReportPublicationError> {
    document.validate()?;
    let mut writer = BoundedWriter::new(MAX_REPORT_ARTIFACT_BYTES);
    let result = serde_json::to_writer(&mut writer, document);
    if writer.limit_exceeded() {
        return Err(ReportPublicationError::ResourceLimit {
            scope: "canonical report JSON",
            maximum_bytes: MAX_REPORT_ARTIFACT_BYTES,
        });
    }
    result.map_err(ReportPublicationError::Json)?;
    let bytes = writer.finish();
    let restored: ReportDocument = serde_json::from_slice(&bytes)?;
    if restored != *document {
        return Err(ReportPublicationError::RoundTripMismatch);
    }
    ReportPublicationArtifact::new(
        ReportPublicationFormat::CanonicalJson,
        document_file_name(document, "json"),
        bytes,
        document,
        None,
        None,
    )
}

/// Render a self-contained, accessible HTML document.
///
/// All report text is escaped. Linked sources are identified as captured links
/// and never presented as embedded/current source data. Frozen artifacts are
/// embedded as authenticated `data:` resources; only inert raster formats are
/// rendered inline, while other media remain downloadable within the file.
pub fn publish_standalone_html(
    document: &ReportDocument,
) -> Result<ReportPublicationArtifact, ReportPublicationError> {
    document.validate()?;
    let mut html = HtmlWriter::new(MAX_REPORT_ARTIFACT_BYTES);
    render_html_document(&mut html, document)?;
    let bytes = html.finish()?;
    ReportPublicationArtifact::new(
        ReportPublicationFormat::StandaloneHtml,
        document_file_name(document, "html"),
        bytes,
        document,
        None,
        None,
    )
}

/// Export selected data-table blocks in document order as RFC 4180 CSV files.
///
/// Every requested identity must be unique, exist in the document and name a
/// data-table block. Returning one artifact per table avoids ambiguous column
/// schemas and keeps every CSV directly consumable by standard tools.
pub fn publish_selected_table_csv(
    document: &ReportDocument,
    selected_blocks: &[ReportBlockId],
) -> Result<Vec<ReportPublicationArtifact>, ReportPublicationError> {
    document.validate()?;
    if selected_blocks.is_empty() {
        return Err(ReportPublicationError::EmptyCsvSelection);
    }
    if selected_blocks.len() > MAX_REPORT_CSV_TABLES {
        return Err(ReportPublicationError::TooManyCsvTables {
            selected: selected_blocks.len(),
            maximum: MAX_REPORT_CSV_TABLES,
        });
    }
    let selected: HashSet<_> = selected_blocks.iter().copied().collect();
    if selected.len() != selected_blocks.len() {
        return Err(ReportPublicationError::DuplicateCsvSelection);
    }
    for block_id in selected_blocks {
        let block = document
            .block(*block_id)
            .ok_or(ReportPublicationError::BlockNotFound(*block_id))?;
        if !matches!(block.kind(), ReportBlockKind::DataTable(_)) {
            return Err(ReportPublicationError::BlockIsNotDataTable(*block_id));
        }
    }

    let mut artifacts = Vec::with_capacity(selected.len());
    let mut aggregate_bytes = 0_usize;
    for block in document
        .pages()
        .iter()
        .flat_map(|page| page.sections())
        .flat_map(|section| section.blocks())
        .filter(|block| selected.contains(&block.id()))
    {
        let ReportBlockKind::DataTable(table) = block.kind() else {
            return Err(ReportPublicationError::BlockIsNotDataTable(block.id()));
        };
        let bytes = render_csv_table(table)?;
        aggregate_bytes = aggregate_bytes.checked_add(bytes.len()).ok_or(
            ReportPublicationError::ResourceLimit {
                scope: "selected table CSV bundle",
                maximum_bytes: MAX_REPORT_CSV_BUNDLE_BYTES,
            },
        )?;
        if aggregate_bytes > MAX_REPORT_CSV_BUNDLE_BYTES {
            return Err(ReportPublicationError::ResourceLimit {
                scope: "selected table CSV bundle",
                maximum_bytes: MAX_REPORT_CSV_BUNDLE_BYTES,
            });
        }
        artifacts.push(ReportPublicationArtifact::new(
            ReportPublicationFormat::TableCsv,
            table_file_name(document, block.id()),
            bytes,
            document,
            Some(block.id()),
            Some(PublishedReferenceProvenance::from_reference(
                &table.reference,
            )),
        )?);
    }
    debug_assert_eq!(artifacts.len(), selected.len());
    Ok(artifacts)
}

fn document_file_name(document: &ReportDocument, extension: &str) -> String {
    format!(
        "rspice-report-{}-r{}.{}",
        document.id(),
        document.revision().get(),
        extension
    )
}

fn table_file_name(document: &ReportDocument, block_id: ReportBlockId) -> String {
    format!(
        "rspice-report-{}-r{}-table-{}.csv",
        document.id(),
        document.revision().get(),
        block_id
    )
}

fn render_csv_table(table: &DataTableBlock) -> Result<Vec<u8>, ReportPublicationError> {
    let mut csv = BoundedWriter::new(MAX_REPORT_ARTIFACT_BYTES);
    for (index, column) in table.columns.iter().enumerate() {
        if index > 0 {
            csv.write_all(b",").map_err(map_bounded_io)?;
        }
        let heading = match &column.unit {
            Some(unit) => format!("{} [{}]", column.heading, unit),
            None => column.heading.clone(),
        };
        write_csv_field(&mut csv, &heading)?;
    }
    csv.write_all(b"\r\n").map_err(map_bounded_io)?;
    for row in &table.rows {
        for (index, cell) in row.iter().enumerate() {
            if index > 0 {
                csv.write_all(b",").map_err(map_bounded_io)?;
            }
            match cell {
                TableCell::Empty => {}
                TableCell::Text(value) => write_csv_field(&mut csv, value)?,
                TableCell::Number { value, unit } => {
                    let value = match (unit, table.columns[index].unit.as_ref()) {
                        (Some(cell_unit), Some(column_unit)) if cell_unit != column_unit => {
                            format!("{value} {cell_unit}")
                        }
                        (Some(cell_unit), None) => format!("{value} {cell_unit}"),
                        _ => value.to_string(),
                    };
                    write_csv_field(&mut csv, &value)?;
                }
                TableCell::Integer(value) => write_csv_field(&mut csv, &value.to_string())?,
                TableCell::Boolean(value) => {
                    write_csv_field(&mut csv, if *value { "true" } else { "false" })?;
                }
            }
        }
        csv.write_all(b"\r\n").map_err(map_bounded_io)?;
    }
    if csv.limit_exceeded() {
        return Err(ReportPublicationError::ResourceLimit {
            scope: "table CSV",
            maximum_bytes: MAX_REPORT_ARTIFACT_BYTES,
        });
    }
    Ok(csv.finish())
}

fn write_csv_field(writer: &mut BoundedWriter, value: &str) -> Result<(), ReportPublicationError> {
    let quote = value
        .bytes()
        .any(|byte| matches!(byte, b',' | b'"' | b'\r' | b'\n'));
    if quote {
        writer.write_all(b"\"").map_err(map_bounded_io)?;
        for part in value.split_inclusive('"') {
            writer.write_all(part.as_bytes()).map_err(map_bounded_io)?;
            if part.ends_with('"') {
                writer.write_all(b"\"").map_err(map_bounded_io)?;
            }
        }
        writer.write_all(b"\"").map_err(map_bounded_io)?;
    } else {
        writer.write_all(value.as_bytes()).map_err(map_bounded_io)?;
    }
    Ok(())
}

fn render_html_document(
    html: &mut HtmlWriter,
    document: &ReportDocument,
) -> Result<(), ReportPublicationError> {
    html.raw("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">")?;
    html.raw("<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">")?;
    html.raw("<meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; img-src data:; style-src 'unsafe-inline'\">")?;
    html.raw("<title>")?;
    html.text(document.title())?;
    html.raw(" · RSpice report</title><style>")?;
    html.raw(HTML_STYLE)?;
    html.raw(
        "</style></head><body><header><p class=\"eyebrow\">RSPICE ENGINEERING REPORT</p><h1>",
    )?;
    html.text(document.title())?;
    html.raw("</h1><dl class=\"metadata\"><div><dt>Document</dt><dd><code>")?;
    html.text(&document.id().to_string())?;
    html.raw("</code></dd></div><div><dt>Revision</dt><dd>")?;
    html.text(&document.revision().get().to_string())?;
    html.raw("</dd></div><div><dt>Template</dt><dd>")?;
    html.text(template_label(document.template()))?;
    html.raw("</dd></div></dl></header><main>")?;

    if document.pages().is_empty() {
        html.raw("<p class=\"empty\">This report revision contains no pages.</p>")?;
    }
    for (page_index, page) in document.pages().iter().enumerate() {
        let page_heading_id = format!("page-{}-heading", page.id());
        html.raw("<article class=\"report-page\" aria-labelledby=\"")?;
        html.attr(&page_heading_id)?;
        html.raw("\"><header><p class=\"page-number\">Page ")?;
        html.text(&(page_index + 1).to_string())?;
        html.raw("</p><h2 id=\"")?;
        html.attr(&page_heading_id)?;
        html.raw("\">")?;
        html.text(page.title())?;
        html.raw("</h2><p class=\"policy\">Reference policy: ")?;
        html.text(page_policy_label(page.update_policy()))?;
        html.raw("</p></header>")?;
        if page.sections().is_empty() {
            html.raw("<p class=\"empty\">This page contains no sections.</p>")?;
        }
        for section in page.sections() {
            let section_heading_id = format!("section-{}-heading", section.id());
            html.raw("<section aria-labelledby=\"")?;
            html.attr(&section_heading_id)?;
            html.raw("\"><h3 id=\"")?;
            html.attr(&section_heading_id)?;
            html.raw("\">")?;
            html.text(section.title())?;
            html.raw("</h3>")?;
            if section.blocks().is_empty() {
                html.raw("<p class=\"empty\">This section contains no content blocks.</p>")?;
            }
            for block in section.blocks() {
                render_html_block(html, block)?;
            }
            html.raw("</section>")?;
        }
        html.raw("</article>")?;
    }
    html.raw("</main><footer><p>Generated from immutable RSpice report document <code>")?;
    html.text(&document.id().to_string())?;
    html.raw("</code>, revision ")?;
    html.text(&document.revision().get().to_string())?;
    html.raw(".</p></footer></body></html>")
}

fn render_html_block(
    html: &mut HtmlWriter,
    block: &ReportBlock,
) -> Result<(), ReportPublicationError> {
    html.raw("<div class=\"report-block\" id=\"block-")?;
    html.attr(&block.id().to_string())?;
    html.raw("\">")?;
    match block.kind() {
        ReportBlockKind::PlotFigure(figure) => {
            html.raw("<figure><div class=\"figure-content\">")?;
            if let ReportReferenceMode::Frozen { artifact, .. } = &figure.reference {
                if is_safe_inline_raster(artifact.media_type()) {
                    html.raw("<img src=\"data:")?;
                    html.attr(artifact.media_type())?;
                    html.raw(";base64,")?;
                    html.base64(artifact.payload())?;
                    html.raw("\" alt=\"")?;
                    html.attr(&figure.alternative_text)?;
                    html.raw("\">")?;
                } else {
                    html.raw("<p class=\"alternative-text\">")?;
                    html.text(&figure.alternative_text)?;
                    html.raw("</p>")?;
                    render_frozen_download(html, artifact.media_type(), artifact.payload())?;
                }
            } else {
                html.raw("<p class=\"alternative-text\">")?;
                html.text(&figure.alternative_text)?;
                html.raw("</p>")?;
            }
            html.raw("</div><figcaption>")?;
            html.text(&figure.caption)?;
            html.raw(" <span class=\"sizing\">(")?;
            html.text(figure_sizing_label(figure.sizing))?;
            html.raw(")</span></figcaption></figure>")?;
            render_reference(html, &figure.reference, false)?;
        }
        ReportBlockKind::DataTable(table) => {
            html.raw("<div class=\"table-scroll\"><table><caption>")?;
            html.text(&table.title)?;
            html.raw("</caption><thead><tr>")?;
            for column in &table.columns {
                html.raw("<th scope=\"col\">")?;
                html.text(&column.heading)?;
                if let Some(unit) = &column.unit {
                    html.raw(" <span class=\"unit\">[")?;
                    html.text(unit)?;
                    html.raw("]</span>")?;
                }
                html.raw("</th>")?;
            }
            html.raw("</tr></thead><tbody>")?;
            for row in &table.rows {
                html.raw("<tr>")?;
                for cell in row {
                    html.raw("<td>")?;
                    render_html_table_cell(html, cell)?;
                    html.raw("</td>")?;
                }
                html.raw("</tr>")?;
            }
            html.raw("</tbody></table></div>")?;
            render_reference(html, &table.reference, true)?;
        }
        ReportBlockKind::Datasheet(datasheet) => {
            html.raw("<h4>")?;
            html.text(&datasheet.title)?;
            html.raw("</h4><dl class=\"datasheet\">")?;
            for field in &datasheet.fields {
                html.raw("<div><dt>")?;
                html.text(&field.label)?;
                html.raw("</dt><dd>")?;
                html.text(&field.value)?;
                if let Some(unit) = &field.unit {
                    html.raw(" <span class=\"unit\">")?;
                    html.text(unit)?;
                    html.raw("</span>")?;
                }
                html.raw("</dd></div>")?;
            }
            html.raw("</dl>")?;
            render_reference(html, &datasheet.reference, true)?;
        }
        ReportBlockKind::Requirements(requirements) => {
            html.raw("<div class=\"table-scroll\"><table><caption>")?;
            html.text(&requirements.title)?;
            html.raw("</caption><thead><tr><th scope=\"col\">Requirement</th><th scope=\"col\">Statement</th><th scope=\"col\">Disposition</th><th scope=\"col\">Evidence</th></tr></thead><tbody>")?;
            for entry in &requirements.entries {
                html.raw("<tr><th scope=\"row\"><code>")?;
                html.text(&entry.requirement_id)?;
                html.raw("</code></th><td>")?;
                html.text(&entry.statement)?;
                html.raw("</td><td>")?;
                html.text(requirement_disposition_label(entry.disposition))?;
                html.raw("</td><td>")?;
                if let Some(label) = &entry.evidence_label {
                    html.text(label)?;
                } else {
                    html.raw("<span class=\"not-provided\">Not provided</span>")?;
                }
                html.raw("</td></tr>")?;
            }
            html.raw("</tbody></table></div>")?;
            render_reference(html, &requirements.reference, true)?;
        }
        ReportBlockKind::Specifications(specifications) => {
            html.raw("<div class=\"table-scroll\"><table><caption>")?;
            html.text(&specifications.title)?;
            html.raw("</caption><thead><tr><th scope=\"col\">Expression</th><th scope=\"col\">Limit</th><th scope=\"col\">Measured</th><th scope=\"col\">Disposition</th></tr></thead><tbody>")?;
            for entry in &specifications.entries {
                html.raw("<tr><th scope=\"row\"><code>")?;
                html.text(&entry.expression)?;
                html.raw("</code></th><td>")?;
                html.text(&entry.limit)?;
                html.raw("</td><td>")?;
                if let Some(measured) = &entry.measured {
                    html.text(measured)?;
                } else {
                    html.raw("<span class=\"not-provided\">Not evaluated</span>")?;
                }
                html.raw("</td><td>")?;
                html.text(specification_disposition_label(entry.disposition))?;
                html.raw("</td></tr>")?;
            }
            html.raw("</tbody></table></div>")?;
            render_reference(html, &specifications.reference, true)?;
        }
        ReportBlockKind::Prose(prose) => {
            html.raw("<div class=\"prose prose-")?;
            html.attr(prose_style_class(prose.style))?;
            html.raw("\"><p class=\"content-label\">")?;
            html.text(prose_style_label(prose.style))?;
            html.raw("</p><pre>")?;
            html.text(&prose.markdown)?;
            html.raw("</pre></div>")?;
        }
        ReportBlockKind::ReviewNote(note) => {
            html.raw("<aside class=\"review-note\" aria-label=\"Review note\"><p><strong>")?;
            html.text(&note.author)?;
            html.raw("</strong> · ")?;
            html.text(review_status_label(note.status))?;
            html.raw("</p><p>")?;
            html.text(&note.message)?;
            html.raw("</p><p class=\"timestamp\">Created at Unix time ")?;
            html.text(&note.created_at_unix_ms.to_string())?;
            html.raw(" ms")?;
            if let Some(resolved) = note.resolved_at_unix_ms {
                html.raw("; resolved at Unix time ")?;
                html.text(&resolved.to_string())?;
                html.raw(" ms")?;
            }
            html.raw(".</p></aside>")?;
        }
        ReportBlockKind::Evidence(evidence) => {
            html.raw("<h4>")?;
            html.text(&evidence.title)?;
            html.raw("</h4><p>")?;
            html.text(&evidence.summary)?;
            html.raw("</p>")?;
            render_reference(html, &evidence.reference, true)?;
        }
    }
    html.raw("</div>")
}

fn render_html_table_cell(
    html: &mut HtmlWriter,
    cell: &TableCell,
) -> Result<(), ReportPublicationError> {
    match cell {
        TableCell::Empty => html.raw("<span class=\"not-provided\">—</span>"),
        TableCell::Text(value) => html.text(value),
        TableCell::Number { value, unit } => {
            html.text(&value.to_string())?;
            if let Some(unit) = unit {
                html.raw(" <span class=\"unit\">")?;
                html.text(unit)?;
                html.raw("</span>")?;
            }
            Ok(())
        }
        TableCell::Integer(value) => html.text(&value.to_string()),
        TableCell::Boolean(value) => html.text(if *value { "true" } else { "false" }),
    }
}

fn render_reference(
    html: &mut HtmlWriter,
    reference: &ReportReferenceMode,
    embed_frozen_download: bool,
) -> Result<(), ReportPublicationError> {
    let snapshot = reference.snapshot();
    html.raw("<aside class=\"reference\" aria-label=\"Source reference\">")?;
    match reference {
        ReportReferenceMode::Linked { .. } => html.raw("<p class=\"reference-status linked\"><strong>Linked reference.</strong> The displayed report content is captured in this document revision; the external source artifact is not embedded and is not claimed to be current.</p>")?,
        ReportReferenceMode::Frozen { artifact, .. } => {
            html.raw("<p class=\"reference-status frozen\"><strong>Frozen reference.</strong> The exact embedded artifact is authenticated by SHA-256 and does not follow later source changes.</p>")?;
            if embed_frozen_download {
                render_frozen_download(html, artifact.media_type(), artifact.payload())?;
            }
        }
    }
    html.raw("<dl class=\"reference-metadata\"><div><dt>Source</dt><dd><code>")?;
    render_source_id(html, &snapshot.source)?;
    html.raw("</code></dd></div><div><dt>Captured source revision</dt><dd>")?;
    match snapshot.source_revision {
        Some(revision) => html.text(&revision.get().to_string())?,
        None => html.raw("Not revisioned")?,
    }
    html.raw("</dd></div><div><dt>Captured content SHA-256</dt><dd><code>")?;
    html.text(&snapshot.content_digest.to_string())?;
    html.raw("</code></dd></div>")?;
    if let ReportReferenceMode::Frozen { artifact, .. } = reference {
        html.raw("<div><dt>Embedded media type</dt><dd><code>")?;
        html.text(artifact.media_type())?;
        html.raw("</code></dd></div><div><dt>Embedded artifact SHA-256</dt><dd><code>")?;
        html.text(&artifact.content_digest().to_string())?;
        html.raw("</code></dd></div>")?;
    }
    html.raw("</dl>")?;
    if !snapshot.dataset_bindings.is_empty() {
        html.raw("<details><summary>Immutable dataset bindings (")?;
        html.text(&snapshot.dataset_bindings.len().to_string())?;
        html.raw(")</summary><ul class=\"bindings\">")?;
        for binding in &snapshot.dataset_bindings {
            html.raw("<li><code>")?;
            html.text(&binding.dataset_id.to_string())?;
            html.raw("</code> · SHA-256 <code>")?;
            html.text(&binding.content_digest.to_string())?;
            html.raw("</code></li>")?;
        }
        html.raw("</ul></details>")?;
    }
    html.raw("</aside>")
}

fn render_source_id(
    html: &mut HtmlWriter,
    source: &ReportSourceId,
) -> Result<(), ReportPublicationError> {
    match source {
        ReportSourceId::VisualizationDocument { document_id } => {
            html.text("visualization-document:")?;
            html.text(&document_id.to_string())
        }
        ReportSourceId::Dataset { dataset_id } => {
            html.text("dataset:")?;
            html.text(&dataset_id.to_string())
        }
        ReportSourceId::VerificationEvidence { evidence_id } => {
            html.text("verification-evidence:")?;
            html.text(&evidence_id.to_string())
        }
        ReportSourceId::ExternalRecord { namespace, key } => {
            html.text("external-record:")?;
            html.text(namespace)?;
            html.text(":")?;
            html.text(key)
        }
    }
}

fn render_frozen_download(
    html: &mut HtmlWriter,
    media_type: &str,
    payload: &[u8],
) -> Result<(), ReportPublicationError> {
    html.raw("<p><a class=\"embedded-artifact\" download href=\"data:")?;
    html.attr(media_type)?;
    html.raw(";base64,")?;
    html.base64(payload)?;
    html.raw("\">Open embedded frozen artifact (")?;
    html.text(media_type)?;
    html.raw(")</a></p>")
}

fn is_safe_inline_raster(media_type: &str) -> bool {
    matches!(
        media_type,
        "image/png" | "image/jpeg" | "image/gif" | "image/webp"
    )
}

const fn template_label(template: ReportTemplate) -> &'static str {
    match template {
        ReportTemplate::ReleaseVerification42 => "Release verification 4.2",
        ReportTemplate::DesignReview => "Design review",
        ReportTemplate::ModelQualification => "Model qualification",
    }
}

const fn page_policy_label(policy: ReportPageUpdatePolicy) -> &'static str {
    match policy {
        ReportPageUpdatePolicy::RefreshLinkedAutomatically => "refresh linked references",
        ReportPageUpdatePolicy::FreezeSelectedRevision => "freeze selected revision",
    }
}

const fn figure_sizing_label(sizing: FigureSizing) -> &'static str {
    match sizing {
        FigureSizing::FitWidth => "fit width",
        FigureSizing::FitPage => "fit page",
        FigureSizing::Natural => "natural size",
    }
}

const fn requirement_disposition_label(disposition: RequirementDisposition) -> &'static str {
    match disposition {
        RequirementDisposition::NotEvaluated => "Not evaluated",
        RequirementDisposition::Passed => "Passed",
        RequirementDisposition::Failed => "Failed",
        RequirementDisposition::Waived => "Waived",
    }
}

const fn specification_disposition_label(disposition: SpecificationDisposition) -> &'static str {
    match disposition {
        SpecificationDisposition::NotEvaluated => "Not evaluated",
        SpecificationDisposition::InSpecification => "In specification",
        SpecificationDisposition::OutOfSpecification => "Out of specification",
        SpecificationDisposition::Informational => "Informational",
    }
}

const fn prose_style_class(style: ProseStyle) -> &'static str {
    match style {
        ProseStyle::Body => "body",
        ProseStyle::ExecutiveSummary => "executive-summary",
        ProseStyle::Method => "method",
        ProseStyle::Conclusion => "conclusion",
        ProseStyle::Warning => "warning",
    }
}

const fn prose_style_label(style: ProseStyle) -> &'static str {
    match style {
        ProseStyle::Body => "Narrative",
        ProseStyle::ExecutiveSummary => "Executive summary",
        ProseStyle::Method => "Method",
        ProseStyle::Conclusion => "Conclusion",
        ProseStyle::Warning => "Warning",
    }
}

const fn review_status_label(status: ReviewNoteStatus) -> &'static str {
    match status {
        ReviewNoteStatus::Open => "Open",
        ReviewNoteStatus::Addressed => "Addressed",
        ReviewNoteStatus::Accepted => "Accepted",
    }
}

const HTML_STYLE: &str = r#":root{color-scheme:light;--ink:#172129;--muted:#52606a;--line:#c9d0d5;--panel:#f4f6f7;--accent:#a86f00}*{box-sizing:border-box}body{max-width:1120px;margin:0 auto;padding:2rem;font:15px/1.5 system-ui,-apple-system,"Segoe UI",sans-serif;color:var(--ink);background:#fff}header,footer,main{width:100%}h1,h2,h3,h4{line-height:1.2}.eyebrow,.page-number,.content-label{font-size:.75rem;font-weight:700;letter-spacing:.08em;color:var(--muted)}.metadata,.reference-metadata,.datasheet{display:grid;gap:.35rem}.metadata>div,.reference-metadata>div,.datasheet>div{display:grid;grid-template-columns:minmax(10rem,15rem) 1fr;gap:1rem}.metadata dt,.reference-metadata dt,.datasheet dt{font-weight:600;color:var(--muted)}dd{margin:0}.report-page{margin:2rem 0;padding:1.5rem;border:1px solid var(--line);break-after:page}.report-page:last-child{break-after:auto}.policy,.empty,.not-provided,.unit,.sizing,.timestamp{color:var(--muted)}section{margin-top:1.5rem}.report-block{margin:1rem 0;padding:1rem;border-left:3px solid var(--line);background:var(--panel)}table{width:100%;border-collapse:collapse;background:#fff}caption{text-align:left;font-weight:700;padding:.5rem 0}th,td{padding:.55rem;text-align:left;vertical-align:top;border:1px solid var(--line)}.table-scroll{overflow-x:auto}.reference,.review-note{margin-top:1rem;padding:.75rem;border:1px solid var(--line);background:#fff}.reference-status{margin-top:0}.linked strong{color:#7b5700}.frozen strong{color:#17653a}.bindings{overflow-wrap:anywhere}.embedded-artifact{overflow-wrap:anywhere}.prose pre{margin:0;white-space:pre-wrap;overflow-wrap:anywhere;font:inherit}.prose-warning{border-left:3px solid #b2382f;padding-left:.75rem}.alternative-text{padding:2rem;border:1px dashed var(--line);background:#fff}.figure-content img{display:block;max-width:100%;height:auto}code{overflow-wrap:anywhere}footer{padding-top:1rem;border-top:1px solid var(--line);color:var(--muted)}@media(max-width:640px){body{padding:.75rem}.report-page{padding:.85rem}.metadata>div,.reference-metadata>div,.datasheet>div{grid-template-columns:1fr;gap:0}}@media print{body{max-width:none;padding:0}.report-page{border:0;padding:0}.embedded-artifact{display:none}}"#;

#[derive(Debug)]
struct BoundedWriter {
    bytes: Vec<u8>,
    maximum: usize,
    limit_exceeded: bool,
}

impl BoundedWriter {
    fn new(maximum: usize) -> Self {
        Self {
            bytes: Vec::new(),
            maximum,
            limit_exceeded: false,
        }
    }

    const fn limit_exceeded(&self) -> bool {
        self.limit_exceeded
    }

    fn finish(self) -> Vec<u8> {
        self.bytes
    }
}

impl io::Write for BoundedWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        let Some(required) = self.bytes.len().checked_add(buffer.len()) else {
            self.limit_exceeded = true;
            return Err(io::Error::other("publication output size overflow"));
        };
        if required > self.maximum {
            self.limit_exceeded = true;
            return Err(io::Error::other("publication output limit exceeded"));
        }
        if self.bytes.try_reserve(buffer.len()).is_err() {
            self.limit_exceeded = true;
            return Err(io::Error::other("publication output allocation failed"));
        }
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

struct HtmlWriter {
    output: BoundedWriter,
}

impl HtmlWriter {
    fn new(maximum: usize) -> Self {
        Self {
            output: BoundedWriter::new(maximum),
        }
    }

    fn raw(&mut self, value: &str) -> Result<(), ReportPublicationError> {
        self.output
            .write_all(value.as_bytes())
            .map_err(map_bounded_io)
    }

    fn text(&mut self, value: &str) -> Result<(), ReportPublicationError> {
        self.escaped(value)
    }

    fn attr(&mut self, value: &str) -> Result<(), ReportPublicationError> {
        self.escaped(value)
    }

    fn escaped(&mut self, value: &str) -> Result<(), ReportPublicationError> {
        let mut start = 0;
        for (index, character) in value.char_indices() {
            let replacement = match character {
                '&' => "&amp;",
                '<' => "&lt;",
                '>' => "&gt;",
                '"' => "&quot;",
                '\'' => "&#39;",
                _ => continue,
            };
            self.raw(&value[start..index])?;
            self.raw(replacement)?;
            start = index + character.len_utf8();
        }
        self.raw(&value[start..])
    }

    fn base64(&mut self, payload: &[u8]) -> Result<(), ReportPublicationError> {
        let mut encoder = base64::write::EncoderWriter::new(&mut self.output, &BASE64_STANDARD);
        encoder.write_all(payload).map_err(map_bounded_io)?;
        encoder.finish().map_err(map_bounded_io)?;
        Ok(())
    }

    fn finish(self) -> Result<Vec<u8>, ReportPublicationError> {
        if self.output.limit_exceeded() {
            return Err(ReportPublicationError::ResourceLimit {
                scope: "standalone report HTML",
                maximum_bytes: MAX_REPORT_ARTIFACT_BYTES,
            });
        }
        Ok(self.output.finish())
    }
}

fn map_bounded_io(_: io::Error) -> ReportPublicationError {
    ReportPublicationError::ResourceLimit {
        scope: "publication output",
        maximum_bytes: MAX_REPORT_ARTIFACT_BYTES,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReportPublicationError {
    #[error(transparent)]
    InvalidDocument(#[from] ReportError),
    #[error("failed to serialize or restore canonical report JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("canonical report JSON did not restore the exact source document")]
    RoundTripMismatch,
    #[error("{scope} exceeds the {maximum_bytes}-byte publication limit")]
    ResourceLimit {
        scope: &'static str,
        maximum_bytes: usize,
    },
    #[error("at least one data-table block must be selected for CSV publication")]
    EmptyCsvSelection,
    #[error("CSV publication selected the same block more than once")]
    DuplicateCsvSelection,
    #[error("CSV publication selected {selected} tables; the maximum is {maximum}")]
    TooManyCsvTables { selected: usize, maximum: usize },
    #[error("selected report block {0} does not exist")]
    BlockNotFound(ReportBlockId),
    #[error("selected report block {0} is not a data table")]
    BlockIsNotDataTable(ReportBlockId),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr as _;

    use uuid::Uuid;

    use crate::product::{DatasetBinding, DatasetId};

    use super::super::report_document::{
        FrozenReportArtifact, ProseBlock, ReportEdit, TableColumn,
    };
    use super::*;

    fn digest(seed: u8) -> ContentDigest {
        ContentDigest::from_bytes([seed; 32])
    }

    fn external_reference(frozen: bool) -> ReportReferenceMode {
        let snapshot = ReportReferenceSnapshot::new(
            ReportSourceId::ExternalRecord {
                namespace: "lab".to_owned(),
                key: "run-42".to_owned(),
            },
            Some(ObjectRevision::new(7).unwrap()),
            digest(9),
            vec![DatasetBinding::new(DatasetId::new(), digest(10))],
        )
        .unwrap();
        if frozen {
            ReportReferenceMode::Frozen {
                snapshot,
                artifact: FrozenReportArtifact::new(
                    "application/octet-stream",
                    b"exact frozen bytes".to_vec(),
                )
                .unwrap(),
            }
        } else {
            ReportReferenceMode::Linked { snapshot }
        }
    }

    fn report_with_blocks(blocks: Vec<ReportBlockKind>) -> ReportDocument {
        let mut report = ReportDocument::new("Precision <AFE> & review").unwrap();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddPage {
                    title: "Results & sign-off".to_owned(),
                }],
                1,
            )
            .unwrap();
        let page_id = report.pages()[0].id();
        report
            .transact(
                report.revision(),
                vec![ReportEdit::AddSection {
                    page_id,
                    title: "Measured <results>".to_owned(),
                }],
                2,
            )
            .unwrap();
        let section_id = report.pages()[0].sections()[0].id();
        report
            .transact(
                report.revision(),
                blocks
                    .into_iter()
                    .map(|kind| ReportEdit::AddBlock { section_id, kind })
                    .collect(),
                3,
            )
            .unwrap();
        report
    }

    fn data_table(title: &str, reference: ReportReferenceMode) -> ReportBlockKind {
        ReportBlockKind::DataTable(DataTableBlock {
            title: title.to_owned(),
            columns: vec![
                TableColumn {
                    key: "label".to_owned(),
                    heading: "Label".to_owned(),
                    unit: None,
                },
                TableColumn {
                    key: "voltage".to_owned(),
                    heading: "Voltage".to_owned(),
                    unit: Some("V".to_owned()),
                },
            ],
            rows: vec![
                vec![
                    TableCell::Text("comma, quote \" and line".to_owned()),
                    TableCell::Number {
                        value: -0.000_125,
                        unit: Some("V".to_owned()),
                    },
                ],
                vec![TableCell::Empty, TableCell::Boolean(true)],
            ],
            reference,
        })
    }

    #[test]
    fn canonical_json_matches_stable_fixture_and_round_trips() {
        let fixture = r#"{"schema_version":2,"id":"8e125f25-73b3-4b13-9d50-77c66f9b6a9a","revision":1,"title":"Fixture report","template":"release-verification42","pages":[],"receipts":[],"tombstones":[],"legacy_origin_entities":[]}"#;
        let report: ReportDocument = serde_json::from_str(fixture).unwrap();
        let artifact = publish_canonical_json(&report).unwrap();

        assert_eq!(artifact.bytes(), fixture.as_bytes());
        assert_eq!(artifact.media_type(), "application/vnd.rspice.report+json");
        assert_eq!(
            artifact.file_name(),
            "rspice-report-8e125f25-73b3-4b13-9d50-77c66f9b6a9a-r1.json"
        );
        assert_eq!(
            artifact.content_digest(),
            ContentDigest::from_bytes(Sha256::digest(fixture.as_bytes()).into())
        );
        let restored: ReportDocument = serde_json::from_slice(artifact.bytes()).unwrap();
        assert_eq!(restored, report);
    }

    #[test]
    fn canonical_and_html_publication_are_byte_deterministic() {
        let report = report_with_blocks(vec![ReportBlockKind::Prose(ProseBlock {
            style: ProseStyle::ExecutiveSummary,
            markdown: "Exact result & interpretation.".to_owned(),
        })]);
        assert_eq!(
            publish_canonical_json(&report).unwrap().bytes(),
            publish_canonical_json(&report).unwrap().bytes()
        );
        assert_eq!(
            publish_standalone_html(&report).unwrap().bytes(),
            publish_standalone_html(&report).unwrap().bytes()
        );
    }

    #[test]
    fn html_escapes_untrusted_content_and_states_link_truthfully() {
        let report = report_with_blocks(vec![
            ReportBlockKind::Prose(ProseBlock {
                style: ProseStyle::Body,
                markdown: "<script>alert(\"x\")</script> & 'quoted'".to_owned(),
            }),
            data_table("Measurements <unsafe>", external_reference(false)),
        ]);
        let artifact = publish_standalone_html(&report).unwrap();
        let html = std::str::from_utf8(artifact.bytes()).unwrap();

        assert!(html.starts_with("<!doctype html><html lang=\"en\">"));
        assert!(html.contains("Precision &lt;AFE&gt; &amp; review"));
        assert!(
            html.contains(
                "&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt; &amp; &#39;quoted&#39;"
            )
        );
        assert!(!html.contains("<script>"));
        assert!(html.contains("<th scope=\"col\">"));
        assert!(html.contains("<strong>Linked reference.</strong>"));
        assert!(html.contains(
            "the external source artifact is not embedded and is not claimed to be current"
        ));
        assert!(html.contains(&digest(9).to_string()));
    }

    #[test]
    fn html_embeds_and_authenticates_frozen_payload_without_scriptable_inline_media() {
        let report = report_with_blocks(vec![data_table(
            "Frozen measurements",
            external_reference(true),
        )]);
        let artifact = publish_standalone_html(&report).unwrap();
        let html = std::str::from_utf8(artifact.bytes()).unwrap();

        assert!(html.contains("<strong>Frozen reference.</strong>"));
        assert!(html.contains("data:application/octet-stream;base64,ZXhhY3QgZnJvemVuIGJ5dGVz"));
        assert!(html.contains("Embedded artifact SHA-256"));
        assert!(!html.contains("<iframe"));
        assert!(!html.contains("<object"));
    }

    #[test]
    fn selected_csv_is_rfc4180_exact_and_carries_reference_provenance() {
        let report =
            report_with_blocks(vec![data_table("Measurements", external_reference(false))]);
        let block_id = report.pages()[0].sections()[0].blocks()[0].id();
        let artifacts = publish_selected_table_csv(&report, &[block_id]).unwrap();
        let artifact = &artifacts[0];

        assert_eq!(artifact.media_type(), "text/csv; charset=utf-8");
        assert_eq!(artifact.block_id(), Some(block_id));
        assert_eq!(
            std::str::from_utf8(artifact.bytes()).unwrap(),
            "Label,Voltage [V]\r\n\"comma, quote \"\" and line\",-0.000125\r\n,true\r\n"
        );
        let provenance = artifact.reference().unwrap();
        assert_eq!(provenance.snapshot().content_digest, digest(9));
        assert_eq!(provenance.mode(), &PublishedReferenceMode::Linked);
    }

    #[test]
    fn csv_never_silently_drops_cell_level_units() {
        let table = DataTableBlock {
            title: "Heterogeneous readings".to_owned(),
            columns: vec![TableColumn {
                key: "reading".to_owned(),
                heading: "Reading".to_owned(),
                unit: Some("V".to_owned()),
            }],
            rows: vec![
                vec![TableCell::Number {
                    value: 1.25,
                    unit: Some("V".to_owned()),
                }],
                vec![TableCell::Number {
                    value: 2.5,
                    unit: Some("mV".to_owned()),
                }],
            ],
            reference: external_reference(false),
        };

        assert_eq!(
            std::str::from_utf8(&render_csv_table(&table).unwrap()).unwrap(),
            "Reading [V]\r\n1.25\r\n2.5 mV\r\n"
        );
    }

    #[test]
    fn csv_selection_is_validated_atomically_and_rendered_in_document_order() {
        let report = report_with_blocks(vec![
            data_table("First", external_reference(false)),
            ReportBlockKind::Prose(ProseBlock {
                style: ProseStyle::Body,
                markdown: "Narrative".to_owned(),
            }),
            data_table("Second", external_reference(true)),
        ]);
        let blocks = report.pages()[0].sections()[0].blocks();
        let first = blocks[0].id();
        let prose = blocks[1].id();
        let second = blocks[2].id();

        let artifacts = publish_selected_table_csv(&report, &[second, first]).unwrap();
        assert_eq!(artifacts[0].block_id(), Some(first));
        assert_eq!(artifacts[1].block_id(), Some(second));
        assert!(matches!(
            publish_selected_table_csv(&report, &[]),
            Err(ReportPublicationError::EmptyCsvSelection)
        ));
        assert!(matches!(
            publish_selected_table_csv(&report, &[first, first]),
            Err(ReportPublicationError::DuplicateCsvSelection)
        ));
        assert!(matches!(
            publish_selected_table_csv(&report, &[prose]),
            Err(ReportPublicationError::BlockIsNotDataTable(id)) if id == prose
        ));
        let missing = ReportBlockId::try_from_uuid(Uuid::new_v4()).unwrap();
        assert!(matches!(
            publish_selected_table_csv(&report, &[missing]),
            Err(ReportPublicationError::BlockNotFound(id)) if id == missing
        ));
    }

    #[test]
    fn html_attribute_escaping_covers_all_delimiters() {
        let mut html = HtmlWriter::new(1024);
        html.attr("<&>\"'").unwrap();
        assert_eq!(html.finish().unwrap(), b"&lt;&amp;&gt;&quot;&#39;".to_vec());
    }

    #[test]
    fn csv_field_quoting_handles_all_rfc4180_delimiters() {
        let mut writer = BoundedWriter::new(1024);
        write_csv_field(&mut writer, "comma, quote \" and\r\nline").unwrap();
        assert_eq!(
            writer.finish(),
            b"\"comma, quote \"\" and\r\nline\"".to_vec()
        );
    }

    #[test]
    fn bounded_writer_rejects_growth_before_exceeding_its_contract() {
        let mut writer = BoundedWriter::new(3);
        writer.write_all(b"abc").unwrap();
        assert!(writer.write_all(b"d").is_err());
        assert!(writer.limit_exceeded());
        assert_eq!(writer.finish(), b"abc");
    }

    #[test]
    fn content_digest_parser_fixture_remains_lowercase_and_exact() {
        let parsed = ContentDigest::from_str(&digest(5).to_string()).unwrap();
        assert_eq!(parsed, digest(5));
    }
}

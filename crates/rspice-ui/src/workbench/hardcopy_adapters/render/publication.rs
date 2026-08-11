//! Turning a scene into an output artifact.
//!
//! The preview pages, printer rasters, and the PDF/HTML publication the
//! renderer produces, plus the validation each output has to pass first.
//!
//! Validation runs against the scene, not the artifact. Every primitive is
//! checked to lie inside the page's content extent, every font to be covered,
//! and every decoration to be within capacity, before a single byte is
//! written — a page that fails is refused rather than emitted clipped, since
//! a silently cropped schematic is worse than no print at all.
//!
//! Rendering can be dispatched to a worker, so the worker contracts here
//! authenticate what came back: a transfer is bound to the plan digest and
//! part count it was issued for, and a response that does not match is
//! rejected rather than trusted.

mod pdf;
mod raster;
mod svg;

use pdf::*;
/// The dialog asks the publisher what it can afford rather than predicting it,
/// so this crosses the module boundary the glob import above does not.
pub(crate) use raster::max_raster_dpi;
use raster::*;

use svg::*;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PdfConformance {
    StandardPdf,
    PdfA2bValidated,
}

/// One renderer-owned dialog preview. The dimensions are the exact physical
/// page at the requested preview DPI; no viewport scaling or screen capture is
/// involved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardcopyPreviewPage {
    page_number: u32,
    coordinate: String,
    width: u32,
    height: u32,
    dpi: u16,
    rgba: Vec<u8>,
    soft_proof_applied: bool,
    digest: ContentDigest,
}

/// Transferable browser-worker envelope for a single preview page. Pixels
/// remain a distinct byte buffer so the WASM boundary can transfer the
/// backing `ArrayBuffer` without base64 expansion or a JSON pixel copy.
#[derive(Debug)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) struct PreviewWorkerTransfer {
    manifest_json: Vec<u8>,
    rgba: Vec<u8>,
}
#[cfg(any(test, target_arch = "wasm32"))]
impl PreviewWorkerTransfer {
    pub(crate) fn into_parts(self) -> (Vec<u8>, Vec<u8>) {
        (self.manifest_json, self.rgba)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(super) struct PreviewWorkerManifest {
    pub(super) schema_version: u32,
    pub(super) plan_id: HardcopyPlanId,
    pub(super) plan_digest: ContentDigest,
    pub(super) source_document_id: HardcopyDocumentId,
    pub(super) source_revision: ObjectRevision,
    pub(super) source_digest: ContentDigest,
    pub(super) zero_based_page: u32,
    pub(super) page_number: u32,
    pub(super) coordinate: String,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) dpi: u16,
    pub(super) soft_proof_applied: bool,
    pub(super) rgba_byte_length: u64,
    pub(super) rgba_digest: ContentDigest,
    pub(super) preview_digest: ContentDigest,
    pub(super) transport_digest: ContentDigest,
}

#[derive(Serialize)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(super) struct PreviewWorkerManifestMaterial<'a> {
    pub(super) schema_version: u32,
    pub(super) plan_id: HardcopyPlanId,
    pub(super) plan_digest: ContentDigest,
    pub(super) source_document_id: HardcopyDocumentId,
    pub(super) source_revision: ObjectRevision,
    pub(super) source_digest: ContentDigest,
    pub(super) zero_based_page: u32,
    pub(super) page_number: u32,
    pub(super) coordinate: &'a str,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) dpi: u16,
    pub(super) soft_proof_applied: bool,
    pub(super) rgba_byte_length: u64,
    pub(super) rgba_digest: ContentDigest,
    pub(super) preview_digest: ContentDigest,
}

impl HardcopyPreviewPage {
    #[must_use]
    pub const fn page_number(&self) -> u32 {
        self.page_number
    }

    #[must_use]
    pub fn coordinate(&self) -> &str {
        &self.coordinate
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    #[cfg(test)]
    pub const fn dpi(&self) -> u16 {
        self.dpi
    }

    #[must_use]
    pub fn rgba(&self) -> &[u8] {
        &self.rgba
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    /// Consume this page into a small authenticated manifest and a raw RGBA
    /// payload suitable for a transferable browser `ArrayBuffer`.
    ///
    /// The immutable plan, authenticated source, planned page, exact physical
    /// dimensions, DPI, pixel bytes, and renderer preview digest are all
    /// checked before either buffer crosses the worker boundary.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn into_worker_transfer(
        self,
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        zero_based_page: usize,
    ) -> Result<PreviewWorkerTransfer, HardcopyRenderError> {
        validate_preview_worker_contract(&self, plan, source, zero_based_page)?;
        let rgba_digest = ContentDigest::from_bytes(Sha256::digest(&self.rgba).into());
        let rgba_byte_length = u64::try_from(self.rgba.len())
            .map_err(|_| HardcopyRenderError::WorkerSnapshotTooLarge)?;
        let zero_based_page = u32::try_from(zero_based_page).map_err(|_| {
            HardcopyRenderError::WorkerSnapshot("preview page index exceeds u32".to_owned())
        })?;
        let material = PreviewWorkerManifestMaterial {
            schema_version: PREVIEW_WORKER_MANIFEST_SCHEMA_VERSION,
            plan_id: plan.id(),
            plan_digest: plan.content_digest(),
            source_document_id: source.authority().document_id(),
            source_revision: source.authority().revision(),
            source_digest: source.authority().content_digest(),
            zero_based_page,
            page_number: self.page_number,
            coordinate: &self.coordinate,
            width: self.width,
            height: self.height,
            dpi: self.dpi,
            soft_proof_applied: self.soft_proof_applied,
            rgba_byte_length,
            rgba_digest,
            preview_digest: self.digest,
        };
        let transport_digest = preview_worker_material_digest(&material)?;
        let manifest = PreviewWorkerManifest {
            schema_version: material.schema_version,
            plan_id: material.plan_id,
            plan_digest: material.plan_digest,
            source_document_id: material.source_document_id,
            source_revision: material.source_revision,
            source_digest: material.source_digest,
            zero_based_page: material.zero_based_page,
            page_number: material.page_number,
            coordinate: material.coordinate.to_owned(),
            width: material.width,
            height: material.height,
            dpi: material.dpi,
            soft_proof_applied: material.soft_proof_applied,
            rgba_byte_length: material.rgba_byte_length,
            rgba_digest,
            preview_digest: material.preview_digest,
            transport_digest,
        };
        let manifest_json = serde_json::to_vec(&manifest)
            .map_err(|error| HardcopyRenderError::WorkerSnapshot(error.to_string()))?;
        validate_preview_worker_transfer_budget(manifest_json.len(), self.rgba.len())?;
        Ok(PreviewWorkerTransfer {
            manifest_json,
            rgba: self.rgba,
        })
    }

    /// Reconstruct a renderer-owned preview only after validating the
    /// metadata-only manifest and independently transferred RGBA bytes against
    /// caller-retained plan, source, page, and DPI authority.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn from_worker_transfer(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        expected_zero_based_page: usize,
        expected_dpi: u16,
        manifest_json: &[u8],
        rgba: Vec<u8>,
    ) -> Result<Self, HardcopyRenderError> {
        validate_preview_worker_transfer_budget(manifest_json.len(), rgba.len())?;
        if manifest_json.is_empty() {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker manifest is empty".to_owned(),
            ));
        }
        let manifest: PreviewWorkerManifest = serde_json::from_slice(manifest_json)
            .map_err(|error| HardcopyRenderError::WorkerSnapshot(error.to_string()))?;
        if manifest.schema_version != PREVIEW_WORKER_MANIFEST_SCHEMA_VERSION {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "unsupported preview worker manifest schema".to_owned(),
            ));
        }
        if !(36..=1_200).contains(&expected_dpi) {
            return Err(HardcopyRenderError::InvalidPreviewDpi(expected_dpi));
        }
        let expected_zero_based_page_u32 =
            u32::try_from(expected_zero_based_page).map_err(|_| {
                HardcopyRenderError::WorkerSnapshot("preview page index exceeds u32".to_owned())
            })?;
        let observed_rgba_byte_length =
            u64::try_from(rgba.len()).map_err(|_| HardcopyRenderError::WorkerSnapshotTooLarge)?;
        validate_worker_authority(plan, source)?;
        let page = plan
            .pagination()
            .pages()
            .get(expected_zero_based_page)
            .ok_or(HardcopyRenderError::PreviewPageOutOfRange {
                index: expected_zero_based_page,
                page_count: plan.pagination().pages().len(),
            })?;
        let (expected_width, expected_height, expected_pixels) =
            raster_dimensions(page, expected_dpi)?;
        let expected_rgba_bytes = expected_pixels
            .checked_mul(4)
            .ok_or(HardcopyRenderError::WorkerSnapshotTooLarge)?;
        if manifest.plan_id != plan.id()
            || manifest.plan_digest != plan.content_digest()
            || manifest.source_document_id != source.authority().document_id()
            || manifest.source_revision != source.authority().revision()
            || manifest.source_digest != source.authority().content_digest()
            || manifest.zero_based_page != expected_zero_based_page_u32
            || manifest.page_number != page.number()
            || manifest.coordinate != page.coordinate()
            || manifest.width != expected_width
            || manifest.height != expected_height
            || manifest.dpi != expected_dpi
            || manifest.soft_proof_applied != plan.setup().render().soft_proof_print_safe_colors()
            || manifest.rgba_byte_length != expected_rgba_bytes
            || manifest.rgba_byte_length != observed_rgba_byte_length
        {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker manifest does not match the immutable plan, source, page, or DPI"
                    .to_owned(),
            ));
        }
        let observed_rgba_digest = ContentDigest::from_bytes(Sha256::digest(&rgba).into());
        if observed_rgba_digest != manifest.rgba_digest {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker RGBA digest mismatch".to_owned(),
            ));
        }
        let material = PreviewWorkerManifestMaterial {
            schema_version: manifest.schema_version,
            plan_id: manifest.plan_id,
            plan_digest: manifest.plan_digest,
            source_document_id: manifest.source_document_id,
            source_revision: manifest.source_revision,
            source_digest: manifest.source_digest,
            zero_based_page: manifest.zero_based_page,
            page_number: manifest.page_number,
            coordinate: &manifest.coordinate,
            width: manifest.width,
            height: manifest.height,
            dpi: manifest.dpi,
            soft_proof_applied: manifest.soft_proof_applied,
            rgba_byte_length: manifest.rgba_byte_length,
            rgba_digest: manifest.rgba_digest,
            preview_digest: manifest.preview_digest,
        };
        if preview_worker_material_digest(&material)? != manifest.transport_digest {
            return Err(HardcopyRenderError::WorkerSnapshot(
                "preview worker manifest digest mismatch".to_owned(),
            ));
        }
        let preview = Self {
            page_number: manifest.page_number,
            coordinate: manifest.coordinate,
            width: manifest.width,
            height: manifest.height,
            dpi: manifest.dpi,
            rgba,
            soft_proof_applied: manifest.soft_proof_applied,
            digest: manifest.preview_digest,
        };
        validate_preview_worker_contract(&preview, plan, source, expected_zero_based_page)?;
        Ok(preview)
    }
}
#[cfg(any(test, target_arch = "wasm32"))]
pub(super) fn validate_preview_worker_transfer_budget(
    manifest_bytes: usize,
    rgba_bytes: usize,
) -> Result<(), HardcopyRenderError> {
    let transfer_bytes = manifest_bytes
        .checked_add(rgba_bytes)
        .ok_or(HardcopyRenderError::WorkerSnapshotTooLarge)?;
    if manifest_bytes > MAX_PREVIEW_WORKER_MANIFEST_BYTES
        || rgba_bytes > MAX_PREVIEW_WORKER_RGBA_BYTES
        || transfer_bytes > MAX_PREVIEW_WORKER_TRANSFER_BYTES
    {
        return Err(HardcopyRenderError::WorkerSnapshotTooLarge);
    }
    Ok(())
}
#[cfg(any(test, target_arch = "wasm32"))]
pub(super) fn preview_worker_material_digest(
    material: &PreviewWorkerManifestMaterial<'_>,
) -> Result<ContentDigest, HardcopyRenderError> {
    let payload = serde_json::to_vec(material)
        .map_err(|error| HardcopyRenderError::WorkerSnapshot(error.to_string()))?;
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-preview-worker-manifest-v1");
    digest.update((payload.len() as u64).to_le_bytes());
    digest.update(payload);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}
#[cfg(any(test, target_arch = "wasm32"))]
fn validate_preview_worker_contract(
    preview: &HardcopyPreviewPage,
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    zero_based_page: usize,
) -> Result<(), HardcopyRenderError> {
    validate_worker_authority(plan, source)?;
    validate_text("preview coordinate", &preview.coordinate, 128)?;
    if preview.page_number == 0 || !(36..=1_200).contains(&preview.dpi) {
        return Err(HardcopyRenderError::WorkerSnapshot(
            "preview page identity or DPI is invalid".to_owned(),
        ));
    }
    let page = plan.pagination().pages().get(zero_based_page).ok_or(
        HardcopyRenderError::PreviewPageOutOfRange {
            index: zero_based_page,
            page_count: plan.pagination().pages().len(),
        },
    )?;
    let (expected_width, expected_height, expected_pixels) = raster_dimensions(page, preview.dpi)?;
    let rgba_bytes = expected_pixels
        .checked_mul(4)
        .ok_or(HardcopyRenderError::WorkerSnapshotTooLarge)?;
    let observed_rgba_bytes = u64::try_from(preview.rgba.len())
        .map_err(|_| HardcopyRenderError::WorkerSnapshotTooLarge)?;
    if preview.page_number != page.number()
        || preview.coordinate != page.coordinate()
        || preview.width != expected_width
        || preview.height != expected_height
        || rgba_bytes != observed_rgba_bytes
        || rgba_bytes > MAX_PREVIEW_WORKER_RGBA_BYTES as u64
        || preview.soft_proof_applied != plan.setup().render().soft_proof_print_safe_colors()
    {
        return Err(HardcopyRenderError::WorkerSnapshot(
            "preview does not match the planned page geometry or bounded RGBA payload".to_owned(),
        ));
    }
    let expected_digest = preview_content_digest(
        plan,
        source,
        page,
        preview.dpi,
        preview.width,
        preview.height,
        &preview.rgba,
    );
    if preview.digest != expected_digest {
        return Err(HardcopyRenderError::WorkerSnapshot(
            "preview renderer digest mismatch".to_owned(),
        ));
    }
    Ok(())
}

fn preview_content_digest(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    page: &PreviewPage,
    dpi: u16,
    width: u32,
    height: u32,
    rgba: &[u8],
) -> ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-preview-v1");
    digest.update(plan.content_digest().as_bytes());
    digest.update(source.authority().content_digest().as_bytes());
    digest.update(page.number().to_le_bytes());
    digest.update(dpi.to_le_bytes());
    digest.update(width.to_le_bytes());
    digest.update(height.to_le_bytes());
    digest.update(rgba);
    ContentDigest::from_bytes(digest.finalize().into())
}

/// One fully rendered physical printer page in straight-alpha RGBA8 sRGB.
/// Native adapters may discard the alpha channel after validation; it is
/// always 255 because printer plans require an opaque background.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrinterRasterPage {
    page_number: u32,
    coordinate: String,
    width: u32,
    height: u32,
    dpi: u16,
    rgba: Vec<u8>,
    digest: ContentDigest,
}

#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
impl PrinterRasterPage {
    #[must_use]
    pub const fn page_number(&self) -> u32 {
        self.page_number
    }

    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[must_use]
    pub const fn dpi(&self) -> u16 {
        self.dpi
    }

    #[must_use]
    pub fn rgba(&self) -> &[u8] {
        &self.rgba
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedPrinterPages {
    pages: Vec<PrinterRasterPage>,
    digest: ContentDigest,
}

impl RenderedPrinterPages {
    #[must_use]
    pub fn pages(&self) -> &[PrinterRasterPage] {
        &self.pages
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }
}

/// One independently publishable file. Single-file formats have one part;
/// SVG and PNG tiling produce one numbered part per planned physical page.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedHardcopyPart {
    bytes: Vec<u8>,
    digest: ContentDigest,
    media_type: &'static str,
    filename_extension: &'static str,
    suggested_filename: String,
    first_page: u32,
    page_count: u32,
}

impl RenderedHardcopyPart {
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    #[must_use]
    pub const fn media_type(&self) -> &'static str {
        self.media_type
    }

    #[must_use]
    #[cfg(test)]
    pub const fn filename_extension(&self) -> &'static str {
        self.filename_extension
    }

    #[must_use]
    pub fn suggested_filename(&self) -> &str {
        &self.suggested_filename
    }

    #[must_use]
    #[cfg(test)]
    pub const fn first_page(&self) -> u32 {
        self.first_page
    }

    #[must_use]
    #[cfg(test)]
    pub const fn page_count(&self) -> u32 {
        self.page_count
    }
}

/// Complete typed render publication. `digest` binds the ordered part names,
/// media types, page ranges, and exact bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedHardcopyPublication {
    parts: Vec<RenderedHardcopyPart>,
    digest: ContentDigest,
    format: OutputFormat,
    page_count: u32,
    pdf_conformance: Option<PdfConformance>,
}

/// Transferable browser-worker envelope. Artifact payloads remain distinct
/// byte buffers so the WASM boundary can transfer their backing ArrayBuffers
/// without JSON/base64 expansion or a second aggregate artifact allocation.
#[derive(Debug)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(crate) struct PublicationWorkerTransfer {
    manifest_json: Vec<u8>,
    payloads: Vec<Vec<u8>>,
}
#[cfg(any(test, target_arch = "wasm32"))]
impl PublicationWorkerTransfer {
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn into_parts(self) -> (Vec<u8>, Vec<Vec<u8>>) {
        (self.manifest_json, self.payloads)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(super) struct PublicationWorkerManifest {
    pub(super) schema_version: u32,
    pub(super) plan_digest: ContentDigest,
    pub(super) source_digest: ContentDigest,
    pub(super) publication_digest: ContentDigest,
    pub(super) format: OutputFormat,
    pub(super) page_count: u32,
    pub(super) pdf_conformance: Option<PdfConformance>,
    pub(super) parts: Vec<PublicationWorkerPartManifest>,
    pub(super) transport_digest: ContentDigest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(super) struct PublicationWorkerPartManifest {
    pub(super) ordinal: u32,
    pub(super) byte_length: u64,
    pub(super) digest: ContentDigest,
    pub(super) media_type: String,
    pub(super) filename_extension: String,
    pub(super) suggested_filename: String,
    pub(super) first_page: u32,
    pub(super) page_count: u32,
}

#[derive(Serialize)]
#[cfg(any(test, target_arch = "wasm32"))]
pub(super) struct PublicationWorkerManifestMaterial<'a> {
    pub(super) schema_version: u32,
    pub(super) plan_digest: ContentDigest,
    pub(super) source_digest: ContentDigest,
    pub(super) publication_digest: ContentDigest,
    pub(super) format: OutputFormat,
    pub(super) page_count: u32,
    pub(super) pdf_conformance: Option<PdfConformance>,
    pub(super) parts: &'a [PublicationWorkerPartManifest],
}

impl RenderedHardcopyPublication {
    #[must_use]
    pub fn parts(&self) -> &[RenderedHardcopyPart] {
        &self.parts
    }

    #[must_use]
    pub fn single_part(&self) -> Option<&RenderedHardcopyPart> {
        (self.parts.len() == 1).then(|| &self.parts[0])
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    #[must_use]
    pub const fn format(&self) -> OutputFormat {
        self.format
    }

    #[must_use]
    pub const fn page_count(&self) -> u32 {
        self.page_count
    }

    #[must_use]
    #[cfg(test)]
    pub const fn pdf_conformance(&self) -> Option<PdfConformance> {
        self.pdf_conformance
    }

    pub fn identity(&self) -> Result<HardcopyArtifactIdentity, HardcopyRenderError> {
        let byte_length = self.parts.iter().try_fold(0_u64, |sum, part| {
            sum.checked_add(part.bytes.len() as u64)
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "aggregate artifact bytes",
                    maximum: u64::MAX,
                })
        })?;
        HardcopyArtifactIdentity::try_new(self.digest, byte_length, self.page_count, self.format)
            .map_err(|error| HardcopyRenderError::ArtifactIdentity(error.to_string()))
    }

    /// Consume a validated publication into a small digest-bound manifest and
    /// independent payload buffers suitable for transferable ArrayBuffers.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn into_worker_transfer(
        self,
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
    ) -> Result<PublicationWorkerTransfer, HardcopyRenderError> {
        validate_publication_worker_contract(&self, plan, source)?;
        let parts = self
            .parts
            .iter()
            .enumerate()
            .map(|(index, part)| PublicationWorkerPartManifest {
                ordinal: index as u32,
                byte_length: part.bytes.len() as u64,
                digest: part.digest,
                media_type: part.media_type.to_owned(),
                filename_extension: part.filename_extension.to_owned(),
                suggested_filename: part.suggested_filename.clone(),
                first_page: part.first_page,
                page_count: part.page_count,
            })
            .collect::<Vec<_>>();
        let material = PublicationWorkerManifestMaterial {
            schema_version: PUBLICATION_WORKER_MANIFEST_SCHEMA_VERSION,
            plan_digest: plan.content_digest(),
            source_digest: source.authority().content_digest(),
            publication_digest: self.digest,
            format: self.format,
            page_count: self.page_count,
            pdf_conformance: self.pdf_conformance,
            parts: &parts,
        };
        let transport_digest = publication_worker_manifest_digest(&material)?;
        let manifest = PublicationWorkerManifest {
            schema_version: material.schema_version,
            plan_digest: material.plan_digest,
            source_digest: material.source_digest,
            publication_digest: material.publication_digest,
            format: material.format,
            page_count: material.page_count,
            pdf_conformance: material.pdf_conformance,
            parts,
            transport_digest,
        };
        let manifest_json = serde_json::to_vec(&manifest)
            .map_err(|error| HardcopyRenderError::PublicationWorkerTransfer(error.to_string()))?;
        if manifest_json.len() > MAX_PUBLICATION_WORKER_MANIFEST_BYTES {
            return Err(HardcopyRenderError::PublicationWorkerManifestTooLarge);
        }
        let payloads = self.parts.into_iter().map(|part| part.bytes).collect();
        Ok(PublicationWorkerTransfer {
            manifest_json,
            payloads,
        })
    }

    /// Reconstruct a renderer-owned publication only after validating the
    /// complete manifest and every independently transferred payload against
    /// the immutable plan and authenticated source.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn from_worker_transfer(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        manifest_json: &[u8],
        payloads: Vec<Vec<u8>>,
    ) -> Result<Self, HardcopyRenderError> {
        if manifest_json.is_empty() {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication worker manifest is empty".to_owned(),
            ));
        }
        if manifest_json.len() > MAX_PUBLICATION_WORKER_MANIFEST_BYTES {
            return Err(HardcopyRenderError::PublicationWorkerManifestTooLarge);
        }
        let manifest: PublicationWorkerManifest = serde_json::from_slice(manifest_json)
            .map_err(|error| HardcopyRenderError::PublicationWorkerTransfer(error.to_string()))?;
        if manifest.schema_version != PUBLICATION_WORKER_MANIFEST_SCHEMA_VERSION {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "unsupported publication worker manifest schema".to_owned(),
            ));
        }
        validate_worker_authority(plan, source)?;
        let expected_part_count = publication_worker_part_count(plan);
        if manifest.plan_digest != plan.content_digest()
            || manifest.source_digest != source.authority().content_digest()
            || manifest.format != plan.setup().render().format()
            || manifest.page_count != plan.pagination().pages().len() as u32
            || manifest.pdf_conformance != expected_pdf_conformance(manifest.format)
            || manifest.parts.len() != expected_part_count
            || manifest.parts.len() != payloads.len()
        {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication manifest does not match the immutable plan or source".to_owned(),
            ));
        }
        let material = PublicationWorkerManifestMaterial {
            schema_version: manifest.schema_version,
            plan_digest: manifest.plan_digest,
            source_digest: manifest.source_digest,
            publication_digest: manifest.publication_digest,
            format: manifest.format,
            page_count: manifest.page_count,
            pdf_conformance: manifest.pdf_conformance,
            parts: &manifest.parts,
        };
        if publication_worker_manifest_digest(&material)? != manifest.transport_digest {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication worker manifest digest mismatch".to_owned(),
            ));
        }
        let mut aggregate_bytes = 0_u64;
        let mut parts = Vec::with_capacity(payloads.len());
        for (index, (part_manifest, bytes)) in manifest.parts.iter().zip(payloads).enumerate() {
            let expected = expected_worker_part(plan, index)?;
            if part_manifest.ordinal != index as u32
                || part_manifest.byte_length != bytes.len() as u64
                || part_manifest.media_type != expected.media_type
                || part_manifest.filename_extension != expected.filename_extension
                || part_manifest.suggested_filename != expected.suggested_filename
                || part_manifest.first_page != expected.first_page
                || part_manifest.page_count != expected.page_count
            {
                return Err(HardcopyRenderError::PublicationWorkerTransfer(
                    "publication part metadata is not canonical".to_owned(),
                ));
            }
            if bytes.is_empty() || bytes.len() > MAX_ARTIFACT_BYTES {
                return Err(HardcopyRenderError::ResourceLimit {
                    scope: "artifact part bytes",
                    maximum: MAX_ARTIFACT_BYTES as u64,
                });
            }
            aggregate_bytes = aggregate_bytes.checked_add(bytes.len() as u64).ok_or(
                HardcopyRenderError::ResourceLimit {
                    scope: "aggregate publication bytes",
                    maximum: MAX_PUBLICATION_BYTES,
                },
            )?;
            let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
            if digest != part_manifest.digest {
                return Err(HardcopyRenderError::PublicationWorkerTransfer(
                    "publication part digest mismatch".to_owned(),
                ));
            }
            parts.push(RenderedHardcopyPart {
                bytes,
                digest,
                media_type: expected.media_type,
                filename_extension: expected.filename_extension,
                suggested_filename: expected.suggested_filename,
                first_page: expected.first_page,
                page_count: expected.page_count,
            });
        }
        if aggregate_bytes > MAX_PUBLICATION_BYTES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate publication bytes",
                maximum: MAX_PUBLICATION_BYTES,
            });
        }
        let digest = publication_digest(manifest.format, manifest.page_count, &parts);
        if digest != manifest.publication_digest {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication aggregate digest mismatch".to_owned(),
            ));
        }
        let publication = Self {
            parts,
            digest,
            format: manifest.format,
            page_count: manifest.page_count,
            pdf_conformance: manifest.pdf_conformance,
        };
        validate_publication_worker_contract(&publication, plan, source)?;
        Ok(publication)
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
struct ExpectedWorkerPart {
    media_type: &'static str,
    filename_extension: &'static str,
    suggested_filename: String,
    first_page: u32,
    page_count: u32,
}

#[cfg(any(test, target_arch = "wasm32"))]
fn validate_worker_authority(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
) -> Result<(), HardcopyRenderError> {
    if plan.source() != source.authority() {
        return Err(HardcopyRenderError::SourceAuthorityMismatch);
    }
    if plan.content_extent()
        != source
            .content_extent_for_setup(plan.setup().schematic())
            .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?
    {
        return Err(HardcopyRenderError::ExtentMismatch);
    }
    Ok(())
}

#[cfg(any(test, target_arch = "wasm32"))]
fn expected_pdf_conformance(format: OutputFormat) -> Option<PdfConformance> {
    match format {
        OutputFormat::PdfA => Some(PdfConformance::PdfA2bValidated),
        OutputFormat::PdfVector | OutputFormat::NativePrinter => Some(PdfConformance::StandardPdf),
        OutputFormat::BrowserPrintDocument
        | OutputFormat::SvgVector
        | OutputFormat::Png { .. }
        | OutputFormat::Tiff { .. } => None,
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn publication_worker_part_count(plan: &HardcopyPlan) -> usize {
    match plan.setup().render().format() {
        OutputFormat::SvgVector | OutputFormat::Png { .. } => plan.pagination().pages().len(),
        _ => 1,
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn expected_worker_part(
    plan: &HardcopyPlan,
    index: usize,
) -> Result<ExpectedWorkerPart, HardcopyRenderError> {
    let page_count = plan.pagination().pages().len() as u32;
    let format = plan.setup().render().format();
    match format {
        OutputFormat::SvgVector | OutputFormat::Png { .. } => {
            let page = plan.pagination().pages().get(index).ok_or_else(|| {
                HardcopyRenderError::PublicationWorkerTransfer(
                    "publication has more page parts than the immutable plan".to_owned(),
                )
            })?;
            let (media_type, filename_extension) = match format {
                OutputFormat::SvgVector => ("image/svg+xml", "svg"),
                OutputFormat::Png { .. } => ("image/png", "png"),
                _ => unreachable!("matched above"),
            };
            Ok(ExpectedWorkerPart {
                media_type,
                filename_extension,
                suggested_filename: format!(
                    "page-{:04}-{}.{}",
                    page.number(),
                    page.coordinate(),
                    filename_extension
                ),
                first_page: page.number(),
                page_count: 1,
            })
        }
        _ if index != 0 => Err(HardcopyRenderError::PublicationWorkerTransfer(
            "single-artifact format contains multiple worker payloads".to_owned(),
        )),
        OutputFormat::BrowserPrintDocument => Ok(ExpectedWorkerPart {
            media_type: "text/html",
            filename_extension: "html",
            suggested_filename: "hardcopy-print.html".to_owned(),
            first_page: 1,
            page_count,
        }),
        OutputFormat::Tiff { .. } => Ok(ExpectedWorkerPart {
            media_type: "image/tiff",
            filename_extension: "tiff",
            suggested_filename: "hardcopy.tiff".to_owned(),
            first_page: 1,
            page_count,
        }),
        OutputFormat::PdfA | OutputFormat::PdfVector | OutputFormat::NativePrinter => {
            Ok(ExpectedWorkerPart {
                media_type: "application/pdf",
                filename_extension: "pdf",
                suggested_filename: "hardcopy.pdf".to_owned(),
                first_page: 1,
                page_count,
            })
        }
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn validate_publication_worker_contract(
    publication: &RenderedHardcopyPublication,
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
) -> Result<(), HardcopyRenderError> {
    validate_worker_authority(plan, source)?;
    let expected_page_count = plan.pagination().pages().len() as u32;
    let expected_parts = publication_worker_part_count(plan);
    if publication.format != plan.setup().render().format()
        || publication.page_count != expected_page_count
        || publication.pdf_conformance != expected_pdf_conformance(publication.format)
        || publication.parts.len() != expected_parts
    {
        return Err(HardcopyRenderError::PublicationWorkerTransfer(
            "publication does not match the immutable format and pagination".to_owned(),
        ));
    }
    let mut aggregate_bytes = 0_u64;
    for (index, part) in publication.parts.iter().enumerate() {
        let expected = expected_worker_part(plan, index)?;
        if part.media_type != expected.media_type
            || part.filename_extension != expected.filename_extension
            || part.suggested_filename != expected.suggested_filename
            || part.first_page != expected.first_page
            || part.page_count != expected.page_count
            || part.bytes.is_empty()
            || part.bytes.len() > MAX_ARTIFACT_BYTES
            || ContentDigest::from_bytes(Sha256::digest(&part.bytes).into()) != part.digest
        {
            return Err(HardcopyRenderError::PublicationWorkerTransfer(
                "publication part failed canonical metadata or digest validation".to_owned(),
            ));
        }
        aggregate_bytes = aggregate_bytes.checked_add(part.bytes.len() as u64).ok_or(
            HardcopyRenderError::ResourceLimit {
                scope: "aggregate publication bytes",
                maximum: MAX_PUBLICATION_BYTES,
            },
        )?;
    }
    if aggregate_bytes > MAX_PUBLICATION_BYTES
        || publication_digest(
            publication.format,
            publication.page_count,
            &publication.parts,
        ) != publication.digest
    {
        return Err(HardcopyRenderError::PublicationWorkerTransfer(
            "publication aggregate size or digest is invalid".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(any(test, target_arch = "wasm32"))]
pub(super) fn publication_worker_manifest_digest(
    material: &PublicationWorkerManifestMaterial<'_>,
) -> Result<ContentDigest, HardcopyRenderError> {
    let payload = serde_json::to_vec(material)
        .map_err(|error| HardcopyRenderError::PublicationWorkerTransfer(error.to_string()))?;
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-publication-worker-v1");
    digest.update((payload.len() as u64).to_le_bytes());
    digest.update(payload);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

#[derive(Debug, thiserror::Error)]
pub enum HardcopyRenderError {
    #[error("hardcopy publication timestamp is not a valid UTC Gregorian date and time")]
    InvalidTimestamp,
    #[error("{field} must be trimmed printable text no longer than {maximum} bytes")]
    InvalidText { field: &'static str, maximum: usize },
    #[error("hardcopy scene exceeds the {scope} limit of {maximum}")]
    ResourceLimit { scope: &'static str, maximum: u64 },
    #[error("scene primitive has empty geometry")]
    EmptyPrimitiveGeometry,
    #[error("scene coordinate lies outside the declared content extent")]
    PrimitiveOutsideExtent,
    #[error("polyline requires at least two points, or three points when closed")]
    InvalidPolyline,
    #[error("authored-sheet clipped scene groups must be nonempty, canonical, and non-nested")]
    InvalidClippedScene,
    #[error("stroke width {0:?} is outside the supported physical range")]
    InvalidStrokeWidth(Length),
    #[error("cross-hatch line width and spacing must be ordered physical values within 25 mm")]
    InvalidCrossHatch,
    #[error("text size must be between 1 micrometre and 100 millimetres")]
    InvalidTextSize,
    #[error("embedded hardcopy font has no glyph for U+{codepoint:04X} in {context}")]
    UnsupportedGlyph {
        codepoint: u32,
        context: &'static str,
    },
    #[error("hardcopy scene extent does not match the immutable render plan")]
    ExtentMismatch,
    #[error("aggregate scene sections do not match the immutable pagination plan")]
    AggregatePaginationMismatch,
    #[error("resolved hardcopy source authority does not match the immutable render plan")]
    SourceAuthorityMismatch,
    #[error("could not convert the resolved semantic hardcopy source: {0}")]
    SourceConversion(String),
    #[error(
        "schematic content crosses the authored drawing sheet; choose Clip to authored drawing sheet or Extend output to include content"
    )]
    SchematicOutsideContentDecisionRequired,
    #[error(
        "report {0} lacks authenticated renderable content; publication is blocked instead of flattening evidence"
    )]
    UnsupportedAuthenticatedReportBlock(&'static str),
    #[error("report block reference is absent from or differs from the authenticated snapshot")]
    UnauthenticatedReportReference,
    #[error("report Markdown contains unsupported or unbalanced authored constructs")]
    UnsupportedReportMarkdown,
    #[error("embedded report figure digest does not match its authenticated artifact")]
    EmbeddedFigureDigestMismatch,
    #[error("embedded report figure is not a supported bounded PNG: {0}")]
    InvalidEmbeddedFigure(String),
    #[error("PDF/A export requires an explicit deterministic publication timestamp")]
    PdfARequiresPublicationTimestamp,
    #[error(
        "{decoration} contains {actual} entries but the planned band can render at most {maximum}"
    )]
    DecorationOverflow {
        decoration: &'static str,
        actual: usize,
        maximum: usize,
    },
    #[error("SVG export cannot outline text; enable searchable text or remove text from the scene")]
    SvgTextOutliningUnsupported,
    #[error("deterministic browser printing with text requires embedded fonts")]
    BrowserTextRequiresEmbeddedFonts,
    #[error("native printer output must use an opaque page background")]
    TransparentPrinterPage,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("printer raster pages require a NativePrinter plan targeted at a system printer")]
    PrinterRasterRequiresNativePlan,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("printer device resolution {0} DPI is outside the supported 72–9600 DPI range")]
    InvalidPrinterDpi(u16),
    #[cfg(not(target_arch = "wasm32"))]
    #[error("printer render DPI differs from the immutable native job resolution")]
    PrinterRasterDpiMismatch,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("planned printable area lies outside the sealed native driver printable geometry")]
    PrinterPrintableGeometryMismatch,
    #[cfg(not(target_arch = "wasm32"))]
    #[error("the selected native printer job cannot mix portrait and landscape page geometry")]
    PrinterMixedPageGeometryUnsupported,
    #[error("preview resolution {0} DPI is outside the supported 36–1200 DPI range")]
    InvalidPreviewDpi(u16),
    #[error("preview page index {index} is outside the planned {page_count} pages")]
    PreviewPageOutOfRange { index: usize, page_count: usize },
    #[error("preview requests must contain one or two distinct page indices")]
    InvalidPreviewPageBatch,
    #[cfg(any(test, target_arch = "wasm32"))]
    #[error("hardcopy preview worker transfer exceeds its bounded transport budget")]
    WorkerSnapshotTooLarge,
    #[cfg(any(test, target_arch = "wasm32"))]
    #[error("hardcopy preview worker transfer is invalid: {0}")]
    WorkerSnapshot(String),
    #[cfg(any(test, target_arch = "wasm32"))]
    #[error("hardcopy publication worker manifest exceeds its bounded transport budget")]
    PublicationWorkerManifestTooLarge,
    #[cfg(any(test, target_arch = "wasm32"))]
    #[error("hardcopy publication worker transfer is invalid: {0}")]
    PublicationWorkerTransfer(String),
    #[error("raster dimensions overflow the supported integer range")]
    RasterDimensionOverflow,
    #[error("could not parse canonical SVG for rasterization: {0}")]
    SvgParse(String),
    #[error("could not allocate the requested raster page")]
    RasterAllocation,
    #[error("could not encode {format}: {message}")]
    Encoding {
        format: &'static str,
        message: String,
    },
    #[error("could not load embedded font {0}")]
    InvalidEmbeddedFont(&'static str),
    #[error("could not configure validated PDF/A-2b output: {0}")]
    PdfAConfiguration(String),
    #[error("PDF serialization or validation failed: {0}")]
    PdfSerialization(String),
    #[error("hardcopy artifact identity is invalid: {0}")]
    ArtifactIdentity(String),
}

/// Deterministic, platform-independent hardcopy renderer.
pub struct HardcopyRenderer;

impl HardcopyRenderer {
    /// Convert and render an authenticated semantic source in one fail-closed
    /// operation. The plan must bind the exact same source authority and
    /// physical extent; a stale tab or replaced revision can never be printed
    /// through a newer plan.
    pub fn render_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
    ) -> Result<RenderedHardcopyPublication, HardcopyRenderError> {
        if plan.source() != source.authority() {
            return Err(HardcopyRenderError::SourceAuthorityMismatch);
        }
        if plan.content_extent()
            != source
                .content_extent_for_setup(plan.setup().schematic())
                .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?
        {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        let scene = scene_from_resolved(
            source,
            plan.setup().print_mapping(),
            plan.setup().schematic(),
            metadata,
        )?;
        Self::render_scene(plan, &scene)
    }

    /// Resolve and rasterize a single planned page for the hardcopy dialog.
    /// The selected export/print format is intentionally ignored: preview is
    /// a non-authoritative view of the same sealed semantic plan.
    #[cfg(test)]
    pub fn render_preview_page_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
        zero_based_page: usize,
        dpi: u16,
    ) -> Result<HardcopyPreviewPage, HardcopyRenderError> {
        let mut pages = Self::render_preview_pages_resolved(
            plan,
            source,
            metadata,
            &[zero_based_page],
            dpi,
            || false,
        )?;
        pages
            .pop()
            .ok_or(HardcopyRenderError::InvalidPreviewPageBatch)
    }

    /// Resolve and validate one semantic scene, then rasterize a bounded set
    /// of preview pages. `cancelled` is checked between pages so a worker can
    /// retain the selected page while skipping speculative adjacent work.
    pub fn render_preview_pages_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
        zero_based_pages: &[usize],
        dpi: u16,
        cancelled: impl Fn() -> bool,
    ) -> Result<Vec<HardcopyPreviewPage>, HardcopyRenderError> {
        if plan.source() != source.authority() {
            return Err(HardcopyRenderError::SourceAuthorityMismatch);
        }
        if plan.content_extent()
            != source
                .content_extent_for_setup(plan.setup().schematic())
                .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?
        {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        if !(36..=1_200).contains(&dpi) {
            return Err(HardcopyRenderError::InvalidPreviewDpi(dpi));
        }
        if zero_based_pages.is_empty()
            || zero_based_pages.len() > MAX_PREVIEW_BATCH_PAGES
            || (zero_based_pages.len() == 2 && zero_based_pages[0] == zero_based_pages[1])
        {
            return Err(HardcopyRenderError::InvalidPreviewPageBatch);
        }
        let pages = zero_based_pages
            .iter()
            .copied()
            .map(|index| {
                plan.pagination().pages().get(index).ok_or(
                    HardcopyRenderError::PreviewPageOutOfRange {
                        index,
                        page_count: plan.pagination().pages().len(),
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let scene = scene_from_resolved(
            source,
            plan.setup().print_mapping(),
            plan.setup().schematic(),
            metadata,
        )?;
        scene.validate()?;
        validate_plan_font_coverage(plan)?;
        validate_aggregate_pagination(plan, &scene)?;
        validate_decoration_capacity(plan, &scene)?;
        validate_render_budget(plan, &scene, pages.len())?;
        let mut largest_preview = 0_u64;
        for page in &pages {
            let (_, _, preview_pixels) = raster_dimensions(page, dpi)?;
            largest_preview = largest_preview.max(preview_pixels);
        }
        validate_raster_working_set(largest_preview, 1, 8)?;

        let mut rendered = Vec::with_capacity(pages.len());
        for (position, page) in pages.into_iter().enumerate() {
            if position != 0 && cancelled() {
                break;
            }
            let mut raster = rasterize_page(plan, &scene, page, dpi)?;
            let soft_proof_applied = plan.setup().render().soft_proof_print_safe_colors();
            if soft_proof_applied {
                apply_soft_proof_preview(&mut raster.rgba);
            }
            let digest = preview_content_digest(
                plan,
                source,
                page,
                dpi,
                raster.width,
                raster.height,
                &raster.rgba,
            );
            rendered.push(HardcopyPreviewPage {
                page_number: page.number(),
                coordinate: page.coordinate().to_owned(),
                width: raster.width,
                height: raster.height,
                dpi,
                rgba: raster.rgba,
                soft_proof_applied,
                digest,
            });
        }
        if rendered.is_empty() {
            return Err(HardcopyRenderError::PreviewPageOutOfRange {
                index: zero_based_pages[0],
                page_count: plan.pagination().pages().len(),
            });
        }
        Ok(rendered)
    }

    /// Render exact native-printer pages directly from the authenticated
    /// semantic source, closing the adapter gap without exposing a mutable
    /// intermediate scene.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn render_printer_pages_resolved(
        plan: &HardcopyPlan,
        source: &ResolvedHardcopyDocument,
        metadata: HardcopySceneMetadata,
        device_dpi: u16,
    ) -> Result<RenderedPrinterPages, HardcopyRenderError> {
        if plan.source() != source.authority() {
            return Err(HardcopyRenderError::SourceAuthorityMismatch);
        }
        if plan.content_extent()
            != source
                .content_extent_for_setup(plan.setup().schematic())
                .map_err(|error| HardcopyRenderError::SourceConversion(error.to_string()))?
        {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        let scene = scene_from_resolved(
            source,
            plan.setup().print_mapping(),
            plan.setup().schematic(),
            metadata,
        )?;
        Self::render_printer_scene(plan, &scene, device_dpi)
    }

    /// Raw scene rendering exists only for module and adapter contract tests.
    /// Production callers must use [`Self::render_resolved`] so the semantic
    /// document authority is checked against the immutable plan.
    #[cfg(test)]
    pub(crate) fn render(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
    ) -> Result<RenderedHardcopyPublication, HardcopyRenderError> {
        Self::render_scene(plan, scene)
    }

    fn render_scene(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
    ) -> Result<RenderedHardcopyPublication, HardcopyRenderError> {
        scene.validate()?;
        validate_plan_font_coverage(plan)?;
        if scene.extent != plan.content_extent() {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        validate_aggregate_pagination(plan, scene)?;
        validate_decoration_capacity(plan, scene)?;
        if matches!(
            plan.setup().render().background(),
            BackgroundMode::Transparent
        ) && plan.setup().render().format() == OutputFormat::NativePrinter
        {
            return Err(HardcopyRenderError::TransparentPrinterPage);
        }
        let format = plan.setup().render().format();
        let page_count = u32::try_from(plan.pagination().pages().len()).map_err(|_| {
            HardcopyRenderError::ResourceLimit {
                scope: "pages",
                maximum: u32::MAX as u64,
            }
        })?;
        validate_render_budget(plan, scene, page_count as usize)?;
        let (parts, conformance) = match format {
            OutputFormat::BrowserPrintDocument => {
                if scene_contains_text(scene) && !plan.setup().render().fonts().embed_fonts() {
                    return Err(HardcopyRenderError::BrowserTextRequiresEmbeddedFonts);
                }
                (
                    vec![make_part(
                        render_browser_print_html(plan, scene)?.into_bytes(),
                        "text/html",
                        "html",
                        "hardcopy-print.html".to_owned(),
                        1,
                        page_count,
                    )?],
                    None,
                )
            }
            OutputFormat::SvgVector => {
                if !plan.setup().render().fonts().preserve_searchable_text()
                    && scene_contains_text(scene)
                {
                    return Err(HardcopyRenderError::SvgTextOutliningUnsupported);
                }
                let mut parts = Vec::with_capacity(plan.pagination().pages().len());
                for page in plan.pagination().pages() {
                    parts.push(make_part(
                        render_page_svg(plan, scene, page)?.into_bytes(),
                        "image/svg+xml",
                        "svg",
                        format!("page-{:04}-{}.svg", page.number(), page.coordinate()),
                        page.number(),
                        1,
                    )?);
                }
                (parts, None)
            }
            OutputFormat::Png { dpi } => {
                let (total, largest_page) = aggregate_raster_pixels(plan, dpi)?;
                if total > MAX_RASTER_PIXELS_TOTAL {
                    return Err(HardcopyRenderError::ResourceLimit {
                        scope: "aggregate raster pixels",
                        maximum: MAX_RASTER_PIXELS_TOTAL,
                    });
                }
                validate_raster_working_set(largest_page, 1, PNG_BYTES_PER_PIXEL)?;
                let mut parts = Vec::with_capacity(plan.pagination().pages().len());
                for page in plan.pagination().pages() {
                    let pixels = rasterize_page(plan, scene, page, dpi)?;
                    parts.push(make_part(
                        encode_png(&pixels, dpi)?,
                        "image/png",
                        "png",
                        format!("page-{:04}-{}.png", page.number(), page.coordinate()),
                        page.number(),
                        1,
                    )?);
                }
                (parts, None)
            }
            OutputFormat::Tiff { dpi } => (
                vec![make_part(
                    render_tiff(plan, scene, dpi)?,
                    "image/tiff",
                    "tiff",
                    "hardcopy.tiff".to_owned(),
                    1,
                    page_count,
                )?],
                None,
            ),
            OutputFormat::PdfA => (
                vec![make_part(
                    render_pdf(plan, scene, true)?,
                    "application/pdf",
                    "pdf",
                    "hardcopy.pdf".to_owned(),
                    1,
                    page_count,
                )?],
                Some(PdfConformance::PdfA2bValidated),
            ),
            OutputFormat::PdfVector | OutputFormat::NativePrinter => (
                vec![make_part(
                    render_pdf(plan, scene, false)?,
                    "application/pdf",
                    "pdf",
                    "hardcopy.pdf".to_owned(),
                    1,
                    page_count,
                )?],
                Some(PdfConformance::StandardPdf),
            ),
        };
        let publication_bytes = parts.iter().try_fold(0_u64, |sum, part| {
            sum.checked_add(part.bytes.len() as u64)
                .ok_or(HardcopyRenderError::ResourceLimit {
                    scope: "aggregate publication bytes",
                    maximum: MAX_PUBLICATION_BYTES,
                })
        })?;
        if publication_bytes > MAX_PUBLICATION_BYTES {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate publication bytes",
                maximum: MAX_PUBLICATION_BYTES,
            });
        }
        let digest = publication_digest(format, page_count, &parts);
        Ok(RenderedHardcopyPublication {
            parts,
            digest,
            format,
            page_count,
            pdf_conformance: conformance,
        })
    }

    /// Render exact physical pages for a native printer adapter at the
    /// selected device resolution. This uses the same canonical SVG scene and
    /// pagination as file publication; platform code never has to parse PDF or
    /// duplicate layout logic.
    #[cfg(test)]
    pub(crate) fn render_printer_pages(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
        device_dpi: u16,
    ) -> Result<RenderedPrinterPages, HardcopyRenderError> {
        Self::render_printer_scene(plan, scene, device_dpi)
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render_printer_scene(
        plan: &HardcopyPlan,
        scene: &HardcopyScene,
        device_dpi: u16,
    ) -> Result<RenderedPrinterPages, HardcopyRenderError> {
        let RenderTarget::SystemPrinter { job, .. } = plan.setup().render().target() else {
            return Err(HardcopyRenderError::PrinterRasterRequiresNativePlan);
        };
        if plan.setup().render().format() != OutputFormat::NativePrinter {
            return Err(HardcopyRenderError::PrinterRasterRequiresNativePlan);
        }
        if !(72..=9_600).contains(&device_dpi) {
            return Err(HardcopyRenderError::InvalidPrinterDpi(device_dpi));
        }
        if device_dpi != job.resolution_dpi() {
            return Err(HardcopyRenderError::PrinterRasterDpiMismatch);
        }
        if plan.setup().render().background() == BackgroundMode::Transparent {
            return Err(HardcopyRenderError::TransparentPrinterPage);
        }
        scene.validate()?;
        validate_plan_font_coverage(plan)?;
        if scene.extent != plan.content_extent() {
            return Err(HardcopyRenderError::ExtentMismatch);
        }
        validate_aggregate_pagination(plan, scene)?;
        validate_decoration_capacity(plan, scene)?;
        validate_render_budget(plan, scene, plan.pagination().pages().len())?;
        let first_page = plan
            .pagination()
            .pages()
            .first()
            .ok_or(HardcopyRenderError::AggregatePaginationMismatch)?;
        if plan
            .pagination()
            .pages()
            .iter()
            .any(|page| page.geometry() != first_page.geometry())
        {
            return Err(HardcopyRenderError::PrinterMixedPageGeometryUnsupported);
        }
        let (raster_width, raster_height) = job.raster_geometry().physical_size_px();
        validate_driver_printable_geometry(
            first_page,
            raster_width,
            raster_height,
            job.raster_geometry().printable_rect_px(),
        )?;
        let per_page_pixels = u64::from(raster_width)
            .checked_mul(u64::from(raster_height))
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        if per_page_pixels > MAX_RASTER_PIXELS_PER_PAGE {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "printer raster pixels per page",
                maximum: MAX_RASTER_PIXELS_PER_PAGE,
            });
        }
        let total_pixels = per_page_pixels
            .checked_mul(plan.pagination().pages().len() as u64)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        if total_pixels > MAX_RASTER_PIXELS_TOTAL {
            return Err(HardcopyRenderError::ResourceLimit {
                scope: "aggregate printer raster pixels",
                maximum: MAX_RASTER_PIXELS_TOTAL,
            });
        }
        validate_printer_raster_working_set(
            per_page_pixels,
            plan.pagination().pages().len() as u64,
        )?;
        let mut pages = Vec::with_capacity(plan.pagination().pages().len());
        let mut publication_digest = Sha256::new();
        publication_digest.update(b"rspice-hardcopy-printer-pages-v1");
        publication_digest.update(device_dpi.to_le_bytes());
        for page in plan.pagination().pages() {
            let raster = rasterize_page_at_dimensions(
                plan,
                scene,
                page,
                device_dpi,
                raster_width,
                raster_height,
            )?;
            if raster.rgba.chunks_exact(4).any(|pixel| pixel[3] != u8::MAX) {
                return Err(HardcopyRenderError::TransparentPrinterPage);
            }
            let digest = ContentDigest::from_bytes(Sha256::digest(&raster.rgba).into());
            publication_digest.update(page.number().to_le_bytes());
            publication_digest.update(raster.width.to_le_bytes());
            publication_digest.update(raster.height.to_le_bytes());
            publication_digest.update(&raster.rgba);
            pages.push(PrinterRasterPage {
                page_number: page.number(),
                coordinate: page.coordinate().to_owned(),
                width: raster.width,
                height: raster.height,
                dpi: device_dpi,
                rgba: raster.rgba,
                digest,
            });
        }
        Ok(RenderedPrinterPages {
            pages,
            digest: ContentDigest::from_bytes(publication_digest.finalize().into()),
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn validate_driver_printable_geometry(
    page: &PreviewPage,
    raster_width: u32,
    raster_height: u32,
    driver_printable: (u32, u32, u32, u32),
) -> Result<(), HardcopyRenderError> {
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let (physical_width, physical_height) = geometry.physical_size();
    let map_floor =
        |position: u64, pixels: u32, physical: u64| -> Result<u64, HardcopyRenderError> {
            u64::try_from(
                u128::from(position)
                    .checked_mul(u128::from(pixels))
                    .ok_or(HardcopyRenderError::RasterDimensionOverflow)?
                    / u128::from(physical),
            )
            .map_err(|_| HardcopyRenderError::RasterDimensionOverflow)
        };
    let map_ceil =
        |position: u64, pixels: u32, physical: u64| -> Result<u64, HardcopyRenderError> {
            u64::try_from(
                u128::from(position)
                    .checked_mul(u128::from(pixels))
                    .ok_or(HardcopyRenderError::RasterDimensionOverflow)?
                    .div_ceil(u128::from(physical)),
            )
            .map_err(|_| HardcopyRenderError::RasterDimensionOverflow)
        };
    let right_um = printable
        .x
        .micrometres()
        .checked_add(printable.width.micrometres())
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    let bottom_um = printable
        .y
        .micrometres()
        .checked_add(printable.height.micrometres())
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    let planned_left = map_floor(
        printable.x.micrometres(),
        raster_width,
        physical_width.micrometres(),
    )?;
    let planned_top = map_floor(
        printable.y.micrometres(),
        raster_height,
        physical_height.micrometres(),
    )?;
    let planned_right = map_ceil(right_um, raster_width, physical_width.micrometres())?;
    let planned_bottom = map_ceil(bottom_um, raster_height, physical_height.micrometres())?;
    let (driver_x, driver_y, driver_width, driver_height) = driver_printable;
    let driver_right = u64::from(driver_x)
        .checked_add(u64::from(driver_width))
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    let driver_bottom = u64::from(driver_y)
        .checked_add(u64::from(driver_height))
        .ok_or(HardcopyRenderError::PrinterPrintableGeometryMismatch)?;
    if planned_left < u64::from(driver_x)
        || planned_top < u64::from(driver_y)
        || planned_right > driver_right
        || planned_bottom > driver_bottom
    {
        return Err(HardcopyRenderError::PrinterPrintableGeometryMismatch);
    }
    Ok(())
}

fn render_browser_print_html(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
) -> Result<String, HardcopyRenderError> {
    let mm = |length: Length| length.micrometres() as f64 / 1_000.0;
    let first_orientation = plan
        .pagination()
        .pages()
        .first()
        .map(|page| page.geometry().orientation());
    let orientation = if plan
        .pagination()
        .pages()
        .iter()
        .all(|page| Some(page.geometry().orientation()) == first_orientation)
    {
        match first_orientation {
            Some(ResolvedOrientation::Landscape) => "landscape",
            Some(ResolvedOrientation::Portrait) => "portrait",
            None => "unknown",
        }
    } else {
        "mixed"
    };
    let capacity = checked_vector_capacity(
        4_096,
        plan.pagination().pages().len(),
        32_768,
        scene.primitives.len(),
        96,
    )?;
    let mut html = String::with_capacity(capacity);
    html.push_str("<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">");
    html.push_str("<meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; style-src 'unsafe-inline'; font-src data:; img-src data:; base-uri 'none'; form-action 'none'\">");
    write!(
        html,
        "<meta name=\"rspice-plan-digest\" content=\"{}\"><meta name=\"rspice-source-digest\" content=\"{}\"><meta name=\"rspice-page-count\" content=\"{}\"><title>",
        plan.content_digest(),
        plan.source().content_digest(),
        plan.pagination().pages().len()
    )
    .expect("write to string");
    escape_xml_into(scene.metadata.title(), &mut html);
    html.push_str("</title><style>");
    html.push_str("*{box-sizing:border-box}html,body{margin:0;padding:0;background:#fff}body{-webkit-print-color-adjust:exact;print-color-adjust:exact}");
    html.push_str(".rspice-print-page{position:relative;break-after:page;page-break-after:always;overflow:hidden}.rspice-print-page:last-child{break-after:auto;page-break-after:auto}.rspice-print-page>svg{position:absolute;inset:0;max-width:none;max-height:none;display:block}");
    for page in plan.pagination().pages() {
        let (width, height) = page.geometry().physical_size();
        let width_mm = mm(width);
        let height_mm = mm(height);
        write!(
            html,
            "@page rspice-page-{}{{size:{width_mm:.3}mm {height_mm:.3}mm;margin:0;}}.rspice-print-page[data-page=\"{}\"]{{page:rspice-page-{};width:{width_mm:.3}mm;height:{height_mm:.3}mm}}.rspice-print-page[data-page=\"{}\"]>svg{{width:{width_mm:.3}mm;height:{height_mm:.3}mm}}",
            page.number(),
            page.number(),
            page.number(),
            page.number(),
        )
        .expect("write to string");
    }
    html.push_str("@media screen{body{padding:12mm;background:#c7cbd0}.rspice-print-page{margin:0 auto 12mm;box-shadow:0 2mm 6mm rgba(0,0,0,.28);background:#fff}}@media print{body{background:transparent}.rspice-print-page{margin:0}}</style></head>");
    write!(
        html,
        "<body data-rspice-plan=\"{}\" data-rspice-source=\"{}\" data-rspice-orientation=\"{orientation}\">",
        plan.content_digest(),
        plan.source().content_digest()
    )
    .expect("write to string");
    for page in plan.pagination().pages() {
        write!(
            html,
            "<section class=\"rspice-print-page\" data-page=\"{}\" data-coordinate=\"{}\" aria-label=\"Page {} of {}\">",
            page.number(),
            page.coordinate(),
            page.number(),
            plan.pagination().pages().len()
        )
        .expect("write to string");
        html.push_str(&render_page_svg(plan, scene, page)?);
        html.push_str("</section>");
    }
    // Deliberately script-free. The authenticated browser-print adapter owns
    // the popup load listener and invokes `print()` only after this complete,
    // self-contained document has loaded.
    html.push_str("</body></html>");
    Ok(html)
}

fn validate_plan_font_coverage(plan: &HardcopyPlan) -> Result<(), HardcopyRenderError> {
    let coverage = FontCoverage::load()?;
    if let Watermark::Custom(text) = plan.setup().decorations().watermark() {
        coverage.validate_text(SceneFont::SansSemibold, text, "custom watermark")?;
    }
    // Validate renderer-authored punctuation up front. Authored engineering
    // symbols are checked against their selected face as each scene text node
    // is validated; unsupported symbols fail precisely instead of making all
    // unrelated publications unavailable.
    const FIXED_PUBLICATION_GLYPHS: &str = "rev · page / source plan";
    coverage.validate_text(
        SceneFont::Sans,
        FIXED_PUBLICATION_GLYPHS,
        "fixed publication text",
    )?;
    coverage.validate_text(
        SceneFont::Monospace,
        FIXED_PUBLICATION_GLYPHS,
        "fixed publication text",
    )
}

fn validate_decoration_capacity(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
) -> Result<(), HardcopyRenderError> {
    let decorations = plan.setup().decorations();
    if decorations.includes_header() && scene.metadata.header_lines.len() > 1 {
        return Err(HardcopyRenderError::DecorationOverflow {
            decoration: "header",
            actual: scene.metadata.header_lines.len(),
            maximum: 1,
        });
    }
    if decorations.includes_provenance() && scene.metadata.provenance_lines.len() > 1 {
        return Err(HardcopyRenderError::DecorationOverflow {
            decoration: "provenance",
            actual: scene.metadata.provenance_lines.len(),
            maximum: 1,
        });
    }
    if decorations.includes_legends() {
        for page in plan.pagination().pages() {
            let geometry = page.geometry();
            let columns = geometry.printable_rect().width.micrometres() / LEGEND_COLUMN_UM;
            let usable_height = geometry
                .legend_band()
                .micrometres()
                .saturating_sub(LEGEND_VERTICAL_PADDING_UM);
            let rows = usable_height / LEGEND_ROW_UM;
            let maximum = usize::try_from(columns.saturating_mul(rows)).unwrap_or(usize::MAX);
            if scene.legend.len() > maximum {
                return Err(HardcopyRenderError::DecorationOverflow {
                    decoration: "legend",
                    actual: scene.legend.len(),
                    maximum,
                });
            }
        }
    }
    Ok(())
}

fn make_part(
    bytes: Vec<u8>,
    media_type: &'static str,
    filename_extension: &'static str,
    suggested_filename: String,
    first_page: u32,
    page_count: u32,
) -> Result<RenderedHardcopyPart, HardcopyRenderError> {
    if bytes.is_empty() || bytes.len() > MAX_ARTIFACT_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "artifact part bytes",
            maximum: MAX_ARTIFACT_BYTES as u64,
        });
    }
    let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
    Ok(RenderedHardcopyPart {
        bytes,
        digest,
        media_type,
        filename_extension,
        suggested_filename,
        first_page,
        page_count,
    })
}

fn publication_digest(
    format: OutputFormat,
    page_count: u32,
    parts: &[RenderedHardcopyPart],
) -> ContentDigest {
    let mut digest = Sha256::new();
    digest.update(b"rspice-hardcopy-publication-v1");
    digest.update(serde_json::to_vec(&format).expect("OutputFormat serialization cannot fail"));
    digest.update(page_count.to_le_bytes());
    digest.update((parts.len() as u64).to_le_bytes());
    for part in parts {
        digest.update(part.first_page.to_le_bytes());
        digest.update(part.page_count.to_le_bytes());
        digest.update((part.media_type.len() as u64).to_le_bytes());
        digest.update(part.media_type.as_bytes());
        digest.update((part.suggested_filename.len() as u64).to_le_bytes());
        digest.update(part.suggested_filename.as_bytes());
        digest.update((part.bytes.len() as u64).to_le_bytes());
        digest.update(&part.bytes);
    }
    ContentDigest::from_bytes(digest.finalize().into())
}

pub(super) fn validate_lines(
    field: &'static str,
    lines: &[String],
    maximum_lines: usize,
    maximum_line_bytes: usize,
) -> Result<(), HardcopyRenderError> {
    if lines.len() > maximum_lines {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: field,
            maximum: maximum_lines as u64,
        });
    }
    for line in lines {
        validate_text(field, line, maximum_line_bytes)?;
    }
    Ok(())
}

pub(super) fn validate_text(
    field: &'static str,
    value: &str,
    maximum: usize,
) -> Result<(), HardcopyRenderError> {
    if value.is_empty()
        || value != value.trim()
        || value.len() > maximum
        || value.chars().any(char::is_control)
    {
        Err(HardcopyRenderError::InvalidText { field, maximum })
    } else {
        Ok(())
    }
}

pub(super) fn png_dimensions(payload: &[u8]) -> Result<(u32, u32), HardcopyRenderError> {
    if payload.is_empty() || payload.len() > MAX_EMBEDDED_FIGURE_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "embedded figure bytes",
            maximum: MAX_EMBEDDED_FIGURE_BYTES as u64,
        });
    }
    let reader = png::Decoder::new(Cursor::new(payload))
        .read_info()
        .map_err(|error| HardcopyRenderError::InvalidEmbeddedFigure(error.to_string()))?;
    let info = reader.info();
    let pixels = u64::from(info.width)
        .checked_mul(u64::from(info.height))
        .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
    if info.width == 0 || info.height == 0 || pixels > MAX_RASTER_PIXELS_PER_PAGE {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "embedded figure pixels",
            maximum: MAX_RASTER_PIXELS_PER_PAGE,
        });
    }
    Ok((info.width, info.height))
}

pub(super) fn validate_point(
    point: ScenePoint,
    extent: ContentExtent,
) -> Result<(), HardcopyRenderError> {
    if point.x > extent.width() || point.y > extent.height() {
        Err(HardcopyRenderError::PrimitiveOutsideExtent)
    } else {
        Ok(())
    }
}

pub(super) fn validate_rect(
    rect: SceneRect,
    extent: ContentExtent,
) -> Result<(), HardcopyRenderError> {
    let right = rect
        .x
        .micrometres()
        .checked_add(rect.width.micrometres())
        .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
    let bottom = rect
        .y
        .micrometres()
        .checked_add(rect.height.micrometres())
        .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
    if right > extent.width().micrometres() || bottom > extent.height().micrometres() {
        Err(HardcopyRenderError::PrimitiveOutsideExtent)
    } else {
        Ok(())
    }
}

pub(super) fn validate_primitive(
    primitive: &ScenePrimitive,
    extent: ContentExtent,
    text_bytes: &mut usize,
    coverage: &FontCoverage,
) -> Result<(), HardcopyRenderError> {
    match primitive {
        ScenePrimitive::Line { from, to, stroke } => {
            validate_point(*from, extent)?;
            validate_point(*to, extent)?;
            validate_stroke(*stroke)
        }
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } => {
            if points.len() < if *closed { 3 } else { 2 } {
                return Err(HardcopyRenderError::InvalidPolyline);
            }
            for point in points {
                validate_point(*point, extent)?;
            }
            validate_stroke(*stroke)?;
            validate_fill(*fill)
        }
        ScenePrimitive::Rect { rect, stroke, fill } => {
            validate_rect(*rect, extent)?;
            if let Some(stroke) = stroke {
                validate_stroke(*stroke)?;
            }
            validate_fill(*fill)
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke,
            fill,
        } => {
            let right = center
                .x
                .micrometres()
                .checked_add(radius.micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            let bottom = center
                .y
                .micrometres()
                .checked_add(radius.micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            if *radius == Length::ZERO
                || center.x.micrometres() < radius.micrometres()
                || center.y.micrometres() < radius.micrometres()
                || right > extent.width().micrometres()
                || bottom > extent.height().micrometres()
            {
                return Err(HardcopyRenderError::PrimitiveOutsideExtent);
            }
            if let Some(stroke) = stroke {
                validate_stroke(*stroke)?;
            }
            validate_fill(*fill)
        }
        ScenePrimitive::RasterImage {
            rect,
            png,
            content_digest,
            alternative_text,
        } => {
            validate_rect(*rect, extent)?;
            validate_text("figure alternative text", alternative_text, 16_384)?;
            coverage.validate_text(SceneFont::Sans, alternative_text, "figure alternative text")?;
            if png.is_empty() || png.len() > MAX_EMBEDDED_FIGURE_BYTES {
                return Err(HardcopyRenderError::ResourceLimit {
                    scope: "embedded figure bytes",
                    maximum: MAX_EMBEDDED_FIGURE_BYTES as u64,
                });
            }
            let observed = ContentDigest::from_bytes(Sha256::digest(png).into());
            if observed != *content_digest {
                return Err(HardcopyRenderError::EmbeddedFigureDigestMismatch);
            }
            let _ = png_dimensions(png)?;
            *text_bytes = text_bytes.checked_add(alternative_text.len()).ok_or(
                HardcopyRenderError::ResourceLimit {
                    scope: "scene text bytes",
                    maximum: MAX_SCENE_TEXT_BYTES as u64,
                },
            )?;
            Ok(())
        }
        ScenePrimitive::Text {
            origin,
            text,
            font,
            size,
            ..
        } => {
            validate_point(*origin, extent)?;
            validate_text("scene text", text, 65_536)?;
            if *size == Length::ZERO || size.micrometres() > 100_000 {
                return Err(HardcopyRenderError::InvalidTextSize);
            }
            coverage.validate_text(*font, text, "scene text")?;
            *text_bytes =
                text_bytes
                    .checked_add(text.len())
                    .ok_or(HardcopyRenderError::ResourceLimit {
                        scope: "scene text bytes",
                        maximum: MAX_SCENE_TEXT_BYTES as u64,
                    })?;
            Ok(())
        }
        ScenePrimitive::ClippedGroup {
            source_origin,
            destination_origin,
            clip_extent,
            source_extent,
            primitives,
        } => {
            if primitives.is_empty() || primitives.len() > MAX_SCENE_PRIMITIVES {
                return Err(HardcopyRenderError::ResourceLimit {
                    scope: "clipped scene primitives",
                    maximum: MAX_SCENE_PRIMITIVES as u64,
                });
            }
            let source_right = source_origin
                .x
                .micrometres()
                .checked_add(clip_extent.width().micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            let source_bottom = source_origin
                .y
                .micrometres()
                .checked_add(clip_extent.height().micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            let destination_right = destination_origin
                .x
                .micrometres()
                .checked_add(clip_extent.width().micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            let destination_bottom = destination_origin
                .y
                .micrometres()
                .checked_add(clip_extent.height().micrometres())
                .ok_or(HardcopyRenderError::PrimitiveOutsideExtent)?;
            if source_right > source_extent.width().micrometres()
                || source_bottom > source_extent.height().micrometres()
                || destination_right > extent.width().micrometres()
                || destination_bottom > extent.height().micrometres()
            {
                return Err(HardcopyRenderError::PrimitiveOutsideExtent);
            }
            for primitive in primitives {
                if matches!(primitive, ScenePrimitive::ClippedGroup { .. }) {
                    return Err(HardcopyRenderError::InvalidClippedScene);
                }
                validate_primitive(primitive, *source_extent, text_bytes, coverage)?;
            }
            Ok(())
        }
    }
}

pub(super) fn validate_fill(fill: Option<SceneFill>) -> Result<(), HardcopyRenderError> {
    if let Some(SceneFill::CrossHatch {
        line_width,
        spacing,
        ..
    }) = fill
        && (line_width == Length::ZERO
            || spacing == Length::ZERO
            || line_width.micrometres() > 25_000
            || spacing.micrometres() > 25_000
            || line_width > spacing)
    {
        return Err(HardcopyRenderError::InvalidCrossHatch);
    }
    Ok(())
}

pub(super) fn primitive_vertex_count(primitive: &ScenePrimitive) -> usize {
    match primitive {
        ScenePrimitive::Line { .. } => 2,
        ScenePrimitive::Polyline { points, .. } => points.len(),
        ScenePrimitive::Rect { .. } => 4,
        ScenePrimitive::Circle { .. } => 4,
        ScenePrimitive::RasterImage { .. } => 4,
        ScenePrimitive::Text { .. } => 1,
        ScenePrimitive::ClippedGroup { primitives, .. } => {
            primitives.iter().map(primitive_vertex_count).sum()
        }
    }
}

pub(super) fn primitive_hatch_line_count(primitive: &ScenePrimitive) -> u64 {
    if let ScenePrimitive::ClippedGroup { primitives, .. } = primitive {
        return primitives.iter().map(primitive_hatch_line_count).sum();
    }
    let Some(SceneFill::CrossHatch { spacing, .. }) = primitive_fill(primitive) else {
        return 0;
    };
    let span = match primitive {
        ScenePrimitive::Polyline { points, .. } => {
            let minimum_x = points.iter().map(|point| point.x.micrometres()).min();
            let maximum_x = points.iter().map(|point| point.x.micrometres()).max();
            let minimum_y = points.iter().map(|point| point.y.micrometres()).min();
            let maximum_y = points.iter().map(|point| point.y.micrometres()).max();
            match (minimum_x, maximum_x, minimum_y, maximum_y) {
                (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) => {
                    max_x.saturating_sub(min_x) + max_y.saturating_sub(min_y)
                }
                _ => 0,
            }
        }
        ScenePrimitive::Rect { rect, .. } => rect.width.micrometres() + rect.height.micrometres(),
        ScenePrimitive::Circle { radius, .. } => radius.micrometres().saturating_mul(4),
        ScenePrimitive::Line { .. }
        | ScenePrimitive::RasterImage { .. }
        | ScenePrimitive::Text { .. } => 0,
        ScenePrimitive::ClippedGroup { .. } => unreachable!("handled before fill resolution"),
    };
    span.div_ceil(spacing.micrometres()).saturating_mul(2)
}

pub(super) fn validate_stroke(stroke: StrokeStyle) -> Result<(), HardcopyRenderError> {
    if stroke.width == Length::ZERO || stroke.width.micrometres() > 100_000 {
        Err(HardcopyRenderError::InvalidStrokeWidth(stroke.width))
    } else {
        Ok(())
    }
}

pub(super) fn scene_contains_text(scene: &HardcopyScene) -> bool {
    fn contains_text(primitive: &ScenePrimitive) -> bool {
        match primitive {
            ScenePrimitive::Text { .. } => true,
            ScenePrimitive::ClippedGroup { primitives, .. } => primitives.iter().any(contains_text),
            _ => false,
        }
    }
    scene.primitives.iter().any(contains_text)
        || !scene.legend.is_empty()
        || !scene.metadata.header_lines.is_empty()
        || !scene.metadata.provenance_lines.is_empty()
}

fn validate_render_budget(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page_count: usize,
) -> Result<(), HardcopyRenderError> {
    let mut vertices = 0_u64;
    let mut hatch_lines = 0_u64;
    let mut text_bytes = 0_u64;
    let mut image_bytes = 0_u64;
    let mut image_pixels = 0_u64;
    fn accumulate(
        primitive: &ScenePrimitive,
        vertices: &mut u64,
        hatch_lines: &mut u64,
        text_bytes: &mut u64,
        image_bytes: &mut u64,
        image_pixels: &mut u64,
    ) -> Result<(), HardcopyRenderError> {
        if let ScenePrimitive::ClippedGroup { primitives, .. } = primitive {
            for primitive in primitives {
                accumulate(
                    primitive,
                    vertices,
                    hatch_lines,
                    text_bytes,
                    image_bytes,
                    image_pixels,
                )?;
            }
            return Ok(());
        }
        *vertices = vertices
            .checked_add(primitive_vertex_count(primitive) as u64)
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            })?;
        *hatch_lines = hatch_lines
            .checked_add(primitive_hatch_line_count(primitive))
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            })?;
        match primitive {
            ScenePrimitive::Text { text, .. } => {
                *text_bytes = text_bytes.checked_add(text.len() as u64).ok_or(
                    HardcopyRenderError::ResourceLimit {
                        scope: "render work units",
                        maximum: MAX_RENDER_WORK_UNITS,
                    },
                )?;
            }
            ScenePrimitive::RasterImage { png, .. } => {
                *image_bytes = image_bytes.checked_add(png.len() as u64).ok_or(
                    HardcopyRenderError::ResourceLimit {
                        scope: "render work units",
                        maximum: MAX_RENDER_WORK_UNITS,
                    },
                )?;
                let (width, height) = png_dimensions(png)?;
                *image_pixels = image_pixels
                    .checked_add(u64::from(width).saturating_mul(u64::from(height)))
                    .ok_or(HardcopyRenderError::ResourceLimit {
                        scope: "render work units",
                        maximum: MAX_RENDER_WORK_UNITS,
                    })?;
            }
            _ => {}
        }
        Ok(())
    }
    for primitive in &scene.primitives {
        accumulate(
            primitive,
            &mut vertices,
            &mut hatch_lines,
            &mut text_bytes,
            &mut image_bytes,
            &mut image_pixels,
        )?;
    }
    for entry in &scene.legend {
        text_bytes = text_bytes.checked_add(entry.label.len() as u64).ok_or(
            HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            },
        )?;
    }
    let scene_units = (scene.primitives.len() as u64)
        .checked_add(vertices)
        .and_then(|value| value.checked_add(hatch_lines.saturating_mul(2)))
        .and_then(|value| value.checked_add(text_bytes.div_ceil(16)))
        .and_then(|value| value.checked_add(image_pixels.div_ceil(64)))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "render work units",
            maximum: MAX_RENDER_WORK_UNITS,
        })?;
    let total_work =
        scene_units
            .checked_mul(page_count as u64)
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "render work units",
                maximum: MAX_RENDER_WORK_UNITS,
            })?;
    if total_work > MAX_RENDER_WORK_UNITS {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "render work units",
            maximum: MAX_RENDER_WORK_UNITS,
        });
    }
    let decoded_image_bytes =
        image_pixels
            .checked_mul(4)
            .ok_or(HardcopyRenderError::ResourceLimit {
                scope: "embedded figure decoded bytes",
                maximum: MAX_RASTER_WORKING_BYTES,
            })?;
    if decoded_image_bytes > MAX_RASTER_WORKING_BYTES / 2 {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "embedded figure decoded bytes",
            maximum: MAX_RASTER_WORKING_BYTES / 2,
        });
    }

    let per_page = 32_768_u64
        .checked_add((scene.primitives.len() as u64).saturating_mul(192))
        .and_then(|value| value.checked_add(vertices.saturating_mul(40)))
        .and_then(|value| value.checked_add(text_bytes.saturating_mul(6)))
        .and_then(|value| value.checked_add(image_bytes.saturating_mul(2)))
        .and_then(|value| value.checked_add((scene.legend.len() as u64).saturating_mul(320)))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    let embedded_font_bytes = if plan.setup().render().fonts().embed_fonts() {
        (IBM_PLEX_SANS_REGULAR.len() + IBM_PLEX_SANS_SEMIBOLD.len() + IBM_PLEX_MONO_REGULAR.len())
            as u64
            * 2
    } else {
        0
    };
    let format = plan.setup().render().format();
    let font_instances = if matches!(
        format,
        OutputFormat::SvgVector | OutputFormat::BrowserPrintDocument
    ) {
        page_count as u64
    } else {
        u64::from(page_count != 0)
    };
    let estimated_vector_bytes = per_page
        .checked_mul(page_count as u64)
        .and_then(|value| value.checked_add(embedded_font_bytes.saturating_mul(font_instances)))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    if estimated_vector_bytes > MAX_ESTIMATED_VECTOR_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        });
    }
    Ok(())
}

fn validate_raster_working_set(
    pixels_per_page: u64,
    retained_pages: u64,
    bytes_per_pixel_budget: u64,
) -> Result<(), HardcopyRenderError> {
    let bytes = pixels_per_page
        .checked_mul(retained_pages)
        .and_then(|pixels| pixels.checked_mul(bytes_per_pixel_budget))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            maximum: MAX_RASTER_WORKING_BYTES,
        })?;
    if bytes > MAX_RASTER_WORKING_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            maximum: MAX_RASTER_WORKING_BYTES,
        });
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn validate_printer_raster_working_set(
    pixels_per_page: u64,
    page_count: u64,
) -> Result<(), HardcopyRenderError> {
    // Every returned page remains retained as RGBA (4 B/px), while the
    // current SVG parse/raster and the native adapter's BGRX spool conversion
    // require another conservative 12 B/px at peak.
    let bytes_per_pixel = page_count
        .checked_mul(4)
        .and_then(|retained| retained.checked_add(12))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "raster working-set bytes",
            maximum: MAX_RASTER_WORKING_BYTES,
        })?;
    validate_raster_working_set(pixels_per_page, 1, bytes_per_pixel)
}

fn checked_vector_capacity(
    base: usize,
    page_count: usize,
    per_page: usize,
    primitive_count: usize,
    per_primitive: usize,
) -> Result<usize, HardcopyRenderError> {
    let page_bytes = primitive_count
        .checked_mul(per_primitive)
        .and_then(|value| value.checked_add(per_page))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    let capacity = page_count
        .checked_mul(page_bytes)
        .and_then(|value| value.checked_add(base))
        .ok_or(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        })?;
    if capacity as u64 > MAX_ESTIMATED_VECTOR_BYTES {
        return Err(HardcopyRenderError::ResourceLimit {
            scope: "estimated vector bytes",
            maximum: MAX_ESTIMATED_VECTOR_BYTES,
        });
    }
    Ok(capacity)
}

#[derive(Debug, Clone, Copy)]
struct PageTransform {
    content_rect: PageRect,
    window: PageRect,
    scale: ScaleRatio,
    source_origin: ScenePoint,
    destination_origin: ScenePoint,
}

fn validate_aggregate_pagination(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
) -> Result<(), HardcopyRenderError> {
    let planned = plan.pagination().sections();
    let rendered = &scene.aggregate_sections;
    if planned.len() != rendered.len() {
        return Err(HardcopyRenderError::AggregatePaginationMismatch);
    }
    for (planned, rendered) in planned.iter().copied().zip(rendered) {
        let (origin_x, origin_y) = planned.origin();
        if planned.ordinal() != rendered.ordinal
            || planned.content_digest() != rendered.content_digest
            || origin_x != rendered.origin.x
            || origin_y != rendered.origin.y
            || planned.extent() != rendered.extent
            || planned.page_break_before() != rendered.page_break_before
        {
            return Err(HardcopyRenderError::AggregatePaginationMismatch);
        }
    }
    if rendered.is_empty() {
        if plan
            .pagination()
            .pages()
            .iter()
            .any(|page| page.section_ordinal() != 0)
        {
            return Err(HardcopyRenderError::AggregatePaginationMismatch);
        }
        return Ok(());
    }
    for page in plan.pagination().pages() {
        if !rendered
            .iter()
            .any(|section| section.ordinal == page.section_ordinal())
        {
            return Err(HardcopyRenderError::AggregatePaginationMismatch);
        }
    }
    Ok(())
}

mod style;
pub(super) use style::auto_trace_pattern;
use style::{
    ResolvedStroke, background_color, page_primitives, page_transform, resolve_color,
    resolve_stroke, svg_color,
};

//! Dedicated browser-worker protocol for hardcopy source resolution, preview,
//! and publication rendering.
//!
//! The main thread retains the trusted source and plan. Worker requests carry
//! only validated reconstruction inputs plus bounded binary snapshots; worker
//! responses return metadata manifests and raw transferable byte buffers.

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::render::{
    HardcopyRenderer, HardcopySceneMetadata, MAX_ARTIFACT_BYTES, MAX_PREVIEW_WORKER_MANIFEST_BYTES,
    MAX_PREVIEW_WORKER_RGBA_BYTES, MAX_PUBLICATION_BYTES, MAX_PUBLICATION_WORKER_MANIFEST_BYTES,
    RenderedHardcopyPublication,
};
use crate::sources::{MAX_WORKER_SNAPSHOT_BYTES, ResolvedHardcopyDocument};
use rspice_app_types::product::ContentDigest;
use rspice_formats::zip::deterministic_stored_zip;
use rspice_hardcopy_contract::sources::HardcopySourceError;
use rspice_hardcopy_contract::{
    HardcopyArtifactIdentity, HardcopyPlan, HardcopyPlanId, HardcopyScope, HardcopySetup,
    MAX_PREVIEW_PAGES, OutputFormat,
};

pub const HARDCOPY_WORKER_PROTOCOL_VERSION: u32 = 1;
const MAX_REQUEST_METADATA_BYTES: usize = MAX_WORKER_SNAPSHOT_BYTES;
pub const MAX_REQUEST_BUFFERS: usize = 2;
const MAX_RESPONSE_BUFFERS: usize = MAX_PREVIEW_PAGES as usize + 1;
const MAX_RESPONSE_TOTAL_BYTES: usize =
    MAX_PUBLICATION_WORKER_MANIFEST_BYTES + MAX_PUBLICATION_BYTES as usize;
const PACKAGED_PUBLICATION_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HardcopyWorkerOperation {
    ResolveSource,
    Preview,
    Publication,
    PackagedPublication,
}

impl HardcopyWorkerOperation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveSource => "resolve-source",
            Self::Preview => "preview",
            Self::Publication => "publication",
            Self::PackagedPublication => "packaged-publication",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "kebab-case", deny_unknown_fields)]
pub enum HardcopyWorkerCommand {
    ResolveSource {
        source_key: String,
        scope: HardcopyScope,
    },
    Preview {
        plan_id: HardcopyPlanId,
        expected_plan_digest: ContentDigest,
        setup: HardcopySetup,
        metadata: HardcopySceneMetadata,
        page_indices: Vec<usize>,
        dpi: u16,
    },
    Publication {
        plan_id: HardcopyPlanId,
        expected_plan_digest: ContentDigest,
        expected_part_count: usize,
        package_multi_part: bool,
        setup: HardcopySetup,
        metadata: HardcopySceneMetadata,
    },
}

impl HardcopyWorkerCommand {
    const fn operation(&self) -> HardcopyWorkerOperation {
        match self {
            Self::ResolveSource { .. } => HardcopyWorkerOperation::ResolveSource,
            Self::Preview { .. } => HardcopyWorkerOperation::Preview,
            Self::Publication {
                package_multi_part: true,
                ..
            } => HardcopyWorkerOperation::PackagedPublication,
            Self::Publication { .. } => HardcopyWorkerOperation::Publication,
        }
    }
}

/// Wire envelope. Execution revalidates decoded metadata before consuming snapshots.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HardcopyWorkerRequest {
    protocol_version: u32,
    id: u32,
    epoch: String,
    generation: String,
    command: HardcopyWorkerCommand,
}

impl HardcopyWorkerRequest {
    pub fn try_new(
        id: u32,
        epoch: u64,
        generation: u64,
        command: HardcopyWorkerCommand,
    ) -> Result<Self, String> {
        let request = Self {
            protocol_version: HARDCOPY_WORKER_PROTOCOL_VERSION,
            id,
            epoch: epoch.to_string(),
            generation: generation.to_string(),
            command,
        };
        request.validate()?;
        Ok(request)
    }

    pub const fn operation(&self) -> HardcopyWorkerOperation {
        self.command.operation()
    }

    pub fn expected_response_buffer_count(&self) -> usize {
        match &self.command {
            HardcopyWorkerCommand::ResolveSource { .. } => 1,
            HardcopyWorkerCommand::Preview { page_indices, .. } => {
                page_indices.len().saturating_mul(2)
            }
            HardcopyWorkerCommand::Publication {
                expected_part_count,
                package_multi_part,
                ..
            } => {
                if *package_multi_part {
                    2
                } else {
                    expected_part_count.saturating_add(1)
                }
            }
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.protocol_version != HARDCOPY_WORKER_PROTOCOL_VERSION {
            return Err(format!(
                "Unsupported hardcopy worker protocol {}; expected {}.",
                self.protocol_version, HARDCOPY_WORKER_PROTOCOL_VERSION
            ));
        }
        if self.id == 0 {
            return Err("Hardcopy worker request id must be non-zero.".to_owned());
        }
        parse_counter(&self.epoch, "epoch")?;
        if parse_counter(&self.generation, "generation")? == 0 {
            return Err("Hardcopy worker generation must be non-zero.".to_owned());
        }
        match &self.command {
            HardcopyWorkerCommand::ResolveSource {
                source_key, scope, ..
            } => {
                if source_key.is_empty()
                    || source_key.len() > 4_096
                    || source_key.chars().any(char::is_control)
                {
                    return Err("Hardcopy worker source key is invalid.".to_owned());
                }
                if matches!(scope, HardcopyScope::NamedPrintSet(name) if name.is_empty()) {
                    return Err("Hardcopy worker named print-set scope is empty.".to_owned());
                }
            }
            HardcopyWorkerCommand::Preview {
                page_indices, dpi, ..
            } => {
                if page_indices.is_empty() || page_indices.len() > 2 {
                    return Err(
                        "Hardcopy worker preview requires one or two ordered pages.".to_owned()
                    );
                }
                if page_indices.iter().any(|page| *page > u32::MAX as usize) {
                    return Err("A hardcopy worker preview page is out of range.".to_owned());
                }
                if page_indices.len() == 2 && page_indices[0] == page_indices[1] {
                    return Err("Hardcopy worker preview pages must be distinct.".to_owned());
                }
                if !(36..=1_200).contains(dpi) {
                    return Err("Hardcopy worker preview DPI is out of range.".to_owned());
                }
            }
            HardcopyWorkerCommand::Publication {
                setup,
                expected_part_count,
                package_multi_part,
                ..
            } => {
                if setup.render().format() == OutputFormat::NativePrinter {
                    return Err(
                        "A browser worker cannot render a native-printer publication.".to_owned(),
                    );
                }
                if !(1..MAX_RESPONSE_BUFFERS).contains(expected_part_count) {
                    return Err(
                        "Hardcopy worker publication part count is out of range.".to_owned()
                    );
                }
                if *package_multi_part && *expected_part_count < 2 {
                    return Err(
                        "Hardcopy worker packaging requires a multi-part publication.".to_owned(),
                    );
                }
            }
        }
        let metadata = serde_json::to_vec(self)
            .map_err(|error| format!("Could not encode hardcopy worker metadata: {error}"))?;
        validate_metadata_length(metadata.len())
    }
}

pub struct HardcopyWorkerResponse {
    protocol_version: u32,
    id: u32,
    epoch: String,
    generation: String,
    operation: HardcopyWorkerOperation,
    buffers: Vec<Vec<u8>>,
}

impl HardcopyWorkerResponse {
    pub const fn protocol_version(&self) -> u32 {
        self.protocol_version
    }
    pub const fn id(&self) -> u32 {
        self.id
    }
    pub fn epoch(&self) -> &str {
        &self.epoch
    }
    pub fn generation(&self) -> &str {
        &self.generation
    }
    pub const fn operation(&self) -> HardcopyWorkerOperation {
        self.operation
    }
    pub fn into_buffers(self) -> Vec<Vec<u8>> {
        self.buffers
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PackagedPublicationManifest {
    schema_version: u32,
    plan_content_digest: ContentDigest,
    source_content_digest: ContentDigest,
    artifact: HardcopyArtifactIdentity,
}

pub struct PackagedHardcopyPublication {
    bytes: Vec<u8>,
    artifact: HardcopyArtifactIdentity,
    page_count: u32,
}

impl PackagedHardcopyPublication {
    pub fn into_parts(self) -> (Vec<u8>, HardcopyArtifactIdentity, u32) {
        (self.bytes, self.artifact, self.page_count)
    }
}

fn parse_counter(value: &str, name: &str) -> Result<u64, String> {
    let parsed = value
        .parse::<u64>()
        .map_err(|_| format!("Hardcopy worker {name} is not a canonical unsigned integer."))?;
    if parsed.to_string() != value {
        return Err(format!(
            "Hardcopy worker {name} is not a canonical unsigned integer."
        ));
    }
    Ok(parsed)
}

fn validate_metadata_length(length: usize) -> Result<(), String> {
    if length > MAX_REQUEST_METADATA_BYTES {
        return Err(format!(
            "Hardcopy worker metadata exceeds the {MAX_REQUEST_METADATA_BYTES}-byte limit."
        ));
    }
    Ok(())
}

fn validate_request_buffer_lengths(lengths: impl IntoIterator<Item = usize>) -> Result<(), String> {
    let mut count = 0usize;
    let mut aggregate = 0usize;
    for length in lengths {
        count = count.saturating_add(1);
        if count > MAX_REQUEST_BUFFERS {
            return Err("Hardcopy worker request has too many binary buffers.".to_owned());
        }
        if length > MAX_WORKER_SNAPSHOT_BYTES {
            return Err("Hardcopy worker request exceeds its binary transport budget.".to_owned());
        }
        aggregate = aggregate
            .checked_add(length)
            .ok_or_else(|| "Hardcopy worker request buffer size overflowed.".to_owned())?;
        if aggregate > MAX_WORKER_SNAPSHOT_BYTES {
            return Err("Hardcopy worker request exceeds its binary transport budget.".to_owned());
        }
    }
    Ok(())
}

/// Validate response buffer cardinality and byte budgets before copying any
/// worker-owned `ArrayBuffer` into Rust memory. Native tests exercise this
/// same pure contract so browser transport checks cannot drift from renderer
/// resource limits.
pub fn validate_response_buffer_lengths(
    operation: HardcopyWorkerOperation,
    expected_buffer_count: Option<usize>,
    lengths: impl IntoIterator<Item = usize>,
) -> Result<(), String> {
    let mut observed =
        Vec::with_capacity(expected_buffer_count.unwrap_or(1).min(MAX_RESPONSE_BUFFERS));
    for length in lengths {
        if observed.len() == MAX_RESPONSE_BUFFERS {
            return Err("Hardcopy worker response has too many binary buffers.".to_owned());
        }
        observed.push(length);
    }

    let count = observed.len();
    if let Some(expected) = expected_buffer_count
        && count != expected
    {
        return Err("Hardcopy worker response has the wrong operation buffer count.".to_owned());
    }
    match operation {
        HardcopyWorkerOperation::ResolveSource if count != 1 => {
            return Err(
                "Hardcopy source-resolution response requires one resolved snapshot.".to_owned(),
            );
        }
        HardcopyWorkerOperation::Preview if !matches!(count, 2 | 4) => {
            return Err(
                "Hardcopy preview response requires one or two manifest/RGBA pairs.".to_owned(),
            );
        }
        HardcopyWorkerOperation::Publication if !(2..=MAX_RESPONSE_BUFFERS).contains(&count) => {
            return Err(
                "Hardcopy publication response requires a manifest and at least one artifact."
                    .to_owned(),
            );
        }
        HardcopyWorkerOperation::PackagedPublication if count != 2 => {
            return Err(
                "Packaged hardcopy publication requires one manifest and one ZIP payload."
                    .to_owned(),
            );
        }
        _ => {}
    }

    let mut aggregate = 0usize;
    let mut publication_payload_bytes = 0usize;
    for (index, length) in observed.into_iter().enumerate() {
        let per_buffer_limit = match operation {
            HardcopyWorkerOperation::ResolveSource => MAX_WORKER_SNAPSHOT_BYTES,
            HardcopyWorkerOperation::Preview if index.is_multiple_of(2) => {
                MAX_PREVIEW_WORKER_MANIFEST_BYTES
            }
            HardcopyWorkerOperation::Preview => MAX_PREVIEW_WORKER_RGBA_BYTES,
            HardcopyWorkerOperation::Publication if index == 0 => {
                MAX_PUBLICATION_WORKER_MANIFEST_BYTES
            }
            HardcopyWorkerOperation::Publication => MAX_ARTIFACT_BYTES,
            HardcopyWorkerOperation::PackagedPublication if index == 0 => {
                MAX_PUBLICATION_WORKER_MANIFEST_BYTES
            }
            HardcopyWorkerOperation::PackagedPublication => MAX_PUBLICATION_BYTES as usize,
        };
        if length > per_buffer_limit {
            return Err("Hardcopy worker response buffer exceeds its transport budget.".to_owned());
        }
        aggregate = aggregate
            .checked_add(length)
            .ok_or_else(|| "Hardcopy worker response size overflowed.".to_owned())?;
        if matches!(
            operation,
            HardcopyWorkerOperation::Publication | HardcopyWorkerOperation::PackagedPublication
        ) && index != 0
        {
            publication_payload_bytes = publication_payload_bytes
                .checked_add(length)
                .ok_or_else(|| "Hardcopy worker publication payload size overflowed.".to_owned())?;
            if publication_payload_bytes > MAX_PUBLICATION_BYTES as usize {
                return Err(
                    "Hardcopy worker publication exceeds its aggregate artifact budget.".to_owned(),
                );
            }
        }
    }

    let aggregate_limit = match operation {
        HardcopyWorkerOperation::ResolveSource => MAX_WORKER_SNAPSHOT_BYTES,
        HardcopyWorkerOperation::Preview => count
            .saturating_div(2)
            .saturating_mul(MAX_PREVIEW_WORKER_MANIFEST_BYTES + MAX_PREVIEW_WORKER_RGBA_BYTES),
        HardcopyWorkerOperation::Publication => MAX_RESPONSE_TOTAL_BYTES,
        HardcopyWorkerOperation::PackagedPublication => MAX_RESPONSE_TOTAL_BYTES,
    };
    if aggregate > aggregate_limit {
        return Err("Hardcopy worker response exceeds its total transport budget.".to_owned());
    }
    Ok(())
}

/// Execute a bounded worker command, using the host's prepared-source decoder
/// only for source resolution. Rendering and packaging consume authenticated snapshots.
pub fn execute_request_with_source_resolver(
    request: HardcopyWorkerRequest,
    mut buffers: Vec<Vec<u8>>,
    resolve_prepared: impl FnOnce(&[u8]) -> Result<ResolvedHardcopyDocument, HardcopySourceError>,
) -> Result<HardcopyWorkerResponse, String> {
    request.validate()?;
    validate_request_buffer_lengths(buffers.iter().map(Vec::len))?;
    let operation = request.command.operation();
    let expected_response_buffer_count = request.expected_response_buffer_count();
    let output = match request.command {
        HardcopyWorkerCommand::ResolveSource { source_key, scope } => {
            if buffers.len() != 1 {
                return Err(
                    "Hardcopy source-resolution request requires one prepared snapshot.".to_owned(),
                );
            }
            let snapshot = buffers.pop().expect("validated one-buffer request");
            let resolved = resolve_prepared(&snapshot).map_err(|error| error.to_string())?;
            if resolved.source_key() != source_key || resolved.authority().scope() != &scope {
                return Err(
                    "Hardcopy worker resolved a source other than the requested retained identity."
                        .to_owned(),
                );
            }
            vec![
                resolved
                    .worker_snapshot_json()
                    .map_err(|error| error.to_string())?,
            ]
        }
        HardcopyWorkerCommand::Preview {
            plan_id,
            expected_plan_digest,
            setup,
            metadata,
            page_indices,
            dpi,
        } => {
            let source = decode_resolved_source(&mut buffers)?;
            let plan = reconstruct_plan(plan_id, expected_plan_digest, setup, &source)?;
            let previews = HardcopyRenderer::render_preview_pages_resolved(
                &plan,
                &source,
                metadata,
                &page_indices,
                dpi,
                || false,
            )
            .map_err(|error| error.to_string())?;
            if previews.len() != page_indices.len() {
                return Err("Hardcopy worker preview returned an unexpected page count.".to_owned());
            }
            let mut output = Vec::with_capacity(previews.len() * 2);
            for (preview, page_index) in previews.into_iter().zip(page_indices) {
                let transfer = preview
                    .into_worker_transfer(&plan, &source, page_index)
                    .map_err(|error| error.to_string())?;
                let (manifest, rgba) = transfer.into_parts();
                output.push(manifest);
                output.push(rgba);
            }
            output
        }
        HardcopyWorkerCommand::Publication {
            plan_id,
            expected_plan_digest,
            expected_part_count,
            package_multi_part,
            setup,
            metadata,
        } => {
            let source = decode_resolved_source(&mut buffers)?;
            let plan = reconstruct_plan(plan_id, expected_plan_digest, setup, &source)?;
            if publication_part_count(&plan) != expected_part_count {
                return Err(
                    "Hardcopy worker publication part count does not match the exact plan."
                        .to_owned(),
                );
            }
            let publication = HardcopyRenderer::render_resolved(&plan, &source, metadata)
                .map_err(|error| error.to_string())?;
            if package_multi_part {
                package_publication(&plan, &source, publication)?
            } else {
                let transfer = publication
                    .into_worker_transfer(&plan, &source)
                    .map_err(|error| error.to_string())?;
                let (manifest, payloads) = transfer.into_parts();
                let mut output = Vec::with_capacity(payloads.len().saturating_add(1));
                output.push(manifest);
                output.extend(payloads);
                output
            }
        }
    };
    validate_response_buffer_lengths(
        operation,
        Some(expected_response_buffer_count),
        output.iter().map(Vec::len),
    )?;
    Ok(HardcopyWorkerResponse {
        protocol_version: HARDCOPY_WORKER_PROTOCOL_VERSION,
        id: request.id,
        epoch: request.epoch,
        generation: request.generation,
        operation,
        buffers: output,
    })
}

pub fn publication_part_count(plan: &HardcopyPlan) -> usize {
    match plan.setup().render().format() {
        OutputFormat::SvgVector | OutputFormat::Png { .. } => plan.pagination().pages().len(),
        _ => 1,
    }
}

fn package_publication(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    publication: RenderedHardcopyPublication,
) -> Result<Vec<Vec<u8>>, String> {
    if publication.parts().len() < 2 {
        return Err("Hardcopy worker packaging requires multiple rendered artifacts.".to_owned());
    }
    let entries = publication
        .parts()
        .iter()
        .map(|part| (part.suggested_filename(), part.bytes()))
        .collect::<Vec<_>>();
    let bytes = deterministic_stored_zip(&entries).map_err(|error| error.to_string())?;
    if bytes.len() > MAX_PUBLICATION_BYTES as usize {
        return Err("Packaged hardcopy publication exceeds its byte budget.".to_owned());
    }
    let artifact = HardcopyArtifactIdentity::try_new(
        ContentDigest::from_bytes(Sha256::digest(&bytes).into()),
        bytes.len() as u64,
        publication.page_count(),
        publication.format(),
    )
    .map_err(|error| error.to_string())?;
    let manifest = serde_json::to_vec(&PackagedPublicationManifest {
        schema_version: PACKAGED_PUBLICATION_SCHEMA_VERSION,
        plan_content_digest: plan.content_digest(),
        source_content_digest: source.authority().content_digest(),
        artifact,
    })
    .map_err(|error| format!("Could not encode packaged publication manifest: {error}"))?;
    if manifest.len() > MAX_PUBLICATION_WORKER_MANIFEST_BYTES {
        return Err("Packaged publication manifest exceeds its byte budget.".to_owned());
    }
    Ok(vec![manifest, bytes])
}

pub fn decode_packaged_publication(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    buffers: Vec<Vec<u8>>,
) -> Result<PackagedHardcopyPublication, String> {
    validate_response_buffer_lengths(
        HardcopyWorkerOperation::PackagedPublication,
        Some(2),
        buffers.iter().map(Vec::len),
    )?;
    let [manifest_bytes, bytes]: [Vec<u8>; 2] = buffers
        .try_into()
        .map_err(|_| "Packaged hardcopy publication returned the wrong buffer count.".to_owned())?;
    let manifest: PackagedPublicationManifest = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| format!("Could not decode packaged publication manifest: {error}"))?;
    if manifest.schema_version != PACKAGED_PUBLICATION_SCHEMA_VERSION
        || manifest.plan_content_digest != plan.content_digest()
        || manifest.source_content_digest != source.authority().content_digest()
    {
        return Err(
            "Packaged hardcopy publication authority does not match the request.".to_owned(),
        );
    }
    let expected_pages = plan.pagination().pages().len() as u32;
    let expected_format = plan.setup().render().format();
    let digest = ContentDigest::from_bytes(Sha256::digest(&bytes).into());
    if manifest.artifact.content_digest() != digest
        || manifest.artifact.byte_length() != bytes.len() as u64
        || manifest.artifact.page_count() != expected_pages
        || manifest.artifact.format() != expected_format
    {
        return Err("Packaged hardcopy publication identity does not match its bytes.".to_owned());
    }
    Ok(PackagedHardcopyPublication {
        bytes,
        artifact: manifest.artifact,
        page_count: expected_pages,
    })
}

fn decode_resolved_source(buffers: &mut Vec<Vec<u8>>) -> Result<ResolvedHardcopyDocument, String> {
    if buffers.len() != 1 {
        return Err("Hardcopy render request requires one resolved-source snapshot.".to_owned());
    }
    ResolvedHardcopyDocument::from_worker_snapshot_json(
        &buffers.pop().expect("validated one-buffer request"),
    )
    .map_err(|error| error.to_string())
}

fn reconstruct_plan(
    plan_id: HardcopyPlanId,
    expected_digest: ContentDigest,
    setup: HardcopySetup,
    source: &ResolvedHardcopyDocument,
) -> Result<HardcopyPlan, String> {
    let sections = source
        .hardcopy_sections_for_setup(setup.schematic())
        .map_err(|error| error.to_string())?;
    let output_extent = source
        .content_extent_for_setup(setup.schematic())
        .map_err(|error| error.to_string())?;
    let plan = if sections.is_empty() {
        HardcopyPlan::compile_with_id(plan_id, source.authority().clone(), setup, output_extent)
    } else {
        HardcopyPlan::compile_with_id_and_sections(
            plan_id,
            source.authority().clone(),
            setup,
            output_extent,
            sections,
        )
    }
    .map_err(|error| error.to_string())?;
    if plan.content_digest() != expected_digest {
        return Err(
            "Hardcopy worker reconstruction did not match the trusted plan digest.".to_owned(),
        );
    }
    Ok(plan)
}

#[cfg(test)]
mod tests;

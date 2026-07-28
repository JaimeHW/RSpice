//! Dedicated browser-worker protocol for hardcopy source resolution, preview,
//! and publication rendering.
//!
//! The main thread retains the trusted source and plan. Worker requests carry
//! only validated reconstruction inputs plus bounded binary snapshots; worker
//! responses return metadata manifests and raw transferable byte buffers.

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::hardcopy::{
    HardcopyArtifactIdentity, HardcopyPlan, HardcopyPlanId, HardcopyScope, HardcopySetup,
    MAX_PREVIEW_PAGES, OutputFormat,
};
use crate::product::ContentDigest;
use crate::workbench::export_workflow::deterministic_stored_zip;
use crate::workbench::hardcopy_adapters::render::{
    HardcopyRenderer, HardcopySceneMetadata, MAX_ARTIFACT_BYTES, MAX_PREVIEW_WORKER_MANIFEST_BYTES,
    MAX_PREVIEW_WORKER_RGBA_BYTES, MAX_PUBLICATION_BYTES, MAX_PUBLICATION_WORKER_MANIFEST_BYTES,
    RenderedHardcopyPublication,
};
use crate::workbench::hardcopy_adapters::sources::{MAX_WORKER_SNAPSHOT_BYTES, ResolvedHardcopyDocument};

const HARDCOPY_WORKER_PROTOCOL_VERSION: u32 = 1;
const MAX_REQUEST_METADATA_BYTES: usize = MAX_WORKER_SNAPSHOT_BYTES;
const MAX_REQUEST_BUFFERS: usize = 2;
const MAX_RESPONSE_BUFFERS: usize = MAX_PREVIEW_PAGES as usize + 1;
const MAX_RESPONSE_TOTAL_BYTES: usize =
    MAX_PUBLICATION_WORKER_MANIFEST_BYTES + MAX_PUBLICATION_BYTES as usize;
const PACKAGED_PUBLICATION_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum HardcopyWorkerOperation {
    ResolveSource,
    Preview,
    Publication,
    PackagedPublication,
}

impl HardcopyWorkerOperation {
    #[cfg(target_arch = "wasm32")]
    const fn as_str(self) -> &'static str {
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
enum HardcopyWorkerCommand {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct HardcopyWorkerRequest {
    protocol_version: u32,
    id: u32,
    epoch: String,
    generation: String,
    command: HardcopyWorkerCommand,
}

impl HardcopyWorkerRequest {
    fn try_new(
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

struct HardcopyWorkerResponse {
    protocol_version: u32,
    id: u32,
    epoch: String,
    generation: String,
    operation: HardcopyWorkerOperation,
    buffers: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PackagedPublicationManifest {
    schema_version: u32,
    plan_content_digest: ContentDigest,
    source_content_digest: ContentDigest,
    artifact: HardcopyArtifactIdentity,
}

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) struct PackagedBrowserPublication {
    pub(crate) bytes: Vec<u8>,
    pub(crate) artifact: HardcopyArtifactIdentity,
    pub(crate) page_count: u32,
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
fn validate_response_buffer_lengths(
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

fn execute_request(
    request: HardcopyWorkerRequest,
    mut buffers: Vec<Vec<u8>>,
) -> Result<HardcopyWorkerResponse, String> {
    request.validate()?;
    validate_request_buffer_lengths(buffers.iter().map(Vec::len))?;
    let operation = request.command.operation();
    let expected_response_buffer_count = match &request.command {
        HardcopyWorkerCommand::ResolveSource { .. } => 1,
        HardcopyWorkerCommand::Preview { page_indices, .. } => page_indices.len() * 2,
        HardcopyWorkerCommand::Publication {
            expected_part_count,
            package_multi_part,
            ..
        } => {
            if *package_multi_part {
                2
            } else {
                expected_part_count + 1
            }
        }
    };
    let output = match request.command {
        HardcopyWorkerCommand::ResolveSource { source_key, scope } => {
            if buffers.len() != 1 {
                return Err(
                    "Hardcopy source-resolution request requires one prepared snapshot.".to_owned(),
                );
            }
            let snapshot = buffers.pop().expect("validated one-buffer request");
            let prepared =
                crate::workbench::hardcopy_adapters::sources::PreparedRetainedHardcopyResolution::from_worker_snapshot_json(
                    &snapshot,
                )
                .map_err(|error| error.to_string())?;
            let resolved = prepared
                .resolve_owned()
                .map_err(|error| error.to_string())?;
            if resolved.source_key() != source_key || resolved.authority().scope() != &scope {
                return Err(
                    "Hardcopy worker resolved a source other than the requested retained identity."
                        .to_owned(),
                );
            }
            vec![
                resolved
                    .to_worker_snapshot_json()
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

fn publication_part_count(plan: &HardcopyPlan) -> usize {
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
    let bytes = deterministic_stored_zip(&entries)?;
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

#[cfg(any(target_arch = "wasm32", test))]
pub(crate) fn decode_packaged_publication(
    plan: &HardcopyPlan,
    source: &ResolvedHardcopyDocument,
    buffers: Vec<Vec<u8>>,
) -> Result<PackagedBrowserPublication, String> {
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
    Ok(PackagedBrowserPublication {
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
        .hardcopy_sections()
        .map_err(|error| error.to_string())?;
    let plan = if sections.is_empty() {
        HardcopyPlan::compile_with_id(
            plan_id,
            source.authority().clone(),
            setup,
            source.content_extent(),
        )
    } else {
        HardcopyPlan::compile_with_id_and_sections(
            plan_id,
            source.authority().clone(),
            setup,
            source.content_extent(),
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

#[cfg(target_arch = "wasm32")]
pub(crate) fn run_worker_request_value(
    value: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;

    let metadata =
        Reflect::get(&value, &wasm_bindgen::JsValue::from_str("metadata")).map_err(js_error)?;
    let request: HardcopyWorkerRequest = serde_wasm_bindgen::from_value(metadata)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error.to_string()))?;
    let transferred = Reflect::get(&value, &wasm_bindgen::JsValue::from_str("buffers"))
        .map_err(js_error)?
        .dyn_into::<Array>()
        .map_err(|_| {
            wasm_bindgen::JsValue::from_str("Hardcopy worker buffers must be an array.")
        })?;
    if transferred.length() as usize > MAX_REQUEST_BUFFERS {
        return Err(wasm_bindgen::JsValue::from_str(
            "Hardcopy worker request has too many binary buffers.",
        ));
    }
    let mut buffers = Vec::with_capacity(transferred.length() as usize);
    let mut aggregate_bytes = 0usize;
    for index in 0..transferred.length() {
        let view = transferred
            .get(index)
            .dyn_into::<Uint8Array>()
            .map_err(|_| {
                wasm_bindgen::JsValue::from_str(
                    "Hardcopy worker request buffers must be Uint8Array values.",
                )
            })?;
        let byte_length = view.byte_length() as usize;
        aggregate_bytes = aggregate_bytes.checked_add(byte_length).ok_or_else(|| {
            wasm_bindgen::JsValue::from_str("Hardcopy worker request buffer size overflowed.")
        })?;
        if byte_length > MAX_WORKER_SNAPSHOT_BYTES || aggregate_bytes > MAX_WORKER_SNAPSHOT_BYTES {
            return Err(wasm_bindgen::JsValue::from_str(
                "Hardcopy worker request exceeds its binary transport budget.",
            ));
        }
        buffers.push(view.to_vec());
    }
    let response = execute_request(request, buffers)
        .map_err(|error| wasm_bindgen::JsValue::from_str(&error))?;
    let object = Object::new();
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("protocolVersion"),
        &wasm_bindgen::JsValue::from_f64(f64::from(response.protocol_version)),
    )
    .map_err(js_error)?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("id"),
        &wasm_bindgen::JsValue::from_f64(f64::from(response.id)),
    )
    .map_err(js_error)?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("epoch"),
        &wasm_bindgen::JsValue::from_str(&response.epoch),
    )
    .map_err(js_error)?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("generation"),
        &wasm_bindgen::JsValue::from_str(&response.generation),
    )
    .map_err(js_error)?;
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("operation"),
        &wasm_bindgen::JsValue::from_str(response.operation.as_str()),
    )
    .map_err(js_error)?;
    let output = Array::new();
    for buffer in response.buffers {
        output.push(&Uint8Array::from(buffer.as_slice()));
    }
    Reflect::set(
        &object,
        &wasm_bindgen::JsValue::from_str("buffers"),
        &output,
    )
    .map_err(js_error)?;
    Ok(object.into())
}

#[cfg(target_arch = "wasm32")]
fn js_error(error: wasm_bindgen::JsValue) -> wasm_bindgen::JsValue {
    error
}

#[cfg(target_arch = "wasm32")]
mod browser {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use js_sys::{Array, Object, Reflect, Uint8Array};
    use wasm_bindgen::JsCast as _;
    use wasm_bindgen::prelude::*;

    use super::{
        HARDCOPY_WORKER_PROTOCOL_VERSION, HardcopyWorkerCommand, HardcopyWorkerOperation,
        HardcopyWorkerRequest, publication_part_count, validate_response_buffer_lengths,
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) struct HardcopyWorkerTicket {
        pub(crate) id: u32,
        pub(crate) epoch: u64,
        pub(crate) generation: u64,
        pub(crate) operation: HardcopyWorkerOperation,
        expected_buffer_count: Option<usize>,
    }

    struct ActiveHardcopyWorker {
        ticket: HardcopyWorkerTicket,
        worker: web_sys::Worker,
        result: Rc<RefCell<Option<Result<Vec<Vec<u8>>, String>>>>,
        _onmessage: Closure<dyn FnMut(web_sys::MessageEvent)>,
        _onerror: Closure<dyn FnMut(web_sys::ErrorEvent)>,
        _onmessageerror: Closure<dyn FnMut(web_sys::MessageEvent)>,
        deadline_ms: f64,
    }

    impl Drop for ActiveHardcopyWorker {
        fn drop(&mut self) {
            self.worker.set_onmessage(None);
            self.worker.set_onerror(None);
            self.worker.set_onmessageerror(None);
            self.worker.terminate();
        }
    }

    thread_local! {
        static NEXT_REQUEST_ID: Cell<u32> = const { Cell::new(0) };
        static ACTIVE_WORKER: RefCell<Option<ActiveHardcopyWorker>> = const { RefCell::new(None) };
    }

    pub(crate) fn start_source_resolution(
        prepared: crate::workbench::hardcopy_adapters::sources::PreparedRetainedHardcopyResolution,
        source_key: String,
        scope: crate::hardcopy::HardcopyScope,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = prepared
            .to_worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::ResolveSource { source_key, scope },
            vec![snapshot],
            repaint,
        )
    }

    pub(crate) fn start_preview(
        plan: &crate::hardcopy::HardcopyPlan,
        source: &crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument,
        metadata: crate::workbench::hardcopy_adapters::render::HardcopySceneMetadata,
        page_indices: Vec<usize>,
        dpi: u16,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = source
            .to_worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::Preview {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                setup: plan.setup().clone(),
                metadata,
                page_indices,
                dpi,
            },
            vec![snapshot],
            repaint,
        )
    }

    pub(crate) fn start_publication(
        plan: &crate::hardcopy::HardcopyPlan,
        source: &crate::workbench::hardcopy_adapters::sources::ResolvedHardcopyDocument,
        metadata: crate::workbench::hardcopy_adapters::render::HardcopySceneMetadata,
        package_multi_part: bool,
        epoch: u64,
        generation: u64,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        let snapshot = source
            .to_worker_snapshot_json()
            .map_err(|error| error.to_string())?;
        start(
            epoch,
            generation,
            HardcopyWorkerCommand::Publication {
                plan_id: plan.id(),
                expected_plan_digest: plan.content_digest(),
                expected_part_count: publication_part_count(plan),
                package_multi_part,
                setup: plan.setup().clone(),
                metadata,
            },
            vec![snapshot],
            repaint,
        )
    }

    fn start(
        epoch: u64,
        generation: u64,
        command: HardcopyWorkerCommand,
        buffers: Vec<Vec<u8>>,
        repaint: egui::Context,
    ) -> Result<HardcopyWorkerTicket, String> {
        if ACTIVE_WORKER.with(|active| active.borrow().is_some()) {
            return Err("A browser hardcopy worker operation is already active.".to_owned());
        }
        let id = allocate_request_id();
        let request = HardcopyWorkerRequest::try_new(id, epoch, generation, command)?;
        let operation = request.command.operation();
        let expected_buffer_count = match &request.command {
            HardcopyWorkerCommand::ResolveSource { .. } => Some(1),
            HardcopyWorkerCommand::Preview { page_indices, .. } => {
                Some(page_indices.len().saturating_mul(2))
            }
            HardcopyWorkerCommand::Publication {
                expected_part_count,
                package_multi_part,
                ..
            } => Some(if *package_multi_part {
                2
            } else {
                expected_part_count.saturating_add(1)
            }),
        };
        let metadata = serde_wasm_bindgen::to_value(&request)
            .map_err(|error| format!("Could not encode hardcopy worker request: {error}"))?;
        let worker_url = worker_url()?;
        let options = web_sys::WorkerOptions::new();
        options.set_type(web_sys::WorkerType::Module);
        let worker =
            web_sys::Worker::new_with_options(&worker_url, &options).map_err(js_error_message)?;
        let ticket = HardcopyWorkerTicket {
            id,
            epoch,
            generation,
            operation,
            expected_buffer_count,
        };
        let result = Rc::new(RefCell::new(None));

        let message_result = Rc::clone(&result);
        let message_repaint = repaint.clone();
        let onmessage = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |event: web_sys::MessageEvent| {
                let data = event.data();
                let message_type = string_property(&data, "type");
                if message_type.as_deref() == Some("hardcopy-result") {
                    if numeric_property(&data, "id") != Some(id) {
                        complete_once(
                            &message_result,
                            &message_repaint,
                            Err(
                                "Browser hardcopy worker returned a stale outer response id."
                                    .to_owned(),
                            ),
                        );
                        return;
                    }
                    let parsed = Reflect::get(&data, &JsValue::from_str("response"))
                        .map_err(js_error_message)
                        .and_then(|response| parse_response(&response, ticket));
                    complete_once(&message_result, &message_repaint, parsed);
                } else if matches!(
                    message_type.as_deref(),
                    Some("hardcopy-error") | Some("error")
                ) {
                    let response_id = numeric_property(&data, "id").unwrap_or(0);
                    if response_id != id && response_id != 0 {
                        complete_once(
                            &message_result,
                            &message_repaint,
                            Err("Browser hardcopy worker returned a stale outer error id."
                                .to_owned()),
                        );
                        return;
                    }
                    complete_once(
                        &message_result,
                        &message_repaint,
                        Err(string_property(&data, "error")
                            .or_else(|| string_property(&data, "message"))
                            .unwrap_or_else(|| "Browser hardcopy worker failed.".to_owned())),
                    );
                } else if message_type.as_deref() != Some("ready") {
                    complete_once(
                        &message_result,
                        &message_repaint,
                        Err("Browser hardcopy worker returned an unexpected message.".to_owned()),
                    );
                }
            },
        ));
        worker.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));

        let error_result = Rc::clone(&result);
        let error_repaint = repaint.clone();
        let onerror = Closure::<dyn FnMut(web_sys::ErrorEvent)>::wrap(Box::new(
            move |event: web_sys::ErrorEvent| {
                complete_once(
                    &error_result,
                    &error_repaint,
                    Err(if event.message().is_empty() {
                        "Browser hardcopy worker failed.".to_owned()
                    } else {
                        event.message()
                    }),
                );
            },
        ));
        worker.set_onerror(Some(onerror.as_ref().unchecked_ref()));

        let message_error_result = Rc::clone(&result);
        let message_error_repaint = repaint;
        let onmessageerror = Closure::<dyn FnMut(web_sys::MessageEvent)>::wrap(Box::new(
            move |_event: web_sys::MessageEvent| {
                complete_once(
                    &message_error_result,
                    &message_error_repaint,
                    Err("Browser hardcopy worker returned an unreadable message.".to_owned()),
                );
            },
        ));
        worker.set_onmessageerror(Some(onmessageerror.as_ref().unchecked_ref()));

        let message = Object::new();
        Reflect::set(
            &message,
            &JsValue::from_str("type"),
            &JsValue::from_str("run-hardcopy"),
        )
        .map_err(js_error_message)?;
        Reflect::set(
            &message,
            &JsValue::from_str("id"),
            &JsValue::from_f64(f64::from(id)),
        )
        .map_err(js_error_message)?;
        let request_value = Object::new();
        Reflect::set(&request_value, &JsValue::from_str("metadata"), &metadata)
            .map_err(js_error_message)?;
        let views = Array::new();
        let transfer = Array::new();
        for bytes in buffers {
            let view = Uint8Array::from(bytes.as_slice());
            transfer.push(&view.buffer());
            views.push(&view);
        }
        Reflect::set(&request_value, &JsValue::from_str("buffers"), &views)
            .map_err(js_error_message)?;
        Reflect::set(&message, &JsValue::from_str("request"), &request_value)
            .map_err(js_error_message)?;

        ACTIVE_WORKER.with(|active| {
            *active.borrow_mut() = Some(ActiveHardcopyWorker {
                ticket,
                worker: worker.clone(),
                result,
                _onmessage: onmessage,
                _onerror: onerror,
                _onmessageerror: onmessageerror,
                deadline_ms: js_sys::Date::now()
                    + match operation {
                        HardcopyWorkerOperation::ResolveSource
                        | HardcopyWorkerOperation::Preview => 120_000.0,
                        HardcopyWorkerOperation::Publication
                        | HardcopyWorkerOperation::PackagedPublication => 1_800_000.0,
                    },
            });
        });
        if let Err(error) = worker.post_message_with_transfer(&message, &transfer) {
            cancel();
            return Err(format!(
                "Could not dispatch browser hardcopy work: {}",
                js_error_message(error)
            ));
        }
        Ok(ticket)
    }

    pub(crate) fn poll(expected: HardcopyWorkerTicket) -> Option<Result<Vec<Vec<u8>>, String>> {
        ACTIVE_WORKER.with(|active| {
            let mut active = active.borrow_mut();
            if active.as_ref().map(|worker| worker.ticket) != Some(expected) {
                return None;
            }
            if active
                .as_ref()
                .is_some_and(|worker| js_sys::Date::now() >= worker.deadline_ms)
            {
                active.take();
                return Some(Err(
                    "Browser hardcopy worker exceeded its bounded execution deadline.".to_owned(),
                ));
            }
            let result = active
                .as_ref()
                .and_then(|worker| worker.result.borrow_mut().take());
            if result.is_some() {
                active.take();
            }
            result
        })
    }

    pub(crate) fn cancel() {
        ACTIVE_WORKER.with(|active| {
            active.borrow_mut().take();
        });
    }

    pub(crate) fn is_active() -> bool {
        ACTIVE_WORKER.with(|active| active.borrow().is_some())
    }

    fn parse_response(
        response: &JsValue,
        expected: HardcopyWorkerTicket,
    ) -> Result<Vec<Vec<u8>>, String> {
        if numeric_property(response, "protocolVersion") != Some(HARDCOPY_WORKER_PROTOCOL_VERSION) {
            return Err("Browser hardcopy worker returned an unsupported protocol.".to_owned());
        }
        if numeric_property(response, "id") != Some(expected.id) {
            return Err("Browser hardcopy worker returned a stale response id.".to_owned());
        }
        if string_property(response, "epoch") != Some(expected.epoch.to_string()) {
            return Err("Browser hardcopy worker returned a stale epoch.".to_owned());
        }
        if string_property(response, "generation") != Some(expected.generation.to_string()) {
            return Err("Browser hardcopy worker returned a stale generation.".to_owned());
        }
        if string_property(response, "operation").as_deref() != Some(expected.operation.as_str()) {
            return Err("Browser hardcopy worker returned the wrong operation.".to_owned());
        }
        let buffers = Reflect::get(response, &JsValue::from_str("buffers"))
            .map_err(js_error_message)?
            .dyn_into::<Array>()
            .map_err(|_| "Browser hardcopy response buffers are not an array.".to_owned())?;
        let buffer_count = buffers.length() as usize;
        let mut views = Vec::with_capacity(buffer_count);
        let mut lengths = Vec::with_capacity(buffer_count);
        for index in 0..buffers.length() {
            let view = buffers.get(index).dyn_into::<Uint8Array>().map_err(|_| {
                "Browser hardcopy response contains a non-Uint8Array buffer.".to_owned()
            })?;
            lengths.push(view.byte_length() as usize);
            views.push(view);
        }
        validate_response_buffer_lengths(
            expected.operation,
            expected.expected_buffer_count,
            lengths,
        )?;
        Ok(views.into_iter().map(|view| view.to_vec()).collect())
    }

    fn complete_once(
        slot: &RefCell<Option<Result<Vec<Vec<u8>>, String>>>,
        repaint: &egui::Context,
        result: Result<Vec<Vec<u8>>, String>,
    ) {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            *slot = Some(result);
            repaint.request_repaint();
        }
    }

    fn allocate_request_id() -> u32 {
        NEXT_REQUEST_ID.with(|next| {
            let value = next.get().wrapping_add(1).max(1);
            next.set(value);
            value
        })
    }

    fn worker_url() -> Result<String, String> {
        Reflect::get(
            &js_sys::global(),
            &JsValue::from_str("__RSPICE_SIM_WORKER_URL"),
        )
        .map_err(js_error_message)?
        .as_string()
        .filter(|url| !url.trim().is_empty())
        .ok_or_else(|| "Browser hardcopy worker URL is unavailable.".to_owned())
    }

    fn string_property(value: &JsValue, property: &str) -> Option<String> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_string())
    }

    fn numeric_property(value: &JsValue, property: &str) -> Option<u32> {
        Reflect::get(value, &JsValue::from_str(property))
            .ok()
            .and_then(|value| value.as_f64())
            .filter(|value| {
                value.is_finite()
                    && *value >= 1.0
                    && *value <= f64::from(u32::MAX)
                    && value.fract() == 0.0
            })
            .map(|value| value as u32)
    }

    fn js_error_message(error: JsValue) -> String {
        error
            .as_string()
            .or_else(|| {
                Reflect::get(&error, &JsValue::from_str("message"))
                    .ok()
                    .and_then(|message| message.as_string())
            })
            .unwrap_or_else(|| "unknown JavaScript error".to_owned())
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) use browser::{
    HardcopyWorkerTicket, cancel, is_active, poll, start_preview, start_publication,
    start_source_resolution,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hardcopy::{
        BackgroundMode, ColorMapping, FontPolicy, RenderSetup, RenderTarget, ScaleMode,
    };
    use crate::state::{Point, Wire};
    use crate::workbench::AppState;
    use crate::workbench::hardcopy_adapters::render::{HardcopyPreviewPage, RenderedHardcopyPublication};
    use crate::workbench::hardcopy_adapters::sources::{
        PreparedRetainedHardcopyResolution, prepare_retained_hardcopy_resolution,
        resolve_retained_hardcopy_source,
    };
    use crate::workbench::state::WorkspaceDocumentId;

    struct WorkerFixture {
        prepared: PreparedRetainedHardcopyResolution,
        source: ResolvedHardcopyDocument,
        source_key: String,
        scope: HardcopyScope,
    }

    fn worker_fixture() -> WorkerFixture {
        worker_fixture_with_wire_endpoint(80)
    }

    fn worker_fixture_with_wire_endpoint(endpoint_x: i32) -> WorkerFixture {
        let mut state = AppState::default();
        state.schematic.wires.push(Wire::segment(
            881,
            Point::new(-20, 5),
            Point::new(endpoint_x, 5),
        ));
        let active_view = state.workspace.active_view.clone();
        state
            .workbench
            .documents
            .activate(WorkspaceDocumentId::CellView(active_view));
        let source_key = format!(
            "project:{}:cell-view:{}",
            state.workspace.project.id().as_uuid(),
            state.workspace.active_key()
        );
        let scope = HardcopyScope::ActiveDocument;
        let source = resolve_retained_hardcopy_source(&state, &source_key, scope.clone())
            .expect("fixture source resolves");
        let prepared = prepare_retained_hardcopy_resolution(&state, &source_key, scope.clone())
            .expect("fixture source prepares");
        WorkerFixture {
            prepared,
            source,
            source_key,
            scope,
        }
    }

    fn two_page_setup() -> HardcopySetup {
        let defaults = HardcopySetup::default();
        HardcopySetup::try_new(
            defaults.physical_page().clone(),
            ScaleMode::EngineeringOneToOne,
            defaults.tiling(),
            defaults.render().clone(),
            defaults.decorations().clone(),
            defaults.print_mapping().clone(),
        )
        .expect("two-page setup is valid")
    }

    fn two_page_svg_setup() -> HardcopySetup {
        let defaults = two_page_setup();
        HardcopySetup::try_new(
            defaults.physical_page().clone(),
            defaults.scale(),
            defaults.tiling(),
            RenderSetup::try_new(
                RenderTarget::ExportArtifact,
                OutputFormat::SvgVector,
                ColorMapping::PrintSafeEngineeringPalette,
                BackgroundMode::White,
                FontPolicy::new(true, true),
                true,
            )
            .expect("SVG render setup is valid"),
            defaults.decorations().clone(),
            defaults.print_mapping().clone(),
        )
        .expect("two-page SVG setup is valid")
    }

    fn fixture_plan(source: &ResolvedHardcopyDocument, setup: HardcopySetup) -> HardcopyPlan {
        let plan_id = HardcopyPlanId::new();
        let sections = source
            .hardcopy_sections()
            .expect("fixture sections resolve");
        if sections.is_empty() {
            HardcopyPlan::compile_with_id(
                plan_id,
                source.authority().clone(),
                setup,
                source.content_extent(),
            )
        } else {
            HardcopyPlan::compile_with_id_and_sections(
                plan_id,
                source.authority().clone(),
                setup,
                source.content_extent(),
                sections,
            )
        }
        .expect("fixture plan compiles")
    }

    fn fixture_metadata(source: &ResolvedHardcopyDocument) -> HardcopySceneMetadata {
        HardcopySceneMetadata::for_resolved_source(source, "RSpice worker tests")
            .expect("fixture metadata is valid")
    }

    fn request(
        id: u32,
        epoch: u64,
        generation: u64,
        command: HardcopyWorkerCommand,
    ) -> HardcopyWorkerRequest {
        HardcopyWorkerRequest::try_new(id, epoch, generation, command)
            .expect("fixture request is valid")
    }

    fn execution_error(result: Result<HardcopyWorkerResponse, String>, context: &str) -> String {
        match result {
            Ok(_) => panic!("{context}"),
            Err(error) => error,
        }
    }

    #[test]
    fn request_rejects_protocol_counter_and_unknown_field_drift() {
        let valid = request(
            17,
            0,
            29,
            HardcopyWorkerCommand::ResolveSource {
                source_key: "retained:test-source".to_owned(),
                scope: HardcopyScope::ActiveDocument,
            },
        );
        assert_eq!(valid.epoch, "0");
        assert_eq!(valid.generation, "29");
        valid.validate().expect("canonical request validates");
        let maximum = request(
            u32::MAX,
            u64::MAX,
            u64::MAX,
            HardcopyWorkerCommand::ResolveSource {
                source_key: "retained:test-source".to_owned(),
                scope: HardcopyScope::ActiveDocument,
            },
        );
        assert_eq!(maximum.epoch, u64::MAX.to_string());
        assert_eq!(maximum.generation, u64::MAX.to_string());

        let mut wrong_protocol = valid.clone();
        wrong_protocol.protocol_version = HARDCOPY_WORKER_PROTOCOL_VERSION + 1;
        assert!(wrong_protocol.validate().is_err());

        let mut zero_id = valid.clone();
        zero_id.id = 0;
        assert!(zero_id.validate().is_err());

        for epoch in ["00", "01", "+1", "-1", " 1"] {
            let mut invalid = valid.clone();
            invalid.epoch = epoch.to_owned();
            assert!(
                invalid.validate().is_err(),
                "noncanonical epoch {epoch:?} must fail"
            );
        }
        for generation in ["0", "00", "01", "+1", "-1", " 1"] {
            let mut invalid = valid.clone();
            invalid.generation = generation.to_owned();
            assert!(
                invalid.validate().is_err(),
                "noncanonical generation {generation:?} must fail"
            );
        }

        let mut unknown_request = serde_json::to_value(&valid).expect("request serializes");
        unknown_request
            .as_object_mut()
            .expect("request is an object")
            .insert("futureField".to_owned(), serde_json::json!(true));
        assert!(
            serde_json::from_value::<HardcopyWorkerRequest>(unknown_request).is_err(),
            "unknown request metadata must fail closed"
        );

        let mut unknown_command = serde_json::to_value(&valid).expect("request serializes");
        unknown_command["command"]
            .as_object_mut()
            .expect("command is an object")
            .insert("future-field".to_owned(), serde_json::json!(42));
        assert!(
            serde_json::from_value::<HardcopyWorkerRequest>(unknown_command).is_err(),
            "unknown operation metadata must fail closed"
        );
    }

    #[test]
    fn metadata_and_request_buffer_budgets_are_exact_and_overflow_safe() {
        assert!(validate_metadata_length(0).is_ok());
        assert!(validate_metadata_length(MAX_REQUEST_METADATA_BYTES).is_ok());
        assert!(validate_metadata_length(MAX_REQUEST_METADATA_BYTES + 1).is_err());

        assert!(validate_request_buffer_lengths([]).is_ok());
        assert!(validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES]).is_ok());
        assert!(
            validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES.saturating_sub(1), 1,])
                .is_ok()
        );
        assert!(
            validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES.saturating_sub(1), 2,])
                .is_err()
        );
        assert!(validate_request_buffer_lengths([MAX_WORKER_SNAPSHOT_BYTES + 1]).is_err());
        assert!(validate_request_buffer_lengths([0, 0, 0]).is_err());
        assert!(validate_request_buffer_lengths([usize::MAX, 1]).is_err());
    }

    #[test]
    fn response_preflight_enforces_operation_cardinality_and_transport_budgets() {
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::ResolveSource,
                Some(1),
                [MAX_WORKER_SNAPSHOT_BYTES],
            )
            .is_ok()
        );
        assert!(
            validate_response_buffer_lengths(HardcopyWorkerOperation::ResolveSource, Some(1), [],)
                .is_err()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::ResolveSource,
                Some(1),
                [1, 1],
            )
            .is_err()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::ResolveSource,
                Some(1),
                [MAX_WORKER_SNAPSHOT_BYTES + 1],
            )
            .is_err()
        );

        let one_preview = [
            MAX_PREVIEW_WORKER_MANIFEST_BYTES,
            MAX_PREVIEW_WORKER_RGBA_BYTES,
        ];
        let two_previews = [
            MAX_PREVIEW_WORKER_MANIFEST_BYTES,
            MAX_PREVIEW_WORKER_RGBA_BYTES,
            MAX_PREVIEW_WORKER_MANIFEST_BYTES,
            MAX_PREVIEW_WORKER_RGBA_BYTES,
        ];
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Preview,
                Some(2),
                one_preview,
            )
            .is_ok()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Preview,
                Some(4),
                two_previews,
            )
            .is_ok()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Preview,
                Some(4),
                one_preview,
            )
            .is_err()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Preview,
                Some(2),
                [MAX_PREVIEW_WORKER_MANIFEST_BYTES + 1, 0],
            )
            .is_err()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Preview,
                Some(2),
                [0, MAX_PREVIEW_WORKER_RGBA_BYTES + 1],
            )
            .is_err()
        );
        assert!(
            validate_response_buffer_lengths(HardcopyWorkerOperation::Preview, Some(3), [0, 0, 0],)
                .is_err()
        );

        let publication_bytes = MAX_PUBLICATION_BYTES as usize;
        let second_maximum_part = publication_bytes
            .checked_sub(MAX_ARTIFACT_BYTES)
            .expect("publication aggregate exceeds one artifact");
        assert!(second_maximum_part <= MAX_ARTIFACT_BYTES);
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Publication,
                Some(3),
                [
                    MAX_PUBLICATION_WORKER_MANIFEST_BYTES,
                    MAX_ARTIFACT_BYTES,
                    second_maximum_part,
                ],
            )
            .is_ok()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Publication,
                Some(2),
                [MAX_PUBLICATION_WORKER_MANIFEST_BYTES + 1, 0],
            )
            .is_err()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Publication,
                Some(2),
                [0, MAX_ARTIFACT_BYTES + 1],
            )
            .is_err()
        );
        assert!(
            validate_response_buffer_lengths(
                HardcopyWorkerOperation::Publication,
                Some(4),
                [0, MAX_ARTIFACT_BYTES, second_maximum_part, 1,],
            )
            .is_err()
        );
        assert!(
            validate_response_buffer_lengths(HardcopyWorkerOperation::Publication, Some(1), [0],)
                .is_err()
        );
    }

    #[test]
    fn resolve_source_preserves_exact_identity_and_response_ticket() {
        let fixture = worker_fixture();
        let expected = fixture.source.clone();
        let response = execute_request(
            request(
                31,
                44,
                55,
                HardcopyWorkerCommand::ResolveSource {
                    source_key: fixture.source_key,
                    scope: fixture.scope,
                },
            ),
            vec![
                fixture
                    .prepared
                    .to_worker_snapshot_json()
                    .expect("prepared snapshot encodes"),
            ],
        )
        .expect("source worker request succeeds");

        assert_eq!(response.protocol_version, HARDCOPY_WORKER_PROTOCOL_VERSION);
        assert_eq!(response.id, 31);
        assert_eq!(response.epoch, "44");
        assert_eq!(response.generation, "55");
        assert_eq!(response.operation, HardcopyWorkerOperation::ResolveSource);
        assert_eq!(response.buffers.len(), 1);
        let restored = ResolvedHardcopyDocument::from_worker_snapshot_json(&response.buffers[0])
            .expect("resolved worker snapshot decodes");
        assert_eq!(restored, expected);
    }

    #[test]
    fn resolve_source_rejects_requested_key_and_scope_mismatch() {
        let wrong_key = worker_fixture();
        let error = execution_error(
            execute_request(
                request(
                    1,
                    1,
                    1,
                    HardcopyWorkerCommand::ResolveSource {
                        source_key: format!("{}:other", wrong_key.source_key),
                        scope: wrong_key.scope,
                    },
                ),
                vec![
                    wrong_key
                        .prepared
                        .to_worker_snapshot_json()
                        .expect("prepared snapshot encodes"),
                ],
            ),
            "a different requested source key must fail",
        );
        assert!(error.contains("other than the requested retained identity"));

        let wrong_scope = worker_fixture();
        let error = execution_error(
            execute_request(
                request(
                    2,
                    1,
                    1,
                    HardcopyWorkerCommand::ResolveSource {
                        source_key: wrong_scope.source_key,
                        scope: HardcopyScope::CurrentSheet,
                    },
                ),
                vec![
                    wrong_scope
                        .prepared
                        .to_worker_snapshot_json()
                        .expect("prepared snapshot encodes"),
                ],
            ),
            "a different requested source scope must fail",
        );
        assert!(error.contains("other than the requested retained identity"));
    }

    #[test]
    fn preview_operation_returns_decodable_transfer_and_rejects_digest_mismatch() {
        let fixture = worker_fixture();
        let setup = HardcopySetup::default();
        let plan = fixture_plan(&fixture.source, setup.clone());
        let dpi = 72;
        let response = execute_request(
            request(
                90,
                12,
                34,
                HardcopyWorkerCommand::Preview {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    setup: setup.clone(),
                    metadata: fixture_metadata(&fixture.source),
                    page_indices: vec![0],
                    dpi,
                },
            ),
            vec![
                fixture
                    .source
                    .to_worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("preview worker request succeeds");

        assert_eq!(response.operation, HardcopyWorkerOperation::Preview);
        assert_eq!(response.id, 90);
        assert_eq!(response.epoch, "12");
        assert_eq!(response.generation, "34");
        assert_eq!(response.buffers.len(), 2);
        let preview = HardcopyPreviewPage::from_worker_transfer(
            &plan,
            &fixture.source,
            0,
            dpi,
            &response.buffers[0],
            response.buffers[1].clone(),
        )
        .expect("preview transfer authenticates and decodes");
        assert_eq!(preview.page_number(), 1);
        assert_eq!(preview.dpi(), dpi);
        assert!(!preview.rgba().is_empty());

        let mismatch = execution_error(
            execute_request(
                request(
                    91,
                    12,
                    35,
                    HardcopyWorkerCommand::Preview {
                        plan_id: plan.id(),
                        expected_plan_digest: ContentDigest::from_bytes([0xa5; 32]),
                        setup,
                        metadata: fixture_metadata(&fixture.source),
                        page_indices: vec![0],
                        dpi,
                    },
                ),
                vec![
                    fixture
                        .source
                        .to_worker_snapshot_json()
                        .expect("source snapshot encodes"),
                ],
            ),
            "a mismatched trusted plan digest must fail",
        );
        assert!(mismatch.contains("trusted plan digest"));
    }

    #[test]
    fn preview_operation_preserves_two_page_pair_order_and_count() {
        let fixture = worker_fixture_with_wire_endpoint(1_600);
        let setup = two_page_setup();
        let plan = fixture_plan(&fixture.source, setup.clone());
        assert_eq!(
            plan.pagination().pages().len(),
            2,
            "fixture must remain an exact two-page plan"
        );
        let dpi = 72;
        let response = execute_request(
            request(
                92,
                13,
                36,
                HardcopyWorkerCommand::Preview {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    setup,
                    metadata: fixture_metadata(&fixture.source),
                    page_indices: vec![0, 1],
                    dpi,
                },
            ),
            vec![
                fixture
                    .source
                    .to_worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("two-page preview worker request succeeds");

        assert_eq!(response.buffers.len(), 4);
        let previews = [0usize, 1]
            .into_iter()
            .map(|page_index| {
                let buffer_index = page_index * 2;
                HardcopyPreviewPage::from_worker_transfer(
                    &plan,
                    &fixture.source,
                    page_index,
                    dpi,
                    &response.buffers[buffer_index],
                    response.buffers[buffer_index + 1].clone(),
                )
                .expect("ordered preview pair authenticates")
            })
            .collect::<Vec<_>>();
        assert_eq!(
            previews
                .iter()
                .map(HardcopyPreviewPage::page_number)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        assert_ne!(previews[0].coordinate(), previews[1].coordinate());
    }

    #[test]
    fn publication_operation_returns_exact_decodable_parts_and_rejects_count_mismatch() {
        let fixture = worker_fixture();
        let setup = HardcopySetup::default();
        let plan = fixture_plan(&fixture.source, setup.clone());
        let expected_part_count = publication_part_count(&plan);
        assert_eq!(expected_part_count, 1, "PDF publication is one artifact");

        let response = execute_request(
            request(
                101,
                22,
                33,
                HardcopyWorkerCommand::Publication {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    expected_part_count,
                    package_multi_part: false,
                    setup: setup.clone(),
                    metadata: fixture_metadata(&fixture.source),
                },
            ),
            vec![
                fixture
                    .source
                    .to_worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("publication worker request succeeds");

        assert_eq!(response.operation, HardcopyWorkerOperation::Publication);
        assert_eq!(response.id, 101);
        assert_eq!(response.epoch, "22");
        assert_eq!(response.generation, "33");
        assert_eq!(response.buffers.len(), expected_part_count + 1);
        let publication = RenderedHardcopyPublication::from_worker_transfer(
            &plan,
            &fixture.source,
            &response.buffers[0],
            response.buffers[1..].to_vec(),
        )
        .expect("publication transfer authenticates and decodes");
        assert_eq!(publication.format(), OutputFormat::PdfVector);
        assert_eq!(
            publication.page_count(),
            plan.pagination().pages().len() as u32
        );

        let mismatch = execution_error(
            execute_request(
                request(
                    102,
                    22,
                    34,
                    HardcopyWorkerCommand::Publication {
                        plan_id: plan.id(),
                        expected_plan_digest: plan.content_digest(),
                        expected_part_count: expected_part_count + 1,
                        package_multi_part: false,
                        setup,
                        metadata: fixture_metadata(&fixture.source),
                    },
                ),
                vec![
                    fixture
                        .source
                        .to_worker_snapshot_json()
                        .expect("source snapshot encodes"),
                ],
            ),
            "an inexact publication part count must fail",
        );
        assert!(mismatch.contains("part count does not match the exact plan"));
    }

    #[test]
    fn packaged_publication_is_deterministic_authority_bound_and_tamper_evident() {
        // Schematic coordinates are 254 um per unit. Extend the authenticated
        // source beyond one Letter-landscape printable viewport so this is a
        // real two-part SVG publication rather than a mislabeled fixture.
        let fixture = worker_fixture_with_wire_endpoint(1_420);
        let setup = two_page_svg_setup();
        let plan = fixture_plan(&fixture.source, setup.clone());
        let expected_part_count = publication_part_count(&plan);
        assert!(
            expected_part_count >= 2,
            "fixture must produce a multi-page SVG publication"
        );
        let metadata = fixture_metadata(&fixture.source);
        let response = execute_request(
            request(
                111,
                24,
                35,
                HardcopyWorkerCommand::Publication {
                    plan_id: plan.id(),
                    expected_plan_digest: plan.content_digest(),
                    expected_part_count,
                    package_multi_part: true,
                    setup,
                    metadata: metadata.clone(),
                },
            ),
            vec![
                fixture
                    .source
                    .to_worker_snapshot_json()
                    .expect("source snapshot encodes"),
            ],
        )
        .expect("packaged publication worker request succeeds");
        assert_eq!(
            response.operation,
            HardcopyWorkerOperation::PackagedPublication
        );
        assert_eq!(response.buffers.len(), 2);

        let packaged =
            decode_packaged_publication(&plan, &fixture.source, response.buffers.clone())
                .expect("packaged transfer authenticates");
        let independently_rendered =
            HardcopyRenderer::render_resolved(&plan, &fixture.source, metadata)
                .expect("independent publication renders");
        let expected_entries = independently_rendered
            .parts()
            .iter()
            .map(|part| (part.suggested_filename(), part.bytes()))
            .collect::<Vec<_>>();
        let expected_zip =
            deterministic_stored_zip(&expected_entries).expect("expected ZIP assembles");
        assert_eq!(
            packaged.bytes, expected_zip,
            "ZIP entry order, names, and bytes must match the ordered rendered publication"
        );
        assert_eq!(
            packaged.artifact.content_digest(),
            ContentDigest::from_bytes(Sha256::digest(&packaged.bytes).into())
        );
        assert_eq!(packaged.page_count, plan.pagination().pages().len() as u32);

        let mut tampered_bytes = response.buffers.clone();
        tampered_bytes[1][0] ^= 0x01;
        assert!(
            decode_packaged_publication(&plan, &fixture.source, tampered_bytes).is_err(),
            "tampered ZIP bytes must fail artifact authentication"
        );

        let mut manifest: PackagedPublicationManifest =
            serde_json::from_slice(&response.buffers[0]).expect("manifest decodes");
        manifest.plan_content_digest =
            ContentDigest::from_bytes(Sha256::digest(b"wrong hardcopy plan").into());
        let mut tampered_manifest = response.buffers;
        tampered_manifest[0] = serde_json::to_vec(&manifest).expect("tampered manifest encodes");
        assert!(
            decode_packaged_publication(&plan, &fixture.source, tampered_manifest).is_err(),
            "tampered authority metadata must fail closed"
        );
    }
}

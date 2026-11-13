//! Browser-worker PDK import: validates a package archive against the
//! configuration the page sent and returns the candidate configuration.

use crate::pdk::{PdkConfig, ValidatedPdkTechnologyPackage};
use rspice_model_library::pdk::{PdkAdministrativeAuthority, contracts::MAX_PDK_ARCHIVE_BYTES};

const BROWSER_PDK_IMPORT_PROTOCOL_VERSION: u16 = 1;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkImportMetadata {
    protocol_version: u16,
    config: PdkConfig,
    authority: PdkAdministrativeAuthority,
    reason: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct BrowserPdkImportPayload {
    protocol_version: u16,
    config: PdkConfig,
    validated_packages: Vec<ValidatedPdkTechnologyPackage>,
    package_id: String,
    revision: String,
    sequence: u64,
}

pub fn run_pdk_import_worker_request_value(
    request: wasm_bindgen::JsValue,
) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue> {
    use js_sys::{Object, Reflect, Uint8Array};
    use wasm_bindgen::JsValue;

    let metadata = Uint8Array::new(&Reflect::get(
        &request,
        &JsValue::from_str("metadataBytes"),
    )?);
    let mut metadata_bytes = vec![
        0;
        usize::try_from(metadata.length()).map_err(|_| {
            JsValue::from_str("PDK import metadata exceeds host limits.")
        })?
    ];
    metadata.copy_to(&mut metadata_bytes);
    let metadata: BrowserPdkImportMetadata = serde_json::from_slice(&metadata_bytes)
        .map_err(|error| JsValue::from_str(&format!("Invalid PDK import metadata: {error}")))?;
    if metadata.protocol_version != BROWSER_PDK_IMPORT_PROTOCOL_VERSION {
        return Err(JsValue::from_str(&format!(
            "Unsupported PDK import protocol {}.",
            metadata.protocol_version
        )));
    }
    let archive = Uint8Array::new(&Reflect::get(&request, &JsValue::from_str("archiveBytes"))?);
    let archive_length = usize::try_from(archive.length())
        .map_err(|_| JsValue::from_str("PDK package exceeds host limits."))?;
    if archive_length == 0 || archive_length > MAX_PDK_ARCHIVE_BYTES {
        return Err(JsValue::from_str(
            "PDK package size is outside supported limits.",
        ));
    }
    let mut archive_bytes = vec![0; archive_length];
    archive.copy_to(&mut archive_bytes);

    let mut config = metadata.config;
    if !config.technology_registry.archives().is_empty() {
        config
            .technology_registry
            .revalidate_installed()
            .map_err(|errors| JsValue::from_str(&errors.join("; ")))?;
    }
    let receipt = config
        .technology_registry
        .install_archive_bytes(&archive_bytes, &metadata.authority, &metadata.reason)
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let validated_packages = config.technology_registry.take_worker_validated_packages();
    let payload = BrowserPdkImportPayload {
        protocol_version: BROWSER_PDK_IMPORT_PROTOCOL_VERSION,
        config,
        validated_packages,
        package_id: receipt.target.package_id,
        revision: receipt.target.revision,
        sequence: receipt.sequence,
    };
    let encoded = serde_json::to_vec(&payload)
        .map_err(|error| JsValue::from_str(&format!("Could not encode PDK candidate: {error}")))?;
    if encoded.is_empty() || encoded.len() > 256 * 1024 * 1024 {
        return Err(JsValue::from_str(
            "Validated PDK candidate exceeds the browser worker response limit.",
        ));
    }
    let bytes = Uint8Array::new_with_length(
        u32::try_from(encoded.len())
            .map_err(|_| JsValue::from_str("PDK candidate exceeds browser array limits."))?,
    );
    bytes.copy_from(&encoded);
    let response = Object::new();
    Reflect::set(
        &response,
        &JsValue::from_str("protocolVersion"),
        &JsValue::from_f64(f64::from(BROWSER_PDK_IMPORT_PROTOCOL_VERSION)),
    )?;
    Reflect::set(&response, &JsValue::from_str("payloadBytes"), &bytes)?;
    Ok(response.into())
}

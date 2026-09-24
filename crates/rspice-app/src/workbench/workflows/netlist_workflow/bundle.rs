//! Reading an RSpice generated-netlist bundle.
//!
//! Every entry is verified against the manifest digest before any of it
//! reaches the project, and entry names are validated against traversal.

#[derive(Debug)]
pub(super) struct ImportedNetlistBundle {
    pub(super) source: String,
    pub(super) dependencies: Vec<crate::state::DependencyMetadata>,
    pub(super) expanded_source: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GeneratedBundleManifest {
    schema: String,
    main: String,
    main_content_digest: String,
    dialect: String,
    retained_generated_source: String,
    generated_content_digest: String,
    generator: String,
    input_revision: u64,
    input_digest: String,
    source_map: Option<String>,
    dependencies: Vec<GeneratedBundleDependency>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GeneratedBundleDependency {
    requested_locator: String,
    logical_identity: String,
    #[serde(default)]
    authority: crate::state::DependencySourceAuthority,
    bundle_entry: String,
    content_digest: String,
    retained_entry: String,
    retained_content_digest: Option<String>,
    edge: GeneratedBundleEdge,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GeneratedBundleEdge {
    owner: String,
    include_index: usize,
}

pub(super) fn parse_generated_netlist_bundle(
    bytes: &[u8],
) -> Result<ImportedNetlistBundle, String> {
    use std::collections::{BTreeMap, HashMap, HashSet};

    const MAX_ARCHIVE_ENTRIES: usize = 20_004;
    let entries = read_stored_zip_entries(bytes, MAX_ARCHIVE_ENTRIES)?;
    let manifest_bytes = entries
        .get("manifest.json")
        .ok_or_else(|| "Netlist bundle has no manifest.json entry.".to_owned())?;
    let manifest: GeneratedBundleManifest = serde_json::from_slice(manifest_bytes)
        .map_err(|error| format!("Netlist bundle manifest is invalid: {error}"))?;
    if manifest.schema != "rspice-generated-netlist-bundle/v1" {
        return Err(format!(
            "Unsupported netlist bundle schema '{}'.",
            manifest.schema
        ));
    }
    if manifest.dependencies.len() > 10_000 {
        return Err("Netlist bundle declares more than 10,000 dependencies.".to_owned());
    }
    if manifest.input_revision == 0
        || manifest.generator.trim().is_empty()
        || manifest.dialect.trim().is_empty()
    {
        return Err("Netlist bundle manifest has an invalid generation identity.".to_owned());
    }
    validate_digest_literal(&manifest.input_digest, "input digest")?;
    let main = bundle_entry(&entries, &manifest.main)?;
    verify_content_digest(
        main,
        &manifest.main_content_digest,
        "materialized main deck",
    )?;
    let retained_root = bundle_entry(&entries, &manifest.retained_generated_source)?;
    verify_content_digest(
        retained_root,
        &manifest.generated_content_digest,
        "retained generated deck",
    )?;
    let source = std::str::from_utf8(retained_root)
        .map_err(|error| format!("Retained generated deck is not UTF-8: {error}"))?
        .to_owned();

    let mut expected_entries = HashSet::new();
    expected_entries.insert("manifest.json".to_owned());
    expected_entries.insert(manifest.main.clone());
    expected_entries.insert(manifest.retained_generated_source.clone());
    if let Some(source_map) = manifest.source_map.as_deref() {
        let map = bundle_entry(&entries, source_map)?;
        let value: serde_json::Value = serde_json::from_slice(map)
            .map_err(|error| format!("Generated source map is invalid: {error}"))?;
        if value.get("schema").and_then(serde_json::Value::as_str)
            != Some("rspice-generated-source-map/v1")
            || value
                .get("generated_content_digest")
                .and_then(serde_json::Value::as_str)
                != Some(manifest.generated_content_digest.as_str())
        {
            return Err(
                "Generated source map does not authenticate the retained generated deck."
                    .to_owned(),
            );
        }
        expected_entries.insert(source_map.to_owned());
    }

    let mut locators = HashMap::with_capacity(manifest.dependencies.len());
    let mut retained_sources = BTreeMap::new();
    for dependency in &manifest.dependencies {
        validate_bundle_entry_name(&dependency.bundle_entry)?;
        validate_bundle_entry_name(&dependency.retained_entry)?;
        let materialized = bundle_entry(&entries, &dependency.bundle_entry)?;
        verify_content_digest(
            materialized,
            &dependency.content_digest,
            &format!("materialized dependency '{}'", dependency.logical_identity),
        )?;
        let retained = bundle_entry(&entries, &dependency.retained_entry)?;
        let retained_digest = dependency
            .retained_content_digest
            .as_deref()
            .ok_or_else(|| {
                format!(
                    "Dependency '{}' has no retained content digest.",
                    dependency.logical_identity
                )
            })?;
        verify_content_digest(
            retained,
            retained_digest,
            &format!("retained dependency '{}'", dependency.logical_identity),
        )?;
        let retained = std::str::from_utf8(retained).map_err(|error| {
            format!(
                "Retained dependency '{}' is not UTF-8: {error}",
                dependency.logical_identity
            )
        })?;
        let display_name = dependency
            .logical_identity
            .rsplit(['/', '\\'])
            .find(|component| !component.is_empty())
            .unwrap_or("dependency.sp");
        let locator = crate::state::SourceLocator::try_new(
            dependency.logical_identity.clone(),
            display_name.to_owned(),
        )
        .map_err(|error| error.to_string())?;
        if locators
            .insert(dependency.logical_identity.clone(), locator)
            .is_some()
            || retained_sources
                .insert(dependency.logical_identity.clone(), retained.to_owned())
                .is_some()
        {
            return Err(format!(
                "Netlist bundle repeats dependency identity '{}'.",
                dependency.logical_identity
            ));
        }
        expected_entries.insert(dependency.bundle_entry.clone());
        expected_entries.insert(dependency.retained_entry.clone());
    }
    if entries.keys().any(|name| !expected_entries.contains(name)) {
        let extras = entries
            .keys()
            .filter(|name| !expected_entries.contains(*name))
            .take(4)
            .cloned()
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "Netlist bundle contains undeclared archive entries: {extras}."
        ));
    }

    let mut dependencies = Vec::with_capacity(manifest.dependencies.len());
    for dependency in &manifest.dependencies {
        let locator = locators
            .get(&dependency.logical_identity)
            .cloned()
            .ok_or_else(|| "Netlist bundle dependency map is inconsistent.".to_owned())?;
        let retained = retained_sources
            .get(&dependency.logical_identity)
            .ok_or_else(|| "Netlist bundle retained-source map is inconsistent.".to_owned())?;
        let record = if dependency.edge.owner == "generated" {
            crate::state::DependencyMetadata::unresolved_direct_to(
                dependency.edge.include_index,
                dependency.requested_locator.clone(),
                locator,
            )
        } else {
            let parent = locators
                .get(&dependency.edge.owner)
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "Dependency '{}' references undeclared parent '{}'.",
                        dependency.logical_identity, dependency.edge.owner
                    )
                })?;
            crate::state::DependencyMetadata::unresolved_transitive_to(
                parent,
                dependency.edge.include_index,
                dependency.requested_locator.clone(),
                locator,
            )
        }
        .map(|record| record.with_authority(dependency.authority))
        .and_then(|record| record.resolve_utf8(retained.as_bytes().to_vec()))
        .map_err(|error| error.to_string())?;
        dependencies.push(record);
    }

    let document_id = crate::state::NetlistDocumentId::new();
    let expanded = if dependencies.is_empty() {
        source.clone()
    } else {
        crate::state::expand_retained_netlist_dependencies(document_id, &source, &dependencies)?
            .source
    };
    rspice_core::Netlist::parse(&expanded)
        .map_err(|error| format!("Retained archive closure does not parse: {error}"))?;
    Ok(ImportedNetlistBundle {
        source,
        dependencies,
        expanded_source: expanded,
    })
}

pub(super) fn bundle_entry<'a>(
    entries: &'a std::collections::BTreeMap<String, Vec<u8>>,
    name: &str,
) -> Result<&'a [u8], String> {
    validate_bundle_entry_name(name)?;
    entries
        .get(name)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("Netlist bundle is missing declared entry '{name}'."))
}

pub(super) fn validate_digest_literal(value: &str, label: &str) -> Result<(), String> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(format!(
            "Netlist bundle {label} is not a canonical SHA-256 digest."
        ));
    }
    Ok(())
}

pub(super) fn verify_content_digest(
    bytes: &[u8],
    expected: &str,
    label: &str,
) -> Result<(), String> {
    validate_digest_literal(expected, label)?;
    let actual = crate::state::content_digest(
        std::str::from_utf8(bytes)
            .map_err(|error| format!("Netlist bundle {label} is not UTF-8: {error}"))?,
    )
    .to_string();
    if actual != expected {
        return Err(format!(
            "Netlist bundle {label} failed digest verification: expected {expected}, found {actual}."
        ));
    }
    Ok(())
}

pub(super) fn validate_bundle_entry_name(name: &str) -> Result<(), String> {
    if name.is_empty()
        || name.len() > 4_096
        || name.starts_with('/')
        || name.starts_with('\\')
        || name.contains('\\')
        || name.contains(':')
        || name.chars().any(char::is_control)
        || name
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(format!("Unsafe netlist bundle entry name '{name}'."));
    }
    Ok(())
}

pub(super) fn read_stored_zip_entries(
    bytes: &[u8],
    max_entries: usize,
) -> Result<std::collections::BTreeMap<String, Vec<u8>>, String> {
    const EOCD_BYTES: usize = 22;
    if bytes.len() < EOCD_BYTES {
        return Err("Netlist bundle is truncated before the ZIP end record.".to_owned());
    }
    let eocd = bytes.len() - EOCD_BYTES;
    if zip_u32(bytes, eocd)? != 0x0605_4b50 || zip_u16(bytes, eocd + 20)? != 0 {
        return Err(
            "Netlist bundle must be a single-disk ZIP with no trailing comment.".to_owned(),
        );
    }
    if zip_u16(bytes, eocd + 4)? != 0 || zip_u16(bytes, eocd + 6)? != 0 {
        return Err("Multi-disk ZIP archives are not supported.".to_owned());
    }
    let entry_count = usize::from(zip_u16(bytes, eocd + 10)?);
    if entry_count != usize::from(zip_u16(bytes, eocd + 8)?) || entry_count > max_entries {
        return Err(format!(
            "Netlist bundle entry count is inconsistent or exceeds {max_entries}."
        ));
    }
    let central_size = usize::try_from(zip_u32(bytes, eocd + 12)?)
        .map_err(|_| "ZIP central directory size is invalid.".to_owned())?;
    let central_offset = usize::try_from(zip_u32(bytes, eocd + 16)?)
        .map_err(|_| "ZIP central directory offset is invalid.".to_owned())?;
    if central_offset.checked_add(central_size) != Some(eocd) {
        return Err(
            "ZIP central directory does not end at the authenticated end record.".to_owned(),
        );
    }
    let mut cursor = central_offset;
    let mut entries = std::collections::BTreeMap::new();
    let mut retained_bytes = 0usize;
    for _ in 0..entry_count {
        if zip_u32(bytes, cursor)? != 0x0201_4b50 {
            return Err("ZIP central directory contains an invalid record signature.".to_owned());
        }
        let flags = zip_u16(bytes, cursor + 8)?;
        let method = zip_u16(bytes, cursor + 10)?;
        let crc = zip_u32(bytes, cursor + 16)?;
        let compressed = usize::try_from(zip_u32(bytes, cursor + 20)?)
            .map_err(|_| "ZIP entry size is invalid.".to_owned())?;
        let uncompressed = usize::try_from(zip_u32(bytes, cursor + 24)?)
            .map_err(|_| "ZIP entry size is invalid.".to_owned())?;
        let name_len = usize::from(zip_u16(bytes, cursor + 28)?);
        let extra_len = usize::from(zip_u16(bytes, cursor + 30)?);
        let comment_len = usize::from(zip_u16(bytes, cursor + 32)?);
        let local_offset = usize::try_from(zip_u32(bytes, cursor + 42)?)
            .map_err(|_| "ZIP local record offset is invalid.".to_owned())?;
        if flags != 0x0800 || method != 0 || compressed != uncompressed {
            return Err(
                "Netlist bundles require UTF-8 names and stored, unencrypted ZIP members."
                    .to_owned(),
            );
        }
        let name_start = cursor
            .checked_add(46)
            .ok_or_else(|| "ZIP central record overflowed.".to_owned())?;
        let name_end = name_start
            .checked_add(name_len)
            .ok_or_else(|| "ZIP entry name overflowed.".to_owned())?;
        let record_end = name_end
            .checked_add(extra_len)
            .and_then(|end| end.checked_add(comment_len))
            .ok_or_else(|| "ZIP central record overflowed.".to_owned())?;
        let name = std::str::from_utf8(
            bytes
                .get(name_start..name_end)
                .ok_or_else(|| "ZIP central entry name is truncated.".to_owned())?,
        )
        .map_err(|error| format!("ZIP entry name is not UTF-8: {error}"))?
        .to_owned();
        validate_bundle_entry_name(&name)?;

        if zip_u32(bytes, local_offset)? != 0x0403_4b50
            || zip_u16(bytes, local_offset + 6)? != flags
            || zip_u16(bytes, local_offset + 8)? != method
            || zip_u32(bytes, local_offset + 14)? != crc
            || usize::try_from(zip_u32(bytes, local_offset + 18)?).ok() != Some(compressed)
            || usize::try_from(zip_u32(bytes, local_offset + 22)?).ok() != Some(uncompressed)
        {
            return Err(format!(
                "ZIP local record for '{name}' disagrees with its directory."
            ));
        }
        let local_name_len = usize::from(zip_u16(bytes, local_offset + 26)?);
        let local_extra_len = usize::from(zip_u16(bytes, local_offset + 28)?);
        let local_name_start = local_offset
            .checked_add(30)
            .ok_or_else(|| "ZIP local record overflowed.".to_owned())?;
        let local_name_end = local_name_start
            .checked_add(local_name_len)
            .ok_or_else(|| "ZIP local entry name overflowed.".to_owned())?;
        if bytes.get(local_name_start..local_name_end) != Some(name.as_bytes()) {
            return Err(format!(
                "ZIP local record for '{name}' has a different name."
            ));
        }
        let content_start = local_name_end
            .checked_add(local_extra_len)
            .ok_or_else(|| "ZIP local entry overflowed.".to_owned())?;
        let content_end = content_start
            .checked_add(uncompressed)
            .ok_or_else(|| "ZIP entry content overflowed.".to_owned())?;
        let content = bytes
            .get(content_start..content_end)
            .ok_or_else(|| format!("ZIP entry '{name}' is truncated."))?;
        if bundle_crc32(content) != crc {
            return Err(format!("ZIP entry '{name}' failed CRC-32 verification."));
        }
        retained_bytes = retained_bytes
            .checked_add(content.len())
            .ok_or_else(|| "Netlist bundle expanded size overflowed.".to_owned())?;
        if retained_bytes as u64 > crate::io::project_io::MAX_PROJECT_FILE_BYTES {
            return Err(format!(
                "Netlist bundle expands beyond the supported {}-byte limit.",
                crate::io::project_io::MAX_PROJECT_FILE_BYTES
            ));
        }
        if entries.insert(name.clone(), content.to_vec()).is_some() {
            return Err(format!("Netlist bundle repeats ZIP entry '{name}'."));
        }
        cursor = record_end;
    }
    if cursor != eocd {
        return Err("ZIP central directory length does not match its records.".to_owned());
    }
    Ok(entries)
}

pub(super) fn zip_u16(bytes: &[u8], offset: usize) -> Result<u16, String> {
    let value = bytes
        .get(offset..offset.saturating_add(2))
        .ok_or_else(|| "ZIP structure is truncated.".to_owned())?;
    Ok(u16::from_le_bytes([value[0], value[1]]))
}

pub(super) fn zip_u32(bytes: &[u8], offset: usize) -> Result<u32, String> {
    let value = bytes
        .get(offset..offset.saturating_add(4))
        .ok_or_else(|| "ZIP structure is truncated.".to_owned())?;
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

pub(super) fn bundle_crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff_u32;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xedb8_8320 & mask);
        }
    }
    !crc
}

pub(super) fn decode_import_bytes(
    bytes: &[u8],
) -> Result<(String, crate::state::NetlistTextEncoding), String> {
    let encoding = if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        crate::state::NetlistTextEncoding::Utf8Bom
    } else if bytes.starts_with(&[0xff, 0xfe]) {
        crate::state::NetlistTextEncoding::Utf16LeBom
    } else if bytes.starts_with(&[0xfe, 0xff]) {
        crate::state::NetlistTextEncoding::Utf16BeBom
    } else if std::str::from_utf8(bytes).is_ok() {
        crate::state::NetlistTextEncoding::Utf8
    } else {
        crate::state::NetlistTextEncoding::Latin1
    };
    let source = rspice_core::netlist::decode_source_bytes(bytes)
        .map_err(|error| format!("source decoding failed: {error}"))?;
    Ok((source, encoding))
}

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    canonical::canonical_bytes,
    error::{PackError, invalid},
    path::{MANIFEST_ENTRY, SIGNATURE_ENTRY, is_reserved_path, validate_path},
};

/// The only manifest schema this crate reads or writes.
pub const MANIFEST_SCHEMA: u32 = 1;

const MAX_FILES: usize = 10_000;
const MAX_PARTS: usize = 100_000;
const MAX_CAPABILITIES: usize = 256;
const MAX_ALIASES: usize = 64;
const MAX_TERMINALS: usize = 64;
const MAX_IDENTIFIER_CHARS: usize = 128;
const MAX_NAME_CHARS: usize = 128;
const MAX_TERMINAL_CHARS: usize = 32;
const MAX_SOURCE_LINE: u32 = 100_000_000;
const SHA256_HEX_CHARS: usize = 64;

/// The signed document at the root of a `.rspicepack`.
///
/// No field may ever become a floating-point type: the canonical encoding is
/// defined only over integers, and a float would be rejected on read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    pub pack: PackIdentity,
    pub license: License,
    pub requires: Requires,
    pub files: Vec<FileEntry>,
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PackIdentity {
    pub id: String,
    pub version: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct License {
    pub spdx: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Requires {
    pub capabilities: Vec<String>,
}

/// One archive member, bound to the signature by its exact expanded length and
/// digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileEntry {
    pub path: String,
    pub length: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartKind {
    Model,
    Subckt,
}

/// One addressable model or subcircuit the pack publishes.
///
/// Optional content is absent rather than empty or null, so one part has
/// exactly one canonical encoding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Part {
    pub id: String,
    pub kind: PartKind,
    pub device: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<String>,
    pub source: SourceRef,
    pub terminals: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol: Option<Symbol>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceRef {
    pub path: String,
    pub line: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Symbol {
    #[serde(rename = "ref")]
    pub reference: String,
    pub pins: BTreeMap<String, String>,
}

/// A manifest minus the `files[]` the packer derives from the file set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManifestTemplate {
    pub pack: PackIdentity,
    pub license: License,
    pub requires: Requires,
    pub parts: Vec<Part>,
}

/// Encodes a manifest as the exact bytes that carry its signature.
///
/// Validation runs first, so an invalid manifest can never be signed.
pub fn canonical_manifest_bytes(manifest: &Manifest) -> Result<Vec<u8>, PackError> {
    validate_manifest(manifest)?;
    let value =
        serde_json::to_value(manifest).map_err(|error| PackError::Encoding(error.to_string()))?;
    canonical_bytes(&value)
}

/// Parses and fully validates untrusted `manifest.json` bytes.
///
/// Canonicality is proved twice against different failure modes: once
/// structurally, so encoding-level noise (whitespace, key order, duplicate
/// keys, non-integer numbers) is caught with a precise error, and once through
/// the typed projection, so an encoding that carries content the schema does
/// not model — an empty array where absence is canonical, a null where the key
/// belongs omitted — cannot ride along under a valid signature.
pub fn parse_manifest(bytes: &[u8]) -> Result<Manifest, PackError> {
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|error| PackError::MalformedManifest(error.to_string()))?;
    if canonical_bytes(&value)? != bytes {
        return Err(PackError::NonCanonicalManifest);
    }
    let manifest: Manifest = serde_json::from_value(value)
        .map_err(|error| PackError::MalformedManifest(error.to_string()))?;
    if canonical_manifest_bytes(&manifest)? != bytes {
        return Err(PackError::NonCanonicalManifest);
    }
    Ok(manifest)
}

/// Proves a manifest is internally consistent, independent of any archive.
pub fn validate_manifest(manifest: &Manifest) -> Result<(), PackError> {
    if manifest.schema != MANIFEST_SCHEMA {
        return Err(invalid(
            "schema",
            format!("only schema {MANIFEST_SCHEMA} is defined"),
        ));
    }
    validate_pack_id(&manifest.pack.id)?;
    validate_semver(&manifest.pack.version, "pack.version")?;
    validate_display_name(&manifest.pack.name, "pack.name")?;
    validate_kebab(&manifest.pack.category, "pack.category")?;
    validate_spdx(&manifest.license.spdx)?;
    validate_capabilities(&manifest.requires.capabilities)?;
    let paths = validate_files(&manifest.files)?;
    validate_parts(&manifest.parts, &paths)?;
    Ok(())
}

fn validate_files(files: &[FileEntry]) -> Result<BTreeSet<&str>, PackError> {
    if files.is_empty() || files.len() > MAX_FILES {
        return Err(invalid(
            "files",
            format!("a pack lists between 1 and {MAX_FILES} files"),
        ));
    }
    let mut paths = BTreeSet::new();
    let mut previous: Option<&str> = None;
    for entry in files {
        validate_path(&entry.path)?;
        if is_reserved_path(&entry.path) {
            return Err(PackError::ReservedFileName(entry.path.clone()));
        }
        // Ascending order is what makes one file set have one encoding; the
        // packer derives it, so a manifest out of order was hand-edited.
        if let Some(previous) = previous
            && previous.as_bytes() >= entry.path.as_bytes()
        {
            return Err(invalid(
                "files",
                "listed paths must be unique and ascending by path bytes",
            ));
        }
        previous = Some(&entry.path);
        if !is_sha256_hex(&entry.sha256) {
            return Err(invalid(
                "files[].sha256",
                "a digest is 64 lowercase hexadecimal characters",
            ));
        }
        paths.insert(entry.path.as_str());
    }
    Ok(paths)
}

fn validate_parts(parts: &[Part], paths: &BTreeSet<&str>) -> Result<(), PackError> {
    if parts.len() > MAX_PARTS {
        return Err(invalid(
            "parts",
            format!("a pack publishes at most {MAX_PARTS} parts"),
        ));
    }
    // SPICE resolves model names without regard to case, so a pack whose ids
    // and aliases collide case-insensitively would be ambiguous to index.
    let mut lookup = BTreeSet::new();
    for part in parts {
        validate_identifier(&part.id, "parts[].id")?;
        validate_kebab(&part.device, "parts[].device")?;
        if part.aliases.len() > MAX_ALIASES {
            return Err(invalid(
                "parts[].aliases",
                format!("a part carries at most {MAX_ALIASES} aliases"),
            ));
        }
        for alias in &part.aliases {
            validate_identifier(alias, "parts[].aliases")?;
        }
        for name in std::iter::once(&part.id).chain(&part.aliases) {
            if !lookup.insert(name.to_ascii_lowercase()) {
                return Err(invalid(
                    "parts",
                    format!("{name:?} is claimed by more than one part name or alias"),
                ));
            }
        }
        if !paths.contains(part.source.path.as_str()) {
            return Err(invalid(
                "parts[].source.path",
                format!("{:?} is not one of the listed files", part.source.path),
            ));
        }
        if part.source.line == 0 || part.source.line > MAX_SOURCE_LINE {
            return Err(invalid(
                "parts[].source.line",
                "a source line is a positive, bounded line number",
            ));
        }
        validate_terminals(&part.terminals)?;
        if let Some(symbol) = &part.symbol {
            validate_symbol(symbol, &part.terminals)?;
        }
    }
    Ok(())
}

pub(crate) fn validate_terminals(terminals: &[String]) -> Result<(), PackError> {
    if terminals.is_empty() || terminals.len() > MAX_TERMINALS {
        return Err(invalid(
            "parts[].terminals",
            format!("a part has between 1 and {MAX_TERMINALS} terminals"),
        ));
    }
    let mut seen = BTreeSet::new();
    for terminal in terminals {
        if terminal.is_empty()
            || terminal.chars().count() > MAX_TERMINAL_CHARS
            || !terminal
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'+' | b'-'))
        {
            return Err(invalid(
                "parts[].terminals",
                format!("{terminal:?} is not a bounded alphanumeric terminal name"),
            ));
        }
        if !seen.insert(terminal.to_ascii_lowercase()) {
            return Err(invalid(
                "parts[].terminals",
                format!("{terminal:?} appears more than once"),
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_symbol(symbol: &Symbol, terminals: &[String]) -> Result<(), PackError> {
    validate_identifier(&symbol.reference, "parts[].symbol.ref")?;
    let declared: BTreeSet<&str> = terminals.iter().map(String::as_str).collect();
    let mapped: BTreeSet<&str> = symbol.pins.keys().map(String::as_str).collect();
    if declared != mapped {
        return Err(invalid(
            "parts[].symbol.pins",
            "a symbol maps exactly the part's terminals, one pin each",
        ));
    }
    let mut pins = BTreeSet::new();
    for pin in symbol.pins.values() {
        validate_identifier(pin, "parts[].symbol.pins")?;
        if !pins.insert(pin.as_str()) {
            return Err(invalid(
                "parts[].symbol.pins",
                format!("{pin:?} is mapped by more than one terminal"),
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_pack_id(id: &str) -> Result<(), PackError> {
    let family = id.strip_prefix("rspice-").unwrap_or_default();
    if family.is_empty() || !is_kebab(id) {
        return Err(invalid(
            "pack.id",
            "a pack identifier is rspice-<family> in lowercase kebab case",
        ));
    }
    Ok(())
}

pub(crate) fn validate_kebab(value: &str, field: &str) -> Result<(), PackError> {
    if is_kebab(value) {
        Ok(())
    } else {
        Err(invalid(field, "expected bounded lowercase kebab case"))
    }
}

fn is_kebab(value: &str) -> bool {
    let bounded = !value.is_empty() && value.len() <= 64;
    let charset = value
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-');
    let shaped = !value.starts_with('-') && !value.ends_with('-') && !value.contains("--");
    bounded && charset && shaped
}

pub(crate) fn validate_capabilities(capabilities: &[String]) -> Result<(), PackError> {
    if capabilities.len() > MAX_CAPABILITIES {
        return Err(invalid(
            "requires.capabilities",
            format!("a pack requires at most {MAX_CAPABILITIES} capabilities"),
        ));
    }
    let mut seen = BTreeSet::new();
    for capability in capabilities {
        validate_kebab(capability, "requires.capabilities")?;
        if !seen.insert(capability.as_str()) {
            return Err(invalid(
                "requires.capabilities",
                format!("{capability:?} is required more than once"),
            ));
        }
    }
    Ok(())
}

/// Part, alias, symbol, and pin names: bounded printable ASCII without
/// whitespace, quoting, or path separators, so a name can be printed, matched,
/// and embedded in a netlist without escaping.
pub(crate) fn validate_identifier(value: &str, field: &str) -> Result<(), PackError> {
    let bounded = !value.is_empty() && value.len() <= MAX_IDENTIFIER_CHARS;
    let charset = value
        .bytes()
        .all(|byte| byte.is_ascii_graphic() && !matches!(byte, b'"' | b'\'' | b'\\' | b'`' | b'/'));
    if bounded && charset {
        Ok(())
    } else {
        Err(invalid(
            field,
            format!("{value:?} is not a bounded printable identifier"),
        ))
    }
}

pub(crate) fn validate_display_name(value: &str, field: &str) -> Result<(), PackError> {
    let bounded = !value.trim().is_empty() && value.chars().count() <= MAX_NAME_CHARS;
    let printable = !value.chars().any(char::is_control) && value.trim() == value;
    if bounded && printable {
        Ok(())
    } else {
        Err(invalid(field, "expected a bounded, trimmed display name"))
    }
}

pub(crate) fn validate_spdx(value: &str) -> Result<(), PackError> {
    let bounded = !value.is_empty() && value.len() <= MAX_NAME_CHARS;
    let charset = value
        .bytes()
        .all(|byte| byte.is_ascii_graphic() || byte == b' ');
    if bounded && charset && value.trim() == value {
        Ok(())
    } else {
        Err(invalid(
            "license.spdx",
            "expected a bounded SPDX identifier or expression",
        ))
    }
}

/// Semantic Versioning 2.0.0, validated without a dependency.
pub(crate) fn validate_semver(value: &str, field: &str) -> Result<(), PackError> {
    let refuse = || invalid(field, format!("{value:?} is not a semantic version"));
    if value.is_empty() || value.len() > 64 || !value.is_ascii() {
        return Err(refuse());
    }
    let boundary = value
        .bytes()
        .position(|byte| matches!(byte, b'-' | b'+'))
        .unwrap_or(value.len());
    let (core, remainder) = value.split_at(boundary);
    let mut numbers = core.split('.');
    for _ in 0..3 {
        let Some(number) = numbers.next() else {
            return Err(refuse());
        };
        if !is_numeric_identifier(number) {
            return Err(refuse());
        }
    }
    if numbers.next().is_some() {
        return Err(refuse());
    }
    let (prerelease, build) = match remainder.strip_prefix('-') {
        Some(rest) => match rest.split_once('+') {
            Some((prerelease, build)) => (Some(prerelease), Some(build)),
            None => (Some(rest), None),
        },
        None => (None, remainder.strip_prefix('+')),
    };
    if let Some(prerelease) = prerelease {
        for identifier in prerelease.split('.') {
            let alphanumeric = !identifier.is_empty()
                && identifier
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-');
            let numeric_ok = !identifier.bytes().all(|byte| byte.is_ascii_digit())
                || is_numeric_identifier(identifier);
            if !alphanumeric || !numeric_ok {
                return Err(refuse());
            }
        }
    }
    if let Some(build) = build {
        for identifier in build.split('.') {
            if identifier.is_empty()
                || !identifier
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            {
                return Err(refuse());
            }
        }
    }
    Ok(())
}

fn is_numeric_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
}

pub(crate) fn is_sha256_hex(value: &str) -> bool {
    value.len() == SHA256_HEX_CHARS
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// The reserved archive entries, which never appear in `files[]`.
pub(crate) const RESERVED_ENTRIES: [&str; 2] = [MANIFEST_ENTRY, SIGNATURE_ENTRY];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing;

    #[test]
    fn a_reference_manifest_validates_and_round_trips() {
        let manifest = testing::manifest();
        let bytes = canonical_manifest_bytes(&manifest).expect("canonicalizes");
        let parsed = parse_manifest(&bytes).expect("parses");
        assert_eq!(parsed, manifest);
    }

    #[test]
    fn perturbed_whitespace_is_not_canonical() {
        let bytes = canonical_manifest_bytes(&testing::manifest()).expect("canonicalizes");
        let mut perturbed = Vec::from(b"{ ".as_slice());
        perturbed.extend_from_slice(&bytes[1..]);
        assert!(matches!(
            parse_manifest(&perturbed),
            Err(PackError::NonCanonicalManifest)
        ));
    }

    #[test]
    fn an_empty_optional_array_is_not_canonical() {
        let mut manifest = testing::manifest();
        for part in &mut manifest.parts {
            part.aliases.clear();
        }
        let bytes = canonical_manifest_bytes(&manifest).expect("canonicalizes");
        let text = String::from_utf8(bytes).expect("UTF-8");
        assert!(!text.contains("aliases"));
        let smuggled = text.replace("\"device\":", "\"aliases\":[],\"device\":");
        assert!(matches!(
            parse_manifest(smuggled.as_bytes()),
            Err(PackError::NonCanonicalManifest)
        ));
    }

    #[test]
    fn a_float_anywhere_is_refused_before_typing() {
        let bytes = canonical_manifest_bytes(&testing::manifest()).expect("canonicalizes");
        let text = String::from_utf8(bytes).expect("UTF-8");
        let smuggled = text.replace("\"line\":214", "\"line\":214.0");
        assert!(matches!(
            parse_manifest(smuggled.as_bytes()),
            Err(PackError::FloatInManifest(_))
        ));
    }

    #[test]
    fn reserved_names_cannot_be_listed_as_files() {
        for reserved in RESERVED_ENTRIES {
            let mut manifest = testing::manifest();
            reserved.clone_into(&mut manifest.files[0].path);
            for part in &mut manifest.parts {
                reserved.clone_into(&mut part.source.path);
            }
            assert!(matches!(
                validate_manifest(&manifest),
                Err(PackError::ReservedFileName(name)) if name == reserved
            ));
        }
    }

    #[test]
    fn listed_files_must_be_unique_and_ascending() {
        let mut manifest = testing::manifest();
        manifest.files.reverse();
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PackError::InvalidField { .. })
        ));
    }

    #[test]
    fn part_names_may_not_collide_case_insensitively() {
        let mut manifest = testing::manifest();
        manifest.parts[0].aliases.push("tl072".to_owned());
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PackError::InvalidField { .. })
        ));
    }

    #[test]
    fn a_symbol_maps_exactly_its_terminals() {
        let mut manifest = testing::manifest();
        if let Some(symbol) = manifest.parts[0].symbol.as_mut() {
            symbol.pins.remove("OUT");
        }
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PackError::InvalidField { field, .. }) if field == "parts[].symbol.pins"
        ));
    }

    #[test]
    fn a_part_source_must_name_a_listed_file() {
        let mut manifest = testing::manifest();
        manifest.parts[0].source.path = "models/absent.lib".to_owned();
        assert!(matches!(
            validate_manifest(&manifest),
            Err(PackError::InvalidField { field, .. }) if field == "parts[].source.path"
        ));
    }

    #[test]
    fn pack_identifiers_carry_the_product_prefix() {
        for id in ["opamps", "rspice-", "rspice-Opamps", "rspice--opamps", "-x"] {
            let mut manifest = testing::manifest();
            manifest.pack.id = id.to_owned();
            assert!(
                matches!(
                    validate_manifest(&manifest),
                    Err(PackError::InvalidField { .. })
                ),
                "{id:?} must be refused"
            );
        }
    }

    #[test]
    fn semantic_versions_are_enforced() {
        for accepted in ["0.0.0", "1.0.0", "1.2.3-rc.1", "1.2.3+build.5", "1.2.3-a+b"] {
            validate_semver(accepted, "pack.version").expect("accepted version");
        }
        for refused in [
            "1", "1.2", "1.2.3.4", "01.2.3", "1.2.3-", "1.2.3-01", "v1.2.3", "",
        ] {
            assert!(
                validate_semver(refused, "pack.version").is_err(),
                "{refused:?} must be refused"
            );
        }
    }
}

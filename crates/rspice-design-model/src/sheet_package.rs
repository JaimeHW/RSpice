//! The signed drawing-sheet package contract.
//!
//! One implementation serves both ends of the ceremony: `rspice-ui` imports a
//! package, and the offline `rspice-sheet-publisher` signs one. Because they
//! share this code rather than agreeing by convention, "the publisher signed
//! it" and "the importer will accept it" are the same statement.
//!
//! A signature covers exactly the projection built by
//! [`canonical_package_contract`], prefixed with a domain separator. The
//! stored digest and the signature bytes themselves sit outside that
//! projection and so cannot influence what was signed.

use std::collections::BTreeSet;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use ed25519_dalek::{Signature, Signer as _, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::design_management::{
    AuthoredDrawingSheetSize, DrawingSheetBorderTemplate, DrawingSheetPreset,
    DrawingSheetPresetScope, DrawingSheetTitleBlockTemplate, MAX_DRAWING_SHEET_PROJECT_PRESETS,
    SchematicSheetFormat,
};

pub const PACKAGE_SCHEMA: &str = "rspice-sheet-formats";
pub const PACKAGE_VERSION: u16 = 1;
pub const DRAWING_SHEET_PACKAGE_MAX_BYTES: usize = 4 * 1024 * 1024;
const PACKAGE_SIGNATURE_DOMAIN: &[u8] = b"RSPICE\0DRAWING-SHEET-PRESET-PACKAGE\0SCHEMA-1\0";

/// Publisher-trust authority consulted when an imported package claims a
/// signature.
///
/// The application supplies a revocation-aware trust store; the offline
/// publisher verifies against the single explicit key it was handed. Putting
/// the authority behind a trait lets this crate own the whole authenticity
/// rule without owning either trust model.
pub trait PublisherTrust {
    /// Verify `signature` over `message` for the named publisher key.
    ///
    /// The error is shown to an operator, so it must say why the key was
    /// rejected: unknown, revoked, malformed, or simply not a match.
    fn verify_publisher_signature(
        &self,
        publisher_id: &str,
        key_id: &str,
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), String>;
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DrawingSheetPackageEncoding {
    #[default]
    CanonicalSchema1,
    HumanReviewJson,
}

impl DrawingSheetPackageEncoding {
    pub const fn label(self) -> &'static str {
        match self {
            Self::CanonicalSchema1 => "RSpice sheet formats \u{00b7} schema 1",
            Self::HumanReviewJson => "JSON + schema \u{00b7} human review",
        }
    }
}

/// Identity and integrity facts returned after an offline publisher signs a
/// drawing-sheet package. Private signing material is deliberately absent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishedDrawingSheetPackage {
    pub encoded: String,
    pub publisher_id: String,
    pub signing_key_id: String,
    pub source_digest_sha256: String,
    pub verifying_key: [u8; 32],
    pub preset_count: usize,
}

/// Verified publisher identity and content facts for a signed package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetPackageVerification {
    pub publisher_id: String,
    pub signing_key_id: String,
    pub source_digest_sha256: String,
    pub preset_count: usize,
    pub organization_preset_count: usize,
}

/// Structurally validated facts used to bind an offline signing ceremony to
/// the exact unsigned package reviewed by an operator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawingSheetPackageInspection {
    pub source_digest_sha256: String,
    pub preset_count: usize,
    pub organization_preset_count: usize,
    pub is_signed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetPackage {
    pub schema: String,
    pub version: u16,
    pub source_digest_sha256: String,
    pub includes_builtin_frame_references: bool,
    pub includes_source_metadata: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub builtin_frame_references: Vec<BuiltinFrameReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_metadata: Option<PresetPackageSourceMetadata>,
    /// Present only when an external organization publisher has signed the
    /// canonical package contract. RSpice persists public verification keys,
    /// never private signing material.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_signature: Option<DrawingSheetPresetPublisherSignature>,
    pub presets: Vec<PortableDrawingSheetPreset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DrawingSheetPresetPublisherSignature {
    pub publisher_id: String,
    pub signing_key_id: String,
    pub signature_base64: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinFrameReference {
    pub kind: String,
    pub identity: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresetPackageSourceMetadata {
    pub producer: String,
    pub content: String,
    pub exact_dimension_unit: String,
    pub source_scopes: Vec<DrawingSheetPresetScope>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PortableDrawingSheetPreset {
    pub stable_id: String,
    pub name: String,
    pub source_scope: DrawingSheetPresetScope,
    pub portrait_width_um: u64,
    pub portrait_height_um: u64,
    pub format: SchematicSheetFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HumanReviewPackageDocument {
    document: String,
    schema: HumanReviewSchemaDescriptor,
    package: DrawingSheetPresetPackage,
    review: Vec<HumanReviewPreset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HumanReviewSchemaDescriptor {
    identity: String,
    version: u16,
    representation: String,
    exact_dimension_unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct HumanReviewPreset {
    stable_id: String,
    name: String,
    source_scope: DrawingSheetPresetScope,
    portrait_dimensions: String,
    portrait_width_um: u64,
    portrait_height_um: u64,
    dependencies: Vec<String>,
}

pub fn portable_from_preset(
    preset: &DrawingSheetPreset,
) -> Result<PortableDrawingSheetPreset, String> {
    let preset = preset
        .clone()
        .normalized_for_storage()
        .map_err(|error| error.to_string())?;
    let AuthoredDrawingSheetSize::Custom { snapshot } = &preset.format.authored_size else {
        return Err(format!(
            "'{}' is not a custom physical sheet size.",
            preset.name
        ));
    };
    Ok(PortableDrawingSheetPreset {
        stable_id: preset.id.clone(),
        name: preset.name.clone(),
        source_scope: preset.scope,
        portrait_width_um: snapshot.portrait_width_um,
        portrait_height_um: snapshot.portrait_height_um,
        format: preset.format.as_reusable_drawing_sheet_preset(),
    })
}

pub fn build_package_with_options(
    presets: impl IntoIterator<Item = DrawingSheetPreset>,
    include_builtin_frame_references: bool,
    include_source_metadata: bool,
) -> Result<DrawingSheetPresetPackage, String> {
    let mut presets = presets
        .into_iter()
        .map(|preset| portable_from_preset(&preset))
        .collect::<Result<Vec<_>, _>>()?;
    presets.sort_by(|left, right| {
        left.stable_id
            .cmp(&right.stable_id)
            .then_with(|| left.name.cmp(&right.name))
            .then_with(|| scope_rank(left.source_scope).cmp(&scope_rank(right.source_scope)))
    });
    let builtin_frame_references = if include_builtin_frame_references {
        builtin_frame_references(&presets)
    } else {
        Vec::new()
    };
    let source_metadata = include_source_metadata.then(|| source_metadata(&presets));
    let mut package = DrawingSheetPresetPackage {
        schema: PACKAGE_SCHEMA.to_owned(),
        version: PACKAGE_VERSION,
        source_digest_sha256: String::new(),
        includes_builtin_frame_references: include_builtin_frame_references,
        includes_source_metadata: include_source_metadata,
        builtin_frame_references,
        source_metadata,
        publisher_signature: None,
        presets,
    };
    package.source_digest_sha256 = package_digest(&package)?;
    validate_package(&package)?;
    Ok(package)
}

pub fn encode_package_with_format(
    package: &DrawingSheetPresetPackage,
    format: DrawingSheetPackageEncoding,
) -> Result<String, String> {
    validate_package(package)?;
    if package
        .presets
        .iter()
        .any(|preset| preset.source_scope == DrawingSheetPresetScope::Organization)
        && package.publisher_signature.is_none()
    {
        return Err(
            "Organization sheet-format definitions must be signed by external publisher tooling before distribution."
                .to_owned(),
        );
    }
    match format {
        DrawingSheetPackageEncoding::CanonicalSchema1 => serde_json::to_string_pretty(package),
        DrawingSheetPackageEncoding::HumanReviewJson => {
            let document = HumanReviewPackageDocument {
                document: "rspice-sheet-formats-human-review".to_owned(),
                schema: HumanReviewSchemaDescriptor {
                    identity: PACKAGE_SCHEMA.to_owned(),
                    version: PACKAGE_VERSION,
                    representation: "human-review".to_owned(),
                    exact_dimension_unit: "micrometre".to_owned(),
                },
                package: package.clone(),
                review: human_review_rows(&package.presets),
            };
            serde_json::to_string_pretty(&document)
        }
    }
    .map_err(|error| format!("Could not encode the sheet-format package: {error}"))
}

pub fn parse_package(
    source: &str,
    trust: &dyn PublisherTrust,
) -> Result<DrawingSheetPresetPackage, String> {
    let package = decode_package_without_authenticity(source)?;
    validate_package_authenticity(&package, trust)?;
    Ok(package)
}

fn decode_package_without_authenticity(source: &str) -> Result<DrawingSheetPresetPackage, String> {
    if source.len() > DRAWING_SHEET_PACKAGE_MAX_BYTES {
        return Err(format!(
            "The preset package exceeds the {} byte limit.",
            DRAWING_SHEET_PACKAGE_MAX_BYTES
        ));
    }
    let value = serde_json::from_str::<serde_json::Value>(source)
        .map_err(|error| format!("The preset package is not valid JSON: {error}"))?;
    let package = if value
        .get("document")
        .and_then(serde_json::Value::as_str)
        .is_some()
    {
        let document =
            serde_json::from_value::<HumanReviewPackageDocument>(value).map_err(|error| {
                format!("The human-review package is not valid schema-1 JSON: {error}")
            })?;
        validate_human_review_document(&document)?;
        document.package
    } else {
        serde_json::from_value::<DrawingSheetPresetPackage>(value)
            .map_err(|error| format!("The preset package is not valid schema-1 JSON: {error}"))?
    };
    validate_package(&package)?;
    Ok(package)
}

/// Inspect a package only after its schema, redundant manifests, embedded
/// identities, and canonical SHA-256 digest have all validated. This does not
/// grant publisher trust and deliberately does not verify a claimed signature.
pub fn inspect_drawing_sheet_package(
    source: &str,
) -> Result<DrawingSheetPackageInspection, String> {
    let package = decode_package_without_authenticity(source)?;
    Ok(DrawingSheetPackageInspection {
        source_digest_sha256: package.source_digest_sha256,
        preset_count: package.presets.len(),
        organization_preset_count: package
            .presets
            .iter()
            .filter(|preset| preset.source_scope == DrawingSheetPresetScope::Organization)
            .count(),
        is_signed: package.publisher_signature.is_some(),
    })
}

/// Sign a structurally valid package with the same domain-separated Ed25519
/// contract used by RSpice's importer.
///
/// This is the intentionally small boundary used by the offline publisher
/// executable. It never persists or logs the supplied private seed. Existing
/// signatures are rejected so changing publisher identity is always an
/// explicit, reviewable operation starting from an unsigned source package.
pub fn publish_organization_drawing_sheet_package(
    source: &str,
    private_key_seed: &[u8; 32],
    publisher_id: &str,
    signing_key_id: &str,
    promote_all_presets: bool,
    output_encoding: DrawingSheetPackageEncoding,
) -> Result<PublishedDrawingSheetPackage, String> {
    let mut package = decode_package_without_authenticity(source)?;
    if package.publisher_signature.is_some() {
        return Err(
            "The source package is already signed. Export a new unsigned package before publishing it."
                .to_owned(),
        );
    }
    if package.presets.is_empty() {
        return Err("An empty drawing-sheet package cannot be published.".to_owned());
    }
    if package.presets.iter().any(|preset| {
        matches!(
            &preset.format.authored_size,
            AuthoredDrawingSheetSize::Custom { snapshot }
                if snapshot.source_preset_unavailable
        )
    }) {
        return Err(
            "A package containing unavailable custom-size definitions cannot be published."
                .to_owned(),
        );
    }
    if promote_all_presets {
        for preset in &mut package.presets {
            preset.source_scope = DrawingSheetPresetScope::Organization;
        }
        if package.includes_source_metadata {
            package.source_metadata = Some(source_metadata(&package.presets));
        }
    } else if package
        .presets
        .iter()
        .any(|preset| preset.source_scope != DrawingSheetPresetScope::Organization)
    {
        return Err(
            "Every preset must already have organization scope, or publishing must explicitly promote all presets."
                .to_owned(),
        );
    }

    validate_publisher_identifier("publisher_id", publisher_id)?;
    validate_publisher_identifier("signing_key_id", signing_key_id)?;
    package.publisher_signature = Some(DrawingSheetPresetPublisherSignature {
        publisher_id: publisher_id.to_owned(),
        signing_key_id: signing_key_id.to_owned(),
        signature_base64: String::new(),
    });
    package.source_digest_sha256 = package_digest(&package)?;

    let signing_key = SigningKey::from_bytes(private_key_seed);
    let signature = signing_key.sign(&package_signature_message(&package)?);
    package
        .publisher_signature
        .as_mut()
        .expect("publisher identity was assigned above")
        .signature_base64 = STANDARD.encode(signature.to_bytes());
    validate_package(&package)?;

    let encoded = encode_package_with_format(&package, output_encoding)?;
    let verification =
        verify_published_drawing_sheet_package(&encoded, &signing_key.verifying_key().to_bytes())?;
    Ok(PublishedDrawingSheetPackage {
        encoded,
        publisher_id: verification.publisher_id,
        signing_key_id: verification.signing_key_id,
        source_digest_sha256: verification.source_digest_sha256,
        verifying_key: signing_key.verifying_key().to_bytes(),
        preset_count: verification.preset_count,
    })
}

/// Verify a signed package against one explicit Ed25519 public key without
/// consulting mutable application trust state. The publisher CLI uses this
/// both before reporting success and when independently auditing an artifact.
pub fn verify_published_drawing_sheet_package(
    source: &str,
    verifying_key_bytes: &[u8; 32],
) -> Result<DrawingSheetPackageVerification, String> {
    let package = decode_package_without_authenticity(source)?;
    let publisher = package.publisher_signature.as_ref().ok_or_else(|| {
        "The drawing-sheet package does not contain a publisher signature.".to_owned()
    })?;
    let signature_bytes = STANDARD
        .decode(&publisher.signature_base64)
        .map_err(|error| format!("The publisher signature is not valid base64: {error}"))?;
    let signature_bytes: [u8; 64] = signature_bytes.try_into().map_err(|bytes: Vec<u8>| {
        format!(
            "The publisher signature contains {} bytes; exactly 64 are required.",
            bytes.len()
        )
    })?;
    let verifying_key = VerifyingKey::from_bytes(verifying_key_bytes)
        .map_err(|error| format!("The publisher public key is invalid: {error}"))?;
    verifying_key
        .verify_strict(
            &package_signature_message(&package)?,
            &Signature::from_bytes(&signature_bytes),
        )
        .map_err(|_| {
            format!(
                "The publisher signature is invalid for {}/{}.",
                publisher.publisher_id, publisher.signing_key_id
            )
        })?;

    Ok(DrawingSheetPackageVerification {
        publisher_id: publisher.publisher_id.clone(),
        signing_key_id: publisher.signing_key_id.clone(),
        source_digest_sha256: package.source_digest_sha256.clone(),
        preset_count: package.presets.len(),
        organization_preset_count: package
            .presets
            .iter()
            .filter(|preset| preset.source_scope == DrawingSheetPresetScope::Organization)
            .count(),
    })
}

/// Derive the public key that must be provisioned into RSpice's publisher
/// trust store. The signing key itself is zeroized on drop by ed25519-dalek.
pub fn drawing_sheet_publisher_public_key(private_key_seed: &[u8; 32]) -> [u8; 32] {
    SigningKey::from_bytes(private_key_seed)
        .verifying_key()
        .to_bytes()
}

fn validate_package(package: &DrawingSheetPresetPackage) -> Result<(), String> {
    if package.schema != PACKAGE_SCHEMA || package.version != PACKAGE_VERSION {
        return Err(format!(
            "Unsupported sheet-format package '{}', version {}.",
            package.schema, package.version
        ));
    }
    if package.presets.len() > MAX_DRAWING_SHEET_PROJECT_PRESETS {
        return Err("The preset package contains too many definitions.".to_owned());
    }
    if !package.includes_builtin_frame_references && !package.builtin_frame_references.is_empty() {
        return Err(
            "The built-in frame-reference declaration does not match its manifest.".to_owned(),
        );
    }
    let expected_references = builtin_frame_references(&package.presets);
    if package.includes_builtin_frame_references
        && package.builtin_frame_references != expected_references
    {
        return Err(
            "The built-in frame-reference manifest does not match the preset definitions."
                .to_owned(),
        );
    }
    if !package.includes_source_metadata && package.source_metadata.is_some() {
        return Err("The source-metadata declaration does not match its payload.".to_owned());
    }
    if package.includes_source_metadata
        && package.source_metadata.as_ref() != Some(&source_metadata(&package.presets))
    {
        return Err("The source metadata does not match the preset definitions.".to_owned());
    }
    if let Some(publisher) = &package.publisher_signature {
        validate_publisher_identifier("publisher_id", &publisher.publisher_id)?;
        validate_publisher_identifier("signing_key_id", &publisher.signing_key_id)?;
        if publisher.signature_base64.len() > 128 {
            return Err("The publisher signature exceeds its encoded size limit.".to_owned());
        }
    }
    let digest = package_digest(package)?;
    if digest != package.source_digest_sha256 {
        return Err("The preset package digest does not match its contract.".to_owned());
    }
    let mut ids = BTreeSet::new();
    let mut names = BTreeSet::new();
    for preset in &package.presets {
        if !ids.insert(preset.stable_id.to_lowercase()) {
            return Err(format!(
                "Preset identity '{}' is duplicated in the package.",
                preset.stable_id
            ));
        }
        if !names.insert(preset.name.to_lowercase()) {
            return Err(format!(
                "Preset name '{}' is duplicated in the package.",
                preset.name
            ));
        }
        preset
            .format
            .validate()
            .map_err(|error| error.to_string())?;
        let AuthoredDrawingSheetSize::Custom { snapshot } = &preset.format.authored_size else {
            return Err(format!(
                "Preset '{}' is not a custom physical size.",
                preset.name
            ));
        };
        if snapshot.preset_id.as_deref() != Some(preset.stable_id.as_str())
            || snapshot.name != preset.name
        {
            return Err(format!(
                "Preset '{}' does not match its embedded custom-size identity.",
                preset.name
            ));
        }
        if snapshot.portrait_width_um != preset.portrait_width_um
            || snapshot.portrait_height_um != preset.portrait_height_um
        {
            return Err(format!(
                "Preset '{}' dimensions do not match its exact sheet snapshot.",
                preset.name
            ));
        }
        if preset
            .format
            .title_block
            .fields
            .values()
            .any(|field| !field.value.is_empty())
        {
            return Err(format!(
                "Preset '{}' contains authored title-block values; preset packages may retain visibility only.",
                preset.name
            ));
        }
    }
    Ok(())
}

fn validate_package_authenticity(
    package: &DrawingSheetPresetPackage,
    trust: &dyn PublisherTrust,
) -> Result<(), String> {
    let requires_signature = package
        .presets
        .iter()
        .any(|preset| preset.source_scope == DrawingSheetPresetScope::Organization);
    let Some(publisher) = &package.publisher_signature else {
        return if requires_signature {
            Err(
                "Organization sheet-format definitions require a publisher signature from a trusted, non-revoked key."
                    .to_owned(),
            )
        } else {
            Ok(())
        };
    };
    let signature = STANDARD
        .decode(&publisher.signature_base64)
        .map_err(|error| format!("The publisher signature is not valid base64: {error}"))?;
    if signature.len() != 64 {
        return Err(format!(
            "The publisher signature contains {} bytes; exactly 64 are required.",
            signature.len()
        ));
    }
    let message = package_signature_message(package)?;
    trust
        .verify_publisher_signature(
            &publisher.publisher_id,
            &publisher.signing_key_id,
            &message,
            &signature,
        )
        .map_err(|error| format!("The publisher signature is not trusted: {error}"))
}

fn validate_publisher_identifier(field: &str, value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.len() > 128
        || value.trim() != value
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_' | b'.' | b':' | b'@')
        })
    {
        return Err(format!(
            "Publisher {field} must contain 1..=128 lowercase ASCII identifier characters."
        ));
    }
    Ok(())
}

#[derive(Serialize)]
struct PackageDigestContract<'a> {
    schema: &'a str,
    version: u16,
    includes_builtin_frame_references: bool,
    includes_source_metadata: bool,
    builtin_frame_references: &'a [BuiltinFrameReference],
    source_metadata: &'a Option<PresetPackageSourceMetadata>,
    publisher_identity: Option<PackagePublisherIdentity<'a>>,
    presets: &'a [PortableDrawingSheetPreset],
}

#[derive(Serialize)]
struct PackagePublisherIdentity<'a> {
    publisher_id: &'a str,
    signing_key_id: &'a str,
}

pub fn canonical_package_contract(package: &DrawingSheetPresetPackage) -> Result<Vec<u8>, String> {
    let contract = PackageDigestContract {
        schema: &package.schema,
        version: package.version,
        includes_builtin_frame_references: package.includes_builtin_frame_references,
        includes_source_metadata: package.includes_source_metadata,
        builtin_frame_references: &package.builtin_frame_references,
        source_metadata: &package.source_metadata,
        publisher_identity: package.publisher_signature.as_ref().map(|publisher| {
            PackagePublisherIdentity {
                publisher_id: &publisher.publisher_id,
                signing_key_id: &publisher.signing_key_id,
            }
        }),
        presets: &package.presets,
    };
    serde_json::to_vec(&contract)
        .map_err(|error| format!("Could not canonicalize the sheet-format package: {error}"))
}

pub fn package_digest(package: &DrawingSheetPresetPackage) -> Result<String, String> {
    Ok(Sha256::digest(canonical_package_contract(package)?)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

pub fn package_signature_message(package: &DrawingSheetPresetPackage) -> Result<Vec<u8>, String> {
    let canonical = canonical_package_contract(package)?;
    let mut message = Vec::with_capacity(PACKAGE_SIGNATURE_DOMAIN.len() + canonical.len());
    message.extend_from_slice(PACKAGE_SIGNATURE_DOMAIN);
    message.extend_from_slice(&canonical);
    Ok(message)
}

fn scope_rank(scope: DrawingSheetPresetScope) -> u8 {
    match scope {
        DrawingSheetPresetScope::Project => 0,
        DrawingSheetPresetScope::User => 1,
        DrawingSheetPresetScope::Organization => 2,
    }
}

fn source_metadata(presets: &[PortableDrawingSheetPreset]) -> PresetPackageSourceMetadata {
    let mut scopes = presets
        .iter()
        .map(|preset| preset.source_scope)
        .collect::<Vec<_>>();
    scopes.sort_by_key(|scope| scope_rank(*scope));
    scopes.dedup();
    PresetPackageSourceMetadata {
        producer: "RSpice".to_owned(),
        content: "drawing-sheet-format-definitions".to_owned(),
        exact_dimension_unit: "micrometre".to_owned(),
        source_scopes: scopes,
    }
}

fn builtin_frame_references(presets: &[PortableDrawingSheetPreset]) -> Vec<BuiltinFrameReference> {
    let mut references = BTreeSet::new();
    for preset in presets {
        let border = match preset.format.border {
            DrawingSheetBorderTemplate::Standard => Some("standard"),
            DrawingSheetBorderTemplate::Plain => Some("plain"),
            DrawingSheetBorderTemplate::None | DrawingSheetBorderTemplate::OrganizationManaged => {
                None
            }
        };
        if let Some(identity) = border {
            references.insert(BuiltinFrameReference {
                kind: "border".to_owned(),
                identity: identity.to_owned(),
            });
        }
        let title_block = match preset.format.title_block.template {
            DrawingSheetTitleBlockTemplate::Compact => Some("compact"),
            DrawingSheetTitleBlockTemplate::Standard => Some("standard"),
            DrawingSheetTitleBlockTemplate::Wide => Some("wide"),
            DrawingSheetTitleBlockTemplate::None
            | DrawingSheetTitleBlockTemplate::OrganizationManaged => None,
        };
        if let Some(identity) = title_block {
            references.insert(BuiltinFrameReference {
                kind: "title-block".to_owned(),
                identity: identity.to_owned(),
            });
        }
    }
    references.into_iter().collect()
}

fn human_review_rows(presets: &[PortableDrawingSheetPreset]) -> Vec<HumanReviewPreset> {
    presets
        .iter()
        .map(|preset| {
            let mut dependencies = Vec::new();
            dependencies.push(format!("border:{}", border_identity(preset.format.border)));
            dependencies.push(format!(
                "title-block:{}",
                title_block_identity(preset.format.title_block.template)
            ));
            HumanReviewPreset {
                stable_id: preset.stable_id.clone(),
                name: preset.name.clone(),
                source_scope: preset.source_scope,
                portrait_dimensions: format!(
                    "{} \u{00d7} {} mm",
                    format_millimetres(preset.portrait_width_um),
                    format_millimetres(preset.portrait_height_um)
                ),
                portrait_width_um: preset.portrait_width_um,
                portrait_height_um: preset.portrait_height_um,
                dependencies,
            }
        })
        .collect()
}

fn validate_human_review_document(document: &HumanReviewPackageDocument) -> Result<(), String> {
    if document.document != "rspice-sheet-formats-human-review"
        || document.schema.identity != PACKAGE_SCHEMA
        || document.schema.version != PACKAGE_VERSION
        || document.schema.representation != "human-review"
        || document.schema.exact_dimension_unit != "micrometre"
    {
        return Err("Unsupported human-review sheet-format schema.".to_owned());
    }
    if document.review != human_review_rows(&document.package.presets) {
        return Err(
            "The human-review projection does not match the canonical preset definitions."
                .to_owned(),
        );
    }
    Ok(())
}

fn border_identity(template: DrawingSheetBorderTemplate) -> &'static str {
    match template {
        DrawingSheetBorderTemplate::Standard => "builtin/standard",
        DrawingSheetBorderTemplate::Plain => "builtin/plain",
        DrawingSheetBorderTemplate::None => "none",
        DrawingSheetBorderTemplate::OrganizationManaged => "organization-managed",
    }
}

fn title_block_identity(template: DrawingSheetTitleBlockTemplate) -> &'static str {
    match template {
        DrawingSheetTitleBlockTemplate::Compact => "builtin/compact",
        DrawingSheetTitleBlockTemplate::Standard => "builtin/standard",
        DrawingSheetTitleBlockTemplate::Wide => "builtin/wide",
        DrawingSheetTitleBlockTemplate::OrganizationManaged => "organization-managed",
        DrawingSheetTitleBlockTemplate::None => "none",
    }
}

fn format_millimetres(value_um: u64) -> String {
    let whole = value_um / 1_000;
    let fraction = value_um % 1_000;
    if fraction == 0 {
        whole.to_string()
    } else {
        format!("{whole}.{fraction:03}")
            .trim_end_matches('0')
            .to_owned()
    }
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::Signer as _;

    use crate::sheet_authoring::{StartingFrame, custom_format};

    use super::*;

    /// Stand-in for the application's revocation-aware trust store. It
    /// resolves one key per publisher and refuses revoked keys, which is the
    /// whole authority the contract consults.
    #[derive(Clone, Default)]
    struct PdkPublisherTrustStore {
        keys: Vec<TrustedPdkPublisherKey>,
    }

    #[derive(Clone)]
    struct TrustedPdkPublisherKey {
        publisher_id: String,
        key_id: String,
        verifying_key: [u8; 32],
        revoked: bool,
    }

    impl PublisherTrust for PdkPublisherTrustStore {
        fn verify_publisher_signature(
            &self,
            publisher_id: &str,
            key_id: &str,
            message: &[u8],
            signature: &[u8],
        ) -> Result<(), String> {
            let key = self
                .keys
                .iter()
                .find(|key| key.publisher_id == publisher_id && key.key_id == key_id)
                .ok_or_else(|| format!("{publisher_id}/{key_id} is not a trusted publisher key"))?;
            if key.revoked {
                return Err(format!("{publisher_id}/{key_id} is revoked"));
            }
            let verifying_key = VerifyingKey::from_bytes(&key.verifying_key)
                .map_err(|error| format!("trusted key is invalid: {error}"))?;
            let signature: [u8; 64] = signature
                .try_into()
                .map_err(|_| "signature is not 64 bytes".to_owned())?;
            verifying_key
                .verify_strict(message, &Signature::from_bytes(&signature))
                .map_err(|_| format!("signature is invalid for {publisher_id}/{key_id}"))
        }
    }

    fn build_package(
        presets: impl IntoIterator<Item = DrawingSheetPreset>,
    ) -> Result<DrawingSheetPresetPackage, String> {
        build_package_with_options(presets, true, true)
    }

    fn encode_package(package: &DrawingSheetPresetPackage) -> Result<String, String> {
        encode_package_with_format(package, DrawingSheetPackageEncoding::CanonicalSchema1)
    }

    fn signed_organization_package() -> (
        DrawingSheetPresetPackage,
        PdkPublisherTrustStore,
        SigningKey,
    ) {
        let preset = DrawingSheetPreset {
            id: "organization-panel".to_owned(),
            name: "Organization panel".to_owned(),
            scope: DrawingSheetPresetScope::Organization,
            format: custom_format("Organization panel", 250_000, 400_000, StartingFrame::IsoA)
                .unwrap(),
        };
        let signing_key = SigningKey::from_bytes(&[0x53; 32]);
        let mut package = build_package([preset]).unwrap();
        package.publisher_signature = Some(DrawingSheetPresetPublisherSignature {
            publisher_id: "rspice-test-publisher".to_owned(),
            signing_key_id: "sheet-formats-2026".to_owned(),
            signature_base64: String::new(),
        });
        package.source_digest_sha256 = package_digest(&package).unwrap();
        let signature = signing_key.sign(&package_signature_message(&package).unwrap());
        package
            .publisher_signature
            .as_mut()
            .unwrap()
            .signature_base64 = STANDARD.encode(signature.to_bytes());
        let mut trust = PdkPublisherTrustStore::default();
        trust.keys.push(TrustedPdkPublisherKey {
            publisher_id: "rspice-test-publisher".to_owned(),
            key_id: "sheet-formats-2026".to_owned(),
            verifying_key: signing_key.verifying_key().to_bytes(),
            revoked: false,
        });
        (package, trust, signing_key)
    }

    #[test]
    fn packages_preserve_exact_snapshot_and_detect_tampering() {
        let preset = DrawingSheetPreset {
            id: "custom-lab".to_owned(),
            name: "Lab panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Lab panel", 250_001, 400_002, StartingFrame::IsoA).unwrap(),
        };
        let package = build_package([preset]).unwrap();
        let encoded = encode_package(&package).unwrap();
        let restored = parse_package(&encoded, &PdkPublisherTrustStore::default()).unwrap();
        assert_eq!(restored.presets[0].portrait_width_um, 250_001);
        assert_eq!(restored.presets[0].portrait_height_um, 400_002);

        let tampered = encoded.replace("250001", "250002");
        assert!(parse_package(&tampered, &PdkPublisherTrustStore::default()).is_err());
    }

    #[test]
    fn offline_publisher_promotes_signs_and_self_verifies_the_import_contract() {
        let preset = DrawingSheetPreset {
            id: "publisher-panel".to_owned(),
            name: "Publisher panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Publisher panel", 250_003, 400_007, StartingFrame::IsoA)
                .unwrap(),
        };
        let unsigned = encode_package(&build_package([preset]).unwrap()).unwrap();
        let unsigned_inspection = inspect_drawing_sheet_package(&unsigned).unwrap();
        assert!(!unsigned_inspection.is_signed);
        assert_eq!(unsigned_inspection.preset_count, 1);
        assert_eq!(unsigned_inspection.organization_preset_count, 0);
        let seed = [0x29; 32];

        for encoding in [
            DrawingSheetPackageEncoding::CanonicalSchema1,
            DrawingSheetPackageEncoding::HumanReviewJson,
        ] {
            let published = publish_organization_drawing_sheet_package(
                &unsigned,
                &seed,
                "acme.eda",
                "drawing-sheets-2026",
                true,
                encoding,
            )
            .unwrap();
            assert_eq!(published.preset_count, 1);
            assert_eq!(
                published.verifying_key,
                drawing_sheet_publisher_public_key(&seed)
            );

            let verified = verify_published_drawing_sheet_package(
                &published.encoded,
                &published.verifying_key,
            )
            .unwrap();
            assert_eq!(verified.publisher_id, "acme.eda");
            assert_eq!(verified.signing_key_id, "drawing-sheets-2026");
            assert_eq!(verified.organization_preset_count, 1);
            assert_eq!(
                verified.source_digest_sha256,
                published.source_digest_sha256
            );
            let signed_inspection = inspect_drawing_sheet_package(&published.encoded).unwrap();
            assert!(signed_inspection.is_signed);
            assert_eq!(
                signed_inspection.source_digest_sha256,
                published.source_digest_sha256
            );

            let mut trust = PdkPublisherTrustStore::default();
            trust.keys.push(TrustedPdkPublisherKey {
                publisher_id: verified.publisher_id,
                key_id: verified.signing_key_id,
                verifying_key: published.verifying_key,
                revoked: false,
            });
            let imported = parse_package(&published.encoded, &trust).unwrap();
            assert_eq!(
                imported.presets[0].source_scope,
                DrawingSheetPresetScope::Organization
            );
        }
    }

    #[test]
    fn offline_publisher_rejects_implicit_scope_changes_and_resigning() {
        let preset = DrawingSheetPreset {
            id: "publisher-panel".to_owned(),
            name: "Publisher panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Publisher panel", 250_000, 400_000, StartingFrame::IsoA)
                .unwrap(),
        };
        let unsigned = encode_package(&build_package([preset]).unwrap()).unwrap();
        let seed = [0x31; 32];
        let error = publish_organization_drawing_sheet_package(
            &unsigned,
            &seed,
            "acme.eda",
            "drawing-sheets-2026",
            false,
            DrawingSheetPackageEncoding::CanonicalSchema1,
        )
        .unwrap_err();
        assert!(error.contains("explicitly promote"));

        let signed = publish_organization_drawing_sheet_package(
            &unsigned,
            &seed,
            "acme.eda",
            "drawing-sheets-2026",
            true,
            DrawingSheetPackageEncoding::CanonicalSchema1,
        )
        .unwrap();
        let error = publish_organization_drawing_sheet_package(
            &signed.encoded,
            &seed,
            "acme.eda",
            "drawing-sheets-2027",
            false,
            DrawingSheetPackageEncoding::CanonicalSchema1,
        )
        .unwrap_err();
        assert!(error.contains("already signed"));
    }

    #[test]
    fn explicit_public_key_verification_detects_content_and_key_tampering() {
        let preset = DrawingSheetPreset {
            id: "publisher-panel".to_owned(),
            name: "Publisher panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Publisher panel", 250_000, 400_000, StartingFrame::IsoA)
                .unwrap(),
        };
        let unsigned = encode_package(&build_package([preset]).unwrap()).unwrap();
        let published = publish_organization_drawing_sheet_package(
            &unsigned,
            &[0x42; 32],
            "acme.eda",
            "drawing-sheets-2026",
            true,
            DrawingSheetPackageEncoding::CanonicalSchema1,
        )
        .unwrap();

        assert!(
            verify_published_drawing_sheet_package(
                &published.encoded,
                &drawing_sheet_publisher_public_key(&[0x43; 32])
            )
            .is_err()
        );
        let tampered = published.encoded.replace("250000", "250001");
        assert!(
            verify_published_drawing_sheet_package(&tampered, &published.verifying_key).is_err()
        );
    }

    #[test]
    fn organization_packages_require_a_current_trusted_publisher_signature() {
        let (package, trust, _) = signed_organization_package();
        for format in [
            DrawingSheetPackageEncoding::CanonicalSchema1,
            DrawingSheetPackageEncoding::HumanReviewJson,
        ] {
            let encoded = encode_package_with_format(&package, format).unwrap();
            assert_eq!(parse_package(&encoded, &trust).unwrap(), package);
        }

        let mut revoked = trust.clone();
        revoked.keys[0].revoked = true;
        let encoded = encode_package(&package).unwrap();
        assert!(parse_package(&encoded, &revoked).is_err());
        assert!(parse_package(&encoded, &PdkPublisherTrustStore::default()).is_err());
    }

    #[test]
    fn unsigned_organization_packages_fail_closed() {
        let (mut package, trust, _) = signed_organization_package();
        package.publisher_signature = None;
        package.source_digest_sha256 = package_digest(&package).unwrap();
        assert!(encode_package(&package).is_err());

        let encoded = serde_json::to_string(&package).unwrap();
        let error = parse_package(&encoded, &trust).unwrap_err();
        assert!(error.contains("require a publisher signature"));
    }

    #[test]
    fn organization_signature_binds_content_identity_and_domain() {
        let (package, trust, signing_key) = signed_organization_package();

        let mut content_tampered = package.clone();
        content_tampered.presets[0].portrait_width_um += 1;
        let AuthoredDrawingSheetSize::Custom { snapshot } =
            &mut content_tampered.presets[0].format.authored_size
        else {
            panic!("fixture is custom");
        };
        snapshot.portrait_width_um += 1;
        content_tampered.source_digest_sha256 = package_digest(&content_tampered).unwrap();
        let encoded = serde_json::to_string(&content_tampered).unwrap();
        assert!(parse_package(&encoded, &trust).is_err());

        let mut identity_tampered = package.clone();
        identity_tampered
            .publisher_signature
            .as_mut()
            .unwrap()
            .signing_key_id = "unknown-key".to_owned();
        identity_tampered.source_digest_sha256 = package_digest(&identity_tampered).unwrap();
        let encoded = serde_json::to_string(&identity_tampered).unwrap();
        assert!(parse_package(&encoded, &trust).is_err());

        let mut wrong_domain = package;
        let signature = signing_key.sign(&canonical_package_contract(&wrong_domain).unwrap());
        wrong_domain
            .publisher_signature
            .as_mut()
            .unwrap()
            .signature_base64 = STANDARD.encode(signature.to_bytes());
        let encoded = serde_json::to_string(&wrong_domain).unwrap();
        assert!(parse_package(&encoded, &trust).is_err());
    }

    #[test]
    fn signed_package_schema_downgrades_and_unknown_fields_are_rejected() {
        let (package, trust, signing_key) = signed_organization_package();
        let mut downgraded = package.clone();
        downgraded.version = 0;
        downgraded.source_digest_sha256 = package_digest(&downgraded).unwrap();
        let signature = signing_key.sign(&package_signature_message(&downgraded).unwrap());
        downgraded
            .publisher_signature
            .as_mut()
            .unwrap()
            .signature_base64 = STANDARD.encode(signature.to_bytes());
        assert!(parse_package(&serde_json::to_string(&downgraded).unwrap(), &trust).is_err());

        let mut value = serde_json::to_value(package).unwrap();
        value.as_object_mut().unwrap().insert(
            "unsigned_override".to_owned(),
            serde_json::Value::Bool(true),
        );
        assert!(parse_package(&serde_json::to_string(&value).unwrap(), &trust).is_err());
    }

    #[test]
    fn package_options_are_retained_and_covered_by_the_digest() {
        let preset = DrawingSheetPreset {
            id: "custom-lab".to_owned(),
            name: "Lab panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Lab panel", 250_000, 400_000, StartingFrame::IsoA).unwrap(),
        };
        let complete = build_package_with_options([preset.clone()], true, true).unwrap();
        assert!(complete.includes_builtin_frame_references);
        assert!(complete.includes_source_metadata);
        assert!(!complete.builtin_frame_references.is_empty());
        assert!(complete.source_metadata.is_some());

        let minimal = build_package_with_options([preset], false, false).unwrap();
        assert!(!minimal.includes_builtin_frame_references);
        assert!(!minimal.includes_source_metadata);
        assert!(minimal.builtin_frame_references.is_empty());
        assert!(minimal.source_metadata.is_none());
        assert_ne!(complete.source_digest_sha256, minimal.source_digest_sha256);
        let minimal_json = encode_package(&minimal).unwrap();
        assert!(!minimal_json.contains("\"builtin_frame_references\""));
        assert!(!minimal_json.contains("\"source_metadata\""));

        let mut reference_tamper = complete.clone();
        reference_tamper.builtin_frame_references[0].identity = "forged".to_owned();
        assert!(validate_package(&reference_tamper).is_err());
        let mut metadata_tamper = complete;
        metadata_tamper.source_metadata.as_mut().unwrap().producer = "Unknown".to_owned();
        assert!(validate_package(&metadata_tamper).is_err());

        let mut tampered = minimal;
        tampered.includes_source_metadata = true;
        assert!(validate_package(&tampered).is_err());
    }

    #[test]
    fn both_export_representations_are_deterministic_and_importable() {
        let first = DrawingSheetPreset {
            id: "z-strip".to_owned(),
            name: "Z strip".to_owned(),
            scope: DrawingSheetPresetScope::User,
            format: custom_format("Z strip", 210_000, 594_000, StartingFrame::Plain).unwrap(),
        };
        let second = DrawingSheetPreset {
            id: "a-panel".to_owned(),
            name: "A panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("A panel", 250_001, 400_002, StartingFrame::IsoA).unwrap(),
        };
        let package =
            build_package_with_options([first.clone(), second.clone()], true, true).unwrap();
        let reordered = build_package_with_options([second, first], true, true).unwrap();
        assert_eq!(package, reordered);

        for format in [
            DrawingSheetPackageEncoding::CanonicalSchema1,
            DrawingSheetPackageEncoding::HumanReviewJson,
        ] {
            let encoded = encode_package_with_format(&package, format).unwrap();
            assert_eq!(
                encoded,
                encode_package_with_format(&package, format).unwrap()
            );
            assert_eq!(
                parse_package(&encoded, &PdkPublisherTrustStore::default()).unwrap(),
                package
            );
        }
    }

    #[test]
    fn human_review_projection_is_validated_against_canonical_definitions() {
        let preset = DrawingSheetPreset {
            id: "custom-lab".to_owned(),
            name: "Lab panel".to_owned(),
            scope: DrawingSheetPresetScope::Project,
            format: custom_format("Lab panel", 250_001, 400_002, StartingFrame::IsoA).unwrap(),
        };
        let package = build_package([preset]).unwrap();
        let encoded =
            encode_package_with_format(&package, DrawingSheetPackageEncoding::HumanReviewJson)
                .unwrap();
        let tampered = encoded.replace("250.001 \u{00d7} 400.002 mm", "250 \u{00d7} 400 mm");
        assert!(parse_package(&tampered, &PdkPublisherTrustStore::default()).is_err());
    }
}

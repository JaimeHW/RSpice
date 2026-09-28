//! Owned SPICE source metadata, bounded history, and compatibility contracts.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod profile;
#[cfg(test)]
mod profile_tests;
mod validation;

pub use profile::NetlistExecutionProfile;
pub use validation::{validate_owned_netlist_artifact_path, validate_owned_netlist_projection};

/// Editing strategy for a project-owned SPICE artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum OwnedNetlistEditStrategy {
    #[default]
    OwnedSource,
    ParameterOptionOverride,
    IncludeOrderOverride,
    AnalysisOnlyDeck,
}

impl OwnedNetlistEditStrategy {
    pub const ALL: [Self; 4] = [
        Self::OwnedSource,
        Self::ParameterOptionOverride,
        Self::IncludeOrderOverride,
        Self::AnalysisOnlyDeck,
    ];
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistSaveRecord {
    pub document_revision: u64,
    pub content_digest: rspice_app_types::product::ContentDigest,
    pub message: String,
}

pub const MAX_OWNED_NETLIST_HISTORY_REVISIONS: usize = 64;
pub const MAX_OWNED_NETLIST_HISTORY_BYTES: usize = 16 * 1024 * 1024;

/// Content-complete, bounded revision evidence for a project-owned deck.
/// Dependency bytes are retained with the authored root so compare, revert,
/// recovery, and merge never have to reopen an ambient filesystem path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistRevisionSnapshot {
    pub document_revision: u64,
    pub content_digest: rspice_app_types::product::ContentDigest,
    pub source: String,
    #[serde(default)]
    pub dependencies: Vec<crate::netlist_document::DependencyMetadata>,
    /// Exact include-document ownership catalog at this revision. Restoring a
    /// root snapshot restores ownership and dependency bytes together so no
    /// stale editable authority can survive across history boundaries.
    #[serde(default)]
    pub owned_includes: Vec<OwnedNetlistIncludeDescriptor>,
    pub message: String,
    #[serde(default)]
    pub source_encoding: NetlistTextEncoding,
    #[serde(default)]
    pub source_line_ending: NetlistLineEnding,
}

/// Stable project ownership for one dependency document retained by an owned
/// netlist. The source bytes remain canonical in `NetlistDocument` so
/// execution, editor, history, and archive export cannot diverge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistIncludeDescriptor {
    pub document_id: Uuid,
    pub logical_identity: String,
    pub display_name: String,
    pub revision: u64,
    pub content_digest: rspice_app_types::product::ContentDigest,
}

impl OwnedNetlistIncludeDescriptor {
    pub fn try_new(
        dependency: &crate::netlist_document::DependencyMetadata,
    ) -> Result<Self, String> {
        let source = dependency.source().ok_or_else(|| {
            "Only a resolved dependency can be copied into the project.".to_owned()
        })?;
        let value = Self {
            document_id: Uuid::new_v4(),
            logical_identity: dependency.locator().logical_identity().to_owned(),
            display_name: dependency.locator().display_name().to_owned(),
            revision: 1,
            content_digest: crate::netlist_document::content_digest(source),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.document_id.is_nil() {
            return Err("owned include document identity cannot be nil".to_owned());
        }
        if self.logical_identity.trim().is_empty()
            || self.logical_identity != self.logical_identity.trim()
            || self.logical_identity.len() > 4_096
            || self.logical_identity.chars().any(char::is_control)
        {
            return Err("owned include logical identity is invalid".to_owned());
        }
        if self.display_name.trim().is_empty()
            || self.display_name != self.display_name.trim()
            || self.display_name.len() > 4_096
            || self.display_name.chars().any(char::is_control)
            || self.revision == 0
        {
            return Err("owned include display name or revision is invalid".to_owned());
        }
        Ok(())
    }
}

/// Encoding used by a project-owned netlist at its durable file boundary.
///
/// The editor and parser operate on Rust UTF-8 strings, but an imported deck
/// can legitimately be UTF-8 with a BOM, UTF-16, or legacy ISO-8859-1. The
/// encoding is therefore retained as project metadata and reapplied on Save;
/// RSpice never silently converts an imported source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetlistTextEncoding {
    #[default]
    Utf8,
    Utf8Bom,
    Utf16LeBom,
    Utf16BeBom,
    Latin1,
}

impl NetlistTextEncoding {
    pub const ALL: [Self; 5] = [
        Self::Utf8,
        Self::Utf8Bom,
        Self::Utf16LeBom,
        Self::Utf16BeBom,
        Self::Latin1,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Utf8Bom => "UTF-8 with BOM",
            Self::Utf16LeBom => "UTF-16 LE with BOM",
            Self::Utf16BeBom => "UTF-16 BE with BOM",
            Self::Latin1 => "ISO-8859-1",
        }
    }

    pub fn encode(self, source: &str) -> Result<Vec<u8>, String> {
        match self {
            Self::Utf8 => Ok(source.as_bytes().to_vec()),
            Self::Utf8Bom => {
                let mut bytes = Vec::with_capacity(source.len().saturating_add(3));
                bytes.extend_from_slice(&[0xef, 0xbb, 0xbf]);
                bytes.extend_from_slice(source.as_bytes());
                Ok(bytes)
            }
            Self::Utf16LeBom | Self::Utf16BeBom => {
                let mut bytes =
                    Vec::with_capacity(source.len().saturating_mul(2).saturating_add(2));
                bytes.extend_from_slice(if self == Self::Utf16LeBom {
                    &[0xff, 0xfe]
                } else {
                    &[0xfe, 0xff]
                });
                for unit in source.encode_utf16() {
                    let encoded = if self == Self::Utf16LeBom {
                        unit.to_le_bytes()
                    } else {
                        unit.to_be_bytes()
                    };
                    bytes.extend_from_slice(&encoded);
                }
                Ok(bytes)
            }
            Self::Latin1 => {
                let mut bytes = Vec::with_capacity(source.chars().count());
                for (character_index, character) in source.chars().enumerate() {
                    let value = u32::from(character);
                    if value > u32::from(u8::MAX) {
                        return Err(format!(
                            "character {} (U+{value:04X}) cannot be represented in ISO-8859-1; use Save As with UTF-8 or remove the character",
                            character_index + 1
                        ));
                    }
                    bytes.push(value as u8);
                }
                Ok(bytes)
            }
        }
    }
}

/// Line-ending form observed in the imported source. Source text retains its
/// exact separators; this value is durable evidence for the editor status and
/// import review rather than a request to rewrite the deck.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetlistLineEnding {
    #[default]
    None,
    Lf,
    Crlf,
    Cr,
    Mixed,
}

impl NetlistLineEnding {
    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "no line terminators",
            Self::Lf => "LF",
            Self::Crlf => "CRLF",
            Self::Cr => "CR",
            Self::Mixed => "mixed",
        }
    }

    pub fn detect(source: &str) -> Self {
        let bytes = source.as_bytes();
        let mut lf = 0usize;
        let mut crlf = 0usize;
        let mut cr = 0usize;
        let mut index = 0usize;
        while index < bytes.len() {
            match bytes[index] {
                b'\r' if bytes.get(index + 1) == Some(&b'\n') => {
                    crlf += 1;
                    index += 2;
                }
                b'\r' => {
                    cr += 1;
                    index += 1;
                }
                b'\n' => {
                    lf += 1;
                    index += 1;
                }
                _ => index += 1,
            }
        }
        match (lf > 0, crlf > 0, cr > 0) {
            (false, false, false) => Self::None,
            (true, false, false) => Self::Lf,
            (false, true, false) => Self::Crlf,
            (false, false, true) => Self::Cr,
            _ => Self::Mixed,
        }
    }
}

/// Declared source dialect retained with an imported owned deck. Detection is
/// advisory; a non-native dialect requires an explicit compatibility review
/// before the staged import can commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum NetlistSourceDialect {
    #[default]
    RSpice,
    Spice3Ngspice,
    Hspice,
    Pspice,
    Spectre,
    Ads,
    Unknown,
}

impl NetlistSourceDialect {
    pub const ALL: [Self; 7] = [
        Self::RSpice,
        Self::Spice3Ngspice,
        Self::Hspice,
        Self::Pspice,
        Self::Spectre,
        Self::Ads,
        Self::Unknown,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::RSpice => "RSpice canonical SPICE",
            Self::Spice3Ngspice => "SPICE3 / ngspice",
            Self::Hspice => "HSPICE",
            Self::Pspice => "PSpice",
            Self::Spectre => "Cadence Spectre",
            Self::Ads => "Keysight ADS netlist",
            Self::Unknown => "Unknown SPICE-family dialect",
        }
    }

    pub const fn requires_compatibility_review(self) -> bool {
        !matches!(self, Self::RSpice)
    }

    /// Exact executable semantics qualified for this source dialect.
    ///
    /// This is intentionally fallible. A display label or a completed review
    /// must never manufacture an execution adapter for a vendor dialect.
    pub const fn execution_profile(self) -> Option<NetlistExecutionProfile> {
        match self {
            Self::RSpice => Some(NetlistExecutionProfile::RSpiceCanonicalV1),
            Self::Spice3Ngspice => Some(NetlistExecutionProfile::Spice3NgspiceV2),
            Self::Hspice => Some(NetlistExecutionProfile::HspiceDeclarativeV1),
            Self::Pspice => Some(NetlistExecutionProfile::PspiceDeclarativeV2),
            Self::Spectre => Some(NetlistExecutionProfile::SpectreSpiceV1),
            Self::Ads => Some(NetlistExecutionProfile::AdsSpiceExportV1),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnedNetlistDescriptor {
    /// Stable project identity for this top-level deck. This identity follows
    /// the deck across logical rename/move operations and remains distinct when
    /// a deck is duplicated.
    #[serde(default)]
    pub deck_id: Uuid,
    pub artifact_name: String,
    pub strategy: OwnedNetlistEditStrategy,
    #[serde(default)]
    pub source_encoding: NetlistTextEncoding,
    #[serde(default)]
    pub source_line_ending: NetlistLineEnding,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_dialect: Option<NetlistSourceDialect>,
    #[serde(default)]
    pub compatibility_reviewed: bool,
    /// Exact reviewed execution semantics. Legacy canonical projects may omit
    /// this field; a non-canonical dialect without an exact profile is always
    /// rejected at preflight and must be reviewed again.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_profile: Option<NetlistExecutionProfile>,
    /// SHA-256 of the last imported or successfully published raw file. This
    /// is the compare-and-exchange baseline used by ordinary Save so an
    /// external edit is never overwritten silently.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_file_sha256: Option<[u8; 32]>,
    #[serde(default)]
    pub save_history: Vec<OwnedNetlistSaveRecord>,
    /// Exact project-persisted source/dependency snapshots. This is separate
    /// from the compact save ledger because recovery may retain an unsaved
    /// pre-restore working revision as well as externally published states.
    #[serde(default)]
    pub revision_history: Vec<OwnedNetlistRevisionSnapshot>,
    /// Explicit copy-to-project ownership for dependency documents. Any
    /// retained dependency absent from this list remains find-only/read-only.
    #[serde(default)]
    pub owned_includes: Vec<OwnedNetlistIncludeDescriptor>,
}

/// One inactive, project-owned top-level deck. The active deck continues to
/// use the long-standing `netlist_*` workspace fields as the single execution
/// authority; this catalog retains complete inactive documents so switching
/// decks is an atomic swap rather than a lossy import.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedOwnedNetlistDeck {
    pub descriptor: OwnedNetlistDescriptor,
    pub document: crate::netlist_document::NetlistDocument,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_path: Option<PathBuf>,
}

impl OwnedNetlistDescriptor {
    /// Whether this source is intentionally retained but cannot execute until
    /// its exact current bytes receive a versioned compatibility receipt.
    #[must_use]
    pub fn execution_profile_review_required(&self) -> bool {
        let dialect = self
            .imported_dialect
            .unwrap_or(NetlistSourceDialect::RSpice);
        dialect.requires_compatibility_review()
            && (!self.compatibility_reviewed
                || self.execution_profile != dialect.execution_profile()
                || self.execution_profile.is_none())
    }

    #[must_use]
    pub fn owned_include(&self, logical_identity: &str) -> Option<&OwnedNetlistIncludeDescriptor> {
        self.owned_includes
            .iter()
            .find(|include| include.logical_identity == logical_identity)
    }

    pub fn retain_revision(
        &mut self,
        document: &crate::netlist_document::NetlistDocument,
        message: impl Into<String>,
    ) -> Result<(), String> {
        let mut snapshot = OwnedNetlistRevisionSnapshot::from_document(
            document,
            message,
            self.source_encoding,
            self.source_line_ending,
        )?;
        snapshot.owned_includes.clone_from(&self.owned_includes);
        snapshot.validate()?;
        if let Some(last) = self.revision_history.last() {
            if last.document_revision == snapshot.document_revision
                && last.content_digest == snapshot.content_digest
            {
                return Ok(());
            }
            if last.document_revision >= snapshot.document_revision {
                return Err(
                    "owned netlist history cannot append a non-monotonic revision".to_owned(),
                );
            }
        }
        if snapshot.retained_bytes() > MAX_OWNED_NETLIST_HISTORY_BYTES {
            return Err("owned netlist revision exceeds the bounded history size".to_owned());
        }
        let mut next = self.revision_history.clone();
        next.push(snapshot);
        let mut retained_bytes = next
            .iter()
            .map(OwnedNetlistRevisionSnapshot::retained_bytes)
            .sum::<usize>();
        while next.len() > MAX_OWNED_NETLIST_HISTORY_REVISIONS
            || (retained_bytes > MAX_OWNED_NETLIST_HISTORY_BYTES && next.len() > 1)
        {
            retained_bytes = retained_bytes.saturating_sub(next[0].retained_bytes());
            next.remove(0);
        }
        self.revision_history = next;
        Ok(())
    }
}

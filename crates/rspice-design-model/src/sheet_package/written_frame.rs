//! The frame a preset was written with before every sheet took its series'
//! standard frame.
//!
//! A format used to store its own margins, a plain or organization border,
//! custom zone counts, a choice of zone labels and lettered edges, and corner
//! registration and folding flags. A preset written that way loads onto the
//! standard frame, but its package's digest and signature were sealed over
//! the frame as it was written, and its frame-reference manifest and review
//! rows name the border it was written with.

use serde::{Deserialize, Serialize};

use crate::design_management::SchematicSheetFormat;

/// A preset format's frame as it was written before the standard frames:
/// the fields between the format's display unit and its title block, in the
/// order and shape every build before them wrote.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct WrittenFrame {
    margins: WrittenMargins,
    bleed_um: u64,
    border: WrittenBorder,
    zones: WrittenZones,
    marks: WrittenMarks,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenMargins {
    top_um: u64,
    right_um: u64,
    bottom_um: u64,
    left_um: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum WrittenBorder {
    Standard,
    Plain,
    None,
    OrganizationManaged,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenZones {
    mode: String,
    custom_columns: Option<u8>,
    custom_rows: Option<u8>,
    labels: String,
    edges: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WrittenMarks {
    registration: bool,
    folding: bool,
}

impl WrittenFrame {
    /// The frame `format`, a preset's format as its package holds it, was
    /// written with, if it was written in that shape.
    pub(super) fn of(format: &serde_json::Value) -> Option<Self> {
        let format = format.as_object()?;
        let frame = ["margins", "bleed_um", "border", "zones", "marks"]
            .into_iter()
            .map(|key| Some((key.to_owned(), format.get(key)?.clone())))
            .collect::<Option<serde_json::Map<_, _>>>()?;
        serde_json::from_value(serde_json::Value::Object(frame)).ok()
    }

    /// The frame as the canonical contract wrote it.
    pub(super) fn span(&self) -> Vec<u8> {
        let object = serde_json::to_vec(self).unwrap_or_default();
        object
            .get(1..object.len().saturating_sub(1))
            .unwrap_or_default()
            .to_vec()
    }

    /// The identity the frame-reference manifest gave the border.
    pub(super) const fn border_reference(&self) -> Option<&'static str> {
        match self.border {
            WrittenBorder::Standard => Some("standard"),
            WrittenBorder::Plain => Some("plain"),
            WrittenBorder::None | WrittenBorder::OrganizationManaged => None,
        }
    }

    /// The identity the review rows gave the border.
    pub(super) const fn border_identity(&self) -> &'static str {
        match self.border {
            WrittenBorder::Standard => "builtin/standard",
            WrittenBorder::Plain => "builtin/plain",
            WrittenBorder::None => "none",
            WrittenBorder::OrganizationManaged => "organization-managed",
        }
    }
}

/// `format`'s frame as the canonical contract writes it today: everything
/// between its display unit and its title block.
///
/// Strings are written escaped, so neither key can be matched inside a
/// name, and the display unit is a bare enumeration value.
pub(super) fn frame_span(format: &SchematicSheetFormat) -> Option<Vec<u8>> {
    const DISPLAY_UNIT: &[u8] = br#""display_unit":""#;
    const TITLE_BLOCK: &[u8] = br#","title_block":"#;
    let bytes = serde_json::to_vec(format).ok()?;
    let unit = find(&bytes, DISPLAY_UNIT)? + DISPLAY_UNIT.len();
    // Past the unit's closing quote and the comma after it.
    let start = unit + find(&bytes[unit..], b"\",")? + 2;
    let end = start + find(&bytes[start..], TITLE_BLOCK)?;
    Some(bytes[start..end].to_vec())
}

/// Where `needle` first occurs in `haystack`.
pub(super) fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

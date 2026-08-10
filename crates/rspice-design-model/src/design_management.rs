//! Durable design-management authority for schematic sheets, assembly
//! variants, reference annotation, and hierarchy preflight evidence.
//!
//! The UI only edits drafts. Every mutation in this module is applied to a
//! cloned candidate and committed after complete validation, so malformed or
//! stale dialog input cannot partially change the project. Stable identities,
//! revisions, semantic digests, and immutable receipts are owned here rather
//! than inferred from labels rendered by the workbench.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as ShaDigest, Sha256};
use thiserror::Error;
use unicode_normalization::UnicodeNormalization;
use uuid::Uuid;

use crate::primitives::{
    ConfigurationSetId, ContentDigest, Point, SchematicPageOrientation, SchematicPageSize,
};

pub const DESIGN_MANAGEMENT_SCHEMA_VERSION: u16 = 1;
pub const SHEET_CATALOG_SCHEMA_VERSION: u16 = 1;
pub const VARIANT_CATALOG_SCHEMA_VERSION: u16 = 1;
pub const ANNOTATION_STATE_SCHEMA_VERSION: u16 = 1;
pub const MAX_DESIGN_SHEETS: usize = 1_024;
pub const MAX_SHEET_PORTS: usize = 65_536;
pub const MAX_SHEET_OBJECT_ASSIGNMENTS: usize = 1_000_000;
pub const MAX_ASSEMBLY_VARIANTS: usize = 1_024;
pub const MAX_VARIANT_OVERRIDES: usize = 250_000;
pub const MAX_ANNOTATION_RANGES: usize = 16_384;
pub const MAX_ANNOTATION_JOURNAL_ENTRIES: usize = 16_384;
pub const MAX_ANNOTATION_MAPPINGS_PER_ENTRY: usize = 1_000_000;
pub const MAX_ANNOTATION_OBJECT_AUTHORITIES: usize = 1_000_000;
pub const MAX_HIERARCHY_AUDIT_RECEIPTS: usize = 4_096;
pub const MAX_HIERARCHY_AUDIT_SUBJECTS: usize = 250_000;
pub const MAX_HIERARCHY_AUDIT_FINDINGS: usize = 250_000;
pub const MAX_DESIGN_NAME_BYTES: usize = 128;
pub const MAX_DESIGN_PATH_BYTES: usize = 4_096;
pub const MAX_DESIGN_VALUE_BYTES: usize = 8_192;
pub const MAX_PREFIX_BYTES: usize = 16;

mod validation;
use validation::*;

mod identity;
pub use identity::*;

mod drawing_sheet;
pub use drawing_sheet::*;

mod sheets;
pub use sheets::*;

mod variants;
pub use variants::*;

mod annotation;
pub use annotation::*;

mod hierarchy;
pub use hierarchy::*;

mod catalog;
pub use catalog::*;

mod error;
pub use error::*;

#[cfg(test)]
mod tests;

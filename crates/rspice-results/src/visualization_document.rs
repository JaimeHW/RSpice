//! Canonical, UI-independent domain model for a result document.
//!
//! A visualization document is a versioned presentation over immutable result
//! datasets.  Source samples and their content digests are never edited in
//! place; presentation changes are committed atomically through [`DocumentEdit`].

mod apply;
mod comparison;
mod edits;
mod family;
mod journal;
mod lookup;
mod query;
mod run_source;
mod strips;
mod trace_style;
mod undo;
mod working_view;

pub use comparison::*;
pub use edits::*;
pub use family::*;
pub use query::*;
pub use run_source::*;
pub use strips::*;
pub use trace_style::*;
pub use undo::ViewStep;
pub use working_view::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::num::NonZeroU64;
use std::sync::Arc;

use serde::de::{IgnoredAny, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest as _, Sha256};

use crate::viewer_catalog::{ViewerArt, viewer_document};
use rspice_vocabulary::product::{
    AnalysisInstanceId, ContentDigest, DatasetBinding, DatasetId, ObjectRevision, ResultDocumentId,
    RevisionError, SimulationPlanId,
};

// The bounds below hold against a hostile file, not a reader: each one
// bounds what reading a file can cost beyond the bytes the file holds, and
// a document reads its runs by reference, so none of them bounds a run. A
// count of anything a reader makes (panes, traces, cursors, markers, runs
// compared) has none: checking a document costs in proportion to what it
// holds.

/// The typed cells of one embedded table. An embedded table (an archived or
/// imported one) is held whole in memory, each cell many times its size in
/// the file (a two-byte number is a typed value of tens of bytes), so its
/// cells are counted as the file is read.
pub const MAX_SOURCE_CELLS_PER_DATASET: usize = 8_000_000;
/// The typed cells across the embedded tables of one document, for the same
/// reason.
pub const MAX_SOURCE_CELLS_TOTAL: usize = 8_000_000;
/// Maximum UTF-8 bytes retained by text-valued cells in one source dataset.
pub const MAX_SOURCE_TEXT_BYTES_PER_DATASET: usize = 64 * 1024 * 1024;
/// Maximum UTF-8 bytes retained by text-valued cells across one document.
pub const MAX_SOURCE_TEXT_BYTES_TOTAL: usize = 128 * 1024 * 1024;
/// The length of a stable key in UTF-8 bytes. This and the string bounds
/// below are checked as each string is read, so no one string holds a file's
/// worth of text; each is hashed, compared or drawn whole wherever it is
/// used.
pub const MAX_VISUALIZATION_KEY_BYTES: usize = 256;
/// The length of a name or a label in UTF-8 bytes: a title, a trace's label,
/// and the name of the signal a trace plots. One bound holds for all of them,
/// so any signal name a run can hold is also a label a trace can carry.
pub const MAX_VISUALIZATION_LABEL_BYTES: usize = 4_096;
/// Maximum engineering-unit length in UTF-8 bytes.
pub const MAX_VISUALIZATION_UNIT_BYTES: usize = 64;
/// The length of a text-valued source cell in UTF-8 bytes. A cell can hold a
/// signal's name, so it holds any name a label can.
pub const MAX_SOURCE_TEXT_BYTES: usize = MAX_VISUALIZATION_LABEL_BYTES;
/// Maximum retained annotation length in UTF-8 bytes.
pub const MAX_ANNOTATION_TEXT_BYTES: usize = 16_384;
/// How deeply one family predicate may nest, `Not` nodes included. Reading,
/// checking and evaluating a predicate each recurse once per level, so this
/// is what keeps a hostile document from overflowing the stack; a filter a
/// person writes nests a handful of levels. How many dimensions, members,
/// children or encodings a policy holds is bounded only by the document that
/// holds it: a pick of every run of a large Monte Carlo is one member each.
pub const MAX_FAMILY_PREDICATE_DEPTH: usize = 32;

struct BoundedVec<T, const MAX: usize>(Vec<T>);

impl<T, const MAX: usize> BoundedVec<T, MAX> {
    fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<'de, T, const MAX: usize> Deserialize<'de> for BoundedVec<T, MAX>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BoundedVecVisitor<T, const MAX: usize>(PhantomData<T>);

        impl<'de, T, const MAX: usize> Visitor<'de> for BoundedVecVisitor<T, MAX>
        where
            T: Deserialize<'de>,
        {
            type Value = BoundedVec<T, MAX>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a sequence containing at most {MAX} entries")
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                if sequence.size_hint().is_some_and(|hint| hint > MAX) {
                    return Err(serde::de::Error::custom(format!(
                        "sequence exceeds the {MAX}-entry resource limit"
                    )));
                }
                let mut values = Vec::with_capacity(sequence.size_hint().unwrap_or(0).min(MAX));
                while values.len() < MAX {
                    match sequence.next_element()? {
                        Some(value) => values.push(value),
                        None => return Ok(BoundedVec(values)),
                    }
                }
                if sequence.next_element::<IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::custom(format!(
                        "sequence exceeds the {MAX}-entry resource limit"
                    )));
                }
                Ok(BoundedVec(values))
            }
        }

        deserializer.deserialize_seq(BoundedVecVisitor::<T, MAX>(PhantomData))
    }
}

struct BoundedString<const MAX: usize>(String);

impl<'de, const MAX: usize> Deserialize<'de> for BoundedString<MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BoundedStringVisitor<const MAX: usize>;

        impl<const MAX: usize> Visitor<'_> for BoundedStringVisitor<MAX> {
            type Value = BoundedString<MAX>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "a UTF-8 string containing at most {MAX} bytes")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value.len() > MAX {
                    return Err(E::custom(format!(
                        "string exceeds the {MAX}-byte resource limit"
                    )));
                }
                Ok(BoundedString(value.to_owned()))
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value.len() > MAX {
                    return Err(E::custom(format!(
                        "string exceeds the {MAX}-byte resource limit"
                    )));
                }
                Ok(BoundedString(value))
            }
        }

        deserializer.deserialize_string(BoundedStringVisitor::<MAX>)
    }
}

fn deserialize_key_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoundedString::<MAX_VISUALIZATION_KEY_BYTES>::deserialize(deserializer).map(|value| value.0)
}

fn deserialize_shared_key<'de, D>(deserializer: D) -> Result<Arc<str>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_key_string(deserializer).map(Arc::from)
}

fn deserialize_label_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoundedString::<MAX_VISUALIZATION_LABEL_BYTES>::deserialize(deserializer).map(|value| value.0)
}

fn deserialize_optional_label_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<BoundedString<MAX_VISUALIZATION_LABEL_BYTES>>::deserialize(deserializer)
        .map(|value| value.map(|value| value.0))
}

fn deserialize_unit_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<BoundedString<MAX_VISUALIZATION_UNIT_BYTES>>::deserialize(deserializer)
        .map(|value| value.map(|value| value.0))
}

fn deserialize_annotation_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoundedString::<MAX_ANNOTATION_TEXT_BYTES>::deserialize(deserializer).map(|value| value.0)
}

fn deserialize_source_text_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoundedString::<MAX_SOURCE_TEXT_BYTES>::deserialize(deserializer).map(|value| value.0)
}

fn deserialize_label_prefix<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<BoundedString<64>>::deserialize(deserializer).map(|value| value.map(|value| value.0))
}

#[derive(Deserialize)]
#[serde(transparent)]
struct BoundedKey(#[serde(deserialize_with = "deserialize_key_string")] String);

fn deserialize_comparison_signal_keys<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Vec::<BoundedKey>::deserialize(deserializer)
        .map(|keys| keys.into_iter().map(|key| key.0).collect())
}

#[derive(Default)]
struct Sha256Writer(Sha256);

/// The content digest of the content one generation names, kept so a reader
/// that asks again of content that has not changed reads it rather than
/// writing the whole document out once more.
#[derive(Default)]
struct DigestMemo(std::sync::Mutex<Option<(u64, ContentDigest)>>);

impl DigestMemo {
    /// The digest kept for `generation`, if it is the one kept.
    fn get(&self, generation: u64) -> Option<ContentDigest> {
        let kept = *self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        kept.and_then(|(kept, digest)| (kept == generation).then_some(digest))
    }

    fn keep(&self, generation: u64, digest: ContentDigest) {
        *self
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some((generation, digest));
    }
}

impl Clone for DigestMemo {
    /// A copy keeps its generation, so it keeps the digest too.
    fn clone(&self) -> Self {
        Self(std::sync::Mutex::new(
            *self
                .0
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        ))
    }
}

impl PartialEq for DigestMemo {
    /// What a document has worked out about itself is not part of it.
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Debug for DigestMemo {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("DigestMemo").finish_non_exhaustive()
    }
}

impl Sha256Writer {
    fn finish(self) -> ContentDigest {
        ContentDigest::from_bytes(self.0.finalize().into())
    }
}

impl Write for Sha256Writer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.update(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

macro_rules! stable_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(NonZeroU64);

        impl $name {
            fn allocate(serial: u64) -> Result<Self, VisualizationError> {
                NonZeroU64::new(serial)
                    .map(Self)
                    .ok_or(VisualizationError::IdentitySpaceExhausted)
            }

            #[must_use]
            pub const fn get(self) -> u64 {
                self.0.get()
            }
        }
    };
}

stable_id!(PageId);
stable_id!(PaneId);
stable_id!(TraceId);
stable_id!(AxisId);
stable_id!(CursorId);
stable_id!(MarkerId);
stable_id!(MeasurementId);
stable_id!(AnnotationId);
stable_id!(LinkGroupId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "id")]
pub enum EntityRef {
    Page(PageId),
    Pane(PaneId),
    Trace(TraceId),
    Axis(AxisId),
    Cursor(CursorId),
    Marker(MarkerId),
    Measurement(MeasurementId),
    Annotation(AnnotationId),
    LinkGroup(LinkGroupId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ValueType {
    Real,
    Integer,
    Boolean,
    Text,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "value")]
pub enum TypedValue {
    /// An explicitly unavailable signal sample of the declared column type.
    Missing(ValueType),
    Real(f64),
    Integer(i64),
    Boolean(bool),
    Text(String),
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "value")]
enum TypedValueWire {
    Missing(ValueType),
    Real(f64),
    Integer(i64),
    Boolean(bool),
    Text(#[serde(deserialize_with = "deserialize_source_text_string")] String),
}

impl<'de> Deserialize<'de> for TypedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match TypedValueWire::deserialize(deserializer)? {
            TypedValueWire::Missing(kind) => Self::Missing(kind),
            TypedValueWire::Real(value) => Self::Real(value),
            TypedValueWire::Integer(value) => Self::Integer(value),
            TypedValueWire::Boolean(value) => Self::Boolean(value),
            TypedValueWire::Text(value) => Self::Text(value),
        })
    }
}

impl TypedValue {
    /// Preserve numerical gaps at the typed table boundary. Infinities remain
    /// invalid real values and are rejected by source validation.
    #[cfg(test)]
    pub fn real_sample(value: f64) -> Self {
        if value.is_nan() {
            Self::Missing(ValueType::Real)
        } else {
            Self::Real(value)
        }
    }

    pub const fn is_missing(&self) -> bool {
        matches!(self, Self::Missing(_))
    }

    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        match self {
            Self::Missing(kind) => *kind,
            Self::Real(_) => ValueType::Real,
            Self::Integer(_) => ValueType::Integer,
            Self::Boolean(_) => ValueType::Boolean,
            Self::Text(_) => ValueType::Text,
        }
    }

    fn validate(&self, field: &'static str) -> Result<(), VisualizationError> {
        match self {
            Self::Missing(_) => Err(VisualizationError::InvalidValue {
                field,
                message: "only source signal samples may be unavailable".into(),
            }),
            Self::Real(value) if !value.is_finite() => Err(VisualizationError::InvalidValue {
                field,
                message: "real values must be finite".to_owned(),
            }),
            Self::Text(value) if value.is_empty() || value.len() > MAX_SOURCE_TEXT_BYTES => {
                Err(VisualizationError::InvalidValue {
                    field,
                    message: format!(
                        "text values must contain 1 to {MAX_SOURCE_TEXT_BYTES} UTF-8 bytes"
                    ),
                })
            }
            _ => Ok(()),
        }
    }

    pub(crate) fn exact_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Missing(left), Self::Missing(right)) => left == right,
            (Self::Real(left), Self::Real(right)) => left.to_bits() == right.to_bits(),
            (Self::Integer(left), Self::Integer(right)) => left == right,
            (Self::Boolean(left), Self::Boolean(right)) => left == right,
            (Self::Text(left), Self::Text(right)) => left == right,
            _ => false,
        }
    }
}

/// Borrowed, hashable identity for one coordinate value.
///
/// `f64` coordinates deliberately use their IEEE-754 bit pattern so this
/// remains identical to [`TypedValue::exact_eq`]: positive and negative zero
/// are distinct exact coordinates, and no tolerance or normalization is
/// introduced while validating immutable source rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ExactCoordinateValue<'a> {
    Missing(ValueType),
    Real(u64),
    Integer(i64),
    Boolean(bool),
    Text(&'a str),
}

impl<'a> From<&'a TypedValue> for ExactCoordinateValue<'a> {
    fn from(value: &'a TypedValue) -> Self {
        match value {
            TypedValue::Missing(kind) => Self::Missing(*kind),
            TypedValue::Real(value) => Self::Real(value.to_bits()),
            TypedValue::Integer(value) => Self::Integer(*value),
            TypedValue::Boolean(value) => Self::Boolean(*value),
            TypedValue::Text(value) => Self::Text(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColumnRole {
    Coordinate,
    Signal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceColumn {
    #[serde(deserialize_with = "deserialize_key_string")]
    key: String,
    #[serde(deserialize_with = "deserialize_label_string")]
    label: String,
    value_type: ValueType,
    role: ColumnRole,
    #[serde(deserialize_with = "deserialize_unit_string")]
    unit: Option<String>,
}

impl SourceColumn {
    pub fn new(
        key: impl Into<String>,
        label: impl Into<String>,
        value_type: ValueType,
        role: ColumnRole,
        unit: Option<String>,
    ) -> Result<Self, VisualizationError> {
        let column = Self {
            key: key.into(),
            label: label.into(),
            value_type,
            role,
            unit,
        };
        column.validate()?;
        Ok(column)
    }

    fn validate(&self) -> Result<(), VisualizationError> {
        validate_key("source-column.key", &self.key)?;
        validate_label("source-column.label", &self.label)?;
        validate_optional_unit("source-column.unit", self.unit.as_deref())
    }

    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        self.value_type
    }

    #[must_use]
    pub const fn role(&self) -> ColumnRole {
        self.role
    }

    /// The engineering unit the column's values carry, when it has one.
    #[must_use]
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SourceRow {
    values: Vec<TypedValue>,
}

#[derive(Deserialize)]
struct SourceRowWire {
    values: BoundedVec<TypedValue, MAX_SOURCE_CELLS_PER_DATASET>,
}

impl<'de> Deserialize<'de> for SourceRow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SourceRowWire::deserialize(deserializer)?;
        Ok(Self {
            values: wire.values.into_inner(),
        })
    }
}

impl SourceRow {
    #[must_use]
    pub fn new(values: Vec<TypedValue>) -> Self {
        Self { values }
    }

    #[must_use]
    pub fn values(&self) -> &[TypedValue] {
        &self.values
    }

    fn retained_text_bytes(&self) -> Result<usize, VisualizationError> {
        self.values.iter().try_fold(0_usize, |total, value| {
            let bytes = match value {
                TypedValue::Text(value) => value.len(),
                TypedValue::Missing(_)
                | TypedValue::Real(_)
                | TypedValue::Integer(_)
                | TypedValue::Boolean(_) => 0,
            };
            total
                .checked_add(bytes)
                .ok_or_else(|| VisualizationError::InvalidValue {
                    field: "source-dataset.retained-text-bytes",
                    message: "the source text is too large to count".to_owned(),
                })
        })
    }
}

struct BoundedSourceRows(Vec<SourceRow>);

impl<'de> Deserialize<'de> for BoundedSourceRows {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SourceRowsVisitor;

        impl<'de> Visitor<'de> for SourceRowsVisitor {
            type Value = BoundedSourceRows;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "at most {MAX_SOURCE_CELLS_PER_DATASET} cells")
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                // Nothing is reserved for rows the file only claims: each row
                // is counted as it is read, an empty one as a cell, since it
                // is held all the same.
                let mut rows = Vec::new();
                let mut cells = 0_usize;
                let mut text_bytes = 0_usize;
                loop {
                    let Some(row) = sequence.next_element::<SourceRow>()? else {
                        return Ok(BoundedSourceRows(rows));
                    };
                    cells = cells.checked_add(row.values.len().max(1)).ok_or_else(|| {
                        <A::Error as serde::de::Error>::custom("source cell count overflowed")
                    })?;
                    if cells > MAX_SOURCE_CELLS_PER_DATASET {
                        return Err(serde::de::Error::custom(format!(
                            "source rows exceed the {MAX_SOURCE_CELLS_PER_DATASET}-cell resource limit"
                        )));
                    }
                    let row_text_bytes = row.retained_text_bytes().map_err(|error| {
                        <A::Error as serde::de::Error>::custom(error.to_string())
                    })?;
                    text_bytes = checked_bounded_sum(
                        "source-dataset.retained-text-bytes",
                        text_bytes,
                        row_text_bytes,
                        MAX_SOURCE_TEXT_BYTES_PER_DATASET,
                    )
                    .map_err(|error| <A::Error as serde::de::Error>::custom(error.to_string()))?;
                    rows.push(row);
                }
            }
        }

        deserializer.deserialize_seq(SourceRowsVisitor)
    }
}

/// One immutable result dataset a document reads.
///
/// A run reference names a retained run and reads its samples from the run
/// store (see [`RunSource`]). An embedded table holds its typed rows in the
/// document itself.
#[derive(Debug, Clone, PartialEq)]
pub struct SourceDataset {
    binding: DatasetBinding,
    columns: Vec<SourceColumn>,
    rows: Vec<SourceRow>,
    run: Option<RunSource>,
}

impl Serialize for SourceDataset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // A run reference's columns are fixed, so only its catalog is written.
        if let Some(run) = &self.run {
            let mut dataset = serializer.serialize_struct("SourceDataset", 2)?;
            dataset.serialize_field("binding", &self.binding)?;
            dataset.serialize_field("run", run)?;
            return dataset.end();
        }
        let mut dataset = serializer.serialize_struct("SourceDataset", 3)?;
        dataset.serialize_field("binding", &self.binding)?;
        dataset.serialize_field("columns", &self.columns)?;
        dataset.serialize_field("rows", &self.rows)?;
        dataset.end()
    }
}

#[derive(Deserialize)]
struct SourceDatasetWire {
    binding: DatasetBinding,
    #[serde(default)]
    columns: Option<Vec<SourceColumn>>,
    #[serde(default)]
    rows: Option<BoundedSourceRows>,
    #[serde(default)]
    run: Option<RunSource>,
}

impl<'de> Deserialize<'de> for SourceDataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SourceDatasetWire::deserialize(deserializer)?;
        let dataset = match (wire.columns, wire.rows, wire.run) {
            (Some(columns), Some(rows), None) => Self {
                binding: wire.binding,
                columns,
                rows: rows.0,
                run: None,
            },
            (None, None, Some(run)) => Self {
                binding: wire.binding,
                columns: run_source_columns(),
                rows: Vec::new(),
                run: Some(run),
            },
            _ => {
                return Err(serde::de::Error::custom(
                    "a dataset is either a run reference or a table of typed columns and rows",
                ));
            }
        };
        dataset.validate().map_err(serde::de::Error::custom)?;
        Ok(dataset)
    }
}

impl SourceDataset {
    pub fn new(
        binding: DatasetBinding,
        columns: Vec<SourceColumn>,
        rows: Vec<SourceRow>,
    ) -> Result<Self, VisualizationError> {
        let dataset = Self {
            binding,
            columns,
            rows,
            run: None,
        };
        dataset.validate()?;
        Ok(dataset)
    }

    #[must_use]
    pub const fn binding(&self) -> DatasetBinding {
        self.binding
    }

    #[must_use]
    pub fn columns(&self) -> &[SourceColumn] {
        &self.columns
    }

    #[must_use]
    pub fn rows(&self) -> &[SourceRow] {
        &self.rows
    }

    fn validate(&self) -> Result<(), VisualizationError> {
        if let Some(run) = &self.run {
            if self.columns != run_source_columns() || !self.rows.is_empty() {
                return Err(VisualizationError::InvalidValue {
                    field: "source-dataset.run",
                    message: "a run reference holds no rows of its own".to_owned(),
                });
            }
            return run.validate();
        }
        if self.columns.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "source-dataset.columns",
                message: "a dataset requires at least one typed column".to_owned(),
            });
        }
        let cell_count = self
            .rows
            .len()
            .checked_mul(self.columns.len())
            .ok_or_else(|| VisualizationError::InvalidValue {
                field: "source-dataset.cells",
                message: "source cell count overflowed the supported address space".to_owned(),
            })?;
        ensure_maximum_len(
            "source-dataset.cells",
            cell_count,
            MAX_SOURCE_CELLS_PER_DATASET,
        )?;
        let retained_text_bytes = self.retained_text_bytes()?;
        ensure_maximum_len(
            "source-dataset.retained-text-bytes",
            retained_text_bytes,
            MAX_SOURCE_TEXT_BYTES_PER_DATASET,
        )?;
        let mut keys = HashSet::with_capacity(self.columns.len());
        let mut coordinates = 0;
        let mut signals = 0;
        for column in &self.columns {
            column.validate()?;
            if !keys.insert(column.key.as_str()) {
                return Err(VisualizationError::DuplicateKey(column.key.clone()));
            }
            match column.role {
                ColumnRole::Coordinate => coordinates += 1,
                ColumnRole::Signal => signals += 1,
            }
        }
        if coordinates == 0 || signals == 0 {
            return Err(VisualizationError::InvalidValue {
                field: "source-dataset.columns",
                message: "at least one coordinate and one signal column are required".to_owned(),
            });
        }
        let coordinate_indices: Vec<_> = self
            .columns
            .iter()
            .enumerate()
            .filter_map(|(index, column)| (column.role == ColumnRole::Coordinate).then_some(index))
            .collect();
        for (row_index, row) in self.rows.iter().enumerate() {
            if row.values.len() != self.columns.len() {
                return Err(VisualizationError::RowWidth {
                    row: row_index,
                    expected: self.columns.len(),
                    actual: row.values.len(),
                });
            }
            for (column, value) in self.columns.iter().zip(&row.values) {
                if !value.is_missing() {
                    value.validate("source-row.value")?;
                }
                if column.role == ColumnRole::Coordinate && value.is_missing() {
                    return Err(VisualizationError::InvalidValue {
                        field: "source-row.coordinate",
                        message: "coordinate values cannot be unavailable".into(),
                    });
                }
                if value.value_type() != column.value_type {
                    return Err(VisualizationError::ColumnTypeMismatch {
                        column: column.key.clone(),
                        expected: column.value_type,
                        actual: value.value_type(),
                    });
                }
            }
        }
        let mut coordinate_rows = HashSet::with_capacity(self.rows.len());
        for (row_index, row) in self.rows.iter().enumerate() {
            let coordinate = coordinate_indices
                .iter()
                .map(|index| ExactCoordinateValue::from(&row.values[*index]))
                .collect::<Vec<_>>();
            if !coordinate_rows.insert(coordinate) {
                return Err(VisualizationError::DuplicateCoordinateRow(row_index));
            }
        }
        Ok(())
    }

    fn retained_text_bytes(&self) -> Result<usize, VisualizationError> {
        self.rows.iter().try_fold(0_usize, |total, row| {
            checked_bounded_sum(
                "source-dataset.retained-text-bytes",
                total,
                row.retained_text_bytes()?,
                MAX_SOURCE_TEXT_BYTES_PER_DATASET,
            )
        })
    }
}

struct BoundedSourceDatasets(Vec<SourceDataset>);

impl<'de> Deserialize<'de> for BoundedSourceDatasets {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SourceDatasetsVisitor;

        impl<'de> Visitor<'de> for SourceDatasetsVisitor {
            type Value = BoundedSourceDatasets;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "datasets of at most {MAX_SOURCE_CELLS_TOTAL} embedded cells"
                )
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut datasets = Vec::new();
                let mut cells = 0_usize;
                let mut text_bytes = 0_usize;
                loop {
                    let Some(dataset) = sequence.next_element::<SourceDataset>()? else {
                        return Ok(BoundedSourceDatasets(datasets));
                    };
                    let dataset_cells = dataset
                        .rows
                        .len()
                        .checked_mul(dataset.columns.len())
                        .ok_or_else(|| {
                            <A::Error as serde::de::Error>::custom("source cell count overflowed")
                        })?;
                    cells = cells.checked_add(dataset_cells).ok_or_else(|| {
                        <A::Error as serde::de::Error>::custom(
                            "aggregate source cell count overflowed",
                        )
                    })?;
                    if cells > MAX_SOURCE_CELLS_TOTAL {
                        return Err(serde::de::Error::custom(format!(
                            "datasets exceed the {MAX_SOURCE_CELLS_TOTAL}-cell resource limit"
                        )));
                    }
                    let dataset_text_bytes = dataset.retained_text_bytes().map_err(|error| {
                        <A::Error as serde::de::Error>::custom(error.to_string())
                    })?;
                    text_bytes = checked_bounded_sum(
                        "visualization-document.retained-source-text-bytes",
                        text_bytes,
                        dataset_text_bytes,
                        MAX_SOURCE_TEXT_BYTES_TOTAL,
                    )
                    .map_err(|error| <A::Error as serde::de::Error>::custom(error.to_string()))?;
                    datasets.push(dataset);
                }
            }
        }

        deserializer.deserialize_seq(SourceDatasetsVisitor)
    }
}

// A document lists each kind of entity in the order its entities were made,
// by ascending identity (its own check holds it to that), so an entity is
// found by identity in a binary search: an edit that touches one entity of a
// view of a hundred thousand reads a few of them, not all.
impl VisualizationDocument {
    fn require_page(&self, id: PageId) -> Result<(), VisualizationError> {
        self.pages
            .position(id.get())
            .map(|_| ())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Page(id)))
    }

    fn require_pane(&self, id: PaneId) -> Result<(), VisualizationError> {
        self.panes
            .position(id.get())
            .map(|_| ())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(id)))
    }

    fn require_axis_in_pane(&self, id: AxisId, pane_id: PaneId) -> Result<(), VisualizationError> {
        self.axes
            .position(id.get())
            .filter(|&index| self.axes[index].pane_id == pane_id)
            .map(|_| ())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Axis(id)))
    }

    fn require_trace_in_pane(
        &self,
        id: TraceId,
        pane_id: PaneId,
    ) -> Result<(), VisualizationError> {
        self.traces
            .position(id.get())
            .filter(|&index| self.traces[index].pane_id == pane_id)
            .map(|_| ())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Trace(id)))
    }

    fn require_entity(&self, entity: EntityRef) -> Result<(), VisualizationError> {
        let exists = match entity {
            EntityRef::Page(id) => self.pages.position(id.get()).is_some(),
            EntityRef::Pane(id) => self.panes.position(id.get()).is_some(),
            EntityRef::Trace(id) => self.traces.position(id.get()).is_some(),
            EntityRef::Axis(id) => self.axes.position(id.get()).is_some(),
            EntityRef::Cursor(id) => self.cursors.position(id.get()).is_some(),
            EntityRef::Marker(id) => self.markers.position(id.get()).is_some(),
            EntityRef::Measurement(id) => self.measurements.position(id.get()).is_some(),
            EntityRef::Annotation(id) => self.annotations.position(id.get()).is_some(),
            EntityRef::LinkGroup(id) => self.link_groups.position(id.get()).is_some(),
        };
        exists
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(entity))
    }

    fn page_mut(&mut self, id: PageId) -> Result<&mut Page, VisualizationError> {
        self.pages
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Page(id)))
    }

    fn pane_mut(&mut self, id: PaneId) -> Result<&mut Pane, VisualizationError> {
        self.panes
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(id)))
    }

    /// Change the trace `id` with `change`.
    fn update_trace<R>(
        &mut self,
        id: TraceId,
        change: impl FnOnce(&mut Trace) -> R,
    ) -> Result<R, VisualizationError> {
        self.traces
            .update(id.get(), change)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Trace(id)))
    }

    fn axis_mut(&mut self, id: AxisId) -> Result<&mut Axis, VisualizationError> {
        self.axes
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Axis(id)))
    }

    fn cursor_mut(&mut self, id: CursorId) -> Result<&mut Cursor, VisualizationError> {
        self.cursors
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Cursor(id)))
    }

    fn marker_mut(&mut self, id: MarkerId) -> Result<&mut Marker, VisualizationError> {
        self.markers
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Marker(id)))
    }

    fn measurement_mut(
        &mut self,
        id: MeasurementId,
    ) -> Result<&mut Measurement, VisualizationError> {
        self.measurements
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Measurement(
                id,
            )))
    }

    fn annotation_mut(&mut self, id: AnnotationId) -> Result<&mut Annotation, VisualizationError> {
        self.annotations
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Annotation(
                id,
            )))
    }

    fn link_group_mut(&mut self, id: LinkGroupId) -> Result<&mut LinkGroup, VisualizationError> {
        self.link_groups
            .get_mut(id.get())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::LinkGroup(id)))
    }
}

impl NumericTolerance {
    fn validate(self) -> Result<(), VisualizationError> {
        Self::new(self.absolute, self.relative).map(|_| ())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PaneKind {
    Cartesian,
    Smith,
    Polar,
    Histogram,
    Table,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum PageLayout {
    SinglePane,
    #[default]
    Rows,
    Columns,
    Grid {
        columns: u8,
    },
}

impl PageLayout {
    fn validate(self) -> Result<(), VisualizationError> {
        if let Self::Grid { columns } = self
            && !(2..=16).contains(&columns)
        {
            return Err(VisualizationError::InvalidValue {
                field: "page.layout.columns",
                message: "grid columns must be between 2 and 16".to_owned(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PageUpdatePolicy {
    #[default]
    RefreshLinkedFigures,
    FreezeFigureRevision,
}

fn default_page_template_id() -> String {
    "engineering-dark".to_owned()
}

const fn default_viewer_id_for_kind(kind: PaneKind) -> &'static str {
    match kind {
        PaneKind::Cartesian => "viewer-waveform",
        PaneKind::Smith => "viewer-smith",
        PaneKind::Polar => "viewer-polar",
        PaneKind::Histogram => "viewer-histogram",
        PaneKind::Table => "viewer-table",
    }
}

const fn pane_kind_for_viewer_art(art: ViewerArt) -> PaneKind {
    match art {
        ViewerArt::Smith => PaneKind::Smith,
        ViewerArt::Polar => PaneKind::Polar,
        ViewerArt::Histogram => PaneKind::Histogram,
        ViewerArt::Table => PaneKind::Table,
        ViewerArt::Wave
        | ViewerArt::Bode
        | ViewerArt::Spectrum
        | ViewerArt::Phase
        | ViewerArt::Field
        | ViewerArt::Contour
        | ViewerArt::Wireless
        | ViewerArt::Scatter
        | ViewerArt::Eye
        | ViewerArt::Bathtub
        | ViewerArt::Margin
        | ViewerArt::DigitalEvents
        | ViewerArt::Soa
        | ViewerArt::Optimization
        | ViewerArt::PoleZero
        | ViewerArt::Thermal
        | ViewerArt::Mesh => PaneKind::Cartesian,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneDataBinding {
    pub analysis_id: AnalysisInstanceId,
    pub dataset: DatasetBinding,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum PanePlacement {
    #[default]
    Primary,
    Below {
        anchor_pane_id: PaneId,
    },
    RightOf {
        anchor_pane_id: PaneId,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AxisOrientation {
    Horizontal,
    VerticalLeft,
    VerticalRight,
    Radial,
    Angular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AxisScale {
    Linear,
    Logarithmic,
    Decibels,
    PhaseDegrees,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AxisRange {
    pub minimum: f64,
    pub maximum: f64,
}

impl AxisRange {
    pub fn new(minimum: f64, maximum: f64) -> Result<Self, VisualizationError> {
        if !minimum.is_finite() || !maximum.is_finite() || minimum >= maximum {
            return Err(VisualizationError::InvalidValue {
                field: "axis.range",
                message: "range endpoints must be finite and minimum must be less than maximum"
                    .to_owned(),
            });
        }
        Ok(Self { minimum, maximum })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub id: PageId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub layout: PageLayout,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub template_id: String,
    pub update_policy: PageUpdatePolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pane {
    pub id: PaneId,
    pub page_id: PageId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub kind: PaneKind,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub viewer_id: String,
    #[serde(default)]
    pub binding: Option<PaneDataBinding>,
    pub placement: PanePlacement,
    pub order: u32,
    #[serde(default)]
    pub family_policy: Option<FamilyPresentationPolicy>,
    /// The reader closed the pane's strip. It keeps everything it plots and
    /// comes back as it was.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
    /// The strips the reader arranged the pane's waveform stack into, top
    /// to bottom. Empty, the stack gives each unit a strip of its own.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub strips: Vec<Strip>,
    /// The reader maximized the pane's strip over the others.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub maximized: bool,
    /// The data browser last showed the pane's analysis, so a run of the
    /// plan opens on it. One pane of a view at most.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub shown: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Axis {
    pub id: AxisId,
    pub pane_id: PaneId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    pub orientation: AxisOrientation,
    pub scale: AxisScale,
    #[serde(deserialize_with = "deserialize_unit_string")]
    pub unit: Option<String>,
    pub range: Option<AxisRange>,
}

/// Long-form column naming the result vector a trace was provisioned from.
///
/// It is written as a row predicate when the trace is created and read back to
/// tell a derived label from one the reader typed, so both ends name it here.
pub const TRACE_NAME_COLUMN: &str = "trace-name";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trace {
    pub id: TraceId,
    pub pane_id: PaneId,
    pub binding: DatasetBinding,
    /// The source columns and rows a trace reads are shared by its copies:
    /// a step that keeps a trace as it stood copies none of them.
    #[serde(deserialize_with = "deserialize_shared_key")]
    pub signal_key: Arc<str>,
    #[serde(deserialize_with = "deserialize_shared_key")]
    pub coordinate_key: Arc<str>,
    /// Exact coordinate-column predicates selecting this trace's rows from a
    /// lossless long-form source dataset.
    pub row_predicates: Arc<[QueryCoordinate]>,
    pub x_axis_id: AxisId,
    pub y_axis_id: AxisId,
    /// What the reader sees, and the one field a rename writes.
    ///
    /// A trace provisioned from a dataset starts with the label derivation
    /// gave it — the result vector's own name — so the stored string is at
    /// that moment the engine's spelling of the signal. That is a value a
    /// display surface may re-spell and never a value it may rewrite: the
    /// engine name is the trace's identity in the dataset, a rename must
    /// survive verbatim, and this field is persisted, so writing a display
    /// spelling here would both lose the identity and edit a saved document
    /// that nobody asked to change.
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    pub visible: bool,
    /// The colour the reader gave this trace. Absent, it draws in the colour
    /// its run gives the signal, or its place in the palette: the run keeps
    /// no colour a reader chose.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<TraceColor>,
    /// How the waveform stack draws this trace. Absent for an interpolated
    /// value trace.
    #[serde(default, skip_serializing_if = "TraceLane::is_value")]
    pub lane: TraceLane,
    /// The width, dash, symbol and joining the reader gave this trace.
    /// Absent, it draws as its place in the palette draws it.
    #[serde(default, skip_serializing_if = "TraceStyle::is_default")]
    pub style: TraceStyle,
    /// The run the trace reads has no signal by its name. The trace keeps
    /// its place and draws nothing, and reads the signal again from the
    /// first later run that has it.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub missing: bool,
}

/// What a trace's lane draws: an interpolated value, or one of the exact
/// event forms a digital or real event trace takes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TraceLaneKind {
    /// An interpolated value trace in a unit pane: `V()`, `I()`, expressions.
    #[default]
    Value,
    /// A scalar digital event trace, `D()`: levels and edges at exact instants.
    Logic,
    /// A declared bus, `D(bus)`: one hexagonal segment per stable word.
    Bus,
    /// A real event trace, `E()`: a step plot in a value pane.
    Real,
}

/// How a bus word is spelled.
///
/// Binary is the VCD spelling — one character per member, declared MSB first,
/// with `x` for unknown and `z` for high impedance — and is the only one that
/// can spell every word a run can produce. The others read the word's bits
/// as a number or as characters, which a word with an unknown or
/// high-impedance bit does not have.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BusRadix {
    #[default]
    Binary,
    Octal,
    Hex,
    Unsigned,
    Signed,
    /// Eight bits to a character, as a Verilog string holds them.
    Ascii,
}

impl BusRadix {
    /// Every radix, in the order a menu offers them.
    pub const ALL: [Self; 6] = [
        Self::Binary,
        Self::Octal,
        Self::Hex,
        Self::Unsigned,
        Self::Signed,
        Self::Ascii,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Octal => "octal",
            Self::Hex => "hexadecimal",
            Self::Unsigned => "unsigned decimal",
            Self::Signed => "signed decimal",
            Self::Ascii => "ASCII",
        }
    }

    /// The radix's short name, as a radix menu or tag prints it.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Binary => "BIN",
            Self::Octal => "OCT",
            Self::Hex => "HEX",
            Self::Unsigned => "DEC",
            Self::Signed => "±DEC",
            Self::Ascii => "ASCII",
        }
    }
}

/// A colour a reader gave one trace, kept by the view that draws it and
/// written `#rrggbb`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TraceColor([u8; 3]);

impl TraceColor {
    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self([red, green, blue])
    }

    /// The red, green and blue parts.
    pub const fn rgb(self) -> [u8; 3] {
        self.0
    }

    /// Read `#rrggbb` (the `#` may be left off), in either case.
    pub fn parse(text: &str) -> Option<Self> {
        let hex = text.strip_prefix('#').unwrap_or(text);
        if hex.len() != 6 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return None;
        }
        let part = |at: usize| u8::from_str_radix(&hex[at..at + 2], 16).ok();
        Some(Self([part(0)?, part(2)?, part(4)?]))
    }
}

impl fmt::Display for TraceColor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [red, green, blue] = self.0;
        write!(formatter, "#{red:02x}{green:02x}{blue:02x}")
    }
}

impl Serialize for TraceColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for TraceColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        Self::parse(&text).ok_or_else(|| {
            serde::de::Error::custom(format!("trace colour '{text}' is not written #rrggbb"))
        })
    }
}

/// One trace's lane presentation: its kind, the radix a bus spells its words
/// in, and whether a bus shows its member lanes under it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceLane {
    #[serde(default)]
    pub kind: TraceLaneKind,
    #[serde(default)]
    pub radix: BusRadix,
    #[serde(default)]
    pub expanded: bool,
}

impl TraceLane {
    /// The lane every trace had before lanes were recorded.
    pub fn is_value(&self) -> bool {
        *self == Self::default()
    }
}

/// Logic lanes the reader grouped into one word, drawn as a bus lane.
///
/// Members are logic traces of the bus's own pane, declared MSB first, or,
/// when the source keeps them only as event history, those nets by their
/// `D()` names; a bus groups one kind or the other. A user bus is
/// presentation only: the word at each instant is read from the members'
/// exact event histories, as a declared bus is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserBus {
    pub pane_id: PaneId,
    /// The bus's name, unique within its pane.
    #[serde(deserialize_with = "deserialize_label_string")]
    pub name: String,
    #[serde(default)]
    pub members: Vec<TraceId>,
    /// Members the source has no column for, by their `D()` names, MSB first.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub nets: Vec<String>,
    #[serde(default)]
    pub radix: BusRadix,
    #[serde(default)]
    pub expanded: bool,
    /// Names the reader gave the bus's words, by unsigned value, ascending.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub value_names: Vec<BusValueName>,
}

/// A name for one word of a bus, such as a state machine's state: the text
/// the bus shows for that value, as a SimVision mnemonic map gives it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BusValueName {
    /// The word as an unsigned integer, whatever radix the bus reads in.
    pub value: u64,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub name: String,
}

/// The name `names`, ascending by value, gives `value`.
pub fn bus_value_name(names: &[BusValueName], value: u64) -> Option<&str> {
    names
        .binary_search_by_key(&value, |name| name.value)
        .ok()
        .map(|index| names[index].name.as_str())
}

/// A calculator expression plotted on a pane.
///
/// It is kept as the text the reader typed and the datasets it reads, never
/// as samples: the view evaluates it against those datasets each time it is
/// drawn, so a saved view keeps the expression and a view re-pointed at
/// another run evaluates it there.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpressionTrace {
    pub pane_id: PaneId,
    /// The expression as typed, unique within its pane.
    #[serde(deserialize_with = "deserialize_source_text_string")]
    pub text: String,
    /// The attached datasets the expression reads.
    pub datasets: Vec<DatasetBinding>,
    /// The unit the reader declared, which replaces the one unit algebra
    /// infers from the expression.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_unit_string"
    )]
    pub unit: Option<String>,
    #[serde(default = "default_expression_visible")]
    pub visible: bool,
}

const fn default_expression_visible() -> bool {
    true
}

/// What one unit axis of a pane draws its curves against, in place of the
/// analysis' own sweep: a transfer curve, a hysteresis loop or a Lissajous
/// figure is one signal drawn against another.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Abscissa {
    /// A result vector of the pane's analysis, by its name.
    Signal {
        #[serde(deserialize_with = "deserialize_label_string")]
        name: String,
    },
    /// A calculator expression over the pane's analysis, as typed.
    Expression {
        #[serde(deserialize_with = "deserialize_source_text_string")]
        text: String,
    },
}

impl Abscissa {
    /// The signal name or the expression text, as the X axis is labelled.
    #[must_use]
    pub fn text(&self) -> &str {
        match self {
            Self::Signal { name } => name,
            Self::Expression { text } => text,
        }
    }
}

/// The longest unit-axis identity a pane abscissa names: a unit, or for an
/// expression whose unit cannot be stated, its text and a short suffix.
pub const MAX_UNIT_AXIS_BYTES: usize = MAX_SOURCE_TEXT_BYTES + 64;

/// The curves of one unit axis of a pane drawn against an [`Abscissa`], in
/// a plot of their own.
///
/// A transfer curve, a hysteresis loop or a Lissajous figure is one signal
/// drawn against another. The pane keeps drawing its axes against the
/// analysis' sweep; each of these is a further plot whose only horizontal
/// axis is the abscissa, as ViVA's YvsY and ADS's `vs()` make a new graph.
/// The unit names which of the pane's axes supplies the curves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XyPlot {
    pub pane_id: PaneId,
    /// The unit axis whose curves are drawn, by its unit; empty for a plain
    /// number.
    #[serde(deserialize_with = "deserialize_unit_axis_string")]
    pub unit: String,
    pub abscissa: Abscissa,
    /// The span of the analysis' sweep the curves are drawn over: one
    /// hysteresis cycle, or the settled part of a run. The whole record when
    /// absent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sweep_range: Option<AxisRange>,
    /// Whether the span follows the window of the sweep the pane shows, as
    /// a scope's XY display follows its zoom, in place of `sweep_range`.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub follow_sweep_view: bool,
    /// Whether the abscissa is read on a logarithmic axis.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub log_abscissa: bool,
}

impl XyPlot {
    /// Whether `self` and `other` are the same plot: one pane's axis drawn
    /// against one abscissa. A pane holds each plot once.
    #[must_use]
    pub fn same_plot(&self, other: &Self) -> bool {
        self.pane_id == other.pane_id && self.unit == other.unit && self.abscissa == other.abscissa
    }
}

fn deserialize_unit_axis_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoundedString::<MAX_UNIT_AXIS_BYTES>::deserialize(deserializer).map(|value| value.0)
}

fn deserialize_optional_unit_axis_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<BoundedString<MAX_UNIT_AXIS_BYTES>>::deserialize(deserializer)
        .map(|value| value.map(|value| value.0))
}

/// One row of a pane's event-lane stack.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum EventLaneRef {
    /// A logic trace of the pane.
    Trace { trace_id: TraceId },
    /// A bus by name: one of the pane's user buses, or a bus the run
    /// declared, which the document cannot name any other way because a
    /// declared bus is no column of its source.
    Bus {
        #[serde(deserialize_with = "deserialize_label_string")]
        name: String,
    },
    /// A net the source keeps only as event history, by the name its `D()`
    /// or `E()` column would have. The document has no trace to hold it by,
    /// so the net is on the pane while it is listed here.
    Net {
        #[serde(deserialize_with = "deserialize_label_string")]
        name: String,
    },
}

/// The order a pane draws its event lanes in, top to bottom.
///
/// Rows the reader added since the order was recorded join at the bottom; a
/// row that names something the pane no longer shows is skipped. So the
/// order is a presentation of the lanes, never what decides which exist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventLaneOrder {
    pub pane_id: PaneId,
    pub lanes: Vec<EventLaneRef>,
}

/// How the reader set a pane's spectrum sheet: the signal, the record, the
/// window and what its figures are measured over. A later run the view
/// follows is transformed the same way.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaneSpectrumSettings {
    pub pane_id: PaneId,
    pub settings: crate::fft::settings::SpectrumSettings,
}

/// How the reader set a pane's coefficient spectrum sheet (harmonic
/// balance, `.FOUR`, PSS or QPSS): the level its lines are drawn at, and
/// whether their phase and their waveform are drawn.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaneCoefficientSettings {
    pub pane_id: PaneId,
    pub settings: crate::harmonic_spectrum::settings::CoefficientSettings,
}

/// The X one sheet's readout is pinned at on a pane: the reader clicked the
/// plot there. It is a place on the X axis, not a sample, so a later run
/// the view follows reads its own values at the same X.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ReadoutPin {
    pub pane_id: PaneId,
    pub sheet: crate::result_presentation::ResultViewer,
    pub x: f64,
}

/// What the reader left one sheet showing on a pane, for a sheet that draws
/// its pane's analysis its own way rather than through the pane's axes and
/// traces. Kept by the view, so a re-run and a reopen show it again.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SheetSetting {
    pub pane_id: PaneId,
    pub setting: SheetSettingValue,
}

/// One sheet's own settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SheetSettingValue {
    /// The Smith chart's markers, m1 first.
    Smith { markers: Vec<SmithMarker> },
    /// The network matrix's form and how each coefficient reads.
    NetworkMatrix {
        form: NetworkMatrixForm,
        format: NetworkMatrixFormat,
    },
    /// The response the Bode card measures, by the name the card gives it:
    /// a stability analysis's loop gain, or one AC response.
    Bode {
        #[serde(deserialize_with = "deserialize_label_string")]
        measured_signal: String,
    },
    /// Whether the pole-zero map is drawn on a logarithmic plane.
    PoleZero { logarithmic: bool },
    /// The offsets the phase-noise sheet reads L(f) at, in hertz, the band
    /// of offsets it integrates (the whole sweep when `None`), and whether
    /// it draws slope guides.
    PhaseNoise {
        spots: Vec<f64>,
        band: Option<[f64; 2]>,
        slopes: bool,
    },
    /// The noise contributors the reader pinned above the rest of the
    /// contributor table, first pinned first.
    NoiseContributors { pinned: Vec<PinnedContributor> },
    /// How the histogram reads one measurement of a population.
    Histogram(HistogramReading),
}

/// What the histogram sheet was left showing: the measurement by name, how
/// its distribution is drawn and binned, whether the normal fit is drawn,
/// and the specification limits the reader set where the run states none.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HistogramReading {
    #[serde(default, deserialize_with = "deserialize_optional_label_string")]
    pub measurement: Option<String>,
    pub plot: HistogramPlotKind,
    pub bins: u16,
    /// The binned range, lower then upper; `None` fits the samples.
    pub range: Option<[f64; 2]>,
    pub normal_fit: bool,
    pub lower_limit: Option<f64>,
    pub upper_limit: Option<f64>,
}

/// How the histogram sheet draws its distribution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HistogramPlotKind {
    Count,
    Density,
    Percent,
    Cumulative,
    /// The samples against their standard normal quantiles.
    NormalQuantile,
}

/// A noise contributor by the device and mechanism a run names it with, so
/// a later run of the plan pins its own row for the same source.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PinnedContributor {
    #[serde(deserialize_with = "deserialize_label_string")]
    pub device: String,
    /// Empty for a source a run ranks without naming its mechanism.
    #[serde(deserialize_with = "deserialize_label_string")]
    pub mechanism: String,
}

impl SheetSettingValue {
    /// The sheet the settings are for.
    pub const fn sheet(&self) -> crate::result_presentation::ResultViewer {
        match self {
            Self::Smith { .. } => crate::result_presentation::ResultViewer::Smith,
            Self::NetworkMatrix { .. } => crate::result_presentation::ResultViewer::NetworkMatrix,
            Self::Bode { .. } => crate::result_presentation::ResultViewer::Bode,
            Self::PoleZero { .. } => crate::result_presentation::ResultViewer::PoleZero,
            Self::Histogram(_) => crate::result_presentation::ResultViewer::Hist,
            Self::PhaseNoise { .. } => crate::result_presentation::ResultViewer::PhaseNoise,
            Self::NoiseContributors { .. } => {
                crate::result_presentation::ResultViewer::NoiseContrib
            }
        }
    }
}

/// A Smith marker: the locus it sits on, by name, and the frequency it sits
/// at. A frequency rather than a sample, so a later run the view follows
/// reads its own point at the same frequency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmithMarker {
    #[serde(deserialize_with = "deserialize_label_string")]
    pub trace: String,
    pub frequency: f64,
}

/// The parameters a network matrix holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkMatrixForm {
    S,
    Y,
    Z,
    H,
    Abcd,
}

/// How a network matrix's coefficient reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkMatrixFormat {
    /// The real and imaginary parts.
    RealImaginary,
    /// The magnitude and angle.
    MagnitudeAngle,
    /// The magnitude in decibels and the angle.
    DecibelAngle,
}

impl Trace {
    /// The result vector this trace was provisioned from, as its own row
    /// predicates record it.
    ///
    /// A long-form source pins each trace to its vector with a
    /// [`TRACE_NAME_COLUMN`] predicate, which is the trace's identity in the
    /// dataset and stays the engine's spelling for as long as the trace
    /// exists. The label may not: the reader can type over it. So retargeting
    /// matches on this rather than on what the trace is called, and a display
    /// surface re-spells a label only while the two are still equal — that
    /// equality is what proves the label is the one derivation wrote.
    pub fn source_signal(&self) -> Option<&str> {
        self.row_predicates.iter().find_map(|coordinate| {
            let TypedValue::Text(signal) = &coordinate.value else {
                return None;
            };
            (coordinate.column == TRACE_NAME_COLUMN).then_some(signal.as_str())
        })
    }

    /// The analysis the trace was provisioned from, when its exact predicates
    /// name one.
    pub fn source_analysis(&self) -> Option<AnalysisInstanceId> {
        self.row_predicates.iter().find_map(|coordinate| {
            let TypedValue::Text(analysis) = &coordinate.value else {
                return None;
            };
            (coordinate.column == ANALYSIS_ID_COLUMN)
                .then(|| analysis.parse().ok())
                .flatten()
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cursor {
    pub id: CursorId,
    pub pane_id: PaneId,
    pub axis_id: AxisId,
    pub position: TypedValue,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
}

/// Semantic role of a retained plot marker.
///
/// The role is independent from its coordinate so a specification boundary,
/// peak flag, and freeform point note never become indistinguishable after a
/// project round trip or hardcopy export.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlotMarkerKind {
    #[default]
    PointNote,
    /// The top of the hill the trace climbs to from the marker's X, at
    /// least `size` clear of the curve either side, found again whenever the
    /// curve changes.
    Peak {
        size: crate::result_presentation::FeatureSize,
    },
    SpecificationLine,
    MeasurementAnchor,
    /// The bottom of the basin the trace descends to from the marker's X,
    /// at least `size` deep, found again whenever the curve changes.
    Valley {
        size: crate::result_presentation::FeatureSize,
    },
    /// The crossing of `level` nearest the marker's X, found again whenever
    /// the curve changes.
    Crossing {
        level: crate::result_presentation::CrossingLevel,
    },
    /// A line at the marker's X, reading every trace it crosses.
    VerticalLine,
    /// The crossing of marker `from`'s level plus `by`, on its trace,
    /// nearest the marker's X, found again whenever either moves.
    Offset {
        from: u64,
        by: crate::result_presentation::CrossingLevel,
    },
}

impl PlotMarkerKind {
    pub const ALL: [Self; 8] = [
        Self::PointNote,
        Self::Peak {
            size: crate::result_presentation::FeatureSize(0.0),
        },
        Self::SpecificationLine,
        Self::MeasurementAnchor,
        Self::Valley {
            size: crate::result_presentation::FeatureSize(0.0),
        },
        Self::Crossing {
            level: crate::result_presentation::CrossingLevel(0.0),
        },
        Self::VerticalLine,
        Self::Offset {
            from: 0,
            by: crate::result_presentation::CrossingLevel(0.0),
        },
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PointNote => "Point note",
            Self::Peak { .. } => "Peak flag",
            Self::SpecificationLine => "Specification line",
            Self::MeasurementAnchor => "Measurement anchor",
            Self::Valley { .. } => "Valley flag",
            Self::Crossing { .. } => "Crossing flag",
            Self::VerticalLine => "Vertical line",
            Self::Offset { .. } => "Offset flag",
        }
    }
}

/// Visibility and publication scope of a retained plot marker.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlotMarkerScope {
    #[default]
    Document,
    Pane,
    DatasetFamily,
}

impl PlotMarkerScope {
    pub const ALL: [Self; 3] = [Self::Document, Self::Pane, Self::DatasetFamily];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Document => "Document",
            Self::Pane => "Pane",
            Self::DatasetFamily => "Dataset family",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub id: MarkerId,
    pub pane_id: PaneId,
    pub trace_id: TraceId,
    pub coordinate: TypedValue,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    #[serde(default)]
    pub kind: PlotMarkerKind,
    #[serde(default)]
    pub scope: PlotMarkerScope,
    /// Stable source specification name when this marker was imported from a
    /// project requirement. `None` denotes an authored plot annotation.
    #[serde(default, deserialize_with = "deserialize_optional_label_string")]
    pub source_specification: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MeasurementKind {
    Point,
    Difference,
    Minimum,
    Maximum,
    Mean,
    RootMeanSquare,
    Integral,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub id: MeasurementId,
    pub pane_id: PaneId,
    /// The traces a measurement of a kind reads. An authored one (with an
    /// `expression`) names its signals there instead and keeps none.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub trace_ids: Vec<TraceId>,
    pub kind: MeasurementKind,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    /// Authored scalar expression for a retained Studio measurement.
    #[serde(default, deserialize_with = "deserialize_optional_source_text_string")]
    pub expression: Option<String>,
    /// Over what part of the result an authored measurement is read. No
    /// value is kept: it is read from the run shown, through this window,
    /// whenever it is shown.
    #[serde(default)]
    pub window: MeasurementWindow,
    /// When the pane shows a family, the expression is read once per member
    /// and the measurement reports the statistic this names, its yield
    /// judged against these limits. Absent for one evaluation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family: Option<FamilyMeasure>,
    /// Where the reader put it in the measurements' order: before every
    /// measurement of a higher place. Measurements of one place stand in
    /// the order they were made.
    #[serde(default, skip_serializing_if = "is_first_place")]
    pub place: u64,
}

fn is_first_place(place: &u64) -> bool {
    *place == 0
}

/// Over what part of its result an authored measurement is read: the three
/// EZwave's measurement tool offers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MeasurementWindow {
    /// Every sample of the result.
    #[default]
    WholeRecord,
    /// The X range the pane shows, as it is zoomed.
    VisibleRange,
    /// From cursor A to cursor B on the pane.
    BetweenCursors,
}

impl MeasurementWindow {
    pub const ALL: [MeasurementWindow; 3] = [
        MeasurementWindow::WholeRecord,
        MeasurementWindow::VisibleRange,
        MeasurementWindow::BetweenCursors,
    ];

    /// The window as a menu and a table name it.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            MeasurementWindow::WholeRecord => "Whole record",
            MeasurementWindow::VisibleRange => "Visible range",
            MeasurementWindow::BetweenCursors => "Between cursors",
        }
    }
}

/// How a measurement over a family reports what its members read: the
/// statistic it shows, and the limits its yield is judged against.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMeasure {
    pub statistic: FamilyStatistic,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper: Option<f64>,
}

impl FamilyMeasure {
    fn validate(&self) -> Result<(), VisualizationError> {
        let invalid = |message: &str| VisualizationError::InvalidValue {
            field: "measurement.family",
            message: message.to_owned(),
        };
        if self
            .lower
            .into_iter()
            .chain(self.upper)
            .any(|limit| !limit.is_finite())
        {
            return Err(invalid("a family measurement's limits are finite numbers"));
        }
        if let (Some(lower), Some(upper)) = (self.lower, self.upper)
            && lower > upper
        {
            return Err(invalid(
                "a family measurement's lower limit is not above its upper",
            ));
        }
        if self.statistic == FamilyStatistic::Yield && self.lower.is_none() && self.upper.is_none()
        {
            return Err(invalid("a yield is judged against at least one limit"));
        }
        Ok(())
    }
}

/// Which statistic of a family measurement its value reports.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FamilyStatistic {
    #[default]
    Mean,
    /// The sample standard deviation, divided by n − 1.
    StdDev,
    Min,
    Max,
    /// The passing share of the members, in percent.
    Yield,
}

impl FamilyStatistic {
    pub const ALL: [FamilyStatistic; 5] = [
        FamilyStatistic::Mean,
        FamilyStatistic::StdDev,
        FamilyStatistic::Min,
        FamilyStatistic::Max,
        FamilyStatistic::Yield,
    ];

    /// The statistic as a measurement states it.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            FamilyStatistic::Mean => "mean",
            FamilyStatistic::StdDev => "\u{3c3}",
            FamilyStatistic::Min => "min",
            FamilyStatistic::Max => "max",
            FamilyStatistic::Yield => "yield",
        }
    }
}

/// What a family measurement reads on the family shown: the distribution
/// its members' values form. It is read, never kept.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyMeasurementSummary {
    pub statistic: FamilyStatistic,
    /// Every member the expression was evaluated on.
    pub members: u32,
    /// Members with a finite value; the rest could not be measured.
    pub measured: u32,
    pub mean: f64,
    /// Absent below two measured members, where it is not defined.
    pub std_dev: Option<f64>,
    pub min: f64,
    pub max: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper: Option<f64>,
    /// Members inside the limits, present exactly when a limit is.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passed: Option<u32>,
    /// The failing members by name, in family order. An unmeasured member
    /// fails.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub failing: Vec<String>,
}

impl FamilyMeasurementSummary {
    /// The passing share in percent, when the summary was judged.
    #[must_use]
    pub fn yield_percent(&self) -> Option<f64> {
        self.passed
            .map(|passed| 100.0 * f64::from(passed) / f64::from(self.members.max(1)))
    }

    /// The number the measurement's value holds.
    #[must_use]
    pub fn statistic_value(&self) -> Option<f64> {
        match self.statistic {
            FamilyStatistic::Mean => Some(self.mean),
            FamilyStatistic::StdDev => self.std_dev,
            FamilyStatistic::Min => Some(self.min),
            FamilyStatistic::Max => Some(self.max),
            FamilyStatistic::Yield => self.yield_percent(),
        }
    }
}

fn deserialize_optional_source_text_string<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<BoundedString<MAX_SOURCE_TEXT_BYTES>>::deserialize(deserializer)
        .map(|value| value.map(|value| value.0))
}

/// What an annotation stands on.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "value")]
pub enum AnnotationAnchor {
    /// A place in the pane's frame, as shares of its width and height from
    /// its top left: it stays put as the view zooms and pans.
    Pane {
        x_fraction: f32,
        y_fraction: f32,
        /// The strip of a pane of several whose frame it is in, by the unit
        /// of the strip's first axis; the pane's first strip when absent.
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "deserialize_optional_unit_axis_string"
        )]
        unit: Option<String>,
    },
    /// The point of a trace at `coordinate`, which follows the trace.
    Trace {
        trace_id: TraceId,
        coordinate: TypedValue,
    },
    /// A point of the pane's data: `x` on its abscissa and `y` on the axis
    /// that reads `unit`, empty for a plain number. It follows the view as
    /// it zooms and pans, and stays where it is when the traces move.
    Data {
        x: f64,
        y: f64,
        #[serde(deserialize_with = "deserialize_unit_axis_string")]
        unit: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Annotation {
    pub id: AnnotationId,
    pub pane_id: PaneId,
    pub anchor: AnnotationAnchor,
    #[serde(deserialize_with = "deserialize_annotation_string")]
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LinkKind {
    HorizontalViewport,
    VerticalViewport,
    CursorPosition,
}

/// How a project-owned result document resolves future executions.
///
/// A pinned document is an immutable review artifact. A latest-bound
/// document may advance only to a newer retained run from the same simulation
/// plan and authored analysis instance; it never follows analysis kind, label,
/// display order, or whichever run happens to be selected.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ResultDocumentTrackingMode {
    Latest,
    #[default]
    Pinned,
}

/// Exact authority required to implement the Results toolbar's Latest/Pinned
/// contract without ambiguous rebinding.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResultDocumentTracking {
    pub mode: ResultDocumentTrackingMode,
    pub simulation_plan_id: Option<SimulationPlanId>,
    pub authored_analysis_id: Option<AnalysisInstanceId>,
}

impl ResultDocumentTracking {
    #[must_use]
    pub const fn pinned() -> Self {
        Self {
            mode: ResultDocumentTrackingMode::Pinned,
            simulation_plan_id: None,
            authored_analysis_id: None,
        }
    }

    #[must_use]
    pub const fn for_plan(
        mode: ResultDocumentTrackingMode,
        simulation_plan_id: SimulationPlanId,
        authored_analysis_id: AnalysisInstanceId,
    ) -> Self {
        Self {
            mode,
            simulation_plan_id: Some(simulation_plan_id),
            authored_analysis_id: Some(authored_analysis_id),
        }
    }

    fn validate(self) -> Result<(), VisualizationError> {
        if self.simulation_plan_id.is_some() != self.authored_analysis_id.is_some() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.tracking",
                message:
                    "simulation plan and authored analysis identities must be present together"
                        .to_owned(),
            });
        }
        if self.mode == ResultDocumentTrackingMode::Latest && self.simulation_plan_id.is_none() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.tracking",
                message: "latest tracking requires exact simulation-plan and analysis identities"
                    .to_owned(),
            });
        }
        Ok(())
    }
}

/// Project-owned formatting policy shared by interactive rendering and
/// document export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VisualizationPresentationPolicy {
    #[serde(default = "default_visualization_significant_digits")]
    pub significant_digits: u8,
    #[serde(default)]
    pub phase_continuous: bool,
}

/// A document that states no digits shows the reader's default.
const fn default_visualization_significant_digits() -> u8 {
    crate::display_preferences::DisplayedSignificantDigits::DEFAULT
}

impl Default for VisualizationPresentationPolicy {
    fn default() -> Self {
        Self {
            significant_digits: default_visualization_significant_digits(),
            phase_continuous: false,
        }
    }
}

impl VisualizationPresentationPolicy {
    fn validate(self) -> Result<(), VisualizationError> {
        if !(3..=17).contains(&self.significant_digits) {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.presentation.significant-digits",
                message: "significant digits must be between 3 and 17".to_owned(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkGroup {
    pub id: LinkGroupId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    pub kind: LinkKind,
    pub members: Vec<EntityRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VisualizationTransactionReceipt {
    pub document_id: ResultDocumentId,
    pub previous_revision: ObjectRevision,
    pub committed_revision: ObjectRevision,
    pub created: Vec<EntityRef>,
    pub tombstoned: Vec<EntityRef>,
    pub edit_count: usize,
}
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum VisualizationError {
    #[error("invalid {field}: {message}")]
    InvalidValue {
        field: &'static str,
        message: String,
    },
    #[error("duplicate source column key {0}")]
    DuplicateKey(String),
    #[error("source row {row} has {actual} values; expected {expected}")]
    RowWidth {
        row: usize,
        expected: usize,
        actual: usize,
    },
    #[error("column {column} expects {expected:?}, received {actual:?}")]
    ColumnTypeMismatch {
        column: String,
        expected: ValueType,
        actual: ValueType,
    },
    #[error("source row {0} duplicates an existing coordinate tuple")]
    DuplicateCoordinateRow(usize),
    #[error("dataset {0} is not bound to this document")]
    DatasetNotFound(DatasetId),
    #[error(
        "dataset {dataset_id} is bound to immutable digest {bound}; requested digest {requested}"
    )]
    SourceDigestMismatch {
        dataset_id: DatasetId,
        bound: ContentDigest,
        requested: ContentDigest,
    },
    #[error("column {0} does not exist")]
    ColumnNotFound(String),
    #[error("query has no exact source row; interpolation would be required")]
    InterpolationRequired,
    #[error("transaction expected revision {expected:?}, current revision is {actual:?}")]
    RevisionConflict {
        expected: ObjectRevision,
        actual: ObjectRevision,
    },
    #[error("stable identity space is exhausted")]
    IdentitySpaceExhausted,
    #[error("entity {0:?} does not exist")]
    EntityNotFound(EntityRef),
    #[error("entity {entity:?} is still referenced by {dependent:?}")]
    EntityInUse {
        entity: EntityRef,
        dependent: EntityRef,
    },
    #[error("link group members are invalid for {0:?}")]
    InvalidLinkMembers(LinkKind),
    #[error("comparison requires at least one signal")]
    EmptyComparison,
    #[error("comparison rows are not identical")]
    ComparisonRowsDiffer,
    #[error("comparison has no exact rows in common")]
    NoComparableRows,
    #[error("comparison signal {0} is not a real-valued signal")]
    NonNumericComparison(String),
    #[error("comparison signal {signal} has incompatible units {baseline:?} and {candidate:?}")]
    UnitMismatch {
        signal: String,
        baseline: Option<String>,
        candidate: Option<String>,
    },
    #[error("visualization document serialization failed: {0}")]
    Serialization(String),
    #[error(transparent)]
    Revision(#[from] RevisionError),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VisualizationDocument {
    schema_version: u16,
    id: ResultDocumentId,
    revision: ObjectRevision,
    title: String,
    tracking: ResultDocumentTracking,
    presentation: VisualizationPresentationPolicy,
    /// Whose working view this is; absent for a saved document.
    #[serde(skip_serializing_if = "Option::is_none")]
    working_view: Option<WorkingViewSource>,
    next_serial: u64,
    /// The number the next marker takes. Markers are numbered apart from
    /// the view's other identities, so they read M1, M2, ... as they are
    /// placed, and a number once given is never given again.
    next_marker_number: u64,
    datasets: journal::SpliceList<SourceDataset>,
    pages: journal::EntityList<Page>,
    panes: journal::EntityList<Pane>,
    axes: journal::EntityList<Axis>,
    traces: journal::EntityList<Trace, journal::SignalIndex>,
    cursors: journal::EntityList<Cursor>,
    markers: journal::EntityList<Marker>,
    measurements: journal::EntityList<Measurement>,
    annotations: journal::EntityList<Annotation>,
    link_groups: journal::EntityList<LinkGroup>,
    tombstones: journal::SpliceList<Tombstone>,
    comparisons: journal::SpliceList<ComparisonReceipt>,
    #[serde(skip_serializing_if = "journal::SpliceList::is_empty")]
    user_buses: journal::SpliceList<UserBus>,
    #[serde(skip_serializing_if = "journal::SpliceList::is_empty")]
    event_lane_orders: journal::SpliceList<EventLaneOrder>,
    #[serde(skip_serializing_if = "journal::SpliceList::is_empty")]
    expression_traces: journal::SpliceList<ExpressionTrace>,
    #[serde(skip_serializing_if = "journal::SpliceList::is_empty")]
    xy_plots: journal::SpliceList<XyPlot>,
    #[serde(skip_serializing_if = "journal::SpliceList::is_empty")]
    readout_pins: journal::SpliceList<ReadoutPin>,
    #[serde(skip_serializing_if = "journal::SpliceList::is_empty")]
    sheet_settings: journal::SpliceList<SheetSetting>,
    spectrum_settings: journal::SpliceList<PaneSpectrumSettings>,
    #[serde(skip_serializing_if = "journal::SpliceList::is_empty")]
    coefficient_settings: journal::SpliceList<PaneCoefficientSettings>,
    /// The rows of embedded tables a check is reading, while it reads them.
    #[serde(skip)]
    row_index: query::RowIndex,
    /// Which content the document holds, and what its last change changed.
    #[serde(skip)]
    changes: journal::Changes,
    /// The content digest of the generation it names, once asked for.
    #[serde(skip)]
    digest: DigestMemo,
}

#[derive(Deserialize)]
struct VisualizationDocumentWire {
    schema_version: u16,
    id: ResultDocumentId,
    revision: ObjectRevision,
    #[serde(deserialize_with = "deserialize_label_string")]
    title: String,
    tracking: ResultDocumentTracking,
    presentation: VisualizationPresentationPolicy,
    #[serde(default)]
    working_view: Option<WorkingViewSource>,
    next_serial: u64,
    #[serde(default)]
    next_marker_number: u64,
    datasets: BoundedSourceDatasets,
    pages: Vec<Page>,
    panes: Vec<Pane>,
    axes: Vec<Axis>,
    traces: Vec<Trace>,
    cursors: Vec<Cursor>,
    markers: Vec<Marker>,
    measurements: Vec<Measurement>,
    annotations: Vec<Annotation>,
    link_groups: Vec<LinkGroup>,
    tombstones: Vec<Tombstone>,
    comparisons: Vec<ComparisonReceipt>,
    #[serde(default)]
    user_buses: Option<Vec<UserBus>>,
    #[serde(default)]
    event_lane_orders: Option<Vec<EventLaneOrder>>,
    #[serde(default)]
    expression_traces: Option<Vec<ExpressionTrace>>,
    #[serde(default)]
    xy_plots: Option<Vec<XyPlot>>,
    #[serde(default)]
    readout_pins: Option<Vec<ReadoutPin>>,
    #[serde(default)]
    sheet_settings: Option<Vec<SheetSetting>>,
    spectrum_settings: Option<Vec<PaneSpectrumSettings>>,
    #[serde(default)]
    coefficient_settings: Option<Vec<PaneCoefficientSettings>>,
}

impl<'de> Deserialize<'de> for VisualizationDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VisualizationDocumentWire::deserialize(deserializer)?;
        let document = Self {
            schema_version: wire.schema_version,
            id: wire.id,
            revision: wire.revision,
            title: wire.title,
            tracking: wire.tracking,
            presentation: wire.presentation,
            working_view: wire.working_view,
            next_serial: wire.next_serial,
            next_marker_number: wire.next_marker_number,
            datasets: journal::SpliceList::new(wire.datasets.0),
            pages: journal::EntityList::new(wire.pages),
            panes: journal::EntityList::new(wire.panes),
            axes: journal::EntityList::new(wire.axes),
            traces: journal::EntityList::new(wire.traces),
            cursors: journal::EntityList::new(wire.cursors),
            markers: journal::EntityList::new(wire.markers),
            measurements: journal::EntityList::new(wire.measurements),
            annotations: journal::EntityList::new(wire.annotations),
            link_groups: journal::EntityList::new(wire.link_groups),
            tombstones: journal::SpliceList::new(wire.tombstones),
            comparisons: journal::SpliceList::new(wire.comparisons),
            user_buses: journal::SpliceList::new(wire.user_buses.unwrap_or_default()),
            event_lane_orders: journal::SpliceList::new(wire.event_lane_orders.unwrap_or_default()),
            expression_traces: journal::SpliceList::new(wire.expression_traces.unwrap_or_default()),
            xy_plots: journal::SpliceList::new(wire.xy_plots.unwrap_or_default()),
            readout_pins: journal::SpliceList::new(wire.readout_pins.unwrap_or_default()),
            sheet_settings: journal::SpliceList::new(wire.sheet_settings.unwrap_or_default()),
            spectrum_settings: journal::SpliceList::new(wire.spectrum_settings.unwrap_or_default()),
            coefficient_settings: journal::SpliceList::new(
                wire.coefficient_settings.unwrap_or_default(),
            ),
            row_index: query::RowIndex::default(),
            changes: journal::Changes::fresh(),
            digest: DigestMemo::default(),
        };
        if document.schema_version != Self::SCHEMA_VERSION {
            return Err(serde::de::Error::custom(format!(
                "unsupported visualization document schema version {}",
                document.schema_version
            )));
        }
        document.validate().map_err(serde::de::Error::custom)?;
        Ok(document)
    }
}

impl VisualizationDocument {
    /// The one schema a document is written in and read from. A document
    /// stating any other version is refused, never migrated. Unknown
    /// extension fields are ignored; a new required meaning takes a new
    /// version.
    pub const SCHEMA_VERSION: u16 = 16;

    pub fn new(
        title: impl Into<String>,
        datasets: Vec<SourceDataset>,
    ) -> Result<Self, VisualizationError> {
        let title = title.into();
        validate_label("visualization-document.title", &title)?;
        if datasets.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "at least one immutable dataset is required".to_owned(),
            });
        }
        validate_dataset_set(&datasets)?;
        let page_id = PageId::allocate(1)?;
        let pane_id = PaneId::allocate(2)?;
        let document = Self {
            schema_version: Self::SCHEMA_VERSION,
            id: ResultDocumentId::new(),
            revision: ObjectRevision::INITIAL,
            title,
            tracking: ResultDocumentTracking::pinned(),
            presentation: VisualizationPresentationPolicy::default(),
            working_view: None,
            next_serial: 3,
            next_marker_number: 1,
            datasets: journal::SpliceList::new(datasets),
            pages: journal::EntityList::new(vec![Page {
                id: page_id,
                title: "Page 1".to_owned(),
                layout: PageLayout::default(),
                template_id: default_page_template_id(),
                update_policy: PageUpdatePolicy::default(),
            }]),
            panes: journal::EntityList::new(vec![Pane {
                id: pane_id,
                page_id,
                title: "Plot 1".to_owned(),
                kind: PaneKind::Cartesian,
                viewer_id: default_viewer_id_for_kind(PaneKind::Cartesian).to_owned(),
                binding: None,
                placement: PanePlacement::Primary,
                order: 0,
                family_policy: None,
                hidden: false,
                strips: Vec::new(),
                maximized: false,
                shown: false,
            }]),
            axes: journal::EntityList::new(Vec::new()),
            traces: journal::EntityList::new(Vec::new()),
            cursors: journal::EntityList::new(Vec::new()),
            markers: journal::EntityList::new(Vec::new()),
            measurements: journal::EntityList::new(Vec::new()),
            annotations: journal::EntityList::new(Vec::new()),
            link_groups: journal::EntityList::new(Vec::new()),
            tombstones: journal::SpliceList::new(Vec::new()),
            comparisons: journal::SpliceList::new(Vec::new()),
            user_buses: journal::SpliceList::new(Vec::new()),
            event_lane_orders: journal::SpliceList::new(Vec::new()),
            expression_traces: journal::SpliceList::new(Vec::new()),
            xy_plots: journal::SpliceList::new(Vec::new()),
            readout_pins: journal::SpliceList::new(Vec::new()),
            sheet_settings: journal::SpliceList::new(Vec::new()),
            spectrum_settings: journal::SpliceList::new(Vec::new()),
            coefficient_settings: journal::SpliceList::new(Vec::new()),
            row_index: query::RowIndex::default(),
            changes: journal::Changes::fresh(),
            digest: DigestMemo::default(),
        };
        document.validate()?;
        Ok(document)
    }

    /// Create a document whose initial pane already has its canonical viewer,
    /// kind, and immutable source binding.
    ///
    /// The generic constructor deliberately starts with an unbound Cartesian
    /// plot. Dataset-driven creation cannot temporarily bind a table, Smith,
    /// histogram, or other typed viewer to that incompatible placeholder, so
    /// this constructor validates and provisions the first pane atomically.
    pub fn new_with_initial_pane(
        title: impl Into<String>,
        datasets: Vec<SourceDataset>,
        pane: NewPagePane,
    ) -> Result<Self, VisualizationError> {
        let mut document = Self::new(title, datasets)?;
        document.validate_pane_source(&document, pane.kind, &pane.viewer_id, pane.binding)?;
        validate_label("pane.title", &pane.title)?;
        let pane_id = document.panes[0].id;
        {
            let initial = document.panes.at_mut(0);
            initial.title = pane.title;
            initial.kind = pane.kind;
            initial.viewer_id = pane.viewer_id.clone();
            initial.binding = pane.binding;
        }
        if let Some(binding) = pane.binding {
            document.provision_bound_pane(
                pane_id,
                &pane.viewer_id,
                binding,
                &pane.plotted,
                &mut Vec::new(),
            )?;
        }
        document.validate()?;
        Ok(document)
    }

    #[must_use]
    pub const fn id(&self) -> ResultDocumentId {
        self.id
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    /// Human-facing document title retained by the project document registry.
    ///
    /// The title is authoritative project content rather than presentation
    /// state: document tabs, reports, and duplicate-name validation all read
    /// this exact value.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Give this document `title`, as renaming it does: a new revision of
    /// the same document. The project catalog checks the title is free.
    pub fn retitle(
        &mut self,
        title: impl Into<String>,
    ) -> Result<VisualizationTransactionReceipt, VisualizationError> {
        let title = title.into();
        validate_label("visualization-document.title", &title)?;
        let previous_revision = self.revision;
        let committed_revision = previous_revision.next()?;
        self.title = title;
        self.revision = committed_revision;
        self.changes.bump();
        Ok(VisualizationTransactionReceipt {
            document_id: self.id,
            previous_revision,
            committed_revision,
            created: Vec::new(),
            tombstoned: Vec::new(),
            edit_count: 0,
        })
    }

    /// A saved document of its own copied whole from this one under
    /// `title`: a new identity at the first revision, with every page, pane,
    /// trace, marker, axis and binding this one holds. A working view is
    /// saved as a document rather than duplicated.
    pub fn duplicate(&self, title: impl Into<String>) -> Result<Self, VisualizationError> {
        if self.working_view.is_some() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.working-view",
                message: "a working view is saved as a document, not duplicated".to_owned(),
            });
        }
        let title = title.into();
        validate_label("visualization-document.title", &title)?;
        let mut document = self.clone();
        document.id = ResultDocumentId::new();
        document.revision = ObjectRevision::INITIAL;
        document.title = title;
        document.changes = journal::Changes::fresh();
        document.validate()?;
        Ok(document)
    }

    #[must_use]
    pub const fn tracking(&self) -> ResultDocumentTracking {
        self.tracking
    }

    #[must_use]
    pub const fn presentation(&self) -> VisualizationPresentationPolicy {
        self.presentation
    }

    #[must_use]
    pub fn datasets(&self) -> &[SourceDataset] {
        &self.datasets
    }

    #[must_use]
    pub fn pages(&self) -> &[Page] {
        &self.pages
    }

    #[must_use]
    pub fn panes(&self) -> &[Pane] {
        &self.panes
    }

    #[must_use]
    pub fn axes(&self) -> &[Axis] {
        &self.axes
    }

    #[must_use]
    pub fn traces(&self) -> &[Trace] {
        &self.traces
    }

    /// The trace `trace_id`, found by its identity.
    #[must_use]
    pub fn trace(&self, trace_id: TraceId) -> Option<&Trace> {
        self.traces
            .position(trace_id.get())
            .map(|index| &self.traces[index])
    }

    /// The traces of pane `pane_id` that read `signal` (the vector each was
    /// provisioned from, or its label when it names none), in order, found
    /// without reading the pane's other traces.
    #[must_use]
    pub fn traces_reading(&self, pane_id: PaneId, signal: &str) -> Vec<&Trace> {
        self.traces.reading_items(pane_id, signal)
    }

    /// A number that names the content the document holds now: no other
    /// content of any document in this process had it, a copy keeps it, and
    /// every change gives a new one.
    #[must_use]
    pub fn generation(&self) -> u64 {
        self.changes.generation
    }

    /// The pane and signal of each trace the document changed since it held
    /// the content `generation` named: made, changed or taken out, each as it
    /// stands now and, for one moved off it, as it stood before. Empty when
    /// it holds that content still; `None` when more than its last change
    /// lies between, or it was made or read whole since, so a reader draws
    /// it whole.
    #[must_use]
    pub fn signals_changed_since(&self, generation: u64) -> Option<Vec<(PaneId, &str)>> {
        self.last_change_since(generation).map(|last| {
            let Some(last) = last else {
                return Vec::new();
            };
            let present = last.present.iter().filter_map(|trace_id| {
                self.trace(*trace_id)
                    .map(|trace| (trace.pane_id, journal::signal_of(trace)))
            });
            let gone = last
                .gone
                .iter()
                .map(|(pane_id, signal)| (*pane_id, signal.as_str()));
            present.chain(gone).collect()
        })
    }

    /// How many traces the document made, changed or took out since it held
    /// the content `generation` named, as [`Self::signals_changed_since`]
    /// names them; `None` as it names none.
    #[must_use]
    pub fn signals_changed_count_since(&self, generation: u64) -> Option<usize> {
        self.last_change_since(generation)
            .map(|last| last.map_or(0, |last| last.present.len() + last.gone.len()))
    }

    /// [`Self::signals_changed_since`], each signal with every trace of its
    /// pane that reads it now, in the document's order. The traces are
    /// taken from the change where it touched every one, as a bulk tick and
    /// its steps touch the one trace of each signal, and looked up
    /// otherwise.
    #[must_use]
    pub fn signal_traces_changed_since(
        &self,
        generation: u64,
    ) -> Option<Vec<(PaneId, &str, Vec<&Trace>)>> {
        let last = self.last_change_since(generation)?;
        let Some(last) = last else {
            return Some(Vec::new());
        };
        let mut changed = Vec::with_capacity(last.present.len() + last.gone.len());
        // A signal read by more traces than the change touched, or by one
        // the change took off and one it kept, is looked up, once.
        let mut read = Vec::new();
        for trace in last
            .present
            .iter()
            .filter_map(|trace_id| self.trace(*trace_id))
        {
            let signal = journal::signal_of(trace);
            if self.traces.held_for(trace.pane_id, signal) == 1 {
                changed.push((trace.pane_id, signal, vec![trace]));
            } else {
                read.push((trace.pane_id, signal));
            }
        }
        for (pane_id, signal) in &last.gone {
            if self.traces.held_for(*pane_id, signal) == 0 {
                changed.push((*pane_id, signal.as_str(), Vec::new()));
            } else {
                read.push((*pane_id, signal.as_str()));
            }
        }
        read.sort_unstable();
        read.dedup();
        changed.extend(read.into_iter().map(|(pane_id, signal)| {
            (pane_id, signal, self.traces.reading_items(pane_id, signal))
        }));
        Some(changed)
    }

    /// Whether the document changed any pane's event-lane order or buses
    /// since it held the content `generation` named; `None` as for
    /// [`Self::signals_changed_since`].
    #[must_use]
    pub fn event_lanes_changed_since(&self, generation: u64) -> Option<bool> {
        self.last_change_since(generation)
            .map(|last| last.is_some_and(|last| last.lanes))
    }

    /// The one change since the content `generation` named: `Some(None)`
    /// when the document holds it still, `None` when more lies between.
    fn last_change_since(&self, generation: u64) -> Option<Option<&journal::LastChange>> {
        if generation == self.changes.generation {
            return Some(None);
        }
        self.changes
            .last
            .as_ref()
            .filter(|last| last.from == generation)
            .map(Some)
    }

    #[must_use]
    pub fn cursors(&self) -> &[Cursor] {
        &self.cursors
    }

    #[must_use]
    pub fn markers(&self) -> &[Marker] {
        &self.markers
    }

    #[must_use]
    pub fn measurements(&self) -> &[Measurement] {
        &self.measurements
    }

    /// The measurements in the order the reader put them in, which the
    /// Measurements table lists them in.
    #[must_use]
    pub fn measurements_in_order(&self) -> Vec<&Measurement> {
        let mut ordered: Vec<&Measurement> = self.measurements.iter().collect();
        ordered.sort_by_key(|measurement| measurement.place);
        ordered
    }

    /// The place a measurement made now takes: after every one held.
    fn next_measurement_place(&self) -> u64 {
        self.measurements
            .iter()
            .map(|measurement| measurement.place + 1)
            .max()
            .unwrap_or(0)
    }

    #[must_use]
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }

    #[must_use]
    pub fn link_groups(&self) -> &[LinkGroup] {
        &self.link_groups
    }

    #[must_use]
    pub fn comparisons(&self) -> &[ComparisonReceipt] {
        &self.comparisons
    }

    /// The logic lanes the reader grouped into buses, in definition order.
    #[must_use]
    pub fn user_buses(&self) -> &[UserBus] {
        &self.user_buses
    }

    /// The calculator expressions plotted on the document's panes.
    pub fn expression_traces(&self) -> &[ExpressionTrace] {
        &self.expression_traces
    }

    /// The plots of the document's pane axes against a chosen signal or
    /// expression, each pane's in the order the reader made them.
    pub fn xy_plots(&self) -> &[XyPlot] {
        &self.xy_plots
    }

    /// The readouts pinned on the document's panes, one per pane and sheet.
    pub fn readout_pins(&self) -> &[ReadoutPin] {
        &self.readout_pins
    }

    /// What the reader left each settings-keeping sheet showing, per pane.
    pub fn sheet_settings(&self) -> &[SheetSetting] {
        &self.sheet_settings
    }

    /// How the reader set `pane_id`'s spectrum sheet, when they changed it.
    #[must_use]
    pub fn spectrum_settings(
        &self,
        pane_id: PaneId,
    ) -> Option<&crate::fft::settings::SpectrumSettings> {
        self.spectrum_settings
            .iter()
            .find(|held| held.pane_id == pane_id)
            .map(|held| &held.settings)
    }

    /// How the reader set `pane_id`'s coefficient spectrum sheet, when they
    /// changed it.
    #[must_use]
    pub fn coefficient_settings(
        &self,
        pane_id: PaneId,
    ) -> Option<&crate::harmonic_spectrum::settings::CoefficientSettings> {
        self.coefficient_settings
            .iter()
            .find(|held| held.pane_id == pane_id)
            .map(|held| &held.settings)
    }

    /// The order one pane draws its event lanes in; empty when it recorded
    /// none.
    #[must_use]
    pub fn event_lane_order(&self, pane_id: PaneId) -> &[EventLaneRef] {
        self.event_lane_orders
            .iter()
            .find(|order| order.pane_id == pane_id)
            .map_or(&[], |order| order.lanes.as_slice())
    }

    #[must_use]
    pub fn tombstones(&self) -> &[Tombstone] {
        &self.tombstones
    }

    /// Authenticates the complete validated document envelope.
    ///
    /// The digest includes schema, stable identity, revision, immutable source
    /// datasets, presentation entities, and comparison receipts. It is the
    /// publication boundary for references that must identify one exact
    /// visualization revision rather than merely the underlying dataset.
    ///
    /// Content keeps its digest: asked again of the generation it was worked
    /// out for, the document answers from what it kept.
    pub fn content_digest(&self) -> Result<ContentDigest, VisualizationError> {
        let generation = self.changes.generation;
        if let Some(digest) = self.digest.get(generation) {
            return Ok(digest);
        }
        let digest = self.work_out_content_digest()?;
        self.digest.keep(generation, digest);
        Ok(digest)
    }

    fn work_out_content_digest(&self) -> Result<ContentDigest, VisualizationError> {
        // A report's copy carries the samples its figures draw. They are the
        // referenced runs' own samples, which each binding's digest already
        // authenticates, so the copy authenticates as the document it copies.
        if self.datasets.iter().any(|dataset| {
            dataset
                .run
                .as_ref()
                .is_some_and(RunSource::carries_figure_samples)
        }) {
            let mut document = self.clone();
            for run in document
                .datasets
                .edit()
                .iter_mut()
                .filter_map(|dataset| dataset.run.as_mut())
            {
                run.set_figure_samples(Vec::new());
            }
            return document.work_out_content_digest();
        }
        self.validate()?;
        let mut writer = Sha256Writer::default();
        serde_json::to_writer(&mut writer, self)
            .map_err(|error| VisualizationError::Serialization(error.to_string()))?;
        Ok(writer.finish())
    }

    /// Applies the edits to the document in place, checks what they
    /// changed, and commits once. Any error puts back what the edits
    /// changed, so it leaves `self` byte-for-byte unchanged.
    pub fn transact(
        &mut self,
        expected_revision: ObjectRevision,
        edits: Vec<DocumentEdit>,
    ) -> Result<VisualizationTransactionReceipt, VisualizationError> {
        self.commit_edits(expected_revision, edits, false)
            .map(|(receipt, _)| receipt)
    }

    /// [`Self::transact`], returning with the receipt the step that undoes
    /// the transaction: what it changed, as it stood before.
    pub fn transact_with_undo(
        &mut self,
        expected_revision: ObjectRevision,
        edits: Vec<DocumentEdit>,
    ) -> Result<(VisualizationTransactionReceipt, ViewStep), VisualizationError> {
        self.commit_edits(expected_revision, edits, true)
            .map(|(receipt, step)| (receipt, step.expect("a step is kept when asked for")))
    }

    fn commit_edits(
        &mut self,
        expected_revision: ObjectRevision,
        edits: Vec<DocumentEdit>,
        undo: bool,
    ) -> Result<(VisualizationTransactionReceipt, Option<ViewStep>), VisualizationError> {
        if expected_revision != self.revision {
            return Err(VisualizationError::RevisionConflict {
                expected: expected_revision,
                actual: self.revision,
            });
        }
        if edits.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "transaction.edits",
                message: "a transaction holds at least one edit".to_owned(),
            });
        }
        let previous_revision = self.revision;
        let committed_revision = previous_revision.next()?;
        let edit_count = edits.len();
        let mut created = Vec::new();
        let mut tombstoned = Vec::new();
        let began = self.begin();
        let applied = self
            .apply_edits(edits, committed_revision, &mut created, &mut tombstoned)
            .and_then(|()| self.check_changes());
        if let Err(error) = applied {
            self.roll_back(began);
            return Err(error);
        }
        self.assert_whole();
        self.note_change();
        let step = undo.then(|| ViewStep::back_to(self, &began));
        self.end();
        self.revision = committed_revision;
        Ok((
            VisualizationTransactionReceipt {
                document_id: self.id,
                previous_revision,
                committed_revision,
                created,
                tombstoned,
                edit_count,
            },
            step,
        ))
    }

    /// In a debug build, a whole check of the document after a check of
    /// what an edit changed passed: the two agree, or the narrower one
    /// missed an invariant the edit can break.
    fn assert_whole(&self) {
        #[cfg(debug_assertions)]
        if let Err(error) = self.validate() {
            panic!("an edit the document accepted left it invalid: {error}");
        }
    }

    /// Put back what `step` holds, over the datasets this document reads
    /// now, as the next revision, and return the step that reverses it: an
    /// undo step gives the redo step and the other way round. The title is
    /// the project's name for the document, not its view, and identities are
    /// never reused, so neither steps back. A step of another document, or
    /// one that leaves the view naming what it cannot read, is refused and
    /// leaves `self` unchanged.
    pub fn take_step(
        &mut self,
        expected_revision: ObjectRevision,
        step: ViewStep,
    ) -> Result<(VisualizationTransactionReceipt, ViewStep), VisualizationError> {
        if expected_revision != self.revision {
            return Err(VisualizationError::RevisionConflict {
                expected: expected_revision,
                actual: self.revision,
            });
        }
        if step.document_id() != self.id {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.step",
                message: "the step belongs to another document".to_owned(),
            });
        }
        let previous_revision = self.revision;
        let committed_revision = previous_revision.next()?;
        let shown: Vec<(PaneId, PaneDataBinding)> = self
            .panes
            .iter()
            .filter_map(|pane| Some((pane.id, pane.binding?)))
            .collect();
        let stale: Vec<DatasetBinding> = step
            .bindings()
            .filter(|binding| {
                !self
                    .datasets
                    .iter()
                    .any(|dataset| dataset.binding.dataset_id == binding.dataset_id)
            })
            .collect();
        let began = self.begin();
        step.put_back(self);
        let followed = self
            .follow_into(stale, &shown)
            .and_then(|()| self.check_changes());
        if let Err(error) = followed {
            self.roll_back(began);
            return Err(error);
        }
        self.assert_whole();
        self.note_change();
        let reverse = ViewStep::back_to(self, &began);
        self.end();
        self.revision = committed_revision;
        Ok((
            VisualizationTransactionReceipt {
                document_id: self.id,
                previous_revision,
                committed_revision,
                created: Vec::new(),
                tombstoned: Vec::new(),
                edit_count: 0,
            },
            reverse,
        ))
    }

    /// Move what a step just put back read from a run this document no
    /// longer holds, the `stale` bindings, onto the run it reads now: each
    /// pane onto the binding the same pane had before the step, in `shown`,
    /// and each trace onto the signal of the same name there. A view
    /// followed a newer run between the two, so the earlier view is restored
    /// on the run now shown.
    fn follow_into(
        &mut self,
        mut stale: Vec<DatasetBinding>,
        shown: &[(PaneId, PaneDataBinding)],
    ) -> Result<(), VisualizationError> {
        stale.sort_by_key(|binding| binding.dataset_id.as_uuid());
        stale.dedup();
        let Some(&(_, first)) = shown.first() else {
            return Ok(());
        };
        let now = |pane_id: PaneId| {
            shown
                .iter()
                .find(|(shown_id, _)| *shown_id == pane_id)
                .map(|(_, binding)| *binding)
        };
        for previous in stale {
            let pairs = self
                .panes
                .iter()
                .filter_map(|pane| {
                    let before = pane.binding.filter(|binding| binding.dataset == previous)?;
                    Some((before.analysis_id, now(pane.id)?))
                })
                .collect::<Vec<_>>();
            let next = pairs.first().map_or(first.dataset, |(_, now)| now.dataset);
            let analyses = pairs
                .iter()
                .filter(|(_, now)| now.dataset == next)
                .map(|(before, now)| (*before, now.analysis_id))
                .collect::<HashMap<_, _>>();
            let fallback = pairs
                .iter()
                .find(|(_, now)| now.dataset == next)
                .map_or(first.analysis_id, |(_, now)| now.analysis_id);
            self.rebind_consumers(
                previous,
                next,
                |from| {
                    Some(
                        from.and_then(|from| analyses.get(&from).copied())
                            .unwrap_or(fallback),
                    )
                },
                "shown",
                apply::RebindMode::KeepMissing,
            )?;
        }
        Ok(())
    }

    pub fn compare(
        &self,
        request: &ComparisonRequest,
    ) -> Result<ComparisonReceipt, VisualizationError> {
        if request.signal_keys.is_empty() {
            return Err(VisualizationError::EmptyComparison);
        }
        request.policy.tolerance.validate()?;
        let baseline = self.dataset_for_binding(request.baseline)?;
        let candidate = self.dataset_for_binding(request.candidate)?;
        compare_source_datasets(baseline, candidate, request)
    }

    fn dataset_for_binding(
        &self,
        binding: DatasetBinding,
    ) -> Result<&SourceDataset, VisualizationError> {
        let dataset = self
            .datasets
            .iter()
            .find(|dataset| dataset.binding.dataset_id == binding.dataset_id)
            .ok_or(VisualizationError::DatasetNotFound(binding.dataset_id))?;
        if dataset.binding.content_digest != binding.content_digest {
            return Err(VisualizationError::SourceDigestMismatch {
                dataset_id: binding.dataset_id,
                bound: dataset.binding.content_digest,
                requested: binding.content_digest,
            });
        }
        Ok(dataset)
    }

    fn allocate_serial(&mut self) -> Result<u64, VisualizationError> {
        let serial = self.next_serial;
        self.next_serial = self
            .next_serial
            .checked_add(1)
            .ok_or(VisualizationError::IdentitySpaceExhausted)?;
        Ok(serial)
    }

    /// A new marker's identity: the view's next marker number, past any
    /// marker it holds or deleted, so a view's markers read M1, M2, ... in
    /// the order they are placed however many panes and traces it holds,
    /// and an expression naming a deleted or undone marker never reads a
    /// newer one. A marker still takes a serial, and its number is never
    /// above it: the view's identities stay below the next serial, and a
    /// transaction tells what it made from what it found as it does for
    /// every other entity.
    fn allocate_marker_number(&mut self) -> Result<MarkerId, VisualizationError> {
        let serial = self.allocate_serial()?;
        let deleted = self
            .tombstones
            .iter()
            .filter_map(|tombstone| match tombstone.entity {
                EntityRef::Marker(id) => Some(id.get()),
                _ => None,
            });
        let highest = self
            .markers
            .iter()
            .map(|marker| marker.id.get())
            .chain(deleted)
            .max()
            .unwrap_or(0);
        let number = (highest + 1).max(self.next_marker_number).min(serial);
        self.next_marker_number = number + 1;
        MarkerId::allocate(number)
    }
}

#[cfg(test)]
mod abscissa_tests;
#[cfg(test)]
mod expression_tests;
#[cfg(test)]
mod lane_tests;
#[cfg(test)]
mod run_source_tests;
#[cfg(test)]
mod strip_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod transaction_tests;
#[cfg(test)]
mod working_view_tests;

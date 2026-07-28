//! Canonical, UI-independent domain model for Visualization Studio.
//!
//! A visualization document is a versioned presentation over immutable result
//! datasets.  Source samples and their content digests are never edited in
//! place; presentation changes are committed atomically through [`DocumentEdit`].

mod family;
mod query;

pub use family::*;
pub use query::*;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::num::NonZeroU64;

use serde::de::{IgnoredAny, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as _, Sha256};

use crate::product::{
    AnalysisInstanceId, ContentDigest, DatasetBinding, DatasetId, ObjectRevision, ResultDocumentId,
    RevisionError,
};
use crate::results::viewer_catalog::{ViewerArt, viewer_document};

/// Maximum immutable result projections retained by one visualization document.
pub const MAX_VISUALIZATION_DATASETS: usize = 32;
/// Maximum typed columns retained by one immutable result projection.
pub const MAX_SOURCE_COLUMNS: usize = 4_096;
/// Maximum rows retained by one immutable result projection.
pub const MAX_SOURCE_ROWS: usize = 1_000_000;
/// Maximum typed cells retained by one immutable result projection.
pub const MAX_SOURCE_CELLS_PER_DATASET: usize = 8_000_000;
/// Maximum typed cells retained across one visualization document.
pub const MAX_SOURCE_CELLS_TOTAL: usize = 8_000_000;
/// Maximum UTF-8 bytes retained by text-valued cells in one source dataset.
pub const MAX_SOURCE_TEXT_BYTES_PER_DATASET: usize = 64 * 1024 * 1024;
/// Maximum UTF-8 bytes retained by text-valued cells across one document.
pub const MAX_SOURCE_TEXT_BYTES_TOTAL: usize = 128 * 1024 * 1024;
/// Maximum number of report-style pages in one visualization document.
pub const MAX_VISUALIZATION_PAGES: usize = 64;
/// Maximum number of panes in one visualization document.
pub const MAX_VISUALIZATION_PANES: usize = 256;
/// Maximum number of axes in one visualization document.
pub const MAX_VISUALIZATION_AXES: usize = 512;
/// Maximum number of traces in one visualization document.
pub const MAX_VISUALIZATION_TRACES: usize = 4_096;
/// Maximum number of cursors in one visualization document.
pub const MAX_VISUALIZATION_CURSORS: usize = 4_096;
/// Maximum number of markers in one visualization document.
pub const MAX_VISUALIZATION_MARKERS: usize = 8_192;
/// Maximum number of measurements in one visualization document.
pub const MAX_VISUALIZATION_MEASUREMENTS: usize = 4_096;
/// Maximum number of annotations in one visualization document.
pub const MAX_VISUALIZATION_ANNOTATIONS: usize = 4_096;
/// Maximum number of viewport/cursor link groups in one visualization document.
pub const MAX_VISUALIZATION_LINK_GROUPS: usize = 1_024;
/// Maximum number of immutable deletion records in one visualization document.
pub const MAX_VISUALIZATION_TOMBSTONES: usize = 65_536;
/// Maximum number of retained exact-comparison receipts.
pub const MAX_VISUALIZATION_COMPARISONS: usize = 4_096;
/// Maximum members carried by one measurement or link group.
pub const MAX_ENTITY_REFERENCES: usize = 4_096;
/// Maximum signal records carried by one comparison receipt.
pub const MAX_COMPARISON_SIGNALS: usize = 4_096;
/// Maximum stable-key length in UTF-8 bytes.
pub const MAX_VISUALIZATION_KEY_BYTES: usize = 256;
/// Maximum ordinary title/label length in UTF-8 bytes.
pub const MAX_VISUALIZATION_LABEL_BYTES: usize = 1_024;
/// Maximum engineering-unit length in UTF-8 bytes.
pub const MAX_VISUALIZATION_UNIT_BYTES: usize = 64;
/// Maximum retained text-valued source cell length in UTF-8 bytes.
pub const MAX_SOURCE_TEXT_BYTES: usize = 4_096;
/// Maximum retained annotation length in UTF-8 bytes.
pub const MAX_ANNOTATION_TEXT_BYTES: usize = 16_384;
/// Maximum number of edits accepted by one atomic visualization transaction.
pub const MAX_VISUALIZATION_TRANSACTION_EDITS: usize = 4_096;
/// Maximum declared family dimensions in one presentation policy.
pub const MAX_FAMILY_DIMENSIONS: usize = 32;
/// Maximum visual encoding maps in one family presentation policy.
pub const MAX_FAMILY_ENCODINGS: usize = 16;
/// Maximum exact members in one family set-membership predicate.
pub const MAX_FAMILY_PREDICATE_VALUES: usize = 256;
/// Maximum direct children in one family logical predicate node.
pub const MAX_FAMILY_PREDICATE_CHILDREN: usize = 64;
/// Maximum nesting depth of one family predicate root, including `Not` nodes.
pub const MAX_FAMILY_PREDICATE_DEPTH: usize = 32;
/// Maximum total nodes retained by one family predicate root.
pub const MAX_FAMILY_PREDICATE_NODES: usize = 1_024;
/// Maximum comparison-signal records retained across all receipts in one document.
pub const MAX_VISUALIZATION_COMPARISON_SIGNALS_TOTAL: usize = 16_384;
/// Maximum trace references retained across all measurements in one document.
pub const MAX_VISUALIZATION_MEASUREMENT_TRACE_REFERENCES_TOTAL: usize = 16_384;
/// Maximum entity references retained across all link groups in one document.
pub const MAX_VISUALIZATION_LINK_MEMBER_REFERENCES_TOTAL: usize = 16_384;

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

trait NestedResourceCount {
    fn nested_resource_count(&self) -> usize;
}

struct AggregateBoundedVec<T, const MAX_ITEMS: usize, const MAX_NESTED: usize>(Vec<T>);

impl<T, const MAX_ITEMS: usize, const MAX_NESTED: usize>
    AggregateBoundedVec<T, MAX_ITEMS, MAX_NESTED>
{
    fn into_inner(self) -> Vec<T> {
        self.0
    }
}

impl<'de, T, const MAX_ITEMS: usize, const MAX_NESTED: usize> Deserialize<'de>
    for AggregateBoundedVec<T, MAX_ITEMS, MAX_NESTED>
where
    T: Deserialize<'de> + NestedResourceCount,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct AggregateVisitor<T, const MAX_ITEMS: usize, const MAX_NESTED: usize>(PhantomData<T>);

        impl<'de, T, const MAX_ITEMS: usize, const MAX_NESTED: usize> Visitor<'de>
            for AggregateVisitor<T, MAX_ITEMS, MAX_NESTED>
        where
            T: Deserialize<'de> + NestedResourceCount,
        {
            type Value = AggregateBoundedVec<T, MAX_ITEMS, MAX_NESTED>;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    formatter,
                    "at most {MAX_ITEMS} entries containing {MAX_NESTED} aggregate nested resources"
                )
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                if sequence.size_hint().is_some_and(|hint| hint > MAX_ITEMS) {
                    return Err(serde::de::Error::custom(format!(
                        "sequence exceeds the {MAX_ITEMS}-entry resource limit"
                    )));
                }
                let mut values =
                    Vec::with_capacity(sequence.size_hint().unwrap_or(0).min(MAX_ITEMS));
                let mut nested = 0_usize;
                while values.len() < MAX_ITEMS {
                    let Some(value) = sequence.next_element::<T>()? else {
                        return Ok(AggregateBoundedVec(values));
                    };
                    nested = nested
                        .checked_add(value.nested_resource_count())
                        .ok_or_else(|| {
                            <A::Error as serde::de::Error>::custom(
                                "aggregate nested resource count overflowed",
                            )
                        })?;
                    if nested > MAX_NESTED {
                        return Err(serde::de::Error::custom(format!(
                            "sequence exceeds the {MAX_NESTED}-entry aggregate nested resource limit"
                        )));
                    }
                    values.push(value);
                }
                if sequence.next_element::<IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::custom(format!(
                        "sequence exceeds the {MAX_ITEMS}-entry resource limit"
                    )));
                }
                Ok(AggregateBoundedVec(values))
            }
        }

        deserializer.deserialize_seq(AggregateVisitor::<T, MAX_ITEMS, MAX_NESTED>(PhantomData))
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

fn deserialize_label_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoundedString::<MAX_VISUALIZATION_LABEL_BYTES>::deserialize(deserializer).map(|value| value.0)
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

fn deserialize_filter_source_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    BoundedString::<4_096>::deserialize(deserializer).map(|value| value.0)
}

fn deserialize_label_prefix<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<BoundedString<64>>::deserialize(deserializer).map(|value| value.map(|value| value.0))
}

fn deserialize_bounded_vec<'de, D, T, const MAX: usize>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    BoundedVec::<T, MAX>::deserialize(deserializer).map(BoundedVec::into_inner)
}

fn deserialize_measurement_trace_ids<'de, D>(deserializer: D) -> Result<Vec<TraceId>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, _, MAX_ENTITY_REFERENCES>(deserializer)
}

fn deserialize_link_members<'de, D>(deserializer: D) -> Result<Vec<EntityRef>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, _, MAX_ENTITY_REFERENCES>(deserializer)
}

fn deserialize_comparison_signals<'de, D>(
    deserializer: D,
) -> Result<Vec<SignalComparison>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, _, MAX_COMPARISON_SIGNALS>(deserializer)
}

fn deserialize_family_dimensions<'de, D>(deserializer: D) -> Result<Vec<FamilyDimension>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, _, MAX_FAMILY_DIMENSIONS>(deserializer)
}

fn deserialize_family_encodings<'de, D>(deserializer: D) -> Result<Vec<FamilyEncodingMap>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, _, MAX_FAMILY_ENCODINGS>(deserializer)
}

fn deserialize_family_predicate_values<'de, D>(deserializer: D) -> Result<Vec<TypedValue>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, _, MAX_FAMILY_PREDICATE_VALUES>(deserializer)
}

fn deserialize_family_predicate_children<'de, D>(
    deserializer: D,
) -> Result<Vec<FamilyPredicate>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, _, MAX_FAMILY_PREDICATE_CHILDREN>(deserializer)
}

#[derive(Deserialize)]
#[serde(transparent)]
struct BoundedKey(#[serde(deserialize_with = "deserialize_key_string")] String);

fn deserialize_comparison_signal_keys<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_bounded_vec::<_, BoundedKey, MAX_COMPARISON_SIGNALS>(deserializer)
        .map(|keys| keys.into_iter().map(|key| key.0).collect())
}

#[derive(Default)]
struct Sha256Writer(Sha256);

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
stable_id!(OperationId);

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
    Real(f64),
    Integer(i64),
    Boolean(bool),
    Text(String),
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "value")]
enum TypedValueWire {
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
            TypedValueWire::Real(value) => Self::Real(value),
            TypedValueWire::Integer(value) => Self::Integer(value),
            TypedValueWire::Boolean(value) => Self::Boolean(value),
            TypedValueWire::Text(value) => Self::Text(value),
        })
    }
}

impl TypedValue {
    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        match self {
            Self::Real(_) => ValueType::Real,
            Self::Integer(_) => ValueType::Integer,
            Self::Boolean(_) => ValueType::Boolean,
            Self::Text(_) => ValueType::Text,
        }
    }

    fn validate(&self, field: &'static str) -> Result<(), VisualizationError> {
        match self {
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

    fn exact_eq(&self, other: &Self) -> bool {
        match (self, other) {
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
    Real(u64),
    Integer(i64),
    Boolean(bool),
    Text(&'a str),
}

impl<'a> From<&'a TypedValue> for ExactCoordinateValue<'a> {
    fn from(value: &'a TypedValue) -> Self {
        match value {
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
    pub fn label(&self) -> &str {
        &self.label
    }

    #[must_use]
    pub const fn value_type(&self) -> ValueType {
        self.value_type
    }

    #[must_use]
    pub const fn role(&self) -> ColumnRole {
        self.role
    }

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
    values: BoundedVec<TypedValue, MAX_SOURCE_COLUMNS>,
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
                TypedValue::Real(_) | TypedValue::Integer(_) | TypedValue::Boolean(_) => 0,
            };
            total
                .checked_add(bytes)
                .ok_or_else(|| VisualizationError::InvalidValue {
                    field: "source-dataset.retained-text-bytes",
                    message: "retained source text byte count overflowed".to_owned(),
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
                write!(
                    formatter,
                    "at most {MAX_SOURCE_ROWS} rows and {MAX_SOURCE_CELLS_PER_DATASET} cells"
                )
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                if sequence
                    .size_hint()
                    .is_some_and(|hint| hint > MAX_SOURCE_ROWS)
                {
                    return Err(serde::de::Error::custom(format!(
                        "source row sequence exceeds the {MAX_SOURCE_ROWS}-row resource limit"
                    )));
                }
                let mut rows =
                    Vec::with_capacity(sequence.size_hint().unwrap_or(0).min(MAX_SOURCE_ROWS));
                let mut cells = 0_usize;
                let mut text_bytes = 0_usize;
                while rows.len() < MAX_SOURCE_ROWS {
                    let Some(row) = sequence.next_element::<SourceRow>()? else {
                        return Ok(BoundedSourceRows(rows));
                    };
                    cells = cells.checked_add(row.values.len()).ok_or_else(|| {
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
                if sequence.next_element::<IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::custom(format!(
                        "source row sequence exceeds the {MAX_SOURCE_ROWS}-row resource limit"
                    )));
                }
                Ok(BoundedSourceRows(rows))
            }
        }

        deserializer.deserialize_seq(SourceRowsVisitor)
    }
}

/// An immutable snapshot of one persisted result dataset.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SourceDataset {
    binding: DatasetBinding,
    columns: Vec<SourceColumn>,
    rows: Vec<SourceRow>,
}

#[derive(Deserialize)]
struct SourceDatasetWire {
    binding: DatasetBinding,
    columns: BoundedVec<SourceColumn, MAX_SOURCE_COLUMNS>,
    rows: BoundedSourceRows,
}

impl<'de> Deserialize<'de> for SourceDataset {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = SourceDatasetWire::deserialize(deserializer)?;
        let dataset = Self {
            binding: wire.binding,
            columns: wire.columns.into_inner(),
            rows: wire.rows.0,
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
        if self.columns.is_empty() || self.columns.len() > MAX_SOURCE_COLUMNS {
            return Err(VisualizationError::InvalidValue {
                field: "source-dataset.columns",
                message: format!("a dataset requires 1 to {MAX_SOURCE_COLUMNS} typed columns"),
            });
        }
        ensure_maximum_len("source-dataset.rows", self.rows.len(), MAX_SOURCE_ROWS)?;
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
                value.validate("source-row.value")?;
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
                    "at most {MAX_VISUALIZATION_DATASETS} datasets and {MAX_SOURCE_CELLS_TOTAL} cells"
                )
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                if sequence
                    .size_hint()
                    .is_some_and(|hint| hint > MAX_VISUALIZATION_DATASETS)
                {
                    return Err(serde::de::Error::custom(format!(
                        "dataset sequence exceeds the {MAX_VISUALIZATION_DATASETS}-dataset resource limit"
                    )));
                }
                let mut datasets = Vec::with_capacity(
                    sequence
                        .size_hint()
                        .unwrap_or(0)
                        .min(MAX_VISUALIZATION_DATASETS),
                );
                let mut cells = 0_usize;
                let mut text_bytes = 0_usize;
                while datasets.len() < MAX_VISUALIZATION_DATASETS {
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
                if sequence.next_element::<IgnoredAny>()?.is_some() {
                    return Err(serde::de::Error::custom(format!(
                        "dataset sequence exceeds the {MAX_VISUALIZATION_DATASETS}-dataset resource limit"
                    )));
                }
                Ok(BoundedSourceDatasets(datasets))
            }
        }

        deserializer.deserialize_seq(SourceDatasetsVisitor)
    }
}

impl VisualizationDocument {
    fn require_page(&self, id: PageId) -> Result<(), VisualizationError> {
        self.pages
            .iter()
            .any(|x| x.id == id)
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Page(id)))
    }

    fn require_pane(&self, id: PaneId) -> Result<(), VisualizationError> {
        self.panes
            .iter()
            .any(|x| x.id == id)
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(id)))
    }

    fn require_axis_in_pane(&self, id: AxisId, pane_id: PaneId) -> Result<(), VisualizationError> {
        self.axes
            .iter()
            .any(|x| x.id == id && x.pane_id == pane_id)
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Axis(id)))
    }

    fn require_trace_in_pane(
        &self,
        id: TraceId,
        pane_id: PaneId,
    ) -> Result<(), VisualizationError> {
        self.traces
            .iter()
            .any(|x| x.id == id && x.pane_id == pane_id)
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Trace(id)))
    }

    fn require_entity(&self, entity: EntityRef) -> Result<(), VisualizationError> {
        let exists = match entity {
            EntityRef::Page(id) => self.pages.iter().any(|x| x.id == id),
            EntityRef::Pane(id) => self.panes.iter().any(|x| x.id == id),
            EntityRef::Trace(id) => self.traces.iter().any(|x| x.id == id),
            EntityRef::Axis(id) => self.axes.iter().any(|x| x.id == id),
            EntityRef::Cursor(id) => self.cursors.iter().any(|x| x.id == id),
            EntityRef::Marker(id) => self.markers.iter().any(|x| x.id == id),
            EntityRef::Measurement(id) => self.measurements.iter().any(|x| x.id == id),
            EntityRef::Annotation(id) => self.annotations.iter().any(|x| x.id == id),
            EntityRef::LinkGroup(id) => self.link_groups.iter().any(|x| x.id == id),
        };
        exists
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(entity))
    }

    fn page_mut(&mut self, id: PageId) -> Result<&mut Page, VisualizationError> {
        self.pages
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Page(id)))
    }

    fn pane_mut(&mut self, id: PaneId) -> Result<&mut Pane, VisualizationError> {
        self.panes
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(id)))
    }

    fn trace_mut(&mut self, id: TraceId) -> Result<&mut Trace, VisualizationError> {
        self.traces
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Trace(id)))
    }

    fn axis_mut(&mut self, id: AxisId) -> Result<&mut Axis, VisualizationError> {
        self.axes
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Axis(id)))
    }

    fn cursor_mut(&mut self, id: CursorId) -> Result<&mut Cursor, VisualizationError> {
        self.cursors
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Cursor(id)))
    }

    fn marker_mut(&mut self, id: MarkerId) -> Result<&mut Marker, VisualizationError> {
        self.markers
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Marker(id)))
    }

    fn measurement_mut(
        &mut self,
        id: MeasurementId,
    ) -> Result<&mut Measurement, VisualizationError> {
        self.measurements
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Measurement(
                id,
            )))
    }

    fn annotation_mut(&mut self, id: AnnotationId) -> Result<&mut Annotation, VisualizationError> {
        self.annotations
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Annotation(
                id,
            )))
    }

    fn link_group_mut(&mut self, id: LinkGroupId) -> Result<&mut LinkGroup, VisualizationError> {
        self.link_groups
            .iter_mut()
            .find(|x| x.id == id)
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

fn default_viewer_id() -> String {
    "viewer-waveform".to_owned()
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
    #[serde(default)]
    pub layout: PageLayout,
    #[serde(default = "default_page_template_id")]
    #[serde(deserialize_with = "deserialize_key_string")]
    pub template_id: String,
    #[serde(default)]
    pub update_policy: PageUpdatePolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pane {
    pub id: PaneId,
    pub page_id: PageId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub kind: PaneKind,
    #[serde(default = "default_viewer_id")]
    #[serde(deserialize_with = "deserialize_key_string")]
    pub viewer_id: String,
    #[serde(default)]
    pub binding: Option<PaneDataBinding>,
    #[serde(default)]
    pub placement: PanePlacement,
    #[serde(default)]
    pub order: u32,
    #[serde(default)]
    pub family_policy: Option<FamilyPresentationPolicy>,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trace {
    pub id: TraceId,
    pub pane_id: PaneId,
    pub binding: DatasetBinding,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub signal_key: String,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub coordinate_key: String,
    pub x_axis_id: AxisId,
    pub y_axis_id: AxisId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    pub visible: bool,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Marker {
    pub id: MarkerId,
    pub pane_id: PaneId,
    pub trace_id: TraceId,
    pub coordinate: TypedValue,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
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
    #[serde(deserialize_with = "deserialize_measurement_trace_ids")]
    pub trace_ids: Vec<TraceId>,
    pub kind: MeasurementKind,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
}

impl NestedResourceCount for Measurement {
    fn nested_resource_count(&self) -> usize {
        self.trace_ids.len()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind", content = "value")]
pub enum AnnotationAnchor {
    Pane {
        x_fraction: f32,
        y_fraction: f32,
    },
    Trace {
        trace_id: TraceId,
        coordinate: TypedValue,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkGroup {
    pub id: LinkGroupId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    pub kind: LinkKind,
    #[serde(deserialize_with = "deserialize_link_members")]
    pub members: Vec<EntityRef>,
}

impl NestedResourceCount for LinkGroup {
    fn nested_resource_count(&self) -> usize {
        self.members.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tombstone {
    pub entity: EntityRef,
    pub deleted_at_revision: ObjectRevision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewAxis {
    pub pane_id: PaneId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    pub orientation: AxisOrientation,
    pub scale: AxisScale,
    #[serde(deserialize_with = "deserialize_unit_string")]
    pub unit: Option<String>,
    pub range: Option<AxisRange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewTrace {
    pub pane_id: PaneId,
    pub binding: DatasetBinding,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub signal_key: String,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub coordinate_key: String,
    pub x_axis_id: AxisId,
    pub y_axis_id: AxisId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewPage {
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub layout: PageLayout,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub template_id: String,
    pub update_policy: PageUpdatePolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewPane {
    pub page_id: PageId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub kind: PaneKind,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub viewer_id: String,
    pub binding: Option<PaneDataBinding>,
    pub placement: PanePlacement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewPagePane {
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub kind: PaneKind,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub viewer_id: String,
    pub binding: Option<PaneDataBinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentEdit {
    AttachDataset(SourceDataset),
    AddPage {
        title: String,
    },
    AddComposedPage(NewPage),
    AddPane {
        page_id: PageId,
        title: String,
        kind: PaneKind,
    },
    AddBoundPane(NewPane),
    AddPaneOnNewPage {
        page: NewPage,
        pane: NewPagePane,
    },
    SetPageComposition {
        page_id: PageId,
        layout: PageLayout,
        template_id: String,
        update_policy: PageUpdatePolicy,
    },
    SetPaneSource {
        pane_id: PaneId,
        viewer_id: String,
        binding: Option<PaneDataBinding>,
    },
    SetPaneFamilyPresentation {
        pane_id: PaneId,
        policy: Option<FamilyPresentationPolicy>,
    },
    PlacePane {
        pane_id: PaneId,
        page_id: PageId,
        placement: PanePlacement,
    },
    AddAxis(NewAxis),
    AddTrace(NewTrace),
    AddCursor {
        pane_id: PaneId,
        axis_id: AxisId,
        position: TypedValue,
        label: String,
    },
    AddMarker {
        pane_id: PaneId,
        trace_id: TraceId,
        coordinate: TypedValue,
        label: String,
    },
    AddMeasurement {
        pane_id: PaneId,
        trace_ids: Vec<TraceId>,
        kind: MeasurementKind,
        label: String,
    },
    AddAnnotation {
        pane_id: PaneId,
        anchor: AnnotationAnchor,
        text: String,
    },
    AddLinkGroup {
        label: String,
        kind: LinkKind,
        members: Vec<EntityRef>,
    },
    Rename {
        entity: EntityRef,
        value: String,
    },
    SetAxisRange {
        axis_id: AxisId,
        range: Option<AxisRange>,
    },
    SetTraceVisibility {
        trace_id: TraceId,
        visible: bool,
    },
    MoveCursor {
        cursor_id: CursorId,
        position: TypedValue,
    },
    MoveMarker {
        marker_id: MarkerId,
        coordinate: TypedValue,
    },
    SetAnnotation {
        annotation_id: AnnotationId,
        anchor: AnnotationAnchor,
        text: String,
    },
    SetLinkMembers {
        link_group_id: LinkGroupId,
        members: Vec<EntityRef>,
    },
    Remove(EntityRef),
    RecordComparison(ComparisonReceipt),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RowAlignmentPolicy {
    RequireIdentical,
    ExactIntersection,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NumericTolerance {
    pub absolute: f64,
    pub relative: f64,
}

impl NumericTolerance {
    pub fn new(absolute: f64, relative: f64) -> Result<Self, VisualizationError> {
        if !absolute.is_finite() || absolute < 0.0 || !relative.is_finite() || relative < 0.0 {
            return Err(VisualizationError::InvalidValue {
                field: "comparison.tolerance",
                message: "absolute and relative tolerances must be finite and non-negative"
                    .to_owned(),
            });
        }
        Ok(Self { absolute, relative })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ComparisonPolicy {
    pub row_alignment: RowAlignmentPolicy,
    pub tolerance: NumericTolerance,
    pub require_identical_units: bool,
    #[serde(default)]
    pub execution: ComparisonExecutionContract,
}

/// Fully declared numerical behavior for the currently implemented exact
/// comparison engine. New algorithms must add explicit variants rather than
/// silently changing alignment or interpolation semantics.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComparisonExecutionContract {
    #[serde(default)]
    pub alignment: ComparisonAlignmentMethod,
    #[serde(default)]
    pub interpolation: ComparisonInterpolationPolicy,
    #[serde(default)]
    pub resampling: ComparisonResamplingPolicy,
    #[serde(default)]
    pub extrapolation: ComparisonExtrapolationPolicy,
    #[serde(default)]
    pub precision: ComparisonPrecisionPolicy,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonAlignmentMethod {
    #[default]
    ExactCoordinateRows,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonInterpolationPolicy {
    #[default]
    NoneExactOnly,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonResamplingPolicy {
    #[default]
    NoneRetainSourceGrid,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonExtrapolationPolicy {
    #[default]
    Forbid,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonPrecisionPolicy {
    #[default]
    SourceF64NoRounding,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonRequest {
    pub baseline: DatasetBinding,
    pub candidate: DatasetBinding,
    #[serde(deserialize_with = "deserialize_comparison_signal_keys")]
    pub signal_keys: Vec<String>,
    pub policy: ComparisonPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonDisposition {
    Passed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalComparison {
    #[serde(deserialize_with = "deserialize_key_string")]
    pub signal_key: String,
    pub compared_rows: usize,
    pub failed_rows: usize,
    pub maximum_absolute_error: f64,
    pub maximum_relative_error: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonReceipt {
    pub baseline: DatasetBinding,
    pub candidate: DatasetBinding,
    pub policy: ComparisonPolicy,
    pub rows_compared: usize,
    #[serde(deserialize_with = "deserialize_comparison_signals")]
    pub signals: Vec<SignalComparison>,
    pub disposition: ComparisonDisposition,
}

impl NestedResourceCount for ComparisonReceipt {
    fn nested_resource_count(&self) -> usize {
        self.signals.len()
    }
}

impl ComparisonReceipt {
    pub(crate) fn validate_structure(&self) -> Result<(), VisualizationError> {
        self.policy.tolerance.validate()?;
        if self.baseline.dataset_id == self.candidate.dataset_id {
            return Err(VisualizationError::InvalidValue {
                field: "comparison-receipt.datasets",
                message: "baseline and candidate must be distinct immutable datasets".to_owned(),
            });
        }
        if self.rows_compared == 0
            || self.signals.is_empty()
            || self.signals.len() > MAX_COMPARISON_SIGNALS
        {
            return Err(VisualizationError::InvalidValue {
                field: "comparison-receipt",
                message: format!(
                    "a receipt must contain compared rows and 1 to {MAX_COMPARISON_SIGNALS} signals"
                ),
            });
        }
        let mut signal_keys = HashSet::new();
        for signal in &self.signals {
            validate_key("comparison-receipt.signal-key", &signal.signal_key)?;
            if !signal_keys.insert(signal.signal_key.as_str()) {
                return Err(VisualizationError::DuplicateKey(signal.signal_key.clone()));
            }
            if signal.compared_rows != self.rows_compared
                || signal.failed_rows > signal.compared_rows
                || !signal.maximum_absolute_error.is_finite()
                || signal.maximum_absolute_error < 0.0
                || !signal.maximum_relative_error.is_finite()
                || signal.maximum_relative_error < 0.0
            {
                return Err(VisualizationError::InvalidValue {
                    field: "comparison-receipt.signal",
                    message: "signal row counts and maximum errors must agree with the receipt"
                        .to_owned(),
                });
            }
        }
        let expected = if self.signals.iter().any(|signal| signal.failed_rows > 0) {
            ComparisonDisposition::Failed
        } else {
            ComparisonDisposition::Passed
        };
        if self.disposition != expected {
            return Err(VisualizationError::InvalidValue {
                field: "comparison-receipt.disposition",
                message: "disposition does not agree with signal outcomes".to_owned(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProgressiveOperationKind {
    Export,
    Transform,
    Comparison,
    MeasurementEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "state")]
pub enum ProgressiveOperationState {
    Running,
    Completed { output_digest: ContentDigest },
    Cancelling,
    Cancelled,
    Failed { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressiveOperation {
    id: OperationId,
    kind: ProgressiveOperationKind,
    document_id: ResultDocumentId,
    source_revision: ObjectRevision,
    total_units: u64,
    completed_units: u64,
    checkpoint_digest: Option<ContentDigest>,
    recovery_count: u32,
    state: ProgressiveOperationState,
}

impl ProgressiveOperation {
    pub fn start(
        id: OperationId,
        kind: ProgressiveOperationKind,
        document_id: ResultDocumentId,
        source_revision: ObjectRevision,
        total_units: u64,
    ) -> Result<Self, VisualizationError> {
        if total_units == 0 {
            return Err(VisualizationError::InvalidValue {
                field: "operation.total-units",
                message: "total units must be greater than zero".to_owned(),
            });
        }
        Ok(Self {
            id,
            kind,
            document_id,
            source_revision,
            total_units,
            completed_units: 0,
            checkpoint_digest: None,
            recovery_count: 0,
            state: ProgressiveOperationState::Running,
        })
    }

    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    #[must_use]
    pub const fn completed_units(&self) -> u64 {
        self.completed_units
    }

    #[must_use]
    pub const fn total_units(&self) -> u64 {
        self.total_units
    }

    #[must_use]
    pub const fn state(&self) -> &ProgressiveOperationState {
        &self.state
    }

    #[must_use]
    pub const fn checkpoint_digest(&self) -> Option<ContentDigest> {
        self.checkpoint_digest
    }

    #[must_use]
    pub const fn recovery_count(&self) -> u32 {
        self.recovery_count
    }

    pub fn advance(
        &mut self,
        completed_units: u64,
        checkpoint_digest: ContentDigest,
        completed_output: Option<ContentDigest>,
    ) -> Result<(), VisualizationError> {
        if self.state != ProgressiveOperationState::Running {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "advance",
            });
        }
        if completed_units <= self.completed_units || completed_units > self.total_units {
            return Err(VisualizationError::InvalidProgress {
                previous: self.completed_units,
                next: completed_units,
                total: self.total_units,
            });
        }
        if completed_units == self.total_units {
            let output_digest = completed_output.ok_or(VisualizationError::MissingOutputDigest)?;
            self.completed_units = completed_units;
            self.checkpoint_digest = Some(checkpoint_digest);
            self.state = ProgressiveOperationState::Completed { output_digest };
        } else {
            if completed_output.is_some() {
                return Err(VisualizationError::UnexpectedOutputDigest);
            }
            self.completed_units = completed_units;
            self.checkpoint_digest = Some(checkpoint_digest);
        }
        Ok(())
    }

    pub fn request_cancel(&mut self) -> Result<(), VisualizationError> {
        if self.state != ProgressiveOperationState::Running {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "request-cancel",
            });
        }
        self.state = ProgressiveOperationState::Cancelling;
        Ok(())
    }

    pub fn confirm_cancelled(&mut self) -> Result<(), VisualizationError> {
        if self.state != ProgressiveOperationState::Cancelling {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "confirm-cancelled",
            });
        }
        self.state = ProgressiveOperationState::Cancelled;
        Ok(())
    }

    pub fn fail(
        &mut self,
        message: impl Into<String>,
        checkpoint_digest: Option<ContentDigest>,
    ) -> Result<(), VisualizationError> {
        if !matches!(
            &self.state,
            ProgressiveOperationState::Running | ProgressiveOperationState::Cancelling
        ) {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "fail",
            });
        }
        let message = message.into();
        validate_label("operation.failure", &message)?;
        if checkpoint_digest.is_some() {
            self.checkpoint_digest = checkpoint_digest;
        }
        self.state = ProgressiveOperationState::Failed { message };
        Ok(())
    }

    pub fn recover(&mut self) -> Result<(), VisualizationError> {
        if !matches!(
            &self.state,
            ProgressiveOperationState::Cancelled | ProgressiveOperationState::Failed { .. }
        ) {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "recover",
            });
        }
        self.recovery_count = self
            .recovery_count
            .checked_add(1)
            .ok_or(VisualizationError::RecoverySpaceExhausted)?;
        self.state = ProgressiveOperationState::Running;
        Ok(())
    }

    fn state_label(&self) -> &'static str {
        match &self.state {
            ProgressiveOperationState::Running => "running",
            ProgressiveOperationState::Completed { .. } => "completed",
            ProgressiveOperationState::Cancelling => "cancelling",
            ProgressiveOperationState::Cancelled => "cancelled",
            ProgressiveOperationState::Failed { .. } => "failed",
        }
    }
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
    #[error("query must specify every coordinate column exactly once")]
    IncompleteCoordinateQuery,
    #[error("query has no exact source row; interpolation would be required")]
    InterpolationRequired,
    #[error("query has no matching source row")]
    RowNotFound,
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
    #[error("operation cannot {event} from {from}")]
    InvalidOperationTransition {
        from: &'static str,
        event: &'static str,
    },
    #[error(
        "operation progress must increase from {previous}, not exceed total {total}; received {next}"
    )]
    InvalidProgress {
        previous: u64,
        next: u64,
        total: u64,
    },
    #[error("a completed operation requires an output digest")]
    MissingOutputDigest,
    #[error("an output digest is only valid on the final progress update")]
    UnexpectedOutputDigest,
    #[error("operation recovery counter is exhausted")]
    RecoverySpaceExhausted,
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
    next_serial: u64,
    datasets: Vec<SourceDataset>,
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
}

#[derive(Deserialize)]
struct VisualizationDocumentWire {
    #[serde(default = "legacy_visualization_schema_version")]
    schema_version: u16,
    id: ResultDocumentId,
    revision: ObjectRevision,
    #[serde(deserialize_with = "deserialize_label_string")]
    title: String,
    next_serial: u64,
    datasets: BoundedSourceDatasets,
    pages: BoundedVec<Page, MAX_VISUALIZATION_PAGES>,
    panes: BoundedVec<Pane, MAX_VISUALIZATION_PANES>,
    axes: BoundedVec<Axis, MAX_VISUALIZATION_AXES>,
    traces: BoundedVec<Trace, MAX_VISUALIZATION_TRACES>,
    cursors: BoundedVec<Cursor, MAX_VISUALIZATION_CURSORS>,
    markers: BoundedVec<Marker, MAX_VISUALIZATION_MARKERS>,
    measurements: AggregateBoundedVec<
        Measurement,
        MAX_VISUALIZATION_MEASUREMENTS,
        MAX_VISUALIZATION_MEASUREMENT_TRACE_REFERENCES_TOTAL,
    >,
    annotations: BoundedVec<Annotation, MAX_VISUALIZATION_ANNOTATIONS>,
    link_groups: AggregateBoundedVec<
        LinkGroup,
        MAX_VISUALIZATION_LINK_GROUPS,
        MAX_VISUALIZATION_LINK_MEMBER_REFERENCES_TOTAL,
    >,
    tombstones: BoundedVec<Tombstone, MAX_VISUALIZATION_TOMBSTONES>,
    comparisons: AggregateBoundedVec<
        ComparisonReceipt,
        MAX_VISUALIZATION_COMPARISONS,
        MAX_VISUALIZATION_COMPARISON_SIGNALS_TOTAL,
    >,
}

const fn legacy_visualization_schema_version() -> u16 {
    1
}

impl<'de> Deserialize<'de> for VisualizationDocument {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = VisualizationDocumentWire::deserialize(deserializer)?;
        let mut document = Self {
            schema_version: wire.schema_version,
            id: wire.id,
            revision: wire.revision,
            title: wire.title,
            next_serial: wire.next_serial,
            datasets: wire.datasets.0,
            pages: wire.pages.into_inner(),
            panes: wire.panes.into_inner(),
            axes: wire.axes.into_inner(),
            traces: wire.traces.into_inner(),
            cursors: wire.cursors.into_inner(),
            markers: wire.markers.into_inner(),
            measurements: wire.measurements.into_inner(),
            annotations: wire.annotations.into_inner(),
            link_groups: wire.link_groups.into_inner(),
            tombstones: wire.tombstones.into_inner(),
            comparisons: wire.comparisons.into_inner(),
        };
        match document.schema_version {
            1 => {
                document.migrate_v1_to_v2();
                document.migrate_v2_to_v3();
            }
            2 => document.migrate_v2_to_v3(),
            Self::SCHEMA_VERSION => {}
            version => {
                return Err(serde::de::Error::custom(format!(
                    "unsupported visualization document schema version {version}"
                )));
            }
        }
        document.validate().map_err(serde::de::Error::custom)?;
        Ok(document)
    }
}

impl VisualizationDocument {
    /// Resource hardening does not change the schema-v3 wire vocabulary.
    ///
    /// V1, V2, and V3 documents within the published resource limits retain
    /// their prior deterministic interpretation. Inputs above those limits
    /// are rejected as unsafe containers; the limits do not reinterpret or
    /// migrate any accepted source value, identity, or presentation entity.
    /// Unknown extension fields remain ignored for schema-v3 forward
    /// compatibility; introducing required semantics still requires a schema
    /// revision.
    pub const SCHEMA_VERSION: u16 = 3;

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
            next_serial: 3,
            datasets,
            pages: vec![Page {
                id: page_id,
                title: "Page 1".to_owned(),
                layout: PageLayout::default(),
                template_id: default_page_template_id(),
                update_policy: PageUpdatePolicy::default(),
            }],
            panes: vec![Pane {
                id: pane_id,
                page_id,
                title: "Plot 1".to_owned(),
                kind: PaneKind::Cartesian,
                viewer_id: default_viewer_id(),
                binding: None,
                placement: PanePlacement::Primary,
                order: 0,
                family_policy: None,
            }],
            axes: Vec::new(),
            traces: Vec::new(),
            cursors: Vec::new(),
            markers: Vec::new(),
            measurements: Vec::new(),
            annotations: Vec::new(),
            link_groups: Vec::new(),
            tombstones: Vec::new(),
            comparisons: Vec::new(),
        };
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

    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
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

    #[must_use]
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }

    #[must_use]
    pub fn link_groups(&self) -> &[LinkGroup] {
        &self.link_groups
    }

    #[must_use]
    pub fn tombstones(&self) -> &[Tombstone] {
        &self.tombstones
    }

    #[must_use]
    pub fn comparisons(&self) -> &[ComparisonReceipt] {
        &self.comparisons
    }

    /// Authenticates the complete validated document envelope.
    ///
    /// The digest includes schema, stable identity, revision, immutable source
    /// datasets, presentation entities, and comparison receipts. It is the
    /// publication boundary for references that must identify one exact
    /// visualization revision rather than merely the underlying dataset.
    pub fn content_digest(&self) -> Result<ContentDigest, VisualizationError> {
        self.validate()?;
        let mut writer = Sha256Writer::default();
        serde_json::to_writer(&mut writer, self)
            .map_err(|error| VisualizationError::Serialization(error.to_string()))?;
        Ok(writer.finish())
    }

    /// Applies all edits to a candidate clone, validates the complete graph,
    /// and commits once. Any error leaves `self` byte-for-byte unchanged.
    pub fn transact(
        &mut self,
        expected_revision: ObjectRevision,
        edits: Vec<DocumentEdit>,
    ) -> Result<VisualizationTransactionReceipt, VisualizationError> {
        if expected_revision != self.revision {
            return Err(VisualizationError::RevisionConflict {
                expected: expected_revision,
                actual: self.revision,
            });
        }
        if edits.is_empty() || edits.len() > MAX_VISUALIZATION_TRANSACTION_EDITS {
            return Err(VisualizationError::InvalidValue {
                field: "transaction.edits",
                message: format!(
                    "a transaction must contain 1 to {MAX_VISUALIZATION_TRANSACTION_EDITS} edits"
                ),
            });
        }
        let previous_revision = self.revision;
        let committed_revision = previous_revision.next()?;
        let mut candidate = self.clone();
        let mut created = Vec::new();
        let mut tombstoned = Vec::new();
        for edit in edits.iter().cloned() {
            candidate.apply_edit(edit, committed_revision, &mut created, &mut tombstoned)?;
        }
        candidate.revision = committed_revision;
        candidate.validate()?;
        *self = candidate;
        Ok(VisualizationTransactionReceipt {
            document_id: self.id,
            previous_revision,
            committed_revision,
            created,
            tombstoned,
            edit_count: edits.len(),
        })
    }

    /// Reserves a stable operation identity and advances the document revision
    /// so persisted identity allocation can never be replayed or reused.
    pub fn start_operation(
        &mut self,
        expected_revision: ObjectRevision,
        kind: ProgressiveOperationKind,
        total_units: u64,
    ) -> Result<(VisualizationTransactionReceipt, ProgressiveOperation), VisualizationError> {
        if expected_revision != self.revision {
            return Err(VisualizationError::RevisionConflict {
                expected: expected_revision,
                actual: self.revision,
            });
        }
        let previous_revision = self.revision;
        let committed_revision = previous_revision.next()?;
        let mut candidate = self.clone();
        let operation_id = OperationId::allocate(candidate.allocate_serial()?)?;
        let operation = ProgressiveOperation::start(
            operation_id,
            kind,
            self.id,
            previous_revision,
            total_units,
        )?;
        candidate.revision = committed_revision;
        *self = candidate;
        Ok((
            VisualizationTransactionReceipt {
                document_id: self.id,
                previous_revision,
                committed_revision,
                created: Vec::new(),
                tombstoned: Vec::new(),
                edit_count: 1,
            },
            operation,
        ))
    }

    pub fn query_exact_row(&self, query: &ExactRowQuery) -> Result<ExactRow, VisualizationError> {
        let dataset = self.dataset_for_binding(query.binding)?;
        query_dataset_exact(dataset, query)
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

    fn migrate_v1_to_v2(&mut self) {
        for page in &mut self.pages {
            page.layout = PageLayout::default();
            page.template_id = default_page_template_id();
            page.update_policy = PageUpdatePolicy::default();
        }
        let mut previous_by_page = HashMap::<PageId, PaneId>::new();
        let mut next_order_by_page = HashMap::<PageId, u32>::new();
        for pane in &mut self.panes {
            pane.viewer_id = default_viewer_id_for_kind(pane.kind).to_owned();
            pane.binding = None;
            pane.order = *next_order_by_page.entry(pane.page_id).or_default();
            pane.placement = previous_by_page
                .get(&pane.page_id)
                .copied()
                .map_or(PanePlacement::Primary, |anchor_pane_id| {
                    PanePlacement::Below { anchor_pane_id }
                });
            previous_by_page.insert(pane.page_id, pane.id);
            next_order_by_page
                .entry(pane.page_id)
                .and_modify(|order| *order = order.saturating_add(1));
        }
        self.schema_version = 2;
    }

    fn migrate_v2_to_v3(&mut self) {
        // Schema 2 had no family policy. Ignore any forward field smuggled into
        // an older envelope so migration has one deterministic interpretation.
        for pane in &mut self.panes {
            pane.family_policy = None;
        }
        self.schema_version = Self::SCHEMA_VERSION;
    }

    fn validate_page_definition(page: &NewPage) -> Result<(), VisualizationError> {
        validate_label("page.title", &page.title)?;
        page.layout.validate()?;
        validate_key("page.template-id", &page.template_id)
    }

    fn validate_pane_source(
        &self,
        kind: PaneKind,
        viewer_id: &str,
        binding: Option<PaneDataBinding>,
    ) -> Result<(), VisualizationError> {
        validate_key("pane.viewer-id", viewer_id)?;
        let viewer =
            viewer_document(viewer_id).ok_or_else(|| VisualizationError::InvalidValue {
                field: "pane.viewer-id",
                message: format!("unknown viewer document '{viewer_id}'"),
            })?;
        let expected_kind = pane_kind_for_viewer_art(viewer.art);
        if kind != expected_kind {
            return Err(VisualizationError::InvalidValue {
                field: "pane.kind",
                message: format!(
                    "viewer '{viewer_id}' requires pane kind {expected_kind:?}, received {kind:?}"
                ),
            });
        }
        if let Some(binding) = binding {
            self.dataset_for_binding(binding.dataset)?;
        }
        Ok(())
    }

    fn validate_pane_family_policy(
        &self,
        pane_id: PaneId,
        policy: &FamilyPresentationPolicy,
    ) -> Result<(), VisualizationError> {
        policy.validate()?;
        let pane = self
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
        let mut bindings = Vec::<DatasetBinding>::new();
        if let Some(binding) = pane.binding.map(|binding| binding.dataset) {
            bindings.push(binding);
        }
        for binding in self
            .traces
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .map(|trace| trace.binding)
        {
            if !bindings.contains(&binding) {
                bindings.push(binding);
            }
        }
        if bindings.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "pane.family-policy",
                message: "a family policy requires at least one immutable pane or trace source"
                    .to_owned(),
            });
        }
        for binding in bindings {
            let dataset = self.dataset_for_binding(binding)?;
            for dimension in std::iter::once(&policy.x_dimension.dimension)
                .chain(policy.family_dimensions.iter())
            {
                let column = dataset
                    .columns
                    .iter()
                    .find(|column| column.key == dimension.key)
                    .ok_or_else(|| VisualizationError::InvalidValue {
                        field: "pane.family-policy.dimension",
                        message: format!(
                            "source dataset {} has no declared dimension '{}'",
                            binding.dataset_id, dimension.key
                        ),
                    })?;
                if column.role != ColumnRole::Coordinate
                    || column.value_type != dimension.value_type
                {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.family-policy.dimension",
                        message: format!(
                            "source dimension '{}' must be a {:?} coordinate, received {:?} {:?}",
                            dimension.key, dimension.value_type, column.value_type, column.role
                        ),
                    });
                }
            }
        }
        Ok(())
    }

    fn create_page(&mut self, page: NewPage) -> Result<PageId, VisualizationError> {
        Self::validate_page_definition(&page)?;
        let id = PageId::allocate(self.allocate_serial()?)?;
        self.pages.push(Page {
            id,
            title: page.title,
            layout: page.layout,
            template_id: page.template_id,
            update_policy: page.update_policy,
        });
        Ok(id)
    }

    fn placement_order(
        &self,
        page_id: PageId,
        placement: PanePlacement,
        excluded: Option<PaneId>,
    ) -> Result<u32, VisualizationError> {
        self.require_page(page_id)?;
        let pane_count = self
            .panes
            .iter()
            .filter(|pane| pane.page_id == page_id && Some(pane.id) != excluded)
            .count();
        match placement {
            PanePlacement::Primary if pane_count == 0 => Ok(0),
            PanePlacement::Primary => Err(VisualizationError::InvalidValue {
                field: "pane.placement",
                message: "primary placement requires an empty page".to_owned(),
            }),
            PanePlacement::Below { anchor_pane_id } | PanePlacement::RightOf { anchor_pane_id } => {
                if Some(anchor_pane_id) == excluded {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.placement",
                        message: "a pane cannot be placed relative to itself".to_owned(),
                    });
                }
                let anchor = self
                    .panes
                    .iter()
                    .find(|pane| pane.id == anchor_pane_id && Some(pane.id) != excluded)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(
                        anchor_pane_id,
                    )))?;
                if anchor.page_id != page_id {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.placement",
                        message: "placement anchor must belong to the destination page".to_owned(),
                    });
                }
                anchor
                    .order
                    .checked_add(1)
                    .ok_or(VisualizationError::IdentitySpaceExhausted)
            }
        }
    }

    fn create_pane(&mut self, pane: NewPane) -> Result<PaneId, VisualizationError> {
        validate_label("pane.title", &pane.title)?;
        self.validate_pane_source(pane.kind, &pane.viewer_id, pane.binding)?;
        let order = self.placement_order(pane.page_id, pane.placement, None)?;
        for existing in &mut self.panes {
            if existing.page_id == pane.page_id && existing.order >= order {
                existing.order = existing
                    .order
                    .checked_add(1)
                    .ok_or(VisualizationError::IdentitySpaceExhausted)?;
            }
        }
        let id = PaneId::allocate(self.allocate_serial()?)?;
        self.panes.push(Pane {
            id,
            page_id: pane.page_id,
            title: pane.title,
            kind: pane.kind,
            viewer_id: pane.viewer_id,
            binding: pane.binding,
            placement: pane.placement,
            order,
            family_policy: None,
        });
        Ok(id)
    }

    fn default_append_placement(&self, page_id: PageId) -> PanePlacement {
        self.panes
            .iter()
            .filter(|pane| pane.page_id == page_id)
            .max_by_key(|pane| pane.order)
            .map_or(PanePlacement::Primary, |pane| PanePlacement::Below {
                anchor_pane_id: pane.id,
            })
    }

    fn normalize_page_pane_layout(&mut self, page_id: PageId, excluded: Option<PaneId>) {
        let mut ordered: Vec<_> = self
            .panes
            .iter()
            .filter(|pane| pane.page_id == page_id && Some(pane.id) != excluded)
            .map(|pane| (pane.id, pane.order, pane.placement))
            .collect();
        ordered.sort_by_key(|(id, order, _)| (*order, *id));
        let mut preceding = HashSet::new();
        let mut previous = None;
        for (order, (pane_id, _, placement)) in ordered.into_iter().enumerate() {
            let normalized_placement = if order == 0 {
                PanePlacement::Primary
            } else {
                match placement {
                    PanePlacement::Below { anchor_pane_id }
                        if preceding.contains(&anchor_pane_id) =>
                    {
                        placement
                    }
                    PanePlacement::RightOf { anchor_pane_id }
                        if preceding.contains(&anchor_pane_id) =>
                    {
                        placement
                    }
                    _ => PanePlacement::Below {
                        anchor_pane_id: previous.expect("a non-primary pane has a predecessor"),
                    },
                }
            };
            let pane = self
                .panes
                .iter_mut()
                .find(|pane| pane.id == pane_id)
                .expect("pane identity was projected from this document");
            pane.order = u32::try_from(order).expect("pane count exceeds u32 address space");
            pane.placement = normalized_placement;
            preceding.insert(pane_id);
            previous = Some(pane_id);
        }
    }

    fn place_pane(
        &mut self,
        pane_id: PaneId,
        page_id: PageId,
        placement: PanePlacement,
    ) -> Result<(), VisualizationError> {
        let source_page_id = self
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| pane.page_id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
        self.require_page(page_id)?;
        self.normalize_page_pane_layout(source_page_id, Some(pane_id));
        let order = self.placement_order(page_id, placement, Some(pane_id))?;
        for pane in &mut self.panes {
            if pane.id != pane_id && pane.page_id == page_id && pane.order >= order {
                pane.order = pane
                    .order
                    .checked_add(1)
                    .ok_or(VisualizationError::IdentitySpaceExhausted)?;
            }
        }
        let pane = self.pane_mut(pane_id)?;
        pane.page_id = page_id;
        pane.placement = placement;
        pane.order = order;
        Ok(())
    }

    fn apply_edit(
        &mut self,
        edit: DocumentEdit,
        revision: ObjectRevision,
        created: &mut Vec<EntityRef>,
        tombstoned: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        match edit {
            DocumentEdit::AttachDataset(dataset) => self.attach_dataset(dataset),
            DocumentEdit::AddPage { title } => {
                let id = self.create_page(NewPage {
                    title,
                    layout: PageLayout::default(),
                    template_id: default_page_template_id(),
                    update_policy: PageUpdatePolicy::default(),
                })?;
                created.push(EntityRef::Page(id));
                Ok(())
            }
            DocumentEdit::AddComposedPage(page) => {
                let id = self.create_page(page)?;
                created.push(EntityRef::Page(id));
                Ok(())
            }
            DocumentEdit::AddPane {
                page_id,
                title,
                kind,
            } => {
                let placement = self.default_append_placement(page_id);
                let id = self.create_pane(NewPane {
                    page_id,
                    title,
                    kind,
                    viewer_id: default_viewer_id_for_kind(kind).to_owned(),
                    binding: None,
                    placement,
                })?;
                created.push(EntityRef::Pane(id));
                Ok(())
            }
            DocumentEdit::AddBoundPane(pane) => {
                let id = self.create_pane(pane)?;
                created.push(EntityRef::Pane(id));
                Ok(())
            }
            DocumentEdit::AddPaneOnNewPage { page, pane } => {
                let page_id = self.create_page(page)?;
                created.push(EntityRef::Page(page_id));
                let pane_id = self.create_pane(NewPane {
                    page_id,
                    title: pane.title,
                    kind: pane.kind,
                    viewer_id: pane.viewer_id,
                    binding: pane.binding,
                    placement: PanePlacement::Primary,
                })?;
                created.push(EntityRef::Pane(pane_id));
                Ok(())
            }
            DocumentEdit::SetPageComposition {
                page_id,
                layout,
                template_id,
                update_policy,
            } => {
                Self::validate_page_definition(&NewPage {
                    title: self.page_mut(page_id)?.title.clone(),
                    layout,
                    template_id: template_id.clone(),
                    update_policy,
                })?;
                let page = self.page_mut(page_id)?;
                page.layout = layout;
                page.template_id = template_id;
                page.update_policy = update_policy;
                Ok(())
            }
            DocumentEdit::SetPaneSource {
                pane_id,
                viewer_id,
                binding,
            } => {
                let kind = self
                    .panes
                    .iter()
                    .find(|pane| pane.id == pane_id)
                    .map(|pane| pane.kind)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
                self.validate_pane_source(kind, &viewer_id, binding)?;
                let pane = self.pane_mut(pane_id)?;
                pane.viewer_id = viewer_id;
                pane.binding = binding;
                Ok(())
            }
            DocumentEdit::SetPaneFamilyPresentation { pane_id, policy } => {
                if let Some(policy) = &policy {
                    self.validate_pane_family_policy(pane_id, policy)?;
                } else {
                    self.require_pane(pane_id)?;
                }
                self.pane_mut(pane_id)?.family_policy = policy;
                Ok(())
            }
            DocumentEdit::PlacePane {
                pane_id,
                page_id,
                placement,
            } => self.place_pane(pane_id, page_id, placement),
            DocumentEdit::AddAxis(axis) => {
                self.require_pane(axis.pane_id)?;
                validate_label("axis.label", &axis.label)?;
                validate_optional_unit("axis.unit", axis.unit.as_deref())?;
                let id = AxisId::allocate(self.allocate_serial()?)?;
                self.axes.push(Axis {
                    id,
                    pane_id: axis.pane_id,
                    label: axis.label,
                    orientation: axis.orientation,
                    scale: axis.scale,
                    unit: axis.unit,
                    range: axis.range,
                });
                created.push(EntityRef::Axis(id));
                Ok(())
            }
            DocumentEdit::AddTrace(trace) => self.add_trace(trace, created),
            DocumentEdit::AddCursor {
                pane_id,
                axis_id,
                position,
                label,
            } => {
                validate_label("cursor.label", &label)?;
                position.validate("cursor.position")?;
                self.require_axis_in_pane(axis_id, pane_id)?;
                let id = CursorId::allocate(self.allocate_serial()?)?;
                self.cursors.push(Cursor {
                    id,
                    pane_id,
                    axis_id,
                    position,
                    label,
                });
                created.push(EntityRef::Cursor(id));
                Ok(())
            }
            DocumentEdit::AddMarker {
                pane_id,
                trace_id,
                coordinate,
                label,
            } => {
                validate_label("marker.label", &label)?;
                coordinate.validate("marker.coordinate")?;
                self.require_trace_in_pane(trace_id, pane_id)?;
                let id = MarkerId::allocate(self.allocate_serial()?)?;
                self.markers.push(Marker {
                    id,
                    pane_id,
                    trace_id,
                    coordinate,
                    label,
                });
                created.push(EntityRef::Marker(id));
                Ok(())
            }
            DocumentEdit::AddMeasurement {
                pane_id,
                trace_ids,
                kind,
                label,
            } => {
                validate_label("measurement.label", &label)?;
                if trace_ids.is_empty() || trace_ids.len() > MAX_ENTITY_REFERENCES {
                    return Err(VisualizationError::InvalidValue {
                        field: "measurement.traces",
                        message: format!(
                            "a measurement requires 1 to {MAX_ENTITY_REFERENCES} traces"
                        ),
                    });
                }
                for trace_id in &trace_ids {
                    self.require_trace_in_pane(*trace_id, pane_id)?;
                }
                let id = MeasurementId::allocate(self.allocate_serial()?)?;
                self.measurements.push(Measurement {
                    id,
                    pane_id,
                    trace_ids,
                    kind,
                    label,
                });
                created.push(EntityRef::Measurement(id));
                Ok(())
            }
            DocumentEdit::AddAnnotation {
                pane_id,
                anchor,
                text,
            } => {
                self.require_pane(pane_id)?;
                validate_annotation(self, pane_id, &anchor, &text)?;
                let id = AnnotationId::allocate(self.allocate_serial()?)?;
                self.annotations.push(Annotation {
                    id,
                    pane_id,
                    anchor,
                    text,
                });
                created.push(EntityRef::Annotation(id));
                Ok(())
            }
            DocumentEdit::AddLinkGroup {
                label,
                kind,
                members,
            } => {
                validate_label("link-group.label", &label)?;
                validate_link_members(self, kind, &members)?;
                let id = LinkGroupId::allocate(self.allocate_serial()?)?;
                self.link_groups.push(LinkGroup {
                    id,
                    label,
                    kind,
                    members,
                });
                created.push(EntityRef::LinkGroup(id));
                Ok(())
            }
            DocumentEdit::Rename { entity, value } => self.rename(entity, value),
            DocumentEdit::SetAxisRange { axis_id, range } => {
                self.axis_mut(axis_id)?.range = range;
                Ok(())
            }
            DocumentEdit::SetTraceVisibility { trace_id, visible } => {
                self.trace_mut(trace_id)?.visible = visible;
                Ok(())
            }
            DocumentEdit::MoveCursor {
                cursor_id,
                position,
            } => {
                position.validate("cursor.position")?;
                self.cursor_mut(cursor_id)?.position = position;
                Ok(())
            }
            DocumentEdit::MoveMarker {
                marker_id,
                coordinate,
            } => {
                coordinate.validate("marker.coordinate")?;
                self.marker_mut(marker_id)?.coordinate = coordinate;
                Ok(())
            }
            DocumentEdit::SetAnnotation {
                annotation_id,
                anchor,
                text,
            } => {
                let pane_id = self
                    .annotations
                    .iter()
                    .find(|annotation| annotation.id == annotation_id)
                    .map(|annotation| annotation.pane_id)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::Annotation(
                        annotation_id,
                    )))?;
                validate_annotation(self, pane_id, &anchor, &text)?;
                let annotation = self.annotation_mut(annotation_id)?;
                annotation.anchor = anchor;
                annotation.text = text;
                Ok(())
            }
            DocumentEdit::SetLinkMembers {
                link_group_id,
                members,
            } => {
                let kind = self
                    .link_groups
                    .iter()
                    .find(|group| group.id == link_group_id)
                    .map(|group| group.kind)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::LinkGroup(
                        link_group_id,
                    )))?;
                validate_link_members(self, kind, &members)?;
                self.link_group_mut(link_group_id)?.members = members;
                Ok(())
            }
            DocumentEdit::Remove(entity) => self.remove(entity, revision, tombstoned),
            DocumentEdit::RecordComparison(receipt) => {
                self.validate_comparison_receipt(&receipt)?;
                self.comparisons.push(receipt);
                Ok(())
            }
        }
    }

    fn attach_dataset(&mut self, dataset: SourceDataset) -> Result<(), VisualizationError> {
        dataset.validate()?;
        if let Some(existing) = self
            .datasets
            .iter()
            .find(|existing| existing.binding.dataset_id == dataset.binding.dataset_id)
        {
            if existing.binding.content_digest != dataset.binding.content_digest {
                return Err(VisualizationError::SourceDigestMismatch {
                    dataset_id: dataset.binding.dataset_id,
                    bound: existing.binding.content_digest,
                    requested: dataset.binding.content_digest,
                });
            }
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: format!("dataset {} is already attached", dataset.binding.dataset_id),
            });
        }
        self.datasets.push(dataset);
        Ok(())
    }

    fn add_trace(
        &mut self,
        trace: NewTrace,
        created: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        validate_label("trace.label", &trace.label)?;
        validate_key("trace.signal-key", &trace.signal_key)?;
        validate_key("trace.coordinate-key", &trace.coordinate_key)?;
        self.require_pane(trace.pane_id)?;
        self.require_axis_in_pane(trace.x_axis_id, trace.pane_id)?;
        self.require_axis_in_pane(trace.y_axis_id, trace.pane_id)?;
        let dataset = self.dataset_for_binding(trace.binding)?;
        let signal = find_column(dataset, &trace.signal_key)?;
        let coordinate = find_column(dataset, &trace.coordinate_key)?;
        if signal.role != ColumnRole::Signal || coordinate.role != ColumnRole::Coordinate {
            return Err(VisualizationError::InvalidValue {
                field: "trace.source-columns",
                message: "trace requires one signal column and one coordinate column".to_owned(),
            });
        }
        let id = TraceId::allocate(self.allocate_serial()?)?;
        self.traces.push(Trace {
            id,
            pane_id: trace.pane_id,
            binding: trace.binding,
            signal_key: trace.signal_key,
            coordinate_key: trace.coordinate_key,
            x_axis_id: trace.x_axis_id,
            y_axis_id: trace.y_axis_id,
            label: trace.label,
            visible: true,
        });
        created.push(EntityRef::Trace(id));
        Ok(())
    }

    fn rename(&mut self, entity: EntityRef, value: String) -> Result<(), VisualizationError> {
        validate_label("entity.label", &value)?;
        match entity {
            EntityRef::Page(id) => self.page_mut(id)?.title = value,
            EntityRef::Pane(id) => self.pane_mut(id)?.title = value,
            EntityRef::Trace(id) => self.trace_mut(id)?.label = value,
            EntityRef::Axis(id) => self.axis_mut(id)?.label = value,
            EntityRef::Cursor(id) => self.cursor_mut(id)?.label = value,
            EntityRef::Marker(id) => self.marker_mut(id)?.label = value,
            EntityRef::Measurement(id) => self.measurement_mut(id)?.label = value,
            EntityRef::Annotation(_) => {
                return Err(VisualizationError::InvalidValue {
                    field: "entity.label",
                    message: "annotations are edited with SetAnnotation".to_owned(),
                });
            }
            EntityRef::LinkGroup(id) => self.link_group_mut(id)?.label = value,
        }
        Ok(())
    }

    fn remove(
        &mut self,
        entity: EntityRef,
        revision: ObjectRevision,
        tombstoned: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        if let Some(group) = self
            .link_groups
            .iter()
            .find(|group| group.members.contains(&entity))
        {
            return Err(VisualizationError::EntityInUse {
                entity,
                dependent: EntityRef::LinkGroup(group.id),
            });
        }
        match entity {
            EntityRef::Page(id) => {
                self.require_page(id)?;
                let pane_ids: Vec<_> = self
                    .panes
                    .iter()
                    .filter_map(|pane| (pane.page_id == id).then_some(pane.id))
                    .collect();
                for pane_id in pane_ids {
                    self.remove_pane_cascade(pane_id, revision, tombstoned);
                }
                self.pages.retain(|page| page.id != id);
            }
            EntityRef::Pane(id) => {
                self.require_pane(id)?;
                self.remove_pane_cascade(id, revision, tombstoned);
                return Ok(());
            }
            EntityRef::Axis(id) => {
                self.require_entity(entity)?;
                if let Some(trace) = self
                    .traces
                    .iter()
                    .find(|trace| trace.x_axis_id == id || trace.y_axis_id == id)
                {
                    return Err(VisualizationError::EntityInUse {
                        entity,
                        dependent: EntityRef::Trace(trace.id),
                    });
                }
                if let Some(cursor) = self.cursors.iter().find(|cursor| cursor.axis_id == id) {
                    return Err(VisualizationError::EntityInUse {
                        entity,
                        dependent: EntityRef::Cursor(cursor.id),
                    });
                }
                self.axes.retain(|axis| axis.id != id);
            }
            EntityRef::Trace(id) => {
                self.require_entity(entity)?;
                let dependent = self
                    .markers
                    .iter()
                    .find(|marker| marker.trace_id == id)
                    .map(|marker| EntityRef::Marker(marker.id))
                    .or_else(|| {
                        self.measurements
                            .iter()
                            .find(|measurement| measurement.trace_ids.contains(&id))
                            .map(|measurement| EntityRef::Measurement(measurement.id))
                    })
                    .or_else(|| {
                        self.annotations
                            .iter()
                            .find_map(|annotation| match &annotation.anchor {
                                AnnotationAnchor::Trace { trace_id, .. } if *trace_id == id => {
                                    Some(EntityRef::Annotation(annotation.id))
                                }
                                _ => None,
                            })
                    });
                if let Some(dependent) = dependent {
                    return Err(VisualizationError::EntityInUse { entity, dependent });
                }
                self.traces.retain(|trace| trace.id != id);
            }
            EntityRef::Cursor(id) => {
                self.require_entity(entity)?;
                self.cursors.retain(|cursor| cursor.id != id);
            }
            EntityRef::Marker(id) => {
                self.require_entity(entity)?;
                self.markers.retain(|marker| marker.id != id);
            }
            EntityRef::Measurement(id) => {
                self.require_entity(entity)?;
                self.measurements.retain(|measurement| measurement.id != id);
            }
            EntityRef::Annotation(id) => {
                self.require_entity(entity)?;
                self.annotations.retain(|annotation| annotation.id != id);
            }
            EntityRef::LinkGroup(id) => {
                self.require_entity(entity)?;
                self.link_groups.retain(|group| group.id != id);
            }
        }
        self.record_tombstone(entity, revision, tombstoned);
        Ok(())
    }

    fn remove_pane_cascade(
        &mut self,
        pane_id: PaneId,
        revision: ObjectRevision,
        tombstoned: &mut Vec<EntityRef>,
    ) {
        let mut removed = Vec::new();
        removed.extend(
            self.axes
                .iter()
                .filter_map(|axis| (axis.pane_id == pane_id).then_some(EntityRef::Axis(axis.id))),
        );
        removed.extend(
            self.traces.iter().filter_map(|trace| {
                (trace.pane_id == pane_id).then_some(EntityRef::Trace(trace.id))
            }),
        );
        removed.extend(self.cursors.iter().filter_map(|cursor| {
            (cursor.pane_id == pane_id).then_some(EntityRef::Cursor(cursor.id))
        }));
        removed.extend(self.markers.iter().filter_map(|marker| {
            (marker.pane_id == pane_id).then_some(EntityRef::Marker(marker.id))
        }));
        removed.extend(self.measurements.iter().filter_map(|measurement| {
            (measurement.pane_id == pane_id).then_some(EntityRef::Measurement(measurement.id))
        }));
        removed.extend(self.annotations.iter().filter_map(|annotation| {
            (annotation.pane_id == pane_id).then_some(EntityRef::Annotation(annotation.id))
        }));
        self.axes.retain(|axis| axis.pane_id != pane_id);
        self.traces.retain(|trace| trace.pane_id != pane_id);
        self.cursors.retain(|cursor| cursor.pane_id != pane_id);
        self.markers.retain(|marker| marker.pane_id != pane_id);
        self.measurements
            .retain(|measurement| measurement.pane_id != pane_id);
        self.annotations
            .retain(|annotation| annotation.pane_id != pane_id);
        let page_id = self
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| pane.page_id);
        self.panes.retain(|pane| pane.id != pane_id);
        if let Some(page_id) = page_id {
            self.normalize_page_pane_layout(page_id, None);
        }
        for entity in removed {
            self.record_tombstone(entity, revision, tombstoned);
        }
        self.record_tombstone(EntityRef::Pane(pane_id), revision, tombstoned);
    }

    fn record_tombstone(
        &mut self,
        entity: EntityRef,
        revision: ObjectRevision,
        receipt: &mut Vec<EntityRef>,
    ) {
        self.tombstones.push(Tombstone {
            entity,
            deleted_at_revision: revision,
        });
        receipt.push(entity);
    }

    fn validate_comparison_receipt(
        &self,
        receipt: &ComparisonReceipt,
    ) -> Result<(), VisualizationError> {
        self.dataset_for_binding(receipt.baseline)?;
        self.dataset_for_binding(receipt.candidate)?;
        receipt.validate_structure()
    }

    fn validate(&self) -> Result<(), VisualizationError> {
        if self.schema_version != Self::SCHEMA_VERSION {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.schema-version",
                message: format!("unsupported schema version {}", self.schema_version),
            });
        }
        validate_label("visualization-document.title", &self.title)?;
        validate_dataset_set(&self.datasets)?;
        ensure_maximum_len(
            "visualization-document.pages",
            self.pages.len(),
            MAX_VISUALIZATION_PAGES,
        )?;
        ensure_maximum_len(
            "visualization-document.panes",
            self.panes.len(),
            MAX_VISUALIZATION_PANES,
        )?;
        ensure_maximum_len(
            "visualization-document.axes",
            self.axes.len(),
            MAX_VISUALIZATION_AXES,
        )?;
        ensure_maximum_len(
            "visualization-document.traces",
            self.traces.len(),
            MAX_VISUALIZATION_TRACES,
        )?;
        ensure_maximum_len(
            "visualization-document.cursors",
            self.cursors.len(),
            MAX_VISUALIZATION_CURSORS,
        )?;
        ensure_maximum_len(
            "visualization-document.markers",
            self.markers.len(),
            MAX_VISUALIZATION_MARKERS,
        )?;
        ensure_maximum_len(
            "visualization-document.measurements",
            self.measurements.len(),
            MAX_VISUALIZATION_MEASUREMENTS,
        )?;
        ensure_maximum_len(
            "visualization-document.annotations",
            self.annotations.len(),
            MAX_VISUALIZATION_ANNOTATIONS,
        )?;
        ensure_maximum_len(
            "visualization-document.link-groups",
            self.link_groups.len(),
            MAX_VISUALIZATION_LINK_GROUPS,
        )?;
        ensure_maximum_len(
            "visualization-document.tombstones",
            self.tombstones.len(),
            MAX_VISUALIZATION_TOMBSTONES,
        )?;
        ensure_maximum_len(
            "visualization-document.comparisons",
            self.comparisons.len(),
            MAX_VISUALIZATION_COMPARISONS,
        )?;
        validate_aggregate_nested_resources(
            "visualization-document.measurement-trace-references",
            &self.measurements,
            MAX_VISUALIZATION_MEASUREMENT_TRACE_REFERENCES_TOTAL,
        )?;
        validate_aggregate_nested_resources(
            "visualization-document.link-member-references",
            &self.link_groups,
            MAX_VISUALIZATION_LINK_MEMBER_REFERENCES_TOTAL,
        )?;
        validate_aggregate_nested_resources(
            "visualization-document.comparison-signals",
            &self.comparisons,
            MAX_VISUALIZATION_COMPARISON_SIGNALS_TOTAL,
        )?;
        if self.pages.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.pages",
                message: "at least one page is required".to_owned(),
            });
        }
        let mut identities = HashSet::new();
        for page in &self.pages {
            validate_label("page.title", &page.title)?;
            page.layout.validate()?;
            validate_key("page.template-id", &page.template_id)?;
            if page.layout == PageLayout::SinglePane
                && self
                    .panes
                    .iter()
                    .filter(|pane| pane.page_id == page.id)
                    .count()
                    > 1
            {
                return Err(VisualizationError::InvalidValue {
                    field: "page.layout",
                    message: "single-pane layout cannot contain more than one pane".to_owned(),
                });
            }
            ensure_identity(&mut identities, EntityRef::Page(page.id))?;
        }
        for pane in &self.panes {
            validate_label("pane.title", &pane.title)?;
            self.require_page(pane.page_id)?;
            self.validate_pane_source(pane.kind, &pane.viewer_id, pane.binding)?;
            if let Some(policy) = &pane.family_policy {
                self.validate_pane_family_policy(pane.id, policy)?;
            }
            ensure_identity(&mut identities, EntityRef::Pane(pane.id))?;
        }
        for page in &self.pages {
            let mut panes: Vec<_> = self
                .panes
                .iter()
                .filter(|pane| pane.page_id == page.id)
                .collect();
            panes.sort_by_key(|pane| (pane.order, pane.id));
            let mut preceding = HashSet::new();
            for (expected_order, pane) in panes.into_iter().enumerate() {
                if pane.order != u32::try_from(expected_order).unwrap_or(u32::MAX) {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.order",
                        message: format!(
                            "pane orders on page {} must be unique and contiguous from zero",
                            page.id.get()
                        ),
                    });
                }
                match pane.placement {
                    PanePlacement::Primary if expected_order == 0 => {}
                    PanePlacement::Primary => {
                        return Err(VisualizationError::InvalidValue {
                            field: "pane.placement",
                            message: "only the first pane on a page may be primary".to_owned(),
                        });
                    }
                    PanePlacement::Below { anchor_pane_id }
                    | PanePlacement::RightOf { anchor_pane_id }
                        if preceding.contains(&anchor_pane_id) => {}
                    PanePlacement::Below { .. } | PanePlacement::RightOf { .. } => {
                        return Err(VisualizationError::InvalidValue {
                            field: "pane.placement",
                            message:
                                "pane placement must reference an earlier pane on the same page"
                                    .to_owned(),
                        });
                    }
                }
                preceding.insert(pane.id);
            }
        }
        for axis in &self.axes {
            validate_label("axis.label", &axis.label)?;
            validate_optional_unit("axis.unit", axis.unit.as_deref())?;
            self.require_pane(axis.pane_id)?;
            ensure_identity(&mut identities, EntityRef::Axis(axis.id))?;
        }
        for trace in &self.traces {
            validate_label("trace.label", &trace.label)?;
            validate_key("trace.signal-key", &trace.signal_key)?;
            validate_key("trace.coordinate-key", &trace.coordinate_key)?;
            self.require_pane(trace.pane_id)?;
            self.require_axis_in_pane(trace.x_axis_id, trace.pane_id)?;
            self.require_axis_in_pane(trace.y_axis_id, trace.pane_id)?;
            let dataset = self.dataset_for_binding(trace.binding)?;
            if find_column(dataset, &trace.signal_key)?.role != ColumnRole::Signal
                || find_column(dataset, &trace.coordinate_key)?.role != ColumnRole::Coordinate
            {
                return Err(VisualizationError::InvalidValue {
                    field: "trace.source-columns",
                    message: "invalid signal or coordinate column role".to_owned(),
                });
            }
            ensure_identity(&mut identities, EntityRef::Trace(trace.id))?;
        }
        for cursor in &self.cursors {
            validate_label("cursor.label", &cursor.label)?;
            cursor.position.validate("cursor.position")?;
            self.require_axis_in_pane(cursor.axis_id, cursor.pane_id)?;
            ensure_identity(&mut identities, EntityRef::Cursor(cursor.id))?;
        }
        for marker in &self.markers {
            validate_label("marker.label", &marker.label)?;
            marker.coordinate.validate("marker.coordinate")?;
            self.require_trace_in_pane(marker.trace_id, marker.pane_id)?;
            ensure_identity(&mut identities, EntityRef::Marker(marker.id))?;
        }
        for measurement in &self.measurements {
            validate_label("measurement.label", &measurement.label)?;
            if measurement.trace_ids.is_empty()
                || measurement.trace_ids.len() > MAX_ENTITY_REFERENCES
            {
                return Err(VisualizationError::InvalidValue {
                    field: "measurement.traces",
                    message: format!("a measurement requires 1 to {MAX_ENTITY_REFERENCES} traces"),
                });
            }
            for trace in &measurement.trace_ids {
                self.require_trace_in_pane(*trace, measurement.pane_id)?;
            }
            ensure_identity(&mut identities, EntityRef::Measurement(measurement.id))?;
        }
        for annotation in &self.annotations {
            validate_annotation(
                self,
                annotation.pane_id,
                &annotation.anchor,
                &annotation.text,
            )?;
            ensure_identity(&mut identities, EntityRef::Annotation(annotation.id))?;
        }
        for group in &self.link_groups {
            validate_label("link-group.label", &group.label)?;
            validate_link_members(self, group.kind, &group.members)?;
            ensure_identity(&mut identities, EntityRef::LinkGroup(group.id))?;
        }
        let mut deleted = HashSet::new();
        for tombstone in &self.tombstones {
            if !deleted.insert(tombstone.entity) || identities.contains(&tombstone.entity) {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.tombstones",
                    message: "tombstones must be unique and must not identify live entities"
                        .to_owned(),
                });
            }
        }
        for receipt in &self.comparisons {
            self.validate_comparison_receipt(receipt)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding(seed: u8) -> DatasetBinding {
        DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([seed; 32]))
    }

    fn dataset(binding: DatasetBinding, offset: f64) -> SourceDataset {
        SourceDataset::new(
            binding,
            vec![
                SourceColumn::new(
                    "time",
                    "Time",
                    ValueType::Real,
                    ColumnRole::Coordinate,
                    Some("s".to_owned()),
                )
                .unwrap(),
                SourceColumn::new(
                    "v(out)",
                    "V(out)",
                    ValueType::Real,
                    ColumnRole::Signal,
                    Some("V".to_owned()),
                )
                .unwrap(),
            ],
            vec![
                SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(offset)]),
                SourceRow::new(vec![TypedValue::Real(1.0), TypedValue::Real(1.0 + offset)]),
                SourceRow::new(vec![TypedValue::Real(2.0), TypedValue::Real(2.0 + offset)]),
            ],
        )
        .unwrap()
    }

    fn document() -> (VisualizationDocument, DatasetBinding) {
        let source = binding(1);
        (
            VisualizationDocument::new("Transient review", vec![dataset(source, 0.0)]).unwrap(),
            source,
        )
    }

    #[test]
    fn content_digest_authenticates_the_exact_document_revision() {
        let (mut document, _) = document();
        let first = document.content_digest().unwrap();
        assert_eq!(document.content_digest().unwrap(), first);

        let restored: VisualizationDocument =
            serde_json::from_slice(&serde_json::to_vec(&document).unwrap()).unwrap();
        assert_eq!(restored.content_digest().unwrap(), first);

        document
            .transact(
                document.revision(),
                vec![DocumentEdit::Rename {
                    entity: EntityRef::Page(document.pages()[0].id),
                    value: "Updated page".to_owned(),
                }],
            )
            .unwrap();
        assert_ne!(document.content_digest().unwrap(), first);
    }

    fn family_dataset(binding: DatasetBinding) -> SourceDataset {
        SourceDataset::new(
            binding,
            vec![
                SourceColumn::new(
                    "time",
                    "Time",
                    ValueType::Real,
                    ColumnRole::Coordinate,
                    Some("s".to_owned()),
                )
                .unwrap(),
                SourceColumn::new(
                    "process",
                    "Process",
                    ValueType::Text,
                    ColumnRole::Coordinate,
                    None,
                )
                .unwrap(),
                SourceColumn::new(
                    "temperature",
                    "Temperature",
                    ValueType::Real,
                    ColumnRole::Coordinate,
                    Some("degC".to_owned()),
                )
                .unwrap(),
                SourceColumn::new(
                    "sample",
                    "Sample",
                    ValueType::Integer,
                    ColumnRole::Coordinate,
                    None,
                )
                .unwrap(),
                SourceColumn::new(
                    "v(out)",
                    "V(out)",
                    ValueType::Real,
                    ColumnRole::Signal,
                    Some("V".to_owned()),
                )
                .unwrap(),
            ],
            vec![
                SourceRow::new(vec![
                    TypedValue::Real(0.0),
                    TypedValue::Text("TT".to_owned()),
                    TypedValue::Real(27.0),
                    TypedValue::Integer(1),
                    TypedValue::Real(0.0),
                ]),
                SourceRow::new(vec![
                    TypedValue::Real(1.0),
                    TypedValue::Text("TT".to_owned()),
                    TypedValue::Real(27.0),
                    TypedValue::Integer(1),
                    TypedValue::Real(1.0),
                ]),
                SourceRow::new(vec![
                    TypedValue::Real(0.0),
                    TypedValue::Text("SS".to_owned()),
                    TypedValue::Real(125.0),
                    TypedValue::Integer(2),
                    TypedValue::Real(-0.1),
                ]),
                SourceRow::new(vec![
                    TypedValue::Real(1.0),
                    TypedValue::Text("SS".to_owned()),
                    TypedValue::Real(125.0),
                    TypedValue::Integer(2),
                    TypedValue::Real(0.9),
                ]),
            ],
        )
        .unwrap()
    }

    fn dimension(key: &str, value_type: ValueType) -> FamilyDimension {
        FamilyDimension::new(key, value_type).unwrap()
    }

    fn family_policy() -> FamilyPresentationPolicy {
        let process = dimension("process", ValueType::Text);
        let temperature = dimension("temperature", ValueType::Real);
        let sample = dimension("sample", ValueType::Integer);
        FamilyPresentationPolicy {
            x_dimension: FamilyXDimension {
                dimension: dimension("time", ValueType::Real),
                ordering: FamilyXOrdering::Ascending,
            },
            family_dimensions: vec![process.clone(), temperature.clone(), sample.clone()],
            facet_layout: Some(FamilyFacetLayout {
                axis_sharing: FacetAxisSharing::Shared,
                overflow: FacetOverflowPolicy::Paginate,
                maximum_panels: 12,
            }),
            aggregation: FamilyAggregationPolicy {
                method: FamilyAggregationMethod::Mean,
                over_dimensions: vec![sample],
            },
            filter: Some(FamilyFilterExpression {
                source: "process in {TT,SS} and temperature >= 27".to_owned(),
                predicate: FamilyPredicate::All {
                    predicates: vec![
                        FamilyPredicate::In {
                            dimension: process.clone(),
                            values: vec![
                                TypedValue::Text("TT".to_owned()),
                                TypedValue::Text("SS".to_owned()),
                            ],
                        },
                        FamilyPredicate::Compare {
                            dimension: temperature.clone(),
                            operator: FamilyComparisonOperator::GreaterThanOrEqual,
                            value: TypedValue::Real(27.0),
                        },
                    ],
                },
            }),
            missing_points: MissingPointPolicy::PreserveAsNotRun,
            encodings: vec![
                FamilyEncodingMap::Color {
                    dimension: process.clone(),
                    palette: AccessibleColorPalette::OkabeItoCategorical,
                },
                FamilyEncodingMap::Label {
                    dimension: process.clone(),
                    prefix: Some("P=".to_owned()),
                },
                FamilyEncodingMap::Dash {
                    dimension: temperature.clone(),
                },
                FamilyEncodingMap::Thickness {
                    dimension: temperature,
                    minimum_points: 1.0,
                    maximum_points: 3.0,
                },
                FamilyEncodingMap::Facet {
                    dimension: process,
                    direction: FacetDirection::Rows,
                },
            ],
        }
    }

    #[test]
    fn source_dataset_rejects_shape_type_and_duplicate_coordinate_errors() {
        let source = binding(2);
        let columns = vec![
            SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Coordinate, None).unwrap(),
            SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None).unwrap(),
        ];
        assert!(matches!(
            SourceDataset::new(
                source,
                columns.clone(),
                vec![SourceRow::new(vec![TypedValue::Real(0.0)])]
            ),
            Err(VisualizationError::RowWidth { .. })
        ));
        assert!(matches!(
            SourceDataset::new(
                source,
                columns.clone(),
                vec![SourceRow::new(vec![
                    TypedValue::Integer(0),
                    TypedValue::Real(1.0)
                ])]
            ),
            Err(VisualizationError::ColumnTypeMismatch { .. })
        ));
        assert!(matches!(
            SourceDataset::new(
                source,
                columns,
                vec![
                    SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(1.0)]),
                    SourceRow::new(vec![TypedValue::Real(0.0), TypedValue::Real(2.0)]),
                ]
            ),
            Err(VisualizationError::DuplicateCoordinateRow(1))
        ));
    }

    #[test]
    fn source_dataset_limits_are_enforced_at_construction_and_deserialization() {
        let source = binding(31);
        let columns = (0..MAX_SOURCE_COLUMNS)
            .map(|index| {
                SourceColumn::new(
                    format!("c{index}"),
                    format!("Column {index}"),
                    ValueType::Real,
                    if index == 0 {
                        ColumnRole::Coordinate
                    } else {
                        ColumnRole::Signal
                    },
                    None,
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        let at_limit = SourceDataset::new(source, columns, Vec::new()).unwrap();

        let mut encoded = serde_json::to_value(&at_limit).unwrap();
        let encoded_columns = encoded["columns"].as_array_mut().unwrap();
        let extra_column = encoded_columns[0].clone();
        encoded_columns.push(extra_column);
        let error = serde_json::from_value::<SourceDataset>(encoded).unwrap_err();
        assert!(error.to_string().contains("resource limit"));

        let oversized_text = SourceDataset::new(
            binding(32),
            vec![
                SourceColumn::new(
                    "corner",
                    "Corner",
                    ValueType::Text,
                    ColumnRole::Coordinate,
                    None,
                )
                .unwrap(),
                SourceColumn::new("value", "Value", ValueType::Real, ColumnRole::Signal, None)
                    .unwrap(),
            ],
            vec![SourceRow::new(vec![
                TypedValue::Text("x".repeat(MAX_SOURCE_TEXT_BYTES + 1)),
                TypedValue::Real(1.0),
            ])],
        );
        assert!(matches!(
            oversized_text,
            Err(VisualizationError::InvalidValue {
                field: "source-row.value",
                ..
            })
        ));
    }

    #[test]
    fn visualization_document_deserialization_rejects_oversized_entity_sequences() {
        let (document, _) = document();
        let mut encoded = serde_json::to_value(&document).unwrap();
        let pages = encoded["pages"].as_array_mut().unwrap();
        let page = pages[0].clone();
        while pages.len() <= MAX_VISUALIZATION_PAGES {
            pages.push(page.clone());
        }
        let error = serde_json::from_value::<VisualizationDocument>(encoded).unwrap_err();
        assert!(error.to_string().contains("resource limit"));
    }

    #[test]
    fn atomic_transactions_reject_oversized_edit_batches_before_mutation() {
        let (mut document, _) = document();
        let before = document.clone();
        let page = document.pages()[0].id;
        let edits = (0..=MAX_VISUALIZATION_TRANSACTION_EDITS)
            .map(|_| DocumentEdit::Rename {
                entity: EntityRef::Page(page),
                value: "Bounded".to_owned(),
            })
            .collect();
        assert!(matches!(
            document.transact(document.revision(), edits),
            Err(VisualizationError::InvalidValue {
                field: "transaction.edits",
                ..
            })
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn nested_sequences_and_source_strings_are_bounded_during_deserialization() {
        let predicate = FamilyPredicate::All {
            predicates: vec![FamilyPredicate::Constant { value: true }],
        };
        let mut predicate_json = serde_json::to_value(predicate).unwrap();
        let child = predicate_json["predicates"][0].clone();
        let children = predicate_json["predicates"].as_array_mut().unwrap();
        while children.len() <= MAX_FAMILY_PREDICATE_CHILDREN {
            children.push(child.clone());
        }
        let predicate_error =
            serde_json::from_value::<FamilyPredicate>(predicate_json).unwrap_err();
        assert!(predicate_error.to_string().contains("resource limit"));

        let measurement = Measurement {
            id: MeasurementId::allocate(1).unwrap(),
            pane_id: PaneId::allocate(2).unwrap(),
            trace_ids: vec![TraceId::allocate(3).unwrap()],
            kind: MeasurementKind::Point,
            label: "Point".to_owned(),
        };
        let mut measurement_json = serde_json::to_value(measurement).unwrap();
        let trace = measurement_json["trace_ids"][0].clone();
        let traces = measurement_json["trace_ids"].as_array_mut().unwrap();
        while traces.len() <= MAX_ENTITY_REFERENCES {
            traces.push(trace.clone());
        }
        let measurement_error =
            serde_json::from_value::<Measurement>(measurement_json).unwrap_err();
        assert!(measurement_error.to_string().contains("resource limit"));

        let source = dataset(binding(33), 0.0);
        let mut source_json = serde_json::to_value(source).unwrap();
        source_json["rows"][0]["values"][0] = serde_json::json!({
            "type": "text",
            "value": "x".repeat(MAX_SOURCE_TEXT_BYTES + 1),
        });
        let text_error = serde_json::from_value::<SourceDataset>(source_json).unwrap_err();
        assert!(text_error.to_string().contains("resource limit"));
    }

    #[test]
    fn predicate_deserialization_enforces_shared_depth_and_total_node_budgets() {
        let leaf = FamilyPredicate::Constant { value: true };
        let branch = FamilyPredicate::All {
            predicates: vec![leaf.clone(); MAX_FAMILY_PREDICATE_CHILDREN],
        };
        let broad = FamilyPredicate::All {
            predicates: vec![branch; 16],
        };
        let broad_error =
            serde_json::from_value::<FamilyPredicate>(serde_json::to_value(broad).unwrap())
                .unwrap_err();
        assert!(broad_error.to_string().contains("1024-node"));

        let nested_not = |levels: usize| {
            (1..levels).fold(leaf.clone(), |predicate, _| FamilyPredicate::Not {
                predicate: Box::new(predicate),
            })
        };
        let at_depth_limit = nested_not(MAX_FAMILY_PREDICATE_DEPTH);
        let restored: FamilyPredicate =
            serde_json::from_value(serde_json::to_value(&at_depth_limit).unwrap()).unwrap();
        assert_eq!(restored, at_depth_limit);

        let over_depth_limit = nested_not(MAX_FAMILY_PREDICATE_DEPTH + 1);
        let depth_error = serde_json::from_value::<FamilyPredicate>(
            serde_json::to_value(over_depth_limit).unwrap(),
        )
        .unwrap_err();
        assert!(depth_error.to_string().contains("32-level"));

        let restored_after_errors: FamilyPredicate =
            serde_json::from_value(serde_json::to_value(leaf).unwrap()).unwrap();
        assert_eq!(
            restored_after_errors,
            FamilyPredicate::Constant { value: true }
        );
    }

    #[test]
    fn document_aggregate_nested_resource_budgets_accept_boundaries_and_reject_overflow() {
        for (field, maximum) in [
            (
                "visualization-document.comparison-signals",
                MAX_VISUALIZATION_COMPARISON_SIGNALS_TOTAL,
            ),
            (
                "visualization-document.measurement-trace-references",
                MAX_VISUALIZATION_MEASUREMENT_TRACE_REFERENCES_TOTAL,
            ),
            (
                "visualization-document.link-member-references",
                MAX_VISUALIZATION_LINK_MEMBER_REFERENCES_TOTAL,
            ),
        ] {
            assert_eq!(
                checked_bounded_sum(field, maximum - 1, 1, maximum).unwrap(),
                maximum
            );
            assert!(matches!(
                checked_bounded_sum(field, maximum, 1, maximum),
                Err(VisualizationError::InvalidValue { .. })
            ));
        }

        let measurement = Measurement {
            id: MeasurementId::allocate(1).unwrap(),
            pane_id: PaneId::allocate(2).unwrap(),
            trace_ids: vec![TraceId::allocate(3).unwrap(); MAX_ENTITY_REFERENCES],
            kind: MeasurementKind::Point,
            label: "Aggregate boundary".to_owned(),
        };
        let mut encoded = serde_json::to_value(document().0).unwrap();
        encoded["measurements"] = serde_json::to_value(vec![measurement; 5]).unwrap();
        let error = serde_json::from_value::<VisualizationDocument>(encoded).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("aggregate nested resource limit")
        );
    }

    #[test]
    fn retained_source_text_accounting_accepts_the_boundary_and_rejects_one_more_byte() {
        assert_eq!(
            checked_bounded_sum(
                "source-dataset.retained-text-bytes",
                MAX_SOURCE_TEXT_BYTES_PER_DATASET - 1,
                1,
                MAX_SOURCE_TEXT_BYTES_PER_DATASET,
            )
            .unwrap(),
            MAX_SOURCE_TEXT_BYTES_PER_DATASET
        );
        assert!(matches!(
            checked_bounded_sum(
                "source-dataset.retained-text-bytes",
                MAX_SOURCE_TEXT_BYTES_PER_DATASET,
                1,
                MAX_SOURCE_TEXT_BYTES_PER_DATASET,
            ),
            Err(VisualizationError::InvalidValue {
                field: "source-dataset.retained-text-bytes",
                ..
            })
        ));
    }

    #[test]
    fn resource_hardening_preserves_extensions_and_bounded_schema_migrations() {
        let source = dataset(binding(34), 0.0);
        let mut source_json = serde_json::to_value(&source).unwrap();
        source_json["untrusted_extension"] = serde_json::json!([]);
        source_json["rows"][0]["untrusted_row_extension"] = serde_json::json!({});
        let restored_source: SourceDataset = serde_json::from_value(source_json).unwrap();
        assert_eq!(restored_source, source);

        let (document, source_binding) = document();
        let mut extended = serde_json::to_value(&document).unwrap();
        extended["untrusted_document_extension"] = serde_json::json!({ "future": true });
        let restored_extended: VisualizationDocument = serde_json::from_value(extended).unwrap();
        assert_eq!(restored_extended, document);
        for schema_version in [1, 2] {
            let mut legacy = serde_json::to_value(&document).unwrap();
            legacy["schema_version"] = serde_json::json!(schema_version);
            let restored: VisualizationDocument = serde_json::from_value(legacy).unwrap();
            assert_eq!(
                restored.schema_version,
                VisualizationDocument::SCHEMA_VERSION
            );
            assert_eq!(restored.datasets()[0].binding(), source_binding);
            assert_eq!(restored.datasets()[0].rows(), document.datasets()[0].rows());
        }
    }

    #[test]
    fn coordinate_identity_uses_exact_typed_values_and_real_bit_patterns() {
        let source = binding(8);
        let columns = vec![
            SourceColumn::new(
                "real",
                "Real coordinate",
                ValueType::Real,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "integer",
                "Integer coordinate",
                ValueType::Integer,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "boolean",
                "Boolean coordinate",
                ValueType::Boolean,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "text",
                "Text coordinate",
                ValueType::Text,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new("value", "Value", ValueType::Real, ColumnRole::Signal, None).unwrap(),
        ];
        let row = |real| {
            SourceRow::new(vec![
                TypedValue::Real(real),
                TypedValue::Integer(1),
                TypedValue::Boolean(true),
                TypedValue::Text("corner-tt".to_owned()),
                TypedValue::Real(1.0),
            ])
        };

        let distinct_signed_zeroes =
            SourceDataset::new(source, columns.clone(), vec![row(0.0), row(-0.0)]).unwrap();
        assert_eq!(distinct_signed_zeroes.rows().len(), 2);

        assert_eq!(
            SourceDataset::new(source, columns, vec![row(0.0), row(-0.0), row(0.0)]),
            Err(VisualizationError::DuplicateCoordinateRow(2))
        );
    }

    #[test]
    fn one_hundred_thousand_exact_coordinate_rows_validate_in_one_pass() {
        const ROW_COUNT: usize = 100_000;
        let source = binding(9);
        let columns = vec![
            SourceColumn::new(
                "sample",
                "Sample",
                ValueType::Integer,
                ColumnRole::Coordinate,
                None,
            )
            .unwrap(),
            SourceColumn::new(
                "v(out)",
                "V(out)",
                ValueType::Real,
                ColumnRole::Signal,
                Some("V".to_owned()),
            )
            .unwrap(),
        ];
        let rows = (0..ROW_COUNT)
            .map(|index| {
                SourceRow::new(vec![
                    TypedValue::Integer(index as i64),
                    TypedValue::Real(index as f64 * 1.0e-6),
                ])
            })
            .collect();

        let dataset = SourceDataset::new(source, columns, rows).unwrap();
        assert_eq!(dataset.rows().len(), ROW_COUNT);
    }

    #[test]
    fn exact_query_returns_typed_values_and_never_interpolates() {
        let (document, source) = document();
        let exact = document
            .query_exact_row(&ExactRowQuery {
                binding: source,
                coordinates: vec![QueryCoordinate {
                    column: "time".to_owned(),
                    value: TypedValue::Real(1.0),
                }],
                projections: vec!["v(out)".to_owned()],
            })
            .unwrap();
        assert_eq!(exact.row_index, 1);
        assert!(exact.values[0].1.exact_eq(&TypedValue::Real(1.0)));
        assert_eq!(
            document.query_exact_row(&ExactRowQuery {
                binding: source,
                coordinates: vec![QueryCoordinate {
                    column: "time".to_owned(),
                    value: TypedValue::Real(1.5),
                }],
                projections: vec!["v(out)".to_owned()],
            }),
            Err(VisualizationError::InterpolationRequired)
        );
    }

    #[test]
    fn digest_mismatch_and_incomplete_queries_fail_explicitly() {
        let (document, source) = document();
        let wrong = DatasetBinding::new(source.dataset_id, ContentDigest::from_bytes([9; 32]));
        assert!(matches!(
            document.query_exact_row(&ExactRowQuery {
                binding: wrong,
                coordinates: vec![],
                projections: vec!["v(out)".to_owned()],
            }),
            Err(VisualizationError::SourceDigestMismatch { .. })
        ));
        assert_eq!(
            document.query_exact_row(&ExactRowQuery {
                binding: source,
                coordinates: vec![],
                projections: vec!["v(out)".to_owned()],
            }),
            Err(VisualizationError::IncompleteCoordinateQuery)
        );
    }

    #[test]
    fn transaction_is_atomic_and_preserves_identity_on_rollback() {
        let (mut document, _) = document();
        let before = document.clone();
        let result = document.transact(
            document.revision(),
            vec![
                DocumentEdit::AddPage {
                    title: "Review".to_owned(),
                },
                DocumentEdit::AddPane {
                    page_id: PageId::allocate(999).unwrap(),
                    title: "Invalid".to_owned(),
                    kind: PaneKind::Cartesian,
                },
            ],
        );
        assert!(matches!(result, Err(VisualizationError::EntityNotFound(_))));
        assert_eq!(document, before);
        let receipt = document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddPage {
                    title: "Review".to_owned(),
                }],
            )
            .unwrap();
        assert_eq!(
            receipt.created,
            vec![EntityRef::Page(PageId::allocate(3).unwrap())]
        );
    }

    #[test]
    fn stale_revision_and_source_digest_rebinding_do_not_commit() {
        let (mut document, source) = document();
        let stale = ObjectRevision::new(document.revision().get() + 1).unwrap();
        assert!(matches!(
            document.transact(
                stale,
                vec![DocumentEdit::AddPage {
                    title: "X".to_owned()
                }]
            ),
            Err(VisualizationError::RevisionConflict { .. })
        ));
        let before = document.clone();
        let conflicting =
            DatasetBinding::new(source.dataset_id, ContentDigest::from_bytes([7; 32]));
        assert!(matches!(
            document.transact(
                document.revision(),
                vec![DocumentEdit::AttachDataset(dataset(conflicting, 0.0))]
            ),
            Err(VisualizationError::SourceDigestMismatch { .. })
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn full_presentation_graph_validates_and_cascade_creates_tombstones() {
        let (mut document, source) = document();
        let pane = document.panes()[0].id;
        let receipt = document
            .transact(
                document.revision(),
                vec![
                    DocumentEdit::AddAxis(NewAxis {
                        pane_id: pane,
                        label: "Time".to_owned(),
                        orientation: AxisOrientation::Horizontal,
                        scale: AxisScale::Linear,
                        unit: Some("s".to_owned()),
                        range: Some(AxisRange::new(0.0, 2.0).unwrap()),
                    }),
                    DocumentEdit::AddAxis(NewAxis {
                        pane_id: pane,
                        label: "Voltage".to_owned(),
                        orientation: AxisOrientation::VerticalLeft,
                        scale: AxisScale::Linear,
                        unit: Some("V".to_owned()),
                        range: None,
                    }),
                ],
            )
            .unwrap();
        let x_axis = match receipt.created[0] {
            EntityRef::Axis(id) => id,
            _ => unreachable!(),
        };
        let y_axis = match receipt.created[1] {
            EntityRef::Axis(id) => id,
            _ => unreachable!(),
        };
        let trace_receipt = document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddTrace(NewTrace {
                    pane_id: pane,
                    binding: source,
                    signal_key: "v(out)".to_owned(),
                    coordinate_key: "time".to_owned(),
                    x_axis_id: x_axis,
                    y_axis_id: y_axis,
                    label: "V(out)".to_owned(),
                })],
            )
            .unwrap();
        let trace = match trace_receipt.created[0] {
            EntityRef::Trace(id) => id,
            _ => unreachable!(),
        };
        document
            .transact(
                document.revision(),
                vec![
                    DocumentEdit::AddCursor {
                        pane_id: pane,
                        axis_id: x_axis,
                        position: TypedValue::Real(1.0),
                        label: "C1".to_owned(),
                    },
                    DocumentEdit::AddMarker {
                        pane_id: pane,
                        trace_id: trace,
                        coordinate: TypedValue::Real(1.0),
                        label: "M1".to_owned(),
                    },
                    DocumentEdit::AddMeasurement {
                        pane_id: pane,
                        trace_ids: vec![trace],
                        kind: MeasurementKind::Maximum,
                        label: "Peak".to_owned(),
                    },
                    DocumentEdit::AddAnnotation {
                        pane_id: pane,
                        anchor: AnnotationAnchor::Trace {
                            trace_id: trace,
                            coordinate: TypedValue::Real(1.0),
                        },
                        text: "Nominal peak".to_owned(),
                    },
                ],
            )
            .unwrap();
        let removed = document
            .transact(
                document.revision(),
                vec![DocumentEdit::Remove(EntityRef::Pane(pane))],
            )
            .unwrap();
        assert!(removed.tombstoned.contains(&EntityRef::Trace(trace)));
        assert!(removed.tombstoned.contains(&EntityRef::Pane(pane)));
        assert!(document.traces().is_empty());
        assert_eq!(document.tombstones().len(), removed.tombstoned.len());
    }

    #[test]
    fn comparison_policy_produces_auditable_pass_and_fail_receipts() {
        let baseline = binding(3);
        let candidate = binding(4);
        let document = VisualizationDocument::new(
            "Comparison",
            vec![dataset(baseline, 0.0), dataset(candidate, 0.01)],
        )
        .unwrap();
        let request = |absolute| ComparisonRequest {
            baseline,
            candidate,
            signal_keys: vec!["v(out)".to_owned()],
            policy: ComparisonPolicy {
                row_alignment: RowAlignmentPolicy::RequireIdentical,
                tolerance: NumericTolerance::new(absolute, 0.0).unwrap(),
                require_identical_units: true,
                execution: ComparisonExecutionContract::default(),
            },
        };
        assert_eq!(
            document.compare(&request(0.02)).unwrap().disposition,
            ComparisonDisposition::Passed
        );
        let failed = document.compare(&request(0.001)).unwrap();
        assert_eq!(failed.disposition, ComparisonDisposition::Failed);
        assert_eq!(failed.rows_compared, 3);
        assert_eq!(failed.signals[0].failed_rows, 3);
    }

    #[test]
    fn comparison_rejects_mismatched_bindings_and_inconsistent_receipts() {
        let baseline = binding(31);
        let candidate = binding(32);
        let baseline_data = dataset(baseline, 0.0);
        let candidate_data = dataset(candidate, 0.0);
        let policy = ComparisonPolicy {
            row_alignment: RowAlignmentPolicy::RequireIdentical,
            tolerance: NumericTolerance::new(0.0, 0.0).unwrap(),
            require_identical_units: true,
            execution: ComparisonExecutionContract::default(),
        };
        let request = ComparisonRequest {
            baseline: binding(33),
            candidate,
            signal_keys: vec!["v(out)".to_owned()],
            policy,
        };
        assert!(matches!(
            compare_source_datasets(&baseline_data, &candidate_data, &request),
            Err(VisualizationError::InvalidValue {
                field: "comparison.binding",
                ..
            })
        ));

        let malformed = ComparisonReceipt {
            baseline,
            candidate,
            policy,
            rows_compared: 3,
            signals: vec![SignalComparison {
                signal_key: "v(out)".to_owned(),
                compared_rows: 2,
                failed_rows: 3,
                maximum_absolute_error: f64::NAN,
                maximum_relative_error: 0.0,
            }],
            disposition: ComparisonDisposition::Failed,
        };
        assert!(malformed.validate_structure().is_err());
    }

    #[test]
    fn comparison_rejects_oversized_or_overlong_signal_keys_before_result_work() {
        let baseline = binding(35);
        let candidate = binding(36);
        let baseline_data = dataset(baseline, 0.0);
        let candidate_data = dataset(candidate, 0.0);
        let policy = ComparisonPolicy {
            row_alignment: RowAlignmentPolicy::RequireIdentical,
            tolerance: NumericTolerance::new(0.0, 0.0).unwrap(),
            require_identical_units: true,
            execution: ComparisonExecutionContract::default(),
        };
        let overlong = ComparisonRequest {
            baseline,
            candidate,
            signal_keys: vec!["s".repeat(MAX_VISUALIZATION_KEY_BYTES + 1)],
            policy,
        };
        assert!(matches!(
            compare_source_datasets(&baseline_data, &candidate_data, &overlong),
            Err(VisualizationError::InvalidValue {
                field: "comparison.signal-key",
                ..
            })
        ));

        let oversized = ComparisonRequest {
            baseline,
            candidate,
            signal_keys: vec!["v(out)".to_owned(); MAX_COMPARISON_SIGNALS + 1],
            policy,
        };
        assert!(matches!(
            compare_source_datasets(&baseline_data, &candidate_data, &oversized),
            Err(VisualizationError::InvalidValue {
                field: "comparison.signal-keys",
                ..
            })
        ));
        let wire_error =
            serde_json::from_value::<ComparisonRequest>(serde_json::to_value(oversized).unwrap())
                .unwrap_err();
        assert!(wire_error.to_string().contains("resource limit"));
    }

    #[test]
    fn comparison_exact_intersection_never_synthesizes_rows() {
        let baseline = binding(5);
        let candidate = binding(6);
        let candidate_data = SourceDataset::new(
            candidate,
            dataset(candidate, 0.0).columns().to_vec(),
            vec![SourceRow::new(vec![
                TypedValue::Real(1.0),
                TypedValue::Real(1.0),
            ])],
        )
        .unwrap();
        let document = VisualizationDocument::new(
            "Intersection",
            vec![dataset(baseline, 0.0), candidate_data],
        )
        .unwrap();
        let receipt = document
            .compare(&ComparisonRequest {
                baseline,
                candidate,
                signal_keys: vec!["v(out)".to_owned()],
                policy: ComparisonPolicy {
                    row_alignment: RowAlignmentPolicy::ExactIntersection,
                    tolerance: NumericTolerance::new(0.0, 0.0).unwrap(),
                    require_identical_units: true,
                    execution: ComparisonExecutionContract::default(),
                },
            })
            .unwrap();
        assert_eq!(receipt.rows_compared, 1);
    }

    #[test]
    fn progressive_operation_enforces_progress_cancel_and_recovery_transitions() {
        let (mut document, _) = document();
        let (_, mut operation) = document
            .start_operation(document.revision(), ProgressiveOperationKind::Export, 10)
            .unwrap();
        operation
            .advance(4, ContentDigest::from_bytes([2; 32]), None)
            .unwrap();
        assert!(matches!(
            operation.advance(3, ContentDigest::from_bytes([3; 32]), None),
            Err(VisualizationError::InvalidProgress { .. })
        ));
        operation.request_cancel().unwrap();
        operation.confirm_cancelled().unwrap();
        operation.recover().unwrap();
        assert_eq!(operation.recovery_count(), 1);
        operation
            .advance(
                10,
                ContentDigest::from_bytes([4; 32]),
                Some(ContentDigest::from_bytes([5; 32])),
            )
            .unwrap();
        assert!(matches!(
            operation.state(),
            ProgressiveOperationState::Completed { .. }
        ));
    }

    #[test]
    fn invalid_operation_updates_leave_operation_unchanged() {
        let (mut document, _) = document();
        let (_, mut operation) = document
            .start_operation(document.revision(), ProgressiveOperationKind::Transform, 2)
            .unwrap();
        let before = operation.clone();
        assert_eq!(
            operation.advance(
                1,
                ContentDigest::from_bytes([3; 32]),
                Some(ContentDigest::from_bytes([4; 32]))
            ),
            Err(VisualizationError::UnexpectedOutputDigest)
        );
        assert_eq!(operation, before);
    }

    #[test]
    fn composed_page_and_bound_pane_commit_exact_source_identity() {
        let (mut document, source) = document();
        let analysis_id = AnalysisInstanceId::new();
        let page_receipt = document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddComposedPage(NewPage {
                    title: "Publication".to_owned(),
                    layout: PageLayout::Columns,
                    template_id: "design-review".to_owned(),
                    update_policy: PageUpdatePolicy::FreezeFigureRevision,
                })],
            )
            .unwrap();
        let page_id = match page_receipt.created[0] {
            EntityRef::Page(id) => id,
            _ => unreachable!(),
        };
        let pane_receipt = document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddBoundPane(NewPane {
                    page_id,
                    title: "Exact transient".to_owned(),
                    kind: PaneKind::Cartesian,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id,
                        dataset: source,
                    }),
                    placement: PanePlacement::Primary,
                })],
            )
            .unwrap();
        let pane_id = match pane_receipt.created[0] {
            EntityRef::Pane(id) => id,
            _ => unreachable!(),
        };
        let page = document
            .pages()
            .iter()
            .find(|page| page.id == page_id)
            .unwrap();
        assert_eq!(page.layout, PageLayout::Columns);
        assert_eq!(page.template_id, "design-review");
        assert_eq!(page.update_policy, PageUpdatePolicy::FreezeFigureRevision);
        let pane = document
            .panes()
            .iter()
            .find(|pane| pane.id == pane_id)
            .unwrap();
        assert_eq!(pane.viewer_id, "viewer-waveform");
        assert_eq!(pane.binding.unwrap().analysis_id, analysis_id);
        assert_eq!(pane.binding.unwrap().dataset, source);
        assert_eq!(pane.placement, PanePlacement::Primary);
        assert_eq!(pane.order, 0);
        let restored: VisualizationDocument =
            serde_json::from_str(&serde_json::to_string(&document).unwrap()).unwrap();
        assert_eq!(restored, document);
    }

    #[test]
    fn invalid_pane_binding_and_single_pane_layout_roll_back_atomically() {
        let (mut document, source) = document();
        let page_id = document.pages()[0].id;
        let primary = document.panes()[0].id;
        let wrong_digest =
            DatasetBinding::new(source.dataset_id, ContentDigest::from_bytes([99; 32]));
        let before = document.clone();
        assert!(matches!(
            document.transact(
                document.revision(),
                vec![DocumentEdit::AddBoundPane(NewPane {
                    page_id,
                    title: "Mismatched viewer".to_owned(),
                    kind: PaneKind::Table,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: None,
                    placement: PanePlacement::Below {
                        anchor_pane_id: primary,
                    },
                })],
            ),
            Err(VisualizationError::InvalidValue {
                field: "pane.kind",
                ..
            })
        ));
        assert_eq!(document, before);
        assert!(matches!(
            document.transact(
                document.revision(),
                vec![DocumentEdit::AddBoundPane(NewPane {
                    page_id,
                    title: "Invalid".to_owned(),
                    kind: PaneKind::Cartesian,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id: AnalysisInstanceId::new(),
                        dataset: wrong_digest,
                    }),
                    placement: PanePlacement::Below {
                        anchor_pane_id: primary,
                    },
                })],
            ),
            Err(VisualizationError::SourceDigestMismatch { .. })
        ));
        assert_eq!(document, before);

        document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddPane {
                    page_id,
                    title: "Second".to_owned(),
                    kind: PaneKind::Table,
                }],
            )
            .unwrap();
        let before = document.clone();
        assert!(matches!(
            document.transact(
                document.revision(),
                vec![DocumentEdit::SetPageComposition {
                    page_id,
                    layout: PageLayout::SinglePane,
                    template_id: "engineering-dark".to_owned(),
                    update_policy: PageUpdatePolicy::RefreshLinkedFigures,
                }],
            ),
            Err(VisualizationError::InvalidValue {
                field: "page.layout",
                ..
            })
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn pane_placement_orders_are_stable_across_insert_move_and_remove() {
        let (mut document, _) = document();
        let page_id = document.pages()[0].id;
        let primary = document.panes()[0].id;
        let first = document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddBoundPane(NewPane {
                    page_id,
                    title: "Below".to_owned(),
                    kind: PaneKind::Table,
                    viewer_id: "viewer-table".to_owned(),
                    binding: None,
                    placement: PanePlacement::Below {
                        anchor_pane_id: primary,
                    },
                })],
            )
            .unwrap();
        let below = match first.created[0] {
            EntityRef::Pane(id) => id,
            _ => unreachable!(),
        };
        let second = document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddBoundPane(NewPane {
                    page_id,
                    title: "Right".to_owned(),
                    kind: PaneKind::Histogram,
                    viewer_id: "viewer-histogram".to_owned(),
                    binding: None,
                    placement: PanePlacement::RightOf {
                        anchor_pane_id: primary,
                    },
                })],
            )
            .unwrap();
        let right = match second.created[0] {
            EntityRef::Pane(id) => id,
            _ => unreachable!(),
        };
        assert_eq!(
            document
                .panes()
                .iter()
                .find(|pane| pane.id == right)
                .unwrap()
                .order,
            1
        );
        assert_eq!(
            document
                .panes()
                .iter()
                .find(|pane| pane.id == below)
                .unwrap()
                .order,
            2
        );
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::PlacePane {
                    pane_id: below,
                    page_id,
                    placement: PanePlacement::RightOf {
                        anchor_pane_id: right,
                    },
                }],
            )
            .unwrap();
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::Remove(EntityRef::Pane(right))],
            )
            .unwrap();
        let mut panes: Vec<_> = document.panes().iter().collect();
        panes.sort_by_key(|pane| pane.order);
        assert_eq!(
            panes.iter().map(|pane| pane.order).collect::<Vec<_>>(),
            vec![0, 1]
        );
        assert_eq!(panes[0].placement, PanePlacement::Primary);
        assert!(matches!(panes[1].placement, PanePlacement::Below { .. }));
    }

    #[test]
    fn pane_family_policy_roundtrips_typed_dimensions_and_accessible_encodings() {
        let source = binding(41);
        let mut document =
            VisualizationDocument::new("PVT family", vec![family_dataset(source)]).unwrap();
        let pane_id = document.panes()[0].id;
        let policy = family_policy();
        policy.validate().unwrap();

        let receipt = document
            .transact(
                document.revision(),
                vec![
                    DocumentEdit::SetPaneSource {
                        pane_id,
                        viewer_id: "viewer-waveform".to_owned(),
                        binding: Some(PaneDataBinding {
                            analysis_id: AnalysisInstanceId::new(),
                            dataset: source,
                        }),
                    },
                    DocumentEdit::SetPaneFamilyPresentation {
                        pane_id,
                        policy: Some(policy.clone()),
                    },
                ],
            )
            .unwrap();

        assert_eq!(receipt.edit_count, 2);
        assert_eq!(document.panes()[0].family_policy.as_ref(), Some(&policy));
        let serialized = serde_json::to_string_pretty(&document).unwrap();
        let restored: VisualizationDocument = serde_json::from_str(&serialized).unwrap();
        assert_eq!(restored, document);
        assert!(serialized.contains("okabe-ito-categorical"));
        assert!(serialized.contains("preserve-as-not-run"));
    }

    #[test]
    fn inaccessible_or_source_incompatible_family_policy_rolls_back_every_edit() {
        let source = binding(42);
        let mut document =
            VisualizationDocument::new("PVT family", vec![family_dataset(source)]).unwrap();
        let pane_id = document.panes()[0].id;
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::SetPaneSource {
                    pane_id,
                    viewer_id: "viewer-waveform".to_owned(),
                    binding: Some(PaneDataBinding {
                        analysis_id: AnalysisInstanceId::new(),
                        dataset: source,
                    }),
                }],
            )
            .unwrap();

        let mut inaccessible = family_policy();
        inaccessible.encodings.retain(|encoding| {
            !matches!(
                encoding,
                FamilyEncodingMap::Label { .. } | FamilyEncodingMap::Facet { .. }
            )
        });
        inaccessible.facet_layout = None;
        let before = document.clone();
        assert!(matches!(
            document.transact(
                document.revision(),
                vec![
                    DocumentEdit::Rename {
                        entity: EntityRef::Pane(pane_id),
                        value: "Must roll back".to_owned(),
                    },
                    DocumentEdit::SetPaneFamilyPresentation {
                        pane_id,
                        policy: Some(inaccessible),
                    },
                ],
            ),
            Err(VisualizationError::InvalidValue {
                field: "family.encodings.accessibility",
                ..
            })
        ));
        assert_eq!(document, before);

        let mut incompatible = family_policy();
        incompatible.x_dimension.dimension = dimension("frequency", ValueType::Real);
        assert!(matches!(
            document.transact(
                document.revision(),
                vec![DocumentEdit::SetPaneFamilyPresentation {
                    pane_id,
                    policy: Some(incompatible),
                }],
            ),
            Err(VisualizationError::InvalidValue {
                field: "pane.family-policy.dimension",
                ..
            })
        ));
        assert_eq!(document, before);
    }

    #[test]
    fn typed_family_filter_rejects_type_drift_before_commit() {
        let mut policy = family_policy();
        policy.filter = Some(FamilyFilterExpression {
            source: "temperature >= hot".to_owned(),
            predicate: FamilyPredicate::Compare {
                dimension: dimension("temperature", ValueType::Real),
                operator: FamilyComparisonOperator::GreaterThanOrEqual,
                value: TypedValue::Text("hot".to_owned()),
            },
        });
        assert!(matches!(
            policy.validate(),
            Err(VisualizationError::InvalidValue {
                field: "family.filter.value",
                ..
            })
        ));
    }

    #[test]
    fn schema_v2_documents_migrate_to_an_unconfigured_family_policy_deterministically() {
        let source = binding(43);
        let mut document =
            VisualizationDocument::new("PVT family", vec![family_dataset(source)]).unwrap();
        let pane_id = document.panes()[0].id;
        document
            .transact(
                document.revision(),
                vec![
                    DocumentEdit::SetPaneSource {
                        pane_id,
                        viewer_id: "viewer-waveform".to_owned(),
                        binding: Some(PaneDataBinding {
                            analysis_id: AnalysisInstanceId::new(),
                            dataset: source,
                        }),
                    },
                    DocumentEdit::SetPaneFamilyPresentation {
                        pane_id,
                        policy: Some(family_policy()),
                    },
                ],
            )
            .unwrap();
        let mut legacy = serde_json::to_value(&document).unwrap();
        legacy["schema_version"] = serde_json::json!(2);

        let migrated: VisualizationDocument = serde_json::from_value(legacy).unwrap();
        assert_eq!(
            migrated.schema_version,
            VisualizationDocument::SCHEMA_VERSION
        );
        assert!(
            migrated
                .panes()
                .iter()
                .all(|pane| pane.family_policy.is_none())
        );
    }

    #[test]
    fn schema_v1_documents_migrate_page_and_pane_composition_deterministically() {
        let (mut document, _) = document();
        let page_id = document.pages()[0].id;
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::AddPane {
                    page_id,
                    title: "Legacy table".to_owned(),
                    kind: PaneKind::Table,
                }],
            )
            .unwrap();
        let mut legacy = serde_json::to_value(&document).unwrap();
        legacy["schema_version"] = serde_json::json!(1);
        for page in legacy["pages"].as_array_mut().unwrap() {
            let page = page.as_object_mut().unwrap();
            page.remove("layout");
            page.remove("template_id");
            page.remove("update_policy");
        }
        for pane in legacy["panes"].as_array_mut().unwrap() {
            let pane = pane.as_object_mut().unwrap();
            pane.remove("viewer_id");
            pane.remove("binding");
            pane.remove("placement");
            pane.remove("order");
        }
        let migrated: VisualizationDocument = serde_json::from_value(legacy).unwrap();
        assert_eq!(
            migrated.schema_version,
            VisualizationDocument::SCHEMA_VERSION
        );
        assert_eq!(migrated.pages()[0].layout, PageLayout::Rows);
        assert_eq!(migrated.pages()[0].template_id, "engineering-dark");
        assert_eq!(migrated.panes()[0].viewer_id, "viewer-waveform");
        assert_eq!(migrated.panes()[1].viewer_id, "viewer-table");
        assert_eq!(migrated.panes()[0].order, 0);
        assert_eq!(migrated.panes()[1].order, 1);
        assert_eq!(
            migrated.panes()[1].placement,
            PanePlacement::Below {
                anchor_pane_id: migrated.panes()[0].id,
            }
        );
    }
}

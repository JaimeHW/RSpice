//! Canonical, UI-independent domain model for Visualization Studio.
//!
//! A visualization document is a versioned presentation over immutable result
//! datasets.  Source samples and their content digests are never edited in
//! place; presentation changes are committed atomically through [`DocumentEdit`].

mod apply;
mod comparison;
mod edits;
mod family;
mod query;

pub use comparison::*;
pub use edits::*;
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
    RevisionError, SimulationPlanId,
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
    pub const fn value_type(&self) -> ValueType {
        self.value_type
    }

    #[must_use]
    pub const fn role(&self) -> ColumnRole {
        self.role
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
    Peak,
    SpecificationLine,
    MeasurementAnchor,
}

impl PlotMarkerKind {
    pub const ALL: [Self; 4] = [
        Self::PointNote,
        Self::Peak,
        Self::SpecificationLine,
        Self::MeasurementAnchor,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::PointNote => "Point note",
            Self::Peak => "Peak flag",
            Self::SpecificationLine => "Specification line",
            Self::MeasurementAnchor => "Measurement anchor",
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
    tracking: ResultDocumentTracking,
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
    #[serde(default)]
    tracking: ResultDocumentTracking,
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
            tracking: wire.tracking,
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
                document.migrate_v3_to_v4();
            }
            2 => {
                document.migrate_v2_to_v3();
                document.migrate_v3_to_v4();
            }
            3 => document.migrate_v3_to_v4(),
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
    /// Resource hardening does not change the schema-v4 wire vocabulary.
    ///
    /// V1 through V4 documents within the published resource limits retain
    /// their prior deterministic interpretation. Inputs above those limits
    /// are rejected as unsafe containers; the limits do not reinterpret or
    /// migrate any accepted source value, identity, or presentation entity.
    /// Unknown extension fields remain ignored for schema-v4 forward
    /// compatibility; introducing required semantics still requires a schema
    /// revision.
    pub const SCHEMA_VERSION: u16 = 4;

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

    /// Human-facing document title retained by the project document registry.
    ///
    /// The title is authoritative project content rather than presentation
    /// state: document tabs, reports, and duplicate-name validation all read
    /// this exact value.
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[must_use]
    pub const fn tracking(&self) -> ResultDocumentTracking {
        self.tracking
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
    pub fn tombstones(&self) -> &[Tombstone] {
        &self.tombstones
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
        self.schema_version = 3;
    }

    fn migrate_v3_to_v4(&mut self) {
        // Earlier documents were bound only by immutable DatasetBinding and
        // therefore had pinned semantics. Migration must not invent plan
        // ownership or silently follow a newer run.
        self.tracking = ResultDocumentTracking::pinned();
        self.schema_version = Self::SCHEMA_VERSION;
    }
}

#[cfg(test)]
mod tests;

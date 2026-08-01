//! Typed domain contract for deterministic schematic arrays.
//!
//! Array counts include the retained source selection as member zero.  This is
//! important for identity and naming: `U4…U11` describes an eight-member array
//! that creates seven new components while retaining `U4`.

use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

use super::bus::{Bus, BusTap};
use super::component::Component;
use super::design_note::DesignNote;
use super::documentation_shape::DocumentationShape;
use super::net_label::{Junction, NetLabel};
use super::point::Point;
use super::selection::Selection;
use super::wire::{Wire, WireConnection};

const fn unit_naming_stride() -> u64 {
    1
}

/// Hard safety bound for a single array transaction, including its source.
///
/// The bound keeps previews and commits deterministic on desktop and WebAssembly
/// and prevents a malformed count from allocating an unbounded candidate graph.
pub const MAX_SCHEMATIC_ARRAY_MEMBERS: usize = 4_096;

/// Geometry rule used to place repeated array members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchematicArrayKind {
    #[default]
    Linear,
    Rectangular,
    RadialDocumentation,
}

impl SchematicArrayKind {
    pub const ALL: [Self; 3] = [Self::Linear, Self::Rectangular, Self::RadialDocumentation];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Linear => "Linear",
            Self::Rectangular => "Rectangular",
            Self::RadialDocumentation => "Radial documentation",
        }
    }

    /// Whether the array may alter the electrical topology revision.
    pub const fn is_electrical(self) -> bool {
        !matches!(self, Self::RadialDocumentation)
    }
}

impl fmt::Display for SchematicArrayKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.label())
    }
}

/// Validated two-dimensional member count.
///
/// Dimensions are private so checked multiplication and the transaction bound
/// cannot be bypassed after construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct SchematicArrayCount {
    columns: usize,
    rows: usize,
}

impl<'de> Deserialize<'de> for SchematicArrayCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SerializedCount {
            columns: usize,
            rows: usize,
        }

        let count = SerializedCount::deserialize(deserializer)?;
        Self::new(count.columns, count.rows).map_err(serde::de::Error::custom)
    }
}

impl SchematicArrayCount {
    pub fn new(columns: usize, rows: usize) -> Result<Self, SchematicArrayError> {
        if columns == 0 || rows == 0 {
            return Err(SchematicArrayError::ZeroCountDimension);
        }
        let members = columns
            .checked_mul(rows)
            .ok_or(SchematicArrayError::CountOverflow)?;
        if members > MAX_SCHEMATIC_ARRAY_MEMBERS {
            return Err(SchematicArrayError::CountExceedsLimit {
                members,
                maximum: MAX_SCHEMATIC_ARRAY_MEMBERS,
            });
        }
        Ok(Self { columns, rows })
    }

    pub fn parse(input: &str) -> Result<Self, SchematicArrayError> {
        input.parse()
    }

    pub const fn columns(self) -> usize {
        self.columns
    }

    pub const fn rows(self) -> usize {
        self.rows
    }

    /// Total membership, including the retained source at member zero.
    pub const fn member_count(self) -> usize {
        self.columns.saturating_mul(self.rows)
    }

    pub const fn replica_count(self) -> usize {
        self.member_count().saturating_sub(1)
    }

    pub fn validate_for(self, kind: SchematicArrayKind) -> Result<(), SchematicArrayError> {
        Self::new(self.columns, self.rows)?;
        if self.member_count() < 2 {
            return Err(SchematicArrayError::AtLeastTwoMembersRequired);
        }
        match kind {
            SchematicArrayKind::Linear if (self.columns == 1) == (self.rows == 1) => {
                Err(SchematicArrayError::LinearCountRequiresOneAxis)
            }
            SchematicArrayKind::Rectangular if self.columns == 1 || self.rows == 1 => {
                Err(SchematicArrayError::RectangularCountRequiresTwoAxes)
            }
            SchematicArrayKind::RadialDocumentation if self.rows != 1 => {
                Err(SchematicArrayError::RadialCountRequiresOneRow)
            }
            _ => Ok(()),
        }
    }
}

impl fmt::Display for SchematicArrayCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} × {}", self.columns, self.rows)
    }
}

impl FromStr for SchematicArrayCount {
    type Err = SchematicArrayError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut dimensions = input.trim().split('×');
        let Some(columns) = dimensions.next() else {
            return Err(SchematicArrayError::InvalidCountFormat {
                input: input.to_owned(),
            });
        };
        let Some(rows) = dimensions.next() else {
            return Err(SchematicArrayError::InvalidCountFormat {
                input: input.to_owned(),
            });
        };
        if dimensions.next().is_some() {
            return Err(SchematicArrayError::InvalidCountFormat {
                input: input.to_owned(),
            });
        }
        let columns = columns.trim().parse::<usize>().map_err(|_| {
            SchematicArrayError::InvalidCountFormat {
                input: input.to_owned(),
            }
        })?;
        let rows =
            rows.trim()
                .parse::<usize>()
                .map_err(|_| SchematicArrayError::InvalidCountFormat {
                    input: input.to_owned(),
                })?;
        Self::new(columns, rows)
    }
}

/// One endpoint of a typed schematic naming sequence.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SchematicArrayNameAtom {
    /// A component-style identifier containing one numeric run (`U4`, `X3A`).
    Reference {
        prefix: String,
        number: u64,
        suffix: String,
        minimum_width: Option<usize>,
    },
    /// A scalar bus-style indexed name (`DATA[7]`).
    Indexed {
        stem: String,
        index: u64,
        minimum_width: Option<usize>,
    },
}

impl SchematicArrayNameAtom {
    pub fn parse(input: &str) -> Result<Self, SchematicArrayError> {
        if input.is_empty()
            || input
                .chars()
                .any(|character| character.is_whitespace() || matches!(character, '…' | '·' | '×'))
        {
            return Err(invalid_naming_clause(input, "invalid name endpoint"));
        }

        if let Some(stem) = input.strip_suffix(']').and_then(|body| {
            let bracket = body.rfind('[')?;
            Some((&body[..bracket], &body[bracket + 1..]))
        }) {
            let (stem, digits) = stem;
            if stem.is_empty() || !is_ascii_digits(digits) {
                return Err(invalid_naming_clause(input, "invalid indexed endpoint"));
            }
            let index = digits
                .parse::<u64>()
                .map_err(|_| invalid_naming_clause(input, "index is out of range"))?;
            return Ok(Self::Indexed {
                stem: stem.to_owned(),
                index,
                minimum_width: leading_zero_width(digits),
            });
        }

        let Some(start) = input.find(|character: char| character.is_ascii_digit()) else {
            return Err(invalid_naming_clause(
                input,
                "reference endpoint has no numeric run",
            ));
        };
        if start == 0 {
            return Err(invalid_naming_clause(
                input,
                "reference endpoint must have a prefix",
            ));
        }
        let end = input[start..]
            .find(|character: char| !character.is_ascii_digit())
            .map_or(input.len(), |offset| start + offset);
        let prefix = &input[..start];
        let digits = &input[start..end];
        let suffix = &input[end..];
        if suffix.chars().any(|character| character.is_ascii_digit()) {
            return Err(invalid_naming_clause(
                input,
                "reference endpoint has multiple numeric runs",
            ));
        }
        let number = digits
            .parse::<u64>()
            .map_err(|_| invalid_naming_clause(input, "reference number is out of range"))?;
        Ok(Self::Reference {
            prefix: prefix.to_owned(),
            number,
            suffix: suffix.to_owned(),
            minimum_width: leading_zero_width(digits),
        })
    }

    pub const fn numeric_value(&self) -> u64 {
        match self {
            Self::Reference { number, .. } => *number,
            Self::Indexed { index, .. } => *index,
        }
    }

    fn pattern_matches(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Reference {
                    prefix,
                    number,
                    suffix,
                    minimum_width,
                    ..
                },
                Self::Reference {
                    prefix: other_prefix,
                    number: other_number,
                    suffix: other_suffix,
                    minimum_width: other_width,
                    ..
                },
            ) => {
                prefix == other_prefix
                    && suffix == other_suffix
                    && compatible_width(*minimum_width, *number, *other_width, *other_number)
            }
            (
                Self::Indexed {
                    stem,
                    index,
                    minimum_width,
                    ..
                },
                Self::Indexed {
                    stem: other_stem,
                    index: other_index,
                    minimum_width: other_width,
                    ..
                },
            ) => {
                stem == other_stem
                    && compatible_width(*minimum_width, *index, *other_width, *other_index)
            }
            _ => false,
        }
    }

    pub fn with_numeric_value(&self, value: u64) -> Self {
        match self {
            Self::Reference {
                prefix,
                suffix,
                minimum_width,
                ..
            } => Self::Reference {
                prefix: prefix.clone(),
                number: value,
                suffix: suffix.clone(),
                minimum_width: *minimum_width,
            },
            Self::Indexed {
                stem,
                minimum_width,
                ..
            } => Self::Indexed {
                stem: stem.clone(),
                index: value,
                minimum_width: *minimum_width,
            },
        }
    }
}

impl fmt::Display for SchematicArrayNameAtom {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reference {
                prefix,
                number,
                suffix,
                minimum_width,
            } => {
                formatter.write_str(prefix)?;
                write_padded(formatter, *number, *minimum_width)?;
                formatter.write_str(suffix)
            }
            Self::Indexed {
                stem,
                index,
                minimum_width,
            } => {
                formatter.write_str(stem)?;
                formatter.write_str("[")?;
                write_padded(formatter, *index, *minimum_width)?;
                formatter.write_str("]")
            }
        }
    }
}

/// Inclusive, typed name range associated with one selected source name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchematicArrayNameRange {
    start: SchematicArrayNameAtom,
    end: SchematicArrayNameAtom,
    length: usize,
    #[serde(default = "unit_naming_stride")]
    stride: u64,
    source: String,
}

impl SchematicArrayNameRange {
    pub fn parse(clause: &str) -> Result<Self, SchematicArrayError> {
        let mut endpoints = clause.trim().split('…');
        let Some(start_text) = endpoints.next() else {
            return Err(invalid_naming_clause(clause, "missing range start"));
        };
        let Some(end_text) = endpoints.next() else {
            return Err(invalid_naming_clause(
                clause,
                "use one Unicode ellipsis between endpoints",
            ));
        };
        if endpoints.next().is_some() || start_text.trim().is_empty() || end_text.trim().is_empty()
        {
            return Err(invalid_naming_clause(
                clause,
                "use exactly one Unicode ellipsis between endpoints",
            ));
        }
        let start = SchematicArrayNameAtom::parse(start_text.trim())?;
        let end = SchematicArrayNameAtom::parse(end_text.trim())?;
        if !start.pattern_matches(&end) {
            return Err(SchematicArrayError::MismatchedNamingEndpoints {
                clause: clause.trim().to_owned(),
            });
        }
        let distance = start.numeric_value().abs_diff(end.numeric_value());
        let length = distance
            .checked_add(1)
            .and_then(|length| usize::try_from(length).ok())
            .ok_or(SchematicArrayError::NamingRangeLengthOverflow)?;
        let source = start.to_string();
        Ok(Self {
            start,
            end,
            length,
            stride: 1,
            source,
        })
    }

    pub const fn start(&self) -> &SchematicArrayNameAtom {
        &self.start
    }

    pub const fn end(&self) -> &SchematicArrayNameAtom {
        &self.end
    }

    pub const fn len(&self) -> usize {
        self.length
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    pub fn source_name(&self) -> &str {
        &self.source
    }

    /// Deterministically materialize the name for one zero-based array member.
    pub fn value_at(&self, member_index: usize) -> Option<String> {
        if member_index >= self.length {
            return None;
        }
        let start = self.start.numeric_value();
        let offset = u64::try_from(member_index).ok()?.checked_mul(self.stride)?;
        let value = if self.end.numeric_value() >= start {
            start.checked_add(offset)?
        } else {
            start.checked_sub(offset)?
        };
        Some(self.start.with_numeric_value(value).to_string())
    }
}

impl fmt::Display for SchematicArrayNameRange {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}…{}", self.start, self.end)
    }
}

/// Ordered naming contract for all nameable objects in the source selection.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SchematicArrayNaming {
    ranges: Vec<SchematicArrayNameRange>,
}

impl SchematicArrayNaming {
    pub fn parse(input: &str) -> Result<Self, SchematicArrayError> {
        input.parse()
    }

    pub fn new(ranges: Vec<SchematicArrayNameRange>) -> Result<Self, SchematicArrayError> {
        let mut sources = HashSet::with_capacity(ranges.len());
        for range in &ranges {
            let source = range.source_name();
            if !sources.insert(source.to_ascii_lowercase()) {
                return Err(SchematicArrayError::DuplicateNamingSource {
                    source: source.to_owned(),
                });
            }
        }
        Ok(Self { ranges })
    }

    pub fn ranges(&self) -> &[SchematicArrayNameRange] {
        &self.ranges
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = &SchematicArrayNameRange> {
        self.ranges.iter()
    }

    pub fn sources(&self) -> impl ExactSizeIterator<Item = &str> {
        self.ranges.iter().map(SchematicArrayNameRange::source_name)
    }

    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn len(&self) -> usize {
        self.ranges.len()
    }

    pub fn contains_source(&self, source: &str) -> bool {
        self.range_for_source(source).is_some()
    }

    pub fn range_for_source(&self, source: &str) -> Option<&SchematicArrayNameRange> {
        self.ranges
            .iter()
            .find(|range| range.source_name().eq_ignore_ascii_case(source))
    }

    pub fn value_at(&self, range_index: usize, member_index: usize) -> Option<String> {
        self.ranges.get(range_index)?.value_at(member_index)
    }

    pub fn value_for_source(&self, source: &str, member_index: usize) -> Option<String> {
        self.range_for_source(source)?.value_at(member_index)
    }

    pub fn validate_for_members(&self, members: usize) -> Result<(), SchematicArrayError> {
        let mut normalized = self.clone();
        normalized.normalize_for_members(members)
    }

    pub fn normalized_for_members(mut self, members: usize) -> Result<Self, SchematicArrayError> {
        self.normalize_for_members(members)?;
        Ok(self)
    }

    fn normalize_for_members(&mut self, members: usize) -> Result<(), SchematicArrayError> {
        if members < 2 {
            return Err(SchematicArrayError::AtLeastTwoMembersRequired);
        }
        for range in &mut self.ranges {
            let distance = range
                .start
                .numeric_value()
                .abs_diff(range.end.numeric_value());
            let intervals = u64::try_from(members - 1)
                .map_err(|_| SchematicArrayError::NamingRangeLengthOverflow)?;
            if distance == 0 || distance % intervals != 0 {
                return Err(SchematicArrayError::NamingRangeLengthMismatch {
                    source: range.source_name().to_owned(),
                    actual: range.len(),
                    expected: members,
                });
            }
            range.stride = distance / intervals;
            range.length = members;
        }
        Ok(())
    }
}

impl fmt::Display for SchematicArrayNaming {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, range) in self.ranges.iter().enumerate() {
            if index > 0 {
                formatter.write_str(" · ")?;
            }
            range.fmt(formatter)?;
        }
        Ok(())
    }
}

impl FromStr for SchematicArrayNaming {
    type Err = SchematicArrayError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim();
        if input.is_empty() {
            return Ok(Self::default());
        }
        let mut ranges = Vec::new();
        for clause in input.split('·') {
            if clause.trim().is_empty() {
                return Err(invalid_naming_clause(clause, "empty naming clause"));
            }
            ranges.push(SchematicArrayNameRange::parse(clause)?);
        }
        Self::new(ranges)
    }
}

/// Placement input resolved by the canvas interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "point", rename_all = "snake_case")]
pub enum SchematicArrayPlacement {
    /// Per-member pitch for linear arrays, or X/Y cell pitch for rectangular arrays.
    Pitch(Point),
    /// Rotation center for a full-circle documentation array.
    Center(Point),
}

/// Fully parsed and locally validated array command input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SchematicArrayPlan {
    pub(crate) kind: SchematicArrayKind,
    pub(crate) count: SchematicArrayCount,
    pub(crate) naming: SchematicArrayNaming,
    pub(crate) placement: SchematicArrayPlacement,
}

impl<'de> Deserialize<'de> for SchematicArrayPlan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SerializedPlan {
            kind: SchematicArrayKind,
            count: SchematicArrayCount,
            naming: SchematicArrayNaming,
            placement: SchematicArrayPlacement,
        }

        let plan = SerializedPlan::deserialize(deserializer)?;
        Self::new(plan.kind, plan.count, plan.naming, plan.placement)
            .map_err(serde::de::Error::custom)
    }
}

impl SchematicArrayPlan {
    pub fn new(
        kind: SchematicArrayKind,
        count: SchematicArrayCount,
        naming: SchematicArrayNaming,
        placement: SchematicArrayPlacement,
    ) -> Result<Self, SchematicArrayError> {
        count.validate_for(kind)?;
        let naming = SchematicArrayNaming::new(
            naming
                .ranges
                .into_iter()
                .map(|range| SchematicArrayNameRange::parse(&range.to_string()))
                .collect::<Result<_, _>>()?,
        )?
        .normalized_for_members(count.member_count())?;
        validate_placement(kind, count, placement)?;
        Ok(Self {
            kind,
            count,
            naming,
            placement,
        })
    }

    pub fn parse(
        kind: SchematicArrayKind,
        count: &str,
        naming: &str,
        placement: SchematicArrayPlacement,
    ) -> Result<Self, SchematicArrayError> {
        Self::new(kind, count.parse()?, naming.parse()?, placement)
    }

    pub const fn member_count(&self) -> usize {
        self.count.member_count()
    }

    pub const fn kind(&self) -> SchematicArrayKind {
        self.kind
    }

    pub const fn count(&self) -> SchematicArrayCount {
        self.count
    }

    pub const fn naming(&self) -> &SchematicArrayNaming {
        &self.naming
    }

    pub const fn placement(&self) -> SchematicArrayPlacement {
        self.placement
    }

    pub const fn replica_count(&self) -> usize {
        self.count.replica_count()
    }

    pub const fn is_electrical(&self) -> bool {
        self.kind.is_electrical()
    }
}

/// Exact additions and topology effect produced by an array candidate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchematicArrayImpact {
    pub members: usize,
    pub replicas: usize,
    pub components: usize,
    pub wires: usize,
    pub junctions: usize,
    pub buses: usize,
    pub bus_taps: usize,
    pub net_labels: usize,
    pub design_notes: usize,
    pub documentation_shapes: usize,
    pub connections: usize,
    pub electrical: bool,
}

impl SchematicArrayImpact {
    pub const fn is_empty(self) -> bool {
        self.components == 0
            && self.wires == 0
            && self.junctions == 0
            && self.buses == 0
            && self.bus_taps == 0
            && self.net_labels == 0
            && self.design_notes == 0
            && self.documentation_shapes == 0
            && self.connections == 0
    }

    pub fn added_object_count(self) -> usize {
        self.components
            .saturating_add(self.wires)
            .saturating_add(self.junctions)
            .saturating_add(self.buses)
            .saturating_add(self.bus_taps)
            .saturating_add(self.net_labels)
            .saturating_add(self.design_notes)
            .saturating_add(self.documentation_shapes)
    }
}

/// Immutable candidate additions used by both canvas preview and atomic commit.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SchematicArrayPreview {
    pub(crate) impact: SchematicArrayImpact,
    pub(crate) components: Vec<Component>,
    pub(crate) wires: Vec<Wire>,
    pub(crate) junctions: Vec<Junction>,
    pub(crate) buses: Vec<Bus>,
    pub(crate) bus_taps: Vec<BusTap>,
    pub(crate) net_labels: Vec<NetLabel>,
    pub(crate) design_notes: Vec<DesignNote>,
    pub(crate) documentation_shapes: Vec<DocumentationShape>,
    pub(crate) connections: Vec<WireConnection>,
    pub(crate) selection: Selection,
}

impl SchematicArrayPreview {
    pub fn is_empty(&self) -> bool {
        self.impact.is_empty()
    }

    pub const fn impact(&self) -> SchematicArrayImpact {
        self.impact
    }

    pub fn components(&self) -> &[Component] {
        &self.components
    }

    pub fn wires(&self) -> &[Wire] {
        &self.wires
    }

    pub fn junctions(&self) -> &[Junction] {
        &self.junctions
    }

    pub fn buses(&self) -> &[Bus] {
        &self.buses
    }

    pub fn bus_taps(&self) -> &[BusTap] {
        &self.bus_taps
    }

    pub fn net_labels(&self) -> &[NetLabel] {
        &self.net_labels
    }

    pub fn design_notes(&self) -> &[DesignNote] {
        &self.design_notes
    }

    pub fn documentation_shapes(&self) -> &[DocumentationShape] {
        &self.documentation_shapes
    }

    pub fn connections(&self) -> &[WireConnection] {
        &self.connections
    }

    pub const fn selection(&self) -> &Selection {
        &self.selection
    }
}

/// Validation or transaction failure. Every variant is non-mutating.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchematicArrayError {
    InvalidCountFormat {
        input: String,
    },
    ZeroCountDimension,
    CountOverflow,
    CountExceedsLimit {
        members: usize,
        maximum: usize,
    },
    GeneratedObjectBudgetExceeded {
        requested: usize,
        maximum: usize,
    },
    GeneratedSegmentBudgetExceeded {
        requested: usize,
        maximum: usize,
    },
    AtLeastTwoMembersRequired,
    LinearCountRequiresOneAxis,
    RectangularCountRequiresTwoAxes,
    RadialCountRequiresOneRow,
    PlacementKindMismatch,
    ZeroPitch,
    RectangularPitchRequiresTwoAxes,
    InvalidNamingClause {
        clause: String,
        reason: &'static str,
    },
    MismatchedNamingEndpoints {
        clause: String,
    },
    NamingRangeLengthOverflow,
    DuplicateNamingSource {
        source: String,
    },
    NamingRangeLengthMismatch {
        source: String,
        actual: usize,
        expected: usize,
    },
    EmptySelection,
    PartialSelection,
    UnsupportedSelection,
    ProbeSelectionUnsupported,
    ReadOnly,
    StaleSelection {
        object_id: u64,
    },
    RadialDocumentationOnly {
        object_id: u64,
    },
    CoordinateOverflow,
    IdentifierExhausted,
    DuplicateIdentity(u64),
    InvalidConnection(u64),
    AmbiguousTerminalContact(Point),
    InvalidSourceName {
        name: String,
    },
    MissingNamingRange {
        source: String,
    },
    UnmatchedNamingRange {
        source: String,
    },
    NameCollision {
        name: String,
    },
    InvalidGeometry {
        object_id: u64,
    },
    InvalidBusTap {
        tap_id: u64,
    },
    GeometryCollision {
        object_id: u64,
        other_id: u64,
    },
    UnintendedContact {
        object_id: u64,
        other_id: u64,
        point: Point,
    },
    CommitFailed,
}

impl fmt::Display for SchematicArrayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCountFormat { input } => write!(
                formatter,
                "'{input}' is not an array count; use the exact form 'columns × rows'."
            ),
            Self::ZeroCountDimension => formatter.write_str("Array dimensions must be non-zero."),
            Self::CountOverflow => formatter.write_str("The array member count overflows this runtime."),
            Self::CountExceedsLimit { members, maximum } => write!(
                formatter,
                "The array contains {members} members; the maximum is {maximum}."
            ),
            Self::GeneratedObjectBudgetExceeded { requested, maximum } => write!(
                formatter,
                "The array would generate {requested} objects; the transaction maximum is {maximum}."
            ),
            Self::GeneratedSegmentBudgetExceeded { requested, maximum } => write!(
                formatter,
                "The array would generate {requested} geometry segments; the transaction maximum is {maximum}."
            ),
            Self::AtLeastTwoMembersRequired => {
                formatter.write_str("An array must contain at least two members.")
            }
            Self::LinearCountRequiresOneAxis => formatter.write_str(
                "A linear array count must have exactly one dimension greater than one.",
            ),
            Self::RectangularCountRequiresTwoAxes => formatter.write_str(
                "A rectangular array count must have both dimensions greater than one.",
            ),
            Self::RadialCountRequiresOneRow => {
                formatter.write_str("A radial documentation array count must use N × 1.")
            }
            Self::PlacementKindMismatch => formatter
                .write_str("The placement input does not match the selected array kind."),
            Self::ZeroPitch => formatter.write_str("Array pitch must not be zero."),
            Self::RectangularPitchRequiresTwoAxes => formatter.write_str(
                "Rectangular array pitch must have non-zero horizontal and vertical axes.",
            ),
            Self::InvalidNamingClause { clause, reason } => {
                write!(formatter, "Invalid array naming clause '{clause}': {reason}.")
            }
            Self::MismatchedNamingEndpoints { clause } => write!(
                formatter,
                "Array naming endpoints in '{clause}' do not describe the same identifier pattern."
            ),
            Self::NamingRangeLengthOverflow => {
                formatter.write_str("The naming range length overflows this runtime.")
            }
            Self::DuplicateNamingSource { source } => {
                write!(formatter, "More than one naming range starts at '{source}'.")
            }
            Self::NamingRangeLengthMismatch {
                source,
                actual,
                expected,
            } => write!(
                formatter,
                "Naming range '{source}' contains {actual} values; the array requires {expected}."
            ),
            Self::EmptySelection => formatter.write_str("Select at least one complete object to array."),
            Self::PartialSelection => formatter.write_str(
                "Array creation requires whole objects; segment and vertex selections are not supported.",
            ),
            Self::UnsupportedSelection => formatter.write_str(
                "The current selection contains an object that cannot participate in an array.",
            ),
            Self::ProbeSelectionUnsupported => formatter.write_str(
                "Probe markers cannot participate in arrays; duplicate or reposition each retained probe explicitly.",
            ),
            Self::ReadOnly => formatter.write_str(
                "The active schematic is read-only and cannot accept an array transaction.",
            ),
            Self::StaleSelection { object_id } => write!(
                formatter,
                "Selected object {object_id} no longer exists in the schematic."
            ),
            Self::RadialDocumentationOnly { object_id } => write!(
                formatter,
                "Object {object_id} is electrical and cannot be used in a radial documentation array."
            ),
            Self::CoordinateOverflow => formatter.write_str(
                "The array would exceed the supported schematic coordinate range.",
            ),
            Self::IdentifierExhausted => {
                formatter.write_str("No stable schematic object identifiers remain available.")
            }
            Self::DuplicateIdentity(object_id) => write!(
                formatter,
                "Generated object identity {object_id} is already in use."
            ),
            Self::InvalidConnection(connection_index) => write!(
                formatter,
                "Generated connection {connection_index} does not reference valid candidate objects."
            ),
            Self::AmbiguousTerminalContact(point) => write!(
                formatter,
                "More than one terminal would own the generated contact at ({}, {}).",
                point.x, point.y
            ),
            Self::InvalidSourceName { name } => {
                write!(formatter, "Source name '{name}' cannot be sequenced safely.")
            }
            Self::MissingNamingRange { source } => {
                write!(formatter, "No array naming range starts at source '{source}'.")
            }
            Self::UnmatchedNamingRange { source } => write!(
                formatter,
                "Naming range '{source}' does not match a nameable source object."
            ),
            Self::NameCollision { name } => {
                write!(formatter, "Generated name '{name}' is already in use.")
            }
            Self::InvalidGeometry { object_id } => {
                write!(formatter, "Generated geometry for object {object_id} is invalid.")
            }
            Self::InvalidBusTap { tap_id } => write!(
                formatter,
                "Generated bus tap {tap_id} is not valid for its owning bus."
            ),
            Self::GeometryCollision {
                object_id,
                other_id,
            } => write!(
                formatter,
                "Generated object {object_id} collides with object {other_id}."
            ),
            Self::UnintendedContact {
                object_id,
                other_id,
                point,
            } => write!(
                formatter,
                "Generated object {object_id} would contact object {other_id} at ({}, {}).",
                point.x, point.y
            ),
            Self::CommitFailed => formatter.write_str(
                "The validated array candidate could not be committed atomically.",
            ),
        }
    }
}

impl std::error::Error for SchematicArrayError {}

fn validate_placement(
    kind: SchematicArrayKind,
    count: SchematicArrayCount,
    placement: SchematicArrayPlacement,
) -> Result<(), SchematicArrayError> {
    match (kind, placement) {
        (SchematicArrayKind::Linear, SchematicArrayPlacement::Pitch(delta)) => {
            if delta == Point::origin() {
                Err(SchematicArrayError::ZeroPitch)
            } else {
                Ok(())
            }
        }
        (SchematicArrayKind::Rectangular, SchematicArrayPlacement::Pitch(delta)) => {
            if delta.x == 0 || delta.y == 0 {
                Err(SchematicArrayError::RectangularPitchRequiresTwoAxes)
            } else if count.columns() <= 1 || count.rows() <= 1 {
                Err(SchematicArrayError::RectangularCountRequiresTwoAxes)
            } else {
                Ok(())
            }
        }
        (SchematicArrayKind::RadialDocumentation, SchematicArrayPlacement::Center(_)) => Ok(()),
        _ => Err(SchematicArrayError::PlacementKindMismatch),
    }
}

fn is_ascii_digits(input: &str) -> bool {
    !input.is_empty() && input.bytes().all(|byte| byte.is_ascii_digit())
}

fn leading_zero_width(digits: &str) -> Option<usize> {
    (digits.len() > 1 && digits.starts_with('0')).then_some(digits.len())
}

fn compatible_width(
    first: Option<usize>,
    first_value: u64,
    second: Option<usize>,
    second_value: u64,
) -> bool {
    match (first, second) {
        (None, None) => true,
        (Some(first), Some(second)) => first == second,
        (Some(width), None) => decimal_width(second_value) >= width,
        (None, Some(width)) => decimal_width(first_value) >= width,
    }
}

fn decimal_width(value: u64) -> usize {
    value.checked_ilog10().map_or(1, |power| power as usize + 1)
}

fn write_padded(
    formatter: &mut fmt::Formatter<'_>,
    value: u64,
    minimum_width: Option<usize>,
) -> fmt::Result {
    if let Some(width) = minimum_width {
        write!(formatter, "{value:0width$}")
    } else {
        write!(formatter, "{value}")
    }
}

fn invalid_naming_clause(clause: &str, reason: &'static str) -> SchematicArrayError {
    SchematicArrayError::InvalidNamingClause {
        clause: clause.trim().to_owned(),
        reason,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_parser_requires_the_mockup_multiplication_sign_and_round_trips() {
        let count = SchematicArrayCount::parse(" 8 × 1 ").unwrap();
        assert_eq!(count.columns(), 8);
        assert_eq!(count.rows(), 1);
        assert_eq!(count.member_count(), 8);
        assert_eq!(count.replica_count(), 7);
        assert_eq!(count.to_string(), "8 × 1");

        assert!(matches!(
            SchematicArrayCount::parse("8 x 1"),
            Err(SchematicArrayError::InvalidCountFormat { .. })
        ));
        assert_eq!(
            SchematicArrayCount::parse("0 × 8"),
            Err(SchematicArrayError::ZeroCountDimension)
        );
    }

    #[test]
    fn count_constructor_checks_arithmetic_and_transaction_limit() {
        assert_eq!(
            SchematicArrayCount::new(2_048, 2).unwrap().member_count(),
            MAX_SCHEMATIC_ARRAY_MEMBERS
        );
        assert_eq!(
            SchematicArrayCount::new(4_097, 1),
            Err(SchematicArrayError::CountExceedsLimit {
                members: 4_097,
                maximum: MAX_SCHEMATIC_ARRAY_MEMBERS,
            })
        );
        assert_eq!(
            SchematicArrayCount::new(usize::MAX, 2),
            Err(SchematicArrayError::CountOverflow)
        );
    }

    #[test]
    fn deserialization_cannot_construct_an_invalid_count_or_plan() {
        assert!(
            serde_json::from_value::<SchematicArrayCount>(
                serde_json::json!({ "columns": 0, "rows": 1 })
            )
            .is_err()
        );
        assert!(
            serde_json::from_value::<SchematicArrayCount>(serde_json::json!({
                "columns": usize::MAX,
                "rows": 2
            }))
            .is_err()
        );

        let invalid_plan = serde_json::json!({
            "kind": "linear",
            "count": { "columns": 2, "rows": 2 },
            "naming": { "ranges": [] },
            "placement": { "kind": "pitch", "point": { "x": 100, "y": 100 } }
        });
        assert!(serde_json::from_value::<SchematicArrayPlan>(invalid_plan).is_err());
    }

    #[test]
    fn count_validation_is_specific_to_each_array_kind() {
        SchematicArrayCount::new(8, 1)
            .unwrap()
            .validate_for(SchematicArrayKind::Linear)
            .unwrap();
        SchematicArrayCount::new(1, 8)
            .unwrap()
            .validate_for(SchematicArrayKind::Linear)
            .unwrap();
        assert_eq!(
            SchematicArrayCount::new(2, 2)
                .unwrap()
                .validate_for(SchematicArrayKind::Linear),
            Err(SchematicArrayError::LinearCountRequiresOneAxis)
        );
        SchematicArrayCount::new(4, 3)
            .unwrap()
            .validate_for(SchematicArrayKind::Rectangular)
            .unwrap();
        assert_eq!(
            SchematicArrayCount::new(8, 1)
                .unwrap()
                .validate_for(SchematicArrayKind::Rectangular),
            Err(SchematicArrayError::RectangularCountRequiresTwoAxes)
        );
        assert_eq!(
            SchematicArrayCount::new(1, 8)
                .unwrap()
                .validate_for(SchematicArrayKind::RadialDocumentation),
            Err(SchematicArrayError::RadialCountRequiresOneRow)
        );
    }

    #[test]
    fn exact_mockup_naming_parses_round_trips_and_materializes() {
        let naming = SchematicArrayNaming::parse("U4…U11 · DATA[0]…DATA[7]").unwrap();
        assert_eq!(naming.len(), 2);
        assert!(naming.contains_source("u4"));
        assert!(naming.contains_source("DATA[0]"));
        assert_eq!(naming.value_for_source("U4", 0).as_deref(), Some("U4"));
        assert_eq!(naming.value_for_source("U4", 7).as_deref(), Some("U11"));
        assert_eq!(
            naming.value_for_source("DATA[0]", 6).as_deref(),
            Some("DATA[6]")
        );
        assert_eq!(naming.value_at(1, 7).as_deref(), Some("DATA[7]"));
        assert_eq!(naming.value_at(1, 8), None);
        assert_eq!(naming.to_string(), "U4…U11 · DATA[0]…DATA[7]");
    }

    #[test]
    fn naming_supports_descending_and_zero_padded_sequences() {
        let naming = SchematicArrayNaming::parse("R011…R004 · A[07]…A[00]").unwrap();
        assert_eq!(naming.value_for_source("R011", 3).as_deref(), Some("R008"));
        assert_eq!(
            naming.value_for_source("A[07]", 7).as_deref(),
            Some("A[00]")
        );
        assert_eq!(naming.to_string(), "R011…R004 · A[07]…A[00]");
    }

    #[test]
    fn naming_normalizes_integral_interleaved_stride_for_group_arrays() {
        let naming = SchematicArrayNaming::parse("R1…R15 · R2…R16").unwrap();
        naming.validate_for_members(8).unwrap();
        let plan = SchematicArrayPlan::new(
            SchematicArrayKind::Linear,
            SchematicArrayCount::new(8, 1).unwrap(),
            naming,
            SchematicArrayPlacement::Pitch(Point::new(100, 0)),
        )
        .unwrap();
        assert_eq!(
            plan.naming.value_for_source("R1", 7).as_deref(),
            Some("R15")
        );
        assert_eq!(plan.naming.value_for_source("R2", 3).as_deref(), Some("R8"));
    }

    #[test]
    fn empty_naming_is_valid_but_ambiguous_or_duplicate_clauses_are_not() {
        let empty = SchematicArrayNaming::parse("   ").unwrap();
        assert!(empty.is_empty());
        assert_eq!(empty.to_string(), "");

        assert!(matches!(
            SchematicArrayNaming::parse("U4...U11"),
            Err(SchematicArrayError::InvalidNamingClause { .. })
        ));
        assert!(matches!(
            SchematicArrayNaming::parse("U4…R11"),
            Err(SchematicArrayError::MismatchedNamingEndpoints { .. })
        ));
        assert!(matches!(
            SchematicArrayNaming::parse("U4…U11 · u4…u11"),
            Err(SchematicArrayError::DuplicateNamingSource { .. })
        ));
        let padded = SchematicArrayNaming::parse("U04…U11").unwrap();
        assert_eq!(padded.value_for_source("U04", 1).as_deref(), Some("U05"));
    }

    #[test]
    fn plan_validates_naming_cardinality_and_placement() {
        let count = SchematicArrayCount::new(8, 1).unwrap();
        let naming = SchematicArrayNaming::parse("U4…U11").unwrap();
        let plan = SchematicArrayPlan::new(
            SchematicArrayKind::Linear,
            count,
            naming,
            SchematicArrayPlacement::Pitch(Point::new(30, 0)),
        )
        .unwrap();
        assert_eq!(plan.member_count(), 8);
        assert_eq!(plan.replica_count(), 7);
        assert!(plan.is_electrical());

        assert_eq!(
            SchematicArrayPlan::new(
                SchematicArrayKind::Linear,
                count,
                SchematicArrayNaming::parse("U4…U10").unwrap(),
                SchematicArrayPlacement::Pitch(Point::new(30, 0)),
            ),
            Err(SchematicArrayError::NamingRangeLengthMismatch {
                source: "U4".to_owned(),
                actual: 7,
                expected: 8,
            })
        );
        assert!(matches!(
            SchematicArrayPlan::new(
                SchematicArrayKind::Linear,
                count,
                SchematicArrayNaming::default(),
                SchematicArrayPlacement::Center(Point::origin()),
            ),
            Err(SchematicArrayError::PlacementKindMismatch)
        ));
    }

    #[test]
    fn radial_documentation_accepts_empty_naming_and_a_center() {
        let plan = SchematicArrayPlan::parse(
            SchematicArrayKind::RadialDocumentation,
            "8 × 1",
            "",
            SchematicArrayPlacement::Center(Point::new(100, 100)),
        )
        .unwrap();
        assert!(!plan.is_electrical());
    }

    #[test]
    fn preview_default_is_empty() {
        assert!(SchematicArrayPreview::default().is_empty());
    }
}

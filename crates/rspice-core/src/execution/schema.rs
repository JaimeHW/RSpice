use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use super::plan::RunCoordinateId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SignalKind {
    Voltage,
    Current,
    DeviceObservable,
    Scalar,
    /// A digital event node, projected onto the analysis time grid as a
    /// `D(node)` column.
    ///
    /// This kind describes the *flattened* signal, not the event history it
    /// came from. `transient_projection_signals` holds each committed value
    /// until the next recorded event and writes one `f64` per analysis time
    /// point: `0.0`, `1.0`, or `0.5` for anything the value does not resolve
    /// to a level. So the twelve `state × strength` combinations the engine
    /// resolves reach a table as three numbers, the drive strength is gone,
    /// and `x` and `z` are indistinguishable once written.
    ///
    /// Real-valued event nets have no counterpart here at all: the projection
    /// walks the digital histories only, so nothing carrying
    /// `SignalKind::Digital` ever stands for one.
    ///
    /// The lossless routes past this flattening are the typed result document
    /// (`payload.digitalTraces` / `payload.realTraces`, which keep state,
    /// strength and the event's own time), the rawfile event plots
    /// [`crate::io::write_event_plots`] appends, and a VCD dump
    /// [`crate::execution::event_vcd_document`] projects — of which VCD keeps
    /// the level and the timeline but not the strength, because the format
    /// has four bit states and no strength to put one in.
    Digital,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SignalValueType {
    Real,
    Complex,
    /// A logic level carried in the same `f64` column an analog signal uses.
    ///
    /// It marks the column as digital for a consumer that renders or exports
    /// it — a rawfile writes it under variable type `digital` — but it does
    /// not widen the column: the values are the three levels
    /// [`SignalKind::Digital`] describes, not a four-state or twelve-state
    /// encoding.
    Logic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SignalShape {
    Scalar,
    Vector,
    Matrix,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SignalUnit {
    Volt,
    Ampere,
    Ohm,
    Siemens,
    Watt,
    Hertz,
    Second,
    Degree,
    Radian,
    Dimensionless,
    Logic,
    /// The producing analysis knows of no physical unit for this quantity.
    ///
    /// This is not [`Self::Dimensionless`]: a reflection coefficient really is
    /// a pure ratio, while a `.DC` sweep over a user parameter or a `.FOUR` of
    /// a braced expression has a unit the deck simply never declared. Folding
    /// the second case onto the first asserts a physical fact nobody stated,
    /// and spelling it `Custom("unspecified")` invents a symbol that no
    /// consumer can distinguish from a real one.
    Unspecified,
    Custom(String),
}

impl SignalUnit {
    /// Short symbol a published artifact spells this unit with.
    ///
    /// One table, because two surfaces that disagree on how a volt is written
    /// publish two files a reader cannot compare: the engine adapter's
    /// measurement manifest and the command line's HDF5 columns both come
    /// through here.
    ///
    /// `SignalUnit` is `#[non_exhaustive]`, so a future core unit renders as
    /// its own lower-case tag rather than being folded onto an existing
    /// symbol: an unrecognized unit must never be reported as a different one.
    #[must_use]
    pub fn symbol(&self) -> String {
        match self {
            Self::Volt => "V".to_owned(),
            Self::Ampere => "A".to_owned(),
            Self::Ohm => "ohm".to_owned(),
            Self::Siemens => "S".to_owned(),
            Self::Watt => "W".to_owned(),
            Self::Hertz => "Hz".to_owned(),
            Self::Second => "s".to_owned(),
            Self::Degree => "deg".to_owned(),
            Self::Radian => "rad".to_owned(),
            Self::Dimensionless => "1".to_owned(),
            Self::Logic => "logic".to_owned(),
            Self::Custom(symbol) => symbol.clone(),
            other => format!("{other:?}").to_ascii_lowercase(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum SignalOwner {
    Node(String),
    Branch(String),
    Device(String),
    Analysis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalDescriptor {
    canonical_name: String,
    display_name: String,
    kind: SignalKind,
    unit: SignalUnit,
    value_type: SignalValueType,
    shape: SignalShape,
    owner: SignalOwner,
}

impl SignalDescriptor {
    pub fn new(
        canonical_name: impl Into<String>,
        display_name: impl Into<String>,
        kind: SignalKind,
        unit: SignalUnit,
        value_type: SignalValueType,
        shape: SignalShape,
        owner: SignalOwner,
    ) -> Result<Self, SignalSchemaError> {
        let canonical_name = canonical_name.into();
        let display_name = display_name.into();
        if canonical_name.trim().is_empty() {
            return Err(SignalSchemaError::EmptyCanonicalName);
        }
        if display_name.trim().is_empty() {
            return Err(SignalSchemaError::EmptyDisplayName);
        }
        validate_descriptor(kind, &unit, value_type, shape, &owner)?;
        Ok(Self {
            canonical_name: canonical_name.trim().to_ascii_lowercase(),
            display_name: display_name.trim().to_string(),
            kind,
            unit: canonicalize_unit(unit)?,
            value_type,
            shape,
            owner: canonicalize_owner(owner)?,
        })
    }

    pub fn canonical_name(&self) -> &str {
        &self.canonical_name
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub const fn kind(&self) -> SignalKind {
        self.kind
    }

    pub const fn unit(&self) -> &SignalUnit {
        &self.unit
    }

    pub const fn value_type(&self) -> SignalValueType {
        self.value_type
    }

    pub const fn shape(&self) -> SignalShape {
        self.shape
    }

    pub const fn owner(&self) -> &SignalOwner {
        &self.owner
    }

    fn identity(&self) -> SignalIdentity {
        SignalIdentity {
            kind: self.kind,
            canonical_name: self.canonical_name.clone(),
        }
    }
}

fn canonicalize_owner(owner: SignalOwner) -> Result<SignalOwner, SignalSchemaError> {
    let canonical = |value: String| {
        if value.trim().is_empty() {
            Err(SignalSchemaError::EmptyOwner)
        } else {
            Ok(value.trim().to_ascii_lowercase())
        }
    };
    match owner {
        SignalOwner::Node(name) => canonical(name).map(SignalOwner::Node),
        SignalOwner::Branch(name) => canonical(name).map(SignalOwner::Branch),
        SignalOwner::Device(name) => canonical(name).map(SignalOwner::Device),
        SignalOwner::Analysis => Ok(SignalOwner::Analysis),
    }
}

fn canonicalize_unit(unit: SignalUnit) -> Result<SignalUnit, SignalSchemaError> {
    match unit {
        SignalUnit::Custom(name) if name.trim().is_empty() => {
            Err(SignalSchemaError::EmptyCustomUnit)
        }
        SignalUnit::Custom(name) => Ok(SignalUnit::Custom(name.trim().to_string())),
        other => Ok(other),
    }
}

fn validate_descriptor(
    kind: SignalKind,
    unit: &SignalUnit,
    value_type: SignalValueType,
    shape: SignalShape,
    owner: &SignalOwner,
) -> Result<(), SignalSchemaError> {
    let valid = match kind {
        SignalKind::Voltage => {
            *unit == SignalUnit::Volt
                && matches!(value_type, SignalValueType::Real | SignalValueType::Complex)
                && matches!(shape, SignalShape::Scalar | SignalShape::Vector)
                && matches!(owner, SignalOwner::Node(_))
        }
        SignalKind::Current => {
            *unit == SignalUnit::Ampere
                && matches!(value_type, SignalValueType::Real | SignalValueType::Complex)
                && matches!(shape, SignalShape::Scalar | SignalShape::Vector)
                && matches!(owner, SignalOwner::Branch(_))
        }
        SignalKind::DeviceObservable => {
            !matches!(unit, SignalUnit::Logic)
                && matches!(value_type, SignalValueType::Real | SignalValueType::Complex)
                && matches!(owner, SignalOwner::Device(_))
        }
        SignalKind::Scalar => {
            !matches!(unit, SignalUnit::Logic)
                && matches!(value_type, SignalValueType::Real | SignalValueType::Complex)
                && matches!(owner, SignalOwner::Analysis)
        }
        SignalKind::Digital => {
            *unit == SignalUnit::Logic
                && value_type == SignalValueType::Logic
                && matches!(shape, SignalShape::Scalar | SignalShape::Vector)
                && matches!(owner, SignalOwner::Node(_))
        }
    };
    if valid {
        Ok(())
    } else {
        Err(SignalSchemaError::InvalidDescriptor {
            kind,
            unit: unit.clone(),
            value_type,
            shape,
            owner: owner.clone(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SignalIdentity {
    kind: SignalKind,
    canonical_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SignalSchema {
    descriptors: Vec<SignalDescriptor>,
}

impl SignalSchema {
    pub fn new(descriptors: Vec<SignalDescriptor>) -> Result<Self, SignalSchemaError> {
        let mut identities = BTreeMap::new();
        for (index, descriptor) in descriptors.iter().enumerate() {
            let identity = descriptor.identity();
            if let Some(first_index) = identities.insert(identity.clone(), index) {
                return Err(SignalSchemaError::DuplicateSignal {
                    name: identity.canonical_name,
                    first_index,
                    duplicate_index: index,
                });
            }
        }
        Ok(Self { descriptors })
    }

    pub fn descriptors(&self) -> &[SignalDescriptor] {
        &self.descriptors
    }

    pub fn union<'a>(
        sources: impl IntoIterator<Item = CoordinateSchema<'a>>,
    ) -> Result<SchemaUnion, SignalSchemaError> {
        let sources = sources.into_iter().collect::<Vec<_>>();
        let mut descriptors_by_identity = BTreeMap::<SignalIdentity, SignalDescriptor>::new();
        let mut coordinate_ids = BTreeSet::new();

        for source in &sources {
            if !coordinate_ids.insert(source.coordinate) {
                return Err(SignalSchemaError::DuplicateCoordinate(source.coordinate));
            }
            for descriptor in &source.schema.descriptors {
                let identity = descriptor.identity();
                if let Some(existing) = descriptors_by_identity.get_mut(&identity) {
                    if !descriptors_are_compatible(existing, descriptor) {
                        return Err(SignalSchemaError::DescriptorConflict {
                            name: descriptor.canonical_name.clone(),
                        });
                    }
                    if descriptor.display_name < existing.display_name {
                        existing.display_name = descriptor.display_name.clone();
                    }
                } else {
                    descriptors_by_identity.insert(identity, descriptor.clone());
                }
            }
        }

        let union_identities = descriptors_by_identity.keys().cloned().collect::<Vec<_>>();
        let union_descriptors = descriptors_by_identity.into_values().collect::<Vec<_>>();
        let mut source_indices = BTreeMap::new();
        for source in sources {
            let local_indices = source
                .schema
                .descriptors
                .iter()
                .enumerate()
                .map(|(index, descriptor)| (descriptor.identity(), index))
                .collect::<BTreeMap<_, _>>();
            let mapping = union_identities
                .iter()
                .map(|identity| local_indices.get(identity).copied())
                .collect::<Vec<_>>();
            source_indices.insert(source.coordinate, mapping);
        }

        Ok(SchemaUnion {
            schema: SignalSchema {
                descriptors: union_descriptors,
            },
            source_indices,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CoordinateSchema<'a> {
    coordinate: RunCoordinateId,
    schema: &'a SignalSchema,
}

impl<'a> CoordinateSchema<'a> {
    pub const fn new(coordinate: RunCoordinateId, schema: &'a SignalSchema) -> Self {
        Self { coordinate, schema }
    }

    pub const fn coordinate(self) -> RunCoordinateId {
        self.coordinate
    }

    pub const fn schema(self) -> &'a SignalSchema {
        self.schema
    }
}

fn descriptors_are_compatible(first: &SignalDescriptor, second: &SignalDescriptor) -> bool {
    first.canonical_name == second.canonical_name
        && first
            .display_name
            .eq_ignore_ascii_case(&second.display_name)
        && first.kind == second.kind
        && first.unit == second.unit
        && first.value_type == second.value_type
        && first.shape == second.shape
        && first.owner == second.owner
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchemaUnion {
    schema: SignalSchema,
    source_indices: BTreeMap<RunCoordinateId, Vec<Option<usize>>>,
}

impl SchemaUnion {
    pub const fn schema(&self) -> &SignalSchema {
        &self.schema
    }

    /// Union-column to source-local index maps keyed by stable coordinate ID.
    /// A `None` entry is explicit missingness and must never become zero.
    pub fn source_indices(&self) -> &BTreeMap<RunCoordinateId, Vec<Option<usize>>> {
        &self.source_indices
    }

    pub fn align_values<T: Clone>(
        &self,
        coordinate: RunCoordinateId,
        source_values: &[T],
    ) -> Result<Vec<Option<T>>, SignalSchemaError> {
        let indices = self
            .source_indices
            .get(&coordinate)
            .ok_or(SignalSchemaError::UnknownCoordinate(coordinate))?;
        let expected = indices.iter().flatten().count();
        if source_values.len() != expected {
            return Err(SignalSchemaError::SourceValueCount {
                coordinate,
                expected,
                actual: source_values.len(),
            });
        }
        Ok(indices
            .iter()
            .map(|index| index.and_then(|index| source_values.get(index).cloned()))
            .collect())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SignalSchemaError {
    EmptyCanonicalName,
    EmptyDisplayName,
    EmptyOwner,
    EmptyCustomUnit,
    InvalidDescriptor {
        kind: SignalKind,
        unit: SignalUnit,
        value_type: SignalValueType,
        shape: SignalShape,
        owner: SignalOwner,
    },
    DuplicateSignal {
        name: String,
        first_index: usize,
        duplicate_index: usize,
    },
    DescriptorConflict {
        name: String,
    },
    DuplicateCoordinate(RunCoordinateId),
    UnknownCoordinate(RunCoordinateId),
    SourceValueCount {
        coordinate: RunCoordinateId,
        expected: usize,
        actual: usize,
    },
}

impl fmt::Display for SignalSchemaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCanonicalName => formatter.write_str("signal canonical name is empty"),
            Self::EmptyDisplayName => formatter.write_str("signal display name is empty"),
            Self::EmptyOwner => formatter.write_str("signal owner name is empty"),
            Self::EmptyCustomUnit => formatter.write_str("custom signal unit is empty"),
            Self::InvalidDescriptor {
                kind,
                unit,
                value_type,
                shape,
                owner,
            } => write!(
                formatter,
                "invalid signal descriptor combination: kind={kind:?}, unit={unit:?}, value_type={value_type:?}, shape={shape:?}, owner={owner:?}"
            ),
            Self::DuplicateSignal {
                name,
                first_index,
                duplicate_index,
            } => write!(
                formatter,
                "signal '{name}' is duplicated at schema indices {first_index} and {duplicate_index}"
            ),
            Self::DescriptorConflict { name } => write!(
                formatter,
                "signal '{name}' has conflicting descriptors across coordinate schemas"
            ),
            Self::DuplicateCoordinate(coordinate) => {
                write!(formatter, "duplicate coordinate schema '{coordinate}'")
            }
            Self::UnknownCoordinate(coordinate) => {
                write!(
                    formatter,
                    "coordinate schema '{coordinate}' is not in this union"
                )
            }
            Self::SourceValueCount {
                coordinate,
                expected,
                actual,
            } => write!(
                formatter,
                "coordinate '{coordinate}' supplied {actual} values for a {expected}-signal source schema"
            ),
        }
    }
}

impl std::error::Error for SignalSchemaError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn voltage(name: &str) -> SignalDescriptor {
        SignalDescriptor::new(
            format!("v({name})"),
            format!("V({name})"),
            SignalKind::Voltage,
            SignalUnit::Volt,
            SignalValueType::Real,
            SignalShape::Vector,
            SignalOwner::Node(name.to_string()),
        )
        .expect("valid voltage descriptor")
    }

    #[test]
    fn schema_equality_is_exact_and_ordered() {
        let first = SignalSchema::new(vec![voltage("a"), voltage("b")]).expect("schema");
        let same = SignalSchema::new(vec![voltage("a"), voltage("b")]).expect("schema");
        let reordered = SignalSchema::new(vec![voltage("b"), voltage("a")]).expect("schema");
        assert_eq!(first, same);
        assert_ne!(first, reordered);
    }

    #[test]
    fn schema_union_preserves_identity_and_explicit_missingness() {
        let first = SignalSchema::new(vec![voltage("a"), voltage("b")]).expect("first schema");
        let second = SignalSchema::new(vec![voltage("b"), voltage("c")]).expect("second schema");
        let first_id = RunCoordinateId::from_parts([1; 16], 0);
        let second_id = RunCoordinateId::from_parts([2; 16], 0);
        let union = SignalSchema::union([
            CoordinateSchema::new(first_id, &first),
            CoordinateSchema::new(second_id, &second),
        ])
        .expect("compatible union");
        let names = union
            .schema()
            .descriptors()
            .iter()
            .map(SignalDescriptor::canonical_name)
            .collect::<Vec<_>>();
        assert_eq!(names, ["v(a)", "v(b)", "v(c)"]);
        assert_eq!(
            union.source_indices().get(&first_id),
            Some(&vec![Some(0), Some(1), None])
        );
        assert_eq!(
            union.source_indices().get(&second_id),
            Some(&vec![None, Some(0), Some(1)])
        );
        assert_eq!(
            union
                .align_values(second_id, &[20.0, 30.0])
                .expect("values align by source-local identity"),
            vec![None, Some(20.0), Some(30.0)]
        );
    }

    #[test]
    fn case_variants_share_identity_but_conflicting_metadata_fails() {
        let lower = voltage("out");
        let upper = SignalDescriptor::new(
            "V(OUT)",
            "V(OUT)",
            SignalKind::Voltage,
            SignalUnit::Volt,
            SignalValueType::Real,
            SignalShape::Vector,
            SignalOwner::Node("OUT".to_string()),
        )
        .expect("case variant");
        let first = SignalSchema::new(vec![lower]).expect("first");
        let second = SignalSchema::new(vec![upper]).expect("second");
        let first_id = RunCoordinateId::from_parts([1; 16], 0);
        let second_id = RunCoordinateId::from_parts([2; 16], 0);
        let union = SignalSchema::union([
            CoordinateSchema::new(first_id, &first),
            CoordinateSchema::new(second_id, &second),
        ])
        .expect("case-compatible union");
        assert_eq!(union.schema().descriptors().len(), 1);

        let wrong_unit = SignalDescriptor::new(
            "V(out)",
            "V(out)",
            SignalKind::Voltage,
            SignalUnit::Volt,
            SignalValueType::Real,
            SignalShape::Scalar,
            SignalOwner::Node("out".to_string()),
        )
        .expect("structurally valid conflicting descriptor");
        let conflicting = SignalSchema::new(vec![wrong_unit]).expect("conflicting schema");
        assert!(matches!(
            SignalSchema::union([
                CoordinateSchema::new(first_id, &first),
                CoordinateSchema::new(second_id, &conflicting)
            ]),
            Err(SignalSchemaError::DescriptorConflict { .. })
        ));
    }

    #[test]
    fn reordered_local_schemas_align_values_without_swapping() {
        let first = SignalSchema::new(vec![voltage("a"), voltage("b")]).expect("first");
        let second = SignalSchema::new(vec![voltage("b"), voltage("a")]).expect("second");
        let first_id = RunCoordinateId::from_parts([1; 16], 0);
        let second_id = RunCoordinateId::from_parts([2; 16], 0);
        let union = SignalSchema::union([
            CoordinateSchema::new(second_id, &second),
            CoordinateSchema::new(first_id, &first),
        ])
        .expect("reordered schemas are compatible");
        assert_eq!(
            union
                .align_values(first_id, &[1.0, 2.0])
                .expect("first values"),
            vec![Some(1.0), Some(2.0)]
        );
        assert_eq!(
            union
                .align_values(second_id, &[20.0, 10.0])
                .expect("second values"),
            vec![Some(10.0), Some(20.0)]
        );
    }

    #[test]
    fn every_core_unit_has_a_distinct_non_empty_symbol() {
        let units = [
            SignalUnit::Volt,
            SignalUnit::Ampere,
            SignalUnit::Ohm,
            SignalUnit::Siemens,
            SignalUnit::Watt,
            SignalUnit::Hertz,
            SignalUnit::Second,
            SignalUnit::Degree,
            SignalUnit::Radian,
            SignalUnit::Dimensionless,
            SignalUnit::Logic,
            SignalUnit::Unspecified,
            SignalUnit::Custom("V^2/Hz".to_owned()),
        ];
        let mut seen = std::collections::BTreeSet::new();
        for unit in &units {
            let symbol = unit.symbol();
            assert!(!symbol.trim().is_empty(), "{unit:?} has no symbol");
            assert!(seen.insert(symbol), "{unit:?} shares a symbol");
        }
    }
}

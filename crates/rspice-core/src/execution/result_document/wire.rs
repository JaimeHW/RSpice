//! JSON adapters for the core identity and schema types the document carries.
//!
//! The document stores the real `rspice-core` types so that a frontend cannot
//! reinvent analysis identity, coordinate identity, or signal schema. Those
//! types deliberately have validating constructors and private fields, so this
//! module supplies the `serde` bridge: encoding projects the public accessors,
//! and decoding goes back through the validating constructor. A tag this build
//! does not know is an error, never a default.

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Value;
use crate::execution::capability::AnalysisResultKind;
use crate::execution::plan::{
    AnalysisInstanceId, AnalysisKind, AxisKind, DataBinding, RunAxisValue, RunCoordinateId,
    StepAxisTarget,
};
use crate::execution::schema::{
    SignalDescriptor, SignalKind, SignalOwner, SignalShape, SignalUnit, SignalValueType,
};
use crate::execution::topology::TopologyFingerprint;

/// Generate a `serde(with)` module for a fieldless enum with stable tags.
///
/// The generated `match` is exhaustive, so a new variant in the source enum
/// stops this crate from compiling until it has a wire tag.
macro_rules! tagged_enum_adapter {
    ($module:ident, $ty:ident, { $($variant:ident => $tag:literal),+ $(,)? }) => {
        pub(super) mod $module {
            use super::*;

            pub(in crate::execution::result_document) const fn tag(value: $ty) -> &'static str {
                match value {
                    $($ty::$variant => $tag,)+
                }
            }

            pub(in crate::execution::result_document) fn from_tag(tag: &str) -> Option<$ty> {
                match tag {
                    $($tag => Some($ty::$variant),)+
                    _ => None,
                }
            }

            pub fn serialize<S: Serializer>(value: &$ty, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(tag(*value))
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(
                deserializer: D,
            ) -> Result<$ty, D::Error> {
                let tag = String::deserialize(deserializer)?;
                from_tag(&tag).ok_or_else(|| {
                    D::Error::custom(format!(
                        "unknown {} tag {tag:?}",
                        stringify!($ty)
                    ))
                })
            }
        }
    };
}

tagged_enum_adapter!(analysis_kind, AnalysisKind, {
    ImplicitOp => "implicit-op",
    Op => "op",
    Dc => "dc",
    Ac => "ac",
    Tran => "tran",
    Noise => "noise",
    Sp => "sp",
    Stb => "stb",
    Distortion => "disto",
    PoleZero => "pz",
    Sensitivity => "sens",
    TransferFunction => "tf",
    Pss => "pss",
    Pac => "pac",
    PNoise => "pnoise",
    HarmonicBalance => "hb",
    Envelope => "env",
    MonteCarlo => "mc",
    Fourier => "four",
    Fft => "fft",
});

tagged_enum_adapter!(
    analysis_result_kind,
    AnalysisResultKind,
    {
        OperatingPoint => "op",
        DcSweep => "dc",
        Ac => "ac",
        Transient => "tran",
        Noise => "noise",
        SParameters => "sp",
        PortNoise => "port-noise",
        Distortion => "distortion",
        TransferFunction => "tf",
        Stability => "stb",
        Sensitivity => "sensitivity",
        PoleZero => "pole-zero",
        Fourier => "fourier",
        Fft => "fft",
        MonteCarlo => "monte-carlo",
        Pss => "pss",
        Pac => "pac",
        PNoise => "pnoise",
        HarmonicBalance => "hb",
        Envelope => "envelope",
    }
);

tagged_enum_adapter!(axis_kind, AxisKind, {
    Alter => "alter",
    Data => "data",
    Step => "step",
    Temperature => "temperature",
});

tagged_enum_adapter!(signal_kind, SignalKind, {
    Voltage => "voltage",
    Current => "current",
    DeviceObservable => "device_observable",
    Scalar => "scalar",
    Digital => "digital",
});

tagged_enum_adapter!(
    signal_value_type,
    SignalValueType,
    {
        Real => "real",
        Complex => "complex",
        Logic => "logic",
    }
);

tagged_enum_adapter!(signal_shape, SignalShape, {
    Scalar => "scalar",
    Vector => "vector",
    Matrix => "matrix",
});

//=============================================================================
// Signal units and owners
//=============================================================================

#[derive(Serialize, Deserialize)]
#[serde(tag = "unit", rename_all = "snake_case", deny_unknown_fields)]
enum SignalUnitWire {
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
    Unspecified,
    Custom { symbol: String },
}

impl From<&SignalUnit> for SignalUnitWire {
    fn from(unit: &SignalUnit) -> Self {
        match unit {
            SignalUnit::Volt => Self::Volt,
            SignalUnit::Ampere => Self::Ampere,
            SignalUnit::Ohm => Self::Ohm,
            SignalUnit::Siemens => Self::Siemens,
            SignalUnit::Watt => Self::Watt,
            SignalUnit::Hertz => Self::Hertz,
            SignalUnit::Second => Self::Second,
            SignalUnit::Degree => Self::Degree,
            SignalUnit::Radian => Self::Radian,
            SignalUnit::Dimensionless => Self::Dimensionless,
            SignalUnit::Logic => Self::Logic,
            SignalUnit::Unspecified => Self::Unspecified,
            SignalUnit::Custom(symbol) => Self::Custom {
                symbol: symbol.clone(),
            },
        }
    }
}

impl From<SignalUnitWire> for SignalUnit {
    fn from(unit: SignalUnitWire) -> Self {
        match unit {
            SignalUnitWire::Volt => Self::Volt,
            SignalUnitWire::Ampere => Self::Ampere,
            SignalUnitWire::Ohm => Self::Ohm,
            SignalUnitWire::Siemens => Self::Siemens,
            SignalUnitWire::Watt => Self::Watt,
            SignalUnitWire::Hertz => Self::Hertz,
            SignalUnitWire::Second => Self::Second,
            SignalUnitWire::Degree => Self::Degree,
            SignalUnitWire::Radian => Self::Radian,
            SignalUnitWire::Dimensionless => Self::Dimensionless,
            SignalUnitWire::Logic => Self::Logic,
            SignalUnitWire::Unspecified => Self::Unspecified,
            SignalUnitWire::Custom { symbol } => Self::Custom(symbol),
        }
    }
}

pub(super) mod signal_unit {
    use super::*;

    pub fn serialize<S: Serializer>(value: &SignalUnit, serializer: S) -> Result<S::Ok, S::Error> {
        SignalUnitWire::from(value).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<SignalUnit, D::Error> {
        Ok(SignalUnitWire::deserialize(deserializer)?.into())
    }
}

pub(super) mod optional_signal_unit {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &Option<SignalUnit>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(unit) => serializer.serialize_some(&SignalUnitWire::from(unit)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<SignalUnit>, D::Error> {
        Ok(Option::<SignalUnitWire>::deserialize(deserializer)?.map(SignalUnit::from))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "owner", rename_all = "snake_case", deny_unknown_fields)]
enum SignalOwnerWire {
    Node { name: String },
    Branch { name: String },
    Device { name: String },
    Analysis,
}

impl From<&SignalOwner> for SignalOwnerWire {
    fn from(owner: &SignalOwner) -> Self {
        match owner {
            SignalOwner::Node(name) => Self::Node { name: name.clone() },
            SignalOwner::Branch(name) => Self::Branch { name: name.clone() },
            SignalOwner::Device(name) => Self::Device { name: name.clone() },
            SignalOwner::Analysis => Self::Analysis,
        }
    }
}

impl From<SignalOwnerWire> for SignalOwner {
    fn from(owner: SignalOwnerWire) -> Self {
        match owner {
            SignalOwnerWire::Node { name } => Self::Node(name),
            SignalOwnerWire::Branch { name } => Self::Branch(name),
            SignalOwnerWire::Device { name } => Self::Device(name),
            SignalOwnerWire::Analysis => Self::Analysis,
        }
    }
}

//=============================================================================
// Signal descriptors
//=============================================================================

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SignalDescriptorWire {
    canonical_name: String,
    display_name: String,
    #[serde(with = "signal_kind")]
    kind: SignalKind,
    unit: SignalUnitWire,
    #[serde(with = "signal_value_type")]
    value_type: SignalValueType,
    #[serde(with = "signal_shape")]
    shape: SignalShape,
    owner: SignalOwnerWire,
}

impl From<&SignalDescriptor> for SignalDescriptorWire {
    fn from(descriptor: &SignalDescriptor) -> Self {
        Self {
            canonical_name: descriptor.canonical_name().to_owned(),
            display_name: descriptor.display_name().to_owned(),
            kind: descriptor.kind(),
            unit: SignalUnitWire::from(descriptor.unit()),
            value_type: descriptor.value_type(),
            shape: descriptor.shape(),
            owner: SignalOwnerWire::from(descriptor.owner()),
        }
    }
}

pub(super) mod signal_descriptor {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &SignalDescriptor,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        SignalDescriptorWire::from(value).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<SignalDescriptor, D::Error> {
        let wire = SignalDescriptorWire::deserialize(deserializer)?;
        SignalDescriptor::new(
            wire.canonical_name,
            wire.display_name,
            wire.kind,
            SignalUnit::from(wire.unit),
            wire.value_type,
            wire.shape,
            SignalOwner::from(wire.owner),
        )
        .map_err(D::Error::custom)
    }
}

//=============================================================================
// Analysis and coordinate identity
//=============================================================================

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct AnalysisInstanceIdWire {
    #[serde(with = "analysis_kind")]
    kind: AnalysisKind,
    ordinal: u32,
    /// Redundant display tag (`ac-002`) retained for readers that index by
    /// name. Decoding rejects a tag that disagrees with `kind`/`ordinal`.
    tag: String,
}

impl From<AnalysisInstanceId> for AnalysisInstanceIdWire {
    fn from(id: AnalysisInstanceId) -> Self {
        Self {
            kind: id.kind(),
            ordinal: id.ordinal(),
            tag: id.tag(),
        }
    }
}

fn analysis_instance_id_from_wire<E: serde::de::Error>(
    wire: AnalysisInstanceIdWire,
) -> Result<AnalysisInstanceId, E> {
    let id = AnalysisInstanceId::new(wire.kind, wire.ordinal);
    if id.tag() != wire.tag {
        return Err(E::custom(format!(
            "analysis tag {:?} does not match its kind and ordinal",
            wire.tag
        )));
    }
    Ok(id)
}

pub(super) mod analysis_instance_id {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &AnalysisInstanceId,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        AnalysisInstanceIdWire::from(*value).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<AnalysisInstanceId, D::Error> {
        analysis_instance_id_from_wire(AnalysisInstanceIdWire::deserialize(deserializer)?)
    }
}

pub(super) mod optional_analysis_instance_id {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &Option<AnalysisInstanceId>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(id) => serializer.serialize_some(&AnalysisInstanceIdWire::from(*id)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<AnalysisInstanceId>, D::Error> {
        match Option::<AnalysisInstanceIdWire>::deserialize(deserializer)? {
            Some(wire) => analysis_instance_id_from_wire(wire).map(Some),
            None => Ok(None),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct RunCoordinateIdWire {
    semantic: String,
    occurrence: u32,
}

pub(super) mod run_coordinate_id {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &RunCoordinateId,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        RunCoordinateIdWire {
            semantic: encode_hex(&value.semantic_bytes()),
            occurrence: value.occurrence(),
        }
        .serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RunCoordinateId, D::Error> {
        let wire = RunCoordinateIdWire::deserialize(deserializer)?;
        let mut semantic = [0u8; 16];
        decode_hex(&wire.semantic, &mut semantic).map_err(D::Error::custom)?;
        Ok(RunCoordinateId::from_parts(semantic, wire.occurrence))
    }
}

pub(super) mod optional_run_coordinate_id {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &Option<RunCoordinateId>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(id) => serializer.serialize_some(&RunCoordinateIdWire {
                semantic: encode_hex(&id.semantic_bytes()),
                occurrence: id.occurrence(),
            }),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<RunCoordinateId>, D::Error> {
        let Some(wire) = Option::<RunCoordinateIdWire>::deserialize(deserializer)? else {
            return Ok(None);
        };
        let mut semantic = [0u8; 16];
        decode_hex(&wire.semantic, &mut semantic).map_err(D::Error::custom)?;
        Ok(Some(RunCoordinateId::from_parts(semantic, wire.occurrence)))
    }
}

pub(super) mod optional_topology_fingerprint {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &Option<TopologyFingerprint>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(fingerprint) => serializer.serialize_some(&encode_hex(&fingerprint.bytes())),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<TopologyFingerprint>, D::Error> {
        let Some(text) = Option::<String>::deserialize(deserializer)? else {
            return Ok(None);
        };
        let mut bytes = [0u8; 32];
        decode_hex(&text, &mut bytes).map_err(D::Error::custom)?;
        Ok(Some(TopologyFingerprint::from_bytes(bytes)))
    }
}

//=============================================================================
// Run-axis values and step targets
//=============================================================================

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DataBindingWire {
    name: String,
    value: Value,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum RunAxisValueWire {
    Numeric {
        value: Value,
    },
    DataRow {
        bindings: Vec<DataBindingWire>,
    },
    AlterVariant {
        label: String,
        materialization_digest: String,
    },
}

impl From<&RunAxisValue> for RunAxisValueWire {
    fn from(value: &RunAxisValue) -> Self {
        match value {
            RunAxisValue::Numeric(value) => Self::Numeric { value: *value },
            RunAxisValue::DataRow(bindings) => Self::DataRow {
                bindings: bindings
                    .iter()
                    .map(|binding| DataBindingWire {
                        name: binding.name().to_owned(),
                        value: binding.value(),
                    })
                    .collect(),
            },
            RunAxisValue::AlterVariant {
                label,
                materialization_digest,
            } => Self::AlterVariant {
                label: label.clone(),
                materialization_digest: encode_hex(materialization_digest),
            },
        }
    }
}

fn run_axis_value_from_wire<E: serde::de::Error>(
    wire: RunAxisValueWire,
) -> Result<RunAxisValue, E> {
    match wire {
        RunAxisValueWire::Numeric { value } => {
            if !value.is_finite() {
                return Err(E::custom(format!("run-axis value {value} is not finite")));
            }
            Ok(RunAxisValue::Numeric(value))
        }
        RunAxisValueWire::DataRow { bindings } => {
            let mut decoded = Vec::with_capacity(bindings.len());
            for binding in bindings {
                decoded.push(
                    DataBinding::new(binding.name, binding.value)
                        .map_err(|error| E::custom(format!("invalid DATA binding: {error}")))?,
                );
            }
            Ok(RunAxisValue::DataRow(decoded))
        }
        RunAxisValueWire::AlterVariant {
            label,
            materialization_digest,
        } => {
            let mut digest = [0u8; 32];
            decode_hex(&materialization_digest, &mut digest).map_err(E::custom)?;
            Ok(RunAxisValue::AlterVariant {
                label,
                materialization_digest: digest,
            })
        }
    }
}

pub(super) mod run_axis_value {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &RunAxisValue,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        RunAxisValueWire::from(value).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<RunAxisValue, D::Error> {
        run_axis_value_from_wire(RunAxisValueWire::deserialize(deserializer)?)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum StepAxisTargetWire {
    Parameter {
        name: String,
    },
    Device {
        name: String,
        parameter: Option<String>,
    },
    Model {
        name: String,
        parameter: String,
    },
    Temperature,
}

impl From<&StepAxisTarget> for StepAxisTargetWire {
    fn from(target: &StepAxisTarget) -> Self {
        match target {
            StepAxisTarget::Parameter { name } => Self::Parameter { name: name.clone() },
            StepAxisTarget::Device { name, parameter } => Self::Device {
                name: name.clone(),
                parameter: parameter.clone(),
            },
            StepAxisTarget::Model { name, parameter } => Self::Model {
                name: name.clone(),
                parameter: parameter.clone(),
            },
            StepAxisTarget::Temperature => Self::Temperature,
        }
    }
}

impl From<StepAxisTargetWire> for StepAxisTarget {
    fn from(target: StepAxisTargetWire) -> Self {
        match target {
            StepAxisTargetWire::Parameter { name } => Self::Parameter { name },
            StepAxisTargetWire::Device { name, parameter } => Self::Device { name, parameter },
            StepAxisTargetWire::Model { name, parameter } => Self::Model { name, parameter },
            StepAxisTargetWire::Temperature => Self::Temperature,
        }
    }
}

pub(super) mod optional_step_axis_target {
    use super::*;

    pub fn serialize<S: Serializer>(
        value: &Option<StepAxisTarget>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(target) => serializer.serialize_some(&StepAxisTargetWire::from(target)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<StepAxisTarget>, D::Error> {
        Ok(Option::<StepAxisTargetWire>::deserialize(deserializer)?.map(StepAxisTarget::from))
    }
}

//=============================================================================
// Hexadecimal digests
//=============================================================================

const HEX_DIGITS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

fn encode_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(HEX_DIGITS[usize::from(byte >> 4)]);
        encoded.push(HEX_DIGITS[usize::from(byte & 0x0f)]);
    }
    encoded
}

fn decode_hex(text: &str, output: &mut [u8]) -> Result<(), String> {
    if text.len() != output.len() * 2 {
        return Err(format!(
            "expected {} hexadecimal characters, found {}",
            output.len() * 2,
            text.len()
        ));
    }
    let bytes = text.as_bytes();
    for (index, slot) in output.iter_mut().enumerate() {
        let high = decode_nibble(bytes[index * 2])?;
        let low = decode_nibble(bytes[index * 2 + 1])?;
        *slot = (high << 4) | low;
    }
    Ok(())
}

fn decode_nibble(byte: u8) -> Result<u8, String> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(format!(
            "invalid lower-case hexadecimal character {:?}",
            char::from(byte)
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_round_trips_and_rejects_upper_case() {
        let source = [0x00u8, 0x0f, 0xa5, 0xff];
        let encoded = encode_hex(&source);
        assert_eq!(encoded, "000fa5ff");
        let mut decoded = [0u8; 4];
        decode_hex(&encoded, &mut decoded).expect("round trip");
        assert_eq!(decoded, source);
        assert!(decode_hex("000FA5FF", &mut decoded).is_err());
        assert!(decode_hex("000fa5f", &mut decoded).is_err());
    }
}

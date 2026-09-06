//! JSON-safe encoding for the floats a compiled artifact carries.
//!
//! A [`CompiledModel`](crate::CompiledModel) and a
//! [`CanonicalIrArtifact`](crate::canonical_ir::CanonicalIrArtifact) travel as
//! JSON: `rspice-core` caches them that way, and the browser worker is handed
//! nothing else — `PreparedVerilogARuntime` seals both as JSON strings.
//!
//! JSON has no spelling for an infinity or a NaN. `serde_json` writes one as
//! `null`, and what happens next depends only on the field's type:
//!
//! * a bare `f64` refuses to load, so the artifact never arrives;
//! * an `Option<f64>` loads `null` as `None`, so the artifact arrives with a
//!   bound silently deleted.
//!
//! Neither is hypothetical. `$bound_step` resets its hidden task variable to
//! `+inf` on every evaluation, so every model that calls it carries an
//! infinity; `from (0:inf)` puts one in a parameter range, which the compact
//! models are full of.
//!
//! [`serialize`] therefore writes a non-finite float as one of the three
//! strings [`POSITIVE_INFINITY`], [`NEGATIVE_INFINITY`] and [`NOT_A_NUMBER`],
//! and a finite one as a plain JSON number — byte for byte what `serde_json`
//! writes without this module, so no artifact digest moves for a model that
//! has no non-finite float to encode.
//!
//! [`refuse_non_finite_floats`] is the guard against the class returning: it
//! names the exact field path of any bare non-finite float still reachable in
//! a value about to be written, so a field that acquires one and is never
//! annotated fails loudly at the seal instead of quietly on arrival.

use std::fmt;

use serde::de::{Unexpected, Visitor};
use serde::{Deserializer, Serialize, Serializer};

/// How [`serialize`] spells `f64::INFINITY`.
pub const POSITIVE_INFINITY: &str = "inf";
/// How [`serialize`] spells `f64::NEG_INFINITY`.
pub const NEGATIVE_INFINITY: &str = "-inf";
/// How [`serialize`] spells a NaN. Payload bits are not preserved: no shipped
/// artifact distinguishes one NaN from another.
pub const NOT_A_NUMBER: &str = "nan";

/// Encode a `f64` so a non-finite value survives JSON.
///
/// Finite values take `serialize_f64` unchanged, which is the whole point:
/// the encoding is byte-identical wherever there is nothing to rescue.
pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if value.is_finite() {
        serializer.serialize_f64(*value)
    } else {
        serializer.serialize_str(non_finite_label(*value))
    }
}

/// Decode what [`serialize`] wrote, and a plain number written before it
/// existed.
pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(FloatVisitor)
}

/// The `Option<f64>` twin.
///
/// `null` decodes as `None` and nothing else does, so an absent bound stays
/// absent and a present one can never arrive as an absence.
pub mod option {
    use serde::{Deserializer, Serializer};

    /// Encode an optional `f64`, rescuing a present non-finite value.
    pub fn serialize<S>(value: &Option<f64>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            None => serializer.serialize_none(),
            Some(value) => serializer.serialize_some(&super::Encoded(*value)),
        }
    }

    /// Decode an optional `f64`. Only `null` yields `None`.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_option(super::OptionalFloatVisitor)
    }
}

/// The `Vec<f64>` twin.
///
/// Elementwise: a finite element is the plain JSON number it always was and a
/// non-finite one takes its spelling, so a list with nothing to rescue encodes
/// byte for byte as it did without this module. Unlike [`option`] there is no
/// absence to preserve — a list has a length, and it is the same length either
/// way.
pub mod vec {
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serializer};

    /// Encode a list of `f64`, rescuing every non-finite element.
    ///
    /// Takes a slice rather than a `&Vec<f64>` so the same function serves a
    /// field of either shape; `#[serde(with)]` coerces the field reference at
    /// the call site.
    pub fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            sequence.serialize_element(&super::Encoded(*value))?;
        }
        sequence.end()
    }

    /// Decode a list of `f64`, element by element through the same decoder
    /// [`super::deserialize`] uses, so the three spellings and a plain number
    /// mean here exactly what they mean anywhere else.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = Vec::<super::Encoded>::deserialize(deserializer)?;
        Ok(values
            .into_iter()
            .map(|super::Encoded(value)| value)
            .collect())
    }
}

/// A `f64` that serializes through [`serialize`], for the places that need a
/// value rather than a field attribute.
struct Encoded(f64);

impl Serialize for Encoded {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serialize(&self.0, serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Encoded {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize(deserializer).map(Self)
    }
}

fn non_finite_label(value: f64) -> &'static str {
    if value.is_nan() {
        NOT_A_NUMBER
    } else if value.is_sign_positive() {
        POSITIVE_INFINITY
    } else {
        NEGATIVE_INFINITY
    }
}

fn labelled_value<E>(text: &str) -> Result<f64, E>
where
    E: serde::de::Error,
{
    match text {
        POSITIVE_INFINITY => Ok(f64::INFINITY),
        NEGATIVE_INFINITY => Ok(f64::NEG_INFINITY),
        NOT_A_NUMBER => Ok(f64::NAN),
        other => Err(E::invalid_value(
            Unexpected::Str(other),
            &"a number, \"inf\", \"-inf\" or \"nan\"",
        )),
    }
}

struct FloatVisitor;

impl<'de> Visitor<'de> for FloatVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a number, \"inf\", \"-inf\" or \"nan\"")
    }

    fn visit_f64<E>(self, value: f64) -> Result<f64, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }

    fn visit_f32<E>(self, value: f32) -> Result<f64, E>
    where
        E: serde::de::Error,
    {
        Ok(f64::from(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<f64, E>
    where
        E: serde::de::Error,
    {
        Ok(value as f64)
    }

    fn visit_u64<E>(self, value: u64) -> Result<f64, E>
    where
        E: serde::de::Error,
    {
        Ok(value as f64)
    }

    fn visit_str<E>(self, value: &str) -> Result<f64, E>
    where
        E: serde::de::Error,
    {
        labelled_value(value)
    }
}

struct OptionalFloatVisitor;

impl<'de> Visitor<'de> for OptionalFloatVisitor {
    type Value = Option<f64>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("null, a number, \"inf\", \"-inf\" or \"nan\"")
    }

    fn visit_none<E>(self) -> Result<Option<f64>, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_unit<E>(self) -> Result<Option<f64>, E>
    where
        E: serde::de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FloatVisitor).map(Some)
    }
}

/// One bare non-finite float found in a value, and where it sits.
#[derive(Debug, Clone, PartialEq)]
pub struct NonFiniteFloat {
    /// Dotted field path from the root name the scan was given, with `[i]`
    /// for a sequence position, `["k"]` for a map key and the variant name
    /// for an enum.
    pub path: String,
    pub value: f64,
}

impl fmt::Display for NonFiniteFloat {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} = {}",
            self.path,
            non_finite_label(self.value)
        )
    }
}

/// A `Serialize` implementation that refused to be walked.
///
/// Nothing in the compiled artifacts can produce one; it exists so the scan
/// reports a failure instead of a clean bill of health.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanError(String);

impl fmt::Display for ScanError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for ScanError {}

impl serde::ser::Error for ScanError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

/// Why a value cannot be sealed as JSON.
#[derive(Debug, Clone, PartialEq)]
pub enum NonFiniteFloatError {
    /// The value could not be walked, so nothing was proven either way.
    Scan(ScanError),
    /// Bare non-finite floats that JSON would lose, in the order found.
    Unencodable(Vec<NonFiniteFloat>),
}

/// How many paths a refusal names before it stops listing them.
const REFUSAL_PATH_LIMIT: usize = 8;

impl fmt::Display for NonFiniteFloatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scan(error) => write!(formatter, "the payload could not be scanned: {error}"),
            Self::Unencodable(found) => {
                write!(
                    formatter,
                    "{} non-finite float{} JSON cannot encode: ",
                    found.len(),
                    if found.len() == 1 { "" } else { "s" }
                )?;
                for (position, entry) in found.iter().take(REFUSAL_PATH_LIMIT).enumerate() {
                    if position > 0 {
                        formatter.write_str(", ")?;
                    }
                    write!(formatter, "{entry}")?;
                }
                if let Some(remaining) = found.len().checked_sub(REFUSAL_PATH_LIMIT)
                    && remaining > 0
                {
                    write!(formatter, ", and {remaining} more")?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for NonFiniteFloatError {}

/// Every bare non-finite float `value` would hand a JSON serializer, with the
/// field path of each.
///
/// A float already routed through [`serialize`] is not bare and is not
/// reported: the scan sees the string it writes, which is exactly the
/// question being asked — what would still be lost.
///
/// `root` names the value being walked and becomes the first path segment.
pub fn non_finite_floats<T>(root: &str, value: &T) -> Result<Vec<NonFiniteFloat>, ScanError>
where
    T: ?Sized + Serialize,
{
    let mut scan = Scan {
        path: vec![Segment::Root(root.to_owned())],
        counters: Vec::new(),
        pending_key: Vec::new(),
        found: Vec::new(),
    };
    value.serialize(&mut scan)?;
    Ok(scan.found)
}

/// Refuse `value` if any bare non-finite float would be lost by writing it as
/// JSON.
///
/// This is the guard the seal runs before it writes. Every field that can
/// legitimately carry a non-finite float is annotated
/// `#[serde(with = "crate::json_float")]`, so anything this finds is a field
/// that acquired one without being annotated — a defect, and one that is
/// otherwise silent on an `Option<f64>`.
pub fn refuse_non_finite_floats<T>(root: &str, value: &T) -> Result<(), NonFiniteFloatError>
where
    T: ?Sized + Serialize,
{
    match non_finite_floats(root, value) {
        Ok(found) if found.is_empty() => Ok(()),
        Ok(found) => Err(NonFiniteFloatError::Unencodable(found)),
        Err(error) => Err(NonFiniteFloatError::Scan(error)),
    }
}

/// One step of a field path.
///
/// Only the root and a map key are owned. Struct field names and variant names
/// arrive from `serde` as `&'static str`, and a scan of a compiled model walks
/// millions of them — allocating one `String` per field would cost more than
/// the seal it guards.
enum Segment {
    Root(String),
    Field(&'static str),
    Index(usize),
    Key(String),
}

struct Scan {
    path: Vec<Segment>,
    /// One element position counter per open sequence or tuple.
    counters: Vec<usize>,
    /// One rendered key per open map, set by `serialize_key`.
    pending_key: Vec<String>,
    found: Vec<NonFiniteFloat>,
}

impl Scan {
    fn render_path(&self) -> String {
        let mut rendered = String::new();
        for segment in &self.path {
            match segment {
                Segment::Root(name) => rendered.push_str(name),
                Segment::Field(name) => {
                    if !rendered.is_empty() {
                        rendered.push('.');
                    }
                    rendered.push_str(name);
                }
                Segment::Index(index) => {
                    rendered.push_str(&format!("[{index}]"));
                }
                Segment::Key(key) => {
                    rendered.push_str(&format!("[{key}]"));
                }
            }
        }
        rendered
    }

    fn record(&mut self, value: f64) {
        if value.is_finite() {
            return;
        }
        self.found.push(NonFiniteFloat {
            path: self.render_path(),
            value,
        });
    }

    fn nested<T>(&mut self, segment: Segment, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        self.path.push(segment);
        let result = value.serialize(&mut *self);
        self.path.pop();
        result
    }

    fn next_index(&mut self) -> usize {
        match self.counters.last_mut() {
            Some(counter) => {
                let index = *counter;
                *counter += 1;
                index
            }
            None => 0,
        }
    }
}

/// Render a map key for a path segment. Keys are strings in every artifact
/// map, so this is a quoted string in practice; anything else falls back to
/// its JSON form, and an unrenderable key to `?`.
fn render_key<T>(key: &T) -> String
where
    T: ?Sized + Serialize,
{
    serde_json::to_string(key).unwrap_or_else(|_| "?".to_owned())
}

macro_rules! ignored_scalar {
    ($($method:ident($type:ty),)*) => {
        $(
            fn $method(self, _: $type) -> Result<(), ScanError> {
                Ok(())
            }
        )*
    };
}

impl serde::Serializer for &mut Scan {
    type Ok = ();
    type Error = ScanError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    ignored_scalar! {
        serialize_bool(bool),
        serialize_i8(i8),
        serialize_i16(i16),
        serialize_i32(i32),
        serialize_i64(i64),
        serialize_i128(i128),
        serialize_u8(u8),
        serialize_u16(u16),
        serialize_u32(u32),
        serialize_u64(u64),
        serialize_u128(u128),
        serialize_char(char),
        serialize_str(&str),
        serialize_bytes(&[u8]),
    }

    fn serialize_f32(self, value: f32) -> Result<(), ScanError> {
        self.record(f64::from(value));
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<(), ScanError> {
        self.record(value);
        Ok(())
    }

    fn serialize_none(self) -> Result<(), ScanError> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), ScanError> {
        Ok(())
    }

    fn serialize_unit_struct(self, _: &'static str) -> Result<(), ScanError> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
    ) -> Result<(), ScanError> {
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _: &'static str, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        self.nested(Segment::Field(variant), value)
    }

    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, ScanError> {
        self.counters.push(0);
        Ok(self)
    }

    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, ScanError> {
        self.counters.push(0);
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, ScanError> {
        self.counters.push(0);
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, ScanError> {
        self.path.push(Segment::Field(variant));
        self.counters.push(0);
        Ok(self)
    }

    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, ScanError> {
        self.pending_key.push(String::new());
        Ok(self)
    }

    fn serialize_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStruct, ScanError> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, ScanError> {
        self.path.push(Segment::Field(variant));
        Ok(self)
    }
}

impl serde::ser::SerializeSeq for &mut Scan {
    type Ok = ();
    type Error = ScanError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        let index = self.next_index();
        self.nested(Segment::Index(index), value)
    }

    fn end(self) -> Result<(), ScanError> {
        self.counters.pop();
        Ok(())
    }
}

impl serde::ser::SerializeTuple for &mut Scan {
    type Ok = ();
    type Error = ScanError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        let index = self.next_index();
        self.nested(Segment::Index(index), value)
    }

    fn end(self) -> Result<(), ScanError> {
        self.counters.pop();
        Ok(())
    }
}

impl serde::ser::SerializeTupleStruct for &mut Scan {
    type Ok = ();
    type Error = ScanError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        let index = self.next_index();
        self.nested(Segment::Index(index), value)
    }

    fn end(self) -> Result<(), ScanError> {
        self.counters.pop();
        Ok(())
    }
}

impl serde::ser::SerializeTupleVariant for &mut Scan {
    type Ok = ();
    type Error = ScanError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        let index = self.next_index();
        self.nested(Segment::Index(index), value)
    }

    fn end(self) -> Result<(), ScanError> {
        self.counters.pop();
        self.path.pop();
        Ok(())
    }
}

impl serde::ser::SerializeMap for &mut Scan {
    type Ok = ();
    type Error = ScanError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        let rendered = render_key(key);
        match self.pending_key.last_mut() {
            Some(slot) => *slot = rendered,
            None => self.pending_key.push(rendered),
        }
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        let key = self.pending_key.last().cloned().unwrap_or_default();
        self.nested(Segment::Key(key), value)
    }

    fn end(self) -> Result<(), ScanError> {
        self.pending_key.pop();
        Ok(())
    }
}

impl serde::ser::SerializeStruct for &mut Scan {
    type Ok = ();
    type Error = ScanError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        self.nested(Segment::Field(key), value)
    }

    fn end(self) -> Result<(), ScanError> {
        Ok(())
    }
}

impl serde::ser::SerializeStructVariant for &mut Scan {
    type Ok = ();
    type Error = ScanError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), ScanError>
    where
        T: ?Sized + Serialize,
    {
        self.nested(Segment::Field(key), value)
    }

    fn end(self) -> Result<(), ScanError> {
        self.path.pop();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Bounded {
        #[serde(with = "crate::json_float")]
        default: f64,
        #[serde(with = "crate::json_float::option")]
        min: Option<f64>,
        #[serde(with = "crate::json_float::option")]
        max: Option<f64>,
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Plain {
        default: f64,
        min: Option<f64>,
        max: Option<f64>,
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Excluding {
        #[serde(with = "crate::json_float::vec")]
        exclude: Vec<f64>,
    }

    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct PlainExcluding {
        exclude: Vec<f64>,
    }

    #[test]
    fn a_finite_value_encodes_byte_for_byte_as_it_did_without_the_helper() {
        // The whole reason the encoding is a string only for the values JSON
        // has no spelling for: a model with nothing to rescue must hash the
        // same before and after, or every artifact digest in the tree moves.
        for value in [
            0.0_f64,
            -0.0,
            1.0,
            -2.5,
            1.380_650_5e-23,
            6.25e41,
            f64::MIN,
            f64::MAX,
            f64::MIN_POSITIVE,
            f64::EPSILON,
        ] {
            let bounded = Bounded {
                default: value,
                min: Some(value),
                max: None,
            };
            let plain = Plain {
                default: value,
                min: Some(value),
                max: None,
            };
            assert_eq!(
                serde_json::to_string(&bounded).unwrap(),
                serde_json::to_string(&plain).unwrap(),
                "{value:e} must encode identically with and without the helper"
            );
        }
    }

    #[test]
    fn a_finite_list_encodes_byte_for_byte_as_it_did_without_the_helper() {
        // The `Vec<f64>` twin owes the same debt as the scalar one: a list with
        // nothing to rescue must serialize to the bytes it always did, or every
        // artifact carrying an `exclude` list moves its digest.
        for values in [
            Vec::new(),
            vec![0.0_f64],
            vec![-0.0, 1.0, -2.5],
            vec![1.380_650_5e-23, f64::MIN, f64::MAX],
            vec![f64::MIN_POSITIVE, f64::EPSILON, 6.25e41],
        ] {
            assert_eq!(
                serde_json::to_string(&Excluding {
                    exclude: values.clone()
                })
                .unwrap(),
                serde_json::to_string(&PlainExcluding {
                    exclude: values.clone()
                })
                .unwrap(),
                "{values:?} must encode identically with and without the helper"
            );
        }
    }

    #[test]
    fn a_list_keeps_every_non_finite_element_in_place() {
        // An excluded value is identified by position as well as by value, so
        // the encoding has to preserve the length and the order, not only the
        // values it can spell.
        let excluding = Excluding {
            exclude: vec![1.0, f64::INFINITY, -2.5, f64::NEG_INFINITY, f64::NAN],
        };
        let encoded = serde_json::to_string(&excluding).unwrap();
        assert_eq!(encoded, r#"{"exclude":[1.0,"inf",-2.5,"-inf","nan"]}"#);

        let decoded: Excluding = serde_json::from_str(&encoded).unwrap();
        assert_eq!(
            decoded
                .exclude
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>(),
            excluding
                .exclude
                .iter()
                .map(|value| value.to_bits())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn a_bare_list_loses_its_non_finite_elements_the_loud_way() {
        // The failure the annotation removes: `serde_json` writes each one as
        // `null`, and a bare `f64` element refuses to load it back, so the
        // whole artifact is unreadable rather than subtly wrong.
        let plain = serde_json::to_string(&PlainExcluding {
            exclude: vec![1.0, f64::INFINITY],
        })
        .unwrap();
        assert_eq!(plain, r#"{"exclude":[1.0,null]}"#);
        assert!(serde_json::from_str::<PlainExcluding>(&plain).is_err());
    }

    #[test]
    fn a_list_written_before_the_helper_existed_still_decodes() {
        let decoded: Excluding = serde_json::from_str(r#"{"exclude":[3,-1.5]}"#).unwrap();
        assert_eq!(decoded.exclude, vec![3.0, -1.5]);
    }

    #[test]
    fn an_annotated_list_is_not_reported_because_its_elements_are_no_longer_bare() {
        let excluding = Excluding {
            exclude: vec![1.0, f64::INFINITY],
        };
        assert_eq!(non_finite_floats("model", &excluding).unwrap(), Vec::new());
        assert_eq!(refuse_non_finite_floats("model", &excluding), Ok(()));

        let plain = PlainExcluding {
            exclude: vec![1.0, f64::INFINITY],
        };
        let error = refuse_non_finite_floats("model", &plain)
            .expect_err("a bare non-finite element must be refused");
        assert_eq!(
            error.to_string(),
            "1 non-finite float JSON cannot encode: model.exclude[1] = inf"
        );
    }

    #[test]
    fn every_non_finite_value_round_trips_through_json() {
        for value in [f64::INFINITY, f64::NEG_INFINITY] {
            let encoded = serde_json::to_string(&Bounded {
                default: value,
                min: Some(value),
                max: Some(-value),
            })
            .unwrap();
            let decoded: Bounded = serde_json::from_str(&encoded).unwrap();
            assert_eq!(decoded.default.to_bits(), value.to_bits(), "{encoded}");
            assert_eq!(decoded.min.unwrap().to_bits(), value.to_bits(), "{encoded}");
            assert_eq!(
                decoded.max.unwrap().to_bits(),
                (-value).to_bits(),
                "{encoded}"
            );
        }

        let encoded = serde_json::to_string(&Bounded {
            default: f64::NAN,
            min: Some(f64::NAN),
            max: None,
        })
        .unwrap();
        assert_eq!(encoded, r#"{"default":"nan","min":"nan","max":null}"#);
        let decoded: Bounded = serde_json::from_str(&encoded).unwrap();
        assert!(decoded.default.is_nan());
        assert!(decoded.min.is_some_and(f64::is_nan));
        assert_eq!(decoded.max, None);
    }

    #[test]
    fn the_encoding_is_the_three_documented_spellings() {
        let encoded = serde_json::to_string(&Bounded {
            default: f64::INFINITY,
            min: Some(f64::NEG_INFINITY),
            max: Some(2.5),
        })
        .unwrap();
        assert_eq!(encoded, r#"{"default":"inf","min":"-inf","max":2.5}"#);
    }

    #[test]
    fn an_absent_bound_stays_absent_and_a_present_one_can_never_arrive_absent() {
        // The silent half of the defect: without the helper an infinite bound
        // encodes as `null` and decodes as `None`, deleting the bound.
        let plain = serde_json::to_string(&Plain {
            default: 1.0,
            min: Some(f64::INFINITY),
            max: None,
        })
        .unwrap();
        let lost: Plain = serde_json::from_str(&plain).unwrap();
        assert_eq!(lost.min, None, "this is the loss the helper exists to stop");

        let helped = serde_json::to_string(&Bounded {
            default: 1.0,
            min: Some(f64::INFINITY),
            max: None,
        })
        .unwrap();
        let kept: Bounded = serde_json::from_str(&helped).unwrap();
        assert_eq!(kept.min.map(f64::to_bits), Some(f64::INFINITY.to_bits()));
        assert_eq!(kept.max, None);
    }

    #[test]
    fn a_bare_non_finite_float_refuses_to_load_rather_than_arriving_wrong() {
        // The loud half. Kept beside the silent half so the two failure modes
        // stay documented by something that runs.
        let plain = serde_json::to_string(&Plain {
            default: f64::INFINITY,
            min: None,
            max: None,
        })
        .unwrap();
        assert_eq!(plain, r#"{"default":null,"min":null,"max":null}"#);
        assert!(serde_json::from_str::<Plain>(&plain).is_err());
    }

    #[test]
    fn a_number_written_before_the_helper_existed_still_decodes() {
        let decoded: Bounded =
            serde_json::from_str(r#"{"default":3,"min":-1.5,"max":null}"#).unwrap();
        assert_eq!(decoded.default, 3.0);
        assert_eq!(decoded.min, Some(-1.5));
        assert_eq!(decoded.max, None);
    }

    #[test]
    fn an_unknown_spelling_is_refused_rather_than_guessed_at() {
        let error =
            serde_json::from_str::<Bounded>(r#"{"default":"Infinity","min":null,"max":null}"#)
                .expect_err("only the three documented spellings decode");
        assert!(error.to_string().contains("inf"), "{error}");
    }

    #[test]
    fn the_scan_names_the_path_of_every_bare_non_finite_float() {
        #[derive(serde::Serialize)]
        struct Program {
            steps: Vec<Step>,
            table: std::collections::BTreeMap<String, f64>,
        }
        #[derive(serde::Serialize)]
        enum Step {
            Const(f64),
            Range { min: Option<f64>, max: Option<f64> },
        }

        let value = Program {
            steps: vec![
                Step::Const(1.0),
                Step::Const(f64::INFINITY),
                Step::Range {
                    min: Some(f64::NEG_INFINITY),
                    max: None,
                },
            ],
            table: [("gmin".to_owned(), f64::NAN)].into_iter().collect(),
        };

        let found = non_finite_floats("model", &value).unwrap();
        let paths: Vec<_> = found.iter().map(|entry| entry.to_string()).collect();
        assert_eq!(
            paths,
            vec![
                "model.steps[1].Const = inf",
                "model.steps[2].Range.min = -inf",
                r#"model.table["gmin"] = nan"#,
            ]
        );
    }

    #[test]
    fn an_annotated_field_is_not_reported_because_it_is_no_longer_bare() {
        let bounded = Bounded {
            default: f64::INFINITY,
            min: Some(f64::NAN),
            max: None,
        };
        assert_eq!(non_finite_floats("model", &bounded).unwrap(), Vec::new());
        assert_eq!(refuse_non_finite_floats("model", &bounded), Ok(()));
    }

    #[test]
    fn the_refusal_names_the_field_it_refuses_for() {
        let plain = Plain {
            default: f64::INFINITY,
            min: Some(f64::NEG_INFINITY),
            max: None,
        };
        let error = refuse_non_finite_floats("model", &plain)
            .expect_err("a bare non-finite float must be refused");
        assert_eq!(
            error.to_string(),
            "2 non-finite floats JSON cannot encode: model.default = inf, model.min = -inf"
        );
    }

    #[test]
    fn a_long_refusal_stops_listing_and_says_how_many_it_left_out() {
        let value: Vec<f64> = std::iter::repeat_n(f64::INFINITY, REFUSAL_PATH_LIMIT + 3).collect();
        let error =
            refuse_non_finite_floats("ir", &value).expect_err("bare non-finite floats are refused");
        assert!(error.to_string().ends_with(", and 3 more"), "{error}");
    }

    /// The digital half's constants are the one part of a sealed artifact that
    /// looks like it should need the encoding and does not, so the reason is
    /// pinned rather than written down: three separate refusals stand between
    /// a source and a non-finite `CfgValueKind::RealConstant`, and if any one
    /// of them goes, this test says so and the field needs annotating.
    ///
    /// `CanonicalIrArtifact.digital` is the only place a [`CfgFunction`] is
    /// serialized, and `digital_lower` is its only producer — no optimizer or
    /// derivative pass runs over a process function — so what that lowering
    /// can emit is the whole question.
    ///
    /// [`CfgFunction`]: crate::canonical_ir::cfg::CfgFunction
    #[test]
    fn a_digital_process_cannot_carry_a_non_finite_constant() {
        fn compile(source: &str) -> Result<crate::RuntimeCompileReport, String> {
            crate::VerilogACompiler::new(crate::CompilerOptions {
                enable_ams: true,
                ..crate::CompilerOptions::default()
            })
            .compile_runtime(source, None)
            .map_err(|error| error.to_string())
        }

        // 1. A `parameter real` whose default folds to an infinity never enters
        //    the digital constant table at all: `digital_constants` keeps only
        //    finite defaults, so the name is not a discrete-domain identifier
        //    and the module is refused rather than lowered.
        let refused = compile(
            r#"
module rspice_digital_parameter_probe(p, n, clk, q);
  inout p, n; electrical p, n;
  input clk; output q; wire clk; reg q;
  parameter real big = 1.0/0.0;
  real acc;
  initial q = 1'b0;
  always @(posedge clk) begin acc = big; q <= ~q; end
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#,
        )
        .expect_err("an infinite real parameter is invisible to the discrete domain");
        assert!(
            refused.contains("`big` is not a discrete-domain signal"),
            "{refused}"
        );

        // 2. A literal too large for a `f64` is refused by the number parser,
        //    which is the same refusal the continuous domain gets.
        let refused = compile(
            r#"
module rspice_digital_literal_probe(p, n, clk, q);
  inout p, n; electrical p, n;
  input clk; output q; wire clk; reg q;
  real acc;
  initial q = 1'b0;
  always @(posedge clk) begin acc = 1.0e400; q <= ~q; end
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#,
        )
        .expect_err("an overflowing literal is not a number");
        assert!(
            refused.contains("outside the finite real range"),
            "{refused}"
        );

        // 3. Arithmetic that *evaluates* to an infinity is not folded at
        //    lowering: it stays an operation over finite operands, so the plan
        //    carries `1.0` and `0.0` and the infinity only ever exists at run
        //    time, where JSON never sees it.
        let report = compile(
            r#"
module rspice_digital_division_probe(p, n, clk, q);
  inout p, n; electrical p, n;
  input clk; output q; wire clk; reg q;
  real acc;
  initial q = 1'b0;
  always @(posedge clk) begin acc = 1.0 / 0.0; q <= ~q; end
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#,
        )
        .expect("a division by a literal zero is a legal discrete-domain expression");
        let plan = &report.canonical_ir.digital;
        assert!(
            !plan.is_empty(),
            "the fixture must actually carry a digital half"
        );
        let constants: Vec<f64> = plan
            .processes
            .iter()
            .flat_map(|process| process.function.values.iter())
            .filter_map(|value| match value.kind {
                crate::canonical_ir::cfg::CfgValueKind::RealConstant(constant) => Some(constant),
                _ => None,
            })
            .collect();
        assert!(
            constants.iter().all(|constant| constant.is_finite()),
            "a folded infinity would need annotating, found {constants:?}"
        );

        // And the whole artifact, which is the question the seal asks.
        assert_eq!(
            non_finite_floats("canonical_ir", &report.canonical_ir).unwrap(),
            Vec::new()
        );
        assert_eq!(
            non_finite_floats("model", &report.model).unwrap(),
            Vec::new()
        );
    }
}

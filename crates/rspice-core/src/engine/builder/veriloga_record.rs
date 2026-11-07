//! The compact binary encoding of a Verilog-A cache record.
//!
//! A compiled model is mostly bytecode: millions of small enum values, each
//! with an integer or a float. JSON spends most of a record's bytes on decimal
//! floats, field names and variant names, and most of a cache hit's time
//! parsing them back. Here every value sits behind a one-byte tag: integers
//! are LEB128 varints, floats their eight IEEE bytes, enum variants their
//! index, and a struct field name is written once per record and referenced
//! by its table index after that.
//!
//! The encoding is self-describing, which the artifact types rely on. A field
//! skipped on write (`skip_serializing_if`) is simply absent from its struct,
//! and a float written as a number or as a non-finite label (`json_float`)
//! reads back either way.
//!
//! The decoder trusts nothing it reads: every length is checked against the
//! bytes that remain, nesting is bounded, and a malformed record is an error,
//! never a panic. The record's checksum is verified before decoding starts,
//! so a damaged file is refused without being interpreted at all.

use serde::de::{self, DeserializeSeed, IntoDeserializer, Visitor};
use serde::ser::{self, Serialize};
use std::collections::HashMap;
use std::fmt;

const NONE: u8 = 0;
const SOME: u8 = 1;
const UNIT: u8 = 2;
const FALSE: u8 = 3;
const TRUE: u8 = 4;
const UINT: u8 = 5;
/// A negative integer `v`, stored as the varint of `!v` (that is, `-v - 1`).
const NINT: u8 = 6;
const F64: u8 = 7;
const F32: u8 = 8;
const STR: u8 = 9;
const BYTES: u8 = 10;
const SEQ: u8 = 11;
const MAP: u8 = 12;
const STRUCT: u8 = 13;
const UNIT_VARIANT: u8 = 14;
const NEWTYPE_VARIANT: u8 = 15;
const TUPLE_VARIANT: u8 = 16;
const STRUCT_VARIANT: u8 = 17;
const CHAR: u8 = 18;
const U128: u8 = 19;
const I128: u8 = 20;
/// A sequence whose length was unknown when it started, closed by [`END`].
const OPEN_SEQ: u8 = 21;
/// A map whose length was unknown when it started, closed by [`END`].
const OPEN_MAP: u8 = 22;
const END: u8 = 23;

/// Deeper nesting than any compiled artifact has is a malformed record.
const MAX_DEPTH: u32 = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RecordCodecError(String);

impl fmt::Display for RecordCodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for RecordCodecError {}

impl ser::Error for RecordCodecError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

impl de::Error for RecordCodecError {
    fn custom<T: fmt::Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

type Result<T> = std::result::Result<T, RecordCodecError>;

/// Encode `value`.
pub(super) fn encode<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let mut encoder = Encoder {
        out: Vec::new(),
        names: HashMap::new(),
    };
    value.serialize(&mut encoder)?;
    Ok(encoder.out)
}

/// The length [`encode`] would return, without building the encoding.
pub(super) fn encoded_len<T: Serialize + ?Sized>(value: &T) -> Result<usize> {
    let mut encoder = Encoder {
        out: Count(0),
        names: HashMap::new(),
    };
    value.serialize(&mut encoder)?;
    Ok(encoder.out.0)
}

/// Decode a value [`encode`] wrote. Every byte of `bytes` must belong to it.
pub(super) fn decode<'de, T: de::Deserialize<'de>>(bytes: &'de [u8]) -> Result<T> {
    let mut decoder = Decoder {
        input: bytes,
        position: 0,
        names: Vec::new(),
        depth: 0,
    };
    let value = T::deserialize(&mut decoder)?;
    if decoder.position != bytes.len() {
        return Err(RecordCodecError(format!(
            "{} trailing bytes after the record",
            bytes.len() - decoder.position
        )));
    }
    Ok(value)
}

/// Where an encoding goes: a buffer, or only a running length.
trait Sink {
    fn push(&mut self, byte: u8);
    fn extend_from_slice(&mut self, bytes: &[u8]);
}

impl Sink for Vec<u8> {
    fn push(&mut self, byte: u8) {
        Vec::push(self, byte);
    }

    fn extend_from_slice(&mut self, bytes: &[u8]) {
        Vec::extend_from_slice(self, bytes);
    }
}

struct Count(usize);

impl Sink for Count {
    fn push(&mut self, _byte: u8) {
        self.0 += 1;
    }

    fn extend_from_slice(&mut self, bytes: &[u8]) {
        self.0 += bytes.len();
    }
}

struct Encoder<S> {
    out: S,
    /// Each field name written so far, by its table index.
    names: HashMap<&'static str, u64>,
}

impl<S: Sink> Encoder<S> {
    fn varint(&mut self, mut value: u64) {
        while value >= 0x80 {
            self.out.push((value as u8) | 0x80);
            value >>= 7;
        }
        self.out.push(value as u8);
    }

    fn length(&mut self, len: usize) {
        self.varint(len as u64);
    }

    fn tagged_len(&mut self, tag: u8, len: usize) {
        self.out.push(tag);
        self.length(len);
    }

    /// A name already in the table is its index plus one; zero introduces a
    /// new name, spelled out once.
    fn name(&mut self, name: &'static str) {
        if let Some(&index) = self.names.get(name) {
            self.varint(index + 1);
        } else {
            let index = self.names.len() as u64;
            self.names.insert(name, index);
            self.varint(0);
            self.length(name.len());
            self.out.extend_from_slice(name.as_bytes());
        }
    }

    fn signed(&mut self, value: i64) {
        if value >= 0 {
            self.out.push(UINT);
            self.varint(value as u64);
        } else {
            self.out.push(NINT);
            self.varint(!(value as u64));
        }
    }
}

/// A sequence or map in progress: one with a known length needs no end marker.
struct Compound<'a, S> {
    encoder: &'a mut Encoder<S>,
    open: bool,
}

impl<S: Sink> Compound<'_, S> {
    fn finish(self) {
        if self.open {
            self.encoder.out.push(END);
        }
    }
}

impl<'a, S: Sink> ser::Serializer for &'a mut Encoder<S> {
    type Ok = ();
    type Error = RecordCodecError;
    type SerializeSeq = Compound<'a, S>;
    type SerializeTuple = Compound<'a, S>;
    type SerializeTupleStruct = Compound<'a, S>;
    type SerializeTupleVariant = Compound<'a, S>;
    type SerializeMap = Compound<'a, S>;
    type SerializeStruct = Compound<'a, S>;
    type SerializeStructVariant = Compound<'a, S>;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.out.push(if value { TRUE } else { FALSE });
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.signed(value.into());
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.signed(value.into());
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.signed(value.into());
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.signed(value);
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<()> {
        self.out.push(I128);
        self.out.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.serialize_u64(value.into())
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        self.serialize_u64(value.into())
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        self.serialize_u64(value.into())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        self.out.push(UINT);
        self.varint(value);
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<()> {
        self.out.push(U128);
        self.out.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        self.out.push(F32);
        self.out.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        self.out.push(F64);
        self.out.extend_from_slice(&value.to_le_bytes());
        Ok(())
    }

    fn serialize_char(self, value: char) -> Result<()> {
        self.out.push(CHAR);
        self.varint(value.into());
        Ok(())
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.tagged_len(STR, value.len());
        self.out.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.tagged_len(BYTES, value.len());
        self.out.extend_from_slice(value);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.out.push(NONE);
        Ok(())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<()> {
        self.out.push(SOME);
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.out.push(UNIT);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.out.push(UNIT_VARIANT);
        self.varint(variant_index.into());
        Ok(())
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()> {
        self.out.push(NEWTYPE_VARIANT);
        self.varint(variant_index.into());
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Compound<'a, S>> {
        match len {
            Some(len) => {
                self.tagged_len(SEQ, len);
                Ok(Compound {
                    encoder: self,
                    open: false,
                })
            }
            None => {
                self.out.push(OPEN_SEQ);
                Ok(Compound {
                    encoder: self,
                    open: true,
                })
            }
        }
    }

    fn serialize_tuple(self, len: usize) -> Result<Compound<'a, S>> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Compound<'a, S>> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Compound<'a, S>> {
        self.out.push(TUPLE_VARIANT);
        self.varint(variant_index.into());
        self.length(len);
        Ok(Compound {
            encoder: self,
            open: false,
        })
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Compound<'a, S>> {
        match len {
            Some(len) => {
                self.tagged_len(MAP, len);
                Ok(Compound {
                    encoder: self,
                    open: false,
                })
            }
            None => {
                self.out.push(OPEN_MAP);
                Ok(Compound {
                    encoder: self,
                    open: true,
                })
            }
        }
    }

    /// `len` counts the fields that will be written: derived code subtracts
    /// every field its `skip_serializing_if` skips.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Compound<'a, S>> {
        self.tagged_len(STRUCT, len);
        Ok(Compound {
            encoder: self,
            open: false,
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Compound<'a, S>> {
        self.out.push(STRUCT_VARIANT);
        self.varint(variant_index.into());
        self.length(len);
        Ok(Compound {
            encoder: self,
            open: false,
        })
    }

    fn is_human_readable(&self) -> bool {
        false
    }
}

impl<S: Sink> ser::SerializeSeq for Compound<'_, S> {
    type Ok = ();
    type Error = RecordCodecError;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.encoder)
    }

    fn end(self) -> Result<()> {
        self.finish();
        Ok(())
    }
}

impl<S: Sink> ser::SerializeTuple for Compound<'_, S> {
    type Ok = ();
    type Error = RecordCodecError;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.encoder)
    }

    fn end(self) -> Result<()> {
        self.finish();
        Ok(())
    }
}

impl<S: Sink> ser::SerializeTupleStruct for Compound<'_, S> {
    type Ok = ();
    type Error = RecordCodecError;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.encoder)
    }

    fn end(self) -> Result<()> {
        self.finish();
        Ok(())
    }
}

impl<S: Sink> ser::SerializeTupleVariant for Compound<'_, S> {
    type Ok = ();
    type Error = RecordCodecError;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.encoder)
    }

    fn end(self) -> Result<()> {
        self.finish();
        Ok(())
    }
}

impl<S: Sink> ser::SerializeMap for Compound<'_, S> {
    type Ok = ();
    type Error = RecordCodecError;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<()> {
        key.serialize(&mut *self.encoder)
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.encoder)
    }

    fn end(self) -> Result<()> {
        self.finish();
        Ok(())
    }
}

impl<S: Sink> ser::SerializeStruct for Compound<'_, S> {
    type Ok = ();
    type Error = RecordCodecError;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.encoder.name(key);
        value.serialize(&mut *self.encoder)
    }

    fn end(self) -> Result<()> {
        self.finish();
        Ok(())
    }
}

impl<S: Sink> ser::SerializeStructVariant for Compound<'_, S> {
    type Ok = ();
    type Error = RecordCodecError;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.encoder.name(key);
        value.serialize(&mut *self.encoder)
    }

    fn end(self) -> Result<()> {
        self.finish();
        Ok(())
    }
}

struct Decoder<'de> {
    input: &'de [u8],
    position: usize,
    /// Each field name read so far, by its table index.
    names: Vec<&'de str>,
    depth: u32,
}

impl<'de> Decoder<'de> {
    fn malformed(&self, what: &str) -> RecordCodecError {
        RecordCodecError(format!(
            "malformed record at byte {}: {what}",
            self.position
        ))
    }

    fn byte(&mut self) -> Result<u8> {
        let byte = *self
            .input
            .get(self.position)
            .ok_or_else(|| self.malformed("unexpected end"))?;
        self.position += 1;
        Ok(byte)
    }

    fn peek(&self) -> Result<u8> {
        self.input
            .get(self.position)
            .copied()
            .ok_or_else(|| self.malformed("unexpected end"))
    }

    fn take(&mut self, len: usize) -> Result<&'de [u8]> {
        let input = self.input;
        let end = self
            .position
            .checked_add(len)
            .filter(|end| *end <= input.len())
            .ok_or_else(|| self.malformed("length runs past the end"))?;
        let bytes = &input[self.position..end];
        self.position = end;
        Ok(bytes)
    }

    fn array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let bytes = self.take(N)?;
        let mut array = [0; N];
        array.copy_from_slice(bytes);
        Ok(array)
    }

    fn varint(&mut self) -> Result<u64> {
        let mut value = 0_u64;
        let mut shift = 0_u32;
        loop {
            let byte = self.byte()?;
            if shift == 63 && byte > 1 {
                return Err(self.malformed("integer overflows 64 bits"));
            }
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
            shift += 7;
            if shift > 63 {
                return Err(self.malformed("integer overflows 64 bits"));
            }
        }
    }

    /// A count of things that each take at least one byte: one larger than
    /// the bytes left cannot be honest.
    fn count(&mut self) -> Result<usize> {
        let count = self.varint()?;
        usize::try_from(count)
            .ok()
            .filter(|count| *count <= self.input.len() - self.position)
            .ok_or_else(|| self.malformed("count exceeds the record"))
    }

    fn text(&mut self) -> Result<&'de str> {
        let len = self.count()?;
        let bytes = self.take(len)?;
        std::str::from_utf8(bytes).map_err(|_| self.malformed("text is not UTF-8"))
    }

    fn name(&mut self) -> Result<&'de str> {
        let reference = self.varint()?;
        if reference == 0 {
            let name = self.text()?;
            self.names.push(name);
            return Ok(name);
        }
        usize::try_from(reference - 1)
            .ok()
            .and_then(|index| self.names.get(index).copied())
            .ok_or_else(|| self.malformed("unknown field name reference"))
    }

    fn enter(&mut self) -> Result<()> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            return Err(self.malformed("nesting is too deep"));
        }
        Ok(())
    }

    fn leave(&mut self) {
        self.depth -= 1;
    }

    fn expect(&mut self, tag: u8, what: &str) -> Result<()> {
        if self.byte()? == tag {
            Ok(())
        } else {
            self.position -= 1;
            Err(self.malformed(&format!("expected {what}")))
        }
    }

    fn sequence<V: Visitor<'de>>(&mut self, open: bool, visitor: V) -> Result<V::Value> {
        let remaining = if open { None } else { Some(self.count()?) };
        self.enter()?;
        let mut elements = Elements {
            decoder: &mut *self,
            remaining,
        };
        let value = visitor.visit_seq(&mut elements)?;
        let unread = elements.remaining;
        self.close(unread)?;
        self.leave();
        Ok(value)
    }

    fn map<V: Visitor<'de>>(&mut self, open: bool, visitor: V) -> Result<V::Value> {
        let remaining = if open { None } else { Some(self.count()?) };
        self.enter()?;
        let mut entries = Entries {
            decoder: &mut *self,
            remaining,
        };
        let value = visitor.visit_map(&mut entries)?;
        let unread = entries.remaining;
        self.close(unread)?;
        self.leave();
        Ok(value)
    }

    fn fields<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        let remaining = self.count()?;
        self.enter()?;
        let mut fields = Fields {
            decoder: &mut *self,
            remaining,
        };
        let value = visitor.visit_map(&mut fields)?;
        let unread = fields.remaining;
        self.close(Some(unread))?;
        self.leave();
        Ok(value)
    }

    /// A visitor must read everything a sequence, map or struct holds: an
    /// open one ends at its marker, a counted one at its count.
    fn close(&mut self, unread: Option<usize>) -> Result<()> {
        match unread {
            None => self.expect(END, "the end of an open sequence"),
            Some(0) => Ok(()),
            Some(_) => Err(self.malformed("a value holds more than its type reads")),
        }
    }

    fn variant<V: Visitor<'de>>(&mut self, visitor: V) -> Result<V::Value> {
        self.enter()?;
        let value = visitor.visit_enum(Variant {
            decoder: &mut *self,
        })?;
        self.leave();
        Ok(value)
    }
}

impl<'de> de::Deserializer<'de> for &mut Decoder<'de> {
    type Error = RecordCodecError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.byte()? {
            NONE => visitor.visit_none(),
            SOME => {
                self.enter()?;
                let value = visitor.visit_some(&mut *self)?;
                self.leave();
                Ok(value)
            }
            UNIT => visitor.visit_unit(),
            FALSE => visitor.visit_bool(false),
            TRUE => visitor.visit_bool(true),
            UINT => visitor.visit_u64(self.varint()?),
            NINT => visitor.visit_i64(!(self.varint()? as i64)),
            F64 => visitor.visit_f64(f64::from_le_bytes(self.array()?)),
            F32 => visitor.visit_f32(f32::from_le_bytes(self.array()?)),
            STR => visitor.visit_borrowed_str(self.text()?),
            BYTES => {
                let len = self.count()?;
                visitor.visit_borrowed_bytes(self.take(len)?)
            }
            SEQ => self.sequence(false, visitor),
            OPEN_SEQ => self.sequence(true, visitor),
            MAP => self.map(false, visitor),
            OPEN_MAP => self.map(true, visitor),
            STRUCT => self.fields(visitor),
            UNIT_VARIANT | NEWTYPE_VARIANT | TUPLE_VARIANT | STRUCT_VARIANT => {
                self.position -= 1;
                self.variant(visitor)
            }
            CHAR => {
                let code = u32::try_from(self.varint()?)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or_else(|| self.malformed("invalid character"))?;
                visitor.visit_char(code)
            }
            U128 => visitor.visit_u128(u128::from_le_bytes(self.array()?)),
            I128 => visitor.visit_i128(i128::from_le_bytes(self.array()?)),
            _ => {
                self.position -= 1;
                Err(self.malformed("unknown value tag"))
            }
        }
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.expect(STRUCT, "a struct")?;
        self.fields(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        match self.peek()? {
            UNIT_VARIANT | NEWTYPE_VARIANT | TUPLE_VARIANT | STRUCT_VARIANT => {
                self.variant(visitor)
            }
            _ => Err(self.malformed("expected an enum variant")),
        }
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_any(de::IgnoredAny)?;
        visitor.visit_unit()
    }

    fn is_human_readable(&self) -> bool {
        false
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct seq tuple tuple_struct map identifier
    }
}

struct Elements<'a, 'de> {
    decoder: &'a mut Decoder<'de>,
    /// `None` for an open sequence, which runs to its end marker.
    remaining: Option<usize>,
}

impl<'de> de::SeqAccess<'de> for Elements<'_, 'de> {
    type Error = RecordCodecError;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        match &mut self.remaining {
            Some(0) => return Ok(None),
            Some(remaining) => *remaining -= 1,
            None if self.decoder.peek()? == END => return Ok(None),
            None => {}
        }
        seed.deserialize(&mut *self.decoder).map(Some)
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

struct Entries<'a, 'de> {
    decoder: &'a mut Decoder<'de>,
    remaining: Option<usize>,
}

impl<'de> de::MapAccess<'de> for Entries<'_, 'de> {
    type Error = RecordCodecError;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        match &mut self.remaining {
            Some(0) => return Ok(None),
            Some(remaining) => *remaining -= 1,
            None if self.decoder.peek()? == END => return Ok(None),
            None => {}
        }
        seed.deserialize(&mut *self.decoder).map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut *self.decoder)
    }

    fn size_hint(&self) -> Option<usize> {
        self.remaining
    }
}

/// A struct's fields, keyed by name for the derived visitor.
struct Fields<'a, 'de> {
    decoder: &'a mut Decoder<'de>,
    remaining: usize,
}

impl<'de> de::MapAccess<'de> for Fields<'_, 'de> {
    type Error = RecordCodecError;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        if self.remaining == 0 {
            return Ok(None);
        }
        self.remaining -= 1;
        let name = self.decoder.name()?;
        seed.deserialize(de::value::BorrowedStrDeserializer::new(name))
            .map(Some)
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        seed.deserialize(&mut *self.decoder)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct Variant<'a, 'de> {
    decoder: &'a mut Decoder<'de>,
}

impl<'a, 'de> de::EnumAccess<'de> for Variant<'a, 'de> {
    type Error = RecordCodecError;
    type Variant = VariantContent<'a, 'de>;

    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant)> {
        let tag = self.decoder.byte()?;
        let index = u32::try_from(self.decoder.varint()?)
            .map_err(|_| self.decoder.malformed("variant index overflows"))?;
        let variant = seed.deserialize(index.into_deserializer())?;
        Ok((
            variant,
            VariantContent {
                decoder: self.decoder,
                tag,
            },
        ))
    }
}

struct VariantContent<'a, 'de> {
    decoder: &'a mut Decoder<'de>,
    tag: u8,
}

impl VariantContent<'_, '_> {
    fn shape(&self, tag: u8, what: &str) -> Result<()> {
        if self.tag == tag {
            Ok(())
        } else {
            Err(self.decoder.malformed(&format!("expected {what}")))
        }
    }
}

impl<'de> de::VariantAccess<'de> for VariantContent<'_, 'de> {
    type Error = RecordCodecError;

    fn unit_variant(self) -> Result<()> {
        self.shape(UNIT_VARIANT, "a unit variant")
    }

    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value> {
        self.shape(NEWTYPE_VARIANT, "a newtype variant")?;
        seed.deserialize(&mut *self.decoder)
    }

    fn tuple_variant<V: Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        self.shape(TUPLE_VARIANT, "a tuple variant")?;
        self.decoder.sequence(false, visitor)
    }

    fn struct_variant<V: Visitor<'de>>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.shape(STRUCT_VARIANT, "a struct variant")?;
        self.decoder.fields(visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::{decode, encode};
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    enum Instruction {
        Push(f64),
        Load(usize),
        Add,
        Jump { offset: i64, taken: bool },
        Pair(u8, i16),
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct Program {
        name: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        bound: Option<f64>,
        instructions: Vec<Instruction>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        notes: Vec<String>,
        table: BTreeMap<String, (u32, Option<char>)>,
        wide: u128,
        unit: (),
    }

    fn program(bound: Option<f64>, notes: Vec<String>) -> Program {
        let mut table = BTreeMap::new();
        table.insert("alpha".to_owned(), (7, Some('Ω')));
        table.insert("beta".to_owned(), (u32::MAX, None));
        Program {
            name: "kernel".into(),
            bound,
            instructions: vec![
                Instruction::Push(-1.5e-300),
                Instruction::Load(usize::MAX),
                Instruction::Add,
                Instruction::Jump {
                    offset: i64::MIN,
                    taken: true,
                },
                Instruction::Pair(255, -32768),
            ],
            notes,
            table,
            wide: u128::MAX - 3,
            unit: (),
        }
    }

    #[test]
    fn values_round_trip_with_and_without_skipped_fields() {
        for value in [
            program(None, Vec::new()),
            program(Some(f64::INFINITY), vec!["one".into(), "two".into()]),
        ] {
            let bytes = encode(&value).expect("encode");
            assert_eq!(decode::<Program>(&bytes).expect("decode"), value);
        }
    }

    #[test]
    fn a_field_name_is_spelled_once_per_record() {
        let programs = vec![program(None, Vec::new()); 64];
        let bytes = encode(&programs).expect("encode");
        let spelled = bytes
            .windows(b"instructions".len())
            .filter(|window| *window == b"instructions")
            .count();
        assert_eq!(spelled, 1);
        assert_eq!(decode::<Vec<Program>>(&bytes).expect("decode"), programs);
    }

    /// A float a JSON-oriented type writes as a label still reads as itself,
    /// through `deserialize_any`, as `json_float` does.
    #[test]
    fn a_value_read_through_any_keeps_its_written_kind() {
        #[derive(Serialize)]
        struct Written {
            finite: f64,
            label: &'static str,
        }
        #[derive(Debug, PartialEq, Deserialize)]
        struct Read {
            finite: serde_json::Value,
            label: serde_json::Value,
        }
        let bytes = encode(&Written {
            finite: 2.5,
            label: "inf",
        })
        .expect("encode");
        assert_eq!(
            decode::<Read>(&bytes).expect("decode"),
            Read {
                finite: serde_json::json!(2.5),
                label: serde_json::json!("inf"),
            }
        );
    }

    #[test]
    fn an_iterator_of_unknown_length_round_trips() {
        struct Unsized(Vec<u16>);
        impl Serialize for Unsized {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_seq(self.0.iter().filter(|_| true))
            }
        }
        let bytes = encode(&Unsized(vec![1, 300, 65535])).expect("encode");
        assert_eq!(
            decode::<Vec<u16>>(&bytes).expect("decode"),
            vec![1, 300, 65535]
        );
    }

    /// Every truncation and every single-byte change of a valid record either
    /// decodes or is refused; none panics or allocates beyond the input.
    #[test]
    fn damaged_records_are_refused_without_panicking() {
        let bytes = encode(&program(Some(3.0), vec!["note".into()])).expect("encode");
        for len in 0..bytes.len() {
            assert!(decode::<Program>(&bytes[..len]).is_err(), "prefix {len}");
        }
        for index in 0..bytes.len() {
            for flip in [0x01_u8, 0x80, 0xff] {
                let mut damaged = bytes.clone();
                damaged[index] ^= flip;
                let _ = decode::<Program>(&damaged);
            }
        }
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(decode::<Program>(&trailing).is_err());
    }

    #[test]
    fn hostile_lengths_and_nesting_are_refused() {
        // A sequence claiming more elements than bytes remain.
        assert!(decode::<Vec<u8>>(&[super::SEQ, 0xff, 0xff, 0xff, 0xff, 0x0f]).is_err());
        // A varint longer than 64 bits.
        assert!(
            decode::<u64>(&[
                super::UINT,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0xff,
                0x7f
            ])
            .is_err()
        );
        // Options nested past the depth bound.
        let mut nested = vec![super::SOME; 4096];
        nested.push(super::UNIT);
        assert!(decode::<serde_json::Value>(&nested).is_err());
    }
}

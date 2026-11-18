//! The canonical JSON encoding of a value, written as the value serializes.
//!
//! The encoding is the one a digest has always hashed: the value as
//! `serde_json` would represent it, with every object's keys in byte order,
//! no whitespace, and numbers and strings as `serde_json` writes them. A
//! drawing of 8,000 parts used to be turned into a `serde_json::Value` tree
//! first, which allocated every key and node; this writes the same bytes
//! straight from the value. Only an object's entries are buffered, to put its
//! keys in order.

use std::cell::RefCell;
use std::fmt::Display;
use std::io::Write as _;

use serde::Serialize;
use serde::ser::{self, Impossible};

/// The struct name `serde_json` gives a raw JSON value, whose one field is
/// the raw text.
const RAW_VALUE: &str = "$serde_json::private::RawValue";

/// Append the canonical encoding of `value` to `out`.
pub(super) fn write<T: ?Sized + Serialize>(value: &T, out: &mut Vec<u8>) -> Result<(), Error> {
    value.serialize(Canonical { out })
}

/// Why a value has no canonical encoding: a map key that is not a string, a
/// number JSON cannot hold, or the value's own serialization error.
#[derive(Debug)]
pub(super) struct Error(String);

impl Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        Self(message.to_string())
    }
}

fn key_must_be_a_string() -> Error {
    Error("key must be a string".to_owned())
}

fn number_out_of_range() -> Error {
    Error("number out of range".to_owned())
}

fn json(error: serde_json::Error) -> Error {
    Error(error.to_string())
}

/// A string as JSON writes it, quoted and escaped.
fn string(out: &mut Vec<u8>, value: &str) -> Result<(), Error> {
    serde_json::to_writer(out, value).map_err(json)
}

/// A float as `serde_json` holds it in a value: a finite one in its shortest
/// round-trip form, any other as `null`.
fn float(out: &mut Vec<u8>, value: f64) -> Result<(), Error> {
    serde_json::to_writer(out, &value).map_err(json)
}

fn integer(out: &mut Vec<u8>, value: impl Display) {
    let _ = write!(out, "{value}");
}

thread_local! {
    /// Entry buffers of objects that have been written, kept for the next.
    static SPARE: RefCell<Vec<(Vec<u8>, Vec<Entry>)>> = const { RefCell::new(Vec::new()) };
}

/// One object entry in its object's buffer: the key's bytes, then the
/// value's canonical bytes, from `key` to `value` to `end`.
#[derive(Clone, Copy)]
struct Entry {
    key: usize,
    value: usize,
    end: usize,
}

struct Canonical<'a> {
    out: &'a mut Vec<u8>,
}

impl<'a> ser::Serializer for Canonical<'a> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Sequence<'a>;
    type SerializeTuple = Sequence<'a>;
    type SerializeTupleStruct = Sequence<'a>;
    type SerializeTupleVariant = Sequence<'a>;
    type SerializeMap = Object<'a>;
    type SerializeStruct = Object<'a>;
    type SerializeStructVariant = Object<'a>;

    fn serialize_bool(self, value: bool) -> Result<(), Error> {
        self.out
            .extend_from_slice(if value { b"true" } else { b"false" });
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<(), Error> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i16(self, value: i16) -> Result<(), Error> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i32(self, value: i32) -> Result<(), Error> {
        self.serialize_i64(i64::from(value))
    }

    fn serialize_i64(self, value: i64) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<(), Error> {
        if let Ok(value) = u64::try_from(value) {
            integer(self.out, value);
        } else if let Ok(value) = i64::try_from(value) {
            integer(self.out, value);
        } else {
            return Err(number_out_of_range());
        }
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<(), Error> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u16(self, value: u16) -> Result<(), Error> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u32(self, value: u32) -> Result<(), Error> {
        self.serialize_u64(u64::from(value))
    }

    fn serialize_u64(self, value: u64) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<(), Error> {
        let value = u64::try_from(value).map_err(|_| number_out_of_range())?;
        integer(self.out, value);
        Ok(())
    }

    /// Held as the double it widens to, as a JSON value holds it.
    fn serialize_f32(self, value: f32) -> Result<(), Error> {
        float(self.out, f64::from(value))
    }

    fn serialize_f64(self, value: f64) -> Result<(), Error> {
        float(self.out, value)
    }

    fn serialize_char(self, value: char) -> Result<(), Error> {
        string(self.out, value.encode_utf8(&mut [0; 4]))
    }

    fn serialize_str(self, value: &str) -> Result<(), Error> {
        string(self.out, value)
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<(), Error> {
        self.out.push(b'[');
        for (index, byte) in value.iter().enumerate() {
            if index != 0 {
                self.out.push(b',');
            }
            integer(self.out, byte);
        }
        self.out.push(b']');
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<(), Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Error> {
        self.out.extend_from_slice(b"null");
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        string(self.out, variant)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(self)
    }

    /// `{"variant":value}`: an object of one key needs no ordering.
    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.out.push(b'{');
        string(self.out, variant)?;
        self.out.push(b':');
        value.serialize(Canonical {
            out: &mut *self.out,
        })?;
        self.out.push(b'}');
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Sequence<'a>, Error> {
        self.out.push(b'[');
        Ok(Sequence {
            out: self.out,
            first: true,
            variant: false,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Sequence<'a>, Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Sequence<'a>, Error> {
        self.serialize_seq(Some(len))
    }

    /// `{"variant":[fields]}`.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Sequence<'a>, Error> {
        self.out.push(b'{');
        string(self.out, variant)?;
        self.out.extend_from_slice(b":[");
        Ok(Sequence {
            out: self.out,
            first: true,
            variant: true,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Object<'a>, Error> {
        Ok(Object::new(self.out, Shape::Object))
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Object<'a>, Error> {
        Ok(Object::new(
            self.out,
            if name == RAW_VALUE {
                Shape::Raw
            } else {
                Shape::Object
            },
        ))
    }

    /// `{"variant":{fields}}`.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Object<'a>, Error> {
        self.out.push(b'{');
        string(self.out, variant)?;
        self.out.push(b':');
        Ok(Object::new(self.out, Shape::Variant))
    }
}

struct Sequence<'a> {
    out: &'a mut Vec<u8>,
    first: bool,
    /// Closes the object a tuple variant wraps its fields in, too.
    variant: bool,
}

impl Sequence<'_> {
    fn element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        if !std::mem::take(&mut self.first) {
            self.out.push(b',');
        }
        value.serialize(Canonical {
            out: &mut *self.out,
        })
    }

    fn close(self) {
        self.out.push(b']');
        if self.variant {
            self.out.push(b'}');
        }
    }
}

impl ser::SerializeSeq for Sequence<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.element(value)
    }

    fn end(self) -> Result<(), Error> {
        self.close();
        Ok(())
    }
}

impl ser::SerializeTuple for Sequence<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.element(value)
    }

    fn end(self) -> Result<(), Error> {
        self.close();
        Ok(())
    }
}

impl ser::SerializeTupleStruct for Sequence<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.element(value)
    }

    fn end(self) -> Result<(), Error> {
        self.close();
        Ok(())
    }
}

impl ser::SerializeTupleVariant for Sequence<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.element(value)
    }

    fn end(self) -> Result<(), Error> {
        self.close();
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Object,
    /// A struct variant's fields, closed with the object that wraps them.
    Variant,
    /// A raw JSON value, re-read and written canonically.
    Raw,
}

/// An object's entries, buffered so they can be written in key order.
struct Object<'a> {
    out: &'a mut Vec<u8>,
    shape: Shape,
    buffer: Vec<u8>,
    entries: Vec<Entry>,
    key: Option<usize>,
}

impl<'a> Object<'a> {
    fn new(out: &'a mut Vec<u8>, shape: Shape) -> Self {
        let (buffer, entries) = SPARE
            .with(|spare| spare.borrow_mut().pop())
            .unwrap_or_default();
        Self {
            out,
            shape,
            buffer,
            entries,
            key: None,
        }
    }

    fn key<K: ?Sized + Serialize>(&mut self, key: &K) -> Result<(), Error> {
        let start = self.buffer.len();
        key.serialize(Key {
            out: &mut self.buffer,
        })?;
        self.key = Some(start);
        Ok(())
    }

    fn value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        let key = self.key.take().expect("an object value follows its key");
        let start = self.buffer.len();
        value.serialize(Canonical {
            out: &mut self.buffer,
        })?;
        self.entries.push(Entry {
            key,
            value: start,
            end: self.buffer.len(),
        });
        Ok(())
    }

    /// Write the entries in key order; of two with one key, the later one,
    /// as a JSON object keeps it.
    fn close(mut self) -> Result<(), Error> {
        let buffer = &self.buffer;
        let key_of = |entry: &Entry| &buffer[entry.key..entry.value];
        if self.shape == Shape::Raw {
            let raw = self
                .entries
                .first()
                .map(|entry| &buffer[entry.value..entry.end])
                .ok_or_else(|| Error("a raw value holds no text".to_owned()))?;
            let text: String = serde_json::from_slice(raw).map_err(json)?;
            let value: serde_json::Value = serde_json::from_str(&text).map_err(json)?;
            super::write_canonical_json(&value, self.out).map_err(Error)?;
        } else {
            self.entries
                .sort_by(|left, right| key_of(left).cmp(key_of(right)));
            self.out.push(b'{');
            let mut first = true;
            for (index, entry) in self.entries.iter().enumerate() {
                if self
                    .entries
                    .get(index + 1)
                    .is_some_and(|next| key_of(next) == key_of(entry))
                {
                    continue;
                }
                if !std::mem::take(&mut first) {
                    self.out.push(b',');
                }
                let key =
                    std::str::from_utf8(key_of(entry)).map_err(|error| Error(error.to_string()))?;
                string(self.out, key)?;
                self.out.push(b':');
                self.out.extend_from_slice(&buffer[entry.value..entry.end]);
            }
            self.out.push(b'}');
            if self.shape == Shape::Variant {
                self.out.push(b'}');
            }
        }
        self.buffer.clear();
        self.entries.clear();
        let spare = (
            std::mem::take(&mut self.buffer),
            std::mem::take(&mut self.entries),
        );
        SPARE.with(|pool| pool.borrow_mut().push(spare));
        Ok(())
    }
}

impl ser::SerializeMap for Object<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Error> {
        self.key(key)
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.value(value)
    }

    fn end(self) -> Result<(), Error> {
        self.close()
    }
}

impl ser::SerializeStruct for Object<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.key(key)?;
        self.value(value)
    }

    fn end(self) -> Result<(), Error> {
        self.close()
    }
}

impl ser::SerializeStructVariant for Object<'_> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.key(key)?;
        self.value(value)
    }

    fn end(self) -> Result<(), Error> {
        self.close()
    }
}

/// A map key as a JSON object holds it: the raw text, unescaped, which the
/// object orders by and then writes as a string.
struct Key<'a> {
    out: &'a mut Vec<u8>,
}

impl ser::Serializer for Key<'_> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTuple = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, value: bool) -> Result<(), Error> {
        self.out
            .extend_from_slice(if value { b"true" } else { b"false" });
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<(), Error> {
        integer(self.out, value);
        Ok(())
    }

    /// In its own shortest form, as a JSON object key holds a float.
    fn serialize_f32(self, value: f32) -> Result<(), Error> {
        if !value.is_finite() {
            return Err(Error("float key must be finite".to_owned()));
        }
        serde_json::to_writer(self.out, &value).map_err(json)
    }

    fn serialize_f64(self, value: f64) -> Result<(), Error> {
        if !value.is_finite() {
            return Err(Error("float key must be finite".to_owned()));
        }
        serde_json::to_writer(self.out, &value).map_err(json)
    }

    fn serialize_char(self, value: char) -> Result<(), Error> {
        self.out
            .extend_from_slice(value.encode_utf8(&mut [0; 4]).as_bytes());
        Ok(())
    }

    fn serialize_str(self, value: &str) -> Result<(), Error> {
        self.out.extend_from_slice(value.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<(), Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_none(self) -> Result<(), Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<(), Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit(self) -> Result<(), Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<(), Error> {
        self.out.extend_from_slice(variant.as_bytes());
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<(), Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Err(key_must_be_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Err(key_must_be_a_string())
    }

    fn collect_str<T: ?Sized + Display>(self, value: &T) -> Result<(), Error> {
        let _ = write!(self.out, "{value}");
        Ok(())
    }
}

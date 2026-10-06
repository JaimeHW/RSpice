//! Count typed payload numbers without allocating a JSON tree or byte buffer.
use serde::ser::{self, Serialize};

#[derive(Debug)]
struct Error;
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("numeric counting failed")
    }
}
impl std::error::Error for Error {}
impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(_: T) -> Self {
        Self
    }
}

#[derive(Default)]
struct Counter(usize);

pub(super) fn count(value: &impl Serialize) -> usize {
    let mut counter = Counter::default();
    match value.serialize(&mut counter) {
        Ok(()) => counter.0,
        Err(_) => usize::MAX,
    }
}

macro_rules! number {
    ($($name:ident($ty:ty)),* $(,)?) => { $(
        fn $name(self, _: $ty) -> Result<(), Error> {
            self.0 = self.0.saturating_add(1); Ok(())
        }
    )*};
}

impl ser::Serializer for &mut Counter {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    number!(
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
        serialize_f32(f32),
        serialize_f64(f64)
    );
    fn serialize_bool(self, _: bool) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_char(self, _: char) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_str(self, _: &str) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_bytes(self, value: &[u8]) -> Result<(), Error> {
        self.0 = self.0.saturating_add(value.len());
        Ok(())
    }
    fn serialize_none(self) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), Error> {
        value.serialize(self)
    }
    fn serialize_unit(self) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_unit_variant(self, _: &'static str, _: u32, _: &'static str) -> Result<(), Error> {
        Ok(())
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        value.serialize(self)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self, Error> {
        Ok(self)
    }
    fn serialize_tuple(self, _: usize) -> Result<Self, Error> {
        Ok(self)
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> Result<Self, Error> {
        Ok(self)
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, Error> {
        Ok(self)
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self, Error> {
        Ok(self)
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self, Error> {
        Ok(self)
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        _: &'static str,
        _: usize,
    ) -> Result<Self, Error> {
        Ok(self)
    }
}

macro_rules! sequence {
    ($trait:ident, $method:ident $(, $key:ident: $key_ty:ty)?) => {
        impl ser::$trait for &mut Counter {
            type Ok = ();
            type Error = Error;
            fn $method<T: Serialize + ?Sized>(&mut self, $($key: $key_ty,)? value: &T) -> Result<(), Error> {
                $(let _ = $key;)?
                value.serialize(&mut **self)
            }
            fn end(self) -> Result<(), Error> { Ok(()) }
        }
    }
}
sequence!(SerializeSeq, serialize_element);
sequence!(SerializeTuple, serialize_element);
sequence!(SerializeTupleStruct, serialize_field);
sequence!(SerializeTupleVariant, serialize_field);
sequence!(SerializeStruct, serialize_field, key: &'static str);
sequence!(SerializeStructVariant, serialize_field, key: &'static str);
impl ser::SerializeMap for &mut Counter {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
        key.serialize(&mut **self)
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        value.serialize(&mut **self)
    }
    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn counts_nested_numeric_values_without_counting_names_or_flags() {
        let payload = serde_json::json!({"schema": 3, "name": "123", "rows": [[1,2],[3.5]], "flag": true, "missing": null});
        assert_eq!(super::count(&payload), 4);
    }
}

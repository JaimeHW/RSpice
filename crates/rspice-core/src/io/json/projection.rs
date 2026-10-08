//! Inspect only typed branches containing a potentially rounded source integer.

use super::Suspect;
use serde::ser::{self, Error as _, Serialize, SerializeMap as _};

#[derive(Debug)]
pub(super) struct Error {
    path: String,
    detail: String,
}

impl Error {
    fn at(mut self, key: impl std::fmt::Display) -> Self {
        let key = key.to_string().replace('~', "~0").replace('/', "~1");
        self.path = format!("/{key}{}", self.path);
        self
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            f.write_str(&self.detail)
        } else {
            write!(f, "{}: {}", self.path, self.detail)
        }
    }
}
impl std::error::Error for Error {}
impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(message: T) -> Self {
        Self {
            path: String::new(),
            detail: message.to_string(),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Check<'a, 'source> {
    node: Option<&'a Suspect<'source>>,
}

impl<'a, 'source> Check<'a, 'source> {
    pub(super) fn new(node: &'a Suspect<'source>) -> Self {
        Self { node: Some(node) }
    }

    fn child(self, name: &str) -> Self {
        Self {
            node: match self.node {
                Some(Suspect::Object(fields)) => fields.get(name),
                _ => None,
            },
        }
    }

    fn field<T: Serialize + ?Sized>(self, name: &str, value: &T) -> Result<(), Error> {
        let child = self.child(name);
        if child.node.is_some() {
            value.serialize(child).map_err(|error| error.at(name))?;
        }
        Ok(())
    }

    fn float(self) -> Result<(), Error> {
        if let Some(Suspect::Integer(spelling)) = self.node {
            return Err(Error::custom(format!(
                "JSON integer {spelling} cannot be represented exactly as f64"
            )));
        }
        Ok(())
    }
}

macro_rules! ignore_scalar {
    ($($method:ident($ty:ty)),* $(,)?) => { $(
        fn $method(self, _: $ty) -> Result<(), Error> { Ok(()) }
    )* };
}

impl<'a, 'source> ser::Serializer for Check<'a, 'source> {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Sequence<'a, 'source>;
    type SerializeTuple = Sequence<'a, 'source>;
    type SerializeTupleStruct = Sequence<'a, 'source>;
    type SerializeTupleVariant = Sequence<'a, 'source>;
    type SerializeMap = Members<'a, 'source>;
    type SerializeStruct = Members<'a, 'source>;
    type SerializeStructVariant = Members<'a, 'source>;

    // A native integer field has not been cast to a float; serde itself checks
    // its width. Do not impose binary64's precision on seeds, indices or IDs.
    ignore_scalar!(
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
        serialize_bool(bool),
        serialize_char(char),
        serialize_str(&str),
        serialize_bytes(&[u8])
    );

    fn serialize_f32(self, _: f32) -> Result<(), Error> {
        self.float()
    }
    fn serialize_f64(self, _: f64) -> Result<(), Error> {
        self.float()
    }
    fn serialize_none(self) -> Result<(), Error> {
        Ok(())
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
    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<(), Error> {
        value.serialize(self)
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
        variant: &'static str,
        value: &T,
    ) -> Result<(), Error> {
        self.field(variant, value)
    }
    fn serialize_seq(self, _: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(Sequence::new(self, None))
    }
    fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Error> {
        Ok(Sequence::new(self, None))
    }
    fn serialize_tuple_struct(
        self,
        _: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Ok(Sequence::new(self, None))
    }
    fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Ok(Sequence::new(self.child(variant), Some(variant)))
    }
    fn serialize_map(self, _: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(Members::new(self, None))
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> Result<Self::SerializeStruct, Error> {
        Ok(Members::new(self, None))
    }
    fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Ok(Members::new(self.child(variant), Some(variant)))
    }
}

pub(super) struct Sequence<'a, 'source> {
    check: Check<'a, 'source>,
    index: usize,
    variant: Option<&'static str>,
}
impl<'a, 'source> Sequence<'a, 'source> {
    fn new(check: Check<'a, 'source>, variant: Option<&'static str>) -> Self {
        Self {
            check,
            index: 0,
            variant,
        }
    }
    fn element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        if let Some(Suspect::Array(elements)) = self.check.node
            && let Some(child) = elements.get(&self.index)
        {
            value.serialize(Check::new(child)).map_err(|error| {
                let error = error.at(self.index);
                match self.variant {
                    Some(variant) => error.at(variant),
                    None => error,
                }
            })?;
        }
        self.index += 1;
        Ok(())
    }
}

macro_rules! sequence {
    ($trait:ident, $method:ident) => {
        impl ser::$trait for Sequence<'_, '_> {
            type Ok = ();
            type Error = Error;
            fn $method<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
                self.element(value)
            }
            fn end(self) -> Result<(), Error> {
                Ok(())
            }
        }
    };
}
sequence!(SerializeSeq, serialize_element);
sequence!(SerializeTuple, serialize_element);
sequence!(SerializeTupleStruct, serialize_field);
sequence!(SerializeTupleVariant, serialize_field);

pub(super) struct Members<'a, 'source> {
    check: Check<'a, 'source>,
    key: Option<String>,
    variant: Option<&'static str>,
}
impl<'a, 'source> Members<'a, 'source> {
    fn new(check: Check<'a, 'source>, variant: Option<&'static str>) -> Self {
        Self {
            check,
            key: None,
            variant,
        }
    }
    fn field<T: Serialize + ?Sized>(&self, key: &str, value: &T) -> Result<(), Error> {
        self.check
            .field(key, value)
            .map_err(|error| match self.variant {
                Some(variant) => error.at(variant),
                None => error,
            })
    }
}

macro_rules! structure {
    ($trait:ident) => {
        impl ser::$trait for Members<'_, '_> {
            type Ok = ();
            type Error = Error;
            fn serialize_field<T: Serialize + ?Sized>(
                &mut self,
                key: &'static str,
                value: &T,
            ) -> Result<(), Error> {
                self.field(key, value)
            }
            fn end(self) -> Result<(), Error> {
                Ok(())
            }
        }
    };
}
structure!(SerializeStruct);
structure!(SerializeStructVariant);

impl ser::SerializeMap for Members<'_, '_> {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<(), Error> {
        if self.check.node.is_none() {
            return Ok(());
        }
        // Use JSON's own map-key rules, including integer keys and escaping.
        struct Entry<'a, T: ?Sized>(&'a T);
        impl<T: Serialize + ?Sized> Serialize for Entry<'_, T> {
            fn serialize<S: ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry(self.0, &())?;
                map.end()
            }
        }
        let encoded = serde_json::to_string(&Entry(key)).map_err(Error::custom)?;
        let key = encoded
            .strip_prefix('{')
            .and_then(|text| text.strip_suffix(":null}"))
            .ok_or_else(|| Error::custom("invalid encoded JSON map key"))?;
        self.key = Some(serde_json::from_str(key).map_err(Error::custom)?);
        Ok(())
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<(), Error> {
        if self.check.node.is_none() {
            return Ok(());
        }
        let key = self
            .key
            .take()
            .ok_or_else(|| Error::custom("JSON map value has no key"))?;
        self.field(&key, value)
    }
    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

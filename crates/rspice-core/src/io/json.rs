//! JSON decoding that refuses numeric information loss at typed boundaries.
//!
//! A sparse source inspection rejects decimal underflow and remembers only
//! integer literals that would round in binary64. A projection of the decoded
//! type then distinguishes integer metadata from floating-point fields, even
//! inside serde's internally tagged enums. No second JSON value tree is built.

use std::borrow::Cow;
use std::collections::BTreeMap;

use serde::de::{self, DeserializeOwned, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

pub mod numbers;
mod projection;
#[cfg(test)]
mod tests;

/// Failure to decode numerical JSON without losing source information.
#[derive(Debug, thiserror::Error)]
pub(crate) enum JsonDecodeError {
    #[error("JSON decoding aborted")]
    Aborted,
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}

/// Decode binary64 result JSON, rejecting nonzero decimals rounded to zero and
/// integer literals rounded when read into binary64 fields. Native integer
/// fields retain their full range. The caller owns schema and resource policy.
/// This internal wire adapter requires matching serialized and deserialized
/// numeric field paths; aliases or adapters that relocate values must be
/// normalized explicitly before applying the projection check.
pub(crate) fn from_str<T: DeserializeOwned + Serialize>(
    content: &str,
    abort: &dyn crate::AbortSignal,
) -> Result<T, JsonDecodeError> {
    let mut scan = Scan {
        numbers: numbers::Numbers::new(content),
        abort,
        visited: 0,
        aborted: false,
    };
    let mut decoder = serde_json::Deserializer::from_str(content);
    let suspect = (&mut scan)
        .deserialize(&mut decoder)
        .and_then(|suspect| decoder.end().map(|()| suspect));
    if scan.aborted || abort.is_aborted() {
        return Err(JsonDecodeError::Aborted);
    }
    let suspect = suspect?;
    let decoded = serde_json::from_str::<T>(content)?;
    if let Some(suspect) = suspect {
        decoded
            .serialize(projection::Check::new(&suspect))
            .map_err(|error| JsonDecodeError::Json(de::Error::custom(error)))?;
    }
    if abort.is_aborted() {
        return Err(JsonDecodeError::Aborted);
    }
    Ok(decoded)
}

enum Suspect<'a> {
    Integer(&'a str),
    Array(BTreeMap<usize, Suspect<'a>>),
    Object(BTreeMap<Cow<'a, str>, Suspect<'a>>),
}

struct Scan<'a> {
    numbers: numbers::Numbers<'a>,
    abort: &'a dyn crate::AbortSignal,
    visited: usize,
    aborted: bool,
}

impl<'a> Scan<'a> {
    fn number<E: de::Error>(&mut self, value: f64) -> Result<Option<Suspect<'a>>, E> {
        let spelling = self
            .numbers
            .next()
            .ok_or_else(|| E::custom("missing source spelling for JSON number"))?;
        if numbers::decimal_underflowed(spelling, value) {
            return Err(E::custom("JSON number underflows at binary64 precision"));
        }
        Ok(numbers::integer_rounded(spelling, value).then_some(Suspect::Integer(spelling)))
    }
}

impl<'de> DeserializeSeed<'de> for &mut Scan<'de> {
    type Value = Option<Suspect<'de>>;

    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<Self::Value, D::Error> {
        if self.visited.is_multiple_of(256) && self.abort.is_aborted() {
            self.aborted = true;
            return Err(de::Error::custom("JSON decoding aborted"));
        }
        self.visited = self.visited.wrapping_add(1);
        decoder.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for &mut Scan<'de> {
    type Value = Option<Suspect<'de>>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("JSON with representable numeric values")
    }

    fn visit_bool<E: de::Error>(self, _: bool) -> Result<Self::Value, E> {
        Ok(None)
    }
    fn visit_str<E: de::Error>(self, _: &str) -> Result<Self::Value, E> {
        Ok(None)
    }
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        self.number(value as f64)
    }
    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        self.number(value as f64)
    }
    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Self::Value, E> {
        self.number(value)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
        let mut suspects = BTreeMap::new();
        let mut index = 0;
        while let Some(suspect) = sequence.next_element_seed(&mut *self)? {
            if let Some(suspect) = suspect {
                suspects.insert(index, suspect);
            }
            index += 1;
        }
        Ok((!suspects.is_empty()).then_some(Suspect::Array(suspects)))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<Self::Value, A::Error> {
        let mut suspects = BTreeMap::new();
        while let Some(Key(key)) = object.next_key()? {
            if let Some(suspect) = object.next_value_seed(&mut *self)? {
                suspects.insert(key, suspect);
            }
        }
        Ok((!suspects.is_empty()).then_some(Suspect::Object(suspects)))
    }
}

/// Borrow ordinary field names; allocate only for JSON escapes.
struct Key<'a>(Cow<'a, str>);
impl<'de> Deserialize<'de> for Key<'de> {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        struct KeyVisitor;
        impl<'de> Visitor<'de> for KeyVisitor {
            type Value = Key<'de>;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("an object key")
            }
            fn visit_borrowed_str<E: de::Error>(self, key: &'de str) -> Result<Self::Value, E> {
                Ok(Key(Cow::Borrowed(key)))
            }
            fn visit_str<E: de::Error>(self, key: &str) -> Result<Self::Value, E> {
                Ok(Key(Cow::Owned(key.to_owned())))
            }
            fn visit_string<E: de::Error>(self, key: String) -> Result<Self::Value, E> {
                Ok(Key(Cow::Owned(key)))
            }
        }
        decoder.deserialize_str(KeyVisitor)
    }
}

//! Bound table metadata arrays by the already-admitted variable count before
//! serde materializes their indices, unit strings, or escaped variable labels.
use serde::Deserializer;
use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor};

#[derive(Clone, Copy)]
enum Scope {
    Metadata,
    Text,
    Variables,
}

struct Admission {
    scope: Scope,
    variables: usize,
}

pub(super) fn check(source: &str, variables: usize) -> Result<(), serde_json::Error> {
    let mut decoder = serde_json::Deserializer::from_str(source);
    Admission {
        scope: Scope::Metadata,
        variables,
    }
    .deserialize(&mut decoder)?;
    decoder.end()
}

impl<'de> DeserializeSeed<'de> for Admission {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, decoder: D) -> Result<(), D::Error> {
        decoder.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Admission {
    type Value = ();

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("RAW table metadata bounded by its variable count")
    }

    fn visit_unit<E: de::Error>(self) -> Result<(), E> {
        // Optional units and text can be null. The typed decoder below this
        // admission pass still validates required fields and their types.
        Ok(())
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<(), A::Error> {
        while let Some(key) = object.next_key::<String>()? {
            let scope = match (self.scope, key.as_str()) {
                (Scope::Metadata, "real_variables" | "units") | (Scope::Text, "variables") => {
                    Some(Scope::Variables)
                }
                (Scope::Metadata, "text") => Some(Scope::Text),
                _ => None,
            };
            if let Some(scope) = scope {
                object.next_value_seed(Admission {
                    scope,
                    variables: self.variables,
                })?;
            } else {
                object.next_value::<IgnoredAny>()?;
            }
        }
        Ok(())
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut array: A) -> Result<(), A::Error> {
        let mut count = 0usize;
        while array.next_element::<IgnoredAny>()?.is_some() {
            count = count.saturating_add(1);
            if matches!(self.scope, Scope::Variables) && count > self.variables {
                return Err(de::Error::custom(format!(
                    "RAW table metadata array exceeds declared variable count {}",
                    self.variables
                )));
            }
        }
        Ok(())
    }
}

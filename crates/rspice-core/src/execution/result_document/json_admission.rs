//! Admit numerical storage before serde buffers tagged variants or builds vectors.
//!
//! This pass retains no result arrays. The accounting rules mirror
//! `total_value_count`, including missing samples and structural budget slots;
//! the family census tests check that exact budgets remain usable. Schema and
//! version probing precede this pass, and full typed validation follows it.

mod rules;

use super::ResultDocumentError;
use crate::{AbortSignal, ResourceKind, ResourceLimitError, ResourceLimits};
use serde::Deserialize;
use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor};
use std::fmt;

#[derive(Clone, Copy)]
enum Rule {
    Skip,
    Numbers,
    Fixed(usize),
    Items(usize),
    Array(&'static Rule),
    Object(usize, &'static [(&'static str, Rule)]),
    Payload,
    Series,
    Impulse,
}

struct Counter<'a> {
    count: usize,
    visited: usize,
    limits: &'a ResourceLimits,
    abort: &'a dyn AbortSignal,
    failure: Option<ResultDocumentError>,
}

impl Counter<'_> {
    fn fail<E: de::Error>(&mut self, error: ResultDocumentError) -> E {
        let message = error.to_string();
        self.failure = Some(error);
        E::custom(message)
    }

    fn poll<E: de::Error>(&mut self) -> Result<(), E> {
        self.visited = self.visited.wrapping_add(1);
        if self.visited.is_multiple_of(256) && self.abort.is_aborted() {
            return Err(self.fail(ResultDocumentError::Aborted));
        }
        Ok(())
    }

    fn add<E: de::Error>(&mut self, amount: usize) -> Result<(), E> {
        self.count = self.count.saturating_add(amount);
        for (resource, limit) in [
            (ResourceKind::ResultValues, self.limits.max_result_values),
            (
                ResourceKind::ExternalDataValues,
                self.limits.max_external_data_values,
            ),
        ] {
            if let Err(error) = ResourceLimitError::ensure(resource, self.count, limit) {
                return Err(self.fail(ResultDocumentError::ResourceLimit(error)));
            }
        }
        Ok(())
    }
}

pub(super) fn check(
    json: &str,
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<usize, ResultDocumentError> {
    super::check_abort(abort)?;
    let mut counter = Counter {
        count: 0,
        visited: 0,
        limits,
        abort,
        failure: None,
    };
    let result = scan(json, rules::DOCUMENT, &mut counter);
    if let Err(error) = result {
        return Err(counter
            .failure
            .unwrap_or_else(|| ResultDocumentError::Json(error.to_string())));
    }
    super::check_abort(abort)?;
    Ok(counter.count)
}

fn scan(json: &str, rule: Rule, counter: &mut Counter<'_>) -> Result<(), serde_json::Error> {
    let mut decoder = serde_json::Deserializer::from_str(json);
    Seed { rule, counter }.deserialize(&mut decoder)?;
    decoder.end()
}

struct Seed<'a, 'b> {
    rule: Rule,
    counter: &'a mut Counter<'b>,
}

impl<'de> DeserializeSeed<'de> for Seed<'_, '_> {
    type Value = ();

    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<(), D::Error> {
        self.counter.poll()?;
        match self.rule {
            Rule::Fixed(count) => {
                // Called only after SeqAccess has found another element. Charge
                // it before parsing even a null or a malformed sample record.
                self.counter.add(count)?;
                IgnoredAny::deserialize(decoder).map(|_| ())
            }
            Rule::Payload | Rule::Series => {
                // Tags may follow the arrays they describe. Borrow the source,
                // probe the tag without retaining Content/Value, then walk it.
                let source = <&serde_json::value::RawValue>::deserialize(decoder)?;
                let rule = if matches!(self.rule, Rule::Payload) {
                    #[derive(Deserialize)]
                    struct Tag {
                        family: String,
                    }
                    let tag: Tag = serde_json::from_str(source.get()).map_err(de::Error::custom)?;
                    rules::payload(&tag.family).map_err(de::Error::custom)?
                } else {
                    #[derive(Deserialize)]
                    struct Tag {
                        representation: String,
                    }
                    let tag: Tag = serde_json::from_str(source.get()).map_err(de::Error::custom)?;
                    match tag.representation.as_str() {
                        "complex" => rules::COMPLEX_SERIES,
                        "real" | "logic" => rules::REAL_SERIES,
                        _ => {
                            return Err(de::Error::custom("unknown result series representation"));
                        }
                    }
                };
                scan(source.get(), rule, self.counter).map_err(de::Error::custom)
            }
            _ => decoder.deserialize_any(self),
        }
    }
}

impl<'de> Visitor<'de> for Seed<'_, '_> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("result document data")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<(), M::Error> {
        let base = match self.rule {
            Rule::Object(base, _) => base,
            Rule::Impulse => 1,
            _ => 0,
        };
        self.counter.add(base)?;
        let mut name_bytes = 0usize;
        while let Some(key) = map.next_key::<String>()? {
            if matches!(self.rule, Rule::Impulse)
                && matches!(
                    key.as_str(),
                    "branchName" | "deviceName" | "parameter" | "nodeName"
                )
            {
                let name = map.next_value::<String>()?;
                let previous = name_bytes;
                name_bytes = name_bytes.saturating_add(name.len());
                let width = std::mem::size_of::<crate::Value>();
                self.counter
                    .add(name_bytes.div_ceil(width) - previous.div_ceil(width))?;
                continue;
            }
            let rule = match self.rule {
                Rule::Numbers => Rule::Numbers,
                Rule::Object(_, fields) => fields
                    .iter()
                    .find_map(|(name, rule)| (*name == key).then_some(*rule))
                    .unwrap_or(Rule::Skip),
                Rule::Impulse => match key.as_str() {
                    "points" => Rule::Items(2),
                    "derivatives" => Rule::Items(3),
                    _ => Rule::Skip,
                },
                _ => Rule::Skip,
            };
            map.next_value_seed(Seed {
                rule,
                counter: self.counter,
            })?;
        }
        Ok(())
    }

    fn visit_seq<S: SeqAccess<'de>>(self, mut sequence: S) -> Result<(), S::Error> {
        let rule = match self.rule {
            Rule::Items(count) => Rule::Fixed(count),
            Rule::Array(rule) => *rule,
            Rule::Numbers => Rule::Numbers,
            _ => Rule::Skip,
        };
        while sequence
            .next_element_seed(Seed {
                rule,
                counter: self.counter,
            })?
            .is_some()
        {}
        Ok(())
    }

    fn visit_u64<E: de::Error>(self, _: u64) -> Result<(), E> {
        self.number()
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> Result<(), E> {
        self.number()
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<(), E> {
        self.number()
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, _: &str) -> Result<(), E> {
        Ok(())
    }
    fn visit_unit<E: de::Error>(self) -> Result<(), E> {
        Ok(())
    }
}

impl Seed<'_, '_> {
    fn number<E: de::Error>(self) -> Result<(), E> {
        if matches!(self.rule, Rule::Numbers) {
            self.counter.add(1)?;
        }
        Ok(())
    }
}

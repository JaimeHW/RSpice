//! Admit JSON numeric payloads while decoding and reject repeated object keys.

use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};
use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, SeqAccess, Visitor};
use serde_json::Value;

mod header;
pub(crate) use header::{Kind, kind};
use rspice_core::io::json::numbers;

pub(super) fn parse(
    path: &std::path::Path,
    content: &str,
    kind: Kind,
    limits: ResourceLimits,
) -> Result<Value, crate::cli::CliError> {
    let mut admission = Admission {
        numbers: numbers::Numbers::new(content),
        limits,
        count: 0,
        failure: None,
    };
    let mut deserializer = serde_json::Deserializer::from_str(content);
    let decoded = Seed {
        admission: &mut admission,
        scope: if kind == Kind::Fft {
            Scope::AllNumbers
        } else {
            Scope::Table
        },
    }
    .deserialize(&mut deserializer)
    .and_then(|value| deserializer.end().map(|()| value));
    decoded.map_err(|error| match admission.failure {
        Some(source) => crate::cli::CliError::ResourceLimit {
            path: path.to_owned(),
            source,
        },
        None => super::conversion_error(path, error),
    })
}

/// Missing or null legacy metadata is unstated; other non-text values are
/// malformed declarations and must not silently opt out of quantity checks.
pub(super) fn optional_text<'a>(
    path: &std::path::Path,
    value: Option<&'a Value>,
    field: &str,
) -> Result<Option<&'a str>, crate::cli::CliError> {
    match value {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value)),
        _ => Err(super::conversion_error(
            path,
            format!("'{field}' must be a string or null"),
        )),
    }
}

struct Admission<'a> {
    numbers: numbers::Numbers<'a>,
    limits: ResourceLimits,
    count: usize,
    failure: Option<ResourceLimitError>,
}

impl Admission<'_> {
    fn number<E: de::Error>(&mut self, value: f64, scope: Scope) -> Result<(), E> {
        if scope == Scope::AllNumbers {
            self.admit()?;
        }
        // Advance for metadata too: all numeric visitors share source order.
        let spelling = self
            .numbers
            .next()
            .ok_or_else(|| E::custom("missing source spelling for JSON number"))?;
        if matches!(
            scope,
            Scope::Sample | Scope::NullableSample | Scope::AllNumbers
        ) && numbers::decimal_underflowed(spelling, value)
        {
            return Err(E::custom("JSON number underflows at binary64 precision"));
        }
        if matches!(scope, Scope::Sample | Scope::NullableSample)
            && numbers::integer_rounded(spelling, value)
        {
            return Err(E::custom(
                "JSON integer sample cannot be represented exactly as f64",
            ));
        }
        Ok(())
    }

    fn admit<E: de::Error>(&mut self) -> Result<(), E> {
        let requested = self.count.saturating_add(1);
        for (resource, limit) in [
            (
                ResourceKind::ExternalDataValues,
                self.limits.max_external_data_values,
            ),
            (ResourceKind::ResultValues, self.limits.max_result_values),
        ] {
            if requested > limit {
                self.failure = Some(ResourceLimitError {
                    resource,
                    requested,
                    limit,
                });
                return Err(E::custom("JSON numeric value budget exceeded"));
            }
        }
        self.count = requested;
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scope {
    Table,
    Scale,
    Signals,
    Signal,
    Samples,
    Sample,
    NullableSamples,
    NullableSample,
    Other,
    AllNumbers,
}

impl Scope {
    fn field(self, key: &str) -> Self {
        match (self, key) {
            (Self::AllNumbers, _) => Self::AllNumbers,
            (Self::Table, "scale") => Self::Scale,
            (Self::Table, "signals") => Self::Signals,
            (Self::Scale, "values") => Self::Samples,
            (Self::Signal, "values" | "real" | "imag") => Self::NullableSamples,
            _ => Self::Other,
        }
    }

    fn element(self) -> Self {
        match self {
            Self::AllNumbers => Self::AllNumbers,
            Self::Signals => Self::Signal,
            Self::Samples => Self::Sample,
            Self::NullableSamples => Self::NullableSample,
            _ => Self::Other,
        }
    }

    fn non_numeric<E: de::Error>(self) -> Result<(), E> {
        if matches!(self, Self::Sample | Self::NullableSample) {
            Err(E::custom("non-numeric entry in a result sample array"))
        } else {
            Ok(())
        }
    }
}

struct Seed<'a, 'source> {
    admission: &'a mut Admission<'source>,
    scope: Scope,
}

impl<'de> DeserializeSeed<'de> for Seed<'_, '_> {
    type Value = Value;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Value, D::Error> {
        // Charge a table sample before decoding or retaining it. FFTs count
        // every numeric source field, including metadata, in the visitor.
        if matches!(self.scope, Scope::Sample | Scope::NullableSample) {
            self.admission.admit()?;
        }
        deserializer.deserialize_any(self)
    }
}

impl<'de> Visitor<'de> for Seed<'_, '_> {
    type Value = Value;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a JSON value with unique object keys")
    }

    fn visit_bool<E: de::Error>(self, value: bool) -> Result<Value, E> {
        self.scope.non_numeric()?;
        Ok(value.into())
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Value, E> {
        self.admission.number(value as f64, self.scope)?;
        Ok(value.into())
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Value, E> {
        self.admission.number(value as f64, self.scope)?;
        Ok(value.into())
    }

    fn visit_f64<E: de::Error>(self, value: f64) -> Result<Value, E> {
        self.admission.number(value, self.scope)?;
        serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Value, E> {
        self.scope.non_numeric()?;
        Ok(value.into())
    }

    fn visit_string<E: de::Error>(self, value: String) -> Result<Value, E> {
        self.scope.non_numeric()?;
        Ok(value.into())
    }

    fn visit_unit<E: de::Error>(self) -> Result<Value, E> {
        if self.scope != Scope::NullableSample {
            self.scope.non_numeric()?;
        }
        Ok(Value::Null)
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Value, A::Error> {
        self.scope.non_numeric()?;
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(Seed {
            admission: &mut *self.admission,
            scope: self.scope.element(),
        })? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A: MapAccess<'de>>(self, mut object: A) -> Result<Value, A::Error> {
        self.scope.non_numeric()?;
        let mut values = serde_json::Map::new();
        while let Some(key) = object.next_key::<String>()? {
            // Check decoded keys before reading their values. Escaped key
            // spellings are aliases too, and even identical repeated values
            // make the source ambiguous to other readers.
            if values.contains_key(&key) {
                return Err(de::Error::custom(format!("duplicate JSON field {key:?}")));
            }
            let value = object.next_value_seed(Seed {
                admission: &mut *self.admission,
                scope: self.scope.field(&key),
            })?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn numeric_admission_stops_decoding_at_the_first_excess_value() {
        let tail = "0,".repeat(100_000);
        for (scope, content, expected_kind) in [
            (
                Scope::Table,
                format!(r#"{{"signals":[{{"values":[null,null,{tail}0]}}]}}"#),
                Kind::Table,
            ),
            (
                Scope::Table,
                format!(r#"{{"scale":{{"values":[0,1,{tail}0]}},"signals":[]}}"#),
                Kind::Table,
            ),
            // FFT metadata contributes to the same budget even when the
            // format discriminator follows the numeric payload in the file.
            (
                Scope::AllNumbers,
                format!(r#"{{"metadata":[0,1,{tail}0],"analysis":"fft"}}"#),
                Kind::Fft,
            ),
        ] {
            assert_eq!(
                kind(std::path::Path::new("input.json"), &content).unwrap(),
                expected_kind
            );
            for resource in [ResourceKind::ExternalDataValues, ResourceKind::ResultValues] {
                let mut limits = ResourceLimits::default();
                if resource == ResourceKind::ExternalDataValues {
                    limits.max_external_data_values = 1;
                } else {
                    limits.max_result_values = 1;
                }
                let mut admission = Admission {
                    numbers: numbers::Numbers::new(&content),
                    limits,
                    count: 0,
                    failure: None,
                };
                let mut input = Cursor::new(content.as_bytes());
                let result = Seed {
                    admission: &mut admission,
                    scope,
                }
                .deserialize(&mut serde_json::Deserializer::from_reader(&mut input));
                assert!(result.is_err());
                let error = admission.failure.unwrap();
                assert_eq!(error.resource, resource);
                assert_eq!((error.requested, error.limit), (2, 1));
                assert!(
                    input.position() < 128,
                    "parsed {} bytes despite a one-value budget",
                    input.position()
                );
            }
        }
    }

    #[test]
    fn a_non_numeric_sample_is_rejected_without_materializing_its_contents() {
        let content = format!(
            r#"{{"scale":{{"values":[{{"nested":[{}0]}}]}}}}"#,
            "0,".repeat(100_000)
        );
        let mut admission = Admission {
            numbers: numbers::Numbers::new(&content),
            limits: ResourceLimits::default(),
            count: 0,
            failure: None,
        };
        let mut input = Cursor::new(content.as_bytes());
        let error = Seed {
            admission: &mut admission,
            scope: Scope::Table,
        }
        .deserialize(&mut serde_json::Deserializer::from_reader(&mut input))
        .unwrap_err();
        assert!(error.to_string().contains("non-numeric"), "{error}");
        assert!(input.position() < 128);
        assert!(admission.failure.is_none());
    }

    #[test]
    fn fft_underflow_is_rejected_in_metadata_bins_and_harmonics() {
        let path = std::path::Path::new("fft.json");
        for content in [
            r#"{"analysis":"fft","sampling":{"start_time_s":1e-999}}"#,
            r#"{"analysis":"fft","bins":[{"index":0,"real":-1e-999}]}"#,
            r#"{"largest_harmonics":[{"phase_degrees":2e-324}],"analysis":"fft"}"#,
        ] {
            let error = parse(path, content, Kind::Fft, ResourceLimits::default()).unwrap_err();
            assert!(error.to_string().contains("underflow"), "{error}");
            assert!(error.to_string().contains("line"), "{error}");
        }
    }

    #[test]
    fn unrelated_integer_metadata_is_retained_without_float_conversion() {
        let content = r#"{"metadata":{"count":18446744073709551615,"value":9007199254740993},"scale":{"values":[0,1]},"signals":[{"values":[null,2]}]}"#;
        let decoded = parse(
            std::path::Path::new("input.json"),
            content,
            Kind::Table,
            ResourceLimits::default(),
        )
        .unwrap();
        assert_eq!(decoded["metadata"]["count"].as_u64(), Some(u64::MAX));
        assert_eq!(
            decoded["metadata"]["value"].as_u64(),
            Some(9007199254740993)
        );
        assert!(decoded["signals"][0]["values"][0].is_null());
    }
}

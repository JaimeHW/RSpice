//! Check source JSON numbers before their spellings are lost to binary64.
//! Raw values borrow the verified dataset member; no extra sample strings or
//! untyped payload tree are allocated. Coordinates cannot contain nulls.

use serde::de::{Error as _, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};
use serde_json::value::RawValue;

struct Samples<const NULLABLE: bool>(Vec<f64>);

impl<'de, const NULLABLE: bool> Deserialize<'de> for Samples<NULLABLE> {
    fn deserialize<D: Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
        struct SampleVisitor<const NULLABLE: bool>;
        impl<'de, const NULLABLE: bool> Visitor<'de> for SampleVisitor<NULLABLE> {
            type Value = Samples<NULLABLE>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(if NULLABLE {
                    "an array of finite numeric samples or explicit nulls"
                } else {
                    "an array of finite numeric coordinates"
                })
            }

            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<Self::Value, A::Error> {
                let mut values = Vec::new();
                while let Some(raw) = sequence.next_element::<&RawValue>()? {
                    let source = raw.get();
                    let value = if NULLABLE && source == "null" {
                        f64::NAN
                    } else {
                        let value: f64 = serde_json::from_str(source).map_err(A::Error::custom)?;
                        if crate::numeric::decimal_underflowed(source, value) {
                            return Err(A::Error::custom(format!(
                                "numeric sample {} underflows at binary64 precision",
                                values.len()
                            )));
                        }
                        if value.abs() >= crate::numeric::MAX_EXACT_F64_INTEGER as f64
                            && !source.contains(['.', 'e', 'E'])
                            && source != format!("{value:.0}")
                        {
                            return Err(A::Error::custom(format!(
                                "integer sample {} cannot be represented exactly as f64",
                                values.len()
                            )));
                        }
                        value
                    };
                    values.try_reserve(1).map_err(A::Error::custom)?;
                    values.push(value);
                }
                Ok(Samples(values))
            }
        }
        decoder.deserialize_seq(SampleVisitor::<NULLABLE>)
    }
}

pub(super) fn coordinates<'de, D: Deserializer<'de>>(decoder: D) -> Result<Vec<f64>, D::Error> {
    Samples::<false>::deserialize(decoder).map(|samples| samples.0)
}

pub(super) fn optional_signal<'de, D: Deserializer<'de>>(
    decoder: D,
) -> Result<Option<Vec<f64>>, D::Error> {
    Option::<Samples<true>>::deserialize(decoder).map(|samples| samples.map(|samples| samples.0))
}

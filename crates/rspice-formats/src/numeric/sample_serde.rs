//! Explicit nulls at persistence boundaries, NaN gaps only inside numeric code.
//! Infinity is never an unavailable sample and is refused in both directions.

use serde::de::{Error as _, SeqAccess, Visitor};
use serde::ser::{Error as _, SerializeSeq};
use serde::{Deserialize, Deserializer, Serializer};
use std::sync::Arc;

pub fn serialize<S: Serializer>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error> {
    let mut sequence = serializer.serialize_seq(Some(values.len()))?;
    for &value in values {
        if value.is_infinite() {
            return Err(S::Error::custom("infinite waveform sample"));
        }
        sequence.serialize_element(&(!value.is_nan()).then_some(value))?;
    }
    sequence.end()
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<f64>, D::Error> {
    struct Samples;
    impl<'de> Visitor<'de> for Samples {
        type Value = Vec<f64>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("an array of finite numeric samples or explicit nulls")
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
            let mut values = Vec::with_capacity(sequence.size_hint().unwrap_or(0).min(4096));
            while let Some(value) = sequence.next_element::<Option<f64>>()? {
                values.push(match value {
                    Some(value) if value.is_finite() => value,
                    None => f64::NAN,
                    _ => {
                        return Err(A::Error::custom(
                            "non-finite waveform value must be an explicit null",
                        ));
                    }
                });
            }
            Ok(values)
        }
    }
    deserializer.deserialize_seq(Samples)
}

pub fn deserialize_shared<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Arc<Vec<f64>>, D::Error> {
    deserialize(deserializer).map(Arc::new)
}

pub fn deserialize_optional<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Vec<f64>>, D::Error> {
    struct Samples(Vec<f64>);
    impl<'de> Deserialize<'de> for Samples {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            deserialize(deserializer).map(Self)
        }
    }
    Option::<Samples>::deserialize(deserializer).map(|values| values.map(|samples| samples.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Wire {
        #[serde(serialize_with = "serialize", deserialize_with = "deserialize_shared")]
        samples: Arc<Vec<f64>>,
    }

    #[test]
    fn missing_samples_are_null_and_infinities_cannot_be_mislabeled_missing() {
        let wire = Wire {
            samples: Arc::new(vec![1.0, f64::NAN, -0.0]),
        };
        let encoded = serde_json::to_string(&wire).unwrap();
        assert_eq!(encoded, r#"{"samples":[1.0,null,-0.0]}"#);
        let decoded: Wire = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded.samples[0], 1.0);
        assert!(decoded.samples[1].is_nan());
        assert_eq!(decoded.samples[2].to_bits(), (-0.0_f64).to_bits());
        for value in [f64::INFINITY, f64::NEG_INFINITY] {
            assert!(
                serde_json::to_string(&Wire {
                    samples: Arc::new(vec![value])
                })
                .is_err()
            );
        }
        for invalid in [
            r#"{"samples":null}"#,
            r#"{"samples":["NaN"]}"#,
            r#"{"samples":[1e400]}"#,
        ] {
            assert!(serde_json::from_str::<Wire>(invalid).is_err());
        }
    }
}

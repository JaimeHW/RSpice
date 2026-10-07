//! Inspect format discriminators without retaining numeric payloads or metadata.
use serde::de::{self, Deserialize, Deserializer, IgnoredAny, MapAccess, SeqAccess, Visitor};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Kind {
    #[default]
    Table,
    Fft,
    Typed,
}

pub(crate) fn kind(path: &std::path::Path, content: &str) -> Result<Kind, crate::cli::CliError> {
    serde_json::from_str::<Header>(content)
        .map(|header| header.0)
        .map_err(|error| super::super::conversion_error(path, error))
}

struct Header(Kind);

impl<'de> Deserialize<'de> for Header {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct HeaderVisitor;
        impl<'de> Visitor<'de> for HeaderVisitor {
            type Value = Header;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a JSON result object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Header, A::Error> {
                let mut schema = None;
                let mut analysis = None;
                while let Some(key) = map.next_key::<String>()? {
                    let target = match key.as_str() {
                        "schema" => &mut schema,
                        "analysis" => &mut analysis,
                        _ => {
                            map.next_value::<IgnoredAny>()?;
                            continue;
                        }
                    };
                    if target.is_some() {
                        return Err(de::Error::custom(format!("duplicate JSON field {key:?}")));
                    }
                    *target = Some(map.next_value::<Discriminator>()?.0);
                }
                Ok(Header(if schema == Some(Kind::Typed) {
                    Kind::Typed
                } else if analysis == Some(Kind::Fft) {
                    Kind::Fft
                } else {
                    Kind::Table
                }))
            }
        }
        deserializer.deserialize_map(HeaderVisitor)
    }
}

struct Discriminator(Kind);

impl<'de> Deserialize<'de> for Discriminator {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct TextVisitor;
        impl<'de> Visitor<'de> for TextVisitor {
            type Value = Kind;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a result format discriminator")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Kind, E> {
                Ok(match value {
                    rspice_core::execution::ANALYSIS_RESULT_DOCUMENT_SCHEMA => Kind::Typed,
                    "fft" => Kind::Fft,
                    _ => Kind::Table,
                })
            }

            fn visit_bool<E: de::Error>(self, _: bool) -> Result<Kind, E> {
                Ok(Kind::Table)
            }
            fn visit_i64<E: de::Error>(self, _: i64) -> Result<Kind, E> {
                Ok(Kind::Table)
            }
            fn visit_u64<E: de::Error>(self, _: u64) -> Result<Kind, E> {
                Ok(Kind::Table)
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> Result<Kind, E> {
                Ok(Kind::Table)
            }
            fn visit_unit<E: de::Error>(self) -> Result<Kind, E> {
                Ok(Kind::Table)
            }

            fn visit_seq<A: SeqAccess<'de>>(self, sequence: A) -> Result<Kind, A::Error> {
                IgnoredAny.visit_seq(sequence).map(|_| Kind::Table)
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Kind, A::Error> {
                IgnoredAny.visit_map(map).map(|_| Kind::Table)
            }
        }
        deserializer.deserialize_any(TextVisitor).map(Self)
    }
}

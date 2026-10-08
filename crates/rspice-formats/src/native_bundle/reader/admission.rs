//! Count source arrays before the allocating typed decoder sees them.
use super::{NativeBundleError, NativeBundleReadLimits};
use serde::Deserialize;
use serde::de::{self, DeserializeSeed, IgnoredAny, MapAccess, SeqAccess, Visitor};
use std::fmt;

#[derive(Clone, Copy)]
enum Scope {
    Dataset,
    Coordinate,
    Signals,
    Signal,
    Samples(usize),
    Sample { row: usize, weight: usize },
    Skip,
}

struct Admission {
    limits: NativeBundleReadLimits,
    columns: usize,
    values: usize,
    failure: Option<String>,
}

impl Admission {
    fn ensure<E: de::Error>(
        &mut self,
        resource: &str,
        requested: usize,
        limit: usize,
    ) -> Result<(), E> {
        if requested > limit {
            let detail =
                format!("native bundle contains {requested} {resource}; the limit is {limit}");
            let error = E::custom(&detail);
            self.failure = Some(detail);
            return Err(error);
        }
        Ok(())
    }
}

pub(super) fn check(bytes: &[u8], limits: NativeBundleReadLimits) -> Result<(), NativeBundleError> {
    let mut admission = Admission {
        limits,
        columns: 1,
        values: 0,
        failure: None,
    };
    let mut decoder = serde_json::Deserializer::from_slice(bytes);
    let result = admission
        .ensure("columns", 1, limits.max_columns)
        .and_then(|()| {
            Seed {
                admission: &mut admission,
                scope: Scope::Dataset,
            }
            .deserialize(&mut decoder)
        })
        .and_then(|()| decoder.end());
    result.map_err(|source| match admission.failure {
        Some(detail) => NativeBundleError::InvalidData(detail),
        None => NativeBundleError::Json {
            context: "dataset.json is invalid",
            source,
        },
    })
}

struct Seed<'a> {
    admission: &'a mut Admission,
    scope: Scope,
}

impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = ();
    fn deserialize<D: de::Deserializer<'de>>(self, decoder: D) -> Result<(), D::Error> {
        match self.scope {
            Scope::Sample { row, weight } => {
                self.admission
                    .ensure("samples per column", row, self.admission.limits.max_rows)?;
                self.admission.values = self.admission.values.saturating_add(weight);
                self.admission.ensure(
                    "retained numeric values",
                    self.admission.values,
                    self.admission.limits.max_numeric_values,
                )?;
                IgnoredAny::deserialize(decoder).map(|_| ())
            }
            Scope::Skip => IgnoredAny::deserialize(decoder).map(|_| ()),
            _ => {
                if matches!(self.scope, Scope::Signal) {
                    self.admission.columns = self.admission.columns.saturating_add(1);
                    self.admission.ensure(
                        "columns",
                        self.admission.columns,
                        self.admission.limits.max_columns,
                    )?;
                }
                decoder.deserialize_any(self)
            }
        }
    }
}

impl<'de> Visitor<'de> for Seed<'_> {
    type Value = ();
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("native dataset fields")
    }
    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<(), M::Error> {
        while let Some(key) = map.next_key::<String>()? {
            let scope = match (self.scope, key.as_str()) {
                (Scope::Dataset, "coordinate") => Scope::Coordinate,
                (Scope::Dataset, "signals") => Scope::Signals,
                (Scope::Coordinate | Scope::Signal, "values") => Scope::Samples(1),
                // A complex trace retains a derived magnitude alongside its two
                // source arrays. Charge it with real, irrespective of key order.
                (Scope::Signal, "real") => Scope::Samples(2),
                (Scope::Signal, "imag") => Scope::Samples(1),
                _ => Scope::Skip,
            };
            map.next_value_seed(Seed {
                admission: self.admission,
                scope,
            })?;
        }
        Ok(())
    }
    fn visit_seq<S: SeqAccess<'de>>(self, mut sequence: S) -> Result<(), S::Error> {
        let mut row = 0usize;
        loop {
            let scope = match self.scope {
                Scope::Samples(weight) => Scope::Sample {
                    row: row.saturating_add(1),
                    weight,
                },
                Scope::Signals => Scope::Signal,
                _ => Scope::Skip,
            };
            if sequence
                .next_element_seed(Seed {
                    admission: self.admission,
                    scope,
                })?
                .is_none()
            {
                break;
            }
            row = row.saturating_add(1);
        }
        Ok(())
    }
    // Preserve the typed decoder's record-specific errors for malformed shapes.
    fn visit_unit<E: de::Error>(self) -> Result<(), E> {
        Ok(())
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, _: &str) -> Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> Result<(), E> {
        Ok(())
    }
}

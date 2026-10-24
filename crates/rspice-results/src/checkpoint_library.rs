//! Imported checkpoints are project inputs, independent of native result history.
//!
//! The Monte Carlo journals and the transient continuation images share one
//! library shape. It is a macro rather than a generic type because serde
//! names the entry struct in its load errors: each instance keeps its own
//! struct name and messages.

/// Define `$library` holding `$imported` entries of `$evidence`, named in
/// messages as `$label` checkpoints. `$evidence` provides `byte_limit()`,
/// `bytes()` and `digest()`.
macro_rules! checkpoint_library {
    ($library:ident, $imported:ident, $evidence:ty, $label:literal) => {
        #[derive(Debug, Clone, PartialEq, Eq, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $imported {
            name: String,
            checkpoint: $evidence,
        }
        impl $imported {
            pub fn name(&self) -> &str {
                &self.name
            }
            pub fn checkpoint(&self) -> &$evidence {
                &self.checkpoint
            }
        }

        #[derive(Debug, Clone, Default, PartialEq, Eq, ::serde::Serialize)]
        #[serde(transparent)]
        pub struct $library(::std::sync::Arc<Vec<$imported>>);

        impl $library {
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            pub fn iter(&self) -> impl Iterator<Item = &$imported> {
                self.0.iter()
            }
            pub fn get(
                &self,
                digest: ::rspice_vocabulary::product::ContentDigest,
            ) -> Option<&$evidence> {
                self.0
                    .iter()
                    .find(|entry| entry.checkpoint.digest() == digest)
                    .map(|entry| &entry.checkpoint)
            }
            pub fn shares_content_with(&self, other: &Self) -> bool {
                ::std::sync::Arc::ptr_eq(&self.0, &other.0)
            }
            pub fn insert(&mut self, name: String, checkpoint: $evidence) -> Result<bool, String> {
                self.insert_bounded(name, checkpoint, <$evidence>::byte_limit())
            }
            pub fn insert_bounded(
                &mut self,
                name: String,
                checkpoint: $evidence,
                limit: usize,
            ) -> Result<bool, String> {
                if name.trim().is_empty() || name.len() > 512 || name.chars().any(char::is_control) {
                    return Err(
                        "Checkpoint name must contain 1–512 bytes without control characters"
                            .into(),
                    );
                }
                if self.get(checkpoint.digest()).is_some() {
                    return Ok(false);
                }
                let bytes = self.0.iter().fold(
                    checkpoint.bytes().len().saturating_add(name.len()),
                    |total, entry| {
                        total
                            .saturating_add(entry.name.len())
                            .saturating_add(entry.checkpoint.bytes().len())
                    },
                );
                if bytes > limit {
                    return Err("Imported checkpoints exceed the combined storage limit; remove an imported checkpoint first".into());
                }
                ::std::sync::Arc::make_mut(&mut self.0).push($imported { name, checkpoint });
                Ok(true)
            }
            pub fn remove(&mut self, digest: ::rspice_vocabulary::product::ContentDigest) -> bool {
                let Some(index) = self
                    .0
                    .iter()
                    .position(|entry| entry.checkpoint.digest() == digest)
                else {
                    return false;
                };
                ::std::sync::Arc::make_mut(&mut self.0).remove(index);
                true
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $library {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct LibraryVisitor;
                impl<'de> ::serde::de::Visitor<'de> for LibraryVisitor {
                    type Value = $library;
                    fn expecting(
                        &self,
                        formatter: &mut ::std::fmt::Formatter<'_>,
                    ) -> ::std::fmt::Result {
                        formatter.write_str(concat!(
                            "a bounded collection of imported ",
                            $label,
                            " checkpoints"
                        ))
                    }
                    fn visit_seq<A: ::serde::de::SeqAccess<'de>>(
                        self,
                        mut sequence: A,
                    ) -> Result<Self::Value, A::Error> {
                        let mut entries = Vec::new();
                        let mut seen = ::std::collections::HashSet::new();
                        let mut bytes = 0usize;
                        while let Some(entry) = sequence.next_element::<$imported>()? {
                            if entry.name.trim().is_empty()
                                || entry.name.len() > 512
                                || entry.name.chars().any(char::is_control)
                            {
                                return Err(::serde::de::Error::custom(
                                    "Invalid imported checkpoint name",
                                ));
                            }
                            bytes = bytes
                                .saturating_add(entry.name.len())
                                .saturating_add(entry.checkpoint.bytes().len());
                            if bytes > <$evidence>::byte_limit() {
                                return Err(::serde::de::Error::custom(
                                    "Imported checkpoints exceed the combined storage limit",
                                ));
                            }
                            if !seen.insert(entry.checkpoint.digest()) {
                                return Err(::serde::de::Error::custom(concat!(
                                    "Repeated imported ",
                                    $label,
                                    " checkpoint"
                                )));
                            }
                            entries.push(entry);
                        }
                        Ok($library(::std::sync::Arc::new(entries)))
                    }
                }
                deserializer.deserialize_seq(LibraryVisitor)
            }
        }
    };
}
pub(crate) use checkpoint_library;

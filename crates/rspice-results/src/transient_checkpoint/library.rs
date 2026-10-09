//! Imported continuation images are project inputs, independent of native result history.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedTransientCheckpoint {
    name: String,
    checkpoint: TransientCheckpointEvidence,
}
impl ImportedTransientCheckpoint {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn checkpoint(&self) -> &TransientCheckpointEvidence {
        &self.checkpoint
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct TransientCheckpointLibrary(Arc<Vec<ImportedTransientCheckpoint>>);

impl TransientCheckpointLibrary {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = &ImportedTransientCheckpoint> {
        self.0.iter()
    }
    pub fn get(&self, digest: ContentDigest) -> Option<&TransientCheckpointEvidence> {
        self.0
            .iter()
            .find(|entry| entry.checkpoint.digest() == digest)
            .map(|entry| &entry.checkpoint)
    }
    pub fn shares_content_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
    pub fn insert(
        &mut self,
        name: String,
        checkpoint: TransientCheckpointEvidence,
    ) -> Result<bool, String> {
        self.insert_bounded(name, checkpoint, TransientCheckpointEvidence::byte_limit())
    }
    pub fn insert_bounded(
        &mut self,
        name: String,
        checkpoint: TransientCheckpointEvidence,
        limit: usize,
    ) -> Result<bool, String> {
        if name.trim().is_empty() || name.len() > 512 || name.chars().any(char::is_control) {
            return Err(
                "Checkpoint name must contain 1–512 bytes without control characters".into(),
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
        Arc::make_mut(&mut self.0).push(ImportedTransientCheckpoint { name, checkpoint });
        Ok(true)
    }
    pub fn remove(&mut self, digest: ContentDigest) -> bool {
        let Some(index) = self
            .0
            .iter()
            .position(|entry| entry.checkpoint.digest() == digest)
        else {
            return false;
        };
        Arc::make_mut(&mut self.0).remove(index);
        true
    }
}

impl<'de> Deserialize<'de> for TransientCheckpointLibrary {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct LibraryVisitor;
        impl<'de> serde::de::Visitor<'de> for LibraryVisitor {
            type Value = TransientCheckpointLibrary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a bounded collection of imported transient checkpoints")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<Self::Value, A::Error> {
                let mut entries = Vec::new();
                let mut seen = std::collections::HashSet::new();
                let mut bytes = 0usize;
                while let Some(entry) = sequence.next_element::<ImportedTransientCheckpoint>()? {
                    if entry.name.trim().is_empty()
                        || entry.name.len() > 512
                        || entry.name.chars().any(char::is_control)
                    {
                        return Err(serde::de::Error::custom("Invalid imported checkpoint name"));
                    }
                    bytes = bytes
                        .saturating_add(entry.name.len())
                        .saturating_add(entry.checkpoint.bytes().len());
                    if bytes > TransientCheckpointEvidence::byte_limit() {
                        return Err(serde::de::Error::custom(
                            "Imported checkpoints exceed the combined storage limit",
                        ));
                    }
                    if !seen.insert(entry.checkpoint.digest()) {
                        return Err(serde::de::Error::custom(
                            "Repeated imported transient checkpoint",
                        ));
                    }
                    entries.push(entry);
                }
                Ok(TransientCheckpointLibrary(Arc::new(entries)))
            }
        }
        deserializer.deserialize_seq(LibraryVisitor)
    }
}

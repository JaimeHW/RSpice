//! Imported journals are project inputs, independent of native result history.
use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportedMonteCarloCheckpoint {
    name: String,
    checkpoint: MonteCarloCheckpointEvidence,
}
impl ImportedMonteCarloCheckpoint {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn checkpoint(&self) -> &MonteCarloCheckpointEvidence {
        &self.checkpoint
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct MonteCarloCheckpointLibrary(Arc<Vec<ImportedMonteCarloCheckpoint>>);

impl MonteCarloCheckpointLibrary {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = &ImportedMonteCarloCheckpoint> {
        self.0.iter()
    }
    pub fn get(&self, digest: ContentDigest) -> Option<&MonteCarloCheckpointEvidence> {
        self.0
            .iter()
            .find(|entry| entry.checkpoint.digest() == digest)
            .map(|entry| &entry.checkpoint)
    }
    pub(crate) fn shares_content_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
    pub(crate) fn insert(
        &mut self,
        name: String,
        checkpoint: MonteCarloCheckpointEvidence,
    ) -> Result<bool, String> {
        self.insert_bounded(
            name,
            checkpoint,
            ResourceLimits::default().max_external_data_bytes,
        )
    }
    pub(crate) fn insert_bounded(
        &mut self,
        name: String,
        checkpoint: MonteCarloCheckpointEvidence,
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
        Arc::make_mut(&mut self.0).push(ImportedMonteCarloCheckpoint { name, checkpoint });
        Ok(true)
    }
    pub(crate) fn remove(&mut self, digest: ContentDigest) -> bool {
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

impl<'de> Deserialize<'de> for MonteCarloCheckpointLibrary {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct LibraryVisitor;
        impl<'de> serde::de::Visitor<'de> for LibraryVisitor {
            type Value = MonteCarloCheckpointLibrary;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a bounded collection of imported Monte Carlo checkpoints")
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> Result<Self::Value, A::Error> {
                let mut entries = Vec::new();
                let mut seen = std::collections::HashSet::new();
                let mut bytes = 0usize;
                while let Some(entry) = sequence.next_element::<ImportedMonteCarloCheckpoint>()? {
                    if entry.name.trim().is_empty()
                        || entry.name.len() > 512
                        || entry.name.chars().any(char::is_control)
                    {
                        return Err(serde::de::Error::custom("Invalid imported checkpoint name"));
                    }
                    bytes = bytes
                        .saturating_add(entry.name.len())
                        .saturating_add(entry.checkpoint.bytes().len());
                    if bytes > ResourceLimits::default().max_external_data_bytes {
                        return Err(serde::de::Error::custom(
                            "Imported checkpoints exceed the combined storage limit",
                        ));
                    }
                    if !seen.insert(entry.checkpoint.digest()) {
                        return Err(serde::de::Error::custom(
                            "Repeated imported Monte Carlo checkpoint",
                        ));
                    }
                    entries.push(entry);
                }
                Ok(MonteCarloCheckpointLibrary(Arc::new(entries)))
            }
        }
        deserializer.deserialize_seq(LibraryVisitor)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CheckpointFile {
    format: String,
    version: u32,
    checkpoint: MonteCarloCheckpointEvidence,
}
impl MonteCarloCheckpointEvidence {
    pub(crate) fn portable_file_limit() -> usize {
        ResourceLimits::default()
            .max_external_data_bytes
            .div_ceil(3)
            .saturating_mul(4)
            .saturating_add(4096)
    }
    pub(crate) fn to_portable_file(&self) -> Result<Vec<u8>, String> {
        serde_json::to_vec(&CheckpointFile {
            format: "rspice.monte-carlo-checkpoint".into(),
            version: 1,
            checkpoint: self.clone(),
        })
        .map_err(|error| error.to_string())
    }
    pub(crate) fn from_portable_file(source: &str) -> Result<Self, String> {
        if source.len() > Self::portable_file_limit() {
            return Err("Checkpoint file exceeds its size limit".into());
        }
        let file: CheckpointFile = serde_json::from_str(source)
            .map_err(|error| format!("Invalid checkpoint file: {error}"))?;
        if file.format != "rspice.monte-carlo-checkpoint" || file.version != 1 {
            return Err("Unsupported Monte Carlo checkpoint file format or version".into());
        }
        Ok(file.checkpoint)
    }
}

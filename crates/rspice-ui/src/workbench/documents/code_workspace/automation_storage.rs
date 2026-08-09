//! Bounded, append-only storage for governed Automation output and artifacts.
//!
//! These collections are independent of page presentation so their scale,
//! memory, identity, and failure-atomicity contracts can be qualified on both
//! native and WebAssembly targets without constructing the complete workbench.

use std::{collections::HashMap, sync::Arc};

use crate::product::ContentDigest;

pub const MAX_AUTOMATION_STRUCTURED_LOG_RECORDS: usize = 5_000_000;
pub const MAX_AUTOMATION_ARTIFACT_RECORDS: usize = 100_000;
pub const MAX_AUTOMATION_ARTIFACT_PUBLISHED_BYTES: usize = 512 * 1024 * 1024;
pub const MAX_AUTOMATION_LOG_LEVEL_BYTES: usize = 128;
pub const MAX_AUTOMATION_LOG_EVENT_BYTES: usize = 1_024;
pub const MAX_AUTOMATION_LOG_DETAIL_BYTES: usize = 256 * 1_024;
pub const MAX_AUTOMATION_LOG_RETAINED_TEXT_BYTES: usize = 64 * 1_024 * 1_024;
const AUTOMATION_LOG_CHUNK_RECORDS: usize = 4_096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationStructuredOutput {
    pub sequence: u64,
    pub level: Arc<str>,
    pub event: Arc<str>,
    pub detail: Arc<str>,
}

#[derive(Debug, Clone, Default)]
pub struct AutomationStructuredOutputStore {
    chunks: Vec<Vec<AutomationStructuredOutput>>,
    interned_labels: HashMap<Arc<str>, Arc<str>>,
    len: usize,
    next_sequence: u64,
    retained_text_bytes: usize,
}

impl AutomationStructuredOutputStore {
    pub fn try_push(&mut self, level: &str, event: &str, detail: &str) -> Result<u64, String> {
        self.try_push_with_limits(
            level,
            event,
            detail,
            MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
            MAX_AUTOMATION_LOG_RETAINED_TEXT_BYTES,
        )
    }

    fn try_push_with_limits(
        &mut self,
        level: &str,
        event: &str,
        detail: &str,
        maximum_records: usize,
        maximum_retained_text_bytes: usize,
    ) -> Result<u64, String> {
        validate_collection_count(
            "Structured Automation log",
            self.len.saturating_add(1),
            maximum_records,
        )?;
        validate_automation_log_text("level", level, MAX_AUTOMATION_LOG_LEVEL_BYTES)?;
        validate_automation_log_text("event", event, MAX_AUTOMATION_LOG_EVENT_BYTES)?;
        validate_automation_log_text("detail", detail, MAX_AUTOMATION_LOG_DETAIL_BYTES)?;
        let additional_text_bytes = additional_interned_automation_text_bytes(
            &self.interned_labels,
            &[level, event, detail],
        )?;
        let retained_text_bytes = self
            .retained_text_bytes
            .checked_add(additional_text_bytes)
            .ok_or_else(|| "Structured Automation log text accounting overflowed.".to_owned())?;
        if retained_text_bytes > maximum_retained_text_bytes {
            return Err(format!(
                "Structured Automation log retains {retained_text_bytes} text bytes; the supported maximum is {maximum_retained_text_bytes}."
            ));
        }
        let sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or_else(|| "Structured Automation log sequence space is exhausted.".to_owned())?;
        let level = intern_automation_label(&mut self.interned_labels, level);
        let event = intern_automation_label(&mut self.interned_labels, event);
        let detail = intern_automation_label(&mut self.interned_labels, detail);
        if self
            .chunks
            .last()
            .is_none_or(|chunk| chunk.len() == AUTOMATION_LOG_CHUNK_RECORDS)
        {
            self.chunks
                .push(Vec::with_capacity(AUTOMATION_LOG_CHUNK_RECORDS));
        }
        let Some(chunk) = self.chunks.last_mut() else {
            return Err("Structured Automation log could not allocate a storage chunk.".to_owned());
        };
        chunk.push(AutomationStructuredOutput {
            sequence,
            level,
            event,
            detail,
        });
        self.len += 1;
        self.next_sequence = sequence;
        self.retained_text_bytes = retained_text_bytes;
        Ok(sequence)
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &AutomationStructuredOutput> {
        self.chunks.iter().flat_map(|chunk| chunk.iter())
    }
}

fn intern_automation_label(pool: &mut HashMap<Arc<str>, Arc<str>>, value: &str) -> Arc<str> {
    if let Some(existing) = pool.get(value) {
        return Arc::clone(existing);
    }
    let value: Arc<str> = Arc::from(value);
    pool.insert(Arc::clone(&value), Arc::clone(&value));
    value
}

fn validate_automation_log_text(label: &str, value: &str, maximum: usize) -> Result<(), String> {
    if value.len() > maximum {
        return Err(format!(
            "Structured Automation log {label} contains {} bytes; the supported maximum is {maximum}.",
            value.len()
        ));
    }
    Ok(())
}

fn additional_interned_automation_text_bytes(
    pool: &HashMap<Arc<str>, Arc<str>>,
    values: &[&str],
) -> Result<usize, String> {
    let mut additional = 0_usize;
    for (index, value) in values.iter().copied().enumerate() {
        if pool.contains_key(value) || values[..index].contains(&value) {
            continue;
        }
        additional = additional
            .checked_add(value.len())
            .ok_or_else(|| "Structured Automation log text accounting overflowed.".to_owned())?;
    }
    Ok(additional)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutomationArtifactRecord {
    sequence: u64,
    kind: crate::automation_workflow::ArtifactKind,
    digest: ContentDigest,
    bytes: Arc<[u8]>,
}

impl AutomationArtifactRecord {
    #[cfg(test)]
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub const fn kind(&self) -> crate::automation_workflow::ArtifactKind {
        self.kind
    }

    pub const fn file_name(&self) -> &'static str {
        self.kind.file_name()
    }

    pub const fn media_type(&self) -> &'static str {
        self.kind.media_type()
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, Default)]
pub struct AutomationArtifactStore {
    records: Vec<AutomationArtifactRecord>,
    blobs: HashMap<ContentDigest, Arc<[u8]>>,
    latest_by_kind: HashMap<crate::automation_workflow::ArtifactKind, usize>,
    published_bytes: usize,
}

impl AutomationArtifactStore {
    pub fn try_new(
        records: Vec<crate::automation_workflow::RenderedArtifact>,
    ) -> Result<Self, String> {
        validate_collection_count(
            "Automation artifact collection",
            records.len(),
            MAX_AUTOMATION_ARTIFACT_RECORDS,
        )?;
        let mut store = Self::default();
        store.records.reserve(records.len());
        for record in records {
            store.try_push(record)?;
        }
        Ok(store)
    }

    pub fn try_push(
        &mut self,
        artifact: crate::automation_workflow::RenderedArtifact,
    ) -> Result<u64, String> {
        self.try_push_with_byte_limit(artifact, MAX_AUTOMATION_ARTIFACT_PUBLISHED_BYTES)
    }

    fn try_push_with_byte_limit(
        &mut self,
        artifact: crate::automation_workflow::RenderedArtifact,
        maximum_published_bytes: usize,
    ) -> Result<u64, String> {
        validate_collection_count(
            "Automation artifact collection",
            self.records.len().saturating_add(1),
            MAX_AUTOMATION_ARTIFACT_RECORDS,
        )?;
        let published_bytes = self
            .published_bytes
            .checked_add(artifact.bytes().len())
            .ok_or_else(|| "Automation artifact byte accounting overflowed.".to_owned())?;
        if published_bytes > maximum_published_bytes {
            return Err(format!(
                "Automation artifacts publish {published_bytes} bytes; the supported maximum is {maximum_published_bytes}."
            ));
        }
        let digest = artifact.digest();
        let bytes = if let Some(existing) = self.blobs.get(&digest) {
            if existing.as_ref() != artifact.bytes() {
                return Err(format!(
                    "Automation artifact digest collision for {digest}; no record was retained."
                ));
            }
            Arc::clone(existing)
        } else {
            let bytes = artifact.shared_bytes();
            self.blobs.insert(digest, Arc::clone(&bytes));
            bytes
        };
        let sequence = u64::try_from(self.records.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or_else(|| "Automation artifact sequence space is exhausted.".to_owned())?;
        let kind = artifact.kind();
        self.records.push(AutomationArtifactRecord {
            sequence,
            kind,
            digest,
            bytes,
        });
        self.latest_by_kind.insert(kind, self.records.len() - 1);
        self.published_bytes = published_bytes;
        Ok(sequence)
    }

    pub fn get(
        &self,
        kind: crate::automation_workflow::ArtifactKind,
    ) -> Option<&AutomationArtifactRecord> {
        self.latest_by_kind
            .get(&kind)
            .and_then(|index| self.records.get(*index))
    }

    pub fn iter(&self) -> std::slice::Iter<'_, AutomationArtifactRecord> {
        self.records.iter()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn clear(&mut self) {
        self.records.clear();
        self.blobs.clear();
        self.latest_by_kind.clear();
        self.published_bytes = 0;
    }
}

fn validate_collection_count(label: &str, count: usize, maximum: usize) -> Result<(), String> {
    if count > maximum {
        return Err(format!(
            "{label} contains {count} records; the supported maximum is {maximum}."
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structured_output_store_chunks_without_discarding_early_records() {
        let mut output = AutomationStructuredOutputStore::default();
        for index in 0..=AUTOMATION_LOG_CHUNK_RECORDS {
            output
                .try_push("info", "event", &index.to_string())
                .unwrap();
        }
        assert_eq!(output.len(), AUTOMATION_LOG_CHUNK_RECORDS + 1);
        assert_eq!(output.iter().next().unwrap().sequence, 1);
        assert_eq!(
            output.iter().next_back().unwrap().sequence,
            (AUTOMATION_LOG_CHUNK_RECORDS + 1) as u64
        );
        let first = output.iter().next().unwrap();
        let last = output.iter().next_back().unwrap();
        assert!(Arc::ptr_eq(&first.level, &last.level));
        assert!(Arc::ptr_eq(&first.event, &last.event));
    }

    #[test]
    fn structured_output_store_bounds_text_and_rejects_budget_overflow_atomically() {
        let mut output = AutomationStructuredOutputStore::default();
        assert!(
            output
                .try_push(
                    &"l".repeat(MAX_AUTOMATION_LOG_LEVEL_BYTES + 1),
                    "event",
                    "detail",
                )
                .is_err()
        );
        assert!(
            output
                .try_push(
                    "info",
                    &"e".repeat(MAX_AUTOMATION_LOG_EVENT_BYTES + 1),
                    "detail",
                )
                .is_err()
        );
        assert!(
            output
                .try_push(
                    "info",
                    "event",
                    &"d".repeat(MAX_AUTOMATION_LOG_DETAIL_BYTES + 1),
                )
                .is_err()
        );
        assert_eq!(output.len, 0);
        assert_eq!(output.next_sequence, 0);
        assert_eq!(output.retained_text_bytes, 0);
        assert!(output.interned_labels.is_empty());

        output
            .try_push_with_limits("i", "first", "one", 3, 10)
            .expect("first bounded output");
        let labels_after_first = output.interned_labels.len();
        let retained_after_first = output.retained_text_bytes;
        assert!(
            output
                .try_push_with_limits("i", "second", "two", 3, 10)
                .is_err()
        );
        assert_eq!(output.len, 1);
        assert_eq!(output.next_sequence, 1);
        assert_eq!(output.retained_text_bytes, retained_after_first);
        assert_eq!(output.interned_labels.len(), labels_after_first);

        assert_eq!(
            output
                .try_push_with_limits("i", "first", "one", 3, 10)
                .expect("retry with retained text"),
            2
        );
        let first = output.iter().next().expect("first output");
        let last = output.iter().next_back().expect("last output");
        assert!(Arc::ptr_eq(&first.level, &last.level));
        assert!(Arc::ptr_eq(&first.event, &last.event));
        assert!(Arc::ptr_eq(&first.detail, &last.detail));
    }

    #[test]
    fn automation_scale_limits_accept_exact_maxima_and_reject_one_more() {
        assert!(
            validate_collection_count(
                "Structured Automation log",
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
            )
            .is_ok()
        );
        assert!(
            validate_collection_count(
                "Structured Automation log",
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS + 1,
                MAX_AUTOMATION_STRUCTURED_LOG_RECORDS,
            )
            .is_err()
        );
        assert!(
            validate_collection_count(
                "Automation artifact collection",
                MAX_AUTOMATION_ARTIFACT_RECORDS,
                MAX_AUTOMATION_ARTIFACT_RECORDS,
            )
            .is_ok()
        );
        assert!(
            validate_collection_count(
                "Automation artifact collection",
                MAX_AUTOMATION_ARTIFACT_RECORDS + 1,
                MAX_AUTOMATION_ARTIFACT_RECORDS,
            )
            .is_err()
        );
    }

    #[test]
    fn artifact_store_accepts_exact_limit_with_content_addressed_payloads() {
        let artifact = crate::automation_workflow::RenderedArtifact::new(
            crate::automation_workflow::ArtifactKind::JunitXml,
            b"shared artifact payload".to_vec(),
        );
        let mut store = AutomationArtifactStore::default();
        for _ in 0..MAX_AUTOMATION_ARTIFACT_RECORDS {
            store.try_push(artifact.clone()).unwrap();
        }

        assert_eq!(store.len(), MAX_AUTOMATION_ARTIFACT_RECORDS);
        assert_eq!(store.blobs.len(), 1);
        assert_eq!(
            store
                .get(crate::automation_workflow::ArtifactKind::JunitXml)
                .unwrap()
                .sequence(),
            MAX_AUTOMATION_ARTIFACT_RECORDS as u64
        );
        assert!(Arc::ptr_eq(
            &store.records.first().unwrap().bytes,
            &store.records.last().unwrap().bytes
        ));
        assert!(store.try_push(artifact).is_err());
    }

    #[test]
    fn artifact_byte_limit_is_failure_atomic_and_counts_every_publication() {
        let artifact = crate::automation_workflow::RenderedArtifact::new(
            crate::automation_workflow::ArtifactKind::SummaryJson,
            b"12345".to_vec(),
        );
        let mut store = AutomationArtifactStore::default();
        store
            .try_push_with_byte_limit(artifact.clone(), 10)
            .expect("first artifact fits");
        store
            .try_push_with_byte_limit(artifact.clone(), 10)
            .expect("exact byte limit fits");
        assert_eq!(store.published_bytes, 10);
        assert_eq!(store.len(), 2);
        assert_eq!(store.blobs.len(), 1);

        assert!(store.try_push_with_byte_limit(artifact, 10).is_err());
        assert_eq!(store.published_bytes, 10);
        assert_eq!(store.len(), 2);
        assert_eq!(store.blobs.len(), 1);
    }

    #[test]
    fn artifact_constructor_rejects_oversized_input_before_store_reservation() {
        let artifact = crate::automation_workflow::RenderedArtifact::new(
            crate::automation_workflow::ArtifactKind::SummaryJson,
            b"{}".to_vec(),
        );
        let oversized = vec![artifact; MAX_AUTOMATION_ARTIFACT_RECORDS + 1];
        assert!(AutomationArtifactStore::try_new(oversized).is_err());
    }
}

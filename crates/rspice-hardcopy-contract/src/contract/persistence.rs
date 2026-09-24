//! Validated, bounded project persistence for print setups and outcomes.

use super::*;
use std::collections::BTreeMap;

const MAX_RETAINED_HARDCOPY_RECEIPTS: usize = 512;

/// Bounded project-owned history of every observed hardcopy outcome,
/// including cancellation and failure. Entries are individually sealed and
/// revalidated when a project is loaded.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct HardcopyReceiptLedger {
    receipts: Vec<HardcopyReceipt>,
}

impl HardcopyReceiptLedger {
    pub fn append(&mut self, receipt: HardcopyReceipt) -> Result<(), HardcopyError> {
        receipt.validate_integrity()?;
        if self
            .receipts
            .iter()
            .any(|existing| existing.id == receipt.id)
        {
            return Err(HardcopyError::DuplicateReceiptIdentity(receipt.id));
        }
        if self.receipts.len() == MAX_RETAINED_HARDCOPY_RECEIPTS {
            self.receipts.remove(0);
        }
        self.receipts.push(receipt);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), HardcopyError> {
        if self.receipts.len() > MAX_RETAINED_HARDCOPY_RECEIPTS {
            return Err(HardcopyError::TooManyRetainedReceipts(self.receipts.len()));
        }
        let mut ids = std::collections::HashSet::with_capacity(self.receipts.len());
        for receipt in &self.receipts {
            receipt.validate_integrity()?;
            if !ids.insert(receipt.id) {
                return Err(HardcopyError::DuplicateReceiptIdentity(receipt.id));
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn receipts(&self) -> &[HardcopyReceipt] {
        &self.receipts
    }

    #[must_use]
    pub fn latest(&self) -> Option<&HardcopyReceipt> {
        self.receipts.last()
    }
}

impl<'de> Deserialize<'de> for HardcopyReceiptLedger {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(default)]
            receipts: Vec<HardcopyReceipt>,
        }
        let ledger = Self {
            receipts: Wire::deserialize(deserializer)?.receipts,
        };
        ledger.validate().map_err(serde::de::Error::custom)?;
        Ok(ledger)
    }
}

/// One validated per-document setup revision retained in project settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedHardcopySetup {
    document_id: HardcopyDocumentId,
    document_kind: HardcopyDocumentKind,
    scope: HardcopyScope,
    revision: ObjectRevision,
    setup: HardcopySetup,
    content_digest: ContentDigest,
}

impl SavedHardcopySetup {
    fn validate(&self) -> Result<(), HardcopyError> {
        self.scope.validate_for(self.document_kind)?;
        self.setup.validate()?;
        let expected = saved_setup_digest(self.document_kind, &self.scope, &self.setup)?;
        if self.content_digest != expected {
            return Err(HardcopyError::PersistedSetupDigestMismatch(
                self.document_id,
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn document_id(&self) -> HardcopyDocumentId {
        self.document_id
    }

    #[must_use]
    pub const fn document_kind(&self) -> HardcopyDocumentKind {
        self.document_kind
    }

    #[must_use]
    pub const fn scope(&self) -> &HardcopyScope {
        &self.scope
    }

    #[must_use]
    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    #[must_use]
    pub const fn setup(&self) -> &HardcopySetup {
        &self.setup
    }

    #[must_use]
    pub const fn content_digest(&self) -> ContentDigest {
        self.content_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupSaveDisposition {
    Inserted,
    Updated,
    Unchanged,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupSaveOutcome {
    disposition: SetupSaveDisposition,
    saved: SavedHardcopySetup,
}

impl SetupSaveOutcome {
    #[must_use]
    pub const fn disposition(&self) -> SetupSaveDisposition {
        self.disposition
    }

    #[must_use]
    pub const fn saved(&self) -> &SavedHardcopySetup {
        &self.saved
    }
}

/// Versioned per-document page-setup store. Deserialization is fail-closed:
/// schema, map identity, setup validation, and content digests are all checked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HardcopySetupStore {
    schema_version: u32,
    documents: BTreeMap<HardcopyDocumentId, SavedHardcopySetup>,
}

#[derive(Deserialize)]
struct HardcopySetupStoreWire {
    schema_version: u32,
    documents: BTreeMap<HardcopyDocumentId, SavedHardcopySetup>,
}

impl<'de> Deserialize<'de> for HardcopySetupStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = HardcopySetupStoreWire::deserialize(deserializer)?;
        if wire.schema_version != SETUP_STORE_SCHEMA_VERSION {
            return Err(serde::de::Error::custom(
                HardcopyError::UnsupportedSetupStoreSchema(wire.schema_version),
            ));
        }
        for (document_id, saved) in &wire.documents {
            if *document_id != saved.document_id {
                return Err(serde::de::Error::custom(
                    HardcopyError::PersistedSetupKeyMismatch {
                        key: *document_id,
                        entry: saved.document_id,
                    },
                ));
            }
            saved.validate().map_err(serde::de::Error::custom)?;
        }
        Ok(Self {
            schema_version: wire.schema_version,
            documents: wire.documents,
        })
    }
}

impl Default for HardcopySetupStore {
    fn default() -> Self {
        Self {
            schema_version: SETUP_STORE_SCHEMA_VERSION,
            documents: BTreeMap::new(),
        }
    }
}

impl HardcopySetupStore {
    pub fn save(
        &mut self,
        source: &ActiveHardcopySource,
        setup: HardcopySetup,
    ) -> Result<SetupSaveOutcome, HardcopyError> {
        source.scope.validate_for(source.document_kind)?;
        setup.validate()?;
        let digest = saved_setup_digest(source.document_kind, &source.scope, &setup)?;
        let prior = self.documents.get(&source.document_id);
        if let Some(prior) = prior {
            if prior.document_kind != source.document_kind {
                return Err(HardcopyError::PersistedDocumentKindChanged {
                    document_id: source.document_id,
                    retained: prior.document_kind,
                    observed: source.document_kind,
                });
            }
            if prior.content_digest == digest {
                return Ok(SetupSaveOutcome {
                    disposition: SetupSaveDisposition::Unchanged,
                    saved: prior.clone(),
                });
            }
        }
        let (revision, disposition) = if let Some(prior) = prior {
            (prior.revision.next()?, SetupSaveDisposition::Updated)
        } else {
            (ObjectRevision::INITIAL, SetupSaveDisposition::Inserted)
        };
        let saved = SavedHardcopySetup {
            document_id: source.document_id,
            document_kind: source.document_kind,
            scope: source.scope.clone(),
            revision,
            setup,
            content_digest: digest,
        };
        self.documents.insert(source.document_id, saved.clone());
        Ok(SetupSaveOutcome { disposition, saved })
    }

    #[must_use]
    pub fn get(&self, document_id: HardcopyDocumentId) -> Option<&SavedHardcopySetup> {
        self.documents.get(&document_id)
    }

    pub fn setup_for(
        &self,
        source: &ActiveHardcopySource,
    ) -> Result<Option<&SavedHardcopySetup>, HardcopyError> {
        let Some(saved) = self.documents.get(&source.document_id) else {
            return Ok(None);
        };
        if saved.document_kind != source.document_kind {
            return Err(HardcopyError::PersistedDocumentKindChanged {
                document_id: source.document_id,
                retained: saved.document_kind,
                observed: source.document_kind,
            });
        }
        Ok(Some(saved))
    }

    pub fn remove(&mut self, document_id: HardcopyDocumentId) -> Option<SavedHardcopySetup> {
        self.documents.remove(&document_id)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.documents.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
}

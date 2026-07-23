//! Durable routing for reusable hardcopy mapping tables.
//!
//! A mapping's declared save scope is an ownership contract, not a label. A
//! project print set is persisted with the project, while a portable personal
//! preset belongs to the user/device profile. Document-scoped mappings remain
//! embedded in the document's [`super::hardcopy::HardcopySetup`].

use std::{collections::BTreeMap, error::Error, fmt};

use serde::{Deserialize, Deserializer, Serialize};

use super::hardcopy::{PrintMappingSaveScope, PrintMappingTable};

const PRINT_MAPPING_CATALOG_SCHEMA_VERSION: u16 = 1;
const MAX_PRINT_MAPPING_PRESETS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PrintMappingCatalogOwner {
    Project,
    Personal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintMappingSaveDisposition {
    Created,
    Updated,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrintMappingSaveReceipt {
    disposition: PrintMappingSaveDisposition,
    revision: u64,
}

impl PrintMappingSaveReceipt {
    #[must_use]
    pub const fn disposition(self) -> PrintMappingSaveDisposition {
        self.disposition
    }

    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PrintMappingPresetCatalog {
    schema_version: u16,
    owner: PrintMappingCatalogOwner,
    revision: u64,
    presets: BTreeMap<String, PrintMappingTable>,
}

#[derive(Deserialize)]
struct PrintMappingPresetCatalogWire {
    schema_version: u16,
    owner: PrintMappingCatalogOwner,
    revision: u64,
    presets: BTreeMap<String, PrintMappingTable>,
}

impl<'de> Deserialize<'de> for PrintMappingPresetCatalog {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = PrintMappingPresetCatalogWire::deserialize(deserializer)?;
        let value = Self {
            schema_version: wire.schema_version,
            owner: wire.owner,
            revision: wire.revision,
            presets: wire.presets,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

impl PrintMappingPresetCatalog {
    #[must_use]
    pub const fn new(owner: PrintMappingCatalogOwner) -> Self {
        Self {
            schema_version: PRINT_MAPPING_CATALOG_SCHEMA_VERSION,
            owner,
            revision: 0,
            presets: BTreeMap::new(),
        }
    }

    #[must_use]
    pub const fn owner(&self) -> PrintMappingCatalogOwner {
        self.owner
    }

    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.presets.is_empty()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.presets.len()
    }

    #[must_use]
    pub fn get(&self, name: &str) -> Option<&PrintMappingTable> {
        self.presets.get(name)
    }

    pub fn iter(&self) -> impl ExactSizeIterator<Item = (&str, &PrintMappingTable)> {
        self.presets
            .iter()
            .map(|(name, table)| (name.as_str(), table))
    }

    pub fn save(
        &mut self,
        table: PrintMappingTable,
    ) -> Result<PrintMappingSaveReceipt, PrintMappingPersistenceError> {
        let name = self.validated_name(&table)?.to_owned();
        let disposition = match self.presets.get(&name) {
            Some(existing) if existing == &table => PrintMappingSaveDisposition::Unchanged,
            Some(_) => PrintMappingSaveDisposition::Updated,
            None => PrintMappingSaveDisposition::Created,
        };
        if disposition != PrintMappingSaveDisposition::Unchanged {
            if disposition == PrintMappingSaveDisposition::Created
                && self.presets.len() >= MAX_PRINT_MAPPING_PRESETS
            {
                return Err(PrintMappingPersistenceError::CatalogFull(
                    MAX_PRINT_MAPPING_PRESETS,
                ));
            }
            self.revision = self
                .revision
                .checked_add(1)
                .ok_or(PrintMappingPersistenceError::RevisionExhausted)?;
            self.presets.insert(name, table);
        }
        Ok(PrintMappingSaveReceipt {
            disposition,
            revision: self.revision,
        })
    }

    pub fn remove(
        &mut self,
        name: &str,
    ) -> Result<PrintMappingSaveReceipt, PrintMappingPersistenceError> {
        let disposition = if self.presets.contains_key(name) {
            let next_revision = self
                .revision
                .checked_add(1)
                .ok_or(PrintMappingPersistenceError::RevisionExhausted)?;
            self.presets.remove(name);
            self.revision = next_revision;
            PrintMappingSaveDisposition::Updated
        } else {
            PrintMappingSaveDisposition::Unchanged
        };
        Ok(PrintMappingSaveReceipt {
            disposition,
            revision: self.revision,
        })
    }

    fn validate(&self) -> Result<(), PrintMappingPersistenceError> {
        if self.schema_version != PRINT_MAPPING_CATALOG_SCHEMA_VERSION {
            return Err(PrintMappingPersistenceError::UnsupportedSchemaVersion(
                self.schema_version,
            ));
        }
        if self.presets.len() > MAX_PRINT_MAPPING_PRESETS {
            return Err(PrintMappingPersistenceError::CatalogFull(
                MAX_PRINT_MAPPING_PRESETS,
            ));
        }
        for (name, table) in &self.presets {
            let reconstructed =
                PrintMappingTable::try_new(table.save_scope().clone(), table.entries().to_vec())
                    .map_err(|error| {
                        PrintMappingPersistenceError::InvalidMapping(error.to_string())
                    })?;
            let scoped_name = self.validated_name(&reconstructed)?;
            if scoped_name != name {
                return Err(PrintMappingPersistenceError::PresetNameMismatch {
                    key: name.clone(),
                    scoped_name: scoped_name.to_owned(),
                });
            }
        }
        Ok(())
    }

    fn validated_name<'a>(
        &self,
        table: &'a PrintMappingTable,
    ) -> Result<&'a str, PrintMappingPersistenceError> {
        match (self.owner, table.save_scope()) {
            (PrintMappingCatalogOwner::Project, PrintMappingSaveScope::ProjectPrintSet(name))
            | (
                PrintMappingCatalogOwner::Personal,
                PrintMappingSaveScope::PortablePersonalPreset(name),
            ) => Ok(name),
            (_, PrintMappingSaveScope::Document) => {
                Err(PrintMappingPersistenceError::DocumentScopeIsEmbedded)
            }
            (owner, scope) => Err(PrintMappingPersistenceError::OwnershipMismatch {
                owner,
                actual_scope: scope.clone(),
            }),
        }
    }
}

impl Default for PrintMappingPresetCatalog {
    fn default() -> Self {
        Self::new(PrintMappingCatalogOwner::Project)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrintMappingPersistenceError {
    UnsupportedSchemaVersion(u16),
    CatalogFull(usize),
    RevisionExhausted,
    DocumentScopeIsEmbedded,
    OwnershipMismatch {
        owner: PrintMappingCatalogOwner,
        actual_scope: PrintMappingSaveScope,
    },
    PresetNameMismatch {
        key: String,
        scoped_name: String,
    },
    InvalidMapping(String),
}

impl fmt::Display for PrintMappingPersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion(version) => {
                write!(
                    formatter,
                    "unsupported print-mapping catalog schema {version}"
                )
            }
            Self::CatalogFull(limit) => {
                write!(
                    formatter,
                    "print-mapping catalog is limited to {limit} presets"
                )
            }
            Self::RevisionExhausted => {
                formatter.write_str("print-mapping catalog revision is exhausted")
            }
            Self::DocumentScopeIsEmbedded => formatter
                .write_str("document-scoped mappings are persisted by the document hardcopy setup"),
            Self::OwnershipMismatch {
                owner,
                actual_scope,
            } => write!(
                formatter,
                "{owner:?} catalog cannot own mapping scope {actual_scope:?}"
            ),
            Self::PresetNameMismatch { key, scoped_name } => write!(
                formatter,
                "persisted print-mapping key '{key}' differs from scoped name '{scoped_name}'"
            ),
            Self::InvalidMapping(message) => write!(formatter, "invalid print mapping: {message}"),
        }
    }
}

impl Error for PrintMappingPersistenceError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(scope: PrintMappingSaveScope) -> PrintMappingTable {
        PrintMappingTable::try_new(scope, Vec::new()).unwrap()
    }

    #[test]
    fn project_and_personal_ownership_are_enforced() {
        let mut project = PrintMappingPresetCatalog::new(PrintMappingCatalogOwner::Project);
        let receipt = project
            .save(table(PrintMappingSaveScope::ProjectPrintSet(
                "fab-review".to_owned(),
            )))
            .unwrap();
        assert_eq!(receipt.disposition(), PrintMappingSaveDisposition::Created);
        assert_eq!(project.get("fab-review").unwrap().entries().len(), 0);

        assert!(matches!(
            project.save(table(PrintMappingSaveScope::PortablePersonalPreset(
                "portable".to_owned()
            ))),
            Err(PrintMappingPersistenceError::OwnershipMismatch { .. })
        ));
        assert!(matches!(
            project.save(table(PrintMappingSaveScope::Document)),
            Err(PrintMappingPersistenceError::DocumentScopeIsEmbedded)
        ));
    }

    #[test]
    fn byte_identical_save_is_a_no_op_and_round_trips() {
        let mut personal = PrintMappingPresetCatalog::new(PrintMappingCatalogOwner::Personal);
        let mapping = table(PrintMappingSaveScope::PortablePersonalPreset(
            "lab-printer".to_owned(),
        ));
        personal.save(mapping.clone()).unwrap();
        let unchanged = personal.save(mapping).unwrap();
        assert_eq!(
            unchanged.disposition(),
            PrintMappingSaveDisposition::Unchanged
        );
        assert_eq!(unchanged.revision(), 1);

        let encoded = serde_json::to_string(&personal).unwrap();
        let restored: PrintMappingPresetCatalog = serde_json::from_str(&encoded).unwrap();
        assert_eq!(restored, personal);
    }

    #[test]
    fn wire_key_must_equal_authenticated_scope_name() {
        let source = r#"{
            "schema_version": 1,
            "owner": "project",
            "revision": 1,
            "presets": {
                "wrong": {
                    "save_scope": {"kind":"project-print-set","value":"right"},
                    "entries": []
                }
            }
        }"#;
        let error = serde_json::from_str::<PrintMappingPresetCatalog>(source).unwrap_err();
        assert!(error.to_string().contains("differs from scoped name"));
    }
}

//! Stable identities and project-scoped schematic object keys.

use super::*;

macro_rules! stable_uuid_id {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            #[must_use]
            pub const fn from_uuid(value: Uuid) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }

            #[must_use]
            pub fn is_nil(self) -> bool {
                self.0.is_nil()
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

stable_uuid_id!(SheetId);
stable_uuid_id!(CrossSheetPortId);
stable_uuid_id!(AssemblyVariantId);
stable_uuid_id!(AnnotationJournalId);
stable_uuid_id!(HierarchyAuditReceiptId);

/// Project-unique schematic-object identity. Raw object numbers are stable
/// only inside one cell/view buffer, so persisted project policy always pairs
/// them with the canonical owning cell/view key. The transparent string form
/// keeps this type usable as a deterministic JSON map key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct SchematicObjectKey(String);

impl SchematicObjectKey {
    pub fn new(cell_view_key: &str, object_id: u64) -> Result<Self, DesignManagementError> {
        require_object_id(object_id)?;
        let cell_view_key = canonical_cell_view_key(cell_view_key)?;
        Ok(Self(format!("{cell_view_key}#{object_id}")))
    }

    #[must_use]
    pub fn cell_view_key(&self) -> &str {
        self.parts()
            .map(|(cell_view_key, _)| cell_view_key)
            .expect("validated schematic object key")
    }

    #[must_use]
    pub fn object_id(&self) -> u64 {
        self.parts()
            .map(|(_, object_id)| object_id)
            .expect("validated schematic object key")
    }

    pub(super) fn validate(&self) -> Result<(), DesignManagementError> {
        let (cell_view_key, object_id) = self
            .parts()
            .ok_or_else(|| DesignManagementError::InvalidSchematicObjectKey(self.0.clone()))?;
        if canonical_cell_view_key(cell_view_key)? != cell_view_key {
            return Err(DesignManagementError::InvalidSchematicObjectKey(
                self.0.clone(),
            ));
        }
        require_object_id(object_id)
    }

    fn parts(&self) -> Option<(&str, u64)> {
        let (cell_view_key, object_id) = self.0.rsplit_once('#')?;
        let object_id = object_id.parse::<u64>().ok()?;
        Some((cell_view_key, object_id))
    }

    pub(super) fn remap_cell_owner(
        &self,
        source_library: &str,
        source_cell: &str,
        destination_library: &str,
        destination_cell: &str,
    ) -> Result<Option<Self>, DesignManagementError> {
        let [library, cell, view] = cell_view_key_segments(self.cell_view_key())?;
        if library != source_library || cell != source_cell {
            return Ok(None);
        }
        Self::new(
            &format!("{destination_library}/{destination_cell}/{view}"),
            self.object_id(),
        )
        .map(Some)
    }
}

impl fmt::Display for SchematicObjectKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl<'de> Deserialize<'de> for SchematicObjectKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Self(String::deserialize(deserializer)?);
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}

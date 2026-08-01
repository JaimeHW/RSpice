//! Authoritative physical-layout document model.
//!
//! Layout coordinates are exact signed database-unit integers. Floating-point
//! display coordinates and schematic geometry never enter this authority.
//! Every mutation is an expected-revision transaction over a validated clone,
//! so failed edits cannot partially change a persisted cell view.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest as _, Sha256};
use uuid::Uuid;

use crate::product::{ContentDigest, ObjectRevision};
use crate::quantity::LayoutDatabaseUnit;
use crate::state::CellViewRef;

pub const PHYSICAL_LAYOUT_DOCUMENT_SCHEMA_VERSION: u16 = 1;
pub const MAX_LAYOUT_OBJECTS: usize = 1_000_000;
pub const MAX_LAYOUT_NETS: usize = 1_000_000;
pub const MAX_LAYOUT_TERMINALS: usize = 100_000;
pub const MAX_LAYOUT_TRANSACTION_EDITS: usize = 100_000;
pub const MAX_LAYOUT_POLYGON_VERTICES: usize = 4_096;
pub const MAX_LAYOUT_PATH_POINTS: usize = 65_536;
pub const MAX_LAYOUT_PROPERTIES: usize = 1_024;
pub const MAX_LAYOUT_TEXT_BYTES: usize = 16_384;
pub const MAX_LAYOUT_NAME_BYTES: usize = 512;
pub const MAX_ABS_LAYOUT_COORDINATE_DBU: i64 = 1_000_000_000_000;

macro_rules! layout_uuid_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            #[must_use]
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn try_from_uuid(value: Uuid) -> Result<Self, LayoutDocumentError> {
                (!value.is_nil()).then_some(Self(value)).ok_or_else(|| {
                    LayoutDocumentError::Invalid {
                        path: stringify!($name).to_owned(),
                        message: "identity must not be the nil UUID".to_owned(),
                    }
                })
            }

            #[must_use]
            pub const fn as_uuid(self) -> Uuid {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = Uuid::deserialize(deserializer)?;
                Self::try_from_uuid(value).map_err(serde::de::Error::custom)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(formatter)
            }
        }
    };
}

layout_uuid_id!(LayoutObjectId);
layout_uuid_id!(LayoutNetId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutPoint {
    pub x: i64,
    pub y: i64,
}

impl LayoutPoint {
    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutLayerPurpose {
    pub layer: String,
    pub purpose: String,
}

impl LayoutLayerPurpose {
    pub fn try_new(
        layer: impl Into<String>,
        purpose: impl Into<String>,
    ) -> Result<Self, LayoutDocumentError> {
        let value = Self {
            layer: layer.into(),
            purpose: purpose.into(),
        };
        value.validate("layer_purpose")?;
        Ok(value)
    }

    fn validate(&self, path: &str) -> Result<(), LayoutDocumentError> {
        validate_text(&format!("{path}.layer"), &self.layer, MAX_LAYOUT_NAME_BYTES)?;
        validate_text(
            &format!("{path}.purpose"),
            &self.purpose,
            MAX_LAYOUT_NAME_BYTES,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutTechnologyBinding {
    package_id: String,
    revision: String,
    manifest_digest: ContentDigest,
    archive_digest: ContentDigest,
    process_id: String,
    stack_id: String,
    database_unit: LayoutDatabaseUnit,
}

impl LayoutTechnologyBinding {
    pub fn try_new(
        package_id: impl Into<String>,
        revision: impl Into<String>,
        manifest_digest: ContentDigest,
        archive_digest: ContentDigest,
        process_id: impl Into<String>,
        stack_id: impl Into<String>,
        database_unit: LayoutDatabaseUnit,
    ) -> Result<Self, LayoutDocumentError> {
        let binding = Self {
            package_id: package_id.into(),
            revision: revision.into(),
            manifest_digest,
            archive_digest,
            process_id: process_id.into(),
            stack_id: stack_id.into(),
            database_unit,
        };
        binding.validate()?;
        Ok(binding)
    }

    fn validate(&self) -> Result<(), LayoutDocumentError> {
        validate_text(
            "technology.package_id",
            &self.package_id,
            MAX_LAYOUT_NAME_BYTES,
        )?;
        validate_text("technology.revision", &self.revision, MAX_LAYOUT_NAME_BYTES)?;
        validate_text(
            "technology.process_id",
            &self.process_id,
            MAX_LAYOUT_NAME_BYTES,
        )?;
        validate_text("technology.stack_id", &self.stack_id, MAX_LAYOUT_NAME_BYTES)
    }

    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    pub fn revision(&self) -> &str {
        &self.revision
    }

    pub const fn manifest_digest(&self) -> ContentDigest {
        self.manifest_digest
    }

    pub const fn archive_digest(&self) -> ContentDigest {
        self.archive_digest
    }

    pub fn process_id(&self) -> &str {
        &self.process_id
    }

    pub fn stack_id(&self) -> &str {
        &self.stack_id
    }

    pub const fn database_unit(&self) -> LayoutDatabaseUnit {
        self.database_unit
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum LayoutGeometry {
    Rectangle {
        lower_left: LayoutPoint,
        upper_right: LayoutPoint,
    },
    Polygon {
        vertices: Vec<LayoutPoint>,
    },
    Path {
        centerline: Vec<LayoutPoint>,
        width_dbu: u64,
        begin_extension_dbu: u64,
        end_extension_dbu: u64,
    },
}

impl LayoutGeometry {
    fn validate(&self, path: &str) -> Result<(), LayoutDocumentError> {
        match self {
            Self::Rectangle {
                lower_left,
                upper_right,
            } => {
                validate_point(&format!("{path}.lower_left"), *lower_left)?;
                validate_point(&format!("{path}.upper_right"), *upper_right)?;
                if lower_left.x >= upper_right.x || lower_left.y >= upper_right.y {
                    return invalid(path, "rectangle must have positive width and height");
                }
            }
            Self::Polygon { vertices } => validate_polygon(path, vertices)?,
            Self::Path {
                centerline,
                width_dbu,
                begin_extension_dbu,
                end_extension_dbu,
            } => {
                if centerline.len() < 2 || centerline.len() > MAX_LAYOUT_PATH_POINTS {
                    return invalid(
                        path,
                        format!("path must contain 2..={MAX_LAYOUT_PATH_POINTS} centerline points"),
                    );
                }
                validate_unsigned_distance(&format!("{path}.width_dbu"), *width_dbu, false)?;
                validate_unsigned_distance(
                    &format!("{path}.begin_extension_dbu"),
                    *begin_extension_dbu,
                    true,
                )?;
                validate_unsigned_distance(
                    &format!("{path}.end_extension_dbu"),
                    *end_extension_dbu,
                    true,
                )?;
                for (index, point) in centerline.iter().copied().enumerate() {
                    validate_point(&format!("{path}.centerline[{index}]"), point)?;
                    if index > 0 && centerline[index - 1] == point {
                        return invalid(path, "path has adjacent duplicate points");
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutShape {
    pub layer_purpose: LayoutLayerPurpose,
    pub geometry: LayoutGeometry,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net: Option<LayoutNetId>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LayoutOrientation {
    R0,
    R90,
    R180,
    R270,
    MirrorX,
    MirrorXR90,
    MirrorY,
    MirrorYR90,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutTransform {
    pub origin: LayoutPoint,
    pub orientation: LayoutOrientation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutArray {
    pub columns: u32,
    pub rows: u32,
    pub column_step: LayoutPoint,
    pub row_step: LayoutPoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutInstance {
    pub master: CellViewRef,
    pub transform: LayoutTransform,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub array: Option<LayoutArray>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub terminal_bindings: BTreeMap<String, LayoutNetId>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutText {
    pub layer_purpose: LayoutLayerPurpose,
    pub origin: LayoutPoint,
    pub orientation: LayoutOrientation,
    pub height_dbu: u64,
    pub text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net: Option<LayoutNetId>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutNet {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutTerminal {
    pub name: String,
    pub net: LayoutNetId,
    pub shapes: BTreeSet<LayoutObjectId>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PhysicalLayoutDocument {
    schema_version: u16,
    revision: ObjectRevision,
    owner: CellViewRef,
    technology: LayoutTechnologyBinding,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    nets: BTreeMap<LayoutNetId, LayoutNet>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    shapes: BTreeMap<LayoutObjectId, LayoutShape>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    instances: BTreeMap<LayoutObjectId, LayoutInstance>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    texts: BTreeMap<LayoutObjectId, LayoutText>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    terminals: BTreeMap<LayoutObjectId, LayoutTerminal>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    properties: BTreeMap<String, String>,
}

impl PhysicalLayoutDocument {
    pub fn try_new(
        owner: CellViewRef,
        technology: LayoutTechnologyBinding,
    ) -> Result<Self, LayoutDocumentError> {
        let document = Self {
            schema_version: PHYSICAL_LAYOUT_DOCUMENT_SCHEMA_VERSION,
            revision: ObjectRevision::INITIAL,
            owner,
            technology,
            nets: BTreeMap::new(),
            shapes: BTreeMap::new(),
            instances: BTreeMap::new(),
            texts: BTreeMap::new(),
            terminals: BTreeMap::new(),
            properties: BTreeMap::new(),
        };
        document.validate()?;
        Ok(document)
    }

    pub const fn revision(&self) -> ObjectRevision {
        self.revision
    }

    pub fn owner(&self) -> &CellViewRef {
        &self.owner
    }

    pub fn technology(&self) -> &LayoutTechnologyBinding {
        &self.technology
    }

    pub fn nets(&self) -> &BTreeMap<LayoutNetId, LayoutNet> {
        &self.nets
    }

    pub fn shapes(&self) -> &BTreeMap<LayoutObjectId, LayoutShape> {
        &self.shapes
    }

    pub fn instances(&self) -> &BTreeMap<LayoutObjectId, LayoutInstance> {
        &self.instances
    }

    pub fn texts(&self) -> &BTreeMap<LayoutObjectId, LayoutText> {
        &self.texts
    }

    pub fn terminals(&self) -> &BTreeMap<LayoutObjectId, LayoutTerminal> {
        &self.terminals
    }

    pub fn properties(&self) -> &BTreeMap<String, String> {
        &self.properties
    }

    pub fn validate(&self) -> Result<(), LayoutDocumentError> {
        if self.schema_version != PHYSICAL_LAYOUT_DOCUMENT_SCHEMA_VERSION {
            return Err(LayoutDocumentError::UnsupportedSchema {
                found: self.schema_version,
                supported: PHYSICAL_LAYOUT_DOCUMENT_SCHEMA_VERSION,
            });
        }
        self.owner
            .validate_name_segments()
            .map_err(|error| LayoutDocumentError::Invalid {
                path: "owner".to_owned(),
                message: error.to_string(),
            })?;
        self.technology.validate()?;
        validate_count("nets", self.nets.len(), MAX_LAYOUT_NETS)?;
        let object_count = self
            .shapes
            .len()
            .checked_add(self.instances.len())
            .and_then(|value| value.checked_add(self.texts.len()))
            .and_then(|value| value.checked_add(self.terminals.len()))
            .ok_or_else(|| LayoutDocumentError::Invalid {
                path: "objects".to_owned(),
                message: "object count overflow".to_owned(),
            })?;
        validate_count("objects", object_count, MAX_LAYOUT_OBJECTS)?;
        validate_count("terminals", self.terminals.len(), MAX_LAYOUT_TERMINALS)?;
        validate_properties("properties", &self.properties)?;

        let mut named_nets = HashSet::new();
        for (id, net) in &self.nets {
            if let Some(name) = &net.name {
                validate_text(&format!("nets[{id}].name"), name, MAX_LAYOUT_NAME_BYTES)?;
                if !named_nets.insert(name.as_str()) {
                    return invalid(format!("nets[{id}].name"), "net name is duplicated");
                }
            }
            validate_properties(&format!("nets[{id}].properties"), &net.properties)?;
        }

        let mut object_ids = HashSet::with_capacity(object_count);
        for (id, shape) in &self.shapes {
            require_unique_object_id(&mut object_ids, *id)?;
            shape
                .layer_purpose
                .validate(&format!("shapes[{id}].layer_purpose"))?;
            shape.geometry.validate(&format!("shapes[{id}].geometry"))?;
            if let Some(net) = shape.net {
                require_net(&self.nets, net, &format!("shapes[{id}].net"))?;
            }
            validate_properties(&format!("shapes[{id}].properties"), &shape.properties)?;
        }
        for (id, instance) in &self.instances {
            require_unique_object_id(&mut object_ids, *id)?;
            instance.master.validate_name_segments().map_err(|error| {
                LayoutDocumentError::Invalid {
                    path: format!("instances[{id}].master"),
                    message: error.to_string(),
                }
            })?;
            if instance.master == self.owner {
                return invalid(
                    format!("instances[{id}].master"),
                    "a layout view cannot directly instantiate itself",
                );
            }
            validate_point(
                &format!("instances[{id}].transform.origin"),
                instance.transform.origin,
            )?;
            if let Some(array) = &instance.array {
                validate_array(&format!("instances[{id}].array"), array)?;
            }
            for (terminal, net) in &instance.terminal_bindings {
                validate_text(
                    &format!("instances[{id}].terminal_bindings key"),
                    terminal,
                    MAX_LAYOUT_NAME_BYTES,
                )?;
                require_net(
                    &self.nets,
                    *net,
                    &format!("instances[{id}].terminal_bindings[{terminal}]"),
                )?;
            }
            validate_properties(&format!("instances[{id}].properties"), &instance.properties)?;
        }
        for (id, text) in &self.texts {
            require_unique_object_id(&mut object_ids, *id)?;
            text.layer_purpose
                .validate(&format!("texts[{id}].layer_purpose"))?;
            validate_point(&format!("texts[{id}].origin"), text.origin)?;
            validate_unsigned_distance(&format!("texts[{id}].height_dbu"), text.height_dbu, false)?;
            validate_text(
                &format!("texts[{id}].text"),
                &text.text,
                MAX_LAYOUT_TEXT_BYTES,
            )?;
            if let Some(net) = text.net {
                require_net(&self.nets, net, &format!("texts[{id}].net"))?;
            }
            validate_properties(&format!("texts[{id}].properties"), &text.properties)?;
        }

        let mut terminal_names = HashSet::new();
        for (id, terminal) in &self.terminals {
            require_unique_object_id(&mut object_ids, *id)?;
            validate_text(
                &format!("terminals[{id}].name"),
                &terminal.name,
                MAX_LAYOUT_NAME_BYTES,
            )?;
            if !terminal_names.insert(terminal.name.as_str()) {
                return invalid(
                    format!("terminals[{id}].name"),
                    "terminal name is duplicated",
                );
            }
            require_net(&self.nets, terminal.net, &format!("terminals[{id}].net"))?;
            if terminal.shapes.is_empty() {
                return invalid(format!("terminals[{id}].shapes"), "terminal has no shapes");
            }
            for shape_id in &terminal.shapes {
                let shape =
                    self.shapes
                        .get(shape_id)
                        .ok_or_else(|| LayoutDocumentError::Invalid {
                            path: format!("terminals[{id}].shapes[{shape_id}]"),
                            message: "terminal references a missing shape".to_owned(),
                        })?;
                if shape.net != Some(terminal.net) {
                    return invalid(
                        format!("terminals[{id}].shapes[{shape_id}]"),
                        "terminal shape is not bound to the terminal net",
                    );
                }
            }
            validate_properties(&format!("terminals[{id}].properties"), &terminal.properties)?;
        }
        Ok(())
    }

    pub fn apply_transaction(
        &mut self,
        expected_revision: ObjectRevision,
        edits: &[LayoutEdit],
    ) -> Result<ObjectRevision, LayoutDocumentError> {
        if self.revision != expected_revision {
            return Err(LayoutDocumentError::RevisionConflict {
                expected: expected_revision,
                actual: self.revision,
            });
        }
        if edits.is_empty() {
            return Err(LayoutDocumentError::EmptyTransaction);
        }
        if edits.len() > MAX_LAYOUT_TRANSACTION_EDITS {
            return Err(LayoutDocumentError::LimitExceeded {
                path: "transaction.edits".to_owned(),
                count: edits.len(),
                maximum: MAX_LAYOUT_TRANSACTION_EDITS,
            });
        }
        self.validate()?;
        let mut candidate = self.clone();
        for edit in edits {
            candidate.apply_edit(edit)?;
        }
        candidate.revision =
            candidate
                .revision
                .next()
                .map_err(|error| LayoutDocumentError::Invalid {
                    path: "revision".to_owned(),
                    message: error.to_string(),
                })?;
        candidate.validate()?;
        let revision = candidate.revision;
        *self = candidate;
        Ok(revision)
    }

    pub fn content_digest(&self) -> Result<ContentDigest, LayoutDocumentError> {
        self.validate()?;
        let encoded = serde_json::to_vec(self).map_err(|error| LayoutDocumentError::Invalid {
            path: "document".to_owned(),
            message: format!("cannot serialize canonical layout document: {error}"),
        })?;
        let mut hasher = Sha256::new();
        hasher.update(b"rspice-physical-layout-document\0v1\0");
        hasher.update((encoded.len() as u64).to_be_bytes());
        hasher.update(encoded);
        Ok(ContentDigest::from_bytes(hasher.finalize().into()))
    }

    pub(crate) fn copy_for_cell(
        &self,
        source_library: &str,
        source_cell: &str,
        target_library: &str,
        target_cell: &str,
    ) -> Result<Self, LayoutDocumentError> {
        self.validate()?;
        if self.owner.library != source_library || self.owner.cell != source_cell {
            return invalid(
                "owner",
                "layout document is not owned by the requested source cell",
            );
        }
        let mut candidate = self.clone();
        candidate.owner.library = target_library.to_owned();
        candidate.owner.cell = target_cell.to_owned();
        candidate.revision = ObjectRevision::INITIAL;
        for instance in candidate.instances.values_mut() {
            if instance.master.library == source_library && instance.master.cell == source_cell {
                instance.master.library = target_library.to_owned();
                instance.master.cell = target_cell.to_owned();
            }
        }
        candidate.validate()?;
        Ok(candidate)
    }

    pub(crate) fn rename_cell_references(
        &self,
        library: &str,
        source_cell: &str,
        target_cell: &str,
    ) -> Result<Self, LayoutDocumentError> {
        self.validate()?;
        let mut candidate = self.clone();
        let mut changed = false;
        if candidate.owner.library == library && candidate.owner.cell == source_cell {
            candidate.owner.cell = target_cell.to_owned();
            changed = true;
        }
        for instance in candidate.instances.values_mut() {
            if instance.master.library == library && instance.master.cell == source_cell {
                instance.master.cell = target_cell.to_owned();
                changed = true;
            }
        }
        if changed {
            candidate.revision =
                candidate
                    .revision
                    .next()
                    .map_err(|error| LayoutDocumentError::Invalid {
                        path: "revision".to_owned(),
                        message: error.to_string(),
                    })?;
        }
        candidate.validate()?;
        Ok(candidate)
    }

    fn apply_edit(&mut self, edit: &LayoutEdit) -> Result<(), LayoutDocumentError> {
        match edit {
            LayoutEdit::InsertNet { id, value } => {
                insert(&mut self.nets, *id, value.clone(), "net")
            }
            LayoutEdit::ReplaceNet { id, value } => {
                replace(&mut self.nets, *id, value.clone(), "net")
            }
            LayoutEdit::RemoveNet { id } => remove(&mut self.nets, *id, "net"),
            LayoutEdit::InsertShape { id, value } => {
                insert(&mut self.shapes, *id, value.clone(), "shape")
            }
            LayoutEdit::ReplaceShape { id, value } => {
                replace(&mut self.shapes, *id, value.clone(), "shape")
            }
            LayoutEdit::RemoveShape { id } => remove(&mut self.shapes, *id, "shape"),
            LayoutEdit::InsertInstance { id, value } => {
                insert(&mut self.instances, *id, value.clone(), "instance")
            }
            LayoutEdit::ReplaceInstance { id, value } => {
                replace(&mut self.instances, *id, value.clone(), "instance")
            }
            LayoutEdit::RemoveInstance { id } => remove(&mut self.instances, *id, "instance"),
            LayoutEdit::InsertText { id, value } => {
                insert(&mut self.texts, *id, value.clone(), "text")
            }
            LayoutEdit::ReplaceText { id, value } => {
                replace(&mut self.texts, *id, value.clone(), "text")
            }
            LayoutEdit::RemoveText { id } => remove(&mut self.texts, *id, "text"),
            LayoutEdit::InsertTerminal { id, value } => {
                insert(&mut self.terminals, *id, value.clone(), "terminal")
            }
            LayoutEdit::ReplaceTerminal { id, value } => {
                replace(&mut self.terminals, *id, value.clone(), "terminal")
            }
            LayoutEdit::RemoveTerminal { id } => remove(&mut self.terminals, *id, "terminal"),
            LayoutEdit::SetProperty { key, value } => {
                validate_text("property.key", key, MAX_LAYOUT_NAME_BYTES)?;
                validate_text("property.value", value, MAX_LAYOUT_TEXT_BYTES)?;
                self.properties.insert(key.clone(), value.clone());
                Ok(())
            }
            LayoutEdit::RemoveProperty { key } => {
                self.properties.remove(key).map(|_| ()).ok_or_else(|| {
                    LayoutDocumentError::MissingObject {
                        kind: "property",
                        id: key.clone(),
                    }
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutEdit {
    InsertNet {
        id: LayoutNetId,
        value: LayoutNet,
    },
    ReplaceNet {
        id: LayoutNetId,
        value: LayoutNet,
    },
    RemoveNet {
        id: LayoutNetId,
    },
    InsertShape {
        id: LayoutObjectId,
        value: LayoutShape,
    },
    ReplaceShape {
        id: LayoutObjectId,
        value: LayoutShape,
    },
    RemoveShape {
        id: LayoutObjectId,
    },
    InsertInstance {
        id: LayoutObjectId,
        value: LayoutInstance,
    },
    ReplaceInstance {
        id: LayoutObjectId,
        value: LayoutInstance,
    },
    RemoveInstance {
        id: LayoutObjectId,
    },
    InsertText {
        id: LayoutObjectId,
        value: LayoutText,
    },
    ReplaceText {
        id: LayoutObjectId,
        value: LayoutText,
    },
    RemoveText {
        id: LayoutObjectId,
    },
    InsertTerminal {
        id: LayoutObjectId,
        value: LayoutTerminal,
    },
    ReplaceTerminal {
        id: LayoutObjectId,
        value: LayoutTerminal,
    },
    RemoveTerminal {
        id: LayoutObjectId,
    },
    SetProperty {
        key: String,
        value: String,
    },
    RemoveProperty {
        key: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum LayoutDocumentError {
    #[error("unsupported physical-layout schema {found}; supported schema is {supported}")]
    UnsupportedSchema { found: u16, supported: u16 },
    #[error("{path}: {message}")]
    Invalid { path: String, message: String },
    #[error("{path} contains {count} entries; maximum is {maximum}")]
    LimitExceeded {
        path: String,
        count: usize,
        maximum: usize,
    },
    #[error("layout revision conflict: expected {expected:?}, current revision is {actual:?}")]
    RevisionConflict {
        expected: ObjectRevision,
        actual: ObjectRevision,
    },
    #[error("layout transaction must contain at least one edit")]
    EmptyTransaction,
    #[error("{kind} {id} already exists")]
    DuplicateObject { kind: &'static str, id: String },
    #[error("{kind} {id} does not exist")]
    MissingObject { kind: &'static str, id: String },
}

fn insert<K, V>(
    map: &mut BTreeMap<K, V>,
    id: K,
    value: V,
    kind: &'static str,
) -> Result<(), LayoutDocumentError>
where
    K: Ord + Copy + fmt::Display,
{
    if map.contains_key(&id) {
        return Err(LayoutDocumentError::DuplicateObject {
            kind,
            id: id.to_string(),
        });
    }
    map.insert(id, value);
    Ok(())
}

fn replace<K, V>(
    map: &mut BTreeMap<K, V>,
    id: K,
    value: V,
    kind: &'static str,
) -> Result<(), LayoutDocumentError>
where
    K: Ord + Copy + fmt::Display,
{
    let slot = map
        .get_mut(&id)
        .ok_or_else(|| LayoutDocumentError::MissingObject {
            kind,
            id: id.to_string(),
        })?;
    *slot = value;
    Ok(())
}

fn remove<K, V>(
    map: &mut BTreeMap<K, V>,
    id: K,
    kind: &'static str,
) -> Result<(), LayoutDocumentError>
where
    K: Ord + Copy + fmt::Display,
{
    map.remove(&id)
        .map(|_| ())
        .ok_or_else(|| LayoutDocumentError::MissingObject {
            kind,
            id: id.to_string(),
        })
}

fn validate_count(path: &str, count: usize, maximum: usize) -> Result<(), LayoutDocumentError> {
    if count > maximum {
        return Err(LayoutDocumentError::LimitExceeded {
            path: path.to_owned(),
            count,
            maximum,
        });
    }
    Ok(())
}

fn validate_text(path: &str, value: &str, maximum: usize) -> Result<(), LayoutDocumentError> {
    if value.is_empty() || value != value.trim() {
        return invalid(path, "text must be nonempty and trimmed");
    }
    if value.len() > maximum {
        return invalid(path, format!("text exceeds {maximum} bytes"));
    }
    if value.chars().any(char::is_control) {
        return invalid(path, "text contains a control character");
    }
    Ok(())
}

fn validate_properties(
    path: &str,
    properties: &BTreeMap<String, String>,
) -> Result<(), LayoutDocumentError> {
    validate_count(path, properties.len(), MAX_LAYOUT_PROPERTIES)?;
    for (key, value) in properties {
        validate_text(&format!("{path}.{key}.key"), key, MAX_LAYOUT_NAME_BYTES)?;
        validate_text(&format!("{path}.{key}.value"), value, MAX_LAYOUT_TEXT_BYTES)?;
    }
    Ok(())
}

fn validate_point(path: &str, point: LayoutPoint) -> Result<(), LayoutDocumentError> {
    for (axis, value) in [("x", point.x), ("y", point.y)] {
        if !(-MAX_ABS_LAYOUT_COORDINATE_DBU..=MAX_ABS_LAYOUT_COORDINATE_DBU).contains(&value) {
            return invalid(
                format!("{path}.{axis}"),
                format!(
                    "coordinate must be within +/-{MAX_ABS_LAYOUT_COORDINATE_DBU} database units"
                ),
            );
        }
    }
    Ok(())
}

fn validate_unsigned_distance(
    path: &str,
    value: u64,
    allow_zero: bool,
) -> Result<(), LayoutDocumentError> {
    if !allow_zero && value == 0 {
        return invalid(path, "distance must be greater than zero");
    }
    if value > MAX_ABS_LAYOUT_COORDINATE_DBU as u64 {
        return invalid(
            path,
            format!("distance exceeds {MAX_ABS_LAYOUT_COORDINATE_DBU} database units"),
        );
    }
    Ok(())
}

fn validate_array(path: &str, array: &LayoutArray) -> Result<(), LayoutDocumentError> {
    if array.columns == 0 || array.rows == 0 {
        return invalid(path, "array rows and columns must be greater than zero");
    }
    let members = u64::from(array.columns) * u64::from(array.rows);
    if members > MAX_LAYOUT_OBJECTS as u64 {
        return invalid(path, format!("array expands to {members} members"));
    }
    validate_point(&format!("{path}.column_step"), array.column_step)?;
    validate_point(&format!("{path}.row_step"), array.row_step)?;
    if array.columns > 1 && array.column_step == LayoutPoint::new(0, 0) {
        return invalid(path, "multi-column array has a zero column step");
    }
    if array.rows > 1 && array.row_step == LayoutPoint::new(0, 0) {
        return invalid(path, "multi-row array has a zero row step");
    }
    Ok(())
}

fn validate_polygon(path: &str, vertices: &[LayoutPoint]) -> Result<(), LayoutDocumentError> {
    if vertices.len() < 3 || vertices.len() > MAX_LAYOUT_POLYGON_VERTICES {
        return invalid(
            path,
            format!("polygon must contain 3..={MAX_LAYOUT_POLYGON_VERTICES} vertices"),
        );
    }
    for (index, point) in vertices.iter().copied().enumerate() {
        validate_point(&format!("{path}.vertices[{index}]"), point)?;
        if index > 0 && vertices[index - 1] == point {
            return invalid(path, "polygon has adjacent duplicate vertices");
        }
    }
    if vertices.first() == vertices.last() {
        return invalid(
            path,
            "polygon closure is implicit; first and last vertices must differ",
        );
    }
    let mut twice_area = 0_i128;
    for index in 0..vertices.len() {
        let left = vertices[index];
        let right = vertices[(index + 1) % vertices.len()];
        let cross =
            i128::from(left.x) * i128::from(right.y) - i128::from(right.x) * i128::from(left.y);
        twice_area = twice_area
            .checked_add(cross)
            .ok_or_else(|| LayoutDocumentError::Invalid {
                path: path.to_owned(),
                message: "polygon area arithmetic overflow".to_owned(),
            })?;
    }
    if twice_area == 0 {
        return invalid(path, "polygon has zero signed area");
    }
    for left_index in 0..vertices.len() {
        let left_next = (left_index + 1) % vertices.len();
        for right_index in (left_index + 1)..vertices.len() {
            let right_next = (right_index + 1) % vertices.len();
            if left_index == right_index || left_next == right_index || right_next == left_index {
                continue;
            }
            if segments_intersect(
                vertices[left_index],
                vertices[left_next],
                vertices[right_index],
                vertices[right_next],
            ) {
                return invalid(path, "polygon has self-intersecting non-adjacent edges");
            }
        }
    }
    Ok(())
}

fn segments_intersect(a: LayoutPoint, b: LayoutPoint, c: LayoutPoint, d: LayoutPoint) -> bool {
    let ab_c = orientation(a, b, c);
    let ab_d = orientation(a, b, d);
    let cd_a = orientation(c, d, a);
    let cd_b = orientation(c, d, b);
    if ab_c == 0 && point_on_segment(a, b, c) {
        return true;
    }
    if ab_d == 0 && point_on_segment(a, b, d) {
        return true;
    }
    if cd_a == 0 && point_on_segment(c, d, a) {
        return true;
    }
    if cd_b == 0 && point_on_segment(c, d, b) {
        return true;
    }
    (ab_c > 0) != (ab_d > 0) && (cd_a > 0) != (cd_b > 0)
}

fn orientation(a: LayoutPoint, b: LayoutPoint, c: LayoutPoint) -> i128 {
    (i128::from(b.x) - i128::from(a.x)) * (i128::from(c.y) - i128::from(a.y))
        - (i128::from(b.y) - i128::from(a.y)) * (i128::from(c.x) - i128::from(a.x))
}

fn point_on_segment(a: LayoutPoint, b: LayoutPoint, point: LayoutPoint) -> bool {
    point.x >= a.x.min(b.x)
        && point.x <= a.x.max(b.x)
        && point.y >= a.y.min(b.y)
        && point.y <= a.y.max(b.y)
}

fn require_unique_object_id(
    ids: &mut HashSet<LayoutObjectId>,
    id: LayoutObjectId,
) -> Result<(), LayoutDocumentError> {
    if !ids.insert(id) {
        return Err(LayoutDocumentError::DuplicateObject {
            kind: "layout object identity",
            id: id.to_string(),
        });
    }
    Ok(())
}

fn require_net(
    nets: &BTreeMap<LayoutNetId, LayoutNet>,
    id: LayoutNetId,
    path: &str,
) -> Result<(), LayoutDocumentError> {
    if !nets.contains_key(&id) {
        return invalid(path, format!("references missing net {id}"));
    }
    Ok(())
}

fn invalid<T>(
    path: impl Into<String>,
    message: impl Into<String>,
) -> Result<T, LayoutDocumentError> {
    Err(LayoutDocumentError::Invalid {
        path: path.into(),
        message: message.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn technology() -> LayoutTechnologyBinding {
        LayoutTechnologyBinding::try_new(
            "org.rspice.test-pdk",
            "1.0.0",
            ContentDigest::from_bytes([0x11; 32]),
            ContentDigest::from_bytes([0x22; 32]),
            "tt",
            "six-metal",
            LayoutDatabaseUnit::from_metres(1.0e-9).unwrap(),
        )
        .unwrap()
    }

    fn document() -> PhysicalLayoutDocument {
        PhysicalLayoutDocument::try_new(
            CellViewRef::new("user", "inverter", "layout"),
            technology(),
        )
        .unwrap()
    }

    #[test]
    fn atomic_transaction_builds_exact_net_shape_and_terminal_authority() {
        let mut document = document();
        let net = LayoutNetId::new();
        let shape = LayoutObjectId::new();
        let terminal = LayoutObjectId::new();
        let start = document.revision();
        let revision = document
            .apply_transaction(
                start,
                &[
                    LayoutEdit::InsertNet {
                        id: net,
                        value: LayoutNet {
                            name: Some("VDD".to_owned()),
                            properties: BTreeMap::new(),
                        },
                    },
                    LayoutEdit::InsertShape {
                        id: shape,
                        value: LayoutShape {
                            layer_purpose: LayoutLayerPurpose::try_new("M1", "drawing").unwrap(),
                            geometry: LayoutGeometry::Rectangle {
                                lower_left: LayoutPoint::new(0, 0),
                                upper_right: LayoutPoint::new(200, 80),
                            },
                            net: Some(net),
                            properties: BTreeMap::new(),
                        },
                    },
                    LayoutEdit::InsertTerminal {
                        id: terminal,
                        value: LayoutTerminal {
                            name: "VDD".to_owned(),
                            net,
                            shapes: BTreeSet::from([shape]),
                            properties: BTreeMap::new(),
                        },
                    },
                ],
            )
            .unwrap();

        assert_eq!(revision, start.next().unwrap());
        assert_eq!(document.nets().len(), 1);
        assert_eq!(document.shapes().len(), 1);
        assert_eq!(document.terminals().len(), 1);
        document.validate().unwrap();
        assert_eq!(
            document.content_digest().unwrap(),
            document.content_digest().unwrap()
        );
    }

    #[test]
    fn failed_transaction_leaves_revision_and_content_unchanged() {
        let mut document = document();
        let revision = document.revision();
        let digest = document.content_digest().unwrap();
        let missing_net = LayoutNetId::new();
        let result = document.apply_transaction(
            revision,
            &[LayoutEdit::InsertShape {
                id: LayoutObjectId::new(),
                value: LayoutShape {
                    layer_purpose: LayoutLayerPurpose::try_new("M1", "drawing").unwrap(),
                    geometry: LayoutGeometry::Rectangle {
                        lower_left: LayoutPoint::new(0, 0),
                        upper_right: LayoutPoint::new(10, 10),
                    },
                    net: Some(missing_net),
                    properties: BTreeMap::new(),
                },
            }],
        );

        assert!(result.is_err());
        assert_eq!(document.revision(), revision);
        assert_eq!(document.content_digest().unwrap(), digest);
        assert!(document.shapes().is_empty());
    }

    #[test]
    fn stale_revision_and_cross_kind_duplicate_identity_fail_closed() {
        let mut document = document();
        let object = LayoutObjectId::new();
        let revision = document.revision();
        document
            .apply_transaction(
                revision,
                &[LayoutEdit::InsertText {
                    id: object,
                    value: LayoutText {
                        layer_purpose: LayoutLayerPurpose::try_new("TEXT", "drawing").unwrap(),
                        origin: LayoutPoint::new(0, 0),
                        orientation: LayoutOrientation::R0,
                        height_dbu: 10,
                        text: "inverter".to_owned(),
                        net: None,
                        properties: BTreeMap::new(),
                    },
                }],
            )
            .unwrap();
        assert!(matches!(
            document.apply_transaction(
                revision,
                &[LayoutEdit::SetProperty {
                    key: "owner".to_owned(),
                    value: "layout".to_owned(),
                }]
            ),
            Err(LayoutDocumentError::RevisionConflict { .. })
        ));

        let current = document.revision();
        let result = document.apply_transaction(
            current,
            &[LayoutEdit::InsertShape {
                id: object,
                value: LayoutShape {
                    layer_purpose: LayoutLayerPurpose::try_new("M1", "drawing").unwrap(),
                    geometry: LayoutGeometry::Rectangle {
                        lower_left: LayoutPoint::new(0, 0),
                        upper_right: LayoutPoint::new(10, 10),
                    },
                    net: None,
                    properties: BTreeMap::new(),
                },
            }],
        );
        assert!(matches!(
            result,
            Err(LayoutDocumentError::DuplicateObject { .. })
        ));
        assert_eq!(document.revision(), current);
    }

    #[test]
    fn invalid_and_self_intersecting_geometry_is_rejected_without_commit() {
        let mut document = document();
        let revision = document.revision();
        let bow_tie = LayoutShape {
            layer_purpose: LayoutLayerPurpose::try_new("M1", "drawing").unwrap(),
            geometry: LayoutGeometry::Polygon {
                vertices: vec![
                    LayoutPoint::new(0, 0),
                    LayoutPoint::new(10, 10),
                    LayoutPoint::new(0, 10),
                    LayoutPoint::new(10, 0),
                ],
            },
            net: None,
            properties: BTreeMap::new(),
        };
        assert!(
            document
                .apply_transaction(
                    revision,
                    &[LayoutEdit::InsertShape {
                        id: LayoutObjectId::new(),
                        value: bow_tie,
                    }]
                )
                .is_err()
        );
        assert_eq!(document.revision(), revision);
        assert!(document.shapes().is_empty());
    }

    #[test]
    fn tampered_serialized_terminal_reference_is_rejected() {
        let mut document = document();
        let net = LayoutNetId::new();
        let shape = LayoutObjectId::new();
        document
            .apply_transaction(
                document.revision(),
                &[
                    LayoutEdit::InsertNet {
                        id: net,
                        value: LayoutNet {
                            name: Some("OUT".to_owned()),
                            properties: BTreeMap::new(),
                        },
                    },
                    LayoutEdit::InsertShape {
                        id: shape,
                        value: LayoutShape {
                            layer_purpose: LayoutLayerPurpose::try_new("M1", "drawing").unwrap(),
                            geometry: LayoutGeometry::Rectangle {
                                lower_left: LayoutPoint::new(0, 0),
                                upper_right: LayoutPoint::new(10, 10),
                            },
                            net: Some(net),
                            properties: BTreeMap::new(),
                        },
                    },
                ],
            )
            .unwrap();
        let mut value = serde_json::to_value(&document).unwrap();
        let mut terminals = serde_json::Map::new();
        terminals.insert(
            LayoutObjectId::new().to_string(),
            serde_json::json!({
                "name": "OUT",
                "net": net,
                "shapes": [LayoutObjectId::new()],
            }),
        );
        value["terminals"] = serde_json::Value::Object(terminals);
        let tampered: PhysicalLayoutDocument = serde_json::from_value(value).unwrap();
        assert!(tampered.validate().is_err());
    }

    #[test]
    fn canonical_digest_is_independent_of_transaction_edit_order() {
        let net = LayoutNetId::new();
        let shape = LayoutObjectId::new();
        let net_edit = LayoutEdit::InsertNet {
            id: net,
            value: LayoutNet {
                name: Some("OUT".to_owned()),
                properties: BTreeMap::new(),
            },
        };
        let shape_edit = LayoutEdit::InsertShape {
            id: shape,
            value: LayoutShape {
                layer_purpose: LayoutLayerPurpose::try_new("M1", "drawing").unwrap(),
                geometry: LayoutGeometry::Rectangle {
                    lower_left: LayoutPoint::new(0, 0),
                    upper_right: LayoutPoint::new(10, 10),
                },
                net: Some(net),
                properties: BTreeMap::new(),
            },
        };
        let mut first = document();
        first
            .apply_transaction(first.revision(), &[net_edit.clone(), shape_edit.clone()])
            .unwrap();
        let mut second = document();
        second
            .apply_transaction(second.revision(), &[shape_edit, net_edit])
            .unwrap();

        assert_eq!(first, second);
        assert_eq!(
            first.content_digest().unwrap(),
            second.content_digest().unwrap()
        );
    }

    #[test]
    fn removing_a_referenced_net_is_rejected_atomically() {
        let mut document = document();
        let net = LayoutNetId::new();
        document
            .apply_transaction(
                document.revision(),
                &[
                    LayoutEdit::InsertNet {
                        id: net,
                        value: LayoutNet {
                            name: Some("VSS".to_owned()),
                            properties: BTreeMap::new(),
                        },
                    },
                    LayoutEdit::InsertShape {
                        id: LayoutObjectId::new(),
                        value: LayoutShape {
                            layer_purpose: LayoutLayerPurpose::try_new("M1", "drawing").unwrap(),
                            geometry: LayoutGeometry::Rectangle {
                                lower_left: LayoutPoint::new(0, 0),
                                upper_right: LayoutPoint::new(10, 10),
                            },
                            net: Some(net),
                            properties: BTreeMap::new(),
                        },
                    },
                ],
            )
            .unwrap();
        let revision = document.revision();
        let digest = document.content_digest().unwrap();

        assert!(
            document
                .apply_transaction(revision, &[LayoutEdit::RemoveNet { id: net }])
                .is_err()
        );
        assert_eq!(document.revision(), revision);
        assert_eq!(document.content_digest().unwrap(), digest);
    }

    #[test]
    fn cell_copy_and_rename_remap_layout_owner_and_hierarchical_masters() {
        let mut source = document();
        source
            .apply_transaction(
                source.revision(),
                &[LayoutEdit::InsertInstance {
                    id: LayoutObjectId::new(),
                    value: LayoutInstance {
                        master: CellViewRef::new("user", "inverter", "layout_alt"),
                        transform: LayoutTransform {
                            origin: LayoutPoint::new(20, 30),
                            orientation: LayoutOrientation::R90,
                        },
                        array: None,
                        terminal_bindings: BTreeMap::new(),
                        properties: BTreeMap::new(),
                    },
                }],
            )
            .unwrap();
        let copied = source
            .copy_for_cell("user", "inverter", "work", "inverter_copy")
            .unwrap();
        assert_eq!(
            copied.owner(),
            &CellViewRef::new("work", "inverter_copy", "layout")
        );
        assert_eq!(copied.revision(), ObjectRevision::INITIAL);
        let copied_master = &copied.instances().values().next().unwrap().master;
        assert_eq!(
            copied_master,
            &CellViewRef::new("work", "inverter_copy", "layout_alt")
        );

        let renamed = copied
            .rename_cell_references("work", "inverter_copy", "inverter_final")
            .unwrap();
        assert_eq!(
            renamed.owner(),
            &CellViewRef::new("work", "inverter_final", "layout")
        );
        assert_eq!(renamed.revision(), ObjectRevision::new(2).unwrap());
        assert_eq!(
            renamed.instances().values().next().unwrap().master,
            CellViewRef::new("work", "inverter_final", "layout_alt")
        );
    }
}

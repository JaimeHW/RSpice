//! What the schematic bulk editor's queries select, and the named filters the
//! reader keeps with the session. Device-local, never project data.

use serde::{Deserialize, Serialize};

/// Object family searched by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkObjectKind {
    #[default]
    InstancesAndParameters,
    NetsAndLabels,
    PortsAndPins,
    GraphicsAndNotes,
}

impl SelectionBulkObjectKind {
    pub const ALL: [Self; 4] = [
        Self::InstancesAndParameters,
        Self::NetsAndLabels,
        Self::PortsAndPins,
        Self::GraphicsAndNotes,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::InstancesAndParameters => "Instances + parameters",
            Self::NetsAndLabels => "Nets and labels",
            Self::PortsAndPins => "Ports and pins",
            Self::GraphicsAndNotes => "Graphics and notes",
        }
    }
}

/// Project extent inspected by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkHierarchyScope {
    #[default]
    ActiveHierarchyPath,
    CurrentSheet,
    CompleteProject,
}

impl SelectionBulkHierarchyScope {
    pub const ALL: [Self; 3] = [
        Self::ActiveHierarchyPath,
        Self::CurrentSheet,
        Self::CompleteProject,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ActiveHierarchyPath => "Active hierarchy path",
            Self::CurrentSheet => "Current sheet",
            Self::CompleteProject => "Complete project",
        }
    }
}

/// Device-local, reusable query used by the mockup-owned bulk editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SelectionBulkFilter {
    pub query: String,
    pub object_kind: SelectionBulkObjectKind,
    pub hierarchy_scope: SelectionBulkHierarchyScope,
    pub model_cell: String,
    pub current_property: String,
}

impl Default for SelectionBulkFilter {
    fn default() -> Self {
        Self {
            query: String::new(),
            object_kind: SelectionBulkObjectKind::InstancesAndParameters,
            hierarchy_scope: SelectionBulkHierarchyScope::ActiveHierarchyPath,
            model_cell: String::new(),
            current_property: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedSelectionBulkFilter {
    pub name: String,
    pub filter: SelectionBulkFilter,
}

/// Persisted active query and named filters. This is user/session state, never
/// project-owned design data.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SelectionBulkFilterSession {
    pub active: SelectionBulkFilter,
    saved: Vec<SavedSelectionBulkFilter>,
}

impl SelectionBulkFilterSession {
    const MAX_SAVED_FILTERS: usize = 32;
    const MAX_FILTER_NAME_BYTES: usize = 96;

    pub fn saved(&self) -> &[SavedSelectionBulkFilter] {
        &self.saved
    }

    pub fn save_active(&mut self, name: &str) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("A saved filter name is required.".to_owned());
        }
        if name.len() > Self::MAX_FILTER_NAME_BYTES || name.chars().any(char::is_control) {
            return Err("The saved filter name is invalid or too long.".to_owned());
        }
        if let Some(existing) = self
            .saved
            .iter_mut()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
        {
            existing.name = name.to_owned();
            existing.filter = self.active.clone();
            return Ok(());
        }
        if self.saved.len() >= Self::MAX_SAVED_FILTERS {
            return Err(format!(
                "At most {} selection filters may be saved.",
                Self::MAX_SAVED_FILTERS
            ));
        }
        self.saved.push(SavedSelectionBulkFilter {
            name: name.to_owned(),
            filter: self.active.clone(),
        });
        self.saved
            .sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
        Ok(())
    }

    pub fn load(&mut self, name: &str) -> bool {
        let Some(filter) = self
            .saved
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .map(|entry| entry.filter.clone())
        else {
            return false;
        };
        self.active = filter;
        true
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.saved.len();
        self.saved
            .retain(|entry| !entry.name.eq_ignore_ascii_case(name));
        self.saved.len() != before
    }
}

#[cfg(test)]
mod symbol_selection_tests {
    use super::*;

    #[test]
    fn named_bulk_filters_update_load_and_delete_case_insensitively() {
        let mut filters = SelectionBulkFilterSession::default();
        filters.active.query = "R*".to_owned();
        filters.save_active("Passives").expect("save");
        filters.active.query = "C*".to_owned();
        filters.save_active("passives").expect("replace");
        assert_eq!(filters.saved().len(), 1);
        assert!(filters.load("PASSIVES"));
        assert_eq!(filters.active.query, "C*");
        assert!(filters.remove("pAsSiVeS"));
        assert!(filters.saved().is_empty());
    }
}

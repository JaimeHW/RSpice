//! What the schematic bulk editor's queries select, and the named filters the
//! reader keeps with the session. Device-local, never project data.

use serde::{Deserialize, Serialize};

/// Object family searched by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkObjectKind {
    #[default]
    InstancesAndParameters,
    PortsAndPins,
}

impl SelectionBulkObjectKind {
    pub const ALL: [Self; 2] = [Self::InstancesAndParameters, Self::PortsAndPins];

    pub const fn label(self) -> &'static str {
        match self {
            Self::InstancesAndParameters => "Parts",
            Self::PortsAndPins => "Ports and pins",
        }
    }
}

/// Project extent inspected by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkHierarchyScope {
    /// The cell view in front and every block placed below it, as KiCad's
    /// "current sheet and below" reaches the sheets under the one shown.
    #[default]
    CellAndBlocksBelow,
    CurrentSheet,
    CompleteProject,
}

impl SelectionBulkHierarchyScope {
    pub const ALL: [Self; 3] = [
        Self::CellAndBlocksBelow,
        Self::CurrentSheet,
        Self::CompleteProject,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::CellAndBlocksBelow => "This cell and blocks below",
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
            hierarchy_scope: SelectionBulkHierarchyScope::CellAndBlocksBelow,
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
    pub fn saved(&self) -> &[SavedSelectionBulkFilter] {
        &self.saved
    }

    pub fn save_active(&mut self, name: &str) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("A saved filter name is required.".to_owned());
        }
        // The list scrolls and a long name is elided in it, so neither the
        // count nor the length is held to a number; only a name the list
        // could not show on one line is refused.
        if name.chars().any(char::is_control) {
            return Err("A saved filter name is one line of text.".to_owned());
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

    /// Filters are kept however many there are and however long their
    /// names; they were held to 32, of at most 96 bytes each.
    #[test]
    fn any_number_of_filters_with_names_of_any_length_are_kept() {
        let mut filters = SelectionBulkFilterSession::default();
        let long = "Passives on the output stage, sorted by tolerance, ".repeat(4);
        for index in 0..40 {
            filters.active.query = format!("R{index}");
            filters
                .save_active(&format!("{long}{index:02}"))
                .expect("the filter is kept");
        }
        assert_eq!(filters.saved().len(), 40);
        assert!(filters.load(&format!("{long}39")));
        assert_eq!(filters.active.query, "R39");
        assert_eq!(
            filters.save_active("two\nlines"),
            Err("A saved filter name is one line of text.".to_owned())
        );
    }
}

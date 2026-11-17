//! Library mutation vocabulary and validation shared by catalog and project transactions.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::project_sources::canonical_cell_view_owner_key;
use rspice_vocabulary::product::ContentDigest;

/// Exact semantic operation retained by the project library audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProjectLibraryMutation {
    CreateLibrary {
        library: String,
    },
    RenameLibrary {
        from_library: String,
        to_library: String,
    },
    DeleteLibrary {
        library: String,
    },
    /// A library copied whole under a new name: every cell, view and source
    /// it holds.
    CopyLibrary {
        source_library: String,
        target_library: String,
    },
    CreateCell {
        library: String,
        cell: String,
    },
    CreateView {
        library: String,
        cell: String,
        view: String,
    },
    CopyCell {
        source_library: String,
        source_cell: String,
        target_library: String,
        target_cell: String,
    },
    RenameCell {
        library: String,
        from_cell: String,
        to_cell: String,
    },
    /// A cell moved to another library, under its name there; whatever
    /// placed or named it follows.
    MoveCell {
        source_library: String,
        source_cell: String,
        target_library: String,
        target_cell: String,
    },
    /// One view copied into a cell, under its name there.
    CopyView {
        source_library: String,
        source_cell: String,
        source_view: String,
        target_library: String,
        target_cell: String,
        target_view: String,
    },
    /// The properties of a library or, with `cell`, of one of its cells:
    /// a description, or whether the library takes changes.
    EditProperties {
        library: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cell: Option<String>,
    },
    RenameView {
        library: String,
        cell: String,
        from_view: String,
        to_view: String,
    },
    DeleteCell {
        library: String,
        cell: String,
    },
    DeleteView {
        library: String,
        cell: String,
        view: String,
    },
    RollbackPublication {
        publication_id: Uuid,
        publication_label: String,
        snapshot_digest: ContentDigest,
        actor_id: String,
        authority_id: String,
        reason: String,
    },
    /// Several operations taken as one, in order: a multi-selection's
    /// deletion, which one review confirms and one step of Undo reverses.
    Batch {
        operations: Vec<ProjectLibraryMutation>,
    },
}

impl ProjectLibraryMutation {
    #[must_use]
    pub const fn operation_label(&self) -> &'static str {
        match self {
            Self::CreateLibrary { .. } => "library creation",
            Self::RenameLibrary { .. } => "library rename",
            Self::DeleteLibrary { .. } => "library deletion",
            Self::CopyLibrary { .. } => "library copy",
            Self::MoveCell { .. } => "cell move",
            Self::CopyView { .. } => "view copy",
            Self::EditProperties { .. } => "properties change",
            Self::CreateCell { .. } => "cell creation",
            Self::CreateView { .. } => "view creation",
            Self::CopyCell { .. } => "cell copy",
            Self::RenameCell { .. } => "cell rename",
            Self::RenameView { .. } => "view rename",
            Self::DeleteCell { .. } => "cell deletion",
            Self::DeleteView { .. } => "view deletion",
            Self::RollbackPublication { .. } => "restore from publication",
            Self::Batch { .. } => "library changes",
        }
    }

    /// Validate the operation before it enters a project audit or publication.
    pub fn validate(&self) -> Result<(), String> {
        let fields: &[(&str, &str)] = match self {
            Self::Batch { operations } => {
                if operations.is_empty() {
                    return Err("a batch of library changes holds none".to_owned());
                }
                return operations.iter().try_for_each(|operation| match operation {
                    Self::Batch { .. } => {
                        Err("a batch of library changes holds another batch".to_owned())
                    }
                    Self::RollbackPublication { .. } => {
                        Err("a batch of library changes holds a restore".to_owned())
                    }
                    _ => operation.validate(),
                });
            }
            Self::CreateLibrary { library }
            | Self::DeleteLibrary { library }
            | Self::EditProperties {
                library,
                cell: None,
            } => &[("library", library)],
            Self::EditProperties {
                library,
                cell: Some(cell),
            } => &[("library", library), ("cell", cell)],
            Self::CopyLibrary {
                source_library,
                target_library,
            } => &[
                ("source_library", source_library),
                ("target_library", target_library),
            ],
            Self::MoveCell {
                source_library,
                source_cell,
                target_library,
                target_cell,
            } => &[
                ("source_library", source_library),
                ("source_cell", source_cell),
                ("target_library", target_library),
                ("target_cell", target_cell),
            ],
            Self::CopyView {
                source_library,
                source_cell,
                source_view,
                target_library,
                target_cell,
                target_view,
            } => &[
                ("source_library", source_library),
                ("source_cell", source_cell),
                ("source_view", source_view),
                ("target_library", target_library),
                ("target_cell", target_cell),
                ("target_view", target_view),
            ],
            Self::RenameLibrary {
                from_library,
                to_library,
            } => &[("from_library", from_library), ("to_library", to_library)],
            Self::CreateCell { library, cell } | Self::DeleteCell { library, cell } => {
                &[("library", library), ("cell", cell)]
            }
            Self::CreateView {
                library,
                cell,
                view,
            }
            | Self::DeleteView {
                library,
                cell,
                view,
            } => &[("library", library), ("cell", cell), ("view", view)],
            Self::CopyCell {
                source_library,
                source_cell,
                target_library,
                target_cell,
            } => &[
                ("source_library", source_library),
                ("source_cell", source_cell),
                ("target_library", target_library),
                ("target_cell", target_cell),
            ],
            Self::RenameCell {
                library,
                from_cell,
                to_cell,
            } => &[
                ("library", library),
                ("from_cell", from_cell),
                ("to_cell", to_cell),
            ],
            Self::RenameView {
                library,
                cell,
                from_view,
                to_view,
            } => &[
                ("library", library),
                ("cell", cell),
                ("from_view", from_view),
                ("to_view", to_view),
            ],
            Self::RollbackPublication {
                publication_label,
                actor_id,
                authority_id,
                reason,
                ..
            } => &[
                ("publication_label", publication_label),
                ("actor_id", actor_id),
                ("authority_id", authority_id),
                ("reason", reason),
            ],
        };
        for (field, value) in fields {
            validate_library_audit_text(field, value)?;
        }

        match self {
            Self::RenameLibrary {
                from_library,
                to_library,
            } if canonical_cell_view_owner_key(from_library, "", "")
                == canonical_cell_view_owner_key(to_library, "", "") =>
            {
                Err("library rename source and target identities are equal".to_owned())
            }
            Self::RenameView {
                library,
                cell,
                from_view,
                to_view,
            } if canonical_cell_view_owner_key(library, cell, from_view)
                == canonical_cell_view_owner_key(library, cell, to_view) =>
            {
                Err("view rename source and target identities are equal".to_owned())
            }
            Self::CopyCell {
                source_library,
                source_cell,
                target_library,
                target_cell,
            } if canonical_cell_view_owner_key(source_library, source_cell, "")
                == canonical_cell_view_owner_key(target_library, target_cell, "") =>
            {
                Err("cell copy source and target identities are equal".to_owned())
            }
            Self::RenameCell {
                library,
                from_cell,
                to_cell,
            } if canonical_cell_view_owner_key(library, from_cell, "")
                == canonical_cell_view_owner_key(library, to_cell, "") =>
            {
                Err("cell rename source and target identities are equal".to_owned())
            }
            Self::CopyLibrary {
                source_library,
                target_library,
            } if canonical_cell_view_owner_key(source_library, "", "")
                == canonical_cell_view_owner_key(target_library, "", "") =>
            {
                Err("library copy source and target identities are equal".to_owned())
            }
            Self::MoveCell {
                source_library,
                target_library,
                ..
            } if canonical_cell_view_owner_key(source_library, "", "")
                == canonical_cell_view_owner_key(target_library, "", "") =>
            {
                Err("a cell moves to another library; within one it is renamed".to_owned())
            }
            Self::CopyView {
                source_library,
                source_cell,
                source_view,
                target_library,
                target_cell,
                target_view,
            } if canonical_cell_view_owner_key(source_library, source_cell, source_view)
                == canonical_cell_view_owner_key(target_library, target_cell, target_view) =>
            {
                Err("view copy source and target identities are equal".to_owned())
            }
            Self::RollbackPublication { publication_id, .. } if publication_id.is_nil() => {
                Err("library publication rollback identity must not be nil".to_owned())
            }
            _ => Ok(()),
        }
    }
}

/// Validate bounded audit text for a library operation or publication.
pub fn validate_library_audit_text(field: &str, value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{field} is required"));
    }
    if value != value.trim() {
        return Err(format!("{field} must not begin or end with whitespace"));
    }
    if value.chars().count() > 240 {
        return Err(format!("{field} exceeds 240 Unicode scalar values"));
    }
    if value.chars().any(char::is_control) {
        return Err(format!("{field} contains a control character"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delete(cell: &str) -> ProjectLibraryMutation {
        ProjectLibraryMutation::DeleteCell {
            library: "work".to_owned(),
            cell: cell.to_owned(),
        }
    }

    /// A batch holds plain operations, each valid, and at least one; it
    /// round-trips through the audit's own encoding.
    #[test]
    fn a_batch_holds_valid_plain_operations_and_round_trips() {
        let batch = ProjectLibraryMutation::Batch {
            operations: vec![delete("amp"), delete("bias")],
        };
        assert_eq!(batch.validate(), Ok(()));
        let encoded = serde_json::to_string(&batch).expect("the batch encodes");
        assert_eq!(
            serde_json::from_str::<ProjectLibraryMutation>(&encoded).expect("it decodes"),
            batch
        );

        let empty = ProjectLibraryMutation::Batch {
            operations: Vec::new(),
        };
        assert!(empty.validate().is_err());
        let nested = ProjectLibraryMutation::Batch {
            operations: vec![batch.clone()],
        };
        assert!(nested.validate().is_err());
        let invalid = ProjectLibraryMutation::Batch {
            operations: vec![delete(" amp")],
        };
        assert!(invalid.validate().is_err());
    }
}

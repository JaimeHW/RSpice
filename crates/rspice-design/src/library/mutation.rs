//! Library mutation vocabulary and validation shared by catalog and project transactions.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::project_sources::canonical_cell_view_owner_key;
use rspice_app_types::product::ContentDigest;

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
}

impl ProjectLibraryMutation {
    #[must_use]
    pub const fn operation_label(&self) -> &'static str {
        match self {
            Self::CreateLibrary { .. } => "library creation",
            Self::RenameLibrary { .. } => "library rename",
            Self::DeleteLibrary { .. } => "library deletion",
            Self::CreateCell { .. } => "cell creation",
            Self::CreateView { .. } => "view creation",
            Self::CopyCell { .. } => "cell copy",
            Self::RenameCell { .. } => "cell rename",
            Self::RenameView { .. } => "view rename",
            Self::DeleteCell { .. } => "cell deletion",
            Self::DeleteView { .. } => "view deletion",
            Self::RollbackPublication { .. } => "library publication rollback",
        }
    }

    /// Validate the operation before it enters a project audit or publication.
    pub fn validate(&self) -> Result<(), String> {
        let fields: &[(&str, &str)] = match self {
            Self::CreateLibrary { library } | Self::DeleteLibrary { library } => {
                &[("library", library)]
            }
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

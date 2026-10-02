//! Document context retained by an armed placement batch.

use crate::requests::EditorRequestSource;

/// The document an armed placement belongs to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlacementAuthority {
    source: EditorRequestSource,
}

impl PlacementAuthority {
    pub fn new(source: EditorRequestSource) -> Self {
        Self { source }
    }

    /// Match the project, document, occurrence and sheet that own the batch.
    /// Revisions may advance within that context. Each placement owner remains
    /// responsible for validating its contract against the live design.
    /// Requiring frozen content or sheet-catalog revisions here would reject
    /// a batch's own subsequent placements and same-document conflict retries.
    /// This context check does not grant edit permission.
    pub fn matches(&self, current: &EditorRequestSource) -> bool {
        self.source.project == current.project
            && self.source.document == current.document
            && self.source.occurrence == current.occurrence
            && self.source.design_epoch == current.design_epoch
            && self.source.document_epoch == current.document_epoch
            && self.source.sheet.map(|(id, _)| id) == current.sheet.map(|(id, _)| id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_authority_matches_only_its_own_document() {
        use rspice_app_types::product::ProjectId;
        use rspice_design::occurrence::DocumentOccurrence;
        use rspice_design_model::{cell_view::CellViewRef, design_management::SheetId};

        let document = CellViewRef::new("work", "ota_5t", "schematic");
        let source = EditorRequestSource {
            project: ProjectId::new(),
            occurrence: Some(DocumentOccurrence::rooted(document.clone())),
            document,
            design_epoch: 7,
            document_epoch: 3,
            content_version: 11,
            topology_version: 5,
            symbol_revision: 2,
            sheet: Some((Some(SheetId::new()), 4)),
        };
        let authority = PlacementAuthority::new(source.clone());
        assert!(authority.matches(&source));
        for change in [
            "project",
            "document",
            "occurrence",
            "design",
            "buffer",
            "sheet",
            "catalog",
        ] {
            let mut current = source.clone();
            match change {
                "project" => current.project = ProjectId::new(),
                "document" => current.document.view = "symbol".to_owned(),
                "occurrence" => current
                    .occurrence
                    .as_mut()
                    .unwrap()
                    .descend("X1".to_owned(), source.document.clone()),
                "design" => current.design_epoch += 1,
                "buffer" => current.document_epoch += 1,
                "sheet" => current.sheet.as_mut().unwrap().0 = Some(SheetId::new()),
                "catalog" => current.sheet = None,
                _ => unreachable!(),
            }
            assert!(!authority.matches(&current), "{change}");
        }
        let mut revised = source;
        revised.content_version += 1;
        revised.topology_version += 1;
        revised.symbol_revision += 1;
        revised.sheet.as_mut().unwrap().1 += 1;
        assert!(authority.matches(&revised));
    }
}

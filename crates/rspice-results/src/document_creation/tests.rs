use super::*;
use crate::viewer_catalog::VIEWER_DOCUMENTS;

#[test]
fn all_catalog_rows_are_covered_by_family_classification() {
    for viewer in VIEWER_DOCUMENTS {
        assert!(
            ResultDocumentFamily::ALL
                .into_iter()
                .any(|family| family.includes(viewer)),
            "{} has no result-document family",
            viewer.id
        );
    }
}

/// The Create dialog binds a new document's first pane from the family's
/// own `includes` list, and the persistent docbar scopes that page with
/// [`ResultDocumentFamily::offers_sheet`]. A docbar refusing the pane the
/// dialog had just bound would strand the document with no reachable sheet
/// — which is what the digital family did while the two lists were
/// maintained apart: it composed waveform and eye panes, then admitted
/// neither sheet.
#[test]
fn every_family_offers_the_sheets_its_create_path_can_bind() {
    for family in ResultDocumentFamily::ALL {
        for viewer in ResultViewer::all() {
            let Some(document_id) = viewer.viewer_document_id() else {
                continue;
            };
            let composes = VIEWER_DOCUMENTS
                .iter()
                .any(|document| document.id == document_id && family.includes(document));
            assert!(
                !composes || family.offers_sheet(viewer),
                "{} composes {document_id} but its docbar refuses {viewer:?}",
                family.label()
            );
        }
    }
}

/// Pages are titled with their family label at creation and nothing else
/// records the family, so this coupling is what makes the scoping work at
/// all. If either side is renamed, every page of that family silently
/// widens to offering all sheets.
#[test]
fn each_family_label_round_trips_to_the_family_it_titles_pages_with() {
    assert_eq!(
        ResultDocumentFamily::ALL.map(ResultDocumentFamily::id),
        crate::viewer_catalog::RESULT_CREATION_FAMILIES
            .iter()
            .map(|family| family.id)
            .collect::<Vec<_>>()
            .as_slice()
    );
    for family in ResultDocumentFamily::ALL {
        assert_eq!(
            ResultDocumentFamily::from_label(family.label()),
            Some(family)
        );
    }
}

//! The publish dialog's output contract, in the case that exports the pane
//! the Results workspace has selected.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::hardcopy::hardcopy_parts;
use crate::workbench::app::dialogs::hardcopy::publish::active_retained_source_selection;

#[test]
fn results_export_selects_the_active_project_document_pane_authority() {
    use crate::product::{AnalysisInstanceId, ContentDigest, DatasetBinding, DatasetId};
    use crate::results::visualization_document::{
        ColumnRole, DocumentEdit, PaneDataBinding, SourceColumn, SourceDataset, SourceRow,
        TypedValue, ValueType, VisualizationDocument,
    };
    use crate::workbench::state::{Workspace, WorkspaceDocumentId};

    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([0x6e; 32]));
    let source = SourceDataset::new(
        binding,
        vec![
            SourceColumn::new("x", "X", ValueType::Real, ColumnRole::Coordinate, None).unwrap(),
            SourceColumn::new("y", "Y", ValueType::Real, ColumnRole::Signal, None).unwrap(),
        ],
        vec![SourceRow::new(vec![
            TypedValue::Real(0.0),
            TypedValue::Real(1.0),
        ])],
    )
    .unwrap();
    let mut document = VisualizationDocument::new("Results export", vec![source]).unwrap();
    let pane_id = document.panes()[0].id;
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetPaneSource {
                pane_id,
                viewer_id: "viewer-waveform".to_owned(),
                binding: Some(PaneDataBinding {
                    analysis_id: AnalysisInstanceId::new(),
                    dataset: binding,
                }),
            }],
        )
        .unwrap();
    let document_id = document.id();
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .workspace
        .content
        .visualization_documents
        .push(document);
    app.state.workbench.activate(Workspace::Results);
    app.state
        .workbench
        .documents
        .activate(WorkspaceDocumentId::VisualizationDocument(document_id));
    app.state
        .results
        .results
        .session
        .select_persistent_document_pane(document_id, pane_id);

    let (source_key, scope) = active_retained_source_selection(&hardcopy_parts(&mut app)).unwrap();

    assert!(source_key.contains(&format!(
        ":result-document:{document_id}:pane:{}",
        pane_id.get()
    )));
    assert_eq!(
        scope,
        rspice_hardcopy_contract::HardcopyScope::ActivePlotDocument
    );
}

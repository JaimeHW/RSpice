//! Exact executable model provenance through production catalog transactions.

use super::*;
use rspice_model_library::ModelCatalog;

#[test]
fn prepared_model_receipts_authenticate_exact_executable_model_definition() {
    let mut catalog = ModelCatalog::default();
    let definition = prepared_model_receipt_definition();
    create_project_model(&mut catalog, "models-a", &definition).unwrap();
    let source = definition.canonical_source().unwrap();
    let used = prepared_project_model_sources(
        &catalog,
        &format!("receipt fixture\n{source}M1 d g 0 0 nch_receipt\n.op\n.end\n"),
    )
    .unwrap();
    assert_eq!(used.len(), 1);
    assert_eq!(used[0].model_name(), "nch_receipt");
}

#[test]
fn prepared_model_receipts_reject_modified_same_name_manual_card() {
    let mut catalog = ModelCatalog::default();
    let definition = prepared_model_receipt_definition();
    create_project_model(&mut catalog, "models-a", &definition).unwrap();

    let modified = prepared_project_model_sources(
            &catalog,
            "receipt fixture\n.model nch_receipt NMOS (level=1 vth0=0.51)\nM1 d g 0 0 nch_receipt\n.op\n.end\n",
        )
        .unwrap();
    assert!(
        modified.is_empty(),
        "a same-name card with different executable parameters must not inherit project provenance"
    );
}

#[test]
fn prepared_model_receipts_ignore_exact_but_unused_model() {
    let mut catalog = ModelCatalog::default();
    let definition = prepared_model_receipt_definition();
    create_project_model(&mut catalog, "models-a", &definition).unwrap();
    let source = definition.canonical_source().unwrap();
    let unused = prepared_project_model_sources(
        &catalog,
        &format!("receipt fixture\n{source}V1 out 0 1\nR1 out 0 1k\n.op\n.end\n"),
    )
    .unwrap();
    assert!(unused.is_empty());
}

#[test]
fn prepared_model_receipts_reject_duplicate_project_model_name() {
    let mut catalog = ModelCatalog::default();
    let definition = prepared_model_receipt_definition();
    create_project_model(&mut catalog, "models-a", &definition).unwrap();
    create_project_model(&mut catalog, "models-b", &definition).unwrap();
    let source = definition.canonical_source().unwrap();
    let ambiguous = prepared_project_model_sources(
        &catalog,
        &format!("receipt fixture\n{source}M1 d g 0 0 nch_receipt\n.op\n.end\n"),
    )
    .unwrap();
    assert!(
        ambiguous.is_empty(),
        "an executable model name shared by multiple project sources cannot authenticate either source"
    );
}

fn prepared_model_receipt_definition() -> rspice_model_library::ProjectModelDefinition {
    rspice_model_library::ProjectModelDefinition {
        name: "nch_receipt".to_owned(),
        spice_type: "NMOS".to_owned(),
        description: "Prepared receipt fixture".to_owned(),
        numeric_parameters: std::collections::BTreeMap::from([
            ("level".to_owned(), 1.0),
            ("vth0".to_owned(), 0.48),
        ]),
        string_parameters: std::collections::BTreeMap::new(),
    }
}

fn create_project_model(
    catalog: &mut ModelCatalog,
    library_name: &str,
    definition: &rspice_model_library::ProjectModelDefinition,
) -> Result<rspice_model_library::ProjectModelCommit, String> {
    let metadata = definition.reconcile_metadata(None)?;
    catalog.create_project_model_revision(
        library_name,
        &rspice_model_library::ProjectModelRevisionDefinition::new(definition.clone(), metadata),
        &rspice_model_library::qualification::ModelQualificationState::default(),
    )
}

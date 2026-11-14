//! What the netlist surface's cases seed: a generated primary retained in
//! the model, and a manual-deck run sealed into history. The surface's own
//! cases and the code feature's whole-application tests share them.

// Test-only code compiled under test-support, outside the parent's cfg(test) exemption.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable
)]

use super::generation;

pub fn retain_generated(
    model: &mut rspice_app_state::workbench::app_state::model::Model,
    netlist: &mut crate::workbench::documents::netlist_document::NetlistDocumentState,
    source: &str,
) {
    let input_digest = crate::product::ContentDigest::from_bytes([0x41; 32]);
    let source = source.to_owned();
    let (document, owned) =
        generation::publish_generated_document(model, netlist, input_digest, source.clone())
            .expect("canonical generated fixture");
    model.netlist_artifacts.generated_source = source;
    model.netlist_artifacts.generated_document = Some(document);
    netlist.owned_document = owned;
    model.netlist_artifacts.generated_input_digest = Some(input_digest);
    model.netlist_artifacts.current_generation_input_digest = Some(input_digest);
}

/// Seal one manual-deck run into history the way a dispatch does, so the
/// strip has a real receipt to read its digest and revision back off.
pub fn seal_manual_run(
    model: &mut rspice_app_state::workbench::app_state::model::Model,
    run_number: u64,
    deck_digest: u8,
    revision: u64,
) {
    let project_revision =
        crate::product::ObjectRevision::new(revision).expect("non-zero revision");
    let task = crate::state::PreparedRunTaskReceipt::new(
        crate::product::AnalysisInstanceId::new(),
        project_revision,
        Vec::new(),
        0,
        crate::product::ContentDigest::from_bytes([0x11; 32]),
    )
    .expect("valid task receipt");
    let receipt = crate::state::PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: crate::state::AnalysisResultSourceDomain::ManualDeck,
        simulation_plan_id: None,
        project_revision,
        prepared_snapshot_digest: crate::product::ContentDigest::from_bytes([0x22; 32]),
        source_content_digest: crate::product::ContentDigest::from_bytes([deck_digest; 32]),
        source_check_receipt: crate::state::PreparedSourceCheckReceipt::ManualSourceCheck(
            crate::product::ContentDigest::from_bytes([0x33; 32]),
        ),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![task],
    })
    .expect("valid manual deck receipt");
    let mut run = crate::state::SimulationRun::new(run_number);
    run.restore_provenance(crate::state::SimulationRunProvenance::Prepared(Box::new(
        receipt,
    )))
    .expect("fresh run accepts its receipt");
    model.simulation.retained.runs.push(run);
}

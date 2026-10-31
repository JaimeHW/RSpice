//! A binned family is one catalog row, named as an instance binds it.
//!
//! A foundry kit's `nch.1` … `nch.48` were 48 rows, and binding from one
//! pinned the instance to that bin, which then ran at every size. The row is
//! the family now: its bins are its detail, the instances its bins run for are
//! counted against it, and Bind gives an instance `nch` with its library.

use super::*;
use crate::workbench::models::tests::kit_project;

fn scan_rows(state: &mut FeatureHarness<ModelsFeature>) -> (Vec<(String, usize, usize)>, usize) {
    let mut pending_actions = Vec::new();
    let render = ManagerRenderContext::over(state, &mut pending_actions);
    let consumers = ConsumerIndex::build(&render);
    let scan = project_catalog_scan(&render, &consumers);
    assert!(
        scan.consumer_diagnostics.is_empty(),
        "every instance resolves: {:?}",
        scan.consumer_diagnostics
    );
    let rows = scan
        .rows
        .iter()
        .map(|row| (row.model.clone(), row.bins, row.usage_count))
        .collect();
    (rows, scan.total)
}

/// One row for the family, none for its bins, and both the instance naming
/// the family and the one naming a bin are its consumers.
#[test]
fn a_family_is_one_row_that_counts_every_instance_its_bins_run_for() {
    let mut app = harness();
    kit_project(&mut app.model, &mut app.state, &["nch", "nch.2", "nch_25"]);
    let (rows, total) = scan_rows(&mut app);
    assert_eq!(
        rows,
        [("nch".to_owned(), 2, 2), ("nch_25".to_owned(), 0, 1)],
        "the bins are the family's detail, not rows beside it"
    );
    assert_eq!(total, 2, "the footer counts a family once");

    let mut pending_actions = Vec::new();
    let render = ManagerRenderContext::over(&mut app, &mut pending_actions);
    let consumers = ConsumerIndex::build(&render);
    let kit = render
        .model
        .model_library_manager
        .get_library("kit")
        .expect("kit");
    let family = kit.bin_family("nch").expect("the kit provides nch");
    let labels = consumers.of_family(kit, &family);
    assert_eq!(labels.len(), 2, "{labels:?}");
    assert!(
        labels
            .iter()
            .any(|label| bindings::consumer_designator(label) == "M1"
                && !label.contains("names bin")),
        "{labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| bindings::consumer_designator(label) == "M2"
                && label.ends_with("names bin nch.2")),
        "an instance naming one bin says which: {labels:?}"
    );
}

/// A library holding a card of the family's own name runs that card for the
/// name, so it provides no family and its bins stay cards an instance can
/// bind exactly.
#[test]
fn a_card_of_the_familys_own_name_keeps_its_bins_as_rows() {
    let mut app = harness();
    kit_project(&mut app.model, &mut app.state, &["nch.1"]);
    app.model
        .model_library_manager
        .get_library_mut("kit")
        .expect("kit")
        .add_model(DeviceModel::new("nch", ModelType::Nmos));
    let (rows, total) = scan_rows(&mut app);
    assert_eq!(
        rows,
        [
            ("nch".to_owned(), 0, 0),
            ("nch.1".to_owned(), 0, 1),
            ("nch.2".to_owned(), 0, 0),
            ("nch_25".to_owned(), 0, 0),
        ]
    );
    assert_eq!(total, 4);
}

//! The boundary cases where the old duplicated derivations disagreed.

use std::path::PathBuf;

use super::*;
use crate::product::ContentDigest;
use crate::state::model_library::{ModelSourceEdge, ModelSourcePin, ModelType};

fn binned(name: &str, l_min: f64, l_max: f64, w_min: f64, w_max: f64) -> DeviceModel {
    DeviceModel::new(name, ModelType::Nmos).with_geometry(l_min, l_max, w_min, w_max)
}

fn pin(path: &str) -> ModelSourcePin {
    ModelSourcePin {
        path: PathBuf::from(path),
        digest: ContentDigest::from_bytes([0x11; 32]),
    }
}

#[test]
fn a_fixed_geometry_bin_is_valid_because_the_engine_range_test_includes_both_ends() {
    // The page said valid (`min > max`), the inspector said invalid
    // (`min >= max`). Core's `validate_model_bin_range` only rejects a genuine
    // reversal, so the page was right and the inspector was inventing a fault.
    let point = binned("nch_fixed", 5e-7, 5e-7, 1e-6, 1e-6);
    assert!(!envelope_is_invalid(&point));

    let reversed = binned("nch_reversed", 9e-7, 5e-7, 1e-6, 2e-6);
    assert!(envelope_is_invalid(&reversed));
}

#[test]
fn a_reversal_inside_the_boundary_tolerance_is_not_a_finding() {
    // A deck writing 0.22u lands one ULP off a card's 2.2e-007. Comparing
    // exactly would report a reversal the engine does not have.
    let mut model = binned("nch_ulp", 2.2e-7, 2.2e-7, 1e-6, 2e-6);
    model.l_min = Some(2.2e-7 + 1e-12);
    assert!(!envelope_is_invalid(&model));
}

#[test]
fn a_source_included_by_two_libraries_is_one_file() {
    // The page counted distinct paths; the inspector summed each library's
    // closure length. Two libraries sharing `common.inc` made them disagree.
    let mut first = ModelLibrary::new("pdk_a");
    first.root_path = Some(PathBuf::from("a.lib"));
    first.source_closure = vec![pin("a.lib"), pin("common.inc")];
    let mut second = ModelLibrary::new("pdk_b");
    second.root_path = Some(PathBuf::from("b.lib"));
    second.source_closure = vec![pin("b.lib"), pin("common.inc")];

    let facts = closure_facts([&first, &second]);
    assert_eq!(facts.files, 3, "common.inc must be counted once");
    assert_eq!(
        first.source_closure.len() + second.source_closure.len(),
        4,
        "the summed derivation this replaces double-counted the shared include"
    );
    assert_eq!(facts.unpinned_roots, 0);
}

#[test]
fn a_library_with_a_root_and_no_retained_closure_is_an_unpinned_root() {
    let mut library = ModelLibrary::new("external");
    library.root_path = Some(PathBuf::from("vendor.lib"));
    let facts = closure_facts([&library]);
    assert_eq!(facts.unpinned_roots, 1);
    assert_eq!(facts.diagnostics(), 1);
}

#[test]
fn only_nodes_remaining_after_a_topological_sort_are_cyclic() {
    let mut acyclic = ModelLibrary::new("acyclic");
    acyclic.source_edges = vec![
        ModelSourceEdge {
            owner: PathBuf::from("a"),
            requested_path: "b".to_owned(),
            target: PathBuf::from("b"),
        },
        ModelSourceEdge {
            owner: PathBuf::from("b"),
            requested_path: "c".to_owned(),
            target: PathBuf::from("c"),
        },
    ];
    assert_eq!(closure_facts([&acyclic]).cyclic_nodes, 0);

    let mut self_cycle = ModelLibrary::new("cyclic");
    self_cycle.source_edges = vec![ModelSourceEdge {
        owner: PathBuf::from("a"),
        requested_path: "a".to_owned(),
        target: PathBuf::from("a"),
    }];
    assert_eq!(closure_facts([&self_cycle]).cyclic_nodes, 1);
}

#[test]
fn the_section_index_reads_what_parsing_produced_not_the_retained_bytes() {
    let mut library = ModelLibrary::new("pdk");
    let mut typical = DeviceModel::new("nch", ModelType::Nmos);
    typical.section = Some("TT".to_owned());
    let mut slow = DeviceModel::new("nch_ss", ModelType::Nmos);
    slow.section = Some("ss".to_owned());
    library.add_model(typical);
    library.add_model(slow);

    let index = library.section_index();
    assert!(
        index.contains("tt"),
        "section names are matched case-folded"
    );
    assert!(index.contains("ss"));
    assert!(library.defines_section("tt"));
    assert!(library.defines_section("TT"));
    assert!(
        !library.defines_section("ff"),
        "a section nothing defines must not resolve"
    );
}

#[test]
fn one_digest_rendering_keeps_the_head_and_the_tail() {
    let digest = ContentDigest::from_bytes([0xab; 32]).to_string();
    let short = short_digest(&digest);
    assert!(short.starts_with(&digest[..8]));
    assert!(short.ends_with(&digest[digest.len() - 4..]));
    assert_eq!(short_digest("short"), "short");
}

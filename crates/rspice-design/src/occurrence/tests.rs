//! What an occurrence has to be true of on its own, before any document owns
//! one: it names only the instances below the implicit root, it always opens
//! the master it ends at, and it truncates rather than inventing a step.

use super::*;

fn master(cell: &str) -> CellViewRef {
    CellViewRef::new("work", cell, "schematic")
}

fn descended(instances: &[&str]) -> DocumentOccurrence {
    let mut occurrence = DocumentOccurrence::rooted(master("tb_ota"));
    for (index, instance) in instances.iter().enumerate() {
        occurrence.descend((*instance).to_owned(), master(&format!("level_{index}")));
    }
    occurrence
}

#[test]
fn the_occurrence_path_names_instances_below_the_implicit_root() {
    assert!(descended(&[]).instance_path().is_root());
    assert_eq!(descended(&["X1"]).instance_path().to_string(), "/X1");
    assert_eq!(
        descended(&["X1", "XB"]).instance_path().to_string(),
        "/X1/XB"
    );
    assert!(
        descended(&["X 1"]).instance_path().is_root(),
        "a name the grammar cannot spell resolves to the root, not to half a path"
    );
    assert!(descended(&["X1", "X 2"]).instance_path().is_root());
}

#[test]
fn every_mutation_leaves_the_occurrence_opening_its_terminal_master() {
    let mut occurrence = DocumentOccurrence::rooted(master("tb_ota"));
    assert_eq!(occurrence.terminal_master(), &master("tb_ota"));
    occurrence.debug_assert_opens(&master("tb_ota"));

    occurrence.descend("X1".to_owned(), master("ota_5t"));
    assert_eq!(occurrence.terminal_master(), &master("ota_5t"));
    occurrence.debug_assert_opens(&master("ota_5t"));

    occurrence.descend("XB".to_owned(), master("bias_2t"));
    assert_eq!(occurrence.depth(), 3);
    occurrence.debug_assert_opens(&master("bias_2t"));

    occurrence.truncate_to(1);
    assert_eq!(occurrence.terminal_master(), &master("ota_5t"));
    assert_eq!(occurrence.instance_path().to_string(), "/X1");
    occurrence.debug_assert_opens(&master("ota_5t"));

    occurrence.truncate_to(0);
    assert_eq!(occurrence.terminal_master(), &master("tb_ota"));
    assert_eq!(occurrence.depth(), 1);
    assert!(occurrence.instance_path().is_root());
    occurrence.debug_assert_opens(&master("tb_ota"));

    occurrence.truncate_to(4);
    assert_eq!(
        occurrence.depth(),
        1,
        "the root has no descent a deeper truncation could restore"
    );
}

#[test]
fn labels_read_the_root_cell_then_the_instance_at_each_level() {
    let mut occurrence = DocumentOccurrence::rooted(master("tb_ota"));
    occurrence.descend("X1".to_owned(), master("ota_5t"));
    occurrence.descend("XB".to_owned(), master("bias_2t"));

    assert_eq!(occurrence.labels(), ["tb_ota", "X1", "XB"]);
    assert_eq!(
        occurrence.masters().cloned().collect::<Vec<_>>(),
        [master("tb_ota"), master("ota_5t"), master("bias_2t")]
    );
}

#[test]
fn pruning_keeps_the_deepest_prefix_that_still_resolves() {
    let mut occurrence = DocumentOccurrence::rooted(master("tb_ota"));
    occurrence.descend("X1".to_owned(), master("ota_5t"));
    occurrence.descend("XB".to_owned(), master("bias_2t"));

    let mut intact = occurrence.clone();
    assert_eq!(
        intact.retain_valid_prefix(|_| true),
        OccurrencePrune::Intact
    );
    assert_eq!(intact, occurrence);

    let mut truncated = occurrence.clone();
    assert_eq!(
        truncated.retain_valid_prefix(|reference| reference.cell != "ota_5t"),
        OccurrencePrune::Truncated
    );
    assert!(
        truncated.instance_path().is_root(),
        "the level below a master that is gone cannot be named, so it is dropped"
    );
    assert_eq!(truncated.terminal_master(), &master("tb_ota"));

    let mut rootless = occurrence.clone();
    assert_eq!(
        rootless.retain_valid_prefix(|reference| reference.cell != "tb_ota"),
        OccurrencePrune::Rootless
    );
}

#[test]
fn a_rename_rewrites_every_master_on_the_occurrence() {
    let mut occurrence = DocumentOccurrence::rooted(master("tb_ota"));
    occurrence.descend("X1".to_owned(), master("ota_5t"));

    for reference in occurrence.masters_mut() {
        if reference.cell == "ota_5t" {
            reference.cell = "ota_5t_v2".to_owned();
        }
    }

    assert_eq!(occurrence.terminal_master(), &master("ota_5t_v2"));
    assert_eq!(
        occurrence.instance_path().to_string(),
        "/X1",
        "renaming a master does not rename the instance that reaches it"
    );
}

#[test]
fn a_record_written_before_occurrences_existed_is_recognizable() {
    assert!(DocumentOccurrence::default().is_unrooted());
    assert!(!DocumentOccurrence::rooted(master("tb_ota")).is_unrooted());
}

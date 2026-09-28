use super::*;
use rspice_design_model::Point;

#[test]
fn reference_deltas_prepare_forward_and_reverse_without_mutating_sources() {
    let root = CellViewRef::new("work", "top", "schematic");
    let child = CellViewRef::new("work", "amp", "schematic");
    let mut source = ConfigurationSetCatalog::default();
    let id = source
        .create(ConfigurationSetDefinition {
            name: "release".to_owned(),
            root: root.clone(),
            dut_path: "/X1".to_owned(),
            executable_view_policy: vec!["schematic".to_owned()],
            stop_views: Vec::new(),
            unresolved_policy: Default::default(),
            black_box_policy: Default::default(),
            overrides: Vec::new(),
            model_profile: Default::default(),
            owner: "design".to_owned(),
        })
        .unwrap();
    let mut target = source.clone();
    let mut definition = source.find(id).unwrap().definition().clone();
    definition.dut_path = "/X2".to_owned();
    target.update(id, 1, definition).unwrap();
    let before = [
        Component::new(1, ComponentType::CellInstance, Point::origin())
            .with_name_value("X1", "amp"),
    ];
    let mut after = before.clone();
    after[0].name = "X2".to_owned();
    let mut changes = DesignReferenceChanges::between(&source, &target);
    changes.add_instance_renames(&root, &before, &after);
    let mut occurrence = DocumentOccurrence::rooted(root);
    occurrence.descend("x1".to_owned(), child.clone());
    let source_bytes = serde_json::to_vec(&source).unwrap();
    let occurrence_bytes = serde_json::to_vec(&occurrence).unwrap();

    let prepared = changes
        .checked_configurations(&source, true)
        .unwrap()
        .prepare()
        .unwrap();
    let updates = changes
        .prepare_occurrences([(&child, &occurrence)], true)
        .unwrap();
    assert_eq!(
        prepared.find(id).unwrap().definition(),
        target.find(id).unwrap().definition()
    );
    assert_eq!(prepared.find(id).unwrap().revision(), 2);
    assert_eq!(updates[0].0, child);
    assert_eq!(updates[0].1.instance_path().to_string(), "/X2");
    assert_eq!(updates[0].1.terminal_master(), &child);
    assert!(changes.checked_configurations(&prepared, true).is_none());
    assert!(changes.checked_configurations(&source, false).is_none());

    let restored = changes
        .checked_configurations(&prepared, false)
        .unwrap()
        .prepare()
        .unwrap();
    assert_eq!(
        restored.find(id).unwrap().definition(),
        source.find(id).unwrap().definition()
    );
    assert_eq!(restored.find(id).unwrap().revision(), 3);
    let restored_occurrences = changes
        .prepare_occurrences([(&child, &updates[0].1)], false)
        .unwrap();
    assert_eq!(restored_occurrences[0].1.instance_path().to_string(), "/X1");
    let reversed = changes.reversed();
    let reversed_catalog = reversed
        .checked_configurations(&prepared, true)
        .unwrap()
        .prepare()
        .unwrap();
    assert_eq!(
        serde_json::to_vec(&reversed_catalog).unwrap(),
        serde_json::to_vec(&restored).unwrap()
    );
    assert_eq!(
        reversed
            .prepare_occurrences([(&child, &updates[0].1)], true)
            .unwrap(),
        restored_occurrences
    );
    assert_eq!(serde_json::to_vec(&source).unwrap(), source_bytes);
    assert_eq!(serde_json::to_vec(&occurrence).unwrap(), occurrence_bytes);
    assert_eq!(before[0].name, "X1");
}

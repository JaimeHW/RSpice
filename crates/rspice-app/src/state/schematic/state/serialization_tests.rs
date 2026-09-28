//! Regression coverage for schematic wire layout and load-time editor defaults.

use super::*;
use crate::state::SchematicDocumentPolicy;

#[test]
fn schematic_wire_layout_and_loaded_runtime_defaults_are_stable() {
    let mut state = SchematicState::default();
    state.zoom = 3.0;
    state.pan = (10.0, 20.0);
    state
        .design
        .set_identity_for_test(SchematicIdentity::with_cursor(99));
    state.is_dirty = true;
    state.read_only = true;
    let json = serde_json::to_string(&state).unwrap();
    let fields = [
        "components",
        "wires",
        "buses",
        "bus_taps",
        "design_notes",
        "documentation_shapes",
        "probes",
        "grid_size",
        "document_policy",
        "net_labels",
        "junctions",
        "connections",
        "validated_revisions",
    ];
    let value: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(value.as_object().unwrap().len(), fields.len());
    let positions = fields.map(|field| json.find(&format!("\"{field}\":")).unwrap());
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    let ron =
        ron::ser::to_string_pretty(&state, ron::ser::PrettyConfig::default().struct_names(true))
            .unwrap();
    assert!(ron.starts_with("SchematicState("));
    for restored in [
        serde_json::from_str::<SchematicState>(&json).unwrap(),
        ron::from_str::<SchematicState>(&ron).unwrap(),
    ] {
        assert_eq!(serde_json::to_string(&restored).unwrap(), json);
        assert_eq!(restored.zoom, 1.0);
        assert_eq!(restored.pan, (0.0, 0.0));
        assert_eq!(restored.identity_cursor(), 0);
        assert_eq!(SchematicState::default().identity_cursor(), 1);
        assert!(!restored.is_dirty);
        assert!(!restored.read_only);
        assert!(!restored.needs_fit);
        assert!(!restored.needs_history_reset);
        assert!(!restored.has_pending_operation());
        assert!(!restored.can_undo());
    }
}

#[test]
fn schematic_readers_preserve_legacy_fields_and_reject_missing_or_duplicate_data() {
    let fields = [
        ("components", "[]"),
        ("wires", "[]"),
        ("grid_size", "7"),
        ("junctions", "[]"),
        ("connections", "[]"),
    ];
    let json = |fields: &[(&str, &str)]| {
        format!(
            "{{{}}}",
            fields
                .iter()
                .map(|(key, value)| format!("\"{key}\":{value}"))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let ron = |fields: &[(&str, &str)]| {
        format!(
            "SchematicState({})",
            fields
                .iter()
                .map(|(key, value)| format!("{key}:{value}"))
                .collect::<Vec<_>>()
                .join(",")
        )
    };
    let mut with_runtime = fields.to_vec();
    with_runtime.extend([
        ("zoom", "-3.0"),
        ("next_id", "99"),
        ("selection", "\"ignored\""),
        ("unknown_field", "[]"),
    ]);
    for input in [&fields[..], &with_runtime] {
        for restored in [
            serde_json::from_str::<SchematicState>(&json(input)).unwrap(),
            ron::from_str::<SchematicState>(&ron(input)).unwrap(),
        ] {
            assert_eq!(restored.design.document().grid_size, 7);
            assert_eq!(
                restored.design.document().document_policy,
                SchematicDocumentPolicy::default()
            );
            assert!(
                restored.design.document().buses.is_empty()
                    && restored.design.document().bus_taps.is_empty()
            );
            assert!(
                restored.design.document().design_notes.is_empty()
                    && restored.design.document().documentation_shapes.is_empty()
            );
            assert!(
                restored.design.document().probes.is_empty()
                    && restored.design.document().net_labels.is_empty()
            );
            assert!(restored.design.document().validated_revisions.is_empty());
            assert_eq!(restored.zoom, 1.0);
            assert_eq!(restored.identity_cursor(), 0);
            assert_eq!(
                restored.snap_engine.grid_size,
                SnapEngine::default().grid_size
            );
        }
    }
    for index in 0..fields.len() {
        let mut missing = fields.to_vec();
        missing.remove(index);
        assert!(serde_json::from_str::<SchematicState>(&json(&missing)).is_err());
        assert!(ron::from_str::<SchematicState>(&ron(&missing)).is_err());
        let mut duplicated = fields.to_vec();
        duplicated.push(fields[index]);
        assert!(serde_json::from_str::<SchematicState>(&json(&duplicated)).is_err());
        assert!(ron::from_str::<SchematicState>(&ron(&duplicated)).is_err());
    }
}

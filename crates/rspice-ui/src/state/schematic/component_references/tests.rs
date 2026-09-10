//! Replicas preserve reference ownership, reviewed previews, and atomic history.

use super::*;
use crate::state::{
    Point, SchematicArrayCount, SchematicArrayKind, SchematicArrayPlacement, SchematicArrayPlan,
    SchematicState,
};

fn coupled_selection() -> SchematicState {
    let mut state = SchematicState::default();
    for (index, kind) in [
        ComponentType::Inductor,
        ComponentType::Inductor,
        ComponentType::CoupledInductor,
        ComponentType::VoltageSource,
        ComponentType::Cccs,
        ComponentType::Ccvs,
    ]
    .into_iter()
    .enumerate()
    {
        let id = state.add_component(kind, Point::new(index as i32 * 100, 0));
        state.selection.select_component(id);
    }
    state.components[0].params = "coupled_to=l2 coupling_factor=0.9".to_owned();
    state.components[2].params = "inductors=\"l1, L2\"".to_owned();
    state.components[2].value = "0.9".to_owned();
    state.components[4].params = "vref=v1 m=2".to_owned();
    state.components[5].params = "vref=V_external".to_owned();
    // References precede their targets in the captured document order.
    state.components.rotate_right(4);
    state.clear_undo_history();
    state.is_dirty = false;
    state
}

fn component<'a>(components: &'a [Component], name: &str) -> &'a Component {
    components
        .iter()
        .find(|component| component.name == name)
        .unwrap()
}

fn parameter(component: &Component, name: &str) -> String {
    crate::state::parse_params_string(&component.params)[name].clone()
}

#[test]
fn rename_preparation_rebinds_only_the_target_and_preserves_the_source() {
    let state = coupled_selection();
    for (old, new, owner, parameter_name, expected) in [
        ("L2", "L9", "L1", "coupled_to", "L9"),
        ("L2", "L9", "K1", "inductors", "l1, L9"),
        ("V1", "V9", "F1", "vref", "V9"),
    ] {
        let original = state.components.clone();
        let renamed = state
            .prepare_component_rename(component(&original, old), new.to_owned())
            .unwrap();
        assert_eq!(
            parameter(component(&renamed, owner), parameter_name),
            expected
        );
        assert_eq!(component(&renamed, "H1").params, "vref=V_external");
        assert_eq!(state.components, original);
        assert!(!state.can_undo());
    }
}

#[test]
fn emitted_alias_handles_unprefixed_and_non_ascii_authored_names() {
    let inductor =
        Component::new(1, ComponentType::Inductor, Point::origin()).with_name_value("λ", "1u");
    assert_eq!(inductor.emitted_instance_name(), "Lλ");
    let voltage = Component::new(2, ComponentType::VoltageSource, Point::origin())
        .with_name_value("bias", "1");
    assert_eq!(voltage.emitted_instance_name(), "Vbias");
}

#[test]
fn unrelated_display_names_cannot_steal_a_physical_reference() {
    let mut state = coupled_selection();
    let resistor = Component::new(100, ComponentType::Resistor, Point::new(0, 200))
        .with_name_value("L1", "1k");
    let capacitor = Component::new(101, ComponentType::Capacitor, Point::new(100, 200))
        .with_name_value("V1", "1n");
    state.components.extend([resistor, capacitor]);
    let prepared = PreparedCopyReferences::new(&state.components).unwrap();
    let mut copies = state.components.clone();
    for copy in &mut copies {
        copy.name = format!("{}_copy", copy.emitted_instance_name());
    }
    prepared.apply(&mut copies);
    assert_eq!(
        parameter(component(&copies, "K1_copy"), "inductors"),
        "L1_copy, L2_copy"
    );
    assert_eq!(parameter(component(&copies, "F1_copy"), "vref"), "V1_copy");
}

#[test]
fn paste_rebinds_forward_references_in_one_undo_step() {
    let mut state = coupled_selection();
    let original = state.components.clone();
    state.copy_selection();
    assert!(state.paste_at_checked(Point::new(0, 1000)).unwrap());
    let pasted = state.components.clone();
    assert_eq!(&pasted[..original.len()], original);
    assert_eq!(parameter(component(&pasted, "L3"), "coupled_to"), "L4");
    assert_eq!(parameter(component(&pasted, "K2"), "inductors"), "L3, L4");
    assert_eq!(parameter(component(&pasted, "F2"), "vref"), "V2");
    assert_eq!(parameter(component(&pasted, "F2"), "m"), "2");
    assert_eq!(component(&pasted, "H2").params, "vref=V_external");

    assert_eq!(state.undo_description(), Some("paste"));
    assert!(state.undo());
    assert_eq!(state.components, original);
    assert!(!state.can_undo());
    assert!(state.redo());
    assert_eq!(state.components, pasted);
}

#[test]
fn array_preview_and_commit_reference_each_members_own_targets() {
    let mut state = coupled_selection();
    let count = SchematicArrayCount::new(3, 1).unwrap();
    let plan = SchematicArrayPlan::new(
        SchematicArrayKind::Linear,
        count,
        state.default_array_naming(count).unwrap(),
        SchematicArrayPlacement::Pitch(Point::new(0, 1000)),
    )
    .unwrap();
    let original = state.components.clone();
    let preview = state.preview_array_selection(&plan).unwrap();
    assert_eq!(state.components, original);
    for member in 1..3 {
        let named = |source| {
            component(
                &preview.components,
                &plan.naming.value_for_source(source, member).unwrap(),
            )
        };
        let first = named("L1");
        let second = named("L2");
        assert_eq!(parameter(first, "coupled_to"), second.spice_instance_name());
        assert_eq!(
            parameter(named("K1"), "inductors"),
            format!(
                "{}, {}",
                first.spice_instance_name(),
                second.spice_instance_name()
            )
        );
        assert_eq!(
            parameter(named("F1"), "vref"),
            named("V1").spice_instance_name()
        );
        assert_eq!(named("H1").params, "vref=V_external");
    }
    state.array_selection(&plan).unwrap();
    assert_eq!(&state.components[original.len()..], preview.components);
    assert!(state.undo());
    assert_eq!(state.components, original);
    assert!(!state.can_undo());
}

#[test]
fn only_structural_parameters_change_and_aliases_resolve_case_insensitively() {
    let mut state = coupled_selection();
    state
        .components
        .iter_mut()
        .find(|component| component.name == "L1")
        .unwrap()
        .name = "coil".to_owned();
    let coupling = state
        .components
        .iter_mut()
        .find(|component| component.name == "K1")
        .unwrap();
    coupling.params =
        r#"l1=coil l2=LCOIL l8=L2 note="coil L2" inductors="coil,L2,L_external""#.to_owned();
    let prepared = PreparedCopyReferences::new(&state.components).unwrap();
    let mut copies = state.components.clone();
    for copy in &mut copies {
        copy.name = format!("{}_copy", copy.spice_instance_name());
    }
    prepared.apply(&mut copies);
    let coupling = component(&copies, "K1_copy");
    assert_eq!(parameter(coupling, "l1"), "Lcoil_copy");
    assert_eq!(parameter(coupling, "l2"), "Lcoil_copy");
    assert_eq!(parameter(coupling, "l8"), "L2_copy");
    assert_eq!(
        parameter(coupling, "inductors"),
        "Lcoil_copy,L2_copy,L_external"
    );
    assert_eq!(parameter(coupling, "note"), "coil L2");
}

#[test]
fn rejected_copy_and_array_leave_the_document_history_and_allocators_unchanged() {
    for malformed in [true, false] {
        let mut state = coupled_selection();
        if malformed {
            state
                .components
                .iter_mut()
                .find(|component| component.name == "K1")
                .unwrap()
                .params = "inductors=\"L1 L2".to_owned();
        } else {
            state
                .components
                .iter_mut()
                .find(|component| component.name == "L2")
                .unwrap()
                .name = "l1".to_owned();
        }
        state.copy_selection();
        let mut before = state.clone();
        let error = state.paste_at_checked(Point::new(0, 1000)).unwrap_err();
        assert!(error.contains("K1"), "{error}");
        assert_eq!(state.components, before.components);
        assert_eq!(state.selection, before.selection);
        assert_eq!(state.topology_version(), before.topology_version());
        assert_eq!(state.is_dirty, before.is_dirty);
        assert!(!state.can_undo());
        if malformed {
            let count = SchematicArrayCount::new(2, 1).unwrap();
            let plan = SchematicArrayPlan::new(
                SchematicArrayKind::Linear,
                count,
                state.default_array_naming(count).unwrap(),
                SchematicArrayPlacement::Pitch(Point::new(0, 1000)),
            )
            .unwrap();
            assert!(
                state
                    .preview_array_selection(&plan)
                    .unwrap_err()
                    .to_string()
                    .contains("K1")
            );
            assert!(state.array_selection(&plan).is_err());
            assert_eq!(state.components, before.components);
            assert!(!state.can_undo());
        }
        assert_eq!(
            state.add_component(ComponentType::Inductor, Point::new(999, 999)),
            before.add_component(ComponentType::Inductor, Point::new(999, 999))
        );
        assert_eq!(state.components, before.components);
    }
}

//! What Bulk edit finds, what one apply writes, and how its dialog stands.
//!
//! An apply is checked whole before it touches anything and lands as one undo
//! step; restoring a value and leaving it unchanged are different writes; rows
//! another owner holds are shown but locked.

use super::*;
use crate::state::{LibraryCellInstance, Point};
#[cfg(test)]
use crate::workbench::design::test_host::DesignTestHost;

pub fn component(id: u64, name: &str, section: Option<&str>) -> Component {
    let mut component = Component::new(id, ComponentType::CellInstance, Point::new(id as i32, 0))
        .with_name_value(name, "OPA189");
    let mut binding = LibraryCellInstance::new("analog", "OPA189", "symbol");
    binding.model_section = section.map(str::to_owned);
    component.library_cell = Some(binding);
    component
}

#[test]
fn the_scope_line_names_the_occurrence_below_the_implicit_root() {
    let mut state = DesignTestHost::default();
    assert_eq!(active_hierarchy_path(&state.model), "/");

    state.model.workspace.descend_into(
        "XAFE".to_owned(),
        crate::state::CellViewRef::new("user", "afe_core", "schematic"),
        crate::state::ViewType::Schematic,
    );
    assert_eq!(active_hierarchy_path(&state.model), "/XAFE");

    state.model.workspace.descend_into(
        "XBIAS".to_owned(),
        crate::state::CellViewRef::new("user", "bias", "schematic"),
        crate::state::ViewType::Schematic,
    );
    assert_eq!(active_hierarchy_path(&state.model), "/XAFE/XBIAS");

    assert_eq!(
        schematic_display_path("analog/opamp/schematic"),
        "analog/opamp",
        "a buffer outside the occurrence is named as its library cell"
    );
}

#[test]
fn wildcard_filter_is_case_insensitive_and_anchored_when_requested() {
    assert!(wildcard_match("OPA*", "opa189/tt"));
    assert!(wildcard_match("*189*", "OPA189/tt"));
    assert!(!wildcard_match("OPA*", "xOPA189"));
    assert!(!wildcard_match("*189", "OPA189/tt"));
}

/// A pattern without `*` is the whole text, so `value = 1k` does not tick
/// 5.1k, 11k or 1k5; with `*`, a piece is found wherever it falls, so `R*0`
/// finds R100 though R's first 0 is not its last character.
#[test]
fn a_pattern_is_exact_unless_a_star_is_typed_and_a_star_finds_any_run() {
    assert!(wildcard_match("1k", "1K"));
    for other in ["5.1k", "11k", "1k5"] {
        assert!(!wildcard_match("1k", other), "{other}");
    }
    assert!(wildcard_match("R*0", "R100"));
    assert!(wildcard_match("1*0", "100"));
    assert!(wildcard_match("*0", "100"));
    assert!(wildcard_match("a*b*c", "abc"));
    assert!(wildcard_match("*", "anything"));
    assert!(!wildcard_match("R*0", "R101"));
    assert!(!wildcard_match("ab*ba", "aba"), "the ends do not overlap");

    let mut state = DesignTestHost::default();
    let mut parts = Vec::new();
    for (id, value) in [(1, "1k"), (2, "5.1k"), (3, "11k")] {
        let mut part = Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        part.name = format!("R{id}");
        part.value = value.to_owned();
        parts.push(part);
    }
    state.model.schematic.document_mut_for_test().components = parts;
    let filter = SelectionBulkFilter {
        current_property: "value = 1k".to_owned(),
        ..SelectionBulkFilter::default()
    };
    let rows = build_rows(&state.model, &filter, SelectionBulkProperty::Tolerance);
    assert_eq!(
        rows.iter()
            .map(|row| row.object.as_str())
            .collect::<Vec<_>>(),
        vec!["R1"]
    );
}

#[test]
fn free_text_query_searches_hidden_instance_parameters_and_binding_fields() {
    let mut state = DesignTestHost::default();
    let mut instance = component(1, "U1", Some("tt"));
    instance.params = "gain=100".to_owned();
    state.model.schematic.document_mut_for_test().components = vec![instance];

    let mut filter = SelectionBulkFilter {
        query: "gain=100".to_owned(),
        ..SelectionBulkFilter::default()
    };
    assert_eq!(
        build_rows(&state.model, &filter, SelectionBulkProperty::ModelSection).len(),
        1
    );

    filter.query = "analog".to_owned();
    assert_eq!(
        build_rows(&state.model, &filter, SelectionBulkProperty::ModelSection).len(),
        1
    );
}

#[test]
fn exact_mockup_default_section_filter_matches_an_inherited_binding() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![component(1, "U1", None)];
    let filter = SelectionBulkFilter {
        model_cell: "OPA189*".to_owned(),
        current_property: "section = default".to_owned(),
        ..SelectionBulkFilter::default()
    };

    let rows = build_rows(&state.model, &filter, SelectionBulkProperty::ModelSection);
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].object, "U1");
    assert!(rows[0].editable());
}

/// The dialog proposes nothing: it opens with no value and Apply says
/// what it is waiting for.
#[test]
fn bulk_edit_opens_with_no_value_and_apply_waits_for_one() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", None)];

    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );

    let dialog = &state.design.selection_bulk_edit;
    assert!(dialog.open);
    assert_eq!(dialog.new_value, "");
    assert!(missing_value_reason(dialog).is_some());
}

/// Bulk edit opened on three selected resistors ticks those three and no
/// other part the filter lists; a different property keeps the ticks; with
/// nothing selected nothing is ticked, so an Apply cannot reach parts the
/// reader never chose.
#[test]
fn bulk_edit_starts_from_the_selection() {
    let mut state = DesignTestHost::default();
    let mut parts = Vec::new();
    for id in 1..=5 {
        let mut part = Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        part.name = format!("R{id}");
        part.value = "1k".to_owned();
        parts.push(part);
    }
    state.model.schematic.document_mut_for_test().components = parts;
    for id in 1..=3 {
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(id);
    }
    let ticked = |state: &DesignTestHost| -> Vec<u64> {
        state
            .design
            .selection_bulk_edit
            .selected
            .iter()
            .map(|key| key.object_id)
            .collect()
    };

    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    assert_eq!(ticked(&state), vec![1, 2, 3]);

    state.design.selection_bulk_edit.property = SelectionBulkProperty::Tolerance;
    handle_selection_bulk_body_action(
        &mut state.design.selection_bulk_edit,
        &mut state.design.schematic_bulk_edit_filters,
        &state.model,
        SelectionBulkBodyAction::PropertyChanged,
    );
    assert_eq!(
        ticked(&state),
        vec![1, 2, 3],
        "a new property keeps the ticks"
    );

    state.model.schematic.session.editor.selection.clear();
    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    assert!(ticked(&state).is_empty());
}

/// The value is what is most often changed in bulk: every 10k resistor to
/// 12k. It is a property of its own and is written to the part's value,
/// and `value=` is refused as a parameter override, which would have added
/// a parameter named value to the card and left the value at 10k.
#[test]
fn bulk_edit_sets_the_value_of_the_ticked_parts() {
    let mut state = DesignTestHost::default();
    let mut parts = Vec::new();
    for (id, value) in [(1, "10k"), (2, "10k"), (3, "10k")] {
        let mut part = Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        part.name = format!("R{id}");
        part.value = value.to_owned();
        parts.push(part);
    }
    state.model.schematic.document_mut_for_test().components = parts;
    state.model.schematic.init_undo_history();
    for id in [1, 2] {
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(id);
    }
    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    assert_eq!(
        state.design.selection_bulk_edit.property,
        SelectionBulkProperty::Value
    );
    state.design.selection_bulk_edit.new_value = "12k".to_owned();

    commit_selection_bulk_edit(
        &mut state.design.selection_bulk_edit,
        &mut state.model,
        &mut state.workbench,
    )
    .expect("the value applies");

    let values: Vec<(&str, &str)> = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .map(|part| (part.value.as_str(), part.params.as_str()))
        .collect();
    assert_eq!(values, vec![("12k", ""), ("12k", ""), ("10k", "")]);
    let prompt = state
        .workbench
        .prompt
        .as_ref()
        .map(|prompt| prompt.text.clone())
        .unwrap_or_default();
    assert!(
        prompt.starts_with("Bulk edit changed 2 objects") && !prompt.contains("undo step"),
        "{prompt}"
    );
    assert!(
        validate_bulk_value(
            SelectionBulkProperty::ParameterOverride,
            "value=12k",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            None,
        )
        .is_err()
    );
}

/// Temperature reached every ticked part, so a source took `temp=` and the
/// netlist refused it. A part whose kind takes no temperature is locked for
/// it, as one without a tolerance is locked for Tolerance.
#[test]
fn temperature_reaches_only_parts_that_take_one() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)),
        Component::new(2, ComponentType::VoltageSource, Point::new(100, 0)),
    ];
    let rows = build_rows(
        &state.model,
        &SelectionBulkFilter::default(),
        SelectionBulkProperty::Temperature,
    );
    let editable: Vec<bool> = rows.iter().map(SelectionBulkRow::editable).collect();
    assert_eq!(editable, vec![true, false]);
}

/// The sections offered are the bound library's own, not a fixed list,
/// and a library that defines none says so.
#[test]
fn bulk_edit_offers_the_bound_librarys_sections() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", None)];

    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    assert!(state.design.selection_bulk_edit.bound_sections.is_empty());
    state.design.selection_bulk_edit.property = SelectionBulkProperty::ModelSection;
    assert_eq!(
        missing_value_reason(&state.design.selection_bulk_edit),
        Some(NO_BOUND_SECTIONS)
    );

    let mut library = crate::state::model_library::ModelLibrary::new("analog");
    for (name, default) in [("TT_PRE", true), ("mc_ss", false)] {
        library.corners.insert(
            name.to_owned(),
            rspice_model_library::ProcessCorner::from_composite_section(
                name,
                std::path::PathBuf::from("analog.lib"),
                default,
            ),
        );
    }
    state.model.model_library_manager.add_library(library);

    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    state.design.selection_bulk_edit.property = SelectionBulkProperty::ModelSection;
    let dialog = &state.design.selection_bulk_edit;
    assert_eq!(dialog.bound_sections, ["mc_ss", "TT_PRE"]);
    assert_eq!(
        missing_value_reason(dialog),
        Some("Choose a section to apply")
    );
    assert_eq!(
        validate_bulk_value(
            SelectionBulkProperty::ModelSection,
            "TT_PRE",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            None,
        ),
        Ok("TT_PRE".to_owned()),
        "a library's section is applied as it names it"
    );
}

/// A section is written to every ticked part, so the list holds only the
/// sections every ticked part's library defines. It offered the union, and
/// a section of one library was written into parts bound to another.
#[test]
fn bulk_edit_offers_only_sections_every_ticked_library_has() {
    let mut state = DesignTestHost::default();
    let mut second = component(2, "X2", None);
    second.library_cell.as_mut().expect("bound").library = "digital".to_owned();
    state.model.schematic.document_mut_for_test().components =
        vec![component(1, "X1", None), second];
    state.model.schematic.init_undo_history();
    for (name, sections) in [("analog", ["tt", "ff"]), ("digital", ["TT", "ss"])] {
        let mut library = crate::state::model_library::ModelLibrary::new(name);
        for (index, section) in sections.into_iter().enumerate() {
            library.corners.insert(
                section.to_owned(),
                rspice_model_library::ProcessCorner::from_composite_section(
                    section,
                    std::path::PathBuf::from(format!("{name}.lib")),
                    index == 0,
                ),
            );
        }
        state.model.model_library_manager.add_library(library);
    }
    for id in [1, 2] {
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .select_component(id);
    }
    let act = |state: &mut DesignTestHost, action: SelectionBulkBodyAction| {
        handle_selection_bulk_body_action(
            &mut state.design.selection_bulk_edit,
            &mut state.design.schematic_bulk_edit_filters,
            &state.model,
            action,
        );
    };
    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    state.design.selection_bulk_edit.property = SelectionBulkProperty::ModelSection;
    act(&mut state, SelectionBulkBodyAction::PropertyChanged);
    assert_eq!(state.design.selection_bulk_edit.bound_sections, ["tt"]);

    let second_row = state
        .design
        .selection_bulk_edit
        .selected
        .iter()
        .find(|key| key.object_id == 2)
        .cloned()
        .expect("X2 is ticked");
    state
        .design
        .selection_bulk_edit
        .selected
        .remove(&second_row);
    act(&mut state, SelectionBulkBodyAction::TicksChanged);
    assert_eq!(
        state.design.selection_bulk_edit.bound_sections,
        ["ff", "tt"],
        "with X2 unticked, X1's library alone decides"
    );
    state.design.selection_bulk_edit.new_value = "ff".to_owned();

    state
        .design
        .selection_bulk_edit
        .selected
        .insert(second_row.clone());
    act(&mut state, SelectionBulkBodyAction::TicksChanged);
    assert_eq!(state.design.selection_bulk_edit.bound_sections, ["tt"]);
    assert!(
        state.design.selection_bulk_edit.new_value.is_empty(),
        "a section X2's library lacks is let go when X2 is ticked"
    );

    state.design.selection_bulk_edit.new_value = "ff".to_owned();
    let before = state.model.schematic.document().components.clone();
    assert!(
        commit_selection_bulk_edit(
            &mut state.design.selection_bulk_edit,
            &mut state.model,
            &mut state.workbench,
        )
        .is_err(),
        "ff is never written into a part bound to a library without it"
    );
    assert_eq!(state.model.schematic.document().components, before);

    state.design.selection_bulk_edit.new_value = "tt".to_owned();
    commit_selection_bulk_edit(
        &mut state.design.selection_bulk_edit,
        &mut state.model,
        &mut state.workbench,
    )
    .expect("a section both libraries define applies");
    let sections: Vec<Option<&str>> = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .map(|part| {
            part.library_cell
                .as_ref()
                .and_then(|binding| binding.model_section.as_deref())
        })
        .collect();
    assert_eq!(sections, vec![Some("tt"), Some("tt")]);
}

#[test]
fn invalid_target_preflight_never_partially_mutates() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().components = vec![
        component(1, "X1", Some("tt")),
        component(2, "X2", Some("tt")),
    ];
    schematic.document_mut_for_test().components[1].library_cell = None;
    let before = schematic.document().components.clone();
    let ids = BTreeSet::from([1, 2]);

    assert!(
        apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::ModelSection,
            "ff",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            None,
        )
        .is_err()
    );
    assert_eq!(schematic.document().components, before);
    assert!(!schematic.can_undo());
}

#[test]
fn model_section_bulk_edit_is_one_undo_boundary() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().components = vec![
        component(1, "X1", Some("tt")),
        component(2, "X2", Some("tt")),
    ];
    let ids = BTreeSet::from([1, 2]);
    let receipt = apply_bulk_edit(
        &mut schematic,
        &ids,
        SelectionBulkProperty::ModelSection,
        "ff",
        SelectionBulkUnsetBehavior::SetExplicitValue,
        None,
    )
    .expect("valid bulk edit");
    assert_eq!(receipt.changed, 2);
    assert!(schematic.can_undo());
    assert_eq!(
        schematic.document().components[0]
            .library_cell
            .as_ref()
            .and_then(|binding| binding.model_section.as_deref()),
        Some("ff")
    );
    assert!(schematic.undo());
    assert_eq!(
        schematic.document().components[0]
            .library_cell
            .as_ref()
            .and_then(|binding| binding.model_section.as_deref()),
        Some("tt")
    );
    assert!(!schematic.can_undo());
}

#[test]
fn restore_and_leave_unchanged_have_distinct_semantics() {
    let mut schematic = SchematicState::default();
    let mut first = component(1, "X1", None);
    first.params = "temp=27".to_owned();
    let second = component(2, "X2", None);
    schematic.document_mut_for_test().components = vec![first, second];
    let ids = BTreeSet::from([1, 2]);

    let receipt = apply_bulk_edit(
        &mut schematic,
        &ids,
        SelectionBulkProperty::Temperature,
        "85",
        SelectionBulkUnsetBehavior::LeaveUnchanged,
        None,
    )
    .expect("leave unchanged");
    assert_eq!(receipt.changed, 1);
    assert_eq!(schematic.document().components[0].params, "temp=85");
    assert!(schematic.document().components[1].params.is_empty());

    let receipt = apply_bulk_edit(
        &mut schematic,
        &ids,
        SelectionBulkProperty::Temperature,
        "",
        SelectionBulkUnsetBehavior::RestoreInheritedValue,
        None,
    )
    .expect("restore");
    assert_eq!(receipt.changed, 1);
    assert!(schematic.document().components[0].params.is_empty());
}

#[test]
fn display_override_changes_durable_component_state() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().components = vec![component(1, "X1", None)];
    let ids = BTreeSet::from([1]);
    apply_bulk_edit(
        &mut schematic,
        &ids,
        SelectionBulkProperty::Display,
        "hidden",
        SelectionBulkUnsetBehavior::SetExplicitValue,
        None,
    )
    .expect("display edit");
    assert_eq!(
        schematic.document().components[0].display_mode,
        ComponentDisplayMode::Hidden
    );
}

/// A part number set across a selection lands on each part's BOM record,
/// and clearing it restores the blank; a distribution is stored as its
/// keyword, and only on parts a tolerance applies to.
#[test]
fn bom_and_distribution_bulk_edits_reach_the_parts_they_apply_to() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().components = vec![
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)).with_name_value("R1", "1k"),
        Component::new(2, ComponentType::Resistor, Point::new(1, 0)).with_name_value("R2", "1k"),
    ];
    let ids = BTreeSet::from([1, 2]);
    let edit = |schematic: &mut SchematicState, property, value: &str, unset| {
        apply_bulk_edit(schematic, &ids, property, value, unset, None).expect("bulk edit")
    };
    edit(
        &mut schematic,
        SelectionBulkProperty::Bom(BomField::Mpn),
        " RC0603FR-0710KL ",
        SelectionBulkUnsetBehavior::SetExplicitValue,
    );
    assert!(
        schematic
            .document()
            .components
            .iter()
            .all(|part| part.bom.mpn == "RC0603FR-0710KL" && part.params.is_empty())
    );
    edit(
        &mut schematic,
        SelectionBulkProperty::Bom(BomField::Mpn),
        "",
        SelectionBulkUnsetBehavior::RestoreInheritedValue,
    );
    assert!(
        schematic
            .document()
            .components
            .iter()
            .all(|part| part.bom.mpn.is_empty())
    );

    edit(
        &mut schematic,
        SelectionBulkProperty::ToleranceDistribution,
        "Uniform",
        SelectionBulkUnsetBehavior::SetExplicitValue,
    );
    assert!(
        schematic
            .document()
            .components
            .iter()
            .all(|part| part.params == "tol_dist=uniform")
    );

    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", None)];
    let rows = build_rows(
        &state.model,
        &SelectionBulkFilter::default(),
        SelectionBulkProperty::ToleranceDistribution,
    );
    assert_eq!(rows[0].lock.as_deref(), Some("Not applicable"));
}

#[test]
fn parameter_override_uses_current_property_key_and_restores_inheritance() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().components = vec![component(1, "X1", None)];
    let ids = BTreeSet::from([1]);
    apply_bulk_edit(
        &mut schematic,
        &ids,
        SelectionBulkProperty::ParameterOverride,
        "4",
        SelectionBulkUnsetBehavior::SetExplicitValue,
        Some("m=1"),
    )
    .expect("set parameter override");
    assert_eq!(schematic.document().components[0].params, "m=4");

    apply_bulk_edit(
        &mut schematic,
        &ids,
        SelectionBulkProperty::ParameterOverride,
        "",
        SelectionBulkUnsetBehavior::RestoreInheritedValue,
        Some("m=4"),
    )
    .expect("restore parameter inheritance");
    assert!(schematic.document().components[0].params.is_empty());
}

#[test]
fn invalid_numeric_value_leaves_document_and_undo_history_unchanged() {
    let mut schematic = SchematicState::default();
    schematic.document_mut_for_test().components = vec![component(1, "X1", None)];
    let before = schematic.document().components.clone();
    let ids = BTreeSet::from([1]);
    assert!(
        apply_bulk_edit(
            &mut schematic,
            &ids,
            SelectionBulkProperty::Temperature,
            "-300",
            SelectionBulkUnsetBehavior::SetExplicitValue,
            None,
        )
        .is_err()
    );
    assert_eq!(schematic.document().components, before);
    assert!(!schematic.can_undo());
}

#[test]
fn impact_preview_is_exact_without_cloning_an_undo_transaction() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![
        component(1, "X1", Some("tt")),
        component(2, "X2", Some("tt")),
    ];
    let rows = build_rows(
        &state.model,
        &SelectionBulkFilter::default(),
        SelectionBulkProperty::ModelSection,
    );
    let selected = rows.iter().map(|row| row.key.clone()).collect();

    let impact = preview_impact(
        &state.model.schematic,
        &rows,
        &selected,
        SelectionBulkProperty::ModelSection,
        "ff",
        SelectionBulkUnsetBehavior::SetExplicitValue,
        "",
    );
    assert_eq!(impact.changed, 2);
    assert_eq!(impact.locked_or_excluded, 0);
    assert!(impact.error.is_none());
    assert!(!state.model.schematic.can_undo());
    assert!(
        state
            .model
            .schematic
            .document()
            .components
            .iter()
            .all(|component| {
                component
                    .library_cell
                    .as_ref()
                    .and_then(|binding| binding.model_section.as_deref())
                    == Some("tt")
            })
    );
}

#[test]
fn authority_rejects_active_selection_and_external_buffer_drift() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", Some("tt"))];
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(1);
    state
        .model
        .workspace
        .insert_schematic_editor("user/child/schematic".to_owned(), SchematicState::default());
    let authority = SelectionBulkEditAuthority::capture(&state.model);
    assert!(authority.stale_reason(&state.model).is_none());

    state.model.schematic.session.editor.selection.clear();
    assert!(authority.stale_reason(&state.model).is_some());
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(1);
    state
        .model
        .workspace
        .schematic_editor_mut("user/child/schematic")
        .expect("external buffer")
        .editor
        .document_mut_for_test()
        .components
        .push(component(2, "X2", Some("tt")));
    assert!(authority.stale_reason(&state.model).is_some());
}

#[test]
fn complete_project_rows_are_visible_but_other_owners_are_locked() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", Some("tt"))];
    let mut external = SchematicState::default();
    external.document_mut_for_test().components = vec![component(2, "X2", Some("ff"))];
    state
        .model
        .workspace
        .insert_schematic_editor("user/child/schematic".to_owned(), external);
    let filter = SelectionBulkFilter {
        hierarchy_scope: SelectionBulkHierarchyScope::CompleteProject,
        ..SelectionBulkFilter::default()
    };
    let rows = build_rows(&state.model, &filter, SelectionBulkProperty::ModelSection);

    assert_eq!(rows.len(), 2);
    assert_eq!(rows.iter().filter(|row| row.editable()).count(), 1);
    assert!(
        rows.iter()
            .any(|row| row.lock.as_deref() == Some("Other owner"))
    );
}

#[test]
fn port_rows_only_accept_the_display_property() {
    let mut state = DesignTestHost::default();
    state.model.schematic.document_mut_for_test().components =
        vec![Component::new(1, ComponentType::Port, Point::new(0, 0)).with_name_value("VIN", "")];
    let filter = SelectionBulkFilter {
        object_kind: SelectionBulkObjectKind::PortsAndPins,
        ..SelectionBulkFilter::default()
    };

    let temperature_rows = build_rows(&state.model, &filter, SelectionBulkProperty::Temperature);
    assert_eq!(temperature_rows[0].lock.as_deref(), Some("Not applicable"));
    let display_rows = build_rows(&state.model, &filter, SelectionBulkProperty::Display);
    assert!(display_rows[0].editable());
}

/// Nets, labels, notes and shapes were offered as object kinds though no
/// property reached them: every row was locked and Apply never acted on
/// one. Every kind the filter offers now lists something a property edits.
#[test]
fn every_object_kind_offered_lists_rows_a_property_edits() {
    let mut state = DesignTestHost::default();
    let mut resistor = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
    resistor.name = "R1".to_owned();
    resistor.value = "1k".to_owned();
    state.model.schematic.document_mut_for_test().components = vec![
        resistor,
        Component::new(2, ComponentType::Port, Point::new(200, 0)).with_name_value("VIN", ""),
    ];
    for kind in SelectionBulkObjectKind::ALL {
        let filter = SelectionBulkFilter {
            object_kind: kind,
            ..SelectionBulkFilter::default()
        };
        assert!(
            BULK_PROPERTIES.into_iter().any(|property| {
                build_rows(&state.model, &filter, property)
                    .iter()
                    .any(SelectionBulkRow::editable)
            }),
            "{} lists nothing any property can edit",
            kind.label()
        );
    }
}

/// A manufacturer or a display mode changes no analysis, so neither the
/// impact pane nor the status line counts analyses for it. Both reported
/// every enabled analysis as affected whatever the property.
#[test]
fn a_bom_edit_reports_no_analyses_affected() {
    let mut state = DesignTestHost::default();
    let mut part = Component::new(1, ComponentType::Resistor, Point::new(0, 0));
    part.name = "R1".to_owned();
    part.value = "1k".to_owned();
    state.model.schematic.document_mut_for_test().components = vec![part];
    state.model.schematic.init_undo_history();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(1);
    state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("the test plan is stable")
        .insert(crate::simulation::plan::AnalysisKind::OperatingPoint)
        .expect("an operating point inserts");
    assert!(state.model.sim_setup.enabled_analysis_instance_count() > 0);
    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    state.design.selection_bulk_edit.property = SelectionBulkProperty::Bom(BomField::Manufacturer);
    state.design.selection_bulk_edit.new_value = "Vishay".to_owned();
    assert!(!changes_the_circuit(
        state.design.selection_bulk_edit.property
    ));

    commit_selection_bulk_edit(
        &mut state.design.selection_bulk_edit,
        &mut state.model,
        &mut state.workbench,
    )
    .expect("the manufacturer applies");
    assert_eq!(
        state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| prompt.text.as_str()),
        Some("Bulk edit changed 1 object.")
    );
    assert!(changes_the_circuit(SelectionBulkProperty::Value));
}

/// Two parts bound to libraries with different sections, nothing selected,
/// and the dialog open.
fn two_libraries() -> DesignTestHost {
    let mut state = DesignTestHost::default();
    let mut second = component(2, "X2", None);
    second.library_cell.as_mut().expect("bound").library = "digital".to_owned();
    state.model.schematic.document_mut_for_test().components =
        vec![component(1, "X1", None), second];
    state.model.schematic.init_undo_history();
    for (name, sections) in [("analog", ["tt", "ff"]), ("digital", ["TT", "ss"])] {
        let mut library = crate::state::model_library::ModelLibrary::new(name);
        for (index, section) in sections.into_iter().enumerate() {
            library.corners.insert(
                section.to_owned(),
                rspice_model_library::ProcessCorner::from_composite_section(
                    section,
                    std::path::PathBuf::from(format!("{name}.lib")),
                    index == 0,
                ),
            );
        }
        state.model.model_library_manager.add_library(library);
    }
    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    state
}

fn act(state: &mut DesignTestHost, action: SelectionBulkBodyAction) {
    handle_selection_bulk_body_action(
        &mut state.design.selection_bulk_edit,
        &mut state.design.schematic_bulk_edit_filters,
        &state.model,
        action,
    );
}

/// Loading a saved filter lists the sections of what it finds, as typing
/// the same filter does. The list kept the sections of the rows found
/// before, until a filter field was edited.
#[test]
fn loading_a_saved_filter_lists_the_sections_of_what_it_finds() {
    let mut state = two_libraries();
    state.design.selection_bulk_edit.property = SelectionBulkProperty::ModelSection;
    act(&mut state, SelectionBulkBodyAction::PropertyChanged);
    assert_eq!(state.design.selection_bulk_edit.bound_sections, ["tt"]);

    state.design.selection_bulk_edit.filter.query = "X2".to_owned();
    act(&mut state, SelectionBulkBodyAction::FilterChanged);
    state.design.selection_bulk_edit.save_filter_name = "Digital".to_owned();
    act(&mut state, SelectionBulkBodyAction::SaveFilter);
    state.design.selection_bulk_edit.filter.query.clear();
    act(&mut state, SelectionBulkBodyAction::FilterChanged);
    assert_eq!(state.design.selection_bulk_edit.bound_sections, ["tt"]);

    state.design.selection_bulk_edit.selected_saved_filter = Some("Digital".to_owned());
    act(&mut state, SelectionBulkBodyAction::LoadFilter);
    assert_eq!(state.design.selection_bulk_edit.filter.query, "X2");
    assert_eq!(
        state.design.selection_bulk_edit.bound_sections,
        ["ss", "TT"],
        "the loaded filter finds X2 alone, so its library's sections are offered"
    );
}

/// A filter name that cannot be saved is refused under the name, and the
/// dialog's band, which reports a failed Apply as "Nothing was changed",
/// stays shut: saving a filter changes nothing in the design either way.
#[test]
fn a_refused_filter_name_is_explained_under_the_name() {
    let mut state = two_libraries();
    state.design.selection_bulk_edit.save_filter_open = true;
    state.design.selection_bulk_edit.save_filter_name = "two\nlines".to_owned();
    act(&mut state, SelectionBulkBodyAction::SaveFilter);
    assert!(state.design.selection_bulk_edit.error.is_none());

    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut texts = Vec::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| {
                render_selection_bulk_edit_dialog(
                    &mut state.design.selection_bulk_edit,
                    &mut state.design.schematic_bulk_edit_filters,
                    &mut state.model,
                    &mut state.workbench,
                    ctx,
                );
            },
        );
        texts.extend(
            output
                .platform_output
                .accesskit_update
                .expect("bulk-edit access tree")
                .nodes
                .into_iter()
                .filter_map(|(_, node)| node.label().or_else(|| node.value()).map(str::to_owned)),
        );
    }
    assert!(
        texts
            .iter()
            .any(|text| text == "A saved filter name is one line of text."),
        "the refusal is shown: {texts:?}"
    );
    assert!(
        !texts
            .iter()
            .any(|text| text.contains("Nothing was changed")),
        "the Apply band stays shut"
    );
    assert!(state.design.schematic_bulk_edit_filters.saved().is_empty());

    state.design.selection_bulk_edit.save_filter_name = "Passives".to_owned();
    act(&mut state, SelectionBulkBodyAction::SaveFilter);
    assert_eq!(state.design.schematic_bulk_edit_filters.saved().len(), 1);
}

/// The Display list opens on how the first ticked part is lettered, so the
/// value it edits is in view; it opened blank, and Apply waited on a choice
/// even to keep a mode.
#[test]
fn the_display_list_opens_on_the_current_value() {
    let mut state = two_libraries();
    state.model.schematic.document_mut_for_test().components[1].display_mode =
        ComponentDisplayMode::NameOnly;
    state.design.selection_bulk_edit.filter.query = "X2".to_owned();
    act(&mut state, SelectionBulkBodyAction::FilterChanged);
    state.design.selection_bulk_edit.property = SelectionBulkProperty::Display;
    act(&mut state, SelectionBulkBodyAction::PropertyChanged);
    assert_eq!(state.design.selection_bulk_edit.new_value, "name only");
    assert_eq!(
        missing_value_reason(&state.design.selection_bulk_edit),
        None
    );

    state.design.selection_bulk_edit.selected.clear();
    act(&mut state, SelectionBulkBodyAction::PropertyChanged);
    assert_eq!(
        state.design.selection_bulk_edit.new_value, "inherit",
        "with nothing ticked the list opens on Inherit"
    );
}

//! What Bulk edit finds, what one apply writes, and how its dialog stands.
//!
//! An apply is checked whole before it touches anything and lands as one undo
//! step; restoring a value and leaving it unchanged are different writes; rows
//! another owner holds are shown but locked.

use super::*;
use crate::state::{LibraryCellInstance, Point};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

/// Draw the dialog as its dialog step does.
fn render(app: &mut RSpiceApp, ctx: &Context) {
    let state = &mut app.state;
    render_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &mut state.design.schematic_bulk_edit_filters,
        &mut state.model,
        &mut state.workbench,
        ctx,
    );
}

fn component(id: u64, name: &str, section: Option<&str>) -> Component {
    let mut component = Component::new(id, ComponentType::CellInstance, Point::new(id as i32, 0))
        .with_name_value(name, "OPA189");
    let mut binding = LibraryCellInstance::new("analog", "OPA189", "symbol");
    binding.model_section = section.map(str::to_owned);
    component.library_cell = Some(binding);
    component
}

/// The four result columns and the select box fit the pane they are laid
/// in. They were fixed widths that summed past it, so the last column
/// painted over the impact panel beside it.
#[test]
fn the_result_columns_fit_inside_their_pane() {
    // A 28 pt select box and a 14 pt scroll bar.
    let fixed = 42.0;
    for pane in [240.0_f32, 300.0, 360.0, 520.0, 900.0] {
        let columns = result_columns(pane, fixed);
        let used = fixed + RESULT_COLUMN_GAP * 4.0 + columns.iter().sum::<f32>();
        assert!(
            used <= pane.max(fixed + RESULT_COLUMN_GAP * 4.0 + 140.0),
            "{pane} pt pane lays out {used} pt of columns"
        );
    }
}

#[test]
fn the_scope_line_names_the_occurrence_below_the_implicit_root() {
    let mut state = AppState::default();
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

#[test]
fn free_text_query_searches_hidden_instance_parameters_and_binding_fields() {
    let mut state = AppState::default();
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
    let mut state = AppState::default();
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
    let mut state = AppState::default();
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

/// The sections offered are the bound library's own, not a fixed list,
/// and a library that defines none says so.
#[test]
fn bulk_edit_offers_the_bound_librarys_sections() {
    let mut state = AppState::default();
    state.model.schematic.document_mut_for_test().components = vec![component(1, "X1", None)];

    open_selection_bulk_edit_dialog(
        &mut state.design.selection_bulk_edit,
        &state.design.schematic_bulk_edit_filters,
        &state.model,
    );
    assert!(state.design.selection_bulk_edit.bound_sections.is_empty());
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

    let mut state = AppState::default();
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
    let mut state = AppState::default();
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
    let mut state = AppState::default();
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
    let mut state = AppState::default();
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
    let mut state = AppState::default();
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

#[test]
fn accessibility_tree_exposes_only_the_mockup_bulk_edit_surface_actions() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.model.schematic.document_mut_for_test().components =
        vec![component(1, "X1", Some("tt"))];
    open_selection_bulk_edit_dialog(
        &mut app.state.design.selection_bulk_edit,
        &app.state.design.schematic_bulk_edit_filters,
        &app.state.model,
    );

    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 900.0),
            )),
            ..egui::RawInput::default()
        },
        |ctx| render(&mut app, ctx),
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("bulk-edit access tree")
        .nodes;
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Dialog && node.label() == Some(TITLE)
    }));
    for label in [PRIMARY, "Cancel", "Save filter\u{2026}"] {
        assert!(
            nodes.iter().any(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            }),
            "missing action {label}"
        );
    }
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::CheckBox
            && node.label() == Some("Select X1")
            && node.toggled() == Some(egui::accesskit::Toggled::True)
    }));
}

/// The filter's text fields stand as tall as the selects above them and
/// are named by their labels, on desktop and on touch. They were 21 pt
/// beside 28 pt selects (44 on touch), and a screen reader heard nothing.
#[test]
fn the_filter_fields_match_the_selects_and_say_their_names() {
    for coarse in [false, true] {
        let ctx = Context::default();
        ctx.enable_accesskit();
        crate::ui::Theme::default().apply(&ctx);
        crate::ui::Theme::default().apply_responsive_metrics_with_target(
            &ctx,
            coarse.then_some(crate::ui::tokens::TOUCH_TARGET),
        );
        let mut app = RSpiceApp::test_instance();
        app.state.model.schematic.document_mut_for_test().components =
            vec![component(1, "X1", Some("tt"))];
        open_selection_bulk_edit_dialog(
            &mut app.state.design.selection_bulk_edit,
            &app.state.design.schematic_bulk_edit_filters,
            &app.state.model,
        );
        let mut nodes = std::collections::HashMap::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1440.0, 900.0),
                    )),
                    ..egui::RawInput::default()
                },
                |ctx| render(&mut app, ctx),
            );
            if let Some(update) = output.platform_output.accesskit_update {
                nodes.extend(update.nodes);
            }
        }
        let height = |label: &str| {
            let bounds = nodes
                .values()
                .find(|node| {
                    node.label() == Some(label) && node.role() != egui::accesskit::Role::Label
                })
                .and_then(egui::accesskit::Node::bounds)
                .unwrap_or_else(|| panic!("no control named {label:?}"));
            bounds.y1 - bounds.y0
        };
        let select = height("Object kinds");
        for field in ["Model / cell", "Current property"] {
            assert!(
                (height(field) - select).abs() < 0.5,
                "{field} is {} pt beside a {select} pt select (touch {coarse})",
                height(field)
            );
        }
    }
}

/// Nothing in the body reaches past the footer's right edge. With no
/// section to offer, New value used to state why in a line wider than
/// its column; the body grew about 90 pt past the surface, every row
/// under it took the new width, and the split rule moved with it.
#[test]
fn the_body_stays_inside_the_dialog_when_no_section_is_defined() {
    let ctx = Context::default();
    ctx.enable_accesskit();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    app.state.model.schematic.document_mut_for_test().components =
        vec![component(1, "X1", Some("tt"))];
    open_selection_bulk_edit_dialog(
        &mut app.state.design.selection_bulk_edit,
        &app.state.design.schematic_bulk_edit_filters,
        &app.state.model,
    );
    assert!(
        app.state
            .design
            .selection_bulk_edit
            .bound_sections
            .is_empty()
    );

    // Access updates are incremental: keep the latest of every node.
    let mut nodes = std::collections::HashMap::new();
    for _ in 0..4 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..egui::RawInput::default()
            },
            |ctx| render(&mut app, ctx),
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
    }
    let apply = nodes
        .values()
        .find(|node| node.role() == egui::accesskit::Role::Button && node.label() == Some(PRIMARY))
        .and_then(egui::accesskit::Node::bounds)
        .expect("Apply has bounds");
    let footer_right = apply.x1 + 12.0;
    for node in nodes.values() {
        let Some(bounds) = node.bounds() else {
            continue;
        };
        if matches!(
            node.role(),
            egui::accesskit::Role::ComboBox
                | egui::accesskit::Role::TextInput
                | egui::accesskit::Role::CheckBox
                | egui::accesskit::Role::Button
        ) {
            assert!(
                bounds.x1 <= footer_right + 0.5,
                "{:?} {:?} ends at {} past the dialog's {footer_right}",
                node.role(),
                node.label(),
                bounds.x1,
            );
        }
    }
    // Every choice is the kit's select, named for what it picks: the
    // raw combo boxes it replaced had no name to announce.
    for select in [
        "Object kinds",
        "Hierarchy scope",
        "Property",
        "Unset behavior",
        "Model section",
    ] {
        assert!(
            nodes.values().any(|node| {
                node.role() == egui::accesskit::Role::ComboBox && node.label() == Some(select)
            }),
            "no select is named {select:?}"
        );
    }
}

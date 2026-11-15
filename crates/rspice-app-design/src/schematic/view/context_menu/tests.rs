//! What the schematic selection context menu promises, and to whom.
//!
//! The catalog, its availability predicates, the keyboard contract, and the
//! destination each row hands off to. Split from `context_menu.rs` because a
//! command contract and the evidence that it holds are separate concerns.

use super::*;
use crate::schematic::view::schematic_symbol_context;
use crate::workbench::design::test_host::{DesignTestHost, canvas_facts, canvas_parts};
use egui::{Context, Rect, pos2, vec2};

use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType, Junction,
    LibraryCellInstance, NetLabel, Wire,
};

// The menu answers over the canvas's facts and acts over its parts. These
// lend both from a test's state under the menu's own names, and answer what
// the menu asked of another workspace as the shell does.

fn action_availability(action: ContextAction, state: &DesignTestHost) -> (bool, &'static str) {
    super::action_availability(action, &canvas_facts(state))
}

fn menu_entries(state: &DesignTestHost, target: ContextTarget) -> Vec<ContextEntry> {
    super::menu_entries(&canvas_facts(state), target)
}

fn menu_summary(state: &DesignTestHost, target: ContextTarget) -> Option<String> {
    super::menu_summary(&canvas_facts(state), target)
}

fn selection_summary(state: &DesignTestHost, target: ContextTarget) -> String {
    super::selection_summary(&canvas_facts(state), target)
}

fn select_pointer_target(
    state: &mut DesignTestHost,
    hit: PointerHit,
    hit_radius: i32,
    symbol_context: &SchematicSymbolContext,
    ctx: &Context,
    viewport: &Viewport,
    pointer_pos: egui::Pos2,
) -> ContextTarget {
    super::select_pointer_target(
        &mut canvas_parts(state),
        hit,
        hit_radius,
        symbol_context,
        ctx,
        viewport,
        pointer_pos,
    )
}

fn pointer_viewport() -> Viewport {
    Viewport {
        offset: egui::Pos2::ZERO,
        zoom: 1.0,
        bounds: Rect::from_min_size(egui::Pos2::ZERO, vec2(400.0, 400.0)),
    }
}

fn labels_of(entries: &[ContextEntry]) -> Vec<&'static str> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            ContextEntry::Command(command) => Some(command.label),
            ContextEntry::Separator => None,
        })
        .collect()
}

fn separators_in(entries: &[ContextEntry]) -> usize {
    entries
        .iter()
        .filter(|entry| matches!(entry, ContextEntry::Separator))
        .count()
}

/// One placed sine source, selected, in a project whose library holds
/// `sensor_drive` at the given revision-bumping edits.
fn state_with_selected_source(library_edits: usize, adopt: bool) -> DesignTestHost {
    use crate::state::stimulus_library::definition::StimulusDefinition;

    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let mut definition = StimulusDefinition::new(
        "sensor_drive",
        ComponentType::VoltageSourceSin,
        crate::state::stimulus_library::now_unix_ms,
    )
    .expect("a definition");
    definition.value = "0".to_owned();
    definition.params = "va=1 freq=1k".to_owned();
    let mut source = Component::new(7, ComponentType::VoltageSourceSin, Point::origin());
    source.name = "V1".to_owned();
    if adopt {
        definition.adopt_onto(&mut source).expect("adopt");
    }
    state
        .model
        .workspace
        .content
        .stimulus_library
        .insert(definition)
        .expect("insert");
    for edit in 0..library_edits {
        let held = state
            .model
            .workspace
            .content
            .stimulus_library
            .get("sensor_drive")
            .expect("held")
            .clone();
        let mut draft = crate::state::stimulus_library::draft::DefinitionDraft::new(held);
        draft.edit(|working| working.params = format!("va=1 freq={}k", edit + 2));
        state
            .model
            .workspace
            .content
            .stimulus_library
            .apply(&mut draft, crate::state::stimulus_library::now_unix_ms);
    }
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(source);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(7);
    state
}

/// One Verilog-A instance of `behavioral/gain/veriloga`, whose view owns its
/// module's source, selected.
fn state_with_selected_veriloga_instance() -> (DesignTestHost, u64, crate::state::CellViewRef) {
    let mut state = DesignTestHost::default();
    let (id, reference) = select_a_veriloga_instance(&mut state);
    (state, id, reference)
}

/// Place one Verilog-A instance of `behavioral/gain/veriloga` on `state`'s
/// otherwise empty schematic and select it.
pub(crate) fn select_a_veriloga_instance(
    state: &mut DesignTestHost,
) -> (u64, crate::state::CellViewRef) {
    use crate::state::{
        Cell, CellViewRef, Library, PortDirection, PortSpec, ProjectSourceBundle,
        ProjectSourceLanguage, ProjectSourceOwner, View, ViewType,
    };

    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let reference = CellViewRef::new("behavioral", "gain", "veriloga");
    let mut view = View::new("veriloga", ViewType::VerilogA);
    view.metadata
        .insert("veriloga.module".to_owned(), "gain".to_owned());
    view.metadata
        .insert("veriloga.ports".to_owned(), r#"["p","n"]"#.to_owned());
    let mut cell = Cell::new("gain");
    cell.add_view(view);
    let mut library = Library::new("behavioral");
    library.add_cell(cell);
    state.model.library_manager.add_library(library);
    state
        .model
        .workspace
        .content
        .project_sources
        .insert_bundle(
            ProjectSourceBundle::try_new(
                ProjectSourceOwner::cell_view(reference.clone()),
                ProjectSourceLanguage::VerilogA,
                "gain.va",
                "module gain(p, n);\n  inout p, n;\n  electrical p, n;\nendmodule\n",
                [],
                [],
            )
            .expect("a cell-view bundle"),
        )
        .expect("one bundle per cell view");
    let mut binding = LibraryCellInstance::new("behavioral", "gain", "veriloga");
    binding.bind_interface(&[
        PortSpec {
            name: "p".to_owned(),
            direction: PortDirection::InOut,
        },
        PortSpec {
            name: "n".to_owned(),
            direction: PortDirection::InOut,
        },
    ]);
    let id = state
        .model
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    (id, reference)
}

fn catalog_of(entries: &[ContextEntry]) -> Vec<(&'static str, Option<CommandKey>)> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            ContextEntry::Command(command) => Some((command.label, command.shortcut_command)),
            ContextEntry::Separator => None,
        })
        .collect()
}

/// Each target's menu, row for row and rule for rule. Only the
/// instance-master slot differs: an instance's four master rows, a Verilog-A
/// instance's "Open Verilog-A view" in place of Descend, and the empty
/// canvas's one verb, "New Verilog-A device…", in place of all four, with
/// the zoom box after Fit and the sheet's own symbol after Create hierarchy.
/// The row and rule counts are the ones
/// `rspice-schematic-editor`'s geometry test measures against the 635 px
/// ceiling: 20 + 4 (625 px), 19 + 4 (598 px).
#[test]
fn each_target_gets_the_catalog_its_instance_master_slot_calls_for() {
    let head = [
        (
            "Object properties…",
            Some(DesignCommand::ObjectProperties.key()),
        ),
        ("Rotate 90°", Some(DesignCommand::RotateSelection.key())),
        (
            "Mirror",
            Some(DesignCommand::MirrorSelectionHorizontal.key()),
        ),
        (
            "Mirror vertically",
            Some(DesignCommand::MirrorSelectionVertical.key()),
        ),
        // G-12: the Arrange page, which a phone reaches only from here.
        ("Arrange", None),
        ("Cut selection", Some(ids::CUT)),
        ("Copy selection", Some(ids::COPY)),
        ("Paste here", Some(ids::PASTE)),
        ("Duplicate", Some(ids::DUPLICATE)),
        ("Delete", Some(ids::DELETE)),
    ];
    let tail = [
        (
            "Create hierarchy from selection…",
            Some(DesignCommand::CreateHierarchy.key()),
        ),
        ("Sheet properties", Some(ids::SHEET_PROPERTIES)),
        (
            "Fit schematic content",
            Some(DesignCommand::FitSchematicContent.key()),
        ),
        ("Show in netlist", Some(ids::SHOW_IN_NETLIST)),
        (
            "Add voltage or current probe…",
            Some(DesignCommand::PlaceProbe.key()),
        ),
        ("Open operating point", Some(ids::RESULT_OPERATING_POINT)),
    ];
    let menu = |slot: &[(&'static str, Option<CommandKey>)]| {
        head.iter()
            .chain(slot)
            .chain(tail.iter())
            .copied()
            .collect::<Vec<_>>()
    };
    let master_rows = [
        (
            "Edit symbol in place",
            Some(DesignCommand::EditSymbolInPlace.key()),
        ),
        (
            "Update instance interface",
            Some(DesignCommand::UpdateInstanceInterface.key()),
        ),
        (
            "Replace instance…",
            Some(DesignCommand::ReplaceInstance.key()),
        ),
    ];
    let descend = (
        "Descend into selected instance",
        Some(DesignCommand::DescendHierarchyDirect.key()),
    );
    let open_veriloga = (
        "Open Verilog-A view",
        Some(DesignCommand::DescendHierarchyDirect.key()),
    );

    // The bare canvas offers what can be started on a sheet and none of the
    // selection's verbs; the zoom box is here alone because a finger has no Z.
    let state = DesignTestHost::default();
    let canvas = menu_entries(&state, ContextTarget::Canvas);
    assert_eq!(
        catalog_of(&canvas),
        vec![
            ("Place instance…", Some(DesignCommand::PlaceInstance.key())),
            ("Draw wire", Some(DesignCommand::PlaceWire.key())),
            ("Place net label", Some(DesignCommand::PlaceLabel.key())),
            ("Place text or note…", Some(DesignCommand::PlaceText.key())),
            ("New Verilog-A device…", Some(ids::NEW_VERILOGA_DEVICE)),
            ("Create symbol from schematic ports…", None),
            ("Sheet properties", Some(ids::SHEET_PROPERTIES)),
            (
                "Fit schematic content",
                Some(DesignCommand::FitSchematicContent.key())
            ),
            ("Zoom box", Some(ids::ZOOM_BOX)),
            (
                "Add voltage or current probe…",
                Some(DesignCommand::PlaceProbe.key())
            ),
            ("Open operating point", Some(ids::RESULT_OPERATING_POINT)),
        ],
    );
    // Something selected, then a click on the bare sheet: the selection's
    // verbs are back, and only the bare canvas offers the creation verbs.
    let mut selected = state_with_selected_source(0, false);
    let selected_canvas = labels_of(&menu_entries(&selected, ContextTarget::Canvas));
    assert!(selected_canvas.contains(&"Cut selection"));
    assert!(!selected_canvas.contains(&"Draw wire"));
    selected.model.schematic.session.editor.selection = Default::default();
    assert!(labels_of(&menu_entries(&selected, ContextTarget::Canvas)).contains(&"Draw wire"));
    assert_eq!((catalog_of(&canvas).len(), separators_in(&canvas)), (11, 3));

    // A schematic instance, and a wire: the master rows, Descend first.
    let mut resistor = DesignTestHost::default();
    resistor
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let id = resistor
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 0));
    for target in [ContextTarget::Component(id), ContextTarget::Wire(17)] {
        let entries = menu_entries(&resistor, target);
        assert_eq!(
            catalog_of(&entries),
            menu(&[descend, master_rows[0], master_rows[1], master_rows[2]]),
            "{target:?}"
        );
        assert_eq!(
            (catalog_of(&entries).len(), separators_in(&entries)),
            (20, 4)
        );
    }

    // A Verilog-A instance: its Descend row opens the module's view.
    let (veriloga, id, _) = state_with_selected_veriloga_instance();
    let entries = menu_entries(&veriloga, ContextTarget::Component(id));
    assert_eq!(
        catalog_of(&entries),
        menu(&[
            open_veriloga,
            master_rows[0],
            master_rows[1],
            master_rows[2]
        ])
    );
    assert_eq!(
        (catalog_of(&entries).len(), separators_in(&entries)),
        (20, 4)
    );
}

/// A placed source's menu states the stimulus-library verbs its standing calls
/// for, in a group of their own, in place of the three rows about a cell
/// instance's master.
#[test]
fn a_placed_source_gets_the_stimulus_verbs_its_standing_calls_for() {
    let group = |state: &DesignTestHost| -> Vec<&'static str> {
        labels_of(&menu_entries(state, ContextTarget::Component(7)))
            .into_iter()
            .filter(|label| label.contains("timulus") || label.contains("Re-adopt"))
            .collect()
    };

    // Drawn on the sheet: adopt one, or publish this one.
    let local = state_with_selected_source(0, false);
    assert_eq!(
        group(&local),
        vec!["Adopt stimulus definition…", "Save as stimulus definition…"]
    );
    // Adopted and current: the definition can also be opened.
    let adopted = state_with_selected_source(0, true);
    assert_eq!(
        group(&adopted),
        vec![
            "Adopt stimulus definition…",
            "Save as stimulus definition…",
            "Open in Stimulus Library",
        ]
    );
    // The library has published past the copy: re-adoption takes adopt's row.
    let behind = state_with_selected_source(1, true);
    assert_eq!(
        group(&behind),
        vec![
            "Re-adopt library revision",
            "Save as stimulus definition…",
            "Open in Stimulus Library",
        ]
    );

    for state in [&local, &adopted, &behind] {
        // The source is the menu's subject whether it was right-clicked or
        // is what the canvas menu acts on.
        for target in [ContextTarget::Component(7), ContextTarget::Canvas] {
            let entries = menu_entries(state, target);
            assert!(labels_of(&entries).len() <= 20, "{target:?}");
            assert_eq!(separators_in(&entries), 5, "{target:?}");
        }
        let entries = menu_entries(state, ContextTarget::Component(7));
        let labels = labels_of(&entries);
        for master_row in [
            "Descend into selected instance",
            "Open Verilog-A view",
            "New Verilog-A device…",
            "Update instance interface",
            "Replace instance…",
        ] {
            assert!(!labels.contains(&master_row), "{labels:?}");
        }
        assert_eq!(separators_in(&entries), 5);
        assert!(labels.len() <= 20);
        assert!(
            !matches!(entries.first(), Some(ContextEntry::Separator))
                && !matches!(entries.last(), Some(ContextEntry::Separator))
        );
    }
}

/// The verbs are the editor's: the same entry points, so the same outcomes.
#[test]
fn a_stimulus_verb_goes_where_component_properties_sends_it() {
    let mut behind = state_with_selected_source(1, true);
    assert!(action_availability(ContextAction::ReadoptStimulus, &behind).0);
    stimulus::execute(
        ContextAction::ReadoptStimulus,
        &mut canvas_parts(&mut behind),
    );
    let source = &behind.model.schematic.document().components[0];
    assert!(source.params.contains("freq=2k"), "{}", source.params);
    assert_eq!(
        behind
            .model
            .workspace
            .content
            .stimulus_library
            .provenance_state(source)
            .label(),
        "adopted · r2"
    );

    let mut local = state_with_selected_source(0, false);
    stimulus::execute(ContextAction::SaveStimulus, &mut canvas_parts(&mut local));
    assert!(local.design.stimulus_link.open);

    let mut adopted = state_with_selected_source(0, true);
    stimulus::execute(
        ContextAction::OpenStimulusDefinition,
        &mut canvas_parts(&mut adopted),
    );
    assert_eq!(
        adopted.design.selected_stimulus_definition.as_deref(),
        Some("sensor_drive")
    );

    // With nothing in the library there is nothing to adopt, and the row says
    // what to do about it rather than opening a dialog that refuses.
    let mut empty = state_with_selected_source(0, false);
    empty.model.workspace.content.stimulus_library = crate::state::StimulusLibrary::default();
    let (enabled, reason) = action_availability(ContextAction::AdoptStimulus, &empty);
    assert!(!enabled);
    assert!(reason.contains("save this source"), "{reason}");
}

#[test]
fn a_bare_canvas_menu_names_nothing_and_offers_paste_from_a_filled_clipboard() {
    let mut state = state_with_selected_source(0, false);
    assert!(
        menu_summary(&state, ContextTarget::Canvas).is_some(),
        "a menu on a selection keeps its header"
    );

    state.model.schematic.session.editor.selection = Default::default();
    assert_eq!(menu_summary(&state, ContextTarget::Canvas), None);
    assert!(!labels_of(&menu_entries(&state, ContextTarget::Canvas)).contains(&"Paste here"));

    state
        .design
        .schematic_clipboard
        .components
        .push(Component::new(1, ComponentType::Resistor, Point::origin()));
    assert!(labels_of(&menu_entries(&state, ContextTarget::Canvas)).contains(&"Paste here"));
}

#[test]
fn header_summary_uses_the_selected_instance_identity_and_master() {
    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let mut component =
        Component::new(7, ComponentType::CellInstance, Point::origin()).with_library_cell(
            LibraryCellInstance::new("vendor_analog", "OPA189_A", "schematic"),
        );
    component.name = "U1".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(7);

    let summary = selection_summary(&state, ContextTarget::Canvas);

    assert!(summary.starts_with("U1 · OPA189_A · /"));
}

#[test]
fn actions_are_truthfully_disabled_without_a_compatible_selection() {
    let mut state = DesignTestHost::default();
    assert!(!action_availability(ContextAction::Properties, &state).0);
    assert!(!action_availability(ContextAction::Rotate, &state).0);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    assert!(!action_availability(ContextAction::Delete, &state).0);
    assert!(action_availability(ContextAction::Probe, &state).0);
    assert!(!action_availability(ContextAction::OperatingPoint, &state).0);

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire_segment(17, 0);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    assert!(!action_availability(ContextAction::Duplicate, &state).0);
    assert!(!action_availability(ContextAction::Delete, &state).0);

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(999);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    state.model.schematic.session.read_only = true;
    assert!(!action_availability(ContextAction::Probe, &state).0);

    let mut junction_state = DesignTestHost::default();
    let point = Point::new(4, 4);
    junction_state
        .model
        .schematic
        .document_mut_for_test()
        .junctions
        .push(Junction::new(81, point));
    junction_state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_junction(point);
    assert!(action_availability(ContextAction::Delete, &junction_state).0);
    assert!(action_availability(ContextAction::Copy, &junction_state).0);
    assert!(!action_availability(ContextAction::Duplicate, &junction_state).0);
}

#[test]
fn properties_context_action_is_available_for_one_live_bus_or_tap() {
    let mut state = DesignTestHost::default();
    let bus = Bus::segment(
        31,
        Point::new(0, 0),
        Point::new(20, 0),
        Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
    )
    .unwrap();
    let tap = BusTap::new(
        32,
        &bus,
        Point::new(5, 0),
        Point::new(5, 5),
        BusSlice::parse("DATA[3]").unwrap(),
        BusTapOrientation::Down,
    )
    .unwrap();
    state
        .model
        .schematic
        .document_mut_for_test()
        .buses
        .push(bus);
    state
        .model
        .schematic
        .document_mut_for_test()
        .bus_taps
        .push(tap);

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_bus(31);
    assert!(action_availability(ContextAction::Properties, &state).0);
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_bus_tap(32);
    assert!(action_availability(ContextAction::Properties, &state).0);

    state.model.schematic.session.read_only = true;
    assert!(!action_availability(ContextAction::Properties, &state).0);
}

#[test]
fn pointer_target_prefers_a_junction_over_its_underlying_wire() {
    let mut state = DesignTestHost::default();
    let point = Point::new(10, 10);
    state.model.schematic.document_mut_for_test().wires =
        vec![Wire::new(17, vec![Point::new(0, 10), Point::new(20, 10)])];
    state.model.schematic.document_mut_for_test().junctions = vec![Junction::new(18, point)];
    let symbol_context = schematic_symbol_context(&state);
    let ctx = Context::default();
    let viewport = pointer_viewport();

    let target = select_pointer_target(
        &mut state,
        PointerHit::new(point, point),
        1,
        &symbol_context,
        &ctx,
        &viewport,
        pos2(point.x as f32, point.y as f32),
    );

    assert!(matches!(target, ContextTarget::Canvas));
    assert_eq!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_junction(),
        Some(point)
    );
    assert!(
        state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .is_empty()
    );
}

#[test]
fn duplicate_and_delete_rows_run_the_real_undoable_commands() {
    let mut state = DesignTestHost::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    state.model.schematic.document_mut_for_test().wires.clear();
    let mut component = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    component.name = "R1".to_owned();
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state.sync_active_schematic_to_workspace();
    state.model.schematic.init_undo_history();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(41);

    duplicate_schematic_selection_at(
        state.design.floating_copy_parts(),
        &mut state.model,
        &mut state.workbench,
        Point::new(22, 32),
    );
    assert!(crate::workbench::app::place_floating_paste(
        &mut state.design.floating_paste,
        &mut state.model,
        &mut state.workbench
    ));
    assert_eq!(state.model.schematic.document().components.len(), 2);
    assert!(state.model.schematic.can_undo());
    assert!(state.model.schematic.undo());
    assert_eq!(state.model.schematic.document().components.len(), 1);

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_component(41);
    delete_schematic_selection(&mut state.model, &mut state.workbench);
    assert!(state.model.schematic.document().components.is_empty());
    assert!(state.model.schematic.undo());
    assert_eq!(state.model.schematic.document().components.len(), 1);

    let label = NetLabel::new(73, Point::new(40, 40), "sense_out");
    state
        .model
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(label.clone());
    state.sync_active_schematic_to_workspace();
    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);

    duplicate_schematic_selection_at(
        state.design.floating_copy_parts(),
        &mut state.model,
        &mut state.workbench,
        Point::new(42, 42),
    );
    assert!(crate::workbench::app::place_floating_paste(
        &mut state.design.floating_paste,
        &mut state.model,
        &mut state.workbench
    ));
    assert_eq!(state.model.schematic.document().net_labels.len(), 2);
    assert!(state.model.schematic.can_undo());
    assert!(state.model.schematic.undo());
    assert_eq!(
        state.model.schematic.document().net_labels,
        vec![label.clone()]
    );

    state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);
    delete_schematic_selection(&mut state.model, &mut state.workbench);
    assert!(state.model.schematic.document().net_labels.is_empty());
    assert!(state.model.schematic.undo());
    assert_eq!(state.model.schematic.document().net_labels, vec![label]);
}

/// A paste on Sheet 1 joins its wire to Sheet 1's wiring only. Sheet 2 has
/// a wire through the very point where the pasted wire ends: it keeps its
/// id and points and gets no dot, while Sheet 1's wire there is split and
/// dotted, and one undo takes the paste and its wiring back.
#[test]
fn a_paste_joins_wiring_on_its_own_sheet_only() {
    use rspice_design::schematic::clipboard::ClipboardData;

    let mut state = DesignTestHost::default();
    crate::schematic::view::sheet_visibility::seed_two_sheets(&mut state.model, &[1], &[2]);
    let here = Wire::segment(1, Point::new(0, 100), Point::new(200, 100));
    let there = Wire::segment(2, Point::new(100, 0), Point::new(100, 200));
    state.model.schematic.document_mut_for_test().wires = vec![here.clone(), there.clone()];
    state.model.schematic.init_undo_history();
    let stub = Wire::segment(9, Point::new(0, 0), Point::new(0, 50));
    state.design.schematic_clipboard =
        ClipboardData::from_selection(Vec::new(), vec![stub], Vec::new());

    assert_eq!(
        paste_schematic_clipboard_at(
            state.design.floating_copy_parts(),
            &mut state.model,
            &mut state.workbench,
            Point::new(100, 50)
        ),
        Ok(true)
    );
    assert_eq!(
        state.model.schematic.document().wires,
        vec![here.clone(), there.clone()],
        "a floating paste changes nothing until it is placed"
    );
    assert!(crate::workbench::app::place_floating_paste(
        &mut state.design.floating_paste,
        &mut state.model,
        &mut state.workbench
    ));

    let document = state.model.schematic.document();
    assert_eq!(
        document.wires.iter().find(|wire| wire.id == 2),
        Some(&there),
        "Sheet 2's wire was touched"
    );
    let tee = Point::new(100, 100);
    let sheet_one_ends = document
        .wires
        .iter()
        .filter(|wire| wire.id != 2)
        .flat_map(|wire| [wire.points[0], *wire.points.last().expect("end")])
        .filter(|point| *point == tee)
        .count();
    assert_eq!(
        sheet_one_ends, 3,
        "Sheet 1's T was not split: {:?}",
        document.wires
    );
    assert_eq!(
        document
            .junctions
            .iter()
            .filter(|junction| junction.pos == tee)
            .count(),
        1
    );

    assert!(state.model.schematic.undo());
    assert_eq!(state.model.schematic.document().wires, vec![here, there]);
    assert!(state.model.schematic.document().junctions.is_empty());
}

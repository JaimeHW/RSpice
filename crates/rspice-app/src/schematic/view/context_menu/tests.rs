//! What the schematic selection context menu promises, and to whom.
//!
//! The catalog, its availability predicates, the keyboard contract, and the
//! destination each row hands off to. Split from `context_menu.rs` because a
//! command contract and the evidence that it holds are separate concerns.

use super::*;
use crate::schematic::view::schematic_symbol_context;
use egui::{Context, Key, Modifiers, Rect, Ui, pos2, vec2};

use crate::state::{
    Bus, BusDeclaration, BusSlice, BusTap, BusTapOrientation, Component, ComponentType, DesignNote,
    DesignNoteKind, DocumentationShape, DocumentationShapeGeometry, Junction, LibraryCellInstance,
    NetLabel, Wire,
};

fn pointer_viewport() -> Viewport {
    Viewport {
        offset: egui::Pos2::ZERO,
        zoom: 1.0,
        bounds: Rect::from_min_size(egui::Pos2::ZERO, vec2(400.0, 400.0)),
    }
}

fn context_key_input(key: Key) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(640.0, 480.0))),
        focused: true,
        events: vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: Modifiers::NONE,
        }],
        ..Default::default()
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
fn state_with_selected_source(library_edits: usize, adopt: bool) -> AppState {
    use crate::state::stimulus_library::definition::StimulusDefinition;

    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
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
        .workspace
        .content
        .stimulus_library
        .insert(definition)
        .expect("insert");
    for edit in 0..library_edits {
        let held = state
            .workspace
            .content
            .stimulus_library
            .get("sensor_drive")
            .expect("held")
            .clone();
        let mut draft = crate::state::stimulus_library::draft::DefinitionDraft::new(held);
        draft.edit(|working| working.params = format!("va=1 freq={}k", edit + 2));
        state
            .workspace
            .content
            .stimulus_library
            .apply(&mut draft, crate::state::stimulus_library::now_unix_ms);
    }
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(source);
    state.schematic.session.editor.selection.select_component(7);
    state
}

/// One Verilog-A instance of `behavioral/gain/veriloga`, whose view owns its
/// module's source, selected.
fn state_with_selected_veriloga_instance() -> (AppState, u64, crate::state::CellViewRef) {
    let mut state = AppState::default();
    let (id, reference) = select_a_veriloga_instance(&mut state);
    (state, id, reference)
}

/// Place one Verilog-A instance of `behavioral/gain/veriloga` on `state`'s
/// otherwise empty schematic and select it.
fn select_a_veriloga_instance(state: &mut AppState) -> (u64, crate::state::CellViewRef) {
    use crate::state::{
        Cell, CellViewRef, Library, PortDirection, PortSpec, ProjectSourceBundle,
        ProjectSourceLanguage, ProjectSourceOwner, View, ViewType,
    };

    state.schematic.document_mut_for_test().components.clear();
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
    state.library_manager.add_library(library);
    state
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
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(id);
    (id, reference)
}

fn catalog_of(entries: &[ContextEntry]) -> Vec<(&'static str, Option<Command>)> {
    entries
        .iter()
        .filter_map(|entry| match entry {
            ContextEntry::Command(command) => Some((command.label, command.shortcut_command)),
            ContextEntry::Separator => None,
        })
        .collect()
}

/// Each target's menu, row for row and rule for rule. Only the
/// instance-master slot differs: an instance's three master rows, a Verilog-A
/// instance's "Open Verilog-A view" in place of Descend, and the empty
/// canvas's one verb, "New Verilog-A device…", in place of all three, with
/// the zoom box after Fit. The row and rule counts are the ones
/// `rspice-schematic-editor`'s geometry test measures against the 635 px
/// ceiling: 20 + 4 (625 px), 19 + 4 (598 px).
#[test]
fn each_target_gets_the_catalog_its_instance_master_slot_calls_for() {
    let head = [
        ("Object properties…", Some(Command::ObjectProperties)),
        ("Rotate 90°", Some(Command::RotateSelection)),
        ("Mirror", Some(Command::MirrorSelectionHorizontal)),
        ("Mirror vertically", Some(Command::MirrorSelectionVertical)),
        // G-12: the Arrange page, which a phone reaches only from here.
        ("Arrange", None),
        ("Cut selection", Some(Command::Cut)),
        ("Copy selection", Some(Command::Copy)),
        ("Paste here", Some(Command::Paste)),
        ("Duplicate", Some(Command::Duplicate)),
        ("Delete", Some(Command::Delete)),
    ];
    let tail = [
        (
            "Create hierarchy from selection…",
            Some(Command::CreateHierarchy),
        ),
        ("Create symbol from schematic ports…", None),
        ("Page setup…", Some(Command::PageSetup)),
        ("Fit schematic content", Some(Command::FitSchematicContent)),
        ("Show in netlist", Some(Command::ShowInNetlist)),
        ("Add voltage or current probe…", Some(Command::PlaceProbe)),
        (
            "Open operating point",
            Some(Command::ResultViewer(ResultViewer::Op)),
        ),
    ];
    let menu = |slot: &[(&'static str, Option<Command>)]| {
        head.iter()
            .chain(slot)
            .chain(tail.iter())
            .copied()
            .collect::<Vec<_>>()
    };
    let master_rows = [
        (
            "Update instance interface",
            Some(Command::UpdateInstanceInterface),
        ),
        ("Replace instance…", Some(Command::ReplaceInstance)),
    ];
    let descend = (
        "Descend into selected instance",
        Some(Command::DescendHierarchyDirect),
    );
    let open_veriloga = ("Open Verilog-A view", Some(Command::DescendHierarchyDirect));

    // The empty canvas, which alone offers the zoom box: a finger has no Z.
    let state = AppState::default();
    let canvas = menu_entries(&state, ContextTarget::Canvas);
    let mut expected = menu(&[("New Verilog-A device…", Some(Command::NewVerilogADevice))]);
    let fit = expected
        .iter()
        .position(|(label, _)| *label == "Fit schematic content")
        .expect("the canvas menu fits the content");
    expected.insert(fit + 1, ("Zoom box", Some(Command::ZoomBox)));
    assert_eq!(catalog_of(&canvas), expected);
    assert_eq!((catalog_of(&canvas).len(), separators_in(&canvas)), (19, 4));

    // A schematic instance, and a wire: the master rows, Descend first.
    let mut resistor = AppState::default();
    resistor
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let id = resistor
        .schematic
        .add_component(ComponentType::Resistor, Point::new(40, 0));
    for target in [ContextTarget::Component(id), ContextTarget::Wire(17)] {
        let entries = menu_entries(&resistor, target);
        assert_eq!(
            catalog_of(&entries),
            menu(&[descend, master_rows[0], master_rows[1]]),
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
        menu(&[open_veriloga, master_rows[0], master_rows[1]])
    );
    assert_eq!(
        (catalog_of(&entries).len(), separators_in(&entries)),
        (20, 4)
    );
}

/// "Open Verilog-A view" opens the module's cell view as a Design tab: no
/// descent, because a module has no occurrences below it.
#[test]
fn open_veriloga_view_opens_the_instances_module_in_design() {
    let (mut state, _, reference) = state_with_selected_veriloga_instance();
    assert!(action_availability(ContextAction::OpenVerilogAView, &state).0);
    assert!(
        !action_availability(ContextAction::DescendHierarchy, &state).0,
        "a module is not a schematic master to descend into"
    );

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::OpenVerilogAView,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    assert_eq!(state.workspace.content.active_view, reference);
    assert_eq!(state.workbench.workspace, Workspace::Design);
    let open = state
        .workspace
        .content
        .open_views
        .iter()
        .find(|open| open.reference == reference)
        .expect("the module's view is an open tab");
    assert_eq!(
        open.occurrence,
        rspice_design::occurrence::DocumentOccurrence::rooted(reference.clone()),
        "opening a module records no occurrence step"
    );
}

/// The row names its shortcut, the Descend command, so the command must do
/// what the row does on a Verilog-A instance: it was disabled there, because
/// a module is not a schematic master.
#[test]
fn the_descend_shortcut_opens_a_veriloga_instances_view_as_its_row_does() {
    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    app.state.workbench.activate(Workspace::Design);
    let (_, reference) = select_a_veriloga_instance(&mut app.state);
    assert!(Command::DescendHierarchyDirect.is_enabled(&app));
    assert!(
        !Command::DescendHierarchy.is_enabled(&app),
        "the descent dialog lists schematic masters only"
    );

    Command::DescendHierarchyDirect.execute(&mut app);

    assert_eq!(app.state.workspace.content.active_view, reference);
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
}

/// New Verilog-A device… from the canvas opens the form, and is refused with
/// a reason where no library could take the cell.
#[test]
fn new_veriloga_device_opens_the_form_from_the_canvas() {
    let mut state = AppState::default();
    assert!(action_availability(ContextAction::NewVerilogADevice, &state).0);
    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::NewVerilogADevice,
        &mut state,
        Point::origin(),
        &symbol_context,
    );
    assert!(state.dialogs.new_veriloga_device.open);

    let names = state
        .library_manager
        .libraries_sorted()
        .iter()
        .map(|library| library.name.clone())
        .collect::<Vec<_>>();
    for name in names {
        state
            .library_manager
            .get_library_mut(&name)
            .expect("the library")
            .read_only = true;
    }
    let (enabled, reason) = action_availability(ContextAction::NewVerilogADevice, &state);
    assert!(!enabled);
    assert!(reason.contains("writable"), "{reason}");
}

/// A placed source's menu states the stimulus-library verbs its standing calls
/// for, in a group of their own, in place of the three rows about a cell
/// instance's master.
#[test]
fn a_placed_source_gets_the_stimulus_verbs_its_standing_calls_for() {
    let group = |state: &AppState| -> Vec<&'static str> {
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
    stimulus::execute(ContextAction::ReadoptStimulus, &mut behind);
    let source = &behind.schematic.document().components[0];
    assert!(source.params.contains("freq=2k"), "{}", source.params);
    assert_eq!(
        behind
            .workspace
            .content
            .stimulus_library
            .provenance_state(source)
            .label(),
        "adopted · r2"
    );

    let mut local = state_with_selected_source(0, false);
    stimulus::execute(ContextAction::SaveStimulus, &mut local);
    assert!(local.dialogs.stimulus_link.open);

    let mut adopted = state_with_selected_source(0, true);
    stimulus::execute(ContextAction::OpenStimulusDefinition, &mut adopted);
    assert_eq!(
        adopted.workbench.selected_stimulus_definition.as_deref(),
        Some("sensor_drive")
    );

    // With nothing in the library there is nothing to adopt, and the row says
    // what to do about it rather than opening a dialog that refuses.
    let mut empty = state_with_selected_source(0, false);
    empty.workspace.content.stimulus_library = crate::state::StimulusLibrary::default();
    let (enabled, reason) = action_availability(ContextAction::AdoptStimulus, &empty);
    assert!(!enabled);
    assert!(reason.contains("save this source"), "{reason}");
}

#[test]
fn header_summary_uses_the_selected_instance_identity_and_master() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    let mut component =
        Component::new(7, ComponentType::CellInstance, Point::origin()).with_library_cell(
            LibraryCellInstance::new("vendor_analog", "OPA189_A", "schematic"),
        );
    component.name = "U1".to_owned();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state.schematic.session.editor.selection.select_component(7);

    let summary = selection_summary(&state, ContextTarget::Canvas);

    assert!(summary.starts_with("U1 · OPA189_A · /"));
}

#[test]
fn focused_keyboard_context_row_activates_with_enter_or_space() {
    for key in [Key::Enter, Key::Space] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components.clear();
        state
            .schematic
            .document_mut_for_test()
            .components
            .push(Component::new(7, ComponentType::Resistor, Point::origin()));
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(7);
        state.dialogs.interaction.context_target = Some((ContextTarget::Component(7), (0, 0)));
        let symbol_context = schematic_symbol_context(&state);

        let _ = ctx.run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                render_context_contents(ui, &mut state, &symbol_context, 27.0, true);
            });
        });

        let mut key_still_available = true;
        let _ = ctx.run_ui(context_key_input(key), |root| {
            egui::CentralPanel::default().show(root, |ui| {
                render_context_contents(ui, &mut state, &symbol_context, 27.0, false);
                key_still_available = ui
                    .ctx()
                    .input_mut(|input| input.consume_key(Modifiers::NONE, key));
            });
        });

        assert!(
            state.tabbed_property_dialog.open || state.dialogs.object_properties.open,
            "{key:?} should activate the focused Object properties row"
        );
        assert!(
            !key_still_available,
            "{key:?} should be owned by the focused context-menu row"
        );
    }
}

#[test]
fn actions_are_truthfully_disabled_without_a_compatible_selection() {
    let mut state = AppState::default();
    assert!(!action_availability(ContextAction::Properties, &state).0);
    assert!(!action_availability(ContextAction::Rotate, &state).0);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    assert!(!action_availability(ContextAction::Delete, &state).0);
    assert!(action_availability(ContextAction::Probe, &state).0);
    assert!(!action_availability(ContextAction::OperatingPoint, &state).0);

    state
        .schematic
        .session
        .editor
        .selection
        .select_wire_segment(17, 0);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    assert!(!action_availability(ContextAction::Duplicate, &state).0);
    assert!(!action_availability(ContextAction::Delete, &state).0);

    state
        .schematic
        .session
        .editor
        .selection
        .select_component(999);
    assert!(!action_availability(ContextAction::Copy, &state).0);
    state.schematic.session.read_only = true;
    assert!(!action_availability(ContextAction::Probe, &state).0);

    let mut junction_state = AppState::default();
    let point = Point::new(4, 4);
    junction_state
        .schematic
        .document_mut_for_test()
        .junctions
        .push(Junction::new(81, point));
    junction_state
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
    let mut state = AppState::default();
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
    state.schematic.document_mut_for_test().buses.push(bus);
    state.schematic.document_mut_for_test().bus_taps.push(tap);

    state.schematic.session.editor.selection.select_only_bus(31);
    assert!(action_availability(ContextAction::Properties, &state).0);
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_bus_tap(32);
    assert!(action_availability(ContextAction::Properties, &state).0);

    state.schematic.session.read_only = true;
    assert!(!action_availability(ContextAction::Properties, &state).0);
}

#[test]
fn pointer_target_prefers_a_junction_over_its_underlying_wire() {
    let mut state = AppState::default();
    let point = Point::new(10, 10);
    state.schematic.document_mut_for_test().wires =
        vec![Wire::new(17, vec![Point::new(0, 10), Point::new(20, 10)])];
    state.schematic.document_mut_for_test().junctions = vec![Junction::new(18, point)];
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
        state.schematic.session.editor.selection.single_junction(),
        Some(point)
    );
    assert!(state.schematic.session.editor.selection.wires.is_empty());
}

#[test]
fn net_label_context_exposes_the_complete_object_lifecycle() {
    let mut state = AppState::default();
    let label = NetLabel::new(73, Point::new(40, 40), "afe_out");
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(label.clone());
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    assert!(selection_summary(&state, ContextTarget::Canvas).contains("net label · afe_out"));

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Properties,
        &mut state,
        label.pos,
        &symbol_context,
    );
    assert!(state.dialogs.object_properties.open);
    assert!(!state.dialogs.rename_selection.open);
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::NetLabel(ref draft))
            if draft.original == label
    ));
}

#[test]
fn design_note_context_exposes_only_compatible_object_lifecycle_actions() {
    let mut state = AppState::default();
    let note = DesignNote::new(
        74,
        Point::new(40, 40),
        DesignNoteKind::ReviewNote,
        "Review bias path",
    )
    .unwrap();
    state
        .schematic
        .document_mut_for_test()
        .design_notes
        .push(note.clone());
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_design_note(note.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    // A note turns and flips with the rest of the drawing (audit E-2).
    assert!(action_availability(ContextAction::Rotate, &state).0);
    assert!(action_availability(ContextAction::Mirror, &state).0);
    let summary = selection_summary(&state, ContextTarget::Canvas);
    assert!(summary.contains("Review note"));
    assert!(summary.contains("Review bias path"));

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Properties,
        &mut state,
        note.pos,
        &symbol_context,
    );
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::DesignNote(ref draft))
            if draft.original == note
    ));
}

#[test]
fn documentation_shape_context_exposes_the_complete_non_electrical_lifecycle() {
    let mut state = AppState::default();
    let shape = DocumentationShape::new(
        75,
        DocumentationShapeGeometry::Callout {
            tip: Point::new(10, 10),
            elbow: Point::new(30, 20),
            box_corner: Point::new(80, 50),
        },
    )
    .unwrap();
    state
        .schematic
        .document_mut_for_test()
        .documentation_shapes
        .push(shape.clone());
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_documentation_shape(shape.id);

    assert!(action_availability(ContextAction::Properties, &state).0);
    assert!(action_availability(ContextAction::Copy, &state).0);
    assert!(action_availability(ContextAction::Duplicate, &state).0);
    assert!(action_availability(ContextAction::Delete, &state).0);
    // A shape turns and flips with the rest of the drawing (audit E-2).
    assert!(action_availability(ContextAction::Rotate, &state).0);
    assert!(action_availability(ContextAction::Mirror, &state).0);
    assert!(selection_summary(&state, ContextTarget::Canvas).contains("Callout"));

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Properties,
        &mut state,
        Point::new(30, 20),
        &symbol_context,
    );
    assert!(matches!(
        state.dialogs.object_properties.draft,
        Some(rspice_schematic_editor::object_properties::ObjectPropertiesDraft::DocumentationShape(ref draft))
            if draft.original == shape
    ));
}

#[test]
fn duplicate_and_delete_rows_run_the_real_undoable_commands() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    state.schematic.document_mut_for_test().wires.clear();
    let mut component = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    component.name = "R1".to_owned();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state.sync_active_schematic_to_workspace();
    state.schematic.init_undo_history();
    state
        .schematic
        .session
        .editor
        .selection
        .select_component(41);

    state.duplicate_schematic_selection_at(Point::new(22, 32));
    assert!(state.place_floating_paste());
    assert_eq!(state.schematic.document().components.len(), 2);
    assert!(state.schematic.can_undo());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().components.len(), 1);

    state
        .schematic
        .session
        .editor
        .selection
        .select_component(41);
    state.delete_schematic_selection();
    assert!(state.schematic.document().components.is_empty());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().components.len(), 1);

    let label = NetLabel::new(73, Point::new(40, 40), "sense_out");
    state
        .schematic
        .document_mut_for_test()
        .net_labels
        .push(label.clone());
    state.sync_active_schematic_to_workspace();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);

    state.duplicate_schematic_selection_at(Point::new(42, 42));
    assert!(state.place_floating_paste());
    assert_eq!(state.schematic.document().net_labels.len(), 2);
    assert!(state.schematic.can_undo());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().net_labels, vec![label.clone()]);

    state
        .schematic
        .session
        .editor
        .selection
        .select_only_net_label(label.id);
    state.delete_schematic_selection();
    assert!(state.schematic.document().net_labels.is_empty());
    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().net_labels, vec![label]);
}

/// The canvas row asks the schematic the same staleness question the
/// Design command asks, and runs the same repair. A row with its own
/// answer would offer the repair where the deck does not need it.
#[test]
fn the_interface_repair_row_is_offered_and_runs_only_for_a_stale_instance() {
    const MASTER: &str = "work/div/schematic";

    let mut state = AppState::default();
    let mut master = crate::state::SchematicState::default();
    let port = master.add_component(ComponentType::Port, Point::new(20, 0));
    master
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == port)
        .expect("the placed port is retained")
        .value = "a".to_owned();
    let mut binding = LibraryCellInstance::new("work", "div", "schematic");
    binding.bind_interface(&master.interface_ports());
    state
        .workspace
        .insert_schematic_editor(MASTER.to_owned(), master);
    let instance = state
        .schematic
        .add_library_cell_component(Point::new(100, 0), binding);
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(instance);

    assert!(
        !action_availability(ContextAction::UpdateInstanceInterface, &state).0,
        "a placement that still matches its master is not stale"
    );

    state
        .workspace
        .schematic_editor_mut(MASTER)
        .expect("the fixture registers the master")
        .editor
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.value == "a")
        .expect("the master declares port a")
        .value = "ain".to_owned();

    assert!(action_availability(ContextAction::UpdateInstanceInterface, &state).0);

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::UpdateInstanceInterface,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    assert_eq!(
        state.schematic.document().components[0]
            .library_cell
            .as_ref()
            .expect("the instance stays bound")
            .terminal_order,
        ["ain"]
    );
    assert!(!action_availability(ContextAction::UpdateInstanceInterface, &state).0);
}

/// The replacement row is offered exactly where the command is, and opens
/// the same reviewed transaction. A row with its own answer would offer a
/// replacement the dialog would immediately refuse.
#[test]
fn the_replace_instance_row_is_offered_and_runs_only_for_one_replaceable_instance() {
    let mut state = AppState::default();
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &state).0,
        "no selection is not one replaceable instance"
    );

    let first = state
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(40, 0));
    let second = state
        .schematic
        .add_component(ComponentType::VoltageSource, Point::new(120, 0));
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    assert!(action_availability(ContextAction::ReplaceInstance, &state).0);
    assert_eq!(
        action_availability(ContextAction::ReplaceInstance, &state).0,
        replace_instance_available(&state),
        "the row must answer with the command's own predicate"
    );

    state
        .schematic
        .session
        .editor
        .selection
        .select_component(second);
    assert!(
        !action_availability(ContextAction::ReplaceInstance, &state).0,
        "two selected instances are not one replaceable instance"
    );

    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(first);
    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::ReplaceInstance,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    assert!(state.dialogs.replace_instance.open);
    assert_eq!(state.dialogs.replace_instance.source_component_id, first);
}

/// One reported device, selected on the canvas.
fn state_with_reported_device_op(device: &str) -> AppState {
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun};

    let mut state = AppState::default();
    let mut component = Component::new(1, ComponentType::Nmos, Point::new(40, 30));
    component.name = device.to_owned();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(1);

    let mut analysis = AnalysisResult::new(1, AnalysisType::DcOp, "OP");
    analysis.device_op = Some(rspice_core::circuit::DeviceOpReport {
        entries: vec![rspice_core::circuit::DeviceOpEntry {
            name: device.to_owned(),
            device_kind: "MOSFET",
            region: Some("saturation"),
            params: vec![("id", 1.0e-4)],
        }],
    });
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    state.simulation.retained.runs.insert(0, run);
    state.simulation.view.active_run_idx = Some(0);
    state.simulation.view.active_analysis_idx = Some(0);
    state
}

#[test]
fn operating_point_hop_carries_the_clicked_device_into_the_op_inspector() {
    let mut state = state_with_reported_device_op("M1");
    assert!(action_availability(ContextAction::OperatingPoint, &state).0);

    open_operating_point(&mut state);

    assert_eq!(state.workbench.workspace, Workspace::Results);
    assert_eq!(state.ui.results.session.viewer, ResultViewer::Op);
    assert_eq!(
        state.ui.results.session.op_filter, "M1",
        "the Op inspector must arrive selected on the device the menu named"
    );
}

#[test]
fn operating_point_hop_leaves_the_report_unfiltered_when_the_device_is_unreported() {
    let mut state = state_with_reported_device_op("M1");
    // A second instance the run never reported. Filtering to it would show
    // an empty inspector instead of the report the reader asked to open.
    let mut unreported = Component::new(2, ComponentType::Nmos, Point::new(80, 30));
    unreported.name = "M2".to_owned();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(unreported);
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(2);

    // A stale filter is the failure this guards: the previous device's name
    // was left in place, so the inspector opened on another instance's row
    // and read as a hop that had worked.
    state.ui.results.session.op_filter = "M1".to_owned();

    open_operating_point(&mut state);

    assert_eq!(state.workbench.workspace, Workspace::Results);
    assert_eq!(state.ui.results.session.viewer, ResultViewer::Op);
    assert!(state.ui.results.session.op_filter.is_empty());
}

/// The hop selects the operating point, not just the workspace and viewer.
///
/// The Op inspector renders the *selected* analysis. A run of [OP, TRAN] with
/// the transient selected — which is where a reader usually is when they go
/// looking for a bias point — landed on "not a DC operating-point result": the
/// workspace and the viewer were right and the one thing that decides what they
/// show was left wherever it had been.
#[test]
fn operating_point_hop_selects_the_operating_point_analysis() {
    use crate::state::{AnalysisResult, AnalysisType};

    let mut state = state_with_reported_device_op("M1");
    let transient = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    state
        .simulation
        .retained
        .runs
        .first_mut()
        .expect("the fixture run is retained")
        .add_analysis(transient);
    state.simulation.view.active_analysis_idx = Some(1);
    assert_eq!(
        state
            .simulation
            .active_analysis()
            .expect("the fixture selects an analysis")
            .analysis_type,
        AnalysisType::Transient,
        "the reader starts on the transient"
    );

    open_operating_point(&mut state);

    assert_eq!(
        state
            .simulation
            .active_analysis()
            .expect("the hop leaves an analysis selected")
            .analysis_type,
        AnalysisType::DcOp,
        "the hop must select the result its viewer can render"
    );
    assert_eq!(state.ui.results.session.op_filter, "M1");
}

fn render_context_contents(
    ui: &mut Ui,
    state: &mut AppState,
    symbol_context: &SchematicSymbolContext,
    row_height: f32,
    focus_first: bool,
) {
    let view = context_menu_view(state, ui.ctx()).expect("context target");
    if let Some(request) =
        context_surface::render_context_contents(ui, &view, row_height, focus_first)
    {
        apply_context_request(state, request, symbol_context);
    }
}

#[test]
fn context_requests_recheck_the_opening_selection_source_and_authority() {
    for change in [
        "source",
        "selection",
        "target",
        "tool",
        "read-only",
        "modal",
        "none",
    ] {
        let mut state = AppState::default();
        state.schematic.document_mut_for_test().components.clear();
        for id in [7, 8] {
            state
                .schematic
                .document_mut_for_test()
                .components
                .push(Component::new(
                    id,
                    ComponentType::Resistor,
                    Point::new(id as i32 * 10, 0),
                ));
        }
        state.schematic.init_undo_history();
        state
            .schematic
            .session
            .editor
            .selection
            .select_only_component(7);
        state.dialogs.interaction.context_target = Some((ContextTarget::Component(7), (70, 0)));
        let request = ContextMenuRequest {
            binding: context_menu_binding(&state).unwrap(),
            action: ContextAction::Delete,
        };
        match change {
            "source" => state.bump_active_schematic_epoch(),
            "selection" => state
                .schematic
                .session
                .editor
                .selection
                .select_only_component(8),
            "target" => {
                state.dialogs.interaction.context_target =
                    Some((ContextTarget::Component(8), (80, 0)))
            }
            "tool" => state.schematic.session.editor.tool = Tool::Wire,
            "read-only" => state.schematic.session.read_only = true,
            "modal" => state.dialogs.preferences_open = true,
            "none" => {}
            _ => unreachable!(),
        }
        let before = state.schematic.document().clone();
        let selection = state.schematic.session.editor.selection.clone();
        let symbol_context = schematic_symbol_context(&state);
        apply_context_request(&mut state, request, &symbol_context);
        if change == "none" {
            assert_eq!(state.schematic.document().components.len(), 1);
            assert_eq!(state.schematic.document().components[0].id, 8);
            assert!(state.schematic.undo());
            assert_eq!(state.schematic.document().components, before.components);
        } else {
            assert_eq!(
                state.schematic.document().components,
                before.components,
                "{change}"
            );
            assert_eq!(
                state.schematic.session.editor.selection, selection,
                "{change}"
            );
            assert!(!state.schematic.can_undo(), "{change}");
        }
    }
}

/// Each target's menu as the popup draws it: the empty canvas, a schematic
/// instance, and a Verilog-A instance.
#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_the_per_target_context_menus_for_review() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();

    let mut instance = AppState::default();
    instance
        .schematic
        .document_mut_for_test()
        .components
        .clear();
    let mut resistor = Component::new(41, ComponentType::Resistor, Point::new(40, 0));
    resistor.name = "R1".to_owned();
    resistor.value = "10k".to_owned();
    instance
        .schematic
        .document_mut_for_test()
        .components
        .push(resistor);
    instance
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    let (veriloga, id, _) = state_with_selected_veriloga_instance();
    for (name, mut state, target) in [
        (
            "context-menu-canvas",
            AppState::default(),
            ContextTarget::Canvas,
        ),
        (
            "context-menu-instance",
            instance,
            ContextTarget::Component(41),
        ),
        (
            "context-menu-veriloga-instance",
            veriloga,
            ContextTarget::Component(id),
        ),
    ] {
        state.dialogs.interaction.context_target = Some((target, (0, 0)));
        // The popup's own frame and 286 px width, at the desktop row height.
        let size = vec2(300.0, 620.0);
        let canvas = crate::ui::raster::render(size, |ui, _| {
            let view = context_menu_view(&state, ui.ctx()).expect("a context target");
            let t = crate::ui::tokens::Tokens::get(ui.ctx());
            egui::Frame::new()
                .fill(t.color.bg_elevated)
                .stroke(egui::Stroke::new(1.0, t.color.border_strong))
                .corner_radius(3)
                .show(ui, |ui| {
                    ui.set_width(284.0);
                    ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                    let _ = context_surface::render_context_contents(ui, &view, 27.0, false);
                });
        });
        let path = directory.join(format!("{name}.png"));
        std::fs::write(&path, canvas.png(size.y as usize)).expect("write the render");
        writeln!(report, "{}", path.display()).ok();
    }
}

/// Sheet 2 reuses Sheet 1's layout: a wire joins its two parts at the very
/// points where R3 and R4 stand unconnected on Sheet 1, and a junction sits
/// where Sheet 1's selected wires cross without one. Copy took that wire
/// and that dot as if they joined the selection, so every paste carried a
/// short the source sheet does not have (audit U-5).
#[test]
fn copy_takes_no_wire_or_junction_from_another_sheet() {
    let mut state =
        crate::schematic::view::sheet_visibility::two_sheet_state(&[1, 3, 5, 7], &[2, 6]);
    let r3 = Component::new(1, ComponentType::Resistor, Point::origin());
    let r4 = Component::new(3, ComponentType::Resistor, Point::new(100, 0));
    let r3_right = r3.terminal_positions()[1].1;
    let r4_left = r4.terminal_positions()[0].1;
    let drawing = state.schematic.document_mut_for_test();
    drawing.components.extend([r3, r4]);
    drawing.wires.push(Wire::segment(2, r3_right, r4_left));
    drawing
        .wires
        .push(Wire::segment(5, Point::new(-50, -50), Point::new(150, -50)));
    drawing
        .wires
        .push(Wire::segment(7, Point::new(50, -100), Point::new(50, 0)));
    drawing
        .junctions
        .push(Junction::new(6, Point::new(50, -50)));
    let selection = &mut state.schematic.session.editor.selection;
    for id in [1, 3] {
        selection.select_component(id);
    }
    for id in [5, 7] {
        selection.select_wire(id);
    }

    let symbol_context = schematic_symbol_context(&state);
    execute_context_action(
        ContextAction::Copy,
        &mut state,
        Point::origin(),
        &symbol_context,
    );

    let clipboard = &state.ui.schematic_clipboard;
    assert_eq!(clipboard.components.len(), 2);
    let mut copied: Vec<u64> = clipboard.wires.iter().map(|wire| wire.id).collect();
    copied.sort_unstable();
    assert_eq!(copied, vec![5, 7], "Sheet 2's wire was copied");
    assert!(
        clipboard.junctions.is_empty(),
        "Sheet 2's junction was copied: {:?}",
        clipboard.junctions
    );
}

/// A paste on Sheet 1 joins its wire to Sheet 1's wiring only. Sheet 2 has
/// a wire through the very point where the pasted wire ends: it keeps its
/// id and points and gets no dot, while Sheet 1's wire there is split and
/// dotted, and one undo takes the paste and its wiring back.
#[test]
fn a_paste_joins_wiring_on_its_own_sheet_only() {
    use rspice_design::schematic::clipboard::ClipboardData;

    let mut state = crate::schematic::view::sheet_visibility::two_sheet_state(&[1], &[2]);
    let here = Wire::segment(1, Point::new(0, 100), Point::new(200, 100));
    let there = Wire::segment(2, Point::new(100, 0), Point::new(100, 200));
    state.schematic.document_mut_for_test().wires = vec![here.clone(), there.clone()];
    state.schematic.init_undo_history();
    let stub = Wire::segment(9, Point::new(0, 0), Point::new(0, 50));
    state.ui.schematic_clipboard =
        ClipboardData::from_selection(Vec::new(), vec![stub], Vec::new());

    assert_eq!(
        state.paste_schematic_clipboard_at(Point::new(100, 50)),
        Ok(true)
    );
    assert_eq!(
        state.schematic.document().wires,
        vec![here.clone(), there.clone()],
        "a floating paste changes nothing until it is placed"
    );
    assert!(state.place_floating_paste());

    let document = state.schematic.document();
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

    assert!(state.schematic.undo());
    assert_eq!(state.schematic.document().wires, vec![here, there]);
    assert!(state.schematic.document().junctions.is_empty());
}

/// G-1 and G-12: while a duplicate floats under the pointer, R turns it, and
/// so does the long-press menu's Rotate row, which is a finger's R. The part
/// it was copied from stays as it was.
#[test]
fn the_rotate_row_turns_a_floating_duplicate_not_its_original() {
    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    let mut part = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    part.name = "R1".to_owned();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(part);
    state.sync_active_schematic_to_workspace();
    state.schematic.init_undo_history();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    assert!(state.duplicate_schematic_selection_at(Point::new(60, 30)));
    let symbol_context = schematic_symbol_context(&state);

    assert!(action_availability(ContextAction::Rotate, &state).0);
    execute_context_action(
        ContextAction::Rotate,
        &mut state,
        Point::new(60, 30),
        &symbol_context,
    );
    assert!(state.place_floating_paste());
    let quarter = rspice_design::schematic::rotation::Rotation::default().rotate_cw();
    let turned: Vec<_> = state
        .schematic
        .document()
        .components
        .iter()
        .map(|part| (part.id == 41, part.rotation))
        .collect();
    assert_eq!(
        turned,
        vec![(true, Default::default()), (false, quarter)],
        "only the copy turned"
    );
}

/// The canvas menu had Copy, Duplicate and Delete but no Cut, no Paste and
/// no vertical mirror, so a reader working from the menu had to reach for
/// keys to finish the job (audit E-9). Each row is offered on every target
/// and runs its command: Cut takes the part onto the clipboard, Paste puts
/// it back where the menu was opened, and Mirror vertically flips the
/// selection top to bottom.
#[test]
fn cut_paste_and_mirror_vertically_rows_run_their_commands() {
    use crate::schematic::view::selection_transform::SelectionTransform;

    let mut state = AppState::default();
    state.schematic.document_mut_for_test().components.clear();
    state.schematic.document_mut_for_test().wires.clear();
    let mut part = Component::new(41, ComponentType::Resistor, Point::new(20, 30));
    part.name = "R1".to_owned();
    state
        .schematic
        .document_mut_for_test()
        .components
        .push(part);
    state.sync_active_schematic_to_workspace();
    state.schematic.init_undo_history();
    state
        .schematic
        .session
        .editor
        .selection
        .select_only_component(41);
    let symbol_context = schematic_symbol_context(&state);

    for target in [ContextTarget::Component(41), ContextTarget::Canvas] {
        let actions: Vec<ContextAction> = menu_entries(&state, target)
            .iter()
            .filter_map(|entry| match entry {
                ContextEntry::Command(command) => Some(command.action),
                ContextEntry::Separator => None,
            })
            .collect();
        for action in [
            ContextAction::Cut,
            ContextAction::Paste,
            ContextAction::MirrorVertical,
        ] {
            assert!(actions.contains(&action), "{target:?} lacks {action:?}");
        }
    }
    assert!(action_availability(ContextAction::Cut, &state).0);
    assert!(
        !action_availability(ContextAction::Paste, &state).0,
        "nothing has been copied yet"
    );

    execute_context_action(
        ContextAction::Cut,
        &mut state,
        Point::origin(),
        &symbol_context,
    );
    assert!(state.schematic.document().components.is_empty());
    assert_eq!(state.ui.schematic_clipboard.components.len(), 1);
    assert!(action_availability(ContextAction::Paste, &state).0);

    let click = Point::new(200, 100);
    let offset = click - state.ui.schematic_clipboard.origin;
    execute_context_action(ContextAction::Paste, &mut state, click, &symbol_context);
    assert!(state.place_floating_paste());
    let pasted = state.schematic.document().components.clone();
    assert_eq!(pasted.len(), 1);
    assert_eq!(pasted[0].pos, Point::new(20, 30) + offset);
    assert!(
        state
            .schematic
            .session
            .editor
            .selection
            .has_component(pasted[0].id)
    );

    assert!(action_availability(ContextAction::MirrorVertical, &state).0);
    execute_context_action(
        ContextAction::MirrorVertical,
        &mut state,
        click,
        &symbol_context,
    );
    let mirrored = &state.schematic.document().components[0];
    assert_ne!(
        (mirrored.mirror_v, mirrored.mirror_h, mirrored.rotation),
        (pasted[0].mirror_v, pasted[0].mirror_h, pasted[0].rotation),
        "the part was not flipped"
    );
    assert_eq!(
        state.schematic.undo_description(),
        Some(SelectionTransform::MirrorVertical.description())
    );
}

/// Under "Selected net across hierarchy" an Alt+click net highlight is
/// carried into the instance a user descends into. Picking that instance
/// cleared it first: the click before the descend key or a double-click,
/// and the context menu that offers Descend, so the child lit nothing and
/// neither did the parent on the way back up. Picking a descendable
/// instance now leaves it lit; picking an ordinary part still clears it.
#[test]
fn picking_an_instance_to_descend_keeps_the_net_highlight_it_carries() {
    use crate::state::SchematicNetHighlighting;
    use crate::workbench::examples::hierarchy_reference::{self, AMP_INSTANCES};
    use rspice_schematic_editor::requests::{EditorAction, EditorRequest};

    fn lit(state: &AppState) -> Option<String> {
        let highlight = &state.schematic.session.editor.net_highlight;
        highlight
            .active
            .then(|| highlight.selected_net_name.clone())
            .flatten()
    }
    fn click(state: &mut AppState, id: u64) {
        let request = EditorRequest {
            source: super::super::requests::editor_request_source(state),
            selection: state.schematic.session.editor.selection.clone(),
            action: EditorAction::SelectPointer {
                target: Some(PointerTarget::Component(id)),
                additive: false,
                alt_held: false,
            },
        };
        super::super::requests::apply_editor_request(state, request);
    }

    let reference = hierarchy_reference::build();
    let mut state = reference.state;
    state.open_workspace_view(reference.top.clone());
    state.ui.schematic_visibility.net_highlighting =
        SchematicNetHighlighting::SelectedNetAcrossHierarchy;
    let component_named = |state: &AppState, name: &str| {
        state
            .schematic
            .document()
            .components
            .iter()
            .find(|component| component.name == name)
            .map(|component| component.id)
            .expect("top places the component")
    };
    let x1 = component_named(&state, AMP_INSTANCES[0]);

    for by_menu in [false, true] {
        assert!(state.light_net_named("in"), "top has a net named in");
        if by_menu {
            select_context_target(&mut state, Some(PointerTarget::Component(x1)));
        } else {
            click(&mut state, x1);
        }
        assert_eq!(
            state.schematic.session.editor.selection.single_component(),
            Some(x1)
        );
        assert_eq!(lit(&state).as_deref(), Some("in"), "menu {by_menu}");

        let symbol_context = schematic_symbol_context(&state);
        execute_context_action(
            ContextAction::DescendHierarchy,
            &mut state,
            Point::origin(),
            &symbol_context,
        );
        assert_eq!(lit(&state).as_deref(), Some("IN"), "X1's port is lit");
        state.ascend_workspace_level();
        assert_eq!(lit(&state).as_deref(), Some("in"), "back up, in is lit");
        state.schematic.session.editor.selection.clear();
    }

    let part = state
        .schematic
        .document()
        .components
        .iter()
        .find(|component| state.hierarchy_master_for_component(component.id).is_none())
        .map(|component| component.id)
        .expect("top places an ordinary part");
    click(&mut state, part);
    assert_eq!(lit(&state), None, "an ordinary part still clears it");
}

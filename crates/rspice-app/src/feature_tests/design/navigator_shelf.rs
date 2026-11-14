//! The component shelf drawn through a whole application: arming, variants, supplies, pins,
//! the recent band and its history, keyboard walks and the stimulus section.

use crate::schematic::view::SchematicShelfDragPayload;
use crate::schematic::{
    ComponentCategory, PaletteRow, component_palette, palette_rows, supply_palette,
};
use crate::state::model_hub::PartPlacement;
use crate::state::{
    ComponentType, Tool, builtin_xspice_library_binding, builtin_xspice_vector_ports,
    engine_only_xspice_devices,
};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::design::navigator::shelf::*;
use crate::workbench::design::navigator::tests::shelf::{ShelfHarness, retained_model_library};
use crate::workbench::design::navigator::tests::*;
use crate::workbench::framework::requests::FocusPlacementSearch;
use std::collections::HashSet;

/// The canvas-side shelf can place what the project already holds: a retained
/// part arms the cursor directly instead of re-raising the pack confirmation.
/// This is the road that used to dead-end — the row was filtered out of this
/// section, and the Project-library section below reads the symbol library,
/// not the model-library manager, so an adopted pack part had no shelf door.
#[test]
fn a_part_the_project_retained_arms_from_the_component_shelf() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model
        .model_library_manager
        .add_library(retained_model_library("proving_parts"));

    let rows = library_part_rows(
        &app.model_hub,
        &app.state.model,
        &crate::workbench::design::navigator::shelf::search::ShelfQuery::new("proving_parts"),
    );
    let row = |name: &str| {
        rows.iter()
            .find(|row| row.part_id == name)
            .unwrap_or_else(|| panic!("no shelf row for '{name}' in {rows:?}"))
    };

    let macromodel = row("PROVING_DIV");
    // The catalog's word for a subcircuit, not a second one: the same column
    // carries pack rows, and the class chip that narrows it spells it this
    // way. Where it came from is normal and is the Models workspace's.
    assert_eq!(macromodel.meta, "subckt");
    let LibraryPartAction::Arm(placement) = &macromodel.action else {
        panic!(
            "a retained macromodel arms directly: {:?}",
            macromodel.action
        );
    };
    let PartPlacement::CellInstance(binding) = placement.as_ref() else {
        panic!("a macromodel places as a cell instance over its own ports");
    };
    assert_eq!(
        binding.terminal_order,
        ["IN", "OUT", "GND"].map(str::to_owned)
    );

    assert_eq!(
        row("RSPICE_ZENER").action,
        LibraryPartAction::Arm(Box::new(PartPlacement::NativeDevice {
            component_type: ComponentType::Diode,
            variant: None,
            model: "RSPICE_ZENER".to_owned(),
        }))
    );

    // The card no schematic device is drawn for stays listed, refused as a
    // sentence rather than hidden.
    let LibraryPartAction::Refused(reason) = &row("VENDOR_PRIVATE").action else {
        panic!("an undrawable card is refused");
    };
    assert!(
        reason.contains("VENDOR_PRIVATE") && reason.ends_with('.'),
        "{reason}"
    );

    // A section-scoped subcircuit key is not a part a reader picks.
    assert!(!rows.iter().any(|row| row.part_id.contains('\u{1f}')));
}

/// Every symbol skin the artwork library registers is one a reader can reach
/// without placing something else first: a shelf row places it, or the type's
/// Symbol property offers it. And every row's skin is one the property offers,
/// so a placed variant can be switched back.
#[test]
fn every_registered_symbol_variant_is_reachable_from_a_row_or_a_property() {
    use rspice_design::properties::PropertyValue;

    let app = RSpiceApp::test_instance();
    let library =
        rspice_design::symbol_artwork::SymbolLibrary::load_embedded().expect("artwork loads");
    let offered = |kind: ComponentType, variant: &str| {
        app.state
            .design
            .property_registry
            .get(kind)
            .into_iter()
            .flat_map(|sheet| sheet.iter())
            .filter(|definition| definition.name == "symbol")
            .any(|definition| {
                matches!(&definition.default_value, PropertyValue::Enum { options, .. }
                    if options.iter().any(|option| option == variant))
            })
    };
    let rows = component_palette()
        .iter()
        .flat_map(|section| section.entries)
        .filter_map(|entry| Some((entry.kind, entry.variant?)))
        .collect::<HashSet<_>>();
    for (kind, variant) in library.variants() {
        assert!(
            rows.contains(&(kind, variant)) || offered(kind, variant),
            "{kind:?} registers the {variant:?} skin, but no row or property reaches it"
        );
    }
    for (kind, variant) in rows {
        assert!(
            library
                .variants()
                .any(|registered| registered == (kind, variant)),
            "a row places the {variant:?} skin {kind:?} does not register"
        );
        assert!(
            offered(kind, variant),
            "{kind:?}'s Symbol property does not offer the {variant:?} skin a row places"
        );
    }
}

/// A variant row arms its base type with its own card and skin, and the
/// instance it places carries both.
#[test]
fn a_variant_row_places_its_type_with_its_card_and_skin() {
    let mut app = RSpiceApp::test_instance();
    let zener = component_palette()
        .iter()
        .flat_map(|section| section.entries.iter())
        .find(|entry| entry.label == "Zener Diode")
        .expect("the Zener row");
    arm_variant(
        &mut app.state.model,
        &mut app.state.workbench,
        zener,
        &egui::Context::default(),
    );
    assert_eq!(
        app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::Diode)
    );
    let id = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::origin());
    let placed = app
        .state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("placed");
    assert_eq!(placed.value, "RSPICE_ZENER");
    assert_eq!(placed.symbol_variant.as_deref(), Some("zener"));
}

/// The four rails are filed with the connectivity objects. A rail's row arms
/// the Label tool with its name and symbol, every click places that rail as
/// a global label, and the row reads as armed until the tool changes.
#[test]
fn a_supply_row_places_its_global_rail_on_every_click() {
    use crate::state::{NetLabelKind, Point};
    use rspice_design::schematic::net_label::SupplyGlyph;

    let filed = palette_rows(ComponentCategory::HierarchyConnectivity)
        .into_iter()
        .filter_map(|row| match row {
            PaletteRow::Supply(entry) => Some(entry.name),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(filed, ["VDD", "VCC", "VSS", "VEE"]);

    let mut app = RSpiceApp::test_instance();
    for (name, glyph) in [
        ("VDD", SupplyGlyph::Bar),
        ("VCC", SupplyGlyph::Arrow),
        ("VSS", SupplyGlyph::Down),
        ("VEE", SupplyGlyph::Down),
    ] {
        let entry = supply_palette()
            .iter()
            .find(|entry| entry.name == name)
            .expect("a filed rail");
        arm_supply(
            &mut app.state.model,
            &mut app.state.workbench,
            entry,
            &egui::Context::default(),
        );
        assert_eq!(app.state.model.schematic.session.editor.tool, Tool::Label);
        let before = app.state.model.schematic.document().net_labels.len();
        for y in [0, 20] {
            let dialog_modal_open = app.state.dialog_modal_open();
            assert!(crate::workbench::app::place_sequenced_net_label_in(
                &mut app.state.model,
                &mut app.state.workbench,
                dialog_modal_open,
                Point::new(10 * before as i32, y)
            ));
        }
        let labels = &app.state.model.schematic.document().net_labels[before..];
        assert_eq!(labels.len(), 2, "{name}: one label per click");
        for label in labels {
            assert_eq!(label.name, name);
            assert_eq!(label.kind, NetLabelKind::Supply { glyph });
            assert_eq!(label.global_declaration(), Some(format!("{name}!")));
        }
        let armed = ShelfEntry::Supply(name.to_owned());
        assert!(
            shelf_hits(
                &app.state.model,
                &crate::workbench::design::navigator::shelf::search::ShelfQuery::new(name),
                &[],
                &[],
            )
            .iter()
            .any(|hit| hit.label == name && hit.selected),
            "{name}: the armed rail's row reads as armed"
        );
        assert_eq!(
            ShelfEntry::from_storage_key(&armed.storage_key()),
            Some(armed)
        );
    }
    app.state.model.schematic.arm_tool(Tool::Select);
    let dialog_modal_open = app.state.dialog_modal_open();
    assert!(
        !crate::workbench::app::place_sequenced_net_label_in(
            &mut app.state.model,
            &mut app.state.workbench,
            dialog_modal_open,
            Point::new(0, 80)
        ),
        "a changed tool retires the rail"
    );
}

/// One painted frame of the primitive catalog: every text run it painted and
/// every control it announced, with the disclosure position each publishes.
#[cfg(not(target_arch = "wasm32"))]
fn shelf_frame(
    ctx: &egui::Context,
    state: &mut AppState,
    events: Vec<egui::Event>,
) -> (String, Vec<(String, egui::Rect, Option<bool>)>) {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(260.0, 1600.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.set_width(260.0);
                    component_catalog(ui, &mut state.model);
                });
        },
    );
    (painted_text(&output), announced_controls(&output))
}

/// Every control one painted frame announced: its label, where it sits, and
/// the disclosure position it publishes.
#[cfg(not(target_arch = "wasm32"))]
fn announced_controls(output: &egui::FullOutput) -> Vec<(String, egui::Rect, Option<bool>)> {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .map(|update| {
            update
                .nodes
                .iter()
                .filter_map(|(_, node)| {
                    let label = node.label()?.to_owned();
                    let bounds = node.bounds()?;
                    Some((
                        label,
                        egui::Rect::from_min_max(
                            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                        ),
                        node.is_expanded(),
                    ))
                })
                .collect()
        })
        .unwrap_or_default()
}

/// A fresh install's shelf leads with the Passives rows: that one band opens
/// by default, every other band folds — and states its count — until the
/// reader unfolds it. The reader's own position then outlives the shipped
/// default, because the default sits only behind the persisted flag.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_fresh_shelf_opens_passives_and_folds_the_rest_until_the_reader_moves_one() {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();

    // Two settling frames: the first builds the font set, the second lays
    // out against it, and only then are the band rects worth clicking.
    let _ = shelf_frame(&ctx, &mut app.state, Vec::new());
    let (text, controls) = shelf_frame(&ctx, &mut app.state, Vec::new());

    for passive in ["Resistor", "Capacitor", "Potentiometer"] {
        assert!(
            text.contains(passive),
            "a fresh shelf shows the Passives rows, missing {passive}: {text}"
        );
    }
    for folded in [
        "Voltage Source",
        "NMOS",
        "AND Gate (event-driven)",
        "Ideal Op-Amp",
    ] {
        assert!(
            !text.contains(folded),
            "{folded} belongs to a band a fresh install keeps folded: {text}"
        );
    }
    let expanded = |controls: &[(String, egui::Rect, Option<bool>)], band: &str| {
        let hits = controls
            .iter()
            .filter(|(label, _, _)| label == band)
            .collect::<Vec<_>>();
        assert_eq!(hits.len(), 1, "exactly one control announces {band:?}");
        (hits[0].1, hits[0].2)
    };
    assert_eq!(expanded(&controls, "Passives").1, Some(true));
    for band in [
        "Hierarchy & connectivity",
        "Sources",
        "Semiconductors",
        "Digital (event-driven)",
    ] {
        assert_eq!(
            expanded(&controls, band).1,
            Some(false),
            "{band} starts folded"
        );
    }

    // The reader folds Passives; the press must beat the shipped default in
    // the frame it lands in and in every frame after it.
    let at = expanded(&controls, "Passives").0.center();
    let _ = shelf_frame(&ctx, &mut app.state, click_events(at));
    let (text, controls) = shelf_frame(&ctx, &mut app.state, Vec::new());
    assert_eq!(
        expanded(&controls, "Passives").1,
        Some(false),
        "the reader's fold outlives the default-open position"
    );
    assert!(
        !text.contains("Resistor"),
        "a folded Passives band paints no rows: {text}"
    );
}

/// A code model whose port widths are not fixed cannot be armed until they are
/// chosen, so its click raises the placement dialog and arms nothing. The
/// decision is held in one place, so every row that offers the model asks the
/// same question.
#[test]
fn a_code_model_with_open_port_widths_asks_before_it_arms() {
    let (mut open, mut fixed) = (None, None);
    for descriptor in engine_only_xspice_devices() {
        if builtin_xspice_library_binding(descriptor).is_err() {
            continue;
        }
        let Ok(ports) = builtin_xspice_vector_ports(descriptor) else {
            continue;
        };
        if ports
            .iter()
            .any(|port| port.maximum.is_none_or(|maximum| maximum != port.minimum))
        {
            open.get_or_insert(descriptor);
        } else {
            fixed.get_or_insert(descriptor);
        }
    }
    let open = open.expect("the registry carries a code model with an open port width");
    let fixed = fixed.expect("the registry carries a code model whose ports are all fixed");

    let mut app = RSpiceApp::test_instance();
    assert!(
        place_builtin_xspice(
            &mut app.state.design,
            &mut app.state.model,
            &mut app.state.workbench,
            fixed.stable_id
        )
        .is_some(),
        "{} has nothing left to choose, so it arms straight away",
        fixed.stable_id
    );
    assert!(
        !app.state.design.builtin_xspice_placement.open,
        "and asks nothing"
    );

    app.state.workbench.drawer = Some(crate::workbench::state::Drawer::Navigator);
    assert!(
        place_builtin_xspice(
            &mut app.state.design,
            &mut app.state.model,
            &mut app.state.workbench,
            open.stable_id
        )
        .is_none(),
        "{} cannot be armed until its widths are chosen",
        open.stable_id
    );
    assert!(
        app.state.design.builtin_xspice_placement.open,
        "so the click raises the placement dialog instead"
    );
    assert_eq!(
        app.state.design.builtin_xspice_placement.stable_id, open.stable_id,
        "for the model the row named"
    );
    assert_eq!(
        app.state.workbench.drawer, None,
        "the configuration dialog takes input from the shelf"
    );
    assert!(app.state.workbench.navigator_visible);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn shelf_selection_returns_to_canvas_without_hiding_desktop_dock() {
    use crate::workbench::state::Drawer;

    let builtin = engine_only_xspice_devices()
        .iter()
        .find(|descriptor| {
            builtin_xspice_library_binding(descriptor).is_ok()
                && builtin_xspice_vector_ports(descriptor).is_ok_and(|ports| ports.is_empty())
        })
        .expect("a fixed-port built-in is available");
    for (entry, label, tool) in [
        (
            ShelfEntry::Primitive(ComponentType::Resistor),
            "Resistor",
            Tool::Place(ComponentType::Resistor),
        ),
        (
            ShelfEntry::BuiltinXspice(builtin.stable_id.to_owned()),
            builtin.display_name,
            Tool::Place(ComponentType::CellInstance),
        ),
        (
            ShelfEntry::LibraryPart("RSPICE_ZENER".to_owned()),
            "RSPICE_ZENER",
            Tool::Place(ComponentType::Diode),
        ),
    ] {
        for drawer in [Some(Drawer::Navigator), None] {
            let mut shelf = ShelfHarness::opened();
            shelf
                .app
                .state
                .model
                .model_library_manager
                .add_library(retained_model_library("proving_parts"));
            shelf.app.state.workbench.drawer = drawer;
            shelf.app.state.workbench.navigator_visible = true;
            if !is_pinned(&shelf.app.state.model, &entry) {
                toggle_pin(&mut shelf.app.state.model, &entry);
            }
            let mut shelf = shelf.settled();
            let (_, runs) = shelf.frame(Vec::new());
            let row = run_rect(&runs, label).expect("the pinned part is rendered");
            let _ = shelf.frame(click_events(row.center()));
            assert_eq!(
                shelf.app.state.model.schematic.session.editor.tool, tool,
                "{label}"
            );
            assert_eq!(
                shelf.app.state.workbench.drawer, None,
                "{label}: the canvas must be reachable after selection"
            );
            assert!(
                shelf.app.state.workbench.navigator_visible,
                "desktop dock preference must survive {label} placement"
            );
        }
    }
}

/// A press and a release of the secondary button, which is what egui reads as
/// the gesture that opens a context menu.
#[cfg(not(target_arch = "wasm32"))]
fn secondary_click_events(at: egui::Pos2) -> Vec<egui::Event> {
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Secondary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Secondary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

/// Painted runs are one galley each, so a row label is its own line: an exact
/// line match tells a band's row apart from a catalog row whose name contains
/// it.
#[cfg(not(target_arch = "wasm32"))]
fn paints_line(text: &str, line: &str) -> bool {
    text.lines().any(|painted| painted == line)
}

/// The stored history, as the durable domain holds it.
#[cfg(not(target_arch = "wasm32"))]
fn stored_recent(state: &AppState) -> Vec<String> {
    state.model.preferences.component_shelf().recent
}

/// Arm one primitive and put it on the sheet, the way a click-to-arm placement
/// reaches the design.
#[cfg(not(target_arch = "wasm32"))]
fn place(state: &mut AppState, kind: ComponentType) {
    state.model.schematic.arm_tool(Tool::Place(kind));
    state
        .model
        .schematic
        .add_component(kind, crate::state::Point::new(40, 40));
}

/// A fresh profile takes the shipped pin set, and the first pin or unpin makes
/// the reader's own set authoritative: the shipped list never merges back in,
/// so a default the reader removed stays removed.
#[test]
fn a_fresh_profile_takes_the_shipped_pins_until_the_reader_moves_one() {
    let mut app = RSpiceApp::test_instance();
    let key = |kind| ShelfEntry::Primitive(kind).storage_key();

    assert_eq!(
        pinned_keys(&app.state.model),
        vec![
            key(ComponentType::Resistor),
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
        ],
        "a profile that has never pinned anything sees the shipped set"
    );
    assert!(
        app.state
            .model
            .preferences
            .component_shelf()
            .pinned
            .is_none(),
        "and the shipped set is not yet written as the reader's own"
    );

    // A pin lands at the end: the rail the reader builds keeps its positions.
    toggle_pin(
        &mut app.state.model,
        &ShelfEntry::Primitive(ComponentType::Diode),
    );
    assert_eq!(
        pinned_keys(&app.state.model),
        vec![
            key(ComponentType::Resistor),
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
            key(ComponentType::Diode),
        ]
    );

    // Unpinning a shipped default removes it for good.
    toggle_pin(
        &mut app.state.model,
        &ShelfEntry::Primitive(ComponentType::Resistor),
    );
    assert_eq!(
        pinned_keys(&app.state.model),
        vec![
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
            key(ComponentType::Diode),
        ],
        "the shipped default must not merge back into the reader's set"
    );

    // Emptied on purpose stays empty rather than reverting to the shipped set.
    for kind in [
        ComponentType::Capacitor,
        ComponentType::Ground,
        ComponentType::Diode,
    ] {
        toggle_pin(&mut app.state.model, &ShelfEntry::Primitive(kind));
    }
    assert_eq!(pinned_keys(&app.state.model), Vec::<String>::new());
    assert_eq!(
        app.state.model.preferences.component_shelf().pinned,
        Some(Vec::new()),
        "an emptied set is the reader's answer, not an absent one"
    );
}

/// The Pinned band paints the reader's set, and disappears — heading and all —
/// when that set empties. An empty band under a heading would claim a rail the
/// reader just took apart.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_pinned_band_paints_the_set_and_is_absent_once_it_empties() {
    let mut shelf = ShelfHarness::opened().settled();

    let (text, _) = shelf.frame(Vec::new());
    assert!(paints_line(&text, "Pinned"), "the fresh band is painted");
    for shipped in ["Resistor", "Capacitor", "Ground"] {
        assert!(
            paints_line(&text, shipped),
            "the shipped pin {shipped} is missing: {text}"
        );
    }
    // A folded catalog band's row, pinned, still reaches the top of the shelf.
    assert!(
        !paints_line(&text, "NMOS"),
        "NMOS belongs to a band a fresh install folds: {text}"
    );
    toggle_pin(
        &mut shelf.app.state.model,
        &ShelfEntry::Primitive(ComponentType::Nmos),
    );
    let (text, _) = shelf.frame(Vec::new());
    assert!(
        paints_line(&text, "NMOS"),
        "a pinned row is painted even while its catalog band is folded: {text}"
    );

    for kind in [
        ComponentType::Resistor,
        ComponentType::Capacitor,
        ComponentType::Ground,
        ComponentType::Nmos,
    ] {
        toggle_pin(&mut shelf.app.state.model, &ShelfEntry::Primitive(kind));
    }
    let (text, _) = shelf.frame(Vec::new());
    assert!(
        !paints_line(&text, "Pinned"),
        "an emptied set leaves no band at all: {text}"
    );
    assert!(
        !paints_line(&text, "Ground"),
        "and none of its rows: {text}"
    );
}

/// A placeable row is pinned from its own context menu, where the reader found
/// the part — and unpinned from the same menu, which states the position it is
/// in rather than one fixed verb.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_row_is_pinned_and_unpinned_from_its_context_menu() {
    let mut shelf = ShelfHarness::opened().settled();
    let (_, runs) = shelf.frame(Vec::new());
    let row = run_rect(&runs, "Memristor").expect("the Passives band paints a Memristor row");
    let entry = ShelfEntry::Primitive(ComponentType::Memristor);
    assert!(
        !is_pinned(&shelf.app.state.model, &entry),
        "the row starts unpinned"
    );

    let _ = shelf.frame(secondary_click_events(row.center()));
    let (text, runs) = shelf.frame(Vec::new());
    assert!(
        paints_line(&text, "Pin to shelf"),
        "the menu offers the position the row is not in: {text}"
    );

    let pin = run_rect(&runs, "Pin to shelf").expect("the open menu paints its one command");
    let _ = shelf.frame(click_events(pin.center()));
    assert!(
        is_pinned(&shelf.app.state.model, &entry),
        "the command pinned the row it was opened on"
    );

    // The pinned row now leads the shelf, and its menu offers the other
    // position — the same control, stating where the row stands.
    let (_, runs) = shelf.frame(Vec::new());
    let pinned_row = run_rect(&runs, "Memristor").expect("the pinned row is painted at the top");
    let _ = shelf.frame(secondary_click_events(pinned_row.center()));
    let (text, runs) = shelf.frame(Vec::new());
    assert!(
        paints_line(&text, "Unpin") && !paints_line(&text, "Pin to shelf"),
        "a pinned row's menu offers only the way back out: {text}"
    );

    let unpin = run_rect(&runs, "Unpin").expect("the open menu paints its one command");
    let _ = shelf.frame(click_events(unpin.center()));
    assert!(
        !is_pinned(&shelf.app.state.model, &entry),
        "and the command takes the row back off the rail"
    );
}

/// Placement feeds Recent, newest first, with a re-place moving the part to
/// the front instead of listing it twice.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn placement_feeds_recent_front_ordered_and_deduplicated() {
    let mut shelf = ShelfHarness::opened().settled();
    let key = |kind| ShelfEntry::Primitive(kind).storage_key();

    assert!(
        stored_recent(&shelf.app.state).is_empty(),
        "a profile that has placed nothing has no history"
    );

    place(&mut shelf.app.state, ComponentType::Diode);
    let (text, _) = shelf.frame(Vec::new());
    assert_eq!(
        stored_recent(&shelf.app.state),
        vec![key(ComponentType::Diode)]
    );
    assert!(paints_line(&text, "Recent"), "the band appears: {text}");

    place(&mut shelf.app.state, ComponentType::Nmos);
    let _ = shelf.frame(Vec::new());
    assert_eq!(
        stored_recent(&shelf.app.state),
        vec![key(ComponentType::Nmos), key(ComponentType::Diode)],
        "the newest placement leads"
    );

    place(&mut shelf.app.state, ComponentType::Diode);
    let _ = shelf.frame(Vec::new());
    assert_eq!(
        stored_recent(&shelf.app.state),
        vec![key(ComponentType::Diode), key(ComponentType::Nmos)],
        "a re-place moves the part to the front rather than listing it twice"
    );
}

/// The band is capped: a session that places more parts than it can show keeps
/// the newest, and the ones that scrolled off are gone from the band.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_recent_band_shows_only_its_newest_entries() {
    let mut shelf = ShelfHarness::opened().settled();

    // Eight parts, all from bands a fresh install folds, so each label can
    // only reach the paint list through the Recent band.
    let placed = [
        ComponentType::Diode,
        ComponentType::Nmos,
        ComponentType::Pmos,
        ComponentType::NpnBjt,
        ComponentType::PnpBjt,
        ComponentType::VoltageSource,
        ComponentType::CurrentSource,
        ComponentType::OpAmp,
    ];
    for kind in placed {
        place(&mut shelf.app.state, kind);
        let _ = shelf.frame(Vec::new());
    }
    assert_eq!(stored_recent(&shelf.app.state).len(), placed.len());

    let (text, _) = shelf.frame(Vec::new());
    let painted = placed
        .iter()
        .filter(|kind| paints_line(&text, primitive_label(**kind)))
        .count();
    assert_eq!(
        painted, RECENT_SHOWN,
        "the band paints its cap and no more: {text}"
    );
    for dropped in &placed[..placed.len() - RECENT_SHOWN] {
        assert!(
            !paints_line(&text, primitive_label(*dropped)),
            "{} fell off the band and must not be painted: {text}",
            primitive_label(*dropped)
        );
    }
}

/// A pinned part is not listed twice. It keeps its place in the stored
/// history, so unpinning restores it to the band rather than losing it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pinned_part_is_filtered_out_of_recent_but_kept_in_the_history() {
    let mut shelf = ShelfHarness::opened().settled();
    let entry = ShelfEntry::Primitive(ComponentType::Diode);

    place(&mut shelf.app.state, ComponentType::Diode);
    let (text, _) = shelf.frame(Vec::new());
    assert!(
        paints_line(&text, "Diode"),
        "the placement is listed: {text}"
    );

    toggle_pin(&mut shelf.app.state.model, &entry);
    let (text, _) = shelf.frame(Vec::new());
    assert!(
        !paints_line(&text, "Recent"),
        "the only recent part is pinned, so the band has nothing left: {text}"
    );
    assert!(
        paints_line(&text, "Diode"),
        "and it is painted once, in the Pinned band: {text}"
    );
    assert_eq!(
        stored_recent(&shelf.app.state),
        vec![entry.storage_key()],
        "the history keeps it, so unpinning restores its position"
    );

    toggle_pin(&mut shelf.app.state.model, &entry);
    let (text, _) = shelf.frame(Vec::new());
    assert!(
        paints_line(&text, "Recent") && paints_line(&text, "Diode"),
        "unpinning returns it to the band: {text}"
    );
}

/// Only what the shelf offered the canvas is credited. A design that grows on
/// its own — a paste, an import, a script — is not this reader's placement
/// history, and the bands must not claim it is.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_design_that_grows_without_the_shelf_writes_no_history() {
    let mut shelf = ShelfHarness::opened().settled();

    shelf.app.state.model.schematic.cancel_tool();
    shelf
        .app
        .state
        .model
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::new(40, 40));
    let (text, _) = shelf.frame(Vec::new());

    assert!(
        stored_recent(&shelf.app.state).is_empty(),
        "nothing was offered"
    );
    assert!(
        !paints_line(&text, "Recent"),
        "so there is no band to paint: {text}"
    );
}

/// Opening a different design does not read its existing objects as
/// placements: the watch adopts the new count in silence.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn switching_designs_credits_nothing_to_the_reader() {
    let mut shelf = ShelfHarness::opened().settled();

    shelf
        .app
        .state
        .model
        .schematic
        .arm_tool(Tool::Place(ComponentType::Diode));
    let _ = shelf.frame(Vec::new());

    // A design the reader opened, already holding objects someone else placed.
    shelf.app.state.model.active_schematic_epoch += 1;
    for offset in 0..4 {
        shelf.app.state.model.schematic.add_component(
            ComponentType::Nmos,
            crate::state::Point::new(40 + offset, 40),
        );
    }
    let _ = shelf.frame(Vec::new());
    assert!(
        stored_recent(&shelf.app.state).is_empty(),
        "the objects already on an opened design were not placed by this reader"
    );

    // From there the watch tracks the new design normally.
    place(&mut shelf.app.state, ComponentType::Diode);
    let _ = shelf.frame(Vec::new());
    assert_eq!(
        stored_recent(&shelf.app.state),
        vec![ShelfEntry::Primitive(ComponentType::Diode).storage_key()]
    );
}

/// A band row is a door to the same place its catalog row leads: clicking it
/// arms the part it names, and the row then states that it is the armed one.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_band_row_arms_the_part_it_names_and_says_so() {
    /// The tone one painted run was set in.
    fn tone(runs: &[(String, egui::Rect, egui::Color32)], text: &str) -> Option<egui::Color32> {
        runs.iter()
            .find(|(run, _, _)| run == text)
            .map(|(_, _, colour)| *colour)
    }

    let mut shelf = ShelfHarness::opened().settled();
    // NMOS belongs to a band a fresh install folds, so the only NMOS row on
    // screen is the pinned one — the click cannot land on a catalog row by
    // accident.
    toggle_pin(
        &mut shelf.app.state.model,
        &ShelfEntry::Primitive(ComponentType::Nmos),
    );
    let (_, runs) = shelf.frame(Vec::new());
    let row = run_rect(&runs, "NMOS").expect("the pinned NMOS row is painted");
    let resting = tone(&runs, "NMOS");
    assert_ne!(
        shelf.app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::Nmos),
        "nothing is armed before the click"
    );

    let _ = shelf.frame(click_events(row.center()));
    assert_eq!(
        shelf.app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::Nmos),
        "the click armed the part the row names"
    );

    let (_, runs) = shelf.frame(Vec::new());
    assert_ne!(
        tone(&runs, "NMOS"),
        resting,
        "and the row states that it is the armed one"
    );
    assert_eq!(
        tone(&runs, "Ground"),
        resting,
        "while the rest of the rail is left alone"
    );
}

/// A part dragged onto the sheet is credited, which is the route the armed
/// tool never covers: the drop consumes the payload, so the shelf sees the
/// design grow only on the frame after the offer is gone.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_drag_onto_the_sheet_credits_the_part_the_drop_consumed() {
    let mut shelf = ShelfHarness::opened().settled();
    let payload = SchematicShelfDragPayload::primitive(ComponentType::Nmos)
        .expect("a primitive travels to the canvas as a drag payload");

    // The drag in flight: an identity is on offer and nothing has landed.
    egui::DragAndDrop::set_payload(&shelf.ctx, payload);
    let _ = shelf.frame(Vec::new());
    assert!(
        stored_recent(&shelf.app.state).is_empty(),
        "a drag still in flight has placed nothing"
    );

    // The drop: the canvas takes the payload and commits the object.
    egui::DragAndDrop::clear_payload(&shelf.ctx);
    shelf
        .app
        .state
        .model
        .schematic
        .add_component(ComponentType::Nmos, crate::state::Point::new(40, 40));
    let (text, _) = shelf.frame(Vec::new());

    assert_eq!(
        stored_recent(&shelf.app.state),
        vec![ShelfEntry::Primitive(ComponentType::Nmos).storage_key()],
        "the dropped part is what this reader most recently placed"
    );
    assert!(paints_line(&text, "NMOS"), "and the band lists it: {text}");
}

/// An offer the reader took back credits nothing later. The shelf holds a
/// spent offer exactly one frame past the act that consumed it — long enough
/// for the drop to land — and no longer, so a paste or an import made
/// afterwards is not read as another placement of the part last armed.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_tool_the_reader_put_away_credits_no_later_growth() {
    let mut shelf = ShelfHarness::opened().settled();

    shelf
        .app
        .state
        .model
        .schematic
        .arm_tool(Tool::Place(ComponentType::Diode));
    let _ = shelf.frame(Vec::new());
    shelf.app.state.model.schematic.cancel_tool();
    let _ = shelf.frame(Vec::new());
    let _ = shelf.frame(Vec::new());
    assert!(
        stored_recent(&shelf.app.state).is_empty(),
        "arming a tool and putting it away places nothing"
    );

    // The design grows on its own, well after the offer was withdrawn.
    shelf
        .app
        .state
        .model
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::new(40, 40));
    let (text, _) = shelf.frame(Vec::new());

    assert!(
        stored_recent(&shelf.app.state).is_empty(),
        "the shelf had nothing on offer when the design grew: {:?}",
        stored_recent(&shelf.app.state)
    );
    assert!(
        !paints_line(&text, "Recent"),
        "so there is no band to paint: {text}"
    );
}

/// The stored history outlasts the band it feeds, and stops at its own cap.
///
/// Both halves matter: keeping more than is shown is what lets unpinning
/// restore a full band, and stopping is what keeps a personal preference file
/// from growing without bound.
#[test]
fn the_history_outlasts_the_band_and_stops_at_its_own_cap() {
    assert!(
        RECENT_STORED > RECENT_SHOWN,
        "a history no longer than the band cannot survive a pin"
    );

    let mut app = RSpiceApp::test_instance();
    let part = |index: usize| ShelfEntry::LibraryPart(format!("PART{index}"));
    for index in 0..RECENT_STORED + 4 {
        record_placement(&mut app.state.model, &part(index));
    }

    let recent = app.state.model.preferences.component_shelf().recent;
    assert_eq!(recent.len(), RECENT_STORED, "the history stops at its cap");
    assert_eq!(
        recent.first(),
        Some(&part(RECENT_STORED + 3).storage_key()),
        "and it is the newest end that is kept"
    );
    assert!(
        !recent.contains(&part(0).storage_key()),
        "while the oldest falls off"
    );
}

/// The Component shelf answers the traversal its sibling navigator does.
///
/// One panel, one grammar: the shelf's bands are rows of the same rail, its
/// catalog groups fold from Right and Left, and its parts are reached by
/// stepping rather than by tabbing past every row above them. A shelf that
/// only answered Tab would have made the four-hundred-odd rows of the
/// primitive catalog unreachable in practice.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_shelf_rail_walks_and_folds_from_the_keyboard() {
    /// What the keyboard is on.
    fn focus(output: &egui::FullOutput) -> Option<String> {
        let update = output.platform_output.accesskit_update.as_ref()?;
        update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| announced_name(node))
    }
    /// The disclosure position one named control publishes.
    fn expanded(output: &egui::FullOutput, label: &str) -> Option<bool> {
        let update = output.platform_output.accesskit_update.as_ref()?;
        update
            .nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .and_then(|(_, node)| node.is_expanded())
    }

    let mut shelf = ShelfHarness::opened().reporting_accessibility().settled();

    shelf.app.state.workbench.outbox.push(FocusPlacementSearch);
    let output = shelf.output(Vec::new());
    assert_eq!(
        focus(&output).as_deref(),
        Some("Place component or cell…"),
        "the shelf's filter takes the keyboard it was asked for"
    );

    // Step down to a folded catalog group the way a reader without a pointer
    // reaches it. The rail is finite, so a group that never takes the keyboard
    // is one such a reader could never open.
    let mut output = shelf.output(vec![key_event(egui::Key::ArrowDown)]);
    assert!(
        focus(&output).is_some_and(|row| row != "Place component or cell…"),
        "and Down steps out of the query onto the rail"
    );
    let mut landed = false;
    for _ in 0..24 {
        if focus(&output).as_deref() == Some("Sources") {
            landed = true;
            break;
        }
        output = shelf.output(vec![key_event(egui::Key::ArrowDown)]);
    }
    assert!(landed, "the Sources group must be reachable by stepping");
    assert_eq!(
        expanded(&output, "Sources"),
        Some(false),
        "which a fresh shelf shows folded"
    );

    let _ = shelf.output(vec![key_event(egui::Key::ArrowRight)]);
    let output = shelf.output(Vec::new());
    assert_eq!(
        expanded(&output, "Sources"),
        Some(true),
        "Right unfolds the group the keyboard is on"
    );
    assert_eq!(
        focus(&output).as_deref(),
        Some("Sources"),
        "and leaves the keyboard on it"
    );

    let _ = shelf.output(vec![key_event(egui::Key::ArrowRight)]);
    let output = shelf.output(Vec::new());
    assert_eq!(
        focus(&output).as_deref(),
        Some("Voltage Source"),
        "a second Right steps onto the first part it disclosed"
    );

    let _ = shelf.output(vec![key_event(egui::Key::ArrowLeft)]);
    let output = shelf.output(Vec::new());
    assert_eq!(
        focus(&output).as_deref(),
        Some("Sources"),
        "and Left climbs out of a part to the group that holds it"
    );

    let _ = shelf.output(vec![key_event(egui::Key::ArrowLeft)]);
    let output = shelf.output(Vec::new());
    assert_eq!(
        expanded(&output, "Sources"),
        Some(false),
        "which a second Left folds again"
    );
}

/// The pin menu is reachable without a pointer: the shelf's rows are in the
/// tab ring, and Shift+F10 opens the focused row's menu — the same key the
/// navigator's object menu answers to.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_pin_menu_opens_from_the_keyboard_on_the_focused_row() {
    /// The row the keyboard is on, as the accessibility tree reports it.
    fn focused_label(output: &egui::FullOutput) -> Option<String> {
        let update = output.platform_output.accesskit_update.as_ref()?;
        update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| node.label().map(str::to_owned))
    }

    let mut shelf = ShelfHarness::opened().reporting_accessibility().settled();

    let key = |key, modifiers| egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    };
    // Walk the keyboard onto a catalog row the way a reader without a pointer
    // reaches it. The ring is finite, so a row that never takes focus is a
    // row no such reader could ever pin.
    let mut landed = false;
    for _ in 0..80 {
        let output = shelf.output(vec![key(egui::Key::Tab, egui::Modifiers::NONE)]);
        if focused_label(&output).as_deref() == Some("Memristor") {
            landed = true;
            break;
        }
    }
    assert!(landed, "a shelf row must be reachable from the keyboard");

    let _ = shelf.frame(vec![key(egui::Key::F10, egui::Modifiers::SHIFT)]);
    let (text, runs) = shelf.frame(Vec::new());
    assert!(
        paints_line(&text, "Pin to shelf"),
        "Shift+F10 opens the focused row's pin menu: {text}"
    );

    let pin = run_rect(&runs, "Pin to shelf").expect("the open menu paints its one command");
    let _ = shelf.frame(click_events(pin.center()));
    assert!(
        is_pinned(
            &shelf.app.state.model,
            &ShelfEntry::Primitive(ComponentType::Memristor)
        ),
        "and the menu acts on the row the keyboard was on"
    );
}

/// A project's stimulus definitions are placeable parts, so the shelf lists
/// them and a click arms the cursor with the saved revision.
///
/// Driven through the search field rather than by opening the band: the
/// section folds shut on a fresh profile like every other catalog, and a query
/// is how a reader reaches a definition without first learning where it lives.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_stimulus_definition_is_listed_as_a_part_and_arms_the_cursor() {
    let mut shelf = ShelfHarness::opened();
    let mut definition = crate::state::stimulus_library::definition::StimulusDefinition::new(
        "sensor_drive",
        ComponentType::VoltageSourceSin,
        crate::state::stimulus_library::now_unix_ms,
    )
    .expect("definition");
    definition.value = "0".to_owned();
    definition.params = "va=3m freq=1k".to_owned();
    shelf
        .app
        .state
        .model
        .workspace
        .content
        .stimulus_library
        .insert(definition)
        .expect("insert");
    shelf.app.state.design.placement_query = "sensor_drive".to_owned();

    let mut shelf = shelf.settled();
    let (text, runs) = shelf.frame(Vec::new());
    assert!(
        paints_line(&text, "Matches") && !paints_line(&text, "Stimulus library"),
        "a query answers in one ranked list, not under the catalog's sections: {text}"
    );
    assert!(
        paints_line(&text, "V \u{00b7} SIN \u{00b7} 1kHz"),
        "the meta column reads the instance the definition would place: {text}"
    );

    // Aimed at the meta cell rather than the name: the query that reveals the
    // section is the definition's own name, so the search field paints that
    // string first and a press aimed at it lands in the text box.
    let row = run_rect(&runs, "V \u{00b7} SIN \u{00b7} 1kHz")
        .expect("the definition is rendered as a row");
    let _ = shelf.frame(click_events(row.center()));
    assert_eq!(
        shelf.app.state.model.schematic.session.editor.tool,
        Tool::Place(ComponentType::VoltageSourceSin),
        "the row's click must reach the arming action"
    );
    let armed = shelf
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .pending_stimulus
        .as_ref()
        .expect("the click armed the definition");
    assert_eq!(armed.definition(), "sensor_drive");
    assert_eq!(armed.revision(), 1);
}

/// A project that has authored no stimulus paints no section at all. An empty
/// header is a door that leads nowhere, and the shelf's other catalogs already
/// hold that line.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn an_empty_stimulus_library_paints_no_section() {
    let mut shelf = ShelfHarness::opened().settled();
    assert!(
        shelf
            .app
            .state
            .model
            .workspace
            .content
            .stimulus_library
            .is_empty()
    );
    let (text, _) = shelf.frame(Vec::new());
    assert!(
        !paints_line(&text, "Stimulus library"),
        "an unauthored library must leave the shelf as it was: {text}"
    );
}
